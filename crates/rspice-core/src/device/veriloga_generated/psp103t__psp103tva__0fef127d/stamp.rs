#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;
#[path = "stamp_blocks_2.rs"]
mod stamp_blocks_2;
#[path = "stamp_blocks_3.rs"]
mod stamp_blocks_3;
#[path = "stamp_blocks_4.rs"]
mod stamp_blocks_4;
#[path = "stamp_blocks_5.rs"]
mod stamp_blocks_5;
#[path = "stamp_blocks_6.rs"]
mod stamp_blocks_6;
#[path = "stamp_blocks_7.rs"]
mod stamp_blocks_7;
#[path = "stamp_blocks_8.rs"]
mod stamp_blocks_8;
#[path = "stamp_blocks_9.rs"]
mod stamp_blocks_9;
#[path = "stamp_blocks_10.rs"]
mod stamp_blocks_10;
#[path = "stamp_blocks_11.rs"]
mod stamp_blocks_11;
#[path = "stamp_blocks_12.rs"]
mod stamp_blocks_12;
#[path = "stamp_blocks_13.rs"]
mod stamp_blocks_13;
#[path = "stamp_blocks_14.rs"]
mod stamp_blocks_14;
#[path = "stamp_blocks_15.rs"]
mod stamp_blocks_15;

const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

struct Scratch {
    values: [f64; Instance::VARIABLE_COUNT],
    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
}

impl Scratch {
    fn new() -> Self {
        Self {
            values: [0.0; Instance::VARIABLE_COUNT],
            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
        }
    }

    #[inline]
    fn ad_value(&self, index: usize) -> AdValue {
        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }
    }

    #[inline]
    fn store_ad(&mut self, index: usize, value: &AdValue) {
        self.values[index] = value.value;
        self.node_derivatives[index] = value.node_derivatives;
        self.branch_derivatives[index] = value.branch_derivatives;
    }
}

struct ReactiveScratch {
    values: [f64; Instance::VARIABLE_COUNT],
    node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
    branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
    reactive_values: [f64; Instance::VARIABLE_COUNT],
    reactive_node_derivatives: [[f64; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
    reactive_branch_derivatives: [[f64; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
}

impl ReactiveScratch {
    fn new() -> Self {
        Self {
            values: [0.0; Instance::VARIABLE_COUNT],
            node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
            branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
            reactive_values: [0.0; Instance::VARIABLE_COUNT],
            reactive_node_derivatives: [[0.0; Instance::NODE_COUNT]; Instance::VARIABLE_COUNT],
            reactive_branch_derivatives: [[0.0; Instance::BRANCH_COUNT]; Instance::VARIABLE_COUNT],
        }
    }

    #[inline]
    fn ad_value(&self, index: usize) -> AdValue {
        AdValue { value: self.values[index], node_derivatives: self.node_derivatives[index], branch_derivatives: self.branch_derivatives[index] }
    }

    #[inline]
    fn store_ad(&mut self, index: usize, value: &AdValue) {
        self.values[index] = value.value;
        self.node_derivatives[index] = value.node_derivatives;
        self.branch_derivatives[index] = value.branch_derivatives;
    }
}

struct AdValue {
    value: f64,
    node_derivatives: [f64; Instance::NODE_COUNT],
    branch_derivatives: [f64; Instance::BRANCH_COUNT],
}

impl AdValue {
    #[inline]
    fn constant(value: f64) -> Self {
        Self { value, node_derivatives: [0.0; Instance::NODE_COUNT], branch_derivatives: [0.0; Instance::BRANCH_COUNT] }
    }
    #[inline]
    fn voltage(ctx: &GeneratedEvalContext<'_>, nodes: &[usize; Instance::NODE_COUNT], pos: Option<usize>, neg: Option<usize>) -> Self {
        let pos_value = pos.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);
        let neg_value = neg.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);
        let mut value = Self::constant(pos_value - neg_value);
        if let Some(index) = pos { value.node_derivatives[index] += 1.0; }
        if let Some(index) = neg { value.node_derivatives[index] -= 1.0; }
        value
    }

    #[inline]
    fn branch_current(ctx: &GeneratedEvalContext<'_>, branches: &[usize; Instance::BRANCH_COUNT], slot: usize) -> Self {
        let mut value = Self::constant(ctx.branch_current(branches[slot]));
        value.branch_derivatives[slot] = 1.0;
        value
    }

    #[inline]
    fn neg(mut value: Self) -> Self {
        value.value = -value.value;
        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }
        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }
        value
    }

    #[inline]
    fn add(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value + right.value);
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] + right.node_derivatives[index]; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] + right.branch_derivatives[index]; }
        value
    }

    #[inline]
    fn sub(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value - right.value);
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] - right.node_derivatives[index]; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] - right.branch_derivatives[index]; }
        value
    }

    #[inline]
    fn mul(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value * right.value);
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] * right.value + left.value * right.node_derivatives[index]; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] * right.value + left.value * right.branch_derivatives[index]; }
        value
    }

    #[inline]
    fn square(arg: Self) -> Self {
        let mut value = Self::constant(arg.value * arg.value);
        let derivative_scale = 2.0 * arg.value;
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = derivative_scale * arg.node_derivatives[index]; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = derivative_scale * arg.branch_derivatives[index]; }
        value
    }

    #[inline]
    fn div(left: Self, right: Self) -> Self {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        let mut value = Self::constant(quotient);
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = left.node_derivatives[index] * reciprocal + right.node_derivatives[index] * right_scale; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = left.branch_derivatives[index] * reciprocal + right.branch_derivatives[index] * right_scale; }
        value
    }

    #[inline]
    fn div_from_scalar(scalar: f64, right: Self) -> Self {
        let reciprocal = 1.0 / right.value;
        let quotient = scalar * reciprocal;
        let right_scale = -quotient * reciprocal;
        let mut value = Self::constant(quotient);
        for index in 0..Instance::NODE_COUNT { value.node_derivatives[index] = right.node_derivatives[index] * right_scale; }
        for index in 0..Instance::BRANCH_COUNT { value.branch_derivatives[index] = right.branch_derivatives[index] * right_scale; }
        value
    }

    #[inline]
    fn scale(mut value: Self, scale: f64) -> Self {
        value.value *= scale;
        for derivative in &mut value.node_derivatives { *derivative *= scale; }
        for derivative in &mut value.branch_derivatives { *derivative *= scale; }
        value
    }

    #[inline]
    fn offset(mut value: Self, offset: f64) -> Self {
        value.value += offset;
        value
    }

    #[inline]
    fn sub_from_scalar(scalar: f64, mut value: Self) -> Self {
        value.value = scalar - value.value;
        for derivative in &mut value.node_derivatives { *derivative = -*derivative; }
        for derivative in &mut value.branch_derivatives { *derivative = -*derivative; }
        value
    }

    #[inline]
    fn unary_intrinsic(mut arg: Self, value: f64, derivative_scale: f64) -> Self {
        arg.value = value;
        for derivative in &mut arg.node_derivatives { *derivative *= derivative_scale; }
        for derivative in &mut arg.branch_derivatives { *derivative *= derivative_scale; }
        arg
    }

    #[inline]
    fn abs(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 }) }
    #[inline]
    fn sqrt(arg: Self) -> Self { let value = arg.value.sqrt(); Self::unary_intrinsic(arg, value, 1.0 / (2.0 * value)) }
    #[inline]
    fn exp(arg: Self) -> Self { let value = arg.value.exp(); Self::unary_intrinsic(arg, value, value) }
    #[inline]
    fn ln(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.ln(), 1.0 / raw) }
    #[inline]
    fn log10(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.log10(), 1.0 / (raw * std::f64::consts::LN_10)) }
    #[inline]
    fn sin(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sin(), raw.cos()) }
    #[inline]
    fn cos(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cos(), -raw.sin()) }
    #[inline]
    fn tan(arg: Self) -> Self { let raw = arg.value; let cos = raw.cos(); Self::unary_intrinsic(arg, raw.tan(), 1.0 / (cos * cos)) }
    #[inline]
    fn sinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sinh(), raw.cosh()) }
    #[inline]
    fn cosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cosh(), raw.sinh()) }
    #[inline]
    fn tanh(arg: Self) -> Self { let raw = arg.value; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), 1.0 / (cosh * cosh)) }
    #[inline]
    fn floor(arg: Self) -> Self { Self::constant(arg.value.floor()) }
    #[inline]
    fn ceil(arg: Self) -> Self { Self::constant(arg.value.ceil()) }
    #[inline]
    fn pow_derivative(value: f64, base: f64, exponent: f64, dbase: f64, dexponent: f64) -> f64 {
        if dexponent == 0.0 && exponent.is_finite() && exponent.fract() == 0.0 {
            if exponent == 0.0 { 0.0 } else { exponent * base.powf(exponent - 1.0) * dbase }
        } else {
            value * (dexponent * base.ln() + exponent * (dbase / base))
        }
    }
    #[inline]
    fn powf(left: Self, exponent: f64) -> Self {
        let value = left.value.powf(exponent);
        let mut result = Self::constant(value);
        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, left.value, exponent, left.node_derivatives[index], 0.0); }
        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, left.value, exponent, left.branch_derivatives[index], 0.0); }
        result
    }
    #[inline]
    fn pow_from_scalar(base: f64, right: Self) -> Self {
        let value = base.powf(right.value);
        let mut result = Self::constant(value);
        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, base, right.value, 0.0, right.node_derivatives[index]); }
        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, base, right.value, 0.0, right.branch_derivatives[index]); }
        result
    }
    #[inline]
    fn pow(left: Self, right: Self) -> Self {
        let value = left.value.powf(right.value);
        let mut result = Self::constant(value);
        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = Self::pow_derivative(value, left.value, right.value, left.node_derivatives[index], right.node_derivatives[index]); }
        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = Self::pow_derivative(value, left.value, right.value, left.branch_derivatives[index], right.branch_derivatives[index]); }
        result
    }
    #[inline]
    fn min(left: Self, right: Self) -> Self { if left.value <= right.value { left } else { right } }
    #[inline]
    fn min_with_scalar(left: Self, right: f64) -> Self { if left.value <= right { left } else { Self::constant(right) } }
    #[inline]
    fn min_from_scalar(left: f64, right: Self) -> Self { if left <= right.value { Self::constant(left) } else { right } }
    #[inline]
    fn max(left: Self, right: Self) -> Self { if left.value >= right.value { left } else { right } }
    #[inline]
    fn max_with_scalar(left: Self, right: f64) -> Self { if left.value >= right { left } else { Self::constant(right) } }
    #[inline]
    fn max_from_scalar(left: f64, right: Self) -> Self { if left >= right.value { Self::constant(left) } else { right } }
    #[inline]
    fn hypot(left: Self, right: Self) -> Self {
        let value = left.value.hypot(right.value);
        let mut result = Self::constant(value);
        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (left.value * left.node_derivatives[index] + right.value * right.node_derivatives[index]) / value; }
        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (left.value * left.branch_derivatives[index] + right.value * right.branch_derivatives[index]) / value; }
        result
    }
    #[inline]
    fn atan2(y: Self, x: Self) -> Self {
        let denominator = x.value * x.value + y.value * y.value;
        let mut result = Self::constant(y.value.atan2(x.value));
        for index in 0..Instance::NODE_COUNT { result.node_derivatives[index] = (x.value * y.node_derivatives[index] - y.value * x.node_derivatives[index]) / denominator; }
        for index in 0..Instance::BRANCH_COUNT { result.branch_derivatives[index] = (x.value * y.branch_derivatives[index] - y.value * x.branch_derivatives[index]) / denominator; }
        result
    }

    #[inline]
    fn ddt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {
        operand.value = value;
        for derivative in &mut operand.node_derivatives { *derivative *= derivative_scale; }
        for derivative in &mut operand.branch_derivatives { *derivative *= derivative_scale; }
        operand
    }

    #[inline]
    fn ddx_projection(expr: &Self, pos: Option<usize>, neg: Option<usize>) -> f64 {
        let pos = pos.map(|index| expr.node_derivatives[index]).unwrap_or(0.0);
        if let Some(neg) = neg { 0.5 * (pos - expr.node_derivatives[neg]) } else { pos }
    }
}

impl Instance {
    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let mut scratch = Scratch::new();

        self.stamp_transient_block_0(ctx, stamper, &mut scratch);
        self.stamp_transient_block_1(ctx, stamper, &mut scratch);
        self.stamp_transient_block_2(ctx, stamper, &mut scratch);
        self.stamp_transient_block_3(ctx, stamper, &mut scratch);
        self.stamp_transient_block_4(ctx, stamper, &mut scratch);
        self.stamp_transient_block_5(ctx, stamper, &mut scratch);
        self.stamp_transient_block_6(ctx, stamper, &mut scratch);
        self.stamp_transient_block_7(ctx, stamper, &mut scratch);
        self.stamp_transient_block_8(ctx, stamper, &mut scratch);
        self.stamp_transient_block_9(ctx, stamper, &mut scratch);
        self.stamp_transient_block_10(ctx, stamper, &mut scratch);
        self.stamp_transient_block_11(ctx, stamper, &mut scratch);
        self.stamp_transient_block_12(ctx, stamper, &mut scratch);
        self.stamp_transient_block_13(ctx, stamper, &mut scratch);
        self.stamp_transient_block_14(ctx, stamper, &mut scratch);
        self.stamp_transient_block_15(ctx, stamper, &mut scratch);
        self.stamp_transient_block_16(ctx, stamper, &mut scratch);
        self.stamp_transient_block_17(ctx, stamper, &mut scratch);
        self.stamp_transient_block_18(ctx, stamper, &mut scratch);
        self.stamp_transient_block_19(ctx, stamper, &mut scratch);
        self.stamp_transient_block_20(ctx, stamper, &mut scratch);
        self.stamp_transient_block_21(ctx, stamper, &mut scratch);
        self.stamp_transient_block_22(ctx, stamper, &mut scratch);
        self.stamp_transient_block_23(ctx, stamper, &mut scratch);
        self.stamp_transient_block_24(ctx, stamper, &mut scratch);
        self.stamp_transient_block_25(ctx, stamper, &mut scratch);
        self.stamp_transient_block_26(ctx, stamper, &mut scratch);
        self.stamp_transient_block_27(ctx, stamper, &mut scratch);
        self.stamp_transient_block_28(ctx, stamper, &mut scratch);
        self.stamp_transient_block_29(ctx, stamper, &mut scratch);
        self.stamp_transient_block_30(ctx, stamper, &mut scratch);
        self.stamp_transient_block_31(ctx, stamper, &mut scratch);
        self.stamp_transient_block_32(ctx, stamper, &mut scratch);
        self.stamp_transient_block_33(ctx, stamper, &mut scratch);
        self.stamp_transient_block_34(ctx, stamper, &mut scratch);
        self.stamp_transient_block_35(ctx, stamper, &mut scratch);
        self.stamp_transient_block_36(ctx, stamper, &mut scratch);
        self.stamp_transient_block_37(ctx, stamper, &mut scratch);
        self.stamp_transient_block_38(ctx, stamper, &mut scratch);
        self.stamp_transient_block_39(ctx, stamper, &mut scratch);
        self.stamp_transient_block_40(ctx, stamper, &mut scratch);
        self.stamp_transient_block_41(ctx, stamper, &mut scratch);
        self.stamp_transient_block_42(ctx, stamper, &mut scratch);
        self.stamp_transient_block_43(ctx, stamper, &mut scratch);

