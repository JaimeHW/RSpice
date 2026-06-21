//! Noise Analysis Module
//!
//! Computes noise spectral density as a function of frequency.

#![allow(clippy::needless_range_loop)]
//! Supports:
//! - **Thermal noise**: Johnson-Nyquist noise from resistors (4kTR)
//! - **Shot noise**: From PN junctions in diodes and BJTs (2qI)
//! - **Flicker noise**: 1/f^EF noise in semiconductors (KF * I^AF / f^EF)
//!
//! # Algorithm
//! 1. Compute small-signal AC solution at each frequency
//! 2. For each noise source, compute noise current spectral density
//! 3. Use transfer function to compute output voltage noise contribution
//! 4. Sum all contributions (in mean-square) for total output noise
//!
//! # Example
//! ```ignore
//! .NOISE V(out) Vin DEC 10 1Hz 100kHz
//! ```
//! This computes noise at output node referenced to input source Vin.

use crate::Value;
use crate::analysis::AnalysisConfig;

//=============================================================================
// Constants
//=============================================================================

/// Boltzmann constant (J/K)
pub const K_BOLTZMANN: Value = 1.380649e-23;
/// Electron charge (C)
pub const Q_ELECTRON: Value = 1.602176634e-19;
/// Default temperature (K): 27°C = 300.15K (SPICE convention, ngspice REFTEMP)
pub const T_NOMINAL: Value = 300.15;

const BSIM4_MIN_LOG_ARG: Value = 1.0e-38;
const BSIM3_MIN_LOG_ARG: Value = 1.0e-38;
const BSIM3_K_OVER_Q: Value = 8.62e-5;

//=============================================================================
// Noise Source Types
//=============================================================================

/// Bias snapshot for the BSIM4 physical 1/f noise model (`fnoiMod=1`).
#[derive(Debug, Clone)]
pub struct Bsim4FlickerNoise {
    pub multiplier: Value,
    pub cd: Value,
    pub vds: Value,
    pub vdseff: Value,
    pub vsattemp: Value,
    pub ueff: Value,
    pub abulk: Value,
    pub ab_ov_vgst2vtm: Value,
    pub vgsteff: Value,
    pub nstar: Value,
    pub leff: Value,
    pub leff_noise: Value,
    pub litl: Value,
    pub weff: Value,
    pub nf: Value,
    pub coxe: Value,
    pub oxide_trap_density_a: Value,
    pub oxide_trap_density_b: Value,
    pub oxide_trap_density_c: Value,
    pub em: Value,
    pub ef: Value,
}

impl Bsim4FlickerNoise {
    /// Evaluate b4noi.c's physical BSIM4 1/f channel-current PSD.
    pub fn spectral_density(&self, frequency: Value, temperature: Value) -> Value {
        if frequency <= 0.0
            || self.cd == 0.0
            || self.leff <= 0.0
            || self.leff_noise <= 0.0
            || self.weff <= 0.0
            || self.nf <= 0.0
            || self.coxe <= 0.0
            || self.ueff <= 0.0
            || self.abulk <= 0.0
            || self.nstar == 0.0
        {
            return 0.0;
        }

        let cd = self.cd.abs();
        let leff_sq = self.leff_noise * self.leff_noise;
        let esat = 2.0 * self.vsattemp / self.ueff;
        let del_clm = if self.em <= 0.0 || self.litl <= 0.0 || esat <= 0.0 {
            0.0
        } else {
            let arg = ((((self.vds.abs() - self.vdseff) / self.litl) + self.em) / esat)
                .max(BSIM4_MIN_LOG_ARG);
            (self.litl * arg.ln()).max(0.0)
        };
        let eff_freq = frequency.powf(self.ef);
        if eff_freq <= 0.0 || !eff_freq.is_finite() {
            return 0.0;
        }

        let n0 = self.coxe * self.vgsteff / Q_ELECTRON;
        let nl = self.coxe * self.vgsteff * (1.0 - self.ab_ov_vgst2vtm * self.vdseff) / Q_ELECTRON;
        let n0_star = (n0 + self.nstar).max(BSIM4_MIN_LOG_ARG);
        let nl_star = (nl + self.nstar).max(BSIM4_MIN_LOG_ARG);
        let t3 = self.oxide_trap_density_a * (n0_star / nl_star).max(BSIM4_MIN_LOG_ARG).ln();
        let t4 = self.oxide_trap_density_b * (n0 - nl);
        let t5 = self.oxide_trap_density_c * 0.5 * (n0 * n0 - nl * nl);

        let denom_ssi = 1.0e10 * eff_freq * self.abulk * self.coxe * leff_sq;
        let term_ssi = if denom_ssi > 0.0 {
            Q_ELECTRON * Q_ELECTRON * K_BOLTZMANN * cd * temperature * self.ueff / denom_ssi
                * (t3 + t4 + t5)
        } else {
            0.0
        };

        let t8 = self.oxide_trap_density_a
            + self.oxide_trap_density_b * nl
            + self.oxide_trap_density_c * nl * nl;
        let t9 = nl_star * nl_star;
        let denom_clm = 1.0e10 * eff_freq * leff_sq * self.weff * self.nf;
        let clm = if denom_clm > 0.0 && t9 > 0.0 {
            K_BOLTZMANN * temperature * cd * cd / denom_clm * del_clm * t8 / t9
        } else {
            0.0
        };
        let ssi = term_ssi + clm;

        let denom_swi =
            self.weff * self.nf * self.leff * eff_freq * 1.0e10 * self.nstar * self.nstar;
        let swi = if denom_swi > 0.0 {
            self.oxide_trap_density_a * K_BOLTZMANN * temperature / denom_swi * cd * cd
        } else {
            0.0
        };

        let total = ssi + swi;
        if total > 0.0 && total.is_finite() {
            (self.multiplier * (ssi * swi) / total).max(0.0)
        } else {
            0.0
        }
    }
}

