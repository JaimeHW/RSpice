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
    pub p72: f64, pub p73: f64, pub p74: f64, pub p75: f64, pub p76: f64, pub p77: f64, pub p78: f64, pub p79: f64, 
    pub p80: f64, pub p81: f64, pub p82: f64, pub p83: f64, pub p84: f64, pub p85: f64, 
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 86] = [
                1.0, 0.0, 0.0, 25.0, 0.0, 0.0, 2.0, 0.0,
                0.05, -0.2, 0.2, 0.8, 0.0, 0.0, 0.1, 1.0,
                0.001, 0.0, 0.1, 4.0, 0.0, 20.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.2, 0.0, 0.2, 0.0, 1.0, 0.0, 5e-5, 15.0,
                1.0, 0.7, 0.05, 0.05, 0.0, 0.05, 0.05, 0.05,
                0.0, 0.0, 0.0, 0.0, 1000.0, 10000.0, 0.0, 100000.0,
                0.0, 0.001, 0.0001, -0.002, -0.002, 0.002, 0.002, 0.0,
                0.0, 0.0, 0.003, 0.001, 0.001, -0.001, 0.0, 0.5,
                1.0, 0.9, 0.0, 0.0, 1.0, 1.0, 25.0, 0.1,
                1.0, 1e-14, 60000.0, 0.3, 0.1, 25.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 86);
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
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 88] = [
    ("noise", 0), ("selft", 1), ("trise", 2), ("temp", 3), ("idsmod", 4), ("igmod", 5), ("capmod", 6), ("noimod", 7), ("ipk0", 8), ("vpks", 9), ("dvpks", 10), ("p1", 11), ("p2", 12), ("p3", 13), ("alphar", 14), ("alphas", 15), 
    ("lambda", 16), ("lvg", 17), ("b1", 18), ("b2", 19), ("lsb0", 20), ("vtr", 21), ("vsb2", 22), ("cds", 23), ("cgspi", 24), ("cgs0", 25), ("cgdpi", 26), ("cgdpe", 27), ("cgd0", 28), ("p10", 29), ("p11", 30), ("p20", 31), 
    ("p21", 32), ("p30", 33), ("p31", 34), ("p40", 35), ("p41", 36), ("p111", 37), ("ij", 38), ("pg", 39), ("ne", 40), ("vjg", 41), ("rg", 42), ("rd", 43), ("rd2", 44), ("ri", 45), ("rs", 46), ("rgd", 47), 
    ("ld", 48), ("ls", 49), ("lg", 50), ("tau", 51), ("rcmin", 52), ("rc", 53), ("crf", 54), ("rcin", 55), ("crfin", 56), ("rth", 57), ("rtherm", 57), ("cth", 58), ("ctherm", 58), ("tcipk0", 59), ("tcp1", 60), ("tccgs0", 61), 
    ("tccgd0", 62), ("tclsb0", 63), ("tcrc", 64), ("tccrf", 65), ("tcrs", 66), ("tcrtherm", 67), ("tcvpk", 68), ("tcvjg", 69), ("tcvtr", 70), ("noiser", 71), ("noisep", 72), ("noisec", 73), ("fnc", 74), ("kf", 75), ("af", 76), ("ffe", 77), 
    ("td", 78), ("td1", 79), ("tmn", 80), ("klf", 81), ("fgr", 82), ("np", 83), ("lw", 84), ("tnom", 85), 
];

const PARAMETER_DISPLAY_NAMES: [&str; 86] = [
    "Noise", "Selft", "Trise", "Temp", "Idsmod", "Igmod", "Capmod", "Noimod", "Ipk0", "Vpks", "Dvpks", "P1", "P2", "P3", "Alphar", "Alphas", 
    "Lambda", "Lvg", "B1", "B2", "Lsb0", "Vtr", "Vsb2", "Cds", "Cgspi", "Cgs0", "Cgdpi", "Cgdpe", "Cgd0", "P10", "P11", "P20", 
    "P21", "P30", "P31", "P40", "P41", "P111", "Ij", "Pg", "Ne", "Vjg", "Rg", "Rd", "Rd2", "Ri", "Rs", "Rgd", 
    "Ld", "Ls", "Lg", "Tau", "Rcmin", "Rc", "Crf", "Rcin", "Crfin", "Rth", "Cth", "Tcipk0", "Tcp1", "Tccgs0", "Tccgd0", "Tclsb0", 
    "Tcrc", "Tccrf", "Tcrs", "TcRtherm", "TcVpk", "TcVjg", "TcVtr", "NoiseR", "NoiseP", "NoiseC", "Fnc", "Kf", "Af", "Ffe", "Td", "Td1", 
    "Tmn", "Klf", "Fgr", "Np", "Lw", "Tnom", 
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 86] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -273.15, label: "-273.15" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), 
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-7, label: "1e-7" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: -0.003, label: "-0.003" }), Some(ParameterBound { value: -0.003, label: "-0.003" }), Some(ParameterBound { value: -0.002, label: "-0.002" }), Some(ParameterBound { value: -0.002, label: "-0.002" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -0.1, label: "-0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, 
    None, None, None, None, None, Some(ParameterBound { value: -273.15, label: "-273.15" }), 
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 86] = [
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), 
    Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.002, label: "0.002" }), Some(ParameterBound { value: 0.002, label: "0.002" }), Some(ParameterBound { value: 0.01, label: "0.01" }), 
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.01, label: "0.01" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, 
];

