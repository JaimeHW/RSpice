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
    pub nodes: [usize; 11],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 143]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<9, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<947, 110>>,
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
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 11;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 143;
    pub const VARIABLE_COUNT: usize = 571;
    pub const DDT_STATE_COUNT: usize = 9;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "4e935d53255f4b22ecaae6632f824c0ccf6e4186b988df148f9c1df91a1115c9";
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
        self.scalar_static.f64_values[0]=p[3];
        self.scalar_static.bool_values[0]=(self.scalar_static.f64_values[0]==1.0);
        self.scalar_static.f64_values[1]=(if self.scalar_static.bool_values[0]{1.0}else{0.0});
        self.scalar_static.f64_values[2]=(if ((self.scalar_static.f64_values[1])!=0.0){70300000.0}else{0.0});
        self.scalar_static.f64_values[3]=(if ((self.scalar_static.f64_values[1])!=0.0){123000000.0}else{0.0});
        self.scalar_static.bool_values[1]=(!((self.scalar_static.f64_values[1])!=0.0));
        self.scalar_static.f64_values[4]=(if self.scalar_static.bool_values[1]{158000000.0}else{self.scalar_static.f64_values[2]});
        self.scalar_static.f64_values[5]=(if self.scalar_static.bool_values[1]{204000000.0}else{self.scalar_static.f64_values[3]});
        self.scalar_static.f64_values[6]=p[32];
        self.scalar_static.f64_values[7]=(1.0-self.scalar_static.f64_values[6]);
        self.scalar_static.f64_values[8]=p[4];
        self.scalar_static.f64_values[9]=(self.scalar_static.f64_values[8]+273.15);
        self.scalar_static.f64_values[10]=p[0];
        self.scalar_static.f64_values[11]=p[137];
        self.scalar_static.bool_values[2]=(0.0==self.scalar_static.f64_values[11]);
        self.scalar_static.f64_values[12]=(if self.scalar_static.bool_values[2]{1.0}else{0.0});
        self.scalar_static.f64_values[13]=(if ((self.scalar_static.f64_values[12])!=0.0){1e-12}else{0.0});
        self.scalar_static.bool_values[3]=(!((self.scalar_static.f64_values[12])!=0.0));
        self.scalar_static.f64_values[14]=(if self.scalar_static.bool_values[3]{self.scalar_static.f64_values[11]}else{self.scalar_static.f64_values[13]});
        self.scalar_static.f64_values[15]=p[1];
        self.scalar_static.f64_values[16]=(self.scalar_static.f64_values[14]*self.scalar_static.f64_values[15]);
        self.scalar_static.f64_values[17]=(1.0/self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[18]=p[66];
        self.scalar_static.f64_values[19]=(2.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[20]=f64::powf(2.0,self.scalar_static.f64_values[19]);
        self.scalar_static.f64_values[21]=(1.0/self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[22]=p[113];
        self.scalar_static.f64_values[23]=p[114];
        self.scalar_static.f64_values[24]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[25]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[24]);
        self.scalar_static.f64_values[26]=p[115];
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
        self.scalar_static.f64_values[47]=p[65];
        self.scalar_static.f64_values[48]=(1.0/self.scalar_static.f64_values[47]);
        self.scalar_static.f64_values[49]=p[70];
        self.scalar_static.f64_values[50]=p[71];
        self.scalar_static.f64_values[51]=(2.0-self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[52]=f64::powf(2.0,self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[53]=(1.0/self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[54]=p[116];
        self.scalar_static.f64_values[55]=p[117];
        self.scalar_static.f64_values[56]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[55]);
        self.scalar_static.f64_values[57]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[56]);
        self.scalar_static.f64_values[58]=p[118];
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
        self.scalar_static.f64_values[80]=p[82];
        self.scalar_static.f64_values[81]=(1.0/self.scalar_static.f64_values[80]);
        self.scalar_static.f64_values[82]=(1.0-self.scalar_static.f64_values[81]);
        self.scalar_static.f64_values[83]=(self.scalar_static.f64_values[9]*8.617086918058125e-5);
        self.scalar_static.f64_values[84]=(1.0/self.scalar_static.f64_values[83]);
        self.scalar_static.f64_values[85]=p[104];
        self.scalar_static.f64_values[86]=p[63];
        self.scalar_static.f64_values[87]=p[109];
        self.scalar_static.f64_values[88]=p[79];
        self.scalar_static.f64_values[89]=p[26];
        self.scalar_static.f64_values[90]=p[108];
        self.scalar_static.f64_values[91]=p[64];
        self.scalar_static.f64_values[92]=p[74];
        self.scalar_static.f64_values[93]=(1.0-self.scalar_static.f64_values[92]);
        self.scalar_static.f64_values[94]=p[69];
        self.scalar_static.f64_values[95]=p[53];
        self.scalar_static.f64_values[96]=p[96];
        self.scalar_static.f64_values[97]=p[55];
        self.scalar_static.f64_values[98]=p[97];
        self.scalar_static.f64_values[99]=p[95];
        self.scalar_static.f64_values[100]=(self.scalar_static.f64_values[98]-self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[101]=p[54];
        self.scalar_static.f64_values[102]=p[100];
        self.scalar_static.f64_values[103]=p[56];
        self.scalar_static.f64_values[104]=p[101];
        self.scalar_static.f64_values[105]=p[57];
        self.scalar_static.f64_values[106]=p[103];
        self.scalar_static.f64_values[107]=p[58];
        self.scalar_static.f64_values[108]=p[59];
        self.scalar_static.f64_values[109]=p[98];
        self.scalar_static.f64_values[110]=p[121];
        self.scalar_static.bool_values[8]=(0.0!=self.scalar_static.f64_values[110]);
        self.scalar_static.f64_values[111]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.f64_values[112]=p[9];
        self.scalar_static.bool_values[9]=(!((self.scalar_static.f64_values[111])!=0.0));
        self.scalar_static.f64_values[113]=p[122];
        self.scalar_static.bool_values[10]=(0.0!=self.scalar_static.f64_values[113]);
        self.scalar_static.f64_values[114]=(if self.scalar_static.bool_values[10]{1.0}else{0.0});
        self.scalar_static.f64_values[115]=p[10];
        self.scalar_static.bool_values[11]=(!((self.scalar_static.f64_values[114])!=0.0));
        self.scalar_static.f64_values[116]=p[42];
        self.scalar_static.f64_values[117]=p[123];
        self.scalar_static.f64_values[118]=p[8];
        self.scalar_static.f64_values[119]=(4.0-self.scalar_static.f64_values[98]);
        self.scalar_static.f64_values[120]=(self.scalar_static.f64_values[119]-self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[121]=p[120];
        self.scalar_static.f64_values[122]=(self.scalar_static.f64_values[120]+self.scalar_static.f64_values[121]);
        self.scalar_static.f64_values[123]=(-self.scalar_static.f64_values[85]);
        self.scalar_static.f64_values[124]=p[11];
        self.scalar_static.f64_values[125]=(1.0-self.scalar_static.f64_values[98]);
        self.scalar_static.f64_values[126]=p[29];
        self.scalar_static.f64_values[127]=p[102];
        self.scalar_static.f64_values[128]=(1.0-self.scalar_static.f64_values[127]);
        self.scalar_static.f64_values[129]=p[19];
        self.scalar_static.f64_values[130]=p[20];
        self.scalar_static.f64_values[131]=(2.0*self.scalar_static.f64_values[130]);
        self.scalar_static.f64_values[132]=(6.0-self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[133]=p[112];
        self.scalar_static.f64_values[134]=(-self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[135]=p[30];
        self.scalar_static.f64_values[136]=p[31];
        self.scalar_static.f64_values[137]=(2.0*self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[138]=(6.0-self.scalar_static.f64_values[137]);
        self.scalar_static.f64_values[139]=(-self.scalar_static.f64_values[87]);
        self.scalar_static.f64_values[140]=p[15];
        self.scalar_static.f64_values[141]=(4.0-self.scalar_static.f64_values[96]);
        self.scalar_static.f64_values[142]=(self.scalar_static.f64_values[121]+self.scalar_static.f64_values[141]);
        self.scalar_static.f64_values[143]=p[16];
        self.scalar_static.f64_values[144]=p[110];
        self.scalar_static.f64_values[145]=(-self.scalar_static.f64_values[144]);
        self.scalar_static.f64_values[146]=p[17];
        self.scalar_static.f64_values[147]=p[18];
        self.scalar_static.f64_values[148]=p[23];
        self.scalar_static.bool_values[12]=(1.0==self.scalar_static.f64_values[148]);
        self.scalar_static.f64_values[149]=(if self.scalar_static.bool_values[12]{1.0}else{0.0});
        self.scalar_static.f64_values[150]=p[24];
        self.scalar_static.f64_values[151]=p[106];
        self.scalar_static.f64_values[152]=(-self.scalar_static.f64_values[151]);
        self.scalar_static.f64_values[153]=p[27];
        self.scalar_static.f64_values[154]=p[105];
        self.scalar_static.f64_values[155]=(-self.scalar_static.f64_values[154]);
        self.scalar_static.f64_values[156]=p[25];
        self.scalar_static.f64_values[157]=p[107];
        self.scalar_static.f64_values[158]=(-self.scalar_static.f64_values[157]);
        self.scalar_static.f64_values[159]=p[28];
        self.scalar_static.f64_values[160]=(4.0-self.scalar_static.f64_values[127]);
        self.scalar_static.f64_values[161]=(self.scalar_static.f64_values[121]+self.scalar_static.f64_values[160]);
        self.scalar_static.f64_values[162]=p[111];
        self.scalar_static.f64_values[163]=(-self.scalar_static.f64_values[162]);
        self.scalar_static.f64_values[164]=p[21];
        self.scalar_static.f64_values[165]=p[22];
        self.scalar_static.f64_values[166]=(2.0*self.scalar_static.f64_values[165]);
        self.scalar_static.f64_values[167]=(6.0-self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[168]=p[132];
        self.scalar_static.f64_values[169]=p[133];
        self.scalar_static.f64_values[170]=(4.0/self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[171]=p[138];
        self.scalar_static.f64_values[172]=p[140];
        self.scalar_static.f64_values[173]=p[34];
        self.scalar_static.f64_values[174]=p[33];
        self.scalar_static.f64_values[175]=p[36];
        self.scalar_static.f64_values[176]=p[35];
        self.scalar_static.f64_values[177]=p[13];
        self.scalar_static.f64_values[178]=p[12];
        self.scalar_static.f64_values[179]=p[85];
        self.scalar_static.f64_values[180]=(self.scalar_static.f64_values[98]-2.0);
        self.scalar_static.f64_values[181]=p[119];
        self.scalar_static.f64_values[182]=(-self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[183]=p[86];
        self.scalar_static.f64_values[184]=(self.scalar_static.f64_values[98]+self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[185]=(self.scalar_static.f64_values[184]-1.0);
        self.scalar_static.f64_values[186]=p[87];
        self.scalar_static.f64_values[187]=(self.scalar_static.f64_values[109]-1.0);
        self.scalar_static.f64_values[188]=p[88];
        self.scalar_static.f64_values[189]=(self.scalar_static.f64_values[183]+self.scalar_static.f64_values[186]);
        self.scalar_static.f64_values[190]=p[89];
        self.scalar_static.f64_values[191]=p[99];
        self.scalar_static.f64_values[192]=(self.scalar_static.f64_values[191]-1.0);
        self.scalar_static.f64_values[193]=(self.scalar_static.f64_values[5]*1.081);
        self.scalar_static.f64_values[194]=p[91];
        self.scalar_static.bool_values[13]=(self.scalar_static.f64_values[103]>0.0);
        self.scalar_static.f64_values[195]=(if self.scalar_static.bool_values[13]{1.0}else{0.0});
        self.scalar_static.bool_values[14]=(!((self.scalar_static.f64_values[195])!=0.0));
        self.scalar_static.bool_values[15]=(self.scalar_static.f64_values[105]>0.0);
        self.scalar_static.f64_values[196]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.bool_values[16]=(!((self.scalar_static.f64_values[196])!=0.0));
        self.scalar_static.bool_values[17]=(self.scalar_static.f64_values[107]>0.0);
        self.scalar_static.f64_values[197]=(if self.scalar_static.bool_values[17]{1.0}else{0.0});
        self.scalar_static.bool_values[18]=(!((self.scalar_static.f64_values[197])!=0.0));
        self.scalar_static.f64_values[198]=p[134];
        self.scalar_static.f64_values[199]=(self.scalar_static.f64_values[198]).exp();
        self.scalar_static.f64_values[200]=p[136];
        self.scalar_static.f64_values[201]=p[61];
        self.scalar_static.f64_values[202]=p[60];
        self.scalar_static.f64_values[203]=(self.scalar_static.f64_values[201]*self.scalar_static.f64_values[202]);
        self.scalar_static.f64_values[204]=p[62];
        self.scalar_static.f64_values[205]=(-1.0/self.scalar_static.f64_values[204]);
        self.scalar_static.f64_values[206]=(self.scalar_static.f64_values[205]).exp();
        self.scalar_static.f64_values[207]=(1.0+self.scalar_static.f64_values[206]);
        self.scalar_static.f64_values[208]=(self.scalar_static.f64_values[207]).ln();
        self.scalar_static.f64_values[209]=(self.scalar_static.f64_values[204]*self.scalar_static.f64_values[208]);
        self.scalar_static.f64_values[210]=(1.0+self.scalar_static.f64_values[209]);
        self.scalar_static.f64_values[211]=p[135];
        self.scalar_static.f64_values[212]=(0.5*self.scalar_static.f64_values[202]);
        self.scalar_static.f64_values[213]=p[72];
        self.scalar_static.bool_values[19]=(0.0==self.scalar_static.f64_values[213]);
        self.scalar_static.f64_values[214]=(if self.scalar_static.bool_values[19]{1.0}else{0.0});
        self.scalar_static.bool_values[20]=(!((self.scalar_static.f64_values[214])!=0.0));
        self.scalar_static.f64_values[215]=(-1.0/self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[216]=f64::powf(3.0,self.scalar_static.f64_values[215]);
        self.scalar_static.f64_values[217]=(1.0-self.scalar_static.f64_values[216]);
        self.scalar_static.f64_values[218]=(1.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[219]=p[73];
        self.scalar_static.bool_values[21]=(1.0==self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[220]=(if self.scalar_static.bool_values[21]{1.0}else{0.0});
        self.scalar_static.bool_values[22]=(2.0==self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[221]=(if self.scalar_static.bool_values[22]{1.0}else{0.0});
        self.scalar_static.bool_values[23]=(!((self.scalar_static.f64_values[220])!=0.0));
        self.scalar_static.bool_values[24]=(((self.scalar_static.f64_values[221])!=0.0)&&self.scalar_static.bool_values[23]);
        self.scalar_static.bool_values[25]=(!((self.scalar_static.f64_values[221])!=0.0));
        self.scalar_static.bool_values[26]=(self.scalar_static.bool_values[23]&&self.scalar_static.bool_values[25]);
        self.scalar_static.f64_values[222]=(-1.0/self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[223]=p[75];
        self.scalar_static.f64_values[224]=(1.0-self.scalar_static.f64_values[50]);
        self.scalar_static.bool_values[27]=(0.0==self.scalar_static.f64_values[194]);
        self.scalar_static.f64_values[225]=(if self.scalar_static.bool_values[27]{1.0}else{0.0});
        self.scalar_static.bool_values[28]=(!((self.scalar_static.f64_values[225])!=0.0));
        self.scalar_static.f64_values[226]=p[14];
        self.scalar_static.f64_values[227]=p[139];
        self.scalar_static.f64_values[228]=p[141];
        self.scalar_static.f64_values[229]=p[142];
        self.scalar_static.f64_values[230]=p[92];
        self.scalar_static.bool_values[29]=(0.0==self.scalar_static.f64_values[230]);
        self.scalar_static.f64_values[231]=(if self.scalar_static.bool_values[29]{1.0}else{0.0});
        self.scalar_static.bool_values[30]=(!((self.scalar_static.f64_values[149])!=0.0));
        self.scalar_static.bool_values[31]=(((self.scalar_static.f64_values[231])!=0.0)&&self.scalar_static.bool_values[30]);
        self.scalar_static.bool_values[32]=(!((self.scalar_static.f64_values[231])!=0.0));
        self.scalar_static.bool_values[33]=(self.scalar_static.bool_values[30]&&self.scalar_static.bool_values[32]);
        self.scalar_static.f64_values[232]=(1.0-self.scalar_static.f64_values[230]);
        self.scalar_static.bool_values[34]=(self.scalar_static.f64_values[174]>0.0);
        self.scalar_static.bool_values[35]=(self.scalar_static.f64_values[173]>0.0);
        self.scalar_static.bool_values[36]=(self.scalar_static.bool_values[34]&&self.scalar_static.bool_values[35]);
        self.scalar_static.f64_values[233]=(-2.0-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[234]=(self.scalar_static.f64_values[18]*self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[235]=(1.0-self.scalar_static.f64_values[234]);
        self.scalar_static.f64_values[236]=(self.scalar_static.f64_values[18]-1.0);
        self.scalar_static.bool_values[37]=(self.scalar_static.f64_values[176]>0.0);
        self.scalar_static.bool_values[38]=(self.scalar_static.f64_values[175]>0.0);
        self.scalar_static.bool_values[39]=(self.scalar_static.bool_values[37]&&self.scalar_static.bool_values[38]);
        self.scalar_static.f64_values[237]=(-2.0-self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[238]=(self.scalar_static.f64_values[50]*self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[239]=(1.0-self.scalar_static.f64_values[238]);
        self.scalar_static.f64_values[240]=(self.scalar_static.f64_values[50]-1.0);
        self.scalar_static.f64_values[241]=p[5];
        self.scalar_static.bool_values[40]=(self.scalar_static.f64_values[241]>0.0);
        self.scalar_static.bool_values[41]=(self.scalar_static.f64_values[6]>0.0);
        self.scalar_static.bool_values[42]=(self.scalar_static.bool_values[40]&&self.scalar_static.bool_values[41]);
        self.scalar_static.f64_values[242]=(if self.scalar_static.bool_values[42]{1.0}else{0.0});
        self.scalar_static.f64_values[243]=(self.scalar_static.f64_values[6]*2.0);
        self.scalar_static.bool_values[43]=(1.0==self.scalar_static.f64_values[241]);
        self.scalar_static.f64_values[244]=(if self.scalar_static.bool_values[43]{1.0}else{0.0});
        self.scalar_static.bool_values[44]=(((self.scalar_static.f64_values[242])!=0.0)&&((self.scalar_static.f64_values[244])!=0.0));
        self.scalar_static.f64_values[245]=(if self.scalar_static.bool_values[44]{0.0121}else{0.010000000000000002});
        self.scalar_static.f64_values[246]=(0.5*self.scalar_static.f64_values[245]);
        self.scalar_static.bool_values[45]=(!((self.scalar_static.f64_values[244])!=0.0));
        self.scalar_static.bool_values[46]=(((self.scalar_static.f64_values[242])!=0.0)&&self.scalar_static.bool_values[45]);
        self.scalar_static.f64_values[247]=p[83];
        self.scalar_static.bool_values[47]=(1.0==self.scalar_static.f64_values[247]);
        self.scalar_static.f64_values[248]=(if self.scalar_static.bool_values[47]{1.0}else{0.0});
        self.scalar_static.f64_values[249]=(if ((self.scalar_static.f64_values[248])!=0.0){1e-12}else{self.scalar_static.f64_values[245]});
        self.scalar_static.f64_values[250]=(0.5*self.scalar_static.f64_values[249]);
        self.scalar_static.f64_values[251]=p[81];
        self.scalar_static.f64_values[252]=f64::powf(self.scalar_static.f64_values[82],self.scalar_static.f64_values[251]);
        self.scalar_static.f64_values[253]=(1.0-self.scalar_static.f64_values[252]);
        self.scalar_static.f64_values[254]=(1.0/self.scalar_static.f64_values[253]);
        self.scalar_static.f64_values[255]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[254]}else{0.0});
        self.scalar_static.f64_values[256]=p[80];
        self.scalar_static.f64_values[257]=(self.scalar_static.f64_values[82]*self.scalar_static.f64_values[256]);
        self.scalar_static.f64_values[258]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[257]}else{0.0});
        self.scalar_static.f64_values[259]=(self.scalar_static.f64_values[255]*self.scalar_static.f64_values[255]);
        self.scalar_static.f64_values[260]=(self.scalar_static.f64_values[251]-1.0);
        self.scalar_static.f64_values[261]=f64::powf(self.scalar_static.f64_values[82],self.scalar_static.f64_values[260]);
        self.scalar_static.f64_values[262]=(self.scalar_static.f64_values[259]*self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[263]=(self.scalar_static.f64_values[251]*self.scalar_static.f64_values[262]);
        self.scalar_static.f64_values[264]=(self.scalar_static.f64_values[263]/self.scalar_static.f64_values[256]);
        self.scalar_static.f64_values[265]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[264]}else{0.0});
        self.scalar_static.bool_values[48]=(!((self.scalar_static.f64_values[248])!=0.0));
        self.scalar_static.f64_values[266]=p[38];
        self.scalar_static.bool_values[49]=(1.0==self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[267]=(if self.scalar_static.bool_values[49]{1.0}else{0.0});
        self.scalar_static.f64_values[268]=p[43];
        self.scalar_static.f64_values[269]=p[41];
        self.scalar_static.f64_values[270]=p[40];
        self.scalar_static.f64_values[271]=p[39];
        self.scalar_static.bool_values[50]=(2.0==self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[272]=(if self.scalar_static.bool_values[50]{1.0}else{0.0});
        self.scalar_static.bool_values[51]=(!((self.scalar_static.f64_values[267])!=0.0));
        self.scalar_static.f64_values[273]=p[45];
        self.scalar_static.f64_values[274]=(2.0*self.scalar_static.f64_values[273]);
        self.scalar_static.f64_values[275]=p[44];
        self.scalar_static.f64_values[276]=(self.scalar_static.f64_values[275]*self.scalar_static.f64_values[275]);
        self.scalar_static.f64_values[277]=(self.scalar_static.f64_values[274]/self.scalar_static.f64_values[276]);
        self.scalar_static.f64_values[278]=p[7];
        self.scalar_static.bool_values[52]=(0.0==self.scalar_static.f64_values[278]);
        self.scalar_static.f64_values[279]=(if self.scalar_static.bool_values[52]{1.0}else{0.0});
        self.scalar_static.bool_values[53]=(!((self.scalar_static.f64_values[279])!=0.0));
        self.scalar_static.f64_values[280]=p[46];
        self.scalar_static.f64_values[281]=(2.0*self.scalar_static.f64_values[280]);
        self.scalar_static.f64_values[282]=(1.0+self.scalar_static.f64_values[280]);
        self.scalar_static.f64_values[283]=(1.0+self.scalar_static.f64_values[281]);
        self.scalar_static.f64_values[284]=(self.scalar_static.f64_values[282]/self.scalar_static.f64_values[283]);
        self.scalar_static.bool_values[54]=(3.0==self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[285]=(if self.scalar_static.bool_values[54]{1.0}else{0.0});
        self.scalar_static.bool_values[55]=(!((self.scalar_static.f64_values[272])!=0.0));
        self.scalar_static.f64_values[286]=p[47];
        self.scalar_static.f64_values[287]=p[48];
        self.scalar_static.f64_values[288]=p[51];
        self.scalar_static.f64_values[289]=p[50];
        self.scalar_static.f64_values[290]=p[49];
        self.scalar_static.f64_values[291]=p[52];
        self.scalar_static.bool_values[56]=(1.0==self.scalar_static.f64_values[291]);
        self.scalar_static.f64_values[292]=(if self.scalar_static.bool_values[56]{1.0}else{0.0});
        self.scalar_static.bool_values[57]=(!((self.scalar_static.f64_values[285])!=0.0));
        self.scalar_static.bool_values[58]=(!((self.scalar_static.f64_values[292])!=0.0));
        self.scalar_static.f64_values[293]=p[67];
        self.scalar_static.f64_values[294]=(1.0-self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[295]=p[76];
        self.scalar_static.f64_values[296]=(1.0-self.scalar_static.f64_values[295]);
        self.scalar_static.f64_values[297]=p[84];
        self.scalar_static.f64_values[298]=(1.0/self.scalar_static.f64_values[297]);
        self.scalar_static.f64_values[299]=p[78];
        self.scalar_static.bool_values[59]=(0.0==self.scalar_static.f64_values[299]);
        self.scalar_static.f64_values[300]=(if self.scalar_static.bool_values[59]{1.0}else{0.0});
        self.scalar_static.f64_values[301]=p[90];
        self.scalar_static.bool_values[60]=(!((self.scalar_static.f64_values[300])!=0.0));
        self.scalar_static.bool_values[61]=(3.0==self.scalar_static.f64_values[241]);
        self.scalar_static.bool_values[62]=(self.scalar_static.bool_values[43]||self.scalar_static.bool_values[61]);
        self.scalar_static.bool_values[63]=(self.scalar_static.bool_values[41]&&self.scalar_static.bool_values[62]);
        self.scalar_static.f64_values[302]=(if self.scalar_static.bool_values[63]{1.0}else{0.0});
        self.scalar_static.bool_values[64]=(((self.scalar_static.f64_values[300])!=0.0)&&((self.scalar_static.f64_values[302])!=0.0));
        self.scalar_static.f64_values[303]=(self.scalar_static.f64_values[6]*0.5);
        self.scalar_static.bool_values[65]=(self.scalar_static.bool_values[60]&&((self.scalar_static.f64_values[302])!=0.0));
        self.scalar_static.f64_values[304]=p[6];
        self.scalar_static.bool_values[66]=(1.0==self.scalar_static.f64_values[304]);
        self.scalar_static.f64_values[305]=(if self.scalar_static.bool_values[66]{1.0}else{0.0});
        self.scalar_static.f64_values[306]=(-self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[307]=p[94];
        self.scalar_static.f64_values[308]=(1.0-self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[309]=p[93];
        self.scalar_static.f64_values[310]=(1.0-self.scalar_static.f64_values[309]);
        self.scalar_static.bool_values[67]=(!((self.scalar_static.f64_values[305])!=0.0));
        self.scalar_static.f64_values[311]=p[129];
        self.scalar_static.bool_values[68]=(self.scalar_static.f64_values[311]>0.0);
        self.scalar_static.f64_values[312]=(if self.scalar_static.bool_values[68]{1.0}else{0.0});
        self.scalar_static.bool_values[69]=(!((self.scalar_static.f64_values[312])!=0.0));
        self.scalar_static.f64_values[313]=p[130];
        self.scalar_static.bool_values[70]=(1.0==self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[314]=(if self.scalar_static.bool_values[70]{1.0}else{0.0});
        self.scalar_static.bool_values[71]=(2.0==self.scalar_static.f64_values[313]);
        self.scalar_static.f64_values[315]=(if self.scalar_static.bool_values[71]{1.0}else{0.0});
        self.scalar_static.bool_values[72]=(!((self.scalar_static.f64_values[314])!=0.0));
        self.scalar_static.bool_values[73]=(((self.scalar_static.f64_values[315])!=0.0)&&self.scalar_static.bool_values[72]);
        self.scalar_static.f64_values[316]=p[131];
        self.scalar_static.bool_values[74]=(!((self.scalar_static.f64_values[315])!=0.0));
        self.scalar_static.bool_values[75]=(self.scalar_static.bool_values[72]&&self.scalar_static.bool_values[74]);
        self.scalar_static.f64_values[317]=p[68];
        self.scalar_static.f64_values[318]=p[77];
        self.scalar_static.f64_values[319]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[317]);
        self.scalar_static.f64_values[320]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[318]);
        self.scalar_static.f64_values[321]=(-self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[322]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[323]=(self.scalar_static.f64_values[321]-self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[324]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[325]=(self.scalar_static.f64_values[218]-1.0);
        self.scalar_static.f64_values[326]=(if ((self.scalar_static.f64_values[220])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[327]=(if ((self.scalar_static.f64_values[220])!=0.0){self.scalar_static.f64_values[321]}else{0.0});
        self.scalar_static.f64_values[328]=(self.scalar_static.f64_values[223]-1.0);
        self.scalar_static.f64_values[329]=(self.scalar_static.f64_values[224]-1.0);
        self.scalar_static.f64_values[330]=(self.scalar_static.f64_values[321]/0.0001);
        self.scalar_static.f64_values[331]=(self.scalar_static.f64_values[0]/0.0001);
        self.scalar_static.f64_values[332]=(-self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[333]=(-self.scalar_static.f64_values[331]);
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[321]/0.001);
        self.scalar_static.f64_values[335]=(self.scalar_static.f64_values[0]/0.001);
        self.scalar_static.f64_values[336]=(-self.scalar_static.f64_values[334]);
        self.scalar_static.f64_values[337]=(-self.scalar_static.f64_values[335]);
        self.scalar_static.f64_values[338]=(self.scalar_static.f64_values[233]-1.0);
        self.scalar_static.f64_values[339]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[340]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[341]=(0.5*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[342]=(self.scalar_static.f64_values[0]*0.5);
        self.scalar_static.f64_values[343]=(self.scalar_static.f64_values[237]-1.0);
        self.scalar_static.f64_values[344]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[345]=(self.scalar_static.f64_values[52]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[346]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[322]}else{0.0});
        self.scalar_static.f64_values[347]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[324]}else{0.0});
        self.scalar_static.f64_values[348]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[323]}else{0.0});
        self.scalar_static.f64_values[349]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[321]}else{0.0});
        self.scalar_static.f64_values[350]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[351]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[322]}else{0.0});
        self.scalar_static.f64_values[352]=(if ((self.scalar_static.f64_values[248])!=0.0){self.scalar_static.f64_values[321]}else{0.0});
        self.scalar_static.f64_values[353]=(-self.scalar_static.f64_values[350]);
        self.scalar_static.f64_values[354]=(-self.scalar_static.f64_values[351]);
        self.scalar_static.f64_values[355]=(-self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[356]=(self.scalar_static.f64_values[270]-1.0);
        self.scalar_static.f64_values[357]=(self.scalar_static.f64_values[287]-1.0);
        self.scalar_static.f64_values[358]=(self.scalar_static.f64_values[290]-1.0);
        self.scalar_static.f64_values[359]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[301]);
        self.scalar_static.f64_values[360]=(self.scalar_static.f64_values[322]/self.scalar_static.f64_values[301]);
        self.scalar_static.f64_values[361]=(self.scalar_static.f64_values[323]/self.scalar_static.f64_values[301]);
        self.scalar_static.f64_values[362]=(self.scalar_static.f64_values[321]/self.scalar_static.f64_values[301]);
        self.scalar_static.f64_values[363]=(self.scalar_static.f64_values[306]-1.0);
        self.scalar_static.f64_values[364]=(self.scalar_static.f64_values[0]*0.2);
        self.scalar_static.f64_values[365]=(0.2*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[366]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[367]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[368]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[319]);
        self.scalar_static.f64_values[369]=(self.scalar_static.f64_values[319]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[370]=(self.scalar_static.f64_values[320]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[371]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[320]);
        self.scalar_static.f64_values[372]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[373]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[323]);
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
        self.scalar_static.f64_values[374]=(temperature+self.scalar_static.f64_values[10]);
        self.scalar_static.f64_values[375]=(self.scalar_static.f64_values[374]/self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[376]=(self.scalar_static.f64_values[374]*8.617086918058125e-5);
        self.scalar_static.f64_values[377]=(1.0/self.scalar_static.f64_values[376]);
        self.scalar_static.f64_values[378]=(self.scalar_static.f64_values[377]-self.scalar_static.f64_values[84]);
        self.scalar_static.f64_values[379]=(self.scalar_static.f64_values[374]-self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[380]=(self.scalar_static.f64_values[375]).ln();
        self.scalar_static.f64_values[381]=(self.scalar_static.f64_values[374]*self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[382]=(self.scalar_static.f64_values[374]*self.scalar_static.f64_values[381]);
        self.scalar_static.f64_values[383]=(self.scalar_static.f64_values[374]+self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[384]=(self.scalar_static.f64_values[382]/self.scalar_static.f64_values[383]);
        self.scalar_static.f64_values[385]=(self.scalar_static.f64_values[45]-self.scalar_static.f64_values[384]);
        self.scalar_static.f64_values[386]=(self.scalar_static.f64_values[385]-0.05);
        self.scalar_static.f64_values[387]=(self.scalar_static.f64_values[386]/0.1);
        self.scalar_static.bool_values[76]=(self.scalar_static.f64_values[385]<0.05);
        self.scalar_static.f64_values[388]=(if self.scalar_static.bool_values[76]{1.0}else{0.0});
        self.scalar_static.f64_values[389]=(self.scalar_static.f64_values[387]).exp();
        self.scalar_static.f64_values[390]=(1.0+self.scalar_static.f64_values[389]);
        self.scalar_static.f64_values[391]=(self.scalar_static.f64_values[390]).ln();
        self.scalar_static.f64_values[392]=(0.1*self.scalar_static.f64_values[391]);
        self.scalar_static.f64_values[393]=(0.05+self.scalar_static.f64_values[392]);
        self.scalar_static.f64_values[394]=(if ((self.scalar_static.f64_values[388])!=0.0){self.scalar_static.f64_values[393]}else{0.0});
        self.scalar_static.bool_values[77]=(!((self.scalar_static.f64_values[388])!=0.0));
        self.scalar_static.f64_values[395]=(-self.scalar_static.f64_values[387]);
        self.scalar_static.f64_values[396]=(self.scalar_static.f64_values[395]).exp();
        self.scalar_static.f64_values[397]=(1.0+self.scalar_static.f64_values[396]);
        self.scalar_static.f64_values[398]=(self.scalar_static.f64_values[397]).ln();
        self.scalar_static.f64_values[399]=(0.1*self.scalar_static.f64_values[398]);
        self.scalar_static.f64_values[400]=(self.scalar_static.f64_values[385]+self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[401]=(if self.scalar_static.bool_values[77]{self.scalar_static.f64_values[400]}else{self.scalar_static.f64_values[394]});
        self.scalar_static.f64_values[402]=(self.scalar_static.f64_values[374]*self.scalar_static.f64_values[55]);
        self.scalar_static.f64_values[403]=(self.scalar_static.f64_values[374]*self.scalar_static.f64_values[402]);
        self.scalar_static.f64_values[404]=(self.scalar_static.f64_values[374]+self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[405]=(self.scalar_static.f64_values[403]/self.scalar_static.f64_values[404]);
        self.scalar_static.f64_values[406]=(self.scalar_static.f64_values[77]-self.scalar_static.f64_values[405]);
        self.scalar_static.f64_values[407]=(self.scalar_static.f64_values[406]-0.05);
        self.scalar_static.f64_values[408]=(self.scalar_static.f64_values[407]/0.1);
        self.scalar_static.bool_values[78]=(self.scalar_static.f64_values[406]<0.05);
        self.scalar_static.f64_values[409]=(if self.scalar_static.bool_values[78]{1.0}else{0.0});
        self.scalar_static.f64_values[410]=(self.scalar_static.f64_values[408]).exp();
        self.scalar_static.f64_values[411]=(1.0+self.scalar_static.f64_values[410]);
        self.scalar_static.f64_values[412]=(self.scalar_static.f64_values[411]).ln();
        self.scalar_static.f64_values[413]=(0.1*self.scalar_static.f64_values[412]);
        self.scalar_static.f64_values[414]=(0.05+self.scalar_static.f64_values[413]);
        self.scalar_static.f64_values[415]=(if ((self.scalar_static.f64_values[409])!=0.0){self.scalar_static.f64_values[414]}else{0.0});
        self.scalar_static.bool_values[79]=(!((self.scalar_static.f64_values[409])!=0.0));
        self.scalar_static.f64_values[416]=(-self.scalar_static.f64_values[408]);
        self.scalar_static.f64_values[417]=(self.scalar_static.f64_values[416]).exp();
        self.scalar_static.f64_values[418]=(1.0+self.scalar_static.f64_values[417]);
        self.scalar_static.f64_values[419]=(self.scalar_static.f64_values[418]).ln();
        self.scalar_static.f64_values[420]=(0.1*self.scalar_static.f64_values[419]);
        self.scalar_static.f64_values[421]=(self.scalar_static.f64_values[406]+self.scalar_static.f64_values[420]);
        self.scalar_static.f64_values[422]=(if self.scalar_static.bool_values[79]{self.scalar_static.f64_values[421]}else{self.scalar_static.f64_values[415]});
        self.scalar_static.f64_values[423]=(self.scalar_static.f64_values[376]* -3.0);
        self.scalar_static.f64_values[424]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[423]);
        self.scalar_static.f64_values[425]=(self.scalar_static.f64_values[47]*self.scalar_static.f64_values[375]);
        self.scalar_static.f64_values[426]=(self.scalar_static.f64_values[424]+self.scalar_static.f64_values[425]);
        self.scalar_static.f64_values[427]=(1.0-self.scalar_static.f64_values[375]);
        self.scalar_static.f64_values[428]=(self.scalar_static.f64_values[427]*self.scalar_static.f64_values[85]);
        self.scalar_static.f64_values[429]=(self.scalar_static.f64_values[426]+self.scalar_static.f64_values[428]);
        self.scalar_static.f64_values[430]=(0.05-self.scalar_static.f64_values[429]);
        self.scalar_static.f64_values[431]=(self.scalar_static.f64_values[430]/self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[80]=(0.05<self.scalar_static.f64_values[429]);
        self.scalar_static.f64_values[432]=(if self.scalar_static.bool_values[80]{1.0}else{0.0});
        self.scalar_static.f64_values[433]=(self.scalar_static.f64_values[431]).exp();
        self.scalar_static.f64_values[434]=(1.0+self.scalar_static.f64_values[433]);
        self.scalar_static.f64_values[435]=(self.scalar_static.f64_values[434]).ln();
        self.scalar_static.f64_values[436]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[435]);
        self.scalar_static.f64_values[437]=(self.scalar_static.f64_values[429]+self.scalar_static.f64_values[436]);
        self.scalar_static.f64_values[438]=(if ((self.scalar_static.f64_values[432])!=0.0){self.scalar_static.f64_values[437]}else{0.0});
        self.scalar_static.bool_values[81]=(!((self.scalar_static.f64_values[432])!=0.0));
        self.scalar_static.f64_values[439]=(-self.scalar_static.f64_values[431]);
        self.scalar_static.f64_values[440]=(self.scalar_static.f64_values[439]).exp();
        self.scalar_static.f64_values[441]=(1.0+self.scalar_static.f64_values[440]);
        self.scalar_static.f64_values[442]=(self.scalar_static.f64_values[441]).ln();
        self.scalar_static.f64_values[443]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[442]);
        self.scalar_static.f64_values[444]=(0.05+self.scalar_static.f64_values[443]);
        self.scalar_static.f64_values[445]=(if self.scalar_static.bool_values[81]{self.scalar_static.f64_values[444]}else{self.scalar_static.f64_values[438]});
        self.scalar_static.f64_values[446]=(self.scalar_static.f64_values[375]*self.scalar_static.f64_values[86]);
        self.scalar_static.f64_values[447]=(self.scalar_static.f64_values[424]+self.scalar_static.f64_values[446]);
        self.scalar_static.f64_values[448]=(self.scalar_static.f64_values[427]*self.scalar_static.f64_values[87]);
        self.scalar_static.f64_values[449]=(self.scalar_static.f64_values[447]+self.scalar_static.f64_values[448]);
        self.scalar_static.f64_values[450]=(0.05-self.scalar_static.f64_values[449]);
        self.scalar_static.f64_values[451]=(self.scalar_static.f64_values[450]/self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[82]=(0.05<self.scalar_static.f64_values[449]);
        self.scalar_static.f64_values[452]=(if self.scalar_static.bool_values[82]{1.0}else{0.0});
        self.scalar_static.f64_values[453]=(self.scalar_static.f64_values[451]).exp();
        self.scalar_static.f64_values[454]=(1.0+self.scalar_static.f64_values[453]);
        self.scalar_static.f64_values[455]=(self.scalar_static.f64_values[454]).ln();
        self.scalar_static.f64_values[456]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[455]);
        self.scalar_static.f64_values[457]=(self.scalar_static.f64_values[449]+self.scalar_static.f64_values[456]);
        self.scalar_static.f64_values[458]=(if ((self.scalar_static.f64_values[452])!=0.0){self.scalar_static.f64_values[457]}else{0.0});
        self.scalar_static.bool_values[83]=(!((self.scalar_static.f64_values[452])!=0.0));
        self.scalar_static.f64_values[459]=(-self.scalar_static.f64_values[451]);
        self.scalar_static.f64_values[460]=(self.scalar_static.f64_values[459]).exp();
        self.scalar_static.f64_values[461]=(1.0+self.scalar_static.f64_values[460]);
        self.scalar_static.f64_values[462]=(self.scalar_static.f64_values[461]).ln();
        self.scalar_static.f64_values[463]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[462]);
        self.scalar_static.f64_values[464]=(0.05+self.scalar_static.f64_values[463]);
        self.scalar_static.f64_values[465]=(if self.scalar_static.bool_values[83]{self.scalar_static.f64_values[464]}else{self.scalar_static.f64_values[458]});
        self.scalar_static.f64_values[466]=(self.scalar_static.f64_values[375]*self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[467]=(self.scalar_static.f64_values[424]+self.scalar_static.f64_values[466]);
        self.scalar_static.f64_values[468]=(self.scalar_static.f64_values[448]+self.scalar_static.f64_values[467]);
        self.scalar_static.f64_values[469]=(0.05-self.scalar_static.f64_values[468]);
        self.scalar_static.f64_values[470]=(self.scalar_static.f64_values[469]/self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[84]=(0.05<self.scalar_static.f64_values[468]);
        self.scalar_static.f64_values[471]=(if self.scalar_static.bool_values[84]{1.0}else{0.0});
        self.scalar_static.f64_values[472]=(self.scalar_static.f64_values[470]).exp();
        self.scalar_static.f64_values[473]=(1.0+self.scalar_static.f64_values[472]);
        self.scalar_static.f64_values[474]=(self.scalar_static.f64_values[473]).ln();
        self.scalar_static.f64_values[475]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[474]);
        self.scalar_static.f64_values[476]=(self.scalar_static.f64_values[468]+self.scalar_static.f64_values[475]);
        self.scalar_static.f64_values[477]=(if ((self.scalar_static.f64_values[471])!=0.0){self.scalar_static.f64_values[476]}else{0.0});
        self.scalar_static.bool_values[85]=(!((self.scalar_static.f64_values[471])!=0.0));
        self.scalar_static.f64_values[478]=(-self.scalar_static.f64_values[470]);
        self.scalar_static.f64_values[479]=(self.scalar_static.f64_values[478]).exp();
        self.scalar_static.f64_values[480]=(1.0+self.scalar_static.f64_values[479]);
        self.scalar_static.f64_values[481]=(self.scalar_static.f64_values[480]).ln();
        self.scalar_static.f64_values[482]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[481]);
        self.scalar_static.f64_values[483]=(0.05+self.scalar_static.f64_values[482]);
        self.scalar_static.f64_values[484]=(if self.scalar_static.bool_values[85]{self.scalar_static.f64_values[483]}else{self.scalar_static.f64_values[477]});
        self.scalar_static.f64_values[485]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[375]);
        self.scalar_static.f64_values[486]=(self.scalar_static.f64_values[424]+self.scalar_static.f64_values[485]);
        self.scalar_static.f64_values[487]=(self.scalar_static.f64_values[448]+self.scalar_static.f64_values[486]);
        self.scalar_static.f64_values[488]=(0.05-self.scalar_static.f64_values[487]);
        self.scalar_static.f64_values[489]=(self.scalar_static.f64_values[488]/self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[86]=(0.05<self.scalar_static.f64_values[487]);
        self.scalar_static.f64_values[490]=(if self.scalar_static.bool_values[86]{1.0}else{0.0});
        self.scalar_static.f64_values[491]=(self.scalar_static.f64_values[489]).exp();
        self.scalar_static.f64_values[492]=(1.0+self.scalar_static.f64_values[491]);
        self.scalar_static.f64_values[493]=(self.scalar_static.f64_values[492]).ln();
        self.scalar_static.f64_values[494]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[493]);
        self.scalar_static.f64_values[495]=(self.scalar_static.f64_values[487]+self.scalar_static.f64_values[494]);
        self.scalar_static.f64_values[496]=(if ((self.scalar_static.f64_values[490])!=0.0){self.scalar_static.f64_values[495]}else{0.0});
        self.scalar_static.bool_values[87]=(!((self.scalar_static.f64_values[490])!=0.0));
        self.scalar_static.f64_values[497]=(-self.scalar_static.f64_values[489]);
        self.scalar_static.f64_values[498]=(self.scalar_static.f64_values[497]).exp();
        self.scalar_static.f64_values[499]=(1.0+self.scalar_static.f64_values[498]);
        self.scalar_static.f64_values[500]=(self.scalar_static.f64_values[499]).ln();
        self.scalar_static.f64_values[501]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[500]);
        self.scalar_static.f64_values[502]=(0.05+self.scalar_static.f64_values[501]);
        self.scalar_static.f64_values[503]=(if self.scalar_static.bool_values[87]{self.scalar_static.f64_values[502]}else{self.scalar_static.f64_values[496]});
        self.scalar_static.f64_values[504]=(self.scalar_static.f64_values[375]*self.scalar_static.f64_values[89]);
        self.scalar_static.f64_values[505]=(self.scalar_static.f64_values[424]+self.scalar_static.f64_values[504]);
        self.scalar_static.f64_values[506]=(self.scalar_static.f64_values[427]*self.scalar_static.f64_values[90]);
        self.scalar_static.f64_values[507]=(self.scalar_static.f64_values[505]+self.scalar_static.f64_values[506]);
        self.scalar_static.f64_values[508]=(0.05-self.scalar_static.f64_values[507]);
        self.scalar_static.f64_values[509]=(self.scalar_static.f64_values[508]/self.scalar_static.f64_values[376]);
        self.scalar_static.bool_values[88]=(0.05<self.scalar_static.f64_values[507]);
        self.scalar_static.f64_values[510]=(if self.scalar_static.bool_values[88]{1.0}else{0.0});
        self.scalar_static.f64_values[511]=(self.scalar_static.f64_values[509]).exp();
        self.scalar_static.f64_values[512]=(1.0+self.scalar_static.f64_values[511]);
        self.scalar_static.f64_values[513]=(self.scalar_static.f64_values[512]).ln();
        self.scalar_static.f64_values[514]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[513]);
        self.scalar_static.f64_values[515]=(self.scalar_static.f64_values[507]+self.scalar_static.f64_values[514]);
        self.scalar_static.f64_values[516]=(if ((self.scalar_static.f64_values[510])!=0.0){self.scalar_static.f64_values[515]}else{0.0});
        self.scalar_static.bool_values[89]=(!((self.scalar_static.f64_values[510])!=0.0));
        self.scalar_static.f64_values[517]=(-self.scalar_static.f64_values[509]);
        self.scalar_static.f64_values[518]=(self.scalar_static.f64_values[517]).exp();
        self.scalar_static.f64_values[519]=(1.0+self.scalar_static.f64_values[518]);
        self.scalar_static.f64_values[520]=(self.scalar_static.f64_values[519]).ln();
        self.scalar_static.f64_values[521]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[520]);
        self.scalar_static.f64_values[522]=(0.05+self.scalar_static.f64_values[521]);
        self.scalar_static.f64_values[523]=(if self.scalar_static.bool_values[89]{self.scalar_static.f64_values[522]}else{self.scalar_static.f64_values[516]});
        self.scalar_static.f64_values[524]=(1.0/self.scalar_static.f64_values[445]);
        self.scalar_static.f64_values[525]=(1.0/self.scalar_static.f64_values[503]);
        self.scalar_static.f64_values[526]=(self.scalar_static.f64_values[47]*self.scalar_static.f64_values[524]);
        self.scalar_static.f64_values[527]=f64::powf(self.scalar_static.f64_values[526],self.scalar_static.f64_values[18]);
        self.scalar_static.f64_values[528]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[525]);
        self.scalar_static.f64_values[529]=f64::powf(self.scalar_static.f64_values[528],self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[530]=(self.scalar_static.f64_values[527]*self.scalar_static.f64_values[91]);
        self.scalar_static.f64_values[531]=(self.scalar_static.f64_values[49]/self.scalar_static.f64_values[503]);
        self.scalar_static.f64_values[532]=f64::powf(self.scalar_static.f64_values[531],self.scalar_static.f64_values[50]);
        self.scalar_static.f64_values[533]=(self.scalar_static.f64_values[93]*self.scalar_static.f64_values[532]);
        self.scalar_static.f64_values[534]=(self.scalar_static.f64_values[92]+self.scalar_static.f64_values[533]);
        self.scalar_static.f64_values[535]=(1.0/self.scalar_static.f64_values[534]);
        self.scalar_static.f64_values[536]=(self.scalar_static.f64_values[534]*self.scalar_static.f64_values[94]);
        self.scalar_static.f64_values[537]=(self.scalar_static.f64_values[92]*self.scalar_static.f64_values[535]);
        self.scalar_static.f64_values[538]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[96]);
        self.scalar_static.f64_values[539]=(self.scalar_static.f64_values[538]).exp();
        self.scalar_static.f64_values[540]=(self.scalar_static.f64_values[95]*self.scalar_static.f64_values[539]);
        self.scalar_static.bool_values[90]=(self.scalar_static.f64_values[540]<self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[541]=(if self.scalar_static.bool_values[90]{1.0}else{0.0});
        self.scalar_static.f64_values[542]=(if ((self.scalar_static.f64_values[541])!=0.0){self.scalar_static.f64_values[16]}else{self.scalar_static.f64_values[540]});
        self.scalar_static.f64_values[543]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[544]=(self.scalar_static.f64_values[543]).exp();
        self.scalar_static.f64_values[545]=(self.scalar_static.f64_values[97]*self.scalar_static.f64_values[544]);
        self.scalar_static.f64_values[546]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[102]);
        self.scalar_static.f64_values[547]=(self.scalar_static.f64_values[546]).exp();
        self.scalar_static.f64_values[548]=(self.scalar_static.f64_values[101]*self.scalar_static.f64_values[547]);
        self.scalar_static.bool_values[91]=(self.scalar_static.f64_values[548]<self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[549]=(if self.scalar_static.bool_values[91]{1.0}else{0.0});
        self.scalar_static.f64_values[550]=(if ((self.scalar_static.f64_values[549])!=0.0){self.scalar_static.f64_values[16]}else{self.scalar_static.f64_values[548]});
        self.scalar_static.f64_values[551]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[104]);
        self.scalar_static.f64_values[552]=(self.scalar_static.f64_values[551]).exp();
        self.scalar_static.f64_values[553]=(self.scalar_static.f64_values[103]*self.scalar_static.f64_values[552]);
        self.scalar_static.f64_values[554]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[555]=(self.scalar_static.f64_values[554]).exp();
        self.scalar_static.f64_values[556]=(self.scalar_static.f64_values[105]*self.scalar_static.f64_values[555]);
        self.scalar_static.f64_values[557]=(self.scalar_static.f64_values[555]*self.scalar_static.f64_values[107]);
        self.scalar_static.f64_values[558]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[109]);
        self.scalar_static.f64_values[559]=(self.scalar_static.f64_values[558]).exp();
        self.scalar_static.f64_values[560]=(self.scalar_static.f64_values[108]*self.scalar_static.f64_values[559]);
        self.scalar_static.f64_values[561]=(self.scalar_static.f64_values[379]*self.scalar_static.f64_values[110]);
        self.scalar_static.f64_values[562]=(1.0+self.scalar_static.f64_values[561]);
        self.scalar_static.f64_values[563]=(self.scalar_static.f64_values[112]*self.scalar_static.f64_values[562]);
        self.scalar_static.f64_values[564]=(if ((self.scalar_static.f64_values[111])!=0.0){self.scalar_static.f64_values[563]}else{0.0});
        self.scalar_static.f64_values[565]=(self.scalar_static.f64_values[564]-1.0);
        self.scalar_static.f64_values[566]=(self.scalar_static.f64_values[565]/0.001);
        self.scalar_static.f64_values[567]=(if ((self.scalar_static.f64_values[111])!=0.0){self.scalar_static.f64_values[566]}else{self.scalar_static.f64_values[509]});
        self.scalar_static.bool_values[92]=(self.scalar_static.f64_values[564]<1.0);
        self.scalar_static.f64_values[568]=(if self.scalar_static.bool_values[92]{1.0}else{0.0});
        self.scalar_static.bool_values[93]=(((self.scalar_static.f64_values[111])!=0.0)&&((self.scalar_static.f64_values[568])!=0.0));
        self.scalar_static.f64_values[569]=(self.scalar_static.f64_values[567]).exp();
        self.scalar_static.f64_values[570]=(1.0+self.scalar_static.f64_values[569]);
        self.scalar_static.f64_values[571]=(self.scalar_static.f64_values[570]).ln();
        self.scalar_static.f64_values[572]=(0.001*self.scalar_static.f64_values[571]);
        self.scalar_static.f64_values[573]=(1.0+self.scalar_static.f64_values[572]);
        self.scalar_static.f64_values[574]=(if self.scalar_static.bool_values[93]{self.scalar_static.f64_values[573]}else{self.scalar_static.f64_values[564]});
        self.scalar_static.bool_values[94]=(!((self.scalar_static.f64_values[568])!=0.0));
        self.scalar_static.bool_values[95]=(((self.scalar_static.f64_values[111])!=0.0)&&self.scalar_static.bool_values[94]);
        self.scalar_static.f64_values[575]=(-self.scalar_static.f64_values[567]);
        self.scalar_static.f64_values[576]=(self.scalar_static.f64_values[575]).exp();
        self.scalar_static.f64_values[577]=(1.0+self.scalar_static.f64_values[576]);
        self.scalar_static.f64_values[578]=(self.scalar_static.f64_values[577]).ln();
        self.scalar_static.f64_values[579]=(0.001*self.scalar_static.f64_values[578]);
        self.scalar_static.f64_values[580]=(self.scalar_static.f64_values[574]+self.scalar_static.f64_values[579]);
        self.scalar_static.f64_values[581]=(if self.scalar_static.bool_values[95]{self.scalar_static.f64_values[580]}else{self.scalar_static.f64_values[574]});
        self.scalar_static.f64_values[582]=(self.scalar_static.f64_values[581]-0.0006931471805599453);
        self.scalar_static.f64_values[583]=(if ((self.scalar_static.f64_values[111])!=0.0){self.scalar_static.f64_values[582]}else{0.0});
        self.scalar_static.f64_values[584]=(if self.scalar_static.bool_values[9]{self.scalar_static.f64_values[112]}else{self.scalar_static.f64_values[583]});
        self.scalar_static.f64_values[585]=(self.scalar_static.f64_values[379]*self.scalar_static.f64_values[113]);
        self.scalar_static.f64_values[586]=(1.0+self.scalar_static.f64_values[585]);
        self.scalar_static.f64_values[587]=(self.scalar_static.f64_values[115]*self.scalar_static.f64_values[586]);
        self.scalar_static.f64_values[588]=(if ((self.scalar_static.f64_values[114])!=0.0){self.scalar_static.f64_values[587]}else{0.0});
        self.scalar_static.f64_values[589]=(self.scalar_static.f64_values[588]-1.0);
        self.scalar_static.f64_values[590]=(self.scalar_static.f64_values[589]/0.001);
        self.scalar_static.f64_values[591]=(if ((self.scalar_static.f64_values[114])!=0.0){self.scalar_static.f64_values[590]}else{self.scalar_static.f64_values[567]});
        self.scalar_static.bool_values[96]=(self.scalar_static.f64_values[588]<1.0);
        self.scalar_static.f64_values[592]=(if self.scalar_static.bool_values[96]{1.0}else{0.0});
        self.scalar_static.bool_values[97]=(((self.scalar_static.f64_values[114])!=0.0)&&((self.scalar_static.f64_values[592])!=0.0));
        self.scalar_static.f64_values[593]=(self.scalar_static.f64_values[591]).exp();
        self.scalar_static.f64_values[594]=(1.0+self.scalar_static.f64_values[593]);
        self.scalar_static.f64_values[595]=(self.scalar_static.f64_values[594]).ln();
        self.scalar_static.f64_values[596]=(0.001*self.scalar_static.f64_values[595]);
        self.scalar_static.f64_values[597]=(1.0+self.scalar_static.f64_values[596]);
        self.scalar_static.f64_values[598]=(if self.scalar_static.bool_values[97]{self.scalar_static.f64_values[597]}else{self.scalar_static.f64_values[588]});
        self.scalar_static.bool_values[98]=(!((self.scalar_static.f64_values[592])!=0.0));
        self.scalar_static.bool_values[99]=(((self.scalar_static.f64_values[114])!=0.0)&&self.scalar_static.bool_values[98]);
        self.scalar_static.f64_values[599]=(-self.scalar_static.f64_values[591]);
        self.scalar_static.f64_values[600]=(self.scalar_static.f64_values[599]).exp();
        self.scalar_static.f64_values[601]=(1.0+self.scalar_static.f64_values[600]);
        self.scalar_static.f64_values[602]=(self.scalar_static.f64_values[601]).ln();
        self.scalar_static.f64_values[603]=(0.001*self.scalar_static.f64_values[602]);
        self.scalar_static.f64_values[604]=(self.scalar_static.f64_values[598]+self.scalar_static.f64_values[603]);
        self.scalar_static.f64_values[605]=(if self.scalar_static.bool_values[99]{self.scalar_static.f64_values[604]}else{self.scalar_static.f64_values[598]});
        self.scalar_static.f64_values[606]=(self.scalar_static.f64_values[605]-0.0006931471805599453);
        self.scalar_static.f64_values[607]=(if ((self.scalar_static.f64_values[114])!=0.0){self.scalar_static.f64_values[606]}else{0.0});
        self.scalar_static.f64_values[608]=(if self.scalar_static.bool_values[11]{self.scalar_static.f64_values[115]}else{self.scalar_static.f64_values[607]});
        self.scalar_static.f64_values[609]=(self.scalar_static.f64_values[379]*self.scalar_static.f64_values[117]);
        self.scalar_static.f64_values[610]=(1.0+self.scalar_static.f64_values[609]);
        self.scalar_static.f64_values[611]=(self.scalar_static.f64_values[116]*self.scalar_static.f64_values[610]);
        self.scalar_static.f64_values[612]=(self.scalar_static.f64_values[611]*self.scalar_static.f64_values[611]);
        self.scalar_static.bool_values[100]=(self.scalar_static.f64_values[611]<0.0);
        self.scalar_static.f64_values[613]=(if self.scalar_static.bool_values[100]{1.0}else{0.0});
        self.scalar_static.f64_values[614]=(1e-6+self.scalar_static.f64_values[612]);
        self.scalar_static.f64_values[615]=(self.scalar_static.f64_values[614]).sqrt();
        self.scalar_static.f64_values[616]=(self.scalar_static.f64_values[615]-self.scalar_static.f64_values[611]);
        self.scalar_static.f64_values[617]=(5e-7/self.scalar_static.f64_values[616]);
        self.scalar_static.f64_values[618]=(if ((self.scalar_static.f64_values[613])!=0.0){self.scalar_static.f64_values[617]}else{0.0});
        self.scalar_static.bool_values[101]=(!((self.scalar_static.f64_values[613])!=0.0));
        self.scalar_static.f64_values[619]=(self.scalar_static.f64_values[611]+self.scalar_static.f64_values[615]);
        self.scalar_static.f64_values[620]=(0.5*self.scalar_static.f64_values[619]);
        self.scalar_static.f64_values[621]=(if self.scalar_static.bool_values[101]{self.scalar_static.f64_values[620]}else{self.scalar_static.f64_values[618]});
        self.scalar_static.f64_values[622]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[122]);
        self.scalar_static.f64_values[623]=(self.scalar_static.f64_values[622]/self.scalar_static.f64_values[584]);
        self.scalar_static.f64_values[624]=(self.scalar_static.f64_values[623]).exp();
        self.scalar_static.f64_values[625]=(self.scalar_static.f64_values[118]*self.scalar_static.f64_values[624]);
        self.scalar_static.f64_values[626]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[123]);
        self.scalar_static.f64_values[627]=(self.scalar_static.f64_values[626]/self.scalar_static.f64_values[584]);
        self.scalar_static.f64_values[628]=(self.scalar_static.f64_values[627]).exp();
        self.scalar_static.f64_values[629]=(self.scalar_static.f64_values[625]*self.scalar_static.f64_values[628]);
        self.scalar_static.f64_values[630]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[125]);
        self.scalar_static.f64_values[631]=(self.scalar_static.f64_values[630]).exp();
        self.scalar_static.f64_values[632]=(self.scalar_static.f64_values[124]*self.scalar_static.f64_values[631]);
        self.scalar_static.f64_values[633]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[634]=(self.scalar_static.f64_values[633]).exp();
        self.scalar_static.f64_values[635]=(self.scalar_static.f64_values[126]*self.scalar_static.f64_values[634]);
        self.scalar_static.f64_values[636]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[132]);
        self.scalar_static.f64_values[637]=(self.scalar_static.f64_values[636]).exp();
        self.scalar_static.f64_values[638]=(self.scalar_static.f64_values[129]*self.scalar_static.f64_values[637]);
        self.scalar_static.f64_values[639]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[640]=(self.scalar_static.f64_values[639]/self.scalar_static.f64_values[130]);
        self.scalar_static.f64_values[641]=(self.scalar_static.f64_values[640]).exp();
        self.scalar_static.f64_values[642]=(self.scalar_static.f64_values[638]*self.scalar_static.f64_values[641]);
        self.scalar_static.f64_values[643]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[138]);
        self.scalar_static.f64_values[644]=(self.scalar_static.f64_values[643]).exp();
        self.scalar_static.f64_values[645]=(self.scalar_static.f64_values[135]*self.scalar_static.f64_values[644]);
        self.scalar_static.f64_values[646]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[139]);
        self.scalar_static.f64_values[647]=(self.scalar_static.f64_values[646]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[648]=(self.scalar_static.f64_values[647]).exp();
        self.scalar_static.f64_values[649]=(self.scalar_static.f64_values[645]*self.scalar_static.f64_values[648]);
        self.scalar_static.f64_values[650]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[651]=(self.scalar_static.f64_values[650]/self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[652]=(self.scalar_static.f64_values[651]).exp();
        self.scalar_static.f64_values[653]=(self.scalar_static.f64_values[140]*self.scalar_static.f64_values[652]);
        self.scalar_static.f64_values[654]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[145]);
        self.scalar_static.f64_values[655]=(self.scalar_static.f64_values[654]/self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[656]=(self.scalar_static.f64_values[655]).exp();
        self.scalar_static.f64_values[657]=(self.scalar_static.f64_values[653]*self.scalar_static.f64_values[656]);
        self.scalar_static.f64_values[658]=(self.scalar_static.f64_values[650]/self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[659]=(self.scalar_static.f64_values[658]).exp();
        self.scalar_static.f64_values[660]=(self.scalar_static.f64_values[146]*self.scalar_static.f64_values[659]);
        self.scalar_static.f64_values[661]=(self.scalar_static.f64_values[654]/self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[662]=(self.scalar_static.f64_values[661]).exp();
        self.scalar_static.f64_values[663]=(self.scalar_static.f64_values[660]*self.scalar_static.f64_values[662]);
        self.scalar_static.f64_values[664]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[152]);
        self.scalar_static.f64_values[665]=(self.scalar_static.f64_values[664]/self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[666]=(self.scalar_static.f64_values[665]).exp();
        self.scalar_static.f64_values[667]=(self.scalar_static.f64_values[150]*self.scalar_static.f64_values[666]);
        self.scalar_static.f64_values[668]=(if ((self.scalar_static.f64_values[149])!=0.0){self.scalar_static.f64_values[667]}else{0.0});
        self.scalar_static.f64_values[669]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[155]);
        self.scalar_static.f64_values[670]=(self.scalar_static.f64_values[669]).exp();
        self.scalar_static.f64_values[671]=(self.scalar_static.f64_values[153]*self.scalar_static.f64_values[670]);
        self.scalar_static.f64_values[672]=(if ((self.scalar_static.f64_values[149])!=0.0){self.scalar_static.f64_values[671]}else{0.0});
        self.scalar_static.f64_values[673]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[158]);
        self.scalar_static.f64_values[674]=(self.scalar_static.f64_values[673]/self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[675]=(self.scalar_static.f64_values[674]).exp();
        self.scalar_static.f64_values[676]=(self.scalar_static.f64_values[156]*self.scalar_static.f64_values[675]);
        self.scalar_static.f64_values[677]=(if ((self.scalar_static.f64_values[149])!=0.0){self.scalar_static.f64_values[676]}else{0.0});
        self.scalar_static.f64_values[678]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[161]);
        self.scalar_static.f64_values[679]=(self.scalar_static.f64_values[678]).exp();
        self.scalar_static.f64_values[680]=(self.scalar_static.f64_values[159]*self.scalar_static.f64_values[679]);
        self.scalar_static.f64_values[681]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[163]);
        self.scalar_static.f64_values[682]=(self.scalar_static.f64_values[681]).exp();
        self.scalar_static.f64_values[683]=(self.scalar_static.f64_values[680]*self.scalar_static.f64_values[682]);
        self.scalar_static.f64_values[684]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[167]);
        self.scalar_static.f64_values[685]=(self.scalar_static.f64_values[684]).exp();
        self.scalar_static.f64_values[686]=(self.scalar_static.f64_values[164]*self.scalar_static.f64_values[685]);
        self.scalar_static.f64_values[687]=(self.scalar_static.f64_values[639]/self.scalar_static.f64_values[165]);
        self.scalar_static.f64_values[688]=(self.scalar_static.f64_values[687]).exp();
        self.scalar_static.f64_values[689]=(self.scalar_static.f64_values[686]*self.scalar_static.f64_values[688]);
        self.scalar_static.f64_values[690]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[170]);
        self.scalar_static.f64_values[691]=(self.scalar_static.f64_values[690]).exp();
        self.scalar_static.f64_values[692]=(self.scalar_static.f64_values[168]*self.scalar_static.f64_values[691]);
        self.scalar_static.f64_values[693]=(self.scalar_static.f64_values[639]/self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[694]=(self.scalar_static.f64_values[693]).exp();
        self.scalar_static.f64_values[695]=(self.scalar_static.f64_values[692]*self.scalar_static.f64_values[694]);
        self.scalar_static.f64_values[696]=(self.scalar_static.f64_values[375]).sqrt();
        self.scalar_static.f64_values[697]=(self.scalar_static.f64_values[171]*self.scalar_static.f64_values[696]);
        self.scalar_static.f64_values[698]=(self.scalar_static.f64_values[379]*self.scalar_static.f64_values[172]);
        self.scalar_static.f64_values[699]=(self.scalar_static.f64_values[698]).exp();
        self.scalar_static.f64_values[700]=(self.scalar_static.f64_values[697]*self.scalar_static.f64_values[699]);
        self.scalar_static.f64_values[701]=(self.scalar_static.f64_values[46]*self.scalar_static.f64_values[401]);
        self.scalar_static.f64_values[702]=f64::powf(self.scalar_static.f64_values[701],-0.5);
        self.scalar_static.f64_values[703]=(1.0/self.scalar_static.f64_values[527]);
        self.scalar_static.f64_values[704]=(self.scalar_static.f64_values[401]*self.scalar_static.f64_values[173]);
        self.scalar_static.f64_values[705]=(self.scalar_static.f64_values[401]*self.scalar_static.f64_values[704]);
        self.scalar_static.f64_values[706]=(self.scalar_static.f64_values[702]*self.scalar_static.f64_values[705]);
        self.scalar_static.f64_values[707]=(self.scalar_static.f64_values[703]*self.scalar_static.f64_values[706]);
        self.scalar_static.f64_values[708]=(self.scalar_static.f64_values[47]*self.scalar_static.f64_values[707]);
        self.scalar_static.f64_values[709]=(self.scalar_static.f64_values[524]*self.scalar_static.f64_values[708]);
        self.scalar_static.f64_values[710]=(self.scalar_static.f64_values[46]*self.scalar_static.f64_values[709]);
        self.scalar_static.f64_values[711]=(self.scalar_static.f64_values[46]*self.scalar_static.f64_values[710]);
        self.scalar_static.f64_values[712]=(self.scalar_static.f64_values[702]*self.scalar_static.f64_values[174]);
        self.scalar_static.f64_values[713]=(self.scalar_static.f64_values[445]*self.scalar_static.f64_values[712]);
        self.scalar_static.f64_values[714]=(self.scalar_static.f64_values[445]*self.scalar_static.f64_values[713]);
        self.scalar_static.f64_values[715]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[714]);
        self.scalar_static.f64_values[716]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[715]);
        self.scalar_static.f64_values[717]=(self.scalar_static.f64_values[527]*self.scalar_static.f64_values[716]);
        self.scalar_static.f64_values[718]=(self.scalar_static.f64_values[173]-self.scalar_static.f64_values[711]);
        self.scalar_static.f64_values[719]=(self.scalar_static.f64_values[718]).exp();
        self.scalar_static.f64_values[720]=(self.scalar_static.f64_values[717]*self.scalar_static.f64_values[719]);
        self.scalar_static.f64_values[721]=(self.scalar_static.f64_values[78]*self.scalar_static.f64_values[422]);
        self.scalar_static.f64_values[722]=f64::powf(self.scalar_static.f64_values[721],-0.5);
        self.scalar_static.f64_values[723]=(1.0/self.scalar_static.f64_values[529]);
        self.scalar_static.f64_values[724]=(self.scalar_static.f64_values[422]*self.scalar_static.f64_values[175]);
        self.scalar_static.f64_values[725]=(self.scalar_static.f64_values[422]*self.scalar_static.f64_values[724]);
        self.scalar_static.f64_values[726]=(self.scalar_static.f64_values[722]*self.scalar_static.f64_values[725]);
        self.scalar_static.f64_values[727]=(self.scalar_static.f64_values[723]*self.scalar_static.f64_values[726]);
        self.scalar_static.f64_values[728]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[727]);
        self.scalar_static.f64_values[729]=(self.scalar_static.f64_values[525]*self.scalar_static.f64_values[728]);
        self.scalar_static.f64_values[730]=(self.scalar_static.f64_values[78]*self.scalar_static.f64_values[729]);
        self.scalar_static.f64_values[731]=(self.scalar_static.f64_values[78]*self.scalar_static.f64_values[730]);
        self.scalar_static.f64_values[732]=(self.scalar_static.f64_values[722]*self.scalar_static.f64_values[176]);
        self.scalar_static.f64_values[733]=(self.scalar_static.f64_values[503]*self.scalar_static.f64_values[732]);
        self.scalar_static.f64_values[734]=(self.scalar_static.f64_values[503]*self.scalar_static.f64_values[733]);
        self.scalar_static.f64_values[735]=(self.scalar_static.f64_values[79]*self.scalar_static.f64_values[734]);
        self.scalar_static.f64_values[736]=(self.scalar_static.f64_values[79]*self.scalar_static.f64_values[735]);
        self.scalar_static.f64_values[737]=(self.scalar_static.f64_values[529]*self.scalar_static.f64_values[736]);
        self.scalar_static.f64_values[738]=(self.scalar_static.f64_values[175]-self.scalar_static.f64_values[731]);
        self.scalar_static.f64_values[739]=(self.scalar_static.f64_values[738]).exp();
        self.scalar_static.f64_values[740]=(self.scalar_static.f64_values[737]*self.scalar_static.f64_values[739]);
        self.scalar_static.f64_values[741]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[99]);
        self.scalar_static.f64_values[742]=(self.scalar_static.f64_values[741]).exp();
        self.scalar_static.f64_values[743]=(self.scalar_static.f64_values[742]*self.scalar_static.f64_values[177]);
        self.scalar_static.f64_values[744]=(self.scalar_static.f64_values[535]*self.scalar_static.f64_values[743]);
        self.scalar_static.f64_values[745]=(self.scalar_static.f64_values[742]*self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[746]=(self.scalar_static.f64_values[703]*self.scalar_static.f64_values[745]);
        self.scalar_static.f64_values[747]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[180]);
        self.scalar_static.f64_values[748]=(self.scalar_static.f64_values[747]).exp();
        self.scalar_static.f64_values[749]=(self.scalar_static.f64_values[179]*self.scalar_static.f64_values[748]);
        self.scalar_static.f64_values[750]=(self.scalar_static.f64_values[378]*self.scalar_static.f64_values[182]);
        self.scalar_static.f64_values[751]=(self.scalar_static.f64_values[750]).exp();
        self.scalar_static.f64_values[752]=(self.scalar_static.f64_values[749]*self.scalar_static.f64_values[751]);
        self.scalar_static.f64_values[753]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[185]);
        self.scalar_static.f64_values[754]=(self.scalar_static.f64_values[753]).exp();
        self.scalar_static.f64_values[755]=(self.scalar_static.f64_values[183]*self.scalar_static.f64_values[754]);
        self.scalar_static.f64_values[756]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[187]);
        self.scalar_static.f64_values[757]=(self.scalar_static.f64_values[756]).exp();
        self.scalar_static.f64_values[758]=(self.scalar_static.f64_values[186]*self.scalar_static.f64_values[757]);
        self.scalar_static.f64_values[759]=(self.scalar_static.f64_values[755]+self.scalar_static.f64_values[758]);
        self.scalar_static.f64_values[760]=(self.scalar_static.f64_values[188]*self.scalar_static.f64_values[759]);
        self.scalar_static.f64_values[761]=(self.scalar_static.f64_values[760]/self.scalar_static.f64_values[189]);
        self.scalar_static.f64_values[762]=(self.scalar_static.f64_values[380]*self.scalar_static.f64_values[192]);
        self.scalar_static.f64_values[763]=(self.scalar_static.f64_values[762]).exp();
        self.scalar_static.f64_values[764]=(self.scalar_static.f64_values[190]*self.scalar_static.f64_values[763]);
        self.scalar_static.f64_values[765]=(self.scalar_static.f64_values[374]-300.0);
        self.scalar_static.bool_values[102]=(self.scalar_static.f64_values[374]<525.0);
        self.scalar_static.f64_values[766]=(if self.scalar_static.bool_values[102]{1.0}else{0.0});
        self.scalar_static.f64_values[767]=(self.scalar_static.f64_values[765]*0.00072);
        self.scalar_static.f64_values[768]=(1.0+self.scalar_static.f64_values[767]);
        self.scalar_static.f64_values[769]=(self.scalar_static.f64_values[765]*1.6e-6);
        self.scalar_static.f64_values[770]=(self.scalar_static.f64_values[765]*self.scalar_static.f64_values[769]);
        self.scalar_static.f64_values[771]=(self.scalar_static.f64_values[768]-self.scalar_static.f64_values[770]);
        self.scalar_static.f64_values[772]=(self.scalar_static.f64_values[5]*self.scalar_static.f64_values[771]);
        self.scalar_static.f64_values[773]=(if ((self.scalar_static.f64_values[766])!=0.0){self.scalar_static.f64_values[772]}else{0.0});
        self.scalar_static.bool_values[103]=(!((self.scalar_static.f64_values[766])!=0.0));
        self.scalar_static.f64_values[774]=(if self.scalar_static.bool_values[103]{self.scalar_static.f64_values[193]}else{self.scalar_static.f64_values[773]});
        self.scalar_static.f64_values[775]=(self.scalar_static.f64_values[742]*self.scalar_static.f64_values[194]);
        self.scalar_static.f64_values[776]=(1.0/self.scalar_static.f64_values[553]);
        self.scalar_static.f64_values[777]=(if ((self.scalar_static.f64_values[195])!=0.0){self.scalar_static.f64_values[776]}else{0.0});
        self.scalar_static.bool_values[104]=(self.scalar_static.f64_values[777]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[778]=(if self.scalar_static.bool_values[104]{1.0}else{0.0});
        self.scalar_static.bool_values[105]=(((self.scalar_static.f64_values[195])!=0.0)&&((self.scalar_static.f64_values[778])!=0.0));
        self.scalar_static.f64_values[779]=(if self.scalar_static.bool_values[105]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[777]});
        self.scalar_static.f64_values[780]=(if self.scalar_static.bool_values[14]{0.0}else{self.scalar_static.f64_values[779]});
        self.scalar_static.f64_values[781]=(1.0/self.scalar_static.f64_values[556]);
        self.scalar_static.f64_values[782]=(if ((self.scalar_static.f64_values[196])!=0.0){self.scalar_static.f64_values[781]}else{0.0});
        self.scalar_static.bool_values[106]=(self.scalar_static.f64_values[782]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[783]=(if self.scalar_static.bool_values[106]{1.0}else{0.0});
        self.scalar_static.bool_values[107]=(((self.scalar_static.f64_values[196])!=0.0)&&((self.scalar_static.f64_values[783])!=0.0));
        self.scalar_static.f64_values[784]=(if self.scalar_static.bool_values[107]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[782]});
        self.scalar_static.f64_values[785]=(if self.scalar_static.bool_values[16]{0.0}else{self.scalar_static.f64_values[784]});
        self.scalar_static.f64_values[786]=(1.0/self.scalar_static.f64_values[557]);
        self.scalar_static.f64_values[787]=(if ((self.scalar_static.f64_values[197])!=0.0){self.scalar_static.f64_values[786]}else{0.0});
        self.scalar_static.bool_values[108]=(self.scalar_static.f64_values[787]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[788]=(if self.scalar_static.bool_values[108]{1.0}else{0.0});
        self.scalar_static.bool_values[109]=(((self.scalar_static.f64_values[197])!=0.0)&&((self.scalar_static.f64_values[788])!=0.0));
        self.scalar_static.f64_values[789]=(if self.scalar_static.bool_values[109]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[787]});
        self.scalar_static.f64_values[790]=(if self.scalar_static.bool_values[18]{0.0}else{self.scalar_static.f64_values[789]});
        self.scalar_static.f64_values[791]=(2.0*self.scalar_static.f64_values[376]);
        self.scalar_static.f64_values[792]=(self.scalar_static.f64_values[465]*0.2);
        self.scalar_static.f64_values[793]=(self.scalar_static.f64_values[560]*self.scalar_static.f64_values[201]);
        self.scalar_static.f64_values[794]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[465]);
        self.scalar_static.f64_values[795]=(self.scalar_static.f64_values[794]).exp();
        self.scalar_static.f64_values[796]=(self.scalar_static.f64_values[560]*self.scalar_static.f64_values[202]);
        self.scalar_static.f64_values[797]=(self.scalar_static.f64_values[201]*self.scalar_static.f64_values[796]);
        self.scalar_static.f64_values[798]=(0.1*self.scalar_static.f64_values[503]);
        self.scalar_static.f64_values[799]=(self.scalar_static.f64_values[376]*1e-5);
        self.scalar_static.f64_values[800]=(self.scalar_static.f64_values[376]*1e-40);
        self.scalar_static.f64_values[801]=(self.scalar_static.f64_values[445]*self.scalar_static.f64_values[217]);
        self.scalar_static.f64_values[802]=(0.1*self.scalar_static.f64_values[445]);
        self.scalar_static.f64_values[803]=(self.scalar_static.f64_values[445]/self.scalar_static.f64_values[218]);
        self.scalar_static.f64_values[804]=(2.0-self.scalar_static.f64_values[537]);
        self.scalar_static.f64_values[805]=(1.0-self.scalar_static.f64_values[537]);
        self.scalar_static.f64_values[806]=(self.scalar_static.f64_values[804]/self.scalar_static.f64_values[805]);
        self.scalar_static.f64_values[807]=f64::powf(self.scalar_static.f64_values[806],self.scalar_static.f64_values[222]);
        self.scalar_static.f64_values[808]=(1.0-self.scalar_static.f64_values[807]);
        self.scalar_static.f64_values[809]=(self.scalar_static.f64_values[503]*self.scalar_static.f64_values[808]);
        self.scalar_static.f64_values[810]=(self.scalar_static.f64_values[503]/self.scalar_static.f64_values[224]);
        self.scalar_static.f64_values[811]=(4.0*self.scalar_static.f64_values[629]);
        self.scalar_static.f64_values[812]=(self.scalar_static.f64_values[811]/self.scalar_static.f64_values[632]);
        self.scalar_static.f64_values[813]=(1.0/self.scalar_static.f64_values[608]);
        self.scalar_static.f64_values[814]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[775]);
        self.scalar_static.f64_values[815]=(self.scalar_static.f64_values[814]).exp();
        self.scalar_static.f64_values[816]=(self.scalar_static.f64_values[815]-1.0);
        self.scalar_static.f64_values[817]=(self.scalar_static.f64_values[629]*self.scalar_static.f64_values[226]);
        self.scalar_static.f64_values[818]=(2.0*self.scalar_static.f64_values[668]);
        self.scalar_static.f64_values[819]=(2.0*self.scalar_static.f64_values[677]);
        self.scalar_static.f64_values[820]=(2.0*self.scalar_static.f64_values[720]);
        self.scalar_static.f64_values[821]=(2.0*self.scalar_static.f64_values[740]);
        self.scalar_static.f64_values[822]=(2.0*self.scalar_static.f64_values[683]);
        self.scalar_static.f64_values[823]=(4.0*self.scalar_static.f64_values[683]);
        self.scalar_static.f64_values[824]=(self.scalar_static.f64_values[823]/self.scalar_static.f64_values[635]);
        self.scalar_static.f64_values[825]=(self.scalar_static.f64_values[683]*self.scalar_static.f64_values[243]);
        self.scalar_static.f64_values[826]=(self.scalar_static.f64_values[6]*self.scalar_static.f64_values[683]);
        self.scalar_static.f64_values[827]=(self.scalar_static.f64_values[553]*self.scalar_static.f64_values[826]);
        self.scalar_static.f64_values[828]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[827]}else{0.0});
        self.scalar_static.f64_values[829]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[828]);
        self.scalar_static.f64_values[830]=(self.scalar_static.f64_values[829]).ln();
        self.scalar_static.f64_values[831]=(2.0-self.scalar_static.f64_values[830]);
        self.scalar_static.f64_values[832]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[831]);
        self.scalar_static.f64_values[833]=(if self.scalar_static.bool_values[44]{self.scalar_static.f64_values[832]}else{0.0});
        self.scalar_static.f64_values[834]=(-self.scalar_static.f64_values[621]);
        self.scalar_static.f64_values[835]=(self.scalar_static.f64_values[271]/self.scalar_static.f64_values[621]);
        self.scalar_static.f64_values[836]=(self.scalar_static.f64_values[4]/self.scalar_static.f64_values[774]);
        self.scalar_static.f64_values[837]=(-self.scalar_static.f64_values[774]);
        self.scalar_static.f64_values[838]=(self.scalar_static.f64_values[530]*self.scalar_static.f64_values[294]);
        self.scalar_static.f64_values[839]=(self.scalar_static.f64_values[530]*self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[840]=(self.scalar_static.f64_values[536]*self.scalar_static.f64_values[295]);
        self.scalar_static.f64_values[841]=(self.scalar_static.f64_values[632]*self.scalar_static.f64_values[755]);
        self.scalar_static.f64_values[842]=(0.5*self.scalar_static.f64_values[841]);
        self.scalar_static.f64_values[843]=(self.scalar_static.f64_values[632]*self.scalar_static.f64_values[752]);
        self.scalar_static.f64_values[844]=(self.scalar_static.f64_values[629]/self.scalar_static.f64_values[632]);
        self.scalar_static.f64_values[845]=f64::powf(self.scalar_static.f64_values[844],self.scalar_static.f64_values[298]);
        self.scalar_static.f64_values[846]=(self.scalar_static.f64_values[843]*self.scalar_static.f64_values[845]);
        self.scalar_static.f64_values[847]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[297]);
        self.scalar_static.f64_values[848]=(4.0*self.scalar_static.f64_values[758]);
        self.scalar_static.f64_values[849]=(self.scalar_static.f64_values[376]*self.scalar_static.f64_values[848]);
        self.scalar_static.f64_values[850]=(self.scalar_static.f64_values[849]/self.scalar_static.f64_values[560]);
        self.scalar_static.f64_values[851]=(0.5*self.scalar_static.f64_values[850]);
        self.scalar_static.f64_values[852]=(0.5*self.scalar_static.f64_values[761]);
        self.scalar_static.f64_values[853]=(self.scalar_static.f64_values[764]*self.scalar_static.f64_values[822]);
        self.scalar_static.f64_values[854]=(self.scalar_static.f64_values[761]*self.scalar_static.f64_values[303]);
        self.scalar_static.f64_values[855]=(self.scalar_static.f64_values[764]*self.scalar_static.f64_values[825]);
        self.scalar_static.f64_values[856]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[377]);
        self.scalar_static.f64_values[857]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[858]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[584]);
        self.scalar_static.f64_values[859]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[584]);
        self.scalar_static.f64_values[860]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[861]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[323]);
        self.scalar_static.f64_values[862]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[324]);
        self.scalar_static.f64_values[863]=(self.scalar_static.f64_values[321]/self.scalar_static.f64_values[802]);
        self.scalar_static.f64_values[864]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[802]);
        self.scalar_static.f64_values[865]=(-self.scalar_static.f64_values[863]);
        self.scalar_static.f64_values[866]=(-self.scalar_static.f64_values[864]);
        self.scalar_static.f64_values[867]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[537]);
        self.scalar_static.f64_values[868]=(self.scalar_static.f64_values[537]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[869]=(self.scalar_static.f64_values[813]-1.0);
        self.scalar_static.f64_values[870]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[871]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[872]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[873]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[874]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[130]);
        self.scalar_static.f64_values[875]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[130]);
        self.scalar_static.f64_values[876]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[165]);
        self.scalar_static.f64_values[877]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[165]);
        self.scalar_static.f64_values[878]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[879]=(self.scalar_static.f64_values[860]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[880]=(self.scalar_static.f64_values[861]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[881]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[882]=(self.scalar_static.f64_values[857]/self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[883]=(self.scalar_static.f64_values[856]/self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[884]=(self.scalar_static.f64_values[524]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[885]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[524]);
        self.scalar_static.f64_values[886]=(self.scalar_static.f64_values[711]*self.scalar_static.f64_values[339]);
        self.scalar_static.f64_values[887]=(self.scalar_static.f64_values[711]*self.scalar_static.f64_values[340]);
        self.scalar_static.f64_values[888]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[525]);
        self.scalar_static.f64_values[889]=(self.scalar_static.f64_values[525]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[890]=(-self.scalar_static.f64_values[888]);
        self.scalar_static.f64_values[891]=(-self.scalar_static.f64_values[889]);
        self.scalar_static.f64_values[892]=(self.scalar_static.f64_values[731]*self.scalar_static.f64_values[344]);
        self.scalar_static.f64_values[893]=(self.scalar_static.f64_values[731]*self.scalar_static.f64_values[345]);
        self.scalar_static.f64_values[894]=(self.scalar_static.f64_values[835]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[895]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[835]);
        self.scalar_static.f64_values[896]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[897]=(self.scalar_static.f64_values[322]/self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[898]=(self.scalar_static.f64_values[323]/self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[899]=(self.scalar_static.f64_values[321]/self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[900]=(-self.scalar_static.f64_values[896]);
        self.scalar_static.f64_values[901]=(-self.scalar_static.f64_values[897]);
        self.scalar_static.f64_values[902]=(-self.scalar_static.f64_values[898]);
        self.scalar_static.f64_values[903]=(-self.scalar_static.f64_values[899]);
        self.scalar_static.f64_values[904]=(self.scalar_static.f64_values[537]*self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[905]=(self.scalar_static.f64_values[537]*self.scalar_static.f64_values[323]);
        self.scalar_static.f64_values[906]=(self.scalar_static.f64_values[324]/self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[907]=(-self.scalar_static.f64_values[906]);
        self.scalar_static.f64_values[908]=(self.scalar_static.f64_values[537]*self.scalar_static.f64_values[324]);
        self.scalar_static.f64_values[909]=(self.scalar_static.f64_values[321]/self.scalar_static.f64_values[847]);
        self.scalar_static.f64_values[910]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[847]);
        self.scalar_static.f64_values[911]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[359]);
        self.scalar_static.f64_values[912]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[360]);
        self.scalar_static.f64_values[913]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[361]);
        self.scalar_static.f64_values[914]=(self.scalar_static.f64_values[377]*self.scalar_static.f64_values[362]);
        self.scalar_static.f64_values[915]=(if ((self.scalar_static.f64_values[305])!=0.0){self.scalar_static.f64_values[863]}else{0.0});
        self.scalar_static.f64_values[916]=(if ((self.scalar_static.f64_values[305])!=0.0){self.scalar_static.f64_values[864]}else{0.0});
        self.scalar_static.f64_values[917]=(-self.scalar_static.f64_values[915]);
        self.scalar_static.f64_values[918]=(-self.scalar_static.f64_values[916]);
        self.scalar_static.f64_values[919]=(self.scalar_static.f64_values[366]/self.scalar_static.f64_values[542]);
        self.scalar_static.f64_values[920]=(self.scalar_static.f64_values[367]/self.scalar_static.f64_values[542]);
        self.scalar_static.f64_values[921]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[919]);
        self.scalar_static.f64_values[922]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[920]);
        self.scalar_static.f64_values[923]=(self.scalar_static.f64_values[366]/self.scalar_static.f64_values[550]);
        self.scalar_static.f64_values[924]=(self.scalar_static.f64_values[367]/self.scalar_static.f64_values[550]);
        self.scalar_static.f64_values[925]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[923]);
        self.scalar_static.f64_values[926]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[924]);
        self.scalar_static.f64_values[927]=(self.scalar_static.f64_values[780]*self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[928]=(self.scalar_static.f64_values[780]*self.scalar_static.f64_values[372]);
        self.scalar_static.f64_values[929]=(self.scalar_static.f64_values[780]*self.scalar_static.f64_values[373]);
        self.scalar_static.f64_values[930]=(self.scalar_static.f64_values[780]*self.scalar_static.f64_values[367]);
        self.scalar_static.f64_values[931]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[927]);
        self.scalar_static.f64_values[932]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[928]);
        self.scalar_static.f64_values[933]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[929]);
        self.scalar_static.f64_values[934]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[930]);
        self.scalar_static.f64_values[935]=(self.scalar_static.f64_values[785]*self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[936]=(self.scalar_static.f64_values[785]*self.scalar_static.f64_values[367]);
        self.scalar_static.f64_values[937]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[935]);
        self.scalar_static.f64_values[938]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[936]);
        self.scalar_static.f64_values[939]=(if ((self.scalar_static.f64_values[196])!=0.0){self.scalar_static.f64_values[937]}else{0.0});
        self.scalar_static.f64_values[940]=(if ((self.scalar_static.f64_values[196])!=0.0){self.scalar_static.f64_values[938]}else{0.0});
        self.scalar_static.f64_values[941]=(self.scalar_static.f64_values[790]*self.scalar_static.f64_values[367]);
        self.scalar_static.f64_values[942]=(self.scalar_static.f64_values[790]*self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[943]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[941]);
        self.scalar_static.f64_values[944]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[942]);
        self.scalar_static.f64_values[945]=(if ((self.scalar_static.f64_values[197])!=0.0){self.scalar_static.f64_values[943]}else{0.0});
        self.scalar_static.f64_values[946]=(if ((self.scalar_static.f64_values[197])!=0.0){self.scalar_static.f64_values[944]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
