//! The passive, protection, electromechanical and power-semiconductor
//! functional parts. Each default describes a sensible generic part of its
//! kind; none is any manufacturer's part.

use rspice_design_model::symbol_pin::SymbolPinSide;

use super::{
    ComponentCategory, FunctionalBlock, FunctionalParameter, FunctionalPartDescriptor,
    FunctionalPin, FunctionalPreset, FunctionalPresetChoice, In, InOut, Out, choice, number, pin,
    selector,
};

/// Two terminals on the block's centre line, one each side.
const TWO_TERMINAL_BLOCK: FunctionalBlock = FunctionalBlock {
    height: 20,
    pins: &[(SymbolPinSide::Left, -30, 0), (SymbolPinSide::Right, 30, 0)],
};

const TWO_TERMINAL_PINS: &[FunctionalPin] = &[pin("A", InOut), pin("B", InOut)];

/// A 10 MHz AT-cut fundamental crystal.
const CRYSTAL_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "FS",
        "Series resonance",
        "Hz",
        "10Meg",
        "Frequency at which the motional arm resonates in series; for an overtone crystal, the marked overtone's.",
    ),
    number(
        "ESR",
        "Series resistance",
        "Ω",
        "25",
        "Motional resistance: the crystal's impedance at series resonance.",
    ),
    number(
        "C0",
        "Shunt capacitance",
        "F",
        "4p",
        "Electrode and holder capacitance across the terminals.",
    ),
    number(
        "C1",
        "Motional capacitance",
        "F",
        "15f",
        "Capacitance of the motional arm; the motional inductance resonates with it at the series resonance.",
    ),
    number(
        "Q",
        "Quality factor",
        "",
        "0",
        "When positive, sets the motional capacitance to 1/(2π·FS·Q·ESR) in place of C1.",
    ),
    number(
        "OVERTONE",
        "Overtone",
        "",
        "1",
        "Mode the crystal is marked at: 1 for a fundamental crystal, 3 or 5 for an overtone one, which also resonates at its fundamental, FS/OVERTONE.",
    ),
];

const CRYSTAL_PRESET: FunctionalPreset = FunctionalPreset {
    name: "CUT",
    label: "Crystal",
    description: "Fills the parameters below with a typical crystal of that kind; each stays editable.",
    choices: &[
        FunctionalPresetChoice {
            label: "10 MHz AT-cut",
            values: &[
                ("FS", "10Meg"),
                ("ESR", "25"),
                ("C0", "4p"),
                ("C1", "15f"),
                ("Q", "0"),
                ("OVERTONE", "1"),
            ],
        },
        FunctionalPresetChoice {
            label: "32.768 kHz watch",
            values: &[
                ("FS", "32.768k"),
                ("ESR", "35k"),
                ("C0", "1.3p"),
                ("C1", "3f"),
                ("Q", "0"),
                ("OVERTONE", "1"),
            ],
        },
        FunctionalPresetChoice {
            label: "30 MHz third overtone",
            values: &[
                ("FS", "30Meg"),
                ("ESR", "40"),
                ("C0", "4p"),
                ("C1", "1.6f"),
                ("Q", "0"),
                ("OVERTONE", "3"),
            ],
        },
    ],
};

pub(super) const CRYSTAL: FunctionalPartDescriptor = FunctionalPartDescriptor {
    stable_id: "rspice.functional.crystal",
    display_name: "Crystal",
    subcircuit: "RSPICE_FN_CRYSTAL",
    pins: TWO_TERMINAL_PINS,
    symbol_asset: "functional_crystal.svg",
    block: Some(TWO_TERMINAL_BLOCK),
    keywords: &[
        "crystal",
        "xtal",
        "quartz",
        "resonator",
        "quartz crystal",
        "oscillator crystal",
    ],
    category: ComponentCategory::Passives,
    parameters: CRYSTAL_PARAMETERS,
    preset: Some(CRYSTAL_PRESET),
    inputs: None,
};

/// A 275 VAC, 14 mm metal-oxide disc.
const VARISTOR_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "VBREAK",
        "Breakdown voltage",
        "V",
        "430",
        "Voltage across the varistor, either way, beyond which it conducts.",
    ),
    number(
        "RCLAMP",
        "Clamp resistance",
        "Ω",
        "5.6",
        "Resistance the voltage beyond breakdown drives current through: the clamp voltage at current I is VBREAK + I·RCLAMP.",
    ),
    number(
        "CP",
        "Capacitance",
        "F",
        "450p",
        "Capacitance of the disc across the terminals.",
    ),
];

