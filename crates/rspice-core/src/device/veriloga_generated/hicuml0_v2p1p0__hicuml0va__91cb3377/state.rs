#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 112],
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
            const DEFAULTS_0: [f64; 10] = [
                210.0, 1e-16, 0.0, 1.0, 1.0, 1000000.0, 1000000.0, 0.0,
                2.0, 1000000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 10);
            {
                let params = &mut *ptr;
                params[10] = if (params[0] <= 200.0) { 1.0 } else { 0.0 };
                validate_parameter("fiqf", params[10], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 8] = [
                1000000.0, 1000000.0, 0.0, 0.0, 1e-18, 1.0, 0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(11), 8);
            {
                let params = &mut *ptr;
                params[19] = if (params[0] <= 200.0) { 0.0 } else { 1e-16 };
                validate_parameter("ibcs", params[19], false, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 76] = [
                1.0, 0.0, 0.0, 0.0, 2.5, 1000000.0, 0.0, 0.656,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1e-20, 0.9,
                0.5, 2.5, 0.9, 0.5, 2.5, 1e-20, 0.7, 0.333,
                100.0, 1e-20, 0.7, 0.333, 100.0, 1.0, 1e-20, 0.3,
                0.3, 100.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.1, 150.0, 0.5, 100.0, 0.1, 0.0, 0.001, 2.0,
                0.0, 0.0, 0.0, 0.167, 0.333, 0.0, 0.0, 2.0,
                1.2, 1.17, 1.17, 1.17, -0.000102377, 3.0, 3.5, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (*ptr).values.as_mut_ptr().add(20), 76);
            {
                let params = &mut *ptr;
                params[96] = if (params[0] <= 200.0) { 1.0 } else { 0.0 };
                validate_parameter("flteft", params[96], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 14] = [
                -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 27.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (*ptr).values.as_mut_ptr().add(97), 14);
            {
                let params = &mut *ptr;
                params[111] = 0.001;
                validate_parameter("minr", params[111], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 114] = [
    ("flcomp", 0), ("is", 1), ("flitm", 2), ("mcf", 3), ("mcr", 4), ("vef", 5), ("ver", 6), ("aver", 7), ("rver", 8), ("iqf", 9), ("fiqf", 10), ("iqr", 11), ("iqfh", 12), ("tfh", 13), ("ahq", 14), ("ibes", 15),
    ("mbe", 16), ("ires", 17), ("mre", 18), ("ibcs", 19), ("mbc", 20), ("favl", 21), ("qavl", 22), ("rbi0", 23), ("vr0e", 24), ("vr0c", 25), ("rbx", 26), ("fgeo", 27), ("re", 28), ("rcx", 29), ("itss", 30), ("msf", 31),
    ("iscs", 32), ("msc", 33), ("cje0", 34), ("vde", 35), ("ze", 36), ("aje", 37), ("vdedc", 38), ("zedc", 39), ("ajedc", 40), ("cjci0", 41), ("vdci", 42), ("zci", 43), ("vptci", 44), ("cjcx0", 45), ("vdcx", 46), ("zcx", 47),
    ("vptcx", 48), ("fbc", 49), ("cjs0", 50), ("vds", 51), ("zs", 52), ("vpts", 53), ("t0", 54), ("dt0h", 55), ("tbvl", 56), ("tef0", 57), ("gte", 58), ("thcs", 59), ("ahc", 60), ("rci0", 61), ("vlim", 62), ("vpt", 63),
    ("vces", 64), ("vdck", 65), ("aick", 66), ("delck", 67), ("tr", 68), ("cbepar", 69), ("cbcpar", 70), ("alqf", 71), ("alit", 72), ("flnqs", 73), ("kf", 74), ("af", 75), ("vgb", 76), ("vge", 77), ("vgc", 78), ("vgs", 79),
    ("f1vg", 80), ("zetact", 81), ("zetabet", 82), ("dvgbe", 83), ("zetavgbe", 84), ("alt0", 85), ("kt0", 86), ("zetaci", 87), ("alvs", 88), ("alces", 89), ("aldck", 90), ("zetarbi", 91), ("zetarbx", 92), ("zetarcx", 93), ("zetare", 94), ("zetaiqf", 95),
    ("flteft", 96), ("zetaver", 97), ("zetaiqfh", 98), ("alfav", 99), ("alqav", 100), ("aliqfh", 101), ("kiqfh", 102), ("flsh", 103), ("rth", 104), ("zetarth", 105), ("alrth", 106), ("cth", 107), ("tnom", 108), ("dt", 109), ("dtemp", 109), ("trise", 109),
    ("type", 110), ("minr", 111),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 112] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 112] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 112] = [
    "flcomp", "is", "flitm", "mcf", "mcr", "vef", "ver", "aver", "rver", "iqf", "fiqf", "iqr", "iqfh", "tfh", "ahq", "ibes",
    "mbe", "ires", "mre", "ibcs", "mbc", "favl", "qavl", "rbi0", "vr0e", "vr0c", "rbx", "fgeo", "re", "rcx", "itss", "msf",
    "iscs", "msc", "cje0", "vde", "ze", "aje", "vdedc", "zedc", "ajedc", "cjci0", "vdci", "zci", "vptci", "cjcx0", "vdcx", "zcx",
    "vptcx", "fbc", "cjs0", "vds", "zs", "vpts", "t0", "dt0h", "tbvl", "tef0", "gte", "thcs", "ahc", "rci0", "vlim", "vpt",
    "vces", "vdck", "aick", "delck", "tr", "cbepar", "cbcpar", "alqf", "alit", "flnqs", "kf", "af", "vgb", "vge", "vgc", "vgs",
    "f1vg", "zetact", "zetabet", "dvgbe", "zetavgbe", "alt0", "kt0", "zetaci", "alvs", "alces", "aldck", "zetarbi", "zetarbx", "zetarcx", "zetare", "zetaiqf",
    "flteft", "zetaver", "zetaiqfh", "alfav", "alqav", "aliqfh", "kiqfh", "flsh", "rth", "zetarth", "alrth", "cth", "tnom", "dt", "type", "minr",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 112] = [
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

const PARAMETER_INTEGER_FLAGS: [bool; 112] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 112] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -0.9, label: "-0.9" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 112] = [
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), None, Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, Some(ParameterBound { value: 1000000.0, label: "1000000.0" }),
    Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), None,
    Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 20.0, label: "20.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1000000.0, label: "1000000.0" }), Some(ParameterBound { value: 600.0, label: "600.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
];

