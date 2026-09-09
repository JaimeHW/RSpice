//! Inexpensive response enclosures and bounded two-component recovery.
use super::SmallExact;

#[derive(Clone, Copy)]
struct Complex64 {
    re: f64,
    im: f64,
}
impl Complex64 {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

#[derive(Clone, Copy)]
struct BoundedComplex {
    value: Complex64,
    error: Complex64,
}

fn ordinary_product(a: f64, b: f64) -> Option<f64> {
    let value = a * b;
    value.is_finite().then_some(value)
}

// Normal products retain the final shared outward inflation; subnormal
// positive bounds round up explicitly rather than vanishing.
fn error_product(a: f64, b: f64) -> Option<f64> {
    let value = a * b;
    if !value.is_finite() {
        return None;
    }
    Some(if value.is_normal() || a == 0.0 || b == 0.0 {
        value
    } else {
        value.next_up()
    })
}
fn product_floor(a: f64, b: f64, value: f64) -> f64 {
    if !value.is_normal() && a != 0.0 && b != 0.0 {
        f64::from_bits(1)
    } else {
        0.0
    }
}
fn error_bound(scale: f64, propagated: f64) -> Option<f64> {
    let unit_roundoff = 0.5 * f64::EPSILON * (1.0 + f64::EPSILON);
    let rounding = error_product(unit_roundoff, scale)?;
    let value = rounding + propagated;
    let bound = if value.is_normal() || value == 0.0 {
        value * (1.0 + 16.0 * f64::EPSILON)
    } else {
        value.next_up()
    };
    bound.is_finite().then_some(bound)
}

fn bounded_polynomial(
    coefficients: &[f64],
    z: Complex64,
    z_error: Complex64,
) -> Option<BoundedComplex> {
    let (&leading, remaining) = coefficients.split_last()?;
    let mut value = Complex64::new(leading, 0.0);
    let mut error = Complex64::new(0.0, 0.0);
    for &coefficient in remaining.iter().rev() {
        let p = ordinary_product(value.re, z.re)?;
        let q = ordinary_product(value.im, z.im)?;
        let r = ordinary_product(value.re, z.im)?;
        let s = ordinary_product(value.im, z.re)?;
        let difference = p - q;
        let real = difference + coefficient;
        let imaginary = r + s;
        if ![difference, real, imaginary]
            .into_iter()
            .all(|v| v.is_finite())
        {
            return None;
        }
        error = Complex64::new(
            error_bound(
                p.abs() + q.abs() + difference.abs() + real.abs(),
                product_floor(value.re, z.re, p)
                    + product_floor(value.im, z.im, q)
                    + product_error(value.re, error.re, z.re, z_error.re)?
                    + product_error(value.im, error.im, z.im, z_error.im)?,
            )?,
            error_bound(
                r.abs() + s.abs() + imaginary.abs(),
                product_floor(value.re, z.im, r)
                    + product_floor(value.im, z.re, s)
                    + product_error(value.re, error.re, z.im, z_error.im)?
                    + product_error(value.im, error.im, z.re, z_error.re)?,
            )?,
        );
        value = Complex64::new(real, imaginary);
    }
    Some(BoundedComplex { value, error })
}

fn bounded_quotient(n: BoundedComplex, d: BoundedComplex, tolerance: f64) -> Option<Complex64> {
    let p = ordinary_product(d.value.re, d.value.re)?;
    let q = ordinary_product(d.value.im, d.value.im)?;
    let norm = p + q;
    let norm_error = error_bound(
        p.abs() + q.abs() + norm.abs(),
        product_floor(d.value.re, d.value.re, p)
            + product_floor(d.value.im, d.value.im, q)
            + product_error(d.value.re, d.error.re, d.value.re, d.error.re)?
            + product_error(d.value.im, d.error.im, d.value.im, d.error.im)?,
    )?;
    let lower_norm = (norm - norm_error) * (1.0 - 4.0 * f64::EPSILON);
    if !lower_norm.is_normal() || lower_norm <= 0.0 || !norm.is_finite() {
        return None;
    }
    let component = |a: f64, ea: f64, b: f64, eb: f64, c: f64, ec: f64, e: f64, ee: f64| {
        let left = ordinary_product(a, b)?;
        let right = ordinary_product(c, e)?;
        let numerator = left + right;
        let numerator_error = error_bound(
            left.abs() + right.abs() + numerator.abs(),
            product_floor(a, b, left)
                + product_floor(c, e, right)
                + product_error(a, ea, b, eb)?
                + product_error(c, ec, e, ee)?,
        )?;
        let result = numerator / norm;
        if !(result.is_normal() || result == 0.0) {
            return None;
        }
        let rounding = error_product(0.5 * f64::EPSILON * (1.0 + f64::EPSILON), result.abs())?;
        let residual = numerator_error
            + error_product(result.abs(), norm_error)?
            + error_product(rounding, norm)?;
        let propagated = residual / lower_norm;
        let propagated = if propagated.is_normal() || residual == 0.0 {
            propagated
        } else {
            propagated.next_up()
        };
        let result_error = error_bound(0.0, propagated)?;
        (result_error.is_finite()
            && (result_error <= (tolerance * result.abs()).next_down()
                || (result == 0.0 && result_error == 0.0)))
            .then_some(result)
    };
    Some(Complex64::new(
        component(
            n.value.re, n.error.re, d.value.re, d.error.re, n.value.im, n.error.im, d.value.im,
            d.error.im,
        )?,
        component(
            n.value.im,
            n.error.im,
            d.value.re,
            d.error.re,
            -n.value.re,
            n.error.re,
            d.value.im,
            d.error.im,
        )?,
    ))
}

fn product_error(a: f64, ea: f64, b: f64, eb: f64) -> Option<f64> {
    Some(error_product(a.abs() + ea, eb)? + error_product(b.abs(), ea)?)
}

pub(super) fn ordinary_ratio(
    n: &[f64],
    d: &[f64],
    z: [f64; 2],
    z_error: [f64; 2],
) -> Option<[f64; 2]> {
    let z = Complex64::new(z[0], z[1]);
    let z_error = Complex64::new(z_error[0], z_error[1]);
    let result = bounded_quotient(
        bounded_polynomial(n, z, z_error)?,
        bounded_polynomial(d, z, z_error)?,
        8.0 * f64::EPSILON * (n.len() + d.len() + 1) as f64,
    )?;
    Some([result.re, result.im])
}

// For the exact unit-circle argument z, |z|=1. With computed center v and
// approximate argument zh, the exact propagation identity is
//   v*zh - p*z = (v-p)*z + v*(zh-z).
// Consequently a disk radius grows additively, even though rectangular
// component bounds can grow like (|Re z|+|Im z|)^degree. L1 magnitudes bound
// the two complex norms below without any platform hypot accuracy assumption.
fn disk_polynomial(c: &[f64], z: Complex64, error_z: f64) -> Option<BoundedComplex> {
    let (&leading, rest) = c.split_last()?;
    let mut value = Complex64::new(leading, 0.0);
    let mut error = 0.0;
    for &coefficient in rest.iter().rev() {
        let p = ordinary_product(value.re, z.re)?;
        let q = ordinary_product(value.im, z.im)?;
        let r = ordinary_product(value.re, z.im)?;
        let s = ordinary_product(value.im, z.re)?;
        let difference = p - q;
        let real = difference + coefficient;
        let imaginary = r + s;
        if ![difference, real, imaginary]
            .into_iter()
            .all(f64::is_finite)
        {
            return None;
        }
        let scale =
            p.abs() + q.abs() + r.abs() + s.abs() + difference.abs() + real.abs() + imaginary.abs();
        error = error_bound(
            scale,
            error
                + error_product(value.re.abs() + value.im.abs(), error_z)?
                + product_floor(value.re, z.re, p)
                + product_floor(value.im, z.im, q)
                + product_floor(value.re, z.im, r)
                + product_floor(value.im, z.re, s),
        )?;
        value = Complex64::new(real, imaginary);
    }
    Some(BoundedComplex {
        value,
        error: Complex64::new(error, error),
    })
}
pub(super) fn ordinary_circle_ratio(
    n: &[f64],
    d: &[f64],
    z: [f64; 2],
    ze: [f64; 2],
) -> Option<[f64; 2]> {
    let argument = Complex64::new(z[0], z[1]);
    let radius = error_bound(0.0, ze[0] + ze[1])?;
    let result = bounded_quotient(
        disk_polynomial(n, argument, radius)?,
        disk_polynomial(d, argument, radius)?,
        8.0 * f64::EPSILON * (n.len() + d.len() + 1) as f64,
    )?;
    Some([result.re, result.im])
}

// Bounded two-component arithmetic between the binary64 and dyadic paths.
const DD_U: f64 = 0.5 * f64::EPSILON * (1.0 + f64::EPSILON);
fn dd_up_add(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        b
    } else if b == 0.0 {
        a
    } else {
        (a + b).next_up()
    }
}
fn dd_up_mul(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        0.0
    } else {
        (a * b).next_up()
    }
}
fn dd_up_div(a: f64, b: f64) -> f64 {
    if a == 0.0 { 0.0 } else { (a / b).next_up() }
}
fn dd_round(value: f64, may_underflow: bool) -> f64 {
    if value.is_normal() {
        dd_up_mul(DD_U, value.abs())
    } else if may_underflow {
        f64::from_bits(1)
    } else {
        0.0
    }
}
fn dd_raw_round(value: f64, may_underflow: bool) -> f64 {
    if value.is_normal() {
        let error = DD_U * value.abs();
        if error.is_normal() {
            error
        } else {
            error.next_up()
        }
    } else if may_underflow {
        f64::from_bits(1)
    } else {
        0.0
    }
}
fn dd_inflate(error: f64) -> Option<f64> {
    // Each bound expression below has fewer than sixteen positive rounded
    // operations along any dependency path. This upward margin also covers
    // their normal products; subnormal products round upward separately.
    let upper = if error.is_normal() || error == 0.0 {
        error * (1.0 + 32.0 * f64::EPSILON)
    } else {
        error.next_up()
    };
    upper.is_finite().then_some(upper)
}
fn dd_two_sum(a: f64, b: f64) -> Option<(f64, f64)> {
    let s = a + b;
    let bv = s - a;
    let av = s - bv;
    let br = b - bv;
    let ar = a - av;
    let e = ar + br;
    [s, bv, av, br, ar, e]
        .iter()
        .all(|v| v.is_finite())
        .then_some((s, e))
}
#[derive(Clone, Copy)]
struct DdBound {
    hi: f64,
    lo: f64,
    error: f64,
}
impl DdBound {
    fn exact(value: f64) -> Self {
        Self {
            hi: value,
            lo: 0.0,
            error: 0.0,
        }
    }
    fn finish(hi: f64, lo: f64, error: f64) -> Option<Self> {
        let (hi, lo) = dd_two_sum(hi, lo)?;
        error.is_finite().then_some(Self { hi, lo, error })
    }
    fn abs_center(self) -> f64 {
        dd_up_add(self.hi.abs(), self.lo.abs())
    }
    fn magnitude(self) -> f64 {
        dd_up_add(self.abs_center(), self.error)
    }
    fn negated(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
            error: self.error,
        }
    }
    fn add(self, b: Self) -> Option<Self> {
        let (hi, e) = dd_two_sum(self.hi, b.hi)?;
        let low = self.lo + b.lo;
        let combined = e + low;
        let error = dd_inflate(
            self.error + b.error + dd_raw_round(low, false) + dd_raw_round(combined, false),
        )?;
        Self::finish(hi, combined, error)
    }
    fn multiply(self, b: Self) -> Option<Self> {
        let hi = self.hi * b.hi;
        if !hi.is_finite() {
            return None;
        }
        let residual = self.hi.mul_add(b.hi, -hi);
        // Above this conservative product threshold, the exact product's
        // least significant bit lies above minsub. A zero FMA residual is
        // therefore exact; it does not need an artificial subnormal radius.
        let mut error = dd_raw_round(
            residual,
            hi.abs() < 2.0_f64.powi(-968) && self.hi != 0.0 && b.hi != 0.0,
        );
        let mut low = residual;
        for (a, b) in [(self.hi, b.lo), (self.lo, b.hi), (self.lo, b.lo)] {
            let product = a * b;
            error += dd_raw_round(product, a != 0.0 && b != 0.0);
            low += product;
            error += dd_raw_round(low, false);
        }
        let propagated = error_product(self.hi.abs() + self.lo.abs() + self.error, b.error)?
            + error_product(b.hi.abs() + b.lo.abs(), self.error)?;
        Self::finish(hi, low, dd_inflate(error + propagated)?)
    }
    fn divide_scalar(self, b: f64) -> Option<Self> {
        if b == 0.0 || !b.is_finite() {
            return None;
        }
        let hi = self.hi / b;
        if !hi.is_finite() {
            return None;
        }
        let residual = (-hi).mul_add(b, self.hi);
        let combined = residual + self.lo;
        let low = combined / b;
        let numerator_error = self.error
            + dd_raw_round(
                residual,
                self.hi.abs() < 2.0_f64.powi(-968) && self.hi != 0.0,
            )
            + dd_raw_round(combined, false);
        let propagated = numerator_error / b.abs();
        let propagated = if propagated.is_normal() || numerator_error == 0.0 {
            propagated
        } else {
            propagated.next_up()
        };
        let error = dd_inflate(propagated + dd_raw_round(low, combined != 0.0))?;
        Self::finish(hi, low, error)
    }
    fn divide_positive(self, b: Self, tolerance: f64) -> Option<f64> {
        let numerator_error = dd_up_add(self.lo.abs(), self.error);
        let denominator_error = dd_up_add(b.lo.abs(), b.error);
        let lower = if denominator_error == 0.0 {
            b.hi
        } else {
            (b.hi - denominator_error).next_down()
        };
        if !lower.is_finite() || lower <= 0.0 {
            return None;
        }
        let result = self.hi / b.hi;
        if !result.is_finite() {
            return None;
        }
        let residual = dd_up_add(
            numerator_error,
            dd_up_add(
                dd_up_mul(result.abs(), denominator_error),
                dd_up_mul(dd_round(result, self.hi != 0.0), b.hi),
            ),
        );
        let radius = dd_up_div(residual, lower);
        (radius.is_finite()
            && (radius <= (tolerance * result.abs()).next_down()
                || (result == 0.0 && radius == 0.0)))
            .then_some(result)
    }
}
fn dd_sine_cosine(phase: SmallExact) -> Option<[DdBound; 2]> {
    // Scaling both pieces of this <=106-bit integer remains exact here.
    // Smaller phase exponents retain the existing arbitrary-precision path.
    if !(-900..=0).contains(&phase.exponent) {
        return None;
    }
    let hi = phase.value as f64;
    let lo = (phase.value - hi as i128) as f64;
    let scale = 2.0_f64.powi(phase.exponent);
    let phase = DdBound {
        hi: hi * scale,
        lo: lo * scale,
        error: 0.0,
    };
    // The exact Machin-series interval checker verifies this seed enclosure.
    let tau = DdBound {
        hi: -std::f64::consts::TAU,
        lo: -2.4492935982947064e-16,
        error: 2.0_f64.powi(-103),
    };
    let angle = phase.multiply(tau)?;
    if angle.magnitude() > 0.8 {
        return None;
    }
    let square = angle.multiply(angle)?.negated();
    let mut sine = angle;
    let mut cosine = DdBound::exact(1.0);
    let mut sine_term = sine;
    let mut cosine_term = cosine;
    let mut sine_tail = 0.0;
    let mut cosine_tail = 0.0;
    for k in 1..=16 {
        sine_term = sine_term
            .multiply(square)?
            .divide_scalar((2 * k * (2 * k + 1)) as f64)?;
        cosine_term = cosine_term
            .multiply(square)?
            .divide_scalar(((2 * k - 1) * 2 * k) as f64)?;
        sine = sine.add(sine_term)?;
        cosine = cosine.add(cosine_term)?;
        // The alternating real Taylor series has remainder at most the next
        // term. 0.65 is strictly above the allowed angle's squared magnitude.
        let factor = (0.65 / ((2 * k + 1) * (2 * k + 2)) as f64).next_up();
        sine_tail = dd_up_mul(sine_term.magnitude(), factor);
        cosine_tail = dd_up_mul(cosine_term.magnitude(), factor);
        if k >= 4
            && sine_tail <= sine.magnitude() * 2.0_f64.powi(-80)
            && cosine_tail <= cosine.magnitude() * 2.0_f64.powi(-80)
        {
            break;
        }
    }
    sine.error = dd_up_add(sine.error, sine_tail);
    cosine.error = dd_up_add(cosine.error, cosine_tail);
    Some([cosine, sine])
}
fn dd_polynomial(c: &[f64], z: [DdBound; 2]) -> Option<[DdBound; 2]> {
    let (&leading, rest) = c.split_last()?;
    let mut v = [DdBound::exact(leading), DdBound::exact(0.0)];
    for &coefficient in rest.iter().rev() {
        v = [
            v[0].multiply(z[0])?
                .add(v[1].multiply(z[1])?.negated())?
                .add(DdBound::exact(coefficient))?,
            v[0].multiply(z[1])?.add(v[1].multiply(z[0])?)?,
        ];
    }
    Some(v)
}
pub(super) fn dd_circle(n: &[f64], d: &[f64], quarter: u8, offset: SmallExact) -> Option<[f64; 2]> {
    let [cosine, sine] = dd_sine_cosine(offset)?;
    let z = match quarter {
        0 => [cosine, sine],
        1 => [sine, cosine.negated()],
        2 => [cosine.negated(), sine.negated()],
        _ => [sine.negated(), cosine],
    };
    dd_ratio(n, d, z)
}

pub(super) fn dd_complex_ratio(n: &[f64], d: &[f64], z: [f64; 2]) -> Option<[f64; 2]> {
    dd_ratio(n, d, z.map(DdBound::exact))
}

fn dd_ratio(n: &[f64], d: &[f64], z: [DdBound; 2]) -> Option<[f64; 2]> {
    let n_value = dd_polynomial(n, z)?;
    let d_value = dd_polynomial(d, z)?;
    let norm = d_value[0]
        .multiply(d_value[0])?
        .add(d_value[1].multiply(d_value[1])?)?;
    let real = n_value[0]
        .multiply(d_value[0])?
        .add(n_value[1].multiply(d_value[1])?)?;
    let imaginary = n_value[1]
        .multiply(d_value[0])?
        .add(n_value[0].multiply(d_value[1])?.negated())?;
    let tolerance = 8.0 * f64::EPSILON * (n.len() + d.len() + 1) as f64;
    Some([
        real.divide_positive(norm, tolerance)?,
        imaginary.divide_positive(norm, tolerance)?,
    ])
}
