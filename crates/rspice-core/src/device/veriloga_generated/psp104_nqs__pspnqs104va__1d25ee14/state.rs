#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub values: [f64; 932],
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
            const DEFAULTS_1: [f64; 897] = [
                0.0, 104.0, 1.0, 21.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0005,
                0.0, 2e-9, 3.9, 5e23, 1.0, 0.0, 1.0, 0.0,
                1e26, 2e-9, 2e-9, 5e25, 5e25, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.03,
                1.0, 0.5, 0.0, 1.5, 1.5, 0.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 1.0, 50.0, 1.0, 0.0, 0.0,
                0.3, 1.0, 0.0, 0.0, 1.0, 8.0, 0.01, 0.0,
                0.0, 0.05, 1.0, 10.0, 0.0, 1.0, 0.0, 10.0,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.375, 0.063, 0.375,
                0.063, 0.375, 0.063, 3.1, 0.0, 0.0, 41.0, 41.0,
                0.0, 0.0, 0.0, 0.0, 1e-14, 0.0, 1.0, 0.1,
                8.0, 0.0, 0.0, 1e-15, 1e-15, 0.5, 0.5, 1.0,
                1e-15, 5e-16, 5e-16, 0.0, 0.3, 0.5, 0.4, 1e-15,
                1e-15, 1.0, 0.0, 8e22, 30000000.0, 0.0, 1.0, -1.0,
                0.0005, 0.0, 5e23, 0.0, 0.0006, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 4e24, 1500000000.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0,
                0.0005, 0.0, 0.0, 0.0, 0.0, 2e-9, 3.9, 4e23,
                0.0, 1e-8, 1e24, 0.0, 1e-8, 1e-8, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1e26, 0.0, 2e-9,
                2e-9, 1e-8, 1e-8, 5e25, 5e25, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.03,
                0.0, 0.0, 1e-8, 0.0, 0.0, 1e-8, 0.0, 0.0,
                1e-9, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0,
                1.5, 1.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                50.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.3, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 16.0, 1.0, 0.01, 1.0, 0.0, 0.0, 0.5,
                0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.05, 1.0,
                0.0, 0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.375, 0.063, 0.375, 0.063, 0.375, 0.063, 3.1, 0.0,
                0.0, 41.0, 41.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.1, 1.0, 0.0, 0.0, 16.0, 1.0, 0.0,
                1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.5,
                1.0, 1e-15, 5e-16, 5e-16, 0.0, 0.3, 0.5, 0.4,
                1e-15, 1e-15, 1.0, 0.0, 8e22, 30000000.0, 0.0, 1.0,
                0.0, 2.0, 1e-8, 0.0, -1.0, 0.0005, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5e23, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1e-8,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 1.0,
                8e22, 30000000.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                -1.0, 0.0, 0.0, 0.0, 0.0005, 0.0, 0.0, 0.0,
                5e23, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e26, 0.0, 0.0, 0.0, 5e25, 0.0, 0.0, 0.0,
                5e25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0,
                1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                50.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.3, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                8.0, 0.0, 0.0, 0.0, 0.01, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-14, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0,
                8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0, 0.0,
                1e-15, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0, 0.0,
                5e-16, 0.0, 0.0, 0.0, 5e-16, 0.0, 0.0, 0.0,
                1e-15, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 8e22, 0.0, 0.0, 0.0,
                30000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                -1.0, 0.0, 0.0, 0.0, 0.0005, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 5e23, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 8e22, 0.0, 0.0, 0.0,
                30000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1e-6, 1e-6, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1000.0, 21.0, 1000.0, 0.001, 1e-9,
                1e-9, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 1.16,
                1.16, 1.16, 1e-12, 1e-18, 1e-18, 100.0, 0.0001, 0.0001,
                1e-7, 1e-7, 100.0, 0.0001, 0.0001, 0.25, 0.25, 0.25,
                1e-12, 1e-18, 1e-18, 1000000000.0, 1000000000.0, 1000000000.0, -0.001, -0.001,
                -0.001, 10.0, 10.0, 10.0, 4.0, 4.0, 4.0, 1.0,
                1.0, 1.0, 1.0, -1.0, 0.1, 0.0, 0.5, 0.0,
                0.5, 0.001, 1e-9, 1e-9, 1.0, 1.0, 1.0, 0.5,
                0.5, 0.5, 1.16, 1.16, 1.16, 1e-12, 1e-18, 1e-18,
                100.0, 0.0001, 0.0001, 1e-7, 1e-7, 100.0, 0.0001, 0.0001,
                0.25, 0.25, 0.25, 1e-12, 1e-18, 1e-18, 1000000000.0, 1000000000.0,
                1000000000.0, -0.001, -0.001, -0.001, 10.0, 10.0, 10.0, 4.0,
                4.0, 4.0, 1.0, 1.0, 1.0, 1.0, -1.0, 0.1,
                0.0, 0.5, 0.0, 0.5, 0.0, 2.5, 0.03, 2.5,
                0.03,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (*ptr).values.as_mut_ptr().add(35), 897);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 934] = [
    ("l", 0), ("w", 1), ("sa", 2), ("sb", 3), ("sd", 4), ("sca", 5), ("scb", 6), ("scc", 7), ("sc", 8), ("nf", 9), ("ngcon", 10), ("xgw", 11), ("nrs", 12), ("nrd", 13), ("jw", 14), ("delvto", 15),
    ("factuo", 16), ("delvtoedge", 17), ("factuoedge", 18), ("absource", 19), ("lssource", 20), ("lgsource", 21), ("abdrain", 22), ("lsdrain", 23), ("lgdrain", 24), ("as", 25), ("ps", 26), ("ad", 27), ("pd", 28), ("ifactor", 29), ("cfactor", 30), ("mult", 31),
    ("mult_i", 32), ("mult_q", 33), ("mult_fn", 34), ("trise", 35), ("dtemp", 35), ("level", 36), ("type", 37), ("tr", 38), ("tref", 38), ("swgeo", 39), ("swigate", 40), ("swimpact", 41), ("swgidl", 42), ("swjuncap", 43), ("swjunasym", 44), ("swnud", 45),
    ("swedge", 46), ("swdelvtac", 47), ("swqsat", 48), ("swqpart", 49), ("swign", 50), ("swnqs", 51), ("qmc", 52), ("swoprext", 53), ("swoppmos", 54), ("swopdrain", 55), ("dta", 56), ("vfb", 57), ("stvfb", 58), ("st2vfb", 59), ("tox", 60), ("epsrox", 61),
    ("neff", 62), ("gfacnud", 63), ("vsbnud", 64), ("dvsbnud", 65), ("dphib", 66), ("np", 67), ("toxov", 68), ("toxovd", 69), ("nov", 70), ("novd", 71), ("ct", 72), ("ctb", 73), ("ctg", 74), ("stct", 75), ("cf", 76), ("cfb", 77),
    ("cfd", 78), ("psce", 79), ("psceb", 80), ("psced", 81), ("betn", 82), ("stbet", 83), ("mue", 84), ("stmue", 85), ("themu", 86), ("stthemu", 87), ("cs", 88), ("stcs", 89), ("thecs", 90), ("stthecs", 91), ("xcor", 92), ("stxcor", 93),
    ("feta", 94), ("rs", 95), ("strs", 96), ("rsb", 97), ("rsg", 98), ("thesat", 99), ("stthesat", 100), ("thesatb", 101), ("thesatg", 102), ("thesatt", 103), ("ax", 104), ("alp", 105), ("alp1", 106), ("alp2", 107), ("vp", 108), ("a1", 109),
    ("a2", 110), ("sta2", 111), ("a3", 112), ("a4", 113), ("imaxii", 114), ("gco", 115), ("iginv", 116), ("igov", 117), ("igovd", 118), ("stig", 119), ("gc2", 120), ("gc3", 121), ("gc2ov", 122), ("gc3ov", 123), ("gc2ovd", 124), ("gc3ovd", 125),
    ("chib", 126), ("agidl", 127), ("agidld", 128), ("bgidl", 129), ("bgidld", 130), ("stbgidl", 131), ("stbgidld", 132), ("cgidl", 133), ("cgidld", 134), ("cox", 135), ("delvtac", 136), ("facneffac", 137), ("thesatac", 138), ("axac", 139), ("alpac", 140), ("alp1ac", 141),
    ("cgov", 142), ("cgovd", 143), ("fcgovacc", 144), ("fcgovaccd", 145), ("cgovaccg", 146), ("cgbov", 147), ("cinr", 148), ("cinrd", 149), ("dvfbinr", 150), ("fcinrdep", 151), ("fcinracc", 152), ("axinr", 153), ("cfr", 154), ("cfrd", 155), ("fnt", 156), ("fntexc", 157),
    ("nfa", 158), ("nfb", 159), ("nfc", 160), ("ef", 161), ("vfbedge", 162), ("stvfbedge", 163), ("dphibedge", 164), ("neffedge", 165), ("ctedge", 166), ("betnedge", 167), ("stbetedge", 168), ("psceedge", 169), ("pscebedge", 170), ("pscededge", 171), ("cfedge", 172), ("cfbedge", 173),
    ("cfdedge", 174), ("fntedge", 175), ("nfaedge", 176), ("nfbedge", 177), ("nfcedge", 178), ("efedge", 179), ("rg", 180), ("rse", 181), ("rde", 182), ("rbulk", 183), ("rwell", 184), ("rjuns", 185), ("rjund", 186), ("munqs", 187), ("lvaro", 188), ("lvarl", 189),
    ("lvarw", 190), ("lap", 191), ("wvaro", 192), ("wvarl", 193), ("wvarw", 194), ("wot", 195), ("dlq", 196), ("dwq", 197), ("vfbo", 198), ("vfbl", 199), ("vfblexp", 200), ("vfbw", 201), ("vfblw", 202), ("stvfbo", 203), ("stvfbl", 204), ("stvfbw", 205),
    ("stvfblw", 206), ("st2vfbo", 207), ("toxo", 208), ("epsroxo", 209), ("nsubo", 210), ("nsubw", 211), ("wseg", 212), ("npck", 213), ("npckw", 214), ("wsegp", 215), ("lpck", 216), ("lpckw", 217), ("fol1", 218), ("fol2", 219), ("gfacnudo", 220), ("gfacnudl", 221),
    ("gfacnudlexp", 222), ("gfacnudw", 223), ("gfacnudlw", 224), ("vsbnudo", 225), ("dvsbnudo", 226), ("dphibo", 227), ("dphibl", 228), ("dphiblexp", 229), ("dphibw", 230), ("dphiblw", 231), ("npo", 232), ("npl", 233), ("toxovo", 234), ("toxovdo", 235), ("lov", 236), ("lovd", 237),
    ("novo", 238), ("novdo", 239), ("cto", 240), ("ctl", 241), ("ctlexp", 242), ("ctw", 243), ("ctlw", 244), ("ctbo", 245), ("ctgo", 246), ("stcto", 247), ("cfl", 248), ("cflexp", 249), ("cfw", 250), ("cfbo", 251), ("cfdo", 252), ("pscel", 253),
    ("pscelexp", 254), ("pscew", 255), ("pscebo", 256), ("pscedo", 257), ("uo", 258), ("fbet1", 259), ("fbet1w", 260), ("lp1", 261), ("lp1w", 262), ("fbet2", 263), ("lp2", 264), ("betw1", 265), ("betw2", 266), ("wbet", 267), ("stbeto", 268), ("stbetl", 269),
    ("stbetw", 270), ("stbetlw", 271), ("mueo", 272), ("muew", 273), ("stmueo", 274), ("themuo", 275), ("stthemuo", 276), ("cso", 277), ("csl", 278), ("cslexp", 279), ("csw", 280), ("cslw", 281), ("stcso", 282), ("thecso", 283), ("stthecso", 284), ("xcoro", 285),
    ("xcorl", 286), ("xcorw", 287), ("xcorlw", 288), ("stxcoro", 289), ("fetao", 290), ("rsw1", 291), ("rsw2", 292), ("strso", 293), ("rsbo", 294), ("rsgo", 295), ("thesato", 296), ("thesatl", 297), ("thesatlexp", 298), ("thesatw", 299), ("thesatlw", 300), ("stthesato", 301),
    ("stthesatl", 302), ("stthesatw", 303), ("stthesatlw", 304), ("thesatbo", 305), ("thesatgo", 306), ("thesatto", 307), ("axo", 308), ("axl", 309), ("alpl", 310), ("alplexp", 311), ("alpw", 312), ("alp1l1", 313), ("alp1lexp", 314), ("alp1l2", 315), ("alp1w", 316), ("alp2l1", 317),
    ("alp2lexp", 318), ("alp2l2", 319), ("alp2w", 320), ("vpo", 321), ("a1o", 322), ("a1l", 323), ("a1w", 324), ("a2o", 325), ("sta2o", 326), ("a3o", 327), ("a3l", 328), ("a3w", 329), ("a4o", 330), ("a4l", 331), ("a4w", 332), ("imaxiio", 333),
    ("gcoo", 334), ("iginvlw", 335), ("igovw", 336), ("igovdw", 337), ("stigo", 338), ("gc2o", 339), ("gc3o", 340), ("gc2ovo", 341), ("gc3ovo", 342), ("gc2ovdo", 343), ("gc3ovdo", 344), ("chibo", 345), ("agidlw", 346), ("agidldw", 347), ("bgidlo", 348), ("bgidldo", 349),
    ("stbgidlo", 350), ("stbgidldo", 351), ("cgidlo", 352), ("cgidldo", 353), ("delvtaco", 354), ("delvtacl", 355), ("delvtaclexp", 356), ("delvtacw", 357), ("delvtaclw", 358), ("facneffaco", 359), ("facneffacl", 360), ("facneffacw", 361), ("facneffaclw", 362), ("thesataco", 363), ("thesatacl", 364), ("thesataclexp", 365),
    ("thesatacw", 366), ("thesataclw", 367), ("axaco", 368), ("axacl", 369), ("alpacl", 370), ("alpaclexp", 371), ("alpacw", 372), ("alp1acl1", 373), ("alp1aclexp", 374), ("alp1acl2", 375), ("alp1acw", 376), ("fcgovacco", 377), ("fcgovaccdo", 378), ("cgovaccgo", 379), ("cgbovl", 380), ("cinrw", 381),
    ("cinrdw", 382), ("dvfbinro", 383), ("fcinrdepo", 384), ("fcinracco", 385), ("axinro", 386), ("cfrw", 387), ("cfrdw", 388), ("fnto", 389), ("fntexcl", 390), ("nfalw", 391), ("nfblw", 392), ("nfclw", 393), ("efo", 394), ("lintnoi", 395), ("alpnoi", 396), ("wedge", 397),
    ("wedgew", 398), ("vfbedgeo", 399), ("stvfbedgeo", 400), ("stvfbedgel", 401), ("stvfbedgew", 402), ("stvfbedgelw", 403), ("dphibedgeo", 404), ("dphibedgel", 405), ("dphibedgelexp", 406), ("dphibedgew", 407), ("dphibedgelw", 408), ("nsubedgeo", 409), ("nsubedgel", 410), ("nsubedgelexp", 411), ("nsubedgew", 412), ("nsubedgelw", 413),
    ("ctedgeo", 414), ("ctedgel", 415), ("ctedgelexp", 416), ("fbetedge", 417), ("lpedge", 418), ("betedgew", 419), ("stbetedgeo", 420), ("stbetedgel", 421), ("stbetedgew", 422), ("stbetedgelw", 423), ("psceedgel", 424), ("psceedgelexp", 425), ("psceedgew", 426), ("pscebedgeo", 427), ("pscededgeo", 428), ("cfedgel", 429),
    ("cfedgelexp", 430), ("cfedgew", 431), ("cfbedgeo", 432), ("cfdedgeo", 433), ("fntedgeo", 434), ("nfaedgelw", 435), ("nfbedgelw", 436), ("nfcedgelw", 437), ("efedgeo", 438), ("rgo", 439), ("rint", 440), ("rvpoly", 441), ("rshg", 442), ("dlsil", 443), ("rsh", 444), ("rshd", 445),
    ("rbulko", 446), ("rwello", 447), ("rjunso", 448), ("rjundo", 449), ("munqso", 450), ("povfb", 451), ("plvfb", 452), ("pwvfb", 453), ("plwvfb", 454), ("postvfb", 455), ("plstvfb", 456), ("pwstvfb", 457), ("plwstvfb", 458), ("poneff", 459), ("plneff", 460), ("pwneff", 461),
    ("plwneff", 462), ("pogfacnud", 463), ("plgfacnud", 464), ("pwgfacnud", 465), ("plwgfacnud", 466), ("povsbnud", 467), ("plvsbnud", 468), ("pwvsbnud", 469), ("plwvsbnud", 470), ("podphib", 471), ("pldphib", 472), ("pwdphib", 473), ("plwdphib", 474), ("ponp", 475), ("plnp", 476), ("pwnp", 477),
    ("plwnp", 478), ("ponov", 479), ("plnov", 480), ("pwnov", 481), ("plwnov", 482), ("ponovd", 483), ("plnovd", 484), ("pwnovd", 485), ("plwnovd", 486), ("poct", 487), ("plct", 488), ("pwct", 489), ("plwct", 490), ("poctb", 491), ("plctb", 492), ("pwctb", 493),
    ("plwctb", 494), ("poctg", 495), ("plctg", 496), ("pwctg", 497), ("plwctg", 498), ("postct", 499), ("plstct", 500), ("pwstct", 501), ("plwstct", 502), ("pocf", 503), ("plcf", 504), ("pwcf", 505), ("plwcf", 506), ("pocfb", 507), ("plcfb", 508), ("pwcfb", 509),
    ("plwcfb", 510), ("pocfd", 511), ("plcfd", 512), ("pwcfd", 513), ("plwcfd", 514), ("popsce", 515), ("plpsce", 516), ("pwpsce", 517), ("plwpsce", 518), ("popsceb", 519), ("plpsceb", 520), ("pwpsceb", 521), ("plwpsceb", 522), ("popsced", 523), ("plpsced", 524), ("pwpsced", 525),
    ("plwpsced", 526), ("pobetn", 527), ("plbetn", 528), ("pwbetn", 529), ("plwbetn", 530), ("postbet", 531), ("plstbet", 532), ("pwstbet", 533), ("plwstbet", 534), ("pomue", 535), ("plmue", 536), ("pwmue", 537), ("plwmue", 538), ("pothemu", 539), ("plthemu", 540), ("pwthemu", 541),
    ("plwthemu", 542), ("pocs", 543), ("plcs", 544), ("pwcs", 545), ("plwcs", 546), ("pothecs", 547), ("plthecs", 548), ("pwthecs", 549), ("plwthecs", 550), ("poxcor", 551), ("plxcor", 552), ("pwxcor", 553), ("plwxcor", 554), ("pors", 555), ("plrs", 556), ("pwrs", 557),
    ("plwrs", 558), ("postrs", 559), ("plstrs", 560), ("pwstrs", 561), ("plwstrs", 562), ("porsb", 563), ("plrsb", 564), ("pwrsb", 565), ("plwrsb", 566), ("porsg", 567), ("plrsg", 568), ("pwrsg", 569), ("plwrsg", 570), ("pothesat", 571), ("plthesat", 572), ("pwthesat", 573),
    ("plwthesat", 574), ("postthesat", 575), ("plstthesat", 576), ("pwstthesat", 577), ("plwstthesat", 578), ("pothesatb", 579), ("plthesatb", 580), ("pwthesatb", 581), ("plwthesatb", 582), ("pothesatg", 583), ("plthesatg", 584), ("pwthesatg", 585), ("plwthesatg", 586), ("poax", 587), ("plax", 588), ("pwax", 589),
    ("plwax", 590), ("poalp", 591), ("plalp", 592), ("pwalp", 593), ("plwalp", 594), ("poalp1", 595), ("plalp1", 596), ("pwalp1", 597), ("plwalp1", 598), ("poalp2", 599), ("plalp2", 600), ("pwalp2", 601), ("plwalp2", 602), ("poa1", 603), ("pla1", 604), ("pwa1", 605),
    ("plwa1", 606), ("posta2", 607), ("plsta2", 608), ("pwsta2", 609), ("plwsta2", 610), ("poa3", 611), ("pla3", 612), ("pwa3", 613), ("plwa3", 614), ("poa4", 615), ("pla4", 616), ("pwa4", 617), ("plwa4", 618), ("poiginv", 619), ("pliginv", 620), ("pwiginv", 621),
    ("plwiginv", 622), ("poigov", 623), ("pligov", 624), ("pwigov", 625), ("plwigov", 626), ("poigovd", 627), ("pligovd", 628), ("pwigovd", 629), ("plwigovd", 630), ("postig", 631), ("plstig", 632), ("pwstig", 633), ("plwstig", 634), ("poagidl", 635), ("plagidl", 636), ("pwagidl", 637),
    ("plwagidl", 638), ("poagidld", 639), ("plagidld", 640), ("pwagidld", 641), ("plwagidld", 642), ("postbgidl", 643), ("plstbgidl", 644), ("pwstbgidl", 645), ("plwstbgidl", 646), ("postbgidld", 647), ("plstbgidld", 648), ("pwstbgidld", 649), ("plwstbgidld", 650), ("pocox", 651), ("plcox", 652), ("pwcox", 653),
    ("plwcox", 654), ("podelvtac", 655), ("pldelvtac", 656), ("pwdelvtac", 657), ("plwdelvtac", 658), ("pofacneffac", 659), ("plfacneffac", 660), ("pwfacneffac", 661), ("plwfacneffac", 662), ("pothesatac", 663), ("plthesatac", 664), ("pwthesatac", 665), ("plwthesatac", 666), ("poaxac", 667), ("plaxac", 668), ("pwaxac", 669),
    ("plwaxac", 670), ("poalpac", 671), ("plalpac", 672), ("pwalpac", 673), ("plwalpac", 674), ("poalp1ac", 675), ("plalp1ac", 676), ("pwalp1ac", 677), ("plwalp1ac", 678), ("pocgov", 679), ("plcgov", 680), ("pwcgov", 681), ("plwcgov", 682), ("pocgovd", 683), ("plcgovd", 684), ("pwcgovd", 685),
    ("plwcgovd", 686), ("pocgbov", 687), ("plcgbov", 688), ("pwcgbov", 689), ("plwcgbov", 690), ("pocinr", 691), ("plcinr", 692), ("pwcinr", 693), ("plwcinr", 694), ("pocinrd", 695), ("plcinrd", 696), ("pwcinrd", 697), ("plwcinrd", 698), ("pocfr", 699), ("plcfr", 700), ("pwcfr", 701),
    ("plwcfr", 702), ("pocfrd", 703), ("plcfrd", 704), ("pwcfrd", 705), ("plwcfrd", 706), ("pofntexc", 707), ("plfntexc", 708), ("pwfntexc", 709), ("plwfntexc", 710), ("ponfa", 711), ("plnfa", 712), ("pwnfa", 713), ("plwnfa", 714), ("ponfb", 715), ("plnfb", 716), ("pwnfb", 717),
    ("plwnfb", 718), ("ponfc", 719), ("plnfc", 720), ("pwnfc", 721), ("plwnfc", 722), ("povfbedge", 723), ("plvfbedge", 724), ("pwvfbedge", 725), ("plwvfbedge", 726), ("postvfbedge", 727), ("plstvfbedge", 728), ("pwstvfbedge", 729), ("plwstvfbedge", 730), ("podphibedge", 731), ("pldphibedge", 732), ("pwdphibedge", 733),
    ("plwdphibedge", 734), ("poneffedge", 735), ("plneffedge", 736), ("pwneffedge", 737), ("plwneffedge", 738), ("poctedge", 739), ("plctedge", 740), ("pwctedge", 741), ("plwctedge", 742), ("pobetnedge", 743), ("plbetnedge", 744), ("pwbetnedge", 745), ("plwbetnedge", 746), ("postbetedge", 747), ("plstbetedge", 748), ("pwstbetedge", 749),
    ("plwstbetedge", 750), ("popsceedge", 751), ("plpsceedge", 752), ("pwpsceedge", 753), ("plwpsceedge", 754), ("popscebedge", 755), ("plpscebedge", 756), ("pwpscebedge", 757), ("plwpscebedge", 758), ("popscededge", 759), ("plpscededge", 760), ("pwpscededge", 761), ("plwpscededge", 762), ("pocfedge", 763), ("plcfedge", 764), ("pwcfedge", 765),
    ("plwcfedge", 766), ("pocfbedge", 767), ("plcfbedge", 768), ("pwcfbedge", 769), ("plwcfbedge", 770), ("pocfdedge", 771), ("plcfdedge", 772), ("pwcfdedge", 773), ("plwcfdedge", 774), ("ponfaedge", 775), ("plnfaedge", 776), ("pwnfaedge", 777), ("plwnfaedge", 778), ("ponfbedge", 779), ("plnfbedge", 780), ("pwnfbedge", 781),
    ("plwnfbedge", 782), ("ponfcedge", 783), ("plnfcedge", 784), ("pwnfcedge", 785), ("plwnfcedge", 786), ("pomunqs", 787), ("plmunqs", 788), ("pwmunqs", 789), ("plwmunqs", 790), ("saref", 791), ("sbref", 792), ("wlod", 793), ("kuo", 794), ("kvsat", 795), ("kvsatac", 796), ("tkuo", 797),
    ("lkuo", 798), ("wkuo", 799), ("pkuo", 800), ("llodkuo", 801), ("wlodkuo", 802), ("kvtho", 803), ("lkvtho", 804), ("wkvtho", 805), ("pkvtho", 806), ("llodvth", 807), ("wlodvth", 808), ("stetao", 809), ("lodetao", 810), ("scref", 811), ("web", 812), ("wec", 813),
    ("kvthoweo", 814), ("kvthowel", 815), ("kvthowew", 816), ("kvthowelw", 817), ("kuoweo", 818), ("kuowel", 819), ("kuowew", 820), ("kuowelw", 821), ("imax", 822), ("trj", 823), ("frev", 824), ("cjorbot", 825), ("cjorsti", 826), ("cjorgat", 827), ("vbirbot", 828), ("vbirsti", 829),
    ("vbirgat", 830), ("pbot", 831), ("psti", 832), ("pgat", 833), ("phigbot", 834), ("phigsti", 835), ("phiggat", 836), ("idsatrbot", 837), ("idsatrsti", 838), ("idsatrgat", 839), ("csrhbot", 840), ("csrhsti", 841), ("csrhgat", 842), ("xjunsti", 843), ("xjungat", 844), ("ctatbot", 845),
    ("ctatsti", 846), ("ctatgat", 847), ("mefftatbot", 848), ("mefftatsti", 849), ("mefftatgat", 850), ("cbbtbot", 851), ("cbbtsti", 852), ("cbbtgat", 853), ("fbbtrbot", 854), ("fbbtrsti", 855), ("fbbtrgat", 856), ("stfbbtbot", 857), ("stfbbtsti", 858), ("stfbbtgat", 859), ("vbrbot", 860), ("vbrsti", 861),
    ("vbrgat", 862), ("pbrbot", 863), ("pbrsti", 864), ("pbrgat", 865), ("fcjorgat2", 866), ("fvbirgat2", 867), ("fpgat2", 868), ("fphiggat2", 869), ("vtrgat", 870), ("anugat", 871), ("advbrgat", 872), ("bdvbrgat", 873), ("adbbtgat", 874), ("bdbbtgat", 875), ("cjorbotd", 876), ("cjorstid", 877),
    ("cjorgatd", 878), ("vbirbotd", 879), ("vbirstid", 880), ("vbirgatd", 881), ("pbotd", 882), ("pstid", 883), ("pgatd", 884), ("phigbotd", 885), ("phigstid", 886), ("phiggatd", 887), ("idsatrbotd", 888), ("idsatrstid", 889), ("idsatrgatd", 890), ("csrhbotd", 891), ("csrhstid", 892), ("csrhgatd", 893),
    ("xjunstid", 894), ("xjungatd", 895), ("ctatbotd", 896), ("ctatstid", 897), ("ctatgatd", 898), ("mefftatbotd", 899), ("mefftatstid", 900), ("mefftatgatd", 901), ("cbbtbotd", 902), ("cbbtstid", 903), ("cbbtgatd", 904), ("fbbtrbotd", 905), ("fbbtrstid", 906), ("fbbtrgatd", 907), ("stfbbtbotd", 908), ("stfbbtstid", 909),
    ("stfbbtgatd", 910), ("vbrbotd", 911), ("vbrstid", 912), ("vbrgatd", 913), ("pbrbotd", 914), ("pbrstid", 915), ("pbrgatd", 916), ("fcjorgat2d", 917), ("fvbirgat2d", 918), ("fpgat2d", 919), ("fphiggat2d", 920), ("vtrgatd", 921), ("anugatd", 922), ("advbrgatd", 923), ("bdvbrgatd", 924), ("adbbtgatd", 925),
    ("bdbbtgatd", 926), ("swjunexp", 927), ("vjunref", 928), ("fjunq", 929), ("vjunrefd", 930), ("fjunqd", 931),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 932] = [
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
    None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 932] = [
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
    None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 932] = [
    "L", "W", "SA", "SB", "SD", "SCA", "SCB", "SCC", "SC", "NF", "NGCON", "XGW", "NRS", "NRD", "JW", "DELVTO",
    "FACTUO", "DELVTOEDGE", "FACTUOEDGE", "ABSOURCE", "LSSOURCE", "LGSOURCE", "ABDRAIN", "LSDRAIN", "LGDRAIN", "AS", "PS", "AD", "PD", "IFACTOR", "CFACTOR", "MULT",
    "MULT_I", "MULT_Q", "MULT_FN", "TRISE", "LEVEL", "TYPE", "TR", "SWGEO", "SWIGATE", "SWIMPACT", "SWGIDL", "SWJUNCAP", "SWJUNASYM", "SWNUD", "SWEDGE", "SWDELVTAC",
    "SWQSAT", "SWQPART", "SWIGN", "SWNQS", "QMC", "SWOPREXT", "SWOPPMOS", "SWOPDRAIN", "DTA", "VFB", "STVFB", "ST2VFB", "TOX", "EPSROX", "NEFF", "GFACNUD",
    "VSBNUD", "DVSBNUD", "DPHIB", "NP", "TOXOV", "TOXOVD", "NOV", "NOVD", "CT", "CTB", "CTG", "STCT", "CF", "CFB", "CFD", "PSCE",
    "PSCEB", "PSCED", "BETN", "STBET", "MUE", "STMUE", "THEMU", "STTHEMU", "CS", "STCS", "THECS", "STTHECS", "XCOR", "STXCOR", "FETA", "RS",
    "STRS", "RSB", "RSG", "THESAT", "STTHESAT", "THESATB", "THESATG", "THESATT", "AX", "ALP", "ALP1", "ALP2", "VP", "A1", "A2", "STA2",
    "A3", "A4", "IMAXII", "GCO", "IGINV", "IGOV", "IGOVD", "STIG", "GC2", "GC3", "GC2OV", "GC3OV", "GC2OVD", "GC3OVD", "CHIB", "AGIDL",
    "AGIDLD", "BGIDL", "BGIDLD", "STBGIDL", "STBGIDLD", "CGIDL", "CGIDLD", "COX", "DELVTAC", "FACNEFFAC", "THESATAC", "AXAC", "ALPAC", "ALP1AC", "CGOV", "CGOVD",
    "FCGOVACC", "FCGOVACCD", "CGOVACCG", "CGBOV", "CINR", "CINRD", "DVFBINR", "FCINRDEP", "FCINRACC", "AXINR", "CFR", "CFRD", "FNT", "FNTEXC", "NFA", "NFB",
    "NFC", "EF", "VFBEDGE", "STVFBEDGE", "DPHIBEDGE", "NEFFEDGE", "CTEDGE", "BETNEDGE", "STBETEDGE", "PSCEEDGE", "PSCEBEDGE", "PSCEDEDGE", "CFEDGE", "CFBEDGE", "CFDEDGE", "FNTEDGE",
    "NFAEDGE", "NFBEDGE", "NFCEDGE", "EFEDGE", "RG", "RSE", "RDE", "RBULK", "RWELL", "RJUNS", "RJUND", "MUNQS", "LVARO", "LVARL", "LVARW", "LAP",
    "WVARO", "WVARL", "WVARW", "WOT", "DLQ", "DWQ", "VFBO", "VFBL", "VFBLEXP", "VFBW", "VFBLW", "STVFBO", "STVFBL", "STVFBW", "STVFBLW", "ST2VFBO",
    "TOXO", "EPSROXO", "NSUBO", "NSUBW", "WSEG", "NPCK", "NPCKW", "WSEGP", "LPCK", "LPCKW", "FOL1", "FOL2", "GFACNUDO", "GFACNUDL", "GFACNUDLEXP", "GFACNUDW",
    "GFACNUDLW", "VSBNUDO", "DVSBNUDO", "DPHIBO", "DPHIBL", "DPHIBLEXP", "DPHIBW", "DPHIBLW", "NPO", "NPL", "TOXOVO", "TOXOVDO", "LOV", "LOVD", "NOVO", "NOVDO",
    "CTO", "CTL", "CTLEXP", "CTW", "CTLW", "CTBO", "CTGO", "STCTO", "CFL", "CFLEXP", "CFW", "CFBO", "CFDO", "PSCEL", "PSCELEXP", "PSCEW",
    "PSCEBO", "PSCEDO", "UO", "FBET1", "FBET1W", "LP1", "LP1W", "FBET2", "LP2", "BETW1", "BETW2", "WBET", "STBETO", "STBETL", "STBETW", "STBETLW",
    "MUEO", "MUEW", "STMUEO", "THEMUO", "STTHEMUO", "CSO", "CSL", "CSLEXP", "CSW", "CSLW", "STCSO", "THECSO", "STTHECSO", "XCORO", "XCORL", "XCORW",
    "XCORLW", "STXCORO", "FETAO", "RSW1", "RSW2", "STRSO", "RSBO", "RSGO", "THESATO", "THESATL", "THESATLEXP", "THESATW", "THESATLW", "STTHESATO", "STTHESATL", "STTHESATW",
    "STTHESATLW", "THESATBO", "THESATGO", "THESATTO", "AXO", "AXL", "ALPL", "ALPLEXP", "ALPW", "ALP1L1", "ALP1LEXP", "ALP1L2", "ALP1W", "ALP2L1", "ALP2LEXP", "ALP2L2",
    "ALP2W", "VPO", "A1O", "A1L", "A1W", "A2O", "STA2O", "A3O", "A3L", "A3W", "A4O", "A4L", "A4W", "IMAXIIO", "GCOO", "IGINVLW",
    "IGOVW", "IGOVDW", "STIGO", "GC2O", "GC3O", "GC2OVO", "GC3OVO", "GC2OVDO", "GC3OVDO", "CHIBO", "AGIDLW", "AGIDLDW", "BGIDLO", "BGIDLDO", "STBGIDLO", "STBGIDLDO",
    "CGIDLO", "CGIDLDO", "DELVTACO", "DELVTACL", "DELVTACLEXP", "DELVTACW", "DELVTACLW", "FACNEFFACO", "FACNEFFACL", "FACNEFFACW", "FACNEFFACLW", "THESATACO", "THESATACL", "THESATACLEXP", "THESATACW", "THESATACLW",
    "AXACO", "AXACL", "ALPACL", "ALPACLEXP", "ALPACW", "ALP1ACL1", "ALP1ACLEXP", "ALP1ACL2", "ALP1ACW", "FCGOVACCO", "FCGOVACCDO", "CGOVACCGO", "CGBOVL", "CINRW", "CINRDW", "DVFBINRO",
    "FCINRDEPO", "FCINRACCO", "AXINRO", "CFRW", "CFRDW", "FNTO", "FNTEXCL", "NFALW", "NFBLW", "NFCLW", "EFO", "LINTNOI", "ALPNOI", "WEDGE", "WEDGEW", "VFBEDGEO",
    "STVFBEDGEO", "STVFBEDGEL", "STVFBEDGEW", "STVFBEDGELW", "DPHIBEDGEO", "DPHIBEDGEL", "DPHIBEDGELEXP", "DPHIBEDGEW", "DPHIBEDGELW", "NSUBEDGEO", "NSUBEDGEL", "NSUBEDGELEXP", "NSUBEDGEW", "NSUBEDGELW", "CTEDGEO", "CTEDGEL",
    "CTEDGELEXP", "FBETEDGE", "LPEDGE", "BETEDGEW", "STBETEDGEO", "STBETEDGEL", "STBETEDGEW", "STBETEDGELW", "PSCEEDGEL", "PSCEEDGELEXP", "PSCEEDGEW", "PSCEBEDGEO", "PSCEDEDGEO", "CFEDGEL", "CFEDGELEXP", "CFEDGEW",
    "CFBEDGEO", "CFDEDGEO", "FNTEDGEO", "NFAEDGELW", "NFBEDGELW", "NFCEDGELW", "EFEDGEO", "RGO", "RINT", "RVPOLY", "RSHG", "DLSIL", "RSH", "RSHD", "RBULKO", "RWELLO",
    "RJUNSO", "RJUNDO", "MUNQSO", "POVFB", "PLVFB", "PWVFB", "PLWVFB", "POSTVFB", "PLSTVFB", "PWSTVFB", "PLWSTVFB", "PONEFF", "PLNEFF", "PWNEFF", "PLWNEFF", "POGFACNUD",
    "PLGFACNUD", "PWGFACNUD", "PLWGFACNUD", "POVSBNUD", "PLVSBNUD", "PWVSBNUD", "PLWVSBNUD", "PODPHIB", "PLDPHIB", "PWDPHIB", "PLWDPHIB", "PONP", "PLNP", "PWNP", "PLWNP", "PONOV",
    "PLNOV", "PWNOV", "PLWNOV", "PONOVD", "PLNOVD", "PWNOVD", "PLWNOVD", "POCT", "PLCT", "PWCT", "PLWCT", "POCTB", "PLCTB", "PWCTB", "PLWCTB", "POCTG",
    "PLCTG", "PWCTG", "PLWCTG", "POSTCT", "PLSTCT", "PWSTCT", "PLWSTCT", "POCF", "PLCF", "PWCF", "PLWCF", "POCFB", "PLCFB", "PWCFB", "PLWCFB", "POCFD",
    "PLCFD", "PWCFD", "PLWCFD", "POPSCE", "PLPSCE", "PWPSCE", "PLWPSCE", "POPSCEB", "PLPSCEB", "PWPSCEB", "PLWPSCEB", "POPSCED", "PLPSCED", "PWPSCED", "PLWPSCED", "POBETN",
    "PLBETN", "PWBETN", "PLWBETN", "POSTBET", "PLSTBET", "PWSTBET", "PLWSTBET", "POMUE", "PLMUE", "PWMUE", "PLWMUE", "POTHEMU", "PLTHEMU", "PWTHEMU", "PLWTHEMU", "POCS",
    "PLCS", "PWCS", "PLWCS", "POTHECS", "PLTHECS", "PWTHECS", "PLWTHECS", "POXCOR", "PLXCOR", "PWXCOR", "PLWXCOR", "PORS", "PLRS", "PWRS", "PLWRS", "POSTRS",
    "PLSTRS", "PWSTRS", "PLWSTRS", "PORSB", "PLRSB", "PWRSB", "PLWRSB", "PORSG", "PLRSG", "PWRSG", "PLWRSG", "POTHESAT", "PLTHESAT", "PWTHESAT", "PLWTHESAT", "POSTTHESAT",
    "PLSTTHESAT", "PWSTTHESAT", "PLWSTTHESAT", "POTHESATB", "PLTHESATB", "PWTHESATB", "PLWTHESATB", "POTHESATG", "PLTHESATG", "PWTHESATG", "PLWTHESATG", "POAX", "PLAX", "PWAX", "PLWAX", "POALP",
    "PLALP", "PWALP", "PLWALP", "POALP1", "PLALP1", "PWALP1", "PLWALP1", "POALP2", "PLALP2", "PWALP2", "PLWALP2", "POA1", "PLA1", "PWA1", "PLWA1", "POSTA2",
    "PLSTA2", "PWSTA2", "PLWSTA2", "POA3", "PLA3", "PWA3", "PLWA3", "POA4", "PLA4", "PWA4", "PLWA4", "POIGINV", "PLIGINV", "PWIGINV", "PLWIGINV", "POIGOV",
    "PLIGOV", "PWIGOV", "PLWIGOV", "POIGOVD", "PLIGOVD", "PWIGOVD", "PLWIGOVD", "POSTIG", "PLSTIG", "PWSTIG", "PLWSTIG", "POAGIDL", "PLAGIDL", "PWAGIDL", "PLWAGIDL", "POAGIDLD",
    "PLAGIDLD", "PWAGIDLD", "PLWAGIDLD", "POSTBGIDL", "PLSTBGIDL", "PWSTBGIDL", "PLWSTBGIDL", "POSTBGIDLD", "PLSTBGIDLD", "PWSTBGIDLD", "PLWSTBGIDLD", "POCOX", "PLCOX", "PWCOX", "PLWCOX", "PODELVTAC",
    "PLDELVTAC", "PWDELVTAC", "PLWDELVTAC", "POFACNEFFAC", "PLFACNEFFAC", "PWFACNEFFAC", "PLWFACNEFFAC", "POTHESATAC", "PLTHESATAC", "PWTHESATAC", "PLWTHESATAC", "POAXAC", "PLAXAC", "PWAXAC", "PLWAXAC", "POALPAC",
    "PLALPAC", "PWALPAC", "PLWALPAC", "POALP1AC", "PLALP1AC", "PWALP1AC", "PLWALP1AC", "POCGOV", "PLCGOV", "PWCGOV", "PLWCGOV", "POCGOVD", "PLCGOVD", "PWCGOVD", "PLWCGOVD", "POCGBOV",
    "PLCGBOV", "PWCGBOV", "PLWCGBOV", "POCINR", "PLCINR", "PWCINR", "PLWCINR", "POCINRD", "PLCINRD", "PWCINRD", "PLWCINRD", "POCFR", "PLCFR", "PWCFR", "PLWCFR", "POCFRD",
    "PLCFRD", "PWCFRD", "PLWCFRD", "POFNTEXC", "PLFNTEXC", "PWFNTEXC", "PLWFNTEXC", "PONFA", "PLNFA", "PWNFA", "PLWNFA", "PONFB", "PLNFB", "PWNFB", "PLWNFB", "PONFC",
    "PLNFC", "PWNFC", "PLWNFC", "POVFBEDGE", "PLVFBEDGE", "PWVFBEDGE", "PLWVFBEDGE", "POSTVFBEDGE", "PLSTVFBEDGE", "PWSTVFBEDGE", "PLWSTVFBEDGE", "PODPHIBEDGE", "PLDPHIBEDGE", "PWDPHIBEDGE", "PLWDPHIBEDGE", "PONEFFEDGE",
    "PLNEFFEDGE", "PWNEFFEDGE", "PLWNEFFEDGE", "POCTEDGE", "PLCTEDGE", "PWCTEDGE", "PLWCTEDGE", "POBETNEDGE", "PLBETNEDGE", "PWBETNEDGE", "PLWBETNEDGE", "POSTBETEDGE", "PLSTBETEDGE", "PWSTBETEDGE", "PLWSTBETEDGE", "POPSCEEDGE",
    "PLPSCEEDGE", "PWPSCEEDGE", "PLWPSCEEDGE", "POPSCEBEDGE", "PLPSCEBEDGE", "PWPSCEBEDGE", "PLWPSCEBEDGE", "POPSCEDEDGE", "PLPSCEDEDGE", "PWPSCEDEDGE", "PLWPSCEDEDGE", "POCFEDGE", "PLCFEDGE", "PWCFEDGE", "PLWCFEDGE", "POCFBEDGE",
    "PLCFBEDGE", "PWCFBEDGE", "PLWCFBEDGE", "POCFDEDGE", "PLCFDEDGE", "PWCFDEDGE", "PLWCFDEDGE", "PONFAEDGE", "PLNFAEDGE", "PWNFAEDGE", "PLWNFAEDGE", "PONFBEDGE", "PLNFBEDGE", "PWNFBEDGE", "PLWNFBEDGE", "PONFCEDGE",
    "PLNFCEDGE", "PWNFCEDGE", "PLWNFCEDGE", "POMUNQS", "PLMUNQS", "PWMUNQS", "PLWMUNQS", "SAREF", "SBREF", "WLOD", "KUO", "KVSAT", "KVSATAC", "TKUO", "LKUO", "WKUO",
    "PKUO", "LLODKUO", "WLODKUO", "KVTHO", "LKVTHO", "WKVTHO", "PKVTHO", "LLODVTH", "WLODVTH", "STETAO", "LODETAO", "SCREF", "WEB", "WEC", "KVTHOWEO", "KVTHOWEL",
    "KVTHOWEW", "KVTHOWELW", "KUOWEO", "KUOWEL", "KUOWEW", "KUOWELW", "IMAX", "TRJ", "FREV", "CJORBOT", "CJORSTI", "CJORGAT", "VBIRBOT", "VBIRSTI", "VBIRGAT", "PBOT",
    "PSTI", "PGAT", "PHIGBOT", "PHIGSTI", "PHIGGAT", "IDSATRBOT", "IDSATRSTI", "IDSATRGAT", "CSRHBOT", "CSRHSTI", "CSRHGAT", "XJUNSTI", "XJUNGAT", "CTATBOT", "CTATSTI", "CTATGAT",
    "MEFFTATBOT", "MEFFTATSTI", "MEFFTATGAT", "CBBTBOT", "CBBTSTI", "CBBTGAT", "FBBTRBOT", "FBBTRSTI", "FBBTRGAT", "STFBBTBOT", "STFBBTSTI", "STFBBTGAT", "VBRBOT", "VBRSTI", "VBRGAT", "PBRBOT",
    "PBRSTI", "PBRGAT", "FCJORGAT2", "FVBIRGAT2", "FPGAT2", "FPHIGGAT2", "VTRGAT", "ANUGAT", "ADVBRGAT", "BDVBRGAT", "ADBBTGAT", "BDBBTGAT", "CJORBOTD", "CJORSTID", "CJORGATD", "VBIRBOTD",
    "VBIRSTID", "VBIRGATD", "PBOTD", "PSTID", "PGATD", "PHIGBOTD", "PHIGSTID", "PHIGGATD", "IDSATRBOTD", "IDSATRSTID", "IDSATRGATD", "CSRHBOTD", "CSRHSTID", "CSRHGATD", "XJUNSTID", "XJUNGATD",
    "CTATBOTD", "CTATSTID", "CTATGATD", "MEFFTATBOTD", "MEFFTATSTID", "MEFFTATGATD", "CBBTBOTD", "CBBTSTID", "CBBTGATD", "FBBTRBOTD", "FBBTRSTID", "FBBTRGATD", "STFBBTBOTD", "STFBBTSTID", "STFBBTGATD", "VBRBOTD",
    "VBRSTID", "VBRGATD", "PBRBOTD", "PBRSTID", "PBRGATD", "FCJORGAT2D", "FVBIRGAT2D", "FPGAT2D", "FPHIGGAT2D", "VTRGATD", "ANUGATD", "ADVBRGATD", "BDVBRGATD", "ADBBTGATD", "BDBBTGATD", "SWJUNEXP",
    "VJUNREF", "FJUNQ", "VJUNREFD", "FJUNQD",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 932] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

