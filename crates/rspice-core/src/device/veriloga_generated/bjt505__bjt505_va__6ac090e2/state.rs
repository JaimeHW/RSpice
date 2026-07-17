#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState};

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
            let params = &*ptr;
            for index in 0..PARAMETER_DISPLAY_NAMES.len() {
                let value = read_parameter_slot(params, index);
                validate_parameter_metadata(params, index, value).expect("generated Verilog-A parameter defaults must satisfy declared ranges");
            }
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

#[inline]
fn read_parameter_slot(parameters: &Parameters, index: usize) -> f64 {
    debug_assert!(index < PARAMETER_DISPLAY_NAMES.len(), "generated parameter index out of range");
    // SAFETY: Parameters is repr(C), contains only f64 fields, and every caller validates or generates the index.
    unsafe { *((parameters as *const Parameters as *const f64).add(index)) }
}

fn validate_parameter_scalar_metadata(index: usize, value: f64) -> Result<(), String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter index {} is out of range", index));
    };
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    validate_parameter_bounds(
        name,
        value,
        flags,
        PARAMETER_MIN_BOUNDS[index],
        PARAMETER_MAX_BOUNDS[index],
        PARAMETER_EXCLUDED_BOUNDS[index],
    )
}

fn validate_parameter_metadata(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    validate_parameter_scalar_metadata(index, value)?;
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    let computed_min = parameter_computed_min_bound(parameters, index)?;
    let lower_source_count = usize::from(PARAMETER_MIN_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MIN_REFERENCES[index].is_some())
        + usize::from(computed_min.is_some());
    if lower_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting lower-bound sources", name));
    }
    let min = match PARAMETER_MIN_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_min.or(PARAMETER_MIN_BOUNDS[index]),
    };
    let computed_max = parameter_computed_max_bound(parameters, index)?;
    let upper_source_count = usize::from(PARAMETER_MAX_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MAX_REFERENCES[index].is_some())
        + usize::from(computed_max.is_some());
    if upper_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting upper-bound sources", name));
    }
    let max = match PARAMETER_MAX_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_max.or(PARAMETER_MAX_BOUNDS[index]),
    };
    if let (Some(min), Some(max)) = (min, max) {
        let empty = min.value > max.value
            || (min.value == max.value
                && flags & (PARAMETER_MIN_EXCLUSIVE_FLAG | PARAMETER_MAX_EXCLUSIVE_FLAG) != 0);
        if empty {
            return Err(format!(
                "parameter '{}' has an empty range: lower bound {}={} exceeds upper bound {}={}",
                name, min.label, min.value, max.label, max.value
            ));
        }
    }
    validate_parameter_bounds(name, value, flags, min, max, PARAMETER_EXCLUDED_BOUNDS[index])?;
    for &reference in PARAMETER_EXCLUDED_REFERENCES[index] {
        let excluded = parameter_bound_from_reference(parameters, reference)?;
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}={}, got {}",
                name, excluded.label, excluded.value, value
            ));
        }
    }
    validate_parameter_computed_exclusions(parameters, index, value)?;
    Ok(())
}

fn parameter_bound_from_reference(
    parameters: &Parameters,
    index: usize,
) -> Result<ParameterBound, String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter range reference {} is out of range", index));
    };
    let value = read_parameter_slot(parameters, index);
    validate_finite_parameter(name, value)?;
    Ok(ParameterBound { value, label: name })
}

