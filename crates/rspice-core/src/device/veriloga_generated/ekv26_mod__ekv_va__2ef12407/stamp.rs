#![allow(dead_code, unused_assignments, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[path = "stamp_blocks_0.rs"]
mod stamp_blocks_0;

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

        let eq0_e87: f64 = (self.params.type_ * scratch.values[44]);
        let eq0_e87_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[44][0]);
        let eq0_e87_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[44][1]);
        let eq0_e87_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[44][2]);
        let eq0_e87_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[44][3]);
        let eq0_e89: f64 = (eq0_e87 * scratch.values[150]);
        let eq0_e89_d_n0: f64 = ((eq0_e87_d_n0 * scratch.values[150]) + (eq0_e87 * scratch.node_derivatives[150][0]));
        let eq0_e89_d_n1: f64 = ((eq0_e87_d_n1 * scratch.values[150]) + (eq0_e87 * scratch.node_derivatives[150][1]));
        let eq0_e89_d_n2: f64 = ((eq0_e87_d_n2 * scratch.values[150]) + (eq0_e87 * scratch.node_derivatives[150][2]));
        let eq0_e89_d_n3: f64 = ((eq0_e87_d_n3 * scratch.values[150]) + (eq0_e87 * scratch.node_derivatives[150][3]));
        let eq0_value: f64 = eq0_e89;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[2]),
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq0_e89_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq0_e89_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq0_e89_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq0_e89_d_n3),
            ],
        );
        let (eq1_e95, eq1_e95_d_n0, eq1_e95_d_n1, eq1_e95_d_n2, eq1_e95_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (scratch.values[258] != 0.0) {
        let eq1_e93: f64 = (self.params.type_ * scratch.values[200]);
        let eq1_e93_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[200][0]);
        let eq1_e93_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[200][1]);
        let eq1_e93_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[200][2]);
        let eq1_e93_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[200][3]);
        (eq1_e93, eq1_e93_d_n0, eq1_e93_d_n1, eq1_e93_d_n2, eq1_e93_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e95;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            self.multiplicity * (eq1_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq1_e95_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq1_e95_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq1_e95_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq1_e95_d_n3),
            ],
        );
        let (eq2_e101, eq2_e101_d_n0, eq2_e101_d_n1, eq2_e101_d_n2, eq2_e101_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (scratch.values[258] != 0.0) {
        let eq2_e99: f64 = (self.params.type_ * scratch.values[201]);
        let eq2_e99_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[201][0]);
        let eq2_e99_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[201][1]);
        let eq2_e99_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[201][2]);
        let eq2_e99_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[201][3]);
        (eq2_e99, eq2_e99_d_n0, eq2_e99_d_n1, eq2_e99_d_n2, eq2_e99_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e101;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            self.multiplicity * (eq2_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq2_e101_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq2_e101_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq2_e101_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq2_e101_d_n3),
            ],
        );
        let (eq3_e107, eq3_e107_d_n0, eq3_e107_d_n1, eq3_e107_d_n2, eq3_e107_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (scratch.values[258] != 0.0) {
        let eq3_e105: f64 = (self.params.type_ * scratch.values[23]);
        let eq3_e105_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[23][0]);
        let eq3_e105_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[23][1]);
        let eq3_e105_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[23][2]);
        let eq3_e105_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[23][3]);
        (eq3_e105, eq3_e105_d_n0, eq3_e105_d_n1, eq3_e105_d_n2, eq3_e105_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e107;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            self.multiplicity * (eq3_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq3_e107_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq3_e107_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq3_e107_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq3_e107_d_n3),
            ],
        );
        let (eq4_e114, eq4_e114_d_n0, eq4_e114_d_n1, eq4_e114_d_n2, eq4_e114_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[258] != 0.0)) {
        let eq4_e112: f64 = (self.params.type_ * scratch.values[200]);
        let eq4_e112_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[200][0]);
        let eq4_e112_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[200][1]);
        let eq4_e112_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[200][2]);
        let eq4_e112_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[200][3]);
        (eq4_e112, eq4_e112_d_n0, eq4_e112_d_n1, eq4_e112_d_n2, eq4_e112_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e114;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            self.multiplicity * (eq4_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq4_e114_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq4_e114_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq4_e114_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq4_e114_d_n3),
            ],
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[258] != 0.0)) {
        let eq5_e119: f64 = (self.params.type_ * scratch.values[201]);
        let eq5_e119_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[201][0]);
        let eq5_e119_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[201][1]);
        let eq5_e119_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[201][2]);
        let eq5_e119_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[201][3]);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            self.multiplicity * (eq5_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq5_e121_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq5_e121_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq5_e121_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq5_e121_d_n3),
            ],
        );
        let (eq6_e128, eq6_e128_d_n0, eq6_e128_d_n1, eq6_e128_d_n2, eq6_e128_d_n3,): (f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[258] != 0.0)) {
        let eq6_e126: f64 = (self.params.type_ * scratch.values[23]);
        let eq6_e126_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[23][0]);
        let eq6_e126_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[23][1]);
        let eq6_e126_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[23][2]);
        let eq6_e126_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[23][3]);
        (eq6_e126, eq6_e126_d_n0, eq6_e126_d_n1, eq6_e126_d_n2, eq6_e126_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e128;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            self.multiplicity * (eq6_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq6_e128_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq6_e128_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq6_e128_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq6_e128_d_n3),
            ],
        );
        let eq7_e131: f64 = self.eval_ddt(2, scratch.values[106]);
        let eq7_e131_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[106][0]);
        let eq7_e131_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[106][1]);
        let eq7_e131_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[106][2]);
        let eq7_e131_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[106][3]);
        let eq7_e132: f64 = (self.params.type_ * eq7_e131);
        let eq7_e132_d_n0: f64 = (self.params.type_ * eq7_e131_d_n0);
        let eq7_e132_d_n1: f64 = (self.params.type_ * eq7_e131_d_n1);
        let eq7_e132_d_n2: f64 = (self.params.type_ * eq7_e131_d_n2);
        let eq7_e132_d_n3: f64 = (self.params.type_ * eq7_e131_d_n3);
        let eq7_value: f64 = eq7_e132;
        stamper.stamp_current(
            Some(self.nodes[1]),
            Some(self.nodes[3]),
            self.multiplicity * (eq7_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq7_e132_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq7_e132_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq7_e132_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq7_e132_d_n3),
            ],
        );
        let (eq8_e143,): (f64,) = {
    if (self.params.noise != 0.0) {
        let eq8_e141: f64 = 0.0;
        (eq8_e141,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e143;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[2]),
            self.multiplicity * (eq8_value),
            &[
            ],
        );
        let eq9_e147: f64 = (scratch.values[223]).exp();
        let eq9_e147_d_n0: f64 = (eq9_e147 * scratch.node_derivatives[223][0]);
        let eq9_e147_d_n1: f64 = (eq9_e147 * scratch.node_derivatives[223][1]);
        let eq9_e147_d_n2: f64 = (eq9_e147 * scratch.node_derivatives[223][2]);
        let eq9_e147_d_n3: f64 = (eq9_e147 * scratch.node_derivatives[223][3]);
        let eq9_e148: f64 = (1.0 - eq9_e147);
        let eq9_e148_d_n0: f64 = (-eq9_e147_d_n0);
        let eq9_e148_d_n1: f64 = (-eq9_e147_d_n1);
        let eq9_e148_d_n2: f64 = (-eq9_e147_d_n2);
        let eq9_e148_d_n3: f64 = (-eq9_e147_d_n3);
        let eq9_e149: f64 = (scratch.values[222] * eq9_e148);
        let eq9_e149_d_n0: f64 = ((scratch.node_derivatives[222][0] * eq9_e148) + (scratch.values[222] * eq9_e148_d_n0));
        let eq9_e149_d_n1: f64 = ((scratch.node_derivatives[222][1] * eq9_e148) + (scratch.values[222] * eq9_e148_d_n1));
        let eq9_e149_d_n2: f64 = ((scratch.node_derivatives[222][2] * eq9_e148) + (scratch.values[222] * eq9_e148_d_n2));
        let eq9_e149_d_n3: f64 = ((scratch.node_derivatives[222][3] * eq9_e148) + (scratch.values[222] * eq9_e148_d_n3));
        let eq9_e151: f64 = (eq9_e149 * scratch.values[226]);
        let eq9_e151_d_n0: f64 = ((eq9_e149_d_n0 * scratch.values[226]) + (eq9_e149 * scratch.node_derivatives[226][0]));
        let eq9_e151_d_n1: f64 = ((eq9_e149_d_n1 * scratch.values[226]) + (eq9_e149 * scratch.node_derivatives[226][1]));
        let eq9_e151_d_n2: f64 = ((eq9_e149_d_n2 * scratch.values[226]) + (eq9_e149 * scratch.node_derivatives[226][2]));
        let eq9_e151_d_n3: f64 = ((eq9_e149_d_n3 * scratch.values[226]) + (eq9_e149 * scratch.node_derivatives[226][3]));
        let eq9_e154: f64 = (scratch.values[206] * self.params.xd_gmin);
        let eq9_e154_d_n0: f64 = (scratch.node_derivatives[206][0] * self.params.xd_gmin);
        let eq9_e154_d_n1: f64 = (scratch.node_derivatives[206][1] * self.params.xd_gmin);
        let eq9_e154_d_n2: f64 = (scratch.node_derivatives[206][2] * self.params.xd_gmin);
        let eq9_e154_d_n3: f64 = (scratch.node_derivatives[206][3] * self.params.xd_gmin);
        let eq9_e155: f64 = (eq9_e151 + eq9_e154);
        let eq9_e155_d_n0: f64 = (eq9_e151_d_n0 + eq9_e154_d_n0);
        let eq9_e155_d_n1: f64 = (eq9_e151_d_n1 + eq9_e154_d_n1);
        let eq9_e155_d_n2: f64 = (eq9_e151_d_n2 + eq9_e154_d_n2);
        let eq9_e155_d_n3: f64 = (eq9_e151_d_n3 + eq9_e154_d_n3);
        let eq9_e157: f64 = (eq9_e155 + scratch.values[228]);
        let eq9_e157_d_n0: f64 = (eq9_e155_d_n0 + scratch.node_derivatives[228][0]);
        let eq9_e157_d_n1: f64 = (eq9_e155_d_n1 + scratch.node_derivatives[228][1]);
        let eq9_e157_d_n2: f64 = (eq9_e155_d_n2 + scratch.node_derivatives[228][2]);
        let eq9_e157_d_n3: f64 = (eq9_e155_d_n3 + scratch.node_derivatives[228][3]);
        let eq9_e159: f64 = (eq9_e157 * self.params.type_);
        let eq9_e159_d_n0: f64 = (eq9_e157_d_n0 * self.params.type_);
        let eq9_e159_d_n1: f64 = (eq9_e157_d_n1 * self.params.type_);
        let eq9_e159_d_n2: f64 = (eq9_e157_d_n2 * self.params.type_);
        let eq9_e159_d_n3: f64 = (eq9_e157_d_n3 * self.params.type_);
        let eq9_e161: f64 = (eq9_e159 * self.params.m);
        let eq9_e161_d_n0: f64 = (eq9_e159_d_n0 * self.params.m);
        let eq9_e161_d_n1: f64 = (eq9_e159_d_n1 * self.params.m);
        let eq9_e161_d_n2: f64 = (eq9_e159_d_n2 * self.params.m);
        let eq9_e161_d_n3: f64 = (eq9_e159_d_n3 * self.params.m);
        let eq9_value: f64 = eq9_e161;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            self.multiplicity * (eq9_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq9_e161_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq9_e161_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq9_e161_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq9_e161_d_n3),
            ],
        );
        let eq10_e165: f64 = (scratch.values[225]).exp();
        let eq10_e165_d_n0: f64 = (eq10_e165 * scratch.node_derivatives[225][0]);
        let eq10_e165_d_n1: f64 = (eq10_e165 * scratch.node_derivatives[225][1]);
        let eq10_e165_d_n2: f64 = (eq10_e165 * scratch.node_derivatives[225][2]);
        let eq10_e165_d_n3: f64 = (eq10_e165 * scratch.node_derivatives[225][3]);
        let eq10_e166: f64 = (1.0 - eq10_e165);
        let eq10_e166_d_n0: f64 = (-eq10_e165_d_n0);
        let eq10_e166_d_n1: f64 = (-eq10_e165_d_n1);
        let eq10_e166_d_n2: f64 = (-eq10_e165_d_n2);
        let eq10_e166_d_n3: f64 = (-eq10_e165_d_n3);
        let eq10_e167: f64 = (scratch.values[224] * eq10_e166);
        let eq10_e167_d_n0: f64 = ((scratch.node_derivatives[224][0] * eq10_e166) + (scratch.values[224] * eq10_e166_d_n0));
        let eq10_e167_d_n1: f64 = ((scratch.node_derivatives[224][1] * eq10_e166) + (scratch.values[224] * eq10_e166_d_n1));
        let eq10_e167_d_n2: f64 = ((scratch.node_derivatives[224][2] * eq10_e166) + (scratch.values[224] * eq10_e166_d_n2));
        let eq10_e167_d_n3: f64 = ((scratch.node_derivatives[224][3] * eq10_e166) + (scratch.values[224] * eq10_e166_d_n3));
        let eq10_e169: f64 = (eq10_e167 * scratch.values[227]);
        let eq10_e169_d_n0: f64 = ((eq10_e167_d_n0 * scratch.values[227]) + (eq10_e167 * scratch.node_derivatives[227][0]));
        let eq10_e169_d_n1: f64 = ((eq10_e167_d_n1 * scratch.values[227]) + (eq10_e167 * scratch.node_derivatives[227][1]));
        let eq10_e169_d_n2: f64 = ((eq10_e167_d_n2 * scratch.values[227]) + (eq10_e167 * scratch.node_derivatives[227][2]));
        let eq10_e169_d_n3: f64 = ((eq10_e167_d_n3 * scratch.values[227]) + (eq10_e167 * scratch.node_derivatives[227][3]));
        let eq10_e172: f64 = (scratch.values[207] * self.params.xd_gmin);
        let eq10_e172_d_n0: f64 = (scratch.node_derivatives[207][0] * self.params.xd_gmin);
        let eq10_e172_d_n1: f64 = (scratch.node_derivatives[207][1] * self.params.xd_gmin);
        let eq10_e172_d_n2: f64 = (scratch.node_derivatives[207][2] * self.params.xd_gmin);
        let eq10_e172_d_n3: f64 = (scratch.node_derivatives[207][3] * self.params.xd_gmin);
        let eq10_e173: f64 = (eq10_e169 + eq10_e172);
        let eq10_e173_d_n0: f64 = (eq10_e169_d_n0 + eq10_e172_d_n0);
        let eq10_e173_d_n1: f64 = (eq10_e169_d_n1 + eq10_e172_d_n1);
        let eq10_e173_d_n2: f64 = (eq10_e169_d_n2 + eq10_e172_d_n2);
        let eq10_e173_d_n3: f64 = (eq10_e169_d_n3 + eq10_e172_d_n3);
        let eq10_e175: f64 = (eq10_e173 + scratch.values[229]);
        let eq10_e175_d_n0: f64 = (eq10_e173_d_n0 + scratch.node_derivatives[229][0]);
        let eq10_e175_d_n1: f64 = (eq10_e173_d_n1 + scratch.node_derivatives[229][1]);
        let eq10_e175_d_n2: f64 = (eq10_e173_d_n2 + scratch.node_derivatives[229][2]);
        let eq10_e175_d_n3: f64 = (eq10_e173_d_n3 + scratch.node_derivatives[229][3]);
        let eq10_e177: f64 = (eq10_e175 * self.params.type_);
        let eq10_e177_d_n0: f64 = (eq10_e175_d_n0 * self.params.type_);
        let eq10_e177_d_n1: f64 = (eq10_e175_d_n1 * self.params.type_);
        let eq10_e177_d_n2: f64 = (eq10_e175_d_n2 * self.params.type_);
        let eq10_e177_d_n3: f64 = (eq10_e175_d_n3 * self.params.type_);
        let eq10_e179: f64 = (eq10_e177 * self.params.m);
        let eq10_e179_d_n0: f64 = (eq10_e177_d_n0 * self.params.m);
        let eq10_e179_d_n1: f64 = (eq10_e177_d_n1 * self.params.m);
        let eq10_e179_d_n2: f64 = (eq10_e177_d_n2 * self.params.m);
        let eq10_e179_d_n3: f64 = (eq10_e177_d_n3 * self.params.m);
        let eq10_value: f64 = eq10_e179;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq10_e179_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq10_e179_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq10_e179_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq10_e179_d_n3),
            ],
        );
        let eq11_e181: f64 = self.eval_ddt(3, scratch.values[236]);
        let eq11_e181_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[236][0]);
        let eq11_e181_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[236][1]);
        let eq11_e181_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[236][2]);
        let eq11_e181_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[236][3]);
        let eq11_e183: f64 = (eq11_e181 * self.params.type_);
        let eq11_e183_d_n0: f64 = (eq11_e181_d_n0 * self.params.type_);
        let eq11_e183_d_n1: f64 = (eq11_e181_d_n1 * self.params.type_);
        let eq11_e183_d_n2: f64 = (eq11_e181_d_n2 * self.params.type_);
        let eq11_e183_d_n3: f64 = (eq11_e181_d_n3 * self.params.type_);
        let eq11_e185: f64 = (eq11_e183 * self.params.m);
        let eq11_e185_d_n0: f64 = (eq11_e183_d_n0 * self.params.m);
        let eq11_e185_d_n1: f64 = (eq11_e183_d_n1 * self.params.m);
        let eq11_e185_d_n2: f64 = (eq11_e183_d_n2 * self.params.m);
        let eq11_e185_d_n3: f64 = (eq11_e183_d_n3 * self.params.m);
        let eq11_value: f64 = eq11_e185;
        stamper.stamp_current(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            self.multiplicity * (eq11_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq11_e185_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq11_e185_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq11_e185_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq11_e185_d_n3),
            ],
        );
        let eq12_e187: f64 = self.eval_ddt(4, scratch.values[237]);
        let eq12_e187_d_n0: f64 = self.ddt_jacobian(scratch.node_derivatives[237][0]);
        let eq12_e187_d_n1: f64 = self.ddt_jacobian(scratch.node_derivatives[237][1]);
        let eq12_e187_d_n2: f64 = self.ddt_jacobian(scratch.node_derivatives[237][2]);
        let eq12_e187_d_n3: f64 = self.ddt_jacobian(scratch.node_derivatives[237][3]);
        let eq12_e189: f64 = (eq12_e187 * self.params.type_);
        let eq12_e189_d_n0: f64 = (eq12_e187_d_n0 * self.params.type_);
        let eq12_e189_d_n1: f64 = (eq12_e187_d_n1 * self.params.type_);
        let eq12_e189_d_n2: f64 = (eq12_e187_d_n2 * self.params.type_);
        let eq12_e189_d_n3: f64 = (eq12_e187_d_n3 * self.params.type_);
        let eq12_e191: f64 = (eq12_e189 * self.params.m);
        let eq12_e191_d_n0: f64 = (eq12_e189_d_n0 * self.params.m);
        let eq12_e191_d_n1: f64 = (eq12_e189_d_n1 * self.params.m);
        let eq12_e191_d_n2: f64 = (eq12_e189_d_n2 * self.params.m);
        let eq12_e191_d_n3: f64 = (eq12_e189_d_n3 * self.params.m);
        let eq12_value: f64 = eq12_e191;
        stamper.stamp_current(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            self.multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * eq12_e191_d_n0),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * eq12_e191_d_n1),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * eq12_e191_d_n2),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * eq12_e191_d_n3),
            ],
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let mut scratch = ReactiveScratch::new();

        self.stamp_reactive_block_0(ctx, stamper, &mut scratch);
        self.stamp_reactive_block_1(ctx, stamper, &mut scratch);

        let (eq1_e95, eq1_e95_d_n0, eq1_e95_d_n1, eq1_e95_d_n2, eq1_e95_d_n3, eq1_e95_q, eq1_e95_q_d_n0, eq1_e95_q_d_n1, eq1_e95_q_d_n2, eq1_e95_q_d_n3,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[258] != 0.0) {
        let eq1_e92_q: f64 = scratch.reactive_values[200];
        let eq1_e93: f64 = (self.params.type_ * scratch.values[200]);
        let eq1_e93_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[200][0]);
        let eq1_e93_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[200][1]);
        let eq1_e93_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[200][2]);
        let eq1_e93_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[200][3]);
        let eq1_e93_q: f64 = (self.params.type_ * eq1_e92_q);
        let eq1_e93_q_d_n0: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][0]);
        let eq1_e93_q_d_n1: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][1]);
        let eq1_e93_q_d_n2: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][2]);
        let eq1_e93_q_d_n3: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][3]);
        (eq1_e93, eq1_e93_d_n0, eq1_e93_d_n1, eq1_e93_d_n2, eq1_e93_d_n3, eq1_e93_q, eq1_e93_q_d_n0, eq1_e93_q_d_n1, eq1_e93_q_d_n2, eq1_e93_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq1_e95_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq1_e95_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq1_e95_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq1_e95_q_d_n3)),
            ],
        );
        let (eq2_e101, eq2_e101_d_n0, eq2_e101_d_n1, eq2_e101_d_n2, eq2_e101_d_n3, eq2_e101_q, eq2_e101_q_d_n0, eq2_e101_q_d_n1, eq2_e101_q_d_n2, eq2_e101_q_d_n3,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (scratch.values[258] != 0.0) {
        let eq2_e98_q: f64 = scratch.reactive_values[201];
        let eq2_e99: f64 = (self.params.type_ * scratch.values[201]);
        let eq2_e99_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[201][0]);
        let eq2_e99_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[201][1]);
        let eq2_e99_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[201][2]);
        let eq2_e99_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[201][3]);
        let eq2_e99_q: f64 = (self.params.type_ * eq2_e98_q);
        let eq2_e99_q_d_n0: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][0]);
        let eq2_e99_q_d_n1: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][1]);
        let eq2_e99_q_d_n2: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][2]);
        let eq2_e99_q_d_n3: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][3]);
        (eq2_e99, eq2_e99_d_n0, eq2_e99_d_n1, eq2_e99_d_n2, eq2_e99_d_n3, eq2_e99_q, eq2_e99_q_d_n0, eq2_e99_q_d_n1, eq2_e99_q_d_n2, eq2_e99_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq2_e101_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq2_e101_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq2_e101_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq2_e101_q_d_n3)),
            ],
        );
        let (eq4_e114, eq4_e114_d_n0, eq4_e114_d_n1, eq4_e114_d_n2, eq4_e114_d_n3, eq4_e114_q, eq4_e114_q_d_n0, eq4_e114_q_d_n1, eq4_e114_q_d_n2, eq4_e114_q_d_n3,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[258] != 0.0)) {
        let eq4_e111_q: f64 = scratch.reactive_values[200];
        let eq4_e112: f64 = (self.params.type_ * scratch.values[200]);
        let eq4_e112_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[200][0]);
        let eq4_e112_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[200][1]);
        let eq4_e112_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[200][2]);
        let eq4_e112_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[200][3]);
        let eq4_e112_q: f64 = (self.params.type_ * eq4_e111_q);
        let eq4_e112_q_d_n0: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][0]);
        let eq4_e112_q_d_n1: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][1]);
        let eq4_e112_q_d_n2: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][2]);
        let eq4_e112_q_d_n3: f64 = (self.params.type_ * scratch.reactive_node_derivatives[200][3]);
        (eq4_e112, eq4_e112_d_n0, eq4_e112_d_n1, eq4_e112_d_n2, eq4_e112_d_n3, eq4_e112_q, eq4_e112_q_d_n0, eq4_e112_q_d_n1, eq4_e112_q_d_n2, eq4_e112_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq4_e114_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq4_e114_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq4_e114_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq4_e114_q_d_n3)),
            ],
        );
        let (eq5_e121, eq5_e121_d_n0, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n3, eq5_e121_q, eq5_e121_q_d_n0, eq5_e121_q_d_n1, eq5_e121_q_d_n2, eq5_e121_q_d_n3,): (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,) = {
    if (!(scratch.values[258] != 0.0)) {
        let eq5_e118_q: f64 = scratch.reactive_values[201];
        let eq5_e119: f64 = (self.params.type_ * scratch.values[201]);
        let eq5_e119_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[201][0]);
        let eq5_e119_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[201][1]);
        let eq5_e119_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[201][2]);
        let eq5_e119_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[201][3]);
        let eq5_e119_q: f64 = (self.params.type_ * eq5_e118_q);
        let eq5_e119_q_d_n0: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][0]);
        let eq5_e119_q_d_n1: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][1]);
        let eq5_e119_q_d_n2: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][2]);
        let eq5_e119_q_d_n3: f64 = (self.params.type_ * scratch.reactive_node_derivatives[201][3]);
        (eq5_e119, eq5_e119_d_n0, eq5_e119_d_n1, eq5_e119_d_n2, eq5_e119_d_n3, eq5_e119_q, eq5_e119_q_d_n0, eq5_e119_q_d_n1, eq5_e119_q_d_n2, eq5_e119_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq5_e121_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq5_e121_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq5_e121_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq5_e121_q_d_n3)),
            ],
        );
        let eq7_e131_q: f64 = scratch.values[106];
        let eq7_e132: f64 = (self.params.type_ * scratch.values[106]);
        let eq7_e132_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[106][0]);
        let eq7_e132_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[106][1]);
        let eq7_e132_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[106][2]);
        let eq7_e132_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[106][3]);
        let eq7_e132_q: f64 = (self.params.type_ * eq7_e131_q);
        let eq7_e132_q_d_n0: f64 = (self.params.type_ * scratch.node_derivatives[106][0]);
        let eq7_e132_q_d_n1: f64 = (self.params.type_ * scratch.node_derivatives[106][1]);
        let eq7_e132_q_d_n2: f64 = (self.params.type_ * scratch.node_derivatives[106][2]);
        let eq7_e132_q_d_n3: f64 = (self.params.type_ * scratch.node_derivatives[106][3]);
        stamper.stamp_current_reactive(
            Some(self.nodes[1]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq7_e132_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq7_e132_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq7_e132_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq7_e132_q_d_n3)),
            ],
        );
        let eq11_e181_q: f64 = scratch.values[236];
        let eq11_e183: f64 = (scratch.values[236] * self.params.type_);
        let eq11_e183_d_n0: f64 = (scratch.node_derivatives[236][0] * self.params.type_);
        let eq11_e183_d_n1: f64 = (scratch.node_derivatives[236][1] * self.params.type_);
        let eq11_e183_d_n2: f64 = (scratch.node_derivatives[236][2] * self.params.type_);
        let eq11_e183_d_n3: f64 = (scratch.node_derivatives[236][3] * self.params.type_);
        let eq11_e183_q: f64 = (eq11_e181_q * self.params.type_);
        let eq11_e183_q_d_n0: f64 = (scratch.node_derivatives[236][0] * self.params.type_);
        let eq11_e183_q_d_n1: f64 = (scratch.node_derivatives[236][1] * self.params.type_);
        let eq11_e183_q_d_n2: f64 = (scratch.node_derivatives[236][2] * self.params.type_);
        let eq11_e183_q_d_n3: f64 = (scratch.node_derivatives[236][3] * self.params.type_);
        let eq11_e185: f64 = (eq11_e183 * self.params.m);
        let eq11_e185_d_n0: f64 = (eq11_e183_d_n0 * self.params.m);
        let eq11_e185_d_n1: f64 = (eq11_e183_d_n1 * self.params.m);
        let eq11_e185_d_n2: f64 = (eq11_e183_d_n2 * self.params.m);
        let eq11_e185_d_n3: f64 = (eq11_e183_d_n3 * self.params.m);
        let eq11_e185_q: f64 = (eq11_e183_q * self.params.m);
        let eq11_e185_q_d_n0: f64 = (eq11_e183_q_d_n0 * self.params.m);
        let eq11_e185_q_d_n1: f64 = (eq11_e183_q_d_n1 * self.params.m);
        let eq11_e185_q_d_n2: f64 = (eq11_e183_q_d_n2 * self.params.m);
        let eq11_e185_q_d_n3: f64 = (eq11_e183_q_d_n3 * self.params.m);
        stamper.stamp_current_reactive(
            Some(self.nodes[0]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq11_e185_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq11_e185_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq11_e185_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq11_e185_q_d_n3)),
            ],
        );
        let eq12_e187_q: f64 = scratch.values[237];
        let eq12_e189: f64 = (scratch.values[237] * self.params.type_);
        let eq12_e189_d_n0: f64 = (scratch.node_derivatives[237][0] * self.params.type_);
        let eq12_e189_d_n1: f64 = (scratch.node_derivatives[237][1] * self.params.type_);
        let eq12_e189_d_n2: f64 = (scratch.node_derivatives[237][2] * self.params.type_);
        let eq12_e189_d_n3: f64 = (scratch.node_derivatives[237][3] * self.params.type_);
        let eq12_e189_q: f64 = (eq12_e187_q * self.params.type_);
        let eq12_e189_q_d_n0: f64 = (scratch.node_derivatives[237][0] * self.params.type_);
        let eq12_e189_q_d_n1: f64 = (scratch.node_derivatives[237][1] * self.params.type_);
        let eq12_e189_q_d_n2: f64 = (scratch.node_derivatives[237][2] * self.params.type_);
        let eq12_e189_q_d_n3: f64 = (scratch.node_derivatives[237][3] * self.params.type_);
        let eq12_e191: f64 = (eq12_e189 * self.params.m);
        let eq12_e191_d_n0: f64 = (eq12_e189_d_n0 * self.params.m);
        let eq12_e191_d_n1: f64 = (eq12_e189_d_n1 * self.params.m);
        let eq12_e191_d_n2: f64 = (eq12_e189_d_n2 * self.params.m);
        let eq12_e191_d_n3: f64 = (eq12_e189_d_n3 * self.params.m);
        let eq12_e191_q: f64 = (eq12_e189_q * self.params.m);
        let eq12_e191_q_d_n0: f64 = (eq12_e189_q_d_n0 * self.params.m);
        let eq12_e191_q_d_n1: f64 = (eq12_e189_q_d_n1 * self.params.m);
        let eq12_e191_q_d_n2: f64 = (eq12_e189_q_d_n2 * self.params.m);
        let eq12_e191_q_d_n3: f64 = (eq12_e189_q_d_n3 * self.params.m);
        stamper.stamp_current_reactive(
            Some(self.nodes[2]),
            Some(self.nodes[3]),
            &[
                GeneratedDerivative::node(self.nodes[0], self.multiplicity * (eq12_e191_q_d_n0)),
                GeneratedDerivative::node(self.nodes[1], self.multiplicity * (eq12_e191_q_d_n1)),
                GeneratedDerivative::node(self.nodes[2], self.multiplicity * (eq12_e191_q_d_n2)),
                GeneratedDerivative::node(self.nodes[3], self.multiplicity * (eq12_e191_q_d_n3)),
            ],
        );
    }
}
