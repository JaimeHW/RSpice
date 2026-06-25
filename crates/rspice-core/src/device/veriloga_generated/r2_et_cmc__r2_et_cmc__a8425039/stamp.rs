#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = Scratch::new();

        s.v[51] = if self.param_given[10] { 1.0 } else { 0.0 };

        if (s.v[51] != 0.0) {
            s.store_scalar(13, p.p10);
        }

        if (!(s.v[51] != 0.0)) {
            s.store_scalar(13, 1.0);
        }

        s.v[52] = if self.param_given[11] { 1.0 } else { 0.0 };

        if (s.v[52] != 0.0) {
            s.store_scalar(14, (1.0 - (0.01 * p.p11)));
        }

        if (!(s.v[52] != 0.0)) {
            s.store_scalar(14, 1.0);
        }

        s.store_scaled_mul(18, 14, 13, 1000000.0);

        s.v[11] = (273.15 + p.p16);

        s.v[28] = ((ctx.temperature() + p.p5) - 273.15);

        s.v[56] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[56] != 0.0) {
            s.store_scalar(17, p.p23);
        }

        s.v[57] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[56] != 0.0)) && (s.v[57] != 0.0)) {
            s.store_scalar(17, (p.p23 * 0.5));
        }

        if ((!(s.v[56] != 0.0)) && (!(s.v[57] != 0.0))) {
            s.store_scalar(17, 0.0);
        }

        s.v[58] = if (((if self.param_given[1] { 1.0 } else { 0.0 } != 0.0) && (if self.param_given[2] { 1.0 } else { 0.0 } != 0.0)) && (!(if self.param_given[0] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[59] = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(22, 1e99);
        }

        if ((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if ((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        s.v[61] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scale(4, 3, (p.p17 / p.p2));
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_offset(20, 4, (-p.p22));
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scalar(5, p.p2);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_div_from_scalar(22, 1.0, 5);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scalar(5, 0.0);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scalar(22, 1e99);
        }

        s.v[63] = if ((if self.param_given[2] { 1.0 } else { 0.0 } != 0.0) && (!(if self.param_given[1] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[64] = if (p.p2 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(22, 1e99);
        }

        s.v[65] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(20, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scale(19, 18, p.p1);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_add(3, 19, 17);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(5, 1e99);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(22, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        s.v[67] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_scale(3, 4, (p.p2 / p.p17));
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_sub(19, 3, 17);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_scalar(5, p.p2);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_div_from_scalar(22, 1.0, 5);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_scalar(5, 1e99);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_scalar(22, 0.0);
        }

        s.v[69] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(20, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scale(19, 18, p.p1);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_add(3, 19, 17);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(5, 1e99);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(22, 0.0);
        }

        s.v[70] = if (p.p1 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(22, 1e99);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        s.v[72] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        s.v[74] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (s.v[72] != 0.0)) && (s.v[74] != 0.0)) {
            s.store_scaled_div(5, 3, 4, p.p17);
        }

        if ((((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (s.v[72] != 0.0)) && (s.v[74] != 0.0)) {
            s.store_div_from_scalar(22, 1.0, 5);
        }

        if ((((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (s.v[72] != 0.0)) && (!(s.v[74] != 0.0))) {
            s.store_scalar(5, 0.0);
        }

        if ((((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (s.v[72] != 0.0)) && (!(s.v[74] != 0.0))) {
            s.store_scalar(22, 1e99);
        }

        if (((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (!(s.v[72] != 0.0))) {
            s.store_scalar(5, 1e99);
        }

        if (((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) && (!(s.v[72] != 0.0))) {
            s.store_scalar(22, 0.0);
        }

        if (p.p25 != 0.0) {
            s.store_offset(21, 3, p.p24);
        }

        if (!(p.p25 != 0.0)) {
            s.store_offset(21, 19, p.p24);
        }

        s.v[37] = p.p37;

        s.v[38] = p.p38;

        s.v[80] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        s.v[81] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[80] != 0.0) && (s.v[81] != 0.0)) {
            s.store_offset_ad(37, A::div_from_scalar(p.p39, s.ad_value(3)), s.v[37]);
        }

        if ((s.v[80] != 0.0) && (s.v[81] != 0.0)) {
            s.store_offset_ad(38, A::div_from_scalar(p.p40, s.ad_value(3)), s.v[38]);
        }

        s.v[82] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[80] != 0.0) && (!(s.v[81] != 0.0))) && (s.v[82] != 0.0)) {
            s.store_add_ad_rhs(37, 37, A::div_from_scalar((0.5 * p.p39), s.ad_value(3)));
        }

        if (((s.v[80] != 0.0) && (!(s.v[81] != 0.0))) && (s.v[82] != 0.0)) {
            s.store_add_ad_rhs(38, 38, A::div_from_scalar((0.5 * p.p40), s.ad_value(3)));
        }

        s.v[83] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[83] != 0.0) {
            s.store_add_ad_rhs(37, 37, A::div_from_scalar(p.p41, s.ad_value(4)));
        }

        if (s.v[83] != 0.0) {
            s.store_add_ad_rhs(38, 38, A::div_from_scalar(p.p42, s.ad_value(4)));
        }

        s.v[85] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[85] != 0.0) {
            s.store_scaled_add(46, 19, 20, 2.0);
        }

        s.v[86] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[85] != 0.0)) && (s.v[86] != 0.0)) {
            s.store_add_ad_lhs(46, A::scale(s.ad_value(19), 2.0), 20);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[86] != 0.0))) {
            s.store_scale(46, 19, 2.0);
        }

        s.store_mul(47, 19, 20);

        s.store_add_ad(41, A::offset(A::scale(s.ad_value(46), p.p45), p.p44), A::scale(s.ad_value(47), p.p46));

        s.store_add_ad(9, A::offset(A::scale(s.ad_value(46), p.p48), p.p47), A::scale(s.ad_value(47), p.p49));

        s.store_ad(42, &A::voltage(ctx, &nodes, Some(2), None));

        s.store_offset_scaled(28, 42, p.p7, s.v[28]);

        s.v[88] = if (s.v[28] < (p.p35 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[88] != 0.0) {
            s.store_offset_ad(28, A::exp(A::offset(A::offset(s.ad_value(28), (-p.p35)), (-1.0))), p.p35);
        }

        s.v[89] = if (s.v[28] > (p.p36 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[88] != 0.0)) && (s.v[89] != 0.0)) {
            s.store_sub_from_scalar_ad(28, p.p36, A::exp(A::offset(A::sub_from_scalar(p.p36, s.ad_value(28)), (-1.0))));
        }

        if ((!(s.v[88] != 0.0)) && (!(s.v[89] != 0.0))) {
        }

        s.store_offset(12, 28, 273.15);

        s.store_offset(15, 12, (-s.v[11]));

        s.store_offset_ad(16, A::mul(s.ad_value(15), A::add(s.ad_value(37), A::mul(s.ad_value(15), s.ad_value(38)))), 1.0);

        s.v[90] = if (s.v[16] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (s.v[90] != 0.0) {
            s.store_offset_ad(16, A::scale(A::exp(A::offset(A::scale(A::offset(s.ad_value(16), (-0.01)), 10.0), (-1.0))), 0.1), 0.01);
        }

        if (!(s.v[90] != 0.0)) {
        }

        s.store_mul(23, 5, 16);

        s.store_div(24, 22, 16);

        s.store_scale_ad(25, A::offset(A::scale(s.ad_value(15), p.p43), 1.0), p.p30);

        s.v[91] = if (s.v[25] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[91] != 0.0) {
            s.store_scalar(25, 0.0);
        }

        s.store_ad(33, &A::voltage(ctx, &nodes, Some(0), Some(1)));

        s.v[92] = if ((s.v[5] > 0.0) && ((p.p29 > 0.0) || (p.p27 > 0.0))) { 1.0 } else { 0.0 };

        if (s.v[92] != 0.0) {
            s.store_div(34, 33, 21);
        }

        if (s.v[92] != 0.0) {
            s.store_scale(35, 34, p.p28);
        }

        if (s.v[92] != 0.0) {
            s.store_sqrt_ad(26, A::offset(A::square(s.ad_value(35)), 1.0));
        }

        if (s.v[92] != 0.0) {
            s.store_scale_ad(36, A::abs(s.ad_value(34)), p.p26);
        }

        if (s.v[92] != 0.0) {
            s.store_powf_ad(27, A::offset(A::mul(A::square(s.ad_value(36)), s.ad_value(36)), 1.0), 0.3333333333333333);
        }

        if (s.v[92] != 0.0) {
            s.store_add_ad(32, A::offset(A::scale(s.ad_value(26), p.p29), ((1.0 - p.p29) - p.p27)), A::scale(s.ad_value(27), p.p27));
        }

        if (!(s.v[92] != 0.0)) {
            s.store_scalar(32, 1.0);
        }

        s.store_mul(6, 23, 32);

        s.copy_ad(0, 33);

        s.store_div(1, 0, 6);

        s.store_mul_ad_lhs(43, A::neg(s.ad_value(0)), 1);

        s.store_mul(44, 42, 41);

        s.store_mul(45, 42, 9);

        s.v[95] = if (((p.p6 != 0.0) && (s.v[5] > 0.0)) && (s.v[22] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[95] != 0.0) {
            s.store_div_ad_lhs(29, A::mul(A::scale(s.ad_value(12), (4.0 * 1.3806505e-23)), s.ad_value(24)), 32);
        }

        s.v[96] = if (((p.p33 != 0.0) && (s.v[3] > 0.0)) && (s.v[4] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[95] != 0.0) && (s.v[96] != 0.0)) {
            s.store_div_ad_lhs(30, A::mul(A::mul(s.ad_value(25), A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(4))), p.p31)), s.ad_value(4)), 3);
        }

        s.v[97] = if ((s.v[19] > 0.0) && (s.v[20] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[95] != 0.0) && (!(s.v[96] != 0.0))) && (s.v[97] != 0.0)) {
            s.store_div_ad_lhs(30, A::mul(A::mul(s.ad_value(25), A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(20))), p.p31)), s.ad_value(20)), 19);
        }

        if (((s.v[95] != 0.0) && (!(s.v[96] != 0.0))) && (!(s.v[97] != 0.0))) {
            s.store_scalar(30, 0.0);
        }

        s.v[98] = if (s.v[1] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[95] != 0.0) && (s.v[98] != 0.0)) {
            s.store_neg(30, 30);
        }

        if (!(s.v[95] != 0.0)) {
            s.store_scalar(29, 0.0);
        }

        if (!(s.v[95] != 0.0)) {
            s.store_scalar(30, 0.0);
        }

        s.v[99] = if ((s.v[5] > 0.0) && (s.v[22] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[99] != 0.0) {
            s.store_mul(6, 23, 32);
        }

        if (!(s.v[99] != 0.0)) {
            s.copy_ad(6, 5);
        }

        let eq0_value: f64 = s.v[1];
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * s.dn[1][0]),
                GeneratedDerivative::node(nodes[1], self.multiplicity * s.dn[1][1]),
                GeneratedDerivative::node(nodes[2], self.multiplicity * s.dn[1][2]),
            ],
        );
        let (eq1_e56, eq1_e56_d_n0, eq1_e56_d_n1, eq1_e56_d_n2,) = {
    if (p.p7 != 0.0) {
        (s.v[44], s.dn[44][0], s.dn[44][1], s.dn[44][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e56;
        stamper.stamp_current(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq1_e56_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq1_e56_d_n1),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq1_e56_d_n2),
            ],
        );
        let (eq2_e60, eq2_e60_d_n0, eq2_e60_d_n1, eq2_e60_d_n2,) = {
    if (p.p7 != 0.0) {
        (s.v[43], s.dn[43][0], s.dn[43][1], s.dn[43][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e60;
        stamper.stamp_current(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq2_e60_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq2_e60_d_n1),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq2_e60_d_n2),
            ],
        );
        let (eq3_e67, eq3_e67_d_n0, eq3_e67_d_n1, eq3_e67_d_n2,) = {
    if (!(p.p7 != 0.0)) {
        let eq3_e65: f64 = (1000000.0 * s.v[42]);
        let eq3_e65_d_n0: f64 = (1000000.0 * s.dn[42][0]);
        let eq3_e65_d_n1: f64 = (1000000.0 * s.dn[42][1]);
        let eq3_e65_d_n2: f64 = (1000000.0 * s.dn[42][2]);
        (eq3_e65, eq3_e65_d_n0, eq3_e65_d_n1, eq3_e65_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e67;
        stamper.stamp_current(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq3_e67_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq3_e67_d_n1),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq3_e67_d_n2),
            ],
        );
        let (eq4_e72, eq4_e72_d_n0, eq4_e72_d_n1, eq4_e72_d_n2,) = {
    if (p.p7 != 0.0) {
        let eq4_e70: f64 = self.eval_ddt(0, s.v[45]);
        let eq4_e70_d_n0: f64 = self.ddt_jacobian(s.dn[45][0]);
        let eq4_e70_d_n1: f64 = self.ddt_jacobian(s.dn[45][1]);
        let eq4_e70_d_n2: f64 = self.ddt_jacobian(s.dn[45][2]);
        (eq4_e70, eq4_e70_d_n0, eq4_e70_d_n1, eq4_e70_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e72;
        stamper.stamp_current(
            Some(nodes[2]),
            None,
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * eq4_e72_d_n0),
                GeneratedDerivative::node(nodes[1], self.multiplicity * eq4_e72_d_n1),
                GeneratedDerivative::node(nodes[2], self.multiplicity * eq4_e72_d_n2),
            ],
        );
        let eq5_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq5_value),
            &[
            ],
        );
        let eq6_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq6_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = ReactiveScratch::new();

        s.v[51] = if self.param_given[10] { 1.0 } else { 0.0 };

        if (s.v[51] != 0.0) {
            s.store_scalar(13, p.p10);
        }

        if (!(s.v[51] != 0.0)) {
            s.store_scalar(13, 1.0);
        }

        s.v[52] = if self.param_given[11] { 1.0 } else { 0.0 };

        if (s.v[52] != 0.0) {
            s.store_scalar(14, (1.0 - (0.01 * p.p11)));
        }

        if (!(s.v[52] != 0.0)) {
            s.store_scalar(14, 1.0);
        }

        s.store_scaled_mul(18, 14, 13, 1000000.0);

        s.v[56] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[56] != 0.0) {
            s.store_scalar(17, p.p23);
        }

        s.v[57] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[56] != 0.0)) && (s.v[57] != 0.0)) {
            s.store_scalar(17, (p.p23 * 0.5));
        }

        if ((!(s.v[56] != 0.0)) && (!(s.v[57] != 0.0))) {
            s.store_scalar(17, 0.0);
        }

        s.v[58] = if (((if self.param_given[1] { 1.0 } else { 0.0 } != 0.0) && (if self.param_given[2] { 1.0 } else { 0.0 } != 0.0)) && (!(if self.param_given[0] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[59] = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if ((s.v[58] != 0.0) && (s.v[59] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        if ((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if ((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        s.v[61] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scale(4, 3, (p.p17 / p.p2));
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_offset(20, 4, (-p.p22));
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if (((s.v[58] != 0.0) && (!(s.v[59] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        s.v[63] = if ((if self.param_given[2] { 1.0 } else { 0.0 } != 0.0) && (!(if self.param_given[1] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[64] = if (p.p2 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if (((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (s.v[64] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        s.v[65] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(20, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_scale(19, 18, p.p1);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (s.v[65] != 0.0)) {
            s.store_add(3, 19, 17);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        s.v[67] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_scale(3, 4, (p.p2 / p.p17));
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (s.v[67] != 0.0)) {
            s.store_sub(19, 3, 17);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if (((((!(s.v[58] != 0.0)) && (s.v[63] != 0.0)) && (!(s.v[64] != 0.0))) && (!(s.v[65] != 0.0))) && (!(s.v[67] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        s.v[69] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(20, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_scale(19, 18, p.p1);
        }

        if (((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (s.v[69] != 0.0)) {
            s.store_add(3, 19, 17);
        }

        s.v[70] = if (p.p1 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (s.v[70] != 0.0)) {
            s.store_offset(4, 20, p.p22);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_scale(20, 18, p.p0);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_offset(4, 20, p.p22);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_scale(19, 18, p.p1);
        }

        if ((((!(s.v[58] != 0.0)) && (!(s.v[63] != 0.0))) && (!(s.v[69] != 0.0))) && (!(s.v[70] != 0.0))) {
            s.store_add(3, 19, 17);
        }

        s.v[85] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[85] != 0.0) {
            s.store_scaled_add(46, 19, 20, 2.0);
        }

        s.v[86] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[85] != 0.0)) && (s.v[86] != 0.0)) {
            s.store_add_ad_lhs(46, A::scale(s.ad_value(19), 2.0), 20);
        }

        if ((!(s.v[85] != 0.0)) && (!(s.v[86] != 0.0))) {
            s.store_scale(46, 19, 2.0);
        }

        s.store_mul(47, 19, 20);

        s.store_add_ad(9, A::offset(A::scale(s.ad_value(46), p.p48), p.p47), A::scale(s.ad_value(47), p.p49));

        s.store_ad(42, &A::voltage(ctx, &nodes, Some(2), None));

        s.store_mul(45, 42, 9);

        let (eq4_e72, eq4_e72_d_n0, eq4_e72_d_n1, eq4_e72_d_n2, eq4_e72_q, eq4_e72_q_d_n0, eq4_e72_q_d_n1, eq4_e72_q_d_n2,) = {
    if (p.p7 != 0.0) {
        let eq4_e70_q: f64 = s.v[45];
        (s.v[45], s.dn[45][0], s.dn[45][1], s.dn[45][2], eq4_e70_q, s.dn[45][0], s.dn[45][1], s.dn[45][2],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            None,
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * (eq4_e72_q_d_n0)),
                GeneratedDerivative::node(nodes[1], self.multiplicity * (eq4_e72_q_d_n1)),
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq4_e72_q_d_n2)),
            ],
        );
    }
}
