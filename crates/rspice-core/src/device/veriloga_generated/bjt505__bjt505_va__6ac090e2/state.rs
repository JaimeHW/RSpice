#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub p0: f64, pub p1: f64, pub p2: f64, pub p3: f64, pub p4: f64, pub p5: f64, pub p6: f64, pub p7: f64,
    pub p8: f64, pub p9: f64, pub p10: f64, pub p11: f64, pub p12: f64, pub p13: f64, pub p14: f64, pub p15: f64,
    pub p16: f64, pub p17: f64, pub p18: f64, pub p19: f64, pub p20: f64, pub p21: f64, pub p22: f64, pub p23: f64,
    pub p24: f64, pub p25: f64, pub p26: f64, pub p27: f64, pub p28: f64, pub p29: f64, pub p30: f64, pub p31: f64,
    pub p32: f64, pub p33: f64, pub p34: f64, pub p35: f64, pub p36: f64, pub p37: f64, pub p38: f64, pub p39: f64,
    pub p40: f64, pub p41: f64, pub p42: f64, pub p43: f64, pub p44: f64, pub p45: f64, pub p46: f64, pub p47: f64,
    pub p48: f64, pub p49: f64, pub p50: f64, pub p51: f64, pub p52: f64, pub p53: f64, pub p54: f64, pub p55: f64,
    pub p56: f64, pub p57: f64, pub p58: f64, pub p59: f64, pub p60: f64, pub p61: f64, pub p62: f64, pub p63: f64,
    pub p64: f64, pub p65: f64, pub p66: f64, pub p67: f64, pub p68: f64, pub p69: f64, pub p70: f64, pub p71: f64,
    pub p72: f64, pub p73: f64, pub p74: f64, pub p75: f64, pub p76: f64, pub p77: f64, pub p78: f64, pub p79: f64,
    pub p80: f64, pub p81: f64, pub p82: f64, pub p83: f64, pub p84: f64, pub p85: f64, pub p86: f64, pub p87: f64,
    pub p88: f64, pub p89: f64, pub p90: f64, pub p91: f64, pub p92: f64, pub p93: f64, pub p94: f64, pub p95: f64,
    pub p96: f64, pub p97: f64, pub p98: f64, pub p99: f64, pub p100: f64, pub p101: f64, pub p102: f64, pub p103: f64,
    pub p104: f64, pub p105: f64, pub p106: f64, pub p107: f64, pub p108: f64, pub p109: f64, pub p110: f64, pub p111: f64,
    pub p112: f64, pub p113: f64, pub p114: f64, pub p115: f64, pub p116: f64, pub p117: f64, pub p118: f64, pub p119: f64,
    pub p120: f64, pub p121: f64, pub p122: f64, pub p123: f64, pub p124: f64, pub p125: f64, pub p126: f64, pub p127: f64,
    pub p128: f64, pub p129: f64, pub p130: f64, pub p131: f64, pub p132: f64, pub p133: f64, pub p134: f64, pub p135: f64,
    pub p136: f64, pub p137: f64, pub p138: f64, pub p139: f64, pub p140: f64, pub p141: f64, pub p142: f64, pub p143: f64,
    pub p144: f64, pub p145: f64, pub p146: f64, pub p147: f64, pub p148: f64, pub p149: f64, pub p150: f64, pub p151: f64,
    pub p152: f64, pub p153: f64, pub p154: f64, pub p155: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 150] = [
                0.0, 1.0, 505.5, 1.0, 25.0, 1.0, 1.0, 0.0,
                1.0, 2.2e-17, 1.0, 1.0, 0.1, 2.5, 44.0, 1.0,
                1.0000000000000001e-19, 1.0, 0.0, 1.0, 2.7000000000000005e-15, 2.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.68, 0.0, 3.1400000000000002e-18, 0.014289999999999999, 1e-15,
                2.0, 0.63, 0.0, 22.0, 0.0, 22.0, 1e-6, 1.0,
                400.0, -0.37, 0.5, 25.0, 0.1, 1.1e-6, 3.0, 0.3,
                0.004, -0.37, -0.37, 0.3, 0.004, 1.0, 5.0, 23.0,
                18.0, 12.0, 0.0, 0.0, 150.0, 1250.0, 0.004, 0.3,
                0.68, 7.3e-14, 0.95, 0.4, 0.4, 0.0, 7.800000000000001e-14, 0.68,
                0.5, 0.0, 0.0, 0.35, 0.5, 0.032, 0.0, 0.0,
                0.68, 100.0, 4.0, 1000.0, 0.0, 1.0, 2e-12, 4.2e-12,
                4.1e-11, 5.2e-10, 1e-11, 1.0, 0.0, 0.0, 0.3333333333333333, 0.0,
                0.3, 0.0, 1.0, 2.5, 2.5, 0.62, 2.0, 1.3,
                2.0, 1.17, 1.12, 1.12, 1.12, 1.12, 1.18, 1.12,
                1.125, 1.15, 1.15, 0.000473, 636.0, 1.15, 0.000473, 636.0,
                0.05, 0.0, 0.0, 0.0, 0.0005, 200.0, 2.0, 2.0,
                2e-11, 2e-11, 0.0, 0.0, 0.0, 4.8000000000000003e-17, 0.0, 0.0005455,
                4.9999999999999996e-5, 3.15e-13, 0.62, 0.34, 1.2, 1.58, 2.0, 0.0,
                0.0, 0.0, 2.0, 400.0, 1e-40, 1e-40,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 150);
            {
                let params = &mut *ptr;
                params.p150 = 0.001;
                validate_parameter("minr", params.p150, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 0.0, 0.16, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(151), 5);
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
}

#[derive(Copy, Clone)]
struct ParameterBound {
    value: f64,
    label: &'static str,
}

const PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
const PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

fn validate_parameter_metadata(index: usize, value: f64) -> Result<(), String> {
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some(min) = PARAMETER_MIN_BOUNDS[index] {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = PARAMETER_MAX_BOUNDS[index] {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in PARAMETER_EXCLUDED_BOUNDS[index] {
        if value == excluded.value {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
        }
    }
    Ok(())
}

fn validate_finite_parameter(name: &str, value: f64) -> Result<(), String> {
    if !value.is_finite() {
        return Err(format!("parameter '{}' must be finite, got {}", name, value));
    }
    Ok(())
}

fn validate_parameter(
    name: &str,
    value: f64,
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some((min, label)) = min {
        if min_exclusive {
            if value <= min {
                return Err(format!("parameter '{}' must be > {}, got {}", name, label, value));
            }
        } else if value < min {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, label, value));
        }
    }
    if let Some((max, label)) = max {
        if max_exclusive {
            if value >= max {
                return Err(format!("parameter '{}' must be < {}, got {}", name, label, value));
            }
        } else if value > max {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, label, value));
        }
    }
    for (excluded, label) in excluded {
        if value == *excluded {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, label, value));
        }
    }
    Ok(())
}
const PARAMETER_NAME_LOOKUP: [(&str, usize); 158] = [
    ("dta", 0), ("trise", 0), ("dtemp", 0), ("mult", 1), ("version", 2), ("type", 3), ("tref", 4), ("exmod", 5), ("exphi", 6), ("exavl", 7), ("exsub", 8), ("is", 9), ("nff", 10), ("nfr", 11), ("ik", 12), ("ver", 13),
    ("vef", 14), ("issr", 15), ("ibi", 16), ("nbi", 17), ("ibis", 18), ("nbis", 19), ("ibf", 20), ("mlf", 21), ("ibfs", 22), ("mlfs", 23), ("swib1", 24), ("ibinbr", 25), ("ibinbrs", 26), ("vknbr", 27), ("ibinbrqs", 28), ("ibx", 29),
    ("ikbx", 30), ("ibr", 31), ("mlr", 32), ("xext", 33), ("izeb", 34), ("nzeb", 35), ("izcb", 36), ("nzcb", 37), ("vzmin", 38), ("swavl", 39), ("aavl", 40), ("cavl", 41), ("itoavl", 42), ("bavl", 43), ("vdcavl", 44), ("wavl", 45),
    ("vavl", 46), ("sfh", 47), ("ihcavl", 48), ("davl", 49), ("eavl", 50), ("aexavl", 51), ("ionexavl", 52), ("swgemlim", 53), ("re", 54), ("rbc", 55), ("rbv", 56), ("rcc", 57), ("rcblx", 58), ("rcbli", 59), ("rcv", 60), ("scrcv", 61),
    ("ihc", 62), ("axi", 63), ("vdc", 64), ("cje", 65), ("vde", 66), ("pe", 67), ("xcje", 68), ("cbeo", 69), ("cjc", 70), ("vdcctc", 71), ("pc", 72), ("swvchc", 73), ("swvjunc", 74), ("xp", 75), ("mc", 76), ("xcjc", 77),
    ("cbco", 78), ("swqex", 79), ("vdcex", 80), ("vbrcb", 81), ("pbrcb", 82), ("frevcb", 83), ("swjbrcb", 84), ("mtau", 85), ("taue", 86), ("taub", 87), ("tepi", 88), ("taur", 89), ("tauex", 90), ("nex", 91), ("deg", 92), ("xrec", 93),
    ("xqb", 94), ("ke", 95), ("aqbo", 96), ("ae", 97), ("ab", 98), ("aepi", 99), ("aepiex", 100), ("aex", 101), ("ac", 102), ("acx", 103), ("acbl", 104), ("vgb", 105), ("vgbnbrqs", 106), ("vgbnbr", 107), ("vgbnbrs", 108), ("vgknbr", 109),
    ("vgc", 110), ("vge", 111), ("vgcx", 112), ("vgj", 113), ("vgzeb", 114), ("avgeb", 115), ("tvgeb", 116), ("vgzcb", 117), ("avgcb", 118), ("tvgcb", 119), ("dvgte", 120), ("dais", 121), ("tnff", 122), ("tnfr", 123), ("tbavl", 124), ("dtmax", 125),
    ("af", 126), ("afn", 127), ("kf", 128), ("kfn", 129), ("kavl", 130), ("kc", 131), ("ftaun", 132), ("iss", 133), ("icss", 134), ("iks", 135), ("ikcs", 136), ("cjs", 137), ("vds", 138), ("ps", 139), ("vgs", 140), ("as", 141),
    ("asub", 142), ("xisubi", 143), ("swvsch", 144), ("isibrel", 145), ("nfibrel", 146), ("vexlim", 147), ("p0starlim", 148), ("pwlim", 149), ("minr", 150), ("istat", 151), ("vtat", 152), ("ktat", 153), ("vbtbt", 154), ("kbtbt", 155),
];

const PARAMETER_DISPLAY_NAMES: [&str; 156] = [
    "dta", "mult", "version", "type", "tref", "exmod", "exphi", "exavl", "exsub", "is", "nff", "nfr", "ik", "ver", "vef", "issr",
    "ibi", "nbi", "ibis", "nbis", "ibf", "mlf", "ibfs", "mlfs", "swib1", "ibinbr", "ibinbrs", "vknbr", "ibinbrqs", "ibx", "ikbx", "ibr",
    "mlr", "xext", "izeb", "nzeb", "izcb", "nzcb", "vzmin", "swavl", "aavl", "cavl", "itoavl", "bavl", "vdcavl", "wavl", "vavl", "sfh",
    "ihcavl", "davl", "eavl", "aexavl", "ionexavl", "swgemlim", "re", "rbc", "rbv", "rcc", "rcblx", "rcbli", "rcv", "scrcv", "ihc", "axi",
    "vdc", "cje", "vde", "pe", "xcje", "cbeo", "cjc", "vdcctc", "pc", "swvchc", "swvjunc", "xp", "mc", "xcjc", "cbco", "swqex",
    "vdcex", "vbrcb", "pbrcb", "frevcb", "swjbrcb", "mtau", "taue", "taub", "tepi", "taur", "tauex", "nex", "deg", "xrec", "xqb", "ke",
    "aqbo", "ae", "ab", "aepi", "aepiex", "aex", "ac", "acx", "acbl", "vgb", "vgbnbrqs", "vgbnbr", "vgbnbrs", "vgknbr", "vgc", "vge",
    "vgcx", "vgj", "vgzeb", "avgeb", "tvgeb", "vgzcb", "avgcb", "tvgcb", "dvgte", "dais", "tnff", "tnfr", "tbavl", "dtmax", "af", "afn",
    "kf", "kfn", "kavl", "kc", "ftaun", "iss", "icss", "iks", "ikcs", "cjs", "vds", "ps", "vgs", "as", "asub", "xisubi",
    "swvsch", "isibrel", "nfibrel", "vexlim", "p0starlim", "pwlim", "minr", "istat", "vtat", "ktat", "vbtbt", "kbtbt",
];

const PARAMETER_INTEGER_FLAGS: [bool; 156] = [
    false, false, false, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 156] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 505.5, label: "505.5" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1e-12, label: "1e-12" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }),
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.02, label: "0.02" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }),
    Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 40.0, label: "40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 156] = [
    None, None, Some(ParameterBound { value: 505.51, label: "505.51" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, Some(ParameterBound { value: 2000.0, label: "2000.0" }), Some(ParameterBound { value: 500.0, label: "500.0" }), Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 400.0, label: "400.0" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), None, None,
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 156] = [
    0, 3, 2, 0, 2, 0, 0, 0, 0, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 2, 2,
    2, 0, 2, 2, 2, 2, 3, 0, 2, 3, 3, 3, 0, 2, 2, 2, 2, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 2, 0, 2, 1, 1, 1, 0, 2, 2, 3, 2, 2, 2, 2, 0, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2,
    2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 2, 0, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 3, 0, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 156] = [
    &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
];

fn parameter_index_for_name(name: &str) -> Option<usize> {
    PARAMETER_NAME_LOOKUP
        .iter()
        .find_map(|(candidate, index)| (*candidate == name).then_some(*index))
}

fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {
    let mut boxed = Box::<[f64; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {
    let mut boxed = Box::<[bool; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

pub struct Instance {
    pub nodes: [usize; 12],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 156]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 10]>,
    pub(crate) ddt_state_previous: Box<[f64; 10]>,
    pub(crate) ddt_state_older: Box<[f64; 10]>,
    pub(crate) ddt_state_initialized: Box<[bool; 10]>,
    pub(crate) ddt_derivative_current: Box<[f64; 10]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 10]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 1039]>,
    pub(crate) scalar_static_bool: Box<[bool; 116]>,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes,
            branches: self.branches,
            params: self.params.clone(),
            param_given: self.param_given.clone(),
            multiplicity: self.multiplicity,
            ddt_state_current: self.ddt_state_current.clone(),
            ddt_state_previous: self.ddt_state_previous.clone(),
            ddt_state_older: self.ddt_state_older.clone(),
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            ddt_derivative_current: self.ddt_derivative_current.clone(),
            ddt_derivative_previous: self.ddt_derivative_previous.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
            ddt_coefficients: self.ddt_coefficients,
            scalar_static_f64: self.scalar_static_f64.clone(),
            scalar_static_bool: self.scalar_static_bool.clone(),
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 156;
    pub const VARIABLE_COUNT: usize = 616;
    pub const DDT_STATE_COUNT: usize = 10;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        let mut instance = Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            ddt_state_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_older: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            ddt_coefficients: GeneratedDdtCoefficients::inactive(),
            scalar_static_f64: boxed_zero_f64_array::<1039>(),
            scalar_static_bool: boxed_zero_bool_array::<116>(),
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
        };
        instance.recompute_instance_static();
        instance
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
            scalar_static_f64,
            scalar_static_bool,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
        } = snapshot;
        *self = Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
            scalar_static_f64,
            scalar_static_bool,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjt505_va'", name));
        };
        validate_parameter_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
        Ok(())
    }

    #[inline]
    fn write_parameter_slot(&mut self, index: usize, value: f64) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        // SAFETY: Parameters is repr(C), contains only f64 fields, and index is produced from generated parameter metadata.
        unsafe {
            let ptr = self.params.as_mut() as *mut Parameters as *mut f64;
            *ptr.add(index) = value;
        }
    }

    #[inline]
    fn finish_set_parameter(&mut self, index: usize) {
        self.mark_param_given(index);
        self.recompute_instance_static();
        self.invalidate_temperature_static();
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: f64, timestep: f64, ddt_coefficients: GeneratedDdtCoefficients) {
        self.time = time;
        self.timestep = timestep;
        self.ddt_coefficients = ddt_coefficients;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_older[index] = self.ddt_state_previous[index];
            self.ddt_state_previous[index] = self.ddt_state_current[index];
            self.ddt_derivative_previous[index] = self.ddt_derivative_current[index];
            self.ddt_state_initialized[index] = true;
            index += 1;
        }
        let mut index = 0usize;
        while index < Self::IDT_STATE_COUNT {
            self.idt_state_previous[index] = self.idt_state_current[index];
            self.idt_state_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.ddt_state_initialized[slot] {
            self.ddt_state_previous[slot]
        } else {
            value
        };
        let older = if self.ddt_state_initialized[slot] {
            self.ddt_state_older[slot]
        } else {
            value
        };
        self.ddt_state_current[slot] = value;
        if self.ddt_coefficients.active {
            let result = value * self.ddt_coefficients.derivative_scale
                - previous * self.ddt_coefficients.previous_value_scale
                - older * self.ddt_coefficients.older_value_scale
                - self.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;
            self.ddt_derivative_current[slot] = result;
            result
        } else {
            self.ddt_state_current[slot] = value;
            self.ddt_state_previous[slot] = value;
            self.ddt_state_older[slot] = value;
            self.ddt_derivative_current[slot] = 0.0;
            self.ddt_derivative_previous[slot] = 0.0;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.ddt_coefficients.active {
            derivative * self.ddt_coefficients.derivative_scale
        } else {
            0.0
        }
    }

    #[inline]
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        self.scalar_static_f64[0]=p.p3;
        self.scalar_static_bool[0]=(self.scalar_static_f64[0]==1.0);
        self.scalar_static_f64[1]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[2]=(if (self.scalar_static_f64[1]!=0.0){70300000.0}else{0.0});
        self.scalar_static_f64[3]=(if (self.scalar_static_f64[1]!=0.0){123000000.0}else{0.0});
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[1]!=0.0));
        self.scalar_static_f64[4]=(if self.scalar_static_bool[1]{158000000.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[5]=(if self.scalar_static_bool[1]{204000000.0}else{self.scalar_static_f64[3]});
        self.scalar_static_f64[6]=p.p33;
        self.scalar_static_f64[7]=(1.0-self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=p.p4;
        self.scalar_static_f64[9]=(self.scalar_static_f64[8]+273.15);
        self.scalar_static_f64[10]=p.p0;
        self.scalar_static_f64[11]=p.p150;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[11]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[13]=(if (self.scalar_static_f64[12]!=0.0){1e-12}else{0.0});
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[12]!=0.0));
        self.scalar_static_f64[14]=(if self.scalar_static_bool[3]{self.scalar_static_f64[11]}else{self.scalar_static_f64[13]});
        self.scalar_static_f64[15]=p.p1;
        self.scalar_static_f64[16]=(self.scalar_static_f64[14]*self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=(1.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=p.p134;
        self.scalar_static_f64[19]=p.p67;
        self.scalar_static_f64[20]=(2.0-self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=f64::powf(2.0,self.scalar_static_f64[20]);
        self.scalar_static_f64[22]=(1.0/self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=p.p114;
        self.scalar_static_f64[24]=p.p115;
        self.scalar_static_f64[25]=(self.scalar_static_f64[9]*self.scalar_static_f64[24]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[9]*self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=p.p116;
        self.scalar_static_f64[28]=(self.scalar_static_f64[9]+self.scalar_static_f64[27]);
        self.scalar_static_f64[29]=(self.scalar_static_f64[26]/self.scalar_static_f64[28]);
        self.scalar_static_f64[30]=(self.scalar_static_f64[23]+self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=(self.scalar_static_f64[30]-0.05);
        self.scalar_static_f64[32]=(self.scalar_static_f64[31]/0.1);
        self.scalar_static_bool[4]=(self.scalar_static_f64[30]<0.05);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[34]=(self.scalar_static_f64[32]).exp();
        self.scalar_static_f64[35]=(1.0+self.scalar_static_f64[34]);
        self.scalar_static_f64[36]=(self.scalar_static_f64[35]).ln();
        self.scalar_static_f64[37]=(0.1*self.scalar_static_f64[36]);
        self.scalar_static_f64[38]=(0.05+self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=(if (self.scalar_static_f64[33]!=0.0){self.scalar_static_f64[38]}else{0.0});
        self.scalar_static_bool[5]=(!(self.scalar_static_f64[33]!=0.0));
        self.scalar_static_f64[40]=(-self.scalar_static_f64[32]);
        self.scalar_static_f64[41]=(self.scalar_static_f64[40]).exp();
        self.scalar_static_f64[42]=(1.0+self.scalar_static_f64[41]);
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]).ln();
        self.scalar_static_f64[44]=(0.1*self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=(self.scalar_static_f64[30]+self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=(if self.scalar_static_bool[5]{self.scalar_static_f64[45]}else{self.scalar_static_f64[39]});
        self.scalar_static_f64[47]=(1.0/self.scalar_static_f64[23]);
        self.scalar_static_f64[48]=p.p66;
        self.scalar_static_f64[49]=(1.0/self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=p.p71;
        self.scalar_static_f64[51]=p.p72;
        self.scalar_static_f64[52]=(2.0-self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=f64::powf(2.0,self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(1.0/self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=p.p117;
        self.scalar_static_f64[56]=p.p118;
        self.scalar_static_f64[57]=(self.scalar_static_f64[9]*self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=(self.scalar_static_f64[9]*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=p.p119;
        self.scalar_static_f64[60]=(self.scalar_static_f64[9]+self.scalar_static_f64[59]);
        self.scalar_static_f64[61]=(self.scalar_static_f64[58]/self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[55]+self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(self.scalar_static_f64[62]-0.05);
        self.scalar_static_f64[64]=(self.scalar_static_f64[63]/0.1);
        self.scalar_static_bool[6]=(self.scalar_static_f64[62]<0.05);
        self.scalar_static_f64[65]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_f64[66]=(self.scalar_static_f64[64]).exp();
        self.scalar_static_f64[67]=(1.0+self.scalar_static_f64[66]);
        self.scalar_static_f64[68]=(self.scalar_static_f64[67]).ln();
        self.scalar_static_f64[69]=(0.1*self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(0.05+self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=(if (self.scalar_static_f64[65]!=0.0){self.scalar_static_f64[70]}else{0.0});
        self.scalar_static_bool[7]=(!(self.scalar_static_f64[65]!=0.0));
        self.scalar_static_f64[72]=(-self.scalar_static_f64[64]);
        self.scalar_static_f64[73]=(self.scalar_static_f64[72]).exp();
        self.scalar_static_f64[74]=(1.0+self.scalar_static_f64[73]);
        self.scalar_static_f64[75]=(self.scalar_static_f64[74]).ln();
        self.scalar_static_f64[76]=(0.1*self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=(self.scalar_static_f64[62]+self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(if self.scalar_static_bool[7]{self.scalar_static_f64[77]}else{self.scalar_static_f64[71]});
        self.scalar_static_f64[79]=(1.0/self.scalar_static_f64[55]);
        self.scalar_static_f64[80]=(1.0/self.scalar_static_f64[50]);
        self.scalar_static_f64[81]=p.p83;
        self.scalar_static_f64[82]=(1.0/self.scalar_static_f64[81]);
        self.scalar_static_f64[83]=(1.0-self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(self.scalar_static_f64[9]*8.617086918058125e-5);
        self.scalar_static_f64[85]=(1.0/self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=p.p105;
        self.scalar_static_f64[87]=p.p64;
        self.scalar_static_f64[88]=p.p110;
        self.scalar_static_f64[89]=p.p80;
        self.scalar_static_f64[90]=p.p27;
        self.scalar_static_f64[91]=p.p109;
        self.scalar_static_f64[92]=p.p138;
        self.scalar_static_f64[93]=p.p140;
        self.scalar_static_f64[94]=p.p65;
        self.scalar_static_f64[95]=p.p137;
        self.scalar_static_f64[96]=p.p139;
        self.scalar_static_f64[97]=p.p75;
        self.scalar_static_f64[98]=(1.0-self.scalar_static_f64[97]);
        self.scalar_static_f64[99]=p.p70;
        self.scalar_static_f64[100]=p.p54;
        self.scalar_static_f64[101]=p.p97;
        self.scalar_static_f64[102]=p.p56;
        self.scalar_static_f64[103]=p.p98;
        self.scalar_static_f64[104]=p.p96;
        self.scalar_static_f64[105]=(self.scalar_static_f64[103]-self.scalar_static_f64[104]);
        self.scalar_static_f64[106]=p.p55;
        self.scalar_static_f64[107]=p.p101;
        self.scalar_static_f64[108]=p.p57;
        self.scalar_static_f64[109]=p.p102;
        self.scalar_static_f64[110]=p.p58;
        self.scalar_static_f64[111]=p.p104;
        self.scalar_static_f64[112]=p.p59;
        self.scalar_static_f64[113]=p.p60;
        self.scalar_static_f64[114]=p.p99;
        self.scalar_static_f64[115]=p.p122;
        self.scalar_static_bool[8]=(0.0!=self.scalar_static_f64[115]);
        self.scalar_static_f64[116]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[117]=p.p10;
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[116]!=0.0));
        self.scalar_static_f64[118]=p.p123;
        self.scalar_static_bool[10]=(0.0!=self.scalar_static_f64[118]);
        self.scalar_static_f64[119]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[120]=p.p11;
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[119]!=0.0));
        self.scalar_static_f64[121]=p.p43;
        self.scalar_static_f64[122]=p.p124;
        self.scalar_static_f64[123]=p.p9;
        self.scalar_static_f64[124]=(4.0-self.scalar_static_f64[103]);
        self.scalar_static_f64[125]=(self.scalar_static_f64[124]-self.scalar_static_f64[104]);
        self.scalar_static_f64[126]=p.p121;
        self.scalar_static_f64[127]=(self.scalar_static_f64[125]+self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=(-self.scalar_static_f64[86]);
        self.scalar_static_f64[129]=p.p12;
        self.scalar_static_f64[130]=(1.0-self.scalar_static_f64[103]);
        self.scalar_static_f64[131]=p.p30;
        self.scalar_static_f64[132]=p.p103;
        self.scalar_static_f64[133]=(1.0-self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=p.p20;
        self.scalar_static_f64[135]=p.p21;
        self.scalar_static_f64[136]=(2.0*self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(6.0-self.scalar_static_f64[136]);
        self.scalar_static_f64[138]=p.p113;
        self.scalar_static_f64[139]=(-self.scalar_static_f64[138]);
        self.scalar_static_f64[140]=p.p31;
        self.scalar_static_f64[141]=p.p32;
        self.scalar_static_f64[142]=(2.0*self.scalar_static_f64[141]);
        self.scalar_static_f64[143]=(6.0-self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(-self.scalar_static_f64[88]);
        self.scalar_static_f64[145]=p.p16;
        self.scalar_static_f64[146]=(4.0-self.scalar_static_f64[101]);
        self.scalar_static_f64[147]=(self.scalar_static_f64[126]+self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=p.p17;
        self.scalar_static_f64[149]=p.p111;
        self.scalar_static_f64[150]=(-self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=p.p18;
        self.scalar_static_f64[152]=p.p19;
        self.scalar_static_f64[153]=p.p24;
        self.scalar_static_bool[12]=(1.0==self.scalar_static_f64[153]);
        self.scalar_static_f64[154]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_f64[155]=p.p25;
        self.scalar_static_f64[156]=p.p107;
        self.scalar_static_f64[157]=(-self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=p.p28;
        self.scalar_static_f64[159]=p.p106;
        self.scalar_static_f64[160]=(-self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=p.p26;
        self.scalar_static_f64[162]=p.p108;
        self.scalar_static_f64[163]=(-self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=p.p29;
        self.scalar_static_f64[165]=(4.0-self.scalar_static_f64[132]);
        self.scalar_static_f64[166]=(self.scalar_static_f64[126]+self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=p.p112;
        self.scalar_static_f64[168]=(-self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=p.p22;
        self.scalar_static_f64[170]=p.p23;
        self.scalar_static_f64[171]=(2.0*self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=(6.0-self.scalar_static_f64[171]);
        self.scalar_static_f64[173]=p.p145;
        self.scalar_static_f64[174]=p.p146;
        self.scalar_static_f64[175]=(4.0/self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=p.p151;
        self.scalar_static_f64[177]=p.p153;
        self.scalar_static_f64[178]=p.p35;
        self.scalar_static_f64[179]=p.p34;
        self.scalar_static_f64[180]=p.p37;
        self.scalar_static_f64[181]=p.p36;
        self.scalar_static_f64[182]=p.p14;
        self.scalar_static_f64[183]=p.p13;
        self.scalar_static_f64[184]=p.p133;
        self.scalar_static_f64[185]=p.p141;
        self.scalar_static_f64[186]=(4.0-self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=(-self.scalar_static_f64[93]);
        self.scalar_static_f64[188]=p.p142;
        self.scalar_static_f64[189]=(0.5*self.scalar_static_f64[188]);
        self.scalar_static_f64[190]=(3.5-self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=p.p135;
        self.scalar_static_f64[192]=(1.0-self.scalar_static_f64[185]);
        self.scalar_static_f64[193]=p.p136;
        self.scalar_static_f64[194]=(1.0-self.scalar_static_f64[188]);
        self.scalar_static_f64[195]=p.p86;
        self.scalar_static_f64[196]=(self.scalar_static_f64[103]-2.0);
        self.scalar_static_f64[197]=p.p120;
        self.scalar_static_f64[198]=(-self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=p.p87;
        self.scalar_static_f64[200]=(self.scalar_static_f64[103]+self.scalar_static_f64[104]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[200]-1.0);
        self.scalar_static_f64[202]=p.p88;
        self.scalar_static_f64[203]=(self.scalar_static_f64[114]-1.0);
        self.scalar_static_f64[204]=p.p89;
        self.scalar_static_f64[205]=(self.scalar_static_f64[199]+self.scalar_static_f64[202]);
        self.scalar_static_f64[206]=p.p90;
        self.scalar_static_f64[207]=p.p100;
        self.scalar_static_f64[208]=(self.scalar_static_f64[207]-1.0);
        self.scalar_static_f64[209]=(self.scalar_static_f64[5]*1.081);
        self.scalar_static_f64[210]=p.p92;
        self.scalar_static_bool[13]=(self.scalar_static_f64[108]>0.0);
        self.scalar_static_f64[211]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_bool[14]=(!(self.scalar_static_f64[211]!=0.0));
        self.scalar_static_bool[15]=(self.scalar_static_f64[110]>0.0);
        self.scalar_static_f64[212]=(if self.scalar_static_bool[15]{1.0}else{0.0});
        self.scalar_static_bool[16]=(!(self.scalar_static_f64[212]!=0.0));
        self.scalar_static_bool[17]=(self.scalar_static_f64[112]>0.0);
        self.scalar_static_f64[213]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=(!(self.scalar_static_f64[213]!=0.0));
        self.scalar_static_f64[214]=p.p147;
        self.scalar_static_f64[215]=(self.scalar_static_f64[214]).exp();
        self.scalar_static_f64[216]=p.p149;
        self.scalar_static_f64[217]=p.p62;
        self.scalar_static_f64[218]=p.p61;
        self.scalar_static_f64[219]=(self.scalar_static_f64[217]*self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=p.p63;
        self.scalar_static_f64[221]=(-1.0/self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[221]).exp();
        self.scalar_static_f64[223]=(1.0+self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[223]).ln();
        self.scalar_static_f64[225]=(self.scalar_static_f64[220]*self.scalar_static_f64[224]);
        self.scalar_static_f64[226]=(1.0+self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=p.p148;
        self.scalar_static_f64[228]=(0.5*self.scalar_static_f64[218]);
        self.scalar_static_f64[229]=p.p73;
        self.scalar_static_bool[19]=(0.0==self.scalar_static_f64[229]);
        self.scalar_static_f64[230]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_bool[20]=(!(self.scalar_static_f64[230]!=0.0));
        self.scalar_static_f64[231]=(-1.0/self.scalar_static_f64[19]);
        self.scalar_static_f64[232]=f64::powf(3.0,self.scalar_static_f64[231]);
        self.scalar_static_f64[233]=(1.0-self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(1.0-self.scalar_static_f64[19]);
        self.scalar_static_f64[235]=p.p74;
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[235]);
        self.scalar_static_f64[236]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_bool[22]=(2.0==self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_bool[23]=(!(self.scalar_static_f64[236]!=0.0));
        self.scalar_static_bool[24]=((self.scalar_static_f64[237]!=0.0)&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=(!(self.scalar_static_f64[237]!=0.0));
        self.scalar_static_bool[26]=(self.scalar_static_bool[23]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[238]=(-1.0/self.scalar_static_f64[51]);
        self.scalar_static_f64[239]=p.p76;
        self.scalar_static_f64[240]=(1.0-self.scalar_static_f64[51]);
        self.scalar_static_bool[27]=(0.0==self.scalar_static_f64[210]);
        self.scalar_static_f64[241]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_bool[28]=(!(self.scalar_static_f64[241]!=0.0));
        self.scalar_static_f64[242]=p.p15;
        self.scalar_static_f64[243]=p.p152;
        self.scalar_static_f64[244]=p.p154;
        self.scalar_static_f64[245]=p.p155;
        self.scalar_static_f64[246]=p.p93;
        self.scalar_static_bool[29]=(0.0==self.scalar_static_f64[246]);
        self.scalar_static_f64[247]=(if self.scalar_static_bool[29]{1.0}else{0.0});
        self.scalar_static_bool[30]=(!(self.scalar_static_f64[154]!=0.0));
        self.scalar_static_bool[31]=((self.scalar_static_f64[247]!=0.0)&&self.scalar_static_bool[30]);
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[247]!=0.0));
        self.scalar_static_bool[33]=(self.scalar_static_bool[30]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[248]=(1.0-self.scalar_static_f64[246]);
        self.scalar_static_bool[34]=(self.scalar_static_f64[179]>0.0);
        self.scalar_static_bool[35]=(self.scalar_static_f64[178]>0.0);
        self.scalar_static_bool[36]=(self.scalar_static_bool[34]&&self.scalar_static_bool[35]);
        self.scalar_static_f64[249]=(-2.0-self.scalar_static_f64[19]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[19]*self.scalar_static_f64[19]);
        self.scalar_static_f64[251]=(1.0-self.scalar_static_f64[250]);
        self.scalar_static_f64[252]=(self.scalar_static_f64[19]-1.0);
        self.scalar_static_bool[37]=(self.scalar_static_f64[181]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_f64[180]>0.0);
        self.scalar_static_bool[39]=(self.scalar_static_bool[37]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[253]=(-2.0-self.scalar_static_f64[51]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[51]*self.scalar_static_f64[51]);
        self.scalar_static_f64[255]=(1.0-self.scalar_static_f64[254]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[51]-1.0);
        self.scalar_static_f64[257]=p.p8;
        self.scalar_static_bool[40]=(1.0==self.scalar_static_f64[257]);
        self.scalar_static_f64[258]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_f64[259]=p.p143;
        self.scalar_static_f64[260]=(2.0*self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=p.p144;
        self.scalar_static_f64[262]=(1.0-self.scalar_static_f64[259]);
        self.scalar_static_f64[263]=(2.0*self.scalar_static_f64[262]);
        self.scalar_static_bool[41]=(!(self.scalar_static_f64[258]!=0.0));
        self.scalar_static_f64[264]=(4.0*self.scalar_static_f64[261]);
        self.scalar_static_f64[265]=p.p5;
        self.scalar_static_bool[42]=(self.scalar_static_f64[265]>0.0);
        self.scalar_static_bool[43]=(self.scalar_static_f64[6]>0.0);
        self.scalar_static_bool[44]=(self.scalar_static_bool[42]&&self.scalar_static_bool[43]);
        self.scalar_static_f64[266]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_f64[267]=(self.scalar_static_f64[6]*2.0);
        self.scalar_static_bool[45]=((self.scalar_static_f64[258]!=0.0)&&(self.scalar_static_f64[266]!=0.0));
        self.scalar_static_f64[268]=(self.scalar_static_f64[6]*self.scalar_static_f64[262]);
        self.scalar_static_f64[269]=(2.0*self.scalar_static_f64[268]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[41]&&(self.scalar_static_f64[266]!=0.0));
        self.scalar_static_bool[47]=(1.0==self.scalar_static_f64[265]);
        self.scalar_static_f64[270]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_bool[48]=((self.scalar_static_f64[266]!=0.0)&&(self.scalar_static_f64[270]!=0.0));
        self.scalar_static_f64[271]=(if self.scalar_static_bool[48]{0.0121}else{0.010000000000000002});
        self.scalar_static_f64[272]=(0.5*self.scalar_static_f64[271]);
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[270]!=0.0));
        self.scalar_static_bool[50]=((self.scalar_static_f64[266]!=0.0)&&self.scalar_static_bool[49]);
        self.scalar_static_f64[273]=p.p84;
        self.scalar_static_bool[51]=(1.0==self.scalar_static_f64[273]);
        self.scalar_static_f64[274]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_f64[275]=(if (self.scalar_static_f64[274]!=0.0){1e-12}else{self.scalar_static_f64[271]});
        self.scalar_static_f64[276]=(0.5*self.scalar_static_f64[275]);
        self.scalar_static_f64[277]=p.p82;
        self.scalar_static_f64[278]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(1.0-self.scalar_static_f64[278]);
        self.scalar_static_f64[280]=(1.0/self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[280]}else{0.0});
        self.scalar_static_f64[282]=p.p81;
        self.scalar_static_f64[283]=(self.scalar_static_f64[83]*self.scalar_static_f64[282]);
        self.scalar_static_f64[284]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[283]}else{0.0});
        self.scalar_static_f64[285]=(self.scalar_static_f64[281]*self.scalar_static_f64[281]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[277]-1.0);
        self.scalar_static_f64[287]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[286]);
        self.scalar_static_f64[288]=(self.scalar_static_f64[285]*self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[277]*self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[289]/self.scalar_static_f64[282]);
        self.scalar_static_f64[291]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[290]}else{0.0});
        self.scalar_static_bool[52]=(!(self.scalar_static_f64[274]!=0.0));
        self.scalar_static_f64[292]=p.p39;
        self.scalar_static_bool[53]=(1.0==self.scalar_static_f64[292]);
        self.scalar_static_f64[293]=(if self.scalar_static_bool[53]{1.0}else{0.0});
        self.scalar_static_f64[294]=p.p44;
        self.scalar_static_f64[295]=p.p42;
        self.scalar_static_f64[296]=p.p41;
        self.scalar_static_f64[297]=p.p40;
        self.scalar_static_bool[54]=(2.0==self.scalar_static_f64[292]);
        self.scalar_static_f64[298]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_bool[55]=(!(self.scalar_static_f64[293]!=0.0));
        self.scalar_static_f64[299]=p.p46;
        self.scalar_static_f64[300]=(2.0*self.scalar_static_f64[299]);
        self.scalar_static_f64[301]=p.p45;
        self.scalar_static_f64[302]=(self.scalar_static_f64[301]*self.scalar_static_f64[301]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[300]/self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=p.p7;
        self.scalar_static_bool[56]=(0.0==self.scalar_static_f64[304]);
        self.scalar_static_f64[305]=(if self.scalar_static_bool[56]{1.0}else{0.0});
        self.scalar_static_bool[57]=(!(self.scalar_static_f64[305]!=0.0));
        self.scalar_static_f64[306]=p.p47;
        self.scalar_static_f64[307]=(2.0*self.scalar_static_f64[306]);
        self.scalar_static_f64[308]=(1.0+self.scalar_static_f64[306]);
        self.scalar_static_f64[309]=(1.0+self.scalar_static_f64[307]);
        self.scalar_static_f64[310]=(self.scalar_static_f64[308]/self.scalar_static_f64[309]);
        self.scalar_static_bool[58]=(3.0==self.scalar_static_f64[292]);
        self.scalar_static_f64[311]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_bool[59]=(!(self.scalar_static_f64[298]!=0.0));
        self.scalar_static_f64[312]=p.p48;
        self.scalar_static_f64[313]=p.p49;
        self.scalar_static_f64[314]=p.p52;
        self.scalar_static_f64[315]=p.p51;
        self.scalar_static_f64[316]=p.p50;
        self.scalar_static_f64[317]=p.p53;
        self.scalar_static_bool[60]=(1.0==self.scalar_static_f64[317]);
        self.scalar_static_f64[318]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=(!(self.scalar_static_f64[311]!=0.0));
        self.scalar_static_bool[62]=(!(self.scalar_static_f64[318]!=0.0));
        self.scalar_static_f64[319]=p.p68;
        self.scalar_static_f64[320]=(1.0-self.scalar_static_f64[319]);
        self.scalar_static_f64[321]=p.p77;
        self.scalar_static_f64[322]=(1.0-self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(-1.0/self.scalar_static_f64[96]);
        self.scalar_static_f64[324]=f64::powf(2.0,self.scalar_static_f64[323]);
        self.scalar_static_f64[325]=(1.0-self.scalar_static_f64[324]);
        self.scalar_static_f64[326]=(1.0-self.scalar_static_f64[96]);
        self.scalar_static_f64[327]=p.p85;
        self.scalar_static_f64[328]=(1.0/self.scalar_static_f64[327]);
        self.scalar_static_f64[329]=p.p79;
        self.scalar_static_bool[63]=(0.0==self.scalar_static_f64[329]);
        self.scalar_static_f64[330]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_f64[331]=p.p91;
        self.scalar_static_bool[64]=(!(self.scalar_static_f64[330]!=0.0));
        self.scalar_static_bool[65]=(3.0==self.scalar_static_f64[265]);
        self.scalar_static_bool[66]=(self.scalar_static_bool[47]||self.scalar_static_bool[65]);
        self.scalar_static_bool[67]=(self.scalar_static_bool[43]&&self.scalar_static_bool[66]);
        self.scalar_static_f64[332]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_bool[68]=((self.scalar_static_f64[330]!=0.0)&&(self.scalar_static_f64[332]!=0.0));
        self.scalar_static_f64[333]=(self.scalar_static_f64[6]*0.5);
        self.scalar_static_bool[69]=(self.scalar_static_bool[64]&&(self.scalar_static_f64[332]!=0.0));
        self.scalar_static_f64[334]=p.p6;
        self.scalar_static_bool[70]=(1.0==self.scalar_static_f64[334]);
        self.scalar_static_f64[335]=(if self.scalar_static_bool[70]{1.0}else{0.0});
        self.scalar_static_f64[336]=(-self.scalar_static_f64[19]);
        self.scalar_static_f64[337]=p.p95;
        self.scalar_static_f64[338]=(1.0-self.scalar_static_f64[337]);
        self.scalar_static_f64[339]=p.p94;
        self.scalar_static_f64[340]=(1.0-self.scalar_static_f64[339]);
        self.scalar_static_bool[71]=(!(self.scalar_static_f64[335]!=0.0));
        self.scalar_static_f64[341]=p.p130;
        self.scalar_static_bool[72]=(self.scalar_static_f64[341]>0.0);
        self.scalar_static_f64[342]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_bool[73]=(!(self.scalar_static_f64[342]!=0.0));
        self.scalar_static_f64[343]=p.p131;
        self.scalar_static_bool[74]=(1.0==self.scalar_static_f64[343]);
        self.scalar_static_f64[344]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_bool[75]=(2.0==self.scalar_static_f64[343]);
        self.scalar_static_f64[345]=(if self.scalar_static_bool[75]{1.0}else{0.0});
        self.scalar_static_bool[76]=(!(self.scalar_static_f64[344]!=0.0));
        self.scalar_static_bool[77]=((self.scalar_static_f64[345]!=0.0)&&self.scalar_static_bool[76]);
        self.scalar_static_f64[346]=p.p132;
        self.scalar_static_bool[78]=(!(self.scalar_static_f64[345]!=0.0));
        self.scalar_static_bool[79]=(self.scalar_static_bool[76]&&self.scalar_static_bool[78]);
        self.scalar_static_f64[347]=p.p69;
        self.scalar_static_f64[348]=p.p78;
        self.scalar_static_f64[349]=(self.scalar_static_f64[0]*self.scalar_static_f64[347]);
        self.scalar_static_f64[350]=(self.scalar_static_f64[0]*self.scalar_static_f64[348]);
        self.scalar_static_f64[351]=(-self.scalar_static_f64[0]);
        self.scalar_static_f64[352]=(self.scalar_static_f64[0]+self.scalar_static_f64[351]);
        self.scalar_static_f64[353]=(self.scalar_static_f64[351]-self.scalar_static_f64[351]);
        self.scalar_static_f64[354]=(self.scalar_static_f64[0]+self.scalar_static_f64[352]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[234]-1.0);
        self.scalar_static_f64[356]=(if (self.scalar_static_f64[236]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[357]=(if (self.scalar_static_f64[236]!=0.0){self.scalar_static_f64[351]}else{0.0});
        self.scalar_static_f64[358]=(self.scalar_static_f64[239]-1.0);
        self.scalar_static_f64[359]=(self.scalar_static_f64[240]-1.0);
        self.scalar_static_f64[360]=(self.scalar_static_f64[351]/0.0001);
        self.scalar_static_f64[361]=(self.scalar_static_f64[0]/0.0001);
        self.scalar_static_f64[362]=(-self.scalar_static_f64[360]);
        self.scalar_static_f64[363]=(-self.scalar_static_f64[361]);
        self.scalar_static_f64[364]=(self.scalar_static_f64[351]/0.001);
        self.scalar_static_f64[365]=(self.scalar_static_f64[0]/0.001);
        self.scalar_static_f64[366]=(-self.scalar_static_f64[364]);
        self.scalar_static_f64[367]=(-self.scalar_static_f64[365]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[249]-1.0);
        self.scalar_static_f64[369]=(self.scalar_static_f64[21]*self.scalar_static_f64[351]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[0]*self.scalar_static_f64[21]);
        self.scalar_static_f64[371]=(0.5*self.scalar_static_f64[351]);
        self.scalar_static_f64[372]=(self.scalar_static_f64[0]*0.5);
        self.scalar_static_f64[373]=(self.scalar_static_f64[253]-1.0);
        self.scalar_static_f64[374]=(self.scalar_static_f64[0]*self.scalar_static_f64[53]);
        self.scalar_static_f64[375]=(self.scalar_static_f64[53]*self.scalar_static_f64[351]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[0]*0.0);
        self.scalar_static_f64[377]=(0.0*self.scalar_static_f64[351]);
        self.scalar_static_f64[378]=(if self.scalar_static_bool[48]{self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[379]=(if self.scalar_static_bool[48]{self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_f64[380]=(if self.scalar_static_bool[48]{self.scalar_static_f64[353]}else{0.0});
        self.scalar_static_f64[381]=(if self.scalar_static_bool[48]{self.scalar_static_f64[351]}else{0.0});
        self.scalar_static_f64[382]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[383]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[384]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[351]}else{0.0});
        self.scalar_static_f64[385]=(-self.scalar_static_f64[382]);
        self.scalar_static_f64[386]=(-self.scalar_static_f64[383]);
        self.scalar_static_f64[387]=(-self.scalar_static_f64[384]);
        self.scalar_static_f64[388]=(self.scalar_static_f64[296]-1.0);
        self.scalar_static_f64[389]=(self.scalar_static_f64[313]-1.0);
        self.scalar_static_f64[390]=(self.scalar_static_f64[316]-1.0);
        self.scalar_static_f64[391]=(self.scalar_static_f64[326]-1.0);
        self.scalar_static_f64[392]=(self.scalar_static_f64[0]/self.scalar_static_f64[331]);
        self.scalar_static_f64[393]=(self.scalar_static_f64[352]/self.scalar_static_f64[331]);
        self.scalar_static_f64[394]=(self.scalar_static_f64[353]/self.scalar_static_f64[331]);
        self.scalar_static_f64[395]=(self.scalar_static_f64[351]/self.scalar_static_f64[331]);
        self.scalar_static_f64[396]=(self.scalar_static_f64[336]-1.0);
        self.scalar_static_f64[397]=(self.scalar_static_f64[0]*0.2);
        self.scalar_static_f64[398]=(0.2*self.scalar_static_f64[351]);
        self.scalar_static_f64[399]=(0.0*self.scalar_static_f64[352]);
        self.scalar_static_f64[400]=(0.0*self.scalar_static_f64[353]);
        self.scalar_static_f64[401]=(self.scalar_static_f64[0]*self.scalar_static_f64[0]);
        self.scalar_static_f64[402]=(self.scalar_static_f64[0]*self.scalar_static_f64[351]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[0]*self.scalar_static_f64[349]);
        self.scalar_static_f64[404]=(self.scalar_static_f64[349]*self.scalar_static_f64[351]);
        self.scalar_static_f64[405]=(self.scalar_static_f64[350]*self.scalar_static_f64[351]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[0]*self.scalar_static_f64[350]);
        self.scalar_static_f64[407]=(self.scalar_static_f64[0]*self.scalar_static_f64[352]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[0]*self.scalar_static_f64[353]);
    }

    #[inline]
    fn invalidate_temperature_static(&mut self) {
        self.scalar_temperature_static_valid = false;
    }

    #[inline]
    pub(super) fn ensure_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        if !self.scalar_temperature_static_valid
            || self.scalar_temperature_static_temperature.to_bits() != temperature.to_bits()
            || self.scalar_temperature_static_thermal_voltage.to_bits() != thermal_voltage.to_bits()
        {
            self.recompute_temperature_static(temperature, thermal_voltage);
        }
    }

    #[inline]
    fn recompute_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        let p = &(*self.params);
        self.scalar_static_f64[409]=(temperature+self.scalar_static_f64[10]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[409]/self.scalar_static_f64[9]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[409]*8.617086918058125e-5);
        self.scalar_static_f64[412]=(1.0/self.scalar_static_f64[411]);
        self.scalar_static_f64[413]=(self.scalar_static_f64[412]-self.scalar_static_f64[85]);
        self.scalar_static_f64[414]=(self.scalar_static_f64[409]-self.scalar_static_f64[9]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[410]).ln();
        self.scalar_static_f64[416]=(self.scalar_static_f64[409]*self.scalar_static_f64[24]);
        self.scalar_static_f64[417]=(self.scalar_static_f64[409]*self.scalar_static_f64[416]);
        self.scalar_static_f64[418]=(self.scalar_static_f64[409]+self.scalar_static_f64[27]);
        self.scalar_static_f64[419]=(self.scalar_static_f64[417]/self.scalar_static_f64[418]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[46]-self.scalar_static_f64[419]);
        self.scalar_static_f64[421]=(self.scalar_static_f64[420]-0.05);
        self.scalar_static_f64[422]=(self.scalar_static_f64[421]/0.1);
        self.scalar_static_bool[80]=(self.scalar_static_f64[420]<0.05);
        self.scalar_static_f64[423]=(if self.scalar_static_bool[80]{1.0}else{0.0});
        self.scalar_static_f64[424]=(self.scalar_static_f64[422]).exp();
        self.scalar_static_f64[425]=(1.0+self.scalar_static_f64[424]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[425]).ln();
        self.scalar_static_f64[427]=(0.1*self.scalar_static_f64[426]);
        self.scalar_static_f64[428]=(0.05+self.scalar_static_f64[427]);
        self.scalar_static_f64[429]=(if (self.scalar_static_f64[423]!=0.0){self.scalar_static_f64[428]}else{0.0});
        self.scalar_static_bool[81]=(!(self.scalar_static_f64[423]!=0.0));
        self.scalar_static_f64[430]=(-self.scalar_static_f64[422]);
        self.scalar_static_f64[431]=(self.scalar_static_f64[430]).exp();
        self.scalar_static_f64[432]=(1.0+self.scalar_static_f64[431]);
        self.scalar_static_f64[433]=(self.scalar_static_f64[432]).ln();
        self.scalar_static_f64[434]=(0.1*self.scalar_static_f64[433]);
        self.scalar_static_f64[435]=(self.scalar_static_f64[420]+self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=(if self.scalar_static_bool[81]{self.scalar_static_f64[435]}else{self.scalar_static_f64[429]});
        self.scalar_static_f64[437]=(self.scalar_static_f64[409]*self.scalar_static_f64[56]);
        self.scalar_static_f64[438]=(self.scalar_static_f64[409]*self.scalar_static_f64[437]);
        self.scalar_static_f64[439]=(self.scalar_static_f64[409]+self.scalar_static_f64[59]);
        self.scalar_static_f64[440]=(self.scalar_static_f64[438]/self.scalar_static_f64[439]);
        self.scalar_static_f64[441]=(self.scalar_static_f64[78]-self.scalar_static_f64[440]);
        self.scalar_static_f64[442]=(self.scalar_static_f64[441]-0.05);
        self.scalar_static_f64[443]=(self.scalar_static_f64[442]/0.1);
        self.scalar_static_bool[82]=(self.scalar_static_f64[441]<0.05);
        self.scalar_static_f64[444]=(if self.scalar_static_bool[82]{1.0}else{0.0});
        self.scalar_static_f64[445]=(self.scalar_static_f64[443]).exp();
        self.scalar_static_f64[446]=(1.0+self.scalar_static_f64[445]);
        self.scalar_static_f64[447]=(self.scalar_static_f64[446]).ln();
        self.scalar_static_f64[448]=(0.1*self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=(0.05+self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=(if (self.scalar_static_f64[444]!=0.0){self.scalar_static_f64[449]}else{0.0});
        self.scalar_static_bool[83]=(!(self.scalar_static_f64[444]!=0.0));
        self.scalar_static_f64[451]=(-self.scalar_static_f64[443]);
        self.scalar_static_f64[452]=(self.scalar_static_f64[451]).exp();
        self.scalar_static_f64[453]=(1.0+self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(self.scalar_static_f64[453]).ln();
        self.scalar_static_f64[455]=(0.1*self.scalar_static_f64[454]);
        self.scalar_static_f64[456]=(self.scalar_static_f64[441]+self.scalar_static_f64[455]);
        self.scalar_static_f64[457]=(if self.scalar_static_bool[83]{self.scalar_static_f64[456]}else{self.scalar_static_f64[450]});
        self.scalar_static_f64[458]=(self.scalar_static_f64[411]* -3.0);
        self.scalar_static_f64[459]=(self.scalar_static_f64[415]*self.scalar_static_f64[458]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[48]*self.scalar_static_f64[410]);
        self.scalar_static_f64[461]=(self.scalar_static_f64[459]+self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(1.0-self.scalar_static_f64[410]);
        self.scalar_static_f64[463]=(self.scalar_static_f64[462]*self.scalar_static_f64[86]);
        self.scalar_static_f64[464]=(self.scalar_static_f64[461]+self.scalar_static_f64[463]);
        self.scalar_static_f64[465]=(0.05-self.scalar_static_f64[464]);
        self.scalar_static_f64[466]=(self.scalar_static_f64[465]/self.scalar_static_f64[411]);
        self.scalar_static_bool[84]=(0.05<self.scalar_static_f64[464]);
        self.scalar_static_f64[467]=(if self.scalar_static_bool[84]{1.0}else{0.0});
        self.scalar_static_f64[468]=(self.scalar_static_f64[466]).exp();
        self.scalar_static_f64[469]=(1.0+self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[469]).ln();
        self.scalar_static_f64[471]=(self.scalar_static_f64[411]*self.scalar_static_f64[470]);
        self.scalar_static_f64[472]=(self.scalar_static_f64[464]+self.scalar_static_f64[471]);
        self.scalar_static_f64[473]=(if (self.scalar_static_f64[467]!=0.0){self.scalar_static_f64[472]}else{0.0});
        self.scalar_static_bool[85]=(!(self.scalar_static_f64[467]!=0.0));
        self.scalar_static_f64[474]=(-self.scalar_static_f64[466]);
        self.scalar_static_f64[475]=(self.scalar_static_f64[474]).exp();
        self.scalar_static_f64[476]=(1.0+self.scalar_static_f64[475]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[476]).ln();
        self.scalar_static_f64[478]=(self.scalar_static_f64[411]*self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(0.05+self.scalar_static_f64[478]);
        self.scalar_static_f64[480]=(if self.scalar_static_bool[85]{self.scalar_static_f64[479]}else{self.scalar_static_f64[473]});
        self.scalar_static_f64[481]=(self.scalar_static_f64[410]*self.scalar_static_f64[87]);
        self.scalar_static_f64[482]=(self.scalar_static_f64[459]+self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[462]*self.scalar_static_f64[88]);
        self.scalar_static_f64[484]=(self.scalar_static_f64[482]+self.scalar_static_f64[483]);
        self.scalar_static_f64[485]=(0.05-self.scalar_static_f64[484]);
        self.scalar_static_f64[486]=(self.scalar_static_f64[485]/self.scalar_static_f64[411]);
        self.scalar_static_bool[86]=(0.05<self.scalar_static_f64[484]);
        self.scalar_static_f64[487]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_f64[488]=(self.scalar_static_f64[486]).exp();
        self.scalar_static_f64[489]=(1.0+self.scalar_static_f64[488]);
        self.scalar_static_f64[490]=(self.scalar_static_f64[489]).ln();
        self.scalar_static_f64[491]=(self.scalar_static_f64[411]*self.scalar_static_f64[490]);
        self.scalar_static_f64[492]=(self.scalar_static_f64[484]+self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=(if (self.scalar_static_f64[487]!=0.0){self.scalar_static_f64[492]}else{0.0});
        self.scalar_static_bool[87]=(!(self.scalar_static_f64[487]!=0.0));
        self.scalar_static_f64[494]=(-self.scalar_static_f64[486]);
        self.scalar_static_f64[495]=(self.scalar_static_f64[494]).exp();
        self.scalar_static_f64[496]=(1.0+self.scalar_static_f64[495]);
        self.scalar_static_f64[497]=(self.scalar_static_f64[496]).ln();
        self.scalar_static_f64[498]=(self.scalar_static_f64[411]*self.scalar_static_f64[497]);
        self.scalar_static_f64[499]=(0.05+self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=(if self.scalar_static_bool[87]{self.scalar_static_f64[499]}else{self.scalar_static_f64[493]});
        self.scalar_static_f64[501]=(self.scalar_static_f64[410]*self.scalar_static_f64[89]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[459]+self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[483]+self.scalar_static_f64[502]);
        self.scalar_static_f64[504]=(0.05-self.scalar_static_f64[503]);
        self.scalar_static_f64[505]=(self.scalar_static_f64[504]/self.scalar_static_f64[411]);
        self.scalar_static_bool[88]=(0.05<self.scalar_static_f64[503]);
        self.scalar_static_f64[506]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_f64[507]=(self.scalar_static_f64[505]).exp();
        self.scalar_static_f64[508]=(1.0+self.scalar_static_f64[507]);
        self.scalar_static_f64[509]=(self.scalar_static_f64[508]).ln();
        self.scalar_static_f64[510]=(self.scalar_static_f64[411]*self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[503]+self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=(if (self.scalar_static_f64[506]!=0.0){self.scalar_static_f64[511]}else{0.0});
        self.scalar_static_bool[89]=(!(self.scalar_static_f64[506]!=0.0));
        self.scalar_static_f64[513]=(-self.scalar_static_f64[505]);
        self.scalar_static_f64[514]=(self.scalar_static_f64[513]).exp();
        self.scalar_static_f64[515]=(1.0+self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[515]).ln();
        self.scalar_static_f64[517]=(self.scalar_static_f64[411]*self.scalar_static_f64[516]);
        self.scalar_static_f64[518]=(0.05+self.scalar_static_f64[517]);
        self.scalar_static_f64[519]=(if self.scalar_static_bool[89]{self.scalar_static_f64[518]}else{self.scalar_static_f64[512]});
        self.scalar_static_f64[520]=(self.scalar_static_f64[50]*self.scalar_static_f64[410]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[459]+self.scalar_static_f64[520]);
        self.scalar_static_f64[522]=(self.scalar_static_f64[483]+self.scalar_static_f64[521]);
        self.scalar_static_f64[523]=(0.05-self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(self.scalar_static_f64[523]/self.scalar_static_f64[411]);
        self.scalar_static_bool[90]=(0.05<self.scalar_static_f64[522]);
        self.scalar_static_f64[525]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_f64[526]=(self.scalar_static_f64[524]).exp();
        self.scalar_static_f64[527]=(1.0+self.scalar_static_f64[526]);
        self.scalar_static_f64[528]=(self.scalar_static_f64[527]).ln();
        self.scalar_static_f64[529]=(self.scalar_static_f64[411]*self.scalar_static_f64[528]);
        self.scalar_static_f64[530]=(self.scalar_static_f64[522]+self.scalar_static_f64[529]);
        self.scalar_static_f64[531]=(if (self.scalar_static_f64[525]!=0.0){self.scalar_static_f64[530]}else{0.0});
        self.scalar_static_bool[91]=(!(self.scalar_static_f64[525]!=0.0));
        self.scalar_static_f64[532]=(-self.scalar_static_f64[524]);
        self.scalar_static_f64[533]=(self.scalar_static_f64[532]).exp();
        self.scalar_static_f64[534]=(1.0+self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(self.scalar_static_f64[534]).ln();
        self.scalar_static_f64[536]=(self.scalar_static_f64[411]*self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=(0.05+self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=(if self.scalar_static_bool[91]{self.scalar_static_f64[537]}else{self.scalar_static_f64[531]});
        self.scalar_static_f64[539]=(self.scalar_static_f64[410]*self.scalar_static_f64[90]);
        self.scalar_static_f64[540]=(self.scalar_static_f64[459]+self.scalar_static_f64[539]);
        self.scalar_static_f64[541]=(self.scalar_static_f64[462]*self.scalar_static_f64[91]);
        self.scalar_static_f64[542]=(self.scalar_static_f64[540]+self.scalar_static_f64[541]);
        self.scalar_static_f64[543]=(0.05-self.scalar_static_f64[542]);
        self.scalar_static_f64[544]=(self.scalar_static_f64[543]/self.scalar_static_f64[411]);
        self.scalar_static_bool[92]=(0.05<self.scalar_static_f64[542]);
        self.scalar_static_f64[545]=(if self.scalar_static_bool[92]{1.0}else{0.0});
        self.scalar_static_f64[546]=(self.scalar_static_f64[544]).exp();
        self.scalar_static_f64[547]=(1.0+self.scalar_static_f64[546]);
        self.scalar_static_f64[548]=(self.scalar_static_f64[547]).ln();
        self.scalar_static_f64[549]=(self.scalar_static_f64[411]*self.scalar_static_f64[548]);
        self.scalar_static_f64[550]=(self.scalar_static_f64[542]+self.scalar_static_f64[549]);
        self.scalar_static_f64[551]=(if (self.scalar_static_f64[545]!=0.0){self.scalar_static_f64[550]}else{0.0});
        self.scalar_static_bool[93]=(!(self.scalar_static_f64[545]!=0.0));
        self.scalar_static_f64[552]=(-self.scalar_static_f64[544]);
        self.scalar_static_f64[553]=(self.scalar_static_f64[552]).exp();
        self.scalar_static_f64[554]=(1.0+self.scalar_static_f64[553]);
        self.scalar_static_f64[555]=(self.scalar_static_f64[554]).ln();
        self.scalar_static_f64[556]=(self.scalar_static_f64[411]*self.scalar_static_f64[555]);
        self.scalar_static_f64[557]=(0.05+self.scalar_static_f64[556]);
        self.scalar_static_f64[558]=(if self.scalar_static_bool[93]{self.scalar_static_f64[557]}else{self.scalar_static_f64[551]});
        self.scalar_static_f64[559]=(self.scalar_static_f64[410]*self.scalar_static_f64[92]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[459]+self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=(self.scalar_static_f64[462]*self.scalar_static_f64[93]);
        self.scalar_static_f64[562]=(self.scalar_static_f64[560]+self.scalar_static_f64[561]);
        self.scalar_static_f64[563]=(0.05-self.scalar_static_f64[562]);
        self.scalar_static_f64[564]=(self.scalar_static_f64[563]/self.scalar_static_f64[411]);
        self.scalar_static_bool[94]=(0.05<self.scalar_static_f64[562]);
        self.scalar_static_f64[565]=(if self.scalar_static_bool[94]{1.0}else{0.0});
        self.scalar_static_f64[566]=(self.scalar_static_f64[564]).exp();
        self.scalar_static_f64[567]=(1.0+self.scalar_static_f64[566]);
        self.scalar_static_f64[568]=(self.scalar_static_f64[567]).ln();
        self.scalar_static_f64[569]=(self.scalar_static_f64[411]*self.scalar_static_f64[568]);
        self.scalar_static_f64[570]=(self.scalar_static_f64[562]+self.scalar_static_f64[569]);
        self.scalar_static_f64[571]=(if (self.scalar_static_f64[565]!=0.0){self.scalar_static_f64[570]}else{0.0});
        self.scalar_static_bool[95]=(!(self.scalar_static_f64[565]!=0.0));
        self.scalar_static_f64[572]=(-self.scalar_static_f64[564]);
        self.scalar_static_f64[573]=(self.scalar_static_f64[572]).exp();
        self.scalar_static_f64[574]=(1.0+self.scalar_static_f64[573]);
        self.scalar_static_f64[575]=(self.scalar_static_f64[574]).ln();
        self.scalar_static_f64[576]=(self.scalar_static_f64[411]*self.scalar_static_f64[575]);
        self.scalar_static_f64[577]=(0.05+self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=(if self.scalar_static_bool[95]{self.scalar_static_f64[577]}else{self.scalar_static_f64[571]});
        self.scalar_static_f64[579]=(1.0/self.scalar_static_f64[480]);
        self.scalar_static_f64[580]=(1.0/self.scalar_static_f64[538]);
        self.scalar_static_f64[581]=(self.scalar_static_f64[48]*self.scalar_static_f64[579]);
        self.scalar_static_f64[582]=f64::powf(self.scalar_static_f64[581],self.scalar_static_f64[19]);
        self.scalar_static_f64[583]=(self.scalar_static_f64[50]*self.scalar_static_f64[580]);
        self.scalar_static_f64[584]=f64::powf(self.scalar_static_f64[583],self.scalar_static_f64[51]);
        self.scalar_static_f64[585]=(self.scalar_static_f64[582]*self.scalar_static_f64[94]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[92]/self.scalar_static_f64[578]);
        self.scalar_static_f64[587]=f64::powf(self.scalar_static_f64[586],self.scalar_static_f64[96]);
        self.scalar_static_f64[588]=(self.scalar_static_f64[95]*self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(self.scalar_static_f64[50]/self.scalar_static_f64[538]);
        self.scalar_static_f64[590]=f64::powf(self.scalar_static_f64[589],self.scalar_static_f64[51]);
        self.scalar_static_f64[591]=(self.scalar_static_f64[98]*self.scalar_static_f64[590]);
        self.scalar_static_f64[592]=(self.scalar_static_f64[97]+self.scalar_static_f64[591]);
        self.scalar_static_f64[593]=(1.0/self.scalar_static_f64[592]);
        self.scalar_static_f64[594]=(self.scalar_static_f64[592]*self.scalar_static_f64[99]);
        self.scalar_static_f64[595]=(self.scalar_static_f64[97]*self.scalar_static_f64[593]);
        self.scalar_static_f64[596]=(self.scalar_static_f64[415]*self.scalar_static_f64[101]);
        self.scalar_static_f64[597]=(self.scalar_static_f64[596]).exp();
        self.scalar_static_f64[598]=(self.scalar_static_f64[100]*self.scalar_static_f64[597]);
        self.scalar_static_bool[96]=(self.scalar_static_f64[598]<self.scalar_static_f64[16]);
        self.scalar_static_f64[599]=(if self.scalar_static_bool[96]{1.0}else{0.0});
        self.scalar_static_f64[600]=(if (self.scalar_static_f64[599]!=0.0){self.scalar_static_f64[16]}else{self.scalar_static_f64[598]});
        self.scalar_static_f64[601]=(self.scalar_static_f64[415]*self.scalar_static_f64[105]);
        self.scalar_static_f64[602]=(self.scalar_static_f64[601]).exp();
        self.scalar_static_f64[603]=(self.scalar_static_f64[102]*self.scalar_static_f64[602]);
        self.scalar_static_f64[604]=(self.scalar_static_f64[415]*self.scalar_static_f64[107]);
        self.scalar_static_f64[605]=(self.scalar_static_f64[604]).exp();
        self.scalar_static_f64[606]=(self.scalar_static_f64[106]*self.scalar_static_f64[605]);
        self.scalar_static_bool[97]=(self.scalar_static_f64[606]<self.scalar_static_f64[16]);
        self.scalar_static_f64[607]=(if self.scalar_static_bool[97]{1.0}else{0.0});
        self.scalar_static_f64[608]=(if (self.scalar_static_f64[607]!=0.0){self.scalar_static_f64[16]}else{self.scalar_static_f64[606]});
        self.scalar_static_f64[609]=(self.scalar_static_f64[415]*self.scalar_static_f64[109]);
        self.scalar_static_f64[610]=(self.scalar_static_f64[609]).exp();
        self.scalar_static_f64[611]=(self.scalar_static_f64[108]*self.scalar_static_f64[610]);
        self.scalar_static_f64[612]=(self.scalar_static_f64[415]*self.scalar_static_f64[111]);
        self.scalar_static_f64[613]=(self.scalar_static_f64[612]).exp();
        self.scalar_static_f64[614]=(self.scalar_static_f64[110]*self.scalar_static_f64[613]);
        self.scalar_static_f64[615]=(self.scalar_static_f64[613]*self.scalar_static_f64[112]);
        self.scalar_static_f64[616]=(self.scalar_static_f64[415]*self.scalar_static_f64[114]);
        self.scalar_static_f64[617]=(self.scalar_static_f64[616]).exp();
        self.scalar_static_f64[618]=(self.scalar_static_f64[113]*self.scalar_static_f64[617]);
        self.scalar_static_f64[619]=(self.scalar_static_f64[414]*self.scalar_static_f64[115]);
        self.scalar_static_f64[620]=(1.0+self.scalar_static_f64[619]);
        self.scalar_static_f64[621]=(self.scalar_static_f64[117]*self.scalar_static_f64[620]);
        self.scalar_static_f64[622]=(if (self.scalar_static_f64[116]!=0.0){self.scalar_static_f64[621]}else{0.0});
        self.scalar_static_f64[623]=(self.scalar_static_f64[622]-1.0);
        self.scalar_static_f64[624]=(self.scalar_static_f64[623]/0.001);
        self.scalar_static_f64[625]=(if (self.scalar_static_f64[116]!=0.0){self.scalar_static_f64[624]}else{self.scalar_static_f64[564]});
        self.scalar_static_bool[98]=(self.scalar_static_f64[622]<1.0);
        self.scalar_static_f64[626]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_bool[99]=((self.scalar_static_f64[116]!=0.0)&&(self.scalar_static_f64[626]!=0.0));
        self.scalar_static_f64[627]=(self.scalar_static_f64[625]).exp();
        self.scalar_static_f64[628]=(1.0+self.scalar_static_f64[627]);
        self.scalar_static_f64[629]=(self.scalar_static_f64[628]).ln();
        self.scalar_static_f64[630]=(0.001*self.scalar_static_f64[629]);
        self.scalar_static_f64[631]=(1.0+self.scalar_static_f64[630]);
        self.scalar_static_f64[632]=(if self.scalar_static_bool[99]{self.scalar_static_f64[631]}else{self.scalar_static_f64[622]});
        self.scalar_static_bool[100]=(!(self.scalar_static_f64[626]!=0.0));
        self.scalar_static_bool[101]=((self.scalar_static_f64[116]!=0.0)&&self.scalar_static_bool[100]);
        self.scalar_static_f64[633]=(-self.scalar_static_f64[625]);
        self.scalar_static_f64[634]=(self.scalar_static_f64[633]).exp();
        self.scalar_static_f64[635]=(1.0+self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(self.scalar_static_f64[635]).ln();
        self.scalar_static_f64[637]=(0.001*self.scalar_static_f64[636]);
        self.scalar_static_f64[638]=(self.scalar_static_f64[632]+self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=(if self.scalar_static_bool[101]{self.scalar_static_f64[638]}else{self.scalar_static_f64[632]});
        self.scalar_static_f64[640]=(self.scalar_static_f64[639]-0.0006931471805599453);
        self.scalar_static_f64[641]=(if (self.scalar_static_f64[116]!=0.0){self.scalar_static_f64[640]}else{0.0});
        self.scalar_static_f64[642]=(if self.scalar_static_bool[9]{self.scalar_static_f64[117]}else{self.scalar_static_f64[641]});
        self.scalar_static_f64[643]=(self.scalar_static_f64[414]*self.scalar_static_f64[118]);
        self.scalar_static_f64[644]=(1.0+self.scalar_static_f64[643]);
        self.scalar_static_f64[645]=(self.scalar_static_f64[120]*self.scalar_static_f64[644]);
        self.scalar_static_f64[646]=(if (self.scalar_static_f64[119]!=0.0){self.scalar_static_f64[645]}else{0.0});
        self.scalar_static_f64[647]=(self.scalar_static_f64[646]-1.0);
        self.scalar_static_f64[648]=(self.scalar_static_f64[647]/0.001);
        self.scalar_static_f64[649]=(if (self.scalar_static_f64[119]!=0.0){self.scalar_static_f64[648]}else{self.scalar_static_f64[625]});
        self.scalar_static_bool[102]=(self.scalar_static_f64[646]<1.0);
        self.scalar_static_f64[650]=(if self.scalar_static_bool[102]{1.0}else{0.0});
        self.scalar_static_bool[103]=((self.scalar_static_f64[119]!=0.0)&&(self.scalar_static_f64[650]!=0.0));
        self.scalar_static_f64[651]=(self.scalar_static_f64[649]).exp();
        self.scalar_static_f64[652]=(1.0+self.scalar_static_f64[651]);
        self.scalar_static_f64[653]=(self.scalar_static_f64[652]).ln();
        self.scalar_static_f64[654]=(0.001*self.scalar_static_f64[653]);
        self.scalar_static_f64[655]=(1.0+self.scalar_static_f64[654]);
        self.scalar_static_f64[656]=(if self.scalar_static_bool[103]{self.scalar_static_f64[655]}else{self.scalar_static_f64[646]});
        self.scalar_static_bool[104]=(!(self.scalar_static_f64[650]!=0.0));
        self.scalar_static_bool[105]=((self.scalar_static_f64[119]!=0.0)&&self.scalar_static_bool[104]);
        self.scalar_static_f64[657]=(-self.scalar_static_f64[649]);
        self.scalar_static_f64[658]=(self.scalar_static_f64[657]).exp();
        self.scalar_static_f64[659]=(1.0+self.scalar_static_f64[658]);
        self.scalar_static_f64[660]=(self.scalar_static_f64[659]).ln();
        self.scalar_static_f64[661]=(0.001*self.scalar_static_f64[660]);
        self.scalar_static_f64[662]=(self.scalar_static_f64[656]+self.scalar_static_f64[661]);
        self.scalar_static_f64[663]=(if self.scalar_static_bool[105]{self.scalar_static_f64[662]}else{self.scalar_static_f64[656]});
        self.scalar_static_f64[664]=(self.scalar_static_f64[663]-0.0006931471805599453);
        self.scalar_static_f64[665]=(if (self.scalar_static_f64[119]!=0.0){self.scalar_static_f64[664]}else{0.0});
        self.scalar_static_f64[666]=(if self.scalar_static_bool[11]{self.scalar_static_f64[120]}else{self.scalar_static_f64[665]});
        self.scalar_static_f64[667]=(self.scalar_static_f64[414]*self.scalar_static_f64[122]);
        self.scalar_static_f64[668]=(1.0+self.scalar_static_f64[667]);
        self.scalar_static_f64[669]=(self.scalar_static_f64[121]*self.scalar_static_f64[668]);
        self.scalar_static_f64[670]=(self.scalar_static_f64[669]*self.scalar_static_f64[669]);
        self.scalar_static_bool[106]=(self.scalar_static_f64[669]<0.0);
        self.scalar_static_f64[671]=(if self.scalar_static_bool[106]{1.0}else{0.0});
        self.scalar_static_f64[672]=(1e-6+self.scalar_static_f64[670]);
        self.scalar_static_f64[673]=(self.scalar_static_f64[672]).sqrt();
        self.scalar_static_f64[674]=(self.scalar_static_f64[673]-self.scalar_static_f64[669]);
        self.scalar_static_f64[675]=(5e-7/self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=(if (self.scalar_static_f64[671]!=0.0){self.scalar_static_f64[675]}else{0.0});
        self.scalar_static_bool[107]=(!(self.scalar_static_f64[671]!=0.0));
        self.scalar_static_f64[677]=(self.scalar_static_f64[669]+self.scalar_static_f64[673]);
        self.scalar_static_f64[678]=(0.5*self.scalar_static_f64[677]);
        self.scalar_static_f64[679]=(if self.scalar_static_bool[107]{self.scalar_static_f64[678]}else{self.scalar_static_f64[676]});
        self.scalar_static_f64[680]=(self.scalar_static_f64[415]*self.scalar_static_f64[127]);
        self.scalar_static_f64[681]=(self.scalar_static_f64[680]/self.scalar_static_f64[642]);
        self.scalar_static_f64[682]=(self.scalar_static_f64[681]).exp();
        self.scalar_static_f64[683]=(self.scalar_static_f64[123]*self.scalar_static_f64[682]);
        self.scalar_static_f64[684]=(self.scalar_static_f64[413]*self.scalar_static_f64[128]);
        self.scalar_static_f64[685]=(self.scalar_static_f64[684]/self.scalar_static_f64[642]);
        self.scalar_static_f64[686]=(self.scalar_static_f64[685]).exp();
        self.scalar_static_f64[687]=(self.scalar_static_f64[683]*self.scalar_static_f64[686]);
        self.scalar_static_f64[688]=(self.scalar_static_f64[415]*self.scalar_static_f64[130]);
        self.scalar_static_f64[689]=(self.scalar_static_f64[688]).exp();
        self.scalar_static_f64[690]=(self.scalar_static_f64[129]*self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[415]*self.scalar_static_f64[133]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[691]).exp();
        self.scalar_static_f64[693]=(self.scalar_static_f64[131]*self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(self.scalar_static_f64[415]*self.scalar_static_f64[137]);
        self.scalar_static_f64[695]=(self.scalar_static_f64[694]).exp();
        self.scalar_static_f64[696]=(self.scalar_static_f64[134]*self.scalar_static_f64[695]);
        self.scalar_static_f64[697]=(self.scalar_static_f64[413]*self.scalar_static_f64[139]);
        self.scalar_static_f64[698]=(self.scalar_static_f64[697]/self.scalar_static_f64[135]);
        self.scalar_static_f64[699]=(self.scalar_static_f64[698]).exp();
        self.scalar_static_f64[700]=(self.scalar_static_f64[696]*self.scalar_static_f64[699]);
        self.scalar_static_f64[701]=(self.scalar_static_f64[415]*self.scalar_static_f64[143]);
        self.scalar_static_f64[702]=(self.scalar_static_f64[701]).exp();
        self.scalar_static_f64[703]=(self.scalar_static_f64[140]*self.scalar_static_f64[702]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[413]*self.scalar_static_f64[144]);
        self.scalar_static_f64[705]=(self.scalar_static_f64[704]/self.scalar_static_f64[141]);
        self.scalar_static_f64[706]=(self.scalar_static_f64[705]).exp();
        self.scalar_static_f64[707]=(self.scalar_static_f64[703]*self.scalar_static_f64[706]);
        self.scalar_static_f64[708]=(self.scalar_static_f64[415]*self.scalar_static_f64[147]);
        self.scalar_static_f64[709]=(self.scalar_static_f64[708]/self.scalar_static_f64[148]);
        self.scalar_static_f64[710]=(self.scalar_static_f64[709]).exp();
        self.scalar_static_f64[711]=(self.scalar_static_f64[145]*self.scalar_static_f64[710]);
        self.scalar_static_f64[712]=(self.scalar_static_f64[413]*self.scalar_static_f64[150]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[712]/self.scalar_static_f64[148]);
        self.scalar_static_f64[714]=(self.scalar_static_f64[713]).exp();
        self.scalar_static_f64[715]=(self.scalar_static_f64[711]*self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(self.scalar_static_f64[708]/self.scalar_static_f64[152]);
        self.scalar_static_f64[717]=(self.scalar_static_f64[716]).exp();
        self.scalar_static_f64[718]=(self.scalar_static_f64[151]*self.scalar_static_f64[717]);
        self.scalar_static_f64[719]=(self.scalar_static_f64[712]/self.scalar_static_f64[152]);
        self.scalar_static_f64[720]=(self.scalar_static_f64[719]).exp();
        self.scalar_static_f64[721]=(self.scalar_static_f64[718]*self.scalar_static_f64[720]);
        self.scalar_static_f64[722]=(self.scalar_static_f64[413]*self.scalar_static_f64[157]);
        self.scalar_static_f64[723]=(self.scalar_static_f64[722]/self.scalar_static_f64[148]);
        self.scalar_static_f64[724]=(self.scalar_static_f64[723]).exp();
        self.scalar_static_f64[725]=(self.scalar_static_f64[155]*self.scalar_static_f64[724]);
        self.scalar_static_f64[726]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[725]}else{0.0});
        self.scalar_static_f64[727]=(self.scalar_static_f64[413]*self.scalar_static_f64[160]);
        self.scalar_static_f64[728]=(self.scalar_static_f64[727]).exp();
        self.scalar_static_f64[729]=(self.scalar_static_f64[158]*self.scalar_static_f64[728]);
        self.scalar_static_f64[730]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[729]}else{0.0});
        self.scalar_static_f64[731]=(self.scalar_static_f64[413]*self.scalar_static_f64[163]);
        self.scalar_static_f64[732]=(self.scalar_static_f64[731]/self.scalar_static_f64[152]);
        self.scalar_static_f64[733]=(self.scalar_static_f64[732]).exp();
        self.scalar_static_f64[734]=(self.scalar_static_f64[161]*self.scalar_static_f64[733]);
        self.scalar_static_f64[735]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[734]}else{0.0});
        self.scalar_static_f64[736]=(self.scalar_static_f64[415]*self.scalar_static_f64[166]);
        self.scalar_static_f64[737]=(self.scalar_static_f64[736]).exp();
        self.scalar_static_f64[738]=(self.scalar_static_f64[164]*self.scalar_static_f64[737]);
        self.scalar_static_f64[739]=(self.scalar_static_f64[413]*self.scalar_static_f64[168]);
        self.scalar_static_f64[740]=(self.scalar_static_f64[739]).exp();
        self.scalar_static_f64[741]=(self.scalar_static_f64[738]*self.scalar_static_f64[740]);
        self.scalar_static_f64[742]=(self.scalar_static_f64[415]*self.scalar_static_f64[172]);
        self.scalar_static_f64[743]=(self.scalar_static_f64[742]).exp();
        self.scalar_static_f64[744]=(self.scalar_static_f64[169]*self.scalar_static_f64[743]);
        self.scalar_static_f64[745]=(self.scalar_static_f64[697]/self.scalar_static_f64[170]);
        self.scalar_static_f64[746]=(self.scalar_static_f64[745]).exp();
        self.scalar_static_f64[747]=(self.scalar_static_f64[744]*self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=(self.scalar_static_f64[415]*self.scalar_static_f64[175]);
        self.scalar_static_f64[749]=(self.scalar_static_f64[748]).exp();
        self.scalar_static_f64[750]=(self.scalar_static_f64[173]*self.scalar_static_f64[749]);
        self.scalar_static_f64[751]=(self.scalar_static_f64[697]/self.scalar_static_f64[174]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[751]).exp();
        self.scalar_static_f64[753]=(self.scalar_static_f64[750]*self.scalar_static_f64[752]);
        self.scalar_static_f64[754]=(self.scalar_static_f64[410]).sqrt();
        self.scalar_static_f64[755]=(self.scalar_static_f64[176]*self.scalar_static_f64[754]);
        self.scalar_static_f64[756]=(self.scalar_static_f64[414]*self.scalar_static_f64[177]);
        self.scalar_static_f64[757]=(self.scalar_static_f64[756]).exp();
        self.scalar_static_f64[758]=(self.scalar_static_f64[755]*self.scalar_static_f64[757]);
        self.scalar_static_f64[759]=(self.scalar_static_f64[47]*self.scalar_static_f64[436]);
        self.scalar_static_f64[760]=f64::powf(self.scalar_static_f64[759],-0.5);
        self.scalar_static_f64[761]=(1.0/self.scalar_static_f64[582]);
        self.scalar_static_f64[762]=(self.scalar_static_f64[436]*self.scalar_static_f64[178]);
        self.scalar_static_f64[763]=(self.scalar_static_f64[436]*self.scalar_static_f64[762]);
        self.scalar_static_f64[764]=(self.scalar_static_f64[760]*self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=(self.scalar_static_f64[761]*self.scalar_static_f64[764]);
        self.scalar_static_f64[766]=(self.scalar_static_f64[48]*self.scalar_static_f64[765]);
        self.scalar_static_f64[767]=(self.scalar_static_f64[579]*self.scalar_static_f64[766]);
        self.scalar_static_f64[768]=(self.scalar_static_f64[47]*self.scalar_static_f64[767]);
        self.scalar_static_f64[769]=(self.scalar_static_f64[47]*self.scalar_static_f64[768]);
        self.scalar_static_f64[770]=(self.scalar_static_f64[760]*self.scalar_static_f64[179]);
        self.scalar_static_f64[771]=(self.scalar_static_f64[480]*self.scalar_static_f64[770]);
        self.scalar_static_f64[772]=(self.scalar_static_f64[480]*self.scalar_static_f64[771]);
        self.scalar_static_f64[773]=(self.scalar_static_f64[49]*self.scalar_static_f64[772]);
        self.scalar_static_f64[774]=(self.scalar_static_f64[49]*self.scalar_static_f64[773]);
        self.scalar_static_f64[775]=(self.scalar_static_f64[582]*self.scalar_static_f64[774]);
        self.scalar_static_f64[776]=(self.scalar_static_f64[178]-self.scalar_static_f64[769]);
        self.scalar_static_f64[777]=(self.scalar_static_f64[776]).exp();
        self.scalar_static_f64[778]=(self.scalar_static_f64[775]*self.scalar_static_f64[777]);
        self.scalar_static_f64[779]=(self.scalar_static_f64[79]*self.scalar_static_f64[457]);
        self.scalar_static_f64[780]=f64::powf(self.scalar_static_f64[779],-0.5);
        self.scalar_static_f64[781]=(1.0/self.scalar_static_f64[584]);
        self.scalar_static_f64[782]=(self.scalar_static_f64[457]*self.scalar_static_f64[180]);
        self.scalar_static_f64[783]=(self.scalar_static_f64[457]*self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=(self.scalar_static_f64[780]*self.scalar_static_f64[783]);
        self.scalar_static_f64[785]=(self.scalar_static_f64[781]*self.scalar_static_f64[784]);
        self.scalar_static_f64[786]=(self.scalar_static_f64[50]*self.scalar_static_f64[785]);
        self.scalar_static_f64[787]=(self.scalar_static_f64[580]*self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=(self.scalar_static_f64[79]*self.scalar_static_f64[787]);
        self.scalar_static_f64[789]=(self.scalar_static_f64[79]*self.scalar_static_f64[788]);
        self.scalar_static_f64[790]=(self.scalar_static_f64[780]*self.scalar_static_f64[181]);
        self.scalar_static_f64[791]=(self.scalar_static_f64[538]*self.scalar_static_f64[790]);
        self.scalar_static_f64[792]=(self.scalar_static_f64[538]*self.scalar_static_f64[791]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[80]*self.scalar_static_f64[792]);
        self.scalar_static_f64[794]=(self.scalar_static_f64[80]*self.scalar_static_f64[793]);
        self.scalar_static_f64[795]=(self.scalar_static_f64[584]*self.scalar_static_f64[794]);
        self.scalar_static_f64[796]=(self.scalar_static_f64[180]-self.scalar_static_f64[789]);
        self.scalar_static_f64[797]=(self.scalar_static_f64[796]).exp();
        self.scalar_static_f64[798]=(self.scalar_static_f64[795]*self.scalar_static_f64[797]);
        self.scalar_static_f64[799]=(self.scalar_static_f64[415]*self.scalar_static_f64[104]);
        self.scalar_static_f64[800]=(self.scalar_static_f64[799]).exp();
        self.scalar_static_f64[801]=(self.scalar_static_f64[800]*self.scalar_static_f64[182]);
        self.scalar_static_f64[802]=(self.scalar_static_f64[593]*self.scalar_static_f64[801]);
        self.scalar_static_f64[803]=(self.scalar_static_f64[800]*self.scalar_static_f64[183]);
        self.scalar_static_f64[804]=(self.scalar_static_f64[761]*self.scalar_static_f64[803]);
        self.scalar_static_f64[805]=(self.scalar_static_f64[415]*self.scalar_static_f64[186]);
        self.scalar_static_f64[806]=(self.scalar_static_f64[805]).exp();
        self.scalar_static_f64[807]=(self.scalar_static_f64[184]*self.scalar_static_f64[806]);
        self.scalar_static_f64[808]=(self.scalar_static_f64[413]*self.scalar_static_f64[187]);
        self.scalar_static_f64[809]=(self.scalar_static_f64[808]).exp();
        self.scalar_static_f64[810]=(self.scalar_static_f64[807]*self.scalar_static_f64[809]);
        self.scalar_static_f64[811]=(self.scalar_static_f64[415]*self.scalar_static_f64[190]);
        self.scalar_static_f64[812]=(self.scalar_static_f64[811]).exp();
        self.scalar_static_f64[813]=(self.scalar_static_f64[18]*self.scalar_static_f64[812]);
        self.scalar_static_f64[814]=(self.scalar_static_f64[809]*self.scalar_static_f64[813]);
        self.scalar_static_f64[815]=(self.scalar_static_f64[415]*self.scalar_static_f64[192]);
        self.scalar_static_f64[816]=(self.scalar_static_f64[815]).exp();
        self.scalar_static_f64[817]=(self.scalar_static_f64[191]*self.scalar_static_f64[816]);
        self.scalar_static_f64[818]=(self.scalar_static_f64[415]*self.scalar_static_f64[194]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[818]).exp();
        self.scalar_static_f64[820]=(self.scalar_static_f64[193]*self.scalar_static_f64[819]);
        self.scalar_static_f64[821]=(self.scalar_static_f64[415]*self.scalar_static_f64[196]);
        self.scalar_static_f64[822]=(self.scalar_static_f64[821]).exp();
        self.scalar_static_f64[823]=(self.scalar_static_f64[195]*self.scalar_static_f64[822]);
        self.scalar_static_f64[824]=(self.scalar_static_f64[413]*self.scalar_static_f64[198]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[824]).exp();
        self.scalar_static_f64[826]=(self.scalar_static_f64[823]*self.scalar_static_f64[825]);
        self.scalar_static_f64[827]=(self.scalar_static_f64[415]*self.scalar_static_f64[201]);
        self.scalar_static_f64[828]=(self.scalar_static_f64[827]).exp();
        self.scalar_static_f64[829]=(self.scalar_static_f64[199]*self.scalar_static_f64[828]);
        self.scalar_static_f64[830]=(self.scalar_static_f64[415]*self.scalar_static_f64[203]);
        self.scalar_static_f64[831]=(self.scalar_static_f64[830]).exp();
        self.scalar_static_f64[832]=(self.scalar_static_f64[202]*self.scalar_static_f64[831]);
        self.scalar_static_f64[833]=(self.scalar_static_f64[829]+self.scalar_static_f64[832]);
        self.scalar_static_f64[834]=(self.scalar_static_f64[204]*self.scalar_static_f64[833]);
        self.scalar_static_f64[835]=(self.scalar_static_f64[834]/self.scalar_static_f64[205]);
        self.scalar_static_f64[836]=(self.scalar_static_f64[415]*self.scalar_static_f64[208]);
        self.scalar_static_f64[837]=(self.scalar_static_f64[836]).exp();
        self.scalar_static_f64[838]=(self.scalar_static_f64[206]*self.scalar_static_f64[837]);
        self.scalar_static_f64[839]=(self.scalar_static_f64[409]-300.0);
        self.scalar_static_bool[108]=(self.scalar_static_f64[409]<525.0);
        self.scalar_static_f64[840]=(if self.scalar_static_bool[108]{1.0}else{0.0});
        self.scalar_static_f64[841]=(self.scalar_static_f64[839]*0.00072);
        self.scalar_static_f64[842]=(1.0+self.scalar_static_f64[841]);
        self.scalar_static_f64[843]=(self.scalar_static_f64[839]*1.6e-6);
        self.scalar_static_f64[844]=(self.scalar_static_f64[839]*self.scalar_static_f64[843]);
        self.scalar_static_f64[845]=(self.scalar_static_f64[842]-self.scalar_static_f64[844]);
        self.scalar_static_f64[846]=(self.scalar_static_f64[5]*self.scalar_static_f64[845]);
        self.scalar_static_f64[847]=(if (self.scalar_static_f64[840]!=0.0){self.scalar_static_f64[846]}else{0.0});
        self.scalar_static_bool[109]=(!(self.scalar_static_f64[840]!=0.0));
        self.scalar_static_f64[848]=(if self.scalar_static_bool[109]{self.scalar_static_f64[209]}else{self.scalar_static_f64[847]});
        self.scalar_static_f64[849]=(self.scalar_static_f64[800]*self.scalar_static_f64[210]);
        self.scalar_static_f64[850]=(1.0/self.scalar_static_f64[611]);
        self.scalar_static_f64[851]=(if (self.scalar_static_f64[211]!=0.0){self.scalar_static_f64[850]}else{0.0});
        self.scalar_static_bool[110]=(self.scalar_static_f64[851]>self.scalar_static_f64[17]);
        self.scalar_static_f64[852]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_bool[111]=((self.scalar_static_f64[211]!=0.0)&&(self.scalar_static_f64[852]!=0.0));
        self.scalar_static_f64[853]=(if self.scalar_static_bool[111]{self.scalar_static_f64[17]}else{self.scalar_static_f64[851]});
        self.scalar_static_f64[854]=(if self.scalar_static_bool[14]{0.0}else{self.scalar_static_f64[853]});
        self.scalar_static_f64[855]=(1.0/self.scalar_static_f64[614]);
        self.scalar_static_f64[856]=(if (self.scalar_static_f64[212]!=0.0){self.scalar_static_f64[855]}else{0.0});
        self.scalar_static_bool[112]=(self.scalar_static_f64[856]>self.scalar_static_f64[17]);
        self.scalar_static_f64[857]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_bool[113]=((self.scalar_static_f64[212]!=0.0)&&(self.scalar_static_f64[857]!=0.0));
        self.scalar_static_f64[858]=(if self.scalar_static_bool[113]{self.scalar_static_f64[17]}else{self.scalar_static_f64[856]});
        self.scalar_static_f64[859]=(if self.scalar_static_bool[16]{0.0}else{self.scalar_static_f64[858]});
        self.scalar_static_f64[860]=(1.0/self.scalar_static_f64[615]);
        self.scalar_static_f64[861]=(if (self.scalar_static_f64[213]!=0.0){self.scalar_static_f64[860]}else{0.0});
        self.scalar_static_bool[114]=(self.scalar_static_f64[861]>self.scalar_static_f64[17]);
        self.scalar_static_f64[862]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_bool[115]=((self.scalar_static_f64[213]!=0.0)&&(self.scalar_static_f64[862]!=0.0));
        self.scalar_static_f64[863]=(if self.scalar_static_bool[115]{self.scalar_static_f64[17]}else{self.scalar_static_f64[861]});
        self.scalar_static_f64[864]=(if self.scalar_static_bool[18]{0.0}else{self.scalar_static_f64[863]});
        self.scalar_static_f64[865]=(2.0*self.scalar_static_f64[411]);
        self.scalar_static_f64[866]=(self.scalar_static_f64[500]*0.2);
        self.scalar_static_f64[867]=(self.scalar_static_f64[618]*self.scalar_static_f64[217]);
        self.scalar_static_f64[868]=(self.scalar_static_f64[412]*self.scalar_static_f64[500]);
        self.scalar_static_f64[869]=(self.scalar_static_f64[868]).exp();
        self.scalar_static_f64[870]=(self.scalar_static_f64[618]*self.scalar_static_f64[218]);
        self.scalar_static_f64[871]=(self.scalar_static_f64[217]*self.scalar_static_f64[870]);
        self.scalar_static_f64[872]=(0.1*self.scalar_static_f64[538]);
        self.scalar_static_f64[873]=(self.scalar_static_f64[411]*1e-5);
        self.scalar_static_f64[874]=(self.scalar_static_f64[411]*1e-40);
        self.scalar_static_f64[875]=(self.scalar_static_f64[480]*self.scalar_static_f64[233]);
        self.scalar_static_f64[876]=(0.1*self.scalar_static_f64[480]);
        self.scalar_static_f64[877]=(self.scalar_static_f64[480]/self.scalar_static_f64[234]);
        self.scalar_static_f64[878]=(2.0-self.scalar_static_f64[595]);
        self.scalar_static_f64[879]=(1.0-self.scalar_static_f64[595]);
        self.scalar_static_f64[880]=(self.scalar_static_f64[878]/self.scalar_static_f64[879]);
        self.scalar_static_f64[881]=f64::powf(self.scalar_static_f64[880],self.scalar_static_f64[238]);
        self.scalar_static_f64[882]=(1.0-self.scalar_static_f64[881]);
        self.scalar_static_f64[883]=(self.scalar_static_f64[538]*self.scalar_static_f64[882]);
        self.scalar_static_f64[884]=(self.scalar_static_f64[538]/self.scalar_static_f64[240]);
        self.scalar_static_f64[885]=(4.0*self.scalar_static_f64[687]);
        self.scalar_static_f64[886]=(self.scalar_static_f64[885]/self.scalar_static_f64[690]);
        self.scalar_static_f64[887]=(1.0/self.scalar_static_f64[666]);
        self.scalar_static_f64[888]=(self.scalar_static_f64[412]*self.scalar_static_f64[849]);
        self.scalar_static_f64[889]=(self.scalar_static_f64[888]).exp();
        self.scalar_static_f64[890]=(self.scalar_static_f64[889]-1.0);
        self.scalar_static_f64[891]=(self.scalar_static_f64[687]*self.scalar_static_f64[242]);
        self.scalar_static_f64[892]=(2.0*self.scalar_static_f64[726]);
        self.scalar_static_f64[893]=(2.0*self.scalar_static_f64[735]);
        self.scalar_static_f64[894]=(2.0*self.scalar_static_f64[778]);
        self.scalar_static_f64[895]=(2.0*self.scalar_static_f64[798]);
        self.scalar_static_f64[896]=(2.0*self.scalar_static_f64[741]);
        self.scalar_static_f64[897]=(4.0*self.scalar_static_f64[741]);
        self.scalar_static_f64[898]=(self.scalar_static_f64[897]/self.scalar_static_f64[693]);
        self.scalar_static_f64[899]=(self.scalar_static_f64[810]*self.scalar_static_f64[260]);
        self.scalar_static_f64[900]=(self.scalar_static_f64[810]/self.scalar_static_f64[817]);
        self.scalar_static_f64[901]=(4.0*self.scalar_static_f64[900]);
        self.scalar_static_f64[902]=(self.scalar_static_f64[810]*self.scalar_static_f64[263]);
        self.scalar_static_f64[903]=(2.0*self.scalar_static_f64[814]);
        self.scalar_static_f64[904]=(self.scalar_static_f64[814]/self.scalar_static_f64[820]);
        self.scalar_static_f64[905]=(self.scalar_static_f64[264]*self.scalar_static_f64[904]);
        self.scalar_static_f64[906]=(self.scalar_static_f64[741]*self.scalar_static_f64[267]);
        self.scalar_static_f64[907]=(self.scalar_static_f64[810]*self.scalar_static_f64[269]);
        self.scalar_static_f64[908]=(4.0*self.scalar_static_f64[810]);
        self.scalar_static_f64[909]=(self.scalar_static_f64[908]/self.scalar_static_f64[817]);
        self.scalar_static_f64[910]=(self.scalar_static_f64[741]+self.scalar_static_f64[810]);
        self.scalar_static_f64[911]=(self.scalar_static_f64[6]*self.scalar_static_f64[910]);
        self.scalar_static_f64[912]=(self.scalar_static_f64[611]*self.scalar_static_f64[911]);
        self.scalar_static_f64[913]=(if self.scalar_static_bool[48]{self.scalar_static_f64[912]}else{0.0});
        self.scalar_static_f64[914]=(self.scalar_static_f64[412]*self.scalar_static_f64[913]);
        self.scalar_static_f64[915]=(self.scalar_static_f64[914]).ln();
        self.scalar_static_f64[916]=(2.0-self.scalar_static_f64[915]);
        self.scalar_static_f64[917]=(self.scalar_static_f64[411]*self.scalar_static_f64[916]);
        self.scalar_static_f64[918]=(if self.scalar_static_bool[48]{self.scalar_static_f64[917]}else{0.0});
        self.scalar_static_f64[919]=(-self.scalar_static_f64[679]);
        self.scalar_static_f64[920]=(self.scalar_static_f64[297]/self.scalar_static_f64[679]);
        self.scalar_static_f64[921]=(self.scalar_static_f64[4]/self.scalar_static_f64[848]);
        self.scalar_static_f64[922]=(-self.scalar_static_f64[848]);
        self.scalar_static_f64[923]=(self.scalar_static_f64[585]*self.scalar_static_f64[320]);
        self.scalar_static_f64[924]=(self.scalar_static_f64[585]*self.scalar_static_f64[319]);
        self.scalar_static_f64[925]=(self.scalar_static_f64[594]*self.scalar_static_f64[321]);
        self.scalar_static_f64[926]=(self.scalar_static_f64[690]*self.scalar_static_f64[829]);
        self.scalar_static_f64[927]=(0.5*self.scalar_static_f64[926]);
        self.scalar_static_f64[928]=(0.1*self.scalar_static_f64[578]);
        self.scalar_static_f64[929]=(self.scalar_static_f64[578]*self.scalar_static_f64[325]);
        self.scalar_static_f64[930]=(self.scalar_static_f64[578]/self.scalar_static_f64[326]);
        self.scalar_static_f64[931]=(self.scalar_static_f64[690]*self.scalar_static_f64[826]);
        self.scalar_static_f64[932]=(self.scalar_static_f64[687]/self.scalar_static_f64[690]);
        self.scalar_static_f64[933]=f64::powf(self.scalar_static_f64[932],self.scalar_static_f64[328]);
        self.scalar_static_f64[934]=(self.scalar_static_f64[931]*self.scalar_static_f64[933]);
        self.scalar_static_f64[935]=(self.scalar_static_f64[411]*self.scalar_static_f64[327]);
        self.scalar_static_f64[936]=(4.0*self.scalar_static_f64[832]);
        self.scalar_static_f64[937]=(self.scalar_static_f64[411]*self.scalar_static_f64[936]);
        self.scalar_static_f64[938]=(self.scalar_static_f64[937]/self.scalar_static_f64[618]);
        self.scalar_static_f64[939]=(0.5*self.scalar_static_f64[938]);
        self.scalar_static_f64[940]=(0.5*self.scalar_static_f64[835]);
        self.scalar_static_f64[941]=(self.scalar_static_f64[838]*self.scalar_static_f64[896]);
        self.scalar_static_f64[942]=(self.scalar_static_f64[835]*self.scalar_static_f64[333]);
        self.scalar_static_f64[943]=(self.scalar_static_f64[838]*self.scalar_static_f64[906]);
        self.scalar_static_f64[944]=(self.scalar_static_f64[0]*self.scalar_static_f64[412]);
        self.scalar_static_f64[945]=(self.scalar_static_f64[412]*self.scalar_static_f64[351]);
        self.scalar_static_f64[946]=(self.scalar_static_f64[945]/self.scalar_static_f64[642]);
        self.scalar_static_f64[947]=(self.scalar_static_f64[944]/self.scalar_static_f64[642]);
        self.scalar_static_f64[948]=(self.scalar_static_f64[412]*self.scalar_static_f64[352]);
        self.scalar_static_f64[949]=(self.scalar_static_f64[412]*self.scalar_static_f64[353]);
        self.scalar_static_f64[950]=(self.scalar_static_f64[412]*self.scalar_static_f64[354]);
        self.scalar_static_f64[951]=(self.scalar_static_f64[351]/self.scalar_static_f64[876]);
        self.scalar_static_f64[952]=(self.scalar_static_f64[0]/self.scalar_static_f64[876]);
        self.scalar_static_f64[953]=(-self.scalar_static_f64[951]);
        self.scalar_static_f64[954]=(-self.scalar_static_f64[952]);
        self.scalar_static_f64[955]=(self.scalar_static_f64[0]*self.scalar_static_f64[595]);
        self.scalar_static_f64[956]=(self.scalar_static_f64[595]*self.scalar_static_f64[351]);
        self.scalar_static_f64[957]=(self.scalar_static_f64[887]-1.0);
        self.scalar_static_f64[958]=(self.scalar_static_f64[945]/self.scalar_static_f64[148]);
        self.scalar_static_f64[959]=(self.scalar_static_f64[944]/self.scalar_static_f64[148]);
        self.scalar_static_f64[960]=(self.scalar_static_f64[945]/self.scalar_static_f64[152]);
        self.scalar_static_f64[961]=(self.scalar_static_f64[944]/self.scalar_static_f64[152]);
        self.scalar_static_f64[962]=(self.scalar_static_f64[945]/self.scalar_static_f64[135]);
        self.scalar_static_f64[963]=(self.scalar_static_f64[944]/self.scalar_static_f64[135]);
        self.scalar_static_f64[964]=(self.scalar_static_f64[945]/self.scalar_static_f64[170]);
        self.scalar_static_f64[965]=(self.scalar_static_f64[944]/self.scalar_static_f64[170]);
        self.scalar_static_f64[966]=(self.scalar_static_f64[944]/self.scalar_static_f64[141]);
        self.scalar_static_f64[967]=(self.scalar_static_f64[948]/self.scalar_static_f64[141]);
        self.scalar_static_f64[968]=(self.scalar_static_f64[949]/self.scalar_static_f64[141]);
        self.scalar_static_f64[969]=(self.scalar_static_f64[945]/self.scalar_static_f64[141]);
        self.scalar_static_f64[970]=(self.scalar_static_f64[945]/self.scalar_static_f64[174]);
        self.scalar_static_f64[971]=(self.scalar_static_f64[944]/self.scalar_static_f64[174]);
        self.scalar_static_f64[972]=(self.scalar_static_f64[579]*self.scalar_static_f64[351]);
        self.scalar_static_f64[973]=(self.scalar_static_f64[0]*self.scalar_static_f64[579]);
        self.scalar_static_f64[974]=(self.scalar_static_f64[769]*self.scalar_static_f64[369]);
        self.scalar_static_f64[975]=(self.scalar_static_f64[769]*self.scalar_static_f64[370]);
        self.scalar_static_f64[976]=(self.scalar_static_f64[0]*self.scalar_static_f64[580]);
        self.scalar_static_f64[977]=(self.scalar_static_f64[580]*self.scalar_static_f64[351]);
        self.scalar_static_f64[978]=(-self.scalar_static_f64[976]);
        self.scalar_static_f64[979]=(-self.scalar_static_f64[977]);
        self.scalar_static_f64[980]=(self.scalar_static_f64[789]*self.scalar_static_f64[374]);
        self.scalar_static_f64[981]=(self.scalar_static_f64[789]*self.scalar_static_f64[375]);
        self.scalar_static_f64[982]=(self.scalar_static_f64[920]*self.scalar_static_f64[351]);
        self.scalar_static_f64[983]=(self.scalar_static_f64[0]*self.scalar_static_f64[920]);
        self.scalar_static_f64[984]=(self.scalar_static_f64[0]/self.scalar_static_f64[872]);
        self.scalar_static_f64[985]=(self.scalar_static_f64[352]/self.scalar_static_f64[872]);
        self.scalar_static_f64[986]=(self.scalar_static_f64[353]/self.scalar_static_f64[872]);
        self.scalar_static_f64[987]=(self.scalar_static_f64[351]/self.scalar_static_f64[872]);
        self.scalar_static_f64[988]=(-self.scalar_static_f64[984]);
        self.scalar_static_f64[989]=(-self.scalar_static_f64[985]);
        self.scalar_static_f64[990]=(-self.scalar_static_f64[986]);
        self.scalar_static_f64[991]=(-self.scalar_static_f64[987]);
        self.scalar_static_f64[992]=(self.scalar_static_f64[595]*self.scalar_static_f64[352]);
        self.scalar_static_f64[993]=(self.scalar_static_f64[595]*self.scalar_static_f64[353]);
        self.scalar_static_f64[994]=(self.scalar_static_f64[354]/self.scalar_static_f64[872]);
        self.scalar_static_f64[995]=(-self.scalar_static_f64[994]);
        self.scalar_static_f64[996]=(self.scalar_static_f64[595]*self.scalar_static_f64[354]);
        self.scalar_static_f64[997]=(self.scalar_static_f64[0]/self.scalar_static_f64[928]);
        self.scalar_static_f64[998]=(self.scalar_static_f64[351]/self.scalar_static_f64[928]);
        self.scalar_static_f64[999]=(-self.scalar_static_f64[997]);
        self.scalar_static_f64[1000]=(-self.scalar_static_f64[998]);
        self.scalar_static_f64[1001]=(self.scalar_static_f64[351]/self.scalar_static_f64[935]);
        self.scalar_static_f64[1002]=(self.scalar_static_f64[0]/self.scalar_static_f64[935]);
        self.scalar_static_f64[1003]=(self.scalar_static_f64[412]*self.scalar_static_f64[392]);
        self.scalar_static_f64[1004]=(self.scalar_static_f64[412]*self.scalar_static_f64[393]);
        self.scalar_static_f64[1005]=(self.scalar_static_f64[412]*self.scalar_static_f64[394]);
        self.scalar_static_f64[1006]=(self.scalar_static_f64[412]*self.scalar_static_f64[395]);
        self.scalar_static_f64[1007]=(if (self.scalar_static_f64[335]!=0.0){self.scalar_static_f64[951]}else{0.0});
        self.scalar_static_f64[1008]=(if (self.scalar_static_f64[335]!=0.0){self.scalar_static_f64[952]}else{0.0});
        self.scalar_static_f64[1009]=(-self.scalar_static_f64[1007]);
        self.scalar_static_f64[1010]=(-self.scalar_static_f64[1008]);
        self.scalar_static_f64[1011]=(self.scalar_static_f64[401]/self.scalar_static_f64[600]);
        self.scalar_static_f64[1012]=(self.scalar_static_f64[402]/self.scalar_static_f64[600]);
        self.scalar_static_f64[1013]=(self.scalar_static_f64[15]*self.scalar_static_f64[1011]);
        self.scalar_static_f64[1014]=(self.scalar_static_f64[15]*self.scalar_static_f64[1012]);
        self.scalar_static_f64[1015]=(self.scalar_static_f64[401]/self.scalar_static_f64[608]);
        self.scalar_static_f64[1016]=(self.scalar_static_f64[402]/self.scalar_static_f64[608]);
        self.scalar_static_f64[1017]=(self.scalar_static_f64[15]*self.scalar_static_f64[1015]);
        self.scalar_static_f64[1018]=(self.scalar_static_f64[15]*self.scalar_static_f64[1016]);
        self.scalar_static_f64[1019]=(self.scalar_static_f64[854]*self.scalar_static_f64[401]);
        self.scalar_static_f64[1020]=(self.scalar_static_f64[854]*self.scalar_static_f64[407]);
        self.scalar_static_f64[1021]=(self.scalar_static_f64[854]*self.scalar_static_f64[408]);
        self.scalar_static_f64[1022]=(self.scalar_static_f64[854]*self.scalar_static_f64[402]);
        self.scalar_static_f64[1023]=(self.scalar_static_f64[15]*self.scalar_static_f64[1019]);
        self.scalar_static_f64[1024]=(self.scalar_static_f64[15]*self.scalar_static_f64[1020]);
        self.scalar_static_f64[1025]=(self.scalar_static_f64[15]*self.scalar_static_f64[1021]);
        self.scalar_static_f64[1026]=(self.scalar_static_f64[15]*self.scalar_static_f64[1022]);
        self.scalar_static_f64[1027]=(self.scalar_static_f64[859]*self.scalar_static_f64[401]);
        self.scalar_static_f64[1028]=(self.scalar_static_f64[859]*self.scalar_static_f64[402]);
        self.scalar_static_f64[1029]=(self.scalar_static_f64[15]*self.scalar_static_f64[1027]);
        self.scalar_static_f64[1030]=(self.scalar_static_f64[15]*self.scalar_static_f64[1028]);
        self.scalar_static_f64[1031]=(if (self.scalar_static_f64[212]!=0.0){self.scalar_static_f64[1029]}else{0.0});
        self.scalar_static_f64[1032]=(if (self.scalar_static_f64[212]!=0.0){self.scalar_static_f64[1030]}else{0.0});
        self.scalar_static_f64[1033]=(self.scalar_static_f64[864]*self.scalar_static_f64[402]);
        self.scalar_static_f64[1034]=(self.scalar_static_f64[864]*self.scalar_static_f64[401]);
        self.scalar_static_f64[1035]=(self.scalar_static_f64[15]*self.scalar_static_f64[1033]);
        self.scalar_static_f64[1036]=(self.scalar_static_f64[15]*self.scalar_static_f64[1034]);
        self.scalar_static_f64[1037]=(if (self.scalar_static_f64[213]!=0.0){self.scalar_static_f64[1035]}else{0.0});
        self.scalar_static_f64[1038]=(if (self.scalar_static_f64[213]!=0.0){self.scalar_static_f64[1036]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