pub(super) const VARISTOR: FunctionalPartDescriptor = FunctionalPartDescriptor {
    stable_id: "rspice.functional.varistor",
    display_name: "Varistor",
    subcircuit: "RSPICE_FN_VARISTOR",
    pins: TWO_TERMINAL_PINS,
    symbol_asset: "functional_varistor.svg",
    block: Some(TWO_TERMINAL_BLOCK),
    keywords: &[
        "varistor",
        "mov",
        "metal oxide varistor",
        "surge",
        "surge protection",
        "transient suppressor",
        "vdr",
    ],
    category: ComponentCategory::ProtectionElectromechanical,
    parameters: VARISTOR_PARAMETERS,
    preset: None,
    inputs: None,
};

/// A 1 A fast-acting fuse.
const FUSE_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "IRATED",
        "Rated current",
        "A",
        "1",
        "Current the fuse carries indefinitely: the element settles just short of melting.",
    ),
    number(
        "I2T",
        "Melting I²t",
        "A²s",
        "0.5",
        "Heat at which the element melts; an overload I melts it after about I2T/I², which it approaches as the overload grows.",
    ),
    number(
        "RCOLD",
        "Cold resistance",
        "Ω",
        "0.1",
        "Resistance of the intact element.",
    ),
    number(
        "ROPEN",
        "Open resistance",
        "Ω",
        "1G",
        "Resistance once blown; the fuse never closes again.",
    ),
    number(
        "TARC",
        "Arcing time",
        "s",
        "100u",
        "Time the element takes to go from intact to open once it melts.",
    ),
];

pub(super) const FUSE: FunctionalPartDescriptor = FunctionalPartDescriptor {
    stable_id: "rspice.functional.fuse",
    display_name: "Fuse",
    subcircuit: "RSPICE_FN_FUSE",
    pins: TWO_TERMINAL_PINS,
    symbol_asset: "functional_fuse.svg",
    block: Some(TWO_TERMINAL_BLOCK),
    keywords: &[
        "fuse",
        "blown",
        "i2t",
        "fast acting fuse",
        "cartridge fuse",
        "overcurrent",
    ],
    category: ComponentCategory::ProtectionElectromechanical,
    parameters: FUSE_PARAMETERS,
    preset: None,
    inputs: None,
};

const RELAY_PINS: &[FunctionalPin] = &[
    pin("A1", InOut),
    pin("A2", InOut),
    pin("COM", InOut),
    pin("NO", InOut),
    pin("NC", InOut),
];

/// The coil drawn upright on the left, its terminals top and bottom; the
/// contact's common below on the right, under its NC and NO terminals.
const RELAY_BLOCK: FunctionalBlock = FunctionalBlock {
    height: 40,
    pins: &[
        (SymbolPinSide::Top, -20, -20),
        (SymbolPinSide::Bottom, -20, 20),
        (SymbolPinSide::Bottom, 10, 20),
        (SymbolPinSide::Top, 20, -20),
        (SymbolPinSide::Top, 0, -20),
    ],
};

/// A 12 V, 400 Ω general-purpose relay with a changeover contact.
const RELAY_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "RCOIL",
        "Coil resistance",
        "Ω",
        "400",
        "Resistance of the coil.",
    ),
    number(
        "LCOIL",
        "Coil inductance",
        "H",
        "600m",
        "Inductance of the coil, which slows its current after a voltage step.",
    ),
    number(
        "VPULLIN",
        "Pull-in voltage",
        "V",
        "9",
        "Coil voltage, either polarity, at which the armature pulls in: the contacts operate once the coil current reaches VPULLIN/RCOIL.",
    ),
    number(
        "VDROPOUT",
        "Drop-out voltage",
        "V",
        "1.2",
        "Coil voltage at which the armature drops out again; below the pull-in voltage, so the relay holds between them.",
    ),
    number(
        "TOP",
        "Operate time",
        "s",
        "5m",
        "Time from the coil reaching pull-in to the contacts changing over.",
    ),
    number(
        "TREL",
        "Release time",
        "s",
        "2m",
        "Time from the coil falling to drop-out to the contacts changing back.",
    ),
    number(
        "RON",
        "Contact resistance",
        "Ω",
        "50m",
        "Resistance of a closed contact.",
    ),
    number(
        "ROFF",
        "Open resistance",
        "Ω",
        "1G",
        "Resistance of an open contact.",
    ),
    selector(
        "FORM",
        "Contacts",
        "SPDT",
        "SPDT: COM connects to NC at rest and to NO once operated. SPST: a make contact from COM to NO, with NC never connected.",
        &[choice("SPDT", "1"), choice("SPST", "0")],
    ),
];

