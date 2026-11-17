//! The parameters a new `.MODEL` card of each device type starts with, at
//! the engine's own defaults.
//!
//! A model is created as Virtuoso's and ADS's are: the device type first,
//! then the card with that type's parameters at their defaults, which the
//! designer edits from there. This is the one table those defaults come
//! from. Each value is what the engine takes when the card leaves the
//! parameter out, so a card written from the table simulates the same device
//! as the bare card; the engine's drift test builds both and compares them,
//! parameter by parameter.
//!
//! A parameter whose presence changes the model, not only its value, is left
//! out: an omitted `BV` means no breakdown, an omitted `VAF` no Early effect,
//! a given `TOX` derives `KP` and `GAMMA` from the process, a given `CBD`
//! replaces the area junction, and an omitted `IBV` is the dialect's own (1
//! mA in ngspice and Xyce, LTspice's 1e-10 A times AREA and M in the
//! default). Writing such a parameter at any value would make a different
//! device, so the designer adds it when the device has it, IBV with BV;
//! so are the geometry parameters of a resistor or capacitor model (`RSH`,
//! `CJ`), which make its value from the instance's length and width.

/// One parameter of a new card: its name as the card spells it, the value
/// the engine takes when the card omits it, its unit and what it is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelParameterDefault {
    pub name: &'static str,
    pub default: f64,
    pub unit: &'static str,
    pub description: &'static str,
}

/// One device type a new card can be: its `.MODEL` type, the model level the
/// card states when the type has more than one, what the type is called, and
/// its parameters at their defaults, in the order a designer reads them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceModelDefaults {
    pub spice_type: &'static str,
    pub level: Option<u32>,
    pub label: &'static str,
    pub parameters: &'static [ModelParameterDefault],
}

const fn p(
    name: &'static str,
    default: f64,
    unit: &'static str,
    description: &'static str,
) -> ModelParameterDefault {
    ModelParameterDefault {
        name,
        default,
        unit,
        description,
    }
}