/// Bias snapshot for the BSIM3 physical 1/f noise model (`noiMod=2/3/6`).
#[derive(Debug, Clone)]
pub struct Bsim3FlickerNoise {
    pub multiplier: Value,
    pub cd: Value,
    pub vds: Value,
    pub vdseff: Value,
    pub vsattemp: Value,
    pub ueff: Value,
    pub abulk: Value,
    pub ab_ov_vgst2vtm: Value,
    pub vgsteff: Value,
    pub leff: Value,
    pub leff_noise: Value,
    pub litl: Value,
    pub weff: Value,
    pub cox: Value,
    pub oxide_trap_density_a: Value,
    pub oxide_trap_density_b: Value,
    pub oxide_trap_density_c: Value,
    pub em: Value,
    pub ef: Value,
}

impl Bsim3FlickerNoise {
    /// Evaluate b3noi.c's strong-inversion BSIM3 1/f channel-current PSD.
    pub fn spectral_density(&self, frequency: Value, temperature: Value) -> Value {
        if frequency <= 0.0
            || self.cd == 0.0
            || self.leff <= 0.0
            || self.leff_noise <= 0.0
            || self.weff <= 0.0
            || self.cox <= 0.0
            || self.ueff <= 0.0
            || self.abulk <= 0.0
        {
            return 0.0;
        }

        let cd = self.cd.abs();
        let leff_sq = self.leff_noise * self.leff_noise;
        let esat = 2.0 * self.vsattemp / self.ueff;
        let del_clm = if self.em <= 0.0 || self.litl <= 0.0 || esat <= 0.0 {
            0.0
        } else {
            let arg = ((((self.vds.abs() - self.vdseff) / self.litl) + self.em) / esat)
                .max(BSIM3_MIN_LOG_ARG);
            (self.litl * arg.ln()).max(0.0)
        };
        let eff_freq = frequency.powf(self.ef);
        if eff_freq <= 0.0 || !eff_freq.is_finite() {
            return 0.0;
        }

        let n0 = self.cox * self.vgsteff / Q_ELECTRON;
        let nl = self.cox * self.vgsteff * (1.0 - self.ab_ov_vgst2vtm * self.vdseff) / Q_ELECTRON;
        let n0_star = n0 + 2.0e14;
        let nl_star = nl + 2.0e14;
        let ratio = if nl_star != 0.0 {
            (n0_star / nl_star).max(BSIM3_MIN_LOG_ARG)
        } else {
            BSIM3_MIN_LOG_ARG
        };
        let t3 = self.oxide_trap_density_a * ratio.ln();
        let t4 = self.oxide_trap_density_b * (n0 - nl);
        let t5 = self.oxide_trap_density_c * 0.5 * (n0 * n0 - nl * nl);

        let denom_ssi = 1.0e8 * eff_freq * self.abulk * self.cox * leff_sq;
        let term_ssi = if denom_ssi > 0.0 {
            Q_ELECTRON * Q_ELECTRON * BSIM3_K_OVER_Q * cd * temperature * self.ueff / denom_ssi
                * (t3 + t4 + t5)
        } else {
            0.0
        };

        let t8 = self.oxide_trap_density_a
            + self.oxide_trap_density_b * nl
            + self.oxide_trap_density_c * nl * nl;
        let t9 = nl_star * nl_star;
        let denom_clm = 1.0e8 * eff_freq * leff_sq * self.weff;
        let clm = if denom_clm > 0.0 && t9 > 0.0 {
            BSIM3_K_OVER_Q * temperature * cd * cd / denom_clm * del_clm * t8 / t9
        } else {
            0.0
        };
        let ssi = term_ssi + clm;

        let denom_swi = self.weff * self.leff * eff_freq * 4.0e36;
        let swi = if denom_swi > 0.0 {
            self.oxide_trap_density_a * BSIM3_K_OVER_Q * temperature / denom_swi * cd * cd
        } else {
            0.0
        };

        let total = ssi + swi;
        if total > 0.0 && total.is_finite() {
            (self.multiplier * (ssi * swi) / total).max(0.0)
        } else {
            0.0
        }
    }
}

