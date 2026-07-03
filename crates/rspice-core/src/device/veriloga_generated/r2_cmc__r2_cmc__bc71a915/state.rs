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
    pub p40: f64, pub p41: f64, pub p42: f64, 
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 43] = [
                1e-6, 1e-6, 100.0, 1.0, 1.0, 0.0, 1.0, 1.0,
                2.0, 1.0, 0.0, -100.0, 500.0, 0.001, 1002.0, 27.0,
                100.0, 0.0, 9900000000.0, 0.0, 9900000000.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0,
                0.0, 100.0, -100.0, 500.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 43);
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 45] = [
    ("w", 0), ("l", 1), ("r", 2), ("c1", 3), ("c2", 4), ("trise", 5), ("dtemp", 5), ("dra", 5), ("isnoisy", 6), ("version", 7), ("revision", 8), ("scale", 9), ("shrink", 10), ("tmin", 11), ("tmax", 12), ("rthresh", 13), 
    ("level", 14), ("tnom", 15), ("rsh", 16), ("lmin", 17), ("lmax", 18), ("wmin", 19), ("wmax", 20), ("xw", 21), ("xl", 22), ("dxle", 23), ("sw_efgeo", 24), ("q3", 25), ("p3", 26), ("q2", 27), ("p2", 28), ("kfn", 29), 
    ("afn", 30), ("bfn", 31), ("sw_fngeo", 32), ("jmax", 33), ("tminclip", 34), ("tmaxclip", 35), ("tc1", 36), ("tc2", 37), ("tc1l", 38), ("tc2l", 39), ("tc1w", 40), ("tc2w", 41), ("tc1kfn", 42), 
];

const PARAMETER_DISPLAY_NAMES: [&str; 43] = [
    "w", "l", "r", "c1", "c2", "trise", "isnoisy", "version", "revision", "scale", "shrink", "tmin", "tmax", "rthresh", "level", "tnom", 
    "rsh", "lmin", "lmax", "wmin", "wmax", "xw", "xl", "dxle", "sw_efgeo", "q3", "p3", "q2", "p2", "kfn", "afn", "bfn", 
    "sw_fngeo", "jmax", "tminclip", "tmaxclip", "tc1", "tc2", "tc1l", "tc2l", "tc1w", "tc2w", "tc1kfn", 
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 43] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -250.0, label: "-250.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), None, None, None, None, 
    None, None, None, 
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 43] = [
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, 
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, Some(ParameterBound { value: 1000.0, label: "1000.0" }), 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, 
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 27.0, label: "27.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, None, 
    None, None, None, 
];

