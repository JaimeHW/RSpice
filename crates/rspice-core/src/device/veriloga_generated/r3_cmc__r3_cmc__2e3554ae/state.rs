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
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 22] = [
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.0, 1.0, 1.0, 2.0, 1003.0, -1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 22);
            {
                let params = &mut *ptr;
                params.p22 = 1.0;
                validate_parameter("scale", params.p22, false, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p23 = 0.0;
                validate_parameter("shrink", params.p23, false, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 2] = [
                -100.0, 500.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(24), 2);
            {
                let params = &mut *ptr;
                params.p26 = 0.001;
                validate_parameter("rthresh", params.p26, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p27 = 1.0;
                validate_parameter("imax", params.p27, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 100] = [
                27.0, 0.0, 9900000000.0, 0.0, 9900000000.0, 100.0, 9900000000.0, -100.0,
                500.0, 100.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.01, 0.0, 0.0,
                0.0, 1.0, 2.0, 0.0, 0.5, 0.0, 2.0, 0.0,
                4.0, 0.4, 0.0, 0.0, 1e-12, 0.02, 0.0, 0.0,
                0.9, 0.0, 1.0, 0.0, 0.0, 0.75, 0.33, -0.5,
                0.0, 1.0, 0.0, 0.0, 0.75, 0.33, -0.5, 0.0,
                1e-6, 1.0, 0.0, 2.0, 1.0, 0.0, 1.12, 3.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1000000.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(28), 100);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 132] = [
    ("w", 0), ("l", 1), ("wd", 2), ("a1", 3), ("p1", 4), ("c1", 5), ("a2", 6), ("p2", 7), ("c2", 8), ("trise", 9), ("dtemp", 9), ("dta", 9), ("nsmm_rsh", 10), ("nsmm_w", 11), ("nsmm_l", 12), ("sw_noise", 13),
    ("sw_et", 14), ("sw_lin", 15), ("sw_mman", 16), ("version", 17), ("subversion", 18), ("revision", 19), ("level", 20), ("type", 21), ("scale", 22), ("shrink", 23), ("tmin", 24), ("tmax", 25), ("rthresh", 26), ("imax", 27), ("tnom", 28), ("lmin", 29),
    ("lmax", 30), ("wmin", 31), ("wmax", 32), ("jmax", 33), ("vmax", 34), ("tminclip", 35), ("tmaxclip", 36), ("rsh", 37), ("xw", 38), ("nwxw", 39), ("wexw", 40), ("fdrw", 41), ("fdxwinf", 42), ("xl", 43), ("xlw", 44), ("dxlsat", 45),
    ("nst", 46), ("ats", 47), ("atsinf", 47), ("atsl", 48), ("dfinf", 49), ("dfw", 50), ("dfl", 51), ("dfwl", 52), ("sw_dfgeo", 53), ("dp", 54), ("dpinf", 54), ("dpw", 55), ("dpwe", 56), ("dpl", 57), ("dple", 58), ("dpwl", 59),
    ("ecrit", 60), ("ecorn", 61), ("sw_vsatt", 62), ("sw_accpo", 63), ("grpo", 64), ("du", 65), ("rc", 66), ("rcw", 67), ("fc", 68), ("isa", 69), ("na", 70), ("ca", 71), ("cja", 72), ("pa", 73), ("ma", 74), ("aja", 75),
    ("isp", 76), ("np", 77), ("cp", 78), ("cjp", 79), ("pp", 80), ("mp", 81), ("ajp", 82), ("vbv", 83), ("ibv", 84), ("nbv", 85), ("kfn", 86), ("afn", 87), ("bfn", 88), ("sw_fngeo", 89), ("ea", 90), ("xis", 91),
    ("xvsat", 92), ("tc1", 93), ("tc2", 94), ("tc1l", 95), ("tc2l", 96), ("tc1w", 97), ("tc2w", 98), ("tc1wl", 99), ("tc2wl", 100), ("tc1rc", 101), ("tc2rc", 102), ("tc1dp", 103), ("tc2dp", 104), ("tc1vbv", 105), ("tc2vbv", 106), ("tc1nbv", 107),
    ("tc1kfn", 108), ("tegth", 109), ("gth0", 110), ("gthp", 111), ("gtha", 112), ("gthc", 113), ("cth0", 114), ("cthp", 115), ("ctha", 116), ("cthc", 117), ("nsig_rsh", 118), ("nsig_w", 119), ("nsig_l", 120), ("sig_rsh", 121), ("sig_w", 122), ("sig_l", 123),
    ("smm_rsh", 124), ("smm_w", 125), ("smm_l", 126), ("sw_mmgeo", 127),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 128] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, Some(29), None,
    Some(31), None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 128] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, Some(60), None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 128] = [
    "w", "l", "wd", "a1", "p1", "c1", "a2", "p2", "c2", "trise", "nsmm_rsh", "nsmm_w", "nsmm_l", "sw_noise", "sw_et", "sw_lin",
    "sw_mman", "version", "subversion", "revision", "level", "type", "scale", "shrink", "tmin", "tmax", "rthresh", "imax", "tnom", "lmin", "lmax", "wmin",
    "wmax", "jmax", "vmax", "tminclip", "tmaxclip", "rsh", "xw", "nwxw", "wexw", "fdrw", "fdxwinf", "xl", "xlw", "dxlsat", "nst", "ats",
    "atsl", "dfinf", "dfw", "dfl", "dfwl", "sw_dfgeo", "dp", "dpw", "dpwe", "dpl", "dple", "dpwl", "ecrit", "ecorn", "sw_vsatt", "sw_accpo",
    "grpo", "du", "rc", "rcw", "fc", "isa", "na", "ca", "cja", "pa", "ma", "aja", "isp", "np", "cp", "cjp",
    "pp", "mp", "ajp", "vbv", "ibv", "nbv", "kfn", "afn", "bfn", "sw_fngeo", "ea", "xis", "xvsat", "tc1", "tc2", "tc1l",
    "tc2l", "tc1w", "tc2w", "tc1wl", "tc2wl", "tc1rc", "tc2rc", "tc1dp", "tc2dp", "tc1vbv", "tc2vbv", "tc1nbv", "tc1kfn", "tegth", "gth0", "gthp",
    "gtha", "gthc", "cth0", "cthp", "ctha", "cthc", "nsig_rsh", "nsig_w", "nsig_l", "sig_rsh", "sig_w", "sig_l", "smm_rsh", "smm_w", "smm_l", "sw_mmgeo",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 128] = [
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
];

const PARAMETER_INTEGER_FLAGS: [bool; 128] = [
    false, false, false, false, false, true, false, false, true, false, false, false, false, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 128] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0001, label: "0.0001" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 128] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), None,
    None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
];