const DIODE: &[ModelParameterDefault] = &[
    p("IS", 1e-14, "A", "saturation current"),
    p("N", 1.0, "", "emission coefficient"),
    p("RS", 0.0, "Ω", "ohmic resistance"),
    p("CJO", 0.0, "F", "zero-bias junction capacitance"),
    p("VJ", 1.0, "V", "junction potential"),
    p("M", 0.5, "", "grading coefficient"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
    p("TT", 0.0, "s", "transit time"),
    p("ISR", 0.0, "A", "recombination current"),
    p("NR", 2.0, "", "recombination emission coefficient"),
    p("EG", 1.11, "eV", "activation energy"),
    p("XTI", 3.0, "", "saturation-current temperature exponent"),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
];

const BJT: &[ModelParameterDefault] = &[
    p("IS", 1e-16, "A", "transport saturation current"),
    p("BF", 100.0, "", "ideal maximum forward beta"),
    p("NF", 1.0, "", "forward emission coefficient"),
    p("ISE", 0.0, "A", "base-emitter leakage saturation current"),
    p("NE", 1.5, "", "base-emitter leakage emission coefficient"),
    p("BR", 1.0, "", "ideal maximum reverse beta"),
    p("NR", 1.0, "", "reverse emission coefficient"),
    p("ISC", 0.0, "A", "base-collector leakage saturation current"),
    p("NC", 2.0, "", "base-collector leakage emission coefficient"),
    p("RB", 0.0, "Ω", "zero-bias base resistance"),
    p("RE", 0.0, "Ω", "emitter resistance"),
    p("RC", 0.0, "Ω", "collector resistance"),
    p(
        "CJE",
        0.0,
        "F",
        "base-emitter zero-bias depletion capacitance",
    ),
    p("VJE", 0.75, "V", "base-emitter built-in potential"),
    p("MJE", 0.33, "", "base-emitter junction grading coefficient"),
    p("TF", 0.0, "s", "ideal forward transit time"),
    p("XTF", 0.0, "", "transit-time bias coefficient"),
    p("ITF", 0.0, "A", "transit-time high-current coefficient"),
    p(
        "CJC",
        0.0,
        "F",
        "base-collector zero-bias depletion capacitance",
    ),
    p("VJC", 0.75, "V", "base-collector built-in potential"),
    p(
        "MJC",
        0.33,
        "",
        "base-collector junction grading coefficient",
    ),
    p("XCJC", 1.0, "", "fraction of CJC at the internal base"),
    p("TR", 0.0, "s", "ideal reverse transit time"),
    p("CJS", 0.0, "F", "substrate zero-bias capacitance"),
    p("VJS", 0.75, "V", "substrate junction built-in potential"),
    p("MJS", 0.0, "", "substrate junction grading coefficient"),
    p(
        "XTB",
        0.0,
        "",
        "forward and reverse beta temperature exponent",
    ),
    p("XTI", 3.0, "", "saturation-current temperature exponent"),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
];

const MOS1: &[ModelParameterDefault] = &[
    p("VTO", 0.0, "V", "zero-bias threshold voltage"),
    p("KP", 2e-5, "A/V²", "transconductance parameter"),
    p("GAMMA", 0.0, "√V", "bulk threshold parameter"),
    p("PHI", 0.6, "V", "surface potential"),
    p("RD", 0.0, "Ω", "drain ohmic resistance"),
    p("RS", 0.0, "Ω", "source ohmic resistance"),
    p("IS", 1e-14, "A", "bulk junction saturation current"),
    p("PB", 0.8, "V", "bulk junction potential"),
    p(
        "CGSO",
        0.0,
        "F/m",
        "gate-source overlap capacitance per channel width",
    ),
    p(
        "CGDO",
        0.0,
        "F/m",
        "gate-drain overlap capacitance per channel width",
    ),
    p(
        "CGBO",
        0.0,
        "F/m",
        "gate-bulk overlap capacitance per channel length",
    ),
    p(
        "RSH",
        0.0,
        "Ω/□",
        "drain and source diffusion sheet resistance",
    ),
    p(
        "CJ",
        0.0,
        "F/m²",
        "zero-bias bulk junction bottom capacitance",
    ),
    p("MJ", 0.5, "", "bulk junction bottom grading coefficient"),
    p(
        "CJSW",
        0.0,
        "F/m",
        "zero-bias bulk junction sidewall capacitance",
    ),
    p(
        "MJSW",
        0.33,
        "",
        "bulk junction sidewall grading coefficient",
    ),
    p(
        "JS",
        0.0,
        "A/m²",
        "bulk junction saturation current density",
    ),
    p("LD", 0.0, "m", "lateral diffusion"),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
];

const JFET: &[ModelParameterDefault] = &[
    p("VTO", -2.0, "V", "threshold voltage"),
    p("BETA", 1e-4, "A/V²", "transconductance parameter"),
    p("LAMBDA", 0.0, "1/V", "channel-length modulation"),
    p("RD", 0.0, "Ω", "drain ohmic resistance"),
    p("RS", 0.0, "Ω", "source ohmic resistance"),
    p(
        "CGS",
        0.0,
        "F",
        "zero-bias gate-source junction capacitance",
    ),
    p("CGD", 0.0, "F", "zero-bias gate-drain junction capacitance"),
    p("PB", 1.0, "V", "gate junction potential"),
    p("IS", 1e-14, "A", "gate junction saturation current"),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
];

const N_MESFET: &[ModelParameterDefault] = &[
    p("VTO", -2.0, "V", "pinch-off voltage"),
    MESFET[0],
    MESFET[1],
    MESFET[2],
    MESFET[3],
    MESFET[4],
    MESFET[5],
    MESFET[6],
    MESFET[7],
    MESFET[8],
    MESFET[9],
    MESFET[10],
    MESFET[11],
];

const P_MESFET: &[ModelParameterDefault] = &[
    p("VTO", 2.0, "V", "pinch-off voltage"),
    MESFET[0],
    MESFET[1],
    MESFET[2],
    MESFET[3],
    MESFET[4],
    MESFET[5],
    MESFET[6],
    MESFET[7],
    MESFET[8],
    MESFET[9],
    MESFET[10],
    MESFET[11],
];

/// The MESFET's parameters after its pinch-off voltage, whose sign is the
/// channel's.
const MESFET: &[ModelParameterDefault] = &[
    p("BETA", 2.5e-3, "A/V²", "transconductance parameter"),
    p("B", 0.3, "1/V", "doping tail extending parameter"),
    p("ALPHA", 2.0, "1/V", "saturation voltage parameter"),
    p("LAMBDA", 0.0, "1/V", "channel-length modulation"),
    p("RD", 0.0, "Ω", "drain ohmic resistance"),
    p("RS", 0.0, "Ω", "source ohmic resistance"),
    p(
        "CGS",
        0.0,
        "F",
        "zero-bias gate-source junction capacitance",
    ),
    p("CGD", 0.0, "F", "zero-bias gate-drain junction capacitance"),
    p("PB", 1.0, "V", "gate junction potential"),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
];

const VDMOS: &[ModelParameterDefault] = &[
    p("VTO", 0.0, "V", "zero-bias threshold voltage"),
    p("KP", 1.0, "A/V²", "transconductance parameter"),
    p("PHI", 0.6, "V", "surface potential"),
    p("LAMBDA", 0.0, "1/V", "channel-length modulation"),
    p("THETA", 0.0, "1/V", "mobility modulation"),
    p("RD", 0.0, "Ω", "drain ohmic resistance"),
    p("RS", 0.0, "Ω", "source ohmic resistance"),
    p("RG", 0.0, "Ω", "gate ohmic resistance"),
    p("IS", 1e-14, "A", "body diode saturation current"),
    p("N", 1.0, "", "body diode emission coefficient"),
    p("RB", 0.0, "Ω", "body diode ohmic resistance"),
    p("CJO", 0.0, "F", "body diode zero-bias junction capacitance"),
    p("VJ", 1.0, "V", "body diode junction potential"),
    p("M", 0.5, "", "body diode grading coefficient"),
    p(
        "FC",
        0.5,
        "",
        "forward-bias depletion capacitance coefficient",
    ),
    p("TT", 0.0, "s", "body diode transit time"),
    p("CGDMAX", 0.0, "F", "maximum gate-drain capacitance"),
    p("CGDMIN", 0.0, "F", "minimum gate-drain capacitance"),
    p("A", 1.0, "", "gate-drain capacitance non-linearity"),
    p("CGS", 0.0, "F", "gate-source capacitance"),
    p("EG", 1.11, "eV", "body diode activation energy"),
    p(
        "XTI",
        3.0,
        "",
        "body diode saturation-current temperature exponent",
    ),
    p("KF", 0.0, "", "flicker-noise coefficient"),
    p("AF", 1.0, "", "flicker-noise exponent"),
];

const RESISTOR: &[ModelParameterDefault] = &[
    p("TC1", 0.0, "1/°C", "first-order temperature coefficient"),
    p("TC2", 0.0, "1/°C²", "second-order temperature coefficient"),
];

const CAPACITOR: &[ModelParameterDefault] = &[
    p("TC1", 0.0, "1/°C", "first-order temperature coefficient"),
    p("TC2", 0.0, "1/°C²", "second-order temperature coefficient"),
];

const INDUCTOR: &[ModelParameterDefault] = &[
    p("TC1", 0.0, "1/°C", "first-order temperature coefficient"),
    p("TC2", 0.0, "1/°C²", "second-order temperature coefficient"),
];

const VOLTAGE_SWITCH: &[ModelParameterDefault] = &[
    p("VT", 0.0, "V", "threshold voltage"),
    p("VH", 0.0, "V", "hysteresis voltage"),
    p("RON", 1.0, "Ω", "on resistance"),
    p("ROFF", 1e12, "Ω", "off resistance"),
];

const CURRENT_SWITCH: &[ModelParameterDefault] = &[
    p("IT", 0.0, "A", "threshold current"),
    p("IH", 0.0, "A", "hysteresis current"),
    p("RON", 1.0, "Ω", "on resistance"),
    p("ROFF", 1e12, "Ω", "off resistance"),
];

/// Every device type a new card can be, in the order the New model form
/// lists them.
pub const DEVICE_MODEL_DEFAULTS: &[DeviceModelDefaults] = &[
    DeviceModelDefaults {
        spice_type: "D",
        level: None,
        label: "Diode",
        parameters: DIODE,
    },
    DeviceModelDefaults {
        spice_type: "NPN",
        level: None,
        label: "NPN bipolar (Gummel-Poon)",
        parameters: BJT,
    },
    DeviceModelDefaults {
        spice_type: "PNP",
        level: None,
        label: "PNP bipolar (Gummel-Poon)",
        parameters: BJT,
    },
    DeviceModelDefaults {
        spice_type: "NMOS",
        level: Some(1),
        label: "N-channel MOSFET (level 1)",
        parameters: MOS1,
    },
    DeviceModelDefaults {
        spice_type: "PMOS",
        level: Some(1),
        label: "P-channel MOSFET (level 1)",
        parameters: MOS1,
    },
    DeviceModelDefaults {
        spice_type: "NJF",
        level: None,
        label: "N-channel JFET",
        parameters: JFET,
    },
    DeviceModelDefaults {
        spice_type: "PJF",
        level: None,
        label: "P-channel JFET",
        parameters: JFET,
    },
    DeviceModelDefaults {
        spice_type: "NMF",
        level: None,
        label: "N-channel MESFET (Statz)",
        parameters: N_MESFET,
    },
    DeviceModelDefaults {
        spice_type: "PMF",
        level: None,
        label: "P-channel MESFET (Statz)",
        parameters: P_MESFET,
    },
    DeviceModelDefaults {
        spice_type: "VDMOS",
        level: None,
        label: "Vertical power MOSFET",
        parameters: VDMOS,
    },
    DeviceModelDefaults {
        spice_type: "R",
        level: None,
        label: "Resistor",
        parameters: RESISTOR,
    },
    DeviceModelDefaults {
        spice_type: "C",
        level: None,
        label: "Capacitor",
        parameters: CAPACITOR,
    },
    DeviceModelDefaults {
        spice_type: "L",
        level: None,
        label: "Inductor",
        parameters: INDUCTOR,
    },
    DeviceModelDefaults {
        spice_type: "SW",
        level: None,
        label: "Voltage-controlled switch",
        parameters: VOLTAGE_SWITCH,
    },
    DeviceModelDefaults {
        spice_type: "CSW",
        level: None,
        label: "Current-controlled switch",
        parameters: CURRENT_SWITCH,
    },
];

/// The defaults of the device type `spice_type` names, compared as SPICE
/// compares names.
#[must_use]
pub fn device_model_defaults(spice_type: &str) -> Option<&'static DeviceModelDefaults> {
    DEVICE_MODEL_DEFAULTS
        .iter()
        .find(|defaults| defaults.spice_type.eq_ignore_ascii_case(spice_type))
}

impl DeviceModelDefaults {
    /// The bare card of this type for `name`: the level when the type has
    /// more than one, and no parameter, so the engine takes every default.
    #[must_use]
    pub fn bare_card(&self, name: &str) -> String {
        match self.level {
            Some(level) => format!(".model {name} {} (LEVEL={level})", self.spice_type),
            None => format!(".model {name} {}", self.spice_type),
        }
    }

    /// The card of this type for `name` with `parameters` written out, the
    /// level first when the type states one.
    #[must_use]
    pub fn card_with<'a>(
        &self,
        name: &str,
        parameters: impl IntoIterator<Item = &'a ModelParameterDefault>,
    ) -> String {
        let mut card = format!(".model {name} {} (", self.spice_type);
        if let Some(level) = self.level {
            card.push_str(&format!(" LEVEL={level}"));
        }
        for parameter in parameters {
            card.push_str(&format!(" {}={}", parameter.name, parameter.default));
        }
        card.push_str(" )");
        card
    }
}
