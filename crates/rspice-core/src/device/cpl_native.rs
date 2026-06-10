//! Native CPL constants and runtime helpers.
//!
//! This is a direct, private port of the ngspice CPL setup math that produces
//! the CPLine constant tables (cplsetup.c), plus the transient convolution
//! runtime (cplload.c: right_consts/update_cnv/update_delayed_cnv/get_pvs_vi).
//! The runtime is driven by [`crate::device::CoupledTransmissionLine`].

use std::{collections::VecDeque, fmt};

use dd::Dd;

/// Compensated double-double (dd) arithmetic for the CPL setup chain.
///
/// The CPL modal-moment extraction (`polint` Neville interpolation +
/// `match_coefficients` divided differences feeding `pade_apx`) is
/// catastrophically ill-conditioned (condition ~1e16: eigenvalue samples of
/// magnitude ~50 must recover Pade coefficients down to ~1e-22). At that
/// conditioning, the ~3 ULP difference between an f64 eigensolve and a
/// gcc/FMA-contracted build flips the sign of the high-order moments and
/// throws the slow convolution poles off by 4-5%.
///
/// To make the setup deterministic and *accurate* (not merely matching either
/// compiler's roundoff), the entire setup chain runs in ~106-bit double-double
/// precision and only collapses to f64 at the final runtime-table construction.
///
/// `Dd` is a representation of a real value as an unevaluated sum `hi + lo` of
/// two non-overlapping f64s (Dekker / Knuth error-free transformations). The
/// primitives below are the standard ones from Hida/Li/Bailey "Library for
/// Double-Double and Quad-Double Arithmetic"; they are exact up to the final
/// rounding of each operation and do not rely on FMA contraction.
mod dd {
    /// Knuth's TwoSum: returns `(s, e)` with `s = fl(a + b)` and
    /// `a + b = s + e` exactly. Works for arbitrary magnitudes.
    #[inline]
    fn two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let bb = s - a;
        let err = (a - (s - bb)) + (b - bb);
        (s, err)
    }

    /// Dekker's quick TwoSum, valid only when `|a| >= |b|`.
    #[inline]
    fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
        let s = a + b;
        let err = b - (s - a);
        (s, err)
    }

    /// TwoProd via fused multiply-add: returns `(p, e)` with `p = fl(a*b)` and
    /// `a*b = p + e` exactly. `f64::mul_add` is a true FMA on supported targets
    /// (x86-64 with FMA, AArch64) and a correctly-rounded library fallback
    /// otherwise; either way the error term is exact, so the result is the true
    /// double-double product regardless of compiler contraction.
    #[inline]
    fn two_prod(a: f64, b: f64) -> (f64, f64) {
        let p = a * b;
        let err = a.mul_add(b, -p);
        (p, err)
    }

    /// Double-double: an unevaluated sum `hi + lo` of two non-overlapping f64s.
    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    pub(crate) struct Dd {
        pub hi: f64,
        pub lo: f64,
    }

    impl Dd {
        pub const ZERO: Dd = Dd { hi: 0.0, lo: 0.0 };

        #[inline]
        pub fn new(hi: f64, lo: f64) -> Dd {
            Dd { hi, lo }
        }

        #[inline]
        pub fn from_f64(x: f64) -> Dd {
            Dd { hi: x, lo: 0.0 }
        }

        /// Round to the nearest f64.
        #[inline]
        pub fn to_f64(self) -> f64 {
            self.hi + self.lo
        }

        #[inline]
        pub fn is_zero(self) -> bool {
            self.hi == 0.0
        }

        #[inline]
        pub fn neg(self) -> Dd {
            Dd {
                hi: -self.hi,
                lo: -self.lo,
            }
        }

        #[inline]
        pub fn abs(self) -> Dd {
            if self.hi < 0.0 { self.neg() } else { self }
        }

        #[inline]
        pub fn add(self, b: Dd) -> Dd {
            // Knuth two-sum on the hi parts, then fold both lo parts in.
            let (s, e) = two_sum(self.hi, b.hi);
            let e = e + self.lo + b.lo;
            let (hi, lo) = quick_two_sum(s, e);
            Dd { hi, lo }
        }

        #[inline]
        pub fn add_f64(self, b: f64) -> Dd {
            let (s, e) = two_sum(self.hi, b);
            let e = e + self.lo;
            let (hi, lo) = quick_two_sum(s, e);
            Dd { hi, lo }
        }

        #[inline]
        pub fn sub(self, b: Dd) -> Dd {
            self.add(b.neg())
        }

        // Used by the dd unit tests; kept for API completeness alongside the
        // other `*_f64` helpers.
        #[cfg(test)]
        #[inline]
        pub fn sub_f64(self, b: f64) -> Dd {
            self.add_f64(-b)
        }

        #[inline]
        pub fn mul(self, b: Dd) -> Dd {
            let (p, e) = two_prod(self.hi, b.hi);
            let e = e + (self.hi * b.lo + self.lo * b.hi);
            let (hi, lo) = quick_two_sum(p, e);
            Dd { hi, lo }
        }

        #[inline]
        pub fn mul_f64(self, b: f64) -> Dd {
            let (p, e) = two_prod(self.hi, b);
            let e = e + self.lo * b;
            let (hi, lo) = quick_two_sum(p, e);
            Dd { hi, lo }
        }

        #[inline]
        pub fn div(self, b: Dd) -> Dd {
            // Long division: successive correction steps in dd.
            let q1 = self.hi / b.hi;
            let r = self.sub(b.mul_f64(q1));
            let q2 = r.hi / b.hi;
            let r = r.sub(b.mul_f64(q2));
            let q3 = r.hi / b.hi;
            let (hi, lo) = quick_two_sum(q1, q2);
            Dd { hi, lo }.add_f64(q3)
        }

        #[inline]
        pub fn div_f64(self, b: f64) -> Dd {
            self.div(Dd::from_f64(b))
        }

        #[inline]
        pub fn sqrt(self) -> Dd {
            if self.hi == 0.0 {
                return Dd::ZERO;
            }
            // Karp/Markstein refinement (Hida/Li/Bailey QD `sqrt`):
            //   x  ~ 1/sqrt(a)   (f64)
            //   ax = a * x       (~ sqrt(a))
            //   sqrt(a) ~ ax + (a - ax^2) * (x/2)
            // with `ax^2` evaluated in full dd so the residual is accurate.
            let x = 1.0 / self.hi.sqrt();
            let ax = self.hi * x;
            let ax_dd = Dd::from_f64(ax);
            let diff = self.sub(ax_dd.mul(ax_dd));
            let e = diff.mul_f64(x * 0.5);
            ax_dd.add(e)
        }

        /// Multiply by an integer power of two — exact, no rounding.
        #[inline]
        fn ldexp(self, n: i32) -> Dd {
            let scale = 2.0f64.powi(n);
            Dd {
                hi: self.hi * scale,
                lo: self.lo * scale,
            }
        }

        /// e^x in double-double precision (Hida/Li/Bailey QD `exp`):
        /// argument reduction `x = k*ln2 + r` with `|r| <= ln2/2`, then a
        /// further halving so the Taylor series for `e^r` converges fast, then
        /// squaring back up and scaling by `2^k`.
        pub fn exp(self) -> Dd {
            if self.hi == 0.0 {
                return Dd::from_f64(1.0);
            }
            if self.hi <= -709.0 {
                return Dd::ZERO;
            }
            // ln(2) in double-double.
            let ln2 = Dd::new(0.6931471805599453, 2.3190468138462996e-17);
            let inv_ln2 = 1.0 / 0.6931471805599453;
            let k = (self.hi * inv_ln2 + 0.5).floor();
            let r = self.sub(ln2.mul_f64(k));

            // Halve r `m` times to speed Taylor convergence.
            let m = 9i32;
            let r = r.ldexp(-m);

            // Taylor series sum_{i>=1} r^i / i!  (so e^r = 1 + series).
            let mut term = r;
            let mut sum = r;
            let mut i = 2.0f64;
            for _ in 0..18 {
                term = term.mul(r).div_f64(i);
                let new_sum = sum.add(term);
                if new_sum.hi == sum.hi && new_sum.lo == sum.lo {
                    sum = new_sum;
                    break;
                }
                sum = new_sum;
                i += 1.0;
            }

            // Undo the halvings: e^r = (1 + s)^(2^m). Repeatedly square,
            // tracking the series part s where (1+s)^2 = 1 + (2s + s^2).
            let mut s = sum;
            for _ in 0..m {
                s = s.mul_f64(2.0).add(s.mul(s));
            }
            let e_r = s.add_f64(1.0);

            e_r.ldexp(k as i32)
        }

        #[inline]
        pub fn lt(self, b: Dd) -> bool {
            self.hi < b.hi || (self.hi == b.hi && self.lo < b.lo)
        }

        #[inline]
        pub fn gt(self, b: Dd) -> bool {
            self.hi > b.hi || (self.hi == b.hi && self.lo > b.lo)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn two_sum_is_exact() {
            let a = 1.0;
            let b = 1.0e-20;
            let (s, e) = two_sum(a, b);
            // The tiny addend is fully captured in the error term.
            assert_eq!(s, 1.0);
            assert_eq!(e, 1.0e-20);
        }

        #[test]
        fn two_prod_error_free() {
            let a = 1.0 + 2.0f64.powi(-20);
            let b = 1.0 - 2.0f64.powi(-20);
            let (p, e) = two_prod(a, b);
            // p+e reconstructs the exact product 1 - 2^-40 with no f64 roundoff.
            let exact = 1.0 - 2.0f64.powi(-40);
            assert_eq!(p + e, exact);
        }

        #[test]
        fn dd_add_beats_f64() {
            // 1 + 1e-20 - 1 == 0 in f64, == 1e-20 in dd.
            let r = Dd::from_f64(1.0).add_f64(1.0e-20).sub_f64(1.0);
            assert!((r.to_f64() - 1.0e-20).abs() < 1.0e-35);
            assert_eq!(1.0f64 + 1.0e-20 - 1.0, 0.0);
        }

        #[test]
        fn dd_mul_associativity_precision() {
            // (1/3 in dd) * 3 reconstructs 1 to far better than f64.
            let third = Dd::from_f64(1.0).div_f64(3.0);
            let one = third.mul_f64(3.0);
            assert!((one.to_f64() - 1.0).abs() <= f64::EPSILON);
            // dd's stored value is accurate well below an f64 ulp of 1.
            let err = one.sub_f64(1.0).to_f64().abs();
            assert!(err < 1.0e-30, "err={err:e}");
        }

        #[test]
        fn dd_div_roundtrip() {
            let a = Dd::from_f64(2.0).add_f64(1.0e-18);
            let b = Dd::from_f64(7.0).add_f64(-3.0e-19);
            let q = a.div(b);
            let back = q.mul(b);
            let err = back.sub(a).to_f64().abs();
            assert!(err < 1.0e-30, "err={err:e}");
        }

        #[test]
        fn dd_sqrt_accurate() {
            let two = Dd::from_f64(2.0);
            let r = two.sqrt();
            let sq = r.mul(r);
            let err = sq.sub(two).to_f64().abs();
            assert!(err < 1.0e-30, "err={err:e}");
            // hi is within one ulp of the f64 sqrt; lo carries the correction.
            assert!((r.hi - 2.0f64.sqrt()).abs() <= f64::EPSILON);
        }

        #[test]
        fn dd_exp_accurate() {
            // e^1 to dd precision: compare against the known constant e and its
            // tail, and check e^x * e^-x == 1.
            let one = Dd::from_f64(1.0);
            let e = one.exp();
            // e = 2.718281828459045235360287...; hi is the f64 of e.
            assert!((e.hi - std::f64::consts::E).abs() <= f64::EPSILON);
            let e_full = Dd::new(
                std::f64::consts::E,
                1.4456468917292502e-16, // tail of e beyond the f64 mantissa
            );
            let err = e.sub(e_full).to_f64().abs();
            assert!(err < 1.0e-30, "err={err:e}");

            for &x in &[0.5, -3.25, 12.0, -0.001, 50.0] {
                let lhs = Dd::from_f64(x).exp().mul(Dd::from_f64(-x).exp());
                assert!((lhs.to_f64() - 1.0).abs() < 1.0e-13, "x={x}");
            }
            assert_eq!(Dd::ZERO.exp().to_f64(), 1.0);
        }

        #[test]
        fn dd_compare() {
            let a = Dd::from_f64(1.0).add_f64(1.0e-20);
            let b = Dd::from_f64(1.0);
            assert!(a.gt(b));
            assert!(b.lt(a));
            assert!(!a.lt(b));
        }
    }
}

const MAX_CP_TX_LINES: usize = 8;
const MAX_DEG: usize = 8;
const LEFT_DEG: usize = 7;
const CPL_MIN_SERIES_RESISTANCE_PER_LENGTH: f64 = 1.0e-4;
const EPSILON: f64 = 1.0e-88;
const EPSI_MULT: f64 = 1.0e-28;
const EPSI2: f64 = 1.0e-8;

type Matrix = Vec<Vec<f64>>;

// Double-double counterparts used throughout the setup chain. The runtime
// tables remain f64 (see `Matrix`); only the once-per-line setup math runs in
// dd, collapsing to f64 at `into_runtime`.
type DdMatrix = Vec<Vec<Dd>>;
type DdPolyMatrix = Vec<Vec<Vec<Dd>>>;

fn dd_zero_matrix(dim: usize) -> DdMatrix {
    vec![vec![Dd::ZERO; dim]; dim]
}

fn dd_zero_poly_matrix(dim: usize, poly_len: usize) -> DdPolyMatrix {
    vec![vec![vec![Dd::ZERO; poly_len]; dim]; dim]
}

fn dd_matrix_from_f64(matrix: &Matrix) -> DdMatrix {
    matrix
        .iter()
        .map(|row| row.iter().map(|&v| Dd::from_f64(v)).collect())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NativeCplError {
    InvalidDimension(usize),
    InvalidLength(f64),
    InvalidMatrix {
        label: &'static str,
        expected: usize,
    },
    NonFiniteMatrix {
        label: &'static str,
        row: usize,
        col: usize,
    },
    NonPositiveCapacitanceMatrix,
    NonPositiveModeFrequency,
    SingularMatrix(&'static str),
    InterpolationFailure,
    DiagonalizationDidNotConverge,
    InvalidHistoryDimension {
        label: &'static str,
        expected: usize,
        actual: usize,
    },
    NonMonotonicHistory {
        previous_ps: i64,
        next_ps: i64,
    },
    EmptyHistory,
    InvalidHistoryTimeStep {
        previous_ps: i64,
        current_ps: i64,
    },
    InvalidTransientStep(f64),
    InsufficientHistory {
        target_ps: f64,
    },
    NonFiniteHistoryValue {
        label: &'static str,
        index: usize,
    },
}

impl fmt::Display for NativeCplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimension(dim) => write!(
                f,
                "native CPL setup requires 2..={MAX_CP_TX_LINES} conductors, found {dim}"
            ),
            Self::InvalidLength(length) => {
                write!(
                    f,
                    "native CPL setup requires positive finite length, found {length}"
                )
            }
            Self::InvalidMatrix { label, expected } => {
                write!(f, "native CPL {label} matrix must be {expected}x{expected}")
            }
            Self::NonFiniteMatrix { label, row, col } => write!(
                f,
                "native CPL {label}[{},{}] is not finite",
                row + 1,
                col + 1
            ),
            Self::NonPositiveCapacitanceMatrix => write!(
                f,
                "native CPL capacitance/conductance matrix is not positive definite"
            ),
            Self::NonPositiveModeFrequency => {
                write!(f, "native CPL mode frequency is not positive")
            }
            Self::SingularMatrix(label) => write!(f, "native CPL singular matrix in {label}"),
            Self::InterpolationFailure => write!(f, "native CPL polynomial interpolation failed"),
            Self::DiagonalizationDidNotConverge => {
                write!(f, "native CPL diagonalization did not converge")
            }
            Self::InvalidHistoryDimension {
                label,
                expected,
                actual,
            } => write!(
                f,
                "native CPL {label} history vector must have {expected} entries, found {actual}"
            ),
            Self::NonMonotonicHistory {
                previous_ps,
                next_ps,
            } => write!(
                f,
                "native CPL VI history times must increase strictly ({previous_ps} ps then {next_ps} ps)"
            ),
            Self::EmptyHistory => write!(f, "native CPL VI history is empty"),
            Self::InvalidHistoryTimeStep {
                previous_ps,
                current_ps,
            } => write!(
                f,
                "native CPL history sampling requires current time > previous time ({previous_ps} ps, {current_ps} ps)"
            ),
            Self::InvalidTransientStep(step) => write!(
                f,
                "native CPL transient step must be positive and finite, found {step}"
            ),
            Self::InsufficientHistory { target_ps } => write!(
                f,
                "native CPL VI history does not bracket delayed time {target_ps:.6} ps"
            ),
            Self::NonFiniteHistoryValue { label, index } => write!(
                f,
                "native CPL {label} history vector entry {} is not finite",
                index + 1
            ),
        }
    }
}

