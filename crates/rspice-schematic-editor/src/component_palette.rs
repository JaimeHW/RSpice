//! The component palette: the one taxonomy of built-in placeable parts.
//!
//! Every native primitive is filed here under its functional group, with the
//! words a reader searches it by. Code models carry their own group on their
//! catalog descriptor; [`palette_rows`] merges the two, so the shelf, the
//! instance catalog and the symbol audit all read one list. The supply rails
//! are filed with the connectivity objects: each places a global net label
//! drawn with its rail symbol.

pub use rspice_design::schematic::component_category::ComponentCategory;
use rspice_design::schematic::component_type::ComponentType;
use rspice_design::schematic::device_catalog::{
    CatalogXspiceDeviceDescriptor, engine_only_xspice_devices,
};
use rspice_design::schematic::net_label::{NetLabelKind, SupplyGlyph};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteEntry {
    pub kind: ComponentType,
    pub label: &'static str,
    /// What a reader may type for this row besides its label: synonyms, the
    /// family it belongs to, and the SPICE card letter. Matched from word
    /// starts, never as substrings.
    pub keywords: &'static [&'static str],
    /// The symbol skin the row places, as the type's Symbol property names
    /// it; `None` for the type's own artwork.
    pub variant: Option<&'static str>,
    /// The model card the row places in place of the type's own default.
    pub default_model: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentPaletteSection {
    pub category: ComponentCategory,
    pub entries: &'static [ComponentPaletteEntry],
}

/// A supply rail the shelf places: a global net label named for the rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupplyPaletteEntry {
    /// The rail's net name, which is also the row's label.
    pub name: &'static str,
    /// What a reader may type for this row besides its name.
    pub keywords: &'static [&'static str],
}

impl SupplyPaletteEntry {
    /// The rail's conventional symbol.
    pub fn glyph(&self) -> SupplyGlyph {
        SupplyGlyph::for_name(self.name)
    }

    /// The label kind the row places.
    pub fn kind(&self) -> NetLabelKind {
        NetLabelKind::Supply {
            glyph: self.glyph(),
        }
    }
}

/// One row of a functional group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteRow {
    Primitive(&'static ComponentPaletteEntry),
    Xspice(&'static CatalogXspiceDeviceDescriptor),
    Supply(&'static SupplyPaletteEntry),
}

impl PaletteRow {
    pub fn label(self) -> &'static str {
        match self {
            Self::Primitive(entry) => entry.label,
            Self::Xspice(descriptor) => descriptor.display_name,
            Self::Supply(entry) => entry.name,
        }
    }
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
        variant: None,
        default_model: None,
    }
}

/// A row that places `kind` with a symbol skin, and a model card when the
/// skin names a different device.
const fn variant(
    kind: ComponentType,
    label: &'static str,
    variant: &'static str,
    default_model: Option<&'static str>,
    keywords: &'static [&'static str],
) -> ComponentPaletteEntry {
    ComponentPaletteEntry {
        kind,
        label,
        keywords,
        variant: Some(variant),
        default_model,
    }
}

use ComponentType as T;

const HIERARCHY: &[ComponentPaletteEntry] = &[
    entry(
        T::Port,
        "Port",
        &["pin", "io", "interface", "terminal", "hierarchical port"],
    ),
    entry(
        T::Ground,
        "Ground",
        &["gnd", "0", "node 0", "common", "reference"],
    ),
    variant(
        T::Ground,
        "Earth Ground",
        "earth",
        None,
        &["earth", "protective earth", "pe"],
    ),
    variant(
        T::Ground,
        "Chassis Ground",
        "chassis",
        None,
        &["chassis", "frame", "enclosure"],
    ),
];

