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

        s.v[39] = if self.param_given[9] { 1.0 } else { 0.0 };

        if (s.v[39] != 0.0) {
            s.store_scalar(10, p.p9);
        }

        if (!(s.v[39] != 0.0)) {
            s.store_scalar(10, 1.0);
        }

        s.v[40] = if self.param_given[10] { 1.0 } else { 0.0 };

        if (s.v[40] != 0.0) {
            s.store_scalar(11, (1.0 - (0.01 * p.p10)));
        }

        if (!(s.v[40] != 0.0)) {
            s.store_scalar(11, 1.0);
        }

        s.store_scaled_mul(15, 11, 10, 1000000.0);

        s.v[8] = (273.15 + p.p15);

        s.v[25] = ((ctx.temperature() + p.p5) - 273.15);

        s.v[44] = if (s.v[25] < (p.p34 + 1.0)) { 1.0 } else { 0.0 };

        if (s.v[44] != 0.0) {
            s.store_scalar(25, (p.p34 + ((((s.v[25] - p.p34) - 1.0)) as f64).exp()));
        }

        s.v[45] = if (s.v[25] > (p.p35 - 1.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[44] != 0.0)) && (s.v[45] != 0.0)) {
            s.store_sub_from_scalar_ad(25, p.p35, A::exp(A::offset(A::sub_from_scalar(p.p35, s.ad_value(25)), (-1.0))));
        }

        if ((!(s.v[44] != 0.0)) && (!(s.v[45] != 0.0))) {
        }

        s.store_offset(9, 25, 273.15);

        s.store_offset(12, 9, (-s.v[8]));

        s.store_scale_ad(22, A::offset(A::scale(s.ad_value(12), p.p42), 1.0), p.p29);

        s.v[46] = if (s.v[22] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[46] != 0.0) {
            s.store_scalar(22, 0.0);
        }

        s.v[47] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[47] != 0.0) {
            s.store_scalar(14, p.p22);
        }

        s.v[48] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[47] != 0.0)) && (s.v[48] != 0.0)) {
            s.store_scalar(14, (p.p22 * 0.5));
        }

        if ((!(s.v[47] != 0.0)) && (!(s.v[48] != 0.0))) {
            s.store_scalar(14, 0.0);
        }

        s.v[49] = if (((if self.param_given[1] { 1.0 } else { 0.0 } != 0.0) && (if self.param_given[2] { 1.0 } else { 0.0 } != 0.0)) && (!(if self.param_given[0] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[50] = if ((p.p2 == 0.0) || (p.p1 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_scalar(16, 0.0);
        }

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_scale(17, 15, p.p0);
        }

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_offset(4, 17, p.p21);
        }

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if ((s.v[49] != 0.0) && (s.v[50] != 0.0)) {
            s.store_scalar(19, 1e99);
        }

        if ((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) {
            s.store_scale(16, 15, p.p1);
        }

        if ((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) {
            s.store_add(3, 16, 14);
        }

        s.v[52] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (s.v[52] != 0.0)) {
            s.store_scale(4, 3, (p.p16 / p.p2));
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (s.v[52] != 0.0)) {
            s.store_offset(17, 4, (-p.p21));
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (s.v[52] != 0.0)) {
            s.store_scalar(5, p.p2);
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (s.v[52] != 0.0)) {
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (!(s.v[52] != 0.0))) {
            s.store_scale(17, 15, p.p0);
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (!(s.v[52] != 0.0))) {
            s.store_offset(4, 17, p.p21);
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (!(s.v[52] != 0.0))) {
            s.store_scalar(5, 0.0);
        }

        if (((s.v[49] != 0.0) && (!(s.v[50] != 0.0))) && (!(s.v[52] != 0.0))) {
            s.store_scalar(19, 1e99);
        }

        s.v[54] = if ((if self.param_given[2] { 1.0 } else { 0.0 } != 0.0) && (!(if self.param_given[1] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[55] = if (p.p2 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_scalar(16, 0.0);
        }

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_scale(17, 15, p.p0);
        }

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_offset(4, 17, p.p21);
        }

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if (((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (s.v[55] != 0.0)) {
            s.store_scalar(19, 1e99);
        }

        s.v[56] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_scalar(17, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_scale(16, 15, p.p1);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_add(3, 16, 14);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_scalar(5, 1e99);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (s.v[56] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) {
            s.store_scale(17, 15, p.p0);
        }

        if ((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) {
            s.store_offset(4, 17, p.p21);
        }

        s.v[58] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (s.v[58] != 0.0)) {
            s.store_scale(3, 4, (p.p2 / p.p16));
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (s.v[58] != 0.0)) {
            s.store_sub(16, 3, 14);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (s.v[58] != 0.0)) {
            s.store_scalar(5, p.p2);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (s.v[58] != 0.0)) {
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (!(s.v[58] != 0.0))) {
            s.store_scale(16, 15, p.p1);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (!(s.v[58] != 0.0))) {
            s.store_add(3, 16, 14);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (!(s.v[58] != 0.0))) {
            s.store_scalar(5, 1e99);
        }

        if (((((!(s.v[49] != 0.0)) && (s.v[54] != 0.0)) && (!(s.v[55] != 0.0))) && (!(s.v[56] != 0.0))) && (!(s.v[58] != 0.0))) {
            s.store_scalar(19, 0.0);
        }

        s.v[60] = if (p.p0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_scalar(17, 0.0);
        }

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_scale(16, 15, p.p1);
        }

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_add(3, 16, 14);
        }

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_scalar(5, 1e99);
        }

        if (((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (s.v[60] != 0.0)) {
            s.store_scalar(19, 0.0);
        }

        s.v[61] = if (p.p1 == 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scalar(16, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scalar(3, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scale(17, 15, p.p0);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_offset(4, 17, p.p21);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (s.v[61] != 0.0)) {
            s.store_scalar(19, 1e99);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scale(17, 15, p.p0);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_offset(4, 17, p.p21);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_scale(16, 15, p.p1);
        }

        if ((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) {
            s.store_add(3, 16, 14);
        }

        s.v[63] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        s.v[65] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (s.v[63] != 0.0)) && (s.v[65] != 0.0)) {
            s.store_scaled_div(5, 3, 4, p.p16);
        }

        if ((((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (s.v[63] != 0.0)) && (s.v[65] != 0.0)) {
            s.store_div_from_scalar(19, 1.0, 5);
        }

        if ((((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (s.v[63] != 0.0)) && (!(s.v[65] != 0.0))) {
            s.store_scalar(5, 0.0);
        }

        if ((((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (s.v[63] != 0.0)) && (!(s.v[65] != 0.0))) {
            s.store_scalar(19, 1e99);
        }

        if (((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (!(s.v[63] != 0.0))) {
            s.store_scalar(5, 1e99);
        }

        if (((((!(s.v[49] != 0.0)) && (!(s.v[54] != 0.0))) && (!(s.v[60] != 0.0))) && (!(s.v[61] != 0.0))) && (!(s.v[63] != 0.0))) {
            s.store_scalar(19, 0.0);
        }

        if (p.p24 != 0.0) {
            s.store_offset(18, 3, p.p23);
        }

        if (!(p.p24 != 0.0)) {
            s.store_offset(18, 16, p.p23);
        }

        s.v[34] = p.p36;

        s.v[35] = p.p37;

        s.v[71] = if (s.v[3] > 0.0) { 1.0 } else { 0.0 };

        s.v[72] = if ((p.p3 != 0.0) && (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[71] != 0.0) && (s.v[72] != 0.0)) {
            s.store_offset_ad(34, A::div_from_scalar(p.p38, s.ad_value(3)), s.v[34]);
        }

        if ((s.v[71] != 0.0) && (s.v[72] != 0.0)) {
            s.store_offset_ad(35, A::div_from_scalar(p.p39, s.ad_value(3)), s.v[35]);
        }

        s.v[73] = if ((p.p3 != 0.0) || (p.p4 != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[71] != 0.0) && (!(s.v[72] != 0.0))) && (s.v[73] != 0.0)) {
            s.store_add_ad_rhs(34, 34, A::div_from_scalar((0.5 * p.p38), s.ad_value(3)));
        }

        if (((s.v[71] != 0.0) && (!(s.v[72] != 0.0))) && (s.v[73] != 0.0)) {
            s.store_add_ad_rhs(35, 35, A::div_from_scalar((0.5 * p.p39), s.ad_value(3)));
        }

        s.v[74] = if (s.v[4] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[74] != 0.0) {
            s.store_add_ad_rhs(34, 34, A::div_from_scalar(p.p40, s.ad_value(4)));
        }

        if (s.v[74] != 0.0) {
            s.store_add_ad_rhs(35, 35, A::div_from_scalar(p.p41, s.ad_value(4)));
        }

        s.store_offset_ad(13, A::mul(s.ad_value(12), A::add(s.ad_value(34), A::mul(s.ad_value(12), s.ad_value(35)))), 1.0);

        s.v[76] = if (s.v[13] < (0.01 + 0.1)) { 1.0 } else { 0.0 };

        if (s.v[76] != 0.0) {
            s.store_offset_ad(13, A::scale(A::exp(A::offset(A::scale(A::offset(s.ad_value(13), (-0.01)), 10.0), (-1.0))), 0.1), 0.01);
        }

        if (!(s.v[76] != 0.0)) {
        }

        s.store_mul(20, 5, 13);

        s.store_div(21, 19, 13);

        s.store_ad(30, &A::voltage(ctx, &nodes, Some(0), Some(1)));

        s.v[77] = if ((s.v[5] > 0.0) && ((p.p28 > 0.0) || (p.p26 > 0.0))) { 1.0 } else { 0.0 };

        if (s.v[77] != 0.0) {
            s.store_div(31, 30, 18);
        }

        if (s.v[77] != 0.0) {
            s.store_scale(32, 31, p.p27);
        }

        if (s.v[77] != 0.0) {
            s.store_sqrt_ad(23, A::offset(A::square(s.ad_value(32)), 1.0));
        }

        if (s.v[77] != 0.0) {
            s.store_scale_ad(33, A::abs(s.ad_value(31)), p.p25);
        }

        if (s.v[77] != 0.0) {
            s.store_powf_ad(24, A::offset(A::mul(A::square(s.ad_value(33)), s.ad_value(33)), 1.0), 0.3333333333333333);
        }

        if (s.v[77] != 0.0) {
            s.store_add_ad(29, A::offset(A::scale(s.ad_value(23), p.p28), ((1.0 - p.p28) - p.p26)), A::scale(s.ad_value(24), p.p26));
        }

        if (!(s.v[77] != 0.0)) {
            s.store_scalar(29, 1.0);
        }

        s.store_mul(6, 20, 29);

        s.copy_ad(0, 30);

        s.store_div(1, 0, 6);

        s.v[80] = if (((p.p6 != 0.0) && (s.v[5] > 0.0)) && (s.v[19] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[80] != 0.0) {
            s.store_div_ad_lhs(26, A::mul(A::scale(s.ad_value(9), (4.0 * 1.3806505e-23)), s.ad_value(21)), 29);
        }

        s.v[81] = if (((p.p32 != 0.0) && (s.v[3] > 0.0)) && (s.v[4] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[80] != 0.0) && (s.v[81] != 0.0)) {
            s.store_div_ad_lhs(27, A::mul(A::mul(s.ad_value(22), A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(4))), p.p30)), s.ad_value(4)), 3);
        }

        s.v[82] = if ((s.v[16] > 0.0) && (s.v[17] > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[80] != 0.0) && (!(s.v[81] != 0.0))) && (s.v[82] != 0.0)) {
            s.store_div_ad_lhs(27, A::mul(A::mul(s.ad_value(22), A::powf(A::abs(A::div(s.ad_value(1), s.ad_value(17))), p.p30)), s.ad_value(17)), 16);
        }

        if (((s.v[80] != 0.0) && (!(s.v[81] != 0.0))) && (!(s.v[82] != 0.0))) {
            s.store_scalar(27, 0.0);
        }

        s.v[83] = if (s.v[1] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[80] != 0.0) && (s.v[83] != 0.0)) {
            s.store_neg(27, 27);
        }

        if (!(s.v[80] != 0.0)) {
            s.store_scalar(26, 0.0);
        }

        if (!(s.v[80] != 0.0)) {
            s.store_scalar(27, 0.0);
        }

        s.v[84] = if ((s.v[5] > 0.0) && (s.v[19] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[84] != 0.0) {
            s.store_mul(6, 20, 29);
        }

        if (!(s.v[84] != 0.0)) {
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
            ],
        );
        let eq1_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq1_value),
            &[
            ],
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[1]),
            self.multiplicity * (eq2_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
    }
}
