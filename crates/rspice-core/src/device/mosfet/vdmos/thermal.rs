use super::*;

//=============================================================================
// Thermal Network for Self-Heating
//=============================================================================

/// Thermal network for electro-thermal simulation
///
/// Models the junction temperature rise due to power dissipation using
/// a single-pole Foster RC network:
///   dTj/dt = (P - (Tj - Ta)/Rth) / Cth
///
/// At steady-state: Tj = Ta + P * Rth
#[derive(Debug, Clone, Copy)]
pub struct ThermalNetwork {
    /// Thermal resistance junction-to-ambient (K/W)
    pub rth: Value,
    /// Thermal capacitance (J/K)
    pub cth: Value,
    /// Ambient temperature (K)
    pub t_ambient: Value,
    /// Current junction temperature (K)
    pub t_junction: Value,
    /// Previous junction temperature for transient integration
    prev_t_junction: Value,
    /// Accumulated power for averaging (W·s)
    power_integral: Value,
    /// Time of last thermal update (s)
    last_update_time: Value,
}

impl Default for ThermalNetwork {
    fn default() -> Self {
        Self {
            rth: 1.0,           // 1 K/W - typical for TO-220 with heatsink
            cth: 0.01,          // 10 mJ/K - typical thermal mass
            t_ambient: 300.15,  // 27°C in Kelvin
            t_junction: 300.15, // Start at ambient
            prev_t_junction: 300.15,
            power_integral: 0.0,
            last_update_time: 0.0,
        }
    }
}

impl ThermalNetwork {
    /// Create a new thermal network with specified parameters
    pub fn new(rth: Value, cth: Value, t_ambient: Value) -> Self {
        Self {
            rth,
            cth,
            t_ambient,
            t_junction: t_ambient,
            prev_t_junction: t_ambient,
            power_integral: 0.0,
            last_update_time: 0.0,
        }
    }

    /// Check if thermal network is enabled (Rth > 0)
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.rth > 0.0 && self.cth > 0.0
    }

    /// Get thermal voltage at current junction temperature
    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * self.t_junction / Q_ELECTRON
    }

    /// Update junction temperature based on instantaneous power
    ///
    /// Uses backward Euler integration for stability:
    ///   Tj(n+1) = (Cth*Tj(n) + dt*(P + Ta/Rth)) / (Cth + dt/Rth)
    pub fn update(&mut self, power: Value, time: Value, dt: Value) {
        if !self.is_enabled() || dt <= 0.0 {
            return;
        }

        self.prev_t_junction = self.t_junction;

        // Backward Euler for thermal RC network
        let g_th = 1.0 / self.rth;
        let denominator = self.cth + dt * g_th;
        let numerator = self.cth * self.t_junction + dt * (power + g_th * self.t_ambient);
        self.t_junction = numerator / denominator;

        // Clamp to reasonable range (prevent runaway)
        self.t_junction = self
            .t_junction
            .clamp(self.t_ambient, self.t_ambient + 200.0);

        self.last_update_time = time;
    }

    /// Get steady-state junction temperature for given power
    #[inline]
    pub fn steady_state_temperature(&self, power: Value) -> Value {
        self.t_ambient + power * self.rth
    }

    /// Reset thermal state to ambient
    pub fn reset(&mut self) {
        self.t_junction = self.t_ambient;
        self.prev_t_junction = self.t_ambient;
        self.power_integral = 0.0;
        self.last_update_time = 0.0;
    }
}