/// One current-injection port used by a correlated noise source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoisePort {
    pub node_pos: usize,
    pub node_neg: usize,
}

/// Per-frequency spectral-density amplitudes for two fully correlated
/// current-noise ports.
#[derive(Debug, Clone, Copy)]
pub struct CorrelatedNoiseDensities {
    pub first_psd: Value,
    pub second_psd: Value,
    pub phase_rad: Value,
}

#[derive(Debug, Clone)]
enum CorrelatedNoisePairModel {
    Bsim4Tnoi2 {
        gamma_gd0: Value,
        ctnoi: Value,
        sigrat: Value,
        multiplier: Value,
    },
}

/// Two fully correlated current-noise ports evaluated as a single
/// covariance contribution.
#[derive(Debug, Clone)]
pub struct CorrelatedNoisePair {
    /// Name of the device/mechanism generating this pair.
    pub device_name: String,
    /// Summary-table mechanism label.
    pub noise_type: NoiseSourceType,
    /// First injection port.
    pub first: NoisePort,
    /// Second injection port.
    pub second: NoisePort,
    /// Thermal-noise temperature offset in kelvin.
    pub temperature_offset: Value,
    model: CorrelatedNoisePairModel,
}

impl CorrelatedNoisePair {
    /// Create a BSIM4 `tnoiMod=2` correlated channel/gate thermal source.
    pub fn bsim4_tnoi2(
        device_name: String,
        first: NoisePort,
        second: NoisePort,
        gamma_gd0: Value,
        ctnoi: Value,
        sigrat: Value,
        multiplier: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Bsim4CorrelatedThermal,
            first,
            second,
            temperature_offset: 0.0,
            model: CorrelatedNoisePairModel::Bsim4Tnoi2 {
                gamma_gd0,
                ctnoi,
                sigrat,
                multiplier,
            },
        }
    }

    /// Current-noise PSDs (A²/Hz) and relative phase at `frequency`.
    pub fn spectral_densities(
        &self,
        frequency: Value,
        temperature: Value,
    ) -> Option<CorrelatedNoiseDensities> {
        match self.model {
            CorrelatedNoisePairModel::Bsim4Tnoi2 {
                gamma_gd0,
                ctnoi,
                sigrat,
                multiplier,
            } => {
                if frequency < 0.0
                    || gamma_gd0 <= 0.0
                    || multiplier <= 0.0
                    || !gamma_gd0.is_finite()
                    || !ctnoi.is_finite()
                    || !sigrat.is_finite()
                    || !multiplier.is_finite()
                {
                    return None;
                }

                let ctnoi_sq = (ctnoi.clamp(0.0, 1.0)).powi(2);
                let omega_sigrat = 2.0 * std::f64::consts::PI * frequency * sigrat;
                let gate_fraction = if omega_sigrat.is_finite() {
                    let shaped = omega_sigrat * omega_sigrat;
                    shaped / (1.0 + shaped)
                } else {
                    1.0
                };
                let first_g = gamma_gd0 * ctnoi_sq * multiplier;
                let second_g = gamma_gd0 * gate_fraction * multiplier;
                let scale = 4.0 * K_BOLTZMANN * (temperature + self.temperature_offset);
                let first_psd = (scale * first_g).max(0.0);
                let second_psd = (scale * second_g).max(0.0);
                if first_psd <= 0.0 && second_psd <= 0.0 {
                    return None;
                }
                Some(CorrelatedNoiseDensities {
                    first_psd,
                    second_psd,
                    phase_rad: std::f64::consts::FRAC_PI_2,
                })
            }
        }
    }
}

