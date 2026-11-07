//! Functional parts: subcircuits of the embedded functional library, placed
//! from the Component shelf as library cells.
//!
//! A functional part is data rather than a component type. Its subcircuit
//! lives in the functional library the engine resolves by name; this table
//! owns the rest of the part: its pins, artwork, the words a reader finds it
//! by, its shelf group and its parameter sheet. Placement freezes the part's
//! identity, pins and parameter names on the instance, and every later read
//! revalidates them here, so a placement whose part has since changed shape
//! fails closed instead of binding the wrong pins or parameters.

use rspice_design_model::{Point, symbol_pin::SymbolPinSide};

use super::{
    component::{BuiltinFunctionalInstance, LibraryCellInstance},
    component_category::ComponentCategory,
    port::{PortDirection, PortSpec},
};
use crate::symbol_generation::{GENERATED_WIDTH, GeneratedBody, GeneratedPin, GeneratedSymbol};

/// One pin of a functional part, in subcircuit port order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalPin {
    pub name: &'static str,
    pub direction: PortDirection,
}

/// One named setting of a selector parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalChoice {
    /// What the sheet offers and the instance stores.
    pub label: &'static str,
    /// What the instance card passes to the subcircuit.
    pub value: &'static str,
}

/// One subcircuit parameter as the parameter sheet presents it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalParameter {
    /// The subcircuit parameter, as the instance card spells it.
    pub name: &'static str,
    pub label: &'static str,
    pub unit: &'static str,
    /// The subcircuit's own default: for a selector, the label of the
    /// choice whose value the subcircuit defaults to.
    pub default: &'static str,
    pub description: &'static str,
    /// The settings of a selector parameter; empty for a numeric one.
    pub choices: &'static [FunctionalChoice],
}

/// One setting of a preset and the parameter values it fills in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalPresetChoice {
    pub label: &'static str,
    pub values: &'static [(&'static str, &'static str)],
}

/// A sheet setting that is no subcircuit parameter: choosing one of its
/// settings fills parameters in, and each stays editable afterwards. The
/// instance keeps the setting it was given, and its card never carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalPreset {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub choices: &'static [FunctionalPresetChoice],
}

/// Pin geometry a part draws instead of the generated block's: the block
/// height and, in pin order, the edge each pin meets and its terminal
/// relative to the block's centre. The block is the generated width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalBlock {
    pub height: i32,
    pub pins: &'static [(SymbolPinSide, i32, i32)],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionalPartDescriptor {
    pub stable_id: &'static str,
    pub display_name: &'static str,
    /// The functional-library subcircuit the part instantiates.
    pub subcircuit: &'static str,
    pub pins: &'static [FunctionalPin],
    pub symbol_asset: &'static str,
    /// The part's own pin geometry, or None for the generated block.
    pub block: Option<FunctionalBlock>,
    /// What a reader may type for this part on the Component shelf besides
    /// its display name.
    pub keywords: &'static [&'static str],
    pub category: ComponentCategory,
    pub parameters: &'static [FunctionalParameter],
    /// A setting that fills parameters in, shown above them.
    pub preset: Option<FunctionalPreset>,
}

const fn pin(name: &'static str, direction: PortDirection) -> FunctionalPin {
    FunctionalPin { name, direction }
}

const fn number(
    name: &'static str,
    label: &'static str,
    unit: &'static str,
    default: &'static str,
    description: &'static str,
) -> FunctionalParameter {
    FunctionalParameter {
        name,
        label,
        unit,
        default,
        description,
        choices: &[],
    }
}

const fn selector(
    name: &'static str,
    label: &'static str,
    default: &'static str,
    description: &'static str,
    choices: &'static [FunctionalChoice],
) -> FunctionalParameter {
    FunctionalParameter {
        name,
        label,
        unit: "",
        default,
        description,
        choices,
    }
}

const fn choice(label: &'static str, value: &'static str) -> FunctionalChoice {
    FunctionalChoice { label, value }
}

use PortDirection::{In, InOut, Out, Supply};

const AMPLIFIER_PINS: &[FunctionalPin] = &[
    pin("IN+", In),
    pin("IN-", In),
    pin("V+", Supply),
    pin("V-", Supply),
    pin("OUT", Out),
];

