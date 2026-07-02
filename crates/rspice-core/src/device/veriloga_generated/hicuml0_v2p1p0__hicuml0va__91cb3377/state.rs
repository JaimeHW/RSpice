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
    pub p75: f64,
    pub p76: f64,
    pub p77: f64,
    pub p78: f64,
    pub p79: f64,
    pub p80: f64,
    pub p81: f64,
    pub p82: f64,
    pub p83: f64,
    pub p84: f64,
    pub p85: f64,
    pub p86: f64,
    pub p87: f64,
    pub p88: f64,
    pub p89: f64,
    pub p90: f64,
    pub p91: f64,
    pub p92: f64,
    pub p93: f64,
    pub p94: f64,
    pub p95: f64,
    pub p96: f64,
    pub p97: f64,
    pub p98: f64,
    pub p99: f64,
    pub p100: f64,
    pub p101: f64,
    pub p102: f64,
    pub p103: f64,
    pub p104: f64,
    pub p105: f64,
    pub p106: f64,
    pub p107: f64,
    pub p108: f64,
    pub p109: f64,
    pub p110: f64,
    pub p111: f64,
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
            params.p0 = 210.0;
            params.p1 = 1e-16;
            params.p2 = 0.0;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 1000000.0;
            params.p6 = 1000000.0;
            params.p7 = 0.0;
            params.p8 = 2.0;
            params.p9 = 1000000.0;
            params.p10 = if (params.p0 <= 200.0) { 1.0 } else { 0.0 };
            validate_parameter("fiqf", params.p10, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p11 = 1000000.0;
            params.p12 = 1000000.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 1e-18;
            params.p16 = 1.0;
            params.p17 = 0.0;
            params.p18 = 2.0;
            params.p19 = if (params.p0 <= 200.0) { 0.0 } else { 1e-16 };
            validate_parameter("ibcs", params.p19, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p20 = 1.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 2.5;
            params.p25 = 1000000.0;
            params.p26 = 0.0;
            params.p27 = 0.656;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 1.0;
            params.p34 = 1e-20;
            params.p35 = 0.9;
            params.p36 = 0.5;
            params.p37 = 2.5;
            params.p38 = 0.9;
            params.p39 = 0.5;
            params.p40 = 2.5;
            params.p41 = 1e-20;
            params.p42 = 0.7;
            params.p43 = 0.333;
            params.p44 = 100.0;
            params.p45 = 1e-20;
            params.p46 = 0.7;
            params.p47 = 0.333;
            params.p48 = 100.0;
            params.p49 = 1.0;
            params.p50 = 1e-20;
            params.p51 = 0.3;
            params.p52 = 0.3;
            params.p53 = 100.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 1.0;
            params.p59 = 0.0;
            params.p60 = 0.1;
            params.p61 = 150.0;
            params.p62 = 0.5;
            params.p63 = 100.0;
            params.p64 = 0.1;
            params.p65 = 0.0;
            params.p66 = 0.001;
            params.p67 = 2.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.167;
            params.p72 = 0.333;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 2.0;
            params.p76 = 1.2;
            params.p77 = 1.17;
            params.p78 = 1.17;
            params.p79 = 1.17;
            params.p80 = -0.000102377;
            params.p81 = 3.0;
            params.p82 = 3.5;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = if (params.p0 <= 200.0) { 1.0 } else { 0.0 };
            validate_parameter("flteft", params.p96, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p97 = -1.0;
            params.p98 = 0.0;
            params.p99 = 0.0;
            params.p100 = 0.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 27.0;
            params.p109 = 0.0;
            params.p110 = 1.0;
            params.p111 = 0.001;
            validate_parameter("minr", params.p111, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
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
    pub nodes: [usize; 10],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 112]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 9]>,
    pub(crate) ddt_state_previous: Box<[f64; 9]>,
    pub(crate) ddt_state_older: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
    pub(crate) ddt_derivative_current: Box<[f64; 9]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 9]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 538]>,
    pub(crate) scalar_static_bool: Box<[bool; 76]>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 5;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 5] = ["ci", "bi", "ei", "nd_qf_nqs", "nd_itf_nqs"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 112;
    pub const VARIABLE_COUNT: usize = 386;
    pub const DDT_STATE_COUNT: usize = 9;
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
            scalar_static_f64: boxed_zero_f64_array::<538>(),
            scalar_static_bool: boxed_zero_bool_array::<76>(),
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flitm" => { validate_parameter("flitm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcr" => { validate_parameter("mcr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aver" => { validate_parameter("aver", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rver" => { validate_parameter("rver", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqf" => { validate_parameter("iqf", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fiqf" => { validate_parameter("fiqf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqr" => { validate_parameter("iqr", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqfh" => { validate_parameter("iqfh", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tfh" => { validate_parameter("tfh", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahq" => { validate_parameter("ahq", value, Some((-0.9, "-0.9")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibes" => { validate_parameter("ibes", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbe" => { validate_parameter("mbe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ires" => { validate_parameter("ires", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mre" => { validate_parameter("mre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcs" => { validate_parameter("ibcs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbc" => { validate_parameter("mbc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vr0e" => { validate_parameter("vr0e", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vr0c" => { validate_parameter("vr0c", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje0" => { validate_parameter("cje0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ze" => { validate_parameter("ze", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aje" => { validate_parameter("aje", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdedc" => { validate_parameter("vdedc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zedc" => { validate_parameter("zedc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajedc" => { validate_parameter("ajedc", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbc" => { validate_parameter("fbc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbvl" => { validate_parameter("tbvl", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gte" => { validate_parameter("gte", value, Some((0.0, "0.0")), true, Some((20.0, "20.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetact" => { validate_finite_parameter("zetact", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetabet" => { validate_finite_parameter("zetabet", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgbe" => { validate_finite_parameter("dvgbe", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetavgbe" => { validate_finite_parameter("zetavgbe", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaci" => { validate_finite_parameter("zetaci", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbi" => { validate_finite_parameter("zetarbi", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbx" => { validate_finite_parameter("zetarbx", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarcx" => { validate_finite_parameter("zetarcx", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetare" => { validate_finite_parameter("zetare", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaiqf" => { validate_finite_parameter("zetaiqf", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flteft" => { validate_parameter("flteft", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaver" => { validate_finite_parameter("zetaver", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaiqfh" => { validate_finite_parameter("zetaiqfh", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aliqfh" => { validate_finite_parameter("aliqfh", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kiqfh" => { validate_finite_parameter("kiqfh", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarth" => { validate_finite_parameter("zetarth", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), true, Some((600.0, "600.0")), false, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hicumL0va'", name)),
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
        self.scalar_static_f64[0]=p.p110;
        self.scalar_static_f64[1]=p.p108;
        self.scalar_static_f64[2]=(self.scalar_static_f64[1]+273.15);
        self.scalar_static_f64[3]=(self.scalar_static_f64[2]*1.3806226e-23);
        self.scalar_static_f64[4]=(self.scalar_static_f64[3]/1.602176462e-19);
        self.scalar_static_f64[5]=p.p88;
        self.scalar_static_f64[6]=(self.scalar_static_f64[2]*self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=p.p76;
        self.scalar_static_f64[8]=p.p77;
        self.scalar_static_f64[9]=(self.scalar_static_f64[7]+self.scalar_static_f64[8]);
        self.scalar_static_f64[10]=(0.5*self.scalar_static_f64[9]);
        self.scalar_static_f64[11]=p.p78;
        self.scalar_static_f64[12]=(self.scalar_static_f64[7]+self.scalar_static_f64[11]);
        self.scalar_static_f64[13]=(0.5*self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=p.p79;
        self.scalar_static_f64[15]=(self.scalar_static_f64[11]+self.scalar_static_f64[14]);
        self.scalar_static_f64[16]=(0.5*self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=p.p80;
        self.scalar_static_f64[18]=(1.602176462e-19*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(self.scalar_static_f64[18]/1.3806226e-23);
        self.scalar_static_f64[20]=(3.0-self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=(self.scalar_static_f64[20]+1.0);
        self.scalar_static_f64[22]=p.p87;
        self.scalar_static_f64[23]=(self.scalar_static_f64[21]-self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=(self.scalar_static_f64[20]-1.5);
        self.scalar_static_f64[25]=p.p82;
        self.scalar_static_f64[26]=p.p81;
        self.scalar_static_f64[27]=(self.scalar_static_f64[25]-self.scalar_static_f64[26]);
        self.scalar_static_f64[28]=(self.scalar_static_f64[27]-0.5);
        self.scalar_static_f64[29]=(self.scalar_static_f64[7]-self.scalar_static_f64[8]);
        self.scalar_static_f64[30]=p.p34;
        self.scalar_static_f64[31]=p.p21;
        self.scalar_static_bool[0]=(self.scalar_static_f64[31]>0.0);
        self.scalar_static_f64[32]=p.p41;
        self.scalar_static_bool[1]=(self.scalar_static_f64[32]>0.0);
        self.scalar_static_bool[2]=(self.scalar_static_bool[0]&&self.scalar_static_bool[1]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[34]=(if (self.scalar_static_f64[33]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[33]!=0.0));
        self.scalar_static_f64[35]=(if self.scalar_static_bool[3]{0.0}else{self.scalar_static_f64[34]});
        self.scalar_static_f64[36]=p.p109;
        self.scalar_static_f64[37]=p.p35;
        self.scalar_static_f64[38]=(0.5*self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=(self.scalar_static_f64[38]/self.scalar_static_f64[4]);
        self.scalar_static_f64[40]=(self.scalar_static_f64[4]*2.0);
        self.scalar_static_f64[41]=(self.scalar_static_f64[39]).exp();
        self.scalar_static_f64[42]=(-self.scalar_static_f64[39]);
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]).exp();
        self.scalar_static_f64[44]=(self.scalar_static_f64[41]-self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=(self.scalar_static_f64[44]).ln();
        self.scalar_static_f64[46]=(self.scalar_static_f64[40]*self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=p.p36;
        self.scalar_static_f64[48]=p.p37;
        self.scalar_static_f64[49]=p.p38;
        self.scalar_static_f64[50]=(0.5*self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=(self.scalar_static_f64[50]/self.scalar_static_f64[4]);
        self.scalar_static_f64[52]=(self.scalar_static_f64[51]).exp();
        self.scalar_static_f64[53]=(-self.scalar_static_f64[51]);
        self.scalar_static_f64[54]=(self.scalar_static_f64[53]).exp();
        self.scalar_static_f64[55]=(self.scalar_static_f64[52]-self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(self.scalar_static_f64[55]).ln();
        self.scalar_static_f64[57]=(self.scalar_static_f64[40]*self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=p.p39;
        self.scalar_static_f64[59]=p.p40;
        self.scalar_static_f64[60]=p.p15;
        self.scalar_static_f64[61]=p.p17;
        self.scalar_static_f64[62]=(0.5*self.scalar_static_f64[20]);
        self.scalar_static_f64[63]=(0.5*self.scalar_static_f64[10]);
        self.scalar_static_f64[64]=p.p42;
        self.scalar_static_f64[65]=(0.5*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[65]/self.scalar_static_f64[4]);
        self.scalar_static_f64[67]=(self.scalar_static_f64[66]).exp();
        self.scalar_static_f64[68]=(-self.scalar_static_f64[66]);
        self.scalar_static_f64[69]=(self.scalar_static_f64[68]).exp();
        self.scalar_static_f64[70]=(self.scalar_static_f64[67]-self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=(self.scalar_static_f64[70]).ln();
        self.scalar_static_f64[72]=(self.scalar_static_f64[40]*self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=p.p43;
        self.scalar_static_f64[74]=p.p19;
        self.scalar_static_f64[75]=p.p1;
        self.scalar_static_f64[76]=p.p9;
        self.scalar_static_f64[77]=p.p95;
        self.scalar_static_f64[78]=p.p83;
        self.scalar_static_f64[79]=p.p62;
        self.scalar_static_f64[80]=(self.scalar_static_f64[22]-self.scalar_static_f64[6]);
        self.scalar_static_f64[81]=p.p61;
        self.scalar_static_f64[82]=p.p64;
        self.scalar_static_f64[83]=p.p89;
        self.scalar_static_f64[84]=p.p65;
        self.scalar_static_bool[4]=(self.scalar_static_f64[84]>0.0);
        self.scalar_static_f64[85]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[86]=p.p90;
        self.scalar_static_bool[5]=(!(self.scalar_static_f64[85]!=0.0));
        self.scalar_static_f64[87]=p.p54;
        self.scalar_static_f64[88]=p.p85;
        self.scalar_static_f64[89]=p.p86;
        self.scalar_static_f64[90]=p.p96;
        self.scalar_static_bool[6]=(1.0==self.scalar_static_f64[90]);
        self.scalar_static_f64[91]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_f64[92]=p.p57;
        self.scalar_static_bool[7]=(!(self.scalar_static_f64[91]!=0.0));
        self.scalar_static_f64[93]=p.p59;
        self.scalar_static_f64[94]=(self.scalar_static_f64[22]-1.0);
        self.scalar_static_bool[8]=(1.0==self.scalar_static_f64[35]);
        self.scalar_static_f64[95]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[96]=p.p99;
        self.scalar_static_f64[97]=p.p22;
        self.scalar_static_f64[98]=p.p100;
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[95]!=0.0));
        self.scalar_static_f64[99]=p.p23;
        self.scalar_static_f64[100]=p.p91;
        self.scalar_static_f64[101]=p.p46;
        self.scalar_static_f64[102]=(0.5*self.scalar_static_f64[101]);
        self.scalar_static_f64[103]=(self.scalar_static_f64[102]/self.scalar_static_f64[4]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[103]).exp();
        self.scalar_static_f64[105]=(-self.scalar_static_f64[103]);
        self.scalar_static_f64[106]=(self.scalar_static_f64[105]).exp();
        self.scalar_static_f64[107]=(self.scalar_static_f64[104]-self.scalar_static_f64[106]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[107]).ln();
        self.scalar_static_f64[109]=(self.scalar_static_f64[40]*self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=p.p45;
        self.scalar_static_f64[111]=p.p47;
        self.scalar_static_f64[112]=p.p51;
        self.scalar_static_f64[113]=(0.5*self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=(self.scalar_static_f64[113]/self.scalar_static_f64[4]);
        self.scalar_static_f64[115]=(self.scalar_static_f64[114]).exp();
        self.scalar_static_f64[116]=(-self.scalar_static_f64[114]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[116]).exp();
        self.scalar_static_f64[118]=(self.scalar_static_f64[115]-self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=(self.scalar_static_f64[118]).ln();
        self.scalar_static_f64[120]=(self.scalar_static_f64[40]*self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=p.p50;
        self.scalar_static_f64[122]=p.p52;
        self.scalar_static_f64[123]=p.p32;
        self.scalar_static_f64[124]=p.p30;
        self.scalar_static_f64[125]=p.p7;
        self.scalar_static_f64[126]=p.p97;
        self.scalar_static_f64[127]=p.p6;
        self.scalar_static_f64[128]=p.p84;
        self.scalar_static_f64[129]=p.p0;
        self.scalar_static_bool[10]=(self.scalar_static_f64[129]<=200.0);
        self.scalar_static_f64[130]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[131]=p.p101;
        self.scalar_static_f64[132]=p.p102;
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[130]!=0.0));
        self.scalar_static_f64[133]=p.p98;
        self.scalar_static_f64[134]=p.p12;
        self.scalar_static_f64[135]=p.p13;
        self.scalar_static_f64[136]=p.p14;
        self.scalar_static_f64[137]=p.p29;
        self.scalar_static_f64[138]=p.p93;
        self.scalar_static_f64[139]=p.p26;
        self.scalar_static_f64[140]=p.p92;
        self.scalar_static_f64[141]=p.p28;
        self.scalar_static_f64[142]=p.p94;
        self.scalar_static_f64[143]=p.p104;
        self.scalar_static_f64[144]=p.p105;
        self.scalar_static_f64[145]=p.p106;
        self.scalar_static_f64[146]=p.p103;
        self.scalar_static_bool[12]=(0.0!=self.scalar_static_f64[146]);
        self.scalar_static_f64[147]=p.p111;
        self.scalar_static_bool[13]=(self.scalar_static_f64[143]>=self.scalar_static_f64[147]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[12]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[148]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[149]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[39]}else{self.scalar_static_f64[114]});
        self.scalar_static_f64[150]=(self.scalar_static_f64[149]).exp();
        self.scalar_static_f64[151]=(-self.scalar_static_f64[149]);
        self.scalar_static_f64[152]=(self.scalar_static_f64[151]).exp();
        self.scalar_static_f64[153]=(self.scalar_static_f64[150]-self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=(self.scalar_static_f64[153]).ln();
        self.scalar_static_f64[155]=(self.scalar_static_f64[40]*self.scalar_static_f64[154]);
        self.scalar_static_f64[156]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[155]}else{self.scalar_static_f64[120]});
        self.scalar_static_f64[157]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[51]}else{self.scalar_static_f64[149]});
        self.scalar_static_f64[158]=(self.scalar_static_f64[157]).exp();
        self.scalar_static_f64[159]=(-self.scalar_static_f64[157]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[159]).exp();
        self.scalar_static_f64[161]=(self.scalar_static_f64[158]-self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=(self.scalar_static_f64[161]).ln();
        self.scalar_static_f64[163]=(self.scalar_static_f64[40]*self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[163]}else{self.scalar_static_f64[156]});
        self.scalar_static_f64[165]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[66]}else{self.scalar_static_f64[157]});
        self.scalar_static_f64[166]=(self.scalar_static_f64[165]).exp();
        self.scalar_static_f64[167]=(-self.scalar_static_f64[165]);
        self.scalar_static_f64[168]=(self.scalar_static_f64[167]).exp();
        self.scalar_static_f64[169]=(self.scalar_static_f64[166]-self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[169]).ln();
        self.scalar_static_f64[171]=(self.scalar_static_f64[40]*self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[171]}else{self.scalar_static_f64[164]});
        self.scalar_static_bool[15]=((self.scalar_static_f64[85]!=0.0)&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[16]=(self.scalar_static_bool[5]&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[17]=((self.scalar_static_f64[91]!=0.0)&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[18]=(self.scalar_static_bool[7]&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[19]=((self.scalar_static_f64[95]!=0.0)&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[20]=(self.scalar_static_bool[9]&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_f64[173]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[103]}else{self.scalar_static_f64[165]});
        self.scalar_static_f64[174]=(self.scalar_static_f64[173]).exp();
        self.scalar_static_f64[175]=(-self.scalar_static_f64[173]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[175]).exp();
        self.scalar_static_f64[177]=(self.scalar_static_f64[174]-self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[177]).ln();
        self.scalar_static_f64[179]=(self.scalar_static_f64[40]*self.scalar_static_f64[178]);
        self.scalar_static_f64[180]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[179]}else{self.scalar_static_f64[172]});
        self.scalar_static_f64[181]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[114]}else{self.scalar_static_f64[173]});
        self.scalar_static_f64[182]=(self.scalar_static_f64[181]).exp();
        self.scalar_static_f64[183]=(-self.scalar_static_f64[181]);
        self.scalar_static_f64[184]=(self.scalar_static_f64[183]).exp();
        self.scalar_static_f64[185]=(self.scalar_static_f64[182]-self.scalar_static_f64[184]);
        self.scalar_static_f64[186]=(self.scalar_static_f64[185]).ln();
        self.scalar_static_f64[187]=(self.scalar_static_f64[40]*self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[187]}else{self.scalar_static_f64[180]});
        self.scalar_static_bool[21]=((self.scalar_static_f64[130]!=0.0)&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_bool[22]=(self.scalar_static_bool[11]&&(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_f64[189]=p.p49;
        self.scalar_static_f64[190]=(1.0-self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=p.p44;
        self.scalar_static_bool[23]=(self.scalar_static_f64[191]<100.0);
        self.scalar_static_f64[192]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_f64[193]=(self.scalar_static_f64[73]/4.0);
        self.scalar_static_f64[194]=(-0.8754687373538999/self.scalar_static_f64[73]);
        self.scalar_static_f64[195]=(self.scalar_static_f64[194]).exp();
        self.scalar_static_f64[196]=(1.0-self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=(1.0-self.scalar_static_f64[73]);
        self.scalar_static_f64[198]=(-self.scalar_static_f64[73]);
        self.scalar_static_bool[24]=(!(self.scalar_static_f64[192]!=0.0));
        self.scalar_static_f64[199]=p.p48;
        self.scalar_static_bool[25]=(self.scalar_static_f64[199]<100.0);
        self.scalar_static_f64[200]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_f64[201]=(self.scalar_static_f64[111]/4.0);
        self.scalar_static_f64[202]=(-0.8754687373538999/self.scalar_static_f64[111]);
        self.scalar_static_f64[203]=(self.scalar_static_f64[202]).exp();
        self.scalar_static_f64[204]=(1.0-self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(1.0-self.scalar_static_f64[111]);
        self.scalar_static_f64[206]=(-self.scalar_static_f64[111]);
        self.scalar_static_bool[26]=(!(self.scalar_static_f64[200]!=0.0));
        self.scalar_static_f64[207]=p.p67;
        self.scalar_static_f64[208]=p.p63;
        self.scalar_static_f64[209]=p.p66;
        self.scalar_static_f64[210]=(-self.scalar_static_f64[47]);
        self.scalar_static_f64[211]=(1.0-self.scalar_static_f64[47]);
        self.scalar_static_f64[212]=(-self.scalar_static_f64[58]);
        self.scalar_static_f64[213]=(1.0-self.scalar_static_f64[58]);
        self.scalar_static_f64[214]=(if (self.scalar_static_f64[130]!=0.0){self.scalar_static_f64[58]}else{0.0});
        self.scalar_static_f64[215]=(if self.scalar_static_bool[11]{self.scalar_static_f64[47]}else{self.scalar_static_f64[214]});
        self.scalar_static_bool[27]=(0.0==self.scalar_static_f64[125]);
        self.scalar_static_f64[216]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_f64[217]=(if (self.scalar_static_f64[216]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[28]=(!(self.scalar_static_f64[216]!=0.0));
        self.scalar_static_f64[218]=p.p8;
        self.scalar_static_f64[219]=p.p5;
        self.scalar_static_f64[220]=p.p55;
        self.scalar_static_f64[221]=p.p56;
        self.scalar_static_f64[222]=p.p10;
        self.scalar_static_bool[29]=(1.0==self.scalar_static_f64[222]);
        self.scalar_static_f64[223]=(if self.scalar_static_bool[29]{1.0}else{0.0});
        self.scalar_static_bool[30]=(!(self.scalar_static_f64[223]!=0.0));
        self.scalar_static_f64[224]=p.p11;
        self.scalar_static_f64[225]=p.p3;
        self.scalar_static_f64[226]=p.p4;
        self.scalar_static_bool[31]=(0.0!=self.scalar_static_f64[135]);
        self.scalar_static_f64[227]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[227]!=0.0));
        self.scalar_static_f64[228]=(1.0+self.scalar_static_f64[136]);
        self.scalar_static_f64[229]=p.p2;
        self.scalar_static_bool[33]=(0.0==self.scalar_static_f64[229]);
        self.scalar_static_f64[230]=(if self.scalar_static_bool[33]{1.0}else{0.0});
        self.scalar_static_bool[34]=((self.scalar_static_f64[227]!=0.0)&&(self.scalar_static_f64[230]!=0.0));
        self.scalar_static_bool[35]=(self.scalar_static_bool[32]&&(self.scalar_static_f64[230]!=0.0));
        self.scalar_static_bool[36]=(!(self.scalar_static_f64[230]!=0.0));
        self.scalar_static_f64[231]=(if self.scalar_static_bool[36]{0.3333333333333333}else{0.0});
        self.scalar_static_bool[37]=(self.scalar_static_f64[76]==1000000.0);
        self.scalar_static_bool[38]=(self.scalar_static_f64[134]==1000000.0);
        self.scalar_static_bool[39]=(self.scalar_static_bool[37]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[232]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_bool[40]=(!(self.scalar_static_f64[232]!=0.0));
        self.scalar_static_bool[41]=(self.scalar_static_bool[36]&&self.scalar_static_bool[40]);
        self.scalar_static_f64[233]=p.p60;
        self.scalar_static_f64[234]=(1.0+self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=(self.scalar_static_f64[234]).sqrt();
        self.scalar_static_f64[236]=(1.0+self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=p.p58;
        self.scalar_static_f64[238]=(1.0+self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=p.p68;
        self.scalar_static_bool[42]=(self.scalar_static_f64[60]>0.0);
        self.scalar_static_f64[240]=(if self.scalar_static_bool[42]{1.0}else{0.0});
        self.scalar_static_f64[241]=p.p16;
        self.scalar_static_bool[43]=(!(self.scalar_static_f64[240]!=0.0));
        self.scalar_static_bool[44]=(self.scalar_static_f64[61]>0.0);
        self.scalar_static_f64[242]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_f64[243]=p.p18;
        self.scalar_static_bool[45]=(!(self.scalar_static_f64[242]!=0.0));
        self.scalar_static_bool[46]=(self.scalar_static_f64[74]>0.0);
        self.scalar_static_f64[244]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_f64[245]=p.p20;
        self.scalar_static_bool[47]=(!(self.scalar_static_f64[244]!=0.0));
        self.scalar_static_f64[246]=p.p24;
        self.scalar_static_f64[247]=p.p25;
        self.scalar_static_f64[248]=p.p27;
        self.scalar_static_bool[48]=(self.scalar_static_f64[124]>0.0);
        self.scalar_static_f64[249]=(if self.scalar_static_bool[48]{1.0}else{0.0});
        self.scalar_static_f64[250]=p.p31;
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[249]!=0.0));
        self.scalar_static_bool[50]=(self.scalar_static_f64[123]>0.0);
        self.scalar_static_f64[251]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[252]=p.p33;
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[251]!=0.0));
        self.scalar_static_f64[253]=p.p53;
        self.scalar_static_bool[52]=(self.scalar_static_f64[253]<100.0);
        self.scalar_static_f64[254]=(if self.scalar_static_bool[52]{1.0}else{0.0});
        self.scalar_static_f64[255]=(self.scalar_static_f64[122]/4.0);
        self.scalar_static_f64[256]=(-0.8754687373538999/self.scalar_static_f64[122]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[256]).exp();
        self.scalar_static_f64[258]=(1.0-self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(1.0-self.scalar_static_f64[122]);
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[254]!=0.0));
        self.scalar_static_bool[54]=(1.0==self.scalar_static_f64[146]);
        self.scalar_static_bool[55]=(self.scalar_static_bool[13]&&self.scalar_static_bool[54]);
        self.scalar_static_f64[260]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_f64[261]=p.p73;
        self.scalar_static_bool[56]=(0.0!=self.scalar_static_f64[261]);
        self.scalar_static_bool[57]=(0.0!=self.scalar_static_f64[87]);
        self.scalar_static_bool[58]=(self.scalar_static_bool[56]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[262]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_f64[263]=p.p71;
        self.scalar_static_f64[264]=p.p72;
        self.scalar_static_bool[59]=(!(self.scalar_static_f64[262]!=0.0));
        self.scalar_static_f64[265]=p.p70;
        self.scalar_static_f64[266]=p.p69;
        self.scalar_static_bool[60]=(self.scalar_static_f64[141]>=self.scalar_static_f64[147]);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=(self.scalar_static_f64[137]>=self.scalar_static_f64[147]);
        self.scalar_static_f64[268]=(if self.scalar_static_bool[61]{1.0}else{0.0});
        self.scalar_static_bool[62]=(self.scalar_static_f64[99]>=self.scalar_static_f64[147]);
        self.scalar_static_bool[63]=(self.scalar_static_f64[139]>=self.scalar_static_f64[147]);
        self.scalar_static_bool[64]=(self.scalar_static_bool[62]||self.scalar_static_bool[63]);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[64]{1.0}else{0.0});
        self.scalar_static_bool[65]=(0.0==self.scalar_static_f64[146]);
        self.scalar_static_f64[270]=p.p107;
        self.scalar_static_bool[66]=(0.0==self.scalar_static_f64[270]);
        self.scalar_static_bool[67]=(self.scalar_static_bool[65]||self.scalar_static_bool[66]);
        self.scalar_static_f64[271]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_bool[68]=(!(self.scalar_static_f64[271]!=0.0));
        self.scalar_static_bool[69]=(self.scalar_static_f64[143]<self.scalar_static_f64[147]);
        self.scalar_static_bool[70]=(self.scalar_static_bool[65]||self.scalar_static_bool[69]);
        self.scalar_static_f64[272]=(if self.scalar_static_bool[70]{1.0}else{0.0});
        self.scalar_static_bool[71]=(!(self.scalar_static_f64[272]!=0.0));
        self.scalar_static_f64[273]=(-self.scalar_static_f64[0]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[0]-self.scalar_static_f64[0]);
        self.scalar_static_f64[275]=(if (self.scalar_static_f64[148]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[276]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[277]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[278]=(if self.scalar_static_bool[5]{self.scalar_static_f64[0]}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[279]=(if self.scalar_static_bool[5]{self.scalar_static_f64[274]}else{self.scalar_static_f64[277]});
        self.scalar_static_f64[280]=(if self.scalar_static_bool[5]{self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[281]=(if (self.scalar_static_f64[95]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[282]=(if (self.scalar_static_f64[95]!=0.0){self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[283]=(if (self.scalar_static_f64[262]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[284]=(if (self.scalar_static_f64[262]!=0.0){self.scalar_static_f64[283]}else{0.0});
        self.scalar_static_f64[285]=(self.scalar_static_f64[263]*self.scalar_static_f64[283]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[87]*self.scalar_static_f64[285]);
        self.scalar_static_f64[287]=(if (self.scalar_static_f64[262]!=0.0){self.scalar_static_f64[286]}else{0.0});
        self.scalar_static_f64[288]=(self.scalar_static_f64[264]*self.scalar_static_f64[283]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[87]*self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(if (self.scalar_static_f64[262]!=0.0){self.scalar_static_f64[289]}else{0.0});
        self.scalar_static_f64[291]=(if self.scalar_static_bool[59]{1.0}else{self.scalar_static_f64[284]});
        self.scalar_static_f64[292]=(if self.scalar_static_bool[59]{0.0}else{self.scalar_static_f64[287]});
        self.scalar_static_f64[293]=(if self.scalar_static_bool[59]{0.0}else{self.scalar_static_f64[290]});
        self.scalar_static_f64[294]=(self.scalar_static_f64[0]*self.scalar_static_f64[265]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[265]*self.scalar_static_f64[273]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[0]*self.scalar_static_f64[266]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[266]*self.scalar_static_f64[273]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[0]*self.scalar_static_f64[294]);
        self.scalar_static_f64[299]=(self.scalar_static_f64[0]*self.scalar_static_f64[295]);
        self.scalar_static_f64[300]=(self.scalar_static_f64[0]*self.scalar_static_f64[296]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[0]*self.scalar_static_f64[297]);
        self.scalar_static_f64[302]=(self.scalar_static_f64[0]*self.scalar_static_f64[283]);
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
        self.scalar_static_f64[303]=(temperature+self.scalar_static_f64[36]);
        self.scalar_static_bool[72]=(self.scalar_static_f64[303]<173.14999999999998);
        self.scalar_static_f64[304]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_f64[305]=(if (self.scalar_static_f64[304]!=0.0){173.14999999999998}else{self.scalar_static_f64[303]});
        self.scalar_static_bool[73]=(self.scalar_static_f64[305]>600.0);
        self.scalar_static_f64[306]=(if self.scalar_static_bool[73]{1.0}else{0.0});
        self.scalar_static_bool[74]=(!(self.scalar_static_f64[304]!=0.0));
        self.scalar_static_bool[75]=((self.scalar_static_f64[306]!=0.0)&&self.scalar_static_bool[74]);
        self.scalar_static_f64[307]=(if self.scalar_static_bool[75]{600.0}else{self.scalar_static_f64[305]});
        self.scalar_static_f64[308]=(1.3806226e-23*self.scalar_static_f64[307]);
        self.scalar_static_f64[309]=(self.scalar_static_f64[308]/1.602176462e-19);
        self.scalar_static_f64[310]=(1.0/self.scalar_static_f64[309]);
        self.scalar_static_f64[311]=(self.scalar_static_f64[307]-self.scalar_static_f64[2]);
        self.scalar_static_f64[312]=(self.scalar_static_f64[307]/self.scalar_static_f64[2]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[312]).ln();
        self.scalar_static_f64[314]=(self.scalar_static_f64[312]-1.0);
        self.scalar_static_f64[315]=(self.scalar_static_f64[310]*self.scalar_static_f64[314]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[312]*self.scalar_static_f64[46]);
        self.scalar_static_f64[317]=(1.0-self.scalar_static_f64[312]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[10]*self.scalar_static_f64[317]);
        self.scalar_static_f64[319]=(self.scalar_static_f64[316]+self.scalar_static_f64[318]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[20]*self.scalar_static_f64[309]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[313]*self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[319]-self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[309]*2.0);
        self.scalar_static_f64[324]=(-self.scalar_static_f64[322]);
        self.scalar_static_f64[325]=(self.scalar_static_f64[310]*self.scalar_static_f64[324]);
        self.scalar_static_f64[326]=(self.scalar_static_f64[325]).exp();
        self.scalar_static_f64[327]=(4.0*self.scalar_static_f64[326]);
        self.scalar_static_f64[328]=(1.0+self.scalar_static_f64[327]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[328]).sqrt();
        self.scalar_static_f64[330]=(1.0+self.scalar_static_f64[329]);
        self.scalar_static_f64[331]=(0.5*self.scalar_static_f64[330]);
        self.scalar_static_f64[332]=(self.scalar_static_f64[331]).ln();
        self.scalar_static_f64[333]=(self.scalar_static_f64[323]*self.scalar_static_f64[332]);
        self.scalar_static_f64[334]=(self.scalar_static_f64[322]+self.scalar_static_f64[333]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[37]/self.scalar_static_f64[334]);
        self.scalar_static_f64[336]=(self.scalar_static_f64[335]).ln();
        self.scalar_static_f64[337]=(self.scalar_static_f64[47]*self.scalar_static_f64[336]);
        self.scalar_static_f64[338]=(self.scalar_static_f64[337]).exp();
        self.scalar_static_f64[339]=(self.scalar_static_f64[30]*self.scalar_static_f64[338]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[334]*self.scalar_static_f64[48]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[340]/self.scalar_static_f64[37]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[312]*self.scalar_static_f64[57]);
        self.scalar_static_f64[343]=(self.scalar_static_f64[318]+self.scalar_static_f64[342]);
        self.scalar_static_f64[344]=(self.scalar_static_f64[343]-self.scalar_static_f64[321]);
        self.scalar_static_f64[345]=(-self.scalar_static_f64[344]);
        self.scalar_static_f64[346]=(self.scalar_static_f64[310]*self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=(self.scalar_static_f64[346]).exp();
        self.scalar_static_f64[348]=(4.0*self.scalar_static_f64[347]);
        self.scalar_static_f64[349]=(1.0+self.scalar_static_f64[348]);
        self.scalar_static_f64[350]=(self.scalar_static_f64[349]).sqrt();
        self.scalar_static_f64[351]=(1.0+self.scalar_static_f64[350]);
        self.scalar_static_f64[352]=(0.5*self.scalar_static_f64[351]);
        self.scalar_static_f64[353]=(self.scalar_static_f64[352]).ln();
        self.scalar_static_f64[354]=(self.scalar_static_f64[323]*self.scalar_static_f64[353]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[344]+self.scalar_static_f64[354]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[49]/self.scalar_static_f64[355]);
        self.scalar_static_f64[357]=(self.scalar_static_f64[356]).ln();
        self.scalar_static_f64[358]=(self.scalar_static_f64[58]*self.scalar_static_f64[357]);
        self.scalar_static_f64[359]=(self.scalar_static_f64[358]).exp();
        self.scalar_static_f64[360]=(self.scalar_static_f64[30]*self.scalar_static_f64[359]);
        self.scalar_static_f64[361]=(self.scalar_static_f64[355]*self.scalar_static_f64[59]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[361]/self.scalar_static_f64[49]);
        self.scalar_static_f64[363]=(self.scalar_static_f64[25]*self.scalar_static_f64[313]);
        self.scalar_static_f64[364]=(self.scalar_static_f64[8]*self.scalar_static_f64[315]);
        self.scalar_static_f64[365]=(self.scalar_static_f64[363]+self.scalar_static_f64[364]);
        self.scalar_static_f64[366]=(self.scalar_static_f64[365]).exp();
        self.scalar_static_f64[367]=(self.scalar_static_f64[60]*self.scalar_static_f64[366]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[313]*self.scalar_static_f64[62]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[315]*self.scalar_static_f64[63]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[368]+self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[370]).exp();
        self.scalar_static_f64[372]=(self.scalar_static_f64[61]*self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=(self.scalar_static_f64[312]*self.scalar_static_f64[72]);
        self.scalar_static_f64[374]=(self.scalar_static_f64[13]*self.scalar_static_f64[317]);
        self.scalar_static_f64[375]=(self.scalar_static_f64[373]+self.scalar_static_f64[374]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[375]-self.scalar_static_f64[321]);
        self.scalar_static_f64[377]=(-self.scalar_static_f64[376]);
        self.scalar_static_f64[378]=(self.scalar_static_f64[310]*self.scalar_static_f64[377]);
        self.scalar_static_f64[379]=(self.scalar_static_f64[378]).exp();
        self.scalar_static_f64[380]=(4.0*self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=(1.0+self.scalar_static_f64[380]);
        self.scalar_static_f64[382]=(self.scalar_static_f64[381]).sqrt();
        self.scalar_static_f64[383]=(1.0+self.scalar_static_f64[382]);
        self.scalar_static_f64[384]=(0.5*self.scalar_static_f64[383]);
        self.scalar_static_f64[385]=(self.scalar_static_f64[384]).ln();
        self.scalar_static_f64[386]=(self.scalar_static_f64[323]*self.scalar_static_f64[385]);
        self.scalar_static_f64[387]=(self.scalar_static_f64[376]+self.scalar_static_f64[386]);
        self.scalar_static_f64[388]=(self.scalar_static_f64[64]/self.scalar_static_f64[387]);
        self.scalar_static_f64[389]=(self.scalar_static_f64[388]).ln();
        self.scalar_static_f64[390]=(self.scalar_static_f64[73]*self.scalar_static_f64[389]);
        self.scalar_static_f64[391]=(self.scalar_static_f64[390]).exp();
        self.scalar_static_f64[392]=(self.scalar_static_f64[32]*self.scalar_static_f64[391]);
        self.scalar_static_f64[393]=(self.scalar_static_f64[23]*self.scalar_static_f64[313]);
        self.scalar_static_f64[394]=(self.scalar_static_f64[11]*self.scalar_static_f64[315]);
        self.scalar_static_f64[395]=(self.scalar_static_f64[393]+self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=(self.scalar_static_f64[395]).exp();
        self.scalar_static_f64[397]=(self.scalar_static_f64[74]*self.scalar_static_f64[396]);
        self.scalar_static_f64[398]=(self.scalar_static_f64[26]*self.scalar_static_f64[313]);
        self.scalar_static_f64[399]=(self.scalar_static_f64[7]*self.scalar_static_f64[315]);
        self.scalar_static_f64[400]=(self.scalar_static_f64[398]+self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=(self.scalar_static_f64[400]).exp();
        self.scalar_static_f64[402]=(self.scalar_static_f64[75]*self.scalar_static_f64[401]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[313]*self.scalar_static_f64[77]);
        self.scalar_static_f64[404]=(self.scalar_static_f64[315]*self.scalar_static_f64[78]);
        self.scalar_static_f64[405]=(self.scalar_static_f64[403]-self.scalar_static_f64[404]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[405]).exp();
        self.scalar_static_f64[407]=(self.scalar_static_f64[76]*self.scalar_static_f64[406]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[313]*self.scalar_static_f64[80]);
        self.scalar_static_f64[409]=(self.scalar_static_f64[408]).exp();
        self.scalar_static_f64[410]=(self.scalar_static_f64[79]*self.scalar_static_f64[409]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[22]*self.scalar_static_f64[313]);
        self.scalar_static_f64[412]=(self.scalar_static_f64[411]).exp();
        self.scalar_static_f64[413]=(self.scalar_static_f64[81]*self.scalar_static_f64[412]);
        self.scalar_static_f64[414]=(1.0/self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[311]*self.scalar_static_f64[83]);
        self.scalar_static_f64[416]=(1.0+self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=(self.scalar_static_f64[82]*self.scalar_static_f64[416]);
        self.scalar_static_f64[418]=(self.scalar_static_f64[311]*self.scalar_static_f64[86]);
        self.scalar_static_f64[419]=(1.0-self.scalar_static_f64[418]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[84]*self.scalar_static_f64[419]);
        self.scalar_static_f64[421]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[420]}else{0.0});
        self.scalar_static_f64[422]=(if (self.scalar_static_f64[85]!=0.0){self.scalar_static_f64[82]}else{self.scalar_static_f64[417]});
        self.scalar_static_f64[423]=(if self.scalar_static_bool[5]{self.scalar_static_f64[417]}else{self.scalar_static_f64[422]});
        self.scalar_static_f64[424]=(if self.scalar_static_bool[5]{self.scalar_static_f64[84]}else{self.scalar_static_f64[421]});
        self.scalar_static_f64[425]=(self.scalar_static_f64[311]*self.scalar_static_f64[88]);
        self.scalar_static_f64[426]=(1.0+self.scalar_static_f64[425]);
        self.scalar_static_f64[427]=(self.scalar_static_f64[311]*self.scalar_static_f64[89]);
        self.scalar_static_f64[428]=(self.scalar_static_f64[311]*self.scalar_static_f64[427]);
        self.scalar_static_f64[429]=(self.scalar_static_f64[426]+self.scalar_static_f64[428]);
        self.scalar_static_f64[430]=(self.scalar_static_f64[87]*self.scalar_static_f64[429]);
        self.scalar_static_f64[431]=(self.scalar_static_f64[28]*self.scalar_static_f64[313]);
        self.scalar_static_f64[432]=(self.scalar_static_f64[29]*self.scalar_static_f64[315]);
        self.scalar_static_f64[433]=(self.scalar_static_f64[431]-self.scalar_static_f64[432]);
        self.scalar_static_f64[434]=(self.scalar_static_f64[433]).exp();
        self.scalar_static_f64[435]=(self.scalar_static_f64[92]*self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=(if (self.scalar_static_f64[91]!=0.0){self.scalar_static_f64[435]}else{0.0});
        self.scalar_static_f64[437]=(if self.scalar_static_bool[7]{self.scalar_static_f64[92]}else{self.scalar_static_f64[436]});
        self.scalar_static_f64[438]=(self.scalar_static_f64[313]*self.scalar_static_f64[94]);
        self.scalar_static_f64[439]=(self.scalar_static_f64[438]).exp();
        self.scalar_static_f64[440]=(self.scalar_static_f64[93]*self.scalar_static_f64[439]);
        self.scalar_static_f64[441]=(self.scalar_static_f64[311]*self.scalar_static_f64[96]);
        self.scalar_static_f64[442]=(self.scalar_static_f64[441]).exp();
        self.scalar_static_f64[443]=(self.scalar_static_f64[31]*self.scalar_static_f64[442]);
        self.scalar_static_f64[444]=(if (self.scalar_static_f64[95]!=0.0){self.scalar_static_f64[443]}else{0.0});
        self.scalar_static_f64[445]=(self.scalar_static_f64[311]*self.scalar_static_f64[98]);
        self.scalar_static_f64[446]=(self.scalar_static_f64[445]).exp();
        self.scalar_static_f64[447]=(self.scalar_static_f64[97]*self.scalar_static_f64[446]);
        self.scalar_static_f64[448]=(if (self.scalar_static_f64[95]!=0.0){self.scalar_static_f64[447]}else{0.0});
        self.scalar_static_f64[449]=(if self.scalar_static_bool[9]{self.scalar_static_f64[31]}else{self.scalar_static_f64[444]});
        self.scalar_static_f64[450]=(if self.scalar_static_bool[9]{self.scalar_static_f64[97]}else{self.scalar_static_f64[448]});
        self.scalar_static_f64[451]=(self.scalar_static_f64[313]*self.scalar_static_f64[100]);
        self.scalar_static_f64[452]=(self.scalar_static_f64[451]).exp();
        self.scalar_static_f64[453]=(self.scalar_static_f64[99]*self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(self.scalar_static_f64[312]*self.scalar_static_f64[109]);
        self.scalar_static_f64[455]=(self.scalar_static_f64[374]+self.scalar_static_f64[454]);
        self.scalar_static_f64[456]=(self.scalar_static_f64[455]-self.scalar_static_f64[321]);
        self.scalar_static_f64[457]=(-self.scalar_static_f64[456]);
        self.scalar_static_f64[458]=(self.scalar_static_f64[310]*self.scalar_static_f64[457]);
        self.scalar_static_f64[459]=(self.scalar_static_f64[458]).exp();
        self.scalar_static_f64[460]=(4.0*self.scalar_static_f64[459]);
        self.scalar_static_f64[461]=(1.0+self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(self.scalar_static_f64[461]).sqrt();
        self.scalar_static_f64[463]=(1.0+self.scalar_static_f64[462]);
        self.scalar_static_f64[464]=(0.5*self.scalar_static_f64[463]);
        self.scalar_static_f64[465]=(self.scalar_static_f64[464]).ln();
        self.scalar_static_f64[466]=(self.scalar_static_f64[323]*self.scalar_static_f64[465]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[456]+self.scalar_static_f64[466]);
        self.scalar_static_f64[468]=(self.scalar_static_f64[101]/self.scalar_static_f64[467]);
        self.scalar_static_f64[469]=(self.scalar_static_f64[468]).ln();
        self.scalar_static_f64[470]=(self.scalar_static_f64[111]*self.scalar_static_f64[469]);
        self.scalar_static_f64[471]=(self.scalar_static_f64[470]).exp();
        self.scalar_static_f64[472]=(self.scalar_static_f64[110]*self.scalar_static_f64[471]);
        self.scalar_static_f64[473]=(self.scalar_static_f64[312]*self.scalar_static_f64[120]);
        self.scalar_static_f64[474]=(self.scalar_static_f64[16]*self.scalar_static_f64[317]);
        self.scalar_static_f64[475]=(self.scalar_static_f64[473]+self.scalar_static_f64[474]);
        self.scalar_static_f64[476]=(self.scalar_static_f64[475]-self.scalar_static_f64[321]);
        self.scalar_static_f64[477]=(-self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(self.scalar_static_f64[310]*self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[478]).exp();
        self.scalar_static_f64[480]=(4.0*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(1.0+self.scalar_static_f64[480]);
        self.scalar_static_f64[482]=(self.scalar_static_f64[481]).sqrt();
        self.scalar_static_f64[483]=(1.0+self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=(0.5*self.scalar_static_f64[483]);
        self.scalar_static_f64[485]=(self.scalar_static_f64[484]).ln();
        self.scalar_static_f64[486]=(self.scalar_static_f64[323]*self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=(self.scalar_static_f64[476]+self.scalar_static_f64[486]);
        self.scalar_static_f64[488]=(self.scalar_static_f64[112]/self.scalar_static_f64[487]);
        self.scalar_static_f64[489]=(self.scalar_static_f64[488]).ln();
        self.scalar_static_f64[490]=(self.scalar_static_f64[122]*self.scalar_static_f64[489]);
        self.scalar_static_f64[491]=(self.scalar_static_f64[490]).exp();
        self.scalar_static_f64[492]=(self.scalar_static_f64[121]*self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=(self.scalar_static_f64[24]*self.scalar_static_f64[313]);
        self.scalar_static_f64[494]=(self.scalar_static_f64[14]*self.scalar_static_f64[315]);
        self.scalar_static_f64[495]=(self.scalar_static_f64[493]+self.scalar_static_f64[494]);
        self.scalar_static_f64[496]=(self.scalar_static_f64[495]).exp();
        self.scalar_static_f64[497]=(self.scalar_static_f64[123]*self.scalar_static_f64[496]);
        self.scalar_static_f64[498]=(self.scalar_static_f64[394]+self.scalar_static_f64[493]);
        self.scalar_static_f64[499]=(self.scalar_static_f64[498]).exp();
        self.scalar_static_f64[500]=(self.scalar_static_f64[124]*self.scalar_static_f64[499]);
        self.scalar_static_f64[501]=(self.scalar_static_f64[313]*self.scalar_static_f64[126]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[501]).exp();
        self.scalar_static_f64[503]=(self.scalar_static_f64[125]*self.scalar_static_f64[502]);
        self.scalar_static_f64[504]=(self.scalar_static_f64[310]*self.scalar_static_f64[78]);
        self.scalar_static_f64[505]=(self.scalar_static_f64[313]*self.scalar_static_f64[128]);
        self.scalar_static_f64[506]=(self.scalar_static_f64[505]).exp();
        self.scalar_static_f64[507]=(self.scalar_static_f64[506]-1.0);
        self.scalar_static_f64[508]=(self.scalar_static_f64[504]*self.scalar_static_f64[507]);
        self.scalar_static_f64[509]=(self.scalar_static_f64[508]).exp();
        self.scalar_static_f64[510]=(self.scalar_static_f64[127]/self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[311]*self.scalar_static_f64[132]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[131]+self.scalar_static_f64[511]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[311]*self.scalar_static_f64[512]);
        self.scalar_static_f64[514]=(1.0+self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(if (self.scalar_static_f64[130]!=0.0){self.scalar_static_f64[514]}else{0.0});
        self.scalar_static_f64[516]=(self.scalar_static_f64[313]*self.scalar_static_f64[133]);
        self.scalar_static_f64[517]=(self.scalar_static_f64[516]).exp();
        self.scalar_static_f64[518]=(if self.scalar_static_bool[11]{self.scalar_static_f64[517]}else{self.scalar_static_f64[515]});
        self.scalar_static_f64[519]=(self.scalar_static_f64[518]*self.scalar_static_f64[134]);
        self.scalar_static_f64[520]=(self.scalar_static_f64[518]*self.scalar_static_f64[135]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[432]).exp();
        self.scalar_static_f64[522]=(self.scalar_static_f64[520]*self.scalar_static_f64[521]);
        self.scalar_static_f64[523]=(self.scalar_static_f64[313]*self.scalar_static_f64[138]);
        self.scalar_static_f64[524]=(self.scalar_static_f64[523]).exp();
        self.scalar_static_f64[525]=(self.scalar_static_f64[137]*self.scalar_static_f64[524]);
        self.scalar_static_f64[526]=(self.scalar_static_f64[313]*self.scalar_static_f64[140]);
        self.scalar_static_f64[527]=(self.scalar_static_f64[526]).exp();
        self.scalar_static_f64[528]=(self.scalar_static_f64[139]*self.scalar_static_f64[527]);
        self.scalar_static_f64[529]=(self.scalar_static_f64[313]*self.scalar_static_f64[142]);
        self.scalar_static_f64[530]=(self.scalar_static_f64[529]).exp();
        self.scalar_static_f64[531]=(self.scalar_static_f64[141]*self.scalar_static_f64[530]);
        self.scalar_static_f64[532]=(self.scalar_static_f64[313]*self.scalar_static_f64[144]);
        self.scalar_static_f64[533]=(self.scalar_static_f64[532]).exp();
        self.scalar_static_f64[534]=(self.scalar_static_f64[143]*self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(self.scalar_static_f64[311]*self.scalar_static_f64[145]);
        self.scalar_static_f64[536]=(1.0+self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=(self.scalar_static_f64[534]*self.scalar_static_f64[536]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
