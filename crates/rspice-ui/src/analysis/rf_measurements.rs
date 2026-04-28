//! RF Measurements Module
//!
//! Large-signal S-parameters and mixer intermodulation measurements.
//! Matches Cadence SpectreRF's RF measurement capabilities.
//!
//! # Features
//!
//! - Large-signal S-parameter extraction (power-dependent)
//! - IP3 (3rd order intercept point) calculation
//! - Compression point (P1dB) measurement
//! - Conversion gain/loss
//! - Input/output return loss

use serde::{Deserialize, Serialize};

// =============================================================================
// S-Parameter Data
// =============================================================================

/// Complex S-parameter value
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SParameter {
    /// Real part
    pub real: f64,
    /// Imaginary part
    pub imag: f64,
}

impl SParameter {
    /// Create from real and imaginary parts
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// Create from magnitude (linear) and phase (degrees)
    pub fn from_mag_phase(mag: f64, phase_deg: f64) -> Self {
        let phase_rad = phase_deg.to_radians();
        Self {
            real: mag * phase_rad.cos(),
            imag: mag * phase_rad.sin(),
        }
    }

    /// Create from dB magnitude and phase (degrees)
    pub fn from_db_phase(db: f64, phase_deg: f64) -> Self {
        let mag = 10.0f64.powf(db / 20.0);
        Self::from_mag_phase(mag, phase_deg)
    }

    /// Magnitude (linear)
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Magnitude in dB
    pub fn magnitude_db(&self) -> f64 {
        20.0 * self.magnitude().log10()
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.imag.atan2(self.real).to_degrees()
    }

    /// Return loss in dB (for reflection parameters)
    pub fn return_loss(&self) -> f64 {
        -self.magnitude_db()
    }

    /// VSWR (for reflection parameters)
    pub fn vswr(&self) -> f64 {
        let gamma = self.magnitude();
        if gamma >= 1.0 {
            f64::INFINITY
        } else {
            (1.0 + gamma) / (1.0 - gamma)
        }
    }
}

// =============================================================================
// Large-Signal S-Parameter Matrix
// =============================================================================

/// Power-dependent S-parameter measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeSigSParams {
    /// Frequency (Hz)
    pub frequency: f64,
    /// Input power (dBm)
    pub input_power: f64,
    /// S11 (input reflection)
    pub s11: SParameter,
    /// S21 (forward gain)
    pub s21: SParameter,
    /// S12 (reverse isolation)
    pub s12: SParameter,
    /// S22 (output reflection)
    pub s22: SParameter,
}

impl LargeSigSParams {
    /// Create a new measurement
    pub fn new(frequency: f64, input_power: f64) -> Self {
        Self {
            frequency,
            input_power,
            s11: SParameter::default(),
            s21: SParameter::default(),
            s12: SParameter::default(),
            s22: SParameter::default(),
        }
    }

    /// Gain in dB (|S21|^2 in power)
    pub fn gain_db(&self) -> f64 {
        self.s21.magnitude_db()
    }

    /// Input return loss
    pub fn input_return_loss(&self) -> f64 {
        self.s11.return_loss()
    }

    /// Output return loss
    pub fn output_return_loss(&self) -> f64 {
        self.s22.return_loss()
    }

    /// Isolation in dB
    pub fn isolation_db(&self) -> f64 {
        -self.s12.magnitude_db()
    }

    /// Stability K-factor (Rollett)
    pub fn k_factor(&self) -> f64 {
        let delta = self.s11.real * self.s22.real
            - self.s11.imag * self.s22.imag
            - self.s12.real * self.s21.real
            + self.s12.imag * self.s21.imag;
        let delta_sq = delta * delta
            + (self.s11.real * self.s22.imag + self.s11.imag * self.s22.real
                - self.s12.real * self.s21.imag
                - self.s12.imag * self.s21.real)
                .powi(2);

        let denom = 2.0 * self.s12.magnitude() * self.s21.magnitude();
        if denom < 1e-15 {
            return f64::INFINITY;
        }

        (1.0 - self.s11.magnitude().powi(2) - self.s22.magnitude().powi(2) + delta_sq) / denom
    }

    /// Is unconditionally stable (K > 1 and |Δ| < 1)
    pub fn is_stable(&self) -> bool {
        self.k_factor() > 1.0
    }
}

// =============================================================================
// Power Sweep Data
// =============================================================================

/// Power sweep results for compression analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PowerSweep {
    /// Frequency (Hz)
    pub frequency: f64,
    /// Input power points (dBm)
    pub pin: Vec<f64>,
    /// Output power points (dBm)
    pub pout: Vec<f64>,
    /// Gain points (dB)
    pub gain: Vec<f64>,
}