/// The rails every analog and mixed-signal design draws: two positive, two
/// negative or return.
const SUPPLIES: &[SupplyPaletteEntry] = &[
    SupplyPaletteEntry {
        name: "VDD",
        keywords: &["vdd", "supply", "rail", "power", "global", "positive"],
    },
    SupplyPaletteEntry {
        name: "VCC",
        keywords: &["vcc", "supply", "rail", "power", "global", "positive"],
    },
    SupplyPaletteEntry {
        name: "VSS",
        keywords: &["vss", "supply", "rail", "power", "global", "negative", "return"],
    },
    SupplyPaletteEntry {
        name: "VEE",
        keywords: &["vee", "supply", "rail", "power", "global", "negative"],
    },
];

const PASSIVES: &[ComponentPaletteEntry] = &[
    entry(T::Resistor, "Resistor", &["r", "res", "resistance", "ohm"]),
    entry(
        T::Capacitor,
        "Capacitor",
        &["c", "cap", "capacitance", "condenser"],
    ),
    variant(
        T::Capacitor,
        "Polarized Capacitor",
        "polarized",
        None,
        &["electrolytic", "polarised", "tantalum", "cap"],
    ),
    entry(
        T::Inductor,
        "Inductor",
        &["l", "ind", "inductance", "coil", "choke"],
    ),
    entry(T::Memristor, "Memristor", &["memristive"]),
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
    variant(
        T::VoltageSource,
        "Battery",
        "battery",
        None,
        &["cell", "dc source"],
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
];

const SEMICONDUCTORS: &[ComponentPaletteEntry] = &[
    entry(
        T::Diode,
        "Diode",
        &["d", "rectifier", "pn junction", "junction diode"],
    ),
    variant(
        T::Diode,
        "Zener Diode",
        "zener",
        Some("RSPICE_ZENER"),
        &["zener", "avalanche", "voltage reference", "regulator"],
    ),
    variant(
        T::Diode,
        "Schottky Diode",
        "schottky",
        Some("RSPICE_SCHOTTKY"),
        &["schottky", "barrier diode", "rectifier"],
    ),
    variant(
        T::Diode,
        "LED (red)",
        "led",
        Some("RSPICE_LED_RED"),
        &["led", "light emitting diode", "indicator", "red"],
    ),
    variant(
        T::Diode,
        "LED (green)",
        "led",
        Some("RSPICE_LED_GREEN"),
        &["led", "light emitting diode", "indicator", "green"],
    ),
    variant(
        T::Diode,
        "LED (blue)",
        "led",
        Some("RSPICE_LED_BLUE"),
        &["led", "light emitting diode", "indicator", "blue"],
    ),
    variant(
        T::Diode,
        "LED (white)",
        "led",
        Some("RSPICE_LED_WHITE"),
        &["led", "light emitting diode", "indicator", "white"],
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

const AMPLIFIERS: &[ComponentPaletteEntry] = &[entry(
    T::OpAmp,
    "Ideal Op-Amp",
    &[
        "opamp",
        "operational amplifier",
        "amplifier",
        "amp",
        "ideal op amp",
    ],
)];

const CONTROLLED: &[ComponentPaletteEntry] = &[
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

const DIGITAL: &[ComponentPaletteEntry] = &[
    entry(
        T::XspiceBuffer,
        "Buffer (event-driven)",
        &["logic", "digital buffer"],
    ),
    entry(
        T::XspiceInverter,
        "Inverter (event-driven)",
        &["not", "not gate", "logic"],
    ),
    entry(
        T::XspiceTristate,
        "Tri-State Buffer (event-driven)",
        &["tristate", "three state", "logic"],
    ),
    entry(
        T::XspiceAndGate,
        "AND Gate (event-driven)",
        &["and", "logic"],
    ),
    entry(T::XspiceOrGate, "OR Gate (event-driven)", &["or", "logic"]),
    entry(
        T::XspiceNandGate,
        "NAND Gate (event-driven)",
        &["nand", "logic"],
    ),
    entry(
        T::XspiceNorGate,
        "NOR Gate (event-driven)",
        &["nor", "logic"],
    ),
    entry(
        T::XspiceXorGate,
        "XOR Gate (event-driven)",
        &["xor", "exclusive or", "logic"],
    ),
    entry(
        T::XspiceDFlipFlop,
        "D Flip-Flop (event-driven)",
        &["dff", "flip flop", "flop", "register"],
    ),
    entry(
        T::XspiceJkFlipFlop,
        "JK Flip-Flop (event-driven)",
        &["jkff", "flip flop", "flop"],
    ),
    entry(
        T::XspiceSrLatch,
        "SR Latch (event-driven)",
        &["latch", "set reset"],
    ),
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

const BEHAVIORAL: &[ComponentPaletteEntry] = &[
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
];

const MAGNETICS: &[ComponentPaletteEntry] = &[
    entry(
        T::Transformer,
        "Transformer",
        &["xfmr", "coupled inductors", "magnetics"],
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
    entry(
        T::SaturableInductor,
        "Saturable Inductor",
        &["jiles atherton", "saturation", "nonlinear inductor", "core"],
    ),
];

const LINES: &[ComponentPaletteEntry] = &[
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
    entry(
        T::RfPort,
        "RF Port",
        &["port", "s parameters", "termination", "50 ohm"],
    ),
];

const PROBES: &[ComponentPaletteEntry] = &[entry(
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
)];

const PALETTE: &[ComponentPaletteSection] = &[
    ComponentPaletteSection {
        category: ComponentCategory::HierarchyConnectivity,
        entries: HIERARCHY,
    },
    ComponentPaletteSection {
        category: ComponentCategory::Passives,
        entries: PASSIVES,
    },
    ComponentPaletteSection {
        category: ComponentCategory::Sources,
        entries: SOURCES,
    },
    ComponentPaletteSection {
        category: ComponentCategory::Semiconductors,
        entries: SEMICONDUCTORS,
    },
    ComponentPaletteSection {
        category: ComponentCategory::AmplifiersComparators,
        entries: AMPLIFIERS,
    },
    ComponentPaletteSection {
        category: ComponentCategory::ControlledSourcesSwitches,
        entries: CONTROLLED,
    },
    ComponentPaletteSection {
        category: ComponentCategory::DigitalEventDriven,
        entries: DIGITAL,
    },
    ComponentPaletteSection {
        category: ComponentCategory::BehavioralMath,
        entries: BEHAVIORAL,
    },
    ComponentPaletteSection {
        category: ComponentCategory::Magnetics,
        entries: MAGNETICS,
    },
    ComponentPaletteSection {
        category: ComponentCategory::LinesRf,
        entries: LINES,
    },
    ComponentPaletteSection {
        category: ComponentCategory::ProbesMeters,
        entries: PROBES,
    },
];

pub fn component_palette() -> &'static [ComponentPaletteSection] {
    PALETTE
}

/// The supply rails, in shelf order.
pub fn supply_palette() -> &'static [SupplyPaletteEntry] {
    SUPPLIES
}

/// Every row filed under `category`: its native primitives in palette order,
/// then its code models by name, then (for the connectivity group) the supply
/// rails. A code model that duplicates a native part carries no group and is
/// never listed; it stays valid in netlists.
pub fn palette_rows(category: ComponentCategory) -> Vec<PaletteRow> {
    let mut rows = PALETTE
        .iter()
        .filter(|section| section.category == category)
        .flat_map(|section| section.entries.iter().map(PaletteRow::Primitive))
        .collect::<Vec<_>>();
    let mut models = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| descriptor.category == Some(category))
        .collect::<Vec<_>>();
    models.sort_by_key(|descriptor| descriptor.display_name);
    rows.extend(models.into_iter().map(PaletteRow::Xspice));
    if category == ComponentCategory::HierarchyConnectivity {
        rows.extend(SUPPLIES.iter().map(PaletteRow::Supply));
    }
    rows
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
