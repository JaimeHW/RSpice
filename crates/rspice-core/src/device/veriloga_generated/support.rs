#![allow(dead_code)]

use super::GeneratedEvalContext;

const LIMEXP_MAX: f64 = 5.54062238439351e34;

pub(crate) struct Scratch<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> {
    pub(crate) v: [f64; VARIABLE_COUNT],
    pub(crate) b: [bool; VARIABLE_COUNT],
    pub(crate) dn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) db: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
}

impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> Scratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT> {
    pub(crate) fn new() -> Self {
        *Self::new_box()
    }

    pub(crate) fn new_box() -> Box<Self> {
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    pub(crate) fn new_value() -> Self {
        Self {
            v: [0.0; VARIABLE_COUNT],
            b: [false; VARIABLE_COUNT],
            dn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            db: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
        }
    }

    #[inline]
    pub(crate) fn ad_value(&self, index: usize) -> AdValue<NODE_COUNT, BRANCH_COUNT> {
        AdValue { value: self.v[index], dn: self.dn[index], db: self.db[index] }
    }

    #[inline]
    pub(crate) fn store_ad(&mut self, index: usize, value: &AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn copy_ad(&mut self, target: usize, source: usize) {
        self.v[target] = self.v[source];
        self.dn[target] = self.dn[source];
        self.db[target] = self.db[source];
    }

    #[inline]
    pub(crate) fn store_scalar(&mut self, index: usize, value: f64) {
        self.v[index] = value;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
    }

    #[inline]
    pub(crate) fn store_ad_value(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn store_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        self.store_scaled_voltage(index, ctx, nodes, pos, neg, 1.0);
    }

    #[inline]
    pub(crate) fn store_scaled_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, scale: f64) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        self.v[index] = (pos_value - neg_value) * scale;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += scale; }
        if let Some(node) = neg { self.dn[index][node] -= scale; }
    }

    #[inline]
    pub(crate) fn store_offset_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, offset: f64) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        self.v[index] = pos_value - neg_value + offset;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += 1.0; }
        if let Some(node) = neg { self.dn[index][node] -= 1.0; }
    }

    #[inline]
    pub(crate) fn store_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let voltage = pos_value - neg_value;
        let derivative_scale = if voltage >= 0.0 { 1.0 } else { -1.0 };
        self.v[index] = voltage.abs();
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += derivative_scale; }
        if let Some(node) = neg { self.dn[index][node] -= derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sub_voltage_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], left_pos: Option<usize>, left_neg: Option<usize>, abs_pos: Option<usize>, abs_neg: Option<usize>) {
        let left_pos_value = left_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let left_neg_value = left_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_pos_value = abs_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_neg_value = abs_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_voltage = abs_pos_value - abs_neg_value;
        let abs_derivative_scale = if abs_voltage >= 0.0 { 1.0 } else { -1.0 };
        self.v[index] = left_pos_value - left_neg_value - abs_voltage.abs();
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = left_pos { self.dn[index][node] += 1.0; }
        if let Some(node) = left_neg { self.dn[index][node] -= 1.0; }
        if let Some(node) = abs_pos { self.dn[index][node] -= abs_derivative_scale; }
        if let Some(node) = abs_neg { self.dn[index][node] += abs_derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_voltage_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let voltage = pos_value - neg_value;
        self.v[index] = value.value * voltage;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * voltage; }
        if let Some(node) = pos { self.dn[index][node] += value.value; }
        if let Some(node) = neg { self.dn[index][node] -= value.value; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * voltage; }
    }

    #[inline]
    pub(crate) fn store_div_voltage_by_ad(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let reciprocal = 1.0 / right.value;
        let quotient = (pos_value - neg_value) * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = right.dn[axis] * right_scale; }
        if let Some(node) = pos { self.dn[index][node] += reciprocal; }
        if let Some(node) = neg { self.dn[index][node] -= reciprocal; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right.value + left.value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right.value + left.value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_rem_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::rem(left, right));
    }

    #[inline]
    pub(crate) fn store_pow_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_min_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let selected = if left.value <= right.value { left } else { right };
        self.v[index] = selected.value;
        self.dn[index] = selected.dn;
        self.db[index] = selected.db;
    }

    #[inline]
    pub(crate) fn store_max_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let selected = if left.value >= right.value { left } else { right };
        self.v[index] = selected.value;
        self.dn[index] = selected.dn;
        self.db[index] = selected.db;
    }

    #[inline]
    pub(crate) fn store_hypot_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::hypot(left, right));
    }

    #[inline]
    pub(crate) fn store_atan2_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atan2(left, right));
    }

    #[inline]
    pub(crate) fn store_scale_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, value, offset);
    }

    #[inline]
    pub(crate) fn store_offset_ad_value(&mut self, index: usize, mut value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        value.value += offset;
        self.store_ad_value(index, value);
    }

    #[inline]
    pub(crate) fn store_offset_scaled_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64) {
        self.v[index] = value.value * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value + right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value - right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value * right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right.value + left.value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right.value + left.value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let output = value.value.exp();
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_offset_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let root = value.value.sqrt();
        self.v[index] = root + offset;
        let derivative_scale = 1.0 / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = value.value.ln() + offset;
        let derivative_scale = 1.0 / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        if value.value > 80.0 {
            self.v[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) + offset;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * LIMEXP_MAX; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * LIMEXP_MAX; }
        } else if value.value < -80.0 {
            self.store_scalar(index, 1.804851387e-35 + offset);
        } else {
            let output = value.value.exp();
            self.v[index] = output + offset;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        }
    }

    #[inline]
    pub(crate) fn store_offset_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, offset: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0); }
    }

    #[inline]
    pub(crate) fn store_offset_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = scalar - value.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_pow_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_offset_min_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_hypot_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::hypot(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atan2_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atan2(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let exponent = value.value;
        let output = scalar.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_offset_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_min_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::abs(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_square_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::square(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::limexp(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_log10_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::log10(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_sin_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::sin(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_cos_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::cos(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_tan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::tan(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atan(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_sinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::sinh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_cosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::cosh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_tanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::tanh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_asinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::asinh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_acosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::acosh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atanh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_floor_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::floor(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::ceil(value), offset);
    }

    #[inline]
    pub(crate) fn store_scaled_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64, scale: f64) {
        self.v[index] = (value.value + offset) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (left.value + right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (left.value - right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = left.value * right.value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right.dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right.db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let output = value.value.exp() * scale;
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_scaled_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value.ln() * scale;
        let derivative_scale = scale / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let root = value.value.sqrt();
        self.v[index] = root * scale;
        let derivative_scale = scale / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        if value.value < 80.0 {
            let output = value.value.exp() * scale;
            self.v[index] = output;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        } else {
            self.v[index] = LIMEXP_MAX * (1.0 + (value.value - 80.0)) * scale;
            let derivative_scale = LIMEXP_MAX * scale;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        if value.value > 80.0 {
            self.v[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) * scale;
            let derivative_scale = LIMEXP_MAX * scale;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
        } else if value.value < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * scale);
        } else {
            let output = value.value.exp() * scale;
            self.v[index] = output;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        }
    }

    #[inline]
    pub(crate) fn store_scaled_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value.abs() * scale;
        let derivative_scale = if value.value >= 0.0 { scale } else { -scale };
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, scale: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (scalar - value.value) * scale;
        let derivative_scale = -scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * scale;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_neg_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = -value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = value.value.sqrt();
        self.v[index] = root;
        let derivative_scale = 1.0 / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let root = (value.value + offset).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_scaled_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let root = (value.value * scale).sqrt();
        let derivative_scale = scale / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (left.value + right.value).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (left.value - right.value).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = left.value * right.value;
        let root = raw.sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / right.value;
        let raw = left.value * reciprocal;
        let root = raw.sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        let right_scale = -raw * reciprocal;
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right.dn[axis] * right_scale) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right.db[axis] * right_scale) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value.abs();
        let root = raw.sqrt();
        let derivative_scale = if value.value >= 0.0 { 1.0 / (2.0 * root) } else { -1.0 / (2.0 * root) };
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (scalar - value.value).sqrt();
        let derivative_scale = -1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = scalar / value.value;
        let root = raw.sqrt();
        let derivative_scale = -raw / (value.value * 2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let output = value.value.exp();
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value.ln();
        let derivative_scale = 1.0 / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value.abs();
        let derivative_scale = if value.value >= 0.0 { 1.0 } else { -1.0 };
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_square_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value * value.value;
        let derivative_scale = 2.0 * value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::limexp(value));
    }

    #[inline]
    pub(crate) fn store_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::limited_exp(value));
    }

    #[inline]
    pub(crate) fn store_log10_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::log10(value));
    }

    #[inline]
    pub(crate) fn store_sin_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sin(value));
    }

    #[inline]
    pub(crate) fn store_cos_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::cos(value));
    }

    #[inline]
    pub(crate) fn store_tan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::tan(value));
    }

    #[inline]
    pub(crate) fn store_atan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atan(value));
    }

    #[inline]
    pub(crate) fn store_sinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sinh(value));
    }

    #[inline]
    pub(crate) fn store_cosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::cosh(value));
    }

    #[inline]
    pub(crate) fn store_tanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::tanh(value));
    }

    #[inline]
    pub(crate) fn store_asinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::asinh(value));
    }

    #[inline]
    pub(crate) fn store_acosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::acosh(value));
    }

    #[inline]
    pub(crate) fn store_atanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atanh(value));
    }

    #[inline]
    pub(crate) fn store_floor_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::floor(value));
    }

    #[inline]
    pub(crate) fn store_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ceil(value));
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = scalar - value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let denominator = value.value + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_scaled_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let denominator = value.value * scale;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * scale;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_add_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value + right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sub_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value - right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_mul_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value * right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_div_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let right_reciprocal = 1.0 / right.value;
        let denominator = left.value * right_reciprocal;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        let right_scale = -denominator * right_reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right_reciprocal + right.dn[axis] * right_scale) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right_reciprocal + right.db[axis] * right_scale) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sqrt_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = value.value.sqrt();
        let reciprocal = 1.0 / root;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient / (2.0 * value.value);
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_square_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = value.value * value.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -2.0 * quotient / value.value;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = denominator_scalar - value.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_div_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let quotient_scale = scalar / denominator_scalar;
        self.v[index] = value.value * quotient_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * quotient_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * quotient_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_pow_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value.powf(right.value);
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, left.value, right.value, left.dn[axis], right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, left.value, right.value, left.db[axis], right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_powf_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let denominator = value.value.powf(exponent);
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, value.value, exponent, value.dn[axis], 0.0) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, value.value, exponent, value.db[axis], 0.0) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_exp_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = value.value.exp();
        let quotient = scalar / denominator;
        let derivative_scale = -quotient;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sin_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let denominator = raw.sin();
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * raw.cos();
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sinh_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let denominator = raw.sinh();
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * raw.cosh();
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::rem_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let exponent = value.value;
        let output = scalar.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::min_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::max_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::rem_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_min_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::min_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_max_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::max_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0); }
    }

    #[inline]
    pub(crate) fn store_add(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_square(&mut self, index: usize, source: usize) {
        let source_value = self.v[source];
        self.store_unary_scaled(index, source, source_value * source_value, 2.0 * source_value);
    }

    #[inline]
    pub(crate) fn store_div(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = (left_value + right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = (left.value + right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = (left_value - right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = (left.value - right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right.value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right.value + left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = left_value * middle_value;
        self.v[index] = product_value * right_value;
        for axis in 0..NODE_COUNT { let product_derivative = left_dn[axis] * middle_value + left_value * middle_dn[axis]; self.dn[index][axis] = product_derivative * right_value + product_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left_db[axis] * middle_value + left_value * middle_db[axis]; self.db[index][axis] = product_derivative * right_value + product_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = middle_value * right_value;
        self.v[index] = left_value * product_value;
        for axis in 0..NODE_COUNT { let product_derivative = middle_dn[axis] * right_value + middle_value * right_dn[axis]; self.dn[index][axis] = left_dn[axis] * product_value + left_value * product_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = middle_db[axis] * right_value + middle_value * right_db[axis]; self.db[index][axis] = left_db[axis] * product_value + left_value * product_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_product_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        self.v[index] = product_value * source_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; self.dn[index][axis] = product_derivative * source_value + product_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; self.db[index][axis] = product_derivative * source_value + product_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_product_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        self.v[index] = source_value * product_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; self.dn[index][axis] = source_dn[axis] * product_value + source_value * product_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; self.db[index][axis] = source_db[axis] * product_value + source_value * product_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul3_affine_lhs(&mut self, index: usize, left: usize, middle: usize, scale: f64, offset: f64, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = left_value * middle_value;
        let affine_value = product_value * scale + offset;
        self.v[index] = affine_value * right_value;
        for axis in 0..NODE_COUNT { let product_derivative = left_dn[axis] * middle_value + left_value * middle_dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = affine_derivative * right_value + affine_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left_db[axis] * middle_value + left_value * middle_db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = affine_derivative * right_value + affine_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_affine_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = middle_value * right_value;
        let affine_value = product_value * scale + offset;
        self.v[index] = left_value * affine_value;
        for axis in 0..NODE_COUNT { let product_derivative = middle_dn[axis] * right_value + middle_value * right_dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = left_dn[axis] * affine_value + left_value * affine_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = middle_db[axis] * right_value + middle_value * right_db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = left_db[axis] * affine_value + left_value * affine_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_affine_product_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        let affine_value = product_value * scale + offset;
        self.v[index] = affine_value * source_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = affine_derivative * source_value + affine_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = affine_derivative * source_value + affine_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_affine_product_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        let affine_value = product_value * scale + offset;
        self.v[index] = source_value * affine_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = source_dn[axis] * affine_value + source_value * affine_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = source_db[axis] * affine_value + source_value * affine_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_add_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let sum = left_value + middle_value;
        self.v[index] = sum * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + middle_dn[axis]) * right_value + sum * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + middle_db[axis]) * right_value + sum * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_add_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let sum = middle_value + right_value;
        self.v[index] = left_value * sum;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * sum + left_value * (middle_dn[axis] + right_dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * sum + left_value * (middle_db[axis] + right_db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let difference = left_value - middle_value;
        self.v[index] = difference * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - middle_dn[axis]) * right_value + difference * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - middle_db[axis]) * right_value + difference * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let difference = middle_value - right_value;
        self.v[index] = left_value * difference;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * difference + left_value * (middle_dn[axis] - right_dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * difference + left_value * (middle_db[axis] - right_db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let sum = left.value + right.value;
        self.v[index] = sum * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * source_value + sum * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * source_value + sum * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_add_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let sum = left.value + right.value;
        self.v[index] = source_value * sum;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * sum + source_value * (left.dn[axis] + right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * sum + source_value * (left.db[axis] + right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let difference = left.value - right.value;
        self.v[index] = difference * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * source_value + difference * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * source_value + difference * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let difference = left.value - right.value;
        self.v[index] = source_value * difference;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * difference + source_value * (left.dn[axis] - right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * difference + source_value * (left.db[axis] - right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {
        let left_value = scalar - self.v[value];
        let source_value = self.v[source];
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value_dn[axis] * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value_db[axis] * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {
        let source_value = self.v[source];
        let right_value = scalar - self.v[value];
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value - source_value * value_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value - source_value * value_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let left_value = scalar - value.value;
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis] * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis] * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let right_value = scalar - value.value;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value - source_value * value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value - source_value * value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {
        let source_value = self.v[source];
        let denominator = self.v[value];
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {
        let source_value = self.v[source];
        let denominator = self.v[value];
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * quotient + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * quotient + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * quotient + source_value * value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * quotient + source_value * value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_pow_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output * source_value;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); self.dn[index][axis] = derivative * source_value + output * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); self.db[index][axis] = derivative * source_value + output * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_pow_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = source_value * output;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); self.dn[index][axis] = source_dn[axis] * output + source_value * derivative; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); self.db[index][axis] = source_db[axis] * output + source_value * derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_powf_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = value.value;
        let output = base.powf(exponent);
        self.v[index] = output * source_value;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.dn[axis], 0.0); self.dn[index][axis] = derivative * source_value + output * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.db[axis], 0.0); self.db[index][axis] = derivative * source_value + output * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_powf_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = value.value;
        let output = base.powf(exponent);
        self.v[index] = source_value * output;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.dn[axis], 0.0); self.dn[index][axis] = source_dn[axis] * output + source_value * derivative; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.db[axis], 0.0); self.db[index][axis] = source_db[axis] * output + source_value * derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_lhs(&mut self, index: usize, left: usize, right: usize) {
        let left_value = -self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_rhs(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = -self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value - left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value - left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let left_value = -left.value;
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -left.dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -left.db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let right_value = -right.value;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value - left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value - left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_lhs(&mut self, index: usize, numerator: usize, denominator: usize, source: usize) {
        let numerator_value = self.v[numerator];
        let denominator_value = self.v[denominator];
        let source_value = self.v[source];
        let numerator_dn = self.dn[numerator];
        let denominator_dn = self.dn[denominator];
        let source_dn = self.dn[source];
        let numerator_db = self.db[numerator];
        let denominator_db = self.db[denominator];
        let source_db = self.db[source];
        let reciprocal = 1.0 / denominator_value;
        let quotient = numerator_value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { let quotient_derivative = numerator_dn[axis] * reciprocal + denominator_dn[axis] * denominator_scale; self.dn[index][axis] = quotient_derivative * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = numerator_db[axis] * reciprocal + denominator_db[axis] * denominator_scale; self.db[index][axis] = quotient_derivative * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_rhs(&mut self, index: usize, source: usize, numerator: usize, denominator: usize) {
        let source_value = self.v[source];
        let numerator_value = self.v[numerator];
        let denominator_value = self.v[denominator];
        let source_dn = self.dn[source];
        let numerator_dn = self.dn[numerator];
        let denominator_dn = self.dn[denominator];
        let source_db = self.db[source];
        let numerator_db = self.db[numerator];
        let denominator_db = self.db[denominator];
        let reciprocal = 1.0 / denominator_value;
        let quotient = numerator_value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { let quotient_derivative = numerator_dn[axis] * reciprocal + denominator_dn[axis] * denominator_scale; self.dn[index][axis] = source_dn[axis] * quotient + source_value * quotient_derivative; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = numerator_db[axis] * reciprocal + denominator_db[axis] * denominator_scale; self.db[index][axis] = source_db[axis] * quotient + source_value * quotient_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { let quotient_derivative = left.dn[axis] * reciprocal + right.dn[axis] * denominator_scale; self.dn[index][axis] = quotient_derivative * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = left.db[axis] * reciprocal + right.db[axis] * denominator_scale; self.db[index][axis] = quotient_derivative * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { let quotient_derivative = left.dn[axis] * reciprocal + right.dn[axis] * denominator_scale; self.dn[index][axis] = source_dn[axis] * quotient + source_value * quotient_derivative; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = left.db[axis] * reciprocal + right.db[axis] * denominator_scale; self.db[index][axis] = source_db[axis] * quotient + source_value * quotient_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_square_lhs(&mut self, index: usize, value: usize, source: usize) {
        let raw = self.v[value];
        let square = raw * raw;
        let source_value = self.v[source];
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        let derivative_scale = 2.0 * raw;
        self.v[index] = square * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + square * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + square * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_square_rhs(&mut self, index: usize, source: usize, value: usize) {
        let source_value = self.v[source];
        let raw = self.v[value];
        let square = raw * raw;
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        let derivative_scale = 2.0 * raw;
        self.v[index] = source_value * square;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * square + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * square + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_lhs(&mut self, index: usize, value_source: usize, source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let value_dn = self.dn[value_source];
        let source_dn = self.dn[source];
        let value_db = self.db[value_source];
        let source_db = self.db[source];
        self.v[index] = unary_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + unary_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + unary_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_rhs(&mut self, index: usize, source: usize, value_source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let value_dn = self.dn[value_source];
        let source_db = self.db[source];
        let value_db = self.db[value_source];
        self.v[index] = source_value * unary_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * unary_value + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * unary_value + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = unary_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale * source_value + unary_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale * source_value + unary_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * unary_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * unary_value + source_value * value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * unary_value + source_value * value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let unary_value = self.v[value_source].exp();
        self.store_mul_unary_lhs(index, value_source, source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let unary_value = self.v[value_source].exp();
        self.store_mul_unary_rhs(index, source, value_source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let unary_value = value.value.exp();
        self.store_mul_unary_ad_lhs(index, value, source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let unary_value = value.value.exp();
        self.store_mul_unary_ad_rhs(index, source, value, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_ln_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let unary_value = self.v[value_source].sqrt();
        self.store_mul_unary_lhs(index, value_source, source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let unary_value = self.v[value_source].sqrt();
        self.store_mul_unary_rhs(index, source, value_source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let unary_value = value.value.sqrt();
        self.store_mul_unary_ad_lhs(index, value, source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let unary_value = value.value.sqrt();
        self.store_mul_unary_ad_rhs(index, source, value, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_abs_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_cos_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_tanh_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        let cosh = raw.cosh();
        self.store_mul_unary_lhs(index, value_source, source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        let cosh = raw.cosh();
        self.store_mul_unary_rhs(index, source, value_source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        let cosh = raw.cosh();
        self.store_mul_unary_ad_lhs(index, value, source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let cosh = raw.cosh();
        self.store_mul_unary_ad_rhs(index, source, value, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_limexp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); } else { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); } else { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); } else { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); } else { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        if raw > 80.0 { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_lhs(index, value_source, source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        if raw > 80.0 { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_rhs(index, source, value_source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        if raw > 80.0 { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_lhs(index, value, source, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        if raw > 80.0 { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_rhs(index, source, value, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); }
    }

    #[inline]
    pub(crate) fn store_mul_offset_lhs(&mut self, index: usize, left: usize, offset: f64, right: usize) {
        let left_value = self.v[left] + offset;
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_rhs(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right] + offset;
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64, right: usize) {
        let left_value = left.value + offset;
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let left_value = self.v[left];
        let right_value = right.value + offset;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right.db[axis]; }
    }

    pub(crate) fn store_mul_scale_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, source: usize) {
        let left_value = value.value * scale;
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_scale_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let source_value = self.v[source];
        let right_value = value.value * scale;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value + source_value * value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value + source_value * value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_mul_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale * right.value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale * right.value + left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right.dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right.db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value + right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value - right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * left_scale + right_value * right_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * left_scale - right_value * right_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale - right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale - right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_add(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value + right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_sub(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value - right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_mul(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value + right_value) * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value - right_value) * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scale(&mut self, index: usize, source: usize, scale: f64) {
        self.store_unary_scaled(index, source, self.v[source] * scale, scale);
    }

    #[inline]
    pub(crate) fn store_offset(&mut self, index: usize, source: usize, offset: f64) {
        self.v[index] = self.v[source] + offset;
        self.dn[index] = self.dn[source];
        self.db[index] = self.db[source];
    }

    #[inline]
    pub(crate) fn store_offset_scaled(&mut self, index: usize, source: usize, scale: f64, offset: f64) {
        self.store_unary_scaled(index, source, self.v[source] * scale + offset, scale);
    }

    #[inline]
    pub(crate) fn store_scaled_offset(&mut self, index: usize, source: usize, offset: f64, scale: f64) {
        self.store_unary_scaled(index, source, (self.v[source] + offset) * scale, scale);
    }

    #[inline]
    pub(crate) fn store_neg(&mut self, index: usize, source: usize) {
        self.store_scale(index, source, -1.0);
    }

    #[inline]
    pub(crate) fn store_abs(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_min_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {
        if self.v[source] <= scalar {
            self.copy_ad(index, source);
        } else {
            self.store_scalar(index, scalar);
        }
    }

    #[inline]
    pub(crate) fn store_max_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {
        if self.v[source] >= scalar {
            self.copy_ad(index, source);
        } else {
            self.store_scalar(index, scalar);
        }
    }

    #[inline]
    pub(crate) fn store_min(&mut self, index: usize, left: usize, right: usize) {
        if self.v[left] <= self.v[right] {
            self.copy_ad(index, left);
        } else {
            self.copy_ad(index, right);
        }
    }

    #[inline]
    pub(crate) fn store_max(&mut self, index: usize, left: usize, right: usize) {
        if self.v[left] >= self.v[right] {
            self.copy_ad(index, left);
        } else {
            self.copy_ad(index, right);
        }
    }

    #[inline]
    pub(crate) fn store_min3(&mut self, index: usize, first: usize, second: usize, third: usize) {
        let mut selected = first;
        if self.v[second] < self.v[selected] { selected = second; }
        if self.v[third] < self.v[selected] { selected = third; }
        self.copy_ad(index, selected);
    }

    #[inline]
    pub(crate) fn store_max3(&mut self, index: usize, first: usize, second: usize, third: usize) {
        let mut selected = first;
        if self.v[second] > self.v[selected] { selected = second; }
        if self.v[third] > self.v[selected] { selected = third; }
        self.copy_ad(index, selected);
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {
        self.store_unary_scaled(index, source, scalar - self.v[source], -1.0);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {
        let reciprocal = 1.0 / self.v[source];
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_input(&mut self, index: usize, scalar: f64, source: usize, offset: f64) {
        let denominator = self.v[source] + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64) {
        let denominator = self.v[source] * scale;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64, offset: f64) {
        let denominator = self.v[source] * scale + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);
    }

    #[inline]
    pub(crate) fn store_sqrt(&mut self, index: usize, source: usize) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_scaled_input(&mut self, index: usize, source: usize, scale: f64, offset: f64) {
        let raw = self.v[source] * scale + offset;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_square_offset(&mut self, index: usize, source: usize, offset: f64) {
        let source_value = self.v[source];
        let value = (source_value * source_value + offset).sqrt();
        self.store_unary_scaled(index, source, value, source_value / value);
    }

    #[inline]
    pub(crate) fn store_sqrt_square_add(&mut self, index: usize, square_source: usize, add_source: usize) {
        let square_value = self.v[square_source];
        let value = (square_value * square_value + self.v[add_source]).sqrt();
        let square_scale = square_value / value;
        let add_scale = 1.0 / (2.0 * value);
        let square_dn = self.dn[square_source];
        let add_dn = self.dn[add_source];
        let square_db = self.db[square_source];
        let add_db = self.db[add_source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = square_dn[axis] * square_scale + add_dn[axis] * add_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = square_db[axis] * square_scale + add_db[axis] * add_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_square_sum(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let value = (left_value * left_value + right_value * right_value).sqrt();
        let left_scale = left_value / value;
        let right_scale = right_value / value;
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_exp(&mut self, index: usize, source: usize) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_limexp(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_exp(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].exp() * scale;
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_ln(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn ln_one_plus_exp_raw(raw: f64) -> (f64, f64) {
        if raw > 0.0 {
            (raw + (-raw).exp().ln_1p(), 1.0 / (1.0 + (-raw).exp()))
        } else {
            let exp = raw.exp();
            (exp.ln_1p(), exp / (1.0 + exp))
        }
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp(&mut self, index: usize, source: usize) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source]);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_ln_one_plus_exp(&mut self, index: usize, source: usize, scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source]);
        self.store_unary_scaled(index, source, value * scale, derivative_scale * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value * scale, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_scaled_square(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw * raw * scale, 2.0 * raw * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_abs(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs() * scale, if raw >= 0.0 { scale } else { -scale });
    }

    #[inline]
    pub(crate) fn store_scaled_ln(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln() * scale, scale / raw);
    }

    #[inline]
    pub(crate) fn store_scaled_limexp(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp() * scale;
            self.store_unary_scaled(index, source, value, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * scale, LIMEXP_MAX * scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * scale, LIMEXP_MAX * scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * scale);
        } else {
            let value = raw.exp() * scale;
            self.store_unary_scaled(index, source, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_sqrt_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value, value * scale);
    }

    #[inline]
    pub(crate) fn store_limexp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value * scale);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX * scale);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX * scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value * scale);
        }
    }

    #[inline]
    pub(crate) fn store_ln_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        self.store_unary_scaled(index, source, raw.ln(), scale / raw);
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source] * scale);
        self.store_unary_scaled(index, source, value, derivative_scale * scale);
    }

    #[inline]
    pub(crate) fn store_sin_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        self.store_unary_scaled(index, source, raw.sin(), raw.cos() * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value * output_scale, output_scale * input_scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_scaled_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_limexp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * output_scale, LIMEXP_MAX * output_scale * input_scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * output_scale, LIMEXP_MAX * output_scale * input_scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * output_scale);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_ln_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        self.store_unary_scaled(index, source, raw.ln() * output_scale, output_scale * input_scale / raw);
    }

    #[inline]
    pub(crate) fn store_scaled_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source] * input_scale);
        self.store_unary_scaled(index, source, value * output_scale, derivative_scale * input_scale * output_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sin_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        self.store_unary_scaled(index, source, raw.sin() * output_scale, raw.cos() * output_scale * input_scale);
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_ln_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_offset_square(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw * raw + offset, 2.0 * raw);
    }

    #[inline]
    pub(crate) fn store_offset_abs(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs() + offset, if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_offset_sqrt(&mut self, index: usize, source: usize, offset: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value + offset, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_offset_exp(&mut self, index: usize, source: usize, offset: f64) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value + offset, value);
    }

    #[inline]
    pub(crate) fn store_offset_ln(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln() + offset, 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_offset_limexp(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value + offset, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) + offset, LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_offset_limited_exp(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) + offset, LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 + offset);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value + offset, value);
        }
    }

    #[inline]
    pub(crate) fn store_sqrt_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, -1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_neg_input(&mut self, index: usize, source: usize) {
        let value = (-self.v[source]).exp();
        self.store_unary_scaled(index, source, value, -value);
    }

    #[inline]
    pub(crate) fn store_limexp_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, -value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), -LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), -LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, -value);
        }
    }

    #[inline]
    pub(crate) fn store_ln_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        self.store_unary_scaled(index, source, raw.ln(), -1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp_neg_input(&mut self, index: usize, source: usize) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(-self.v[source]);
        self.store_unary_scaled(index, source, value, -derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_add_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_sub_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_mul_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_div_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        let value = raw.sqrt();
        self.store_unary_add_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        let value = raw.sqrt();
        self.store_unary_sub_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        let value = raw.sqrt();
        self.store_unary_mul_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        let value = raw.sqrt();
        self.store_unary_div_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_add(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] + self.v[right]).exp();
        self.store_unary_add_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_sub(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] - self.v[right]).exp();
        self.store_unary_sub_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_mul(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] * self.v[right]).exp();
        self.store_unary_mul_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_div(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] / self.v[right]).exp();
        self.store_unary_div_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_ln_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        self.store_unary_add_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        self.store_unary_sub_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        self.store_unary_mul_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        self.store_unary_div_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_limexp_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_add_scaled(index, left, right, value, value);
        } else {
            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_sub_scaled(index, left, right, value, value);
        } else {
            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_mul_scaled(index, left, right, value, value);
        } else {
            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_div_scaled(index, left, right, value, value);
        } else {
            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        if raw > 80.0 {
            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_add_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        if raw > 80.0 {
            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_sub_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        if raw > 80.0 {
            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_mul_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        if raw > 80.0 {
            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_div_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_sin(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.sin(), raw.cos());
    }

    #[inline]
    pub(crate) fn store_sinh(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.sinh(), raw.cosh());
    }

    #[inline]
    pub(crate) fn store_asinh(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt());
    }

    #[inline]
    pub(crate) fn store_powf(&mut self, index: usize, source: usize, exponent: f64) {
        let base = self.v[source];
        let value = base.powf(exponent);
        let derivative_scale = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(value, base, exponent, 1.0, 0.0);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_scaled(&mut self, index: usize, source: usize, value: f64, derivative_scale: f64) {
        let dn = self.dn[source];
        let db = self.db[source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = derivative_scale * dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = derivative_scale * db[axis]; }
    }

}


pub(crate) struct ReactiveScratch<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> {
    pub(crate) v: [f64; VARIABLE_COUNT],
    pub(crate) b: [bool; VARIABLE_COUNT],
    pub(crate) dn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) db: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
    pub(crate) rv: [f64; VARIABLE_COUNT],
    pub(crate) rdn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) rdb: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
}

impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize> ReactiveScratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT> {
    pub(crate) fn new() -> Self {
        *Self::new_box()
    }

    pub(crate) fn new_box() -> Box<Self> {
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    pub(crate) fn new_value() -> Self {
        Self {
            v: [0.0; VARIABLE_COUNT],
            b: [false; VARIABLE_COUNT],
            dn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            db: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
            rv: [0.0; VARIABLE_COUNT],
            rdn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            rdb: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
        }
    }

    #[inline]
    pub(crate) fn ad_value(&self, index: usize) -> AdValue<NODE_COUNT, BRANCH_COUNT> {
        AdValue { value: self.v[index], dn: self.dn[index], db: self.db[index] }
    }

    #[inline]
    pub(crate) fn store_ad(&mut self, index: usize, value: &AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn copy_ad(&mut self, target: usize, source: usize) {
        self.v[target] = self.v[source];
        self.dn[target] = self.dn[source];
        self.db[target] = self.db[source];
    }

    #[inline]
    pub(crate) fn store_scalar(&mut self, index: usize, value: f64) {
        self.v[index] = value;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
    }

    #[inline]
    pub(crate) fn store_ad_value(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn store_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        self.store_scaled_voltage(index, ctx, nodes, pos, neg, 1.0);
    }

    #[inline]
    pub(crate) fn store_scaled_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, scale: f64) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        self.v[index] = (pos_value - neg_value) * scale;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += scale; }
        if let Some(node) = neg { self.dn[index][node] -= scale; }
    }

    #[inline]
    pub(crate) fn store_offset_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, offset: f64) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        self.v[index] = pos_value - neg_value + offset;
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += 1.0; }
        if let Some(node) = neg { self.dn[index][node] -= 1.0; }
    }

    #[inline]
    pub(crate) fn store_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let voltage = pos_value - neg_value;
        let derivative_scale = if voltage >= 0.0 { 1.0 } else { -1.0 };
        self.v[index] = voltage.abs();
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = pos { self.dn[index][node] += derivative_scale; }
        if let Some(node) = neg { self.dn[index][node] -= derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sub_voltage_abs_voltage(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], left_pos: Option<usize>, left_neg: Option<usize>, abs_pos: Option<usize>, abs_neg: Option<usize>) {
        let left_pos_value = left_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let left_neg_value = left_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_pos_value = abs_pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_neg_value = abs_neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let abs_voltage = abs_pos_value - abs_neg_value;
        let abs_derivative_scale = if abs_voltage >= 0.0 { 1.0 } else { -1.0 };
        self.v[index] = left_pos_value - left_neg_value - abs_voltage.abs();
        self.dn[index] = [0.0; NODE_COUNT];
        self.db[index] = [0.0; BRANCH_COUNT];
        if let Some(node) = left_pos { self.dn[index][node] += 1.0; }
        if let Some(node) = left_neg { self.dn[index][node] -= 1.0; }
        if let Some(node) = abs_pos { self.dn[index][node] -= abs_derivative_scale; }
        if let Some(node) = abs_neg { self.dn[index][node] += abs_derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_voltage_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let voltage = pos_value - neg_value;
        self.v[index] = value.value * voltage;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * voltage; }
        if let Some(node) = pos { self.dn[index][node] += value.value; }
        if let Some(node) = neg { self.dn[index][node] -= value.value; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * voltage; }
    }

    #[inline]
    pub(crate) fn store_div_voltage_by_ad(&mut self, index: usize, ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let pos_value = pos.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let neg_value = neg.map(|node| ctx.node_voltage(nodes[node])).unwrap_or(0.0);
        let reciprocal = 1.0 / right.value;
        let quotient = (pos_value - neg_value) * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = right.dn[axis] * right_scale; }
        if let Some(node) = pos { self.dn[index][node] += reciprocal; }
        if let Some(node) = neg { self.dn[index][node] -= reciprocal; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = left.value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right.value + left.value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right.value + left.value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_rem_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::rem(left, right));
    }

    #[inline]
    pub(crate) fn store_pow_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_min_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let selected = if left.value <= right.value { left } else { right };
        self.v[index] = selected.value;
        self.dn[index] = selected.dn;
        self.db[index] = selected.db;
    }

    #[inline]
    pub(crate) fn store_max_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let selected = if left.value >= right.value { left } else { right };
        self.v[index] = selected.value;
        self.dn[index] = selected.dn;
        self.db[index] = selected.db;
    }

    #[inline]
    pub(crate) fn store_hypot_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::hypot(left, right));
    }

    #[inline]
    pub(crate) fn store_atan2_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atan2(left, right));
    }

    #[inline]
    pub(crate) fn store_scale_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, value, offset);
    }

    #[inline]
    pub(crate) fn store_offset_ad_value(&mut self, index: usize, mut value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        value.value += offset;
        self.store_ad_value(index, value);
    }

    #[inline]
    pub(crate) fn store_offset_scaled_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64) {
        self.v[index] = value.value * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value + right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value - right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = left.value * right.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right.value + left.value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right.value + left.value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let output = value.value.exp();
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_offset_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let root = value.value.sqrt();
        self.v[index] = root + offset;
        let derivative_scale = 1.0 / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = value.value.ln() + offset;
        let derivative_scale = 1.0 / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        if value.value > 80.0 {
            self.v[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) + offset;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * LIMEXP_MAX; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * LIMEXP_MAX; }
        } else if value.value < -80.0 {
            self.store_scalar(index, 1.804851387e-35 + offset);
        } else {
            let output = value.value.exp();
            self.v[index] = output + offset;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        }
    }

    #[inline]
    pub(crate) fn store_offset_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, offset: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0); }
    }

    #[inline]
    pub(crate) fn store_offset_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.v[index] = scalar - value.value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_pow_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_offset_min_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_hypot_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::hypot(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atan2_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atan2(left, right), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let exponent = value.value;
        let output = scalar.powf(exponent);
        self.v[index] = output + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_offset_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max_from_scalar(scalar, value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_rem_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::rem_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_min_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::min_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_max_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64, offset: f64) {
        self.store_offset_ad_value(index, AdValue::max_with_scalar(value, scalar), offset);
    }

    #[inline]
    pub(crate) fn store_offset_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::abs(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_square_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::square(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::limexp(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_log10_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::log10(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_sin_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::sin(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_cos_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::cos(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_tan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::tan(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atan(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_sinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::sinh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_cosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::cosh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_tanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::tanh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_asinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::asinh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_acosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::acosh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_atanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::atanh(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_floor_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::floor(value), offset);
    }

    #[inline]
    pub(crate) fn store_offset_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        self.store_offset_ad_value(index, AdValue::ceil(value), offset);
    }

    #[inline]
    pub(crate) fn store_scaled_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64, scale: f64) {
        self.v[index] = (value.value + offset) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (left.value + right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (left.value - right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = left.value * right.value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right.dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right.db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let output = value.value.exp() * scale;
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_scaled_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value.ln() * scale;
        let derivative_scale = scale / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let root = value.value.sqrt();
        self.v[index] = root * scale;
        let derivative_scale = scale / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        if value.value < 80.0 {
            let output = value.value.exp() * scale;
            self.v[index] = output;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        } else {
            self.v[index] = LIMEXP_MAX * (1.0 + (value.value - 80.0)) * scale;
            let derivative_scale = LIMEXP_MAX * scale;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        if value.value > 80.0 {
            self.v[index] = LIMEXP_MAX * (1.0 + value.value - 80.0) * scale;
            let derivative_scale = LIMEXP_MAX * scale;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
        } else if value.value < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * scale);
        } else {
            let output = value.value.exp() * scale;
            self.v[index] = output;
            for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
            for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
        }
    }

    #[inline]
    pub(crate) fn store_scaled_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = value.value.abs() * scale;
        let derivative_scale = if value.value >= 0.0 { scale } else { -scale };
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, scale: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        self.v[index] = (scalar - value.value) * scale;
        let derivative_scale = -scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * scale;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_neg_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = -value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = value.value.sqrt();
        self.v[index] = root;
        let derivative_scale = 1.0 / (2.0 * root);
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let root = (value.value + offset).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_scaled_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let root = (value.value * scale).sqrt();
        let derivative_scale = scale / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_add_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (left.value + right.value).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_sub_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (left.value - right.value).sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_mul_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = left.value * right.value;
        let root = raw.sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_div_ad(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / right.value;
        let raw = left.value * reciprocal;
        let root = raw.sqrt();
        let derivative_scale = 1.0 / (2.0 * root);
        let right_scale = -raw * reciprocal;
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right.dn[axis] * right_scale) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right.db[axis] * right_scale) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value.abs();
        let root = raw.sqrt();
        let derivative_scale = if value.value >= 0.0 { 1.0 / (2.0 * root) } else { -1.0 / (2.0 * root) };
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = (scalar - value.value).sqrt();
        let derivative_scale = -1.0 / (2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = scalar / value.value;
        let root = raw.sqrt();
        let derivative_scale = -raw / (value.value * 2.0 * root);
        self.v[index] = root;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let output = value.value.exp();
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * output; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * output; }
    }

    #[inline]
    pub(crate) fn store_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value.ln();
        let derivative_scale = 1.0 / value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value.abs();
        let derivative_scale = if value.value >= 0.0 { 1.0 } else { -1.0 };
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_square_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = value.value * value.value;
        let derivative_scale = 2.0 * value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_limexp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::limexp(value));
    }

    #[inline]
    pub(crate) fn store_limited_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::limited_exp(value));
    }

    #[inline]
    pub(crate) fn store_log10_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::log10(value));
    }

    #[inline]
    pub(crate) fn store_sin_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sin(value));
    }

    #[inline]
    pub(crate) fn store_cos_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::cos(value));
    }

    #[inline]
    pub(crate) fn store_tan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::tan(value));
    }

    #[inline]
    pub(crate) fn store_atan_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atan(value));
    }

    #[inline]
    pub(crate) fn store_sinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sinh(value));
    }

    #[inline]
    pub(crate) fn store_cosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::cosh(value));
    }

    #[inline]
    pub(crate) fn store_tanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::tanh(value));
    }

    #[inline]
    pub(crate) fn store_asinh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::asinh(value));
    }

    #[inline]
    pub(crate) fn store_acosh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::acosh(value));
    }

    #[inline]
    pub(crate) fn store_atanh_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::atanh(value));
    }

    #[inline]
    pub(crate) fn store_floor_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::floor(value));
    }

    #[inline]
    pub(crate) fn store_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ceil(value));
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.v[index] = scalar - value.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let denominator = value.value + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_scaled_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let denominator = value.value * scale;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * scale;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_add_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value + right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sub_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value - right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_mul_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value * right.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right.value + left.value * right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right.value + left.value * right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_div_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let right_reciprocal = 1.0 / right.value;
        let denominator = left.value * right_reciprocal;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        let right_scale = -denominator * right_reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * right_reciprocal + right.dn[axis] * right_scale) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * right_reciprocal + right.db[axis] * right_scale) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sqrt_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let root = value.value.sqrt();
        let reciprocal = 1.0 / root;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient / (2.0 * value.value);
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_square_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = value.value * value.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -2.0 * quotient / value.value;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sub_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = denominator_scalar - value.value;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_div_from_scalar_ad(&mut self, index: usize, scalar: f64, denominator_scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let quotient_scale = scalar / denominator_scalar;
        self.v[index] = value.value * quotient_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * quotient_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * quotient_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_pow_ad(&mut self, index: usize, scalar: f64, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = left.value.powf(right.value);
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, left.value, right.value, left.dn[axis], right.dn[axis]) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, left.value, right.value, left.db[axis], right.db[axis]) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_powf_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let denominator = value.value.powf(exponent);
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, value.value, exponent, value.dn[axis], 0.0) * denominator_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(denominator, value.value, exponent, value.db[axis], 0.0) * denominator_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_exp_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let denominator = value.value.exp();
        let quotient = scalar / denominator;
        let derivative_scale = -quotient;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sin_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let denominator = raw.sin();
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * raw.cos();
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_sinh_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let denominator = raw.sinh();
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal * raw.cosh();
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_rem_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::rem_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_pow_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let exponent = value.value;
        let output = scalar.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, scalar, exponent, 0.0, value.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_min_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::min_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_max_from_scalar_ad(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::max_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::rem_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_min_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::min_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_max_with_scalar_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scalar: f64) {
        self.store_ad_value(index, AdValue::max_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_powf_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let output = value.value.powf(exponent);
        self.v[index] = output;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.dn[axis], 0.0); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, value.value, exponent, value.db[axis], 0.0); }
    }

    #[inline]
    pub(crate) fn store_add(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_square(&mut self, index: usize, source: usize) {
        let source_value = self.v[source];
        self.store_unary_scaled(index, source, source_value * source_value, 2.0 * source_value);
    }

    #[inline]
    pub(crate) fn store_div(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = (left_value + right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = (left.value + right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = (left_value - right.value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right.dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right.db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = (left.value - right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale + right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale + right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] + right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] + right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale - right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale - right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] - right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] - right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right.value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right.value + left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = left_value * middle_value;
        self.v[index] = product_value * right_value;
        for axis in 0..NODE_COUNT { let product_derivative = left_dn[axis] * middle_value + left_value * middle_dn[axis]; self.dn[index][axis] = product_derivative * right_value + product_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left_db[axis] * middle_value + left_value * middle_db[axis]; self.db[index][axis] = product_derivative * right_value + product_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = middle_value * right_value;
        self.v[index] = left_value * product_value;
        for axis in 0..NODE_COUNT { let product_derivative = middle_dn[axis] * right_value + middle_value * right_dn[axis]; self.dn[index][axis] = left_dn[axis] * product_value + left_value * product_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = middle_db[axis] * right_value + middle_value * right_db[axis]; self.db[index][axis] = left_db[axis] * product_value + left_value * product_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_product_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        self.v[index] = product_value * source_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; self.dn[index][axis] = product_derivative * source_value + product_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; self.db[index][axis] = product_derivative * source_value + product_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_product_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        self.v[index] = source_value * product_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; self.dn[index][axis] = source_dn[axis] * product_value + source_value * product_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; self.db[index][axis] = source_db[axis] * product_value + source_value * product_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul3_affine_lhs(&mut self, index: usize, left: usize, middle: usize, scale: f64, offset: f64, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = left_value * middle_value;
        let affine_value = product_value * scale + offset;
        self.v[index] = affine_value * right_value;
        for axis in 0..NODE_COUNT { let product_derivative = left_dn[axis] * middle_value + left_value * middle_dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = affine_derivative * right_value + affine_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left_db[axis] * middle_value + left_value * middle_db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = affine_derivative * right_value + affine_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul3_affine_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let product_value = middle_value * right_value;
        let affine_value = product_value * scale + offset;
        self.v[index] = left_value * affine_value;
        for axis in 0..NODE_COUNT { let product_derivative = middle_dn[axis] * right_value + middle_value * right_dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = left_dn[axis] * affine_value + left_value * affine_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = middle_db[axis] * right_value + middle_value * right_db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = left_db[axis] * affine_value + left_value * affine_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_affine_product_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        let affine_value = product_value * scale + offset;
        self.v[index] = affine_value * source_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = affine_derivative * source_value + affine_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = affine_derivative * source_value + affine_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_ad_affine_product_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, offset: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let product_value = left.value * right.value;
        let affine_value = product_value * scale + offset;
        self.v[index] = source_value * affine_value;
        for axis in 0..NODE_COUNT { let product_derivative = left.dn[axis] * right.value + left.value * right.dn[axis]; let affine_derivative = product_derivative * scale; self.dn[index][axis] = source_dn[axis] * affine_value + source_value * affine_derivative; }
        for axis in 0..BRANCH_COUNT { let product_derivative = left.db[axis] * right.value + left.value * right.db[axis]; let affine_derivative = product_derivative * scale; self.db[index][axis] = source_db[axis] * affine_value + source_value * affine_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_add_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let sum = left_value + middle_value;
        self.v[index] = sum * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + middle_dn[axis]) * right_value + sum * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + middle_db[axis]) * right_value + sum * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_add_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let sum = middle_value + right_value;
        self.v[index] = left_value * sum;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * sum + left_value * (middle_dn[axis] + right_dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * sum + left_value * (middle_db[axis] + right_db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_lhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let difference = left_value - middle_value;
        self.v[index] = difference * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - middle_dn[axis]) * right_value + difference * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - middle_db[axis]) * right_value + difference * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_rhs(&mut self, index: usize, left: usize, middle: usize, right: usize) {
        let left_value = self.v[left];
        let middle_value = self.v[middle];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let middle_dn = self.dn[middle];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let middle_db = self.db[middle];
        let right_db = self.db[right];
        let difference = middle_value - right_value;
        self.v[index] = left_value * difference;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * difference + left_value * (middle_dn[axis] - right_dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * difference + left_value * (middle_db[axis] - right_db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_add_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let sum = left.value + right.value;
        self.v[index] = sum * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] + right.dn[axis]) * source_value + sum * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] + right.db[axis]) * source_value + sum * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_add_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let sum = left.value + right.value;
        self.v[index] = source_value * sum;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * sum + source_value * (left.dn[axis] + right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * sum + source_value * (left.db[axis] + right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let difference = left.value - right.value;
        self.v[index] = difference * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] - right.dn[axis]) * source_value + difference * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] - right.db[axis]) * source_value + difference * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let difference = left.value - right.value;
        self.v[index] = source_value * difference;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * difference + source_value * (left.dn[axis] - right.dn[axis]); }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * difference + source_value * (left.db[axis] - right.db[axis]); }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {
        let left_value = scalar - self.v[value];
        let source_value = self.v[source];
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value_dn[axis] * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value_db[axis] * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {
        let source_value = self.v[source];
        let right_value = scalar - self.v[value];
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value - source_value * value_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value - source_value * value_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let left_value = scalar - value.value;
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -value.dn[axis] * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -value.db[axis] * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_sub_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let right_value = scalar - value.value;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value - source_value * value.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value - source_value * value.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_lhs(&mut self, index: usize, scalar: f64, value: usize, source: usize) {
        let source_value = self.v[source];
        let denominator = self.v[value];
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_rhs(&mut self, index: usize, source: usize, scalar: f64, value: usize) {
        let source_value = self.v[source];
        let denominator = self.v[value];
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * quotient + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * quotient + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_ad_lhs(&mut self, index: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_from_scalar_ad_rhs(&mut self, index: usize, source: usize, scalar: f64, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let derivative_scale = -quotient * reciprocal;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * quotient + source_value * value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * quotient + source_value * value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_pow_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = output * source_value;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); self.dn[index][axis] = derivative * source_value + output * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); self.db[index][axis] = derivative * source_value + output * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_pow_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = left.value;
        let exponent = right.value;
        let output = base.powf(exponent);
        self.v[index] = source_value * output;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.dn[axis], right.dn[axis]); self.dn[index][axis] = source_dn[axis] * output + source_value * derivative; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, left.db[axis], right.db[axis]); self.db[index][axis] = source_db[axis] * output + source_value * derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_powf_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = value.value;
        let output = base.powf(exponent);
        self.v[index] = output * source_value;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.dn[axis], 0.0); self.dn[index][axis] = derivative * source_value + output * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.db[axis], 0.0); self.db[index][axis] = derivative * source_value + output * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_powf_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, exponent: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let base = value.value;
        let output = base.powf(exponent);
        self.v[index] = source_value * output;
        for axis in 0..NODE_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.dn[axis], 0.0); self.dn[index][axis] = source_dn[axis] * output + source_value * derivative; }
        for axis in 0..BRANCH_COUNT { let derivative = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(output, base, exponent, value.db[axis], 0.0); self.db[index][axis] = source_db[axis] * output + source_value * derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_lhs(&mut self, index: usize, left: usize, right: usize) {
        let left_value = -self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_rhs(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = -self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value - left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value - left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let left_value = -left.value;
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = -left.dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = -left.db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_neg_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let right_value = -right.value;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value - left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value - left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_lhs(&mut self, index: usize, numerator: usize, denominator: usize, source: usize) {
        let numerator_value = self.v[numerator];
        let denominator_value = self.v[denominator];
        let source_value = self.v[source];
        let numerator_dn = self.dn[numerator];
        let denominator_dn = self.dn[denominator];
        let source_dn = self.dn[source];
        let numerator_db = self.db[numerator];
        let denominator_db = self.db[denominator];
        let source_db = self.db[source];
        let reciprocal = 1.0 / denominator_value;
        let quotient = numerator_value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { let quotient_derivative = numerator_dn[axis] * reciprocal + denominator_dn[axis] * denominator_scale; self.dn[index][axis] = quotient_derivative * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = numerator_db[axis] * reciprocal + denominator_db[axis] * denominator_scale; self.db[index][axis] = quotient_derivative * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_rhs(&mut self, index: usize, source: usize, numerator: usize, denominator: usize) {
        let source_value = self.v[source];
        let numerator_value = self.v[numerator];
        let denominator_value = self.v[denominator];
        let source_dn = self.dn[source];
        let numerator_dn = self.dn[numerator];
        let denominator_dn = self.dn[denominator];
        let source_db = self.db[source];
        let numerator_db = self.db[numerator];
        let denominator_db = self.db[denominator];
        let reciprocal = 1.0 / denominator_value;
        let quotient = numerator_value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { let quotient_derivative = numerator_dn[axis] * reciprocal + denominator_dn[axis] * denominator_scale; self.dn[index][axis] = source_dn[axis] * quotient + source_value * quotient_derivative; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = numerator_db[axis] * reciprocal + denominator_db[axis] * denominator_scale; self.db[index][axis] = source_db[axis] * quotient + source_value * quotient_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = quotient * source_value;
        for axis in 0..NODE_COUNT { let quotient_derivative = left.dn[axis] * reciprocal + right.dn[axis] * denominator_scale; self.dn[index][axis] = quotient_derivative * source_value + quotient * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = left.db[axis] * reciprocal + right.db[axis] * denominator_scale; self.db[index][axis] = quotient_derivative * source_value + quotient * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_div_ad_rhs(&mut self, index: usize, source: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let denominator_scale = -quotient * reciprocal;
        self.v[index] = source_value * quotient;
        for axis in 0..NODE_COUNT { let quotient_derivative = left.dn[axis] * reciprocal + right.dn[axis] * denominator_scale; self.dn[index][axis] = source_dn[axis] * quotient + source_value * quotient_derivative; }
        for axis in 0..BRANCH_COUNT { let quotient_derivative = left.db[axis] * reciprocal + right.db[axis] * denominator_scale; self.db[index][axis] = source_db[axis] * quotient + source_value * quotient_derivative; }
    }

    #[inline]
    pub(crate) fn store_mul_square_lhs(&mut self, index: usize, value: usize, source: usize) {
        let raw = self.v[value];
        let square = raw * raw;
        let source_value = self.v[source];
        let value_dn = self.dn[value];
        let source_dn = self.dn[source];
        let value_db = self.db[value];
        let source_db = self.db[source];
        let derivative_scale = 2.0 * raw;
        self.v[index] = square * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + square * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + square * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_square_rhs(&mut self, index: usize, source: usize, value: usize) {
        let source_value = self.v[source];
        let raw = self.v[value];
        let square = raw * raw;
        let source_dn = self.dn[source];
        let value_dn = self.dn[value];
        let source_db = self.db[source];
        let value_db = self.db[value];
        let derivative_scale = 2.0 * raw;
        self.v[index] = source_value * square;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * square + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * square + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_lhs(&mut self, index: usize, value_source: usize, source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let value_dn = self.dn[value_source];
        let source_dn = self.dn[source];
        let value_db = self.db[value_source];
        let source_db = self.db[source];
        self.v[index] = unary_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value_dn[axis] * derivative_scale * source_value + unary_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value_db[axis] * derivative_scale * source_value + unary_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_rhs(&mut self, index: usize, source: usize, value_source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let value_dn = self.dn[value_source];
        let source_db = self.db[source];
        let value_db = self.db[value_source];
        self.v[index] = source_value * unary_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * unary_value + source_value * value_dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * unary_value + source_value * value_db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = unary_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * derivative_scale * source_value + unary_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * derivative_scale * source_value + unary_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_unary_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, unary_value: f64, derivative_scale: f64) {
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * unary_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * unary_value + source_value * value.dn[axis] * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * unary_value + source_value * value.db[axis] * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_mul_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let unary_value = self.v[value_source].exp();
        self.store_mul_unary_lhs(index, value_source, source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let unary_value = self.v[value_source].exp();
        self.store_mul_unary_rhs(index, source, value_source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let unary_value = value.value.exp();
        self.store_mul_unary_ad_lhs(index, value, source, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let unary_value = value.value.exp();
        self.store_mul_unary_ad_rhs(index, source, value, unary_value, unary_value);
    }

    #[inline]
    pub(crate) fn store_mul_ln_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_ln_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let unary_value = self.v[value_source].sqrt();
        self.store_mul_unary_lhs(index, value_source, source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let unary_value = self.v[value_source].sqrt();
        self.store_mul_unary_rhs(index, source, value_source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let unary_value = value.value.sqrt();
        self.store_mul_unary_ad_lhs(index, value, source, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_sqrt_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let unary_value = value.value.sqrt();
        self.store_mul_unary_ad_rhs(index, source, value, unary_value, 1.0 / (2.0 * unary_value));
    }

    #[inline]
    pub(crate) fn store_mul_abs_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_abs_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_mul_cos_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_lhs(index, value_source, source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        self.store_mul_unary_rhs(index, source, value_source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        self.store_mul_unary_ad_lhs(index, value, source, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_cos_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        self.store_mul_unary_ad_rhs(index, source, value, raw.cos(), -raw.sin());
    }

    #[inline]
    pub(crate) fn store_mul_tanh_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        let cosh = raw.cosh();
        self.store_mul_unary_lhs(index, value_source, source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        let cosh = raw.cosh();
        self.store_mul_unary_rhs(index, source, value_source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        let cosh = raw.cosh();
        self.store_mul_unary_ad_lhs(index, value, source, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_tanh_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        let cosh = raw.cosh();
        self.store_mul_unary_ad_rhs(index, source, value, raw.tanh(), 1.0 / (cosh * cosh));
    }

    #[inline]
    pub(crate) fn store_mul_limexp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); } else { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        if raw < 80.0 { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); } else { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); } else { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limexp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        if raw < 80.0 { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); } else { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_lhs(&mut self, index: usize, value_source: usize, source: usize) {
        let raw = self.v[value_source];
        if raw > 80.0 { self.store_mul_unary_lhs(index, value_source, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_lhs(index, value_source, source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_lhs(index, value_source, source, value, value); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_rhs(&mut self, index: usize, source: usize, value_source: usize) {
        let raw = self.v[value_source];
        if raw > 80.0 { self.store_mul_unary_rhs(index, source, value_source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_rhs(index, source, value_source, 1.804851387e-35, 0.0); } else { let value = raw.exp(); self.store_mul_unary_rhs(index, source, value_source, value, value); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, source: usize) {
        let raw = value.value;
        if raw > 80.0 { self.store_mul_unary_ad_lhs(index, value, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_lhs(index, value, source, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_lhs(index, value, source, output, output); }
    }

    #[inline]
    pub(crate) fn store_mul_limited_exp_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let raw = value.value;
        if raw > 80.0 { self.store_mul_unary_ad_rhs(index, source, value, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX); } else if raw < -80.0 { self.store_mul_unary_ad_rhs(index, source, value, 1.804851387e-35, 0.0); } else { let output = raw.exp(); self.store_mul_unary_ad_rhs(index, source, value, output, output); }
    }

    #[inline]
    pub(crate) fn store_mul_offset_lhs(&mut self, index: usize, left: usize, offset: f64, right: usize) {
        let left_value = self.v[left] + offset;
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_rhs(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right] + offset;
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64, right: usize) {
        let left_value = left.value + offset;
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_offset_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, offset: f64) {
        let left_value = self.v[left];
        let right_value = right.value + offset;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right.db[axis]; }
    }

    pub(crate) fn store_mul_scale_ad_lhs(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64, source: usize) {
        let left_value = value.value * scale;
        let source_value = self.v[source];
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = left_value * source_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = value.dn[axis] * scale * source_value + left_value * source_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = value.db[axis] * scale * source_value + left_value * source_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_scale_ad_rhs(&mut self, index: usize, source: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let source_value = self.v[source];
        let right_value = value.value * scale;
        let source_dn = self.dn[source];
        let source_db = self.db[source];
        self.v[index] = source_value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = source_dn[axis] * right_value + source_value * value.dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = source_db[axis] * right_value + source_value * value.db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_mul_scaled_ad_rhs(&mut self, index: usize, left: usize, scale: f64, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left] * scale;
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * scale * right.value + left_value * right.dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * scale * right.value + left_value * right.db[axis]; }
    }

    #[inline]
    pub(crate) fn store_mul_scaled_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right] * scale;
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis] * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis] * scale; }
    }

    #[inline]
    pub(crate) fn store_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right.dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right.db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left.dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left.db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad_rhs(&mut self, index: usize, left: usize, right: AdValue<NODE_COUNT, BRANCH_COUNT>, scale: f64) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right.dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right.db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div_ad_lhs(&mut self, index: usize, left: AdValue<NODE_COUNT, BRANCH_COUNT>, right: usize, scale: f64) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left.dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left.db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value + right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value - right_value) * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_add_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * left_scale + right_value * right_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_sub_scaled_inputs(&mut self, index: usize, left: usize, left_scale: f64, right: usize, right_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * left_scale - right_value * right_scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale - right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale - right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_add(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value + right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] + right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] + right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_sub(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value - right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] - right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] - right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_mul(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis]; }
    }

    #[inline]
    pub(crate) fn store_offset_div(&mut self, index: usize, left: usize, right: usize, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_add(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value + right_value) * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_sub(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = (left_value - right_value) * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_mul(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = left_value * right_value * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * scale; }
    }

    #[inline]
    pub(crate) fn store_offset_scaled_div(&mut self, index: usize, left: usize, right: usize, scale: f64, offset: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient * scale + offset;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale; }
    }

    #[inline]
    pub(crate) fn store_scale(&mut self, index: usize, source: usize, scale: f64) {
        self.store_unary_scaled(index, source, self.v[source] * scale, scale);
    }

    #[inline]
    pub(crate) fn store_offset(&mut self, index: usize, source: usize, offset: f64) {
        self.v[index] = self.v[source] + offset;
        self.dn[index] = self.dn[source];
        self.db[index] = self.db[source];
    }

    #[inline]
    pub(crate) fn store_offset_scaled(&mut self, index: usize, source: usize, scale: f64, offset: f64) {
        self.store_unary_scaled(index, source, self.v[source] * scale + offset, scale);
    }

    #[inline]
    pub(crate) fn store_scaled_offset(&mut self, index: usize, source: usize, offset: f64, scale: f64) {
        self.store_unary_scaled(index, source, (self.v[source] + offset) * scale, scale);
    }

    #[inline]
    pub(crate) fn store_neg(&mut self, index: usize, source: usize) {
        self.store_scale(index, source, -1.0);
    }

    #[inline]
    pub(crate) fn store_abs(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_min_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {
        if self.v[source] <= scalar {
            self.copy_ad(index, source);
        } else {
            self.store_scalar(index, scalar);
        }
    }

    #[inline]
    pub(crate) fn store_max_with_scalar(&mut self, index: usize, source: usize, scalar: f64) {
        if self.v[source] >= scalar {
            self.copy_ad(index, source);
        } else {
            self.store_scalar(index, scalar);
        }
    }

    #[inline]
    pub(crate) fn store_min(&mut self, index: usize, left: usize, right: usize) {
        if self.v[left] <= self.v[right] {
            self.copy_ad(index, left);
        } else {
            self.copy_ad(index, right);
        }
    }

    #[inline]
    pub(crate) fn store_max(&mut self, index: usize, left: usize, right: usize) {
        if self.v[left] >= self.v[right] {
            self.copy_ad(index, left);
        } else {
            self.copy_ad(index, right);
        }
    }

    #[inline]
    pub(crate) fn store_min3(&mut self, index: usize, first: usize, second: usize, third: usize) {
        let mut selected = first;
        if self.v[second] < self.v[selected] { selected = second; }
        if self.v[third] < self.v[selected] { selected = third; }
        self.copy_ad(index, selected);
    }

    #[inline]
    pub(crate) fn store_max3(&mut self, index: usize, first: usize, second: usize, third: usize) {
        let mut selected = first;
        if self.v[second] > self.v[selected] { selected = second; }
        if self.v[third] > self.v[selected] { selected = third; }
        self.copy_ad(index, selected);
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {
        self.store_unary_scaled(index, source, scalar - self.v[source], -1.0);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar(&mut self, index: usize, scalar: f64, source: usize) {
        let reciprocal = 1.0 / self.v[source];
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_input(&mut self, index: usize, scalar: f64, source: usize, offset: f64) {
        let denominator = self.v[source] + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64) {
        let denominator = self.v[source] * scale;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_offset_scaled_input(&mut self, index: usize, scalar: f64, source: usize, scale: f64, offset: f64) {
        let denominator = self.v[source] * scale + offset;
        let reciprocal = 1.0 / denominator;
        let quotient = scalar * reciprocal;
        self.store_unary_scaled(index, source, quotient, -quotient * reciprocal * scale);
    }

    #[inline]
    pub(crate) fn store_sqrt(&mut self, index: usize, source: usize) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_scaled_input(&mut self, index: usize, source: usize, scale: f64, offset: f64) {
        let raw = self.v[source] * scale + offset;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_square_offset(&mut self, index: usize, source: usize, offset: f64) {
        let source_value = self.v[source];
        let value = (source_value * source_value + offset).sqrt();
        self.store_unary_scaled(index, source, value, source_value / value);
    }

    #[inline]
    pub(crate) fn store_sqrt_square_add(&mut self, index: usize, square_source: usize, add_source: usize) {
        let square_value = self.v[square_source];
        let value = (square_value * square_value + self.v[add_source]).sqrt();
        let square_scale = square_value / value;
        let add_scale = 1.0 / (2.0 * value);
        let square_dn = self.dn[square_source];
        let add_dn = self.dn[add_source];
        let square_db = self.db[square_source];
        let add_db = self.db[add_source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = square_dn[axis] * square_scale + add_dn[axis] * add_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = square_db[axis] * square_scale + add_db[axis] * add_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_square_sum(&mut self, index: usize, left: usize, right: usize) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let value = (left_value * left_value + right_value * right_value).sqrt();
        let left_scale = left_value / value;
        let right_scale = right_value / value;
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = left_dn[axis] * left_scale + right_dn[axis] * right_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = left_db[axis] * left_scale + right_db[axis] * right_scale; }
    }

    #[inline]
    pub(crate) fn store_exp(&mut self, index: usize, source: usize) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_limexp(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_exp(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].exp() * scale;
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_ln(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn ln_one_plus_exp_raw(raw: f64) -> (f64, f64) {
        if raw > 0.0 {
            (raw + (-raw).exp().ln_1p(), 1.0 / (1.0 + (-raw).exp()))
        } else {
            let exp = raw.exp();
            (exp.ln_1p(), exp / (1.0 + exp))
        }
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp(&mut self, index: usize, source: usize) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source]);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_ln_one_plus_exp(&mut self, index: usize, source: usize, scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source]);
        self.store_unary_scaled(index, source, value * scale, derivative_scale * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value * scale, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_scaled_square(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw * raw * scale, 2.0 * raw * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_abs(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs() * scale, if raw >= 0.0 { scale } else { -scale });
    }

    #[inline]
    pub(crate) fn store_scaled_ln(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln() * scale, scale / raw);
    }

    #[inline]
    pub(crate) fn store_scaled_limexp(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp() * scale;
            self.store_unary_scaled(index, source, value, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * scale, LIMEXP_MAX * scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * scale, LIMEXP_MAX * scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * scale);
        } else {
            let value = raw.exp() * scale;
            self.store_unary_scaled(index, source, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_sqrt_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value, value * scale);
    }

    #[inline]
    pub(crate) fn store_limexp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value * scale);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX * scale);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX * scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, value * scale);
        }
    }

    #[inline]
    pub(crate) fn store_ln_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        self.store_unary_scaled(index, source, raw.ln(), scale / raw);
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source] * scale);
        self.store_unary_scaled(index, source, value, derivative_scale * scale);
    }

    #[inline]
    pub(crate) fn store_sin_scaled_input(&mut self, index: usize, source: usize, scale: f64) {
        let raw = self.v[source] * scale;
        self.store_unary_scaled(index, source, raw.sin(), raw.cos() * scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sqrt_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value * output_scale, output_scale * input_scale / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_scaled_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_limexp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) * output_scale, LIMEXP_MAX * output_scale * input_scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_limited_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) * output_scale, LIMEXP_MAX * output_scale * input_scale);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 * output_scale);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value * output_scale, value * output_scale * input_scale);
        }
    }

    #[inline]
    pub(crate) fn store_scaled_ln_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        self.store_unary_scaled(index, source, raw.ln() * output_scale, output_scale * input_scale / raw);
    }

    #[inline]
    pub(crate) fn store_scaled_ln_one_plus_exp_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(self.v[source] * input_scale);
        self.store_unary_scaled(index, source, value * output_scale, derivative_scale * input_scale * output_scale);
    }

    #[inline]
    pub(crate) fn store_scaled_sin_scaled_input(&mut self, index: usize, source: usize, input_scale: f64, output_scale: f64) {
        let raw = self.v[source] * input_scale;
        self.store_unary_scaled(index, source, raw.sin() * output_scale, raw.cos() * output_scale * input_scale);
    }

    #[inline]
    pub(crate) fn store_sqrt_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        let value = raw.exp();
        self.store_unary_scaled(index, source, value, value);
    }

    #[inline]
    pub(crate) fn store_ln_offset_input(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source] + offset;
        self.store_unary_scaled(index, source, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_offset_square(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw * raw + offset, 2.0 * raw);
    }

    #[inline]
    pub(crate) fn store_offset_abs(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.abs() + offset, if raw >= 0.0 { 1.0 } else { -1.0 });
    }

    #[inline]
    pub(crate) fn store_offset_sqrt(&mut self, index: usize, source: usize, offset: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value + offset, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_offset_exp(&mut self, index: usize, source: usize, offset: f64) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value + offset, value);
    }

    #[inline]
    pub(crate) fn store_offset_ln(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.ln() + offset, 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_offset_limexp(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value + offset, value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)) + offset, LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_offset_limited_exp(&mut self, index: usize, source: usize, offset: f64) {
        let raw = self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0) + offset, LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35 + offset);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value + offset, value);
        }
    }

    #[inline]
    pub(crate) fn store_sqrt_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        let value = raw.sqrt();
        self.store_unary_scaled(index, source, value, -1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_neg_input(&mut self, index: usize, source: usize) {
        let value = (-self.v[source]).exp();
        self.store_unary_scaled(index, source, value, -value);
    }

    #[inline]
    pub(crate) fn store_limexp_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, -value);
        } else {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + (raw - 80.0)), -LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        if raw > 80.0 {
            self.store_unary_scaled(index, source, LIMEXP_MAX * (1.0 + raw - 80.0), -LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_scaled(index, source, value, -value);
        }
    }

    #[inline]
    pub(crate) fn store_ln_neg_input(&mut self, index: usize, source: usize) {
        let raw = -self.v[source];
        self.store_unary_scaled(index, source, raw.ln(), -1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_one_plus_exp_neg_input(&mut self, index: usize, source: usize) {
        let (value, derivative_scale) = Self::ln_one_plus_exp_raw(-self.v[source]);
        self.store_unary_scaled(index, source, value, -derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_add_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] + right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_sub_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] - right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_mul_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * right_value + left_value * right_dn[axis]) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * right_value + left_value * right_db[axis]) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_unary_div_scaled(&mut self, index: usize, left: usize, right: usize, value: f64, derivative_scale: f64) {
        let left_value = self.v[left];
        let right_value = self.v[right];
        let left_dn = self.dn[left];
        let right_dn = self.dn[right];
        let left_db = self.db[left];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * derivative_scale; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = (left_db[axis] * reciprocal + right_db[axis] * right_scale) * derivative_scale; }
    }

    #[inline]
    pub(crate) fn store_sqrt_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        let value = raw.sqrt();
        self.store_unary_add_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        let value = raw.sqrt();
        self.store_unary_sub_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        let value = raw.sqrt();
        self.store_unary_mul_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_sqrt_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        let value = raw.sqrt();
        self.store_unary_div_scaled(index, left, right, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp_add(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] + self.v[right]).exp();
        self.store_unary_add_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_sub(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] - self.v[right]).exp();
        self.store_unary_sub_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_mul(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] * self.v[right]).exp();
        self.store_unary_mul_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_exp_div(&mut self, index: usize, left: usize, right: usize) {
        let value = (self.v[left] / self.v[right]).exp();
        self.store_unary_div_scaled(index, left, right, value, value);
    }

    #[inline]
    pub(crate) fn store_ln_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        self.store_unary_add_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        self.store_unary_sub_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        self.store_unary_mul_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_ln_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        self.store_unary_div_scaled(index, left, right, raw.ln(), 1.0 / raw);
    }

    #[inline]
    pub(crate) fn store_limexp_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_add_scaled(index, left, right, value, value);
        } else {
            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_sub_scaled(index, left, right, value, value);
        } else {
            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_mul_scaled(index, left, right, value, value);
        } else {
            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limexp_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        if raw < 80.0 {
            let value = raw.exp();
            self.store_unary_div_scaled(index, left, right, value, value);
        } else {
            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_add(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] + self.v[right];
        if raw > 80.0 {
            self.store_unary_add_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_add_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_sub(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] - self.v[right];
        if raw > 80.0 {
            self.store_unary_sub_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_sub_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_mul(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] * self.v[right];
        if raw > 80.0 {
            self.store_unary_mul_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_mul_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_limited_exp_div(&mut self, index: usize, left: usize, right: usize) {
        let raw = self.v[left] / self.v[right];
        if raw > 80.0 {
            self.store_unary_div_scaled(index, left, right, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX);
        } else if raw < -80.0 {
            self.store_scalar(index, 1.804851387e-35);
        } else {
            let value = raw.exp();
            self.store_unary_div_scaled(index, left, right, value, value);
        }
    }

    #[inline]
    pub(crate) fn store_sin(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.sin(), raw.cos());
    }

    #[inline]
    pub(crate) fn store_sinh(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.sinh(), raw.cosh());
    }

    #[inline]
    pub(crate) fn store_asinh(&mut self, index: usize, source: usize) {
        let raw = self.v[source];
        self.store_unary_scaled(index, source, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt());
    }

    #[inline]
    pub(crate) fn store_powf(&mut self, index: usize, source: usize, exponent: f64) {
        let base = self.v[source];
        let value = base.powf(exponent);
        let derivative_scale = AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(value, base, exponent, 1.0, 0.0);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_scaled(&mut self, index: usize, source: usize, value: f64, derivative_scale: f64) {
        let dn = self.dn[source];
        let db = self.db[source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT { self.dn[index][axis] = derivative_scale * dn[axis]; }
        for axis in 0..BRANCH_COUNT { self.db[index][axis] = derivative_scale * db[axis]; }
    }

}


pub(crate) struct AdValue<const NODE_COUNT: usize, const BRANCH_COUNT: usize> {
    pub(crate) value: f64,
    pub(crate) dn: [f64; NODE_COUNT],
    pub(crate) db: [f64; BRANCH_COUNT],
}

impl<const NODE_COUNT: usize, const BRANCH_COUNT: usize> AdValue<NODE_COUNT, BRANCH_COUNT> {
    #[inline]
    pub(crate) fn constant(value: f64) -> Self {
        Self { value, dn: [0.0; NODE_COUNT], db: [0.0; BRANCH_COUNT] }
    }
    #[inline]
    pub(crate) fn voltage(ctx: &GeneratedEvalContext<'_>, nodes: &[usize; NODE_COUNT], pos: Option<usize>, neg: Option<usize>) -> Self {
        let pos_value = pos.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);
        let neg_value = neg.map(|index| ctx.node_voltage(nodes[index])).unwrap_or(0.0);
        let mut value = Self::constant(pos_value - neg_value);
        if let Some(index) = pos { value.dn[index] += 1.0; }
        if let Some(index) = neg { value.dn[index] -= 1.0; }
        value
    }

    #[inline]
    pub(crate) fn branch_current(ctx: &GeneratedEvalContext<'_>, branches: &[usize; BRANCH_COUNT], slot: usize) -> Self {
        let mut value = Self::constant(ctx.branch_current(branches[slot]));
        value.db[slot] = 1.0;
        value
    }

    #[inline]
    pub(crate) fn neg(mut value: Self) -> Self {
        value.value = -value.value;
        for derivative in &mut value.dn { *derivative = -*derivative; }
        for derivative in &mut value.db { *derivative = -*derivative; }
        value
    }

    #[inline]
    pub(crate) fn add(left: Self, right: Self) -> Self {
        let mut value = left;
        value.value += right.value;
        for index in 0..NODE_COUNT { value.dn[index] += right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] += right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn sub(left: Self, right: Self) -> Self {
        let mut value = left;
        value.value -= right.value;
        for index in 0..NODE_COUNT { value.dn[index] -= right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] -= right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn add_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value * left_scale;
        let right_value = right.value * right_scale;
        value.value = left_value + right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * left_scale + right.dn[index] * right_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * left_scale + right.db[index] * right_scale; }
        value
    }

    #[inline]
    pub(crate) fn sub_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value * left_scale;
        let right_value = right.value * right_scale;
        value.value = left_value - right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * left_scale - right.dn[index] * right_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * left_scale - right.db[index] * right_scale; }
        value
    }

    #[inline]
    pub(crate) fn add_scaled_inputs3(first: Self, first_scale: f64, second: Self, second_scale: f64, third: Self, third_scale: f64) -> Self {
        let mut value = first;
        let first_value = value.value * first_scale;
        let second_value = second.value * second_scale;
        let third_value = third.value * third_scale;
        value.value = (first_value + second_value) + third_value;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * first_scale + second.dn[index] * second_scale) + third.dn[index] * third_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * first_scale + second.db[index] * second_scale) + third.db[index] * third_scale; }
        value
    }

    #[inline]
    pub(crate) fn add_scaled_inputs3_offset(first: Self, first_scale: f64, second: Self, second_scale: f64, third: Self, third_scale: f64, offset: f64) -> Self {
        let mut value = first;
        let first_value = value.value * first_scale;
        let second_value = second.value * second_scale;
        let third_value = third.value * third_scale;
        value.value = ((first_value + second_value) + third_value) + offset;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * first_scale + second.dn[index] * second_scale) + third.dn[index] * third_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * first_scale + second.db[index] * second_scale) + third.db[index] * third_scale; }
        value
    }

    #[inline]
    pub(crate) fn add_scaled_product(value: Self, value_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {
        let mut result = value;
        let value_term = result.value * value_scale;
        let product_left_value = product_left.value;
        let product_right_value = product_right.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * value_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * value_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_offset_product_lhs(value: Self, value_scale: f64, product_left: Self, product_left_offset: f64, product_right: Self, product_scale: f64) -> Self {
        let mut result = value;
        let value_term = result.value * value_scale;
        let product_left_value = product_left.value + product_left_offset;
        let product_right_value = product_right.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * value_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * value_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_offset_product_rhs(value: Self, value_scale: f64, product_left: Self, product_right: Self, product_right_offset: f64, product_scale: f64) -> Self {
        let mut result = value;
        let value_term = result.value * value_scale;
        let product_left_value = product_left.value;
        let product_right_value = product_right.value + product_right_offset;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * value_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * value_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_inputs_product(first: Self, first_scale: f64, second: Self, second_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {
        let mut result = first;
        let first_value = result.value * first_scale;
        let second_value = second.value * second_scale;
        let product_left_value = product_left.value;
        let product_right_value = product_right.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = first_value + second_value + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * first_scale + second.dn[index] * second_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * first_scale + second.db[index] * second_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_sub_value_product(scalar: f64, subtrahend: Self, value_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {
        let mut result = subtrahend;
        let value_term = (scalar - result.value) * value_scale;
        let product_left_value = product_left.value;
        let product_right_value = product_right.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = -result.dn[index] * value_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = -result.db[index] * value_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_sub_product_lhs(value: Self, value_scale: f64, scalar: f64, subtrahend: Self, product_right: Self, product_scale: f64) -> Self {
        let mut result = value;
        let value_term = result.value * value_scale;
        let product_left_value = scalar - subtrahend.value;
        let product_right_value = product_right.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * value_scale + (-subtrahend.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * value_scale + (-subtrahend.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_sub_product_rhs(value: Self, value_scale: f64, product_left: Self, scalar: f64, subtrahend: Self, product_scale: f64) -> Self {
        let mut result = value;
        let value_term = result.value * value_scale;
        let product_left_value = product_left.value;
        let product_right_value = scalar - subtrahend.value;
        let product_term = product_left_value * product_right_value * product_scale;
        result.value = value_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * value_scale + (product_left.dn[index] * product_right_value - product_left_value * subtrahend.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * value_scale + (product_left.db[index] * product_right_value - product_left_value * subtrahend.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_square_product(square_value: Self, square_scale: f64, product_left: Self, product_right: Self, product_scale: f64) -> Self {
        let mut result = square_value;
        let square_raw = result.value;
        let product_left_value = product_left.value;
        let product_right_value = product_right.value;
        let square_term = square_raw * square_raw * square_scale;
        let product_term = product_left_value * product_right_value * product_scale;
        let square_derivative_scale = 2.0 * square_raw * square_scale;
        result.value = square_term + product_term;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * square_derivative_scale + (product_left.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * square_derivative_scale + (product_left.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_scale; }
        result
    }

    #[inline]
    pub(crate) fn add_scaled_products(left_product_left: Self, left_product_right: Self, left_scale: f64, right_product_left: Self, right_product_right: Self, right_scale: f64) -> Self {
        let mut result = left_product_left;
        let left_product_left_value = result.value;
        let left_product_right_value = left_product_right.value;
        let right_product_left_value = right_product_left.value;
        let right_product_right_value = right_product_right.value;
        let left_product_term = left_product_left_value * left_product_right_value * left_scale;
        let right_product_term = right_product_left_value * right_product_right_value * right_scale;
        result.value = left_product_term + right_product_term;
        for index in 0..NODE_COUNT { result.dn[index] = (result.dn[index] * left_product_right_value + left_product_left_value * left_product_right.dn[index]) * left_scale + (right_product_left.dn[index] * right_product_right_value + right_product_left_value * right_product_right.dn[index]) * right_scale; }
        for index in 0..BRANCH_COUNT { result.db[index] = (result.db[index] * left_product_right_value + left_product_left_value * left_product_right.db[index]) * left_scale + (right_product_left.db[index] * right_product_right_value + right_product_left_value * right_product_right.db[index]) * right_scale; }
        result
    }

    #[inline]
    pub(crate) fn mul(left: Self, right: Self) -> Self {
        let mut value = left;
        let left_value = value.value;
        value.value = left_value * right.value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * right.value + left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * right.value + left_value * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn mul_scaled_lhs(left: Self, scale: f64, right: Self) -> Self {
        let mut value = left;
        let left_value = value.value;
        let scaled_left_value = left_value * scale;
        value.value = scaled_left_value * right.value;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * right.value + left_value * right.dn[index]) * scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * right.value + left_value * right.db[index]) * scale; }
        value
    }

    #[inline]
    pub(crate) fn mul_scaled_rhs(left: Self, right: Self, scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value;
        let scaled_right_value = right.value * scale;
        value.value = left_value * scaled_right_value;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * right.value + left_value * right.dn[index]) * scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * right.value + left_value * right.db[index]) * scale; }
        value
    }

    #[inline]
    pub(crate) fn mul_scaled_output(left: Self, right: Self, scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value;
        let product = left_value * right.value;
        value.value = product * scale;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * right.value + left_value * right.dn[index]) * scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * right.value + left_value * right.db[index]) * scale; }
        value
    }

    #[inline]
    pub(crate) fn mul_offset_lhs(left: Self, offset: f64, right: Self) -> Self {
        let mut value = left;
        let left_value = value.value + offset;
        let right_value = right.value;
        value.value = left_value * right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * right_value + left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * right_value + left_value * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn mul_offset_rhs(left: Self, right: Self, offset: f64) -> Self {
        let mut value = left;
        let left_value = value.value;
        let right_value = right.value + offset;
        value.value = left_value * right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * right_value + left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * right_value + left_value * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn mul_offset_lhs_scaled_output(left: Self, offset: f64, right: Self, scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value + offset;
        let right_value = right.value;
        let scaled_left_value = left_value * scale;
        let scaled_right_value = right_value * scale;
        value.value = scaled_left_value * right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * scaled_right_value + scaled_left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * scaled_right_value + scaled_left_value * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn mul_offset_rhs_scaled_output(left: Self, right: Self, offset: f64, scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value;
        let right_value = right.value + offset;
        let scaled_left_value = left_value * scale;
        let scaled_right_value = right_value * scale;
        value.value = left_value * scaled_right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * scaled_right_value + scaled_left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * scaled_right_value + scaled_left_value * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn mul_sub_from_scalar_lhs(scalar: f64, value: Self, right: Self) -> Self {
        let mut result = value;
        let left_value = scalar - result.value;
        result.value = left_value * right.value;
        for index in 0..NODE_COUNT { result.dn[index] = -result.dn[index] * right.value + left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { result.db[index] = -result.db[index] * right.value + left_value * right.db[index]; }
        result
    }

    #[inline]
    pub(crate) fn mul_sub_from_scalar_rhs(left: Self, scalar: f64, value: Self) -> Self {
        let mut result = left;
        let left_value = result.value;
        let right_value = scalar - value.value;
        result.value = left_value * right_value;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * right_value - left_value * value.dn[index]; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * right_value - left_value * value.db[index]; }
        result
    }

    #[inline]
    pub(crate) fn mul_sub_from_scalar_lhs_scaled_output(scalar: f64, value: Self, right: Self, scale: f64) -> Self {
        let mut result = value;
        let left_value = scalar - result.value;
        let scaled_left_value = left_value * scale;
        let scaled_right_value = right.value * scale;
        result.value = scaled_left_value * right.value;
        for index in 0..NODE_COUNT { result.dn[index] = -result.dn[index] * scaled_right_value + scaled_left_value * right.dn[index]; }
        for index in 0..BRANCH_COUNT { result.db[index] = -result.db[index] * scaled_right_value + scaled_left_value * right.db[index]; }
        result
    }

    #[inline]
    pub(crate) fn mul_sub_from_scalar_rhs_scaled_output(left: Self, scalar: f64, value: Self, scale: f64) -> Self {
        let mut result = left;
        let left_value = result.value;
        let right_value = scalar - value.value;
        let scaled_left_value = left_value * scale;
        let scaled_right_value = right_value * scale;
        result.value = left_value * scaled_right_value;
        for index in 0..NODE_COUNT { result.dn[index] = result.dn[index] * scaled_right_value - scaled_left_value * value.dn[index]; }
        for index in 0..BRANCH_COUNT { result.db[index] = result.db[index] * scaled_right_value - scaled_left_value * value.db[index]; }
        result
    }

    #[inline]
    pub(crate) fn mul_sub_from_scalar_scaled_offset_self(scalar: f64, value: Self, input_scale: f64, offset: f64, output_scale: f64) -> Self {
        let mut result = value;
        let sub_value = scalar - result.value;
        let affine_value = sub_value * input_scale + offset;
        result.value = sub_value * affine_value * output_scale;
        let derivative_scale = -((2.0 * input_scale * sub_value + offset) * output_scale);
        for derivative in &mut result.dn { *derivative *= derivative_scale; }
        for derivative in &mut result.db { *derivative *= derivative_scale; }
        result
    }

    #[inline]
    pub(crate) fn mul3(left: Self, middle: Self, right: Self) -> Self {
        let mut value = left;
        let left_value = value.value;
        let middle_value = middle.value;
        let right_value = right.value;
        let left_middle_value = left_value * middle_value;
        let left_right_value = left_value * right_value;
        let middle_right_value = middle_value * right_value;
        value.value = left_middle_value * right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * middle_right_value + middle.dn[index] * left_right_value + right.dn[index] * left_middle_value; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * middle_right_value + middle.db[index] * left_right_value + right.db[index] * left_middle_value; }
        value
    }

    #[inline]
    pub(crate) fn mul3_scaled_output(left: Self, middle: Self, right: Self, scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value;
        let middle_value = middle.value;
        let right_value = right.value;
        let left_middle_value = left_value * middle_value;
        let left_right_value = left_value * right_value;
        let middle_right_value = middle_value * right_value;
        let scaled_left_middle_value = left_middle_value * scale;
        let scaled_left_right_value = left_right_value * scale;
        let scaled_middle_right_value = middle_right_value * scale;
        value.value = scaled_left_middle_value * right_value;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * scaled_middle_right_value + middle.dn[index] * scaled_left_right_value + right.dn[index] * scaled_left_middle_value; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * scaled_middle_right_value + middle.db[index] * scaled_left_right_value + right.db[index] * scaled_left_middle_value; }
        value
    }

    #[inline]
    pub(crate) fn square(arg: Self) -> Self {
        let mut value = arg;
        let raw = value.value;
        value.value = raw * raw;
        let derivative_scale = 2.0 * raw;
        for derivative in &mut value.dn { *derivative *= derivative_scale; }
        for derivative in &mut value.db { *derivative *= derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div(left: Self, right: Self) -> Self {
        let mut value = left;
        let left_value = value.value;
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * reciprocal + right.dn[index] * right_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * reciprocal + right.db[index] * right_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_inputs(left: Self, left_scale: f64, right: Self, right_scale: f64) -> Self {
        let mut value = left;
        let left_value = value.value * left_scale;
        let right_value = right.value * right_scale;
        let reciprocal = 1.0 / right_value;
        let quotient = left_value * reciprocal;
        let left_derivative_scale = left_scale * reciprocal;
        let right_derivative_scale = -quotient * reciprocal * right_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = value.dn[index] * left_derivative_scale + right.dn[index] * right_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = value.db[index] * left_derivative_scale + right.db[index] * right_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product(product_left: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_right_value = product_right.value;
        let denominator_value = denominator.value * denominator_scale;
        let reciprocal = 1.0 / denominator_value;
        let product_value = product_left_value * product_right_value;
        let scaled_product_value = product_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_derivative_scale + denominator.dn[index] * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_derivative_scale + denominator.db[index] * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product_by_product(product_left: Self, product_right: Self, product_scale: f64, denominator_left: Self, denominator_right: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_right_value = product_right.value;
        let denominator_left_value = denominator_left.value;
        let denominator_right_value = denominator_right.value;
        let reciprocal = 1.0 / (denominator_left_value * denominator_right_value * denominator_scale);
        let product_value = product_left_value * product_right_value;
        let scaled_product_value = product_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_derivative_scale + (denominator_left.dn[index] * denominator_right_value + denominator_left_value * denominator_right.dn[index]) * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_derivative_scale + (denominator_left.db[index] * denominator_right_value + denominator_left_value * denominator_right.db[index]) * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product_offset_lhs(product_left: Self, product_left_offset: f64, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value + product_left_offset;
        let product_right_value = product_right.value;
        let denominator_value = denominator.value * denominator_scale;
        let reciprocal = 1.0 / denominator_value;
        let product_value = product_left_value * product_right_value;
        let scaled_product_value = product_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_derivative_scale + denominator.dn[index] * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_derivative_scale + denominator.db[index] * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product_offset_rhs(product_left: Self, product_right: Self, product_right_offset: f64, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_right_value = product_right.value + product_right_offset;
        let denominator_value = denominator.value * denominator_scale;
        let reciprocal = 1.0 / denominator_value;
        let product_value = product_left_value * product_right_value;
        let scaled_product_value = product_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_derivative_scale + denominator.dn[index] * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_derivative_scale + denominator.db[index] * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product_offset_denominator(product_left: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_offset: f64, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_right_value = product_right.value;
        let denominator_value = (denominator.value + denominator_offset) * denominator_scale;
        let reciprocal = 1.0 / denominator_value;
        let product_value = product_left_value * product_right_value;
        let scaled_product_value = product_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * product_right_value + product_left_value * product_right.dn[index]) * product_derivative_scale + denominator.dn[index] * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * product_right_value + product_left_value * product_right.db[index]) * product_derivative_scale + denominator.db[index] * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product3(product_left: Self, product_middle: Self, product_right: Self, product_scale: f64, denominator: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_middle_value = product_middle.value;
        let product_right_value = product_right.value;
        let denominator_value = denominator.value * denominator_scale;
        let reciprocal = 1.0 / denominator_value;
        let left_middle_value = product_left_value * product_middle_value;
        let left_right_value = product_left_value * product_right_value;
        let middle_right_value = product_middle_value * product_right_value;
        let scaled_product_value = left_middle_value * product_right_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * middle_right_value + product_middle.dn[index] * left_right_value + product_right.dn[index] * left_middle_value) * product_derivative_scale + denominator.dn[index] * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * middle_right_value + product_middle.db[index] * left_right_value + product_right.db[index] * left_middle_value) * product_derivative_scale + denominator.db[index] * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn div_scaled_product3_by_product(product_left: Self, product_middle: Self, product_right: Self, product_scale: f64, denominator_left: Self, denominator_right: Self, denominator_scale: f64) -> Self {
        let mut value = product_left;
        let product_left_value = value.value;
        let product_middle_value = product_middle.value;
        let product_right_value = product_right.value;
        let denominator_left_value = denominator_left.value;
        let denominator_right_value = denominator_right.value;
        let reciprocal = 1.0 / (denominator_left_value * denominator_right_value * denominator_scale);
        let left_middle_value = product_left_value * product_middle_value;
        let left_right_value = product_left_value * product_right_value;
        let middle_right_value = product_middle_value * product_right_value;
        let scaled_product_value = left_middle_value * product_right_value * product_scale;
        let quotient = scaled_product_value * reciprocal;
        let product_derivative_scale = product_scale * reciprocal;
        let denominator_derivative_scale = -quotient * reciprocal * denominator_scale;
        value.value = quotient;
        for index in 0..NODE_COUNT { value.dn[index] = (value.dn[index] * middle_right_value + product_middle.dn[index] * left_right_value + product_right.dn[index] * left_middle_value) * product_derivative_scale + (denominator_left.dn[index] * denominator_right_value + denominator_left_value * denominator_right.dn[index]) * denominator_derivative_scale; }
        for index in 0..BRANCH_COUNT { value.db[index] = (value.db[index] * middle_right_value + product_middle.db[index] * left_right_value + product_right.db[index] * left_middle_value) * product_derivative_scale + (denominator_left.db[index] * denominator_right_value + denominator_left_value * denominator_right.db[index]) * denominator_derivative_scale; }
        value
    }

    #[inline]
    pub(crate) fn rem(left: Self, right: Self) -> Self {
        let quotient = (left.value / right.value).trunc();
        let mut value = left;
        value.value %= right.value;
        for index in 0..NODE_COUNT { value.dn[index] -= quotient * right.dn[index]; }
        for index in 0..BRANCH_COUNT { value.db[index] -= quotient * right.db[index]; }
        value
    }

    #[inline]
    pub(crate) fn rem_with_scalar(left: Self, right: f64) -> Self {
        let mut value = left;
        value.value %= right;
        value
    }

    #[inline]
    pub(crate) fn rem_from_scalar(left: f64, right: Self) -> Self {
        let mut value = right;
        let right_value = value.value;
        let quotient = (left / right_value).trunc();
        value.value = left % right_value;
        for derivative in &mut value.dn { *derivative *= -quotient; }
        for derivative in &mut value.db { *derivative *= -quotient; }
        value
    }

    #[inline]
    pub(crate) fn div_from_scalar(scalar: f64, right: Self) -> Self {
        let mut value = right;
        let reciprocal = 1.0 / value.value;
        let quotient = scalar * reciprocal;
        let right_scale = -quotient * reciprocal;
        value.value = quotient;
        for derivative in &mut value.dn { *derivative *= right_scale; }
        for derivative in &mut value.db { *derivative *= right_scale; }
        value
    }

    #[inline]
    pub(crate) fn scale(mut value: Self, scale: f64) -> Self {
        value.value *= scale;
        for derivative in &mut value.dn { *derivative *= scale; }
        for derivative in &mut value.db { *derivative *= scale; }
        value
    }

    #[inline]
    pub(crate) fn scale_offset(mut value: Self, scale: f64, offset: f64) -> Self {
        value.value = value.value * scale + offset;
        for derivative in &mut value.dn { *derivative *= scale; }
        for derivative in &mut value.db { *derivative *= scale; }
        value
    }

    #[inline]
    pub(crate) fn scaled_offset(mut value: Self, offset: f64, scale: f64) -> Self {
        value.value = (value.value + offset) * scale;
        for derivative in &mut value.dn { *derivative *= scale; }
        for derivative in &mut value.db { *derivative *= scale; }
        value
    }

    #[inline]
    pub(crate) fn offset(mut value: Self, offset: f64) -> Self {
        value.value += offset;
        value
    }

    #[inline]
    pub(crate) fn sub_from_scalar(scalar: f64, mut value: Self) -> Self {
        value.value = scalar - value.value;
        for derivative in &mut value.dn { *derivative = -*derivative; }
        for derivative in &mut value.db { *derivative = -*derivative; }
        value
    }

    #[inline]
    pub(crate) fn unary_intrinsic(mut arg: Self, value: f64, derivative_scale: f64) -> Self {
        arg.value = value;
        for derivative in &mut arg.dn { *derivative *= derivative_scale; }
        for derivative in &mut arg.db { *derivative *= derivative_scale; }
        arg
    }

    #[inline]
    pub(crate) fn abs(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 }) }
    #[inline]
    pub(crate) fn abs_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { scale } else { -scale }) }
    #[inline]
    pub(crate) fn sqrt(arg: Self) -> Self { let value = arg.value.sqrt(); Self::unary_intrinsic(arg, value, 1.0 / (2.0 * value)) }
    #[inline]
    pub(crate) fn sqrt_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let value = raw.sqrt(); Self::unary_intrinsic(arg, value, scale / (2.0 * value)) }
    #[inline]
    pub(crate) fn exp(arg: Self) -> Self { let value = arg.value.exp(); Self::unary_intrinsic(arg, value, value) }
    #[inline]
    pub(crate) fn exp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) }
    #[inline]
    pub(crate) fn limexp(arg: Self) -> Self { let raw = arg.value; if raw < 80.0 { let value = raw.exp(); Self::unary_intrinsic(arg, value, value) } else { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX) } }
    #[inline]
    pub(crate) fn limexp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; if raw < 80.0 { let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) } else { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + (raw - 80.0)), LIMEXP_MAX * scale) } }
    #[inline]
    pub(crate) fn limited_exp(arg: Self) -> Self { let raw = arg.value; if raw > 80.0 { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX) } else if raw < -80.0 { Self::constant(1.804851387e-35) } else { let value = raw.exp(); Self::unary_intrinsic(arg, value, value) } }
    #[inline]
    pub(crate) fn limited_exp_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; if raw > 80.0 { Self::unary_intrinsic(arg, LIMEXP_MAX * (1.0 + raw - 80.0), LIMEXP_MAX * scale) } else if raw < -80.0 { Self::constant(1.804851387e-35) } else { let value = raw.exp(); Self::unary_intrinsic(arg, value, value * scale) } }
    #[inline]
    pub(crate) fn ln(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.ln(), 1.0 / raw) }
    #[inline]
    pub(crate) fn ln_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; Self::unary_intrinsic(arg, raw.ln(), scale / raw) }
    #[inline]
    pub(crate) fn ln_one_plus_exp_raw(raw: f64) -> (f64, f64) { if raw > 0.0 { (raw + (-raw).exp().ln_1p(), 1.0 / (1.0 + (-raw).exp())) } else { let exp = raw.exp(); (exp.ln_1p(), exp / (1.0 + exp)) } }
    #[inline]
    pub(crate) fn ln_one_plus_exp(arg: Self) -> Self { let raw = arg.value; let (value, derivative_scale) = Self::ln_one_plus_exp_raw(raw); Self::unary_intrinsic(arg, value, derivative_scale) }
    #[inline]
    pub(crate) fn log10(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.log10(), 1.0 / (raw * std::f64::consts::LN_10)) }
    #[inline]
    pub(crate) fn sin(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sin(), raw.cos()) }
    #[inline]
    pub(crate) fn cos(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cos(), -raw.sin()) }
    #[inline]
    pub(crate) fn tan(arg: Self) -> Self { let raw = arg.value; let cos = raw.cos(); Self::unary_intrinsic(arg, raw.tan(), 1.0 / (cos * cos)) }
    #[inline]
    pub(crate) fn atan(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.atan(), 1.0 / (1.0 + raw * raw)) }
    #[inline]
    pub(crate) fn sinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.sinh(), raw.cosh()) }
    #[inline]
    pub(crate) fn cosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.cosh(), raw.sinh()) }
    #[inline]
    pub(crate) fn tanh(arg: Self) -> Self { let raw = arg.value; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), 1.0 / (cosh * cosh)) }
    #[inline]
    pub(crate) fn tanh_scaled_input(arg: Self, scale: f64) -> Self { let raw = arg.value * scale; let cosh = raw.cosh(); Self::unary_intrinsic(arg, raw.tanh(), scale / (cosh * cosh)) }
    #[inline]
    pub(crate) fn asinh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt()) }
    #[inline]
    pub(crate) fn acosh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.acosh(), 1.0 / ((raw - 1.0).sqrt() * (raw + 1.0).sqrt())) }
    #[inline]
    pub(crate) fn atanh(arg: Self) -> Self { let raw = arg.value; Self::unary_intrinsic(arg, raw.atanh(), 1.0 / (1.0 - raw * raw)) }
    #[inline]
    pub(crate) fn floor(arg: Self) -> Self { Self::constant(arg.value.floor()) }
    #[inline]
    pub(crate) fn ceil(arg: Self) -> Self { Self::constant(arg.value.ceil()) }
    #[inline]
    pub(crate) fn pow_derivative(value: f64, base: f64, exponent: f64, dbase: f64, dexponent: f64) -> f64 {
        if dexponent == 0.0 && exponent.is_finite() && exponent.fract() == 0.0 {
            if exponent == 0.0 { 0.0 } else { exponent * base.powf(exponent - 1.0) * dbase }
        } else {
            value * (dexponent * base.ln() + exponent * (dbase / base))
        }
    }
    #[inline]
    pub(crate) fn powf(left: Self, exponent: f64) -> Self {
        let base = left.value;
        let value = base.powf(exponent);
        let mut result = left;
        result.value = value;
        for index in 0..NODE_COUNT { result.dn[index] = Self::pow_derivative(value, base, exponent, result.dn[index], 0.0); }
        for index in 0..BRANCH_COUNT { result.db[index] = Self::pow_derivative(value, base, exponent, result.db[index], 0.0); }
        result
    }
    #[inline]
    pub(crate) fn pow_from_scalar(base: f64, right: Self) -> Self {
        let exponent = right.value;
        let value = base.powf(exponent);
        let mut result = right;
        result.value = value;
        for index in 0..NODE_COUNT { result.dn[index] = Self::pow_derivative(value, base, exponent, 0.0, result.dn[index]); }
        for index in 0..BRANCH_COUNT { result.db[index] = Self::pow_derivative(value, base, exponent, 0.0, result.db[index]); }
        result
    }
    #[inline]
    pub(crate) fn pow(left: Self, right: Self) -> Self {
        let base = left.value;
        let exponent = right.value;
        let value = base.powf(exponent);
        let mut result = left;
        result.value = value;
        for index in 0..NODE_COUNT { result.dn[index] = Self::pow_derivative(value, base, exponent, result.dn[index], right.dn[index]); }
        for index in 0..BRANCH_COUNT { result.db[index] = Self::pow_derivative(value, base, exponent, result.db[index], right.db[index]); }
        result
    }
    #[inline]
    pub(crate) fn min(left: Self, right: Self) -> Self { if left.value <= right.value { left } else { right } }
    #[inline]
    pub(crate) fn min_with_scalar(left: Self, right: f64) -> Self { if left.value <= right { left } else { Self::constant(right) } }
    #[inline]
    pub(crate) fn min_from_scalar(left: f64, right: Self) -> Self { if left <= right.value { Self::constant(left) } else { right } }
    #[inline]
    pub(crate) fn max(left: Self, right: Self) -> Self { if left.value >= right.value { left } else { right } }
    #[inline]
    pub(crate) fn max_with_scalar(left: Self, right: f64) -> Self { if left.value >= right { left } else { Self::constant(right) } }
    #[inline]
    pub(crate) fn max_from_scalar(left: f64, right: Self) -> Self { if left >= right.value { Self::constant(left) } else { right } }
    #[inline]
    pub(crate) fn hypot(left: Self, right: Self) -> Self {
        let left_value = left.value;
        let right_value = right.value;
        let value = left_value.hypot(right_value);
        let mut result = left;
        result.value = value;
        for index in 0..NODE_COUNT { result.dn[index] = (left_value * result.dn[index] + right_value * right.dn[index]) / value; }
        for index in 0..BRANCH_COUNT { result.db[index] = (left_value * result.db[index] + right_value * right.db[index]) / value; }
        result
    }
    #[inline]
    pub(crate) fn atan2(y: Self, x: Self) -> Self {
        let y_value = y.value;
        let x_value = x.value;
        let denominator = x_value * x_value + y_value * y_value;
        let mut result = y;
        result.value = y_value.atan2(x_value);
        for index in 0..NODE_COUNT { result.dn[index] = (x_value * result.dn[index] - y_value * x.dn[index]) / denominator; }
        for index in 0..BRANCH_COUNT { result.db[index] = (x_value * result.db[index] - y_value * x.db[index]) / denominator; }
        result
    }

    #[inline]
    pub(crate) fn ddt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {
        operand.value = value;
        for derivative in &mut operand.dn { *derivative *= derivative_scale; }
        for derivative in &mut operand.db { *derivative *= derivative_scale; }
        operand
    }

    #[inline]
    pub(crate) fn idt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {
        operand.value = value;
        for derivative in &mut operand.dn { *derivative *= derivative_scale; }
        for derivative in &mut operand.db { *derivative *= derivative_scale; }
        operand
    }

    #[inline]
    pub(crate) fn ddx_projection(expr: &Self, pos: Option<usize>, neg: Option<usize>) -> f64 {
        let pos = pos.map(|index| expr.dn[index]).unwrap_or(0.0);
        if let Some(neg) = neg { 0.5 * (pos - expr.dn[neg]) } else { pos }
    }
}

