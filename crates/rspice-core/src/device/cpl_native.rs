//! Native CPL constants and dormant runtime helpers.
//!
//! This is a direct, private port of the ngspice CPL setup math that produces
//! the CPLine constant tables. Runtime helpers are kept private and dormant
//! until branch-current transient stamping is complete enough to activate.

#![allow(dead_code)]

use std::{collections::VecDeque, fmt};

const MAX_CP_TX_LINES: usize = 8;
const MAX_DEG: usize = 8;
const LEFT_DEG: usize = 7;
const CPL_MIN_SERIES_RESISTANCE_PER_LENGTH: f64 = 1.0e-4;
const EPSILON: f64 = 1.0e-88;
const EPSI_MULT: f64 = 1.0e-28;
const EPSI2: f64 = 1.0e-8;

type Matrix = Vec<Vec<f64>>;
type PolyMatrix = Vec<Vec<Vec<f64>>>;

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
    InsufficientHistory {
        target_ps: f64,
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
            Self::InsufficientHistory { target_ps } => write!(
                f,
                "native CPL VI history does not bracket delayed time {target_ps:.6} ps"
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
    fn from_pade(atten: f64, pade: &[f64]) -> Self {
        Self {
            if_img: ((pade[6] - 1.0) as i32) != 0,
            aten: atten,
            tm: [
                NativeCplTerm {
                    c: pade[0] * atten,
                    x: pade[3],
                    cnv_i: 0.0,
                    cnv_o: 0.0,
                },
                NativeCplTerm {
                    c: pade[1] * atten,
                    x: pade[4],
                    cnv_i: 0.0,
                    cnv_o: 0.0,
                },
                NativeCplTerm {
                    c: pade[2] * atten,
                    x: pade[5],
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
}

#[derive(Debug, Clone, PartialEq)]
struct NativeCplRightConsts {
    ext: bool,
    ratio: Vec<f64>,
    ff: Vec<f64>,
    gg: Vec<f64>,
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
    poly: Vec<Vec<f64>>,
    c0: Vec<f64>,
}

impl MultOut {
    fn new(dim: usize, poly_len: usize) -> Self {
        Self {
            poly: vec![vec![0.0; poly_len]; dim],
            c0: vec![0.0; dim],
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SingleOut {
    poly: Vec<f64>,
    c0: f64,
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
    r: Matrix,
    g: Matrix,
    l: Matrix,
    c: Matrix,
    zy: Matrix,
    sv: Matrix,
    d: Vec<f64>,
    y5: Matrix,
    y5_inv: Matrix,
    sv_inv: Matrix,
    frequency: Vec<f64>,
    si: Matrix,
    si_inv: Matrix,
    si_sv_inv_p: PolyMatrix,
    sip: PolyMatrix,
    si_inv_p: PolyMatrix,
    sv_inv_p: PolyMatrix,
    w: Vec<Vec<f64>>,
    iwi: Vec<Vec<MultOut>>,
    iwv: Vec<Vec<MultOut>>,
    siv: Vec<Vec<SingleOut>>,
    tau: Vec<f64>,
    scaling_f: f64,
    scaling_f2: f64,
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
            r: r_full,
            g: g_full,
            l: l_full,
            c: c_full,
            zy: zero_matrix(dim),
            sv: zero_matrix(dim),
            d: vec![0.0; dim],
            y5: zero_matrix(dim),
            y5_inv: zero_matrix(dim),
            sv_inv: zero_matrix(dim),
            frequency: vec![0.0; LEFT_DEG + 1],
            si: zero_matrix(dim),
            si_inv: zero_matrix(dim),
            si_sv_inv_p: zero_poly_matrix(dim, LEFT_DEG + 1),
            sip: zero_poly_matrix(dim, LEFT_DEG + 1),
            si_inv_p: zero_poly_matrix(dim, LEFT_DEG + 1),
            sv_inv_p: zero_poly_matrix(dim, LEFT_DEG + 1),
            w: vec![vec![0.0; MAX_DEG]; dim],
            iwi: vec![vec![MultOut::new(dim, LEFT_DEG + 1); dim]; dim],
            iwv: vec![vec![MultOut::new(dim, LEFT_DEG + 1); dim]; dim],
            siv: vec![vec![SingleOut::default(); dim]; dim],
            tau: vec![0.0; dim],
            scaling_f: 1.0,
            scaling_f2: 1.0,
        })
    }

    fn coupled(&mut self) -> Result<(), NativeCplError> {
        self.scaling_f = 1.0;
        self.scaling_f2 = 1.0;

        self.loop_zy(0.0)?;
        self.eval_frequency()?;
        self.eval_si_si_inv(0.0)?;
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
            taul_ps[mode] = self.tau[mode] * 1.0e12;
        }

        for row in 0..dim {
            for col in 0..dim {
                if self.siv[row][col].c0 != 0.0 {
                    let series = NativeCplTimeSeries::from_pade(
                        self.siv[row][col].c0,
                        &self.siv[row][col].poly,
                    );
                    h1c[row][col] = series.constant();
                    h1t[row][col] = Some(series);
                }

                for mode in 0..dim {
                    if self.iwi[row][col].c0[mode] != 0.0 {
                        let series = NativeCplTimeSeries::from_pade(
                            self.iwi[row][col].c0[mode],
                            &self.iwi[row][col].poly[mode],
                        );
                        h2c[row][col][mode] = series.constant();
                        h2t[row][col][mode] = Some(series);
                    }

                    if self.iwv[row][col].c0[mode] != 0.0 {
                        let series = NativeCplTimeSeries::from_pade(
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

    fn eval_si_si_inv(&mut self, y: f64) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                self.si_inv[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.si_inv[row][col] += self.sv_inv[row][mode]
                        * (y * self.r[mode][col] + self.scaling_f * self.l[mode][col]);
                }
            }
        }

        for row in 0..self.dim {
            let scale = self.d[row].sqrt();
            for col in 0..self.dim {
                self.si_inv[row][col] /= scale;
            }
        }

        self.si = invert_ngspice(&self.si_inv)?;
        Ok(())
    }

    fn loop_zy(&mut self, y: f64) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                self.zy[row][col] = self.scaling_f * self.c[row][col] + self.g[row][col] * y;
            }
        }

        self.diag()?;

        let mut fmin = self.d[0];
        for &value in &self.d[1..] {
            if value < fmin {
                fmin = value;
            }
        }
        if fmin < 0.0 {
            return Err(NativeCplError::NonPositiveCapacitanceMatrix);
        }

        let fmin = fmin.sqrt();
        let fmin_inv = 1.0 / fmin;

        for mode in 0..self.dim {
            self.d[mode] = self.d[mode].sqrt();
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.y5[row][col] = self.d[row] * self.sv[col][row];
                self.y5_inv[row][col] = self.sv[col][row] / self.d[row];
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv_inv[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.sv_inv[row][col] += self.sv[row][mode] * self.y5[mode][col];
                }
            }
        }
        self.y5.clone_from(&self.sv_inv);

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv_inv[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.sv_inv[row][col] += self.sv[row][mode] * self.y5_inv[mode][col];
                }
            }
        }
        self.y5_inv.clone_from(&self.sv_inv);

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.zy[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.zy[row][col] += (self.scaling_f * self.l[row][mode]
                        + self.r[row][mode] * y)
                        * self.y5[mode][col];
                }
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv_inv[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.sv_inv[row][col] += self.y5[row][mode] * self.zy[mode][col];
                }
            }
        }
        self.zy.clone_from(&self.sv_inv);

        self.diag()?;

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv_inv[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.sv_inv[row][col] += self.sv[mode][row] * self.y5[mode][col];
                }
                self.sv_inv[row][col] *= fmin_inv;
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.zy[row][col] = 0.0;
                for mode in 0..self.dim {
                    self.zy[row][col] += self.y5_inv[row][mode] * self.sv[mode][col];
                }
                self.zy[row][col] *= fmin;
            }
        }
        self.sv.clone_from(&self.zy);

        Ok(())
    }

    fn eval_frequency(&mut self) -> Result<(), NativeCplError> {
        let mut min = self.d[0];
        for &value in &self.d[1..] {
            if value < min {
                min = value;
            }
        }

        if min <= 0.0 {
            return Err(NativeCplError::NonPositiveModeFrequency);
        }

        self.scaling_f2 = 1.0 / min;
        self.scaling_f = self.scaling_f2.sqrt();

        let spacing = self.length * 8.0;
        self.frequency[0] = 0.0;
        for idx in 1..=LEFT_DEG {
            self.frequency[idx] = self.frequency[idx - 1] + spacing;
        }

        for value in &mut self.d {
            *value *= self.scaling_f2;
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
                let mut temp = 0.0;
                for mode in 0..self.dim {
                    temp += self.si[row][mode] * self.sv_inv[mode][col];
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

    fn approx_mode(&mut self, mode: usize) -> f64 {
        let w0 = self.w[mode][0];
        let w1 = self.w[mode][1] / w0;
        let w2 = self.w[mode][2] / w0;
        let w3 = self.w[mode][3] / w0;
        let w4 = self.w[mode][4] / w0;
        let w5 = self.w[mode][5] / w0;

        let y1 = 0.5 * w1;
        let y2 = w2 - y1 * y1;
        let y3 = 3.0 * w3 - 3.0 * y1 * y2;
        let y4 = 12.0 * w4 - 3.0 * y2 * y2 - 4.0 * y1 * y3;
        let y5 = 60.0 * w5 - 5.0 * y1 * y4 - 10.0 * y2 * y3;
        let y6 = -10.0 * y3 * y3 - 15.0 * y2 * y4 - 6.0 * y1 * y5;

        let delay = w0.sqrt() * self.length / self.scaling_f;
        let atten = (-delay * y1).exp();

        let mut a = [0.0; 6];
        a[1] = y2 / 2.0;
        a[2] = y3 / 6.0;
        a[3] = y4 / 24.0;
        a[4] = y5 / 120.0;
        a[5] = y6 / 720.0;

        for value in a.iter_mut().skip(1) {
            *value *= -delay;
        }

        let mut b = [0.0; 6];
        b[0] = 1.0;
        b[1] = a[1];
        for idx in 2..=5 {
            b[idx] = 0.0;
            for j in 1..=idx {
                b[idx] += j as f64 * a[j] * b[idx - j];
            }
            b[idx] /= idx as f64;
        }

        for (idx, value) in b.iter().enumerate() {
            self.w[mode][idx] = value * atten;
        }

        delay
    }

    fn generate_out(&mut self) -> Result<(), NativeCplError> {
        for row in 0..self.dim {
            for col in 0..self.dim {
                let constant = self.si_sv_inv_p[row][col][0];
                self.siv[row][col].c0 = constant;
                if constant == 0.0 {
                    continue;
                }

                for idx in 0..=LEFT_DEG {
                    self.si_sv_inv_p[row][col][idx] /= constant;
                }

                let a_b = if row == col {
                    (self.g[row][row] / self.r[row][row]).sqrt() / constant
                } else {
                    0.0
                };
                let (kind, pade) = pade_apx(a_b, &self.si_sv_inv_p[row][col])?;
                self.siv[row][col].poly = pade_to_vec(kind, pade);
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                for mode in 0..self.dim {
                    let constant = self.iwi[row][col].c0[mode];
                    if constant == 0.0 {
                        continue;
                    }

                    let a_b = if row == col && mode == row {
                        (-(self.g[row][row] * self.r[row][row]).sqrt() * self.length).exp()
                            / constant
                    } else {
                        0.0
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
                    if constant == 0.0 {
                        continue;
                    }

                    let a_b = if row == col && mode == row {
                        (self.g[row][row] / self.r[row][row]).sqrt()
                            * (-(self.g[row][row] * self.r[row][row]).sqrt() * self.length).exp()
                            / constant
                    } else {
                        0.0
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
                if value > fmax {
                    fmax = value;
                } else if value < fmin {
                    fmin = value;
                }
            }
        }

        let scale = 2.0 / (fmin + fmax);
        for row in 0..self.dim {
            for col in row..self.dim {
                self.zy[row][col] *= scale;
            }
        }

        for row in 0..self.dim {
            for col in 0..self.dim {
                self.sv[row][col] = if row == col { 1.0 } else { 0.0 };
            }
        }

        for row in 0..self.dim.saturating_sub(1) {
            let mut mode = row + 1;
            let mut max_value = self.zy[row][mode].abs();
            for col in mode + 1..self.dim {
                if ((self.zy[row][col].abs() * 1.0e7) as i32) > ((1.0e7 * max_value) as i32) {
                    max_value = self.zy[row][col].abs();
                    mode = col;
                }
            }
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
            self.rotate(entry.row, entry.col);
            self.reordering(entry.row, &mut row_entries);
            if entry.col + 1 != self.dim {
                self.reordering(entry.col, &mut row_entries);
            }

            rotations += 1;
            if rotations > 1_000_000 {
                return Err(NativeCplError::DiagonalizationDidNotConverge);
            }
        }

        for mode in 0..self.dim {
            self.d[mode] = self.zy[mode][mode] / scale;
        }

        Ok(())
    }

    fn reordering(&self, row: usize, rows: &mut Vec<RowEntry>) {
        let mut mode = row + 1;
        let mut max_value = self.zy[row][mode].abs();
        for col in mode + 1..self.dim {
            if ((self.zy[row][col].abs() * 1.0e7) as i32) > ((1.0e7 * max_value) as i32) {
                max_value = self.zy[row][col].abs();
                mode = col;
            }
        }

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

    fn rotate(&mut self, p: usize, q: usize) {
        let ld = -self.zy[p][q];
        let mu = 0.5 * (self.zy[p][p] - self.zy[q][q]);
        let ve = (ld * ld + mu * mu).sqrt();
        let co = ((ve + mu.abs()) / (2.0 * ve)).sqrt();
        let si = sgn(mu) * ld / (2.0 * ve * co);

        let mut t = vec![0.0; self.dim];
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
                self.zy[p][col] = t[col] * co - self.zy[q][col] * si;
            } else {
                self.zy[p][col] = t[col] * co - self.zy[col][q] * si;
            }
        }

        for col in q + 1..self.dim {
            if col == p {
                continue;
            }
            self.zy[q][col] = t[col] * si + self.zy[q][col] * co;
        }

        for col in 0..p {
            if col == q {
                continue;
            }
            self.zy[col][p] = t[col] * co - self.zy[col][q] * si;
        }

        for col in 0..q {
            if col == p {
                continue;
            }
            self.zy[col][q] = t[col] * si + self.zy[col][q] * co;
        }

        let z_pp = self.zy[p][p];
        let z_qq = self.zy[q][q];
        let z_pq = self.zy[p][q];
        self.zy[p][p] = z_pp * co * co + z_qq * si * si - 2.0 * z_pq * si * co;
        self.zy[q][q] = z_pp * si * si + z_qq * co * co + 2.0 * z_pq * si * co;
        self.zy[p][q] = 0.0;

        let mut sv_p = vec![0.0; self.dim];
        let mut sv_q = vec![0.0; self.dim];
        for row in 0..self.dim {
            sv_p[row] = self.sv[row][p];
            sv_q[row] = self.sv[row][q];
        }

        for row in 0..self.dim {
            self.sv[row][p] = sv_p[row] * co - sv_q[row] * si;
            self.sv[row][q] = sv_p[row] * si + sv_q[row] * co;
        }
    }
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
    Ok(())
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

fn zero_poly_matrix(dim: usize, poly_len: usize) -> PolyMatrix {
    vec![vec![vec![0.0; poly_len]; dim]; dim]
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
    matrix: &mut PolyMatrix,
    frequency: &[f64],
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

fn match_coefficients(n: usize, xa: &[f64], ya: &[f64]) -> Result<Vec<f64>, NativeCplError> {
    let mut x = xa[..=n].to_vec();
    let mut y = ya[..=n].to_vec();
    let mut cof = vec![0.0; n + 1];

    for degree in 0..=n {
        cof[degree] = polint(&x[..=n - degree], &y[..=n - degree], 0.0)?;

        let mut xmin = 1.0e38;
        let mut pivot = 0usize;
        for idx in 0..=n - degree {
            if x[idx].abs() < xmin {
                xmin = x[idx].abs();
                pivot = idx;
            }
            if x[idx] != 0.0 {
                y[idx] = (y[idx] - cof[degree]) / x[idx];
            }
        }

        for idx in pivot + 1..=n - degree {
            y[idx - 1] = y[idx];
            x[idx - 1] = x[idx];
        }
    }

    Ok(cof)
}

fn polint(xa_zero: &[f64], ya_zero: &[f64], x: f64) -> Result<f64, NativeCplError> {
    let n = xa_zero.len();
    let mut xa = vec![0.0; n + 1];
    let mut ya = vec![0.0; n + 1];
    for idx in 0..n {
        xa[idx + 1] = xa_zero[idx];
        ya[idx + 1] = ya_zero[idx];
    }

    let mut ns = 1usize;
    let mut dif = (x - xa[1]).abs();
    let mut c = vec![0.0; n + 1];
    let mut d = vec![0.0; n + 1];

    for idx in 1..=n {
        let dift = (x - xa[idx]).abs();
        if dift < dif {
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
            let ho = xa[idx] - x;
            let hp = xa[idx + m] - x;
            let w = c[idx + 1] - d[idx];
            let den = ho - hp;
            if den == 0.0 {
                return Err(NativeCplError::InterpolationFailure);
            }
            let den = w / den;
            d[idx] = hp * den;
            c[idx] = ho * den;
        }

        let dy = if 2 * ns < n - m {
            c[ns + 1]
        } else {
            let value = d[ns];
            ns -= 1;
            value
        };
        y += dy;
    }

    Ok(y)
}

fn matrix_p_mult(
    a: &PolyMatrix,
    d: &[Vec<f64>],
    b: &PolyMatrix,
    dim: usize,
    deg: usize,
    deg_o: usize,
) -> Vec<Vec<MultOut>> {
    let mut x = vec![vec![MultOut::new(dim, deg_o + 1); dim]; dim];
    let mut t = zero_poly_matrix(dim, deg_o + 1);

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
                if constant != 0.0 {
                    poly[0] = 1.0;
                    for coeff in poly.iter_mut().take(deg_o + 1).skip(1) {
                        *coeff /= constant;
                    }
                }
                x[row][col].poly[mode] = poly;
            }
        }
    }

    x
}

fn mult_p(p1: &[f64], p2: &[f64], d1: usize, d2: usize, d3: usize) -> Vec<f64> {
    let mut p3 = vec![0.0; d3 + 1];
    for i in 0..=d1 {
        let mut j = i;
        for k in 0..=d2 {
            if j > d3 {
                break;
            }
            p3[j] += p1[i] * p2[k];
            j += 1;
        }
    }
    p3
}

fn invert_ngspice(left: &Matrix) -> Result<Matrix, NativeCplError> {
    let dims = left.len();
    let dim = 2 * dims;
    let mut a = vec![vec![0.0; dim + 1]; dim];

    for row in 0..dims {
        for col in 0..dims {
            a[row][col] = left[row][col];
        }
        for col in dims..2 * dims {
            a[row][col] = 0.0;
        }
        a[row][row + dims] = 1.0;
    }

    gaussian_elimination2(&mut a, dims, -1)?;

    let mut inverse = zero_matrix(dims);
    for row in 0..dims {
        for col in 0..dims {
            inverse[row][col] = a[row][col + dims];
        }
    }
    Ok(inverse)
}

fn gaussian_elimination2(a: &mut [Vec<f64>], dims: usize, kind: i32) -> Result<(), NativeCplError> {
    let dim = if kind == -1 { 2 * dims } else { dims };

    for i in 0..dims {
        let mut imax = i;
        let mut max = a[i][i].abs();
        for row in i + 1..dim {
            if a[row][i].abs() > max {
                imax = row;
                max = a[row][i].abs();
            }
        }
        if max < EPSILON {
            return Err(NativeCplError::SingularMatrix("setup inverse"));
        }

        if imax != i {
            for col in i..=dim {
                let value = a[i][col];
                a[i][col] = a[imax][col];
                a[imax][col] = value;
            }
        }

        let factor = 1.0 / a[i][i];
        a[i][i] = 1.0;
        for col in i + 1..=dim {
            a[i][col] *= factor;
        }

        for row in 0..dims {
            if i == row {
                continue;
            }
            let factor = a[row][i];
            a[row][i] = 0.0;
            for col in i + 1..=dim {
                a[row][col] -= factor * a[i][col];
            }
        }
    }

    Ok(())
}

fn pade_to_vec(kind: usize, pade: [f64; 6]) -> Vec<f64> {
    vec![
        pade[0],
        pade[1],
        pade[2],
        pade[3],
        pade[4],
        pade[5],
        kind as f64,
    ]
}

fn pade_apx(a_b: f64, b: &[f64]) -> Result<(usize, [f64; 6]), NativeCplError> {
    let mut at = [[0.0; 4]; 4];
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

    gaussian_elimination_at(&mut at, 3)?;

    let p3 = at[0][3];
    let p2 = at[1][3];
    let p1 = at[2][3];
    let q1 = p1 + b[1];
    let q2 = b[1] * p1 + p2 + b[2];
    let q3 = p3 * a_b;

    let roots = find_roots(p1, p2, p3);
    let mut pade = [0.0; 6];
    pade[3] = roots.x1;
    pade[4] = roots.x2;
    pade[5] = roots.x3;
    pade[0] = eval2(q1 - p1, q2 - p2, q3 - p3, roots.x1) / eval2(3.0, 2.0 * p1, p2, roots.x1);

    if roots.complex_pair {
        let (cr, ci) = get_c(q1 - p1, q2 - p2, q3 - p3, p1, p2, roots.x2, roots.x3);
        pade[1] = cr;
        pade[2] = ci;
        Ok((2, pade))
    } else {
        pade[1] = eval2(q1 - p1, q2 - p2, q3 - p3, roots.x2) / eval2(3.0, 2.0 * p1, p2, roots.x2);
        pade[2] = eval2(q1 - p1, q2 - p2, q3 - p3, roots.x3) / eval2(3.0, 2.0 * p1, p2, roots.x3);
        Ok((1, pade))
    }
}

fn gaussian_elimination_at(at: &mut [[f64; 4]; 4], dims: usize) -> Result<(), NativeCplError> {
    let dim = dims;

    for i in 0..dim {
        let mut imax = i;
        let mut max = at[i][i].abs();
        for row in i + 1..dim {
            if at[row][i].abs() > max {
                imax = row;
                max = at[row][i].abs();
            }
        }
        if max < EPSI_MULT {
            return Err(NativeCplError::SingularMatrix("Pade approximation"));
        }

        if imax != i {
            for col in i..=dim {
                let value = at[i][col];
                at[i][col] = at[imax][col];
                at[imax][col] = value;
            }
        }

        let factor = 1.0 / at[i][i];
        at[i][i] = 1.0;
        for col in i + 1..=dim {
            at[i][col] *= factor;
        }

        for row in 0..dim {
            if i == row {
                continue;
            }
            let factor = at[row][i];
            at[row][i] = 0.0;
            for col in i + 1..=dim {
                at[row][col] -= factor * at[i][col];
            }
        }
    }

    Ok(())
}

fn eval2(a: f64, b: f64, c: f64, x: f64) -> f64 {
    a * x * x + b * x + c
}

fn get_c(q1: f64, q2: f64, q3: f64, p1: f64, p2: f64, a: f64, b: f64) -> (f64, f64) {
    let d =
        (3.0 * (a * a - b * b) + 2.0 * p1 * a + p2).powi(2) + (6.0 * a * b + 2.0 * p1 * b).powi(2);
    let mut n = -(q1 * (a * a - b * b) + q2 * a + q3) * (6.0 * a * b + 2.0 * p1 * b);
    n += (2.0 * q1 * a * b + q2 * b) * (3.0 * (a * a - b * b) + 2.0 * p1 * a + p2);
    let ci = n / d;

    n = (3.0 * (a * a - b * b) + 2.0 * p1 * a + p2) * (q1 * (a * a - b * b) + q2 * a + q3);
    n += (6.0 * a * b + 2.0 * p1 * b) * (2.0 * q1 * a * b + q2 * b);
    let cr = n / d;

    (cr, ci)
}

#[derive(Debug, Clone, Copy)]
struct Roots {
    x1: f64,
    x2: f64,
    x3: f64,
    complex_pair: bool,
}

fn find_roots(mut a1: f64, mut a2: f64, a3: f64) -> Roots {
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
    let mut iterations = 0usize;
    loop {
        let next = root3(a1, a2, a3, x);
        if (next - x).abs() <= 5.0e-4 {
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

    t = a1 * a1 - 4.0 * a2;
    if t < 0.0 {
        Roots {
            x1,
            x2: -0.5 * a1,
            x3: 0.5 * (-t).sqrt(),
            complex_pair: true,
        }
    } else {
        t = t.sqrt();
        let x2 = if a1 >= 0.0 {
            -0.5 * (a1 + t)
        } else {
            -0.5 * (a1 - t)
        };
        Roots {
            x1,
            x2,
            x3: a2 / x2,
            complex_pair: false,
        }
    }
}

fn root3(a1: f64, a2: f64, a3: f64, x: f64) -> f64 {
    let t1 = x * (x * (x + a1) + a2) + a3;
    let t2 = x * (2.0 * a1 + 3.0 * x) + a2;
    x - t1 / t2
}

fn div3(a1: f64, a3: f64, x: f64) -> (f64, f64) {
    (a1 + x, -a3 / x)
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
}
