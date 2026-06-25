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
            Some(self.nodes[5]),
            self.branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[2]),
            Some(self.nodes[6]),
            self.branches[1],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[0]),
            Some(self.nodes[7]),
            self.branches[2],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[8]),
            Some(self.nodes[9]),
            self.branches[3],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[10]),
            Some(self.nodes[9]),
            self.branches[4],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[11]),
            Some(self.nodes[9]),
            self.branches[5],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[3]),
            Some(self.nodes[9]),
            self.branches[6],
            self.multiplicity,
        );

        let (eq0_e827, eq0_e827_d_n0, eq0_e827_d_n1, eq0_e827_d_n2, eq0_e827_d_n3, eq0_e827_d_n4, eq0_e827_d_n5, eq0_e827_d_n6, eq0_e827_d_n7, eq0_e827_d_n8, eq0_e827_d_n9, eq0_e827_d_n10, eq0_e827_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2580] != 0.0) {
        let eq0_e823: f64 = (scratch.values[0] * scratch.values[25]);
        let eq0_e823_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq0_e823_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq0_e823_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq0_e823_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq0_e823_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq0_e823_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq0_e823_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq0_e823_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq0_e823_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq0_e823_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq0_e823_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq0_e823_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq0_e825: f64 = (eq0_e823 * scratch.values[2022]);
        let eq0_e825_d_n0: f64 = ((eq0_e823_d_n0 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][0]));
        let eq0_e825_d_n1: f64 = ((eq0_e823_d_n1 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][1]));
        let eq0_e825_d_n2: f64 = ((eq0_e823_d_n2 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][2]));
        let eq0_e825_d_n3: f64 = ((eq0_e823_d_n3 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][3]));
        let eq0_e825_d_n4: f64 = ((eq0_e823_d_n4 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][4]));
        let eq0_e825_d_n5: f64 = ((eq0_e823_d_n5 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][5]));
        let eq0_e825_d_n6: f64 = ((eq0_e823_d_n6 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][6]));
        let eq0_e825_d_n7: f64 = ((eq0_e823_d_n7 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][7]));
        let eq0_e825_d_n8: f64 = ((eq0_e823_d_n8 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][8]));
        let eq0_e825_d_n9: f64 = ((eq0_e823_d_n9 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][9]));
        let eq0_e825_d_n10: f64 = ((eq0_e823_d_n10 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][10]));
        let eq0_e825_d_n11: f64 = ((eq0_e823_d_n11 * scratch.values[2022]) + (eq0_e823 * scratch.node_derivatives[2022][11]));
        (eq0_e825, eq0_e825_d_n0, eq0_e825_d_n1, eq0_e825_d_n2, eq0_e825_d_n3, eq0_e825_d_n4, eq0_e825_d_n5, eq0_e825_d_n6, eq0_e825_d_n7, eq0_e825_d_n8, eq0_e825_d_n9, eq0_e825_d_n10, eq0_e825_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e827;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[8]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq0_e827_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq0_e827_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq0_e827_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq0_e827_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq0_e827_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq0_e827_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq0_e827_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq0_e827_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq0_e827_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq0_e827_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq0_e827_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq0_e827_d_n11),
            ],
        );
        let (eq1_e837, eq1_e837_d_n0, eq1_e837_d_n1, eq1_e837_d_n2, eq1_e837_d_n3, eq1_e837_d_n4, eq1_e837_d_n5, eq1_e837_d_n6, eq1_e837_d_n7, eq1_e837_d_n8, eq1_e837_d_n9, eq1_e837_d_n10, eq1_e837_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2580] != 0.0) {
        let eq1_e831: f64 = (scratch.values[0] * scratch.values[25]);
        let eq1_e831_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq1_e831_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq1_e831_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq1_e831_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq1_e831_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq1_e831_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq1_e831_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq1_e831_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq1_e831_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq1_e831_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq1_e831_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq1_e831_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq1_e834: f64 = (scratch.values[2021] + scratch.values[2072]);
        let eq1_e834_d_n0: f64 = (scratch.node_derivatives[2021][0] + scratch.node_derivatives[2072][0]);
        let eq1_e834_d_n1: f64 = (scratch.node_derivatives[2021][1] + scratch.node_derivatives[2072][1]);
        let eq1_e834_d_n2: f64 = (scratch.node_derivatives[2021][2] + scratch.node_derivatives[2072][2]);
        let eq1_e834_d_n3: f64 = (scratch.node_derivatives[2021][3] + scratch.node_derivatives[2072][3]);
        let eq1_e834_d_n4: f64 = (scratch.node_derivatives[2021][4] + scratch.node_derivatives[2072][4]);
        let eq1_e834_d_n5: f64 = (scratch.node_derivatives[2021][5] + scratch.node_derivatives[2072][5]);
        let eq1_e834_d_n6: f64 = (scratch.node_derivatives[2021][6] + scratch.node_derivatives[2072][6]);
        let eq1_e834_d_n7: f64 = (scratch.node_derivatives[2021][7] + scratch.node_derivatives[2072][7]);
        let eq1_e834_d_n8: f64 = (scratch.node_derivatives[2021][8] + scratch.node_derivatives[2072][8]);
        let eq1_e834_d_n9: f64 = (scratch.node_derivatives[2021][9] + scratch.node_derivatives[2072][9]);
        let eq1_e834_d_n10: f64 = (scratch.node_derivatives[2021][10] + scratch.node_derivatives[2072][10]);
        let eq1_e834_d_n11: f64 = (scratch.node_derivatives[2021][11] + scratch.node_derivatives[2072][11]);
        let eq1_e835: f64 = (eq1_e831 * eq1_e834);
        let eq1_e835_d_n0: f64 = ((eq1_e831_d_n0 * eq1_e834) + (eq1_e831 * eq1_e834_d_n0));
        let eq1_e835_d_n1: f64 = ((eq1_e831_d_n1 * eq1_e834) + (eq1_e831 * eq1_e834_d_n1));
        let eq1_e835_d_n2: f64 = ((eq1_e831_d_n2 * eq1_e834) + (eq1_e831 * eq1_e834_d_n2));
        let eq1_e835_d_n3: f64 = ((eq1_e831_d_n3 * eq1_e834) + (eq1_e831 * eq1_e834_d_n3));
        let eq1_e835_d_n4: f64 = ((eq1_e831_d_n4 * eq1_e834) + (eq1_e831 * eq1_e834_d_n4));
        let eq1_e835_d_n5: f64 = ((eq1_e831_d_n5 * eq1_e834) + (eq1_e831 * eq1_e834_d_n5));
        let eq1_e835_d_n6: f64 = ((eq1_e831_d_n6 * eq1_e834) + (eq1_e831 * eq1_e834_d_n6));
        let eq1_e835_d_n7: f64 = ((eq1_e831_d_n7 * eq1_e834) + (eq1_e831 * eq1_e834_d_n7));
        let eq1_e835_d_n8: f64 = ((eq1_e831_d_n8 * eq1_e834) + (eq1_e831 * eq1_e834_d_n8));
        let eq1_e835_d_n9: f64 = ((eq1_e831_d_n9 * eq1_e834) + (eq1_e831 * eq1_e834_d_n9));
        let eq1_e835_d_n10: f64 = ((eq1_e831_d_n10 * eq1_e834) + (eq1_e831 * eq1_e834_d_n10));
        let eq1_e835_d_n11: f64 = ((eq1_e831_d_n11 * eq1_e834) + (eq1_e831 * eq1_e834_d_n11));
        (eq1_e835, eq1_e835_d_n0, eq1_e835_d_n1, eq1_e835_d_n2, eq1_e835_d_n3, eq1_e835_d_n4, eq1_e835_d_n5, eq1_e835_d_n6, eq1_e835_d_n7, eq1_e835_d_n8, eq1_e835_d_n9, eq1_e835_d_n10, eq1_e835_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e837;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq1_e837_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq1_e837_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq1_e837_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq1_e837_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq1_e837_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq1_e837_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq1_e837_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq1_e837_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq1_e837_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq1_e837_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq1_e837_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq1_e837_d_n11),
            ],
        );
        let (eq2_e845, eq2_e845_d_n0, eq2_e845_d_n1, eq2_e845_d_n2, eq2_e845_d_n3, eq2_e845_d_n4, eq2_e845_d_n5, eq2_e845_d_n6, eq2_e845_d_n7, eq2_e845_d_n8, eq2_e845_d_n9, eq2_e845_d_n10, eq2_e845_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2580] != 0.0) {
        let eq2_e841: f64 = (scratch.values[0] * scratch.values[25]);
        let eq2_e841_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq2_e841_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq2_e841_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq2_e841_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq2_e841_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq2_e841_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq2_e841_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq2_e841_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq2_e841_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq2_e841_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq2_e841_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq2_e841_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq2_e843: f64 = (eq2_e841 * scratch.values[2027]);
        let eq2_e843_d_n0: f64 = ((eq2_e841_d_n0 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][0]));
        let eq2_e843_d_n1: f64 = ((eq2_e841_d_n1 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][1]));
        let eq2_e843_d_n2: f64 = ((eq2_e841_d_n2 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][2]));
        let eq2_e843_d_n3: f64 = ((eq2_e841_d_n3 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][3]));
        let eq2_e843_d_n4: f64 = ((eq2_e841_d_n4 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][4]));
        let eq2_e843_d_n5: f64 = ((eq2_e841_d_n5 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][5]));
        let eq2_e843_d_n6: f64 = ((eq2_e841_d_n6 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][6]));
        let eq2_e843_d_n7: f64 = ((eq2_e841_d_n7 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][7]));
        let eq2_e843_d_n8: f64 = ((eq2_e841_d_n8 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][8]));
        let eq2_e843_d_n9: f64 = ((eq2_e841_d_n9 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][9]));
        let eq2_e843_d_n10: f64 = ((eq2_e841_d_n10 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][10]));
        let eq2_e843_d_n11: f64 = ((eq2_e841_d_n11 * scratch.values[2027]) + (eq2_e841 * scratch.node_derivatives[2027][11]));
        (eq2_e843, eq2_e843_d_n0, eq2_e843_d_n1, eq2_e843_d_n2, eq2_e843_d_n3, eq2_e843_d_n4, eq2_e843_d_n5, eq2_e843_d_n6, eq2_e843_d_n7, eq2_e843_d_n8, eq2_e843_d_n9, eq2_e843_d_n10, eq2_e843_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e845;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq2_e845_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq2_e845_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq2_e845_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq2_e845_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq2_e845_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq2_e845_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq2_e845_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq2_e845_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq2_e845_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq2_e845_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq2_e845_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq2_e845_d_n11),
            ],
        );
        let (eq3_e853, eq3_e853_d_n0, eq3_e853_d_n1, eq3_e853_d_n2, eq3_e853_d_n3, eq3_e853_d_n4, eq3_e853_d_n5, eq3_e853_d_n6, eq3_e853_d_n7, eq3_e853_d_n8, eq3_e853_d_n9, eq3_e853_d_n10, eq3_e853_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2580] != 0.0) {
        let eq3_e849: f64 = (scratch.values[0] * scratch.values[25]);
        let eq3_e849_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq3_e849_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq3_e849_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq3_e849_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq3_e849_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq3_e849_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq3_e849_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq3_e849_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq3_e849_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq3_e849_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq3_e849_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq3_e849_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq3_e851: f64 = (eq3_e849 * scratch.values[2026]);
        let eq3_e851_d_n0: f64 = ((eq3_e849_d_n0 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][0]));
        let eq3_e851_d_n1: f64 = ((eq3_e849_d_n1 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][1]));
        let eq3_e851_d_n2: f64 = ((eq3_e849_d_n2 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][2]));
        let eq3_e851_d_n3: f64 = ((eq3_e849_d_n3 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][3]));
        let eq3_e851_d_n4: f64 = ((eq3_e849_d_n4 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][4]));
        let eq3_e851_d_n5: f64 = ((eq3_e849_d_n5 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][5]));
        let eq3_e851_d_n6: f64 = ((eq3_e849_d_n6 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][6]));
        let eq3_e851_d_n7: f64 = ((eq3_e849_d_n7 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][7]));
        let eq3_e851_d_n8: f64 = ((eq3_e849_d_n8 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][8]));
        let eq3_e851_d_n9: f64 = ((eq3_e849_d_n9 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][9]));
        let eq3_e851_d_n10: f64 = ((eq3_e849_d_n10 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][10]));
        let eq3_e851_d_n11: f64 = ((eq3_e849_d_n11 * scratch.values[2026]) + (eq3_e849 * scratch.node_derivatives[2026][11]));
        (eq3_e851, eq3_e851_d_n0, eq3_e851_d_n1, eq3_e851_d_n2, eq3_e851_d_n3, eq3_e851_d_n4, eq3_e851_d_n5, eq3_e851_d_n6, eq3_e851_d_n7, eq3_e851_d_n8, eq3_e851_d_n9, eq3_e851_d_n10, eq3_e851_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e853;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq3_e853_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq3_e853_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq3_e853_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq3_e853_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq3_e853_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq3_e853_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq3_e853_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq3_e853_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq3_e853_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq3_e853_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq3_e853_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq3_e853_d_n11),
            ],
        );
        let (eq4_e862, eq4_e862_d_n0, eq4_e862_d_n1, eq4_e862_d_n2, eq4_e862_d_n3, eq4_e862_d_n4, eq4_e862_d_n5, eq4_e862_d_n6, eq4_e862_d_n7, eq4_e862_d_n8, eq4_e862_d_n9, eq4_e862_d_n10, eq4_e862_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2580] != 0.0)) {
        let eq4_e858: f64 = (scratch.values[0] * scratch.values[25]);
        let eq4_e858_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq4_e858_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq4_e858_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq4_e858_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq4_e858_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq4_e858_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq4_e858_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq4_e858_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq4_e858_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq4_e858_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq4_e858_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq4_e858_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq4_e860: f64 = (eq4_e858 * scratch.values[2022]);
        let eq4_e860_d_n0: f64 = ((eq4_e858_d_n0 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][0]));
        let eq4_e860_d_n1: f64 = ((eq4_e858_d_n1 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][1]));
        let eq4_e860_d_n2: f64 = ((eq4_e858_d_n2 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][2]));
        let eq4_e860_d_n3: f64 = ((eq4_e858_d_n3 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][3]));
        let eq4_e860_d_n4: f64 = ((eq4_e858_d_n4 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][4]));
        let eq4_e860_d_n5: f64 = ((eq4_e858_d_n5 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][5]));
        let eq4_e860_d_n6: f64 = ((eq4_e858_d_n6 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][6]));
        let eq4_e860_d_n7: f64 = ((eq4_e858_d_n7 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][7]));
        let eq4_e860_d_n8: f64 = ((eq4_e858_d_n8 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][8]));
        let eq4_e860_d_n9: f64 = ((eq4_e858_d_n9 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][9]));
        let eq4_e860_d_n10: f64 = ((eq4_e858_d_n10 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][10]));
        let eq4_e860_d_n11: f64 = ((eq4_e858_d_n11 * scratch.values[2022]) + (eq4_e858 * scratch.node_derivatives[2022][11]));
        (eq4_e860, eq4_e860_d_n0, eq4_e860_d_n1, eq4_e860_d_n2, eq4_e860_d_n3, eq4_e860_d_n4, eq4_e860_d_n5, eq4_e860_d_n6, eq4_e860_d_n7, eq4_e860_d_n8, eq4_e860_d_n9, eq4_e860_d_n10, eq4_e860_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e862;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq4_e862_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq4_e862_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq4_e862_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq4_e862_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq4_e862_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq4_e862_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq4_e862_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq4_e862_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq4_e862_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq4_e862_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq4_e862_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq4_e862_d_n11),
            ],
        );
        let (eq5_e873, eq5_e873_d_n0, eq5_e873_d_n1, eq5_e873_d_n2, eq5_e873_d_n3, eq5_e873_d_n4, eq5_e873_d_n5, eq5_e873_d_n6, eq5_e873_d_n7, eq5_e873_d_n8, eq5_e873_d_n9, eq5_e873_d_n10, eq5_e873_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2580] != 0.0)) {
        let eq5_e867: f64 = (scratch.values[0] * scratch.values[25]);
        let eq5_e867_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq5_e867_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq5_e867_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq5_e867_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq5_e867_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq5_e867_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq5_e867_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq5_e867_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq5_e867_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq5_e867_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq5_e867_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq5_e867_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq5_e870: f64 = (scratch.values[2021] + scratch.values[2072]);
        let eq5_e870_d_n0: f64 = (scratch.node_derivatives[2021][0] + scratch.node_derivatives[2072][0]);
        let eq5_e870_d_n1: f64 = (scratch.node_derivatives[2021][1] + scratch.node_derivatives[2072][1]);
        let eq5_e870_d_n2: f64 = (scratch.node_derivatives[2021][2] + scratch.node_derivatives[2072][2]);
        let eq5_e870_d_n3: f64 = (scratch.node_derivatives[2021][3] + scratch.node_derivatives[2072][3]);
        let eq5_e870_d_n4: f64 = (scratch.node_derivatives[2021][4] + scratch.node_derivatives[2072][4]);
        let eq5_e870_d_n5: f64 = (scratch.node_derivatives[2021][5] + scratch.node_derivatives[2072][5]);
        let eq5_e870_d_n6: f64 = (scratch.node_derivatives[2021][6] + scratch.node_derivatives[2072][6]);
        let eq5_e870_d_n7: f64 = (scratch.node_derivatives[2021][7] + scratch.node_derivatives[2072][7]);
        let eq5_e870_d_n8: f64 = (scratch.node_derivatives[2021][8] + scratch.node_derivatives[2072][8]);
        let eq5_e870_d_n9: f64 = (scratch.node_derivatives[2021][9] + scratch.node_derivatives[2072][9]);
        let eq5_e870_d_n10: f64 = (scratch.node_derivatives[2021][10] + scratch.node_derivatives[2072][10]);
        let eq5_e870_d_n11: f64 = (scratch.node_derivatives[2021][11] + scratch.node_derivatives[2072][11]);
        let eq5_e871: f64 = (eq5_e867 * eq5_e870);
        let eq5_e871_d_n0: f64 = ((eq5_e867_d_n0 * eq5_e870) + (eq5_e867 * eq5_e870_d_n0));
        let eq5_e871_d_n1: f64 = ((eq5_e867_d_n1 * eq5_e870) + (eq5_e867 * eq5_e870_d_n1));
        let eq5_e871_d_n2: f64 = ((eq5_e867_d_n2 * eq5_e870) + (eq5_e867 * eq5_e870_d_n2));
        let eq5_e871_d_n3: f64 = ((eq5_e867_d_n3 * eq5_e870) + (eq5_e867 * eq5_e870_d_n3));
        let eq5_e871_d_n4: f64 = ((eq5_e867_d_n4 * eq5_e870) + (eq5_e867 * eq5_e870_d_n4));
        let eq5_e871_d_n5: f64 = ((eq5_e867_d_n5 * eq5_e870) + (eq5_e867 * eq5_e870_d_n5));
        let eq5_e871_d_n6: f64 = ((eq5_e867_d_n6 * eq5_e870) + (eq5_e867 * eq5_e870_d_n6));
        let eq5_e871_d_n7: f64 = ((eq5_e867_d_n7 * eq5_e870) + (eq5_e867 * eq5_e870_d_n7));
        let eq5_e871_d_n8: f64 = ((eq5_e867_d_n8 * eq5_e870) + (eq5_e867 * eq5_e870_d_n8));
        let eq5_e871_d_n9: f64 = ((eq5_e867_d_n9 * eq5_e870) + (eq5_e867 * eq5_e870_d_n9));
        let eq5_e871_d_n10: f64 = ((eq5_e867_d_n10 * eq5_e870) + (eq5_e867 * eq5_e870_d_n10));
        let eq5_e871_d_n11: f64 = ((eq5_e867_d_n11 * eq5_e870) + (eq5_e867 * eq5_e870_d_n11));
        (eq5_e871, eq5_e871_d_n0, eq5_e871_d_n1, eq5_e871_d_n2, eq5_e871_d_n3, eq5_e871_d_n4, eq5_e871_d_n5, eq5_e871_d_n6, eq5_e871_d_n7, eq5_e871_d_n8, eq5_e871_d_n9, eq5_e871_d_n10, eq5_e871_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e873;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[7]),
            self.multiplicity * (eq5_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq5_e873_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq5_e873_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq5_e873_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq5_e873_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq5_e873_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq5_e873_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq5_e873_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq5_e873_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq5_e873_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq5_e873_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq5_e873_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq5_e873_d_n11),
            ],
        );
        let (eq6_e882, eq6_e882_d_n0, eq6_e882_d_n1, eq6_e882_d_n2, eq6_e882_d_n3, eq6_e882_d_n4, eq6_e882_d_n5, eq6_e882_d_n6, eq6_e882_d_n7, eq6_e882_d_n8, eq6_e882_d_n9, eq6_e882_d_n10, eq6_e882_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2580] != 0.0)) {
        let eq6_e878: f64 = (scratch.values[0] * scratch.values[25]);
        let eq6_e878_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq6_e878_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq6_e878_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq6_e878_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq6_e878_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq6_e878_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq6_e878_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq6_e878_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq6_e878_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq6_e878_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq6_e878_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq6_e878_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq6_e880: f64 = (eq6_e878 * scratch.values[2027]);
        let eq6_e880_d_n0: f64 = ((eq6_e878_d_n0 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][0]));
        let eq6_e880_d_n1: f64 = ((eq6_e878_d_n1 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][1]));
        let eq6_e880_d_n2: f64 = ((eq6_e878_d_n2 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][2]));
        let eq6_e880_d_n3: f64 = ((eq6_e878_d_n3 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][3]));
        let eq6_e880_d_n4: f64 = ((eq6_e878_d_n4 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][4]));
        let eq6_e880_d_n5: f64 = ((eq6_e878_d_n5 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][5]));
        let eq6_e880_d_n6: f64 = ((eq6_e878_d_n6 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][6]));
        let eq6_e880_d_n7: f64 = ((eq6_e878_d_n7 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][7]));
        let eq6_e880_d_n8: f64 = ((eq6_e878_d_n8 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][8]));
        let eq6_e880_d_n9: f64 = ((eq6_e878_d_n9 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][9]));
        let eq6_e880_d_n10: f64 = ((eq6_e878_d_n10 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][10]));
        let eq6_e880_d_n11: f64 = ((eq6_e878_d_n11 * scratch.values[2027]) + (eq6_e878 * scratch.node_derivatives[2027][11]));
        (eq6_e880, eq6_e880_d_n0, eq6_e880_d_n1, eq6_e880_d_n2, eq6_e880_d_n3, eq6_e880_d_n4, eq6_e880_d_n5, eq6_e880_d_n6, eq6_e880_d_n7, eq6_e880_d_n8, eq6_e880_d_n9, eq6_e880_d_n10, eq6_e880_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e882;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq6_e882_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq6_e882_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq6_e882_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq6_e882_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq6_e882_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq6_e882_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq6_e882_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq6_e882_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq6_e882_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq6_e882_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq6_e882_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq6_e882_d_n11),
            ],
        );
        let (eq7_e891, eq7_e891_d_n0, eq7_e891_d_n1, eq7_e891_d_n2, eq7_e891_d_n3, eq7_e891_d_n4, eq7_e891_d_n5, eq7_e891_d_n6, eq7_e891_d_n7, eq7_e891_d_n8, eq7_e891_d_n9, eq7_e891_d_n10, eq7_e891_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[2580] != 0.0)) {
        let eq7_e887: f64 = (scratch.values[0] * scratch.values[25]);
        let eq7_e887_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq7_e887_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq7_e887_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq7_e887_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq7_e887_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq7_e887_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq7_e887_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq7_e887_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq7_e887_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq7_e887_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq7_e887_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq7_e887_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq7_e889: f64 = (eq7_e887 * scratch.values[2026]);
        let eq7_e889_d_n0: f64 = ((eq7_e887_d_n0 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][0]));
        let eq7_e889_d_n1: f64 = ((eq7_e887_d_n1 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][1]));
        let eq7_e889_d_n2: f64 = ((eq7_e887_d_n2 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][2]));
        let eq7_e889_d_n3: f64 = ((eq7_e887_d_n3 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][3]));
        let eq7_e889_d_n4: f64 = ((eq7_e887_d_n4 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][4]));
        let eq7_e889_d_n5: f64 = ((eq7_e887_d_n5 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][5]));
        let eq7_e889_d_n6: f64 = ((eq7_e887_d_n6 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][6]));
        let eq7_e889_d_n7: f64 = ((eq7_e887_d_n7 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][7]));
        let eq7_e889_d_n8: f64 = ((eq7_e887_d_n8 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][8]));
        let eq7_e889_d_n9: f64 = ((eq7_e887_d_n9 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][9]));
        let eq7_e889_d_n10: f64 = ((eq7_e887_d_n10 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][10]));
        let eq7_e889_d_n11: f64 = ((eq7_e887_d_n11 * scratch.values[2026]) + (eq7_e887 * scratch.node_derivatives[2026][11]));
        (eq7_e889, eq7_e889_d_n0, eq7_e889_d_n1, eq7_e889_d_n2, eq7_e889_d_n3, eq7_e889_d_n4, eq7_e889_d_n5, eq7_e889_d_n6, eq7_e889_d_n7, eq7_e889_d_n8, eq7_e889_d_n9, eq7_e889_d_n10, eq7_e889_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e891;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq7_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq7_e891_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq7_e891_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq7_e891_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq7_e891_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq7_e891_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq7_e891_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq7_e891_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq7_e891_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq7_e891_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq7_e891_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq7_e891_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq7_e891_d_n11),
            ],
        );
        let eq8_e894: f64 = (scratch.values[0] * scratch.values[25]);
        let eq8_e894_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq8_e894_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq8_e894_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq8_e894_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq8_e894_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq8_e894_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq8_e894_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq8_e894_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq8_e894_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq8_e894_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq8_e894_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq8_e894_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq8_e896: f64 = (eq8_e894 * scratch.values[922]);
        let eq8_e896_d_n0: f64 = ((eq8_e894_d_n0 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][0]));
        let eq8_e896_d_n1: f64 = ((eq8_e894_d_n1 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][1]));
        let eq8_e896_d_n2: f64 = ((eq8_e894_d_n2 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][2]));
        let eq8_e896_d_n3: f64 = ((eq8_e894_d_n3 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][3]));
        let eq8_e896_d_n4: f64 = ((eq8_e894_d_n4 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][4]));
        let eq8_e896_d_n5: f64 = ((eq8_e894_d_n5 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][5]));
        let eq8_e896_d_n6: f64 = ((eq8_e894_d_n6 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][6]));
        let eq8_e896_d_n7: f64 = ((eq8_e894_d_n7 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][7]));
        let eq8_e896_d_n8: f64 = ((eq8_e894_d_n8 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][8]));
        let eq8_e896_d_n9: f64 = ((eq8_e894_d_n9 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][9]));
        let eq8_e896_d_n10: f64 = ((eq8_e894_d_n10 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][10]));
        let eq8_e896_d_n11: f64 = ((eq8_e894_d_n11 * scratch.values[922]) + (eq8_e894 * scratch.node_derivatives[922][11]));
        let eq8_value: f64 = eq8_e896;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[8]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq8_e896_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq8_e896_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq8_e896_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq8_e896_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq8_e896_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq8_e896_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq8_e896_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq8_e896_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq8_e896_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq8_e896_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq8_e896_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq8_e896_d_n11),
            ],
        );
        let eq9_e899: f64 = (scratch.values[0] * scratch.values[25]);
        let eq9_e899_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq9_e899_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq9_e899_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq9_e899_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq9_e899_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq9_e899_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq9_e899_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq9_e899_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq9_e899_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq9_e899_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq9_e899_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq9_e899_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq9_e901: f64 = (eq9_e899 * scratch.values[2025]);
        let eq9_e901_d_n0: f64 = ((eq9_e899_d_n0 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][0]));
        let eq9_e901_d_n1: f64 = ((eq9_e899_d_n1 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][1]));
        let eq9_e901_d_n2: f64 = ((eq9_e899_d_n2 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][2]));
        let eq9_e901_d_n3: f64 = ((eq9_e899_d_n3 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][3]));
        let eq9_e901_d_n4: f64 = ((eq9_e899_d_n4 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][4]));
        let eq9_e901_d_n5: f64 = ((eq9_e899_d_n5 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][5]));
        let eq9_e901_d_n6: f64 = ((eq9_e899_d_n6 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][6]));
        let eq9_e901_d_n7: f64 = ((eq9_e899_d_n7 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][7]));
        let eq9_e901_d_n8: f64 = ((eq9_e899_d_n8 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][8]));
        let eq9_e901_d_n9: f64 = ((eq9_e899_d_n9 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][9]));
        let eq9_e901_d_n10: f64 = ((eq9_e899_d_n10 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][10]));
        let eq9_e901_d_n11: f64 = ((eq9_e899_d_n11 * scratch.values[2025]) + (eq9_e899 * scratch.node_derivatives[2025][11]));
        let eq9_value: f64 = eq9_e901;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq9_e901_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq9_e901_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq9_e901_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq9_e901_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq9_e901_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq9_e901_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq9_e901_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq9_e901_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq9_e901_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq9_e901_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq9_e901_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq9_e901_d_n11),
            ],
        );
        let eq10_e904: f64 = (scratch.values[0] * scratch.values[25]);
        let eq10_e904_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq10_e904_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq10_e904_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq10_e904_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq10_e904_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq10_e904_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq10_e904_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq10_e904_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq10_e904_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq10_e904_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq10_e904_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq10_e904_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq10_e906: f64 = (eq10_e904 * scratch.values[2024]);
        let eq10_e906_d_n0: f64 = ((eq10_e904_d_n0 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][0]));
        let eq10_e906_d_n1: f64 = ((eq10_e904_d_n1 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][1]));
        let eq10_e906_d_n2: f64 = ((eq10_e904_d_n2 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][2]));
        let eq10_e906_d_n3: f64 = ((eq10_e904_d_n3 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][3]));
        let eq10_e906_d_n4: f64 = ((eq10_e904_d_n4 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][4]));
        let eq10_e906_d_n5: f64 = ((eq10_e904_d_n5 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][5]));
        let eq10_e906_d_n6: f64 = ((eq10_e904_d_n6 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][6]));
        let eq10_e906_d_n7: f64 = ((eq10_e904_d_n7 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][7]));
        let eq10_e906_d_n8: f64 = ((eq10_e904_d_n8 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][8]));
        let eq10_e906_d_n9: f64 = ((eq10_e904_d_n9 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][9]));
        let eq10_e906_d_n10: f64 = ((eq10_e904_d_n10 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][10]));
        let eq10_e906_d_n11: f64 = ((eq10_e904_d_n11 * scratch.values[2024]) + (eq10_e904 * scratch.node_derivatives[2024][11]));
        let eq10_value: f64 = eq10_e906;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq10_e906_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq10_e906_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq10_e906_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq10_e906_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq10_e906_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq10_e906_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq10_e906_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq10_e906_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq10_e906_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq10_e906_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq10_e906_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq10_e906_d_n11),
            ],
        );
        let eq11_e909: f64 = (scratch.values[0] * scratch.values[25]);
        let eq11_e909_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq11_e909_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq11_e909_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq11_e909_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq11_e909_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq11_e909_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq11_e909_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq11_e909_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq11_e909_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq11_e909_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq11_e909_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq11_e909_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq11_e911: f64 = (eq11_e909 * scratch.values[926]);
        let eq11_e911_d_n0: f64 = ((eq11_e909_d_n0 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][0]));
        let eq11_e911_d_n1: f64 = ((eq11_e909_d_n1 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][1]));
        let eq11_e911_d_n2: f64 = ((eq11_e909_d_n2 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][2]));
        let eq11_e911_d_n3: f64 = ((eq11_e909_d_n3 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][3]));
        let eq11_e911_d_n4: f64 = ((eq11_e909_d_n4 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][4]));
        let eq11_e911_d_n5: f64 = ((eq11_e909_d_n5 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][5]));
        let eq11_e911_d_n6: f64 = ((eq11_e909_d_n6 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][6]));
        let eq11_e911_d_n7: f64 = ((eq11_e909_d_n7 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][7]));
        let eq11_e911_d_n8: f64 = ((eq11_e909_d_n8 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][8]));
        let eq11_e911_d_n9: f64 = ((eq11_e909_d_n9 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][9]));
        let eq11_e911_d_n10: f64 = ((eq11_e909_d_n10 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][10]));
        let eq11_e911_d_n11: f64 = ((eq11_e909_d_n11 * scratch.values[926]) + (eq11_e909 * scratch.node_derivatives[926][11]));
        let eq11_value: f64 = eq11_e911;
        stamper.stamp_current(
            Some(self.nodes[6]),
            Some(self.nodes[8]),
            self.multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq11_e911_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq11_e911_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq11_e911_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq11_e911_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq11_e911_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq11_e911_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq11_e911_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq11_e911_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq11_e911_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq11_e911_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq11_e911_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq11_e911_d_n11),
            ],
        );
        let eq12_e914: f64 = (scratch.values[0] * scratch.values[25]);
        let eq12_e914_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq12_e914_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq12_e914_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq12_e914_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq12_e914_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq12_e914_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq12_e914_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq12_e914_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq12_e914_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq12_e914_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq12_e914_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq12_e914_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq12_e916: f64 = (eq12_e914 * scratch.values[924]);
        let eq12_e916_d_n0: f64 = ((eq12_e914_d_n0 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][0]));
        let eq12_e916_d_n1: f64 = ((eq12_e914_d_n1 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][1]));
        let eq12_e916_d_n2: f64 = ((eq12_e914_d_n2 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][2]));
        let eq12_e916_d_n3: f64 = ((eq12_e914_d_n3 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][3]));
        let eq12_e916_d_n4: f64 = ((eq12_e914_d_n4 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][4]));
        let eq12_e916_d_n5: f64 = ((eq12_e914_d_n5 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][5]));
        let eq12_e916_d_n6: f64 = ((eq12_e914_d_n6 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][6]));
        let eq12_e916_d_n7: f64 = ((eq12_e914_d_n7 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][7]));
        let eq12_e916_d_n8: f64 = ((eq12_e914_d_n8 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][8]));
        let eq12_e916_d_n9: f64 = ((eq12_e914_d_n9 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][9]));
        let eq12_e916_d_n10: f64 = ((eq12_e914_d_n10 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][10]));
        let eq12_e916_d_n11: f64 = ((eq12_e914_d_n11 * scratch.values[924]) + (eq12_e914 * scratch.node_derivatives[924][11]));
        let eq12_value: f64 = eq12_e916;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[8]),
            self.multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq12_e916_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq12_e916_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq12_e916_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq12_e916_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq12_e916_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq12_e916_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq12_e916_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq12_e916_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq12_e916_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq12_e916_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq12_e916_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq12_e916_d_n11),
            ],
        );
        let eq13_e919: f64 = (scratch.values[0] * scratch.values[25]);
        let eq13_e919_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq13_e919_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq13_e919_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq13_e919_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq13_e919_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq13_e919_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq13_e919_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq13_e919_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq13_e919_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq13_e919_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq13_e919_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq13_e919_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq13_e921: f64 = (eq13_e919 * scratch.values[2033]);
        let eq13_e921_d_n0: f64 = ((eq13_e919_d_n0 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][0]));
        let eq13_e921_d_n1: f64 = ((eq13_e919_d_n1 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][1]));
        let eq13_e921_d_n2: f64 = ((eq13_e919_d_n2 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][2]));
        let eq13_e921_d_n3: f64 = ((eq13_e919_d_n3 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][3]));
        let eq13_e921_d_n4: f64 = ((eq13_e919_d_n4 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][4]));
        let eq13_e921_d_n5: f64 = ((eq13_e919_d_n5 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][5]));
        let eq13_e921_d_n6: f64 = ((eq13_e919_d_n6 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][6]));
        let eq13_e921_d_n7: f64 = ((eq13_e919_d_n7 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][7]));
        let eq13_e921_d_n8: f64 = ((eq13_e919_d_n8 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][8]));
        let eq13_e921_d_n9: f64 = ((eq13_e919_d_n9 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][9]));
        let eq13_e921_d_n10: f64 = ((eq13_e919_d_n10 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][10]));
        let eq13_e921_d_n11: f64 = ((eq13_e919_d_n11 * scratch.values[2033]) + (eq13_e919 * scratch.node_derivatives[2033][11]));
        let eq13_value: f64 = eq13_e921;
        stamper.stamp_current(
            Some(self.nodes[10]),
            Some(self.nodes[6]),
            self.multiplicity * (eq13_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq13_e921_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq13_e921_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq13_e921_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq13_e921_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq13_e921_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq13_e921_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq13_e921_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq13_e921_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq13_e921_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq13_e921_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq13_e921_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq13_e921_d_n11),
            ],
        );
        let eq14_e924: f64 = (scratch.values[0] * scratch.values[25]);
        let eq14_e924_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq14_e924_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq14_e924_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq14_e924_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq14_e924_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq14_e924_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq14_e924_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq14_e924_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq14_e924_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq14_e924_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq14_e924_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq14_e924_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq14_e926: f64 = (eq14_e924 * scratch.values[2037]);
        let eq14_e926_d_n0: f64 = ((eq14_e924_d_n0 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][0]));
        let eq14_e926_d_n1: f64 = ((eq14_e924_d_n1 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][1]));
        let eq14_e926_d_n2: f64 = ((eq14_e924_d_n2 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][2]));
        let eq14_e926_d_n3: f64 = ((eq14_e924_d_n3 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][3]));
        let eq14_e926_d_n4: f64 = ((eq14_e924_d_n4 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][4]));
        let eq14_e926_d_n5: f64 = ((eq14_e924_d_n5 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][5]));
        let eq14_e926_d_n6: f64 = ((eq14_e924_d_n6 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][6]));
        let eq14_e926_d_n7: f64 = ((eq14_e924_d_n7 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][7]));
        let eq14_e926_d_n8: f64 = ((eq14_e924_d_n8 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][8]));
        let eq14_e926_d_n9: f64 = ((eq14_e924_d_n9 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][9]));
        let eq14_e926_d_n10: f64 = ((eq14_e924_d_n10 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][10]));
        let eq14_e926_d_n11: f64 = ((eq14_e924_d_n11 * scratch.values[2037]) + (eq14_e924 * scratch.node_derivatives[2037][11]));
        let eq14_value: f64 = eq14_e926;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq14_e926_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq14_e926_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq14_e926_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq14_e926_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq14_e926_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq14_e926_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq14_e926_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq14_e926_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq14_e926_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq14_e926_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq14_e926_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq14_e926_d_n11),
            ],
        );
        let (eq15_e934, eq15_e934_d_n0, eq15_e934_d_n1, eq15_e934_d_n2, eq15_e934_d_n3, eq15_e934_d_n4, eq15_e934_d_n5, eq15_e934_d_n6, eq15_e934_d_n7, eq15_e934_d_n8, eq15_e934_d_n9, eq15_e934_d_n10, eq15_e934_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2581] != 0.0) {
        let eq15_e930: f64 = (scratch.values[25] * scratch.values[846]);
        let eq15_e930_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][0]));
        let eq15_e930_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][1]));
        let eq15_e930_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][2]));
        let eq15_e930_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][3]));
        let eq15_e930_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][4]));
        let eq15_e930_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][5]));
        let eq15_e930_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][6]));
        let eq15_e930_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][7]));
        let eq15_e930_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][8]));
        let eq15_e930_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][9]));
        let eq15_e930_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][10]));
        let eq15_e930_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[846]) + (scratch.values[25] * scratch.node_derivatives[846][11]));
        let eq15_e932: f64 = (eq15_e930 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n0: f64 = (eq15_e930_d_n0 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n1: f64 = ((eq15_e930_d_n1 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5]))) + eq15_e930);
        let eq15_e932_d_n2: f64 = (eq15_e930_d_n2 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n3: f64 = (eq15_e930_d_n3 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n4: f64 = (eq15_e930_d_n4 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n5: f64 = ((eq15_e930_d_n5 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5]))) + (eq15_e930 * -1.0));
        let eq15_e932_d_n6: f64 = (eq15_e930_d_n6 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n7: f64 = (eq15_e930_d_n7 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n8: f64 = (eq15_e930_d_n8 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n9: f64 = (eq15_e930_d_n9 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n10: f64 = (eq15_e930_d_n10 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        let eq15_e932_d_n11: f64 = (eq15_e930_d_n11 * (ctx.node_voltage(self.nodes[1]) - ctx.node_voltage(self.nodes[5])));
        (eq15_e932, eq15_e932_d_n0, eq15_e932_d_n1, eq15_e932_d_n2, eq15_e932_d_n3, eq15_e932_d_n4, eq15_e932_d_n5, eq15_e932_d_n6, eq15_e932_d_n7, eq15_e932_d_n8, eq15_e932_d_n9, eq15_e932_d_n10, eq15_e932_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e934;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            self.multiplicity * (eq15_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq15_e934_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq15_e934_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq15_e934_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq15_e934_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq15_e934_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq15_e934_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq15_e934_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq15_e934_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq15_e934_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq15_e934_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq15_e934_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq15_e934_d_n11),
            ],
        );
        let (eq16_e942,): (f64,) = {
    if (scratch.values[2581] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e942;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
        let (eq17_e947,): (f64,) = {
    if (!(scratch.values[2581] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e947;
        stamper.stamp_potential(
            self.branches[0],
            eq17_value,
            &[
            ],
        );
        let (eq18_e955, eq18_e955_d_n0, eq18_e955_d_n1, eq18_e955_d_n2, eq18_e955_d_n3, eq18_e955_d_n4, eq18_e955_d_n5, eq18_e955_d_n6, eq18_e955_d_n7, eq18_e955_d_n8, eq18_e955_d_n9, eq18_e955_d_n10, eq18_e955_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2582] != 0.0) {
        let eq18_e951: f64 = (scratch.values[25] * scratch.values[847]);
        let eq18_e951_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][0]));
        let eq18_e951_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][1]));
        let eq18_e951_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][2]));
        let eq18_e951_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][3]));
        let eq18_e951_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][4]));
        let eq18_e951_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][5]));
        let eq18_e951_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][6]));
        let eq18_e951_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][7]));
        let eq18_e951_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][8]));
        let eq18_e951_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][9]));
        let eq18_e951_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][10]));
        let eq18_e951_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[847]) + (scratch.values[25] * scratch.node_derivatives[847][11]));
        let eq18_e953: f64 = (eq18_e951 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n0: f64 = (eq18_e951_d_n0 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n1: f64 = (eq18_e951_d_n1 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n2: f64 = ((eq18_e951_d_n2 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6]))) + eq18_e951);
        let eq18_e953_d_n3: f64 = (eq18_e951_d_n3 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n4: f64 = (eq18_e951_d_n4 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n5: f64 = (eq18_e951_d_n5 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n6: f64 = ((eq18_e951_d_n6 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6]))) + (eq18_e951 * -1.0));
        let eq18_e953_d_n7: f64 = (eq18_e951_d_n7 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n8: f64 = (eq18_e951_d_n8 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n9: f64 = (eq18_e951_d_n9 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n10: f64 = (eq18_e951_d_n10 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        let eq18_e953_d_n11: f64 = (eq18_e951_d_n11 * (ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[6])));
        (eq18_e953, eq18_e953_d_n0, eq18_e953_d_n1, eq18_e953_d_n2, eq18_e953_d_n3, eq18_e953_d_n4, eq18_e953_d_n5, eq18_e953_d_n6, eq18_e953_d_n7, eq18_e953_d_n8, eq18_e953_d_n9, eq18_e953_d_n10, eq18_e953_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e955;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[6]),
            self.multiplicity * (eq18_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq18_e955_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq18_e955_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq18_e955_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq18_e955_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq18_e955_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq18_e955_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq18_e955_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq18_e955_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq18_e955_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq18_e955_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq18_e955_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq18_e955_d_n11),
            ],
        );
        let (eq19_e963,): (f64,) = {
    if (scratch.values[2582] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e963;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[6]),
            self.multiplicity * (eq19_value),
            &[
            ],
        );
        let (eq20_e968,): (f64,) = {
    if (!(scratch.values[2582] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e968;
        stamper.stamp_potential(
            self.branches[1],
            eq20_value,
            &[
            ],
        );
        let (eq21_e976, eq21_e976_d_n0, eq21_e976_d_n1, eq21_e976_d_n2, eq21_e976_d_n3, eq21_e976_d_n4, eq21_e976_d_n5, eq21_e976_d_n6, eq21_e976_d_n7, eq21_e976_d_n8, eq21_e976_d_n9, eq21_e976_d_n10, eq21_e976_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2583] != 0.0) {
        let eq21_e972: f64 = (scratch.values[25] * scratch.values[848]);
        let eq21_e972_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][0]));
        let eq21_e972_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][1]));
        let eq21_e972_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][2]));
        let eq21_e972_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][3]));
        let eq21_e972_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][4]));
        let eq21_e972_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][5]));
        let eq21_e972_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][6]));
        let eq21_e972_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][7]));
        let eq21_e972_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][8]));
        let eq21_e972_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][9]));
        let eq21_e972_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][10]));
        let eq21_e972_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[848]) + (scratch.values[25] * scratch.node_derivatives[848][11]));
        let eq21_e974: f64 = (eq21_e972 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n0: f64 = ((eq21_e972_d_n0 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7]))) + eq21_e972);
        let eq21_e974_d_n1: f64 = (eq21_e972_d_n1 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n2: f64 = (eq21_e972_d_n2 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n3: f64 = (eq21_e972_d_n3 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n4: f64 = (eq21_e972_d_n4 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n5: f64 = (eq21_e972_d_n5 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n6: f64 = (eq21_e972_d_n6 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n7: f64 = ((eq21_e972_d_n7 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7]))) + (eq21_e972 * -1.0));
        let eq21_e974_d_n8: f64 = (eq21_e972_d_n8 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n9: f64 = (eq21_e972_d_n9 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n10: f64 = (eq21_e972_d_n10 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        let eq21_e974_d_n11: f64 = (eq21_e972_d_n11 * (ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[7])));
        (eq21_e974, eq21_e974_d_n0, eq21_e974_d_n1, eq21_e974_d_n2, eq21_e974_d_n3, eq21_e974_d_n4, eq21_e974_d_n5, eq21_e974_d_n6, eq21_e974_d_n7, eq21_e974_d_n8, eq21_e974_d_n9, eq21_e974_d_n10, eq21_e974_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e976;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[7]),
            self.multiplicity * (eq21_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq21_e976_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq21_e976_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq21_e976_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq21_e976_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq21_e976_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq21_e976_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq21_e976_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq21_e976_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq21_e976_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq21_e976_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq21_e976_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq21_e976_d_n11),
            ],
        );
        let (eq22_e984,): (f64,) = {
    if (scratch.values[2583] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e984;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[7]),
            self.multiplicity * (eq22_value),
            &[
            ],
        );
        let (eq23_e989,): (f64,) = {
    if (!(scratch.values[2583] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e989;
        stamper.stamp_potential(
            self.branches[2],
            eq23_value,
            &[
            ],
        );
        let (eq24_e997, eq24_e997_d_n0, eq24_e997_d_n1, eq24_e997_d_n2, eq24_e997_d_n3, eq24_e997_d_n4, eq24_e997_d_n5, eq24_e997_d_n6, eq24_e997_d_n7, eq24_e997_d_n8, eq24_e997_d_n9, eq24_e997_d_n10, eq24_e997_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2584] != 0.0) {
        let eq24_e993: f64 = (scratch.values[25] * scratch.values[849]);
        let eq24_e993_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][0]));
        let eq24_e993_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][1]));
        let eq24_e993_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][2]));
        let eq24_e993_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][3]));
        let eq24_e993_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][4]));
        let eq24_e993_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][5]));
        let eq24_e993_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][6]));
        let eq24_e993_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][7]));
        let eq24_e993_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][8]));
        let eq24_e993_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][9]));
        let eq24_e993_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][10]));
        let eq24_e993_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[849]) + (scratch.values[25] * scratch.node_derivatives[849][11]));
        let eq24_e995: f64 = (eq24_e993 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n0: f64 = (eq24_e993_d_n0 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n1: f64 = (eq24_e993_d_n1 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n2: f64 = (eq24_e993_d_n2 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n3: f64 = (eq24_e993_d_n3 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n4: f64 = (eq24_e993_d_n4 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n5: f64 = (eq24_e993_d_n5 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n6: f64 = (eq24_e993_d_n6 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n7: f64 = (eq24_e993_d_n7 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n8: f64 = ((eq24_e993_d_n8 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9]))) + eq24_e993);
        let eq24_e995_d_n9: f64 = ((eq24_e993_d_n9 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9]))) + (eq24_e993 * -1.0));
        let eq24_e995_d_n10: f64 = (eq24_e993_d_n10 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        let eq24_e995_d_n11: f64 = (eq24_e993_d_n11 * (ctx.node_voltage(self.nodes[8]) - ctx.node_voltage(self.nodes[9])));
        (eq24_e995, eq24_e995_d_n0, eq24_e995_d_n1, eq24_e995_d_n2, eq24_e995_d_n3, eq24_e995_d_n4, eq24_e995_d_n5, eq24_e995_d_n6, eq24_e995_d_n7, eq24_e995_d_n8, eq24_e995_d_n9, eq24_e995_d_n10, eq24_e995_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e997;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[9]),
            self.multiplicity * (eq24_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq24_e997_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq24_e997_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq24_e997_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq24_e997_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq24_e997_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq24_e997_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq24_e997_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq24_e997_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq24_e997_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq24_e997_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq24_e997_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq24_e997_d_n11),
            ],
        );
        let (eq25_e1005,): (f64,) = {
    if (scratch.values[2584] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1005;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[9]),
            self.multiplicity * (eq25_value),
            &[
            ],
        );
        let (eq26_e1010,): (f64,) = {
    if (!(scratch.values[2584] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1010;
        stamper.stamp_potential(
            self.branches[3],
            eq26_value,
            &[
            ],
        );
        let (eq27_e1018, eq27_e1018_d_n0, eq27_e1018_d_n1, eq27_e1018_d_n2, eq27_e1018_d_n3, eq27_e1018_d_n4, eq27_e1018_d_n5, eq27_e1018_d_n6, eq27_e1018_d_n7, eq27_e1018_d_n8, eq27_e1018_d_n9, eq27_e1018_d_n10, eq27_e1018_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2585] != 0.0) {
        let eq27_e1014: f64 = (scratch.values[25] * scratch.values[850]);
        let eq27_e1014_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][0]));
        let eq27_e1014_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][1]));
        let eq27_e1014_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][2]));
        let eq27_e1014_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][3]));
        let eq27_e1014_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][4]));
        let eq27_e1014_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][5]));
        let eq27_e1014_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][6]));
        let eq27_e1014_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][7]));
        let eq27_e1014_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][8]));
        let eq27_e1014_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][9]));
        let eq27_e1014_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][10]));
        let eq27_e1014_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[850]) + (scratch.values[25] * scratch.node_derivatives[850][11]));
        let eq27_e1016: f64 = (eq27_e1014 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n0: f64 = (eq27_e1014_d_n0 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n1: f64 = (eq27_e1014_d_n1 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n2: f64 = (eq27_e1014_d_n2 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n3: f64 = (eq27_e1014_d_n3 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n4: f64 = (eq27_e1014_d_n4 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n5: f64 = (eq27_e1014_d_n5 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n6: f64 = (eq27_e1014_d_n6 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n7: f64 = (eq27_e1014_d_n7 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n8: f64 = (eq27_e1014_d_n8 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        let eq27_e1016_d_n9: f64 = ((eq27_e1014_d_n9 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9]))) + (eq27_e1014 * -1.0));
        let eq27_e1016_d_n10: f64 = ((eq27_e1014_d_n10 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9]))) + eq27_e1014);
        let eq27_e1016_d_n11: f64 = (eq27_e1014_d_n11 * (ctx.node_voltage(self.nodes[10]) - ctx.node_voltage(self.nodes[9])));
        (eq27_e1016, eq27_e1016_d_n0, eq27_e1016_d_n1, eq27_e1016_d_n2, eq27_e1016_d_n3, eq27_e1016_d_n4, eq27_e1016_d_n5, eq27_e1016_d_n6, eq27_e1016_d_n7, eq27_e1016_d_n8, eq27_e1016_d_n9, eq27_e1016_d_n10, eq27_e1016_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1018;
        stamper.stamp_current(
            Some(self.nodes[10]),
            Some(self.nodes[9]),
            self.multiplicity * (eq27_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq27_e1018_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq27_e1018_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq27_e1018_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq27_e1018_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq27_e1018_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq27_e1018_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq27_e1018_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq27_e1018_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq27_e1018_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq27_e1018_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq27_e1018_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq27_e1018_d_n11),
            ],
        );
        let (eq28_e1026,): (f64,) = {
    if (scratch.values[2585] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1026;
        stamper.stamp_current(
            Some(self.nodes[10]),
            Some(self.nodes[9]),
            self.multiplicity * (eq28_value),
            &[
            ],
        );
        let (eq29_e1031,): (f64,) = {
    if (!(scratch.values[2585] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1031;
        stamper.stamp_potential(
            self.branches[4],
            eq29_value,
            &[
            ],
        );
        let (eq30_e1039, eq30_e1039_d_n0, eq30_e1039_d_n1, eq30_e1039_d_n2, eq30_e1039_d_n3, eq30_e1039_d_n4, eq30_e1039_d_n5, eq30_e1039_d_n6, eq30_e1039_d_n7, eq30_e1039_d_n8, eq30_e1039_d_n9, eq30_e1039_d_n10, eq30_e1039_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2586] != 0.0) {
        let eq30_e1035: f64 = (scratch.values[25] * scratch.values[851]);
        let eq30_e1035_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][0]));
        let eq30_e1035_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][1]));
        let eq30_e1035_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][2]));
        let eq30_e1035_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][3]));
        let eq30_e1035_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][4]));
        let eq30_e1035_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][5]));
        let eq30_e1035_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][6]));
        let eq30_e1035_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][7]));
        let eq30_e1035_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][8]));
        let eq30_e1035_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][9]));
        let eq30_e1035_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][10]));
        let eq30_e1035_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[851]) + (scratch.values[25] * scratch.node_derivatives[851][11]));
        let eq30_e1037: f64 = (eq30_e1035 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n0: f64 = (eq30_e1035_d_n0 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n1: f64 = (eq30_e1035_d_n1 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n2: f64 = (eq30_e1035_d_n2 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n3: f64 = (eq30_e1035_d_n3 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n4: f64 = (eq30_e1035_d_n4 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n5: f64 = (eq30_e1035_d_n5 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n6: f64 = (eq30_e1035_d_n6 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n7: f64 = (eq30_e1035_d_n7 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n8: f64 = (eq30_e1035_d_n8 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n9: f64 = ((eq30_e1035_d_n9 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9]))) + (eq30_e1035 * -1.0));
        let eq30_e1037_d_n10: f64 = (eq30_e1035_d_n10 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9])));
        let eq30_e1037_d_n11: f64 = ((eq30_e1035_d_n11 * (ctx.node_voltage(self.nodes[11]) - ctx.node_voltage(self.nodes[9]))) + eq30_e1035);
        (eq30_e1037, eq30_e1037_d_n0, eq30_e1037_d_n1, eq30_e1037_d_n2, eq30_e1037_d_n3, eq30_e1037_d_n4, eq30_e1037_d_n5, eq30_e1037_d_n6, eq30_e1037_d_n7, eq30_e1037_d_n8, eq30_e1037_d_n9, eq30_e1037_d_n10, eq30_e1037_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1039;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[9]),
            self.multiplicity * (eq30_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq30_e1039_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq30_e1039_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq30_e1039_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq30_e1039_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq30_e1039_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq30_e1039_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq30_e1039_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq30_e1039_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq30_e1039_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq30_e1039_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq30_e1039_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq30_e1039_d_n11),
            ],
        );
        let (eq31_e1047,): (f64,) = {
    if (scratch.values[2586] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1047;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[9]),
            self.multiplicity * (eq31_value),
            &[
            ],
        );
        let (eq32_e1052,): (f64,) = {
    if (!(scratch.values[2586] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1052;
        stamper.stamp_potential(
            self.branches[5],
            eq32_value,
            &[
            ],
        );
        let (eq33_e1060, eq33_e1060_d_n0, eq33_e1060_d_n1, eq33_e1060_d_n2, eq33_e1060_d_n3, eq33_e1060_d_n4, eq33_e1060_d_n5, eq33_e1060_d_n6, eq33_e1060_d_n7, eq33_e1060_d_n8, eq33_e1060_d_n9, eq33_e1060_d_n10, eq33_e1060_d_n11,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[2587] != 0.0) {
        let eq33_e1056: f64 = (scratch.values[25] * scratch.values[852]);
        let eq33_e1056_d_n0: f64 = ((scratch.node_derivatives[25][0] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][0]));
        let eq33_e1056_d_n1: f64 = ((scratch.node_derivatives[25][1] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][1]));
        let eq33_e1056_d_n2: f64 = ((scratch.node_derivatives[25][2] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][2]));
        let eq33_e1056_d_n3: f64 = ((scratch.node_derivatives[25][3] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][3]));
        let eq33_e1056_d_n4: f64 = ((scratch.node_derivatives[25][4] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][4]));
        let eq33_e1056_d_n5: f64 = ((scratch.node_derivatives[25][5] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][5]));
        let eq33_e1056_d_n6: f64 = ((scratch.node_derivatives[25][6] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][6]));
        let eq33_e1056_d_n7: f64 = ((scratch.node_derivatives[25][7] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][7]));
        let eq33_e1056_d_n8: f64 = ((scratch.node_derivatives[25][8] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][8]));
        let eq33_e1056_d_n9: f64 = ((scratch.node_derivatives[25][9] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][9]));
        let eq33_e1056_d_n10: f64 = ((scratch.node_derivatives[25][10] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][10]));
        let eq33_e1056_d_n11: f64 = ((scratch.node_derivatives[25][11] * scratch.values[852]) + (scratch.values[25] * scratch.node_derivatives[852][11]));
        let eq33_e1058: f64 = (eq33_e1056 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n0: f64 = (eq33_e1056_d_n0 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n1: f64 = (eq33_e1056_d_n1 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n2: f64 = (eq33_e1056_d_n2 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n3: f64 = ((eq33_e1056_d_n3 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9]))) + eq33_e1056);
        let eq33_e1058_d_n4: f64 = (eq33_e1056_d_n4 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n5: f64 = (eq33_e1056_d_n5 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n6: f64 = (eq33_e1056_d_n6 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n7: f64 = (eq33_e1056_d_n7 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n8: f64 = (eq33_e1056_d_n8 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n9: f64 = ((eq33_e1056_d_n9 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9]))) + (eq33_e1056 * -1.0));
        let eq33_e1058_d_n10: f64 = (eq33_e1056_d_n10 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        let eq33_e1058_d_n11: f64 = (eq33_e1056_d_n11 * (ctx.node_voltage(self.nodes[3]) - ctx.node_voltage(self.nodes[9])));
        (eq33_e1058, eq33_e1058_d_n0, eq33_e1058_d_n1, eq33_e1058_d_n2, eq33_e1058_d_n3, eq33_e1058_d_n4, eq33_e1058_d_n5, eq33_e1058_d_n6, eq33_e1058_d_n7, eq33_e1058_d_n8, eq33_e1058_d_n9, eq33_e1058_d_n10, eq33_e1058_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1060;
        stamper.stamp_current(
            Some(self.nodes[3]),
            Some(self.nodes[9]),
            self.multiplicity * (eq33_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq33_e1060_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq33_e1060_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq33_e1060_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq33_e1060_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq33_e1060_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq33_e1060_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq33_e1060_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq33_e1060_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq33_e1060_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq33_e1060_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq33_e1060_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq33_e1060_d_n11),
            ],
        );
        let (eq34_e1068,): (f64,) = {
    if (scratch.values[2587] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1068;
        stamper.stamp_current(
            Some(self.nodes[3]),
            Some(self.nodes[9]),
            self.multiplicity * (eq34_value),
            &[
            ],
        );
        let (eq35_e1073,): (f64,) = {
    if (!(scratch.values[2587] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1073;
        stamper.stamp_potential(
            self.branches[6],
            eq35_value,
            &[
            ],
        );
        let eq36_e1076: f64 = (1e-15 * (ctx.node_voltage(self.nodes[7]) - ctx.node_voltage(self.nodes[6])));
        let eq36_e1076_d_n6: f64 = (1e-15 * -1.0);
        let eq36_e1076_d_n7: f64 = 1e-15;
        let eq36_value: f64 = eq36_e1076;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq36_value),
            &[
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq36_e1076_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq36_e1076_d_n7),
            ],
        );
        let eq37_e1079: f64 = (scratch.values[0] * scratch.values[25]);
        let eq37_e1079_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq37_e1079_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq37_e1079_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq37_e1079_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq37_e1079_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq37_e1079_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq37_e1079_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq37_e1079_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq37_e1079_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq37_e1079_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq37_e1079_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq37_e1079_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq37_e1081: f64 = (eq37_e1079 * scratch.values[944]);
        let eq37_e1081_d_n0: f64 = ((eq37_e1079_d_n0 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][0]));
        let eq37_e1081_d_n1: f64 = ((eq37_e1079_d_n1 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][1]));
        let eq37_e1081_d_n2: f64 = ((eq37_e1079_d_n2 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][2]));
        let eq37_e1081_d_n3: f64 = ((eq37_e1079_d_n3 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][3]));
        let eq37_e1081_d_n4: f64 = ((eq37_e1079_d_n4 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][4]));
        let eq37_e1081_d_n5: f64 = ((eq37_e1079_d_n5 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][5]));
        let eq37_e1081_d_n6: f64 = ((eq37_e1079_d_n6 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][6]));
        let eq37_e1081_d_n7: f64 = ((eq37_e1079_d_n7 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][7]));
        let eq37_e1081_d_n8: f64 = ((eq37_e1079_d_n8 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][8]));
        let eq37_e1081_d_n9: f64 = ((eq37_e1079_d_n9 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][9]));
        let eq37_e1081_d_n10: f64 = ((eq37_e1079_d_n10 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][10]));
        let eq37_e1081_d_n11: f64 = ((eq37_e1079_d_n11 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][11]));
        let eq37_e1082: f64 = self.eval_ddt(0, eq37_e1081);
        let eq37_e1082_d_n0: f64 = self.ddt_jacobian(eq37_e1081_d_n0);
        let eq37_e1082_d_n1: f64 = self.ddt_jacobian(eq37_e1081_d_n1);
        let eq37_e1082_d_n2: f64 = self.ddt_jacobian(eq37_e1081_d_n2);
        let eq37_e1082_d_n3: f64 = self.ddt_jacobian(eq37_e1081_d_n3);
        let eq37_e1082_d_n4: f64 = self.ddt_jacobian(eq37_e1081_d_n4);
        let eq37_e1082_d_n5: f64 = self.ddt_jacobian(eq37_e1081_d_n5);
        let eq37_e1082_d_n6: f64 = self.ddt_jacobian(eq37_e1081_d_n6);
        let eq37_e1082_d_n7: f64 = self.ddt_jacobian(eq37_e1081_d_n7);
        let eq37_e1082_d_n8: f64 = self.ddt_jacobian(eq37_e1081_d_n8);
        let eq37_e1082_d_n9: f64 = self.ddt_jacobian(eq37_e1081_d_n9);
        let eq37_e1082_d_n10: f64 = self.ddt_jacobian(eq37_e1081_d_n10);
        let eq37_e1082_d_n11: f64 = self.ddt_jacobian(eq37_e1081_d_n11);
        let eq37_value: f64 = eq37_e1082;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq37_e1082_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq37_e1082_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq37_e1082_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq37_e1082_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq37_e1082_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq37_e1082_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq37_e1082_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq37_e1082_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq37_e1082_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq37_e1082_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq37_e1082_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq37_e1082_d_n11),
            ],
        );
        let eq38_e1085: f64 = (scratch.values[0] * scratch.values[25]);
        let eq38_e1085_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq38_e1085_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq38_e1085_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq38_e1085_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq38_e1085_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq38_e1085_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq38_e1085_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq38_e1085_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq38_e1085_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq38_e1085_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq38_e1085_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq38_e1085_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq38_e1087: f64 = (eq38_e1085 * scratch.values[946]);
        let eq38_e1087_d_n0: f64 = ((eq38_e1085_d_n0 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][0]));
        let eq38_e1087_d_n1: f64 = ((eq38_e1085_d_n1 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][1]));
        let eq38_e1087_d_n2: f64 = ((eq38_e1085_d_n2 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][2]));
        let eq38_e1087_d_n3: f64 = ((eq38_e1085_d_n3 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][3]));
        let eq38_e1087_d_n4: f64 = ((eq38_e1085_d_n4 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][4]));
        let eq38_e1087_d_n5: f64 = ((eq38_e1085_d_n5 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][5]));
        let eq38_e1087_d_n6: f64 = ((eq38_e1085_d_n6 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][6]));
        let eq38_e1087_d_n7: f64 = ((eq38_e1085_d_n7 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][7]));
        let eq38_e1087_d_n8: f64 = ((eq38_e1085_d_n8 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][8]));
        let eq38_e1087_d_n9: f64 = ((eq38_e1085_d_n9 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][9]));
        let eq38_e1087_d_n10: f64 = ((eq38_e1085_d_n10 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][10]));
        let eq38_e1087_d_n11: f64 = ((eq38_e1085_d_n11 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][11]));
        let eq38_e1088: f64 = self.eval_ddt(1, eq38_e1087);
        let eq38_e1088_d_n0: f64 = self.ddt_jacobian(eq38_e1087_d_n0);
        let eq38_e1088_d_n1: f64 = self.ddt_jacobian(eq38_e1087_d_n1);
        let eq38_e1088_d_n2: f64 = self.ddt_jacobian(eq38_e1087_d_n2);
        let eq38_e1088_d_n3: f64 = self.ddt_jacobian(eq38_e1087_d_n3);
        let eq38_e1088_d_n4: f64 = self.ddt_jacobian(eq38_e1087_d_n4);
        let eq38_e1088_d_n5: f64 = self.ddt_jacobian(eq38_e1087_d_n5);
        let eq38_e1088_d_n6: f64 = self.ddt_jacobian(eq38_e1087_d_n6);
        let eq38_e1088_d_n7: f64 = self.ddt_jacobian(eq38_e1087_d_n7);
        let eq38_e1088_d_n8: f64 = self.ddt_jacobian(eq38_e1087_d_n8);
        let eq38_e1088_d_n9: f64 = self.ddt_jacobian(eq38_e1087_d_n9);
        let eq38_e1088_d_n10: f64 = self.ddt_jacobian(eq38_e1087_d_n10);
        let eq38_e1088_d_n11: f64 = self.ddt_jacobian(eq38_e1087_d_n11);
        let eq38_value: f64 = eq38_e1088;
        stamper.stamp_current(
            Some(self.nodes[8]),
            Some(self.nodes[6]),
            self.multiplicity * (eq38_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq38_e1088_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq38_e1088_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq38_e1088_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq38_e1088_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq38_e1088_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq38_e1088_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq38_e1088_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq38_e1088_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq38_e1088_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq38_e1088_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq38_e1088_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq38_e1088_d_n11),
            ],
        );
        let eq39_e1091: f64 = (scratch.values[0] * scratch.values[25]);
        let eq39_e1091_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq39_e1091_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq39_e1091_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq39_e1091_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq39_e1091_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq39_e1091_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq39_e1091_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq39_e1091_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq39_e1091_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq39_e1091_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq39_e1091_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq39_e1091_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq39_e1093: f64 = (eq39_e1091 * scratch.values[945]);
        let eq39_e1093_d_n0: f64 = ((eq39_e1091_d_n0 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][0]));
        let eq39_e1093_d_n1: f64 = ((eq39_e1091_d_n1 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][1]));
        let eq39_e1093_d_n2: f64 = ((eq39_e1091_d_n2 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][2]));
        let eq39_e1093_d_n3: f64 = ((eq39_e1091_d_n3 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][3]));
        let eq39_e1093_d_n4: f64 = ((eq39_e1091_d_n4 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][4]));
        let eq39_e1093_d_n5: f64 = ((eq39_e1091_d_n5 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][5]));
        let eq39_e1093_d_n6: f64 = ((eq39_e1091_d_n6 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][6]));
        let eq39_e1093_d_n7: f64 = ((eq39_e1091_d_n7 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][7]));
        let eq39_e1093_d_n8: f64 = ((eq39_e1091_d_n8 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][8]));
        let eq39_e1093_d_n9: f64 = ((eq39_e1091_d_n9 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][9]));
        let eq39_e1093_d_n10: f64 = ((eq39_e1091_d_n10 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][10]));
        let eq39_e1093_d_n11: f64 = ((eq39_e1091_d_n11 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][11]));
        let eq39_e1094: f64 = self.eval_ddt(2, eq39_e1093);
        let eq39_e1094_d_n0: f64 = self.ddt_jacobian(eq39_e1093_d_n0);
        let eq39_e1094_d_n1: f64 = self.ddt_jacobian(eq39_e1093_d_n1);
        let eq39_e1094_d_n2: f64 = self.ddt_jacobian(eq39_e1093_d_n2);
        let eq39_e1094_d_n3: f64 = self.ddt_jacobian(eq39_e1093_d_n3);
        let eq39_e1094_d_n4: f64 = self.ddt_jacobian(eq39_e1093_d_n4);
        let eq39_e1094_d_n5: f64 = self.ddt_jacobian(eq39_e1093_d_n5);
        let eq39_e1094_d_n6: f64 = self.ddt_jacobian(eq39_e1093_d_n6);
        let eq39_e1094_d_n7: f64 = self.ddt_jacobian(eq39_e1093_d_n7);
        let eq39_e1094_d_n8: f64 = self.ddt_jacobian(eq39_e1093_d_n8);
        let eq39_e1094_d_n9: f64 = self.ddt_jacobian(eq39_e1093_d_n9);
        let eq39_e1094_d_n10: f64 = self.ddt_jacobian(eq39_e1093_d_n10);
        let eq39_e1094_d_n11: f64 = self.ddt_jacobian(eq39_e1093_d_n11);
        let eq39_value: f64 = eq39_e1094;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq39_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq39_e1094_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq39_e1094_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq39_e1094_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq39_e1094_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq39_e1094_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq39_e1094_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq39_e1094_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq39_e1094_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq39_e1094_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq39_e1094_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq39_e1094_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq39_e1094_d_n11),
            ],
        );
        let eq40_e1097: f64 = (scratch.values[0] * scratch.values[25]);
        let eq40_e1097_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq40_e1097_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq40_e1097_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq40_e1097_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq40_e1097_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq40_e1097_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq40_e1097_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq40_e1097_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq40_e1097_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq40_e1097_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq40_e1097_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq40_e1097_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq40_e1099: f64 = (eq40_e1097 * scratch.values[951]);
        let eq40_e1099_d_n0: f64 = ((eq40_e1097_d_n0 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][0]));
        let eq40_e1099_d_n1: f64 = ((eq40_e1097_d_n1 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][1]));
        let eq40_e1099_d_n2: f64 = ((eq40_e1097_d_n2 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][2]));
        let eq40_e1099_d_n3: f64 = ((eq40_e1097_d_n3 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][3]));
        let eq40_e1099_d_n4: f64 = ((eq40_e1097_d_n4 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][4]));
        let eq40_e1099_d_n5: f64 = ((eq40_e1097_d_n5 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][5]));
        let eq40_e1099_d_n6: f64 = ((eq40_e1097_d_n6 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][6]));
        let eq40_e1099_d_n7: f64 = ((eq40_e1097_d_n7 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][7]));
        let eq40_e1099_d_n8: f64 = ((eq40_e1097_d_n8 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][8]));
        let eq40_e1099_d_n9: f64 = ((eq40_e1097_d_n9 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][9]));
        let eq40_e1099_d_n10: f64 = ((eq40_e1097_d_n10 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][10]));
        let eq40_e1099_d_n11: f64 = ((eq40_e1097_d_n11 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][11]));
        let eq40_e1100: f64 = self.eval_ddt(3, eq40_e1099);
        let eq40_e1100_d_n0: f64 = self.ddt_jacobian(eq40_e1099_d_n0);
        let eq40_e1100_d_n1: f64 = self.ddt_jacobian(eq40_e1099_d_n1);
        let eq40_e1100_d_n2: f64 = self.ddt_jacobian(eq40_e1099_d_n2);
        let eq40_e1100_d_n3: f64 = self.ddt_jacobian(eq40_e1099_d_n3);
        let eq40_e1100_d_n4: f64 = self.ddt_jacobian(eq40_e1099_d_n4);
        let eq40_e1100_d_n5: f64 = self.ddt_jacobian(eq40_e1099_d_n5);
        let eq40_e1100_d_n6: f64 = self.ddt_jacobian(eq40_e1099_d_n6);
        let eq40_e1100_d_n7: f64 = self.ddt_jacobian(eq40_e1099_d_n7);
        let eq40_e1100_d_n8: f64 = self.ddt_jacobian(eq40_e1099_d_n8);
        let eq40_e1100_d_n9: f64 = self.ddt_jacobian(eq40_e1099_d_n9);
        let eq40_e1100_d_n10: f64 = self.ddt_jacobian(eq40_e1099_d_n10);
        let eq40_e1100_d_n11: f64 = self.ddt_jacobian(eq40_e1099_d_n11);
        let eq40_value: f64 = eq40_e1100;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq40_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq40_e1100_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq40_e1100_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq40_e1100_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq40_e1100_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq40_e1100_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq40_e1100_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq40_e1100_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq40_e1100_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq40_e1100_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq40_e1100_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq40_e1100_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq40_e1100_d_n11),
            ],
        );
        let eq41_e1103: f64 = (scratch.values[0] * scratch.values[25]);
        let eq41_e1103_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq41_e1103_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq41_e1103_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq41_e1103_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq41_e1103_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq41_e1103_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq41_e1103_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq41_e1103_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq41_e1103_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq41_e1103_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq41_e1103_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq41_e1103_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq41_e1105: f64 = (eq41_e1103 * scratch.values[952]);
        let eq41_e1105_d_n0: f64 = ((eq41_e1103_d_n0 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][0]));
        let eq41_e1105_d_n1: f64 = ((eq41_e1103_d_n1 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][1]));
        let eq41_e1105_d_n2: f64 = ((eq41_e1103_d_n2 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][2]));
        let eq41_e1105_d_n3: f64 = ((eq41_e1103_d_n3 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][3]));
        let eq41_e1105_d_n4: f64 = ((eq41_e1103_d_n4 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][4]));
        let eq41_e1105_d_n5: f64 = ((eq41_e1103_d_n5 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][5]));
        let eq41_e1105_d_n6: f64 = ((eq41_e1103_d_n6 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][6]));
        let eq41_e1105_d_n7: f64 = ((eq41_e1103_d_n7 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][7]));
        let eq41_e1105_d_n8: f64 = ((eq41_e1103_d_n8 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][8]));
        let eq41_e1105_d_n9: f64 = ((eq41_e1103_d_n9 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][9]));
        let eq41_e1105_d_n10: f64 = ((eq41_e1103_d_n10 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][10]));
        let eq41_e1105_d_n11: f64 = ((eq41_e1103_d_n11 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][11]));
        let eq41_e1106: f64 = self.eval_ddt(4, eq41_e1105);
        let eq41_e1106_d_n0: f64 = self.ddt_jacobian(eq41_e1105_d_n0);
        let eq41_e1106_d_n1: f64 = self.ddt_jacobian(eq41_e1105_d_n1);
        let eq41_e1106_d_n2: f64 = self.ddt_jacobian(eq41_e1105_d_n2);
        let eq41_e1106_d_n3: f64 = self.ddt_jacobian(eq41_e1105_d_n3);
        let eq41_e1106_d_n4: f64 = self.ddt_jacobian(eq41_e1105_d_n4);
        let eq41_e1106_d_n5: f64 = self.ddt_jacobian(eq41_e1105_d_n5);
        let eq41_e1106_d_n6: f64 = self.ddt_jacobian(eq41_e1105_d_n6);
        let eq41_e1106_d_n7: f64 = self.ddt_jacobian(eq41_e1105_d_n7);
        let eq41_e1106_d_n8: f64 = self.ddt_jacobian(eq41_e1105_d_n8);
        let eq41_e1106_d_n9: f64 = self.ddt_jacobian(eq41_e1105_d_n9);
        let eq41_e1106_d_n10: f64 = self.ddt_jacobian(eq41_e1105_d_n10);
        let eq41_e1106_d_n11: f64 = self.ddt_jacobian(eq41_e1105_d_n11);
        let eq41_value: f64 = eq41_e1106;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq41_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq41_e1106_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq41_e1106_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq41_e1106_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq41_e1106_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq41_e1106_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq41_e1106_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq41_e1106_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq41_e1106_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq41_e1106_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq41_e1106_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq41_e1106_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq41_e1106_d_n11),
            ],
        );
        let eq42_e1109: f64 = (scratch.values[0] * scratch.values[25]);
        let eq42_e1109_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq42_e1109_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq42_e1109_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq42_e1109_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq42_e1109_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq42_e1109_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq42_e1109_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq42_e1109_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq42_e1109_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq42_e1109_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq42_e1109_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq42_e1109_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq42_e1111: f64 = (eq42_e1109 * scratch.values[950]);
        let eq42_e1111_d_n0: f64 = ((eq42_e1109_d_n0 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][0]));
        let eq42_e1111_d_n1: f64 = ((eq42_e1109_d_n1 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][1]));
        let eq42_e1111_d_n2: f64 = ((eq42_e1109_d_n2 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][2]));
        let eq42_e1111_d_n3: f64 = ((eq42_e1109_d_n3 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][3]));
        let eq42_e1111_d_n4: f64 = ((eq42_e1109_d_n4 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][4]));
        let eq42_e1111_d_n5: f64 = ((eq42_e1109_d_n5 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][5]));
        let eq42_e1111_d_n6: f64 = ((eq42_e1109_d_n6 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][6]));
        let eq42_e1111_d_n7: f64 = ((eq42_e1109_d_n7 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][7]));
        let eq42_e1111_d_n8: f64 = ((eq42_e1109_d_n8 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][8]));
        let eq42_e1111_d_n9: f64 = ((eq42_e1109_d_n9 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][9]));
        let eq42_e1111_d_n10: f64 = ((eq42_e1109_d_n10 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][10]));
        let eq42_e1111_d_n11: f64 = ((eq42_e1109_d_n11 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][11]));
        let eq42_e1112: f64 = self.eval_ddt(5, eq42_e1111);
        let eq42_e1112_d_n0: f64 = self.ddt_jacobian(eq42_e1111_d_n0);
        let eq42_e1112_d_n1: f64 = self.ddt_jacobian(eq42_e1111_d_n1);
        let eq42_e1112_d_n2: f64 = self.ddt_jacobian(eq42_e1111_d_n2);
        let eq42_e1112_d_n3: f64 = self.ddt_jacobian(eq42_e1111_d_n3);
        let eq42_e1112_d_n4: f64 = self.ddt_jacobian(eq42_e1111_d_n4);
        let eq42_e1112_d_n5: f64 = self.ddt_jacobian(eq42_e1111_d_n5);
        let eq42_e1112_d_n6: f64 = self.ddt_jacobian(eq42_e1111_d_n6);
        let eq42_e1112_d_n7: f64 = self.ddt_jacobian(eq42_e1111_d_n7);
        let eq42_e1112_d_n8: f64 = self.ddt_jacobian(eq42_e1111_d_n8);
        let eq42_e1112_d_n9: f64 = self.ddt_jacobian(eq42_e1111_d_n9);
        let eq42_e1112_d_n10: f64 = self.ddt_jacobian(eq42_e1111_d_n10);
        let eq42_e1112_d_n11: f64 = self.ddt_jacobian(eq42_e1111_d_n11);
        let eq42_value: f64 = eq42_e1112;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[8]),
            self.multiplicity * (eq42_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq42_e1112_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq42_e1112_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq42_e1112_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq42_e1112_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq42_e1112_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq42_e1112_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq42_e1112_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq42_e1112_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq42_e1112_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq42_e1112_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq42_e1112_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq42_e1112_d_n11),
            ],
        );
        let eq43_e1115: f64 = (scratch.values[0] * scratch.values[25]);
        let eq43_e1115_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq43_e1115_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq43_e1115_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq43_e1115_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq43_e1115_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq43_e1115_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq43_e1115_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq43_e1115_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq43_e1115_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq43_e1115_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq43_e1115_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq43_e1115_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq43_e1117: f64 = (eq43_e1115 * scratch.values[2041]);
        let eq43_e1117_d_n0: f64 = ((eq43_e1115_d_n0 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][0]));
        let eq43_e1117_d_n1: f64 = ((eq43_e1115_d_n1 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][1]));
        let eq43_e1117_d_n2: f64 = ((eq43_e1115_d_n2 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][2]));
        let eq43_e1117_d_n3: f64 = ((eq43_e1115_d_n3 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][3]));
        let eq43_e1117_d_n4: f64 = ((eq43_e1115_d_n4 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][4]));
        let eq43_e1117_d_n5: f64 = ((eq43_e1115_d_n5 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][5]));
        let eq43_e1117_d_n6: f64 = ((eq43_e1115_d_n6 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][6]));
        let eq43_e1117_d_n7: f64 = ((eq43_e1115_d_n7 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][7]));
        let eq43_e1117_d_n8: f64 = ((eq43_e1115_d_n8 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][8]));
        let eq43_e1117_d_n9: f64 = ((eq43_e1115_d_n9 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][9]));
        let eq43_e1117_d_n10: f64 = ((eq43_e1115_d_n10 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][10]));
        let eq43_e1117_d_n11: f64 = ((eq43_e1115_d_n11 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][11]));
        let eq43_e1118: f64 = self.eval_ddt(6, eq43_e1117);
        let eq43_e1118_d_n0: f64 = self.ddt_jacobian(eq43_e1117_d_n0);
        let eq43_e1118_d_n1: f64 = self.ddt_jacobian(eq43_e1117_d_n1);
        let eq43_e1118_d_n2: f64 = self.ddt_jacobian(eq43_e1117_d_n2);
        let eq43_e1118_d_n3: f64 = self.ddt_jacobian(eq43_e1117_d_n3);
        let eq43_e1118_d_n4: f64 = self.ddt_jacobian(eq43_e1117_d_n4);
        let eq43_e1118_d_n5: f64 = self.ddt_jacobian(eq43_e1117_d_n5);
        let eq43_e1118_d_n6: f64 = self.ddt_jacobian(eq43_e1117_d_n6);
        let eq43_e1118_d_n7: f64 = self.ddt_jacobian(eq43_e1117_d_n7);
        let eq43_e1118_d_n8: f64 = self.ddt_jacobian(eq43_e1117_d_n8);
        let eq43_e1118_d_n9: f64 = self.ddt_jacobian(eq43_e1117_d_n9);
        let eq43_e1118_d_n10: f64 = self.ddt_jacobian(eq43_e1117_d_n10);
        let eq43_e1118_d_n11: f64 = self.ddt_jacobian(eq43_e1117_d_n11);
        let eq43_value: f64 = eq43_e1118;
        stamper.stamp_current(
            Some(self.nodes[10]),
            Some(self.nodes[6]),
            self.multiplicity * (eq43_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq43_e1118_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq43_e1118_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq43_e1118_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq43_e1118_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq43_e1118_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq43_e1118_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq43_e1118_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq43_e1118_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq43_e1118_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq43_e1118_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq43_e1118_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq43_e1118_d_n11),
            ],
        );
        let eq44_e1121: f64 = (scratch.values[0] * scratch.values[25]);
        let eq44_e1121_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq44_e1121_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq44_e1121_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq44_e1121_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq44_e1121_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq44_e1121_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq44_e1121_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq44_e1121_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq44_e1121_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq44_e1121_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq44_e1121_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq44_e1121_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq44_e1123: f64 = (eq44_e1121 * scratch.values[2045]);
        let eq44_e1123_d_n0: f64 = ((eq44_e1121_d_n0 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][0]));
        let eq44_e1123_d_n1: f64 = ((eq44_e1121_d_n1 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][1]));
        let eq44_e1123_d_n2: f64 = ((eq44_e1121_d_n2 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][2]));
        let eq44_e1123_d_n3: f64 = ((eq44_e1121_d_n3 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][3]));
        let eq44_e1123_d_n4: f64 = ((eq44_e1121_d_n4 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][4]));
        let eq44_e1123_d_n5: f64 = ((eq44_e1121_d_n5 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][5]));
        let eq44_e1123_d_n6: f64 = ((eq44_e1121_d_n6 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][6]));
        let eq44_e1123_d_n7: f64 = ((eq44_e1121_d_n7 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][7]));
        let eq44_e1123_d_n8: f64 = ((eq44_e1121_d_n8 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][8]));
        let eq44_e1123_d_n9: f64 = ((eq44_e1121_d_n9 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][9]));
        let eq44_e1123_d_n10: f64 = ((eq44_e1121_d_n10 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][10]));
        let eq44_e1123_d_n11: f64 = ((eq44_e1121_d_n11 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][11]));
        let eq44_e1124: f64 = self.eval_ddt(7, eq44_e1123);
        let eq44_e1124_d_n0: f64 = self.ddt_jacobian(eq44_e1123_d_n0);
        let eq44_e1124_d_n1: f64 = self.ddt_jacobian(eq44_e1123_d_n1);
        let eq44_e1124_d_n2: f64 = self.ddt_jacobian(eq44_e1123_d_n2);
        let eq44_e1124_d_n3: f64 = self.ddt_jacobian(eq44_e1123_d_n3);
        let eq44_e1124_d_n4: f64 = self.ddt_jacobian(eq44_e1123_d_n4);
        let eq44_e1124_d_n5: f64 = self.ddt_jacobian(eq44_e1123_d_n5);
        let eq44_e1124_d_n6: f64 = self.ddt_jacobian(eq44_e1123_d_n6);
        let eq44_e1124_d_n7: f64 = self.ddt_jacobian(eq44_e1123_d_n7);
        let eq44_e1124_d_n8: f64 = self.ddt_jacobian(eq44_e1123_d_n8);
        let eq44_e1124_d_n9: f64 = self.ddt_jacobian(eq44_e1123_d_n9);
        let eq44_e1124_d_n10: f64 = self.ddt_jacobian(eq44_e1123_d_n10);
        let eq44_e1124_d_n11: f64 = self.ddt_jacobian(eq44_e1123_d_n11);
        let eq44_value: f64 = eq44_e1124;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq44_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq44_e1124_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq44_e1124_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq44_e1124_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq44_e1124_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq44_e1124_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq44_e1124_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq44_e1124_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq44_e1124_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq44_e1124_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq44_e1124_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq44_e1124_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq44_e1124_d_n11),
            ],
        );
        let eq45_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq45_value),
            &[
            ],
        );
        let eq46_e1132: f64 = ((ctx.node_voltage(self.nodes[4]) - 0.0) / scratch.values[1003]);
        let eq46_e1132_d_n0: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][0]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n1: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][1]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n2: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][2]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n3: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][3]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n4: f64 = ((scratch.values[1003] - ((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][4])) / (scratch.values[1003] * scratch.values[1003]));
        let eq46_e1132_d_n5: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][5]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n6: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][6]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n7: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][7]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n8: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][8]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n9: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][9]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n10: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][10]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_e1132_d_n11: f64 = (-(((ctx.node_voltage(self.nodes[4]) - 0.0) * scratch.node_derivatives[1003][11]) / (scratch.values[1003] * scratch.values[1003])));
        let eq46_value: f64 = eq46_e1132;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq46_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq46_e1132_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq46_e1132_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq46_e1132_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq46_e1132_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq46_e1132_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq46_e1132_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq46_e1132_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq46_e1132_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq46_e1132_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq46_e1132_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq46_e1132_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq46_e1132_d_n11),
            ],
        );
        let eq47_e1135: f64 = (scratch.values[1006] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n0: f64 = (scratch.node_derivatives[1006][0] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n1: f64 = (scratch.node_derivatives[1006][1] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n2: f64 = (scratch.node_derivatives[1006][2] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n3: f64 = (scratch.node_derivatives[1006][3] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n4: f64 = ((scratch.node_derivatives[1006][4] * (ctx.node_voltage(self.nodes[4]) - 0.0)) + scratch.values[1006]);
        let eq47_e1135_d_n5: f64 = (scratch.node_derivatives[1006][5] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n6: f64 = (scratch.node_derivatives[1006][6] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n7: f64 = (scratch.node_derivatives[1006][7] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n8: f64 = (scratch.node_derivatives[1006][8] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n9: f64 = (scratch.node_derivatives[1006][9] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n10: f64 = (scratch.node_derivatives[1006][10] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n11: f64 = (scratch.node_derivatives[1006][11] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1136: f64 = self.eval_ddt(8, eq47_e1135);
        let eq47_e1136_d_n0: f64 = self.ddt_jacobian(eq47_e1135_d_n0);
        let eq47_e1136_d_n1: f64 = self.ddt_jacobian(eq47_e1135_d_n1);
        let eq47_e1136_d_n2: f64 = self.ddt_jacobian(eq47_e1135_d_n2);
        let eq47_e1136_d_n3: f64 = self.ddt_jacobian(eq47_e1135_d_n3);
        let eq47_e1136_d_n4: f64 = self.ddt_jacobian(eq47_e1135_d_n4);
        let eq47_e1136_d_n5: f64 = self.ddt_jacobian(eq47_e1135_d_n5);
        let eq47_e1136_d_n6: f64 = self.ddt_jacobian(eq47_e1135_d_n6);
        let eq47_e1136_d_n7: f64 = self.ddt_jacobian(eq47_e1135_d_n7);
        let eq47_e1136_d_n8: f64 = self.ddt_jacobian(eq47_e1135_d_n8);
        let eq47_e1136_d_n9: f64 = self.ddt_jacobian(eq47_e1135_d_n9);
        let eq47_e1136_d_n10: f64 = self.ddt_jacobian(eq47_e1135_d_n10);
        let eq47_e1136_d_n11: f64 = self.ddt_jacobian(eq47_e1135_d_n11);
        let eq47_value: f64 = eq47_e1136;
        stamper.stamp_current(
            Some(self.nodes[4]),
            None,
            self.multiplicity * (eq47_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq47_e1136_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq47_e1136_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq47_e1136_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq47_e1136_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq47_e1136_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq47_e1136_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq47_e1136_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq47_e1136_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq47_e1136_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq47_e1136_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq47_e1136_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq47_e1136_d_n11),
            ],
        );
        let eq48_e1138: f64 = (scratch.values[25]).sqrt();
        let eq48_e1138_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq48_e1138));
        let eq48_e1140: f64 = (eq48_e1138 * 0.5);
        let eq48_e1140_d_n0: f64 = (eq48_e1138_d_n0 * 0.5);
        let eq48_e1140_d_n1: f64 = (eq48_e1138_d_n1 * 0.5);
        let eq48_e1140_d_n2: f64 = (eq48_e1138_d_n2 * 0.5);
        let eq48_e1140_d_n3: f64 = (eq48_e1138_d_n3 * 0.5);
        let eq48_e1140_d_n4: f64 = (eq48_e1138_d_n4 * 0.5);
        let eq48_e1140_d_n5: f64 = (eq48_e1138_d_n5 * 0.5);
        let eq48_e1140_d_n6: f64 = (eq48_e1138_d_n6 * 0.5);
        let eq48_e1140_d_n7: f64 = (eq48_e1138_d_n7 * 0.5);
        let eq48_e1140_d_n8: f64 = (eq48_e1138_d_n8 * 0.5);
        let eq48_e1140_d_n9: f64 = (eq48_e1138_d_n9 * 0.5);
        let eq48_e1140_d_n10: f64 = (eq48_e1138_d_n10 * 0.5);
        let eq48_e1140_d_n11: f64 = (eq48_e1138_d_n11 * 0.5);
        let eq48_e1142: f64 = (eq48_e1140 * scratch.values[1006]);
        let eq48_e1142_d_n0: f64 = ((eq48_e1140_d_n0 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][0]));
        let eq48_e1142_d_n1: f64 = ((eq48_e1140_d_n1 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][1]));
        let eq48_e1142_d_n2: f64 = ((eq48_e1140_d_n2 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][2]));
        let eq48_e1142_d_n3: f64 = ((eq48_e1140_d_n3 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][3]));
        let eq48_e1142_d_n4: f64 = ((eq48_e1140_d_n4 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][4]));
        let eq48_e1142_d_n5: f64 = ((eq48_e1140_d_n5 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][5]));
        let eq48_e1142_d_n6: f64 = ((eq48_e1140_d_n6 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][6]));
        let eq48_e1142_d_n7: f64 = ((eq48_e1140_d_n7 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][7]));
        let eq48_e1142_d_n8: f64 = ((eq48_e1140_d_n8 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][8]));
        let eq48_e1142_d_n9: f64 = ((eq48_e1140_d_n9 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][9]));
        let eq48_e1142_d_n10: f64 = ((eq48_e1140_d_n10 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][10]));
        let eq48_e1142_d_n11: f64 = ((eq48_e1140_d_n11 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][11]));
        let eq48_e1144: f64 = (eq48_e1142 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n0: f64 = (eq48_e1142_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n1: f64 = (eq48_e1142_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n2: f64 = (eq48_e1142_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n3: f64 = (eq48_e1142_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n4: f64 = ((eq48_e1142_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq48_e1142);
        let eq48_e1144_d_n5: f64 = (eq48_e1142_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n6: f64 = (eq48_e1142_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n7: f64 = (eq48_e1142_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n8: f64 = (eq48_e1142_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n9: f64 = (eq48_e1142_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n10: f64 = (eq48_e1142_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n11: f64 = (eq48_e1142_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1145: f64 = self.eval_ddt(9, eq48_e1144);
        let eq48_e1145_d_n0: f64 = self.ddt_jacobian(eq48_e1144_d_n0);
        let eq48_e1145_d_n1: f64 = self.ddt_jacobian(eq48_e1144_d_n1);
        let eq48_e1145_d_n2: f64 = self.ddt_jacobian(eq48_e1144_d_n2);
        let eq48_e1145_d_n3: f64 = self.ddt_jacobian(eq48_e1144_d_n3);
        let eq48_e1145_d_n4: f64 = self.ddt_jacobian(eq48_e1144_d_n4);
        let eq48_e1145_d_n5: f64 = self.ddt_jacobian(eq48_e1144_d_n5);
        let eq48_e1145_d_n6: f64 = self.ddt_jacobian(eq48_e1144_d_n6);
        let eq48_e1145_d_n7: f64 = self.ddt_jacobian(eq48_e1144_d_n7);
        let eq48_e1145_d_n8: f64 = self.ddt_jacobian(eq48_e1144_d_n8);
        let eq48_e1145_d_n9: f64 = self.ddt_jacobian(eq48_e1144_d_n9);
        let eq48_e1145_d_n10: f64 = self.ddt_jacobian(eq48_e1144_d_n10);
        let eq48_e1145_d_n11: f64 = self.ddt_jacobian(eq48_e1144_d_n11);
        let eq48_e1146: f64 = (-eq48_e1145);
        let eq48_e1146_d_n0: f64 = (-eq48_e1145_d_n0);
        let eq48_e1146_d_n1: f64 = (-eq48_e1145_d_n1);
        let eq48_e1146_d_n2: f64 = (-eq48_e1145_d_n2);
        let eq48_e1146_d_n3: f64 = (-eq48_e1145_d_n3);
        let eq48_e1146_d_n4: f64 = (-eq48_e1145_d_n4);
        let eq48_e1146_d_n5: f64 = (-eq48_e1145_d_n5);
        let eq48_e1146_d_n6: f64 = (-eq48_e1145_d_n6);
        let eq48_e1146_d_n7: f64 = (-eq48_e1145_d_n7);
        let eq48_e1146_d_n8: f64 = (-eq48_e1145_d_n8);
        let eq48_e1146_d_n9: f64 = (-eq48_e1145_d_n9);
        let eq48_e1146_d_n10: f64 = (-eq48_e1145_d_n10);
        let eq48_e1146_d_n11: f64 = (-eq48_e1145_d_n11);
        let eq48_value: f64 = eq48_e1146;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq48_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq48_e1146_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq48_e1146_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq48_e1146_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq48_e1146_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq48_e1146_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq48_e1146_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq48_e1146_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq48_e1146_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq48_e1146_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq48_e1146_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq48_e1146_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq48_e1146_d_n11),
            ],
        );
        let eq49_e1148: f64 = (scratch.values[25]).sqrt();
        let eq49_e1148_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq49_e1148));
        let eq49_e1150: f64 = (eq49_e1148 * 0.5);
        let eq49_e1150_d_n0: f64 = (eq49_e1148_d_n0 * 0.5);
        let eq49_e1150_d_n1: f64 = (eq49_e1148_d_n1 * 0.5);
        let eq49_e1150_d_n2: f64 = (eq49_e1148_d_n2 * 0.5);
        let eq49_e1150_d_n3: f64 = (eq49_e1148_d_n3 * 0.5);
        let eq49_e1150_d_n4: f64 = (eq49_e1148_d_n4 * 0.5);
        let eq49_e1150_d_n5: f64 = (eq49_e1148_d_n5 * 0.5);
        let eq49_e1150_d_n6: f64 = (eq49_e1148_d_n6 * 0.5);
        let eq49_e1150_d_n7: f64 = (eq49_e1148_d_n7 * 0.5);
        let eq49_e1150_d_n8: f64 = (eq49_e1148_d_n8 * 0.5);
        let eq49_e1150_d_n9: f64 = (eq49_e1148_d_n9 * 0.5);
        let eq49_e1150_d_n10: f64 = (eq49_e1148_d_n10 * 0.5);
        let eq49_e1150_d_n11: f64 = (eq49_e1148_d_n11 * 0.5);
        let eq49_e1152: f64 = (eq49_e1150 * scratch.values[1006]);
        let eq49_e1152_d_n0: f64 = ((eq49_e1150_d_n0 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][0]));
        let eq49_e1152_d_n1: f64 = ((eq49_e1150_d_n1 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][1]));
        let eq49_e1152_d_n2: f64 = ((eq49_e1150_d_n2 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][2]));
        let eq49_e1152_d_n3: f64 = ((eq49_e1150_d_n3 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][3]));
        let eq49_e1152_d_n4: f64 = ((eq49_e1150_d_n4 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][4]));
        let eq49_e1152_d_n5: f64 = ((eq49_e1150_d_n5 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][5]));
        let eq49_e1152_d_n6: f64 = ((eq49_e1150_d_n6 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][6]));
        let eq49_e1152_d_n7: f64 = ((eq49_e1150_d_n7 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][7]));
        let eq49_e1152_d_n8: f64 = ((eq49_e1150_d_n8 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][8]));
        let eq49_e1152_d_n9: f64 = ((eq49_e1150_d_n9 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][9]));
        let eq49_e1152_d_n10: f64 = ((eq49_e1150_d_n10 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][10]));
        let eq49_e1152_d_n11: f64 = ((eq49_e1150_d_n11 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][11]));
        let eq49_e1154: f64 = (eq49_e1152 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n0: f64 = (eq49_e1152_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n1: f64 = (eq49_e1152_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n2: f64 = (eq49_e1152_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n3: f64 = (eq49_e1152_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n4: f64 = ((eq49_e1152_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq49_e1152);
        let eq49_e1154_d_n5: f64 = (eq49_e1152_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n6: f64 = (eq49_e1152_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n7: f64 = (eq49_e1152_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n8: f64 = (eq49_e1152_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n9: f64 = (eq49_e1152_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n10: f64 = (eq49_e1152_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n11: f64 = (eq49_e1152_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1155: f64 = self.eval_ddt(10, eq49_e1154);
        let eq49_e1155_d_n0: f64 = self.ddt_jacobian(eq49_e1154_d_n0);
        let eq49_e1155_d_n1: f64 = self.ddt_jacobian(eq49_e1154_d_n1);
        let eq49_e1155_d_n2: f64 = self.ddt_jacobian(eq49_e1154_d_n2);
        let eq49_e1155_d_n3: f64 = self.ddt_jacobian(eq49_e1154_d_n3);
        let eq49_e1155_d_n4: f64 = self.ddt_jacobian(eq49_e1154_d_n4);
        let eq49_e1155_d_n5: f64 = self.ddt_jacobian(eq49_e1154_d_n5);
        let eq49_e1155_d_n6: f64 = self.ddt_jacobian(eq49_e1154_d_n6);
        let eq49_e1155_d_n7: f64 = self.ddt_jacobian(eq49_e1154_d_n7);
        let eq49_e1155_d_n8: f64 = self.ddt_jacobian(eq49_e1154_d_n8);
        let eq49_e1155_d_n9: f64 = self.ddt_jacobian(eq49_e1154_d_n9);
        let eq49_e1155_d_n10: f64 = self.ddt_jacobian(eq49_e1154_d_n10);
        let eq49_e1155_d_n11: f64 = self.ddt_jacobian(eq49_e1154_d_n11);
        let eq49_e1156: f64 = (-eq49_e1155);
        let eq49_e1156_d_n0: f64 = (-eq49_e1155_d_n0);
        let eq49_e1156_d_n1: f64 = (-eq49_e1155_d_n1);
        let eq49_e1156_d_n2: f64 = (-eq49_e1155_d_n2);
        let eq49_e1156_d_n3: f64 = (-eq49_e1155_d_n3);
        let eq49_e1156_d_n4: f64 = (-eq49_e1155_d_n4);
        let eq49_e1156_d_n5: f64 = (-eq49_e1155_d_n5);
        let eq49_e1156_d_n6: f64 = (-eq49_e1155_d_n6);
        let eq49_e1156_d_n7: f64 = (-eq49_e1155_d_n7);
        let eq49_e1156_d_n8: f64 = (-eq49_e1155_d_n8);
        let eq49_e1156_d_n9: f64 = (-eq49_e1155_d_n9);
        let eq49_e1156_d_n10: f64 = (-eq49_e1155_d_n10);
        let eq49_e1156_d_n11: f64 = (-eq49_e1155_d_n11);
        let eq49_value: f64 = eq49_e1156;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq49_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq49_e1156_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq49_e1156_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq49_e1156_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq49_e1156_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq49_e1156_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq49_e1156_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq49_e1156_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq49_e1156_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq49_e1156_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq49_e1156_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq49_e1156_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq49_e1156_d_n11),
            ],
        );
        let eq50_e1159: f64 = (scratch.values[25]).sqrt();
        let eq50_e1159_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq50_e1159));
        let eq50_e1159_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq50_e1159));
        let eq50_e1160: f64 = (scratch.values[2002] * eq50_e1159);
        let eq50_e1160_d_n0: f64 = ((scratch.node_derivatives[2002][0] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n0));
        let eq50_e1160_d_n1: f64 = ((scratch.node_derivatives[2002][1] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n1));
        let eq50_e1160_d_n2: f64 = ((scratch.node_derivatives[2002][2] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n2));
        let eq50_e1160_d_n3: f64 = ((scratch.node_derivatives[2002][3] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n3));
        let eq50_e1160_d_n4: f64 = ((scratch.node_derivatives[2002][4] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n4));
        let eq50_e1160_d_n5: f64 = ((scratch.node_derivatives[2002][5] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n5));
        let eq50_e1160_d_n6: f64 = ((scratch.node_derivatives[2002][6] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n6));
        let eq50_e1160_d_n7: f64 = ((scratch.node_derivatives[2002][7] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n7));
        let eq50_e1160_d_n8: f64 = ((scratch.node_derivatives[2002][8] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n8));
        let eq50_e1160_d_n9: f64 = ((scratch.node_derivatives[2002][9] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n9));
        let eq50_e1160_d_n10: f64 = ((scratch.node_derivatives[2002][10] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n10));
        let eq50_e1160_d_n11: f64 = ((scratch.node_derivatives[2002][11] * eq50_e1159) + (scratch.values[2002] * eq50_e1159_d_n11));
        let eq50_e1162: f64 = (eq50_e1160 * scratch.values[1005]);
        let eq50_e1162_d_n0: f64 = ((eq50_e1160_d_n0 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][0]));
        let eq50_e1162_d_n1: f64 = ((eq50_e1160_d_n1 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][1]));
        let eq50_e1162_d_n2: f64 = ((eq50_e1160_d_n2 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][2]));
        let eq50_e1162_d_n3: f64 = ((eq50_e1160_d_n3 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][3]));
        let eq50_e1162_d_n4: f64 = ((eq50_e1160_d_n4 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][4]));
        let eq50_e1162_d_n5: f64 = ((eq50_e1160_d_n5 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][5]));
        let eq50_e1162_d_n6: f64 = ((eq50_e1160_d_n6 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][6]));
        let eq50_e1162_d_n7: f64 = ((eq50_e1160_d_n7 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][7]));
        let eq50_e1162_d_n8: f64 = ((eq50_e1160_d_n8 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][8]));
        let eq50_e1162_d_n9: f64 = ((eq50_e1160_d_n9 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][9]));
        let eq50_e1162_d_n10: f64 = ((eq50_e1160_d_n10 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][10]));
        let eq50_e1162_d_n11: f64 = ((eq50_e1160_d_n11 * scratch.values[1005]) + (eq50_e1160 * scratch.node_derivatives[1005][11]));
        let eq50_e1164: f64 = (eq50_e1162 * eq45_value);
        let eq50_e1164_d_n0: f64 = (eq50_e1162_d_n0 * eq45_value);
        let eq50_e1164_d_n1: f64 = (eq50_e1162_d_n1 * eq45_value);
        let eq50_e1164_d_n2: f64 = (eq50_e1162_d_n2 * eq45_value);
        let eq50_e1164_d_n3: f64 = (eq50_e1162_d_n3 * eq45_value);
        let eq50_e1164_d_n4: f64 = (eq50_e1162_d_n4 * eq45_value);
        let eq50_e1164_d_n5: f64 = (eq50_e1162_d_n5 * eq45_value);
        let eq50_e1164_d_n6: f64 = (eq50_e1162_d_n6 * eq45_value);
        let eq50_e1164_d_n7: f64 = (eq50_e1162_d_n7 * eq45_value);
        let eq50_e1164_d_n8: f64 = (eq50_e1162_d_n8 * eq45_value);
        let eq50_e1164_d_n9: f64 = (eq50_e1162_d_n9 * eq45_value);
        let eq50_e1164_d_n10: f64 = (eq50_e1162_d_n10 * eq45_value);
        let eq50_e1164_d_n11: f64 = (eq50_e1162_d_n11 * eq45_value);
        let eq50_value: f64 = eq50_e1164;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq50_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq50_e1164_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq50_e1164_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq50_e1164_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq50_e1164_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq50_e1164_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq50_e1164_d_n5),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * eq50_e1164_d_n6),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * eq50_e1164_d_n7),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * eq50_e1164_d_n8),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * eq50_e1164_d_n9),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * eq50_e1164_d_n10),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * eq50_e1164_d_n11),
            ],
        );
        let eq51_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq51_value),
            &[
            ],
        );
        let eq52_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq52_value),
            &[
            ],
        );
        let eq53_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            self.multiplicity * (eq53_value),
            &[
            ],
        );
        let eq54_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            self.multiplicity * (eq54_value),
            &[
            ],
        );
        let eq55_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[10]),
            Some(self.nodes[6]),
            self.multiplicity * (eq55_value),
            &[
            ],
        );
        let eq56_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            self.multiplicity * (eq56_value),
            &[
            ],
        );
        let eq57_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq57_value),
            &[
            ],
        );
        let eq58_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            self.multiplicity * (eq58_value),
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

        let eq37_e1079: f64 = (scratch.values[0] * scratch.values[25]);
        let eq37_e1079_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq37_e1079_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq37_e1079_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq37_e1079_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq37_e1079_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq37_e1079_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq37_e1079_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq37_e1079_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq37_e1079_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq37_e1079_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq37_e1079_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq37_e1079_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq37_e1081: f64 = (eq37_e1079 * scratch.values[944]);
        let eq37_e1081_d_n0: f64 = ((eq37_e1079_d_n0 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][0]));
        let eq37_e1081_d_n1: f64 = ((eq37_e1079_d_n1 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][1]));
        let eq37_e1081_d_n2: f64 = ((eq37_e1079_d_n2 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][2]));
        let eq37_e1081_d_n3: f64 = ((eq37_e1079_d_n3 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][3]));
        let eq37_e1081_d_n4: f64 = ((eq37_e1079_d_n4 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][4]));
        let eq37_e1081_d_n5: f64 = ((eq37_e1079_d_n5 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][5]));
        let eq37_e1081_d_n6: f64 = ((eq37_e1079_d_n6 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][6]));
        let eq37_e1081_d_n7: f64 = ((eq37_e1079_d_n7 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][7]));
        let eq37_e1081_d_n8: f64 = ((eq37_e1079_d_n8 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][8]));
        let eq37_e1081_d_n9: f64 = ((eq37_e1079_d_n9 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][9]));
        let eq37_e1081_d_n10: f64 = ((eq37_e1079_d_n10 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][10]));
        let eq37_e1081_d_n11: f64 = ((eq37_e1079_d_n11 * scratch.values[944]) + (eq37_e1079 * scratch.node_derivatives[944][11]));
        let eq37_e1082_q: f64 = eq37_e1081;
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq37_e1081_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq37_e1081_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq37_e1081_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq37_e1081_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq37_e1081_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq37_e1081_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq37_e1081_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq37_e1081_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq37_e1081_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq37_e1081_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq37_e1081_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq37_e1081_d_n11)),
            ],
        );
        let eq38_e1085: f64 = (scratch.values[0] * scratch.values[25]);
        let eq38_e1085_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq38_e1085_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq38_e1085_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq38_e1085_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq38_e1085_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq38_e1085_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq38_e1085_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq38_e1085_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq38_e1085_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq38_e1085_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq38_e1085_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq38_e1085_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq38_e1087: f64 = (eq38_e1085 * scratch.values[946]);
        let eq38_e1087_d_n0: f64 = ((eq38_e1085_d_n0 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][0]));
        let eq38_e1087_d_n1: f64 = ((eq38_e1085_d_n1 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][1]));
        let eq38_e1087_d_n2: f64 = ((eq38_e1085_d_n2 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][2]));
        let eq38_e1087_d_n3: f64 = ((eq38_e1085_d_n3 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][3]));
        let eq38_e1087_d_n4: f64 = ((eq38_e1085_d_n4 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][4]));
        let eq38_e1087_d_n5: f64 = ((eq38_e1085_d_n5 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][5]));
        let eq38_e1087_d_n6: f64 = ((eq38_e1085_d_n6 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][6]));
        let eq38_e1087_d_n7: f64 = ((eq38_e1085_d_n7 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][7]));
        let eq38_e1087_d_n8: f64 = ((eq38_e1085_d_n8 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][8]));
        let eq38_e1087_d_n9: f64 = ((eq38_e1085_d_n9 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][9]));
        let eq38_e1087_d_n10: f64 = ((eq38_e1085_d_n10 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][10]));
        let eq38_e1087_d_n11: f64 = ((eq38_e1085_d_n11 * scratch.values[946]) + (eq38_e1085 * scratch.node_derivatives[946][11]));
        let eq38_e1088_q: f64 = eq38_e1087;
        stamper.stamp_current_reactive(
            Some(self.nodes[8]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq38_e1087_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq38_e1087_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq38_e1087_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq38_e1087_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq38_e1087_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq38_e1087_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq38_e1087_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq38_e1087_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq38_e1087_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq38_e1087_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq38_e1087_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq38_e1087_d_n11)),
            ],
        );
        let eq39_e1091: f64 = (scratch.values[0] * scratch.values[25]);
        let eq39_e1091_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq39_e1091_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq39_e1091_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq39_e1091_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq39_e1091_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq39_e1091_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq39_e1091_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq39_e1091_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq39_e1091_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq39_e1091_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq39_e1091_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq39_e1091_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq39_e1093: f64 = (eq39_e1091 * scratch.values[945]);
        let eq39_e1093_d_n0: f64 = ((eq39_e1091_d_n0 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][0]));
        let eq39_e1093_d_n1: f64 = ((eq39_e1091_d_n1 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][1]));
        let eq39_e1093_d_n2: f64 = ((eq39_e1091_d_n2 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][2]));
        let eq39_e1093_d_n3: f64 = ((eq39_e1091_d_n3 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][3]));
        let eq39_e1093_d_n4: f64 = ((eq39_e1091_d_n4 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][4]));
        let eq39_e1093_d_n5: f64 = ((eq39_e1091_d_n5 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][5]));
        let eq39_e1093_d_n6: f64 = ((eq39_e1091_d_n6 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][6]));
        let eq39_e1093_d_n7: f64 = ((eq39_e1091_d_n7 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][7]));
        let eq39_e1093_d_n8: f64 = ((eq39_e1091_d_n8 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][8]));
        let eq39_e1093_d_n9: f64 = ((eq39_e1091_d_n9 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][9]));
        let eq39_e1093_d_n10: f64 = ((eq39_e1091_d_n10 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][10]));
        let eq39_e1093_d_n11: f64 = ((eq39_e1091_d_n11 * scratch.values[945]) + (eq39_e1091 * scratch.node_derivatives[945][11]));
        let eq39_e1094_q: f64 = eq39_e1093;
        stamper.stamp_current_reactive(
            Some(self.nodes[7]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq39_e1093_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq39_e1093_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq39_e1093_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq39_e1093_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq39_e1093_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq39_e1093_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq39_e1093_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq39_e1093_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq39_e1093_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq39_e1093_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq39_e1093_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq39_e1093_d_n11)),
            ],
        );
        let eq40_e1097: f64 = (scratch.values[0] * scratch.values[25]);
        let eq40_e1097_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq40_e1097_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq40_e1097_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq40_e1097_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq40_e1097_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq40_e1097_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq40_e1097_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq40_e1097_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq40_e1097_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq40_e1097_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq40_e1097_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq40_e1097_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq40_e1099: f64 = (eq40_e1097 * scratch.values[951]);
        let eq40_e1099_d_n0: f64 = ((eq40_e1097_d_n0 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][0]));
        let eq40_e1099_d_n1: f64 = ((eq40_e1097_d_n1 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][1]));
        let eq40_e1099_d_n2: f64 = ((eq40_e1097_d_n2 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][2]));
        let eq40_e1099_d_n3: f64 = ((eq40_e1097_d_n3 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][3]));
        let eq40_e1099_d_n4: f64 = ((eq40_e1097_d_n4 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][4]));
        let eq40_e1099_d_n5: f64 = ((eq40_e1097_d_n5 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][5]));
        let eq40_e1099_d_n6: f64 = ((eq40_e1097_d_n6 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][6]));
        let eq40_e1099_d_n7: f64 = ((eq40_e1097_d_n7 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][7]));
        let eq40_e1099_d_n8: f64 = ((eq40_e1097_d_n8 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][8]));
        let eq40_e1099_d_n9: f64 = ((eq40_e1097_d_n9 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][9]));
        let eq40_e1099_d_n10: f64 = ((eq40_e1097_d_n10 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][10]));
        let eq40_e1099_d_n11: f64 = ((eq40_e1097_d_n11 * scratch.values[951]) + (eq40_e1097 * scratch.node_derivatives[951][11]));
        let eq40_e1100_q: f64 = eq40_e1099;
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq40_e1099_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq40_e1099_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq40_e1099_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq40_e1099_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq40_e1099_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq40_e1099_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq40_e1099_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq40_e1099_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq40_e1099_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq40_e1099_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq40_e1099_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq40_e1099_d_n11)),
            ],
        );
        let eq41_e1103: f64 = (scratch.values[0] * scratch.values[25]);
        let eq41_e1103_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq41_e1103_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq41_e1103_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq41_e1103_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq41_e1103_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq41_e1103_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq41_e1103_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq41_e1103_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq41_e1103_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq41_e1103_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq41_e1103_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq41_e1103_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq41_e1105: f64 = (eq41_e1103 * scratch.values[952]);
        let eq41_e1105_d_n0: f64 = ((eq41_e1103_d_n0 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][0]));
        let eq41_e1105_d_n1: f64 = ((eq41_e1103_d_n1 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][1]));
        let eq41_e1105_d_n2: f64 = ((eq41_e1103_d_n2 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][2]));
        let eq41_e1105_d_n3: f64 = ((eq41_e1103_d_n3 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][3]));
        let eq41_e1105_d_n4: f64 = ((eq41_e1103_d_n4 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][4]));
        let eq41_e1105_d_n5: f64 = ((eq41_e1103_d_n5 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][5]));
        let eq41_e1105_d_n6: f64 = ((eq41_e1103_d_n6 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][6]));
        let eq41_e1105_d_n7: f64 = ((eq41_e1103_d_n7 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][7]));
        let eq41_e1105_d_n8: f64 = ((eq41_e1103_d_n8 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][8]));
        let eq41_e1105_d_n9: f64 = ((eq41_e1103_d_n9 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][9]));
        let eq41_e1105_d_n10: f64 = ((eq41_e1103_d_n10 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][10]));
        let eq41_e1105_d_n11: f64 = ((eq41_e1103_d_n11 * scratch.values[952]) + (eq41_e1103 * scratch.node_derivatives[952][11]));
        let eq41_e1106_q: f64 = eq41_e1105;
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq41_e1105_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq41_e1105_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq41_e1105_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq41_e1105_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq41_e1105_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq41_e1105_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq41_e1105_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq41_e1105_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq41_e1105_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq41_e1105_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq41_e1105_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq41_e1105_d_n11)),
            ],
        );
        let eq42_e1109: f64 = (scratch.values[0] * scratch.values[25]);
        let eq42_e1109_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq42_e1109_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq42_e1109_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq42_e1109_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq42_e1109_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq42_e1109_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq42_e1109_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq42_e1109_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq42_e1109_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq42_e1109_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq42_e1109_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq42_e1109_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq42_e1111: f64 = (eq42_e1109 * scratch.values[950]);
        let eq42_e1111_d_n0: f64 = ((eq42_e1109_d_n0 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][0]));
        let eq42_e1111_d_n1: f64 = ((eq42_e1109_d_n1 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][1]));
        let eq42_e1111_d_n2: f64 = ((eq42_e1109_d_n2 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][2]));
        let eq42_e1111_d_n3: f64 = ((eq42_e1109_d_n3 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][3]));
        let eq42_e1111_d_n4: f64 = ((eq42_e1109_d_n4 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][4]));
        let eq42_e1111_d_n5: f64 = ((eq42_e1109_d_n5 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][5]));
        let eq42_e1111_d_n6: f64 = ((eq42_e1109_d_n6 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][6]));
        let eq42_e1111_d_n7: f64 = ((eq42_e1109_d_n7 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][7]));
        let eq42_e1111_d_n8: f64 = ((eq42_e1109_d_n8 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][8]));
        let eq42_e1111_d_n9: f64 = ((eq42_e1109_d_n9 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][9]));
        let eq42_e1111_d_n10: f64 = ((eq42_e1109_d_n10 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][10]));
        let eq42_e1111_d_n11: f64 = ((eq42_e1109_d_n11 * scratch.values[950]) + (eq42_e1109 * scratch.node_derivatives[950][11]));
        let eq42_e1112_q: f64 = eq42_e1111;
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[8]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq42_e1111_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq42_e1111_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq42_e1111_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq42_e1111_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq42_e1111_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq42_e1111_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq42_e1111_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq42_e1111_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq42_e1111_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq42_e1111_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq42_e1111_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq42_e1111_d_n11)),
            ],
        );
        let eq43_e1115: f64 = (scratch.values[0] * scratch.values[25]);
        let eq43_e1115_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq43_e1115_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq43_e1115_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq43_e1115_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq43_e1115_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq43_e1115_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq43_e1115_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq43_e1115_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq43_e1115_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq43_e1115_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq43_e1115_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq43_e1115_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq43_e1117: f64 = (eq43_e1115 * scratch.values[2041]);
        let eq43_e1117_d_n0: f64 = ((eq43_e1115_d_n0 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][0]));
        let eq43_e1117_d_n1: f64 = ((eq43_e1115_d_n1 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][1]));
        let eq43_e1117_d_n2: f64 = ((eq43_e1115_d_n2 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][2]));
        let eq43_e1117_d_n3: f64 = ((eq43_e1115_d_n3 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][3]));
        let eq43_e1117_d_n4: f64 = ((eq43_e1115_d_n4 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][4]));
        let eq43_e1117_d_n5: f64 = ((eq43_e1115_d_n5 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][5]));
        let eq43_e1117_d_n6: f64 = ((eq43_e1115_d_n6 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][6]));
        let eq43_e1117_d_n7: f64 = ((eq43_e1115_d_n7 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][7]));
        let eq43_e1117_d_n8: f64 = ((eq43_e1115_d_n8 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][8]));
        let eq43_e1117_d_n9: f64 = ((eq43_e1115_d_n9 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][9]));
        let eq43_e1117_d_n10: f64 = ((eq43_e1115_d_n10 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][10]));
        let eq43_e1117_d_n11: f64 = ((eq43_e1115_d_n11 * scratch.values[2041]) + (eq43_e1115 * scratch.node_derivatives[2041][11]));
        let eq43_e1118_q: f64 = eq43_e1117;
        stamper.stamp_current_reactive(
            Some(self.nodes[10]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq43_e1117_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq43_e1117_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq43_e1117_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq43_e1117_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq43_e1117_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq43_e1117_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq43_e1117_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq43_e1117_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq43_e1117_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq43_e1117_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq43_e1117_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq43_e1117_d_n11)),
            ],
        );
        let eq44_e1121: f64 = (scratch.values[0] * scratch.values[25]);
        let eq44_e1121_d_n0: f64 = ((scratch.node_derivatives[0][0] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][0]));
        let eq44_e1121_d_n1: f64 = ((scratch.node_derivatives[0][1] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][1]));
        let eq44_e1121_d_n2: f64 = ((scratch.node_derivatives[0][2] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][2]));
        let eq44_e1121_d_n3: f64 = ((scratch.node_derivatives[0][3] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][3]));
        let eq44_e1121_d_n4: f64 = ((scratch.node_derivatives[0][4] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][4]));
        let eq44_e1121_d_n5: f64 = ((scratch.node_derivatives[0][5] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][5]));
        let eq44_e1121_d_n6: f64 = ((scratch.node_derivatives[0][6] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][6]));
        let eq44_e1121_d_n7: f64 = ((scratch.node_derivatives[0][7] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][7]));
        let eq44_e1121_d_n8: f64 = ((scratch.node_derivatives[0][8] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][8]));
        let eq44_e1121_d_n9: f64 = ((scratch.node_derivatives[0][9] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][9]));
        let eq44_e1121_d_n10: f64 = ((scratch.node_derivatives[0][10] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][10]));
        let eq44_e1121_d_n11: f64 = ((scratch.node_derivatives[0][11] * scratch.values[25]) + (scratch.values[0] * scratch.node_derivatives[25][11]));
        let eq44_e1123: f64 = (eq44_e1121 * scratch.values[2045]);
        let eq44_e1123_d_n0: f64 = ((eq44_e1121_d_n0 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][0]));
        let eq44_e1123_d_n1: f64 = ((eq44_e1121_d_n1 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][1]));
        let eq44_e1123_d_n2: f64 = ((eq44_e1121_d_n2 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][2]));
        let eq44_e1123_d_n3: f64 = ((eq44_e1121_d_n3 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][3]));
        let eq44_e1123_d_n4: f64 = ((eq44_e1121_d_n4 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][4]));
        let eq44_e1123_d_n5: f64 = ((eq44_e1121_d_n5 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][5]));
        let eq44_e1123_d_n6: f64 = ((eq44_e1121_d_n6 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][6]));
        let eq44_e1123_d_n7: f64 = ((eq44_e1121_d_n7 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][7]));
        let eq44_e1123_d_n8: f64 = ((eq44_e1121_d_n8 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][8]));
        let eq44_e1123_d_n9: f64 = ((eq44_e1121_d_n9 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][9]));
        let eq44_e1123_d_n10: f64 = ((eq44_e1121_d_n10 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][10]));
        let eq44_e1123_d_n11: f64 = ((eq44_e1121_d_n11 * scratch.values[2045]) + (eq44_e1121 * scratch.node_derivatives[2045][11]));
        let eq44_e1124_q: f64 = eq44_e1123;
        stamper.stamp_current_reactive(
            Some(self.nodes[11]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq44_e1123_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq44_e1123_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq44_e1123_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq44_e1123_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq44_e1123_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq44_e1123_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq44_e1123_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq44_e1123_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq44_e1123_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq44_e1123_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq44_e1123_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq44_e1123_d_n11)),
            ],
        );
        let eq47_e1135: f64 = (scratch.values[1006] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n0: f64 = (scratch.node_derivatives[1006][0] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n1: f64 = (scratch.node_derivatives[1006][1] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n2: f64 = (scratch.node_derivatives[1006][2] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n3: f64 = (scratch.node_derivatives[1006][3] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n4: f64 = ((scratch.node_derivatives[1006][4] * (ctx.node_voltage(self.nodes[4]) - 0.0)) + scratch.values[1006]);
        let eq47_e1135_d_n5: f64 = (scratch.node_derivatives[1006][5] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n6: f64 = (scratch.node_derivatives[1006][6] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n7: f64 = (scratch.node_derivatives[1006][7] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n8: f64 = (scratch.node_derivatives[1006][8] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n9: f64 = (scratch.node_derivatives[1006][9] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n10: f64 = (scratch.node_derivatives[1006][10] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1135_d_n11: f64 = (scratch.node_derivatives[1006][11] * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq47_e1136_q: f64 = eq47_e1135;
        stamper.stamp_current_reactive(
            Some(self.nodes[4]),
            None,
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq47_e1135_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq47_e1135_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq47_e1135_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq47_e1135_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq47_e1135_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq47_e1135_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq47_e1135_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq47_e1135_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq47_e1135_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq47_e1135_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq47_e1135_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq47_e1135_d_n11)),
            ],
        );
        let eq48_e1138: f64 = (scratch.values[25]).sqrt();
        let eq48_e1138_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq48_e1138));
        let eq48_e1138_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq48_e1138));
        let eq48_e1140: f64 = (eq48_e1138 * 0.5);
        let eq48_e1140_d_n0: f64 = (eq48_e1138_d_n0 * 0.5);
        let eq48_e1140_d_n1: f64 = (eq48_e1138_d_n1 * 0.5);
        let eq48_e1140_d_n2: f64 = (eq48_e1138_d_n2 * 0.5);
        let eq48_e1140_d_n3: f64 = (eq48_e1138_d_n3 * 0.5);
        let eq48_e1140_d_n4: f64 = (eq48_e1138_d_n4 * 0.5);
        let eq48_e1140_d_n5: f64 = (eq48_e1138_d_n5 * 0.5);
        let eq48_e1140_d_n6: f64 = (eq48_e1138_d_n6 * 0.5);
        let eq48_e1140_d_n7: f64 = (eq48_e1138_d_n7 * 0.5);
        let eq48_e1140_d_n8: f64 = (eq48_e1138_d_n8 * 0.5);
        let eq48_e1140_d_n9: f64 = (eq48_e1138_d_n9 * 0.5);
        let eq48_e1140_d_n10: f64 = (eq48_e1138_d_n10 * 0.5);
        let eq48_e1140_d_n11: f64 = (eq48_e1138_d_n11 * 0.5);
        let eq48_e1142: f64 = (eq48_e1140 * scratch.values[1006]);
        let eq48_e1142_d_n0: f64 = ((eq48_e1140_d_n0 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][0]));
        let eq48_e1142_d_n1: f64 = ((eq48_e1140_d_n1 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][1]));
        let eq48_e1142_d_n2: f64 = ((eq48_e1140_d_n2 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][2]));
        let eq48_e1142_d_n3: f64 = ((eq48_e1140_d_n3 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][3]));
        let eq48_e1142_d_n4: f64 = ((eq48_e1140_d_n4 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][4]));
        let eq48_e1142_d_n5: f64 = ((eq48_e1140_d_n5 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][5]));
        let eq48_e1142_d_n6: f64 = ((eq48_e1140_d_n6 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][6]));
        let eq48_e1142_d_n7: f64 = ((eq48_e1140_d_n7 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][7]));
        let eq48_e1142_d_n8: f64 = ((eq48_e1140_d_n8 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][8]));
        let eq48_e1142_d_n9: f64 = ((eq48_e1140_d_n9 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][9]));
        let eq48_e1142_d_n10: f64 = ((eq48_e1140_d_n10 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][10]));
        let eq48_e1142_d_n11: f64 = ((eq48_e1140_d_n11 * scratch.values[1006]) + (eq48_e1140 * scratch.node_derivatives[1006][11]));
        let eq48_e1144: f64 = (eq48_e1142 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n0: f64 = (eq48_e1142_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n1: f64 = (eq48_e1142_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n2: f64 = (eq48_e1142_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n3: f64 = (eq48_e1142_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n4: f64 = ((eq48_e1142_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq48_e1142);
        let eq48_e1144_d_n5: f64 = (eq48_e1142_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n6: f64 = (eq48_e1142_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n7: f64 = (eq48_e1142_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n8: f64 = (eq48_e1142_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n9: f64 = (eq48_e1142_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n10: f64 = (eq48_e1142_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1144_d_n11: f64 = (eq48_e1142_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq48_e1145_q: f64 = eq48_e1144;
        let eq48_e1146: f64 = (-eq48_e1144);
        let eq48_e1146_d_n0: f64 = (-eq48_e1144_d_n0);
        let eq48_e1146_d_n1: f64 = (-eq48_e1144_d_n1);
        let eq48_e1146_d_n2: f64 = (-eq48_e1144_d_n2);
        let eq48_e1146_d_n3: f64 = (-eq48_e1144_d_n3);
        let eq48_e1146_d_n4: f64 = (-eq48_e1144_d_n4);
        let eq48_e1146_d_n5: f64 = (-eq48_e1144_d_n5);
        let eq48_e1146_d_n6: f64 = (-eq48_e1144_d_n6);
        let eq48_e1146_d_n7: f64 = (-eq48_e1144_d_n7);
        let eq48_e1146_d_n8: f64 = (-eq48_e1144_d_n8);
        let eq48_e1146_d_n9: f64 = (-eq48_e1144_d_n9);
        let eq48_e1146_d_n10: f64 = (-eq48_e1144_d_n10);
        let eq48_e1146_d_n11: f64 = (-eq48_e1144_d_n11);
        let eq48_e1146_q: f64 = (-eq48_e1145_q);
        let eq48_e1146_q_d_n0: f64 = (-eq48_e1144_d_n0);
        let eq48_e1146_q_d_n1: f64 = (-eq48_e1144_d_n1);
        let eq48_e1146_q_d_n2: f64 = (-eq48_e1144_d_n2);
        let eq48_e1146_q_d_n3: f64 = (-eq48_e1144_d_n3);
        let eq48_e1146_q_d_n4: f64 = (-eq48_e1144_d_n4);
        let eq48_e1146_q_d_n5: f64 = (-eq48_e1144_d_n5);
        let eq48_e1146_q_d_n6: f64 = (-eq48_e1144_d_n6);
        let eq48_e1146_q_d_n7: f64 = (-eq48_e1144_d_n7);
        let eq48_e1146_q_d_n8: f64 = (-eq48_e1144_d_n8);
        let eq48_e1146_q_d_n9: f64 = (-eq48_e1144_d_n9);
        let eq48_e1146_q_d_n10: f64 = (-eq48_e1144_d_n10);
        let eq48_e1146_q_d_n11: f64 = (-eq48_e1144_d_n11);
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[6]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq48_e1146_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq48_e1146_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq48_e1146_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq48_e1146_q_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq48_e1146_q_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq48_e1146_q_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq48_e1146_q_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq48_e1146_q_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq48_e1146_q_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq48_e1146_q_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq48_e1146_q_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq48_e1146_q_d_n11)),
            ],
        );
        let eq49_e1148: f64 = (scratch.values[25]).sqrt();
        let eq49_e1148_d_n0: f64 = (scratch.node_derivatives[25][0] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n1: f64 = (scratch.node_derivatives[25][1] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n2: f64 = (scratch.node_derivatives[25][2] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n3: f64 = (scratch.node_derivatives[25][3] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n4: f64 = (scratch.node_derivatives[25][4] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n5: f64 = (scratch.node_derivatives[25][5] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n6: f64 = (scratch.node_derivatives[25][6] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n7: f64 = (scratch.node_derivatives[25][7] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n8: f64 = (scratch.node_derivatives[25][8] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n9: f64 = (scratch.node_derivatives[25][9] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n10: f64 = (scratch.node_derivatives[25][10] / (2.0 * eq49_e1148));
        let eq49_e1148_d_n11: f64 = (scratch.node_derivatives[25][11] / (2.0 * eq49_e1148));
        let eq49_e1150: f64 = (eq49_e1148 * 0.5);
        let eq49_e1150_d_n0: f64 = (eq49_e1148_d_n0 * 0.5);
        let eq49_e1150_d_n1: f64 = (eq49_e1148_d_n1 * 0.5);
        let eq49_e1150_d_n2: f64 = (eq49_e1148_d_n2 * 0.5);
        let eq49_e1150_d_n3: f64 = (eq49_e1148_d_n3 * 0.5);
        let eq49_e1150_d_n4: f64 = (eq49_e1148_d_n4 * 0.5);
        let eq49_e1150_d_n5: f64 = (eq49_e1148_d_n5 * 0.5);
        let eq49_e1150_d_n6: f64 = (eq49_e1148_d_n6 * 0.5);
        let eq49_e1150_d_n7: f64 = (eq49_e1148_d_n7 * 0.5);
        let eq49_e1150_d_n8: f64 = (eq49_e1148_d_n8 * 0.5);
        let eq49_e1150_d_n9: f64 = (eq49_e1148_d_n9 * 0.5);
        let eq49_e1150_d_n10: f64 = (eq49_e1148_d_n10 * 0.5);
        let eq49_e1150_d_n11: f64 = (eq49_e1148_d_n11 * 0.5);
        let eq49_e1152: f64 = (eq49_e1150 * scratch.values[1006]);
        let eq49_e1152_d_n0: f64 = ((eq49_e1150_d_n0 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][0]));
        let eq49_e1152_d_n1: f64 = ((eq49_e1150_d_n1 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][1]));
        let eq49_e1152_d_n2: f64 = ((eq49_e1150_d_n2 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][2]));
        let eq49_e1152_d_n3: f64 = ((eq49_e1150_d_n3 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][3]));
        let eq49_e1152_d_n4: f64 = ((eq49_e1150_d_n4 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][4]));
        let eq49_e1152_d_n5: f64 = ((eq49_e1150_d_n5 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][5]));
        let eq49_e1152_d_n6: f64 = ((eq49_e1150_d_n6 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][6]));
        let eq49_e1152_d_n7: f64 = ((eq49_e1150_d_n7 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][7]));
        let eq49_e1152_d_n8: f64 = ((eq49_e1150_d_n8 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][8]));
        let eq49_e1152_d_n9: f64 = ((eq49_e1150_d_n9 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][9]));
        let eq49_e1152_d_n10: f64 = ((eq49_e1150_d_n10 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][10]));
        let eq49_e1152_d_n11: f64 = ((eq49_e1150_d_n11 * scratch.values[1006]) + (eq49_e1150 * scratch.node_derivatives[1006][11]));
        let eq49_e1154: f64 = (eq49_e1152 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n0: f64 = (eq49_e1152_d_n0 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n1: f64 = (eq49_e1152_d_n1 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n2: f64 = (eq49_e1152_d_n2 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n3: f64 = (eq49_e1152_d_n3 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n4: f64 = ((eq49_e1152_d_n4 * (ctx.node_voltage(self.nodes[4]) - 0.0)) + eq49_e1152);
        let eq49_e1154_d_n5: f64 = (eq49_e1152_d_n5 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n6: f64 = (eq49_e1152_d_n6 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n7: f64 = (eq49_e1152_d_n7 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n8: f64 = (eq49_e1152_d_n8 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n9: f64 = (eq49_e1152_d_n9 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n10: f64 = (eq49_e1152_d_n10 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1154_d_n11: f64 = (eq49_e1152_d_n11 * (ctx.node_voltage(self.nodes[4]) - 0.0));
        let eq49_e1155_q: f64 = eq49_e1154;
        let eq49_e1156: f64 = (-eq49_e1154);
        let eq49_e1156_d_n0: f64 = (-eq49_e1154_d_n0);
        let eq49_e1156_d_n1: f64 = (-eq49_e1154_d_n1);
        let eq49_e1156_d_n2: f64 = (-eq49_e1154_d_n2);
        let eq49_e1156_d_n3: f64 = (-eq49_e1154_d_n3);
        let eq49_e1156_d_n4: f64 = (-eq49_e1154_d_n4);
        let eq49_e1156_d_n5: f64 = (-eq49_e1154_d_n5);
        let eq49_e1156_d_n6: f64 = (-eq49_e1154_d_n6);
        let eq49_e1156_d_n7: f64 = (-eq49_e1154_d_n7);
        let eq49_e1156_d_n8: f64 = (-eq49_e1154_d_n8);
        let eq49_e1156_d_n9: f64 = (-eq49_e1154_d_n9);
        let eq49_e1156_d_n10: f64 = (-eq49_e1154_d_n10);
        let eq49_e1156_d_n11: f64 = (-eq49_e1154_d_n11);
        let eq49_e1156_q: f64 = (-eq49_e1155_q);
        let eq49_e1156_q_d_n0: f64 = (-eq49_e1154_d_n0);
        let eq49_e1156_q_d_n1: f64 = (-eq49_e1154_d_n1);
        let eq49_e1156_q_d_n2: f64 = (-eq49_e1154_d_n2);
        let eq49_e1156_q_d_n3: f64 = (-eq49_e1154_d_n3);
        let eq49_e1156_q_d_n4: f64 = (-eq49_e1154_d_n4);
        let eq49_e1156_q_d_n5: f64 = (-eq49_e1154_d_n5);
        let eq49_e1156_q_d_n6: f64 = (-eq49_e1154_d_n6);
        let eq49_e1156_q_d_n7: f64 = (-eq49_e1154_d_n7);
        let eq49_e1156_q_d_n8: f64 = (-eq49_e1154_d_n8);
        let eq49_e1156_q_d_n9: f64 = (-eq49_e1154_d_n9);
        let eq49_e1156_q_d_n10: f64 = (-eq49_e1154_d_n10);
        let eq49_e1156_q_d_n11: f64 = (-eq49_e1154_d_n11);
        stamper.stamp_current_reactive(
            Some(self.nodes[5]),
            Some(self.nodes[7]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq49_e1156_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq49_e1156_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq49_e1156_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq49_e1156_q_d_n3)),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (eq49_e1156_q_d_n4)),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (eq49_e1156_q_d_n5)),
                GeneratedDerivative::node(self.nodes[6], self.multiplicity * (eq49_e1156_q_d_n6)),
                GeneratedDerivative::node(self.nodes[7], self.multiplicity * (eq49_e1156_q_d_n7)),
                GeneratedDerivative::node(self.nodes[8], self.multiplicity * (eq49_e1156_q_d_n8)),
                GeneratedDerivative::node(self.nodes[9], self.multiplicity * (eq49_e1156_q_d_n9)),
                GeneratedDerivative::node(self.nodes[10], self.multiplicity * (eq49_e1156_q_d_n10)),
                GeneratedDerivative::node(self.nodes[11], self.multiplicity * (eq49_e1156_q_d_n11)),
            ],
        );
    }
}
