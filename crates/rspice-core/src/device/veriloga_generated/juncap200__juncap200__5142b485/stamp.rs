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

        let eq0_e59: f64 = (self.params.type_ * scratch.values[0]);
        let eq0_e61: f64 = (eq0_e59 * scratch.values[633]);
        let eq0_e61_d_n0: f64 = (eq0_e59 * scratch.node_derivatives[633][0]);
        let eq0_e61_d_n1: f64 = (eq0_e59 * scratch.node_derivatives[633][1]);
        let eq0_value: f64 = eq0_e61;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[1]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq0_e61_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq0_e61_d_n1),
            ],
        );
        let eq1_e64: f64 = (self.params.type_ * scratch.values[0]);
        let eq1_e66: f64 = (eq1_e64 * scratch.values[634]);
        let eq1_e66_d_n0: f64 = (eq1_e64 * scratch.node_derivatives[634][0]);
        let eq1_e66_d_n1: f64 = (eq1_e64 * scratch.node_derivatives[634][1]);
        let eq1_e67: f64 = self.eval_ddt(0, eq1_e66);
        let eq1_e67_d_n0: f64 = self.ddt_jacobian(eq1_e66_d_n0);
        let eq1_e67_d_n1: f64 = self.ddt_jacobian(eq1_e66_d_n1);
        let eq1_value: f64 = eq1_e67;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[1]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq1_e67_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq1_e67_d_n1),
            ],
        );
        let eq2_value: f64 = 0.0;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[1]),
            self.multiplicity * (eq2_value),
            &[
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let mut scratch = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_1(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_2(ctx, stamper, &mut scratch);

        let eq1_e64: f64 = (self.params.type_ * scratch.values[0]);
        let eq1_e66: f64 = (eq1_e64 * scratch.values[634]);
        let eq1_e66_d_n0: f64 = (eq1_e64 * scratch.node_derivatives[634][0]);
        let eq1_e66_d_n1: f64 = (eq1_e64 * scratch.node_derivatives[634][1]);
        let eq1_e67_q: f64 = eq1_e66;
        stamper.stamp_current_reactive(
            Some(self.nodes[0]),
            Some(self.nodes[1]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq1_e66_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq1_e66_d_n1)),
            ],
        );
    }
}
