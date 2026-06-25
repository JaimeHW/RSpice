#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign11690_loop_guard: usize = 0;
        while {
            let assign11690_cond_e14989: f64 = if s.v[63] <= s.v[29] { 1.0 } else { 0.0 };
            assign11690_cond_e14989 != 0.0
        } {
            assign11690_loop_guard += 1;
            assert!(assign11690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 310);
            s.store_mul(297, 120, 279);
            s.store_exp_ad(278, A::neg(s.ad_value(297)));
            s.v[855] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[855] != 0.0) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if (s.v[855] != 0.0) {
                s.store_mul_ad_rhs(314, 439, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))));
            }
            if (s.v[855] != 0.0) {
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 314);
            }
            s.v[856] = if (s.v[279] > (1e-8 / 10.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_mul_ad(314, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
            }
            if ((!(s.v[855] != 0.0)) && (s.v[856] != 0.0)) {
                s.store_div_ad_lhs(344, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 314);
            }
            if ((!(s.v[855] != 0.0)) && (!(s.v[856] != 0.0))) {
                s.store_scale_ad(314, A::mul(A::neg(s.ad_value(439)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[855] != 0.0)) && (!(s.v[856] != 0.0))) {
                s.store_scale_ad(344, A::mul(A::neg(s.ad_value(439)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            s.store_add_ad_lhs(309, A::add(A::sub(s.ad_value(310), A::scale(s.ad_value(314), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(582, 1.0, A::scale(s.ad_value(344), 1.0 / (s.v[294])));
            s.store_sub(279, 308, 584);
            s.store_mul(297, 120, 279);
            s.v[857] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[857] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[857] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[857] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[857] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[858] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[858] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[858] != 0.0) {
                s.store_mul(576, 141, 280);
            }
            if (s.v[858] != 0.0) {
                s.store_div_ad(577, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[858] != 0.0) {
                s.store_neg(578, 577);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(313, 0.0);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(579, 0.0);
            }
            if (s.v[858] != 0.0) {
                s.store_scalar(580, 0.0);
            }
            s.v[859] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_mul_ad_lhs(576, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(577, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_neg(578, 577);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(576)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(537, A::add(A::div(A::mul(A::scale(s.ad_value(576), 2.0), s.ad_value(577)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(576), 2.0), s.ad_value(578)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(313, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 576);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(579, A::mul(A::neg(s.ad_value(141)), s.ad_value(537)), 577);
            }
            if ((!(s.v[858] != 0.0)) && (s.v[859] != 0.0)) {
                s.store_sub_ad_lhs(580, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 578);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scale_ad(576, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scale_ad(577, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_neg(578, 577);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(313, 0.0);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(579, 0.0);
            }
            if ((!(s.v[858] != 0.0)) && (!(s.v[859] != 0.0))) {
                s.store_scalar(580, 0.0);
            }
            s.store_sub(279, 309, 584);
            s.store_mul(297, 120, 279);
            s.v[860] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[860] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[860] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[860] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[860] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[861] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[861] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[861] != 0.0) {
                s.store_mul(585, 141, 280);
            }
            if (s.v[861] != 0.0) {
                s.store_div_ad(586, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[861] != 0.0) {
                s.store_neg(587, 586);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(588, 0.0);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(589, 0.0);
            }
            if (s.v[861] != 0.0) {
                s.store_scalar(590, 0.0);
            }
            s.v[862] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_mul_ad_lhs(585, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(586, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_neg(587, 586);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), A::sub(s.ad_value(584), s.ad_value(51))));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(585)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(539, A::add(A::div(A::mul(A::scale(s.ad_value(585), 2.0), s.ad_value(586)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(585), 2.0), s.ad_value(587)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(588, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 585);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(589, A::mul(A::neg(s.ad_value(141)), s.ad_value(539)), 586);
            }
            if ((!(s.v[861] != 0.0)) && (s.v[862] != 0.0)) {
                s.store_sub_ad_lhs(590, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 587);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scale_ad(585, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scale_ad(586, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_neg(587, 586);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(588, 0.0);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(589, 0.0);
            }
            if ((!(s.v[861] != 0.0)) && (!(s.v[862] != 0.0))) {
                s.store_scalar(590, 0.0);
            }
            s.v[863] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if (s.v[863] != 0.0) {
                s.store_scalar(574, s.v[63]);
            }
            if (s.v[863] != 0.0) {
                s.store_scalar(63, s.v[29]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(346, A::sub(s.ad_value(308), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(314), s.ad_value(313)), s.ad_value(576)), s.ad_value(588)), s.ad_value(585)), s.ad_value(337)), s.ad_value(270)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(579), s.ad_value(577)), s.ad_value(270)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(580), s.ad_value(578)), s.ad_value(590)), s.ad_value(587))), 270);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(344), A::mul(A::add(s.ad_value(589), s.ad_value(586)), s.ad_value(582)))), 270);
            }
            s.v[864] = if (s.v[314] <= s.v[599]) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[864] != 0.0)) {
                s.store_sqrt_ad(279, A::mul(s.ad_value(296), A::add(A::scale(s.ad_value(314), 2.0), s.ad_value(296))));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[864] != 0.0)) {
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(344)), 279);
            }
            s.v[865] = if (s.v[314] <= s.v[603]) { 1.0 } else { 0.0 };
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (s.v[865] != 0.0)) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(602)));
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (s.v[865] != 0.0)) {
                s.store_mul_ad_lhs(604, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(314), s.ad_value(603))), A::sub(s.ad_value(314), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(314), s.ad_value(602)), 3.0), A::sub(s.ad_value(314), s.ad_value(603)))), 344);
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (!(s.v[865] != 0.0))) {
                s.store_scalar(279, 0.0);
            }
            if (((!(s.v[863] != 0.0)) && (!(s.v[864] != 0.0))) && (!(s.v[865] != 0.0))) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[650]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scaled_add(350, 576, 279, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(351, 577, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(352, 578, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(353, 604, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[651]);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(605, 0.0);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scaled_add(354, 585, 280, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(355, 587, 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale_ad(356, A::add(A::mul(s.ad_value(586), s.ad_value(582)), s.ad_value(605)), 1.0 / (s.v[535]));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.v[866] = if (s.v[357] > 0.0) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[866] != 0.0)) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), 1e-50));
            }
            if ((!(s.v[863] != 0.0)) && (!(s.v[866] != 0.0))) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), (-1e-50)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad_lhs(362, A::neg(s.ad_value(351)), 356);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(363, 347, 356);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul(365, 351, 355);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad_lhs(366, A::neg(s.ad_value(347)), 355);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(368, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(369, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_mul_ad(370, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(368)));
            }
            s.v[867] = if (s.v[279] < ((s.v[369]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[867] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(369)));
            }
            s.v[868] = if (s.v[279] < ((s.v[370]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[868] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(370)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scalar(606, 1.0);
            }
            s.v[869] = if (s.v[63] > 80.0) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[869] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[870] = if (s.v[63] > 40.0) { 1.0 } else { 0.0 };
            if (((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (s.v[870] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[871] = if (s.v[63] > 20.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (!(s.v[870] != 0.0))) && (s.v[871] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[872] = if (s.v[63] > 10.0) { 1.0 } else { 0.0 };
            if (((((!(s.v[863] != 0.0)) && (!(s.v[869] != 0.0))) && (!(s.v[870] != 0.0))) && (!(s.v[871] != 0.0))) && (s.v[872] != 0.0)) {
                s.store_scalar(606, 5.0);
            }
            s.v[873] = if (s.v[279] > (0.1 / s.v[606])) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(368, 368, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(369, 369, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[863] != 0.0)) && (s.v[873] != 0.0)) {
                s.store_mul_ad_rhs(370, 370, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(308, 308, 368);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(584, 584, 369);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_add(310, 310, 370);
            }
            if (!(s.v[863] != 0.0)) {
                s.store_scale(607, 606, 1e-12);
            }
            s.v[874] = if (s.v[279] < s.v[607]) { 1.0 } else { 0.0 };
            if ((!(s.v[863] != 0.0)) && (s.v[874] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(63, 63, 1.0);
        }

        s.v[875] = if (s.v[574] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[875] != 0.0) {
            s.copy_ad(63, 574);
        }

        if (s.v[875] != 0.0) {
            s.store_scalar(574, 0.0);
        }

        s.v[876] = if (s.v[63] > s.v[29]) { 1.0 } else { 0.0 };

        if (s.v[876] != 0.0) {
            s.copy_ad(308, 302);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(309, 303);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(310, 304);
        }

        if (s.v[876] != 0.0) {
            s.copy_ad(584, 581);
        }

        s.copy_ad(57, 308);

        s.store_sub(59, 57, 56);

        s.copy_ad(51, 396);

        s.v[878] = if ((s.v[292] <= (-1.0)) || (s.v[305] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[878] != 0.0) {
            s.store_scalar(34, 1.0);
        }

        s.copy_ad(317, 305);

        s.copy_ad(318, 308);

        s.store_sub(59, 318, 317);

        s.copy_ad(322, 306);

        s.copy_ad(323, 309);

        s.store_sub(155, 323, 322);

        s.store_sub_ad(153, A::sub(s.ad_value(313), s.ad_value(311)), A::scale(A::mul(A::mul(s.ad_value(120), A::add(s.ad_value(313), s.ad_value(311))), A::sub(s.ad_value(318), s.ad_value(317))), 0.5));

        s.store_sub_ad(154, A::sub(s.ad_value(588), s.ad_value(528)), A::scale(A::mul(A::mul(s.ad_value(120), A::add(s.ad_value(588), s.ad_value(528))), A::sub(s.ad_value(323), s.ad_value(322))), 0.5));

        s.v[879] = if ((s.v[153] < 0.0) || (s.v[51] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[879] != 0.0) {
            s.store_scalar(153, 0.0);
        }

        s.v[880] = if ((s.v[154] < 0.0) || (s.v[51] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[880] != 0.0) {
            s.store_scalar(154, 0.0);
        }

        s.store_add(151, 153, 154);

        s.store_scaled_add(384, 576, 523, (-0.5));

        s.store_offset_ad(371, A::sub(s.ad_value(308), s.ad_value(305)), 1e-12);

        s.store_neg_ad(373, A::sub(s.ad_value(313), s.ad_value(311)));

        s.v[881] = if ((-s.v[373]) < 1e-18) { 1.0 } else { 0.0 };

        if (s.v[881] != 0.0) {
            s.store_scalar(373, 0.0);
        }

        s.store_offset_ad(372, A::div(A::scale(A::neg(s.ad_value(373)), 2.0), A::mul(A::mul(A::mul(s.ad_value(120), s.ad_value(270)), s.ad_value(371)), s.ad_value(371))), 1.0);

        s.store_sub_from_scalar_ad(85, 1.0, A::div(A::mul(s.ad_value(372), s.ad_value(371)), s.ad_value(86)));

        s.v[882] = if (s.v[85] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[882] != 0.0) {
            s.store_scalar(85, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_scaled_add(383, 311, 313, (-0.5));

        s.store_scaled_add(167, 528, 588, (-0.5));

        s.v[262] = 0.0;

        s.v[883] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        s.v[884] = if ((s.v[446] < (10.0 * 2.220446049250313e-16)) && (p.p178 < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[883] != 0.0) && (s.v[884] != 0.0)) {
            s.store_scalar(262, 0.0);
        }

        if ((s.v[883] != 0.0) && (s.v[884] != 0.0)) {
            s.copy_ad(260, 57);
        }

        s.v[885] = if (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (s.v[884] != 0.0)) && (s.v[885] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scalar(263, p.p227);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_from_scalar_ad(282, 1.034943e-10, A::add(A::mul(s.ad_value(446), s.ad_value(126)), A::div(A::scale(s.ad_value(149), p.p178), s.ad_value(263))));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_add_ad(260, A::scale(A::add(s.ad_value(51), s.ad_value(56)), p.p176), A::scale(s.ad_value(57), (1.0 - p.p176)));
        }

        s.v[886] = if (s.v[260] > ((s.v[56] + s.v[71]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[886] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sub(284, 260, 57);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[887] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[887] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if (((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) && (s.v[887] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_ad_rhs(283, 151, A::mul(s.ad_value(120), s.ad_value(149)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale(288, 126, 9662367879.197212);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scalar(279, 1000000000.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_div_ad_lhs(387, A::add(A::add(A::scale(s.ad_value(283), 2.0), A::mul(A::mul(A::scale(s.ad_value(288), 2.0), s.ad_value(284)), s.ad_value(282))), A::mul(s.ad_value(279), s.ad_value(282))), 123);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul(285, 387, 282);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scale_ad(387, A::add(A::mul(A::scale(s.ad_value(288), 2.0), s.ad_value(284)), s.ad_value(279)), 4.0);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul_ad_lhs(286, A::mul(s.ad_value(387), s.ad_value(282)), 282);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_sqrt_ad(287, A::add(A::square(s.ad_value(285)), s.ad_value(286)));
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_scaled_sub(262, 287, 285, 0.5);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.copy_ad(279, 262);
        }

        if ((s.v[883] != 0.0) && (!(s.v[884] != 0.0))) {
            s.store_mul(262, 276, 279);
        }

        if (s.v[883] != 0.0) {
            s.store_scale(262, 262, s.v[483]);
        }

        s.store_sub(386, 123, 262);

        s.v[888] = if (s.v[386] < 1e-9) { 1.0 } else { 0.0 };

        if (s.v[888] != 0.0) {
            s.store_scalar(386, 1e-9);
        }

        s.store_mul_ad(91, A::scale(s.ad_value(123), (-s.v[513])), A::add(s.ad_value(383), s.ad_value(167)));

        s.store_scale_ad(336, A::mul(A::scale(A::add(s.ad_value(312), s.ad_value(314)), 0.5), s.ad_value(123)), s.v[513]);

        s.store_scaled_sub(279, 51, 59, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p217)));

        s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);

        s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));

        s.store_div_from_scalar(75, p.p217, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.v[889] = if (s.v[75] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (s.v[889] != 0.0) {
            s.store_scalar(75, (10.0 * 2.220446049250313e-16));
        }

        s.store_add(74, 56, 75);

        s.v[499] = (1.034943e-10 / 100.0);

        s.store_scale(500, 313, 0.0001);

        s.store_scale(501, 588, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(502, 383, 0.0001);

        s.store_scale(503, 167, 0.0001);

        s.store_scale(504, 531, 0.0001);

        s.store_scale(505, 585, 0.0001);

        s.store_scale(506, 384, 0.0001);

        s.v[507] = (p.p229 * 100.0);

        s.v[591] = ((p.p81 * (1.0 + (p.p82 / ((s.v[375]) as f64).powf(p.p83)))) / s.v[499]);

        s.v[592] = ((p.p78 * (1.0 + (p.p79 / ((s.v[375]) as f64).powf(p.p80)))) / s.v[499]);

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(59)), ((4.0 * 1e-6) * 1e-6)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(59), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(598, A::scale(A::add(s.ad_value(59), s.ad_value(639)), 0.5), (1e-10 * 1e-6));

        s.v[890] = if (s.v[598] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[890] != 0.0) {
            s.store_scalar(598, 0.0);
        }

        if (s.v[890] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.store_offset_ad(168, A::sqrt(A::offset(A::square(s.ad_value(598)), p.p216)), (-((p.p216) as f64).sqrt()));

        s.store_powf(168, 168, p.p85);

        s.store_offset_scaled(282, 168, p.p84, 1.0);

        s.v[497] = (p.p299 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301))));

        s.store_sub_ad_rhs(288, 502, A::scale(s.ad_value(501), s.v[497]));

        s.store_add_ad(283, A::scale(s.ad_value(506), s.v[592]), A::scale(s.ad_value(288), s.v[591]));

        s.store_div(156, 283, 282);

        if (p.p32 != 0.0) {
            s.store_scaled_add(596, 306, 309, 0.5);
        }

        if (p.p32 != 0.0) {
            s.store_scaled_add(597, 307, 310, 0.5);
        }

        if (p.p32 != 0.0) {
            s.store_scale_ad(163, A::sub(A::sub(s.ad_value(596), s.ad_value(597)), s.ad_value(440)), (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (p.p32 != 0.0) {
            s.store_add(156, 156, 163);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(596, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(597, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(163, 0.0);
        }

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(156)), ((4.0 * 3000.0) * 3000.0)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(156), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(156, A::scale(A::add(s.ad_value(156), s.ad_value(639)), 0.5), (1e-10 * 3000.0));

        s.v[891] = if (s.v[156] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[891] != 0.0) {
            s.store_scalar(156, 0.0);
        }

        if (s.v[891] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 156, p.p94);

        s.store_powf(284, 156, s.v[470]);

        s.store_scale(157, 502, 6.241449993689894e18);

        s.store_add_ad(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[449] * 1e-11)), s.v[448])), A::mul(s.ad_value(469), s.ad_value(286))), A::scale(s.ad_value(284), 1.0 / (p.p105)));

        s.store_div_from_scalar(159, 1.0, 279);

        s.store_scale(159, 159, 0.0001);

        if (p.p32 != 0.0) {
            s.store_scaled_sub(163, 596, 597, (3.9 * 1.0 / ((11.7 * s.v[507]))));
        }

        if (!(p.p32 != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(155)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (!(p.p32 != 0.0)) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(155), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_ad(598, A::scale(A::add(s.ad_value(155), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
        }

        s.v[892] = if (s.v[598] < 0.0) { 1.0 } else { 0.0 };

        if ((!(p.p32 != 0.0)) && (s.v[892] != 0.0)) {
            s.store_scalar(598, 0.0);
        }

        if ((!(p.p32 != 0.0)) && (s.v[892] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_ad(168, A::sqrt(A::offset(A::square(s.ad_value(598)), p.p216)), (-((p.p216) as f64).sqrt()));
        }

        if (!(p.p32 != 0.0)) {
            s.store_powf(168, 168, p.p85);
        }

        if (!(p.p32 != 0.0)) {
            s.store_offset_scaled(282, 168, p.p84, 1.0);
        }

        if (!(p.p32 != 0.0)) {
            s.store_scalar(498, (p.p302 * (1.0 + (p.p300 / ((s.v[375]) as f64).powf(p.p301)))));
        }

        if (!(p.p32 != 0.0)) {
            s.store_sub_ad_rhs(288, 503, A::mul(s.ad_value(498), s.ad_value(500)));
        }

        if (!(p.p32 != 0.0)) {
            s.store_scaled_add(508, 505, 504, (-0.5));
        }

        if (!(p.p32 != 0.0)) {
            s.store_add_ad(283, A::scale(s.ad_value(508), s.v[592]), A::scale(s.ad_value(288), s.v[591]));
        }

        if (!(p.p32 != 0.0)) {
            s.store_div(163, 283, 282);
        }

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(163)), ((4.0 * 30.0) * 30.0)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(163), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(163, A::scale(A::add(s.ad_value(163), s.ad_value(639)), 0.5), (1e-10 * 30.0));

        s.v[893] = if (s.v[163] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[893] != 0.0) {
            s.store_scalar(163, 0.0);
        }

        if (s.v[893] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        s.store_powf(286, 163, p.p275);

        s.store_powf(284, 163, s.v[594]);

        s.store_scale(157, 503, 6.241449993689894e18);

        s.store_add_ad(279, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(157), (s.v[451] * 1e-11)), s.v[450])), A::mul(s.ad_value(595), s.ad_value(286))), A::scale(s.ad_value(284), 1.0 / (p.p284)));

        s.store_div_from_scalar(166, 1.0, 279);

        s.store_scale(166, 166, 0.0001);

        s.store_div_ad_lhs(454, A::scale(s.ad_value(162), 0.2), 159);

        s.store_div_ad_rhs(291, 153, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(149), 1e-50)), s.ad_value(386)));

        s.store_sqrt_ad(160, A::add(A::square(s.ad_value(291)), A::square(s.ad_value(454))));

        s.store_mul(161, 159, 160);

        s.store_div(279, 161, 162);

        s.v[894] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[894] != 0.0) {
            s.store_scalar(281, 1.0);
        }

        s.v[895] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[894] != 0.0)) && (s.v[895] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if ((!(s.v[894] != 0.0)) && (!(s.v[895] != 0.0))) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_ad(282, A::mul(s.ad_value(279), s.ad_value(281)), 1.0);

        s.v[896] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[896] != 0.0) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[897] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[896] != 0.0)) && (s.v[897] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if ((!(s.v[896] != 0.0)) && (!(s.v[897] != 0.0))) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
        }

        if ((!(s.v[896] != 0.0)) && (!(s.v[897] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        s.store_mul(158, 159, 283);

        s.store_div_ad_lhs(455, A::scale(s.ad_value(162), 0.2), 166);

        s.store_div_ad_rhs(291, 154, A::mul(A::mul(s.ad_value(120), A::offset(s.ad_value(150), 1e-50)), s.ad_value(386)));

        s.store_sqrt_ad(164, A::add(A::square(s.ad_value(291)), A::square(s.ad_value(455))));

        s.store_mul(161, 166, 164);

        s.store_div(279, 161, 162);

        s.v[898] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[898] != 0.0) {
            s.store_scalar(281, 1.0);
        }

        s.v[899] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[898] != 0.0)) && (s.v[899] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if ((!(s.v[898] != 0.0)) && (!(s.v[899] != 0.0))) {
            s.store_powf(281, 279, (p.p114 - 1.0));
        }

        s.store_offset_ad(282, A::mul(s.ad_value(279), s.ad_value(281)), 1.0);

        s.v[900] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (s.v[900] != 0.0) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[901] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((!(s.v[900] != 0.0)) && (s.v[901] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if ((!(s.v[900] != 0.0)) && (!(s.v[901] != 0.0))) {
            s.store_powf(284, 282, (((-1.0) / p.p114) - 1.0));
        }

        if ((!(s.v[900] != 0.0)) && (!(s.v[901] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        s.store_mul(165, 166, 283);

        s.store_div_ad(189, A::scale(s.ad_value(122), s.v[466]), A::sub(s.ad_value(123), s.ad_value(262)));

        s.store_mul_ad_lhs(96, A::mul(s.ad_value(189), s.ad_value(153)), 158);

        s.store_mul_ad_lhs(97, A::mul(s.ad_value(189), s.ad_value(154)), 165);

        s.store_add(95, 96, 97);

        s.v[173] = 0.0;

        s.v[169] = 0.0;

        s.v[171] = 0.0;

        s.v[172] = 0.0;

        s.v[902] = if (p.p239 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[902] != 0.0) {
            s.store_scaled_sub(279, 51, 59, 0.5);
        }

        if (s.v[902] != 0.0) {
            s.store_scale(638, 279, (2.0 * 100.0));
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (s.v[902] != 0.0) {
            s.store_div_from_scalar(284, 0.01, 639);
        }

        if (s.v[902] != 0.0) {
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
        }

        if (s.v[902] != 0.0) {
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(56), s.ad_value(284)));
        }

        if (s.v[902] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.05) * 0.05)));
        }

        if (s.v[902] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[902] != 0.0) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.05));
        }

        s.v[903] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[902] != 0.0) && (s.v[903] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[902] != 0.0) && (s.v[903] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[902] != 0.0) {
            s.store_mul_ad(287, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[475]), A::powf(s.ad_value(280), p.p240));
        }

        if (s.v[902] != 0.0) {
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul(A::scale(s.ad_value(71), s.v[476]), A::sub(A::add(s.ad_value(56), s.ad_value(284)), s.ad_value(70))));
        }

        if (s.v[902] != 0.0) {
            s.store_mul(287, 287, 282);
        }

        if (!(s.v[902] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.v[904] = if (p.p246 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[904] != 0.0) {
            s.store_mul_ad_lhs(286, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[477]), 71);
        }

        if (!(s.v[904] != 0.0)) {
            s.store_scalar(286, 0.0);
        }

        s.v[905] = if ((s.v[287] + s.v[286]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[905] != 0.0) {
            s.store_mul_ad_rhs(152, 59, A::add(s.ad_value(287), s.ad_value(286)));
        }

        if (s.v[905] != 0.0) {
            s.store_mul_ad_lhs(173, A::mul(s.ad_value(189), s.ad_value(152)), 158);
        }

        if (s.v[905] != 0.0) {
            s.store_div_from_scalar_ad(172, 1.0, A::offset(A::exp(A::scale(s.ad_value(440), (-p.p245))), 1.0));
        }

        if (s.v[905] != 0.0) {
            s.store_sub_from_scalar(171, 1.0, 172);
        }

        if (s.v[905] != 0.0) {
            s.store_mul(169, 171, 173);
        }

        s.v[174] = 0.0;

        s.v[170] = 0.0;

        s.v[906] = if (p.p239 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[906] != 0.0) {
            s.store_scaled_sub(279, 51, 155, 0.5);
        }

        if (s.v[906] != 0.0) {
            s.store_scale(638, 279, (2.0 * 100.0));
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (s.v[906] != 0.0) {
            s.store_div_from_scalar(284, 0.01, 639);
        }

        if (s.v[906] != 0.0) {
            s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));
        }

        if (s.v[906] != 0.0) {
            s.store_sub_from_scalar_ad(279, 1.1, A::add(s.ad_value(322), s.ad_value(284)));
        }

        if (s.v[906] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.05) * 0.05)));
        }

        if (s.v[906] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[906] != 0.0) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.05));
        }

        s.v[907] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[906] != 0.0) && (s.v[907] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if ((s.v[906] != 0.0) && (s.v[907] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[906] != 0.0) {
            s.store_mul_ad(287, A::scale(A::mul(s.ad_value(270), s.ad_value(120)), s.v[475]), A::powf(s.ad_value(280), p.p240));
        }

        if (s.v[906] != 0.0) {
            s.store_add_ad(282, A::offset(A::scale(s.ad_value(71), p.p241), 1.0), A::mul(A::scale(s.ad_value(71), s.v[476]), A::sub(A::add(s.ad_value(322), s.ad_value(284)), s.ad_value(70))));
        }

        if (s.v[906] != 0.0) {
            s.store_mul(287, 287, 282);
        }

        if (!(s.v[906] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.v[908] = if ((s.v[287] + s.v[286]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[908] != 0.0) {
            s.store_mul_ad_rhs(152, 155, A::add(s.ad_value(287), s.ad_value(286)));
        }

        if (s.v[908] != 0.0) {
            s.store_mul_ad_lhs(174, A::mul(s.ad_value(189), s.ad_value(152)), 165);
        }

        s.v[909] = if ((s.v[174] > (s.v[173] - (s.v[173] * 0.05))) && ((s.v[173] * 0.05) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add_ad(638, A::sub(s.ad_value(174), s.ad_value(173)), A::scale(s.ad_value(173), 0.05));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_square(642, 638);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul_ad(643, A::scale(s.ad_value(173), 0.05), A::scale(s.ad_value(173), 0.05));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[910] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[911] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (s.v[911] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[912] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (s.v[912] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[913] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[913] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[914] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (!(s.v[911] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) && (s.v[914] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign14450_loop_guard: usize = 0;
        while {
            let assign14450_cond_e18791: f64 = if ((((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign14450_cond_e18791 != 0.0
        } {
            assign14450_loop_guard += 1;
            assert!(assign14450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (s.v[910] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((s.v[908] != 0.0) && (s.v[909] != 0.0)) && (!(s.v[910] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_mul_ad_lhs(637, A::mul(s.ad_value(638), A::scale(s.ad_value(173), 0.05)), 646);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_div_ad(278, A::mul(A::mul(A::scale(s.ad_value(173), 0.05), s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
            s.store_add_ad_lhs(174, A::sub(s.ad_value(173), A::scale(s.ad_value(173), 0.05)), 637);
        }

        if ((s.v[908] != 0.0) && (s.v[909] != 0.0)) {
        }

        if ((s.v[908] != 0.0) && (!(s.v[909] != 0.0))) {
        }

        if ((s.v[908] != 0.0) && (!(s.v[909] != 0.0))) {
            s.store_scalar(278, 1.0);
        }

        if (s.v[908] != 0.0) {
            s.store_mul(170, 172, 174);
        }

        s.store_add(175, 169, 170);

        s.store_add(94, 95, 175);

        s.v[915] = if (p.p22 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[915] != 0.0) {
            s.store_scale(279, 271, 1.034943e-10);
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(280, 132);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(281, (s.v[133] - p.p57));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(282, 1.0, A::square(s.ad_value(281)));
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(283, A::mul(A::mul(A::scale(A::sub_from_scalar(p.p55, s.ad_value(130)), 2.0), s.ad_value(279)), s.ad_value(280)), 282);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(81, 283, 135);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(282, p.p158);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(284, p.p159);
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_rhs(279, 282, A::mul(s.ad_value(284), s.ad_value(71)));
        }

        if (s.v[915] != 0.0) {
            s.store_mul(98, 81, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_sub_from_scalar_ad(279, p.p160, A::scale(s.ad_value(51), p.p161));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_lhs(99, A::add(A::sub(s.ad_value(72), s.ad_value(138)), s.ad_value(279)), 98);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(102, A::mul(s.ad_value(119), s.ad_value(271)), 271);
        }

        if (s.v[915] != 0.0) {
            s.store_scaled_mul(103, 102, 120, 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_scaled_mul(104, 103, 120, 2.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scale(387, 120, 0.25);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(288, A::sub(A::offset(A::add(A::sub(s.ad_value(122), A::mul(s.ad_value(102), s.ad_value(387))), s.ad_value(138)), (-p.p160)), s.ad_value(98)), 1e-50);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::sub(s.ad_value(72), s.ad_value(288)), (-0.005));
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(288)), 0.005)));
        }

        if (s.v[915] != 0.0) {
            s.store_sub_ad_lhs(281, A::add(A::offset(A::sub(A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), s.ad_value(138)), p.p160), s.ad_value(98)), 70);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(282, A::mul(s.ad_value(120), s.ad_value(281)), (-1.0));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar(283, 4.0, 104);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[916] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[916] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[916] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, 1e-50);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(105, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(278, 103, A::sub_from_scalar(1.0, s.ad_value(105)));
        }

        if (s.v[915] != 0.0) {
            s.store_add(107, 99, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(278, 1.0, A::add(s.ad_value(120), A::div_from_scalar(2.0, A::offset(s.ad_value(99), 1e-50))));
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_lhs(109, A::ln(A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(101)), s.ad_value(102)), A::square(s.ad_value(99)))), 278);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad_rhs(281, 109, A::offset(s.ad_value(99), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(110, A::sub(s.ad_value(109), s.ad_value(107)), (-p.p136));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad(278, A::square(s.ad_value(110)), A::scale(s.ad_value(109), (4.0 * p.p136)));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(278)), ((4.0 * 1e-6) * 1e-6)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(278), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(278, A::scale(A::add(s.ad_value(278), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
        }

        s.v[917] = if (s.v[278] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[917] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[917] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(278, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), s.ad_value(278)), 0.5));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar(279, 1.0, 278);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(278, 101, A::exp(A::mul(s.ad_value(120), s.ad_value(111))));
        }

        if (s.v[915] != 0.0) {
            s.store_add_ad_lhs(279, A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0)), 278);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[918] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[918] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[918] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(113, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(120), A::sub(s.ad_value(111), s.ad_value(70))), (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[919] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[919] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[919] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt(114, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_mul_ad_rhs(115, 100, A::sub(s.ad_value(113), s.ad_value(114)));
        }

        if (s.v[915] != 0.0) {
            s.store_sub(279, 107, 111);
        }

        if (s.v[915] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[915] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[915] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[920] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[915] != 0.0) && (s.v[920] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[915] != 0.0) && (s.v[920] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_offset(279, 279, (10.0 * 2.220446049250313e-16));
        }

        if (s.v[915] != 0.0) {
            s.store_div(290, 51, 279);
        }

        if (s.v[915] != 0.0) {
            s.store_square(642, 290);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(643, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[915] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[915] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[921] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[922] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (s.v[922] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[923] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[924] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) && (s.v[924] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[925] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) && (!(s.v[924] != 0.0))) && (s.v[925] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign15630_loop_guard: usize = 0;
        while {
            let assign15630_cond_e19733: f64 = if (((s.v[915] != 0.0) && (s.v[921] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign15630_cond_e19733 != 0.0
        } {
            assign15630_loop_guard += 1;
            assert!(assign15630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[915] != 0.0) && (s.v[921] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[915] != 0.0) && (!(s.v[921] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if (s.v[915] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_mul(291, 290, 646);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad(280, A::mul(s.ad_value(645), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[915] != 0.0) {
            s.store_scale(106, 122, ((2.0 * s.v[453]) * p.p5));
        }

        if (s.v[915] != 0.0) {
            s.copy_ad(279, 386);
        }

        if (s.v[915] != 0.0) {
            s.store_div_ad_lhs(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(291)), 279);
        }

        if (s.v[915] != 0.0) {
            s.store_add(94, 94, 116);
        }

        s.v[926] = if ((p.p20 != 0.0) && (p.p23 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[926] != 0.0) {
            s.store_square(231, 86);
        }

        if (s.v[926] != 0.0) {
            s.store_mul_ad_lhs(232, A::mul(A::scale(s.ad_value(122), 2.0), s.ad_value(271)), 151);
        }

        if (s.v[926] != 0.0) {
            s.store_sub(233, 231, 232);
        }

        if (s.v[926] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(231)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[926] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(231), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[926] != 0.0) {
            s.store_offset_ad(231, A::scale(A::add(s.ad_value(231), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[927] = if (s.v[231] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if ((s.v[926] != 0.0) && (s.v[927] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[926] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(233)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[926] != 0.0) {
            s.store_scale_ad(278, A::offset(A::div(s.ad_value(233), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[926] != 0.0) {
            s.store_offset_ad(233, A::scale(A::add(s.ad_value(233), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[928] = if (s.v[233] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[928] != 0.0)) {
            s.store_scalar(233, 0.0);
        }

        if ((s.v[926] != 0.0) && (s.v[928] != 0.0)) {
            s.store_scalar(278, 0.0);
        }

        if (s.v[926] != 0.0) {
            s.store_sub(234, 231, 233);
        }

        s.v[929] = if ((s.v[149] < (10.0 * 2.220446049250313e-16)) || (s.v[234] < (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[926] != 0.0) && (s.v[929] != 0.0)) {
            s.store_scalar(35, 0.0);
        }

        if ((s.v[926] != 0.0) && (!(s.v[929] != 0.0))) {
            s.store_scalar(35, 1.0);
        }

        s.v[930] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[930] != 0.0) {
            s.copy_ad(279, 388);
        }

        if (s.v[930] != 0.0) {
            s.store_square(285, 270);
        }

        if (s.v[930] != 0.0) {
            s.store_mul_ad_lhs(282, A::div_from_scalar(2.0, s.ad_value(472)), 285);
        }

        if (s.v[930] != 0.0) {
            s.store_sub_ad(283, A::sub(s.ad_value(279), s.ad_value(122)), A::scale(s.ad_value(70), s.v[486]));
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(284, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (s.v[930] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[930] != 0.0) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[931] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[930] != 0.0) && (s.v[931] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((s.v[930] != 0.0) && (s.v[931] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[930] != 0.0) {
            s.store_offset(284, 284, 1e-50);
        }

        if (s.v[930] != 0.0) {
            s.store_add_ad(186, A::scale(s.ad_value(279), s.v[491]), A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
        }

        if (s.v[930] != 0.0) {
            s.store_sub_ad(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), A::scale(s.ad_value(186), (s.v[487] * s.v[485])));
        }

        if (s.v[930] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(187)), ((4.0 * 0.01) * 0.01)));
        }

        if (s.v[930] != 0.0) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(187), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[930] != 0.0) {
            s.store_offset_ad(187, A::scale(A::add(s.ad_value(187), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[932] = if (s.v[187] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[930] != 0.0) && (s.v[932] != 0.0)) {
            s.store_scalar(187, 0.0);
        }

        if ((s.v[930] != 0.0) && (s.v[932] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[930] != 0.0) {
            s.store_offset(187, 187, 1e-50);
        }

        if (s.v[930] != 0.0) {
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
        }

        if (s.v[930] != 0.0) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(187), s.v[488]), s.ad_value(94)), 280);
        }

        s.v[933] = if (((s.v[34] == 0.0) && (s.v[185] > 0.0)) && (p.p145 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[933] != 0.0) {
            s.store_offset_scaled(278, 80, p.p146, 1.0);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_lhs(188, A::scale(s.ad_value(278), p.p145), 185);
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(64, A::mul(s.ad_value(120), s.ad_value(56)), (-1.0));
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(64)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(64, A::scale(A::add(s.ad_value(64), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[934] = if (s.v[64] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[933] != 0.0) && (s.v[934] != 0.0)) {
            s.store_scalar(64, 0.0);
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt(65, 64);
        }

        if (s.v[933] != 0.0) {
            s.store_mul(66, 64, 65);
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(69, A::mul(s.ad_value(120), s.ad_value(57)), (-1.0));
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(69)), ((4.0 * 0.1) * 0.1)));
        }

        if (s.v[933] != 0.0) {
            s.store_offset_ad(69, A::scale(A::add(s.ad_value(69), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[935] = if (s.v[69] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[933] != 0.0) && (s.v[935] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (s.v[933] != 0.0) {
            s.store_sqrt(67, 69);
        }

        if (s.v[933] != 0.0) {
            s.store_mul(68, 69, 67);
        }

        if (s.v[933] != 0.0) {
            s.store_div_ad_lhs(279, A::mul(s.ad_value(120), s.ad_value(188)), 64);
        }

        if (s.v[933] != 0.0) {
            s.store_div_ad_lhs(280, A::mul(s.ad_value(120), s.ad_value(188)), 69);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_rhs(190, 141, A::sub(A::mul(s.ad_value(68), s.ad_value(280)), A::mul(s.ad_value(66), s.ad_value(279))));
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad(191, A::scale(s.ad_value(141), 0.5), A::add(A::mul(A::neg(s.ad_value(67)), s.ad_value(280)), A::mul(s.ad_value(65), s.ad_value(279))));
        }

        if (s.v[933] != 0.0) {
            s.store_add(192, 190, 191);
        }

        if (s.v[933] != 0.0) {
            s.store_mul_ad_lhs(193, A::mul(s.ad_value(189), s.ad_value(192)), 158);
        }

        s.v[949] = (s.v[272] * 100.0);

        s.store_scale(951, 123, 100.0);

        s.v[952] = (s.v[466] * 100.0);

        s.store_scale(953, 160, 0.01);

        s.v[956] = if (p.p17 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[956] != 0.0) {
            s.store_scalar(256, 0.0);
        }

        s.v[957] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(948, A::add(s.ad_value(74), s.ad_value(71)), (-(10.0 * 2.220446049250313e-16)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sub_ad(938, A::add(A::sub(s.ad_value(72), A::scale(s.ad_value(138), p.p256)), A::div(A::add(A::scale(s.ad_value(50), (-p.p258)), A::scale(A::sub(s.ad_value(80), s.ad_value(267)), p.p206)), s.ad_value(951))), A::scale(s.ad_value(948), p.p205));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_scaled(944, 953, 1.0 / (p.p207), 1.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scaled_mul(947, 944, 938, 1.0 / (s.v[949]));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[958] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[958] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[958] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(72)), ((4.0 * 0.001) * 0.001)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale_ad(941, A::offset(A::div(s.ad_value(72), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(72), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[959] = if (s.v[940] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[959] != 0.0)) {
            s.store_scalar(940, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (s.v[959] != 0.0)) {
            s.store_scalar(941, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scaled_offset(936, 940, (-p.p216), 10.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_sub_from_scalar_ad(938, 1.0, A::div_from_scalar(1.0, A::offset(A::square(s.ad_value(936)), 1.0)));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_mul(947, 947, 938);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_scale(937, 951, s.v[952]);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_div_from_scalar_ad(944, p.p209, A::offset(s.ad_value(937), p.p209));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_div_from_scalar_ad(941, 1.0, A::offset(A::square(s.ad_value(947)), 1e-50));
        }

        if ((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) {
            s.store_mul_ad_lhs(938, A::scale(s.ad_value(246), (-p.p204)), 941);
        }

        s.v[960] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (s.v[957] != 0.0)) && (!(s.v[960] != 0.0))) {
            s.store_mul_ad_lhs(940, A::scale(A::div_from_scalar(p.p203, s.ad_value(245)), 1.6021918e-19), 937);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_scaled(937, 52, (-p.p211), p.p212);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_exp_ad(939, A::scale(s.ad_value(937), s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale(938, 52, p.p260);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(940, A::square(s.ad_value(938)), 937);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_sub(942, 52, 51);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_scaled(937, 942, (-p.p211), p.p212);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_exp_ad(939, A::scale(s.ad_value(937), s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale(938, 942, p.p260);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(937, ((1.0 / s.v[949]) / s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_mul_ad_lhs(940, A::square(s.ad_value(938)), 937);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(941, (((p.p210 / 1000000.0) * s.v[952]) * ((s.v[375]) as f64).powf(p.p259)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale_ad(947, A::offset(A::add(A::sub(A::scale(s.ad_value(50), p.p261), s.ad_value(52)), s.ad_value(138)), p.p215), 1.0 / (s.v[949]));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[963] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (s.v[963] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (s.v[963] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_offset(947, 947, 1e-50);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_div_from_scalar_ad(938, (-p.p214), A::powf(s.ad_value(947), p.p263));
        }

        s.v[964] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_exp(939, 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scalar(940, (s.v[375] + p.p264));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad(638, A::offset(s.ad_value(940), (-p.p265)), A::scale(s.ad_value(940), 0.001));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale(639, 940, (0.001 * (4.0 * p.p265)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(937, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p265);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(940, A::scale(s.ad_value(940), (p.p213 * 1e-6)), s.v[952]);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_mul_ad_lhs(252, A::mul(s.ad_value(940), A::powf(s.ad_value(947), p.p262)), 939);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(947, A::offset(A::add(A::sub(A::scale(s.ad_value(50), p.p269), s.ad_value(52)), s.ad_value(138)), p.p268), 1.0 / (s.v[949]));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(947)), ((4.0 * 0.01) * 0.01)));
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(942, A::offset(A::div(s.ad_value(947), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset_ad(947, A::scale(A::add(s.ad_value(947), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[965] = if (s.v[947] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[965] != 0.0)) {
            s.store_scalar(947, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[965] != 0.0)) {
            s.store_scalar(942, 0.0);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_offset(947, 947, 1e-50);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_div_from_scalar_ad(938, (-p.p267), A::powf(s.ad_value(947), p.p271));
        }

        s.v[966] = if (s.v[938] < (-34.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[966] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_exp(939, 938);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scalar(940, (s.v[375] + p.p272));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_sub_ad(638, A::offset(s.ad_value(940), (-p.p273)), A::scale(s.ad_value(940), 0.001));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale(639, 940, (0.001 * (4.0 * p.p273)));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale_ad(937, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_offset_ad(940, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p273);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_scale_ad(940, A::scale(s.ad_value(940), (p.p266 * 1e-6)), s.v[952]);
        }

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (!(s.v[966] != 0.0))) {
            s.store_mul_ad_lhs(253, A::mul(s.ad_value(940), A::powf(s.ad_value(947), p.p270)), 939);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_scale_ad(938, A::neg(s.ad_value(252)), 0.001);
        }

        s.v[967] = if (s.v[938] < 1e-50) { 1.0 } else { 0.0 };

        if (((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) && (s.v[967] != 0.0)) {
            s.store_scalar(938, 1e-50);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sub_ad_lhs(638, A::sub(A::neg(s.ad_value(252)), A::neg(s.ad_value(253))), 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_mul_ad_lhs(639, A::scale(A::neg(s.ad_value(253)), 4.0), 938);
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[956] != 0.0)) && (!(s.v[964] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(256, 0.5);
        }

        s.v[968] = if (p.p18 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[968] != 0.0)) {
            s.store_sub_ad(279, A::sub(A::scale(A::offset(s.ad_value(51), p.p199), p.p198), s.ad_value(52)), A::scale(A::add(s.ad_value(82), s.ad_value(266)), p.p200));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_scale(247, 279, 1.0 / (p.p228));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(247)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[968] != 0.0)) {
            s.store_scale_ad(283, A::offset(A::div(s.ad_value(247), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[968] != 0.0)) {
            s.store_offset_ad(248, A::scale(A::add(s.ad_value(247), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[969] = if (s.v[248] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[968] != 0.0)) && (s.v[969] != 0.0)) {
            s.store_scalar(248, 0.0);
        }

        if ((!(s.v[968] != 0.0)) && (s.v[969] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        if (!(s.v[968] != 0.0)) {
            s.store_div_ad(278, A::scale(s.ad_value(246), (-s.v[627])), A::offset(s.ad_value(248), 1e-50));
        }

        s.v[970] = if (s.v[278] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[968] != 0.0)) && (!(s.v[970] != 0.0))) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.v[971] = if (p.p18 == 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[971] != 0.0)) {
            s.store_sub_ad(279, A::sub(A::scale(A::sub_from_scalar(p.p199, s.ad_value(51)), p.p198), A::sub(s.ad_value(52), s.ad_value(51))), A::scale(A::add(s.ad_value(82), s.ad_value(266)), p.p200));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_scale(247, 279, 1.0 / (p.p228));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(247)), ((4.0 * 0.01) * 0.01)));
        }

        if (!(s.v[971] != 0.0)) {
            s.store_scale_ad(283, A::offset(A::div(s.ad_value(247), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[971] != 0.0)) {
            s.store_offset_ad(249, A::scale(A::add(s.ad_value(247), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[972] = if (s.v[249] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[971] != 0.0)) && (s.v[972] != 0.0)) {
            s.store_scalar(249, 0.0);
        }

        if ((!(s.v[971] != 0.0)) && (s.v[972] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        if (!(s.v[971] != 0.0)) {
            s.store_div_ad(278, A::scale(s.ad_value(246), (-s.v[627])), A::offset(s.ad_value(249), 1e-50));
        }

        s.v[973] = if (s.v[278] < (-34.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[971] != 0.0)) && (!(s.v[973] != 0.0))) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        s.v[264] = p.p176;

        s.v[261] = 0.0;

        s.v[974] = if (s.v[34] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[974] != 0.0) {
            s.store_add(280, 51, 56);
        }

        if (s.v[974] != 0.0) {
            s.store_add_ad(260, A::scale(s.ad_value(280), s.v[264]), A::scale(s.ad_value(57), (1.0 - s.v[264])));
        }

        s.v[975] = if (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[974] != 0.0) && (s.v[975] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(51)), (-(10.0 * 2.220446049250313e-16)));
        }

        s.v[976] = if (p.p45 != 0.0) { 1.0 } else { 0.0 };

        s.v[977] = if (s.v[151] > 1e-15) { 1.0 } else { 0.0 };

        if (((!(s.v[974] != 0.0)) && (s.v[976] != 0.0)) && (s.v[977] != 0.0)) {
            s.store_div_ad_lhs(261, A::div(A::mul(s.ad_value(151), s.ad_value(122)), s.ad_value(123)), 149);
        }

        s.v[435] = s.v[273];

        s.v[436] = (1.0 / s.v[435]);

        s.v[978] = if (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[978] != 0.0) {
            s.store_scalar(195, p.p175);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad_rhs(437, 141, A::sqrt(A::div_from_scalar(s.v[624], s.ad_value(457))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(399, ((1.0 - -1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(400, ((1.0 + -1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(402, A::mul(s.ad_value(399), s.ad_value(412)), A::mul(s.ad_value(400), s.ad_value(413)));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(403, A::mul(s.ad_value(399), s.ad_value(413)), A::mul(s.ad_value(400), s.ad_value(412)));
        }

        if ((s.v[978] != 0.0) && (s.v[399] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(412), s.ad_value(42)), A::mul(s.ad_value(413), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if ((s.v[978] != 0.0) && (s.v[400] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(413), s.ad_value(42)), A::mul(s.ad_value(412), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(415, 0.0);
        }

        if (s.v[978] != 0.0) {
            s.store_neg(278, 415);
        }

        s.v[979] = if (s.v[278] > s.v[31]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_sub(279, 278, 31);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_sub_from_scalar(280, s.v[30], 31);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_div(638, 279, 280);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_square(639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul(640, 639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_square(641, 639);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul_ad_lhs(387, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(291)), 291);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul_ad_rhs(291, 280, A::sub_from_scalar(1.0, s.ad_value(291)));
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_neg(387, 387);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_add(288, 31, 291);
        }

        if ((s.v[978] != 0.0) && (!(s.v[979] != 0.0))) {
            s.copy_ad(288, 278);
        }

        if (s.v[978] != 0.0) {
            s.store_offset_ad(416, A::neg(s.ad_value(288)), (-1e-12));
        }

        if (s.v[978] != 0.0) {
            s.store_scale(144, 437, s.v[436]);
        }

        if (s.v[978] != 0.0) {
            s.store_square(145, 144);
        }

        if (s.v[978] != 0.0) {
            s.store_sub_from_scalar(404, p.p39, 414);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        if (s.v[978] != 0.0) {
            s.store_neg(419, 416);
        }

        s.v[980] = if (s.v[404] < s.v[419]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div_from_scalar_ad(291, s.v[435], A::mul(s.ad_value(120), s.ad_value(437)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_mul_ad_lhs(182, A::mul(A::scale(s.ad_value(184), 8.0), s.ad_value(184)), 184);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub(176, 137, 417);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_mul_ad_rhs(290, 120, A::add(s.ad_value(404), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_square(181, 183);
        }

        s.v[981] = if (s.v[182] < (s.v[181] * 1e-8)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (s.v[981] != 0.0)) {
            s.store_add_ad(179, A::add(A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(182), 0.5), s.ad_value(183))), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (!(s.v[981] != 0.0))) {
            s.store_sqrt_ad(180, A::add(s.ad_value(182), s.ad_value(181)));
        }

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (!(s.v[981] != 0.0))) {
            s.store_add_ad(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_powf(178, 179, 0.3333333333333333);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_add_ad(177, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), A::scale(s.ad_value(178), 2.0)), A::mul(A::scale(s.ad_value(178), 1.414213562373095), s.ad_value(178)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div(77, 177, 178);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_ad_lhs(259, A::mul(s.ad_value(77), s.ad_value(122)), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_add(279, 259, 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div(280, 279, 176);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.copy_ad(407, 408);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_scalar(77, 3.0);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[982] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[982] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[983] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[983] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[984] = if (s.v[77] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_offset_ad(423, A::div_from_scalar(1.0, A::mul(s.ad_value(120), s.ad_value(144))), (1.0 / 1.414213562373095));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_div_ad_lhs(425, A::neg(A::add(s.ad_value(404), s.ad_value(416))), 144);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_add_ad(426, A::sub(A::div(A::mul(A::square(s.ad_value(422)), s.ad_value(422)), A::mul(A::mul(A::scale(s.ad_value(421), 27.0), s.ad_value(421)), s.ad_value(421))), A::div(A::mul(s.ad_value(422), s.ad_value(423)), A::mul(A::scale(s.ad_value(421), 6.0), s.ad_value(421)))), A::div(s.ad_value(425), A::scale(s.ad_value(421), 2.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_div_ad(424, A::sub(A::mul(A::scale(s.ad_value(421), 3.0), s.ad_value(423)), A::square(s.ad_value(422))), A::mul(A::scale(s.ad_value(421), 9.0), s.ad_value(421)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sqrt_ad(283, A::add(A::square(s.ad_value(426)), A::mul(A::square(s.ad_value(424)), s.ad_value(424))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sub_ad(290, A::add(s.ad_value(427), s.ad_value(428)), A::div(s.ad_value(422), A::scale(s.ad_value(421), 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sub_ad_lhs(319, A::mul(s.ad_value(290), s.ad_value(122)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[985] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(420, A::add(s.ad_value(404), s.ad_value(416)), 0.1);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(203, A::exp(A::mul(s.ad_value(120), A::neg(s.ad_value(416)))), 1e-50);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(430, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(278, 121, 145);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(434, 120, 420);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad(433, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(434), s.ad_value(433)), (-1.0));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(639, 434, 4.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), 2.0), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_rhs(433, 434, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub(434, 434, 433);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad_rhs(434, 434, A::scale(s.ad_value(120), 0.1));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad(432, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.copy_ad(431, 77);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(432), s.ad_value(431)), (-(0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), ((2.0 * 0.0008) * 75.0)), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_rhs(77, 432, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp(A::neg(s.ad_value(77))));
        }

        s.v[986] = if (s.v[279] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[986] != 0.0)) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(407, 437, A::sqrt(s.ad_value(279)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.v[987] = if (p.p30 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_exp_ad(203, A::mul(s.ad_value(120), A::neg(s.ad_value(416))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul(204, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scalar(379, 0.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

        let mut assign19230_loop_guard: usize = 0;
        while {
            let assign19230_cond_e23449: f64 = (40.0 + 1.0);
            let assign19230_cond_e23451: f64 = if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[62] <= assign19230_cond_e23449)) { 1.0 } else { 0.0 };
            assign19230_cond_e23451 != 0.0
        } {
            assign19230_loop_guard += 1;
            assert!(assign19230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(410), s.ad_value(416)));
            }
            s.v[988] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad(205, A::mul(A::square(s.ad_value(77)), s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad(206, A::square(s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_lhs(207, A::mul(s.ad_value(204), s.ad_value(205)), 205);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(A::scale(A::mul(s.ad_value(204), s.ad_value(120)), 2.0), s.ad_value(205)), 206);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_rhs(146, 77, A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_sqrt_ad(209, A::offset(A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_div_ad(210, A::add(A::mul(A::scale(A::mul(s.ad_value(120), s.ad_value(148)), 2.0), s.ad_value(146)), s.ad_value(208)), A::scale(s.ad_value(209), 2.0));
            }
            s.v[989] = if (s.v[77] < 80.0) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_exp(147, 77);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_mul_ad_rhs(207, 204, A::offset(s.ad_value(147), (-1.0)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(204), s.ad_value(120)), 147);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_exp_ad(202, A::mul(s.ad_value(120), s.ad_value(410)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_mul_ad_rhs(207, 429, A::sub(s.ad_value(202), s.ad_value(203)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(429), s.ad_value(120)), 202);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) {
                s.store_sqrt_ad(209, A::add(A::offset(s.ad_value(77), (-1.0)), s.ad_value(207)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) {
                s.store_scale_ad(210, A::div(A::add(s.ad_value(120), s.ad_value(208)), s.ad_value(209)), 0.5);
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_sub_ad(211, A::sub(s.ad_value(404), s.ad_value(410)), A::mul(s.ad_value(144), s.ad_value(209)));
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.v[990] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[990] != 0.0)) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_div_ad_lhs(213, A::neg(s.ad_value(211)), 212);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_scale_ad(214, A::offset({
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[991] = if (((s.v[213]) as f64).abs() > s.v[214]) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) && (s.v[991] != 0.0)) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_add(410, 410, 213);
            }
            s.v[992] = if ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) && (s.v[992] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[994] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[994] != 0.0)) {
            s.store_offset_ad(64, A::square(s.ad_value(146)), (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[994] != 0.0)) {
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[994] != 0.0))) {
            s.store_offset(64, 77, (-1.0));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[994] != 0.0))) {
            s.store_sqrt(65, 64);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul(407, 437, 65);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_div_from_scalar_ad(279, 1.0, A::add(s.ad_value(209), s.ad_value(65)));
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul_ad_lhs(409, A::mul(s.ad_value(437), s.ad_value(207)), 279);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_add(408, 407, 409);
        }

        if (s.v[978] != 0.0) {
            s.store_sub(409, 408, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scale(282, 195, s.v[513]);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(406, 282, 407);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(405, 282, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(399, ((1.0 - 1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(400, ((1.0 + 1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(402, A::mul(s.ad_value(399), s.ad_value(412)), A::mul(s.ad_value(400), s.ad_value(413)));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(403, A::mul(s.ad_value(399), s.ad_value(413)), A::mul(s.ad_value(400), s.ad_value(412)));
        }

        if ((s.v[978] != 0.0) && (s.v[399] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(412), s.ad_value(42)), A::mul(s.ad_value(413), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if ((s.v[978] != 0.0) && (s.v[400] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(413), s.ad_value(42)), A::mul(s.ad_value(412), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(415, 0.0);
        }

        if (s.v[978] != 0.0) {
            s.store_neg(278, 415);
        }

        s.v[996] = if (s.v[278] > s.v[31]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_sub(279, 278, 31);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_sub_from_scalar(280, s.v[30], 31);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_div(638, 279, 280);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_square(639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul(640, 639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_square(641, 639);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul_ad_lhs(387, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(291)), 291);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul_ad_rhs(291, 280, A::sub_from_scalar(1.0, s.ad_value(291)));
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_neg(387, 387);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_add(288, 31, 291);
        }

        if ((s.v[978] != 0.0) && (!(s.v[996] != 0.0))) {
            s.copy_ad(288, 278);
        }

        if (s.v[978] != 0.0) {
            s.store_offset_ad(416, A::neg(s.ad_value(288)), (-1e-12));
        }

        if (s.v[978] != 0.0) {
            s.store_scale(144, 437, s.v[436]);
        }

        if (s.v[978] != 0.0) {
            s.store_square(145, 144);
        }

        if (s.v[978] != 0.0) {
            s.store_sub_from_scalar(404, p.p39, 414);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        if (s.v[978] != 0.0) {
            s.store_neg(419, 416);
        }

        s.v[997] = if (s.v[404] < s.v[419]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div_from_scalar_ad(291, s.v[435], A::mul(s.ad_value(120), s.ad_value(437)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_mul_ad_lhs(182, A::mul(A::scale(s.ad_value(184), 8.0), s.ad_value(184)), 184);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub(176, 137, 417);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_mul_ad_rhs(290, 120, A::add(s.ad_value(404), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_square(181, 183);
        }

        s.v[998] = if (s.v[182] < (s.v[181] * 1e-8)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (s.v[998] != 0.0)) {
            s.store_add_ad(179, A::add(A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(182), 0.5), s.ad_value(183))), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (!(s.v[998] != 0.0))) {
            s.store_sqrt_ad(180, A::add(s.ad_value(182), s.ad_value(181)));
        }

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (!(s.v[998] != 0.0))) {
            s.store_add_ad(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_powf(178, 179, 0.3333333333333333);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_add_ad(177, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), A::scale(s.ad_value(178), 2.0)), A::mul(A::scale(s.ad_value(178), 1.414213562373095), s.ad_value(178)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div(77, 177, 178);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_ad_lhs(259, A::mul(s.ad_value(77), s.ad_value(122)), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_add(279, 259, 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div(280, 279, 176);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.copy_ad(407, 408);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_scalar(77, 3.0);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[999] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[999] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[1000] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1000] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[1001] = if (s.v[77] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_offset_ad(423, A::div_from_scalar(1.0, A::mul(s.ad_value(120), s.ad_value(144))), (1.0 / 1.414213562373095));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_div_ad_lhs(425, A::neg(A::add(s.ad_value(404), s.ad_value(416))), 144);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_add_ad(426, A::sub(A::div(A::mul(A::square(s.ad_value(422)), s.ad_value(422)), A::mul(A::mul(A::scale(s.ad_value(421), 27.0), s.ad_value(421)), s.ad_value(421))), A::div(A::mul(s.ad_value(422), s.ad_value(423)), A::mul(A::scale(s.ad_value(421), 6.0), s.ad_value(421)))), A::div(s.ad_value(425), A::scale(s.ad_value(421), 2.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_div_ad(424, A::sub(A::mul(A::scale(s.ad_value(421), 3.0), s.ad_value(423)), A::square(s.ad_value(422))), A::mul(A::scale(s.ad_value(421), 9.0), s.ad_value(421)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sqrt_ad(283, A::add(A::square(s.ad_value(426)), A::mul(A::square(s.ad_value(424)), s.ad_value(424))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sub_ad(290, A::add(s.ad_value(427), s.ad_value(428)), A::div(s.ad_value(422), A::scale(s.ad_value(421), 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sub_ad_lhs(319, A::mul(s.ad_value(290), s.ad_value(122)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[1002] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(420, A::add(s.ad_value(404), s.ad_value(416)), 0.1);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(203, A::exp(A::mul(s.ad_value(120), A::neg(s.ad_value(416)))), 1e-50);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(430, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(278, 121, 145);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(434, 120, 420);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad(433, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(434), s.ad_value(433)), (-1.0));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(639, 434, 4.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), 2.0), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_rhs(433, 434, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub(434, 434, 433);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad_rhs(434, 434, A::scale(s.ad_value(120), 0.1));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad(432, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.copy_ad(431, 77);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(432), s.ad_value(431)), (-(0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), ((2.0 * 0.0008) * 75.0)), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_rhs(77, 432, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp(A::neg(s.ad_value(77))));
        }

        s.v[1003] = if (s.v[279] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1003] != 0.0)) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(407, 437, A::sqrt(s.ad_value(279)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.v[1004] = if (p.p30 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_exp_ad(203, A::mul(s.ad_value(120), A::neg(s.ad_value(416))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul(204, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scalar(379, 0.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[62] <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(410), s.ad_value(416)));
            }
            s.v[1005] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad(205, A::mul(A::square(s.ad_value(77)), s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad(206, A::square(s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_lhs(207, A::mul(s.ad_value(204), s.ad_value(205)), 205);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(A::scale(A::mul(s.ad_value(204), s.ad_value(120)), 2.0), s.ad_value(205)), 206);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_rhs(146, 77, A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_sqrt_ad(209, A::offset(A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_div_ad(210, A::add(A::mul(A::scale(A::mul(s.ad_value(120), s.ad_value(148)), 2.0), s.ad_value(146)), s.ad_value(208)), A::scale(s.ad_value(209), 2.0));
            }
            s.v[1006] = if (s.v[77] < 80.0) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_exp(147, 77);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_mul_ad_rhs(207, 204, A::offset(s.ad_value(147), (-1.0)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(204), s.ad_value(120)), 147);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_exp_ad(202, A::mul(s.ad_value(120), s.ad_value(410)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_mul_ad_rhs(207, 429, A::sub(s.ad_value(202), s.ad_value(203)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(429), s.ad_value(120)), 202);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) {
                s.store_sqrt_ad(209, A::add(A::offset(s.ad_value(77), (-1.0)), s.ad_value(207)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) {
                s.store_scale_ad(210, A::div(A::add(s.ad_value(120), s.ad_value(208)), s.ad_value(209)), 0.5);
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_sub_ad(211, A::sub(s.ad_value(404), s.ad_value(410)), A::mul(s.ad_value(144), s.ad_value(209)));
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.v[1007] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1007] != 0.0)) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_div_ad_lhs(213, A::neg(s.ad_value(211)), 212);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_scale_ad(214, A::offset({
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1008] = if (((s.v[213]) as f64).abs() > s.v[214]) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) && (s.v[1008] != 0.0)) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_add(410, 410, 213);
            }
            s.v[1009] = if ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) && (s.v[1009] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[1011] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1011] != 0.0)) {
            s.store_offset_ad(64, A::square(s.ad_value(146)), (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1011] != 0.0)) {
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1011] != 0.0))) {
            s.store_offset(64, 77, (-1.0));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1011] != 0.0))) {
            s.store_sqrt(65, 64);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul(407, 437, 65);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_div_from_scalar_ad(279, 1.0, A::add(s.ad_value(209), s.ad_value(65)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul_ad_lhs(409, A::mul(s.ad_value(437), s.ad_value(207)), 279);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_add(408, 407, 409);
        }

        if (s.v[978] != 0.0) {
            s.store_sub(409, 408, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scale(282, 195, s.v[513]);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(406, 282, 407);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(405, 282, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(194, A::scale(s.ad_value(413), s.v[519]), A::scale(s.ad_value(412), s.v[518]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_add_ad(198, A::scale(s.ad_value(413), p.p174), A::scale(s.ad_value(412), p.p173));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_scale(198, 198, (-s.v[513]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_offset_ad(197, A::mul(A::neg(s.ad_value(198)), A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(194, A::scale(s.ad_value(412), s.v[519]), A::scale(s.ad_value(413), s.v[518]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_add_ad(199, A::scale(s.ad_value(412), p.p174), A::scale(s.ad_value(413), p.p173));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_scale(199, 199, (-s.v[513]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_offset_ad(196, A::mul(A::neg(s.ad_value(199)), s.ad_value(52)), s.v[196]);
        }

        s.v[1013] = if (((s.v[575] == 1.0) && (!(s.v[518] != 0.0))) || ((s.v[575] != 1.0) && (!(s.v[519] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1014] = if (p.p175 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[978] != 0.0)) && (s.v[1013] != 0.0)) && (s.v[1014] != 0.0)) {
            s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if (((!(s.v[978] != 0.0)) && (s.v[1013] != 0.0)) && (!(s.v[1014] != 0.0))) {
            s.store_scalar(198, 0.0);
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1013] != 0.0))) {
            s.store_add_ad(198, A::scale(s.ad_value(413), p.p174), A::scale(s.ad_value(412), p.p173));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1013] != 0.0))) {
            s.store_scale(198, 198, (-s.v[513]));
        }

        if (!(s.v[978] != 0.0)) {
            s.store_mul_ad(197, A::neg(s.ad_value(198)), A::sub(s.ad_value(52), s.ad_value(51)));
        }

        s.v[1015] = if (((s.v[575] == 1.0) && (!(s.v[519] != 0.0))) || ((s.v[575] != 1.0) && (!(s.v[518] != 0.0)))) { 1.0 } else { 0.0 };

        if ((!(s.v[978] != 0.0)) && (s.v[1015] != 0.0)) {
            s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1015] != 0.0))) {
            s.store_add_ad(199, A::scale(s.ad_value(412), p.p174), A::scale(s.ad_value(413), p.p173));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1015] != 0.0))) {
            s.store_scale(199, 199, (-s.v[513]));
        }

        if (!(s.v[978] != 0.0)) {
            s.store_mul_ad_lhs(196, A::neg(s.ad_value(199)), 52);
        }

        s.v[1016] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_mul_ad_lhs(279, A::scale(s.ad_value(386), (p.p223 * p.p224)), 386);
        }

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_offset_ad(280, A::add(A::scale(A::mul(s.ad_value(158), s.ad_value(86)), p.p223), A::mul(A::scale(s.ad_value(386), p.p224), s.ad_value(386))), 1e-50);
        }

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_div(221, 279, 280);
        }

        if ((s.v[38] != 0.0) && (!(s.v[1016] != 0.0))) {
            s.store_scalar(221, (p.p223 + 1e-50));
        }

        if (s.v[38] != 0.0) {
            s.store_scale(222, 270, (p.p225 * 0.0001));
        }

        s.v[1017] = if ((p.p21 != 0.0) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1017] != 0.0) {
            s.store_scalar(223, s.v[617]);
        }

        if (s.v[1017] != 0.0) {
            s.store_scalar(225, s.v[619]);
        }

        if (s.v[1017] != 0.0) {
            s.store_scale(279, 149, 6.241449993689894e18);
        }

        if (s.v[1017] != 0.0) {
            s.store_scale_ad(280, A::mul(A::add(A::add(s.ad_value(270), A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50)))), s.ad_value(225)), s.ad_value(122)), 6.241449993689894e18);
        }

        if (s.v[1017] != 0.0) {
            s.store_sub_ad_lhs(281, A::scale(A::div(A::scale(s.ad_value(91), ((-2.0) * 6.241449993689894e18)), s.ad_value(386)), 1.0 / (s.v[513])), 279);
        }

        s.v[1018] = if ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1017] != 0.0) && (s.v[1018] != 0.0)) {
            let assign21170_ad_e26697: A = A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(279), s.ad_value(280))), A::add(s.ad_value(281), s.ad_value(280))), A::mul(A::div(A::mul(A::mul(A::scale(s.ad_value(223), 2.0), s.ad_value(160)), s.ad_value(158)), A::sub(s.ad_value(281), s.ad_value(279))), A::ln(A::div(A::add(s.ad_value(281), s.ad_value(280)), A::add(s.ad_value(279), s.ad_value(280))))));
            s.store_add_ad(282, assign21170_ad_e26697, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(223), s.ad_value(160)), s.ad_value(158)), s.ad_value(223)), s.ad_value(160)), s.ad_value(158)));
        }

        if ((s.v[1017] != 0.0) && (!(s.v[1018] != 0.0))) {
            let assign21180_ad_e26748: A = A::add(A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(279), s.ad_value(280))), A::add(s.ad_value(281), s.ad_value(280))), A::div(A::mul(A::mul(A::scale(s.ad_value(223), 2.0), s.ad_value(160)), s.ad_value(158)), A::add(s.ad_value(279), s.ad_value(280)))), A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(223), s.ad_value(160)), s.ad_value(158)), s.ad_value(223)), s.ad_value(160)), s.ad_value(158)));
            s.store_ad(282, &assign21180_ad_e26748);
        }

        s.v[1019] = if ((p.p23 != 0.0) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1019] != 0.0) {
            s.store_div_ad_lhs(227, A::sub(s.ad_value(260), s.ad_value(56)), 386);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));
        }

        s.v[1020] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(285, 1.0);
        }

        s.v[1021] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (s.v[1021] != 0.0)) {
            s.copy_ad(285, 289);
        }

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (!(s.v[1021] != 0.0))) {
            s.store_powf(285, 289, (p.p114 - 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(287, A::mul(s.ad_value(289), s.ad_value(285)), 1.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(230, A::mul(s.ad_value(159), s.ad_value(287)), 288);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_add(228, 158, 230, 0.5);
        }

        if (s.v[1019] != 0.0) {
            s.store_square(278, 85);
        }

        if (s.v[1019] != 0.0) {
            let assign21340_ad_e26942: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 3.0), 1.0), A::scale(s.ad_value(278), 6.0)), s.ad_value(230)), s.ad_value(230)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 4.0), 3.0), A::scale(s.ad_value(278), 3.0)), s.ad_value(230)), s.ad_value(158))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 3.0), 6.0), s.ad_value(278)), s.ad_value(158)), s.ad_value(158)));
            s.store_div_ad(229, A::mul(A::mul(A::mul(A::scale(s.ad_value(270), s.v[466]), s.ad_value(86)), s.ad_value(158)), assign21340_ad_e26942), A::mul(A::mul(A::mul(A::scale(s.ad_value(386), 15.0), A::offset(s.ad_value(85), 1.0)), s.ad_value(228)), s.ad_value(228)));
        }

        if (!(s.v[1019] != 0.0)) {
            s.store_scalar(229, 0.0);
        }

        s.v[1022] = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1022] != 0.0) {
            s.store_sqrt(235, 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_add(280, 86, 235);
        }

        if (s.v[1022] != 0.0) {
            s.store_square(281, 231);
        }

        if (s.v[1022] != 0.0) {
            s.store_square(282, 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_lhs(283, A::scale(s.ad_value(231), 42.0), 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_rhs(283, 283, A::scale(A::add(s.ad_value(281), s.ad_value(282)), 4.0));
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_rhs(283, 283, A::mul(A::mul(A::scale(s.ad_value(235), 20.0), s.ad_value(86)), A::add(s.ad_value(231), s.ad_value(233))));
        }

        if (s.v[1022] != 0.0) {
            s.store_square(288, 280);
        }

        if (s.v[1022] != 0.0) {
            s.store_div_ad_rhs(236, 283, A::mul(A::square(s.ad_value(288)), s.ad_value(280)));
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_lhs(237, A::mul(A::div_from_scalar(s.v[466], s.ad_value(386)), s.ad_value(158)), 270);
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_lhs(285, A::add(s.ad_value(231), A::mul(A::scale(s.ad_value(86), 4.0), s.ad_value(235))), 233);
        }

        s.store_add(94, 94, 193);

        if (s.v[517] != 0.0) {
            s.store_scalar(200, ((-p.p172) * s.v[277]));
        }

        if (s.v[517] != 0.0) {
            s.store_mul_ad_rhs(201, 200, A::sub(s.ad_value(42), s.ad_value(40)));
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(200, 0.0);
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(201, 0.0);
        }

        s.v[215] = 0.0;

        s.store_scaled_sub(216, 42, 41, s.v[215]);

        s.store_scale(217, 42, s.v[215]);

        s.store_add(197, 197, 216);

        s.store_add(196, 196, 217);

        s.store_scale(0, 94, s.v[394]);

        s.store_scale(279, 123, (-s.v[513]));

        s.store_scaled_add(280, 523, 576, (-0.5));

        s.store_scaled_add(281, 531, 585, (-0.5));

        s.store_mul_ad_lhs(444, A::scale(s.ad_value(279), (0.1 * s.v[294])), 40);

        s.store_mul_ad(443, A::scale(s.ad_value(279), (0.1 * s.v[294])), A::sub(s.ad_value(40), s.ad_value(41)));

        s.store_mul(441, 279, 280);

        s.store_mul(442, 279, 281);

        if (p.p303 != 0.0) {
            s.store_scalar(336, 0.0);
        }

        if (p.p303 != 0.0) {
            s.copy_ad(92, 91);
        }

        if (!(p.p303 != 0.0)) {
            s.store_add_ad_lhs(92, A::add(s.ad_value(91), s.ad_value(441)), 442);
        }

        s.store_scale(93, 92, s.v[385]);

        if (s.v[38] != 0.0) {
            s.store_scalar(15, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scalar(14, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scalar(492, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(556, 336, s.v[394]);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(555, 92, s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scale_ad(14, A::sub(A::neg(s.ad_value(336)), s.ad_value(92)), s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scaled_add(15, 93, 443, s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scale_ad(16, A::add(A::sub(s.ad_value(92), s.ad_value(93)), s.ad_value(444)), s.v[394]);
        }

        s.v[1023] = if (p.p45 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1023] != 0.0) {
            s.store_scalar(219, 0.0);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_add_ad_lhs(218, A::mul(s.ad_value(261), s.ad_value(123)), 56);
        }

        s.v[1024] = if (s.v[218] > s.v[260]) { 1.0 } else { 0.0 };

        if ((!(s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) {
            s.copy_ad(218, 260);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_add_ad(279, A::scale(A::add(s.ad_value(51), s.ad_value(56)), s.v[264]), A::scale(s.ad_value(218), (1.0 - s.v[264])));
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_sqrt_ad(288, A::div_from_scalar((2.0 * 1.034943e-10), s.ad_value(126)));
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_scale(281, 288, 1.3);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_scale(280, 281, (1.034943e-10 * s.v[513]));
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1023] != 0.0)) {
            s.store_mul_ad_lhs(219, A::sub(A::scale(A::sub(A::add(s.ad_value(56), s.ad_value(51)), s.ad_value(279)), 1.0 / (p.p45)), s.ad_value(261)), 280);
        }

        s.v[1025] = if (p.p46 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1025] != 0.0) {
            s.store_add_ad_rhs(219, 219, A::scale(s.ad_value(50), s.v[490]));
        }

        s.v[1026] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(14, 14, A::scale(A::sub(A::sub(A::sub(A::sub(A::add(s.ad_value(197), s.ad_value(196)), s.ad_value(201)), s.ad_value(219)), s.ad_value(398)), s.ad_value(397)), s.v[394]));
        }

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(15, 15, A::scale(A::add(A::sub(s.ad_value(219), s.ad_value(197)), s.ad_value(405)), s.v[394]));
        }

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(16, 16, A::scale(A::sub(s.ad_value(406), s.ad_value(196)), s.v[394]));
        }

        s.store_scale(494, 185, s.v[394]);

        s.v[1027] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (!(s.v[1027] != 0.0)) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.v[1028] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1028] != 0.0) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        s.store_scale(573, 374, (4.0 * 1.3806226e-23));

        s.store_scale(564, 229, s.v[394]);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }

        s.v[1029] = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1029] != 0.0) {
            s.store_mul_ad_lhs(278, A::scale(s.ad_value(270), (1e-6 * s.v[513])), 123);
        }

        if (s.v[1029] != 0.0) {
            s.store_scale(288, 493, 1.0 / (s.v[394]));
        }

        if (s.v[1029] != 0.0) {
            s.store_div_ad_lhs(241, A::mul(A::mul(A::scale(s.ad_value(122), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(288)), s.ad_value(288)), 237);
        }

        s.v[1030] = if ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_div(242, 159, 158);
        }

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_div_ad_lhs(243, A::sub(A::div(s.ad_value(159), s.ad_value(230)), s.ad_value(242)), 51);
        }

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_add_ad_rhs(244, 242, A::div(A::mul(A::scale(s.ad_value(243), 0.6666666666666667), A::add(A::add(s.ad_value(231), A::mul(s.ad_value(86), s.ad_value(235))), s.ad_value(233))), A::add(s.ad_value(86), s.ad_value(235))));
        }

        if ((s.v[1029] != 0.0) && (!(s.v[1030] != 0.0))) {
            s.store_div(244, 159, 230);
        }

        if (s.v[1029] != 0.0) {
            s.store_mul_ad_lhs(495, A::mul(A::scale(s.ad_value(241), s.v[394]), s.ad_value(236)), 244);
        }

        if (s.v[1029] != 0.0) {
            s.store_ad(495, &{
                if (s.v[495] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(495)
                }
            });
        }

        if (s.v[1029] != 0.0) {
            s.store_ad(495, &{
                if ((-s.v[288]) > s.v[278]) {
                    s.ad_value(495)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1029] != 0.0)) {
            s.store_scalar(495, 0.0);
        }

        s.store_mul(608, 573, 564);

        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_ad(610, A::div(s.ad_value(495), s.ad_value(608)));
        } else {
            s.store_scalar(610, 0.0);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }

        s.v[1031] = if (p.p312 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1031] != 0.0) {
            s.store_scalar(1035, p.p317);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1036, p.p319);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1037, p.p324);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1041, p.p311);
        }

        if (s.v[1031] != 0.0) {
            s.store_ad(1039, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(2)), p.p33));
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1035, 1035, 0.0001);
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1036, 1036, 0.01);
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1040, 374, 1.0 / (s.v[445]));
        }

        if (s.v[1031] != 0.0) {
            s.store_powf(279, 1040, p.p320);
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1043, 1035, 279);
        }

        if (s.v[1031] != 0.0) {
            s.store_sub_ad(278, A::add(A::offset(A::scale(s.ad_value(1040), 0.4), 1.8), A::mul(A::scale(s.ad_value(1040), 0.1), s.ad_value(1040))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1040)), p.p321));
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1044, 1036, 278);
        }

        if (s.v[1031] != 0.0) {
            s.store_add_ad_rhs(1037, 1037, A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p325));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(1043, 1043, 1032);
        }

        if (s.v[1031] != 0.0) {
            s.store_offset_ad(1044, A::mul(A::mul(s.ad_value(1044), s.ad_value(1033)), s.ad_value(1034)), 1e-50);
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1045, 1039, 1041);
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(1046, 1043, 1045);
        }

        s.v[1051] = if (s.v[1039] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_div(279, 1046, 1044);
        }

        if ((s.v[1031] != 0.0) && (!(s.v[1051] != 0.0))) {
            s.store_div_ad_lhs(279, A::neg(s.ad_value(1046)), 1044);
        }

        s.v[1052] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_scalar(281, 1.0);
        }

        s.v[1053] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1031] != 0.0) && (!(s.v[1052] != 0.0))) && (s.v[1053] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1052] != 0.0))) && (!(s.v[1053] != 0.0))) {
            s.store_ad(281, &A::pow(s.ad_value(279), A::offset(s.ad_value(1037), (-1.0))));
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(280, 279, 281);
        }

        if (s.v[1031] != 0.0) {
            s.store_offset(282, 280, 1.0);
        }

        s.v[1054] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[1055] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (s.v[1055] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (!(s.v[1055] != 0.0))) {
            s.store_ad(284, &A::pow(s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0))));
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (!(s.v[1055] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        if (s.v[1031] != 0.0) {
            s.store_div_from_scalar(279, 1.6021918e-19, 1041);
        }

        s.v[1058] = if (p.p313 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1058] != 0.0) {
            s.store_scalar(1062, p.p316);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1063, p.p318);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1064, p.p323);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1068, p.p310);
        }

        if (s.v[1058] != 0.0) {
            s.store_ad(1066, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(11)), p.p33));
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1062, 1062, 0.0001);
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1063, 1063, 0.01);
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1067, 374, 1.0 / (s.v[445]));
        }

        if (s.v[1058] != 0.0) {
            s.store_powf(279, 1067, p.p320);
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1070, 1062, 279);
        }

        if (s.v[1058] != 0.0) {
            s.store_sub_ad(278, A::add(A::offset(A::scale(s.ad_value(1067), 0.4), 1.8), A::mul(A::scale(s.ad_value(1067), 0.1), s.ad_value(1067))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1067)), p.p321));
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1071, 1063, 278);
        }

        if (s.v[1058] != 0.0) {
            s.store_add_ad_rhs(1064, 1064, A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p325));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(1070, 1070, 1059);
        }

        if (s.v[1058] != 0.0) {
            s.store_offset_ad(1071, A::mul(A::mul(s.ad_value(1071), s.ad_value(1060)), s.ad_value(1061)), 1e-50);
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1072, 1066, 1068);
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(1073, 1070, 1072);
        }

        s.v[1078] = if (s.v[1066] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_div(279, 1073, 1071);
        }

        if ((s.v[1058] != 0.0) && (!(s.v[1078] != 0.0))) {
            s.store_div_ad_lhs(279, A::neg(s.ad_value(1073)), 1071);
        }

        s.v[1079] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_scalar(281, 1.0);
        }

        s.v[1080] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1058] != 0.0) && (!(s.v[1079] != 0.0))) && (s.v[1080] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1079] != 0.0))) && (!(s.v[1080] != 0.0))) {
            s.store_ad(281, &A::pow(s.ad_value(279), A::offset(s.ad_value(1064), (-1.0))));
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(280, 279, 281);
        }

        if (s.v[1058] != 0.0) {
            s.store_offset(282, 280, 1.0);
        }

        s.v[1081] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[1082] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (s.v[1082] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
            s.store_ad(284, &A::pow(s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0))));
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        if (s.v[1058] != 0.0) {
            s.store_div_from_scalar(279, 1.6021918e-19, 1068);
        }

        s.v[1085] = if (s.v[221] < 1e-18) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1085] != 0.0)) {
            s.store_scalar(221, 1e-18);
        }

        s.v[1086] = if (s.v[222] < 1e-18) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1086] != 0.0)) {
            s.store_scalar(222, 1e-18);
        }

        if (s.v[38] != 0.0) {
            s.store_div_ad_lhs(549, A::sub(s.ad_value(551), s.ad_value(555)), 221);
        }

        if (s.v[38] != 0.0) {
            s.store_div_ad_lhs(550, A::sub(s.ad_value(548), s.ad_value(556)), 222);
        }

        if (s.v[38] != 0.0) {
            s.store_sub_ad_lhs(554, A::neg(s.ad_value(551)), 548);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(552, 551, s.v[385]);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(553, 551, (1.0 - s.v[385]));
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(549, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(550, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(552, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(553, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(554, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(548, 0.0);
        }

        s.v[1087] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1087] != 0.0) {
            s.copy_ad(94, 0);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(185, 494);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(561, 14);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(93, 15);
        }

        if (s.v[1087] != 0.0) {
            s.store_neg_ad(492, A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16)));
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(90, 492);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_neg(94, 0);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(561, 14);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(93, 16);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_neg_ad(492, A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16)));
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(90, 492);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(16, 15);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(15, 93);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(279, 552);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(552, 553);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(553, 279);
        }

        s.v[1088] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1088] != 0.0) {
            s.store_mul(547, 0, 51);
        }

        if (s.v[1088] != 0.0) {
            s.store_scalar(516, s.v[468]);
        }

        if (s.v[1088] != 0.0) {
            s.store_scalar(557, (1.0 / s.v[467]));
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(547, 0.0);
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(557, 0.0);
        }

        s.copy_ad(0, 94);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        s.v[1094] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1095] = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };

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
        let eq0_e342: f64 = (p.p33 * s.v[94]);
        let eq0_e342_d_n0: f64 = (p.p33 * s.dn[94][0]);
        let eq0_e342_d_n1: f64 = (p.p33 * s.dn[94][1]);
        let eq0_e342_d_n2: f64 = (p.p33 * s.dn[94][2]);
        let eq0_e342_d_n3: f64 = (p.p33 * s.dn[94][3]);
        let eq0_e342_d_n4: f64 = (p.p33 * s.dn[94][4]);
        let eq0_e342_d_n5: f64 = (p.p33 * s.dn[94][5]);
        let eq0_e342_d_n6: f64 = (p.p33 * s.dn[94][6]);
        let eq0_e342_d_n7: f64 = (p.p33 * s.dn[94][7]);
        let eq0_e342_d_n8: f64 = (p.p33 * s.dn[94][8]);
        let eq0_e342_d_n9: f64 = (p.p33 * s.dn[94][9]);
        let eq0_e342_d_n10: f64 = (p.p33 * s.dn[94][10]);
        let eq0_e342_d_n11: f64 = (p.p33 * s.dn[94][11]);
        let eq0_e342_d_n12: f64 = (p.p33 * s.dn[94][12]);
        let eq0_e342_d_b0: f64 = (p.p33 * s.db[94][0]);
        let eq0_e342_d_b1: f64 = (p.p33 * s.db[94][1]);
        let eq0_e342_d_b2: f64 = (p.p33 * s.db[94][2]);
        let eq0_e342_d_b3: f64 = (p.p33 * s.db[94][3]);
        let eq0_value: f64 = eq0_e342;
        let eq0_node_derivatives: [f64; 13] = [eq0_e342_d_n0, eq0_e342_d_n1, eq0_e342_d_n2, eq0_e342_d_n3, eq0_e342_d_n4, eq0_e342_d_n5, eq0_e342_d_n6, eq0_e342_d_n7, eq0_e342_d_n8, eq0_e342_d_n9, eq0_e342_d_n10, eq0_e342_d_n11, eq0_e342_d_n12];
        let eq0_branch_derivatives: [f64; 4] = [eq0_e342_d_b0, eq0_e342_d_b1, eq0_e342_d_b2, eq0_e342_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
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
        let eq1_e346: f64 = (s.v[257] + s.v[185]);
        let eq1_e346_d_n0: f64 = (s.dn[257][0] + s.dn[185][0]);
        let eq1_e346_d_n1: f64 = (s.dn[257][1] + s.dn[185][1]);
        let eq1_e346_d_n2: f64 = (s.dn[257][2] + s.dn[185][2]);
        let eq1_e346_d_n3: f64 = (s.dn[257][3] + s.dn[185][3]);
        let eq1_e346_d_n4: f64 = (s.dn[257][4] + s.dn[185][4]);
        let eq1_e346_d_n5: f64 = (s.dn[257][5] + s.dn[185][5]);
        let eq1_e346_d_n6: f64 = (s.dn[257][6] + s.dn[185][6]);
        let eq1_e346_d_n7: f64 = (s.dn[257][7] + s.dn[185][7]);
        let eq1_e346_d_n8: f64 = (s.dn[257][8] + s.dn[185][8]);
        let eq1_e346_d_n9: f64 = (s.dn[257][9] + s.dn[185][9]);
        let eq1_e346_d_n10: f64 = (s.dn[257][10] + s.dn[185][10]);
        let eq1_e346_d_n11: f64 = (s.dn[257][11] + s.dn[185][11]);
        let eq1_e346_d_n12: f64 = (s.dn[257][12] + s.dn[185][12]);
        let eq1_e346_d_b0: f64 = (s.db[257][0] + s.db[185][0]);
        let eq1_e346_d_b1: f64 = (s.db[257][1] + s.db[185][1]);
        let eq1_e346_d_b2: f64 = (s.db[257][2] + s.db[185][2]);
        let eq1_e346_d_b3: f64 = (s.db[257][3] + s.db[185][3]);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n1: f64 = (p.p33 * eq1_e346_d_n1);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n3: f64 = (p.p33 * eq1_e346_d_n3);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n7: f64 = (p.p33 * eq1_e346_d_n7);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n9: f64 = (p.p33 * eq1_e346_d_n9);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_e347_d_b0: f64 = (p.p33 * eq1_e346_d_b0);
        let eq1_e347_d_b1: f64 = (p.p33 * eq1_e346_d_b1);
        let eq1_e347_d_b2: f64 = (p.p33 * eq1_e346_d_b2);
        let eq1_e347_d_b3: f64 = (p.p33 * eq1_e346_d_b3);
        let eq1_value: f64 = eq1_e347;
        let eq1_node_derivatives: [f64; 13] = [eq1_e347_d_n0, eq1_e347_d_n1, eq1_e347_d_n2, eq1_e347_d_n3, eq1_e347_d_n4, eq1_e347_d_n5, eq1_e347_d_n6, eq1_e347_d_n7, eq1_e347_d_n8, eq1_e347_d_n9, eq1_e347_d_n10, eq1_e347_d_n11, eq1_e347_d_n12];
        let eq1_branch_derivatives: [f64; 4] = [eq1_e347_d_b0, eq1_e347_d_b1, eq1_e347_d_b2, eq1_e347_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
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
        let eq2_e351: f64 = (s.v[258] + s.v[546]);
        let eq2_e351_d_n0: f64 = (s.dn[258][0] + s.dn[546][0]);
        let eq2_e351_d_n1: f64 = (s.dn[258][1] + s.dn[546][1]);
        let eq2_e351_d_n2: f64 = (s.dn[258][2] + s.dn[546][2]);
        let eq2_e351_d_n3: f64 = (s.dn[258][3] + s.dn[546][3]);
        let eq2_e351_d_n4: f64 = (s.dn[258][4] + s.dn[546][4]);
        let eq2_e351_d_n5: f64 = (s.dn[258][5] + s.dn[546][5]);
        let eq2_e351_d_n6: f64 = (s.dn[258][6] + s.dn[546][6]);
        let eq2_e351_d_n7: f64 = (s.dn[258][7] + s.dn[546][7]);
        let eq2_e351_d_n8: f64 = (s.dn[258][8] + s.dn[546][8]);
        let eq2_e351_d_n9: f64 = (s.dn[258][9] + s.dn[546][9]);
        let eq2_e351_d_n10: f64 = (s.dn[258][10] + s.dn[546][10]);
        let eq2_e351_d_n11: f64 = (s.dn[258][11] + s.dn[546][11]);
        let eq2_e351_d_n12: f64 = (s.dn[258][12] + s.dn[546][12]);
        let eq2_e351_d_b0: f64 = (s.db[258][0] + s.db[546][0]);
        let eq2_e351_d_b1: f64 = (s.db[258][1] + s.db[546][1]);
        let eq2_e351_d_b2: f64 = (s.db[258][2] + s.db[546][2]);
        let eq2_e351_d_b3: f64 = (s.db[258][3] + s.db[546][3]);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n1: f64 = (p.p33 * eq2_e351_d_n1);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n3: f64 = (p.p33 * eq2_e351_d_n3);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n7: f64 = (p.p33 * eq2_e351_d_n7);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n9: f64 = (p.p33 * eq2_e351_d_n9);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_e352_d_b0: f64 = (p.p33 * eq2_e351_d_b0);
        let eq2_e352_d_b1: f64 = (p.p33 * eq2_e351_d_b1);
        let eq2_e352_d_b2: f64 = (p.p33 * eq2_e351_d_b2);
        let eq2_e352_d_b3: f64 = (p.p33 * eq2_e351_d_b3);
        let eq2_value: f64 = eq2_e352;
        let eq2_node_derivatives: [f64; 13] = [eq2_e352_d_n0, eq2_e352_d_n1, eq2_e352_d_n2, eq2_e352_d_n3, eq2_e352_d_n4, eq2_e352_d_n5, eq2_e352_d_n6, eq2_e352_d_n7, eq2_e352_d_n8, eq2_e352_d_n9, eq2_e352_d_n10, eq2_e352_d_n11, eq2_e352_d_n12];
        let eq2_branch_derivatives: [f64; 4] = [eq2_e352_d_b0, eq2_e352_d_b1, eq2_e352_d_b2, eq2_e352_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[11]),
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
        let eq3_e355: f64 = (p.p33 * s.v[250]);
        let eq3_e355_d_n0: f64 = (p.p33 * s.dn[250][0]);
        let eq3_e355_d_n1: f64 = (p.p33 * s.dn[250][1]);
        let eq3_e355_d_n2: f64 = (p.p33 * s.dn[250][2]);
        let eq3_e355_d_n3: f64 = (p.p33 * s.dn[250][3]);
        let eq3_e355_d_n4: f64 = (p.p33 * s.dn[250][4]);
        let eq3_e355_d_n5: f64 = (p.p33 * s.dn[250][5]);
        let eq3_e355_d_n6: f64 = (p.p33 * s.dn[250][6]);
        let eq3_e355_d_n7: f64 = (p.p33 * s.dn[250][7]);
        let eq3_e355_d_n8: f64 = (p.p33 * s.dn[250][8]);
        let eq3_e355_d_n9: f64 = (p.p33 * s.dn[250][9]);
        let eq3_e355_d_n10: f64 = (p.p33 * s.dn[250][10]);
        let eq3_e355_d_n11: f64 = (p.p33 * s.dn[250][11]);
        let eq3_e355_d_n12: f64 = (p.p33 * s.dn[250][12]);
        let eq3_e355_d_b0: f64 = (p.p33 * s.db[250][0]);
        let eq3_e355_d_b1: f64 = (p.p33 * s.db[250][1]);
        let eq3_e355_d_b2: f64 = (p.p33 * s.db[250][2]);
        let eq3_e355_d_b3: f64 = (p.p33 * s.db[250][3]);
        let eq3_value: f64 = eq3_e355;
        let eq3_node_derivatives: [f64; 13] = [eq3_e355_d_n0, eq3_e355_d_n1, eq3_e355_d_n2, eq3_e355_d_n3, eq3_e355_d_n4, eq3_e355_d_n5, eq3_e355_d_n6, eq3_e355_d_n7, eq3_e355_d_n8, eq3_e355_d_n9, eq3_e355_d_n10, eq3_e355_d_n11, eq3_e355_d_n12];
        let eq3_branch_derivatives: [f64; 4] = [eq3_e355_d_b0, eq3_e355_d_b1, eq3_e355_d_b2, eq3_e355_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[12]),
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
        let eq4_e358: f64 = (p.p33 * s.v[251]);
        let eq4_e358_d_n0: f64 = (p.p33 * s.dn[251][0]);
        let eq4_e358_d_n1: f64 = (p.p33 * s.dn[251][1]);
        let eq4_e358_d_n2: f64 = (p.p33 * s.dn[251][2]);
        let eq4_e358_d_n3: f64 = (p.p33 * s.dn[251][3]);
        let eq4_e358_d_n4: f64 = (p.p33 * s.dn[251][4]);
        let eq4_e358_d_n5: f64 = (p.p33 * s.dn[251][5]);
        let eq4_e358_d_n6: f64 = (p.p33 * s.dn[251][6]);
        let eq4_e358_d_n7: f64 = (p.p33 * s.dn[251][7]);
        let eq4_e358_d_n8: f64 = (p.p33 * s.dn[251][8]);
        let eq4_e358_d_n9: f64 = (p.p33 * s.dn[251][9]);
        let eq4_e358_d_n10: f64 = (p.p33 * s.dn[251][10]);
        let eq4_e358_d_n11: f64 = (p.p33 * s.dn[251][11]);
        let eq4_e358_d_n12: f64 = (p.p33 * s.dn[251][12]);
        let eq4_e358_d_b0: f64 = (p.p33 * s.db[251][0]);
        let eq4_e358_d_b1: f64 = (p.p33 * s.db[251][1]);
        let eq4_e358_d_b2: f64 = (p.p33 * s.db[251][2]);
        let eq4_e358_d_b3: f64 = (p.p33 * s.db[251][3]);
        let eq4_value: f64 = eq4_e358;
        let eq4_node_derivatives: [f64; 13] = [eq4_e358_d_n0, eq4_e358_d_n1, eq4_e358_d_n2, eq4_e358_d_n3, eq4_e358_d_n4, eq4_e358_d_n5, eq4_e358_d_n6, eq4_e358_d_n7, eq4_e358_d_n8, eq4_e358_d_n9, eq4_e358_d_n10, eq4_e358_d_n11, eq4_e358_d_n12];
        let eq4_branch_derivatives: [f64; 4] = [eq4_e358_d_b0, eq4_e358_d_b1, eq4_e358_d_b2, eq4_e358_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[11]),
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
        let eq5_e361: f64 = (p.p33 * s.v[254]);
        let eq5_e361_d_n0: f64 = (p.p33 * s.dn[254][0]);
        let eq5_e361_d_n1: f64 = (p.p33 * s.dn[254][1]);
        let eq5_e361_d_n2: f64 = (p.p33 * s.dn[254][2]);
        let eq5_e361_d_n3: f64 = (p.p33 * s.dn[254][3]);
        let eq5_e361_d_n4: f64 = (p.p33 * s.dn[254][4]);
        let eq5_e361_d_n5: f64 = (p.p33 * s.dn[254][5]);
        let eq5_e361_d_n6: f64 = (p.p33 * s.dn[254][6]);
        let eq5_e361_d_n7: f64 = (p.p33 * s.dn[254][7]);
        let eq5_e361_d_n8: f64 = (p.p33 * s.dn[254][8]);
        let eq5_e361_d_n9: f64 = (p.p33 * s.dn[254][9]);
        let eq5_e361_d_n10: f64 = (p.p33 * s.dn[254][10]);
        let eq5_e361_d_n11: f64 = (p.p33 * s.dn[254][11]);
        let eq5_e361_d_n12: f64 = (p.p33 * s.dn[254][12]);
        let eq5_e361_d_b0: f64 = (p.p33 * s.db[254][0]);
        let eq5_e361_d_b1: f64 = (p.p33 * s.db[254][1]);
        let eq5_e361_d_b2: f64 = (p.p33 * s.db[254][2]);
        let eq5_e361_d_b3: f64 = (p.p33 * s.db[254][3]);
        let eq5_value: f64 = eq5_e361;
        let eq5_node_derivatives: [f64; 13] = [eq5_e361_d_n0, eq5_e361_d_n1, eq5_e361_d_n2, eq5_e361_d_n3, eq5_e361_d_n4, eq5_e361_d_n5, eq5_e361_d_n6, eq5_e361_d_n7, eq5_e361_d_n8, eq5_e361_d_n9, eq5_e361_d_n10, eq5_e361_d_n11, eq5_e361_d_n12];
        let eq5_branch_derivatives: [f64; 4] = [eq5_e361_d_b0, eq5_e361_d_b1, eq5_e361_d_b2, eq5_e361_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
        );
    }
}