/// A general-purpose 10 MHz rail-to-rail CMOS voltage-feedback op-amp.
const OP_AMP_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "AVOL",
        "Open-loop gain",
        "V/V",
        "200k",
        "DC open-loop voltage gain.",
    ),
    number(
        "GBW",
        "Gain-bandwidth",
        "Hz",
        "10Meg",
        "Frequency at which the single-pole open-loop gain falls to one.",
    ),
    number(
        "SLEW",
        "Slew rate",
        "V/s",
        "10Meg",
        "Largest rate of change of the output, rising and falling.",
    ),
    number(
        "VOS",
        "Input offset voltage",
        "V",
        "0",
        "Added to the non-inverting input.",
    ),
    number(
        "IB",
        "Input bias current",
        "A",
        "0",
        "Mean of the currents flowing into the two inputs.",
    ),
    number(
        "IOS",
        "Input offset current",
        "A",
        "0",
        "IN+ bias current less IN- bias current.",
    ),
    number(
        "RIN",
        "Input resistance",
        "Ω",
        "1T",
        "Differential resistance between the inputs.",
    ),
    number(
        "CIN",
        "Input capacitance",
        "F",
        "2p",
        "Differential capacitance between the inputs.",
    ),
    number(
        "EN",
        "Voltage noise",
        "V/√Hz",
        "10n",
        "White input-referred voltage noise density.",
    ),
    number(
        "ENK",
        "Voltage noise corner",
        "Hz",
        "100",
        "1/f corner of the voltage noise; 0 leaves it white.",
    ),
    number(
        "IN",
        "Current noise",
        "A/√Hz",
        "1f",
        "White current noise density between the inputs.",
    ),
    number(
        "INK",
        "Current noise corner",
        "Hz",
        "0",
        "1/f corner of the current noise; 0 leaves it white.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "50",
        "Open-loop output resistance.",
    ),
    number(
        "ISRC",
        "Source current limit",
        "A",
        "25m",
        "Largest current the output sources, drawn from V+.",
    ),
    number(
        "ISNK",
        "Sink current limit",
        "A",
        "25m",
        "Largest current the output sinks, returned to V-.",
    ),
    number(
        "HEADROOM",
        "Rail headroom",
        "V",
        "0",
        "How far inside each supply pin the output swing stops; 0 is rail-to-rail.",
    ),
    number(
        "IQ",
        "Quiescent current",
        "A",
        "1m",
        "Supply current from V+ to V- with no load.",
    ),
];

/// A fast general-purpose comparator with a push-pull output.
const COMPARATOR_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "VOS",
        "Input offset voltage",
        "V",
        "0",
        "Added to the non-inverting input; the thresholds centre on -VOS.",
    ),
    number(
        "HYST",
        "Hysteresis",
        "V",
        "1m",
        "Distance between the rising and falling input thresholds.",
    ),
    number(
        "TPD",
        "Propagation delay",
        "s",
        "100n",
        "Time from the input crossing a threshold to the output edge starting.",
    ),
    number(
        "TRISE",
        "Rise time",
        "s",
        "10n",
        "Time the output takes to rise across its full swing.",
    ),
    number(
        "TFALL",
        "Fall time",
        "s",
        "10n",
        "Time the output takes to fall across its full swing.",
    ),
    selector(
        "OC",
        "Output stage",
        "Push-pull",
        "Push-pull drives both ways between the supply pins; open collector only sinks toward V- and needs a pull-up.",
        &[choice("Push-pull", "0"), choice("Open collector", "1")],
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "50",
        "Resistance of the output stage.",
    ),
    number(
        "ISRC",
        "Source current limit",
        "A",
        "20m",
        "Largest current the output sources, drawn from V+.",
    ),
    number(
        "ISNK",
        "Sink current limit",
        "A",
        "20m",
        "Largest current the output sinks, returned to V-.",
    ),
    number(
        "IQ",
        "Quiescent current",
        "A",
        "500u",
        "Supply current from V+ to V- with no load.",
    ),
];

const OTA_PINS: &[FunctionalPin] = &[
    pin("IN+", In),
    pin("IN-", In),
    pin("IABC", In),
    pin("V+", Supply),
    pin("V-", Supply),
    pin("OUT", Out),
];

/// The Op-Amp's triangle and pins, with the bias current entering the lower
/// edge left of V-, as the LM13700 and CA3080 datasheets draw it.
const OTA_BLOCK: FunctionalBlock = FunctionalBlock {
    height: 40,
    pins: &[
        (SymbolPinSide::Left, -30, -10),
        (SymbolPinSide::Left, -30, 10),
        (SymbolPinSide::Bottom, -10, 20),
        (SymbolPinSide::Top, 0, -20),
        (SymbolPinSide::Bottom, 0, 20),
        (SymbolPinSide::Right, 30, 0),
    ],
};

/// A bipolar operational transconductance amplifier biased by the current
/// into its IABC pin.
const OTA_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "VLIN",
        "Input linear range",
        "V",
        "51.7m",
        "The output current is IABC·tanh(vd/VLIN), so the transconductance is IABC/VLIN; 51.7 mV is a bipolar pair's 2kT/q.",
    ),
    number(
        "VOS",
        "Input offset voltage",
        "V",
        "0",
        "Added to the non-inverting input.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "10Meg",
        "Resistance from the output to ground.",
    ),
    number(
        "COUT",
        "Output capacitance",
        "F",
        "5p",
        "Capacitance from the output to ground.",
    ),
    number(
        "HEADROOM",
        "Rail headroom",
        "V",
        "1",
        "How far inside each supply pin the output voltage stops.",
    ),
    number(
        "IQ",
        "Quiescent current",
        "A",
        "1m",
        "Supply current from V+ to V- besides the bias current.",
    ),
];

const ONE_INPUT_PINS: &[FunctionalPin] = &[pin("A", In), pin("Y", Out)];

