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
    pub p112: f64,
    pub p113: f64,
    pub p114: f64,
    pub p115: f64,
    pub p116: f64,
    pub p117: f64,
    pub p118: f64,
    pub p119: f64,
    pub p120: f64,
    pub p121: f64,
    pub p122: f64,
    pub p123: f64,
    pub p124: f64,
    pub p125: f64,
    pub p126: f64,
    pub p127: f64,
    pub p128: f64,
    pub p129: f64,
    pub p130: f64,
    pub p131: f64,
    pub p132: f64,
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
            params.p0 = 0.0;
            params.p1 = 1.0;
            params.p2 = 1.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = -100.0;
            params.p9 = 500.0;
            params.p10 = 1e-12;
            params.p11 = 1.0;
            params.p12 = 1e22;
            params.p13 = 27.0;
            params.p14 = -100.0;
            params.p15 = 500.0;
            params.p16 = 0.0;
            params.p17 = 0.0;
            params.p18 = 0.0;
            params.p19 = 0.0;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 1e-16;
            params.p27 = 1.0;
            params.p28 = 1.0;
            params.p29 = 1.0;
            params.p30 = 0.0;
            params.p31 = 0.0;
            params.p32 = 1.0;
            params.p33 = 1.0;
            params.p34 = 0.9;
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 0.75;
            params.p38 = 0.33;
            params.p39 = -0.5;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.75;
            params.p43 = 0.33;
            params.p44 = -0.5;
            params.p45 = 0.0;
            params.p46 = 0.1;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 0.75;
            params.p51 = 0.33;
            params.p52 = -0.5;
            params.p53 = 0.0;
            params.p54 = 1e-18;
            params.p55 = 1.0;
            params.p56 = 1.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 2.0;
            params.p60 = 1e-16;
            params.p61 = 1.0;
            params.p62 = 0.0;
            params.p63 = 2.0;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.0;
            params.p67 = 1.0;
            params.p68 = 0.0;
            params.p69 = 2.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.5;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.0;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.33;
            params.p88 = 0.0;
            params.p89 = 1.0;
            params.p90 = 1e-6;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 1.0;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 1.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 0.0;
            params.p112 = 0.0;
            params.p113 = 1.12;
            params.p114 = 1.12;
            params.p115 = 1.12;
            params.p116 = 1.12;
            params.p117 = 1.12;
            params.p118 = 1.12;
            params.p119 = 1.12;
            params.p120 = 1.12;
            params.p121 = 0.0;
            params.p122 = 3.0;
            params.p123 = 3.0;
            params.p124 = 3.0;
            params.p125 = 0.0;
            params.p126 = 0.0;
            params.p127 = 0.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
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
    pub nodes: [usize; 14],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 133]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 11]>,
    pub(crate) ddt_state_previous: Box<[f64; 11]>,
    pub(crate) ddt_state_older: Box<[f64; 11]>,
    pub(crate) ddt_state_initialized: Box<[bool; 11]>,
    pub(crate) ddt_derivative_current: Box<[f64; 11]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 11]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 281]>,
    pub(crate) scalar_static_bool: Box<[bool; 78]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["dt", "cx", "ci", "bx", "bi", "ei", "bp", "si", "xf1", "xf2"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 133;
    pub const VARIABLE_COUNT: usize = 359;
    pub const DDT_STATE_COUNT: usize = 11;
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
            scalar_static_f64: boxed_zero_f64_array::<281>(),
            scalar_static_bool: boxed_zero_bool_array::<78>(),
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
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dta" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_noise" => { validate_parameter("sw_noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_et" => { validate_parameter("sw_et", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npn" => { validate_finite_parameter("npn", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnp" => { validate_finite_parameter("pnp", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gmin" => { validate_parameter("gmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnjmaxi" => { validate_parameter("pnjmaxi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "maxexp" => { validate_parameter("maxexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci" => { validate_parameter("rci", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vo" => { validate_parameter("vo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamm" => { validate_parameter("gamm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hrcf" => { validate_parameter("hrcf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi" => { validate_parameter("rbi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("rs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbp" => { validate_parameter("rbp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isrr" => { validate_parameter("isrr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nr" => { validate_parameter("nr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qbm" => { validate_parameter("qbm", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isp" => { validate_parameter("isp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsp" => { validate_parameter("wsp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfp" => { validate_parameter("nfp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "me" => { validate_parameter("me", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aje" => { validate_finite_parameter("aje", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajc" => { validate_finite_parameter("ajc", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vrt" => { validate_parameter("vrt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "art" => { validate_parameter("art", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qco" => { validate_parameter("qco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjep" => { validate_parameter("cjep", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcp" => { validate_parameter("cjcp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("ps", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ms" => { validate_parameter("ms", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajs" => { validate_finite_parameter("ajs", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccso" => { validate_parameter("ccso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibei" => { validate_parameter("ibei", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbe" => { validate_parameter("wbe", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nei" => { validate_parameter("nei", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qnibeir" => { validate_parameter("qnibeir", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iben" => { validate_parameter("iben", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nen" => { validate_finite_parameter("nen", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibci" => { validate_parameter("ibci", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nci" => { validate_parameter("nci", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcn" => { validate_parameter("ibcn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncn" => { validate_finite_parameter("ncn", value)?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeip" => { validate_parameter("ibeip", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibenp" => { validate_parameter("ibenp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcip" => { validate_parameter("ibcip", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncip" => { validate_parameter("ncip", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcnp" => { validate_parameter("ibcnp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncnp" => { validate_finite_parameter("ncnp", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikf" => { validate_parameter("ikf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nkf" => { validate_parameter("nkf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikr" => { validate_parameter("ikr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikp" => { validate_parameter("ikp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qtf" => { validate_parameter("qtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xtf" => { validate_parameter("xtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtf" => { validate_parameter("vtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itf" => { validate_parameter("itf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td" => { validate_parameter("td", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc1" => { validate_parameter("avc1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc2" => { validate_parameter("avc2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcx1" => { validate_parameter("avcx1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcx2" => { validate_parameter("avcx2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcx" => { validate_parameter("mcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbbe" => { validate_parameter("vbbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbbe" => { validate_parameter("nbbe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibbe" => { validate_parameter("ibbe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe1" => { validate_finite_parameter("tvbbe1", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe2" => { validate_finite_parameter("tvbbe2", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnbbe" => { validate_finite_parameter("tnbbe", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpte" => { validate_parameter("vpte", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibk0" => { validate_parameter("ibk0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abk" => { validate_parameter("abk", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bbk" => { validate_parameter("bbk", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xre" => { validate_finite_parameter("xre", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrb" => { validate_finite_parameter("xrb", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbi" => { validate_finite_parameter("xrbi", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbx" => { validate_finite_parameter("xrbx", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrc" => { validate_finite_parameter("xrc", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrci" => { validate_finite_parameter("xrci", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrcx" => { validate_finite_parameter("xrcx", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbp" => { validate_finite_parameter("xrbp", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrs" => { validate_finite_parameter("xrs", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xvo" => { validate_finite_parameter("xvo", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaie" => { validate_finite_parameter("eaie", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaic" => { validate_finite_parameter("eaic", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eais" => { validate_finite_parameter("eais", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eane" => { validate_finite_parameter("eane", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eanc" => { validate_finite_parameter("eanc", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eans" => { validate_finite_parameter("eans", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eap" => { validate_finite_parameter("eap", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dear" => { validate_finite_parameter("dear", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xis" => { validate_finite_parameter("xis", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xii" => { validate_finite_parameter("xii", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xin" => { validate_finite_parameter("xin", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xisr" => { validate_finite_parameter("xisr", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xikf" => { validate_finite_parameter("xikf", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tavc" => { validate_finite_parameter("tavc", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tavcx" => { validate_finite_parameter("tavcx", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnf" => { validate_finite_parameter("tnf", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvef" => { validate_finite_parameter("tcvef", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcver" => { validate_finite_parameter("tcver", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrth" => { validate_finite_parameter("tcrth", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic13_4t'", name)),
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
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=if param_given[10] { 1.0 } else { 0.0 };
        self.scalar_static_f64[1]=p.p10;
        self.scalar_static_bool[0]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[2]=if param_given[11] { 1.0 } else { 0.0 };
        self.scalar_static_f64[3]=p.p11;
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_f64[4]=if param_given[3] { 1.0 } else { 0.0 };
        self.scalar_static_f64[5]=if param_given[4] { 1.0 } else { 0.0 };
        self.scalar_static_bool[2]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[6]=if param_given[5] { 1.0 } else { 0.0 };
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[5]!=0.0));
        self.scalar_static_f64[7]=p.p5;
        self.scalar_static_bool[4]=(!(self.scalar_static_f64[6]!=0.0));
        self.scalar_static_f64[8]=p.p12;
        self.scalar_static_f64[9]=(self.scalar_static_f64[8]).ln();
        self.scalar_static_f64[10]=p.p74;
        self.scalar_static_bool[5]=(self.scalar_static_f64[10]>0.0);
        self.scalar_static_f64[11]=(1.0/self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[5]{self.scalar_static_f64[11]}else{0.0});
        self.scalar_static_f64[13]=p.p75;
        self.scalar_static_bool[6]=(self.scalar_static_f64[13]>0.0);
        self.scalar_static_f64[14]=(1.0/self.scalar_static_f64[13]);
        self.scalar_static_f64[15]=(if self.scalar_static_bool[6]{self.scalar_static_f64[14]}else{0.0});
        self.scalar_static_f64[16]=p.p20;
        self.scalar_static_bool[7]=(self.scalar_static_f64[16]>0.0);
        self.scalar_static_f64[17]=(1.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(if self.scalar_static_bool[7]{self.scalar_static_f64[17]}else{0.0});
        self.scalar_static_f64[19]=p.p79;
        self.scalar_static_bool[8]=(self.scalar_static_f64[19]>0.0);
        self.scalar_static_f64[20]=(1.0/self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[8]{self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[22]=p.p80;
        self.scalar_static_bool[9]=(self.scalar_static_f64[22]>0.0);
        self.scalar_static_f64[23]=(1.0/self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=(if self.scalar_static_bool[9]{self.scalar_static_f64[23]}else{0.0});
        self.scalar_static_f64[25]=(if self.scalar_static_bool[9]{0.0}else{1.0});
        self.scalar_static_f64[26]=p.p13;
        self.scalar_static_f64[27]=(273.15+self.scalar_static_f64[26]);
        self.scalar_static_f64[28]=p.p0;
        self.scalar_static_f64[29]=p.p14;
        self.scalar_static_f64[30]=(1.0+self.scalar_static_f64[29]);
        self.scalar_static_f64[31]=p.p15;
        self.scalar_static_f64[32]=(self.scalar_static_f64[31]-1.0);
        self.scalar_static_f64[33]=p.p26;
        self.scalar_static_f64[34]=p.p90;
        self.scalar_static_bool[10]=(self.scalar_static_f64[34]>0.0);
        self.scalar_static_f64[35]=p.p89;
        self.scalar_static_f64[36]=p.p88;
        self.scalar_static_f64[37]=(-self.scalar_static_f64[36]);
        self.scalar_static_bool[11]=(!self.scalar_static_bool[10]);
        self.scalar_static_f64[38]=p.p122;
        self.scalar_static_f64[39]=p.p28;
        self.scalar_static_f64[40]=(self.scalar_static_f64[38]/self.scalar_static_f64[39]);
        self.scalar_static_f64[41]=p.p113;
        self.scalar_static_f64[42]=(-self.scalar_static_f64[41]);
        self.scalar_static_f64[43]=p.p72;
        self.scalar_static_bool[12]=(self.scalar_static_f64[43]>0.0);
        self.scalar_static_f64[44]=(4.0/self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=p.p73;
        self.scalar_static_f64[46]=f64::powf(self.scalar_static_f64[44],self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=(1.0-self.scalar_static_f64[45]);
        self.scalar_static_f64[48]=(1.0/self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p27;
        self.scalar_static_f64[50]=p.p125;
        self.scalar_static_f64[51]=p.p29;
        self.scalar_static_f64[52]=(self.scalar_static_f64[50]/self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=p.p121;
        self.scalar_static_f64[54]=(-self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=(4.0/self.scalar_static_f64[10]);
        self.scalar_static_f64[56]=f64::powf(self.scalar_static_f64[55],self.scalar_static_f64[45]);
        self.scalar_static_f64[57]=p.p31;
        self.scalar_static_f64[58]=p.p33;
        self.scalar_static_f64[59]=(self.scalar_static_f64[38]/self.scalar_static_f64[58]);
        self.scalar_static_f64[60]=p.p120;
        self.scalar_static_f64[61]=(-self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=p.p54;
        self.scalar_static_f64[63]=p.p123;
        self.scalar_static_f64[64]=p.p56;
        self.scalar_static_f64[65]=(self.scalar_static_f64[63]/self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=p.p114;
        self.scalar_static_f64[67]=(-self.scalar_static_f64[66]);
        self.scalar_static_f64[68]=p.p58;
        self.scalar_static_f64[69]=p.p124;
        self.scalar_static_f64[70]=p.p59;
        self.scalar_static_f64[71]=(self.scalar_static_f64[69]/self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=p.p117;
        self.scalar_static_f64[73]=(-self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=p.p60;
        self.scalar_static_f64[75]=p.p61;
        self.scalar_static_f64[76]=(self.scalar_static_f64[63]/self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=p.p115;
        self.scalar_static_f64[78]=(-self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=p.p62;
        self.scalar_static_f64[80]=p.p63;
        self.scalar_static_f64[81]=(self.scalar_static_f64[69]/self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=p.p118;
        self.scalar_static_f64[83]=(-self.scalar_static_f64[82]);
        self.scalar_static_f64[84]=p.p64;
        self.scalar_static_f64[85]=p.p65;
        self.scalar_static_f64[86]=p.p66;
        self.scalar_static_f64[87]=p.p67;
        self.scalar_static_f64[88]=(self.scalar_static_f64[63]/self.scalar_static_f64[87]);
        self.scalar_static_f64[89]=p.p116;
        self.scalar_static_f64[90]=(-self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=p.p68;
        self.scalar_static_f64[92]=p.p69;
        self.scalar_static_f64[93]=(self.scalar_static_f64[69]/self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=p.p119;
        self.scalar_static_f64[95]=(-self.scalar_static_f64[94]);
        self.scalar_static_f64[96]=p.p126;
        self.scalar_static_f64[97]=if param_given[109] { 1.0 } else { 0.0 };
        self.scalar_static_f64[98]=p.p16;
        self.scalar_static_f64[99]=p.p109;
        self.scalar_static_bool[13]=(!(self.scalar_static_f64[97]!=0.0));
        self.scalar_static_f64[100]=p.p107;
        self.scalar_static_f64[101]=if param_given[108] { 1.0 } else { 0.0 };
        self.scalar_static_f64[102]=p.p17;
        self.scalar_static_f64[103]=p.p108;
        self.scalar_static_bool[14]=(!(self.scalar_static_f64[101]!=0.0));
        self.scalar_static_f64[104]=if param_given[106] { 1.0 } else { 0.0 };
        self.scalar_static_f64[105]=p.p21;
        self.scalar_static_f64[106]=p.p106;
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[104]!=0.0));
        self.scalar_static_f64[107]=p.p104;
        self.scalar_static_f64[108]=if param_given[105] { 1.0 } else { 0.0 };
        self.scalar_static_f64[109]=p.p22;
        self.scalar_static_f64[110]=p.p105;
        self.scalar_static_bool[16]=(!(self.scalar_static_f64[108]!=0.0));
        self.scalar_static_f64[111]=p.p23;
        self.scalar_static_f64[112]=p.p103;
        self.scalar_static_f64[113]=p.p24;
        self.scalar_static_f64[114]=p.p111;
        self.scalar_static_f64[115]=if param_given[110] { 1.0 } else { 0.0 };
        self.scalar_static_f64[116]=p.p25;
        self.scalar_static_f64[117]=p.p110;
        self.scalar_static_bool[17]=(!(self.scalar_static_f64[115]!=0.0));
        self.scalar_static_f64[118]=p.p101;
        self.scalar_static_f64[119]=p.p132;
        self.scalar_static_f64[120]=p.p129;
        self.scalar_static_f64[121]=p.p84;
        self.scalar_static_f64[122]=p.p127;
        self.scalar_static_f64[123]=p.p86;
        self.scalar_static_f64[124]=p.p128;
        self.scalar_static_f64[125]=p.p91;
        self.scalar_static_f64[126]=p.p92;
        self.scalar_static_f64[127]=p.p93;
        self.scalar_static_f64[128]=p.p37;
        self.scalar_static_f64[129]=(0.5*self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[128]* -0.5);
        self.scalar_static_f64[131]=p.p42;
        self.scalar_static_f64[132]=(0.5*self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(-0.5*self.scalar_static_f64[131]);
        self.scalar_static_f64[134]=p.p50;
        self.scalar_static_f64[135]=(0.5*self.scalar_static_f64[134]);
        self.scalar_static_f64[136]=(-0.5*self.scalar_static_f64[134]);
        self.scalar_static_f64[137]=p.p36;
        self.scalar_static_f64[138]=p.p38;
        self.scalar_static_f64[139]=p.p41;
        self.scalar_static_f64[140]=p.p43;
        self.scalar_static_f64[141]=p.p48;
        self.scalar_static_f64[142]=p.p49;
        self.scalar_static_f64[143]=p.p51;
        self.scalar_static_f64[144]=p.p19;
        self.scalar_static_f64[145]=p.p18;
        self.scalar_static_f64[146]=p.p112;
        self.scalar_static_f64[147]=p.p70;
        self.scalar_static_f64[148]=p.p130;
        self.scalar_static_f64[149]=p.p71;
        self.scalar_static_f64[150]=p.p131;
        self.scalar_static_f64[151]=p.p34;
        self.scalar_static_f64[152]=p.p39;
        self.scalar_static_bool[18]=(self.scalar_static_f64[152]<=0.0);
        self.scalar_static_f64[153]=(1.0-self.scalar_static_f64[151]);
        self.scalar_static_f64[154]=(-self.scalar_static_f64[138]);
        self.scalar_static_f64[155]=f64::powf(self.scalar_static_f64[153],self.scalar_static_f64[154]);
        self.scalar_static_f64[156]=(1.0-self.scalar_static_f64[138]);
        self.scalar_static_f64[157]=(0.5*self.scalar_static_f64[138]);
        self.scalar_static_bool[19]=(!self.scalar_static_bool[18]);
        self.scalar_static_f64[158]=(4.0*self.scalar_static_f64[152]);
        self.scalar_static_f64[159]=(self.scalar_static_f64[152]*self.scalar_static_f64[158]);
        self.scalar_static_f64[160]=p.p44;
        self.scalar_static_bool[20]=(self.scalar_static_f64[160]<=0.0);
        self.scalar_static_f64[161]=(-1.0-self.scalar_static_f64[140]);
        self.scalar_static_f64[162]=f64::powf(self.scalar_static_f64[153],self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(1.0-self.scalar_static_f64[140]);
        self.scalar_static_f64[164]=(0.5*self.scalar_static_f64[140]);
        self.scalar_static_f64[165]=p.p45;
        self.scalar_static_bool[21]=(self.scalar_static_f64[165]>0.0);
        self.scalar_static_f64[166]=(-self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=p.p46;
        self.scalar_static_bool[22]=(self.scalar_static_f64[167]>0.0);
        self.scalar_static_bool[23]=(self.scalar_static_bool[21]&&self.scalar_static_bool[22]);
        self.scalar_static_bool[24]=(!self.scalar_static_bool[20]);
        self.scalar_static_bool[25]=(self.scalar_static_bool[23]&&self.scalar_static_bool[24]);
        self.scalar_static_f64[168]=(4.0*self.scalar_static_f64[160]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[160]*self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(4.0*self.scalar_static_f64[167]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[167]*self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=(-self.scalar_static_f64[140]);
        self.scalar_static_bool[26]=(!self.scalar_static_bool[23]);
        self.scalar_static_bool[27]=(self.scalar_static_bool[24]&&self.scalar_static_bool[26]);
        self.scalar_static_f64[173]=f64::powf(self.scalar_static_f64[153],self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=p.p30;
        self.scalar_static_bool[28]=(self.scalar_static_f64[174]<0.5);
        self.scalar_static_f64[175]=(1.0/self.scalar_static_f64[45]);
        self.scalar_static_f64[176]=f64::powf(1e-8,self.scalar_static_f64[45]);
        self.scalar_static_bool[29]=(!self.scalar_static_bool[28]);
        self.scalar_static_f64[177]=(1.0+self.scalar_static_f64[176]);
        self.scalar_static_bool[30]=(self.scalar_static_f64[57]>0.0);
        self.scalar_static_f64[178]=p.p32;
        self.scalar_static_f64[179]=(1.0-self.scalar_static_f64[178]);
        self.scalar_static_bool[31]=(!self.scalar_static_bool[30]);
        self.scalar_static_f64[180]=p.p55;
        self.scalar_static_bool[32]=(1.0==self.scalar_static_f64[180]);
        self.scalar_static_f64[181]=p.p57;
        self.scalar_static_bool[33]=(self.scalar_static_f64[181]>0.0);
        self.scalar_static_bool[34]=(self.scalar_static_bool[32]&&self.scalar_static_bool[33]);
        self.scalar_static_bool[35]=(!self.scalar_static_bool[33]);
        self.scalar_static_bool[36]=(self.scalar_static_bool[32]&&self.scalar_static_bool[35]);
        self.scalar_static_bool[37]=(self.scalar_static_f64[36]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_bool[32]&&self.scalar_static_bool[37]);
        self.scalar_static_bool[39]=(0.0==self.scalar_static_f64[180]);
        self.scalar_static_bool[40]=(!self.scalar_static_bool[32]);
        self.scalar_static_bool[41]=(self.scalar_static_bool[39]&&self.scalar_static_bool[40]);
        self.scalar_static_bool[42]=(self.scalar_static_bool[37]&&self.scalar_static_bool[41]);
        self.scalar_static_bool[43]=(!self.scalar_static_bool[39]);
        self.scalar_static_bool[44]=(self.scalar_static_bool[40]&&self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=(self.scalar_static_bool[33]&&self.scalar_static_bool[44]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[35]&&self.scalar_static_bool[44]);
        self.scalar_static_bool[47]=(self.scalar_static_bool[37]&&self.scalar_static_bool[44]);
        self.scalar_static_f64[182]=(self.scalar_static_f64[34]*self.scalar_static_f64[180]);
        self.scalar_static_f64[183]=(1.0-self.scalar_static_f64[180]);
        self.scalar_static_f64[184]=(self.scalar_static_f64[34]*self.scalar_static_f64[183]);
        self.scalar_static_bool[48]=(self.scalar_static_f64[84]>0.0);
        self.scalar_static_bool[49]=(self.scalar_static_f64[85]>0.0);
        self.scalar_static_bool[50]=(self.scalar_static_bool[48]||self.scalar_static_bool[49]);
        self.scalar_static_bool[51]=(!self.scalar_static_bool[50]);
        self.scalar_static_f64[185]=p.p83;
        self.scalar_static_bool[52]=(self.scalar_static_f64[185]>0.0);
        self.scalar_static_f64[186]=(1.01-self.scalar_static_f64[140]);
        self.scalar_static_f64[187]=(1.0/self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=(self.scalar_static_f64[140]-1.0);
        self.scalar_static_bool[53]=(!self.scalar_static_bool[52]);
        self.scalar_static_f64[189]=p.p85;
        self.scalar_static_bool[54]=(self.scalar_static_f64[189]>0.0);
        self.scalar_static_f64[190]=p.p87;
        self.scalar_static_f64[191]=(1.01-self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(1.0/self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[190]-1.0);
        self.scalar_static_bool[55]=(!self.scalar_static_bool[54]);
        self.scalar_static_f64[194]=p.p97;
        self.scalar_static_bool[56]=(self.scalar_static_f64[194]>0.0);
        self.scalar_static_f64[195]=p.p95;
        self.scalar_static_bool[57]=(self.scalar_static_f64[195]>0.0);
        self.scalar_static_bool[58]=(self.scalar_static_bool[56]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[196]=p.p94;
        self.scalar_static_bool[59]=(self.scalar_static_f64[196]>0.0);
        self.scalar_static_bool[60]=(self.scalar_static_bool[58]&&self.scalar_static_bool[59]);
        self.scalar_static_bool[61]=(!self.scalar_static_bool[59]);
        self.scalar_static_bool[62]=(self.scalar_static_bool[58]&&self.scalar_static_bool[61]);
        self.scalar_static_f64[197]=p.p96;
        self.scalar_static_bool[63]=(!self.scalar_static_bool[58]);
        self.scalar_static_bool[64]=(self.scalar_static_f64[86]>0.0);
        self.scalar_static_bool[65]=(self.scalar_static_f64[91]>0.0);
        self.scalar_static_bool[66]=(self.scalar_static_bool[64]||self.scalar_static_bool[65]);
        self.scalar_static_bool[67]=(!self.scalar_static_bool[66]);
        self.scalar_static_f64[198]=p.p2;
        self.scalar_static_f64[199]=(-self.scalar_static_f64[198]);
        self.scalar_static_bool[68]=(self.scalar_static_f64[142]>0.0);
        self.scalar_static_f64[200]=p.p52;
        self.scalar_static_bool[69]=(self.scalar_static_f64[200]<=0.0);
        self.scalar_static_bool[70]=(self.scalar_static_bool[68]&&self.scalar_static_bool[69]);
        self.scalar_static_f64[201]=(-self.scalar_static_f64[143]);
        self.scalar_static_f64[202]=f64::powf(self.scalar_static_f64[153],self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=(1.0-self.scalar_static_f64[143]);
        self.scalar_static_f64[204]=(0.5*self.scalar_static_f64[143]);
        self.scalar_static_bool[71]=(!self.scalar_static_bool[69]);
        self.scalar_static_bool[72]=(self.scalar_static_bool[68]&&self.scalar_static_bool[71]);
        self.scalar_static_f64[205]=(4.0*self.scalar_static_f64[200]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[200]*self.scalar_static_f64[205]);
        self.scalar_static_bool[73]=(!self.scalar_static_bool[68]);
        self.scalar_static_f64[207]=p.p76;
        self.scalar_static_f64[208]=p.p77;
        self.scalar_static_f64[209]=p.p78;
        self.scalar_static_f64[210]=p.p81;
        self.scalar_static_f64[211]=p.p47;
        self.scalar_static_f64[212]=p.p53;
        self.scalar_static_f64[213]=p.p35;
        self.scalar_static_f64[214]=p.p40;
        self.scalar_static_f64[215]=p.p102;
        self.scalar_static_f64[216]=p.p82;
        self.scalar_static_f64[217]=(self.scalar_static_f64[96]-1.0);
        self.scalar_static_f64[218]=(self.scalar_static_f64[99]-1.0);
        self.scalar_static_f64[219]=(self.scalar_static_f64[100]-1.0);
        self.scalar_static_f64[220]=(self.scalar_static_f64[103]-1.0);
        self.scalar_static_f64[221]=(self.scalar_static_f64[106]-1.0);
        self.scalar_static_f64[222]=(self.scalar_static_f64[107]-1.0);
        self.scalar_static_f64[223]=(self.scalar_static_f64[110]-1.0);
        self.scalar_static_f64[224]=(self.scalar_static_f64[112]-1.0);
        self.scalar_static_f64[225]=(self.scalar_static_f64[114]-1.0);
        self.scalar_static_f64[226]=(self.scalar_static_f64[117]-1.0);
        self.scalar_static_f64[227]=(self.scalar_static_f64[40]-1.0);
        self.scalar_static_f64[228]=(self.scalar_static_f64[52]-1.0);
        self.scalar_static_f64[229]=(self.scalar_static_f64[59]-1.0);
        self.scalar_static_f64[230]=(self.scalar_static_f64[65]-1.0);
        self.scalar_static_f64[231]=(self.scalar_static_f64[71]-1.0);
        self.scalar_static_f64[232]=(self.scalar_static_f64[76]-1.0);
        self.scalar_static_f64[233]=(self.scalar_static_f64[81]-1.0);
        self.scalar_static_f64[234]=(self.scalar_static_f64[88]-1.0);
        self.scalar_static_f64[235]=(self.scalar_static_f64[93]-1.0);
        self.scalar_static_f64[236]=(self.scalar_static_f64[138]-1.0);
        self.scalar_static_f64[237]=(self.scalar_static_f64[143]-1.0);
        self.scalar_static_f64[238]=(self.scalar_static_f64[38]-1.0);
        self.scalar_static_f64[239]=(self.scalar_static_f64[146]-1.0);
        self.scalar_static_f64[240]=(self.scalar_static_f64[156]-1.0);
        self.scalar_static_f64[241]=(self.scalar_static_f64[163]-1.0);
        self.scalar_static_f64[242]=(self.scalar_static_f64[172]-1.0);
        self.scalar_static_f64[243]=(self.scalar_static_f64[175]-1.0);
        self.scalar_static_f64[244]=(self.scalar_static_f64[45]-1.0);
        self.scalar_static_f64[245]=(self.scalar_static_f64[187]-1.0);
        self.scalar_static_f64[246]=(self.scalar_static_f64[188]-1.0);
        self.scalar_static_f64[247]=(self.scalar_static_f64[192]-1.0);
        self.scalar_static_f64[248]=(self.scalar_static_f64[193]-1.0);
        self.scalar_static_f64[249]=(self.scalar_static_f64[197]-1.0);
        self.scalar_static_f64[250]=(self.scalar_static_f64[203]-1.0);
        self.scalar_static_f64[251]=(-self.scalar_static_f64[213]);
        self.scalar_static_f64[252]=(-self.scalar_static_f64[214]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[216]*0.3333333333333333);
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
        self.scalar_static_f64[254]=(temperature+self.scalar_static_f64[28]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[254]-273.15);
        self.scalar_static_bool[74]=(self.scalar_static_f64[255]<self.scalar_static_f64[30]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[255]-self.scalar_static_f64[29]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[256]-1.0);
        self.scalar_static_f64[258]=(self.scalar_static_f64[257]).exp();
        self.scalar_static_f64[259]=(self.scalar_static_f64[29]+self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(if self.scalar_static_bool[74]{self.scalar_static_f64[259]}else{self.scalar_static_f64[255]});
        self.scalar_static_bool[75]=(self.scalar_static_f64[260]>self.scalar_static_f64[32]);
        self.scalar_static_bool[76]=(!self.scalar_static_bool[74]);
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]&&self.scalar_static_bool[76]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[31]-self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]-1.0);
        self.scalar_static_f64[263]=(self.scalar_static_f64[262]).exp();
        self.scalar_static_f64[264]=(self.scalar_static_f64[31]-self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=(if self.scalar_static_bool[77]{self.scalar_static_f64[264]}else{self.scalar_static_f64[260]});
        self.scalar_static_f64[266]=(273.15+self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[266]*1.380662e-23);
        self.scalar_static_f64[268]=(self.scalar_static_f64[267]/1.602189e-19);
        self.scalar_static_f64[269]=(self.scalar_static_f64[268]*self.scalar_static_f64[35]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[37]/self.scalar_static_f64[269]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[270]).exp();
        self.scalar_static_f64[272]=(self.scalar_static_f64[268]*self.scalar_static_f64[39]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[268]*self.scalar_static_f64[51]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[268]*self.scalar_static_f64[58]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[268]*self.scalar_static_f64[64]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[268]*self.scalar_static_f64[70]);
        self.scalar_static_f64[277]=(self.scalar_static_f64[268]*self.scalar_static_f64[75]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[268]*self.scalar_static_f64[80]);
        self.scalar_static_f64[279]=(self.scalar_static_f64[268]*self.scalar_static_f64[87]);
        self.scalar_static_f64[280]=(self.scalar_static_f64[268]*self.scalar_static_f64[92]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
