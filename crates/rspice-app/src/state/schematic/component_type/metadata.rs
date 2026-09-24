//! Per-component-type metadata: display name, reference-designator prefix,
//! and SPICE card letter.

use super::ComponentType;

impl ComponentType {
    /// Get the SPICE prefix for this component type.
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "R",
            ComponentType::Capacitor => "C",
            ComponentType::Inductor => "L",
            ComponentType::Transformer => "T",
            ComponentType::CoupledInductor => "K",
            ComponentType::Diode => "D",
            ComponentType::NpnBjt
            | ComponentType::PnpBjt
            | ComponentType::NpnBjt4
            | ComponentType::PnpBjt4
            | ComponentType::NpnBjt5
            | ComponentType::PnpBjt5 => "Q",
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos
            | ComponentType::NmosSoi
            | ComponentType::PmosSoi => "M",
            ComponentType::Njfet | ComponentType::Pjfet => "J",
            ComponentType::Nmesfet | ComponentType::Pmesfet => "Z",
            ComponentType::SaturableInductor => "L",
            // The Xyce element keyword is YMEMRISTOR; the designator uses a
            // distinct MR prefix and the emitter writes the full form.
            ComponentType::Memristor => "MR",
            ComponentType::VoltageSource
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
            | ComponentType::VoltageSourceRandom => "V",
            ComponentType::CurrentSource
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
            | ComponentType::CurrentSourceRandom => "I",
            ComponentType::Vcvs | ComponentType::OpAmp => "E",
            ComponentType::Vccs => "G",
            ComponentType::Ccvs => "H",
            ComponentType::Cccs => "F",
            ComponentType::BehavioralSource => "B",
            ComponentType::VSwitch => "S",
            ComponentType::ISwitch => "W",
            ComponentType::GenericSwitch => "S",
            ComponentType::TransmissionLine => "T",
            ComponentType::LossyTransmissionLine => "O",
            ComponentType::CoupledTransmissionLine => "P",
            ComponentType::RfPort => "P",
            // The probe is a 0 V voltage source and nothing else: the engine
            // resolves `.STB probe=` against the circuit's voltage-source
            // names, so the probe has to be named in that namespace or it
            // could not be referred to at all.
            ComponentType::LoopProbe => "V",
            ComponentType::Ground => "",
            ComponentType::Port => "",
            ComponentType::CellInstance => "X",
            ComponentType::XspiceGain
            | ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator
            | ComponentType::XspiceInverter
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
            | ComponentType::XspiceDacBridge => "A",
        }
    }

    /// Validate a user-authored SPICE instance designator for this type.
    ///
    /// Ground and interface ports are structural objects and intentionally do
    /// not own emitted instance names. Every emitted element uses its device
    /// prefix followed by a non-empty ASCII identifier suffix.
    pub fn validate_reference_designator(&self, value: &str) -> Result<(), String> {
        let prefix = self.spice_prefix();
        if prefix.is_empty() {
            return Err(
                "This component type does not own a SPICE reference designator.".to_owned(),
            );
        }
        if !value
            .get(..prefix.len())
            .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        {
            return Err(format!(
                "{} designators must begin with `{prefix}`.",
                self.display_name()
            ));
        }
        let suffix = &value[prefix.len()..];
        if suffix.is_empty()
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(format!(
                "Enter `{prefix}` followed by one or more ASCII letters, digits, or underscores."
            ));
        }
        Ok(())
    }

    /// Get the display name for this component type.
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "Resistor",
            ComponentType::Capacitor => "Capacitor",
            ComponentType::Inductor => "Inductor",
            ComponentType::Transformer => "Transformer",
            ComponentType::CoupledInductor => "Coupled Inductor",
            ComponentType::Diode => "Diode",
            ComponentType::NpnBjt => "NPN BJT",
            ComponentType::PnpBjt => "PNP BJT",
            ComponentType::NpnBjt4 => "NPN BJT 4T",
            ComponentType::PnpBjt4 => "PNP BJT 4T",
            ComponentType::NpnBjt5 => "NPN BJT 5T",
            ComponentType::PnpBjt5 => "PNP BJT 5T",
            ComponentType::Nmos => "NMOS",
            ComponentType::Pmos => "PMOS",
            ComponentType::Njfet => "N-JFET",
            ComponentType::Pjfet => "P-JFET",
            ComponentType::Nmesfet => "N-MESFET",
            ComponentType::Pmesfet => "P-MESFET",
            ComponentType::NVdmos => "N-VDMOS",
            ComponentType::PVdmos => "P-VDMOS",
            ComponentType::NmosSoi => "NMOS SOI",
            ComponentType::PmosSoi => "PMOS SOI",
            ComponentType::SaturableInductor => "Saturable L",
            ComponentType::Memristor => "Memristor",
            ComponentType::VoltageSource => "V DC",
            ComponentType::CurrentSource => "I DC",
            ComponentType::VoltageSourceAc => "V AC",
            ComponentType::VoltageSourcePulse => "V Pulse",
            ComponentType::VoltageSourceSin => "V Sin",
            ComponentType::VoltageSourcePwl => "V PWL",
            ComponentType::VoltageSourcePwlFile => "V PWL File",
            ComponentType::VoltageSourceExp => "V Exp",
            ComponentType::VoltageSourceSffm => "V SFFM",
            ComponentType::VoltageSourceAm => "V AM",
            ComponentType::VoltageSourcePat => "V Pattern",
            ComponentType::VoltageSourceNoise => "V Noise",
            ComponentType::VoltageSourceRandom => "V Random",
            ComponentType::CurrentSourceAc => "I AC",
            ComponentType::CurrentSourcePulse => "I Pulse",
            ComponentType::CurrentSourceSin => "I Sin",
            ComponentType::CurrentSourcePwl => "I PWL",
            ComponentType::CurrentSourcePwlFile => "I PWL File",
            ComponentType::CurrentSourceExp => "I Exp",
            ComponentType::CurrentSourceSffm => "I SFFM",
            ComponentType::CurrentSourceAm => "I AM",
            ComponentType::CurrentSourcePat => "I Pattern",
            ComponentType::CurrentSourceNoise => "I Noise",
            ComponentType::CurrentSourceRandom => "I Random",
            ComponentType::Vcvs => "VCVS (E)",
            ComponentType::OpAmp => "Op-Amp",
            ComponentType::Vccs => "VCCS (G)",
            ComponentType::Ccvs => "CCVS (H)",
            ComponentType::Cccs => "CCCS (F)",
            ComponentType::BehavioralSource => "Behavioral Source",
            ComponentType::VSwitch => "V-Switch (S)",
            ComponentType::ISwitch => "I-Switch (W)",
            ComponentType::GenericSwitch => "Expression Switch (S)",
            ComponentType::TransmissionLine => "Transmission Line",
            ComponentType::LossyTransmissionLine => "Lossy T-Line",
            ComponentType::CoupledTransmissionLine => "Coupled T-Line",
            ComponentType::RfPort => "RF Port",
            ComponentType::LoopProbe => "Loop Probe",
            ComponentType::Ground => "Ground",
            ComponentType::Port => "Port",
            ComponentType::CellInstance => "Cell Instance",
            ComponentType::XspiceGain => "Gain",
            ComponentType::XspiceSummer => "Summer",
            ComponentType::XspiceMultiplier => "Multiplier",
            ComponentType::XspiceDivider => "Divider",
            ComponentType::XspiceLimiter => "Limiter",
            ComponentType::XspiceIntegrator => "Integrator",
            ComponentType::XspiceDifferentiator => "Differentiator",
            ComponentType::XspiceInverter => "Inverter",
            ComponentType::XspiceBuffer => "Buffer",
            ComponentType::XspiceAndGate => "AND Gate",
            ComponentType::XspiceOrGate => "OR Gate",
            ComponentType::XspiceNandGate => "NAND Gate",
            ComponentType::XspiceNorGate => "NOR Gate",
            ComponentType::XspiceXorGate => "XOR Gate",
            ComponentType::XspiceTristate => "Tri-State",
            ComponentType::XspiceDFlipFlop => "D Flip-Flop",
            ComponentType::XspiceJkFlipFlop => "JK Flip-Flop",
            ComponentType::XspiceSrLatch => "SR Latch",
            ComponentType::XspiceAdcBridge => "ADC Bridge",
            ComponentType::XspiceDacBridge => "DAC Bridge",
        }
    }

    /// Get default value for this component type.
    pub fn default_value(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "1k",
            ComponentType::Capacitor => "1u",
            ComponentType::Inductor => "1m",
            // The Jiles-Atherton path requires a positive inductance target.
            ComponentType::SaturableInductor => "1m",
            ComponentType::Transformer => "1m",
            // K element coefficient; the winding names come from properties.
            ComponentType::CoupledInductor => "0.99",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "5",
            ComponentType::CurrentSource => "1m",
            // Open-loop gain of the ideal stage.
            ComponentType::OpAmp => "100k",
            // The whole V=<expr> / I=<expr> expression is the value.
            ComponentType::BehavioralSource => "V=0",
            ComponentType::GenericSwitch => "0",
            ComponentType::CellInstance => "",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_designators_use_the_device_prefix_and_portable_suffix() {
        assert!(
            ComponentType::Resistor
                .validate_reference_designator("R17")
                .is_ok()
        );
        assert!(
            ComponentType::Nmos
                .validate_reference_designator("m_input_2")
                .is_ok()
        );
        assert!(
            ComponentType::CellInstance
                .validate_reference_designator("XAFE")
                .is_ok()
        );

        assert!(
            ComponentType::Resistor
                .validate_reference_designator("C17")
                .is_err()
        );
        assert!(
            ComponentType::Resistor
                .validate_reference_designator("R")
                .is_err()
        );
        assert!(
            ComponentType::Resistor
                .validate_reference_designator("R 17")
                .is_err()
        );
        assert!(
            ComponentType::Ground
                .validate_reference_designator("GND")
                .is_err()
        );
    }
}