const RELAY_PRESET: FunctionalPreset = FunctionalPreset {
    name: "COIL",
    label: "Coil",
    description: "Fills the coil's resistance, inductance, pull-in and drop-out with a typical relay's for that coil voltage; each stays editable.",
    choices: &[
        FunctionalPresetChoice {
            label: "12 V",
            values: &[
                ("RCOIL", "400"),
                ("LCOIL", "600m"),
                ("VPULLIN", "9"),
                ("VDROPOUT", "1.2"),
            ],
        },
        FunctionalPresetChoice {
            label: "5 V",
            values: &[
                ("RCOIL", "70"),
                ("LCOIL", "100m"),
                ("VPULLIN", "3.75"),
                ("VDROPOUT", "0.5"),
            ],
        },
        FunctionalPresetChoice {
            label: "24 V",
            values: &[
                ("RCOIL", "1.6k"),
                ("LCOIL", "2.4"),
                ("VPULLIN", "18"),
                ("VDROPOUT", "2.4"),
            ],
        },
    ],
};

pub(super) const RELAY: FunctionalPartDescriptor = FunctionalPartDescriptor {
    stable_id: "rspice.functional.relay",
    display_name: "Relay",
    subcircuit: "RSPICE_FN_RELAY",
    pins: RELAY_PINS,
    symbol_asset: "functional_relay.svg",
    block: Some(RELAY_BLOCK),
    keywords: &[
        "relay",
        "coil",
        "contact",
        "spdt",
        "spst",
        "electromechanical relay",
        "changeover",
    ],
    category: ComponentCategory::ProtectionElectromechanical,
    parameters: RELAY_PARAMETERS,
    preset: Some(RELAY_PRESET),
    inputs: None,
};

const OPTO_PINS: &[FunctionalPin] = &[pin("A", In), pin("K", In), pin("C", Out), pin("E", Out)];

/// The LED's anode and cathode down the left edge, the phototransistor's
/// collector and emitter down the right.
const OPTO_BLOCK: FunctionalBlock = FunctionalBlock {
    height: 40,
    pins: &[
        (SymbolPinSide::Left, -30, -10),
        (SymbolPinSide::Left, -30, 10),
        (SymbolPinSide::Right, 30, -10),
        (SymbolPinSide::Right, 30, 10),
    ],
};

/// A 4N25-class phototransistor coupler.
const OPTO_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "CTR",
        "Current transfer ratio",
        "",
        "0.2",
        "Collector current per unit of LED current while the phototransistor is out of saturation.",
    ),
    number(
        "VF",
        "LED forward voltage",
        "V",
        "1.15",
        "LED voltage at the test current IFT.",
    ),
    number(
        "IFT",
        "LED test current",
        "A",
        "10m",
        "LED current at which its forward voltage is VF.",
    ),
    number(
        "TAU",
        "Response time constant",
        "s",
        "2u",
        "Time constant with which the collector current follows the LED current.",
    ),
    number(
        "VCESAT",
        "Saturation voltage",
        "V",
        "0.4",
        "Collector-emitter voltage at which the output carries half of CTR times the LED current; the output saturates below it.",
    ),
    number(
        "CISO",
        "Isolation capacitance",
        "F",
        "0.5p",
        "Capacitance across the isolation barrier, from the LED's cathode to the emitter.",
    ),
];

pub(super) const OPTOCOUPLER: FunctionalPartDescriptor = FunctionalPartDescriptor {
    stable_id: "rspice.functional.optocoupler",
    display_name: "Optocoupler",
    subcircuit: "RSPICE_FN_OPTO",
    pins: OPTO_PINS,
    symbol_asset: "functional_optocoupler.svg",
    block: Some(OPTO_BLOCK),
    keywords: &[
        "opto",
        "optocoupler",
        "optoisolator",
        "photocoupler",
        "phototransistor",
        "isolator",
        "4n25",
        "pc817",
    ],
    category: ComponentCategory::Semiconductors,
    parameters: OPTO_PARAMETERS,
    preset: None,
    inputs: None,
};
