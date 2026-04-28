pub const TAB_DC_OP: usize = 0;
pub const TAB_TRANSIENT: usize = 1;
pub const TAB_AC: usize = 2;
pub const TAB_DC_SWEEP: usize = 3;
pub const TAB_NOISE: usize = 4;
pub const TAB_POLE_ZERO: usize = 5;
pub const TAB_SENSITIVITY: usize = 6;
pub const TAB_MONTE_CARLO: usize = 7;
pub const TAB_PSS: usize = 8;
pub const TAB_STB: usize = 9;
pub const TAB_TEMPERATURE: usize = 10;
pub const TAB_HARMONIC_BALANCE: usize = 11;
pub const TAB_SPARAMETER: usize = 12;
pub const TAB_PAC: usize = 13;
pub const TAB_PNOISE: usize = 14;
pub const TAB_PXF: usize = 15;
pub const TAB_PSTB: usize = 16;
pub const TAB_TRANSFER_FUNCTION: usize = 17;
pub const TAB_CORNER: usize = 18;
pub const TAB_ENVELOPE: usize = 19;
pub const TAB_FOURIER: usize = 20;
pub const TAB_RELIABILITY: usize = 21;
pub const TAB_OPTIMIZATION: usize = 22;
pub const TAB_SOA: usize = 23;
pub const TAB_DISTO: usize = 24;

pub type AnalysisCategory = (&'static str, &'static [(usize, &'static str)]);

pub const SIMULATION_ANALYSIS_CATEGORIES: &[AnalysisCategory] = &[
    (
        "Time & Frequency Domain",
        &[
            (TAB_TRANSIENT, "Transient"),
            (TAB_AC, "AC Analysis"),
            (TAB_DC_SWEEP, "DC Sweep"),
            (TAB_DC_OP, "DC Operating Point"),
            (TAB_NOISE, "Noise"),
        ],
    ),
    (
        "Steady-State",
        &[
            (TAB_PSS, "PSS (Periodic)"),
            (TAB_HARMONIC_BALANCE, "Harmonic Balance"),
        ],
    ),
    (
        "Periodic Small-Signal",
        &[
            (TAB_PAC, "PAC"),
            (TAB_PNOISE, "PNoise"),
            (TAB_PXF, "PXF"),
            (TAB_PSTB, "PSTB"),
        ],
    ),
    (
        "Transfer & Stability",
        &[
            (TAB_POLE_ZERO, "Pole-Zero"),
            (TAB_SENSITIVITY, "Sensitivity"),
            (TAB_STB, "Stability (STB)"),
            (TAB_TRANSFER_FUNCTION, "Transfer Func (XF)"),
        ],
    ),
    ("RF & S-Parameters", &[(TAB_SPARAMETER, "S-Parameter")]),
    (
        "Statistical & Sweep",
        &[
            (TAB_MONTE_CARLO, "Monte Carlo"),
            (TAB_TEMPERATURE, "Temperature"),
            (TAB_CORNER, "Corner"),
            (TAB_ENVELOPE, "Envelope"),
            (TAB_FOURIER, "Fourier"),
        ],
    ),
    (
        "Advanced",
        &[
            (TAB_RELIABILITY, "Reliability"),
            (TAB_OPTIMIZATION, "Optimization"),
            (TAB_SOA, "Safety (SOA)"),
            (TAB_DISTO, "DISTO"),
        ],
    ),
];

pub const QUICK_RUN_ANALYSES: &[(&str, usize)] = &[
    ("DC Operating Point", TAB_DC_OP),
    ("Transient", TAB_TRANSIENT),
    ("AC Analysis", TAB_AC),
    ("DISTO", TAB_DISTO),
    ("DC Sweep", TAB_DC_SWEEP),
    ("Noise", TAB_NOISE),
    ("Pole-Zero", TAB_POLE_ZERO),
    ("Sensitivity", TAB_SENSITIVITY),
    ("Monte Carlo", TAB_MONTE_CARLO),
    ("PSS", TAB_PSS),
    ("Stability (STB)", TAB_STB),
    ("Temperature Sweep", TAB_TEMPERATURE),
    ("Reliability (Aging)", TAB_RELIABILITY),
];
