#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && (!s.b[1673])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(334, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1509, (-0.8), 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && (!s.b[1672])) {s.copy_ad(336, 1518);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_ad_affine_product_lhs(1503, s.ad_value(964), A::exp(A::mul(s.ad_value(154), A::sub(s.ad_value(336), s.ad_value(1509)))), (-1.6021918e-19), 0.0, 1445);}
        s.b[1678] = (((s.v[1459] - s.v[1509]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1459), s.ad_value(1509)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1679] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });s.b[1680] = (2.0 == 1.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && s.b[1680]) {s.store_scalar(720, 1.0);}
        s.b[1681] = (2.0 == 2.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && s.b[1681]) {s.store_scalar(720, 2.0);}
        s.b[1682] = (2.0 == 4.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && s.b[1682]) {s.store_scalar(720, 3.0);}
        s.b[1683] = (2.0 == 8.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (!s.b[1680])) && (!s.b[1681])) && (!s.b[1682])) && s.b[1683]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1678]) && s.b[1679]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1678]) && (!s.b[1679])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1678]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1678])) {s.store_sub(336, 1459, 1509);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1513, 209, -1.0, 338);s.store_sub_scaled_inputs_mixed_ai(338, A::offset(A::exp_scaled_input(s.ad_value(154), 0.1), (-1.0)), 1.0, 154, 0.1);s.store_mul_sqrt_rhs(1538, 209, 338);s.copy_ad(349, 790);}
        s.b[1684] = (s.v[790] > 1e-6);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_div_square_rhs(336, 1500, 185);s.store_add_scaled_inputs3_offset_indices(334, 85, 1.0, 155, (-1.0), 1436, -1.0, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1685] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1686] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });s.b[1687] = (2.0 == 1.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && s.b[1687]) {s.store_scalar(720, 1.0);}
        s.b[1688] = (2.0 == 2.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && s.b[1688]) {s.store_scalar(720, 2.0);}
        s.b[1689] = (2.0 == 4.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && s.b[1689]) {s.store_scalar(720, 3.0);}
        s.b[1690] = (2.0 == 8.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (!s.b[1687])) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && s.b[1686]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) && (!s.b[1686])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1685]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1685])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_offset_lhs(344, 85, 2.0, 338);}
        s.b[1691] = ((s.v[344] < (0.3 + 0.2)) && (0.2 >= 0.0));s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {s.store_sub_from_scalar(781, (0.3 + 0.2), 344);s.store_square(722, 781);s.store_scalar(723, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1692] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });s.b[1693] = (4.0 == 1.0);s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && s.b[1693]) {s.store_scalar(720, 1.0);}
        s.b[1694] = (4.0 == 2.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && s.b[1694]) {s.store_scalar(720, 2.0);}
        s.b[1695] = (4.0 == 4.0);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && (!s.b[1694])) && s.b[1695]) {s.store_scalar(720, 3.0);}
        s.b[1696] = (4.0 == 8.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (!s.b[1693])) && (!s.b[1694])) && (!s.b[1695])) && s.b[1696]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && s.b[1692]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) && (!s.b[1692])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_from_scalar(344, (0.3 + 0.2), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1691]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1691])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1691])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);}
        s.b[1697] = ((s.v[85] < 0.5) && (0.5 >= 0.0));s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {s.store_sub_from_scalar(781, 0.5, 85);s.store_square(722, 781);s.store_scalar(723, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1698] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });s.b[1699] = (2.0 == 1.0);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && s.b[1699]) {s.store_scalar(720, 1.0);}
        s.b[1700] = (2.0 == 2.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && s.b[1700]) {s.store_scalar(720, 2.0);}
        s.b[1701] = (2.0 == 4.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && (!s.b[1700])) && s.b[1701]) {s.store_scalar(720, 3.0);}
        s.b[1702] = (2.0 == 8.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (!s.b[1699])) && (!s.b[1700])) && (!s.b[1701])) && s.b[1702]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && s.b[1698]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) && (!s.b[1698])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.5);s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);s.store_sub_from_scalar(1535, 0.5, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1697]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1697])) {s.copy_ad(1535, 85);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1684]) {s.store_scale(335, 1535, 0.8);}
        s.b[1703] = ((s.v[348] > (s.v[1535] - s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1535, (-1.0), 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1704] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });s.b[1705] = (2.0 == 1.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && s.b[1705]) {s.store_scalar(720, 1.0);}
        s.b[1706] = (2.0 == 2.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && s.b[1706]) {s.store_scalar(720, 2.0);}
        s.b[1707] = (2.0 == 4.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1707]) {s.store_scalar(720, 3.0);}
        s.b[1708] = (2.0 == 8.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1707])) && s.b[1708]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) {s.store_scalar(719, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && s.b[1704]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) && (!s.b[1704])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(790, 1535, 1.0, 335, (-1.0), 780, 1.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && s.b[1703]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1684]) && (!s.b[1703])) {s.copy_ad(790, 348);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1684])) {s.copy_ad(348, 790);}
        s.b[1709] = (s.v[790] <= 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1709]) {s.copy_ad(1460, 1459);s.copy_ad(1482, 1481);s.copy_ad(1463, 1462);s.copy_ad(1476, 1475);s.copy_ad(1537, 1536);s.copy_ad(1497, 1495);s.copy_ad(1498, 1496);s.copy_ad(1516, 1515);s.copy_ad(1514, 1513);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_sqrt_mul_ad(1452, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));}
        s.b[1710] = (s.v[1452] > s.v[965]);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.copy_ad(1464, 790);s.copy_ad(1446, 965);s.copy_ad(1482, 790);s.copy_ad(1510, 790);s.store_sub_mixed_ia(1463, 1482, A::mul3(s.ad_value(1546), s.ad_value(1446), s.ad_value(1446)));s.copy_ad(1508, 1464);s.copy_ad(1471, 1463);s.store_mul(1497, 1446, 1544);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t12: usize = 0;
        while {
            let t10: f64 = (150.0 + 1.0);let t11: f64 = if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (s.v[97] <= t10)) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;
            if t12 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t12, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.store_sqrt_mul_sub_rhs(1446, 1545, 1482, 1463);}
            s.b[1711] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1712] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });s.b[1713] = (2.0 == 1.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && s.b[1713]) {s.store_scalar(720, 1.0);}
            s.b[1714] = (2.0 == 2.0);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && s.b[1714]) {s.store_scalar(720, 2.0);}
            s.b[1715] = (2.0 == 4.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && (!s.b[1714])) && s.b[1715]) {s.store_scalar(720, 3.0);}
            s.b[1716] = (2.0 == 8.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
            if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (!s.b[1713])) && (!s.b[1714])) && (!s.b[1715])) && s.b[1716]) {s.store_scalar(720, 4.0);}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) {s.store_scalar(719, 0.0);}
            let mut td: usize = 0;
            while {
                let tc: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tc != 0.0
            } {
                td += 1;
                if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && s.b[1712]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) && (!s.b[1712])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1711]) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1711])) {s.store_scalar(334, 1.0);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.store_add_scaled_inputs3_indices(335, 1463, 1.0, 1433, (-1.0), 1461, 1.0);}
            s.b[1717] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1718] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });s.b[1719] = (2.0 == 1.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && s.b[1719]) {s.store_scalar(720, 1.0);}
            s.b[1720] = (2.0 == 2.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && s.b[1720]) {s.store_scalar(720, 2.0);}
            s.b[1721] = (2.0 == 4.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && (!s.b[1720])) && s.b[1721]) {s.store_scalar(720, 3.0);}
            s.b[1722] = (2.0 == 8.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
            if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (!s.b[1719])) && (!s.b[1720])) && (!s.b[1721])) && s.b[1722]) {s.store_scalar(720, 4.0);}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) {s.store_scalar(719, 0.0);}
            let mut tf: usize = 0;
            while {
                let te: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                te != 0.0
            } {
                tf += 1;
                if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && s.b[1718]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) && (!s.b[1718])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1717]) {
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1717])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.store_sqrt_mul(1450, 1548, 336);s.store_mul(1497, 1446, 1544);s.store_mul_div_from_scalar_lhs_ad_indices(1528, (-1.034943e-10), 1446, 334);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1530, (-1.034943e-10), 1450, 341);s.store_add_mixed_ai(1483, A::add_scaled_product(s.ad_value(1497), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1482)), 1.0), 1498);s.copy_ad(1485, 185);s.store_add(1486, 1528, 1530);s.store_add_scaled_product_mixed_iia(1484, 1463, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), s.ad_value(1461)), (-1.0));s.store_scalar(1487, 0.0);s.store_scalar(1488, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1723] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1723]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1723]) {s.store_offset(1463, 1463, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1723])) {s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1463, 1463, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1724] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1463] - s.v[1471])) as f64).abs() <= 1e-12));s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1724]) {s.store_scalar(97, (150.0 + 1.0));}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.copy_ad(1508, 1464);s.copy_ad(1471, 1463);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_50(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) {s.copy_ad(1512, 1463);s.store_mul(1450, 965, 1534);s.store_add_scaled_inputs3_mixed_aii(1463, A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_scaled_product_indices(1482, 1463, 1.0, 1546, 1541, 1.0);s.copy_ad(1460, 1482);s.copy_ad(1465, 1482);s.copy_ad(1507, 1482);}
        s.b[1725] = (s.v[85] > s.v[1464]);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && s.b[1725]) {s.store_scalar(1477, 1.0);}
        s.b[1726] = (s.v[85] > s.v[1507]);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1725])) && s.b[1726]) {s.store_scalar(1477, 3.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1710]) && (!s.b[1725])) && (!s.b[1726])) {s.store_scalar(1477, 2.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) {s.copy_ad(1464, 790);s.copy_ad(1507, 1464);s.copy_ad(1465, 1464);s.copy_ad(1510, 1464);s.copy_ad(1446, 1452);s.store_mul(1450, 1446, 1534);s.store_add_scaled_inputs3_mixed_aii(1463, A::mul3(s.ad_value(1549), s.ad_value(1450), s.ad_value(1450)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_mixed_ai(1482, A::mul3(s.ad_value(1546), s.ad_value(1446), s.ad_value(1446)), 1463);s.copy_ad(1512, 1463);}
        s.b[1727] = (s.v[85] > s.v[1464]);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) && s.b[1727]) {s.store_scalar(1477, 1.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1710])) && (!s.b[1727])) {s.store_scalar(1477, 2.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1547, 1465, 1.0, 1433, -1.0, 961, 1.0, 0.0);}
        s.b[1728] = (s.v[335] > 0.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1728]) {s.store_add_scaled_inputs3_mixed_iia(1453, 1433, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1728])) {s.store_sub(1453, 1433, 961);}
        s.b[1729] = (s.v[85] > s.v[1464]);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1729]) {s.copy_ad(1463, 1512);s.copy_ad(1482, 790);s.store_add_div_lhs(1479, A::ln(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 790);}
        s.b[1730] = (s.v[1479] < (s.v[1510] + s.v[1551]));s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1729]) && s.b[1730]) {s.store_add(1479, 1510, 1551);}
        s.b[1731] = (s.v[85] > s.v[1507]);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && s.b[1731]) {s.copy_ad(1479, 1460);}
        s.b[1732] = (s.v[85] > s.v[1453]);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) {s.store_add_scaled_product_indices(1455, 154, 1.0, 1454, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1482, (-1.0));s.copy_ad(1469, 1482);s.store_div_scaled_inputs2_mixed_aii(1479, A::sqrt(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, 1455, (-0.5), 1454, 1.0);}
        s.b[1733] = (s.v[1479] > (s.v[1465] - s.v[1551]));s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1733]) {s.store_sub(1479, 1465, 1551);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) {s.store_sqrt_mul_sub_rhs(1448, 1545, 1482, 1479);s.store_sqrt_mul_sub_rhs(1446, 1545, 1482, 1463);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1734] = ((s.v[1448] + s.v[1446]) > s.v[965]);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {s.store_scalar(97, 1.0);}
        let mut t15: usize = 0;
        while {
            let t13: f64 = (150.0 + 1.0);let t14: f64 = if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (s.v[97] <= t13)) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {s.store_add_scaled_inputs3_indices(1466, 1448, 1.0, 1446, 1.0, 965, -1.0);s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1448), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1446)));}
            s.b[1735] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1735]) {s.store_offset(1482, 1482, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1735])) {s.store_sub_div_rhs_indices(1482, 1482, 1466, 1506);}
            s.b[1736] = (((s.v[1482] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1736]) {s.store_offset_sub(1482, 1433, 1461, (10.0 * 2.220446049250313e-16));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1482, (-1.0));s.store_add_scaled_square_product_indices(335, 1455, 1.0, 1454, 1456, (-4.0));}
            s.b[1737] = (s.v[335] > 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1737]) {s.store_div_scaled_inputs2_sqrt_first(1479, 335, 0.5, 1455, (-0.5), 1454, 1.0);}
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && (!s.b[1737])) {s.store_div_scaled_inputs_indices(1479, 1455, (-0.5), 1454, 1.0);}
            s.b[1738] = (s.v[1479] > s.v[1465]);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1738]) {s.copy_ad(1479, 1465);}
            s.b[1739] = (s.v[1479] > s.v[1482]);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1739]) {s.store_sub(1479, 1482, 1551);s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {s.store_sqrt_mul_sub_rhs(1448, 1545, 1482, 1479);s.store_div_scaled_inputs2_mixed_aia(1463, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1446, 1545, 1482, 1463);}
            s.b[1740] = ((((s.v[1482] - s.v[1469])) as f64).abs() <= 1e-8);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) && s.b[1740]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && s.b[1732]) && s.b[1734]) {s.copy_ad(1469, 1482);s.store_primal_offset(97, 97, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1729])) && (!s.b[1731])) && (!s.b[1732])) {s.copy_ad(1482, 1481);s.copy_ad(1463, 1462);s.copy_ad(1479, 1459);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.copy_ad(1480, 1482);s.store_scalar(79, 0.0);s.copy_ad(1460, 1479);s.copy_ad(1482, 1480);s.copy_ad(1472, 1460);s.copy_ad(1469, 1482);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t1e: usize = 0;
        while {
            let t1c: f64 = (150.0 + 1.0);let t1d: f64 = if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (s.v[97] <= t1c)) { 1.0 } else { 0.0 };
            t1d != 0.0
        } {
            t1e += 1;
            if t1e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_mul_sub_mixed_iai(1463, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), 1461);s.store_mul(1532, 1533, 1534);s.store_sub(335, 1482, 1463);}
            s.b[1741] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1742] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });s.b[1743] = (2.0 == 1.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && s.b[1743]) {s.store_scalar(720, 1.0);}
            s.b[1744] = (2.0 == 2.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && s.b[1744]) {s.store_scalar(720, 2.0);}
            s.b[1745] = (2.0 == 4.0);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && (!s.b[1744])) && s.b[1745]) {s.store_scalar(720, 3.0);}
            s.b[1746] = (2.0 == 8.0);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (!s.b[1743])) && (!s.b[1744])) && (!s.b[1745])) && s.b[1746]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) {s.store_scalar(719, 0.0);}
            let mut t19: usize = 0;
            while {
                let t18: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t18 != 0.0
            } {
                t19 += 1;
                if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && s.b[1742]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) && (!s.b[1742])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1741]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1741])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_sqrt_mul(1446, 1545, 336);}
            s.b[1747] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1748] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });s.b[1749] = (2.0 == 1.0);s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && s.b[1749]) {s.store_scalar(720, 1.0);}
            s.b[1750] = (2.0 == 2.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && s.b[1750]) {s.store_scalar(720, 2.0);}
            s.b[1751] = (2.0 == 4.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && (!s.b[1750])) && s.b[1751]) {s.store_scalar(720, 3.0);}
            s.b[1752] = (2.0 == 8.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (!s.b[1749])) && (!s.b[1750])) && (!s.b[1751])) && s.b[1752]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) {s.store_scalar(719, 0.0);}
            let mut t1b: usize = 0;
            while {
                let t1a: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t1a != 0.0
            } {
                t1b += 1;
                if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && s.b[1748]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) && (!s.b[1748])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1747]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1747])) {s.store_scalar(337, 1.0);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1497, 1446, 1544);s.store_mul_ad_product_lhs_mixed_ai(1526, A::div_from_scalar(1.034943e-10, s.ad_value(1446)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1528, A::div_from_scalar((-1.034943e-10), s.ad_value(1446)), 334, 337);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);s.store_div_from_scalar(1530, (-1.034943e-10), 1450);s.store_scaled_mul(335, 1500, 1541, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1518, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1463), s.ad_value(1540), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1540), s.ad_value(1460), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1460), s.ad_value(1460), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1463), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1460), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), 1541, 1.0, 335, 1.0);s.store_div_mixed_ai(1519, A::add_scaled_products3(s.ad_value(1463), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1460), (4.0 * 2.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_div_mixed_ai(1520, A::add_scaled_products3(s.ad_value(1463), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1460), (-8.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1460, 1482);s.store_exp(336, 335);}
            s.b[1753] = (s.v[1460] >= s.v[1482]);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1753]) {s.store_mul_scaled_sqrt_ad_rhs(1474, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1522, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1474, 1.0);s.store_neg(1524, 1522);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1753])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1474, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1474, 1.0);s.store_mul_add_mixed_iaa(1522, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1524, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1754] = ((s.v[1518] > (s.v[1510] - s.v[1517])) && (s.v[1517] >= 0.0));s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {s.store_add_scaled_inputs3_indices(781, 1518, 1.0, 1510, (-1.0), 1517, 1.0);s.store_square(722, 781);s.store_square(723, 1517);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1755] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });s.b[1756] = (4.0 == 1.0);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && s.b[1756]) {s.store_scalar(720, 1.0);}
            s.b[1757] = (4.0 == 2.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && s.b[1757]) {s.store_scalar(720, 2.0);}
            s.b[1758] = (4.0 == 4.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && (!s.b[1757])) && s.b[1758]) {s.store_scalar(720, 3.0);}
            s.b[1759] = (4.0 == 8.0);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (!s.b[1756])) && (!s.b[1757])) && (!s.b[1758])) && s.b[1759]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) {s.store_scalar(719, 0.0);}
            let mut t17: usize = 0;
            while {
                let t16: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t16 != 0.0
            } {
                t17 += 1;
                if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && s.b[1755]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) && (!s.b[1755])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1517, 726);s.store_div_scaled_product3_indices(334, 1517, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1510, 1.0, 1517, (-1.0), 780, 1.0);}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1754]) {
            }
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1754])) {s.copy_ad(335, 1518);s.store_scalar(334, 1.0);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_sub(1483, 1482, 335);s.store_mul_scale_offset_indices(1485, 334, 1519, -1.0, 0.0);s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1532), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1484, A::add_scaled_product(s.ad_value(1474), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1460)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);s.store_sub(1487, 1522, 185);s.store_add_scaled_inputs_products_indices(1488, 1524, 1.0, 1526, 1.0, 1528, 1532, 1.0, 1530, 1532, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1760] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1760]) {s.store_offset(1460, 1460, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1760]) {s.store_offset(1482, 1482, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1760])) {s.store_sub_mixed_ia(1460, 1460, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1482, 1482, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1761] = (((((s.v[1460] - s.v[1472])) as f64).abs() <= 1e-12) && ((((s.v[1482] - s.v[1469])) as f64).abs() <= 1e-12));s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1761]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.copy_ad(1472, 1460);s.copy_ad(1469, 1482);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1763] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });s.b[1764] = ((s.v[1482] > (s.v[1460] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {s.store_offset_sub(781, 1482, 1460, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1765] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });s.b[1766] = (2.0 == 1.0);s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && s.b[1766]) {s.store_scalar(720, 1.0);}
        s.b[1767] = (2.0 == 2.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && s.b[1767]) {s.store_scalar(720, 2.0);}
        s.b[1768] = (2.0 == 4.0);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && s.b[1768]) {s.store_scalar(720, 3.0);}
        s.b[1769] = (2.0 == 8.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (!s.b[1766])) && (!s.b[1767])) && (!s.b[1768])) && s.b[1769]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {s.store_scalar(719, 0.0);}
        let mut t20: usize = 0;
        while {
            let t1f: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1f != 0.0
        } {
            t20 += 1;
            if t20 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t20, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && s.b[1765]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) && (!s.b[1765])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1482, 1460, (-0.02), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && s.b[1764]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1763]) && (!s.b[1764])) {s.store_scalar(335, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_mul_sub_mixed_iai(1463, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1482), 1.0), 1461);s.store_mul_sub_rhs(335, 154, 1460, 1482);s.store_exp(336, 335);}
        s.b[1770] = (s.v[1460] >= s.v[1482]);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {s.store_mul_scaled_sqrt_ad_rhs(1474, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1537, 1474);s.store_scalar(1516, 0.0);s.store_scalar(1476, 0.0);s.store_sqrt_mul_sub_rhs(1446, 1545, 1482, 1463);}
        s.b[1771] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1772] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });s.b[1773] = (2.0 == 1.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && s.b[1773]) {s.store_scalar(720, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1774] = (2.0 == 2.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && s.b[1774]) {s.store_scalar(720, 2.0);}
        s.b[1775] = (2.0 == 4.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && s.b[1775]) {s.store_scalar(720, 3.0);}
        s.b[1776] = (2.0 == 8.0);s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (!s.b[1773])) && (!s.b[1774])) && (!s.b[1775])) && s.b[1776]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {s.store_scalar(719, 0.0);}
        let mut t22: usize = 0;
        while {
            let t21: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t21 != 0.0
        } {
            t22 += 1;
            if t22 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t22, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && s.b[1772]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) && (!s.b[1772])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && s.b[1771]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) && (!s.b[1771])) {s.store_scalar(337, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1770]) {s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1497, 1446, 1544);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1474, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1777] = ((s.v[1452] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1777]) {s.store_scalar(1476, 0.0);s.store_scalar(1516, 0.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1777])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1460), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1482), s.ad_value(1433)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1476, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1516, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_scalar(1537, 0.0);s.store_sub(335, 1482, 1463);}
        s.b[1778] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1779] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });s.b[1780] = (2.0 == 1.0);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && s.b[1780]) {s.store_scalar(720, 1.0);}
        s.b[1781] = (2.0 == 2.0);s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && s.b[1781]) {s.store_scalar(720, 2.0);}
        s.b[1782] = (2.0 == 4.0);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && s.b[1782]) {s.store_scalar(720, 3.0);}
        s.b[1783] = (2.0 == 8.0);s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (!s.b[1780])) && (!s.b[1781])) && (!s.b[1782])) && s.b[1783]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {s.store_scalar(719, 0.0);}
        let mut t24: usize = 0;
        while {
            let t23: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t23 != 0.0
        } {
            t24 += 1;
            if t24 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t24, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && s.b[1779]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) && (!s.b[1779])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1778]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1778])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_sqrt_mul(1446, 1545, 336);}
        s.b[1784] = ((s.v[1446] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {s.store_offset_sub(781, 1446, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1785] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });s.b[1786] = (2.0 == 1.0);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && s.b[1786]) {s.store_scalar(720, 1.0);}
        s.b[1787] = (2.0 == 2.0);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && s.b[1787]) {s.store_scalar(720, 2.0);}
        s.b[1788] = (2.0 == 4.0);s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && s.b[1788]) {s.store_scalar(720, 3.0);}
        s.b[1789] = (2.0 == 8.0);s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });
        if (((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (!s.b[1786])) && (!s.b[1787])) && (!s.b[1788])) && s.b[1789]) {s.store_scalar(720, 4.0);}
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {s.store_scalar(719, 0.0);}
        let mut t26: usize = 0;
        while {
            let t25: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t25 != 0.0
        } {
            t26 += 1;
            if t26 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t26, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && s.b[1785]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) && (!s.b[1785])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1446, 965, (-1e-8), 780);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && s.b[1784]) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) && (!s.b[1784])) {s.store_scalar(337, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1770])) {s.store_sqrt_mul_ad(1450, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1463), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1497, 1446, 1544);s.store_mul_scale_offset_indices(1498, 1542, 1450, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1790] = (((s.v[1460] - s.v[1510]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1460), s.ad_value(1510)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });s.b[1792] = (2.0 == 1.0);s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && s.b[1792]) {s.store_scalar(720, 1.0);}
        s.b[1793] = (2.0 == 2.0);s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && s.b[1793]) {s.store_scalar(720, 2.0);}
        s.b[1794] = (2.0 == 4.0);s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && s.b[1794]) {s.store_scalar(720, 3.0);}
        s.b[1795] = (2.0 == 8.0);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (!s.b[1792])) && (!s.b[1793])) && (!s.b[1794])) && s.b[1795]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {s.store_scalar(719, 0.0);}
        let mut t28: usize = 0;
        while {
            let t27: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t27 != 0.0
        } {
            t28 += 1;
            if t28 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t28, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && s.b[1791]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) && (!s.b[1791])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && s.b[1790]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1709])) && (!s.b[1790])) {s.store_sub(336, 1460, 1510);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1709])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1514, 209, -1.0, 338);}
        if (s.b[1441] && s.b[1442]) {s.copy_ad(87, 1459);s.copy_ad(91, 1460);s.store_sub(94, 1460, 1459);s.store_neg_add(335, 1473, 1474);}
        s.b[1796] = ((s.v[335] < s.v[1538]) && (s.v[1538] >= 0.0));s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {s.store_sub(781, 1538, 335);s.store_square(722, 781);s.store_square(723, 1538);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1797] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });s.b[1798] = (2.0 == 1.0);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && s.b[1798]) {s.store_scalar(720, 1.0);}
        s.b[1799] = (2.0 == 2.0);s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && s.b[1799]) {s.store_scalar(720, 2.0);}
        s.b[1800] = (2.0 == 4.0);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && s.b[1800]) {s.store_scalar(720, 3.0);}
        s.b[1801] = (2.0 == 8.0);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (!s.b[1798])) && (!s.b[1799])) && (!s.b[1800])) && s.b[1801]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2a: usize = 0;
        while {
            let t29: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t29 != 0.0
        } {
            t2a += 1;
            if t2a > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2a, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1796]) && s.b[1797]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1796]) && (!s.b[1797])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1538, 726);s.store_div_scaled_product3_indices(334, 1538, 725, 726, 1.0, 770, 1.0);s.store_sub(1554, 1538, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1796]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1796])) {s.copy_ad(1554, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul3_affine_lhs(1501, 154, 1554, 1.0 / (2.0), 0.0, 94);s.store_sub(1502, 1514, 1513);s.store_add(248, 1501, 1502);s.store_neg(133, 1513);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_mul(339, 336, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[160] - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(133), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);s.copy_ad(1556, 255);}
        s.b[1802] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1802]) {s.store_scalar(337, 1.0);}
        s.b[1803] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && s.b[1803]) {s.copy_ad(337, 335);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1802])) && (!s.b[1803])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p[178] - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[1804] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1804]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[1805] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && s.b[1805]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p[178]) - 1.0));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1804])) && (!s.b[1805])) {s.store_mul(339, 338, 340);}
        if (s.b[1441] && s.b[1442]) {s.store_mul(253, 254, 339);}
        s.b[1806] = (s.v[349] > 1e-6);s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_div_square_rhs(336, 1500, 185);s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1436, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1807] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });s.b[1809] = (2.0 == 1.0);s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {s.store_scalar(720, 1.0);}
        s.b[1810] = (2.0 == 2.0);s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && s.b[1810]) {s.store_scalar(720, 2.0);}
        s.b[1811] = (2.0 == 4.0);s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && s.b[1811]) {s.store_scalar(720, 3.0);}
        s.b[1812] = (2.0 == 8.0);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (!s.b[1809])) && (!s.b[1810])) && (!s.b[1811])) && s.b[1812]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {s.store_scalar(719, 0.0);}
        let mut t2c: usize = 0;
        while {
            let t2b: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2b != 0.0
        } {
            t2c += 1;
            if t2c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && s.b[1808]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) && (!s.b[1808])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1807]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1807])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);s.store_mul(344, 344, 975);}
        s.b[1813] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {s.store_sub_offset_lhs(781, 972, 4.0, 344);s.store_square(722, 781);s.store_scalar(723, (4.0 * 4.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1814] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });s.b[1815] = (4.0 == 1.0);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && s.b[1815]) {s.store_scalar(720, 1.0);}
        s.b[1816] = (4.0 == 2.0);s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && s.b[1816]) {s.store_scalar(720, 2.0);}
        s.b[1817] = (4.0 == 4.0);s.store_scalar(1817, if s.b[1817] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && s.b[1817]) {s.store_scalar(720, 3.0);}
        s.b[1818] = (4.0 == 8.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (!s.b[1815])) && (!s.b[1816])) && (!s.b[1817])) && s.b[1818]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t2e: usize = 0;
        while {
            let t2d: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2d != 0.0
        } {
            t2e += 1;
            if t2e > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2e, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && s.b[1814]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) && (!s.b[1814])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 4.0);s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);s.store_sub_offset_lhs(344, 972, 4.0, 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && s.b[1813]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1806]) && (!s.b[1813])) {s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_div(335, 349, 344);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1806]) {s.store_mul(340, 338, 337);s.store_div(1555, 349, 340);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1806])) {s.copy_ad(1555, 349);}
        if (s.b[1441] && s.b[1442]) {s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_neg(133, 1494);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[376] - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1553, 1555, 170);s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_div(1504, 254, 338);s.store_mul3_affine_lhs(987, 1494, 1504, (-s.v[632]), 0.0, 1553);s.store_offset_mul_ad(338, A::sub(s.ad_value(1460), s.ad_value(1459)), s.ad_value(682), 1.0);s.store_neg(133, 1503);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p[376] - 1.0));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && s.b[1442]) {s.store_div(1553, 1555, 170);s.store_div_scaled_product_indices(335, 254, 1553, 1.0, 973, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p[378]);
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1441] && s.b[1442]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p[378]));
            }
        }
        if (s.b[1441] && s.b[1442]) {s.store_div(1505, 254, 338);s.store_mul3_affine_lhs(1552, 1503, 1505, (-s.v[632]), 0.0, 1553);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1552, 1.0);s.store_mul3_lhs(986, 115, 248, 253);s.copy_ad(984, 253);s.copy_ad(790, 349);}
        s.b[1819] = (p[283] != 0.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1459), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[1820] = (s.v[336] < 0.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1819]) && s.b[1820]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1819]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p[284]);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1437, p[285], 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 1459, 1.0, 340, 1.0, 1436, -1.0);s.store_add_product3_rhs_indices(338, 338, 1437, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1819])) {s.store_scalar(343, 0.0);}
        s.b[1821] = (p[287] != 0.0);s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1821]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1437);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1821])) {s.store_scalar(342, 0.0);}
        s.b[1822] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(1822, if s.b[1822] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1822]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_add_product3_rhs_indices(135, 135, 115, 249, 253, 1.0);}
        s.b[1823] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(1823, if s.b[1823] { 1.0 } else { 0.0 });s.b[1824] = (p[296] > 0.0);s.store_scalar(1824, if s.b[1824] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1823]) && s.b[1824]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p[300]), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));}
    }
}
