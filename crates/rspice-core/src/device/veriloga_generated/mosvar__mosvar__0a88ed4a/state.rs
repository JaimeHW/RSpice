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
    pub p64: f64, pub p65: f64, pub p66: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 67] = [
                1e-6, 1e-6, 1.0, 0.0, 1.4, 0.0, 0.0, 1000.0,
                -100.0, 500.0, 10000.0, 21.0, 1e-8, 9900000000.0, 1e-8, 9900000000.0,
                1.0, -1.0, -1.0, 2e-9, 3.9, 1.0, 0.1, 0.0,
                3e23, 1.0, 0.0, 0.0, 0.1, 1e27, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0001, 1000.0,
                0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 3.1, 4.5, 2.0, 0.0, 5e25, 0.0,
                0.0, 0.0, 0.375, 0.063, 0.0, 0.0, 0.0, 0.375,
                0.063, 1e-5, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 67);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 69] = [
    ("w", 0), ("l", 1), ("ngcon", 2), ("dta", 3), ("dtemp", 3), ("version", 4), ("subversion", 5), ("revision", 6), ("level", 7), ("tmin", 8), ("tmax", 9), ("vmax", 10), ("tr", 11), ("tref", 11), ("lmin", 12), ("lmax", 13),
    ("wmin", 14), ("wmax", 15), ("swres", 16), ("type", 17), ("typep", 18), ("toxo", 19), ("epsroxo", 20), ("swqinv", 21), ("tau", 22), ("vfbo", 23), ("nsubo", 24), ("mnsubo", 25), ("dnsubo", 26), ("vnsubo", 27), ("nslpo", 28), ("npo", 29),
    ("qmc", 30), ("dlq", 31), ("dwq", 32), ("dwr", 33), ("cfrl", 34), ("cfrw", 35), ("rshg", 36), ("rpv", 37), ("rend", 38), ("rshs", 39), ("uac", 40), ("uacred", 41), ("stvfb", 42), ("strshg", 43), ("strpv", 44), ("strend", 45),
    ("strshs", 46), ("stuac", 47), ("feta", 48), ("swigate", 49), ("chibo", 50), ("chibpo", 51), ("stig", 52), ("lov", 53), ("novo", 54), ("iginvlw", 55), ("igovw", 56), ("gcoo", 57), ("gc2o", 58), ("gc3o", 59), ("igchvlw", 60), ("igovhvw", 61),
    ("gcohvo", 62), ("gc2hvo", 63), ("gc3hvo", 64), ("igmax", 65), ("racnoise", 66),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 67] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 67] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 67] = [
    "W", "L", "NGCON", "DTA", "VERSION", "SUBVERSION", "REVISION", "LEVEL", "TMIN", "TMAX", "VMAX", "TR", "LMIN", "LMAX", "WMIN", "WMAX",
    "SWRES", "TYPE", "TYPEP", "TOXO", "EPSROXO", "SWQINV", "TAU", "VFBO", "NSUBO", "MNSUBO", "DNSUBO", "VNSUBO", "NSLPO", "NPO", "QMC", "DLQ",
    "DWQ", "DWR", "CFRL", "CFRW", "RSHG", "RPV", "REND", "RSHS", "UAC", "UACRED", "STVFB", "STRSHG", "STRPV", "STREND", "STRSHS", "STUAC",
    "FETA", "SWIGATE", "CHIBO", "CHIBPO", "STIG", "LOV", "NOVO", "IGINVLW", "IGOVW", "GCOO", "GC2O", "GC3O", "IGCHVLW", "IGOVHVW", "GCOHVO", "GC2HVO",
    "GC3HVO", "IGMAX", "RACNOISE",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 67] = [
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

const PARAMETER_INTEGER_FLAGS: [bool; 67] = [
    false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 67] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 21.0, label: "21.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 5e-10, label: "5e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -5.0, label: "-5.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 1e24, label: "1e24" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e22, label: "1e22" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 67] = [
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 21.0, label: "21.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2e-6, label: "2e-6" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None,
    Some(ParameterBound { value: 1e25, label: "1e25" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e27, label: "1e27" }), None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None,
    None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), None, Some(ParameterBound { value: 2.0, label: "2.0" }),
];

const PARAMETER_RANGE_FLAGS: [u8; 67] = [
    3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 3, 3, 3, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 2, 2, 2, 2, 2, 0, 3, 2, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 0, 2, 0, 2, 2, 0, 0, 0, 2, 2, 0, 0,
    0, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 67] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[],
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

