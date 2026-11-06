//! The component palette.
//!
//! The placeable devices, grouped into the sections the palette shows, with
//! the words a reader searches each one by.

use rspice_design::schematic::component_type::ComponentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteEntry {
    pub kind: ComponentType,
    pub label: &'static str,
    /// What a reader may type for this row besides its label: synonyms, the
    /// family it belongs to, and the SPICE card letter. Matched from word
    /// starts, never as substrings.
    pub keywords: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteSection {
    pub title: &'static str,
    pub entries: &'static [ComponentPaletteEntry],
}

const fn entry(
    kind: ComponentType,
    label: &'static str,
    keywords: &'static [&'static str],
) -> ComponentPaletteEntry {
    ComponentPaletteEntry {
        kind,
        label,
        keywords,
    }
}

use ComponentType as T;

const PASSIVES: &[ComponentPaletteEntry] = &[
    entry(T::Resistor, "Resistor", &["r", "res", "resistance", "ohm"]),
    entry(
        T::Capacitor,
        "Capacitor",
        &["c", "cap", "capacitance", "condenser"],
    ),
    entry(
        T::Inductor,
        "Inductor",
        &["l", "ind", "inductance", "coil", "choke"],
    ),
    entry(
        T::Transformer,
        "Transformer",
        &["xfmr", "coupled inductors", "magnetics"],
    ),
    entry(
        T::SaturableInductor,
        "Saturable Inductor",
        &["jiles atherton", "saturation", "nonlinear inductor", "core"],
    ),
    entry(
        T::CoupledInductor,
        "K Coupling",
        &[
            "k",
            "mutual inductance",
            "coupling coefficient",
            "coupled inductors",
        ],
    ),
    entry(T::Memristor, "Memristor", &["memristive"]),
    entry(
        T::TransmissionLine,
        "Transmission Line",
        &["t", "tline", "lossless line", "delay line", "coax"],
    ),
    entry(
        T::LossyTransmissionLine,
        "Lossy Transmission Line",
        &["o", "ltra", "lossy line", "rlgc"],
    ),
    entry(
        T::CoupledTransmissionLine,
        "Coupled Transmission Line",
        &["p", "cpl", "multiconductor line", "coupled line"],
    ),
];

const SOURCES: &[ComponentPaletteEntry] = &[
    entry(
        T::VoltageSource,
        "Voltage Source",
        &[
            "v",
            "vdc",
            "dc voltage",
            "supply",
            "power supply",
            "vcc",
            "vdd",
        ],
    ),
    entry(
        T::VoltageSourceAc,
        "AC Voltage Source",
        &["vac", "ac", "small signal"],
    ),
    entry(
        T::VoltageSourcePulse,
        "Pulse Voltage Source",
        &["vpulse", "square wave", "clock", "step"],
    ),
    entry(
        T::VoltageSourceSin,
        "Sinusoidal Voltage Source",
        &["vsin", "sine", "sin"],
    ),
    entry(
        T::VoltageSourcePwl,
        "PWL Voltage Source",
        &["vpwl", "piecewise linear"],
    ),
    entry(
        T::VoltageSourcePwlFile,
        "PWL File Voltage Source",
        &["vpwl file", "piecewise linear file", "waveform file"],
    ),
    entry(
        T::VoltageSourceExp,
        "Exponential Voltage Source",
        &["vexp", "exp"],
    ),
    entry(
        T::VoltageSourceSffm,
        "SFFM Voltage Source",
        &["vsffm", "fm", "frequency modulation"],
    ),
    entry(
        T::VoltageSourceAm,
        "AM Voltage Source",
        &["vam", "amplitude modulation"],
    ),
    entry(
        T::VoltageSourcePat,
        "Pattern Voltage Source",
        &["vpat", "bit pattern", "data pattern"],
    ),
    entry(
        T::VoltageSourceNoise,
        "Noise Voltage Source",
        &["vnoise", "trnoise", "white noise"],
    ),
    entry(
        T::VoltageSourceRandom,
        "Random Voltage Source",
        &["vrandom", "trrandom"],
    ),
    entry(
        T::CurrentSource,
        "Current Source",
        &["i", "idc", "dc current", "bias current"],
    ),
    entry(T::CurrentSourceAc, "AC Current Source", &["iac", "ac"]),
    entry(
        T::CurrentSourcePulse,
        "Pulse Current Source",
        &["ipulse", "step"],
    ),
    entry(
        T::CurrentSourceSin,
        "Sinusoidal Current Source",
        &["isin", "sine", "sin"],
    ),
    entry(
        T::CurrentSourcePwl,
        "PWL Current Source",
        &["ipwl", "piecewise linear"],
    ),
    entry(
        T::CurrentSourcePwlFile,
        "PWL File Current Source",
        &["ipwl file", "piecewise linear file"],
    ),
    entry(
        T::CurrentSourceExp,
        "Exponential Current Source",
        &["iexp", "exp"],
    ),
    entry(
        T::CurrentSourceSffm,
        "SFFM Current Source",
        &["isffm", "fm", "frequency modulation"],
    ),
    entry(
        T::CurrentSourceAm,
        "AM Current Source",
        &["iam", "amplitude modulation"],
    ),
    entry(
        T::CurrentSourcePat,
        "Pattern Current Source",
        &["ipat", "bit pattern"],
    ),
    entry(
        T::CurrentSourceNoise,
        "Noise Current Source",
        &["inoise", "trnoise", "white noise"],
    ),
    entry(
        T::CurrentSourceRandom,
        "Random Current Source",
        &["irandom", "trrandom"],
    ),
    entry(
        T::BehavioralSource,
        "Behavioral Source",
        &[
            "b",
            "bsource",
            "expression",
            "arbitrary source",
            "behavioral voltage",
            "behavioral current",
        ],
    ),
    entry(
        T::RfPort,
        "RF Port",
        &["port", "s parameters", "termination", "50 ohm"],
    ),
    entry(
        T::LoopProbe,
        "Loop Probe",
        &[
            "stability",
            "loop gain",
            "phase margin",
            "middlebrook",
            "tian",
            "probe",
        ],
    ),
    entry(
        T::Ground,
        "Ground",
        &["gnd", "0", "node 0", "common", "reference"],
    ),
];

