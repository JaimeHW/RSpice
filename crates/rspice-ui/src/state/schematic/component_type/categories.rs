//! Grouping component types into palette categories.

use super::ComponentType;

impl ComponentType {
    /// Check if this is a passive component (R, L, C).
    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            ComponentType::Resistor
                | ComponentType::Capacitor
                | ComponentType::Inductor
                | ComponentType::SaturableInductor
                | ComponentType::Transformer
                | ComponentType::CoupledInductor
                | ComponentType::TransmissionLine
                | ComponentType::LossyTransmissionLine
                | ComponentType::CoupledTransmissionLine
                | ComponentType::Memristor
        )
    }

    /// Check if this is a semiconductor device.
    pub fn is_semiconductor(&self) -> bool {
        matches!(
            self,
            ComponentType::Diode
                | ComponentType::NpnBjt
                | ComponentType::PnpBjt
                | ComponentType::NpnBjt4
                | ComponentType::PnpBjt4
                | ComponentType::NpnBjt5
                | ComponentType::PnpBjt5
                | ComponentType::Nmos
                | ComponentType::Pmos
                | ComponentType::Njfet
                | ComponentType::Pjfet
                | ComponentType::Nmesfet
                | ComponentType::Pmesfet
                | ComponentType::NVdmos
                | ComponentType::PVdmos
                | ComponentType::NmosSoi
                | ComponentType::PmosSoi
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
                | ComponentType::VoltageSourcePwlFile
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
                | ComponentType::VoltageSourceAm
                | ComponentType::VoltageSourcePat
                | ComponentType::VoltageSourceNoise
                | ComponentType::VoltageSourceRandom
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourcePwl
                | ComponentType::CurrentSourcePwlFile
                | ComponentType::CurrentSourceExp
                | ComponentType::CurrentSourceSffm
                | ComponentType::CurrentSourceAm
                | ComponentType::CurrentSourcePat
                | ComponentType::CurrentSourceNoise
                | ComponentType::CurrentSourceRandom
                | ComponentType::Vcvs
                | ComponentType::Vccs
                | ComponentType::Ccvs
                | ComponentType::Cccs
                | ComponentType::OpAmp
                | ComponentType::BehavioralSource
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

    /// Check if this source carries an inline table of PWL points.
    ///
    /// The file-backed PWL sources are deliberately absent: their breakpoints
    /// live in the referenced CSV or WAV, so they have no point table for the
    /// grid editor to edit.
    pub fn is_pwl_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl
        )
    }

    /// Check if this source reads its waveform from an external data file.
    pub fn is_pwl_file_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile
        )
    }
}
