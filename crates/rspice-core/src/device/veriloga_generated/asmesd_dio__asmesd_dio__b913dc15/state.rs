#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 52],
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
            const DEFAULTS_0: [f64; 46] = [
                1e-17, 1.0, 0.0, 5.0, 10.0, 10.0, 0.0, 0.0,
                0.01, 1.11, 0.0, 10.0, 1e-5, 0.0, 1e-6, 0.0,
                0.0, 0.75, 0.33, 0.0, 0.001, 1.11, 3.0, 0.5,
                0.5, 25.0, 1000.0, 0.0, 1.0, 1.0, 2.0, 0.0,
                1.0, 0.0005, 0.0005, 5e-6, 1e-7, 0.0, 0.0, 2.0,
                100.0, 0.0, 1e-5, 1.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 46);
            {
                let params = &mut *ptr;
                params[46] = 0.001;
                validate_parameter("minr", params[46], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                5.0, 100.0, 2.0, 100.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(47), 5);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 53] = [
    ("is", 0), ("nf", 1), ("isr", 2), ("ntr", 3), ("vtr", 4), ("bvr", 5), ("xbvr", 6), ("xjbv", 7), ("ther", 8), ("theexp", 9), ("xtheexp", 10), ("nbv", 11), ("rb", 12), ("rbe", 13), ("re", 14), ("ree", 15),
    ("cje", 16), ("vje", 17), ("mje", 18), ("tf", 19), ("qtt0", 20), ("vtt0", 20), ("eg", 21), ("xti", 22), ("xtir", 23), ("fc", 24), ("tnom", 25), ("tfail", 26), ("kf", 27), ("af", 28), ("type", 29), ("shmod", 30),
    ("extmod", 31), ("rbmod", 32), ("rth0", 33), ("cth0", 34), ("rth1", 35), ("cth1", 36), ("arb", 37), ("are", 38), ("texp", 39), ("vtf0", 40), ("atff", 41), ("l", 42), ("n", 43), ("qexp", 44), ("dtemp", 45), ("minr", 46),
    ("ijbv", 47), ("vsatb", 48), ("mexp", 49), ("vsate", 50), ("mexpe", 51),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 52] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 52] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 52] = [
    "is", "nf", "isr", "ntr", "vtr", "bvr", "xbvr", "xjbv", "ther", "theexp", "xtheexp", "nbv", "rb", "rbe", "re", "ree",
    "cje", "vje", "mje", "tf", "qtt0", "eg", "xti", "xtir", "fc", "tnom", "tfail", "kf", "af", "type", "shmod", "extmod",
    "rbmod", "rth0", "cth0", "rth1", "cth1", "arb", "are", "texp", "vtf0", "atff", "l", "n", "qexp", "dtemp", "minr", "ijbv",
    "vsatb", "mexp", "vsate", "mexpe",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 52] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 52] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true,
    true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 52] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -20.0, label: "-20.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -40.0, label: "-40.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 52] = [
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 500.0, label: "500.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 500.0, label: "500.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 20.0, label: "20.0" }), Some(ParameterBound { value: 20.0, label: "20.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 125.0, label: "125.0" }), None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 52] = [
    0, 3, 0, 1, 2, 2, 3, 2, 2, 2, 3, 1, 2, 2, 2, 2, 2, 2, 3, 2, 3, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0,
    0, 3, 2, 3, 2, 2, 2, 3, 3, 2, 2, 2, 2, 3, 2, 2, 3, 3, 3, 3,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 52] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
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
    pub nodes: [usize; 7],
    pub branches: [usize; 7],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 52]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<6, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static: Box<ScalarStaticState<90, 21>>,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 7;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["bi", "ei", "dt1", "tt"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 52;
    pub const VARIABLE_COUNT: usize = 75;
    pub const DDT_STATE_COUNT: usize = 6;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "4675332b1191069d2dd27a75c90569b47278e59443c49d0c44e8295e9f552a6b";
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
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(30);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(6);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 30);
        debug_assert_eq!(state.flags.len(), 6);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmesd_dio'", name));
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
        self.scalar_static.f64_values[0]=p[45];
        self.scalar_static.f64_values[1]=p[43];
        self.scalar_static.f64_values[2]=p[42];
        self.scalar_static.f64_values[3]=(self.scalar_static.f64_values[1]*self.scalar_static.f64_values[2]);
        self.scalar_static.f64_values[4]=p[25];
        self.scalar_static.f64_values[5]=(273.15+self.scalar_static.f64_values[4]);
        self.scalar_static.f64_values[6]=p[22];
        self.scalar_static.f64_values[7]=p[21];
        self.scalar_static.f64_values[8]=p[23];
        self.scalar_static.f64_values[9]=p[0];
        self.scalar_static.f64_values[10]=p[2];
        self.scalar_static.f64_values[11]=p[47];
        self.scalar_static.f64_values[12]=p[7];
        self.scalar_static.f64_values[13]=p[5];
        self.scalar_static.f64_values[14]=p[6];
        self.scalar_static.f64_values[15]=p[9];
        self.scalar_static.f64_values[16]=p[10];
        self.scalar_static.f64_values[17]=p[16];
        self.scalar_static.f64_values[18]=(self.scalar_static.f64_values[5]/300.15);
        self.scalar_static.f64_values[19]=p[17];
        self.scalar_static.f64_values[20]=p[18];
        self.scalar_static.f64_values[21]=(self.scalar_static.f64_values[5]-300.15);
        self.scalar_static.f64_values[22]=(0.0004*self.scalar_static.f64_values[21]);
        self.scalar_static.f64_values[23]=p[29];
        self.scalar_static.f64_values[24]=p[1];
        self.scalar_static.f64_values[25]=p[11];
        self.scalar_static.f64_values[26]=p[8];
        self.scalar_static.f64_values[27]=p[4];
        self.scalar_static.f64_values[28]=p[3];
        self.scalar_static.f64_values[29]=p[48];
        self.scalar_static.f64_values[30]=p[49];
        self.scalar_static.f64_values[31]=p[50];
        self.scalar_static.f64_values[32]=p[51];
        self.scalar_static.f64_values[33]=p[12];
        self.scalar_static.f64_values[34]=p[37];
        self.scalar_static.f64_values[35]=(1.0/self.scalar_static.f64_values[30]);
        self.scalar_static.f64_values[36]=p[14];
        self.scalar_static.f64_values[37]=p[38];
        self.scalar_static.f64_values[38]=(1.0/self.scalar_static.f64_values[32]);
        self.scalar_static.f64_values[39]=p[31];
        self.scalar_static.bool_values[0]=(1.0==self.scalar_static.f64_values[39]);
        self.scalar_static.f64_values[40]=(if self.scalar_static.bool_values[0]{1.0}else{0.0});
        self.scalar_static.f64_values[41]=p[13];
        self.scalar_static.f64_values[42]=p[15];
        self.scalar_static.f64_values[43]=p[40];
        self.scalar_static.f64_values[44]=p[39];
        self.scalar_static.f64_values[45]=(1.0/self.scalar_static.f64_values[44]);
        self.scalar_static.f64_values[46]=p[19];
        self.scalar_static.f64_values[47]=p[41];
        self.scalar_static.f64_values[48]=p[32];
        self.scalar_static.bool_values[1]=(1.0==self.scalar_static.f64_values[48]);
        self.scalar_static.f64_values[49]=(if self.scalar_static.bool_values[1]{1.0}else{0.0});
        self.scalar_static.f64_values[50]=p[20];
        self.scalar_static.f64_values[51]=p[44];
        self.scalar_static.f64_values[52]=p[24];
        self.scalar_static.f64_values[53]=(-1.0-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[54]=(1.0-self.scalar_static.f64_values[52]);
        self.scalar_static.f64_values[55]=(self.scalar_static.f64_values[54]).ln();
        self.scalar_static.f64_values[56]=(self.scalar_static.f64_values[53]*self.scalar_static.f64_values[55]);
        self.scalar_static.f64_values[57]=(self.scalar_static.f64_values[56]).exp();
        self.scalar_static.f64_values[58]=(1.0-self.scalar_static.f64_values[20]);
        self.scalar_static.f64_values[59]=(self.scalar_static.f64_values[20]*0.5);
        self.scalar_static.f64_values[60]=p[30];
        self.scalar_static.bool_values[2]=(1.0==self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[61]=p[33];
        self.scalar_static.bool_values[3]=(self.scalar_static.f64_values[61]>0.0);
        self.scalar_static.bool_values[4]=(self.scalar_static.bool_values[2]&&self.scalar_static.bool_values[3]);
        self.scalar_static.f64_values[62]=(if self.scalar_static.bool_values[4]{1.0}else{0.0});
        self.scalar_static.bool_values[5]=(self.scalar_static.f64_values[60]==2.0);
        self.scalar_static.bool_values[6]=(self.scalar_static.bool_values[3]&&self.scalar_static.bool_values[5]);
        self.scalar_static.f64_values[63]=p[35];
        self.scalar_static.bool_values[7]=(self.scalar_static.f64_values[63]>0.0);
        self.scalar_static.bool_values[8]=(self.scalar_static.bool_values[6]&&self.scalar_static.bool_values[7]);
        self.scalar_static.f64_values[64]=(if self.scalar_static.bool_values[8]{1.0}else{0.0});
        self.scalar_static.bool_values[9]=(-1.0==self.scalar_static.f64_values[60]);
        self.scalar_static.f64_values[65]=(if self.scalar_static.bool_values[9]{1.0}else{0.0});
        self.scalar_static.f64_values[66]=(self.scalar_static.f64_values[39]*self.scalar_static.f64_values[41]);
        self.scalar_static.f64_values[67]=(self.scalar_static.f64_values[33]+self.scalar_static.f64_values[66]);
        self.scalar_static.f64_values[68]=(self.scalar_static.f64_values[67]/self.scalar_static.f64_values[3]);
        self.scalar_static.f64_values[69]=(self.scalar_static.f64_values[39]*self.scalar_static.f64_values[42]);
        self.scalar_static.f64_values[70]=(self.scalar_static.f64_values[36]+self.scalar_static.f64_values[69]);
        self.scalar_static.f64_values[71]=(self.scalar_static.f64_values[70]/self.scalar_static.f64_values[3]);
        self.scalar_static.bool_values[10]=(self.scalar_static.f64_values[68]>0.0);
        self.scalar_static.f64_values[72]=p[46];
        self.scalar_static.bool_values[11]=(self.scalar_static.f64_values[68]>=self.scalar_static.f64_values[72]);
        self.scalar_static.bool_values[12]=(self.scalar_static.bool_values[10]&&self.scalar_static.bool_values[11]);
        self.scalar_static.f64_values[73]=(if self.scalar_static.bool_values[12]{1.0}else{0.0});
        self.scalar_static.bool_values[13]=(self.scalar_static.f64_values[71]>0.0);
        self.scalar_static.bool_values[14]=(self.scalar_static.f64_values[71]>=self.scalar_static.f64_values[72]);
        self.scalar_static.bool_values[15]=(self.scalar_static.bool_values[13]&&self.scalar_static.bool_values[14]);
        self.scalar_static.f64_values[74]=(if self.scalar_static.bool_values[15]{1.0}else{0.0});
        self.scalar_static.f64_values[75]=p[34];
        self.scalar_static.bool_values[16]=(!((self.scalar_static.f64_values[62])!=0.0));
        self.scalar_static.bool_values[17]=(((self.scalar_static.f64_values[64])!=0.0)&&self.scalar_static.bool_values[16]);
        self.scalar_static.f64_values[76]=p[36];
        self.scalar_static.bool_values[18]=(!((self.scalar_static.f64_values[64])!=0.0));
        self.scalar_static.bool_values[19]=(self.scalar_static.bool_values[16]&&self.scalar_static.bool_values[18]);
        self.scalar_static.bool_values[20]=(((self.scalar_static.f64_values[65])!=0.0)&&self.scalar_static.bool_values[19]);
        self.scalar_static.f64_values[77]=(-self.scalar_static.f64_values[23]);
        self.scalar_static.f64_values[78]=(self.scalar_static.f64_values[27]*self.scalar_static.f64_values[77]);
        self.scalar_static.f64_values[79]=(self.scalar_static.f64_values[23]*self.scalar_static.f64_values[27]);
        self.scalar_static.f64_values[80]=(self.scalar_static.f64_values[23]*self.scalar_static.f64_values[59]);
        self.scalar_static.f64_values[81]=(self.scalar_static.f64_values[59]*self.scalar_static.f64_values[77]);
        self.scalar_static.f64_values[82]=(if ((self.scalar_static.f64_values[49])!=0.0){1.0}else{0.0});
        self.scalar_static.f64_values[83]=(1.0/self.scalar_static.f64_values[61]);
        self.scalar_static.f64_values[84]=(if ((self.scalar_static.f64_values[62])!=0.0){self.scalar_static.f64_values[83]}else{0.0});
        self.scalar_static.f64_values[85]=(-1.0/self.scalar_static.f64_values[61]);
        self.scalar_static.f64_values[86]=(if self.scalar_static.bool_values[17]{self.scalar_static.f64_values[83]}else{0.0});
        self.scalar_static.f64_values[87]=(if self.scalar_static.bool_values[17]{self.scalar_static.f64_values[85]}else{0.0});
        self.scalar_static.f64_values[88]=(1.0/self.scalar_static.f64_values[63]);
        self.scalar_static.f64_values[89]=(if self.scalar_static.bool_values[17]{self.scalar_static.f64_values[88]}else{0.0});
        self.scalar_static.instance_dirty = false;
    }
}
