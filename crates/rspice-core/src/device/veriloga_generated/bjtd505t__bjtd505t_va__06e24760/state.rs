#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

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
    pub p144: f64, pub p145: f64, pub p146: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 141] = [
                0.0, 1.0, 505.5, 1.0, 25.0, 1.0, 1.0, 0.0,
                2.2e-17, 1.0, 1.0, 0.1, 2.5, 44.0, 1.0, 1.0000000000000001e-19,
                1.0, 0.0, 1.0, 2.7000000000000005e-15, 2.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.68, 0.0, 3.1400000000000002e-18, 0.014289999999999999, 1e-15, 2.0,
                0.63, 0.0, 22.0, 0.0, 22.0, 1e-6, 1.0, 400.0,
                -0.37, 0.5, 25.0, 0.1, 1.1e-6, 3.0, 0.3, 0.004,
                -0.37, -0.37, 0.3, 0.004, 1.0, 5.0, 23.0, 18.0,
                12.0, 0.0, 0.0, 150.0, 1250.0, 0.004, 0.3, 0.68,
                7.3e-14, 0.95, 0.4, 0.4, 0.0, 7.800000000000001e-14, 0.68, 0.5,
                0.0, 0.0, 0.35, 0.5, 0.032, 0.0, 0.0, 0.68,
                100.0, 4.0, 1000.0, 0.0, 1.0, 2e-12, 4.2e-12, 4.1e-11,
                5.2e-10, 1e-11, 1.0, 0.0, 0.0, 0.3333333333333333, 0.0, 0.3,
                0.0, 1.0, 2.5, 2.5, 0.62, 2.0, 1.3, 2.0,
                1.17, 1.12, 1.12, 1.12, 1.12, 1.18, 1.12, 1.125,
                1.15, 1.15, 0.000473, 636.0, 1.15, 0.000473, 636.0, 0.05,
                0.0, 0.0, 0.0, 0.0005, 200.0, 2.0, 2.0, 2e-11,
                2e-11, 0.0, 0.0, 0.0, 0.0, 300.0, 3.0000000000000004e-9, 0.0,
                0.0, 2.0, 400.0, 1e-40, 1e-40,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 141);
            {
                let params = &mut *ptr;
                params.p141 = 0.001;
                validate_parameter("minr", params.p141, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 0.0, 0.16, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(142), 5);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 149] = [
    ("dta", 0), ("trise", 0), ("dtemp", 0), ("mult", 1), ("version", 2), ("type", 3), ("tref", 4), ("exmod", 5), ("exphi", 6), ("exavl", 7), ("is", 8), ("nff", 9), ("nfr", 10), ("ik", 11), ("ver", 12), ("vef", 13),
    ("issr", 14), ("ibi", 15), ("nbi", 16), ("ibis", 17), ("nbis", 18), ("ibf", 19), ("mlf", 20), ("ibfs", 21), ("mlfs", 22), ("swib1", 23), ("ibinbr", 24), ("ibinbrs", 25), ("vknbr", 26), ("ibinbrqs", 27), ("ibx", 28), ("ikbx", 29),
    ("ibr", 30), ("mlr", 31), ("xext", 32), ("izeb", 33), ("nzeb", 34), ("izcb", 35), ("nzcb", 36), ("vzmin", 37), ("swavl", 38), ("aavl", 39), ("cavl", 40), ("itoavl", 41), ("bavl", 42), ("vdcavl", 43), ("wavl", 44), ("vavl", 45),
    ("sfh", 46), ("ihcavl", 47), ("davl", 48), ("eavl", 49), ("aexavl", 50), ("ionexavl", 51), ("swgemlim", 52), ("re", 53), ("rbc", 54), ("rbv", 55), ("rcc", 56), ("rcblx", 57), ("rcbli", 58), ("rcv", 59), ("scrcv", 60), ("ihc", 61),
    ("axi", 62), ("vdc", 63), ("cje", 64), ("vde", 65), ("pe", 66), ("xcje", 67), ("cbeo", 68), ("cjc", 69), ("vdcctc", 70), ("pc", 71), ("swvchc", 72), ("swvjunc", 73), ("xp", 74), ("mc", 75), ("xcjc", 76), ("cbco", 77),
    ("swqex", 78), ("vdcex", 79), ("vbrcb", 80), ("pbrcb", 81), ("frevcb", 82), ("swjbrcb", 83), ("mtau", 84), ("taue", 85), ("taub", 86), ("tepi", 87), ("taur", 88), ("tauex", 89), ("nex", 90), ("deg", 91), ("xrec", 92), ("xqb", 93),
    ("ke", 94), ("aqbo", 95), ("ae", 96), ("ab", 97), ("aepi", 98), ("aepiex", 99), ("aex", 100), ("ac", 101), ("acx", 102), ("acbl", 103), ("vgb", 104), ("vgbnbrqs", 105), ("vgbnbr", 106), ("vgbnbrs", 107), ("vgknbr", 108), ("vgc", 109),
    ("vge", 110), ("vgcx", 111), ("vgj", 112), ("vgzeb", 113), ("avgeb", 114), ("tvgeb", 115), ("vgzcb", 116), ("avgcb", 117), ("tvgcb", 118), ("dvgte", 119), ("dais", 120), ("tnff", 121), ("tnfr", 122), ("tbavl", 123), ("dtmax", 124), ("af", 125),
    ("afn", 126), ("kf", 127), ("kfn", 128), ("kavl", 129), ("kc", 130), ("ftaun", 131), ("swnlsh", 132), ("rth", 133), ("cth", 134), ("ath", 135), ("isibrel", 136), ("nfibrel", 137), ("vexlim", 138), ("p0starlim", 139), ("pwlim", 140), ("minr", 141),
    ("istat", 142), ("vtat", 143), ("ktat", 144), ("vbtbt", 145), ("kbtbt", 146),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 147] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 147] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 147] = [
    "dta", "mult", "version", "type", "tref", "exmod", "exphi", "exavl", "is", "nff", "nfr", "ik", "ver", "vef", "issr", "ibi",
    "nbi", "ibis", "nbis", "ibf", "mlf", "ibfs", "mlfs", "swib1", "ibinbr", "ibinbrs", "vknbr", "ibinbrqs", "ibx", "ikbx", "ibr", "mlr",
    "xext", "izeb", "nzeb", "izcb", "nzcb", "vzmin", "swavl", "aavl", "cavl", "itoavl", "bavl", "vdcavl", "wavl", "vavl", "sfh", "ihcavl",
    "davl", "eavl", "aexavl", "ionexavl", "swgemlim", "re", "rbc", "rbv", "rcc", "rcblx", "rcbli", "rcv", "scrcv", "ihc", "axi", "vdc",
    "cje", "vde", "pe", "xcje", "cbeo", "cjc", "vdcctc", "pc", "swvchc", "swvjunc", "xp", "mc", "xcjc", "cbco", "swqex", "vdcex",
    "vbrcb", "pbrcb", "frevcb", "swjbrcb", "mtau", "taue", "taub", "tepi", "taur", "tauex", "nex", "deg", "xrec", "xqb", "ke", "aqbo",
    "ae", "ab", "aepi", "aepiex", "aex", "ac", "acx", "acbl", "vgb", "vgbnbrqs", "vgbnbr", "vgbnbrs", "vgknbr", "vgc", "vge", "vgcx",
    "vgj", "vgzeb", "avgeb", "tvgeb", "vgzcb", "avgcb", "tvgcb", "dvgte", "dais", "tnff", "tnfr", "tbavl", "dtmax", "af", "afn", "kf",
    "kfn", "kavl", "kc", "ftaun", "swnlsh", "rth", "cth", "ath", "isibrel", "nfibrel", "vexlim", "p0starlim", "pwlim", "minr", "istat", "vtat",
    "ktat", "vbtbt", "kbtbt",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 147] = [
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
    &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 147] = [
    false, false, false, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 147] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 505.5, label: "505.5" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }),
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.02, label: "0.02" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 40.0, label: "40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 147] = [
    None, None, Some(ParameterBound { value: 505.51, label: "505.51" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.99, label: "0.99" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 2000.0, label: "2000.0" }), Some(ParameterBound { value: 500.0, label: "500.0" }), Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 400.0, label: "400.0" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), None, None, None,
    None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 147] = [
    0, 3, 2, 0, 2, 0, 0, 0, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 2, 2, 2,
    0, 2, 2, 2, 2, 3, 0, 2, 3, 3, 3, 0, 2, 2, 2, 2, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 2, 0, 2, 1, 1, 1, 0, 2, 2, 3, 2, 2, 2, 2, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2,
    2, 0, 0, 0, 0, 3, 2, 0, 2, 2, 0, 0, 0, 2, 2, 3, 0, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 147] = [
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
    &[], &[], &[],
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

#[derive(Clone)]
pub(crate) struct StampState<const DDT: usize, const IDT: usize> {
    pub(crate) ddt_current: [f64; DDT],
    pub(crate) ddt_previous: [f64; DDT],
    pub(crate) ddt_older: [f64; DDT],
    pub(crate) ddt_derivative_current: [f64; DDT],
    pub(crate) ddt_derivative_previous: [f64; DDT],
    pub(crate) idt_current: [f64; IDT],
    pub(crate) idt_previous: [f64; IDT],
    pub(crate) ddt_initialized: [bool; DDT],
    pub(crate) idt_initialized: [bool; IDT],
}

impl<const DDT: usize, const IDT: usize> StampState<DDT, IDT> {
    fn new_box() -> Box<Self> {
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            // SAFETY: every field is an array of f64 or bool; all-zero bytes are valid values for both.
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }
}

#[derive(Clone)]
pub(crate) struct ScalarStaticState<const F64_COUNT: usize, const BOOL_COUNT: usize> {
    pub(crate) f64_values: [f64; F64_COUNT],
    pub(crate) bool_values: [bool; BOOL_COUNT],
    pub(crate) instance_dirty: bool,
}

impl<const F64_COUNT: usize, const BOOL_COUNT: usize> ScalarStaticState<F64_COUNT, BOOL_COUNT> {
    fn new_box() -> Box<Self> {
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            // SAFETY: every field is an array of f64 or bool, plus bool; all-zero bytes are valid values.
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            let mut boxed = boxed.assume_init();
            boxed.instance_dirty = true;
            boxed
        }
    }
}

