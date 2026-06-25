#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

use crate::device::veriloga_generated::support::{AdValue as GenericAdValue, ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

type A = GenericAdValue<{ Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type Scratch = GenericScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;
type ReactiveScratch = GenericReactiveScratch<{ Instance::VARIABLE_COUNT }, { Instance::NODE_COUNT }, { Instance::BRANCH_COUNT }>;

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = Scratch::new();

        s.store_offset_ad(12, A::offset(A::voltage(ctx, &nodes, Some(2), None), ctx.temperature()), p.p45);

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad(10, &{
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_add_ad(34, A::scale(s.ad_value(14), p.p22), A::div(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p21), s.ad_value(15)));

        s.store_scale(54, 14, p.p23);

        s.store_scaled_exp(16, 34, p.p0);

        s.store_scaled_exp(55, 54, p.p2);

        s.store_scale_ad(19, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p7), 1.0), p.p47);

        s.store_scale_ad(20, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p6), 1.0), p.p5);

        s.store_scale_ad(21, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p10), 1.0), p.p9);

        s.v[22] = p.p16;

        s.v[43] = (s.v[11] / 300.15);

        s.store_scale(44, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(45, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(46, A::div(A::neg(s.ad_value(45)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(47, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(44)), 1.5), A::scale(s.ad_value(46), 1.6021918e-19)));

        s.store_scale_ad(48, A::sub_from_scalar(p.p17, s.ad_value(47)), 1.0 / (s.v[43]));

        s.store_div_ad_lhs(49, A::sub_from_scalar(p.p17, s.ad_value(48)), 48);

        s.store_div_from_scalar_ad(51, s.v[22], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(49)), p.p18), 1.0));

        s.store_add_ad_lhs(18, A::mul(s.ad_value(44), s.ad_value(48)), 47);

        s.store_div_ad_lhs(50, A::sub(s.ad_value(18), s.ad_value(48)), 48);

        s.store_mul_ad_rhs(17, 51, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(50)), p.p18), 1.0));

        s.v[9] = p.p29;

        s.store_ad(40, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(4)), s.v[9]));

        s.store_ad(41, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(3)), s.v[9]));

        s.store_ad(42, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), s.v[9]));

        s.v[63] = if (s.v[16] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[63] != 0.0) {
            s.store_div_ad_rhs(0, 40, A::scale(s.ad_value(15), p.p1));
        }

        if (s.v[63] != 0.0) {
            s.store_div_ad(52, A::sub(A::neg(s.ad_value(40)), s.ad_value(20)), A::scale(s.ad_value(15), p.p11));
        }

        if (s.v[63] != 0.0) {
            s.store_div_ad(53, A::neg(s.ad_value(20)), A::scale(s.ad_value(15), p.p11));
        }

        s.v[64] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[63] != 0.0) && (s.v[64] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[63] != 0.0) && (s.v[64] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[63] != 0.0) && (!(s.v[64] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[63] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[63] != 0.0) {
            let assign410_ad_e599: A = {
                if ((!(s.v[52] >= 37.0)) && (!(s.v[52] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(52)), 1.0))
                } else {
                    {
                        if ((!(s.v[52] >= 37.0)) && (s.v[52] <= (-37.0))) {
                            A::exp(s.ad_value(52))
                        } else {
                            {
                                if (s.v[52] >= 37.0) {
                                    s.ad_value(52)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign410_ad_e633: A = {
                if ((!(s.v[53] >= 37.0)) && (!(s.v[53] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(53)), 1.0))
                } else {
                    {
                        if ((!(s.v[53] >= 37.0)) && (s.v[53] <= (-37.0))) {
                            A::exp(s.ad_value(53))
                        } else {
                            {
                                if (s.v[53] >= 37.0) {
                                    s.ad_value(53)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign410_ad_e599, assign410_ad_e633);
        }

        if (s.v[63] != 0.0) {
            s.store_sub_ad(23, A::mul(s.ad_value(16), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(19), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(40)), s.ad_value(21)), p.p8), 1.0)));
        }

        if (!(s.v[63] != 0.0)) {
            s.store_scalar(23, 0.0);
        }

        s.v[65] = if (s.v[55] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[65] != 0.0) {
            s.store_max_with_scalar_ad(60, A::sub_from_scalar(p.p4, s.ad_value(40)), 0.001);
        }

        if (s.v[65] != 0.0) {
            s.store_div_ad(0, A::scale(s.ad_value(40), ((-1.0) * p.p4)), A::mul(A::scale(s.ad_value(15), p.p3), s.ad_value(60)));
        }

        s.v[66] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[65] != 0.0) && (s.v[66] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[65] != 0.0) && (s.v[66] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[65] != 0.0) && (!(s.v[66] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[65] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[65] != 0.0) {
            s.store_mul_ad_rhs(26, 55, A::offset(s.ad_value(1), (-1.0)));
        }

        if (!(s.v[65] != 0.0)) {
            s.store_scalar(26, 0.0);
        }

        s.store_sub(24, 23, 26);

        s.store_offset_ad(58, A::powf(A::abs(A::scale(s.ad_value(41), 1.0 / (p.p48))), p.p49), 1.0);

        s.store_offset_ad(59, A::powf(A::abs(A::scale(s.ad_value(42), 1.0 / (p.p50))), p.p51), 1.0);

        s.store_mul_ad(29, A::scale(A::exp(A::scale(s.ad_value(14), p.p37)), p.p12), A::powf(s.ad_value(58), (1.0 / p.p49)));

        s.store_mul_ad(30, A::scale(A::exp(A::scale(s.ad_value(14), p.p38)), p.p14), A::powf(s.ad_value(59), (1.0 / p.p51)));

        s.v[67] = if (p.p31 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[67] != 0.0) {
            s.store_offset(29, 29, p.p13);
        }

        if (s.v[67] != 0.0) {
            s.store_offset(30, 30, p.p15);
        }

        s.copy_ad(25, 23);

        s.store_powf_ad(56, A::abs(A::scale(A::voltage(ctx, &nodes, Some(0), Some(1)), 1.0 / (p.p40))), p.p39);

        s.store_offset_ad(57, A::powf(A::offset(s.ad_value(56), 1.0), (1.0 / p.p39)), (-1.0));

        s.store_scale_ad(31, A::offset(A::scale(s.ad_value(57), p.p41), 1.0), p.p19);

        s.store_mul(32, 31, 25);

        s.v[68] = if (p.p32 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[68] != 0.0) {
            s.store_div_ad_rhs(29, 29, A::offset(A::powf(A::scale(A::abs(A::voltage(ctx, &nodes, Some(6), None)), 1.0 / (p.p20)), p.p44), 1.0));
        }

        if (!(s.v[68] != 0.0)) {
        }

        s.store_scale_ad(4, A::neg(s.ad_value(18)), p.p24);

        s.store_add(5, 40, 4);

        s.v[69] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[69] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[69] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p18)));
        }

        if (s.v[69] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p18)), s.ad_value(18)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[69] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(40), s.ad_value(18)))), (1.0 - p.p18))))), 1.0 / ((1.0 - p.p18)));
        }

        if (!(s.v[69] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(33, 17, A::add(s.ad_value(7), s.ad_value(8)));

        s.v[70] = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        s.v[71] = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };

        s.v[72] = if (p.p30 == (-1.0)) { 1.0 } else { 0.0 };

        s.store_scale(35, 10, (4.0 * 1.3806226e-23));

        s.v[28] = ((p.p12 + (p.p31 * p.p13)) / s.v[3]);

        s.v[27] = ((p.p14 + (p.p31 * p.p15)) / s.v[3]);

        s.v[73] = if ((s.v[28] > 0.0) && (s.v[28] >= p.p46)) { 1.0 } else { 0.0 };

        if (s.v[73] != 0.0) {
            s.store_ad(38, &{
                if ((s.v[29] / s.v[3]) >= p.p46) {
                    A::div(s.ad_value(35), A::scale(s.ad_value(29), 1.0 / (s.v[3])))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[74] = if ((s.v[27] > 0.0) && (s.v[27] >= p.p46)) { 1.0 } else { 0.0 };

        if (s.v[74] != 0.0) {
            s.store_ad(39, &{
                if ((s.v[30] / s.v[3]) >= p.p46) {
                    A::div(s.ad_value(35), A::scale(s.ad_value(30), 1.0 / (s.v[3])))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (if ((p.p28 > 0.0) && (p.p27 > 0.0)) { 1.0 } else { 0.0 } > 0.0) {
            s.store_scale_ad(37, A::powf(A::abs(s.ad_value(24)), p.p28), p.p27);
        } else {
            s.store_scalar(37, 0.0);
        }

        s.v[36] = (2.0 * 1.6021918e-19);

        stamper.stamp_potential_branch(
            Some(nodes[6]),
            None,
            branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[5]),
            None,
            branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[5]),
            None,
            branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[2]),
            None,
            branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[5]),
            None,
            branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[0]),
            Some(nodes[3]),
            branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(nodes[1]),
            Some(nodes[4]),
            branches[6],
            self.multiplicity,
        );

        self.stamp_transient_equation_0_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_1_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_2_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_3_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_4_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_5_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_6_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_7_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_8_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_9_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_10_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_11_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_12_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_13_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_14_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_15_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_16_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_17_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_18_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_19_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_20_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_21_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_22_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_23_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_24_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_25_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_26_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_27_block_0(ctx, stamper, &mut s);
        self.stamp_transient_equation_28_block_0(ctx, stamper, &mut s);
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut s = ReactiveScratch::new();

        s.store_offset_ad(12, A::offset(A::voltage(ctx, &nodes, Some(2), None), ctx.temperature()), p.p45);

        if ((1026.85 + 273.15) < (if (s.v[12] > ((-100.0) + 273.15)) { s.v[12] } else { ((-100.0) + 273.15) })) {
            s.store_scalar(10, (1026.85 + 273.15));
        } else {
            s.store_ad(10, &{
                if (s.v[12] > ((-100.0) + 273.15)) {
                    s.ad_value(12)
                } else {
                    A::constant(((-100.0) + 273.15))
                }
            });
        }

        s.v[3] = (p.p43 * p.p42);

        s.v[11] = (p.p25 + 273.15);

        s.store_scale(15, 10, 8.6170869e-5);

        s.store_scale(13, 10, 1.0 / (s.v[11]));

        s.store_ln(14, 13);

        s.store_add_ad(34, A::scale(s.ad_value(14), p.p22), A::div(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p21), s.ad_value(15)));

        s.store_scale(54, 14, p.p23);

        s.store_scaled_exp(16, 34, p.p0);

        s.store_scaled_exp(55, 54, p.p2);

        s.store_scale_ad(19, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p7), 1.0), p.p47);

        s.store_scale_ad(20, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p6), 1.0), p.p5);

        s.store_scale_ad(21, A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), p.p10), 1.0), p.p9);

        s.v[22] = p.p16;

        s.v[43] = (s.v[11] / 300.15);

        s.store_scale(44, 10, 0.003331667499583542);

        s.store_sub_from_scalar_ad(45, 1.16, A::div(A::mul(A::scale(s.ad_value(10), 0.000702), s.ad_value(10)), A::offset(s.ad_value(10), 1108.0)));

        s.store_offset_ad(46, A::div(A::neg(s.ad_value(45)), A::scale(s.ad_value(10), (2.0 * 1.3806226e-23))), (1.1150877 / (1.3806226e-23 * (300.15 + 300.15))));

        s.store_mul_ad(47, A::neg(A::scale(s.ad_value(15), 2.0)), A::add(A::scale(A::ln(s.ad_value(44)), 1.5), A::scale(s.ad_value(46), 1.6021918e-19)));

        s.store_scale_ad(48, A::sub_from_scalar(p.p17, s.ad_value(47)), 1.0 / (s.v[43]));

        s.store_div_ad_lhs(49, A::sub_from_scalar(p.p17, s.ad_value(48)), 48);

        s.store_div_from_scalar_ad(51, s.v[22], A::offset(A::scale(A::sub_from_scalar((0.0004 * (s.v[11] - 300.15)), s.ad_value(49)), p.p18), 1.0));

        s.store_add_ad_lhs(18, A::mul(s.ad_value(44), s.ad_value(48)), 47);

        s.store_div_ad_lhs(50, A::sub(s.ad_value(18), s.ad_value(48)), 48);

        s.store_mul_ad_rhs(17, 51, A::offset(A::scale(A::sub(A::scale(A::offset(s.ad_value(10), (-300.15)), 0.0004), s.ad_value(50)), p.p18), 1.0));

        s.v[9] = p.p29;

        s.store_ad(40, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(4)), s.v[9]));

        s.v[63] = if (s.v[16] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[63] != 0.0) {
            s.store_div_ad_rhs(0, 40, A::scale(s.ad_value(15), p.p1));
        }

        if (s.v[63] != 0.0) {
            s.store_div_ad(52, A::sub(A::neg(s.ad_value(40)), s.ad_value(20)), A::scale(s.ad_value(15), p.p11));
        }

        if (s.v[63] != 0.0) {
            s.store_div_ad(53, A::neg(s.ad_value(20)), A::scale(s.ad_value(15), p.p11));
        }

        s.v[64] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[63] != 0.0) && (s.v[64] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[63] != 0.0) && (s.v[64] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[63] != 0.0) && (!(s.v[64] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[63] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        if (s.v[63] != 0.0) {
            let assign410_ad_e599: A = {
                if ((!(s.v[52] >= 37.0)) && (!(s.v[52] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(52)), 1.0))
                } else {
                    {
                        if ((!(s.v[52] >= 37.0)) && (s.v[52] <= (-37.0))) {
                            A::exp(s.ad_value(52))
                        } else {
                            {
                                if (s.v[52] >= 37.0) {
                                    s.ad_value(52)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            let assign410_ad_e633: A = {
                if ((!(s.v[53] >= 37.0)) && (!(s.v[53] <= (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(53)), 1.0))
                } else {
                    {
                        if ((!(s.v[53] >= 37.0)) && (s.v[53] <= (-37.0))) {
                            A::exp(s.ad_value(53))
                        } else {
                            {
                                if (s.v[53] >= 37.0) {
                                    s.ad_value(53)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_sub_ad(2, assign410_ad_e599, assign410_ad_e633);
        }

        if (s.v[63] != 0.0) {
            s.store_sub_ad(23, A::mul(s.ad_value(16), A::offset(s.ad_value(1), (-1.0))), A::div(A::mul(s.ad_value(19), s.ad_value(2)), A::offset(A::scale(A::pow(A::abs(s.ad_value(40)), s.ad_value(21)), p.p8), 1.0)));
        }

        if (!(s.v[63] != 0.0)) {
            s.store_scalar(23, 0.0);
        }

        s.v[65] = if (s.v[55] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[65] != 0.0) {
            s.store_max_with_scalar_ad(60, A::sub_from_scalar(p.p4, s.ad_value(40)), 0.001);
        }

        if (s.v[65] != 0.0) {
            s.store_div_ad(0, A::scale(s.ad_value(40), ((-1.0) * p.p4)), A::mul(A::scale(s.ad_value(15), p.p3), s.ad_value(60)));
        }

        s.v[66] = if (s.v[0] > 80.0) { 1.0 } else { 0.0 };

        if ((s.v[65] != 0.0) && (s.v[66] != 0.0)) {
            s.store_offset(1, 0, (((-80.0)) + (1.0)));
        }

        if ((s.v[65] != 0.0) && (s.v[66] != 0.0)) {
            s.store_scalar(0, 80.0);
        }

        if ((s.v[65] != 0.0) && (!(s.v[66] != 0.0))) {
            s.store_scalar(1, 1.0);
        }

        if (s.v[65] != 0.0) {
            s.store_mul_ad_rhs(1, 1, A::exp(s.ad_value(0)));
        }

        s.copy_ad(25, 23);

        s.store_powf_ad(56, A::abs(A::scale(A::voltage(ctx, &nodes, Some(0), Some(1)), 1.0 / (p.p40))), p.p39);

        s.store_offset_ad(57, A::powf(A::offset(s.ad_value(56), 1.0), (1.0 / p.p39)), (-1.0));

        s.store_scale_ad(31, A::offset(A::scale(s.ad_value(57), p.p41), 1.0), p.p19);

        s.store_mul(32, 31, 25);

        s.v[68] = if (p.p32 == 1.0) { 1.0 } else { 0.0 };

        s.store_scale_ad(4, A::neg(s.ad_value(18)), p.p24);

        s.store_add(5, 40, 4);

        s.v[69] = if (s.v[5] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[69] != 0.0) {
            s.store_scalar(6, (((((-1.0) - p.p18) * (((1.0 - p.p24)) as f64).ln())) as f64).exp());
        }

        if (s.v[69] != 0.0) {
            s.store_scale_ad(7, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::scale(s.ad_value(6), ((1.0 - p.p24) * (1.0 - p.p24))))), 1.0 / ((1.0 - p.p18)));
        }

        if (s.v[69] != 0.0) {
            s.store_mul_ad_lhs(8, A::mul(s.ad_value(5), A::offset(A::div(A::scale(s.ad_value(5), (0.5 * p.p18)), s.ad_value(18)), (1.0 - p.p24))), 6);
        }

        if (!(s.v[69] != 0.0)) {
            s.store_scale_ad(7, A::mul(s.ad_value(18), A::sub_from_scalar(1.0, A::exp(A::scale(A::ln(A::sub_from_scalar(1.0, A::div(s.ad_value(40), s.ad_value(18)))), (1.0 - p.p18))))), 1.0 / ((1.0 - p.p18)));
        }

        if (!(s.v[69] != 0.0)) {
            s.store_scalar(8, 0.0);
        }

        s.store_mul_ad_rhs(33, 17, A::add(s.ad_value(7), s.ad_value(8)));

        s.v[70] = if ((p.p30 == 1.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        s.v[71] = if (((p.p30 == 2.0) && (p.p33 > 0.0)) && (p.p35 > 0.0)) { 1.0 } else { 0.0 };

        self.stamp_reactive_equation_2_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_6_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_10_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_12_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_25_block_0(ctx, stamper, &mut s);
        self.stamp_reactive_equation_26_block_0(ctx, stamper, &mut s);
    }
}
