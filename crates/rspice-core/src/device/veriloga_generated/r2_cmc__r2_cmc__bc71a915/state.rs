#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

pub struct Parameters {
    pub p0: f64,
    pub p1: f64,
    pub p2: f64,
    pub p3: f64,
    pub p4: f64,
    pub p5: f64,
    pub p6: f64,
    pub p7: f64,
    pub p8: f64,
    pub p9: f64,
    pub p10: f64,
    pub p11: f64,
    pub p12: f64,
    pub p13: f64,
    pub p14: f64,
    pub p15: f64,
    pub p16: f64,
    pub p17: f64,
    pub p18: f64,
    pub p19: f64,
    pub p20: f64,
    pub p21: f64,
    pub p22: f64,
    pub p23: f64,
    pub p24: f64,
    pub p25: f64,
    pub p26: f64,
    pub p27: f64,
    pub p28: f64,
    pub p29: f64,
    pub p30: f64,
    pub p31: f64,
    pub p32: f64,
    pub p33: f64,
    pub p34: f64,
    pub p35: f64,
    pub p36: f64,
    pub p37: f64,
    pub p38: f64,
    pub p39: f64,
    pub p40: f64,
    pub p41: f64,
    pub p42: f64,
}

impl Copy for Parameters {}

impl Clone for Parameters {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            let params = &mut *ptr;
            params.p0 = 1e-6;
            params.p1 = 1e-6;
            params.p2 = 100.0;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 0.0;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 2.0;
            params.p9 = 1.0;
            params.p10 = 0.0;
            params.p11 = -100.0;
            params.p12 = 500.0;
            params.p13 = 0.001;
            params.p14 = 1002.0;
            params.p15 = 27.0;
            params.p16 = 100.0;
            params.p17 = 0.0;
            params.p18 = 9900000000.0;
            params.p19 = 0.0;
            params.p20 = 9900000000.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 2.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 100.0;
            params.p34 = -100.0;
            params.p35 = 500.0;
            params.p36 = 0.0;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.0;
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
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
    pub(crate) scalar_static_f64: Box<[f64; 172]>,
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
            scalar_static_f64: boxed_zero_f64_array::<172>(),
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
        match name.to_ascii_lowercase().as_str() {
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "r" => { validate_parameter("r", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c1" => { validate_parameter("c1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c2" => { validate_parameter("c2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dra" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isnoisy" => { validate_parameter("isnoisy", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthresh" => { validate_parameter("rthresh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "level" => { validate_finite_parameter("level", value)?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_parameter("lmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_parameter("lmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_parameter("wmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_parameter("wmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dxle" => { validate_finite_parameter("dxle", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_efgeo" => { validate_parameter("sw_efgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q3" => { validate_parameter("q3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p3" => { validate_parameter("p3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q2" => { validate_parameter("q2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p2" => { validate_parameter("p2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_fngeo" => { validate_parameter("sw_fngeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "jmax" => { validate_parameter("jmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1" => { validate_finite_parameter("tc1", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2" => { validate_finite_parameter("tc2", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1l" => { validate_finite_parameter("tc1l", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2l" => { validate_finite_parameter("tc2l", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1w" => { validate_finite_parameter("tc1w", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2w" => { validate_finite_parameter("tc2w", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1kfn" => { validate_finite_parameter("tc1kfn", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r2_cmc'", name)),
        }
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
        self.scalar_static_f64[0]=if param_given[9] { 1.0 } else { 0.0 };
        self.scalar_static_f64[1]=p.p9;
        self.scalar_static_f64[2]=(if (self.scalar_static_f64[0]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_bool[0]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[3]=(if self.scalar_static_bool[0]{1.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[4]=if param_given[10] { 1.0 } else { 0.0 };
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
        self.scalar_static_f64[21]=p.p22;
        self.scalar_static_f64[22]=(if self.scalar_static_bool[2]{self.scalar_static_f64[21]}else{0.0});
        self.scalar_static_bool[3]=((self.scalar_static_f64[19]!=0.0)||(self.scalar_static_f64[20]!=0.0));
        self.scalar_static_bool[4]=(!self.scalar_static_bool[2]);
        self.scalar_static_bool[5]=(self.scalar_static_bool[3]&&self.scalar_static_bool[4]);
        self.scalar_static_f64[23]=(self.scalar_static_f64[21]*0.5);
        self.scalar_static_f64[24]=(if self.scalar_static_bool[5]{self.scalar_static_f64[23]}else{self.scalar_static_f64[22]});
        self.scalar_static_bool[6]=(!self.scalar_static_bool[3]);
        self.scalar_static_bool[7]=(self.scalar_static_bool[4]&&self.scalar_static_bool[6]);
        self.scalar_static_f64[25]=(if self.scalar_static_bool[7]{0.0}else{self.scalar_static_f64[24]});
        self.scalar_static_f64[26]=if param_given[1] { 1.0 } else { 0.0 };
        self.scalar_static_f64[27]=if param_given[2] { 1.0 } else { 0.0 };
        self.scalar_static_bool[8]=((self.scalar_static_f64[26]!=0.0)&&(self.scalar_static_f64[27]!=0.0));
        self.scalar_static_f64[28]=if param_given[0] { 1.0 } else { 0.0 };
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[28]!=0.0));
        self.scalar_static_bool[10]=(self.scalar_static_bool[8]&&self.scalar_static_bool[9]);
        self.scalar_static_f64[29]=p.p2;
        self.scalar_static_bool[11]=(0.0==self.scalar_static_f64[29]);
        self.scalar_static_f64[30]=p.p1;
        self.scalar_static_bool[12]=(0.0==self.scalar_static_f64[30]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[11]||self.scalar_static_bool[12]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[10]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[31]=p.p0;
        self.scalar_static_f64[32]=(self.scalar_static_f64[11]*self.scalar_static_f64[31]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[14]{self.scalar_static_f64[32]}else{0.0});
        self.scalar_static_f64[34]=p.p21;
        self.scalar_static_f64[35]=(self.scalar_static_f64[33]+self.scalar_static_f64[34]);
        self.scalar_static_f64[36]=(if self.scalar_static_bool[14]{self.scalar_static_f64[35]}else{0.0});
        self.scalar_static_bool[15]=(!self.scalar_static_bool[13]);
        self.scalar_static_bool[16]=(self.scalar_static_bool[10]&&self.scalar_static_bool[15]);
        self.scalar_static_f64[37]=(self.scalar_static_f64[11]*self.scalar_static_f64[30]);
        self.scalar_static_f64[38]=(if self.scalar_static_bool[16]{self.scalar_static_f64[37]}else{0.0});
        self.scalar_static_f64[39]=(self.scalar_static_f64[25]+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=(if self.scalar_static_bool[16]{self.scalar_static_f64[39]}else{0.0});
        self.scalar_static_bool[17]=(self.scalar_static_f64[40]>0.0);
        self.scalar_static_bool[18]=(self.scalar_static_bool[16]&&self.scalar_static_bool[17]);
        self.scalar_static_f64[41]=p.p16;
        self.scalar_static_f64[42]=(self.scalar_static_f64[41]/self.scalar_static_f64[29]);
        self.scalar_static_f64[43]=(self.scalar_static_f64[40]*self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=(if self.scalar_static_bool[18]{self.scalar_static_f64[43]}else{self.scalar_static_f64[36]});
        self.scalar_static_f64[45]=(self.scalar_static_f64[44]-self.scalar_static_f64[34]);
        self.scalar_static_f64[46]=(if self.scalar_static_bool[18]{self.scalar_static_f64[45]}else{self.scalar_static_f64[33]});
        self.scalar_static_f64[47]=(if self.scalar_static_bool[18]{self.scalar_static_f64[29]}else{0.0});
        self.scalar_static_bool[19]=(!self.scalar_static_bool[17]);
        self.scalar_static_bool[20]=(self.scalar_static_bool[16]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[48]=(if self.scalar_static_bool[20]{self.scalar_static_f64[32]}else{self.scalar_static_f64[46]});
        self.scalar_static_f64[49]=(self.scalar_static_f64[34]+self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=(if self.scalar_static_bool[20]{self.scalar_static_f64[49]}else{self.scalar_static_f64[44]});
        self.scalar_static_f64[51]=(if self.scalar_static_bool[20]{0.0}else{self.scalar_static_f64[47]});
        self.scalar_static_bool[21]=(!(self.scalar_static_f64[26]!=0.0));
        self.scalar_static_bool[22]=((self.scalar_static_f64[27]!=0.0)&&self.scalar_static_bool[21]);
        self.scalar_static_bool[23]=(!self.scalar_static_bool[10]);
        self.scalar_static_bool[24]=(self.scalar_static_bool[22]&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=(self.scalar_static_bool[11]&&self.scalar_static_bool[24]);
        self.scalar_static_f64[52]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[38]});
        self.scalar_static_f64[53]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[40]});
        self.scalar_static_f64[54]=(if self.scalar_static_bool[25]{self.scalar_static_f64[32]}else{self.scalar_static_f64[48]});
        self.scalar_static_f64[55]=(self.scalar_static_f64[34]+self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(if self.scalar_static_bool[25]{self.scalar_static_f64[55]}else{self.scalar_static_f64[50]});
        self.scalar_static_f64[57]=(if self.scalar_static_bool[25]{0.0}else{self.scalar_static_f64[51]});
        self.scalar_static_bool[26]=(0.0==self.scalar_static_f64[31]);
        self.scalar_static_bool[27]=(!self.scalar_static_bool[11]);
        self.scalar_static_bool[28]=(self.scalar_static_bool[24]&&self.scalar_static_bool[27]);
        self.scalar_static_bool[29]=(self.scalar_static_bool[26]&&self.scalar_static_bool[28]);
        self.scalar_static_f64[58]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[54]});
        self.scalar_static_f64[59]=(if self.scalar_static_bool[29]{0.0}else{self.scalar_static_f64[56]});
        self.scalar_static_f64[60]=(if self.scalar_static_bool[29]{self.scalar_static_f64[37]}else{self.scalar_static_f64[52]});
        self.scalar_static_f64[61]=(self.scalar_static_f64[25]+self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(if self.scalar_static_bool[29]{self.scalar_static_f64[61]}else{self.scalar_static_f64[53]});
        self.scalar_static_f64[63]=(if self.scalar_static_bool[29]{1e99}else{self.scalar_static_f64[57]});
        self.scalar_static_bool[30]=(!self.scalar_static_bool[26]);
        self.scalar_static_bool[31]=(self.scalar_static_bool[28]&&self.scalar_static_bool[30]);
        self.scalar_static_f64[64]=(if self.scalar_static_bool[31]{self.scalar_static_f64[32]}else{self.scalar_static_f64[58]});
        self.scalar_static_f64[65]=(self.scalar_static_f64[34]+self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[31]{self.scalar_static_f64[65]}else{self.scalar_static_f64[59]});
        self.scalar_static_bool[32]=(self.scalar_static_f64[66]>0.0);
        self.scalar_static_bool[33]=(self.scalar_static_bool[31]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[67]=(self.scalar_static_f64[29]/self.scalar_static_f64[41]);
        self.scalar_static_f64[68]=(self.scalar_static_f64[66]*self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(if self.scalar_static_bool[33]{self.scalar_static_f64[68]}else{self.scalar_static_f64[62]});
        self.scalar_static_f64[70]=(self.scalar_static_f64[69]-self.scalar_static_f64[25]);
        self.scalar_static_f64[71]=(if self.scalar_static_bool[33]{self.scalar_static_f64[70]}else{self.scalar_static_f64[60]});
        self.scalar_static_f64[72]=(if self.scalar_static_bool[33]{self.scalar_static_f64[29]}else{self.scalar_static_f64[63]});
        self.scalar_static_bool[34]=(!self.scalar_static_bool[32]);
        self.scalar_static_bool[35]=(self.scalar_static_bool[31]&&self.scalar_static_bool[34]);
        self.scalar_static_f64[73]=(if self.scalar_static_bool[35]{self.scalar_static_f64[37]}else{self.scalar_static_f64[71]});
        self.scalar_static_f64[74]=(self.scalar_static_f64[25]+self.scalar_static_f64[73]);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[35]{self.scalar_static_f64[74]}else{self.scalar_static_f64[69]});
        self.scalar_static_f64[76]=(if self.scalar_static_bool[35]{1e99}else{self.scalar_static_f64[72]});
        self.scalar_static_bool[36]=(!self.scalar_static_bool[22]);
        self.scalar_static_bool[37]=(self.scalar_static_bool[23]&&self.scalar_static_bool[36]);
        self.scalar_static_bool[38]=(self.scalar_static_bool[26]&&self.scalar_static_bool[37]);
        self.scalar_static_f64[77]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[64]});
        self.scalar_static_f64[78]=(if self.scalar_static_bool[38]{0.0}else{self.scalar_static_f64[66]});
        self.scalar_static_f64[79]=(if self.scalar_static_bool[38]{self.scalar_static_f64[37]}else{self.scalar_static_f64[73]});
        self.scalar_static_f64[80]=(self.scalar_static_f64[25]+self.scalar_static_f64[79]);
        self.scalar_static_f64[81]=(if self.scalar_static_bool[38]{self.scalar_static_f64[80]}else{self.scalar_static_f64[75]});
        self.scalar_static_f64[82]=(if self.scalar_static_bool[38]{1e99}else{self.scalar_static_f64[76]});
        self.scalar_static_bool[39]=(self.scalar_static_bool[30]&&self.scalar_static_bool[37]);
        self.scalar_static_bool[40]=(self.scalar_static_bool[12]&&self.scalar_static_bool[39]);
        self.scalar_static_f64[83]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[79]});
        self.scalar_static_f64[84]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[81]});
        self.scalar_static_f64[85]=(if self.scalar_static_bool[40]{self.scalar_static_f64[32]}else{self.scalar_static_f64[77]});
        self.scalar_static_f64[86]=(self.scalar_static_f64[34]+self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=(if self.scalar_static_bool[40]{self.scalar_static_f64[86]}else{self.scalar_static_f64[78]});
        self.scalar_static_f64[88]=(if self.scalar_static_bool[40]{0.0}else{self.scalar_static_f64[82]});
        self.scalar_static_bool[41]=(!self.scalar_static_bool[12]);
        self.scalar_static_bool[42]=(self.scalar_static_bool[39]&&self.scalar_static_bool[41]);
        self.scalar_static_f64[89]=(if self.scalar_static_bool[42]{self.scalar_static_f64[32]}else{self.scalar_static_f64[85]});
        self.scalar_static_f64[90]=(self.scalar_static_f64[34]+self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=(if self.scalar_static_bool[42]{self.scalar_static_f64[90]}else{self.scalar_static_f64[87]});
        self.scalar_static_f64[92]=(if self.scalar_static_bool[42]{self.scalar_static_f64[37]}else{self.scalar_static_f64[83]});
        self.scalar_static_f64[93]=(self.scalar_static_f64[25]+self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=(if self.scalar_static_bool[42]{self.scalar_static_f64[93]}else{self.scalar_static_f64[84]});
        self.scalar_static_bool[43]=(self.scalar_static_f64[91]>0.0);
        self.scalar_static_bool[44]=(self.scalar_static_f64[94]>0.0);
        self.scalar_static_bool[45]=(self.scalar_static_bool[42]&&self.scalar_static_bool[43]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[44]&&self.scalar_static_bool[45]);
        self.scalar_static_f64[95]=(self.scalar_static_f64[94]/self.scalar_static_f64[91]);
        self.scalar_static_f64[96]=(self.scalar_static_f64[41]*self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(if self.scalar_static_bool[46]{self.scalar_static_f64[96]}else{self.scalar_static_f64[88]});
        self.scalar_static_bool[47]=(!self.scalar_static_bool[44]);
        self.scalar_static_bool[48]=(self.scalar_static_bool[45]&&self.scalar_static_bool[47]);
        self.scalar_static_f64[98]=(if self.scalar_static_bool[48]{0.0}else{self.scalar_static_f64[97]});
        self.scalar_static_bool[49]=(!self.scalar_static_bool[43]);
        self.scalar_static_bool[50]=(self.scalar_static_bool[42]&&self.scalar_static_bool[49]);
        self.scalar_static_f64[99]=(if self.scalar_static_bool[50]{1e99}else{self.scalar_static_f64[98]});
        self.scalar_static_f64[100]=p.p24;
        self.scalar_static_f64[101]=p.p23;
        self.scalar_static_f64[102]=(self.scalar_static_f64[94]+self.scalar_static_f64[101]);
        self.scalar_static_f64[103]=(if (self.scalar_static_f64[100]!=0.0){self.scalar_static_f64[102]}else{0.0});
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[100]!=0.0));
        self.scalar_static_f64[104]=(self.scalar_static_f64[92]+self.scalar_static_f64[101]);
        self.scalar_static_f64[105]=(if self.scalar_static_bool[51]{self.scalar_static_f64[104]}else{self.scalar_static_f64[103]});
        self.scalar_static_bool[52]=(self.scalar_static_f64[99]>0.0);
        self.scalar_static_f64[106]=p.p28;
        self.scalar_static_bool[53]=(self.scalar_static_f64[106]>0.0);
        self.scalar_static_f64[107]=p.p26;
        self.scalar_static_bool[54]=(self.scalar_static_f64[107]>0.0);
        self.scalar_static_bool[55]=(self.scalar_static_bool[53]||self.scalar_static_bool[54]);
        self.scalar_static_f64[108]=p.p36;
        self.scalar_static_f64[109]=p.p37;
        self.scalar_static_bool[56]=(self.scalar_static_bool[2]&&self.scalar_static_bool[44]);
        self.scalar_static_f64[110]=p.p38;
        self.scalar_static_f64[111]=(self.scalar_static_f64[110]/self.scalar_static_f64[94]);
        self.scalar_static_f64[112]=(self.scalar_static_f64[108]+self.scalar_static_f64[111]);
        self.scalar_static_f64[113]=(if self.scalar_static_bool[56]{self.scalar_static_f64[112]}else{self.scalar_static_f64[108]});
        self.scalar_static_f64[114]=p.p39;
        self.scalar_static_f64[115]=(self.scalar_static_f64[114]/self.scalar_static_f64[94]);
        self.scalar_static_f64[116]=(self.scalar_static_f64[109]+self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=(if self.scalar_static_bool[56]{self.scalar_static_f64[116]}else{self.scalar_static_f64[109]});
        self.scalar_static_bool[57]=(self.scalar_static_bool[4]&&self.scalar_static_bool[44]);
        self.scalar_static_bool[58]=(self.scalar_static_bool[3]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[118]=(0.5*self.scalar_static_f64[110]);
        self.scalar_static_f64[119]=(self.scalar_static_f64[118]/self.scalar_static_f64[94]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[113]+self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(if self.scalar_static_bool[58]{self.scalar_static_f64[120]}else{self.scalar_static_f64[113]});
        self.scalar_static_f64[122]=(0.5*self.scalar_static_f64[114]);
        self.scalar_static_f64[123]=(self.scalar_static_f64[122]/self.scalar_static_f64[94]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[117]+self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(if self.scalar_static_bool[58]{self.scalar_static_f64[124]}else{self.scalar_static_f64[117]});
        self.scalar_static_f64[126]=p.p40;
        self.scalar_static_f64[127]=(self.scalar_static_f64[126]/self.scalar_static_f64[91]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[121]+self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(if self.scalar_static_bool[43]{self.scalar_static_f64[128]}else{self.scalar_static_f64[121]});
        self.scalar_static_f64[130]=p.p41;
        self.scalar_static_f64[131]=(self.scalar_static_f64[130]/self.scalar_static_f64[91]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[125]+self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(if self.scalar_static_bool[43]{self.scalar_static_f64[132]}else{self.scalar_static_f64[125]});
        self.scalar_static_bool[59]=(self.scalar_static_bool[52]&&self.scalar_static_bool[55]);
        self.scalar_static_f64[134]=p.p27;
        self.scalar_static_f64[135]=p.p25;
        self.scalar_static_f64[136]=(1.0-self.scalar_static_f64[106]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[136]-self.scalar_static_f64[107]);
        self.scalar_static_bool[60]=(!self.scalar_static_bool[59]);
        self.scalar_static_f64[138]=(1.0/self.scalar_static_f64[105]);
        self.scalar_static_f64[139]=(-1.0/self.scalar_static_f64[105]);
        self.scalar_static_f64[140]=(if self.scalar_static_bool[59]{self.scalar_static_f64[138]}else{0.0});
        self.scalar_static_f64[141]=(if self.scalar_static_bool[59]{self.scalar_static_f64[139]}else{0.0});
        self.scalar_static_f64[142]=(self.scalar_static_f64[134]*self.scalar_static_f64[140]);
        self.scalar_static_f64[143]=(self.scalar_static_f64[134]*self.scalar_static_f64[141]);
        self.scalar_static_f64[144]=(if self.scalar_static_bool[59]{self.scalar_static_f64[142]}else{0.0});
        self.scalar_static_f64[145]=(if self.scalar_static_bool[59]{self.scalar_static_f64[143]}else{0.0});
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
        self.scalar_static_f64[146]=(temperature+self.scalar_static_f64[14]);
        self.scalar_static_f64[147]=(self.scalar_static_f64[146]-273.15);
        self.scalar_static_bool[61]=(self.scalar_static_f64[147]<self.scalar_static_f64[16]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[147]-self.scalar_static_f64[15]);
        self.scalar_static_f64[149]=(self.scalar_static_f64[148]-1.0);
        self.scalar_static_f64[150]=(self.scalar_static_f64[149]).exp();
        self.scalar_static_f64[151]=(self.scalar_static_f64[15]+self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=(if self.scalar_static_bool[61]{self.scalar_static_f64[151]}else{self.scalar_static_f64[147]});
        self.scalar_static_bool[62]=(self.scalar_static_f64[152]>self.scalar_static_f64[18]);
        self.scalar_static_bool[63]=(!self.scalar_static_bool[61]);
        self.scalar_static_bool[64]=(self.scalar_static_bool[62]&&self.scalar_static_bool[63]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[17]-self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=(self.scalar_static_f64[153]-1.0);
        self.scalar_static_f64[155]=(self.scalar_static_f64[154]).exp();
        self.scalar_static_f64[156]=(self.scalar_static_f64[17]-self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=(if self.scalar_static_bool[64]{self.scalar_static_f64[156]}else{self.scalar_static_f64[152]});
        self.scalar_static_f64[158]=(273.15+self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=(self.scalar_static_f64[158]-self.scalar_static_f64[13]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[159]*self.scalar_static_f64[133]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[129]+self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=(self.scalar_static_f64[159]*self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(1.0+self.scalar_static_f64[162]);
        self.scalar_static_bool[65]=(self.scalar_static_f64[163]<0.11);
        self.scalar_static_f64[164]=(self.scalar_static_f64[163]-0.01);
        self.scalar_static_f64[165]=(10.0*self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=(self.scalar_static_f64[165]-1.0);
        self.scalar_static_f64[167]=(self.scalar_static_f64[166]).exp();
        self.scalar_static_f64[168]=(0.1*self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=(0.01+self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(if self.scalar_static_bool[65]{self.scalar_static_f64[169]}else{self.scalar_static_f64[163]});
        self.scalar_static_f64[171]=(self.scalar_static_f64[99]*self.scalar_static_f64[170]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
