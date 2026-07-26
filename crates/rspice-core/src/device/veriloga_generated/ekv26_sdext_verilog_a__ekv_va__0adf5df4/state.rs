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
    pub p72: f64, pub p73: f64, pub p74: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 75] = [
                1.0, 1.0, 0.0, 1e21, 1e21, 1e-5, 1e-5, 1.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.002, 3e-7, 0.5,
                0.001, 0.7, 0.5, 0.00015, -1.5, 0.0, 100000000.0, 2000000.0,
                0.8, 0.8, -1e-8, -1e-8, 0.2, 0.3, 0.00023, 4e-7,
                500000000.0, 400000000.0, 0.0009, 1.0, 0.0, 5e-7, 1e-6, 1e-6,
                1e-6, 1.0, 0.0, 1.0, 1e-9, 1e-12, 1e-12, 0.9,
                0.7, 0.7, 0.8, 0.6, 0.6, 1e-9, 1e-12, 1e-12,
                0.0, 0.0, 10.0, 1.0, 1.0, 1.0, 0.0, 0.0,
                0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 75);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 75] = [
    ("type", 0), ("noise", 1), ("trise", 2), ("temp", 3), ("tnom", 4), ("l", 5), ("w", 6), ("m", 7), ("ns", 8), ("as", 9), ("ad", 10), ("ps", 11), ("pd", 12), ("cox", 13), ("xj", 14), ("vto", 15),
    ("tcv", 16), ("gamma", 17), ("phi", 18), ("kp", 19), ("bex", 20), ("theta", 21), ("e0", 22), ("ucrit", 23), ("ucex", 24), ("lambda", 25), ("dl", 26), ("dw", 27), ("weta", 28), ("leta", 29), ("q0", 30), ("lk", 31),
    ("iba", 32), ("ibb", 33), ("ibbt", 34), ("ibn", 35), ("rsh", 36), ("hdif", 37), ("avto", 38), ("akp", 39), ("agamma", 40), ("af", 41), ("kf", 42), ("xd_n", 43), ("xd_js", 44), ("xd_jsw", 45), ("xd_jswg", 46), ("xd_mj", 47),
    ("xd_mjsw", 48), ("xd_mjswg", 49), ("xd_pb", 50), ("xd_pbsw", 51), ("xd_pbswg", 52), ("xd_cj", 53), ("xd_cjsw", 54), ("xd_cjswg", 55), ("xd_gmin", 56), ("xd_xjbv", 57), ("xd_bv", 58), ("xd_njts", 59), ("xd_njtssw", 60), ("xd_njtsswg", 61), ("xd_vts", 62), ("xd_vtssw", 63),
    ("xd_vtsswg", 64), ("tp_xti", 65), ("tp_cj", 66), ("tp_cjsw", 67), ("tp_cjswg", 68), ("tp_pb", 69), ("tp_pbsw", 70), ("tp_pbswg", 71), ("tp_njts", 72), ("tp_njtssw", 73), ("tp_njtsswg", 74),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 75] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 75] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 75] = [
    "TYPE", "Noise", "Trise", "TEMP", "TNOM", "L", "W", "M", "NS", "AS", "AD", "PS", "PD", "COX", "XJ", "VTO",
    "TCV", "GAMMA", "PHI", "KP", "BEX", "THETA", "E0", "UCRIT", "UCEX", "LAMBDA", "DL", "DW", "WETA", "LETA", "Q0", "LK",
    "IBA", "IBB", "IBBT", "IBN", "RSH", "HDIF", "AVTO", "AKP", "AGAMMA", "AF", "KF", "xd_n", "xd_js", "xd_jsw", "xd_jswg", "xd_mj",
    "xd_mjsw", "xd_mjswg", "xd_pb", "xd_pbsw", "xd_pbswg", "xd_cj", "xd_cjsw", "xd_cjswg", "xd_gmin", "xd_xjbv", "xd_bv", "xd_njts", "xd_njtssw", "xd_njtsswg", "xd_vts", "xd_vtssw",
    "xd_vtsswg", "tp_xti", "tp_cj", "tp_cjsw", "tp_cjswg", "tp_pb", "tp_pbsw", "tp_pbswg", "tp_njts", "tp_njtssw", "tp_njtsswg",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 75] = [
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

const PARAMETER_INTEGER_FLAGS: [bool; 75] = [
    true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 75] = [
    Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 273.15, label: "273.15" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 75] = [
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 75] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 2, 2, 2, 0, 0, 0, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 75] = [
    &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 4],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 75]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<5, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<350, 27>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 4;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 75;
    pub const VARIABLE_COUNT: usize = 271;
    pub const DDT_STATE_COUNT: usize = 5;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "4f385f216923511a8105fb0975a17ccd7d6a8e8672528f440da6e203d3b5adbf";
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

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            stamp_state,
            time,
            timestep,
            ddt_coefficients,
            scalar_static,
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
            stamp_state,
            time,
            timestep,
            ddt_coefficients,
            scalar_static,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
        };
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name));
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
        self.scalar_static.f64_values[0]=p.p13;
        self.scalar_static.f64_values[1]=(1.0359399871014713e-10/self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[2]=p.p14;
        self.scalar_static.f64_values[3]=(self.scalar_static.f64_values[1]*self.scalar_static.f64_values[2]);
        self.scalar_static.f64_values[4]=(self.scalar_static.f64_values[3]).sqrt();
        self.scalar_static.f64_values[5]=p.p25;
        self.scalar_static.f64_values[6]=(self.scalar_static.f64_values[4]*self.scalar_static.f64_values[5]);
        self.scalar_static.f64_values[7]=(self.scalar_static.f64_values[1]*3.0);
        self.scalar_static.f64_values[8]=p.p28;
        self.scalar_static.f64_values[9]=(self.scalar_static.f64_values[7]*self.scalar_static.f64_values[8]);
        self.scalar_static.f64_values[10]=p.p29;
        self.scalar_static.f64_values[11]=(self.scalar_static.f64_values[1]*self.scalar_static.f64_values[10]);
        self.scalar_static.f64_values[12]=p.p35;
        self.scalar_static.f64_values[13]=(self.scalar_static.f64_values[12]+self.scalar_static.f64_values[12]);
        self.scalar_static.f64_values[14]=p.p22;
        self.scalar_static.f64_values[15]=(1.0359399871014713e-10*self.scalar_static.f64_values[14]);
        self.scalar_static.f64_values[16]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[15]);
        self.scalar_static.f64_values[17]=p.p30;
        self.scalar_static.f64_values[18]=(self.scalar_static.f64_values[17]+self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[19]=(self.scalar_static.f64_values[18]/self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[20]=p.p0;
        self.scalar_static.bool_values[0]=(self.scalar_static.f64_values[20]>0.0);
        self.scalar_static.f64_values[21]=(if self.scalar_static.bool_values[0]{0.5}else{0.3333333333333});
        self.scalar_static.f64_values[22]=p.p3;
        self.scalar_static.bool_values[1]=(self.scalar_static.f64_values[22]==1e21);
        self.scalar_static.f64_values[23]=(if self.scalar_static.bool_values[1]{1.0}else{0.0});
        self.scalar_static.f64_values[24]=p.p2;
        self.scalar_static.bool_values[2]=(!((self.scalar_static.f64_values[23])!=0.0));
        self.scalar_static.f64_values[25]=(self.scalar_static.f64_values[22]+273.15);
        self.scalar_static.f64_values[26]=p.p4;
        self.scalar_static.bool_values[3]=(1e21==self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[27]=(if self.scalar_static.bool_values[3]{1.0}else{0.0});
        self.scalar_static.f64_values[28]=(if ((self.scalar_static.f64_values[27])!=0.0){298.15}else{0.0});
        self.scalar_static.bool_values[4]=(!((self.scalar_static.f64_values[27])!=0.0));
        self.scalar_static.f64_values[29]=(273.15+self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[30]=(if self.scalar_static.bool_values[4]{self.scalar_static.f64_values[29]}else{self.scalar_static.f64_values[28]});
        self.scalar_static.f64_values[31]=(self.scalar_static.f64_values[30]*0.000702);
        self.scalar_static.f64_values[32]=(self.scalar_static.f64_values[30]*self.scalar_static.f64_values[31]);
        self.scalar_static.f64_values[33]=(self.scalar_static.f64_values[30]+1108.0);
        self.scalar_static.f64_values[34]=(self.scalar_static.f64_values[32]/self.scalar_static.f64_values[33]);
        self.scalar_static.f64_values[35]=(1.16-self.scalar_static.f64_values[34]);
        self.scalar_static.f64_values[36]=p.p15;
        self.scalar_static.f64_values[37]=p.p16;
        self.scalar_static.f64_values[38]=p.p19;
        self.scalar_static.f64_values[39]=p.p20;
        self.scalar_static.f64_values[40]=p.p23;
        self.scalar_static.f64_values[41]=p.p24;
        self.scalar_static.f64_values[42]=p.p33;
        self.scalar_static.f64_values[43]=p.p34;
        self.scalar_static.f64_values[44]=p.p18;
        self.scalar_static.f64_values[45]=p.p32;
        self.scalar_static.f64_values[46]=p.p5;
        self.scalar_static.f64_values[47]=p.p26;
        self.scalar_static.f64_values[48]=(self.scalar_static.f64_values[46]+self.scalar_static.f64_values[47]);
        self.scalar_static.f64_values[49]=p.p6;
        self.scalar_static.f64_values[50]=p.p27;
        self.scalar_static.f64_values[51]=(self.scalar_static.f64_values[49]+self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[52]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[53]=(self.scalar_static.f64_values[52]).sqrt();
        self.scalar_static.f64_values[54]=(1.0/self.scalar_static.f64_values[53]);
        self.scalar_static.f64_values[55]=(if self.scalar_static.bool_values[0]{1.0}else{0.0});
        self.scalar_static.f64_values[56]=p.p38;
        self.scalar_static.bool_values[5]=(self.scalar_static.f64_values[56]!=1e-6);
        self.scalar_static.f64_values[57]=(self.scalar_static.f64_values[56]-1e-6);
        self.scalar_static.f64_values[58]=(self.scalar_static.f64_values[54]*self.scalar_static.f64_values[57]);
        self.scalar_static.bool_values[6]=(!((self.scalar_static.f64_values[55])!=0.0));
        self.scalar_static.f64_values[59]=(1e-6-self.scalar_static.f64_values[56]);
        self.scalar_static.f64_values[60]=(self.scalar_static.f64_values[54]*self.scalar_static.f64_values[59]);
        self.scalar_static.f64_values[61]=p.p39;
        self.scalar_static.bool_values[7]=(1e-6!=self.scalar_static.f64_values[61]);
        self.scalar_static.f64_values[62]=(self.scalar_static.f64_values[61]-1e-6);
        self.scalar_static.f64_values[63]=(self.scalar_static.f64_values[54]*self.scalar_static.f64_values[62]);
        self.scalar_static.f64_values[64]=(1.0+self.scalar_static.f64_values[63]);
        self.scalar_static.f64_values[65]=p.p40;
        self.scalar_static.bool_values[8]=(1e-6!=self.scalar_static.f64_values[65]);
        self.scalar_static.f64_values[66]=p.p17;
        self.scalar_static.f64_values[67]=(self.scalar_static.f64_values[65]-1e-6);
        self.scalar_static.f64_values[68]=(self.scalar_static.f64_values[54]*self.scalar_static.f64_values[67]);
        self.scalar_static.f64_values[69]=(self.scalar_static.f64_values[66]+self.scalar_static.f64_values[68]);
        self.scalar_static.f64_values[70]=(if self.scalar_static.bool_values[8]{self.scalar_static.f64_values[69]}else{self.scalar_static.f64_values[66]});
        self.scalar_static.bool_values[9]=(0.0==self.scalar_static.f64_values[19]);
        self.scalar_static.f64_values[71]=(if self.scalar_static.bool_values[9]{1.0}else{0.0});
        self.scalar_static.bool_values[10]=(!((self.scalar_static.f64_values[71])!=0.0));
        self.scalar_static.f64_values[72]=p.p31;
        self.scalar_static.f64_values[73]=p.p8;
        self.scalar_static.f64_values[74]=(self.scalar_static.f64_values[72]*self.scalar_static.f64_values[73]);
        self.scalar_static.f64_values[75]=(self.scalar_static.f64_values[48]/self.scalar_static.f64_values[74]);
        self.scalar_static.f64_values[76]=(self.scalar_static.f64_values[75]-0.1);
        self.scalar_static.f64_values[77]=(0.28*self.scalar_static.f64_values[76]);
        self.scalar_static.f64_values[78]=(if self.scalar_static.bool_values[10]{self.scalar_static.f64_values[77]}else{0.0});
        self.scalar_static.f64_values[79]=(self.scalar_static.f64_values[78]*self.scalar_static.f64_values[78]);
        self.scalar_static.f64_values[80]=(self.scalar_static.f64_values[79]+0.001936);
        self.scalar_static.f64_values[81]=(self.scalar_static.f64_values[80]).sqrt();
        self.scalar_static.f64_values[82]=(self.scalar_static.f64_values[78]+self.scalar_static.f64_values[81]);
        self.scalar_static.f64_values[83]=(0.5*self.scalar_static.f64_values[82]);
        self.scalar_static.f64_values[84]=(1.0+self.scalar_static.f64_values[83]);
        self.scalar_static.f64_values[85]=(1.0/self.scalar_static.f64_values[84]);
        self.scalar_static.f64_values[86]=(if self.scalar_static.bool_values[10]{self.scalar_static.f64_values[85]}else{0.0});
        self.scalar_static.f64_values[87]=(self.scalar_static.f64_values[19]*self.scalar_static.f64_values[86]);
        self.scalar_static.f64_values[88]=(self.scalar_static.f64_values[86]*self.scalar_static.f64_values[87]);
        self.scalar_static.f64_values[89]=(if self.scalar_static.bool_values[10]{self.scalar_static.f64_values[88]}else{0.0});
        self.scalar_static.f64_values[90]=p.p7;
        self.scalar_static.f64_values[91]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[90]);
        self.scalar_static.f64_values[92]=(self.scalar_static.f64_values[91]/self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[93]=(self.scalar_static.f64_values[11]*self.scalar_static.f64_values[73]);
        self.scalar_static.f64_values[94]=(self.scalar_static.f64_values[93]/self.scalar_static.f64_values[48]);
        self.scalar_static.f64_values[95]=(self.scalar_static.f64_values[70]*0.25);
        self.scalar_static.f64_values[96]=(self.scalar_static.f64_values[70]*self.scalar_static.f64_values[95]);
        self.scalar_static.f64_values[97]=(0.5*self.scalar_static.f64_values[70]);
        self.scalar_static.f64_values[98]=(0.1*self.scalar_static.f64_values[48]);
        self.scalar_static.f64_values[99]=(self.scalar_static.f64_values[98]*self.scalar_static.f64_values[98]);
        self.scalar_static.f64_values[100]=(self.scalar_static.f64_values[70]* -0.5);
        self.scalar_static.bool_values[11]=(0.0==self.scalar_static.f64_values[14]);
        self.scalar_static.f64_values[101]=(if self.scalar_static.bool_values[11]{1.0}else{0.0});
        self.scalar_static.f64_values[102]=p.p21;
        self.scalar_static.bool_values[12]=(!((self.scalar_static.f64_values[101])!=0.0));
        self.scalar_static.f64_values[103]=(-self.scalar_static.f64_values[94]);
        self.scalar_static.f64_values[104]=(-self.scalar_static.f64_values[70]);
        self.scalar_static.f64_values[105]=p.p36;
        self.scalar_static.f64_values[106]=p.p37;
        self.scalar_static.f64_values[107]=(self.scalar_static.f64_values[105]*self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[108]=(self.scalar_static.f64_values[51]-self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[109]=(self.scalar_static.f64_values[107]/self.scalar_static.f64_values[108]);
        self.scalar_static.f64_values[110]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[111]=p.p9;
        self.scalar_static.bool_values[13]=(0.0==self.scalar_static.f64_values[111]);
        self.scalar_static.bool_values[14]=(self.scalar_static.f64_values[106]>0.0);
        self.scalar_static.bool_values[15]=(self.scalar_static.bool_values[13]&&self.scalar_static.bool_values[14]);
        self.scalar_static.f64_values[112]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.f64_values[113]=(2.0*self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[114]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[113]);
        self.scalar_static.f64_values[115]=(if ((self.scalar_static.f64_values[112])!=0.0){self.scalar_static.f64_values[114]}else{0.0});
        self.scalar_static.bool_values[16]=(!((self.scalar_static.f64_values[112])!=0.0));
        self.scalar_static.f64_values[116]=(if self.scalar_static.bool_values[16]{self.scalar_static.f64_values[111]}else{self.scalar_static.f64_values[115]});
        self.scalar_static.f64_values[117]=p.p11;
        self.scalar_static.bool_values[17]=(0.0==self.scalar_static.f64_values[117]);
        self.scalar_static.bool_values[18]=(self.scalar_static.bool_values[14]&&self.scalar_static.bool_values[17]);
        self.scalar_static.f64_values[118]=(if self.scalar_static.bool_values[18]{1.0}else{0.0});
        self.scalar_static.f64_values[119]=(4.0*self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[120]=(self.scalar_static.f64_values[51]+self.scalar_static.f64_values[119]);
        self.scalar_static.f64_values[121]=(if ((self.scalar_static.f64_values[118])!=0.0){self.scalar_static.f64_values[120]}else{0.0});
        self.scalar_static.bool_values[19]=(!((self.scalar_static.f64_values[118])!=0.0));
        self.scalar_static.f64_values[122]=(if self.scalar_static.bool_values[19]{self.scalar_static.f64_values[117]}else{self.scalar_static.f64_values[121]});
        self.scalar_static.f64_values[123]=p.p10;
        self.scalar_static.bool_values[20]=(0.0==self.scalar_static.f64_values[123]);
        self.scalar_static.bool_values[21]=(self.scalar_static.bool_values[14]&&self.scalar_static.bool_values[20]);
        self.scalar_static.f64_values[124]=(if self.scalar_static.bool_values[21]{1.0}else{0.0});
        self.scalar_static.f64_values[125]=(if ((self.scalar_static.f64_values[124])!=0.0){self.scalar_static.f64_values[114]}else{0.0});
        self.scalar_static.bool_values[22]=(!((self.scalar_static.f64_values[124])!=0.0));
        self.scalar_static.f64_values[126]=(if self.scalar_static.bool_values[22]{self.scalar_static.f64_values[123]}else{self.scalar_static.f64_values[125]});
        self.scalar_static.f64_values[127]=p.p12;
        self.scalar_static.bool_values[23]=(0.0==self.scalar_static.f64_values[127]);
        self.scalar_static.bool_values[24]=(self.scalar_static.bool_values[14]&&self.scalar_static.bool_values[23]);
        self.scalar_static.f64_values[128]=(if self.scalar_static.bool_values[24]{1.0}else{0.0});
        self.scalar_static.f64_values[129]=(if ((self.scalar_static.f64_values[128])!=0.0){self.scalar_static.f64_values[120]}else{0.0});
        self.scalar_static.bool_values[25]=(!((self.scalar_static.f64_values[128])!=0.0));
        self.scalar_static.f64_values[130]=(if self.scalar_static.bool_values[25]{self.scalar_static.f64_values[127]}else{self.scalar_static.f64_values[129]});
        self.scalar_static.f64_values[131]=(self.scalar_static.f64_values[30]*8.617333262145179e-5);
        self.scalar_static.f64_values[132]=(self.scalar_static.f64_values[35]/self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[133]=p.p65;
        self.scalar_static.f64_values[134]=p.p43;
        self.scalar_static.f64_values[135]=p.p44;
        self.scalar_static.f64_values[136]=p.p45;
        self.scalar_static.f64_values[137]=p.p46;
        self.scalar_static.f64_values[138]=p.p50;
        self.scalar_static.f64_values[139]=p.p69;
        self.scalar_static.f64_values[140]=p.p51;
        self.scalar_static.f64_values[141]=p.p70;
        self.scalar_static.f64_values[142]=p.p52;
        self.scalar_static.f64_values[143]=p.p71;
        self.scalar_static.f64_values[144]=p.p53;
        self.scalar_static.f64_values[145]=p.p66;
        self.scalar_static.f64_values[146]=p.p54;
        self.scalar_static.f64_values[147]=p.p67;
        self.scalar_static.f64_values[148]=p.p55;
        self.scalar_static.f64_values[149]=p.p68;
        self.scalar_static.f64_values[150]=p.p59;
        self.scalar_static.f64_values[151]=p.p72;
        self.scalar_static.f64_values[152]=p.p60;
        self.scalar_static.f64_values[153]=p.p73;
        self.scalar_static.f64_values[154]=p.p61;
        self.scalar_static.f64_values[155]=p.p74;
        self.scalar_static.f64_values[156]=p.p58;
        self.scalar_static.f64_values[157]=p.p57;
        self.scalar_static.f64_values[158]=(-self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[159]=p.p64;
        self.scalar_static.f64_values[160]=p.p63;
        self.scalar_static.f64_values[161]=p.p62;
        self.scalar_static.f64_values[162]=p.p47;
        self.scalar_static.f64_values[163]=(-self.scalar_static.f64_values[162]);
        self.scalar_static.f64_values[164]=p.p48;
        self.scalar_static.f64_values[165]=(-self.scalar_static.f64_values[164]);
        self.scalar_static.f64_values[166]=p.p49;
        self.scalar_static.f64_values[167]=(-self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[168]=p.p56;
        self.scalar_static.f64_values[169]=(-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[170]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[162]);
        self.scalar_static.f64_values[171]=(self.scalar_static.f64_values[162]*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[172]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[164]);
        self.scalar_static.f64_values[173]=(self.scalar_static.f64_values[164]*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[174]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[175]=(self.scalar_static.f64_values[166]*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[176]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[168]);
        self.scalar_static.f64_values[177]=(self.scalar_static.f64_values[168]*self.scalar_static.f64_values[169]);
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
        self.scalar_static.f64_values[178]=(temperature+self.scalar_static.f64_values[24]);
        self.scalar_static.f64_values[179]=(if ((self.scalar_static.f64_values[23])!=0.0){self.scalar_static.f64_values[178]}else{0.0});
        self.scalar_static.f64_values[180]=(if self.scalar_static.bool_values[2]{self.scalar_static.f64_values[25]}else{self.scalar_static.f64_values[179]});
        self.scalar_static.f64_values[181]=(self.scalar_static.f64_values[180]*8.617333262145179e-5);
        self.scalar_static.f64_values[182]=(self.scalar_static.f64_values[181]*0.1);
        self.scalar_static.f64_values[183]=(1.0/self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[184]=(self.scalar_static.f64_values[181]+self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[185]=(self.scalar_static.f64_values[184]+self.scalar_static.f64_values[184]);
        self.scalar_static.f64_values[186]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[187]=(self.scalar_static.f64_values[186]+self.scalar_static.f64_values[186]);
        self.scalar_static.f64_values[188]=(self.scalar_static.f64_values[186]*16.0);
        self.scalar_static.f64_values[189]=(self.scalar_static.f64_values[180]*0.000702);
        self.scalar_static.f64_values[190]=(self.scalar_static.f64_values[180]*self.scalar_static.f64_values[189]);
        self.scalar_static.f64_values[191]=(self.scalar_static.f64_values[180]+1108.0);
        self.scalar_static.f64_values[192]=(self.scalar_static.f64_values[190]/self.scalar_static.f64_values[191]);
        self.scalar_static.f64_values[193]=(1.16-self.scalar_static.f64_values[192]);
        self.scalar_static.f64_values[194]=(self.scalar_static.f64_values[180]-self.scalar_static.f64_values[30]);
        self.scalar_static.f64_values[195]=(self.scalar_static.f64_values[180]/self.scalar_static.f64_values[30]);
        self.scalar_static.f64_values[196]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[37]);
        self.scalar_static.f64_values[197]=(self.scalar_static.f64_values[36]-self.scalar_static.f64_values[196]);
        self.scalar_static.f64_values[198]=f64::powf(self.scalar_static.f64_values[195],self.scalar_static.f64_values[39]);
        self.scalar_static.f64_values[199]=(self.scalar_static.f64_values[38]*self.scalar_static.f64_values[198]);
        self.scalar_static.f64_values[200]=f64::powf(self.scalar_static.f64_values[195],self.scalar_static.f64_values[41]);
        self.scalar_static.f64_values[201]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[200]);
        self.scalar_static.f64_values[202]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[43]);
        self.scalar_static.f64_values[203]=(1.0+self.scalar_static.f64_values[202]);
        self.scalar_static.f64_values[204]=(self.scalar_static.f64_values[42]*self.scalar_static.f64_values[203]);
        self.scalar_static.f64_values[205]=(self.scalar_static.f64_values[195]*self.scalar_static.f64_values[44]);
        self.scalar_static.f64_values[206]=(3.0*self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[207]=(self.scalar_static.f64_values[195]).ln();
        self.scalar_static.f64_values[208]=(self.scalar_static.f64_values[206]*self.scalar_static.f64_values[207]);
        self.scalar_static.f64_values[209]=(self.scalar_static.f64_values[205]-self.scalar_static.f64_values[208]);
        self.scalar_static.f64_values[210]=(self.scalar_static.f64_values[35]*self.scalar_static.f64_values[195]);
        self.scalar_static.f64_values[211]=(self.scalar_static.f64_values[209]-self.scalar_static.f64_values[210]);
        self.scalar_static.f64_values[212]=(self.scalar_static.f64_values[193]+self.scalar_static.f64_values[211]);
        self.scalar_static.f64_values[213]=(self.scalar_static.f64_values[212]-0.2);
        self.scalar_static.f64_values[214]=(self.scalar_static.f64_values[213]*self.scalar_static.f64_values[213]);
        self.scalar_static.f64_values[215]=(self.scalar_static.f64_values[186]+self.scalar_static.f64_values[214]);
        self.scalar_static.f64_values[216]=(self.scalar_static.f64_values[215]).sqrt();
        self.scalar_static.f64_values[217]=(self.scalar_static.f64_values[213]+self.scalar_static.f64_values[216]);
        self.scalar_static.f64_values[218]=(0.5*self.scalar_static.f64_values[217]);
        self.scalar_static.f64_values[219]=(0.2+self.scalar_static.f64_values[218]);
        self.scalar_static.f64_values[220]=(self.scalar_static.f64_values[219]).sqrt();
        self.scalar_static.f64_values[221]=(1.0/self.scalar_static.f64_values[201]);
        self.scalar_static.f64_values[222]=(self.scalar_static.f64_values[4]*self.scalar_static.f64_values[201]);
        self.scalar_static.f64_values[223]=(self.scalar_static.f64_values[4]*self.scalar_static.f64_values[204]);
        self.scalar_static.f64_values[224]=(self.scalar_static.f64_values[45]/self.scalar_static.f64_values[204]);
        self.scalar_static.f64_values[225]=(self.scalar_static.f64_values[201]*self.scalar_static.f64_values[48]);
        self.scalar_static.f64_values[226]=(0.5*self.scalar_static.f64_values[225]);
        self.scalar_static.f64_values[227]=(self.scalar_static.f64_values[183]*self.scalar_static.f64_values[226]);
        self.scalar_static.f64_values[228]=(self.scalar_static.f64_values[227]).ln();
        self.scalar_static.f64_values[229]=(self.scalar_static.f64_values[228]-0.6);
        self.scalar_static.f64_values[230]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[229]);
        self.scalar_static.f64_values[231]=(self.scalar_static.f64_values[197]+self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[232]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[231]}else{self.scalar_static.f64_values[197]});
        self.scalar_static.f64_values[233]=(if ((self.scalar_static.f64_values[55])!=0.0){self.scalar_static.f64_values[232]}else{0.0});
        self.scalar_static.f64_values[234]=(self.scalar_static.f64_values[60]-self.scalar_static.f64_values[197]);
        self.scalar_static.f64_values[235]=(-self.scalar_static.f64_values[197]);
        self.scalar_static.f64_values[236]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[234]}else{self.scalar_static.f64_values[235]});
        self.scalar_static.f64_values[237]=(if self.scalar_static.bool_values[6]{self.scalar_static.f64_values[236]}else{self.scalar_static.f64_values[233]});
        self.scalar_static.f64_values[238]=(self.scalar_static.f64_values[199]*self.scalar_static.f64_values[64]);
        self.scalar_static.f64_values[239]=(if self.scalar_static.bool_values[7]{self.scalar_static.f64_values[238]}else{self.scalar_static.f64_values[199]});
        self.scalar_static.f64_values[240]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[239]);
        self.scalar_static.f64_values[241]=(self.scalar_static.f64_values[220]*self.scalar_static.f64_values[70]);
        self.scalar_static.f64_values[242]=(self.scalar_static.f64_values[188]*2.0);
        self.scalar_static.f64_values[243]=(self.scalar_static.f64_values[181]/self.scalar_static.f64_values[225]);
        self.scalar_static.f64_values[244]=(self.scalar_static.f64_values[16]*self.scalar_static.f64_values[241]);
        self.scalar_static.f64_values[245]=(1.0+self.scalar_static.f64_values[244]);
        self.scalar_static.f64_values[246]=(if self.scalar_static.bool_values[12]{self.scalar_static.f64_values[245]}else{0.0});
        self.scalar_static.f64_values[247]=(self.scalar_static.f64_values[240]*self.scalar_static.f64_values[246]);
        self.scalar_static.f64_values[248]=(self.scalar_static.f64_values[185]+self.scalar_static.f64_values[185]);
        self.scalar_static.f64_values[249]=(self.scalar_static.f64_values[5]*self.scalar_static.f64_values[248]);
        self.scalar_static.bool_values[26]=(self.scalar_static.f64_values[224]>0.0);
        self.scalar_static.f64_values[250]=(-self.scalar_static.f64_values[223]);
        self.scalar_static.f64_values[251]=(self.scalar_static.f64_values[193]/self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[252]=(self.scalar_static.f64_values[132]-self.scalar_static.f64_values[251]);
        self.scalar_static.f64_values[253]=(self.scalar_static.f64_values[207]*self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[254]=(self.scalar_static.f64_values[252]+self.scalar_static.f64_values[253]);
        self.scalar_static.f64_values[255]=(self.scalar_static.f64_values[254]/self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[256]=(self.scalar_static.f64_values[255]).exp();
        self.scalar_static.f64_values[257]=(self.scalar_static.f64_values[256]*self.scalar_static.f64_values[135]);
        self.scalar_static.f64_values[258]=(self.scalar_static.f64_values[256]*self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[259]=(self.scalar_static.f64_values[256]*self.scalar_static.f64_values[137]);
        self.scalar_static.f64_values[260]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[139]);
        self.scalar_static.f64_values[261]=(self.scalar_static.f64_values[138]-self.scalar_static.f64_values[260]);
        self.scalar_static.f64_values[262]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[141]);
        self.scalar_static.f64_values[263]=(self.scalar_static.f64_values[140]-self.scalar_static.f64_values[262]);
        self.scalar_static.f64_values[264]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[265]=(self.scalar_static.f64_values[142]-self.scalar_static.f64_values[264]);
        self.scalar_static.f64_values[266]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[145]);
        self.scalar_static.f64_values[267]=(1.0+self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[268]=(self.scalar_static.f64_values[144]*self.scalar_static.f64_values[267]);
        self.scalar_static.f64_values[269]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[270]=(1.0+self.scalar_static.f64_values[269]);
        self.scalar_static.f64_values[271]=(self.scalar_static.f64_values[146]*self.scalar_static.f64_values[270]);
        self.scalar_static.f64_values[272]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[273]=(1.0+self.scalar_static.f64_values[272]);
        self.scalar_static.f64_values[274]=(self.scalar_static.f64_values[148]*self.scalar_static.f64_values[273]);
        self.scalar_static.f64_values[275]=(self.scalar_static.f64_values[195]-1.0);
        self.scalar_static.f64_values[276]=(self.scalar_static.f64_values[275]*self.scalar_static.f64_values[151]);
        self.scalar_static.f64_values[277]=(1.0+self.scalar_static.f64_values[276]);
        self.scalar_static.f64_values[278]=(self.scalar_static.f64_values[150]*self.scalar_static.f64_values[277]);
        self.scalar_static.f64_values[279]=(self.scalar_static.f64_values[275]*self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[280]=(1.0+self.scalar_static.f64_values[279]);
        self.scalar_static.f64_values[281]=(self.scalar_static.f64_values[152]*self.scalar_static.f64_values[280]);
        self.scalar_static.f64_values[282]=(self.scalar_static.f64_values[275]*self.scalar_static.f64_values[155]);
        self.scalar_static.f64_values[283]=(1.0+self.scalar_static.f64_values[282]);
        self.scalar_static.f64_values[284]=(self.scalar_static.f64_values[154]*self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[285]=(self.scalar_static.f64_values[126]*self.scalar_static.f64_values[257]);
        self.scalar_static.f64_values[286]=(self.scalar_static.f64_values[130]*self.scalar_static.f64_values[258]);
        self.scalar_static.f64_values[287]=(self.scalar_static.f64_values[285]+self.scalar_static.f64_values[286]);
        self.scalar_static.f64_values[288]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[259]);
        self.scalar_static.f64_values[289]=(self.scalar_static.f64_values[287]+self.scalar_static.f64_values[288]);
        self.scalar_static.f64_values[290]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[291]=(self.scalar_static.f64_values[259]*self.scalar_static.f64_values[158]);
        self.scalar_static.f64_values[292]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[284]);
        self.scalar_static.f64_values[293]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[281]);
        self.scalar_static.f64_values[294]=(self.scalar_static.f64_values[181]*self.scalar_static.f64_values[278]);
        self.scalar_static.f64_values[295]=(self.scalar_static.f64_values[116]*self.scalar_static.f64_values[257]);
        self.scalar_static.f64_values[296]=(self.scalar_static.f64_values[122]*self.scalar_static.f64_values[258]);
        self.scalar_static.f64_values[297]=(self.scalar_static.f64_values[295]+self.scalar_static.f64_values[296]);
        self.scalar_static.f64_values[298]=(self.scalar_static.f64_values[288]+self.scalar_static.f64_values[297]);
        self.scalar_static.f64_values[299]=(self.scalar_static.f64_values[126]*self.scalar_static.f64_values[268]);
        self.scalar_static.f64_values[300]=(self.scalar_static.f64_values[130]*self.scalar_static.f64_values[271]);
        self.scalar_static.f64_values[301]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[274]);
        self.scalar_static.f64_values[302]=(self.scalar_static.f64_values[116]*self.scalar_static.f64_values[268]);
        self.scalar_static.f64_values[303]=(self.scalar_static.f64_values[122]*self.scalar_static.f64_values[271]);
        self.scalar_static.f64_values[304]=(self.scalar_static.f64_values[195]*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[305]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[195]);
        self.scalar_static.f64_values[306]=(self.scalar_static.f64_values[304]/self.scalar_static.f64_values[290]);
        self.scalar_static.f64_values[307]=(self.scalar_static.f64_values[305]/self.scalar_static.f64_values[290]);
        self.scalar_static.f64_values[308]=(-self.scalar_static.f64_values[306]);
        self.scalar_static.f64_values[309]=(-self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[310]=(self.scalar_static.f64_values[305]/self.scalar_static.f64_values[292]);
        self.scalar_static.f64_values[311]=(self.scalar_static.f64_values[304]/self.scalar_static.f64_values[292]);
        self.scalar_static.f64_values[312]=(self.scalar_static.f64_values[159]*self.scalar_static.f64_values[310]);
        self.scalar_static.f64_values[313]=(self.scalar_static.f64_values[159]*self.scalar_static.f64_values[311]);
        self.scalar_static.f64_values[314]=(self.scalar_static.f64_values[305]/self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[315]=(self.scalar_static.f64_values[304]/self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[316]=(self.scalar_static.f64_values[160]*self.scalar_static.f64_values[314]);
        self.scalar_static.f64_values[317]=(self.scalar_static.f64_values[160]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[318]=(self.scalar_static.f64_values[305]/self.scalar_static.f64_values[294]);
        self.scalar_static.f64_values[319]=(self.scalar_static.f64_values[304]/self.scalar_static.f64_values[294]);
        self.scalar_static.f64_values[320]=(self.scalar_static.f64_values[161]*self.scalar_static.f64_values[318]);
        self.scalar_static.f64_values[321]=(self.scalar_static.f64_values[161]*self.scalar_static.f64_values[319]);
        self.scalar_static.f64_values[322]=(self.scalar_static.f64_values[20]/self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[323]=(self.scalar_static.f64_values[169]/self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[324]=(self.scalar_static.f64_values[20]/self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[325]=(self.scalar_static.f64_values[169]/self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[326]=(self.scalar_static.f64_values[20]/self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[327]=(self.scalar_static.f64_values[169]/self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[328]=(self.scalar_static.f64_values[170]/self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[329]=(self.scalar_static.f64_values[171]/self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[330]=(-self.scalar_static.f64_values[328]);
        self.scalar_static.f64_values[331]=(-self.scalar_static.f64_values[329]);
        self.scalar_static.f64_values[332]=(self.scalar_static.f64_values[299]*self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[333]=(self.scalar_static.f64_values[299]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[172]/self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[335]=(self.scalar_static.f64_values[173]/self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[336]=(-self.scalar_static.f64_values[334]);
        self.scalar_static.f64_values[337]=(-self.scalar_static.f64_values[335]);
        self.scalar_static.f64_values[338]=(self.scalar_static.f64_values[300]*self.scalar_static.f64_values[336]);
        self.scalar_static.f64_values[339]=(self.scalar_static.f64_values[300]*self.scalar_static.f64_values[337]);
        self.scalar_static.f64_values[340]=(self.scalar_static.f64_values[174]/self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[341]=(self.scalar_static.f64_values[175]/self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[342]=(-self.scalar_static.f64_values[340]);
        self.scalar_static.f64_values[343]=(-self.scalar_static.f64_values[341]);
        self.scalar_static.f64_values[344]=(self.scalar_static.f64_values[301]*self.scalar_static.f64_values[342]);
        self.scalar_static.f64_values[345]=(self.scalar_static.f64_values[301]*self.scalar_static.f64_values[343]);
        self.scalar_static.f64_values[346]=(self.scalar_static.f64_values[302]*self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[347]=(self.scalar_static.f64_values[302]*self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[348]=(self.scalar_static.f64_values[303]*self.scalar_static.f64_values[336]);
        self.scalar_static.f64_values[349]=(self.scalar_static.f64_values[303]*self.scalar_static.f64_values[337]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