/// Types of noise sources in the circuit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseSourceType {
    /// Thermal (Johnson-Nyquist) noise: 4kTR
    Thermal,
    /// Shot noise: 2qI
    Shot,
    /// Flicker (1/f) noise: KF * I^AF / f
    Flicker,
    /// Burst (popcorn) noise: KB * I^AB / (1 + (f/FB)^2)
    /// Lorentzian spectrum with corner frequency FB
    Burst,
    /// Frequency-flat source with an explicitly given spectral density
    /// (Verilog-A `white_noise(pwr)`): Si = pwr, temperature-independent
    White,
    /// Frequency-interpolated spectral density (Verilog-A `noise_table` /
    /// `noise_table_log`)
    Table,
    /// BSIM4 physical channel flicker (`fnoiMod=1`).
    Bsim4Flicker,
    /// BSIM3 physical channel flicker (`noiMod=2/3/6`).
    Bsim3Flicker,
    /// BSIM4 correlated channel/gate thermal noise (`tnoiMod=2`).
    Bsim4CorrelatedThermal,
}

/// Shared tabulated noise PSD: sorted `(frequency, power)` points plus a log-log
/// interpolation flag.
pub type NoiseTable = std::sync::Arc<(Vec<(Value, Value)>, bool)>;

/// A noise source in the circuit
#[derive(Debug, Clone)]
pub struct NoiseSource {
    /// Name of the device generating this noise
    pub device_name: String,
    /// Type of noise
    pub noise_type: NoiseSourceType,
    /// Node where noise current is injected (+)
    pub node_pos: usize,
    /// Node where noise current is injected (-)
    pub node_neg: usize,
    /// Spectral density parameter (R for thermal, I for shot, KF for flicker)
    pub parameter: Value,
    /// Flicker noise exponent (AF, typically 1.0)
    pub af: Value,
    /// Flicker frequency exponent (EF, typically 1.0)
    pub ef: Value,
    /// Current for flicker/burst noise
    pub current: Value,
    /// Corner frequency for burst noise (FB, Hz)
    pub corner_freq: Value,
    /// Thermal-noise temperature offset in kelvin: ngspice `dtemp`
    /// semantics, where the source runs at the analysis temperature plus
    /// this per-instance offset (nevalsrc.c THERMNOISE).
    pub temperature_offset: Value,
    /// Tabulated PSD for [`NoiseSourceType::Table`]: sorted (f, p) points
    /// and the log-log interpolation flag, scaled by `parameter`
    pub table: Option<NoiseTable>,
    /// BSIM4 `fnoiMod=1` physical flicker-noise state.
    pub bsim4_flicker: Option<std::sync::Arc<Bsim4FlickerNoise>>,
    /// BSIM3 `noiMod=2/3/6` physical flicker-noise state.
    pub bsim3_flicker: Option<std::sync::Arc<Bsim3FlickerNoise>>,
}

