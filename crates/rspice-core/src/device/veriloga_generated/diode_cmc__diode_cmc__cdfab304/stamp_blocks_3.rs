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
        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(185, A::sub(s.ad_value(180), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(190), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(190), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(186, A::sub(s.ad_value(181), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(191), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(191), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(187, A::sub(s.ad_value(182), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(192), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(192), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        s.v[851] = if (((s.v[180] < 0.0) && (s.v[181] < 0.0)) && (s.v[182] < 0.0)) { 1.0 } else { 0.0 };

        s.v[852] = if (((((((s.v[185] / s.v[180]) > 0.001) || ((s.v[186] / s.v[181]) > 0.001)) || ((s.v[187] / s.v[182]) > 0.001)) && (s.v[185] < 0.0)) && (s.v[186] < 0.0)) && (s.v[187] < 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div(195, 185, 186);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad(196, A::scale(A::ln(s.ad_value(195)), (-s.v[84])), A::sub(s.ad_value(190), s.ad_value(191)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad_rhs(198, 191, A::sub(s.ad_value(191), s.ad_value(190)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_mul_ad(199, A::scale(A::offset(s.ad_value(195), (-1.0)), s.v[84]), A::offset(A::pow(s.ad_value(195), s.ad_value(198)), (-1.0)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad_rhs(198, 190, A::sub(s.ad_value(190), s.ad_value(191)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_sub_ad_lhs(200, A::add(A::mul(A::pow(s.ad_value(195), s.ad_value(198)), A::sub(s.ad_value(191), s.ad_value(190))), A::mul(s.ad_value(195), s.ad_value(190))), 191);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div(197, 199, 200);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_add(305, 196, 197);
        }

        s.v[853] = if (((((s.v[192] * s.v[85]) * s.v[305])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_scalar(306, 1.0);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_mul_ad_rhs(304, 187, A::add(A::div_from_scalar(1.0, s.ad_value(192)), A::scale(s.ad_value(305), (0.5 * s.v[85]))));
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_div_ad_lhs(305, A::scale(A::mul(A::scale(s.ad_value(187), (-0.5)), s.ad_value(305)), s.v[85]), 192);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (!(s.v[853] != 0.0))) {
            s.store_scalar(306, 0.0);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (!(s.v[853] != 0.0))) {
            s.store_div_ad(304, A::neg(s.ad_value(187)), A::offset(A::exp(A::mul(A::scale(A::neg(s.ad_value(192)), s.v[85]), s.ad_value(305))), (-1.0)));
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(208, A::add(A::add(A::scale(s.ad_value(128), s.v[256]), A::scale(s.ad_value(129), s.v[257])), A::scale(s.ad_value(130), s.v[258])), s.v[47]);
        }

        s.v[854] = if ((s.v[256] * s.v[128]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[854] != 0.0)) {
            s.store_scalar(259, 0.0);
        }

        s.v[855] = if ((s.v[257] * s.v[129]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[855] != 0.0)) {
            s.store_scalar(260, 0.0);
        }

        s.v[856] = if ((s.v[258] * s.v[130]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[856] != 0.0)) {
            s.store_scalar(261, 0.0);
        }

        s.v[857] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(294, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(300), 1e-21)));
        }

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(296, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(302), 1e-21)));
        }

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(298, A::div_from_scalar((0.5 * s.v[2]), A::offset(A::abs(s.ad_value(304)), 1e-21)));
        }

        if (s.v[418] != 0.0) {
            s.store_ad(294, &A::min_with_scalar(s.ad_value(294), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(295, 294);
        }

        if (s.v[418] != 0.0) {
            s.store_ad(296, &A::min_with_scalar(s.ad_value(296), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(297, 296);
        }

        if (s.v[418] != 0.0) {
            s.store_ad(298, &A::min_with_scalar(s.ad_value(298), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(299, 298);
        }

        s.store_ad(277, &A::voltage(ctx, &nodes, Some(0), Some(2)));

        s.v[858] = if (s.v[45] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(277), s.v[85]), 301);
        }

        if (s.v[858] != 0.0) {
            let assign28780_ad_e42653: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[294]) {
                            A::mul(s.ad_value(295), A::offset(A::sub(s.ad_value(201), s.ad_value(294)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28780_ad_e42653);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_rhs(209, 300, A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(277), s.v[85]), 303);
        }

        if (s.v[858] != 0.0) {
            let assign28810_ad_e42698: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[296]) {
                            A::mul(s.ad_value(297), A::offset(A::sub(s.ad_value(201), s.ad_value(296)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28810_ad_e42698);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_rhs(210, 302, A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(211, 0.0);
        }

        s.v[859] = if (s.v[306] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[858] != 0.0) && (s.v[859] != 0.0)) {
            s.store_mul_ad_rhs(211, 277, A::add(s.ad_value(304), A::mul(s.ad_value(277), s.ad_value(305))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            s.store_mul_ad_lhs(201, A::scale(A::neg(s.ad_value(277)), s.v[85]), 305);
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            let assign28870_ad_e42769: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[298]) {
                            A::mul(s.ad_value(299), A::offset(A::sub(s.ad_value(201), s.ad_value(298)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28870_ad_e42769);
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            s.store_mul_ad(211, A::neg(s.ad_value(304)), A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_add_ad_lhs(274, A::add(s.ad_value(209), s.ad_value(210)), 211);
        }

        if (s.v[858] != 0.0) {
            s.store_add(290, 210, 211);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(268, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(270, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(272, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(291, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(292, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(293, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(215, A::scale(s.ad_value(265), 4.0), 265);
        }

        if (s.v[858] != 0.0) {
            s.store_div(216, 265, 266);
        }

        if (s.v[858] != 0.0) {
            s.store_add_ad_rhs(217, 277, A::mul(s.ad_value(265), s.ad_value(216)));
        }

        if (s.v[858] != 0.0) {
            s.store_add(218, 266, 217);
        }

        if (s.v[858] != 0.0) {
            s.store_sub(219, 266, 217);
        }

        if (s.v[858] != 0.0) {
            s.store_sqrt_ad(220, A::add(A::square(s.ad_value(219)), s.ad_value(215)));
        }

        if (s.v[858] != 0.0) {
            s.store_scale_ad(204, A::div(A::mul(s.ad_value(277), s.ad_value(266)), A::add(s.ad_value(218), s.ad_value(220))), 2.0);
        }

        s.v[860] = if (s.v[259] > 0.5) { 1.0 } else { 0.0 };

        s.v[861] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[860] != 0.0)) && (s.v[861] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))));
        }

        if (((s.v[858] != 0.0) && (s.v[860] != 0.0)) && (!(s.v[861] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[858] != 0.0) && (s.v[860] != 0.0)) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[860] != 0.0))) {
            s.store_scalar(269, 0.0);
        }

        s.v[862] = if (s.v[260] > 0.5) { 1.0 } else { 0.0 };

        s.v[863] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[862] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))));
        }

        if (((s.v[858] != 0.0) && (s.v[862] != 0.0)) && (!(s.v[863] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[858] != 0.0) && (s.v[862] != 0.0)) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[862] != 0.0))) {
            s.store_scalar(271, 0.0);
        }

        s.v[864] = if (s.v[261] > 0.5) { 1.0 } else { 0.0 };

        s.v[865] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[864] != 0.0)) && (s.v[865] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))));
        }

        if (((s.v[858] != 0.0) && (s.v[864] != 0.0)) && (!(s.v[865] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[858] != 0.0) && (s.v[864] != 0.0)) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[864] != 0.0))) {
            s.store_scalar(273, 0.0);
        }

        s.v[866] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_mul_ad_lhs(215, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_div(216, 265, 266);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_add_ad_rhs(217, 277, A::mul(s.ad_value(265), s.ad_value(216)));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_add(218, 266, 217);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sub(219, 266, 217);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sqrt_ad(220, A::add(A::square(s.ad_value(219)), s.ad_value(215)));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(221, A::div(A::mul(s.ad_value(277), s.ad_value(266)), A::add(s.ad_value(218), s.ad_value(220))), 2.0);
        }

        s.v[867] = if (s.v[277] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[868] = if ((((0.5 * (s.v[277] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[868] != 0.0)) {
            s.store_exp_ad(223, A::scale(s.ad_value(277), (s.v[85] * 0.5)));
        }

        s.v[869] = if ((0.5 * (s.v[277] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[868] != 0.0))) && (s.v[869] != 0.0)) {
            let assign29340_ad_e43219: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(223, &assign29340_ad_e43219);
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[868] != 0.0))) && (!(s.v[869] != 0.0))) {
            s.store_scale_ad(223, A::offset(A::mul(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[870] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[870] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[870] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[871] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[871] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[872] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[871] != 0.0))) && (s.v[872] != 0.0)) {
            let assign29660_ad_e43814: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign29660_ad_e43814, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[871] != 0.0))) && (!(s.v[872] != 0.0))) {
            let assign29670_ad_e43893: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign29670_ad_e43893, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[873] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[873] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[873] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[874] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[874] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[875] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[874] != 0.0))) && (s.v[875] != 0.0)) {
            let assign29980_ad_e44444: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign29980_ad_e44444, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[874] != 0.0))) && (!(s.v[875] != 0.0))) {
            let assign29990_ad_e44523: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign29990_ad_e44523, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[876] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[876] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[876] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[877] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[877] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[878] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[877] != 0.0))) && (s.v[878] != 0.0)) {
            let assign30300_ad_e45074: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign30300_ad_e45074, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[877] != 0.0))) && (!(s.v[878] != 0.0))) {
            let assign30310_ad_e45153: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign30310_ad_e45153, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_sqrt_ad(223, A::mul(A::offset(A::scale(A::sub(s.ad_value(277), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[879] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[880] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[880] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[881] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[880] != 0.0))) && (s.v[881] != 0.0)) {
            let assign30670_ad_e45816: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign30670_ad_e45816, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[880] != 0.0))) && (!(s.v[881] != 0.0))) {
            let assign30680_ad_e45896: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign30680_ad_e45896, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[882] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

    }

    pub(super) fn stamp_reactive_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[883] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[883] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[884] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[883] != 0.0))) && (s.v[884] != 0.0)) {
            let assign31050_ad_e46586: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign31050_ad_e46586, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[883] != 0.0))) && (!(s.v[884] != 0.0))) {
            let assign31060_ad_e46666: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign31060_ad_e46666, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[885] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[886] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[886] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[887] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[886] != 0.0))) && (s.v[887] != 0.0)) {
            let assign31430_ad_e47356: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign31430_ad_e47356, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[886] != 0.0))) && (!(s.v[887] != 0.0))) {
            let assign31440_ad_e47436: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign31440_ad_e47436, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_div_from_scalar(222, 1.0, 223);
        }

        s.v[888] = if (s.v[277] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[888] != 0.0)) {
            s.store_scale_ad(224, A::ln(A::add(A::offset(s.ad_value(222), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(222), 1.0), A::offset(s.ad_value(222), 3.0))))), (s.v[84] * 2.0));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[888] != 0.0))) {
            s.store_sub_ad_lhs(224, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(223), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(223), 1.0), A::offset(A::scale(s.ad_value(223), 3.0), 1.0))))), (s.v[84] * 2.0)), 277);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sub(225, 264, 224);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(226, A::sub(A::add(s.ad_value(277), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(225)), A::sub(s.ad_value(277), s.ad_value(225))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(227, A::sub(A::add(s.ad_value(277), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(277), s.ad_value(267)), A::sub(s.ad_value(277), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(228, A::sub(s.ad_value(277), A::sqrt(A::offset(A::mul(s.ad_value(277), s.ad_value(277)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(224, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(221, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(223, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(226, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(227, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        s.v[889] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[890] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[890] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[890] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))), s.v[122]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul(230, 101, 370);
        }

        s.v[891] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_sub(232, 107, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[892] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (s.v[892] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (!(s.v[892] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[9])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[893] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (s.v[893] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[143]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (!(s.v[893] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[143]), s.v[9]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_scale(236, 229, s.v[137]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_mul_ad_rhs(237, 98, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[20]);
        }

        s.v[894] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[894] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[122]), s.ad_value(232)), s.v[152]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[149]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[895] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[895] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[895] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[149]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[149])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[896] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[896] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[896] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[897] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[897] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[897] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[898] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[898] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[899] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) && (s.v[899] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) && (!(s.v[899] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[149]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[23]);
        }

        s.v[900] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[900] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[901] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (s.v[901] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[901] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]), s.v[9]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[140]), s.ad_value(229)), s.v[125]);
        }

        s.v[902] = if (((((-s.v[155]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (s.v[902] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(155)), s.ad_value(254)));
        }

        s.v[903] = if (((-s.v[155]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
            let assign32350_ad_e48745: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign32350_ad_e48745);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
            let assign32360_ad_e48794: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign32360_ad_e48794);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[29]);
        }

        s.v[904] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[904] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[905] = if (s.v[228] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[906] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) && (s.v[906] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(162))), A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) && (!(s.v[906] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(162))), s.v[41]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (!(s.v[905] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        s.v[907] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[908] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[908] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[908] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))), s.v[123]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul(230, 102, 371);
        }

        s.v[909] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_sub(232, 108, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[910] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (s.v[910] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (!(s.v[910] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[10])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[911] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (s.v[911] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[144]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (!(s.v[911] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[144]), s.v[10]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_scale(236, 229, s.v[138]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_mul_ad_rhs(237, 99, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[21]);
        }

        s.v[912] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[912] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[123]), s.ad_value(232)), s.v[153]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[150]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[913] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[913] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[150]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[150])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[914] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[914] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[914] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[915] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[915] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[915] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[916] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[916] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[917] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) && (s.v[917] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) && (!(s.v[917] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[150]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[24]);
        }

        s.v[918] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[918] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[919] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (s.v[919] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[919] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]), s.v[10]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[141]), s.ad_value(229)), s.v[126]);
        }

        s.v[920] = if (((((-s.v[156]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (s.v[920] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(156)), s.ad_value(254)));
        }

        s.v[921] = if (((-s.v[156]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[920] != 0.0))) && (s.v[921] != 0.0)) {
            let assign33160_ad_e49964: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign33160_ad_e49964);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[920] != 0.0))) && (!(s.v[921] != 0.0))) {
            let assign33170_ad_e50013: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign33170_ad_e50013);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[30]);
        }

        s.v[922] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[922] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[923] = if (s.v[228] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[924] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) && (s.v[924] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) && (!(s.v[924] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(163))), s.v[42]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        s.v[925] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[926] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[926] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[926] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))), s.v[124]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul(230, 103, 372);
        }

        s.v[927] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_sub(232, 109, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[928] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (s.v[928] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (!(s.v[928] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[11])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[929] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (s.v[929] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[145]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (!(s.v[929] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[145]), s.v[11]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_scale(236, 229, s.v[139]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_mul_ad_rhs(237, 100, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[22]);
        }

        s.v[930] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[930] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[124]), s.ad_value(232)), s.v[154]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[151]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[931] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[931] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[931] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[151]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[151])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[932] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[932] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[932] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[933] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[933] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[933] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[934] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[934] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[935] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) && (!(s.v[935] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[151]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[25]);
        }

        s.v[936] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[936] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[937] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (s.v[937] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[937] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]), s.v[11]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[142]), s.ad_value(229)), s.v[127]);
        }

        s.v[938] = if (((((-s.v[157]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (s.v[938] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(157)), s.ad_value(254)));
        }

        s.v[939] = if (((-s.v[157]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            let assign33970_ad_e51183: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign33970_ad_e51183);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            let assign33980_ad_e51232: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign33980_ad_e51232);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[31]);
        }

        s.v[940] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[940] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[941] = if (s.v[228] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[942] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) && (s.v[942] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(164))), A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) && (!(s.v[942] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(164))), s.v[43]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (!(s.v[941] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if (!(s.v[858] != 0.0)) {
            s.store_add_ad(274, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        if (!(s.v[858] != 0.0)) {
            s.store_add_ad(290, A::add(A::scale(s.ad_value(291), s.v[256]), A::scale(s.ad_value(292), s.v[257])), A::scale(s.ad_value(293), s.v[258]));
        }

        s.store_add_ad(275, A::add(A::scale(s.ad_value(269), s.v[256]), A::scale(s.ad_value(271), s.v[257])), A::scale(s.ad_value(273), s.v[258]));

        s.v[945] = if (p.p84 > 0.0) { 1.0 } else { 0.0 };

        s.v[946] = if (s.v[313] < p.p85) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(349, A::scale(A::sub(s.ad_value(277), s.ad_value(348)), p.p86), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[313], A::scale(s.ad_value(348), p.p86));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(349)), (-0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(351, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(352, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (!(s.v[946] != 0.0))) {
            s.store_scalar(352, s.v[313]);
        }

        if ((s.v[945] != 0.0) && (!(s.v[946] != 0.0))) {
            s.store_scalar(350, s.v[313]);
        }

        if (s.v[945] != 0.0) {
            s.copy_ad(353, 370);
        }

        s.v[947] = if ((s.v[277] - (s.v[348] - s.v[347])) > 0.0) { 1.0 } else { 0.0 };

        s.v[948] = if ((((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (s.v[948] != 0.0)) {
            s.store_exp_ad(354, A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[949] = if ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (!(s.v[948] != 0.0))) && (s.v[949] != 0.0)) {
            let assign34480_ad_e51901: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            let assign34480_ad_e51905: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign34480_ad_e51901, 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(354, 1e-100, assign34480_ad_e51905);
        }

        if ((((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (!(s.v[948] != 0.0))) && (!(s.v[949] != 0.0))) {
            let assign34490_ad_e51995: A = A::mul(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            let assign34490_ad_e51999: A = A::offset(A::mul(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign34490_ad_e51995, 0.5), 1.0)), 1.0);
            s.store_scale_ad(354, assign34490_ad_e51999, 1e100);
        }

        if ((s.v[945] != 0.0) && (!(s.v[947] != 0.0))) {
            s.store_scalar(354, 1.0);
        }

        s.v[950] = if ((p.p91 == 0.0) || (s.v[277] < s.v[347])) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[950] != 0.0)) {
            s.store_scale(357, 353, p.p90);
        }

        if ((s.v[945] != 0.0) && (!(s.v[950] != 0.0))) {
            s.store_mul_ad(357, A::scale(s.ad_value(353), p.p90), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(277), s.ad_value(347)), (-p.p91)), A::sub(s.ad_value(277), s.ad_value(347))), A::exp(A::scale(A::ln(A::scale(s.ad_value(78), 1.0 / (s.v[79]))), p.p98)))));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(357, &{
                if (s.v[357] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(357)
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_mul(355, 319, 357);
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_sub(331, 355, 319, (1.6021918e-19 * s.v[256]));
        }

        s.v[951] = if (p.p92 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scale(334, 331, (1e-23 / s.v[333]));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_ad(336, &A::voltage(ctx, &nodes, Some(3), None));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scale(340, 336, 1.0 / ((1e-23 / s.v[333])));
        }

        if ((s.v[945] != 0.0) && (!(s.v[951] != 0.0))) {
            s.copy_ad(334, 331);
        }

        if ((s.v[945] != 0.0) && (!(s.v[951] != 0.0))) {
            s.copy_ad(340, 334);
        }

        s.v[952] = if ((p.p91 == 0.0) || (s.v[277] < s.v[348])) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[952] != 0.0)) {
            s.store_scale(358, 354, p.p90);
        }

        if ((s.v[945] != 0.0) && (!(s.v[952] != 0.0))) {
            s.store_mul_ad(358, A::scale(s.ad_value(354), p.p90), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(277), s.ad_value(348)), (-p.p91)), A::sub(s.ad_value(277), s.ad_value(348))), A::exp(A::scale(A::ln(A::scale(s.ad_value(78), 1.0 / (s.v[79]))), p.p98)))));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(358, &{
                if (s.v[358] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(358)
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_mul(356, 319, 358);
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_sub(332, 356, 319, (1.6021918e-19 * s.v[256]));
        }

        s.v[953] = if (p.p92 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scale(335, 332, (1e-23 / s.v[333]));
        }

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_ad(337, &A::voltage(ctx, &nodes, Some(4), None));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scaled_sub(339, 337, 335, 1.0 / (p.p92));
        }

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scale(341, 337, 1.0 / ((1e-23 / s.v[333])));
        }

        if ((s.v[945] != 0.0) && (!(s.v[953] != 0.0))) {
            s.copy_ad(335, 332);
        }

        if ((s.v[945] != 0.0) && (!(s.v[953] != 0.0))) {
            s.copy_ad(341, 335);
        }

        if (s.v[945] != 0.0) {
            s.store_sub_from_scalar(325, s.v[368], 277);
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(315, A::offset(A::square(s.ad_value(325)), ((4.0 * s.v[369]) * s.v[369])));
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_add(325, 325, 315, 0.5);
        }

        s.v[954] = if (s.v[325] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[954] != 0.0)) {
            s.store_scalar(325, 0.0);
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(326, A::scale(s.ad_value(325), ((2.0 * s.v[0]) * 1.0 / ((1.6021918e-19 * s.v[307])))));
        }

        if (s.v[945] != 0.0) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p94, s.ad_value(326)), (-1e-7));
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if (s.v[945] != 0.0) {
            s.store_sub_from_scalar_ad(326, p.p94, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        s.v[955] = if (p.p95 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_mul_ad_rhs(342, 326, A::div_from_scalar(1.0, s.ad_value(343)));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_ad(344, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_div_ad_rhs(346, 344, A::div_from_scalar(1.0, s.ad_value(343)));
        }

        if ((s.v[945] != 0.0) && (!(s.v[955] != 0.0))) {
            s.copy_ad(342, 326);
        }

        if ((s.v[945] != 0.0) && (!(s.v[955] != 0.0))) {
            s.copy_ad(346, 342);
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(327, ((-((s.v[307] * s.v[256]) * 1.6021918e-19)) * p.p94));
        }

        if (s.v[945] != 0.0) {
            s.store_mul_ad(328, A::mul(s.ad_value(323), s.ad_value(340)), A::sub(A::exp(A::div_from_scalar((-p.p94), s.ad_value(323))), A::exp(A::div(A::neg(s.ad_value(346)), s.ad_value(323)))));
        }

        if (s.v[945] != 0.0) {
            s.store_mul_ad(329, A::mul(s.ad_value(323), s.ad_value(341)), A::offset(A::exp(A::div(A::neg(A::sub_from_scalar(p.p94, s.ad_value(346))), s.ad_value(323))), (-1.0)));
        }

        if (s.v[945] != 0.0) {
            s.store_neg_ad(330, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (s.v[945] != 0.0) {
            s.store_add(275, 275, 330);
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(55, 0.0);
        }

        if (!(s.v[945] != 0.0)) {
            s.store_mul_ad_rhs(330, 55, A::sub(s.ad_value(274), s.ad_value(290)));
        }

        s.v[958] = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };

        s.v[959] = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };

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
        let eq0_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq0_value),
            &[
            ],
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
        let eq1_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq1_value),
            &[
            ],
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
        let eq2_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[1]),
            self.multiplicity * (eq2_value),
            &[
            ],
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
        let eq3_value: f64 = s.v[274];
        let eq3_node_derivatives: [f64; 6] = [s.dn[274][0], s.dn[274][1], s.dn[274][2], s.dn[274][3], s.dn[274][4], s.dn[274][5]];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let eq4_e122: f64 = 0.0;
        let eq4_e124: f64 = (eq4_e122 * (nv0 - nv2));
        let eq4_e124_d_n2: f64 = (-eq4_e122);
        let eq4_value: f64 = eq4_e124;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq4_e122),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq4_e124_d_n2),
            ],
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
        let (eq5_e130, eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5,) = {
    if (s.v[957] != 0.0) {
        let eq5_e128: f64 = (s.v[284] / s.v[171]);
        let eq5_e128_d_n0: f64 = (((s.dn[284][0] * s.v[171]) - (s.v[284] * s.dn[171][0])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n1: f64 = (((s.dn[284][1] * s.v[171]) - (s.v[284] * s.dn[171][1])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n2: f64 = (((s.dn[284][2] * s.v[171]) - (s.v[284] * s.dn[171][2])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n3: f64 = (((s.dn[284][3] * s.v[171]) - (s.v[284] * s.dn[171][3])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n4: f64 = (((s.dn[284][4] * s.v[171]) - (s.v[284] * s.dn[171][4])) / (s.v[171] * s.v[171]));
        let eq5_e128_d_n5: f64 = (((s.dn[284][5] * s.v[171]) - (s.v[284] * s.dn[171][5])) / (s.v[171] * s.v[171]));
        (eq5_e128, eq5_e128_d_n0, eq5_e128_d_n1, eq5_e128_d_n2, eq5_e128_d_n3, eq5_e128_d_n4, eq5_e128_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e130;
        let eq5_node_derivatives: [f64; 6] = [eq5_e130_d_n0, eq5_e130_d_n1, eq5_e130_d_n2, eq5_e130_d_n3, eq5_e130_d_n4, eq5_e130_d_n5];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[1]),
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
        let (eq6_e135,) = {
    if (!(s.v[957] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e135;
        stamper.stamp_potential(
            branches[0],
            eq6_value,
            &[
            ],
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
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5,) = {
    if (s.v[958] != 0.0) {
        let eq7_e140: f64 = self.eval_ddt(0, s.v[336]);
        let eq7_e140_d_n0: f64 = self.ddt_jacobian(s.dn[336][0]);
        let eq7_e140_d_n1: f64 = self.ddt_jacobian(s.dn[336][1]);
        let eq7_e140_d_n2: f64 = self.ddt_jacobian(s.dn[336][2]);
        let eq7_e140_d_n3: f64 = self.ddt_jacobian(s.dn[336][3]);
        let eq7_e140_d_n4: f64 = self.ddt_jacobian(s.dn[336][4]);
        let eq7_e140_d_n5: f64 = self.ddt_jacobian(s.dn[336][5]);
        let eq7_e141: f64 = (s.v[338] + eq7_e140);
        let eq7_e141_d_n0: f64 = (s.dn[338][0] + eq7_e140_d_n0);
        let eq7_e141_d_n1: f64 = (s.dn[338][1] + eq7_e140_d_n1);
        let eq7_e141_d_n2: f64 = (s.dn[338][2] + eq7_e140_d_n2);
        let eq7_e141_d_n3: f64 = (s.dn[338][3] + eq7_e140_d_n3);
        let eq7_e141_d_n4: f64 = (s.dn[338][4] + eq7_e140_d_n4);
        let eq7_e141_d_n5: f64 = (s.dn[338][5] + eq7_e140_d_n5);
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e144;
        let eq7_node_derivatives: [f64; 6] = [eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            None,
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
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5,) = {
    if (s.v[958] != 0.0) {
        let eq8_e149: f64 = self.eval_ddt(1, s.v[337]);
        let eq8_e149_d_n0: f64 = self.ddt_jacobian(s.dn[337][0]);
        let eq8_e149_d_n1: f64 = self.ddt_jacobian(s.dn[337][1]);
        let eq8_e149_d_n2: f64 = self.ddt_jacobian(s.dn[337][2]);
        let eq8_e149_d_n3: f64 = self.ddt_jacobian(s.dn[337][3]);
        let eq8_e149_d_n4: f64 = self.ddt_jacobian(s.dn[337][4]);
        let eq8_e149_d_n5: f64 = self.ddt_jacobian(s.dn[337][5]);
        let eq8_e150: f64 = (s.v[339] + eq8_e149);
        let eq8_e150_d_n0: f64 = (s.dn[339][0] + eq8_e149_d_n0);
        let eq8_e150_d_n1: f64 = (s.dn[339][1] + eq8_e149_d_n1);
        let eq8_e150_d_n2: f64 = (s.dn[339][2] + eq8_e149_d_n2);
        let eq8_e150_d_n3: f64 = (s.dn[339][3] + eq8_e149_d_n3);
        let eq8_e150_d_n4: f64 = (s.dn[339][4] + eq8_e149_d_n4);
        let eq8_e150_d_n5: f64 = (s.dn[339][5] + eq8_e149_d_n5);
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e153;
        let eq8_node_derivatives: [f64; 6] = [eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let (eq9_e158,) = {
    if (!(s.v[958] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e158;
        stamper.stamp_potential(
            branches[1],
            eq9_value,
            &[
            ],
        );
    }
}