impl std::error::Error for NativeCplError {}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct NativeCplTerm {
    pub(crate) c: f64,
    pub(crate) x: f64,
    pub(crate) cnv_i: f64,
    pub(crate) cnv_o: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NativeCplTimeSeries {
    pub(crate) if_img: bool,
    pub(crate) aten: f64,
    pub(crate) tm: [NativeCplTerm; 3],
}

impl NativeCplTimeSeries {
    /// Build the runtime series from the dd-precision Pade output and dd
    /// attenuation constant. The `c = pade[i] * atten` products are formed in dd
    /// and only then rounded to f64, so the runtime poles inherit the accurate
    /// setup values rather than an f64-roundoff intermediate.
    fn from_pade_dd(atten: Dd, pade: &[Dd]) -> Self {
        Self {
            if_img: ((pade[6].to_f64() - 1.0) as i32) != 0,
            aten: atten.to_f64(),
            tm: [
                NativeCplTerm {
                    c: pade[0].mul(atten).to_f64(),
                    x: pade[3].to_f64(),
                    cnv_i: 0.0,
                    cnv_o: 0.0,
                },
                NativeCplTerm {
                    c: pade[1].mul(atten).to_f64(),
                    x: pade[4].to_f64(),
                    cnv_i: 0.0,
                    cnv_o: 0.0,
                },
                NativeCplTerm {
                    c: pade[2].mul(atten).to_f64(),
                    x: pade[5].to_f64(),
                    cnv_i: 0.0,
                    cnv_o: 0.0,
                },
            ],
        }
    }

    fn constant(&self) -> f64 {
        if self.if_img {
            self.tm[0].c + 2.0 * self.tm[1].c
        } else {
            self.tm.iter().map(|term| term.c).sum()
        }
    }

    fn cnv_i_sum(&self) -> f64 {
        if self.if_img {
            self.tm[0].cnv_i + 2.0 * self.tm[1].cnv_i
        } else {
            self.tm.iter().map(|term| term.cnv_i).sum()
        }
    }

