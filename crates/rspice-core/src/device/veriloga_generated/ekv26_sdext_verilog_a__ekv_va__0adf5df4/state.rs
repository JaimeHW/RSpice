#![allow(dead_code, unused_parens, unused_variables)]

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

fn validate_parameter_metadata(index: usize, value: f64) -> Result<(), String> {
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some(min) = PARAMETER_MIN_BOUNDS[index] {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = PARAMETER_MAX_BOUNDS[index] {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in PARAMETER_EXCLUDED_BOUNDS[index] {
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

const PARAMETER_DISPLAY_NAMES: [&str; 75] = [
    "TYPE", "Noise", "Trise", "TEMP", "TNOM", "L", "W", "M", "NS", "AS", "AD", "PS", "PD", "COX", "XJ", "VTO",
    "TCV", "GAMMA", "PHI", "KP", "BEX", "THETA", "E0", "UCRIT", "UCEX", "LAMBDA", "DL", "DW", "WETA", "LETA", "Q0", "LK",
    "IBA", "IBB", "IBBT", "IBN", "RSH", "HDIF", "AVTO", "AKP", "AGAMMA", "AF", "KF", "xd_n", "xd_js", "xd_jsw", "xd_jswg", "xd_mj",
    "xd_mjsw", "xd_mjswg", "xd_pb", "xd_pbsw", "xd_pbswg", "xd_cj", "xd_cjsw", "xd_cjswg", "xd_gmin", "xd_xjbv", "xd_bv", "xd_njts", "xd_njtssw", "xd_njtsswg", "xd_vts", "xd_vtssw",
    "xd_vtsswg", "tp_xti", "tp_cj", "tp_cjsw", "tp_cjswg", "tp_pb", "tp_pbsw", "tp_pbswg", "tp_njts", "tp_njtssw", "tp_njtsswg",
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
    pub nodes: [usize; 4],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 75]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 5]>,
    pub(crate) ddt_state_previous: Box<[f64; 5]>,
    pub(crate) ddt_state_older: Box<[f64; 5]>,
    pub(crate) ddt_state_initialized: Box<[bool; 5]>,
    pub(crate) ddt_derivative_current: Box<[f64; 5]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 5]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 350]>,
    pub(crate) scalar_static_bool: Box<[bool; 27]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 4;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 75;
    pub const VARIABLE_COUNT: usize = 271;
    pub const DDT_STATE_COUNT: usize = 5;
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
            scalar_static_f64: boxed_zero_f64_array::<350>(),
            scalar_static_bool: boxed_zero_bool_array::<27>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name));
        };
        validate_parameter_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
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
        self.scalar_static_f64[0]=p.p13;
        self.scalar_static_f64[1]=(1.0359399871014713e-10/self.scalar_static_f64[0]);
        self.scalar_static_f64[2]=p.p14;
        self.scalar_static_f64[3]=(self.scalar_static_f64[1]*self.scalar_static_f64[2]);
        self.scalar_static_f64[4]=(self.scalar_static_f64[3]).sqrt();
        self.scalar_static_f64[5]=p.p25;
        self.scalar_static_f64[6]=(self.scalar_static_f64[4]*self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=(self.scalar_static_f64[1]*3.0);
        self.scalar_static_f64[8]=p.p28;
        self.scalar_static_f64[9]=(self.scalar_static_f64[7]*self.scalar_static_f64[8]);
        self.scalar_static_f64[10]=p.p29;
        self.scalar_static_f64[11]=(self.scalar_static_f64[1]*self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=p.p35;
        self.scalar_static_f64[13]=(self.scalar_static_f64[12]+self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=p.p22;
        self.scalar_static_f64[15]=(1.0359399871014713e-10*self.scalar_static_f64[14]);
        self.scalar_static_f64[16]=(self.scalar_static_f64[0]/self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=p.p30;
        self.scalar_static_f64[18]=(self.scalar_static_f64[17]+self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(self.scalar_static_f64[18]/self.scalar_static_f64[0]);
        self.scalar_static_f64[20]=p.p0;
        self.scalar_static_bool[0]=(self.scalar_static_f64[20]>0.0);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[0]{0.5}else{0.3333333333333});
        self.scalar_static_f64[22]=p.p3;
        self.scalar_static_bool[1]=(self.scalar_static_f64[22]==1e21);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_f64[24]=p.p2;
        self.scalar_static_bool[2]=(!(self.scalar_static_f64[23]!=0.0));
        self.scalar_static_f64[25]=(self.scalar_static_f64[22]+273.15);
        self.scalar_static_f64[26]=p.p4;
        self.scalar_static_bool[3]=(1e21==self.scalar_static_f64[26]);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_f64[28]=(if (self.scalar_static_f64[27]!=0.0){298.15}else{0.0});
        self.scalar_static_bool[4]=(!(self.scalar_static_f64[27]!=0.0));
        self.scalar_static_f64[29]=(273.15+self.scalar_static_f64[26]);
        self.scalar_static_f64[30]=(if self.scalar_static_bool[4]{self.scalar_static_f64[29]}else{self.scalar_static_f64[28]});
        self.scalar_static_f64[31]=(self.scalar_static_f64[30]*0.000702);
        self.scalar_static_f64[32]=(self.scalar_static_f64[30]*self.scalar_static_f64[31]);
        self.scalar_static_f64[33]=(self.scalar_static_f64[30]+1108.0);
        self.scalar_static_f64[34]=(self.scalar_static_f64[32]/self.scalar_static_f64[33]);
        self.scalar_static_f64[35]=(1.16-self.scalar_static_f64[34]);
        self.scalar_static_f64[36]=p.p15;
        self.scalar_static_f64[37]=p.p16;
        self.scalar_static_f64[38]=p.p19;
        self.scalar_static_f64[39]=p.p20;
        self.scalar_static_f64[40]=p.p23;
        self.scalar_static_f64[41]=p.p24;
        self.scalar_static_f64[42]=p.p33;
        self.scalar_static_f64[43]=p.p34;
        self.scalar_static_f64[44]=p.p18;
        self.scalar_static_f64[45]=p.p32;
        self.scalar_static_f64[46]=p.p5;
        self.scalar_static_f64[47]=p.p26;
        self.scalar_static_f64[48]=(self.scalar_static_f64[46]+self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p6;
        self.scalar_static_f64[50]=p.p27;
        self.scalar_static_f64[51]=(self.scalar_static_f64[49]+self.scalar_static_f64[50]);
        self.scalar_static_f64[52]=(self.scalar_static_f64[48]*self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=(self.scalar_static_f64[52]).sqrt();
        self.scalar_static_f64[54]=(1.0/self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[56]=p.p38;
        self.scalar_static_bool[5]=(self.scalar_static_f64[56]!=1e-6);
        self.scalar_static_f64[57]=(self.scalar_static_f64[56]-1e-6);
        self.scalar_static_f64[58]=(self.scalar_static_f64[54]*self.scalar_static_f64[57]);
        self.scalar_static_bool[6]=(!(self.scalar_static_f64[55]!=0.0));
        self.scalar_static_f64[59]=(1e-6-self.scalar_static_f64[56]);
        self.scalar_static_f64[60]=(self.scalar_static_f64[54]*self.scalar_static_f64[59]);
        self.scalar_static_f64[61]=p.p39;
        self.scalar_static_bool[7]=(1e-6!=self.scalar_static_f64[61]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[61]-1e-6);
        self.scalar_static_f64[63]=(self.scalar_static_f64[54]*self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=(1.0+self.scalar_static_f64[63]);
        self.scalar_static_f64[65]=p.p40;
        self.scalar_static_bool[8]=(1e-6!=self.scalar_static_f64[65]);
        self.scalar_static_f64[66]=p.p17;
        self.scalar_static_f64[67]=(self.scalar_static_f64[65]-1e-6);
        self.scalar_static_f64[68]=(self.scalar_static_f64[54]*self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(self.scalar_static_f64[66]+self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(if self.scalar_static_bool[8]{self.scalar_static_f64[69]}else{self.scalar_static_f64[66]});
        self.scalar_static_bool[9]=(0.0==self.scalar_static_f64[19]);
        self.scalar_static_f64[71]=(if self.scalar_static_bool[9]{1.0}else{0.0});
        self.scalar_static_bool[10]=(!(self.scalar_static_f64[71]!=0.0));
        self.scalar_static_f64[72]=p.p31;
        self.scalar_static_f64[73]=p.p8;
        self.scalar_static_f64[74]=(self.scalar_static_f64[72]*self.scalar_static_f64[73]);
        self.scalar_static_f64[75]=(self.scalar_static_f64[48]/self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=(self.scalar_static_f64[75]-0.1);
        self.scalar_static_f64[77]=(0.28*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(if self.scalar_static_bool[10]{self.scalar_static_f64[77]}else{0.0});
        self.scalar_static_f64[79]=(self.scalar_static_f64[78]*self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(self.scalar_static_f64[79]+0.001936);
        self.scalar_static_f64[81]=(self.scalar_static_f64[80]).sqrt();
        self.scalar_static_f64[82]=(self.scalar_static_f64[78]+self.scalar_static_f64[81]);
        self.scalar_static_f64[83]=(0.5*self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(1.0+self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(1.0/self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=(if self.scalar_static_bool[10]{self.scalar_static_f64[85]}else{0.0});
        self.scalar_static_f64[87]=(self.scalar_static_f64[19]*self.scalar_static_f64[86]);
        self.scalar_static_f64[88]=(self.scalar_static_f64[86]*self.scalar_static_f64[87]);
        self.scalar_static_f64[89]=(if self.scalar_static_bool[10]{self.scalar_static_f64[88]}else{0.0});
        self.scalar_static_f64[90]=p.p7;
        self.scalar_static_f64[91]=(self.scalar_static_f64[9]*self.scalar_static_f64[90]);
        self.scalar_static_f64[92]=(self.scalar_static_f64[91]/self.scalar_static_f64[51]);
        self.scalar_static_f64[93]=(self.scalar_static_f64[11]*self.scalar_static_f64[73]);
        self.scalar_static_f64[94]=(self.scalar_static_f64[93]/self.scalar_static_f64[48]);
        self.scalar_static_f64[95]=(self.scalar_static_f64[70]*0.25);
        self.scalar_static_f64[96]=(self.scalar_static_f64[70]*self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(0.5*self.scalar_static_f64[70]);
        self.scalar_static_f64[98]=(0.1*self.scalar_static_f64[48]);
        self.scalar_static_f64[99]=(self.scalar_static_f64[98]*self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(self.scalar_static_f64[70]* -0.5);
        self.scalar_static_bool[11]=(0.0==self.scalar_static_f64[14]);
        self.scalar_static_f64[101]=(if self.scalar_static_bool[11]{1.0}else{0.0});
        self.scalar_static_f64[102]=p.p21;
        self.scalar_static_bool[12]=(!(self.scalar_static_f64[101]!=0.0));
        self.scalar_static_f64[103]=(-self.scalar_static_f64[94]);
        self.scalar_static_f64[104]=(-self.scalar_static_f64[70]);
        self.scalar_static_f64[105]=p.p36;
        self.scalar_static_f64[106]=p.p37;
        self.scalar_static_f64[107]=(self.scalar_static_f64[105]*self.scalar_static_f64[106]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[51]-self.scalar_static_f64[50]);
        self.scalar_static_f64[109]=(self.scalar_static_f64[107]/self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=(self.scalar_static_f64[0]*self.scalar_static_f64[52]);
        self.scalar_static_f64[111]=p.p9;
        self.scalar_static_bool[13]=(0.0==self.scalar_static_f64[111]);
        self.scalar_static_bool[14]=(self.scalar_static_f64[106]>0.0);
        self.scalar_static_bool[15]=(self.scalar_static_bool[13]&&self.scalar_static_bool[14]);
        self.scalar_static_f64[112]=(if self.scalar_static_bool[15]{1.0}else{0.0});
        self.scalar_static_f64[113]=(2.0*self.scalar_static_f64[106]);
        self.scalar_static_f64[114]=(self.scalar_static_f64[51]*self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=(if (self.scalar_static_f64[112]!=0.0){self.scalar_static_f64[114]}else{0.0});
        self.scalar_static_bool[16]=(!(self.scalar_static_f64[112]!=0.0));
        self.scalar_static_f64[116]=(if self.scalar_static_bool[16]{self.scalar_static_f64[111]}else{self.scalar_static_f64[115]});
        self.scalar_static_f64[117]=p.p11;
        self.scalar_static_bool[17]=(0.0==self.scalar_static_f64[117]);
        self.scalar_static_bool[18]=(self.scalar_static_bool[14]&&self.scalar_static_bool[17]);
        self.scalar_static_f64[118]=(if self.scalar_static_bool[18]{1.0}else{0.0});
        self.scalar_static_f64[119]=(4.0*self.scalar_static_f64[106]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[51]+self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(if (self.scalar_static_f64[118]!=0.0){self.scalar_static_f64[120]}else{0.0});
        self.scalar_static_bool[19]=(!(self.scalar_static_f64[118]!=0.0));
        self.scalar_static_f64[122]=(if self.scalar_static_bool[19]{self.scalar_static_f64[117]}else{self.scalar_static_f64[121]});
        self.scalar_static_f64[123]=p.p10;
        self.scalar_static_bool[20]=(0.0==self.scalar_static_f64[123]);
        self.scalar_static_bool[21]=(self.scalar_static_bool[14]&&self.scalar_static_bool[20]);
        self.scalar_static_f64[124]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_f64[125]=(if (self.scalar_static_f64[124]!=0.0){self.scalar_static_f64[114]}else{0.0});
        self.scalar_static_bool[22]=(!(self.scalar_static_f64[124]!=0.0));
        self.scalar_static_f64[126]=(if self.scalar_static_bool[22]{self.scalar_static_f64[123]}else{self.scalar_static_f64[125]});
        self.scalar_static_f64[127]=p.p12;
        self.scalar_static_bool[23]=(0.0==self.scalar_static_f64[127]);
        self.scalar_static_bool[24]=(self.scalar_static_bool[14]&&self.scalar_static_bool[23]);
        self.scalar_static_f64[128]=(if self.scalar_static_bool[24]{1.0}else{0.0});
        self.scalar_static_f64[129]=(if (self.scalar_static_f64[128]!=0.0){self.scalar_static_f64[120]}else{0.0});
        self.scalar_static_bool[25]=(!(self.scalar_static_f64[128]!=0.0));
        self.scalar_static_f64[130]=(if self.scalar_static_bool[25]{self.scalar_static_f64[127]}else{self.scalar_static_f64[129]});
        self.scalar_static_f64[131]=(self.scalar_static_f64[30]*8.617333262145179e-5);
        self.scalar_static_f64[132]=(self.scalar_static_f64[35]/self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=p.p65;
        self.scalar_static_f64[134]=p.p43;
        self.scalar_static_f64[135]=p.p44;
        self.scalar_static_f64[136]=p.p45;
        self.scalar_static_f64[137]=p.p46;
        self.scalar_static_f64[138]=p.p50;
        self.scalar_static_f64[139]=p.p69;
        self.scalar_static_f64[140]=p.p51;
        self.scalar_static_f64[141]=p.p70;
        self.scalar_static_f64[142]=p.p52;
        self.scalar_static_f64[143]=p.p71;
        self.scalar_static_f64[144]=p.p53;
        self.scalar_static_f64[145]=p.p66;
        self.scalar_static_f64[146]=p.p54;
        self.scalar_static_f64[147]=p.p67;
        self.scalar_static_f64[148]=p.p55;
        self.scalar_static_f64[149]=p.p68;
        self.scalar_static_f64[150]=p.p59;
        self.scalar_static_f64[151]=p.p72;
        self.scalar_static_f64[152]=p.p60;
        self.scalar_static_f64[153]=p.p73;
        self.scalar_static_f64[154]=p.p61;
        self.scalar_static_f64[155]=p.p74;
        self.scalar_static_f64[156]=p.p58;
        self.scalar_static_f64[157]=p.p57;
        self.scalar_static_f64[158]=(-self.scalar_static_f64[51]);
        self.scalar_static_f64[159]=p.p64;
        self.scalar_static_f64[160]=p.p63;
        self.scalar_static_f64[161]=p.p62;
        self.scalar_static_f64[162]=p.p47;
        self.scalar_static_f64[163]=(-self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=p.p48;
        self.scalar_static_f64[165]=(-self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=p.p49;
        self.scalar_static_f64[167]=(-self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=p.p56;
        self.scalar_static_f64[169]=(-self.scalar_static_f64[20]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[20]*self.scalar_static_f64[162]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[162]*self.scalar_static_f64[169]);
        self.scalar_static_f64[172]=(self.scalar_static_f64[20]*self.scalar_static_f64[164]);
        self.scalar_static_f64[173]=(self.scalar_static_f64[164]*self.scalar_static_f64[169]);
        self.scalar_static_f64[174]=(self.scalar_static_f64[20]*self.scalar_static_f64[166]);
        self.scalar_static_f64[175]=(self.scalar_static_f64[166]*self.scalar_static_f64[169]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[20]*self.scalar_static_f64[168]);
        self.scalar_static_f64[177]=(self.scalar_static_f64[168]*self.scalar_static_f64[169]);
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
        self.scalar_static_f64[178]=(temperature+self.scalar_static_f64[24]);
        self.scalar_static_f64[179]=(if (self.scalar_static_f64[23]!=0.0){self.scalar_static_f64[178]}else{0.0});
        self.scalar_static_f64[180]=(if self.scalar_static_bool[2]{self.scalar_static_f64[25]}else{self.scalar_static_f64[179]});
        self.scalar_static_f64[181]=(self.scalar_static_f64[180]*8.617333262145179e-5);
        self.scalar_static_f64[182]=(self.scalar_static_f64[181]*0.1);
        self.scalar_static_f64[183]=(1.0/self.scalar_static_f64[181]);
        self.scalar_static_f64[184]=(self.scalar_static_f64[181]+self.scalar_static_f64[181]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[184]+self.scalar_static_f64[184]);
        self.scalar_static_f64[186]=(self.scalar_static_f64[181]*self.scalar_static_f64[181]);
        self.scalar_static_f64[187]=(self.scalar_static_f64[186]+self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=(self.scalar_static_f64[186]*16.0);
        self.scalar_static_f64[189]=(self.scalar_static_f64[180]*0.000702);
        self.scalar_static_f64[190]=(self.scalar_static_f64[180]*self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(self.scalar_static_f64[180]+1108.0);
        self.scalar_static_f64[192]=(self.scalar_static_f64[190]/self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=(1.16-self.scalar_static_f64[192]);
        self.scalar_static_f64[194]=(self.scalar_static_f64[180]-self.scalar_static_f64[30]);
        self.scalar_static_f64[195]=(self.scalar_static_f64[180]/self.scalar_static_f64[30]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[194]*self.scalar_static_f64[37]);
        self.scalar_static_f64[197]=(self.scalar_static_f64[36]-self.scalar_static_f64[196]);
        self.scalar_static_f64[198]=f64::powf(self.scalar_static_f64[195],self.scalar_static_f64[39]);
        self.scalar_static_f64[199]=(self.scalar_static_f64[38]*self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=f64::powf(self.scalar_static_f64[195],self.scalar_static_f64[41]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[40]*self.scalar_static_f64[200]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[194]*self.scalar_static_f64[43]);
        self.scalar_static_f64[203]=(1.0+self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=(self.scalar_static_f64[42]*self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(self.scalar_static_f64[195]*self.scalar_static_f64[44]);
        self.scalar_static_f64[206]=(3.0*self.scalar_static_f64[181]);
        self.scalar_static_f64[207]=(self.scalar_static_f64[195]).ln();
        self.scalar_static_f64[208]=(self.scalar_static_f64[206]*self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=(self.scalar_static_f64[205]-self.scalar_static_f64[208]);
        self.scalar_static_f64[210]=(self.scalar_static_f64[35]*self.scalar_static_f64[195]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[209]-self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(self.scalar_static_f64[193]+self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[212]-0.2);
        self.scalar_static_f64[214]=(self.scalar_static_f64[213]*self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=(self.scalar_static_f64[186]+self.scalar_static_f64[214]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[215]).sqrt();
        self.scalar_static_f64[217]=(self.scalar_static_f64[213]+self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=(0.5*self.scalar_static_f64[217]);
        self.scalar_static_f64[219]=(0.2+self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=(self.scalar_static_f64[219]).sqrt();
        self.scalar_static_f64[221]=(1.0/self.scalar_static_f64[201]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[4]*self.scalar_static_f64[201]);
        self.scalar_static_f64[223]=(self.scalar_static_f64[4]*self.scalar_static_f64[204]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[45]/self.scalar_static_f64[204]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[201]*self.scalar_static_f64[48]);
        self.scalar_static_f64[226]=(0.5*self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=(self.scalar_static_f64[183]*self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=(self.scalar_static_f64[227]).ln();
        self.scalar_static_f64[229]=(self.scalar_static_f64[228]-0.6);
        self.scalar_static_f64[230]=(self.scalar_static_f64[181]*self.scalar_static_f64[229]);
        self.scalar_static_f64[231]=(self.scalar_static_f64[197]+self.scalar_static_f64[58]);
        self.scalar_static_f64[232]=(if self.scalar_static_bool[5]{self.scalar_static_f64[231]}else{self.scalar_static_f64[197]});
        self.scalar_static_f64[233]=(if (self.scalar_static_f64[55]!=0.0){self.scalar_static_f64[232]}else{0.0});
        self.scalar_static_f64[234]=(self.scalar_static_f64[60]-self.scalar_static_f64[197]);
        self.scalar_static_f64[235]=(-self.scalar_static_f64[197]);
        self.scalar_static_f64[236]=(if self.scalar_static_bool[5]{self.scalar_static_f64[234]}else{self.scalar_static_f64[235]});
        self.scalar_static_f64[237]=(if self.scalar_static_bool[6]{self.scalar_static_f64[236]}else{self.scalar_static_f64[233]});
        self.scalar_static_f64[238]=(self.scalar_static_f64[199]*self.scalar_static_f64[64]);
        self.scalar_static_f64[239]=(if self.scalar_static_bool[7]{self.scalar_static_f64[238]}else{self.scalar_static_f64[199]});
        self.scalar_static_f64[240]=(self.scalar_static_f64[51]*self.scalar_static_f64[239]);
        self.scalar_static_f64[241]=(self.scalar_static_f64[220]*self.scalar_static_f64[70]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[188]*2.0);
        self.scalar_static_f64[243]=(self.scalar_static_f64[181]/self.scalar_static_f64[225]);
        self.scalar_static_f64[244]=(self.scalar_static_f64[16]*self.scalar_static_f64[241]);
        self.scalar_static_f64[245]=(1.0+self.scalar_static_f64[244]);
        self.scalar_static_f64[246]=(if self.scalar_static_bool[12]{self.scalar_static_f64[245]}else{0.0});
        self.scalar_static_f64[247]=(self.scalar_static_f64[240]*self.scalar_static_f64[246]);
        self.scalar_static_f64[248]=(self.scalar_static_f64[185]+self.scalar_static_f64[185]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[5]*self.scalar_static_f64[248]);
        self.scalar_static_bool[26]=(self.scalar_static_f64[224]>0.0);
        self.scalar_static_f64[250]=(-self.scalar_static_f64[223]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[193]/self.scalar_static_f64[181]);
        self.scalar_static_f64[252]=(self.scalar_static_f64[132]-self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[207]*self.scalar_static_f64[133]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[252]+self.scalar_static_f64[253]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[254]/self.scalar_static_f64[134]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[255]).exp();
        self.scalar_static_f64[257]=(self.scalar_static_f64[256]*self.scalar_static_f64[135]);
        self.scalar_static_f64[258]=(self.scalar_static_f64[256]*self.scalar_static_f64[136]);
        self.scalar_static_f64[259]=(self.scalar_static_f64[256]*self.scalar_static_f64[137]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[194]*self.scalar_static_f64[139]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[138]-self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[194]*self.scalar_static_f64[141]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[140]-self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=(self.scalar_static_f64[194]*self.scalar_static_f64[143]);
        self.scalar_static_f64[265]=(self.scalar_static_f64[142]-self.scalar_static_f64[264]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[194]*self.scalar_static_f64[145]);
        self.scalar_static_f64[267]=(1.0+self.scalar_static_f64[266]);
        self.scalar_static_f64[268]=(self.scalar_static_f64[144]*self.scalar_static_f64[267]);
        self.scalar_static_f64[269]=(self.scalar_static_f64[194]*self.scalar_static_f64[147]);
        self.scalar_static_f64[270]=(1.0+self.scalar_static_f64[269]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[146]*self.scalar_static_f64[270]);
        self.scalar_static_f64[272]=(self.scalar_static_f64[194]*self.scalar_static_f64[149]);
        self.scalar_static_f64[273]=(1.0+self.scalar_static_f64[272]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[148]*self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[195]-1.0);
        self.scalar_static_f64[276]=(self.scalar_static_f64[275]*self.scalar_static_f64[151]);
        self.scalar_static_f64[277]=(1.0+self.scalar_static_f64[276]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[150]*self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(self.scalar_static_f64[275]*self.scalar_static_f64[153]);
        self.scalar_static_f64[280]=(1.0+self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(self.scalar_static_f64[152]*self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[275]*self.scalar_static_f64[155]);
        self.scalar_static_f64[283]=(1.0+self.scalar_static_f64[282]);
        self.scalar_static_f64[284]=(self.scalar_static_f64[154]*self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=(self.scalar_static_f64[126]*self.scalar_static_f64[257]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[130]*self.scalar_static_f64[258]);
        self.scalar_static_f64[287]=(self.scalar_static_f64[285]+self.scalar_static_f64[286]);
        self.scalar_static_f64[288]=(self.scalar_static_f64[51]*self.scalar_static_f64[259]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[287]+self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[181]*self.scalar_static_f64[134]);
        self.scalar_static_f64[291]=(self.scalar_static_f64[259]*self.scalar_static_f64[158]);
        self.scalar_static_f64[292]=(self.scalar_static_f64[181]*self.scalar_static_f64[284]);
        self.scalar_static_f64[293]=(self.scalar_static_f64[181]*self.scalar_static_f64[281]);
        self.scalar_static_f64[294]=(self.scalar_static_f64[181]*self.scalar_static_f64[278]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[116]*self.scalar_static_f64[257]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[122]*self.scalar_static_f64[258]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[295]+self.scalar_static_f64[296]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[288]+self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=(self.scalar_static_f64[126]*self.scalar_static_f64[268]);
        self.scalar_static_f64[300]=(self.scalar_static_f64[130]*self.scalar_static_f64[271]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[51]*self.scalar_static_f64[274]);
        self.scalar_static_f64[302]=(self.scalar_static_f64[116]*self.scalar_static_f64[268]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[122]*self.scalar_static_f64[271]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[195]*self.scalar_static_f64[169]);
        self.scalar_static_f64[305]=(self.scalar_static_f64[20]*self.scalar_static_f64[195]);
        self.scalar_static_f64[306]=(self.scalar_static_f64[304]/self.scalar_static_f64[290]);
        self.scalar_static_f64[307]=(self.scalar_static_f64[305]/self.scalar_static_f64[290]);
        self.scalar_static_f64[308]=(-self.scalar_static_f64[306]);
        self.scalar_static_f64[309]=(-self.scalar_static_f64[307]);
        self.scalar_static_f64[310]=(self.scalar_static_f64[305]/self.scalar_static_f64[292]);
        self.scalar_static_f64[311]=(self.scalar_static_f64[304]/self.scalar_static_f64[292]);
        self.scalar_static_f64[312]=(self.scalar_static_f64[159]*self.scalar_static_f64[310]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[159]*self.scalar_static_f64[311]);
        self.scalar_static_f64[314]=(self.scalar_static_f64[305]/self.scalar_static_f64[293]);
        self.scalar_static_f64[315]=(self.scalar_static_f64[304]/self.scalar_static_f64[293]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[160]*self.scalar_static_f64[314]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[160]*self.scalar_static_f64[315]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[305]/self.scalar_static_f64[294]);
        self.scalar_static_f64[319]=(self.scalar_static_f64[304]/self.scalar_static_f64[294]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[161]*self.scalar_static_f64[318]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[161]*self.scalar_static_f64[319]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[20]/self.scalar_static_f64[261]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[169]/self.scalar_static_f64[261]);
        self.scalar_static_f64[324]=(self.scalar_static_f64[20]/self.scalar_static_f64[263]);
        self.scalar_static_f64[325]=(self.scalar_static_f64[169]/self.scalar_static_f64[263]);
        self.scalar_static_f64[326]=(self.scalar_static_f64[20]/self.scalar_static_f64[265]);
        self.scalar_static_f64[327]=(self.scalar_static_f64[169]/self.scalar_static_f64[265]);
        self.scalar_static_f64[328]=(self.scalar_static_f64[170]/self.scalar_static_f64[261]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[171]/self.scalar_static_f64[261]);
        self.scalar_static_f64[330]=(-self.scalar_static_f64[328]);
        self.scalar_static_f64[331]=(-self.scalar_static_f64[329]);
        self.scalar_static_f64[332]=(self.scalar_static_f64[299]*self.scalar_static_f64[330]);
        self.scalar_static_f64[333]=(self.scalar_static_f64[299]*self.scalar_static_f64[331]);
        self.scalar_static_f64[334]=(self.scalar_static_f64[172]/self.scalar_static_f64[263]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[173]/self.scalar_static_f64[263]);
        self.scalar_static_f64[336]=(-self.scalar_static_f64[334]);
        self.scalar_static_f64[337]=(-self.scalar_static_f64[335]);
        self.scalar_static_f64[338]=(self.scalar_static_f64[300]*self.scalar_static_f64[336]);
        self.scalar_static_f64[339]=(self.scalar_static_f64[300]*self.scalar_static_f64[337]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[174]/self.scalar_static_f64[265]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[175]/self.scalar_static_f64[265]);
        self.scalar_static_f64[342]=(-self.scalar_static_f64[340]);
        self.scalar_static_f64[343]=(-self.scalar_static_f64[341]);
        self.scalar_static_f64[344]=(self.scalar_static_f64[301]*self.scalar_static_f64[342]);
        self.scalar_static_f64[345]=(self.scalar_static_f64[301]*self.scalar_static_f64[343]);
        self.scalar_static_f64[346]=(self.scalar_static_f64[302]*self.scalar_static_f64[330]);
        self.scalar_static_f64[347]=(self.scalar_static_f64[302]*self.scalar_static_f64[331]);
        self.scalar_static_f64[348]=(self.scalar_static_f64[303]*self.scalar_static_f64[336]);
        self.scalar_static_f64[349]=(self.scalar_static_f64[303]*self.scalar_static_f64[337]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
