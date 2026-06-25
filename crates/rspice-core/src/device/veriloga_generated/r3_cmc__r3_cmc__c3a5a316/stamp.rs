#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;
#[path = "stamp_blocks_1.rs"]
mod stamp_blocks_1;

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

        stamper.stamp_potential_branch(
            Some(self.nodes[0]),
            Some(self.nodes[4]),
            self.branches[0],
            self.multiplicity,
        );
        stamper.stamp_potential_branch(
            Some(self.nodes[2]),
            Some(self.nodes[5]),
            self.branches[1],
            self.multiplicity,
        );

        let eq0_value: f64 = scratch.values[81];
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[4]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * scratch.node_derivatives[81][0]),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * scratch.node_derivatives[81][1]),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * scratch.node_derivatives[81][2]),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * scratch.node_derivatives[81][3]),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * scratch.node_derivatives[81][4]),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * scratch.node_derivatives[81][5]),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * scratch.branch_derivatives[81][0]),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * scratch.branch_derivatives[81][1]),
            ],
        );
        let eq1_value: f64 = scratch.values[82];
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[4]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * scratch.node_derivatives[82][0]),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * scratch.node_derivatives[82][1]),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * scratch.node_derivatives[82][2]),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * scratch.node_derivatives[82][3]),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * scratch.node_derivatives[82][4]),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * scratch.node_derivatives[82][5]),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * scratch.branch_derivatives[82][0]),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * scratch.branch_derivatives[82][1]),
            ],
        );
        let eq2_value: f64 = scratch.values[83];
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            self.multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * scratch.node_derivatives[83][0]),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * scratch.node_derivatives[83][1]),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * scratch.node_derivatives[83][2]),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * scratch.node_derivatives[83][3]),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * scratch.node_derivatives[83][4]),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * scratch.node_derivatives[83][5]),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * scratch.branch_derivatives[83][0]),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * scratch.branch_derivatives[83][1]),
            ],
        );
        let eq3_value: f64 = scratch.values[95];
        stamper.stamp_current(
            Some(self.nodes[3]),
            None,
            self.multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * scratch.node_derivatives[95][0]),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * scratch.node_derivatives[95][1]),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * scratch.node_derivatives[95][2]),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * scratch.node_derivatives[95][3]),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * scratch.node_derivatives[95][4]),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * scratch.node_derivatives[95][5]),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * scratch.branch_derivatives[95][0]),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * scratch.branch_derivatives[95][1]),
            ],
        );
        let eq4_value: f64 = scratch.values[94];
        stamper.stamp_current(
            Some(self.nodes[3]),
            None,
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * scratch.node_derivatives[94][0]),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * scratch.node_derivatives[94][1]),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * scratch.node_derivatives[94][2]),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * scratch.node_derivatives[94][3]),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * scratch.node_derivatives[94][4]),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * scratch.node_derivatives[94][5]),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * scratch.branch_derivatives[94][0]),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * scratch.branch_derivatives[94][1]),
            ],
        );
        let (eq5_e153, eq5_e153_d_n0, eq5_e153_d_n1, eq5_e153_d_n2, eq5_e153_d_n3, eq5_e153_d_n4, eq5_e153_d_n5, eq5_e153_d_b0, eq5_e153_d_b1,): (f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[321] != 0.0) {
        let eq5_e149: f64 = (ctx.branch_current(self.branches[0]) * scratch.values[54]);
        let eq5_e149_d_n0: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][0]);
        let eq5_e149_d_n1: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][1]);
        let eq5_e149_d_n2: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][2]);
        let eq5_e149_d_n3: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][3]);
        let eq5_e149_d_n4: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][4]);
        let eq5_e149_d_n5: f64 = (ctx.branch_current(self.branches[0]) * scratch.node_derivatives[54][5]);
        let eq5_e149_d_b0: f64 = (scratch.values[54] + (ctx.branch_current(self.branches[0]) * scratch.branch_derivatives[54][0]));
        let eq5_e149_d_b1: f64 = (ctx.branch_current(self.branches[0]) * scratch.branch_derivatives[54][1]);
        let eq5_e151: f64 = (eq5_e149 * scratch.values[58]);
        let eq5_e151_d_n0: f64 = ((eq5_e149_d_n0 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][0]));
        let eq5_e151_d_n1: f64 = ((eq5_e149_d_n1 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][1]));
        let eq5_e151_d_n2: f64 = ((eq5_e149_d_n2 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][2]));
        let eq5_e151_d_n3: f64 = ((eq5_e149_d_n3 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][3]));
        let eq5_e151_d_n4: f64 = ((eq5_e149_d_n4 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][4]));
        let eq5_e151_d_n5: f64 = ((eq5_e149_d_n5 * scratch.values[58]) + (eq5_e149 * scratch.node_derivatives[58][5]));
        let eq5_e151_d_b0: f64 = ((eq5_e149_d_b0 * scratch.values[58]) + (eq5_e149 * scratch.branch_derivatives[58][0]));
        let eq5_e151_d_b1: f64 = ((eq5_e149_d_b1 * scratch.values[58]) + (eq5_e149 * scratch.branch_derivatives[58][1]));
        (eq5_e151, eq5_e151_d_n0, eq5_e151_d_n1, eq5_e151_d_n2, eq5_e151_d_n3, eq5_e151_d_n4, eq5_e151_d_n5, eq5_e151_d_b0, eq5_e151_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e153;
        stamper.stamp_potential(
            self.branches[0],
            eq5_value,
            &[
                GeneratedDerivative::node(self.nodes[0], eq5_e153_d_n0),
                GeneratedDerivative::node(self.nodes[1], eq5_e153_d_n1),
                GeneratedDerivative::node(self.nodes[2], eq5_e153_d_n2),
                GeneratedDerivative::node(self.nodes[3], eq5_e153_d_n3),
                GeneratedDerivative::node(self.nodes[4], eq5_e153_d_n4),
                GeneratedDerivative::node(self.nodes[5], eq5_e153_d_n5),
                GeneratedDerivative::branch(self.branches[0], eq5_e153_d_b0),
                GeneratedDerivative::branch(self.branches[1], eq5_e153_d_b1),
            ],
        );
        let (eq6_e162, eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5, eq6_e162_d_b0, eq6_e162_d_b1,): (f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[321] != 0.0)) {
        let eq6_e159: f64 = (scratch.values[54] * scratch.values[58]);
        let eq6_e159_d_n0: f64 = ((scratch.node_derivatives[54][0] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][0]));
        let eq6_e159_d_n1: f64 = ((scratch.node_derivatives[54][1] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][1]));
        let eq6_e159_d_n2: f64 = ((scratch.node_derivatives[54][2] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][2]));
        let eq6_e159_d_n3: f64 = ((scratch.node_derivatives[54][3] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][3]));
        let eq6_e159_d_n4: f64 = ((scratch.node_derivatives[54][4] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][4]));
        let eq6_e159_d_n5: f64 = ((scratch.node_derivatives[54][5] * scratch.values[58]) + (scratch.values[54] * scratch.node_derivatives[58][5]));
        let eq6_e159_d_b0: f64 = ((scratch.branch_derivatives[54][0] * scratch.values[58]) + (scratch.values[54] * scratch.branch_derivatives[58][0]));
        let eq6_e159_d_b1: f64 = ((scratch.branch_derivatives[54][1] * scratch.values[58]) + (scratch.values[54] * scratch.branch_derivatives[58][1]));
        let eq6_e160: f64 = ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) / eq6_e159);
        let eq6_e160_d_n0: f64 = ((eq6_e159 - ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n0)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n1: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n1) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n2: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n2) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n3: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n3) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n4: f64 = (((-1.0 * eq6_e159) - ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n4)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n5: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_n5) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b0: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_b0) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b1: f64 = (-(((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[4])) * eq6_e159_d_b1) / (eq6_e159 * eq6_e159)));
        (eq6_e160, eq6_e160_d_n0, eq6_e160_d_n1, eq6_e160_d_n2, eq6_e160_d_n3, eq6_e160_d_n4, eq6_e160_d_n5, eq6_e160_d_b0, eq6_e160_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e162;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[4]),
            self.multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq6_e162_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq6_e162_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq6_e162_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq6_e162_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq6_e162_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq6_e162_d_n5),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * eq6_e162_d_b0),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * eq6_e162_d_b1),
            ],
        );
        let (eq7_e170, eq7_e170_d_n0, eq7_e170_d_n1, eq7_e170_d_n2, eq7_e170_d_n3, eq7_e170_d_n4, eq7_e170_d_n5, eq7_e170_d_b0, eq7_e170_d_b1,): (f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[322] != 0.0) {
        let eq7_e166: f64 = (ctx.branch_current(self.branches[1]) * scratch.values[55]);
        let eq7_e166_d_n0: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][0]);
        let eq7_e166_d_n1: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][1]);
        let eq7_e166_d_n2: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][2]);
        let eq7_e166_d_n3: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][3]);
        let eq7_e166_d_n4: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][4]);
        let eq7_e166_d_n5: f64 = (ctx.branch_current(self.branches[1]) * scratch.node_derivatives[55][5]);
        let eq7_e166_d_b0: f64 = (ctx.branch_current(self.branches[1]) * scratch.branch_derivatives[55][0]);
        let eq7_e166_d_b1: f64 = (scratch.values[55] + (ctx.branch_current(self.branches[1]) * scratch.branch_derivatives[55][1]));
        let eq7_e168: f64 = (eq7_e166 * scratch.values[58]);
        let eq7_e168_d_n0: f64 = ((eq7_e166_d_n0 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][0]));
        let eq7_e168_d_n1: f64 = ((eq7_e166_d_n1 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][1]));
        let eq7_e168_d_n2: f64 = ((eq7_e166_d_n2 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][2]));
        let eq7_e168_d_n3: f64 = ((eq7_e166_d_n3 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][3]));
        let eq7_e168_d_n4: f64 = ((eq7_e166_d_n4 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][4]));
        let eq7_e168_d_n5: f64 = ((eq7_e166_d_n5 * scratch.values[58]) + (eq7_e166 * scratch.node_derivatives[58][5]));
        let eq7_e168_d_b0: f64 = ((eq7_e166_d_b0 * scratch.values[58]) + (eq7_e166 * scratch.branch_derivatives[58][0]));
        let eq7_e168_d_b1: f64 = ((eq7_e166_d_b1 * scratch.values[58]) + (eq7_e166 * scratch.branch_derivatives[58][1]));
        (eq7_e168, eq7_e168_d_n0, eq7_e168_d_n1, eq7_e168_d_n2, eq7_e168_d_n3, eq7_e168_d_n4, eq7_e168_d_n5, eq7_e168_d_b0, eq7_e168_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e170;
        stamper.stamp_potential(
            self.branches[1],
            eq7_value,
            &[
                GeneratedDerivative::node(self.nodes[0], eq7_e170_d_n0),
                GeneratedDerivative::node(self.nodes[1], eq7_e170_d_n1),
                GeneratedDerivative::node(self.nodes[2], eq7_e170_d_n2),
                GeneratedDerivative::node(self.nodes[3], eq7_e170_d_n3),
                GeneratedDerivative::node(self.nodes[4], eq7_e170_d_n4),
                GeneratedDerivative::node(self.nodes[5], eq7_e170_d_n5),
                GeneratedDerivative::branch(self.branches[0], eq7_e170_d_b0),
                GeneratedDerivative::branch(self.branches[1], eq7_e170_d_b1),
            ],
        );
        let (eq8_e179, eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5, eq8_e179_d_b0, eq8_e179_d_b1,): (f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[322] != 0.0)) {
        let eq8_e176: f64 = (scratch.values[55] * scratch.values[58]);
        let eq8_e176_d_n0: f64 = ((scratch.node_derivatives[55][0] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][0]));
        let eq8_e176_d_n1: f64 = ((scratch.node_derivatives[55][1] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][1]));
        let eq8_e176_d_n2: f64 = ((scratch.node_derivatives[55][2] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][2]));
        let eq8_e176_d_n3: f64 = ((scratch.node_derivatives[55][3] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][3]));
        let eq8_e176_d_n4: f64 = ((scratch.node_derivatives[55][4] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][4]));
        let eq8_e176_d_n5: f64 = ((scratch.node_derivatives[55][5] * scratch.values[58]) + (scratch.values[55] * scratch.node_derivatives[58][5]));
        let eq8_e176_d_b0: f64 = ((scratch.branch_derivatives[55][0] * scratch.values[58]) + (scratch.values[55] * scratch.branch_derivatives[58][0]));
        let eq8_e176_d_b1: f64 = ((scratch.branch_derivatives[55][1] * scratch.values[58]) + (scratch.values[55] * scratch.branch_derivatives[58][1]));
        let eq8_e177: f64 = ((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) / eq8_e176);
        let eq8_e177_d_n0: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n1: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n1) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n2: f64 = ((eq8_e176 - ((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n2)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_n3: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n3) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n4: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n4) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n5: f64 = (((-1.0 * eq8_e176) - ((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_n5)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_b0: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_b0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_b1: f64 = (-(((ctx.node_voltage(self.nodes[2]) - ctx.node_voltage(self.nodes[5])) * eq8_e176_d_b1) / (eq8_e176 * eq8_e176)));
        (eq8_e177, eq8_e177_d_n0, eq8_e177_d_n1, eq8_e177_d_n2, eq8_e177_d_n3, eq8_e177_d_n4, eq8_e177_d_n5, eq8_e177_d_b0, eq8_e177_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e179;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[5]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq8_e179_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq8_e179_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq8_e179_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq8_e179_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq8_e179_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq8_e179_d_n5),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * eq8_e179_d_b0),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * eq8_e179_d_b1),
            ],
        );
        let eq9_e181: f64 = self.eval_ddt(0, scratch.values[96]);
        let eq9_e181_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[96][0]);
        let eq9_e181_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[96][1]);
        let eq9_e181_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[96][2]);
        let eq9_e181_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[96][3]);
        let eq9_e181_d_n4: f64 = self.ddt_jacobian(scratch.node_derivatives[96][4]);
        let eq9_e181_d_n5: f64 = self.ddt_jacobian(scratch.node_derivatives[96][5]);
        let eq9_e181_d_b0: f64 = self.ddt_jacobian(scratch.branch_derivatives[96][0]);
        let eq9_e181_d_b1: f64 = self.ddt_jacobian(scratch.branch_derivatives[96][1]);
        let eq9_value: f64 = eq9_e181;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[4]),
            self.multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq9_e181_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq9_e181_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq9_e181_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq9_e181_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq9_e181_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq9_e181_d_n5),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * eq9_e181_d_b0),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * eq9_e181_d_b1),
            ],
        );
        let eq10_e183: f64 = self.eval_ddt(1, scratch.values[97]);
        let eq10_e183_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[97][0]);
        let eq10_e183_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[97][1]);
        let eq10_e183_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[97][2]);
        let eq10_e183_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[97][3]);
        let eq10_e183_d_n4: f64 = self.ddt_jacobian(scratch.node_derivatives[97][4]);
        let eq10_e183_d_n5: f64 = self.ddt_jacobian(scratch.node_derivatives[97][5]);
        let eq10_e183_d_b0: f64 = self.ddt_jacobian(scratch.branch_derivatives[97][0]);
        let eq10_e183_d_b1: f64 = self.ddt_jacobian(scratch.branch_derivatives[97][1]);
        let eq10_value: f64 = eq10_e183;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq10_e183_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq10_e183_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq10_e183_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq10_e183_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq10_e183_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq10_e183_d_n5),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * eq10_e183_d_b0),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * eq10_e183_d_b1),
            ],
        );
        let eq11_e185: f64 = self.eval_ddt(2, scratch.values[98]);
        let eq11_e185_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[98][0]);
        let eq11_e185_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[98][1]);
        let eq11_e185_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[98][2]);
        let eq11_e185_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[98][3]);
        let eq11_e185_d_n4: f64 = self.ddt_jacobian(scratch.node_derivatives[98][4]);
        let eq11_e185_d_n5: f64 = self.ddt_jacobian(scratch.node_derivatives[98][5]);
        let eq11_e185_d_b0: f64 = self.ddt_jacobian(scratch.branch_derivatives[98][0]);
        let eq11_e185_d_b1: f64 = self.ddt_jacobian(scratch.branch_derivatives[98][1]);
        let eq11_value: f64 = eq11_e185;
        stamper.stamp_current(
            Some(self.nodes[3]),
            None,
            self.multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq11_e185_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq11_e185_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq11_e185_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq11_e185_d_n3),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * eq11_e185_d_n4),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * eq11_e185_d_n5),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * eq11_e185_d_b0),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * eq11_e185_d_b1),
            ],
        );
        let (eq12_e191,): (f64,) = {
    if (self.params.sw_noise != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e191;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[4]),
            self.multiplicity * (eq12_value),
            &[
            ],
        );
        let (eq13_e198,): (f64,) = {
    if (self.params.sw_noise != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e198;
        stamper.stamp_current(
            Some(self.nodes[5]),
            Some(self.nodes[4]),
            self.multiplicity * (eq13_value),
            &[
            ],
        );
        let (eq14_e210,): (f64,) = {
    if (self.params.sw_noise != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e210;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[4]),
            self.multiplicity * (eq14_value),
            &[
            ],
        );
        let (eq15_e222,): (f64,) = {
    if (self.params.sw_noise != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e222;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[5]),
            self.multiplicity * (eq15_value),
            &[
            ],
        );
        let (eq16_e242,): (f64,) = {
    if ((self.params.sw_noise != 0.0) && (scratch.values[326] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e242;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[4]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
        let (eq17_e262,): (f64,) = {
    if ((self.params.sw_noise != 0.0) && (scratch.values[327] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e262;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            self.multiplicity * (eq17_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let mut scratch = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_1(ctx, stamper, &mut scratch);

        let eq9_e181_q: f64 = scratch.values[96];
        stamper.stamp_current_reactive(
            Some(self.nodes[1]),
            Some(self.nodes[4]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (scratch.node_derivatives[96][0])),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (scratch.node_derivatives[96][1])),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (scratch.node_derivatives[96][2])),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (scratch.node_derivatives[96][3])),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (scratch.node_derivatives[96][4])),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (scratch.node_derivatives[96][5])),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * (scratch.branch_derivatives[96][0])),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * (scratch.branch_derivatives[96][1])),
            ],
        );
        let eq10_e183_q: f64 = scratch.values[97];
        stamper.stamp_current_reactive(
            Some(self.nodes[1]),
            Some(self.nodes[5]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (scratch.node_derivatives[97][0])),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (scratch.node_derivatives[97][1])),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (scratch.node_derivatives[97][2])),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (scratch.node_derivatives[97][3])),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (scratch.node_derivatives[97][4])),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (scratch.node_derivatives[97][5])),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * (scratch.branch_derivatives[97][0])),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * (scratch.branch_derivatives[97][1])),
            ],
        );
        let eq11_e185_q: f64 = scratch.values[98];
        stamper.stamp_current_reactive(
            Some(self.nodes[3]),
            None,
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (scratch.node_derivatives[98][0])),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (scratch.node_derivatives[98][1])),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (scratch.node_derivatives[98][2])),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (scratch.node_derivatives[98][3])),
                GeneratedDerivative::node(self.nodes[4], self.multiplicity * (scratch.node_derivatives[98][4])),
                GeneratedDerivative::node(self.nodes[5], self.multiplicity * (scratch.node_derivatives[98][5])),
                GeneratedDerivative::branch(self.branches[0], self.multiplicity * (scratch.branch_derivatives[98][0])),
                GeneratedDerivative::branch(self.branches[1], self.multiplicity * (scratch.branch_derivatives[98][1])),
            ],
        );
    }
}
