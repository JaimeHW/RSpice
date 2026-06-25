#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_mul_ad_rhs(2551, 574, A::div(A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(589)), s.ad_value(2526)));
        }

        s.v[2692] = if (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2692] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(604)), s.ad_value(2551)));
        }

        s.v[2693] = if (((-s.v[604]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2692] != 0.0))) && (s.v[2693] != 0.0)) {
            let assign61390_ad_e79734: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign61390_ad_e79734);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2692] != 0.0))) && (!(s.v[2693] != 0.0))) {
            let assign61400_ad_e79785: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign61400_ad_e79785);
        }

        s.v[2694] = if (s.v[635] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2695] = if (s.v[2525] > ((-s.v[438]) * s.v[635])) { 1.0 } else { 0.0 };

        s.v[2696] = if (s.v[539] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) && (s.v[2696] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(610)), A::mul(s.ad_value(2525), s.ad_value(610))), A::mul(s.ad_value(2525), s.ad_value(610))), A::mul(s.ad_value(2525), s.ad_value(610)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) && (!(s.v[2696] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(539)));
        }

        s.v[2697] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            let assign61520_ad_e80010: A = {
                if (s.v[827] < s.v[544]) {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                            s.ad_value(544)
                        } else {
                            A::add(s.ad_value(544), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(827), s.ad_value(544)), s.ad_value(545))), 1.0)), s.ad_value(545)))
                        }
                    }
                } else {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                            s.ad_value(827)
                        } else {
                            A::add(s.ad_value(827), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(544), s.ad_value(827)), s.ad_value(545))), 1.0)), s.ad_value(545)))
                        }
                    }
                }
            };
            s.store_ad(2553, &assign61520_ad_e80010);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2698] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (s.v[2698] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (!(s.v[2698] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))), s.ad_value(571)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(1911, A::add(A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(583), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub_ad_lhs(2553, A::add(s.ad_value(827), s.ad_value(544)), 2553);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2699] = if (s.v[624] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (s.v[2699] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (!(s.v[2699] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))), s.ad_value(624)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(466, A::add(A::mul(s.ad_value(627), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(628), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(1911, 1911, 466);
        }

        s.v[2700] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) && (s.v[2700] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))), s.ad_value(571)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) {
            s.store_scale_ad(1911, A::add(A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        s.store_neg_ad(844, A::add(A::add(s.ad_value(845), s.ad_value(846)), s.ad_value(847)));

        s.store_add(848, 848, 1898);

        s.store_add(849, 849, 1899);

        s.store_add_ad(851, A::add(A::mul(s.ad_value(640), s.ad_value(1906)), A::mul(s.ad_value(641), s.ad_value(1907))), A::mul(s.ad_value(642), s.ad_value(1908)));

        s.store_add_ad(852, A::add(A::mul(s.ad_value(667), s.ad_value(1909)), A::mul(s.ad_value(668), s.ad_value(1910))), A::mul(s.ad_value(669), s.ad_value(1911)));

        s.v[2710] = if (s.v[825] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2710] != 0.0) {
            s.copy_ad(2709, 847);
        }

        if (s.v[2710] != 0.0) {
            s.copy_ad(847, 844);
        }

        if (s.v[2710] != 0.0) {
            s.copy_ad(844, 2709);
        }

        s.store_mul(854, 1892, 1883);

        s.v[2743] = if ((s.v[1817] > 0.0) && (s.v[710] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2748] = if ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_div_ad(854, A::mul(A::mul(A::square(s.ad_value(1896)), s.ad_value(1892)), s.ad_value(1883)), A::square(s.ad_value(1894)));
        }

        s.v[2752] = if (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2752] != 0.0) {
            s.store_div_ad_lhs(1920, A::scale(s.ad_value(1871), 4.0), 718);
        }

        if (s.v[2752] != 0.0) {
            s.store_scale(1920, 765, s.v[709]);
        }

        if (s.v[2752] != 0.0) {
            s.store_mul(1920, 1852, 1865);
        }

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e948, eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11,) = {
    if (s.v[2701] != 0.0) {
        let eq0_e942: f64 = (s.v[0] * s.v[15]);
        let eq0_e942_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq0_e942_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq0_e942_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq0_e942_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq0_e942_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq0_e942_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq0_e942_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq0_e942_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq0_e942_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq0_e942_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq0_e942_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq0_e942_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq0_e944: f64 = (eq0_e942 * p.p32);
        let eq0_e944_d_n0: f64 = (eq0_e942_d_n0 * p.p32);
        let eq0_e944_d_n1: f64 = (eq0_e942_d_n1 * p.p32);
        let eq0_e944_d_n2: f64 = (eq0_e942_d_n2 * p.p32);
        let eq0_e944_d_n3: f64 = (eq0_e942_d_n3 * p.p32);
        let eq0_e944_d_n4: f64 = (eq0_e942_d_n4 * p.p32);
        let eq0_e944_d_n5: f64 = (eq0_e942_d_n5 * p.p32);
        let eq0_e944_d_n6: f64 = (eq0_e942_d_n6 * p.p32);
        let eq0_e944_d_n7: f64 = (eq0_e942_d_n7 * p.p32);
        let eq0_e944_d_n8: f64 = (eq0_e942_d_n8 * p.p32);
        let eq0_e944_d_n9: f64 = (eq0_e942_d_n9 * p.p32);
        let eq0_e944_d_n10: f64 = (eq0_e942_d_n10 * p.p32);
        let eq0_e944_d_n11: f64 = (eq0_e942_d_n11 * p.p32);
        let eq0_e946: f64 = (eq0_e944 * s.v[841]);
        let eq0_e946_d_n0: f64 = ((eq0_e944_d_n0 * s.v[841]) + (eq0_e944 * s.dn[841][0]));
        let eq0_e946_d_n1: f64 = ((eq0_e944_d_n1 * s.v[841]) + (eq0_e944 * s.dn[841][1]));
        let eq0_e946_d_n2: f64 = ((eq0_e944_d_n2 * s.v[841]) + (eq0_e944 * s.dn[841][2]));
        let eq0_e946_d_n3: f64 = ((eq0_e944_d_n3 * s.v[841]) + (eq0_e944 * s.dn[841][3]));
        let eq0_e946_d_n4: f64 = ((eq0_e944_d_n4 * s.v[841]) + (eq0_e944 * s.dn[841][4]));
        let eq0_e946_d_n5: f64 = ((eq0_e944_d_n5 * s.v[841]) + (eq0_e944 * s.dn[841][5]));
        let eq0_e946_d_n6: f64 = ((eq0_e944_d_n6 * s.v[841]) + (eq0_e944 * s.dn[841][6]));
        let eq0_e946_d_n7: f64 = ((eq0_e944_d_n7 * s.v[841]) + (eq0_e944 * s.dn[841][7]));
        let eq0_e946_d_n8: f64 = ((eq0_e944_d_n8 * s.v[841]) + (eq0_e944 * s.dn[841][8]));
        let eq0_e946_d_n9: f64 = ((eq0_e944_d_n9 * s.v[841]) + (eq0_e944 * s.dn[841][9]));
        let eq0_e946_d_n10: f64 = ((eq0_e944_d_n10 * s.v[841]) + (eq0_e944 * s.dn[841][10]));
        let eq0_e946_d_n11: f64 = ((eq0_e944_d_n11 * s.v[841]) + (eq0_e944 * s.dn[841][11]));
        (eq0_e946, eq0_e946_d_n0, eq0_e946_d_n1, eq0_e946_d_n2, eq0_e946_d_n3, eq0_e946_d_n4, eq0_e946_d_n5, eq0_e946_d_n6, eq0_e946_d_n7, eq0_e946_d_n8, eq0_e946_d_n9, eq0_e946_d_n10, eq0_e946_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e948;
        let eq0_node_derivatives: [f64; 12] = [eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e960, eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11,) = {
    if (s.v[2701] != 0.0) {
        let eq1_e952: f64 = (s.v[0] * s.v[15]);
        let eq1_e952_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq1_e952_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq1_e952_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq1_e952_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq1_e952_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq1_e952_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq1_e952_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq1_e952_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq1_e952_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq1_e952_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq1_e952_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq1_e952_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq1_e954: f64 = (eq1_e952 * p.p32);
        let eq1_e954_d_n0: f64 = (eq1_e952_d_n0 * p.p32);
        let eq1_e954_d_n1: f64 = (eq1_e952_d_n1 * p.p32);
        let eq1_e954_d_n2: f64 = (eq1_e952_d_n2 * p.p32);
        let eq1_e954_d_n3: f64 = (eq1_e952_d_n3 * p.p32);
        let eq1_e954_d_n4: f64 = (eq1_e952_d_n4 * p.p32);
        let eq1_e954_d_n5: f64 = (eq1_e952_d_n5 * p.p32);
        let eq1_e954_d_n6: f64 = (eq1_e952_d_n6 * p.p32);
        let eq1_e954_d_n7: f64 = (eq1_e952_d_n7 * p.p32);
        let eq1_e954_d_n8: f64 = (eq1_e952_d_n8 * p.p32);
        let eq1_e954_d_n9: f64 = (eq1_e952_d_n9 * p.p32);
        let eq1_e954_d_n10: f64 = (eq1_e952_d_n10 * p.p32);
        let eq1_e954_d_n11: f64 = (eq1_e952_d_n11 * p.p32);
        let eq1_e957: f64 = (s.v[832] + s.v[840]);
        let eq1_e957_d_n0: f64 = (s.dn[832][0] + s.dn[840][0]);
        let eq1_e957_d_n1: f64 = (s.dn[832][1] + s.dn[840][1]);
        let eq1_e957_d_n2: f64 = (s.dn[832][2] + s.dn[840][2]);
        let eq1_e957_d_n3: f64 = (s.dn[832][3] + s.dn[840][3]);
        let eq1_e957_d_n4: f64 = (s.dn[832][4] + s.dn[840][4]);
        let eq1_e957_d_n5: f64 = (s.dn[832][5] + s.dn[840][5]);
        let eq1_e957_d_n6: f64 = (s.dn[832][6] + s.dn[840][6]);
        let eq1_e957_d_n7: f64 = (s.dn[832][7] + s.dn[840][7]);
        let eq1_e957_d_n8: f64 = (s.dn[832][8] + s.dn[840][8]);
        let eq1_e957_d_n9: f64 = (s.dn[832][9] + s.dn[840][9]);
        let eq1_e957_d_n10: f64 = (s.dn[832][10] + s.dn[840][10]);
        let eq1_e957_d_n11: f64 = (s.dn[832][11] + s.dn[840][11]);
        let eq1_e958: f64 = (eq1_e954 * eq1_e957);
        let eq1_e958_d_n0: f64 = ((eq1_e954_d_n0 * eq1_e957) + (eq1_e954 * eq1_e957_d_n0));
        let eq1_e958_d_n1: f64 = ((eq1_e954_d_n1 * eq1_e957) + (eq1_e954 * eq1_e957_d_n1));
        let eq1_e958_d_n2: f64 = ((eq1_e954_d_n2 * eq1_e957) + (eq1_e954 * eq1_e957_d_n2));
        let eq1_e958_d_n3: f64 = ((eq1_e954_d_n3 * eq1_e957) + (eq1_e954 * eq1_e957_d_n3));
        let eq1_e958_d_n4: f64 = ((eq1_e954_d_n4 * eq1_e957) + (eq1_e954 * eq1_e957_d_n4));
        let eq1_e958_d_n5: f64 = ((eq1_e954_d_n5 * eq1_e957) + (eq1_e954 * eq1_e957_d_n5));
        let eq1_e958_d_n6: f64 = ((eq1_e954_d_n6 * eq1_e957) + (eq1_e954 * eq1_e957_d_n6));
        let eq1_e958_d_n7: f64 = ((eq1_e954_d_n7 * eq1_e957) + (eq1_e954 * eq1_e957_d_n7));
        let eq1_e958_d_n8: f64 = ((eq1_e954_d_n8 * eq1_e957) + (eq1_e954 * eq1_e957_d_n8));
        let eq1_e958_d_n9: f64 = ((eq1_e954_d_n9 * eq1_e957) + (eq1_e954 * eq1_e957_d_n9));
        let eq1_e958_d_n10: f64 = ((eq1_e954_d_n10 * eq1_e957) + (eq1_e954 * eq1_e957_d_n10));
        let eq1_e958_d_n11: f64 = ((eq1_e954_d_n11 * eq1_e957) + (eq1_e954 * eq1_e957_d_n11));
        (eq1_e958, eq1_e958_d_n0, eq1_e958_d_n1, eq1_e958_d_n2, eq1_e958_d_n3, eq1_e958_d_n4, eq1_e958_d_n5, eq1_e958_d_n6, eq1_e958_d_n7, eq1_e958_d_n8, eq1_e958_d_n9, eq1_e958_d_n10, eq1_e958_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e960;
        let eq1_node_derivatives: [f64; 12] = [eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq1_value),
            &nodes,
            &eq1_node_derivatives,
            &branches,
            &eq1_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e970, eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11,) = {
    if (s.v[2701] != 0.0) {
        let eq2_e964: f64 = (s.v[0] * s.v[15]);
        let eq2_e964_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq2_e964_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq2_e964_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq2_e964_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq2_e964_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq2_e964_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq2_e964_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq2_e964_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq2_e964_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq2_e964_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq2_e964_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq2_e964_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq2_e966: f64 = (eq2_e964 * p.p32);
        let eq2_e966_d_n0: f64 = (eq2_e964_d_n0 * p.p32);
        let eq2_e966_d_n1: f64 = (eq2_e964_d_n1 * p.p32);
        let eq2_e966_d_n2: f64 = (eq2_e964_d_n2 * p.p32);
        let eq2_e966_d_n3: f64 = (eq2_e964_d_n3 * p.p32);
        let eq2_e966_d_n4: f64 = (eq2_e964_d_n4 * p.p32);
        let eq2_e966_d_n5: f64 = (eq2_e964_d_n5 * p.p32);
        let eq2_e966_d_n6: f64 = (eq2_e964_d_n6 * p.p32);
        let eq2_e966_d_n7: f64 = (eq2_e964_d_n7 * p.p32);
        let eq2_e966_d_n8: f64 = (eq2_e964_d_n8 * p.p32);
        let eq2_e966_d_n9: f64 = (eq2_e964_d_n9 * p.p32);
        let eq2_e966_d_n10: f64 = (eq2_e964_d_n10 * p.p32);
        let eq2_e966_d_n11: f64 = (eq2_e964_d_n11 * p.p32);
        let eq2_e968: f64 = (eq2_e966 * s.v[835]);
        let eq2_e968_d_n0: f64 = ((eq2_e966_d_n0 * s.v[835]) + (eq2_e966 * s.dn[835][0]));
        let eq2_e968_d_n1: f64 = ((eq2_e966_d_n1 * s.v[835]) + (eq2_e966 * s.dn[835][1]));
        let eq2_e968_d_n2: f64 = ((eq2_e966_d_n2 * s.v[835]) + (eq2_e966 * s.dn[835][2]));
        let eq2_e968_d_n3: f64 = ((eq2_e966_d_n3 * s.v[835]) + (eq2_e966 * s.dn[835][3]));
        let eq2_e968_d_n4: f64 = ((eq2_e966_d_n4 * s.v[835]) + (eq2_e966 * s.dn[835][4]));
        let eq2_e968_d_n5: f64 = ((eq2_e966_d_n5 * s.v[835]) + (eq2_e966 * s.dn[835][5]));
        let eq2_e968_d_n6: f64 = ((eq2_e966_d_n6 * s.v[835]) + (eq2_e966 * s.dn[835][6]));
        let eq2_e968_d_n7: f64 = ((eq2_e966_d_n7 * s.v[835]) + (eq2_e966 * s.dn[835][7]));
        let eq2_e968_d_n8: f64 = ((eq2_e966_d_n8 * s.v[835]) + (eq2_e966 * s.dn[835][8]));
        let eq2_e968_d_n9: f64 = ((eq2_e966_d_n9 * s.v[835]) + (eq2_e966 * s.dn[835][9]));
        let eq2_e968_d_n10: f64 = ((eq2_e966_d_n10 * s.v[835]) + (eq2_e966 * s.dn[835][10]));
        let eq2_e968_d_n11: f64 = ((eq2_e966_d_n11 * s.v[835]) + (eq2_e966 * s.dn[835][11]));
        (eq2_e968, eq2_e968_d_n0, eq2_e968_d_n1, eq2_e968_d_n2, eq2_e968_d_n3, eq2_e968_d_n4, eq2_e968_d_n5, eq2_e968_d_n6, eq2_e968_d_n7, eq2_e968_d_n8, eq2_e968_d_n9, eq2_e968_d_n10, eq2_e968_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e970;
        let eq2_node_derivatives: [f64; 12] = [eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e980, eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11,) = {
    if (s.v[2701] != 0.0) {
        let eq3_e974: f64 = (s.v[0] * s.v[15]);
        let eq3_e974_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq3_e974_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq3_e974_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq3_e974_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq3_e974_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq3_e974_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq3_e974_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq3_e974_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq3_e974_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq3_e974_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq3_e974_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq3_e974_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq3_e976: f64 = (eq3_e974 * p.p32);
        let eq3_e976_d_n0: f64 = (eq3_e974_d_n0 * p.p32);
        let eq3_e976_d_n1: f64 = (eq3_e974_d_n1 * p.p32);
        let eq3_e976_d_n2: f64 = (eq3_e974_d_n2 * p.p32);
        let eq3_e976_d_n3: f64 = (eq3_e974_d_n3 * p.p32);
        let eq3_e976_d_n4: f64 = (eq3_e974_d_n4 * p.p32);
        let eq3_e976_d_n5: f64 = (eq3_e974_d_n5 * p.p32);
        let eq3_e976_d_n6: f64 = (eq3_e974_d_n6 * p.p32);
        let eq3_e976_d_n7: f64 = (eq3_e974_d_n7 * p.p32);
        let eq3_e976_d_n8: f64 = (eq3_e974_d_n8 * p.p32);
        let eq3_e976_d_n9: f64 = (eq3_e974_d_n9 * p.p32);
        let eq3_e976_d_n10: f64 = (eq3_e974_d_n10 * p.p32);
        let eq3_e976_d_n11: f64 = (eq3_e974_d_n11 * p.p32);
        let eq3_e978: f64 = (eq3_e976 * s.v[836]);
        let eq3_e978_d_n0: f64 = ((eq3_e976_d_n0 * s.v[836]) + (eq3_e976 * s.dn[836][0]));
        let eq3_e978_d_n1: f64 = ((eq3_e976_d_n1 * s.v[836]) + (eq3_e976 * s.dn[836][1]));
        let eq3_e978_d_n2: f64 = ((eq3_e976_d_n2 * s.v[836]) + (eq3_e976 * s.dn[836][2]));
        let eq3_e978_d_n3: f64 = ((eq3_e976_d_n3 * s.v[836]) + (eq3_e976 * s.dn[836][3]));
        let eq3_e978_d_n4: f64 = ((eq3_e976_d_n4 * s.v[836]) + (eq3_e976 * s.dn[836][4]));
        let eq3_e978_d_n5: f64 = ((eq3_e976_d_n5 * s.v[836]) + (eq3_e976 * s.dn[836][5]));
        let eq3_e978_d_n6: f64 = ((eq3_e976_d_n6 * s.v[836]) + (eq3_e976 * s.dn[836][6]));
        let eq3_e978_d_n7: f64 = ((eq3_e976_d_n7 * s.v[836]) + (eq3_e976 * s.dn[836][7]));
        let eq3_e978_d_n8: f64 = ((eq3_e976_d_n8 * s.v[836]) + (eq3_e976 * s.dn[836][8]));
        let eq3_e978_d_n9: f64 = ((eq3_e976_d_n9 * s.v[836]) + (eq3_e976 * s.dn[836][9]));
        let eq3_e978_d_n10: f64 = ((eq3_e976_d_n10 * s.v[836]) + (eq3_e976 * s.dn[836][10]));
        let eq3_e978_d_n11: f64 = ((eq3_e976_d_n11 * s.v[836]) + (eq3_e976 * s.dn[836][11]));
        (eq3_e978, eq3_e978_d_n0, eq3_e978_d_n1, eq3_e978_d_n2, eq3_e978_d_n3, eq3_e978_d_n4, eq3_e978_d_n5, eq3_e978_d_n6, eq3_e978_d_n7, eq3_e978_d_n8, eq3_e978_d_n9, eq3_e978_d_n10, eq3_e978_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e980;
        let eq3_node_derivatives: [f64; 12] = [eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e991, eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11,) = {
    if (!(s.v[2701] != 0.0)) {
        let eq4_e985: f64 = (s.v[0] * s.v[15]);
        let eq4_e985_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq4_e985_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq4_e985_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq4_e985_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq4_e985_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq4_e985_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq4_e985_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq4_e985_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq4_e985_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq4_e985_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq4_e985_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq4_e985_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq4_e987: f64 = (eq4_e985 * p.p32);
        let eq4_e987_d_n0: f64 = (eq4_e985_d_n0 * p.p32);
        let eq4_e987_d_n1: f64 = (eq4_e985_d_n1 * p.p32);
        let eq4_e987_d_n2: f64 = (eq4_e985_d_n2 * p.p32);
        let eq4_e987_d_n3: f64 = (eq4_e985_d_n3 * p.p32);
        let eq4_e987_d_n4: f64 = (eq4_e985_d_n4 * p.p32);
        let eq4_e987_d_n5: f64 = (eq4_e985_d_n5 * p.p32);
        let eq4_e987_d_n6: f64 = (eq4_e985_d_n6 * p.p32);
        let eq4_e987_d_n7: f64 = (eq4_e985_d_n7 * p.p32);
        let eq4_e987_d_n8: f64 = (eq4_e985_d_n8 * p.p32);
        let eq4_e987_d_n9: f64 = (eq4_e985_d_n9 * p.p32);
        let eq4_e987_d_n10: f64 = (eq4_e985_d_n10 * p.p32);
        let eq4_e987_d_n11: f64 = (eq4_e985_d_n11 * p.p32);
        let eq4_e989: f64 = (eq4_e987 * s.v[841]);
        let eq4_e989_d_n0: f64 = ((eq4_e987_d_n0 * s.v[841]) + (eq4_e987 * s.dn[841][0]));
        let eq4_e989_d_n1: f64 = ((eq4_e987_d_n1 * s.v[841]) + (eq4_e987 * s.dn[841][1]));
        let eq4_e989_d_n2: f64 = ((eq4_e987_d_n2 * s.v[841]) + (eq4_e987 * s.dn[841][2]));
        let eq4_e989_d_n3: f64 = ((eq4_e987_d_n3 * s.v[841]) + (eq4_e987 * s.dn[841][3]));
        let eq4_e989_d_n4: f64 = ((eq4_e987_d_n4 * s.v[841]) + (eq4_e987 * s.dn[841][4]));
        let eq4_e989_d_n5: f64 = ((eq4_e987_d_n5 * s.v[841]) + (eq4_e987 * s.dn[841][5]));
        let eq4_e989_d_n6: f64 = ((eq4_e987_d_n6 * s.v[841]) + (eq4_e987 * s.dn[841][6]));
        let eq4_e989_d_n7: f64 = ((eq4_e987_d_n7 * s.v[841]) + (eq4_e987 * s.dn[841][7]));
        let eq4_e989_d_n8: f64 = ((eq4_e987_d_n8 * s.v[841]) + (eq4_e987 * s.dn[841][8]));
        let eq4_e989_d_n9: f64 = ((eq4_e987_d_n9 * s.v[841]) + (eq4_e987 * s.dn[841][9]));
        let eq4_e989_d_n10: f64 = ((eq4_e987_d_n10 * s.v[841]) + (eq4_e987 * s.dn[841][10]));
        let eq4_e989_d_n11: f64 = ((eq4_e987_d_n11 * s.v[841]) + (eq4_e987 * s.dn[841][11]));
        (eq4_e989, eq4_e989_d_n0, eq4_e989_d_n1, eq4_e989_d_n2, eq4_e989_d_n3, eq4_e989_d_n4, eq4_e989_d_n5, eq4_e989_d_n6, eq4_e989_d_n7, eq4_e989_d_n8, eq4_e989_d_n9, eq4_e989_d_n10, eq4_e989_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e991;
        let eq4_node_derivatives: [f64; 12] = [eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1004, eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11,) = {
    if (!(s.v[2701] != 0.0)) {
        let eq5_e996: f64 = (s.v[0] * s.v[15]);
        let eq5_e996_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq5_e996_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq5_e996_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq5_e996_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq5_e996_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq5_e996_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq5_e996_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq5_e996_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq5_e996_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq5_e996_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq5_e996_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq5_e996_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq5_e998: f64 = (eq5_e996 * p.p32);
        let eq5_e998_d_n0: f64 = (eq5_e996_d_n0 * p.p32);
        let eq5_e998_d_n1: f64 = (eq5_e996_d_n1 * p.p32);
        let eq5_e998_d_n2: f64 = (eq5_e996_d_n2 * p.p32);
        let eq5_e998_d_n3: f64 = (eq5_e996_d_n3 * p.p32);
        let eq5_e998_d_n4: f64 = (eq5_e996_d_n4 * p.p32);
        let eq5_e998_d_n5: f64 = (eq5_e996_d_n5 * p.p32);
        let eq5_e998_d_n6: f64 = (eq5_e996_d_n6 * p.p32);
        let eq5_e998_d_n7: f64 = (eq5_e996_d_n7 * p.p32);
        let eq5_e998_d_n8: f64 = (eq5_e996_d_n8 * p.p32);
        let eq5_e998_d_n9: f64 = (eq5_e996_d_n9 * p.p32);
        let eq5_e998_d_n10: f64 = (eq5_e996_d_n10 * p.p32);
        let eq5_e998_d_n11: f64 = (eq5_e996_d_n11 * p.p32);
        let eq5_e1001: f64 = (s.v[832] + s.v[840]);
        let eq5_e1001_d_n0: f64 = (s.dn[832][0] + s.dn[840][0]);
        let eq5_e1001_d_n1: f64 = (s.dn[832][1] + s.dn[840][1]);
        let eq5_e1001_d_n2: f64 = (s.dn[832][2] + s.dn[840][2]);
        let eq5_e1001_d_n3: f64 = (s.dn[832][3] + s.dn[840][3]);
        let eq5_e1001_d_n4: f64 = (s.dn[832][4] + s.dn[840][4]);
        let eq5_e1001_d_n5: f64 = (s.dn[832][5] + s.dn[840][5]);
        let eq5_e1001_d_n6: f64 = (s.dn[832][6] + s.dn[840][6]);
        let eq5_e1001_d_n7: f64 = (s.dn[832][7] + s.dn[840][7]);
        let eq5_e1001_d_n8: f64 = (s.dn[832][8] + s.dn[840][8]);
        let eq5_e1001_d_n9: f64 = (s.dn[832][9] + s.dn[840][9]);
        let eq5_e1001_d_n10: f64 = (s.dn[832][10] + s.dn[840][10]);
        let eq5_e1001_d_n11: f64 = (s.dn[832][11] + s.dn[840][11]);
        let eq5_e1002: f64 = (eq5_e998 * eq5_e1001);
        let eq5_e1002_d_n0: f64 = ((eq5_e998_d_n0 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n0));
        let eq5_e1002_d_n1: f64 = ((eq5_e998_d_n1 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n1));
        let eq5_e1002_d_n2: f64 = ((eq5_e998_d_n2 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n2));
        let eq5_e1002_d_n3: f64 = ((eq5_e998_d_n3 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n3));
        let eq5_e1002_d_n4: f64 = ((eq5_e998_d_n4 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n4));
        let eq5_e1002_d_n5: f64 = ((eq5_e998_d_n5 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n5));
        let eq5_e1002_d_n6: f64 = ((eq5_e998_d_n6 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n6));
        let eq5_e1002_d_n7: f64 = ((eq5_e998_d_n7 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n7));
        let eq5_e1002_d_n8: f64 = ((eq5_e998_d_n8 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n8));
        let eq5_e1002_d_n9: f64 = ((eq5_e998_d_n9 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n9));
        let eq5_e1002_d_n10: f64 = ((eq5_e998_d_n10 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n10));
        let eq5_e1002_d_n11: f64 = ((eq5_e998_d_n11 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n11));
        (eq5_e1002, eq5_e1002_d_n0, eq5_e1002_d_n1, eq5_e1002_d_n2, eq5_e1002_d_n3, eq5_e1002_d_n4, eq5_e1002_d_n5, eq5_e1002_d_n6, eq5_e1002_d_n7, eq5_e1002_d_n8, eq5_e1002_d_n9, eq5_e1002_d_n10, eq5_e1002_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1004;
        let eq5_node_derivatives: [f64; 12] = [eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq6_e1015, eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11,) = {
    if (!(s.v[2701] != 0.0)) {
        let eq6_e1009: f64 = (s.v[0] * s.v[15]);
        let eq6_e1009_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq6_e1009_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq6_e1009_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq6_e1009_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq6_e1009_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq6_e1009_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq6_e1009_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq6_e1009_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq6_e1009_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq6_e1009_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq6_e1009_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq6_e1009_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq6_e1011: f64 = (eq6_e1009 * p.p32);
        let eq6_e1011_d_n0: f64 = (eq6_e1009_d_n0 * p.p32);
        let eq6_e1011_d_n1: f64 = (eq6_e1009_d_n1 * p.p32);
        let eq6_e1011_d_n2: f64 = (eq6_e1009_d_n2 * p.p32);
        let eq6_e1011_d_n3: f64 = (eq6_e1009_d_n3 * p.p32);
        let eq6_e1011_d_n4: f64 = (eq6_e1009_d_n4 * p.p32);
        let eq6_e1011_d_n5: f64 = (eq6_e1009_d_n5 * p.p32);
        let eq6_e1011_d_n6: f64 = (eq6_e1009_d_n6 * p.p32);
        let eq6_e1011_d_n7: f64 = (eq6_e1009_d_n7 * p.p32);
        let eq6_e1011_d_n8: f64 = (eq6_e1009_d_n8 * p.p32);
        let eq6_e1011_d_n9: f64 = (eq6_e1009_d_n9 * p.p32);
        let eq6_e1011_d_n10: f64 = (eq6_e1009_d_n10 * p.p32);
        let eq6_e1011_d_n11: f64 = (eq6_e1009_d_n11 * p.p32);
        let eq6_e1013: f64 = (eq6_e1011 * s.v[835]);
        let eq6_e1013_d_n0: f64 = ((eq6_e1011_d_n0 * s.v[835]) + (eq6_e1011 * s.dn[835][0]));
        let eq6_e1013_d_n1: f64 = ((eq6_e1011_d_n1 * s.v[835]) + (eq6_e1011 * s.dn[835][1]));
        let eq6_e1013_d_n2: f64 = ((eq6_e1011_d_n2 * s.v[835]) + (eq6_e1011 * s.dn[835][2]));
        let eq6_e1013_d_n3: f64 = ((eq6_e1011_d_n3 * s.v[835]) + (eq6_e1011 * s.dn[835][3]));
        let eq6_e1013_d_n4: f64 = ((eq6_e1011_d_n4 * s.v[835]) + (eq6_e1011 * s.dn[835][4]));
        let eq6_e1013_d_n5: f64 = ((eq6_e1011_d_n5 * s.v[835]) + (eq6_e1011 * s.dn[835][5]));
        let eq6_e1013_d_n6: f64 = ((eq6_e1011_d_n6 * s.v[835]) + (eq6_e1011 * s.dn[835][6]));
        let eq6_e1013_d_n7: f64 = ((eq6_e1011_d_n7 * s.v[835]) + (eq6_e1011 * s.dn[835][7]));
        let eq6_e1013_d_n8: f64 = ((eq6_e1011_d_n8 * s.v[835]) + (eq6_e1011 * s.dn[835][8]));
        let eq6_e1013_d_n9: f64 = ((eq6_e1011_d_n9 * s.v[835]) + (eq6_e1011 * s.dn[835][9]));
        let eq6_e1013_d_n10: f64 = ((eq6_e1011_d_n10 * s.v[835]) + (eq6_e1011 * s.dn[835][10]));
        let eq6_e1013_d_n11: f64 = ((eq6_e1011_d_n11 * s.v[835]) + (eq6_e1011 * s.dn[835][11]));
        (eq6_e1013, eq6_e1013_d_n0, eq6_e1013_d_n1, eq6_e1013_d_n2, eq6_e1013_d_n3, eq6_e1013_d_n4, eq6_e1013_d_n5, eq6_e1013_d_n6, eq6_e1013_d_n7, eq6_e1013_d_n8, eq6_e1013_d_n9, eq6_e1013_d_n10, eq6_e1013_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1015;
        let eq6_node_derivatives: [f64; 12] = [eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e1026, eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11,) = {
    if (!(s.v[2701] != 0.0)) {
        let eq7_e1020: f64 = (s.v[0] * s.v[15]);
        let eq7_e1020_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq7_e1020_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq7_e1020_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq7_e1020_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq7_e1020_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq7_e1020_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq7_e1020_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq7_e1020_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq7_e1020_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq7_e1020_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq7_e1020_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq7_e1020_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq7_e1022: f64 = (eq7_e1020 * p.p32);
        let eq7_e1022_d_n0: f64 = (eq7_e1020_d_n0 * p.p32);
        let eq7_e1022_d_n1: f64 = (eq7_e1020_d_n1 * p.p32);
        let eq7_e1022_d_n2: f64 = (eq7_e1020_d_n2 * p.p32);
        let eq7_e1022_d_n3: f64 = (eq7_e1020_d_n3 * p.p32);
        let eq7_e1022_d_n4: f64 = (eq7_e1020_d_n4 * p.p32);
        let eq7_e1022_d_n5: f64 = (eq7_e1020_d_n5 * p.p32);
        let eq7_e1022_d_n6: f64 = (eq7_e1020_d_n6 * p.p32);
        let eq7_e1022_d_n7: f64 = (eq7_e1020_d_n7 * p.p32);
        let eq7_e1022_d_n8: f64 = (eq7_e1020_d_n8 * p.p32);
        let eq7_e1022_d_n9: f64 = (eq7_e1020_d_n9 * p.p32);
        let eq7_e1022_d_n10: f64 = (eq7_e1020_d_n10 * p.p32);
        let eq7_e1022_d_n11: f64 = (eq7_e1020_d_n11 * p.p32);
        let eq7_e1024: f64 = (eq7_e1022 * s.v[836]);
        let eq7_e1024_d_n0: f64 = ((eq7_e1022_d_n0 * s.v[836]) + (eq7_e1022 * s.dn[836][0]));
        let eq7_e1024_d_n1: f64 = ((eq7_e1022_d_n1 * s.v[836]) + (eq7_e1022 * s.dn[836][1]));
        let eq7_e1024_d_n2: f64 = ((eq7_e1022_d_n2 * s.v[836]) + (eq7_e1022 * s.dn[836][2]));
        let eq7_e1024_d_n3: f64 = ((eq7_e1022_d_n3 * s.v[836]) + (eq7_e1022 * s.dn[836][3]));
        let eq7_e1024_d_n4: f64 = ((eq7_e1022_d_n4 * s.v[836]) + (eq7_e1022 * s.dn[836][4]));
        let eq7_e1024_d_n5: f64 = ((eq7_e1022_d_n5 * s.v[836]) + (eq7_e1022 * s.dn[836][5]));
        let eq7_e1024_d_n6: f64 = ((eq7_e1022_d_n6 * s.v[836]) + (eq7_e1022 * s.dn[836][6]));
        let eq7_e1024_d_n7: f64 = ((eq7_e1022_d_n7 * s.v[836]) + (eq7_e1022 * s.dn[836][7]));
        let eq7_e1024_d_n8: f64 = ((eq7_e1022_d_n8 * s.v[836]) + (eq7_e1022 * s.dn[836][8]));
        let eq7_e1024_d_n9: f64 = ((eq7_e1022_d_n9 * s.v[836]) + (eq7_e1022 * s.dn[836][9]));
        let eq7_e1024_d_n10: f64 = ((eq7_e1022_d_n10 * s.v[836]) + (eq7_e1022 * s.dn[836][10]));
        let eq7_e1024_d_n11: f64 = ((eq7_e1022_d_n11 * s.v[836]) + (eq7_e1022 * s.dn[836][11]));
        (eq7_e1024, eq7_e1024_d_n0, eq7_e1024_d_n1, eq7_e1024_d_n2, eq7_e1024_d_n3, eq7_e1024_d_n4, eq7_e1024_d_n5, eq7_e1024_d_n6, eq7_e1024_d_n7, eq7_e1024_d_n8, eq7_e1024_d_n9, eq7_e1024_d_n10, eq7_e1024_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1026;
        let eq7_node_derivatives: [f64; 12] = [eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq8_e1029: f64 = (s.v[0] * s.v[15]);
        let eq8_e1029_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq8_e1029_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq8_e1029_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq8_e1029_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq8_e1029_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq8_e1029_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq8_e1029_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq8_e1029_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq8_e1029_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq8_e1029_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq8_e1029_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq8_e1029_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq8_e1031: f64 = (eq8_e1029 * p.p32);
        let eq8_e1031_d_n0: f64 = (eq8_e1029_d_n0 * p.p32);
        let eq8_e1031_d_n1: f64 = (eq8_e1029_d_n1 * p.p32);
        let eq8_e1031_d_n2: f64 = (eq8_e1029_d_n2 * p.p32);
        let eq8_e1031_d_n3: f64 = (eq8_e1029_d_n3 * p.p32);
        let eq8_e1031_d_n4: f64 = (eq8_e1029_d_n4 * p.p32);
        let eq8_e1031_d_n5: f64 = (eq8_e1029_d_n5 * p.p32);
        let eq8_e1031_d_n6: f64 = (eq8_e1029_d_n6 * p.p32);
        let eq8_e1031_d_n7: f64 = (eq8_e1029_d_n7 * p.p32);
        let eq8_e1031_d_n8: f64 = (eq8_e1029_d_n8 * p.p32);
        let eq8_e1031_d_n9: f64 = (eq8_e1029_d_n9 * p.p32);
        let eq8_e1031_d_n10: f64 = (eq8_e1029_d_n10 * p.p32);
        let eq8_e1031_d_n11: f64 = (eq8_e1029_d_n11 * p.p32);
        let eq8_e1033: f64 = (eq8_e1031 * s.v[837]);
        let eq8_e1033_d_n0: f64 = ((eq8_e1031_d_n0 * s.v[837]) + (eq8_e1031 * s.dn[837][0]));
        let eq8_e1033_d_n1: f64 = ((eq8_e1031_d_n1 * s.v[837]) + (eq8_e1031 * s.dn[837][1]));
        let eq8_e1033_d_n2: f64 = ((eq8_e1031_d_n2 * s.v[837]) + (eq8_e1031 * s.dn[837][2]));
        let eq8_e1033_d_n3: f64 = ((eq8_e1031_d_n3 * s.v[837]) + (eq8_e1031 * s.dn[837][3]));
        let eq8_e1033_d_n4: f64 = ((eq8_e1031_d_n4 * s.v[837]) + (eq8_e1031 * s.dn[837][4]));
        let eq8_e1033_d_n5: f64 = ((eq8_e1031_d_n5 * s.v[837]) + (eq8_e1031 * s.dn[837][5]));
        let eq8_e1033_d_n6: f64 = ((eq8_e1031_d_n6 * s.v[837]) + (eq8_e1031 * s.dn[837][6]));
        let eq8_e1033_d_n7: f64 = ((eq8_e1031_d_n7 * s.v[837]) + (eq8_e1031 * s.dn[837][7]));
        let eq8_e1033_d_n8: f64 = ((eq8_e1031_d_n8 * s.v[837]) + (eq8_e1031 * s.dn[837][8]));
        let eq8_e1033_d_n9: f64 = ((eq8_e1031_d_n9 * s.v[837]) + (eq8_e1031 * s.dn[837][9]));
        let eq8_e1033_d_n10: f64 = ((eq8_e1031_d_n10 * s.v[837]) + (eq8_e1031 * s.dn[837][10]));
        let eq8_e1033_d_n11: f64 = ((eq8_e1031_d_n11 * s.v[837]) + (eq8_e1031 * s.dn[837][11]));
        let eq8_value: f64 = eq8_e1033;
        let eq8_node_derivatives: [f64; 12] = [eq8_e1033_d_n0, eq8_e1033_d_n1, eq8_e1033_d_n2, eq8_e1033_d_n3, eq8_e1033_d_n4, eq8_e1033_d_n5, eq8_e1033_d_n6, eq8_e1033_d_n7, eq8_e1033_d_n8, eq8_e1033_d_n9, eq8_e1033_d_n10, eq8_e1033_d_n11];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_e1036: f64 = (s.v[0] * s.v[15]);
        let eq9_e1036_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq9_e1036_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq9_e1036_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq9_e1036_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq9_e1036_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq9_e1036_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq9_e1036_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq9_e1036_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq9_e1036_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq9_e1036_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq9_e1036_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq9_e1036_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq9_e1038: f64 = (eq9_e1036 * p.p32);
        let eq9_e1038_d_n0: f64 = (eq9_e1036_d_n0 * p.p32);
        let eq9_e1038_d_n1: f64 = (eq9_e1036_d_n1 * p.p32);
        let eq9_e1038_d_n2: f64 = (eq9_e1036_d_n2 * p.p32);
        let eq9_e1038_d_n3: f64 = (eq9_e1036_d_n3 * p.p32);
        let eq9_e1038_d_n4: f64 = (eq9_e1036_d_n4 * p.p32);
        let eq9_e1038_d_n5: f64 = (eq9_e1036_d_n5 * p.p32);
        let eq9_e1038_d_n6: f64 = (eq9_e1036_d_n6 * p.p32);
        let eq9_e1038_d_n7: f64 = (eq9_e1036_d_n7 * p.p32);
        let eq9_e1038_d_n8: f64 = (eq9_e1036_d_n8 * p.p32);
        let eq9_e1038_d_n9: f64 = (eq9_e1036_d_n9 * p.p32);
        let eq9_e1038_d_n10: f64 = (eq9_e1036_d_n10 * p.p32);
        let eq9_e1038_d_n11: f64 = (eq9_e1036_d_n11 * p.p32);
        let eq9_e1040: f64 = (eq9_e1038 * s.v[833]);
        let eq9_e1040_d_n0: f64 = ((eq9_e1038_d_n0 * s.v[833]) + (eq9_e1038 * s.dn[833][0]));
        let eq9_e1040_d_n1: f64 = ((eq9_e1038_d_n1 * s.v[833]) + (eq9_e1038 * s.dn[833][1]));
        let eq9_e1040_d_n2: f64 = ((eq9_e1038_d_n2 * s.v[833]) + (eq9_e1038 * s.dn[833][2]));
        let eq9_e1040_d_n3: f64 = ((eq9_e1038_d_n3 * s.v[833]) + (eq9_e1038 * s.dn[833][3]));
        let eq9_e1040_d_n4: f64 = ((eq9_e1038_d_n4 * s.v[833]) + (eq9_e1038 * s.dn[833][4]));
        let eq9_e1040_d_n5: f64 = ((eq9_e1038_d_n5 * s.v[833]) + (eq9_e1038 * s.dn[833][5]));
        let eq9_e1040_d_n6: f64 = ((eq9_e1038_d_n6 * s.v[833]) + (eq9_e1038 * s.dn[833][6]));
        let eq9_e1040_d_n7: f64 = ((eq9_e1038_d_n7 * s.v[833]) + (eq9_e1038 * s.dn[833][7]));
        let eq9_e1040_d_n8: f64 = ((eq9_e1038_d_n8 * s.v[833]) + (eq9_e1038 * s.dn[833][8]));
        let eq9_e1040_d_n9: f64 = ((eq9_e1038_d_n9 * s.v[833]) + (eq9_e1038 * s.dn[833][9]));
        let eq9_e1040_d_n10: f64 = ((eq9_e1038_d_n10 * s.v[833]) + (eq9_e1038 * s.dn[833][10]));
        let eq9_e1040_d_n11: f64 = ((eq9_e1038_d_n11 * s.v[833]) + (eq9_e1038 * s.dn[833][11]));
        let eq9_value: f64 = eq9_e1040;
        let eq9_node_derivatives: [f64; 12] = [eq9_e1040_d_n0, eq9_e1040_d_n1, eq9_e1040_d_n2, eq9_e1040_d_n3, eq9_e1040_d_n4, eq9_e1040_d_n5, eq9_e1040_d_n6, eq9_e1040_d_n7, eq9_e1040_d_n8, eq9_e1040_d_n9, eq9_e1040_d_n10, eq9_e1040_d_n11];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e1043: f64 = (s.v[0] * s.v[15]);
        let eq10_e1043_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq10_e1043_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq10_e1043_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq10_e1043_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq10_e1043_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq10_e1043_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq10_e1043_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq10_e1043_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq10_e1043_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq10_e1043_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq10_e1043_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq10_e1043_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq10_e1045: f64 = (eq10_e1043 * p.p32);
        let eq10_e1045_d_n0: f64 = (eq10_e1043_d_n0 * p.p32);
        let eq10_e1045_d_n1: f64 = (eq10_e1043_d_n1 * p.p32);
        let eq10_e1045_d_n2: f64 = (eq10_e1043_d_n2 * p.p32);
        let eq10_e1045_d_n3: f64 = (eq10_e1043_d_n3 * p.p32);
        let eq10_e1045_d_n4: f64 = (eq10_e1043_d_n4 * p.p32);
        let eq10_e1045_d_n5: f64 = (eq10_e1043_d_n5 * p.p32);
        let eq10_e1045_d_n6: f64 = (eq10_e1043_d_n6 * p.p32);
        let eq10_e1045_d_n7: f64 = (eq10_e1043_d_n7 * p.p32);
        let eq10_e1045_d_n8: f64 = (eq10_e1043_d_n8 * p.p32);
        let eq10_e1045_d_n9: f64 = (eq10_e1043_d_n9 * p.p32);
        let eq10_e1045_d_n10: f64 = (eq10_e1043_d_n10 * p.p32);
        let eq10_e1045_d_n11: f64 = (eq10_e1043_d_n11 * p.p32);
        let eq10_e1047: f64 = (eq10_e1045 * s.v[834]);
        let eq10_e1047_d_n0: f64 = ((eq10_e1045_d_n0 * s.v[834]) + (eq10_e1045 * s.dn[834][0]));
        let eq10_e1047_d_n1: f64 = ((eq10_e1045_d_n1 * s.v[834]) + (eq10_e1045 * s.dn[834][1]));
        let eq10_e1047_d_n2: f64 = ((eq10_e1045_d_n2 * s.v[834]) + (eq10_e1045 * s.dn[834][2]));
        let eq10_e1047_d_n3: f64 = ((eq10_e1045_d_n3 * s.v[834]) + (eq10_e1045 * s.dn[834][3]));
        let eq10_e1047_d_n4: f64 = ((eq10_e1045_d_n4 * s.v[834]) + (eq10_e1045 * s.dn[834][4]));
        let eq10_e1047_d_n5: f64 = ((eq10_e1045_d_n5 * s.v[834]) + (eq10_e1045 * s.dn[834][5]));
        let eq10_e1047_d_n6: f64 = ((eq10_e1045_d_n6 * s.v[834]) + (eq10_e1045 * s.dn[834][6]));
        let eq10_e1047_d_n7: f64 = ((eq10_e1045_d_n7 * s.v[834]) + (eq10_e1045 * s.dn[834][7]));
        let eq10_e1047_d_n8: f64 = ((eq10_e1045_d_n8 * s.v[834]) + (eq10_e1045 * s.dn[834][8]));
        let eq10_e1047_d_n9: f64 = ((eq10_e1045_d_n9 * s.v[834]) + (eq10_e1045 * s.dn[834][9]));
        let eq10_e1047_d_n10: f64 = ((eq10_e1045_d_n10 * s.v[834]) + (eq10_e1045 * s.dn[834][10]));
        let eq10_e1047_d_n11: f64 = ((eq10_e1045_d_n11 * s.v[834]) + (eq10_e1045 * s.dn[834][11]));
        let eq10_value: f64 = eq10_e1047;
        let eq10_node_derivatives: [f64; 12] = [eq10_e1047_d_n0, eq10_e1047_d_n1, eq10_e1047_d_n2, eq10_e1047_d_n3, eq10_e1047_d_n4, eq10_e1047_d_n5, eq10_e1047_d_n6, eq10_e1047_d_n7, eq10_e1047_d_n8, eq10_e1047_d_n9, eq10_e1047_d_n10, eq10_e1047_d_n11];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e1050: f64 = (s.v[0] * s.v[15]);
        let eq11_e1050_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq11_e1050_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq11_e1050_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq11_e1050_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq11_e1050_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq11_e1050_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq11_e1050_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq11_e1050_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq11_e1050_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq11_e1050_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq11_e1050_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq11_e1050_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq11_e1052: f64 = (eq11_e1050 * p.p32);
        let eq11_e1052_d_n0: f64 = (eq11_e1050_d_n0 * p.p32);
        let eq11_e1052_d_n1: f64 = (eq11_e1050_d_n1 * p.p32);
        let eq11_e1052_d_n2: f64 = (eq11_e1050_d_n2 * p.p32);
        let eq11_e1052_d_n3: f64 = (eq11_e1050_d_n3 * p.p32);
        let eq11_e1052_d_n4: f64 = (eq11_e1050_d_n4 * p.p32);
        let eq11_e1052_d_n5: f64 = (eq11_e1050_d_n5 * p.p32);
        let eq11_e1052_d_n6: f64 = (eq11_e1050_d_n6 * p.p32);
        let eq11_e1052_d_n7: f64 = (eq11_e1050_d_n7 * p.p32);
        let eq11_e1052_d_n8: f64 = (eq11_e1050_d_n8 * p.p32);
        let eq11_e1052_d_n9: f64 = (eq11_e1050_d_n9 * p.p32);
        let eq11_e1052_d_n10: f64 = (eq11_e1050_d_n10 * p.p32);
        let eq11_e1052_d_n11: f64 = (eq11_e1050_d_n11 * p.p32);
        let eq11_e1054: f64 = (eq11_e1052 * s.v[838]);
        let eq11_e1054_d_n0: f64 = ((eq11_e1052_d_n0 * s.v[838]) + (eq11_e1052 * s.dn[838][0]));
        let eq11_e1054_d_n1: f64 = ((eq11_e1052_d_n1 * s.v[838]) + (eq11_e1052 * s.dn[838][1]));
        let eq11_e1054_d_n2: f64 = ((eq11_e1052_d_n2 * s.v[838]) + (eq11_e1052 * s.dn[838][2]));
        let eq11_e1054_d_n3: f64 = ((eq11_e1052_d_n3 * s.v[838]) + (eq11_e1052 * s.dn[838][3]));
        let eq11_e1054_d_n4: f64 = ((eq11_e1052_d_n4 * s.v[838]) + (eq11_e1052 * s.dn[838][4]));
        let eq11_e1054_d_n5: f64 = ((eq11_e1052_d_n5 * s.v[838]) + (eq11_e1052 * s.dn[838][5]));
        let eq11_e1054_d_n6: f64 = ((eq11_e1052_d_n6 * s.v[838]) + (eq11_e1052 * s.dn[838][6]));
        let eq11_e1054_d_n7: f64 = ((eq11_e1052_d_n7 * s.v[838]) + (eq11_e1052 * s.dn[838][7]));
        let eq11_e1054_d_n8: f64 = ((eq11_e1052_d_n8 * s.v[838]) + (eq11_e1052 * s.dn[838][8]));
        let eq11_e1054_d_n9: f64 = ((eq11_e1052_d_n9 * s.v[838]) + (eq11_e1052 * s.dn[838][9]));
        let eq11_e1054_d_n10: f64 = ((eq11_e1052_d_n10 * s.v[838]) + (eq11_e1052 * s.dn[838][10]));
        let eq11_e1054_d_n11: f64 = ((eq11_e1052_d_n11 * s.v[838]) + (eq11_e1052 * s.dn[838][11]));
        let eq11_value: f64 = eq11_e1054;
        let eq11_node_derivatives: [f64; 12] = [eq11_e1054_d_n0, eq11_e1054_d_n1, eq11_e1054_d_n2, eq11_e1054_d_n3, eq11_e1054_d_n4, eq11_e1054_d_n5, eq11_e1054_d_n6, eq11_e1054_d_n7, eq11_e1054_d_n8, eq11_e1054_d_n9, eq11_e1054_d_n10, eq11_e1054_d_n11];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq12_e1057: f64 = (s.v[0] * s.v[15]);
        let eq12_e1057_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq12_e1057_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq12_e1057_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq12_e1057_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq12_e1057_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq12_e1057_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq12_e1057_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq12_e1057_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq12_e1057_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq12_e1057_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq12_e1057_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq12_e1057_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq12_e1059: f64 = (eq12_e1057 * p.p32);
        let eq12_e1059_d_n0: f64 = (eq12_e1057_d_n0 * p.p32);
        let eq12_e1059_d_n1: f64 = (eq12_e1057_d_n1 * p.p32);
        let eq12_e1059_d_n2: f64 = (eq12_e1057_d_n2 * p.p32);
        let eq12_e1059_d_n3: f64 = (eq12_e1057_d_n3 * p.p32);
        let eq12_e1059_d_n4: f64 = (eq12_e1057_d_n4 * p.p32);
        let eq12_e1059_d_n5: f64 = (eq12_e1057_d_n5 * p.p32);
        let eq12_e1059_d_n6: f64 = (eq12_e1057_d_n6 * p.p32);
        let eq12_e1059_d_n7: f64 = (eq12_e1057_d_n7 * p.p32);
        let eq12_e1059_d_n8: f64 = (eq12_e1057_d_n8 * p.p32);
        let eq12_e1059_d_n9: f64 = (eq12_e1057_d_n9 * p.p32);
        let eq12_e1059_d_n10: f64 = (eq12_e1057_d_n10 * p.p32);
        let eq12_e1059_d_n11: f64 = (eq12_e1057_d_n11 * p.p32);
        let eq12_e1061: f64 = (eq12_e1059 * s.v[839]);
        let eq12_e1061_d_n0: f64 = ((eq12_e1059_d_n0 * s.v[839]) + (eq12_e1059 * s.dn[839][0]));
        let eq12_e1061_d_n1: f64 = ((eq12_e1059_d_n1 * s.v[839]) + (eq12_e1059 * s.dn[839][1]));
        let eq12_e1061_d_n2: f64 = ((eq12_e1059_d_n2 * s.v[839]) + (eq12_e1059 * s.dn[839][2]));
        let eq12_e1061_d_n3: f64 = ((eq12_e1059_d_n3 * s.v[839]) + (eq12_e1059 * s.dn[839][3]));
        let eq12_e1061_d_n4: f64 = ((eq12_e1059_d_n4 * s.v[839]) + (eq12_e1059 * s.dn[839][4]));
        let eq12_e1061_d_n5: f64 = ((eq12_e1059_d_n5 * s.v[839]) + (eq12_e1059 * s.dn[839][5]));
        let eq12_e1061_d_n6: f64 = ((eq12_e1059_d_n6 * s.v[839]) + (eq12_e1059 * s.dn[839][6]));
        let eq12_e1061_d_n7: f64 = ((eq12_e1059_d_n7 * s.v[839]) + (eq12_e1059 * s.dn[839][7]));
        let eq12_e1061_d_n8: f64 = ((eq12_e1059_d_n8 * s.v[839]) + (eq12_e1059 * s.dn[839][8]));
        let eq12_e1061_d_n9: f64 = ((eq12_e1059_d_n9 * s.v[839]) + (eq12_e1059 * s.dn[839][9]));
        let eq12_e1061_d_n10: f64 = ((eq12_e1059_d_n10 * s.v[839]) + (eq12_e1059 * s.dn[839][10]));
        let eq12_e1061_d_n11: f64 = ((eq12_e1059_d_n11 * s.v[839]) + (eq12_e1059 * s.dn[839][11]));
        let eq12_value: f64 = eq12_e1061;
        let eq12_node_derivatives: [f64; 12] = [eq12_e1061_d_n0, eq12_e1061_d_n1, eq12_e1061_d_n2, eq12_e1061_d_n3, eq12_e1061_d_n4, eq12_e1061_d_n5, eq12_e1061_d_n6, eq12_e1061_d_n7, eq12_e1061_d_n8, eq12_e1061_d_n9, eq12_e1061_d_n10, eq12_e1061_d_n11];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e1064: f64 = (s.v[0] * s.v[15]);
        let eq13_e1064_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq13_e1064_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq13_e1064_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq13_e1064_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq13_e1064_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq13_e1064_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq13_e1064_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq13_e1064_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq13_e1064_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq13_e1064_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq13_e1064_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq13_e1064_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq13_e1066: f64 = (eq13_e1064 * p.p32);
        let eq13_e1066_d_n0: f64 = (eq13_e1064_d_n0 * p.p32);
        let eq13_e1066_d_n1: f64 = (eq13_e1064_d_n1 * p.p32);
        let eq13_e1066_d_n2: f64 = (eq13_e1064_d_n2 * p.p32);
        let eq13_e1066_d_n3: f64 = (eq13_e1064_d_n3 * p.p32);
        let eq13_e1066_d_n4: f64 = (eq13_e1064_d_n4 * p.p32);
        let eq13_e1066_d_n5: f64 = (eq13_e1064_d_n5 * p.p32);
        let eq13_e1066_d_n6: f64 = (eq13_e1064_d_n6 * p.p32);
        let eq13_e1066_d_n7: f64 = (eq13_e1064_d_n7 * p.p32);
        let eq13_e1066_d_n8: f64 = (eq13_e1064_d_n8 * p.p32);
        let eq13_e1066_d_n9: f64 = (eq13_e1064_d_n9 * p.p32);
        let eq13_e1066_d_n10: f64 = (eq13_e1064_d_n10 * p.p32);
        let eq13_e1066_d_n11: f64 = (eq13_e1064_d_n11 * p.p32);
        let eq13_e1068: f64 = (eq13_e1066 * s.v[842]);
        let eq13_e1068_d_n0: f64 = ((eq13_e1066_d_n0 * s.v[842]) + (eq13_e1066 * s.dn[842][0]));
        let eq13_e1068_d_n1: f64 = ((eq13_e1066_d_n1 * s.v[842]) + (eq13_e1066 * s.dn[842][1]));
        let eq13_e1068_d_n2: f64 = ((eq13_e1066_d_n2 * s.v[842]) + (eq13_e1066 * s.dn[842][2]));
        let eq13_e1068_d_n3: f64 = ((eq13_e1066_d_n3 * s.v[842]) + (eq13_e1066 * s.dn[842][3]));
        let eq13_e1068_d_n4: f64 = ((eq13_e1066_d_n4 * s.v[842]) + (eq13_e1066 * s.dn[842][4]));
        let eq13_e1068_d_n5: f64 = ((eq13_e1066_d_n5 * s.v[842]) + (eq13_e1066 * s.dn[842][5]));
        let eq13_e1068_d_n6: f64 = ((eq13_e1066_d_n6 * s.v[842]) + (eq13_e1066 * s.dn[842][6]));
        let eq13_e1068_d_n7: f64 = ((eq13_e1066_d_n7 * s.v[842]) + (eq13_e1066 * s.dn[842][7]));
        let eq13_e1068_d_n8: f64 = ((eq13_e1066_d_n8 * s.v[842]) + (eq13_e1066 * s.dn[842][8]));
        let eq13_e1068_d_n9: f64 = ((eq13_e1066_d_n9 * s.v[842]) + (eq13_e1066 * s.dn[842][9]));
        let eq13_e1068_d_n10: f64 = ((eq13_e1066_d_n10 * s.v[842]) + (eq13_e1066 * s.dn[842][10]));
        let eq13_e1068_d_n11: f64 = ((eq13_e1066_d_n11 * s.v[842]) + (eq13_e1066 * s.dn[842][11]));
        let eq13_value: f64 = eq13_e1068;
        let eq13_node_derivatives: [f64; 12] = [eq13_e1068_d_n0, eq13_e1068_d_n1, eq13_e1068_d_n2, eq13_e1068_d_n3, eq13_e1068_d_n4, eq13_e1068_d_n5, eq13_e1068_d_n6, eq13_e1068_d_n7, eq13_e1068_d_n8, eq13_e1068_d_n9, eq13_e1068_d_n10, eq13_e1068_d_n11];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq14_e1071: f64 = (s.v[0] * s.v[15]);
        let eq14_e1071_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq14_e1071_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq14_e1071_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq14_e1071_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq14_e1071_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq14_e1071_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq14_e1071_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq14_e1071_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq14_e1071_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq14_e1071_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq14_e1071_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq14_e1071_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq14_e1073: f64 = (eq14_e1071 * p.p32);
        let eq14_e1073_d_n0: f64 = (eq14_e1071_d_n0 * p.p32);
        let eq14_e1073_d_n1: f64 = (eq14_e1071_d_n1 * p.p32);
        let eq14_e1073_d_n2: f64 = (eq14_e1071_d_n2 * p.p32);
        let eq14_e1073_d_n3: f64 = (eq14_e1071_d_n3 * p.p32);
        let eq14_e1073_d_n4: f64 = (eq14_e1071_d_n4 * p.p32);
        let eq14_e1073_d_n5: f64 = (eq14_e1071_d_n5 * p.p32);
        let eq14_e1073_d_n6: f64 = (eq14_e1071_d_n6 * p.p32);
        let eq14_e1073_d_n7: f64 = (eq14_e1071_d_n7 * p.p32);
        let eq14_e1073_d_n8: f64 = (eq14_e1071_d_n8 * p.p32);
        let eq14_e1073_d_n9: f64 = (eq14_e1071_d_n9 * p.p32);
        let eq14_e1073_d_n10: f64 = (eq14_e1071_d_n10 * p.p32);
        let eq14_e1073_d_n11: f64 = (eq14_e1071_d_n11 * p.p32);
        let eq14_e1075: f64 = (eq14_e1073 * s.v[843]);
        let eq14_e1075_d_n0: f64 = ((eq14_e1073_d_n0 * s.v[843]) + (eq14_e1073 * s.dn[843][0]));
        let eq14_e1075_d_n1: f64 = ((eq14_e1073_d_n1 * s.v[843]) + (eq14_e1073 * s.dn[843][1]));
        let eq14_e1075_d_n2: f64 = ((eq14_e1073_d_n2 * s.v[843]) + (eq14_e1073 * s.dn[843][2]));
        let eq14_e1075_d_n3: f64 = ((eq14_e1073_d_n3 * s.v[843]) + (eq14_e1073 * s.dn[843][3]));
        let eq14_e1075_d_n4: f64 = ((eq14_e1073_d_n4 * s.v[843]) + (eq14_e1073 * s.dn[843][4]));
        let eq14_e1075_d_n5: f64 = ((eq14_e1073_d_n5 * s.v[843]) + (eq14_e1073 * s.dn[843][5]));
        let eq14_e1075_d_n6: f64 = ((eq14_e1073_d_n6 * s.v[843]) + (eq14_e1073 * s.dn[843][6]));
        let eq14_e1075_d_n7: f64 = ((eq14_e1073_d_n7 * s.v[843]) + (eq14_e1073 * s.dn[843][7]));
        let eq14_e1075_d_n8: f64 = ((eq14_e1073_d_n8 * s.v[843]) + (eq14_e1073 * s.dn[843][8]));
        let eq14_e1075_d_n9: f64 = ((eq14_e1073_d_n9 * s.v[843]) + (eq14_e1073 * s.dn[843][9]));
        let eq14_e1075_d_n10: f64 = ((eq14_e1073_d_n10 * s.v[843]) + (eq14_e1073 * s.dn[843][10]));
        let eq14_e1075_d_n11: f64 = ((eq14_e1073_d_n11 * s.v[843]) + (eq14_e1073 * s.dn[843][11]));
        let eq14_value: f64 = eq14_e1075;
        let eq14_node_derivatives: [f64; 12] = [eq14_e1075_d_n0, eq14_e1075_d_n1, eq14_e1075_d_n2, eq14_e1075_d_n3, eq14_e1075_d_n4, eq14_e1075_d_n5, eq14_e1075_d_n6, eq14_e1075_d_n7, eq14_e1075_d_n8, eq14_e1075_d_n9, eq14_e1075_d_n10, eq14_e1075_d_n11];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }
}
