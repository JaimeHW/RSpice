#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 949],
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
            const DEFAULTS_0: [f64; 34] = [
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 1e-7, 0.0, 0.0, 1e-6, 0.0,
                1.0, 0.0, 1.0, 1e-12, 1e-6, 1e-6, 1e-12, 1e-6,
                1e-6, 1e-12, 1e-6, 1e-12, 1e-6, 1.0, 1.0, 1.0,
                1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (*ptr).values.as_mut_ptr().add(0), 34);
            {
                let params = &mut *ptr;
                params[34] = params[32];
                validate_parameter("MULT_FN", params[34], false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 914] = [
                0.0, 104.0, 1.0, 21.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0005, 0.0,
                2e-9, 3.9, 5e23, 1.0, 0.0, 1.0, 0.0, 1e26,
                2e-9, 2e-9, 5e25, 5e25, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 1.0,
                0.5, 0.0, 1.5, 1.5, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 1.0, 50.0, 1.0, 0.0, 0.0, 0.3,
                1.0, 0.0, 0.0, 1.0, 8.0, 0.01, 0.0, 0.0,
                0.05, 1.0, 10.0, 0.0, 1.0, 0.0, 10.0, 0.0,
                0.0, 0.0, 0.0, 2.0, 0.375, 0.063, 0.375, 0.063,
                0.375, 0.063, 3.1, 0.0, 0.0, 41.0, 41.0, 0.0,
                0.0, 0.0, 0.0, 1e-14, 0.0, 1.0, 0.1, 8.0,
                0.0, 0.0, 1e-15, 1e-15, 0.5, 0.5, 1.0, 1e-15,
                5e-16, 5e-16, 0.0, 0.3, 0.5, 0.4, 1e-15, 1e-15,
                1.0, 0.0, 8e22, 30000000.0, 0.0, 1.0, -1.0, 0.0005,
                0.0, 5e23, 0.0, 0.0006, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 4e24, 1500000000.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0,
                0.0, 0.0005, 0.0, 0.0, 0.0, 0.0, 2e-9, 3.9,
                4e23, 0.0, 1e-8, 1e24, 0.0, 1e-8, 1e-8, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1e26, 0.0,
                2e-9, 2e-9, 1e-8, 1e-8, 5e25, 5e25, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                0.03, 0.0, 0.0, 1e-8, 0.0, 0.0, 1e-8, 0.0,
                0.0, 1e-9, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0,
                0.0, 1.5, 1.5, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 50.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.3,
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 16.0, 1.0, 0.01, 1.0, 0.0, 0.0,
                0.5, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.05,
                1.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0,
                2.0, 0.375, 0.063, 0.375, 0.063, 0.375, 0.063, 3.1,
                0.0, 0.0, 41.0, 41.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.1, 1.0, 0.0, 0.0, 16.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.5,
                0.5, 1.0, 1e-15, 5e-16, 5e-16, 0.0, 0.3, 0.5,
                0.4, 1e-15, 1e-15, 1.0, 0.0, 8e22, 30000000.0, 0.0,
                1.0, 0.0, 2.0, 1e-8, 0.0, -1.0, 0.0005, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5e23,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1e-8, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                1.0, 8e22, 30000000.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, -1.0, 0.0, 0.0, 0.0, 0.0005, 0.0, 0.0,
                0.0, 5e23, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e26, 0.0, 0.0, 0.0, 5e25, 0.0, 0.0,
                0.0, 5e25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0,
                0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 50.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.3, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 8.0, 0.0, 0.0, 0.0, 0.01, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0,
                0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0,
                0.0, 1e-15, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0,
                0.0, 5e-16, 0.0, 0.0, 0.0, 5e-16, 0.0, 0.0,
                0.0, 1e-15, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 8e22, 0.0, 0.0,
                0.0, 30000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, -1.0, 0.0, 0.0, 0.0, 0.0005, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 5e23, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 8e22, 0.0, 0.0,
                0.0, 30000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 1e-6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1000.0, 21.0, 1000.0, 0.001,
                1e-9, 1e-9, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5,
                1.16, 1.16, 1.16, 1e-12, 1e-18, 1e-18, 100.0, 0.0001,
                0.0001, 1e-7, 1e-7, 100.0, 0.0001, 0.0001, 0.25, 0.25,
                0.25, 1e-12, 1e-18, 1e-18, 1000000000.0, 1000000000.0, 1000000000.0, -0.001,
                -0.001, -0.001, 10.0, 10.0, 10.0, 4.0, 4.0, 4.0,
                1.0, 1.0, 1.0, 1.0, -1.0, 0.1, 0.0, 0.5,
                0.0, 0.5, 0.001, 1e-9, 1e-9, 1.0, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.16, 1.16, 1.16, 1e-12, 1e-18,
                1e-18, 100.0, 0.0001, 0.0001, 1e-7, 1e-7, 100.0, 0.0001,
                0.0001, 0.25, 0.25, 0.25, 1e-12, 1e-18, 1e-18, 1000000000.0,
                1000000000.0, 1000000000.0, -0.001, -0.001, -0.001, 10.0, 10.0, 10.0,
                4.0, 4.0, 4.0, 1.0, 1.0, 1.0, 1.0, -1.0,
                0.1, 0.0, 0.5, 0.0, 0.5, 0.0, 2.5, 0.03,
                2.5, 0.03,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(35), 914);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 951] = [
    ("l", 0), ("w", 1), ("sa", 2), ("sb", 3), ("sd", 4), ("sca", 5), ("scb", 6), ("scc", 7), ("sc", 8), ("nf", 9), ("ngcon", 10), ("xgw", 11), ("nrs", 12), ("nrd", 13), ("jw", 14), ("delvto", 15),
    ("factuo", 16), ("delvtoedge", 17), ("factuoedge", 18), ("absource", 19), ("lssource", 20), ("lgsource", 21), ("abdrain", 22), ("lsdrain", 23), ("lgdrain", 24), ("as", 25), ("ps", 26), ("ad", 27), ("pd", 28), ("ifactor", 29), ("cfactor", 30), ("mult", 31),
    ("mult_i", 32), ("mult_q", 33), ("mult_fn", 34), ("trise", 35), ("dtemp", 35), ("level", 36), ("type", 37), ("tr", 38), ("tref", 38), ("swgeo", 39), ("swigate", 40), ("swimpact", 41), ("swgidl", 42), ("swjuncap", 43), ("swjunasym", 44), ("swnud", 45),
    ("swedge", 46), ("swdelvtac", 47), ("swqsat", 48), ("swqpart", 49), ("swign", 50), ("qmc", 51), ("swoprext", 52), ("swoppmos", 53), ("swopdrain", 54), ("dta", 55), ("vfb", 56), ("stvfb", 57), ("st2vfb", 58), ("tox", 59), ("epsrox", 60), ("neff", 61),
    ("gfacnud", 62), ("vsbnud", 63), ("dvsbnud", 64), ("dphib", 65), ("np", 66), ("toxov", 67), ("toxovd", 68), ("nov", 69), ("novd", 70), ("ct", 71), ("ctb", 72), ("ctg", 73), ("stct", 74), ("cf", 75), ("cfb", 76), ("cfd", 77),
    ("psce", 78), ("psceb", 79), ("psced", 80), ("betn", 81), ("stbet", 82), ("mue", 83), ("stmue", 84), ("themu", 85), ("stthemu", 86), ("cs", 87), ("stcs", 88), ("thecs", 89), ("stthecs", 90), ("xcor", 91), ("stxcor", 92), ("feta", 93),
    ("rs", 94), ("strs", 95), ("rsb", 96), ("rsg", 97), ("thesat", 98), ("stthesat", 99), ("thesatb", 100), ("thesatg", 101), ("thesatt", 102), ("ax", 103), ("alp", 104), ("alp1", 105), ("alp2", 106), ("vp", 107), ("a1", 108), ("a2", 109),
    ("sta2", 110), ("a3", 111), ("a4", 112), ("imaxii", 113), ("gco", 114), ("iginv", 115), ("igov", 116), ("igovd", 117), ("stig", 118), ("gc2", 119), ("gc3", 120), ("gc2ov", 121), ("gc3ov", 122), ("gc2ovd", 123), ("gc3ovd", 124), ("chib", 125),
    ("agidl", 126), ("agidld", 127), ("bgidl", 128), ("bgidld", 129), ("stbgidl", 130), ("stbgidld", 131), ("cgidl", 132), ("cgidld", 133), ("cox", 134), ("delvtac", 135), ("facneffac", 136), ("thesatac", 137), ("axac", 138), ("alpac", 139), ("alp1ac", 140), ("cgov", 141),
    ("cgovd", 142), ("fcgovacc", 143), ("fcgovaccd", 144), ("cgovaccg", 145), ("cgbov", 146), ("cinr", 147), ("cinrd", 148), ("dvfbinr", 149), ("fcinrdep", 150), ("fcinracc", 151), ("axinr", 152), ("cfr", 153), ("cfrd", 154), ("fnt", 155), ("fntexc", 156), ("nfa", 157),
    ("nfb", 158), ("nfc", 159), ("ef", 160), ("vfbedge", 161), ("stvfbedge", 162), ("dphibedge", 163), ("neffedge", 164), ("ctedge", 165), ("betnedge", 166), ("stbetedge", 167), ("psceedge", 168), ("pscebedge", 169), ("pscededge", 170), ("cfedge", 171), ("cfbedge", 172), ("cfdedge", 173),
    ("fntedge", 174), ("nfaedge", 175), ("nfbedge", 176), ("nfcedge", 177), ("efedge", 178), ("rg", 179), ("rse", 180), ("rde", 181), ("rbulk", 182), ("rwell", 183), ("rjuns", 184), ("rjund", 185), ("rth", 186), ("cth", 187), ("strth", 188), ("lvaro", 189),
    ("lvarl", 190), ("lvarw", 191), ("lap", 192), ("wvaro", 193), ("wvarl", 194), ("wvarw", 195), ("wot", 196), ("dlq", 197), ("dwq", 198), ("vfbo", 199), ("vfbl", 200), ("vfblexp", 201), ("vfbw", 202), ("vfblw", 203), ("stvfbo", 204), ("stvfbl", 205),
    ("stvfbw", 206), ("stvfblw", 207), ("st2vfbo", 208), ("toxo", 209), ("epsroxo", 210), ("nsubo", 211), ("nsubw", 212), ("wseg", 213), ("npck", 214), ("npckw", 215), ("wsegp", 216), ("lpck", 217), ("lpckw", 218), ("fol1", 219), ("fol2", 220), ("gfacnudo", 221),
    ("gfacnudl", 222), ("gfacnudlexp", 223), ("gfacnudw", 224), ("gfacnudlw", 225), ("vsbnudo", 226), ("dvsbnudo", 227), ("dphibo", 228), ("dphibl", 229), ("dphiblexp", 230), ("dphibw", 231), ("dphiblw", 232), ("npo", 233), ("npl", 234), ("toxovo", 235), ("toxovdo", 236), ("lov", 237),
    ("lovd", 238), ("novo", 239), ("novdo", 240), ("cto", 241), ("ctl", 242), ("ctlexp", 243), ("ctw", 244), ("ctlw", 245), ("ctbo", 246), ("ctgo", 247), ("stcto", 248), ("cfl", 249), ("cflexp", 250), ("cfw", 251), ("cfbo", 252), ("cfdo", 253),
    ("pscel", 254), ("pscelexp", 255), ("pscew", 256), ("pscebo", 257), ("pscedo", 258), ("uo", 259), ("fbet1", 260), ("fbet1w", 261), ("lp1", 262), ("lp1w", 263), ("fbet2", 264), ("lp2", 265), ("betw1", 266), ("betw2", 267), ("wbet", 268), ("stbeto", 269),
    ("stbetl", 270), ("stbetw", 271), ("stbetlw", 272), ("mueo", 273), ("muew", 274), ("stmueo", 275), ("themuo", 276), ("stthemuo", 277), ("cso", 278), ("csl", 279), ("cslexp", 280), ("csw", 281), ("cslw", 282), ("stcso", 283), ("thecso", 284), ("stthecso", 285),
    ("xcoro", 286), ("xcorl", 287), ("xcorw", 288), ("xcorlw", 289), ("stxcoro", 290), ("fetao", 291), ("rsw1", 292), ("rsw2", 293), ("strso", 294), ("rsbo", 295), ("rsgo", 296), ("thesato", 297), ("thesatl", 298), ("thesatlexp", 299), ("thesatw", 300), ("thesatlw", 301),
    ("stthesato", 302), ("stthesatl", 303), ("stthesatw", 304), ("stthesatlw", 305), ("thesatbo", 306), ("thesatgo", 307), ("thesatto", 308), ("axo", 309), ("axl", 310), ("alpl", 311), ("alplexp", 312), ("alpw", 313), ("alp1l1", 314), ("alp1lexp", 315), ("alp1l2", 316), ("alp1w", 317),
    ("alp2l1", 318), ("alp2lexp", 319), ("alp2l2", 320), ("alp2w", 321), ("vpo", 322), ("a1o", 323), ("a1l", 324), ("a1w", 325), ("a2o", 326), ("sta2o", 327), ("a3o", 328), ("a3l", 329), ("a3w", 330), ("a4o", 331), ("a4l", 332), ("a4w", 333),
    ("imaxiio", 334), ("gcoo", 335), ("iginvlw", 336), ("igovw", 337), ("igovdw", 338), ("stigo", 339), ("gc2o", 340), ("gc3o", 341), ("gc2ovo", 342), ("gc3ovo", 343), ("gc2ovdo", 344), ("gc3ovdo", 345), ("chibo", 346), ("agidlw", 347), ("agidldw", 348), ("bgidlo", 349),
    ("bgidldo", 350), ("stbgidlo", 351), ("stbgidldo", 352), ("cgidlo", 353), ("cgidldo", 354), ("delvtaco", 355), ("delvtacl", 356), ("delvtaclexp", 357), ("delvtacw", 358), ("delvtaclw", 359), ("facneffaco", 360), ("facneffacl", 361), ("facneffacw", 362), ("facneffaclw", 363), ("thesataco", 364), ("thesatacl", 365),
    ("thesataclexp", 366), ("thesatacw", 367), ("thesataclw", 368), ("axaco", 369), ("axacl", 370), ("alpacl", 371), ("alpaclexp", 372), ("alpacw", 373), ("alp1acl1", 374), ("alp1aclexp", 375), ("alp1acl2", 376), ("alp1acw", 377), ("fcgovacco", 378), ("fcgovaccdo", 379), ("cgovaccgo", 380), ("cgbovl", 381),
    ("cinrw", 382), ("cinrdw", 383), ("dvfbinro", 384), ("fcinrdepo", 385), ("fcinracco", 386), ("axinro", 387), ("cfrw", 388), ("cfrdw", 389), ("fnto", 390), ("fntexcl", 391), ("nfalw", 392), ("nfblw", 393), ("nfclw", 394), ("efo", 395), ("lintnoi", 396), ("alpnoi", 397),
    ("wedge", 398), ("wedgew", 399), ("vfbedgeo", 400), ("stvfbedgeo", 401), ("stvfbedgel", 402), ("stvfbedgew", 403), ("stvfbedgelw", 404), ("dphibedgeo", 405), ("dphibedgel", 406), ("dphibedgelexp", 407), ("dphibedgew", 408), ("dphibedgelw", 409), ("nsubedgeo", 410), ("nsubedgel", 411), ("nsubedgelexp", 412), ("nsubedgew", 413),
    ("nsubedgelw", 414), ("ctedgeo", 415), ("ctedgel", 416), ("ctedgelexp", 417), ("fbetedge", 418), ("lpedge", 419), ("betedgew", 420), ("stbetedgeo", 421), ("stbetedgel", 422), ("stbetedgew", 423), ("stbetedgelw", 424), ("psceedgel", 425), ("psceedgelexp", 426), ("psceedgew", 427), ("pscebedgeo", 428), ("pscededgeo", 429),
    ("cfedgel", 430), ("cfedgelexp", 431), ("cfedgew", 432), ("cfbedgeo", 433), ("cfdedgeo", 434), ("fntedgeo", 435), ("nfaedgelw", 436), ("nfbedgelw", 437), ("nfcedgelw", 438), ("efedgeo", 439), ("rgo", 440), ("rint", 441), ("rvpoly", 442), ("rshg", 443), ("dlsil", 444), ("rsh", 445),
    ("rshd", 446), ("rbulko", 447), ("rwello", 448), ("rjunso", 449), ("rjundo", 450), ("rtho", 451), ("rthw1", 452), ("rthw2", 453), ("rthlw", 454), ("ctho", 455), ("cthw1", 456), ("cthw2", 457), ("cthlw", 458), ("strtho", 459), ("povfb", 460), ("plvfb", 461),
    ("pwvfb", 462), ("plwvfb", 463), ("postvfb", 464), ("plstvfb", 465), ("pwstvfb", 466), ("plwstvfb", 467), ("poneff", 468), ("plneff", 469), ("pwneff", 470), ("plwneff", 471), ("pogfacnud", 472), ("plgfacnud", 473), ("pwgfacnud", 474), ("plwgfacnud", 475), ("povsbnud", 476), ("plvsbnud", 477),
    ("pwvsbnud", 478), ("plwvsbnud", 479), ("podphib", 480), ("pldphib", 481), ("pwdphib", 482), ("plwdphib", 483), ("ponp", 484), ("plnp", 485), ("pwnp", 486), ("plwnp", 487), ("ponov", 488), ("plnov", 489), ("pwnov", 490), ("plwnov", 491), ("ponovd", 492), ("plnovd", 493),
    ("pwnovd", 494), ("plwnovd", 495), ("poct", 496), ("plct", 497), ("pwct", 498), ("plwct", 499), ("poctb", 500), ("plctb", 501), ("pwctb", 502), ("plwctb", 503), ("poctg", 504), ("plctg", 505), ("pwctg", 506), ("plwctg", 507), ("postct", 508), ("plstct", 509),
    ("pwstct", 510), ("plwstct", 511), ("pocf", 512), ("plcf", 513), ("pwcf", 514), ("plwcf", 515), ("pocfb", 516), ("plcfb", 517), ("pwcfb", 518), ("plwcfb", 519), ("pocfd", 520), ("plcfd", 521), ("pwcfd", 522), ("plwcfd", 523), ("popsce", 524), ("plpsce", 525),
    ("pwpsce", 526), ("plwpsce", 527), ("popsceb", 528), ("plpsceb", 529), ("pwpsceb", 530), ("plwpsceb", 531), ("popsced", 532), ("plpsced", 533), ("pwpsced", 534), ("plwpsced", 535), ("pobetn", 536), ("plbetn", 537), ("pwbetn", 538), ("plwbetn", 539), ("postbet", 540), ("plstbet", 541),
    ("pwstbet", 542), ("plwstbet", 543), ("pomue", 544), ("plmue", 545), ("pwmue", 546), ("plwmue", 547), ("pothemu", 548), ("plthemu", 549), ("pwthemu", 550), ("plwthemu", 551), ("pocs", 552), ("plcs", 553), ("pwcs", 554), ("plwcs", 555), ("pothecs", 556), ("plthecs", 557),
    ("pwthecs", 558), ("plwthecs", 559), ("poxcor", 560), ("plxcor", 561), ("pwxcor", 562), ("plwxcor", 563), ("pors", 564), ("plrs", 565), ("pwrs", 566), ("plwrs", 567), ("postrs", 568), ("plstrs", 569), ("pwstrs", 570), ("plwstrs", 571), ("porsb", 572), ("plrsb", 573),
    ("pwrsb", 574), ("plwrsb", 575), ("porsg", 576), ("plrsg", 577), ("pwrsg", 578), ("plwrsg", 579), ("pothesat", 580), ("plthesat", 581), ("pwthesat", 582), ("plwthesat", 583), ("postthesat", 584), ("plstthesat", 585), ("pwstthesat", 586), ("plwstthesat", 587), ("pothesatb", 588), ("plthesatb", 589),
    ("pwthesatb", 590), ("plwthesatb", 591), ("pothesatg", 592), ("plthesatg", 593), ("pwthesatg", 594), ("plwthesatg", 595), ("poax", 596), ("plax", 597), ("pwax", 598), ("plwax", 599), ("poalp", 600), ("plalp", 601), ("pwalp", 602), ("plwalp", 603), ("poalp1", 604), ("plalp1", 605),
    ("pwalp1", 606), ("plwalp1", 607), ("poalp2", 608), ("plalp2", 609), ("pwalp2", 610), ("plwalp2", 611), ("poa1", 612), ("pla1", 613), ("pwa1", 614), ("plwa1", 615), ("posta2", 616), ("plsta2", 617), ("pwsta2", 618), ("plwsta2", 619), ("poa3", 620), ("pla3", 621),
    ("pwa3", 622), ("plwa3", 623), ("poa4", 624), ("pla4", 625), ("pwa4", 626), ("plwa4", 627), ("poiginv", 628), ("pliginv", 629), ("pwiginv", 630), ("plwiginv", 631), ("poigov", 632), ("pligov", 633), ("pwigov", 634), ("plwigov", 635), ("poigovd", 636), ("pligovd", 637),
    ("pwigovd", 638), ("plwigovd", 639), ("postig", 640), ("plstig", 641), ("pwstig", 642), ("plwstig", 643), ("poagidl", 644), ("plagidl", 645), ("pwagidl", 646), ("plwagidl", 647), ("poagidld", 648), ("plagidld", 649), ("pwagidld", 650), ("plwagidld", 651), ("postbgidl", 652), ("plstbgidl", 653),
    ("pwstbgidl", 654), ("plwstbgidl", 655), ("postbgidld", 656), ("plstbgidld", 657), ("pwstbgidld", 658), ("plwstbgidld", 659), ("pocox", 660), ("plcox", 661), ("pwcox", 662), ("plwcox", 663), ("podelvtac", 664), ("pldelvtac", 665), ("pwdelvtac", 666), ("plwdelvtac", 667), ("pofacneffac", 668), ("plfacneffac", 669),
    ("pwfacneffac", 670), ("plwfacneffac", 671), ("pothesatac", 672), ("plthesatac", 673), ("pwthesatac", 674), ("plwthesatac", 675), ("poaxac", 676), ("plaxac", 677), ("pwaxac", 678), ("plwaxac", 679), ("poalpac", 680), ("plalpac", 681), ("pwalpac", 682), ("plwalpac", 683), ("poalp1ac", 684), ("plalp1ac", 685),
    ("pwalp1ac", 686), ("plwalp1ac", 687), ("pocgov", 688), ("plcgov", 689), ("pwcgov", 690), ("plwcgov", 691), ("pocgovd", 692), ("plcgovd", 693), ("pwcgovd", 694), ("plwcgovd", 695), ("pocgbov", 696), ("plcgbov", 697), ("pwcgbov", 698), ("plwcgbov", 699), ("pocinr", 700), ("plcinr", 701),
    ("pwcinr", 702), ("plwcinr", 703), ("pocinrd", 704), ("plcinrd", 705), ("pwcinrd", 706), ("plwcinrd", 707), ("pocfr", 708), ("plcfr", 709), ("pwcfr", 710), ("plwcfr", 711), ("pocfrd", 712), ("plcfrd", 713), ("pwcfrd", 714), ("plwcfrd", 715), ("pofntexc", 716), ("plfntexc", 717),
    ("pwfntexc", 718), ("plwfntexc", 719), ("ponfa", 720), ("plnfa", 721), ("pwnfa", 722), ("plwnfa", 723), ("ponfb", 724), ("plnfb", 725), ("pwnfb", 726), ("plwnfb", 727), ("ponfc", 728), ("plnfc", 729), ("pwnfc", 730), ("plwnfc", 731), ("povfbedge", 732), ("plvfbedge", 733),
    ("pwvfbedge", 734), ("plwvfbedge", 735), ("postvfbedge", 736), ("plstvfbedge", 737), ("pwstvfbedge", 738), ("plwstvfbedge", 739), ("podphibedge", 740), ("pldphibedge", 741), ("pwdphibedge", 742), ("plwdphibedge", 743), ("poneffedge", 744), ("plneffedge", 745), ("pwneffedge", 746), ("plwneffedge", 747), ("poctedge", 748), ("plctedge", 749),
    ("pwctedge", 750), ("plwctedge", 751), ("pobetnedge", 752), ("plbetnedge", 753), ("pwbetnedge", 754), ("plwbetnedge", 755), ("postbetedge", 756), ("plstbetedge", 757), ("pwstbetedge", 758), ("plwstbetedge", 759), ("popsceedge", 760), ("plpsceedge", 761), ("pwpsceedge", 762), ("plwpsceedge", 763), ("popscebedge", 764), ("plpscebedge", 765),
    ("pwpscebedge", 766), ("plwpscebedge", 767), ("popscededge", 768), ("plpscededge", 769), ("pwpscededge", 770), ("plwpscededge", 771), ("pocfedge", 772), ("plcfedge", 773), ("pwcfedge", 774), ("plwcfedge", 775), ("pocfbedge", 776), ("plcfbedge", 777), ("pwcfbedge", 778), ("plwcfbedge", 779), ("pocfdedge", 780), ("plcfdedge", 781),
    ("pwcfdedge", 782), ("plwcfdedge", 783), ("ponfaedge", 784), ("plnfaedge", 785), ("pwnfaedge", 786), ("plwnfaedge", 787), ("ponfbedge", 788), ("plnfbedge", 789), ("pwnfbedge", 790), ("plwnfbedge", 791), ("ponfcedge", 792), ("plnfcedge", 793), ("pwnfcedge", 794), ("plwnfcedge", 795), ("porth", 796), ("plrth", 797),
    ("pwrth", 798), ("plwrth", 799), ("pocth", 800), ("plcth", 801), ("pwcth", 802), ("plwcth", 803), ("postrth", 804), ("plstrth", 805), ("pwstrth", 806), ("plwstrth", 807), ("saref", 808), ("sbref", 809), ("wlod", 810), ("kuo", 811), ("kvsat", 812), ("kvsatac", 813),
    ("tkuo", 814), ("lkuo", 815), ("wkuo", 816), ("pkuo", 817), ("llodkuo", 818), ("wlodkuo", 819), ("kvtho", 820), ("lkvtho", 821), ("wkvtho", 822), ("pkvtho", 823), ("llodvth", 824), ("wlodvth", 825), ("stetao", 826), ("lodetao", 827), ("scref", 828), ("web", 829),
    ("wec", 830), ("kvthoweo", 831), ("kvthowel", 832), ("kvthowew", 833), ("kvthowelw", 834), ("kuoweo", 835), ("kuowel", 836), ("kuowew", 837), ("kuowelw", 838), ("imax", 839), ("trj", 840), ("frev", 841), ("cjorbot", 842), ("cjorsti", 843), ("cjorgat", 844), ("vbirbot", 845),
    ("vbirsti", 846), ("vbirgat", 847), ("pbot", 848), ("psti", 849), ("pgat", 850), ("phigbot", 851), ("phigsti", 852), ("phiggat", 853), ("idsatrbot", 854), ("idsatrsti", 855), ("idsatrgat", 856), ("csrhbot", 857), ("csrhsti", 858), ("csrhgat", 859), ("xjunsti", 860), ("xjungat", 861),
    ("ctatbot", 862), ("ctatsti", 863), ("ctatgat", 864), ("mefftatbot", 865), ("mefftatsti", 866), ("mefftatgat", 867), ("cbbtbot", 868), ("cbbtsti", 869), ("cbbtgat", 870), ("fbbtrbot", 871), ("fbbtrsti", 872), ("fbbtrgat", 873), ("stfbbtbot", 874), ("stfbbtsti", 875), ("stfbbtgat", 876), ("vbrbot", 877),
    ("vbrsti", 878), ("vbrgat", 879), ("pbrbot", 880), ("pbrsti", 881), ("pbrgat", 882), ("fcjorgat2", 883), ("fvbirgat2", 884), ("fpgat2", 885), ("fphiggat2", 886), ("vtrgat", 887), ("anugat", 888), ("advbrgat", 889), ("bdvbrgat", 890), ("adbbtgat", 891), ("bdbbtgat", 892), ("cjorbotd", 893),
    ("cjorstid", 894), ("cjorgatd", 895), ("vbirbotd", 896), ("vbirstid", 897), ("vbirgatd", 898), ("pbotd", 899), ("pstid", 900), ("pgatd", 901), ("phigbotd", 902), ("phigstid", 903), ("phiggatd", 904), ("idsatrbotd", 905), ("idsatrstid", 906), ("idsatrgatd", 907), ("csrhbotd", 908), ("csrhstid", 909),
    ("csrhgatd", 910), ("xjunstid", 911), ("xjungatd", 912), ("ctatbotd", 913), ("ctatstid", 914), ("ctatgatd", 915), ("mefftatbotd", 916), ("mefftatstid", 917), ("mefftatgatd", 918), ("cbbtbotd", 919), ("cbbtstid", 920), ("cbbtgatd", 921), ("fbbtrbotd", 922), ("fbbtrstid", 923), ("fbbtrgatd", 924), ("stfbbtbotd", 925),
    ("stfbbtstid", 926), ("stfbbtgatd", 927), ("vbrbotd", 928), ("vbrstid", 929), ("vbrgatd", 930), ("pbrbotd", 931), ("pbrstid", 932), ("pbrgatd", 933), ("fcjorgat2d", 934), ("fvbirgat2d", 935), ("fpgat2d", 936), ("fphiggat2d", 937), ("vtrgatd", 938), ("anugatd", 939), ("advbrgatd", 940), ("bdvbrgatd", 941),
    ("adbbtgatd", 942), ("bdbbtgatd", 943), ("swjunexp", 944), ("vjunref", 945), ("fjunq", 946), ("vjunrefd", 947), ("fjunqd", 948),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 949] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
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