const PARAMETER_RANGE_FLAGS: [u8; 86] = [
    0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 1, 0, 
    0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 3, 3, 2, 3, 3, 3, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 86] = [
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], 
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
    pub nodes: [usize; 16],
    pub branches: [usize; 19],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 86]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 17]>,
    pub(crate) ddt_state_previous: Box<[f64; 17]>,
    pub(crate) ddt_state_older: Box<[f64; 17]>,
    pub(crate) ddt_state_initialized: Box<[bool; 17]>,
    pub(crate) ddt_derivative_current: Box<[f64; 17]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 17]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 161]>,
    pub(crate) scalar_static_bool: Box<[bool; 57]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 13;
    pub const NODE_COUNT: usize = 16;
    pub const INTERNAL_NODE_NAMES: [&str; 13] = ["di", "gi", "si", "sii", "gdi", "gsi", "bi", "rf", "t", "xt1", "xt2", "ia", "ib"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 86;
    pub const VARIABLE_COUNT: usize = 125;
    pub const DDT_STATE_COUNT: usize = 17;
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
            scalar_static_f64: boxed_zero_f64_array::<161>(),
            scalar_static_bool: boxed_zero_bool_array::<57>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'angelov'", name));
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
        self.recompute_instance_static(); self.invalidate_temperature_static(); 
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
        self.scalar_static_f64[0]=if param_given[3] { 1.0 } else { 0.0 };
        self.scalar_static_f64[1]=p.p3;
        self.scalar_static_f64[2]=(self.scalar_static_f64[1]+273.15);
        self.scalar_static_f64[3]=(if (self.scalar_static_f64[0]!=0.0){self.scalar_static_f64[2]}else{0.0});
        self.scalar_static_bool[0]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[4]=p.p2;
        self.scalar_static_f64[5]=if param_given[85] { 1.0 } else { 0.0 };
        self.scalar_static_f64[6]=p.p85;
        self.scalar_static_f64[7]=(273.15+self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=(if (self.scalar_static_f64[5]!=0.0){self.scalar_static_f64[7]}else{0.0});
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[5]!=0.0));
        self.scalar_static_f64[9]=(if self.scalar_static_bool[1]{300.15}else{self.scalar_static_f64[8]});
        self.scalar_static_f64[10]=p.p1;
        self.scalar_static_f64[11]=p.p57;
        self.scalar_static_bool[2]=(self.scalar_static_f64[11]>0.0);
        self.scalar_static_f64[12]=p.p8;
        self.scalar_static_f64[13]=p.p59;
        self.scalar_static_f64[14]=p.p11;
        self.scalar_static_f64[15]=p.p60;
        self.scalar_static_f64[16]=p.p20;
        self.scalar_static_f64[17]=p.p63;
        self.scalar_static_f64[18]=p.p25;
        self.scalar_static_f64[19]=p.p61;
        self.scalar_static_f64[20]=p.p28;
        self.scalar_static_f64[21]=p.p62;
        self.scalar_static_f64[22]=p.p53;
        self.scalar_static_f64[23]=p.p64;
        self.scalar_static_f64[24]=p.p54;
        self.scalar_static_f64[25]=p.p65;
        self.scalar_static_f64[26]=p.p9;
        self.scalar_static_f64[27]=p.p68;
        self.scalar_static_f64[28]=p.p29;
        self.scalar_static_f64[29]=p.p30;
        self.scalar_static_f64[30]=(self.scalar_static_f64[27]*self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=p.p35;
        self.scalar_static_f64[32]=p.p36;
        self.scalar_static_f64[33]=(self.scalar_static_f64[27]*self.scalar_static_f64[32]);
        self.scalar_static_f64[34]=p.p41;
        self.scalar_static_f64[35]=p.p69;
        self.scalar_static_f64[36]=p.p21;
        self.scalar_static_f64[37]=p.p70;
        self.scalar_static_f64[38]=if param_given[39] { 1.0 } else { 0.0 };
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[38]!=0.0));
        self.scalar_static_f64[39]=if param_given[40] { 1.0 } else { 0.0 };
        self.scalar_static_bool[4]=(self.scalar_static_bool[3]&&(self.scalar_static_f64[39]!=0.0));
        self.scalar_static_f64[40]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[41]=p.p40;
        self.scalar_static_f64[42]=(0.5/self.scalar_static_f64[41]);
        self.scalar_static_bool[5]=(!(self.scalar_static_f64[40]!=0.0));
        self.scalar_static_f64[43]=p.p39;
        self.scalar_static_f64[44]=p.p19;
        self.scalar_static_f64[45]=p.p18;
        self.scalar_static_f64[46]=p.p10;
        self.scalar_static_f64[47]=p.p15;
        self.scalar_static_f64[48]=p.p22;
        self.scalar_static_f64[49]=p.p12;
        self.scalar_static_f64[50]=p.p13;
        self.scalar_static_f64[51]=p.p14;
        self.scalar_static_f64[52]=p.p4;
        self.scalar_static_bool[6]=(0.0==self.scalar_static_f64[52]);
        self.scalar_static_f64[53]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_bool[7]=(1.0==self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(if self.scalar_static_bool[7]{1.0}else{0.0});
        self.scalar_static_bool[8]=(self.scalar_static_f64[52]==2.0);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_bool[9]=(self.scalar_static_f64[52]==3.0);
        self.scalar_static_f64[56]=(if self.scalar_static_bool[9]{1.0}else{0.0});
        self.scalar_static_f64[57]=p.p16;
        self.scalar_static_bool[10]=(!(self.scalar_static_f64[53]!=0.0));
        self.scalar_static_bool[11]=((self.scalar_static_f64[54]!=0.0)&&self.scalar_static_bool[10]);
        self.scalar_static_f64[58]=p.p17;
        self.scalar_static_bool[12]=((self.scalar_static_f64[53]!=0.0)||(self.scalar_static_f64[54]!=0.0));
        self.scalar_static_bool[13]=(!self.scalar_static_bool[12]);
        self.scalar_static_bool[14]=((self.scalar_static_f64[55]!=0.0)&&self.scalar_static_bool[13]);
        self.scalar_static_bool[15]=((self.scalar_static_f64[55]!=0.0)||self.scalar_static_bool[12]);
        self.scalar_static_bool[16]=(!self.scalar_static_bool[15]);
        self.scalar_static_bool[17]=((self.scalar_static_f64[56]!=0.0)&&self.scalar_static_bool[16]);
        self.scalar_static_bool[18]=(self.scalar_static_bool[6]||self.scalar_static_bool[7]);
        self.scalar_static_f64[59]=(if self.scalar_static_bool[18]{1.0}else{0.0});
        self.scalar_static_f64[60]=p.p52;
        self.scalar_static_f64[61]=p.p43;
        self.scalar_static_f64[62]=p.p44;
        self.scalar_static_f64[63]=p.p46;
        self.scalar_static_bool[19]=(!(self.scalar_static_f64[59]!=0.0));
        self.scalar_static_f64[64]=p.p66;
        self.scalar_static_f64[65]=p.p5;
        self.scalar_static_bool[20]=(0.0==self.scalar_static_f64[65]);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[20]{1.0}else{0.0});
        self.scalar_static_bool[21]=(!(self.scalar_static_f64[66]!=0.0));
        self.scalar_static_bool[22]=(1.0==self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_bool[23]=(self.scalar_static_bool[21]&&(self.scalar_static_f64[67]!=0.0));
        self.scalar_static_bool[24]=(!(self.scalar_static_f64[67]!=0.0));
        self.scalar_static_bool[25]=(self.scalar_static_bool[21]&&self.scalar_static_bool[24]);
        self.scalar_static_f64[68]=p.p38;
        self.scalar_static_f64[69]=p.p37;
        self.scalar_static_f64[70]=p.p31;
        self.scalar_static_f64[71]=p.p32;
        self.scalar_static_f64[72]=p.p33;
        self.scalar_static_f64[73]=p.p34;
        self.scalar_static_f64[74]=p.p6;
        self.scalar_static_bool[26]=(0.0==self.scalar_static_f64[74]);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=(1.0==self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_bool[28]=(2.0==self.scalar_static_f64[74]);
        self.scalar_static_f64[77]=(if self.scalar_static_bool[28]{1.0}else{0.0});
        self.scalar_static_f64[78]=p.p24;
        self.scalar_static_f64[79]=(if (self.scalar_static_f64[75]!=0.0){self.scalar_static_f64[78]}else{0.0});
        self.scalar_static_f64[80]=p.p26;
        self.scalar_static_f64[81]=(if (self.scalar_static_f64[75]!=0.0){self.scalar_static_f64[80]}else{0.0});
        self.scalar_static_bool[29]=(!(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_bool[30]=((self.scalar_static_f64[76]!=0.0)&&self.scalar_static_bool[29]);
        self.scalar_static_f64[82]=(2.0*self.scalar_static_f64[69]);
        self.scalar_static_bool[31]=((self.scalar_static_f64[75]!=0.0)||(self.scalar_static_f64[76]!=0.0));
        self.scalar_static_bool[32]=(!self.scalar_static_bool[31]);
        self.scalar_static_bool[33]=((self.scalar_static_f64[77]!=0.0)&&self.scalar_static_bool[32]);
        self.scalar_static_bool[34]=(self.scalar_static_f64[22]>0.0);
        self.scalar_static_f64[83]=(if self.scalar_static_bool[34]{1.0}else{0.0});
        self.scalar_static_f64[84]=p.p55;
        self.scalar_static_bool[35]=(self.scalar_static_f64[84]>0.0);
        self.scalar_static_f64[85]=(if self.scalar_static_bool[35]{1.0}else{0.0});
        self.scalar_static_f64[86]=p.p47;
        self.scalar_static_bool[36]=(self.scalar_static_f64[86]>0.0);
        self.scalar_static_f64[87]=(if self.scalar_static_bool[36]{1.0}else{0.0});
        self.scalar_static_f64[88]=p.p45;
        self.scalar_static_bool[37]=(self.scalar_static_f64[88]>0.0);
        self.scalar_static_f64[89]=(if self.scalar_static_bool[37]{1.0}else{0.0});
        self.scalar_static_f64[90]=p.p42;
        self.scalar_static_bool[38]=(self.scalar_static_f64[90]>0.0);
        self.scalar_static_f64[91]=(if self.scalar_static_bool[38]{1.0}else{0.0});
        self.scalar_static_f64[92]=p.p50;
        self.scalar_static_bool[39]=(self.scalar_static_f64[92]>0.0);
        self.scalar_static_f64[93]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_bool[40]=(self.scalar_static_f64[63]>0.0);
        self.scalar_static_f64[94]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_bool[41]=(self.scalar_static_f64[61]>0.0);
        self.scalar_static_bool[42]=(self.scalar_static_f64[62]>0.0);
        self.scalar_static_bool[43]=(self.scalar_static_bool[41]||self.scalar_static_bool[42]);
        self.scalar_static_f64[95]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_f64[96]=p.p48;
        self.scalar_static_bool[44]=(self.scalar_static_f64[96]>0.0);
        self.scalar_static_f64[97]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_f64[98]=p.p7;
        self.scalar_static_bool[45]=(0.0==self.scalar_static_f64[98]);
        self.scalar_static_f64[99]=(if self.scalar_static_bool[45]{1.0}else{0.0});
        self.scalar_static_bool[46]=(1.0==self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_bool[47]=(!(self.scalar_static_f64[99]!=0.0));
        self.scalar_static_bool[48]=((self.scalar_static_f64[100]!=0.0)&&self.scalar_static_bool[47]);
        self.scalar_static_f64[101]=p.p0;
        self.scalar_static_bool[49]=(self.scalar_static_bool[48]&&(self.scalar_static_f64[101]!=0.0));
        self.scalar_static_f64[102]=p.p72;
        self.scalar_static_f64[103]=p.p71;
        self.scalar_static_f64[104]=p.p73;
        self.scalar_static_f64[105]=(self.scalar_static_f64[102]*self.scalar_static_f64[103]);
        self.scalar_static_f64[106]=(self.scalar_static_f64[105]).sqrt();
        self.scalar_static_bool[50]=((self.scalar_static_f64[10]!=0.0)&&(self.scalar_static_f64[11]!=0.0));
        self.scalar_static_f64[107]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[108]=p.p51;
        self.scalar_static_f64[109]=(self.scalar_static_f64[108]/3.0);
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[77]!=0.0));
        self.scalar_static_f64[110]=p.p27;
        self.scalar_static_f64[111]=p.p23;
        self.scalar_static_f64[112]=p.p56;
        self.scalar_static_bool[52]=(!(self.scalar_static_f64[91]!=0.0));
        self.scalar_static_bool[53]=((self.scalar_static_f64[93]!=0.0)&&self.scalar_static_bool[52]);
        self.scalar_static_f64[113]=p.p49;
        self.scalar_static_bool[54]=(!(self.scalar_static_f64[95]!=0.0));
        self.scalar_static_bool[55]=((self.scalar_static_f64[97]!=0.0)&&self.scalar_static_bool[54]);
        self.scalar_static_f64[114]=p.p58;
        self.scalar_static_bool[56]=(!(self.scalar_static_f64[107]!=0.0));
        self.scalar_static_f64[115]=(-self.scalar_static_f64[44]);
        self.scalar_static_f64[116]=(-self.scalar_static_f64[47]);
        self.scalar_static_f64[117]=(-self.scalar_static_f64[48]);
        self.scalar_static_f64[118]=(-self.scalar_static_f64[57]);
        self.scalar_static_f64[119]=(if self.scalar_static_bool[11]{0.0}else{1.0});
        self.scalar_static_f64[120]=(self.scalar_static_f64[49]*self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[122]=(if self.scalar_static_bool[17]{1.0}else{self.scalar_static_f64[121]});
        self.scalar_static_f64[123]=(if (self.scalar_static_f64[66]!=0.0){0.0}else{self.scalar_static_f64[122]});
        self.scalar_static_f64[124]=(if (self.scalar_static_f64[66]!=0.0){-1.0}else{0.0});
        self.scalar_static_f64[125]=(if (self.scalar_static_f64[66]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[126]=(if self.scalar_static_bool[21]{0.0}else{self.scalar_static_f64[123]});
        self.scalar_static_f64[127]=(-self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[68]*self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(-self.scalar_static_f64[29]);
        self.scalar_static_f64[130]=(-self.scalar_static_f64[69]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[129]+self.scalar_static_f64[130]);
        self.scalar_static_f64[132]=(-self.scalar_static_f64[71]);
        self.scalar_static_f64[133]=(-self.scalar_static_f64[73]);
        self.scalar_static_f64[134]=(-self.scalar_static_f64[32]);
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]-self.scalar_static_f64[69]);
        self.scalar_static_f64[136]=(-self.scalar_static_f64[82]);
        self.scalar_static_f64[137]=(-self.scalar_static_f64[78]);
        self.scalar_static_f64[138]=(-self.scalar_static_f64[80]);
        self.scalar_static_f64[139]=(-self.scalar_static_f64[110]);
        self.scalar_static_f64[140]=(-self.scalar_static_f64[111]);
        self.scalar_static_f64[141]=(-self.scalar_static_f64[112]);
        self.scalar_static_f64[142]=(-1.0/self.scalar_static_f64[84]);
        self.scalar_static_f64[143]=(1.0/self.scalar_static_f64[84]);
        self.scalar_static_f64[144]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[142]}else{0.0});
        self.scalar_static_f64[145]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[143]}else{0.0});
        self.scalar_static_f64[146]=(1.0/self.scalar_static_f64[86]);
        self.scalar_static_f64[147]=(-1.0/self.scalar_static_f64[86]);
        self.scalar_static_f64[148]=(if (self.scalar_static_f64[87]!=0.0){self.scalar_static_f64[146]}else{0.0});
        self.scalar_static_f64[149]=(if (self.scalar_static_f64[87]!=0.0){self.scalar_static_f64[147]}else{0.0});
        self.scalar_static_f64[150]=(1.0/self.scalar_static_f64[88]);
        self.scalar_static_f64[151]=(-1.0/self.scalar_static_f64[88]);
        self.scalar_static_f64[152]=(if (self.scalar_static_f64[89]!=0.0){self.scalar_static_f64[150]}else{0.0});
        self.scalar_static_f64[153]=(if (self.scalar_static_f64[89]!=0.0){self.scalar_static_f64[151]}else{0.0});
        self.scalar_static_f64[154]=(if (self.scalar_static_f64[91]!=0.0){self.scalar_static_f64[90]}else{0.0});
        self.scalar_static_f64[155]=(if self.scalar_static_bool[49]{1.0}else{0.0});
        self.scalar_static_f64[156]=(1.0/self.scalar_static_f64[11]);
        self.scalar_static_f64[157]=(if (self.scalar_static_f64[107]!=0.0){self.scalar_static_f64[156]}else{0.0});
        self.scalar_static_f64[158]=(if self.scalar_static_bool[56]{1e-12}else{0.0});
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
        self.scalar_static_f64[159]=(temperature+self.scalar_static_f64[4]);
        self.scalar_static_f64[160]=(if self.scalar_static_bool[0]{self.scalar_static_f64[159]}else{self.scalar_static_f64[3]});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
