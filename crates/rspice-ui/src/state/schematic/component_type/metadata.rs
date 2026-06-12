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
            ComponentType::NpnBjt | ComponentType::PnpBjt => "Q",
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => "M",
            ComponentType::Njfet | ComponentType::Pjfet => "J",
            ComponentType::SaturableInductor => "L",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => "V",
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => "I",
            ComponentType::Vcvs | ComponentType::OpAmp => "E",
            ComponentType::Vccs => "G",
            ComponentType::Ccvs => "H",
            ComponentType::Cccs => "F",
            ComponentType::BehavioralSource => "B",
            ComponentType::VSwitch => "S",
            ComponentType::TransmissionLine => "T",
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
            ComponentType::Nmos => "NMOS",
            ComponentType::Pmos => "PMOS",
            ComponentType::Njfet => "N-JFET",
            ComponentType::Pjfet => "P-JFET",
            ComponentType::NVdmos => "N-VDMOS",
            ComponentType::PVdmos => "P-VDMOS",
            ComponentType::SaturableInductor => "Saturable L",
            ComponentType::VoltageSource => "V DC",
            ComponentType::CurrentSource => "I DC",
            ComponentType::VoltageSourceAc => "V AC",
            ComponentType::VoltageSourcePulse => "V Pulse",
            ComponentType::VoltageSourceSin => "V Sin",
            ComponentType::VoltageSourcePwl => "V PWL",
            ComponentType::VoltageSourceExp => "V Exp",
            ComponentType::VoltageSourceSffm => "V SFFM",
            ComponentType::CurrentSourceAc => "I AC",
            ComponentType::CurrentSourcePulse => "I Pulse",
            ComponentType::CurrentSourceSin => "I Sin",
            ComponentType::CurrentSourcePwl => "I PWL",
            ComponentType::CurrentSourceExp => "I Exp",
            ComponentType::CurrentSourceNoise => "I Noise",
            ComponentType::Vcvs => "VCVS (E)",
            ComponentType::OpAmp => "Op-Amp",
            ComponentType::Vccs => "VCCS (G)",
            ComponentType::Ccvs => "CCVS (H)",
            ComponentType::Cccs => "CCCS (F)",
            ComponentType::BehavioralSource => "Behavioral Source",
            ComponentType::VSwitch => "V-Switch (S)",
            ComponentType::TransmissionLine => "Transmission Line",
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
            ComponentType::Transformer => "1m",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "5",
            ComponentType::CurrentSource => "1m",
            // Open-loop gain of the ideal stage.
            ComponentType::OpAmp => "100k",
            // The whole V=<expr> / I=<expr> expression is the value.
            ComponentType::BehavioralSource => "V=0",
            ComponentType::CellInstance => "",
            _ => "",
        }
    }
}