const PARAMETER_INTEGER_FLAGS: [bool; 932] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false,
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
    false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 932] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 1e23, label: "1e23" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e20, label: "1e20" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }),
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 1e23, label: "1e23" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e20, label: "1e20" }), None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e-9, label: "1e-9" }),
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -250.0, label: "-250.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 932] = [
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 9.0, label: "9.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }), Some(ParameterBound { value: 1e27, label: "1e27" }),
    None, Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None,
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }), Some(ParameterBound { value: 1e27, label: "1e27" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None,
    None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
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
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.95, label: "0.95" }),
    Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 932] = [
    2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2,
    2, 2, 0, 2, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 2,
    0, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2,
    2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2,
    2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0,
    0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2,
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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 0,
    0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0,
    2, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 932] = [
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

pub struct Instance {
    pub nodes: [usize; 21],
    pub branches: [usize; 25],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 932]>,
    pub(crate) multiplicity: f64,
    pub(crate) stamp_state: Box<StampState<11, 9>>,
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
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 17;
    pub const NODE_COUNT: usize = 21;
    pub const INTERNAL_NODE_NAMES: [&str; 17] = ["noi", "gp", "si", "di", "bp", "bi", "bs", "bd", "int1", "int2", "int3", "int4", "int5", "int6", "int7", "int8", "int9"];

    pub const BRANCH_COUNT: usize = 25;
    pub const PARAMETER_COUNT: usize = 932;
    pub const VARIABLE_COUNT: usize = 3438;
    pub const DDT_STATE_COUNT: usize = 11;
    pub const IDT_STATE_COUNT: usize = 9;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "ac80cebc53047529c3bfe30e15db96e5a535a7dcdc0bce72cbe9d5b3c8de08db";
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
        let mut values = Vec::with_capacity(73);
        values.extend_from_slice(&self.stamp_state.ddt_current);
        values.extend_from_slice(&self.stamp_state.ddt_previous);
        values.extend_from_slice(&self.stamp_state.ddt_older);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_current);
        values.extend_from_slice(&self.stamp_state.ddt_derivative_previous);
        values.extend_from_slice(&self.stamp_state.idt_current);
        values.extend_from_slice(&self.stamp_state.idt_previous);
        let mut flags = Vec::with_capacity(20);
        flags.extend_from_slice(&self.stamp_state.ddt_initialized);
        flags.extend_from_slice(&self.stamp_state.idt_initialized);
        GeneratedVerilogARollbackState { values, flags }
    }

    pub(crate) fn restore_rollback_state(&mut self, state: &GeneratedVerilogARollbackState) {
        debug_assert_eq!(state.values.len(), 73);
        debug_assert_eq!(state.flags.len(), 20);
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'PSPNQS104VA'", name));
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
    pub(crate) fn eval_idt(&mut self, slot: usize, value: f64, ic: f64) -> f64 {
        debug_assert!(slot < Self::IDT_STATE_COUNT, "generated idt state slot out of range");
        let previous = if self.stamp_state.idt_initialized[slot] {
            self.stamp_state.idt_previous[slot]
        } else {
            ic
        };
        let current = if self.timestep.abs() > Self::DDT_EPSILON {
            previous + value * self.timestep
        } else {
            ic
        };
        self.stamp_state.idt_current[slot] = current;
        if self.timestep.abs() <= Self::DDT_EPSILON {
            self.stamp_state.idt_previous[slot] = current;
            self.stamp_state.idt_initialized[slot] = true;
        }
        current
    }

    #[inline]
    pub(crate) fn idt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative * self.timestep
        } else {
            0.0
        }
    }
    #[inline]
    pub fn limiter_converged(&self) -> bool {
        true
    }
}