fn validate_parameter_bounds(
    name: &str,
    value: f64,
    flags: u8,
    min: Option<ParameterBound>,
    max: Option<ParameterBound>,
    excluded: &[ParameterBound],
) -> Result<(), String> {
    if let Some(min) = min {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = max {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in excluded {
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

const PARAMETER_MIN_REFERENCES: [Option<usize>; 156] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 156] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
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

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 156] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
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

fn parameter_computed_min_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn parameter_computed_max_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn validate_parameter_computed_exclusions(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    let params = parameters;
    match index {
        _ => {}
    }
    Ok(())
}

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
    pub(crate) scalar_static_f64: Box<[f64; 1036]>,
    pub(crate) scalar_static_bool: Box<[bool; 118]>,
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
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "8d5ddb6f61c6200626a03b755c679fb7676c6aac44170e982a4961e4020848ad";
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
            scalar_static_f64: boxed_zero_f64_array::<1036>(),
            scalar_static_bool: boxed_zero_bool_array::<118>(),
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

    pub(crate) fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {
        GeneratedVerilogAPersistentState {
            ddt_previous: self.ddt_state_previous.to_vec(),
            ddt_older: self.ddt_state_older.to_vec(),
            ddt_derivative_previous: self.ddt_derivative_previous.to_vec(),
            ddt_initialized: self.ddt_state_initialized.to_vec(),
            idt_previous: self.idt_state_previous.to_vec(),
            idt_initialized: self.idt_state_initialized.to_vec(),
            limiter_anchor: Vec::new(),
            limiter_initialized: Vec::new(),
        }
    }

    pub(crate) fn validate_persistent_state_shape(&self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        if state.ddt_previous.len() != Self::DDT_STATE_COUNT || state.ddt_older.len() != Self::DDT_STATE_COUNT || state.ddt_derivative_previous.len() != Self::DDT_STATE_COUNT || state.ddt_initialized.len() != Self::DDT_STATE_COUNT {
            return Err(format!("generated ddt checkpoint shape mismatch: expected {}, found {} / {} / {} / {}", Self::DDT_STATE_COUNT, state.ddt_previous.len(), state.ddt_older.len(), state.ddt_derivative_previous.len(), state.ddt_initialized.len()));
        }
        if state.idt_previous.len() != Self::IDT_STATE_COUNT || state.idt_initialized.len() != Self::IDT_STATE_COUNT {
            return Err(format!("generated idt checkpoint shape mismatch: expected {}, found {} / {}", Self::IDT_STATE_COUNT, state.idt_previous.len(), state.idt_initialized.len()));
        }
        if state.ddt_previous.iter().chain(&state.ddt_older).chain(&state.ddt_derivative_previous).chain(&state.idt_previous).chain(&state.limiter_anchor).any(|value| !value.is_finite()) {
            return Err("generated Verilog-A checkpoint contains non-finite persistent state".to_string());
        }
        Ok(())
    }

    pub(crate) fn restore_persistent_state(&mut self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        self.validate_persistent_state_shape(state)?;
        self.ddt_state_previous.copy_from_slice(&state.ddt_previous);
        self.ddt_state_current.copy_from_slice(&state.ddt_previous);
        self.ddt_state_older.copy_from_slice(&state.ddt_older);
        self.ddt_derivative_previous.copy_from_slice(&state.ddt_derivative_previous);
        self.ddt_derivative_current.copy_from_slice(&state.ddt_derivative_previous);
        self.ddt_state_initialized.copy_from_slice(&state.ddt_initialized);
        self.idt_state_previous.copy_from_slice(&state.idt_previous);
        self.idt_state_current.copy_from_slice(&state.idt_previous);
        self.idt_state_initialized.copy_from_slice(&state.idt_initialized);
        Ok(())
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
        validate_parameter_scalar_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
        Ok(())
    }

    /// Validate the complete parameter vector after applying all instance overrides.
    pub fn validate_parameters(&self) -> Result<(), String> {
        for index in 0..Self::PARAMETER_COUNT {
            let value = read_parameter_slot(self.params.as_ref(), index);
            validate_parameter_metadata(self.params.as_ref(), index, value)?;
        }
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
    pub fn limiter_converged(&self) -> bool {
        true
    }

    #[inline]
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        self.scalar_static_f64[0]=p.p3;
        self.scalar_static_bool[0]=(self.scalar_static_f64[0]==1.0);
        self.scalar_static_f64[1]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[2]=(if ((self.scalar_static_f64[1])!=0.0){70300000.0}else{0.0});
        self.scalar_static_f64[3]=(if ((self.scalar_static_f64[1])!=0.0){123000000.0}else{0.0});
        self.scalar_static_bool[1]=(!((self.scalar_static_f64[1])!=0.0));
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
        self.scalar_static_f64[13]=(if ((self.scalar_static_f64[12])!=0.0){1e-12}else{0.0});
        self.scalar_static_bool[3]=(!((self.scalar_static_f64[12])!=0.0));
        self.scalar_static_f64[14]=(if self.scalar_static_bool[3]{self.scalar_static_f64[11]}else{self.scalar_static_f64[13]});
        self.scalar_static_f64[15]=p.p1;
        self.scalar_static_f64[16]=(self.scalar_static_f64[14]*self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=(1.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=p.p134;
        self.scalar_static_bool[4]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_f64[19]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_bool[5]=(!((self.scalar_static_f64[19])!=0.0));
        self.scalar_static_f64[20]=p.p67;
        self.scalar_static_f64[21]=(2.0-self.scalar_static_f64[20]);
        self.scalar_static_f64[22]=f64::powf(2.0,self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(1.0/self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=p.p114;
        self.scalar_static_f64[25]=p.p115;
        self.scalar_static_f64[26]=(self.scalar_static_f64[9]*self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=(self.scalar_static_f64[9]*self.scalar_static_f64[26]);
        self.scalar_static_f64[28]=p.p116;
        self.scalar_static_f64[29]=(self.scalar_static_f64[9]+self.scalar_static_f64[28]);
        self.scalar_static_f64[30]=(self.scalar_static_f64[27]/self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=(self.scalar_static_f64[24]+self.scalar_static_f64[30]);
        self.scalar_static_f64[32]=(self.scalar_static_f64[31]-0.05);
        self.scalar_static_f64[33]=(self.scalar_static_f64[32]/0.1);
        self.scalar_static_bool[6]=(self.scalar_static_f64[31]<0.05);
        self.scalar_static_f64[34]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_f64[35]=(self.scalar_static_f64[33]).exp();
        self.scalar_static_f64[36]=(1.0+self.scalar_static_f64[35]);
        self.scalar_static_f64[37]=(self.scalar_static_f64[36]).ln();
        self.scalar_static_f64[38]=(0.1*self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=(0.05+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=(if ((self.scalar_static_f64[34])!=0.0){self.scalar_static_f64[39]}else{0.0});
        self.scalar_static_bool[7]=(!((self.scalar_static_f64[34])!=0.0));
        self.scalar_static_f64[41]=(-self.scalar_static_f64[33]);
        self.scalar_static_f64[42]=(self.scalar_static_f64[41]).exp();
        self.scalar_static_f64[43]=(1.0+self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=(self.scalar_static_f64[43]).ln();
        self.scalar_static_f64[45]=(0.1*self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=(self.scalar_static_f64[31]+self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=(if self.scalar_static_bool[7]{self.scalar_static_f64[46]}else{self.scalar_static_f64[40]});
        self.scalar_static_f64[48]=(1.0/self.scalar_static_f64[24]);
        self.scalar_static_f64[49]=p.p66;
        self.scalar_static_f64[50]=(1.0/self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p71;
        self.scalar_static_f64[52]=p.p72;
        self.scalar_static_f64[53]=(2.0-self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=f64::powf(2.0,self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(1.0/self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=p.p117;
        self.scalar_static_f64[57]=p.p118;
        self.scalar_static_f64[58]=(self.scalar_static_f64[9]*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(self.scalar_static_f64[9]*self.scalar_static_f64[58]);
        self.scalar_static_f64[60]=p.p119;
        self.scalar_static_f64[61]=(self.scalar_static_f64[9]+self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[59]/self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(self.scalar_static_f64[56]+self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=(self.scalar_static_f64[63]-0.05);
        self.scalar_static_f64[65]=(self.scalar_static_f64[64]/0.1);
        self.scalar_static_bool[8]=(self.scalar_static_f64[63]<0.05);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[67]=(self.scalar_static_f64[65]).exp();
        self.scalar_static_f64[68]=(1.0+self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(self.scalar_static_f64[68]).ln();
        self.scalar_static_f64[70]=(0.1*self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=(0.05+self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=(if ((self.scalar_static_f64[66])!=0.0){self.scalar_static_f64[71]}else{0.0});
        self.scalar_static_bool[9]=(!((self.scalar_static_f64[66])!=0.0));
        self.scalar_static_f64[73]=(-self.scalar_static_f64[65]);
        self.scalar_static_f64[74]=(self.scalar_static_f64[73]).exp();
        self.scalar_static_f64[75]=(1.0+self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=(self.scalar_static_f64[75]).ln();
        self.scalar_static_f64[77]=(0.1*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(self.scalar_static_f64[63]+self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=(if self.scalar_static_bool[9]{self.scalar_static_f64[78]}else{self.scalar_static_f64[72]});
        self.scalar_static_f64[80]=(1.0/self.scalar_static_f64[56]);
        self.scalar_static_f64[81]=(1.0/self.scalar_static_f64[51]);
        self.scalar_static_f64[82]=p.p83;
        self.scalar_static_f64[83]=(1.0/self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(1.0-self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(self.scalar_static_f64[9]*8.617086918058125e-5);
        self.scalar_static_f64[86]=(1.0/self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=p.p105;
        self.scalar_static_f64[88]=p.p64;
        self.scalar_static_f64[89]=p.p110;
        self.scalar_static_f64[90]=p.p80;
        self.scalar_static_f64[91]=p.p27;
        self.scalar_static_f64[92]=p.p109;
        self.scalar_static_f64[93]=p.p138;
        self.scalar_static_f64[94]=p.p140;
        self.scalar_static_f64[95]=p.p65;
        self.scalar_static_f64[96]=p.p137;
        self.scalar_static_f64[97]=p.p139;
        self.scalar_static_f64[98]=p.p75;
        self.scalar_static_f64[99]=(1.0-self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=p.p70;
        self.scalar_static_f64[101]=p.p54;
        self.scalar_static_f64[102]=p.p97;
        self.scalar_static_f64[103]=p.p56;
        self.scalar_static_f64[104]=p.p98;
        self.scalar_static_f64[105]=p.p96;
        self.scalar_static_f64[106]=(self.scalar_static_f64[104]-self.scalar_static_f64[105]);
        self.scalar_static_f64[107]=p.p55;
        self.scalar_static_f64[108]=p.p101;
        self.scalar_static_f64[109]=p.p57;
        self.scalar_static_f64[110]=p.p102;
        self.scalar_static_f64[111]=p.p58;
        self.scalar_static_f64[112]=p.p104;
        self.scalar_static_f64[113]=p.p59;
        self.scalar_static_f64[114]=p.p60;
        self.scalar_static_f64[115]=p.p99;
        self.scalar_static_f64[116]=p.p122;
        self.scalar_static_bool[10]=(0.0!=self.scalar_static_f64[116]);
        self.scalar_static_f64[117]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[118]=p.p10;
        self.scalar_static_bool[11]=(!((self.scalar_static_f64[117])!=0.0));
        self.scalar_static_f64[119]=p.p123;
        self.scalar_static_bool[12]=(0.0!=self.scalar_static_f64[119]);
        self.scalar_static_f64[120]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_f64[121]=p.p11;
        self.scalar_static_bool[13]=(!((self.scalar_static_f64[120])!=0.0));
        self.scalar_static_f64[122]=p.p43;
        self.scalar_static_f64[123]=p.p124;
        self.scalar_static_f64[124]=p.p9;
        self.scalar_static_f64[125]=(4.0-self.scalar_static_f64[104]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[125]-self.scalar_static_f64[105]);
        self.scalar_static_f64[127]=p.p121;
        self.scalar_static_f64[128]=(self.scalar_static_f64[126]+self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(-self.scalar_static_f64[87]);
        self.scalar_static_f64[130]=p.p12;
        self.scalar_static_f64[131]=(1.0-self.scalar_static_f64[104]);
        self.scalar_static_f64[132]=p.p30;
        self.scalar_static_f64[133]=p.p103;
        self.scalar_static_f64[134]=(1.0-self.scalar_static_f64[133]);
        self.scalar_static_f64[135]=p.p20;
        self.scalar_static_f64[136]=p.p21;
        self.scalar_static_f64[137]=(2.0*self.scalar_static_f64[136]);
        self.scalar_static_f64[138]=(6.0-self.scalar_static_f64[137]);
        self.scalar_static_f64[139]=p.p113;
        self.scalar_static_f64[140]=(-self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=p.p31;
        self.scalar_static_f64[142]=p.p32;
        self.scalar_static_f64[143]=(2.0*self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(6.0-self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=(-self.scalar_static_f64[89]);
        self.scalar_static_f64[146]=p.p16;
        self.scalar_static_f64[147]=(4.0-self.scalar_static_f64[102]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[127]+self.scalar_static_f64[147]);
        self.scalar_static_f64[149]=p.p17;
        self.scalar_static_f64[150]=p.p111;
        self.scalar_static_f64[151]=(-self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=p.p18;
        self.scalar_static_f64[153]=p.p19;
        self.scalar_static_f64[154]=p.p24;
        self.scalar_static_bool[14]=(1.0==self.scalar_static_f64[154]);
        self.scalar_static_f64[155]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[156]=p.p25;
        self.scalar_static_f64[157]=p.p107;
        self.scalar_static_f64[158]=(-self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=p.p28;
        self.scalar_static_f64[160]=p.p106;
        self.scalar_static_f64[161]=(-self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=p.p26;
        self.scalar_static_f64[163]=p.p108;
        self.scalar_static_f64[164]=(-self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=p.p29;
        self.scalar_static_f64[166]=(4.0-self.scalar_static_f64[133]);
        self.scalar_static_f64[167]=(self.scalar_static_f64[127]+self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=p.p112;
        self.scalar_static_f64[169]=(-self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=p.p22;
        self.scalar_static_f64[171]=p.p23;
        self.scalar_static_f64[172]=(2.0*self.scalar_static_f64[171]);
        self.scalar_static_f64[173]=(6.0-self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=p.p145;
        self.scalar_static_f64[175]=p.p146;
        self.scalar_static_f64[176]=(4.0/self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=p.p151;
        self.scalar_static_f64[178]=p.p153;
        self.scalar_static_f64[179]=p.p35;
        self.scalar_static_f64[180]=p.p34;
        self.scalar_static_f64[181]=p.p37;
        self.scalar_static_f64[182]=p.p36;
        self.scalar_static_f64[183]=p.p14;
        self.scalar_static_f64[184]=p.p13;
        self.scalar_static_f64[185]=p.p133;
        self.scalar_static_f64[186]=p.p141;
        self.scalar_static_f64[187]=(4.0-self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=(-self.scalar_static_f64[94]);
        self.scalar_static_f64[189]=p.p142;
        self.scalar_static_f64[190]=(0.5*self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(3.5-self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=p.p135;
        self.scalar_static_f64[193]=(1.0-self.scalar_static_f64[186]);
        self.scalar_static_f64[194]=p.p136;
        self.scalar_static_f64[195]=(1.0-self.scalar_static_f64[189]);
        self.scalar_static_f64[196]=p.p86;
        self.scalar_static_f64[197]=(self.scalar_static_f64[104]-2.0);
        self.scalar_static_f64[198]=p.p120;
        self.scalar_static_f64[199]=(-self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=p.p87;
        self.scalar_static_f64[201]=(self.scalar_static_f64[104]+self.scalar_static_f64[105]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[201]-1.0);
        self.scalar_static_f64[203]=p.p88;
        self.scalar_static_f64[204]=(self.scalar_static_f64[115]-1.0);
        self.scalar_static_f64[205]=p.p89;
        self.scalar_static_f64[206]=(self.scalar_static_f64[200]+self.scalar_static_f64[203]);
        self.scalar_static_f64[207]=p.p90;
        self.scalar_static_f64[208]=p.p100;
        self.scalar_static_f64[209]=(self.scalar_static_f64[208]-1.0);
        self.scalar_static_f64[210]=(self.scalar_static_f64[5]*1.081);
        self.scalar_static_f64[211]=p.p92;
        self.scalar_static_bool[15]=(self.scalar_static_f64[109]>0.0);
        self.scalar_static_f64[212]=(if self.scalar_static_bool[15]{1.0}else{0.0});
        self.scalar_static_bool[16]=(!((self.scalar_static_f64[212])!=0.0));
        self.scalar_static_bool[17]=(self.scalar_static_f64[111]>0.0);
        self.scalar_static_f64[213]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=(!((self.scalar_static_f64[213])!=0.0));
        self.scalar_static_bool[19]=(self.scalar_static_f64[113]>0.0);
        self.scalar_static_f64[214]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_bool[20]=(!((self.scalar_static_f64[214])!=0.0));
        self.scalar_static_f64[215]=p.p147;
        self.scalar_static_f64[216]=(self.scalar_static_f64[215]).exp();
        self.scalar_static_f64[217]=p.p149;
        self.scalar_static_f64[218]=p.p62;
        self.scalar_static_f64[219]=p.p61;
        self.scalar_static_f64[220]=(self.scalar_static_f64[218]*self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=p.p63;
        self.scalar_static_f64[222]=(-1.0/self.scalar_static_f64[221]);
        self.scalar_static_f64[223]=(self.scalar_static_f64[222]).exp();
        self.scalar_static_f64[224]=(1.0+self.scalar_static_f64[223]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[224]).ln();
        self.scalar_static_f64[226]=(self.scalar_static_f64[221]*self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=(1.0+self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=p.p148;
        self.scalar_static_f64[229]=(0.5*self.scalar_static_f64[219]);
        self.scalar_static_f64[230]=p.p73;
        self.scalar_static_bool[21]=(0.0==self.scalar_static_f64[230]);
        self.scalar_static_f64[231]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_bool[22]=(!((self.scalar_static_f64[231])!=0.0));
        self.scalar_static_f64[232]=(-1.0/self.scalar_static_f64[20]);
        self.scalar_static_f64[233]=f64::powf(3.0,self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(1.0-self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=(1.0-self.scalar_static_f64[20]);
        self.scalar_static_f64[236]=p.p74;
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[236]);
        self.scalar_static_f64[237]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(2.0==self.scalar_static_f64[236]);
        self.scalar_static_f64[238]=(if self.scalar_static_bool[24]{1.0}else{0.0});
        self.scalar_static_bool[25]=(!((self.scalar_static_f64[237])!=0.0));
        self.scalar_static_bool[26]=(((self.scalar_static_f64[238])!=0.0)&&self.scalar_static_bool[25]);
        self.scalar_static_bool[27]=(!((self.scalar_static_f64[238])!=0.0));
        self.scalar_static_bool[28]=(self.scalar_static_bool[25]&&self.scalar_static_bool[27]);
        self.scalar_static_f64[239]=(-1.0/self.scalar_static_f64[52]);
        self.scalar_static_f64[240]=p.p76;
        self.scalar_static_f64[241]=(1.0-self.scalar_static_f64[52]);
        self.scalar_static_bool[29]=(0.0==self.scalar_static_f64[211]);
        self.scalar_static_f64[242]=(if self.scalar_static_bool[29]{1.0}else{0.0});
        self.scalar_static_bool[30]=(!((self.scalar_static_f64[242])!=0.0));
        self.scalar_static_f64[243]=p.p15;
        self.scalar_static_f64[244]=p.p152;
        self.scalar_static_f64[245]=p.p154;
        self.scalar_static_f64[246]=p.p155;
        self.scalar_static_f64[247]=p.p93;
        self.scalar_static_bool[31]=(0.0==self.scalar_static_f64[247]);
        self.scalar_static_f64[248]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(!((self.scalar_static_f64[155])!=0.0));
        self.scalar_static_bool[33]=(((self.scalar_static_f64[248])!=0.0)&&self.scalar_static_bool[32]);
        self.scalar_static_bool[34]=(!((self.scalar_static_f64[248])!=0.0));
        self.scalar_static_bool[35]=(self.scalar_static_bool[32]&&self.scalar_static_bool[34]);
        self.scalar_static_f64[249]=(1.0-self.scalar_static_f64[247]);
        self.scalar_static_bool[36]=(self.scalar_static_f64[180]>0.0);
        self.scalar_static_bool[37]=(self.scalar_static_f64[179]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_bool[36]&&self.scalar_static_bool[37]);
        self.scalar_static_f64[250]=(-2.0-self.scalar_static_f64[20]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[20]*self.scalar_static_f64[20]);
        self.scalar_static_f64[252]=(1.0-self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[20]-1.0);
        self.scalar_static_bool[39]=(self.scalar_static_f64[182]>0.0);
        self.scalar_static_bool[40]=(self.scalar_static_f64[181]>0.0);
        self.scalar_static_bool[41]=(self.scalar_static_bool[39]&&self.scalar_static_bool[40]);
        self.scalar_static_f64[254]=(-2.0-self.scalar_static_f64[52]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[52]*self.scalar_static_f64[52]);
        self.scalar_static_f64[256]=(1.0-self.scalar_static_f64[255]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[52]-1.0);
        self.scalar_static_f64[258]=p.p8;
        self.scalar_static_bool[42]=(1.0==self.scalar_static_f64[258]);
        self.scalar_static_f64[259]=(if self.scalar_static_bool[42]{1.0}else{0.0});
        self.scalar_static_f64[260]=p.p143;
        self.scalar_static_f64[261]=(2.0*self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=p.p144;
        self.scalar_static_f64[263]=(1.0-self.scalar_static_f64[260]);
        self.scalar_static_f64[264]=(2.0*self.scalar_static_f64[263]);
        self.scalar_static_bool[43]=(!((self.scalar_static_f64[259])!=0.0));
        self.scalar_static_f64[265]=(4.0*self.scalar_static_f64[262]);
        self.scalar_static_f64[266]=p.p5;
        self.scalar_static_bool[44]=(self.scalar_static_f64[266]>0.0);
        self.scalar_static_bool[45]=(self.scalar_static_f64[6]>0.0);
        self.scalar_static_bool[46]=(self.scalar_static_bool[44]&&self.scalar_static_bool[45]);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_f64[268]=(self.scalar_static_f64[6]*2.0);
        self.scalar_static_bool[47]=(((self.scalar_static_f64[259])!=0.0)&&((self.scalar_static_f64[267])!=0.0));
        self.scalar_static_f64[269]=(self.scalar_static_f64[6]*self.scalar_static_f64[263]);
        self.scalar_static_f64[270]=(2.0*self.scalar_static_f64[269]);
        self.scalar_static_bool[48]=(self.scalar_static_bool[43]&&((self.scalar_static_f64[267])!=0.0));
        self.scalar_static_bool[49]=(1.0==self.scalar_static_f64[266]);
        self.scalar_static_f64[271]=(if self.scalar_static_bool[49]{1.0}else{0.0});
        self.scalar_static_bool[50]=(((self.scalar_static_f64[267])!=0.0)&&((self.scalar_static_f64[271])!=0.0));
        self.scalar_static_f64[272]=(if self.scalar_static_bool[50]{0.0121}else{0.010000000000000002});
        self.scalar_static_f64[273]=(0.5*self.scalar_static_f64[272]);
        self.scalar_static_bool[51]=(!((self.scalar_static_f64[271])!=0.0));
        self.scalar_static_bool[52]=(((self.scalar_static_f64[267])!=0.0)&&self.scalar_static_bool[51]);
        self.scalar_static_f64[274]=p.p84;
        self.scalar_static_bool[53]=(1.0==self.scalar_static_f64[274]);
        self.scalar_static_f64[275]=(if self.scalar_static_bool[53]{1.0}else{0.0});
        self.scalar_static_f64[276]=(if ((self.scalar_static_f64[275])!=0.0){1e-12}else{self.scalar_static_f64[272]});
        self.scalar_static_f64[277]=(0.5*self.scalar_static_f64[276]);
        self.scalar_static_f64[278]=p.p82;
        self.scalar_static_f64[279]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[278]);
        self.scalar_static_f64[280]=(1.0-self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(1.0/self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[281]}else{0.0});
        self.scalar_static_f64[283]=p.p81;
        self.scalar_static_f64[284]=(self.scalar_static_f64[84]*self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[284]}else{0.0});
        self.scalar_static_f64[286]=(self.scalar_static_f64[282]*self.scalar_static_f64[282]);
        self.scalar_static_f64[287]=(self.scalar_static_f64[278]-1.0);
        self.scalar_static_f64[288]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[286]*self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[278]*self.scalar_static_f64[289]);
        self.scalar_static_f64[291]=(self.scalar_static_f64[290]/self.scalar_static_f64[283]);
        self.scalar_static_f64[292]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[291]}else{0.0});
        self.scalar_static_bool[54]=(!((self.scalar_static_f64[275])!=0.0));
        self.scalar_static_f64[293]=p.p39;
        self.scalar_static_bool[55]=(1.0==self.scalar_static_f64[293]);
        self.scalar_static_f64[294]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_f64[295]=p.p44;
        self.scalar_static_f64[296]=p.p42;
        self.scalar_static_f64[297]=p.p41;
        self.scalar_static_f64[298]=p.p40;
        self.scalar_static_bool[56]=(2.0==self.scalar_static_f64[293]);
        self.scalar_static_f64[299]=(if self.scalar_static_bool[56]{1.0}else{0.0});
        self.scalar_static_bool[57]=(!((self.scalar_static_f64[294])!=0.0));
        self.scalar_static_f64[300]=p.p46;
        self.scalar_static_f64[301]=(2.0*self.scalar_static_f64[300]);
        self.scalar_static_f64[302]=p.p45;
        self.scalar_static_f64[303]=(self.scalar_static_f64[302]*self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[301]/self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=p.p7;
        self.scalar_static_bool[58]=(0.0==self.scalar_static_f64[305]);
        self.scalar_static_f64[306]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_bool[59]=(!((self.scalar_static_f64[306])!=0.0));
        self.scalar_static_f64[307]=p.p47;
        self.scalar_static_f64[308]=(2.0*self.scalar_static_f64[307]);
        self.scalar_static_f64[309]=(1.0+self.scalar_static_f64[307]);
        self.scalar_static_f64[310]=(1.0+self.scalar_static_f64[308]);
        self.scalar_static_f64[311]=(self.scalar_static_f64[309]/self.scalar_static_f64[310]);
        self.scalar_static_bool[60]=(3.0==self.scalar_static_f64[293]);
        self.scalar_static_f64[312]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=(!((self.scalar_static_f64[299])!=0.0));
        self.scalar_static_f64[313]=p.p48;
        self.scalar_static_f64[314]=p.p49;
        self.scalar_static_f64[315]=p.p52;
        self.scalar_static_f64[316]=p.p51;
        self.scalar_static_f64[317]=p.p50;
        self.scalar_static_f64[318]=p.p53;
        self.scalar_static_bool[62]=(1.0==self.scalar_static_f64[318]);
        self.scalar_static_f64[319]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_bool[63]=(!((self.scalar_static_f64[312])!=0.0));
        self.scalar_static_bool[64]=(!((self.scalar_static_f64[319])!=0.0));
        self.scalar_static_f64[320]=p.p68;
        self.scalar_static_f64[321]=(1.0-self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=p.p77;
        self.scalar_static_f64[323]=(1.0-self.scalar_static_f64[322]);
        self.scalar_static_f64[324]=(-1.0/self.scalar_static_f64[97]);
        self.scalar_static_f64[325]=f64::powf(2.0,self.scalar_static_f64[324]);
        self.scalar_static_f64[326]=(1.0-self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=(1.0-self.scalar_static_f64[97]);
        self.scalar_static_f64[328]=p.p85;
        self.scalar_static_f64[329]=(1.0/self.scalar_static_f64[328]);
        self.scalar_static_f64[330]=p.p79;
        self.scalar_static_bool[65]=(0.0==self.scalar_static_f64[330]);
        self.scalar_static_f64[331]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[332]=p.p91;
        self.scalar_static_bool[66]=(!((self.scalar_static_f64[331])!=0.0));
        self.scalar_static_bool[67]=(3.0==self.scalar_static_f64[266]);
        self.scalar_static_bool[68]=(self.scalar_static_bool[49]||self.scalar_static_bool[67]);
        self.scalar_static_bool[69]=(self.scalar_static_bool[45]&&self.scalar_static_bool[68]);
        self.scalar_static_f64[333]=(if self.scalar_static_bool[69]{1.0}else{0.0});
        self.scalar_static_bool[70]=(((self.scalar_static_f64[331])!=0.0)&&((self.scalar_static_f64[333])!=0.0));
        self.scalar_static_f64[334]=(self.scalar_static_f64[6]*0.5);
        self.scalar_static_bool[71]=(self.scalar_static_bool[66]&&((self.scalar_static_f64[333])!=0.0));
        self.scalar_static_f64[335]=p.p6;
        self.scalar_static_bool[72]=(1.0==self.scalar_static_f64[335]);
        self.scalar_static_f64[336]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_f64[337]=(-self.scalar_static_f64[20]);
        self.scalar_static_f64[338]=p.p95;
        self.scalar_static_f64[339]=(1.0-self.scalar_static_f64[338]);
        self.scalar_static_f64[340]=p.p94;
        self.scalar_static_f64[341]=(1.0-self.scalar_static_f64[340]);
        self.scalar_static_bool[73]=(!((self.scalar_static_f64[336])!=0.0));
        self.scalar_static_f64[342]=p.p130;
        self.scalar_static_bool[74]=(self.scalar_static_f64[342]>0.0);
        self.scalar_static_f64[343]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_bool[75]=(!((self.scalar_static_f64[343])!=0.0));
        self.scalar_static_f64[344]=p.p131;
        self.scalar_static_bool[76]=(1.0==self.scalar_static_f64[344]);
        self.scalar_static_f64[345]=(if self.scalar_static_bool[76]{1.0}else{0.0});
        self.scalar_static_bool[77]=(2.0==self.scalar_static_f64[344]);
        self.scalar_static_f64[346]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_bool[78]=(!((self.scalar_static_f64[345])!=0.0));
        self.scalar_static_bool[79]=(((self.scalar_static_f64[346])!=0.0)&&self.scalar_static_bool[78]);
        self.scalar_static_f64[347]=p.p132;
        self.scalar_static_bool[80]=(!((self.scalar_static_f64[346])!=0.0));
        self.scalar_static_bool[81]=(self.scalar_static_bool[78]&&self.scalar_static_bool[80]);
        self.scalar_static_f64[348]=p.p69;
        self.scalar_static_f64[349]=p.p78;
        self.scalar_static_f64[350]=(self.scalar_static_f64[0]*self.scalar_static_f64[348]);
        self.scalar_static_f64[351]=(self.scalar_static_f64[0]*self.scalar_static_f64[349]);
        self.scalar_static_f64[352]=(-self.scalar_static_f64[0]);
        self.scalar_static_f64[353]=(self.scalar_static_f64[0]+self.scalar_static_f64[352]);
        self.scalar_static_f64[354]=(self.scalar_static_f64[352]-self.scalar_static_f64[352]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[0]+self.scalar_static_f64[353]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[235]-1.0);
        self.scalar_static_f64[357]=(if ((self.scalar_static_f64[237])!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[358]=(if ((self.scalar_static_f64[237])!=0.0){self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[359]=(self.scalar_static_f64[240]-1.0);
        self.scalar_static_f64[360]=(self.scalar_static_f64[241]-1.0);
        self.scalar_static_f64[361]=(self.scalar_static_f64[352]/0.0001);
        self.scalar_static_f64[362]=(self.scalar_static_f64[0]/0.0001);
        self.scalar_static_f64[363]=(-self.scalar_static_f64[361]);
        self.scalar_static_f64[364]=(-self.scalar_static_f64[362]);
        self.scalar_static_f64[365]=(self.scalar_static_f64[352]/0.001);
        self.scalar_static_f64[366]=(self.scalar_static_f64[0]/0.001);
        self.scalar_static_f64[367]=(-self.scalar_static_f64[365]);
        self.scalar_static_f64[368]=(-self.scalar_static_f64[366]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[250]-1.0);
        self.scalar_static_f64[370]=(self.scalar_static_f64[22]*self.scalar_static_f64[352]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[0]*self.scalar_static_f64[22]);
        self.scalar_static_f64[372]=(0.5*self.scalar_static_f64[352]);
        self.scalar_static_f64[373]=(self.scalar_static_f64[0]*0.5);
        self.scalar_static_f64[374]=(self.scalar_static_f64[254]-1.0);
        self.scalar_static_f64[375]=(self.scalar_static_f64[0]*self.scalar_static_f64[54]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[54]*self.scalar_static_f64[352]);
        self.scalar_static_f64[377]=(if self.scalar_static_bool[50]{self.scalar_static_f64[353]}else{0.0});
        self.scalar_static_f64[378]=(if self.scalar_static_bool[50]{self.scalar_static_f64[355]}else{0.0});
        self.scalar_static_f64[379]=(if self.scalar_static_bool[50]{self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_f64[380]=(if self.scalar_static_bool[50]{self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[381]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[382]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[353]}else{0.0});
        self.scalar_static_f64[383]=(if ((self.scalar_static_f64[275])!=0.0){self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[384]=(-self.scalar_static_f64[381]);
        self.scalar_static_f64[385]=(-self.scalar_static_f64[382]);
        self.scalar_static_f64[386]=(-self.scalar_static_f64[383]);
        self.scalar_static_f64[387]=(self.scalar_static_f64[297]-1.0);
        self.scalar_static_f64[388]=(self.scalar_static_f64[314]-1.0);
        self.scalar_static_f64[389]=(self.scalar_static_f64[317]-1.0);
        self.scalar_static_f64[390]=(self.scalar_static_f64[327]-1.0);
        self.scalar_static_f64[391]=(self.scalar_static_f64[0]/self.scalar_static_f64[332]);
        self.scalar_static_f64[392]=(self.scalar_static_f64[353]/self.scalar_static_f64[332]);
        self.scalar_static_f64[393]=(self.scalar_static_f64[354]/self.scalar_static_f64[332]);
        self.scalar_static_f64[394]=(self.scalar_static_f64[352]/self.scalar_static_f64[332]);
        self.scalar_static_f64[395]=(self.scalar_static_f64[337]-1.0);
        self.scalar_static_f64[396]=(self.scalar_static_f64[0]*0.2);
        self.scalar_static_f64[397]=(0.2*self.scalar_static_f64[352]);
        self.scalar_static_f64[398]=(self.scalar_static_f64[0]*self.scalar_static_f64[0]);
        self.scalar_static_f64[399]=(self.scalar_static_f64[0]*self.scalar_static_f64[352]);
        self.scalar_static_f64[400]=(self.scalar_static_f64[0]*self.scalar_static_f64[350]);
        self.scalar_static_f64[401]=(self.scalar_static_f64[350]*self.scalar_static_f64[352]);
        self.scalar_static_f64[402]=(self.scalar_static_f64[351]*self.scalar_static_f64[352]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[0]*self.scalar_static_f64[351]);
        self.scalar_static_f64[404]=(self.scalar_static_f64[0]*self.scalar_static_f64[353]);
        self.scalar_static_f64[405]=(self.scalar_static_f64[0]*self.scalar_static_f64[354]);
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
        self.scalar_static_f64[406]=(temperature+self.scalar_static_f64[10]);
        self.scalar_static_f64[407]=(self.scalar_static_f64[406]/self.scalar_static_f64[9]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[406]*8.617086918058125e-5);
        self.scalar_static_f64[409]=(1.0/self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[409]-self.scalar_static_f64[86]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[406]-self.scalar_static_f64[9]);
        self.scalar_static_f64[412]=(self.scalar_static_f64[407]).ln();
        self.scalar_static_f64[413]=(self.scalar_static_f64[406]*self.scalar_static_f64[25]);
        self.scalar_static_f64[414]=(self.scalar_static_f64[406]*self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[406]+self.scalar_static_f64[28]);
        self.scalar_static_f64[416]=(self.scalar_static_f64[414]/self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=(self.scalar_static_f64[47]-self.scalar_static_f64[416]);
        self.scalar_static_f64[418]=(self.scalar_static_f64[417]-0.05);
        self.scalar_static_f64[419]=(self.scalar_static_f64[418]/0.1);
        self.scalar_static_bool[82]=(self.scalar_static_f64[417]<0.05);
        self.scalar_static_f64[420]=(if self.scalar_static_bool[82]{1.0}else{0.0});
        self.scalar_static_f64[421]=(self.scalar_static_f64[419]).exp();
        self.scalar_static_f64[422]=(1.0+self.scalar_static_f64[421]);
        self.scalar_static_f64[423]=(self.scalar_static_f64[422]).ln();
        self.scalar_static_f64[424]=(0.1*self.scalar_static_f64[423]);
        self.scalar_static_f64[425]=(0.05+self.scalar_static_f64[424]);
        self.scalar_static_f64[426]=(if ((self.scalar_static_f64[420])!=0.0){self.scalar_static_f64[425]}else{0.0});
        self.scalar_static_bool[83]=(!((self.scalar_static_f64[420])!=0.0));
        self.scalar_static_f64[427]=(-self.scalar_static_f64[419]);
        self.scalar_static_f64[428]=(self.scalar_static_f64[427]).exp();
        self.scalar_static_f64[429]=(1.0+self.scalar_static_f64[428]);
        self.scalar_static_f64[430]=(self.scalar_static_f64[429]).ln();
        self.scalar_static_f64[431]=(0.1*self.scalar_static_f64[430]);
        self.scalar_static_f64[432]=(self.scalar_static_f64[417]+self.scalar_static_f64[431]);
        self.scalar_static_f64[433]=(if self.scalar_static_bool[83]{self.scalar_static_f64[432]}else{self.scalar_static_f64[426]});
        self.scalar_static_f64[434]=(self.scalar_static_f64[406]*self.scalar_static_f64[57]);
        self.scalar_static_f64[435]=(self.scalar_static_f64[406]*self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=(self.scalar_static_f64[406]+self.scalar_static_f64[60]);
        self.scalar_static_f64[437]=(self.scalar_static_f64[435]/self.scalar_static_f64[436]);
        self.scalar_static_f64[438]=(self.scalar_static_f64[79]-self.scalar_static_f64[437]);
        self.scalar_static_f64[439]=(self.scalar_static_f64[438]-0.05);
        self.scalar_static_f64[440]=(self.scalar_static_f64[439]/0.1);
        self.scalar_static_bool[84]=(self.scalar_static_f64[438]<0.05);
        self.scalar_static_f64[441]=(if self.scalar_static_bool[84]{1.0}else{0.0});
        self.scalar_static_f64[442]=(self.scalar_static_f64[440]).exp();
        self.scalar_static_f64[443]=(1.0+self.scalar_static_f64[442]);
        self.scalar_static_f64[444]=(self.scalar_static_f64[443]).ln();
        self.scalar_static_f64[445]=(0.1*self.scalar_static_f64[444]);
        self.scalar_static_f64[446]=(0.05+self.scalar_static_f64[445]);
        self.scalar_static_f64[447]=(if ((self.scalar_static_f64[441])!=0.0){self.scalar_static_f64[446]}else{0.0});
        self.scalar_static_bool[85]=(!((self.scalar_static_f64[441])!=0.0));
        self.scalar_static_f64[448]=(-self.scalar_static_f64[440]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[448]).exp();
        self.scalar_static_f64[450]=(1.0+self.scalar_static_f64[449]);
        self.scalar_static_f64[451]=(self.scalar_static_f64[450]).ln();
        self.scalar_static_f64[452]=(0.1*self.scalar_static_f64[451]);
        self.scalar_static_f64[453]=(self.scalar_static_f64[438]+self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(if self.scalar_static_bool[85]{self.scalar_static_f64[453]}else{self.scalar_static_f64[447]});
        self.scalar_static_f64[455]=(self.scalar_static_f64[408]* -3.0);
        self.scalar_static_f64[456]=(self.scalar_static_f64[412]*self.scalar_static_f64[455]);
        self.scalar_static_f64[457]=(self.scalar_static_f64[49]*self.scalar_static_f64[407]);
        self.scalar_static_f64[458]=(self.scalar_static_f64[456]+self.scalar_static_f64[457]);
        self.scalar_static_f64[459]=(1.0-self.scalar_static_f64[407]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[459]*self.scalar_static_f64[87]);
        self.scalar_static_f64[461]=(self.scalar_static_f64[458]+self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(0.05-self.scalar_static_f64[461]);
        self.scalar_static_f64[463]=(self.scalar_static_f64[462]/self.scalar_static_f64[408]);
        self.scalar_static_bool[86]=(0.05<self.scalar_static_f64[461]);
        self.scalar_static_f64[464]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_f64[465]=(self.scalar_static_f64[463]).exp();
        self.scalar_static_f64[466]=(1.0+self.scalar_static_f64[465]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[466]).ln();
        self.scalar_static_f64[468]=(self.scalar_static_f64[408]*self.scalar_static_f64[467]);
        self.scalar_static_f64[469]=(self.scalar_static_f64[461]+self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(if ((self.scalar_static_f64[464])!=0.0){self.scalar_static_f64[469]}else{0.0});
        self.scalar_static_bool[87]=(!((self.scalar_static_f64[464])!=0.0));
        self.scalar_static_f64[471]=(-self.scalar_static_f64[463]);
        self.scalar_static_f64[472]=(self.scalar_static_f64[471]).exp();
        self.scalar_static_f64[473]=(1.0+self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=(self.scalar_static_f64[473]).ln();
        self.scalar_static_f64[475]=(self.scalar_static_f64[408]*self.scalar_static_f64[474]);
        self.scalar_static_f64[476]=(0.05+self.scalar_static_f64[475]);
        self.scalar_static_f64[477]=(if self.scalar_static_bool[87]{self.scalar_static_f64[476]}else{self.scalar_static_f64[470]});
        self.scalar_static_f64[478]=(self.scalar_static_f64[407]*self.scalar_static_f64[88]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[456]+self.scalar_static_f64[478]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[459]*self.scalar_static_f64[89]);
        self.scalar_static_f64[481]=(self.scalar_static_f64[479]+self.scalar_static_f64[480]);
        self.scalar_static_f64[482]=(0.05-self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[482]/self.scalar_static_f64[408]);
        self.scalar_static_bool[88]=(0.05<self.scalar_static_f64[481]);
        self.scalar_static_f64[484]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_f64[485]=(self.scalar_static_f64[483]).exp();
        self.scalar_static_f64[486]=(1.0+self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=(self.scalar_static_f64[486]).ln();
        self.scalar_static_f64[488]=(self.scalar_static_f64[408]*self.scalar_static_f64[487]);
        self.scalar_static_f64[489]=(self.scalar_static_f64[481]+self.scalar_static_f64[488]);
        self.scalar_static_f64[490]=(if ((self.scalar_static_f64[484])!=0.0){self.scalar_static_f64[489]}else{0.0});
        self.scalar_static_bool[89]=(!((self.scalar_static_f64[484])!=0.0));
        self.scalar_static_f64[491]=(-self.scalar_static_f64[483]);
        self.scalar_static_f64[492]=(self.scalar_static_f64[491]).exp();
        self.scalar_static_f64[493]=(1.0+self.scalar_static_f64[492]);
        self.scalar_static_f64[494]=(self.scalar_static_f64[493]).ln();
        self.scalar_static_f64[495]=(self.scalar_static_f64[408]*self.scalar_static_f64[494]);
        self.scalar_static_f64[496]=(0.05+self.scalar_static_f64[495]);
        self.scalar_static_f64[497]=(if self.scalar_static_bool[89]{self.scalar_static_f64[496]}else{self.scalar_static_f64[490]});
        self.scalar_static_f64[498]=(self.scalar_static_f64[407]*self.scalar_static_f64[90]);
        self.scalar_static_f64[499]=(self.scalar_static_f64[456]+self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=(self.scalar_static_f64[480]+self.scalar_static_f64[499]);
        self.scalar_static_f64[501]=(0.05-self.scalar_static_f64[500]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[501]/self.scalar_static_f64[408]);
        self.scalar_static_bool[90]=(0.05<self.scalar_static_f64[500]);
        self.scalar_static_f64[503]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_f64[504]=(self.scalar_static_f64[502]).exp();
        self.scalar_static_f64[505]=(1.0+self.scalar_static_f64[504]);
        self.scalar_static_f64[506]=(self.scalar_static_f64[505]).ln();
        self.scalar_static_f64[507]=(self.scalar_static_f64[408]*self.scalar_static_f64[506]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[500]+self.scalar_static_f64[507]);
        self.scalar_static_f64[509]=(if ((self.scalar_static_f64[503])!=0.0){self.scalar_static_f64[508]}else{0.0});
        self.scalar_static_bool[91]=(!((self.scalar_static_f64[503])!=0.0));
        self.scalar_static_f64[510]=(-self.scalar_static_f64[502]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[510]).exp();
        self.scalar_static_f64[512]=(1.0+self.scalar_static_f64[511]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[512]).ln();
        self.scalar_static_f64[514]=(self.scalar_static_f64[408]*self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(0.05+self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=(if self.scalar_static_bool[91]{self.scalar_static_f64[515]}else{self.scalar_static_f64[509]});
        self.scalar_static_f64[517]=(self.scalar_static_f64[51]*self.scalar_static_f64[407]);
        self.scalar_static_f64[518]=(self.scalar_static_f64[456]+self.scalar_static_f64[517]);
        self.scalar_static_f64[519]=(self.scalar_static_f64[480]+self.scalar_static_f64[518]);
        self.scalar_static_f64[520]=(0.05-self.scalar_static_f64[519]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[520]/self.scalar_static_f64[408]);
        self.scalar_static_bool[92]=(0.05<self.scalar_static_f64[519]);
        self.scalar_static_f64[522]=(if self.scalar_static_bool[92]{1.0}else{0.0});
        self.scalar_static_f64[523]=(self.scalar_static_f64[521]).exp();
        self.scalar_static_f64[524]=(1.0+self.scalar_static_f64[523]);
        self.scalar_static_f64[525]=(self.scalar_static_f64[524]).ln();
        self.scalar_static_f64[526]=(self.scalar_static_f64[408]*self.scalar_static_f64[525]);
        self.scalar_static_f64[527]=(self.scalar_static_f64[519]+self.scalar_static_f64[526]);
        self.scalar_static_f64[528]=(if ((self.scalar_static_f64[522])!=0.0){self.scalar_static_f64[527]}else{0.0});
        self.scalar_static_bool[93]=(!((self.scalar_static_f64[522])!=0.0));
        self.scalar_static_f64[529]=(-self.scalar_static_f64[521]);
        self.scalar_static_f64[530]=(self.scalar_static_f64[529]).exp();
        self.scalar_static_f64[531]=(1.0+self.scalar_static_f64[530]);
        self.scalar_static_f64[532]=(self.scalar_static_f64[531]).ln();
        self.scalar_static_f64[533]=(self.scalar_static_f64[408]*self.scalar_static_f64[532]);
        self.scalar_static_f64[534]=(0.05+self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(if self.scalar_static_bool[93]{self.scalar_static_f64[534]}else{self.scalar_static_f64[528]});
        self.scalar_static_f64[536]=(self.scalar_static_f64[407]*self.scalar_static_f64[91]);
        self.scalar_static_f64[537]=(self.scalar_static_f64[456]+self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=(self.scalar_static_f64[459]*self.scalar_static_f64[92]);
        self.scalar_static_f64[539]=(self.scalar_static_f64[537]+self.scalar_static_f64[538]);
        self.scalar_static_f64[540]=(0.05-self.scalar_static_f64[539]);
        self.scalar_static_f64[541]=(self.scalar_static_f64[540]/self.scalar_static_f64[408]);
        self.scalar_static_bool[94]=(0.05<self.scalar_static_f64[539]);
        self.scalar_static_f64[542]=(if self.scalar_static_bool[94]{1.0}else{0.0});
        self.scalar_static_f64[543]=(self.scalar_static_f64[541]).exp();
        self.scalar_static_f64[544]=(1.0+self.scalar_static_f64[543]);
        self.scalar_static_f64[545]=(self.scalar_static_f64[544]).ln();
        self.scalar_static_f64[546]=(self.scalar_static_f64[408]*self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=(self.scalar_static_f64[539]+self.scalar_static_f64[546]);
        self.scalar_static_f64[548]=(if ((self.scalar_static_f64[542])!=0.0){self.scalar_static_f64[547]}else{0.0});
        self.scalar_static_bool[95]=(!((self.scalar_static_f64[542])!=0.0));
        self.scalar_static_f64[549]=(-self.scalar_static_f64[541]);
        self.scalar_static_f64[550]=(self.scalar_static_f64[549]).exp();
        self.scalar_static_f64[551]=(1.0+self.scalar_static_f64[550]);
        self.scalar_static_f64[552]=(self.scalar_static_f64[551]).ln();
        self.scalar_static_f64[553]=(self.scalar_static_f64[408]*self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=(0.05+self.scalar_static_f64[553]);
        self.scalar_static_f64[555]=(if self.scalar_static_bool[95]{self.scalar_static_f64[554]}else{self.scalar_static_f64[548]});
        self.scalar_static_f64[556]=(self.scalar_static_f64[407]*self.scalar_static_f64[93]);
        self.scalar_static_f64[557]=(self.scalar_static_f64[456]+self.scalar_static_f64[556]);
        self.scalar_static_f64[558]=(self.scalar_static_f64[459]*self.scalar_static_f64[94]);
        self.scalar_static_f64[559]=(self.scalar_static_f64[557]+self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(0.05-self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=(self.scalar_static_f64[560]/self.scalar_static_f64[408]);
        self.scalar_static_bool[96]=(0.05<self.scalar_static_f64[559]);
        self.scalar_static_f64[562]=(if self.scalar_static_bool[96]{1.0}else{0.0});
        self.scalar_static_f64[563]=(self.scalar_static_f64[561]).exp();
        self.scalar_static_f64[564]=(1.0+self.scalar_static_f64[563]);
        self.scalar_static_f64[565]=(self.scalar_static_f64[564]).ln();
        self.scalar_static_f64[566]=(self.scalar_static_f64[408]*self.scalar_static_f64[565]);
        self.scalar_static_f64[567]=(self.scalar_static_f64[559]+self.scalar_static_f64[566]);
        self.scalar_static_f64[568]=(if ((self.scalar_static_f64[562])!=0.0){self.scalar_static_f64[567]}else{0.0});
        self.scalar_static_bool[97]=(!((self.scalar_static_f64[562])!=0.0));
        self.scalar_static_f64[569]=(-self.scalar_static_f64[561]);
        self.scalar_static_f64[570]=(self.scalar_static_f64[569]).exp();
        self.scalar_static_f64[571]=(1.0+self.scalar_static_f64[570]);
        self.scalar_static_f64[572]=(self.scalar_static_f64[571]).ln();
        self.scalar_static_f64[573]=(self.scalar_static_f64[408]*self.scalar_static_f64[572]);
        self.scalar_static_f64[574]=(0.05+self.scalar_static_f64[573]);
        self.scalar_static_f64[575]=(if self.scalar_static_bool[97]{self.scalar_static_f64[574]}else{self.scalar_static_f64[568]});
        self.scalar_static_f64[576]=(1.0/self.scalar_static_f64[477]);
        self.scalar_static_f64[577]=(1.0/self.scalar_static_f64[535]);
        self.scalar_static_f64[578]=(self.scalar_static_f64[49]*self.scalar_static_f64[576]);
        self.scalar_static_f64[579]=f64::powf(self.scalar_static_f64[578],self.scalar_static_f64[20]);
        self.scalar_static_f64[580]=(self.scalar_static_f64[51]*self.scalar_static_f64[577]);
        self.scalar_static_f64[581]=f64::powf(self.scalar_static_f64[580],self.scalar_static_f64[52]);
        self.scalar_static_f64[582]=(self.scalar_static_f64[579]*self.scalar_static_f64[95]);
        self.scalar_static_f64[583]=(self.scalar_static_f64[93]/self.scalar_static_f64[575]);
        self.scalar_static_f64[584]=f64::powf(self.scalar_static_f64[583],self.scalar_static_f64[97]);
        self.scalar_static_f64[585]=(self.scalar_static_f64[96]*self.scalar_static_f64[584]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[51]/self.scalar_static_f64[535]);
        self.scalar_static_f64[587]=f64::powf(self.scalar_static_f64[586],self.scalar_static_f64[52]);
        self.scalar_static_f64[588]=(self.scalar_static_f64[99]*self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(self.scalar_static_f64[98]+self.scalar_static_f64[588]);
        self.scalar_static_f64[590]=(1.0/self.scalar_static_f64[589]);
        self.scalar_static_f64[591]=(self.scalar_static_f64[589]*self.scalar_static_f64[100]);
        self.scalar_static_f64[592]=(self.scalar_static_f64[98]*self.scalar_static_f64[590]);
        self.scalar_static_f64[593]=(self.scalar_static_f64[412]*self.scalar_static_f64[102]);
        self.scalar_static_f64[594]=(self.scalar_static_f64[593]).exp();
        self.scalar_static_f64[595]=(self.scalar_static_f64[101]*self.scalar_static_f64[594]);
        self.scalar_static_bool[98]=(self.scalar_static_f64[595]<self.scalar_static_f64[16]);
        self.scalar_static_f64[596]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_f64[597]=(if ((self.scalar_static_f64[596])!=0.0){self.scalar_static_f64[16]}else{self.scalar_static_f64[595]});
        self.scalar_static_f64[598]=(self.scalar_static_f64[412]*self.scalar_static_f64[106]);
        self.scalar_static_f64[599]=(self.scalar_static_f64[598]).exp();
        self.scalar_static_f64[600]=(self.scalar_static_f64[103]*self.scalar_static_f64[599]);
        self.scalar_static_f64[601]=(self.scalar_static_f64[412]*self.scalar_static_f64[108]);
        self.scalar_static_f64[602]=(self.scalar_static_f64[601]).exp();
        self.scalar_static_f64[603]=(self.scalar_static_f64[107]*self.scalar_static_f64[602]);
        self.scalar_static_bool[99]=(self.scalar_static_f64[603]<self.scalar_static_f64[16]);
        self.scalar_static_f64[604]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_f64[605]=(if ((self.scalar_static_f64[604])!=0.0){self.scalar_static_f64[16]}else{self.scalar_static_f64[603]});
        self.scalar_static_f64[606]=(self.scalar_static_f64[412]*self.scalar_static_f64[110]);
        self.scalar_static_f64[607]=(self.scalar_static_f64[606]).exp();
        self.scalar_static_f64[608]=(self.scalar_static_f64[109]*self.scalar_static_f64[607]);
        self.scalar_static_f64[609]=(self.scalar_static_f64[412]*self.scalar_static_f64[112]);
        self.scalar_static_f64[610]=(self.scalar_static_f64[609]).exp();
        self.scalar_static_f64[611]=(self.scalar_static_f64[111]*self.scalar_static_f64[610]);
        self.scalar_static_f64[612]=(self.scalar_static_f64[610]*self.scalar_static_f64[113]);
        self.scalar_static_f64[613]=(self.scalar_static_f64[412]*self.scalar_static_f64[115]);
        self.scalar_static_f64[614]=(self.scalar_static_f64[613]).exp();
        self.scalar_static_f64[615]=(self.scalar_static_f64[114]*self.scalar_static_f64[614]);
        self.scalar_static_f64[616]=(self.scalar_static_f64[411]*self.scalar_static_f64[116]);
        self.scalar_static_f64[617]=(1.0+self.scalar_static_f64[616]);
        self.scalar_static_f64[618]=(self.scalar_static_f64[118]*self.scalar_static_f64[617]);
        self.scalar_static_f64[619]=(if ((self.scalar_static_f64[117])!=0.0){self.scalar_static_f64[618]}else{0.0});
        self.scalar_static_f64[620]=(self.scalar_static_f64[619]-1.0);
        self.scalar_static_f64[621]=(self.scalar_static_f64[620]/0.001);
        self.scalar_static_f64[622]=(if ((self.scalar_static_f64[117])!=0.0){self.scalar_static_f64[621]}else{self.scalar_static_f64[561]});
        self.scalar_static_bool[100]=(self.scalar_static_f64[619]<1.0);
        self.scalar_static_f64[623]=(if self.scalar_static_bool[100]{1.0}else{0.0});
        self.scalar_static_bool[101]=(((self.scalar_static_f64[117])!=0.0)&&((self.scalar_static_f64[623])!=0.0));
        self.scalar_static_f64[624]=(self.scalar_static_f64[622]).exp();
        self.scalar_static_f64[625]=(1.0+self.scalar_static_f64[624]);
        self.scalar_static_f64[626]=(self.scalar_static_f64[625]).ln();
        self.scalar_static_f64[627]=(0.001*self.scalar_static_f64[626]);
        self.scalar_static_f64[628]=(1.0+self.scalar_static_f64[627]);
        self.scalar_static_f64[629]=(if self.scalar_static_bool[101]{self.scalar_static_f64[628]}else{self.scalar_static_f64[619]});
        self.scalar_static_bool[102]=(!((self.scalar_static_f64[623])!=0.0));
        self.scalar_static_bool[103]=(((self.scalar_static_f64[117])!=0.0)&&self.scalar_static_bool[102]);
        self.scalar_static_f64[630]=(-self.scalar_static_f64[622]);
        self.scalar_static_f64[631]=(self.scalar_static_f64[630]).exp();
        self.scalar_static_f64[632]=(1.0+self.scalar_static_f64[631]);
        self.scalar_static_f64[633]=(self.scalar_static_f64[632]).ln();
        self.scalar_static_f64[634]=(0.001*self.scalar_static_f64[633]);
        self.scalar_static_f64[635]=(self.scalar_static_f64[629]+self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(if self.scalar_static_bool[103]{self.scalar_static_f64[635]}else{self.scalar_static_f64[629]});
        self.scalar_static_f64[637]=(self.scalar_static_f64[636]-0.0006931471805599453);
        self.scalar_static_f64[638]=(if ((self.scalar_static_f64[117])!=0.0){self.scalar_static_f64[637]}else{0.0});
        self.scalar_static_f64[639]=(if self.scalar_static_bool[11]{self.scalar_static_f64[118]}else{self.scalar_static_f64[638]});
        self.scalar_static_f64[640]=(self.scalar_static_f64[411]*self.scalar_static_f64[119]);
        self.scalar_static_f64[641]=(1.0+self.scalar_static_f64[640]);
        self.scalar_static_f64[642]=(self.scalar_static_f64[121]*self.scalar_static_f64[641]);
        self.scalar_static_f64[643]=(if ((self.scalar_static_f64[120])!=0.0){self.scalar_static_f64[642]}else{0.0});
        self.scalar_static_f64[644]=(self.scalar_static_f64[643]-1.0);
        self.scalar_static_f64[645]=(self.scalar_static_f64[644]/0.001);
        self.scalar_static_f64[646]=(if ((self.scalar_static_f64[120])!=0.0){self.scalar_static_f64[645]}else{self.scalar_static_f64[622]});
        self.scalar_static_bool[104]=(self.scalar_static_f64[643]<1.0);
        self.scalar_static_f64[647]=(if self.scalar_static_bool[104]{1.0}else{0.0});
        self.scalar_static_bool[105]=(((self.scalar_static_f64[120])!=0.0)&&((self.scalar_static_f64[647])!=0.0));
        self.scalar_static_f64[648]=(self.scalar_static_f64[646]).exp();
        self.scalar_static_f64[649]=(1.0+self.scalar_static_f64[648]);
        self.scalar_static_f64[650]=(self.scalar_static_f64[649]).ln();
        self.scalar_static_f64[651]=(0.001*self.scalar_static_f64[650]);
        self.scalar_static_f64[652]=(1.0+self.scalar_static_f64[651]);
        self.scalar_static_f64[653]=(if self.scalar_static_bool[105]{self.scalar_static_f64[652]}else{self.scalar_static_f64[643]});
        self.scalar_static_bool[106]=(!((self.scalar_static_f64[647])!=0.0));
        self.scalar_static_bool[107]=(((self.scalar_static_f64[120])!=0.0)&&self.scalar_static_bool[106]);
        self.scalar_static_f64[654]=(-self.scalar_static_f64[646]);
        self.scalar_static_f64[655]=(self.scalar_static_f64[654]).exp();
        self.scalar_static_f64[656]=(1.0+self.scalar_static_f64[655]);
        self.scalar_static_f64[657]=(self.scalar_static_f64[656]).ln();
        self.scalar_static_f64[658]=(0.001*self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=(self.scalar_static_f64[653]+self.scalar_static_f64[658]);
        self.scalar_static_f64[660]=(if self.scalar_static_bool[107]{self.scalar_static_f64[659]}else{self.scalar_static_f64[653]});
        self.scalar_static_f64[661]=(self.scalar_static_f64[660]-0.0006931471805599453);
        self.scalar_static_f64[662]=(if ((self.scalar_static_f64[120])!=0.0){self.scalar_static_f64[661]}else{0.0});
        self.scalar_static_f64[663]=(if self.scalar_static_bool[13]{self.scalar_static_f64[121]}else{self.scalar_static_f64[662]});
        self.scalar_static_f64[664]=(self.scalar_static_f64[411]*self.scalar_static_f64[123]);
        self.scalar_static_f64[665]=(1.0+self.scalar_static_f64[664]);
        self.scalar_static_f64[666]=(self.scalar_static_f64[122]*self.scalar_static_f64[665]);
        self.scalar_static_f64[667]=(self.scalar_static_f64[666]*self.scalar_static_f64[666]);
        self.scalar_static_bool[108]=(self.scalar_static_f64[666]<0.0);
        self.scalar_static_f64[668]=(if self.scalar_static_bool[108]{1.0}else{0.0});
        self.scalar_static_f64[669]=(1e-6+self.scalar_static_f64[667]);
        self.scalar_static_f64[670]=(self.scalar_static_f64[669]).sqrt();
        self.scalar_static_f64[671]=(self.scalar_static_f64[670]-self.scalar_static_f64[666]);
        self.scalar_static_f64[672]=(5e-7/self.scalar_static_f64[671]);
        self.scalar_static_f64[673]=(if ((self.scalar_static_f64[668])!=0.0){self.scalar_static_f64[672]}else{0.0});
        self.scalar_static_bool[109]=(!((self.scalar_static_f64[668])!=0.0));
        self.scalar_static_f64[674]=(self.scalar_static_f64[666]+self.scalar_static_f64[670]);
        self.scalar_static_f64[675]=(0.5*self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=(if self.scalar_static_bool[109]{self.scalar_static_f64[675]}else{self.scalar_static_f64[673]});
        self.scalar_static_f64[677]=(self.scalar_static_f64[412]*self.scalar_static_f64[128]);
        self.scalar_static_f64[678]=(self.scalar_static_f64[677]/self.scalar_static_f64[639]);
        self.scalar_static_f64[679]=(self.scalar_static_f64[678]).exp();
        self.scalar_static_f64[680]=(self.scalar_static_f64[124]*self.scalar_static_f64[679]);
        self.scalar_static_f64[681]=(self.scalar_static_f64[410]*self.scalar_static_f64[129]);
        self.scalar_static_f64[682]=(self.scalar_static_f64[681]/self.scalar_static_f64[639]);
        self.scalar_static_f64[683]=(self.scalar_static_f64[682]).exp();
        self.scalar_static_f64[684]=(self.scalar_static_f64[680]*self.scalar_static_f64[683]);
        self.scalar_static_f64[685]=(self.scalar_static_f64[412]*self.scalar_static_f64[131]);
        self.scalar_static_f64[686]=(self.scalar_static_f64[685]).exp();
        self.scalar_static_f64[687]=(self.scalar_static_f64[130]*self.scalar_static_f64[686]);
        self.scalar_static_f64[688]=(self.scalar_static_f64[412]*self.scalar_static_f64[134]);
        self.scalar_static_f64[689]=(self.scalar_static_f64[688]).exp();
        self.scalar_static_f64[690]=(self.scalar_static_f64[132]*self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[412]*self.scalar_static_f64[138]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[691]).exp();
        self.scalar_static_f64[693]=(self.scalar_static_f64[135]*self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(self.scalar_static_f64[410]*self.scalar_static_f64[140]);
        self.scalar_static_f64[695]=(self.scalar_static_f64[694]/self.scalar_static_f64[136]);
        self.scalar_static_f64[696]=(self.scalar_static_f64[695]).exp();
        self.scalar_static_f64[697]=(self.scalar_static_f64[693]*self.scalar_static_f64[696]);
        self.scalar_static_f64[698]=(self.scalar_static_f64[412]*self.scalar_static_f64[144]);
        self.scalar_static_f64[699]=(self.scalar_static_f64[698]).exp();
        self.scalar_static_f64[700]=(self.scalar_static_f64[141]*self.scalar_static_f64[699]);
        self.scalar_static_f64[701]=(self.scalar_static_f64[410]*self.scalar_static_f64[145]);
        self.scalar_static_f64[702]=(self.scalar_static_f64[701]/self.scalar_static_f64[142]);
        self.scalar_static_f64[703]=(self.scalar_static_f64[702]).exp();
        self.scalar_static_f64[704]=(self.scalar_static_f64[700]*self.scalar_static_f64[703]);
        self.scalar_static_f64[705]=(self.scalar_static_f64[412]*self.scalar_static_f64[148]);
        self.scalar_static_f64[706]=(self.scalar_static_f64[705]/self.scalar_static_f64[149]);
        self.scalar_static_f64[707]=(self.scalar_static_f64[706]).exp();
        self.scalar_static_f64[708]=(self.scalar_static_f64[146]*self.scalar_static_f64[707]);
        self.scalar_static_f64[709]=(self.scalar_static_f64[410]*self.scalar_static_f64[151]);
        self.scalar_static_f64[710]=(self.scalar_static_f64[709]/self.scalar_static_f64[149]);
        self.scalar_static_f64[711]=(self.scalar_static_f64[710]).exp();
        self.scalar_static_f64[712]=(self.scalar_static_f64[708]*self.scalar_static_f64[711]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[705]/self.scalar_static_f64[153]);
        self.scalar_static_f64[714]=(self.scalar_static_f64[713]).exp();
        self.scalar_static_f64[715]=(self.scalar_static_f64[152]*self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(self.scalar_static_f64[709]/self.scalar_static_f64[153]);
        self.scalar_static_f64[717]=(self.scalar_static_f64[716]).exp();
        self.scalar_static_f64[718]=(self.scalar_static_f64[715]*self.scalar_static_f64[717]);
        self.scalar_static_f64[719]=(self.scalar_static_f64[410]*self.scalar_static_f64[158]);
        self.scalar_static_f64[720]=(self.scalar_static_f64[719]/self.scalar_static_f64[149]);
        self.scalar_static_f64[721]=(self.scalar_static_f64[720]).exp();
        self.scalar_static_f64[722]=(self.scalar_static_f64[156]*self.scalar_static_f64[721]);
        self.scalar_static_f64[723]=(if ((self.scalar_static_f64[155])!=0.0){self.scalar_static_f64[722]}else{0.0});
        self.scalar_static_f64[724]=(self.scalar_static_f64[410]*self.scalar_static_f64[161]);
        self.scalar_static_f64[725]=(self.scalar_static_f64[724]).exp();
        self.scalar_static_f64[726]=(self.scalar_static_f64[159]*self.scalar_static_f64[725]);
        self.scalar_static_f64[727]=(if ((self.scalar_static_f64[155])!=0.0){self.scalar_static_f64[726]}else{0.0});
        self.scalar_static_f64[728]=(self.scalar_static_f64[410]*self.scalar_static_f64[164]);
        self.scalar_static_f64[729]=(self.scalar_static_f64[728]/self.scalar_static_f64[153]);
        self.scalar_static_f64[730]=(self.scalar_static_f64[729]).exp();
        self.scalar_static_f64[731]=(self.scalar_static_f64[162]*self.scalar_static_f64[730]);
        self.scalar_static_f64[732]=(if ((self.scalar_static_f64[155])!=0.0){self.scalar_static_f64[731]}else{0.0});
        self.scalar_static_f64[733]=(self.scalar_static_f64[412]*self.scalar_static_f64[167]);
        self.scalar_static_f64[734]=(self.scalar_static_f64[733]).exp();
        self.scalar_static_f64[735]=(self.scalar_static_f64[165]*self.scalar_static_f64[734]);
        self.scalar_static_f64[736]=(self.scalar_static_f64[410]*self.scalar_static_f64[169]);
        self.scalar_static_f64[737]=(self.scalar_static_f64[736]).exp();
        self.scalar_static_f64[738]=(self.scalar_static_f64[735]*self.scalar_static_f64[737]);
        self.scalar_static_f64[739]=(self.scalar_static_f64[412]*self.scalar_static_f64[173]);
        self.scalar_static_f64[740]=(self.scalar_static_f64[739]).exp();
        self.scalar_static_f64[741]=(self.scalar_static_f64[170]*self.scalar_static_f64[740]);
        self.scalar_static_f64[742]=(self.scalar_static_f64[694]/self.scalar_static_f64[171]);
        self.scalar_static_f64[743]=(self.scalar_static_f64[742]).exp();
        self.scalar_static_f64[744]=(self.scalar_static_f64[741]*self.scalar_static_f64[743]);
        self.scalar_static_f64[745]=(self.scalar_static_f64[412]*self.scalar_static_f64[176]);
        self.scalar_static_f64[746]=(self.scalar_static_f64[745]).exp();
        self.scalar_static_f64[747]=(self.scalar_static_f64[174]*self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=(self.scalar_static_f64[694]/self.scalar_static_f64[175]);
        self.scalar_static_f64[749]=(self.scalar_static_f64[748]).exp();
        self.scalar_static_f64[750]=(self.scalar_static_f64[747]*self.scalar_static_f64[749]);
        self.scalar_static_f64[751]=(self.scalar_static_f64[407]).sqrt();
        self.scalar_static_f64[752]=(self.scalar_static_f64[177]*self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(self.scalar_static_f64[411]*self.scalar_static_f64[178]);
        self.scalar_static_f64[754]=(self.scalar_static_f64[753]).exp();
        self.scalar_static_f64[755]=(self.scalar_static_f64[752]*self.scalar_static_f64[754]);
        self.scalar_static_f64[756]=(self.scalar_static_f64[48]*self.scalar_static_f64[433]);
        self.scalar_static_f64[757]=f64::powf(self.scalar_static_f64[756],-0.5);
        self.scalar_static_f64[758]=(1.0/self.scalar_static_f64[579]);
        self.scalar_static_f64[759]=(self.scalar_static_f64[433]*self.scalar_static_f64[179]);
        self.scalar_static_f64[760]=(self.scalar_static_f64[433]*self.scalar_static_f64[759]);
        self.scalar_static_f64[761]=(self.scalar_static_f64[757]*self.scalar_static_f64[760]);
        self.scalar_static_f64[762]=(self.scalar_static_f64[758]*self.scalar_static_f64[761]);
        self.scalar_static_f64[763]=(self.scalar_static_f64[49]*self.scalar_static_f64[762]);
        self.scalar_static_f64[764]=(self.scalar_static_f64[576]*self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=(self.scalar_static_f64[48]*self.scalar_static_f64[764]);
        self.scalar_static_f64[766]=(self.scalar_static_f64[48]*self.scalar_static_f64[765]);
        self.scalar_static_f64[767]=(self.scalar_static_f64[757]*self.scalar_static_f64[180]);
        self.scalar_static_f64[768]=(self.scalar_static_f64[477]*self.scalar_static_f64[767]);
        self.scalar_static_f64[769]=(self.scalar_static_f64[477]*self.scalar_static_f64[768]);
        self.scalar_static_f64[770]=(self.scalar_static_f64[50]*self.scalar_static_f64[769]);
        self.scalar_static_f64[771]=(self.scalar_static_f64[50]*self.scalar_static_f64[770]);
        self.scalar_static_f64[772]=(self.scalar_static_f64[579]*self.scalar_static_f64[771]);
        self.scalar_static_f64[773]=(self.scalar_static_f64[179]-self.scalar_static_f64[766]);
        self.scalar_static_f64[774]=(self.scalar_static_f64[773]).exp();
        self.scalar_static_f64[775]=(self.scalar_static_f64[772]*self.scalar_static_f64[774]);
        self.scalar_static_f64[776]=(self.scalar_static_f64[80]*self.scalar_static_f64[454]);
        self.scalar_static_f64[777]=f64::powf(self.scalar_static_f64[776],-0.5);
        self.scalar_static_f64[778]=(1.0/self.scalar_static_f64[581]);
        self.scalar_static_f64[779]=(self.scalar_static_f64[454]*self.scalar_static_f64[181]);
        self.scalar_static_f64[780]=(self.scalar_static_f64[454]*self.scalar_static_f64[779]);
        self.scalar_static_f64[781]=(self.scalar_static_f64[777]*self.scalar_static_f64[780]);
        self.scalar_static_f64[782]=(self.scalar_static_f64[778]*self.scalar_static_f64[781]);
        self.scalar_static_f64[783]=(self.scalar_static_f64[51]*self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=(self.scalar_static_f64[577]*self.scalar_static_f64[783]);
        self.scalar_static_f64[785]=(self.scalar_static_f64[80]*self.scalar_static_f64[784]);
        self.scalar_static_f64[786]=(self.scalar_static_f64[80]*self.scalar_static_f64[785]);
        self.scalar_static_f64[787]=(self.scalar_static_f64[777]*self.scalar_static_f64[182]);
        self.scalar_static_f64[788]=(self.scalar_static_f64[535]*self.scalar_static_f64[787]);
        self.scalar_static_f64[789]=(self.scalar_static_f64[535]*self.scalar_static_f64[788]);
        self.scalar_static_f64[790]=(self.scalar_static_f64[81]*self.scalar_static_f64[789]);
        self.scalar_static_f64[791]=(self.scalar_static_f64[81]*self.scalar_static_f64[790]);
        self.scalar_static_f64[792]=(self.scalar_static_f64[581]*self.scalar_static_f64[791]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[181]-self.scalar_static_f64[786]);
        self.scalar_static_f64[794]=(self.scalar_static_f64[793]).exp();
        self.scalar_static_f64[795]=(self.scalar_static_f64[792]*self.scalar_static_f64[794]);
        self.scalar_static_f64[796]=(self.scalar_static_f64[412]*self.scalar_static_f64[105]);
        self.scalar_static_f64[797]=(self.scalar_static_f64[796]).exp();
        self.scalar_static_f64[798]=(self.scalar_static_f64[797]*self.scalar_static_f64[183]);
        self.scalar_static_f64[799]=(self.scalar_static_f64[590]*self.scalar_static_f64[798]);
        self.scalar_static_f64[800]=(self.scalar_static_f64[797]*self.scalar_static_f64[184]);
        self.scalar_static_f64[801]=(self.scalar_static_f64[758]*self.scalar_static_f64[800]);
        self.scalar_static_f64[802]=(self.scalar_static_f64[412]*self.scalar_static_f64[187]);
        self.scalar_static_f64[803]=(self.scalar_static_f64[802]).exp();
        self.scalar_static_f64[804]=(self.scalar_static_f64[185]*self.scalar_static_f64[803]);
        self.scalar_static_f64[805]=(self.scalar_static_f64[410]*self.scalar_static_f64[188]);
        self.scalar_static_f64[806]=(self.scalar_static_f64[805]).exp();
        self.scalar_static_f64[807]=(self.scalar_static_f64[804]*self.scalar_static_f64[806]);
        self.scalar_static_f64[808]=(self.scalar_static_f64[412]*self.scalar_static_f64[191]);
        self.scalar_static_f64[809]=(self.scalar_static_f64[808]).exp();
        self.scalar_static_f64[810]=(self.scalar_static_f64[18]*self.scalar_static_f64[809]);
        self.scalar_static_f64[811]=(self.scalar_static_f64[806]*self.scalar_static_f64[810]);
        self.scalar_static_f64[812]=(self.scalar_static_f64[412]*self.scalar_static_f64[193]);
        self.scalar_static_f64[813]=(self.scalar_static_f64[812]).exp();
        self.scalar_static_f64[814]=(self.scalar_static_f64[192]*self.scalar_static_f64[813]);
        self.scalar_static_f64[815]=(self.scalar_static_f64[412]*self.scalar_static_f64[195]);
        self.scalar_static_f64[816]=(self.scalar_static_f64[815]).exp();
        self.scalar_static_f64[817]=(self.scalar_static_f64[194]*self.scalar_static_f64[816]);
        self.scalar_static_f64[818]=(self.scalar_static_f64[412]*self.scalar_static_f64[197]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[818]).exp();
        self.scalar_static_f64[820]=(self.scalar_static_f64[196]*self.scalar_static_f64[819]);
        self.scalar_static_f64[821]=(self.scalar_static_f64[410]*self.scalar_static_f64[199]);
        self.scalar_static_f64[822]=(self.scalar_static_f64[821]).exp();
        self.scalar_static_f64[823]=(self.scalar_static_f64[820]*self.scalar_static_f64[822]);
        self.scalar_static_f64[824]=(self.scalar_static_f64[412]*self.scalar_static_f64[202]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[824]).exp();
        self.scalar_static_f64[826]=(self.scalar_static_f64[200]*self.scalar_static_f64[825]);
        self.scalar_static_f64[827]=(self.scalar_static_f64[412]*self.scalar_static_f64[204]);
        self.scalar_static_f64[828]=(self.scalar_static_f64[827]).exp();
        self.scalar_static_f64[829]=(self.scalar_static_f64[203]*self.scalar_static_f64[828]);
        self.scalar_static_f64[830]=(self.scalar_static_f64[826]+self.scalar_static_f64[829]);
        self.scalar_static_f64[831]=(self.scalar_static_f64[205]*self.scalar_static_f64[830]);
        self.scalar_static_f64[832]=(self.scalar_static_f64[831]/self.scalar_static_f64[206]);
        self.scalar_static_f64[833]=(self.scalar_static_f64[412]*self.scalar_static_f64[209]);
        self.scalar_static_f64[834]=(self.scalar_static_f64[833]).exp();
        self.scalar_static_f64[835]=(self.scalar_static_f64[207]*self.scalar_static_f64[834]);
        self.scalar_static_f64[836]=(self.scalar_static_f64[406]-300.0);
        self.scalar_static_bool[110]=(self.scalar_static_f64[406]<525.0);
        self.scalar_static_f64[837]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_f64[838]=(self.scalar_static_f64[836]*0.00072);
        self.scalar_static_f64[839]=(1.0+self.scalar_static_f64[838]);
        self.scalar_static_f64[840]=(self.scalar_static_f64[836]*1.6e-6);
        self.scalar_static_f64[841]=(self.scalar_static_f64[836]*self.scalar_static_f64[840]);
        self.scalar_static_f64[842]=(self.scalar_static_f64[839]-self.scalar_static_f64[841]);
        self.scalar_static_f64[843]=(self.scalar_static_f64[5]*self.scalar_static_f64[842]);
        self.scalar_static_f64[844]=(if ((self.scalar_static_f64[837])!=0.0){self.scalar_static_f64[843]}else{0.0});
        self.scalar_static_bool[111]=(!((self.scalar_static_f64[837])!=0.0));
        self.scalar_static_f64[845]=(if self.scalar_static_bool[111]{self.scalar_static_f64[210]}else{self.scalar_static_f64[844]});
        self.scalar_static_f64[846]=(self.scalar_static_f64[797]*self.scalar_static_f64[211]);
        self.scalar_static_f64[847]=(1.0/self.scalar_static_f64[608]);
        self.scalar_static_f64[848]=(if ((self.scalar_static_f64[212])!=0.0){self.scalar_static_f64[847]}else{0.0});
        self.scalar_static_bool[112]=(self.scalar_static_f64[848]>self.scalar_static_f64[17]);
        self.scalar_static_f64[849]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_bool[113]=(((self.scalar_static_f64[212])!=0.0)&&((self.scalar_static_f64[849])!=0.0));
        self.scalar_static_f64[850]=(if self.scalar_static_bool[113]{self.scalar_static_f64[17]}else{self.scalar_static_f64[848]});
        self.scalar_static_f64[851]=(if self.scalar_static_bool[16]{0.0}else{self.scalar_static_f64[850]});
        self.scalar_static_f64[852]=(1.0/self.scalar_static_f64[611]);
        self.scalar_static_f64[853]=(if ((self.scalar_static_f64[213])!=0.0){self.scalar_static_f64[852]}else{0.0});
        self.scalar_static_bool[114]=(self.scalar_static_f64[853]>self.scalar_static_f64[17]);
        self.scalar_static_f64[854]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_bool[115]=(((self.scalar_static_f64[213])!=0.0)&&((self.scalar_static_f64[854])!=0.0));
        self.scalar_static_f64[855]=(if self.scalar_static_bool[115]{self.scalar_static_f64[17]}else{self.scalar_static_f64[853]});
        self.scalar_static_f64[856]=(if self.scalar_static_bool[18]{0.0}else{self.scalar_static_f64[855]});
        self.scalar_static_f64[857]=(1.0/self.scalar_static_f64[612]);
        self.scalar_static_f64[858]=(if ((self.scalar_static_f64[214])!=0.0){self.scalar_static_f64[857]}else{0.0});
        self.scalar_static_bool[116]=(self.scalar_static_f64[858]>self.scalar_static_f64[17]);
        self.scalar_static_f64[859]=(if self.scalar_static_bool[116]{1.0}else{0.0});
        self.scalar_static_bool[117]=(((self.scalar_static_f64[214])!=0.0)&&((self.scalar_static_f64[859])!=0.0));
        self.scalar_static_f64[860]=(if self.scalar_static_bool[117]{self.scalar_static_f64[17]}else{self.scalar_static_f64[858]});
        self.scalar_static_f64[861]=(if self.scalar_static_bool[20]{0.0}else{self.scalar_static_f64[860]});
        self.scalar_static_f64[862]=(2.0*self.scalar_static_f64[408]);
        self.scalar_static_f64[863]=(self.scalar_static_f64[497]*0.2);
        self.scalar_static_f64[864]=(self.scalar_static_f64[615]*self.scalar_static_f64[218]);
        self.scalar_static_f64[865]=(self.scalar_static_f64[409]*self.scalar_static_f64[497]);
        self.scalar_static_f64[866]=(self.scalar_static_f64[865]).exp();
        self.scalar_static_f64[867]=(self.scalar_static_f64[615]*self.scalar_static_f64[219]);
        self.scalar_static_f64[868]=(self.scalar_static_f64[218]*self.scalar_static_f64[867]);
        self.scalar_static_f64[869]=(0.1*self.scalar_static_f64[535]);
        self.scalar_static_f64[870]=(self.scalar_static_f64[408]*1e-5);
        self.scalar_static_f64[871]=(self.scalar_static_f64[408]*1e-40);
        self.scalar_static_f64[872]=(self.scalar_static_f64[477]*self.scalar_static_f64[234]);
        self.scalar_static_f64[873]=(0.1*self.scalar_static_f64[477]);
        self.scalar_static_f64[874]=(self.scalar_static_f64[477]/self.scalar_static_f64[235]);
        self.scalar_static_f64[875]=(2.0-self.scalar_static_f64[592]);
        self.scalar_static_f64[876]=(1.0-self.scalar_static_f64[592]);
        self.scalar_static_f64[877]=(self.scalar_static_f64[875]/self.scalar_static_f64[876]);
        self.scalar_static_f64[878]=f64::powf(self.scalar_static_f64[877],self.scalar_static_f64[239]);
        self.scalar_static_f64[879]=(1.0-self.scalar_static_f64[878]);
        self.scalar_static_f64[880]=(self.scalar_static_f64[535]*self.scalar_static_f64[879]);
        self.scalar_static_f64[881]=(self.scalar_static_f64[535]/self.scalar_static_f64[241]);
        self.scalar_static_f64[882]=(4.0*self.scalar_static_f64[684]);
        self.scalar_static_f64[883]=(self.scalar_static_f64[882]/self.scalar_static_f64[687]);
        self.scalar_static_f64[884]=(1.0/self.scalar_static_f64[663]);
        self.scalar_static_f64[885]=(self.scalar_static_f64[409]*self.scalar_static_f64[846]);
        self.scalar_static_f64[886]=(self.scalar_static_f64[885]).exp();
        self.scalar_static_f64[887]=(self.scalar_static_f64[886]-1.0);
        self.scalar_static_f64[888]=(self.scalar_static_f64[684]*self.scalar_static_f64[243]);
        self.scalar_static_f64[889]=(2.0*self.scalar_static_f64[723]);
        self.scalar_static_f64[890]=(2.0*self.scalar_static_f64[732]);
        self.scalar_static_f64[891]=(2.0*self.scalar_static_f64[775]);
        self.scalar_static_f64[892]=(2.0*self.scalar_static_f64[795]);
        self.scalar_static_f64[893]=(2.0*self.scalar_static_f64[738]);
        self.scalar_static_f64[894]=(4.0*self.scalar_static_f64[738]);
        self.scalar_static_f64[895]=(self.scalar_static_f64[894]/self.scalar_static_f64[690]);
        self.scalar_static_f64[896]=(self.scalar_static_f64[807]*self.scalar_static_f64[261]);
        self.scalar_static_f64[897]=(self.scalar_static_f64[807]/self.scalar_static_f64[814]);
        self.scalar_static_f64[898]=(4.0*self.scalar_static_f64[897]);
        self.scalar_static_f64[899]=(self.scalar_static_f64[807]*self.scalar_static_f64[264]);
        self.scalar_static_f64[900]=(2.0*self.scalar_static_f64[811]);
        self.scalar_static_f64[901]=(self.scalar_static_f64[811]/self.scalar_static_f64[817]);
        self.scalar_static_f64[902]=(self.scalar_static_f64[265]*self.scalar_static_f64[901]);
        self.scalar_static_f64[903]=(self.scalar_static_f64[738]*self.scalar_static_f64[268]);
        self.scalar_static_f64[904]=(self.scalar_static_f64[807]*self.scalar_static_f64[270]);
        self.scalar_static_f64[905]=(4.0*self.scalar_static_f64[807]);
        self.scalar_static_f64[906]=(self.scalar_static_f64[905]/self.scalar_static_f64[814]);
        self.scalar_static_f64[907]=(self.scalar_static_f64[738]+self.scalar_static_f64[807]);
        self.scalar_static_f64[908]=(self.scalar_static_f64[6]*self.scalar_static_f64[907]);
        self.scalar_static_f64[909]=(self.scalar_static_f64[608]*self.scalar_static_f64[908]);
        self.scalar_static_f64[910]=(if self.scalar_static_bool[50]{self.scalar_static_f64[909]}else{0.0});
        self.scalar_static_f64[911]=(self.scalar_static_f64[409]*self.scalar_static_f64[910]);
        self.scalar_static_f64[912]=(self.scalar_static_f64[911]).ln();
        self.scalar_static_f64[913]=(2.0-self.scalar_static_f64[912]);
        self.scalar_static_f64[914]=(self.scalar_static_f64[408]*self.scalar_static_f64[913]);
        self.scalar_static_f64[915]=(if self.scalar_static_bool[50]{self.scalar_static_f64[914]}else{0.0});
        self.scalar_static_f64[916]=(-self.scalar_static_f64[676]);
        self.scalar_static_f64[917]=(self.scalar_static_f64[298]/self.scalar_static_f64[676]);
        self.scalar_static_f64[918]=(self.scalar_static_f64[4]/self.scalar_static_f64[845]);
        self.scalar_static_f64[919]=(-self.scalar_static_f64[845]);
        self.scalar_static_f64[920]=(self.scalar_static_f64[582]*self.scalar_static_f64[321]);
        self.scalar_static_f64[921]=(self.scalar_static_f64[582]*self.scalar_static_f64[320]);
        self.scalar_static_f64[922]=(self.scalar_static_f64[591]*self.scalar_static_f64[322]);
        self.scalar_static_f64[923]=(self.scalar_static_f64[687]*self.scalar_static_f64[826]);
        self.scalar_static_f64[924]=(0.5*self.scalar_static_f64[923]);
        self.scalar_static_f64[925]=(0.1*self.scalar_static_f64[575]);
        self.scalar_static_f64[926]=(self.scalar_static_f64[575]*self.scalar_static_f64[326]);
        self.scalar_static_f64[927]=(self.scalar_static_f64[575]/self.scalar_static_f64[327]);
        self.scalar_static_f64[928]=(self.scalar_static_f64[687]*self.scalar_static_f64[823]);
        self.scalar_static_f64[929]=(self.scalar_static_f64[684]/self.scalar_static_f64[687]);
        self.scalar_static_f64[930]=f64::powf(self.scalar_static_f64[929],self.scalar_static_f64[329]);
        self.scalar_static_f64[931]=(self.scalar_static_f64[928]*self.scalar_static_f64[930]);
        self.scalar_static_f64[932]=(self.scalar_static_f64[408]*self.scalar_static_f64[328]);
        self.scalar_static_f64[933]=(4.0*self.scalar_static_f64[829]);
        self.scalar_static_f64[934]=(self.scalar_static_f64[408]*self.scalar_static_f64[933]);
        self.scalar_static_f64[935]=(self.scalar_static_f64[934]/self.scalar_static_f64[615]);
        self.scalar_static_f64[936]=(0.5*self.scalar_static_f64[935]);
        self.scalar_static_f64[937]=(0.5*self.scalar_static_f64[832]);
        self.scalar_static_f64[938]=(self.scalar_static_f64[835]*self.scalar_static_f64[893]);
        self.scalar_static_f64[939]=(self.scalar_static_f64[832]*self.scalar_static_f64[334]);
        self.scalar_static_f64[940]=(self.scalar_static_f64[835]*self.scalar_static_f64[903]);
        self.scalar_static_f64[941]=(self.scalar_static_f64[0]*self.scalar_static_f64[409]);
        self.scalar_static_f64[942]=(self.scalar_static_f64[409]*self.scalar_static_f64[352]);
        self.scalar_static_f64[943]=(self.scalar_static_f64[942]/self.scalar_static_f64[639]);
        self.scalar_static_f64[944]=(self.scalar_static_f64[941]/self.scalar_static_f64[639]);
        self.scalar_static_f64[945]=(self.scalar_static_f64[409]*self.scalar_static_f64[353]);
        self.scalar_static_f64[946]=(self.scalar_static_f64[409]*self.scalar_static_f64[354]);
        self.scalar_static_f64[947]=(self.scalar_static_f64[409]*self.scalar_static_f64[355]);
        self.scalar_static_f64[948]=(self.scalar_static_f64[352]/self.scalar_static_f64[873]);
        self.scalar_static_f64[949]=(self.scalar_static_f64[0]/self.scalar_static_f64[873]);
        self.scalar_static_f64[950]=(-self.scalar_static_f64[948]);
        self.scalar_static_f64[951]=(-self.scalar_static_f64[949]);
        self.scalar_static_f64[952]=(self.scalar_static_f64[0]*self.scalar_static_f64[592]);
        self.scalar_static_f64[953]=(self.scalar_static_f64[592]*self.scalar_static_f64[352]);
        self.scalar_static_f64[954]=(self.scalar_static_f64[884]-1.0);
        self.scalar_static_f64[955]=(self.scalar_static_f64[942]/self.scalar_static_f64[149]);
        self.scalar_static_f64[956]=(self.scalar_static_f64[941]/self.scalar_static_f64[149]);
        self.scalar_static_f64[957]=(self.scalar_static_f64[942]/self.scalar_static_f64[153]);
        self.scalar_static_f64[958]=(self.scalar_static_f64[941]/self.scalar_static_f64[153]);
        self.scalar_static_f64[959]=(self.scalar_static_f64[942]/self.scalar_static_f64[136]);
        self.scalar_static_f64[960]=(self.scalar_static_f64[941]/self.scalar_static_f64[136]);
        self.scalar_static_f64[961]=(self.scalar_static_f64[942]/self.scalar_static_f64[171]);
        self.scalar_static_f64[962]=(self.scalar_static_f64[941]/self.scalar_static_f64[171]);
        self.scalar_static_f64[963]=(self.scalar_static_f64[941]/self.scalar_static_f64[142]);
        self.scalar_static_f64[964]=(self.scalar_static_f64[945]/self.scalar_static_f64[142]);
        self.scalar_static_f64[965]=(self.scalar_static_f64[946]/self.scalar_static_f64[142]);
        self.scalar_static_f64[966]=(self.scalar_static_f64[942]/self.scalar_static_f64[142]);
        self.scalar_static_f64[967]=(self.scalar_static_f64[942]/self.scalar_static_f64[175]);
        self.scalar_static_f64[968]=(self.scalar_static_f64[941]/self.scalar_static_f64[175]);
        self.scalar_static_f64[969]=(self.scalar_static_f64[576]*self.scalar_static_f64[352]);
        self.scalar_static_f64[970]=(self.scalar_static_f64[0]*self.scalar_static_f64[576]);
        self.scalar_static_f64[971]=(self.scalar_static_f64[766]*self.scalar_static_f64[370]);
        self.scalar_static_f64[972]=(self.scalar_static_f64[766]*self.scalar_static_f64[371]);
        self.scalar_static_f64[973]=(self.scalar_static_f64[0]*self.scalar_static_f64[577]);
        self.scalar_static_f64[974]=(self.scalar_static_f64[577]*self.scalar_static_f64[352]);
        self.scalar_static_f64[975]=(-self.scalar_static_f64[973]);
        self.scalar_static_f64[976]=(-self.scalar_static_f64[974]);
        self.scalar_static_f64[977]=(self.scalar_static_f64[786]*self.scalar_static_f64[375]);
        self.scalar_static_f64[978]=(self.scalar_static_f64[786]*self.scalar_static_f64[376]);
        self.scalar_static_f64[979]=(self.scalar_static_f64[917]*self.scalar_static_f64[352]);
        self.scalar_static_f64[980]=(self.scalar_static_f64[0]*self.scalar_static_f64[917]);
        self.scalar_static_f64[981]=(self.scalar_static_f64[0]/self.scalar_static_f64[869]);
        self.scalar_static_f64[982]=(self.scalar_static_f64[353]/self.scalar_static_f64[869]);
        self.scalar_static_f64[983]=(self.scalar_static_f64[354]/self.scalar_static_f64[869]);
        self.scalar_static_f64[984]=(self.scalar_static_f64[352]/self.scalar_static_f64[869]);
        self.scalar_static_f64[985]=(-self.scalar_static_f64[981]);
        self.scalar_static_f64[986]=(-self.scalar_static_f64[982]);
        self.scalar_static_f64[987]=(-self.scalar_static_f64[983]);
        self.scalar_static_f64[988]=(-self.scalar_static_f64[984]);
        self.scalar_static_f64[989]=(self.scalar_static_f64[592]*self.scalar_static_f64[353]);
        self.scalar_static_f64[990]=(self.scalar_static_f64[592]*self.scalar_static_f64[354]);
        self.scalar_static_f64[991]=(self.scalar_static_f64[355]/self.scalar_static_f64[869]);
        self.scalar_static_f64[992]=(-self.scalar_static_f64[991]);
        self.scalar_static_f64[993]=(self.scalar_static_f64[592]*self.scalar_static_f64[355]);
        self.scalar_static_f64[994]=(self.scalar_static_f64[0]/self.scalar_static_f64[925]);
        self.scalar_static_f64[995]=(self.scalar_static_f64[352]/self.scalar_static_f64[925]);
        self.scalar_static_f64[996]=(-self.scalar_static_f64[994]);
        self.scalar_static_f64[997]=(-self.scalar_static_f64[995]);
        self.scalar_static_f64[998]=(self.scalar_static_f64[352]/self.scalar_static_f64[932]);
        self.scalar_static_f64[999]=(self.scalar_static_f64[0]/self.scalar_static_f64[932]);
        self.scalar_static_f64[1000]=(self.scalar_static_f64[409]*self.scalar_static_f64[391]);
        self.scalar_static_f64[1001]=(self.scalar_static_f64[409]*self.scalar_static_f64[392]);
        self.scalar_static_f64[1002]=(self.scalar_static_f64[409]*self.scalar_static_f64[393]);
        self.scalar_static_f64[1003]=(self.scalar_static_f64[409]*self.scalar_static_f64[394]);
        self.scalar_static_f64[1004]=(if ((self.scalar_static_f64[336])!=0.0){self.scalar_static_f64[948]}else{0.0});
        self.scalar_static_f64[1005]=(if ((self.scalar_static_f64[336])!=0.0){self.scalar_static_f64[949]}else{0.0});
        self.scalar_static_f64[1006]=(-self.scalar_static_f64[1004]);
        self.scalar_static_f64[1007]=(-self.scalar_static_f64[1005]);
        self.scalar_static_f64[1008]=(self.scalar_static_f64[398]/self.scalar_static_f64[597]);
        self.scalar_static_f64[1009]=(self.scalar_static_f64[399]/self.scalar_static_f64[597]);
        self.scalar_static_f64[1010]=(self.scalar_static_f64[15]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1011]=(self.scalar_static_f64[15]*self.scalar_static_f64[1009]);
        self.scalar_static_f64[1012]=(self.scalar_static_f64[398]/self.scalar_static_f64[605]);
        self.scalar_static_f64[1013]=(self.scalar_static_f64[399]/self.scalar_static_f64[605]);
        self.scalar_static_f64[1014]=(self.scalar_static_f64[15]*self.scalar_static_f64[1012]);
        self.scalar_static_f64[1015]=(self.scalar_static_f64[15]*self.scalar_static_f64[1013]);
        self.scalar_static_f64[1016]=(self.scalar_static_f64[851]*self.scalar_static_f64[398]);
        self.scalar_static_f64[1017]=(self.scalar_static_f64[851]*self.scalar_static_f64[404]);
        self.scalar_static_f64[1018]=(self.scalar_static_f64[851]*self.scalar_static_f64[405]);
        self.scalar_static_f64[1019]=(self.scalar_static_f64[851]*self.scalar_static_f64[399]);
        self.scalar_static_f64[1020]=(self.scalar_static_f64[15]*self.scalar_static_f64[1016]);
        self.scalar_static_f64[1021]=(self.scalar_static_f64[15]*self.scalar_static_f64[1017]);
        self.scalar_static_f64[1022]=(self.scalar_static_f64[15]*self.scalar_static_f64[1018]);
        self.scalar_static_f64[1023]=(self.scalar_static_f64[15]*self.scalar_static_f64[1019]);
        self.scalar_static_f64[1024]=(self.scalar_static_f64[856]*self.scalar_static_f64[398]);
        self.scalar_static_f64[1025]=(self.scalar_static_f64[856]*self.scalar_static_f64[399]);
        self.scalar_static_f64[1026]=(self.scalar_static_f64[15]*self.scalar_static_f64[1024]);
        self.scalar_static_f64[1027]=(self.scalar_static_f64[15]*self.scalar_static_f64[1025]);
        self.scalar_static_f64[1028]=(if ((self.scalar_static_f64[213])!=0.0){self.scalar_static_f64[1026]}else{0.0});
        self.scalar_static_f64[1029]=(if ((self.scalar_static_f64[213])!=0.0){self.scalar_static_f64[1027]}else{0.0});
        self.scalar_static_f64[1030]=(self.scalar_static_f64[861]*self.scalar_static_f64[399]);
        self.scalar_static_f64[1031]=(self.scalar_static_f64[861]*self.scalar_static_f64[398]);
        self.scalar_static_f64[1032]=(self.scalar_static_f64[15]*self.scalar_static_f64[1030]);
        self.scalar_static_f64[1033]=(self.scalar_static_f64[15]*self.scalar_static_f64[1031]);
        self.scalar_static_f64[1034]=(if ((self.scalar_static_f64[214])!=0.0){self.scalar_static_f64[1032]}else{0.0});
        self.scalar_static_f64[1035]=(if ((self.scalar_static_f64[214])!=0.0){self.scalar_static_f64[1033]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