const TWO_INPUT_PINS: &[FunctionalPin] = &[pin("A", In), pin("B", In), pin("Y", Out)];

const LOGIC_LEVELS: [FunctionalParameter; 2] = [
    number(
        "VHIGH",
        "High level",
        "V",
        "5",
        "Output high voltage; an input is high at or above the midpoint of the two levels.",
    ),
    number(
        "VLOW",
        "Low level",
        "V",
        "0",
        "Output low voltage; an input is low below the midpoint of the two levels.",
    ),
];

const LOGIC_TIMING: [FunctionalParameter; 4] = [
    number(
        "TD",
        "Delay",
        "s",
        "10n",
        "Time from an input change to the output edge starting; a pulse shorter than this never reaches the output.",
    ),
    number(
        "TRISE",
        "Rise time",
        "s",
        "5n",
        "Time the output takes to rise from low to high.",
    ),
    number(
        "TFALL",
        "Fall time",
        "s",
        "5n",
        "Time the output takes to fall from high to low.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "50",
        "Resistance the output drives through.",
    ),
];

/// Analog-threshold logic: levels, then timing and drive.
const LOGIC_PARAMETERS: &[FunctionalParameter] = &[
    LOGIC_LEVELS[0],
    LOGIC_LEVELS[1],
    LOGIC_TIMING[0],
    LOGIC_TIMING[1],
    LOGIC_TIMING[2],
    LOGIC_TIMING[3],
];

/// A Schmitt trigger: its two thresholds, then the logic levels and timing.
const SCHMITT_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "VT",
        "Threshold centre",
        "V",
        "2.5",
        "Midpoint of the rising and falling input thresholds.",
    ),
    number(
        "HYST",
        "Hysteresis",
        "V",
        "0.5",
        "The input rises through VT+HYST/2 to switch and falls through VT-HYST/2 to switch back.",
    ),
    number("VHIGH", "High level", "V", "5", "Output high voltage."),
    number("VLOW", "Low level", "V", "0", "Output low voltage."),
    LOGIC_TIMING[0],
    LOGIC_TIMING[1],
    LOGIC_TIMING[2],
    LOGIC_TIMING[3],
];

const TIMER_PINS: &[FunctionalPin] = &[
    pin("TRIG", In),
    pin("THR", In),
    pin("RESET", In),
    pin("OUT", Out),
    pin("DIS", InOut),
    pin("CTRL", InOut),
    pin("VCC", Supply),
    pin("GND", Supply),
];

/// A bipolar 555; the Process preset fills in a CMOS one.
const TIMER_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "DROP",
        "Output high drop",
        "V",
        "1.4",
        "How far below VCC the output stands when high.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "5",
        "Resistance the output drives through.",
    ),
    number(
        "ISRC",
        "Output source limit",
        "A",
        "200m",
        "Most current the output sources when high.",
    ),
    number(
        "ISNK",
        "Output sink limit",
        "A",
        "200m",
        "Most current the output sinks when low.",
    ),
    number(
        "TEDGE",
        "Output edge time",
        "s",
        "100n",
        "Time the output takes to cross its full swing.",
    ),
    number(
        "IQ",
        "Quiescent current",
        "A",
        "3m",
        "Supply current from VCC to GND besides the divider's.",
    ),
    number(
        "RDIV",
        "Divider resistors",
        "Ω",
        "5k",
        "Each of the three equal resistors that set the thresholds at 1/3 and 2/3 of VCC.",
    ),
    number(
        "RDIS",
        "Discharge on-resistance",
        "Ω",
        "10",
        "Resistance from DIS to GND while the output is low.",
    ),
    number(
        "IDIS",
        "Discharge current limit",
        "A",
        "50m",
        "Most current DIS sinks while the output is low.",
    ),
];

const TIMER_PRESET: FunctionalPreset = FunctionalPreset {
    name: "PROCESS",
    label: "Process",
    description: "Fills the parameters below with a bipolar 555's or a CMOS 555's values; each stays editable.",
    choices: &[
        FunctionalPresetChoice {
            label: "Bipolar",
            values: &[
                ("DROP", "1.4"),
                ("ROUT", "5"),
                ("ISRC", "200m"),
                ("ISNK", "200m"),
                ("TEDGE", "100n"),
                ("IQ", "3m"),
                ("RDIV", "5k"),
                ("RDIS", "10"),
                ("IDIS", "50m"),
            ],
        },
        FunctionalPresetChoice {
            label: "CMOS",
            values: &[
                ("DROP", "0"),
                ("ROUT", "50"),
                ("ISRC", "10m"),
                ("ISNK", "100m"),
                ("TEDGE", "20n"),
                ("IQ", "100u"),
                ("RDIV", "100k"),
                ("RDIS", "10"),
                ("IDIS", "50m"),
            ],
        },
    ],
};

const PHASE_DETECTOR_PINS: &[FunctionalPin] = &[pin("A", In), pin("B", In), pin("OUT", Out)];

