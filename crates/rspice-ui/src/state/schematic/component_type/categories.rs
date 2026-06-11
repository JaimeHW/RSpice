use super::ComponentType;

impl ComponentType {
    /// Check if this is a passive component (R, L, C).
    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            ComponentType::Resistor
                | ComponentType::Capacitor
                | ComponentType::Inductor
                | ComponentType::Transformer
                | ComponentType::CoupledInductor
        )
    }

    /// Check if this is a semiconductor device.
    pub fn is_semiconductor(&self) -> bool {
        matches!(
            self,
            ComponentType::Diode
                | ComponentType::NpnBjt
                | ComponentType::PnpBjt
                | ComponentType::Nmos
                | ComponentType::Pmos
                | ComponentType::Njfet
                | ComponentType::Pjfet
                | ComponentType::NVdmos
                | ComponentType::PVdmos
        )
    }

    /// Check if this is a source (voltage, current, or dependent).
    pub fn is_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSource
                | ComponentType::CurrentSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourcePwl
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourcePwl
                | ComponentType::CurrentSourceExp
                | ComponentType::CurrentSourceNoise
                | ComponentType::Vcvs
                | ComponentType::Vccs
                | ComponentType::Ccvs
                | ComponentType::Cccs
                | ComponentType::OpAmp
        )
    }

    /// Check if this is an XSPICE component.
    pub fn is_xspice(&self) -> bool {
        self.spice_prefix() == "A"
    }

    /// Check if this is a digital component.
    pub fn is_digital(&self) -> bool {
        matches!(
            self,
            ComponentType::XspiceInverter
                | ComponentType::XspiceBuffer
                | ComponentType::XspiceAndGate
                | ComponentType::XspiceOrGate
                | ComponentType::XspiceNandGate
                | ComponentType::XspiceNorGate
                | ComponentType::XspiceXorGate
                | ComponentType::XspiceTristate
                | ComponentType::XspiceDFlipFlop
                | ComponentType::XspiceJkFlipFlop
                | ComponentType::XspiceSrLatch
                | ComponentType::XspiceAdcBridge
                | ComponentType::XspiceDacBridge
        )
    }

    /// Check if this is a PWL (Piecewise Linear) source.
    pub fn is_pwl_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl
        )
    }
}
