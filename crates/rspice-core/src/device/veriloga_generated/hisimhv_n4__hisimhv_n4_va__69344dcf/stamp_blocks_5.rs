#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {s.store_square(723, 1906);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2061] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2061, if s.b[2061] { 1.0 } else { 0.0 });s.b[2062] = (2.0 == 1.0);s.store_scalar(2062, if s.b[2062] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && s.b[2062]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[2063] = (2.0 == 2.0);s.store_scalar(2063, if s.b[2063] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && s.b[2063]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[2064] = (2.0 == 4.0);s.store_scalar(2064, if s.b[2064] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && s.b[2064]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[2065] = (2.0 == 8.0);s.store_scalar(2065, if s.b[2065] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && (!s.b[2064])) && s.b[2065]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;assert!(ta <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && (!s.b[2061])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1906, 726);s.store_div_scaled_product3_indices(334, 1906, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1906, 1.0, 780, -1.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2060])) {
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2060])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {s.store_div(335, 989, 344);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {s.store_mul(340, 338, 337);s.store_div(989, 989, 340);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_sub(335, 791, 1883);}
        s.b[2066] = ((s.v[335] < 1.0) && (1.0 >= 0.0));s.store_scalar(2066, if s.b[2066] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {s.store_sub_from_scalar(781, 1.0, 335);s.store_square(722, 781);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb);
        let (tc,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc);
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2067] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2067, if s.b[2067] { 1.0 } else { 0.0 });s.b[2068] = (2.0 == 1.0);s.store_scalar(2068, if s.b[2068] { 1.0 } else { 0.0 });
        let (td,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && s.b[2068]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td);s.b[2069] = (2.0 == 2.0);s.store_scalar(2069, if s.b[2069] { 1.0 } else { 0.0 });
        let (te,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && s.b[2069]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te);s.b[2070] = (2.0 == 4.0);s.store_scalar(2070, if s.b[2070] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (tf,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && s.b[2070]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf);s.b[2071] = (2.0 == 8.0);s.store_scalar(2071, if s.b[2071] { 1.0 } else { 0.0 });
        let (t10,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && (!s.b[2070])) && s.b[2071]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t10);
        let (t11,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t11);let mut t15: usize = 0;
        while {
            let t14: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {s.store_sqrt(726, 726);}
            let (t13,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
        let t12: f64 = (s.v[719] + 1.0);
        (t12,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t13);
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && (!s.b[2067])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1.0);s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(335, 1.0, 780);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2066])) {
        }
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2066])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_div(251, 335, 965);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p353 - 1.0));
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_mul(342, 339, 251);s.store_offset(336, 966, 1e-25);s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1881, 989, 170);s.store_square(781, 989);s.store_scalar(782, {let pb=0.1;pb*pb});s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));s.store_div(335, 335, 170);s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_div(985, 254, 338);s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1881);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_mul3_lhs(986, 115, 248, 984);s.store_add(135, 986, 987);s.copy_ad(790, 349);}
        s.b[2072] = (p.p283 != 0.0);s.store_scalar(2072, if s.b[2072] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1849), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2073] = (s.v[336] < 0.0);s.store_scalar(2073, if s.b[2073] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) && s.b[2073]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p.p284);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1435, p.p285, 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1849, 1.0, 340, 1.0, 1434, -1.0);s.store_add_product3_rhs_indices(338, 338, 1435, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2072])) {s.store_scalar(343, 0.0);}
        s.b[2074] = (p.p287 != 0.0);s.store_scalar(2074, if s.b[2074] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2074]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1435);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2074])) {s.store_scalar(342, 0.0);}
        s.b[2075] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2075, if s.b[2075] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2075]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[2076] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(2076, if s.b[2076] { 1.0 } else { 0.0 });s.b[2077] = (p.p296 > 0.0);s.store_scalar(2077, if s.b[2077] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p.p296 + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2077])) {s.copy_ad(341, 647);}
        s.b[2078] = (s.v[793] >= 0.0);s.store_scalar(2078, if s.b[2078] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2078]) {s.copy_ad(369, 793);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2078])) {s.store_scalar(369, 0.0);}
        s.b[2079] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(2079, if s.b[2079] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2079]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2079])) {s.store_powf_offset_input(335, 369, 1e-12, p.p297);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) {s.store_powf_offset_input(343, 369, 1e-12, p.p299);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2076])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_add_scaled_inputs4_indices(131, 1860, (-0.5), 1861, (-0.5), 1863, (-0.5), 1865, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {s.store_scaled_add_mixed_ai(133, A::add(A::add_scaled_inputs4(s.ad_value(1888), 1.0, s.ad_value(1889), 1.0, s.ad_value(1891), 1.0, s.ad_value(1892), 1.0), s.ad_value(1862)), 1864, (-0.5));s.store_scalar(247, 0.5);s.store_scaled_add(978, 1888, 1889, (-0.5));s.store_neg(238, 1888);s.copy_ad(255, 1882);}
        s.b[2080] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));s.store_scalar(2080, if s.b[2080] { 1.0 } else { 0.0 });
        let (t16,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2080]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t16);
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(2087, 960);s.store_scale(2129, 964, 1.6021918e-19);s.store_scale(2110, 964, (1.6021918e-19 * 1.034943e-10));s.store_div_from_scalar(2132, (2.0 * 1.034943e-10), 2129);s.store_div(2126, 964, 622);s.store_div_from_scalar_offset_input(2125, 1.0, 2126, 1.0);s.store_div_square_rhs(2130, 2110, 185);s.store_div_from_scalar(2131, 2.0, 2130);s.store_scalar(2139, 2.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));}
        s.b[2168] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));s.store_scalar(2168, if s.b[2168] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {s.store_sub_from_scalar(781, (2.0 + 0.1), 508);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t17,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t17);
        let (t18,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t18);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2169] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2169, if s.b[2169] { 1.0 } else { 0.0 });s.b[2170] = (2.0 == 1.0);s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });
        let (t19,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && s.b[2170]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t19);s.b[2171] = (2.0 == 2.0);s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });
        let (t1a,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && s.b[2171]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1a);s.b[2172] = (2.0 == 4.0);s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });
        let (t1b,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && s.b[2172]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1b);s.b[2173] = (2.0 == 8.0);s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });
        let (t1c,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && (!s.b[2172])) && s.b[2173]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1c);
        let (t1d,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1d);let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {s.store_sqrt(726, 726);}
            let (t1f,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
        let t1e: f64 = (s.v[719] + 1.0);
        (t1e,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1f);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && (!s.b[2169])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(508, (2.0 + 0.1), 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2168])) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2168])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scalar(2085, 0.0);s.store_scalar(2086, 0.0);s.store_scalar(2094, 0.0);s.store_scalar(2095, 0.0);s.store_scalar(2167, 0.0);s.store_scalar(2142, 0.0);s.copy_ad(2113, 1431);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));s.store_scalar(782, ((4.0 * 0.3) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2092, 781, (-0.5), 782, (-0.5), 0.3);s.store_add_scaled_inputs3_offset_indices(781, 2092, 1.0, 2113, -1.0, 2087, 1.0, (-0.01));s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2092, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);s.copy_ad(2085, 2092);s.store_scalar(2083, 0.0);s.copy_ad(2088, 2083);s.store_mul_sub_rhs(2090, 2125, 1434, 2087);s.store_mul_scale_offset_indices(2146, 2125, 2087, -1.0, 0.0);}
        s.b[2174] = (((-s.v[2090]) < 0.001) && (0.001 >= 0.0));s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2090)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t22,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t22);
        let (t23,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t23);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2175] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });s.b[2176] = (2.0 == 1.0);s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });
        let (t24,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && s.b[2176]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t24);s.b[2177] = (2.0 == 2.0);s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });
        let (t25,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && s.b[2177]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t25);s.b[2178] = (2.0 == 4.0);s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });
        let (t26,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && s.b[2178]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t26);s.b[2179] = (2.0 == 8.0);s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });
        let (t27,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && (!s.b[2178])) && s.b[2179]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        let (t28,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t28);let mut t2c: usize = 0;
        while {
            let t2b: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;assert!(t2c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {s.store_sqrt(726, 726);}
            let (t2a,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
        let t29: f64 = (s.v[719] + 1.0);
        (t29,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2a);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && (!s.b[2175])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2174])) {s.store_neg(335, 2090);s.store_scalar(337, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sqrt_mul(2081, 2132, 335);}
        s.b[2180] = (((-s.v[2146]) < 0.001) && (0.001 >= 0.0));s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2146)));s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t2d,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2d);
        let (t2e,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2e);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2181] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });s.b[2182] = (2.0 == 1.0);s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });
        let (t2f,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && s.b[2182]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2f);s.b[2183] = (2.0 == 2.0);s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });
        let (t30,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && s.b[2183]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t30);s.b[2184] = (2.0 == 4.0);s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });
        let (t31,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && s.b[2184]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t31);s.b[2185] = (2.0 == 8.0);s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && (!s.b[2184])) && s.b[2185]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);
        let (t33,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t33);let mut t37: usize = 0;
        while {
            let t36: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t36 != 0.0
        } {
            t37 += 1;assert!(t37 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {s.store_sqrt(726, 726);}
            let (t35,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
        let t34: f64 = (s.v[719] + 1.0);
        (t34,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t35);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && (!s.b[2181])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2180])) {s.store_neg(335, 2146);s.store_scalar(337, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sqrt_mul(2147, 2132, 335);}
        s.b[2186] = (p.p345 != 0.0);s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {s.store_mul_scale_offset_mixed_ia(335, 965, A::scale(s.ad_value(790), p.p345), -1.0, 1.0);s.store_scale(336, 965, 0.001);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(2127, 965);s.store_sub(2144, 965, 2081);s.store_sub(2145, 965, 2147);}
        s.b[2187] = ((s.v[2144] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2144);s.store_square(722, 781);s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t38,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t38);
        let (t39,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t39);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2188] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });s.b[2189] = (1.0 == 1.0);s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });
        let (t3a,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && s.b[2189]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3a);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2190] = (1.0 == 2.0);s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });
        let (t3b,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && s.b[2190]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3b);s.b[2191] = (1.0 == 4.0);s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });
        let (t3c,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && s.b[2191]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3c);s.b[2192] = (1.0 == 8.0);s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });
        let (t3d,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && (!s.b[2191])) && s.b[2192]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3d);
        let (t3e,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3e);let mut t42: usize = 0;
        while {
            let t41: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t41 != 0.0
        } {
            t42 += 1;assert!(t42 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {s.store_sqrt(726, 726);}
            let (t40,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
        let t3f: f64 = (s.v[719] + 1.0);
        (t3f,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t40);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && (!s.b[2188])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);s.store_sub_from_scalar(2144, (p.p344 + (p.p344 * 0.1)), 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2187])) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2187])) {s.store_scalar(334, 1.0);}
        s.b[2193] = ((s.v[2145] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {s.store_sub_from_scalar(781, (p.p344 * 0.1), 2145);s.store_square(722, 781);s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t43,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t43);
        let (t44,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t44);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2194] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });s.b[2195] = (1.0 == 1.0);s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });
        let (t45,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && s.b[2195]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t45);s.b[2196] = (1.0 == 2.0);s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });
        let (t46,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && s.b[2196]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t46);s.b[2197] = (1.0 == 4.0);s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });
        let (t47,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && s.b[2197]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t47);s.b[2198] = (1.0 == 8.0);s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });
        let (t48,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && (!s.b[2197])) && s.b[2198]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t48);
        let (t49,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t49);let mut t4d: usize = 0;
        while {
            let t4c: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4c != 0.0
        } {
            t4d += 1;assert!(t4d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {s.store_sqrt(726, 726);}
            let (t4b,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
        let t4a: f64 = (s.v[719] + 1.0);
        (t4a,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4b);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && (!s.b[2194])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);s.store_sub_from_scalar(2145, (p.p344 * 0.1), 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2193])) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2193])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset_scaled_div(2148, 2144, 2145, (p.p394 - p.p395), p.p395);}
        let (t4e,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t4e);
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul(2136, 2125, 2126);}
        let (t4f,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t4f);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t7a: usize = 0;
        while {
            let t79: f64 = if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t79 != 0.0
        } {
            t7a += 1;assert!(t7a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul_sub_mixed_iai(2090, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2088), 1.0), 2087);s.store_sub(335, 2088, 2090);}
            s.b[2199] = ((s.v[335] < 0.001) && (0.001 >= 0.0));s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {s.store_sub_from_scalar(781, 0.001, 335);s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t76,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t76);
            let (t78,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t78);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2200] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });s.b[2201] = (2.0 == 1.0);s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });
            let (t60,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && s.b[2201]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t60);s.b[2202] = (2.0 == 2.0);s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });
            let (t61,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && s.b[2202]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t61);s.b[2203] = (2.0 == 4.0);s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });
            let (t62,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && s.b[2203]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t62);s.b[2204] = (2.0 == 8.0);s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });
            let (t63,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t63);
            let (t64,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t64);let mut t68: usize = 0;
            while {
                let t67: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t67 != 0.0
            } {
                t68 += 1;assert!(t68 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {s.store_sqrt(726, 726);}
                let (t66,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
        let t65: f64 = (s.v[719] + 1.0);
        (t65,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t66);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && (!s.b[2200])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2199])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2199])) {s.store_scalar(336, 1.0);}
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_sqrt_mul(2081, 2132, 335);}
            s.b[2205] = ((s.v[2081] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {s.store_offset_sub(781, 2081, 2127, 1e-12);s.store_square(722, 781);s.store_scalar(723, (1e-12 * 1e-12));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t69,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t69);
            let (t6a,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6a);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2206] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });s.b[2207] = (2.0 == 1.0);s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });
            let (t6b,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && s.b[2207]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6b);s.b[2208] = (2.0 == 2.0);s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });
            let (t6c,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && s.b[2208]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6c);s.b[2209] = (2.0 == 4.0);s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });
            let (t6d,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && s.b[2209]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6d);s.b[2210] = (2.0 == 8.0);s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });
            let (t6e,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && (!s.b[2209])) && s.b[2210]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t6e);
            let (t6f,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t6f);let mut t73: usize = 0;
            while {
                let t72: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t72 != 0.0
            } {
                t73 += 1;assert!(t73 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {s.store_sqrt(726, 726);}
                let (t71,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
        let t70: f64 = (s.v[719] + 1.0);
        (t70,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t71);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && (!s.b[2206])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-12);s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);s.store_add_offset_lhs(2081, 2127, (-1e-12), 780);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2205])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2205])) {s.store_scalar(337, 1.0);}
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul(337, 336, 337);s.store_add_div_rhs_mixed_ai(2133, 2085, A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2081), A::sub_scaled_inputs(s.ad_value(2081), 1.0, s.ad_value(2127), 2.0), 1.0), 2132);s.store_scalar(2134, 1.0);s.store_mul_scale_offset_mixed_ai(2135, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2081)), s.ad_value(337), (-1.0)), 2136, -1.0, 1.0);}
            s.b[2211] = ((s.v[2133] > (s.v[2083] - p.p406)) && (p.p406 >= 0.0));s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {s.store_offset_sub(781, 2133, 2083, p.p406);s.store_square(722, 781);s.store_scalar(723, (p.p406 * p.p406));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t74,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t74);
            let (t75,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t75);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2212] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });s.b[2213] = (4.0 == 1.0);s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });
            let (t77,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && s.b[2213]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t77);s.b[2214] = (4.0 == 2.0);s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });
            let (t50,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && s.b[2214]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t50);s.b[2215] = (4.0 == 4.0);s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });
            let (t51,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && s.b[2215]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t51);s.b[2216] = (4.0 == 8.0);s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });
            let (t52,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && (!s.b[2215])) && s.b[2216]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t52);
            let (t53,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t53);let mut t57: usize = 0;
            while {
                let t56: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t56 != 0.0
            } {
                t57 += 1;assert!(t57 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {s.store_sqrt(726, 726);}
                let (t55,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
        let t54: f64 = (s.v[719] + 1.0);
        (t54,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t55);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && (!s.b[2212])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p406);s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);s.store_add_offset_lhs(2133, 2083, (-p.p406), 780);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2211])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2211])) {s.store_scalar(334, 1.0);}
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul(2134, 2134, 334);s.store_mul(2135, 2135, 334);s.store_mul_sub_rhs(339, 154, 2085, 2088);s.store_exp(340, 339);s.store_sub_offset_lhs(344, 340, (-1.0), 339);}
            s.b[2217] = (s.v[339] >= 1e-7);s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });
            let (t59,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2217]) {
        let t58: f64 = (-1.0);
        (t58,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, t59);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2217]) {s.store_mul_scaled_sqrt_rhs(2094, 209, -1.0, 344);s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2094, 1.0);s.store_mul_scale_offset_indices(2121, 345, 340, 1.0, (-1.0));s.store_mul_scale_offset_indices(2123, 345, 340, -1.0, 1.0);}
            s.b[2218] = (s.v[339] < (-1e-7));s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });
            let (t5a,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && s.b[2218]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, t5a);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && s.b[2218]) {s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2113), p.p398));s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2113), p.p398));s.store_mul_sqrt_mixed_ia(2094, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2094, 1.0);s.store_mul_add_mixed_iaa(2121, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));s.store_mul_mixed_ia(2123, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));}
            s.b[2219] = (s.v[339] > 0.0);s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && (!s.b[2218])) && s.b[2219]) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2094, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2121, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2123, 2121);}
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && (!s.b[2218])) && (!s.b[2219])) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2094, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2121, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2123, 2121);}
            let (t5c,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] != 0.0)) {
        let t5b: f64 = (150.0 + 1.0);
        (t5b,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5c);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2094, 1.0, 185, 85, 2085, 1.0);s.store_sub(2097, 2121, 185);s.copy_ad(2098, 2123);s.store_sub(2099, 2088, 2133);s.store_neg(2100, 2134);s.store_sub_from_scalar(2101, 1.0, 2135);s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                if (s.v[2102] > 0.0) {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, (-1e-25));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {s.copy_ad(2104, 2101);s.store_neg(2105, 2098);s.store_neg(2106, 2100);s.copy_ad(2107, 2097);s.store_mul_add_scaled_products_indices_rhs(2108, 2103, 2104, 2096, -1.0, 2105, 2099, -1.0);s.store_mul_add_scaled_products_indices_rhs(2109, 2103, 2106, 2096, -1.0, 2107, 2099, -1.0);s.store_abs(335, 2108);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                    s.store_abs(335, 2109);
                } else {
                }
            }
            s.b[2220] = (s.v[335] > 0.1);s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2220]) {s.store_mul_div_from_scalar_lhs_ad_indices(2108, 0.1, 335, 2108);s.store_mul_div_from_scalar_lhs_ad_indices(2109, 0.1, 335, 2109);}
            s.b[2221] = (s.v[335] < 1e-10);s.store_scalar(2221, if s.b[2221] { 1.0 } else { 0.0 });
            let (t5d,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2221]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t5d);
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {s.store_add(2085, 2085, 2108);s.store_add(2088, 2088, 2109);}
            let (t5f,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        let t5e: f64 = (s.v[97] + 1.0);
        (t5e,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5f);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_mul_sub_rhs(339, 154, 2085, 2088);s.store_exp(340, 339);s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2118, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2118, 209, 344);
            }
        }
        s.b[2223] = (1.0 == 1.0);s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });s.b[2224] = (((s.v[2085] - s.v[2083]) < p.p403) && (p.p403 >= 0.0));s.store_scalar(2224, if s.b[2224] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2085), s.ad_value(2083)));s.store_square(722, 781);s.store_scalar(723, (p.p403 * p.p403));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t7b,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t7b);
        let (t7c,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7c);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2225] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });s.b[2226] = (6.0 == 1.0);s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });
        let (t7d,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && s.b[2226]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7d);s.b[2227] = (6.0 == 2.0);s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });
        let (t7e,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && s.b[2227]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7e);s.b[2228] = (6.0 == 4.0);s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });
        let (t7f,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && s.b[2228]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7f);s.b[2229] = (6.0 == 8.0);s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });
        let (t80,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && (!s.b[2228])) && s.b[2229]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t80);
        let (t81,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t81);let mut t85: usize = 0;
        while {
            let t84: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t84 != 0.0
        } {
            t85 += 1;assert!(t85 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {s.store_sqrt(726, 726);}
            let (t83,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
        let t82: f64 = (s.v[719] + 1.0);
        (t82,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t83);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && (!s.b[2225])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p403);s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);s.store_sub_from_scalar(336, p.p403, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && (!s.b[2224])) {s.store_sub(336, 2085, 2083);s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2114, 209, -1.0, 338);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2223])) {s.copy_ad(2114, 2118);}
        s.b[2230] = (1.0 == 1.0);s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.copy_ad(2155, 85);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.store_offset_mul(338, 2131, 2155, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.store_offset_add_ad(2156, s.ad_value(2155), A::mul_sub_from_scalar_rhs(s.ad_value(2130), 1.0, s.ad_value(337)), p.p397);s.copy_ad(2152, 2156);}
        let (t86,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t86);
        let (t87,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t87);let mut t8e: usize = 0;
        while {
            let t8d: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            t8d != 0.0
        } {
            t8e += 1;assert!(t8e <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.store_mul_scale_offset_indices(335, 2152, 154, -1.0, 0.0);s.store_exp(336, 335);s.store_sqrt_div_scaled_inputs(338, 2110, 2.0, 154, 1.0);s.store_offset_sub(344, 336, 335, (-1.0));s.store_mul_sqrt_mixed_ia(2153, 338, A::offset(s.ad_value(344), 1e-15));}
            s.b[2231] = (s.v[335] > 0.0);s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && s.b[2231]) {s.store_neg(2153, 2153);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2153, 1.0);s.store_mul_scale_offset_indices(2154, 345, 336, -1.0, 1.0);}
            let (t8c,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] != 0.0)) {
        let t8b: f64 = (150.0 + 1.0);
        (t8b,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t8c);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) {s.store_add_scaled_offset_product_rhs_mixed_iia(2096, 2153, 1.0, 185, A::sub(s.ad_value(2155), s.ad_value(2152)), p.p397, -1.0);s.store_add(2097, 185, 2154);s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);}
            s.b[2232] = (((s.v[2108]) as f64).abs() < 1e-10);s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });
            let (t88,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && s.b[2232]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t88);s.b[2233] = (s.v[2108] > 0.1);s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && (!s.b[2232])) && s.b[2233]) {s.store_scalar(2108, 0.1);}
            s.b[2234] = (s.v[2108] < (-0.1));s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && (!s.b[2232])) && (!s.b[2233])) && s.b[2234]) {s.store_scalar(2108, (-0.1));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) {s.store_add(2152, 2152, 2108);}
            let (t8a,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        let t89: f64 = (s.v[97] + 1.0);
        (t89,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t8a);
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {s.copy_ad(2157, 2152);s.store_sqrt_square_offset(782, 2157, ((4.0 * p.p404) * p.p404));s.store_offset_scaled_div(334, 2157, 782, 0.5, 0.5);s.store_scaled_add(2158, 2157, 782, 0.5);}
        s.b[2235] = (s.v[2158] < 0.0);s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && s.b[2235]) {s.store_scalar(2158, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) {s.store_offset_mul(338, 2131, 85, 1.0);s.store_offset(339, 2131, 1.0);}
        s.b[2236] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {s.store_sub(781, 339, 338);s.store_square(722, 781);s.store_square(723, 339);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t8f,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8f);
        let (t90,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t90);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2237] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });s.b[2238] = (2.0 == 1.0);s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });
        let (t91,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && s.b[2238]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t91);s.b[2239] = (2.0 == 2.0);s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });
        let (t92,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && s.b[2239]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t92);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2240] = (2.0 == 4.0);s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });
        let (t93,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t93);s.b[2241] = (2.0 == 8.0);s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });
        let (t94,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) && s.b[2241]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t94);
        let (t95,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t95);let mut t99: usize = 0;
        while {
            let t98: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t98 != 0.0
        } {
            t99 += 1;assert!(t99 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {s.store_sqrt(726, 726);}
            let (t97,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
        let t96: f64 = (s.v[719] + 1.0);
        (t96,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t97);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && (!s.b[2237])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 339, 726);s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);s.store_sub(338, 339, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2236])) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2236])) {s.store_scalar(334, 1.0);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) {s.store_sqrt(337, 338);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 2130, 1.0, 337);}
        s.b[2242] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {s.store_sub_from_scalar(781, p.p404, 344);s.store_square(722, 781);s.store_scalar(723, (p.p404 * p.p404));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t9a,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9a);
        let (t9b,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9b);
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2243] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });s.b[2244] = (2.0 == 1.0);s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });
        let (t9c,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && s.b[2244]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9c);s.b[2245] = (2.0 == 2.0);s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });
        let (t9d,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && s.b[2245]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9d);s.b[2246] = (2.0 == 4.0);s.store_scalar(2246, if s.b[2246] { 1.0 } else { 0.0 });
        let (t9e,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && s.b[2246]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9e);s.b[2247] = (2.0 == 8.0);s.store_scalar(2247, if s.b[2247] { 1.0 } else { 0.0 });
        let (t9f,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && (!s.b[2246])) && s.b[2247]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9f);
        let (ta0,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta0);let mut ta4: usize = 0;
        while {
            let ta3: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta3 != 0.0
        } {
            ta4 += 1;assert!(ta4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {s.store_sqrt(726, 726);}
            let (ta2,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
        let ta1: f64 = (s.v[719] + 1.0);
        (ta1,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta2);
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && (!s.b[2243])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p404);s.store_div_scaled_product_indices(334, 725, 726, p.p404, 770, 1.0);s.store_sub_from_scalar(2158, p.p404, 780);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2242])) {s.copy_ad(2158, 344);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(349, 790);s.store_div(335, 790, 2158);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_indices(336, 335, 658);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset(337, 336, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
        }
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_div(348, 790, 338);s.copy_ad(790, 348);}
        s.b[2248] = (s.v[790] < 0.0);s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2248]) {s.copy_ad(2086, 2085);s.copy_ad(2091, 2090);s.copy_ad(2089, 2088);s.copy_ad(2119, 2118);s.copy_ad(2115, 2114);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.copy_ad(2084, 790);s.store_add_scaled_inputs3_offset_indices(781, 2085, 1.0, 2084, 1.0, 85, -1.0, (-0.01));s.store_scaled_add(782, 2085, 2084, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2093, 2085, 1.0, 2084, 1.0, 781, (-0.5), 782, (-0.5));s.store_add_scaled_inputs3_offset_indices(781, 2093, 1.0, 2113, -1.0, 2087, 1.0, (-0.01));s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs4_indices(2093, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);s.copy_ad(2089, 2084);s.copy_ad(2086, 2093);}
        let (ta5,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, ta5);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(2137, 2125, 2126);}
        let (ta6,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, ta6);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut td1: usize = 0;
        while {
            let td0: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            td0 != 0.0
        } {
            td1 += 1;assert!(td1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul_sub_mixed_iai(2091, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2089), 1.0), 2087);s.store_sub(335, 2089, 2091);}
            s.b[2249] = ((s.v[335] < 0.001) && (0.001 >= 0.0));s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_sub_from_scalar(781, 0.001, 335);s.store_square(722, 781);s.store_scalar(723, (0.001 * 0.001));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (tcd,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tcd);
            let (tcf,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tcf);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2250] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });s.b[2251] = (2.0 == 1.0);s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });
            let (tb7,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && s.b[2251]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tb7);s.b[2252] = (2.0 == 2.0);s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });
            let (tb8,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && s.b[2252]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tb8);s.b[2253] = (2.0 == 4.0);s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });
            let (tb9,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && s.b[2253]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tb9);s.b[2254] = (2.0 == 8.0);s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });
            let (tba,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && (!s.b[2253])) && s.b[2254]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tba);
            let (tbb,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tbb);let mut tbf: usize = 0;
            while {
                let tbe: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tbe != 0.0
            } {
                tbf += 1;assert!(tbf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {s.store_sqrt(726, 726);}
                let (tbd,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
        let tbc: f64 = (s.v[719] + 1.0);
        (tbc,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, tbd);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && (!s.b[2250])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.001);s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);s.store_sub_from_scalar(335, 0.001, 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {s.store_scalar(336, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_sqrt_mul(2082, 2132, 335);}
            s.b[2255] = ((s.v[2082] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));s.store_scalar(2255, if s.b[2255] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_offset_sub(781, 2082, 2127, 1e-12);s.store_square(722, 781);s.store_scalar(723, (1e-12 * 1e-12));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (tc0,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc0);
            let (tc1,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tc1);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2256] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });s.b[2257] = (2.0 == 1.0);s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });
            let (tc2,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && s.b[2257]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tc2);s.b[2258] = (2.0 == 2.0);s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });
            let (tc3,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tc3);s.b[2259] = (2.0 == 4.0);s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });
            let (tc4,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && s.b[2259]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tc4);s.b[2260] = (2.0 == 8.0);s.store_scalar(2260, if s.b[2260] { 1.0 } else { 0.0 });
            let (tc5,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && (!s.b[2259])) && s.b[2260]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tc5);
            let (tc6,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc6);let mut tca: usize = 0;
            while {
                let tc9: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tc9 != 0.0
            } {
                tca += 1;assert!(tca <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {s.store_sqrt(726, 726);}
                let (tc8,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
        let tc7: f64 = (s.v[719] + 1.0);
        (tc7,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, tc8);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && (!s.b[2256])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-12);s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);s.store_add_offset_lhs(2082, 2127, (-1e-12), 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {s.store_scalar(337, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(337, 336, 337);s.store_add_div_rhs_mixed_ai(2133, 2086, A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2082), A::sub_scaled_inputs(s.ad_value(2082), 1.0, s.ad_value(2127), 2.0), 1.0), 2132);s.store_scalar(2134, 1.0);s.store_mul_scale_offset_mixed_ai(2135, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2082)), s.ad_value(337), (-1.0)), 2137, -1.0, 1.0);}
            s.b[2261] = ((s.v[2133] > (s.v[2084] - p.p406)) && (p.p406 >= 0.0));s.store_scalar(2261, if s.b[2261] { 1.0 } else { 0.0 });
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_offset_sub(781, 2133, 2084, p.p406);s.store_square(722, 781);s.store_scalar(723, (p.p406 * p.p406));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (tcb,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tcb);
            let (tcc,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tcc);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[2262] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2262, if s.b[2262] { 1.0 } else { 0.0 });s.b[2263] = (4.0 == 1.0);s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
            let (tce,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && s.b[2263]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, tce);s.b[2264] = (4.0 == 2.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
            let (ta7,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && s.b[2264]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta7);s.b[2265] = (4.0 == 4.0);s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
            let (ta8,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && s.b[2265]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta8);s.b[2266] = (4.0 == 8.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });
            let (ta9,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && (!s.b[2265])) && s.b[2266]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, ta9);
            let (taa,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, taa);let mut tae: usize = 0;
            while {
                let tad: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tad != 0.0
            } {
                tae += 1;assert!(tae <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {s.store_sqrt(726, 726);}
                let (tac,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
        let tab: f64 = (s.v[719] + 1.0);
        (tab,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, tac);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && (!s.b[2262])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p406);s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);s.store_add_offset_lhs(2133, 2084, (-p.p406), 780);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {s.store_scalar(334, 1.0);}
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul(2134, 2134, 334);s.store_mul(2135, 2135, 334);s.store_mul_sub_rhs(339, 154, 2086, 2089);s.store_exp(340, 339);s.store_sub_offset_lhs(344, 340, (-1.0), 339);}
            s.b[2267] = (s.v[339] >= 1e-7);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
            let (tb0,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {
        let taf: f64 = (-1.0);
        (taf,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, tb0);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {s.store_mul_scaled_sqrt_rhs(2095, 209, -1.0, 344);s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_scale_offset_indices(2122, 345, 340, 1.0, (-1.0));s.store_mul_scale_offset_indices(2124, 345, 340, -1.0, 1.0);}
            s.b[2268] = (s.v[339] < (-1e-7));s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
            let (tb1,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, tb1);
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2086), 1.0, s.ad_value(2113), p.p398));s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2113), p.p398));s.store_mul_sqrt_mixed_ia(2095, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2095, 1.0);s.store_mul_add_mixed_iaa(2122, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));s.store_mul_mixed_ia(2124, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));}
            s.b[2269] = (s.v[339] > 0.0);s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && s.b[2269]) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && (!s.b[2269])) {s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);s.store_sqrt(2160, 2159);s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);s.store_neg(2124, 2122);}
            let (tb3,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] != 0.0)) {
        let tb2: f64 = (150.0 + 1.0);
        (tb2,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, tb3);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.store_add_scaled_product_right_sub(2096, 2095, 1.0, 185, 85, 2086, 1.0);s.store_sub(2097, 2122, 185);s.copy_ad(2098, 2124);s.store_sub(2099, 2089, 2133);s.store_neg(2100, 2134);s.store_sub_from_scalar(2101, 1.0, 2135);s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[2102] > 0.0) {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, (-1e-25));
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.copy_ad(2104, 2101);s.store_neg(2105, 2098);s.store_neg(2106, 2100);s.copy_ad(2107, 2097);s.store_mul_add_scaled_products_indices_rhs(2108, 2103, 2104, 2096, -1.0, 2105, 2099, -1.0);s.store_mul_add_scaled_products_indices_rhs(2109, 2103, 2106, 2096, -1.0, 2107, 2099, -1.0);s.store_abs(335, 2108);}
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                    s.store_abs(335, 2109);
                } else {
                }
            }
            s.b[2270] = (s.v[335] > 0.1);s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2270]) {s.store_mul_div_from_scalar_lhs_ad_indices(2108, 0.1, 335, 2108);s.store_mul_div_from_scalar_lhs_ad_indices(2109, 0.1, 335, 2109);}
            s.b[2271] = (s.v[335] < 1e-10);s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
            let (tb4,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2271]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, tb4);
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {s.store_add(2086, 2086, 2108);s.store_add(2089, 2089, 2109);}
            let (tb6,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        let tb5: f64 = (s.v[98] + 1.0);
        (tb5,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, tb6);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {s.store_mul_sub_rhs(339, 154, 2086, 2089);s.store_exp(340, 339);s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[2086] > s.v[2089]) {
                s.store_mul_scaled_sqrt_rhs(2119, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2119, 209, 344);
            }
        }
        s.b[2273] = (1.0 == 1.0);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });s.b[2274] = (((s.v[2086] - s.v[2084]) < p.p403) && (p.p403 >= 0.0));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2086), s.ad_value(2084)));s.store_square(722, 781);s.store_scalar(723, (p.p403 * p.p403));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (td2,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td2);
        let (td3,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td3);
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2275] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });s.b[2276] = (6.0 == 1.0);s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        let (td4,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && s.b[2276]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td4);s.b[2277] = (6.0 == 2.0);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        let (td5,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && s.b[2277]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td5);s.b[2278] = (6.0 == 4.0);s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        let (td6,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && s.b[2278]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td6);s.b[2279] = (6.0 == 8.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        let (td7,) = {
    if (((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (!s.b[2276])) && (!s.b[2277])) && (!s.b[2278])) && s.b[2279]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td7);
        let (td8,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td8);let mut tdc: usize = 0;
        while {
            let tdb: f64 = if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tdb != 0.0
        } {
            tdc += 1;assert!(tdc <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_sqrt(726, 726);}
            let (tda,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && s.b[2275]) {
        let td9: f64 = (s.v[719] + 1.0);
        (td9,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tda);
        }
        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p403);s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);s.store_sub_from_scalar(336, p.p403, 780);}
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && s.b[2274]) {
        }
        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) && (!s.b[2274])) {s.store_sub(336, 2086, 2084);s.store_scalar(334, 1.0);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2273]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2115, 209, -1.0, 338);}
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2273])) {s.copy_ad(2115, 2119);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.copy_ad(87, 2085);s.copy_ad(91, 2086);s.store_sub(94, 2086, 2085);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, (p.p263 * 0.1), 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[2280] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tdd,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tdd);
        let (tde,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tde);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2281] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });s.b[2282] = (2.0 == 1.0);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        let (tdf,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && s.b[2282]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdf);s.b[2283] = (2.0 == 2.0);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        let (te0,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && s.b[2283]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te0);s.b[2284] = (2.0 == 4.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        let (te1,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && s.b[2284]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te1);s.b[2285] = (2.0 == 8.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        let (te2,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (!s.b[2282])) && (!s.b[2283])) && (!s.b[2284])) && s.b[2285]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te2);
        let (te3,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te3);let mut te7: usize = 0;
        while {
            let te6: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te6 != 0.0
        } {
            te7 += 1;assert!(te7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {s.store_sqrt(726, 726);}
            let (te5,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && s.b[2281]) {
        let te4: f64 = (s.v[719] + 1.0);
        (te4,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, te5);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) && (!s.b[2281])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2280]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2280])) {s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_add(109, 87, 110);}
        s.b[2286] = (((s.v[109] - s.v[2083]) < p.p403) && (p.p403 >= 0.0));s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(109), s.ad_value(2083)));s.store_square(722, 781);s.store_scalar(723, (p.p403 * p.p403));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (te8,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, te8);
        let (te9,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te9);
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_scalar(770, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2287] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });s.b[2288] = (6.0 == 1.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });
        let (tea,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && s.b[2288]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tea);s.b[2289] = (6.0 == 2.0);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        let (teb,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && s.b[2289]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, teb);s.b[2290] = (6.0 == 4.0);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        let (tec,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && s.b[2290]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tec);s.b[2291] = (6.0 == 8.0);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        let (ted,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (!s.b[2288])) && (!s.b[2289])) && (!s.b[2290])) && s.b[2291]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ted);
        let (tee,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tee);let mut tf2: usize = 0;
        while {
            let tf1: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf1 != 0.0
        } {
            tf2 += 1;assert!(tf2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {s.store_sqrt(726, 726);}
            let (tf0,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && s.b[2287]) {
        let tef: f64 = (s.v[719] + 1.0);
        (tef,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tf0);
        }
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) && (!s.b[2287])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, p.p403);s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);s.store_sub_from_scalar(336, p.p403, 780);}
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2286]) {
        }
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2286])) {s.store_sub(336, 109, 2083);s.store_scalar(334, 1.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), 1e-15);s.store_mul_scaled_sqrt_rhs(2116, 209, -1.0, 338);s.store_sqrt_offset_ad(782, A::mul_scaled_lhs(A::add(s.ad_value(2115), s.ad_value(2114)), 1.0, A::add(s.ad_value(2115), s.ad_value(2114))), ((4.0 * (1e-12 * 1e-6)) * (1e-12 * 1e-6)));s.store_scaled_offset_ad(335, A::div_scaled_inputs2(s.ad_value(2115), -1.0, s.ad_value(2114), -1.0, s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(2117, 2115, (-0.5), 2114, (-0.5), 782, 0.5);}
        s.b[2292] = (s.v[2117] < 0.0);s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2292]) {s.store_scalar(2117, 0.0);s.store_scalar(335, 0.0);}
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {s.store_neg(2117, 2117);s.store_mul3_affine_lhs(248, 154, 2117, (-1.0 / (2.0)), 0.0, 94);s.store_neg(238, 2116);s.copy_ad(170, 162);s.copy_ad(790, 349);}
        s.b[2293] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2294] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tf3,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2293]) && s.b[2294]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tf3);
    }
}
