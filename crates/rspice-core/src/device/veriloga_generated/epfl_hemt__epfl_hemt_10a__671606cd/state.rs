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
    pub p43: f64,
    pub p44: f64,
    pub p45: f64,
    pub p46: f64,
    pub p47: f64,
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
            params.p1 = 2e-6;
            params.p2 = 7e-6;
            params.p3 = 0.003;
            params.p4 = 1.0;
            params.p5 = 6e-8;
            params.p6 = 300.0;
            params.p7 = -5.43;
            params.p8 = 0.02;
            params.p9 = -0.01;
            params.p10 = 0.045;
            params.p11 = 1e19;
            params.p12 = 0.3;
            params.p13 = 1.6e-12;
            params.p14 = 200000000.0;
            params.p15 = 8000000.0;
            params.p16 = 200.0;
            params.p17 = 1.0;
            params.p18 = 0.0;
            params.p19 = 0.0;
            params.p20 = 1e-9;
            params.p21 = 0.0;
            params.p22 = 0.165;
            params.p23 = 1.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 1.0;
            params.p27 = -1.7;
            params.p28 = -2.2;
            params.p29 = 0.5;
            params.p30 = 1.0;
            params.p31 = 70.0;
            params.p32 = 1e-8;
            params.p33 = 0.0;
            params.p34 = 1.0;
            params.p35 = 20.0;
            params.p36 = 1e-9;
            params.p37 = 5e17;
            params.p38 = 0.155;
            params.p39 = 30000.0;
            params.p40 = 0.0022;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 0.0022;
            params.p44 = 0.0;
            params.p45 = 2.0;
            params.p46 = 0.0;
            params.p47 = 1.2;
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
    pub nodes: [usize; 12],
    pub branches: [usize; 3],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 48]>,
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
    pub(crate) scalar_static_f64: Box<[f64; 100]>,
    pub(crate) scalar_static_bool: Box<[bool; 3]>,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["di", "si", "gi", "gm", "bi", "sbulk", "dbulk"];

    pub const BRANCH_COUNT: usize = 3;
    pub const PARAMETER_COUNT: usize = 48;
    pub const VARIABLE_COUNT: usize = 149;
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
            scalar_static_f64: boxed_zero_f64_array::<100>(),
            scalar_static_bool: boxed_zero_bool_array::<3>(),
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
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "lard" => { validate_parameter("Lard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "lars" => { validate_parameter("Lars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "x1" => { validate_parameter("x1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "va" => { validate_finite_parameter("VA", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "phin" => { validate_finite_parameter("PHIN", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "ndep" => { validate_parameter("NDEP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "xx" => { validate_parameter("xx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "gg" => { validate_parameter("gg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "e0" => { validate_parameter("E0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ucrit" => { validate_finite_parameter("UCRIT", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "aclm" => { validate_parameter("ACLM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "delta" => { validate_parameter("DELTA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "cit" => { validate_parameter("CIT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "nfactor" => { validate_parameter("NFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_parameter("CDSCD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "cdscb" => { validate_parameter("CDSCB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_parameter("U0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_parameter("UA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_parameter("UC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ud" => { validate_parameter("UD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "eu" => { validate_parameter("EU", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "bex" => { validate_finite_parameter("BEX", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "ucex" => { validate_finite_parameter("UCEX", value)?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "etavsat" => { validate_finite_parameter("ETAVSAT", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "usc" => { validate_parameter("USC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "avdsx" => { validate_parameter("AVDSX", value, Some((5.0, "5.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "lc" => { validate_parameter("LC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "lambda" => { validate_parameter("LAMBDA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ns0" => { validate_parameter("ns0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "mu0acc" => { validate_parameter("mu0acc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "vsat0acc" => { validate_parameter("vsat0acc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "rcs" => { validate_parameter("Rcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "ktrs" => { validate_parameter("ktrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "ktrd" => { validate_parameter("ktrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "rcd" => { validate_parameter("Rcd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "kth1" => { validate_parameter("kth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "kth2" => { validate_parameter("kth2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "kth3" => { validate_finite_parameter("kth3", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "gsar" => { validate_parameter("gsar", value, None, true, None, true, &[(0.0, "0.0")])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'EPFL_HEMT_10a'", name)),
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
        self.scalar_static_f64[0]=p.p34;
        self.scalar_static_f64[1]=p.p6;
        self.scalar_static_f64[2]=p.p12;
        self.scalar_static_f64[3]=(1.0-self.scalar_static_f64[2]);
        self.scalar_static_f64[4]=(self.scalar_static_f64[2]*8.5);
        self.scalar_static_f64[5]=(8.9*self.scalar_static_f64[3]);
        self.scalar_static_f64[6]=(self.scalar_static_f64[4]+self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=(8.85418e-12*self.scalar_static_f64[6]);
        self.scalar_static_bool[0]=(1.0!=self.scalar_static_f64[0]);
        self.scalar_static_f64[8]=(if self.scalar_static_bool[0]{0.3333333333333333}else{0.5});
        self.scalar_static_f64[9]=p.p22;
        self.scalar_static_f64[10]=p.p27;
        self.scalar_static_f64[11]=p.p3;
        self.scalar_static_f64[12]=p.p0;
        self.scalar_static_f64[13]=p.p11;
        self.scalar_static_f64[14]=p.p5;
        self.scalar_static_f64[15]=(self.scalar_static_f64[7]/self.scalar_static_f64[14]);
        self.scalar_static_f64[16]=p.p31;
        self.scalar_static_f64[17]=(2.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(self.scalar_static_f64[17]*0.6931471805599453);
        self.scalar_static_f64[19]=p.p18;
        self.scalar_static_f64[20]=p.p19;
        self.scalar_static_f64[21]=(self.scalar_static_f64[19]+self.scalar_static_f64[20]);
        self.scalar_static_f64[22]=p.p20;
        self.scalar_static_f64[23]=p.p21;
        self.scalar_static_f64[24]=p.p7;
        self.scalar_static_f64[25]=p.p8;
        self.scalar_static_f64[26]=p.p9;
        self.scalar_static_f64[27]=(self.scalar_static_f64[13]*2.52482255208e-29);
        self.scalar_static_f64[28]=p.p13;
        self.scalar_static_f64[29]=(self.scalar_static_f64[13]/1.8e25);
        self.scalar_static_f64[30]=(self.scalar_static_f64[29]).ln();
        self.scalar_static_f64[31]=p.p14;
        self.scalar_static_f64[32]=(7.8802202e-11*self.scalar_static_f64[31]);
        self.scalar_static_f64[33]=(self.scalar_static_f64[15]/self.scalar_static_f64[32]);
        self.scalar_static_f64[34]=p.p30;
        self.scalar_static_f64[35]=p.p23;
        self.scalar_static_f64[36]=p.p24;
        self.scalar_static_f64[37]=p.p26;
        self.scalar_static_f64[38]=p.p25;
        self.scalar_static_f64[39]=p.p15;
        self.scalar_static_f64[40]=p.p28;
        self.scalar_static_f64[41]=(-self.scalar_static_f64[40]);
        self.scalar_static_f64[42]=p.p17;
        self.scalar_static_f64[43]=(2.0-self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=p.p16;
        self.scalar_static_f64[45]=(self.scalar_static_f64[44]/self.scalar_static_f64[42]);
        self.scalar_static_f64[46]=p.p32;
        self.scalar_static_f64[47]=(2.0*self.scalar_static_f64[46]);
        self.scalar_static_f64[48]=(self.scalar_static_f64[12]-self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=(self.scalar_static_f64[46]/self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=p.p33;
        self.scalar_static_f64[51]=(self.scalar_static_f64[46]*self.scalar_static_f64[50]);
        self.scalar_static_f64[52]=(2.0*self.scalar_static_f64[49]);
        self.scalar_static_f64[53]=(1.0+self.scalar_static_f64[49]);
        self.scalar_static_f64[54]=p.p4;
        self.scalar_static_f64[55]=(2.0*self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=p.p37;
        self.scalar_static_f64[57]=p.p39;
        self.scalar_static_f64[58]=p.p44;
        self.scalar_static_f64[59]=p.p45;
        self.scalar_static_f64[60]=p.p38;
        self.scalar_static_f64[61]=p.p46;
        self.scalar_static_f64[62]=(1.602e-19*self.scalar_static_f64[56]);
        self.scalar_static_f64[63]=(self.scalar_static_f64[11]*self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=p.p1;
        self.scalar_static_f64[65]=p.p2;
        self.scalar_static_f64[66]=p.p47;
        self.scalar_static_f64[67]=(1.0/self.scalar_static_f64[66]);
        self.scalar_static_f64[68]=p.p40;
        self.scalar_static_f64[69]=(self.scalar_static_f64[68]/self.scalar_static_f64[11]);
        self.scalar_static_f64[70]=p.p43;
        self.scalar_static_f64[71]=(self.scalar_static_f64[70]/self.scalar_static_f64[11]);
        self.scalar_static_f64[72]=p.p42;
        self.scalar_static_f64[73]=p.p41;
        self.scalar_static_f64[74]=p.p35;
        self.scalar_static_bool[1]=(0.0!=self.scalar_static_f64[74]);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_f64[76]=p.p36;
        self.scalar_static_bool[2]=(!(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_f64[77]=(1.0/self.scalar_static_f64[1]);
        self.scalar_static_f64[78]=(self.scalar_static_f64[10]-1.0);
        self.scalar_static_f64[79]=(0.0259*self.scalar_static_f64[77]);
        self.scalar_static_f64[80]=(-self.scalar_static_f64[79]);
        self.scalar_static_f64[81]=(3.204e-19*self.scalar_static_f64[79]);
        self.scalar_static_f64[82]=(-self.scalar_static_f64[16]);
        self.scalar_static_f64[83]=(self.scalar_static_f64[28]*self.scalar_static_f64[79]);
        self.scalar_static_f64[84]=(-self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(8.353992494899963e17*self.scalar_static_f64[79]);
        self.scalar_static_f64[86]=(1.602e-19*self.scalar_static_f64[79]);
        self.scalar_static_f64[87]=(1.602e-19*self.scalar_static_f64[85]);
        self.scalar_static_f64[88]=(self.scalar_static_f64[34]-1.0);
        self.scalar_static_f64[89]=(self.scalar_static_f64[37]-1.0);
        self.scalar_static_f64[90]=(self.scalar_static_f64[41]-1.0);
        self.scalar_static_f64[91]=(self.scalar_static_f64[58]*self.scalar_static_f64[77]);
        self.scalar_static_f64[92]=(-self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(self.scalar_static_f64[59]-1.0);
        self.scalar_static_f64[94]=(self.scalar_static_f64[61]-1.0);
        self.scalar_static_f64[95]=(self.scalar_static_f64[66]-1.0);
        self.scalar_static_f64[96]=(self.scalar_static_f64[67]-1.0);
        self.scalar_static_f64[97]=(1.0/self.scalar_static_f64[74]);
        self.scalar_static_f64[98]=(if (self.scalar_static_f64[75]!=0.0){self.scalar_static_f64[97]}else{0.0});
        self.scalar_static_f64[99]=(if self.scalar_static_bool[2]{1000000000.0}else{0.0});
    }
}