const PHASE_DETECTOR_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "IOUT",
        "Charge-pump current",
        "A",
        "100u",
        "Current OUT sources while A leads and sinks while B leads.",
    ),
    number(
        "VTH",
        "Input threshold",
        "V",
        "2.5",
        "An input edge counts when it rises through this level.",
    ),
    number(
        "VHIGH",
        "Output high clamp",
        "V",
        "5",
        "Highest voltage the output is driven to.",
    ),
    number(
        "VLOW",
        "Output low clamp",
        "V",
        "0",
        "Lowest voltage the output is driven to.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "1G",
        "Resistance from the output to ground.",
    ),
];

const COUNTER_PINS: &[FunctionalPin] = &[pin("CLK", In), pin("RST", In), pin("OUT", Out)];

const COUNTER_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "N",
        "Divide ratio",
        "",
        "2",
        "Rising CLK edges per output cycle.",
    ),
    number(
        "DUTY",
        "Duty cycle",
        "",
        "0.5",
        "Fraction of each cycle's counts the output is high, rounded to whole counts.",
    ),
    LOGIC_LEVELS[0],
    LOGIC_LEVELS[1],
    LOGIC_TIMING[1],
    LOGIC_TIMING[2],
    LOGIC_TIMING[3],
];

const SAMPLE_HOLD_PINS: &[FunctionalPin] = &[pin("IN", In), pin("SAMPLE", In), pin("OUT", Out)];

const SAMPLE_HOLD_PARAMETERS: &[FunctionalParameter] = &[
    number(
        "VTH",
        "Sample threshold",
        "V",
        "2.5",
        "OUT tracks IN while SAMPLE is above this level and holds when it falls through it.",
    ),
    number(
        "ROUT",
        "Output resistance",
        "Ω",
        "1",
        "Resistance the held value drives the output through.",
    ),
    number(
        "COUT",
        "Output capacitance",
        "F",
        "10p",
        "Capacitance from the output to ground.",
    ),
];

/// A part of the Logic & timing group.
const fn logic_part(
    stable_id: &'static str,
    display_name: &'static str,
    subcircuit: &'static str,
    pins: &'static [FunctionalPin],
    symbol_asset: &'static str,
    keywords: &'static [&'static str],
    parameters: &'static [FunctionalParameter],
) -> FunctionalPartDescriptor {
    FunctionalPartDescriptor {
        stable_id,
        display_name,
        subcircuit,
        pins,
        symbol_asset,
        block: None,
        keywords,
        category: ComponentCategory::LogicTiming,
        parameters,
        preset: None,
    }
}