const SEMICONDUCTORS: &[ComponentPaletteEntry] = &[
    entry(
        T::Diode,
        "Diode",
        &["d", "rectifier", "pn junction", "junction diode"],
    ),
    entry(
        T::Nmos,
        "NMOS",
        &[
            "m",
            "mosfet",
            "nfet",
            "nmosfet",
            "n channel",
            "transistor",
            "fet",
            "mos",
        ],
    ),
    entry(
        T::Pmos,
        "PMOS",
        &[
            "mosfet",
            "pfet",
            "pmosfet",
            "p channel",
            "transistor",
            "fet",
            "mos",
        ],
    ),
    entry(
        T::NmosSoi,
        "NMOS SOI",
        &["soi", "bsimsoi", "mosfet", "nfet", "silicon on insulator"],
    ),
    entry(
        T::PmosSoi,
        "PMOS SOI",
        &["soi", "bsimsoi", "mosfet", "pfet", "silicon on insulator"],
    ),
    entry(
        T::NpnBjt,
        "NPN BJT",
        &["q", "bjt", "npn", "bipolar", "transistor"],
    ),
    entry(
        T::PnpBjt,
        "PNP BJT",
        &["bjt", "pnp", "bipolar", "transistor"],
    ),
    entry(
        T::NpnBjt4,
        "NPN BJT (substrate)",
        &["bjt", "npn", "bipolar", "substrate", "4 terminal"],
    ),
    entry(
        T::PnpBjt4,
        "PNP BJT (substrate)",
        &["bjt", "pnp", "bipolar", "substrate", "4 terminal"],
    ),
    entry(
        T::NpnBjt5,
        "NPN BJT (thermal)",
        &[
            "bjt",
            "npn",
            "bipolar",
            "vbic",
            "self heating",
            "5 terminal",
        ],
    ),
    entry(
        T::PnpBjt5,
        "PNP BJT (thermal)",
        &[
            "bjt",
            "pnp",
            "bipolar",
            "vbic",
            "self heating",
            "5 terminal",
        ],
    ),
    entry(
        T::Njfet,
        "N-JFET",
        &["j", "jfet", "njf", "n channel", "transistor", "fet"],
    ),
    entry(
        T::Pjfet,
        "P-JFET",
        &["jfet", "pjf", "p channel", "transistor", "fet"],
    ),
    entry(
        T::Nmesfet,
        "N-MESFET",
        &["z", "mesfet", "nmf", "gaas", "statz", "fet"],
    ),
    entry(T::Pmesfet, "P-MESFET", &["mesfet", "pmf", "gaas", "fet"]),
    entry(
        T::NVdmos,
        "N-VDMOS",
        &[
            "vdmos",
            "power mosfet",
            "mosfet",
            "nfet",
            "power fet",
            "transistor",
        ],
    ),
    entry(
        T::PVdmos,
        "P-VDMOS",
        &[
            "vdmos",
            "power mosfet",
            "mosfet",
            "pfet",
            "power fet",
            "transistor",
        ],
    ),
];

const CONTROLLED: &[ComponentPaletteEntry] = &[
    entry(
        T::OpAmp,
        "Op-Amp",
        &[
            "opamp",
            "operational amplifier",
            "amplifier",
            "amp",
            "ideal op amp",
        ],
    ),
    entry(
        T::Vcvs,
        "VCVS (E)",
        &[
            "e",
            "vcvs",
            "voltage controlled voltage source",
            "controlled source",
            "dependent source",
        ],
    ),
    entry(
        T::Vccs,
        "VCCS (G)",
        &[
            "g",
            "vccs",
            "voltage controlled current source",
            "transconductance",
            "controlled source",
            "dependent source",
        ],
    ),
    entry(
        T::Ccvs,
        "CCVS (H)",
        &[
            "h",
            "ccvs",
            "current controlled voltage source",
            "transresistance",
            "controlled source",
            "dependent source",
        ],
    ),
    entry(
        T::Cccs,
        "CCCS (F)",
        &[
            "f",
            "cccs",
            "current controlled current source",
            "current gain",
            "controlled source",
            "dependent source",
        ],
    ),
    entry(
        T::VSwitch,
        "V-Switch (S)",
        &[
            "s",
            "switch",
            "vswitch",
            "voltage controlled switch",
            "relay",
        ],
    ),
    entry(
        T::ISwitch,
        "I-Switch (W)",
        &["w", "switch", "iswitch", "csw", "current controlled switch"],
    ),
    entry(
        T::GenericSwitch,
        "Expression Switch (S)",
        &["switch", "behavioral switch"],
    ),
];