impl NoiseSource {
    /// Create a thermal noise source (resistor)
    pub fn thermal(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        resistance: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Thermal,
            node_pos,
            node_neg,
            parameter: resistance,
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Create a shot noise source (diode/BJT junction)
    pub fn shot(device_name: String, node_pos: usize, node_neg: usize, current: Value) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Shot,
            node_pos,
            node_neg,
            parameter: current.abs(),
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Create a flicker (1/f) noise source
    pub fn flicker(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kf: Value,
        af: Value,
        current: Value,
    ) -> Self {
        Self::flicker_with_frequency_exponent(device_name, node_pos, node_neg, kf, af, 1.0, current)
    }

    /// Create a flicker (1/f^EF) noise source
    pub fn flicker_with_frequency_exponent(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kf: Value,
        af: Value,
        ef: Value,
        current: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Flicker,
            node_pos,
            node_neg,
            parameter: kf,
            af,
            ef,
            current,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Create a frequency-flat source from an explicit spectral density
    /// (A²/Hz for current injection, V²/Hz when injected at a branch row)
    pub fn white(device_name: String, node_pos: usize, node_neg: usize, psd: Value) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::White,
            node_pos,
            node_neg,
            parameter: psd,
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Create a 1/f^EF source from an explicit spectral density at 1 Hz:
    /// Si(f) = psd / f^ef (Verilog-A `flicker_noise(pwr, exp)`)
    pub fn flicker_psd(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        psd: Value,
        ef: Value,
    ) -> Self {
        Self::flicker_with_frequency_exponent(device_name, node_pos, node_neg, psd, 1.0, ef, 1.0)
    }

    /// Create a frequency-interpolated source from sorted (f, p) points
    /// (Verilog-A `noise_table` / `noise_table_log`), scaled by `scale`
    pub fn tabulated(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        scale: Value,
        points: Vec<(Value, Value)>,
        log_interp: bool,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Table,
            node_pos,
            node_neg,
            parameter: scale,
            af: 1.0,
            ef: 1.0,
            current: 0.0,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: Some(std::sync::Arc::new((points, log_interp))),
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Create a BSIM4 physical channel flicker source (`fnoiMod=1`).
    pub fn bsim4_flicker(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        model: Bsim4FlickerNoise,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Bsim4Flicker,
            node_pos,
            node_neg,
            parameter: 0.0,
            af: 1.0,
            ef: model.ef,
            current: model.cd,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: Some(std::sync::Arc::new(model)),
            bsim3_flicker: None,
        }
    }

    /// Create a BSIM3 physical channel flicker source (`noiMod=2/3/6`).
    pub fn bsim3_flicker(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        model: Bsim3FlickerNoise,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Bsim3Flicker,
            node_pos,
            node_neg,
            parameter: 0.0,
            af: 1.0,
            ef: model.ef,
            current: model.cd,
            corner_freq: 1.0,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: Some(std::sync::Arc::new(model)),
        }
    }

    /// Create a burst (popcorn) noise source
    ///
    /// Burst noise has a Lorentzian spectrum: Si = KB * I^AB / (1 + (f/FB)^2)
    /// where KB is the burst noise coefficient, AB is the exponent (typically 2),
    /// and FB is the corner frequency.
    pub fn burst(
        device_name: String,
        node_pos: usize,
        node_neg: usize,
        kb: Value,
        ab: Value,
        current: Value,
        corner_freq: Value,
    ) -> Self {
        Self {
            device_name,
            noise_type: NoiseSourceType::Burst,
            node_pos,
            node_neg,
            parameter: kb,
            af: ab, // Reuse af field for AB exponent
            ef: 1.0,
            current,
            corner_freq,
            temperature_offset: 0.0,
            table: None,
            bsim4_flicker: None,
            bsim3_flicker: None,
        }
    }

    /// Compute current noise spectral density (A²/Hz) at given frequency
    pub fn spectral_density(&self, frequency: Value, temperature: Value) -> Value {
        match self.noise_type {
            NoiseSourceType::Thermal => {
                // Thermal noise: Si = 4kT/R (current noise spectral density)
                // at the instance temperature, ngspice nevalsrc.c THERMNOISE:
                // 4k·(CKTtemp + dtemp)·g.
                if self.parameter > 0.0 {
                    4.0 * K_BOLTZMANN * (temperature + self.temperature_offset) / self.parameter
                } else {
                    0.0
                }
            }
            NoiseSourceType::Shot => {
                // Shot noise: Si = 2qI (A²/Hz)
                2.0 * Q_ELECTRON * self.parameter
            }
            NoiseSourceType::Flicker => {
                // Flicker noise: Si = KF * I^AF / f^EF (A²/Hz)
                if frequency > 0.0 {
                    self.parameter * self.current.abs().powf(self.af) / frequency.powf(self.ef)
                } else {
                    0.0 // Avoid division by zero
                }
            }
            NoiseSourceType::Burst => {
                // Burst (popcorn) noise: Si = KB * I^AB / (1 + (f/FB)^2)
                // Lorentzian spectrum with corner frequency FB
                let kb = self.parameter;
                let ab = self.af;
                let fb = self.corner_freq;
                let f_ratio = frequency / fb;
                kb * self.current.abs().powf(ab) / (1.0 + f_ratio * f_ratio)
            }
            // Explicit spectral density evaluated at the operating point
            NoiseSourceType::White => self.parameter.max(0.0),
            // Interpolated table, clamped to the endpoints outside the
            // covered range; log-log when flagged
            NoiseSourceType::Table => {
                let Some(table) = &self.table else { return 0.0 };
                let (points, log_interp) = (&table.0, table.1);
                (self.parameter * Self::interpolate_table(points, log_interp, frequency)).max(0.0)
            }
            NoiseSourceType::Bsim4Flicker => self
                .bsim4_flicker
                .as_ref()
                .map(|model| model.spectral_density(frequency, temperature))
                .unwrap_or(0.0),
            NoiseSourceType::Bsim3Flicker => self
                .bsim3_flicker
                .as_ref()
                .map(|model| model.spectral_density(frequency, temperature))
                .unwrap_or(0.0),
            NoiseSourceType::Bsim4CorrelatedThermal => 0.0,
        }
    }

    /// Interpolate a sorted (f, p) table at `frequency`, clamping to the
    /// endpoints outside the covered range
    fn interpolate_table(points: &[(Value, Value)], log_interp: bool, frequency: Value) -> Value {
        match points {
            [] => return 0.0,
            [only] => return only.1,
            _ => {}
        }
        let first = points[0];
        let last = points[points.len() - 1];
        if frequency <= first.0 {
            return first.1;
        }
        if frequency >= last.0 {
            return last.1;
        }
        let upper = points.partition_point(|&(f, _)| f < frequency);
        let (f0, p0) = points[upper - 1];
        let (f1, p1) = points[upper];
        if f1 == f0 {
            return p0;
        }
        if log_interp {
            // Strict positivity was validated at compile time
            let t = (frequency.ln() - f0.ln()) / (f1.ln() - f0.ln());
            (p0.ln() + t * (p1.ln() - p0.ln())).exp()
        } else {
            let t = (frequency - f0) / (f1 - f0);
            p0 + t * (p1 - p0)
        }
    }
}

//=============================================================================
// Noise Analysis Engine
//=============================================================================

/// Noise Analysis engine
#[derive(Debug)]
pub struct NoiseAnalysis {
    config: AnalysisConfig,
    /// Frequency points for analysis
    frequencies: Vec<Value>,
    /// Temperature for noise calculations (K)
    temperature: Value,
    /// Output node for noise measurement
    output_node: Option<usize>,
    /// Reference node (usually ground)
    reference_node: usize,
    /// Input source name for input-referred noise
    input_source: Option<String>,
}

impl NoiseAnalysis {
    /// Create a new noise analysis
    pub fn new(config: AnalysisConfig) -> Self {
        Self {
            config,
            frequencies: Vec::new(),
            temperature: T_NOMINAL,
            output_node: None,
            reference_node: 0,
            input_source: None,
        }
    }

