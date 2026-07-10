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
    pub p48: f64, pub p49: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 50] = [
                1e-6, 1e-6, 100.0, 1.0, 1.0, 0.0, 1.0, 1.0,
                1.0, 2.0, 1.0, 0.0, -100.0, 500.0, 0.001, 1002.0,
                27.0, 100.0, 0.0, 9900000000.0, 0.0, 9900000000.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                1.0, 0.0, 100.0, -100.0, 500.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1000000.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 50);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 52] = [
    ("w", 0), ("l", 1), ("r", 2), ("c1", 3), ("c2", 4), ("trise", 5), ("dtemp", 5), ("dra", 5), ("isnoisy", 6), ("sw_et", 7), ("version", 8), ("revision", 9), ("scale", 10), ("shrink", 11), ("tmin", 12), ("tmax", 13),
    ("rthresh", 14), ("level", 15), ("tnom", 16), ("rsh", 17), ("lmin", 18), ("lmax", 19), ("wmin", 20), ("wmax", 21), ("xw", 22), ("xl", 23), ("dxle", 24), ("sw_efgeo", 25), ("q3", 26), ("p3", 27), ("q2", 28), ("p2", 29),
    ("kfn", 30), ("afn", 31), ("bfn", 32), ("sw_fngeo", 33), ("jmax", 34), ("tminclip", 35), ("tmaxclip", 36), ("tc1", 37), ("tc2", 38), ("tc1l", 39), ("tc2l", 40), ("tc1w", 41), ("tc2w", 42), ("tc1kfn", 43), ("gth0", 44), ("gthp", 45),
    ("gtha", 46), ("cth0", 47), ("cthp", 48), ("ctha", 49),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 50] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 50] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 50] = [
    "w", "l", "r", "c1", "c2", "trise", "isnoisy", "sw_et", "version", "revision", "scale", "shrink", "tmin", "tmax", "rthresh", "level",
    "tnom", "rsh", "lmin", "lmax", "wmin", "wmax", "xw", "xl", "dxle", "sw_efgeo", "q3", "p3", "q2", "p2", "kfn", "afn",
    "bfn", "sw_fngeo", "jmax", "tminclip", "tmaxclip", "tc1", "tc2", "tc1l", "tc2l", "tc1w", "tc2w", "tc1kfn", "gth0", "gthp", "gtha", "cth0",
    "cthp", "ctha",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 50] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 50] = [
    false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false,
    false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 50] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 50] = [
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None,
    Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 50] = [
    2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 3, 0, 0, 3, 2, 3, 2, 3, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3,
    3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 50] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
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
        29 => Some(ParameterBound { value: (1.0 - params.p27), label: "computed upper-bound expression" }),
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
    pub nodes: [usize; 3],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 50]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 1]>,
    pub(crate) ddt_state_previous: Box<[f64; 1]>,
    pub(crate) ddt_state_older: Box<[f64; 1]>,
    pub(crate) ddt_state_initialized: Box<[bool; 1]>,
    pub(crate) ddt_derivative_current: Box<[f64; 1]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 1]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 186]>,
    pub(crate) scalar_static_bool: Box<[bool; 62]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 3;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 50;
    pub const VARIABLE_COUNT: usize = 102;
    pub const DDT_STATE_COUNT: usize = 1;
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
            scalar_static_f64: boxed_zero_f64_array::<186>(),
            scalar_static_bool: boxed_zero_bool_array::<62>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'r2_et_cmc'", name));
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
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=if param_given[10]{1.0}else{0.0};
        self.scalar_static_f64[1]=p.p10;
        self.scalar_static_f64[2]=(if ((self.scalar_static_f64[0])!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_bool[0]=(!((self.scalar_static_f64[0])!=0.0));
        self.scalar_static_f64[3]=(if self.scalar_static_bool[0]{1.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[4]=if param_given[11]{1.0}else{0.0};
        self.scalar_static_f64[5]=p.p11;
        self.scalar_static_f64[6]=(0.01*self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=(1.0-self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=(if ((self.scalar_static_f64[4])!=0.0){self.scalar_static_f64[7]}else{0.0});
        self.scalar_static_bool[1]=(!((self.scalar_static_f64[4])!=0.0));
        self.scalar_static_f64[9]=(if self.scalar_static_bool[1]{1.0}else{self.scalar_static_f64[8]});
        self.scalar_static_f64[10]=(self.scalar_static_f64[3]*self.scalar_static_f64[9]);
        self.scalar_static_f64[11]=(self.scalar_static_f64[10]*1000000.0);
        self.scalar_static_f64[12]=p.p16;
        self.scalar_static_f64[13]=(273.15+self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=p.p5;
        self.scalar_static_f64[15]=p.p3;
        self.scalar_static_f64[16]=p.p4;
        self.scalar_static_bool[2]=(((self.scalar_static_f64[15])!=0.0)&&((self.scalar_static_f64[16])!=0.0));
        self.scalar_static_f64[17]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[18]=p.p23;
        self.scalar_static_f64[19]=(if ((self.scalar_static_f64[17])!=0.0){self.scalar_static_f64[18]}else{0.0});
        self.scalar_static_bool[3]=(((self.scalar_static_f64[15])!=0.0)||((self.scalar_static_f64[16])!=0.0));
        self.scalar_static_f64[20]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_bool[4]=(!((self.scalar_static_f64[17])!=0.0));
        self.scalar_static_bool[5]=(((self.scalar_static_f64[20])!=0.0)&&self.scalar_static_bool[4]);
        self.scalar_static_f64[21]=(self.scalar_static_f64[18]*0.5);
        self.scalar_static_f64[22]=(if self.scalar_static_bool[5]{self.scalar_static_f64[21]}else{self.scalar_static_f64[19]});
        self.scalar_static_bool[6]=(!((self.scalar_static_f64[20])!=0.0));
        self.scalar_static_bool[7]=(self.scalar_static_bool[4]&&self.scalar_static_bool[6]);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[7]{0.0}else{self.scalar_static_f64[22]});
        self.scalar_static_f64[24]=if param_given[1]{1.0}else{0.0};
        self.scalar_static_f64[25]=if param_given[2]{1.0}else{0.0};
        self.scalar_static_bool[8]=(((self.scalar_static_f64[24])!=0.0)&&((self.scalar_static_f64[25])!=0.0));
        self.scalar_static_f64[26]=if param_given[0]{1.0}else{0.0};
        self.scalar_static_bool[9]=(!((self.scalar_static_f64[26])!=0.0));
        self.scalar_static_bool[10]=(self.scalar_static_bool[8]&&self.scalar_static_bool[9]);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[28]=p.p2;
        self.scalar_static_bool[11]=(0.0==self.scalar_static_f64[28]);
        self.scalar_static_f64[29]=p.p1;
        self.scalar_static_bool[12]=(0.0==self.scalar_static_f64[29]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[11]||self.scalar_static_bool[12]);
        self.scalar_static_f64[30]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_bool[14]=(((self.scalar_static_f64[27])!=0.0)&&((self.scalar_static_f64[30])!=0.0));
        self.scalar_static_f64[31]=p.p0;
        self.scalar_static_f64[32]=(self.scalar_static_f64[11]*self.scalar_static_f64[31]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[14]{self.scalar_static_f64[32]}else{0.0});
        self.scalar_static_f64[34]=p.p22;
        self.scalar_static_f64[35]=(self.scalar_static_f64[33]+self.scalar_static_f64[34]);
        self.scalar_static_f64[36]=(if self.scalar_static_bool[14]{self.scalar_static_f64[35]}else{0.0});
        self.scalar_static_bool[15]=(!((self.scalar_static_f64[30])!=0.0));
        self.scalar_static_bool[16]=(((self.scalar_static_f64[27])!=0.0)&&self.scalar_static_bool[15]);
        self.scalar_static_f64[37]=(self.scalar_static_f64[11]*self.scalar_static_f64[29]);
        self.scalar_static_f64[38]=(if self.scalar_static_bool[16]{self.scalar_static_f64[37]}else{0.0});
        self.scalar_static_f64[39]=(self.scalar_static_f64[23]+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=(if self.scalar_static_bool[16]{self.scalar_static_f64[39]}else{0.0});
        self.scalar_static_bool[17]=(self.scalar_static_f64[40]>0.0);
        self.scalar_static_f64[41]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=(self.scalar_static_bool[16]&&((self.scalar_static_f64[41])!=0.0));
        self.scalar_static_f64[42]=p.p17;
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]/self.scalar_static_f64[28]);
        self.scalar_static_f64[44]=(self.scalar_static_f64[40]*self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=(if self.scalar_static_bool[18]{self.scalar_static_f64[44]}else{self.scalar_static_f64[36]});
        self.scalar_static_f64[46]=(self.scalar_static_f64[45]-self.scalar_static_f64[34]);
        self.scalar_static_f64[47]=(if self.scalar_static_bool[18]{self.scalar_static_f64[46]}else{self.scalar_static_f64[33]});
        self.scalar_static_f64[48]=(if self.scalar_static_bool[18]{self.scalar_static_f64[28]}else{0.0});
        self.scalar_static_bool[19]=(!((self.scalar_static_f64[41])!=0.0));
        self.scalar_static_bool[20]=(self.scalar_static_bool[16]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[49]=(if self.scalar_static_bool[20]{self.scalar_static_f64[32]}else{self.scalar_static_f64[47]});
        self.scalar_static_f64[50]=(self.scalar_static_f64[34]+self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=(if self.scalar_static_bool[20]{self.scalar_static_f64[50]}else{self.scalar_static_f64[45]});
        self.scalar_static_f64[52]=(if self.scalar_static_bool[20]{0.0}else{self.scalar_static_f64[48]});
        self.scalar_static_bool[21]=(!((self.scalar_static_f64[24])!=0.0));
        self.scalar_static_bool[22]=(((self.scalar_static_f64[25])!=0.0)&&self.scalar_static_bool[21]);
        self.scalar_static_f64[53]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[54]=(if self.scalar_static_bool[11]{1.0}else{0.0});
        self.scalar_static_bool[23]=(!((self.scalar_static_f64[27])!=0.0));
        self.scalar_static_bool[24]=(((self.scalar_static_f64[53])!=0.0)&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=(((self.scalar_static_f64[54])!=0.0)&&self.scalar_static_bool[24]);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[38]});
        self.scalar_static_f64[56]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[40]});
        self.scalar_static_f64[57]=(if self.scalar_static_bool[25]{self.scalar_static_f64[32]}else{self.scalar_static_f64[49]});
        self.scalar_static_f64[58]=(self.scalar_static_f64[34]+self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(if self.scalar_static_bool[25]{self.scalar_static_f64[58]}else{self.scalar_static_f64[51]});
        self.scalar_static_f64[60]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[52]});
        self.scalar_static_bool[26]=(0.0==self.scalar_static_f64[31]);
        self.scalar_static_f64[61]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=(!((self.scalar_static_f64[54])!=0.0));
        self.scalar_static_bool[28]=(self.scalar_static_bool[24]&&self.scalar_static_bool[27]);
        self.scalar_static_bool[29]=(((self.scalar_static_f64[61])!=0.0)&&self.scalar_static_bool[28]);
        self.scalar_static_f64[62]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[57]});
        self.scalar_static_f64[63]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[59]});
        self.scalar_static_f64[64]=(if self.scalar_static_bool[29]{self.scalar_static_f64[37]}else{self.scalar_static_f64[55]});
        self.scalar_static_f64[65]=(self.scalar_static_f64[23]+self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[29]{self.scalar_static_f64[65]}else{self.scalar_static_f64[56]});
        self.scalar_static_f64[67]=(if self.scalar_static_bool[29]{1e99}else{self.scalar_static_f64[60]});
        self.scalar_static_bool[30]=(!((self.scalar_static_f64[61])!=0.0));
        self.scalar_static_bool[31]=(self.scalar_static_bool[28]&&self.scalar_static_bool[30]);
        self.scalar_static_f64[68]=(if self.scalar_static_bool[31]{self.scalar_static_f64[32]}else{self.scalar_static_f64[62]});
        self.scalar_static_f64[69]=(self.scalar_static_f64[34]+self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(if self.scalar_static_bool[31]{self.scalar_static_f64[69]}else{self.scalar_static_f64[63]});
        self.scalar_static_bool[32]=(self.scalar_static_f64[70]>0.0);
        self.scalar_static_f64[71]=(if self.scalar_static_bool[32]{1.0}else{0.0});
        self.scalar_static_bool[33]=(self.scalar_static_bool[31]&&((self.scalar_static_f64[71])!=0.0));
        self.scalar_static_f64[72]=(self.scalar_static_f64[28]/self.scalar_static_f64[42]);
        self.scalar_static_f64[73]=(self.scalar_static_f64[70]*self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(if self.scalar_static_bool[33]{self.scalar_static_f64[73]}else{self.scalar_static_f64[66]});
        self.scalar_static_f64[75]=(self.scalar_static_f64[74]-self.scalar_static_f64[23]);
        self.scalar_static_f64[76]=(if self.scalar_static_bool[33]{self.scalar_static_f64[75]}else{self.scalar_static_f64[64]});
        self.scalar_static_f64[77]=(if self.scalar_static_bool[33]{self.scalar_static_f64[28]}else{self.scalar_static_f64[67]});
        self.scalar_static_bool[34]=(!((self.scalar_static_f64[71])!=0.0));
        self.scalar_static_bool[35]=(self.scalar_static_bool[31]&&self.scalar_static_bool[34]);
        self.scalar_static_f64[78]=(if self.scalar_static_bool[35]{self.scalar_static_f64[37]}else{self.scalar_static_f64[76]});
        self.scalar_static_f64[79]=(self.scalar_static_f64[23]+self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(if self.scalar_static_bool[35]{self.scalar_static_f64[79]}else{self.scalar_static_f64[74]});
        self.scalar_static_f64[81]=(if self.scalar_static_bool[35]{1e99}else{self.scalar_static_f64[77]});
        self.scalar_static_bool[36]=(!((self.scalar_static_f64[53])!=0.0));
        self.scalar_static_bool[37]=(self.scalar_static_bool[23]&&self.scalar_static_bool[36]);
        self.scalar_static_bool[38]=(((self.scalar_static_f64[61])!=0.0)&&self.scalar_static_bool[37]);
        self.scalar_static_f64[82]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[68]});
        self.scalar_static_f64[83]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[70]});
        self.scalar_static_f64[84]=(if self.scalar_static_bool[38]{self.scalar_static_f64[37]}else{self.scalar_static_f64[78]});
        self.scalar_static_f64[85]=(self.scalar_static_f64[23]+self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=(if self.scalar_static_bool[38]{self.scalar_static_f64[85]}else{self.scalar_static_f64[80]});
        self.scalar_static_f64[87]=(if self.scalar_static_bool[38]{1e99}else{self.scalar_static_f64[81]});
        self.scalar_static_f64[88]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_bool[39]=(self.scalar_static_bool[30]&&self.scalar_static_bool[37]);
        self.scalar_static_bool[40]=(((self.scalar_static_f64[88])!=0.0)&&self.scalar_static_bool[39]);
        self.scalar_static_f64[89]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[84]});
        self.scalar_static_f64[90]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[86]});
        self.scalar_static_f64[91]=(if self.scalar_static_bool[40]{self.scalar_static_f64[32]}else{self.scalar_static_f64[82]});
        self.scalar_static_f64[92]=(self.scalar_static_f64[34]+self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(if self.scalar_static_bool[40]{self.scalar_static_f64[92]}else{self.scalar_static_f64[83]});
        self.scalar_static_f64[94]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[87]});
        self.scalar_static_bool[41]=(!((self.scalar_static_f64[88])!=0.0));
        self.scalar_static_bool[42]=(self.scalar_static_bool[39]&&self.scalar_static_bool[41]);
        self.scalar_static_f64[95]=(if self.scalar_static_bool[42]{self.scalar_static_f64[32]}else{self.scalar_static_f64[91]});
        self.scalar_static_f64[96]=(self.scalar_static_f64[34]+self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(if self.scalar_static_bool[42]{self.scalar_static_f64[96]}else{self.scalar_static_f64[93]});
        self.scalar_static_f64[98]=(if self.scalar_static_bool[42]{self.scalar_static_f64[37]}else{self.scalar_static_f64[89]});
        self.scalar_static_f64[99]=(self.scalar_static_f64[23]+self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(if self.scalar_static_bool[42]{self.scalar_static_f64[99]}else{self.scalar_static_f64[90]});
        self.scalar_static_bool[43]=(self.scalar_static_f64[97]>0.0);
        self.scalar_static_f64[101]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_bool[44]=(self.scalar_static_f64[100]>0.0);
        self.scalar_static_f64[102]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_bool[45]=(self.scalar_static_bool[42]&&((self.scalar_static_f64[101])!=0.0));
        self.scalar_static_bool[46]=(((self.scalar_static_f64[102])!=0.0)&&self.scalar_static_bool[45]);
        self.scalar_static_f64[103]=(self.scalar_static_f64[100]/self.scalar_static_f64[97]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[42]*self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=(if self.scalar_static_bool[46]{self.scalar_static_f64[104]}else{self.scalar_static_f64[94]});
        self.scalar_static_bool[47]=(!((self.scalar_static_f64[102])!=0.0));
        self.scalar_static_bool[48]=(self.scalar_static_bool[45]&&self.scalar_static_bool[47]);
        self.scalar_static_f64[106]=(if self.scalar_static_bool[48]{0.0}else{self.scalar_static_f64[105]});
        self.scalar_static_bool[49]=(!((self.scalar_static_f64[101])!=0.0));
        self.scalar_static_bool[50]=(self.scalar_static_bool[42]&&self.scalar_static_bool[49]);
        self.scalar_static_f64[107]=(if self.scalar_static_bool[50]{1e99}else{self.scalar_static_f64[106]});
        self.scalar_static_f64[108]=p.p25;
        self.scalar_static_f64[109]=p.p24;
        self.scalar_static_f64[110]=(self.scalar_static_f64[100]+self.scalar_static_f64[109]);
        self.scalar_static_f64[111]=(if ((self.scalar_static_f64[108])!=0.0){self.scalar_static_f64[110]}else{0.0});
        self.scalar_static_bool[51]=(!((self.scalar_static_f64[108])!=0.0));
        self.scalar_static_f64[112]=(self.scalar_static_f64[98]+self.scalar_static_f64[109]);
        self.scalar_static_f64[113]=(if self.scalar_static_bool[51]{self.scalar_static_f64[112]}else{self.scalar_static_f64[111]});
        self.scalar_static_bool[52]=(self.scalar_static_f64[107]>0.0);
        self.scalar_static_f64[114]=p.p29;
        self.scalar_static_bool[53]=(self.scalar_static_f64[114]>0.0);
        self.scalar_static_f64[115]=p.p27;
        self.scalar_static_bool[54]=(self.scalar_static_f64[115]>0.0);
        self.scalar_static_bool[55]=(self.scalar_static_bool[53]||self.scalar_static_bool[54]);
        self.scalar_static_f64[116]=p.p37;
        self.scalar_static_f64[117]=p.p38;
        self.scalar_static_bool[56]=(((self.scalar_static_f64[17])!=0.0)&&((self.scalar_static_f64[102])!=0.0));
        self.scalar_static_f64[118]=p.p39;
        self.scalar_static_f64[119]=(self.scalar_static_f64[118]/self.scalar_static_f64[100]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[116]+self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(if self.scalar_static_bool[56]{self.scalar_static_f64[120]}else{self.scalar_static_f64[116]});
        self.scalar_static_f64[122]=p.p40;
        self.scalar_static_f64[123]=(self.scalar_static_f64[122]/self.scalar_static_f64[100]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[117]+self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(if self.scalar_static_bool[56]{self.scalar_static_f64[124]}else{self.scalar_static_f64[117]});
        self.scalar_static_bool[57]=(self.scalar_static_bool[4]&&((self.scalar_static_f64[102])!=0.0));
        self.scalar_static_bool[58]=(((self.scalar_static_f64[20])!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_f64[126]=(0.5*self.scalar_static_f64[118]);
        self.scalar_static_f64[127]=(self.scalar_static_f64[126]/self.scalar_static_f64[100]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[121]+self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(if self.scalar_static_bool[58]{self.scalar_static_f64[128]}else{self.scalar_static_f64[121]});
        self.scalar_static_f64[130]=(0.5*self.scalar_static_f64[122]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[130]/self.scalar_static_f64[100]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[125]+self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(if self.scalar_static_bool[58]{self.scalar_static_f64[132]}else{self.scalar_static_f64[125]});
        self.scalar_static_f64[134]=p.p41;
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]/self.scalar_static_f64[97]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[129]+self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(if ((self.scalar_static_f64[101])!=0.0){self.scalar_static_f64[136]}else{self.scalar_static_f64[129]});
        self.scalar_static_f64[138]=p.p42;
        self.scalar_static_f64[139]=(self.scalar_static_f64[138]/self.scalar_static_f64[97]);
        self.scalar_static_f64[140]=(self.scalar_static_f64[133]+self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=(if ((self.scalar_static_f64[101])!=0.0){self.scalar_static_f64[140]}else{self.scalar_static_f64[133]});
        self.scalar_static_f64[142]=(self.scalar_static_f64[95]+self.scalar_static_f64[98]);
        self.scalar_static_f64[143]=(2.0*self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(if ((self.scalar_static_f64[17])!=0.0){self.scalar_static_f64[143]}else{0.0});
        self.scalar_static_f64[145]=(self.scalar_static_f64[98]*2.0);
        self.scalar_static_f64[146]=(self.scalar_static_f64[95]+self.scalar_static_f64[145]);
        self.scalar_static_f64[147]=(if self.scalar_static_bool[5]{self.scalar_static_f64[146]}else{self.scalar_static_f64[144]});
        self.scalar_static_f64[148]=(if self.scalar_static_bool[7]{self.scalar_static_f64[145]}else{self.scalar_static_f64[147]});
        self.scalar_static_f64[149]=(self.scalar_static_f64[95]*self.scalar_static_f64[98]);
        self.scalar_static_f64[150]=p.p44;
        self.scalar_static_f64[151]=p.p45;
        self.scalar_static_f64[152]=(self.scalar_static_f64[148]*self.scalar_static_f64[151]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[150]+self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=p.p46;
        self.scalar_static_f64[155]=(self.scalar_static_f64[149]*self.scalar_static_f64[154]);
        self.scalar_static_f64[156]=(self.scalar_static_f64[153]+self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=p.p47;
        self.scalar_static_f64[158]=p.p48;
        self.scalar_static_f64[159]=(self.scalar_static_f64[148]*self.scalar_static_f64[158]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[157]+self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=p.p49;
        self.scalar_static_f64[162]=(self.scalar_static_f64[149]*self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[160]+self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=p.p7;
        self.scalar_static_f64[165]=p.p35;
        self.scalar_static_f64[166]=(1.0+self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=p.p36;
        self.scalar_static_f64[168]=(self.scalar_static_f64[167]-1.0);
        self.scalar_static_bool[59]=(self.scalar_static_bool[52]&&self.scalar_static_bool[55]);
        self.scalar_static_f64[169]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_f64[170]=p.p28;
        self.scalar_static_f64[171]=p.p26;
        self.scalar_static_f64[172]=(1.0-self.scalar_static_f64[114]);
        self.scalar_static_f64[173]=(self.scalar_static_f64[172]-self.scalar_static_f64[115]);
        self.scalar_static_bool[60]=(!((self.scalar_static_f64[169])!=0.0));
        self.scalar_static_bool[61]=(!((self.scalar_static_f64[164])!=0.0));
        self.scalar_static_f64[174]=(1.0/self.scalar_static_f64[113]);
        self.scalar_static_f64[175]=(-1.0/self.scalar_static_f64[113]);
        self.scalar_static_f64[176]=(if ((self.scalar_static_f64[169])!=0.0){self.scalar_static_f64[174]}else{0.0});
        self.scalar_static_f64[177]=(if ((self.scalar_static_f64[169])!=0.0){self.scalar_static_f64[175]}else{0.0});
        self.scalar_static_f64[178]=(self.scalar_static_f64[170]*self.scalar_static_f64[176]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[170]*self.scalar_static_f64[177]);
        self.scalar_static_f64[180]=(if ((self.scalar_static_f64[169])!=0.0){self.scalar_static_f64[178]}else{0.0});
        self.scalar_static_f64[181]=(if ((self.scalar_static_f64[169])!=0.0){self.scalar_static_f64[179]}else{0.0});
        self.scalar_static_f64[182]=(if ((self.scalar_static_f64[164])!=0.0){self.scalar_static_f64[156]}else{0.0});
        self.scalar_static_f64[183]=(if self.scalar_static_bool[61]{1000000.0}else{0.0});
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
        self.scalar_static_f64[184]=(temperature+self.scalar_static_f64[14]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[184]-273.15);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