const PARAMETER_RANGE_FLAGS: [u8; 112] = [
    2, 0, 2, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 2, 0, 0, 1, 0, 1, 0, 1, 2, 2, 2, 1, 1, 2, 0, 2, 2, 0, 1,
    0, 1, 3, 1, 3, 2, 1, 3, 2, 3, 1, 3, 1, 2, 1, 3, 1, 0, 2, 1, 3, 1, 2, 0, 2, 2, 1, 2, 1, 3, 1, 1,
    0, 0, 1, 1, 2, 2, 2, 1, 1, 0, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 1, 0, 0, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 112] = [
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
    &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[],
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
    pub nodes: [usize; 10],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 112]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<9, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<538, 76>>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 5;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 5] = ["ci", "bi", "ei", "nd_qf_nqs", "nd_itf_nqs"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 112;
    pub const VARIABLE_COUNT: usize = 386;
    pub const DDT_STATE_COUNT: usize = 9;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "291b89f487df12cd187a2258e2c278052293507cbaf129b4d550c04ec74b11c7";
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
        let mut values = Vec::with_capacity(45);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(9);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 45);
        debug_assert_eq!(state.flags.len(), 9);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hicumL0va'", name));
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
        self.scalar_static.f64_values[0]=p[110];
        self.scalar_static.f64_values[1]=p[108];
        self.scalar_static.f64_values[2]=(self.scalar_static.f64_values[1]+273.15);
        self.scalar_static.f64_values[3]=(self.scalar_static.f64_values[2]*1.3806226e-23);
        self.scalar_static.f64_values[4]=(self.scalar_static.f64_values[3]/1.602176462e-19);
        self.scalar_static.f64_values[5]=p[88];
        self.scalar_static.f64_values[6]=(self.scalar_static.f64_values[2]*self.scalar_static.f64_values[5]);
        self.scalar_static.f64_values[7]=p[76];
        self.scalar_static.f64_values[8]=p[77];
        self.scalar_static.f64_values[9]=(self.scalar_static.f64_values[7]+self.scalar_static.f64_values[8]);
        self.scalar_static.f64_values[10]=(0.5*self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[11]=p[78];
        self.scalar_static.f64_values[12]=(self.scalar_static.f64_values[7]+self.scalar_static.f64_values[11]);
        self.scalar_static.f64_values[13]=(0.5*self.scalar_static.f64_values[12]);
        self.scalar_static.f64_values[14]=p[79];
        self.scalar_static.f64_values[15]=(self.scalar_static.f64_values[11]+self.scalar_static.f64_values[14]);
        self.scalar_static.f64_values[16]=(0.5*self.scalar_static.f64_values[15]);
        self.scalar_static.f64_values[17]=p[80];
        self.scalar_static.f64_values[18]=(1.602176462e-19*self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[19]=(self.scalar_static.f64_values[18]/1.3806226e-23);
        self.scalar_static.f64_values[20]=(3.0-self.scalar_static.f64_values[19]);
        self.scalar_static.f64_values[21]=(self.scalar_static.f64_values[20]+1.0);
        self.scalar_static.f64_values[22]=p[87];
        self.scalar_static.f64_values[23]=(self.scalar_static.f64_values[21]-self.scalar_static.f64_values[22]);
        self.scalar_static.f64_values[24]=(self.scalar_static.f64_values[20]-1.5);
        self.scalar_static.f64_values[25]=p[82];
        self.scalar_static.f64_values[26]=p[81];
        self.scalar_static.f64_values[27]=(self.scalar_static.f64_values[25]-self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[28]=(self.scalar_static.f64_values[27]-0.5);
        self.scalar_static.f64_values[29]=(self.scalar_static.f64_values[7]-self.scalar_static.f64_values[8]);
        self.scalar_static.f64_values[30]=p[34];
        self.scalar_static.f64_values[31]=p[21];
        self.scalar_static.bool_values[0]=(self.scalar_static.f64_values[31]>0.0);
        self.scalar_static.f64_values[32]=p[41];
        self.scalar_static.bool_values[1]=(self.scalar_static.f64_values[32]>0.0);
        self.scalar_static.bool_values[2]=(self.scalar_static.bool_values[0]&&self.scalar_static.bool_values[1]);
        self.scalar_static.f64_values[33]=(if self.scalar_static.bool_values[2]{1.0}else{0.0});
        self.scalar_static.f64_values[34]=(if ((self.scalar_static.f64_values[33])!=0.0){1.0}else{0.0});
        self.scalar_static.bool_values[3]=(!((self.scalar_static.f64_values[33])!=0.0));
        self.scalar_static.f64_values[35]=(if self.scalar_static.bool_values[3]{0.0}else{self.scalar_static.f64_values[34]});
        self.scalar_static.f64_values[36]=p[109];
        self.scalar_static.f64_values[37]=p[35];
        self.scalar_static.f64_values[38]=(0.5*self.scalar_static.f64_values[37]);
        self.scalar_static.f64_values[39]=(self.scalar_static.f64_values[38]/self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[40]=(self.scalar_static.f64_values[4]*2.0);
        self.scalar_static.f64_values[41]=(self.scalar_static.f64_values[39]).exp();
        self.scalar_static.f64_values[42]=(-self.scalar_static.f64_values[39]);
        self.scalar_static.f64_values[43]=(self.scalar_static.f64_values[42]).exp();
        self.scalar_static.f64_values[44]=(self.scalar_static.f64_values[41]-self.scalar_static.f64_values[43]);
        self.scalar_static.f64_values[45]=(self.scalar_static.f64_values[44]).ln();
        self.scalar_static.f64_values[46]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[45]);
        self.scalar_static.f64_values[47]=p[36];
        self.scalar_static.f64_values[48]=p[37];
        self.scalar_static.f64_values[49]=p[38];
        self.scalar_static.f64_values[50]=(0.5*self.scalar_static.f64_values[49]);
        self.scalar_static.f64_values[51]=(self.scalar_static.f64_values[50]/self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[52]=(self.scalar_static.f64_values[51]).exp();
        self.scalar_static.f64_values[53]=(-self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[54]=(self.scalar_static.f64_values[53]).exp();
        self.scalar_static.f64_values[55]=(self.scalar_static.f64_values[52]-self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[56]=(self.scalar_static.f64_values[55]).ln();
        self.scalar_static.f64_values[57]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[56]);
        self.scalar_static.f64_values[58]=p[39];
        self.scalar_static.f64_values[59]=p[40];
        self.scalar_static.f64_values[60]=p[15];
        self.scalar_static.f64_values[61]=p[17];
        self.scalar_static.f64_values[62]=(0.5*self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[63]=(0.5*self.scalar_static.f64_values[10]);
        self.scalar_static.f64_values[64]=p[42];
        self.scalar_static.f64_values[65]=(0.5*self.scalar_static.f64_values[64]);
        self.scalar_static.f64_values[66]=(self.scalar_static.f64_values[65]/self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[67]=(self.scalar_static.f64_values[66]).exp();
        self.scalar_static.f64_values[68]=(-self.scalar_static.f64_values[66]);
        self.scalar_static.f64_values[69]=(self.scalar_static.f64_values[68]).exp();
        self.scalar_static.f64_values[70]=(self.scalar_static.f64_values[67]-self.scalar_static.f64_values[69]);
        self.scalar_static.f64_values[71]=(self.scalar_static.f64_values[70]).ln();
        self.scalar_static.f64_values[72]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[71]);
        self.scalar_static.f64_values[73]=p[43];
        self.scalar_static.f64_values[74]=p[19];
        self.scalar_static.f64_values[75]=p[1];
        self.scalar_static.f64_values[76]=p[9];
        self.scalar_static.f64_values[77]=p[95];
        self.scalar_static.f64_values[78]=p[83];
        self.scalar_static.f64_values[79]=p[62];
        self.scalar_static.f64_values[80]=(self.scalar_static.f64_values[22]-self.scalar_static.f64_values[6]);
        self.scalar_static.f64_values[81]=p[61];
        self.scalar_static.f64_values[82]=p[64];
        self.scalar_static.f64_values[83]=p[89];
        self.scalar_static.f64_values[84]=p[65];
        self.scalar_static.bool_values[4]=(self.scalar_static.f64_values[84]>0.0);
        self.scalar_static.f64_values[85]=(if self.scalar_static.bool_values[4]{1.0}else{0.0});
        self.scalar_static.f64_values[86]=p[90];
        self.scalar_static.bool_values[5]=(!((self.scalar_static.f64_values[85])!=0.0));
        self.scalar_static.f64_values[87]=p[54];
        self.scalar_static.f64_values[88]=p[85];
        self.scalar_static.f64_values[89]=p[86];
        self.scalar_static.f64_values[90]=p[96];
        self.scalar_static.bool_values[6]=(1.0==self.scalar_static.f64_values[90]);
        self.scalar_static.f64_values[91]=(if self.scalar_static.bool_values[6]{1.0}else{0.0});
        self.scalar_static.f64_values[92]=p[57];
        self.scalar_static.bool_values[7]=(!((self.scalar_static.f64_values[91])!=0.0));
        self.scalar_static.f64_values[93]=p[59];
        self.scalar_static.f64_values[94]=(self.scalar_static.f64_values[22]-1.0);
        self.scalar_static.bool_values[8]=(1.0==self.scalar_static.f64_values[35]);
        self.scalar_static.f64_values[95]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.f64_values[96]=p[99];
        self.scalar_static.f64_values[97]=p[22];
        self.scalar_static.f64_values[98]=p[100];
        self.scalar_static.bool_values[9]=(!((self.scalar_static.f64_values[95])!=0.0));
        self.scalar_static.f64_values[99]=p[23];
        self.scalar_static.f64_values[100]=p[91];
        self.scalar_static.f64_values[101]=p[46];
        self.scalar_static.f64_values[102]=(0.5*self.scalar_static.f64_values[101]);
        self.scalar_static.f64_values[103]=(self.scalar_static.f64_values[102]/self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[104]=(self.scalar_static.f64_values[103]).exp();
        self.scalar_static.f64_values[105]=(-self.scalar_static.f64_values[103]);
        self.scalar_static.f64_values[106]=(self.scalar_static.f64_values[105]).exp();
        self.scalar_static.f64_values[107]=(self.scalar_static.f64_values[104]-self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[108]=(self.scalar_static.f64_values[107]).ln();
        self.scalar_static.f64_values[109]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[108]);
        self.scalar_static.f64_values[110]=p[45];
        self.scalar_static.f64_values[111]=p[47];
        self.scalar_static.f64_values[112]=p[51];
        self.scalar_static.f64_values[113]=(0.5*self.scalar_static.f64_values[112]);
        self.scalar_static.f64_values[114]=(self.scalar_static.f64_values[113]/self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[115]=(self.scalar_static.f64_values[114]).exp();
        self.scalar_static.f64_values[116]=(-self.scalar_static.f64_values[114]);
        self.scalar_static.f64_values[117]=(self.scalar_static.f64_values[116]).exp();
        self.scalar_static.f64_values[118]=(self.scalar_static.f64_values[115]-self.scalar_static.f64_values[117]);
        self.scalar_static.f64_values[119]=(self.scalar_static.f64_values[118]).ln();
        self.scalar_static.f64_values[120]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[119]);
        self.scalar_static.f64_values[121]=p[50];
        self.scalar_static.f64_values[122]=p[52];
        self.scalar_static.f64_values[123]=p[32];
        self.scalar_static.f64_values[124]=p[30];
        self.scalar_static.f64_values[125]=p[7];
        self.scalar_static.f64_values[126]=p[97];
        self.scalar_static.f64_values[127]=p[6];
        self.scalar_static.f64_values[128]=p[84];
        self.scalar_static.f64_values[129]=p[0];
        self.scalar_static.bool_values[10]=(self.scalar_static.f64_values[129]<=200.0);
        self.scalar_static.f64_values[130]=(if self.scalar_static.bool_values[10]{1.0}else{0.0});
        self.scalar_static.f64_values[131]=p[101];
        self.scalar_static.f64_values[132]=p[102];
        self.scalar_static.bool_values[11]=(!((self.scalar_static.f64_values[130])!=0.0));
        self.scalar_static.f64_values[133]=p[98];
        self.scalar_static.f64_values[134]=p[12];
        self.scalar_static.f64_values[135]=p[13];
        self.scalar_static.f64_values[136]=p[14];
        self.scalar_static.f64_values[137]=p[29];
        self.scalar_static.f64_values[138]=p[93];
        self.scalar_static.f64_values[139]=p[26];
        self.scalar_static.f64_values[140]=p[92];
        self.scalar_static.f64_values[141]=p[28];
        self.scalar_static.f64_values[142]=p[94];
        self.scalar_static.f64_values[143]=p[104];
        self.scalar_static.f64_values[144]=p[105];
        self.scalar_static.f64_values[145]=p[106];
        self.scalar_static.f64_values[146]=p[103];
        self.scalar_static.bool_values[12]=(0.0!=self.scalar_static.f64_values[146]);
        self.scalar_static.f64_values[147]=p[111];
        self.scalar_static.bool_values[13]=(self.scalar_static.f64_values[143]>=self.scalar_static.f64_values[147]);
        self.scalar_static.bool_values[14]=(self.scalar_static.bool_values[12]&&self.scalar_static.bool_values[13]);
        self.scalar_static.f64_values[148]=(if self.scalar_static.bool_values[14]{1.0}else{0.0});
        self.scalar_static.f64_values[149]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[39]}else{self.scalar_static.f64_values[114]});
        self.scalar_static.f64_values[150]=(self.scalar_static.f64_values[149]).exp();
        self.scalar_static.f64_values[151]=(-self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[152]=(self.scalar_static.f64_values[151]).exp();
        self.scalar_static.f64_values[153]=(self.scalar_static.f64_values[150]-self.scalar_static.f64_values[152]);
        self.scalar_static.f64_values[154]=(self.scalar_static.f64_values[153]).ln();
        self.scalar_static.f64_values[155]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[154]);
        self.scalar_static.f64_values[156]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[155]}else{self.scalar_static.f64_values[120]});
        self.scalar_static.f64_values[157]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[51]}else{self.scalar_static.f64_values[149]});
        self.scalar_static.f64_values[158]=(self.scalar_static.f64_values[157]).exp();
        self.scalar_static.f64_values[159]=(-self.scalar_static.f64_values[157]);
        self.scalar_static.f64_values[160]=(self.scalar_static.f64_values[159]).exp();
        self.scalar_static.f64_values[161]=(self.scalar_static.f64_values[158]-self.scalar_static.f64_values[160]);
        self.scalar_static.f64_values[162]=(self.scalar_static.f64_values[161]).ln();
        self.scalar_static.f64_values[163]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[162]);
        self.scalar_static.f64_values[164]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[163]}else{self.scalar_static.f64_values[156]});
        self.scalar_static.f64_values[165]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[66]}else{self.scalar_static.f64_values[157]});
        self.scalar_static.f64_values[166]=(self.scalar_static.f64_values[165]).exp();
        self.scalar_static.f64_values[167]=(-self.scalar_static.f64_values[165]);
        self.scalar_static.f64_values[168]=(self.scalar_static.f64_values[167]).exp();
        self.scalar_static.f64_values[169]=(self.scalar_static.f64_values[166]-self.scalar_static.f64_values[168]);
        self.scalar_static.f64_values[170]=(self.scalar_static.f64_values[169]).ln();
        self.scalar_static.f64_values[171]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[170]);
        self.scalar_static.f64_values[172]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[171]}else{self.scalar_static.f64_values[164]});
        self.scalar_static.bool_values[15]=(((self.scalar_static.f64_values[85])!=0.0)&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[16]=(self.scalar_static.bool_values[5]&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[17]=(((self.scalar_static.f64_values[91])!=0.0)&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[18]=(self.scalar_static.bool_values[7]&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[19]=(((self.scalar_static.f64_values[95])!=0.0)&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[20]=(self.scalar_static.bool_values[9]&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.f64_values[173]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[103]}else{self.scalar_static.f64_values[165]});
        self.scalar_static.f64_values[174]=(self.scalar_static.f64_values[173]).exp();
        self.scalar_static.f64_values[175]=(-self.scalar_static.f64_values[173]);
        self.scalar_static.f64_values[176]=(self.scalar_static.f64_values[175]).exp();
        self.scalar_static.f64_values[177]=(self.scalar_static.f64_values[174]-self.scalar_static.f64_values[176]);
        self.scalar_static.f64_values[178]=(self.scalar_static.f64_values[177]).ln();
        self.scalar_static.f64_values[179]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[180]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[179]}else{self.scalar_static.f64_values[172]});
        self.scalar_static.f64_values[181]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[114]}else{self.scalar_static.f64_values[173]});
        self.scalar_static.f64_values[182]=(self.scalar_static.f64_values[181]).exp();
        self.scalar_static.f64_values[183]=(-self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[184]=(self.scalar_static.f64_values[183]).exp();
        self.scalar_static.f64_values[185]=(self.scalar_static.f64_values[182]-self.scalar_static.f64_values[184]);
        self.scalar_static.f64_values[186]=(self.scalar_static.f64_values[185]).ln();
        self.scalar_static.f64_values[187]=(self.scalar_static.f64_values[40]*self.scalar_static.f64_values[186]);
        self.scalar_static.f64_values[188]=(if ((self.scalar_static.f64_values[148])!=0.0){self.scalar_static.f64_values[187]}else{self.scalar_static.f64_values[180]});
        self.scalar_static.bool_values[21]=(((self.scalar_static.f64_values[130])!=0.0)&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.bool_values[22]=(self.scalar_static.bool_values[11]&&((self.scalar_static.f64_values[148])!=0.0));
        self.scalar_static.f64_values[189]=p[49];
        self.scalar_static.f64_values[190]=(1.0-self.scalar_static.f64_values[189]);
        self.scalar_static.f64_values[191]=p[44];
        self.scalar_static.bool_values[23]=(self.scalar_static.f64_values[191]<100.0);
        self.scalar_static.f64_values[192]=(if self.scalar_static.bool_values[23]{1.0}else{0.0});
        self.scalar_static.f64_values[193]=(self.scalar_static.f64_values[73]/4.0);
        self.scalar_static.f64_values[194]=(-0.8754687373538999/self.scalar_static.f64_values[73]);
        self.scalar_static.f64_values[195]=(self.scalar_static.f64_values[194]).exp();
        self.scalar_static.f64_values[196]=(1.0-self.scalar_static.f64_values[195]);
        self.scalar_static.f64_values[197]=(1.0-self.scalar_static.f64_values[73]);
        self.scalar_static.f64_values[198]=(-self.scalar_static.f64_values[73]);
        self.scalar_static.bool_values[24]=(!((self.scalar_static.f64_values[192])!=0.0));
        self.scalar_static.f64_values[199]=p[48];
        self.scalar_static.bool_values[25]=(self.scalar_static.f64_values[199]<100.0);
        self.scalar_static.f64_values[200]=(if self.scalar_static.bool_values[25]{1.0}else{0.0});
        self.scalar_static.f64_values[201]=(self.scalar_static.f64_values[111]/4.0);
        self.scalar_static.f64_values[202]=(-0.8754687373538999/self.scalar_static.f64_values[111]);
        self.scalar_static.f64_values[203]=(self.scalar_static.f64_values[202]).exp();
        self.scalar_static.f64_values[204]=(1.0-self.scalar_static.f64_values[203]);
        self.scalar_static.f64_values[205]=(1.0-self.scalar_static.f64_values[111]);
        self.scalar_static.f64_values[206]=(-self.scalar_static.f64_values[111]);
        self.scalar_static.bool_values[26]=(!((self.scalar_static.f64_values[200])!=0.0));
        self.scalar_static.f64_values[207]=p[67];
        self.scalar_static.f64_values[208]=p[63];
        self.scalar_static.f64_values[209]=p[66];
        self.scalar_static.f64_values[210]=(-self.scalar_static.f64_values[47]);
        self.scalar_static.f64_values[211]=(1.0-self.scalar_static.f64_values[47]);
        self.scalar_static.f64_values[212]=(-self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[213]=(1.0-self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[214]=(if ((self.scalar_static.f64_values[130])!=0.0){self.scalar_static.f64_values[58]}else{0.0});
        self.scalar_static.f64_values[215]=(if self.scalar_static.bool_values[11]{self.scalar_static.f64_values[47]}else{self.scalar_static.f64_values[214]});
        self.scalar_static.bool_values[27]=(0.0==self.scalar_static.f64_values[125]);
        self.scalar_static.f64_values[216]=(if self.scalar_static.bool_values[27]{1.0}else{0.0});
        self.scalar_static.f64_values[217]=(if ((self.scalar_static.f64_values[216])!=0.0){1.0}else{0.0});
        self.scalar_static.bool_values[28]=(!((self.scalar_static.f64_values[216])!=0.0));
        self.scalar_static.f64_values[218]=p[8];
        self.scalar_static.f64_values[219]=p[5];
        self.scalar_static.f64_values[220]=p[55];
        self.scalar_static.f64_values[221]=p[56];
        self.scalar_static.f64_values[222]=p[10];
        self.scalar_static.bool_values[29]=(1.0==self.scalar_static.f64_values[222]);
        self.scalar_static.f64_values[223]=(if self.scalar_static.bool_values[29]{1.0}else{0.0});
        self.scalar_static.bool_values[30]=(!((self.scalar_static.f64_values[223])!=0.0));
        self.scalar_static.f64_values[224]=p[11];
        self.scalar_static.f64_values[225]=p[3];
        self.scalar_static.f64_values[226]=p[4];
        self.scalar_static.bool_values[31]=(0.0!=self.scalar_static.f64_values[135]);
        self.scalar_static.f64_values[227]=(if self.scalar_static.bool_values[31]{1.0}else{0.0});
        self.scalar_static.bool_values[32]=(!((self.scalar_static.f64_values[227])!=0.0));
        self.scalar_static.f64_values[228]=(1.0+self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[229]=p[2];
        self.scalar_static.bool_values[33]=(0.0==self.scalar_static.f64_values[229]);
        self.scalar_static.f64_values[230]=(if self.scalar_static.bool_values[33]{1.0}else{0.0});
        self.scalar_static.bool_values[34]=(((self.scalar_static.f64_values[227])!=0.0)&&((self.scalar_static.f64_values[230])!=0.0));
        self.scalar_static.bool_values[35]=(self.scalar_static.bool_values[32]&&((self.scalar_static.f64_values[230])!=0.0));
        self.scalar_static.bool_values[36]=(!((self.scalar_static.f64_values[230])!=0.0));
        self.scalar_static.f64_values[231]=(if self.scalar_static.bool_values[36]{0.3333333333333333}else{0.0});
        self.scalar_static.bool_values[37]=(self.scalar_static.f64_values[76]==1000000.0);
        self.scalar_static.bool_values[38]=(self.scalar_static.f64_values[134]==1000000.0);
        self.scalar_static.bool_values[39]=(self.scalar_static.bool_values[37]&&self.scalar_static.bool_values[38]);
        self.scalar_static.f64_values[232]=(if self.scalar_static.bool_values[39]{1.0}else{0.0});
        self.scalar_static.bool_values[40]=(!((self.scalar_static.f64_values[232])!=0.0));
        self.scalar_static.bool_values[41]=(self.scalar_static.bool_values[36]&&self.scalar_static.bool_values[40]);
        self.scalar_static.f64_values[233]=p[60];
        self.scalar_static.f64_values[234]=(1.0+self.scalar_static.f64_values[233]);
        self.scalar_static.f64_values[235]=(self.scalar_static.f64_values[234]).sqrt();
        self.scalar_static.f64_values[236]=(1.0+self.scalar_static.f64_values[235]);
        self.scalar_static.f64_values[237]=p[58];
        self.scalar_static.f64_values[238]=(1.0+self.scalar_static.f64_values[237]);
        self.scalar_static.f64_values[239]=p[68];
        self.scalar_static.bool_values[42]=(self.scalar_static.f64_values[60]>0.0);
        self.scalar_static.f64_values[240]=(if self.scalar_static.bool_values[42]{1.0}else{0.0});
        self.scalar_static.f64_values[241]=p[16];
        self.scalar_static.bool_values[43]=(!((self.scalar_static.f64_values[240])!=0.0));
        self.scalar_static.bool_values[44]=(self.scalar_static.f64_values[61]>0.0);
        self.scalar_static.f64_values[242]=(if self.scalar_static.bool_values[44]{1.0}else{0.0});
        self.scalar_static.f64_values[243]=p[18];
        self.scalar_static.bool_values[45]=(!((self.scalar_static.f64_values[242])!=0.0));
        self.scalar_static.bool_values[46]=(self.scalar_static.f64_values[74]>0.0);
        self.scalar_static.f64_values[244]=(if self.scalar_static.bool_values[46]{1.0}else{0.0});
        self.scalar_static.f64_values[245]=p[20];
        self.scalar_static.bool_values[47]=(!((self.scalar_static.f64_values[244])!=0.0));
        self.scalar_static.f64_values[246]=p[24];
        self.scalar_static.f64_values[247]=p[25];
        self.scalar_static.f64_values[248]=p[27];
        self.scalar_static.bool_values[48]=(self.scalar_static.f64_values[124]>0.0);
        self.scalar_static.f64_values[249]=(if self.scalar_static.bool_values[48]{1.0}else{0.0});
        self.scalar_static.f64_values[250]=p[31];
        self.scalar_static.bool_values[49]=(!((self.scalar_static.f64_values[249])!=0.0));
        self.scalar_static.bool_values[50]=(self.scalar_static.f64_values[123]>0.0);
        self.scalar_static.f64_values[251]=(if self.scalar_static.bool_values[50]{1.0}else{0.0});
        self.scalar_static.f64_values[252]=p[33];
        self.scalar_static.bool_values[51]=(!((self.scalar_static.f64_values[251])!=0.0));
        self.scalar_static.f64_values[253]=p[53];
        self.scalar_static.bool_values[52]=(self.scalar_static.f64_values[253]<100.0);
        self.scalar_static.f64_values[254]=(if self.scalar_static.bool_values[52]{1.0}else{0.0});
        self.scalar_static.f64_values[255]=(self.scalar_static.f64_values[122]/4.0);
        self.scalar_static.f64_values[256]=(-0.8754687373538999/self.scalar_static.f64_values[122]);
        self.scalar_static.f64_values[257]=(self.scalar_static.f64_values[256]).exp();
        self.scalar_static.f64_values[258]=(1.0-self.scalar_static.f64_values[257]);
        self.scalar_static.f64_values[259]=(1.0-self.scalar_static.f64_values[122]);
        self.scalar_static.bool_values[53]=(!((self.scalar_static.f64_values[254])!=0.0));
        self.scalar_static.bool_values[54]=(1.0==self.scalar_static.f64_values[146]);
        self.scalar_static.bool_values[55]=(self.scalar_static.bool_values[13]&&self.scalar_static.bool_values[54]);
        self.scalar_static.f64_values[260]=(if self.scalar_static.bool_values[55]{1.0}else{0.0});
        self.scalar_static.f64_values[261]=p[73];
        self.scalar_static.bool_values[56]=(0.0!=self.scalar_static.f64_values[261]);
        self.scalar_static.bool_values[57]=(0.0!=self.scalar_static.f64_values[87]);
        self.scalar_static.bool_values[58]=(self.scalar_static.bool_values[56]&&self.scalar_static.bool_values[57]);
        self.scalar_static.f64_values[262]=(if self.scalar_static.bool_values[58]{1.0}else{0.0});
        self.scalar_static.f64_values[263]=p[71];
        self.scalar_static.f64_values[264]=p[72];
        self.scalar_static.bool_values[59]=(!((self.scalar_static.f64_values[262])!=0.0));
        self.scalar_static.f64_values[265]=p[70];
        self.scalar_static.f64_values[266]=p[69];
        self.scalar_static.bool_values[60]=(self.scalar_static.f64_values[141]>=self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[267]=(if self.scalar_static.bool_values[60]{1.0}else{0.0});
        self.scalar_static.bool_values[61]=(self.scalar_static.f64_values[137]>=self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[268]=(if self.scalar_static.bool_values[61]{1.0}else{0.0});
        self.scalar_static.bool_values[62]=(self.scalar_static.f64_values[99]>=self.scalar_static.f64_values[147]);
        self.scalar_static.bool_values[63]=(self.scalar_static.f64_values[139]>=self.scalar_static.f64_values[147]);
        self.scalar_static.bool_values[64]=(self.scalar_static.bool_values[62]||self.scalar_static.bool_values[63]);
        self.scalar_static.f64_values[269]=(if self.scalar_static.bool_values[64]{1.0}else{0.0});
        self.scalar_static.bool_values[65]=(0.0==self.scalar_static.f64_values[146]);
        self.scalar_static.f64_values[270]=p[107];
        self.scalar_static.bool_values[66]=(0.0==self.scalar_static.f64_values[270]);
        self.scalar_static.bool_values[67]=(self.scalar_static.bool_values[65]||self.scalar_static.bool_values[66]);
        self.scalar_static.f64_values[271]=(if self.scalar_static.bool_values[67]{1.0}else{0.0});
        self.scalar_static.bool_values[68]=(!((self.scalar_static.f64_values[271])!=0.0));
        self.scalar_static.bool_values[69]=(self.scalar_static.f64_values[143]<self.scalar_static.f64_values[147]);
        self.scalar_static.bool_values[70]=(self.scalar_static.bool_values[65]||self.scalar_static.bool_values[69]);
        self.scalar_static.f64_values[272]=(if self.scalar_static.bool_values[70]{1.0}else{0.0});
        self.scalar_static.bool_values[71]=(!((self.scalar_static.f64_values[272])!=0.0));
        self.scalar_static.f64_values[273]=(-self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[274]=(self.scalar_static.f64_values[0]-self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[275]=(if ((self.scalar_static.f64_values[148])!=0.0){1.0}else{0.0});
        self.scalar_static.f64_values[276]=(if ((self.scalar_static.f64_values[85])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[277]=(if ((self.scalar_static.f64_values[85])!=0.0){self.scalar_static.f64_values[273]}else{0.0});
        self.scalar_static.f64_values[278]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[0]}else{self.scalar_static.f64_values[276]});
        self.scalar_static.f64_values[279]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[274]}else{self.scalar_static.f64_values[277]});
        self.scalar_static.f64_values[280]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[273]}else{0.0});
        self.scalar_static.f64_values[281]=(if ((self.scalar_static.f64_values[95])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[282]=(if ((self.scalar_static.f64_values[95])!=0.0){self.scalar_static.f64_values[273]}else{0.0});
        self.scalar_static.f64_values[283]=(if ((self.scalar_static.f64_values[262])!=0.0){1.0}else{0.0});
        self.scalar_static.f64_values[284]=(if ((self.scalar_static.f64_values[262])!=0.0){self.scalar_static.f64_values[283]}else{0.0});
        self.scalar_static.f64_values[285]=(self.scalar_static.f64_values[263]*self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[286]=(self.scalar_static.f64_values[87]*self.scalar_static.f64_values[285]);
        self.scalar_static.f64_values[287]=(if ((self.scalar_static.f64_values[262])!=0.0){self.scalar_static.f64_values[286]}else{0.0});
        self.scalar_static.f64_values[288]=(self.scalar_static.f64_values[264]*self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[289]=(self.scalar_static.f64_values[87]*self.scalar_static.f64_values[288]);
        self.scalar_static.f64_values[290]=(if ((self.scalar_static.f64_values[262])!=0.0){self.scalar_static.f64_values[289]}else{0.0});
        self.scalar_static.f64_values[291]=(if self.scalar_static.bool_values[59]{1.0}else{self.scalar_static.f64_values[284]});
        self.scalar_static.f64_values[292]=(if self.scalar_static.bool_values[59]{0.0}else{self.scalar_static.f64_values[287]});
        self.scalar_static.f64_values[293]=(if self.scalar_static.bool_values[59]{0.0}else{self.scalar_static.f64_values[290]});
        self.scalar_static.f64_values[294]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[265]);
        self.scalar_static.f64_values[295]=(self.scalar_static.f64_values[265]*self.scalar_static.f64_values[273]);
        self.scalar_static.f64_values[296]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[297]=(self.scalar_static.f64_values[266]*self.scalar_static.f64_values[273]);
        self.scalar_static.f64_values[298]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[294]);
        self.scalar_static.f64_values[299]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[295]);
        self.scalar_static.f64_values[300]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[296]);
        self.scalar_static.f64_values[301]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[297]);
        self.scalar_static.f64_values[302]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[283]);
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
        self.scalar_static.f64_values[303]=(temperature+self.scalar_static.f64_values[36]);
        self.scalar_static.bool_values[72]=(self.scalar_static.f64_values[303]<173.14999999999998);
        self.scalar_static.f64_values[304]=(if self.scalar_static.bool_values[72]{1.0}else{0.0});
        self.scalar_static.f64_values[305]=(if ((self.scalar_static.f64_values[304])!=0.0){173.14999999999998}else{self.scalar_static.f64_values[303]});
        self.scalar_static.bool_values[73]=(self.scalar_static.f64_values[305]>600.0);
        self.scalar_static.f64_values[306]=(if self.scalar_static.bool_values[73]{1.0}else{0.0});
        self.scalar_static.bool_values[74]=(!((self.scalar_static.f64_values[304])!=0.0));
        self.scalar_static.bool_values[75]=(((self.scalar_static.f64_values[306])!=0.0)&&self.scalar_static.bool_values[74]);
        self.scalar_static.f64_values[307]=(if self.scalar_static.bool_values[75]{600.0}else{self.scalar_static.f64_values[305]});
        self.scalar_static.f64_values[308]=(1.3806226e-23*self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[309]=(self.scalar_static.f64_values[308]/1.602176462e-19);
        self.scalar_static.f64_values[310]=(1.0/self.scalar_static.f64_values[309]);
        self.scalar_static.f64_values[311]=(self.scalar_static.f64_values[307]-self.scalar_static.f64_values[2]);
        self.scalar_static.f64_values[312]=(self.scalar_static.f64_values[307]/self.scalar_static.f64_values[2]);
        self.scalar_static.f64_values[313]=(self.scalar_static.f64_values[312]).ln();
        self.scalar_static.f64_values[314]=(self.scalar_static.f64_values[312]-1.0);
        self.scalar_static.f64_values[315]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[314]);
        self.scalar_static.f64_values[316]=(self.scalar_static.f64_values[312]*self.scalar_static.f64_values[46]);
        self.scalar_static.f64_values[317]=(1.0-self.scalar_static.f64_values[312]);
        self.scalar_static.f64_values[318]=(self.scalar_static.f64_values[10]*self.scalar_static.f64_values[317]);
        self.scalar_static.f64_values[319]=(self.scalar_static.f64_values[316]+self.scalar_static.f64_values[318]);
        self.scalar_static.f64_values[320]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[309]);
        self.scalar_static.f64_values[321]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[320]);
        self.scalar_static.f64_values[322]=(self.scalar_static.f64_values[319]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[323]=(self.scalar_static.f64_values[309]*2.0);
        self.scalar_static.f64_values[324]=(-self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[325]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[324]);
        self.scalar_static.f64_values[326]=(self.scalar_static.f64_values[325]).exp();
        self.scalar_static.f64_values[327]=(4.0*self.scalar_static.f64_values[326]);
        self.scalar_static.f64_values[328]=(1.0+self.scalar_static.f64_values[327]);
        self.scalar_static.f64_values[329]=(self.scalar_static.f64_values[328]).sqrt();
        self.scalar_static.f64_values[330]=(1.0+self.scalar_static.f64_values[329]);
        self.scalar_static.f64_values[331]=(0.5*self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[332]=(self.scalar_static.f64_values[331]).ln();
        self.scalar_static.f64_values[333]=(self.scalar_static.f64_values[323]*self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[322]+self.scalar_static.f64_values[333]);
        self.scalar_static.f64_values[335]=(self.scalar_static.f64_values[37]/self.scalar_static.f64_values[334]);
        self.scalar_static.f64_values[336]=(self.scalar_static.f64_values[335]).ln();
        self.scalar_static.f64_values[337]=(self.scalar_static.f64_values[47]*self.scalar_static.f64_values[336]);
        self.scalar_static.f64_values[338]=(self.scalar_static.f64_values[337]).exp();
        self.scalar_static.f64_values[339]=(self.scalar_static.f64_values[30]*self.scalar_static.f64_values[338]);
        self.scalar_static.f64_values[340]=(self.scalar_static.f64_values[334]*self.scalar_static.f64_values[48]);
        self.scalar_static.f64_values[341]=(self.scalar_static.f64_values[340]/self.scalar_static.f64_values[37]);
        self.scalar_static.f64_values[342]=(self.scalar_static.f64_values[312]*self.scalar_static.f64_values[57]);
        self.scalar_static.f64_values[343]=(self.scalar_static.f64_values[318]+self.scalar_static.f64_values[342]);
        self.scalar_static.f64_values[344]=(self.scalar_static.f64_values[343]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[345]=(-self.scalar_static.f64_values[344]);
        self.scalar_static.f64_values[346]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[345]);
        self.scalar_static.f64_values[347]=(self.scalar_static.f64_values[346]).exp();
        self.scalar_static.f64_values[348]=(4.0*self.scalar_static.f64_values[347]);
        self.scalar_static.f64_values[349]=(1.0+self.scalar_static.f64_values[348]);
        self.scalar_static.f64_values[350]=(self.scalar_static.f64_values[349]).sqrt();
        self.scalar_static.f64_values[351]=(1.0+self.scalar_static.f64_values[350]);
        self.scalar_static.f64_values[352]=(0.5*self.scalar_static.f64_values[351]);
        self.scalar_static.f64_values[353]=(self.scalar_static.f64_values[352]).ln();
        self.scalar_static.f64_values[354]=(self.scalar_static.f64_values[323]*self.scalar_static.f64_values[353]);
        self.scalar_static.f64_values[355]=(self.scalar_static.f64_values[344]+self.scalar_static.f64_values[354]);
        self.scalar_static.f64_values[356]=(self.scalar_static.f64_values[49]/self.scalar_static.f64_values[355]);
        self.scalar_static.f64_values[357]=(self.scalar_static.f64_values[356]).ln();
        self.scalar_static.f64_values[358]=(self.scalar_static.f64_values[58]*self.scalar_static.f64_values[357]);
        self.scalar_static.f64_values[359]=(self.scalar_static.f64_values[358]).exp();
        self.scalar_static.f64_values[360]=(self.scalar_static.f64_values[30]*self.scalar_static.f64_values[359]);
        self.scalar_static.f64_values[361]=(self.scalar_static.f64_values[355]*self.scalar_static.f64_values[59]);
        self.scalar_static.f64_values[362]=(self.scalar_static.f64_values[361]/self.scalar_static.f64_values[49]);
        self.scalar_static.f64_values[363]=(self.scalar_static.f64_values[25]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[364]=(self.scalar_static.f64_values[8]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[365]=(self.scalar_static.f64_values[363]+self.scalar_static.f64_values[364]);
        self.scalar_static.f64_values[366]=(self.scalar_static.f64_values[365]).exp();
        self.scalar_static.f64_values[367]=(self.scalar_static.f64_values[60]*self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[368]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[62]);
        self.scalar_static.f64_values[369]=(self.scalar_static.f64_values[315]*self.scalar_static.f64_values[63]);
        self.scalar_static.f64_values[370]=(self.scalar_static.f64_values[368]+self.scalar_static.f64_values[369]);
        self.scalar_static.f64_values[371]=(self.scalar_static.f64_values[370]).exp();
        self.scalar_static.f64_values[372]=(self.scalar_static.f64_values[61]*self.scalar_static.f64_values[371]);
        self.scalar_static.f64_values[373]=(self.scalar_static.f64_values[312]*self.scalar_static.f64_values[72]);
        self.scalar_static.f64_values[374]=(self.scalar_static.f64_values[13]*self.scalar_static.f64_values[317]);
        self.scalar_static.f64_values[375]=(self.scalar_static.f64_values[373]+self.scalar_static.f64_values[374]);
        self.scalar_static.f64_values[376]=(self.scalar_static.f64_values[375]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[377]=(-self.scalar_static.f64_values[376]);
        self.scalar_static.f64_values[378]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[377]);
        self.scalar_static.f64_values[379]=(self.scalar_static.f64_values[378]).exp();
        self.scalar_static.f64_values[380]=(4.0*self.scalar_static.f64_values[379]);
        self.scalar_static.f64_values[381]=(1.0+self.scalar_static.f64_values[380]);
        self.scalar_static.f64_values[382]=(self.scalar_static.f64_values[381]).sqrt();
        self.scalar_static.f64_values[383]=(1.0+self.scalar_static.f64_values[382]);
        self.scalar_static.f64_values[384]=(0.5*self.scalar_static.f64_values[383]);
        self.scalar_static.f64_values[385]=(self.scalar_static.f64_values[384]).ln();
        self.scalar_static.f64_values[386]=(self.scalar_static.f64_values[323]*self.scalar_static.f64_values[385]);
        self.scalar_static.f64_values[387]=(self.scalar_static.f64_values[376]+self.scalar_static.f64_values[386]);
        self.scalar_static.f64_values[388]=(self.scalar_static.f64_values[64]/self.scalar_static.f64_values[387]);
        self.scalar_static.f64_values[389]=(self.scalar_static.f64_values[388]).ln();
        self.scalar_static.f64_values[390]=(self.scalar_static.f64_values[73]*self.scalar_static.f64_values[389]);
        self.scalar_static.f64_values[391]=(self.scalar_static.f64_values[390]).exp();
        self.scalar_static.f64_values[392]=(self.scalar_static.f64_values[32]*self.scalar_static.f64_values[391]);
        self.scalar_static.f64_values[393]=(self.scalar_static.f64_values[23]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[394]=(self.scalar_static.f64_values[11]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[395]=(self.scalar_static.f64_values[393]+self.scalar_static.f64_values[394]);
        self.scalar_static.f64_values[396]=(self.scalar_static.f64_values[395]).exp();
        self.scalar_static.f64_values[397]=(self.scalar_static.f64_values[74]*self.scalar_static.f64_values[396]);
        self.scalar_static.f64_values[398]=(self.scalar_static.f64_values[26]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[399]=(self.scalar_static.f64_values[7]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[400]=(self.scalar_static.f64_values[398]+self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[401]=(self.scalar_static.f64_values[400]).exp();
        self.scalar_static.f64_values[402]=(self.scalar_static.f64_values[75]*self.scalar_static.f64_values[401]);
        self.scalar_static.f64_values[403]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[77]);
        self.scalar_static.f64_values[404]=(self.scalar_static.f64_values[315]*self.scalar_static.f64_values[78]);
        self.scalar_static.f64_values[405]=(self.scalar_static.f64_values[403]-self.scalar_static.f64_values[404]);
        self.scalar_static.f64_values[406]=(self.scalar_static.f64_values[405]).exp();
        self.scalar_static.f64_values[407]=(self.scalar_static.f64_values[76]*self.scalar_static.f64_values[406]);
        self.scalar_static.f64_values[408]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[80]);
        self.scalar_static.f64_values[409]=(self.scalar_static.f64_values[408]).exp();
        self.scalar_static.f64_values[410]=(self.scalar_static.f64_values[79]*self.scalar_static.f64_values[409]);
        self.scalar_static.f64_values[411]=(self.scalar_static.f64_values[22]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[412]=(self.scalar_static.f64_values[411]).exp();
        self.scalar_static.f64_values[413]=(self.scalar_static.f64_values[81]*self.scalar_static.f64_values[412]);
        self.scalar_static.f64_values[414]=(1.0/self.scalar_static.f64_values[413]);
        self.scalar_static.f64_values[415]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[83]);
        self.scalar_static.f64_values[416]=(1.0+self.scalar_static.f64_values[415]);
        self.scalar_static.f64_values[417]=(self.scalar_static.f64_values[82]*self.scalar_static.f64_values[416]);
        self.scalar_static.f64_values[418]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[86]);
        self.scalar_static.f64_values[419]=(1.0-self.scalar_static.f64_values[418]);
        self.scalar_static.f64_values[420]=(self.scalar_static.f64_values[84]*self.scalar_static.f64_values[419]);
        self.scalar_static.f64_values[421]=(if ((self.scalar_static.f64_values[85])!=0.0){self.scalar_static.f64_values[420]}else{0.0});
        self.scalar_static.f64_values[422]=(if ((self.scalar_static.f64_values[85])!=0.0){self.scalar_static.f64_values[82]}else{self.scalar_static.f64_values[417]});
        self.scalar_static.f64_values[423]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[417]}else{self.scalar_static.f64_values[422]});
        self.scalar_static.f64_values[424]=(if self.scalar_static.bool_values[5]{self.scalar_static.f64_values[84]}else{self.scalar_static.f64_values[421]});
        self.scalar_static.f64_values[425]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[426]=(1.0+self.scalar_static.f64_values[425]);
        self.scalar_static.f64_values[427]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[89]);
        self.scalar_static.f64_values[428]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[427]);
        self.scalar_static.f64_values[429]=(self.scalar_static.f64_values[426]+self.scalar_static.f64_values[428]);
        self.scalar_static.f64_values[430]=(self.scalar_static.f64_values[87]*self.scalar_static.f64_values[429]);
        self.scalar_static.f64_values[431]=(self.scalar_static.f64_values[28]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[432]=(self.scalar_static.f64_values[29]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[433]=(self.scalar_static.f64_values[431]-self.scalar_static.f64_values[432]);
        self.scalar_static.f64_values[434]=(self.scalar_static.f64_values[433]).exp();
        self.scalar_static.f64_values[435]=(self.scalar_static.f64_values[92]*self.scalar_static.f64_values[434]);
        self.scalar_static.f64_values[436]=(if ((self.scalar_static.f64_values[91])!=0.0){self.scalar_static.f64_values[435]}else{0.0});
        self.scalar_static.f64_values[437]=(if self.scalar_static.bool_values[7]{self.scalar_static.f64_values[92]}else{self.scalar_static.f64_values[436]});
        self.scalar_static.f64_values[438]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[94]);
        self.scalar_static.f64_values[439]=(self.scalar_static.f64_values[438]).exp();
        self.scalar_static.f64_values[440]=(self.scalar_static.f64_values[93]*self.scalar_static.f64_values[439]);
        self.scalar_static.f64_values[441]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[96]);
        self.scalar_static.f64_values[442]=(self.scalar_static.f64_values[441]).exp();
        self.scalar_static.f64_values[443]=(self.scalar_static.f64_values[31]*self.scalar_static.f64_values[442]);
        self.scalar_static.f64_values[444]=(if ((self.scalar_static.f64_values[95])!=0.0){self.scalar_static.f64_values[443]}else{0.0});
        self.scalar_static.f64_values[445]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[98]);
        self.scalar_static.f64_values[446]=(self.scalar_static.f64_values[445]).exp();
        self.scalar_static.f64_values[447]=(self.scalar_static.f64_values[97]*self.scalar_static.f64_values[446]);
        self.scalar_static.f64_values[448]=(if ((self.scalar_static.f64_values[95])!=0.0){self.scalar_static.f64_values[447]}else{0.0});
        self.scalar_static.f64_values[449]=(if self.scalar_static.bool_values[9]{self.scalar_static.f64_values[31]}else{self.scalar_static.f64_values[444]});
        self.scalar_static.f64_values[450]=(if self.scalar_static.bool_values[9]{self.scalar_static.f64_values[97]}else{self.scalar_static.f64_values[448]});
        self.scalar_static.f64_values[451]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[452]=(self.scalar_static.f64_values[451]).exp();
        self.scalar_static.f64_values[453]=(self.scalar_static.f64_values[99]*self.scalar_static.f64_values[452]);
        self.scalar_static.f64_values[454]=(self.scalar_static.f64_values[312]*self.scalar_static.f64_values[109]);
        self.scalar_static.f64_values[455]=(self.scalar_static.f64_values[374]+self.scalar_static.f64_values[454]);
        self.scalar_static.f64_values[456]=(self.scalar_static.f64_values[455]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[457]=(-self.scalar_static.f64_values[456]);
        self.scalar_static.f64_values[458]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[457]);
        self.scalar_static.f64_values[459]=(self.scalar_static.f64_values[458]).exp();
        self.scalar_static.f64_values[460]=(4.0*self.scalar_static.f64_values[459]);
        self.scalar_static.f64_values[461]=(1.0+self.scalar_static.f64_values[460]);
        self.scalar_static.f64_values[462]=(self.scalar_static.f64_values[461]).sqrt();
        self.scalar_static.f64_values[463]=(1.0+self.scalar_static.f64_values[462]);
        self.scalar_static.f64_values[464]=(0.5*self.scalar_static.f64_values[463]);
        self.scalar_static.f64_values[465]=(self.scalar_static.f64_values[464]).ln();
        self.scalar_static.f64_values[466]=(self.scalar_static.f64_values[323]*self.scalar_static.f64_values[465]);
        self.scalar_static.f64_values[467]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[466]);
        self.scalar_static.f64_values[468]=(self.scalar_static.f64_values[101]/self.scalar_static.f64_values[467]);
        self.scalar_static.f64_values[469]=(self.scalar_static.f64_values[468]).ln();
        self.scalar_static.f64_values[470]=(self.scalar_static.f64_values[111]*self.scalar_static.f64_values[469]);
        self.scalar_static.f64_values[471]=(self.scalar_static.f64_values[470]).exp();
        self.scalar_static.f64_values[472]=(self.scalar_static.f64_values[110]*self.scalar_static.f64_values[471]);
        self.scalar_static.f64_values[473]=(self.scalar_static.f64_values[312]*self.scalar_static.f64_values[120]);
        self.scalar_static.f64_values[474]=(self.scalar_static.f64_values[16]*self.scalar_static.f64_values[317]);
        self.scalar_static.f64_values[475]=(self.scalar_static.f64_values[473]+self.scalar_static.f64_values[474]);
        self.scalar_static.f64_values[476]=(self.scalar_static.f64_values[475]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[477]=(-self.scalar_static.f64_values[476]);
        self.scalar_static.f64_values[478]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[477]);
        self.scalar_static.f64_values[479]=(self.scalar_static.f64_values[478]).exp();
        self.scalar_static.f64_values[480]=(4.0*self.scalar_static.f64_values[479]);
        self.scalar_static.f64_values[481]=(1.0+self.scalar_static.f64_values[480]);
        self.scalar_static.f64_values[482]=(self.scalar_static.f64_values[481]).sqrt();
        self.scalar_static.f64_values[483]=(1.0+self.scalar_static.f64_values[482]);
        self.scalar_static.f64_values[484]=(0.5*self.scalar_static.f64_values[483]);
        self.scalar_static.f64_values[485]=(self.scalar_static.f64_values[484]).ln();
        self.scalar_static.f64_values[486]=(self.scalar_static.f64_values[323]*self.scalar_static.f64_values[485]);
        self.scalar_static.f64_values[487]=(self.scalar_static.f64_values[476]+self.scalar_static.f64_values[486]);
        self.scalar_static.f64_values[488]=(self.scalar_static.f64_values[112]/self.scalar_static.f64_values[487]);
        self.scalar_static.f64_values[489]=(self.scalar_static.f64_values[488]).ln();
        self.scalar_static.f64_values[490]=(self.scalar_static.f64_values[122]*self.scalar_static.f64_values[489]);
        self.scalar_static.f64_values[491]=(self.scalar_static.f64_values[490]).exp();
        self.scalar_static.f64_values[492]=(self.scalar_static.f64_values[121]*self.scalar_static.f64_values[491]);
        self.scalar_static.f64_values[493]=(self.scalar_static.f64_values[24]*self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[494]=(self.scalar_static.f64_values[14]*self.scalar_static.f64_values[315]);
        self.scalar_static.f64_values[495]=(self.scalar_static.f64_values[493]+self.scalar_static.f64_values[494]);
        self.scalar_static.f64_values[496]=(self.scalar_static.f64_values[495]).exp();
        self.scalar_static.f64_values[497]=(self.scalar_static.f64_values[123]*self.scalar_static.f64_values[496]);
        self.scalar_static.f64_values[498]=(self.scalar_static.f64_values[394]+self.scalar_static.f64_values[493]);
        self.scalar_static.f64_values[499]=(self.scalar_static.f64_values[498]).exp();
        self.scalar_static.f64_values[500]=(self.scalar_static.f64_values[124]*self.scalar_static.f64_values[499]);
        self.scalar_static.f64_values[501]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[126]);
        self.scalar_static.f64_values[502]=(self.scalar_static.f64_values[501]).exp();
        self.scalar_static.f64_values[503]=(self.scalar_static.f64_values[125]*self.scalar_static.f64_values[502]);
        self.scalar_static.f64_values[504]=(self.scalar_static.f64_values[310]*self.scalar_static.f64_values[78]);
        self.scalar_static.f64_values[505]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[506]=(self.scalar_static.f64_values[505]).exp();
        self.scalar_static.f64_values[507]=(self.scalar_static.f64_values[506]-1.0);
        self.scalar_static.f64_values[508]=(self.scalar_static.f64_values[504]*self.scalar_static.f64_values[507]);
        self.scalar_static.f64_values[509]=(self.scalar_static.f64_values[508]).exp();
        self.scalar_static.f64_values[510]=(self.scalar_static.f64_values[127]/self.scalar_static.f64_values[509]);
        self.scalar_static.f64_values[511]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[132]);
        self.scalar_static.f64_values[512]=(self.scalar_static.f64_values[131]+self.scalar_static.f64_values[511]);
        self.scalar_static.f64_values[513]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[512]);
        self.scalar_static.f64_values[514]=(1.0+self.scalar_static.f64_values[513]);
        self.scalar_static.f64_values[515]=(if ((self.scalar_static.f64_values[130])!=0.0){self.scalar_static.f64_values[514]}else{0.0});
        self.scalar_static.f64_values[516]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[517]=(self.scalar_static.f64_values[516]).exp();
        self.scalar_static.f64_values[518]=(if self.scalar_static.bool_values[11]{self.scalar_static.f64_values[517]}else{self.scalar_static.f64_values[515]});
        self.scalar_static.f64_values[519]=(self.scalar_static.f64_values[518]*self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[520]=(self.scalar_static.f64_values[518]*self.scalar_static.f64_values[135]);
        self.scalar_static.f64_values[521]=(self.scalar_static.f64_values[432]).exp();
        self.scalar_static.f64_values[522]=(self.scalar_static.f64_values[520]*self.scalar_static.f64_values[521]);
        self.scalar_static.f64_values[523]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[138]);
        self.scalar_static.f64_values[524]=(self.scalar_static.f64_values[523]).exp();
        self.scalar_static.f64_values[525]=(self.scalar_static.f64_values[137]*self.scalar_static.f64_values[524]);
        self.scalar_static.f64_values[526]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[140]);
        self.scalar_static.f64_values[527]=(self.scalar_static.f64_values[526]).exp();
        self.scalar_static.f64_values[528]=(self.scalar_static.f64_values[139]*self.scalar_static.f64_values[527]);
        self.scalar_static.f64_values[529]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[530]=(self.scalar_static.f64_values[529]).exp();
        self.scalar_static.f64_values[531]=(self.scalar_static.f64_values[141]*self.scalar_static.f64_values[530]);
        self.scalar_static.f64_values[532]=(self.scalar_static.f64_values[313]*self.scalar_static.f64_values[144]);
        self.scalar_static.f64_values[533]=(self.scalar_static.f64_values[532]).exp();
        self.scalar_static.f64_values[534]=(self.scalar_static.f64_values[143]*self.scalar_static.f64_values[533]);
        self.scalar_static.f64_values[535]=(self.scalar_static.f64_values[311]*self.scalar_static.f64_values[145]);
        self.scalar_static.f64_values[536]=(1.0+self.scalar_static.f64_values[535]);
        self.scalar_static.f64_values[537]=(self.scalar_static.f64_values[534]*self.scalar_static.f64_values[536]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
