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
    pub p48: f64,
    pub p49: f64,
    pub p50: f64,
    pub p51: f64,
    pub p52: f64,
    pub p53: f64,
    pub p54: f64,
    pub p55: f64,
    pub p56: f64,
    pub p57: f64,
    pub p58: f64,
    pub p59: f64,
    pub p60: f64,
    pub p61: f64,
    pub p62: f64,
    pub p63: f64,
    pub p64: f64,
    pub p65: f64,
    pub p66: f64,
    pub p67: f64,
    pub p68: f64,
    pub p69: f64,
    pub p70: f64,
    pub p71: f64,
    pub p72: f64,
    pub p73: f64,
    pub p74: f64,
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
            params.p0 = 1.0;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 1e21;
            params.p4 = 1e21;
            params.p5 = 1e-5;
            params.p6 = 1e-5;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.002;
            params.p14 = 3e-7;
            params.p15 = 0.5;
            params.p16 = 0.001;
            params.p17 = 0.7;
            params.p18 = 0.5;
            params.p19 = 0.00015;
            params.p20 = -1.5;
            params.p21 = 0.0;
            params.p22 = 100000000.0;
            params.p23 = 2000000.0;
            params.p24 = 0.8;
            params.p25 = 0.8;
            params.p26 = -1e-8;
            params.p27 = -1e-8;
            params.p28 = 0.2;
            params.p29 = 0.3;
            params.p30 = 0.00023;
            params.p31 = 4e-7;
            params.p32 = 500000000.0;
            params.p33 = 400000000.0;
            params.p34 = 0.0009;
            params.p35 = 1.0;
            params.p36 = 0.0;
            params.p37 = 5e-7;
            params.p38 = 1e-6;
            params.p39 = 1e-6;
            params.p40 = 1e-6;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 1.0;
            params.p44 = 1e-9;
            params.p45 = 1e-12;
            params.p46 = 1e-12;
            params.p47 = 0.9;
            params.p48 = 0.7;
            params.p49 = 0.7;
            params.p50 = 0.8;
            params.p51 = 0.6;
            params.p52 = 0.6;
            params.p53 = 1e-9;
            params.p54 = 1e-12;
            params.p55 = 1e-12;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 10.0;
            params.p59 = 1.0;
            params.p60 = 1.0;
            params.p61 = 1.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 3.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
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
    pub(crate) scalar_static_f64: Box<[f64; 341]>,
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
            scalar_static_f64: boxed_zero_f64_array::<341>(),
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
        match name.to_ascii_lowercase().as_str() {
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "temp" => { validate_parameter("TEMP", value, Some((273.15, "273.15")), false, None, false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "m" => { validate_parameter("M", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ns" => { validate_parameter("NS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cox" => { validate_parameter("COX", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xj" => { validate_parameter("XJ", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vto" => { validate_finite_parameter("VTO", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcv" => { validate_finite_parameter("TCV", value)?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamma" => { validate_parameter("GAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "phi" => { validate_parameter("PHI", value, Some((0.2, "0.2")), false, None, false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kp" => { validate_parameter("KP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bex" => { validate_finite_parameter("BEX", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "theta" => { validate_parameter("THETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "e0" => { validate_finite_parameter("E0", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ucrit" => { validate_parameter("UCRIT", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ucex" => { validate_finite_parameter("UCEX", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lambda" => { validate_parameter("LAMBDA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dl" => { validate_finite_parameter("DL", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dw" => { validate_finite_parameter("DW", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "weta" => { validate_parameter("WETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "leta" => { validate_parameter("LETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q0" => { validate_parameter("Q0", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lk" => { validate_parameter("LK", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iba" => { validate_parameter("IBA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibb" => { validate_parameter("IBB", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibbt" => { validate_finite_parameter("IBBT", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibn" => { validate_parameter("IBN", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hdif" => { validate_parameter("HDIF", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avto" => { validate_parameter("AVTO", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "akp" => { validate_parameter("AKP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agamma" => { validate_parameter("AGAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("AF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("KF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_n" => { validate_parameter("xd_n", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_js" => { validate_parameter("xd_js", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_jsw" => { validate_parameter("xd_jsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_jswg" => { validate_parameter("xd_jswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mj" => { validate_parameter("xd_mj", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mjsw" => { validate_parameter("xd_mjsw", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mjswg" => { validate_parameter("xd_mjswg", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pb" => { validate_parameter("xd_pb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pbsw" => { validate_parameter("xd_pbsw", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pbswg" => { validate_parameter("xd_pbswg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cj" => { validate_parameter("xd_cj", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cjsw" => { validate_parameter("xd_cjsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cjswg" => { validate_parameter("xd_cjswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_gmin" => { validate_parameter("xd_gmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_xjbv" => { validate_parameter("xd_xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_bv" => { validate_parameter("xd_bv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njts" => { validate_parameter("xd_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njtssw" => { validate_parameter("xd_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njtsswg" => { validate_parameter("xd_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vts" => { validate_parameter("xd_vts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vtssw" => { validate_parameter("xd_vtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vtsswg" => { validate_parameter("xd_vtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_xti" => { validate_finite_parameter("tp_xti", value)?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cj" => { validate_finite_parameter("tp_cj", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cjsw" => { validate_finite_parameter("tp_cjsw", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cjswg" => { validate_finite_parameter("tp_cjswg", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pb" => { validate_finite_parameter("tp_pb", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pbsw" => { validate_finite_parameter("tp_pbsw", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pbswg" => { validate_finite_parameter("tp_pbswg", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njts" => { validate_parameter("tp_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njtssw" => { validate_parameter("tp_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njtsswg" => { validate_parameter("tp_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name)),
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
        self.scalar_static_f64[23]=p.p2;
        self.scalar_static_bool[2]=(!self.scalar_static_bool[1]);
        self.scalar_static_f64[24]=(self.scalar_static_f64[22]+273.15);
        self.scalar_static_f64[25]=p.p4;
        self.scalar_static_bool[3]=(1e21==self.scalar_static_f64[25]);
        self.scalar_static_f64[26]=(if self.scalar_static_bool[3]{298.15}else{0.0});
        self.scalar_static_bool[4]=(!self.scalar_static_bool[3]);
        self.scalar_static_f64[27]=(273.15+self.scalar_static_f64[25]);
        self.scalar_static_f64[28]=(if self.scalar_static_bool[4]{self.scalar_static_f64[27]}else{self.scalar_static_f64[26]});
        self.scalar_static_f64[29]=(self.scalar_static_f64[28]*0.000702);
        self.scalar_static_f64[30]=(self.scalar_static_f64[28]*self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=(self.scalar_static_f64[28]+1108.0);
        self.scalar_static_f64[32]=(self.scalar_static_f64[30]/self.scalar_static_f64[31]);
        self.scalar_static_f64[33]=(1.16-self.scalar_static_f64[32]);
        self.scalar_static_f64[34]=p.p15;
        self.scalar_static_f64[35]=p.p16;
        self.scalar_static_f64[36]=p.p19;
        self.scalar_static_f64[37]=p.p20;
        self.scalar_static_f64[38]=p.p23;
        self.scalar_static_f64[39]=p.p24;
        self.scalar_static_f64[40]=p.p33;
        self.scalar_static_f64[41]=p.p34;
        self.scalar_static_f64[42]=p.p18;
        self.scalar_static_f64[43]=p.p32;
        self.scalar_static_f64[44]=p.p5;
        self.scalar_static_f64[45]=p.p26;
        self.scalar_static_f64[46]=(self.scalar_static_f64[44]+self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=p.p6;
        self.scalar_static_f64[48]=p.p27;
        self.scalar_static_f64[49]=(self.scalar_static_f64[47]+self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=(self.scalar_static_f64[46]*self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=(self.scalar_static_f64[50]).sqrt();
        self.scalar_static_f64[52]=(1.0/self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=p.p38;
        self.scalar_static_bool[5]=(self.scalar_static_f64[53]!=1e-6);
        self.scalar_static_f64[54]=(self.scalar_static_f64[53]-1e-6);
        self.scalar_static_f64[55]=(self.scalar_static_f64[52]*self.scalar_static_f64[54]);
        self.scalar_static_bool[6]=(!self.scalar_static_bool[0]);
        self.scalar_static_f64[56]=(1e-6-self.scalar_static_f64[53]);
        self.scalar_static_f64[57]=(self.scalar_static_f64[52]*self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=p.p39;
        self.scalar_static_bool[7]=(1e-6!=self.scalar_static_f64[58]);
        self.scalar_static_f64[59]=(self.scalar_static_f64[58]-1e-6);
        self.scalar_static_f64[60]=(self.scalar_static_f64[52]*self.scalar_static_f64[59]);
        self.scalar_static_f64[61]=(1.0+self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=p.p40;
        self.scalar_static_bool[8]=(1e-6!=self.scalar_static_f64[62]);
        self.scalar_static_f64[63]=p.p17;
        self.scalar_static_f64[64]=(self.scalar_static_f64[62]-1e-6);
        self.scalar_static_f64[65]=(self.scalar_static_f64[52]*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[63]+self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=(if self.scalar_static_bool[8]{self.scalar_static_f64[66]}else{self.scalar_static_f64[63]});
        self.scalar_static_bool[9]=(0.0==self.scalar_static_f64[19]);
        self.scalar_static_bool[10]=(!self.scalar_static_bool[9]);
        self.scalar_static_f64[68]=p.p31;
        self.scalar_static_f64[69]=p.p8;
        self.scalar_static_f64[70]=(self.scalar_static_f64[68]*self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=(self.scalar_static_f64[46]/self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=(self.scalar_static_f64[71]-0.1);
        self.scalar_static_f64[73]=(0.28*self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(if self.scalar_static_bool[10]{self.scalar_static_f64[73]}else{0.0});
        self.scalar_static_f64[75]=(self.scalar_static_f64[74]*self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=(self.scalar_static_f64[75]+0.001936);
        self.scalar_static_f64[77]=(self.scalar_static_f64[76]).sqrt();
        self.scalar_static_f64[78]=(self.scalar_static_f64[74]+self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=(0.5*self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(1.0+self.scalar_static_f64[79]);
        self.scalar_static_f64[81]=(1.0/self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=(if self.scalar_static_bool[10]{self.scalar_static_f64[81]}else{0.0});
        self.scalar_static_f64[83]=(self.scalar_static_f64[19]*self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(self.scalar_static_f64[82]*self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(if self.scalar_static_bool[10]{self.scalar_static_f64[84]}else{0.0});
        self.scalar_static_f64[86]=p.p7;
        self.scalar_static_f64[87]=(self.scalar_static_f64[9]*self.scalar_static_f64[86]);
        self.scalar_static_f64[88]=(self.scalar_static_f64[87]/self.scalar_static_f64[49]);
        self.scalar_static_f64[89]=(self.scalar_static_f64[11]*self.scalar_static_f64[69]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[89]/self.scalar_static_f64[46]);
        self.scalar_static_f64[91]=(self.scalar_static_f64[67]*0.25);
        self.scalar_static_f64[92]=(self.scalar_static_f64[67]*self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(0.5*self.scalar_static_f64[67]);
        self.scalar_static_f64[94]=(0.1*self.scalar_static_f64[46]);
        self.scalar_static_f64[95]=(self.scalar_static_f64[94]*self.scalar_static_f64[94]);
        self.scalar_static_f64[96]=(self.scalar_static_f64[67]* -0.5);
        self.scalar_static_bool[11]=(0.0==self.scalar_static_f64[14]);
        self.scalar_static_f64[97]=p.p21;
        self.scalar_static_bool[12]=(!self.scalar_static_bool[11]);
        self.scalar_static_f64[98]=(-self.scalar_static_f64[90]);
        self.scalar_static_f64[99]=(-self.scalar_static_f64[67]);
        self.scalar_static_f64[100]=p.p36;
        self.scalar_static_f64[101]=p.p37;
        self.scalar_static_f64[102]=(self.scalar_static_f64[100]*self.scalar_static_f64[101]);
        self.scalar_static_f64[103]=(self.scalar_static_f64[49]-self.scalar_static_f64[48]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[102]/self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=(self.scalar_static_f64[0]*self.scalar_static_f64[50]);
        self.scalar_static_f64[106]=p.p9;
        self.scalar_static_bool[13]=(0.0==self.scalar_static_f64[106]);
        self.scalar_static_bool[14]=(self.scalar_static_f64[101]>0.0);
        self.scalar_static_bool[15]=(self.scalar_static_bool[13]&&self.scalar_static_bool[14]);
        self.scalar_static_f64[107]=(2.0*self.scalar_static_f64[101]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[49]*self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=(if self.scalar_static_bool[15]{self.scalar_static_f64[108]}else{0.0});
        self.scalar_static_bool[16]=(!self.scalar_static_bool[15]);
        self.scalar_static_f64[110]=(if self.scalar_static_bool[16]{self.scalar_static_f64[106]}else{self.scalar_static_f64[109]});
        self.scalar_static_f64[111]=p.p11;
        self.scalar_static_bool[17]=(0.0==self.scalar_static_f64[111]);
        self.scalar_static_bool[18]=(self.scalar_static_bool[14]&&self.scalar_static_bool[17]);
        self.scalar_static_f64[112]=(4.0*self.scalar_static_f64[101]);
        self.scalar_static_f64[113]=(self.scalar_static_f64[49]+self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=(if self.scalar_static_bool[18]{self.scalar_static_f64[113]}else{0.0});
        self.scalar_static_bool[19]=(!self.scalar_static_bool[18]);
        self.scalar_static_f64[115]=(if self.scalar_static_bool[19]{self.scalar_static_f64[111]}else{self.scalar_static_f64[114]});
        self.scalar_static_f64[116]=p.p10;
        self.scalar_static_bool[20]=(0.0==self.scalar_static_f64[116]);
        self.scalar_static_bool[21]=(self.scalar_static_bool[14]&&self.scalar_static_bool[20]);
        self.scalar_static_f64[117]=(if self.scalar_static_bool[21]{self.scalar_static_f64[108]}else{0.0});
        self.scalar_static_bool[22]=(!self.scalar_static_bool[21]);
        self.scalar_static_f64[118]=(if self.scalar_static_bool[22]{self.scalar_static_f64[116]}else{self.scalar_static_f64[117]});
        self.scalar_static_f64[119]=p.p12;
        self.scalar_static_bool[23]=(0.0==self.scalar_static_f64[119]);
        self.scalar_static_bool[24]=(self.scalar_static_bool[14]&&self.scalar_static_bool[23]);
        self.scalar_static_f64[120]=(if self.scalar_static_bool[24]{self.scalar_static_f64[113]}else{0.0});
        self.scalar_static_bool[25]=(!self.scalar_static_bool[24]);
        self.scalar_static_f64[121]=(if self.scalar_static_bool[25]{self.scalar_static_f64[119]}else{self.scalar_static_f64[120]});
        self.scalar_static_f64[122]=(self.scalar_static_f64[28]*8.617333262145179e-5);
        self.scalar_static_f64[123]=(self.scalar_static_f64[33]/self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=p.p65;
        self.scalar_static_f64[125]=p.p43;
        self.scalar_static_f64[126]=p.p44;
        self.scalar_static_f64[127]=p.p45;
        self.scalar_static_f64[128]=p.p46;
        self.scalar_static_f64[129]=p.p50;
        self.scalar_static_f64[130]=p.p69;
        self.scalar_static_f64[131]=p.p51;
        self.scalar_static_f64[132]=p.p70;
        self.scalar_static_f64[133]=p.p52;
        self.scalar_static_f64[134]=p.p71;
        self.scalar_static_f64[135]=p.p53;
        self.scalar_static_f64[136]=p.p66;
        self.scalar_static_f64[137]=p.p54;
        self.scalar_static_f64[138]=p.p67;
        self.scalar_static_f64[139]=p.p55;
        self.scalar_static_f64[140]=p.p68;
        self.scalar_static_f64[141]=p.p59;
        self.scalar_static_f64[142]=p.p72;
        self.scalar_static_f64[143]=p.p60;
        self.scalar_static_f64[144]=p.p73;
        self.scalar_static_f64[145]=p.p61;
        self.scalar_static_f64[146]=p.p74;
        self.scalar_static_f64[147]=p.p58;
        self.scalar_static_f64[148]=p.p57;
        self.scalar_static_f64[149]=(-self.scalar_static_f64[49]);
        self.scalar_static_f64[150]=p.p64;
        self.scalar_static_f64[151]=p.p63;
        self.scalar_static_f64[152]=p.p62;
        self.scalar_static_f64[153]=p.p47;
        self.scalar_static_f64[154]=(-self.scalar_static_f64[153]);
        self.scalar_static_f64[155]=p.p48;
        self.scalar_static_f64[156]=(-self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=p.p49;
        self.scalar_static_f64[158]=(-self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=p.p56;
        self.scalar_static_f64[160]=(-self.scalar_static_f64[20]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[20]*self.scalar_static_f64[153]);
        self.scalar_static_f64[162]=(self.scalar_static_f64[153]*self.scalar_static_f64[160]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[20]*self.scalar_static_f64[155]);
        self.scalar_static_f64[164]=(self.scalar_static_f64[155]*self.scalar_static_f64[160]);
        self.scalar_static_f64[165]=(self.scalar_static_f64[20]*self.scalar_static_f64[157]);
        self.scalar_static_f64[166]=(self.scalar_static_f64[157]*self.scalar_static_f64[160]);
        self.scalar_static_f64[167]=(self.scalar_static_f64[20]*self.scalar_static_f64[159]);
        self.scalar_static_f64[168]=(self.scalar_static_f64[159]*self.scalar_static_f64[160]);
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
        self.scalar_static_f64[169]=(temperature+self.scalar_static_f64[23]);
        self.scalar_static_f64[170]=(if self.scalar_static_bool[1]{self.scalar_static_f64[169]}else{0.0});
        self.scalar_static_f64[171]=(if self.scalar_static_bool[2]{self.scalar_static_f64[24]}else{self.scalar_static_f64[170]});
        self.scalar_static_f64[172]=(self.scalar_static_f64[171]*8.617333262145179e-5);
        self.scalar_static_f64[173]=(self.scalar_static_f64[172]*0.1);
        self.scalar_static_f64[174]=(1.0/self.scalar_static_f64[172]);
        self.scalar_static_f64[175]=(self.scalar_static_f64[172]+self.scalar_static_f64[172]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[175]+self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=(self.scalar_static_f64[172]*self.scalar_static_f64[172]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[177]+self.scalar_static_f64[177]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[177]*16.0);
        self.scalar_static_f64[180]=(self.scalar_static_f64[171]*0.000702);
        self.scalar_static_f64[181]=(self.scalar_static_f64[171]*self.scalar_static_f64[180]);
        self.scalar_static_f64[182]=(self.scalar_static_f64[171]+1108.0);
        self.scalar_static_f64[183]=(self.scalar_static_f64[181]/self.scalar_static_f64[182]);
        self.scalar_static_f64[184]=(1.16-self.scalar_static_f64[183]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[171]-self.scalar_static_f64[28]);
        self.scalar_static_f64[186]=(self.scalar_static_f64[171]/self.scalar_static_f64[28]);
        self.scalar_static_f64[187]=(self.scalar_static_f64[185]*self.scalar_static_f64[35]);
        self.scalar_static_f64[188]=(self.scalar_static_f64[34]-self.scalar_static_f64[187]);
        self.scalar_static_f64[189]=f64::powf(self.scalar_static_f64[186],self.scalar_static_f64[37]);
        self.scalar_static_f64[190]=(self.scalar_static_f64[36]*self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=f64::powf(self.scalar_static_f64[186],self.scalar_static_f64[39]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[38]*self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[185]*self.scalar_static_f64[41]);
        self.scalar_static_f64[194]=(1.0+self.scalar_static_f64[193]);
        self.scalar_static_f64[195]=(self.scalar_static_f64[40]*self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[186]*self.scalar_static_f64[42]);
        self.scalar_static_f64[197]=(3.0*self.scalar_static_f64[172]);
        self.scalar_static_f64[198]=(self.scalar_static_f64[186]).ln();
        self.scalar_static_f64[199]=(self.scalar_static_f64[197]*self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=(self.scalar_static_f64[196]-self.scalar_static_f64[199]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[33]*self.scalar_static_f64[186]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[200]-self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=(self.scalar_static_f64[184]+self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=(self.scalar_static_f64[203]-0.2);
        self.scalar_static_f64[205]=(self.scalar_static_f64[204]*self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[177]+self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=(self.scalar_static_f64[206]).sqrt();
        self.scalar_static_f64[208]=(self.scalar_static_f64[204]+self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=(0.5*self.scalar_static_f64[208]);
        self.scalar_static_f64[210]=(0.2+self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[210]).sqrt();
        self.scalar_static_f64[212]=(1.0/self.scalar_static_f64[192]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[4]*self.scalar_static_f64[192]);
        self.scalar_static_f64[214]=(self.scalar_static_f64[4]*self.scalar_static_f64[195]);
        self.scalar_static_f64[215]=(self.scalar_static_f64[43]/self.scalar_static_f64[195]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[192]*self.scalar_static_f64[46]);
        self.scalar_static_f64[217]=(0.5*self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=(self.scalar_static_f64[174]*self.scalar_static_f64[217]);
        self.scalar_static_f64[219]=(self.scalar_static_f64[218]).ln();
        self.scalar_static_f64[220]=(self.scalar_static_f64[219]-0.6);
        self.scalar_static_f64[221]=(self.scalar_static_f64[172]*self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[188]+self.scalar_static_f64[55]);
        self.scalar_static_f64[223]=(if self.scalar_static_bool[5]{self.scalar_static_f64[222]}else{self.scalar_static_f64[188]});
        self.scalar_static_f64[224]=(if self.scalar_static_bool[0]{self.scalar_static_f64[223]}else{0.0});
        self.scalar_static_f64[225]=(self.scalar_static_f64[57]-self.scalar_static_f64[188]);
        self.scalar_static_f64[226]=(-self.scalar_static_f64[188]);
        self.scalar_static_f64[227]=(if self.scalar_static_bool[5]{self.scalar_static_f64[225]}else{self.scalar_static_f64[226]});
        self.scalar_static_f64[228]=(if self.scalar_static_bool[6]{self.scalar_static_f64[227]}else{self.scalar_static_f64[224]});
        self.scalar_static_f64[229]=(self.scalar_static_f64[190]*self.scalar_static_f64[61]);
        self.scalar_static_f64[230]=(if self.scalar_static_bool[7]{self.scalar_static_f64[229]}else{self.scalar_static_f64[190]});
        self.scalar_static_f64[231]=(self.scalar_static_f64[49]*self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(self.scalar_static_f64[211]*self.scalar_static_f64[67]);
        self.scalar_static_f64[233]=(self.scalar_static_f64[179]*2.0);
        self.scalar_static_f64[234]=(self.scalar_static_f64[172]/self.scalar_static_f64[216]);
        self.scalar_static_f64[235]=(self.scalar_static_f64[16]*self.scalar_static_f64[232]);
        self.scalar_static_f64[236]=(1.0+self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=(if self.scalar_static_bool[12]{self.scalar_static_f64[236]}else{0.0});
        self.scalar_static_f64[238]=(self.scalar_static_f64[231]*self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=(self.scalar_static_f64[176]+self.scalar_static_f64[176]);
        self.scalar_static_f64[240]=(self.scalar_static_f64[5]*self.scalar_static_f64[239]);
        self.scalar_static_bool[26]=(self.scalar_static_f64[215]>0.0);
        self.scalar_static_f64[241]=(-self.scalar_static_f64[214]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[184]/self.scalar_static_f64[172]);
        self.scalar_static_f64[243]=(self.scalar_static_f64[123]-self.scalar_static_f64[242]);
        self.scalar_static_f64[244]=(self.scalar_static_f64[198]*self.scalar_static_f64[124]);
        self.scalar_static_f64[245]=(self.scalar_static_f64[243]+self.scalar_static_f64[244]);
        self.scalar_static_f64[246]=(self.scalar_static_f64[245]/self.scalar_static_f64[125]);
        self.scalar_static_f64[247]=(self.scalar_static_f64[246]).exp();
        self.scalar_static_f64[248]=(self.scalar_static_f64[247]*self.scalar_static_f64[126]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[247]*self.scalar_static_f64[127]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[247]*self.scalar_static_f64[128]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[185]*self.scalar_static_f64[130]);
        self.scalar_static_f64[252]=(self.scalar_static_f64[129]-self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[185]*self.scalar_static_f64[132]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[131]-self.scalar_static_f64[253]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[185]*self.scalar_static_f64[134]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[133]-self.scalar_static_f64[255]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[185]*self.scalar_static_f64[136]);
        self.scalar_static_f64[258]=(1.0+self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(self.scalar_static_f64[135]*self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[185]*self.scalar_static_f64[138]);
        self.scalar_static_f64[261]=(1.0+self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[137]*self.scalar_static_f64[261]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[185]*self.scalar_static_f64[140]);
        self.scalar_static_f64[264]=(1.0+self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=(self.scalar_static_f64[139]*self.scalar_static_f64[264]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[186]-1.0);
        self.scalar_static_f64[267]=(self.scalar_static_f64[266]*self.scalar_static_f64[142]);
        self.scalar_static_f64[268]=(1.0+self.scalar_static_f64[267]);
        self.scalar_static_f64[269]=(self.scalar_static_f64[141]*self.scalar_static_f64[268]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[266]*self.scalar_static_f64[144]);
        self.scalar_static_f64[271]=(1.0+self.scalar_static_f64[270]);
        self.scalar_static_f64[272]=(self.scalar_static_f64[143]*self.scalar_static_f64[271]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[266]*self.scalar_static_f64[146]);
        self.scalar_static_f64[274]=(1.0+self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[145]*self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[118]*self.scalar_static_f64[248]);
        self.scalar_static_f64[277]=(self.scalar_static_f64[121]*self.scalar_static_f64[249]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[276]+self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(self.scalar_static_f64[49]*self.scalar_static_f64[250]);
        self.scalar_static_f64[280]=(self.scalar_static_f64[278]+self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(self.scalar_static_f64[172]*self.scalar_static_f64[125]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[250]*self.scalar_static_f64[149]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[172]*self.scalar_static_f64[275]);
        self.scalar_static_f64[284]=(self.scalar_static_f64[172]*self.scalar_static_f64[272]);
        self.scalar_static_f64[285]=(self.scalar_static_f64[172]*self.scalar_static_f64[269]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[110]*self.scalar_static_f64[248]);
        self.scalar_static_f64[287]=(self.scalar_static_f64[115]*self.scalar_static_f64[249]);
        self.scalar_static_f64[288]=(self.scalar_static_f64[286]+self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[279]+self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[118]*self.scalar_static_f64[259]);
        self.scalar_static_f64[291]=(self.scalar_static_f64[121]*self.scalar_static_f64[262]);
        self.scalar_static_f64[292]=(self.scalar_static_f64[49]*self.scalar_static_f64[265]);
        self.scalar_static_f64[293]=(self.scalar_static_f64[110]*self.scalar_static_f64[259]);
        self.scalar_static_f64[294]=(self.scalar_static_f64[115]*self.scalar_static_f64[262]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[186]*self.scalar_static_f64[160]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[20]*self.scalar_static_f64[186]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[295]/self.scalar_static_f64[281]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[296]/self.scalar_static_f64[281]);
        self.scalar_static_f64[299]=(-self.scalar_static_f64[297]);
        self.scalar_static_f64[300]=(-self.scalar_static_f64[298]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[296]/self.scalar_static_f64[283]);
        self.scalar_static_f64[302]=(self.scalar_static_f64[295]/self.scalar_static_f64[283]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[150]*self.scalar_static_f64[301]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[150]*self.scalar_static_f64[302]);
        self.scalar_static_f64[305]=(self.scalar_static_f64[296]/self.scalar_static_f64[284]);
        self.scalar_static_f64[306]=(self.scalar_static_f64[295]/self.scalar_static_f64[284]);
        self.scalar_static_f64[307]=(self.scalar_static_f64[151]*self.scalar_static_f64[305]);
        self.scalar_static_f64[308]=(self.scalar_static_f64[151]*self.scalar_static_f64[306]);
        self.scalar_static_f64[309]=(self.scalar_static_f64[296]/self.scalar_static_f64[285]);
        self.scalar_static_f64[310]=(self.scalar_static_f64[295]/self.scalar_static_f64[285]);
        self.scalar_static_f64[311]=(self.scalar_static_f64[152]*self.scalar_static_f64[309]);
        self.scalar_static_f64[312]=(self.scalar_static_f64[152]*self.scalar_static_f64[310]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[20]/self.scalar_static_f64[252]);
        self.scalar_static_f64[314]=(self.scalar_static_f64[160]/self.scalar_static_f64[252]);
        self.scalar_static_f64[315]=(self.scalar_static_f64[20]/self.scalar_static_f64[254]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[160]/self.scalar_static_f64[254]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[20]/self.scalar_static_f64[256]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[160]/self.scalar_static_f64[256]);
        self.scalar_static_f64[319]=(self.scalar_static_f64[161]/self.scalar_static_f64[252]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[162]/self.scalar_static_f64[252]);
        self.scalar_static_f64[321]=(-self.scalar_static_f64[319]);
        self.scalar_static_f64[322]=(-self.scalar_static_f64[320]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[290]*self.scalar_static_f64[321]);
        self.scalar_static_f64[324]=(self.scalar_static_f64[290]*self.scalar_static_f64[322]);
        self.scalar_static_f64[325]=(self.scalar_static_f64[163]/self.scalar_static_f64[254]);
        self.scalar_static_f64[326]=(self.scalar_static_f64[164]/self.scalar_static_f64[254]);
        self.scalar_static_f64[327]=(-self.scalar_static_f64[325]);
        self.scalar_static_f64[328]=(-self.scalar_static_f64[326]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[291]*self.scalar_static_f64[327]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[291]*self.scalar_static_f64[328]);
        self.scalar_static_f64[331]=(self.scalar_static_f64[165]/self.scalar_static_f64[256]);
        self.scalar_static_f64[332]=(self.scalar_static_f64[166]/self.scalar_static_f64[256]);
        self.scalar_static_f64[333]=(-self.scalar_static_f64[331]);
        self.scalar_static_f64[334]=(-self.scalar_static_f64[332]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[292]*self.scalar_static_f64[333]);
        self.scalar_static_f64[336]=(self.scalar_static_f64[292]*self.scalar_static_f64[334]);
        self.scalar_static_f64[337]=(self.scalar_static_f64[293]*self.scalar_static_f64[321]);
        self.scalar_static_f64[338]=(self.scalar_static_f64[293]*self.scalar_static_f64[322]);
        self.scalar_static_f64[339]=(self.scalar_static_f64[294]*self.scalar_static_f64[327]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[294]*self.scalar_static_f64[328]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