const PARAMETER_MAX_REFERENCES: [Option<usize>; 949] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
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

const PARAMETER_DISPLAY_NAMES: [&str; 949] = [
    "L", "W", "SA", "SB", "SD", "SCA", "SCB", "SCC", "SC", "NF", "NGCON", "XGW", "NRS", "NRD", "JW", "DELVTO",
    "FACTUO", "DELVTOEDGE", "FACTUOEDGE", "ABSOURCE", "LSSOURCE", "LGSOURCE", "ABDRAIN", "LSDRAIN", "LGDRAIN", "AS", "PS", "AD", "PD", "IFACTOR", "CFACTOR", "MULT",
    "MULT_I", "MULT_Q", "MULT_FN", "TRISE", "LEVEL", "TYPE", "TR", "SWGEO", "SWIGATE", "SWIMPACT", "SWGIDL", "SWJUNCAP", "SWJUNASYM", "SWNUD", "SWEDGE", "SWDELVTAC",
    "SWQSAT", "SWQPART", "SWIGN", "QMC", "SWOPREXT", "SWOPPMOS", "SWOPDRAIN", "DTA", "VFB", "STVFB", "ST2VFB", "TOX", "EPSROX", "NEFF", "GFACNUD", "VSBNUD",
    "DVSBNUD", "DPHIB", "NP", "TOXOV", "TOXOVD", "NOV", "NOVD", "CT", "CTB", "CTG", "STCT", "CF", "CFB", "CFD", "PSCE", "PSCEB",
    "PSCED", "BETN", "STBET", "MUE", "STMUE", "THEMU", "STTHEMU", "CS", "STCS", "THECS", "STTHECS", "XCOR", "STXCOR", "FETA", "RS", "STRS",
    "RSB", "RSG", "THESAT", "STTHESAT", "THESATB", "THESATG", "THESATT", "AX", "ALP", "ALP1", "ALP2", "VP", "A1", "A2", "STA2", "A3",
    "A4", "IMAXII", "GCO", "IGINV", "IGOV", "IGOVD", "STIG", "GC2", "GC3", "GC2OV", "GC3OV", "GC2OVD", "GC3OVD", "CHIB", "AGIDL", "AGIDLD",
    "BGIDL", "BGIDLD", "STBGIDL", "STBGIDLD", "CGIDL", "CGIDLD", "COX", "DELVTAC", "FACNEFFAC", "THESATAC", "AXAC", "ALPAC", "ALP1AC", "CGOV", "CGOVD", "FCGOVACC",
    "FCGOVACCD", "CGOVACCG", "CGBOV", "CINR", "CINRD", "DVFBINR", "FCINRDEP", "FCINRACC", "AXINR", "CFR", "CFRD", "FNT", "FNTEXC", "NFA", "NFB", "NFC",
    "EF", "VFBEDGE", "STVFBEDGE", "DPHIBEDGE", "NEFFEDGE", "CTEDGE", "BETNEDGE", "STBETEDGE", "PSCEEDGE", "PSCEBEDGE", "PSCEDEDGE", "CFEDGE", "CFBEDGE", "CFDEDGE", "FNTEDGE", "NFAEDGE",
    "NFBEDGE", "NFCEDGE", "EFEDGE", "RG", "RSE", "RDE", "RBULK", "RWELL", "RJUNS", "RJUND", "RTH", "CTH", "STRTH", "LVARO", "LVARL", "LVARW",
    "LAP", "WVARO", "WVARL", "WVARW", "WOT", "DLQ", "DWQ", "VFBO", "VFBL", "VFBLEXP", "VFBW", "VFBLW", "STVFBO", "STVFBL", "STVFBW", "STVFBLW",
    "ST2VFBO", "TOXO", "EPSROXO", "NSUBO", "NSUBW", "WSEG", "NPCK", "NPCKW", "WSEGP", "LPCK", "LPCKW", "FOL1", "FOL2", "GFACNUDO", "GFACNUDL", "GFACNUDLEXP",
    "GFACNUDW", "GFACNUDLW", "VSBNUDO", "DVSBNUDO", "DPHIBO", "DPHIBL", "DPHIBLEXP", "DPHIBW", "DPHIBLW", "NPO", "NPL", "TOXOVO", "TOXOVDO", "LOV", "LOVD", "NOVO",
    "NOVDO", "CTO", "CTL", "CTLEXP", "CTW", "CTLW", "CTBO", "CTGO", "STCTO", "CFL", "CFLEXP", "CFW", "CFBO", "CFDO", "PSCEL", "PSCELEXP",
    "PSCEW", "PSCEBO", "PSCEDO", "UO", "FBET1", "FBET1W", "LP1", "LP1W", "FBET2", "LP2", "BETW1", "BETW2", "WBET", "STBETO", "STBETL", "STBETW",
    "STBETLW", "MUEO", "MUEW", "STMUEO", "THEMUO", "STTHEMUO", "CSO", "CSL", "CSLEXP", "CSW", "CSLW", "STCSO", "THECSO", "STTHECSO", "XCORO", "XCORL",
    "XCORW", "XCORLW", "STXCORO", "FETAO", "RSW1", "RSW2", "STRSO", "RSBO", "RSGO", "THESATO", "THESATL", "THESATLEXP", "THESATW", "THESATLW", "STTHESATO", "STTHESATL",
    "STTHESATW", "STTHESATLW", "THESATBO", "THESATGO", "THESATTO", "AXO", "AXL", "ALPL", "ALPLEXP", "ALPW", "ALP1L1", "ALP1LEXP", "ALP1L2", "ALP1W", "ALP2L1", "ALP2LEXP",
    "ALP2L2", "ALP2W", "VPO", "A1O", "A1L", "A1W", "A2O", "STA2O", "A3O", "A3L", "A3W", "A4O", "A4L", "A4W", "IMAXIIO", "GCOO",
    "IGINVLW", "IGOVW", "IGOVDW", "STIGO", "GC2O", "GC3O", "GC2OVO", "GC3OVO", "GC2OVDO", "GC3OVDO", "CHIBO", "AGIDLW", "AGIDLDW", "BGIDLO", "BGIDLDO", "STBGIDLO",
    "STBGIDLDO", "CGIDLO", "CGIDLDO", "DELVTACO", "DELVTACL", "DELVTACLEXP", "DELVTACW", "DELVTACLW", "FACNEFFACO", "FACNEFFACL", "FACNEFFACW", "FACNEFFACLW", "THESATACO", "THESATACL", "THESATACLEXP", "THESATACW",
    "THESATACLW", "AXACO", "AXACL", "ALPACL", "ALPACLEXP", "ALPACW", "ALP1ACL1", "ALP1ACLEXP", "ALP1ACL2", "ALP1ACW", "FCGOVACCO", "FCGOVACCDO", "CGOVACCGO", "CGBOVL", "CINRW", "CINRDW",
    "DVFBINRO", "FCINRDEPO", "FCINRACCO", "AXINRO", "CFRW", "CFRDW", "FNTO", "FNTEXCL", "NFALW", "NFBLW", "NFCLW", "EFO", "LINTNOI", "ALPNOI", "WEDGE", "WEDGEW",
    "VFBEDGEO", "STVFBEDGEO", "STVFBEDGEL", "STVFBEDGEW", "STVFBEDGELW", "DPHIBEDGEO", "DPHIBEDGEL", "DPHIBEDGELEXP", "DPHIBEDGEW", "DPHIBEDGELW", "NSUBEDGEO", "NSUBEDGEL", "NSUBEDGELEXP", "NSUBEDGEW", "NSUBEDGELW", "CTEDGEO",
    "CTEDGEL", "CTEDGELEXP", "FBETEDGE", "LPEDGE", "BETEDGEW", "STBETEDGEO", "STBETEDGEL", "STBETEDGEW", "STBETEDGELW", "PSCEEDGEL", "PSCEEDGELEXP", "PSCEEDGEW", "PSCEBEDGEO", "PSCEDEDGEO", "CFEDGEL", "CFEDGELEXP",
    "CFEDGEW", "CFBEDGEO", "CFDEDGEO", "FNTEDGEO", "NFAEDGELW", "NFBEDGELW", "NFCEDGELW", "EFEDGEO", "RGO", "RINT", "RVPOLY", "RSHG", "DLSIL", "RSH", "RSHD", "RBULKO",
    "RWELLO", "RJUNSO", "RJUNDO", "RTHO", "RTHW1", "RTHW2", "RTHLW", "CTHO", "CTHW1", "CTHW2", "CTHLW", "STRTHO", "POVFB", "PLVFB", "PWVFB", "PLWVFB",
    "POSTVFB", "PLSTVFB", "PWSTVFB", "PLWSTVFB", "PONEFF", "PLNEFF", "PWNEFF", "PLWNEFF", "POGFACNUD", "PLGFACNUD", "PWGFACNUD", "PLWGFACNUD", "POVSBNUD", "PLVSBNUD", "PWVSBNUD", "PLWVSBNUD",
    "PODPHIB", "PLDPHIB", "PWDPHIB", "PLWDPHIB", "PONP", "PLNP", "PWNP", "PLWNP", "PONOV", "PLNOV", "PWNOV", "PLWNOV", "PONOVD", "PLNOVD", "PWNOVD", "PLWNOVD",
    "POCT", "PLCT", "PWCT", "PLWCT", "POCTB", "PLCTB", "PWCTB", "PLWCTB", "POCTG", "PLCTG", "PWCTG", "PLWCTG", "POSTCT", "PLSTCT", "PWSTCT", "PLWSTCT",
    "POCF", "PLCF", "PWCF", "PLWCF", "POCFB", "PLCFB", "PWCFB", "PLWCFB", "POCFD", "PLCFD", "PWCFD", "PLWCFD", "POPSCE", "PLPSCE", "PWPSCE", "PLWPSCE",
    "POPSCEB", "PLPSCEB", "PWPSCEB", "PLWPSCEB", "POPSCED", "PLPSCED", "PWPSCED", "PLWPSCED", "POBETN", "PLBETN", "PWBETN", "PLWBETN", "POSTBET", "PLSTBET", "PWSTBET", "PLWSTBET",
    "POMUE", "PLMUE", "PWMUE", "PLWMUE", "POTHEMU", "PLTHEMU", "PWTHEMU", "PLWTHEMU", "POCS", "PLCS", "PWCS", "PLWCS", "POTHECS", "PLTHECS", "PWTHECS", "PLWTHECS",
    "POXCOR", "PLXCOR", "PWXCOR", "PLWXCOR", "PORS", "PLRS", "PWRS", "PLWRS", "POSTRS", "PLSTRS", "PWSTRS", "PLWSTRS", "PORSB", "PLRSB", "PWRSB", "PLWRSB",
    "PORSG", "PLRSG", "PWRSG", "PLWRSG", "POTHESAT", "PLTHESAT", "PWTHESAT", "PLWTHESAT", "POSTTHESAT", "PLSTTHESAT", "PWSTTHESAT", "PLWSTTHESAT", "POTHESATB", "PLTHESATB", "PWTHESATB", "PLWTHESATB",
    "POTHESATG", "PLTHESATG", "PWTHESATG", "PLWTHESATG", "POAX", "PLAX", "PWAX", "PLWAX", "POALP", "PLALP", "PWALP", "PLWALP", "POALP1", "PLALP1", "PWALP1", "PLWALP1",
    "POALP2", "PLALP2", "PWALP2", "PLWALP2", "POA1", "PLA1", "PWA1", "PLWA1", "POSTA2", "PLSTA2", "PWSTA2", "PLWSTA2", "POA3", "PLA3", "PWA3", "PLWA3",
    "POA4", "PLA4", "PWA4", "PLWA4", "POIGINV", "PLIGINV", "PWIGINV", "PLWIGINV", "POIGOV", "PLIGOV", "PWIGOV", "PLWIGOV", "POIGOVD", "PLIGOVD", "PWIGOVD", "PLWIGOVD",
    "POSTIG", "PLSTIG", "PWSTIG", "PLWSTIG", "POAGIDL", "PLAGIDL", "PWAGIDL", "PLWAGIDL", "POAGIDLD", "PLAGIDLD", "PWAGIDLD", "PLWAGIDLD", "POSTBGIDL", "PLSTBGIDL", "PWSTBGIDL", "PLWSTBGIDL",
    "POSTBGIDLD", "PLSTBGIDLD", "PWSTBGIDLD", "PLWSTBGIDLD", "POCOX", "PLCOX", "PWCOX", "PLWCOX", "PODELVTAC", "PLDELVTAC", "PWDELVTAC", "PLWDELVTAC", "POFACNEFFAC", "PLFACNEFFAC", "PWFACNEFFAC", "PLWFACNEFFAC",
    "POTHESATAC", "PLTHESATAC", "PWTHESATAC", "PLWTHESATAC", "POAXAC", "PLAXAC", "PWAXAC", "PLWAXAC", "POALPAC", "PLALPAC", "PWALPAC", "PLWALPAC", "POALP1AC", "PLALP1AC", "PWALP1AC", "PLWALP1AC",
    "POCGOV", "PLCGOV", "PWCGOV", "PLWCGOV", "POCGOVD", "PLCGOVD", "PWCGOVD", "PLWCGOVD", "POCGBOV", "PLCGBOV", "PWCGBOV", "PLWCGBOV", "POCINR", "PLCINR", "PWCINR", "PLWCINR",
    "POCINRD", "PLCINRD", "PWCINRD", "PLWCINRD", "POCFR", "PLCFR", "PWCFR", "PLWCFR", "POCFRD", "PLCFRD", "PWCFRD", "PLWCFRD", "POFNTEXC", "PLFNTEXC", "PWFNTEXC", "PLWFNTEXC",
    "PONFA", "PLNFA", "PWNFA", "PLWNFA", "PONFB", "PLNFB", "PWNFB", "PLWNFB", "PONFC", "PLNFC", "PWNFC", "PLWNFC", "POVFBEDGE", "PLVFBEDGE", "PWVFBEDGE", "PLWVFBEDGE",
    "POSTVFBEDGE", "PLSTVFBEDGE", "PWSTVFBEDGE", "PLWSTVFBEDGE", "PODPHIBEDGE", "PLDPHIBEDGE", "PWDPHIBEDGE", "PLWDPHIBEDGE", "PONEFFEDGE", "PLNEFFEDGE", "PWNEFFEDGE", "PLWNEFFEDGE", "POCTEDGE", "PLCTEDGE", "PWCTEDGE", "PLWCTEDGE",
    "POBETNEDGE", "PLBETNEDGE", "PWBETNEDGE", "PLWBETNEDGE", "POSTBETEDGE", "PLSTBETEDGE", "PWSTBETEDGE", "PLWSTBETEDGE", "POPSCEEDGE", "PLPSCEEDGE", "PWPSCEEDGE", "PLWPSCEEDGE", "POPSCEBEDGE", "PLPSCEBEDGE", "PWPSCEBEDGE", "PLWPSCEBEDGE",
    "POPSCEDEDGE", "PLPSCEDEDGE", "PWPSCEDEDGE", "PLWPSCEDEDGE", "POCFEDGE", "PLCFEDGE", "PWCFEDGE", "PLWCFEDGE", "POCFBEDGE", "PLCFBEDGE", "PWCFBEDGE", "PLWCFBEDGE", "POCFDEDGE", "PLCFDEDGE", "PWCFDEDGE", "PLWCFDEDGE",
    "PONFAEDGE", "PLNFAEDGE", "PWNFAEDGE", "PLWNFAEDGE", "PONFBEDGE", "PLNFBEDGE", "PWNFBEDGE", "PLWNFBEDGE", "PONFCEDGE", "PLNFCEDGE", "PWNFCEDGE", "PLWNFCEDGE", "PORTH", "PLRTH", "PWRTH", "PLWRTH",
    "POCTH", "PLCTH", "PWCTH", "PLWCTH", "POSTRTH", "PLSTRTH", "PWSTRTH", "PLWSTRTH", "SAREF", "SBREF", "WLOD", "KUO", "KVSAT", "KVSATAC", "TKUO", "LKUO",
    "WKUO", "PKUO", "LLODKUO", "WLODKUO", "KVTHO", "LKVTHO", "WKVTHO", "PKVTHO", "LLODVTH", "WLODVTH", "STETAO", "LODETAO", "SCREF", "WEB", "WEC", "KVTHOWEO",
    "KVTHOWEL", "KVTHOWEW", "KVTHOWELW", "KUOWEO", "KUOWEL", "KUOWEW", "KUOWELW", "IMAX", "TRJ", "FREV", "CJORBOT", "CJORSTI", "CJORGAT", "VBIRBOT", "VBIRSTI", "VBIRGAT",
    "PBOT", "PSTI", "PGAT", "PHIGBOT", "PHIGSTI", "PHIGGAT", "IDSATRBOT", "IDSATRSTI", "IDSATRGAT", "CSRHBOT", "CSRHSTI", "CSRHGAT", "XJUNSTI", "XJUNGAT", "CTATBOT", "CTATSTI",
    "CTATGAT", "MEFFTATBOT", "MEFFTATSTI", "MEFFTATGAT", "CBBTBOT", "CBBTSTI", "CBBTGAT", "FBBTRBOT", "FBBTRSTI", "FBBTRGAT", "STFBBTBOT", "STFBBTSTI", "STFBBTGAT", "VBRBOT", "VBRSTI", "VBRGAT",
    "PBRBOT", "PBRSTI", "PBRGAT", "FCJORGAT2", "FVBIRGAT2", "FPGAT2", "FPHIGGAT2", "VTRGAT", "ANUGAT", "ADVBRGAT", "BDVBRGAT", "ADBBTGAT", "BDBBTGAT", "CJORBOTD", "CJORSTID", "CJORGATD",
    "VBIRBOTD", "VBIRSTID", "VBIRGATD", "PBOTD", "PSTID", "PGATD", "PHIGBOTD", "PHIGSTID", "PHIGGATD", "IDSATRBOTD", "IDSATRSTID", "IDSATRGATD", "CSRHBOTD", "CSRHSTID", "CSRHGATD", "XJUNSTID",
    "XJUNGATD", "CTATBOTD", "CTATSTID", "CTATGATD", "MEFFTATBOTD", "MEFFTATSTID", "MEFFTATGATD", "CBBTBOTD", "CBBTSTID", "CBBTGATD", "FBBTRBOTD", "FBBTRSTID", "FBBTRGATD", "STFBBTBOTD", "STFBBTSTID", "STFBBTGATD",
    "VBRBOTD", "VBRSTID", "VBRGATD", "PBRBOTD", "PBRSTID", "PBRGATD", "FCJORGAT2D", "FVBIRGAT2D", "FPGAT2D", "FPHIGGAT2D", "VTRGATD", "ANUGATD", "ADVBRGATD", "BDVBRGATD", "ADBBTGATD", "BDBBTGATD",
    "SWJUNEXP", "VJUNREF", "FJUNQ", "VJUNREFD", "FJUNQD",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 949] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 949] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 949] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e20, label: "1e20" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e23, label: "1e23" }),
    Some(ParameterBound { value: 1e23, label: "1e23" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None,
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }),
    Some(ParameterBound { value: -0.5, label: "-0.5" }), None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e20, label: "1e20" }), None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }),
    Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }),
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }),
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 949] = [
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }), Some(ParameterBound { value: 1e27, label: "1e27" }), None,
    Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }),
    Some(ParameterBound { value: 1e27, label: "1e27" }), None, None, None, None, None, Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }),
    None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 949] = [
    2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    2, 0, 2, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0,
    0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2,
    2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2,
    2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0,
    0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    2, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2,
    0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2,
    2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 949] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

pub struct Instance {
    pub nodes: [usize; 13],
    pub branches: [usize; 7],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 949]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<12, 0>>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["noi", "gp", "si", "di", "bp", "bi", "bs", "bd"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 949;
    pub const VARIABLE_COUNT: usize = 2932;
    pub const DDT_STATE_COUNT: usize = 12;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "eb789635376f4c2431a10211a02f8b359d4fad057ad602b1663b63c4907cc036";
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
        }
    }

    pub(crate) fn capture_rollback_state(&self) -> GeneratedVerilogARollbackState {
        let mut values = Vec::with_capacity(60);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(12);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 60);
        debug_assert_eq!(state.flags.len(), 12);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'PSP104TVA'", name));
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
        let _ = invalidates_caches;
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
