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
            params.p2 = 1.0;
            params.p3 = 0.0;
            params.p4 = 1.4;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 1000.0;
            params.p8 = -100.0;
            params.p9 = 500.0;
            params.p10 = 10000.0;
            params.p11 = 21.0;
            params.p12 = 1e-8;
            params.p13 = 9900000000.0;
            params.p14 = 1e-8;
            params.p15 = 9900000000.0;
            params.p16 = 1.0;
            params.p17 = -1.0;
            params.p18 = -1.0;
            params.p19 = 2e-9;
            params.p20 = 3.9;
            params.p21 = 1.0;
            params.p22 = 0.1;
            params.p23 = 0.0;
            params.p24 = 3e23;
            params.p25 = 1.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.1;
            params.p29 = 1e27;
            params.p30 = 1.0;
            params.p31 = 0.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = 0.0;
            params.p35 = 0.0;
            params.p36 = 1.0;
            params.p37 = 0.0;
            params.p38 = 0.0001;
            params.p39 = 1000.0;
            params.p40 = 0.05;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 1.0;
            params.p49 = 0.0;
            params.p50 = 3.1;
            params.p51 = 4.5;
            params.p52 = 2.0;
            params.p53 = 0.0;
            params.p54 = 5e25;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.375;
            params.p59 = 0.063;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 0.375;
            params.p64 = 0.063;
            params.p65 = 1e-5;
            params.p66 = 1.0;
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
    pub nodes: [usize; 7],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 67]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 3]>,
    pub(crate) ddt_state_previous: Box<[f64; 3]>,
    pub(crate) ddt_state_older: Box<[f64; 3]>,
    pub(crate) ddt_state_initialized: Box<[bool; 3]>,
    pub(crate) ddt_derivative_current: Box<[f64; 3]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 3]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 364]>,
    pub(crate) scalar_static_bool: Box<[bool; 64]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 7;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["gii", "gi", "ci", "n"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 67;
    pub const VARIABLE_COUNT: usize = 432;
    pub const DDT_STATE_COUNT: usize = 3;
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
            scalar_static_f64: boxed_zero_f64_array::<364>(),
            scalar_static_bool: boxed_zero_bool_array::<64>(),
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
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subversion" => { validate_finite_parameter("SUBVERSION", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "revision" => { validate_finite_parameter("REVISION", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((-273.0, "-273.0")), false, Some((21.0, "21.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((21.0, "21.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_parameter("LMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_parameter("LMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_parameter("WMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_parameter("WMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swres" => { validate_parameter("SWRES", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "typep" => { validate_parameter("TYPEP", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxo" => { validate_parameter("TOXO", value, Some((5e-10, "5e-10")), false, Some((2e-6, "2e-6")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "epsroxo" => { validate_parameter("EPSROXO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqinv" => { validate_parameter("SWQINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tau" => { validate_parameter("TAU", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubo" => { validate_parameter("NSUBO", value, Some((1e18, "1e18")), false, Some((1e25, "1e25")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mnsubo" => { validate_parameter("MNSUBO", value, Some((1.0, "1.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dnsubo" => { validate_parameter("DNSUBO", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vnsubo" => { validate_parameter("VNSUBO", value, Some((-5.0, "-5.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nslpo" => { validate_parameter("NSLPO", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((1e24, "1e24")), false, Some((1e27, "1e27")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dwr" => { validate_finite_parameter("DWR", value)?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrl" => { validate_parameter("CFRL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrw" => { validate_parameter("CFRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rpv" => { validate_parameter("RPV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rend" => { validate_parameter("REND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "uac" => { validate_parameter("UAC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "uacred" => { validate_parameter("UACRED", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strshg" => { validate_finite_parameter("STRSHG", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strpv" => { validate_finite_parameter("STRPV", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strend" => { validate_finite_parameter("STREND", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strshs" => { validate_finite_parameter("STRSHS", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stuac" => { validate_finite_parameter("STUAC", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chibpo" => { validate_parameter("CHIBPO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lov" => { validate_parameter("LOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1e22, "1e22")), false, Some((1e26, "1e26")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovw" => { validate_parameter("IGOVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2o" => { validate_parameter("GC2O", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3o" => { validate_parameter("GC3O", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igchvlw" => { validate_parameter("IGCHVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovhvw" => { validate_parameter("IGOVHVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcohvo" => { validate_parameter("GCOHVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2hvo" => { validate_parameter("GC2HVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3hvo" => { validate_parameter("GC3HVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igmax" => { validate_parameter("IGMAX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "racnoise" => { validate_parameter("RACNOISE", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'mosvar'", name)),
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
        self.scalar_static_f64[0]=p.p20;
        self.scalar_static_f64[1]=(self.scalar_static_f64[0]/3.9);
        self.scalar_static_f64[2]=(3.453e-11*self.scalar_static_f64[1]);
        self.scalar_static_f64[3]=p.p19;
        self.scalar_static_f64[4]=(self.scalar_static_f64[2]/self.scalar_static_f64[3]);
        self.scalar_static_f64[5]=p.p24;
        self.scalar_static_f64[6]=p.p29;
        self.scalar_static_f64[7]=(3.348580862e-29*self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]).sqrt();
        self.scalar_static_f64[9]=(self.scalar_static_f64[8]/self.scalar_static_f64[4]);
        self.scalar_static_f64[10]=p.p54;
        self.scalar_static_f64[11]=(3.348580862e-29*self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=(self.scalar_static_f64[11]).sqrt();
        self.scalar_static_f64[13]=(self.scalar_static_f64[12]/self.scalar_static_f64[4]);
        self.scalar_static_f64[14]=p.p30;
        self.scalar_static_bool[0]=(self.scalar_static_f64[14]>0.0);
        self.scalar_static_f64[15]=(self.scalar_static_f64[14]*2.3807972);
        self.scalar_static_f64[16]=f64::powf(self.scalar_static_f64[4],0.6666666666666666);
        self.scalar_static_f64[17]=(self.scalar_static_f64[15]*self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(if self.scalar_static_bool[0]{self.scalar_static_f64[17]}else{0.0});
        self.scalar_static_f64[19]=p.p17;
        self.scalar_static_bool[1]=(self.scalar_static_f64[19]<0.0);
        self.scalar_static_bool[2]=(self.scalar_static_bool[0]&&self.scalar_static_bool[1]);
        self.scalar_static_f64[20]=(self.scalar_static_f64[18]*1.2514650134837189);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[2]{self.scalar_static_f64[20]}else{self.scalar_static_f64[18]});
        self.scalar_static_bool[3]=(!self.scalar_static_bool[0]);
        self.scalar_static_f64[22]=(if self.scalar_static_bool[3]{0.0}else{self.scalar_static_f64[21]});
        self.scalar_static_f64[23]=p.p48;
        self.scalar_static_f64[24]=(0.3333333333333333*self.scalar_static_f64[23]);
        self.scalar_static_f64[25]=(if self.scalar_static_bool[1]{self.scalar_static_f64[24]}else{0.0});
        self.scalar_static_bool[4]=(!self.scalar_static_bool[1]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[23]*0.5);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[4]{self.scalar_static_f64[26]}else{self.scalar_static_f64[25]});
        self.scalar_static_f64[28]=(self.scalar_static_f64[3]/1e-9);
        self.scalar_static_f64[29]=p.p11;
        self.scalar_static_bool[5]=(self.scalar_static_f64[29]> -273.0);
        self.scalar_static_f64[30]=(if self.scalar_static_bool[5]{self.scalar_static_f64[29]}else{-273.0});
        self.scalar_static_f64[31]=(self.scalar_static_f64[30]+273.15);
        self.scalar_static_f64[32]=p.p3;
        self.scalar_static_f64[33]=p.p23;
        self.scalar_static_f64[34]=p.p42;
        self.scalar_static_f64[35]=p.p43;
        self.scalar_static_f64[36]=p.p36;
        self.scalar_static_f64[37]=p.p44;
        self.scalar_static_f64[38]=p.p37;
        self.scalar_static_f64[39]=p.p45;
        self.scalar_static_f64[40]=p.p38;
        self.scalar_static_f64[41]=p.p46;
        self.scalar_static_f64[42]=p.p39;
        self.scalar_static_f64[43]=p.p47;
        self.scalar_static_f64[44]=p.p40;
        self.scalar_static_f64[45]=p.p1;
        self.scalar_static_f64[46]=p.p0;
        self.scalar_static_f64[47]=p.p31;
        self.scalar_static_f64[48]=(self.scalar_static_f64[45]+self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p32;
        self.scalar_static_f64[50]=(self.scalar_static_f64[46]+self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p35;
        self.scalar_static_f64[52]=(self.scalar_static_f64[46]*self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=p.p34;
        self.scalar_static_f64[54]=(self.scalar_static_f64[45]*self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(self.scalar_static_f64[52]+self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(2.0*self.scalar_static_f64[55]);
        self.scalar_static_f64[57]=p.p16;
        self.scalar_static_f64[58]=p.p2;
        self.scalar_static_f64[59]=(self.scalar_static_f64[58]-1.0);
        self.scalar_static_f64[60]=(self.scalar_static_f64[59]*9.0);
        self.scalar_static_f64[61]=(3.0+self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[45]*self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(self.scalar_static_f64[45]*self.scalar_static_f64[46]);
        self.scalar_static_f64[64]=p.p33;
        self.scalar_static_f64[65]=(self.scalar_static_f64[46]+self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(2.0*self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=(self.scalar_static_f64[65]*12.0);
        self.scalar_static_bool[6]=(!(self.scalar_static_f64[57]!=0.0));
        self.scalar_static_f64[68]=p.p49;
        self.scalar_static_f64[69]=p.p55;
        self.scalar_static_f64[70]=(self.scalar_static_f64[50]*self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=(self.scalar_static_f64[48]*self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=(self.scalar_static_f64[71]*1000000000000.0);
        self.scalar_static_f64[73]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[72]}else{0.0});
        self.scalar_static_f64[74]=p.p56;
        self.scalar_static_f64[75]=(2.0*self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=p.p53;
        self.scalar_static_f64[77]=(self.scalar_static_f64[75]*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(self.scalar_static_f64[50]*self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=(1000000000000.0*self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[79]}else{0.0});
        self.scalar_static_f64[81]=p.p60;
        self.scalar_static_f64[82]=(self.scalar_static_f64[50]*self.scalar_static_f64[81]);
        self.scalar_static_f64[83]=(self.scalar_static_f64[48]*self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=(1000000000000.0*self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[84]}else{0.0});
        self.scalar_static_f64[86]=p.p61;
        self.scalar_static_f64[87]=(2.0*self.scalar_static_f64[86]);
        self.scalar_static_f64[88]=(self.scalar_static_f64[76]*self.scalar_static_f64[87]);
        self.scalar_static_f64[89]=(self.scalar_static_f64[50]*self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=(1000000000000.0*self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[90]}else{0.0});
        self.scalar_static_f64[92]=p.p52;
        self.scalar_static_f64[93]=p.p50;
        self.scalar_static_f64[94]=(1.0/self.scalar_static_f64[93]);
        self.scalar_static_f64[95]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[94]}else{0.0});
        self.scalar_static_f64[96]=p.p51;
        self.scalar_static_f64[97]=(1.0/self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[97]}else{0.0});
        self.scalar_static_f64[99]=(self.scalar_static_f64[93]*2.918995620956536e-49);
        self.scalar_static_f64[100]=(self.scalar_static_f64[99]).sqrt();
        self.scalar_static_f64[101]=(1.3333333333333333*self.scalar_static_f64[100]);
        self.scalar_static_f64[102]=(self.scalar_static_f64[101]/1.05457168e-34);
        self.scalar_static_f64[103]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[102]}else{0.0});
        self.scalar_static_f64[104]=(self.scalar_static_f64[3]*self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[104]}else{0.0});
        self.scalar_static_f64[106]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[105]}else{0.0});
        self.scalar_static_f64[107]=(self.scalar_static_f64[96]*2.918995620956536e-49);
        self.scalar_static_f64[108]=(self.scalar_static_f64[107]).sqrt();
        self.scalar_static_f64[109]=(1.3333333333333333*self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=(self.scalar_static_f64[109]/1.05457168e-34);
        self.scalar_static_f64[111]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[110]}else{self.scalar_static_f64[103]});
        self.scalar_static_f64[112]=(self.scalar_static_f64[3]*self.scalar_static_f64[111]);
        self.scalar_static_f64[113]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[112]}else{0.0});
        self.scalar_static_f64[114]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[113]}else{0.0});
        self.scalar_static_f64[115]=p.p59;
        self.scalar_static_bool[7]=(self.scalar_static_f64[115]<0.0);
        self.scalar_static_bool[8]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[7]);
        self.scalar_static_f64[116]=p.p58;
        self.scalar_static_f64[117]=(-0.495*self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(self.scalar_static_f64[117]/self.scalar_static_f64[115]);
        self.scalar_static_f64[119]=(if self.scalar_static_bool[8]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_bool[9]=(!self.scalar_static_bool[7]);
        self.scalar_static_bool[10]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[9]);
        self.scalar_static_f64[120]=(if self.scalar_static_bool[10]{0.0}else{self.scalar_static_f64[119]});
        self.scalar_static_f64[121]=p.p64;
        self.scalar_static_bool[11]=(self.scalar_static_f64[121]<0.0);
        self.scalar_static_bool[12]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[11]);
        self.scalar_static_f64[122]=p.p63;
        self.scalar_static_f64[123]=(-0.495*self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[123]/self.scalar_static_f64[121]);
        self.scalar_static_f64[125]=(if self.scalar_static_bool[12]{self.scalar_static_f64[124]}else{0.0});
        self.scalar_static_bool[13]=(!self.scalar_static_bool[11]);
        self.scalar_static_bool[14]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[13]);
        self.scalar_static_f64[126]=(if self.scalar_static_bool[14]{0.0}else{self.scalar_static_f64[125]});
        self.scalar_static_f64[127]=p.p57;
        self.scalar_static_f64[128]=p.p62;
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[68]!=0.0));
        self.scalar_static_f64[129]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[120]});
        self.scalar_static_f64[130]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[126]});
        self.scalar_static_f64[131]=(if self.scalar_static_bool[15]{0.1}else{self.scalar_static_f64[95]});
        self.scalar_static_f64[132]=(if self.scalar_static_bool[15]{0.1}else{self.scalar_static_f64[98]});
        self.scalar_static_f64[133]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[105]});
        self.scalar_static_f64[134]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[106]});
        self.scalar_static_f64[135]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[113]});
        self.scalar_static_f64[136]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[114]});
        self.scalar_static_f64[137]=p.p26;
        self.scalar_static_f64[138]=p.p27;
        self.scalar_static_f64[139]=p.p28;
        self.scalar_static_f64[140]=(0.5*self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=(self.scalar_static_f64[139]+1e-32);
        self.scalar_static_f64[142]=(self.scalar_static_f64[141]).sqrt();
        self.scalar_static_f64[143]=p.p25;
        self.scalar_static_f64[144]=(self.scalar_static_f64[22]*0.75);
        self.scalar_static_bool[16]=(self.scalar_static_f64[6]<1e27);
        self.scalar_static_f64[145]=(-self.scalar_static_f64[19]);
        self.scalar_static_f64[146]=p.p18;
        self.scalar_static_f64[147]=(self.scalar_static_f64[145]*self.scalar_static_f64[146]);
        self.scalar_static_bool[17]=(!self.scalar_static_bool[16]);
        self.scalar_static_f64[148]=p.p21;
        self.scalar_static_bool[18]=(self.scalar_static_f64[148]<1.0);
        self.scalar_static_f64[149]=(self.scalar_static_f64[28]*0.37);
        self.scalar_static_f64[150]=(1.0+self.scalar_static_f64[149]);
        self.scalar_static_bool[19]=(self.scalar_static_f64[22]>0.0);
        self.scalar_static_f64[151]=p.p41;
        self.scalar_static_f64[152]=(self.scalar_static_f64[19]*self.scalar_static_f64[146]);
        self.scalar_static_bool[20]=(-1.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[21]=(0.0!=self.scalar_static_f64[68]);
        self.scalar_static_bool[22]=(1.0==self.scalar_static_f64[146]);
        self.scalar_static_f64[153]=p.p22;
        self.scalar_static_f64[154]=(self.scalar_static_f64[19]*0.5);
        self.scalar_static_f64[155]=(0.5*self.scalar_static_f64[145]);
        self.scalar_static_f64[156]=(-self.scalar_static_f64[56]);
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
        self.scalar_static_f64[157]=(temperature+self.scalar_static_f64[32]);
        self.scalar_static_f64[158]=(self.scalar_static_f64[157]-273.15);
        self.scalar_static_f64[159]=(273.15+self.scalar_static_f64[158]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[159]*self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[159]-self.scalar_static_f64[31]);
        self.scalar_static_f64[162]=(self.scalar_static_f64[159]/self.scalar_static_f64[31]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[31]/self.scalar_static_f64[159]);
        self.scalar_static_f64[164]=(self.scalar_static_f64[159]*1.3806505e-23);
        self.scalar_static_f64[165]=(self.scalar_static_f64[164]/1.6021918e-19);
        self.scalar_static_f64[166]=(self.scalar_static_f64[165]*100.0);
        self.scalar_static_f64[167]=(self.scalar_static_f64[165]*self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=(1.0/self.scalar_static_f64[165]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[161]*self.scalar_static_f64[34]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[33]+self.scalar_static_f64[169]);
        self.scalar_static_f64[171]=f64::powf(self.scalar_static_f64[163],self.scalar_static_f64[35]);
        self.scalar_static_f64[172]=(self.scalar_static_f64[171]*self.scalar_static_f64[36]);
        self.scalar_static_f64[173]=f64::powf(self.scalar_static_f64[163],self.scalar_static_f64[37]);
        self.scalar_static_f64[174]=(self.scalar_static_f64[173]*self.scalar_static_f64[38]);
        self.scalar_static_f64[175]=f64::powf(self.scalar_static_f64[163],self.scalar_static_f64[39]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[175]*self.scalar_static_f64[40]);
        self.scalar_static_f64[177]=f64::powf(self.scalar_static_f64[163],self.scalar_static_f64[41]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[177]*self.scalar_static_f64[42]);
        self.scalar_static_f64[179]=f64::powf(self.scalar_static_f64[162],self.scalar_static_f64[43]);
        self.scalar_static_f64[180]=(self.scalar_static_f64[179]*self.scalar_static_f64[44]);
        self.scalar_static_f64[181]=(self.scalar_static_f64[159]*3.05e-7);
        self.scalar_static_f64[182]=(9.025e-5+self.scalar_static_f64[181]);
        self.scalar_static_f64[183]=(self.scalar_static_f64[159]*self.scalar_static_f64[182]);
        self.scalar_static_f64[184]=(1.179-self.scalar_static_f64[183]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[159]*0.00045);
        self.scalar_static_f64[186]=(1.045+self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=(self.scalar_static_f64[159]*0.0014);
        self.scalar_static_f64[188]=(0.523+self.scalar_static_f64[187]);
        self.scalar_static_f64[189]=(self.scalar_static_f64[160]*1.48e-6);
        self.scalar_static_f64[190]=(self.scalar_static_f64[188]-self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(self.scalar_static_f64[186]*self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[160]*self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[192]/90000.0);
        self.scalar_static_bool[23]=(self.scalar_static_f64[193]>0.001);
        self.scalar_static_f64[194]=(if self.scalar_static_bool[23]{self.scalar_static_f64[193]}else{0.001});
        self.scalar_static_f64[195]=(self.scalar_static_f64[194]).sqrt();
        self.scalar_static_f64[196]=(self.scalar_static_f64[195]).sqrt();
        self.scalar_static_f64[197]=(self.scalar_static_f64[195]*2.5e25);
        self.scalar_static_f64[198]=(self.scalar_static_f64[196]*self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=(1.0/self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=(2.0*self.scalar_static_f64[165]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[5]*self.scalar_static_f64[199]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[201]).ln();
        self.scalar_static_f64[203]=(self.scalar_static_f64[200]*self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=(self.scalar_static_f64[184]+self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(self.scalar_static_f64[6]*self.scalar_static_f64[199]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[205]).ln();
        self.scalar_static_f64[207]=(self.scalar_static_f64[200]*self.scalar_static_f64[206]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[184]+self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=(self.scalar_static_f64[165]*6.0);
        self.scalar_static_f64[210]=(self.scalar_static_f64[184]+self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[168]).sqrt();
        self.scalar_static_f64[212]=(self.scalar_static_f64[9]*self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[212]*self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=(1.0/self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=(self.scalar_static_f64[212]*0.7071067811865475);
        self.scalar_static_f64[216]=(1.0+self.scalar_static_f64[215]);
        self.scalar_static_f64[217]=(1.0/self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=(self.scalar_static_f64[216]*1e-5);
        self.scalar_static_f64[219]=(self.scalar_static_f64[168]*self.scalar_static_f64[208]);
        self.scalar_static_f64[220]=(self.scalar_static_f64[13]*self.scalar_static_f64[211]);
        self.scalar_static_f64[221]=(self.scalar_static_f64[220]*self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=(0.7071067811865475*self.scalar_static_f64[220]);
        self.scalar_static_f64[223]=(1.0+self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=(1e-5*self.scalar_static_f64[223]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[220]*0.7324648775608221);
        self.scalar_static_f64[226]=(1.25+self.scalar_static_f64[225]);
        self.scalar_static_bool[24]=(self.scalar_static_f64[219]<460.51701859880916);
        self.scalar_static_f64[227]=(-self.scalar_static_f64[219]);
        self.scalar_static_f64[228]=(self.scalar_static_f64[227]).exp();
        self.scalar_static_f64[229]=(if self.scalar_static_bool[24]{self.scalar_static_f64[228]}else{0.0});
        self.scalar_static_bool[25]=(!self.scalar_static_bool[24]);
        self.scalar_static_f64[230]=(self.scalar_static_f64[219]-460.51701859880916);
        self.scalar_static_f64[231]=(0.5*self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(0.3333333333333333*self.scalar_static_f64[230]);
        self.scalar_static_f64[233]=(1.0+self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(self.scalar_static_f64[231]*self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=(1.0+self.scalar_static_f64[234]);
        self.scalar_static_f64[236]=(self.scalar_static_f64[230]*self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=(1.0+self.scalar_static_f64[236]);
        self.scalar_static_f64[238]=(1e-200/self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=(if self.scalar_static_bool[25]{self.scalar_static_f64[238]}else{self.scalar_static_f64[229]});
        self.scalar_static_f64[240]=(self.scalar_static_f64[172]*self.scalar_static_f64[46]);
        self.scalar_static_f64[241]=(self.scalar_static_f64[240]/self.scalar_static_f64[62]);
        self.scalar_static_f64[242]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[241]}else{0.0});
        self.scalar_static_f64[243]=(self.scalar_static_f64[174]/self.scalar_static_f64[63]);
        self.scalar_static_f64[244]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[243]}else{0.0});
        self.scalar_static_f64[245]=(self.scalar_static_f64[176]/self.scalar_static_f64[66]);
        self.scalar_static_f64[246]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[245]}else{0.0});
        self.scalar_static_f64[247]=(self.scalar_static_f64[178]*self.scalar_static_f64[45]);
        self.scalar_static_f64[248]=(self.scalar_static_f64[247]/self.scalar_static_f64[67]);
        self.scalar_static_f64[249]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[248]}else{0.0});
        self.scalar_static_bool[26]=(self.scalar_static_f64[242]>0.001);
        self.scalar_static_bool[27]=(self.scalar_static_f64[242]<1000.0);
        self.scalar_static_f64[250]=(if self.scalar_static_bool[27]{self.scalar_static_f64[242]}else{1000.0});
        self.scalar_static_f64[251]=(if self.scalar_static_bool[26]{self.scalar_static_f64[250]}else{0.001});
        self.scalar_static_f64[252]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[251]}else{self.scalar_static_f64[242]});
        self.scalar_static_bool[28]=(self.scalar_static_f64[244]>0.001);
        self.scalar_static_bool[29]=(self.scalar_static_f64[244]<100.0);
        self.scalar_static_f64[253]=(if self.scalar_static_bool[29]{self.scalar_static_f64[244]}else{100.0});
        self.scalar_static_f64[254]=(if self.scalar_static_bool[28]{self.scalar_static_f64[253]}else{0.001});
        self.scalar_static_f64[255]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[254]}else{self.scalar_static_f64[244]});
        self.scalar_static_bool[30]=(self.scalar_static_f64[246]>0.001);
        self.scalar_static_bool[31]=(self.scalar_static_f64[246]<1000.0);
        self.scalar_static_f64[256]=(if self.scalar_static_bool[31]{self.scalar_static_f64[246]}else{1000.0});
        self.scalar_static_f64[257]=(if self.scalar_static_bool[30]{self.scalar_static_f64[256]}else{0.001});
        self.scalar_static_f64[258]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[257]}else{self.scalar_static_f64[246]});
        self.scalar_static_bool[32]=(self.scalar_static_f64[249]>0.001);
        self.scalar_static_bool[33]=(self.scalar_static_f64[249]<1000.0);
        self.scalar_static_f64[259]=(if self.scalar_static_bool[33]{self.scalar_static_f64[249]}else{1000.0});
        self.scalar_static_f64[260]=(if self.scalar_static_bool[32]{self.scalar_static_f64[259]}else{0.001});
        self.scalar_static_f64[261]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[260]}else{self.scalar_static_f64[249]});
        self.scalar_static_bool[34]=(self.scalar_static_f64[180]>0.001);
        self.scalar_static_bool[35]=(self.scalar_static_f64[180]<20.0);
        self.scalar_static_f64[262]=(if self.scalar_static_bool[35]{self.scalar_static_f64[180]}else{20.0});
        self.scalar_static_f64[263]=(if self.scalar_static_bool[34]{self.scalar_static_f64[262]}else{0.001});
        self.scalar_static_f64[264]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[263]}else{self.scalar_static_f64[180]});
        self.scalar_static_f64[265]=(1.0/self.scalar_static_f64[252]);
        self.scalar_static_f64[266]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[265]}else{0.0});
        self.scalar_static_f64[267]=(1.0/self.scalar_static_f64[255]);
        self.scalar_static_f64[268]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[267]}else{0.0});
        self.scalar_static_f64[269]=(1.0/self.scalar_static_f64[258]);
        self.scalar_static_f64[270]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[269]}else{0.0});
        self.scalar_static_f64[271]=(1.0/self.scalar_static_f64[261]);
        self.scalar_static_f64[272]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[271]}else{0.0});
        self.scalar_static_f64[273]=(12.0*self.scalar_static_f64[264]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[46]*self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[274]/self.scalar_static_f64[45]);
        self.scalar_static_f64[276]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[266]});
        self.scalar_static_f64[278]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[268]});
        self.scalar_static_f64[279]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[270]});
        self.scalar_static_f64[280]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[272]});
        self.scalar_static_f64[281]=(if self.scalar_static_bool[6]{0.0}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[282]=f64::powf(self.scalar_static_f64[162],self.scalar_static_f64[92]);
        self.scalar_static_f64[283]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[282]}else{0.0});
        self.scalar_static_f64[284]=(self.scalar_static_f64[73]*self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[284]}else{self.scalar_static_f64[73]});
        self.scalar_static_f64[286]=(self.scalar_static_f64[80]*self.scalar_static_f64[283]);
        self.scalar_static_f64[287]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[286]}else{self.scalar_static_f64[80]});
        self.scalar_static_f64[288]=(self.scalar_static_f64[85]*self.scalar_static_f64[283]);
        self.scalar_static_f64[289]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[288]}else{self.scalar_static_f64[85]});
        self.scalar_static_f64[290]=(self.scalar_static_f64[91]*self.scalar_static_f64[283]);
        self.scalar_static_f64[291]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[290]}else{self.scalar_static_f64[91]});
        self.scalar_static_f64[292]=(self.scalar_static_f64[19]*self.scalar_static_f64[204]);
        self.scalar_static_f64[293]=(self.scalar_static_f64[184]+self.scalar_static_f64[292]);
        self.scalar_static_f64[294]=(0.5*self.scalar_static_f64[293]);
        self.scalar_static_f64[295]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[294]}else{0.0});
        self.scalar_static_f64[296]=(self.scalar_static_f64[19]*self.scalar_static_f64[210]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[184]+self.scalar_static_f64[296]);
        self.scalar_static_f64[298]=(0.5*self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[298]}else{0.0});
        self.scalar_static_f64[300]=(self.scalar_static_f64[165]*self.scalar_static_f64[127]);
        self.scalar_static_f64[301]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[300]}else{0.0});
        self.scalar_static_f64[302]=(self.scalar_static_f64[165]*self.scalar_static_f64[128]);
        self.scalar_static_f64[303]=(if (self.scalar_static_f64[68]!=0.0){self.scalar_static_f64[302]}else{0.0});
        self.scalar_static_f64[304]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[285]});
        self.scalar_static_f64[305]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[287]});
        self.scalar_static_f64[306]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[289]});
        self.scalar_static_f64[307]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[291]});
        self.scalar_static_f64[308]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[301]});
        self.scalar_static_f64[309]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[303]});
        self.scalar_static_f64[310]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[295]});
        self.scalar_static_f64[311]=(if self.scalar_static_bool[15]{0.0}else{self.scalar_static_f64[299]});
        self.scalar_static_f64[312]=(self.scalar_static_f64[217]*self.scalar_static_f64[217]);
        self.scalar_static_f64[313]=(0.1666666666666667*self.scalar_static_f64[312]);
        self.scalar_static_f64[314]=(0.7071067811865475*self.scalar_static_f64[313]);
        self.scalar_static_f64[315]=(1.0-self.scalar_static_f64[239]);
        self.scalar_static_f64[316]=(-self.scalar_static_f64[218]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[212]*0.7324648775608221);
        self.scalar_static_f64[318]=(1.25+self.scalar_static_f64[317]);
        self.scalar_static_f64[319]=(1.0/self.scalar_static_f64[318]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[216]*1.25);
        self.scalar_static_f64[321]=(0.5*self.scalar_static_f64[213]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[213]*0.25);
        self.scalar_static_f64[323]=(self.scalar_static_f64[219]+3.0);
        self.scalar_static_f64[324]=(self.scalar_static_f64[219]-230.25850929940458);
        self.scalar_static_f64[325]=(self.scalar_static_f64[163]).sqrt();
        self.scalar_static_f64[326]=(self.scalar_static_f64[184]*self.scalar_static_f64[146]);
        self.scalar_static_f64[327]=(if self.scalar_static_bool[20]{self.scalar_static_f64[326]}else{0.0});
        self.scalar_static_bool[36]=(self.scalar_static_f64[305]>0.0);
        self.scalar_static_bool[37]=(self.scalar_static_f64[307]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_bool[36]||self.scalar_static_bool[37]);
        self.scalar_static_bool[39]=(self.scalar_static_bool[21]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[328]=(self.scalar_static_f64[223]*1.25);
        self.scalar_static_f64[329]=(self.scalar_static_f64[328]/self.scalar_static_f64[226]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[329]-1.0);
        self.scalar_static_f64[331]=(self.scalar_static_f64[330]/self.scalar_static_f64[226]);
        self.scalar_static_f64[332]=(0.5*self.scalar_static_f64[221]);
        self.scalar_static_f64[333]=(self.scalar_static_f64[221]*0.25);
        self.scalar_static_bool[40]=(!self.scalar_static_bool[39]);
        self.scalar_static_bool[41]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[38]);
        self.scalar_static_bool[42]=(self.scalar_static_bool[37]&&self.scalar_static_bool[22]);
        self.scalar_static_bool[43]=(self.scalar_static_bool[41]&&self.scalar_static_bool[42]);
        self.scalar_static_bool[44]=(self.scalar_static_bool[11]&&self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=(self.scalar_static_bool[43]&&true);
        self.scalar_static_f64[334]=(self.scalar_static_f64[184]-self.scalar_static_f64[311]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[43]&&false);
        self.scalar_static_f64[335]=(self.scalar_static_f64[184]-self.scalar_static_f64[310]);
        self.scalar_static_bool[47]=(self.scalar_static_bool[36]&&self.scalar_static_bool[41]);
        self.scalar_static_bool[48]=(self.scalar_static_bool[7]&&self.scalar_static_bool[47]);
        self.scalar_static_bool[49]=(true&&self.scalar_static_bool[47]);
        self.scalar_static_bool[50]=(false&&self.scalar_static_bool[47]);
        self.scalar_static_bool[51]=(self.scalar_static_f64[304]>0.0);
        self.scalar_static_bool[52]=(self.scalar_static_f64[306]>0.0);
        self.scalar_static_bool[53]=(self.scalar_static_bool[51]||self.scalar_static_bool[52]);
        self.scalar_static_bool[54]=((self.scalar_static_f64[68]!=0.0)&&self.scalar_static_bool[53]);
        self.scalar_static_bool[55]=(self.scalar_static_bool[22]&&self.scalar_static_bool[52]);
        self.scalar_static_bool[56]=(self.scalar_static_bool[54]&&self.scalar_static_bool[55]);
        self.scalar_static_bool[57]=(self.scalar_static_bool[11]&&self.scalar_static_bool[56]);
        self.scalar_static_bool[58]=(false&&self.scalar_static_bool[56]);
        self.scalar_static_bool[59]=(true&&self.scalar_static_bool[56]);
        self.scalar_static_bool[60]=(self.scalar_static_bool[51]&&self.scalar_static_bool[54]);
        self.scalar_static_bool[61]=(self.scalar_static_bool[7]&&self.scalar_static_bool[60]);
        self.scalar_static_bool[62]=(false&&self.scalar_static_bool[60]);
        self.scalar_static_bool[63]=(true&&self.scalar_static_bool[60]);
        self.scalar_static_f64[336]=(self.scalar_static_f64[19]*self.scalar_static_f64[168]);
        self.scalar_static_f64[337]=(self.scalar_static_f64[168]*self.scalar_static_f64[145]);
        self.scalar_static_f64[338]=(-self.scalar_static_f64[336]);
        self.scalar_static_f64[339]=(-self.scalar_static_f64[337]);
        self.scalar_static_f64[340]=(-self.scalar_static_f64[168]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[337]/self.scalar_static_f64[223]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[336]/self.scalar_static_f64[223]);
        self.scalar_static_f64[343]=(if self.scalar_static_bool[41]{self.scalar_static_f64[145]}else{0.0});
        self.scalar_static_f64[344]=(if self.scalar_static_bool[41]{self.scalar_static_f64[19]}else{0.0});
        self.scalar_static_f64[345]=(self.scalar_static_f64[19]*self.scalar_static_f64[343]);
        self.scalar_static_f64[346]=(self.scalar_static_f64[19]*self.scalar_static_f64[344]);
        self.scalar_static_f64[347]=(self.scalar_static_f64[168]*self.scalar_static_f64[345]);
        self.scalar_static_f64[348]=(self.scalar_static_f64[168]*self.scalar_static_f64[346]);
        self.scalar_static_f64[349]=(if self.scalar_static_bool[54]{self.scalar_static_f64[19]}else{0.0});
        self.scalar_static_f64[350]=(if self.scalar_static_bool[54]{self.scalar_static_f64[145]}else{0.0});
        self.scalar_static_f64[351]=(self.scalar_static_f64[19]*self.scalar_static_f64[349]);
        self.scalar_static_f64[352]=(self.scalar_static_f64[19]*self.scalar_static_f64[350]);
        self.scalar_static_f64[353]=(self.scalar_static_f64[168]*self.scalar_static_f64[351]);
        self.scalar_static_f64[354]=(self.scalar_static_f64[168]*self.scalar_static_f64[352]);
        self.scalar_static_f64[355]=(-self.scalar_static_f64[277]);
        self.scalar_static_f64[356]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_f64[357]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[355]}else{0.0});
        self.scalar_static_f64[358]=(-self.scalar_static_f64[278]);
        self.scalar_static_f64[359]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[278]}else{0.0});
        self.scalar_static_f64[360]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[358]}else{0.0});
        self.scalar_static_f64[361]=(-self.scalar_static_f64[279]);
        self.scalar_static_f64[362]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[279]}else{0.0});
        self.scalar_static_f64[363]=(if (self.scalar_static_f64[57]!=0.0){self.scalar_static_f64[361]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
