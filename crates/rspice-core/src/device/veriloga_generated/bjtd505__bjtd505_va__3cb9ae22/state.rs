#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 143],
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
            const DEFAULTS_0: [f64; 137] = [
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
                2e-11, 0.0, 0.0, 0.0, 0.0, 2.0, 400.0, 1e-40,
                1e-40,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 137);
            {
                let params = &mut *ptr;
                params[137] = 0.001;
                validate_parameter("minr", params[137], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 0.0, 0.16, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(138), 5);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 145] = [
    ("dta", 0), ("trise", 0), ("dtemp", 0), ("mult", 1), ("version", 2), ("type", 3), ("tref", 4), ("exmod", 5), ("exphi", 6), ("exavl", 7), ("is", 8), ("nff", 9), ("nfr", 10), ("ik", 11), ("ver", 12), ("vef", 13),
    ("issr", 14), ("ibi", 15), ("nbi", 16), ("ibis", 17), ("nbis", 18), ("ibf", 19), ("mlf", 20), ("ibfs", 21), ("mlfs", 22), ("swib1", 23), ("ibinbr", 24), ("ibinbrs", 25), ("vknbr", 26), ("ibinbrqs", 27), ("ibx", 28), ("ikbx", 29),
    ("ibr", 30), ("mlr", 31), ("xext", 32), ("izeb", 33), ("nzeb", 34), ("izcb", 35), ("nzcb", 36), ("vzmin", 37), ("swavl", 38), ("aavl", 39), ("cavl", 40), ("itoavl", 41), ("bavl", 42), ("vdcavl", 43), ("wavl", 44), ("vavl", 45),
    ("sfh", 46), ("ihcavl", 47), ("davl", 48), ("eavl", 49), ("aexavl", 50), ("ionexavl", 51), ("swgemlim", 52), ("re", 53), ("rbc", 54), ("rbv", 55), ("rcc", 56), ("rcblx", 57), ("rcbli", 58), ("rcv", 59), ("scrcv", 60), ("ihc", 61),
    ("axi", 62), ("vdc", 63), ("cje", 64), ("vde", 65), ("pe", 66), ("xcje", 67), ("cbeo", 68), ("cjc", 69), ("vdcctc", 70), ("pc", 71), ("swvchc", 72), ("swvjunc", 73), ("xp", 74), ("mc", 75), ("xcjc", 76), ("cbco", 77),
    ("swqex", 78), ("vdcex", 79), ("vbrcb", 80), ("pbrcb", 81), ("frevcb", 82), ("swjbrcb", 83), ("mtau", 84), ("taue", 85), ("taub", 86), ("tepi", 87), ("taur", 88), ("tauex", 89), ("nex", 90), ("deg", 91), ("xrec", 92), ("xqb", 93),
    ("ke", 94), ("aqbo", 95), ("ae", 96), ("ab", 97), ("aepi", 98), ("aepiex", 99), ("aex", 100), ("ac", 101), ("acx", 102), ("acbl", 103), ("vgb", 104), ("vgbnbrqs", 105), ("vgbnbr", 106), ("vgbnbrs", 107), ("vgknbr", 108), ("vgc", 109),
    ("vge", 110), ("vgcx", 111), ("vgj", 112), ("vgzeb", 113), ("avgeb", 114), ("tvgeb", 115), ("vgzcb", 116), ("avgcb", 117), ("tvgcb", 118), ("dvgte", 119), ("dais", 120), ("tnff", 121), ("tnfr", 122), ("tbavl", 123), ("dtmax", 124), ("af", 125),
    ("afn", 126), ("kf", 127), ("kfn", 128), ("kavl", 129), ("kc", 130), ("ftaun", 131), ("isibrel", 132), ("nfibrel", 133), ("vexlim", 134), ("p0starlim", 135), ("pwlim", 136), ("minr", 137), ("istat", 138), ("vtat", 139), ("ktat", 140), ("vbtbt", 141),
    ("kbtbt", 142),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 143] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 143] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 143] = [
    "dta", "mult", "version", "type", "tref", "exmod", "exphi", "exavl", "is", "nff", "nfr", "ik", "ver", "vef", "issr", "ibi",
    "nbi", "ibis", "nbis", "ibf", "mlf", "ibfs", "mlfs", "swib1", "ibinbr", "ibinbrs", "vknbr", "ibinbrqs", "ibx", "ikbx", "ibr", "mlr",
    "xext", "izeb", "nzeb", "izcb", "nzcb", "vzmin", "swavl", "aavl", "cavl", "itoavl", "bavl", "vdcavl", "wavl", "vavl", "sfh", "ihcavl",
    "davl", "eavl", "aexavl", "ionexavl", "swgemlim", "re", "rbc", "rbv", "rcc", "rcblx", "rcbli", "rcv", "scrcv", "ihc", "axi", "vdc",
    "cje", "vde", "pe", "xcje", "cbeo", "cjc", "vdcctc", "pc", "swvchc", "swvjunc", "xp", "mc", "xcjc", "cbco", "swqex", "vdcex",
    "vbrcb", "pbrcb", "frevcb", "swjbrcb", "mtau", "taue", "taub", "tepi", "taur", "tauex", "nex", "deg", "xrec", "xqb", "ke", "aqbo",
    "ae", "ab", "aepi", "aepiex", "aex", "ac", "acx", "acbl", "vgb", "vgbnbrqs", "vgbnbr", "vgbnbrs", "vgknbr", "vgc", "vge", "vgcx",
    "vgj", "vgzeb", "avgeb", "tvgeb", "vgzcb", "avgcb", "tvgcb", "dvgte", "dais", "tnff", "tnfr", "tbavl", "dtmax", "af", "afn", "kf",
    "kfn", "kavl", "kc", "ftaun", "isibrel", "nfibrel", "vexlim", "p0starlim", "pwlim", "minr", "istat", "vtat", "ktat", "vbtbt", "kbtbt",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 143] = [
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
    &[], &[], &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 143] = [
    false, false, false, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, true, true, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 143] = [
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
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 40.0, label: "40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 143] = [
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
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 400.0, label: "400.0" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }),
    Some(ParameterBound { value: 1e-20, label: "1e-20" }), None, None, None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 143] = [
    0, 3, 2, 0, 2, 0, 0, 0, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 2, 2, 2,
    0, 2, 2, 2, 2, 3, 0, 2, 3, 3, 3, 0, 2, 2, 2, 2, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 2, 0, 2, 1, 1, 1, 0, 2, 2, 3, 2, 2, 2, 2, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2, 2,
    2, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 3, 0, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 143] = [
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
    &[], &[], &[], &[], &[], &[], &[],
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

fn canonical_boxed_zero_f64<const N: usize>() -> Box<[f64; N]> {
    // SAFETY: every slot is an f64, and all-zero bytes are 0.0.
    let mut boxed = Box::<[f64; N]>::new_uninit();
    unsafe {
        std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
        boxed.assume_init()
    }
}

pub struct Instance {
    pub nodes: [usize; 11],
    pub branches: [usize; 2],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 143]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<9, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) canonical_reactive: Box<[f64; 91]>,
    pub(crate) canonical_staged: Box<[f64; 196]>,
    pub(crate) canonical_instance_valid: bool,
    pub(crate) canonical_temperature_valid: bool,
    pub(crate) canonical_temperature: f64,
    pub(crate) canonical_thermal_voltage: f64,
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
            canonical_reactive: self.canonical_reactive.clone(),
            canonical_staged: self.canonical_staged.clone(),
            canonical_instance_valid: self.canonical_instance_valid,
            canonical_temperature_valid: self.canonical_temperature_valid,
            canonical_temperature: self.canonical_temperature,
            canonical_thermal_voltage: self.canonical_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 11;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 143;
    pub const VARIABLE_COUNT: usize = 571;
    pub const DDT_STATE_COUNT: usize = 9;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "b0bf0b5599b35901f6058f53be1b610644b081bcaef795529bf01c0bbbeb7c25";
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
            canonical_reactive: canonical_boxed_zero_f64(),
            canonical_staged: canonical_boxed_zero_f64(),
            canonical_instance_valid: false,
            canonical_temperature_valid: false,
            canonical_temperature: 0.0,
            canonical_thermal_voltage: 0.0,
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505_va'", name));
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
            self.canonical_instance_valid = false;
            self.canonical_temperature_valid = false;
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
}