    /// Set output node for noise measurement
    pub fn set_output(&mut self, node: usize) {
        self.output_node = Some(node);
    }

    /// Set reference node (default is ground = 0)
    pub fn set_reference(&mut self, node: usize) {
        self.reference_node = node;
    }

    /// Set input source for input-referred noise calculation
    pub fn set_input_source(&mut self, source_name: String) {
        self.input_source = Some(source_name);
    }

    /// Set temperature for noise calculations
    pub fn set_temperature(&mut self, temp_kelvin: Value) {
        self.temperature = temp_kelvin;
    }

    /// Set up decade frequency sweep
    pub fn decade_sweep(&mut self, start: Value, stop: Value, points_per_decade: usize) {
        let start_log = start.log10();
        let stop_log = stop.log10();
        let num_decades = stop_log - start_log;
        let total_points = (num_decades * points_per_decade as f64).ceil() as usize;

        self.frequencies = (0..=total_points)
            .map(|i| {
                let log_f = start_log + (stop_log - start_log) * (i as f64) / (total_points as f64);
                10.0_f64.powf(log_f)
            })
            .collect();
    }

    /// Set up linear frequency sweep
    pub fn linear_sweep(&mut self, start: Value, stop: Value, points: usize) {
        self.frequencies = (0..points)
            .map(|i| start + (stop - start) * (i as f64) / ((points - 1).max(1) as f64))
            .collect();
    }

    /// Get frequency points
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }

    /// Get temperature
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    /// Get config
    pub fn config(&self) -> &AnalysisConfig {
        &self.config
    }
}

impl Default for NoiseAnalysis {
    fn default() -> Self {
        Self::new(AnalysisConfig::default())
    }
}

//=============================================================================
// Noise Result
//=============================================================================

/// Noise analysis result at a single frequency
#[derive(Debug, Clone)]
pub struct NoiseResult {
    /// Frequency (Hz)
    pub frequency: Value,
    /// Total output voltage noise spectral density (V²/Hz)
    pub output_noise_density: Value,
    /// Input-referred noise spectral density (V²/Hz)
    pub input_referred_density: Value,
    /// Individual noise contributions from each source
    pub contributions: Vec<NoiseContribution>,
}

/// Contribution from a single noise source
#[derive(Debug, Clone)]
pub struct NoiseContribution {
    /// Device name
    pub device_name: String,
    /// Noise type
    pub noise_type: NoiseSourceType,
    /// Contribution to output noise (V²/Hz)
    pub output_contribution: Value,
    /// Percentage of total noise
    pub percentage: Value,
}