    fn cnv_o_sum(&self) -> f64 {
        if self.if_img {
            self.tm[0].cnv_o + 2.0 * self.tm[1].cnv_o
        } else {
            self.tm.iter().map(|term| term.cnv_o).sum()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeCplRuntime {
    pub(crate) no_l: usize,
    pub(crate) taul_ps: Vec<f64>,
    pub(crate) h1t: Vec<Vec<Option<NativeCplTimeSeries>>>,
    pub(crate) h2t: Vec<Vec<Vec<Option<NativeCplTimeSeries>>>>,
    pub(crate) h3t: Vec<Vec<Vec<Option<NativeCplTimeSeries>>>>,
    pub(crate) h1c: Matrix,
    pub(crate) h2c: Vec<Matrix>,
    pub(crate) h3c: Vec<Matrix>,
    pub(crate) h1e: Vec<Vec<[f64; 3]>>,
}

impl NativeCplRuntime {
    pub(crate) fn setup(
        r: &[Vec<f64>],
        l: &[Vec<f64>],
        c: &[Vec<f64>],
        g: &[Vec<f64>],
        length: f64,
    ) -> Result<Self, NativeCplError> {
        let mut setup = NativeCplSetup::new(r, l, c, g, length)?;
        setup.coupled()?;
        setup.into_runtime()
    }

    fn right_consts(
        &mut self,
        h_seconds: f64,
        h1_seconds: f64,
        input_voltage: &[f64],
        output_voltage: &[f64],
        delayed: &NativeCplDelayedVi,
    ) -> Result<NativeCplRightConsts, NativeCplError> {
        validate_history_vector("input voltage", input_voltage, self.no_l)?;
        validate_history_vector("output voltage", output_voltage, self.no_l)?;
        validate_delayed_vi(delayed, self.no_l)?;

        let mut ff = vec![0.0; self.no_l];
        let mut gg = vec![0.0; self.no_l];

        for row in 0..self.no_l {
            for col in 0..self.no_l {
                let Some(tms) = self.h1t[row][col].as_ref() else {
                    continue;
                };

                if tms.if_img {
                    let e = (tms.tm[0].x * h_seconds).exp();
                    let (er, ei) = exp_complex(tms.tm[1].x, tms.tm[2].x, h_seconds);
                    self.h1e[row][col] = [e, er, ei];

                    let ff1 = tms.tm[0].c * e * h1_seconds;
                    ff[row] -= tms.tm[0].cnv_i * e;
                    gg[row] -= tms.tm[0].cnv_o * e;
                    ff[row] -= ff1 * input_voltage[col];
                    gg[row] -= ff1 * output_voltage[col];

                    let (a1, _) = mult_complex(tms.tm[1].c, tms.tm[2].c, er, ei);
                    let (a, _) = mult_complex(tms.tm[1].cnv_i, tms.tm[2].cnv_i, er, ei);
                    ff[row] -= 2.0 * (a1 * h1_seconds * input_voltage[col] + a);
                    let (a, _) = mult_complex(tms.tm[1].cnv_o, tms.tm[2].cnv_o, er, ei);
                    gg[row] -= 2.0 * (a1 * h1_seconds * output_voltage[col] + a);
                } else {
                    let mut ff1 = 0.0;
                    for pole in 0..3 {
                        let e = (tms.tm[pole].x * h_seconds).exp();
                        self.h1e[row][col][pole] = e;
                        ff1 -= tms.tm[pole].c * e;
                        ff[row] -= tms.tm[pole].cnv_i * e;
                        gg[row] -= tms.tm[pole].cnv_o * e;
                    }
                    ff[row] += ff1 * h1_seconds * input_voltage[col];
                    gg[row] += ff1 * h1_seconds * output_voltage[col];
                }
            }
        }

        for row in 0..self.no_l {
            for col in 0..self.no_l {
                for mode in 0..self.no_l {
                    if let Some(tms) = self.h3t[row][col][mode].as_mut() {
                        update_rhs_time_series(
                            tms,
                            h_seconds,
                            h1_seconds,
                            delayed.v1_i[mode][col],
                            delayed.v2_i[mode][col],
                            delayed.v1_o[mode][col],
                            delayed.v2_o[mode][col],
                        );
                        ff[row] += tms.aten * delayed.v2_o[mode][col] + tms.cnv_o_sum();
                        gg[row] += tms.aten * delayed.v2_i[mode][col] + tms.cnv_i_sum();
                    }

                    if let Some(tms) = self.h2t[row][col][mode].as_mut() {
                        update_rhs_time_series(
                            tms,
                            h_seconds,
                            h1_seconds,
                            delayed.i1_i[mode][col],
                            delayed.i2_i[mode][col],
                            delayed.i1_o[mode][col],
                            delayed.i2_o[mode][col],
                        );
                        ff[row] += tms.aten * delayed.i2_o[mode][col] + tms.cnv_o_sum();
                        gg[row] += tms.aten * delayed.i2_i[mode][col] + tms.cnv_i_sum();
                    }
                }
            }
        }

        Ok(NativeCplRightConsts {
            ext: delayed.ext,
            ratio: delayed.ratio.clone(),
            ff,
            gg,
        })
    }

    /// Compute the per-step MNA stamp for the branch-current convolution
    /// equations, mirroring the second loop of ngspice `CPLload`.
    ///
    /// `self` must hold the accepted convolution state advanced up to `t1`
    /// (the start of the current step). This call clones that state internally
    /// so the accepted state is not perturbed (ngspice restores `cp2` from `cp`
    /// before `right_consts`), then evaluates the RHS constants and the matrix
    /// coefficient matrices used to stamp the branch rows.
    ///
    /// - `t1_ps`/`t2_ps` are the integer-picosecond start/end of the step.
    /// - `dt_seconds` is the step `h` (= CKTdelta).
    /// - `input_voltage`/`output_voltage` are the latest accepted near/far port
    ///   voltages (ngspice `in_node->V`/`out_node->V`, the committed t1 values).
    pub(crate) fn step_stamp_plan(
        &self,
        t1_ps: i64,
        t2_ps: i64,
        dt_seconds: f64,
        input_voltage: &[f64],
        output_voltage: &[f64],
        history: &mut NativeCplViHistory,
    ) -> Result<NativeCplStampPlan, NativeCplError> {
        let h = dt_seconds;
        let h1 = 0.5 * dt_seconds;
        let delayed = history.delayed_vi_samples_ps(t1_ps, t2_ps, &self.taul_ps)?;

        // ngspice runs right_consts on the throwaway working copy `cp2`; clone
        // so the accepted convolution state is preserved for the next commit.
        let mut working = self.clone();
        let rc = working.right_consts(h, h1, input_voltage, output_voltage, &delayed)?;

        let n = self.no_l;
        let mut aten_h1 = zero_matrix(n);
        let mut f2 = zero_matrix(n);
        let mut f3 = zero_matrix(n);

        for m in 0..n {
            for p in 0..n {
                if let Some(tms) = self.h1t[m][p].as_ref() {
                    aten_h1[m][p] = tms.aten + h1 * self.h1c[m][p];
                }
                if rc.ext {
                    for q in 0..n {
                        let ratio = rc.ratio[q];
                        if ratio <= 0.0 {
                            continue;
                        }
                        if let Some(tms) = self.h3t[m][p][q].as_ref() {
                            f3[m][p] += ratio * (h1 * self.h3c[m][p][q] + tms.aten);
                        }
                        if let Some(tms) = self.h2t[m][p][q].as_ref() {
                            f2[m][p] += ratio * (h1 * self.h2c[m][p][q] + tms.aten);
                        }
                    }
                }
            }
        }

        Ok(NativeCplStampPlan {
            ext: rc.ext,
            ff: rc.ff,
            gg: rc.gg,
            aten_h1,
            f2,
            f3,
        })
    }

    /// Commit an accepted step into the persistent convolution state, mirroring
    /// the per-load lifecycle of ngspice `CPLload`.
    ///
    /// ngspice carries each step's `right_consts` result forward (via the
    /// `copy_cp(cp, cp2)` round-trip) and then layers the `update_cnv` /
    /// `update_delayed_cnv` increments on top. This call reproduces that exact
    /// ordering on `self` (the accepted state):
    ///   1. `right_consts` advances the `h2`/`h3` delayed-pole convolutions for
    ///      the just-completed step `[t1, t2]` and records the `h1` exponentials.
    ///   2. `update_accepted_voltage_convolutions` advances the `h1` poles using
    ///      the accepted slope across the step.
    ///   3. `update_delayed_convolutions` adds the extrapolation tail when the
    ///      step's delayed samples reached past the previous accepted time.
    ///
    /// - `t1_ps`/`t2_ps`: integer-picosecond start/end of the accepted step.
    /// - `dt_seconds`: the accepted step `h`.
    /// - `start_near`/`start_far`: port voltages at `t1` (ngspice `in_node->V`).
    /// - `end_near`/`end_far`/`end_near_i`/`end_far_i`: accepted port voltages
    ///   and branch currents at `t2`.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn commit_step(
        &mut self,
        t1_ps: i64,
        t2_ps: i64,
        h_exp_seconds: f64,
        h_grid_seconds: f64,
        start_near: &[f64],
        start_far: &[f64],
        end_near: &[f64],
        end_far: &[f64],
        end_near_i: &[f64],
        end_far_i: &[f64],
        history: &mut NativeCplViHistory,
    ) -> Result<(), NativeCplError> {
        let delayed = history.delayed_vi_samples_ps(t1_ps, t2_ps, &self.taul_ps)?;

        // (1) Advance h2/h3 (and record h1e) for the just-completed step. This
        // is the persistent counterpart of the step's right_consts on cp2 and
        // runs entirely on the solver's fractional step (cplload.c keeps the
        // exponentials on CKTdelta while the grid spans are integer
        // picoseconds).
        let rc = self.right_consts(
            h_exp_seconds,
            0.5 * h_exp_seconds,
            start_near,
            start_far,
            &delayed,
        )?;

        // (2) Advance the h1 poles: exponentials on the fractional step, the
        // accepted slope across the integer-picosecond span.
        self.update_accepted_voltage_convolutions(
            h_exp_seconds,
            h_grid_seconds,
            start_near,
            end_near,
            start_far,
            end_far,
        )?;

        // (3) Add the delayed extrapolation tail when the step was external
        // (grid span, matching ngspice's `h *= 0.5e-12` on the integer delta).
        if rc.ext {
            let tail = NativeCplViSample::new(
                t2_ps,
                end_near.to_vec(),
                end_far.to_vec(),
                end_near_i.to_vec(),
                end_far_i.to_vec(),
            );
            self.update_delayed_convolutions(h_grid_seconds, &rc.ratio, &tail)?;
        }

        Ok(())
    }

    pub(crate) fn initialize_dc_convolutions(
        &mut self,
        input_dc: &[f64],
        output_dc: &[f64],
    ) -> Result<(), NativeCplError> {
        validate_history_vector("input dc voltage", input_dc, self.no_l)?;
        validate_history_vector("output dc voltage", output_dc, self.no_l)?;

        for row in 0..self.no_l {
            for col in 0..self.no_l {
                if let Some(tms) = self.h1t[row][col].as_mut() {
                    initialize_dc_time_series(tms, input_dc[col], output_dc[col]);
                }

                for mode in 0..self.no_l {
                    if let Some(tms) = self.h2t[row][col][mode].as_mut() {
                        zero_time_series_convolutions(tms);
                    }
                    if let Some(tms) = self.h3t[row][col][mode].as_mut() {
                        initialize_dc_time_series(tms, input_dc[col], output_dc[col]);
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) fn update_accepted_voltage_convolutions(
        &mut self,
        h_exp_seconds: f64,
        h_grid_seconds: f64,
        previous_input_voltage: &[f64],
        current_input_voltage: &[f64],
        previous_output_voltage: &[f64],
        current_output_voltage: &[f64],
    ) -> Result<(), NativeCplError> {
        validate_positive_step(h_exp_seconds)?;
        validate_positive_step(h_grid_seconds)?;
        validate_history_vector("previous input voltage", previous_input_voltage, self.no_l)?;
        validate_history_vector("current input voltage", current_input_voltage, self.no_l)?;
        validate_history_vector(
            "previous output voltage",
            previous_output_voltage,
            self.no_l,
        )?;
        validate_history_vector("current output voltage", current_output_voltage, self.no_l)?;

        for row in 0..self.no_l {
            for col in 0..self.no_l {
                let Some(tms) = self.h1t[row][col].as_mut() else {
                    continue;
                };

                let previous_input = previous_input_voltage[col];
                let current_input = current_input_voltage[col];
                let previous_output = previous_output_voltage[col];
                let current_output = current_output_voltage[col];

                if tms.if_img {
                    let e = (tms.tm[0].x * h_exp_seconds).exp();
                    let (er, ei) = exp_complex(tms.tm[1].x, tms.tm[2].x, h_exp_seconds);
                    update_accepted_complex_time_series(
                        tms,
                        h_grid_seconds,
                        previous_input,
                        current_input,
                        previous_output,
                        current_output,
                        er,
                        ei,
                    );
                    update_accepted_real_term(
                        &mut tms.tm[0],
                        h_grid_seconds,
                        previous_input,
                        current_input,
                        previous_output,
                        current_output,
                        e,
                    );
                    self.h1e[row][col] = [e, er, ei];
                } else {
                    let mut exponentials = [0.0; 3];
                    let mut input_slope = (current_input - previous_input) / h_grid_seconds;
                    let mut output_slope = (current_output - previous_output) / h_grid_seconds;
                    for (pole, term) in tms.tm.iter_mut().enumerate() {
                        let e = (term.x * h_exp_seconds).exp();
                        exponentials[pole] = e;
                        let scale = term.c / term.x;
                        input_slope *= scale;
                        output_slope *= scale;
                        term.cnv_i = (term.cnv_i - input_slope * h_grid_seconds) * e
                            + (e - 1.0) * (current_input * scale + input_slope / term.x);
                        term.cnv_o = (term.cnv_o - output_slope * h_grid_seconds) * e
                            + (e - 1.0) * (current_output * scale + output_slope / term.x);
                    }
                    self.h1e[row][col] = exponentials;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn update_delayed_convolutions(
        &mut self,
        h_seconds: f64,
        ratio: &[f64],
        tail: &NativeCplViSample,
    ) -> Result<(), NativeCplError> {
        validate_positive_step(h_seconds)?;
        validate_history_vector("delay ratio", ratio, self.no_l)?;
        validate_history_vector("tail v_i", &tail.v_i, self.no_l)?;
        validate_history_vector("tail v_o", &tail.v_o, self.no_l)?;
        validate_history_vector("tail i_i", &tail.i_i, self.no_l)?;
        validate_history_vector("tail i_o", &tail.i_o, self.no_l)?;

        let h_half = 0.5 * h_seconds;
        for (mode, ratio) in ratio.iter().copied().enumerate() {
            if ratio <= 0.0 {
                continue;
            }
            for row in 0..self.no_l {
                for col in 0..self.no_l {
                    if let Some(tms) = self.h3t[row][col][mode].as_mut() {
                        add_delayed_convolution(tms, h_half * ratio, tail.v_i[col], tail.v_o[col]);
                    }
                    if let Some(tms) = self.h2t[row][col][mode].as_mut() {
                        add_delayed_convolution(tms, h_half * ratio, tail.i_i[col], tail.i_o[col]);
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct NativeCplRightConsts {
    ext: bool,
    ratio: Vec<f64>,
    ff: Vec<f64>,
    gg: Vec<f64>,
}

/// Per-step branch-current MNA stamp for a coupled line, mirroring the matrix
/// pointers and RHS constants written by ngspice `CPLload`.
///
/// Indices `[m][p]` map to (branch-equation conductor `m`, port conductor `p`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeCplStampPlan {
    /// Whether the delayed (`h2`/`h3`) terms are active this step.
    pub(crate) ext: bool,
    /// RHS for the near-end (`ibr1`) branch rows.
    pub(crate) ff: Vec<f64>,
    /// RHS for the far-end (`ibr2`) branch rows.
    pub(crate) gg: Vec<f64>,
    /// `h1t.aten + h1*h1C[m][p]`: ibr1->Vpos[p] and ibr2->Vneg[p] coefficient.
    pub(crate) aten_h1: Matrix,
    /// `sum_q ratio[q]*(h1*h2C[m][p][q] + h2t.aten)`: ibr1->ibr2[p] / ibr2->ibr1[p].
    pub(crate) f2: Matrix,
    /// `sum_q ratio[q]*(h1*h3C[m][p][q] + h3t.aten)`: ibr1->Vneg[p] / ibr2->Vpos[p].
    pub(crate) f3: Matrix,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeCplViSample {
    pub(crate) time_ps: i64,
    pub(crate) v_i: Vec<f64>,
    pub(crate) v_o: Vec<f64>,
    pub(crate) i_i: Vec<f64>,
    pub(crate) i_o: Vec<f64>,
}

impl NativeCplViSample {
    pub(crate) fn new(
        time_ps: i64,
        v_i: Vec<f64>,
        v_o: Vec<f64>,
        i_i: Vec<f64>,
        i_o: Vec<f64>,
    ) -> Self {
        Self {
            time_ps,
            v_i,
            v_o,
            i_i,
            i_o,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeCplDelayedVi {
    pub(crate) ext: bool,
    pub(crate) ratio: Vec<f64>,
    pub(crate) v1_i: Matrix,
    pub(crate) v2_i: Matrix,
    pub(crate) i1_i: Matrix,
    pub(crate) i2_i: Matrix,
    pub(crate) v1_o: Matrix,
    pub(crate) v2_o: Matrix,
    pub(crate) i1_o: Matrix,
    pub(crate) i2_o: Matrix,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeCplViHistory {
    no_l: usize,
    dc1: Vec<f64>,
    dc2: Vec<f64>,
    samples: VecDeque<NativeCplViSample>,
}

impl NativeCplViHistory {
    pub(crate) fn new(no_l: usize, dc1: Vec<f64>, dc2: Vec<f64>) -> Result<Self, NativeCplError> {
        if !(2..=MAX_CP_TX_LINES).contains(&no_l) {
            return Err(NativeCplError::InvalidDimension(no_l));
        }
        validate_history_vector("dc1", &dc1, no_l)?;
        validate_history_vector("dc2", &dc2, no_l)?;
        Ok(Self {
            no_l,
            dc1,
            dc2,
            samples: VecDeque::new(),
        })
    }

    pub(crate) fn push_sample(&mut self, sample: NativeCplViSample) -> Result<(), NativeCplError> {
        validate_history_vector("v_i", &sample.v_i, self.no_l)?;
        validate_history_vector("v_o", &sample.v_o, self.no_l)?;
        validate_history_vector("i_i", &sample.i_i, self.no_l)?;
        validate_history_vector("i_o", &sample.i_o, self.no_l)?;
        if let Some(previous) = self.samples.back() {
            if sample.time_ps <= previous.time_ps {
                return Err(NativeCplError::NonMonotonicHistory {
                    previous_ps: previous.time_ps,
                    next_ps: sample.time_ps,
                });
            }
        }
        self.samples.push_back(sample);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn head_time_ps(&self) -> Option<i64> {
        self.samples.front().map(|sample| sample.time_ps)
    }

    pub(crate) fn delayed_vi_samples_ps(
        &mut self,
        previous_time_ps: i64,
        current_time_ps: i64,
        taul_ps: &[f64],
    ) -> Result<NativeCplDelayedVi, NativeCplError> {
        if current_time_ps <= previous_time_ps {
            return Err(NativeCplError::InvalidHistoryTimeStep {
                previous_ps: previous_time_ps,
                current_ps: current_time_ps,
            });
        }
        validate_history_vector("taul", taul_ps, self.no_l)?;
        if self.samples.is_empty() {
            return Err(NativeCplError::EmptyHistory);
        }

        let mut result = NativeCplDelayedVi {
            ext: false,
            ratio: vec![0.0; self.no_l],
            v1_i: zero_matrix(self.no_l),
            v2_i: zero_matrix(self.no_l),
            i1_i: zero_matrix(self.no_l),
            i2_i: zero_matrix(self.no_l),
            v1_o: zero_matrix(self.no_l),
            v2_o: zero_matrix(self.no_l),
            i1_o: zero_matrix(self.no_l),
            i2_o: zero_matrix(self.no_l),
        };

        let previous_time = previous_time_ps as f64;
        let current_time = current_time_ps as f64;
        let step = current_time - previous_time;
        let mut prune_to = None;
        let mut min_ta = f64::INFINITY;

        for delay_ps in taul_ps {
            let ta = previous_time - delay_ps;
            if ta < min_ta {
                min_ta = ta;
                prune_to = if ta > 0.0 {
                    Some(self.bracket_lower_index(ta)?)
                } else {
                    None
                };
            }
        }

        for (mode, delay_ps) in taul_ps.iter().enumerate() {
            let ta = previous_time - delay_ps;
            let tb = current_time - delay_ps;

            if tb <= 0.0 {
                for conductor in 0..self.no_l {
                    result.v1_i[mode][conductor] = self.dc1[conductor];
                    result.v2_i[mode][conductor] = self.dc1[conductor];
                    result.v1_o[mode][conductor] = self.dc2[conductor];
                    result.v2_o[mode][conductor] = self.dc2[conductor];
                }
                continue;
            }

            if ta <= 0.0 {
                for conductor in 0..self.no_l {
                    result.v1_i[mode][conductor] = self.dc1[conductor];
                    result.v1_o[mode][conductor] = self.dc2[conductor];
                }
            } else {
                let lower = self.bracket_lower_index(ta)?;
                self.interpolate_sample(lower, ta, |conductor, value| {
                    result.v1_i[mode][conductor] = value.v_i;
                    result.v1_o[mode][conductor] = value.v_o;
                    result.i1_i[mode][conductor] = value.i_i;
                    result.i1_o[mode][conductor] = value.i_o;
                })?;
            }

            if tb > previous_time {
                result.ext = true;
                result.ratio[mode] = (tb - previous_time) / step;
                let scale = 1.0 - result.ratio[mode];
                let tail = self.samples.back().ok_or(NativeCplError::EmptyHistory)?;
                for conductor in 0..self.no_l {
                    result.v2_i[mode][conductor] = tail.v_i[conductor] * scale;
                    result.v2_o[mode][conductor] = tail.v_o[conductor] * scale;
                    result.i2_i[mode][conductor] = tail.i_i[conductor] * scale;
                    result.i2_o[mode][conductor] = tail.i_o[conductor] * scale;
                }
            } else {
                let lower = self.bracket_lower_index(tb)?;
                self.interpolate_sample(lower, tb, |conductor, value| {
                    result.v2_i[mode][conductor] = value.v_i;
                    result.v2_o[mode][conductor] = value.v_o;
                    result.i2_i[mode][conductor] = value.i_i;
                    result.i2_o[mode][conductor] = value.i_o;
                })?;
            }
        }

        if let Some(prune_to) = prune_to {
            for _ in 0..prune_to {
                self.samples.pop_front();
            }
        }

        Ok(result)
    }

    fn bracket_lower_index(&self, target_ps: f64) -> Result<usize, NativeCplError> {
        for index in 0..self.samples.len().saturating_sub(1) {
            let lower = &self.samples[index];
            let upper = &self.samples[index + 1];
            if (lower.time_ps as f64) <= target_ps && target_ps <= (upper.time_ps as f64) {
                return Ok(index);
            }
        }
        Err(NativeCplError::InsufficientHistory { target_ps })
    }

    fn interpolate_sample(
        &self,
        lower_index: usize,
        target_ps: f64,
        mut set_value: impl FnMut(usize, InterpolatedViValue),
    ) -> Result<(), NativeCplError> {
        let lower = self
            .samples
            .get(lower_index)
            .ok_or(NativeCplError::InsufficientHistory { target_ps })?;
        let upper = self
            .samples
            .get(lower_index + 1)
            .ok_or(NativeCplError::InsufficientHistory { target_ps })?;
        let span = (upper.time_ps - lower.time_ps) as f64;
        if span <= 0.0 {
            return Err(NativeCplError::NonMonotonicHistory {
                previous_ps: lower.time_ps,
                next_ps: upper.time_ps,
            });
        }
        let f = (target_ps - lower.time_ps as f64) / span;
        for conductor in 0..self.no_l {
            set_value(
                conductor,
                InterpolatedViValue {
                    v_i: lerp(lower.v_i[conductor], upper.v_i[conductor], f),
                    v_o: lerp(lower.v_o[conductor], upper.v_o[conductor], f),
                    i_i: lerp(lower.i_i[conductor], upper.i_i[conductor], f),
                    i_o: lerp(lower.i_o[conductor], upper.i_o[conductor], f),
                },
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct InterpolatedViValue {
    v_i: f64,
    v_o: f64,
    i_i: f64,
    i_o: f64,
}

#[derive(Debug, Clone)]
struct MultOut {
    poly: Vec<Vec<Dd>>,
    c0: Vec<Dd>,
}

impl MultOut {
    fn new(dim: usize, poly_len: usize) -> Self {
        Self {
            poly: vec![vec![Dd::ZERO; poly_len]; dim],
            c0: vec![Dd::ZERO; dim],
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SingleOut {
    poly: Vec<Dd>,
    c0: Dd,
}

#[derive(Debug, Clone, Copy)]
struct RowEntry {
    row: usize,
    col: usize,
    value: f64,
}

#[derive(Debug, Clone)]
struct NativeCplSetup {
    dim: usize,
    length: f64,
    r: DdMatrix,
    g: DdMatrix,
    l: DdMatrix,
    c: DdMatrix,
    zy: DdMatrix,
    sv: DdMatrix,
    d: Vec<Dd>,
    y5: DdMatrix,
    y5_inv: DdMatrix,
    sv_inv: DdMatrix,
    frequency: Vec<Dd>,
    si: DdMatrix,
    si_inv: DdMatrix,
    si_sv_inv_p: DdPolyMatrix,
    sip: DdPolyMatrix,
    si_inv_p: DdPolyMatrix,
    sv_inv_p: DdPolyMatrix,
    w: Vec<Vec<Dd>>,
    iwi: Vec<Vec<MultOut>>,
    iwv: Vec<Vec<MultOut>>,
    siv: Vec<Vec<SingleOut>>,
    tau: Vec<Dd>,
    scaling_f: Dd,
    scaling_f2: Dd,
}

impl NativeCplSetup {
    fn new(
        r: &[Vec<f64>],
        l: &[Vec<f64>],
        c: &[Vec<f64>],
        g: &[Vec<f64>],
        length: f64,
    ) -> Result<Self, NativeCplError> {
        let dim = r.len();
        if !(2..=MAX_CP_TX_LINES).contains(&dim) {
            return Err(NativeCplError::InvalidDimension(dim));
        }
        if !length.is_finite() || length <= 0.0 {
            return Err(NativeCplError::InvalidLength(length));
        }
        validate_square_matrix("R", r, dim)?;
        validate_square_matrix("L", l, dim)?;
        validate_square_matrix("C", c, dim)?;
        validate_square_matrix("G", g, dim)?;

        let mut r_full = zero_matrix(dim);
        let mut l_full = zero_matrix(dim);
        let mut c_full = zero_matrix(dim);
        let mut g_full = zero_matrix(dim);
        for row in 0..dim {
            for col in 0..dim {
                if row > col {
                    r_full[row][col] = r_full[col][row];
                    l_full[row][col] = l_full[col][row];
                    c_full[row][col] = c_full[col][row];
                    g_full[row][col] = g_full[col][row];
                } else {
                    r_full[row][col] = r[row][col].max(CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
                    l_full[row][col] = l[row][col];
                    c_full[row][col] = c[row][col];
                    g_full[row][col] = g[row][col];
                }
            }
        }

        Ok(Self {
            dim,
            length,
            r: dd_matrix_from_f64(&r_full),
            g: dd_matrix_from_f64(&g_full),
            l: dd_matrix_from_f64(&l_full),
            c: dd_matrix_from_f64(&c_full),
            zy: dd_zero_matrix(dim),
            sv: dd_zero_matrix(dim),
            d: vec![Dd::ZERO; dim],
            y5: dd_zero_matrix(dim),
            y5_inv: dd_zero_matrix(dim),
            sv_inv: dd_zero_matrix(dim),
            frequency: vec![Dd::ZERO; LEFT_DEG + 1],
            si: dd_zero_matrix(dim),
            si_inv: dd_zero_matrix(dim),
            si_sv_inv_p: dd_zero_poly_matrix(dim, LEFT_DEG + 1),
            sip: dd_zero_poly_matrix(dim, LEFT_DEG + 1),
            si_inv_p: dd_zero_poly_matrix(dim, LEFT_DEG + 1),
            sv_inv_p: dd_zero_poly_matrix(dim, LEFT_DEG + 1),
            w: vec![vec![Dd::ZERO; MAX_DEG]; dim],
            iwi: vec![vec![MultOut::new(dim, LEFT_DEG + 1); dim]; dim],
            iwv: vec![vec![MultOut::new(dim, LEFT_DEG + 1); dim]; dim],
            siv: vec![vec![SingleOut::default(); dim]; dim],
            tau: vec![Dd::ZERO; dim],
            scaling_f: Dd::from_f64(1.0),
            scaling_f2: Dd::from_f64(1.0),
        })
    }

    fn coupled(&mut self) -> Result<(), NativeCplError> {
        self.scaling_f = Dd::from_f64(1.0);
        self.scaling_f2 = Dd::from_f64(1.0);

        self.loop_zy(Dd::ZERO)?;
        self.eval_frequency()?;
        self.eval_si_si_inv(Dd::ZERO)?;
        self.store_si_sv_inv(0);
        self.store(0);

        for idx in 1..=LEFT_DEG {
            self.loop_zy(self.frequency[idx])?;
            self.eval_si_si_inv(self.frequency[idx])?;
            self.store_si_sv_inv(idx);
            self.store(idx);
        }

        poly_matrix(&mut self.sip, &self.frequency, self.dim, LEFT_DEG)?;
        poly_matrix(&mut self.si_inv_p, &self.frequency, self.dim, LEFT_DEG)?;
        poly_matrix(&mut self.sv_inv_p, &self.frequency, self.dim, LEFT_DEG)?;
        self.poly_w()?;

        self.iwi = matrix_p_mult(
            &self.sip,
            &self.w,
            &self.si_inv_p,
            self.dim,
            LEFT_DEG,
            LEFT_DEG,
        );
        self.iwv = matrix_p_mult(
            &self.sip,
            &self.w,
            &self.sv_inv_p,
            self.dim,
            LEFT_DEG,
            LEFT_DEG,
        );

        poly_matrix(&mut self.si_sv_inv_p, &self.frequency, self.dim, LEFT_DEG)?;
        self.generate_out()?;

        Ok(())
    }

    fn into_runtime(self) -> Result<NativeCplRuntime, NativeCplError> {
        let dim = self.dim;
        let mut taul_ps = vec![0.0; dim];
        let mut h1t = vec![vec![None; dim]; dim];
        let mut h2t = vec![vec![vec![None; dim]; dim]; dim];
        let mut h3t = vec![vec![vec![None; dim]; dim]; dim];
        let mut h1c = zero_matrix(dim);
        let mut h2c = vec![zero_matrix(dim); dim];
        let mut h3c = vec![zero_matrix(dim); dim];

        for mode in 0..dim {
            taul_ps[mode] = self.tau[mode].mul_f64(1.0e12).to_f64();
        }

        for row in 0..dim {
            for col in 0..dim {
                if !self.siv[row][col].c0.is_zero() {
                    let series = NativeCplTimeSeries::from_pade_dd(
                        self.siv[row][col].c0,
                        &self.siv[row][col].poly,
                    );
                    h1c[row][col] = series.constant();
                    h1t[row][col] = Some(series);
                }

                for mode in 0..dim {
                    if !self.iwi[row][col].c0[mode].is_zero() {
                        let series = NativeCplTimeSeries::from_pade_dd(
                            self.iwi[row][col].c0[mode],
                            &self.iwi[row][col].poly[mode],
                        );
                        h2c[row][col][mode] = series.constant();
                        h2t[row][col][mode] = Some(series);
                    }

                    if !self.iwv[row][col].c0[mode].is_zero() {
                        let series = NativeCplTimeSeries::from_pade_dd(
                            self.iwv[row][col].c0[mode],
                            &self.iwv[row][col].poly[mode],
                        );
                        h3c[row][col][mode] = series.constant();
                        h3t[row][col][mode] = Some(series);
                    }
                }
            }
        }

        Ok(NativeCplRuntime {
            no_l: dim,
            taul_ps,
            h1t,
            h2t,
            h3t,
            h1c,
            h2c,
            h3c,
            h1e: vec![vec![[0.0; 3]; dim]; dim],
        })
    }

    fn eval_si_si_inv(&mut self, y: Dd) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    let term = y
                        .mul(self.r[mode][col])
                        .add(self.scaling_f.mul(self.l[mode][col]));
                    acc = acc.add(self.sv_inv[row][mode].mul(term));
                }
                self.si_inv[row][col] = acc;
            }
        }

        for row in 0..self.dim {
            let scale = self.d[row].sqrt();
            for col in 0..self.dim {
                self.si_inv[row][col] = self.si_inv[row][col].div(scale);
            }
        }

        self.si = invert_ngspice(&self.si_inv)?;
        Ok(())
    }

    fn loop_zy(&mut self, y: Dd) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                self.zy[row][col] = self
                    .scaling_f
                    .mul(self.c[row][col])
                    .add(self.g[row][col].mul(y));
            }
        }

        self.diag()?;

        let mut fmin = self.d[0];
        for &value in &self.d[1..] {
            if value.lt(fmin) {
                fmin = value;
            }
        }
        if fmin.hi < 0.0 {
            return Err(NativeCplError::NonPositiveCapacitanceMatrix);
        }

        let fmin = fmin.sqrt();
        let fmin_inv = Dd::from_f64(1.0).div(fmin);

        for mode in 0..self.dim {
            self.d[mode] = self.d[mode].sqrt();
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.y5[row][col] = self.d[row].mul(self.sv[col][row]);
                self.y5_inv[row][col] = self.sv[col][row].div(self.d[row]);
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    acc = acc.add(self.sv[row][mode].mul(self.y5[mode][col]));
                }
                self.sv_inv[row][col] = acc;
            }
        }
        self.y5.clone_from(&self.sv_inv);

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    acc = acc.add(self.sv[row][mode].mul(self.y5_inv[mode][col]));
                }
                self.sv_inv[row][col] = acc;
            }
        }
        self.y5_inv.clone_from(&self.sv_inv);

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    let coeff = self
                        .scaling_f
                        .mul(self.l[row][mode])
                        .add(self.r[row][mode].mul(y));
                    acc = acc.add(coeff.mul(self.y5[mode][col]));
                }
                self.zy[row][col] = acc;
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    acc = acc.add(self.y5[row][mode].mul(self.zy[mode][col]));
                }
                self.sv_inv[row][col] = acc;
            }
        }
        self.zy.clone_from(&self.sv_inv);

        self.diag()?;

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    acc = acc.add(self.sv[mode][row].mul(self.y5[mode][col]));
                }
                self.sv_inv[row][col] = acc.mul(fmin_inv);
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut acc = Dd::ZERO;
                for mode in 0..self.dim {
                    acc = acc.add(self.y5_inv[row][mode].mul(self.sv[mode][col]));
                }
                self.zy[row][col] = acc.mul(fmin);
            }
        }
        self.sv.clone_from(&self.zy);

        Ok(())
    }

    fn eval_frequency(&mut self) -> Result<(), NativeCplError> {
        let mut min = self.d[0];
        for &value in &self.d[1..] {
            if value.lt(min) {
                min = value;
            }
        }

        if min.hi <= 0.0 {
            return Err(NativeCplError::NonPositiveModeFrequency);
        }

        self.scaling_f2 = Dd::from_f64(1.0).div(min);
        self.scaling_f = self.scaling_f2.sqrt();

        let spacing = self.length * 8.0;
        self.frequency[0] = Dd::ZERO;
        for idx in 1..=LEFT_DEG {
            self.frequency[idx] = self.frequency[idx - 1].add_f64(spacing);
        }

        for value in &mut self.d {
            *value = value.mul(self.scaling_f2);
        }

        Ok(())
    }

    fn store(&mut self, index: usize) {
        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sip[row][col][index] = self.si[row][col];
                self.si_inv_p[row][col][index] = self.si_inv[row][col];
                self.sv_inv_p[row][col][index] = self.sv_inv[row][col];
            }
            self.w[row][index] = self.d[row];
        }
    }

    fn store_si_sv_inv(&mut self, index: usize) {
        for row in 0..self.dim {
            for col in 0..self.dim {
                let mut temp = Dd::ZERO;
                for mode in 0..self.dim {
                    temp = temp.add(self.si[row][mode].mul(self.sv_inv[mode][col]));
                }
                self.si_sv_inv_p[row][col][index] = temp;
            }
        }
    }

    fn poly_w(&mut self) -> Result<(), NativeCplError> {
        for mode in 0..self.dim {
            self.w[mode] = match_coefficients(LEFT_DEG, &self.frequency, &self.w[mode])?;
            self.tau[mode] = self.approx_mode(mode);
        }
        Ok(())
    }

    fn approx_mode(&mut self, mode: usize) -> Dd {
        let w0 = self.w[mode][0];
        let w1 = self.w[mode][1].div(w0);
        let w2 = self.w[mode][2].div(w0);
        let w3 = self.w[mode][3].div(w0);
        let w4 = self.w[mode][4].div(w0);
        let w5 = self.w[mode][5].div(w0);

        let y1 = w1.mul_f64(0.5);
        let y2 = w2.sub(y1.mul(y1));
        let y3 = w3.mul_f64(3.0).sub(y1.mul(y2).mul_f64(3.0));
        let y4 = w4
            .mul_f64(12.0)
            .sub(y2.mul(y2).mul_f64(3.0))
            .sub(y1.mul(y3).mul_f64(4.0));
        let y5 = w5
            .mul_f64(60.0)
            .sub(y1.mul(y4).mul_f64(5.0))
            .sub(y2.mul(y3).mul_f64(10.0));
        let y6 = y3
            .mul(y3)
            .mul_f64(-10.0)
            .sub(y2.mul(y4).mul_f64(15.0))
            .sub(y1.mul(y5).mul_f64(6.0));

        let delay = w0.sqrt().mul_f64(self.length).div(self.scaling_f);
        let atten = delay.mul(y1).neg().exp();

        let mut a = [Dd::ZERO; 6];
        a[1] = y2.div_f64(2.0);
        a[2] = y3.div_f64(6.0);
        a[3] = y4.div_f64(24.0);
        a[4] = y5.div_f64(120.0);
        a[5] = y6.div_f64(720.0);

        for value in a.iter_mut().skip(1) {
            *value = value.mul(delay.neg());
        }

        let mut b = [Dd::ZERO; 6];
        b[0] = Dd::from_f64(1.0);
        b[1] = a[1];
        for idx in 2..=5 {
            let mut acc = Dd::ZERO;
            for j in 1..=idx {
                acc = acc.add(a[j].mul_f64(j as f64).mul(b[idx - j]));
            }
            b[idx] = acc.div_f64(idx as f64);
        }

        for (idx, value) in b.iter().enumerate() {
            self.w[mode][idx] = value.mul(atten);
        }

        delay
    }

    fn generate_out(&mut self) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                let constant = self.si_sv_inv_p[row][col][0];
                self.siv[row][col].c0 = constant;
                if constant.is_zero() {
                    continue;
                }

                for idx in 0..=LEFT_DEG {
                    self.si_sv_inv_p[row][col][idx] = self.si_sv_inv_p[row][col][idx].div(constant);
                }

                let a_b = if row == col {
                    self.g[row][row]
                        .div(self.r[row][row])
                        .sqrt()
                        .div(constant)
                } else {
                    Dd::ZERO
                };
                let (kind, pade) = pade_apx(a_b, &self.si_sv_inv_p[row][col])?;
                self.siv[row][col].poly = pade_to_vec(kind, pade);
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                for mode in 0..self.dim {
                    let constant = self.iwi[row][col].c0[mode];
                    if constant.is_zero() {
                        continue;
                    }

                    let a_b = if row == col && mode == row {
                        self.g[row][row]
                            .mul(self.r[row][row])
                            .sqrt()
                            .mul_f64(-self.length)
                            .exp()
                            .div(constant)
                    } else {
                        Dd::ZERO
                    };
                    let (kind, pade) = pade_apx(a_b, &self.iwi[row][col].poly[mode])?;
                    self.iwi[row][col].poly[mode] = pade_to_vec(kind, pade);
                }
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                for mode in 0..self.dim {
                    let constant = self.iwv[row][col].c0[mode];
                    if constant.is_zero() {
                        continue;
                    }

                    let a_b = if row == col && mode == row {
                        let sqrt_gr = self.g[row][row].mul(self.r[row][row]).sqrt();
                        self.g[row][row]
                            .div(self.r[row][row])
                            .sqrt()
                            .mul(sqrt_gr.mul_f64(-self.length).exp())
                            .div(constant)
                    } else {
                        Dd::ZERO
                    };
                    let (kind, pade) = pade_apx(a_b, &self.iwv[row][col].poly[mode])?;
                    self.iwv[row][col].poly[mode] = pade_to_vec(kind, pade);
                }
            }
        }

        Ok(())
    }

    fn diag(&mut self) -> Result<(), NativeCplError> {
        let mut row_entries = Vec::new();
        let mut fmin = self.zy[0][0].abs();
        let mut fmax = fmin;

        for row in 0..self.dim {
            for col in row..self.dim {
                let value = self.zy[row][col].abs();
                if value.gt(fmax) {
                    fmax = value;
                } else if value.lt(fmin) {
                    fmin = value;
                }
            }
        }

        let scale = Dd::from_f64(2.0).div(fmin.add(fmax));
        for row in 0..self.dim {
            for col in row..self.dim {
                self.zy[row][col] = self.zy[row][col].mul(scale);
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv[row][col] = if row == col {
                    Dd::from_f64(1.0)
                } else {
                    Dd::ZERO
                };
            }
        }

        // Maintain an f64 shadow of the (already-scaled) symmetric matrix. All
        // Jacobi *decisions* — pivot selection, the row ordering, and the
        // rotation sign that fixes the eigenvalue-to-index assignment — are made
        // from this shadow with the exact f64 arithmetic ngspice uses, so the
        // mode ordering is bit-identical to ngspice. The accurate dd matrix
        // (`self.zy`/`self.sv`) is rotated in lockstep through the same pivot
        // sequence, so the eigenvalue *values* and eigenvectors stay dd-accurate
        // while their labeling matches ngspice. Without this, dd's true (more
        // symmetric) off-diagonals break near-degenerate pivot ties differently
        // than ngspice, permuting the modes.
        let mut zyf = vec![vec![0.0f64; self.dim]; self.dim];
        for row in 0..self.dim {
            for col in 0..self.dim {
                zyf[row][col] = self.zy[row][col].to_f64();
            }
        }

        for row in 0..self.dim.saturating_sub(1) {
            let (mode, max_value) = best_offdiag(&zyf, row, self.dim);
            insert_row(
                &mut row_entries,
                RowEntry {
                    row,
                    col: mode,
                    value: max_value,
                },
            );
        }

        let mut rotations = 0usize;
        while row_entries.first().is_some_and(|entry| entry.value > EPSI2) {
            let entry = row_entries[0];
            self.rotate(entry.row, entry.col, &mut zyf);
            self.reordering(entry.row, &mut row_entries, &zyf);
            if entry.col + 1 != self.dim {
                self.reordering(entry.col, &mut row_entries, &zyf);
            }

            rotations += 1;
            if rotations > 1_000_000 {
                return Err(NativeCplError::DiagonalizationDidNotConverge);
            }
        }

        for mode in 0..self.dim {
            self.d[mode] = self.zy[mode][mode].div(scale);
        }

        Ok(())
    }

    fn reordering(&self, row: usize, rows: &mut Vec<RowEntry>, zyf: &[Vec<f64>]) {
        let (mode, max_value) = best_offdiag(zyf, row, self.dim);
        delete_row(rows, row);
        insert_row(
            rows,
            RowEntry {
                row,
                col: mode,
                value: max_value,
            },
        );
    }

    fn rotate(&mut self, p: usize, q: usize, zyf: &mut [Vec<f64>]) {
        // The eigenvalue-to-index assignment is fixed by the Jacobi sign
        // convention `sgn(mu)`, which ngspice evaluates in f64. Take that sign
        // from the f64 shadow so the labeling is bit-identical to ngspice, and
        // use it for BOTH the dd rotation (accurate values) and the f64 shadow
        // rotation (decisions). The co/si magnitudes are computed independently
        // in each precision.
        let mu_sign = sgn(0.5 * (zyf[p][p] - zyf[q][q]));

        // --- dd rotation of self.zy / self.sv (accurate values) ---
        let ld = self.zy[p][q].neg();
        let mu = self.zy[p][p].sub(self.zy[q][q]).mul_f64(0.5);
        let ve = ld.mul(ld).add(mu.mul(mu)).sqrt();
        let co = ve.add(mu.abs()).div(ve.mul_f64(2.0)).sqrt();
        let si = Dd::from_f64(mu_sign)
            .mul(ld)
            .div(ve.mul_f64(2.0).mul(co));

        let mut t = vec![Dd::ZERO; self.dim];
        for col in p + 1..self.dim {
            t[col] = self.zy[p][col];
        }
        for col in 0..p {
            t[col] = self.zy[col][p];
        }

        for col in p + 1..self.dim {
            if col == q {
                continue;
            }
            if col > q {
                self.zy[p][col] = t[col].mul(co).sub(self.zy[q][col].mul(si));
            } else {
                self.zy[p][col] = t[col].mul(co).sub(self.zy[col][q].mul(si));
            }
        }

        for col in q + 1..self.dim {
            if col == p {
                continue;
            }
            self.zy[q][col] = t[col].mul(si).add(self.zy[q][col].mul(co));
        }

        for col in 0..p {
            if col == q {
                continue;
            }
            self.zy[col][p] = t[col].mul(co).sub(self.zy[col][q].mul(si));
        }

        for col in 0..q {
            if col == p {
                continue;
            }
            self.zy[col][q] = t[col].mul(si).add(self.zy[col][q].mul(co));
        }

        let z_pp = self.zy[p][p];
        let z_qq = self.zy[q][q];
        let z_pq = self.zy[p][q];
        self.zy[p][p] = z_pp
            .mul(co)
            .mul(co)
            .add(z_qq.mul(si).mul(si))
            .sub(z_pq.mul(si).mul(co).mul_f64(2.0));
        self.zy[q][q] = z_pp
            .mul(si)
            .mul(si)
            .add(z_qq.mul(co).mul(co))
            .add(z_pq.mul(si).mul(co).mul_f64(2.0));
        self.zy[p][q] = Dd::ZERO;

        let mut sv_p = vec![Dd::ZERO; self.dim];
        let mut sv_q = vec![Dd::ZERO; self.dim];
        for row in 0..self.dim {
            sv_p[row] = self.sv[row][p];
            sv_q[row] = self.sv[row][q];
        }

        for row in 0..self.dim {
            self.sv[row][p] = sv_p[row].mul(co).sub(sv_q[row].mul(si));
            self.sv[row][q] = sv_p[row].mul(si).add(sv_q[row].mul(co));
        }

        // --- f64 shadow rotation (decisions only), mirroring ngspice exactly ---
        rotate_f64(zyf, p, q, mu_sign, self.dim);
    }
}

/// Largest off-diagonal in row `row` (cols `row+1..dim`) of the f64 shadow,
/// using ngspice's truncated-integer magnitude comparison. Returns (col, |v|).
fn best_offdiag(zyf: &[Vec<f64>], row: usize, dim: usize) -> (usize, f64) {
    let mut mode = row + 1;
    let mut max_value = zyf[row][mode].abs();
    for col in mode + 1..dim {
        let v = zyf[row][col].abs();
        if ((v * 1.0e7) as i32) > ((1.0e7 * max_value) as i32) {
            max_value = v;
            mode = col;
        }
    }
    (mode, max_value)
}

/// f64 Jacobi rotation on the shadow matrix, identical to ngspice's `rotate`,
/// driven by the sign `mu_sign` already decided by the caller.
fn rotate_f64(zy: &mut [Vec<f64>], p: usize, q: usize, mu_sign: f64, dim: usize) {
    let ld = -zy[p][q];
    let mu = 0.5 * (zy[p][p] - zy[q][q]);
    let ve = (ld * ld + mu * mu).sqrt();
    let co = ((ve + mu.abs()) / (2.0 * ve)).sqrt();
    let si = mu_sign * ld / (2.0 * ve * co);

    let mut t = vec![0.0f64; dim];
    for col in p + 1..dim {
        t[col] = zy[p][col];
    }
    for col in 0..p {
        t[col] = zy[col][p];
    }

    for col in p + 1..dim {
        if col == q {
            continue;
        }
        if col > q {
            zy[p][col] = t[col] * co - zy[q][col] * si;
        } else {
            zy[p][col] = t[col] * co - zy[col][q] * si;
        }
    }

    for col in q + 1..dim {
        if col == p {
            continue;
        }
        zy[q][col] = t[col] * si + zy[q][col] * co;
    }

    for col in 0..p {
        if col == q {
            continue;
        }
        zy[col][p] = t[col] * co - zy[col][q] * si;
    }

    for col in 0..q {
        if col == p {
            continue;
        }
        zy[col][q] = t[col] * si + zy[col][q] * co;
    }

    let z_pp = zy[p][p];
    let z_qq = zy[q][q];
    let z_pq = zy[p][q];
    zy[p][p] = z_pp * co * co + z_qq * si * si - 2.0 * z_pq * si * co;
    zy[q][q] = z_pp * si * si + z_qq * co * co + 2.0 * z_pq * si * co;
    zy[p][q] = 0.0;
}

fn validate_square_matrix(
    label: &'static str,
    matrix: &[Vec<f64>],
    expected: usize,
) -> Result<(), NativeCplError> {
    if matrix.len() != expected || matrix.iter().any(|row| row.len() != expected) {
        return Err(NativeCplError::InvalidMatrix { label, expected });
    }
    for (row, values) in matrix.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            if !value.is_finite() {
                return Err(NativeCplError::NonFiniteMatrix { label, row, col });
            }
        }
    }
    Ok(())
}

fn zero_matrix(dim: usize) -> Matrix {
    vec![vec![0.0; dim]; dim]
}

fn validate_history_vector(
    label: &'static str,
    values: &[f64],
    expected: usize,
) -> Result<(), NativeCplError> {
    if values.len() != expected {
        return Err(NativeCplError::InvalidHistoryDimension {
            label,
            expected,
            actual: values.len(),
        });
    }
    for (index, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(NativeCplError::NonFiniteHistoryValue { label, index });
        }
    }
    Ok(())
}

fn validate_positive_step(step: f64) -> Result<(), NativeCplError> {
    if step.is_finite() && step > 0.0 {
        Ok(())
    } else {
        Err(NativeCplError::InvalidTransientStep(step))
    }
}

fn validate_delayed_vi(
    delayed: &NativeCplDelayedVi,
    expected: usize,
) -> Result<(), NativeCplError> {
    validate_history_vector("delay ratio", &delayed.ratio, expected)?;
    validate_square_matrix("delayed v1_i", &delayed.v1_i, expected)?;
    validate_square_matrix("delayed v2_i", &delayed.v2_i, expected)?;
    validate_square_matrix("delayed i1_i", &delayed.i1_i, expected)?;
    validate_square_matrix("delayed i2_i", &delayed.i2_i, expected)?;
    validate_square_matrix("delayed v1_o", &delayed.v1_o, expected)?;
    validate_square_matrix("delayed v2_o", &delayed.v2_o, expected)?;
    validate_square_matrix("delayed i1_o", &delayed.i1_o, expected)?;
    validate_square_matrix("delayed i2_o", &delayed.i2_o, expected)?;
    Ok(())
}

fn lerp(start: f64, end: f64, fraction: f64) -> f64 {
    start + fraction * (end - start)
}

fn initialize_dc_time_series(tms: &mut NativeCplTimeSeries, input_dc: f64, output_dc: f64) {
    if tms.if_img {
        tms.tm[0].cnv_i = -input_dc * tms.tm[0].c / tms.tm[0].x;
        tms.tm[0].cnv_o = -output_dc * tms.tm[0].c / tms.tm[0].x;
        let (a, b) = div_complex(tms.tm[1].c, tms.tm[2].c, tms.tm[1].x, tms.tm[2].x);
        tms.tm[1].cnv_i = -input_dc * a;
        tms.tm[1].cnv_o = -output_dc * a;
        tms.tm[2].cnv_i = -input_dc * b;
        tms.tm[2].cnv_o = -output_dc * b;
    } else {
        for term in &mut tms.tm {
            term.cnv_i = -input_dc * term.c / term.x;
            term.cnv_o = -output_dc * term.c / term.x;
        }
    }
}

fn zero_time_series_convolutions(tms: &mut NativeCplTimeSeries) {
    for term in &mut tms.tm {
        term.cnv_i = 0.0;
        term.cnv_o = 0.0;
    }
}

fn update_accepted_real_term(
    term: &mut NativeCplTerm,
    h_seconds: f64,
    previous_input: f64,
    current_input: f64,
    previous_output: f64,
    current_output: f64,
    e: f64,
) {
    let scale = term.c / term.x;
    let input_slope = (current_input - previous_input) * scale / h_seconds;
    let output_slope = (current_output - previous_output) * scale / h_seconds;
    term.cnv_i = (term.cnv_i - input_slope * h_seconds) * e
        + (e - 1.0) * (current_input * scale + input_slope / term.x);
    term.cnv_o = (term.cnv_o - output_slope * h_seconds) * e
        + (e - 1.0) * (current_output * scale + output_slope / term.x);
}

#[allow(clippy::too_many_arguments)]
fn update_accepted_complex_time_series(
    tms: &mut NativeCplTimeSeries,
    h_seconds: f64,
    previous_input: f64,
    current_input: f64,
    previous_output: f64,
    current_output: f64,
    er: f64,
    ei: f64,
) {
    let h_half = 0.5 * h_seconds;
    let (a1, b1) = mult_complex(tms.tm[1].c, tms.tm[2].c, er, ei);

    let (a, b) = mult_complex(tms.tm[1].cnv_i, tms.tm[2].cnv_i, er, ei);
    tms.tm[1].cnv_i = a + h_half * (a1 * previous_input + current_input * tms.tm[1].c);
    tms.tm[2].cnv_i = b + h_half * (b1 * previous_input + current_input * tms.tm[2].c);

    let (a, b) = mult_complex(tms.tm[1].cnv_o, tms.tm[2].cnv_o, er, ei);
    tms.tm[1].cnv_o = a + h_half * (a1 * previous_output + current_output * tms.tm[1].c);
    tms.tm[2].cnv_o = b + h_half * (b1 * previous_output + current_output * tms.tm[2].c);
}

fn add_delayed_convolution(tms: &mut NativeCplTimeSeries, scale: f64, input: f64, output: f64) {
    for term in &mut tms.tm {
        term.cnv_i += scale * input * term.c;
        term.cnv_o += scale * output * term.c;
    }
}

fn update_rhs_time_series(
    tms: &mut NativeCplTimeSeries,
    h_seconds: f64,
    h1_seconds: f64,
    input_previous: f64,
    input_current: f64,
    output_previous: f64,
    output_current: f64,
) {
    if tms.if_img {
        let (er, ei) = exp_complex(tms.tm[1].x, tms.tm[2].x, h_seconds);
        let a2 = h1_seconds * tms.tm[1].c;
        let b2 = h1_seconds * tms.tm[2].c;

        let (a, b) = mult_complex(tms.tm[1].cnv_i, tms.tm[2].cnv_i, er, ei);
        let (a1, b1) = mult_complex(
            a2,
            b2,
            input_previous * er + input_current,
            input_previous * ei,
        );
        tms.tm[1].cnv_i = a + a1;
        tms.tm[2].cnv_i = b + b1;

        let (a, b) = mult_complex(tms.tm[1].cnv_o, tms.tm[2].cnv_o, er, ei);
        let (a1, b1) = mult_complex(
            a2,
            b2,
            output_previous * er + output_current,
            output_previous * ei,
        );
        tms.tm[1].cnv_o = a + a1;
        tms.tm[2].cnv_o = b + b1;

        let e = (tms.tm[0].x * h_seconds).exp();
        tms.tm[0].cnv_i =
            tms.tm[0].cnv_i * e + h1_seconds * tms.tm[0].c * (input_previous * e + input_current);
        tms.tm[0].cnv_o =
            tms.tm[0].cnv_o * e + h1_seconds * tms.tm[0].c * (output_previous * e + output_current);
    } else {
        for term in &mut tms.tm {
            let e = (term.x * h_seconds).exp();
            term.cnv_i =
                term.cnv_i * e + h1_seconds * term.c * (input_previous * e + input_current);
            term.cnv_o =
                term.cnv_o * e + h1_seconds * term.c * (output_previous * e + output_current);
        }
    }
}

fn exp_complex(real: f64, imag: f64, h: f64) -> (f64, f64) {
    let e = (real * h).exp();
    (e * (imag * h).cos(), e * (imag * h).sin())
}

fn mult_complex(ar: f64, ai: f64, br: f64, bi: f64) -> (f64, f64) {
    (ar * br - ai * bi, ar * bi + ai * br)
}

fn div_complex(ar: f64, ai: f64, br: f64, bi: f64) -> (f64, f64) {
    let t = br * br + bi * bi;
    ((ar * br + ai * bi) / t, (ai * br - ar * bi) / t)
}

fn sgn(value: f64) -> f64 {
    if value >= 0.0 { 1.0 } else { -1.0 }
}

fn insert_row(rows: &mut Vec<RowEntry>, entry: RowEntry) {
    let index = rows
        .iter()
        .position(|existing| existing.value < entry.value)
        .unwrap_or(rows.len());
    rows.insert(index, entry);
}

fn delete_row(rows: &mut Vec<RowEntry>, row: usize) -> RowEntry {
    let index = rows
        .iter()
        .position(|entry| entry.row == row)
        .expect("ngspice CPL row ordering entry exists");
    rows.remove(index)
}

fn poly_matrix(
    matrix: &mut DdPolyMatrix,
    frequency: &[Dd],
    dim: usize,
    deg: usize,
) -> Result<(), NativeCplError> {
    for row in 0..dim {
        for col in 0..dim {
            matrix[row][col] = match_coefficients(deg, frequency, &matrix[row][col])?;
        }
    }
    Ok(())
}

fn match_coefficients(n: usize, xa: &[Dd], ya: &[Dd]) -> Result<Vec<Dd>, NativeCplError> {
    let mut x = xa[..=n].to_vec();
    let mut y = ya[..=n].to_vec();
    let mut cof = vec![Dd::ZERO; n + 1];

    for degree in 0..=n {
        cof[degree] = polint(&x[..=n - degree], &y[..=n - degree], Dd::ZERO)?;

        let mut xmin = Dd::from_f64(1.0e38);
        let mut pivot = 0usize;
        for idx in 0..=n - degree {
            if x[idx].abs().lt(xmin) {
                xmin = x[idx].abs();
                pivot = idx;
            }
            if !x[idx].is_zero() {
                y[idx] = y[idx].sub(cof[degree]).div(x[idx]);
            }
        }

        for idx in pivot + 1..=n - degree {
            y[idx - 1] = y[idx];
            x[idx - 1] = x[idx];
        }
    }

    Ok(cof)
}

fn polint(xa_zero: &[Dd], ya_zero: &[Dd], x: Dd) -> Result<Dd, NativeCplError> {
    let n = xa_zero.len();
    let mut xa = vec![Dd::ZERO; n + 1];
    let mut ya = vec![Dd::ZERO; n + 1];
    for idx in 0..n {
        xa[idx + 1] = xa_zero[idx];
        ya[idx + 1] = ya_zero[idx];
    }

    let mut ns = 1usize;
    let mut dif = x.sub(xa[1]).abs();
    let mut c = vec![Dd::ZERO; n + 1];
    let mut d = vec![Dd::ZERO; n + 1];

    for idx in 1..=n {
        let dift = x.sub(xa[idx]).abs();
        if dift.lt(dif) {
            ns = idx;
            dif = dift;
        }
        c[idx] = ya[idx];
        d[idx] = ya[idx];
    }

    let mut y = ya[ns];
    ns -= 1;

    for m in 1..n {
        for idx in 1..=n - m {
            let ho = xa[idx].sub(x);
            let hp = xa[idx + m].sub(x);
            let w = c[idx + 1].sub(d[idx]);
            let den = ho.sub(hp);
            if den.is_zero() {
                return Err(NativeCplError::InterpolationFailure);
            }
            let den = w.div(den);
            d[idx] = hp.mul(den);
            c[idx] = ho.mul(den);
        }

        let dy = if 2 * ns < n - m {
            c[ns + 1]
        } else {
            let value = d[ns];
            ns -= 1;
            value
        };
        y = y.add(dy);
    }

    Ok(y)
}

fn matrix_p_mult(
    a: &DdPolyMatrix,
    d: &[Vec<Dd>],
    b: &DdPolyMatrix,
    dim: usize,
    deg: usize,
    deg_o: usize,
) -> Vec<Vec<MultOut>> {
    let mut x = vec![vec![MultOut::new(dim, deg_o + 1); dim]; dim];
    let mut t = dd_zero_poly_matrix(dim, deg_o + 1);

    for row in 0..dim {
        for col in 0..dim {
            t[row][col] = mult_p(&b[row][col], &d[row], deg, deg_o, deg_o);
        }
    }

    for row in 0..dim {
        for col in 0..dim {
            for mode in 0..dim {
                let mut poly = mult_p(&a[row][mode], &t[mode][col], deg, deg_o, deg_o);
                let constant = poly[0];
                x[row][col].c0[mode] = constant;
                if !constant.is_zero() {
                    poly[0] = Dd::from_f64(1.0);
                    for coeff in poly.iter_mut().take(deg_o + 1).skip(1) {
                        *coeff = coeff.div(constant);
                    }
                }
                x[row][col].poly[mode] = poly;
            }
        }
    }

    x
}

fn mult_p(p1: &[Dd], p2: &[Dd], d1: usize, d2: usize, d3: usize) -> Vec<Dd> {
    let mut p3 = vec![Dd::ZERO; d3 + 1];
    for i in 0..=d1 {
        let mut j = i;
        for k in 0..=d2 {
            if j > d3 {
                break;
            }
            p3[j] = p3[j].add(p1[i].mul(p2[k]));
            j += 1;
        }
    }
    p3
}

fn invert_ngspice(left: &DdMatrix) -> Result<DdMatrix, NativeCplError> {
    let dims = left.len();
    let dim = 2 * dims;
    let mut a = vec![vec![Dd::ZERO; dim + 1]; dim];

    for row in 0..dims {
        for col in 0..dims {
            a[row][col] = left[row][col];
        }
        for col in dims..2 * dims {
            a[row][col] = Dd::ZERO;
        }
        a[row][row + dims] = Dd::from_f64(1.0);
    }

    gaussian_elimination2(&mut a, dims, -1)?;

    let mut inverse = dd_zero_matrix(dims);
    for row in 0..dims {
        for col in 0..dims {
            inverse[row][col] = a[row][col + dims];
        }
    }
    Ok(inverse)
}

fn gaussian_elimination2(a: &mut [Vec<Dd>], dims: usize, kind: i32) -> Result<(), NativeCplError> {
    let dim = if kind == -1 { 2 * dims } else { dims };

    for i in 0..dims {
        let mut imax = i;
        let mut max = a[i][i].abs();
        for row in i + 1..dim {
            if a[row][i].abs().gt(max) {
                imax = row;
                max = a[row][i].abs();
            }
        }
        if max.hi < EPSILON {
            return Err(NativeCplError::SingularMatrix("setup inverse"));
        }

        if imax != i {
            for col in i..=dim {
                let value = a[i][col];
                a[i][col] = a[imax][col];
                a[imax][col] = value;
            }
        }

        let factor = Dd::from_f64(1.0).div(a[i][i]);
        a[i][i] = Dd::from_f64(1.0);
        for col in i + 1..=dim {
            a[i][col] = a[i][col].mul(factor);
        }

        for row in 0..dims {
            if i == row {
                continue;
            }
            let factor = a[row][i];
            a[row][i] = Dd::ZERO;
            for col in i + 1..=dim {
                a[row][col] = a[row][col].sub(factor.mul(a[i][col]));
            }
        }
    }

    Ok(())
}

fn pade_to_vec(kind: usize, pade: [Dd; 6]) -> Vec<Dd> {
    vec![
        pade[0],
        pade[1],
        pade[2],
        pade[3],
        pade[4],
        pade[5],
        Dd::from_f64(kind as f64),
    ]
}

fn pade_apx(a_b: Dd, b: &[Dd]) -> Result<(usize, [Dd; 6]), NativeCplError> {
    let mut at = [[Dd::ZERO; 4]; 4];
    at[0][0] = Dd::from_f64(1.0).sub(a_b);
    at[0][1] = b[1];
    at[0][2] = b[2];
    at[0][3] = b[3].neg();

    at[1][0] = b[1];
    at[1][1] = b[2];
    at[1][2] = b[3];
    at[1][3] = b[4].neg();

    at[2][0] = b[2];
    at[2][1] = b[3];
    at[2][2] = b[4];
    at[2][3] = b[5].neg();

    gaussian_elimination_at(&mut at, 3)?;

    let p3 = at[0][3];
    let p2 = at[1][3];
    let p1 = at[2][3];
    let q1 = p1.add(b[1]);
    let q2 = b[1].mul(p1).add(p2).add(b[2]);
    let q3 = p3.mul(a_b);

    let roots = find_roots(p1, p2, p3);
    let mut pade = [Dd::ZERO; 6];
    pade[3] = roots.x1;
    pade[4] = roots.x2;
    pade[5] = roots.x3;
    let n1 = q1.sub(p1);
    let n2 = q2.sub(p2);
    let n3 = q3.sub(p3);
    let two_p1 = p1.mul_f64(2.0);
    let three = Dd::from_f64(3.0);
    pade[0] = eval2(n1, n2, n3, roots.x1).div(eval2(three, two_p1, p2, roots.x1));

    if roots.complex_pair {
        let (cr, ci) = get_c(n1, n2, n3, p1, p2, roots.x2, roots.x3);
        pade[1] = cr;
        pade[2] = ci;
        Ok((2, pade))
    } else {
        pade[1] = eval2(n1, n2, n3, roots.x2).div(eval2(three, two_p1, p2, roots.x2));
        pade[2] = eval2(n1, n2, n3, roots.x3).div(eval2(three, two_p1, p2, roots.x3));
        Ok((1, pade))
    }
}

fn gaussian_elimination_at(at: &mut [[Dd; 4]; 4], dims: usize) -> Result<(), NativeCplError> {
    let dim = dims;

    for i in 0..dim {
        let mut imax = i;
        let mut max = at[i][i].abs();
        for row in i + 1..dim {
            if at[row][i].abs().gt(max) {
                imax = row;
                max = at[row][i].abs();
            }
        }
        if max.hi < EPSI_MULT {
            return Err(NativeCplError::SingularMatrix("Pade approximation"));
        }

        if imax != i {
            for col in i..=dim {
                let value = at[i][col];
                at[i][col] = at[imax][col];
                at[imax][col] = value;
            }
        }

        let factor = Dd::from_f64(1.0).div(at[i][i]);
        at[i][i] = Dd::from_f64(1.0);
        for col in i + 1..=dim {
            at[i][col] = at[i][col].mul(factor);
        }

        for row in 0..dim {
            if i == row {
                continue;
            }
            let factor = at[row][i];
            at[row][i] = Dd::ZERO;
            for col in i + 1..=dim {
                at[row][col] = at[row][col].sub(factor.mul(at[i][col]));
            }
        }
    }

    Ok(())
}

fn eval2(a: Dd, b: Dd, c: Dd, x: Dd) -> Dd {
    a.mul(x).mul(x).add(b.mul(x)).add(c)
}

fn get_c(q1: Dd, q2: Dd, q3: Dd, p1: Dd, p2: Dd, a: Dd, b: Dd) -> (Dd, Dd) {
    let a2_b2 = a.mul(a).sub(b.mul(b)); // a^2 - b^2
    let ab = a.mul(b);
    // re = 3(a^2-b^2) + 2 p1 a + p2
    let re = a2_b2.mul_f64(3.0).add(p1.mul(a).mul_f64(2.0)).add(p2);
    // im = 6 a b + 2 p1 b
    let im = ab.mul_f64(6.0).add(p1.mul(b).mul_f64(2.0));
    let d = re.mul(re).add(im.mul(im));

    // qa = q1(a^2-b^2) + q2 a + q3 ; qb = 2 q1 a b + q2 b
    let qa = q1.mul(a2_b2).add(q2.mul(a)).add(q3);
    let qb = q1.mul(ab).mul_f64(2.0).add(q2.mul(b));

    let ci = qa.neg().mul(im).add(qb.mul(re)).div(d);
    let cr = re.mul(qa).add(im.mul(qb)).div(d);

    (cr, ci)
}

#[derive(Debug, Clone, Copy)]
struct Roots {
    x1: Dd,
    x2: Dd,
    x3: Dd,
    complex_pair: bool,
}

fn find_roots(mut a1: Dd, mut a2: Dd, a3: Dd) -> Roots {
    // q = (a1^2 - 3 a2)/9 ; p = (2 a1^3 - 9 a1 a2 + 27 a3)/54
    let q = a1.mul(a1).sub(a2.mul_f64(3.0)).div_f64(9.0);
    let p = a1
        .mul(a1)
        .mul(a1)
        .mul_f64(2.0)
        .sub(a1.mul(a2).mul_f64(9.0))
        .add(a3.mul_f64(27.0))
        .div_f64(54.0);
    let mut t = q.mul(q).mul(q).sub(p.mul(p));
    let mut x;

    // The closed-form seed uses f64 transcendentals; the dd Newton polish below
    // refines it to the accurate root of the (dd-accurate) cubic.
    if t.hi >= 0.0 {
        let qs = q.sqrt();
        let seed = (p.div(q.mul(qs)).to_f64()).acos();
        x = qs
            .mul_f64(-2.0)
            .mul_f64((seed / 3.0).cos())
            .sub(a1.div_f64(3.0));
    } else if p.hi > 0.0 {
        let cube = (t.neg().sqrt().add(p).to_f64()).powf(1.0 / 3.0);
        let tc = Dd::from_f64(cube);
        x = tc.add(q.div(tc)).neg().sub(a1.div_f64(3.0));
        t = tc;
    } else if p.hi == 0.0 {
        x = a1.neg().div_f64(3.0);
    } else {
        let cube = (t.neg().sqrt().sub(p).to_f64()).powf(1.0 / 3.0);
        let tc = Dd::from_f64(cube);
        x = tc.add(q.div(tc)).sub(a1.div_f64(3.0));
        t = tc;
    }
    let _ = t;

    let original = x;
    let mut iterations = 0usize;
    loop {
        let next = root3(a1, a2, a3, x);
        if next.sub(x).abs().to_f64() <= 5.0e-4 {
            break;
        }
        iterations += 1;
        if iterations == 32 {
            x = original;
            break;
        }
        x = next;
    }

    let x1 = x;
    (a1, a2) = div3(a1, a3, x);

    let disc = a1.mul(a1).sub(a2.mul_f64(4.0));
    if disc.hi < 0.0 {
        Roots {
            x1,
            x2: a1.mul_f64(-0.5),
            x3: disc.neg().sqrt().mul_f64(0.5),
            complex_pair: true,
        }
    } else {
        let s = disc.sqrt();
        let x2 = if a1.hi >= 0.0 {
            a1.add(s).mul_f64(-0.5)
        } else {
            a1.sub(s).mul_f64(-0.5)
        };
        Roots {
            x1,
            x2,
            x3: a2.div(x2),
            complex_pair: false,
        }
    }
}

fn root3(a1: Dd, a2: Dd, a3: Dd, x: Dd) -> Dd {
    let t1 = x.mul(x.mul(x.add(a1)).add(a2)).add(a3);
    let t2 = x.mul(a1.mul_f64(2.0).add(x.mul_f64(3.0))).add(a2);
    x.sub(t1.div(t2))
}

fn div3(a1: Dd, a3: Dd, x: Dd) -> (Dd, Dd) {
    (a1.add(x), a3.neg().div(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix_from_upper(dim: usize, upper: &[f64]) -> Matrix {
        assert_eq!(upper.len(), dim * (dim + 1) / 2);
        let mut matrix = zero_matrix(dim);
        let mut index = 0usize;
        for row in 0..dim {
            for col in row..dim {
                matrix[row][col] = upper[index];
                matrix[col][row] = upper[index];
                index += 1;
            }
        }
        matrix
    }

    fn cpl_ibm2_runtime() -> NativeCplRuntime {
        NativeCplRuntime::setup(
            &matrix_from_upper(2, &[0.5, 0.0, 0.5]),
            &matrix_from_upper(2, &[247.3e-9, 31.65e-9, 247.3e-9]),
            &matrix_from_upper(2, &[31.4e-12, -2.45e-12, 31.4e-12]),
            &matrix_from_upper(2, &[0.0, 0.0, 0.0]),
            0.3048,
        )
        .expect("native CPL setup succeeds")
    }

    fn cpl3_4_line_runtime() -> NativeCplRuntime {
        NativeCplRuntime::setup(
            &matrix_from_upper(4, &[0.3, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0, 0.3, 0.0, 0.3]),
            &matrix_from_upper(
                4,
                &[
                    9e-9, 5.4e-9, 0.0, 0.0, 9e-9, 5.4e-9, 0.0, 9e-9, 5.4e-9, 9e-9,
                ],
            ),
            &matrix_from_upper(
                4,
                &[
                    3.5e-13, -3e-14, 0.0, 0.0, 3.5e-13, -3e-14, 0.0, 3.5e-13, -3e-14, 3.5e-13,
                ],
            ),
            &matrix_from_upper(4, &[0.0; 10]),
            6.3,
        )
        .expect("native CPL setup succeeds")
    }

    fn assert_close(actual: f64, expected: f64) {
        let abs_tol = 1.0e-12;
        let rel_tol = 1.0e-9;
        let limit = abs_tol + rel_tol * expected.abs();
        assert!(
            (actual - expected).abs() <= limit,
            "actual={actual:.17e}, expected={expected:.17e}, limit={limit:.3e}"
        );
    }

    fn assert_slice_close(actual: &[f64], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for (&actual, &expected) in actual.iter().zip(expected) {
            assert_close(actual, expected);
        }
    }

    fn term(c: f64, x: f64, cnv_i: f64, cnv_o: f64) -> NativeCplTerm {
        NativeCplTerm { c, x, cnv_i, cnv_o }
    }

    fn tms(if_img: bool, aten: f64, tm: [NativeCplTerm; 3]) -> NativeCplTimeSeries {
        NativeCplTimeSeries { if_img, aten, tm }
    }

    fn empty_runtime(no_l: usize) -> NativeCplRuntime {
        NativeCplRuntime {
            no_l,
            taul_ps: vec![0.0; no_l],
            h1t: vec![vec![None; no_l]; no_l],
            h2t: vec![vec![vec![None; no_l]; no_l]; no_l],
            h3t: vec![vec![vec![None; no_l]; no_l]; no_l],
            h1c: zero_matrix(no_l),
            h2c: vec![zero_matrix(no_l); no_l],
            h3c: vec![zero_matrix(no_l); no_l],
            h1e: vec![vec![[0.0; 3]; no_l]; no_l],
        }
    }

    fn delayed_fixture(ext: bool, ratio: &[f64]) -> NativeCplDelayedVi {
        let no_l = ratio.len();
        NativeCplDelayedVi {
            ext,
            ratio: ratio.to_vec(),
            v1_i: zero_matrix(no_l),
            v2_i: zero_matrix(no_l),
            i1_i: zero_matrix(no_l),
            i2_i: zero_matrix(no_l),
            v1_o: zero_matrix(no_l),
            v2_o: zero_matrix(no_l),
            i1_o: zero_matrix(no_l),
            i2_o: zero_matrix(no_l),
        }
    }

    fn sample(time_ps: i64, base: f64) -> NativeCplViSample {
        NativeCplViSample::new(
            time_ps,
            vec![base + 1.0, base + 2.0],
            vec![base + 11.0, base + 12.0],
            vec![base + 21.0, base + 22.0],
            vec![base + 31.0, base + 32.0],
        )
    }

    fn history_with_samples() -> NativeCplViHistory {
        let mut history =
            NativeCplViHistory::new(2, vec![0.5, 1.5], vec![10.5, 11.5]).expect("history");
        for (time, base) in [(0, 0.0), (10, 100.0), (20, 200.0), (30, 300.0), (40, 400.0)] {
            history
                .push_sample(sample(time, base))
                .expect("monotonic sample");
        }
        history
    }

    #[test]
    fn cpl_native_delayed_vi_samples_use_dc_before_delay() {
        let mut history =
            NativeCplViHistory::new(2, vec![0.5, 1.5], vec![10.5, 11.5]).expect("history");
        history.push_sample(sample(0, 0.0)).expect("initial sample");

        let delayed = history
            .delayed_vi_samples_ps(10, 20, &[40.0, 60.0])
            .expect("delay samples");

        assert!(!delayed.ext);
        assert_slice_close(&delayed.ratio, &[0.0, 0.0]);
        assert_slice_close(&delayed.v1_i[0], &[0.5, 1.5]);
        assert_slice_close(&delayed.v2_i[0], &[0.5, 1.5]);
        assert_slice_close(&delayed.v1_o[1], &[10.5, 11.5]);
        assert_slice_close(&delayed.v2_o[1], &[10.5, 11.5]);
        assert_slice_close(&delayed.i1_i[0], &[0.0, 0.0]);
        assert_slice_close(&delayed.i2_o[1], &[0.0, 0.0]);
        assert_eq!(history.head_time_ps(), Some(0));
    }

    #[test]
    fn cpl_native_delayed_vi_samples_interpolate_and_prune_like_ngspice() {
        let mut history = history_with_samples();

        let delayed = history
            .delayed_vi_samples_ps(40, 50, &[12.0, 5.0])
            .expect("delay samples");

        assert!(delayed.ext);
        assert_slice_close(&delayed.ratio, &[0.0, 0.5]);

        assert_slice_close(&delayed.v1_i[0], &[281.0, 282.0]);
        assert_slice_close(&delayed.v1_o[0], &[291.0, 292.0]);
        assert_slice_close(&delayed.i1_i[0], &[301.0, 302.0]);
        assert_slice_close(&delayed.i1_o[0], &[311.0, 312.0]);
        assert_slice_close(&delayed.v2_i[0], &[381.0, 382.0]);
        assert_slice_close(&delayed.v2_o[0], &[391.0, 392.0]);
        assert_slice_close(&delayed.i2_i[0], &[401.0, 402.0]);
        assert_slice_close(&delayed.i2_o[0], &[411.0, 412.0]);

        assert_slice_close(&delayed.v1_i[1], &[351.0, 352.0]);
        assert_slice_close(&delayed.v1_o[1], &[361.0, 362.0]);
        assert_slice_close(&delayed.i1_i[1], &[371.0, 372.0]);
        assert_slice_close(&delayed.i1_o[1], &[381.0, 382.0]);
        assert_slice_close(&delayed.v2_i[1], &[200.5, 201.0]);
        assert_slice_close(&delayed.v2_o[1], &[205.5, 206.0]);
        assert_slice_close(&delayed.i2_i[1], &[210.5, 211.0]);
        assert_slice_close(&delayed.i2_o[1], &[215.5, 216.0]);

        assert_eq!(history.head_time_ps(), Some(20));
    }

    #[test]
    fn cpl_native_right_consts_updates_real_pole_convolutions() {
        let mut runtime = empty_runtime(2);
        runtime.h1t[0][0] = Some(tms(
            false,
            0.0,
            [
                term(2.0, 0.0, 3.0, 5.0),
                term(-1.0, 0.0, 7.0, 11.0),
                term(0.5, 0.0, 13.0, 17.0),
            ],
        ));
        runtime.h3t[0][0][0] = Some(tms(
            false,
            0.5,
            [
                term(1.0, 0.0, 0.1, 1.0),
                term(2.0, 0.0, 0.2, 2.0),
                term(4.0, 0.0, 0.4, 4.0),
            ],
        ));
        runtime.h2t[0][0][0] = Some(tms(
            false,
            0.25,
            [
                term(2.0, 0.0, 1.0, 10.0),
                term(3.0, 0.0, 2.0, 20.0),
                term(5.0, 0.0, 4.0, 40.0),
            ],
        ));

        let mut delayed = delayed_fixture(true, &[0.25, 0.0]);
        delayed.v1_i[0][0] = 2.0;
        delayed.v2_i[0][0] = 3.0;
        delayed.v1_o[0][0] = 4.0;
        delayed.v2_o[0][0] = 5.0;
        delayed.i1_i[0][0] = 7.0;
        delayed.i2_i[0][0] = 11.0;
        delayed.i1_o[0][0] = 13.0;
        delayed.i2_o[0][0] = 17.0;

        let rhs = runtime
            .right_consts(0.25, 0.5, &[19.0, 0.0], &[23.0, 0.0], &delayed)
            .expect("right constants");

        assert!(rhs.ext);
        assert_slice_close(&rhs.ratio, &[0.25, 0.0]);
        assert_slice_close(&rhs.ff, &[228.0, 0.0]);
        assert_slice_close(&rhs.gg, &[69.2, 0.0]);
        assert_eq!(runtime.h1e[0][0], [1.0, 1.0, 1.0]);

        let h3 = runtime.h3t[0][0][0].as_ref().expect("h3");
        assert_slice_close(
            &[h3.tm[0].cnv_i, h3.tm[1].cnv_i, h3.tm[2].cnv_i],
            &[2.6, 5.2, 10.4],
        );
        assert_slice_close(
            &[h3.tm[0].cnv_o, h3.tm[1].cnv_o, h3.tm[2].cnv_o],
            &[5.5, 11.0, 22.0],
        );

        let h2 = runtime.h2t[0][0][0].as_ref().expect("h2");
        assert_slice_close(
            &[h2.tm[0].cnv_i, h2.tm[1].cnv_i, h2.tm[2].cnv_i],
            &[19.0, 29.0, 49.0],
        );
        assert_slice_close(
            &[h2.tm[0].cnv_o, h2.tm[1].cnv_o, h2.tm[2].cnv_o],
            &[40.0, 65.0, 115.0],
        );
    }

    #[test]
    fn cpl_native_right_consts_updates_complex_pair_convolutions() {
        let mut runtime = empty_runtime(2);
        runtime.h1t[0][0] = Some(tms(
            true,
            0.0,
            [
                term(2.0, 0.0, 3.0, 5.0),
                term(7.0, 0.0, 11.0, 13.0),
                term(17.0, 4.0, 19.0, 23.0),
            ],
        ));
        runtime.h3t[0][0][0] = Some(tms(
            true,
            0.5,
            [
                term(2.0, 0.0, 1.0, 2.0),
                term(3.0, 0.0, 5.0, 7.0),
                term(11.0, 4.0, 13.0, 17.0),
            ],
        ));

        let mut delayed = delayed_fixture(false, &[0.0, 0.0]);
        delayed.v1_i[0][0] = 2.0;
        delayed.v2_i[0][0] = 3.0;
        delayed.v1_o[0][0] = 4.0;
        delayed.v2_o[0][0] = 5.0;

        let rhs = runtime
            .right_consts(0.0, 0.5, &[19.0, 0.0], &[23.0, 0.0], &delayed)
            .expect("right constants");

        assert!(!rhs.ext);
        assert_slice_close(&rhs.ff, &[-122.5, 0.0]);
        assert_slice_close(&rhs.gg, &[-182.5, 0.0]);
        assert_eq!(runtime.h1e[0][0], [1.0, 1.0, 0.0]);

        let h3 = runtime.h3t[0][0][0].as_ref().expect("h3");
        assert_slice_close(
            &[h3.tm[0].cnv_i, h3.tm[1].cnv_i, h3.tm[2].cnv_i],
            &[6.0, 12.5, 40.5],
        );
        assert_slice_close(
            &[h3.tm[0].cnv_o, h3.tm[1].cnv_o, h3.tm[2].cnv_o],
            &[11.0, 20.5, 66.5],
        );
    }

    #[test]
    fn cpl_ibm2_setup_matches_ngspice_constants() {
        let runtime = cpl_ibm2_runtime();

        assert_eq!(runtime.no_l, 2);
        assert_slice_close(&runtime.taul_ps, &[866.1685875634142, 823.5102904624812]);
        assert_slice_close(
            &runtime.h1c[0],
            &[-3.195331046730955e-5, 7.280668191646096e-6],
        );
        assert_slice_close(
            &runtime.h1c[1],
            &[7.2806681916482995e-6, -3.19533104673183e-5],
        );

        assert_slice_close(
            &[runtime.h2c[0][0][0], runtime.h2c[1][1][1]],
            &[5.340077891682099e-14, 3.7913678396327893e-13],
        );
        assert_slice_close(
            &[runtime.h3c[0][0][0], runtime.h3c[1][1][1]],
            &[-1.2336321137798462e-5, -1.961698932941142e-5],
        );
        assert!(!runtime.h1t[0][0].as_ref().expect("h1[0][0]").if_img);
        assert!(runtime.h1t[0][1].as_ref().expect("h1[0][1]").if_img);
    }

    #[test]
    fn cpl3_4_line_setup_matches_ngspice_constants() {
        let runtime = cpl3_4_line_runtime();

        assert_eq!(runtime.no_l, 4);
        assert_slice_close(
            &runtime.taul_ps,
            &[
                402.8719007104209,
                287.80082831420305,
                64.45224292429776,
                460.6800877199479,
            ],
        );

        let expected_h1c = [
            [
                -3.1769723674031704e-5,
                4.993238447733109e-5,
                -4.911467822702347e-5,
                3.0089229537245273e-5,
            ],
            [
                4.993238454511319e-5,
                -8.088411555274803e-5,
                8.002190036359935e-5,
                -4.911467815996109e-5,
            ],
            [
                -4.911467837856522e-5,
                8.00219004991741e-5,
                -8.088411541260412e-5,
                4.993238432347305e-5,
            ],
            [
                3.0089229672815213e-5,
                -4.9114678311504714e-5,
                4.993238439125772e-5,
                -3.176972353691346e-5,
            ],
        ];
        for (actual, expected) in runtime.h1c.iter().zip(expected_h1c) {
            assert_slice_close(actual, &expected);
        }

        assert_slice_close(
            &[
                runtime.h2c[0][0][0],
                runtime.h2c[1][1][1],
                runtime.h2c[2][2][2],
                runtime.h2c[3][3][3],
            ],
            &[
                1.9317099460034934e-13,
                4.204053079761114e-8,
                4.327326379190999e-5,
                4.204942856642855e-8,
            ],
        );
        assert_slice_close(
            &[
                runtime.h3c[0][0][0],
                runtime.h3c[1][1][1],
                runtime.h3c[2][2][2],
                runtime.h3c[3][3][3],
            ],
            &[
                -2.331830040724032e-7,
                -3.017519368438641e-7,
                -8.03639399818854e-5,
                -4.914591490597437e-8,
            ],
        );
    }

    // Reference f64 reimplementation of pade_apx (the pre-dd algorithm) for
    // cross-checking the dd version on identical inputs.
    fn pade_apx_f64_ref(a_b: f64, b: &[f64]) -> (usize, [f64; 6]) {
        fn eval2(a: f64, bb: f64, c: f64, x: f64) -> f64 {
            a * x * x + bb * x + c
        }
        fn root3(a1: f64, a2: f64, a3: f64, x: f64) -> f64 {
            let t1 = x * (x * (x + a1) + a2) + a3;
            let t2 = x * (2.0 * a1 + 3.0 * x) + a2;
            x - t1 / t2
        }
        fn div3(a1: f64, a3: f64, x: f64) -> (f64, f64) {
            (a1 + x, -a3 / x)
        }
        fn find_roots(mut a1: f64, mut a2: f64, a3: f64) -> (f64, f64, f64, bool) {
            let q = (a1 * a1 - 3.0 * a2) / 9.0;
            let p = (2.0 * a1 * a1 * a1 - 9.0 * a1 * a2 + 27.0 * a3) / 54.0;
            let mut t = q * q * q - p * p;
            let mut x;
            if t >= 0.0 {
                t = (p / (q * q.sqrt())).acos();
                x = -2.0 * q.sqrt() * (t / 3.0).cos() - a1 / 3.0;
            } else if p > 0.0 {
                t = ((-t).sqrt() + p).powf(1.0 / 3.0);
                x = -(t + q / t) - a1 / 3.0;
            } else if p == 0.0 {
                x = -a1 / 3.0;
            } else {
                t = ((-t).sqrt() - p).powf(1.0 / 3.0);
                x = t + q / t - a1 / 3.0;
            }
            let original = x;
            let mut it = 0;
            loop {
                let next = root3(a1, a2, a3, x);
                if (next - x).abs() <= 5.0e-4 {
                    break;
                }
                it += 1;
                if it == 32 {
                    x = original;
                    break;
                }
                x = next;
            }
            let x1 = x;
            (a1, a2) = div3(a1, a3, x);
            t = a1 * a1 - 4.0 * a2;
            if t < 0.0 {
                (x1, -0.5 * a1, 0.5 * (-t).sqrt(), true)
            } else {
                t = t.sqrt();
                let x2 = if a1 >= 0.0 { -0.5 * (a1 + t) } else { -0.5 * (a1 - t) };
                (x1, x2, a2 / x2, false)
            }
        }
        fn get_c(q1: f64, q2: f64, q3: f64, p1: f64, p2: f64, a: f64, bb: f64) -> (f64, f64) {
            let d = (3.0 * (a * a - bb * bb) + 2.0 * p1 * a + p2).powi(2)
                + (6.0 * a * bb + 2.0 * p1 * bb).powi(2);
            let mut n = -(q1 * (a * a - bb * bb) + q2 * a + q3) * (6.0 * a * bb + 2.0 * p1 * bb);
            n += (2.0 * q1 * a * bb + q2 * bb) * (3.0 * (a * a - bb * bb) + 2.0 * p1 * a + p2);
            let ci = n / d;
            n = (3.0 * (a * a - bb * bb) + 2.0 * p1 * a + p2) * (q1 * (a * a - bb * bb) + q2 * a + q3);
            n += (6.0 * a * bb + 2.0 * p1 * bb) * (2.0 * q1 * a * bb + q2 * bb);
            let cr = n / d;
            (cr, ci)
        }
        let mut at = [[0.0f64; 4]; 4];
        at[0][0] = 1.0 - a_b;
        at[0][1] = b[1];
        at[0][2] = b[2];
        at[0][3] = -b[3];
        at[1][0] = b[1];
        at[1][1] = b[2];
        at[1][2] = b[3];
        at[1][3] = -b[4];
        at[2][0] = b[2];
        at[2][1] = b[3];
        at[2][2] = b[4];
        at[2][3] = -b[5];
        // gaussian_elimination_at, dims=3
        for i in 0..3 {
            let mut imax = i;
            let mut max = at[i][i].abs();
            for row in i + 1..3 {
                if at[row][i].abs() > max {
                    imax = row;
                    max = at[row][i].abs();
                }
            }
            if imax != i {
                for col in i..=3 {
                    let value = at[i][col];
                    at[i][col] = at[imax][col];
                    at[imax][col] = value;
                }
            }
            let factor = 1.0 / at[i][i];
            at[i][i] = 1.0;
            for col in i + 1..=3 {
                at[i][col] *= factor;
            }
            for row in 0..3 {
                if i == row {
                    continue;
                }
                let factor = at[row][i];
                at[row][i] = 0.0;
                for col in i + 1..=3 {
                    at[row][col] -= factor * at[i][col];
                }
            }
        }
        let p3 = at[0][3];
        let p2 = at[1][3];
        let p1 = at[2][3];
        let q1 = p1 + b[1];
        let q2 = b[1] * p1 + p2 + b[2];
        let q3 = p3 * a_b;
        let (x1, x2, x3, cplx) = find_roots(p1, p2, p3);
        let mut pade = [0.0f64; 6];
        pade[3] = x1;
        pade[4] = x2;
        pade[5] = x3;
        pade[0] = eval2(q1 - p1, q2 - p2, q3 - p3, x1) / eval2(3.0, 2.0 * p1, p2, x1);
        if cplx {
            let (cr, ci) = get_c(q1 - p1, q2 - p2, q3 - p3, p1, p2, x2, x3);
            pade[1] = cr;
            pade[2] = ci;
            (2, pade)
        } else {
            pade[1] = eval2(q1 - p1, q2 - p2, q3 - p3, x2) / eval2(3.0, 2.0 * p1, p2, x2);
            pade[2] = eval2(q1 - p1, q2 - p2, q3 - p3, x3) / eval2(3.0, 2.0 * p1, p2, x3);
            (1, pade)
        }
    }

    #[test]
    fn pade_apx_dd_matches_f64_reference() {
        // Representative well-behaved inputs (well-separated roots, no
        // degeneracy). Captured-shape moments from the CPL setup chain.
        let cases: &[(f64, [f64; 7])] = &[
            (
                0.0,
                [1.0, 8.0e-3, 2.0e-5, 5.5e-9, 7.0e-12, 9.0e-15, 0.0],
            ),
            (
                0.5,
                [1.0, 1.11e-2, 2.95e-5, -9.36e-9, 1.2e-11, -2.0e-14, 0.0],
            ),
            (
                0.01,
                [1.0, -6.0e-3, 1.5e-5, -3.0e-9, 5.0e-13, -7.0e-16, 0.0],
            ),
        ];
        for (a_b, b) in cases {
            let b_dd: Vec<Dd> = b.iter().map(|&v| Dd::from_f64(v)).collect();
            let (kind, pade) = pade_apx(Dd::from_f64(*a_b), &b_dd).expect("dd pade");
            let (kind_ref, pade_ref) = pade_apx_f64_ref(*a_b, b);
            assert_eq!(kind, kind_ref, "kind mismatch for a_b={a_b}");
            for i in 0..6 {
                let got = pade[i].to_f64();
                let want = pade_ref[i];
                let tol = 1.0e-6 * want.abs().max(1.0e-12);
                assert!(
                    (got - want).abs() <= tol,
                    "pade[{i}] a_b={a_b}: got={got:e} want={want:e}"
                );
            }
        }
    }

    /// Documents the double-double-true slow convolution pole for the
    /// `cpl3_4_line` matrices, and the verified divergence from ngspice's value.
    ///
    /// PROVENANCE / divergence note (verified 2026-06 against the ngspice-46
    /// debug build via gdb on this exact deck):
    ///
    /// The slowest pole of `h3[0][0][3]` (mode 3, taul ~= 460.68 ps) is the
    /// long-memory integrator that builds the multi-bounce reflection tail. Its
    /// value is catastrophically ill-conditioned: the modal-moment extraction
    /// recovers coefficients down to ~1e-22 from eigenvalue samples of magnitude
    /// ~40-54, so a ~1e-13 perturbation in the raw eigenvalues moves this pole by
    /// several percent.
    ///
    ///   - ngspice-46 (f64): tm[2].x = -5.6852265592287934e-6
    ///   - old RSpice (f64): tm[2].x = -5.4435560366884854e-6
    ///   - this dd setup:    tm[2].x = -5.2738988766680034e-6   (asserted below)
    ///
    /// gdb on ngspice shows its raw eigenvalue samples carry ~1.6e-13 roundoff
    /// (e.g. W[0][0] = 39.071302057905996 and the freq-0 eigenvalue sum =
    /// 111.09897480432316), whereas the dd setup computes the *true* eigenvalues
    /// (W[0][0] = 39.0713020579061592, sum = 111.09897480432352) to ~1e-28. The
    /// dd extraction is therefore the accurate one; ngspice's -5.685e-6 is its
    /// own f64 roundoff amplified by the ill-conditioning, NOT the physically
    /// correct pole. We deliberately do NOT reproduce ngspice's roundoff, so this
    /// test pins the dd-true value rather than ngspice's. (See report: the deck
    /// reference oracle was generated by a different ngspice and disagrees with
    /// the current ngspice build by a comparable margin, so neither value makes
    /// the stale deck oracle pass.)
    #[test]
    fn cpl3_4_line_slow_pole_is_dd_true_value() {
        let runtime = cpl3_4_line_runtime();
        let h3 = runtime.h3t[0][0][3].as_ref().expect("h3[0][0][3] present");
        let slow_pole = h3.tm[2].x;
        let dd_true = -5.2738988766680034e-6;
        // Tight tolerance: this is the deterministic dd output, pinned to guard
        // against regressions in the dd setup chain.
        assert!(
            (slow_pole - dd_true).abs() <= 1.0e-12 + 1.0e-9 * dd_true.abs(),
            "slow pole tm[2].x = {slow_pole:.17e}, expected dd-true {dd_true:.17e}"
        );

        // It must NOT have collapsed to ngspice's roundoff value or the old f64
        // value: confirm the dd setup actually changed the pole and lands on the
        // accurate branch (between the two prior values, nearer the true root).
        let ngspice_roundoff = -5.6852265592287934e-6;
        let old_f64 = -5.4435560366884854e-6;
        assert!(
            (slow_pole - ngspice_roundoff).abs() > 1.0e-7,
            "slow pole unexpectedly matches ngspice roundoff value"
        );
        assert!(
            (slow_pole - old_f64).abs() > 1.0e-8,
            "slow pole unexpectedly matches old f64 value"
        );
    }
}

#[cfg(test)]
mod oracle_replay {
    use super::*;

    /// Replay ngspice's committed (v, i) history for cpl3_4_line's P1 element
    /// and compare the branch rhs vectors point by point.
    ///
    /// The fixtures were extracted with gdb from the vendored ngspice debug
    /// build running tests/transmission/cpl3_4_line.cir: committed history
    /// samples at cplload.c:686 (truncated-picosecond time plus per-conductor
    /// v_i/v_o/i_i/i_o) and per-load branch rhs at cplload.c:578 (fractional
    /// time and CKTdelta, per-conductor ff/gg; the last record per truncated
    /// label is the accepted iterate). Driving the runtime with the oracle's
    /// own inputs pins the multiconductor convolution - including the mixed
    /// integer-picosecond/fractional-delta clock - independently of solver
    /// grid differences. All accepted steps reproduce the oracle's ff/gg
    /// vectors to sub-1e-9 relative error.
    #[test]
    fn replay_oracle_cpl34_p1() {
        let hv_text = include_str!("transmission_line/testdata/cpl34_p1_hv.dat");
        let in_text = include_str!("transmission_line/testdata/cpl34_p1_in.dat");

        const N: usize = 4;
        struct Hv {
            t_ps: i64,
            v_i: Vec<f64>,
            v_o: Vec<f64>,
            i_i: Vec<f64>,
            i_o: Vec<f64>,
        }
        let hv: Vec<Hv> = hv_text
            .lines()
            .map(|line| {
                let f: Vec<f64> = line
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();
                assert_eq!(f.len(), 1 + 4 * N);
                Hv {
                    t_ps: f[0] as i64,
                    v_i: f[1..1 + N].to_vec(),
                    v_o: f[1 + N..1 + 2 * N].to_vec(),
                    i_i: f[1 + 2 * N..1 + 3 * N].to_vec(),
                    i_o: f[1 + 3 * N..1 + 4 * N].to_vec(),
                }
            })
            .collect();
        // Last record per truncated-picosecond label = the accepted iterate.
        let mut inputs: std::collections::HashMap<i64, (f64, Vec<f64>, Vec<f64>)> =
            std::collections::HashMap::new();
        for line in in_text.lines() {
            let f: Vec<f64> = line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            assert_eq!(f.len(), 2 + 2 * N);
            let label = (f[0] * 1.0e12).trunc() as i64;
            inputs.insert(label, (f[1], f[2..2 + N].to_vec(), f[2 + N..2 + 2 * N].to_vec()));
        }

        // .MODEL LOSSYMODE CPL: R=diag(0.3), L tridiag(9n, 5.4n),
        // G=0, C tridiag(3.5e-13, -3e-14), length=6.3.
        let mut r = vec![vec![0.0; N]; N];
        let mut l = vec![vec![0.0; N]; N];
        let g = vec![vec![0.0; N]; N];
        let mut c = vec![vec![0.0; N]; N];
        for i in 0..N {
            r[i][i] = 0.3;
            l[i][i] = 9.0e-9;
            c[i][i] = 3.5e-13;
            if i + 1 < N {
                l[i][i + 1] = 5.4e-9;
                l[i + 1][i] = 5.4e-9;
                c[i][i + 1] = -3.0e-14;
                c[i + 1][i] = -3.0e-14;
            }
        }
        let mut rt = NativeCplRuntime::setup(&r, &l, &c, &g, 6.3).unwrap();

        let dc1 = vec![0.0; N];
        let dc2 = vec![0.0; N];
        rt.initialize_dc_convolutions(&dc1, &dc2).unwrap();
        let mut history = NativeCplViHistory::new(N, dc1.clone(), dc2.clone()).unwrap();
        history
            .push_sample(NativeCplViSample::new(
                0,
                dc1.clone(),
                dc2.clone(),
                vec![0.0; N],
                vec![0.0; N],
            ))
            .unwrap();

        let mut prev_v_i = dc1.clone();
        let mut prev_v_o = dc2.clone();
        let mut t1_ps = 0i64;
        let mut compared = 0usize;
        let mut worst: (f64, i64) = (0.0, 0);
        let mut first_bad: Option<i64> = None;
        for sample in &hv {
            // The IN rows carry ngspice's exact fractional CKTtime/CKTdelta, so
            // the candidate clock replays the oracle bit-for-bit (keying both
            // sides by the truncated label reproduces ngspice's own (int) cast).
            let Some((dt_oracle, ff_oracle, gg_oracle)) = inputs.get(&sample.t_ps) else {
                continue;
            };
            {
                let mut trial_history = history.clone();
                let plan = rt
                    .step_stamp_plan(
                        t1_ps,
                        sample.t_ps,
                        *dt_oracle,
                        &prev_v_i,
                        &prev_v_o,
                        &mut trial_history,
                    )
                    .unwrap();
                let mut scale = 1.0e-9f64;
                for m in 0..N {
                    scale = scale.max(ff_oracle[m].abs()).max(gg_oracle[m].abs());
                }
                let mut err = 0.0f64;
                for m in 0..N {
                    err = err
                        .max((plan.ff[m] - ff_oracle[m]).abs() / scale)
                        .max((plan.gg[m] - gg_oracle[m]).abs() / scale);
                }
                compared += 1;
                if err > worst.0 {
                    worst = (err, sample.t_ps);
                }
                if err > 1.0e-6 && first_bad.is_none() {
                    first_bad = Some(sample.t_ps);
                    println!(
                        "first divergence at t={}ps: ours ff[0]={:.12e} oracle ff[0]={:.12e}",
                        sample.t_ps, plan.ff[0], ff_oracle[0]
                    );
                }
            }

            let h_grid = (sample.t_ps - t1_ps) as f64 * 1.0e-12;
            rt.commit_step(
                t1_ps,
                sample.t_ps,
                *dt_oracle,
                h_grid,
                &prev_v_i,
                &prev_v_o,
                &sample.v_i,
                &sample.v_o,
                &sample.i_i,
                &sample.i_o,
                &mut history,
            )
            .unwrap();
            history
                .push_sample(NativeCplViSample::new(
                    sample.t_ps,
                    sample.v_i.clone(),
                    sample.v_o.clone(),
                    sample.i_i.clone(),
                    sample.i_o.clone(),
                ))
                .unwrap();
            prev_v_i = sample.v_i.clone();
            prev_v_o = sample.v_o.clone();
            t1_ps = sample.t_ps;
        }
        assert!(
            compared > 1000,
            "expected to replay the full accepted sequence, compared {compared}"
        );
        assert!(
            first_bad.is_none() && worst.0 < 1.0e-6,
            "CPL rhs fidelity regressed: worst rel err {:.3e} at t={}ps, first>1e-6 at {:?}",
            worst.0,
            worst.1,
            first_bad
        );
    }
}
