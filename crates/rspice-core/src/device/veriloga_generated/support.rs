#![allow(dead_code)]

use super::GeneratedEvalContext;

pub(crate) struct Scratch<
    const VARIABLE_COUNT: usize,
    const NODE_COUNT: usize,
    const BRANCH_COUNT: usize,
> {
    pub(crate) v: [f64; VARIABLE_COUNT],
    pub(crate) dn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) db: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
}

impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize>
    Scratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT>
{
    pub(crate) fn new() -> Self {
        Self {
            v: [0.0; VARIABLE_COUNT],
            dn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            db: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
        }
    }

    #[inline]
    pub(crate) fn ad_value(&self, index: usize) -> AdValue<NODE_COUNT, BRANCH_COUNT> {
        AdValue {
            value: self.v[index],
            dn: self.dn[index],
            db: self.db[index],
        }
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
    pub(crate) fn store_ad_value(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn store_add_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::add(left, right));
    }

    #[inline]
    pub(crate) fn store_sub_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::sub(left, right));
    }

    #[inline]
    pub(crate) fn store_mul_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::mul(left, right));
    }

    #[inline]
    pub(crate) fn store_div_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::div(left, right));
    }

    #[inline]
    pub(crate) fn store_rem_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::rem(left, right));
    }

    #[inline]
    pub(crate) fn store_pow_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::pow(left, right));
    }

    #[inline]
    pub(crate) fn store_min_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::min(left, right));
    }

    #[inline]
    pub(crate) fn store_max_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::max(left, right));
    }

    #[inline]
    pub(crate) fn store_hypot_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::hypot(left, right));
    }

    #[inline]
    pub(crate) fn store_atan2_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::atan2(left, right));
    }

    #[inline]
    pub(crate) fn store_scale_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scale: f64,
    ) {
        self.store_ad_value(index, AdValue::scale(value, scale));
    }

    #[inline]
    pub(crate) fn store_offset_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        offset: f64,
    ) {
        self.store_ad_value(index, AdValue::offset(value, offset));
    }

    #[inline]
    pub(crate) fn store_neg_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::neg(value));
    }

    #[inline]
    pub(crate) fn store_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sqrt(value));
    }

    #[inline]
    pub(crate) fn store_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::exp(value));
    }

    #[inline]
    pub(crate) fn store_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ln(value));
    }

    #[inline]
    pub(crate) fn store_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::abs(value));
    }

    #[inline]
    pub(crate) fn store_square_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::square(value));
    }

    #[inline]
    pub(crate) fn store_limexp_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::limexp(value));
    }

    #[inline]
    pub(crate) fn store_limited_exp_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::limited_exp(value));
    }

    #[inline]
    pub(crate) fn store_log10_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
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
    pub(crate) fn store_asinh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::asinh(value));
    }

    #[inline]
    pub(crate) fn store_acosh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::acosh(value));
    }

    #[inline]
    pub(crate) fn store_atanh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::atanh(value));
    }

    #[inline]
    pub(crate) fn store_floor_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::floor(value));
    }

    #[inline]
    pub(crate) fn store_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ceil(value));
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::sub_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::div_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::rem_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_pow_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::pow_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_min_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::min_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_max_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::max_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::rem_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_min_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::min_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_max_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::max_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_powf_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        exponent: f64,
    ) {
        self.store_ad_value(index, AdValue::powf(value, exponent));
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] + right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] + right_db[axis];
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] - right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] - right_db[axis];
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_square(&mut self, index: usize, source: usize) {
        let source_value = self.v[source];
        self.store_unary_scaled(
            index,
            source,
            source_value * source_value,
            2.0 * source_value,
        );
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale;
        }
    }

    #[inline]
    pub(crate) fn store_add_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] + right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] + right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_add_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] + right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] + right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_sub_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] - right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] - right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_sub_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] - right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] - right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_mul_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * right.value + left_value * right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * right.value + left_value * right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_mul_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_div_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * reciprocal + right.dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * reciprocal + right.db[axis] * right_scale;
        }
    }

    #[inline]
    pub(crate) fn store_div_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] * reciprocal + right_dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] * reciprocal + right_db[axis] * right_scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] =
                (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] =
                (left_db[axis] * right_value + left_value * right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] =
                (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] =
                (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale;
        }
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
    pub(crate) fn store_offset_scaled(
        &mut self,
        index: usize,
        source: usize,
        scale: f64,
        offset: f64,
    ) {
        self.store_unary_scaled(index, source, self.v[source] * scale + offset, scale);
    }

    #[inline]
    pub(crate) fn store_scaled_offset(
        &mut self,
        index: usize,
        source: usize,
        offset: f64,
        scale: f64,
    ) {
        self.store_unary_scaled(index, source, (self.v[source] + offset) * scale, scale);
    }

    #[inline]
    pub(crate) fn store_neg(&mut self, index: usize, source: usize) {
        self.store_scale(index, source, -1.0);
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
    pub(crate) fn store_sqrt(&mut self, index: usize, source: usize) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp(&mut self, index: usize, source: usize) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value, value);
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
    pub(crate) fn store_scaled_sqrt(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value * scale, scale / (2.0 * value));
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
        let derivative_scale =
            AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(value, base, exponent, 1.0, 0.0);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_scaled(
        &mut self,
        index: usize,
        source: usize,
        value: f64,
        derivative_scale: f64,
    ) {
        let dn = self.dn[source];
        let db = self.db[source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = derivative_scale * dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = derivative_scale * db[axis];
        }
    }
}