impl NoiseSourceType {
    /// Short human-readable mechanism label for summary tables.
    pub fn label(&self) -> &'static str {
        match self {
            NoiseSourceType::Thermal => "thermal",
            NoiseSourceType::Shot => "shot",
            NoiseSourceType::Flicker => "flicker",
            NoiseSourceType::Burst => "burst",
            NoiseSourceType::White => "white",
            NoiseSourceType::Table => "table",
            NoiseSourceType::Bsim4Flicker => "bsim4-flicker",
            NoiseSourceType::Bsim3Flicker => "bsim3-flicker",
            NoiseSourceType::Bsim4CorrelatedThermal => "bsim4-correlated-thermal",
        }
    }
}

/// Band-integrated contribution of one device/mechanism pair — one row of
/// the classic ranked noise-summary table.
#[derive(Debug, Clone)]
pub struct IntegratedContribution {
    /// Device instance name.
    pub device_name: String,
    /// Noise mechanism.
    pub noise_type: NoiseSourceType,
    /// Output-referred noise power integrated over the band (V²).
    pub integrated_power: Value,
    /// Share of total integrated output noise (percent).
    pub percentage: Value,
}

impl NoiseResult {
    /// Get total output noise in V/√Hz (RMS voltage noise density)
    pub fn output_noise_rms(&self) -> Value {
        self.output_noise_density.sqrt()
    }

    /// Get input-referred noise in V/√Hz
    pub fn input_referred_rms(&self) -> Value {
        self.input_referred_density.sqrt()
    }

    /// Get total output noise in dBV/Hz
    pub fn output_noise_dbv(&self) -> Value {
        if self.output_noise_density > 0.0 {
            10.0 * self.output_noise_density.log10()
        } else {
            -200.0
        }
    }

