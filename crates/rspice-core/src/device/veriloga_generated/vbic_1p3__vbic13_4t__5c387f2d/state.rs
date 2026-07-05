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
    pub(crate) scalar_static_f64: Box<[f64; 301]>,
    pub(crate) scalar_static_bool: Box<[bool; 78]>,
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
            scalar_static_f64: boxed_zero_f64_array::<301>(),
            scalar_static_bool: boxed_zero_bool_array::<78>(),
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
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=if param_given[10]{1.0}else{0.0};
        self.scalar_static_f64[1]=p.p10;
        self.scalar_static_bool[0]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[2]=if param_given[11]{1.0}else{0.0};
        self.scalar_static_f64[3]=p.p11;
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_f64[4]=if param_given[3]{1.0}else{0.0};
        self.scalar_static_f64[5]=if param_given[4]{1.0}else{0.0};
        self.scalar_static_bool[2]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[6]=if param_given[5]{1.0}else{0.0};
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[5]!=0.0));
        self.scalar_static_f64[7]=p.p5;
        self.scalar_static_bool[4]=(!(self.scalar_static_f64[6]!=0.0));
        self.scalar_static_f64[8]=p.p12;
        self.scalar_static_f64[9]=(self.scalar_static_f64[8]).ln();
        self.scalar_static_f64[10]=p.p74;
        self.scalar_static_bool[5]=(self.scalar_static_f64[10]>0.0);
        self.scalar_static_f64[11]=(1.0/self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[5]{self.scalar_static_f64[11]}else{0.0});
        self.scalar_static_f64[13]=p.p75;
        self.scalar_static_bool[6]=(self.scalar_static_f64[13]>0.0);
        self.scalar_static_f64[14]=(1.0/self.scalar_static_f64[13]);
        self.scalar_static_f64[15]=(if self.scalar_static_bool[6]{self.scalar_static_f64[14]}else{0.0});
        self.scalar_static_f64[16]=p.p20;
        self.scalar_static_bool[7]=(self.scalar_static_f64[16]>0.0);
        self.scalar_static_f64[17]=(1.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(if self.scalar_static_bool[7]{self.scalar_static_f64[17]}else{0.0});
        self.scalar_static_f64[19]=p.p79;
        self.scalar_static_bool[8]=(self.scalar_static_f64[19]>0.0);
        self.scalar_static_f64[20]=(1.0/self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[8]{self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[22]=p.p80;
        self.scalar_static_bool[9]=(self.scalar_static_f64[22]>0.0);
        self.scalar_static_f64[23]=(1.0/self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=(if self.scalar_static_bool[9]{self.scalar_static_f64[23]}else{0.0});
        self.scalar_static_f64[25]=(if self.scalar_static_bool[9]{0.0}else{1.0});
        self.scalar_static_f64[26]=p.p13;
        self.scalar_static_f64[27]=(273.15+self.scalar_static_f64[26]);
        self.scalar_static_f64[28]=p.p0;
        self.scalar_static_f64[29]=p.p14;
        self.scalar_static_f64[30]=(1.0+self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=p.p15;
        self.scalar_static_f64[32]=(self.scalar_static_f64[31]-1.0);
        self.scalar_static_f64[33]=p.p26;
        self.scalar_static_f64[34]=p.p90;
        self.scalar_static_bool[10]=(self.scalar_static_f64[34]>0.0);
        self.scalar_static_f64[35]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[36]=p.p89;
        self.scalar_static_f64[37]=p.p88;
        self.scalar_static_f64[38]=(-self.scalar_static_f64[37]);
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[35]!=0.0));
        self.scalar_static_f64[39]=p.p122;
        self.scalar_static_f64[40]=p.p28;
        self.scalar_static_f64[41]=(self.scalar_static_f64[39]/self.scalar_static_f64[40]);
        self.scalar_static_f64[42]=p.p113;
        self.scalar_static_f64[43]=(-self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=p.p72;
        self.scalar_static_bool[12]=(self.scalar_static_f64[44]>0.0);
        self.scalar_static_f64[45]=(4.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=p.p73;
        self.scalar_static_f64[47]=f64::powf(self.scalar_static_f64[45],self.scalar_static_f64[46]);
        self.scalar_static_f64[48]=(1.0-self.scalar_static_f64[46]);
        self.scalar_static_f64[49]=(1.0/self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=p.p27;
        self.scalar_static_f64[51]=p.p125;
        self.scalar_static_f64[52]=p.p29;
        self.scalar_static_f64[53]=(self.scalar_static_f64[51]/self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=p.p121;
        self.scalar_static_f64[55]=(-self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(4.0/self.scalar_static_f64[10]);
        self.scalar_static_f64[57]=f64::powf(self.scalar_static_f64[56],self.scalar_static_f64[46]);
        self.scalar_static_f64[58]=p.p31;
        self.scalar_static_f64[59]=p.p33;
        self.scalar_static_f64[60]=(self.scalar_static_f64[39]/self.scalar_static_f64[59]);
        self.scalar_static_f64[61]=p.p120;
        self.scalar_static_f64[62]=(-self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=p.p54;
        self.scalar_static_f64[64]=p.p123;
        self.scalar_static_f64[65]=p.p56;
        self.scalar_static_f64[66]=(self.scalar_static_f64[64]/self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=p.p114;
        self.scalar_static_f64[68]=(-self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=p.p58;
        self.scalar_static_f64[70]=p.p124;
        self.scalar_static_f64[71]=p.p59;
        self.scalar_static_f64[72]=(self.scalar_static_f64[70]/self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=p.p117;
        self.scalar_static_f64[74]=(-self.scalar_static_f64[73]);
        self.scalar_static_f64[75]=p.p60;
        self.scalar_static_f64[76]=p.p61;
        self.scalar_static_f64[77]=(self.scalar_static_f64[64]/self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=p.p115;
        self.scalar_static_f64[79]=(-self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=p.p62;
        self.scalar_static_f64[81]=p.p63;
        self.scalar_static_f64[82]=(self.scalar_static_f64[70]/self.scalar_static_f64[81]);
        self.scalar_static_f64[83]=p.p118;
        self.scalar_static_f64[84]=(-self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=p.p64;
        self.scalar_static_f64[86]=p.p65;
        self.scalar_static_f64[87]=p.p66;
        self.scalar_static_f64[88]=p.p67;
        self.scalar_static_f64[89]=(self.scalar_static_f64[64]/self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=p.p116;
        self.scalar_static_f64[91]=(-self.scalar_static_f64[90]);
        self.scalar_static_f64[92]=p.p68;
        self.scalar_static_f64[93]=p.p69;
        self.scalar_static_f64[94]=(self.scalar_static_f64[70]/self.scalar_static_f64[93]);
        self.scalar_static_f64[95]=p.p119;
        self.scalar_static_f64[96]=(-self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=p.p126;
        self.scalar_static_f64[98]=if param_given[109]{1.0}else{0.0};
        self.scalar_static_f64[99]=p.p16;
        self.scalar_static_f64[100]=p.p109;
        self.scalar_static_bool[13]=(!(self.scalar_static_f64[98]!=0.0));
        self.scalar_static_f64[101]=p.p107;
        self.scalar_static_f64[102]=if param_given[108]{1.0}else{0.0};
        self.scalar_static_f64[103]=p.p17;
        self.scalar_static_f64[104]=p.p108;
        self.scalar_static_bool[14]=(!(self.scalar_static_f64[102]!=0.0));
        self.scalar_static_f64[105]=if param_given[106]{1.0}else{0.0};
        self.scalar_static_f64[106]=p.p21;
        self.scalar_static_f64[107]=p.p106;
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[105]!=0.0));
        self.scalar_static_f64[108]=p.p104;
        self.scalar_static_f64[109]=if param_given[105]{1.0}else{0.0};
        self.scalar_static_f64[110]=p.p22;
        self.scalar_static_f64[111]=p.p105;
        self.scalar_static_bool[16]=(!(self.scalar_static_f64[109]!=0.0));
        self.scalar_static_f64[112]=p.p23;
        self.scalar_static_f64[113]=p.p103;
        self.scalar_static_f64[114]=p.p24;
        self.scalar_static_f64[115]=p.p111;
        self.scalar_static_f64[116]=if param_given[110]{1.0}else{0.0};
        self.scalar_static_f64[117]=p.p25;
        self.scalar_static_f64[118]=p.p110;
        self.scalar_static_bool[17]=(!(self.scalar_static_f64[116]!=0.0));
        self.scalar_static_f64[119]=p.p101;
        self.scalar_static_f64[120]=p.p132;
        self.scalar_static_f64[121]=p.p129;
        self.scalar_static_f64[122]=p.p84;
        self.scalar_static_f64[123]=p.p127;
        self.scalar_static_f64[124]=p.p86;
        self.scalar_static_f64[125]=p.p128;
        self.scalar_static_f64[126]=p.p91;
        self.scalar_static_f64[127]=p.p92;
        self.scalar_static_f64[128]=p.p93;
        self.scalar_static_f64[129]=p.p37;
        self.scalar_static_f64[130]=(0.5*self.scalar_static_f64[129]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[129]* -0.5);
        self.scalar_static_f64[132]=p.p42;
        self.scalar_static_f64[133]=(0.5*self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=(-0.5*self.scalar_static_f64[132]);
        self.scalar_static_f64[135]=p.p50;
        self.scalar_static_f64[136]=(0.5*self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(-0.5*self.scalar_static_f64[135]);
        self.scalar_static_f64[138]=p.p36;
        self.scalar_static_f64[139]=p.p38;
        self.scalar_static_f64[140]=p.p41;
        self.scalar_static_f64[141]=p.p43;
        self.scalar_static_f64[142]=p.p48;
        self.scalar_static_f64[143]=p.p49;
        self.scalar_static_f64[144]=p.p51;
        self.scalar_static_f64[145]=p.p19;
        self.scalar_static_f64[146]=p.p18;
        self.scalar_static_f64[147]=p.p112;
        self.scalar_static_f64[148]=p.p70;
        self.scalar_static_f64[149]=p.p130;
        self.scalar_static_f64[150]=p.p71;
        self.scalar_static_f64[151]=p.p131;
        self.scalar_static_f64[152]=p.p34;
        self.scalar_static_f64[153]=p.p39;
        self.scalar_static_bool[18]=(self.scalar_static_f64[153]<=0.0);
        self.scalar_static_f64[154]=(if self.scalar_static_bool[18]{1.0}else{0.0});
        self.scalar_static_f64[155]=(1.0-self.scalar_static_f64[152]);
        self.scalar_static_f64[156]=(-self.scalar_static_f64[139]);
        self.scalar_static_f64[157]=f64::powf(self.scalar_static_f64[155],self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=(1.0-self.scalar_static_f64[139]);
        self.scalar_static_f64[159]=(0.5*self.scalar_static_f64[139]);
        self.scalar_static_bool[19]=(!(self.scalar_static_f64[154]!=0.0));
        self.scalar_static_f64[160]=(4.0*self.scalar_static_f64[153]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[153]*self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=p.p44;
        self.scalar_static_bool[20]=(self.scalar_static_f64[162]<=0.0);
        self.scalar_static_f64[163]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_f64[164]=(-1.0-self.scalar_static_f64[141]);
        self.scalar_static_f64[165]=f64::powf(self.scalar_static_f64[155],self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=(1.0-self.scalar_static_f64[141]);
        self.scalar_static_f64[167]=(0.5*self.scalar_static_f64[141]);
        self.scalar_static_f64[168]=p.p45;
        self.scalar_static_bool[21]=(self.scalar_static_f64[168]>0.0);
        self.scalar_static_f64[169]=(-self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=p.p46;
        self.scalar_static_bool[22]=(self.scalar_static_f64[170]>0.0);
        self.scalar_static_bool[23]=(self.scalar_static_bool[21]&&self.scalar_static_bool[22]);
        self.scalar_static_f64[171]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(!(self.scalar_static_f64[163]!=0.0));
        self.scalar_static_bool[25]=((self.scalar_static_f64[171]!=0.0)&&self.scalar_static_bool[24]);
        self.scalar_static_f64[172]=(4.0*self.scalar_static_f64[162]);
        self.scalar_static_f64[173]=(self.scalar_static_f64[162]*self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=(4.0*self.scalar_static_f64[170]);
        self.scalar_static_f64[175]=(self.scalar_static_f64[170]*self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=(-self.scalar_static_f64[141]);
        self.scalar_static_bool[26]=(!(self.scalar_static_f64[171]!=0.0));
        self.scalar_static_bool[27]=(self.scalar_static_bool[24]&&self.scalar_static_bool[26]);
        self.scalar_static_f64[177]=f64::powf(self.scalar_static_f64[155],self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=p.p30;
        self.scalar_static_bool[28]=(self.scalar_static_f64[178]<0.5);
        self.scalar_static_f64[179]=(if self.scalar_static_bool[28]{1.0}else{0.0});
        self.scalar_static_f64[180]=(1.0/self.scalar_static_f64[46]);
        self.scalar_static_f64[181]=f64::powf(1e-8,self.scalar_static_f64[46]);
        self.scalar_static_bool[29]=(!(self.scalar_static_f64[179]!=0.0));
        self.scalar_static_f64[182]=(1.0+self.scalar_static_f64[181]);
        self.scalar_static_bool[30]=(self.scalar_static_f64[58]>0.0);
        self.scalar_static_f64[183]=(if self.scalar_static_bool[30]{1.0}else{0.0});
        self.scalar_static_f64[184]=p.p32;
        self.scalar_static_f64[185]=(1.0-self.scalar_static_f64[184]);
        self.scalar_static_bool[31]=(!(self.scalar_static_f64[183]!=0.0));
        self.scalar_static_f64[186]=p.p55;
        self.scalar_static_bool[32]=(1.0==self.scalar_static_f64[186]);
        self.scalar_static_f64[187]=(if self.scalar_static_bool[32]{1.0}else{0.0});
        self.scalar_static_f64[188]=p.p57;
        self.scalar_static_bool[33]=(self.scalar_static_f64[188]>0.0);
        self.scalar_static_f64[189]=(if self.scalar_static_bool[33]{1.0}else{0.0});
        self.scalar_static_bool[34]=((self.scalar_static_f64[187]!=0.0)&&(self.scalar_static_f64[189]!=0.0));
        self.scalar_static_bool[35]=(!(self.scalar_static_f64[189]!=0.0));
        self.scalar_static_bool[36]=((self.scalar_static_f64[187]!=0.0)&&self.scalar_static_bool[35]);
        self.scalar_static_bool[37]=(self.scalar_static_f64[37]>0.0);
        self.scalar_static_f64[190]=(if self.scalar_static_bool[37]{1.0}else{0.0});
        self.scalar_static_bool[38]=((self.scalar_static_f64[187]!=0.0)&&(self.scalar_static_f64[190]!=0.0));
        self.scalar_static_bool[39]=(0.0==self.scalar_static_f64[186]);
        self.scalar_static_f64[191]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_bool[40]=(!(self.scalar_static_f64[187]!=0.0));
        self.scalar_static_bool[41]=((self.scalar_static_f64[191]!=0.0)&&self.scalar_static_bool[40]);
        self.scalar_static_bool[42]=((self.scalar_static_f64[190]!=0.0)&&self.scalar_static_bool[41]);
        self.scalar_static_bool[43]=(!(self.scalar_static_f64[191]!=0.0));
        self.scalar_static_bool[44]=(self.scalar_static_bool[40]&&self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=((self.scalar_static_f64[189]!=0.0)&&self.scalar_static_bool[44]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[35]&&self.scalar_static_bool[44]);
        self.scalar_static_bool[47]=((self.scalar_static_f64[190]!=0.0)&&self.scalar_static_bool[44]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[34]*self.scalar_static_f64[186]);
        self.scalar_static_f64[193]=(1.0-self.scalar_static_f64[186]);
        self.scalar_static_f64[194]=(self.scalar_static_f64[34]*self.scalar_static_f64[193]);
        self.scalar_static_bool[48]=(self.scalar_static_f64[85]>0.0);
        self.scalar_static_bool[49]=(self.scalar_static_f64[86]>0.0);
        self.scalar_static_bool[50]=(self.scalar_static_bool[48]||self.scalar_static_bool[49]);
        self.scalar_static_f64[195]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[195]!=0.0));
        self.scalar_static_f64[196]=p.p83;
        self.scalar_static_bool[52]=(self.scalar_static_f64[196]>0.0);
        self.scalar_static_f64[197]=(if self.scalar_static_bool[52]{1.0}else{0.0});
        self.scalar_static_f64[198]=(1.01-self.scalar_static_f64[141]);
        self.scalar_static_f64[199]=(1.0/self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=(self.scalar_static_f64[141]-1.0);
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[197]!=0.0));
        self.scalar_static_f64[201]=p.p85;
        self.scalar_static_bool[54]=(self.scalar_static_f64[201]>0.0);
        self.scalar_static_f64[202]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_f64[203]=p.p87;
        self.scalar_static_f64[204]=(1.01-self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(1.0/self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[203]-1.0);
        self.scalar_static_bool[55]=(!(self.scalar_static_f64[202]!=0.0));
        self.scalar_static_f64[207]=p.p97;
        self.scalar_static_bool[56]=(self.scalar_static_f64[207]>0.0);
        self.scalar_static_f64[208]=p.p95;
        self.scalar_static_bool[57]=(self.scalar_static_f64[208]>0.0);
        self.scalar_static_bool[58]=(self.scalar_static_bool[56]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[209]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_f64[210]=p.p94;
        self.scalar_static_bool[59]=(self.scalar_static_f64[210]>0.0);
        self.scalar_static_f64[211]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_bool[60]=((self.scalar_static_f64[209]!=0.0)&&(self.scalar_static_f64[211]!=0.0));
        self.scalar_static_bool[61]=(!(self.scalar_static_f64[211]!=0.0));
        self.scalar_static_bool[62]=((self.scalar_static_f64[209]!=0.0)&&self.scalar_static_bool[61]);
        self.scalar_static_f64[212]=p.p96;
        self.scalar_static_bool[63]=(!(self.scalar_static_f64[209]!=0.0));
        self.scalar_static_bool[64]=(self.scalar_static_f64[87]>0.0);
        self.scalar_static_bool[65]=(self.scalar_static_f64[92]>0.0);
        self.scalar_static_bool[66]=(self.scalar_static_bool[64]||self.scalar_static_bool[65]);
        self.scalar_static_f64[213]=(if self.scalar_static_bool[66]{1.0}else{0.0});
        self.scalar_static_bool[67]=(!(self.scalar_static_f64[213]!=0.0));
        self.scalar_static_f64[214]=p.p2;
        self.scalar_static_f64[215]=(-self.scalar_static_f64[214]);
        self.scalar_static_bool[68]=(self.scalar_static_f64[143]>0.0);
        self.scalar_static_f64[216]=(if self.scalar_static_bool[68]{1.0}else{0.0});
        self.scalar_static_f64[217]=p.p52;
        self.scalar_static_bool[69]=(self.scalar_static_f64[217]<=0.0);
        self.scalar_static_f64[218]=(if self.scalar_static_bool[69]{1.0}else{0.0});
        self.scalar_static_bool[70]=((self.scalar_static_f64[216]!=0.0)&&(self.scalar_static_f64[218]!=0.0));
        self.scalar_static_f64[219]=(-self.scalar_static_f64[144]);
        self.scalar_static_f64[220]=f64::powf(self.scalar_static_f64[155],self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=(1.0-self.scalar_static_f64[144]);
        self.scalar_static_f64[222]=(0.5*self.scalar_static_f64[144]);
        self.scalar_static_bool[71]=(!(self.scalar_static_f64[218]!=0.0));
        self.scalar_static_bool[72]=((self.scalar_static_f64[216]!=0.0)&&self.scalar_static_bool[71]);
        self.scalar_static_f64[223]=(4.0*self.scalar_static_f64[217]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[217]*self.scalar_static_f64[223]);
        self.scalar_static_bool[73]=(!(self.scalar_static_f64[216]!=0.0));
        self.scalar_static_f64[225]=p.p76;
        self.scalar_static_f64[226]=p.p77;
        self.scalar_static_f64[227]=p.p78;
        self.scalar_static_f64[228]=p.p81;
        self.scalar_static_f64[229]=p.p47;
        self.scalar_static_f64[230]=p.p53;
        self.scalar_static_f64[231]=p.p35;
        self.scalar_static_f64[232]=p.p40;
        self.scalar_static_f64[233]=p.p102;
        self.scalar_static_f64[234]=p.p82;
        self.scalar_static_f64[235]=(self.scalar_static_f64[97]-1.0);
        self.scalar_static_f64[236]=(self.scalar_static_f64[100]-1.0);
        self.scalar_static_f64[237]=(self.scalar_static_f64[101]-1.0);
        self.scalar_static_f64[238]=(self.scalar_static_f64[104]-1.0);
        self.scalar_static_f64[239]=(self.scalar_static_f64[107]-1.0);
        self.scalar_static_f64[240]=(self.scalar_static_f64[108]-1.0);
        self.scalar_static_f64[241]=(self.scalar_static_f64[111]-1.0);
        self.scalar_static_f64[242]=(self.scalar_static_f64[113]-1.0);
        self.scalar_static_f64[243]=(self.scalar_static_f64[115]-1.0);
        self.scalar_static_f64[244]=(self.scalar_static_f64[118]-1.0);
        self.scalar_static_f64[245]=(self.scalar_static_f64[41]-1.0);
        self.scalar_static_f64[246]=(self.scalar_static_f64[53]-1.0);
        self.scalar_static_f64[247]=(self.scalar_static_f64[60]-1.0);
        self.scalar_static_f64[248]=(self.scalar_static_f64[66]-1.0);
        self.scalar_static_f64[249]=(self.scalar_static_f64[72]-1.0);
        self.scalar_static_f64[250]=(self.scalar_static_f64[77]-1.0);
        self.scalar_static_f64[251]=(self.scalar_static_f64[82]-1.0);
        self.scalar_static_f64[252]=(self.scalar_static_f64[89]-1.0);
        self.scalar_static_f64[253]=(self.scalar_static_f64[94]-1.0);
        self.scalar_static_f64[254]=(self.scalar_static_f64[139]-1.0);
        self.scalar_static_f64[255]=(self.scalar_static_f64[144]-1.0);
        self.scalar_static_f64[256]=(self.scalar_static_f64[39]-1.0);
        self.scalar_static_f64[257]=(self.scalar_static_f64[147]-1.0);
        self.scalar_static_f64[258]=(self.scalar_static_f64[158]-1.0);
        self.scalar_static_f64[259]=(self.scalar_static_f64[166]-1.0);
        self.scalar_static_f64[260]=(self.scalar_static_f64[176]-1.0);
        self.scalar_static_f64[261]=(self.scalar_static_f64[180]-1.0);
        self.scalar_static_f64[262]=(self.scalar_static_f64[46]-1.0);
        self.scalar_static_f64[263]=(self.scalar_static_f64[199]-1.0);
        self.scalar_static_f64[264]=(self.scalar_static_f64[200]-1.0);
        self.scalar_static_f64[265]=(self.scalar_static_f64[205]-1.0);
        self.scalar_static_f64[266]=(self.scalar_static_f64[206]-1.0);
        self.scalar_static_f64[267]=(self.scalar_static_f64[212]-1.0);
        self.scalar_static_f64[268]=(self.scalar_static_f64[221]-1.0);
        self.scalar_static_f64[269]=(-self.scalar_static_f64[231]);
        self.scalar_static_f64[270]=(-self.scalar_static_f64[232]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[234]*0.3333333333333333);
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
        self.scalar_static_f64[272]=(temperature+self.scalar_static_f64[28]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[272]-273.15);
        self.scalar_static_bool[74]=(self.scalar_static_f64[273]<self.scalar_static_f64[30]);
        self.scalar_static_f64[274]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_f64[275]=(self.scalar_static_f64[273]-self.scalar_static_f64[29]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[275]-1.0);
        self.scalar_static_f64[277]=(self.scalar_static_f64[276]).exp();
        self.scalar_static_f64[278]=(self.scalar_static_f64[29]+self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[278]}else{self.scalar_static_f64[273]});
        self.scalar_static_bool[75]=(self.scalar_static_f64[279]>self.scalar_static_f64[32]);
        self.scalar_static_f64[280]=(if self.scalar_static_bool[75]{1.0}else{0.0});
        self.scalar_static_bool[76]=(!(self.scalar_static_f64[274]!=0.0));
        self.scalar_static_bool[77]=((self.scalar_static_f64[280]!=0.0)&&self.scalar_static_bool[76]);
        self.scalar_static_f64[281]=(self.scalar_static_f64[31]-self.scalar_static_f64[279]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[281]-1.0);
        self.scalar_static_f64[283]=(self.scalar_static_f64[282]).exp();
        self.scalar_static_f64[284]=(self.scalar_static_f64[31]-self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=(if self.scalar_static_bool[77]{self.scalar_static_f64[284]}else{self.scalar_static_f64[279]});
        self.scalar_static_f64[286]=(273.15+self.scalar_static_f64[285]);
        self.scalar_static_f64[287]=(self.scalar_static_f64[286]*1.380662e-23);
        self.scalar_static_f64[288]=(self.scalar_static_f64[287]/1.602189e-19);
        self.scalar_static_f64[289]=(self.scalar_static_f64[288]*self.scalar_static_f64[36]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[38]/self.scalar_static_f64[289]);
        self.scalar_static_f64[291]=(self.scalar_static_f64[290]).exp();
        self.scalar_static_f64[292]=(self.scalar_static_f64[288]*self.scalar_static_f64[40]);
        self.scalar_static_f64[293]=(self.scalar_static_f64[288]*self.scalar_static_f64[52]);
        self.scalar_static_f64[294]=(self.scalar_static_f64[288]*self.scalar_static_f64[59]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[288]*self.scalar_static_f64[65]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[288]*self.scalar_static_f64[71]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[288]*self.scalar_static_f64[76]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[288]*self.scalar_static_f64[81]);
        self.scalar_static_f64[299]=(self.scalar_static_f64[288]*self.scalar_static_f64[88]);
        self.scalar_static_f64[300]=(self.scalar_static_f64[288]*self.scalar_static_f64[93]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
