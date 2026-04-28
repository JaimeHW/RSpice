use super::*;

// Soft-Recovery Body Diode
//=============================================================================

/// Body diode reverse recovery model
///
/// Models the stored charge in the body diode that must be removed
/// during turn-off, causing reverse recovery current.
///
/// Key parameters:
/// - Qrr: Total reverse recovery charge
/// - trr: Reverse recovery time
/// - Softness: Controls snap-off behavior (0=snappy, 1=soft)
///
/// The recovery current follows: Irr = Qrr * f(t/trr, softness)
#[derive(Debug, Clone, Copy)]
pub struct DiodeRecovery {
    /// Reverse recovery charge (C)
    pub qrr: Value,
    /// Reverse recovery time (s)
    pub trr: Value,
    /// Softness factor (0.0 = snappy, 1.0 = soft)
    pub softness: Value,
    /// Current stored charge (C)
    stored_charge: Value,
    /// Previous diode current for charge tracking
    prev_current: Value,
    /// Time when recovery started
    recovery_start_time: Value,
    /// Flag indicating active recovery
    in_recovery: bool,
}

impl Default for DiodeRecovery {
    fn default() -> Self {
        Self {
            qrr: 0.0,      // Disabled by default
            trr: 100e-9,   // 100ns typical
            softness: 0.5, // Moderate softness
            stored_charge: 0.0,
            prev_current: 0.0,
            recovery_start_time: 0.0,
            in_recovery: false,
        }
    }
}

impl DiodeRecovery {
    /// Create a new recovery model with specified parameters
    pub fn new(qrr: Value, trr: Value, softness: Value) -> Self {
        Self {
            qrr,
            trr,
            softness: softness.clamp(0.0, 1.0),
            stored_charge: 0.0,
            prev_current: 0.0,
            recovery_start_time: 0.0,
            in_recovery: false,
        }
    }

    /// Check if recovery model is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.qrr > 0.0 && self.trr > 0.0
    }

    /// Update stored charge based on diode current
    ///
    /// During forward conduction: charge builds up toward Qrr
    /// During reverse: charge depletes, causing recovery current
    pub fn update(&mut self, diode_current: Value, time: Value, dt: Value) {
        if !self.is_enabled() || dt <= 0.0 {
            self.prev_current = diode_current;
            return;
        }

        if diode_current > 0.0 {
            // Forward conduction: charge builds up with time constant ~ tt
            // Q approaches Qrr * (1 - exp(-If*dt/Qrr))
            let charge_rate = diode_current.min(self.qrr / self.trr);
            self.stored_charge += charge_rate * dt;
            self.stored_charge = self.stored_charge.min(self.qrr);
            self.in_recovery = false;
        } else if self.stored_charge > 1e-15 {
            // Reverse transition: start recovery
            if !self.in_recovery && self.prev_current > 0.0 {
                self.recovery_start_time = time;
                self.in_recovery = true;
            }

            // Deplete charge based on reverse current
            self.stored_charge += diode_current * dt; // diode_current is negative
            self.stored_charge = self.stored_charge.max(0.0);

            if self.stored_charge < 1e-15 {
                self.in_recovery = false;
            }
        }

        self.prev_current = diode_current;
    }

    /// Get recovery current contribution
    ///
    /// Returns additional current that flows during reverse recovery.
    /// Uses a softness-dependent waveform shape.
    pub fn recovery_current(&self, time: Value) -> Value {
        if !self.in_recovery || self.stored_charge <= 0.0 {
            return 0.0;
        }

        let t_rel = time - self.recovery_start_time;
        if t_rel < 0.0 || t_rel > 3.0 * self.trr {
            return 0.0;
        }

        // Recovery waveform: triangular modified by softness
        // ta = trr * softness (time of peak reverse current)
        // tb = trr * (1 - softness) (decay time)
        let ta = self.trr * (1.0 - self.softness * 0.5);
        let tb = self.trr * (1.0 + self.softness);

        let irr_peak = 2.0 * self.qrr / (ta + tb);

        if t_rel < ta {
            // Rising to peak
            -irr_peak * t_rel / ta
        } else if t_rel < ta + tb {
            // Decaying from peak
            -irr_peak * (1.0 - (t_rel - ta) / tb)
        } else {
            0.0
        }
    }

    /// Reset recovery state
    pub fn reset(&mut self) {
        self.stored_charge = 0.0;
        self.prev_current = 0.0;
        self.recovery_start_time = 0.0;
        self.in_recovery = false;
    }
}