pub struct Instance {
    pub nodes: [usize; 7],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 67]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 3]>,
    pub(crate) ddt_state_previous: Box<[f64; 3]>,
    pub(crate) ddt_state_older: Box<[f64; 3]>,
    pub(crate) ddt_state_initialized: Box<[bool; 3]>,
    pub(crate) ddt_derivative_current: Box<[f64; 3]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 3]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 379]>,
    pub(crate) scalar_static_bool: Box<[bool; 64]>,
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
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 7;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["gii", "gi", "ci", "n"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 67;
    pub const VARIABLE_COUNT: usize = 432;
    pub const DDT_STATE_COUNT: usize = 3;
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
            scalar_static_f64: boxed_zero_f64_array::<379>(),
            scalar_static_bool: boxed_zero_bool_array::<64>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'mosvar'", name));
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
        self.scalar_static_f64[0]=p.p20;
        self.scalar_static_f64[1]=(self.scalar_static_f64[0]/3.9);
        self.scalar_static_f64[2]=(3.453e-11*self.scalar_static_f64[1]);
        self.scalar_static_f64[3]=p.p19;
        self.scalar_static_f64[4]=(self.scalar_static_f64[2]/self.scalar_static_f64[3]);
        self.scalar_static_f64[5]=p.p24;
        self.scalar_static_f64[6]=p.p29;
        self.scalar_static_f64[7]=(3.348580862e-29*self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]).sqrt();
        self.scalar_static_f64[9]=(self.scalar_static_f64[8]/self.scalar_static_f64[4]);
        self.scalar_static_f64[10]=p.p54;
        self.scalar_static_f64[11]=(3.348580862e-29*self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=(self.scalar_static_f64[11]).sqrt();
        self.scalar_static_f64[13]=(self.scalar_static_f64[12]/self.scalar_static_f64[4]);
        self.scalar_static_f64[14]=p.p30;
        self.scalar_static_bool[0]=(self.scalar_static_f64[14]>0.0);
        self.scalar_static_f64[15]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[16]=(self.scalar_static_f64[14]*2.3807972);
        self.scalar_static_f64[17]=f64::powf(self.scalar_static_f64[4],0.6666666666666666);
        self.scalar_static_f64[18]=(self.scalar_static_f64[16]*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(if ((self.scalar_static_f64[15])!=0.0){self.scalar_static_f64[18]}else{0.0});
        self.scalar_static_f64[20]=p.p17;
        self.scalar_static_bool[1]=(self.scalar_static_f64[20]<0.0);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_bool[2]=(((self.scalar_static_f64[15])!=0.0)&&((self.scalar_static_f64[21])!=0.0));
        self.scalar_static_f64[22]=(self.scalar_static_f64[19]*1.2514650134837189);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[2]{self.scalar_static_f64[22]}else{self.scalar_static_f64[19]});
        self.scalar_static_bool[3]=(!((self.scalar_static_f64[15])!=0.0));
        self.scalar_static_f64[24]=(if self.scalar_static_bool[3]{0.0}else{self.scalar_static_f64[23]});
        self.scalar_static_f64[25]=p.p48;
        self.scalar_static_f64[26]=(0.3333333333333333*self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=(if ((self.scalar_static_f64[21])!=0.0){self.scalar_static_f64[26]}else{0.0});
        self.scalar_static_bool[4]=(!((self.scalar_static_f64[21])!=0.0));
        self.scalar_static_f64[28]=(self.scalar_static_f64[25]*0.5);
        self.scalar_static_f64[29]=(if self.scalar_static_bool[4]{self.scalar_static_f64[28]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[30]=(self.scalar_static_f64[3]/1e-9);
        self.scalar_static_f64[31]=p.p11;
        self.scalar_static_bool[5]=(self.scalar_static_f64[31]> -273.0);
        self.scalar_static_f64[32]=(if self.scalar_static_bool[5]{self.scalar_static_f64[31]}else{-273.0});
        self.scalar_static_f64[33]=(self.scalar_static_f64[32]+273.15);
        self.scalar_static_f64[34]=p.p3;
        self.scalar_static_f64[35]=p.p23;
        self.scalar_static_f64[36]=p.p42;
        self.scalar_static_f64[37]=p.p43;
        self.scalar_static_f64[38]=p.p36;
        self.scalar_static_f64[39]=p.p44;
        self.scalar_static_f64[40]=p.p37;
        self.scalar_static_f64[41]=p.p45;
        self.scalar_static_f64[42]=p.p38;
        self.scalar_static_f64[43]=p.p46;
        self.scalar_static_f64[44]=p.p39;
        self.scalar_static_f64[45]=p.p47;
        self.scalar_static_f64[46]=p.p40;
        self.scalar_static_f64[47]=p.p1;
        self.scalar_static_f64[48]=p.p0;
        self.scalar_static_f64[49]=p.p31;
        self.scalar_static_f64[50]=(self.scalar_static_f64[47]+self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p32;
        self.scalar_static_f64[52]=(self.scalar_static_f64[48]+self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=p.p35;
        self.scalar_static_f64[54]=(self.scalar_static_f64[48]*self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=p.p34;
        self.scalar_static_f64[56]=(self.scalar_static_f64[47]*self.scalar_static_f64[55]);
        self.scalar_static_f64[57]=(self.scalar_static_f64[54]+self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=(2.0*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=p.p16;
        self.scalar_static_f64[60]=p.p2;
        self.scalar_static_f64[61]=(self.scalar_static_f64[60]-1.0);
        self.scalar_static_f64[62]=(self.scalar_static_f64[61]*9.0);
        self.scalar_static_f64[63]=(3.0+self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=(self.scalar_static_f64[47]*self.scalar_static_f64[63]);
        self.scalar_static_f64[65]=(self.scalar_static_f64[47]*self.scalar_static_f64[48]);
        self.scalar_static_f64[66]=p.p33;
        self.scalar_static_f64[67]=(self.scalar_static_f64[48]+self.scalar_static_f64[66]);
        self.scalar_static_f64[68]=(2.0*self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(self.scalar_static_f64[67]*12.0);
        self.scalar_static_bool[6]=(!((self.scalar_static_f64[59])!=0.0));
        self.scalar_static_f64[70]=p.p49;
        self.scalar_static_f64[71]=p.p55;
        self.scalar_static_f64[72]=(self.scalar_static_f64[52]*self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=(self.scalar_static_f64[50]*self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(self.scalar_static_f64[73]*1000000000000.0);
        self.scalar_static_f64[75]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[74]}else{0.0});
        self.scalar_static_f64[76]=p.p56;
        self.scalar_static_f64[77]=(2.0*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=p.p53;
        self.scalar_static_f64[79]=(self.scalar_static_f64[77]*self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(self.scalar_static_f64[52]*self.scalar_static_f64[79]);
        self.scalar_static_f64[81]=(1000000000000.0*self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[81]}else{0.0});
        self.scalar_static_f64[83]=p.p60;
        self.scalar_static_f64[84]=(self.scalar_static_f64[52]*self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(self.scalar_static_f64[50]*self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=(1000000000000.0*self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[86]}else{0.0});
        self.scalar_static_f64[88]=p.p61;
        self.scalar_static_f64[89]=(2.0*self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[78]*self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=(self.scalar_static_f64[52]*self.scalar_static_f64[90]);
        self.scalar_static_f64[92]=(1000000000000.0*self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[92]}else{0.0});
        self.scalar_static_f64[94]=p.p52;
        self.scalar_static_f64[95]=p.p50;
        self.scalar_static_f64[96]=(1.0/self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[96]}else{0.0});
        self.scalar_static_f64[98]=p.p51;
        self.scalar_static_f64[99]=(1.0/self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[99]}else{0.0});
        self.scalar_static_f64[101]=(self.scalar_static_f64[95]*2.918995620956536e-49);
        self.scalar_static_f64[102]=(self.scalar_static_f64[101]).sqrt();
        self.scalar_static_f64[103]=(1.3333333333333333*self.scalar_static_f64[102]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[103]/1.05457168e-34);
        self.scalar_static_f64[105]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[104]}else{0.0});
        self.scalar_static_f64[106]=(self.scalar_static_f64[3]*self.scalar_static_f64[105]);
        self.scalar_static_f64[107]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[106]}else{0.0});
        self.scalar_static_f64[108]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[107]}else{0.0});
        self.scalar_static_f64[109]=(self.scalar_static_f64[98]*2.918995620956536e-49);
        self.scalar_static_f64[110]=(self.scalar_static_f64[109]).sqrt();
        self.scalar_static_f64[111]=(1.3333333333333333*self.scalar_static_f64[110]);
        self.scalar_static_f64[112]=(self.scalar_static_f64[111]/1.05457168e-34);
        self.scalar_static_f64[113]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[112]}else{self.scalar_static_f64[105]});
        self.scalar_static_f64[114]=(self.scalar_static_f64[3]*self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[114]}else{0.0});
        self.scalar_static_f64[116]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[115]}else{0.0});
        self.scalar_static_f64[117]=p.p59;
        self.scalar_static_bool[7]=(self.scalar_static_f64[117]<0.0);
        self.scalar_static_f64[118]=(if self.scalar_static_bool[7]{1.0}else{0.0});
        self.scalar_static_bool[8]=(((self.scalar_static_f64[70])!=0.0)&&((self.scalar_static_f64[118])!=0.0));
        self.scalar_static_f64[119]=p.p58;
        self.scalar_static_f64[120]=(-0.495*self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(self.scalar_static_f64[120]/self.scalar_static_f64[117]);
        self.scalar_static_f64[122]=(if self.scalar_static_bool[8]{self.scalar_static_f64[121]}else{0.0});
        self.scalar_static_bool[9]=(!((self.scalar_static_f64[118])!=0.0));
        self.scalar_static_bool[10]=(((self.scalar_static_f64[70])!=0.0)&&self.scalar_static_bool[9]);
        self.scalar_static_f64[123]=(if self.scalar_static_bool[10]{0.0}else{self.scalar_static_f64[122]});
        self.scalar_static_f64[124]=p.p64;
        self.scalar_static_bool[11]=(self.scalar_static_f64[124]<0.0);
        self.scalar_static_f64[125]=(if self.scalar_static_bool[11]{1.0}else{0.0});
        self.scalar_static_bool[12]=(((self.scalar_static_f64[70])!=0.0)&&((self.scalar_static_f64[125])!=0.0));
        self.scalar_static_f64[126]=p.p63;
        self.scalar_static_f64[127]=(-0.495*self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[127]/self.scalar_static_f64[124]);
        self.scalar_static_f64[129]=(if self.scalar_static_bool[12]{self.scalar_static_f64[128]}else{0.0});
        self.scalar_static_bool[13]=(!((self.scalar_static_f64[125])!=0.0));
        self.scalar_static_bool[14]=(((self.scalar_static_f64[70])!=0.0)&&self.scalar_static_bool[13]);
        self.scalar_static_f64[130]=(if self.scalar_static_bool[14]{0.0}else{self.scalar_static_f64[129]});
        self.scalar_static_f64[131]=p.p57;
        self.scalar_static_f64[132]=p.p62;
        self.scalar_static_bool[15]=(!((self.scalar_static_f64[70])!=0.0));
        self.scalar_static_f64[133]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[123]});
        self.scalar_static_f64[134]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[130]});
        self.scalar_static_f64[135]=(if self.scalar_static_bool[15]{0.1}else{self.scalar_static_f64[97]});
        self.scalar_static_f64[136]=(if self.scalar_static_bool[15]{0.1}else{self.scalar_static_f64[100]});
        self.scalar_static_f64[137]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[107]});
        self.scalar_static_f64[138]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[108]});
        self.scalar_static_f64[139]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[115]});
        self.scalar_static_f64[140]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[116]});
        self.scalar_static_f64[141]=p.p26;
        self.scalar_static_f64[142]=p.p27;
        self.scalar_static_f64[143]=p.p28;
        self.scalar_static_f64[144]=(0.5*self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=(self.scalar_static_f64[143]+1e-32);
        self.scalar_static_f64[146]=(self.scalar_static_f64[145]).sqrt();
        self.scalar_static_f64[147]=p.p25;
        self.scalar_static_f64[148]=(self.scalar_static_f64[24]*0.75);
        self.scalar_static_bool[16]=(self.scalar_static_f64[6]<1e27);
        self.scalar_static_f64[149]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_f64[150]=(-self.scalar_static_f64[20]);
        self.scalar_static_f64[151]=p.p18;
        self.scalar_static_f64[152]=(self.scalar_static_f64[150]*self.scalar_static_f64[151]);
        self.scalar_static_bool[17]=(!((self.scalar_static_f64[149])!=0.0));
        self.scalar_static_f64[153]=p.p21;
        self.scalar_static_bool[18]=(self.scalar_static_f64[153]<1.0);
        self.scalar_static_f64[154]=(self.scalar_static_f64[30]*0.37);
        self.scalar_static_f64[155]=(1.0+self.scalar_static_f64[154]);
        self.scalar_static_bool[19]=(self.scalar_static_f64[24]>0.0);
        self.scalar_static_f64[156]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_f64[157]=p.p41;
        self.scalar_static_f64[158]=(self.scalar_static_f64[20]*self.scalar_static_f64[151]);
        self.scalar_static_bool[20]=(-1.0==self.scalar_static_f64[158]);
        self.scalar_static_f64[159]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_bool[21]=(0.0!=self.scalar_static_f64[70]);
        self.scalar_static_bool[22]=(1.0==self.scalar_static_f64[151]);
        self.scalar_static_f64[160]=p.p22;
        self.scalar_static_f64[161]=(self.scalar_static_f64[20]*0.5);
        self.scalar_static_f64[162]=(0.5*self.scalar_static_f64[150]);
        self.scalar_static_f64[163]=(-self.scalar_static_f64[58]);
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
        self.scalar_static_f64[164]=(temperature+self.scalar_static_f64[34]);
        self.scalar_static_f64[165]=(self.scalar_static_f64[164]-273.15);
        self.scalar_static_f64[166]=(273.15+self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=(self.scalar_static_f64[166]*self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=(self.scalar_static_f64[166]-self.scalar_static_f64[33]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[166]/self.scalar_static_f64[33]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[33]/self.scalar_static_f64[166]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[166]*1.3806505e-23);
        self.scalar_static_f64[172]=(self.scalar_static_f64[171]/1.6021918e-19);
        self.scalar_static_f64[173]=(self.scalar_static_f64[172]*100.0);
        self.scalar_static_f64[174]=(self.scalar_static_f64[172]*self.scalar_static_f64[173]);
        self.scalar_static_f64[175]=(1.0/self.scalar_static_f64[172]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[168]*self.scalar_static_f64[36]);
        self.scalar_static_f64[177]=(self.scalar_static_f64[35]+self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=f64::powf(self.scalar_static_f64[170],self.scalar_static_f64[37]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[178]*self.scalar_static_f64[38]);
        self.scalar_static_f64[180]=f64::powf(self.scalar_static_f64[170],self.scalar_static_f64[39]);
        self.scalar_static_f64[181]=(self.scalar_static_f64[180]*self.scalar_static_f64[40]);
        self.scalar_static_f64[182]=f64::powf(self.scalar_static_f64[170],self.scalar_static_f64[41]);
        self.scalar_static_f64[183]=(self.scalar_static_f64[182]*self.scalar_static_f64[42]);
        self.scalar_static_f64[184]=f64::powf(self.scalar_static_f64[170],self.scalar_static_f64[43]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[184]*self.scalar_static_f64[44]);
        self.scalar_static_f64[186]=f64::powf(self.scalar_static_f64[169],self.scalar_static_f64[45]);
        self.scalar_static_f64[187]=(self.scalar_static_f64[186]*self.scalar_static_f64[46]);
        self.scalar_static_f64[188]=(self.scalar_static_f64[166]*3.05e-7);
        self.scalar_static_f64[189]=(9.025e-5+self.scalar_static_f64[188]);
        self.scalar_static_f64[190]=(self.scalar_static_f64[166]*self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(1.179-self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[166]*0.00045);
        self.scalar_static_f64[193]=(1.045+self.scalar_static_f64[192]);
        self.scalar_static_f64[194]=(self.scalar_static_f64[166]*0.0014);
        self.scalar_static_f64[195]=(0.523+self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[167]*1.48e-6);
        self.scalar_static_f64[197]=(self.scalar_static_f64[195]-self.scalar_static_f64[196]);
        self.scalar_static_f64[198]=(self.scalar_static_f64[193]*self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=(self.scalar_static_f64[167]*self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=(self.scalar_static_f64[199]/90000.0);
        self.scalar_static_bool[23]=(self.scalar_static_f64[200]>0.001);
        self.scalar_static_f64[201]=(if self.scalar_static_bool[23]{self.scalar_static_f64[200]}else{0.001});
        self.scalar_static_f64[202]=(self.scalar_static_f64[201]).sqrt();
        self.scalar_static_f64[203]=(self.scalar_static_f64[202]).sqrt();
        self.scalar_static_f64[204]=(self.scalar_static_f64[202]*2.5e25);
        self.scalar_static_f64[205]=(self.scalar_static_f64[203]*self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(1.0/self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=(2.0*self.scalar_static_f64[172]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[5]*self.scalar_static_f64[206]);
        self.scalar_static_f64[209]=(self.scalar_static_f64[208]).ln();
        self.scalar_static_f64[210]=(self.scalar_static_f64[207]*self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[191]+self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(self.scalar_static_f64[6]*self.scalar_static_f64[206]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[212]).ln();
        self.scalar_static_f64[214]=(self.scalar_static_f64[207]*self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=(self.scalar_static_f64[191]+self.scalar_static_f64[214]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[172]*6.0);
        self.scalar_static_f64[217]=(self.scalar_static_f64[191]+self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=(self.scalar_static_f64[175]).sqrt();
        self.scalar_static_f64[219]=(self.scalar_static_f64[9]*self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=(self.scalar_static_f64[219]*self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=(1.0/self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[219]*0.7071067811865475);
        self.scalar_static_f64[223]=(1.0+self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=(1.0/self.scalar_static_f64[223]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[223]*1e-5);
        self.scalar_static_f64[226]=(self.scalar_static_f64[175]*self.scalar_static_f64[215]);
        self.scalar_static_f64[227]=(self.scalar_static_f64[13]*self.scalar_static_f64[218]);
        self.scalar_static_f64[228]=(self.scalar_static_f64[227]*self.scalar_static_f64[227]);
        self.scalar_static_f64[229]=(0.7071067811865475*self.scalar_static_f64[227]);
        self.scalar_static_f64[230]=(1.0+self.scalar_static_f64[229]);
        self.scalar_static_f64[231]=(1e-5*self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(self.scalar_static_f64[227]*0.7324648775608221);
        self.scalar_static_f64[233]=(1.25+self.scalar_static_f64[232]);
        self.scalar_static_bool[24]=(self.scalar_static_f64[226]<460.51701859880916);
        self.scalar_static_f64[234]=(if self.scalar_static_bool[24]{1.0}else{0.0});
        self.scalar_static_f64[235]=(-self.scalar_static_f64[226]);
        self.scalar_static_f64[236]=(self.scalar_static_f64[235]).exp();
        self.scalar_static_f64[237]=(if ((self.scalar_static_f64[234])!=0.0){self.scalar_static_f64[236]}else{0.0});
        self.scalar_static_bool[25]=(!((self.scalar_static_f64[234])!=0.0));
        self.scalar_static_f64[238]=(self.scalar_static_f64[226]-460.51701859880916);
        self.scalar_static_f64[239]=(0.5*self.scalar_static_f64[238]);
        self.scalar_static_f64[240]=(0.3333333333333333*self.scalar_static_f64[238]);
        self.scalar_static_f64[241]=(1.0+self.scalar_static_f64[240]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[239]*self.scalar_static_f64[241]);
        self.scalar_static_f64[243]=(1.0+self.scalar_static_f64[242]);
        self.scalar_static_f64[244]=(self.scalar_static_f64[238]*self.scalar_static_f64[243]);
        self.scalar_static_f64[245]=(1.0+self.scalar_static_f64[244]);
        self.scalar_static_f64[246]=(1e-200/self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=(if self.scalar_static_bool[25]{self.scalar_static_f64[246]}else{self.scalar_static_f64[237]});
        self.scalar_static_f64[248]=(self.scalar_static_f64[179]*self.scalar_static_f64[48]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]/self.scalar_static_f64[64]);
        self.scalar_static_f64[250]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[249]}else{0.0});
        self.scalar_static_f64[251]=(self.scalar_static_f64[181]/self.scalar_static_f64[65]);
        self.scalar_static_f64[252]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[251]}else{0.0});
        self.scalar_static_f64[253]=(self.scalar_static_f64[183]/self.scalar_static_f64[68]);
        self.scalar_static_f64[254]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[253]}else{0.0});
        self.scalar_static_f64[255]=(self.scalar_static_f64[185]*self.scalar_static_f64[47]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[255]/self.scalar_static_f64[69]);
        self.scalar_static_f64[257]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[256]}else{0.0});
        self.scalar_static_bool[26]=(self.scalar_static_f64[250]>0.001);
        self.scalar_static_bool[27]=(self.scalar_static_f64[250]<1000.0);
        self.scalar_static_f64[258]=(if self.scalar_static_bool[27]{self.scalar_static_f64[250]}else{1000.0});
        self.scalar_static_f64[259]=(if self.scalar_static_bool[26]{self.scalar_static_f64[258]}else{0.001});
        self.scalar_static_f64[260]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[259]}else{self.scalar_static_f64[250]});
        self.scalar_static_bool[28]=(self.scalar_static_f64[252]>0.001);
        self.scalar_static_bool[29]=(self.scalar_static_f64[252]<100.0);
        self.scalar_static_f64[261]=(if self.scalar_static_bool[29]{self.scalar_static_f64[252]}else{100.0});
        self.scalar_static_f64[262]=(if self.scalar_static_bool[28]{self.scalar_static_f64[261]}else{0.001});
        self.scalar_static_f64[263]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[262]}else{self.scalar_static_f64[252]});
        self.scalar_static_bool[30]=(self.scalar_static_f64[254]>0.001);
        self.scalar_static_bool[31]=(self.scalar_static_f64[254]<1000.0);
        self.scalar_static_f64[264]=(if self.scalar_static_bool[31]{self.scalar_static_f64[254]}else{1000.0});
        self.scalar_static_f64[265]=(if self.scalar_static_bool[30]{self.scalar_static_f64[264]}else{0.001});
        self.scalar_static_f64[266]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[265]}else{self.scalar_static_f64[254]});
        self.scalar_static_bool[32]=(self.scalar_static_f64[257]>0.001);
        self.scalar_static_bool[33]=(self.scalar_static_f64[257]<1000.0);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[33]{self.scalar_static_f64[257]}else{1000.0});
        self.scalar_static_f64[268]=(if self.scalar_static_bool[32]{self.scalar_static_f64[267]}else{0.001});
        self.scalar_static_f64[269]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[268]}else{self.scalar_static_f64[257]});
        self.scalar_static_bool[34]=(self.scalar_static_f64[187]>0.001);
        self.scalar_static_bool[35]=(self.scalar_static_f64[187]<20.0);
        self.scalar_static_f64[270]=(if self.scalar_static_bool[35]{self.scalar_static_f64[187]}else{20.0});
        self.scalar_static_f64[271]=(if self.scalar_static_bool[34]{self.scalar_static_f64[270]}else{0.001});
        self.scalar_static_f64[272]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[271]}else{self.scalar_static_f64[187]});
        self.scalar_static_f64[273]=(1.0/self.scalar_static_f64[260]);
        self.scalar_static_f64[274]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[275]=(1.0/self.scalar_static_f64[263]);
        self.scalar_static_f64[276]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=(1.0/self.scalar_static_f64[266]);
        self.scalar_static_f64[278]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_f64[279]=(1.0/self.scalar_static_f64[269]);
        self.scalar_static_f64[280]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[279]}else{0.0});
        self.scalar_static_f64[281]=(12.0*self.scalar_static_f64[272]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[48]*self.scalar_static_f64[281]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[282]/self.scalar_static_f64[47]);
        self.scalar_static_f64[284]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[283]}else{0.0});
        self.scalar_static_f64[285]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[274]});
        self.scalar_static_f64[286]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[287]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[278]});
        self.scalar_static_f64[288]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[280]});
        self.scalar_static_f64[289]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[284]});
        self.scalar_static_f64[290]=f64::powf(self.scalar_static_f64[169],self.scalar_static_f64[94]);
        self.scalar_static_f64[291]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[290]}else{0.0});
        self.scalar_static_f64[292]=(self.scalar_static_f64[75]*self.scalar_static_f64[291]);
        self.scalar_static_f64[293]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[292]}else{self.scalar_static_f64[75]});
        self.scalar_static_f64[294]=(self.scalar_static_f64[82]*self.scalar_static_f64[291]);
        self.scalar_static_f64[295]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[294]}else{self.scalar_static_f64[82]});
        self.scalar_static_f64[296]=(self.scalar_static_f64[87]*self.scalar_static_f64[291]);
        self.scalar_static_f64[297]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[296]}else{self.scalar_static_f64[87]});
        self.scalar_static_f64[298]=(self.scalar_static_f64[93]*self.scalar_static_f64[291]);
        self.scalar_static_f64[299]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[298]}else{self.scalar_static_f64[93]});
        self.scalar_static_f64[300]=(self.scalar_static_f64[20]*self.scalar_static_f64[211]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[191]+self.scalar_static_f64[300]);
        self.scalar_static_f64[302]=(0.5*self.scalar_static_f64[301]);
        self.scalar_static_f64[303]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[302]}else{0.0});
        self.scalar_static_f64[304]=(self.scalar_static_f64[20]*self.scalar_static_f64[217]);
        self.scalar_static_f64[305]=(self.scalar_static_f64[191]+self.scalar_static_f64[304]);
        self.scalar_static_f64[306]=(0.5*self.scalar_static_f64[305]);
        self.scalar_static_f64[307]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[306]}else{0.0});
        self.scalar_static_f64[308]=(self.scalar_static_f64[172]*self.scalar_static_f64[131]);
        self.scalar_static_f64[309]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[308]}else{0.0});
        self.scalar_static_f64[310]=(self.scalar_static_f64[172]*self.scalar_static_f64[132]);
        self.scalar_static_f64[311]=(if ((self.scalar_static_f64[70])!=0.0){self.scalar_static_f64[310]}else{0.0});
        self.scalar_static_f64[312]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[293]});
        self.scalar_static_f64[313]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[295]});
        self.scalar_static_f64[314]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[297]});
        self.scalar_static_f64[315]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[299]});
        self.scalar_static_f64[316]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[309]});
        self.scalar_static_f64[317]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[311]});
        self.scalar_static_f64[318]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[303]});
        self.scalar_static_f64[319]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[307]});
        self.scalar_static_f64[320]=(self.scalar_static_f64[224]*self.scalar_static_f64[224]);
        self.scalar_static_f64[321]=(0.1666666666666667*self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=(0.7071067811865475*self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(1.0-self.scalar_static_f64[247]);
        self.scalar_static_f64[324]=(-self.scalar_static_f64[225]);
        self.scalar_static_f64[325]=(self.scalar_static_f64[219]*0.7324648775608221);
        self.scalar_static_f64[326]=(1.25+self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=(1.0/self.scalar_static_f64[326]);
        self.scalar_static_f64[328]=(self.scalar_static_f64[223]*1.25);
        self.scalar_static_f64[329]=(0.5*self.scalar_static_f64[220]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[220]*0.25);
        self.scalar_static_f64[331]=(self.scalar_static_f64[226]+3.0);
        self.scalar_static_f64[332]=(self.scalar_static_f64[226]-230.25850929940458);
        self.scalar_static_f64[333]=(self.scalar_static_f64[170]).sqrt();
        self.scalar_static_f64[334]=(self.scalar_static_f64[191]*self.scalar_static_f64[151]);
        self.scalar_static_f64[335]=(if ((self.scalar_static_f64[159])!=0.0){self.scalar_static_f64[334]}else{0.0});
        self.scalar_static_bool[36]=(self.scalar_static_f64[313]>0.0);
        self.scalar_static_bool[37]=(self.scalar_static_f64[315]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_bool[36]||self.scalar_static_bool[37]);
        self.scalar_static_bool[39]=(self.scalar_static_bool[21]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[336]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_f64[337]=(self.scalar_static_f64[230]*1.25);
        self.scalar_static_f64[338]=(self.scalar_static_f64[337]/self.scalar_static_f64[233]);
        self.scalar_static_f64[339]=(self.scalar_static_f64[338]-1.0);
        self.scalar_static_f64[340]=(self.scalar_static_f64[339]/self.scalar_static_f64[233]);
        self.scalar_static_f64[341]=(0.5*self.scalar_static_f64[228]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[228]*0.25);
        self.scalar_static_bool[40]=(!((self.scalar_static_f64[336])!=0.0));
        self.scalar_static_f64[343]=(if self.scalar_static_bool[38]{1.0}else{0.0});
        self.scalar_static_bool[41]=(((self.scalar_static_f64[70])!=0.0)&&((self.scalar_static_f64[343])!=0.0));
        self.scalar_static_bool[42]=(self.scalar_static_bool[37]&&self.scalar_static_bool[22]);
        self.scalar_static_f64[344]=(if self.scalar_static_bool[42]{1.0}else{0.0});
        self.scalar_static_bool[43]=(self.scalar_static_bool[41]&&((self.scalar_static_f64[344])!=0.0));
        self.scalar_static_bool[44]=(((self.scalar_static_f64[125])!=0.0)&&self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=(((1.0)!=0.0)&&self.scalar_static_bool[43]);
        self.scalar_static_f64[345]=(self.scalar_static_f64[191]-self.scalar_static_f64[319]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[43]&&false);
        self.scalar_static_f64[346]=(self.scalar_static_f64[191]-self.scalar_static_f64[318]);
        self.scalar_static_f64[347]=(if self.scalar_static_bool[36]{1.0}else{0.0});
        self.scalar_static_bool[47]=(self.scalar_static_bool[41]&&((self.scalar_static_f64[347])!=0.0));
        self.scalar_static_bool[48]=(((self.scalar_static_f64[118])!=0.0)&&self.scalar_static_bool[47]);
        self.scalar_static_bool[49]=(((1.0)!=0.0)&&self.scalar_static_bool[47]);
        self.scalar_static_bool[50]=(false&&self.scalar_static_bool[47]);
        self.scalar_static_bool[51]=(self.scalar_static_f64[312]>0.0);
        self.scalar_static_bool[52]=(self.scalar_static_f64[314]>0.0);
        self.scalar_static_bool[53]=(self.scalar_static_bool[51]||self.scalar_static_bool[52]);
        self.scalar_static_f64[348]=(if self.scalar_static_bool[53]{1.0}else{0.0});
        self.scalar_static_bool[54]=(((self.scalar_static_f64[70])!=0.0)&&((self.scalar_static_f64[348])!=0.0));
        self.scalar_static_bool[55]=(self.scalar_static_bool[22]&&self.scalar_static_bool[52]);
        self.scalar_static_f64[349]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_bool[56]=(self.scalar_static_bool[54]&&((self.scalar_static_f64[349])!=0.0));
        self.scalar_static_bool[57]=(((self.scalar_static_f64[125])!=0.0)&&self.scalar_static_bool[56]);
        self.scalar_static_bool[58]=(((0.0)!=0.0)&&self.scalar_static_bool[56]);
        self.scalar_static_bool[59]=(true&&self.scalar_static_bool[56]);
        self.scalar_static_f64[350]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_bool[60]=(self.scalar_static_bool[54]&&((self.scalar_static_f64[350])!=0.0));
        self.scalar_static_bool[61]=(((self.scalar_static_f64[118])!=0.0)&&self.scalar_static_bool[60]);
        self.scalar_static_bool[62]=(((0.0)!=0.0)&&self.scalar_static_bool[60]);
        self.scalar_static_bool[63]=(true&&self.scalar_static_bool[60]);
        self.scalar_static_f64[351]=(self.scalar_static_f64[20]*self.scalar_static_f64[175]);
        self.scalar_static_f64[352]=(self.scalar_static_f64[175]*self.scalar_static_f64[150]);
        self.scalar_static_f64[353]=(-self.scalar_static_f64[351]);
        self.scalar_static_f64[354]=(-self.scalar_static_f64[352]);
        self.scalar_static_f64[355]=(-self.scalar_static_f64[175]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[352]/self.scalar_static_f64[230]);
        self.scalar_static_f64[357]=(self.scalar_static_f64[351]/self.scalar_static_f64[230]);
        self.scalar_static_f64[358]=(if self.scalar_static_bool[41]{self.scalar_static_f64[150]}else{0.0});
        self.scalar_static_f64[359]=(if self.scalar_static_bool[41]{self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[360]=(self.scalar_static_f64[20]*self.scalar_static_f64[358]);
        self.scalar_static_f64[361]=(self.scalar_static_f64[20]*self.scalar_static_f64[359]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[175]*self.scalar_static_f64[360]);
        self.scalar_static_f64[363]=(self.scalar_static_f64[175]*self.scalar_static_f64[361]);
        self.scalar_static_f64[364]=(if self.scalar_static_bool[54]{self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[365]=(if self.scalar_static_bool[54]{self.scalar_static_f64[150]}else{0.0});
        self.scalar_static_f64[366]=(self.scalar_static_f64[20]*self.scalar_static_f64[364]);
        self.scalar_static_f64[367]=(self.scalar_static_f64[20]*self.scalar_static_f64[365]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[175]*self.scalar_static_f64[366]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[175]*self.scalar_static_f64[367]);
        self.scalar_static_f64[370]=(-self.scalar_static_f64[285]);
        self.scalar_static_f64[371]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[285]}else{0.0});
        self.scalar_static_f64[372]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[370]}else{0.0});
        self.scalar_static_f64[373]=(-self.scalar_static_f64[286]);
        self.scalar_static_f64[374]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[286]}else{0.0});
        self.scalar_static_f64[375]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[373]}else{0.0});
        self.scalar_static_f64[376]=(-self.scalar_static_f64[287]);
        self.scalar_static_f64[377]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[287]}else{0.0});
        self.scalar_static_f64[378]=(if ((self.scalar_static_f64[59])!=0.0){self.scalar_static_f64[376]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
