//! Certified recovery with exact centers and outward dyadic error radii.
use super::{
    ArithmeticError, BigMagnitude, Dyadic, SmallExact, exact_complex_product,
    has_dyadic_root_factor, reduced_product_phase, rotate_exact, scaled_exact_ratio_to_f64,
    small_finite, trim_coefficients, unit_circle_polynomial_ratio,
};
fn zero() -> Dyadic {
    Dyadic::from_small(SmallExact::ZERO)
}
fn one() -> Dyadic {
    Dyadic::from_small(small_finite(1.0))
}
fn absolute(v: &Dyadic) -> Dyadic {
    let mut r = v.clone();
    r.negative = false;
    r
}
fn scale(mut v: Dyadic, e: i32) -> Result<Dyadic, ArithmeticError> {
    v.exponent = v
        .exponent
        .checked_add(e)
        .ok_or(ArithmeticError::MantissaBounds)?;
    Ok(v)
}
fn power_of_two(e: i32) -> Dyadic {
    Dyadic::from_small(SmallExact {
        value: 1,
        exponent: e,
    })
}
fn unsigned_integer(v: u64) -> Dyadic {
    Dyadic::from_small(SmallExact {
        value: i128::from(v),
        exponent: 0,
    })
}
fn compare(a: &Dyadic, b: &Dyadic) -> Result<std::cmp::Ordering, ArithmeticError> {
    let diff = a.add(&b.clone().negated())?;
    Ok(if diff.magnitude.is_zero() {
        std::cmp::Ordering::Equal
    } else if diff.negative {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    })
}
fn less_equal(a: &Dyadic, b: &Dyadic) -> Result<bool, ArithmeticError> {
    Ok(compare(a, b)? != std::cmp::Ordering::Greater)
}
fn truncate(mut v: Dyadic, p: usize) -> Result<(Dyadic, Dyadic), ArithmeticError> {
    let len = v.magnitude.significant_len();
    if len == 0 {
        return Ok((v, zero()));
    }
    let bits = (len - 1) * 64 + 64 - v.magnitude.word(len - 1).leading_zeros() as usize;
    let shift = bits.saturating_sub(p);
    let discarded = shift > v.magnitude.trailing_zeros();
    v.magnitude = v.magnitude.shift_right(shift);
    v.exponent = v
        .exponent
        .checked_add(i32::try_from(shift).map_err(|_| ArithmeticError::MantissaBounds)?)
        .ok_or(ArithmeticError::MantissaBounds)?;
    let error = if discarded {
        power_of_two(v.exponent)
    } else {
        zero()
    };
    Ok((v, error))
}
fn round_radius_up(v: Dyadic) -> Result<Dyadic, ArithmeticError> {
    debug_assert!(!v.negative || v.magnitude.is_zero());
    let (value, error) = truncate(v, 96)?;
    value.add(&error)
}
fn add_up(a: &Dyadic, b: &Dyadic) -> Result<Dyadic, ArithmeticError> {
    round_radius_up(a.add(b)?)
}
fn multiply_up(a: &Dyadic, b: &Dyadic) -> Result<Dyadic, ArithmeticError> {
    round_radius_up(a.multiply(b)?)
}
fn divide_small(v: Dyadic, d: u64, p: usize) -> Result<(Dyadic, Dyadic), ArithmeticError> {
    if d == 0 {
        return Err(ArithmeticError::ZeroDenominator);
    }
    let len = v.magnitude.significant_len();
    if len == 0 {
        return Ok((v, zero()));
    }
    let bits = (len - 1) * 64 + 64 - v.magnitude.word(len - 1).leading_zeros() as usize;
    let shift = (p + 64).saturating_sub(bits);
    let magnitude = v.magnitude.shift_left(shift);
    let mut q = BigMagnitude::default();
    let mut rem = 0_u128;
    for i in (0..magnitude.significant_len()).rev() {
        let n = (rem << 64) | u128::from(magnitude.word(i));
        q.set_word(i, (n / u128::from(d)) as u64);
        rem = n % u128::from(d);
    }
    let exponent = v
        .exponent
        .checked_sub(i32::try_from(shift).map_err(|_| ArithmeticError::MantissaBounds)?)
        .ok_or(ArithmeticError::MantissaBounds)?;
    let division_error = if rem == 0 {
        zero()
    } else {
        power_of_two(exponent)
    };
    let (value, round_error) = truncate(
        Dyadic {
            negative: v.negative,
            magnitude: q,
            exponent,
        },
        p,
    )?;
    Ok((value, add_up(&division_error, &round_error)?))
}
fn divide_up(v: &Dyadic, d: u64) -> Result<Dyadic, ArithmeticError> {
    let (q, e) = divide_small(v.clone(), d, 96)?;
    add_up(&q, &e)
}
#[derive(Clone)]
struct DyadicBall {
    value: Dyadic,
    error: Dyadic,
}
impl DyadicBall {
    fn exact(value: Dyadic) -> Self {
        Self {
            value,
            error: zero(),
        }
    }
    fn abs_upper(&self) -> Result<Dyadic, ArithmeticError> {
        add_up(&absolute(&self.value), &self.error)
    }
    fn add(&self, b: &Self, p: usize) -> Result<Self, ArithmeticError> {
        let (value, round) = truncate(self.value.add(&b.value)?, p)?;
        Ok(Self {
            value,
            error: add_up(&add_up(&self.error, &b.error)?, &round)?,
        })
    }
    fn negated(mut self) -> Self {
        self.value = self.value.negated();
        self
    }
    fn scale(self, e: i32) -> Result<Self, ArithmeticError> {
        Ok(Self {
            value: scale(self.value, e)?,
            error: scale(self.error, e)?,
        })
    }
    fn mul(&self, b: &Self, p: usize) -> Result<Self, ArithmeticError> {
        let (value, round) = truncate(self.value.multiply(&b.value)?, p)?;
        let propagated = add_up(
            &multiply_up(&self.abs_upper()?, &b.error)?,
            &multiply_up(&absolute(&b.value), &self.error)?,
        )?;
        Ok(Self {
            value,
            error: add_up(&propagated, &round)?,
        })
    }
    fn div_small(&self, d: u64, p: usize) -> Result<Self, ArithmeticError> {
        let (value, round) = divide_small(self.value.clone(), d, p)?;
        Ok(Self {
            value,
            error: add_up(&divide_up(&self.error, d)?, &round)?,
        })
    }
}
fn precision_limit(bits: usize) -> ArithmeticError {
    ArithmeticError::PrecisionLimit { bits: bits as u32 }
}
fn arctangent(divisor: u64, p: usize) -> Result<DyadicBall, ArithmeticError> {
    let work = p + 32;
    let x = DyadicBall::exact(one()).div_small(divisor, work)?;
    let square = x.mul(&x, work)?.negated();
    let mut power = x.clone();
    let mut sum = x;
    for k in 0..p + 32 {
        // The alternating remainder is smaller than the next power; its
        // denominator is at least one. The power ball includes arithmetic error.
        let tail = divide_up(&power.abs_upper()?, divisor * divisor)?;
        if leading_exponent(&tail) < -(p as i32) - 16 {
            sum.error = add_up(&sum.error, &tail)?;
            return Ok(sum);
        }
        power = power.mul(&square, work)?;
        sum = sum.add(&power.div_small(2 * k as u64 + 3, work)?, work)?;
    }
    Err(precision_limit(p))
}
fn compute_pi(p: usize) -> Result<DyadicBall, ArithmeticError> {
    arctangent(5, p)?
        .scale(4)?
        .add(&arctangent(239, p)?.scale(2)?.negated(), p + 32)
}
fn pi(p: usize) -> Result<DyadicBall, ArithmeticError> {
    static CACHE: [std::sync::OnceLock<Result<DyadicBall, ArithmeticError>>; 6] =
        [const { std::sync::OnceLock::new() }; 6];
    match [160, 288, 544, 1056, 2080, 4128]
        .iter()
        .position(|&bits| bits == p)
    {
        Some(i) => CACHE[i].get_or_init(|| compute_pi(p)).clone(),
        None => compute_pi(p),
    }
}
fn sin_cos_half_angle(offset: SmallExact, p: usize) -> Result<[DyadicBall; 2], ArithmeticError> {
    if offset.value == 0 {
        return Ok([DyadicBall::exact(one()), DyadicBall::exact(zero())]);
    }
    let work = p + 32;
    let angle = pi(work)?.mul(&DyadicBall::exact(Dyadic::from_small(offset)), work)?;
    let square = angle.mul(&angle, work)?.negated();
    let mut sine = angle.clone();
    let mut cosine = DyadicBall::exact(one());
    let mut st = sine.clone();
    let mut ct = cosine.clone();
    for k in 0..p + 32 {
        // |pi*offset| < 1/2, so the next squared-angle factor is < 1/4.
        let divisor = 4 * (2 * k as u64 + 1) * (2 * k as u64 + 2);
        let se = divide_up(&st.abs_upper()?, divisor)?;
        let ce = divide_up(&ct.abs_upper()?, divisor)?;
        if leading_exponent(&se) < leading_exponent(&sine.value) - p as i32 - 16
            && leading_exponent(&ce) < leading_exponent(&cosine.value) - p as i32 - 16
        {
            sine.error = add_up(&sine.error, &se)?;
            cosine.error = add_up(&cosine.error, &ce)?;
            return Ok([cosine, sine]);
        }
        let j = k as u64 + 1;
        st = st
            .mul(&square, work)?
            .div_small(2 * j * (2 * j + 1), work)?;
        ct = ct
            .mul(&square, work)?
            .div_small((2 * j - 1) * 2 * j, work)?;
        sine = sine.add(&st, work)?;
        cosine = cosine.add(&ct, work)?;
    }
    Err(precision_limit(p))
}