pub struct Instance {
    pub nodes: [usize; 12],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 147]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<10, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<406, 86>>,
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
            stamp_state: self.stamp_state.clone(),
            time: self.time,
            timestep: self.timestep,
            ddt_coefficients: self.ddt_coefficients,
            scalar_static: self.scalar_static.clone(),
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
    pub const PARAMETER_COUNT: usize = 147;
    pub const VARIABLE_COUNT: usize = 585;
    pub const DDT_STATE_COUNT: usize = 10;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "9ea4a77c8fbb04f05c1dbb544ebde1580625c5492d239019e2b735b1034cf9f5";
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            stamp_state: StampState::new_box(),
            time: 0.0,
            timestep: 0.0,
            ddt_coefficients: GeneratedDdtCoefficients::inactive(),
            scalar_static: ScalarStaticState::new_box(),
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(50);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(10);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 50);
        debug_assert_eq!(state.flags.len(), 10);
        let mut rollback_values = state.values.as_slice();
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_previous.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_older.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_derivative_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_derivative_previous.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_current.copy_from_slice(field);
        rollback_values = remaining;
        let (field, remaining) = rollback_values.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_previous.copy_from_slice(field);
        rollback_values = remaining;
        let mut rollback_flags = state.flags.as_slice();
        let (field, remaining) = rollback_flags.split_at(Self::DDT_STATE_COUNT);
        self.stamp_state.ddt_initialized.copy_from_slice(field);
        rollback_flags = remaining;
        let (field, remaining) = rollback_flags.split_at(Self::IDT_STATE_COUNT);
        self.stamp_state.idt_initialized.copy_from_slice(field);
        rollback_flags = remaining;
        debug_assert!(rollback_values.is_empty());
        debug_assert!(rollback_flags.is_empty());
    }

    pub(crate) fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {
        GeneratedVerilogAPersistentState {
            ddt_previous: self.stamp_state.ddt_previous.to_vec(),
            ddt_older: self.stamp_state.ddt_older.to_vec(),
            ddt_derivative_previous: self.stamp_state.ddt_derivative_previous.to_vec(),
            ddt_initialized: self.stamp_state.ddt_initialized.to_vec(),
            idt_previous: self.stamp_state.idt_previous.to_vec(),
            idt_initialized: self.stamp_state.idt_initialized.to_vec(),
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
        self.stamp_state.ddt_previous.copy_from_slice(&state.ddt_previous);
        self.stamp_state.ddt_current.copy_from_slice(&state.ddt_previous);
        self.stamp_state.ddt_older.copy_from_slice(&state.ddt_older);
        self.stamp_state.ddt_derivative_previous.copy_from_slice(&state.ddt_derivative_previous);
        self.stamp_state.ddt_derivative_current.copy_from_slice(&state.ddt_derivative_previous);
        self.stamp_state.ddt_initialized.copy_from_slice(&state.ddt_initialized);
        self.stamp_state.idt_previous.copy_from_slice(&state.idt_previous);
        self.stamp_state.idt_current.copy_from_slice(&state.idt_previous);
        self.stamp_state.idt_initialized.copy_from_slice(&state.idt_initialized);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505t_va'", name));
        };
        validate_parameter_scalar_metadata(index, value)?;
        let was_given = self.param_given[index];
        let value_changed = self.write_parameter_slot(index, value);
        self.finish_set_parameter(index, value_changed || !was_given);
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
    fn write_parameter_slot(&mut self, index: usize, value: f64) -> bool {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        // SAFETY: Parameters is repr(C), contains only f64 fields, and index is produced from generated parameter metadata.
        unsafe {
            let ptr = self.params.as_mut() as *mut Parameters as *mut f64;
            let changed = (*ptr.add(index)).to_bits() != value.to_bits();
            *ptr.add(index) = value;
            changed
        }
    }

    #[inline]
    fn finish_set_parameter(&mut self, index: usize, invalidates_caches: bool) {
        self.mark_param_given(index);
        if invalidates_caches {
            self.scalar_static.instance_dirty = true;
            self.invalidate_temperature_static();
        }
    }

    #[inline]
    fn mark_param_given(&mut self, index: usize) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        self.param_given[index] = true;
    }

    #[inline]
    pub fn set_multiplicity(&mut self, multiplicity: f64) -> Result<(), String> {
        if multiplicity.is_finite() && multiplicity > 0.0 {
            self.multiplicity = multiplicity;
            Ok(())
        } else {
            Err(format!("instance multiplicity 'm' must be finite and > 0.0, got {}", multiplicity))
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
            self.stamp_state.ddt_older[index] = self.stamp_state.ddt_previous[index];
            self.stamp_state.ddt_previous[index] = self.stamp_state.ddt_current[index];
            self.stamp_state.ddt_derivative_previous[index] = self.stamp_state.ddt_derivative_current[index];
            self.stamp_state.ddt_initialized[index] = true;
            index += 1;
        }
        let mut index = 0usize;
        while index < Self::IDT_STATE_COUNT {
            self.stamp_state.idt_previous[index] = self.stamp_state.idt_current[index];
            self.stamp_state.idt_initialized[index] = true;
            index += 1;
        }
    }

    #[inline]
    pub(crate) fn eval_ddt(&mut self, slot: usize, value: f64) -> f64 {
        debug_assert!(slot < Self::DDT_STATE_COUNT, "generated ddt state slot out of range");
        let previous = if self.stamp_state.ddt_initialized[slot] {
            self.stamp_state.ddt_previous[slot]
        } else {
            value
        };
        let older = if self.stamp_state.ddt_initialized[slot] {
            self.stamp_state.ddt_older[slot]
        } else {
            value
        };
        self.stamp_state.ddt_current[slot] = value;
        if self.ddt_coefficients.active {
            let result = value * self.ddt_coefficients.derivative_scale
                - previous * self.ddt_coefficients.previous_value_scale
                - older * self.ddt_coefficients.older_value_scale
                - self.stamp_state.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;
            self.stamp_state.ddt_derivative_current[slot] = result;
            result
        } else {
            self.stamp_state.ddt_current[slot] = value;
            self.stamp_state.ddt_previous[slot] = value;
            self.stamp_state.ddt_older[slot] = value;
            self.stamp_state.ddt_derivative_current[slot] = 0.0;
            self.stamp_state.ddt_derivative_previous[slot] = 0.0;
            self.stamp_state.ddt_initialized[slot] = true;
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
    pub(super) fn ensure_instance_static(&mut self) {
        if self.scalar_static.instance_dirty {
            self.recompute_instance_static();
        }
    }

    #[inline]
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        self.scalar_static.f64_values[0]=p.p3;
        self.scalar_static.bool_values[0]=(self.scalar_static.f64_values[0]==1.0);
        self.scalar_static.f64_values[1]=(if self.scalar_static.bool_values[0]{1.0}else{0.0});
        self.scalar_static.f64_values[2]=(if ((self.scalar_static.f64_values[1])!=0.0){70300000.0}else{0.0});
        self.scalar_static.f64_values[3]=(if ((self.scalar_static.f64_values[1])!=0.0){123000000.0}else{0.0});
        self.scalar_static.bool_values[1]=(!((self.scalar_static.f64_values[1])!=0.0));
        self.scalar_static.f64_values[4]=(if self.scalar_static.bool_values[1]{158000000.0}else{self.scalar_static.f64_values[2]});
        self.scalar_static.f64_values[5]=(if self.scalar_static.bool_values[1]{204000000.0}else{self.scalar_static.f64_values[3]});
        self.scalar_static.f64_values[6]=p.p32;
        self.scalar_static.f64_values[7]=(1.0-self.scalar_static.f64_values[6]);
        self.scalar_static.f64_values[8]=p.p4;
        self.scalar_static.f64_values[9]=(self.scalar_static.f64_values[8]+273.15);
        self.scalar_static.f64_values[10]=p.p0;
        self.scalar_static.f64_values[11]=p.p141;
        self.scalar_static.bool_values[2]=(0.0==self.scalar_static.f64_values[11]);
        self.scalar_static.f64_values[12]=(if self.scalar_static.bool_values[2]{1.0}else{0.0});
        self.scalar_static.f64_values[13]=(if ((self.scalar_static.f64_values[12])!=0.0){1e-12}else{0.0});
        self.scalar_static.bool_values[3]=(!((self.scalar_static.f64_values[12])!=0.0));
        self.scalar_static.f64_values[14]=(if self.scalar_static.bool_values[3]{self.scalar_static.f64_values[11]}else{self.scalar_static.f64_values[13]});
        self.scalar_static.f64_values[15]=p.p1;
        self.scalar_static.f64_values[16]=(self.scalar_static.f64_values[14]*self.scalar_static.f64_values[15]);
        self.scalar_static.f64_values[17]=(1.0/self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[18]=p.p66;
        self.scalar_static.f64_values[19]=(2.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[20]=f64::powf(2.0,self.scalar_static.f64_values[19]);
        self.scalar_static.f64_values[21]=(1.0/self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[22]=p.p113;
        self.scalar_static.f64_values[23]=p.p114;
        self.scalar_static.f64_values[24]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[25]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[24]);
        self.scalar_static.f64_values[26]=p.p115;
        self.scalar_static.f64_values[27]=(self.scalar_static.f64_values[9]+self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[28]=(self.scalar_static.f64_values[25]/self.scalar_static.f64_values[27]);
        self.scalar_static.f64_values[29]=(self.scalar_static.f64_values[22]+self.scalar_static.f64_values[28]);
        self.scalar_static.f64_values[30]=(self.scalar_static.f64_values[29]-0.05);
        self.scalar_static.f64_values[31]=(self.scalar_static.f64_values[30]/0.1);
        self.scalar_static.bool_values[4]=(self.scalar_static.f64_values[29]<0.05);
        self.scalar_static.f64_values[32]=(if self.scalar_static.bool_values[4]{1.0}else{0.0});
        self.scalar_static.f64_values[33]=(self.scalar_static.f64_values[31]).exp();
        self.scalar_static.f64_values[34]=(1.0+self.scalar_static.f64_values[33]);
        self.scalar_static.f64_values[35]=(self.scalar_static.f64_values[34]).ln();
        self.scalar_static.f64_values[36]=(0.1*self.scalar_static.f64_values[35]);
        self.scalar_static.f64_values[37]=(0.05+self.scalar_static.f64_values[36]);
        self.scalar_static.f64_values[38]=(if ((self.scalar_static.f64_values[32])!=0.0){self.scalar_static.f64_values[37]}else{0.0});
        self.scalar_static.bool_values[5]=(!((self.scalar_static.f64_values[32])!=0.0));
        self.scalar_static.f64_values[39]=(-self.scalar_static.f64_values[31]);
        self.scalar_static.f64_values[40]=(self.scalar_static.f64_values[39]).exp();
        self.scalar_static.f64_values[41]=(1.0+self.scalar_static.f64_values[40]);
        self.scalar_static.f64_values[42]=(self.scalar_static.f64_values[41]).ln();
        self.scalar_static.f64_values[43]=(0.1*self.scalar_static.f64_values[42]);
        self.scalar_static.f64_values[44]=(self.scalar_static.f64_values[29]+self.scalar_static.f64_values[43]);
        self.scalar_static.f64_values[45]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[44]}else{self.scalar_static.f64_values[38]});
        self.scalar_static.f64_values[46]=(1.0/self.scalar_static.f64_values[22]);
        self.scalar_static.f64_values[47]=p.p65;
        self.scalar_static.f64_values[48]=(1.0/self.scalar_static.f64_values[47]);
        self.scalar_static.f64_values[49]=p.p70;
        self.scalar_static.f64_values[50]=p.p71;
        self.scalar_static.f64_values[51]=(2.0-self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[52]=f64::powf(2.0,self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[53]=(1.0/self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[54]=p.p116;
        self.scalar_static.f64_values[55]=p.p117;
        self.scalar_static.f64_values[56]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[55]);
        self.scalar_static.f64_values[57]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[56]);
        self.scalar_static.f64_values[58]=p.p118;
        self.scalar_static.f64_values[59]=(self.scalar_static.f64_values[9]+self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[60]=(self.scalar_static.f64_values[57]/self.scalar_static.f64_values[59]);
        self.scalar_static.f64_values[61]=(self.scalar_static.f64_values[54]+self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[62]=(self.scalar_static.f64_values[61]-0.05);
        self.scalar_static.f64_values[63]=(self.scalar_static.f64_values[62]/0.1);
        self.scalar_static.bool_values[6]=(self.scalar_static.f64_values[61]<0.05);
        self.scalar_static.f64_values[64]=(if self.scalar_static.bool_values[6]{1.0}else{0.0});
        self.scalar_static.f64_values[65]=(self.scalar_static.f64_values[63]).exp();
        self.scalar_static.f64_values[66]=(1.0+self.scalar_static.f64_values[65]);
        self.scalar_static.f64_values[67]=(self.scalar_static.f64_values[66]).ln();
        self.scalar_static.f64_values[68]=(0.1*self.scalar_static.f64_values[67]);
        self.scalar_static.f64_values[69]=(0.05+self.scalar_static.f64_values[68]);
        self.scalar_static.f64_values[70]=(if ((self.scalar_static.f64_values[64])!=0.0){self.scalar_static.f64_values[69]}else{0.0});
        self.scalar_static.bool_values[7]=(!((self.scalar_static.f64_values[64])!=0.0));
        self.scalar_static.f64_values[71]=(-self.scalar_static.f64_values[63]);
        self.scalar_static.f64_values[72]=(self.scalar_static.f64_values[71]).exp();
        self.scalar_static.f64_values[73]=(1.0+self.scalar_static.f64_values[72]);
        self.scalar_static.f64_values[74]=(self.scalar_static.f64_values[73]).ln();
        self.scalar_static.f64_values[75]=(0.1*self.scalar_static.f64_values[74]);
        self.scalar_static.f64_values[76]=(self.scalar_static.f64_values[61]+self.scalar_static.f64_values[75]);
        self.scalar_static.f64_values[77]=(if self.scalar_static.bool_values[7]{self.scalar_static.f64_values[76]}else{self.scalar_static.f64_values[70]});
        self.scalar_static.f64_values[78]=(1.0/self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[79]=(1.0/self.scalar_static.f64_values[49]);
        self.scalar_static.f64_values[80]=p.p82;
        self.scalar_static.f64_values[81]=(1.0/self.scalar_static.f64_values[80]);
        self.scalar_static.f64_values[82]=(1.0-self.scalar_static.f64_values[81]);
        self.scalar_static.f64_values[83]=p.p124;
        self.scalar_static.f64_values[84]=(self.scalar_static.f64_values[9]*8.617086918058125e-5);
        self.scalar_static.f64_values[85]=(1.0/self.scalar_static.f64_values[84]);
        self.scalar_static.f64_values[86]=p.p104;
        self.scalar_static.f64_values[87]=p.p63;
        self.scalar_static.f64_values[88]=p.p109;
        self.scalar_static.f64_values[89]=p.p79;
        self.scalar_static.f64_values[90]=p.p26;
        self.scalar_static.f64_values[91]=p.p108;
        self.scalar_static.f64_values[92]=p.p64;
        self.scalar_static.f64_values[93]=p.p74;
        self.scalar_static.f64_values[94]=(1.0-self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[95]=p.p69;
        self.scalar_static.f64_values[96]=p.p53;
        self.scalar_static.f64_values[97]=p.p96;
        self.scalar_static.f64_values[98]=p.p55;
        self.scalar_static.f64_values[99]=p.p97;
        self.scalar_static.f64_values[100]=p.p95;
        self.scalar_static.f64_values[101]=(self.scalar_static.f64_values[99]-self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[102]=p.p54;
        self.scalar_static.f64_values[103]=p.p100;
        self.scalar_static.f64_values[104]=p.p56;
        self.scalar_static.f64_values[105]=p.p101;
        self.scalar_static.f64_values[106]=p.p57;
        self.scalar_static.f64_values[107]=p.p103;
        self.scalar_static.f64_values[108]=p.p58;
        self.scalar_static.f64_values[109]=p.p59;
        self.scalar_static.f64_values[110]=p.p98;
        self.scalar_static.f64_values[111]=p.p121;
        self.scalar_static.bool_values[8]=(0.0!=self.scalar_static.f64_values[111]);
        self.scalar_static.f64_values[112]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.f64_values[113]=p.p9;
        self.scalar_static.bool_values[9]=(!((self.scalar_static.f64_values[112])!=0.0));
        self.scalar_static.f64_values[114]=p.p122;
        self.scalar_static.bool_values[10]=(0.0!=self.scalar_static.f64_values[114]);
        self.scalar_static.f64_values[115]=(if self.scalar_static.bool_values[10]{1.0}else{0.0});
        self.scalar_static.f64_values[116]=p.p10;
        self.scalar_static.bool_values[11]=(!((self.scalar_static.f64_values[115])!=0.0));
        self.scalar_static.f64_values[117]=p.p42;
        self.scalar_static.f64_values[118]=p.p123;
        self.scalar_static.f64_values[119]=p.p8;
        self.scalar_static.f64_values[120]=(4.0-self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[121]=(self.scalar_static.f64_values[120]-self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[122]=p.p120;
        self.scalar_static.f64_values[123]=(self.scalar_static.f64_values[121]+self.scalar_static.f64_values[122]);
        self.scalar_static.f64_values[124]=(-self.scalar_static.f64_values[86]);
        self.scalar_static.f64_values[125]=p.p11;
        self.scalar_static.f64_values[126]=(1.0-self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[127]=p.p29;
        self.scalar_static.f64_values[128]=p.p102;
        self.scalar_static.f64_values[129]=(1.0-self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[130]=p.p19;
        self.scalar_static.f64_values[131]=p.p20;
        self.scalar_static.f64_values[132]=(2.0*self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[133]=(6.0-self.scalar_static.f64_values[132]);
        self.scalar_static.f64_values[134]=p.p112;
        self.scalar_static.f64_values[135]=(-self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[136]=p.p30;
        self.scalar_static.f64_values[137]=p.p31;
        self.scalar_static.f64_values[138]=(2.0*self.scalar_static.f64_values[137]);
        self.scalar_static.f64_values[139]=(6.0-self.scalar_static.f64_values[138]);
        self.scalar_static.f64_values[140]=(-self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[141]=p.p15;
        self.scalar_static.f64_values[142]=(4.0-self.scalar_static.f64_values[97]);
        self.scalar_static.f64_values[143]=(self.scalar_static.f64_values[122]+self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[144]=p.p16;
        self.scalar_static.f64_values[145]=p.p110;
        self.scalar_static.f64_values[146]=(-self.scalar_static.f64_values[145]);
        self.scalar_static.f64_values[147]=p.p17;
        self.scalar_static.f64_values[148]=p.p18;
        self.scalar_static.f64_values[149]=p.p23;
        self.scalar_static.bool_values[12]=(1.0==self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[150]=(if self.scalar_static.bool_values[12]{1.0}else{0.0});
        self.scalar_static.f64_values[151]=p.p24;
        self.scalar_static.f64_values[152]=p.p106;
        self.scalar_static.f64_values[153]=(-self.scalar_static.f64_values[152]);
        self.scalar_static.f64_values[154]=p.p27;
        self.scalar_static.f64_values[155]=p.p105;
        self.scalar_static.f64_values[156]=(-self.scalar_static.f64_values[155]);
        self.scalar_static.f64_values[157]=p.p25;
        self.scalar_static.f64_values[158]=p.p107;
        self.scalar_static.f64_values[159]=(-self.scalar_static.f64_values[158]);
        self.scalar_static.f64_values[160]=p.p28;
        self.scalar_static.f64_values[161]=(4.0-self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[162]=(self.scalar_static.f64_values[122]+self.scalar_static.f64_values[161]);
        self.scalar_static.f64_values[163]=p.p111;
        self.scalar_static.f64_values[164]=(-self.scalar_static.f64_values[163]);
        self.scalar_static.f64_values[165]=p.p21;
        self.scalar_static.f64_values[166]=p.p22;
        self.scalar_static.f64_values[167]=(2.0*self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[168]=(6.0-self.scalar_static.f64_values[167]);
        self.scalar_static.f64_values[169]=p.p136;
        self.scalar_static.f64_values[170]=p.p137;
        self.scalar_static.f64_values[171]=(4.0/self.scalar_static.f64_values[170]);
        self.scalar_static.f64_values[172]=p.p142;
        self.scalar_static.f64_values[173]=p.p144;
        self.scalar_static.f64_values[174]=p.p34;
        self.scalar_static.f64_values[175]=p.p33;
        self.scalar_static.f64_values[176]=p.p36;
        self.scalar_static.f64_values[177]=p.p35;
        self.scalar_static.f64_values[178]=p.p13;
        self.scalar_static.f64_values[179]=p.p12;
        self.scalar_static.f64_values[180]=p.p85;
        self.scalar_static.f64_values[181]=(self.scalar_static.f64_values[99]-2.0);
        self.scalar_static.f64_values[182]=p.p119;
        self.scalar_static.f64_values[183]=(-self.scalar_static.f64_values[182]);
        self.scalar_static.f64_values[184]=p.p86;
        self.scalar_static.f64_values[185]=(self.scalar_static.f64_values[99]+self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[186]=(self.scalar_static.f64_values[185]-1.0);
        self.scalar_static.f64_values[187]=p.p87;
        self.scalar_static.f64_values[188]=(self.scalar_static.f64_values[110]-1.0);
        self.scalar_static.f64_values[189]=p.p88;
        self.scalar_static.f64_values[190]=(self.scalar_static.f64_values[184]+self.scalar_static.f64_values[187]);
        self.scalar_static.f64_values[191]=p.p89;
        self.scalar_static.f64_values[192]=p.p99;
        self.scalar_static.f64_values[193]=(self.scalar_static.f64_values[192]-1.0);
        self.scalar_static.f64_values[194]=(self.scalar_static.f64_values[5]*1.081);
        self.scalar_static.f64_values[195]=p.p91;
        self.scalar_static.f64_values[196]=p.p133;
        self.scalar_static.f64_values[197]=p.p135;
        self.scalar_static.bool_values[13]=(self.scalar_static.f64_values[104]>0.0);
        self.scalar_static.f64_values[198]=(if self.scalar_static.bool_values[13]{1.0}else{0.0});
        self.scalar_static.bool_values[14]=(!((self.scalar_static.f64_values[198])!=0.0));
        self.scalar_static.bool_values[15]=(self.scalar_static.f64_values[106]>0.0);
        self.scalar_static.f64_values[199]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.bool_values[16]=(!((self.scalar_static.f64_values[199])!=0.0));
        self.scalar_static.bool_values[17]=(self.scalar_static.f64_values[108]>0.0);
        self.scalar_static.f64_values[200]=(if self.scalar_static.bool_values[17]{1.0}else{0.0});
        self.scalar_static.bool_values[18]=(!((self.scalar_static.f64_values[200])!=0.0));
        self.scalar_static.f64_values[201]=p.p138;
        self.scalar_static.f64_values[202]=(self.scalar_static.f64_values[201]).exp();
        self.scalar_static.f64_values[203]=p.p140;
        self.scalar_static.f64_values[204]=p.p61;
        self.scalar_static.f64_values[205]=p.p60;
        self.scalar_static.f64_values[206]=(self.scalar_static.f64_values[204]*self.scalar_static.f64_values[205]);
        self.scalar_static.f64_values[207]=p.p62;
        self.scalar_static.f64_values[208]=(-1.0/self.scalar_static.f64_values[207]);
        self.scalar_static.f64_values[209]=(self.scalar_static.f64_values[208]).exp();
        self.scalar_static.f64_values[210]=(1.0+self.scalar_static.f64_values[209]);
        self.scalar_static.f64_values[211]=(self.scalar_static.f64_values[210]).ln();
        self.scalar_static.f64_values[212]=(self.scalar_static.f64_values[207]*self.scalar_static.f64_values[211]);
        self.scalar_static.f64_values[213]=(1.0+self.scalar_static.f64_values[212]);
        self.scalar_static.f64_values[214]=p.p139;
        self.scalar_static.f64_values[215]=(0.5*self.scalar_static.f64_values[205]);
        self.scalar_static.f64_values[216]=p.p72;
        self.scalar_static.bool_values[19]=(0.0==self.scalar_static.f64_values[216]);
        self.scalar_static.f64_values[217]=(if self.scalar_static.bool_values[19]{1.0}else{0.0});
        self.scalar_static.bool_values[20]=(!((self.scalar_static.f64_values[217])!=0.0));
        self.scalar_static.f64_values[218]=(-1.0/self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[219]=f64::powf(3.0,self.scalar_static.f64_values[218]);
        self.scalar_static.f64_values[220]=(1.0-self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[221]=(1.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[222]=p.p73;
        self.scalar_static.bool_values[21]=(1.0==self.scalar_static.f64_values[222]);
        self.scalar_static.f64_values[223]=(if self.scalar_static.bool_values[21]{1.0}else{0.0});
        self.scalar_static.bool_values[22]=(2.0==self.scalar_static.f64_values[222]);
        self.scalar_static.f64_values[224]=(if self.scalar_static.bool_values[22]{1.0}else{0.0});
        self.scalar_static.bool_values[23]=(!((self.scalar_static.f64_values[223])!=0.0));
        self.scalar_static.bool_values[24]=(((self.scalar_static.f64_values[224])!=0.0)&&self.scalar_static.bool_values[23]);
        self.scalar_static.bool_values[25]=(!((self.scalar_static.f64_values[224])!=0.0));
        self.scalar_static.bool_values[26]=(self.scalar_static.bool_values[23]&&self.scalar_static.bool_values[25]);
        self.scalar_static.f64_values[225]=(-1.0/self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[226]=p.p75;
        self.scalar_static.f64_values[227]=(1.0-self.scalar_static.f64_values[50]);
        self.scalar_static.bool_values[27]=(0.0==self.scalar_static.f64_values[195]);
        self.scalar_static.f64_values[228]=(if self.scalar_static.bool_values[27]{1.0}else{0.0});
        self.scalar_static.bool_values[28]=(!((self.scalar_static.f64_values[228])!=0.0));
        self.scalar_static.f64_values[229]=p.p14;
        self.scalar_static.f64_values[230]=p.p143;
        self.scalar_static.f64_values[231]=p.p145;
        self.scalar_static.f64_values[232]=p.p146;
        self.scalar_static.f64_values[233]=p.p92;
        self.scalar_static.bool_values[29]=(0.0==self.scalar_static.f64_values[233]);
        self.scalar_static.f64_values[234]=(if self.scalar_static.bool_values[29]{1.0}else{0.0});
        self.scalar_static.bool_values[30]=(!((self.scalar_static.f64_values[150])!=0.0));
        self.scalar_static.bool_values[31]=(((self.scalar_static.f64_values[234])!=0.0)&&self.scalar_static.bool_values[30]);
        self.scalar_static.bool_values[32]=(!((self.scalar_static.f64_values[234])!=0.0));
        self.scalar_static.bool_values[33]=(self.scalar_static.bool_values[30]&&self.scalar_static.bool_values[32]);
        self.scalar_static.f64_values[235]=(1.0-self.scalar_static.f64_values[233]);
        self.scalar_static.bool_values[34]=(self.scalar_static.f64_values[175]>0.0);
        self.scalar_static.bool_values[35]=(self.scalar_static.f64_values[174]>0.0);
        self.scalar_static.bool_values[36]=(self.scalar_static.bool_values[34]&&self.scalar_static.bool_values[35]);
        self.scalar_static.f64_values[236]=(-2.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[237]=(self.scalar_static.f64_values[18]*self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[238]=(1.0-self.scalar_static.f64_values[237]);
        self.scalar_static.f64_values[239]=(self.scalar_static.f64_values[18]-1.0);
        self.scalar_static.bool_values[37]=(self.scalar_static.f64_values[177]>0.0);
        self.scalar_static.bool_values[38]=(self.scalar_static.f64_values[176]>0.0);
        self.scalar_static.bool_values[39]=(self.scalar_static.bool_values[37]&&self.scalar_static.bool_values[38]);
        self.scalar_static.f64_values[240]=(-2.0-self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[241]=(self.scalar_static.f64_values[50]*self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[242]=(1.0-self.scalar_static.f64_values[241]);
        self.scalar_static.f64_values[243]=(self.scalar_static.f64_values[50]-1.0);
        self.scalar_static.f64_values[244]=p.p5;
        self.scalar_static.bool_values[40]=(self.scalar_static.f64_values[244]>0.0);
        self.scalar_static.bool_values[41]=(self.scalar_static.f64_values[6]>0.0);
        self.scalar_static.bool_values[42]=(self.scalar_static.bool_values[40]&&self.scalar_static.bool_values[41]);
        self.scalar_static.f64_values[245]=(if self.scalar_static.bool_values[42]{1.0}else{0.0});
        self.scalar_static.f64_values[246]=(self.scalar_static.f64_values[6]*2.0);
        self.scalar_static.bool_values[43]=(1.0==self.scalar_static.f64_values[244]);
        self.scalar_static.f64_values[247]=(if self.scalar_static.bool_values[43]{1.0}else{0.0});
        self.scalar_static.bool_values[44]=(((self.scalar_static.f64_values[245])!=0.0)&&((self.scalar_static.f64_values[247])!=0.0));
        self.scalar_static.f64_values[248]=(if self.scalar_static.bool_values[44]{0.0121}else{0.010000000000000002});
        self.scalar_static.f64_values[249]=(0.5*self.scalar_static.f64_values[248]);
        self.scalar_static.bool_values[45]=(!((self.scalar_static.f64_values[247])!=0.0));
        self.scalar_static.bool_values[46]=(((self.scalar_static.f64_values[245])!=0.0)&&self.scalar_static.bool_values[45]);
        self.scalar_static.f64_values[250]=p.p83;
        self.scalar_static.bool_values[47]=(1.0==self.scalar_static.f64_values[250]);
        self.scalar_static.f64_values[251]=(if self.scalar_static.bool_values[47]{1.0}else{0.0});
        self.scalar_static.f64_values[252]=(if ((self.scalar_static.f64_values[251])!=0.0){1e-12}else{self.scalar_static.f64_values[248]});
        self.scalar_static.f64_values[253]=(0.5*self.scalar_static.f64_values[252]);
        self.scalar_static.f64_values[254]=p.p81;
        self.scalar_static.f64_values[255]=f64::powf(self.scalar_static.f64_values[82],self.scalar_static.f64_values[254]);
        self.scalar_static.f64_values[256]=(1.0-self.scalar_static.f64_values[255]);
        self.scalar_static.f64_values[257]=(1.0/self.scalar_static.f64_values[256]);
        self.scalar_static.f64_values[258]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[257]}else{0.0});
        self.scalar_static.f64_values[259]=p.p80;
        self.scalar_static.f64_values[260]=(self.scalar_static.f64_values[82]*self.scalar_static.f64_values[259]);
        self.scalar_static.f64_values[261]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[260]}else{0.0});
        self.scalar_static.f64_values[262]=(self.scalar_static.f64_values[258]*self.scalar_static.f64_values[258]);
        self.scalar_static.f64_values[263]=(self.scalar_static.f64_values[254]-1.0);
        self.scalar_static.f64_values[264]=f64::powf(self.scalar_static.f64_values[82],self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[265]=(self.scalar_static.f64_values[262]*self.scalar_static.f64_values[264]);
        self.scalar_static.f64_values[266]=(self.scalar_static.f64_values[254]*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[267]=(self.scalar_static.f64_values[266]/self.scalar_static.f64_values[259]);
        self.scalar_static.f64_values[268]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[267]}else{0.0});
        self.scalar_static.bool_values[48]=(!((self.scalar_static.f64_values[251])!=0.0));
        self.scalar_static.f64_values[269]=p.p38;
        self.scalar_static.bool_values[49]=(1.0==self.scalar_static.f64_values[269]);
        self.scalar_static.f64_values[270]=(if self.scalar_static.bool_values[49]{1.0}else{0.0});
        self.scalar_static.f64_values[271]=p.p43;
        self.scalar_static.f64_values[272]=p.p41;
        self.scalar_static.f64_values[273]=p.p40;
        self.scalar_static.f64_values[274]=p.p39;
        self.scalar_static.bool_values[50]=(2.0==self.scalar_static.f64_values[269]);
        self.scalar_static.f64_values[275]=(if self.scalar_static.bool_values[50]{1.0}else{0.0});
        self.scalar_static.bool_values[51]=(!((self.scalar_static.f64_values[270])!=0.0));
        self.scalar_static.f64_values[276]=p.p45;
        self.scalar_static.f64_values[277]=(2.0*self.scalar_static.f64_values[276]);
        self.scalar_static.f64_values[278]=p.p44;
        self.scalar_static.f64_values[279]=(self.scalar_static.f64_values[278]*self.scalar_static.f64_values[278]);
        self.scalar_static.f64_values[280]=(self.scalar_static.f64_values[277]/self.scalar_static.f64_values[279]);
        self.scalar_static.f64_values[281]=p.p7;
        self.scalar_static.bool_values[52]=(0.0==self.scalar_static.f64_values[281]);
        self.scalar_static.f64_values[282]=(if self.scalar_static.bool_values[52]{1.0}else{0.0});
        self.scalar_static.bool_values[53]=(!((self.scalar_static.f64_values[282])!=0.0));
        self.scalar_static.f64_values[283]=p.p46;
        self.scalar_static.f64_values[284]=(2.0*self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[285]=(1.0+self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[286]=(1.0+self.scalar_static.f64_values[284]);
        self.scalar_static.f64_values[287]=(self.scalar_static.f64_values[285]/self.scalar_static.f64_values[286]);
        self.scalar_static.bool_values[54]=(3.0==self.scalar_static.f64_values[269]);
        self.scalar_static.f64_values[288]=(if self.scalar_static.bool_values[54]{1.0}else{0.0});
        self.scalar_static.bool_values[55]=(!((self.scalar_static.f64_values[275])!=0.0));
        self.scalar_static.f64_values[289]=p.p47;
        self.scalar_static.f64_values[290]=p.p48;
        self.scalar_static.f64_values[291]=p.p51;
        self.scalar_static.f64_values[292]=p.p50;
        self.scalar_static.f64_values[293]=p.p49;
        self.scalar_static.f64_values[294]=p.p52;
        self.scalar_static.bool_values[56]=(1.0==self.scalar_static.f64_values[294]);
        self.scalar_static.f64_values[295]=(if self.scalar_static.bool_values[56]{1.0}else{0.0});
        self.scalar_static.bool_values[57]=(!((self.scalar_static.f64_values[288])!=0.0));
        self.scalar_static.bool_values[58]=(!((self.scalar_static.f64_values[295])!=0.0));
        self.scalar_static.f64_values[296]=p.p67;
        self.scalar_static.f64_values[297]=(1.0-self.scalar_static.f64_values[296]);
        self.scalar_static.f64_values[298]=p.p76;
        self.scalar_static.f64_values[299]=(1.0-self.scalar_static.f64_values[298]);
        self.scalar_static.f64_values[300]=p.p84;
        self.scalar_static.f64_values[301]=(1.0/self.scalar_static.f64_values[300]);
        self.scalar_static.f64_values[302]=p.p78;
        self.scalar_static.bool_values[59]=(0.0==self.scalar_static.f64_values[302]);
        self.scalar_static.f64_values[303]=(if self.scalar_static.bool_values[59]{1.0}else{0.0});
        self.scalar_static.f64_values[304]=p.p90;
        self.scalar_static.bool_values[60]=(!((self.scalar_static.f64_values[303])!=0.0));
        self.scalar_static.bool_values[61]=(3.0==self.scalar_static.f64_values[244]);
        self.scalar_static.bool_values[62]=(self.scalar_static.bool_values[43]||self.scalar_static.bool_values[61]);
        self.scalar_static.bool_values[63]=(self.scalar_static.bool_values[41]&&self.scalar_static.bool_values[62]);
        self.scalar_static.f64_values[305]=(if self.scalar_static.bool_values[63]{1.0}else{0.0});
        self.scalar_static.bool_values[64]=(((self.scalar_static.f64_values[303])!=0.0)&&((self.scalar_static.f64_values[305])!=0.0));
        self.scalar_static.f64_values[306]=(self.scalar_static.f64_values[6]*0.5);
        self.scalar_static.bool_values[65]=(self.scalar_static.bool_values[60]&&((self.scalar_static.f64_values[305])!=0.0));
        self.scalar_static.f64_values[307]=p.p6;
        self.scalar_static.bool_values[66]=(1.0==self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[308]=(if self.scalar_static.bool_values[66]{1.0}else{0.0});
        self.scalar_static.f64_values[309]=(-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[310]=p.p94;
        self.scalar_static.f64_values[311]=(1.0-self.scalar_static.f64_values[310]);
        self.scalar_static.f64_values[312]=p.p93;
        self.scalar_static.f64_values[313]=(1.0-self.scalar_static.f64_values[312]);
        self.scalar_static.bool_values[67]=(!((self.scalar_static.f64_values[308])!=0.0));
        self.scalar_static.f64_values[314]=p.p134;
        self.scalar_static.f64_values[315]=(1.0-self.scalar_static.f64_values[197]);
        self.scalar_static.bool_values[68]=(self.scalar_static.f64_values[196]>self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[316]=(if self.scalar_static.bool_values[68]{1.0}else{0.0});
        self.scalar_static.f64_values[317]=p.p132;
        self.scalar_static.bool_values[69]=(0.0==self.scalar_static.f64_values[317]);
        self.scalar_static.f64_values[318]=(if self.scalar_static.bool_values[69]{1.0}else{0.0});
        self.scalar_static.bool_values[70]=(((self.scalar_static.f64_values[316])!=0.0)&&((self.scalar_static.f64_values[318])!=0.0));
        self.scalar_static.f64_values[319]=(self.scalar_static.f64_values[315]).abs();
        self.scalar_static.bool_values[71]=(self.scalar_static.f64_values[319]<1e-6);
        self.scalar_static.f64_values[320]=(if self.scalar_static.bool_values[71]{1.0}else{0.0});
        self.scalar_static.bool_values[72]=(!((self.scalar_static.f64_values[318])!=0.0));
        self.scalar_static.bool_values[73]=(((self.scalar_static.f64_values[316])!=0.0)&&self.scalar_static.bool_values[72]);
        self.scalar_static.bool_values[74]=(((self.scalar_static.f64_values[320])!=0.0)&&self.scalar_static.bool_values[73]);
        self.scalar_static.bool_values[75]=(!((self.scalar_static.f64_values[320])!=0.0));
        self.scalar_static.bool_values[76]=(self.scalar_static.bool_values[73]&&self.scalar_static.bool_values[75]);
        self.scalar_static.bool_values[77]=(!((self.scalar_static.f64_values[316])!=0.0));
        self.scalar_static.f64_values[321]=p.p129;
        self.scalar_static.bool_values[78]=(self.scalar_static.f64_values[321]>0.0);
        self.scalar_static.f64_values[322]=(if self.scalar_static.bool_values[78]{1.0}else{0.0});
        self.scalar_static.bool_values[79]=(!((self.scalar_static.f64_values[322])!=0.0));
        self.scalar_static.f64_values[323]=p.p130;
        self.scalar_static.bool_values[80]=(1.0==self.scalar_static.f64_values[323]);
        self.scalar_static.f64_values[324]=(if self.scalar_static.bool_values[80]{1.0}else{0.0});
        self.scalar_static.bool_values[81]=(2.0==self.scalar_static.f64_values[323]);
        self.scalar_static.f64_values[325]=(if self.scalar_static.bool_values[81]{1.0}else{0.0});
        self.scalar_static.bool_values[82]=(!((self.scalar_static.f64_values[324])!=0.0));
        self.scalar_static.bool_values[83]=(((self.scalar_static.f64_values[325])!=0.0)&&self.scalar_static.bool_values[82]);
        self.scalar_static.f64_values[326]=p.p131;
        self.scalar_static.bool_values[84]=(!((self.scalar_static.f64_values[325])!=0.0));
        self.scalar_static.bool_values[85]=(self.scalar_static.bool_values[82]&&self.scalar_static.bool_values[84]);
        self.scalar_static.f64_values[327]=p.p68;
        self.scalar_static.f64_values[328]=p.p77;
        self.scalar_static.f64_values[329]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[327]);
        self.scalar_static.f64_values[330]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[328]);
        self.scalar_static.f64_values[331]=(-self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[332]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[333]=(self.scalar_static.f64_values[331]-self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[335]=(self.scalar_static.f64_values[221]-1.0);
        self.scalar_static.f64_values[336]=(if ((self.scalar_static.f64_values[223])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[337]=(if ((self.scalar_static.f64_values[223])!=0.0){self.scalar_static.f64_values[331]}else{0.0});
        self.scalar_static.f64_values[338]=(self.scalar_static.f64_values[225]-1.0);
        self.scalar_static.f64_values[339]=(self.scalar_static.f64_values[226]-1.0);
        self.scalar_static.f64_values[340]=(self.scalar_static.f64_values[227]-1.0);
        self.scalar_static.f64_values[341]=(self.scalar_static.f64_values[331]/0.0001);
        self.scalar_static.f64_values[342]=(self.scalar_static.f64_values[0]/0.0001);
        self.scalar_static.f64_values[343]=(-self.scalar_static.f64_values[341]);
        self.scalar_static.f64_values[344]=(-self.scalar_static.f64_values[342]);
        self.scalar_static.f64_values[345]=(self.scalar_static.f64_values[331]/0.001);
        self.scalar_static.f64_values[346]=(self.scalar_static.f64_values[0]/0.001);
        self.scalar_static.f64_values[347]=(-self.scalar_static.f64_values[345]);
        self.scalar_static.f64_values[348]=(-self.scalar_static.f64_values[346]);
        self.scalar_static.f64_values[349]=(self.scalar_static.f64_values[236]-1.0);
        self.scalar_static.f64_values[350]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[351]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[352]=(0.5*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[353]=(self.scalar_static.f64_values[0]*0.5);
        self.scalar_static.f64_values[354]=(self.scalar_static.f64_values[240]-1.0);
        self.scalar_static.f64_values[355]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[356]=(self.scalar_static.f64_values[52]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[357]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[332]}else{0.0});
        self.scalar_static.f64_values[358]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[334]}else{0.0});
        self.scalar_static.f64_values[359]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[333]}else{0.0});
        self.scalar_static.f64_values[360]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[331]}else{0.0});
        self.scalar_static.f64_values[361]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[362]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[332]}else{0.0});
        self.scalar_static.f64_values[363]=(if ((self.scalar_static.f64_values[251])!=0.0){self.scalar_static.f64_values[331]}else{0.0});
        self.scalar_static.f64_values[364]=(-self.scalar_static.f64_values[361]);
        self.scalar_static.f64_values[365]=(-self.scalar_static.f64_values[362]);
        self.scalar_static.f64_values[366]=(-self.scalar_static.f64_values[363]);
        self.scalar_static.f64_values[367]=(self.scalar_static.f64_values[273]-1.0);
        self.scalar_static.f64_values[368]=(self.scalar_static.f64_values[290]-1.0);
        self.scalar_static.f64_values[369]=(self.scalar_static.f64_values[293]-1.0);
        self.scalar_static.f64_values[370]=(if ((self.scalar_static.f64_values[150])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[371]=(if ((self.scalar_static.f64_values[150])!=0.0){self.scalar_static.f64_values[331]}else{0.0});
        self.scalar_static.f64_values[372]=(if self.scalar_static.bool_values[30]{self.scalar_static.f64_values[0]}else{self.scalar_static.f64_values[370]});
        self.scalar_static.f64_values[373]=(if self.scalar_static.bool_values[30]{0.0}else{self.scalar_static.f64_values[371]});
        self.scalar_static.f64_values[374]=(if self.scalar_static.bool_values[30]{self.scalar_static.f64_values[331]}else{0.0});
        self.scalar_static.f64_values[375]=(self.scalar_static.f64_values[301]-1.0);
        self.scalar_static.f64_values[376]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[304]);
        self.scalar_static.f64_values[377]=(self.scalar_static.f64_values[332]/self.scalar_static.f64_values[304]);
        self.scalar_static.f64_values[378]=(self.scalar_static.f64_values[333]/self.scalar_static.f64_values[304]);
        self.scalar_static.f64_values[379]=(self.scalar_static.f64_values[331]/self.scalar_static.f64_values[304]);
        self.scalar_static.f64_values[380]=(self.scalar_static.f64_values[309]-1.0);
        self.scalar_static.f64_values[381]=(self.scalar_static.f64_values[0]*0.2);
        self.scalar_static.f64_values[382]=(0.2*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[383]=(self.scalar_static.f64_values[315]-1.0);
        self.scalar_static.f64_values[384]=(1.0/self.scalar_static.f64_values[14]);
        self.scalar_static.f64_values[385]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[386]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[387]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[329]);
        self.scalar_static.f64_values[388]=(self.scalar_static.f64_values[329]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[389]=(self.scalar_static.f64_values[330]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[390]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[391]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[392]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[333]);
        self.scalar_static.instance_dirty = false;
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
        self.scalar_static.f64_values[393]=(temperature+self.scalar_static.f64_values[10]);
        self.scalar_static.f64_values[394]=(self.scalar_static.f64_values[393]/self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[395]=f64::powf(self.scalar_static.f64_values[394],self.scalar_static.f64_values[197]);
        self.scalar_static.f64_values[396]=(self.scalar_static.f64_values[196]*self.scalar_static.f64_values[395]);
        self.scalar_static.f64_values[397]=(self.scalar_static.f64_values[393]/self.scalar_static.f64_values[396]);
        self.scalar_static.f64_values[398]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[397]);
        self.scalar_static.f64_values[399]=(self.scalar_static.f64_values[396]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[400]=(self.scalar_static.f64_values[393]/self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[401]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[400]);
        self.scalar_static.f64_values[402]=(1.0/self.scalar_static.f64_values[396]);
        self.scalar_static.f64_values[403]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[402]);
        self.scalar_static.f64_values[404]=(if self.scalar_static.bool_values[70]{self.scalar_static.f64_values[403]}else{0.0});
        self.scalar_static.f64_values[405]=(1.0/self.scalar_static.f64_values[393]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
