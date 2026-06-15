use super::*;

#[derive(Debug, Clone)]
pub struct LossyTransmissionLine {
    /// Base lossless line
    pub base: TransmissionLine,

    // Loss parameters (per unit length, normalized)
    /// DC resistance (Ω)
    pub r: Value,
    /// Shunt conductance (S)
    pub g: Value,
    /// Skin effect resistance (Ω/√Hz)
    pub rs: Value,

    /// Attenuation factor
    attenuation: Value,
}

impl LossyTransmissionLine {
    /// Create a new lossy transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
        r: Value,
        g: Value,
    ) -> Self {
        let base = TransmissionLine::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);

        // Calculate attenuation: exp(-(R/2Z0 + G*Z0/2) * length)
        // For normalized line, use TD as proxy for length
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        let attenuation = (-alpha * td / 1e-9).exp().clamp(0.001, 1.0);

        Self {
            base,
            r,
            g,
            rs: 0.0,
            attenuation,
        }
    }

    /// Get attenuation factor (0-1)
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Get delayed and attenuated forward wave
    pub fn delayed_forward(&self) -> Value {
        self.base.delayed_forward() * self.attenuation
    }

    /// Get delayed and attenuated backward wave
    pub fn delayed_backward(&self) -> Value {
        self.base.delayed_backward() * self.attenuation
    }
}