    /// Get the dominant noise source
    pub fn dominant_source(&self) -> Option<&NoiseContribution> {
        self.contributions.iter().max_by(|a, b| {
            a.output_contribution
                .partial_cmp(&b.output_contribution)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

//=============================================================================
// Integrated Noise Calculator
//=============================================================================

/// Calculator for integrated noise over a frequency band
#[derive(Debug)]
pub struct IntegratedNoise {
    /// Results at each frequency point
    results: Vec<NoiseResult>,
}

impl IntegratedNoise {
    /// Create from a vector of noise results
    pub fn new(results: Vec<NoiseResult>) -> Self {
        Self { results }
    }

    /// Calculate total integrated output noise over the frequency band (V RMS)
    /// Uses trapezoidal integration
    pub fn total_output_noise(&self) -> Value {
        if self.results.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.results.len() {
            let f1 = self.results[i - 1].frequency;
            let f2 = self.results[i].frequency;
            let s1 = self.results[i - 1].output_noise_density;
            let s2 = self.results[i].output_noise_density;

            // Trapezoidal integration
            total += 0.5 * (s1 + s2) * (f2 - f1);
        }

        total.sqrt() // Return RMS voltage
    }

    /// Per-device, per-mechanism output-noise contributions integrated over
    /// the analysis band (trapezoidal, matching `total_output_noise`),
    /// ranked descending by integrated power — the ranked noise-summary
    /// table analog designers read first.
    pub fn contribution_summary(&self) -> Vec<IntegratedContribution> {
        use std::collections::HashMap;

        let mut totals: HashMap<(String, &'static str), (NoiseSourceType, Value)> = HashMap::new();

        for window in self.results.windows(2) {
            let (a, b) = (&window[0], &window[1]);
            let df = b.frequency - a.frequency;
            if df <= 0.0 {
                continue;
            }

            // Index the right edge once so each pair match is O(1).
            let mut right: HashMap<(&str, &'static str), Value> =
                HashMap::with_capacity(b.contributions.len());
            for contribution in &b.contributions {
                right.insert(
                    (
                        contribution.device_name.as_str(),
                        contribution.noise_type.label(),
                    ),
                    contribution.output_contribution,
                );
            }

            for contribution in &a.contributions {
                let key = (
                    contribution.device_name.as_str(),
                    contribution.noise_type.label(),
                );
                let s_right = right
                    .get(&key)
                    .copied()
                    .unwrap_or(contribution.output_contribution);
                let power = 0.5 * (contribution.output_contribution + s_right) * df;
                let entry = totals
                    .entry((
                        contribution.device_name.clone(),
                        contribution.noise_type.label(),
                    ))
                    .or_insert((contribution.noise_type, 0.0));
                entry.1 += power;
            }
        }

        let total: Value = totals.values().map(|(_, power)| *power).sum();
        let mut summary: Vec<IntegratedContribution> = totals
            .into_iter()
            .map(
                |((device_name, _), (noise_type, integrated_power))| IntegratedContribution {
                    device_name,
                    noise_type,
                    integrated_power,
                    percentage: if total > 0.0 {
                        100.0 * integrated_power / total
                    } else {
                        0.0
                    },
                },
            )
            .collect();
        summary.sort_by(|x, y| {
            y.integrated_power
                .partial_cmp(&x.integrated_power)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        summary
    }

    /// Calculate integrated input-referred noise (V RMS)
    pub fn total_input_referred_noise(&self) -> Value {
        if self.results.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.results.len() {
            let f1 = self.results[i - 1].frequency;
            let f2 = self.results[i].frequency;
            let s1 = self.results[i - 1].input_referred_density;
            let s2 = self.results[i].input_referred_density;

            total += 0.5 * (s1 + s2) * (f2 - f1);
        }

        total.sqrt()
    }

    /// Get the frequency with maximum noise density
    pub fn peak_noise_frequency(&self) -> Option<Value> {
        self.results
            .iter()
            .max_by(|a, b| {
                a.output_noise_density
                    .partial_cmp(&b.output_noise_density)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.frequency)
    }

    /// Get all results
    pub fn results(&self) -> &[NoiseResult] {
        &self.results
    }
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Calculate thermal noise voltage spectral density for a resistor (V²/Hz)
#[inline]
pub fn thermal_voltage_noise(resistance: Value, temperature: Value) -> Value {
    4.0 * K_BOLTZMANN * temperature * resistance
}

/// Calculate thermal noise current spectral density for a resistor (A²/Hz)
#[inline]
pub fn thermal_current_noise(resistance: Value, temperature: Value) -> Value {
    if resistance > 0.0 {
        4.0 * K_BOLTZMANN * temperature / resistance
    } else {
        0.0
    }
}

/// Calculate shot noise current spectral density (A²/Hz)
#[inline]
pub fn shot_noise(current: Value) -> Value {
    2.0 * Q_ELECTRON * current.abs()
}

/// Calculate equivalent noise bandwidth for a first-order lowpass filter
#[inline]
pub fn noise_bandwidth_first_order(f3db: Value) -> Value {
    std::f64::consts::FRAC_PI_2 * f3db
}

/// Calculate equivalent noise bandwidth for a second-order lowpass filter (Q=0.707)
#[inline]
pub fn noise_bandwidth_second_order_butterworth(f3db: Value) -> Value {
    1.11 * f3db
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod summary_tests {
    use super::*;

    fn result_at(frequency: Value, r1: Value, d1: Value) -> NoiseResult {
        NoiseResult {
            frequency,
            output_noise_density: r1 + d1,
            input_referred_density: r1 + d1,
            contributions: vec![
                NoiseContribution {
                    device_name: "r1".to_string(),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: r1,
                    percentage: 0.0,
                },
                NoiseContribution {
                    device_name: "d1".to_string(),
                    noise_type: NoiseSourceType::Shot,
                    output_contribution: d1,
                    percentage: 0.0,
                },
            ],
        }
    }

    #[test]
    fn contribution_summary_integrates_ranks_and_normalizes() {
        // Flat densities over a 100 Hz band: powers integrate exactly.
        let integrated = IntegratedNoise::new(vec![
            result_at(100.0, 4e-18, 1e-18),
            result_at(200.0, 4e-18, 1e-18),
        ]);

        let summary = integrated.contribution_summary();
        assert_eq!(summary.len(), 2);

        // Ranked descending: the resistor dominates.
        assert_eq!(summary[0].device_name, "r1");
        assert!((summary[0].integrated_power - 4e-16).abs() < 1e-28);
        assert_eq!(summary[1].device_name, "d1");
        assert!((summary[1].integrated_power - 1e-16).abs() < 1e-28);

        // Percentages cover the whole and match the 4:1 split.
        assert!((summary[0].percentage - 80.0).abs() < 1e-9);
        assert!((summary[1].percentage - 20.0).abs() < 1e-9);

        // Band total agrees with the per-contributor sum.
        let total_v = integrated.total_output_noise();
        assert!(((total_v * total_v) - 5e-16).abs() < 1e-27);
    }
}
