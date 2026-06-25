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
        s.v[2702] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        s.v[2703] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2702] != 0.0))) && (s.v[2703] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2702] != 0.0))) && (!(s.v[2703] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2704] = if (s.v[534] == 0.0) { 1.0 } else { 0.0 };

        s.v[2705] = if (s.v[514] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2705] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)), s.ad_value(514)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) {
            s.store_mul_ad_rhs(2565, 581, A::div(A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(596)), s.ad_value(2540)));
        }

        s.v[2706] = if (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2706] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(611)), s.ad_value(2565)));
        }

        s.v[2707] = if (((-s.v[611]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2706] != 0.0))) && (s.v[2707] != 0.0)) {
            let assign61570_ad_e79960: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign61570_ad_e79960);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2706] != 0.0))) && (!(s.v[2707] != 0.0))) {
            let assign61580_ad_e80011: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign61580_ad_e80011);
        }

        s.v[2708] = if (s.v[642] > 1000.0) { 1.0 } else { 0.0 };

        s.v[2709] = if (s.v[2539] > ((-s.v[445]) * s.v[642])) { 1.0 } else { 0.0 };

        s.v[2710] = if (s.v[546] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) && (s.v[2710] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(617)), A::mul(s.ad_value(2539), s.ad_value(617))), A::mul(s.ad_value(2539), s.ad_value(617))), A::mul(s.ad_value(2539), s.ad_value(617)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) && (!(s.v[2710] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(546)));
        }

        s.v[2711] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            let assign61700_ad_e80236: A = {
                if (s.v[822] < s.v[551]) {
                    {
                        if (((s.v[822] - s.v[551]) / s.v[552]) < (-37.0)) {
                            s.ad_value(551)
                        } else {
                            A::add(s.ad_value(551), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(822), s.ad_value(551)), s.ad_value(552))), 1.0)), s.ad_value(552)))
                        }
                    }
                } else {
                    {
                        if (((s.v[822] - s.v[551]) / s.v[552]) > 37.0) {
                            s.ad_value(822)
                        } else {
                            A::add(s.ad_value(822), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(551), s.ad_value(822)), s.ad_value(552))), 1.0)), s.ad_value(552)))
                        }
                    }
                }
            };
            s.store_ad(2567, &assign61700_ad_e80236);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2712] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (s.v[2712] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (!(s.v[2712] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))), s.ad_value(578)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(1907, A::add(A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(590), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub_ad_lhs(2567, A::add(s.ad_value(822), s.ad_value(551)), 2567);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2713] = if (s.v[631] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (s.v[2713] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (!(s.v[2713] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))), s.ad_value(631)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(473, A::add(A::mul(s.ad_value(634), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(635), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(1907, 1907, 473);
        }

        s.v[2714] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) && (s.v[2714] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) && (!(s.v[2714] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))), s.ad_value(578)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) {
            s.store_scale_ad(1907, A::add(A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(590), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        s.store_neg_ad(839, A::add(A::add(s.ad_value(840), s.ad_value(841)), s.ad_value(842)));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_ad(846, A::add(A::mul(s.ad_value(647), s.ad_value(1902)), A::mul(s.ad_value(648), s.ad_value(1903))), A::mul(s.ad_value(649), s.ad_value(1904)));

        s.store_add_ad(847, A::add(A::mul(s.ad_value(674), s.ad_value(1905)), A::mul(s.ad_value(675), s.ad_value(1906))), A::mul(s.ad_value(676), s.ad_value(1907)));

        s.v[2729] = if (s.v[820] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2729] != 0.0) {
            s.copy_ad(2728, 842);
        }

        if (s.v[2729] != 0.0) {
            s.copy_ad(842, 839);
        }

        if (s.v[2729] != 0.0) {
            s.copy_ad(839, 2728);
        }

        s.store_mul(849, 1888, 1879);

        s.v[2762] = if ((s.v[1813] > 0.0) && (s.v[1917] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2767] = if ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_div_ad(849, A::mul(A::mul(A::square(s.ad_value(1892)), s.ad_value(1888)), s.ad_value(1879)), A::square(s.ad_value(1890)));
        }

        s.v[2771] = if (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2771] != 0.0) {
            s.store_div_ad_lhs(1930, A::scale(s.ad_value(1867), 4.0), 1925);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul(1930, 760, 1916);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul(1930, 1848, 1861);
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
        let (eq0_e972, eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12,) = {
    if (s.v[2715] != 0.0) {
        let eq0_e966: f64 = (s.v[0] * s.v[15]);
        let eq0_e966_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq0_e966_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq0_e966_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq0_e966_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq0_e966_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq0_e966_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq0_e966_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq0_e966_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq0_e966_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq0_e966_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq0_e966_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq0_e966_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq0_e966_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e968_d_n0: f64 = (eq0_e966_d_n0 * p.p32);
        let eq0_e968_d_n1: f64 = (eq0_e966_d_n1 * p.p32);
        let eq0_e968_d_n2: f64 = (eq0_e966_d_n2 * p.p32);
        let eq0_e968_d_n3: f64 = (eq0_e966_d_n3 * p.p32);
        let eq0_e968_d_n4: f64 = (eq0_e966_d_n4 * p.p32);
        let eq0_e968_d_n5: f64 = (eq0_e966_d_n5 * p.p32);
        let eq0_e968_d_n6: f64 = (eq0_e966_d_n6 * p.p32);
        let eq0_e968_d_n7: f64 = (eq0_e966_d_n7 * p.p32);
        let eq0_e968_d_n8: f64 = (eq0_e966_d_n8 * p.p32);
        let eq0_e968_d_n9: f64 = (eq0_e966_d_n9 * p.p32);
        let eq0_e968_d_n10: f64 = (eq0_e966_d_n10 * p.p32);
        let eq0_e968_d_n11: f64 = (eq0_e966_d_n11 * p.p32);
        let eq0_e968_d_n12: f64 = (eq0_e966_d_n12 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * s.v[836]);
        let eq0_e970_d_n0: f64 = ((eq0_e968_d_n0 * s.v[836]) + (eq0_e968 * s.dn[836][0]));
        let eq0_e970_d_n1: f64 = ((eq0_e968_d_n1 * s.v[836]) + (eq0_e968 * s.dn[836][1]));
        let eq0_e970_d_n2: f64 = ((eq0_e968_d_n2 * s.v[836]) + (eq0_e968 * s.dn[836][2]));
        let eq0_e970_d_n3: f64 = ((eq0_e968_d_n3 * s.v[836]) + (eq0_e968 * s.dn[836][3]));
        let eq0_e970_d_n4: f64 = ((eq0_e968_d_n4 * s.v[836]) + (eq0_e968 * s.dn[836][4]));
        let eq0_e970_d_n5: f64 = ((eq0_e968_d_n5 * s.v[836]) + (eq0_e968 * s.dn[836][5]));
        let eq0_e970_d_n6: f64 = ((eq0_e968_d_n6 * s.v[836]) + (eq0_e968 * s.dn[836][6]));
        let eq0_e970_d_n7: f64 = ((eq0_e968_d_n7 * s.v[836]) + (eq0_e968 * s.dn[836][7]));
        let eq0_e970_d_n8: f64 = ((eq0_e968_d_n8 * s.v[836]) + (eq0_e968 * s.dn[836][8]));
        let eq0_e970_d_n9: f64 = ((eq0_e968_d_n9 * s.v[836]) + (eq0_e968 * s.dn[836][9]));
        let eq0_e970_d_n10: f64 = ((eq0_e968_d_n10 * s.v[836]) + (eq0_e968 * s.dn[836][10]));
        let eq0_e970_d_n11: f64 = ((eq0_e968_d_n11 * s.v[836]) + (eq0_e968 * s.dn[836][11]));
        let eq0_e970_d_n12: f64 = ((eq0_e968_d_n12 * s.v[836]) + (eq0_e968 * s.dn[836][12]));
        (eq0_e970, eq0_e970_d_n0, eq0_e970_d_n1, eq0_e970_d_n2, eq0_e970_d_n3, eq0_e970_d_n4, eq0_e970_d_n5, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9, eq0_e970_d_n10, eq0_e970_d_n11, eq0_e970_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        let eq0_node_derivatives: [f64; 13] = [eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
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
        let (eq1_e984, eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12,) = {
    if (s.v[2715] != 0.0) {
        let eq1_e976: f64 = (s.v[0] * s.v[15]);
        let eq1_e976_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq1_e976_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq1_e976_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq1_e976_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq1_e976_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq1_e976_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq1_e976_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq1_e976_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq1_e976_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq1_e976_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq1_e976_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq1_e976_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq1_e976_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e978_d_n0: f64 = (eq1_e976_d_n0 * p.p32);
        let eq1_e978_d_n1: f64 = (eq1_e976_d_n1 * p.p32);
        let eq1_e978_d_n2: f64 = (eq1_e976_d_n2 * p.p32);
        let eq1_e978_d_n3: f64 = (eq1_e976_d_n3 * p.p32);
        let eq1_e978_d_n4: f64 = (eq1_e976_d_n4 * p.p32);
        let eq1_e978_d_n5: f64 = (eq1_e976_d_n5 * p.p32);
        let eq1_e978_d_n6: f64 = (eq1_e976_d_n6 * p.p32);
        let eq1_e978_d_n7: f64 = (eq1_e976_d_n7 * p.p32);
        let eq1_e978_d_n8: f64 = (eq1_e976_d_n8 * p.p32);
        let eq1_e978_d_n9: f64 = (eq1_e976_d_n9 * p.p32);
        let eq1_e978_d_n10: f64 = (eq1_e976_d_n10 * p.p32);
        let eq1_e978_d_n11: f64 = (eq1_e976_d_n11 * p.p32);
        let eq1_e978_d_n12: f64 = (eq1_e976_d_n12 * p.p32);
        let eq1_e981: f64 = (s.v[827] + s.v[835]);
        let eq1_e981_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);
        let eq1_e981_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);
        let eq1_e981_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);
        let eq1_e981_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);
        let eq1_e981_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);
        let eq1_e981_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);
        let eq1_e981_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);
        let eq1_e981_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);
        let eq1_e981_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);
        let eq1_e981_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);
        let eq1_e981_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);
        let eq1_e981_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);
        let eq1_e981_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n0: f64 = ((eq1_e978_d_n0 * eq1_e981) + (eq1_e978 * eq1_e981_d_n0));
        let eq1_e982_d_n1: f64 = ((eq1_e978_d_n1 * eq1_e981) + (eq1_e978 * eq1_e981_d_n1));
        let eq1_e982_d_n2: f64 = ((eq1_e978_d_n2 * eq1_e981) + (eq1_e978 * eq1_e981_d_n2));
        let eq1_e982_d_n3: f64 = ((eq1_e978_d_n3 * eq1_e981) + (eq1_e978 * eq1_e981_d_n3));
        let eq1_e982_d_n4: f64 = ((eq1_e978_d_n4 * eq1_e981) + (eq1_e978 * eq1_e981_d_n4));
        let eq1_e982_d_n5: f64 = ((eq1_e978_d_n5 * eq1_e981) + (eq1_e978 * eq1_e981_d_n5));
        let eq1_e982_d_n6: f64 = ((eq1_e978_d_n6 * eq1_e981) + (eq1_e978 * eq1_e981_d_n6));
        let eq1_e982_d_n7: f64 = ((eq1_e978_d_n7 * eq1_e981) + (eq1_e978 * eq1_e981_d_n7));
        let eq1_e982_d_n8: f64 = ((eq1_e978_d_n8 * eq1_e981) + (eq1_e978 * eq1_e981_d_n8));
        let eq1_e982_d_n9: f64 = ((eq1_e978_d_n9 * eq1_e981) + (eq1_e978 * eq1_e981_d_n9));
        let eq1_e982_d_n10: f64 = ((eq1_e978_d_n10 * eq1_e981) + (eq1_e978 * eq1_e981_d_n10));
        let eq1_e982_d_n11: f64 = ((eq1_e978_d_n11 * eq1_e981) + (eq1_e978 * eq1_e981_d_n11));
        let eq1_e982_d_n12: f64 = ((eq1_e978_d_n12 * eq1_e981) + (eq1_e978 * eq1_e981_d_n12));
        (eq1_e982, eq1_e982_d_n0, eq1_e982_d_n1, eq1_e982_d_n2, eq1_e982_d_n3, eq1_e982_d_n4, eq1_e982_d_n5, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9, eq1_e982_d_n10, eq1_e982_d_n11, eq1_e982_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        let eq1_node_derivatives: [f64; 13] = [eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
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
        let (eq2_e994, eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12,) = {
    if (s.v[2715] != 0.0) {
        let eq2_e988: f64 = (s.v[0] * s.v[15]);
        let eq2_e988_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq2_e988_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq2_e988_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq2_e988_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq2_e988_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq2_e988_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq2_e988_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq2_e988_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq2_e988_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq2_e988_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq2_e988_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq2_e988_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq2_e988_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e990_d_n0: f64 = (eq2_e988_d_n0 * p.p32);
        let eq2_e990_d_n1: f64 = (eq2_e988_d_n1 * p.p32);
        let eq2_e990_d_n2: f64 = (eq2_e988_d_n2 * p.p32);
        let eq2_e990_d_n3: f64 = (eq2_e988_d_n3 * p.p32);
        let eq2_e990_d_n4: f64 = (eq2_e988_d_n4 * p.p32);
        let eq2_e990_d_n5: f64 = (eq2_e988_d_n5 * p.p32);
        let eq2_e990_d_n6: f64 = (eq2_e988_d_n6 * p.p32);
        let eq2_e990_d_n7: f64 = (eq2_e988_d_n7 * p.p32);
        let eq2_e990_d_n8: f64 = (eq2_e988_d_n8 * p.p32);
        let eq2_e990_d_n9: f64 = (eq2_e988_d_n9 * p.p32);
        let eq2_e990_d_n10: f64 = (eq2_e988_d_n10 * p.p32);
        let eq2_e990_d_n11: f64 = (eq2_e988_d_n11 * p.p32);
        let eq2_e990_d_n12: f64 = (eq2_e988_d_n12 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * s.v[830]);
        let eq2_e992_d_n0: f64 = ((eq2_e990_d_n0 * s.v[830]) + (eq2_e990 * s.dn[830][0]));
        let eq2_e992_d_n1: f64 = ((eq2_e990_d_n1 * s.v[830]) + (eq2_e990 * s.dn[830][1]));
        let eq2_e992_d_n2: f64 = ((eq2_e990_d_n2 * s.v[830]) + (eq2_e990 * s.dn[830][2]));
        let eq2_e992_d_n3: f64 = ((eq2_e990_d_n3 * s.v[830]) + (eq2_e990 * s.dn[830][3]));
        let eq2_e992_d_n4: f64 = ((eq2_e990_d_n4 * s.v[830]) + (eq2_e990 * s.dn[830][4]));
        let eq2_e992_d_n5: f64 = ((eq2_e990_d_n5 * s.v[830]) + (eq2_e990 * s.dn[830][5]));
        let eq2_e992_d_n6: f64 = ((eq2_e990_d_n6 * s.v[830]) + (eq2_e990 * s.dn[830][6]));
        let eq2_e992_d_n7: f64 = ((eq2_e990_d_n7 * s.v[830]) + (eq2_e990 * s.dn[830][7]));
        let eq2_e992_d_n8: f64 = ((eq2_e990_d_n8 * s.v[830]) + (eq2_e990 * s.dn[830][8]));
        let eq2_e992_d_n9: f64 = ((eq2_e990_d_n9 * s.v[830]) + (eq2_e990 * s.dn[830][9]));
        let eq2_e992_d_n10: f64 = ((eq2_e990_d_n10 * s.v[830]) + (eq2_e990 * s.dn[830][10]));
        let eq2_e992_d_n11: f64 = ((eq2_e990_d_n11 * s.v[830]) + (eq2_e990 * s.dn[830][11]));
        let eq2_e992_d_n12: f64 = ((eq2_e990_d_n12 * s.v[830]) + (eq2_e990 * s.dn[830][12]));
        (eq2_e992, eq2_e992_d_n0, eq2_e992_d_n1, eq2_e992_d_n2, eq2_e992_d_n3, eq2_e992_d_n4, eq2_e992_d_n5, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9, eq2_e992_d_n10, eq2_e992_d_n11, eq2_e992_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        let eq2_node_derivatives: [f64; 13] = [eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let (eq3_e1004, eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12,) = {
    if (s.v[2715] != 0.0) {
        let eq3_e998: f64 = (s.v[0] * s.v[15]);
        let eq3_e998_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq3_e998_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq3_e998_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq3_e998_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq3_e998_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq3_e998_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq3_e998_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq3_e998_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq3_e998_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq3_e998_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq3_e998_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq3_e998_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq3_e998_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1000_d_n0: f64 = (eq3_e998_d_n0 * p.p32);
        let eq3_e1000_d_n1: f64 = (eq3_e998_d_n1 * p.p32);
        let eq3_e1000_d_n2: f64 = (eq3_e998_d_n2 * p.p32);
        let eq3_e1000_d_n3: f64 = (eq3_e998_d_n3 * p.p32);
        let eq3_e1000_d_n4: f64 = (eq3_e998_d_n4 * p.p32);
        let eq3_e1000_d_n5: f64 = (eq3_e998_d_n5 * p.p32);
        let eq3_e1000_d_n6: f64 = (eq3_e998_d_n6 * p.p32);
        let eq3_e1000_d_n7: f64 = (eq3_e998_d_n7 * p.p32);
        let eq3_e1000_d_n8: f64 = (eq3_e998_d_n8 * p.p32);
        let eq3_e1000_d_n9: f64 = (eq3_e998_d_n9 * p.p32);
        let eq3_e1000_d_n10: f64 = (eq3_e998_d_n10 * p.p32);
        let eq3_e1000_d_n11: f64 = (eq3_e998_d_n11 * p.p32);
        let eq3_e1000_d_n12: f64 = (eq3_e998_d_n12 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * s.v[831]);
        let eq3_e1002_d_n0: f64 = ((eq3_e1000_d_n0 * s.v[831]) + (eq3_e1000 * s.dn[831][0]));
        let eq3_e1002_d_n1: f64 = ((eq3_e1000_d_n1 * s.v[831]) + (eq3_e1000 * s.dn[831][1]));
        let eq3_e1002_d_n2: f64 = ((eq3_e1000_d_n2 * s.v[831]) + (eq3_e1000 * s.dn[831][2]));
        let eq3_e1002_d_n3: f64 = ((eq3_e1000_d_n3 * s.v[831]) + (eq3_e1000 * s.dn[831][3]));
        let eq3_e1002_d_n4: f64 = ((eq3_e1000_d_n4 * s.v[831]) + (eq3_e1000 * s.dn[831][4]));
        let eq3_e1002_d_n5: f64 = ((eq3_e1000_d_n5 * s.v[831]) + (eq3_e1000 * s.dn[831][5]));
        let eq3_e1002_d_n6: f64 = ((eq3_e1000_d_n6 * s.v[831]) + (eq3_e1000 * s.dn[831][6]));
        let eq3_e1002_d_n7: f64 = ((eq3_e1000_d_n7 * s.v[831]) + (eq3_e1000 * s.dn[831][7]));
        let eq3_e1002_d_n8: f64 = ((eq3_e1000_d_n8 * s.v[831]) + (eq3_e1000 * s.dn[831][8]));
        let eq3_e1002_d_n9: f64 = ((eq3_e1000_d_n9 * s.v[831]) + (eq3_e1000 * s.dn[831][9]));
        let eq3_e1002_d_n10: f64 = ((eq3_e1000_d_n10 * s.v[831]) + (eq3_e1000 * s.dn[831][10]));
        let eq3_e1002_d_n11: f64 = ((eq3_e1000_d_n11 * s.v[831]) + (eq3_e1000 * s.dn[831][11]));
        let eq3_e1002_d_n12: f64 = ((eq3_e1000_d_n12 * s.v[831]) + (eq3_e1000 * s.dn[831][12]));
        (eq3_e1002, eq3_e1002_d_n0, eq3_e1002_d_n1, eq3_e1002_d_n2, eq3_e1002_d_n3, eq3_e1002_d_n4, eq3_e1002_d_n5, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9, eq3_e1002_d_n10, eq3_e1002_d_n11, eq3_e1002_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        let eq3_node_derivatives: [f64; 13] = [eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let (eq4_e1015, eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12,) = {
    if (!(s.v[2715] != 0.0)) {
        let eq4_e1009: f64 = (s.v[0] * s.v[15]);
        let eq4_e1009_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq4_e1009_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq4_e1009_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq4_e1009_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq4_e1009_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq4_e1009_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq4_e1009_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq4_e1009_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq4_e1009_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq4_e1009_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq4_e1009_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq4_e1009_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq4_e1009_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1011_d_n0: f64 = (eq4_e1009_d_n0 * p.p32);
        let eq4_e1011_d_n1: f64 = (eq4_e1009_d_n1 * p.p32);
        let eq4_e1011_d_n2: f64 = (eq4_e1009_d_n2 * p.p32);
        let eq4_e1011_d_n3: f64 = (eq4_e1009_d_n3 * p.p32);
        let eq4_e1011_d_n4: f64 = (eq4_e1009_d_n4 * p.p32);
        let eq4_e1011_d_n5: f64 = (eq4_e1009_d_n5 * p.p32);
        let eq4_e1011_d_n6: f64 = (eq4_e1009_d_n6 * p.p32);
        let eq4_e1011_d_n7: f64 = (eq4_e1009_d_n7 * p.p32);
        let eq4_e1011_d_n8: f64 = (eq4_e1009_d_n8 * p.p32);
        let eq4_e1011_d_n9: f64 = (eq4_e1009_d_n9 * p.p32);
        let eq4_e1011_d_n10: f64 = (eq4_e1009_d_n10 * p.p32);
        let eq4_e1011_d_n11: f64 = (eq4_e1009_d_n11 * p.p32);
        let eq4_e1011_d_n12: f64 = (eq4_e1009_d_n12 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * s.v[836]);
        let eq4_e1013_d_n0: f64 = ((eq4_e1011_d_n0 * s.v[836]) + (eq4_e1011 * s.dn[836][0]));
        let eq4_e1013_d_n1: f64 = ((eq4_e1011_d_n1 * s.v[836]) + (eq4_e1011 * s.dn[836][1]));
        let eq4_e1013_d_n2: f64 = ((eq4_e1011_d_n2 * s.v[836]) + (eq4_e1011 * s.dn[836][2]));
        let eq4_e1013_d_n3: f64 = ((eq4_e1011_d_n3 * s.v[836]) + (eq4_e1011 * s.dn[836][3]));
        let eq4_e1013_d_n4: f64 = ((eq4_e1011_d_n4 * s.v[836]) + (eq4_e1011 * s.dn[836][4]));
        let eq4_e1013_d_n5: f64 = ((eq4_e1011_d_n5 * s.v[836]) + (eq4_e1011 * s.dn[836][5]));
        let eq4_e1013_d_n6: f64 = ((eq4_e1011_d_n6 * s.v[836]) + (eq4_e1011 * s.dn[836][6]));
        let eq4_e1013_d_n7: f64 = ((eq4_e1011_d_n7 * s.v[836]) + (eq4_e1011 * s.dn[836][7]));
        let eq4_e1013_d_n8: f64 = ((eq4_e1011_d_n8 * s.v[836]) + (eq4_e1011 * s.dn[836][8]));
        let eq4_e1013_d_n9: f64 = ((eq4_e1011_d_n9 * s.v[836]) + (eq4_e1011 * s.dn[836][9]));
        let eq4_e1013_d_n10: f64 = ((eq4_e1011_d_n10 * s.v[836]) + (eq4_e1011 * s.dn[836][10]));
        let eq4_e1013_d_n11: f64 = ((eq4_e1011_d_n11 * s.v[836]) + (eq4_e1011 * s.dn[836][11]));
        let eq4_e1013_d_n12: f64 = ((eq4_e1011_d_n12 * s.v[836]) + (eq4_e1011 * s.dn[836][12]));
        (eq4_e1013, eq4_e1013_d_n0, eq4_e1013_d_n1, eq4_e1013_d_n2, eq4_e1013_d_n3, eq4_e1013_d_n4, eq4_e1013_d_n5, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9, eq4_e1013_d_n10, eq4_e1013_d_n11, eq4_e1013_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        let eq4_node_derivatives: [f64; 13] = [eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let (eq5_e1028, eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12,) = {
    if (!(s.v[2715] != 0.0)) {
        let eq5_e1020: f64 = (s.v[0] * s.v[15]);
        let eq5_e1020_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq5_e1020_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq5_e1020_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq5_e1020_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq5_e1020_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq5_e1020_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq5_e1020_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq5_e1020_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq5_e1020_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq5_e1020_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq5_e1020_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq5_e1020_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq5_e1020_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1022_d_n0: f64 = (eq5_e1020_d_n0 * p.p32);
        let eq5_e1022_d_n1: f64 = (eq5_e1020_d_n1 * p.p32);
        let eq5_e1022_d_n2: f64 = (eq5_e1020_d_n2 * p.p32);
        let eq5_e1022_d_n3: f64 = (eq5_e1020_d_n3 * p.p32);
        let eq5_e1022_d_n4: f64 = (eq5_e1020_d_n4 * p.p32);
        let eq5_e1022_d_n5: f64 = (eq5_e1020_d_n5 * p.p32);
        let eq5_e1022_d_n6: f64 = (eq5_e1020_d_n6 * p.p32);
        let eq5_e1022_d_n7: f64 = (eq5_e1020_d_n7 * p.p32);
        let eq5_e1022_d_n8: f64 = (eq5_e1020_d_n8 * p.p32);
        let eq5_e1022_d_n9: f64 = (eq5_e1020_d_n9 * p.p32);
        let eq5_e1022_d_n10: f64 = (eq5_e1020_d_n10 * p.p32);
        let eq5_e1022_d_n11: f64 = (eq5_e1020_d_n11 * p.p32);
        let eq5_e1022_d_n12: f64 = (eq5_e1020_d_n12 * p.p32);
        let eq5_e1025: f64 = (s.v[827] + s.v[835]);
        let eq5_e1025_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);
        let eq5_e1025_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);
        let eq5_e1025_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);
        let eq5_e1025_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);
        let eq5_e1025_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);
        let eq5_e1025_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);
        let eq5_e1025_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);
        let eq5_e1025_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);
        let eq5_e1025_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);
        let eq5_e1025_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);
        let eq5_e1025_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);
        let eq5_e1025_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);
        let eq5_e1025_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n0: f64 = ((eq5_e1022_d_n0 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n0));
        let eq5_e1026_d_n1: f64 = ((eq5_e1022_d_n1 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n1));
        let eq5_e1026_d_n2: f64 = ((eq5_e1022_d_n2 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n2));
        let eq5_e1026_d_n3: f64 = ((eq5_e1022_d_n3 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n3));
        let eq5_e1026_d_n4: f64 = ((eq5_e1022_d_n4 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n4));
        let eq5_e1026_d_n5: f64 = ((eq5_e1022_d_n5 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n5));
        let eq5_e1026_d_n6: f64 = ((eq5_e1022_d_n6 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n6));
        let eq5_e1026_d_n7: f64 = ((eq5_e1022_d_n7 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n7));
        let eq5_e1026_d_n8: f64 = ((eq5_e1022_d_n8 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n8));
        let eq5_e1026_d_n9: f64 = ((eq5_e1022_d_n9 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n9));
        let eq5_e1026_d_n10: f64 = ((eq5_e1022_d_n10 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n10));
        let eq5_e1026_d_n11: f64 = ((eq5_e1022_d_n11 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n11));
        let eq5_e1026_d_n12: f64 = ((eq5_e1022_d_n12 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n12));
        (eq5_e1026, eq5_e1026_d_n0, eq5_e1026_d_n1, eq5_e1026_d_n2, eq5_e1026_d_n3, eq5_e1026_d_n4, eq5_e1026_d_n5, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9, eq5_e1026_d_n10, eq5_e1026_d_n11, eq5_e1026_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        let eq5_node_derivatives: [f64; 13] = [eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let (eq6_e1039, eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12,) = {
    if (!(s.v[2715] != 0.0)) {
        let eq6_e1033: f64 = (s.v[0] * s.v[15]);
        let eq6_e1033_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq6_e1033_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq6_e1033_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq6_e1033_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq6_e1033_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq6_e1033_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq6_e1033_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq6_e1033_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq6_e1033_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq6_e1033_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq6_e1033_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq6_e1033_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq6_e1033_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1035_d_n0: f64 = (eq6_e1033_d_n0 * p.p32);
        let eq6_e1035_d_n1: f64 = (eq6_e1033_d_n1 * p.p32);
        let eq6_e1035_d_n2: f64 = (eq6_e1033_d_n2 * p.p32);
        let eq6_e1035_d_n3: f64 = (eq6_e1033_d_n3 * p.p32);
        let eq6_e1035_d_n4: f64 = (eq6_e1033_d_n4 * p.p32);
        let eq6_e1035_d_n5: f64 = (eq6_e1033_d_n5 * p.p32);
        let eq6_e1035_d_n6: f64 = (eq6_e1033_d_n6 * p.p32);
        let eq6_e1035_d_n7: f64 = (eq6_e1033_d_n7 * p.p32);
        let eq6_e1035_d_n8: f64 = (eq6_e1033_d_n8 * p.p32);
        let eq6_e1035_d_n9: f64 = (eq6_e1033_d_n9 * p.p32);
        let eq6_e1035_d_n10: f64 = (eq6_e1033_d_n10 * p.p32);
        let eq6_e1035_d_n11: f64 = (eq6_e1033_d_n11 * p.p32);
        let eq6_e1035_d_n12: f64 = (eq6_e1033_d_n12 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * s.v[830]);
        let eq6_e1037_d_n0: f64 = ((eq6_e1035_d_n0 * s.v[830]) + (eq6_e1035 * s.dn[830][0]));
        let eq6_e1037_d_n1: f64 = ((eq6_e1035_d_n1 * s.v[830]) + (eq6_e1035 * s.dn[830][1]));
        let eq6_e1037_d_n2: f64 = ((eq6_e1035_d_n2 * s.v[830]) + (eq6_e1035 * s.dn[830][2]));
        let eq6_e1037_d_n3: f64 = ((eq6_e1035_d_n3 * s.v[830]) + (eq6_e1035 * s.dn[830][3]));
        let eq6_e1037_d_n4: f64 = ((eq6_e1035_d_n4 * s.v[830]) + (eq6_e1035 * s.dn[830][4]));
        let eq6_e1037_d_n5: f64 = ((eq6_e1035_d_n5 * s.v[830]) + (eq6_e1035 * s.dn[830][5]));
        let eq6_e1037_d_n6: f64 = ((eq6_e1035_d_n6 * s.v[830]) + (eq6_e1035 * s.dn[830][6]));
        let eq6_e1037_d_n7: f64 = ((eq6_e1035_d_n7 * s.v[830]) + (eq6_e1035 * s.dn[830][7]));
        let eq6_e1037_d_n8: f64 = ((eq6_e1035_d_n8 * s.v[830]) + (eq6_e1035 * s.dn[830][8]));
        let eq6_e1037_d_n9: f64 = ((eq6_e1035_d_n9 * s.v[830]) + (eq6_e1035 * s.dn[830][9]));
        let eq6_e1037_d_n10: f64 = ((eq6_e1035_d_n10 * s.v[830]) + (eq6_e1035 * s.dn[830][10]));
        let eq6_e1037_d_n11: f64 = ((eq6_e1035_d_n11 * s.v[830]) + (eq6_e1035 * s.dn[830][11]));
        let eq6_e1037_d_n12: f64 = ((eq6_e1035_d_n12 * s.v[830]) + (eq6_e1035 * s.dn[830][12]));
        (eq6_e1037, eq6_e1037_d_n0, eq6_e1037_d_n1, eq6_e1037_d_n2, eq6_e1037_d_n3, eq6_e1037_d_n4, eq6_e1037_d_n5, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9, eq6_e1037_d_n10, eq6_e1037_d_n11, eq6_e1037_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        let eq6_node_derivatives: [f64; 13] = [eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let (eq7_e1050, eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12,) = {
    if (!(s.v[2715] != 0.0)) {
        let eq7_e1044: f64 = (s.v[0] * s.v[15]);
        let eq7_e1044_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq7_e1044_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq7_e1044_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq7_e1044_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq7_e1044_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq7_e1044_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq7_e1044_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq7_e1044_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq7_e1044_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq7_e1044_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq7_e1044_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq7_e1044_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq7_e1044_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1046_d_n0: f64 = (eq7_e1044_d_n0 * p.p32);
        let eq7_e1046_d_n1: f64 = (eq7_e1044_d_n1 * p.p32);
        let eq7_e1046_d_n2: f64 = (eq7_e1044_d_n2 * p.p32);
        let eq7_e1046_d_n3: f64 = (eq7_e1044_d_n3 * p.p32);
        let eq7_e1046_d_n4: f64 = (eq7_e1044_d_n4 * p.p32);
        let eq7_e1046_d_n5: f64 = (eq7_e1044_d_n5 * p.p32);
        let eq7_e1046_d_n6: f64 = (eq7_e1044_d_n6 * p.p32);
        let eq7_e1046_d_n7: f64 = (eq7_e1044_d_n7 * p.p32);
        let eq7_e1046_d_n8: f64 = (eq7_e1044_d_n8 * p.p32);
        let eq7_e1046_d_n9: f64 = (eq7_e1044_d_n9 * p.p32);
        let eq7_e1046_d_n10: f64 = (eq7_e1044_d_n10 * p.p32);
        let eq7_e1046_d_n11: f64 = (eq7_e1044_d_n11 * p.p32);
        let eq7_e1046_d_n12: f64 = (eq7_e1044_d_n12 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * s.v[831]);
        let eq7_e1048_d_n0: f64 = ((eq7_e1046_d_n0 * s.v[831]) + (eq7_e1046 * s.dn[831][0]));
        let eq7_e1048_d_n1: f64 = ((eq7_e1046_d_n1 * s.v[831]) + (eq7_e1046 * s.dn[831][1]));
        let eq7_e1048_d_n2: f64 = ((eq7_e1046_d_n2 * s.v[831]) + (eq7_e1046 * s.dn[831][2]));
        let eq7_e1048_d_n3: f64 = ((eq7_e1046_d_n3 * s.v[831]) + (eq7_e1046 * s.dn[831][3]));
        let eq7_e1048_d_n4: f64 = ((eq7_e1046_d_n4 * s.v[831]) + (eq7_e1046 * s.dn[831][4]));
        let eq7_e1048_d_n5: f64 = ((eq7_e1046_d_n5 * s.v[831]) + (eq7_e1046 * s.dn[831][5]));
        let eq7_e1048_d_n6: f64 = ((eq7_e1046_d_n6 * s.v[831]) + (eq7_e1046 * s.dn[831][6]));
        let eq7_e1048_d_n7: f64 = ((eq7_e1046_d_n7 * s.v[831]) + (eq7_e1046 * s.dn[831][7]));
        let eq7_e1048_d_n8: f64 = ((eq7_e1046_d_n8 * s.v[831]) + (eq7_e1046 * s.dn[831][8]));
        let eq7_e1048_d_n9: f64 = ((eq7_e1046_d_n9 * s.v[831]) + (eq7_e1046 * s.dn[831][9]));
        let eq7_e1048_d_n10: f64 = ((eq7_e1046_d_n10 * s.v[831]) + (eq7_e1046 * s.dn[831][10]));
        let eq7_e1048_d_n11: f64 = ((eq7_e1046_d_n11 * s.v[831]) + (eq7_e1046 * s.dn[831][11]));
        let eq7_e1048_d_n12: f64 = ((eq7_e1046_d_n12 * s.v[831]) + (eq7_e1046 * s.dn[831][12]));
        (eq7_e1048, eq7_e1048_d_n0, eq7_e1048_d_n1, eq7_e1048_d_n2, eq7_e1048_d_n3, eq7_e1048_d_n4, eq7_e1048_d_n5, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9, eq7_e1048_d_n10, eq7_e1048_d_n11, eq7_e1048_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        let eq7_node_derivatives: [f64; 13] = [eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq8_e1053: f64 = (s.v[0] * s.v[15]);
        let eq8_e1053_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq8_e1053_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq8_e1053_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq8_e1053_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq8_e1053_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq8_e1053_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq8_e1053_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq8_e1053_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq8_e1053_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq8_e1053_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq8_e1053_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq8_e1053_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq8_e1053_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1055_d_n0: f64 = (eq8_e1053_d_n0 * p.p32);
        let eq8_e1055_d_n1: f64 = (eq8_e1053_d_n1 * p.p32);
        let eq8_e1055_d_n2: f64 = (eq8_e1053_d_n2 * p.p32);
        let eq8_e1055_d_n3: f64 = (eq8_e1053_d_n3 * p.p32);
        let eq8_e1055_d_n4: f64 = (eq8_e1053_d_n4 * p.p32);
        let eq8_e1055_d_n5: f64 = (eq8_e1053_d_n5 * p.p32);
        let eq8_e1055_d_n6: f64 = (eq8_e1053_d_n6 * p.p32);
        let eq8_e1055_d_n7: f64 = (eq8_e1053_d_n7 * p.p32);
        let eq8_e1055_d_n8: f64 = (eq8_e1053_d_n8 * p.p32);
        let eq8_e1055_d_n9: f64 = (eq8_e1053_d_n9 * p.p32);
        let eq8_e1055_d_n10: f64 = (eq8_e1053_d_n10 * p.p32);
        let eq8_e1055_d_n11: f64 = (eq8_e1053_d_n11 * p.p32);
        let eq8_e1055_d_n12: f64 = (eq8_e1053_d_n12 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * s.v[832]);
        let eq8_e1057_d_n0: f64 = ((eq8_e1055_d_n0 * s.v[832]) + (eq8_e1055 * s.dn[832][0]));
        let eq8_e1057_d_n1: f64 = ((eq8_e1055_d_n1 * s.v[832]) + (eq8_e1055 * s.dn[832][1]));
        let eq8_e1057_d_n2: f64 = ((eq8_e1055_d_n2 * s.v[832]) + (eq8_e1055 * s.dn[832][2]));
        let eq8_e1057_d_n3: f64 = ((eq8_e1055_d_n3 * s.v[832]) + (eq8_e1055 * s.dn[832][3]));
        let eq8_e1057_d_n4: f64 = ((eq8_e1055_d_n4 * s.v[832]) + (eq8_e1055 * s.dn[832][4]));
        let eq8_e1057_d_n5: f64 = ((eq8_e1055_d_n5 * s.v[832]) + (eq8_e1055 * s.dn[832][5]));
        let eq8_e1057_d_n6: f64 = ((eq8_e1055_d_n6 * s.v[832]) + (eq8_e1055 * s.dn[832][6]));
        let eq8_e1057_d_n7: f64 = ((eq8_e1055_d_n7 * s.v[832]) + (eq8_e1055 * s.dn[832][7]));
        let eq8_e1057_d_n8: f64 = ((eq8_e1055_d_n8 * s.v[832]) + (eq8_e1055 * s.dn[832][8]));
        let eq8_e1057_d_n9: f64 = ((eq8_e1055_d_n9 * s.v[832]) + (eq8_e1055 * s.dn[832][9]));
        let eq8_e1057_d_n10: f64 = ((eq8_e1055_d_n10 * s.v[832]) + (eq8_e1055 * s.dn[832][10]));
        let eq8_e1057_d_n11: f64 = ((eq8_e1055_d_n11 * s.v[832]) + (eq8_e1055 * s.dn[832][11]));
        let eq8_e1057_d_n12: f64 = ((eq8_e1055_d_n12 * s.v[832]) + (eq8_e1055 * s.dn[832][12]));
        let eq8_value: f64 = eq8_e1057;
        let eq8_node_derivatives: [f64; 13] = [eq8_e1057_d_n0, eq8_e1057_d_n1, eq8_e1057_d_n2, eq8_e1057_d_n3, eq8_e1057_d_n4, eq8_e1057_d_n5, eq8_e1057_d_n6, eq8_e1057_d_n7, eq8_e1057_d_n8, eq8_e1057_d_n9, eq8_e1057_d_n10, eq8_e1057_d_n11, eq8_e1057_d_n12];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[9]),
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
        let eq9_e1060: f64 = (s.v[0] * s.v[15]);
        let eq9_e1060_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq9_e1060_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq9_e1060_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq9_e1060_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq9_e1060_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq9_e1060_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq9_e1060_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq9_e1060_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq9_e1060_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq9_e1060_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq9_e1060_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq9_e1060_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq9_e1060_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1062_d_n0: f64 = (eq9_e1060_d_n0 * p.p32);
        let eq9_e1062_d_n1: f64 = (eq9_e1060_d_n1 * p.p32);
        let eq9_e1062_d_n2: f64 = (eq9_e1060_d_n2 * p.p32);
        let eq9_e1062_d_n3: f64 = (eq9_e1060_d_n3 * p.p32);
        let eq9_e1062_d_n4: f64 = (eq9_e1060_d_n4 * p.p32);
        let eq9_e1062_d_n5: f64 = (eq9_e1060_d_n5 * p.p32);
        let eq9_e1062_d_n6: f64 = (eq9_e1060_d_n6 * p.p32);
        let eq9_e1062_d_n7: f64 = (eq9_e1060_d_n7 * p.p32);
        let eq9_e1062_d_n8: f64 = (eq9_e1060_d_n8 * p.p32);
        let eq9_e1062_d_n9: f64 = (eq9_e1060_d_n9 * p.p32);
        let eq9_e1062_d_n10: f64 = (eq9_e1060_d_n10 * p.p32);
        let eq9_e1062_d_n11: f64 = (eq9_e1060_d_n11 * p.p32);
        let eq9_e1062_d_n12: f64 = (eq9_e1060_d_n12 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * s.v[828]);
        let eq9_e1064_d_n0: f64 = ((eq9_e1062_d_n0 * s.v[828]) + (eq9_e1062 * s.dn[828][0]));
        let eq9_e1064_d_n1: f64 = ((eq9_e1062_d_n1 * s.v[828]) + (eq9_e1062 * s.dn[828][1]));
        let eq9_e1064_d_n2: f64 = ((eq9_e1062_d_n2 * s.v[828]) + (eq9_e1062 * s.dn[828][2]));
        let eq9_e1064_d_n3: f64 = ((eq9_e1062_d_n3 * s.v[828]) + (eq9_e1062 * s.dn[828][3]));
        let eq9_e1064_d_n4: f64 = ((eq9_e1062_d_n4 * s.v[828]) + (eq9_e1062 * s.dn[828][4]));
        let eq9_e1064_d_n5: f64 = ((eq9_e1062_d_n5 * s.v[828]) + (eq9_e1062 * s.dn[828][5]));
        let eq9_e1064_d_n6: f64 = ((eq9_e1062_d_n6 * s.v[828]) + (eq9_e1062 * s.dn[828][6]));
        let eq9_e1064_d_n7: f64 = ((eq9_e1062_d_n7 * s.v[828]) + (eq9_e1062 * s.dn[828][7]));
        let eq9_e1064_d_n8: f64 = ((eq9_e1062_d_n8 * s.v[828]) + (eq9_e1062 * s.dn[828][8]));
        let eq9_e1064_d_n9: f64 = ((eq9_e1062_d_n9 * s.v[828]) + (eq9_e1062 * s.dn[828][9]));
        let eq9_e1064_d_n10: f64 = ((eq9_e1062_d_n10 * s.v[828]) + (eq9_e1062 * s.dn[828][10]));
        let eq9_e1064_d_n11: f64 = ((eq9_e1062_d_n11 * s.v[828]) + (eq9_e1062 * s.dn[828][11]));
        let eq9_e1064_d_n12: f64 = ((eq9_e1062_d_n12 * s.v[828]) + (eq9_e1062 * s.dn[828][12]));
        let eq9_value: f64 = eq9_e1064;
        let eq9_node_derivatives: [f64; 13] = [eq9_e1064_d_n0, eq9_e1064_d_n1, eq9_e1064_d_n2, eq9_e1064_d_n3, eq9_e1064_d_n4, eq9_e1064_d_n5, eq9_e1064_d_n6, eq9_e1064_d_n7, eq9_e1064_d_n8, eq9_e1064_d_n9, eq9_e1064_d_n10, eq9_e1064_d_n11, eq9_e1064_d_n12];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let eq10_e1067: f64 = (s.v[0] * s.v[15]);
        let eq10_e1067_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq10_e1067_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq10_e1067_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq10_e1067_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq10_e1067_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq10_e1067_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq10_e1067_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq10_e1067_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq10_e1067_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq10_e1067_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq10_e1067_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq10_e1067_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq10_e1067_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1069_d_n0: f64 = (eq10_e1067_d_n0 * p.p32);
        let eq10_e1069_d_n1: f64 = (eq10_e1067_d_n1 * p.p32);
        let eq10_e1069_d_n2: f64 = (eq10_e1067_d_n2 * p.p32);
        let eq10_e1069_d_n3: f64 = (eq10_e1067_d_n3 * p.p32);
        let eq10_e1069_d_n4: f64 = (eq10_e1067_d_n4 * p.p32);
        let eq10_e1069_d_n5: f64 = (eq10_e1067_d_n5 * p.p32);
        let eq10_e1069_d_n6: f64 = (eq10_e1067_d_n6 * p.p32);
        let eq10_e1069_d_n7: f64 = (eq10_e1067_d_n7 * p.p32);
        let eq10_e1069_d_n8: f64 = (eq10_e1067_d_n8 * p.p32);
        let eq10_e1069_d_n9: f64 = (eq10_e1067_d_n9 * p.p32);
        let eq10_e1069_d_n10: f64 = (eq10_e1067_d_n10 * p.p32);
        let eq10_e1069_d_n11: f64 = (eq10_e1067_d_n11 * p.p32);
        let eq10_e1069_d_n12: f64 = (eq10_e1067_d_n12 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * s.v[829]);
        let eq10_e1071_d_n0: f64 = ((eq10_e1069_d_n0 * s.v[829]) + (eq10_e1069 * s.dn[829][0]));
        let eq10_e1071_d_n1: f64 = ((eq10_e1069_d_n1 * s.v[829]) + (eq10_e1069 * s.dn[829][1]));
        let eq10_e1071_d_n2: f64 = ((eq10_e1069_d_n2 * s.v[829]) + (eq10_e1069 * s.dn[829][2]));
        let eq10_e1071_d_n3: f64 = ((eq10_e1069_d_n3 * s.v[829]) + (eq10_e1069 * s.dn[829][3]));
        let eq10_e1071_d_n4: f64 = ((eq10_e1069_d_n4 * s.v[829]) + (eq10_e1069 * s.dn[829][4]));
        let eq10_e1071_d_n5: f64 = ((eq10_e1069_d_n5 * s.v[829]) + (eq10_e1069 * s.dn[829][5]));
        let eq10_e1071_d_n6: f64 = ((eq10_e1069_d_n6 * s.v[829]) + (eq10_e1069 * s.dn[829][6]));
        let eq10_e1071_d_n7: f64 = ((eq10_e1069_d_n7 * s.v[829]) + (eq10_e1069 * s.dn[829][7]));
        let eq10_e1071_d_n8: f64 = ((eq10_e1069_d_n8 * s.v[829]) + (eq10_e1069 * s.dn[829][8]));
        let eq10_e1071_d_n9: f64 = ((eq10_e1069_d_n9 * s.v[829]) + (eq10_e1069 * s.dn[829][9]));
        let eq10_e1071_d_n10: f64 = ((eq10_e1069_d_n10 * s.v[829]) + (eq10_e1069 * s.dn[829][10]));
        let eq10_e1071_d_n11: f64 = ((eq10_e1069_d_n11 * s.v[829]) + (eq10_e1069 * s.dn[829][11]));
        let eq10_e1071_d_n12: f64 = ((eq10_e1069_d_n12 * s.v[829]) + (eq10_e1069 * s.dn[829][12]));
        let eq10_value: f64 = eq10_e1071;
        let eq10_node_derivatives: [f64; 13] = [eq10_e1071_d_n0, eq10_e1071_d_n1, eq10_e1071_d_n2, eq10_e1071_d_n3, eq10_e1071_d_n4, eq10_e1071_d_n5, eq10_e1071_d_n6, eq10_e1071_d_n7, eq10_e1071_d_n8, eq10_e1071_d_n9, eq10_e1071_d_n10, eq10_e1071_d_n11, eq10_e1071_d_n12];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let eq11_e1074: f64 = (s.v[0] * s.v[15]);
        let eq11_e1074_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq11_e1074_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq11_e1074_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq11_e1074_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq11_e1074_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq11_e1074_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq11_e1074_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq11_e1074_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq11_e1074_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq11_e1074_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq11_e1074_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq11_e1074_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq11_e1074_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1076_d_n0: f64 = (eq11_e1074_d_n0 * p.p32);
        let eq11_e1076_d_n1: f64 = (eq11_e1074_d_n1 * p.p32);
        let eq11_e1076_d_n2: f64 = (eq11_e1074_d_n2 * p.p32);
        let eq11_e1076_d_n3: f64 = (eq11_e1074_d_n3 * p.p32);
        let eq11_e1076_d_n4: f64 = (eq11_e1074_d_n4 * p.p32);
        let eq11_e1076_d_n5: f64 = (eq11_e1074_d_n5 * p.p32);
        let eq11_e1076_d_n6: f64 = (eq11_e1074_d_n6 * p.p32);
        let eq11_e1076_d_n7: f64 = (eq11_e1074_d_n7 * p.p32);
        let eq11_e1076_d_n8: f64 = (eq11_e1074_d_n8 * p.p32);
        let eq11_e1076_d_n9: f64 = (eq11_e1074_d_n9 * p.p32);
        let eq11_e1076_d_n10: f64 = (eq11_e1074_d_n10 * p.p32);
        let eq11_e1076_d_n11: f64 = (eq11_e1074_d_n11 * p.p32);
        let eq11_e1076_d_n12: f64 = (eq11_e1074_d_n12 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * s.v[833]);
        let eq11_e1078_d_n0: f64 = ((eq11_e1076_d_n0 * s.v[833]) + (eq11_e1076 * s.dn[833][0]));
        let eq11_e1078_d_n1: f64 = ((eq11_e1076_d_n1 * s.v[833]) + (eq11_e1076 * s.dn[833][1]));
        let eq11_e1078_d_n2: f64 = ((eq11_e1076_d_n2 * s.v[833]) + (eq11_e1076 * s.dn[833][2]));
        let eq11_e1078_d_n3: f64 = ((eq11_e1076_d_n3 * s.v[833]) + (eq11_e1076 * s.dn[833][3]));
        let eq11_e1078_d_n4: f64 = ((eq11_e1076_d_n4 * s.v[833]) + (eq11_e1076 * s.dn[833][4]));
        let eq11_e1078_d_n5: f64 = ((eq11_e1076_d_n5 * s.v[833]) + (eq11_e1076 * s.dn[833][5]));
        let eq11_e1078_d_n6: f64 = ((eq11_e1076_d_n6 * s.v[833]) + (eq11_e1076 * s.dn[833][6]));
        let eq11_e1078_d_n7: f64 = ((eq11_e1076_d_n7 * s.v[833]) + (eq11_e1076 * s.dn[833][7]));
        let eq11_e1078_d_n8: f64 = ((eq11_e1076_d_n8 * s.v[833]) + (eq11_e1076 * s.dn[833][8]));
        let eq11_e1078_d_n9: f64 = ((eq11_e1076_d_n9 * s.v[833]) + (eq11_e1076 * s.dn[833][9]));
        let eq11_e1078_d_n10: f64 = ((eq11_e1076_d_n10 * s.v[833]) + (eq11_e1076 * s.dn[833][10]));
        let eq11_e1078_d_n11: f64 = ((eq11_e1076_d_n11 * s.v[833]) + (eq11_e1076 * s.dn[833][11]));
        let eq11_e1078_d_n12: f64 = ((eq11_e1076_d_n12 * s.v[833]) + (eq11_e1076 * s.dn[833][12]));
        let eq11_value: f64 = eq11_e1078;
        let eq11_node_derivatives: [f64; 13] = [eq11_e1078_d_n0, eq11_e1078_d_n1, eq11_e1078_d_n2, eq11_e1078_d_n3, eq11_e1078_d_n4, eq11_e1078_d_n5, eq11_e1078_d_n6, eq11_e1078_d_n7, eq11_e1078_d_n8, eq11_e1078_d_n9, eq11_e1078_d_n10, eq11_e1078_d_n11, eq11_e1078_d_n12];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let eq12_e1081: f64 = (s.v[0] * s.v[15]);
        let eq12_e1081_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq12_e1081_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq12_e1081_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq12_e1081_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq12_e1081_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq12_e1081_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq12_e1081_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq12_e1081_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq12_e1081_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq12_e1081_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq12_e1081_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq12_e1081_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq12_e1081_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1083_d_n0: f64 = (eq12_e1081_d_n0 * p.p32);
        let eq12_e1083_d_n1: f64 = (eq12_e1081_d_n1 * p.p32);
        let eq12_e1083_d_n2: f64 = (eq12_e1081_d_n2 * p.p32);
        let eq12_e1083_d_n3: f64 = (eq12_e1081_d_n3 * p.p32);
        let eq12_e1083_d_n4: f64 = (eq12_e1081_d_n4 * p.p32);
        let eq12_e1083_d_n5: f64 = (eq12_e1081_d_n5 * p.p32);
        let eq12_e1083_d_n6: f64 = (eq12_e1081_d_n6 * p.p32);
        let eq12_e1083_d_n7: f64 = (eq12_e1081_d_n7 * p.p32);
        let eq12_e1083_d_n8: f64 = (eq12_e1081_d_n8 * p.p32);
        let eq12_e1083_d_n9: f64 = (eq12_e1081_d_n9 * p.p32);
        let eq12_e1083_d_n10: f64 = (eq12_e1081_d_n10 * p.p32);
        let eq12_e1083_d_n11: f64 = (eq12_e1081_d_n11 * p.p32);
        let eq12_e1083_d_n12: f64 = (eq12_e1081_d_n12 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * s.v[834]);
        let eq12_e1085_d_n0: f64 = ((eq12_e1083_d_n0 * s.v[834]) + (eq12_e1083 * s.dn[834][0]));
        let eq12_e1085_d_n1: f64 = ((eq12_e1083_d_n1 * s.v[834]) + (eq12_e1083 * s.dn[834][1]));
        let eq12_e1085_d_n2: f64 = ((eq12_e1083_d_n2 * s.v[834]) + (eq12_e1083 * s.dn[834][2]));
        let eq12_e1085_d_n3: f64 = ((eq12_e1083_d_n3 * s.v[834]) + (eq12_e1083 * s.dn[834][3]));
        let eq12_e1085_d_n4: f64 = ((eq12_e1083_d_n4 * s.v[834]) + (eq12_e1083 * s.dn[834][4]));
        let eq12_e1085_d_n5: f64 = ((eq12_e1083_d_n5 * s.v[834]) + (eq12_e1083 * s.dn[834][5]));
        let eq12_e1085_d_n6: f64 = ((eq12_e1083_d_n6 * s.v[834]) + (eq12_e1083 * s.dn[834][6]));
        let eq12_e1085_d_n7: f64 = ((eq12_e1083_d_n7 * s.v[834]) + (eq12_e1083 * s.dn[834][7]));
        let eq12_e1085_d_n8: f64 = ((eq12_e1083_d_n8 * s.v[834]) + (eq12_e1083 * s.dn[834][8]));
        let eq12_e1085_d_n9: f64 = ((eq12_e1083_d_n9 * s.v[834]) + (eq12_e1083 * s.dn[834][9]));
        let eq12_e1085_d_n10: f64 = ((eq12_e1083_d_n10 * s.v[834]) + (eq12_e1083 * s.dn[834][10]));
        let eq12_e1085_d_n11: f64 = ((eq12_e1083_d_n11 * s.v[834]) + (eq12_e1083 * s.dn[834][11]));
        let eq12_e1085_d_n12: f64 = ((eq12_e1083_d_n12 * s.v[834]) + (eq12_e1083 * s.dn[834][12]));
        let eq12_value: f64 = eq12_e1085;
        let eq12_node_derivatives: [f64; 13] = [eq12_e1085_d_n0, eq12_e1085_d_n1, eq12_e1085_d_n2, eq12_e1085_d_n3, eq12_e1085_d_n4, eq12_e1085_d_n5, eq12_e1085_d_n6, eq12_e1085_d_n7, eq12_e1085_d_n8, eq12_e1085_d_n9, eq12_e1085_d_n10, eq12_e1085_d_n11, eq12_e1085_d_n12];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
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
        let eq13_e1088: f64 = (s.v[0] * s.v[15]);
        let eq13_e1088_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq13_e1088_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq13_e1088_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq13_e1088_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq13_e1088_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq13_e1088_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq13_e1088_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq13_e1088_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq13_e1088_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq13_e1088_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq13_e1088_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq13_e1088_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq13_e1088_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq13_e1090: f64 = (eq13_e1088 * p.p32);
        let eq13_e1090_d_n0: f64 = (eq13_e1088_d_n0 * p.p32);
        let eq13_e1090_d_n1: f64 = (eq13_e1088_d_n1 * p.p32);
        let eq13_e1090_d_n2: f64 = (eq13_e1088_d_n2 * p.p32);
        let eq13_e1090_d_n3: f64 = (eq13_e1088_d_n3 * p.p32);
        let eq13_e1090_d_n4: f64 = (eq13_e1088_d_n4 * p.p32);
        let eq13_e1090_d_n5: f64 = (eq13_e1088_d_n5 * p.p32);
        let eq13_e1090_d_n6: f64 = (eq13_e1088_d_n6 * p.p32);
        let eq13_e1090_d_n7: f64 = (eq13_e1088_d_n7 * p.p32);
        let eq13_e1090_d_n8: f64 = (eq13_e1088_d_n8 * p.p32);
        let eq13_e1090_d_n9: f64 = (eq13_e1088_d_n9 * p.p32);
        let eq13_e1090_d_n10: f64 = (eq13_e1088_d_n10 * p.p32);
        let eq13_e1090_d_n11: f64 = (eq13_e1088_d_n11 * p.p32);
        let eq13_e1090_d_n12: f64 = (eq13_e1088_d_n12 * p.p32);
        let eq13_e1092: f64 = (eq13_e1090 * s.v[837]);
        let eq13_e1092_d_n0: f64 = ((eq13_e1090_d_n0 * s.v[837]) + (eq13_e1090 * s.dn[837][0]));
        let eq13_e1092_d_n1: f64 = ((eq13_e1090_d_n1 * s.v[837]) + (eq13_e1090 * s.dn[837][1]));
        let eq13_e1092_d_n2: f64 = ((eq13_e1090_d_n2 * s.v[837]) + (eq13_e1090 * s.dn[837][2]));
        let eq13_e1092_d_n3: f64 = ((eq13_e1090_d_n3 * s.v[837]) + (eq13_e1090 * s.dn[837][3]));
        let eq13_e1092_d_n4: f64 = ((eq13_e1090_d_n4 * s.v[837]) + (eq13_e1090 * s.dn[837][4]));
        let eq13_e1092_d_n5: f64 = ((eq13_e1090_d_n5 * s.v[837]) + (eq13_e1090 * s.dn[837][5]));
        let eq13_e1092_d_n6: f64 = ((eq13_e1090_d_n6 * s.v[837]) + (eq13_e1090 * s.dn[837][6]));
        let eq13_e1092_d_n7: f64 = ((eq13_e1090_d_n7 * s.v[837]) + (eq13_e1090 * s.dn[837][7]));
        let eq13_e1092_d_n8: f64 = ((eq13_e1090_d_n8 * s.v[837]) + (eq13_e1090 * s.dn[837][8]));
        let eq13_e1092_d_n9: f64 = ((eq13_e1090_d_n9 * s.v[837]) + (eq13_e1090 * s.dn[837][9]));
        let eq13_e1092_d_n10: f64 = ((eq13_e1090_d_n10 * s.v[837]) + (eq13_e1090 * s.dn[837][10]));
        let eq13_e1092_d_n11: f64 = ((eq13_e1090_d_n11 * s.v[837]) + (eq13_e1090 * s.dn[837][11]));
        let eq13_e1092_d_n12: f64 = ((eq13_e1090_d_n12 * s.v[837]) + (eq13_e1090 * s.dn[837][12]));
        let eq13_value: f64 = eq13_e1092;
        let eq13_node_derivatives: [f64; 13] = [eq13_e1092_d_n0, eq13_e1092_d_n1, eq13_e1092_d_n2, eq13_e1092_d_n3, eq13_e1092_d_n4, eq13_e1092_d_n5, eq13_e1092_d_n6, eq13_e1092_d_n7, eq13_e1092_d_n8, eq13_e1092_d_n9, eq13_e1092_d_n10, eq13_e1092_d_n11, eq13_e1092_d_n12];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
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
        let eq14_e1095: f64 = (s.v[0] * s.v[15]);
        let eq14_e1095_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq14_e1095_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq14_e1095_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq14_e1095_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq14_e1095_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq14_e1095_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq14_e1095_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq14_e1095_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq14_e1095_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq14_e1095_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq14_e1095_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq14_e1095_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq14_e1095_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq14_e1097: f64 = (eq14_e1095 * p.p32);
        let eq14_e1097_d_n0: f64 = (eq14_e1095_d_n0 * p.p32);
        let eq14_e1097_d_n1: f64 = (eq14_e1095_d_n1 * p.p32);
        let eq14_e1097_d_n2: f64 = (eq14_e1095_d_n2 * p.p32);
        let eq14_e1097_d_n3: f64 = (eq14_e1095_d_n3 * p.p32);
        let eq14_e1097_d_n4: f64 = (eq14_e1095_d_n4 * p.p32);
        let eq14_e1097_d_n5: f64 = (eq14_e1095_d_n5 * p.p32);
        let eq14_e1097_d_n6: f64 = (eq14_e1095_d_n6 * p.p32);
        let eq14_e1097_d_n7: f64 = (eq14_e1095_d_n7 * p.p32);
        let eq14_e1097_d_n8: f64 = (eq14_e1095_d_n8 * p.p32);
        let eq14_e1097_d_n9: f64 = (eq14_e1095_d_n9 * p.p32);
        let eq14_e1097_d_n10: f64 = (eq14_e1095_d_n10 * p.p32);
        let eq14_e1097_d_n11: f64 = (eq14_e1095_d_n11 * p.p32);
        let eq14_e1097_d_n12: f64 = (eq14_e1095_d_n12 * p.p32);
        let eq14_e1099: f64 = (eq14_e1097 * s.v[838]);
        let eq14_e1099_d_n0: f64 = ((eq14_e1097_d_n0 * s.v[838]) + (eq14_e1097 * s.dn[838][0]));
        let eq14_e1099_d_n1: f64 = ((eq14_e1097_d_n1 * s.v[838]) + (eq14_e1097 * s.dn[838][1]));
        let eq14_e1099_d_n2: f64 = ((eq14_e1097_d_n2 * s.v[838]) + (eq14_e1097 * s.dn[838][2]));
        let eq14_e1099_d_n3: f64 = ((eq14_e1097_d_n3 * s.v[838]) + (eq14_e1097 * s.dn[838][3]));
        let eq14_e1099_d_n4: f64 = ((eq14_e1097_d_n4 * s.v[838]) + (eq14_e1097 * s.dn[838][4]));
        let eq14_e1099_d_n5: f64 = ((eq14_e1097_d_n5 * s.v[838]) + (eq14_e1097 * s.dn[838][5]));
        let eq14_e1099_d_n6: f64 = ((eq14_e1097_d_n6 * s.v[838]) + (eq14_e1097 * s.dn[838][6]));
        let eq14_e1099_d_n7: f64 = ((eq14_e1097_d_n7 * s.v[838]) + (eq14_e1097 * s.dn[838][7]));
        let eq14_e1099_d_n8: f64 = ((eq14_e1097_d_n8 * s.v[838]) + (eq14_e1097 * s.dn[838][8]));
        let eq14_e1099_d_n9: f64 = ((eq14_e1097_d_n9 * s.v[838]) + (eq14_e1097 * s.dn[838][9]));
        let eq14_e1099_d_n10: f64 = ((eq14_e1097_d_n10 * s.v[838]) + (eq14_e1097 * s.dn[838][10]));
        let eq14_e1099_d_n11: f64 = ((eq14_e1097_d_n11 * s.v[838]) + (eq14_e1097 * s.dn[838][11]));
        let eq14_e1099_d_n12: f64 = ((eq14_e1097_d_n12 * s.v[838]) + (eq14_e1097 * s.dn[838][12]));
        let eq14_value: f64 = eq14_e1099;
        let eq14_node_derivatives: [f64; 13] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }
}