// A component vanishes exactly iff its rational Laurent polynomial vanishes
// at the physical dyadic phase. Reduce it modulo the phase's cyclotomic
// polynomial X^(2^(bits-1))+1, retaining every product exactly.
fn component_remainders(
    n: &[f64],
    d: &[f64],
    bits: u32,
) -> Result<[Vec<Dyadic>; 2], ArithmeticError> {
    let degree = n.len().max(d.len()).saturating_sub(1);
    let span = degree
        .checked_mul(2)
        .and_then(|v| v.checked_add(1))
        .ok_or(ArithmeticError::MantissaBounds)?;
    let modulus = if bits == 0 {
        1
    } else {
        1_usize.checked_shl(bits - 1).unwrap_or(span).min(span)
    };
    let mut re = vec![zero(); modulus];
    let mut im = re.clone();
    for (i, &a) in n.iter().enumerate() {
        for (j, &b) in d.iter().enumerate() {
            if a == 0.0 || b == 0.0 {
                continue;
            }
            let product = Dyadic::from_small(
                SmallExact::product(a, b).ok_or(ArithmeticError::MantissaBounds)?,
            );
            for (index, conjugate) in [(degree + i - j, false), (degree + j - i, true)] {
                let flip = bits != 0 && (index / modulus) % 2 != 0;
                let term = if flip {
                    product.clone().negated()
                } else {
                    product.clone()
                };
                re[index % modulus] = re[index % modulus].add(&term)?;
                im[index % modulus] =
                    im[index % modulus].add(&if conjugate { term.negated() } else { term })?;
            }
        }
    }
    Ok([re, im])
}
fn component_zeros(n: &[f64], d: &[f64], bits: u32) -> Result<[bool; 2], ArithmeticError> {
    Ok(component_remainders(n, d, bits)?.map(|v| v.iter().all(|v| v.magnitude.is_zero())))
}
fn component_equals(
    n: &[f64],
    d: &[f64],
    bits: u32,
    odd_mod4: u8,
    component: usize,
    target: &Dyadic,
) -> Result<bool, ArithmeticError> {
    let degree = n.len().max(d.len()).saturating_sub(1);
    let span = 2 * degree + 1;
    let (rotation, imag_negative, modulus) = if component == 0 {
        (
            0,
            false,
            1_usize.checked_shl(bits - 1).unwrap_or(span).min(span),
        )
    } else {
        let Some(modulus) = 1_usize.checked_shl(bits - 1) else {
            return Ok(false);
        };
        // If the shifted norm and the Laurent numerator have disjoint degree
        // ranges below the minimal polynomial's degree, equality is impossible.
        if modulus / 2 > 2 * degree {
            return Ok(false);
        }
        (modulus / 2, odd_mod4 == 1, modulus)
    };
    let [re, im] = component_remainders(n, d, bits)?;
    let mut remainder = if component == 0 { re } else { im };
    remainder.resize_with(modulus, zero);
    for (i, &a) in d.iter().enumerate() {
        for (j, &b) in d.iter().enumerate() {
            if a == 0.0 || b == 0.0 {
                continue;
            }
            let product = Dyadic::from_small(
                SmallExact::product(a, b).ok_or(ArithmeticError::MantissaBounds)?,
            );
            let mut term = scale(product.multiply(target)?, 1)?;
            let index = degree + i - j + rotation;
            if (index / modulus).is_multiple_of(2) ^ imag_negative {
                term = term.negated();
            }
            remainder[index % modulus] = remainder[index % modulus].add(&term)?;
        }
    }
    Ok(remainder.iter().all(|v| v.magnitude.is_zero()))
}
fn complex_zero() -> [Dyadic; 2] {
    [zero(), zero()]
}
fn complex_add(a: &[Dyadic; 2], b: &[Dyadic; 2]) -> Result<[Dyadic; 2], ArithmeticError> {
    Ok([a[0].add(&b[0])?, a[1].add(&b[1])?])
}
fn norm_bound(a: &[Dyadic; 2]) -> Result<Dyadic, ArithmeticError> {
    add_up(&absolute(&a[0]), &absolute(&a[1]))
}
fn curvature_bound(a: &[f64]) -> Result<Dyadic, ArithmeticError> {
    let mut result = zero();
    for (i, &a) in a.iter().enumerate().skip(2) {
        let factor = i
            .checked_mul(i - 1)
            .ok_or(ArithmeticError::MantissaBounds)?
            / 2;
        result = add_up(
            &result,
            &multiply_up(
                &Dyadic::from_small(small_finite(a.abs())),
                &unsigned_integer(factor as u64),
            )?,
        )?;
    }
    Ok(result)
}
enum ComponentResult {
    Value(f64),
    Failure(ArithmeticError),
    Retry,
}
fn rounded_ratio(a: &Dyadic, b: &Dyadic) -> Result<f64, ArithmeticError> {
    scaled_exact_ratio_to_f64(
        &a.magnitude,
        &b.magnitude,
        a.negative ^ b.negative,
        a.exponent
            .checked_sub(b.exponent)
            .ok_or(ArithmeticError::MantissaBounds)?,
    )
}
fn certify_component(
    a: &Dyadic,
    ea: &Dyadic,
    b: &Dyadic,
    eb: &Dyadic,
    zero: bool,
    equals: &impl Fn(&Dyadic) -> Result<bool, ArithmeticError>,
) -> Result<ComponentResult, ArithmeticError> {
    if zero {
        return Ok(ComponentResult::Value(0.0));
    }
    let low = b.add(&eb.clone().negated())?;
    if low.negative || low.magnitude.is_zero() {
        return Ok(ComponentResult::Retry);
    }
    let high = b.add(eb)?;
    let abs = absolute(a);
    let upper = abs.add(ea)?;
    if less_equal(&upper, &scale(low.clone(), -1075)?)? {
        return Ok(ComponentResult::Failure(ArithmeticError::Underflow));
    }
    let lower = abs.add(&ea.clone().negated())?;
    if lower.negative || less_equal(&lower, &scale(high.clone(), -1075)?)? {
        // An interval cannot resolve an exact rounding-boundary tie by adding
        // precision. Prove equality at the physical root of unity instead.
        if equals(&power_of_two(-1075))? || equals(&power_of_two(-1075).negated())? {
            return Ok(ComponentResult::Failure(ArithmeticError::Underflow));
        }
    }
    let overflow = power_of_two(1024).add(&power_of_two(970).negated())?;
    if !lower.negative
        && !lower.magnitude.is_zero()
        && less_equal(&high.multiply(&overflow)?, &lower)?
    {
        return Ok(ComponentResult::Failure(ArithmeticError::Overflow {
            negative: a.negative,
        }));
    }
    if less_equal(&low.multiply(&overflow)?, &upper)? {
        for negative in [false, true] {
            let target = if negative {
                overflow.clone().negated()
            } else {
                overflow.clone()
            };
            if equals(&target)? {
                return Ok(ComponentResult::Failure(ArithmeticError::Overflow {
                    negative,
                }));
            }
        }
    }
    if let Ok(q) = rounded_ratio(a, b)
        && q != 0.0
        && q.is_finite()
    {
        let exact_q = Dyadic::from_small(small_finite(q));
        let residual = absolute(&a.add(&exact_q.multiply(b)?.negated())?);
        let bound = add_up(
            &add_up(ea, &multiply_up(&absolute(&exact_q), eb)?)?,
            &residual,
        )?;
        let allowed = scale(absolute(&exact_q).multiply(&low)?, -50)?;
        if less_equal(&bound, &allowed)? {
            return Ok(ComponentResult::Value(q));
        }
    }
    // Endpoint agreement proves a correctly rounded subnormal as well as a
    // normal result. Unlike agreement of precision levels, these are certified
    // enclosure endpoints of the same exact physical quantity.
    let lo_a = a.add(&ea.clone().negated())?;
    let hi_a = a.add(ea)?;
    let lo = rounded_ratio(&lo_a, if lo_a.negative { &low } else { &high });
    let hi = rounded_ratio(&hi_a, if hi_a.negative { &high } else { &low });
    if let (Ok(lo), Ok(hi)) = (lo, hi) {
        if lo == hi && lo.is_finite() {
            return Ok(if lo == 0.0 {
                ComponentResult::Failure(ArithmeticError::Underflow)
            } else {
                ComponentResult::Value(lo)
            });
        }
        if lo.is_finite() && hi.is_finite() && lo.next_up() == hi {
            let midpoint = scale(
                Dyadic::from_small(small_finite(lo)).add(&Dyadic::from_small(small_finite(hi)))?,
                -1,
            )?;
            if equals(&midpoint)? {
                let rounded = if lo.to_bits() & 1 == 0 { lo } else { hi };
                return Ok(if rounded == 0.0 {
                    ComponentResult::Failure(ArithmeticError::Underflow)
                } else {
                    ComponentResult::Value(rounded)
                });
            }
        }
    }
    Ok(ComponentResult::Retry)
}
fn try_precision(
    n: &[f64],
    d: &[f64],
    quarter: u8,
    offset: SmallExact,
    p: usize,
    zeros: [bool; 2],
    curvature: &[Dyadic; 2],
    full_taylor: bool,
) -> Result<Option<[f64; 2]>, ArithmeticError> {
    let [cosine, sine] = sin_cos_half_angle(offset, p)?;
    if !less_equal(&power_of_two(-1), &cosine.value)? {
        return Ok(None);
    }
    let delta = scale(
        add_up(
            &sine.error,
            &multiply_up(&absolute(&sine.value), &cosine.error)?,
        )?,
        2,
    )?;
    let w = rotate_exact(
        [cosine.value.clone(), sine.value.clone().negated()],
        quarter,
    );
    let v = [cosine.value, sine.value];
    let degree = n.len().max(d.len()).saturating_sub(1);
    let r = v[0].multiply(&v[0])?.add(&v[1].multiply(&v[1])?)?;
    let extra = r.add(&one().negated())?;
    if !extra.negative
        && !less_equal(
            &extra.multiply(&unsigned_integer(2 * degree as u64))?,
            &one(),
        )?
    {
        return Ok(None);
    }
    let mut numerator = [
        Dyadic::from_small(small_finite(n.get(degree).copied().unwrap_or(0.0))),
        zero(),
    ];
    let mut denominator = [
        Dyadic::from_small(small_finite(d.get(degree).copied().unwrap_or(0.0))),
        zero(),
    ];
    let mut nd = complex_zero();
    let mut dd = complex_zero();
    let mut power = [one(), zero()];
    let mut nh = vec![
        complex_zero();
        if full_taylor {
            degree.saturating_sub(1)
        } else {
            0
        }
    ];
    let mut dh = nh.clone();
    for i in (0..degree).rev() {
        // Only materialize higher Taylor coefficients when the inexpensive
        // first-derivative/global-curvature enclosure failed. Exact jets retain
        // repeated-root cancellation instead of increasing trig precision to
        // compensate for a needlessly loose global derivative bound.
        for j in (2..=nh.len() + 1).rev() {
            nh[j - 2] = complex_add(
                &exact_complex_product(&nh[j - 2], &w)?,
                &exact_complex_product(if j == 2 { &nd } else { &nh[j - 3] }, &v)?,
            )?;
            dh[j - 2] = complex_add(
                &exact_complex_product(&dh[j - 2], &w)?,
                &exact_complex_product(if j == 2 { &dd } else { &dh[j - 3] }, &v)?,
            )?;
        }
        nd = complex_add(
            &exact_complex_product(&nd, &w)?,
            &exact_complex_product(&numerator, &v)?,
        )?;
        dd = complex_add(
            &exact_complex_product(&dd, &w)?,
            &exact_complex_product(&denominator, &v)?,
        )?;
        power = exact_complex_product(&power, &v)?;
        numerator = exact_complex_product(&numerator, &w)?;
        denominator = exact_complex_product(&denominator, &w)?;
        let nc = Dyadic::from_small(small_finite(n.get(i).copied().unwrap_or(0.0)));
        let dc = Dyadic::from_small(small_finite(d.get(i).copied().unwrap_or(0.0)));
        for c in 0..2 {
            numerator[c] = numerator[c].add(&power[c].multiply(&nc)?)?;
            denominator[c] = denominator[c].add(&power[c].multiply(&dc)?)?;
        }
    }
    let remainder_scale = scale(multiply_up(&delta, &delta)?, 1)?;
    let mut en = multiply_up(&delta, &norm_bound(&nd)?)?;
    let mut ed = multiply_up(&delta, &norm_bound(&dd)?)?;
    if full_taylor {
        let mut delta_power = delta.clone();
        for (n, d) in nh.iter().zip(&dh) {
            delta_power = multiply_up(&delta_power, &delta)?;
            en = add_up(&en, &multiply_up(&delta_power, &norm_bound(n)?)?)?;
            ed = add_up(&ed, &multiply_up(&delta_power, &norm_bound(d)?)?)?;
        }
    } else {
        en = add_up(&en, &multiply_up(&remainder_scale, &curvature[0])?)?;
        ed = add_up(&ed, &multiply_up(&remainder_scale, &curvature[1])?)?;
    }
    let nl = norm_bound(&numerator)?;
    let dl = norm_bound(&denominator)?;
    let ea = add_up(
        &add_up(&multiply_up(&nl, &ed)?, &multiply_up(&dl, &en)?)?,
        &multiply_up(&en, &ed)?,
    )?;
    let eb = add_up(&scale(multiply_up(&dl, &ed)?, 1)?, &multiply_up(&ed, &ed)?)?;
    let [nr, ni] = numerator;
    let [dr, di] = denominator;
    let b = dr.multiply(&dr)?.add(&di.multiply(&di)?)?;
    let ar = nr.multiply(&dr)?.add(&ni.multiply(&di)?)?;
    let ai = ni.multiply(&dr)?.add(&nr.multiply(&di)?.negated())?;
    let mut result = [0.0; 2];
    let bits = (-offset.exponent) as u32;
    let odd_mod4 = ((offset.value.rem_euclid(4)
        + i128::from(quarter) * (1_i128 << (bits - 2).min(2)))
        % 4) as u8;
    for (i, a) in [ar, ai].iter().enumerate() {
        match certify_component(a, &ea, &b, &eb, zeros[i], &|target| {
            component_equals(n, d, bits, odd_mod4, i, target)
        })? {
            ComponentResult::Value(v) => result[i] = v,
            ComponentResult::Failure(e) => return Err(e),
            ComponentResult::Retry => return Ok(None),
        }
    }
    Ok(Some(result))
}
pub(super) fn recover(n: &[f64], d: &[f64], f: f64, p: f64) -> Result<[f64; 2], ArithmeticError> {
    if n.iter().chain(d).chain([&f, &p]).any(|v| !v.is_finite()) {
        return Err(ArithmeticError::NonFiniteTerm);
    }
    let n = trim_coefficients(n);
    let d = trim_coefficients(d);
    if d.iter().all(|&v| v == 0.0) {
        return Err(ArithmeticError::ZeroDenominator);
    }
    let (quarter, offset, bits) = reduced_product_phase(f, p);
    if offset.value == 0 {
        return unit_circle_polynomial_ratio(n, d, f, p);
    }
    if has_dyadic_root_factor(d, bits)? {
        return Err(ArithmeticError::ZeroDenominator);
    }
    if n.iter().all(|&v| v == 0.0) {
        return Ok([0.0, 0.0]);
    }
    if has_dyadic_root_factor(n, bits)? {
        return Ok([0.0, 0.0]);
    }
    let zeros = component_zeros(n, d, bits)?;
    let curvature = [curvature_bound(n)?, curvature_bound(d)?];
    let higher_derivatives = n.len().max(d.len()) <= 65;
    for precision in [128, 256, 512, 1024, 2048, 4096] {
        if let Some(value) = try_precision(
            n,
            d,
            quarter,
            offset,
            precision,
            zeros,
            &curvature,
            precision > 128 && higher_derivatives,
        )? {
            return Ok(value);
        }
        if precision == 128
            && higher_derivatives
            && let Some(value) =
                try_precision(n, d, quarter, offset, precision, zeros, &curvature, true)?
        {
            return Ok(value);
        }
    }
    Err(precision_limit(4096))
}

fn leading_exponent(v: &Dyadic) -> i32 {
    let len = v.magnitude.significant_len();
    if len == 0 {
        return i32::MIN;
    }
    v.exponent + ((len - 1) * 64 + 63 - v.magnitude.word(len - 1).leading_zeros() as usize) as i32
}
