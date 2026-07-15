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
    pub p80: f64, pub p81: f64, pub p82: f64, pub p83: f64, pub p84: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 46] = [
                1e-17, 1.0, 0.0, 5.0, 10.0, 10.0, 0.0, 0.0,
                0.01, 1.11, 0.0, 10.0, 1e-5, 0.0, 1e-6, 0.0,
                0.0, 0.75, 0.33, 0.0, 0.001, 1.11, 3.0, 0.5,
                0.5, 25.0, 1000.0, 0.0, 1.0, 1.0, 2.0, 0.0,
                1.0, 0.0005, 0.0005, 5e-6, 1e-7, 0.0, 0.0, 2.0,
                100.0, 0.0, 1e-5, 1.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 46);
            {
                let params = &mut *ptr;
                params.p46 = 0.001;
                validate_parameter("minr", params.p46, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 38] = [
                5.0, 100.0, 2.0, 100.0, 2.0, 0.1, 0.0, 0.0,
                5.0, 0.0, 20.0, 0.0, 1.5, 1.0, 10.0, 0.0,
                0.0, 0.0, 2.0, 1e-6, 0.0, 0.0, 0.0, 0.75,
                0.33, 1.0, 0.0, 0.0, 0.75, 0.33, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.9, 1e-8, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(47), 38);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 86] = [
    ("is", 0), ("nf", 1), ("isr", 2), ("ntr", 3), ("vtr", 4), ("bvr", 5), ("xbvr", 6), ("xjbv", 7), ("ther", 8), ("theexp", 9), ("xtheexp", 10), ("nbv", 11), ("rb", 12), ("rbe", 13), ("re", 14), ("ree", 15),
    ("cje", 16), ("vje", 17), ("mje", 18), ("tf", 19), ("qtt0", 20), ("vtt0", 20), ("eg", 21), ("xti", 22), ("xtir", 23), ("fc", 24), ("tnom", 25), ("tfail", 26), ("kf", 27), ("af", 28), ("type", 29), ("shmod", 30),
    ("extmod", 31), ("rbmod", 32), ("rth0", 33), ("cth0", 34), ("rth1", 35), ("cth1", 36), ("arb", 37), ("are", 38), ("texp", 39), ("vtf0", 40), ("atff", 41), ("l", 42), ("n", 43), ("qexp", 44), ("dtemp", 45), ("minr", 46),
    ("ijbv", 47), ("vsatb", 48), ("mexp", 49), ("vsate", 50), ("mexpe", 51), ("bf", 52), ("vaf", 53), ("ikf", 54), ("xjbvc", 55), ("ijbvc", 56), ("nbvc", 57), ("ise", 58), ("ne", 59), ("br", 60), ("nr", 61), ("var", 62),
    ("ikr", 63), ("isc", 64), ("nc", 65), ("rc", 66), ("rce", 67), ("ptf", 68), ("cjc", 69), ("vjc", 70), ("mjc", 71), ("xcjc", 72), ("tr", 73), ("cjs", 74), ("vjs", 75), ("mjs", 76), ("xtb", 77), ("arc", 78),
    ("kbwm", 79), ("xbwm", 80), ("ikbwm", 81), ("xkf", 82), ("cthbb", 83), ("cdelay", 84),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 85] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 85] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 85] = [
    "is", "nf", "isr", "ntr", "vtr", "bvr", "xbvr", "xjbv", "ther", "theexp", "xtheexp", "nbv", "rb", "rbe", "re", "ree",
    "cje", "vje", "mje", "tf", "qtt0", "eg", "xti", "xtir", "fc", "tnom", "tfail", "kf", "af", "type", "shmod", "extmod",
    "rbmod", "rth0", "cth0", "rth1", "cth1", "arb", "are", "texp", "vtf0", "atff", "l", "n", "qexp", "dtemp", "minr", "ijbv",
    "vsatb", "mexp", "vsate", "mexpe", "bf", "vaf", "ikf", "xjbvc", "ijbvc", "nbvc", "ise", "ne", "br", "nr", "var", "ikr",
    "isc", "nc", "rc", "rce", "ptf", "cjc", "vjc", "mjc", "xcjc", "tr", "cjs", "vjs", "mjs", "xtb", "arc", "kbwm",
    "xbwm", "ikbwm", "xkf", "cthbb", "cdelay",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 85] = [
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

const PARAMETER_INTEGER_FLAGS: [bool; 85] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true,
    true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 85] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -20.0, label: "-20.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -40.0, label: "-40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 85] = [
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 500.0, label: "500.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 500.0, label: "500.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 20.0, label: "20.0" }), Some(ParameterBound { value: 20.0, label: "20.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 125.0, label: "125.0" }), None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), None,
    None, Some(ParameterBound { value: 500.0, label: "500.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, Some(ParameterBound { value: 1000.0, label: "1000.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
];

const PARAMETER_RANGE_FLAGS: [u8; 85] = [
    0, 3, 0, 1, 2, 2, 3, 2, 2, 2, 3, 1, 2, 2, 2, 2, 2, 2, 3, 2, 3, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0,
    0, 3, 2, 3, 2, 2, 2, 3, 3, 2, 2, 2, 2, 3, 2, 2, 3, 3, 3, 3, 3, 2, 0, 2, 2, 1, 0, 1, 3, 1, 2, 0,
    0, 1, 2, 2, 2, 2, 2, 3, 0, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 85] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
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
    pub nodes: [usize; 10],
    pub branches: [usize; 8],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 85]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 13]>,
    pub(crate) ddt_state_previous: Box<[f64; 13]>,
    pub(crate) ddt_state_older: Box<[f64; 13]>,
    pub(crate) ddt_state_initialized: Box<[bool; 13]>,
    pub(crate) ddt_derivative_current: Box<[f64; 13]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 13]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 165]>,
    pub(crate) scalar_static_bool: Box<[bool; 32]>,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 6;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 6] = ["ci", "bi", "ei", "dt1", "tt", "tbb"];

    pub const BRANCH_COUNT: usize = 8;
    pub const PARAMETER_COUNT: usize = 85;
    pub const VARIABLE_COUNT: usize = 128;
    pub const DDT_STATE_COUNT: usize = 13;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "8c6389070609414ef8eb0f1cad51464f3635d0f43defd209d06f44e6c148fb1e";
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
            scalar_static_f64: boxed_zero_f64_array::<165>(),
            scalar_static_bool: boxed_zero_bool_array::<32>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmesd'", name));
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
        self.scalar_static_f64[0]=p.p45;
        self.scalar_static_f64[1]=p.p43;
        self.scalar_static_f64[2]=p.p42;
        self.scalar_static_f64[3]=(self.scalar_static_f64[1]*self.scalar_static_f64[2]);
        self.scalar_static_f64[4]=p.p29;
        self.scalar_static_f64[5]=p.p79;
        self.scalar_static_f64[6]=p.p80;
        self.scalar_static_f64[7]=p.p25;
        self.scalar_static_f64[8]=(273.15+self.scalar_static_f64[7]);
        self.scalar_static_f64[9]=p.p77;
        self.scalar_static_f64[10]=p.p52;
        self.scalar_static_f64[11]=p.p60;
        self.scalar_static_f64[12]=p.p53;
        self.scalar_static_bool[0]=(self.scalar_static_f64[12]>0.0);
        self.scalar_static_f64[13]=(1.0/self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=(if self.scalar_static_bool[0]{self.scalar_static_f64[13]}else{0.0});
        self.scalar_static_f64[15]=p.p62;
        self.scalar_static_bool[1]=(self.scalar_static_f64[15]>0.0);
        self.scalar_static_f64[16]=(1.0/self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=(if self.scalar_static_bool[1]{self.scalar_static_f64[16]}else{0.0});
        self.scalar_static_f64[18]=p.p54;
        self.scalar_static_bool[2]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_f64[19]=(1.0/self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=(if self.scalar_static_bool[2]{self.scalar_static_f64[19]}else{0.0});
        self.scalar_static_f64[21]=p.p63;
        self.scalar_static_bool[3]=(self.scalar_static_f64[21]>0.0);
        self.scalar_static_f64[22]=(1.0/self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[3]{self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=p.p22;
        self.scalar_static_f64[25]=p.p21;
        self.scalar_static_f64[26]=p.p23;
        self.scalar_static_f64[27]=p.p0;
        self.scalar_static_f64[28]=p.p2;
        self.scalar_static_f64[29]=p.p58;
        self.scalar_static_f64[30]=p.p59;
        self.scalar_static_f64[31]=p.p64;
        self.scalar_static_f64[32]=p.p65;
        self.scalar_static_f64[33]=p.p47;
        self.scalar_static_f64[34]=p.p7;
        self.scalar_static_f64[35]=p.p5;
        self.scalar_static_f64[36]=p.p6;
        self.scalar_static_f64[37]=p.p9;
        self.scalar_static_f64[38]=p.p10;
        self.scalar_static_f64[39]=p.p56;
        self.scalar_static_f64[40]=p.p55;
        self.scalar_static_f64[41]=p.p16;
        self.scalar_static_f64[42]=p.p69;
        self.scalar_static_f64[43]=p.p74;
        self.scalar_static_f64[44]=(self.scalar_static_f64[8]/300.15);
        self.scalar_static_f64[45]=p.p17;
        self.scalar_static_f64[46]=p.p18;
        self.scalar_static_f64[47]=(self.scalar_static_f64[8]-300.15);
        self.scalar_static_f64[48]=(0.0004*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p70;
        self.scalar_static_f64[50]=p.p71;
        self.scalar_static_f64[51]=p.p75;
        self.scalar_static_f64[52]=p.p76;
        self.scalar_static_f64[53]=p.p1;
        self.scalar_static_f64[54]=p.p11;
        self.scalar_static_f64[55]=p.p8;
        self.scalar_static_f64[56]=p.p4;
        self.scalar_static_f64[57]=p.p3;
        self.scalar_static_f64[58]=p.p57;
        self.scalar_static_f64[59]=p.p61;
        self.scalar_static_f64[60]=p.p81;
        self.scalar_static_f64[61]=p.p82;
        self.scalar_static_f64[62]=p.p84;
        self.scalar_static_f64[63]=(1.0-self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=p.p48;
        self.scalar_static_f64[65]=p.p49;
        self.scalar_static_f64[66]=p.p50;
        self.scalar_static_f64[67]=p.p51;
        self.scalar_static_f64[68]=p.p12;
        self.scalar_static_f64[69]=p.p37;
        self.scalar_static_f64[70]=(1.0/self.scalar_static_f64[65]);
        self.scalar_static_f64[71]=p.p66;
        self.scalar_static_f64[72]=p.p78;
        self.scalar_static_f64[73]=p.p14;
        self.scalar_static_f64[74]=p.p38;
        self.scalar_static_f64[75]=(1.0/self.scalar_static_f64[67]);
        self.scalar_static_f64[76]=p.p40;
        self.scalar_static_f64[77]=p.p39;
        self.scalar_static_f64[78]=(1.0/self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=p.p19;
        self.scalar_static_f64[80]=p.p41;
        self.scalar_static_f64[81]=p.p73;
        self.scalar_static_f64[82]=p.p32;
        self.scalar_static_bool[4]=(1.0==self.scalar_static_f64[82]);
        self.scalar_static_f64[83]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[84]=p.p20;
        self.scalar_static_f64[85]=p.p44;
        self.scalar_static_f64[86]=p.p31;
        self.scalar_static_bool[5]=(1.0==self.scalar_static_f64[86]);
        self.scalar_static_f64[87]=(if self.scalar_static_bool[5]{1.0}else{0.0});
        self.scalar_static_f64[88]=p.p13;
        self.scalar_static_f64[89]=p.p67;
        self.scalar_static_f64[90]=p.p15;
        self.scalar_static_f64[91]=(1.0-self.scalar_static_f64[52]);
        self.scalar_static_f64[92]=(self.scalar_static_f64[52]*0.5);
        self.scalar_static_f64[93]=p.p24;
        self.scalar_static_f64[94]=(-1.0-self.scalar_static_f64[46]);
        self.scalar_static_f64[95]=(1.0-self.scalar_static_f64[93]);
        self.scalar_static_f64[96]=(self.scalar_static_f64[95]).ln();
        self.scalar_static_f64[97]=(self.scalar_static_f64[94]*self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=(self.scalar_static_f64[97]).exp();
        self.scalar_static_f64[99]=(1.0-self.scalar_static_f64[46]);
        self.scalar_static_f64[100]=(self.scalar_static_f64[46]*0.5);
        self.scalar_static_f64[101]=(-1.0-self.scalar_static_f64[50]);
        self.scalar_static_f64[102]=(self.scalar_static_f64[96]*self.scalar_static_f64[101]);
        self.scalar_static_f64[103]=(self.scalar_static_f64[102]).exp();
        self.scalar_static_f64[104]=(1.0-self.scalar_static_f64[50]);
        self.scalar_static_f64[105]=(self.scalar_static_f64[50]*0.5);
        self.scalar_static_f64[106]=p.p72;
        self.scalar_static_f64[107]=(1.0-self.scalar_static_f64[106]);
        self.scalar_static_f64[108]=p.p68;
        self.scalar_static_bool[6]=(0.0!=self.scalar_static_f64[108]);
        self.scalar_static_bool[7]=(0.0!=self.scalar_static_f64[79]);
        self.scalar_static_bool[8]=(self.scalar_static_bool[6]&&self.scalar_static_bool[7]);
        self.scalar_static_f64[109]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[110]=(self.scalar_static_f64[4]*self.scalar_static_f64[108]);
        self.scalar_static_f64[111]=(self.scalar_static_f64[110]*3.141592653589793);
        self.scalar_static_f64[112]=(self.scalar_static_f64[111]/180.0);
        self.scalar_static_f64[113]=(self.scalar_static_f64[79]*self.scalar_static_f64[112]);
        self.scalar_static_bool[9]=(!((self.scalar_static_f64[109])!=0.0));
        self.scalar_static_f64[114]=p.p30;
        self.scalar_static_bool[10]=(1.0==self.scalar_static_f64[114]);
        self.scalar_static_f64[115]=p.p33;
        self.scalar_static_bool[11]=(self.scalar_static_f64[115]>0.0);
        self.scalar_static_bool[12]=(self.scalar_static_bool[10]&&self.scalar_static_bool[11]);
        self.scalar_static_f64[116]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_bool[13]=(2.0==self.scalar_static_f64[114]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[11]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[117]=p.p35;
        self.scalar_static_bool[15]=(self.scalar_static_f64[117]>0.0);
        self.scalar_static_bool[16]=(self.scalar_static_bool[14]&&self.scalar_static_bool[15]);
        self.scalar_static_f64[118]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_bool[17]=(-1.0==self.scalar_static_f64[114]);
        self.scalar_static_f64[119]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_f64[120]=(self.scalar_static_f64[86]*self.scalar_static_f64[88]);
        self.scalar_static_f64[121]=(self.scalar_static_f64[68]+self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[121]/self.scalar_static_f64[3]);
        self.scalar_static_f64[123]=(self.scalar_static_f64[86]*self.scalar_static_f64[90]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[73]+self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(self.scalar_static_f64[124]/self.scalar_static_f64[3]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[86]*self.scalar_static_f64[89]);
        self.scalar_static_f64[127]=(self.scalar_static_f64[71]+self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[127]/self.scalar_static_f64[3]);
        self.scalar_static_bool[18]=(self.scalar_static_f64[122]>0.0);
        self.scalar_static_f64[129]=p.p46;
        self.scalar_static_bool[19]=(self.scalar_static_f64[122]>=self.scalar_static_f64[129]);
        self.scalar_static_bool[20]=(self.scalar_static_bool[18]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[130]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_bool[21]=(self.scalar_static_f64[125]>0.0);
        self.scalar_static_bool[22]=(self.scalar_static_f64[125]>=self.scalar_static_f64[129]);
        self.scalar_static_bool[23]=(self.scalar_static_bool[21]&&self.scalar_static_bool[22]);
        self.scalar_static_f64[131]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(self.scalar_static_f64[128]>0.0);
        self.scalar_static_bool[25]=(self.scalar_static_f64[128]>=self.scalar_static_f64[129]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[24]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[132]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_f64[133]=p.p83;
        self.scalar_static_f64[134]=p.p34;
        self.scalar_static_bool[27]=(!((self.scalar_static_f64[116])!=0.0));
        self.scalar_static_bool[28]=(((self.scalar_static_f64[118])!=0.0)&&self.scalar_static_bool[27]);
        self.scalar_static_f64[135]=p.p36;
        self.scalar_static_bool[29]=(!((self.scalar_static_f64[118])!=0.0));
        self.scalar_static_bool[30]=(self.scalar_static_bool[27]&&self.scalar_static_bool[29]);
        self.scalar_static_bool[31]=(((self.scalar_static_f64[119])!=0.0)&&self.scalar_static_bool[30]);
        self.scalar_static_f64[136]=(-self.scalar_static_f64[4]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[6]-1.0);
        self.scalar_static_f64[138]=(self.scalar_static_f64[56]*self.scalar_static_f64[136]);
        self.scalar_static_f64[139]=(self.scalar_static_f64[4]*self.scalar_static_f64[56]);
        self.scalar_static_f64[140]=(self.scalar_static_f64[4]*self.scalar_static_f64[17]);
        self.scalar_static_f64[141]=(self.scalar_static_f64[17]*self.scalar_static_f64[136]);
        self.scalar_static_f64[142]=(-self.scalar_static_f64[140]);
        self.scalar_static_f64[143]=(-self.scalar_static_f64[141]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[14]*self.scalar_static_f64[136]);
        self.scalar_static_f64[145]=(self.scalar_static_f64[4]*self.scalar_static_f64[14]);
        self.scalar_static_f64[146]=(-self.scalar_static_f64[144]);
        self.scalar_static_f64[147]=(self.scalar_static_f64[142]-self.scalar_static_f64[145]);
        self.scalar_static_f64[148]=(2.0*self.scalar_static_f64[146]);
        self.scalar_static_f64[149]=(2.0*self.scalar_static_f64[147]);
        self.scalar_static_f64[150]=(2.0*self.scalar_static_f64[143]);
        self.scalar_static_f64[151]=(self.scalar_static_f64[4]*self.scalar_static_f64[92]);
        self.scalar_static_f64[152]=(self.scalar_static_f64[92]*self.scalar_static_f64[136]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[4]*self.scalar_static_f64[100]);
        self.scalar_static_f64[154]=(self.scalar_static_f64[100]*self.scalar_static_f64[136]);
        self.scalar_static_f64[155]=(self.scalar_static_f64[4]*self.scalar_static_f64[105]);
        self.scalar_static_f64[156]=(self.scalar_static_f64[105]*self.scalar_static_f64[136]);
        self.scalar_static_f64[157]=(if ((self.scalar_static_f64[83])!=0.0){1.0}else{0.0});
        self.scalar_static_f64[158]=(1.0/self.scalar_static_f64[115]);
        self.scalar_static_f64[159]=(if ((self.scalar_static_f64[116])!=0.0){self.scalar_static_f64[158]}else{0.0});
        self.scalar_static_f64[160]=(-1.0/self.scalar_static_f64[115]);
        self.scalar_static_f64[161]=(if self.scalar_static_bool[28]{self.scalar_static_f64[158]}else{0.0});
        self.scalar_static_f64[162]=(if self.scalar_static_bool[28]{self.scalar_static_f64[160]}else{0.0});
        self.scalar_static_f64[163]=(1.0/self.scalar_static_f64[117]);
        self.scalar_static_f64[164]=(if self.scalar_static_bool[28]{self.scalar_static_f64[163]}else{0.0});
    }
}