        stamper.stamp_potential_branch(
            Some(self.nodes[1]),
            Some(self.nodes[6]),
            self.branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[2]),
            Some(self.nodes[7]),
            self.branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[0]),
            Some(self.nodes[8]),
            self.branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[9]),
            Some(self.nodes[10]),
            self.branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[11]),
            Some(self.nodes[10]),
            self.branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[12]),
            Some(self.nodes[10]),
            self.branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[3]),
            Some(self.nodes[10]),
            self.branches[6],
            self.multiplicity,
        );

        let (eq0_e839, eq0_e839_d_n0, eq0_e839_d_n1, eq0_e839_d_n2, eq0_e839_d_n3, eq0_e839_d_n4, eq0_e839_d_n5, eq0_e839_d_n6, eq0_e839_d_n7, eq0_e839_d_n8, eq0_e839_d_n9, eq0_e839_d_n10, eq0_e839_d_n11, eq0_e839_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2594] != 0.0) {
        let eq0_e835: f64 = (scratch.values[0] * scratch.values[25]);
        let eq0_e835_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq0_e835_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq0_e835_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq0_e835_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq0_e835_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq0_e835_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq0_e835_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq0_e835_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq0_e835_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq0_e835_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq0_e835_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq0_e835_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq0_e835_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq0_e837: f64 = (eq0_e835 * scratch.values[2019]);
        let eq0_e837_d_n0: f64 = ((eq0_e835_d_n0 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][0]));
        let eq0_e837_d_n1: f64 = ((eq0_e835_d_n1 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][1]));
        let eq0_e837_d_n2: f64 = ((eq0_e835_d_n2 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][2]));
        let eq0_e837_d_n3: f64 = ((eq0_e835_d_n3 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][3]));
        let eq0_e837_d_n4: f64 = ((eq0_e835_d_n4 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][4]));
        let eq0_e837_d_n5: f64 = ((eq0_e835_d_n5 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][5]));
        let eq0_e837_d_n6: f64 = ((eq0_e835_d_n6 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][6]));
        let eq0_e837_d_n7: f64 = ((eq0_e835_d_n7 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][7]));
        let eq0_e837_d_n8: f64 = ((eq0_e835_d_n8 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][8]));
        let eq0_e837_d_n9: f64 = ((eq0_e835_d_n9 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][9]));
        let eq0_e837_d_n10: f64 = ((eq0_e835_d_n10 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][10]));
        let eq0_e837_d_n11: f64 = ((eq0_e835_d_n11 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][11]));
        let eq0_e837_d_n12: f64 = ((eq0_e835_d_n12 * scratch.values[2019]) + (eq0_e835 * scratch.node_derivatives[2019][12]));
        (eq0_e837, eq0_e837_d_n0, eq0_e837_d_n1, eq0_e837_d_n2, eq0_e837_d_n3, eq0_e837_d_n4, eq0_e837_d_n5, eq0_e837_d_n6, eq0_e837_d_n7, eq0_e837_d_n8, eq0_e837_d_n9, eq0_e837_d_n10, eq0_e837_d_n11, eq0_e837_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e839;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[9]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq0_e839_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq0_e839_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq0_e839_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq0_e839_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq0_e839_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq0_e839_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq0_e839_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq0_e839_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq0_e839_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq0_e839_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq0_e839_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq0_e839_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq0_e839_d_n12),
            ],
        );
        let (eq1_e849, eq1_e849_d_n0, eq1_e849_d_n1, eq1_e849_d_n2, eq1_e849_d_n3, eq1_e849_d_n4, eq1_e849_d_n5, eq1_e849_d_n6, eq1_e849_d_n7, eq1_e849_d_n8, eq1_e849_d_n9, eq1_e849_d_n10, eq1_e849_d_n11, eq1_e849_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2594] != 0.0) {
        let eq1_e843: f64 = (scratch.values[0] * scratch.values[25]);
        let eq1_e843_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq1_e843_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq1_e843_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq1_e843_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq1_e843_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq1_e843_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq1_e843_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq1_e843_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq1_e843_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq1_e843_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq1_e843_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq1_e843_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq1_e843_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq1_e846: f64 = (scratch.values[2018] + scratch.values[2069]);
        let eq1_e846_d_n0: f64 = (scratch.node_derivatives[2018][0] + scratch.node_derivatives[2069][0]);
        let eq1_e846_d_n1: f64 = (scratch.node_derivatives[2018][1] + scratch.node_derivatives[2069][1]);
        let eq1_e846_d_n2: f64 = (scratch.node_derivatives[2018][2] + scratch.node_derivatives[2069][2]);
        let eq1_e846_d_n3: f64 = (scratch.node_derivatives[2018][3] + scratch.node_derivatives[2069][3]);
        let eq1_e846_d_n4: f64 = (scratch.node_derivatives[2018][4] + scratch.node_derivatives[2069][4]);
        let eq1_e846_d_n5: f64 = (scratch.node_derivatives[2018][5] + scratch.node_derivatives[2069][5]);
        let eq1_e846_d_n6: f64 = (scratch.node_derivatives[2018][6] + scratch.node_derivatives[2069][6]);
        let eq1_e846_d_n7: f64 = (scratch.node_derivatives[2018][7] + scratch.node_derivatives[2069][7]);
        let eq1_e846_d_n8: f64 = (scratch.node_derivatives[2018][8] + scratch.node_derivatives[2069][8]);
        let eq1_e846_d_n9: f64 = (scratch.node_derivatives[2018][9] + scratch.node_derivatives[2069][9]);
        let eq1_e846_d_n10: f64 = (scratch.node_derivatives[2018][10] + scratch.node_derivatives[2069][10]);
        let eq1_e846_d_n11: f64 = (scratch.node_derivatives[2018][11] + scratch.node_derivatives[2069][11]);
        let eq1_e846_d_n12: f64 = (scratch.node_derivatives[2018][12] + scratch.node_derivatives[2069][12]);
        let eq1_e847: f64 = (eq1_e843 * eq1_e846);
        let eq1_e847_d_n0: f64 = ((eq1_e843_d_n0 * eq1_e846) + (eq1_e843 * eq1_e846_d_n0));
        let eq1_e847_d_n1: f64 = ((eq1_e843_d_n1 * eq1_e846) + (eq1_e843 * eq1_e846_d_n1));
        let eq1_e847_d_n2: f64 = ((eq1_e843_d_n2 * eq1_e846) + (eq1_e843 * eq1_e846_d_n2));
        let eq1_e847_d_n3: f64 = ((eq1_e843_d_n3 * eq1_e846) + (eq1_e843 * eq1_e846_d_n3));
        let eq1_e847_d_n4: f64 = ((eq1_e843_d_n4 * eq1_e846) + (eq1_e843 * eq1_e846_d_n4));
        let eq1_e847_d_n5: f64 = ((eq1_e843_d_n5 * eq1_e846) + (eq1_e843 * eq1_e846_d_n5));
        let eq1_e847_d_n6: f64 = ((eq1_e843_d_n6 * eq1_e846) + (eq1_e843 * eq1_e846_d_n6));
        let eq1_e847_d_n7: f64 = ((eq1_e843_d_n7 * eq1_e846) + (eq1_e843 * eq1_e846_d_n7));
        let eq1_e847_d_n8: f64 = ((eq1_e843_d_n8 * eq1_e846) + (eq1_e843 * eq1_e846_d_n8));
        let eq1_e847_d_n9: f64 = ((eq1_e843_d_n9 * eq1_e846) + (eq1_e843 * eq1_e846_d_n9));
        let eq1_e847_d_n10: f64 = ((eq1_e843_d_n10 * eq1_e846) + (eq1_e843 * eq1_e846_d_n10));
        let eq1_e847_d_n11: f64 = ((eq1_e843_d_n11 * eq1_e846) + (eq1_e843 * eq1_e846_d_n11));
        let eq1_e847_d_n12: f64 = ((eq1_e843_d_n12 * eq1_e846) + (eq1_e843 * eq1_e846_d_n12));
        (eq1_e847, eq1_e847_d_n0, eq1_e847_d_n1, eq1_e847_d_n2, eq1_e847_d_n3, eq1_e847_d_n4, eq1_e847_d_n5, eq1_e847_d_n6, eq1_e847_d_n7, eq1_e847_d_n8, eq1_e847_d_n9, eq1_e847_d_n10, eq1_e847_d_n11, eq1_e847_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e849;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq1_e849_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq1_e849_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq1_e849_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq1_e849_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq1_e849_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq1_e849_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq1_e849_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq1_e849_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq1_e849_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq1_e849_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq1_e849_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq1_e849_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq1_e849_d_n12),
            ],
        );
        let (eq2_e857, eq2_e857_d_n0, eq2_e857_d_n1, eq2_e857_d_n2, eq2_e857_d_n3, eq2_e857_d_n4, eq2_e857_d_n5, eq2_e857_d_n6, eq2_e857_d_n7, eq2_e857_d_n8, eq2_e857_d_n9, eq2_e857_d_n10, eq2_e857_d_n11, eq2_e857_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2594] != 0.0) {
        let eq2_e853: f64 = (scratch.values[0] * scratch.values[25]);
        let eq2_e853_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq2_e853_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq2_e853_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq2_e853_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq2_e853_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq2_e853_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq2_e853_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq2_e853_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq2_e853_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq2_e853_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq2_e853_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq2_e853_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq2_e853_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq2_e855: f64 = (eq2_e853 * scratch.values[2024]);
        let eq2_e855_d_n0: f64 = ((eq2_e853_d_n0 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][0]));
        let eq2_e855_d_n1: f64 = ((eq2_e853_d_n1 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][1]));
        let eq2_e855_d_n2: f64 = ((eq2_e853_d_n2 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][2]));
        let eq2_e855_d_n3: f64 = ((eq2_e853_d_n3 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][3]));
        let eq2_e855_d_n4: f64 = ((eq2_e853_d_n4 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][4]));
        let eq2_e855_d_n5: f64 = ((eq2_e853_d_n5 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][5]));
        let eq2_e855_d_n6: f64 = ((eq2_e853_d_n6 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][6]));
        let eq2_e855_d_n7: f64 = ((eq2_e853_d_n7 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][7]));
        let eq2_e855_d_n8: f64 = ((eq2_e853_d_n8 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][8]));
        let eq2_e855_d_n9: f64 = ((eq2_e853_d_n9 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][9]));
        let eq2_e855_d_n10: f64 = ((eq2_e853_d_n10 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][10]));
        let eq2_e855_d_n11: f64 = ((eq2_e853_d_n11 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][11]));
        let eq2_e855_d_n12: f64 = ((eq2_e853_d_n12 * scratch.values[2024]) + (eq2_e853 * scratch.node_derivatives[2024][12]));
        (eq2_e855, eq2_e855_d_n0, eq2_e855_d_n1, eq2_e855_d_n2, eq2_e855_d_n3, eq2_e855_d_n4, eq2_e855_d_n5, eq2_e855_d_n6, eq2_e855_d_n7, eq2_e855_d_n8, eq2_e855_d_n9, eq2_e855_d_n10, eq2_e855_d_n11, eq2_e855_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e857;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq2_e857_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq2_e857_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq2_e857_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq2_e857_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq2_e857_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq2_e857_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq2_e857_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq2_e857_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq2_e857_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq2_e857_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq2_e857_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq2_e857_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq2_e857_d_n12),
            ],
        );
        let (eq3_e865, eq3_e865_d_n0, eq3_e865_d_n1, eq3_e865_d_n2, eq3_e865_d_n3, eq3_e865_d_n4, eq3_e865_d_n5, eq3_e865_d_n6, eq3_e865_d_n7, eq3_e865_d_n8, eq3_e865_d_n9, eq3_e865_d_n10, eq3_e865_d_n11, eq3_e865_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2594] != 0.0) {
        let eq3_e861: f64 = (scratch.values[0] * scratch.values[25]);
        let eq3_e861_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq3_e861_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq3_e861_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq3_e861_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq3_e861_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq3_e861_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq3_e861_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq3_e861_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq3_e861_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq3_e861_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq3_e861_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq3_e861_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq3_e861_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq3_e863: f64 = (eq3_e861 * scratch.values[2023]);
        let eq3_e863_d_n0: f64 = ((eq3_e861_d_n0 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][0]));
        let eq3_e863_d_n1: f64 = ((eq3_e861_d_n1 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][1]));
        let eq3_e863_d_n2: f64 = ((eq3_e861_d_n2 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][2]));
        let eq3_e863_d_n3: f64 = ((eq3_e861_d_n3 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][3]));
        let eq3_e863_d_n4: f64 = ((eq3_e861_d_n4 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][4]));
        let eq3_e863_d_n5: f64 = ((eq3_e861_d_n5 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][5]));
        let eq3_e863_d_n6: f64 = ((eq3_e861_d_n6 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][6]));
        let eq3_e863_d_n7: f64 = ((eq3_e861_d_n7 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][7]));
        let eq3_e863_d_n8: f64 = ((eq3_e861_d_n8 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][8]));
        let eq3_e863_d_n9: f64 = ((eq3_e861_d_n9 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][9]));
        let eq3_e863_d_n10: f64 = ((eq3_e861_d_n10 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][10]));
        let eq3_e863_d_n11: f64 = ((eq3_e861_d_n11 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][11]));
        let eq3_e863_d_n12: f64 = ((eq3_e861_d_n12 * scratch.values[2023]) + (eq3_e861 * scratch.node_derivatives[2023][12]));
        (eq3_e863, eq3_e863_d_n0, eq3_e863_d_n1, eq3_e863_d_n2, eq3_e863_d_n3, eq3_e863_d_n4, eq3_e863_d_n5, eq3_e863_d_n6, eq3_e863_d_n7, eq3_e863_d_n8, eq3_e863_d_n9, eq3_e863_d_n10, eq3_e863_d_n11, eq3_e863_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e865;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq3_e865_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq3_e865_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq3_e865_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq3_e865_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq3_e865_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq3_e865_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq3_e865_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq3_e865_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq3_e865_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq3_e865_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq3_e865_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq3_e865_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq3_e865_d_n12),
            ],
        );
        let (eq4_e874, eq4_e874_d_n0, eq4_e874_d_n1, eq4_e874_d_n2, eq4_e874_d_n3, eq4_e874_d_n4, eq4_e874_d_n5, eq4_e874_d_n6, eq4_e874_d_n7, eq4_e874_d_n8, eq4_e874_d_n9, eq4_e874_d_n10, eq4_e874_d_n11, eq4_e874_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2594] != 0.0)) {
        let eq4_e870: f64 = (scratch.values[0] * scratch.values[25]);
        let eq4_e870_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq4_e870_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq4_e870_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq4_e870_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq4_e870_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq4_e870_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq4_e870_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq4_e870_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq4_e870_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq4_e870_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq4_e870_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq4_e870_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq4_e870_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq4_e872: f64 = (eq4_e870 * scratch.values[2019]);
        let eq4_e872_d_n0: f64 = ((eq4_e870_d_n0 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][0]));
        let eq4_e872_d_n1: f64 = ((eq4_e870_d_n1 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][1]));
        let eq4_e872_d_n2: f64 = ((eq4_e870_d_n2 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][2]));
        let eq4_e872_d_n3: f64 = ((eq4_e870_d_n3 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][3]));
        let eq4_e872_d_n4: f64 = ((eq4_e870_d_n4 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][4]));
        let eq4_e872_d_n5: f64 = ((eq4_e870_d_n5 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][5]));
        let eq4_e872_d_n6: f64 = ((eq4_e870_d_n6 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][6]));
        let eq4_e872_d_n7: f64 = ((eq4_e870_d_n7 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][7]));
        let eq4_e872_d_n8: f64 = ((eq4_e870_d_n8 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][8]));
        let eq4_e872_d_n9: f64 = ((eq4_e870_d_n9 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][9]));
        let eq4_e872_d_n10: f64 = ((eq4_e870_d_n10 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][10]));
        let eq4_e872_d_n11: f64 = ((eq4_e870_d_n11 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][11]));
        let eq4_e872_d_n12: f64 = ((eq4_e870_d_n12 * scratch.values[2019]) + (eq4_e870 * scratch.node_derivatives[2019][12]));
        (eq4_e872, eq4_e872_d_n0, eq4_e872_d_n1, eq4_e872_d_n2, eq4_e872_d_n3, eq4_e872_d_n4, eq4_e872_d_n5, eq4_e872_d_n6, eq4_e872_d_n7, eq4_e872_d_n8, eq4_e872_d_n9, eq4_e872_d_n10, eq4_e872_d_n11, eq4_e872_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e874;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[9]),
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq4_e874_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq4_e874_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq4_e874_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq4_e874_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq4_e874_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq4_e874_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq4_e874_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq4_e874_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq4_e874_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq4_e874_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq4_e874_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq4_e874_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq4_e874_d_n12),
            ],
        );
        let (eq5_e885, eq5_e885_d_n0, eq5_e885_d_n1, eq5_e885_d_n2, eq5_e885_d_n3, eq5_e885_d_n4, eq5_e885_d_n5, eq5_e885_d_n6, eq5_e885_d_n7, eq5_e885_d_n8, eq5_e885_d_n9, eq5_e885_d_n10, eq5_e885_d_n11, eq5_e885_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2594] != 0.0)) {
        let eq5_e879: f64 = (scratch.values[0] * scratch.values[25]);
        let eq5_e879_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq5_e879_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq5_e879_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq5_e879_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq5_e879_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq5_e879_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq5_e879_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq5_e879_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq5_e879_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq5_e879_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq5_e879_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq5_e879_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq5_e879_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq5_e882: f64 = (scratch.values[2018] + scratch.values[2069]);
        let eq5_e882_d_n0: f64 = (scratch.node_derivatives[2018][0] + scratch.node_derivatives[2069][0]);
        let eq5_e882_d_n1: f64 = (scratch.node_derivatives[2018][1] + scratch.node_derivatives[2069][1]);
        let eq5_e882_d_n2: f64 = (scratch.node_derivatives[2018][2] + scratch.node_derivatives[2069][2]);
        let eq5_e882_d_n3: f64 = (scratch.node_derivatives[2018][3] + scratch.node_derivatives[2069][3]);
        let eq5_e882_d_n4: f64 = (scratch.node_derivatives[2018][4] + scratch.node_derivatives[2069][4]);
        let eq5_e882_d_n5: f64 = (scratch.node_derivatives[2018][5] + scratch.node_derivatives[2069][5]);
        let eq5_e882_d_n6: f64 = (scratch.node_derivatives[2018][6] + scratch.node_derivatives[2069][6]);
        let eq5_e882_d_n7: f64 = (scratch.node_derivatives[2018][7] + scratch.node_derivatives[2069][7]);
        let eq5_e882_d_n8: f64 = (scratch.node_derivatives[2018][8] + scratch.node_derivatives[2069][8]);
        let eq5_e882_d_n9: f64 = (scratch.node_derivatives[2018][9] + scratch.node_derivatives[2069][9]);
        let eq5_e882_d_n10: f64 = (scratch.node_derivatives[2018][10] + scratch.node_derivatives[2069][10]);
        let eq5_e882_d_n11: f64 = (scratch.node_derivatives[2018][11] + scratch.node_derivatives[2069][11]);
        let eq5_e882_d_n12: f64 = (scratch.node_derivatives[2018][12] + scratch.node_derivatives[2069][12]);
        let eq5_e883: f64 = (eq5_e879 * eq5_e882);
        let eq5_e883_d_n0: f64 = ((eq5_e879_d_n0 * eq5_e882) + (eq5_e879 * eq5_e882_d_n0));
        let eq5_e883_d_n1: f64 = ((eq5_e879_d_n1 * eq5_e882) + (eq5_e879 * eq5_e882_d_n1));
        let eq5_e883_d_n2: f64 = ((eq5_e879_d_n2 * eq5_e882) + (eq5_e879 * eq5_e882_d_n2));
        let eq5_e883_d_n3: f64 = ((eq5_e879_d_n3 * eq5_e882) + (eq5_e879 * eq5_e882_d_n3));
        let eq5_e883_d_n4: f64 = ((eq5_e879_d_n4 * eq5_e882) + (eq5_e879 * eq5_e882_d_n4));
        let eq5_e883_d_n5: f64 = ((eq5_e879_d_n5 * eq5_e882) + (eq5_e879 * eq5_e882_d_n5));
        let eq5_e883_d_n6: f64 = ((eq5_e879_d_n6 * eq5_e882) + (eq5_e879 * eq5_e882_d_n6));
        let eq5_e883_d_n7: f64 = ((eq5_e879_d_n7 * eq5_e882) + (eq5_e879 * eq5_e882_d_n7));
        let eq5_e883_d_n8: f64 = ((eq5_e879_d_n8 * eq5_e882) + (eq5_e879 * eq5_e882_d_n8));
        let eq5_e883_d_n9: f64 = ((eq5_e879_d_n9 * eq5_e882) + (eq5_e879 * eq5_e882_d_n9));
        let eq5_e883_d_n10: f64 = ((eq5_e879_d_n10 * eq5_e882) + (eq5_e879 * eq5_e882_d_n10));
        let eq5_e883_d_n11: f64 = ((eq5_e879_d_n11 * eq5_e882) + (eq5_e879 * eq5_e882_d_n11));
        let eq5_e883_d_n12: f64 = ((eq5_e879_d_n12 * eq5_e882) + (eq5_e879 * eq5_e882_d_n12));
        (eq5_e883, eq5_e883_d_n0, eq5_e883_d_n1, eq5_e883_d_n2, eq5_e883_d_n3, eq5_e883_d_n4, eq5_e883_d_n5, eq5_e883_d_n6, eq5_e883_d_n7, eq5_e883_d_n8, eq5_e883_d_n9, eq5_e883_d_n10, eq5_e883_d_n11, eq5_e883_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e885;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[8]),
            self.multiplicity * (eq5_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq5_e885_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq5_e885_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq5_e885_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq5_e885_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq5_e885_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq5_e885_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq5_e885_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq5_e885_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq5_e885_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq5_e885_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq5_e885_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq5_e885_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq5_e885_d_n12),
            ],
        );
        let (eq6_e894, eq6_e894_d_n0, eq6_e894_d_n1, eq6_e894_d_n2, eq6_e894_d_n3, eq6_e894_d_n4, eq6_e894_d_n5, eq6_e894_d_n6, eq6_e894_d_n7, eq6_e894_d_n8, eq6_e894_d_n9, eq6_e894_d_n10, eq6_e894_d_n11, eq6_e894_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2594] != 0.0)) {
        let eq6_e890: f64 = (scratch.values[0] * scratch.values[25]);
        let eq6_e890_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq6_e890_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq6_e890_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq6_e890_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq6_e890_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq6_e890_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq6_e890_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq6_e890_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq6_e890_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq6_e890_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq6_e890_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq6_e890_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq6_e890_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq6_e892: f64 = (eq6_e890 * scratch.values[2024]);
        let eq6_e892_d_n0: f64 = ((eq6_e890_d_n0 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][0]));
        let eq6_e892_d_n1: f64 = ((eq6_e890_d_n1 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][1]));
        let eq6_e892_d_n2: f64 = ((eq6_e890_d_n2 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][2]));
        let eq6_e892_d_n3: f64 = ((eq6_e890_d_n3 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][3]));
        let eq6_e892_d_n4: f64 = ((eq6_e890_d_n4 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][4]));
        let eq6_e892_d_n5: f64 = ((eq6_e890_d_n5 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][5]));
        let eq6_e892_d_n6: f64 = ((eq6_e890_d_n6 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][6]));
        let eq6_e892_d_n7: f64 = ((eq6_e890_d_n7 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][7]));
        let eq6_e892_d_n8: f64 = ((eq6_e890_d_n8 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][8]));
        let eq6_e892_d_n9: f64 = ((eq6_e890_d_n9 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][9]));
        let eq6_e892_d_n10: f64 = ((eq6_e890_d_n10 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][10]));
        let eq6_e892_d_n11: f64 = ((eq6_e890_d_n11 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][11]));
        let eq6_e892_d_n12: f64 = ((eq6_e890_d_n12 * scratch.values[2024]) + (eq6_e890 * scratch.node_derivatives[2024][12]));
        (eq6_e892, eq6_e892_d_n0, eq6_e892_d_n1, eq6_e892_d_n2, eq6_e892_d_n3, eq6_e892_d_n4, eq6_e892_d_n5, eq6_e892_d_n6, eq6_e892_d_n7, eq6_e892_d_n8, eq6_e892_d_n9, eq6_e892_d_n10, eq6_e892_d_n11, eq6_e892_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e894;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq6_e894_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq6_e894_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq6_e894_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq6_e894_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq6_e894_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq6_e894_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq6_e894_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq6_e894_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq6_e894_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq6_e894_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq6_e894_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq6_e894_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq6_e894_d_n12),
            ],
        );
        let (eq7_e903, eq7_e903_d_n0, eq7_e903_d_n1, eq7_e903_d_n2, eq7_e903_d_n3, eq7_e903_d_n4, eq7_e903_d_n5, eq7_e903_d_n6, eq7_e903_d_n7, eq7_e903_d_n8, eq7_e903_d_n9, eq7_e903_d_n10, eq7_e903_d_n11, eq7_e903_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2594] != 0.0)) {
        let eq7_e899: f64 = (scratch.values[0] * scratch.values[25]);
        let eq7_e899_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq7_e899_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq7_e899_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq7_e899_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq7_e899_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq7_e899_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq7_e899_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq7_e899_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq7_e899_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq7_e899_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq7_e899_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq7_e899_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq7_e899_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq7_e901: f64 = (eq7_e899 * scratch.values[2023]);
        let eq7_e901_d_n0: f64 = ((eq7_e899_d_n0 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][0]));
        let eq7_e901_d_n1: f64 = ((eq7_e899_d_n1 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][1]));
        let eq7_e901_d_n2: f64 = ((eq7_e899_d_n2 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][2]));
        let eq7_e901_d_n3: f64 = ((eq7_e899_d_n3 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][3]));
        let eq7_e901_d_n4: f64 = ((eq7_e899_d_n4 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][4]));
        let eq7_e901_d_n5: f64 = ((eq7_e899_d_n5 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][5]));
        let eq7_e901_d_n6: f64 = ((eq7_e899_d_n6 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][6]));
        let eq7_e901_d_n7: f64 = ((eq7_e899_d_n7 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][7]));
        let eq7_e901_d_n8: f64 = ((eq7_e899_d_n8 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][8]));
        let eq7_e901_d_n9: f64 = ((eq7_e899_d_n9 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][9]));
        let eq7_e901_d_n10: f64 = ((eq7_e899_d_n10 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][10]));
        let eq7_e901_d_n11: f64 = ((eq7_e899_d_n11 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][11]));
        let eq7_e901_d_n12: f64 = ((eq7_e899_d_n12 * scratch.values[2023]) + (eq7_e899 * scratch.node_derivatives[2023][12]));
        (eq7_e901, eq7_e901_d_n0, eq7_e901_d_n1, eq7_e901_d_n2, eq7_e901_d_n3, eq7_e901_d_n4, eq7_e901_d_n5, eq7_e901_d_n6, eq7_e901_d_n7, eq7_e901_d_n8, eq7_e901_d_n9, eq7_e901_d_n10, eq7_e901_d_n11, eq7_e901_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e903;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq7_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq7_e903_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq7_e903_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq7_e903_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq7_e903_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq7_e903_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq7_e903_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq7_e903_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq7_e903_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq7_e903_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq7_e903_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq7_e903_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq7_e903_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq7_e903_d_n12),
            ],
        );
        let eq8_e906: f64 = (scratch.values[0] * scratch.values[25]);
        let eq8_e906_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq8_e906_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq8_e906_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq8_e906_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq8_e906_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq8_e906_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq8_e906_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq8_e906_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq8_e906_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq8_e906_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq8_e906_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq8_e906_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq8_e906_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq8_e908: f64 = (eq8_e906 * scratch.values[918]);
        let eq8_e908_d_n0: f64 = ((eq8_e906_d_n0 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][0]));
        let eq8_e908_d_n1: f64 = ((eq8_e906_d_n1 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][1]));
        let eq8_e908_d_n2: f64 = ((eq8_e906_d_n2 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][2]));
        let eq8_e908_d_n3: f64 = ((eq8_e906_d_n3 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][3]));
        let eq8_e908_d_n4: f64 = ((eq8_e906_d_n4 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][4]));
        let eq8_e908_d_n5: f64 = ((eq8_e906_d_n5 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][5]));
        let eq8_e908_d_n6: f64 = ((eq8_e906_d_n6 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][6]));
        let eq8_e908_d_n7: f64 = ((eq8_e906_d_n7 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][7]));
        let eq8_e908_d_n8: f64 = ((eq8_e906_d_n8 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][8]));
        let eq8_e908_d_n9: f64 = ((eq8_e906_d_n9 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][9]));
        let eq8_e908_d_n10: f64 = ((eq8_e906_d_n10 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][10]));
        let eq8_e908_d_n11: f64 = ((eq8_e906_d_n11 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][11]));
        let eq8_e908_d_n12: f64 = ((eq8_e906_d_n12 * scratch.values[918]) + (eq8_e906 * scratch.node_derivatives[918][12]));
        let eq8_value: f64 = eq8_e908;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[9]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq8_e908_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq8_e908_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq8_e908_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq8_e908_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq8_e908_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq8_e908_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq8_e908_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq8_e908_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq8_e908_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq8_e908_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq8_e908_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq8_e908_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq8_e908_d_n12),
            ],
        );
        let eq9_e911: f64 = (scratch.values[0] * scratch.values[25]);
        let eq9_e911_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq9_e911_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq9_e911_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq9_e911_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq9_e911_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq9_e911_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq9_e911_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq9_e911_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq9_e911_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq9_e911_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq9_e911_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq9_e911_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq9_e911_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq9_e913: f64 = (eq9_e911 * scratch.values[2022]);
        let eq9_e913_d_n0: f64 = ((eq9_e911_d_n0 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][0]));
        let eq9_e913_d_n1: f64 = ((eq9_e911_d_n1 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][1]));
        let eq9_e913_d_n2: f64 = ((eq9_e911_d_n2 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][2]));
        let eq9_e913_d_n3: f64 = ((eq9_e911_d_n3 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][3]));
        let eq9_e913_d_n4: f64 = ((eq9_e911_d_n4 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][4]));
        let eq9_e913_d_n5: f64 = ((eq9_e911_d_n5 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][5]));
        let eq9_e913_d_n6: f64 = ((eq9_e911_d_n6 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][6]));
        let eq9_e913_d_n7: f64 = ((eq9_e911_d_n7 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][7]));
        let eq9_e913_d_n8: f64 = ((eq9_e911_d_n8 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][8]));
        let eq9_e913_d_n9: f64 = ((eq9_e911_d_n9 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][9]));
        let eq9_e913_d_n10: f64 = ((eq9_e911_d_n10 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][10]));
        let eq9_e913_d_n11: f64 = ((eq9_e911_d_n11 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][11]));
        let eq9_e913_d_n12: f64 = ((eq9_e911_d_n12 * scratch.values[2022]) + (eq9_e911 * scratch.node_derivatives[2022][12]));
        let eq9_value: f64 = eq9_e913;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq9_e913_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq9_e913_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq9_e913_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq9_e913_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq9_e913_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq9_e913_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq9_e913_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq9_e913_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq9_e913_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq9_e913_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq9_e913_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq9_e913_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq9_e913_d_n12),
            ],
        );
        let eq10_e916: f64 = (scratch.values[0] * scratch.values[25]);
        let eq10_e916_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq10_e916_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq10_e916_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq10_e916_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq10_e916_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq10_e916_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq10_e916_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq10_e916_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq10_e916_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq10_e916_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq10_e916_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq10_e916_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq10_e916_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq10_e918: f64 = (eq10_e916 * scratch.values[2021]);
        let eq10_e918_d_n0: f64 = ((eq10_e916_d_n0 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][0]));
        let eq10_e918_d_n1: f64 = ((eq10_e916_d_n1 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][1]));
        let eq10_e918_d_n2: f64 = ((eq10_e916_d_n2 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][2]));
        let eq10_e918_d_n3: f64 = ((eq10_e916_d_n3 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][3]));
        let eq10_e918_d_n4: f64 = ((eq10_e916_d_n4 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][4]));
        let eq10_e918_d_n5: f64 = ((eq10_e916_d_n5 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][5]));
        let eq10_e918_d_n6: f64 = ((eq10_e916_d_n6 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][6]));
        let eq10_e918_d_n7: f64 = ((eq10_e916_d_n7 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][7]));
        let eq10_e918_d_n8: f64 = ((eq10_e916_d_n8 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][8]));
        let eq10_e918_d_n9: f64 = ((eq10_e916_d_n9 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][9]));
        let eq10_e918_d_n10: f64 = ((eq10_e916_d_n10 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][10]));
        let eq10_e918_d_n11: f64 = ((eq10_e916_d_n11 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][11]));
        let eq10_e918_d_n12: f64 = ((eq10_e916_d_n12 * scratch.values[2021]) + (eq10_e916 * scratch.node_derivatives[2021][12]));
        let eq10_value: f64 = eq10_e918;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq10_e918_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq10_e918_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq10_e918_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq10_e918_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq10_e918_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq10_e918_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq10_e918_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq10_e918_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq10_e918_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq10_e918_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq10_e918_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq10_e918_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq10_e918_d_n12),
            ],
        );
        let eq11_e921: f64 = (scratch.values[0] * scratch.values[25]);
        let eq11_e921_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq11_e921_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq11_e921_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq11_e921_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq11_e921_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq11_e921_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq11_e921_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq11_e921_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq11_e921_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq11_e921_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq11_e921_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq11_e921_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq11_e921_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq11_e923: f64 = (eq11_e921 * scratch.values[922]);
        let eq11_e923_d_n0: f64 = ((eq11_e921_d_n0 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][0]));
        let eq11_e923_d_n1: f64 = ((eq11_e921_d_n1 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][1]));
        let eq11_e923_d_n2: f64 = ((eq11_e921_d_n2 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][2]));
        let eq11_e923_d_n3: f64 = ((eq11_e921_d_n3 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][3]));
        let eq11_e923_d_n4: f64 = ((eq11_e921_d_n4 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][4]));
        let eq11_e923_d_n5: f64 = ((eq11_e921_d_n5 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][5]));
        let eq11_e923_d_n6: f64 = ((eq11_e921_d_n6 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][6]));
        let eq11_e923_d_n7: f64 = ((eq11_e921_d_n7 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][7]));
        let eq11_e923_d_n8: f64 = ((eq11_e921_d_n8 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][8]));
        let eq11_e923_d_n9: f64 = ((eq11_e921_d_n9 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][9]));
        let eq11_e923_d_n10: f64 = ((eq11_e921_d_n10 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][10]));
        let eq11_e923_d_n11: f64 = ((eq11_e921_d_n11 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][11]));
        let eq11_e923_d_n12: f64 = ((eq11_e921_d_n12 * scratch.values[922]) + (eq11_e921 * scratch.node_derivatives[922][12]));
        let eq11_value: f64 = eq11_e923;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[9]),
            self.multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq11_e923_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq11_e923_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq11_e923_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq11_e923_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq11_e923_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq11_e923_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq11_e923_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq11_e923_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq11_e923_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq11_e923_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq11_e923_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq11_e923_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq11_e923_d_n12),
            ],
        );
        let eq12_e926: f64 = (scratch.values[0] * scratch.values[25]);
        let eq12_e926_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq12_e926_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq12_e926_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq12_e926_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq12_e926_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq12_e926_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq12_e926_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq12_e926_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq12_e926_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq12_e926_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq12_e926_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq12_e926_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq12_e926_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq12_e928: f64 = (eq12_e926 * scratch.values[920]);
        let eq12_e928_d_n0: f64 = ((eq12_e926_d_n0 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][0]));
        let eq12_e928_d_n1: f64 = ((eq12_e926_d_n1 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][1]));
        let eq12_e928_d_n2: f64 = ((eq12_e926_d_n2 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][2]));
        let eq12_e928_d_n3: f64 = ((eq12_e926_d_n3 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][3]));
        let eq12_e928_d_n4: f64 = ((eq12_e926_d_n4 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][4]));
        let eq12_e928_d_n5: f64 = ((eq12_e926_d_n5 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][5]));
        let eq12_e928_d_n6: f64 = ((eq12_e926_d_n6 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][6]));
        let eq12_e928_d_n7: f64 = ((eq12_e926_d_n7 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][7]));
        let eq12_e928_d_n8: f64 = ((eq12_e926_d_n8 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][8]));
        let eq12_e928_d_n9: f64 = ((eq12_e926_d_n9 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][9]));
        let eq12_e928_d_n10: f64 = ((eq12_e926_d_n10 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][10]));
        let eq12_e928_d_n11: f64 = ((eq12_e926_d_n11 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][11]));
        let eq12_e928_d_n12: f64 = ((eq12_e926_d_n12 * scratch.values[920]) + (eq12_e926 * scratch.node_derivatives[920][12]));
        let eq12_value: f64 = eq12_e928;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[9]),
            self.multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq12_e928_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq12_e928_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq12_e928_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq12_e928_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq12_e928_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq12_e928_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq12_e928_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq12_e928_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq12_e928_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq12_e928_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq12_e928_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq12_e928_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq12_e928_d_n12),
            ],
        );
        let eq13_e931: f64 = (scratch.values[0] * scratch.values[25]);
        let eq13_e931_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq13_e931_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq13_e931_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq13_e931_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq13_e931_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq13_e931_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq13_e931_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq13_e931_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq13_e931_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq13_e931_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq13_e931_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq13_e931_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq13_e931_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq13_e933: f64 = (eq13_e931 * scratch.values[2030]);
        let eq13_e933_d_n0: f64 = ((eq13_e931_d_n0 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][0]));
        let eq13_e933_d_n1: f64 = ((eq13_e931_d_n1 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][1]));
        let eq13_e933_d_n2: f64 = ((eq13_e931_d_n2 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][2]));
        let eq13_e933_d_n3: f64 = ((eq13_e931_d_n3 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][3]));
        let eq13_e933_d_n4: f64 = ((eq13_e931_d_n4 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][4]));
        let eq13_e933_d_n5: f64 = ((eq13_e931_d_n5 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][5]));
        let eq13_e933_d_n6: f64 = ((eq13_e931_d_n6 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][6]));
        let eq13_e933_d_n7: f64 = ((eq13_e931_d_n7 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][7]));
        let eq13_e933_d_n8: f64 = ((eq13_e931_d_n8 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][8]));
        let eq13_e933_d_n9: f64 = ((eq13_e931_d_n9 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][9]));
        let eq13_e933_d_n10: f64 = ((eq13_e931_d_n10 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][10]));
        let eq13_e933_d_n11: f64 = ((eq13_e931_d_n11 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][11]));
        let eq13_e933_d_n12: f64 = ((eq13_e931_d_n12 * scratch.values[2030]) + (eq13_e931 * scratch.node_derivatives[2030][12]));
        let eq13_value: f64 = eq13_e933;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq13_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq13_e933_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq13_e933_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq13_e933_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq13_e933_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq13_e933_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq13_e933_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq13_e933_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq13_e933_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq13_e933_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq13_e933_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq13_e933_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq13_e933_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq13_e933_d_n12),
            ],
        );
        let eq14_e936: f64 = (scratch.values[0] * scratch.values[25]);
        let eq14_e936_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq14_e936_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq14_e936_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq14_e936_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq14_e936_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq14_e936_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq14_e936_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq14_e936_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq14_e936_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq14_e936_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq14_e936_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq14_e936_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq14_e936_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq14_e938: f64 = (eq14_e936 * scratch.values[2034]);
        let eq14_e938_d_n0: f64 = ((eq14_e936_d_n0 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][0]));
        let eq14_e938_d_n1: f64 = ((eq14_e936_d_n1 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][1]));
        let eq14_e938_d_n2: f64 = ((eq14_e936_d_n2 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][2]));
        let eq14_e938_d_n3: f64 = ((eq14_e936_d_n3 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][3]));
        let eq14_e938_d_n4: f64 = ((eq14_e936_d_n4 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][4]));
        let eq14_e938_d_n5: f64 = ((eq14_e936_d_n5 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][5]));
        let eq14_e938_d_n6: f64 = ((eq14_e936_d_n6 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][6]));
        let eq14_e938_d_n7: f64 = ((eq14_e936_d_n7 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][7]));
        let eq14_e938_d_n8: f64 = ((eq14_e936_d_n8 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][8]));
        let eq14_e938_d_n9: f64 = ((eq14_e936_d_n9 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][9]));
        let eq14_e938_d_n10: f64 = ((eq14_e936_d_n10 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][10]));
        let eq14_e938_d_n11: f64 = ((eq14_e936_d_n11 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][11]));
        let eq14_e938_d_n12: f64 = ((eq14_e936_d_n12 * scratch.values[2034]) + (eq14_e936 * scratch.node_derivatives[2034][12]));
        let eq14_value: f64 = eq14_e938;
        stamper.stamp_current(
            Some(self.nodes[12]),
            Some(self.nodes[8]),
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq14_e938_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq14_e938_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq14_e938_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq14_e938_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq14_e938_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq14_e938_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq14_e938_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq14_e938_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq14_e938_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq14_e938_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq14_e938_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq14_e938_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq14_e938_d_n12),
            ],
        );
        let (eq15_e946, eq15_e946_d_n0, eq15_e946_d_n1, eq15_e946_d_n2, eq15_e946_d_n3, eq15_e946_d_n4, eq15_e946_d_n5, eq15_e946_d_n6, eq15_e946_d_n7, eq15_e946_d_n8, eq15_e946_d_n9, eq15_e946_d_n10, eq15_e946_d_n11, eq15_e946_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2595] != 0.0) {
        let eq15_e942: f64 = (scratch.values[25] * scratch.values[842]);
        let eq15_e942_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][0]));
        let eq15_e942_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][1]));
        let eq15_e942_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][2]));
        let eq15_e942_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][3]));
        let eq15_e942_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][4]));
        let eq15_e942_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][5]));
        let eq15_e942_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][6]));
        let eq15_e942_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][7]));
        let eq15_e942_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][8]));
        let eq15_e942_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][9]));
        let eq15_e942_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][10]));
        let eq15_e942_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][11]));
        let eq15_e942_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[842]) + (scratch.values[25] * scratch.node_derivatives[842][12]));
        let eq15_e944: f64 = (eq15_e942 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n0: f64 = (eq15_e942_d_n0 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n1: f64 = ((eq15_e942_d_n1 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6]))) + eq15_e942);
        let eq15_e944_d_n2: f64 = (eq15_e942_d_n2 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n3: f64 = (eq15_e942_d_n3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n4: f64 = (eq15_e942_d_n4 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n5: f64 = (eq15_e942_d_n5 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n6: f64 = ((eq15_e942_d_n6 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6]))) + (eq15_e942 * -1.0));
        let eq15_e944_d_n7: f64 = (eq15_e942_d_n7 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n8: f64 = (eq15_e942_d_n8 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n9: f64 = (eq15_e942_d_n9 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n10: f64 = (eq15_e942_d_n10 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n11: f64 = (eq15_e942_d_n11 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        let eq15_e944_d_n12: f64 = (eq15_e942_d_n12 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[6])));
        (eq15_e944, eq15_e944_d_n0, eq15_e944_d_n1, eq15_e944_d_n2, eq15_e944_d_n3, eq15_e944_d_n4, eq15_e944_d_n5, eq15_e944_d_n6, eq15_e944_d_n7, eq15_e944_d_n8, eq15_e944_d_n9, eq15_e944_d_n10, eq15_e944_d_n11, eq15_e944_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e946;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[6]),
            self.multiplicity * (eq15_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq15_e946_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq15_e946_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq15_e946_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq15_e946_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq15_e946_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq15_e946_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq15_e946_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq15_e946_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq15_e946_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq15_e946_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq15_e946_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq15_e946_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq15_e946_d_n12),
            ],
        );
        let (eq16_e954,): (f64,) = {
    if (scratch.values[2595] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e954;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[6]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
        let (eq17_e959,): (f64,) = {
    if (!(scratch.values[2595] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e959;
        stamper.stamp_potential(
            self.branches[0],
            eq17_value,
            &[
            ],
        );
        let (eq18_e967, eq18_e967_d_n0, eq18_e967_d_n1, eq18_e967_d_n2, eq18_e967_d_n3, eq18_e967_d_n4, eq18_e967_d_n5, eq18_e967_d_n6, eq18_e967_d_n7, eq18_e967_d_n8, eq18_e967_d_n9, eq18_e967_d_n10, eq18_e967_d_n11, eq18_e967_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2596] != 0.0) {
        let eq18_e963: f64 = (scratch.values[25] * scratch.values[843]);
        let eq18_e963_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][0]));
        let eq18_e963_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][1]));
        let eq18_e963_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][2]));
        let eq18_e963_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][3]));
        let eq18_e963_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][4]));
        let eq18_e963_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][5]));
        let eq18_e963_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][6]));
        let eq18_e963_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][7]));
        let eq18_e963_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][8]));
        let eq18_e963_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][9]));
        let eq18_e963_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][10]));
        let eq18_e963_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][11]));
        let eq18_e963_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[843]) + (scratch.values[25] * scratch.node_derivatives[843][12]));
        let eq18_e965: f64 = (eq18_e963 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n0: f64 = (eq18_e963_d_n0 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n1: f64 = (eq18_e963_d_n1 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n2: f64 = ((eq18_e963_d_n2 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7]))) + eq18_e963);
        let eq18_e965_d_n3: f64 = (eq18_e963_d_n3 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n4: f64 = (eq18_e963_d_n4 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n5: f64 = (eq18_e963_d_n5 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n6: f64 = (eq18_e963_d_n6 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n7: f64 = ((eq18_e963_d_n7 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7]))) + (eq18_e963 * -1.0));
        let eq18_e965_d_n8: f64 = (eq18_e963_d_n8 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n9: f64 = (eq18_e963_d_n9 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n10: f64 = (eq18_e963_d_n10 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n11: f64 = (eq18_e963_d_n11 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        let eq18_e965_d_n12: f64 = (eq18_e963_d_n12 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[7])));
        (eq18_e965, eq18_e965_d_n0, eq18_e965_d_n1, eq18_e965_d_n2, eq18_e965_d_n3, eq18_e965_d_n4, eq18_e965_d_n5, eq18_e965_d_n6, eq18_e965_d_n7, eq18_e965_d_n8, eq18_e965_d_n9, eq18_e965_d_n10, eq18_e965_d_n11, eq18_e965_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e967;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[7]),
            self.multiplicity * (eq18_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq18_e967_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq18_e967_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq18_e967_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq18_e967_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq18_e967_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq18_e967_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq18_e967_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq18_e967_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq18_e967_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq18_e967_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq18_e967_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq18_e967_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq18_e967_d_n12),
            ],
        );
        let (eq19_e975,): (f64,) = {
    if (scratch.values[2596] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e975;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[7]),
            self.multiplicity * (eq19_value),
            &[
            ],
        );
        let (eq20_e980,): (f64,) = {
    if (!(scratch.values[2596] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e980;
        stamper.stamp_potential(
            self.branches[1],
            eq20_value,
            &[
            ],
        );
        let (eq21_e988, eq21_e988_d_n0, eq21_e988_d_n1, eq21_e988_d_n2, eq21_e988_d_n3, eq21_e988_d_n4, eq21_e988_d_n5, eq21_e988_d_n6, eq21_e988_d_n7, eq21_e988_d_n8, eq21_e988_d_n9, eq21_e988_d_n10, eq21_e988_d_n11, eq21_e988_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2597] != 0.0) {
        let eq21_e984: f64 = (scratch.values[25] * scratch.values[844]);
        let eq21_e984_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][0]));
        let eq21_e984_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][1]));
        let eq21_e984_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][2]));
        let eq21_e984_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][3]));
        let eq21_e984_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][4]));
        let eq21_e984_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][5]));
        let eq21_e984_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][6]));
        let eq21_e984_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][7]));
        let eq21_e984_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][8]));
        let eq21_e984_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][9]));
        let eq21_e984_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][10]));
        let eq21_e984_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][11]));
        let eq21_e984_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[844]) + (scratch.values[25] * scratch.node_derivatives[844][12]));
        let eq21_e986: f64 = (eq21_e984 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n0: f64 = ((eq21_e984_d_n0 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8]))) + eq21_e984);
        let eq21_e986_d_n1: f64 = (eq21_e984_d_n1 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n2: f64 = (eq21_e984_d_n2 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n3: f64 = (eq21_e984_d_n3 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n4: f64 = (eq21_e984_d_n4 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n5: f64 = (eq21_e984_d_n5 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n6: f64 = (eq21_e984_d_n6 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n7: f64 = (eq21_e984_d_n7 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n8: f64 = ((eq21_e984_d_n8 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8]))) + (eq21_e984 * -1.0));
        let eq21_e986_d_n9: f64 = (eq21_e984_d_n9 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n10: f64 = (eq21_e984_d_n10 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n11: f64 = (eq21_e984_d_n11 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        let eq21_e986_d_n12: f64 = (eq21_e984_d_n12 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[8])));
        (eq21_e986, eq21_e986_d_n0, eq21_e986_d_n1, eq21_e986_d_n2, eq21_e986_d_n3, eq21_e986_d_n4, eq21_e986_d_n5, eq21_e986_d_n6, eq21_e986_d_n7, eq21_e986_d_n8, eq21_e986_d_n9, eq21_e986_d_n10, eq21_e986_d_n11, eq21_e986_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e988;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[8]),
            self.multiplicity * (eq21_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq21_e988_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq21_e988_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq21_e988_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq21_e988_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq21_e988_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq21_e988_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq21_e988_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq21_e988_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq21_e988_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq21_e988_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq21_e988_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq21_e988_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq21_e988_d_n12),
            ],
        );
        let (eq22_e996,): (f64,) = {
    if (scratch.values[2597] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e996;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[8]),
            self.multiplicity * (eq22_value),
            &[
            ],
        );
        let (eq23_e1001,): (f64,) = {
    if (!(scratch.values[2597] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1001;
        stamper.stamp_potential(
            self.branches[2],
            eq23_value,
            &[
            ],
        );
        let (eq24_e1009, eq24_e1009_d_n0, eq24_e1009_d_n1, eq24_e1009_d_n2, eq24_e1009_d_n3, eq24_e1009_d_n4, eq24_e1009_d_n5, eq24_e1009_d_n6, eq24_e1009_d_n7, eq24_e1009_d_n8, eq24_e1009_d_n9, eq24_e1009_d_n10, eq24_e1009_d_n11, eq24_e1009_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2598] != 0.0) {
        let eq24_e1005: f64 = (scratch.values[25] * scratch.values[845]);
        let eq24_e1005_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][0]));
        let eq24_e1005_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][1]));
        let eq24_e1005_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][2]));
        let eq24_e1005_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][3]));
        let eq24_e1005_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][4]));
        let eq24_e1005_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][5]));
        let eq24_e1005_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][6]));
        let eq24_e1005_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][7]));
        let eq24_e1005_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][8]));
        let eq24_e1005_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][9]));
        let eq24_e1005_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][10]));
        let eq24_e1005_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][11]));
        let eq24_e1005_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[845]) + (scratch.values[25] * scratch.node_derivatives[845][12]));
        let eq24_e1007: f64 = (eq24_e1005 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n0: f64 = (eq24_e1005_d_n0 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n1: f64 = (eq24_e1005_d_n1 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n2: f64 = (eq24_e1005_d_n2 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n3: f64 = (eq24_e1005_d_n3 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n4: f64 = (eq24_e1005_d_n4 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n5: f64 = (eq24_e1005_d_n5 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n6: f64 = (eq24_e1005_d_n6 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n7: f64 = (eq24_e1005_d_n7 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n8: f64 = (eq24_e1005_d_n8 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n9: f64 = ((eq24_e1005_d_n9 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10]))) + eq24_e1005);
        let eq24_e1007_d_n10: f64 = ((eq24_e1005_d_n10 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10]))) + (eq24_e1005 * -1.0));
        let eq24_e1007_d_n11: f64 = (eq24_e1005_d_n11 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        let eq24_e1007_d_n12: f64 = (eq24_e1005_d_n12 * (ctx.node_voltage(self.nodes[9]) - ctx.node_voltage(self.nodes[10])));
        (eq24_e1007, eq24_e1007_d_n0, eq24_e1007_d_n1, eq24_e1007_d_n2, eq24_e1007_d_n3, eq24_e1007_d_n4, eq24_e1007_d_n5, eq24_e1007_d_n6, eq24_e1007_d_n7, eq24_e1007_d_n8, eq24_e1007_d_n9, eq24_e1007_d_n10, eq24_e1007_d_n11, eq24_e1007_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1009;
        stamper.stamp_current(
            Some(self.nodes[9]),
            Some(self.nodes[10]),
            self.multiplicity * (eq24_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq24_e1009_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq24_e1009_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq24_e1009_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq24_e1009_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq24_e1009_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq24_e1009_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq24_e1009_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq24_e1009_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq24_e1009_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq24_e1009_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq24_e1009_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq24_e1009_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq24_e1009_d_n12),
            ],
        );
        let (eq25_e1017,): (f64,) = {
    if (scratch.values[2598] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1017;
        stamper.stamp_current(
            Some(self.nodes[9]),
            Some(self.nodes[10]),
            self.multiplicity * (eq25_value),
            &[
            ],
        );
        let (eq26_e1022,): (f64,) = {
    if (!(scratch.values[2598] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1022;
        stamper.stamp_potential(
            self.branches[3],
            eq26_value,
            &[
            ],
        );
        let (eq27_e1030, eq27_e1030_d_n0, eq27_e1030_d_n1, eq27_e1030_d_n2, eq27_e1030_d_n3, eq27_e1030_d_n4, eq27_e1030_d_n5, eq27_e1030_d_n6, eq27_e1030_d_n7, eq27_e1030_d_n8, eq27_e1030_d_n9, eq27_e1030_d_n10, eq27_e1030_d_n11, eq27_e1030_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2599] != 0.0) {
        let eq27_e1026: f64 = (scratch.values[25] * scratch.values[846]);
        let eq27_e1026_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][0]));
        let eq27_e1026_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][1]));
        let eq27_e1026_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][2]));
        let eq27_e1026_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][3]));
        let eq27_e1026_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][4]));
        let eq27_e1026_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][5]));
        let eq27_e1026_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][6]));
        let eq27_e1026_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][7]));
        let eq27_e1026_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][8]));
        let eq27_e1026_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][9]));
        let eq27_e1026_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][10]));
        let eq27_e1026_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][11]));
        let eq27_e1026_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][12]));
        let eq27_e1028: f64 = (eq27_e1026 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n0: f64 = (eq27_e1026_d_n0 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n1: f64 = (eq27_e1026_d_n1 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n2: f64 = (eq27_e1026_d_n2 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n3: f64 = (eq27_e1026_d_n3 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n4: f64 = (eq27_e1026_d_n4 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n5: f64 = (eq27_e1026_d_n5 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n6: f64 = (eq27_e1026_d_n6 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n7: f64 = (eq27_e1026_d_n7 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n8: f64 = (eq27_e1026_d_n8 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n9: f64 = (eq27_e1026_d_n9 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        let eq27_e1028_d_n10: f64 = ((eq27_e1026_d_n10 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10]))) + (eq27_e1026 * -1.0));
        let eq27_e1028_d_n11: f64 = ((eq27_e1026_d_n11 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10]))) + eq27_e1026);
        let eq27_e1028_d_n12: f64 = (eq27_e1026_d_n12 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[10])));
        (eq27_e1028, eq27_e1028_d_n0, eq27_e1028_d_n1, eq27_e1028_d_n2, eq27_e1028_d_n3, eq27_e1028_d_n4, eq27_e1028_d_n5, eq27_e1028_d_n6, eq27_e1028_d_n7, eq27_e1028_d_n8, eq27_e1028_d_n9, eq27_e1028_d_n10, eq27_e1028_d_n11, eq27_e1028_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1030;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[10]),
            self.multiplicity * (eq27_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq27_e1030_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq27_e1030_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq27_e1030_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq27_e1030_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq27_e1030_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq27_e1030_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq27_e1030_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq27_e1030_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq27_e1030_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq27_e1030_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq27_e1030_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq27_e1030_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq27_e1030_d_n12),
            ],
        );
        let (eq28_e1038,): (f64,) = {
    if (scratch.values[2599] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1038;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[10]),
            self.multiplicity * (eq28_value),
            &[
            ],
        );
        let (eq29_e1043,): (f64,) = {
    if (!(scratch.values[2599] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1043;
        stamper.stamp_potential(
            self.branches[4],
            eq29_value,
            &[
            ],
        );
        let (eq30_e1051, eq30_e1051_d_n0, eq30_e1051_d_n1, eq30_e1051_d_n2, eq30_e1051_d_n3, eq30_e1051_d_n4, eq30_e1051_d_n5, eq30_e1051_d_n6, eq30_e1051_d_n7, eq30_e1051_d_n8, eq30_e1051_d_n9, eq30_e1051_d_n10, eq30_e1051_d_n11, eq30_e1051_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2600] != 0.0) {
        let eq30_e1047: f64 = (scratch.values[25] * scratch.values[847]);
        let eq30_e1047_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][0]));
        let eq30_e1047_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][1]));
        let eq30_e1047_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][2]));
        let eq30_e1047_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][3]));
        let eq30_e1047_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][4]));
        let eq30_e1047_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][5]));
        let eq30_e1047_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][6]));
        let eq30_e1047_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][7]));
        let eq30_e1047_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][8]));
        let eq30_e1047_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][9]));
        let eq30_e1047_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][10]));
        let eq30_e1047_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][11]));
        let eq30_e1047_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][12]));
        let eq30_e1049: f64 = (eq30_e1047 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n0: f64 = (eq30_e1047_d_n0 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n1: f64 = (eq30_e1047_d_n1 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n2: f64 = (eq30_e1047_d_n2 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n3: f64 = (eq30_e1047_d_n3 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n4: f64 = (eq30_e1047_d_n4 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n5: f64 = (eq30_e1047_d_n5 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n6: f64 = (eq30_e1047_d_n6 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n7: f64 = (eq30_e1047_d_n7 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n8: f64 = (eq30_e1047_d_n8 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n9: f64 = (eq30_e1047_d_n9 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n10: f64 = ((eq30_e1047_d_n10 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10]))) + (eq30_e1047 * -1.0));
        let eq30_e1049_d_n11: f64 = (eq30_e1047_d_n11 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10])));
        let eq30_e1049_d_n12: f64 = ((eq30_e1047_d_n12 * (ctx.node_voltage(self.nodes[12]) - ctx.node_voltage(self.nodes[10]))) + eq30_e1047);
        (eq30_e1049, eq30_e1049_d_n0, eq30_e1049_d_n1, eq30_e1049_d_n2, eq30_e1049_d_n3, eq30_e1049_d_n4, eq30_e1049_d_n5, eq30_e1049_d_n6, eq30_e1049_d_n7, eq30_e1049_d_n8, eq30_e1049_d_n9, eq30_e1049_d_n10, eq30_e1049_d_n11, eq30_e1049_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1051;
        stamper.stamp_current(
            Some(self.nodes[12]),
            Some(self.nodes[10]),
            self.multiplicity * (eq30_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq30_e1051_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq30_e1051_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq30_e1051_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq30_e1051_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq30_e1051_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq30_e1051_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq30_e1051_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq30_e1051_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq30_e1051_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq30_e1051_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq30_e1051_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq30_e1051_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq30_e1051_d_n12),
            ],
        );
        let (eq31_e1059,): (f64,) = {
    if (scratch.values[2600] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1059;
        stamper.stamp_current(
            Some(self.nodes[12]),
            Some(self.nodes[10]),
            self.multiplicity * (eq31_value),
            &[
            ],
        );
        let (eq32_e1064,): (f64,) = {
    if (!(scratch.values[2600] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1064;
        stamper.stamp_potential(
            self.branches[5],
            eq32_value,
            &[
            ],
        );
        let (eq33_e1072, eq33_e1072_d_n0, eq33_e1072_d_n1, eq33_e1072_d_n2, eq33_e1072_d_n3, eq33_e1072_d_n4, eq33_e1072_d_n5, eq33_e1072_d_n6, eq33_e1072_d_n7, eq33_e1072_d_n8, eq33_e1072_d_n9, eq33_e1072_d_n10, eq33_e1072_d_n11, eq33_e1072_d_n12,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2601] != 0.0) {
        let eq33_e1068: f64 = (scratch.values[25] * scratch.values[848]);
        let eq33_e1068_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][0]));
        let eq33_e1068_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][1]));
        let eq33_e1068_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][2]));
        let eq33_e1068_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][3]));
        let eq33_e1068_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][4]));
        let eq33_e1068_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][5]));
        let eq33_e1068_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][6]));
        let eq33_e1068_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][7]));
        let eq33_e1068_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][8]));
        let eq33_e1068_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][9]));
        let eq33_e1068_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][10]));
        let eq33_e1068_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][11]));
        let eq33_e1068_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][12]));
        let eq33_e1070: f64 = (eq33_e1068 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n0: f64 = (eq33_e1068_d_n0 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n1: f64 = (eq33_e1068_d_n1 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n2: f64 = (eq33_e1068_d_n2 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n3: f64 = ((eq33_e1068_d_n3 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10]))) + eq33_e1068);
        let eq33_e1070_d_n4: f64 = (eq33_e1068_d_n4 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n5: f64 = (eq33_e1068_d_n5 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n6: f64 = (eq33_e1068_d_n6 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n7: f64 = (eq33_e1068_d_n7 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n8: f64 = (eq33_e1068_d_n8 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n9: f64 = (eq33_e1068_d_n9 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n10: f64 = ((eq33_e1068_d_n10 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10]))) + (eq33_e1068 * -1.0));
        let eq33_e1070_d_n11: f64 = (eq33_e1068_d_n11 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        let eq33_e1070_d_n12: f64 = (eq33_e1068_d_n12 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[10])));
        (eq33_e1070, eq33_e1070_d_n0, eq33_e1070_d_n1, eq33_e1070_d_n2, eq33_e1070_d_n3, eq33_e1070_d_n4, eq33_e1070_d_n5, eq33_e1070_d_n6, eq33_e1070_d_n7, eq33_e1070_d_n8, eq33_e1070_d_n9, eq33_e1070_d_n10, eq33_e1070_d_n11, eq33_e1070_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1072;
        stamper.stamp_current(
            Some(self.nodes[3]),
            Some(self.nodes[10]),
            self.multiplicity * (eq33_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq33_e1072_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq33_e1072_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq33_e1072_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq33_e1072_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq33_e1072_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq33_e1072_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq33_e1072_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq33_e1072_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq33_e1072_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq33_e1072_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq33_e1072_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq33_e1072_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq33_e1072_d_n12),
            ],
        );
        let (eq34_e1080,): (f64,) = {
    if (scratch.values[2601] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1080;
        stamper.stamp_current(
            Some(self.nodes[3]),
            Some(self.nodes[10]),
            self.multiplicity * (eq34_value),
            &[
            ],
        );
        let (eq35_e1085,): (f64,) = {
    if (!(scratch.values[2601] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1085;
        stamper.stamp_potential(
            self.branches[6],
            eq35_value,
            &[
            ],
        );
        let eq36_e1088: f64 = (1e-15 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[7])));
        let eq36_e1088_d_n7: f64 = (1e-15 * -1.0);
        let eq36_e1088_d_n8: f64 = 1e-15;
        let eq36_value: f64 = eq36_e1088;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq36_value),
            &[
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq36_e1088_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq36_e1088_d_n8),
            ],
        );
        let eq37_e1090: f64 = (-scratch.values[25]);
        let eq37_e1090_d_n0: f64 = (-scratch.node_derivatives[25][0]);
        let eq37_e1090_d_n1: f64 = (-scratch.node_derivatives[25][1]);
        let eq37_e1090_d_n2: f64 = (-scratch.node_derivatives[25][2]);
        let eq37_e1090_d_n3: f64 = (-scratch.node_derivatives[25][3]);
        let eq37_e1090_d_n4: f64 = (-scratch.node_derivatives[25][4]);
        let eq37_e1090_d_n5: f64 = (-scratch.node_derivatives[25][5]);
        let eq37_e1090_d_n6: f64 = (-scratch.node_derivatives[25][6]);
        let eq37_e1090_d_n7: f64 = (-scratch.node_derivatives[25][7]);
        let eq37_e1090_d_n8: f64 = (-scratch.node_derivatives[25][8]);
        let eq37_e1090_d_n9: f64 = (-scratch.node_derivatives[25][9]);
        let eq37_e1090_d_n10: f64 = (-scratch.node_derivatives[25][10]);
        let eq37_e1090_d_n11: f64 = (-scratch.node_derivatives[25][11]);
        let eq37_e1090_d_n12: f64 = (-scratch.node_derivatives[25][12]);
        let eq37_e1092: f64 = (eq37_e1090 * scratch.values[2073]);
        let eq37_e1092_d_n0: f64 = ((eq37_e1090_d_n0 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][0]));
        let eq37_e1092_d_n1: f64 = ((eq37_e1090_d_n1 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][1]));
        let eq37_e1092_d_n2: f64 = ((eq37_e1090_d_n2 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][2]));
        let eq37_e1092_d_n3: f64 = ((eq37_e1090_d_n3 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][3]));
        let eq37_e1092_d_n4: f64 = ((eq37_e1090_d_n4 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][4]));
        let eq37_e1092_d_n5: f64 = ((eq37_e1090_d_n5 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][5]));
        let eq37_e1092_d_n6: f64 = ((eq37_e1090_d_n6 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][6]));
        let eq37_e1092_d_n7: f64 = ((eq37_e1090_d_n7 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][7]));
        let eq37_e1092_d_n8: f64 = ((eq37_e1090_d_n8 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][8]));
        let eq37_e1092_d_n9: f64 = ((eq37_e1090_d_n9 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][9]));
        let eq37_e1092_d_n10: f64 = ((eq37_e1090_d_n10 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][10]));
        let eq37_e1092_d_n11: f64 = ((eq37_e1090_d_n11 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][11]));
        let eq37_e1092_d_n12: f64 = ((eq37_e1090_d_n12 * scratch.values[2073]) + (eq37_e1090 * scratch.node_derivatives[2073][12]));
        let eq37_value: f64 = eq37_e1092;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq37_e1092_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq37_e1092_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq37_e1092_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq37_e1092_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq37_e1092_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq37_e1092_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq37_e1092_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq37_e1092_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq37_e1092_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq37_e1092_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq37_e1092_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq37_e1092_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq37_e1092_d_n12),
            ],
        );
        let eq38_e1095: f64 = (scratch.values[25] * scratch.values[307]);
        let eq38_e1095_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][0]));
        let eq38_e1095_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][1]));
        let eq38_e1095_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][2]));
        let eq38_e1095_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][3]));
        let eq38_e1095_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][4]));
        let eq38_e1095_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][5]));
        let eq38_e1095_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][6]));
        let eq38_e1095_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][7]));
        let eq38_e1095_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][8]));
        let eq38_e1095_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][9]));
        let eq38_e1095_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][10]));
        let eq38_e1095_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][11]));
        let eq38_e1095_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][12]));
        let eq38_e1097: f64 = (eq38_e1095 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n0: f64 = (eq38_e1095_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n1: f64 = (eq38_e1095_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n2: f64 = (eq38_e1095_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n3: f64 = (eq38_e1095_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n4: f64 = ((eq38_e1095_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq38_e1095);
        let eq38_e1097_d_n5: f64 = (eq38_e1095_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n6: f64 = (eq38_e1095_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n7: f64 = (eq38_e1095_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n8: f64 = (eq38_e1095_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n9: f64 = (eq38_e1095_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n10: f64 = (eq38_e1095_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n11: f64 = (eq38_e1095_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n12: f64 = (eq38_e1095_d_n12 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1098: f64 = self.eval_ddt(0, eq38_e1097);
        let eq38_e1098_d_n0: f64 = self.ddt_jacobian(eq38_e1097_d_n0);
        let eq38_e1098_d_n1: f64 = self.ddt_jacobian(eq38_e1097_d_n1);
        let eq38_e1098_d_n2: f64 = self.ddt_jacobian(eq38_e1097_d_n2);
        let eq38_e1098_d_n3: f64 = self.ddt_jacobian(eq38_e1097_d_n3);
        let eq38_e1098_d_n4: f64 = self.ddt_jacobian(eq38_e1097_d_n4);
        let eq38_e1098_d_n5: f64 = self.ddt_jacobian(eq38_e1097_d_n5);
        let eq38_e1098_d_n6: f64 = self.ddt_jacobian(eq38_e1097_d_n6);
        let eq38_e1098_d_n7: f64 = self.ddt_jacobian(eq38_e1097_d_n7);
        let eq38_e1098_d_n8: f64 = self.ddt_jacobian(eq38_e1097_d_n8);
        let eq38_e1098_d_n9: f64 = self.ddt_jacobian(eq38_e1097_d_n9);
        let eq38_e1098_d_n10: f64 = self.ddt_jacobian(eq38_e1097_d_n10);
        let eq38_e1098_d_n11: f64 = self.ddt_jacobian(eq38_e1097_d_n11);
        let eq38_e1098_d_n12: f64 = self.ddt_jacobian(eq38_e1097_d_n12);
        let eq38_value: f64 = eq38_e1098;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq38_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq38_e1098_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq38_e1098_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq38_e1098_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq38_e1098_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq38_e1098_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq38_e1098_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq38_e1098_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq38_e1098_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq38_e1098_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq38_e1098_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq38_e1098_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq38_e1098_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq38_e1098_d_n12),
            ],
        );
        let eq39_e1101: f64 = (scratch.values[25] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n0: f64 = (scratch.node_derivatives[25][0] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n1: f64 = (scratch.node_derivatives[25][1] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n2: f64 = (scratch.node_derivatives[25][2] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n3: f64 = (scratch.node_derivatives[25][3] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n4: f64 = ((scratch.node_derivatives[25][4] * (ctx.node_voltage(self.nodes[4]) - 0.0)) + scratch.values[25]);
        let eq39_e1101_d_n5: f64 = (scratch.node_derivatives[25][5] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n6: f64 = (scratch.node_derivatives[25][6] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n7: f64 = (scratch.node_derivatives[25][7] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n8: f64 = (scratch.node_derivatives[25][8] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n9: f64 = (scratch.node_derivatives[25][9] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n10: f64 = (scratch.node_derivatives[25][10] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n11: f64 = (scratch.node_derivatives[25][11] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1101_d_n12: f64 = (scratch.node_derivatives[25][12] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq39_e1103: f64 = (eq39_e1101 / scratch.values[766]);
        let eq39_e1103_d_n0: f64 = (((eq39_e1101_d_n0 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][0])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n1: f64 = (((eq39_e1101_d_n1 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][1])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n2: f64 = (((eq39_e1101_d_n2 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][2])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n3: f64 = (((eq39_e1101_d_n3 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][3])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n4: f64 = (((eq39_e1101_d_n4 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][4])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n5: f64 = (((eq39_e1101_d_n5 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][5])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n6: f64 = (((eq39_e1101_d_n6 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][6])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n7: f64 = (((eq39_e1101_d_n7 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][7])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n8: f64 = (((eq39_e1101_d_n8 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][8])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n9: f64 = (((eq39_e1101_d_n9 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][9])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n10: f64 = (((eq39_e1101_d_n10 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][10])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n11: f64 = (((eq39_e1101_d_n11 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][11])) / (scratch.values[766] * scratch.values[766]));
        let eq39_e1103_d_n12: f64 = (((eq39_e1101_d_n12 * scratch.values[766]) - (eq39_e1101 * scratch.node_derivatives[766][12])) / (scratch.values[766] * scratch.values[766]));
        let eq39_value: f64 = eq39_e1103;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq39_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq39_e1103_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq39_e1103_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq39_e1103_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq39_e1103_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq39_e1103_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq39_e1103_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq39_e1103_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq39_e1103_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq39_e1103_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq39_e1103_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq39_e1103_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq39_e1103_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq39_e1103_d_n12),
            ],
        );
        let eq40_e1106: f64 = (scratch.values[0] * scratch.values[25]);
        let eq40_e1106_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq40_e1106_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq40_e1106_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq40_e1106_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq40_e1106_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq40_e1106_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq40_e1106_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq40_e1106_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq40_e1106_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq40_e1106_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq40_e1106_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq40_e1106_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq40_e1106_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq40_e1108: f64 = (eq40_e1106 * scratch.values[940]);
        let eq40_e1108_d_n0: f64 = ((eq40_e1106_d_n0 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][0]));
        let eq40_e1108_d_n1: f64 = ((eq40_e1106_d_n1 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][1]));
        let eq40_e1108_d_n2: f64 = ((eq40_e1106_d_n2 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][2]));
        let eq40_e1108_d_n3: f64 = ((eq40_e1106_d_n3 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][3]));
        let eq40_e1108_d_n4: f64 = ((eq40_e1106_d_n4 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][4]));
        let eq40_e1108_d_n5: f64 = ((eq40_e1106_d_n5 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][5]));
        let eq40_e1108_d_n6: f64 = ((eq40_e1106_d_n6 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][6]));
        let eq40_e1108_d_n7: f64 = ((eq40_e1106_d_n7 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][7]));
        let eq40_e1108_d_n8: f64 = ((eq40_e1106_d_n8 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][8]));
        let eq40_e1108_d_n9: f64 = ((eq40_e1106_d_n9 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][9]));
        let eq40_e1108_d_n10: f64 = ((eq40_e1106_d_n10 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][10]));
        let eq40_e1108_d_n11: f64 = ((eq40_e1106_d_n11 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][11]));
        let eq40_e1108_d_n12: f64 = ((eq40_e1106_d_n12 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][12]));
        let eq40_e1109: f64 = self.eval_ddt(1, eq40_e1108);
        let eq40_e1109_d_n0: f64 = self.ddt_jacobian(eq40_e1108_d_n0);
        let eq40_e1109_d_n1: f64 = self.ddt_jacobian(eq40_e1108_d_n1);
        let eq40_e1109_d_n2: f64 = self.ddt_jacobian(eq40_e1108_d_n2);
        let eq40_e1109_d_n3: f64 = self.ddt_jacobian(eq40_e1108_d_n3);
        let eq40_e1109_d_n4: f64 = self.ddt_jacobian(eq40_e1108_d_n4);
        let eq40_e1109_d_n5: f64 = self.ddt_jacobian(eq40_e1108_d_n5);
        let eq40_e1109_d_n6: f64 = self.ddt_jacobian(eq40_e1108_d_n6);
        let eq40_e1109_d_n7: f64 = self.ddt_jacobian(eq40_e1108_d_n7);
        let eq40_e1109_d_n8: f64 = self.ddt_jacobian(eq40_e1108_d_n8);
        let eq40_e1109_d_n9: f64 = self.ddt_jacobian(eq40_e1108_d_n9);
        let eq40_e1109_d_n10: f64 = self.ddt_jacobian(eq40_e1108_d_n10);
        let eq40_e1109_d_n11: f64 = self.ddt_jacobian(eq40_e1108_d_n11);
        let eq40_e1109_d_n12: f64 = self.ddt_jacobian(eq40_e1108_d_n12);
        let eq40_value: f64 = eq40_e1109;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq40_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq40_e1109_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq40_e1109_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq40_e1109_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq40_e1109_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq40_e1109_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq40_e1109_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq40_e1109_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq40_e1109_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq40_e1109_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq40_e1109_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq40_e1109_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq40_e1109_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq40_e1109_d_n12),
            ],
        );
        let eq41_e1112: f64 = (scratch.values[0] * scratch.values[25]);
        let eq41_e1112_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq41_e1112_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq41_e1112_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq41_e1112_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq41_e1112_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq41_e1112_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq41_e1112_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq41_e1112_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq41_e1112_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq41_e1112_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq41_e1112_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq41_e1112_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq41_e1112_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq41_e1114: f64 = (eq41_e1112 * scratch.values[942]);
        let eq41_e1114_d_n0: f64 = ((eq41_e1112_d_n0 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][0]));
        let eq41_e1114_d_n1: f64 = ((eq41_e1112_d_n1 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][1]));
        let eq41_e1114_d_n2: f64 = ((eq41_e1112_d_n2 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][2]));
        let eq41_e1114_d_n3: f64 = ((eq41_e1112_d_n3 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][3]));
        let eq41_e1114_d_n4: f64 = ((eq41_e1112_d_n4 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][4]));
        let eq41_e1114_d_n5: f64 = ((eq41_e1112_d_n5 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][5]));
        let eq41_e1114_d_n6: f64 = ((eq41_e1112_d_n6 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][6]));
        let eq41_e1114_d_n7: f64 = ((eq41_e1112_d_n7 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][7]));
        let eq41_e1114_d_n8: f64 = ((eq41_e1112_d_n8 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][8]));
        let eq41_e1114_d_n9: f64 = ((eq41_e1112_d_n9 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][9]));
        let eq41_e1114_d_n10: f64 = ((eq41_e1112_d_n10 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][10]));
        let eq41_e1114_d_n11: f64 = ((eq41_e1112_d_n11 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][11]));
        let eq41_e1114_d_n12: f64 = ((eq41_e1112_d_n12 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][12]));
        let eq41_e1115: f64 = self.eval_ddt(2, eq41_e1114);
        let eq41_e1115_d_n0: f64 = self.ddt_jacobian(eq41_e1114_d_n0);
        let eq41_e1115_d_n1: f64 = self.ddt_jacobian(eq41_e1114_d_n1);
        let eq41_e1115_d_n2: f64 = self.ddt_jacobian(eq41_e1114_d_n2);
        let eq41_e1115_d_n3: f64 = self.ddt_jacobian(eq41_e1114_d_n3);
        let eq41_e1115_d_n4: f64 = self.ddt_jacobian(eq41_e1114_d_n4);
        let eq41_e1115_d_n5: f64 = self.ddt_jacobian(eq41_e1114_d_n5);
        let eq41_e1115_d_n6: f64 = self.ddt_jacobian(eq41_e1114_d_n6);
        let eq41_e1115_d_n7: f64 = self.ddt_jacobian(eq41_e1114_d_n7);
        let eq41_e1115_d_n8: f64 = self.ddt_jacobian(eq41_e1114_d_n8);
        let eq41_e1115_d_n9: f64 = self.ddt_jacobian(eq41_e1114_d_n9);
        let eq41_e1115_d_n10: f64 = self.ddt_jacobian(eq41_e1114_d_n10);
        let eq41_e1115_d_n11: f64 = self.ddt_jacobian(eq41_e1114_d_n11);
        let eq41_e1115_d_n12: f64 = self.ddt_jacobian(eq41_e1114_d_n12);
        let eq41_value: f64 = eq41_e1115;
        stamper.stamp_current(
            Some(self.nodes[9]),
            Some(self.nodes[7]),
            self.multiplicity * (eq41_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq41_e1115_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq41_e1115_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq41_e1115_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq41_e1115_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq41_e1115_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq41_e1115_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq41_e1115_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq41_e1115_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq41_e1115_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq41_e1115_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq41_e1115_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq41_e1115_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq41_e1115_d_n12),
            ],
        );
        let eq42_e1118: f64 = (scratch.values[0] * scratch.values[25]);
        let eq42_e1118_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq42_e1118_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq42_e1118_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq42_e1118_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq42_e1118_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq42_e1118_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq42_e1118_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq42_e1118_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq42_e1118_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq42_e1118_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq42_e1118_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq42_e1118_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq42_e1118_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq42_e1120: f64 = (eq42_e1118 * scratch.values[941]);
        let eq42_e1120_d_n0: f64 = ((eq42_e1118_d_n0 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][0]));
        let eq42_e1120_d_n1: f64 = ((eq42_e1118_d_n1 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][1]));
        let eq42_e1120_d_n2: f64 = ((eq42_e1118_d_n2 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][2]));
        let eq42_e1120_d_n3: f64 = ((eq42_e1118_d_n3 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][3]));
        let eq42_e1120_d_n4: f64 = ((eq42_e1118_d_n4 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][4]));
        let eq42_e1120_d_n5: f64 = ((eq42_e1118_d_n5 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][5]));
        let eq42_e1120_d_n6: f64 = ((eq42_e1118_d_n6 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][6]));
        let eq42_e1120_d_n7: f64 = ((eq42_e1118_d_n7 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][7]));
        let eq42_e1120_d_n8: f64 = ((eq42_e1118_d_n8 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][8]));
        let eq42_e1120_d_n9: f64 = ((eq42_e1118_d_n9 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][9]));
        let eq42_e1120_d_n10: f64 = ((eq42_e1118_d_n10 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][10]));
        let eq42_e1120_d_n11: f64 = ((eq42_e1118_d_n11 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][11]));
        let eq42_e1120_d_n12: f64 = ((eq42_e1118_d_n12 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][12]));
        let eq42_e1121: f64 = self.eval_ddt(3, eq42_e1120);
        let eq42_e1121_d_n0: f64 = self.ddt_jacobian(eq42_e1120_d_n0);
        let eq42_e1121_d_n1: f64 = self.ddt_jacobian(eq42_e1120_d_n1);
        let eq42_e1121_d_n2: f64 = self.ddt_jacobian(eq42_e1120_d_n2);
        let eq42_e1121_d_n3: f64 = self.ddt_jacobian(eq42_e1120_d_n3);
        let eq42_e1121_d_n4: f64 = self.ddt_jacobian(eq42_e1120_d_n4);
        let eq42_e1121_d_n5: f64 = self.ddt_jacobian(eq42_e1120_d_n5);
        let eq42_e1121_d_n6: f64 = self.ddt_jacobian(eq42_e1120_d_n6);
        let eq42_e1121_d_n7: f64 = self.ddt_jacobian(eq42_e1120_d_n7);
        let eq42_e1121_d_n8: f64 = self.ddt_jacobian(eq42_e1120_d_n8);
        let eq42_e1121_d_n9: f64 = self.ddt_jacobian(eq42_e1120_d_n9);
        let eq42_e1121_d_n10: f64 = self.ddt_jacobian(eq42_e1120_d_n10);
        let eq42_e1121_d_n11: f64 = self.ddt_jacobian(eq42_e1120_d_n11);
        let eq42_e1121_d_n12: f64 = self.ddt_jacobian(eq42_e1120_d_n12);
        let eq42_value: f64 = eq42_e1121;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq42_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq42_e1121_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq42_e1121_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq42_e1121_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq42_e1121_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq42_e1121_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq42_e1121_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq42_e1121_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq42_e1121_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq42_e1121_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq42_e1121_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq42_e1121_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq42_e1121_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq42_e1121_d_n12),
            ],
        );
        let eq43_e1124: f64 = (scratch.values[0] * scratch.values[25]);
        let eq43_e1124_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq43_e1124_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq43_e1124_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq43_e1124_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq43_e1124_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq43_e1124_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq43_e1124_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq43_e1124_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq43_e1124_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq43_e1124_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq43_e1124_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq43_e1124_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq43_e1124_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq43_e1126: f64 = (eq43_e1124 * scratch.values[947]);
        let eq43_e1126_d_n0: f64 = ((eq43_e1124_d_n0 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][0]));
        let eq43_e1126_d_n1: f64 = ((eq43_e1124_d_n1 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][1]));
        let eq43_e1126_d_n2: f64 = ((eq43_e1124_d_n2 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][2]));
        let eq43_e1126_d_n3: f64 = ((eq43_e1124_d_n3 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][3]));
        let eq43_e1126_d_n4: f64 = ((eq43_e1124_d_n4 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][4]));
        let eq43_e1126_d_n5: f64 = ((eq43_e1124_d_n5 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][5]));
        let eq43_e1126_d_n6: f64 = ((eq43_e1124_d_n6 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][6]));
        let eq43_e1126_d_n7: f64 = ((eq43_e1124_d_n7 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][7]));
        let eq43_e1126_d_n8: f64 = ((eq43_e1124_d_n8 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][8]));
        let eq43_e1126_d_n9: f64 = ((eq43_e1124_d_n9 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][9]));
        let eq43_e1126_d_n10: f64 = ((eq43_e1124_d_n10 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][10]));
        let eq43_e1126_d_n11: f64 = ((eq43_e1124_d_n11 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][11]));
        let eq43_e1126_d_n12: f64 = ((eq43_e1124_d_n12 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][12]));
        let eq43_e1127: f64 = self.eval_ddt(4, eq43_e1126);
        let eq43_e1127_d_n0: f64 = self.ddt_jacobian(eq43_e1126_d_n0);
        let eq43_e1127_d_n1: f64 = self.ddt_jacobian(eq43_e1126_d_n1);
        let eq43_e1127_d_n2: f64 = self.ddt_jacobian(eq43_e1126_d_n2);
        let eq43_e1127_d_n3: f64 = self.ddt_jacobian(eq43_e1126_d_n3);
        let eq43_e1127_d_n4: f64 = self.ddt_jacobian(eq43_e1126_d_n4);
        let eq43_e1127_d_n5: f64 = self.ddt_jacobian(eq43_e1126_d_n5);
        let eq43_e1127_d_n6: f64 = self.ddt_jacobian(eq43_e1126_d_n6);
        let eq43_e1127_d_n7: f64 = self.ddt_jacobian(eq43_e1126_d_n7);
        let eq43_e1127_d_n8: f64 = self.ddt_jacobian(eq43_e1126_d_n8);
        let eq43_e1127_d_n9: f64 = self.ddt_jacobian(eq43_e1126_d_n9);
        let eq43_e1127_d_n10: f64 = self.ddt_jacobian(eq43_e1126_d_n10);
        let eq43_e1127_d_n11: f64 = self.ddt_jacobian(eq43_e1126_d_n11);
        let eq43_e1127_d_n12: f64 = self.ddt_jacobian(eq43_e1126_d_n12);
        let eq43_value: f64 = eq43_e1127;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq43_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq43_e1127_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq43_e1127_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq43_e1127_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq43_e1127_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq43_e1127_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq43_e1127_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq43_e1127_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq43_e1127_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq43_e1127_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq43_e1127_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq43_e1127_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq43_e1127_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq43_e1127_d_n12),
            ],
        );
        let eq44_e1130: f64 = (scratch.values[0] * scratch.values[25]);
        let eq44_e1130_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq44_e1130_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq44_e1130_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq44_e1130_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq44_e1130_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq44_e1130_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq44_e1130_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq44_e1130_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq44_e1130_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq44_e1130_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq44_e1130_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq44_e1130_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq44_e1130_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq44_e1132: f64 = (eq44_e1130 * scratch.values[948]);
        let eq44_e1132_d_n0: f64 = ((eq44_e1130_d_n0 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][0]));
        let eq44_e1132_d_n1: f64 = ((eq44_e1130_d_n1 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][1]));
        let eq44_e1132_d_n2: f64 = ((eq44_e1130_d_n2 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][2]));
        let eq44_e1132_d_n3: f64 = ((eq44_e1130_d_n3 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][3]));
        let eq44_e1132_d_n4: f64 = ((eq44_e1130_d_n4 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][4]));
        let eq44_e1132_d_n5: f64 = ((eq44_e1130_d_n5 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][5]));
        let eq44_e1132_d_n6: f64 = ((eq44_e1130_d_n6 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][6]));
        let eq44_e1132_d_n7: f64 = ((eq44_e1130_d_n7 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][7]));
        let eq44_e1132_d_n8: f64 = ((eq44_e1130_d_n8 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][8]));
        let eq44_e1132_d_n9: f64 = ((eq44_e1130_d_n9 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][9]));
        let eq44_e1132_d_n10: f64 = ((eq44_e1130_d_n10 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][10]));
        let eq44_e1132_d_n11: f64 = ((eq44_e1130_d_n11 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][11]));
        let eq44_e1132_d_n12: f64 = ((eq44_e1130_d_n12 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][12]));
        let eq44_e1133: f64 = self.eval_ddt(5, eq44_e1132);
        let eq44_e1133_d_n0: f64 = self.ddt_jacobian(eq44_e1132_d_n0);
        let eq44_e1133_d_n1: f64 = self.ddt_jacobian(eq44_e1132_d_n1);
        let eq44_e1133_d_n2: f64 = self.ddt_jacobian(eq44_e1132_d_n2);
        let eq44_e1133_d_n3: f64 = self.ddt_jacobian(eq44_e1132_d_n3);
        let eq44_e1133_d_n4: f64 = self.ddt_jacobian(eq44_e1132_d_n4);
        let eq44_e1133_d_n5: f64 = self.ddt_jacobian(eq44_e1132_d_n5);
        let eq44_e1133_d_n6: f64 = self.ddt_jacobian(eq44_e1132_d_n6);
        let eq44_e1133_d_n7: f64 = self.ddt_jacobian(eq44_e1132_d_n7);
        let eq44_e1133_d_n8: f64 = self.ddt_jacobian(eq44_e1132_d_n8);
        let eq44_e1133_d_n9: f64 = self.ddt_jacobian(eq44_e1132_d_n9);
        let eq44_e1133_d_n10: f64 = self.ddt_jacobian(eq44_e1132_d_n10);
        let eq44_e1133_d_n11: f64 = self.ddt_jacobian(eq44_e1132_d_n11);
        let eq44_e1133_d_n12: f64 = self.ddt_jacobian(eq44_e1132_d_n12);
        let eq44_value: f64 = eq44_e1133;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq44_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq44_e1133_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq44_e1133_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq44_e1133_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq44_e1133_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq44_e1133_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq44_e1133_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq44_e1133_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq44_e1133_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq44_e1133_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq44_e1133_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq44_e1133_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq44_e1133_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq44_e1133_d_n12),
            ],
        );
        let eq45_e1136: f64 = (scratch.values[0] * scratch.values[25]);
        let eq45_e1136_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq45_e1136_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq45_e1136_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq45_e1136_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq45_e1136_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq45_e1136_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq45_e1136_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq45_e1136_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq45_e1136_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq45_e1136_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq45_e1136_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq45_e1136_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq45_e1136_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq45_e1138: f64 = (eq45_e1136 * scratch.values[946]);
        let eq45_e1138_d_n0: f64 = ((eq45_e1136_d_n0 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][0]));
        let eq45_e1138_d_n1: f64 = ((eq45_e1136_d_n1 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][1]));
        let eq45_e1138_d_n2: f64 = ((eq45_e1136_d_n2 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][2]));
        let eq45_e1138_d_n3: f64 = ((eq45_e1136_d_n3 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][3]));
        let eq45_e1138_d_n4: f64 = ((eq45_e1136_d_n4 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][4]));
        let eq45_e1138_d_n5: f64 = ((eq45_e1136_d_n5 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][5]));
        let eq45_e1138_d_n6: f64 = ((eq45_e1136_d_n6 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][6]));
        let eq45_e1138_d_n7: f64 = ((eq45_e1136_d_n7 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][7]));
        let eq45_e1138_d_n8: f64 = ((eq45_e1136_d_n8 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][8]));
        let eq45_e1138_d_n9: f64 = ((eq45_e1136_d_n9 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][9]));
        let eq45_e1138_d_n10: f64 = ((eq45_e1136_d_n10 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][10]));
        let eq45_e1138_d_n11: f64 = ((eq45_e1136_d_n11 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][11]));
        let eq45_e1138_d_n12: f64 = ((eq45_e1136_d_n12 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][12]));
        let eq45_e1139: f64 = self.eval_ddt(6, eq45_e1138);
        let eq45_e1139_d_n0: f64 = self.ddt_jacobian(eq45_e1138_d_n0);
        let eq45_e1139_d_n1: f64 = self.ddt_jacobian(eq45_e1138_d_n1);
        let eq45_e1139_d_n2: f64 = self.ddt_jacobian(eq45_e1138_d_n2);
        let eq45_e1139_d_n3: f64 = self.ddt_jacobian(eq45_e1138_d_n3);
        let eq45_e1139_d_n4: f64 = self.ddt_jacobian(eq45_e1138_d_n4);
        let eq45_e1139_d_n5: f64 = self.ddt_jacobian(eq45_e1138_d_n5);
        let eq45_e1139_d_n6: f64 = self.ddt_jacobian(eq45_e1138_d_n6);
        let eq45_e1139_d_n7: f64 = self.ddt_jacobian(eq45_e1138_d_n7);
        let eq45_e1139_d_n8: f64 = self.ddt_jacobian(eq45_e1138_d_n8);
        let eq45_e1139_d_n9: f64 = self.ddt_jacobian(eq45_e1138_d_n9);
        let eq45_e1139_d_n10: f64 = self.ddt_jacobian(eq45_e1138_d_n10);
        let eq45_e1139_d_n11: f64 = self.ddt_jacobian(eq45_e1138_d_n11);
        let eq45_e1139_d_n12: f64 = self.ddt_jacobian(eq45_e1138_d_n12);
        let eq45_value: f64 = eq45_e1139;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[9]),
            self.multiplicity * (eq45_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq45_e1139_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq45_e1139_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq45_e1139_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq45_e1139_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq45_e1139_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq45_e1139_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq45_e1139_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq45_e1139_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq45_e1139_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq45_e1139_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq45_e1139_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq45_e1139_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq45_e1139_d_n12),
            ],
        );
        let eq46_e1142: f64 = (scratch.values[0] * scratch.values[25]);
        let eq46_e1142_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq46_e1142_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq46_e1142_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq46_e1142_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq46_e1142_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq46_e1142_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq46_e1142_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq46_e1142_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq46_e1142_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq46_e1142_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq46_e1142_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq46_e1142_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq46_e1142_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq46_e1144: f64 = (eq46_e1142 * scratch.values[2038]);
        let eq46_e1144_d_n0: f64 = ((eq46_e1142_d_n0 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][0]));
        let eq46_e1144_d_n1: f64 = ((eq46_e1142_d_n1 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][1]));
        let eq46_e1144_d_n2: f64 = ((eq46_e1142_d_n2 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][2]));
        let eq46_e1144_d_n3: f64 = ((eq46_e1142_d_n3 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][3]));
        let eq46_e1144_d_n4: f64 = ((eq46_e1142_d_n4 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][4]));
        let eq46_e1144_d_n5: f64 = ((eq46_e1142_d_n5 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][5]));
        let eq46_e1144_d_n6: f64 = ((eq46_e1142_d_n6 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][6]));
        let eq46_e1144_d_n7: f64 = ((eq46_e1142_d_n7 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][7]));
        let eq46_e1144_d_n8: f64 = ((eq46_e1142_d_n8 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][8]));
        let eq46_e1144_d_n9: f64 = ((eq46_e1142_d_n9 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][9]));
        let eq46_e1144_d_n10: f64 = ((eq46_e1142_d_n10 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][10]));
        let eq46_e1144_d_n11: f64 = ((eq46_e1142_d_n11 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][11]));
        let eq46_e1144_d_n12: f64 = ((eq46_e1142_d_n12 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][12]));
        let eq46_e1145: f64 = self.eval_ddt(7, eq46_e1144);
        let eq46_e1145_d_n0: f64 = self.ddt_jacobian(eq46_e1144_d_n0);
        let eq46_e1145_d_n1: f64 = self.ddt_jacobian(eq46_e1144_d_n1);
        let eq46_e1145_d_n2: f64 = self.ddt_jacobian(eq46_e1144_d_n2);
        let eq46_e1145_d_n3: f64 = self.ddt_jacobian(eq46_e1144_d_n3);
        let eq46_e1145_d_n4: f64 = self.ddt_jacobian(eq46_e1144_d_n4);
        let eq46_e1145_d_n5: f64 = self.ddt_jacobian(eq46_e1144_d_n5);
        let eq46_e1145_d_n6: f64 = self.ddt_jacobian(eq46_e1144_d_n6);
        let eq46_e1145_d_n7: f64 = self.ddt_jacobian(eq46_e1144_d_n7);
        let eq46_e1145_d_n8: f64 = self.ddt_jacobian(eq46_e1144_d_n8);
        let eq46_e1145_d_n9: f64 = self.ddt_jacobian(eq46_e1144_d_n9);
        let eq46_e1145_d_n10: f64 = self.ddt_jacobian(eq46_e1144_d_n10);
        let eq46_e1145_d_n11: f64 = self.ddt_jacobian(eq46_e1144_d_n11);
        let eq46_e1145_d_n12: f64 = self.ddt_jacobian(eq46_e1144_d_n12);
        let eq46_value: f64 = eq46_e1145;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq46_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq46_e1145_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq46_e1145_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq46_e1145_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq46_e1145_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq46_e1145_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq46_e1145_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq46_e1145_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq46_e1145_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq46_e1145_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq46_e1145_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq46_e1145_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq46_e1145_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq46_e1145_d_n12),
            ],
        );
        let eq47_e1148: f64 = (scratch.values[0] * scratch.values[25]);
        let eq47_e1148_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq47_e1148_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq47_e1148_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq47_e1148_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq47_e1148_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq47_e1148_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq47_e1148_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq47_e1148_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq47_e1148_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq47_e1148_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq47_e1148_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq47_e1148_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq47_e1148_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq47_e1150: f64 = (eq47_e1148 * scratch.values[2042]);
        let eq47_e1150_d_n0: f64 = ((eq47_e1148_d_n0 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][0]));
        let eq47_e1150_d_n1: f64 = ((eq47_e1148_d_n1 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][1]));
        let eq47_e1150_d_n2: f64 = ((eq47_e1148_d_n2 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][2]));
        let eq47_e1150_d_n3: f64 = ((eq47_e1148_d_n3 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][3]));
        let eq47_e1150_d_n4: f64 = ((eq47_e1148_d_n4 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][4]));
        let eq47_e1150_d_n5: f64 = ((eq47_e1148_d_n5 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][5]));
        let eq47_e1150_d_n6: f64 = ((eq47_e1148_d_n6 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][6]));
        let eq47_e1150_d_n7: f64 = ((eq47_e1148_d_n7 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][7]));
        let eq47_e1150_d_n8: f64 = ((eq47_e1148_d_n8 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][8]));
        let eq47_e1150_d_n9: f64 = ((eq47_e1148_d_n9 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][9]));
        let eq47_e1150_d_n10: f64 = ((eq47_e1148_d_n10 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][10]));
        let eq47_e1150_d_n11: f64 = ((eq47_e1148_d_n11 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][11]));
        let eq47_e1150_d_n12: f64 = ((eq47_e1148_d_n12 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][12]));
        let eq47_e1151: f64 = self.eval_ddt(8, eq47_e1150);
        let eq47_e1151_d_n0: f64 = self.ddt_jacobian(eq47_e1150_d_n0);
        let eq47_e1151_d_n1: f64 = self.ddt_jacobian(eq47_e1150_d_n1);
        let eq47_e1151_d_n2: f64 = self.ddt_jacobian(eq47_e1150_d_n2);
        let eq47_e1151_d_n3: f64 = self.ddt_jacobian(eq47_e1150_d_n3);
        let eq47_e1151_d_n4: f64 = self.ddt_jacobian(eq47_e1150_d_n4);
        let eq47_e1151_d_n5: f64 = self.ddt_jacobian(eq47_e1150_d_n5);
        let eq47_e1151_d_n6: f64 = self.ddt_jacobian(eq47_e1150_d_n6);
        let eq47_e1151_d_n7: f64 = self.ddt_jacobian(eq47_e1150_d_n7);
        let eq47_e1151_d_n8: f64 = self.ddt_jacobian(eq47_e1150_d_n8);
        let eq47_e1151_d_n9: f64 = self.ddt_jacobian(eq47_e1150_d_n9);
        let eq47_e1151_d_n10: f64 = self.ddt_jacobian(eq47_e1150_d_n10);
        let eq47_e1151_d_n11: f64 = self.ddt_jacobian(eq47_e1150_d_n11);
        let eq47_e1151_d_n12: f64 = self.ddt_jacobian(eq47_e1150_d_n12);
        let eq47_value: f64 = eq47_e1151;
        stamper.stamp_current(
            Some(self.nodes[12]),
            Some(self.nodes[8]),
            self.multiplicity * (eq47_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq47_e1151_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq47_e1151_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq47_e1151_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq47_e1151_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq47_e1151_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq47_e1151_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq47_e1151_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq47_e1151_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq47_e1151_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq47_e1151_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq47_e1151_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq47_e1151_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq47_e1151_d_n12),
            ],
        );
        let eq48_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[5]),
            None,
            self.multiplicity * (eq48_value),
            &[
            ],
        );
        let eq49_e1159: f64 = ((ctx.node_voltage(self.nodes[5]) - 0.0) / scratch.values[999]);
        let eq49_e1159_d_n0: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][0]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n1: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][1]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n2: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][2]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n3: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][3]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n4: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][4]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n5: f64 = ((scratch.values[999] - ((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][5])) / (scratch.values[999] * scratch.values[999]));
        let eq49_e1159_d_n6: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][6]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n7: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][7]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n8: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][8]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n9: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][9]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n10: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][10]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n11: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][11]) / (scratch.values[999] * scratch.values[999])));
        let eq49_e1159_d_n12: f64 = (-(((ctx.node_voltage(self.nodes[5]) - 0.0) * scratch.node_derivatives[999][12]) / (scratch.values[999] * scratch.values[999])));
        let eq49_value: f64 = eq49_e1159;
        stamper.stamp_current(
            Some(self.nodes[5]),
            None,
            self.multiplicity * (eq49_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq49_e1159_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq49_e1159_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq49_e1159_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq49_e1159_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq49_e1159_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq49_e1159_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq49_e1159_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq49_e1159_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq49_e1159_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq49_e1159_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq49_e1159_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq49_e1159_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq49_e1159_d_n12),
            ],
        );
        let eq50_e1162: f64 = (scratch.values[1002] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n0: f64 = (scratch.node_derivatives[1002][0] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n1: f64 = (scratch.node_derivatives[1002][1] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n2: f64 = (scratch.node_derivatives[1002][2] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n3: f64 = (scratch.node_derivatives[1002][3] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n4: f64 = (scratch.node_derivatives[1002][4] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n5: f64 = ((scratch.node_derivatives[1002][5] * (ctx.node_voltage(self.nodes[5]) - 0.0)) + scratch.values[1002]);
        let eq50_e1162_d_n6: f64 = (scratch.node_derivatives[1002][6] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n7: f64 = (scratch.node_derivatives[1002][7] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n8: f64 = (scratch.node_derivatives[1002][8] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n9: f64 = (scratch.node_derivatives[1002][9] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n10: f64 = (scratch.node_derivatives[1002][10] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n11: f64 = (scratch.node_derivatives[1002][11] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n12: f64 = (scratch.node_derivatives[1002][12] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1163: f64 = self.eval_ddt(9, eq50_e1162);
        let eq50_e1163_d_n0: f64 = self.ddt_jacobian(eq50_e1162_d_n0);
        let eq50_e1163_d_n1: f64 = self.ddt_jacobian(eq50_e1162_d_n1);
        let eq50_e1163_d_n2: f64 = self.ddt_jacobian(eq50_e1162_d_n2);
        let eq50_e1163_d_n3: f64 = self.ddt_jacobian(eq50_e1162_d_n3);
        let eq50_e1163_d_n4: f64 = self.ddt_jacobian(eq50_e1162_d_n4);
        let eq50_e1163_d_n5: f64 = self.ddt_jacobian(eq50_e1162_d_n5);
        let eq50_e1163_d_n6: f64 = self.ddt_jacobian(eq50_e1162_d_n6);
        let eq50_e1163_d_n7: f64 = self.ddt_jacobian(eq50_e1162_d_n7);
        let eq50_e1163_d_n8: f64 = self.ddt_jacobian(eq50_e1162_d_n8);
        let eq50_e1163_d_n9: f64 = self.ddt_jacobian(eq50_e1162_d_n9);
        let eq50_e1163_d_n10: f64 = self.ddt_jacobian(eq50_e1162_d_n10);
        let eq50_e1163_d_n11: f64 = self.ddt_jacobian(eq50_e1162_d_n11);
        let eq50_e1163_d_n12: f64 = self.ddt_jacobian(eq50_e1162_d_n12);
        let eq50_value: f64 = eq50_e1163;
        stamper.stamp_current(
            Some(self.nodes[5]),
            None,
            self.multiplicity * (eq50_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq50_e1163_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq50_e1163_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq50_e1163_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq50_e1163_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq50_e1163_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq50_e1163_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq50_e1163_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq50_e1163_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq50_e1163_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq50_e1163_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq50_e1163_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq50_e1163_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq50_e1163_d_n12),
            ],
        );
        let eq51_e1165: f64 = (scratch.values[25]).sqrt();
        let eq51_e1165_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n12: f64 = (scratch.node_derivatives[25][12] / (2.0 * eq51_e1165));
        let eq51_e1167: f64 = (eq51_e1165 * 0.5);
        let eq51_e1167_d_n0: f64 = (eq51_e1165_d_n0 * 0.5);
        let eq51_e1167_d_n1: f64 = (eq51_e1165_d_n1 * 0.5);
        let eq51_e1167_d_n2: f64 = (eq51_e1165_d_n2 * 0.5);
        let eq51_e1167_d_n3: f64 = (eq51_e1165_d_n3 * 0.5);
        let eq51_e1167_d_n4: f64 = (eq51_e1165_d_n4 * 0.5);
        let eq51_e1167_d_n5: f64 = (eq51_e1165_d_n5 * 0.5);
        let eq51_e1167_d_n6: f64 = (eq51_e1165_d_n6 * 0.5);
        let eq51_e1167_d_n7: f64 = (eq51_e1165_d_n7 * 0.5);
        let eq51_e1167_d_n8: f64 = (eq51_e1165_d_n8 * 0.5);
        let eq51_e1167_d_n9: f64 = (eq51_e1165_d_n9 * 0.5);
        let eq51_e1167_d_n10: f64 = (eq51_e1165_d_n10 * 0.5);
        let eq51_e1167_d_n11: f64 = (eq51_e1165_d_n11 * 0.5);
        let eq51_e1167_d_n12: f64 = (eq51_e1165_d_n12 * 0.5);
        let eq51_e1169: f64 = (eq51_e1167 * scratch.values[1002]);
        let eq51_e1169_d_n0: f64 = ((eq51_e1167_d_n0 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][0]));
        let eq51_e1169_d_n1: f64 = ((eq51_e1167_d_n1 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][1]));
        let eq51_e1169_d_n2: f64 = ((eq51_e1167_d_n2 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][2]));
        let eq51_e1169_d_n3: f64 = ((eq51_e1167_d_n3 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][3]));
        let eq51_e1169_d_n4: f64 = ((eq51_e1167_d_n4 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][4]));
        let eq51_e1169_d_n5: f64 = ((eq51_e1167_d_n5 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][5]));
        let eq51_e1169_d_n6: f64 = ((eq51_e1167_d_n6 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][6]));
        let eq51_e1169_d_n7: f64 = ((eq51_e1167_d_n7 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][7]));
        let eq51_e1169_d_n8: f64 = ((eq51_e1167_d_n8 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][8]));
        let eq51_e1169_d_n9: f64 = ((eq51_e1167_d_n9 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][9]));
        let eq51_e1169_d_n10: f64 = ((eq51_e1167_d_n10 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][10]));
        let eq51_e1169_d_n11: f64 = ((eq51_e1167_d_n11 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][11]));
        let eq51_e1169_d_n12: f64 = ((eq51_e1167_d_n12 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][12]));
        let eq51_e1171: f64 = (eq51_e1169 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n0: f64 = (eq51_e1169_d_n0 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n1: f64 = (eq51_e1169_d_n1 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n2: f64 = (eq51_e1169_d_n2 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n3: f64 = (eq51_e1169_d_n3 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n4: f64 = (eq51_e1169_d_n4 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n5: f64 = ((eq51_e1169_d_n5 * (ctx.node_voltage(self.nodes[5]) - 0.0)) + eq51_e1169);
        let eq51_e1171_d_n6: f64 = (eq51_e1169_d_n6 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n7: f64 = (eq51_e1169_d_n7 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n8: f64 = (eq51_e1169_d_n8 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n9: f64 = (eq51_e1169_d_n9 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n10: f64 = (eq51_e1169_d_n10 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n11: f64 = (eq51_e1169_d_n11 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n12: f64 = (eq51_e1169_d_n12 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1172: f64 = self.eval_ddt(10, eq51_e1171);
        let eq51_e1172_d_n0: f64 = self.ddt_jacobian(eq51_e1171_d_n0);
        let eq51_e1172_d_n1: f64 = self.ddt_jacobian(eq51_e1171_d_n1);
        let eq51_e1172_d_n2: f64 = self.ddt_jacobian(eq51_e1171_d_n2);
        let eq51_e1172_d_n3: f64 = self.ddt_jacobian(eq51_e1171_d_n3);
        let eq51_e1172_d_n4: f64 = self.ddt_jacobian(eq51_e1171_d_n4);
        let eq51_e1172_d_n5: f64 = self.ddt_jacobian(eq51_e1171_d_n5);
        let eq51_e1172_d_n6: f64 = self.ddt_jacobian(eq51_e1171_d_n6);
        let eq51_e1172_d_n7: f64 = self.ddt_jacobian(eq51_e1171_d_n7);
        let eq51_e1172_d_n8: f64 = self.ddt_jacobian(eq51_e1171_d_n8);
        let eq51_e1172_d_n9: f64 = self.ddt_jacobian(eq51_e1171_d_n9);
        let eq51_e1172_d_n10: f64 = self.ddt_jacobian(eq51_e1171_d_n10);
        let eq51_e1172_d_n11: f64 = self.ddt_jacobian(eq51_e1171_d_n11);
        let eq51_e1172_d_n12: f64 = self.ddt_jacobian(eq51_e1171_d_n12);
        let eq51_e1173: f64 = (-eq51_e1172);
        let eq51_e1173_d_n0: f64 = (-eq51_e1172_d_n0);
        let eq51_e1173_d_n1: f64 = (-eq51_e1172_d_n1);
        let eq51_e1173_d_n2: f64 = (-eq51_e1172_d_n2);
        let eq51_e1173_d_n3: f64 = (-eq51_e1172_d_n3);
        let eq51_e1173_d_n4: f64 = (-eq51_e1172_d_n4);
        let eq51_e1173_d_n5: f64 = (-eq51_e1172_d_n5);
        let eq51_e1173_d_n6: f64 = (-eq51_e1172_d_n6);
        let eq51_e1173_d_n7: f64 = (-eq51_e1172_d_n7);
        let eq51_e1173_d_n8: f64 = (-eq51_e1172_d_n8);
        let eq51_e1173_d_n9: f64 = (-eq51_e1172_d_n9);
        let eq51_e1173_d_n10: f64 = (-eq51_e1172_d_n10);
        let eq51_e1173_d_n11: f64 = (-eq51_e1172_d_n11);
        let eq51_e1173_d_n12: f64 = (-eq51_e1172_d_n12);
        let eq51_value: f64 = eq51_e1173;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq51_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq51_e1173_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq51_e1173_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq51_e1173_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq51_e1173_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq51_e1173_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq51_e1173_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq51_e1173_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq51_e1173_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq51_e1173_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq51_e1173_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq51_e1173_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq51_e1173_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq51_e1173_d_n12),
            ],
        );
        let eq52_e1175: f64 = (scratch.values[25]).sqrt();
        let eq52_e1175_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n12: f64 = (scratch.node_derivatives[25][12] / (2.0 * eq52_e1175));
        let eq52_e1177: f64 = (eq52_e1175 * 0.5);
        let eq52_e1177_d_n0: f64 = (eq52_e1175_d_n0 * 0.5);
        let eq52_e1177_d_n1: f64 = (eq52_e1175_d_n1 * 0.5);
        let eq52_e1177_d_n2: f64 = (eq52_e1175_d_n2 * 0.5);
        let eq52_e1177_d_n3: f64 = (eq52_e1175_d_n3 * 0.5);
        let eq52_e1177_d_n4: f64 = (eq52_e1175_d_n4 * 0.5);
        let eq52_e1177_d_n5: f64 = (eq52_e1175_d_n5 * 0.5);
        let eq52_e1177_d_n6: f64 = (eq52_e1175_d_n6 * 0.5);
        let eq52_e1177_d_n7: f64 = (eq52_e1175_d_n7 * 0.5);
        let eq52_e1177_d_n8: f64 = (eq52_e1175_d_n8 * 0.5);
        let eq52_e1177_d_n9: f64 = (eq52_e1175_d_n9 * 0.5);
        let eq52_e1177_d_n10: f64 = (eq52_e1175_d_n10 * 0.5);
        let eq52_e1177_d_n11: f64 = (eq52_e1175_d_n11 * 0.5);
        let eq52_e1177_d_n12: f64 = (eq52_e1175_d_n12 * 0.5);
        let eq52_e1179: f64 = (eq52_e1177 * scratch.values[1002]);
        let eq52_e1179_d_n0: f64 = ((eq52_e1177_d_n0 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][0]));
        let eq52_e1179_d_n1: f64 = ((eq52_e1177_d_n1 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][1]));
        let eq52_e1179_d_n2: f64 = ((eq52_e1177_d_n2 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][2]));
        let eq52_e1179_d_n3: f64 = ((eq52_e1177_d_n3 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][3]));
        let eq52_e1179_d_n4: f64 = ((eq52_e1177_d_n4 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][4]));
        let eq52_e1179_d_n5: f64 = ((eq52_e1177_d_n5 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][5]));
        let eq52_e1179_d_n6: f64 = ((eq52_e1177_d_n6 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][6]));
        let eq52_e1179_d_n7: f64 = ((eq52_e1177_d_n7 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][7]));
        let eq52_e1179_d_n8: f64 = ((eq52_e1177_d_n8 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][8]));
        let eq52_e1179_d_n9: f64 = ((eq52_e1177_d_n9 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][9]));
        let eq52_e1179_d_n10: f64 = ((eq52_e1177_d_n10 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][10]));
        let eq52_e1179_d_n11: f64 = ((eq52_e1177_d_n11 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][11]));
        let eq52_e1179_d_n12: f64 = ((eq52_e1177_d_n12 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][12]));
        let eq52_e1181: f64 = (eq52_e1179 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n0: f64 = (eq52_e1179_d_n0 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n1: f64 = (eq52_e1179_d_n1 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n2: f64 = (eq52_e1179_d_n2 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n3: f64 = (eq52_e1179_d_n3 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n4: f64 = (eq52_e1179_d_n4 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n5: f64 = ((eq52_e1179_d_n5 * (ctx.node_voltage(self.nodes[5]) - 0.0)) + eq52_e1179);
        let eq52_e1181_d_n6: f64 = (eq52_e1179_d_n6 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n7: f64 = (eq52_e1179_d_n7 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n8: f64 = (eq52_e1179_d_n8 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n9: f64 = (eq52_e1179_d_n9 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n10: f64 = (eq52_e1179_d_n10 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n11: f64 = (eq52_e1179_d_n11 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n12: f64 = (eq52_e1179_d_n12 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1182: f64 = self.eval_ddt(11, eq52_e1181);
        let eq52_e1182_d_n0: f64 = self.ddt_jacobian(eq52_e1181_d_n0);
        let eq52_e1182_d_n1: f64 = self.ddt_jacobian(eq52_e1181_d_n1);
        let eq52_e1182_d_n2: f64 = self.ddt_jacobian(eq52_e1181_d_n2);
        let eq52_e1182_d_n3: f64 = self.ddt_jacobian(eq52_e1181_d_n3);
        let eq52_e1182_d_n4: f64 = self.ddt_jacobian(eq52_e1181_d_n4);
        let eq52_e1182_d_n5: f64 = self.ddt_jacobian(eq52_e1181_d_n5);
        let eq52_e1182_d_n6: f64 = self.ddt_jacobian(eq52_e1181_d_n6);
        let eq52_e1182_d_n7: f64 = self.ddt_jacobian(eq52_e1181_d_n7);
        let eq52_e1182_d_n8: f64 = self.ddt_jacobian(eq52_e1181_d_n8);
        let eq52_e1182_d_n9: f64 = self.ddt_jacobian(eq52_e1181_d_n9);
        let eq52_e1182_d_n10: f64 = self.ddt_jacobian(eq52_e1181_d_n10);
        let eq52_e1182_d_n11: f64 = self.ddt_jacobian(eq52_e1181_d_n11);
        let eq52_e1182_d_n12: f64 = self.ddt_jacobian(eq52_e1181_d_n12);
        let eq52_e1183: f64 = (-eq52_e1182);
        let eq52_e1183_d_n0: f64 = (-eq52_e1182_d_n0);
        let eq52_e1183_d_n1: f64 = (-eq52_e1182_d_n1);
        let eq52_e1183_d_n2: f64 = (-eq52_e1182_d_n2);
        let eq52_e1183_d_n3: f64 = (-eq52_e1182_d_n3);
        let eq52_e1183_d_n4: f64 = (-eq52_e1182_d_n4);
        let eq52_e1183_d_n5: f64 = (-eq52_e1182_d_n5);
        let eq52_e1183_d_n6: f64 = (-eq52_e1182_d_n6);
        let eq52_e1183_d_n7: f64 = (-eq52_e1182_d_n7);
        let eq52_e1183_d_n8: f64 = (-eq52_e1182_d_n8);
        let eq52_e1183_d_n9: f64 = (-eq52_e1182_d_n9);
        let eq52_e1183_d_n10: f64 = (-eq52_e1182_d_n10);
        let eq52_e1183_d_n11: f64 = (-eq52_e1182_d_n11);
        let eq52_e1183_d_n12: f64 = (-eq52_e1182_d_n12);
        let eq52_value: f64 = eq52_e1183;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq52_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq52_e1183_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq52_e1183_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq52_e1183_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq52_e1183_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq52_e1183_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq52_e1183_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq52_e1183_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq52_e1183_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq52_e1183_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq52_e1183_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq52_e1183_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq52_e1183_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq52_e1183_d_n12),
            ],
        );
        let eq53_e1186: f64 = (scratch.values[25]).sqrt();
        let eq53_e1186_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq53_e1186));
        let eq53_e1186_d_n12: f64 = (scratch.node_derivatives[25][12] / (2.0 * eq53_e1186));
        let eq53_e1187: f64 = (scratch.values[1999] * eq53_e1186);
        let eq53_e1187_d_n0: f64 = ((scratch.node_derivatives[1999][0] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n0));
        let eq53_e1187_d_n1: f64 = ((scratch.node_derivatives[1999][1] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n1));
        let eq53_e1187_d_n2: f64 = ((scratch.node_derivatives[1999][2] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n2));
        let eq53_e1187_d_n3: f64 = ((scratch.node_derivatives[1999][3] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n3));
        let eq53_e1187_d_n4: f64 = ((scratch.node_derivatives[1999][4] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n4));
        let eq53_e1187_d_n5: f64 = ((scratch.node_derivatives[1999][5] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n5));
        let eq53_e1187_d_n6: f64 = ((scratch.node_derivatives[1999][6] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n6));
        let eq53_e1187_d_n7: f64 = ((scratch.node_derivatives[1999][7] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n7));
        let eq53_e1187_d_n8: f64 = ((scratch.node_derivatives[1999][8] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n8));
        let eq53_e1187_d_n9: f64 = ((scratch.node_derivatives[1999][9] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n9));
        let eq53_e1187_d_n10: f64 = ((scratch.node_derivatives[1999][10] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n10));
        let eq53_e1187_d_n11: f64 = ((scratch.node_derivatives[1999][11] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n11));
        let eq53_e1187_d_n12: f64 = ((scratch.node_derivatives[1999][12] * eq53_e1186) + (scratch.values[1999] * eq53_e1186_d_n12));
        let eq53_e1189: f64 = (eq53_e1187 * scratch.values[1001]);
        let eq53_e1189_d_n0: f64 = ((eq53_e1187_d_n0 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][0]));
        let eq53_e1189_d_n1: f64 = ((eq53_e1187_d_n1 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][1]));
        let eq53_e1189_d_n2: f64 = ((eq53_e1187_d_n2 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][2]));
        let eq53_e1189_d_n3: f64 = ((eq53_e1187_d_n3 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][3]));
        let eq53_e1189_d_n4: f64 = ((eq53_e1187_d_n4 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][4]));
        let eq53_e1189_d_n5: f64 = ((eq53_e1187_d_n5 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][5]));
        let eq53_e1189_d_n6: f64 = ((eq53_e1187_d_n6 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][6]));
        let eq53_e1189_d_n7: f64 = ((eq53_e1187_d_n7 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][7]));
        let eq53_e1189_d_n8: f64 = ((eq53_e1187_d_n8 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][8]));
        let eq53_e1189_d_n9: f64 = ((eq53_e1187_d_n9 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][9]));
        let eq53_e1189_d_n10: f64 = ((eq53_e1187_d_n10 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][10]));
        let eq53_e1189_d_n11: f64 = ((eq53_e1187_d_n11 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][11]));
        let eq53_e1189_d_n12: f64 = ((eq53_e1187_d_n12 * scratch.values[1001]) + (eq53_e1187 * scratch.node_derivatives[1001][12]));
        let eq53_e1191: f64 = (eq53_e1189 * eq48_value);
        let eq53_e1191_d_n0: f64 = (eq53_e1189_d_n0 * eq48_value);
        let eq53_e1191_d_n1: f64 = (eq53_e1189_d_n1 * eq48_value);
        let eq53_e1191_d_n2: f64 = (eq53_e1189_d_n2 * eq48_value);
        let eq53_e1191_d_n3: f64 = (eq53_e1189_d_n3 * eq48_value);
        let eq53_e1191_d_n4: f64 = (eq53_e1189_d_n4 * eq48_value);
        let eq53_e1191_d_n5: f64 = (eq53_e1189_d_n5 * eq48_value);
        let eq53_e1191_d_n6: f64 = (eq53_e1189_d_n6 * eq48_value);
        let eq53_e1191_d_n7: f64 = (eq53_e1189_d_n7 * eq48_value);
        let eq53_e1191_d_n8: f64 = (eq53_e1189_d_n8 * eq48_value);
        let eq53_e1191_d_n9: f64 = (eq53_e1189_d_n9 * eq48_value);
        let eq53_e1191_d_n10: f64 = (eq53_e1189_d_n10 * eq48_value);
        let eq53_e1191_d_n11: f64 = (eq53_e1189_d_n11 * eq48_value);
        let eq53_e1191_d_n12: f64 = (eq53_e1189_d_n12 * eq48_value);
        let eq53_value: f64 = eq53_e1191;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq53_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq53_e1191_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq53_e1191_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq53_e1191_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq53_e1191_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq53_e1191_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq53_e1191_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq53_e1191_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq53_e1191_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq53_e1191_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq53_e1191_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq53_e1191_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq53_e1191_d_n11),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * eq53_e1191_d_n12),
            ],
        );
        let eq54_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq54_value),
            &[
            ],
        );
        let eq55_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq55_value),
            &[
            ],
        );
        let eq56_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq56_value),
            &[
            ],
        );
        let eq57_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq57_value),
            &[
            ],
        );
        let eq58_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq58_value),
            &[
            ],
        );
        let eq59_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[12]),
            Some(self.nodes[8]),
            self.multiplicity * (eq59_value),
            &[
            ],
        );
        let eq60_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq60_value),
            &[
            ],
        );
        let eq61_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            self.multiplicity * (eq61_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let mut scratch = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_1(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_2(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_3(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_4(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_5(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_6(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_7(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_8(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_9(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_10(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_11(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_12(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_13(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_14(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_15(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_16(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_17(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_18(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_19(ctx, stamper, &mut scratch);

        let eq38_e1095: f64 = (scratch.values[25] * scratch.values[307]);
        let eq38_e1095_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][0]));
        let eq38_e1095_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][1]));
        let eq38_e1095_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][2]));
        let eq38_e1095_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][3]));
        let eq38_e1095_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][4]));
        let eq38_e1095_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][5]));
        let eq38_e1095_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][6]));
        let eq38_e1095_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][7]));
        let eq38_e1095_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][8]));
        let eq38_e1095_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][9]));
        let eq38_e1095_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][10]));
        let eq38_e1095_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][11]));
        let eq38_e1095_d_n12: f64 = ((scratch.node_derivatives[25][12] * scratch.values[307]) + (scratch.values[25] * scratch.node_derivatives[307][12]));
        let eq38_e1097: f64 = (eq38_e1095 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n0: f64 = (eq38_e1095_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n1: f64 = (eq38_e1095_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n2: f64 = (eq38_e1095_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n3: f64 = (eq38_e1095_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n4: f64 = ((eq38_e1095_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq38_e1095);
        let eq38_e1097_d_n5: f64 = (eq38_e1095_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n6: f64 = (eq38_e1095_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n7: f64 = (eq38_e1095_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n8: f64 = (eq38_e1095_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n9: f64 = (eq38_e1095_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n10: f64 = (eq38_e1095_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n11: f64 = (eq38_e1095_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1097_d_n12: f64 = (eq38_e1095_d_n12 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq38_e1098_q: f64 = eq38_e1097;
        stamper.stamp_current_reactive(
            Some(self.nodes[4]),
            None,
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq38_e1097_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq38_e1097_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq38_e1097_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq38_e1097_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq38_e1097_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq38_e1097_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq38_e1097_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq38_e1097_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq38_e1097_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq38_e1097_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq38_e1097_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq38_e1097_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq38_e1097_d_n12)),
            ],
        );
        let eq40_e1106: f64 = (scratch.values[0] * scratch.values[25]);
        let eq40_e1106_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq40_e1106_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq40_e1106_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq40_e1106_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq40_e1106_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq40_e1106_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq40_e1106_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq40_e1106_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq40_e1106_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq40_e1106_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq40_e1106_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq40_e1106_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq40_e1106_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq40_e1108: f64 = (eq40_e1106 * scratch.values[940]);
        let eq40_e1108_d_n0: f64 = ((eq40_e1106_d_n0 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][0]));
        let eq40_e1108_d_n1: f64 = ((eq40_e1106_d_n1 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][1]));
        let eq40_e1108_d_n2: f64 = ((eq40_e1106_d_n2 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][2]));
        let eq40_e1108_d_n3: f64 = ((eq40_e1106_d_n3 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][3]));
        let eq40_e1108_d_n4: f64 = ((eq40_e1106_d_n4 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][4]));
        let eq40_e1108_d_n5: f64 = ((eq40_e1106_d_n5 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][5]));
        let eq40_e1108_d_n6: f64 = ((eq40_e1106_d_n6 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][6]));
        let eq40_e1108_d_n7: f64 = ((eq40_e1106_d_n7 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][7]));
        let eq40_e1108_d_n8: f64 = ((eq40_e1106_d_n8 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][8]));
        let eq40_e1108_d_n9: f64 = ((eq40_e1106_d_n9 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][9]));
        let eq40_e1108_d_n10: f64 = ((eq40_e1106_d_n10 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][10]));
        let eq40_e1108_d_n11: f64 = ((eq40_e1106_d_n11 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][11]));
        let eq40_e1108_d_n12: f64 = ((eq40_e1106_d_n12 * scratch.values[940]) + (eq40_e1106 * scratch.node_derivatives[940][12]));
        let eq40_e1109_q: f64 = eq40_e1108;
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq40_e1108_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq40_e1108_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq40_e1108_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq40_e1108_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq40_e1108_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq40_e1108_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq40_e1108_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq40_e1108_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq40_e1108_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq40_e1108_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq40_e1108_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq40_e1108_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq40_e1108_d_n12)),
            ],
        );
        let eq41_e1112: f64 = (scratch.values[0] * scratch.values[25]);
        let eq41_e1112_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq41_e1112_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq41_e1112_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq41_e1112_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq41_e1112_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq41_e1112_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq41_e1112_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq41_e1112_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq41_e1112_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq41_e1112_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq41_e1112_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq41_e1112_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq41_e1112_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq41_e1114: f64 = (eq41_e1112 * scratch.values[942]);
        let eq41_e1114_d_n0: f64 = ((eq41_e1112_d_n0 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][0]));
        let eq41_e1114_d_n1: f64 = ((eq41_e1112_d_n1 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][1]));
        let eq41_e1114_d_n2: f64 = ((eq41_e1112_d_n2 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][2]));
        let eq41_e1114_d_n3: f64 = ((eq41_e1112_d_n3 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][3]));
        let eq41_e1114_d_n4: f64 = ((eq41_e1112_d_n4 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][4]));
        let eq41_e1114_d_n5: f64 = ((eq41_e1112_d_n5 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][5]));
        let eq41_e1114_d_n6: f64 = ((eq41_e1112_d_n6 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][6]));
        let eq41_e1114_d_n7: f64 = ((eq41_e1112_d_n7 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][7]));
        let eq41_e1114_d_n8: f64 = ((eq41_e1112_d_n8 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][8]));
        let eq41_e1114_d_n9: f64 = ((eq41_e1112_d_n9 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][9]));
        let eq41_e1114_d_n10: f64 = ((eq41_e1112_d_n10 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][10]));
        let eq41_e1114_d_n11: f64 = ((eq41_e1112_d_n11 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][11]));
        let eq41_e1114_d_n12: f64 = ((eq41_e1112_d_n12 * scratch.values[942]) + (eq41_e1112 * scratch.node_derivatives[942][12]));
        let eq41_e1115_q: f64 = eq41_e1114;
        stamper.stamp_current_reactive(
            Some(self.nodes[9]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq41_e1114_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq41_e1114_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq41_e1114_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq41_e1114_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq41_e1114_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq41_e1114_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq41_e1114_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq41_e1114_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq41_e1114_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq41_e1114_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq41_e1114_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq41_e1114_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq41_e1114_d_n12)),
            ],
        );
        let eq42_e1118: f64 = (scratch.values[0] * scratch.values[25]);
        let eq42_e1118_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq42_e1118_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq42_e1118_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq42_e1118_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq42_e1118_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq42_e1118_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq42_e1118_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq42_e1118_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq42_e1118_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq42_e1118_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq42_e1118_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq42_e1118_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq42_e1118_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq42_e1120: f64 = (eq42_e1118 * scratch.values[941]);
        let eq42_e1120_d_n0: f64 = ((eq42_e1118_d_n0 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][0]));
        let eq42_e1120_d_n1: f64 = ((eq42_e1118_d_n1 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][1]));
        let eq42_e1120_d_n2: f64 = ((eq42_e1118_d_n2 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][2]));
        let eq42_e1120_d_n3: f64 = ((eq42_e1118_d_n3 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][3]));
        let eq42_e1120_d_n4: f64 = ((eq42_e1118_d_n4 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][4]));
        let eq42_e1120_d_n5: f64 = ((eq42_e1118_d_n5 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][5]));
        let eq42_e1120_d_n6: f64 = ((eq42_e1118_d_n6 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][6]));
        let eq42_e1120_d_n7: f64 = ((eq42_e1118_d_n7 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][7]));
        let eq42_e1120_d_n8: f64 = ((eq42_e1118_d_n8 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][8]));
        let eq42_e1120_d_n9: f64 = ((eq42_e1118_d_n9 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][9]));
        let eq42_e1120_d_n10: f64 = ((eq42_e1118_d_n10 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][10]));
        let eq42_e1120_d_n11: f64 = ((eq42_e1118_d_n11 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][11]));
        let eq42_e1120_d_n12: f64 = ((eq42_e1118_d_n12 * scratch.values[941]) + (eq42_e1118 * scratch.node_derivatives[941][12]));
        let eq42_e1121_q: f64 = eq42_e1120;
        stamper.stamp_current_reactive(
            Some(self.nodes[8]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq42_e1120_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq42_e1120_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq42_e1120_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq42_e1120_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq42_e1120_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq42_e1120_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq42_e1120_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq42_e1120_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq42_e1120_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq42_e1120_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq42_e1120_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq42_e1120_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq42_e1120_d_n12)),
            ],
        );
        let eq43_e1124: f64 = (scratch.values[0] * scratch.values[25]);
        let eq43_e1124_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq43_e1124_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq43_e1124_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq43_e1124_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq43_e1124_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq43_e1124_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq43_e1124_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq43_e1124_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq43_e1124_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq43_e1124_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq43_e1124_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq43_e1124_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq43_e1124_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq43_e1126: f64 = (eq43_e1124 * scratch.values[947]);
        let eq43_e1126_d_n0: f64 = ((eq43_e1124_d_n0 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][0]));
        let eq43_e1126_d_n1: f64 = ((eq43_e1124_d_n1 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][1]));
        let eq43_e1126_d_n2: f64 = ((eq43_e1124_d_n2 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][2]));
        let eq43_e1126_d_n3: f64 = ((eq43_e1124_d_n3 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][3]));
        let eq43_e1126_d_n4: f64 = ((eq43_e1124_d_n4 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][4]));
        let eq43_e1126_d_n5: f64 = ((eq43_e1124_d_n5 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][5]));
        let eq43_e1126_d_n6: f64 = ((eq43_e1124_d_n6 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][6]));
        let eq43_e1126_d_n7: f64 = ((eq43_e1124_d_n7 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][7]));
        let eq43_e1126_d_n8: f64 = ((eq43_e1124_d_n8 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][8]));
        let eq43_e1126_d_n9: f64 = ((eq43_e1124_d_n9 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][9]));
        let eq43_e1126_d_n10: f64 = ((eq43_e1124_d_n10 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][10]));
        let eq43_e1126_d_n11: f64 = ((eq43_e1124_d_n11 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][11]));
        let eq43_e1126_d_n12: f64 = ((eq43_e1124_d_n12 * scratch.values[947]) + (eq43_e1124 * scratch.node_derivatives[947][12]));
        let eq43_e1127_q: f64 = eq43_e1126;
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq43_e1126_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq43_e1126_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq43_e1126_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq43_e1126_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq43_e1126_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq43_e1126_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq43_e1126_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq43_e1126_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq43_e1126_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq43_e1126_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq43_e1126_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq43_e1126_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq43_e1126_d_n12)),
            ],
        );
        let eq44_e1130: f64 = (scratch.values[0] * scratch.values[25]);
        let eq44_e1130_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq44_e1130_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq44_e1130_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq44_e1130_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq44_e1130_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq44_e1130_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq44_e1130_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq44_e1130_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq44_e1130_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq44_e1130_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq44_e1130_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq44_e1130_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq44_e1130_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq44_e1132: f64 = (eq44_e1130 * scratch.values[948]);
        let eq44_e1132_d_n0: f64 = ((eq44_e1130_d_n0 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][0]));
        let eq44_e1132_d_n1: f64 = ((eq44_e1130_d_n1 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][1]));
        let eq44_e1132_d_n2: f64 = ((eq44_e1130_d_n2 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][2]));
        let eq44_e1132_d_n3: f64 = ((eq44_e1130_d_n3 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][3]));
        let eq44_e1132_d_n4: f64 = ((eq44_e1130_d_n4 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][4]));
        let eq44_e1132_d_n5: f64 = ((eq44_e1130_d_n5 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][5]));
        let eq44_e1132_d_n6: f64 = ((eq44_e1130_d_n6 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][6]));
        let eq44_e1132_d_n7: f64 = ((eq44_e1130_d_n7 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][7]));
        let eq44_e1132_d_n8: f64 = ((eq44_e1130_d_n8 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][8]));
        let eq44_e1132_d_n9: f64 = ((eq44_e1130_d_n9 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][9]));
        let eq44_e1132_d_n10: f64 = ((eq44_e1130_d_n10 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][10]));
        let eq44_e1132_d_n11: f64 = ((eq44_e1130_d_n11 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][11]));
        let eq44_e1132_d_n12: f64 = ((eq44_e1130_d_n12 * scratch.values[948]) + (eq44_e1130 * scratch.node_derivatives[948][12]));
        let eq44_e1133_q: f64 = eq44_e1132;
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq44_e1132_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq44_e1132_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq44_e1132_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq44_e1132_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq44_e1132_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq44_e1132_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq44_e1132_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq44_e1132_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq44_e1132_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq44_e1132_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq44_e1132_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq44_e1132_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq44_e1132_d_n12)),
            ],
        );
        let eq45_e1136: f64 = (scratch.values[0] * scratch.values[25]);
        let eq45_e1136_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq45_e1136_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq45_e1136_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq45_e1136_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq45_e1136_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq45_e1136_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq45_e1136_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq45_e1136_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq45_e1136_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq45_e1136_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq45_e1136_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq45_e1136_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq45_e1136_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq45_e1138: f64 = (eq45_e1136 * scratch.values[946]);
        let eq45_e1138_d_n0: f64 = ((eq45_e1136_d_n0 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][0]));
        let eq45_e1138_d_n1: f64 = ((eq45_e1136_d_n1 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][1]));
        let eq45_e1138_d_n2: f64 = ((eq45_e1136_d_n2 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][2]));
        let eq45_e1138_d_n3: f64 = ((eq45_e1136_d_n3 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][3]));
        let eq45_e1138_d_n4: f64 = ((eq45_e1136_d_n4 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][4]));
        let eq45_e1138_d_n5: f64 = ((eq45_e1136_d_n5 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][5]));
        let eq45_e1138_d_n6: f64 = ((eq45_e1136_d_n6 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][6]));
        let eq45_e1138_d_n7: f64 = ((eq45_e1136_d_n7 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][7]));
        let eq45_e1138_d_n8: f64 = ((eq45_e1136_d_n8 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][8]));
        let eq45_e1138_d_n9: f64 = ((eq45_e1136_d_n9 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][9]));
        let eq45_e1138_d_n10: f64 = ((eq45_e1136_d_n10 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][10]));
        let eq45_e1138_d_n11: f64 = ((eq45_e1136_d_n11 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][11]));
        let eq45_e1138_d_n12: f64 = ((eq45_e1136_d_n12 * scratch.values[946]) + (eq45_e1136 * scratch.node_derivatives[946][12]));
        let eq45_e1139_q: f64 = eq45_e1138;
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[9]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq45_e1138_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq45_e1138_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq45_e1138_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq45_e1138_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq45_e1138_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq45_e1138_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq45_e1138_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq45_e1138_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq45_e1138_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq45_e1138_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq45_e1138_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq45_e1138_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq45_e1138_d_n12)),
            ],
        );
        let eq46_e1142: f64 = (scratch.values[0] * scratch.values[25]);
        let eq46_e1142_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq46_e1142_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq46_e1142_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq46_e1142_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq46_e1142_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq46_e1142_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq46_e1142_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq46_e1142_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq46_e1142_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq46_e1142_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq46_e1142_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq46_e1142_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq46_e1142_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq46_e1144: f64 = (eq46_e1142 * scratch.values[2038]);
        let eq46_e1144_d_n0: f64 = ((eq46_e1142_d_n0 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][0]));
        let eq46_e1144_d_n1: f64 = ((eq46_e1142_d_n1 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][1]));
        let eq46_e1144_d_n2: f64 = ((eq46_e1142_d_n2 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][2]));
        let eq46_e1144_d_n3: f64 = ((eq46_e1142_d_n3 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][3]));
        let eq46_e1144_d_n4: f64 = ((eq46_e1142_d_n4 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][4]));
        let eq46_e1144_d_n5: f64 = ((eq46_e1142_d_n5 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][5]));
        let eq46_e1144_d_n6: f64 = ((eq46_e1142_d_n6 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][6]));
        let eq46_e1144_d_n7: f64 = ((eq46_e1142_d_n7 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][7]));
        let eq46_e1144_d_n8: f64 = ((eq46_e1142_d_n8 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][8]));
        let eq46_e1144_d_n9: f64 = ((eq46_e1142_d_n9 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][9]));
        let eq46_e1144_d_n10: f64 = ((eq46_e1142_d_n10 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][10]));
        let eq46_e1144_d_n11: f64 = ((eq46_e1142_d_n11 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][11]));
        let eq46_e1144_d_n12: f64 = ((eq46_e1142_d_n12 * scratch.values[2038]) + (eq46_e1142 * scratch.node_derivatives[2038][12]));
        let eq46_e1145_q: f64 = eq46_e1144;
        stamper.stamp_current_reactive(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq46_e1144_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq46_e1144_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq46_e1144_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq46_e1144_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq46_e1144_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq46_e1144_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq46_e1144_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq46_e1144_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq46_e1144_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq46_e1144_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq46_e1144_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq46_e1144_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq46_e1144_d_n12)),
            ],
        );
        let eq47_e1148: f64 = (scratch.values[0] * scratch.values[25]);
        let eq47_e1148_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq47_e1148_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq47_e1148_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq47_e1148_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq47_e1148_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq47_e1148_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq47_e1148_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq47_e1148_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq47_e1148_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq47_e1148_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq47_e1148_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq47_e1148_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq47_e1148_d_n12: f64 = ((scratch.node_derivatives[0][12] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][12]));
        let eq47_e1150: f64 = (eq47_e1148 * scratch.values[2042]);
        let eq47_e1150_d_n0: f64 = ((eq47_e1148_d_n0 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][0]));
        let eq47_e1150_d_n1: f64 = ((eq47_e1148_d_n1 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][1]));
        let eq47_e1150_d_n2: f64 = ((eq47_e1148_d_n2 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][2]));
        let eq47_e1150_d_n3: f64 = ((eq47_e1148_d_n3 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][3]));
        let eq47_e1150_d_n4: f64 = ((eq47_e1148_d_n4 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][4]));
        let eq47_e1150_d_n5: f64 = ((eq47_e1148_d_n5 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][5]));
        let eq47_e1150_d_n6: f64 = ((eq47_e1148_d_n6 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][6]));
        let eq47_e1150_d_n7: f64 = ((eq47_e1148_d_n7 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][7]));
        let eq47_e1150_d_n8: f64 = ((eq47_e1148_d_n8 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][8]));
        let eq47_e1150_d_n9: f64 = ((eq47_e1148_d_n9 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][9]));
        let eq47_e1150_d_n10: f64 = ((eq47_e1148_d_n10 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][10]));
        let eq47_e1150_d_n11: f64 = ((eq47_e1148_d_n11 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][11]));
        let eq47_e1150_d_n12: f64 = ((eq47_e1148_d_n12 * scratch.values[2042]) + (eq47_e1148 * scratch.node_derivatives[2042][12]));
        let eq47_e1151_q: f64 = eq47_e1150;
        stamper.stamp_current_reactive(
            Some(self.nodes[12]),
            Some(self.nodes[8]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq47_e1150_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq47_e1150_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq47_e1150_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq47_e1150_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq47_e1150_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq47_e1150_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq47_e1150_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq47_e1150_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq47_e1150_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq47_e1150_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq47_e1150_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq47_e1150_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq47_e1150_d_n12)),
            ],
        );
        let eq50_e1162: f64 = (scratch.values[1002] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n0: f64 = (scratch.node_derivatives[1002][0] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n1: f64 = (scratch.node_derivatives[1002][1] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n2: f64 = (scratch.node_derivatives[1002][2] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n3: f64 = (scratch.node_derivatives[1002][3] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n4: f64 = (scratch.node_derivatives[1002][4] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n5: f64 = ((scratch.node_derivatives[1002][5] * (ctx.node_voltage(self.nodes[5]) - 0.0)) + scratch.values[1002]);
        let eq50_e1162_d_n6: f64 = (scratch.node_derivatives[1002][6] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n7: f64 = (scratch.node_derivatives[1002][7] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n8: f64 = (scratch.node_derivatives[1002][8] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n9: f64 = (scratch.node_derivatives[1002][9] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n10: f64 = (scratch.node_derivatives[1002][10] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n11: f64 = (scratch.node_derivatives[1002][11] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1162_d_n12: f64 = (scratch.node_derivatives[1002][12] * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq50_e1163_q: f64 = eq50_e1162;
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            None,
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq50_e1162_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq50_e1162_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq50_e1162_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq50_e1162_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq50_e1162_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq50_e1162_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq50_e1162_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq50_e1162_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq50_e1162_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq50_e1162_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq50_e1162_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq50_e1162_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq50_e1162_d_n12)),
            ],
        );
        let eq51_e1165: f64 = (scratch.values[25]).sqrt();
        let eq51_e1165_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq51_e1165));
        let eq51_e1165_d_n12: f64 = (scratch.node_derivatives[25][12] / (2.0 * eq51_e1165));
        let eq51_e1167: f64 = (eq51_e1165 * 0.5);
        let eq51_e1167_d_n0: f64 = (eq51_e1165_d_n0 * 0.5);
        let eq51_e1167_d_n1: f64 = (eq51_e1165_d_n1 * 0.5);
        let eq51_e1167_d_n2: f64 = (eq51_e1165_d_n2 * 0.5);
        let eq51_e1167_d_n3: f64 = (eq51_e1165_d_n3 * 0.5);
        let eq51_e1167_d_n4: f64 = (eq51_e1165_d_n4 * 0.5);
        let eq51_e1167_d_n5: f64 = (eq51_e1165_d_n5 * 0.5);
        let eq51_e1167_d_n6: f64 = (eq51_e1165_d_n6 * 0.5);
        let eq51_e1167_d_n7: f64 = (eq51_e1165_d_n7 * 0.5);
        let eq51_e1167_d_n8: f64 = (eq51_e1165_d_n8 * 0.5);
        let eq51_e1167_d_n9: f64 = (eq51_e1165_d_n9 * 0.5);
        let eq51_e1167_d_n10: f64 = (eq51_e1165_d_n10 * 0.5);
        let eq51_e1167_d_n11: f64 = (eq51_e1165_d_n11 * 0.5);
        let eq51_e1167_d_n12: f64 = (eq51_e1165_d_n12 * 0.5);
        let eq51_e1169: f64 = (eq51_e1167 * scratch.values[1002]);
        let eq51_e1169_d_n0: f64 = ((eq51_e1167_d_n0 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][0]));
        let eq51_e1169_d_n1: f64 = ((eq51_e1167_d_n1 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][1]));
        let eq51_e1169_d_n2: f64 = ((eq51_e1167_d_n2 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][2]));
        let eq51_e1169_d_n3: f64 = ((eq51_e1167_d_n3 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][3]));
        let eq51_e1169_d_n4: f64 = ((eq51_e1167_d_n4 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][4]));
        let eq51_e1169_d_n5: f64 = ((eq51_e1167_d_n5 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][5]));
        let eq51_e1169_d_n6: f64 = ((eq51_e1167_d_n6 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][6]));
        let eq51_e1169_d_n7: f64 = ((eq51_e1167_d_n7 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][7]));
        let eq51_e1169_d_n8: f64 = ((eq51_e1167_d_n8 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][8]));
        let eq51_e1169_d_n9: f64 = ((eq51_e1167_d_n9 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][9]));
        let eq51_e1169_d_n10: f64 = ((eq51_e1167_d_n10 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][10]));
        let eq51_e1169_d_n11: f64 = ((eq51_e1167_d_n11 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][11]));
        let eq51_e1169_d_n12: f64 = ((eq51_e1167_d_n12 * scratch.values[1002]) + (eq51_e1167 * scratch.node_derivatives[1002][12]));
        let eq51_e1171: f64 = (eq51_e1169 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n0: f64 = (eq51_e1169_d_n0 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n1: f64 = (eq51_e1169_d_n1 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n2: f64 = (eq51_e1169_d_n2 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n3: f64 = (eq51_e1169_d_n3 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n4: f64 = (eq51_e1169_d_n4 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n5: f64 = ((eq51_e1169_d_n5 * (ctx.node_voltage(self.nodes[5]) - 0.0)) + eq51_e1169);
        let eq51_e1171_d_n6: f64 = (eq51_e1169_d_n6 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n7: f64 = (eq51_e1169_d_n7 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n8: f64 = (eq51_e1169_d_n8 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n9: f64 = (eq51_e1169_d_n9 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n10: f64 = (eq51_e1169_d_n10 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n11: f64 = (eq51_e1169_d_n11 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1171_d_n12: f64 = (eq51_e1169_d_n12 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq51_e1172_q: f64 = eq51_e1171;
        let eq51_e1173: f64 = (-eq51_e1171);
        let eq51_e1173_d_n0: f64 = (-eq51_e1171_d_n0);
        let eq51_e1173_d_n1: f64 = (-eq51_e1171_d_n1);
        let eq51_e1173_d_n2: f64 = (-eq51_e1171_d_n2);
        let eq51_e1173_d_n3: f64 = (-eq51_e1171_d_n3);
        let eq51_e1173_d_n4: f64 = (-eq51_e1171_d_n4);
        let eq51_e1173_d_n5: f64 = (-eq51_e1171_d_n5);
        let eq51_e1173_d_n6: f64 = (-eq51_e1171_d_n6);
        let eq51_e1173_d_n7: f64 = (-eq51_e1171_d_n7);
        let eq51_e1173_d_n8: f64 = (-eq51_e1171_d_n8);
        let eq51_e1173_d_n9: f64 = (-eq51_e1171_d_n9);
        let eq51_e1173_d_n10: f64 = (-eq51_e1171_d_n10);
        let eq51_e1173_d_n11: f64 = (-eq51_e1171_d_n11);
        let eq51_e1173_d_n12: f64 = (-eq51_e1171_d_n12);
        let eq51_e1173_q: f64 = (-eq51_e1172_q);
        let eq51_e1173_q_d_n0: f64 = (-eq51_e1171_d_n0);
        let eq51_e1173_q_d_n1: f64 = (-eq51_e1171_d_n1);
        let eq51_e1173_q_d_n2: f64 = (-eq51_e1171_d_n2);
        let eq51_e1173_q_d_n3: f64 = (-eq51_e1171_d_n3);
        let eq51_e1173_q_d_n4: f64 = (-eq51_e1171_d_n4);
        let eq51_e1173_q_d_n5: f64 = (-eq51_e1171_d_n5);
        let eq51_e1173_q_d_n6: f64 = (-eq51_e1171_d_n6);
        let eq51_e1173_q_d_n7: f64 = (-eq51_e1171_d_n7);
        let eq51_e1173_q_d_n8: f64 = (-eq51_e1171_d_n8);
        let eq51_e1173_q_d_n9: f64 = (-eq51_e1171_d_n9);
        let eq51_e1173_q_d_n10: f64 = (-eq51_e1171_d_n10);
        let eq51_e1173_q_d_n11: f64 = (-eq51_e1171_d_n11);
        let eq51_e1173_q_d_n12: f64 = (-eq51_e1171_d_n12);
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq51_e1173_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq51_e1173_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq51_e1173_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq51_e1173_q_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq51_e1173_q_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq51_e1173_q_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq51_e1173_q_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq51_e1173_q_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq51_e1173_q_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq51_e1173_q_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq51_e1173_q_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq51_e1173_q_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq51_e1173_q_d_n12)),
            ],
        );
        let eq52_e1175: f64 = (scratch.values[25]).sqrt();
        let eq52_e1175_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq52_e1175));
        let eq52_e1175_d_n12: f64 = (scratch.node_derivatives[25][12] / (2.0 * eq52_e1175));
        let eq52_e1177: f64 = (eq52_e1175 * 0.5);
        let eq52_e1177_d_n0: f64 = (eq52_e1175_d_n0 * 0.5);
        let eq52_e1177_d_n1: f64 = (eq52_e1175_d_n1 * 0.5);
        let eq52_e1177_d_n2: f64 = (eq52_e1175_d_n2 * 0.5);
        let eq52_e1177_d_n3: f64 = (eq52_e1175_d_n3 * 0.5);
        let eq52_e1177_d_n4: f64 = (eq52_e1175_d_n4 * 0.5);
        let eq52_e1177_d_n5: f64 = (eq52_e1175_d_n5 * 0.5);
        let eq52_e1177_d_n6: f64 = (eq52_e1175_d_n6 * 0.5);
        let eq52_e1177_d_n7: f64 = (eq52_e1175_d_n7 * 0.5);
        let eq52_e1177_d_n8: f64 = (eq52_e1175_d_n8 * 0.5);
        let eq52_e1177_d_n9: f64 = (eq52_e1175_d_n9 * 0.5);
        let eq52_e1177_d_n10: f64 = (eq52_e1175_d_n10 * 0.5);
        let eq52_e1177_d_n11: f64 = (eq52_e1175_d_n11 * 0.5);
        let eq52_e1177_d_n12: f64 = (eq52_e1175_d_n12 * 0.5);
        let eq52_e1179: f64 = (eq52_e1177 * scratch.values[1002]);
        let eq52_e1179_d_n0: f64 = ((eq52_e1177_d_n0 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][0]));
        let eq52_e1179_d_n1: f64 = ((eq52_e1177_d_n1 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][1]));
        let eq52_e1179_d_n2: f64 = ((eq52_e1177_d_n2 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][2]));
        let eq52_e1179_d_n3: f64 = ((eq52_e1177_d_n3 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][3]));
        let eq52_e1179_d_n4: f64 = ((eq52_e1177_d_n4 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][4]));
        let eq52_e1179_d_n5: f64 = ((eq52_e1177_d_n5 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][5]));
        let eq52_e1179_d_n6: f64 = ((eq52_e1177_d_n6 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][6]));
        let eq52_e1179_d_n7: f64 = ((eq52_e1177_d_n7 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][7]));
        let eq52_e1179_d_n8: f64 = ((eq52_e1177_d_n8 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][8]));
        let eq52_e1179_d_n9: f64 = ((eq52_e1177_d_n9 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][9]));
        let eq52_e1179_d_n10: f64 = ((eq52_e1177_d_n10 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][10]));
        let eq52_e1179_d_n11: f64 = ((eq52_e1177_d_n11 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][11]));
        let eq52_e1179_d_n12: f64 = ((eq52_e1177_d_n12 * scratch.values[1002]) + (eq52_e1177 * scratch.node_derivatives[1002][12]));
        let eq52_e1181: f64 = (eq52_e1179 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n0: f64 = (eq52_e1179_d_n0 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n1: f64 = (eq52_e1179_d_n1 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n2: f64 = (eq52_e1179_d_n2 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n3: f64 = (eq52_e1179_d_n3 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n4: f64 = (eq52_e1179_d_n4 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n5: f64 = ((eq52_e1179_d_n5 * (ctx.node_voltage(self.nodes[5]) - 0.0)) + eq52_e1179);
        let eq52_e1181_d_n6: f64 = (eq52_e1179_d_n6 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n7: f64 = (eq52_e1179_d_n7 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n8: f64 = (eq52_e1179_d_n8 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n9: f64 = (eq52_e1179_d_n9 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n10: f64 = (eq52_e1179_d_n10 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n11: f64 = (eq52_e1179_d_n11 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1181_d_n12: f64 = (eq52_e1179_d_n12 * (ctx.node_voltage(self.nodes[5]) - 0.0));
        let eq52_e1182_q: f64 = eq52_e1181;
        let eq52_e1183: f64 = (-eq52_e1181);
        let eq52_e1183_d_n0: f64 = (-eq52_e1181_d_n0);
        let eq52_e1183_d_n1: f64 = (-eq52_e1181_d_n1);
        let eq52_e1183_d_n2: f64 = (-eq52_e1181_d_n2);
        let eq52_e1183_d_n3: f64 = (-eq52_e1181_d_n3);
        let eq52_e1183_d_n4: f64 = (-eq52_e1181_d_n4);
        let eq52_e1183_d_n5: f64 = (-eq52_e1181_d_n5);
        let eq52_e1183_d_n6: f64 = (-eq52_e1181_d_n6);
        let eq52_e1183_d_n7: f64 = (-eq52_e1181_d_n7);
        let eq52_e1183_d_n8: f64 = (-eq52_e1181_d_n8);
        let eq52_e1183_d_n9: f64 = (-eq52_e1181_d_n9);
        let eq52_e1183_d_n10: f64 = (-eq52_e1181_d_n10);
        let eq52_e1183_d_n11: f64 = (-eq52_e1181_d_n11);
        let eq52_e1183_d_n12: f64 = (-eq52_e1181_d_n12);
        let eq52_e1183_q: f64 = (-eq52_e1182_q);
        let eq52_e1183_q_d_n0: f64 = (-eq52_e1181_d_n0);
        let eq52_e1183_q_d_n1: f64 = (-eq52_e1181_d_n1);
        let eq52_e1183_q_d_n2: f64 = (-eq52_e1181_d_n2);
        let eq52_e1183_q_d_n3: f64 = (-eq52_e1181_d_n3);
        let eq52_e1183_q_d_n4: f64 = (-eq52_e1181_d_n4);
        let eq52_e1183_q_d_n5: f64 = (-eq52_e1181_d_n5);
        let eq52_e1183_q_d_n6: f64 = (-eq52_e1181_d_n6);
        let eq52_e1183_q_d_n7: f64 = (-eq52_e1181_d_n7);
        let eq52_e1183_q_d_n8: f64 = (-eq52_e1181_d_n8);
        let eq52_e1183_q_d_n9: f64 = (-eq52_e1181_d_n9);
        let eq52_e1183_q_d_n10: f64 = (-eq52_e1181_d_n10);
        let eq52_e1183_q_d_n11: f64 = (-eq52_e1181_d_n11);
        let eq52_e1183_q_d_n12: f64 = (-eq52_e1181_d_n12);
        stamper.stamp_current_reactive(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq52_e1183_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq52_e1183_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq52_e1183_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq52_e1183_q_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq52_e1183_q_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq52_e1183_q_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq52_e1183_q_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq52_e1183_q_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq52_e1183_q_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq52_e1183_q_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq52_e1183_q_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq52_e1183_q_d_n11)),
                GeneratedDerivative::node(self.nodes[12], self.multiplicity * (eq52_e1183_q_d_n12)),
            ],
        );
    }
}