const PARAMETER_RANGE_FLAGS: [u8; 128] = [
    3, 3, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 3, 3, 0, 2, 3, 2,
    3, 3, 3, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 2, 2, 0, 2, 3, 2, 2, 3, 3, 0, 2, 3, 2, 2, 3, 3, 0, 2, 3, 3, 2, 3, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 128] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 6],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 128]>,
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
    pub(crate) scalar_static_f64: Box<[f64; 305]>,
    pub(crate) scalar_static_bool: Box<[bool; 89]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 2;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 2] = ["i1", "i2"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 128;
    pub const VARIABLE_COUNT: usize = 329;
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
            scalar_static_f64: boxed_zero_f64_array::<305>(),
            scalar_static_bool: boxed_zero_bool_array::<89>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'r3_cmc'", name));
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
        self.scalar_static_f64[0]=p.p23;
        self.scalar_static_f64[1]=(0.01*self.scalar_static_f64[0]);
        self.scalar_static_f64[2]=(1.0-self.scalar_static_f64[1]);
        self.scalar_static_f64[3]=p.p22;
        self.scalar_static_f64[4]=(self.scalar_static_f64[2]*self.scalar_static_f64[3]);
        self.scalar_static_f64[5]=(self.scalar_static_f64[4]*1000000.0);
        self.scalar_static_f64[6]=(self.scalar_static_f64[5]*self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=p.p28;
        self.scalar_static_f64[8]=(273.15+self.scalar_static_f64[7]);
        self.scalar_static_f64[9]=p.p9;
        self.scalar_static_f64[10]=p.p35;
        self.scalar_static_f64[11]=(1.0+self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=p.p36;
        self.scalar_static_f64[13]=(self.scalar_static_f64[12]-1.0);
        self.scalar_static_f64[14]=p.p0;
        self.scalar_static_f64[15]=(self.scalar_static_f64[5]*self.scalar_static_f64[14]);
        self.scalar_static_f64[16]=p.p1;
        self.scalar_static_f64[17]=(self.scalar_static_f64[5]*self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=p.p2;
        self.scalar_static_f64[19]=(self.scalar_static_f64[5]*self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=(0.0*self.scalar_static_f64[6]);
        self.scalar_static_f64[21]=p.p4;
        self.scalar_static_f64[22]=(self.scalar_static_f64[5]*self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=p.p7;
        self.scalar_static_f64[24]=(self.scalar_static_f64[5]*self.scalar_static_f64[23]);
        self.scalar_static_f64[25]=(self.scalar_static_f64[15]*self.scalar_static_f64[17]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[17]*2.0);
        self.scalar_static_f64[27]=p.p5;
        self.scalar_static_bool[0]=(self.scalar_static_f64[27]>0.0);
        self.scalar_static_f64[28]=p.p8;
        self.scalar_static_bool[1]=(self.scalar_static_f64[28]>0.0);
        self.scalar_static_f64[29]=((if self.scalar_static_bool[0]{1.0}else{0.0})+(if self.scalar_static_bool[1]{1.0}else{0.0}));
        self.scalar_static_f64[30]=(self.scalar_static_f64[15]*self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=(self.scalar_static_f64[26]+self.scalar_static_f64[30]);
        self.scalar_static_f64[32]=(self.scalar_static_f64[29]*0.5);
        self.scalar_static_f64[33]=p.p43;
        self.scalar_static_f64[34]=p.p44;
        self.scalar_static_f64[35]=(self.scalar_static_f64[34]/self.scalar_static_f64[15]);
        self.scalar_static_f64[36]=(self.scalar_static_f64[33]+self.scalar_static_f64[35]);
        self.scalar_static_f64[37]=(self.scalar_static_f64[32]*self.scalar_static_f64[36]);
        self.scalar_static_f64[38]=p.p38;
        self.scalar_static_f64[39]=(self.scalar_static_f64[15]+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=p.p39;
        self.scalar_static_f64[41]=(self.scalar_static_f64[40]/self.scalar_static_f64[15]);
        self.scalar_static_f64[42]=(self.scalar_static_f64[39]+self.scalar_static_f64[41]);
        self.scalar_static_f64[43]=p.p42;
        self.scalar_static_f64[44]=(-self.scalar_static_f64[15]);
        self.scalar_static_f64[45]=p.p41;
        self.scalar_static_f64[46]=(self.scalar_static_f64[44]/self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=(self.scalar_static_f64[46]).exp();
        self.scalar_static_f64[48]=(1.0-self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=(self.scalar_static_f64[43]*self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=(self.scalar_static_f64[42]+self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p40;
        self.scalar_static_f64[52]=(self.scalar_static_f64[19]*self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=(self.scalar_static_f64[52]/self.scalar_static_f64[25]);
        self.scalar_static_f64[54]=(1.0-self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(self.scalar_static_f64[50]/self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(self.scalar_static_f64[17]+self.scalar_static_f64[37]);
        self.scalar_static_f64[57]=p.p127;
        self.scalar_static_f64[58]=(if ((self.scalar_static_f64[57])!=0.0){self.scalar_static_f64[55]}else{0.0});
        self.scalar_static_f64[59]=(if ((self.scalar_static_f64[57])!=0.0){self.scalar_static_f64[56]}else{0.0});
        self.scalar_static_bool[2]=(!((self.scalar_static_f64[57])!=0.0));
        self.scalar_static_f64[60]=(if self.scalar_static_bool[2]{self.scalar_static_f64[15]}else{self.scalar_static_f64[58]});
        self.scalar_static_f64[61]=(if self.scalar_static_bool[2]{self.scalar_static_f64[17]}else{self.scalar_static_f64[59]});
        self.scalar_static_f64[62]=p.p16;
        self.scalar_static_f64[63]=p.p119;
        self.scalar_static_f64[64]=p.p122;
        self.scalar_static_f64[65]=(self.scalar_static_f64[63]*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[55]+self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=p.p11;
        self.scalar_static_f64[68]=p.p125;
        self.scalar_static_f64[69]=(self.scalar_static_f64[67]*self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=p.p120;
        self.scalar_static_f64[71]=p.p123;
        self.scalar_static_f64[72]=(self.scalar_static_f64[70]*self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=(self.scalar_static_f64[56]+self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=p.p12;
        self.scalar_static_f64[75]=p.p126;
        self.scalar_static_f64[76]=(self.scalar_static_f64[74]*self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=p.p118;
        self.scalar_static_f64[78]=p.p121;
        self.scalar_static_f64[79]=(self.scalar_static_f64[77]*self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=p.p10;
        self.scalar_static_f64[81]=p.p124;
        self.scalar_static_f64[82]=(self.scalar_static_f64[80]*self.scalar_static_f64[81]);
        self.scalar_static_bool[3]=(0.0!=self.scalar_static_f64[63]);
        self.scalar_static_bool[4]=(self.scalar_static_f64[68]>0.0);
        self.scalar_static_bool[5]=(self.scalar_static_f64[64]>0.0);
        self.scalar_static_bool[6]=(self.scalar_static_bool[4]||self.scalar_static_bool[5]);
        self.scalar_static_bool[7]=(self.scalar_static_bool[3]&&self.scalar_static_bool[6]);
        self.scalar_static_f64[83]=(if self.scalar_static_bool[7]{1.0}else{0.0});
        self.scalar_static_bool[8]=(!((self.scalar_static_f64[62])!=0.0));
        self.scalar_static_bool[9]=(((self.scalar_static_f64[83])!=0.0)&&self.scalar_static_bool[8]);
        self.scalar_static_f64[84]=(self.scalar_static_f64[64]*self.scalar_static_f64[64]);
        self.scalar_static_bool[10]=(0.0!=self.scalar_static_f64[70]);
        self.scalar_static_bool[11]=(self.scalar_static_f64[75]>0.0);
        self.scalar_static_bool[12]=(self.scalar_static_f64[71]>0.0);
        self.scalar_static_bool[13]=(self.scalar_static_bool[11]||self.scalar_static_bool[12]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[10]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[85]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_bool[15]=(self.scalar_static_bool[8]&&((self.scalar_static_f64[85])!=0.0));
        self.scalar_static_f64[86]=(self.scalar_static_f64[71]*self.scalar_static_f64[71]);
        self.scalar_static_bool[16]=(0.0!=self.scalar_static_f64[77]);
        self.scalar_static_bool[17]=(self.scalar_static_f64[81]>0.0);
        self.scalar_static_bool[18]=(self.scalar_static_f64[78]>0.0);
        self.scalar_static_bool[19]=(self.scalar_static_bool[17]||self.scalar_static_bool[18]);
        self.scalar_static_bool[20]=(self.scalar_static_bool[16]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[87]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_bool[21]=(self.scalar_static_bool[8]&&((self.scalar_static_f64[87])!=0.0));
        self.scalar_static_f64[88]=(0.01*self.scalar_static_f64[77]);
        self.scalar_static_f64[89]=(self.scalar_static_f64[78]*self.scalar_static_f64[78]);
        self.scalar_static_bool[22]=(!((self.scalar_static_f64[87])!=0.0));
        self.scalar_static_bool[23]=(self.scalar_static_bool[8]&&self.scalar_static_bool[22]);
        self.scalar_static_f64[90]=p.p45;
        self.scalar_static_f64[91]=p.p53;
        self.scalar_static_bool[24]=(!((self.scalar_static_f64[91])!=0.0));
        self.scalar_static_f64[92]=p.p56;
        self.scalar_static_f64[93]=p.p58;
        self.scalar_static_f64[94]=p.p54;
        self.scalar_static_f64[95]=p.p55;
        self.scalar_static_f64[96]=p.p57;
        self.scalar_static_f64[97]=p.p59;
        self.scalar_static_f64[98]=p.p103;
        self.scalar_static_f64[99]=p.p104;
        self.scalar_static_f64[100]=p.p15;
        self.scalar_static_f64[101]=p.p49;
        self.scalar_static_f64[102]=p.p50;
        self.scalar_static_f64[103]=p.p51;
        self.scalar_static_f64[104]=p.p52;
        self.scalar_static_f64[105]=p.p63;
        self.scalar_static_bool[25]=(self.scalar_static_f64[105]>1.0);
        self.scalar_static_f64[106]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_f64[107]=p.p64;
        self.scalar_static_f64[108]=(2.0*self.scalar_static_f64[107]);
        self.scalar_static_bool[26]=(self.scalar_static_f64[105]>0.0);
        self.scalar_static_f64[109]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=(!((self.scalar_static_f64[106])!=0.0));
        self.scalar_static_bool[28]=(((self.scalar_static_f64[109])!=0.0)&&self.scalar_static_bool[27]);
        self.scalar_static_bool[29]=(!((self.scalar_static_f64[109])!=0.0));
        self.scalar_static_bool[30]=(self.scalar_static_bool[27]&&self.scalar_static_bool[29]);
        self.scalar_static_f64[110]=p.p47;
        self.scalar_static_f64[111]=p.p48;
        self.scalar_static_f64[112]=p.p46;
        self.scalar_static_bool[31]=(self.scalar_static_f64[105]>2.0);
        self.scalar_static_f64[113]=(2.0*self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=p.p37;
        self.scalar_static_f64[115]=p.p66;
        self.scalar_static_bool[32]=(self.scalar_static_f64[115]>0.0);
        self.scalar_static_bool[33]=(self.scalar_static_bool[0]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[116]=(if self.scalar_static_bool[33]{1.0}else{0.0});
        self.scalar_static_f64[117]=p.p67;
        self.scalar_static_f64[118]=(self.scalar_static_f64[117]/self.scalar_static_f64[15]);
        self.scalar_static_f64[119]=(self.scalar_static_f64[115]+self.scalar_static_f64[118]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[119]/self.scalar_static_f64[27]);
        self.scalar_static_f64[121]=(if ((self.scalar_static_f64[116])!=0.0){self.scalar_static_f64[120]}else{0.0});
        self.scalar_static_bool[34]=(!((self.scalar_static_f64[116])!=0.0));
        self.scalar_static_f64[122]=(if self.scalar_static_bool[34]{0.0}else{self.scalar_static_f64[121]});
        self.scalar_static_bool[35]=(self.scalar_static_bool[1]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[123]=(if self.scalar_static_bool[35]{1.0}else{0.0});
        self.scalar_static_f64[124]=(self.scalar_static_f64[119]/self.scalar_static_f64[28]);
        self.scalar_static_f64[125]=(if ((self.scalar_static_f64[123])!=0.0){self.scalar_static_f64[124]}else{0.0});
        self.scalar_static_bool[36]=(!((self.scalar_static_f64[123])!=0.0));
        self.scalar_static_f64[126]=(if self.scalar_static_bool[36]{0.0}else{self.scalar_static_f64[125]});
        self.scalar_static_bool[37]=(!((self.scalar_static_f64[100])!=0.0));
        self.scalar_static_f64[127]=p.p110;
        self.scalar_static_f64[128]=p.p111;
        self.scalar_static_f64[129]=(self.scalar_static_f64[31]*self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[127]+self.scalar_static_f64[129]);
        self.scalar_static_f64[131]=p.p112;
        self.scalar_static_f64[132]=(self.scalar_static_f64[25]*self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(self.scalar_static_f64[130]+self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=p.p113;
        self.scalar_static_f64[135]=(self.scalar_static_f64[27]+self.scalar_static_f64[28]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[134]*self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[133]+self.scalar_static_f64[136]);
        self.scalar_static_f64[138]=p.p109;
        self.scalar_static_f64[139]=p.p114;
        self.scalar_static_f64[140]=p.p115;
        self.scalar_static_f64[141]=(self.scalar_static_f64[31]*self.scalar_static_f64[140]);
        self.scalar_static_f64[142]=(self.scalar_static_f64[139]+self.scalar_static_f64[141]);
        self.scalar_static_f64[143]=p.p116;
        self.scalar_static_f64[144]=(self.scalar_static_f64[25]*self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=(self.scalar_static_f64[142]+self.scalar_static_f64[144]);
        self.scalar_static_f64[146]=p.p117;
        self.scalar_static_f64[147]=(self.scalar_static_f64[135]*self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[145]+self.scalar_static_f64[147]);
        self.scalar_static_f64[149]=(if self.scalar_static_bool[37]{self.scalar_static_f64[148]}else{0.0});
        self.scalar_static_f64[150]=p.p93;
        self.scalar_static_f64[151]=p.p97;
        self.scalar_static_f64[152]=p.p95;
        self.scalar_static_f64[153]=p.p99;
        self.scalar_static_f64[154]=p.p94;
        self.scalar_static_f64[155]=p.p98;
        self.scalar_static_f64[156]=p.p96;
        self.scalar_static_f64[157]=p.p100;
        self.scalar_static_f64[158]=p.p71;
        self.scalar_static_f64[159]=(self.scalar_static_f64[20]*self.scalar_static_f64[158]);
        self.scalar_static_f64[160]=p.p78;
        self.scalar_static_f64[161]=(self.scalar_static_f64[22]*self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=(self.scalar_static_f64[159]+self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[24]*self.scalar_static_f64[160]);
        self.scalar_static_f64[164]=(self.scalar_static_f64[159]+self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=p.p72;
        self.scalar_static_f64[166]=(self.scalar_static_f64[20]*self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=p.p79;
        self.scalar_static_f64[168]=(self.scalar_static_f64[22]*self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[166]+self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[24]*self.scalar_static_f64[167]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[166]+self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=p.p21;
        self.scalar_static_f64[173]=(-self.scalar_static_f64[172]);
        self.scalar_static_bool[38]=(!((self.scalar_static_f64[105])!=0.0));
        self.scalar_static_f64[174]=p.p101;
        self.scalar_static_f64[175]=p.p102;
        self.scalar_static_f64[176]=p.p92;
        self.scalar_static_f64[177]=p.p69;
        self.scalar_static_bool[39]=(self.scalar_static_f64[177]>0.0);
        self.scalar_static_f64[178]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_f64[179]=p.p90;
        self.scalar_static_f64[180]=(-self.scalar_static_f64[179]);
        self.scalar_static_f64[181]=p.p91;
        self.scalar_static_f64[182]=p.p70;
        self.scalar_static_f64[183]=p.p27;
        self.scalar_static_bool[40]=(!((self.scalar_static_f64[178])!=0.0));
        self.scalar_static_f64[184]=p.p76;
        self.scalar_static_bool[41]=(self.scalar_static_f64[184]>0.0);
        self.scalar_static_f64[185]=(if self.scalar_static_bool[41]{1.0}else{0.0});
        self.scalar_static_f64[186]=p.p77;
        self.scalar_static_bool[42]=(!((self.scalar_static_f64[185])!=0.0));
        self.scalar_static_bool[43]=(self.scalar_static_f64[165]>0.0);
        self.scalar_static_f64[187]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_f64[188]=p.p73;
        self.scalar_static_f64[189]=(0.5*self.scalar_static_f64[188]);
        self.scalar_static_f64[190]=(self.scalar_static_f64[188]* -0.5);
        self.scalar_static_f64[191]=p.p74;
        self.scalar_static_bool[44]=(!((self.scalar_static_f64[187])!=0.0));
        self.scalar_static_bool[45]=(self.scalar_static_f64[167]>0.0);
        self.scalar_static_f64[192]=(if self.scalar_static_bool[45]{1.0}else{0.0});
        self.scalar_static_f64[193]=p.p80;
        self.scalar_static_f64[194]=(0.5*self.scalar_static_f64[193]);
        self.scalar_static_f64[195]=(-0.5*self.scalar_static_f64[193]);
        self.scalar_static_f64[196]=p.p81;
        self.scalar_static_bool[46]=(!((self.scalar_static_f64[192])!=0.0));
        self.scalar_static_f64[197]=p.p83;
        self.scalar_static_bool[47]=(self.scalar_static_f64[197]>0.0);
        self.scalar_static_f64[198]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_f64[199]=p.p105;
        self.scalar_static_f64[200]=p.p106;
        self.scalar_static_f64[201]=p.p85;
        self.scalar_static_f64[202]=p.p107;
        self.scalar_static_f64[203]=p.p84;
        self.scalar_static_f64[204]=(self.scalar_static_f64[183]/self.scalar_static_f64[203]);
        self.scalar_static_bool[48]=(!((self.scalar_static_f64[198])!=0.0));
        self.scalar_static_f64[205]=p.p60;
        self.scalar_static_bool[49]=(self.scalar_static_f64[205]>0.0);
        self.scalar_static_bool[50]=(self.scalar_static_bool[37]&&self.scalar_static_bool[49]);
        self.scalar_static_f64[206]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[207]=p.p62;
        self.scalar_static_bool[51]=(((self.scalar_static_f64[206])!=0.0)&&((self.scalar_static_f64[207])!=0.0));
        self.scalar_static_f64[208]=p.p61;
        self.scalar_static_bool[52]=(!((self.scalar_static_f64[207])!=0.0));
        self.scalar_static_bool[53]=(((self.scalar_static_f64[206])!=0.0)&&self.scalar_static_bool[52]);
        self.scalar_static_f64[209]=p.p65;
        self.scalar_static_f64[210]=(4.0*self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[209]*self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(2.0*self.scalar_static_f64[209]);
        self.scalar_static_bool[54]=(!((self.scalar_static_f64[206])!=0.0));
        self.scalar_static_f64[213]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_f64[214]=(-self.scalar_static_f64[203]);
        self.scalar_static_f64[215]=p.p14;
        self.scalar_static_bool[55]=(0.0==self.scalar_static_f64[138]);
        self.scalar_static_f64[216]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_bool[56]=(!((self.scalar_static_f64[216])!=0.0));
        self.scalar_static_f64[217]=(1.0+self.scalar_static_f64[138]);
        self.scalar_static_f64[218]=(self.scalar_static_f64[217]).abs();
        self.scalar_static_bool[57]=(self.scalar_static_f64[218]>0.1);
        self.scalar_static_f64[219]=(if self.scalar_static_bool[57]{1.0}else{0.0});
        self.scalar_static_bool[58]=(!((self.scalar_static_f64[219])!=0.0));
        self.scalar_static_f64[220]=(0.5*self.scalar_static_f64[138]);
        self.scalar_static_bool[59]=(self.scalar_static_f64[169]>0.0);
        self.scalar_static_f64[221]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_bool[60]=(((self.scalar_static_f64[105])!=0.0)&&((self.scalar_static_f64[221])!=0.0));
        self.scalar_static_bool[61]=(self.scalar_static_bool[38]&&((self.scalar_static_f64[221])!=0.0));
        self.scalar_static_f64[222]=p.p68;
        self.scalar_static_f64[223]=p.p75;
        self.scalar_static_bool[62]=(self.scalar_static_f64[223]<=0.0);
        self.scalar_static_f64[224]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_f64[225]=(1.0-self.scalar_static_f64[222]);
        self.scalar_static_f64[226]=(-self.scalar_static_f64[191]);
        self.scalar_static_f64[227]=f64::powf(self.scalar_static_f64[225],self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=(1.0-self.scalar_static_f64[191]);
        self.scalar_static_f64[229]=(0.5*self.scalar_static_f64[191]);
        self.scalar_static_bool[63]=(!((self.scalar_static_f64[224])!=0.0));
        self.scalar_static_f64[230]=(4.0*self.scalar_static_f64[223]);
        self.scalar_static_f64[231]=(self.scalar_static_f64[223]*self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=p.p82;
        self.scalar_static_bool[64]=(self.scalar_static_f64[232]<=0.0);
        self.scalar_static_f64[233]=(if self.scalar_static_bool[64]{1.0}else{0.0});
        self.scalar_static_f64[234]=(-self.scalar_static_f64[196]);
        self.scalar_static_f64[235]=f64::powf(self.scalar_static_f64[225],self.scalar_static_f64[234]);
        self.scalar_static_f64[236]=(1.0-self.scalar_static_f64[196]);
        self.scalar_static_f64[237]=(0.5*self.scalar_static_f64[196]);
        self.scalar_static_bool[65]=(!((self.scalar_static_f64[233])!=0.0));
        self.scalar_static_f64[238]=(4.0*self.scalar_static_f64[232]);
        self.scalar_static_f64[239]=(self.scalar_static_f64[232]*self.scalar_static_f64[238]);
        self.scalar_static_bool[66]=(!((self.scalar_static_f64[221])!=0.0));
        self.scalar_static_bool[67]=(self.scalar_static_f64[171]>0.0);
        self.scalar_static_f64[240]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_bool[68]=(((self.scalar_static_f64[105])!=0.0)&&((self.scalar_static_f64[240])!=0.0));
        self.scalar_static_bool[69]=(self.scalar_static_bool[38]&&((self.scalar_static_f64[240])!=0.0));
        self.scalar_static_bool[70]=(!((self.scalar_static_f64[240])!=0.0));
        self.scalar_static_f64[241]=p.p26;
        self.scalar_static_f64[242]=(self.scalar_static_f64[176]-1.0);
        self.scalar_static_f64[243]=(self.scalar_static_f64[191]-1.0);
        self.scalar_static_f64[244]=(self.scalar_static_f64[196]-1.0);
        self.scalar_static_f64[245]=(self.scalar_static_f64[217]-1.0);
        self.scalar_static_f64[246]=(self.scalar_static_f64[228]-1.0);
        self.scalar_static_f64[247]=(self.scalar_static_f64[236]-1.0);
        self.scalar_static_f64[248]=(self.scalar_static_f64[162]*self.scalar_static_f64[173]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[162]*self.scalar_static_f64[172]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[164]*self.scalar_static_f64[173]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[164]*self.scalar_static_f64[172]);
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
        self.scalar_static_f64[252]=(temperature+self.scalar_static_f64[9]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[252]-273.15);
        self.scalar_static_bool[71]=(self.scalar_static_f64[253]<self.scalar_static_f64[11]);
        self.scalar_static_f64[254]=(if self.scalar_static_bool[71]{1.0}else{0.0});
        self.scalar_static_f64[255]=(self.scalar_static_f64[253]-self.scalar_static_f64[10]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[255]-1.0);
        self.scalar_static_f64[257]=(self.scalar_static_f64[256]).exp();
        self.scalar_static_f64[258]=(self.scalar_static_f64[10]+self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(if ((self.scalar_static_f64[254])!=0.0){self.scalar_static_f64[258]}else{self.scalar_static_f64[253]});
        self.scalar_static_bool[72]=(self.scalar_static_f64[259]>self.scalar_static_f64[13]);
        self.scalar_static_f64[260]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_bool[73]=(!((self.scalar_static_f64[254])!=0.0));
        self.scalar_static_bool[74]=(((self.scalar_static_f64[260])!=0.0)&&self.scalar_static_bool[73]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[12]-self.scalar_static_f64[259]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]-1.0);
        self.scalar_static_f64[263]=(self.scalar_static_f64[262]).exp();
        self.scalar_static_f64[264]=(self.scalar_static_f64[12]-self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=(if self.scalar_static_bool[74]{self.scalar_static_f64[264]}else{self.scalar_static_f64[259]});
        self.scalar_static_f64[266]=(273.15+self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[266]*1.3806505e-23);
        self.scalar_static_f64[268]=(self.scalar_static_f64[267]/1.60217653e-19);
        self.scalar_static_f64[269]=(self.scalar_static_f64[266]/self.scalar_static_f64[8]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[266]-self.scalar_static_f64[8]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[270]*self.scalar_static_f64[99]);
        self.scalar_static_f64[272]=(self.scalar_static_f64[98]+self.scalar_static_f64[271]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[270]*self.scalar_static_f64[272]);
        self.scalar_static_f64[274]=(1.0+self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[268]*self.scalar_static_f64[112]);
        self.scalar_static_f64[276]=(if ((self.scalar_static_f64[106])!=0.0){self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=(self.scalar_static_f64[268]*0.55);
        self.scalar_static_f64[278]=(self.scalar_static_f64[268]*1.1);
        self.scalar_static_f64[279]=(self.scalar_static_f64[268]*self.scalar_static_f64[113]);
        self.scalar_static_f64[280]=(if self.scalar_static_bool[28]{self.scalar_static_f64[279]}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[281]=(if self.scalar_static_bool[30]{self.scalar_static_f64[275]}else{self.scalar_static_f64[280]});
        self.scalar_static_f64[282]=f64::powf(self.scalar_static_f64[269],self.scalar_static_f64[138]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[137]*self.scalar_static_f64[282]);
        self.scalar_static_f64[284]=(if self.scalar_static_bool[37]{self.scalar_static_f64[283]}else{0.0});
        self.scalar_static_bool[75]=(self.scalar_static_f64[284]>0.0);
        self.scalar_static_bool[76]=(self.scalar_static_bool[75]&&((self.scalar_static_f64[215])!=0.0));
        self.scalar_static_bool[77]=(self.scalar_static_bool[37]&&self.scalar_static_bool[76]);
        self.scalar_static_f64[285]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_bool[78]=(((self.scalar_static_f64[285])!=0.0)&&((self.scalar_static_f64[216])!=0.0));
        self.scalar_static_bool[79]=(((self.scalar_static_f64[285])!=0.0)&&self.scalar_static_bool[56]);
        self.scalar_static_f64[286]=(if self.scalar_static_bool[79]{self.scalar_static_f64[253]}else{0.0});
        self.scalar_static_bool[80]=(self.scalar_static_f64[286]<self.scalar_static_f64[11]);
        self.scalar_static_f64[287]=(if self.scalar_static_bool[80]{1.0}else{0.0});
        self.scalar_static_bool[81]=(self.scalar_static_bool[79]&&((self.scalar_static_f64[287])!=0.0));
        self.scalar_static_f64[288]=(self.scalar_static_f64[286]-self.scalar_static_f64[10]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[288]-1.0);
        self.scalar_static_f64[290]=(self.scalar_static_f64[289]).exp();
        self.scalar_static_f64[291]=(self.scalar_static_f64[10]+self.scalar_static_f64[290]);
        self.scalar_static_f64[292]=(if self.scalar_static_bool[81]{self.scalar_static_f64[291]}else{self.scalar_static_f64[286]});
        self.scalar_static_bool[82]=(self.scalar_static_f64[292]>self.scalar_static_f64[13]);
        self.scalar_static_f64[293]=(if self.scalar_static_bool[82]{1.0}else{0.0});
        self.scalar_static_bool[83]=(!((self.scalar_static_f64[287])!=0.0));
        self.scalar_static_bool[84]=(self.scalar_static_bool[79]&&self.scalar_static_bool[83]);
        self.scalar_static_bool[85]=(((self.scalar_static_f64[293])!=0.0)&&self.scalar_static_bool[84]);
        self.scalar_static_f64[294]=(self.scalar_static_f64[12]-self.scalar_static_f64[292]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[294]-1.0);
        self.scalar_static_f64[296]=(self.scalar_static_f64[295]).exp();
        self.scalar_static_f64[297]=(self.scalar_static_f64[12]-self.scalar_static_f64[296]);
        self.scalar_static_f64[298]=(if self.scalar_static_bool[85]{self.scalar_static_f64[297]}else{self.scalar_static_f64[292]});
        self.scalar_static_f64[299]=(273.15+self.scalar_static_f64[298]);
        self.scalar_static_f64[300]=(if self.scalar_static_bool[79]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_bool[86]=(self.scalar_static_bool[79]&&((self.scalar_static_f64[219])!=0.0));
        self.scalar_static_f64[301]=(self.scalar_static_f64[284]*self.scalar_static_f64[300]);
        self.scalar_static_bool[87]=(self.scalar_static_bool[79]&&self.scalar_static_bool[58]);
        self.scalar_static_bool[88]=(!((self.scalar_static_f64[285])!=0.0));
        self.scalar_static_f64[302]=(if self.scalar_static_bool[78]{self.scalar_static_f64[284]}else{0.0});
        self.scalar_static_f64[303]=(1.0/self.scalar_static_f64[300]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[220]/self.scalar_static_f64[300]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