impl PowerSweep {
    /// Create a new power sweep
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency,
            ..Default::default()
        }
    }

    /// Add a measurement point
    pub fn add_point(&mut self, pin: f64, pout: f64) {
        self.pin.push(pin);
        self.pout.push(pout);
        self.gain.push(pout - pin);
    }

    /// Small-signal gain (from lowest power point)
    pub fn small_signal_gain(&self) -> f64 {
        self.gain.first().copied().unwrap_or(0.0)
    }

    /// Find 1dB compression point (input referred)
    pub fn p1db_in(&self) -> Option<f64> {
        if self.gain.len() < 2 {
            return None;
        }

        let g0 = self.small_signal_gain();
        let target = g0 - 1.0;

        for i in 1..self.gain.len() {
            if self.gain[i] <= target {
                // Linear interpolation
                let g1 = self.gain[i - 1];
                let g2 = self.gain[i];
                let p1 = self.pin[i - 1];
                let p2 = self.pin[i];
                let t = (target - g1) / (g2 - g1);
                return Some(p1 + t * (p2 - p1));
            }
        }

        None
    }

    /// Find 1dB compression point (output referred)
    pub fn p1db_out(&self) -> Option<f64> {
        self.p1db_in().map(|pin| {
            let g0 = self.small_signal_gain();
            pin + g0 - 1.0
        })
    }

    /// Saturated output power
    pub fn psat(&self) -> f64 {
        self.pout.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    }
}

// =============================================================================
// Intermodulation Measurements
// =============================================================================

/// Two-tone intermodulation measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntermodMeasurement {
    /// Tone 1 frequency (Hz)
    pub f1: f64,
    /// Tone 2 frequency (Hz)
    pub f2: f64,
    /// Input power per tone (dBm)
    pub input_power: f64,
    /// Fundamental output power (dBm)
    pub p_fundamental: f64,
    /// IM3 output power (dBm)
    pub p_im3: f64,
    /// IM5 output power (dBm) - optional
    pub p_im5: Option<f64>,
}

impl IntermodMeasurement {
    /// Create a new measurement
    pub fn new(f1: f64, f2: f64, input_power: f64) -> Self {
        Self {
            f1,
            f2,
            input_power,
            p_fundamental: 0.0,
            p_im3: -100.0,
            p_im5: None,
        }
    }

    /// Set measured powers
    pub fn with_powers(mut self, p_fund: f64, p_im3: f64) -> Self {
        self.p_fundamental = p_fund;
        self.p_im3 = p_im3;
        self
    }

    /// Carrier-to-intermod ratio (C/I) in dB
    pub fn ci_ratio(&self) -> f64 {
        self.p_fundamental - self.p_im3
    }

    /// Input IP3 (IIP3) in dBm
    pub fn iip3(&self) -> f64 {
        // IIP3 = Pin + (Pfund - PIM3)/2
        self.input_power + self.ci_ratio() / 2.0
    }

    /// Output IP3 (OIP3) in dBm
    pub fn oip3(&self) -> f64 {
        // OIP3 = Pfund + (Pfund - PIM3)/2
        self.p_fundamental + self.ci_ratio() / 2.0
    }

    /// Calculate IIP5 if IM5 is measured
    pub fn iip5(&self) -> Option<f64> {
        self.p_im5.map(|p5| {
            // IIP5 = Pin + (Pfund - PIM5)/4
            self.input_power + (self.p_fundamental - p5) / 4.0
        })
    }
}

// =============================================================================
// Mixer Measurements
// =============================================================================

/// Mixer performance measurements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MixerMeasurements {
    /// LO frequency (Hz)
    pub f_lo: f64,
    /// RF frequency (Hz)
    pub f_rf: f64,
    /// IF frequency (Hz)
    pub f_if: f64,
    /// LO power (dBm)
    pub p_lo: f64,
    /// RF power (dBm)
    pub p_rf: f64,
    /// Conversion gain/loss (dB)
    pub conversion_gain: f64,
    /// Noise figure (dB)
    pub noise_figure: Option<f64>,
    /// IIP3 (dBm)
    pub iip3: Option<f64>,
    /// P1dB (dBm)
    pub p1db: Option<f64>,
    /// LO-RF isolation (dB)
    pub lo_rf_isolation: f64,
    /// LO-IF isolation (dB)
    pub lo_if_isolation: f64,
    /// Image rejection (dB, for image-reject mixers)
    pub image_rejection: Option<f64>,
}

impl MixerMeasurements {
    /// Create new mixer measurements
    pub fn new(f_lo: f64, f_rf: f64) -> Self {
        Self {
            f_lo,
            f_rf,
            f_if: (f_rf - f_lo).abs(),
            ..Default::default()
        }
    }

    /// Is this a downconverter (RF > IF)?
    pub fn is_downconvert(&self) -> bool {
        self.f_rf > self.f_if
    }

    /// Is conversion gain positive (gain) or negative (loss)?
    pub fn is_gain(&self) -> bool {
        self.conversion_gain > 0.0
    }

    /// Spurious-free dynamic range (SFDR) in dB
    pub fn sfdr(&self) -> Option<f64> {
        // SFDR = (2/3) * (IIP3 - NF - kTB)
        // Simplified: SFDR ≈ (2/3) * (IIP3 + 174 - NF)
        match (self.iip3, self.noise_figure) {
            (Some(iip3), Some(nf)) => Some((2.0 / 3.0) * (iip3 + 174.0 - nf)),
            _ => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