const BEHAVIORAL: &[ComponentPaletteEntry] = &[
    entry(T::XspiceGain, "Gain", &["scale", "gain block"]),
    entry(
        T::XspiceSummer,
        "Summer",
        &["sum", "adder", "summing junction"],
    ),
    entry(
        T::XspiceMultiplier,
        "Multiplier",
        &["multiply", "product", "mixer"],
    ),
    entry(
        T::XspiceDivider,
        "Divider",
        &["divide", "quotient", "ratio"],
    ),
    entry(
        T::XspiceLimiter,
        "Limiter",
        &["clamp", "clip", "saturation"],
    ),
    entry(
        T::XspiceIntegrator,
        "Integrator",
        &["integral", "integrate"],
    ),
    entry(
        T::XspiceDifferentiator,
        "Differentiator",
        &["derivative", "differentiate"],
    ),
    entry(T::XspiceBuffer, "Buffer", &["logic", "digital buffer"]),
    entry(T::XspiceInverter, "Inverter", &["not", "not gate", "logic"]),
    entry(
        T::XspiceTristate,
        "Tri-State Buffer",
        &["tristate", "three state", "logic"],
    ),
    entry(T::XspiceAndGate, "AND Gate", &["and", "logic"]),
    entry(T::XspiceOrGate, "OR Gate", &["or", "logic"]),
    entry(T::XspiceNandGate, "NAND Gate", &["nand", "logic"]),
    entry(T::XspiceNorGate, "NOR Gate", &["nor", "logic"]),
    entry(
        T::XspiceXorGate,
        "XOR Gate",
        &["xor", "exclusive or", "logic"],
    ),
    entry(
        T::XspiceDFlipFlop,
        "D Flip-Flop",
        &["dff", "flip flop", "flop", "register"],
    ),
    entry(
        T::XspiceJkFlipFlop,
        "JK Flip-Flop",
        &["jkff", "flip flop", "flop"],
    ),
    entry(T::XspiceSrLatch, "SR Latch", &["latch", "set reset"]),
    entry(
        T::XspiceAdcBridge,
        "ADC Bridge",
        &["adc", "analog to digital", "bridge"],
    ),
    entry(
        T::XspiceDacBridge,
        "DAC Bridge",
        &["dac", "digital to analog", "bridge"],
    ),
];

const HIERARCHY: &[ComponentPaletteEntry] = &[entry(
    T::Port,
    "Port",
    &["pin", "io", "interface", "terminal", "hierarchical port"],
)];

const PALETTE: &[ComponentPaletteSection] = &[
    ComponentPaletteSection {
        title: "Hierarchy",
        entries: HIERARCHY,
    },
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

/// The search words for a model-library part's device class, in the class
/// vocabulary the unified part index publishes (`diode`, `mosfet-n`, …).
///
/// A part is found by its number, and by what it is: `transistor` finds every
/// BJT and MOSFET card whatever its number says.
pub fn device_class_keywords(class: &str) -> &'static [&'static str] {
    match class {
        "diode" => &["diode", "rectifier"],
        "bjt-npn" => &["bjt", "npn", "bipolar", "transistor"],
        "bjt-pnp" => &["bjt", "pnp", "bipolar", "transistor"],
        "jfet-n" => &["jfet", "njf", "transistor", "fet"],
        "jfet-p" => &["jfet", "pjf", "transistor", "fet"],
        "mosfet-n" => &["mosfet", "nmos", "nfet", "transistor", "fet"],
        "mosfet-p" => &["mosfet", "pmos", "pfet", "transistor", "fet"],
        "mosfet-vdmos" => &["mosfet", "vdmos", "power mosfet", "transistor", "fet"],
        "mesfet-n" | "mesfet-p" => &["mesfet", "gaas", "fet"],
        "switch" => &["switch", "voltage controlled switch"],
        "switch-current" => &["switch", "current controlled switch"],
        "resistor" => &["resistor"],
        "capacitor" => &["capacitor"],
        "inductor" => &["inductor"],
        "coupling" => &["coupling", "mutual inductance"],
        "magnetic-core" => &["core", "magnetic core"],
        "transmission-line" => &["transmission line", "tline"],
        "distributed-rc" => &["distributed rc", "rc line", "urc"],
        "piezo" => &["piezo", "crystal", "resonator"],
        "opamp" => &["opamp", "operational amplifier", "amplifier"],
        "subckt" => &["subcircuit", "subckt", "macromodel"],
        "digital" | "digital-behavioral" => &["digital", "logic"],
        _ => &[],
    }
}
