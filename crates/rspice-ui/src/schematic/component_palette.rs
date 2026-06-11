use crate::state::ComponentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteEntry {
    pub kind: ComponentType,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteSection {
    pub title: &'static str,
    pub entries: &'static [ComponentPaletteEntry],
}

const PASSIVES: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::Resistor,
        label: "Resistor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Capacitor,
        label: "Capacitor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Inductor,
        label: "Inductor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Transformer,
        label: "Transformer",
    },
    ComponentPaletteEntry {
        kind: ComponentType::SaturableInductor,
        label: "Saturable Inductor",
    },
    ComponentPaletteEntry {
        kind: ComponentType::TransmissionLine,
        label: "Transmission Line",
    },
];

const SOURCES: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSource,
        label: "Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceAc,
        label: "AC Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourcePulse,
        label: "Pulse Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceSin,
        label: "Sinusoidal Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourcePwl,
        label: "PWL Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceExp,
        label: "Exponential Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VoltageSourceSffm,
        label: "SFFM Voltage Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSource,
        label: "Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceAc,
        label: "AC Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourcePulse,
        label: "Pulse Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceSin,
        label: "Sinusoidal Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourcePwl,
        label: "PWL Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceExp,
        label: "Exponential Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::CurrentSourceNoise,
        label: "Noise Current Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::BehavioralSource,
        label: "Behavioral Source",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Ground,
        label: "Ground",
    },
];

const SEMICONDUCTORS: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::Diode,
        label: "Diode",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Nmos,
        label: "NMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Pmos,
        label: "PMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::NpnBjt,
        label: "NPN BJT",
    },
    ComponentPaletteEntry {
        kind: ComponentType::PnpBjt,
        label: "PNP BJT",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Njfet,
        label: "N-JFET",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Pjfet,
        label: "P-JFET",
    },
    ComponentPaletteEntry {
        kind: ComponentType::NVdmos,
        label: "N-VDMOS",
    },
    ComponentPaletteEntry {
        kind: ComponentType::PVdmos,
        label: "P-VDMOS",
    },
];

const CONTROLLED: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::OpAmp,
        label: "Op-Amp",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Vcvs,
        label: "VCVS (E)",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Vccs,
        label: "VCCS (G)",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Ccvs,
        label: "CCVS (H)",
    },
    ComponentPaletteEntry {
        kind: ComponentType::Cccs,
        label: "CCCS (F)",
    },
    ComponentPaletteEntry {
        kind: ComponentType::VSwitch,
        label: "V-Switch (S)",
    },
];

const BEHAVIORAL: &[ComponentPaletteEntry] = &[
    ComponentPaletteEntry {
        kind: ComponentType::XspiceGain,
        label: "Gain",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceSummer,
        label: "Summer",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceMultiplier,
        label: "Multiplier",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceDivider,
        label: "Divider",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceLimiter,
        label: "Limiter",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceIntegrator,
        label: "Integrator",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceDifferentiator,
        label: "Differentiator",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceBuffer,
        label: "Buffer",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceInverter,
        label: "Inverter",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceTristate,
        label: "Tri-State Buffer",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceAndGate,
        label: "AND Gate",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceOrGate,
        label: "OR Gate",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceNandGate,
        label: "NAND Gate",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceNorGate,
        label: "NOR Gate",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceXorGate,
        label: "XOR Gate",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceDFlipFlop,
        label: "D Flip-Flop",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceJkFlipFlop,
        label: "JK Flip-Flop",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceSrLatch,
        label: "SR Latch",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceAdcBridge,
        label: "ADC Bridge",
    },
    ComponentPaletteEntry {
        kind: ComponentType::XspiceDacBridge,
        label: "DAC Bridge",
    },
];

const PALETTE: &[ComponentPaletteSection] = &[
    ComponentPaletteSection {
        title: "Passives",
        entries: PASSIVES,
    },
    ComponentPaletteSection {
        title: "Sources",
        entries: SOURCES,
    },
    ComponentPaletteSection {
        title: "Semiconductors",
        entries: SEMICONDUCTORS,
    },
    ComponentPaletteSection {
        title: "Controlled sources",
        entries: CONTROLLED,
    },
    ComponentPaletteSection {
        title: "Behavioral (XSPICE)",
        entries: BEHAVIORAL,
    },
];

pub fn component_palette() -> &'static [ComponentPaletteSection] {
    PALETTE
}
