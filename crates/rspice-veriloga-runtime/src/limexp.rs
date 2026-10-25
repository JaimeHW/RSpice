//! The bounded exponentials: Verilog-A `limexp` and the CMC `lexp`.

/// The argument beyond which `limexp` stops being an exponential.
///
/// Not the LRM's number — the LRM does not give one. It is the figure the
/// SPICE estate settled on: Xyce's ADMS device headers define
/// `limexp(x) = x < 80 ? exp(x) : exp(80)*(x - 79)` under the comment "NOT
/// what verilog LRM says, but what qucs, ng-spice, and zspice do", and
/// OpenVAF lowers `limexp` to the same two-branch shape at `ln(1e30)`.
/// Every route in this workspace reads this constant; see
/// [`rspice_limexp`] for the one transcription of the function itself.
pub const LIMEXP_THRESHOLD: f64 = 80.0;

/// `exp(LIMEXP_THRESHOLD)`, which is both the value and the slope of
/// `limexp` at the threshold.
///
/// Spelled as a literal rather than computed, so that the value does not
/// depend on the host libm: it is bit-identical to `80.0f64.exp()` on every
/// platform this has been measured on, and the generated bundle's `noise.rs`
/// carries the same digits.
pub const LIMEXP_MAX: f64 = 5.540_622_384_393_51e34;

/// The Verilog-A `limexp` operator: `exp`, made linear past the threshold.
///
/// This is *the* transcription. The VM, the x64/AArch64/WASM JITs, the
/// canonical CFG interpreters and the generated Rust bundle all evaluate
/// `limexp` through this function or reproduce these two branches from these
/// two constants; a route that invents its own threshold is a defect, not a
/// tuning choice. Continuous in value and slope at the threshold, and
/// deliberately *unclamped* below it — a strongly reverse-biased junction
/// must be allowed to underflow to zero rather than be floored at some
/// `exp(-limit)`.
#[doc(hidden)]
#[inline(always)]
pub fn rspice_limexp(x: f64) -> f64 {
    if x < LIMEXP_THRESHOLD {
        x.exp()
    } else {
        LIMEXP_MAX * (x - LIMEXP_THRESHOLD + 1.0)
    }
}

/// The compact-model `lexp` threshold, clamped at *both* ends.
///
/// Numerically equal to [`LIMEXP_THRESHOLD`] and deliberately a separate
/// constant: `limexp` and the CMC bounded exponential are two operators with
/// two conventions that happen to agree today, and folding them onto one name
/// would make a future move to one of them silently move the other.
pub const LIMITED_EXP_THRESHOLD: f64 = 80.0;

/// What the CMC bounded exponential answers below `-LIMITED_EXP_THRESHOLD`.
///
/// The figure the CMC `lexp` macro carries, not `exp(-80)`.
pub const LIMITED_EXP_FLOOR: f64 = 1.804_851_387e-35;

#[doc(hidden)]
#[inline(always)]
pub fn rspice_limited_exp(x: f64) -> f64 {
    if x > LIMITED_EXP_THRESHOLD {
        LIMEXP_MAX * (x - LIMITED_EXP_THRESHOLD + 1.0)
    } else if x < -LIMITED_EXP_THRESHOLD {
        LIMITED_EXP_FLOOR
    } else {
        x.exp()
    }
}

#[doc(hidden)]
#[inline(always)]
pub fn rspice_limited_exp_derivative(x: f64) -> f64 {
    if x > LIMITED_EXP_THRESHOLD {
        LIMEXP_MAX
    } else if x < -LIMITED_EXP_THRESHOLD {
        0.0
    } else {
        x.exp()
    }
}
