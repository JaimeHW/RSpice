#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 156],
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
            const DEFAULTS_0: [f64; 150] = [
                0.0, 1.0, 505.5, 1.0, 25.0, 1.0, 1.0, 0.0,
                1.0, 2.2e-17, 1.0, 1.0, 0.1, 2.5, 44.0, 1.0,
                1.0000000000000001e-19, 1.0, 0.0, 1.0, 2.7000000000000005e-15, 2.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.68, 0.0, 3.1400000000000002e-18, 0.014289999999999999, 1e-15,
                2.0, 0.63, 0.0, 22.0, 0.0, 22.0, 1e-6, 1.0,
                400.0, -0.37, 0.5, 25.0, 0.1, 1.1e-6, 3.0, 0.3,
                0.004, -0.37, -0.37, 0.3, 0.004, 1.0, 5.0, 23.0,
                18.0, 12.0, 0.0, 0.0, 150.0, 1250.0, 0.004, 0.3,
                0.68, 7.3e-14, 0.95, 0.4, 0.4, 0.0, 7.800000000000001e-14, 0.68,
                0.5, 0.0, 0.0, 0.35, 0.5, 0.032, 0.0, 0.0,
                0.68, 100.0, 4.0, 1000.0, 0.0, 1.0, 2e-12, 4.2e-12,
                4.1e-11, 5.2e-10, 1e-11, 1.0, 0.0, 0.0, 0.3333333333333333, 0.0,
                0.3, 0.0, 1.0, 2.5, 2.5, 0.62, 2.0, 1.3,
                2.0, 1.17, 1.12, 1.12, 1.12, 1.12, 1.18, 1.12,
                1.125, 1.15, 1.15, 0.000473, 636.0, 1.15, 0.000473, 636.0,
                0.05, 0.0, 0.0, 0.0, 0.0005, 200.0, 2.0, 2.0,
                2e-11, 2e-11, 0.0, 0.0, 0.0, 4.8000000000000003e-17, 0.0, 0.0005455,
                4.9999999999999996e-5, 3.15e-13, 0.62, 0.34, 1.2, 1.58, 2.0, 0.0,
                0.0, 0.0, 2.0, 400.0, 1e-40, 1e-40,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 150);
            {
                let params = &mut *ptr;
                params[150] = 0.001;
                validate_parameter("minr", params[150], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 0.0, 0.16, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(151), 5);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 158] = [
    ("dta", 0), ("trise", 0), ("dtemp", 0), ("mult", 1), ("version", 2), ("type", 3), ("tref", 4), ("exmod", 5), ("exphi", 6), ("exavl", 7), ("exsub", 8), ("is", 9), ("nff", 10), ("nfr", 11), ("ik", 12), ("ver", 13),
    ("vef", 14), ("issr", 15), ("ibi", 16), ("nbi", 17), ("ibis", 18), ("nbis", 19), ("ibf", 20), ("mlf", 21), ("ibfs", 22), ("mlfs", 23), ("swib1", 24), ("ibinbr", 25), ("ibinbrs", 26), ("vknbr", 27), ("ibinbrqs", 28), ("ibx", 29),
    ("ikbx", 30), ("ibr", 31), ("mlr", 32), ("xext", 33), ("izeb", 34), ("nzeb", 35), ("izcb", 36), ("nzcb", 37), ("vzmin", 38), ("swavl", 39), ("aavl", 40), ("cavl", 41), ("itoavl", 42), ("bavl", 43), ("vdcavl", 44), ("wavl", 45),
    ("vavl", 46), ("sfh", 47), ("ihcavl", 48), ("davl", 49), ("eavl", 50), ("aexavl", 51), ("ionexavl", 52), ("swgemlim", 53), ("re", 54), ("rbc", 55), ("rbv", 56), ("rcc", 57), ("rcblx", 58), ("rcbli", 59), ("rcv", 60), ("scrcv", 61),
    ("ihc", 62), ("axi", 63), ("vdc", 64), ("cje", 65), ("vde", 66), ("pe", 67), ("xcje", 68), ("cbeo", 69), ("cjc", 70), ("vdcctc", 71), ("pc", 72), ("swvchc", 73), ("swvjunc", 74), ("xp", 75), ("mc", 76), ("xcjc", 77),
    ("cbco", 78), ("swqex", 79), ("vdcex", 80), ("vbrcb", 81), ("pbrcb", 82), ("frevcb", 83), ("swjbrcb", 84), ("mtau", 85), ("taue", 86), ("taub", 87), ("tepi", 88), ("taur", 89), ("tauex", 90), ("nex", 91), ("deg", 92), ("xrec", 93),
    ("xqb", 94), ("ke", 95), ("aqbo", 96), ("ae", 97), ("ab", 98), ("aepi", 99), ("aepiex", 100), ("aex", 101), ("ac", 102), ("acx", 103), ("acbl", 104), ("vgb", 105), ("vgbnbrqs", 106), ("vgbnbr", 107), ("vgbnbrs", 108), ("vgknbr", 109),
    ("vgc", 110), ("vge", 111), ("vgcx", 112), ("vgj", 113), ("vgzeb", 114), ("avgeb", 115), ("tvgeb", 116), ("vgzcb", 117), ("avgcb", 118), ("tvgcb", 119), ("dvgte", 120), ("dais", 121), ("tnff", 122), ("tnfr", 123), ("tbavl", 124), ("dtmax", 125),
    ("af", 126), ("afn", 127), ("kf", 128), ("kfn", 129), ("kavl", 130), ("kc", 131), ("ftaun", 132), ("iss", 133), ("icss", 134), ("iks", 135), ("ikcs", 136), ("cjs", 137), ("vds", 138), ("ps", 139), ("vgs", 140), ("as", 141),
    ("asub", 142), ("xisubi", 143), ("swvsch", 144), ("isibrel", 145), ("nfibrel", 146), ("vexlim", 147), ("p0starlim", 148), ("pwlim", 149), ("minr", 150), ("istat", 151), ("vtat", 152), ("ktat", 153), ("vbtbt", 154), ("kbtbt", 155),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 156] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 156] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 156] = [
    "dta", "mult", "version", "type", "tref", "exmod", "exphi", "exavl", "exsub", "is", "nff", "nfr", "ik", "ver", "vef", "issr",
    "ibi", "nbi", "ibis", "nbis", "ibf", "mlf", "ibfs", "mlfs", "swib1", "ibinbr", "ibinbrs", "vknbr", "ibinbrqs", "ibx", "ikbx", "ibr",
    "mlr", "xext", "izeb", "nzeb", "izcb", "nzcb", "vzmin", "swavl", "aavl", "cavl", "itoavl", "bavl", "vdcavl", "wavl", "vavl", "sfh",
    "ihcavl", "davl", "eavl", "aexavl", "ionexavl", "swgemlim", "re", "rbc", "rbv", "rcc", "rcblx", "rcbli", "rcv", "scrcv", "ihc", "axi",
    "vdc", "cje", "vde", "pe", "xcje", "cbeo", "cjc", "vdcctc", "pc", "swvchc", "swvjunc", "xp", "mc", "xcjc", "cbco", "swqex",
    "vdcex", "vbrcb", "pbrcb", "frevcb", "swjbrcb", "mtau", "taue", "taub", "tepi", "taur", "tauex", "nex", "deg", "xrec", "xqb", "ke",
    "aqbo", "ae", "ab", "aepi", "aepiex", "aex", "ac", "acx", "acbl", "vgb", "vgbnbrqs", "vgbnbr", "vgbnbrs", "vgknbr", "vgc", "vge",
    "vgcx", "vgj", "vgzeb", "avgeb", "tvgeb", "vgzcb", "avgcb", "tvgcb", "dvgte", "dais", "tnff", "tnfr", "tbavl", "dtmax", "af", "afn",
    "kf", "kfn", "kavl", "kc", "ftaun", "iss", "icss", "iks", "ikcs", "cjs", "vds", "ps", "vgs", "as", "asub", "xisubi",
    "swvsch", "isibrel", "nfibrel", "vexlim", "p0starlim", "pwlim", "minr", "istat", "vtat", "ktat", "vbtbt", "kbtbt",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 156] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 156] = [
    false, false, false, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 156] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 505.5, label: "505.5" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1e-12, label: "1e-12" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }),
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.02, label: "0.02" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }),
    Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 40.0, label: "40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 156] = [
    None, None, Some(ParameterBound { value: 505.51, label: "505.51" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 0.99, label: "0.99" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, Some(ParameterBound { value: 2000.0, label: "2000.0" }), Some(ParameterBound { value: 500.0, label: "500.0" }), Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 0.99, label: "0.99" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 400.0, label: "400.0" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), None, None,
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 156] = [
    0, 3, 2, 0, 2, 0, 0, 0, 0, 3, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 3, 2, 2,
    2, 0, 2, 2, 2, 2, 3, 0, 2, 3, 3, 3, 0, 2, 2, 2, 2, 3, 3, 3, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 2, 0, 2, 1, 1, 1, 0, 2, 2, 3, 2, 2, 2, 2, 0, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 2, 2,
    2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 2, 0, 0, 0, 0, 2, 2, 0, 0, 0, 2, 2, 3, 0, 2, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 156] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
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
    pub nodes: [usize; 12],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 156]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<10, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<1036, 118>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 156;
    pub const VARIABLE_COUNT: usize = 616;
    pub const DDT_STATE_COUNT: usize = 10;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "3fab5254eac4387a9eab6397c27be80e10ea19aae7d1abc93b8f5771ccd24bdc";
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
        let mut values = Vec::with_capacity(50);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(10);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 50);
        debug_assert_eq!(state.flags.len(), 10);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjt505_va'", name));
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
        self.scalar_static.f64_values[6]=p[33];
        self.scalar_static.f64_values[7]=(1.0-self.scalar_static.f64_values[6]);
        self.scalar_static.f64_values[8]=p[4];
        self.scalar_static.f64_values[9]=(self.scalar_static.f64_values[8]+273.15);
        self.scalar_static.f64_values[10]=p[0];
        self.scalar_static.f64_values[11]=p[150];
        self.scalar_static.bool_values[2]=(0.0==self.scalar_static.f64_values[11]);
        self.scalar_static.f64_values[12]=(if self.scalar_static.bool_values[2]{1.0}else{0.0});
        self.scalar_static.f64_values[13]=(if ((self.scalar_static.f64_values[12])!=0.0){1e-12}else{0.0});
        self.scalar_static.bool_values[3]=(!((self.scalar_static.f64_values[12])!=0.0));
        self.scalar_static.f64_values[14]=(if self.scalar_static.bool_values[3]{self.scalar_static.f64_values[11]}else{self.scalar_static.f64_values[13]});
        self.scalar_static.f64_values[15]=p[1];
        self.scalar_static.f64_values[16]=(self.scalar_static.f64_values[14]*self.scalar_static.f64_values[15]);
        self.scalar_static.f64_values[17]=(1.0/self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[18]=p[134];
        self.scalar_static.bool_values[4]=(self.scalar_static.f64_values[18]>0.0);
        self.scalar_static.f64_values[19]=(if self.scalar_static.bool_values[4]{1.0}else{0.0});
        self.scalar_static.bool_values[5]=(!((self.scalar_static.f64_values[19])!=0.0));
        self.scalar_static.f64_values[20]=p[67];
        self.scalar_static.f64_values[21]=(2.0-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[22]=f64::powf(2.0,self.scalar_static.f64_values[21]);
        self.scalar_static.f64_values[23]=(1.0/self.scalar_static.f64_values[22]);
        self.scalar_static.f64_values[24]=p[114];
        self.scalar_static.f64_values[25]=p[115];
        self.scalar_static.f64_values[26]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[25]);
        self.scalar_static.f64_values[27]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[26]);
        self.scalar_static.f64_values[28]=p[116];
        self.scalar_static.f64_values[29]=(self.scalar_static.f64_values[9]+self.scalar_static.f64_values[28]);
        self.scalar_static.f64_values[30]=(self.scalar_static.f64_values[27]/self.scalar_static.f64_values[29]);
        self.scalar_static.f64_values[31]=(self.scalar_static.f64_values[24]+self.scalar_static.f64_values[30]);
        self.scalar_static.f64_values[32]=(self.scalar_static.f64_values[31]-0.05);
        self.scalar_static.f64_values[33]=(self.scalar_static.f64_values[32]/0.1);
        self.scalar_static.bool_values[6]=(self.scalar_static.f64_values[31]<0.05);
        self.scalar_static.f64_values[34]=(if self.scalar_static.bool_values[6]{1.0}else{0.0});
        self.scalar_static.f64_values[35]=(self.scalar_static.f64_values[33]).exp();
        self.scalar_static.f64_values[36]=(1.0+self.scalar_static.f64_values[35]);
        self.scalar_static.f64_values[37]=(self.scalar_static.f64_values[36]).ln();
        self.scalar_static.f64_values[38]=(0.1*self.scalar_static.f64_values[37]);
        self.scalar_static.f64_values[39]=(0.05+self.scalar_static.f64_values[38]);
        self.scalar_static.f64_values[40]=(if ((self.scalar_static.f64_values[34])!=0.0){self.scalar_static.f64_values[39]}else{0.0});
        self.scalar_static.bool_values[7]=(!((self.scalar_static.f64_values[34])!=0.0));
        self.scalar_static.f64_values[41]=(-self.scalar_static.f64_values[33]);
        self.scalar_static.f64_values[42]=(self.scalar_static.f64_values[41]).exp();
        self.scalar_static.f64_values[43]=(1.0+self.scalar_static.f64_values[42]);
        self.scalar_static.f64_values[44]=(self.scalar_static.f64_values[43]).ln();
        self.scalar_static.f64_values[45]=(0.1*self.scalar_static.f64_values[44]);
        self.scalar_static.f64_values[46]=(self.scalar_static.f64_values[31]+self.scalar_static.f64_values[45]);
        self.scalar_static.f64_values[47]=(if self.scalar_static.bool_values[7]{self.scalar_static.f64_values[46]}else{self.scalar_static.f64_values[40]});
        self.scalar_static.f64_values[48]=(1.0/self.scalar_static.f64_values[24]);
        self.scalar_static.f64_values[49]=p[66];
        self.scalar_static.f64_values[50]=(1.0/self.scalar_static.f64_values[49]);
        self.scalar_static.f64_values[51]=p[71];
        self.scalar_static.f64_values[52]=p[72];
        self.scalar_static.f64_values[53]=(2.0-self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[54]=f64::powf(2.0,self.scalar_static.f64_values[53]);
        self.scalar_static.f64_values[55]=(1.0/self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[56]=p[117];
        self.scalar_static.f64_values[57]=p[118];
        self.scalar_static.f64_values[58]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[57]);
        self.scalar_static.f64_values[59]=(self.scalar_static.f64_values[9]*self.scalar_static.f64_values[58]);
        self.scalar_static.f64_values[60]=p[119];
        self.scalar_static.f64_values[61]=(self.scalar_static.f64_values[9]+self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[62]=(self.scalar_static.f64_values[59]/self.scalar_static.f64_values[61]);
        self.scalar_static.f64_values[63]=(self.scalar_static.f64_values[56]+self.scalar_static.f64_values[62]);
        self.scalar_static.f64_values[64]=(self.scalar_static.f64_values[63]-0.05);
        self.scalar_static.f64_values[65]=(self.scalar_static.f64_values[64]/0.1);
        self.scalar_static.bool_values[8]=(self.scalar_static.f64_values[63]<0.05);
        self.scalar_static.f64_values[66]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.f64_values[67]=(self.scalar_static.f64_values[65]).exp();
        self.scalar_static.f64_values[68]=(1.0+self.scalar_static.f64_values[67]);
        self.scalar_static.f64_values[69]=(self.scalar_static.f64_values[68]).ln();
        self.scalar_static.f64_values[70]=(0.1*self.scalar_static.f64_values[69]);
        self.scalar_static.f64_values[71]=(0.05+self.scalar_static.f64_values[70]);
        self.scalar_static.f64_values[72]=(if ((self.scalar_static.f64_values[66])!=0.0){self.scalar_static.f64_values[71]}else{0.0});
        self.scalar_static.bool_values[9]=(!((self.scalar_static.f64_values[66])!=0.0));
        self.scalar_static.f64_values[73]=(-self.scalar_static.f64_values[65]);
        self.scalar_static.f64_values[74]=(self.scalar_static.f64_values[73]).exp();
        self.scalar_static.f64_values[75]=(1.0+self.scalar_static.f64_values[74]);
        self.scalar_static.f64_values[76]=(self.scalar_static.f64_values[75]).ln();
        self.scalar_static.f64_values[77]=(0.1*self.scalar_static.f64_values[76]);
        self.scalar_static.f64_values[78]=(self.scalar_static.f64_values[63]+self.scalar_static.f64_values[77]);
        self.scalar_static.f64_values[79]=(if self.scalar_static.bool_values[9]{self.scalar_static.f64_values[78]}else{self.scalar_static.f64_values[72]});
        self.scalar_static.f64_values[80]=(1.0/self.scalar_static.f64_values[56]);
        self.scalar_static.f64_values[81]=(1.0/self.scalar_static.f64_values[51]);
        self.scalar_static.f64_values[82]=p[83];
        self.scalar_static.f64_values[83]=(1.0/self.scalar_static.f64_values[82]);
        self.scalar_static.f64_values[84]=(1.0-self.scalar_static.f64_values[83]);
        self.scalar_static.f64_values[85]=(self.scalar_static.f64_values[9]*8.617086918058125e-5);
        self.scalar_static.f64_values[86]=(1.0/self.scalar_static.f64_values[85]);
        self.scalar_static.f64_values[87]=p[105];
        self.scalar_static.f64_values[88]=p[64];
        self.scalar_static.f64_values[89]=p[110];
        self.scalar_static.f64_values[90]=p[80];
        self.scalar_static.f64_values[91]=p[27];
        self.scalar_static.f64_values[92]=p[109];
        self.scalar_static.f64_values[93]=p[138];
        self.scalar_static.f64_values[94]=p[140];
        self.scalar_static.f64_values[95]=p[65];
        self.scalar_static.f64_values[96]=p[137];
        self.scalar_static.f64_values[97]=p[139];
        self.scalar_static.f64_values[98]=p[75];
        self.scalar_static.f64_values[99]=(1.0-self.scalar_static.f64_values[98]);
        self.scalar_static.f64_values[100]=p[70];
        self.scalar_static.f64_values[101]=p[54];
        self.scalar_static.f64_values[102]=p[97];
        self.scalar_static.f64_values[103]=p[56];
        self.scalar_static.f64_values[104]=p[98];
        self.scalar_static.f64_values[105]=p[96];
        self.scalar_static.f64_values[106]=(self.scalar_static.f64_values[104]-self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[107]=p[55];
        self.scalar_static.f64_values[108]=p[101];
        self.scalar_static.f64_values[109]=p[57];
        self.scalar_static.f64_values[110]=p[102];
        self.scalar_static.f64_values[111]=p[58];
        self.scalar_static.f64_values[112]=p[104];
        self.scalar_static.f64_values[113]=p[59];
        self.scalar_static.f64_values[114]=p[60];
        self.scalar_static.f64_values[115]=p[99];
        self.scalar_static.f64_values[116]=p[122];
        self.scalar_static.bool_values[10]=(0.0!=self.scalar_static.f64_values[116]);
        self.scalar_static.f64_values[117]=(if self.scalar_static.bool_values[10]{1.0}else{0.0});
        self.scalar_static.f64_values[118]=p[10];
        self.scalar_static.bool_values[11]=(!((self.scalar_static.f64_values[117])!=0.0));
        self.scalar_static.f64_values[119]=p[123];
        self.scalar_static.bool_values[12]=(0.0!=self.scalar_static.f64_values[119]);
        self.scalar_static.f64_values[120]=(if self.scalar_static.bool_values[12]{1.0}else{0.0});
        self.scalar_static.f64_values[121]=p[11];
        self.scalar_static.bool_values[13]=(!((self.scalar_static.f64_values[120])!=0.0));
        self.scalar_static.f64_values[122]=p[43];
        self.scalar_static.f64_values[123]=p[124];
        self.scalar_static.f64_values[124]=p[9];
        self.scalar_static.f64_values[125]=(4.0-self.scalar_static.f64_values[104]);
        self.scalar_static.f64_values[126]=(self.scalar_static.f64_values[125]-self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[127]=p[121];
        self.scalar_static.f64_values[128]=(self.scalar_static.f64_values[126]+self.scalar_static.f64_values[127]);
        self.scalar_static.f64_values[129]=(-self.scalar_static.f64_values[87]);
        self.scalar_static.f64_values[130]=p[12];
        self.scalar_static.f64_values[131]=(1.0-self.scalar_static.f64_values[104]);
        self.scalar_static.f64_values[132]=p[30];
        self.scalar_static.f64_values[133]=p[103];
        self.scalar_static.f64_values[134]=(1.0-self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[135]=p[20];
        self.scalar_static.f64_values[136]=p[21];
        self.scalar_static.f64_values[137]=(2.0*self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[138]=(6.0-self.scalar_static.f64_values[137]);
        self.scalar_static.f64_values[139]=p[113];
        self.scalar_static.f64_values[140]=(-self.scalar_static.f64_values[139]);
        self.scalar_static.f64_values[141]=p[31];
        self.scalar_static.f64_values[142]=p[32];
        self.scalar_static.f64_values[143]=(2.0*self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[144]=(6.0-self.scalar_static.f64_values[143]);
        self.scalar_static.f64_values[145]=(-self.scalar_static.f64_values[89]);
        self.scalar_static.f64_values[146]=p[16];
        self.scalar_static.f64_values[147]=(4.0-self.scalar_static.f64_values[102]);
        self.scalar_static.f64_values[148]=(self.scalar_static.f64_values[127]+self.scalar_static.f64_values[147]);
        self.scalar_static.f64_values[149]=p[17];
        self.scalar_static.f64_values[150]=p[111];
        self.scalar_static.f64_values[151]=(-self.scalar_static.f64_values[150]);
        self.scalar_static.f64_values[152]=p[18];
        self.scalar_static.f64_values[153]=p[19];
        self.scalar_static.f64_values[154]=p[24];
        self.scalar_static.bool_values[14]=(1.0==self.scalar_static.f64_values[154]);
        self.scalar_static.f64_values[155]=(if self.scalar_static.bool_values[14]{1.0}else{0.0});
        self.scalar_static.f64_values[156]=p[25];
        self.scalar_static.f64_values[157]=p[107];
        self.scalar_static.f64_values[158]=(-self.scalar_static.f64_values[157]);
        self.scalar_static.f64_values[159]=p[28];
        self.scalar_static.f64_values[160]=p[106];
        self.scalar_static.f64_values[161]=(-self.scalar_static.f64_values[160]);
        self.scalar_static.f64_values[162]=p[26];
        self.scalar_static.f64_values[163]=p[108];
        self.scalar_static.f64_values[164]=(-self.scalar_static.f64_values[163]);
        self.scalar_static.f64_values[165]=p[29];
        self.scalar_static.f64_values[166]=(4.0-self.scalar_static.f64_values[133]);
        self.scalar_static.f64_values[167]=(self.scalar_static.f64_values[127]+self.scalar_static.f64_values[166]);
        self.scalar_static.f64_values[168]=p[112];
        self.scalar_static.f64_values[169]=(-self.scalar_static.f64_values[168]);
        self.scalar_static.f64_values[170]=p[22];
        self.scalar_static.f64_values[171]=p[23];
        self.scalar_static.f64_values[172]=(2.0*self.scalar_static.f64_values[171]);
        self.scalar_static.f64_values[173]=(6.0-self.scalar_static.f64_values[172]);
        self.scalar_static.f64_values[174]=p[145];
        self.scalar_static.f64_values[175]=p[146];
        self.scalar_static.f64_values[176]=(4.0/self.scalar_static.f64_values[175]);
        self.scalar_static.f64_values[177]=p[151];
        self.scalar_static.f64_values[178]=p[153];
        self.scalar_static.f64_values[179]=p[35];
        self.scalar_static.f64_values[180]=p[34];
        self.scalar_static.f64_values[181]=p[37];
        self.scalar_static.f64_values[182]=p[36];
        self.scalar_static.f64_values[183]=p[14];
        self.scalar_static.f64_values[184]=p[13];
        self.scalar_static.f64_values[185]=p[133];
        self.scalar_static.f64_values[186]=p[141];
        self.scalar_static.f64_values[187]=(4.0-self.scalar_static.f64_values[186]);
        self.scalar_static.f64_values[188]=(-self.scalar_static.f64_values[94]);
        self.scalar_static.f64_values[189]=p[142];
        self.scalar_static.f64_values[190]=(0.5*self.scalar_static.f64_values[189]);
        self.scalar_static.f64_values[191]=(3.5-self.scalar_static.f64_values[190]);
        self.scalar_static.f64_values[192]=p[135];
        self.scalar_static.f64_values[193]=(1.0-self.scalar_static.f64_values[186]);
        self.scalar_static.f64_values[194]=p[136];
        self.scalar_static.f64_values[195]=(1.0-self.scalar_static.f64_values[189]);
        self.scalar_static.f64_values[196]=p[86];
        self.scalar_static.f64_values[197]=(self.scalar_static.f64_values[104]-2.0);
        self.scalar_static.f64_values[198]=p[120];
        self.scalar_static.f64_values[199]=(-self.scalar_static.f64_values[198]);
        self.scalar_static.f64_values[200]=p[87];
        self.scalar_static.f64_values[201]=(self.scalar_static.f64_values[104]+self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[202]=(self.scalar_static.f64_values[201]-1.0);
        self.scalar_static.f64_values[203]=p[88];
        self.scalar_static.f64_values[204]=(self.scalar_static.f64_values[115]-1.0);
        self.scalar_static.f64_values[205]=p[89];
        self.scalar_static.f64_values[206]=(self.scalar_static.f64_values[200]+self.scalar_static.f64_values[203]);
        self.scalar_static.f64_values[207]=p[90];
        self.scalar_static.f64_values[208]=p[100];
        self.scalar_static.f64_values[209]=(self.scalar_static.f64_values[208]-1.0);
        self.scalar_static.f64_values[210]=(self.scalar_static.f64_values[5]*1.081);
        self.scalar_static.f64_values[211]=p[92];
        self.scalar_static.bool_values[15]=(self.scalar_static.f64_values[109]>0.0);
        self.scalar_static.f64_values[212]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.bool_values[16]=(!((self.scalar_static.f64_values[212])!=0.0));
        self.scalar_static.bool_values[17]=(self.scalar_static.f64_values[111]>0.0);
        self.scalar_static.f64_values[213]=(if self.scalar_static.bool_values[17]{1.0}else{0.0});
        self.scalar_static.bool_values[18]=(!((self.scalar_static.f64_values[213])!=0.0));
        self.scalar_static.bool_values[19]=(self.scalar_static.f64_values[113]>0.0);
        self.scalar_static.f64_values[214]=(if self.scalar_static.bool_values[19]{1.0}else{0.0});
        self.scalar_static.bool_values[20]=(!((self.scalar_static.f64_values[214])!=0.0));
        self.scalar_static.f64_values[215]=p[147];
        self.scalar_static.f64_values[216]=(self.scalar_static.f64_values[215]).exp();
        self.scalar_static.f64_values[217]=p[149];
        self.scalar_static.f64_values[218]=p[62];
        self.scalar_static.f64_values[219]=p[61];
        self.scalar_static.f64_values[220]=(self.scalar_static.f64_values[218]*self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[221]=p[63];
        self.scalar_static.f64_values[222]=(-1.0/self.scalar_static.f64_values[221]);
        self.scalar_static.f64_values[223]=(self.scalar_static.f64_values[222]).exp();
        self.scalar_static.f64_values[224]=(1.0+self.scalar_static.f64_values[223]);
        self.scalar_static.f64_values[225]=(self.scalar_static.f64_values[224]).ln();
        self.scalar_static.f64_values[226]=(self.scalar_static.f64_values[221]*self.scalar_static.f64_values[225]);
        self.scalar_static.f64_values[227]=(1.0+self.scalar_static.f64_values[226]);
        self.scalar_static.f64_values[228]=p[148];
        self.scalar_static.f64_values[229]=(0.5*self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[230]=p[73];
        self.scalar_static.bool_values[21]=(0.0==self.scalar_static.f64_values[230]);
        self.scalar_static.f64_values[231]=(if self.scalar_static.bool_values[21]{1.0}else{0.0});
        self.scalar_static.bool_values[22]=(!((self.scalar_static.f64_values[231])!=0.0));
        self.scalar_static.f64_values[232]=(-1.0/self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[233]=f64::powf(3.0,self.scalar_static.f64_values[232]);
        self.scalar_static.f64_values[234]=(1.0-self.scalar_static.f64_values[233]);
        self.scalar_static.f64_values[235]=(1.0-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[236]=p[74];
        self.scalar_static.bool_values[23]=(1.0==self.scalar_static.f64_values[236]);
        self.scalar_static.f64_values[237]=(if self.scalar_static.bool_values[23]{1.0}else{0.0});
        self.scalar_static.bool_values[24]=(2.0==self.scalar_static.f64_values[236]);
        self.scalar_static.f64_values[238]=(if self.scalar_static.bool_values[24]{1.0}else{0.0});
        self.scalar_static.bool_values[25]=(!((self.scalar_static.f64_values[237])!=0.0));
        self.scalar_static.bool_values[26]=(((self.scalar_static.f64_values[238])!=0.0)&&self.scalar_static.bool_values[25]);
        self.scalar_static.bool_values[27]=(!((self.scalar_static.f64_values[238])!=0.0));
        self.scalar_static.bool_values[28]=(self.scalar_static.bool_values[25]&&self.scalar_static.bool_values[27]);
        self.scalar_static.f64_values[239]=(-1.0/self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[240]=p[76];
        self.scalar_static.f64_values[241]=(1.0-self.scalar_static.f64_values[52]);
        self.scalar_static.bool_values[29]=(0.0==self.scalar_static.f64_values[211]);
        self.scalar_static.f64_values[242]=(if self.scalar_static.bool_values[29]{1.0}else{0.0});
        self.scalar_static.bool_values[30]=(!((self.scalar_static.f64_values[242])!=0.0));
        self.scalar_static.f64_values[243]=p[15];
        self.scalar_static.f64_values[244]=p[152];
        self.scalar_static.f64_values[245]=p[154];
        self.scalar_static.f64_values[246]=p[155];
        self.scalar_static.f64_values[247]=p[93];
        self.scalar_static.bool_values[31]=(0.0==self.scalar_static.f64_values[247]);
        self.scalar_static.f64_values[248]=(if self.scalar_static.bool_values[31]{1.0}else{0.0});
        self.scalar_static.bool_values[32]=(!((self.scalar_static.f64_values[155])!=0.0));
        self.scalar_static.bool_values[33]=(((self.scalar_static.f64_values[248])!=0.0)&&self.scalar_static.bool_values[32]);
        self.scalar_static.bool_values[34]=(!((self.scalar_static.f64_values[248])!=0.0));
        self.scalar_static.bool_values[35]=(self.scalar_static.bool_values[32]&&self.scalar_static.bool_values[34]);
        self.scalar_static.f64_values[249]=(1.0-self.scalar_static.f64_values[247]);
        self.scalar_static.bool_values[36]=(self.scalar_static.f64_values[180]>0.0);
        self.scalar_static.bool_values[37]=(self.scalar_static.f64_values[179]>0.0);
        self.scalar_static.bool_values[38]=(self.scalar_static.bool_values[36]&&self.scalar_static.bool_values[37]);
        self.scalar_static.f64_values[250]=(-2.0-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[251]=(self.scalar_static.f64_values[20]*self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[252]=(1.0-self.scalar_static.f64_values[251]);
        self.scalar_static.f64_values[253]=(self.scalar_static.f64_values[20]-1.0);
        self.scalar_static.bool_values[39]=(self.scalar_static.f64_values[182]>0.0);
        self.scalar_static.bool_values[40]=(self.scalar_static.f64_values[181]>0.0);
        self.scalar_static.bool_values[41]=(self.scalar_static.bool_values[39]&&self.scalar_static.bool_values[40]);
        self.scalar_static.f64_values[254]=(-2.0-self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[255]=(self.scalar_static.f64_values[52]*self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[256]=(1.0-self.scalar_static.f64_values[255]);
        self.scalar_static.f64_values[257]=(self.scalar_static.f64_values[52]-1.0);
        self.scalar_static.f64_values[258]=p[8];
        self.scalar_static.bool_values[42]=(1.0==self.scalar_static.f64_values[258]);
        self.scalar_static.f64_values[259]=(if self.scalar_static.bool_values[42]{1.0}else{0.0});
        self.scalar_static.f64_values[260]=p[143];
        self.scalar_static.f64_values[261]=(2.0*self.scalar_static.f64_values[260]);
        self.scalar_static.f64_values[262]=p[144];
        self.scalar_static.f64_values[263]=(1.0-self.scalar_static.f64_values[260]);
        self.scalar_static.f64_values[264]=(2.0*self.scalar_static.f64_values[263]);
        self.scalar_static.bool_values[43]=(!((self.scalar_static.f64_values[259])!=0.0));
        self.scalar_static.f64_values[265]=(4.0*self.scalar_static.f64_values[262]);
        self.scalar_static.f64_values[266]=p[5];
        self.scalar_static.bool_values[44]=(self.scalar_static.f64_values[266]>0.0);
        self.scalar_static.bool_values[45]=(self.scalar_static.f64_values[6]>0.0);
        self.scalar_static.bool_values[46]=(self.scalar_static.bool_values[44]&&self.scalar_static.bool_values[45]);
        self.scalar_static.f64_values[267]=(if self.scalar_static.bool_values[46]{1.0}else{0.0});
        self.scalar_static.f64_values[268]=(self.scalar_static.f64_values[6]*2.0);
        self.scalar_static.bool_values[47]=(((self.scalar_static.f64_values[259])!=0.0)&&((self.scalar_static.f64_values[267])!=0.0));
        self.scalar_static.f64_values[269]=(self.scalar_static.f64_values[6]*self.scalar_static.f64_values[263]);
        self.scalar_static.f64_values[270]=(2.0*self.scalar_static.f64_values[269]);
        self.scalar_static.bool_values[48]=(self.scalar_static.bool_values[43]&&((self.scalar_static.f64_values[267])!=0.0));
        self.scalar_static.bool_values[49]=(1.0==self.scalar_static.f64_values[266]);
        self.scalar_static.f64_values[271]=(if self.scalar_static.bool_values[49]{1.0}else{0.0});
        self.scalar_static.bool_values[50]=(((self.scalar_static.f64_values[267])!=0.0)&&((self.scalar_static.f64_values[271])!=0.0));
        self.scalar_static.f64_values[272]=(if self.scalar_static.bool_values[50]{0.0121}else{0.010000000000000002});
        self.scalar_static.f64_values[273]=(0.5*self.scalar_static.f64_values[272]);
        self.scalar_static.bool_values[51]=(!((self.scalar_static.f64_values[271])!=0.0));
        self.scalar_static.bool_values[52]=(((self.scalar_static.f64_values[267])!=0.0)&&self.scalar_static.bool_values[51]);
        self.scalar_static.f64_values[274]=p[84];
        self.scalar_static.bool_values[53]=(1.0==self.scalar_static.f64_values[274]);
        self.scalar_static.f64_values[275]=(if self.scalar_static.bool_values[53]{1.0}else{0.0});
        self.scalar_static.f64_values[276]=(if ((self.scalar_static.f64_values[275])!=0.0){1e-12}else{self.scalar_static.f64_values[272]});
        self.scalar_static.f64_values[277]=(0.5*self.scalar_static.f64_values[276]);
        self.scalar_static.f64_values[278]=p[82];
        self.scalar_static.f64_values[279]=f64::powf(self.scalar_static.f64_values[84],self.scalar_static.f64_values[278]);
        self.scalar_static.f64_values[280]=(1.0-self.scalar_static.f64_values[279]);
        self.scalar_static.f64_values[281]=(1.0/self.scalar_static.f64_values[280]);
        self.scalar_static.f64_values[282]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[281]}else{0.0});
        self.scalar_static.f64_values[283]=p[81];
        self.scalar_static.f64_values[284]=(self.scalar_static.f64_values[84]*self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[285]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[284]}else{0.0});
        self.scalar_static.f64_values[286]=(self.scalar_static.f64_values[282]*self.scalar_static.f64_values[282]);
        self.scalar_static.f64_values[287]=(self.scalar_static.f64_values[278]-1.0);
        self.scalar_static.f64_values[288]=f64::powf(self.scalar_static.f64_values[84],self.scalar_static.f64_values[287]);
        self.scalar_static.f64_values[289]=(self.scalar_static.f64_values[286]*self.scalar_static.f64_values[288]);
        self.scalar_static.f64_values[290]=(self.scalar_static.f64_values[278]*self.scalar_static.f64_values[289]);
        self.scalar_static.f64_values[291]=(self.scalar_static.f64_values[290]/self.scalar_static.f64_values[283]);
        self.scalar_static.f64_values[292]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[291]}else{0.0});
        self.scalar_static.bool_values[54]=(!((self.scalar_static.f64_values[275])!=0.0));
        self.scalar_static.f64_values[293]=p[39];
        self.scalar_static.bool_values[55]=(1.0==self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[294]=(if self.scalar_static.bool_values[55]{1.0}else{0.0});
        self.scalar_static.f64_values[295]=p[44];
        self.scalar_static.f64_values[296]=p[42];
        self.scalar_static.f64_values[297]=p[41];
        self.scalar_static.f64_values[298]=p[40];
        self.scalar_static.bool_values[56]=(2.0==self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[299]=(if self.scalar_static.bool_values[56]{1.0}else{0.0});
        self.scalar_static.bool_values[57]=(!((self.scalar_static.f64_values[294])!=0.0));
        self.scalar_static.f64_values[300]=p[46];
        self.scalar_static.f64_values[301]=(2.0*self.scalar_static.f64_values[300]);
        self.scalar_static.f64_values[302]=p[45];
        self.scalar_static.f64_values[303]=(self.scalar_static.f64_values[302]*self.scalar_static.f64_values[302]);
        self.scalar_static.f64_values[304]=(self.scalar_static.f64_values[301]/self.scalar_static.f64_values[303]);
        self.scalar_static.f64_values[305]=p[7];
        self.scalar_static.bool_values[58]=(0.0==self.scalar_static.f64_values[305]);
        self.scalar_static.f64_values[306]=(if self.scalar_static.bool_values[58]{1.0}else{0.0});
        self.scalar_static.bool_values[59]=(!((self.scalar_static.f64_values[306])!=0.0));
        self.scalar_static.f64_values[307]=p[47];
        self.scalar_static.f64_values[308]=(2.0*self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[309]=(1.0+self.scalar_static.f64_values[307]);
        self.scalar_static.f64_values[310]=(1.0+self.scalar_static.f64_values[308]);
        self.scalar_static.f64_values[311]=(self.scalar_static.f64_values[309]/self.scalar_static.f64_values[310]);
        self.scalar_static.bool_values[60]=(3.0==self.scalar_static.f64_values[293]);
        self.scalar_static.f64_values[312]=(if self.scalar_static.bool_values[60]{1.0}else{0.0});
        self.scalar_static.bool_values[61]=(!((self.scalar_static.f64_values[299])!=0.0));
        self.scalar_static.f64_values[313]=p[48];
        self.scalar_static.f64_values[314]=p[49];
        self.scalar_static.f64_values[315]=p[52];
        self.scalar_static.f64_values[316]=p[51];
        self.scalar_static.f64_values[317]=p[50];
        self.scalar_static.f64_values[318]=p[53];
        self.scalar_static.bool_values[62]=(1.0==self.scalar_static.f64_values[318]);
        self.scalar_static.f64_values[319]=(if self.scalar_static.bool_values[62]{1.0}else{0.0});
        self.scalar_static.bool_values[63]=(!((self.scalar_static.f64_values[312])!=0.0));
        self.scalar_static.bool_values[64]=(!((self.scalar_static.f64_values[319])!=0.0));
        self.scalar_static.f64_values[320]=p[68];
        self.scalar_static.f64_values[321]=(1.0-self.scalar_static.f64_values[320]);
        self.scalar_static.f64_values[322]=p[77];
        self.scalar_static.f64_values[323]=(1.0-self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[324]=(-1.0/self.scalar_static.f64_values[97]);
        self.scalar_static.f64_values[325]=f64::powf(2.0,self.scalar_static.f64_values[324]);
        self.scalar_static.f64_values[326]=(1.0-self.scalar_static.f64_values[325]);
        self.scalar_static.f64_values[327]=(1.0-self.scalar_static.f64_values[97]);
        self.scalar_static.f64_values[328]=p[85];
        self.scalar_static.f64_values[329]=(1.0/self.scalar_static.f64_values[328]);
        self.scalar_static.f64_values[330]=p[79];
        self.scalar_static.bool_values[65]=(0.0==self.scalar_static.f64_values[330]);
        self.scalar_static.f64_values[331]=(if self.scalar_static.bool_values[65]{1.0}else{0.0});
        self.scalar_static.f64_values[332]=p[91];
        self.scalar_static.bool_values[66]=(!((self.scalar_static.f64_values[331])!=0.0));
        self.scalar_static.bool_values[67]=(3.0==self.scalar_static.f64_values[266]);
        self.scalar_static.bool_values[68]=(self.scalar_static.bool_values[49]||self.scalar_static.bool_values[67]);
        self.scalar_static.bool_values[69]=(self.scalar_static.bool_values[45]&&self.scalar_static.bool_values[68]);
        self.scalar_static.f64_values[333]=(if self.scalar_static.bool_values[69]{1.0}else{0.0});
        self.scalar_static.bool_values[70]=(((self.scalar_static.f64_values[331])!=0.0)&&((self.scalar_static.f64_values[333])!=0.0));
        self.scalar_static.f64_values[334]=(self.scalar_static.f64_values[6]*0.5);
        self.scalar_static.bool_values[71]=(self.scalar_static.bool_values[66]&&((self.scalar_static.f64_values[333])!=0.0));
        self.scalar_static.f64_values[335]=p[6];
        self.scalar_static.bool_values[72]=(1.0==self.scalar_static.f64_values[335]);
        self.scalar_static.f64_values[336]=(if self.scalar_static.bool_values[72]{1.0}else{0.0});
        self.scalar_static.f64_values[337]=(-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[338]=p[95];
        self.scalar_static.f64_values[339]=(1.0-self.scalar_static.f64_values[338]);
        self.scalar_static.f64_values[340]=p[94];
        self.scalar_static.f64_values[341]=(1.0-self.scalar_static.f64_values[340]);
        self.scalar_static.bool_values[73]=(!((self.scalar_static.f64_values[336])!=0.0));
        self.scalar_static.f64_values[342]=p[130];
        self.scalar_static.bool_values[74]=(self.scalar_static.f64_values[342]>0.0);
        self.scalar_static.f64_values[343]=(if self.scalar_static.bool_values[74]{1.0}else{0.0});
        self.scalar_static.bool_values[75]=(!((self.scalar_static.f64_values[343])!=0.0));
        self.scalar_static.f64_values[344]=p[131];
        self.scalar_static.bool_values[76]=(1.0==self.scalar_static.f64_values[344]);
        self.scalar_static.f64_values[345]=(if self.scalar_static.bool_values[76]{1.0}else{0.0});
        self.scalar_static.bool_values[77]=(2.0==self.scalar_static.f64_values[344]);
        self.scalar_static.f64_values[346]=(if self.scalar_static.bool_values[77]{1.0}else{0.0});
        self.scalar_static.bool_values[78]=(!((self.scalar_static.f64_values[345])!=0.0));
        self.scalar_static.bool_values[79]=(((self.scalar_static.f64_values[346])!=0.0)&&self.scalar_static.bool_values[78]);
        self.scalar_static.f64_values[347]=p[132];
        self.scalar_static.bool_values[80]=(!((self.scalar_static.f64_values[346])!=0.0));
        self.scalar_static.bool_values[81]=(self.scalar_static.bool_values[78]&&self.scalar_static.bool_values[80]);
        self.scalar_static.f64_values[348]=p[69];
        self.scalar_static.f64_values[349]=p[78];
        self.scalar_static.f64_values[350]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[348]);
        self.scalar_static.f64_values[351]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[349]);
        self.scalar_static.f64_values[352]=(-self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[353]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[354]=(self.scalar_static.f64_values[352]-self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[355]=(self.scalar_static.f64_values[0]+self.scalar_static.f64_values[353]);
        self.scalar_static.f64_values[356]=(self.scalar_static.f64_values[235]-1.0);
        self.scalar_static.f64_values[357]=(if ((self.scalar_static.f64_values[237])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[358]=(if ((self.scalar_static.f64_values[237])!=0.0){self.scalar_static.f64_values[352]}else{0.0});
        self.scalar_static.f64_values[359]=(self.scalar_static.f64_values[240]-1.0);
        self.scalar_static.f64_values[360]=(self.scalar_static.f64_values[241]-1.0);
        self.scalar_static.f64_values[361]=(self.scalar_static.f64_values[352]/0.0001);
        self.scalar_static.f64_values[362]=(self.scalar_static.f64_values[0]/0.0001);
        self.scalar_static.f64_values[363]=(-self.scalar_static.f64_values[361]);
        self.scalar_static.f64_values[364]=(-self.scalar_static.f64_values[362]);
        self.scalar_static.f64_values[365]=(self.scalar_static.f64_values[352]/0.001);
        self.scalar_static.f64_values[366]=(self.scalar_static.f64_values[0]/0.001);
        self.scalar_static.f64_values[367]=(-self.scalar_static.f64_values[365]);
        self.scalar_static.f64_values[368]=(-self.scalar_static.f64_values[366]);
        self.scalar_static.f64_values[369]=(self.scalar_static.f64_values[250]-1.0);
        self.scalar_static.f64_values[370]=(self.scalar_static.f64_values[22]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[371]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[22]);
        self.scalar_static.f64_values[372]=(0.5*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[373]=(self.scalar_static.f64_values[0]*0.5);
        self.scalar_static.f64_values[374]=(self.scalar_static.f64_values[254]-1.0);
        self.scalar_static.f64_values[375]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[54]);
        self.scalar_static.f64_values[376]=(self.scalar_static.f64_values[54]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[377]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[353]}else{0.0});
        self.scalar_static.f64_values[378]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[355]}else{0.0});
        self.scalar_static.f64_values[379]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[354]}else{0.0});
        self.scalar_static.f64_values[380]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[352]}else{0.0});
        self.scalar_static.f64_values[381]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[0]}else{0.0});
        self.scalar_static.f64_values[382]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[353]}else{0.0});
        self.scalar_static.f64_values[383]=(if ((self.scalar_static.f64_values[275])!=0.0){self.scalar_static.f64_values[352]}else{0.0});
        self.scalar_static.f64_values[384]=(-self.scalar_static.f64_values[381]);
        self.scalar_static.f64_values[385]=(-self.scalar_static.f64_values[382]);
        self.scalar_static.f64_values[386]=(-self.scalar_static.f64_values[383]);
        self.scalar_static.f64_values[387]=(self.scalar_static.f64_values[297]-1.0);
        self.scalar_static.f64_values[388]=(self.scalar_static.f64_values[314]-1.0);
        self.scalar_static.f64_values[389]=(self.scalar_static.f64_values[317]-1.0);
        self.scalar_static.f64_values[390]=(self.scalar_static.f64_values[327]-1.0);
        self.scalar_static.f64_values[391]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[392]=(self.scalar_static.f64_values[353]/self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[393]=(self.scalar_static.f64_values[354]/self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[394]=(self.scalar_static.f64_values[352]/self.scalar_static.f64_values[332]);
        self.scalar_static.f64_values[395]=(self.scalar_static.f64_values[337]-1.0);
        self.scalar_static.f64_values[396]=(self.scalar_static.f64_values[0]*0.2);
        self.scalar_static.f64_values[397]=(0.2*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[398]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[0]);
        self.scalar_static.f64_values[399]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[400]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[350]);
        self.scalar_static.f64_values[401]=(self.scalar_static.f64_values[350]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[402]=(self.scalar_static.f64_values[351]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[403]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[351]);
        self.scalar_static.f64_values[404]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[353]);
        self.scalar_static.f64_values[405]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[354]);
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
        self.scalar_static.f64_values[406]=(temperature+self.scalar_static.f64_values[10]);
        self.scalar_static.f64_values[407]=(self.scalar_static.f64_values[406]/self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[408]=(self.scalar_static.f64_values[406]*8.617086918058125e-5);
        self.scalar_static.f64_values[409]=(1.0/self.scalar_static.f64_values[408]);
        self.scalar_static.f64_values[410]=(self.scalar_static.f64_values[409]-self.scalar_static.f64_values[86]);
        self.scalar_static.f64_values[411]=(self.scalar_static.f64_values[406]-self.scalar_static.f64_values[9]);
        self.scalar_static.f64_values[412]=(self.scalar_static.f64_values[407]).ln();
        self.scalar_static.f64_values[413]=(self.scalar_static.f64_values[406]*self.scalar_static.f64_values[25]);
        self.scalar_static.f64_values[414]=(self.scalar_static.f64_values[406]*self.scalar_static.f64_values[413]);
        self.scalar_static.f64_values[415]=(self.scalar_static.f64_values[406]+self.scalar_static.f64_values[28]);
        self.scalar_static.f64_values[416]=(self.scalar_static.f64_values[414]/self.scalar_static.f64_values[415]);
        self.scalar_static.f64_values[417]=(self.scalar_static.f64_values[47]-self.scalar_static.f64_values[416]);
        self.scalar_static.f64_values[418]=(self.scalar_static.f64_values[417]-0.05);
        self.scalar_static.f64_values[419]=(self.scalar_static.f64_values[418]/0.1);
        self.scalar_static.bool_values[82]=(self.scalar_static.f64_values[417]<0.05);
        self.scalar_static.f64_values[420]=(if self.scalar_static.bool_values[82]{1.0}else{0.0});
        self.scalar_static.f64_values[421]=(self.scalar_static.f64_values[419]).exp();
        self.scalar_static.f64_values[422]=(1.0+self.scalar_static.f64_values[421]);
        self.scalar_static.f64_values[423]=(self.scalar_static.f64_values[422]).ln();
        self.scalar_static.f64_values[424]=(0.1*self.scalar_static.f64_values[423]);
        self.scalar_static.f64_values[425]=(0.05+self.scalar_static.f64_values[424]);
        self.scalar_static.f64_values[426]=(if ((self.scalar_static.f64_values[420])!=0.0){self.scalar_static.f64_values[425]}else{0.0});
        self.scalar_static.bool_values[83]=(!((self.scalar_static.f64_values[420])!=0.0));
        self.scalar_static.f64_values[427]=(-self.scalar_static.f64_values[419]);
        self.scalar_static.f64_values[428]=(self.scalar_static.f64_values[427]).exp();
        self.scalar_static.f64_values[429]=(1.0+self.scalar_static.f64_values[428]);
        self.scalar_static.f64_values[430]=(self.scalar_static.f64_values[429]).ln();
        self.scalar_static.f64_values[431]=(0.1*self.scalar_static.f64_values[430]);
        self.scalar_static.f64_values[432]=(self.scalar_static.f64_values[417]+self.scalar_static.f64_values[431]);
        self.scalar_static.f64_values[433]=(if self.scalar_static.bool_values[83]{self.scalar_static.f64_values[432]}else{self.scalar_static.f64_values[426]});
        self.scalar_static.f64_values[434]=(self.scalar_static.f64_values[406]*self.scalar_static.f64_values[57]);
        self.scalar_static.f64_values[435]=(self.scalar_static.f64_values[406]*self.scalar_static.f64_values[434]);
        self.scalar_static.f64_values[436]=(self.scalar_static.f64_values[406]+self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[437]=(self.scalar_static.f64_values[435]/self.scalar_static.f64_values[436]);
        self.scalar_static.f64_values[438]=(self.scalar_static.f64_values[79]-self.scalar_static.f64_values[437]);
        self.scalar_static.f64_values[439]=(self.scalar_static.f64_values[438]-0.05);
        self.scalar_static.f64_values[440]=(self.scalar_static.f64_values[439]/0.1);
        self.scalar_static.bool_values[84]=(self.scalar_static.f64_values[438]<0.05);
        self.scalar_static.f64_values[441]=(if self.scalar_static.bool_values[84]{1.0}else{0.0});
        self.scalar_static.f64_values[442]=(self.scalar_static.f64_values[440]).exp();
        self.scalar_static.f64_values[443]=(1.0+self.scalar_static.f64_values[442]);
        self.scalar_static.f64_values[444]=(self.scalar_static.f64_values[443]).ln();
        self.scalar_static.f64_values[445]=(0.1*self.scalar_static.f64_values[444]);
        self.scalar_static.f64_values[446]=(0.05+self.scalar_static.f64_values[445]);
        self.scalar_static.f64_values[447]=(if ((self.scalar_static.f64_values[441])!=0.0){self.scalar_static.f64_values[446]}else{0.0});
        self.scalar_static.bool_values[85]=(!((self.scalar_static.f64_values[441])!=0.0));
        self.scalar_static.f64_values[448]=(-self.scalar_static.f64_values[440]);
        self.scalar_static.f64_values[449]=(self.scalar_static.f64_values[448]).exp();
        self.scalar_static.f64_values[450]=(1.0+self.scalar_static.f64_values[449]);
        self.scalar_static.f64_values[451]=(self.scalar_static.f64_values[450]).ln();
        self.scalar_static.f64_values[452]=(0.1*self.scalar_static.f64_values[451]);
        self.scalar_static.f64_values[453]=(self.scalar_static.f64_values[438]+self.scalar_static.f64_values[452]);
        self.scalar_static.f64_values[454]=(if self.scalar_static.bool_values[85]{self.scalar_static.f64_values[453]}else{self.scalar_static.f64_values[447]});
        self.scalar_static.f64_values[455]=(self.scalar_static.f64_values[408]* -3.0);
        self.scalar_static.f64_values[456]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[455]);
        self.scalar_static.f64_values[457]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[407]);
        self.scalar_static.f64_values[458]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[457]);
        self.scalar_static.f64_values[459]=(1.0-self.scalar_static.f64_values[407]);
        self.scalar_static.f64_values[460]=(self.scalar_static.f64_values[459]*self.scalar_static.f64_values[87]);
        self.scalar_static.f64_values[461]=(self.scalar_static.f64_values[458]+self.scalar_static.f64_values[460]);
        self.scalar_static.f64_values[462]=(0.05-self.scalar_static.f64_values[461]);
        self.scalar_static.f64_values[463]=(self.scalar_static.f64_values[462]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[86]=(0.05<self.scalar_static.f64_values[461]);
        self.scalar_static.f64_values[464]=(if self.scalar_static.bool_values[86]{1.0}else{0.0});
        self.scalar_static.f64_values[465]=(self.scalar_static.f64_values[463]).exp();
        self.scalar_static.f64_values[466]=(1.0+self.scalar_static.f64_values[465]);
        self.scalar_static.f64_values[467]=(self.scalar_static.f64_values[466]).ln();
        self.scalar_static.f64_values[468]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[467]);
        self.scalar_static.f64_values[469]=(self.scalar_static.f64_values[461]+self.scalar_static.f64_values[468]);
        self.scalar_static.f64_values[470]=(if ((self.scalar_static.f64_values[464])!=0.0){self.scalar_static.f64_values[469]}else{0.0});
        self.scalar_static.bool_values[87]=(!((self.scalar_static.f64_values[464])!=0.0));
        self.scalar_static.f64_values[471]=(-self.scalar_static.f64_values[463]);
        self.scalar_static.f64_values[472]=(self.scalar_static.f64_values[471]).exp();
        self.scalar_static.f64_values[473]=(1.0+self.scalar_static.f64_values[472]);
        self.scalar_static.f64_values[474]=(self.scalar_static.f64_values[473]).ln();
        self.scalar_static.f64_values[475]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[474]);
        self.scalar_static.f64_values[476]=(0.05+self.scalar_static.f64_values[475]);
        self.scalar_static.f64_values[477]=(if self.scalar_static.bool_values[87]{self.scalar_static.f64_values[476]}else{self.scalar_static.f64_values[470]});
        self.scalar_static.f64_values[478]=(self.scalar_static.f64_values[407]*self.scalar_static.f64_values[88]);
        self.scalar_static.f64_values[479]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[478]);
        self.scalar_static.f64_values[480]=(self.scalar_static.f64_values[459]*self.scalar_static.f64_values[89]);
        self.scalar_static.f64_values[481]=(self.scalar_static.f64_values[479]+self.scalar_static.f64_values[480]);
        self.scalar_static.f64_values[482]=(0.05-self.scalar_static.f64_values[481]);
        self.scalar_static.f64_values[483]=(self.scalar_static.f64_values[482]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[88]=(0.05<self.scalar_static.f64_values[481]);
        self.scalar_static.f64_values[484]=(if self.scalar_static.bool_values[88]{1.0}else{0.0});
        self.scalar_static.f64_values[485]=(self.scalar_static.f64_values[483]).exp();
        self.scalar_static.f64_values[486]=(1.0+self.scalar_static.f64_values[485]);
        self.scalar_static.f64_values[487]=(self.scalar_static.f64_values[486]).ln();
        self.scalar_static.f64_values[488]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[487]);
        self.scalar_static.f64_values[489]=(self.scalar_static.f64_values[481]+self.scalar_static.f64_values[488]);
        self.scalar_static.f64_values[490]=(if ((self.scalar_static.f64_values[484])!=0.0){self.scalar_static.f64_values[489]}else{0.0});
        self.scalar_static.bool_values[89]=(!((self.scalar_static.f64_values[484])!=0.0));
        self.scalar_static.f64_values[491]=(-self.scalar_static.f64_values[483]);
        self.scalar_static.f64_values[492]=(self.scalar_static.f64_values[491]).exp();
        self.scalar_static.f64_values[493]=(1.0+self.scalar_static.f64_values[492]);
        self.scalar_static.f64_values[494]=(self.scalar_static.f64_values[493]).ln();
        self.scalar_static.f64_values[495]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[494]);
        self.scalar_static.f64_values[496]=(0.05+self.scalar_static.f64_values[495]);
        self.scalar_static.f64_values[497]=(if self.scalar_static.bool_values[89]{self.scalar_static.f64_values[496]}else{self.scalar_static.f64_values[490]});
        self.scalar_static.f64_values[498]=(self.scalar_static.f64_values[407]*self.scalar_static.f64_values[90]);
        self.scalar_static.f64_values[499]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[498]);
        self.scalar_static.f64_values[500]=(self.scalar_static.f64_values[480]+self.scalar_static.f64_values[499]);
        self.scalar_static.f64_values[501]=(0.05-self.scalar_static.f64_values[500]);
        self.scalar_static.f64_values[502]=(self.scalar_static.f64_values[501]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[90]=(0.05<self.scalar_static.f64_values[500]);
        self.scalar_static.f64_values[503]=(if self.scalar_static.bool_values[90]{1.0}else{0.0});
        self.scalar_static.f64_values[504]=(self.scalar_static.f64_values[502]).exp();
        self.scalar_static.f64_values[505]=(1.0+self.scalar_static.f64_values[504]);
        self.scalar_static.f64_values[506]=(self.scalar_static.f64_values[505]).ln();
        self.scalar_static.f64_values[507]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[506]);
        self.scalar_static.f64_values[508]=(self.scalar_static.f64_values[500]+self.scalar_static.f64_values[507]);
        self.scalar_static.f64_values[509]=(if ((self.scalar_static.f64_values[503])!=0.0){self.scalar_static.f64_values[508]}else{0.0});
        self.scalar_static.bool_values[91]=(!((self.scalar_static.f64_values[503])!=0.0));
        self.scalar_static.f64_values[510]=(-self.scalar_static.f64_values[502]);
        self.scalar_static.f64_values[511]=(self.scalar_static.f64_values[510]).exp();
        self.scalar_static.f64_values[512]=(1.0+self.scalar_static.f64_values[511]);
        self.scalar_static.f64_values[513]=(self.scalar_static.f64_values[512]).ln();
        self.scalar_static.f64_values[514]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[513]);
        self.scalar_static.f64_values[515]=(0.05+self.scalar_static.f64_values[514]);
        self.scalar_static.f64_values[516]=(if self.scalar_static.bool_values[91]{self.scalar_static.f64_values[515]}else{self.scalar_static.f64_values[509]});
        self.scalar_static.f64_values[517]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[407]);
        self.scalar_static.f64_values[518]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[517]);
        self.scalar_static.f64_values[519]=(self.scalar_static.f64_values[480]+self.scalar_static.f64_values[518]);
        self.scalar_static.f64_values[520]=(0.05-self.scalar_static.f64_values[519]);
        self.scalar_static.f64_values[521]=(self.scalar_static.f64_values[520]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[92]=(0.05<self.scalar_static.f64_values[519]);
        self.scalar_static.f64_values[522]=(if self.scalar_static.bool_values[92]{1.0}else{0.0});
        self.scalar_static.f64_values[523]=(self.scalar_static.f64_values[521]).exp();
        self.scalar_static.f64_values[524]=(1.0+self.scalar_static.f64_values[523]);
        self.scalar_static.f64_values[525]=(self.scalar_static.f64_values[524]).ln();
        self.scalar_static.f64_values[526]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[525]);
        self.scalar_static.f64_values[527]=(self.scalar_static.f64_values[519]+self.scalar_static.f64_values[526]);
        self.scalar_static.f64_values[528]=(if ((self.scalar_static.f64_values[522])!=0.0){self.scalar_static.f64_values[527]}else{0.0});
        self.scalar_static.bool_values[93]=(!((self.scalar_static.f64_values[522])!=0.0));
        self.scalar_static.f64_values[529]=(-self.scalar_static.f64_values[521]);
        self.scalar_static.f64_values[530]=(self.scalar_static.f64_values[529]).exp();
        self.scalar_static.f64_values[531]=(1.0+self.scalar_static.f64_values[530]);
        self.scalar_static.f64_values[532]=(self.scalar_static.f64_values[531]).ln();
        self.scalar_static.f64_values[533]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[532]);
        self.scalar_static.f64_values[534]=(0.05+self.scalar_static.f64_values[533]);
        self.scalar_static.f64_values[535]=(if self.scalar_static.bool_values[93]{self.scalar_static.f64_values[534]}else{self.scalar_static.f64_values[528]});
        self.scalar_static.f64_values[536]=(self.scalar_static.f64_values[407]*self.scalar_static.f64_values[91]);
        self.scalar_static.f64_values[537]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[536]);
        self.scalar_static.f64_values[538]=(self.scalar_static.f64_values[459]*self.scalar_static.f64_values[92]);
        self.scalar_static.f64_values[539]=(self.scalar_static.f64_values[537]+self.scalar_static.f64_values[538]);
        self.scalar_static.f64_values[540]=(0.05-self.scalar_static.f64_values[539]);
        self.scalar_static.f64_values[541]=(self.scalar_static.f64_values[540]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[94]=(0.05<self.scalar_static.f64_values[539]);
        self.scalar_static.f64_values[542]=(if self.scalar_static.bool_values[94]{1.0}else{0.0});
        self.scalar_static.f64_values[543]=(self.scalar_static.f64_values[541]).exp();
        self.scalar_static.f64_values[544]=(1.0+self.scalar_static.f64_values[543]);
        self.scalar_static.f64_values[545]=(self.scalar_static.f64_values[544]).ln();
        self.scalar_static.f64_values[546]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[545]);
        self.scalar_static.f64_values[547]=(self.scalar_static.f64_values[539]+self.scalar_static.f64_values[546]);
        self.scalar_static.f64_values[548]=(if ((self.scalar_static.f64_values[542])!=0.0){self.scalar_static.f64_values[547]}else{0.0});
        self.scalar_static.bool_values[95]=(!((self.scalar_static.f64_values[542])!=0.0));
        self.scalar_static.f64_values[549]=(-self.scalar_static.f64_values[541]);
        self.scalar_static.f64_values[550]=(self.scalar_static.f64_values[549]).exp();
        self.scalar_static.f64_values[551]=(1.0+self.scalar_static.f64_values[550]);
        self.scalar_static.f64_values[552]=(self.scalar_static.f64_values[551]).ln();
        self.scalar_static.f64_values[553]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[552]);
        self.scalar_static.f64_values[554]=(0.05+self.scalar_static.f64_values[553]);
        self.scalar_static.f64_values[555]=(if self.scalar_static.bool_values[95]{self.scalar_static.f64_values[554]}else{self.scalar_static.f64_values[548]});
        self.scalar_static.f64_values[556]=(self.scalar_static.f64_values[407]*self.scalar_static.f64_values[93]);
        self.scalar_static.f64_values[557]=(self.scalar_static.f64_values[456]+self.scalar_static.f64_values[556]);
        self.scalar_static.f64_values[558]=(self.scalar_static.f64_values[459]*self.scalar_static.f64_values[94]);
        self.scalar_static.f64_values[559]=(self.scalar_static.f64_values[557]+self.scalar_static.f64_values[558]);
        self.scalar_static.f64_values[560]=(0.05-self.scalar_static.f64_values[559]);
        self.scalar_static.f64_values[561]=(self.scalar_static.f64_values[560]/self.scalar_static.f64_values[408]);
        self.scalar_static.bool_values[96]=(0.05<self.scalar_static.f64_values[559]);
        self.scalar_static.f64_values[562]=(if self.scalar_static.bool_values[96]{1.0}else{0.0});
        self.scalar_static.f64_values[563]=(self.scalar_static.f64_values[561]).exp();
        self.scalar_static.f64_values[564]=(1.0+self.scalar_static.f64_values[563]);
        self.scalar_static.f64_values[565]=(self.scalar_static.f64_values[564]).ln();
        self.scalar_static.f64_values[566]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[565]);
        self.scalar_static.f64_values[567]=(self.scalar_static.f64_values[559]+self.scalar_static.f64_values[566]);
        self.scalar_static.f64_values[568]=(if ((self.scalar_static.f64_values[562])!=0.0){self.scalar_static.f64_values[567]}else{0.0});
        self.scalar_static.bool_values[97]=(!((self.scalar_static.f64_values[562])!=0.0));
        self.scalar_static.f64_values[569]=(-self.scalar_static.f64_values[561]);
        self.scalar_static.f64_values[570]=(self.scalar_static.f64_values[569]).exp();
        self.scalar_static.f64_values[571]=(1.0+self.scalar_static.f64_values[570]);
        self.scalar_static.f64_values[572]=(self.scalar_static.f64_values[571]).ln();
        self.scalar_static.f64_values[573]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[572]);
        self.scalar_static.f64_values[574]=(0.05+self.scalar_static.f64_values[573]);
        self.scalar_static.f64_values[575]=(if self.scalar_static.bool_values[97]{self.scalar_static.f64_values[574]}else{self.scalar_static.f64_values[568]});
        self.scalar_static.f64_values[576]=(1.0/self.scalar_static.f64_values[477]);
        self.scalar_static.f64_values[577]=(1.0/self.scalar_static.f64_values[535]);
        self.scalar_static.f64_values[578]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[576]);
        self.scalar_static.f64_values[579]=f64::powf(self.scalar_static.f64_values[578],self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[580]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[577]);
        self.scalar_static.f64_values[581]=f64::powf(self.scalar_static.f64_values[580],self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[582]=(self.scalar_static.f64_values[579]*self.scalar_static.f64_values[95]);
        self.scalar_static.f64_values[583]=(self.scalar_static.f64_values[93]/self.scalar_static.f64_values[575]);
        self.scalar_static.f64_values[584]=f64::powf(self.scalar_static.f64_values[583],self.scalar_static.f64_values[97]);
        self.scalar_static.f64_values[585]=(self.scalar_static.f64_values[96]*self.scalar_static.f64_values[584]);
        self.scalar_static.f64_values[586]=(self.scalar_static.f64_values[51]/self.scalar_static.f64_values[535]);
        self.scalar_static.f64_values[587]=f64::powf(self.scalar_static.f64_values[586],self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[588]=(self.scalar_static.f64_values[99]*self.scalar_static.f64_values[587]);
        self.scalar_static.f64_values[589]=(self.scalar_static.f64_values[98]+self.scalar_static.f64_values[588]);
        self.scalar_static.f64_values[590]=(1.0/self.scalar_static.f64_values[589]);
        self.scalar_static.f64_values[591]=(self.scalar_static.f64_values[589]*self.scalar_static.f64_values[100]);
        self.scalar_static.f64_values[592]=(self.scalar_static.f64_values[98]*self.scalar_static.f64_values[590]);
        self.scalar_static.f64_values[593]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[102]);
        self.scalar_static.f64_values[594]=(self.scalar_static.f64_values[593]).exp();
        self.scalar_static.f64_values[595]=(self.scalar_static.f64_values[101]*self.scalar_static.f64_values[594]);
        self.scalar_static.bool_values[98]=(self.scalar_static.f64_values[595]<self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[596]=(if self.scalar_static.bool_values[98]{1.0}else{0.0});
        self.scalar_static.f64_values[597]=(if ((self.scalar_static.f64_values[596])!=0.0){self.scalar_static.f64_values[16]}else{self.scalar_static.f64_values[595]});
        self.scalar_static.f64_values[598]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[106]);
        self.scalar_static.f64_values[599]=(self.scalar_static.f64_values[598]).exp();
        self.scalar_static.f64_values[600]=(self.scalar_static.f64_values[103]*self.scalar_static.f64_values[599]);
        self.scalar_static.f64_values[601]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[108]);
        self.scalar_static.f64_values[602]=(self.scalar_static.f64_values[601]).exp();
        self.scalar_static.f64_values[603]=(self.scalar_static.f64_values[107]*self.scalar_static.f64_values[602]);
        self.scalar_static.bool_values[99]=(self.scalar_static.f64_values[603]<self.scalar_static.f64_values[16]);
        self.scalar_static.f64_values[604]=(if self.scalar_static.bool_values[99]{1.0}else{0.0});
        self.scalar_static.f64_values[605]=(if ((self.scalar_static.f64_values[604])!=0.0){self.scalar_static.f64_values[16]}else{self.scalar_static.f64_values[603]});
        self.scalar_static.f64_values[606]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[110]);
        self.scalar_static.f64_values[607]=(self.scalar_static.f64_values[606]).exp();
        self.scalar_static.f64_values[608]=(self.scalar_static.f64_values[109]*self.scalar_static.f64_values[607]);
        self.scalar_static.f64_values[609]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[112]);
        self.scalar_static.f64_values[610]=(self.scalar_static.f64_values[609]).exp();
        self.scalar_static.f64_values[611]=(self.scalar_static.f64_values[111]*self.scalar_static.f64_values[610]);
        self.scalar_static.f64_values[612]=(self.scalar_static.f64_values[610]*self.scalar_static.f64_values[113]);
        self.scalar_static.f64_values[613]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[115]);
        self.scalar_static.f64_values[614]=(self.scalar_static.f64_values[613]).exp();
        self.scalar_static.f64_values[615]=(self.scalar_static.f64_values[114]*self.scalar_static.f64_values[614]);
        self.scalar_static.f64_values[616]=(self.scalar_static.f64_values[411]*self.scalar_static.f64_values[116]);
        self.scalar_static.f64_values[617]=(1.0+self.scalar_static.f64_values[616]);
        self.scalar_static.f64_values[618]=(self.scalar_static.f64_values[118]*self.scalar_static.f64_values[617]);
        self.scalar_static.f64_values[619]=(if ((self.scalar_static.f64_values[117])!=0.0){self.scalar_static.f64_values[618]}else{0.0});
        self.scalar_static.f64_values[620]=(self.scalar_static.f64_values[619]-1.0);
        self.scalar_static.f64_values[621]=(self.scalar_static.f64_values[620]/0.001);
        self.scalar_static.f64_values[622]=(if ((self.scalar_static.f64_values[117])!=0.0){self.scalar_static.f64_values[621]}else{self.scalar_static.f64_values[561]});
        self.scalar_static.bool_values[100]=(self.scalar_static.f64_values[619]<1.0);
        self.scalar_static.f64_values[623]=(if self.scalar_static.bool_values[100]{1.0}else{0.0});
        self.scalar_static.bool_values[101]=(((self.scalar_static.f64_values[117])!=0.0)&&((self.scalar_static.f64_values[623])!=0.0));
        self.scalar_static.f64_values[624]=(self.scalar_static.f64_values[622]).exp();
        self.scalar_static.f64_values[625]=(1.0+self.scalar_static.f64_values[624]);
        self.scalar_static.f64_values[626]=(self.scalar_static.f64_values[625]).ln();
        self.scalar_static.f64_values[627]=(0.001*self.scalar_static.f64_values[626]);
        self.scalar_static.f64_values[628]=(1.0+self.scalar_static.f64_values[627]);
        self.scalar_static.f64_values[629]=(if self.scalar_static.bool_values[101]{self.scalar_static.f64_values[628]}else{self.scalar_static.f64_values[619]});
        self.scalar_static.bool_values[102]=(!((self.scalar_static.f64_values[623])!=0.0));
        self.scalar_static.bool_values[103]=(((self.scalar_static.f64_values[117])!=0.0)&&self.scalar_static.bool_values[102]);
        self.scalar_static.f64_values[630]=(-self.scalar_static.f64_values[622]);
        self.scalar_static.f64_values[631]=(self.scalar_static.f64_values[630]).exp();
        self.scalar_static.f64_values[632]=(1.0+self.scalar_static.f64_values[631]);
        self.scalar_static.f64_values[633]=(self.scalar_static.f64_values[632]).ln();
        self.scalar_static.f64_values[634]=(0.001*self.scalar_static.f64_values[633]);
        self.scalar_static.f64_values[635]=(self.scalar_static.f64_values[629]+self.scalar_static.f64_values[634]);
        self.scalar_static.f64_values[636]=(if self.scalar_static.bool_values[103]{self.scalar_static.f64_values[635]}else{self.scalar_static.f64_values[629]});
        self.scalar_static.f64_values[637]=(self.scalar_static.f64_values[636]-0.0006931471805599453);
        self.scalar_static.f64_values[638]=(if ((self.scalar_static.f64_values[117])!=0.0){self.scalar_static.f64_values[637]}else{0.0});
        self.scalar_static.f64_values[639]=(if self.scalar_static.bool_values[11]{self.scalar_static.f64_values[118]}else{self.scalar_static.f64_values[638]});
        self.scalar_static.f64_values[640]=(self.scalar_static.f64_values[411]*self.scalar_static.f64_values[119]);
        self.scalar_static.f64_values[641]=(1.0+self.scalar_static.f64_values[640]);
        self.scalar_static.f64_values[642]=(self.scalar_static.f64_values[121]*self.scalar_static.f64_values[641]);
        self.scalar_static.f64_values[643]=(if ((self.scalar_static.f64_values[120])!=0.0){self.scalar_static.f64_values[642]}else{0.0});
        self.scalar_static.f64_values[644]=(self.scalar_static.f64_values[643]-1.0);
        self.scalar_static.f64_values[645]=(self.scalar_static.f64_values[644]/0.001);
        self.scalar_static.f64_values[646]=(if ((self.scalar_static.f64_values[120])!=0.0){self.scalar_static.f64_values[645]}else{self.scalar_static.f64_values[622]});
        self.scalar_static.bool_values[104]=(self.scalar_static.f64_values[643]<1.0);
        self.scalar_static.f64_values[647]=(if self.scalar_static.bool_values[104]{1.0}else{0.0});
        self.scalar_static.bool_values[105]=(((self.scalar_static.f64_values[120])!=0.0)&&((self.scalar_static.f64_values[647])!=0.0));
        self.scalar_static.f64_values[648]=(self.scalar_static.f64_values[646]).exp();
        self.scalar_static.f64_values[649]=(1.0+self.scalar_static.f64_values[648]);
        self.scalar_static.f64_values[650]=(self.scalar_static.f64_values[649]).ln();
        self.scalar_static.f64_values[651]=(0.001*self.scalar_static.f64_values[650]);
        self.scalar_static.f64_values[652]=(1.0+self.scalar_static.f64_values[651]);
        self.scalar_static.f64_values[653]=(if self.scalar_static.bool_values[105]{self.scalar_static.f64_values[652]}else{self.scalar_static.f64_values[643]});
        self.scalar_static.bool_values[106]=(!((self.scalar_static.f64_values[647])!=0.0));
        self.scalar_static.bool_values[107]=(((self.scalar_static.f64_values[120])!=0.0)&&self.scalar_static.bool_values[106]);
        self.scalar_static.f64_values[654]=(-self.scalar_static.f64_values[646]);
        self.scalar_static.f64_values[655]=(self.scalar_static.f64_values[654]).exp();
        self.scalar_static.f64_values[656]=(1.0+self.scalar_static.f64_values[655]);
        self.scalar_static.f64_values[657]=(self.scalar_static.f64_values[656]).ln();
        self.scalar_static.f64_values[658]=(0.001*self.scalar_static.f64_values[657]);
        self.scalar_static.f64_values[659]=(self.scalar_static.f64_values[653]+self.scalar_static.f64_values[658]);
        self.scalar_static.f64_values[660]=(if self.scalar_static.bool_values[107]{self.scalar_static.f64_values[659]}else{self.scalar_static.f64_values[653]});
        self.scalar_static.f64_values[661]=(self.scalar_static.f64_values[660]-0.0006931471805599453);
        self.scalar_static.f64_values[662]=(if ((self.scalar_static.f64_values[120])!=0.0){self.scalar_static.f64_values[661]}else{0.0});
        self.scalar_static.f64_values[663]=(if self.scalar_static.bool_values[13]{self.scalar_static.f64_values[121]}else{self.scalar_static.f64_values[662]});
        self.scalar_static.f64_values[664]=(self.scalar_static.f64_values[411]*self.scalar_static.f64_values[123]);
        self.scalar_static.f64_values[665]=(1.0+self.scalar_static.f64_values[664]);
        self.scalar_static.f64_values[666]=(self.scalar_static.f64_values[122]*self.scalar_static.f64_values[665]);
        self.scalar_static.f64_values[667]=(self.scalar_static.f64_values[666]*self.scalar_static.f64_values[666]);
        self.scalar_static.bool_values[108]=(self.scalar_static.f64_values[666]<0.0);
        self.scalar_static.f64_values[668]=(if self.scalar_static.bool_values[108]{1.0}else{0.0});
        self.scalar_static.f64_values[669]=(1e-6+self.scalar_static.f64_values[667]);
        self.scalar_static.f64_values[670]=(self.scalar_static.f64_values[669]).sqrt();
        self.scalar_static.f64_values[671]=(self.scalar_static.f64_values[670]-self.scalar_static.f64_values[666]);
        self.scalar_static.f64_values[672]=(5e-7/self.scalar_static.f64_values[671]);
        self.scalar_static.f64_values[673]=(if ((self.scalar_static.f64_values[668])!=0.0){self.scalar_static.f64_values[672]}else{0.0});
        self.scalar_static.bool_values[109]=(!((self.scalar_static.f64_values[668])!=0.0));
        self.scalar_static.f64_values[674]=(self.scalar_static.f64_values[666]+self.scalar_static.f64_values[670]);
        self.scalar_static.f64_values[675]=(0.5*self.scalar_static.f64_values[674]);
        self.scalar_static.f64_values[676]=(if self.scalar_static.bool_values[109]{self.scalar_static.f64_values[675]}else{self.scalar_static.f64_values[673]});
        self.scalar_static.f64_values[677]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[128]);
        self.scalar_static.f64_values[678]=(self.scalar_static.f64_values[677]/self.scalar_static.f64_values[639]);
        self.scalar_static.f64_values[679]=(self.scalar_static.f64_values[678]).exp();
        self.scalar_static.f64_values[680]=(self.scalar_static.f64_values[124]*self.scalar_static.f64_values[679]);
        self.scalar_static.f64_values[681]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[129]);
        self.scalar_static.f64_values[682]=(self.scalar_static.f64_values[681]/self.scalar_static.f64_values[639]);
        self.scalar_static.f64_values[683]=(self.scalar_static.f64_values[682]).exp();
        self.scalar_static.f64_values[684]=(self.scalar_static.f64_values[680]*self.scalar_static.f64_values[683]);
        self.scalar_static.f64_values[685]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[131]);
        self.scalar_static.f64_values[686]=(self.scalar_static.f64_values[685]).exp();
        self.scalar_static.f64_values[687]=(self.scalar_static.f64_values[130]*self.scalar_static.f64_values[686]);
        self.scalar_static.f64_values[688]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[134]);
        self.scalar_static.f64_values[689]=(self.scalar_static.f64_values[688]).exp();
        self.scalar_static.f64_values[690]=(self.scalar_static.f64_values[132]*self.scalar_static.f64_values[689]);
        self.scalar_static.f64_values[691]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[138]);
        self.scalar_static.f64_values[692]=(self.scalar_static.f64_values[691]).exp();
        self.scalar_static.f64_values[693]=(self.scalar_static.f64_values[135]*self.scalar_static.f64_values[692]);
        self.scalar_static.f64_values[694]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[140]);
        self.scalar_static.f64_values[695]=(self.scalar_static.f64_values[694]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[696]=(self.scalar_static.f64_values[695]).exp();
        self.scalar_static.f64_values[697]=(self.scalar_static.f64_values[693]*self.scalar_static.f64_values[696]);
        self.scalar_static.f64_values[698]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[144]);
        self.scalar_static.f64_values[699]=(self.scalar_static.f64_values[698]).exp();
        self.scalar_static.f64_values[700]=(self.scalar_static.f64_values[141]*self.scalar_static.f64_values[699]);
        self.scalar_static.f64_values[701]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[145]);
        self.scalar_static.f64_values[702]=(self.scalar_static.f64_values[701]/self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[703]=(self.scalar_static.f64_values[702]).exp();
        self.scalar_static.f64_values[704]=(self.scalar_static.f64_values[700]*self.scalar_static.f64_values[703]);
        self.scalar_static.f64_values[705]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[148]);
        self.scalar_static.f64_values[706]=(self.scalar_static.f64_values[705]/self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[707]=(self.scalar_static.f64_values[706]).exp();
        self.scalar_static.f64_values[708]=(self.scalar_static.f64_values[146]*self.scalar_static.f64_values[707]);
        self.scalar_static.f64_values[709]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[151]);
        self.scalar_static.f64_values[710]=(self.scalar_static.f64_values[709]/self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[711]=(self.scalar_static.f64_values[710]).exp();
        self.scalar_static.f64_values[712]=(self.scalar_static.f64_values[708]*self.scalar_static.f64_values[711]);
        self.scalar_static.f64_values[713]=(self.scalar_static.f64_values[705]/self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[714]=(self.scalar_static.f64_values[713]).exp();
        self.scalar_static.f64_values[715]=(self.scalar_static.f64_values[152]*self.scalar_static.f64_values[714]);
        self.scalar_static.f64_values[716]=(self.scalar_static.f64_values[709]/self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[717]=(self.scalar_static.f64_values[716]).exp();
        self.scalar_static.f64_values[718]=(self.scalar_static.f64_values[715]*self.scalar_static.f64_values[717]);
        self.scalar_static.f64_values[719]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[158]);
        self.scalar_static.f64_values[720]=(self.scalar_static.f64_values[719]/self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[721]=(self.scalar_static.f64_values[720]).exp();
        self.scalar_static.f64_values[722]=(self.scalar_static.f64_values[156]*self.scalar_static.f64_values[721]);
        self.scalar_static.f64_values[723]=(if ((self.scalar_static.f64_values[155])!=0.0){self.scalar_static.f64_values[722]}else{0.0});
        self.scalar_static.f64_values[724]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[161]);
        self.scalar_static.f64_values[725]=(self.scalar_static.f64_values[724]).exp();
        self.scalar_static.f64_values[726]=(self.scalar_static.f64_values[159]*self.scalar_static.f64_values[725]);
        self.scalar_static.f64_values[727]=(if ((self.scalar_static.f64_values[155])!=0.0){self.scalar_static.f64_values[726]}else{0.0});
        self.scalar_static.f64_values[728]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[164]);
        self.scalar_static.f64_values[729]=(self.scalar_static.f64_values[728]/self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[730]=(self.scalar_static.f64_values[729]).exp();
        self.scalar_static.f64_values[731]=(self.scalar_static.f64_values[162]*self.scalar_static.f64_values[730]);
        self.scalar_static.f64_values[732]=(if ((self.scalar_static.f64_values[155])!=0.0){self.scalar_static.f64_values[731]}else{0.0});
        self.scalar_static.f64_values[733]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[167]);
        self.scalar_static.f64_values[734]=(self.scalar_static.f64_values[733]).exp();
        self.scalar_static.f64_values[735]=(self.scalar_static.f64_values[165]*self.scalar_static.f64_values[734]);
        self.scalar_static.f64_values[736]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[169]);
        self.scalar_static.f64_values[737]=(self.scalar_static.f64_values[736]).exp();
        self.scalar_static.f64_values[738]=(self.scalar_static.f64_values[735]*self.scalar_static.f64_values[737]);
        self.scalar_static.f64_values[739]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[173]);
        self.scalar_static.f64_values[740]=(self.scalar_static.f64_values[739]).exp();
        self.scalar_static.f64_values[741]=(self.scalar_static.f64_values[170]*self.scalar_static.f64_values[740]);
        self.scalar_static.f64_values[742]=(self.scalar_static.f64_values[694]/self.scalar_static.f64_values[171]);
        self.scalar_static.f64_values[743]=(self.scalar_static.f64_values[742]).exp();
        self.scalar_static.f64_values[744]=(self.scalar_static.f64_values[741]*self.scalar_static.f64_values[743]);
        self.scalar_static.f64_values[745]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[176]);
        self.scalar_static.f64_values[746]=(self.scalar_static.f64_values[745]).exp();
        self.scalar_static.f64_values[747]=(self.scalar_static.f64_values[174]*self.scalar_static.f64_values[746]);
        self.scalar_static.f64_values[748]=(self.scalar_static.f64_values[694]/self.scalar_static.f64_values[175]);
        self.scalar_static.f64_values[749]=(self.scalar_static.f64_values[748]).exp();
        self.scalar_static.f64_values[750]=(self.scalar_static.f64_values[747]*self.scalar_static.f64_values[749]);
        self.scalar_static.f64_values[751]=(self.scalar_static.f64_values[407]).sqrt();
        self.scalar_static.f64_values[752]=(self.scalar_static.f64_values[177]*self.scalar_static.f64_values[751]);
        self.scalar_static.f64_values[753]=(self.scalar_static.f64_values[411]*self.scalar_static.f64_values[178]);
        self.scalar_static.f64_values[754]=(self.scalar_static.f64_values[753]).exp();
        self.scalar_static.f64_values[755]=(self.scalar_static.f64_values[752]*self.scalar_static.f64_values[754]);
        self.scalar_static.f64_values[756]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[433]);
        self.scalar_static.f64_values[757]=f64::powf(self.scalar_static.f64_values[756],-0.5);
        self.scalar_static.f64_values[758]=(1.0/self.scalar_static.f64_values[579]);
        self.scalar_static.f64_values[759]=(self.scalar_static.f64_values[433]*self.scalar_static.f64_values[179]);
        self.scalar_static.f64_values[760]=(self.scalar_static.f64_values[433]*self.scalar_static.f64_values[759]);
        self.scalar_static.f64_values[761]=(self.scalar_static.f64_values[757]*self.scalar_static.f64_values[760]);
        self.scalar_static.f64_values[762]=(self.scalar_static.f64_values[758]*self.scalar_static.f64_values[761]);
        self.scalar_static.f64_values[763]=(self.scalar_static.f64_values[49]*self.scalar_static.f64_values[762]);
        self.scalar_static.f64_values[764]=(self.scalar_static.f64_values[576]*self.scalar_static.f64_values[763]);
        self.scalar_static.f64_values[765]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[764]);
        self.scalar_static.f64_values[766]=(self.scalar_static.f64_values[48]*self.scalar_static.f64_values[765]);
        self.scalar_static.f64_values[767]=(self.scalar_static.f64_values[757]*self.scalar_static.f64_values[180]);
        self.scalar_static.f64_values[768]=(self.scalar_static.f64_values[477]*self.scalar_static.f64_values[767]);
        self.scalar_static.f64_values[769]=(self.scalar_static.f64_values[477]*self.scalar_static.f64_values[768]);
        self.scalar_static.f64_values[770]=(self.scalar_static.f64_values[50]*self.scalar_static.f64_values[769]);
        self.scalar_static.f64_values[771]=(self.scalar_static.f64_values[50]*self.scalar_static.f64_values[770]);
        self.scalar_static.f64_values[772]=(self.scalar_static.f64_values[579]*self.scalar_static.f64_values[771]);
        self.scalar_static.f64_values[773]=(self.scalar_static.f64_values[179]-self.scalar_static.f64_values[766]);
        self.scalar_static.f64_values[774]=(self.scalar_static.f64_values[773]).exp();
        self.scalar_static.f64_values[775]=(self.scalar_static.f64_values[772]*self.scalar_static.f64_values[774]);
        self.scalar_static.f64_values[776]=(self.scalar_static.f64_values[80]*self.scalar_static.f64_values[454]);
        self.scalar_static.f64_values[777]=f64::powf(self.scalar_static.f64_values[776],-0.5);
        self.scalar_static.f64_values[778]=(1.0/self.scalar_static.f64_values[581]);
        self.scalar_static.f64_values[779]=(self.scalar_static.f64_values[454]*self.scalar_static.f64_values[181]);
        self.scalar_static.f64_values[780]=(self.scalar_static.f64_values[454]*self.scalar_static.f64_values[779]);
        self.scalar_static.f64_values[781]=(self.scalar_static.f64_values[777]*self.scalar_static.f64_values[780]);
        self.scalar_static.f64_values[782]=(self.scalar_static.f64_values[778]*self.scalar_static.f64_values[781]);
        self.scalar_static.f64_values[783]=(self.scalar_static.f64_values[51]*self.scalar_static.f64_values[782]);
        self.scalar_static.f64_values[784]=(self.scalar_static.f64_values[577]*self.scalar_static.f64_values[783]);
        self.scalar_static.f64_values[785]=(self.scalar_static.f64_values[80]*self.scalar_static.f64_values[784]);
        self.scalar_static.f64_values[786]=(self.scalar_static.f64_values[80]*self.scalar_static.f64_values[785]);
        self.scalar_static.f64_values[787]=(self.scalar_static.f64_values[777]*self.scalar_static.f64_values[182]);
        self.scalar_static.f64_values[788]=(self.scalar_static.f64_values[535]*self.scalar_static.f64_values[787]);
        self.scalar_static.f64_values[789]=(self.scalar_static.f64_values[535]*self.scalar_static.f64_values[788]);
        self.scalar_static.f64_values[790]=(self.scalar_static.f64_values[81]*self.scalar_static.f64_values[789]);
        self.scalar_static.f64_values[791]=(self.scalar_static.f64_values[81]*self.scalar_static.f64_values[790]);
        self.scalar_static.f64_values[792]=(self.scalar_static.f64_values[581]*self.scalar_static.f64_values[791]);
        self.scalar_static.f64_values[793]=(self.scalar_static.f64_values[181]-self.scalar_static.f64_values[786]);
        self.scalar_static.f64_values[794]=(self.scalar_static.f64_values[793]).exp();
        self.scalar_static.f64_values[795]=(self.scalar_static.f64_values[792]*self.scalar_static.f64_values[794]);
        self.scalar_static.f64_values[796]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[105]);
        self.scalar_static.f64_values[797]=(self.scalar_static.f64_values[796]).exp();
        self.scalar_static.f64_values[798]=(self.scalar_static.f64_values[797]*self.scalar_static.f64_values[183]);
        self.scalar_static.f64_values[799]=(self.scalar_static.f64_values[590]*self.scalar_static.f64_values[798]);
        self.scalar_static.f64_values[800]=(self.scalar_static.f64_values[797]*self.scalar_static.f64_values[184]);
        self.scalar_static.f64_values[801]=(self.scalar_static.f64_values[758]*self.scalar_static.f64_values[800]);
        self.scalar_static.f64_values[802]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[187]);
        self.scalar_static.f64_values[803]=(self.scalar_static.f64_values[802]).exp();
        self.scalar_static.f64_values[804]=(self.scalar_static.f64_values[185]*self.scalar_static.f64_values[803]);
        self.scalar_static.f64_values[805]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[188]);
        self.scalar_static.f64_values[806]=(self.scalar_static.f64_values[805]).exp();
        self.scalar_static.f64_values[807]=(self.scalar_static.f64_values[804]*self.scalar_static.f64_values[806]);
        self.scalar_static.f64_values[808]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[191]);
        self.scalar_static.f64_values[809]=(self.scalar_static.f64_values[808]).exp();
        self.scalar_static.f64_values[810]=(self.scalar_static.f64_values[18]*self.scalar_static.f64_values[809]);
        self.scalar_static.f64_values[811]=(self.scalar_static.f64_values[806]*self.scalar_static.f64_values[810]);
        self.scalar_static.f64_values[812]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[193]);
        self.scalar_static.f64_values[813]=(self.scalar_static.f64_values[812]).exp();
        self.scalar_static.f64_values[814]=(self.scalar_static.f64_values[192]*self.scalar_static.f64_values[813]);
        self.scalar_static.f64_values[815]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[195]);
        self.scalar_static.f64_values[816]=(self.scalar_static.f64_values[815]).exp();
        self.scalar_static.f64_values[817]=(self.scalar_static.f64_values[194]*self.scalar_static.f64_values[816]);
        self.scalar_static.f64_values[818]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[197]);
        self.scalar_static.f64_values[819]=(self.scalar_static.f64_values[818]).exp();
        self.scalar_static.f64_values[820]=(self.scalar_static.f64_values[196]*self.scalar_static.f64_values[819]);
        self.scalar_static.f64_values[821]=(self.scalar_static.f64_values[410]*self.scalar_static.f64_values[199]);
        self.scalar_static.f64_values[822]=(self.scalar_static.f64_values[821]).exp();
        self.scalar_static.f64_values[823]=(self.scalar_static.f64_values[820]*self.scalar_static.f64_values[822]);
        self.scalar_static.f64_values[824]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[202]);
        self.scalar_static.f64_values[825]=(self.scalar_static.f64_values[824]).exp();
        self.scalar_static.f64_values[826]=(self.scalar_static.f64_values[200]*self.scalar_static.f64_values[825]);
        self.scalar_static.f64_values[827]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[204]);
        self.scalar_static.f64_values[828]=(self.scalar_static.f64_values[827]).exp();
        self.scalar_static.f64_values[829]=(self.scalar_static.f64_values[203]*self.scalar_static.f64_values[828]);
        self.scalar_static.f64_values[830]=(self.scalar_static.f64_values[826]+self.scalar_static.f64_values[829]);
        self.scalar_static.f64_values[831]=(self.scalar_static.f64_values[205]*self.scalar_static.f64_values[830]);
        self.scalar_static.f64_values[832]=(self.scalar_static.f64_values[831]/self.scalar_static.f64_values[206]);
        self.scalar_static.f64_values[833]=(self.scalar_static.f64_values[412]*self.scalar_static.f64_values[209]);
        self.scalar_static.f64_values[834]=(self.scalar_static.f64_values[833]).exp();
        self.scalar_static.f64_values[835]=(self.scalar_static.f64_values[207]*self.scalar_static.f64_values[834]);
        self.scalar_static.f64_values[836]=(self.scalar_static.f64_values[406]-300.0);
        self.scalar_static.bool_values[110]=(self.scalar_static.f64_values[406]<525.0);
        self.scalar_static.f64_values[837]=(if self.scalar_static.bool_values[110]{1.0}else{0.0});
        self.scalar_static.f64_values[838]=(self.scalar_static.f64_values[836]*0.00072);
        self.scalar_static.f64_values[839]=(1.0+self.scalar_static.f64_values[838]);
        self.scalar_static.f64_values[840]=(self.scalar_static.f64_values[836]*1.6e-6);
        self.scalar_static.f64_values[841]=(self.scalar_static.f64_values[836]*self.scalar_static.f64_values[840]);
        self.scalar_static.f64_values[842]=(self.scalar_static.f64_values[839]-self.scalar_static.f64_values[841]);
        self.scalar_static.f64_values[843]=(self.scalar_static.f64_values[5]*self.scalar_static.f64_values[842]);
        self.scalar_static.f64_values[844]=(if ((self.scalar_static.f64_values[837])!=0.0){self.scalar_static.f64_values[843]}else{0.0});
        self.scalar_static.bool_values[111]=(!((self.scalar_static.f64_values[837])!=0.0));
        self.scalar_static.f64_values[845]=(if self.scalar_static.bool_values[111]{self.scalar_static.f64_values[210]}else{self.scalar_static.f64_values[844]});
        self.scalar_static.f64_values[846]=(self.scalar_static.f64_values[797]*self.scalar_static.f64_values[211]);
        self.scalar_static.f64_values[847]=(1.0/self.scalar_static.f64_values[608]);
        self.scalar_static.f64_values[848]=(if ((self.scalar_static.f64_values[212])!=0.0){self.scalar_static.f64_values[847]}else{0.0});
        self.scalar_static.bool_values[112]=(self.scalar_static.f64_values[848]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[849]=(if self.scalar_static.bool_values[112]{1.0}else{0.0});
        self.scalar_static.bool_values[113]=(((self.scalar_static.f64_values[212])!=0.0)&&((self.scalar_static.f64_values[849])!=0.0));
        self.scalar_static.f64_values[850]=(if self.scalar_static.bool_values[113]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[848]});
        self.scalar_static.f64_values[851]=(if self.scalar_static.bool_values[16]{0.0}else{self.scalar_static.f64_values[850]});
        self.scalar_static.f64_values[852]=(1.0/self.scalar_static.f64_values[611]);
        self.scalar_static.f64_values[853]=(if ((self.scalar_static.f64_values[213])!=0.0){self.scalar_static.f64_values[852]}else{0.0});
        self.scalar_static.bool_values[114]=(self.scalar_static.f64_values[853]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[854]=(if self.scalar_static.bool_values[114]{1.0}else{0.0});
        self.scalar_static.bool_values[115]=(((self.scalar_static.f64_values[213])!=0.0)&&((self.scalar_static.f64_values[854])!=0.0));
        self.scalar_static.f64_values[855]=(if self.scalar_static.bool_values[115]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[853]});
        self.scalar_static.f64_values[856]=(if self.scalar_static.bool_values[18]{0.0}else{self.scalar_static.f64_values[855]});
        self.scalar_static.f64_values[857]=(1.0/self.scalar_static.f64_values[612]);
        self.scalar_static.f64_values[858]=(if ((self.scalar_static.f64_values[214])!=0.0){self.scalar_static.f64_values[857]}else{0.0});
        self.scalar_static.bool_values[116]=(self.scalar_static.f64_values[858]>self.scalar_static.f64_values[17]);
        self.scalar_static.f64_values[859]=(if self.scalar_static.bool_values[116]{1.0}else{0.0});
        self.scalar_static.bool_values[117]=(((self.scalar_static.f64_values[214])!=0.0)&&((self.scalar_static.f64_values[859])!=0.0));
        self.scalar_static.f64_values[860]=(if self.scalar_static.bool_values[117]{self.scalar_static.f64_values[17]}else{self.scalar_static.f64_values[858]});
        self.scalar_static.f64_values[861]=(if self.scalar_static.bool_values[20]{0.0}else{self.scalar_static.f64_values[860]});
        self.scalar_static.f64_values[862]=(2.0*self.scalar_static.f64_values[408]);
        self.scalar_static.f64_values[863]=(self.scalar_static.f64_values[497]*0.2);
        self.scalar_static.f64_values[864]=(self.scalar_static.f64_values[615]*self.scalar_static.f64_values[218]);
        self.scalar_static.f64_values[865]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[497]);
        self.scalar_static.f64_values[866]=(self.scalar_static.f64_values[865]).exp();
        self.scalar_static.f64_values[867]=(self.scalar_static.f64_values[615]*self.scalar_static.f64_values[219]);
        self.scalar_static.f64_values[868]=(self.scalar_static.f64_values[218]*self.scalar_static.f64_values[867]);
        self.scalar_static.f64_values[869]=(0.1*self.scalar_static.f64_values[535]);
        self.scalar_static.f64_values[870]=(self.scalar_static.f64_values[408]*1e-5);
        self.scalar_static.f64_values[871]=(self.scalar_static.f64_values[408]*1e-40);
        self.scalar_static.f64_values[872]=(self.scalar_static.f64_values[477]*self.scalar_static.f64_values[234]);
        self.scalar_static.f64_values[873]=(0.1*self.scalar_static.f64_values[477]);
        self.scalar_static.f64_values[874]=(self.scalar_static.f64_values[477]/self.scalar_static.f64_values[235]);
        self.scalar_static.f64_values[875]=(2.0-self.scalar_static.f64_values[592]);
        self.scalar_static.f64_values[876]=(1.0-self.scalar_static.f64_values[592]);
        self.scalar_static.f64_values[877]=(self.scalar_static.f64_values[875]/self.scalar_static.f64_values[876]);
        self.scalar_static.f64_values[878]=f64::powf(self.scalar_static.f64_values[877],self.scalar_static.f64_values[239]);
        self.scalar_static.f64_values[879]=(1.0-self.scalar_static.f64_values[878]);
        self.scalar_static.f64_values[880]=(self.scalar_static.f64_values[535]*self.scalar_static.f64_values[879]);
        self.scalar_static.f64_values[881]=(self.scalar_static.f64_values[535]/self.scalar_static.f64_values[241]);
        self.scalar_static.f64_values[882]=(4.0*self.scalar_static.f64_values[684]);
        self.scalar_static.f64_values[883]=(self.scalar_static.f64_values[882]/self.scalar_static.f64_values[687]);
        self.scalar_static.f64_values[884]=(1.0/self.scalar_static.f64_values[663]);
        self.scalar_static.f64_values[885]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[846]);
        self.scalar_static.f64_values[886]=(self.scalar_static.f64_values[885]).exp();
        self.scalar_static.f64_values[887]=(self.scalar_static.f64_values[886]-1.0);
        self.scalar_static.f64_values[888]=(self.scalar_static.f64_values[684]*self.scalar_static.f64_values[243]);
        self.scalar_static.f64_values[889]=(2.0*self.scalar_static.f64_values[723]);
        self.scalar_static.f64_values[890]=(2.0*self.scalar_static.f64_values[732]);
        self.scalar_static.f64_values[891]=(2.0*self.scalar_static.f64_values[775]);
        self.scalar_static.f64_values[892]=(2.0*self.scalar_static.f64_values[795]);
        self.scalar_static.f64_values[893]=(2.0*self.scalar_static.f64_values[738]);
        self.scalar_static.f64_values[894]=(4.0*self.scalar_static.f64_values[738]);
        self.scalar_static.f64_values[895]=(self.scalar_static.f64_values[894]/self.scalar_static.f64_values[690]);
        self.scalar_static.f64_values[896]=(self.scalar_static.f64_values[807]*self.scalar_static.f64_values[261]);
        self.scalar_static.f64_values[897]=(self.scalar_static.f64_values[807]/self.scalar_static.f64_values[814]);
        self.scalar_static.f64_values[898]=(4.0*self.scalar_static.f64_values[897]);
        self.scalar_static.f64_values[899]=(self.scalar_static.f64_values[807]*self.scalar_static.f64_values[264]);
        self.scalar_static.f64_values[900]=(2.0*self.scalar_static.f64_values[811]);
        self.scalar_static.f64_values[901]=(self.scalar_static.f64_values[811]/self.scalar_static.f64_values[817]);
        self.scalar_static.f64_values[902]=(self.scalar_static.f64_values[265]*self.scalar_static.f64_values[901]);
        self.scalar_static.f64_values[903]=(self.scalar_static.f64_values[738]*self.scalar_static.f64_values[268]);
        self.scalar_static.f64_values[904]=(self.scalar_static.f64_values[807]*self.scalar_static.f64_values[270]);
        self.scalar_static.f64_values[905]=(4.0*self.scalar_static.f64_values[807]);
        self.scalar_static.f64_values[906]=(self.scalar_static.f64_values[905]/self.scalar_static.f64_values[814]);
        self.scalar_static.f64_values[907]=(self.scalar_static.f64_values[738]+self.scalar_static.f64_values[807]);
        self.scalar_static.f64_values[908]=(self.scalar_static.f64_values[6]*self.scalar_static.f64_values[907]);
        self.scalar_static.f64_values[909]=(self.scalar_static.f64_values[608]*self.scalar_static.f64_values[908]);
        self.scalar_static.f64_values[910]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[909]}else{0.0});
        self.scalar_static.f64_values[911]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[910]);
        self.scalar_static.f64_values[912]=(self.scalar_static.f64_values[911]).ln();
        self.scalar_static.f64_values[913]=(2.0-self.scalar_static.f64_values[912]);
        self.scalar_static.f64_values[914]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[913]);
        self.scalar_static.f64_values[915]=(if self.scalar_static.bool_values[50]{self.scalar_static.f64_values[914]}else{0.0});
        self.scalar_static.f64_values[916]=(-self.scalar_static.f64_values[676]);
        self.scalar_static.f64_values[917]=(self.scalar_static.f64_values[298]/self.scalar_static.f64_values[676]);
        self.scalar_static.f64_values[918]=(self.scalar_static.f64_values[4]/self.scalar_static.f64_values[845]);
        self.scalar_static.f64_values[919]=(-self.scalar_static.f64_values[845]);
        self.scalar_static.f64_values[920]=(self.scalar_static.f64_values[582]*self.scalar_static.f64_values[321]);
        self.scalar_static.f64_values[921]=(self.scalar_static.f64_values[582]*self.scalar_static.f64_values[320]);
        self.scalar_static.f64_values[922]=(self.scalar_static.f64_values[591]*self.scalar_static.f64_values[322]);
        self.scalar_static.f64_values[923]=(self.scalar_static.f64_values[687]*self.scalar_static.f64_values[826]);
        self.scalar_static.f64_values[924]=(0.5*self.scalar_static.f64_values[923]);
        self.scalar_static.f64_values[925]=(0.1*self.scalar_static.f64_values[575]);
        self.scalar_static.f64_values[926]=(self.scalar_static.f64_values[575]*self.scalar_static.f64_values[326]);
        self.scalar_static.f64_values[927]=(self.scalar_static.f64_values[575]/self.scalar_static.f64_values[327]);
        self.scalar_static.f64_values[928]=(self.scalar_static.f64_values[687]*self.scalar_static.f64_values[823]);
        self.scalar_static.f64_values[929]=(self.scalar_static.f64_values[684]/self.scalar_static.f64_values[687]);
        self.scalar_static.f64_values[930]=f64::powf(self.scalar_static.f64_values[929],self.scalar_static.f64_values[329]);
        self.scalar_static.f64_values[931]=(self.scalar_static.f64_values[928]*self.scalar_static.f64_values[930]);
        self.scalar_static.f64_values[932]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[328]);
        self.scalar_static.f64_values[933]=(4.0*self.scalar_static.f64_values[829]);
        self.scalar_static.f64_values[934]=(self.scalar_static.f64_values[408]*self.scalar_static.f64_values[933]);
        self.scalar_static.f64_values[935]=(self.scalar_static.f64_values[934]/self.scalar_static.f64_values[615]);
        self.scalar_static.f64_values[936]=(0.5*self.scalar_static.f64_values[935]);
        self.scalar_static.f64_values[937]=(0.5*self.scalar_static.f64_values[832]);
        self.scalar_static.f64_values[938]=(self.scalar_static.f64_values[835]*self.scalar_static.f64_values[893]);
        self.scalar_static.f64_values[939]=(self.scalar_static.f64_values[832]*self.scalar_static.f64_values[334]);
        self.scalar_static.f64_values[940]=(self.scalar_static.f64_values[835]*self.scalar_static.f64_values[903]);
        self.scalar_static.f64_values[941]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[409]);
        self.scalar_static.f64_values[942]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[943]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[639]);
        self.scalar_static.f64_values[944]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[639]);
        self.scalar_static.f64_values[945]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[353]);
        self.scalar_static.f64_values[946]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[354]);
        self.scalar_static.f64_values[947]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[355]);
        self.scalar_static.f64_values[948]=(self.scalar_static.f64_values[352]/self.scalar_static.f64_values[873]);
        self.scalar_static.f64_values[949]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[873]);
        self.scalar_static.f64_values[950]=(-self.scalar_static.f64_values[948]);
        self.scalar_static.f64_values[951]=(-self.scalar_static.f64_values[949]);
        self.scalar_static.f64_values[952]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[592]);
        self.scalar_static.f64_values[953]=(self.scalar_static.f64_values[592]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[954]=(self.scalar_static.f64_values[884]-1.0);
        self.scalar_static.f64_values[955]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[956]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[149]);
        self.scalar_static.f64_values[957]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[958]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[153]);
        self.scalar_static.f64_values[959]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[960]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[136]);
        self.scalar_static.f64_values[961]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[171]);
        self.scalar_static.f64_values[962]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[171]);
        self.scalar_static.f64_values[963]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[964]=(self.scalar_static.f64_values[945]/self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[965]=(self.scalar_static.f64_values[946]/self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[966]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[142]);
        self.scalar_static.f64_values[967]=(self.scalar_static.f64_values[942]/self.scalar_static.f64_values[175]);
        self.scalar_static.f64_values[968]=(self.scalar_static.f64_values[941]/self.scalar_static.f64_values[175]);
        self.scalar_static.f64_values[969]=(self.scalar_static.f64_values[576]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[970]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[576]);
        self.scalar_static.f64_values[971]=(self.scalar_static.f64_values[766]*self.scalar_static.f64_values[370]);
        self.scalar_static.f64_values[972]=(self.scalar_static.f64_values[766]*self.scalar_static.f64_values[371]);
        self.scalar_static.f64_values[973]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[577]);
        self.scalar_static.f64_values[974]=(self.scalar_static.f64_values[577]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[975]=(-self.scalar_static.f64_values[973]);
        self.scalar_static.f64_values[976]=(-self.scalar_static.f64_values[974]);
        self.scalar_static.f64_values[977]=(self.scalar_static.f64_values[786]*self.scalar_static.f64_values[375]);
        self.scalar_static.f64_values[978]=(self.scalar_static.f64_values[786]*self.scalar_static.f64_values[376]);
        self.scalar_static.f64_values[979]=(self.scalar_static.f64_values[917]*self.scalar_static.f64_values[352]);
        self.scalar_static.f64_values[980]=(self.scalar_static.f64_values[0]*self.scalar_static.f64_values[917]);
        self.scalar_static.f64_values[981]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[869]);
        self.scalar_static.f64_values[982]=(self.scalar_static.f64_values[353]/self.scalar_static.f64_values[869]);
        self.scalar_static.f64_values[983]=(self.scalar_static.f64_values[354]/self.scalar_static.f64_values[869]);
        self.scalar_static.f64_values[984]=(self.scalar_static.f64_values[352]/self.scalar_static.f64_values[869]);
        self.scalar_static.f64_values[985]=(-self.scalar_static.f64_values[981]);
        self.scalar_static.f64_values[986]=(-self.scalar_static.f64_values[982]);
        self.scalar_static.f64_values[987]=(-self.scalar_static.f64_values[983]);
        self.scalar_static.f64_values[988]=(-self.scalar_static.f64_values[984]);
        self.scalar_static.f64_values[989]=(self.scalar_static.f64_values[592]*self.scalar_static.f64_values[353]);
        self.scalar_static.f64_values[990]=(self.scalar_static.f64_values[592]*self.scalar_static.f64_values[354]);
        self.scalar_static.f64_values[991]=(self.scalar_static.f64_values[355]/self.scalar_static.f64_values[869]);
        self.scalar_static.f64_values[992]=(-self.scalar_static.f64_values[991]);
        self.scalar_static.f64_values[993]=(self.scalar_static.f64_values[592]*self.scalar_static.f64_values[355]);
        self.scalar_static.f64_values[994]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[925]);
        self.scalar_static.f64_values[995]=(self.scalar_static.f64_values[352]/self.scalar_static.f64_values[925]);
        self.scalar_static.f64_values[996]=(-self.scalar_static.f64_values[994]);
        self.scalar_static.f64_values[997]=(-self.scalar_static.f64_values[995]);
        self.scalar_static.f64_values[998]=(self.scalar_static.f64_values[352]/self.scalar_static.f64_values[932]);
        self.scalar_static.f64_values[999]=(self.scalar_static.f64_values[0]/self.scalar_static.f64_values[932]);
        self.scalar_static.f64_values[1000]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[391]);
        self.scalar_static.f64_values[1001]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[392]);
        self.scalar_static.f64_values[1002]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[393]);
        self.scalar_static.f64_values[1003]=(self.scalar_static.f64_values[409]*self.scalar_static.f64_values[394]);
        self.scalar_static.f64_values[1004]=(if ((self.scalar_static.f64_values[336])!=0.0){self.scalar_static.f64_values[948]}else{0.0});
        self.scalar_static.f64_values[1005]=(if ((self.scalar_static.f64_values[336])!=0.0){self.scalar_static.f64_values[949]}else{0.0});
        self.scalar_static.f64_values[1006]=(-self.scalar_static.f64_values[1004]);
        self.scalar_static.f64_values[1007]=(-self.scalar_static.f64_values[1005]);
        self.scalar_static.f64_values[1008]=(self.scalar_static.f64_values[398]/self.scalar_static.f64_values[597]);
        self.scalar_static.f64_values[1009]=(self.scalar_static.f64_values[399]/self.scalar_static.f64_values[597]);
        self.scalar_static.f64_values[1010]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1008]);
        self.scalar_static.f64_values[1011]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1009]);
        self.scalar_static.f64_values[1012]=(self.scalar_static.f64_values[398]/self.scalar_static.f64_values[605]);
        self.scalar_static.f64_values[1013]=(self.scalar_static.f64_values[399]/self.scalar_static.f64_values[605]);
        self.scalar_static.f64_values[1014]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1012]);
        self.scalar_static.f64_values[1015]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1013]);
        self.scalar_static.f64_values[1016]=(self.scalar_static.f64_values[851]*self.scalar_static.f64_values[398]);
        self.scalar_static.f64_values[1017]=(self.scalar_static.f64_values[851]*self.scalar_static.f64_values[404]);
        self.scalar_static.f64_values[1018]=(self.scalar_static.f64_values[851]*self.scalar_static.f64_values[405]);
        self.scalar_static.f64_values[1019]=(self.scalar_static.f64_values[851]*self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[1020]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1016]);
        self.scalar_static.f64_values[1021]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1017]);
        self.scalar_static.f64_values[1022]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1018]);
        self.scalar_static.f64_values[1023]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1019]);
        self.scalar_static.f64_values[1024]=(self.scalar_static.f64_values[856]*self.scalar_static.f64_values[398]);
        self.scalar_static.f64_values[1025]=(self.scalar_static.f64_values[856]*self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[1026]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1024]);
        self.scalar_static.f64_values[1027]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1025]);
        self.scalar_static.f64_values[1028]=(if ((self.scalar_static.f64_values[213])!=0.0){self.scalar_static.f64_values[1026]}else{0.0});
        self.scalar_static.f64_values[1029]=(if ((self.scalar_static.f64_values[213])!=0.0){self.scalar_static.f64_values[1027]}else{0.0});
        self.scalar_static.f64_values[1030]=(self.scalar_static.f64_values[861]*self.scalar_static.f64_values[399]);
        self.scalar_static.f64_values[1031]=(self.scalar_static.f64_values[861]*self.scalar_static.f64_values[398]);
        self.scalar_static.f64_values[1032]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1030]);
        self.scalar_static.f64_values[1033]=(self.scalar_static.f64_values[15]*self.scalar_static.f64_values[1031]);
        self.scalar_static.f64_values[1034]=(if ((self.scalar_static.f64_values[214])!=0.0){self.scalar_static.f64_values[1032]}else{0.0});
        self.scalar_static.f64_values[1035]=(if ((self.scalar_static.f64_values[214])!=0.0){self.scalar_static.f64_values[1033]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
