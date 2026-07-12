#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

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
    pub p128: f64, pub p129: f64, pub p130: f64, pub p131: f64, pub p132: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 133] = [
                0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                -100.0, 500.0, 1e-12, 1.0, 1e22, 27.0, -100.0, 500.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1e-16, 1.0, 1.0, 1.0, 0.0, 0.0,
                1.0, 1.0, 0.9, 0.0, 0.0, 0.75, 0.33, -0.5,
                0.0, 0.0, 0.75, 0.33, -0.5, 0.0, 0.1, 0.0,
                0.0, 0.0, 0.75, 0.33, -0.5, 0.0, 1e-18, 1.0,
                1.0, 0.0, 0.0, 2.0, 1e-16, 1.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.33,
                0.0, 1.0, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.12, 1.12, 1.12, 1.12, 1.12, 1.12, 1.12,
                1.12, 0.0, 3.0, 3.0, 3.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 133);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 136] = [
    ("trise", 0), ("dtemp", 0), ("dta", 0), ("sw_noise", 1), ("sw_et", 2), ("npn", 3), ("pnp", 4), ("type", 5), ("scale", 6), ("shrink", 7), ("tmin", 8), ("tmax", 9), ("gmin", 10), ("pnjmaxi", 11), ("maxexp", 12), ("tnom", 13),
    ("tref", 13), ("tminclip", 14), ("tmaxclip", 15), ("rcx", 16), ("rci", 17), ("vo", 18), ("gamm", 19), ("hrcf", 20), ("rbx", 21), ("rbi", 22), ("re", 23), ("rs", 24), ("rbp", 25), ("is", 26), ("isrr", 27), ("nf", 28),
    ("nr", 29), ("qbm", 30), ("isp", 31), ("wsp", 32), ("nfp", 33), ("fc", 34), ("cbeo", 35), ("cje", 36), ("pe", 37), ("me", 38), ("aje", 39), ("cbco", 40), ("cjc", 41), ("pc", 42), ("mc", 43), ("ajc", 44),
    ("vrt", 45), ("art", 46), ("qco", 47), ("cjep", 48), ("cjcp", 49), ("ps", 50), ("ms", 51), ("ajs", 52), ("ccso", 53), ("ibei", 54), ("wbe", 55), ("nei", 56), ("qnibeir", 57), ("iben", 58), ("nen", 59), ("ibci", 60),
    ("nci", 61), ("ibcn", 62), ("ncn", 63), ("ibeip", 64), ("ibenp", 65), ("ibcip", 66), ("ncip", 67), ("ibcnp", 68), ("ncnp", 69), ("vef", 70), ("ver", 71), ("ikf", 72), ("nkf", 73), ("ikr", 74), ("ikp", 75), ("tf", 76),
    ("qtf", 77), ("xtf", 78), ("vtf", 79), ("itf", 80), ("tr", 81), ("td", 82), ("avc1", 83), ("avc2", 84), ("avcx1", 85), ("avcx2", 86), ("mcx", 87), ("vbbe", 88), ("nbbe", 89), ("ibbe", 90), ("tvbbe1", 91), ("tvbbe2", 92),
    ("tnbbe", 93), ("vpte", 94), ("ibk0", 95), ("abk", 96), ("bbk", 97), ("kfn", 98), ("afn", 99), ("bfn", 100), ("rth", 101), ("cth", 102), ("xre", 103), ("xrb", 104), ("xrbi", 105), ("xrbx", 106), ("xrc", 107), ("xrci", 108),
    ("xrcx", 109), ("xrbp", 110), ("xrs", 111), ("xvo", 112), ("ea", 113), ("eaie", 114), ("eaic", 115), ("eais", 116), ("eane", 117), ("eanc", 118), ("eans", 119), ("eap", 120), ("dear", 121), ("xis", 122), ("xii", 123), ("xin", 124),
    ("xisr", 125), ("xikf", 126), ("tavc", 127), ("tavcx", 128), ("tnf", 129), ("tcvef", 130), ("tcver", 131), ("tcrth", 132),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 133] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, Some(56), None, None, None, Some(61),
    None, None, None, None, None, Some(67), None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 133] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 133] = [
    "trise", "sw_noise", "sw_et", "npn", "pnp", "type", "scale", "shrink", "tmin", "tmax", "gmin", "pnjmaxi", "maxexp", "tnom", "tminclip", "tmaxclip",
    "rcx", "rci", "vo", "gamm", "hrcf", "rbx", "rbi", "re", "rs", "rbp", "is", "isrr", "nf", "nr", "qbm", "isp",
    "wsp", "nfp", "fc", "cbeo", "cje", "pe", "me", "aje", "cbco", "cjc", "pc", "mc", "ajc", "vrt", "art", "qco",
    "cjep", "cjcp", "ps", "ms", "ajs", "ccso", "ibei", "wbe", "nei", "qnibeir", "iben", "nen", "ibci", "nci", "ibcn", "ncn",
    "ibeip", "ibenp", "ibcip", "ncip", "ibcnp", "ncnp", "vef", "ver", "ikf", "nkf", "ikr", "ikp", "tf", "qtf", "xtf", "vtf",
    "itf", "tr", "td", "avc1", "avc2", "avcx1", "avcx2", "mcx", "vbbe", "nbbe", "ibbe", "tvbbe1", "tvbbe2", "tnbbe", "vpte", "ibk0",
    "abk", "bbk", "kfn", "afn", "bfn", "rth", "cth", "xre", "xrb", "xrbi", "xrbx", "xrc", "xrci", "xrcx", "xrbp", "xrs",
    "xvo", "ea", "eaie", "eaic", "eais", "eane", "eanc", "eans", "eap", "dear", "xis", "xii", "xin", "xisr", "xikf", "tavc",
    "tavcx", "tnf", "tcvef", "tcver", "tcrth",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 133] = [
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
    &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 133] = [
    false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 133] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 133] = [
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 133] = [
    0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 2, 3, 3, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 0, 2,
    0, 3, 2, 2, 2, 3, 3, 0, 2, 2, 3, 3, 0, 2, 3, 2, 2, 2, 3, 3, 0, 2, 2, 0, 3, 0, 2, 3, 2, 3, 2, 3,
    2, 2, 2, 3, 2, 3, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 3, 0, 0, 0, 2, 2,
    3, 2, 2, 3, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 133] = [
    &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
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
    &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 14],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 133]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 11]>,
    pub(crate) ddt_state_previous: Box<[f64; 11]>,
    pub(crate) ddt_state_older: Box<[f64; 11]>,
    pub(crate) ddt_state_initialized: Box<[bool; 11]>,
    pub(crate) ddt_derivative_current: Box<[f64; 11]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 11]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 526]>,
    pub(crate) scalar_static_bool: Box<[bool; 120]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["dt", "cx", "ci", "bx", "bi", "ei", "bp", "si", "xf1", "xf2"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 133;
    pub const VARIABLE_COUNT: usize = 359;
    pub const DDT_STATE_COUNT: usize = 11;
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
            scalar_static_f64: boxed_zero_f64_array::<526>(),
            scalar_static_bool: boxed_zero_bool_array::<120>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic13_4t'", name));
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
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=p.p88;
        self.scalar_static_bool[0]=(self.scalar_static_f64[0]>0.0);
        self.scalar_static_f64[1]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[2]=if param_given[4]{1.0}else{0.0};
        self.scalar_static_f64[3]=if param_given[5]{1.0}else{0.0};
        self.scalar_static_f64[4]=if param_given[10]{1.0}else{0.0};
        self.scalar_static_f64[5]=if param_given[105]{1.0}else{0.0};
        self.scalar_static_f64[6]=p.p83;
        self.scalar_static_bool[1]=(self.scalar_static_f64[6]>0.0);
        self.scalar_static_f64[7]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_f64[8]=if param_given[108]{1.0}else{0.0};
        self.scalar_static_f64[9]=p.p30;
        self.scalar_static_bool[2]=(self.scalar_static_f64[9]<0.5);
        self.scalar_static_f64[10]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[11]=p.p90;
        self.scalar_static_bool[3]=(self.scalar_static_f64[11]>0.0);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_f64[13]=p.p31;
        self.scalar_static_bool[4]=(self.scalar_static_f64[13]>0.0);
        self.scalar_static_f64[14]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[15]=if param_given[109]{1.0}else{0.0};
        self.scalar_static_f64[16]=p.p55;
        self.scalar_static_bool[5]=(0.0==self.scalar_static_f64[16]);
        self.scalar_static_f64[17]=(if self.scalar_static_bool[5]{1.0}else{0.0});
        self.scalar_static_f64[18]=p.p64;
        self.scalar_static_bool[6]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_f64[19]=p.p65;
        self.scalar_static_bool[7]=(self.scalar_static_f64[19]>0.0);
        self.scalar_static_bool[8]=(self.scalar_static_bool[6]||self.scalar_static_bool[7]);
        self.scalar_static_f64[20]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[21]=p.p57;
        self.scalar_static_bool[9]=(self.scalar_static_f64[21]>0.0);
        self.scalar_static_f64[22]=(if self.scalar_static_bool[9]{1.0}else{0.0});
        self.scalar_static_f64[23]=if param_given[3]{1.0}else{0.0};
        self.scalar_static_f64[24]=if param_given[106]{1.0}else{0.0};
        self.scalar_static_bool[10]=(1.0==self.scalar_static_f64[16]);
        self.scalar_static_f64[25]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[26]=p.p45;
        self.scalar_static_bool[11]=(self.scalar_static_f64[26]>0.0);
        self.scalar_static_f64[27]=p.p46;
        self.scalar_static_bool[12]=(self.scalar_static_f64[27]>0.0);
        self.scalar_static_bool[13]=(self.scalar_static_bool[11]&&self.scalar_static_bool[12]);
        self.scalar_static_f64[28]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_f64[29]=p.p39;
        self.scalar_static_bool[14]=(self.scalar_static_f64[29]<=0.0);
        self.scalar_static_f64[30]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[31]=p.p44;
        self.scalar_static_bool[15]=(self.scalar_static_f64[31]<=0.0);
        self.scalar_static_f64[32]=(if self.scalar_static_bool[15]{1.0}else{0.0});
        self.scalar_static_f64[33]=if param_given[110]{1.0}else{0.0};
        self.scalar_static_f64[34]=if param_given[11]{1.0}else{0.0};
        self.scalar_static_f64[35]=p.p85;
        self.scalar_static_bool[16]=(self.scalar_static_f64[35]>0.0);
        self.scalar_static_f64[36]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_f64[37]=p.p66;
        self.scalar_static_bool[17]=(self.scalar_static_f64[37]>0.0);
        self.scalar_static_f64[38]=p.p68;
        self.scalar_static_bool[18]=(self.scalar_static_f64[38]>0.0);
        self.scalar_static_bool[19]=(self.scalar_static_bool[17]||self.scalar_static_bool[18]);
        self.scalar_static_f64[39]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_f64[40]=p.p94;
        self.scalar_static_bool[20]=(self.scalar_static_f64[40]>0.0);
        self.scalar_static_f64[41]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_f64[42]=p.p49;
        self.scalar_static_bool[21]=(self.scalar_static_f64[42]>0.0);
        self.scalar_static_f64[43]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_f64[44]=p.p52;
        self.scalar_static_bool[22]=(self.scalar_static_f64[44]<=0.0);
        self.scalar_static_f64[45]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[46]=p.p97;
        self.scalar_static_bool[23]=(self.scalar_static_f64[46]>0.0);
        self.scalar_static_f64[47]=p.p95;
        self.scalar_static_bool[24]=(self.scalar_static_f64[47]>0.0);
        self.scalar_static_bool[25]=(self.scalar_static_bool[23]&&self.scalar_static_bool[24]);
        self.scalar_static_f64[48]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_f64[49]=p.p10;
        self.scalar_static_f64[50]=(if ((self.scalar_static_f64[4])!=0.0){self.scalar_static_f64[49]}else{0.0});
        self.scalar_static_bool[26]=(!((self.scalar_static_f64[4])!=0.0));
        self.scalar_static_f64[51]=(if self.scalar_static_bool[26]{1e-12}else{self.scalar_static_f64[50]});
        self.scalar_static_f64[52]=p.p11;
        self.scalar_static_f64[53]=(if ((self.scalar_static_f64[34])!=0.0){self.scalar_static_f64[52]}else{0.0});
        self.scalar_static_bool[27]=(!((self.scalar_static_f64[34])!=0.0));
        self.scalar_static_f64[54]=(if self.scalar_static_bool[27]{1.0}else{self.scalar_static_f64[53]});
        self.scalar_static_f64[55]=p.p74;
        self.scalar_static_bool[28]=(self.scalar_static_f64[55]>0.0);
        self.scalar_static_bool[29]=(self.scalar_static_f64[54]>self.scalar_static_f64[55]);
        self.scalar_static_bool[30]=(self.scalar_static_bool[28]&&self.scalar_static_bool[29]);
        self.scalar_static_f64[56]=(if self.scalar_static_bool[30]{1.0}else{0.0});
        self.scalar_static_f64[57]=p.p72;
        self.scalar_static_bool[31]=(self.scalar_static_f64[57]>0.0);
        self.scalar_static_bool[32]=(self.scalar_static_f64[54]>self.scalar_static_f64[57]);
        self.scalar_static_bool[33]=(self.scalar_static_bool[31]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[58]=(if self.scalar_static_bool[33]{1.0}else{0.0});
        self.scalar_static_f64[59]=p.p75;
        self.scalar_static_bool[34]=(self.scalar_static_f64[59]>0.0);
        self.scalar_static_bool[35]=(self.scalar_static_f64[54]>self.scalar_static_f64[59]);
        self.scalar_static_bool[36]=(self.scalar_static_bool[34]&&self.scalar_static_bool[35]);
        self.scalar_static_f64[60]=(if self.scalar_static_bool[36]{1.0}else{0.0});
        self.scalar_static_f64[61]=(if ((self.scalar_static_f64[23])!=0.0){1.0}else{0.0});
        self.scalar_static_bool[37]=(!((self.scalar_static_f64[23])!=0.0));
        self.scalar_static_bool[38]=(((self.scalar_static_f64[2])!=0.0)&&self.scalar_static_bool[37]);
        self.scalar_static_f64[62]=(if self.scalar_static_bool[38]{-1.0}else{self.scalar_static_f64[61]});
        self.scalar_static_f64[63]=p.p5;
        self.scalar_static_bool[39]=(!((self.scalar_static_f64[2])!=0.0));
        self.scalar_static_bool[40]=(self.scalar_static_bool[37]&&self.scalar_static_bool[39]);
        self.scalar_static_bool[41]=(((self.scalar_static_f64[3])!=0.0)&&self.scalar_static_bool[40]);
        self.scalar_static_f64[64]=(if self.scalar_static_bool[41]{self.scalar_static_f64[63]}else{self.scalar_static_f64[62]});
        self.scalar_static_bool[42]=(!((self.scalar_static_f64[3])!=0.0));
        self.scalar_static_bool[43]=(self.scalar_static_bool[40]&&self.scalar_static_bool[42]);
        self.scalar_static_f64[65]=(if self.scalar_static_bool[43]{1.0}else{self.scalar_static_f64[64]});
        self.scalar_static_f64[66]=p.p12;
        self.scalar_static_f64[67]=(self.scalar_static_f64[66]).ln();
        self.scalar_static_f64[68]=(1.0/self.scalar_static_f64[55]);
        self.scalar_static_f64[69]=(if self.scalar_static_bool[28]{self.scalar_static_f64[68]}else{0.0});
        self.scalar_static_f64[70]=(1.0/self.scalar_static_f64[59]);
        self.scalar_static_f64[71]=(if self.scalar_static_bool[34]{self.scalar_static_f64[70]}else{0.0});
        self.scalar_static_f64[72]=p.p20;
        self.scalar_static_bool[44]=(self.scalar_static_f64[72]>0.0);
        self.scalar_static_f64[73]=(1.0/self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(if self.scalar_static_bool[44]{self.scalar_static_f64[73]}else{0.0});
        self.scalar_static_f64[75]=p.p79;
        self.scalar_static_bool[45]=(self.scalar_static_f64[75]>0.0);
        self.scalar_static_f64[76]=(1.0/self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=(if self.scalar_static_bool[45]{self.scalar_static_f64[76]}else{0.0});
        self.scalar_static_f64[78]=p.p80;
        self.scalar_static_bool[46]=(self.scalar_static_f64[78]>0.0);
        self.scalar_static_f64[79]=(1.0/self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(if self.scalar_static_bool[46]{self.scalar_static_f64[79]}else{0.0});
        self.scalar_static_f64[81]=(if self.scalar_static_bool[46]{0.0}else{1.0});
        self.scalar_static_f64[82]=p.p13;
        self.scalar_static_f64[83]=(273.15+self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=p.p0;
        self.scalar_static_f64[85]=p.p14;
        self.scalar_static_f64[86]=(1.0+self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=p.p15;
        self.scalar_static_f64[88]=(self.scalar_static_f64[87]-1.0);
        self.scalar_static_f64[89]=p.p26;
        self.scalar_static_f64[90]=p.p89;
        self.scalar_static_f64[91]=(-self.scalar_static_f64[0]);
        self.scalar_static_f64[92]=(self.scalar_static_f64[54]/self.scalar_static_f64[11]);
        self.scalar_static_bool[47]=(!((self.scalar_static_f64[12])!=0.0));
        self.scalar_static_f64[93]=p.p122;
        self.scalar_static_f64[94]=p.p28;
        self.scalar_static_f64[95]=(self.scalar_static_f64[93]/self.scalar_static_f64[94]);
        self.scalar_static_f64[96]=p.p113;
        self.scalar_static_f64[97]=(-self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=(0.5*self.scalar_static_f64[54]);
        self.scalar_static_f64[99]=(4.0/self.scalar_static_f64[57]);
        self.scalar_static_f64[100]=p.p73;
        self.scalar_static_f64[101]=f64::powf(self.scalar_static_f64[99],self.scalar_static_f64[100]);
        self.scalar_static_f64[102]=(self.scalar_static_f64[98]*self.scalar_static_f64[101]);
        self.scalar_static_f64[103]=(1.0-self.scalar_static_f64[100]);
        self.scalar_static_f64[104]=(1.0/self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=f64::powf(self.scalar_static_f64[102],self.scalar_static_f64[104]);
        self.scalar_static_bool[48]=(!((self.scalar_static_f64[58])!=0.0));
        self.scalar_static_f64[106]=p.p27;
        self.scalar_static_f64[107]=p.p125;
        self.scalar_static_f64[108]=p.p29;
        self.scalar_static_f64[109]=(self.scalar_static_f64[107]/self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=p.p121;
        self.scalar_static_f64[111]=(-self.scalar_static_f64[110]);
        self.scalar_static_f64[112]=(4.0/self.scalar_static_f64[55]);
        self.scalar_static_f64[113]=f64::powf(self.scalar_static_f64[112],self.scalar_static_f64[100]);
        self.scalar_static_f64[114]=(self.scalar_static_f64[98]*self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=f64::powf(self.scalar_static_f64[114],self.scalar_static_f64[104]);
        self.scalar_static_bool[49]=(!((self.scalar_static_f64[56])!=0.0));
        self.scalar_static_f64[116]=p.p33;
        self.scalar_static_f64[117]=(self.scalar_static_f64[93]/self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=p.p120;
        self.scalar_static_f64[119]=(-self.scalar_static_f64[118]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[54]*self.scalar_static_f64[54]);
        self.scalar_static_f64[121]=(self.scalar_static_f64[71]*self.scalar_static_f64[120]);
        self.scalar_static_bool[50]=(!((self.scalar_static_f64[60])!=0.0));
        self.scalar_static_f64[122]=p.p54;
        self.scalar_static_f64[123]=p.p123;
        self.scalar_static_f64[124]=p.p56;
        self.scalar_static_f64[125]=(self.scalar_static_f64[123]/self.scalar_static_f64[124]);
        self.scalar_static_f64[126]=p.p114;
        self.scalar_static_f64[127]=(-self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=p.p58;
        self.scalar_static_f64[129]=p.p124;
        self.scalar_static_f64[130]=p.p59;
        self.scalar_static_f64[131]=(self.scalar_static_f64[129]/self.scalar_static_f64[130]);
        self.scalar_static_f64[132]=p.p117;
        self.scalar_static_f64[133]=(-self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=p.p60;
        self.scalar_static_f64[135]=p.p61;
        self.scalar_static_f64[136]=(self.scalar_static_f64[123]/self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=p.p115;
        self.scalar_static_f64[138]=(-self.scalar_static_f64[137]);
        self.scalar_static_f64[139]=p.p62;
        self.scalar_static_f64[140]=p.p63;
        self.scalar_static_f64[141]=(self.scalar_static_f64[129]/self.scalar_static_f64[140]);
        self.scalar_static_f64[142]=p.p118;
        self.scalar_static_f64[143]=(-self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=p.p67;
        self.scalar_static_f64[145]=(self.scalar_static_f64[123]/self.scalar_static_f64[144]);
        self.scalar_static_f64[146]=p.p116;
        self.scalar_static_f64[147]=(-self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=p.p69;
        self.scalar_static_f64[149]=(self.scalar_static_f64[129]/self.scalar_static_f64[148]);
        self.scalar_static_f64[150]=p.p119;
        self.scalar_static_f64[151]=(-self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=p.p126;
        self.scalar_static_f64[153]=p.p16;
        self.scalar_static_f64[154]=p.p109;
        self.scalar_static_bool[51]=(!((self.scalar_static_f64[15])!=0.0));
        self.scalar_static_f64[155]=p.p107;
        self.scalar_static_f64[156]=p.p17;
        self.scalar_static_f64[157]=p.p108;
        self.scalar_static_bool[52]=(!((self.scalar_static_f64[8])!=0.0));
        self.scalar_static_f64[158]=p.p21;
        self.scalar_static_f64[159]=p.p106;
        self.scalar_static_bool[53]=(!((self.scalar_static_f64[24])!=0.0));
        self.scalar_static_f64[160]=p.p104;
        self.scalar_static_f64[161]=p.p22;
        self.scalar_static_f64[162]=p.p105;
        self.scalar_static_bool[54]=(!((self.scalar_static_f64[5])!=0.0));
        self.scalar_static_f64[163]=p.p23;
        self.scalar_static_f64[164]=p.p103;
        self.scalar_static_f64[165]=p.p24;
        self.scalar_static_f64[166]=p.p111;
        self.scalar_static_f64[167]=p.p25;
        self.scalar_static_f64[168]=p.p110;
        self.scalar_static_bool[55]=(!((self.scalar_static_f64[33])!=0.0));
        self.scalar_static_f64[169]=p.p101;
        self.scalar_static_f64[170]=p.p132;
        self.scalar_static_f64[171]=p.p129;
        self.scalar_static_f64[172]=p.p84;
        self.scalar_static_f64[173]=p.p127;
        self.scalar_static_f64[174]=p.p86;
        self.scalar_static_f64[175]=p.p128;
        self.scalar_static_f64[176]=p.p91;
        self.scalar_static_f64[177]=p.p92;
        self.scalar_static_f64[178]=p.p93;
        self.scalar_static_f64[179]=p.p37;
        self.scalar_static_f64[180]=(0.5*self.scalar_static_f64[179]);
        self.scalar_static_f64[181]=(self.scalar_static_f64[179]* -0.5);
        self.scalar_static_f64[182]=p.p42;
        self.scalar_static_f64[183]=(0.5*self.scalar_static_f64[182]);
        self.scalar_static_f64[184]=(-0.5*self.scalar_static_f64[182]);
        self.scalar_static_f64[185]=p.p50;
        self.scalar_static_f64[186]=(0.5*self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=(-0.5*self.scalar_static_f64[185]);
        self.scalar_static_f64[188]=p.p36;
        self.scalar_static_f64[189]=p.p38;
        self.scalar_static_f64[190]=p.p41;
        self.scalar_static_f64[191]=p.p43;
        self.scalar_static_f64[192]=p.p48;
        self.scalar_static_f64[193]=p.p51;
        self.scalar_static_f64[194]=p.p19;
        self.scalar_static_f64[195]=p.p18;
        self.scalar_static_f64[196]=p.p112;
        self.scalar_static_f64[197]=p.p70;
        self.scalar_static_f64[198]=p.p130;
        self.scalar_static_f64[199]=p.p71;
        self.scalar_static_f64[200]=p.p131;
        self.scalar_static_f64[201]=p.p34;
        self.scalar_static_f64[202]=(1.0-self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=(-self.scalar_static_f64[189]);
        self.scalar_static_f64[204]=f64::powf(self.scalar_static_f64[202],self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(1.0-self.scalar_static_f64[189]);
        self.scalar_static_f64[206]=(0.5*self.scalar_static_f64[189]);
        self.scalar_static_bool[56]=(!((self.scalar_static_f64[30])!=0.0));
        self.scalar_static_f64[207]=(self.scalar_static_f64[29]*4.0);
        self.scalar_static_f64[208]=(self.scalar_static_f64[29]*self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=(-1.0-self.scalar_static_f64[191]);
        self.scalar_static_f64[210]=f64::powf(self.scalar_static_f64[202],self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(1.0-self.scalar_static_f64[191]);
        self.scalar_static_f64[212]=(0.5*self.scalar_static_f64[191]);
        self.scalar_static_f64[213]=(-self.scalar_static_f64[26]);
        self.scalar_static_bool[57]=(!((self.scalar_static_f64[32])!=0.0));
        self.scalar_static_bool[58]=(((self.scalar_static_f64[28])!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_f64[214]=(self.scalar_static_f64[31]*4.0);
        self.scalar_static_f64[215]=(self.scalar_static_f64[31]*self.scalar_static_f64[214]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[27]*4.0);
        self.scalar_static_f64[217]=(self.scalar_static_f64[27]*self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=(-self.scalar_static_f64[191]);
        self.scalar_static_bool[59]=(!((self.scalar_static_f64[28])!=0.0));
        self.scalar_static_bool[60]=(self.scalar_static_bool[57]&&self.scalar_static_bool[59]);
        self.scalar_static_f64[219]=f64::powf(self.scalar_static_f64[202],self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=(1.0/self.scalar_static_f64[100]);
        self.scalar_static_f64[221]=f64::powf(1e-8,self.scalar_static_f64[100]);
        self.scalar_static_bool[61]=(!((self.scalar_static_f64[10])!=0.0));
        self.scalar_static_f64[222]=(1.0+self.scalar_static_f64[221]);
        self.scalar_static_f64[223]=p.p32;
        self.scalar_static_f64[224]=(1.0-self.scalar_static_f64[223]);
        self.scalar_static_bool[62]=(!((self.scalar_static_f64[14])!=0.0));
        self.scalar_static_bool[63]=(((self.scalar_static_f64[22])!=0.0)&&((self.scalar_static_f64[25])!=0.0));
        self.scalar_static_bool[64]=(!((self.scalar_static_f64[22])!=0.0));
        self.scalar_static_bool[65]=(((self.scalar_static_f64[25])!=0.0)&&self.scalar_static_bool[64]);
        self.scalar_static_bool[66]=(((self.scalar_static_f64[1])!=0.0)&&((self.scalar_static_f64[25])!=0.0));
        self.scalar_static_bool[67]=(!((self.scalar_static_f64[25])!=0.0));
        self.scalar_static_bool[68]=(((self.scalar_static_f64[17])!=0.0)&&self.scalar_static_bool[67]);
        self.scalar_static_bool[69]=(((self.scalar_static_f64[1])!=0.0)&&self.scalar_static_bool[68]);
        self.scalar_static_bool[70]=(!((self.scalar_static_f64[17])!=0.0));
        self.scalar_static_bool[71]=(self.scalar_static_bool[67]&&self.scalar_static_bool[70]);
        self.scalar_static_bool[72]=(((self.scalar_static_f64[22])!=0.0)&&self.scalar_static_bool[71]);
        self.scalar_static_bool[73]=(self.scalar_static_bool[64]&&self.scalar_static_bool[71]);
        self.scalar_static_bool[74]=(((self.scalar_static_f64[1])!=0.0)&&self.scalar_static_bool[71]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[11]*self.scalar_static_f64[16]);
        self.scalar_static_f64[226]=(1.0-self.scalar_static_f64[16]);
        self.scalar_static_f64[227]=(self.scalar_static_f64[11]*self.scalar_static_f64[226]);
        self.scalar_static_bool[75]=(!((self.scalar_static_f64[20])!=0.0));
        self.scalar_static_f64[228]=(self.scalar_static_f64[67]).exp();
        self.scalar_static_f64[229]=(1.01-self.scalar_static_f64[191]);
        self.scalar_static_f64[230]=(1.0/self.scalar_static_f64[229]);
        self.scalar_static_f64[231]=(self.scalar_static_f64[191]-1.0);
        self.scalar_static_bool[76]=(!((self.scalar_static_f64[7])!=0.0));
        self.scalar_static_f64[232]=p.p87;
        self.scalar_static_f64[233]=(1.01-self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(1.0/self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=(self.scalar_static_f64[232]-1.0);
        self.scalar_static_bool[77]=(!((self.scalar_static_f64[36])!=0.0));
        self.scalar_static_bool[78]=(((self.scalar_static_f64[41])!=0.0)&&((self.scalar_static_f64[48])!=0.0));
        self.scalar_static_bool[79]=(!((self.scalar_static_f64[41])!=0.0));
        self.scalar_static_bool[80]=(((self.scalar_static_f64[48])!=0.0)&&self.scalar_static_bool[79]);
        self.scalar_static_f64[236]=p.p96;
        self.scalar_static_bool[81]=(!((self.scalar_static_f64[48])!=0.0));
        self.scalar_static_bool[82]=(!((self.scalar_static_f64[39])!=0.0));
        self.scalar_static_f64[237]=p.p2;
        self.scalar_static_f64[238]=(-self.scalar_static_f64[237]);
        self.scalar_static_bool[83]=(((self.scalar_static_f64[43])!=0.0)&&((self.scalar_static_f64[45])!=0.0));
        self.scalar_static_f64[239]=(-self.scalar_static_f64[193]);
        self.scalar_static_f64[240]=f64::powf(self.scalar_static_f64[202],self.scalar_static_f64[239]);
        self.scalar_static_f64[241]=(1.0-self.scalar_static_f64[193]);
        self.scalar_static_f64[242]=(0.5*self.scalar_static_f64[193]);
        self.scalar_static_bool[84]=(!((self.scalar_static_f64[45])!=0.0));
        self.scalar_static_bool[85]=(((self.scalar_static_f64[43])!=0.0)&&self.scalar_static_bool[84]);
        self.scalar_static_f64[243]=(self.scalar_static_f64[44]*4.0);
        self.scalar_static_f64[244]=(self.scalar_static_f64[44]*self.scalar_static_f64[243]);
        self.scalar_static_bool[86]=(!((self.scalar_static_f64[43])!=0.0));
        self.scalar_static_f64[245]=p.p76;
        self.scalar_static_f64[246]=p.p77;
        self.scalar_static_f64[247]=p.p78;
        self.scalar_static_f64[248]=p.p81;
        self.scalar_static_f64[249]=p.p47;
        self.scalar_static_f64[250]=p.p53;
        self.scalar_static_f64[251]=p.p35;
        self.scalar_static_f64[252]=p.p40;
        self.scalar_static_f64[253]=p.p102;
        self.scalar_static_f64[254]=p.p82;
        self.scalar_static_f64[255]=(self.scalar_static_f64[152]-1.0);
        self.scalar_static_f64[256]=(self.scalar_static_f64[154]-1.0);
        self.scalar_static_f64[257]=(self.scalar_static_f64[155]-1.0);
        self.scalar_static_f64[258]=(self.scalar_static_f64[157]-1.0);
        self.scalar_static_f64[259]=(self.scalar_static_f64[159]-1.0);
        self.scalar_static_f64[260]=(self.scalar_static_f64[160]-1.0);
        self.scalar_static_f64[261]=(self.scalar_static_f64[162]-1.0);
        self.scalar_static_f64[262]=(self.scalar_static_f64[164]-1.0);
        self.scalar_static_f64[263]=(self.scalar_static_f64[166]-1.0);
        self.scalar_static_f64[264]=(self.scalar_static_f64[168]-1.0);
        self.scalar_static_f64[265]=(self.scalar_static_f64[95]-1.0);
        self.scalar_static_f64[266]=(self.scalar_static_f64[109]-1.0);
        self.scalar_static_f64[267]=(self.scalar_static_f64[117]-1.0);
        self.scalar_static_f64[268]=(self.scalar_static_f64[125]-1.0);
        self.scalar_static_f64[269]=(self.scalar_static_f64[131]-1.0);
        self.scalar_static_f64[270]=(self.scalar_static_f64[136]-1.0);
        self.scalar_static_f64[271]=(self.scalar_static_f64[141]-1.0);
        self.scalar_static_f64[272]=(self.scalar_static_f64[145]-1.0);
        self.scalar_static_f64[273]=(self.scalar_static_f64[149]-1.0);
        self.scalar_static_f64[274]=(self.scalar_static_f64[189]-1.0);
        self.scalar_static_f64[275]=(self.scalar_static_f64[193]-1.0);
        self.scalar_static_f64[276]=(self.scalar_static_f64[93]-1.0);
        self.scalar_static_f64[277]=(self.scalar_static_f64[196]-1.0);
        self.scalar_static_f64[278]=(-self.scalar_static_f64[65]);
        self.scalar_static_f64[279]=(if ((self.scalar_static_f64[30])!=0.0){self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[280]=(if ((self.scalar_static_f64[30])!=0.0){self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[281]=(self.scalar_static_f64[206]*self.scalar_static_f64[279]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[206]*self.scalar_static_f64[280]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[205]-1.0);
        self.scalar_static_f64[284]=(if self.scalar_static_bool[56]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[285]=(if self.scalar_static_bool[56]{self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[286]=(if ((self.scalar_static_f64[32])!=0.0){self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[287]=(if ((self.scalar_static_f64[32])!=0.0){self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[288]=(self.scalar_static_f64[212]*self.scalar_static_f64[286]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[212]*self.scalar_static_f64[287]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[211]-1.0);
        self.scalar_static_f64[291]=(self.scalar_static_f64[211]*self.scalar_static_f64[278]);
        self.scalar_static_f64[292]=(self.scalar_static_f64[65]*self.scalar_static_f64[211]);
        self.scalar_static_f64[293]=(2.0*self.scalar_static_f64[278]);
        self.scalar_static_f64[294]=(self.scalar_static_f64[65]*2.0);
        self.scalar_static_f64[295]=(self.scalar_static_f64[218]-1.0);
        self.scalar_static_f64[296]=(if self.scalar_static_bool[60]{self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[297]=(if self.scalar_static_bool[60]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[298]=(self.scalar_static_f64[220]-1.0);
        self.scalar_static_f64[299]=(self.scalar_static_f64[100]-1.0);
        self.scalar_static_f64[300]=(if self.scalar_static_bool[66]{self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[301]=(if self.scalar_static_bool[66]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[302]=(if self.scalar_static_bool[69]{self.scalar_static_f64[278]}else{self.scalar_static_f64[300]});
        self.scalar_static_f64[303]=(if self.scalar_static_bool[69]{self.scalar_static_f64[65]}else{self.scalar_static_f64[301]});
        self.scalar_static_f64[304]=(if self.scalar_static_bool[74]{self.scalar_static_f64[278]}else{self.scalar_static_f64[302]});
        self.scalar_static_f64[305]=(if self.scalar_static_bool[74]{self.scalar_static_f64[65]}else{self.scalar_static_f64[303]});
        self.scalar_static_f64[306]=(if self.scalar_static_bool[74]{self.scalar_static_f64[278]}else{self.scalar_static_f64[304]});
        self.scalar_static_f64[307]=(if self.scalar_static_bool[74]{self.scalar_static_f64[65]}else{self.scalar_static_f64[305]});
        self.scalar_static_f64[308]=(self.scalar_static_f64[230]-1.0);
        self.scalar_static_f64[309]=(self.scalar_static_f64[231]-1.0);
        self.scalar_static_f64[310]=(self.scalar_static_f64[234]-1.0);
        self.scalar_static_f64[311]=(self.scalar_static_f64[235]-1.0);
        self.scalar_static_f64[312]=(self.scalar_static_f64[278]/self.scalar_static_f64[40]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[65]/self.scalar_static_f64[40]);
        self.scalar_static_f64[314]=(-self.scalar_static_f64[312]);
        self.scalar_static_f64[315]=(-self.scalar_static_f64[313]);
        self.scalar_static_f64[316]=(if self.scalar_static_bool[78]{self.scalar_static_f64[314]}else{0.0});
        self.scalar_static_f64[317]=(if self.scalar_static_bool[78]{self.scalar_static_f64[315]}else{0.0});
        self.scalar_static_f64[318]=(self.scalar_static_f64[236]-1.0);
        self.scalar_static_f64[319]=(self.scalar_static_f64[51]*self.scalar_static_f64[65]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[51]*self.scalar_static_f64[278]);
        self.scalar_static_f64[321]=(if self.scalar_static_bool[83]{self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[322]=(if self.scalar_static_bool[83]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[323]=(self.scalar_static_f64[242]*self.scalar_static_f64[321]);
        self.scalar_static_f64[324]=(self.scalar_static_f64[242]*self.scalar_static_f64[322]);
        self.scalar_static_f64[325]=(self.scalar_static_f64[241]-1.0);
        self.scalar_static_f64[326]=(if self.scalar_static_bool[85]{self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[327]=(if self.scalar_static_bool[85]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_f64[328]=(self.scalar_static_f64[77]*self.scalar_static_f64[278]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[65]*self.scalar_static_f64[77]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[328]/1.44);
        self.scalar_static_f64[331]=(self.scalar_static_f64[329]/1.44);
        self.scalar_static_f64[332]=(self.scalar_static_f64[228]*self.scalar_static_f64[330]);
        self.scalar_static_f64[333]=(self.scalar_static_f64[228]*self.scalar_static_f64[331]);
        self.scalar_static_f64[334]=(self.scalar_static_f64[250]*self.scalar_static_f64[278]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[65]*self.scalar_static_f64[250]);
        self.scalar_static_f64[336]=(-self.scalar_static_f64[251]);
        self.scalar_static_f64[337]=(-self.scalar_static_f64[252]);
        self.scalar_static_f64[338]=(self.scalar_static_f64[254]*0.3333333333333333);
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
        self.scalar_static_f64[339]=(temperature+self.scalar_static_f64[84]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[339]-273.15);
        self.scalar_static_bool[87]=(self.scalar_static_f64[340]<self.scalar_static_f64[86]);
        self.scalar_static_f64[341]=(if self.scalar_static_bool[87]{1.0}else{0.0});
        self.scalar_static_f64[342]=(self.scalar_static_f64[340]-self.scalar_static_f64[85]);
        self.scalar_static_f64[343]=(self.scalar_static_f64[342]-1.0);
        self.scalar_static_f64[344]=(self.scalar_static_f64[343]).exp();
        self.scalar_static_f64[345]=(self.scalar_static_f64[85]+self.scalar_static_f64[344]);
        self.scalar_static_f64[346]=(if ((self.scalar_static_f64[341])!=0.0){self.scalar_static_f64[345]}else{self.scalar_static_f64[340]});
        self.scalar_static_bool[88]=(self.scalar_static_f64[346]>self.scalar_static_f64[88]);
        self.scalar_static_f64[347]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_bool[89]=(!((self.scalar_static_f64[341])!=0.0));
        self.scalar_static_bool[90]=(((self.scalar_static_f64[347])!=0.0)&&self.scalar_static_bool[89]);
        self.scalar_static_f64[348]=(self.scalar_static_f64[87]-self.scalar_static_f64[346]);
        self.scalar_static_f64[349]=(self.scalar_static_f64[348]-1.0);
        self.scalar_static_f64[350]=(self.scalar_static_f64[349]).exp();
        self.scalar_static_f64[351]=(self.scalar_static_f64[87]-self.scalar_static_f64[350]);
        self.scalar_static_f64[352]=(if self.scalar_static_bool[90]{self.scalar_static_f64[351]}else{self.scalar_static_f64[346]});
        self.scalar_static_f64[353]=(273.15+self.scalar_static_f64[352]);
        self.scalar_static_f64[354]=(self.scalar_static_f64[353]*1.380662e-23);
        self.scalar_static_f64[355]=(self.scalar_static_f64[354]/1.602189e-19);
        self.scalar_static_f64[356]=(self.scalar_static_f64[353]/self.scalar_static_f64[83]);
        self.scalar_static_f64[357]=(self.scalar_static_f64[355]*self.scalar_static_f64[90]);
        self.scalar_static_f64[358]=(self.scalar_static_f64[91]/self.scalar_static_f64[357]);
        self.scalar_static_f64[359]=(self.scalar_static_f64[358]).exp();
        self.scalar_static_f64[360]=(self.scalar_static_f64[359]+self.scalar_static_f64[92]);
        self.scalar_static_f64[361]=(self.scalar_static_f64[360]).ln();
        self.scalar_static_f64[362]=(self.scalar_static_f64[357]*self.scalar_static_f64[361]);
        self.scalar_static_f64[363]=(if ((self.scalar_static_f64[12])!=0.0){self.scalar_static_f64[362]}else{0.0});
        self.scalar_static_f64[364]=(if self.scalar_static_bool[47]{0.0}else{self.scalar_static_f64[363]});
        self.scalar_static_f64[365]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[95]);
        self.scalar_static_f64[366]=(self.scalar_static_f64[89]*self.scalar_static_f64[365]);
        self.scalar_static_f64[367]=(1.0-self.scalar_static_f64[356]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[97]*self.scalar_static_f64[367]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[355]*self.scalar_static_f64[94]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[368]/self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[370]).exp();
        self.scalar_static_f64[372]=(self.scalar_static_f64[366]*self.scalar_static_f64[371]);
        self.scalar_static_bool[91]=(self.scalar_static_f64[372]>0.0);
        self.scalar_static_f64[373]=(if self.scalar_static_bool[91]{1.0}else{0.0});
        self.scalar_static_bool[92]=(((self.scalar_static_f64[58])!=0.0)&&((self.scalar_static_f64[373])!=0.0));
        self.scalar_static_f64[374]=(self.scalar_static_f64[105]/self.scalar_static_f64[372]);
        self.scalar_static_f64[375]=(1.0+self.scalar_static_f64[374]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[375]).ln();
        self.scalar_static_f64[377]=(self.scalar_static_f64[369]*self.scalar_static_f64[376]);
        self.scalar_static_f64[378]=(if self.scalar_static_bool[92]{self.scalar_static_f64[377]}else{0.0});
        self.scalar_static_bool[93]=(((self.scalar_static_f64[373])!=0.0)&&self.scalar_static_bool[48]);
        self.scalar_static_f64[379]=(self.scalar_static_f64[54]/self.scalar_static_f64[372]);
        self.scalar_static_f64[380]=(1.0+self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=(self.scalar_static_f64[380]).ln();
        self.scalar_static_f64[382]=(self.scalar_static_f64[369]*self.scalar_static_f64[381]);
        self.scalar_static_f64[383]=(if self.scalar_static_bool[93]{self.scalar_static_f64[382]}else{self.scalar_static_f64[378]});
        self.scalar_static_bool[94]=(!((self.scalar_static_f64[373])!=0.0));
        self.scalar_static_f64[384]=(if self.scalar_static_bool[94]{0.0}else{self.scalar_static_f64[383]});
        self.scalar_static_f64[385]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[109]);
        self.scalar_static_f64[386]=(self.scalar_static_f64[106]*self.scalar_static_f64[385]);
        self.scalar_static_f64[387]=(self.scalar_static_f64[367]*self.scalar_static_f64[111]);
        self.scalar_static_f64[388]=(self.scalar_static_f64[355]*self.scalar_static_f64[108]);
        self.scalar_static_f64[389]=(self.scalar_static_f64[387]/self.scalar_static_f64[388]);
        self.scalar_static_f64[390]=(self.scalar_static_f64[389]).exp();
        self.scalar_static_f64[391]=(self.scalar_static_f64[386]*self.scalar_static_f64[390]);
        self.scalar_static_bool[95]=(self.scalar_static_f64[391]>0.0);
        self.scalar_static_bool[96]=(self.scalar_static_bool[91]&&self.scalar_static_bool[95]);
        self.scalar_static_f64[392]=(if self.scalar_static_bool[96]{1.0}else{0.0});
        self.scalar_static_bool[97]=(((self.scalar_static_f64[56])!=0.0)&&((self.scalar_static_f64[392])!=0.0));
        self.scalar_static_f64[393]=(self.scalar_static_f64[372]*self.scalar_static_f64[391]);
        self.scalar_static_f64[394]=(self.scalar_static_f64[115]/self.scalar_static_f64[393]);
        self.scalar_static_f64[395]=(1.0+self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=(self.scalar_static_f64[395]).ln();
        self.scalar_static_f64[397]=(self.scalar_static_f64[388]*self.scalar_static_f64[396]);
        self.scalar_static_f64[398]=(if self.scalar_static_bool[97]{self.scalar_static_f64[397]}else{0.0});
        self.scalar_static_bool[98]=(((self.scalar_static_f64[392])!=0.0)&&self.scalar_static_bool[49]);
        self.scalar_static_f64[399]=(self.scalar_static_f64[54]/self.scalar_static_f64[393]);
        self.scalar_static_f64[400]=(1.0+self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=(self.scalar_static_f64[400]).ln();
        self.scalar_static_f64[402]=(self.scalar_static_f64[388]*self.scalar_static_f64[401]);
        self.scalar_static_f64[403]=(if self.scalar_static_bool[98]{self.scalar_static_f64[402]}else{self.scalar_static_f64[398]});
        self.scalar_static_bool[99]=(!((self.scalar_static_f64[392])!=0.0));
        self.scalar_static_f64[404]=(if self.scalar_static_bool[99]{0.0}else{self.scalar_static_f64[403]});
        self.scalar_static_f64[405]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[117]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[13]*self.scalar_static_f64[405]);
        self.scalar_static_f64[407]=(self.scalar_static_f64[367]*self.scalar_static_f64[119]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[355]*self.scalar_static_f64[116]);
        self.scalar_static_f64[409]=(self.scalar_static_f64[407]/self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[409]).exp();
        self.scalar_static_f64[411]=(self.scalar_static_f64[406]*self.scalar_static_f64[410]);
        self.scalar_static_bool[100]=(self.scalar_static_f64[411]>0.0);
        self.scalar_static_f64[412]=(if self.scalar_static_bool[100]{1.0}else{0.0});
        self.scalar_static_bool[101]=(((self.scalar_static_f64[60])!=0.0)&&((self.scalar_static_f64[412])!=0.0));
        self.scalar_static_f64[413]=(self.scalar_static_f64[121]/self.scalar_static_f64[411]);
        self.scalar_static_f64[414]=(1.0+self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[414]).ln();
        self.scalar_static_f64[416]=(self.scalar_static_f64[408]*self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=(if self.scalar_static_bool[101]{self.scalar_static_f64[416]}else{0.0});
        self.scalar_static_bool[102]=(((self.scalar_static_f64[412])!=0.0)&&self.scalar_static_bool[50]);
        self.scalar_static_f64[418]=(self.scalar_static_f64[54]/self.scalar_static_f64[411]);
        self.scalar_static_f64[419]=(1.0+self.scalar_static_f64[418]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[419]).ln();
        self.scalar_static_f64[421]=(self.scalar_static_f64[408]*self.scalar_static_f64[420]);
        self.scalar_static_f64[422]=(if self.scalar_static_bool[102]{self.scalar_static_f64[421]}else{self.scalar_static_f64[417]});
        self.scalar_static_bool[103]=(!((self.scalar_static_f64[412])!=0.0));
        self.scalar_static_f64[423]=(if self.scalar_static_bool[103]{0.0}else{self.scalar_static_f64[422]});
        self.scalar_static_f64[424]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[125]);
        self.scalar_static_f64[425]=(self.scalar_static_f64[122]*self.scalar_static_f64[424]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[367]*self.scalar_static_f64[127]);
        self.scalar_static_f64[427]=(self.scalar_static_f64[355]*self.scalar_static_f64[124]);
        self.scalar_static_f64[428]=(self.scalar_static_f64[426]/self.scalar_static_f64[427]);
        self.scalar_static_f64[429]=(self.scalar_static_f64[428]).exp();
        self.scalar_static_f64[430]=(self.scalar_static_f64[425]*self.scalar_static_f64[429]);
        self.scalar_static_bool[104]=(self.scalar_static_f64[430]>0.0);
        self.scalar_static_f64[431]=(if self.scalar_static_bool[104]{1.0}else{0.0});
        self.scalar_static_f64[432]=(self.scalar_static_f64[54]/self.scalar_static_f64[430]);
        self.scalar_static_f64[433]=(1.0+self.scalar_static_f64[432]);
        self.scalar_static_f64[434]=(self.scalar_static_f64[433]).ln();
        self.scalar_static_f64[435]=(self.scalar_static_f64[427]*self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=(if ((self.scalar_static_f64[431])!=0.0){self.scalar_static_f64[435]}else{0.0});
        self.scalar_static_bool[105]=(!((self.scalar_static_f64[431])!=0.0));
        self.scalar_static_f64[437]=(if self.scalar_static_bool[105]{0.0}else{self.scalar_static_f64[436]});
        self.scalar_static_f64[438]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[131]);
        self.scalar_static_f64[439]=(self.scalar_static_f64[128]*self.scalar_static_f64[438]);
        self.scalar_static_f64[440]=(self.scalar_static_f64[367]*self.scalar_static_f64[133]);
        self.scalar_static_f64[441]=(self.scalar_static_f64[355]*self.scalar_static_f64[130]);
        self.scalar_static_f64[442]=(self.scalar_static_f64[440]/self.scalar_static_f64[441]);
        self.scalar_static_f64[443]=(self.scalar_static_f64[442]).exp();
        self.scalar_static_f64[444]=(self.scalar_static_f64[439]*self.scalar_static_f64[443]);
        self.scalar_static_bool[106]=(self.scalar_static_f64[444]>0.0);
        self.scalar_static_f64[445]=(if self.scalar_static_bool[106]{1.0}else{0.0});
        self.scalar_static_f64[446]=(self.scalar_static_f64[54]/self.scalar_static_f64[444]);
        self.scalar_static_f64[447]=(1.0+self.scalar_static_f64[446]);
        self.scalar_static_f64[448]=(self.scalar_static_f64[447]).ln();
        self.scalar_static_f64[449]=(self.scalar_static_f64[441]*self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=(if ((self.scalar_static_f64[445])!=0.0){self.scalar_static_f64[449]}else{0.0});
        self.scalar_static_bool[107]=(!((self.scalar_static_f64[445])!=0.0));
        self.scalar_static_f64[451]=(if self.scalar_static_bool[107]{0.0}else{self.scalar_static_f64[450]});
        self.scalar_static_f64[452]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[136]);
        self.scalar_static_f64[453]=(self.scalar_static_f64[134]*self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(self.scalar_static_f64[367]*self.scalar_static_f64[138]);
        self.scalar_static_f64[455]=(self.scalar_static_f64[355]*self.scalar_static_f64[135]);
        self.scalar_static_f64[456]=(self.scalar_static_f64[454]/self.scalar_static_f64[455]);
        self.scalar_static_f64[457]=(self.scalar_static_f64[456]).exp();
        self.scalar_static_f64[458]=(self.scalar_static_f64[453]*self.scalar_static_f64[457]);
        self.scalar_static_bool[108]=(self.scalar_static_f64[458]>0.0);
        self.scalar_static_f64[459]=(if self.scalar_static_bool[108]{1.0}else{0.0});
        self.scalar_static_f64[460]=(self.scalar_static_f64[54]/self.scalar_static_f64[458]);
        self.scalar_static_f64[461]=(1.0+self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(self.scalar_static_f64[461]).ln();
        self.scalar_static_f64[463]=(self.scalar_static_f64[455]*self.scalar_static_f64[462]);
        self.scalar_static_f64[464]=(if ((self.scalar_static_f64[459])!=0.0){self.scalar_static_f64[463]}else{0.0});
        self.scalar_static_bool[109]=(!((self.scalar_static_f64[459])!=0.0));
        self.scalar_static_f64[465]=(if self.scalar_static_bool[109]{0.0}else{self.scalar_static_f64[464]});
        self.scalar_static_f64[466]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[141]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[139]*self.scalar_static_f64[466]);
        self.scalar_static_f64[468]=(self.scalar_static_f64[367]*self.scalar_static_f64[143]);
        self.scalar_static_f64[469]=(self.scalar_static_f64[355]*self.scalar_static_f64[140]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[468]/self.scalar_static_f64[469]);
        self.scalar_static_f64[471]=(self.scalar_static_f64[470]).exp();
        self.scalar_static_f64[472]=(self.scalar_static_f64[467]*self.scalar_static_f64[471]);
        self.scalar_static_bool[110]=(self.scalar_static_f64[472]>0.0);
        self.scalar_static_f64[473]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_f64[474]=(self.scalar_static_f64[54]/self.scalar_static_f64[472]);
        self.scalar_static_f64[475]=(1.0+self.scalar_static_f64[474]);
        self.scalar_static_f64[476]=(self.scalar_static_f64[475]).ln();
        self.scalar_static_f64[477]=(self.scalar_static_f64[469]*self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(if ((self.scalar_static_f64[473])!=0.0){self.scalar_static_f64[477]}else{0.0});
        self.scalar_static_bool[111]=(!((self.scalar_static_f64[473])!=0.0));
        self.scalar_static_f64[479]=(if self.scalar_static_bool[111]{0.0}else{self.scalar_static_f64[478]});
        self.scalar_static_f64[480]=(self.scalar_static_f64[18]*self.scalar_static_f64[452]);
        self.scalar_static_f64[481]=(self.scalar_static_f64[457]*self.scalar_static_f64[480]);
        self.scalar_static_bool[112]=(self.scalar_static_f64[481]>0.0);
        self.scalar_static_f64[482]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_f64[483]=(self.scalar_static_f64[54]/self.scalar_static_f64[481]);
        self.scalar_static_f64[484]=(1.0+self.scalar_static_f64[483]);
        self.scalar_static_f64[485]=(self.scalar_static_f64[484]).ln();
        self.scalar_static_f64[486]=(self.scalar_static_f64[455]*self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=(if ((self.scalar_static_f64[482])!=0.0){self.scalar_static_f64[486]}else{0.0});
        self.scalar_static_bool[113]=(!((self.scalar_static_f64[482])!=0.0));
        self.scalar_static_f64[488]=(if self.scalar_static_bool[113]{0.0}else{self.scalar_static_f64[487]});
        self.scalar_static_f64[489]=(self.scalar_static_f64[19]*self.scalar_static_f64[466]);
        self.scalar_static_f64[490]=(self.scalar_static_f64[471]*self.scalar_static_f64[489]);
        self.scalar_static_bool[114]=(self.scalar_static_f64[490]>0.0);
        self.scalar_static_f64[491]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_f64[492]=(self.scalar_static_f64[54]/self.scalar_static_f64[490]);
        self.scalar_static_f64[493]=(1.0+self.scalar_static_f64[492]);
        self.scalar_static_f64[494]=(self.scalar_static_f64[493]).ln();
        self.scalar_static_f64[495]=(self.scalar_static_f64[469]*self.scalar_static_f64[494]);
        self.scalar_static_f64[496]=(if ((self.scalar_static_f64[491])!=0.0){self.scalar_static_f64[495]}else{0.0});
        self.scalar_static_bool[115]=(!((self.scalar_static_f64[491])!=0.0));
        self.scalar_static_f64[497]=(if self.scalar_static_bool[115]{0.0}else{self.scalar_static_f64[496]});
        self.scalar_static_f64[498]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[145]);
        self.scalar_static_f64[499]=(self.scalar_static_f64[37]*self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=(self.scalar_static_f64[367]*self.scalar_static_f64[147]);
        self.scalar_static_f64[501]=(self.scalar_static_f64[355]*self.scalar_static_f64[144]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[500]/self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[502]).exp();
        self.scalar_static_f64[504]=(self.scalar_static_f64[499]*self.scalar_static_f64[503]);
        self.scalar_static_bool[116]=(self.scalar_static_f64[504]>0.0);
        self.scalar_static_f64[505]=(if self.scalar_static_bool[116]{1.0}else{0.0});
        self.scalar_static_f64[506]=(self.scalar_static_f64[54]/self.scalar_static_f64[504]);
        self.scalar_static_f64[507]=(1.0+self.scalar_static_f64[506]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[507]).ln();
        self.scalar_static_f64[509]=(self.scalar_static_f64[501]*self.scalar_static_f64[508]);
        self.scalar_static_f64[510]=(if ((self.scalar_static_f64[505])!=0.0){self.scalar_static_f64[509]}else{0.0});
        self.scalar_static_bool[117]=(!((self.scalar_static_f64[505])!=0.0));
        self.scalar_static_f64[511]=(if self.scalar_static_bool[117]{0.0}else{self.scalar_static_f64[510]});
        self.scalar_static_f64[512]=f64::powf(self.scalar_static_f64[356],self.scalar_static_f64[149]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[38]*self.scalar_static_f64[512]);
        self.scalar_static_f64[514]=(self.scalar_static_f64[367]*self.scalar_static_f64[151]);
        self.scalar_static_f64[515]=(self.scalar_static_f64[355]*self.scalar_static_f64[148]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[514]/self.scalar_static_f64[515]);
        self.scalar_static_f64[517]=(self.scalar_static_f64[516]).exp();
        self.scalar_static_f64[518]=(self.scalar_static_f64[513]*self.scalar_static_f64[517]);
        self.scalar_static_bool[118]=(self.scalar_static_f64[518]>0.0);
        self.scalar_static_f64[519]=(if self.scalar_static_bool[118]{1.0}else{0.0});
        self.scalar_static_f64[520]=(self.scalar_static_f64[54]/self.scalar_static_f64[518]);
        self.scalar_static_f64[521]=(1.0+self.scalar_static_f64[520]);
        self.scalar_static_f64[522]=(self.scalar_static_f64[521]).ln();
        self.scalar_static_f64[523]=(self.scalar_static_f64[515]*self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(if ((self.scalar_static_f64[519])!=0.0){self.scalar_static_f64[523]}else{0.0});
        self.scalar_static_bool[119]=(!((self.scalar_static_f64[519])!=0.0));
        self.scalar_static_f64[525]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[524]});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