static FUNCTIONAL_PARTS: &[FunctionalPartDescriptor] = &[
    FunctionalPartDescriptor {
        stable_id: "rspice.functional.opamp",
        display_name: "Op-Amp",
        subcircuit: "RSPICE_FN_OPAMP",
        pins: AMPLIFIER_PINS,
        symbol_asset: "functional_opamp.svg",
        block: None,
        keywords: &[
            "op amp",
            "opamp",
            "operational amplifier",
            "amplifier",
            "voltage feedback",
        ],
        category: ComponentCategory::AmplifiersComparators,
        parameters: OP_AMP_PARAMETERS,
        preset: None,
    },
    FunctionalPartDescriptor {
        stable_id: "rspice.functional.comparator",
        display_name: "Comparator",
        subcircuit: "RSPICE_FN_COMPARATOR",
        pins: AMPLIFIER_PINS,
        symbol_asset: "functional_comparator.svg",
        block: None,
        keywords: &[
            "comparator",
            "lm393",
            "lm339",
            "lm311",
            "voltage comparator",
            "open collector",
            "hysteresis",
        ],
        category: ComponentCategory::AmplifiersComparators,
        parameters: COMPARATOR_PARAMETERS,
        preset: None,
    },
    FunctionalPartDescriptor {
        stable_id: "rspice.functional.ota",
        display_name: "OTA",
        subcircuit: "RSPICE_FN_OTA",
        pins: OTA_PINS,
        symbol_asset: "functional_ota.svg",
        block: Some(OTA_BLOCK),
        keywords: &[
            "ota",
            "operational transconductance amplifier",
            "transconductance amplifier",
            "lm13700",
            "ca3080",
            "gm",
            "vca",
        ],
        category: ComponentCategory::AmplifiersComparators,
        parameters: OTA_PARAMETERS,
        preset: None,
    },
    logic_part(
        "rspice.functional.and2",
        "AND Gate",
        "RSPICE_FN_AND2",
        TWO_INPUT_PINS,
        "functional_and.svg",
        &["and", "logic gate", "74hc08"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.nand2",
        "NAND Gate",
        "RSPICE_FN_NAND2",
        TWO_INPUT_PINS,
        "functional_nand.svg",
        &["nand", "logic gate", "74hc00"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.or2",
        "OR Gate",
        "RSPICE_FN_OR2",
        TWO_INPUT_PINS,
        "functional_or.svg",
        &["or", "logic gate", "74hc32"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.nor2",
        "NOR Gate",
        "RSPICE_FN_NOR2",
        TWO_INPUT_PINS,
        "functional_nor.svg",
        &["nor", "logic gate", "74hc02"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.xor2",
        "XOR Gate",
        "RSPICE_FN_XOR2",
        TWO_INPUT_PINS,
        "functional_xor.svg",
        &["xor", "exclusive or", "logic gate", "74hc86"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.inverter",
        "Inverter",
        "RSPICE_FN_INV",
        ONE_INPUT_PINS,
        "functional_inverter.svg",
        &["inverter", "not", "not gate", "logic gate", "74hc04"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.buffer",
        "Buffer",
        "RSPICE_FN_BUF",
        ONE_INPUT_PINS,
        "functional_buffer.svg",
        &["buffer", "logic buffer", "driver", "logic gate"],
        LOGIC_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.schmitt_inverter",
        "Schmitt Inverter",
        "RSPICE_FN_SCHMITT_INV",
        ONE_INPUT_PINS,
        "functional_schmitt_inverter.svg",
        &[
            "schmitt",
            "schmitt trigger",
            "inverter",
            "hysteresis",
            "74hc14",
        ],
        SCHMITT_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.schmitt_buffer",
        "Schmitt Buffer",
        "RSPICE_FN_SCHMITT_BUF",
        ONE_INPUT_PINS,
        "functional_schmitt_buffer.svg",
        &["schmitt", "schmitt trigger", "buffer", "hysteresis"],
        SCHMITT_PARAMETERS,
    ),
    FunctionalPartDescriptor {
        preset: Some(TIMER_PRESET),
        ..logic_part(
            "rspice.functional.timer555",
            "555 Timer",
            "RSPICE_FN_555",
            TIMER_PINS,
            "functional_555.svg",
            &[
                "555",
                "timer",
                "ne555",
                "lm555",
                "tlc555",
                "icm7555",
                "astable",
                "monostable",
                "oscillator",
            ],
            TIMER_PARAMETERS,
        )
    },
    logic_part(
        "rspice.functional.phase_detector",
        "Phase Detector",
        "RSPICE_FN_PHASEDET",
        PHASE_DETECTOR_PINS,
        "functional_phase_detector.svg",
        &[
            "phase detector",
            "pfd",
            "phase frequency detector",
            "charge pump",
            "pll",
        ],
        PHASE_DETECTOR_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.counter",
        "Counter",
        "RSPICE_FN_COUNTER",
        COUNTER_PINS,
        "functional_counter.svg",
        &[
            "divide by n",
            "divider",
            "frequency divider",
            "prescaler",
            "counter",
        ],
        COUNTER_PARAMETERS,
    ),
    logic_part(
        "rspice.functional.sample_hold",
        "Sample & Hold",
        "RSPICE_FN_SAMPLEHOLD",
        SAMPLE_HOLD_PINS,
        "functional_sample_hold.svg",
        &["sample and hold", "track and hold", "s/h", "sampler"],
        SAMPLE_HOLD_PARAMETERS,
    ),
];

/// Every functional part, in catalog order.
pub fn functional_parts() -> &'static [FunctionalPartDescriptor] {
    FUNCTIONAL_PARTS
}

/// The functional part a placement's frozen identity names.
pub fn functional_part(stable_id: &str) -> Option<&'static FunctionalPartDescriptor> {
    FUNCTIONAL_PARTS
        .iter()
        .find(|descriptor| descriptor.stable_id == stable_id)
}

/// The library-cell binding a new placement of `descriptor` carries.
pub fn functional_library_binding(descriptor: &FunctionalPartDescriptor) -> LibraryCellInstance {
    let ports = descriptor
        .pins
        .iter()
        .map(|pin| PortSpec {
            name: pin.name.to_owned(),
            direction: pin.direction,
        })
        .collect::<Vec<_>>();
    let mut binding = LibraryCellInstance::new("rspice", descriptor.subcircuit, "functional");
    binding.module_name = Some(descriptor.subcircuit.to_owned());
    binding.reference_prefix = Some("X".to_owned());
    binding.parameter_order = descriptor
        .parameters
        .iter()
        .map(|parameter| parameter.name.to_owned())
        .collect();
    binding.bind_interface(&ports);
    binding.builtin_functional = Some(BuiltinFunctionalInstance {
        stable_id: descriptor.stable_id.to_owned(),
        subcircuit: descriptor.subcircuit.to_owned(),
        symbol_asset: descriptor.symbol_asset.to_owned(),
    });
    binding
}

/// The block a functional placement draws when its part sets its own pin
/// geometry and the placement still has the part's pins; None otherwise, and
/// the generated block applies.
pub fn functional_block_symbol(binding: &LibraryCellInstance) -> Option<GeneratedSymbol> {
    let descriptor = functional_part(&binding.builtin_functional.as_ref()?.stable_id)?;
    let block = descriptor.block?;
    let same_pins = binding.terminal_order.len() == descriptor.pins.len()
        && block.pins.len() == descriptor.pins.len()
        && binding
            .terminal_order
            .iter()
            .zip(descriptor.pins)
            .all(|(placed, pin)| placed == pin.name);
    same_pins.then(|| GeneratedSymbol {
        pins: descriptor
            .pins
            .iter()
            .zip(block.pins)
            .map(|(pin, &(side, x, y))| GeneratedPin {
                name: pin.name.to_owned(),
                direction: pin.direction,
                side,
                offset: Point::new(x, y),
            })
            .collect(),
        width: GENERATED_WIDTH,
        height: block.height,
        body: GeneratedBody::Block,
    })
}

/// The value a new placement of `binding` shows: a functional part's name,
/// else the cell it places.
pub fn placement_value(binding: &LibraryCellInstance) -> String {
    binding
        .builtin_functional
        .as_ref()
        .and_then(|contract| functional_part(&contract.stable_id))
        .map_or_else(|| binding.cell.clone(), |part| part.display_name.to_owned())
}

/// The part a functional placement still is, or why it no longer binds.
pub fn validate_builtin_functional_binding(
    binding: &LibraryCellInstance,
) -> Result<&'static FunctionalPartDescriptor, String> {
    let contract = binding
        .builtin_functional
        .as_ref()
        .ok_or_else(|| "the placement is not a functional part".to_owned())?;
    let descriptor = functional_part(&contract.stable_id).ok_or_else(|| {
        format!(
            "functional part '{}' is not part of this build",
            contract.stable_id
        )
    })?;
    let expected = functional_library_binding(descriptor);
    let same_parameters = binding.parameter_order.len() == expected.parameter_order.len()
        && binding
            .parameter_order
            .iter()
            .zip(&expected.parameter_order)
            .all(|(placed, current)| placed.eq_ignore_ascii_case(current));
    if binding.builtin_functional != expected.builtin_functional
        || binding.module_name != expected.module_name
        || binding.terminal_order != expected.terminal_order
        || binding.terminal_dirs != expected.terminal_dirs
        || !same_parameters
    {
        return Err(format!(
            "this {} was placed from an earlier revision of the part; replace it from the Component shelf",
            descriptor.display_name
        ));
    }
    Ok(descriptor)
}

/// The parameter text a placement's instance card carries: the parameters
/// the instance sets, in sheet order, with each selector's label replaced by
/// the value the subcircuit reads.
pub fn functional_instance_parameters(
    descriptor: &FunctionalPartDescriptor,
    raw: &str,
) -> Result<String, String> {
    if raw.trim().is_empty() {
        return Ok(String::new());
    }
    if raw.chars().any(char::is_control) {
        return Err("parameter text contains a line break or control character".to_owned());
    }
    let parsed = super::replacement::parse_replacement_parameters_strict(raw)
        .map_err(|error| error.to_string())?;
    let mut written = Vec::new();
    for parameter in descriptor.parameters {
        let Some((_, value)) = parsed
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(parameter.name))
        else {
            continue;
        };
        let value = value.trim();
        if value.is_empty()
            || value.len() > 512
            || value
                .chars()
                .any(|character| matches!(character, ';' | '$' | '#' | '\\'))
        {
            return Err(format!("{} has an empty or unsafe value", parameter.label));
        }
        let value = if parameter.choices.is_empty() {
            value
        } else {
            // A setting whose label has a space is stored quoted.
            let setting = ['"', '\'']
                .into_iter()
                .find_map(|quote| value.strip_prefix(quote)?.strip_suffix(quote))
                .unwrap_or(value);
            parameter
                .choices
                .iter()
                .find(|choice| {
                    choice.label.eq_ignore_ascii_case(setting) || choice.value == setting
                })
                .map(|choice| choice.value)
                .ok_or_else(|| {
                    format!(
                        "{} is '{value}'; it takes {}",
                        parameter.label,
                        parameter
                            .choices
                            .iter()
                            .map(|choice| choice.label)
                            .collect::<Vec<_>>()
                            .join(" or ")
                    )
                })?
        };
        written.push(format!("{}={value}", parameter.name));
    }
    if let Some(preset) = descriptor.preset
        && let Some((_, value)) = parsed
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(preset.name))
        && preset_choice(&preset, value).is_none()
    {
        return Err(format!(
            "{} is '{}'; it takes {}",
            preset.label,
            value.trim(),
            preset
                .choices
                .iter()
                .map(|choice| choice.label)
                .collect::<Vec<_>>()
                .join(" or ")
        ));
    }
    if let Some((key, _)) = parsed.iter().find(|(key, _)| {
        !descriptor
            .parameters
            .iter()
            .any(|parameter| parameter.name.eq_ignore_ascii_case(key))
            && descriptor
                .preset
                .is_none_or(|preset| !preset.name.eq_ignore_ascii_case(key))
    }) {
        return Err(format!(
            "{} has no parameter '{}'",
            descriptor.display_name,
            key.to_ascii_uppercase()
        ));
    }
    Ok(written
        .into_iter()
        .map(|parameter| format!(" {parameter}"))
        .collect())
}

/// The setting of `preset` that `value` names, quoted or not.
fn preset_choice(preset: &FunctionalPreset, value: &str) -> Option<FunctionalPresetChoice> {
    let value = value.trim();
    let setting = ['"', '\'']
        .into_iter()
        .find_map(|quote| value.strip_prefix(quote)?.strip_suffix(quote))
        .unwrap_or(value);
    preset
        .choices
        .iter()
        .find(|choice| choice.label.eq_ignore_ascii_case(setting))
        .copied()
}

/// `params` after choosing `setting` of the part's preset: the setting is
/// recorded, and every parameter it fills is written over.
pub fn apply_functional_preset(
    descriptor: &FunctionalPartDescriptor,
    params: &str,
    setting: &str,
) -> Result<String, String> {
    let preset = descriptor
        .preset
        .ok_or_else(|| format!("{} has no preset", descriptor.display_name))?;
    let choice = preset_choice(&preset, setting)
        .ok_or_else(|| format!("{} has no {} '{setting}'", descriptor.display_name, preset.label))?;
    let mut params = crate::parameters::set_parameter_value(params, preset.name, choice.label)?;
    for (name, value) in choice.values {
        params = crate::parameters::set_parameter_value(&params, name, value)?;
    }
    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core_netlist::netlist::{lexer::parse_spice_value_complete, parse_netlist};

    #[test]
    fn every_part_is_a_library_subcircuit_whose_parameters_are_its_sheet() {
        let library =
            parse_netlist(rspice_core_base::builtin_lib::FUNCTIONAL_LIB).expect("library parses");
        for descriptor in functional_parts() {
            let subcircuit = library
                .subcircuits
                .iter()
                .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case(descriptor.subcircuit))
                .unwrap_or_else(|| panic!("{} has no subcircuit", descriptor.display_name));
            assert_eq!(
                subcircuit.ports.len(),
                descriptor.pins.len(),
                "{} pins",
                descriptor.display_name
            );
            // The sheet is the subcircuit: every parameter it declares is
            // offered, and nothing is offered that it does not declare.
            let declared = subcircuit
                .params
                .iter()
                .map(|(name, _)| name)
                .chain(subcircuit.expr_params.iter().map(|(name, _)| name))
                .map(|name| name.to_ascii_uppercase())
                .collect::<std::collections::BTreeSet<_>>();
            let offered = descriptor
                .parameters
                .iter()
                .map(|parameter| parameter.name.to_ascii_uppercase())
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                offered.len(),
                descriptor.parameters.len(),
                "{} offers a parameter twice",
                descriptor.display_name
            );
            assert_eq!(
                offered, declared,
                "{}: the sheet and the subcircuit name different parameters",
                descriptor.display_name
            );
            for parameter in descriptor.parameters {
                let default = subcircuit
                    .params
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(parameter.name))
                    .map(|(_, value)| *value)
                    .unwrap_or_else(|| {
                        panic!("{} has no {}", descriptor.subcircuit, parameter.name)
                    });
                let shown = if parameter.choices.is_empty() {
                    parameter.default
                } else {
                    parameter
                        .choices
                        .iter()
                        .find(|choice| choice.label == parameter.default)
                        .map(|choice| choice.value)
                        .expect("a selector defaults to one of its choices")
                };
                let shown = parse_spice_value_complete(shown)
                    .unwrap_or_else(|_| panic!("{} default {shown}", parameter.name));
                assert!(
                    (shown - default).abs() <= 1e-12 * default.abs(),
                    "{}.{}: the sheet shows {shown}, the subcircuit defaults to {default}",
                    descriptor.subcircuit,
                    parameter.name
                );
            }
        }
    }

    #[test]
    fn a_placement_binds_and_revalidates_its_frozen_part() {
        for descriptor in functional_parts() {
            let binding = functional_library_binding(descriptor);
            assert_eq!(
                validate_builtin_functional_binding(&binding).map(|part| part.stable_id),
                Ok(descriptor.stable_id)
            );
            let encoded = serde_json::to_string(&binding).expect("serializes");
            let decoded: LibraryCellInstance =
                serde_json::from_str(&encoded).expect("deserializes");
            assert!(validate_builtin_functional_binding(&decoded).is_ok());

            let mut reordered = binding.clone();
            reordered.terminal_order.swap(0, 1);
            assert!(validate_builtin_functional_binding(&reordered).is_err());
            let mut renamed = binding;
            renamed.parameter_order.pop();
            assert!(validate_builtin_functional_binding(&renamed).is_err());
        }
    }

    #[test]
    fn a_part_drawing_its_own_block_keeps_every_pin_on_its_edge_and_on_the_grid() {
        for descriptor in functional_parts() {
            let binding = functional_library_binding(descriptor);
            let Some(block) = functional_block_symbol(&binding) else {
                assert!(descriptor.block.is_none(), "{}", descriptor.display_name);
                continue;
            };
            let (half_width, half_height) = (block.width / 2, block.height / 2);
            for pin in &block.pins {
                let Point { x, y } = pin.offset;
                let on_edge = match pin.side {
                    SymbolPinSide::Left => x == -half_width,
                    SymbolPinSide::Right => x == half_width,
                    SymbolPinSide::Top => y == -half_height,
                    SymbolPinSide::Bottom => y == half_height,
                };
                assert!(
                    on_edge
                        && x % 10 == 0
                        && y % 10 == 0
                        && x.abs() <= half_width
                        && y.abs() <= half_height,
                    "{} pin {} at ({x}, {y})",
                    descriptor.display_name,
                    pin.name
                );
            }
            // Connectivity reads pins through the resolver: it must agree.
            let buffers = std::collections::HashMap::<
                String,
                crate::schematic::document::SchematicDocument,
            >::new();
            let libraries = crate::library::LibraryCatalog::default();
            let resolved = crate::symbol_resolver::SymbolResolver::new(&libraries, &buffers)
                .resolve_binding(&binding)
                .expect("a functional part resolves");
            let geometry =
                |name: &str, side: SymbolPinSide, offset: Point| (name.to_owned(), side, offset);
            assert_eq!(
                resolved
                    .connectable_pins()
                    .map(|pin| geometry(&pin.name, pin.side, pin.offset))
                    .collect::<Vec<_>>(),
                block
                    .pins
                    .iter()
                    .map(|pin| geometry(&pin.name, pin.side, pin.offset))
                    .collect::<Vec<_>>(),
                "{}",
                descriptor.display_name
            );
            let mut stale = binding.clone();
            stale.terminal_order.swap(0, 1);
            assert!(functional_block_symbol(&stale).is_none());
        }
    }

    #[test]
    fn a_placement_shows_its_part_name() {
        for descriptor in functional_parts() {
            assert_eq!(
                placement_value(&functional_library_binding(descriptor)),
                descriptor.display_name
            );
        }
    }

    #[test]
    fn instance_parameters_are_declared_ordered_and_selector_values() {
        let op_amp = functional_part("rspice.functional.opamp").expect("op-amp");
        assert_eq!(
            functional_instance_parameters(op_amp, "slew=1Meg AVOL=1e5"),
            Ok(" AVOL=1e5 SLEW=1Meg".to_owned())
        );
        assert_eq!(
            functional_instance_parameters(op_amp, "  "),
            Ok(String::new())
        );
        assert!(
            functional_instance_parameters(op_amp, "GAIN=2")
                .unwrap_err()
                .contains("no parameter 'GAIN'")
        );
        assert!(functional_instance_parameters(op_amp, "AVOL=1;2").is_err());

        let comparator = functional_part("rspice.functional.comparator").expect("comparator");
        assert_eq!(
            functional_instance_parameters(comparator, "OC=\"Open collector\" HYST=10m"),
            Ok(" HYST=10m OC=1".to_owned())
        );
        assert_eq!(
            functional_instance_parameters(comparator, "oc=push-pull"),
            Ok(" OC=0".to_owned())
        );
        assert!(
            functional_instance_parameters(comparator, "OC=Totem")
                .unwrap_err()
                .contains("takes Push-pull or Open collector")
        );
    }

    #[test]
    fn a_preset_fills_its_parameters_and_never_reaches_the_card() {
        let library =
            parse_netlist(rspice_core_base::builtin_lib::FUNCTIONAL_LIB).expect("library parses");
        let value = |text: &str| parse_spice_value_complete(text).expect("a number");
        for descriptor in functional_parts() {
            let Some(preset) = descriptor.preset else {
                continue;
            };
            let subcircuit = library
                .subcircuits
                .iter()
                .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case(descriptor.subcircuit))
                .expect("subcircuit");
            // The first setting is the subcircuit's own defaults, and every
            // setting fills the same parameters.
            for (name, filled) in preset.choices[0].values {
                let default = subcircuit
                    .params
                    .iter()
                    .find(|(declared, _)| declared.eq_ignore_ascii_case(name))
                    .map(|(_, default)| *default)
                    .unwrap_or_else(|| panic!("{} has no {name}", descriptor.subcircuit));
                assert_eq!(value(filled), default, "{}.{name}", descriptor.subcircuit);
            }
            for choice in preset.choices {
                let names = |choice: &FunctionalPresetChoice| {
                    choice.values.iter().map(|(name, _)| *name).collect::<Vec<_>>()
                };
                assert_eq!(names(choice), names(&preset.choices[0]), "{}", choice.label);
            }
        }

        let timer = functional_part("rspice.functional.timer555").expect("555");
        let cmos = apply_functional_preset(timer, "ROUT=7 IDIS=20m", "cmos").expect("applies");
        assert_eq!(
            cmos,
            "ROUT=50 IDIS=50m PROCESS=CMOS DROP=0 ISRC=10m ISNK=100m TEDGE=20n IQ=100u RDIV=100k RDIS=10"
        );
        // The setting is kept on the instance and left off its card.
        let card = functional_instance_parameters(timer, &cmos).expect("a valid card");
        assert!(!card.contains("PROCESS") && card.contains(" DROP=0 ROUT=50"), "{card}");
        assert!(
            functional_instance_parameters(timer, "PROCESS=GaAs")
                .unwrap_err()
                .contains("takes Bipolar or CMOS")
        );
        assert!(apply_functional_preset(timer, "", "GaAs").is_err());
    }
}