pub(crate) struct ReactiveScratch<
    const VARIABLE_COUNT: usize,
    const NODE_COUNT: usize,
    const BRANCH_COUNT: usize,
> {
    pub(crate) v: [f64; VARIABLE_COUNT],
    pub(crate) dn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) db: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
    pub(crate) rv: [f64; VARIABLE_COUNT],
    pub(crate) rdn: [[f64; NODE_COUNT]; VARIABLE_COUNT],
    pub(crate) rdb: [[f64; BRANCH_COUNT]; VARIABLE_COUNT],
}

impl<const VARIABLE_COUNT: usize, const NODE_COUNT: usize, const BRANCH_COUNT: usize>
    ReactiveScratch<VARIABLE_COUNT, NODE_COUNT, BRANCH_COUNT>
{
    pub(crate) fn new() -> Self {
        Self {
            v: [0.0; VARIABLE_COUNT],
            dn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            db: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
            rv: [0.0; VARIABLE_COUNT],
            rdn: [[0.0; NODE_COUNT]; VARIABLE_COUNT],
            rdb: [[0.0; BRANCH_COUNT]; VARIABLE_COUNT],
        }
    }

    #[inline]
    pub(crate) fn ad_value(&self, index: usize) -> AdValue<NODE_COUNT, BRANCH_COUNT> {
        AdValue {
            value: self.v[index],
            dn: self.dn[index],
            db: self.db[index],
        }
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
    pub(crate) fn store_ad_value(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.v[index] = value.value;
        self.dn[index] = value.dn;
        self.db[index] = value.db;
    }

    #[inline]
    pub(crate) fn store_add_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::add(left, right));
    }

    #[inline]
    pub(crate) fn store_sub_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::sub(left, right));
    }

    #[inline]
    pub(crate) fn store_mul_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::mul(left, right));
    }

    #[inline]
    pub(crate) fn store_div_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::div(left, right));
    }

    #[inline]
    pub(crate) fn store_rem_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::rem(left, right));
    }

    #[inline]
    pub(crate) fn store_pow_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::pow(left, right));
    }

    #[inline]
    pub(crate) fn store_min_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::min(left, right));
    }

    #[inline]
    pub(crate) fn store_max_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::max(left, right));
    }

    #[inline]
    pub(crate) fn store_hypot_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::hypot(left, right));
    }

    #[inline]
    pub(crate) fn store_atan2_ad(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::atan2(left, right));
    }

    #[inline]
    pub(crate) fn store_scale_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scale: f64,
    ) {
        self.store_ad_value(index, AdValue::scale(value, scale));
    }

    #[inline]
    pub(crate) fn store_offset_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        offset: f64,
    ) {
        self.store_ad_value(index, AdValue::offset(value, offset));
    }

    #[inline]
    pub(crate) fn store_neg_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::neg(value));
    }

    #[inline]
    pub(crate) fn store_sqrt_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::sqrt(value));
    }

    #[inline]
    pub(crate) fn store_exp_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::exp(value));
    }

    #[inline]
    pub(crate) fn store_ln_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ln(value));
    }

    #[inline]
    pub(crate) fn store_abs_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::abs(value));
    }

    #[inline]
    pub(crate) fn store_square_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::square(value));
    }

    #[inline]
    pub(crate) fn store_limexp_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::limexp(value));
    }

    #[inline]
    pub(crate) fn store_limited_exp_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::limited_exp(value));
    }

    #[inline]
    pub(crate) fn store_log10_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
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
    pub(crate) fn store_asinh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::asinh(value));
    }

    #[inline]
    pub(crate) fn store_acosh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::acosh(value));
    }

    #[inline]
    pub(crate) fn store_atanh_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::atanh(value));
    }

    #[inline]
    pub(crate) fn store_floor_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::floor(value));
    }

    #[inline]
    pub(crate) fn store_ceil_ad(&mut self, index: usize, value: AdValue<NODE_COUNT, BRANCH_COUNT>) {
        self.store_ad_value(index, AdValue::ceil(value));
    }

    #[inline]
    pub(crate) fn store_sub_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::sub_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_div_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::div_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::rem_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_pow_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::pow_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_min_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::min_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_max_from_scalar_ad(
        &mut self,
        index: usize,
        scalar: f64,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        self.store_ad_value(index, AdValue::max_from_scalar(scalar, value));
    }

    #[inline]
    pub(crate) fn store_rem_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::rem_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_min_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::min_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_max_with_scalar_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        scalar: f64,
    ) {
        self.store_ad_value(index, AdValue::max_with_scalar(value, scalar));
    }

    #[inline]
    pub(crate) fn store_powf_ad(
        &mut self,
        index: usize,
        value: AdValue<NODE_COUNT, BRANCH_COUNT>,
        exponent: f64,
    ) {
        self.store_ad_value(index, AdValue::powf(value, exponent));
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] + right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] + right_db[axis];
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] - right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] - right_db[axis];
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * right_value + left_value * right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * right_value + left_value * right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_square(&mut self, index: usize, source: usize) {
        let source_value = self.v[source];
        self.store_unary_scaled(
            index,
            source,
            source_value * source_value,
            2.0 * source_value,
        );
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * reciprocal + right_dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * reciprocal + right_db[axis] * right_scale;
        }
    }

    #[inline]
    pub(crate) fn store_add_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value + right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] + right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] + right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_add_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value + right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] + right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] + right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_sub_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value - right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] - right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] - right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_sub_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value - right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] - right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] - right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_mul_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        self.v[index] = left_value * right.value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * right.value + left_value * right.dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * right.value + left_value * right.db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_mul_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        self.v[index] = left.value * right_value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] * right_value + left.value * right_dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] * right_value + left.value * right_db[axis];
        }
    }

    #[inline]
    pub(crate) fn store_div_ad_rhs(
        &mut self,
        index: usize,
        left: usize,
        right: AdValue<NODE_COUNT, BRANCH_COUNT>,
    ) {
        let left_value = self.v[left];
        let left_dn = self.dn[left];
        let left_db = self.db[left];
        let reciprocal = 1.0 / right.value;
        let quotient = left_value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left_dn[axis] * reciprocal + right.dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left_db[axis] * reciprocal + right.db[axis] * right_scale;
        }
    }

    #[inline]
    pub(crate) fn store_div_ad_lhs(
        &mut self,
        index: usize,
        left: AdValue<NODE_COUNT, BRANCH_COUNT>,
        right: usize,
    ) {
        let right_value = self.v[right];
        let right_dn = self.dn[right];
        let right_db = self.db[right];
        let reciprocal = 1.0 / right_value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        self.v[index] = quotient;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = left.dn[axis] * reciprocal + right_dn[axis] * right_scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = left.db[axis] * reciprocal + right_db[axis] * right_scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = (left_dn[axis] + right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = (left_db[axis] + right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = (left_dn[axis] - right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = (left_db[axis] - right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] =
                (left_dn[axis] * right_value + left_value * right_dn[axis]) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] =
                (left_db[axis] * right_value + left_value * right_db[axis]) * scale;
        }
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
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] =
                (left_dn[axis] * reciprocal + right_dn[axis] * right_scale) * scale;
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] =
                (left_db[axis] * reciprocal + right_db[axis] * right_scale) * scale;
        }
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
    pub(crate) fn store_offset_scaled(
        &mut self,
        index: usize,
        source: usize,
        scale: f64,
        offset: f64,
    ) {
        self.store_unary_scaled(index, source, self.v[source] * scale + offset, scale);
    }

    #[inline]
    pub(crate) fn store_scaled_offset(
        &mut self,
        index: usize,
        source: usize,
        offset: f64,
        scale: f64,
    ) {
        self.store_unary_scaled(index, source, (self.v[source] + offset) * scale, scale);
    }

    #[inline]
    pub(crate) fn store_neg(&mut self, index: usize, source: usize) {
        self.store_scale(index, source, -1.0);
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
    pub(crate) fn store_sqrt(&mut self, index: usize, source: usize) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value, 1.0 / (2.0 * value));
    }

    #[inline]
    pub(crate) fn store_exp(&mut self, index: usize, source: usize) {
        let value = self.v[source].exp();
        self.store_unary_scaled(index, source, value, value);
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
    pub(crate) fn store_scaled_sqrt(&mut self, index: usize, source: usize, scale: f64) {
        let value = self.v[source].sqrt();
        self.store_unary_scaled(index, source, value * scale, scale / (2.0 * value));
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
        let derivative_scale =
            AdValue::<NODE_COUNT, BRANCH_COUNT>::pow_derivative(value, base, exponent, 1.0, 0.0);
        self.store_unary_scaled(index, source, value, derivative_scale);
    }

    #[inline]
    pub(crate) fn store_unary_scaled(
        &mut self,
        index: usize,
        source: usize,
        value: f64,
        derivative_scale: f64,
    ) {
        let dn = self.dn[source];
        let db = self.db[source];
        self.v[index] = value;
        for axis in 0..NODE_COUNT {
            self.dn[index][axis] = derivative_scale * dn[axis];
        }
        for axis in 0..BRANCH_COUNT {
            self.db[index][axis] = derivative_scale * db[axis];
        }
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
        Self {
            value,
            dn: [0.0; NODE_COUNT],
            db: [0.0; BRANCH_COUNT],
        }
    }
    #[inline]
    pub(crate) fn voltage(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; NODE_COUNT],
        pos: Option<usize>,
        neg: Option<usize>,
    ) -> Self {
        let pos_value = pos
            .map(|index| ctx.node_voltage(nodes[index]))
            .unwrap_or(0.0);
        let neg_value = neg
            .map(|index| ctx.node_voltage(nodes[index]))
            .unwrap_or(0.0);
        let mut value = Self::constant(pos_value - neg_value);
        if let Some(index) = pos {
            value.dn[index] += 1.0;
        }
        if let Some(index) = neg {
            value.dn[index] -= 1.0;
        }
        value
    }

    #[inline]
    pub(crate) fn branch_current(
        ctx: &GeneratedEvalContext<'_>,
        branches: &[usize; BRANCH_COUNT],
        slot: usize,
    ) -> Self {
        let mut value = Self::constant(ctx.branch_current(branches[slot]));
        value.db[slot] = 1.0;
        value
    }

    #[inline]
    pub(crate) fn neg(mut value: Self) -> Self {
        value.value = -value.value;
        for derivative in &mut value.dn {
            *derivative = -*derivative;
        }
        for derivative in &mut value.db {
            *derivative = -*derivative;
        }
        value
    }

    #[inline]
    pub(crate) fn add(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value + right.value);
        for index in 0..NODE_COUNT {
            value.dn[index] = left.dn[index] + right.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = left.db[index] + right.db[index];
        }
        value
    }

    #[inline]
    pub(crate) fn sub(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value - right.value);
        for index in 0..NODE_COUNT {
            value.dn[index] = left.dn[index] - right.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = left.db[index] - right.db[index];
        }
        value
    }

    #[inline]
    pub(crate) fn mul(left: Self, right: Self) -> Self {
        let mut value = Self::constant(left.value * right.value);
        for index in 0..NODE_COUNT {
            value.dn[index] = left.dn[index] * right.value + left.value * right.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = left.db[index] * right.value + left.value * right.db[index];
        }
        value
    }

    #[inline]
    pub(crate) fn square(arg: Self) -> Self {
        let mut value = Self::constant(arg.value * arg.value);
        let derivative_scale = 2.0 * arg.value;
        for index in 0..NODE_COUNT {
            value.dn[index] = derivative_scale * arg.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = derivative_scale * arg.db[index];
        }
        value
    }

    #[inline]
    pub(crate) fn div(left: Self, right: Self) -> Self {
        let reciprocal = 1.0 / right.value;
        let quotient = left.value * reciprocal;
        let right_scale = -quotient * reciprocal;
        let mut value = Self::constant(quotient);
        for index in 0..NODE_COUNT {
            value.dn[index] = left.dn[index] * reciprocal + right.dn[index] * right_scale;
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = left.db[index] * reciprocal + right.db[index] * right_scale;
        }
        value
    }

    #[inline]
    pub(crate) fn rem(left: Self, right: Self) -> Self {
        let quotient = (left.value / right.value).trunc();
        let mut value = Self::constant(left.value % right.value);
        for index in 0..NODE_COUNT {
            value.dn[index] = left.dn[index] - quotient * right.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = left.db[index] - quotient * right.db[index];
        }
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
        let quotient = (left / right.value).trunc();
        let mut value = Self::constant(left % right.value);
        for index in 0..NODE_COUNT {
            value.dn[index] = -quotient * right.dn[index];
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = -quotient * right.db[index];
        }
        value
    }

    #[inline]
    pub(crate) fn div_from_scalar(scalar: f64, right: Self) -> Self {
        let reciprocal = 1.0 / right.value;
        let quotient = scalar * reciprocal;
        let right_scale = -quotient * reciprocal;
        let mut value = Self::constant(quotient);
        for index in 0..NODE_COUNT {
            value.dn[index] = right.dn[index] * right_scale;
        }
        for index in 0..BRANCH_COUNT {
            value.db[index] = right.db[index] * right_scale;
        }
        value
    }

    #[inline]
    pub(crate) fn scale(mut value: Self, scale: f64) -> Self {
        value.value *= scale;
        for derivative in &mut value.dn {
            *derivative *= scale;
        }
        for derivative in &mut value.db {
            *derivative *= scale;
        }
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
        for derivative in &mut value.dn {
            *derivative = -*derivative;
        }
        for derivative in &mut value.db {
            *derivative = -*derivative;
        }
        value
    }

    #[inline]
    pub(crate) fn unary_intrinsic(mut arg: Self, value: f64, derivative_scale: f64) -> Self {
        arg.value = value;
        for derivative in &mut arg.dn {
            *derivative *= derivative_scale;
        }
        for derivative in &mut arg.db {
            *derivative *= derivative_scale;
        }
        arg
    }

    #[inline]
    pub(crate) fn abs(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.abs(), if raw >= 0.0 { 1.0 } else { -1.0 })
    }
    #[inline]
    pub(crate) fn sqrt(arg: Self) -> Self {
        let value = arg.value.sqrt();
        Self::unary_intrinsic(arg, value, 1.0 / (2.0 * value))
    }
    #[inline]
    pub(crate) fn exp(arg: Self) -> Self {
        let value = arg.value.exp();
        Self::unary_intrinsic(arg, value, value)
    }
    #[inline]
    pub(crate) fn limexp(arg: Self) -> Self {
        let raw = arg.value;
        if raw < 80.0 {
            let value = raw.exp();
            Self::unary_intrinsic(arg, value, value)
        } else {
            let scale = 80.0_f64.exp();
            Self::unary_intrinsic(arg, scale * (1.0 + (raw - 80.0)), scale)
        }
    }
    #[inline]
    pub(crate) fn limited_exp(arg: Self) -> Self {
        let raw = arg.value;
        if raw > 80.0 {
            Self::unary_intrinsic(arg, 5.540622384e34 * (1.0 + raw - 80.0), 5.540622384e34)
        } else if raw < -80.0 {
            Self::constant(1.804851387e-35)
        } else {
            let value = raw.exp();
            Self::unary_intrinsic(arg, value, value)
        }
    }
    #[inline]
    pub(crate) fn ln(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.ln(), 1.0 / raw)
    }
    #[inline]
    pub(crate) fn log10(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.log10(), 1.0 / (raw * std::f64::consts::LN_10))
    }
    #[inline]
    pub(crate) fn sin(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.sin(), raw.cos())
    }
    #[inline]
    pub(crate) fn cos(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.cos(), -raw.sin())
    }
    #[inline]
    pub(crate) fn tan(arg: Self) -> Self {
        let raw = arg.value;
        let cos = raw.cos();
        Self::unary_intrinsic(arg, raw.tan(), 1.0 / (cos * cos))
    }
    #[inline]
    pub(crate) fn atan(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.atan(), 1.0 / (1.0 + raw * raw))
    }
    #[inline]
    pub(crate) fn sinh(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.sinh(), raw.cosh())
    }
    #[inline]
    pub(crate) fn cosh(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.cosh(), raw.sinh())
    }
    #[inline]
    pub(crate) fn tanh(arg: Self) -> Self {
        let raw = arg.value;
        let cosh = raw.cosh();
        Self::unary_intrinsic(arg, raw.tanh(), 1.0 / (cosh * cosh))
    }
    #[inline]
    pub(crate) fn asinh(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.asinh(), 1.0 / ((raw * raw) + 1.0).sqrt())
    }
    #[inline]
    pub(crate) fn acosh(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(
            arg,
            raw.acosh(),
            1.0 / ((raw - 1.0).sqrt() * (raw + 1.0).sqrt()),
        )
    }
    #[inline]
    pub(crate) fn atanh(arg: Self) -> Self {
        let raw = arg.value;
        Self::unary_intrinsic(arg, raw.atanh(), 1.0 / (1.0 - raw * raw))
    }
    #[inline]
    pub(crate) fn floor(arg: Self) -> Self {
        Self::constant(arg.value.floor())
    }
    #[inline]
    pub(crate) fn ceil(arg: Self) -> Self {
        Self::constant(arg.value.ceil())
    }
    #[inline]
    pub(crate) fn pow_derivative(
        value: f64,
        base: f64,
        exponent: f64,
        dbase: f64,
        dexponent: f64,
    ) -> f64 {
        if dexponent == 0.0 && exponent.is_finite() && exponent.fract() == 0.0 {
            if exponent == 0.0 {
                0.0
            } else {
                exponent * base.powf(exponent - 1.0) * dbase
            }
        } else {
            value * (dexponent * base.ln() + exponent * (dbase / base))
        }
    }
    #[inline]
    pub(crate) fn powf(left: Self, exponent: f64) -> Self {
        let value = left.value.powf(exponent);
        let mut result = Self::constant(value);
        for index in 0..NODE_COUNT {
            result.dn[index] =
                Self::pow_derivative(value, left.value, exponent, left.dn[index], 0.0);
        }
        for index in 0..BRANCH_COUNT {
            result.db[index] =
                Self::pow_derivative(value, left.value, exponent, left.db[index], 0.0);
        }
        result
    }
    #[inline]
    pub(crate) fn pow_from_scalar(base: f64, right: Self) -> Self {
        let value = base.powf(right.value);
        let mut result = Self::constant(value);
        for index in 0..NODE_COUNT {
            result.dn[index] = Self::pow_derivative(value, base, right.value, 0.0, right.dn[index]);
        }
        for index in 0..BRANCH_COUNT {
            result.db[index] = Self::pow_derivative(value, base, right.value, 0.0, right.db[index]);
        }
        result
    }
    #[inline]
    pub(crate) fn pow(left: Self, right: Self) -> Self {
        let value = left.value.powf(right.value);
        let mut result = Self::constant(value);
        for index in 0..NODE_COUNT {
            result.dn[index] = Self::pow_derivative(
                value,
                left.value,
                right.value,
                left.dn[index],
                right.dn[index],
            );
        }
        for index in 0..BRANCH_COUNT {
            result.db[index] = Self::pow_derivative(
                value,
                left.value,
                right.value,
                left.db[index],
                right.db[index],
            );
        }
        result
    }
    #[inline]
    pub(crate) fn min(left: Self, right: Self) -> Self {
        if left.value <= right.value {
            left
        } else {
            right
        }
    }
    #[inline]
    pub(crate) fn min_with_scalar(left: Self, right: f64) -> Self {
        if left.value <= right {
            left
        } else {
            Self::constant(right)
        }
    }
    #[inline]
    pub(crate) fn min_from_scalar(left: f64, right: Self) -> Self {
        if left <= right.value {
            Self::constant(left)
        } else {
            right
        }
    }
    #[inline]
    pub(crate) fn max(left: Self, right: Self) -> Self {
        if left.value >= right.value {
            left
        } else {
            right
        }
    }
    #[inline]
    pub(crate) fn max_with_scalar(left: Self, right: f64) -> Self {
        if left.value >= right {
            left
        } else {
            Self::constant(right)
        }
    }
    #[inline]
    pub(crate) fn max_from_scalar(left: f64, right: Self) -> Self {
        if left >= right.value {
            Self::constant(left)
        } else {
            right
        }
    }
    #[inline]
    pub(crate) fn hypot(left: Self, right: Self) -> Self {
        let value = left.value.hypot(right.value);
        let mut result = Self::constant(value);
        for index in 0..NODE_COUNT {
            result.dn[index] =
                (left.value * left.dn[index] + right.value * right.dn[index]) / value;
        }
        for index in 0..BRANCH_COUNT {
            result.db[index] =
                (left.value * left.db[index] + right.value * right.db[index]) / value;
        }
        result
    }
    #[inline]
    pub(crate) fn atan2(y: Self, x: Self) -> Self {
        let denominator = x.value * x.value + y.value * y.value;
        let mut result = Self::constant(y.value.atan2(x.value));
        for index in 0..NODE_COUNT {
            result.dn[index] = (x.value * y.dn[index] - y.value * x.dn[index]) / denominator;
        }
        for index in 0..BRANCH_COUNT {
            result.db[index] = (x.value * y.db[index] - y.value * x.db[index]) / denominator;
        }
        result
    }

    #[inline]
    pub(crate) fn ddt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {
        operand.value = value;
        for derivative in &mut operand.dn {
            *derivative *= derivative_scale;
        }
        for derivative in &mut operand.db {
            *derivative *= derivative_scale;
        }
        operand
    }

    #[inline]
    pub(crate) fn idt(mut operand: Self, derivative_scale: f64, value: f64) -> Self {
        operand.value = value;
        for derivative in &mut operand.dn {
            *derivative *= derivative_scale;
        }
        for derivative in &mut operand.db {
            *derivative *= derivative_scale;
        }
        operand
    }

    #[inline]
    pub(crate) fn ddx_projection(expr: &Self, pos: Option<usize>, neg: Option<usize>) -> f64 {
        let pos = pos.map(|index| expr.dn[index]).unwrap_or(0.0);
        if let Some(neg) = neg {
            0.5 * (pos - expr.dn[neg])
        } else {
            pos
        }
    }
}
