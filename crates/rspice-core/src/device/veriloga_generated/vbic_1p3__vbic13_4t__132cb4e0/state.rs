#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 133],
}

impl std::ops::Index<usize> for Parameters {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output { &self.values[index] }
}

impl std::ops::IndexMut<usize> for Parameters {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.values[index] }
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: every parameter slot is f64, so zero bytes are valid 0.0 values; numeric default chunks are copied into the values array.
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
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 133);
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
    parameters.values[index]
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
    pub nodes: [usize; 14],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 133]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<11, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<425, 111>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["dt", "cx", "ci", "bx", "bi", "ei", "bp", "si", "xf1", "xf2"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 133;
    pub const VARIABLE_COUNT: usize = 359;
    pub const DDT_STATE_COUNT: usize = 11;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "987d4a7e1190f13951787a1c84d8efff0d088869d9c30073974f9de6e42baab1";
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
        let mut values = Vec::with_capacity(55);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(11);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 55);
        debug_assert_eq!(state.flags.len(), 11);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic13_4t'", name));
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
        let slot = &mut self.params.values[index];
        let changed = slot.to_bits() != value.to_bits();
        *slot = value;
        changed
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
        let param_given = self.param_given.as_ref();
        self.scalar_static.f64_values[0]=if param_given[10]{1.0}else{0.0};
        self.scalar_static.f64_values[1]=if param_given[11]{1.0}else{0.0};
        self.scalar_static.f64_values[2]=if param_given[3]{1.0}else{0.0};
        self.scalar_static.f64_values[3]=if param_given[4]{1.0}else{0.0};
        self.scalar_static.f64_values[4]=if param_given[5]{1.0}else{0.0};
        self.scalar_static.f64_values[5]=p[90];
        self.scalar_static.bool_values[0]=(self.scalar_static.f64_values[5]>0.0);
        self.scalar_static.f64_values[6]=(if self.scalar_static.bool_values[0]{1.0}else{0.0});
        self.scalar_static.f64_values[7]=if param_given[109]{1.0}else{0.0};
        self.scalar_static.f64_values[8]=if param_given[108]{1.0}else{0.0};
        self.scalar_static.f64_values[9]=if param_given[106]{1.0}else{0.0};
        self.scalar_static.f64_values[10]=if param_given[105]{1.0}else{0.0};
        self.scalar_static.f64_values[11]=if param_given[110]{1.0}else{0.0};
        self.scalar_static.f64_values[12]=p[39];
        self.scalar_static.bool_values[1]=(self.scalar_static.f64_values[12]<=0.0);
        self.scalar_static.f64_values[13]=(if self.scalar_static.bool_values[1]{1.0}else{0.0});
        self.scalar_static.f64_values[14]=p[44];
        self.scalar_static.bool_values[2]=(self.scalar_static.f64_values[14]<=0.0);
        self.scalar_static.f64_values[15]=(if self.scalar_static.bool_values[2]{1.0}else{0.0});
        self.scalar_static.f64_values[16]=p[45];
        self.scalar_static.bool_values[3]=(self.scalar_static.f64_values[16]>0.0);
        self.scalar_static.f64_values[17]=p[46];
        self.scalar_static.bool_values[4]=(self.scalar_static.f64_values[17]>0.0);
        self.scalar_static.bool_values[5]=(self.scalar_static.bool_values[3]&&self.scalar_static.bool_values[4]);
        self.scalar_static.f64_values[18]=(if self.scalar_static.bool_values[5]{1.0}else{0.0});
        self.scalar_static.f64_values[19]=p[30];
        self.scalar_static.bool_values[6]=(self.scalar_static.f64_values[19]<0.5);
        self.scalar_static.f64_values[20]=(if self.scalar_static.bool_values[6]{1.0}else{0.0});
        self.scalar_static.f64_values[21]=p[31];
        self.scalar_static.bool_values[7]=(self.scalar_static.f64_values[21]>0.0);
        self.scalar_static.f64_values[22]=(if self.scalar_static.bool_values[7]{1.0}else{0.0});
        self.scalar_static.f64_values[23]=p[55];
        self.scalar_static.bool_values[8]=(1.0==self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[24]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.f64_values[25]=p[57];
        self.scalar_static.bool_values[9]=(self.scalar_static.f64_values[25]>0.0);
        self.scalar_static.f64_values[26]=(if self.scalar_static.bool_values[9]{1.0}else{0.0});
        self.scalar_static.f64_values[27]=p[88];
        self.scalar_static.bool_values[10]=(self.scalar_static.f64_values[27]>0.0);
        self.scalar_static.f64_values[28]=(if self.scalar_static.bool_values[10]{1.0}else{0.0});
        self.scalar_static.bool_values[11]=(0.0==self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[29]=(if self.scalar_static.bool_values[11]{1.0}else{0.0});
        self.scalar_static.f64_values[30]=p[64];
        self.scalar_static.bool_values[12]=(self.scalar_static.f64_values[30]>0.0);
        self.scalar_static.f64_values[31]=p[65];
        self.scalar_static.bool_values[13]=(self.scalar_static.f64_values[31]>0.0);
        self.scalar_static.bool_values[14]=(self.scalar_static.bool_values[12]||self.scalar_static.bool_values[13]);
        self.scalar_static.f64_values[32]=(if self.scalar_static.bool_values[14]{1.0}else{0.0});
        self.scalar_static.f64_values[33]=p[83];
        self.scalar_static.bool_values[15]=(self.scalar_static.f64_values[33]>0.0);
        self.scalar_static.f64_values[34]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.f64_values[35]=p[85];
        self.scalar_static.bool_values[16]=(self.scalar_static.f64_values[35]>0.0);
        self.scalar_static.f64_values[36]=(if self.scalar_static.bool_values[16]{1.0}else{0.0});
        self.scalar_static.f64_values[37]=p[97];
        self.scalar_static.bool_values[17]=(self.scalar_static.f64_values[37]>0.0);
        self.scalar_static.f64_values[38]=p[95];
        self.scalar_static.bool_values[18]=(self.scalar_static.f64_values[38]>0.0);
        self.scalar_static.bool_values[19]=(self.scalar_static.bool_values[17]&&self.scalar_static.bool_values[18]);
        self.scalar_static.f64_values[39]=(if self.scalar_static.bool_values[19]{1.0}else{0.0});
        self.scalar_static.f64_values[40]=p[94];
        self.scalar_static.bool_values[20]=(self.scalar_static.f64_values[40]>0.0);
        self.scalar_static.f64_values[41]=(if self.scalar_static.bool_values[20]{1.0}else{0.0});
        self.scalar_static.f64_values[42]=p[66];
        self.scalar_static.bool_values[21]=(self.scalar_static.f64_values[42]>0.0);
        self.scalar_static.f64_values[43]=p[68];
        self.scalar_static.bool_values[22]=(self.scalar_static.f64_values[43]>0.0);
        self.scalar_static.bool_values[23]=(self.scalar_static.bool_values[21]||self.scalar_static.bool_values[22]);
        self.scalar_static.f64_values[44]=(if self.scalar_static.bool_values[23]{1.0}else{0.0});
        self.scalar_static.f64_values[45]=p[49];
        self.scalar_static.bool_values[24]=(self.scalar_static.f64_values[45]>0.0);
        self.scalar_static.f64_values[46]=(if self.scalar_static.bool_values[24]{1.0}else{0.0});
        self.scalar_static.f64_values[47]=p[52];
        self.scalar_static.bool_values[25]=(self.scalar_static.f64_values[47]<=0.0);
        self.scalar_static.f64_values[48]=(if self.scalar_static.bool_values[25]{1.0}else{0.0});
        self.scalar_static.f64_values[49]=p[10];
        self.scalar_static.f64_values[50]=(if ((self.scalar_static.f64_values[0])!=0.0){self.scalar_static.f64_values[49]}else{0.0});
        self.scalar_static.bool_values[26]=(!((self.scalar_static.f64_values[0])!=0.0));
        self.scalar_static.f64_values[51]=p[11];
        self.scalar_static.f64_values[52]=(if ((self.scalar_static.f64_values[1])!=0.0){self.scalar_static.f64_values[51]}else{0.0});
        self.scalar_static.bool_values[27]=(!((self.scalar_static.f64_values[1])!=0.0));
        self.scalar_static.f64_values[53]=p[72];
        self.scalar_static.bool_values[28]=(self.scalar_static.f64_values[53]>0.0);
        self.scalar_static.f64_values[54]=p[74];
        self.scalar_static.bool_values[29]=(self.scalar_static.f64_values[54]>0.0);
        self.scalar_static.f64_values[55]=p[75];
        self.scalar_static.bool_values[30]=(self.scalar_static.f64_values[55]>0.0);
        self.scalar_static.f64_values[56]=(if ((self.scalar_static.f64_values[2])!=0.0){1.0}else{0.0});
        self.scalar_static.bool_values[31]=(!((self.scalar_static.f64_values[2])!=0.0));
        self.scalar_static.bool_values[32]=(((self.scalar_static.f64_values[3])!=0.0)&&self.scalar_static.bool_values[31]);
        self.scalar_static.f64_values[57]=(if self.scalar_static.bool_values[32]{-1.0}else{self.scalar_static.f64_values[56]});
        self.scalar_static.f64_values[58]=p[5];
        self.scalar_static.bool_values[33]=(!((self.scalar_static.f64_values[3])!=0.0));
        self.scalar_static.bool_values[34]=(self.scalar_static.bool_values[31]&&self.scalar_static.bool_values[33]);
        self.scalar_static.bool_values[35]=(((self.scalar_static.f64_values[4])!=0.0)&&self.scalar_static.bool_values[34]);
        self.scalar_static.f64_values[59]=(if self.scalar_static.bool_values[35]{self.scalar_static.f64_values[58]}else{self.scalar_static.f64_values[57]});
        self.scalar_static.bool_values[36]=(!((self.scalar_static.f64_values[4])!=0.0));
        self.scalar_static.bool_values[37]=(self.scalar_static.bool_values[34]&&self.scalar_static.bool_values[36]);
        self.scalar_static.f64_values[60]=(if self.scalar_static.bool_values[37]{1.0}else{self.scalar_static.f64_values[59]});
        self.scalar_static.f64_values[61]=p[12];
        self.scalar_static.f64_values[62]=(self.scalar_static.f64_values[61]).ln();
        self.scalar_static.f64_values[63]=(1.0/self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[64]=(if self.scalar_static.bool_values[29]{self.scalar_static.f64_values[63]}else{0.0});
        self.scalar_static.f64_values[65]=(1.0/self.scalar_static.f64_values[55]);
        self.scalar_static.f64_values[66]=(if self.scalar_static.bool_values[30]{self.scalar_static.f64_values[65]}else{0.0});
        self.scalar_static.f64_values[67]=p[20];
        self.scalar_static.bool_values[38]=(self.scalar_static.f64_values[67]>0.0);
        self.scalar_static.f64_values[68]=(1.0/self.scalar_static.f64_values[67]);
        self.scalar_static.f64_values[69]=(if self.scalar_static.bool_values[38]{self.scalar_static.f64_values[68]}else{0.0});
        self.scalar_static.f64_values[70]=p[79];
        self.scalar_static.bool_values[39]=(self.scalar_static.f64_values[70]>0.0);
        self.scalar_static.f64_values[71]=(1.0/self.scalar_static.f64_values[70]);
        self.scalar_static.f64_values[72]=(if self.scalar_static.bool_values[39]{self.scalar_static.f64_values[71]}else{0.0});
        self.scalar_static.f64_values[73]=p[80];
        self.scalar_static.bool_values[40]=(self.scalar_static.f64_values[73]>0.0);
        self.scalar_static.f64_values[74]=(1.0/self.scalar_static.f64_values[73]);
        self.scalar_static.f64_values[75]=(if self.scalar_static.bool_values[40]{self.scalar_static.f64_values[74]}else{0.0});
        self.scalar_static.f64_values[76]=(if self.scalar_static.bool_values[40]{0.0}else{1.0});
        self.scalar_static.f64_values[77]=p[13];
        self.scalar_static.f64_values[78]=(273.15+self.scalar_static.f64_values[77]);
        self.scalar_static.f64_values[79]=p[0];
        self.scalar_static.f64_values[80]=p[14];
        self.scalar_static.f64_values[81]=(1.0+self.scalar_static.f64_values[80]);
        self.scalar_static.f64_values[82]=p[15];
        self.scalar_static.f64_values[83]=(self.scalar_static.f64_values[82]-1.0);
        self.scalar_static.f64_values[84]=p[26];
        self.scalar_static.f64_values[85]=p[89];
        self.scalar_static.f64_values[86]=(-self.scalar_static.f64_values[27]);
        self.scalar_static.bool_values[41]=(!((self.scalar_static.f64_values[6])!=0.0));
        self.scalar_static.f64_values[87]=p[122];
        self.scalar_static.f64_values[88]=p[28];
        self.scalar_static.f64_values[89]=(self.scalar_static.f64_values[87]/self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[90]=p[113];
        self.scalar_static.f64_values[91]=(-self.scalar_static.f64_values[90]);
        self.scalar_static.f64_values[92]=(4.0/self.scalar_static.f64_values[53]);
        self.scalar_static.f64_values[93]=p[73];
        self.scalar_static.f64_values[94]=f64::powf(self.scalar_static.f64_values[92],self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[95]=(1.0-self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[96]=(1.0/self.scalar_static.f64_values[95]);
        self.scalar_static.f64_values[97]=p[27];
        self.scalar_static.f64_values[98]=p[125];
        self.scalar_static.f64_values[99]=p[29];
        self.scalar_static.f64_values[100]=(self.scalar_static.f64_values[98]/self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[101]=p[121];
        self.scalar_static.f64_values[102]=(-self.scalar_static.f64_values[101]);
        self.scalar_static.f64_values[103]=(4.0/self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[104]=f64::powf(self.scalar_static.f64_values[103],self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[105]=p[33];
        self.scalar_static.f64_values[106]=(self.scalar_static.f64_values[87]/self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[107]=p[120];
        self.scalar_static.f64_values[108]=(-self.scalar_static.f64_values[107]);
        self.scalar_static.f64_values[109]=p[54];
        self.scalar_static.f64_values[110]=p[123];
        self.scalar_static.f64_values[111]=p[56];
        self.scalar_static.f64_values[112]=(self.scalar_static.f64_values[110]/self.scalar_static.f64_values[111]);
        self.scalar_static.f64_values[113]=p[114];
        self.scalar_static.f64_values[114]=(-self.scalar_static.f64_values[113]);
        self.scalar_static.f64_values[115]=p[58];
        self.scalar_static.f64_values[116]=p[124];
        self.scalar_static.f64_values[117]=p[59];
        self.scalar_static.f64_values[118]=(self.scalar_static.f64_values[116]/self.scalar_static.f64_values[117]);
        self.scalar_static.f64_values[119]=p[117];
        self.scalar_static.f64_values[120]=(-self.scalar_static.f64_values[119]);
        self.scalar_static.f64_values[121]=p[60];
        self.scalar_static.f64_values[122]=p[61];
        self.scalar_static.f64_values[123]=(self.scalar_static.f64_values[110]/self.scalar_static.f64_values[122]);
        self.scalar_static.f64_values[124]=p[115];
        self.scalar_static.f64_values[125]=(-self.scalar_static.f64_values[124]);
        self.scalar_static.f64_values[126]=p[62];
        self.scalar_static.f64_values[127]=p[63];
        self.scalar_static.f64_values[128]=(self.scalar_static.f64_values[116]/self.scalar_static.f64_values[127]);
        self.scalar_static.f64_values[129]=p[118];
        self.scalar_static.f64_values[130]=(-self.scalar_static.f64_values[129]);
        self.scalar_static.f64_values[131]=p[67];
        self.scalar_static.f64_values[132]=(self.scalar_static.f64_values[110]/self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[133]=p[116];
        self.scalar_static.f64_values[134]=(-self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[135]=p[69];
        self.scalar_static.f64_values[136]=(self.scalar_static.f64_values[116]/self.scalar_static.f64_values[135]);
        self.scalar_static.f64_values[137]=p[119];
        self.scalar_static.f64_values[138]=(-self.scalar_static.f64_values[137]);
        self.scalar_static.f64_values[139]=p[126];
        self.scalar_static.f64_values[140]=p[16];
        self.scalar_static.f64_values[141]=p[109];
        self.scalar_static.bool_values[42]=(!((self.scalar_static.f64_values[7])!=0.0));
        self.scalar_static.f64_values[142]=p[107];
        self.scalar_static.f64_values[143]=p[17];
        self.scalar_static.f64_values[144]=p[108];
        self.scalar_static.bool_values[43]=(!((self.scalar_static.f64_values[8])!=0.0));
        self.scalar_static.f64_values[145]=p[21];
        self.scalar_static.f64_values[146]=p[106];
        self.scalar_static.bool_values[44]=(!((self.scalar_static.f64_values[9])!=0.0));
        self.scalar_static.f64_values[147]=p[104];
        self.scalar_static.f64_values[148]=p[22];
        self.scalar_static.f64_values[149]=p[105];
        self.scalar_static.bool_values[45]=(!((self.scalar_static.f64_values[10])!=0.0));
        self.scalar_static.f64_values[150]=p[23];
        self.scalar_static.f64_values[151]=p[103];
        self.scalar_static.f64_values[152]=p[24];
        self.scalar_static.f64_values[153]=p[111];
        self.scalar_static.f64_values[154]=p[25];
        self.scalar_static.f64_values[155]=p[110];
        self.scalar_static.bool_values[46]=(!((self.scalar_static.f64_values[11])!=0.0));
        self.scalar_static.f64_values[156]=p[101];
        self.scalar_static.f64_values[157]=p[132];
        self.scalar_static.f64_values[158]=p[129];
        self.scalar_static.f64_values[159]=p[84];
        self.scalar_static.f64_values[160]=p[127];
        self.scalar_static.f64_values[161]=p[86];
        self.scalar_static.f64_values[162]=p[128];
        self.scalar_static.f64_values[163]=p[91];
        self.scalar_static.f64_values[164]=p[92];
        self.scalar_static.f64_values[165]=p[93];
        self.scalar_static.f64_values[166]=p[37];
        self.scalar_static.f64_values[167]=(0.5*self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[168]=(self.scalar_static.f64_values[166]* -0.5);
        self.scalar_static.f64_values[169]=p[42];
        self.scalar_static.f64_values[170]=(0.5*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[171]=(-0.5*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[172]=p[50];
        self.scalar_static.f64_values[173]=(0.5*self.scalar_static.f64_values[172]);
        self.scalar_static.f64_values[174]=(-0.5*self.scalar_static.f64_values[172]);
        self.scalar_static.f64_values[175]=p[36];
        self.scalar_static.f64_values[176]=p[38];
        self.scalar_static.f64_values[177]=p[41];
        self.scalar_static.f64_values[178]=p[43];
        self.scalar_static.f64_values[179]=p[48];
        self.scalar_static.f64_values[180]=p[51];
        self.scalar_static.f64_values[181]=p[19];
        self.scalar_static.f64_values[182]=p[18];
        self.scalar_static.f64_values[183]=p[112];
        self.scalar_static.f64_values[184]=p[70];
        self.scalar_static.f64_values[185]=p[130];
        self.scalar_static.f64_values[186]=p[71];
        self.scalar_static.f64_values[187]=p[131];
        self.scalar_static.f64_values[188]=p[34];
        self.scalar_static.f64_values[189]=(1.0-self.scalar_static.f64_values[188]);
        self.scalar_static.f64_values[190]=(-self.scalar_static.f64_values[176]);
        self.scalar_static.f64_values[191]=f64::powf(self.scalar_static.f64_values[189],self.scalar_static.f64_values[190]);
        self.scalar_static.f64_values[192]=(1.0-self.scalar_static.f64_values[176]);
        self.scalar_static.f64_values[193]=(0.5*self.scalar_static.f64_values[176]);
        self.scalar_static.bool_values[47]=(!((self.scalar_static.f64_values[13])!=0.0));
        self.scalar_static.f64_values[194]=(self.scalar_static.f64_values[12]*4.0);
        self.scalar_static.f64_values[195]=(self.scalar_static.f64_values[12]*self.scalar_static.f64_values[194]);
        self.scalar_static.f64_values[196]=(-1.0-self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[197]=f64::powf(self.scalar_static.f64_values[189],self.scalar_static.f64_values[196]);
        self.scalar_static.f64_values[198]=(1.0-self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[199]=(0.5*self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[200]=(-self.scalar_static.f64_values[16]);
        self.scalar_static.bool_values[48]=(!((self.scalar_static.f64_values[15])!=0.0));
        self.scalar_static.bool_values[49]=(((self.scalar_static.f64_values[18])!=0.0)&&self.scalar_static.bool_values[48]);
        self.scalar_static.f64_values[201]=(self.scalar_static.f64_values[14]*4.0);
        self.scalar_static.f64_values[202]=(self.scalar_static.f64_values[14]*self.scalar_static.f64_values[201]);
        self.scalar_static.f64_values[203]=(self.scalar_static.f64_values[17]*4.0);
        self.scalar_static.f64_values[204]=(self.scalar_static.f64_values[17]*self.scalar_static.f64_values[203]);
        self.scalar_static.f64_values[205]=(-self.scalar_static.f64_values[178]);
        self.scalar_static.bool_values[50]=(!((self.scalar_static.f64_values[18])!=0.0));
        self.scalar_static.bool_values[51]=(self.scalar_static.bool_values[48]&&self.scalar_static.bool_values[50]);
        self.scalar_static.f64_values[206]=f64::powf(self.scalar_static.f64_values[189],self.scalar_static.f64_values[205]);
        self.scalar_static.f64_values[207]=(1.0/self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[208]=f64::powf(1e-8,self.scalar_static.f64_values[93]);
        self.scalar_static.bool_values[52]=(!((self.scalar_static.f64_values[20])!=0.0));
        self.scalar_static.f64_values[209]=(1.0+self.scalar_static.f64_values[208]);
        self.scalar_static.f64_values[210]=p[32];
        self.scalar_static.f64_values[211]=(1.0-self.scalar_static.f64_values[210]);
        self.scalar_static.bool_values[53]=(!self.scalar_static.bool_values[6]);
        self.scalar_static.bool_values[54]=(!((self.scalar_static.f64_values[22])!=0.0));
        self.scalar_static.bool_values[55]=(((self.scalar_static.f64_values[24])!=0.0)&&((self.scalar_static.f64_values[26])!=0.0));
        self.scalar_static.bool_values[56]=(!((self.scalar_static.f64_values[26])!=0.0));
        self.scalar_static.bool_values[57]=(((self.scalar_static.f64_values[24])!=0.0)&&self.scalar_static.bool_values[56]);
        self.scalar_static.bool_values[58]=(((self.scalar_static.f64_values[24])!=0.0)&&((self.scalar_static.f64_values[28])!=0.0));
        self.scalar_static.bool_values[59]=(!self.scalar_static.bool_values[9]);
        self.scalar_static.bool_values[60]=(!((self.scalar_static.f64_values[24])!=0.0));
        self.scalar_static.bool_values[61]=(((self.scalar_static.f64_values[29])!=0.0)&&self.scalar_static.bool_values[60]);
        self.scalar_static.bool_values[62]=(!self.scalar_static.bool_values[8]);
        self.scalar_static.bool_values[63]=(self.scalar_static.bool_values[11]&&self.scalar_static.bool_values[62]);
        self.scalar_static.bool_values[64]=(((self.scalar_static.f64_values[28])!=0.0)&&self.scalar_static.bool_values[61]);
        self.scalar_static.bool_values[65]=(!((self.scalar_static.f64_values[29])!=0.0));
        self.scalar_static.bool_values[66]=(self.scalar_static.bool_values[60]&&self.scalar_static.bool_values[65]);
        self.scalar_static.bool_values[67]=(!self.scalar_static.bool_values[11]);
        self.scalar_static.bool_values[68]=(self.scalar_static.bool_values[62]&&self.scalar_static.bool_values[67]);
        self.scalar_static.bool_values[69]=(((self.scalar_static.f64_values[26])!=0.0)&&self.scalar_static.bool_values[66]);
        self.scalar_static.bool_values[70]=(self.scalar_static.bool_values[56]&&self.scalar_static.bool_values[66]);
        self.scalar_static.bool_values[71]=(((self.scalar_static.f64_values[28])!=0.0)&&self.scalar_static.bool_values[66]);
        self.scalar_static.f64_values[212]=(self.scalar_static.f64_values[5]*self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[213]=(1.0-self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[214]=(self.scalar_static.f64_values[5]*self.scalar_static.f64_values[213]);
        self.scalar_static.bool_values[72]=(!((self.scalar_static.f64_values[32])!=0.0));
        self.scalar_static.f64_values[215]=(self.scalar_static.f64_values[62]).exp();
        self.scalar_static.f64_values[216]=(1.01-self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[217]=(1.0/self.scalar_static.f64_values[216]);
        self.scalar_static.f64_values[218]=(self.scalar_static.f64_values[178]-1.0);
        self.scalar_static.bool_values[73]=(!((self.scalar_static.f64_values[34])!=0.0));
        self.scalar_static.f64_values[219]=p[87];
        self.scalar_static.f64_values[220]=(1.01-self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[221]=(1.0/self.scalar_static.f64_values[220]);
        self.scalar_static.f64_values[222]=(self.scalar_static.f64_values[219]-1.0);
        self.scalar_static.bool_values[74]=(!((self.scalar_static.f64_values[36])!=0.0));
        self.scalar_static.bool_values[75]=(((self.scalar_static.f64_values[39])!=0.0)&&((self.scalar_static.f64_values[41])!=0.0));
        self.scalar_static.bool_values[76]=(!((self.scalar_static.f64_values[41])!=0.0));
        self.scalar_static.bool_values[77]=(((self.scalar_static.f64_values[39])!=0.0)&&self.scalar_static.bool_values[76]);
        self.scalar_static.f64_values[223]=p[96];
        self.scalar_static.bool_values[78]=(!((self.scalar_static.f64_values[39])!=0.0));
        self.scalar_static.bool_values[79]=(!((self.scalar_static.f64_values[44])!=0.0));
        self.scalar_static.f64_values[224]=p[2];
        self.scalar_static.f64_values[225]=(-self.scalar_static.f64_values[224]);
        self.scalar_static.bool_values[80]=(((self.scalar_static.f64_values[46])!=0.0)&&((self.scalar_static.f64_values[48])!=0.0));
        self.scalar_static.f64_values[226]=(-self.scalar_static.f64_values[180]);
        self.scalar_static.f64_values[227]=f64::powf(self.scalar_static.f64_values[189],self.scalar_static.f64_values[226]);
        self.scalar_static.f64_values[228]=(1.0-self.scalar_static.f64_values[180]);
        self.scalar_static.f64_values[229]=(0.5*self.scalar_static.f64_values[180]);
        self.scalar_static.bool_values[81]=(!((self.scalar_static.f64_values[48])!=0.0));
        self.scalar_static.bool_values[82]=(((self.scalar_static.f64_values[46])!=0.0)&&self.scalar_static.bool_values[81]);
        self.scalar_static.f64_values[230]=(self.scalar_static.f64_values[47]*4.0);
        self.scalar_static.f64_values[231]=(self.scalar_static.f64_values[47]*self.scalar_static.f64_values[230]);
        self.scalar_static.bool_values[83]=(!((self.scalar_static.f64_values[46])!=0.0));
        self.scalar_static.f64_values[232]=p[76];
        self.scalar_static.f64_values[233]=p[77];
        self.scalar_static.f64_values[234]=p[78];
        self.scalar_static.f64_values[235]=p[81];
        self.scalar_static.f64_values[236]=p[47];
        self.scalar_static.f64_values[237]=p[53];
        self.scalar_static.f64_values[238]=p[35];
        self.scalar_static.f64_values[239]=p[40];
        self.scalar_static.f64_values[240]=p[102];
        self.scalar_static.f64_values[241]=p[82];
        self.scalar_static.f64_values[242]=(self.scalar_static.f64_values[139]-1.0);
        self.scalar_static.f64_values[243]=(self.scalar_static.f64_values[141]-1.0);
        self.scalar_static.f64_values[244]=(self.scalar_static.f64_values[142]-1.0);
        self.scalar_static.f64_values[245]=(self.scalar_static.f64_values[144]-1.0);
        self.scalar_static.f64_values[246]=(self.scalar_static.f64_values[146]-1.0);
        self.scalar_static.f64_values[247]=(self.scalar_static.f64_values[147]-1.0);
        self.scalar_static.f64_values[248]=(self.scalar_static.f64_values[149]-1.0);
        self.scalar_static.f64_values[249]=(self.scalar_static.f64_values[151]-1.0);
        self.scalar_static.f64_values[250]=(self.scalar_static.f64_values[153]-1.0);
        self.scalar_static.f64_values[251]=(self.scalar_static.f64_values[155]-1.0);
        self.scalar_static.f64_values[252]=(self.scalar_static.f64_values[89]-1.0);
        self.scalar_static.f64_values[253]=(self.scalar_static.f64_values[100]-1.0);
        self.scalar_static.f64_values[254]=(self.scalar_static.f64_values[106]-1.0);
        self.scalar_static.f64_values[255]=(self.scalar_static.f64_values[112]-1.0);
        self.scalar_static.f64_values[256]=(self.scalar_static.f64_values[118]-1.0);
        self.scalar_static.f64_values[257]=(self.scalar_static.f64_values[123]-1.0);
        self.scalar_static.f64_values[258]=(self.scalar_static.f64_values[128]-1.0);
        self.scalar_static.f64_values[259]=(self.scalar_static.f64_values[132]-1.0);
        self.scalar_static.f64_values[260]=(self.scalar_static.f64_values[136]-1.0);
        self.scalar_static.f64_values[261]=(self.scalar_static.f64_values[176]-1.0);
        self.scalar_static.f64_values[262]=(self.scalar_static.f64_values[180]-1.0);
        self.scalar_static.f64_values[263]=(self.scalar_static.f64_values[87]-1.0);
        self.scalar_static.f64_values[264]=(self.scalar_static.f64_values[183]-1.0);
        self.scalar_static.f64_values[265]=(-self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[266]=(if ((self.scalar_static.f64_values[13])!=0.0){self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[267]=(if ((self.scalar_static.f64_values[13])!=0.0){self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[268]=(self.scalar_static.f64_values[193]*self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[269]=(self.scalar_static.f64_values[193]*self.scalar_static.f64_values[267]);
        self.scalar_static.f64_values[270]=(self.scalar_static.f64_values[192]-1.0);
        self.scalar_static.f64_values[271]=(if self.scalar_static.bool_values[47]{self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[272]=(if self.scalar_static.bool_values[47]{self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[273]=(if ((self.scalar_static.f64_values[15])!=0.0){self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[274]=(if ((self.scalar_static.f64_values[15])!=0.0){self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[275]=(self.scalar_static.f64_values[199]*self.scalar_static.f64_values[273]);
        self.scalar_static.f64_values[276]=(self.scalar_static.f64_values[199]*self.scalar_static.f64_values[274]);
        self.scalar_static.f64_values[277]=(self.scalar_static.f64_values[198]-1.0);
        self.scalar_static.f64_values[278]=(self.scalar_static.f64_values[198]*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[279]=(self.scalar_static.f64_values[60]*self.scalar_static.f64_values[198]);
        self.scalar_static.f64_values[280]=(2.0*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[281]=(self.scalar_static.f64_values[60]*2.0);
        self.scalar_static.f64_values[282]=(self.scalar_static.f64_values[205]-1.0);
        self.scalar_static.f64_values[283]=(if self.scalar_static.bool_values[51]{self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[284]=(if self.scalar_static.bool_values[51]{self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[285]=(self.scalar_static.f64_values[207]-1.0);
        self.scalar_static.f64_values[286]=(self.scalar_static.f64_values[93]-1.0);
        self.scalar_static.f64_values[287]=(if self.scalar_static.bool_values[58]{self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[288]=(if self.scalar_static.bool_values[58]{self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[289]=(if self.scalar_static.bool_values[64]{self.scalar_static.f64_values[265]}else{self.scalar_static.f64_values[287]});
        self.scalar_static.f64_values[290]=(if self.scalar_static.bool_values[64]{self.scalar_static.f64_values[60]}else{self.scalar_static.f64_values[288]});
        self.scalar_static.f64_values[291]=(if self.scalar_static.bool_values[71]{self.scalar_static.f64_values[265]}else{self.scalar_static.f64_values[289]});
        self.scalar_static.f64_values[292]=(if self.scalar_static.bool_values[71]{self.scalar_static.f64_values[60]}else{self.scalar_static.f64_values[290]});
        self.scalar_static.f64_values[293]=(if self.scalar_static.bool_values[71]{self.scalar_static.f64_values[265]}else{self.scalar_static.f64_values[291]});
        self.scalar_static.f64_values[294]=(if self.scalar_static.bool_values[71]{self.scalar_static.f64_values[60]}else{self.scalar_static.f64_values[292]});
        self.scalar_static.f64_values[295]=(self.scalar_static.f64_values[217]-1.0);
        self.scalar_static.f64_values[296]=(self.scalar_static.f64_values[218]-1.0);
        self.scalar_static.f64_values[297]=(self.scalar_static.f64_values[221]-1.0);
        self.scalar_static.f64_values[298]=(self.scalar_static.f64_values[222]-1.0);
        self.scalar_static.f64_values[299]=(self.scalar_static.f64_values[265]/self.scalar_static.f64_values[40]);
        self.scalar_static.f64_values[300]=(self.scalar_static.f64_values[60]/self.scalar_static.f64_values[40]);
        self.scalar_static.f64_values[301]=(-self.scalar_static.f64_values[299]);
        self.scalar_static.f64_values[302]=(-self.scalar_static.f64_values[300]);
        self.scalar_static.f64_values[303]=(if self.scalar_static.bool_values[75]{self.scalar_static.f64_values[301]}else{0.0});
        self.scalar_static.f64_values[304]=(if self.scalar_static.bool_values[75]{self.scalar_static.f64_values[302]}else{0.0});
        self.scalar_static.f64_values[305]=(self.scalar_static.f64_values[223]-1.0);
        self.scalar_static.f64_values[306]=(if self.scalar_static.bool_values[80]{self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[307]=(if self.scalar_static.bool_values[80]{self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[308]=(self.scalar_static.f64_values[229]*self.scalar_static.f64_values[306]);
        self.scalar_static.f64_values[309]=(self.scalar_static.f64_values[229]*self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[310]=(self.scalar_static.f64_values[228]-1.0);
        self.scalar_static.f64_values[311]=(if self.scalar_static.bool_values[82]{self.scalar_static.f64_values[265]}else{0.0});
        self.scalar_static.f64_values[312]=(if self.scalar_static.bool_values[82]{self.scalar_static.f64_values[60]}else{0.0});
        self.scalar_static.f64_values[313]=(self.scalar_static.f64_values[72]*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[314]=(self.scalar_static.f64_values[60]*self.scalar_static.f64_values[72]);
        self.scalar_static.f64_values[315]=(self.scalar_static.f64_values[313]/1.44);
        self.scalar_static.f64_values[316]=(self.scalar_static.f64_values[314]/1.44);
        self.scalar_static.f64_values[317]=(self.scalar_static.f64_values[215]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[318]=(self.scalar_static.f64_values[215]*self.scalar_static.f64_values[316]);
        self.scalar_static.f64_values[319]=(self.scalar_static.f64_values[237]*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[320]=(self.scalar_static.f64_values[60]*self.scalar_static.f64_values[237]);
        self.scalar_static.f64_values[321]=(-self.scalar_static.f64_values[238]);
        self.scalar_static.f64_values[322]=(-self.scalar_static.f64_values[239]);
        self.scalar_static.f64_values[323]=(self.scalar_static.f64_values[241]*0.3333333333333333);
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
        self.scalar_static.f64_values[324]=(temperature+self.scalar_static.f64_values[79]);
        self.scalar_static.f64_values[325]=(self.scalar_static.f64_values[324]-273.15);
        self.scalar_static.bool_values[84]=(self.scalar_static.f64_values[325]<self.scalar_static.f64_values[81]);
        self.scalar_static.f64_values[326]=(if self.scalar_static.bool_values[84]{1.0}else{0.0});
        self.scalar_static.f64_values[327]=(self.scalar_static.f64_values[325]-self.scalar_static.f64_values[80]);
        self.scalar_static.f64_values[328]=(self.scalar_static.f64_values[327]-1.0);
        self.scalar_static.f64_values[329]=(self.scalar_static.f64_values[328]).exp();
        self.scalar_static.f64_values[330]=(self.scalar_static.f64_values[80]+self.scalar_static.f64_values[329]);
        self.scalar_static.f64_values[331]=(if ((self.scalar_static.f64_values[326])!=0.0){self.scalar_static.f64_values[330]}else{self.scalar_static.f64_values[325]});
        self.scalar_static.bool_values[85]=(self.scalar_static.f64_values[331]>self.scalar_static.f64_values[83]);
        self.scalar_static.f64_values[332]=(if self.scalar_static.bool_values[85]{1.0}else{0.0});
        self.scalar_static.bool_values[86]=(!((self.scalar_static.f64_values[326])!=0.0));
        self.scalar_static.bool_values[87]=(((self.scalar_static.f64_values[332])!=0.0)&&self.scalar_static.bool_values[86]);
        self.scalar_static.f64_values[333]=(self.scalar_static.f64_values[82]-self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[333]-1.0);
        self.scalar_static.f64_values[335]=(self.scalar_static.f64_values[334]).exp();
        self.scalar_static.f64_values[336]=(self.scalar_static.f64_values[82]-self.scalar_static.f64_values[335]);
        self.scalar_static.f64_values[337]=(if self.scalar_static.bool_values[87]{self.scalar_static.f64_values[336]}else{self.scalar_static.f64_values[331]});
        self.scalar_static.f64_values[338]=(273.15+self.scalar_static.f64_values[337]);
        self.scalar_static.f64_values[339]=(self.scalar_static.f64_values[338]*1.380662e-23);
        self.scalar_static.f64_values[340]=(self.scalar_static.f64_values[339]/1.602189e-19);
        self.scalar_static.f64_values[341]=(self.scalar_static.f64_values[338]/self.scalar_static.f64_values[78]);
        self.scalar_static.f64_values[342]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[85]);
        self.scalar_static.f64_values[343]=(self.scalar_static.f64_values[86]/self.scalar_static.f64_values[342]);
        self.scalar_static.f64_values[344]=(self.scalar_static.f64_values[343]).exp();
        self.scalar_static.f64_values[345]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[89]);
        self.scalar_static.f64_values[346]=(self.scalar_static.f64_values[84]*self.scalar_static.f64_values[345]);
        self.scalar_static.f64_values[347]=(1.0-self.scalar_static.f64_values[341]);
        self.scalar_static.f64_values[348]=(self.scalar_static.f64_values[91]*self.scalar_static.f64_values[347]);
        self.scalar_static.f64_values[349]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[350]=(self.scalar_static.f64_values[348]/self.scalar_static.f64_values[349]);
        self.scalar_static.f64_values[351]=(self.scalar_static.f64_values[350]).exp();
        self.scalar_static.f64_values[352]=(self.scalar_static.f64_values[346]*self.scalar_static.f64_values[351]);
        self.scalar_static.bool_values[88]=(self.scalar_static.f64_values[352]>0.0);
        self.scalar_static.f64_values[353]=(if self.scalar_static.bool_values[88]{1.0}else{0.0});
        self.scalar_static.bool_values[89]=(!((self.scalar_static.f64_values[353])!=0.0));
        self.scalar_static.f64_values[354]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[355]=(self.scalar_static.f64_values[97]*self.scalar_static.f64_values[354]);
        self.scalar_static.f64_values[356]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[102]);
        self.scalar_static.f64_values[357]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[358]=(self.scalar_static.f64_values[356]/self.scalar_static.f64_values[357]);
        self.scalar_static.f64_values[359]=(self.scalar_static.f64_values[358]).exp();
        self.scalar_static.f64_values[360]=(self.scalar_static.f64_values[355]*self.scalar_static.f64_values[359]);
        self.scalar_static.bool_values[90]=(self.scalar_static.f64_values[360]>0.0);
        self.scalar_static.bool_values[91]=(self.scalar_static.bool_values[88]&&self.scalar_static.bool_values[90]);
        self.scalar_static.f64_values[361]=(if self.scalar_static.bool_values[91]{1.0}else{0.0});
        self.scalar_static.f64_values[362]=(self.scalar_static.f64_values[352]*self.scalar_static.f64_values[360]);
        self.scalar_static.bool_values[92]=(!((self.scalar_static.f64_values[361])!=0.0));
        self.scalar_static.f64_values[363]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[364]=(self.scalar_static.f64_values[21]*self.scalar_static.f64_values[363]);
        self.scalar_static.f64_values[365]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[108]);
        self.scalar_static.f64_values[366]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[367]=(self.scalar_static.f64_values[365]/self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[368]=(self.scalar_static.f64_values[367]).exp();
        self.scalar_static.f64_values[369]=(self.scalar_static.f64_values[364]*self.scalar_static.f64_values[368]);
        self.scalar_static.bool_values[93]=(self.scalar_static.f64_values[369]>0.0);
        self.scalar_static.f64_values[370]=(if self.scalar_static.bool_values[93]{1.0}else{0.0});
        self.scalar_static.bool_values[94]=(!((self.scalar_static.f64_values[370])!=0.0));
        self.scalar_static.f64_values[371]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[112]);
        self.scalar_static.f64_values[372]=(self.scalar_static.f64_values[109]*self.scalar_static.f64_values[371]);
        self.scalar_static.f64_values[373]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[114]);
        self.scalar_static.f64_values[374]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[111]);
        self.scalar_static.f64_values[375]=(self.scalar_static.f64_values[373]/self.scalar_static.f64_values[374]);
        self.scalar_static.f64_values[376]=(self.scalar_static.f64_values[375]).exp();
        self.scalar_static.f64_values[377]=(self.scalar_static.f64_values[372]*self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[95]=(self.scalar_static.f64_values[377]>0.0);
        self.scalar_static.f64_values[378]=(if self.scalar_static.bool_values[95]{1.0}else{0.0});
        self.scalar_static.bool_values[96]=(!((self.scalar_static.f64_values[378])!=0.0));
        self.scalar_static.f64_values[379]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[118]);
        self.scalar_static.f64_values[380]=(self.scalar_static.f64_values[115]*self.scalar_static.f64_values[379]);
        self.scalar_static.f64_values[381]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[120]);
        self.scalar_static.f64_values[382]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[117]);
        self.scalar_static.f64_values[383]=(self.scalar_static.f64_values[381]/self.scalar_static.f64_values[382]);
        self.scalar_static.f64_values[384]=(self.scalar_static.f64_values[383]).exp();
        self.scalar_static.f64_values[385]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[384]);
        self.scalar_static.bool_values[97]=(self.scalar_static.f64_values[385]>0.0);
        self.scalar_static.f64_values[386]=(if self.scalar_static.bool_values[97]{1.0}else{0.0});
        self.scalar_static.bool_values[98]=(!((self.scalar_static.f64_values[386])!=0.0));
        self.scalar_static.f64_values[387]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[123]);
        self.scalar_static.f64_values[388]=(self.scalar_static.f64_values[121]*self.scalar_static.f64_values[387]);
        self.scalar_static.f64_values[389]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[125]);
        self.scalar_static.f64_values[390]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[122]);
        self.scalar_static.f64_values[391]=(self.scalar_static.f64_values[389]/self.scalar_static.f64_values[390]);
        self.scalar_static.f64_values[392]=(self.scalar_static.f64_values[391]).exp();
        self.scalar_static.f64_values[393]=(self.scalar_static.f64_values[388]*self.scalar_static.f64_values[392]);
        self.scalar_static.bool_values[99]=(self.scalar_static.f64_values[393]>0.0);
        self.scalar_static.f64_values[394]=(if self.scalar_static.bool_values[99]{1.0}else{0.0});
        self.scalar_static.bool_values[100]=(!((self.scalar_static.f64_values[394])!=0.0));
        self.scalar_static.f64_values[395]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[396]=(self.scalar_static.f64_values[126]*self.scalar_static.f64_values[395]);
        self.scalar_static.f64_values[397]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[130]);
        self.scalar_static.f64_values[398]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[127]);
        self.scalar_static.f64_values[399]=(self.scalar_static.f64_values[397]/self.scalar_static.f64_values[398]);
        self.scalar_static.f64_values[400]=(self.scalar_static.f64_values[399]).exp();
        self.scalar_static.f64_values[401]=(self.scalar_static.f64_values[396]*self.scalar_static.f64_values[400]);
        self.scalar_static.bool_values[101]=(self.scalar_static.f64_values[401]>0.0);
        self.scalar_static.f64_values[402]=(if self.scalar_static.bool_values[101]{1.0}else{0.0});
        self.scalar_static.bool_values[102]=(!((self.scalar_static.f64_values[402])!=0.0));
        self.scalar_static.f64_values[403]=(self.scalar_static.f64_values[30]*self.scalar_static.f64_values[387]);
        self.scalar_static.f64_values[404]=(self.scalar_static.f64_values[392]*self.scalar_static.f64_values[403]);
        self.scalar_static.bool_values[103]=(self.scalar_static.f64_values[404]>0.0);
        self.scalar_static.f64_values[405]=(if self.scalar_static.bool_values[103]{1.0}else{0.0});
        self.scalar_static.bool_values[104]=(!((self.scalar_static.f64_values[405])!=0.0));
        self.scalar_static.f64_values[406]=(self.scalar_static.f64_values[31]*self.scalar_static.f64_values[395]);
        self.scalar_static.f64_values[407]=(self.scalar_static.f64_values[400]*self.scalar_static.f64_values[406]);
        self.scalar_static.bool_values[105]=(self.scalar_static.f64_values[407]>0.0);
        self.scalar_static.f64_values[408]=(if self.scalar_static.bool_values[105]{1.0}else{0.0});
        self.scalar_static.bool_values[106]=(!((self.scalar_static.f64_values[408])!=0.0));
        self.scalar_static.f64_values[409]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[132]);
        self.scalar_static.f64_values[410]=(self.scalar_static.f64_values[42]*self.scalar_static.f64_values[409]);
        self.scalar_static.f64_values[411]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[412]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[413]=(self.scalar_static.f64_values[411]/self.scalar_static.f64_values[412]);
        self.scalar_static.f64_values[414]=(self.scalar_static.f64_values[413]).exp();
        self.scalar_static.f64_values[415]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[414]);
        self.scalar_static.bool_values[107]=(self.scalar_static.f64_values[415]>0.0);
        self.scalar_static.f64_values[416]=(if self.scalar_static.bool_values[107]{1.0}else{0.0});
        self.scalar_static.bool_values[108]=(!((self.scalar_static.f64_values[416])!=0.0));
        self.scalar_static.f64_values[417]=f64::powf(self.scalar_static.f64_values[341],self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[418]=(self.scalar_static.f64_values[43]*self.scalar_static.f64_values[417]);
        self.scalar_static.f64_values[419]=(self.scalar_static.f64_values[347]*self.scalar_static.f64_values[138]);
        self.scalar_static.f64_values[420]=(self.scalar_static.f64_values[340]*self.scalar_static.f64_values[135]);
        self.scalar_static.f64_values[421]=(self.scalar_static.f64_values[419]/self.scalar_static.f64_values[420]);
        self.scalar_static.f64_values[422]=(self.scalar_static.f64_values[421]).exp();
        self.scalar_static.f64_values[423]=(self.scalar_static.f64_values[418]*self.scalar_static.f64_values[422]);
        self.scalar_static.bool_values[109]=(self.scalar_static.f64_values[423]>0.0);
        self.scalar_static.f64_values[424]=(if self.scalar_static.bool_values[109]{1.0}else{0.0});
        self.scalar_static.bool_values[110]=(!((self.scalar_static.f64_values[424])!=0.0));
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