const PARAMETER_RANGE_FLAGS: [u8; 43] = [
    2, 2, 2, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 3, 0, 0, 3, 2, 3, 2, 3, 0, 0, 0, 0, 2, 2, 2, 2, 2, 3, 3, 
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 43] = [
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
    pub nodes: [usize; 2],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 43]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 0]>,
    pub(crate) ddt_state_previous: Box<[f64; 0]>,
    pub(crate) ddt_state_older: Box<[f64; 0]>,
    pub(crate) ddt_state_initialized: Box<[bool; 0]>,
    pub(crate) ddt_derivative_current: Box<[f64; 0]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 0]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 188]>,
    pub(crate) scalar_static_bool: Box<[bool; 66]>,
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
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 2;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 43;
    pub const VARIABLE_COUNT: usize = 86;
    pub const DDT_STATE_COUNT: usize = 0;
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
            scalar_static_f64: boxed_zero_f64_array::<188>(),
            scalar_static_bool: boxed_zero_bool_array::<66>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'r2_cmc'", name));
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
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=if param_given[9]{1.0}else{0.0};
        self.scalar_static_f64[1]=p.p9;
        self.scalar_static_f64[2]=(if (self.scalar_static_f64[0]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_bool[0]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[3]=(if self.scalar_static_bool[0]{1.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[4]=if param_given[10]{1.0}else{0.0};
        self.scalar_static_f64[5]=p.p10;
        self.scalar_static_f64[6]=(0.01*self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=(1.0-self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[7]}else{0.0});
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[9]=(if self.scalar_static_bool[1]{1.0}else{self.scalar_static_f64[8]});
        self.scalar_static_f64[10]=(self.scalar_static_f64[3]*self.scalar_static_f64[9]);
        self.scalar_static_f64[11]=(self.scalar_static_f64[10]*1000000.0);
        self.scalar_static_f64[12]=p.p15;
        self.scalar_static_f64[13]=(273.15+self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=p.p5;
        self.scalar_static_f64[15]=p.p34;
        self.scalar_static_f64[16]=(1.0+self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=p.p35;
        self.scalar_static_f64[18]=(self.scalar_static_f64[17]-1.0);
        self.scalar_static_f64[19]=p.p3;
        self.scalar_static_f64[20]=p.p4;
        self.scalar_static_bool[2]=((self.scalar_static_f64[19]!=0.0)&&(self.scalar_static_f64[20]!=0.0));
        self.scalar_static_f64[21]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[22]=p.p22;
        self.scalar_static_f64[23]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_bool[3]=((self.scalar_static_f64[19]!=0.0)||(self.scalar_static_f64[20]!=0.0));
        self.scalar_static_f64[24]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_bool[4]=(!(self.scalar_static_f64[21]!=0.0));
        self.scalar_static_bool[5]=((self.scalar_static_f64[24]!=0.0)&&self.scalar_static_bool[4]);
        self.scalar_static_f64[25]=(self.scalar_static_f64[22]*0.5);
        self.scalar_static_f64[26]=(if self.scalar_static_bool[5]{self.scalar_static_f64[25]}else{self.scalar_static_f64[23]});
        self.scalar_static_bool[6]=(!(self.scalar_static_f64[24]!=0.0));
        self.scalar_static_bool[7]=(self.scalar_static_bool[4]&&self.scalar_static_bool[6]);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[7]{0.0}else{self.scalar_static_f64[26]});
        self.scalar_static_f64[28]=if param_given[1]{1.0}else{0.0};
        self.scalar_static_f64[29]=if param_given[2]{1.0}else{0.0};
        self.scalar_static_bool[8]=((self.scalar_static_f64[28]!=0.0)&&(self.scalar_static_f64[29]!=0.0));
        self.scalar_static_f64[30]=if param_given[0]{1.0}else{0.0};
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[30]!=0.0));
        self.scalar_static_bool[10]=(self.scalar_static_bool[8]&&self.scalar_static_bool[9]);
        self.scalar_static_f64[31]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[32]=p.p2;
        self.scalar_static_bool[11]=(0.0==self.scalar_static_f64[32]);
        self.scalar_static_f64[33]=p.p1;
        self.scalar_static_bool[12]=(0.0==self.scalar_static_f64[33]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[11]||self.scalar_static_bool[12]);
        self.scalar_static_f64[34]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_bool[14]=((self.scalar_static_f64[31]!=0.0)&&(self.scalar_static_f64[34]!=0.0));
        self.scalar_static_f64[35]=p.p0;
        self.scalar_static_f64[36]=(self.scalar_static_f64[11]*self.scalar_static_f64[35]);
        self.scalar_static_f64[37]=(if self.scalar_static_bool[14]{self.scalar_static_f64[36]}else{0.0});
        self.scalar_static_f64[38]=p.p21;
        self.scalar_static_f64[39]=(self.scalar_static_f64[37]+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=(if self.scalar_static_bool[14]{self.scalar_static_f64[39]}else{0.0});
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[34]!=0.0));
        self.scalar_static_bool[16]=((self.scalar_static_f64[31]!=0.0)&&self.scalar_static_bool[15]);
        self.scalar_static_f64[41]=(self.scalar_static_f64[11]*self.scalar_static_f64[33]);
        self.scalar_static_f64[42]=(if self.scalar_static_bool[16]{self.scalar_static_f64[41]}else{0.0});
        self.scalar_static_f64[43]=(self.scalar_static_f64[27]+self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=(if self.scalar_static_bool[16]{self.scalar_static_f64[43]}else{0.0});
        self.scalar_static_bool[17]=(self.scalar_static_f64[44]>0.0);
        self.scalar_static_f64[45]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=(self.scalar_static_bool[16]&&(self.scalar_static_f64[45]!=0.0));
        self.scalar_static_f64[46]=p.p16;
        self.scalar_static_f64[47]=(self.scalar_static_f64[46]/self.scalar_static_f64[32]);
        self.scalar_static_f64[48]=(self.scalar_static_f64[44]*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=(if self.scalar_static_bool[18]{self.scalar_static_f64[48]}else{self.scalar_static_f64[40]});
        self.scalar_static_f64[50]=(self.scalar_static_f64[49]-self.scalar_static_f64[38]);
        self.scalar_static_f64[51]=(if self.scalar_static_bool[18]{self.scalar_static_f64[50]}else{self.scalar_static_f64[37]});
        self.scalar_static_f64[52]=(if self.scalar_static_bool[18]{self.scalar_static_f64[32]}else{0.0});
        self.scalar_static_bool[19]=(!(self.scalar_static_f64[45]!=0.0));
        self.scalar_static_bool[20]=(self.scalar_static_bool[16]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[53]=(if self.scalar_static_bool[20]{self.scalar_static_f64[36]}else{self.scalar_static_f64[51]});
        self.scalar_static_f64[54]=(self.scalar_static_f64[38]+self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[20]{self.scalar_static_f64[54]}else{self.scalar_static_f64[49]});
        self.scalar_static_f64[56]=(if self.scalar_static_bool[20]{0.0}else{self.scalar_static_f64[52]});
        self.scalar_static_bool[21]=(!(self.scalar_static_f64[28]!=0.0));
        self.scalar_static_bool[22]=((self.scalar_static_f64[29]!=0.0)&&self.scalar_static_bool[21]);
        self.scalar_static_f64[57]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[58]=(if self.scalar_static_bool[11]{1.0}else{0.0});
        self.scalar_static_bool[23]=(!(self.scalar_static_f64[31]!=0.0));
        self.scalar_static_bool[24]=((self.scalar_static_f64[57]!=0.0)&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=((self.scalar_static_f64[58]!=0.0)&&self.scalar_static_bool[24]);
        self.scalar_static_f64[59]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[42]});
        self.scalar_static_f64[60]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[44]});
        self.scalar_static_f64[61]=(if self.scalar_static_bool[25]{self.scalar_static_f64[36]}else{self.scalar_static_f64[53]});
        self.scalar_static_f64[62]=(self.scalar_static_f64[38]+self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(if self.scalar_static_bool[25]{self.scalar_static_f64[62]}else{self.scalar_static_f64[55]});
        self.scalar_static_f64[64]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[56]});
        self.scalar_static_bool[26]=(0.0==self.scalar_static_f64[35]);
        self.scalar_static_f64[65]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=(!(self.scalar_static_f64[58]!=0.0));
        self.scalar_static_bool[28]=(self.scalar_static_bool[24]&&self.scalar_static_bool[27]);
        self.scalar_static_bool[29]=((self.scalar_static_f64[65]!=0.0)&&self.scalar_static_bool[28]);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[61]});
        self.scalar_static_f64[67]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[63]});
        self.scalar_static_f64[68]=(if self.scalar_static_bool[29]{self.scalar_static_f64[41]}else{self.scalar_static_f64[59]});
        self.scalar_static_f64[69]=(self.scalar_static_f64[27]+self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(if self.scalar_static_bool[29]{self.scalar_static_f64[69]}else{self.scalar_static_f64[60]});
        self.scalar_static_f64[71]=(if self.scalar_static_bool[29]{1e99}else{self.scalar_static_f64[64]});
        self.scalar_static_bool[30]=(!(self.scalar_static_f64[65]!=0.0));
        self.scalar_static_bool[31]=(self.scalar_static_bool[28]&&self.scalar_static_bool[30]);
        self.scalar_static_f64[72]=(if self.scalar_static_bool[31]{self.scalar_static_f64[36]}else{self.scalar_static_f64[66]});
        self.scalar_static_f64[73]=(self.scalar_static_f64[38]+self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(if self.scalar_static_bool[31]{self.scalar_static_f64[73]}else{self.scalar_static_f64[67]});
        self.scalar_static_bool[32]=(self.scalar_static_f64[74]>0.0);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[32]{1.0}else{0.0});
        self.scalar_static_bool[33]=(self.scalar_static_bool[31]&&(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_f64[76]=(self.scalar_static_f64[32]/self.scalar_static_f64[46]);
        self.scalar_static_f64[77]=(self.scalar_static_f64[74]*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(if self.scalar_static_bool[33]{self.scalar_static_f64[77]}else{self.scalar_static_f64[70]});
        self.scalar_static_f64[79]=(self.scalar_static_f64[78]-self.scalar_static_f64[27]);
        self.scalar_static_f64[80]=(if self.scalar_static_bool[33]{self.scalar_static_f64[79]}else{self.scalar_static_f64[68]});
        self.scalar_static_f64[81]=(if self.scalar_static_bool[33]{self.scalar_static_f64[32]}else{self.scalar_static_f64[71]});
        self.scalar_static_bool[34]=(!(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_bool[35]=(self.scalar_static_bool[31]&&self.scalar_static_bool[34]);
        self.scalar_static_f64[82]=(if self.scalar_static_bool[35]{self.scalar_static_f64[41]}else{self.scalar_static_f64[80]});
        self.scalar_static_f64[83]=(self.scalar_static_f64[27]+self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(if self.scalar_static_bool[35]{self.scalar_static_f64[83]}else{self.scalar_static_f64[78]});
        self.scalar_static_f64[85]=(if self.scalar_static_bool[35]{1e99}else{self.scalar_static_f64[81]});
        self.scalar_static_bool[36]=(!(self.scalar_static_f64[57]!=0.0));
        self.scalar_static_bool[37]=(self.scalar_static_bool[23]&&self.scalar_static_bool[36]);
        self.scalar_static_bool[38]=((self.scalar_static_f64[65]!=0.0)&&self.scalar_static_bool[37]);
        self.scalar_static_f64[86]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[72]});
        self.scalar_static_f64[87]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[74]});
        self.scalar_static_f64[88]=(if self.scalar_static_bool[38]{self.scalar_static_f64[41]}else{self.scalar_static_f64[82]});
        self.scalar_static_f64[89]=(self.scalar_static_f64[27]+self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=(if self.scalar_static_bool[38]{self.scalar_static_f64[89]}else{self.scalar_static_f64[84]});
        self.scalar_static_f64[91]=(if self.scalar_static_bool[38]{1e99}else{self.scalar_static_f64[85]});
        self.scalar_static_f64[92]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_bool[39]=(self.scalar_static_bool[30]&&self.scalar_static_bool[37]);
        self.scalar_static_bool[40]=((self.scalar_static_f64[92]!=0.0)&&self.scalar_static_bool[39]);
        self.scalar_static_f64[93]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[88]});
        self.scalar_static_f64[94]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[90]});
        self.scalar_static_f64[95]=(if self.scalar_static_bool[40]{self.scalar_static_f64[36]}else{self.scalar_static_f64[86]});
        self.scalar_static_f64[96]=(self.scalar_static_f64[38]+self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(if self.scalar_static_bool[40]{self.scalar_static_f64[96]}else{self.scalar_static_f64[87]});
        self.scalar_static_f64[98]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[91]});
        self.scalar_static_bool[41]=(!(self.scalar_static_f64[92]!=0.0));
        self.scalar_static_bool[42]=(self.scalar_static_bool[39]&&self.scalar_static_bool[41]);
        self.scalar_static_f64[99]=(if self.scalar_static_bool[42]{self.scalar_static_f64[36]}else{self.scalar_static_f64[95]});
        self.scalar_static_f64[100]=(self.scalar_static_f64[38]+self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=(if self.scalar_static_bool[42]{self.scalar_static_f64[100]}else{self.scalar_static_f64[97]});
        self.scalar_static_f64[102]=(if self.scalar_static_bool[42]{self.scalar_static_f64[41]}else{self.scalar_static_f64[93]});
        self.scalar_static_f64[103]=(self.scalar_static_f64[27]+self.scalar_static_f64[102]);
        self.scalar_static_f64[104]=(if self.scalar_static_bool[42]{self.scalar_static_f64[103]}else{self.scalar_static_f64[94]});
        self.scalar_static_bool[43]=(self.scalar_static_f64[101]>0.0);
        self.scalar_static_f64[105]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_bool[44]=(self.scalar_static_f64[104]>0.0);
        self.scalar_static_f64[106]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_bool[45]=(self.scalar_static_bool[42]&&(self.scalar_static_f64[105]!=0.0));
        self.scalar_static_bool[46]=((self.scalar_static_f64[106]!=0.0)&&self.scalar_static_bool[45]);
        self.scalar_static_f64[107]=(self.scalar_static_f64[104]/self.scalar_static_f64[101]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[46]*self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=(if self.scalar_static_bool[46]{self.scalar_static_f64[108]}else{self.scalar_static_f64[98]});
        self.scalar_static_bool[47]=(!(self.scalar_static_f64[106]!=0.0));
        self.scalar_static_bool[48]=(self.scalar_static_bool[45]&&self.scalar_static_bool[47]);
        self.scalar_static_f64[110]=(if self.scalar_static_bool[48]{0.0}else{self.scalar_static_f64[109]});
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[105]!=0.0));
        self.scalar_static_bool[50]=(self.scalar_static_bool[42]&&self.scalar_static_bool[49]);
        self.scalar_static_f64[111]=(if self.scalar_static_bool[50]{1e99}else{self.scalar_static_f64[110]});
        self.scalar_static_f64[112]=p.p24;
        self.scalar_static_f64[113]=p.p23;
        self.scalar_static_f64[114]=(self.scalar_static_f64[104]+self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=(if (self.scalar_static_f64[112]!=0.0){self.scalar_static_f64[114]}else{0.0});
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[112]!=0.0));
        self.scalar_static_f64[116]=(self.scalar_static_f64[102]+self.scalar_static_f64[113]);
        self.scalar_static_f64[117]=(if self.scalar_static_bool[51]{self.scalar_static_f64[116]}else{self.scalar_static_f64[115]});
        self.scalar_static_bool[52]=(self.scalar_static_f64[111]>0.0);
        self.scalar_static_f64[118]=p.p28;
        self.scalar_static_bool[53]=(self.scalar_static_f64[118]>0.0);
        self.scalar_static_f64[119]=p.p26;
        self.scalar_static_bool[54]=(self.scalar_static_f64[119]>0.0);
        self.scalar_static_bool[55]=(self.scalar_static_bool[53]||self.scalar_static_bool[54]);
        self.scalar_static_f64[120]=p.p36;
        self.scalar_static_f64[121]=p.p37;
        self.scalar_static_bool[56]=((self.scalar_static_f64[21]!=0.0)&&(self.scalar_static_f64[106]!=0.0));
        self.scalar_static_f64[122]=p.p38;
        self.scalar_static_f64[123]=(self.scalar_static_f64[122]/self.scalar_static_f64[104]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[120]+self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(if self.scalar_static_bool[56]{self.scalar_static_f64[124]}else{self.scalar_static_f64[120]});
        self.scalar_static_f64[126]=p.p39;
        self.scalar_static_f64[127]=(self.scalar_static_f64[126]/self.scalar_static_f64[104]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[121]+self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(if self.scalar_static_bool[56]{self.scalar_static_f64[128]}else{self.scalar_static_f64[121]});
        self.scalar_static_bool[57]=(self.scalar_static_bool[4]&&(self.scalar_static_f64[106]!=0.0));
        self.scalar_static_bool[58]=((self.scalar_static_f64[24]!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_f64[130]=(0.5*self.scalar_static_f64[122]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[130]/self.scalar_static_f64[104]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[125]+self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(if self.scalar_static_bool[58]{self.scalar_static_f64[132]}else{self.scalar_static_f64[125]});
        self.scalar_static_f64[134]=(0.5*self.scalar_static_f64[126]);
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]/self.scalar_static_f64[104]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[129]+self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(if self.scalar_static_bool[58]{self.scalar_static_f64[136]}else{self.scalar_static_f64[129]});
        self.scalar_static_f64[138]=p.p40;
        self.scalar_static_f64[139]=(self.scalar_static_f64[138]/self.scalar_static_f64[101]);
        self.scalar_static_f64[140]=(self.scalar_static_f64[133]+self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=(if (self.scalar_static_f64[105]!=0.0){self.scalar_static_f64[140]}else{self.scalar_static_f64[133]});
        self.scalar_static_f64[142]=p.p41;
        self.scalar_static_f64[143]=(self.scalar_static_f64[142]/self.scalar_static_f64[101]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[137]+self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=(if (self.scalar_static_f64[105]!=0.0){self.scalar_static_f64[144]}else{self.scalar_static_f64[137]});
        self.scalar_static_bool[59]=(self.scalar_static_bool[52]&&self.scalar_static_bool[55]);
        self.scalar_static_f64[146]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_f64[147]=p.p27;
        self.scalar_static_f64[148]=p.p25;
        self.scalar_static_f64[149]=(1.0-self.scalar_static_f64[118]);
        self.scalar_static_f64[150]=(self.scalar_static_f64[149]-self.scalar_static_f64[119]);
        self.scalar_static_bool[60]=(!(self.scalar_static_f64[146]!=0.0));
        self.scalar_static_f64[151]=(1.0/self.scalar_static_f64[117]);
        self.scalar_static_f64[152]=(-1.0/self.scalar_static_f64[117]);
        self.scalar_static_f64[153]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[151]}else{0.0});
        self.scalar_static_f64[154]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[152]}else{0.0});
        self.scalar_static_f64[155]=(self.scalar_static_f64[147]*self.scalar_static_f64[153]);
        self.scalar_static_f64[156]=(self.scalar_static_f64[147]*self.scalar_static_f64[154]);
        self.scalar_static_f64[157]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[155]}else{0.0});
        self.scalar_static_f64[158]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[156]}else{0.0});
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
        self.scalar_static_f64[159]=(temperature+self.scalar_static_f64[14]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[159]-273.15);
        self.scalar_static_bool[61]=(self.scalar_static_f64[160]<self.scalar_static_f64[16]);
        self.scalar_static_f64[161]=(if self.scalar_static_bool[61]{1.0}else{0.0});
        self.scalar_static_f64[162]=(self.scalar_static_f64[160]-self.scalar_static_f64[15]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[162]-1.0);
        self.scalar_static_f64[164]=(self.scalar_static_f64[163]).exp();
        self.scalar_static_f64[165]=(self.scalar_static_f64[15]+self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=(if (self.scalar_static_f64[161]!=0.0){self.scalar_static_f64[165]}else{self.scalar_static_f64[160]});
        self.scalar_static_bool[62]=(self.scalar_static_f64[166]>self.scalar_static_f64[18]);
        self.scalar_static_f64[167]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_bool[63]=(!(self.scalar_static_f64[161]!=0.0));
        self.scalar_static_bool[64]=((self.scalar_static_f64[167]!=0.0)&&self.scalar_static_bool[63]);
        self.scalar_static_f64[168]=(self.scalar_static_f64[17]-self.scalar_static_f64[166]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[168]-1.0);
        self.scalar_static_f64[170]=(self.scalar_static_f64[169]).exp();
        self.scalar_static_f64[171]=(self.scalar_static_f64[17]-self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=(if self.scalar_static_bool[64]{self.scalar_static_f64[171]}else{self.scalar_static_f64[166]});
        self.scalar_static_f64[173]=(273.15+self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=(self.scalar_static_f64[173]-self.scalar_static_f64[13]);
        self.scalar_static_f64[175]=(self.scalar_static_f64[174]*self.scalar_static_f64[145]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[141]+self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=(self.scalar_static_f64[174]*self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=(1.0+self.scalar_static_f64[177]);
        self.scalar_static_bool[65]=(self.scalar_static_f64[178]<0.11);
        self.scalar_static_f64[179]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[180]=(self.scalar_static_f64[178]-0.01);
        self.scalar_static_f64[181]=(10.0*self.scalar_static_f64[180]);
        self.scalar_static_f64[182]=(self.scalar_static_f64[181]-1.0);
        self.scalar_static_f64[183]=(self.scalar_static_f64[182]).exp();
        self.scalar_static_f64[184]=(0.1*self.scalar_static_f64[183]);
        self.scalar_static_f64[185]=(0.01+self.scalar_static_f64[184]);
        self.scalar_static_f64[186]=(if (self.scalar_static_f64[179]!=0.0){self.scalar_static_f64[185]}else{self.scalar_static_f64[178]});
        self.scalar_static_f64[187]=(self.scalar_static_f64[111]*self.scalar_static_f64[186]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
