//! The passive, protection, electromechanical and power-semiconductor
//! functional parts. Each default describes a sensible generic part of its
//! kind; none is any manufacturer's part.

use rspice_design_model::symbol_pin::SymbolPinSide;

use super::{
    ComponentCategory, FunctionalBlock, FunctionalParameter, FunctionalPartDescriptor,
    FunctionalPin, FunctionalPreset, FunctionalPresetChoice, InOut, number, pin,
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
