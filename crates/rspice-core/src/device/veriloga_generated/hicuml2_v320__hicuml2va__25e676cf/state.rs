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
    pub p133: f64,
    pub p134: f64,
    pub p135: f64,
    pub p136: f64,
    pub p137: f64,
    pub p138: f64,
    pub p139: f64,
    pub p140: f64,
    pub p141: f64,
    pub p142: f64,
    pub p143: f64,
    pub p144: f64,
    pub p145: f64,
    pub p146: f64,
    pub p147: f64,
    pub p148: f64,
    pub p149: f64,
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
            params.p0 = 310.0;
            params.p1 = 2e-30;
            params.p2 = 2e-14;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 1.0;
            params.p10 = 0.0;
            params.p11 = 1.0;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 1e-18;
            params.p15 = 1.0;
            params.p16 = 0.0;
            params.p17 = 2.0;
            params.p18 = 0.0;
            params.p19 = 1.0;
            params.p20 = 0.0;
            params.p21 = 2.0;
            params.p22 = 0.0;
            params.p23 = 1e-16;
            params.p24 = 1.0;
            params.p25 = 0.0;
            params.p26 = 1.0;
            params.p27 = 0.0;
            params.p28 = 40.0;
            params.p29 = 1.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = 0.0;
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 0.0;
            params.p38 = 40.0;
            params.p39 = 1e-20;
            params.p40 = 0.9;
            params.p41 = 0.5;
            params.p42 = 2.5;
            params.p43 = 1e-20;
            params.p44 = 0.9;
            params.p45 = 0.5;
            params.p46 = 2.5;
            params.p47 = 1e-20;
            params.p48 = 0.7;
            params.p49 = 0.4;
            params.p50 = 2.4;
            params.p51 = 100.0;
            params.p52 = 1e-20;
            params.p53 = 0.7;
            params.p54 = 0.4;
            params.p55 = 2.4;
            params.p56 = 100.0;
            params.p57 = 0.0;
            params.p58 = 0.6;
            params.p59 = 0.5;
            params.p60 = 2.4;
            params.p61 = 100.0;
            params.p62 = 0.0;
            params.p63 = 0.6;
            params.p64 = 0.5;
            params.p65 = 100.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 1.0;
            params.p71 = 0.0;
            params.p72 = 0.1;
            params.p73 = 0.0;
            params.p74 = 150.0;
            params.p75 = 0.5;
            params.p76 = 100.0;
            params.p77 = 2.0;
            params.p78 = 0.1;
            params.p79 = 0.0;
            params.p80 = 1.921812;
            params.p81 = 0.001;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.01;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.167;
            params.p88 = 0.333;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 0.6557;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 1.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 0.0;
            params.p98 = 1.0;
            params.p99 = 0.0;
            params.p100 = 1.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 1.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 2.0;
            params.p112 = -1.0;
            params.p113 = 0.0;
            params.p114 = 2.0;
            params.p115 = 0.0;
            params.p116 = 0.0;
            params.p117 = 1.17;
            params.p118 = 1.17;
            params.p119 = 1.17;
            params.p120 = 1.17;
            params.p121 = -0.000102377;
            params.p122 = 0.00043215;
            params.p123 = 3.0;
            params.p124 = 3.5;
            params.p125 = 0.0;
            params.p126 = 1.0;
            params.p127 = 1.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 0.0;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 0.0;
            params.p138 = 1.0;
            params.p139 = 0.0;
            params.p140 = 0.0;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = 0.0;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = 27.0;
            params.p147 = 0.0;
            params.p148 = 1.0;
            params.p149 = 0.001;
            validate_parameter("minr", params.p149, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
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
    pub nodes: [usize; 15],
    pub branches: [usize; 6],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 150]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 20]>,
    pub(crate) ddt_state_previous: Box<[f64; 20]>,
    pub(crate) ddt_state_older: Box<[f64; 20]>,
    pub(crate) ddt_state_initialized: Box<[bool; 20]>,
    pub(crate) ddt_derivative_current: Box<[f64; 20]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 20]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 872]>,
    pub(crate) scalar_static_bool: Box<[bool; 216]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 15;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["ci", "ei", "bp", "bi", "si", "xf1", "xf2", "xf", "n1", "n2"];

    pub const BRANCH_COUNT: usize = 6;
    pub const PARAMETER_COUNT: usize = 150;
    pub const VARIABLE_COUNT: usize = 572;
    pub const DDT_STATE_COUNT: usize = 20;
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
            scalar_static_f64: boxed_zero_f64_array::<872>(),
            scalar_static_bool: boxed_zero_bool_array::<216>(),
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
            "c10" => { validate_parameter("c10", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qp0" => { validate_parameter("qp0", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hf0" => { validate_parameter("hf0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfe" => { validate_parameter("hfe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfb" => { validate_parameter("hfb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfc" => { validate_parameter("hfc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hr0" => { validate_parameter("hr0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjei0" => { validate_parameter("hjei0", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjei" => { validate_parameter("hjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahjei" => { validate_parameter("ahjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rhjei" => { validate_parameter("rhjei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjci" => { validate_parameter("hjci", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeis" => { validate_parameter("ibeis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbei" => { validate_parameter("mbei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ireis" => { validate_parameter("ireis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mrei" => { validate_parameter("mrei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeps" => { validate_parameter("ibeps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbep" => { validate_parameter("mbep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ireps" => { validate_parameter("ireps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mrep" => { validate_parameter("mrep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbhrec" => { validate_parameter("tbhrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcis" => { validate_parameter("ibcis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbci" => { validate_parameter("mbci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcxs" => { validate_parameter("ibcxs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbcx" => { validate_parameter("mbcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibets" => { validate_parameter("ibets", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abet" => { validate_parameter("abet", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tunode" => { validate_parameter("tunode", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibetat0" => { validate_parameter("ibetat0", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbetat" => { validate_parameter("vbetat", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hcavl" => { validate_parameter("hcavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hvdavl" => { validate_parameter("hvdavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcts" => { validate_parameter("ibcts", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abct" => { validate_parameter("abct", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjei0" => { validate_parameter("cjei0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdei" => { validate_parameter("vdei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zei" => { validate_parameter("zei", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajei" => { validate_parameter("ajei", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjep0" => { validate_parameter("cjep0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdep" => { validate_parameter("vdep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zep" => { validate_parameter("zep", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajep" => { validate_parameter("ajep", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajci" => { validate_parameter("ajci", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajcx" => { validate_parameter("ajcx", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajs" => { validate_parameter("ajs", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cscp0" => { validate_parameter("cscp0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdsp" => { validate_parameter("vdsp", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zsp" => { validate_parameter("zsp", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptsp" => { validate_parameter("vptsp", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbvl" => { validate_finite_parameter("tbvl", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gtfe" => { validate_parameter("gtfe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fthc" => { validate_parameter("fthc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.2, "1.2")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcsm" => { validate_parameter("avcsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vcbar" => { validate_parameter("vcbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "icbar" => { validate_parameter("icbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbar" => { validate_parameter("acbar", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fdqr0" => { validate_parameter("fdqr0", value, Some((-0.5, "-0.5")), false, Some((100.0, "100.0")), false, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fcrbi" => { validate_parameter("fcrbi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fqi" => { validate_parameter("fqi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tsf" => { validate_parameter("tsf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsu" => { validate_parameter("rsu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csu" => { validate_parameter("csu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbepar" => { validate_parameter("fbepar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbcpar" => { validate_parameter("fbcpar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccepar" => { validate_parameter("ccepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flcono" => { validate_parameter("flcono", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfbe" => { validate_parameter("cfbe", value, Some((-2.0, "-2.0")), false, Some((-1.0, "-1.0")), false, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfre" => { validate_parameter("kfre", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afre" => { validate_parameter("afre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "latb" => { validate_parameter("latb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "latl" => { validate_parameter("latl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f2vg" => { validate_finite_parameter("f2vg", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetact" => { validate_parameter("zetact", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetabet" => { validate_parameter("zetabet", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgbe" => { validate_parameter("dvgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetahjei" => { validate_parameter("zetahjei", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetavgbe" => { validate_parameter("zetavgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaci" => { validate_parameter("zetaci", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbi" => { validate_parameter("zetarbi", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbx" => { validate_parameter("zetarbx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarcx" => { validate_parameter("zetarcx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetare" => { validate_parameter("zetare", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetacx" => { validate_parameter("zetacx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarth" => { validate_parameter("zetarth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("tnom", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hicumL2va'", name)),
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
        self.scalar_static_f64[0]=p.p148;
        self.scalar_static_f64[1]=p.p0;
        self.scalar_static_bool[0]=(self.scalar_static_f64[1]<=310.0);
        self.scalar_static_f64[2]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[3]=(if (self.scalar_static_f64[2]!=0.0){1.6021918e-19}else{0.0});
        self.scalar_static_f64[4]=(if (self.scalar_static_f64[2]!=0.0){1.3806226e-23}else{0.0});
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_f64[5]=(if self.scalar_static_bool[1]{1.602176634e-19}else{self.scalar_static_f64[3]});
        self.scalar_static_f64[6]=(if self.scalar_static_bool[1]{1.380649e-23}else{self.scalar_static_f64[4]});
        self.scalar_static_f64[7]=p.p146;
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]+273.15);
        self.scalar_static_f64[9]=(self.scalar_static_f64[6]/self.scalar_static_f64[5]);
        self.scalar_static_f64[10]=(self.scalar_static_f64[9]*300.0);
        self.scalar_static_f64[11]=(self.scalar_static_f64[8]*self.scalar_static_f64[9]);
        self.scalar_static_f64[12]=(1.0/self.scalar_static_f64[11]);
        self.scalar_static_f64[13]=p.p121;
        self.scalar_static_f64[14]=(self.scalar_static_f64[8]*self.scalar_static_f64[13]);
        self.scalar_static_f64[15]=(self.scalar_static_f64[8]).ln();
        self.scalar_static_f64[16]=(self.scalar_static_f64[14]*self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=p.p122;
        self.scalar_static_f64[18]=(self.scalar_static_f64[8]*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=p.p131;
        self.scalar_static_f64[20]=(self.scalar_static_f64[8]*self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=p.p117;
        self.scalar_static_f64[22]=(self.scalar_static_f64[16]+self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(self.scalar_static_f64[18]+self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=p.p118;
        self.scalar_static_f64[25]=(self.scalar_static_f64[16]+self.scalar_static_f64[24]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[18]+self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=p.p119;
        self.scalar_static_f64[28]=(self.scalar_static_f64[16]+self.scalar_static_f64[27]);
        self.scalar_static_f64[29]=(self.scalar_static_f64[18]+self.scalar_static_f64[28]);
        self.scalar_static_f64[30]=(self.scalar_static_f64[23]+self.scalar_static_f64[26]);
        self.scalar_static_f64[31]=(self.scalar_static_f64[30]*0.5);
        self.scalar_static_f64[32]=(self.scalar_static_f64[23]+self.scalar_static_f64[29]);
        self.scalar_static_f64[33]=(0.5*self.scalar_static_f64[32]);
        self.scalar_static_f64[34]=(self.scalar_static_f64[21]+self.scalar_static_f64[24]);
        self.scalar_static_f64[35]=(0.5*self.scalar_static_f64[34]);
        self.scalar_static_f64[36]=(self.scalar_static_f64[21]+self.scalar_static_f64[27]);
        self.scalar_static_f64[37]=(0.5*self.scalar_static_f64[36]);
        self.scalar_static_f64[38]=p.p120;
        self.scalar_static_f64[39]=(self.scalar_static_f64[27]+self.scalar_static_f64[38]);
        self.scalar_static_f64[40]=(0.5*self.scalar_static_f64[39]);
        self.scalar_static_f64[41]=(self.scalar_static_f64[13]/self.scalar_static_f64[9]);
        self.scalar_static_f64[42]=(3.0-self.scalar_static_f64[41]);
        self.scalar_static_f64[43]=(1.0+self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=p.p130;
        self.scalar_static_f64[45]=(self.scalar_static_f64[43]-self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=p.p138;
        self.scalar_static_f64[47]=(self.scalar_static_f64[43]-self.scalar_static_f64[46]);
        self.scalar_static_f64[48]=(self.scalar_static_f64[42]-1.5);
        self.scalar_static_f64[49]=p.p107;
        self.scalar_static_f64[50]=(1.0-self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p52;
        self.scalar_static_f64[52]=p.p106;
        self.scalar_static_f64[53]=(self.scalar_static_f64[51]+self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(self.scalar_static_f64[50]*self.scalar_static_f64[53]);
        self.scalar_static_bool[2]=(self.scalar_static_f64[54]>=self.scalar_static_f64[52]);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[56]=(if (self.scalar_static_f64[55]!=0.0){self.scalar_static_f64[52]}else{0.0});
        self.scalar_static_f64[57]=(self.scalar_static_f64[54]-self.scalar_static_f64[52]);
        self.scalar_static_f64[58]=(if (self.scalar_static_f64[55]!=0.0){self.scalar_static_f64[57]}else{0.0});
        self.scalar_static_f64[59]=(self.scalar_static_f64[51]-self.scalar_static_f64[58]);
        self.scalar_static_f64[60]=(if (self.scalar_static_f64[55]!=0.0){self.scalar_static_f64[59]}else{0.0});
        self.scalar_static_bool[3]=(!(self.scalar_static_f64[55]!=0.0));
        self.scalar_static_f64[61]=(if self.scalar_static_bool[3]{self.scalar_static_f64[54]}else{self.scalar_static_f64[56]});
        self.scalar_static_f64[62]=(self.scalar_static_f64[52]-self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(if self.scalar_static_bool[3]{self.scalar_static_f64[62]}else{0.0});
        self.scalar_static_f64[64]=(if self.scalar_static_bool[3]{0.0}else{self.scalar_static_f64[58]});
        self.scalar_static_f64[65]=(if self.scalar_static_bool[3]{self.scalar_static_f64[51]}else{self.scalar_static_f64[60]});
        self.scalar_static_f64[66]=p.p105;
        self.scalar_static_f64[67]=p.p104;
        self.scalar_static_f64[68]=(self.scalar_static_f64[66]*self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(self.scalar_static_f64[67]-self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=p.p22;
        self.scalar_static_bool[4]=(0.0!=self.scalar_static_f64[70]);
        self.scalar_static_f64[71]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[72]=(1.0/self.scalar_static_f64[70]);
        self.scalar_static_f64[73]=(if (self.scalar_static_f64[71]!=0.0){self.scalar_static_f64[72]}else{0.0});
        self.scalar_static_bool[5]=(!(self.scalar_static_f64[71]!=0.0));
        self.scalar_static_f64[74]=(if self.scalar_static_bool[5]{0.0}else{self.scalar_static_f64[73]});
        self.scalar_static_bool[6]=(self.scalar_static_f64[1]<=300.0);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_bool[7]=(!(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_f64[76]=(if self.scalar_static_bool[7]{0.7}else{0.0});
        self.scalar_static_f64[77]=p.p32;
        self.scalar_static_bool[8]=(self.scalar_static_f64[77]>0.0);
        self.scalar_static_f64[78]=p.p47;
        self.scalar_static_bool[9]=(self.scalar_static_f64[78]>0.0);
        self.scalar_static_bool[10]=(self.scalar_static_bool[8]&&self.scalar_static_bool[9]);
        self.scalar_static_f64[79]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[80]=(if (self.scalar_static_f64[79]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[79]!=0.0));
        self.scalar_static_f64[81]=(if self.scalar_static_bool[11]{0.0}else{self.scalar_static_f64[80]});
        self.scalar_static_f64[82]=p.p86;
        self.scalar_static_bool[12]=(0.0!=self.scalar_static_f64[82]);
        self.scalar_static_f64[83]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_f64[84]=p.p88;
        self.scalar_static_bool[13]=(0.0==self.scalar_static_f64[84]);
        self.scalar_static_f64[85]=p.p87;
        self.scalar_static_bool[14]=(0.0==self.scalar_static_f64[85]);
        self.scalar_static_bool[15]=(self.scalar_static_bool[13]&&self.scalar_static_bool[14]);
        self.scalar_static_f64[86]=p.p66;
        self.scalar_static_bool[16]=(0.0==self.scalar_static_f64[86]);
        self.scalar_static_bool[17]=(self.scalar_static_bool[15]||self.scalar_static_bool[16]);
        self.scalar_static_f64[87]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=((self.scalar_static_f64[83]!=0.0)&&(self.scalar_static_f64[87]!=0.0));
        self.scalar_static_f64[88]=(if self.scalar_static_bool[18]{0.0}else{self.scalar_static_f64[82]});
        self.scalar_static_f64[89]=p.p115;
        self.scalar_static_bool[19]=(self.scalar_static_f64[89]>=0.01);
        self.scalar_static_f64[90]=p.p116;
        self.scalar_static_bool[20]=(self.scalar_static_f64[90]>=0.01);
        self.scalar_static_bool[21]=(self.scalar_static_bool[19]||self.scalar_static_bool[20]);
        self.scalar_static_f64[91]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_f64[92]=(self.scalar_static_f64[89]-self.scalar_static_f64[90]);
        self.scalar_static_f64[93]=(0.5*self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=(if (self.scalar_static_f64[91]!=0.0){self.scalar_static_f64[93]}else{0.0});
        self.scalar_static_bool[22]=(self.scalar_static_f64[90]<self.scalar_static_f64[89]);
        self.scalar_static_f64[95]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_bool[23]=((self.scalar_static_f64[91]!=0.0)&&(self.scalar_static_f64[95]!=0.0));
        self.scalar_static_f64[96]=(if self.scalar_static_bool[23]{self.scalar_static_f64[90]}else{0.0});
        self.scalar_static_f64[97]=(if self.scalar_static_bool[23]{self.scalar_static_f64[89]}else{0.0});
        self.scalar_static_bool[24]=(!(self.scalar_static_f64[95]!=0.0));
        self.scalar_static_bool[25]=((self.scalar_static_f64[91]!=0.0)&&self.scalar_static_bool[24]);
        self.scalar_static_f64[98]=(if self.scalar_static_bool[25]{self.scalar_static_f64[89]}else{self.scalar_static_f64[96]});
        self.scalar_static_f64[99]=(if self.scalar_static_bool[25]{self.scalar_static_f64[90]}else{self.scalar_static_f64[97]});
        self.scalar_static_bool[26]=(self.scalar_static_f64[98]<0.01);
        self.scalar_static_f64[100]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=((self.scalar_static_f64[91]!=0.0)&&(self.scalar_static_f64[100]!=0.0));
        self.scalar_static_f64[101]=(if self.scalar_static_bool[27]{1000000000.0}else{0.0});
        self.scalar_static_f64[102]=(if self.scalar_static_bool[27]{170000000.0}else{0.0});
        self.scalar_static_f64[103]=(1.0+self.scalar_static_f64[99]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[103]).ln();
        self.scalar_static_f64[105]=(if self.scalar_static_bool[27]{self.scalar_static_f64[104]}else{0.0});
        self.scalar_static_bool[28]=(!(self.scalar_static_f64[100]!=0.0));
        self.scalar_static_bool[29]=((self.scalar_static_f64[91]!=0.0)&&self.scalar_static_bool[28]);
        self.scalar_static_f64[106]=(1.0/self.scalar_static_f64[89]);
        self.scalar_static_f64[107]=(if self.scalar_static_bool[29]{self.scalar_static_f64[106]}else{self.scalar_static_f64[101]});
        self.scalar_static_f64[108]=(1.0/self.scalar_static_f64[90]);
        self.scalar_static_f64[109]=(if self.scalar_static_bool[29]{self.scalar_static_f64[108]}else{self.scalar_static_f64[101]});
        self.scalar_static_f64[110]=(self.scalar_static_f64[89]/6.0);
        self.scalar_static_f64[111]=(if self.scalar_static_bool[29]{self.scalar_static_f64[110]}else{self.scalar_static_f64[102]});
        self.scalar_static_f64[112]=(self.scalar_static_f64[90]/6.0);
        self.scalar_static_f64[113]=(if self.scalar_static_bool[29]{self.scalar_static_f64[112]}else{self.scalar_static_f64[102]});
        self.scalar_static_f64[114]=(1.0+self.scalar_static_f64[89]);
        self.scalar_static_f64[115]=(1.0+self.scalar_static_f64[90]);
        self.scalar_static_f64[116]=(self.scalar_static_f64[114]/self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[116]).ln();
        self.scalar_static_f64[118]=(if self.scalar_static_bool[29]{self.scalar_static_f64[117]}else{self.scalar_static_f64[105]});
        self.scalar_static_bool[30]=(!(self.scalar_static_f64[91]!=0.0));
        self.scalar_static_f64[119]=(if self.scalar_static_bool[30]{0.0}else{self.scalar_static_f64[94]});
        self.scalar_static_f64[120]=(if self.scalar_static_bool[30]{1000000000.0}else{self.scalar_static_f64[107]});
        self.scalar_static_f64[121]=(if self.scalar_static_bool[30]{1000000000.0}else{self.scalar_static_f64[109]});
        self.scalar_static_f64[122]=(if self.scalar_static_bool[30]{170000000.0}else{self.scalar_static_f64[111]});
        self.scalar_static_f64[123]=(if self.scalar_static_bool[30]{170000000.0}else{self.scalar_static_f64[113]});
        self.scalar_static_f64[124]=(if self.scalar_static_bool[30]{self.scalar_static_f64[90]}else{self.scalar_static_f64[98]});
        self.scalar_static_f64[125]=(if self.scalar_static_bool[30]{self.scalar_static_f64[89]}else{self.scalar_static_f64[99]});
        self.scalar_static_f64[126]=(if self.scalar_static_bool[30]{0.0}else{self.scalar_static_f64[118]});
        self.scalar_static_f64[127]=p.p147;
        self.scalar_static_f64[128]=p.p39;
        self.scalar_static_bool[31]=(self.scalar_static_f64[128]>0.0);
        self.scalar_static_f64[129]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_f64[130]=(self.scalar_static_f64[11]*2.0);
        self.scalar_static_f64[131]=p.p40;
        self.scalar_static_f64[132]=(0.5*self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(self.scalar_static_f64[12]*self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=(self.scalar_static_f64[133]).exp();
        self.scalar_static_f64[135]=(self.scalar_static_f64[131]* -0.5);
        self.scalar_static_f64[136]=(self.scalar_static_f64[12]*self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[136]).exp();
        self.scalar_static_f64[138]=(self.scalar_static_f64[134]-self.scalar_static_f64[137]);
        self.scalar_static_f64[139]=(self.scalar_static_f64[138]).ln();
        self.scalar_static_f64[140]=(self.scalar_static_f64[130]*self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=(if (self.scalar_static_f64[129]!=0.0){self.scalar_static_f64[140]}else{0.0});
        self.scalar_static_f64[142]=p.p41;
        self.scalar_static_f64[143]=p.p42;
        self.scalar_static_f64[144]=(self.scalar_static_f64[143]).abs();
        self.scalar_static_f64[145]=(if (self.scalar_static_f64[129]!=0.0){self.scalar_static_f64[144]}else{0.0});
        self.scalar_static_bool[32]=(self.scalar_static_f64[143]>0.0);
        self.scalar_static_f64[146]=(if self.scalar_static_bool[32]{1.0}else{0.0});
        self.scalar_static_bool[33]=((self.scalar_static_f64[129]!=0.0)&&(self.scalar_static_f64[146]!=0.0));
        self.scalar_static_bool[34]=(!(self.scalar_static_f64[129]!=0.0));
        self.scalar_static_f64[147]=p.p14;
        self.scalar_static_f64[148]=p.p124;
        self.scalar_static_f64[149]=(self.scalar_static_f64[12]*self.scalar_static_f64[24]);
        self.scalar_static_f64[150]=p.p16;
        self.scalar_static_f64[151]=p.p17;
        self.scalar_static_f64[152]=(self.scalar_static_f64[42]/self.scalar_static_f64[151]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[12]*self.scalar_static_f64[35]);
        self.scalar_static_f64[154]=(if self.scalar_static_bool[9]{1.0}else{0.0});
        self.scalar_static_f64[155]=p.p48;
        self.scalar_static_f64[156]=(0.5*self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=(self.scalar_static_f64[12]*self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=(self.scalar_static_f64[157]).exp();
        self.scalar_static_f64[159]=(-0.5*self.scalar_static_f64[155]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[12]*self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[160]).exp();
        self.scalar_static_f64[162]=(self.scalar_static_f64[158]-self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[162]).ln();
        self.scalar_static_f64[164]=(self.scalar_static_f64[130]*self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[164]}else{self.scalar_static_f64[141]});
        self.scalar_static_f64[166]=p.p49;
        self.scalar_static_f64[167]=p.p50;
        self.scalar_static_f64[168]=(self.scalar_static_f64[167]).abs();
        self.scalar_static_f64[169]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[168]}else{0.0});
        self.scalar_static_bool[35]=(self.scalar_static_f64[167]>0.0);
        self.scalar_static_f64[170]=(if self.scalar_static_bool[35]{1.0}else{0.0});
        self.scalar_static_bool[36]=((self.scalar_static_f64[154]!=0.0)&&(self.scalar_static_f64[170]!=0.0));
        self.scalar_static_bool[37]=(!(self.scalar_static_f64[154]!=0.0));
        self.scalar_static_f64[171]=p.p23;
        self.scalar_static_f64[172]=(self.scalar_static_f64[12]*self.scalar_static_f64[27]);
        self.scalar_static_f64[173]=p.p2;
        self.scalar_static_f64[174]=p.p1;
        self.scalar_static_f64[175]=p.p123;
        self.scalar_static_f64[176]=(self.scalar_static_f64[12]*self.scalar_static_f64[21]);
        self.scalar_static_f64[177]=p.p10;
        self.scalar_static_f64[178]=p.p126;
        self.scalar_static_f64[179]=p.p8;
        self.scalar_static_f64[180]=(self.scalar_static_f64[179]-1.0);
        self.scalar_static_f64[181]=(self.scalar_static_f64[180]).abs();
        self.scalar_static_bool[38]=(self.scalar_static_f64[181]<1e-5);
        self.scalar_static_bool[39]=(self.scalar_static_bool[6]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[182]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_f64[183]=p.p9;
        self.scalar_static_f64[184]=p.p125;
        self.scalar_static_f64[185]=p.p127;
        self.scalar_static_bool[40]=(!(self.scalar_static_f64[182]!=0.0));
        self.scalar_static_f64[186]=p.p3;
        self.scalar_static_f64[187]=(self.scalar_static_f64[12]*self.scalar_static_f64[184]);
        self.scalar_static_f64[188]=p.p4;
        self.scalar_static_f64[189]=(self.scalar_static_f64[21]-self.scalar_static_f64[24]);
        self.scalar_static_f64[190]=(self.scalar_static_f64[12]*self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=p.p6;
        self.scalar_static_f64[192]=(self.scalar_static_f64[21]-self.scalar_static_f64[27]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[12]*self.scalar_static_f64[192]);
        self.scalar_static_f64[194]=p.p75;
        self.scalar_static_f64[195]=(self.scalar_static_f64[44]-self.scalar_static_f64[20]);
        self.scalar_static_f64[196]=p.p74;
        self.scalar_static_f64[197]=p.p79;
        self.scalar_static_bool[41]=(self.scalar_static_f64[197]>0.0);
        self.scalar_static_f64[198]=(if self.scalar_static_bool[41]{1.0}else{0.0});
        self.scalar_static_f64[199]=p.p133;
        self.scalar_static_f64[200]=p.p78;
        self.scalar_static_f64[201]=(if (self.scalar_static_f64[198]!=0.0){self.scalar_static_f64[200]}else{0.0});
        self.scalar_static_bool[42]=(!(self.scalar_static_f64[198]!=0.0));
        self.scalar_static_f64[202]=p.p132;
        self.scalar_static_f64[203]=p.p128;
        self.scalar_static_f64[204]=p.p129;
        self.scalar_static_f64[205]=p.p69;
        self.scalar_static_f64[206]=p.p71;
        self.scalar_static_f64[207]=(self.scalar_static_f64[44]-1.0);
        self.scalar_static_bool[43]=(1.0==self.scalar_static_f64[81]);
        self.scalar_static_f64[208]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_f64[209]=p.p139;
        self.scalar_static_f64[210]=p.p33;
        self.scalar_static_f64[211]=p.p140;
        self.scalar_static_bool[44]=(!(self.scalar_static_f64[208]!=0.0));
        self.scalar_static_f64[212]=p.p37;
        self.scalar_static_bool[45]=(self.scalar_static_f64[212]>0.0);
        self.scalar_static_f64[213]=p.p38;
        self.scalar_static_bool[46]=(self.scalar_static_f64[155]>0.0);
        self.scalar_static_bool[47]=(self.scalar_static_bool[9]&&self.scalar_static_bool[46]);
        self.scalar_static_f64[214]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_f64[215]=p.p89;
        self.scalar_static_f64[216]=p.p134;
        self.scalar_static_f64[217]=p.p43;
        self.scalar_static_bool[48]=(self.scalar_static_f64[217]>0.0);
        self.scalar_static_f64[218]=(if self.scalar_static_bool[48]{1.0}else{0.0});
        self.scalar_static_f64[219]=p.p44;
        self.scalar_static_f64[220]=(0.5*self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=(self.scalar_static_f64[12]*self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[221]).exp();
        self.scalar_static_f64[223]=(-0.5*self.scalar_static_f64[219]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[12]*self.scalar_static_f64[223]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[224]).exp();
        self.scalar_static_f64[226]=(self.scalar_static_f64[222]-self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=(self.scalar_static_f64[226]).ln();
        self.scalar_static_f64[228]=(self.scalar_static_f64[130]*self.scalar_static_f64[227]);
        self.scalar_static_f64[229]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[228]}else{self.scalar_static_f64[165]});
        self.scalar_static_f64[230]=p.p45;
        self.scalar_static_f64[231]=p.p46;
        self.scalar_static_f64[232]=(self.scalar_static_f64[231]).abs();
        self.scalar_static_f64[233]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[232]}else{0.0});
        self.scalar_static_bool[49]=(self.scalar_static_f64[231]>0.0);
        self.scalar_static_f64[234]=(if self.scalar_static_bool[49]{1.0}else{0.0});
        self.scalar_static_bool[50]=((self.scalar_static_f64[218]!=0.0)&&(self.scalar_static_f64[234]!=0.0));
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[218]!=0.0));
        self.scalar_static_f64[235]=p.p18;
        self.scalar_static_f64[236]=p.p20;
        self.scalar_static_f64[237]=p.p21;
        self.scalar_static_f64[238]=(self.scalar_static_f64[42]/self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=p.p27;
        self.scalar_static_bool[52]=(self.scalar_static_f64[239]>0.0);
        self.scalar_static_f64[240]=p.p29;
        self.scalar_static_bool[53]=(1.0==self.scalar_static_f64[240]);
        self.scalar_static_bool[54]=(self.scalar_static_bool[48]&&self.scalar_static_bool[53]);
        self.scalar_static_bool[55]=(self.scalar_static_f64[219]>0.0);
        self.scalar_static_bool[56]=(self.scalar_static_bool[54]&&self.scalar_static_bool[55]);
        self.scalar_static_f64[241]=(if self.scalar_static_bool[56]{1.0}else{0.0});
        self.scalar_static_bool[57]=(0.0==self.scalar_static_f64[240]);
        self.scalar_static_bool[58]=(self.scalar_static_bool[31]&&self.scalar_static_bool[57]);
        self.scalar_static_bool[59]=(self.scalar_static_f64[131]>0.0);
        self.scalar_static_bool[60]=(self.scalar_static_bool[58]&&self.scalar_static_bool[59]);
        self.scalar_static_f64[242]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=(!(self.scalar_static_f64[241]!=0.0));
        self.scalar_static_f64[243]=p.p28;
        self.scalar_static_f64[244]=p.p30;
        self.scalar_static_f64[245]=p.p31;
        self.scalar_static_f64[246]=p.p53;
        self.scalar_static_f64[247]=(0.5*self.scalar_static_f64[246]);
        self.scalar_static_f64[248]=(self.scalar_static_f64[12]*self.scalar_static_f64[247]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]).exp();
        self.scalar_static_f64[250]=(-0.5*self.scalar_static_f64[246]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[12]*self.scalar_static_f64[250]);
        self.scalar_static_f64[252]=(self.scalar_static_f64[251]).exp();
        self.scalar_static_f64[253]=(self.scalar_static_f64[249]-self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[253]).ln();
        self.scalar_static_f64[255]=(self.scalar_static_f64[130]*self.scalar_static_f64[254]);
        self.scalar_static_f64[256]=(if (1.0!=0.0){self.scalar_static_f64[255]}else{self.scalar_static_f64[229]});
        self.scalar_static_f64[257]=p.p54;
        self.scalar_static_f64[258]=p.p55;
        self.scalar_static_f64[259]=(self.scalar_static_f64[258]).abs();
        self.scalar_static_f64[260]=(if (1.0!=0.0){self.scalar_static_f64[259]}else{0.0});
        self.scalar_static_bool[62]=(self.scalar_static_f64[258]>0.0);
        self.scalar_static_f64[261]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_bool[63]=((1.0!=0.0)&&(self.scalar_static_f64[261]!=0.0));
        self.scalar_static_f64[262]=p.p25;
        self.scalar_static_f64[263]=p.p57;
        self.scalar_static_bool[64]=(self.scalar_static_f64[263]>0.0);
        self.scalar_static_f64[264]=(if self.scalar_static_bool[64]{1.0}else{0.0});
        self.scalar_static_bool[65]=((self.scalar_static_f64[75]!=0.0)&&(self.scalar_static_f64[264]!=0.0));
        self.scalar_static_f64[265]=p.p58;
        self.scalar_static_f64[266]=(0.5*self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[12]*self.scalar_static_f64[266]);
        self.scalar_static_f64[268]=(self.scalar_static_f64[267]).exp();
        self.scalar_static_f64[269]=(-0.5*self.scalar_static_f64[265]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[12]*self.scalar_static_f64[269]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[270]).exp();
        self.scalar_static_f64[272]=(self.scalar_static_f64[268]-self.scalar_static_f64[271]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[272]).ln();
        self.scalar_static_f64[274]=(self.scalar_static_f64[130]*self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(if self.scalar_static_bool[65]{self.scalar_static_f64[274]}else{self.scalar_static_f64[256]});
        self.scalar_static_f64[276]=p.p59;
        self.scalar_static_f64[277]=(if self.scalar_static_bool[65]{2.4}else{0.0});
        self.scalar_static_bool[66]=((0.0!=0.0)&&self.scalar_static_bool[65]);
        self.scalar_static_bool[67]=(!(self.scalar_static_f64[264]!=0.0));
        self.scalar_static_bool[68]=((self.scalar_static_f64[75]!=0.0)&&self.scalar_static_bool[67]);
        self.scalar_static_f64[278]=(if (self.scalar_static_f64[75]!=0.0){2.4}else{0.0});
        self.scalar_static_bool[69]=(self.scalar_static_bool[7]&&(self.scalar_static_f64[264]!=0.0));
        self.scalar_static_f64[279]=(if self.scalar_static_bool[69]{self.scalar_static_f64[274]}else{self.scalar_static_f64[275]});
        self.scalar_static_f64[280]=p.p60;
        self.scalar_static_f64[281]=(-self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[281]).abs();
        self.scalar_static_bool[70]=(self.scalar_static_f64[281]>0.0);
        self.scalar_static_f64[283]=(if self.scalar_static_bool[70]{1.0}else{0.0});
        self.scalar_static_bool[71]=(self.scalar_static_bool[69]&&(self.scalar_static_f64[283]!=0.0));
        self.scalar_static_bool[72]=(self.scalar_static_bool[7]&&self.scalar_static_bool[67]);
        self.scalar_static_f64[284]=(if self.scalar_static_bool[7]{self.scalar_static_f64[280]}else{self.scalar_static_f64[278]});
        self.scalar_static_f64[285]=p.p99;
        self.scalar_static_f64[286]=(self.scalar_static_f64[12]*self.scalar_static_f64[38]);
        self.scalar_static_f64[287]=p.p97;
        self.scalar_static_f64[288]=p.p101;
        self.scalar_static_f64[289]=(self.scalar_static_f64[46]-1.0);
        self.scalar_static_f64[290]=p.p63;
        self.scalar_static_bool[73]=(self.scalar_static_f64[290]>0.0);
        self.scalar_static_f64[291]=(if self.scalar_static_bool[73]{1.0}else{0.0});
        self.scalar_static_f64[292]=p.p62;
        self.scalar_static_bool[74]=(self.scalar_static_f64[292]>0.0);
        self.scalar_static_f64[293]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_bool[75]=((self.scalar_static_f64[291]!=0.0)&&(self.scalar_static_f64[293]!=0.0));
        self.scalar_static_f64[294]=(0.5*self.scalar_static_f64[290]);
        self.scalar_static_f64[295]=(self.scalar_static_f64[12]*self.scalar_static_f64[294]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[295]).exp();
        self.scalar_static_f64[297]=(-0.5*self.scalar_static_f64[290]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[12]*self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=(self.scalar_static_f64[298]).exp();
        self.scalar_static_f64[300]=(self.scalar_static_f64[296]-self.scalar_static_f64[299]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[300]).ln();
        self.scalar_static_f64[302]=(self.scalar_static_f64[130]*self.scalar_static_f64[301]);
        self.scalar_static_f64[303]=(if self.scalar_static_bool[75]{self.scalar_static_f64[302]}else{self.scalar_static_f64[279]});
        self.scalar_static_f64[304]=p.p64;
        self.scalar_static_f64[305]=(-self.scalar_static_f64[284]);
        self.scalar_static_f64[306]=(self.scalar_static_f64[305]).abs();
        self.scalar_static_f64[307]=(if self.scalar_static_bool[75]{self.scalar_static_f64[306]}else{0.0});
        self.scalar_static_bool[76]=(self.scalar_static_f64[305]>0.0);
        self.scalar_static_f64[308]=(if self.scalar_static_bool[76]{1.0}else{0.0});
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]&&(self.scalar_static_f64[308]!=0.0));
        self.scalar_static_bool[78]=(!(self.scalar_static_f64[293]!=0.0));
        self.scalar_static_bool[79]=((self.scalar_static_f64[291]!=0.0)&&self.scalar_static_bool[78]);
        self.scalar_static_bool[80]=(!(self.scalar_static_f64[291]!=0.0));
        self.scalar_static_f64[309]=p.p96;
        self.scalar_static_f64[310]=p.p136;
        self.scalar_static_f64[311]=p.p90;
        self.scalar_static_f64[312]=p.p135;
        self.scalar_static_f64[313]=p.p95;
        self.scalar_static_f64[314]=p.p137;
        self.scalar_static_f64[315]=p.p142;
        self.scalar_static_f64[316]=p.p143;
        self.scalar_static_f64[317]=p.p144;
        self.scalar_static_f64[318]=p.p141;
        self.scalar_static_bool[81]=(0.0!=self.scalar_static_f64[318]);
        self.scalar_static_f64[319]=p.p149;
        self.scalar_static_bool[82]=(self.scalar_static_f64[315]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[83]=(self.scalar_static_bool[81]&&self.scalar_static_bool[82]);
        self.scalar_static_bool[84]=(self.scalar_static_f64[315]>0.0);
        self.scalar_static_bool[85]=(self.scalar_static_bool[83]&&self.scalar_static_bool[84]);
        self.scalar_static_f64[320]=(if self.scalar_static_bool[85]{1.0}else{0.0});
        self.scalar_static_bool[86]=((self.scalar_static_f64[129]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_f64[321]=(if self.scalar_static_bool[86]{self.scalar_static_f64[140]}else{self.scalar_static_f64[303]});
        self.scalar_static_bool[87]=((self.scalar_static_f64[146]!=0.0)&&self.scalar_static_bool[86]);
        self.scalar_static_bool[88]=(self.scalar_static_bool[34]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[89]=((self.scalar_static_f64[154]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_f64[322]=(if self.scalar_static_bool[89]{self.scalar_static_f64[164]}else{self.scalar_static_f64[321]});
        self.scalar_static_bool[90]=((self.scalar_static_f64[170]!=0.0)&&self.scalar_static_bool[89]);
        self.scalar_static_bool[91]=(self.scalar_static_bool[37]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[92]=((self.scalar_static_f64[75]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[93]=((self.scalar_static_f64[182]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[94]=(self.scalar_static_bool[40]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[95]=((self.scalar_static_f64[198]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[96]=(self.scalar_static_bool[42]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[97]=((self.scalar_static_f64[208]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[98]=(self.scalar_static_bool[44]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[99]=((self.scalar_static_f64[218]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_f64[323]=(if self.scalar_static_bool[99]{self.scalar_static_f64[228]}else{self.scalar_static_f64[322]});
        self.scalar_static_bool[100]=((self.scalar_static_f64[234]!=0.0)&&self.scalar_static_bool[99]);
        self.scalar_static_bool[101]=(self.scalar_static_bool[51]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[102]=((1.0!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_f64[324]=(if self.scalar_static_bool[102]{self.scalar_static_f64[255]}else{self.scalar_static_f64[323]});
        self.scalar_static_bool[103]=((self.scalar_static_f64[261]!=0.0)&&self.scalar_static_bool[102]);
        self.scalar_static_bool[104]=(false&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[105]=((self.scalar_static_f64[264]!=0.0)&&self.scalar_static_bool[92]);
        self.scalar_static_f64[325]=(if self.scalar_static_bool[105]{self.scalar_static_f64[274]}else{self.scalar_static_f64[324]});
        self.scalar_static_bool[106]=((0.0!=0.0)&&self.scalar_static_bool[105]);
        self.scalar_static_bool[107]=(self.scalar_static_bool[67]&&self.scalar_static_bool[92]);
        self.scalar_static_f64[326]=(if self.scalar_static_bool[92]{2.4}else{self.scalar_static_f64[284]});
        self.scalar_static_bool[108]=(self.scalar_static_bool[7]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[109]=((self.scalar_static_f64[264]!=0.0)&&self.scalar_static_bool[108]);
        self.scalar_static_f64[327]=(if self.scalar_static_bool[109]{self.scalar_static_f64[274]}else{self.scalar_static_f64[325]});
        self.scalar_static_bool[110]=((self.scalar_static_f64[283]!=0.0)&&self.scalar_static_bool[109]);
        self.scalar_static_bool[111]=(self.scalar_static_bool[67]&&self.scalar_static_bool[108]);
        self.scalar_static_f64[328]=(if self.scalar_static_bool[108]{self.scalar_static_f64[280]}else{self.scalar_static_f64[326]});
        self.scalar_static_bool[112]=((self.scalar_static_f64[291]!=0.0)&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[113]=((self.scalar_static_f64[293]!=0.0)&&self.scalar_static_bool[112]);
        self.scalar_static_f64[329]=(if self.scalar_static_bool[113]{self.scalar_static_f64[302]}else{self.scalar_static_f64[327]});
        self.scalar_static_f64[330]=(-self.scalar_static_f64[328]);
        self.scalar_static_f64[331]=(self.scalar_static_f64[330]).abs();
        self.scalar_static_bool[114]=(self.scalar_static_f64[330]>0.0);
        self.scalar_static_f64[332]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_bool[115]=(self.scalar_static_bool[113]&&(self.scalar_static_f64[332]!=0.0));
        self.scalar_static_bool[116]=(self.scalar_static_bool[78]&&self.scalar_static_bool[112]);
        self.scalar_static_bool[117]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[320]!=0.0));
        self.scalar_static_bool[118]=(self.scalar_static_f64[147]>0.0);
        self.scalar_static_f64[333]=(if self.scalar_static_bool[118]{1.0}else{0.0});
        self.scalar_static_f64[334]=p.p15;
        self.scalar_static_bool[119]=(!(self.scalar_static_f64[333]!=0.0));
        self.scalar_static_bool[120]=(self.scalar_static_f64[150]>0.0);
        self.scalar_static_f64[335]=(if self.scalar_static_bool[120]{1.0}else{0.0});
        self.scalar_static_bool[121]=(!(self.scalar_static_f64[335]!=0.0));
        self.scalar_static_f64[336]=p.p13;
        self.scalar_static_f64[337]=(-self.scalar_static_f64[142]);
        self.scalar_static_f64[338]=(1.0-self.scalar_static_f64[142]);
        self.scalar_static_f64[339]=p.p51;
        self.scalar_static_bool[122]=(self.scalar_static_f64[339]<100.0);
        self.scalar_static_f64[340]=(if self.scalar_static_bool[122]{1.0}else{0.0});
        self.scalar_static_f64[341]=(self.scalar_static_f64[166]/4.0);
        self.scalar_static_f64[342]=(1.0-self.scalar_static_f64[166]);
        self.scalar_static_f64[343]=(-self.scalar_static_f64[166]);
        self.scalar_static_bool[123]=(!(self.scalar_static_f64[340]!=0.0));
        self.scalar_static_bool[124]=(self.scalar_static_f64[177]>0.0);
        self.scalar_static_f64[344]=(if self.scalar_static_bool[124]{1.0}else{0.0});
        self.scalar_static_f64[345]=p.p11;
        self.scalar_static_bool[125]=(!(self.scalar_static_f64[344]!=0.0));
        self.scalar_static_f64[346]=p.p12;
        self.scalar_static_f64[347]=(-0.8754687373538999/self.scalar_static_f64[166]);
        self.scalar_static_f64[348]=(self.scalar_static_f64[347]).exp();
        self.scalar_static_f64[349]=(1.0-self.scalar_static_f64[348]);
        self.scalar_static_f64[350]=p.p67;
        self.scalar_static_f64[351]=p.p68;
        self.scalar_static_f64[352]=p.p80;
        self.scalar_static_f64[353]=p.p77;
        self.scalar_static_f64[354]=p.p76;
        self.scalar_static_f64[355]=p.p81;
        self.scalar_static_f64[356]=p.p85;
        self.scalar_static_bool[126]=(self.scalar_static_f64[356]>0.0);
        self.scalar_static_bool[127]=(self.scalar_static_f64[1]>=310.0);
        self.scalar_static_f64[357]=(if self.scalar_static_bool[127]{1.0}else{0.0});
        self.scalar_static_bool[128]=(!(self.scalar_static_f64[357]!=0.0));
        self.scalar_static_bool[129]=(self.scalar_static_f64[1]>=320.0);
        self.scalar_static_f64[358]=p.p70;
        self.scalar_static_f64[359]=(1.0+self.scalar_static_f64[358]);
        self.scalar_static_f64[360]=p.p83;
        self.scalar_static_f64[361]=(self.scalar_static_f64[194]/self.scalar_static_f64[196]);
        self.scalar_static_f64[362]=(0.05*self.scalar_static_f64[361]);
        self.scalar_static_bool[130]=(self.scalar_static_f64[360]<self.scalar_static_f64[362]);
        self.scalar_static_f64[363]=(if self.scalar_static_bool[130]{1.0}else{0.0});
        self.scalar_static_bool[131]=(!(self.scalar_static_f64[363]!=0.0));
        self.scalar_static_f64[364]=p.p84;
        self.scalar_static_f64[365]=p.p82;
        self.scalar_static_f64[366]=p.p73;
        self.scalar_static_f64[367]=(1.0-self.scalar_static_f64[366]);
        self.scalar_static_f64[368]=p.p72;
        self.scalar_static_f64[369]=(1.0+self.scalar_static_f64[368]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[369]).sqrt();
        self.scalar_static_f64[371]=(1.0+self.scalar_static_f64[370]);
        self.scalar_static_bool[132]=(self.scalar_static_f64[89]<0.01);
        self.scalar_static_bool[133]=(self.scalar_static_f64[90]<0.01);
        self.scalar_static_bool[134]=(self.scalar_static_bool[132]&&self.scalar_static_bool[133]);
        self.scalar_static_f64[372]=(self.scalar_static_f64[119]).abs();
        self.scalar_static_bool[135]=(self.scalar_static_f64[372]>0.001);
        self.scalar_static_f64[373]=(if self.scalar_static_bool[135]{1.0}else{0.0});
        self.scalar_static_bool[136]=(self.scalar_static_f64[124]<0.01);
        self.scalar_static_f64[374]=(if self.scalar_static_bool[136]{1.0}else{0.0});
        self.scalar_static_f64[375]=(self.scalar_static_f64[125]*0.25);
        self.scalar_static_f64[376]=(-self.scalar_static_f64[126]);
        self.scalar_static_bool[137]=(!(self.scalar_static_f64[374]!=0.0));
        self.scalar_static_f64[377]=(self.scalar_static_f64[121]*self.scalar_static_f64[122]);
        self.scalar_static_f64[378]=(self.scalar_static_f64[120]*self.scalar_static_f64[123]);
        self.scalar_static_f64[379]=(self.scalar_static_f64[119]* -2.0);
        self.scalar_static_bool[138]=(!(self.scalar_static_f64[373]!=0.0));
        self.scalar_static_f64[380]=(self.scalar_static_f64[122]*2.0);
        self.scalar_static_f64[381]=p.p5;
        self.scalar_static_f64[382]=p.p7;
        self.scalar_static_f64[383]=(self.scalar_static_f64[356]*self.scalar_static_f64[382]);
        self.scalar_static_f64[384]=p.p93;
        self.scalar_static_bool[139]=(self.scalar_static_f64[171]>0.0);
        self.scalar_static_f64[385]=(if self.scalar_static_bool[139]{1.0}else{0.0});
        self.scalar_static_f64[386]=p.p24;
        self.scalar_static_bool[140]=(!(self.scalar_static_f64[385]!=0.0));
        self.scalar_static_f64[387]=(1.0/self.scalar_static_f64[166]);
        self.scalar_static_f64[388]=(self.scalar_static_f64[387]-1.0);
        self.scalar_static_f64[389]=p.p35;
        self.scalar_static_bool[141]=(self.scalar_static_f64[389]>0.0);
        self.scalar_static_f64[390]=(if self.scalar_static_bool[141]{1.0}else{0.0});
        self.scalar_static_f64[391]=p.p36;
        self.scalar_static_bool[142]=(!(self.scalar_static_f64[390]!=0.0));
        self.scalar_static_f64[392]=p.p34;
        self.scalar_static_bool[143]=(self.scalar_static_f64[392]>0.0);
        self.scalar_static_f64[393]=(if self.scalar_static_bool[143]{1.0}else{0.0});
        self.scalar_static_bool[144]=(!(self.scalar_static_f64[393]!=0.0));
        self.scalar_static_f64[394]=p.p92;
        self.scalar_static_f64[395]=(1.0+self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=p.p91;
        self.scalar_static_f64[397]=p.p94;
        self.scalar_static_bool[145]=(self.scalar_static_f64[235]>0.0);
        self.scalar_static_f64[398]=(if self.scalar_static_bool[145]{1.0}else{0.0});
        self.scalar_static_f64[399]=p.p19;
        self.scalar_static_bool[146]=(!(self.scalar_static_f64[398]!=0.0));
        self.scalar_static_bool[147]=(self.scalar_static_f64[236]>0.0);
        self.scalar_static_f64[400]=(if self.scalar_static_bool[147]{1.0}else{0.0});
        self.scalar_static_bool[148]=(!(self.scalar_static_f64[400]!=0.0));
        self.scalar_static_f64[401]=(-self.scalar_static_f64[230]);
        self.scalar_static_f64[402]=(1.0-self.scalar_static_f64[230]);
        self.scalar_static_f64[403]=(1.0/self.scalar_static_f64[230]);
        self.scalar_static_f64[404]=(1.0-self.scalar_static_f64[403]);
        self.scalar_static_f64[405]=(1.0/self.scalar_static_f64[142]);
        self.scalar_static_f64[406]=(1.0-self.scalar_static_f64[405]);
        self.scalar_static_f64[407]=p.p56;
        self.scalar_static_bool[149]=(self.scalar_static_f64[407]<100.0);
        self.scalar_static_f64[408]=(if self.scalar_static_bool[149]{1.0}else{0.0});
        self.scalar_static_f64[409]=(self.scalar_static_f64[257]/4.0);
        self.scalar_static_f64[410]=(1.0-self.scalar_static_f64[257]);
        self.scalar_static_bool[150]=(!(self.scalar_static_f64[408]!=0.0));
        self.scalar_static_bool[151]=(self.scalar_static_f64[262]>0.0);
        self.scalar_static_f64[411]=(if self.scalar_static_bool[151]{1.0}else{0.0});
        self.scalar_static_f64[412]=p.p26;
        self.scalar_static_bool[152]=(!(self.scalar_static_f64[411]!=0.0));
        self.scalar_static_f64[413]=p.p61;
        self.scalar_static_bool[153]=(self.scalar_static_f64[413]<100.0);
        self.scalar_static_f64[414]=(if self.scalar_static_bool[153]{1.0}else{0.0});
        self.scalar_static_f64[415]=(self.scalar_static_f64[276]/4.0);
        self.scalar_static_f64[416]=(1.0-self.scalar_static_f64[276]);
        self.scalar_static_bool[154]=(!(self.scalar_static_f64[414]!=0.0));
        self.scalar_static_f64[417]=p.p65;
        self.scalar_static_bool[155]=(self.scalar_static_f64[417]<100.0);
        self.scalar_static_f64[418]=(if self.scalar_static_bool[155]{1.0}else{0.0});
        self.scalar_static_bool[156]=((self.scalar_static_f64[291]!=0.0)&&(self.scalar_static_f64[418]!=0.0));
        self.scalar_static_f64[419]=(self.scalar_static_f64[304]/4.0);
        self.scalar_static_f64[420]=(1.0-self.scalar_static_f64[304]);
        self.scalar_static_bool[157]=(!(self.scalar_static_f64[418]!=0.0));
        self.scalar_static_bool[158]=((self.scalar_static_f64[291]!=0.0)&&self.scalar_static_bool[157]);
        self.scalar_static_bool[159]=(self.scalar_static_f64[287]>0.0);
        self.scalar_static_f64[421]=(if self.scalar_static_bool[159]{1.0}else{0.0});
        self.scalar_static_f64[422]=p.p98;
        self.scalar_static_bool[160]=(self.scalar_static_f64[288]>0.0);
        self.scalar_static_f64[423]=(if self.scalar_static_bool[160]{1.0}else{0.0});
        self.scalar_static_bool[161]=((self.scalar_static_f64[421]!=0.0)&&(self.scalar_static_f64[423]!=0.0));
        self.scalar_static_bool[162]=(!(self.scalar_static_f64[423]!=0.0));
        self.scalar_static_bool[163]=((self.scalar_static_f64[421]!=0.0)&&self.scalar_static_bool[162]);
        self.scalar_static_bool[164]=(!(self.scalar_static_f64[421]!=0.0));
        self.scalar_static_bool[165]=(self.scalar_static_f64[285]>0.0);
        self.scalar_static_f64[424]=(if self.scalar_static_bool[165]{1.0}else{0.0});
        self.scalar_static_f64[425]=p.p100;
        self.scalar_static_bool[166]=(!(self.scalar_static_f64[424]!=0.0));
        self.scalar_static_bool[167]=(self.scalar_static_bool[82]&&self.scalar_static_bool[84]);
        self.scalar_static_f64[426]=(if self.scalar_static_bool[167]{1.0}else{0.0});
        self.scalar_static_bool[168]=(1.0==self.scalar_static_f64[318]);
        self.scalar_static_f64[427]=(if self.scalar_static_bool[168]{1.0}else{0.0});
        self.scalar_static_bool[169]=((self.scalar_static_f64[426]!=0.0)&&(self.scalar_static_f64[427]!=0.0));
        self.scalar_static_bool[170]=(2.0==self.scalar_static_f64[318]);
        self.scalar_static_f64[428]=(if self.scalar_static_bool[170]{1.0}else{0.0});
        self.scalar_static_bool[171]=(!(self.scalar_static_f64[427]!=0.0));
        self.scalar_static_bool[172]=((self.scalar_static_f64[426]!=0.0)&&self.scalar_static_bool[171]);
        self.scalar_static_bool[173]=((self.scalar_static_f64[428]!=0.0)&&self.scalar_static_bool[172]);
        self.scalar_static_bool[174]=(!(self.scalar_static_f64[428]!=0.0));
        self.scalar_static_bool[175]=(self.scalar_static_bool[172]&&self.scalar_static_bool[174]);
        self.scalar_static_bool[176]=(0.0!=self.scalar_static_f64[88]);
        self.scalar_static_f64[429]=(if self.scalar_static_bool[176]{1.0}else{0.0});
        self.scalar_static_bool[177]=(!(self.scalar_static_f64[429]!=0.0));
        self.scalar_static_bool[178]=(self.scalar_static_f64[215]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[179]=(self.scalar_static_f64[215]>0.0);
        self.scalar_static_bool[180]=(self.scalar_static_bool[178]&&self.scalar_static_bool[179]);
        self.scalar_static_f64[430]=(if self.scalar_static_bool[180]{1.0}else{0.0});
        self.scalar_static_bool[181]=(self.scalar_static_f64[384]>0.0);
        self.scalar_static_f64[431]=(if self.scalar_static_bool[181]{1.0}else{0.0});
        self.scalar_static_f64[432]=(if self.scalar_static_bool[53]{1.0}else{0.0});
        self.scalar_static_bool[182]=(self.scalar_static_f64[311]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[183]=(self.scalar_static_f64[311]>0.0);
        self.scalar_static_bool[184]=(self.scalar_static_bool[182]&&self.scalar_static_bool[183]);
        self.scalar_static_f64[433]=(if self.scalar_static_bool[184]{1.0}else{0.0});
        self.scalar_static_bool[185]=(self.scalar_static_f64[313]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[186]=(self.scalar_static_f64[313]>0.0);
        self.scalar_static_bool[187]=(self.scalar_static_bool[185]&&self.scalar_static_bool[186]);
        self.scalar_static_f64[434]=(if self.scalar_static_bool[187]{1.0}else{0.0});
        self.scalar_static_bool[188]=(self.scalar_static_f64[309]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[189]=(self.scalar_static_f64[309]>0.0);
        self.scalar_static_bool[190]=(self.scalar_static_bool[188]&&self.scalar_static_bool[189]);
        self.scalar_static_f64[435]=(if self.scalar_static_bool[190]{1.0}else{0.0});
        self.scalar_static_f64[436]=(if self.scalar_static_bool[129]{1.0}else{0.0});
        self.scalar_static_f64[437]=p.p102;
        self.scalar_static_bool[191]=(self.scalar_static_f64[437]>=self.scalar_static_f64[319]);
        self.scalar_static_bool[192]=(self.scalar_static_f64[437]>0.0);
        self.scalar_static_bool[193]=(self.scalar_static_bool[191]&&self.scalar_static_bool[192]);
        self.scalar_static_f64[438]=(if self.scalar_static_bool[193]{1.0}else{0.0});
        self.scalar_static_f64[439]=p.p103;
        self.scalar_static_bool[194]=(self.scalar_static_f64[439]>0.0);
        self.scalar_static_f64[440]=(if self.scalar_static_bool[194]{1.0}else{0.0});
        self.scalar_static_bool[195]=(self.scalar_static_f64[318]>=1.0);
        self.scalar_static_bool[196]=(self.scalar_static_bool[82]&&self.scalar_static_bool[195]);
        self.scalar_static_bool[197]=(self.scalar_static_bool[84]&&self.scalar_static_bool[196]);
        self.scalar_static_f64[441]=(if self.scalar_static_bool[197]{1.0}else{0.0});
        self.scalar_static_f64[442]=p.p145;
        self.scalar_static_bool[198]=(self.scalar_static_f64[442]>0.0);
        self.scalar_static_f64[443]=(if self.scalar_static_bool[198]{1.0}else{0.0});
        self.scalar_static_f64[444]=p.p109;
        self.scalar_static_bool[199]=(1.0==self.scalar_static_f64[444]);
        self.scalar_static_bool[200]=(self.scalar_static_f64[84]>0.0);
        self.scalar_static_bool[201]=(self.scalar_static_f64[85]>0.0);
        self.scalar_static_bool[202]=(self.scalar_static_bool[200]&&self.scalar_static_bool[201]);
        self.scalar_static_bool[203]=(self.scalar_static_bool[199]&&self.scalar_static_bool[202]);
        self.scalar_static_f64[445]=(if self.scalar_static_bool[203]{1.0}else{0.0});
        self.scalar_static_f64[446]=(if (self.scalar_static_f64[445]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[447]=(self.scalar_static_f64[85]*2.0);
        self.scalar_static_f64[448]=(self.scalar_static_f64[84]*self.scalar_static_f64[84]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[447]-self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=(-self.scalar_static_f64[0]);
        self.scalar_static_bool[204]=((self.scalar_static_f64[430]!=0.0)&&(self.scalar_static_f64[431]!=0.0));
        self.scalar_static_bool[205]=(!(self.scalar_static_f64[432]!=0.0));
        self.scalar_static_f64[451]=p.p108;
        self.scalar_static_bool[206]=((self.scalar_static_f64[424]!=0.0)&&(self.scalar_static_f64[436]!=0.0));
        self.scalar_static_bool[207]=(!(self.scalar_static_f64[436]!=0.0));
        self.scalar_static_bool[208]=((self.scalar_static_f64[357]!=0.0)&&self.scalar_static_bool[207]);
        self.scalar_static_bool[209]=((self.scalar_static_f64[438]!=0.0)&&(self.scalar_static_f64[440]!=0.0));
        self.scalar_static_bool[210]=((self.scalar_static_f64[441]!=0.0)&&(self.scalar_static_f64[443]!=0.0));
        self.scalar_static_bool[211]=(!(self.scalar_static_f64[445]!=0.0));
        self.scalar_static_f64[452]=(self.scalar_static_f64[0]-self.scalar_static_f64[0]);
        self.scalar_static_f64[453]=(if (self.scalar_static_f64[320]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[454]=(if (self.scalar_static_f64[198]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[455]=(if (self.scalar_static_f64[198]!=0.0){self.scalar_static_f64[450]}else{0.0});
        self.scalar_static_f64[456]=(if self.scalar_static_bool[42]{self.scalar_static_f64[0]}else{self.scalar_static_f64[454]});
        self.scalar_static_f64[457]=(if self.scalar_static_bool[42]{self.scalar_static_f64[450]}else{0.0});
        self.scalar_static_f64[458]=(if self.scalar_static_bool[42]{self.scalar_static_f64[452]}else{self.scalar_static_f64[455]});
        self.scalar_static_f64[459]=(self.scalar_static_f64[456]/self.scalar_static_f64[10]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[457]/self.scalar_static_f64[10]);
        self.scalar_static_f64[461]=(self.scalar_static_f64[458]/self.scalar_static_f64[10]);
        self.scalar_static_f64[462]=(if (self.scalar_static_f64[208]!=0.0){self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[463]=(if (self.scalar_static_f64[208]!=0.0){self.scalar_static_f64[450]}else{0.0});
        self.scalar_static_f64[464]=(self.scalar_static_f64[450]/self.scalar_static_f64[245]);
        self.scalar_static_f64[465]=(self.scalar_static_f64[0]/self.scalar_static_f64[245]);
        self.scalar_static_f64[466]=(self.scalar_static_f64[292]*self.scalar_static_f64[450]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[0]*self.scalar_static_f64[292]);
        self.scalar_static_f64[468]=(if (self.scalar_static_f64[429]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[469]=(-self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[84]*self.scalar_static_f64[468]);
        self.scalar_static_f64[471]=(self.scalar_static_f64[86]*self.scalar_static_f64[470]);
        self.scalar_static_f64[472]=(if (self.scalar_static_f64[429]!=0.0){self.scalar_static_f64[471]}else{0.0});
        self.scalar_static_f64[473]=(self.scalar_static_f64[470]/3.0);
        self.scalar_static_f64[474]=(self.scalar_static_f64[86]*self.scalar_static_f64[473]);
        self.scalar_static_f64[475]=(if (self.scalar_static_f64[429]!=0.0){self.scalar_static_f64[474]}else{0.0});
        self.scalar_static_f64[476]=(self.scalar_static_f64[85]*self.scalar_static_f64[468]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[86]*self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(if (self.scalar_static_f64[429]!=0.0){self.scalar_static_f64[477]}else{0.0});
        self.scalar_static_f64[479]=(if self.scalar_static_bool[177]{0.0}else{self.scalar_static_f64[472]});
        self.scalar_static_f64[480]=(if self.scalar_static_bool[177]{0.0}else{self.scalar_static_f64[475]});
        self.scalar_static_f64[481]=(if self.scalar_static_bool[177]{0.0}else{self.scalar_static_f64[478]});
        self.scalar_static_f64[482]=(-self.scalar_static_f64[63]);
        self.scalar_static_f64[483]=(-self.scalar_static_f64[61]);
        self.scalar_static_f64[484]=(-self.scalar_static_f64[68]);
        self.scalar_static_f64[485]=(-self.scalar_static_f64[69]);
        self.scalar_static_f64[486]=(-self.scalar_static_f64[451]);
        self.scalar_static_f64[487]=(if self.scalar_static_bool[206]{-0.0}else{0.0});
        self.scalar_static_f64[488]=(if self.scalar_static_bool[208]{-0.0}else{0.0});
        self.scalar_static_f64[489]=(-1.0/self.scalar_static_f64[437]);
        self.scalar_static_f64[490]=(1.0/self.scalar_static_f64[437]);
        self.scalar_static_f64[491]=(if (self.scalar_static_f64[438]!=0.0){self.scalar_static_f64[489]}else{0.0});
        self.scalar_static_f64[492]=(if (self.scalar_static_f64[438]!=0.0){self.scalar_static_f64[490]}else{0.0});
        self.scalar_static_f64[493]=(-self.scalar_static_f64[439]);
        self.scalar_static_f64[494]=(if (self.scalar_static_f64[445]!=0.0){-1.0}else{0.0});
        self.scalar_static_f64[495]=(if self.scalar_static_bool[211]{1.0}else{0.0});
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
        self.scalar_static_f64[496]=(temperature+self.scalar_static_f64[127]);
        self.scalar_static_bool[212]=(self.scalar_static_f64[496]<73.14999999999998);
        self.scalar_static_f64[497]=(if self.scalar_static_bool[212]{1.0}else{0.0});
        self.scalar_static_f64[498]=(if (self.scalar_static_f64[497]!=0.0){73.14999999999998}else{self.scalar_static_f64[496]});
        self.scalar_static_bool[213]=(self.scalar_static_f64[498]>600.0);
        self.scalar_static_f64[499]=(if self.scalar_static_bool[213]{1.0}else{0.0});
        self.scalar_static_bool[214]=(!(self.scalar_static_f64[497]!=0.0));
        self.scalar_static_bool[215]=((self.scalar_static_f64[499]!=0.0)&&self.scalar_static_bool[214]);
        self.scalar_static_f64[500]=(if self.scalar_static_bool[215]{600.0}else{self.scalar_static_f64[498]});
        self.scalar_static_f64[501]=(self.scalar_static_f64[9]*self.scalar_static_f64[500]);
        self.scalar_static_f64[502]=(1.0/self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[500]-self.scalar_static_f64[8]);
        self.scalar_static_f64[504]=(self.scalar_static_f64[8]/self.scalar_static_f64[500]);
        self.scalar_static_f64[505]=(self.scalar_static_f64[500]/self.scalar_static_f64[8]);
        self.scalar_static_f64[506]=(self.scalar_static_f64[505]).ln();
        self.scalar_static_f64[507]=(self.scalar_static_f64[13]*self.scalar_static_f64[500]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[500]).ln();
        self.scalar_static_f64[509]=(self.scalar_static_f64[507]*self.scalar_static_f64[508]);
        self.scalar_static_f64[510]=(self.scalar_static_f64[17]*self.scalar_static_f64[500]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[21]+self.scalar_static_f64[509]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[510]+self.scalar_static_f64[511]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[24]+self.scalar_static_f64[509]);
        self.scalar_static_f64[514]=(self.scalar_static_f64[510]+self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(self.scalar_static_f64[27]+self.scalar_static_f64[509]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[510]+self.scalar_static_f64[515]);
        self.scalar_static_f64[517]=(self.scalar_static_f64[512]+self.scalar_static_f64[514]);
        self.scalar_static_f64[518]=(0.5*self.scalar_static_f64[517]);
        self.scalar_static_f64[519]=(self.scalar_static_f64[512]+self.scalar_static_f64[516]);
        self.scalar_static_f64[520]=(0.5*self.scalar_static_f64[519]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[505]*self.scalar_static_f64[141]);
        self.scalar_static_f64[522]=(1.0-self.scalar_static_f64[505]);
        self.scalar_static_f64[523]=(self.scalar_static_f64[35]*self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(self.scalar_static_f64[521]+self.scalar_static_f64[523]);
        self.scalar_static_f64[525]=(self.scalar_static_f64[42]*self.scalar_static_f64[501]);
        self.scalar_static_f64[526]=(self.scalar_static_f64[506]*self.scalar_static_f64[525]);
        self.scalar_static_f64[527]=(self.scalar_static_f64[524]-self.scalar_static_f64[526]);
        self.scalar_static_f64[528]=(if (self.scalar_static_f64[129]!=0.0){self.scalar_static_f64[527]}else{0.0});
        self.scalar_static_f64[529]=(self.scalar_static_f64[501]*2.0);
        self.scalar_static_f64[530]=(-self.scalar_static_f64[528]);
        self.scalar_static_f64[531]=(self.scalar_static_f64[502]*self.scalar_static_f64[530]);
        self.scalar_static_f64[532]=(self.scalar_static_f64[531]).exp();
        self.scalar_static_f64[533]=(4.0*self.scalar_static_f64[532]);
        self.scalar_static_f64[534]=(1.0+self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(self.scalar_static_f64[534]).sqrt();
        self.scalar_static_f64[536]=(1.0+self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=(0.5*self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=(self.scalar_static_f64[537]).ln();
        self.scalar_static_f64[539]=(self.scalar_static_f64[529]*self.scalar_static_f64[538]);
        self.scalar_static_f64[540]=(self.scalar_static_f64[528]+self.scalar_static_f64[539]);
        self.scalar_static_f64[541]=(if (self.scalar_static_f64[129]!=0.0){self.scalar_static_f64[540]}else{0.0});
        self.scalar_static_f64[542]=(self.scalar_static_f64[131]/self.scalar_static_f64[541]);
        self.scalar_static_f64[543]=(self.scalar_static_f64[542]).ln();
        self.scalar_static_f64[544]=(self.scalar_static_f64[142]*self.scalar_static_f64[543]);
        self.scalar_static_f64[545]=(self.scalar_static_f64[544]).exp();
        self.scalar_static_f64[546]=(self.scalar_static_f64[128]*self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=(if (self.scalar_static_f64[129]!=0.0){self.scalar_static_f64[546]}else{0.0});
        self.scalar_static_f64[548]=(self.scalar_static_f64[541]*self.scalar_static_f64[143]);
        self.scalar_static_f64[549]=(self.scalar_static_f64[548]/self.scalar_static_f64[131]);
        self.scalar_static_f64[550]=(if self.scalar_static_bool[33]{self.scalar_static_f64[549]}else{self.scalar_static_f64[145]});
        self.scalar_static_f64[551]=(if self.scalar_static_bool[34]{self.scalar_static_f64[128]}else{self.scalar_static_f64[547]});
        self.scalar_static_f64[552]=(if self.scalar_static_bool[34]{self.scalar_static_f64[131]}else{self.scalar_static_f64[541]});
        self.scalar_static_f64[553]=(if self.scalar_static_bool[34]{self.scalar_static_f64[143]}else{self.scalar_static_f64[550]});
        self.scalar_static_f64[554]=(self.scalar_static_f64[506]*self.scalar_static_f64[148]);
        self.scalar_static_f64[555]=(1.0-self.scalar_static_f64[504]);
        self.scalar_static_f64[556]=(self.scalar_static_f64[149]*self.scalar_static_f64[555]);
        self.scalar_static_f64[557]=(self.scalar_static_f64[554]+self.scalar_static_f64[556]);
        self.scalar_static_f64[558]=(self.scalar_static_f64[557]).exp();
        self.scalar_static_f64[559]=(self.scalar_static_f64[147]*self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[506]*self.scalar_static_f64[152]);
        self.scalar_static_f64[561]=(self.scalar_static_f64[555]*self.scalar_static_f64[153]);
        self.scalar_static_f64[562]=(self.scalar_static_f64[561]/self.scalar_static_f64[151]);
        self.scalar_static_f64[563]=(self.scalar_static_f64[560]+self.scalar_static_f64[562]);
        self.scalar_static_f64[564]=(self.scalar_static_f64[563]).exp();
        self.scalar_static_f64[565]=(self.scalar_static_f64[150]*self.scalar_static_f64[564]);
        self.scalar_static_f64[566]=(self.scalar_static_f64[505]*self.scalar_static_f64[165]);
        self.scalar_static_f64[567]=(self.scalar_static_f64[37]*self.scalar_static_f64[522]);
        self.scalar_static_f64[568]=(self.scalar_static_f64[566]+self.scalar_static_f64[567]);
        self.scalar_static_f64[569]=(self.scalar_static_f64[568]-self.scalar_static_f64[526]);
        self.scalar_static_f64[570]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[569]}else{self.scalar_static_f64[528]});
        self.scalar_static_f64[571]=(-self.scalar_static_f64[570]);
        self.scalar_static_f64[572]=(self.scalar_static_f64[502]*self.scalar_static_f64[571]);
        self.scalar_static_f64[573]=(self.scalar_static_f64[572]).exp();
        self.scalar_static_f64[574]=(4.0*self.scalar_static_f64[573]);
        self.scalar_static_f64[575]=(1.0+self.scalar_static_f64[574]);
        self.scalar_static_f64[576]=(self.scalar_static_f64[575]).sqrt();
        self.scalar_static_f64[577]=(1.0+self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=(0.5*self.scalar_static_f64[577]);
        self.scalar_static_f64[579]=(self.scalar_static_f64[578]).ln();
        self.scalar_static_f64[580]=(self.scalar_static_f64[529]*self.scalar_static_f64[579]);
        self.scalar_static_f64[581]=(self.scalar_static_f64[570]+self.scalar_static_f64[580]);
        self.scalar_static_f64[582]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[581]}else{0.0});
        self.scalar_static_f64[583]=(self.scalar_static_f64[155]/self.scalar_static_f64[582]);
        self.scalar_static_f64[584]=(self.scalar_static_f64[583]).ln();
        self.scalar_static_f64[585]=(self.scalar_static_f64[166]*self.scalar_static_f64[584]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[585]).exp();
        self.scalar_static_f64[587]=(self.scalar_static_f64[78]*self.scalar_static_f64[586]);
        self.scalar_static_f64[588]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[587]}else{0.0});
        self.scalar_static_f64[589]=(self.scalar_static_f64[582]*self.scalar_static_f64[167]);
        self.scalar_static_f64[590]=(self.scalar_static_f64[589]/self.scalar_static_f64[155]);
        self.scalar_static_f64[591]=(if self.scalar_static_bool[36]{self.scalar_static_f64[590]}else{self.scalar_static_f64[169]});
        self.scalar_static_f64[592]=(if self.scalar_static_bool[37]{self.scalar_static_f64[78]}else{self.scalar_static_f64[588]});
        self.scalar_static_f64[593]=(if self.scalar_static_bool[37]{self.scalar_static_f64[155]}else{self.scalar_static_f64[582]});
        self.scalar_static_f64[594]=(if self.scalar_static_bool[37]{self.scalar_static_f64[167]}else{self.scalar_static_f64[591]});
        self.scalar_static_f64[595]=(if (self.scalar_static_f64[75]!=0.0){2.4}else{self.scalar_static_f64[594]});
        self.scalar_static_f64[596]=(self.scalar_static_f64[45]*self.scalar_static_f64[506]);
        self.scalar_static_f64[597]=(self.scalar_static_f64[555]*self.scalar_static_f64[172]);
        self.scalar_static_f64[598]=(self.scalar_static_f64[596]+self.scalar_static_f64[597]);
        self.scalar_static_f64[599]=(self.scalar_static_f64[598]).exp();
        self.scalar_static_f64[600]=(self.scalar_static_f64[171]*self.scalar_static_f64[599]);
        self.scalar_static_f64[601]=(self.scalar_static_f64[552]/self.scalar_static_f64[131]);
        self.scalar_static_f64[602]=(self.scalar_static_f64[601]).ln();
        self.scalar_static_f64[603]=(self.scalar_static_f64[142]*self.scalar_static_f64[602]);
        self.scalar_static_f64[604]=(self.scalar_static_f64[603]).exp();
        self.scalar_static_f64[605]=(2.0-self.scalar_static_f64[604]);
        self.scalar_static_f64[606]=(self.scalar_static_f64[173]*self.scalar_static_f64[605]);
        self.scalar_static_f64[607]=(self.scalar_static_f64[506]*self.scalar_static_f64[175]);
        self.scalar_static_f64[608]=(self.scalar_static_f64[555]*self.scalar_static_f64[176]);
        self.scalar_static_f64[609]=(self.scalar_static_f64[607]+self.scalar_static_f64[608]);
        self.scalar_static_f64[610]=(self.scalar_static_f64[609]).exp();
        self.scalar_static_f64[611]=(self.scalar_static_f64[174]*self.scalar_static_f64[610]);
        self.scalar_static_f64[612]=(self.scalar_static_f64[506]*self.scalar_static_f64[178]);
        self.scalar_static_f64[613]=(self.scalar_static_f64[612]).exp();
        self.scalar_static_f64[614]=(self.scalar_static_f64[177]*self.scalar_static_f64[613]);
        self.scalar_static_f64[615]=(self.scalar_static_f64[502]*self.scalar_static_f64[184]);
        self.scalar_static_f64[616]=(self.scalar_static_f64[506]*self.scalar_static_f64[185]);
        self.scalar_static_f64[617]=(self.scalar_static_f64[616]).exp();
        self.scalar_static_f64[618]=(self.scalar_static_f64[617]-1.0);
        self.scalar_static_f64[619]=(self.scalar_static_f64[615]*self.scalar_static_f64[618]);
        self.scalar_static_f64[620]=(self.scalar_static_f64[619]).exp();
        self.scalar_static_f64[621]=(self.scalar_static_f64[183]*self.scalar_static_f64[620]);
        self.scalar_static_f64[622]=(if (self.scalar_static_f64[182]!=0.0){self.scalar_static_f64[621]}else{0.0});
        self.scalar_static_f64[623]=(self.scalar_static_f64[179]*self.scalar_static_f64[620]);
        self.scalar_static_f64[624]=(if self.scalar_static_bool[40]{self.scalar_static_f64[623]}else{self.scalar_static_f64[622]});
        self.scalar_static_f64[625]=(self.scalar_static_f64[555]*self.scalar_static_f64[187]);
        self.scalar_static_f64[626]=(self.scalar_static_f64[625]).exp();
        self.scalar_static_f64[627]=(self.scalar_static_f64[186]*self.scalar_static_f64[626]);
        self.scalar_static_f64[628]=(self.scalar_static_f64[555]*self.scalar_static_f64[190]);
        self.scalar_static_f64[629]=(self.scalar_static_f64[628]).exp();
        self.scalar_static_f64[630]=(self.scalar_static_f64[188]*self.scalar_static_f64[629]);
        self.scalar_static_f64[631]=(self.scalar_static_f64[555]*self.scalar_static_f64[193]);
        self.scalar_static_f64[632]=(self.scalar_static_f64[631]).exp();
        self.scalar_static_f64[633]=(self.scalar_static_f64[191]*self.scalar_static_f64[632]);
        self.scalar_static_f64[634]=(self.scalar_static_f64[506]*self.scalar_static_f64[195]);
        self.scalar_static_f64[635]=(self.scalar_static_f64[634]).exp();
        self.scalar_static_f64[636]=(self.scalar_static_f64[194]*self.scalar_static_f64[635]);
        self.scalar_static_f64[637]=(self.scalar_static_f64[44]*self.scalar_static_f64[506]);
        self.scalar_static_f64[638]=(self.scalar_static_f64[637]).exp();
        self.scalar_static_f64[639]=(self.scalar_static_f64[196]*self.scalar_static_f64[638]);
        self.scalar_static_f64[640]=(1.0/self.scalar_static_f64[639]);
        self.scalar_static_f64[641]=(self.scalar_static_f64[503]*self.scalar_static_f64[199]);
        self.scalar_static_f64[642]=(1.0-self.scalar_static_f64[641]);
        self.scalar_static_f64[643]=(self.scalar_static_f64[197]*self.scalar_static_f64[642]);
        self.scalar_static_f64[644]=(if (self.scalar_static_f64[198]!=0.0){self.scalar_static_f64[643]}else{0.0});
        self.scalar_static_f64[645]=(self.scalar_static_f64[503]*self.scalar_static_f64[202]);
        self.scalar_static_f64[646]=(1.0+self.scalar_static_f64[645]);
        self.scalar_static_f64[647]=(self.scalar_static_f64[200]*self.scalar_static_f64[646]);
        self.scalar_static_f64[648]=(if self.scalar_static_bool[42]{self.scalar_static_f64[647]}else{self.scalar_static_f64[201]});
        self.scalar_static_f64[649]=(if self.scalar_static_bool[42]{self.scalar_static_f64[197]}else{self.scalar_static_f64[644]});
        self.scalar_static_f64[650]=(self.scalar_static_f64[503]*self.scalar_static_f64[203]);
        self.scalar_static_f64[651]=(1.0+self.scalar_static_f64[650]);
        self.scalar_static_f64[652]=(self.scalar_static_f64[503]*self.scalar_static_f64[204]);
        self.scalar_static_f64[653]=(self.scalar_static_f64[503]*self.scalar_static_f64[652]);
        self.scalar_static_f64[654]=(self.scalar_static_f64[651]+self.scalar_static_f64[653]);
        self.scalar_static_f64[655]=(self.scalar_static_f64[86]*self.scalar_static_f64[654]);
        self.scalar_static_f64[656]=(self.scalar_static_f64[506]*self.scalar_static_f64[207]);
        self.scalar_static_f64[657]=(self.scalar_static_f64[656]).exp();
        self.scalar_static_f64[658]=(self.scalar_static_f64[206]*self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=(self.scalar_static_f64[503]*self.scalar_static_f64[209]);
        self.scalar_static_f64[660]=(self.scalar_static_f64[659]).exp();
        self.scalar_static_f64[661]=(self.scalar_static_f64[77]*self.scalar_static_f64[660]);
        self.scalar_static_f64[662]=(if (self.scalar_static_f64[208]!=0.0){self.scalar_static_f64[661]}else{0.0});
        self.scalar_static_f64[663]=(self.scalar_static_f64[503]*self.scalar_static_f64[211]);
        self.scalar_static_f64[664]=(self.scalar_static_f64[663]).exp();
        self.scalar_static_f64[665]=(self.scalar_static_f64[210]*self.scalar_static_f64[664]);
        self.scalar_static_f64[666]=(if (self.scalar_static_f64[208]!=0.0){self.scalar_static_f64[665]}else{0.0});
        self.scalar_static_f64[667]=(if self.scalar_static_bool[44]{self.scalar_static_f64[77]}else{self.scalar_static_f64[662]});
        self.scalar_static_f64[668]=(if self.scalar_static_bool[44]{self.scalar_static_f64[210]}else{self.scalar_static_f64[666]});
        self.scalar_static_f64[669]=(self.scalar_static_f64[33]/self.scalar_static_f64[520]);
        self.scalar_static_f64[670]=(self.scalar_static_f64[593]/self.scalar_static_f64[155]);
        self.scalar_static_f64[671]=(self.scalar_static_f64[506]*self.scalar_static_f64[216]);
        self.scalar_static_f64[672]=(self.scalar_static_f64[671]).exp();
        self.scalar_static_f64[673]=(self.scalar_static_f64[215]*self.scalar_static_f64[672]);
        self.scalar_static_f64[674]=(self.scalar_static_f64[505]*self.scalar_static_f64[229]);
        self.scalar_static_f64[675]=(self.scalar_static_f64[523]+self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=(self.scalar_static_f64[675]-self.scalar_static_f64[526]);
        self.scalar_static_f64[677]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[676]}else{self.scalar_static_f64[570]});
        self.scalar_static_f64[678]=(-self.scalar_static_f64[677]);
        self.scalar_static_f64[679]=(self.scalar_static_f64[502]*self.scalar_static_f64[678]);
        self.scalar_static_f64[680]=(self.scalar_static_f64[679]).exp();
        self.scalar_static_f64[681]=(4.0*self.scalar_static_f64[680]);
        self.scalar_static_f64[682]=(1.0+self.scalar_static_f64[681]);
        self.scalar_static_f64[683]=(self.scalar_static_f64[682]).sqrt();
        self.scalar_static_f64[684]=(1.0+self.scalar_static_f64[683]);
        self.scalar_static_f64[685]=(0.5*self.scalar_static_f64[684]);
        self.scalar_static_f64[686]=(self.scalar_static_f64[685]).ln();
        self.scalar_static_f64[687]=(self.scalar_static_f64[529]*self.scalar_static_f64[686]);
        self.scalar_static_f64[688]=(self.scalar_static_f64[677]+self.scalar_static_f64[687]);
        self.scalar_static_f64[689]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[688]}else{0.0});
        self.scalar_static_f64[690]=(self.scalar_static_f64[219]/self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[690]).ln();
        self.scalar_static_f64[692]=(self.scalar_static_f64[230]*self.scalar_static_f64[691]);
        self.scalar_static_f64[693]=(self.scalar_static_f64[692]).exp();
        self.scalar_static_f64[694]=(self.scalar_static_f64[217]*self.scalar_static_f64[693]);
        self.scalar_static_f64[695]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[694]}else{0.0});
        self.scalar_static_f64[696]=(self.scalar_static_f64[689]*self.scalar_static_f64[231]);
        self.scalar_static_f64[697]=(self.scalar_static_f64[696]/self.scalar_static_f64[219]);
        self.scalar_static_f64[698]=(if self.scalar_static_bool[50]{self.scalar_static_f64[697]}else{self.scalar_static_f64[233]});
        self.scalar_static_f64[699]=(if self.scalar_static_bool[51]{self.scalar_static_f64[217]}else{self.scalar_static_f64[695]});
        self.scalar_static_f64[700]=(if self.scalar_static_bool[51]{self.scalar_static_f64[219]}else{self.scalar_static_f64[689]});
        self.scalar_static_f64[701]=(if self.scalar_static_bool[51]{self.scalar_static_f64[231]}else{self.scalar_static_f64[698]});
        self.scalar_static_f64[702]=(self.scalar_static_f64[558]*self.scalar_static_f64[235]);
        self.scalar_static_f64[703]=(self.scalar_static_f64[506]*self.scalar_static_f64[238]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[561]/self.scalar_static_f64[237]);
        self.scalar_static_f64[705]=(self.scalar_static_f64[703]+self.scalar_static_f64[704]);
        self.scalar_static_f64[706]=(self.scalar_static_f64[705]).exp();
        self.scalar_static_f64[707]=(self.scalar_static_f64[236]*self.scalar_static_f64[706]);
        self.scalar_static_f64[708]=(self.scalar_static_f64[31]/self.scalar_static_f64[518]);
        self.scalar_static_f64[709]=(self.scalar_static_f64[700]/self.scalar_static_f64[219]);
        self.scalar_static_f64[710]=(self.scalar_static_f64[699]/self.scalar_static_f64[217]);
        self.scalar_static_f64[711]=(self.scalar_static_f64[217]/self.scalar_static_f64[699]);
        self.scalar_static_f64[712]=(self.scalar_static_f64[551]/self.scalar_static_f64[128]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[128]/self.scalar_static_f64[551]);
        self.scalar_static_f64[714]=(self.scalar_static_f64[552]-self.scalar_static_f64[131]);
        self.scalar_static_f64[715]=(-self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(self.scalar_static_f64[715]/self.scalar_static_f64[245]);
        self.scalar_static_f64[717]=(self.scalar_static_f64[716]).exp();
        self.scalar_static_f64[718]=(self.scalar_static_f64[244]*self.scalar_static_f64[717]);
        self.scalar_static_f64[719]=(self.scalar_static_f64[505]*self.scalar_static_f64[256]);
        self.scalar_static_f64[720]=(self.scalar_static_f64[567]+self.scalar_static_f64[719]);
        self.scalar_static_f64[721]=(self.scalar_static_f64[720]-self.scalar_static_f64[526]);
        self.scalar_static_f64[722]=(if (1.0!=0.0){self.scalar_static_f64[721]}else{self.scalar_static_f64[677]});
        self.scalar_static_f64[723]=(-self.scalar_static_f64[722]);
        self.scalar_static_f64[724]=(self.scalar_static_f64[502]*self.scalar_static_f64[723]);
        self.scalar_static_f64[725]=(self.scalar_static_f64[724]).exp();
        self.scalar_static_f64[726]=(4.0*self.scalar_static_f64[725]);
        self.scalar_static_f64[727]=(1.0+self.scalar_static_f64[726]);
        self.scalar_static_f64[728]=(self.scalar_static_f64[727]).sqrt();
        self.scalar_static_f64[729]=(1.0+self.scalar_static_f64[728]);
        self.scalar_static_f64[730]=(0.5*self.scalar_static_f64[729]);
        self.scalar_static_f64[731]=(self.scalar_static_f64[730]).ln();
        self.scalar_static_f64[732]=(self.scalar_static_f64[529]*self.scalar_static_f64[731]);
        self.scalar_static_f64[733]=(self.scalar_static_f64[722]+self.scalar_static_f64[732]);
        self.scalar_static_f64[734]=(if (1.0!=0.0){self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[735]=(self.scalar_static_f64[246]/self.scalar_static_f64[734]);
        self.scalar_static_f64[736]=(self.scalar_static_f64[735]).ln();
        self.scalar_static_f64[737]=(self.scalar_static_f64[257]*self.scalar_static_f64[736]);
        self.scalar_static_f64[738]=(self.scalar_static_f64[737]).exp();
        self.scalar_static_f64[739]=(if (1.0!=0.0){self.scalar_static_f64[738]}else{0.0});
        self.scalar_static_f64[740]=(self.scalar_static_f64[734]*self.scalar_static_f64[258]);
        self.scalar_static_f64[741]=(self.scalar_static_f64[740]/self.scalar_static_f64[246]);
        self.scalar_static_f64[742]=(if self.scalar_static_bool[63]{self.scalar_static_f64[741]}else{self.scalar_static_f64[260]});
        self.scalar_static_f64[743]=(if (self.scalar_static_f64[75]!=0.0){2.4}else{self.scalar_static_f64[742]});
        self.scalar_static_f64[744]=(self.scalar_static_f64[64]*self.scalar_static_f64[739]);
        self.scalar_static_f64[745]=(self.scalar_static_f64[65]*self.scalar_static_f64[739]);
        self.scalar_static_f64[746]=(self.scalar_static_f64[47]*self.scalar_static_f64[506]);
        self.scalar_static_f64[747]=(self.scalar_static_f64[597]+self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=(self.scalar_static_f64[747]).exp();
        self.scalar_static_f64[749]=(self.scalar_static_f64[262]*self.scalar_static_f64[748]);
        self.scalar_static_f64[750]=(self.scalar_static_f64[505]*self.scalar_static_f64[275]);
        self.scalar_static_f64[751]=(self.scalar_static_f64[40]*self.scalar_static_f64[522]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[750]+self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(self.scalar_static_f64[752]-self.scalar_static_f64[526]);
        self.scalar_static_f64[754]=(if self.scalar_static_bool[65]{self.scalar_static_f64[753]}else{self.scalar_static_f64[722]});
        self.scalar_static_f64[755]=(-self.scalar_static_f64[754]);
        self.scalar_static_f64[756]=(self.scalar_static_f64[502]*self.scalar_static_f64[755]);
        self.scalar_static_f64[757]=(self.scalar_static_f64[756]).exp();
        self.scalar_static_f64[758]=(4.0*self.scalar_static_f64[757]);
        self.scalar_static_f64[759]=(1.0+self.scalar_static_f64[758]);
        self.scalar_static_f64[760]=(self.scalar_static_f64[759]).sqrt();
        self.scalar_static_f64[761]=(1.0+self.scalar_static_f64[760]);
        self.scalar_static_f64[762]=(0.5*self.scalar_static_f64[761]);
        self.scalar_static_f64[763]=(self.scalar_static_f64[762]).ln();
        self.scalar_static_f64[764]=(self.scalar_static_f64[529]*self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=(self.scalar_static_f64[754]+self.scalar_static_f64[764]);
        self.scalar_static_f64[766]=(if self.scalar_static_bool[65]{self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_f64[767]=(self.scalar_static_f64[265]/self.scalar_static_f64[766]);
        self.scalar_static_f64[768]=(self.scalar_static_f64[767]).ln();
        self.scalar_static_f64[769]=(self.scalar_static_f64[276]*self.scalar_static_f64[768]);
        self.scalar_static_f64[770]=(self.scalar_static_f64[769]).exp();
        self.scalar_static_f64[771]=(self.scalar_static_f64[263]*self.scalar_static_f64[770]);
        self.scalar_static_f64[772]=(if self.scalar_static_bool[65]{self.scalar_static_f64[771]}else{0.0});
        self.scalar_static_f64[773]=(self.scalar_static_f64[766]* -2.4);
        self.scalar_static_f64[774]=(self.scalar_static_f64[773]/self.scalar_static_f64[265]);
        self.scalar_static_f64[775]=(if self.scalar_static_bool[66]{self.scalar_static_f64[774]}else{self.scalar_static_f64[277]});
        self.scalar_static_f64[776]=(if self.scalar_static_bool[68]{self.scalar_static_f64[263]}else{self.scalar_static_f64[772]});
        self.scalar_static_f64[777]=(if self.scalar_static_bool[68]{self.scalar_static_f64[265]}else{self.scalar_static_f64[766]});
        self.scalar_static_f64[778]=(if self.scalar_static_bool[68]{-2.4}else{self.scalar_static_f64[775]});
        self.scalar_static_f64[779]=(self.scalar_static_f64[505]*self.scalar_static_f64[279]);
        self.scalar_static_f64[780]=(self.scalar_static_f64[751]+self.scalar_static_f64[779]);
        self.scalar_static_f64[781]=(self.scalar_static_f64[780]-self.scalar_static_f64[526]);
        self.scalar_static_f64[782]=(if self.scalar_static_bool[69]{self.scalar_static_f64[781]}else{self.scalar_static_f64[754]});
        self.scalar_static_f64[783]=(-self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=(self.scalar_static_f64[502]*self.scalar_static_f64[783]);
        self.scalar_static_f64[785]=(self.scalar_static_f64[784]).exp();
        self.scalar_static_f64[786]=(4.0*self.scalar_static_f64[785]);
        self.scalar_static_f64[787]=(1.0+self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=(self.scalar_static_f64[787]).sqrt();
        self.scalar_static_f64[789]=(1.0+self.scalar_static_f64[788]);
        self.scalar_static_f64[790]=(0.5*self.scalar_static_f64[789]);
        self.scalar_static_f64[791]=(self.scalar_static_f64[790]).ln();
        self.scalar_static_f64[792]=(self.scalar_static_f64[529]*self.scalar_static_f64[791]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[782]+self.scalar_static_f64[792]);
        self.scalar_static_f64[794]=(if self.scalar_static_bool[69]{self.scalar_static_f64[793]}else{self.scalar_static_f64[777]});
        self.scalar_static_f64[795]=(self.scalar_static_f64[265]/self.scalar_static_f64[794]);
        self.scalar_static_f64[796]=(self.scalar_static_f64[795]).ln();
        self.scalar_static_f64[797]=(self.scalar_static_f64[276]*self.scalar_static_f64[796]);
        self.scalar_static_f64[798]=(self.scalar_static_f64[797]).exp();
        self.scalar_static_f64[799]=(self.scalar_static_f64[263]*self.scalar_static_f64[798]);
        self.scalar_static_f64[800]=(if self.scalar_static_bool[69]{self.scalar_static_f64[799]}else{self.scalar_static_f64[776]});
        self.scalar_static_f64[801]=(if self.scalar_static_bool[69]{self.scalar_static_f64[282]}else{self.scalar_static_f64[778]});
        self.scalar_static_f64[802]=(self.scalar_static_f64[794]*self.scalar_static_f64[281]);
        self.scalar_static_f64[803]=(self.scalar_static_f64[802]/self.scalar_static_f64[265]);
        self.scalar_static_f64[804]=(if self.scalar_static_bool[71]{self.scalar_static_f64[803]}else{self.scalar_static_f64[801]});
        self.scalar_static_f64[805]=(if self.scalar_static_bool[72]{self.scalar_static_f64[263]}else{self.scalar_static_f64[800]});
        self.scalar_static_f64[806]=(if self.scalar_static_bool[72]{self.scalar_static_f64[265]}else{self.scalar_static_f64[794]});
        self.scalar_static_f64[807]=(if self.scalar_static_bool[72]{self.scalar_static_f64[281]}else{self.scalar_static_f64[804]});
        self.scalar_static_f64[808]=(self.scalar_static_f64[48]*self.scalar_static_f64[506]);
        self.scalar_static_f64[809]=(self.scalar_static_f64[555]*self.scalar_static_f64[286]);
        self.scalar_static_f64[810]=(self.scalar_static_f64[808]+self.scalar_static_f64[809]);
        self.scalar_static_f64[811]=(self.scalar_static_f64[810]).exp();
        self.scalar_static_f64[812]=(self.scalar_static_f64[285]*self.scalar_static_f64[811]);
        self.scalar_static_f64[813]=(self.scalar_static_f64[597]+self.scalar_static_f64[808]);
        self.scalar_static_f64[814]=(self.scalar_static_f64[813]).exp();
        self.scalar_static_f64[815]=(self.scalar_static_f64[287]*self.scalar_static_f64[814]);
        self.scalar_static_f64[816]=(self.scalar_static_f64[506]*self.scalar_static_f64[289]);
        self.scalar_static_f64[817]=(self.scalar_static_f64[816]).exp();
        self.scalar_static_f64[818]=(self.scalar_static_f64[288]*self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[505]*self.scalar_static_f64[303]);
        self.scalar_static_f64[820]=(self.scalar_static_f64[751]+self.scalar_static_f64[819]);
        self.scalar_static_f64[821]=(self.scalar_static_f64[820]-self.scalar_static_f64[526]);
        self.scalar_static_f64[822]=(if self.scalar_static_bool[75]{self.scalar_static_f64[821]}else{self.scalar_static_f64[782]});
        self.scalar_static_f64[823]=(-self.scalar_static_f64[822]);
        self.scalar_static_f64[824]=(self.scalar_static_f64[502]*self.scalar_static_f64[823]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[824]).exp();
        self.scalar_static_f64[826]=(4.0*self.scalar_static_f64[825]);
        self.scalar_static_f64[827]=(1.0+self.scalar_static_f64[826]);
        self.scalar_static_f64[828]=(self.scalar_static_f64[827]).sqrt();
        self.scalar_static_f64[829]=(1.0+self.scalar_static_f64[828]);
        self.scalar_static_f64[830]=(0.5*self.scalar_static_f64[829]);
        self.scalar_static_f64[831]=(self.scalar_static_f64[830]).ln();
        self.scalar_static_f64[832]=(self.scalar_static_f64[529]*self.scalar_static_f64[831]);
        self.scalar_static_f64[833]=(self.scalar_static_f64[822]+self.scalar_static_f64[832]);
        self.scalar_static_f64[834]=(if self.scalar_static_bool[75]{self.scalar_static_f64[833]}else{0.0});
        self.scalar_static_f64[835]=(self.scalar_static_f64[290]/self.scalar_static_f64[834]);
        self.scalar_static_f64[836]=(self.scalar_static_f64[835]).ln();
        self.scalar_static_f64[837]=(self.scalar_static_f64[304]*self.scalar_static_f64[836]);
        self.scalar_static_f64[838]=(self.scalar_static_f64[837]).exp();
        self.scalar_static_f64[839]=(self.scalar_static_f64[292]*self.scalar_static_f64[838]);
        self.scalar_static_f64[840]=(if self.scalar_static_bool[75]{self.scalar_static_f64[839]}else{0.0});
        self.scalar_static_f64[841]=(self.scalar_static_f64[834]*self.scalar_static_f64[305]);
        self.scalar_static_f64[842]=(self.scalar_static_f64[841]/self.scalar_static_f64[290]);
        self.scalar_static_f64[843]=(if self.scalar_static_bool[77]{self.scalar_static_f64[842]}else{self.scalar_static_f64[307]});
        self.scalar_static_f64[844]=(if self.scalar_static_bool[79]{self.scalar_static_f64[292]}else{self.scalar_static_f64[840]});
        self.scalar_static_f64[845]=(if self.scalar_static_bool[79]{self.scalar_static_f64[290]}else{self.scalar_static_f64[834]});
        self.scalar_static_f64[846]=(if self.scalar_static_bool[79]{self.scalar_static_f64[305]}else{self.scalar_static_f64[843]});
        self.scalar_static_f64[847]=(if self.scalar_static_bool[80]{self.scalar_static_f64[292]}else{self.scalar_static_f64[844]});
        self.scalar_static_f64[848]=(if self.scalar_static_bool[80]{self.scalar_static_f64[290]}else{self.scalar_static_f64[845]});
        self.scalar_static_f64[849]=(if self.scalar_static_bool[80]{self.scalar_static_f64[284]}else{self.scalar_static_f64[846]});
        self.scalar_static_f64[850]=(self.scalar_static_f64[506]*self.scalar_static_f64[310]);
        self.scalar_static_f64[851]=(self.scalar_static_f64[850]).exp();
        self.scalar_static_f64[852]=(self.scalar_static_f64[309]*self.scalar_static_f64[851]);
        self.scalar_static_f64[853]=(self.scalar_static_f64[506]*self.scalar_static_f64[312]);
        self.scalar_static_f64[854]=(self.scalar_static_f64[853]).exp();
        self.scalar_static_f64[855]=(self.scalar_static_f64[311]*self.scalar_static_f64[854]);
        self.scalar_static_f64[856]=(self.scalar_static_f64[506]*self.scalar_static_f64[314]);
        self.scalar_static_f64[857]=(self.scalar_static_f64[856]).exp();
        self.scalar_static_f64[858]=(self.scalar_static_f64[313]*self.scalar_static_f64[857]);
        self.scalar_static_f64[859]=(self.scalar_static_f64[506]*self.scalar_static_f64[316]);
        self.scalar_static_f64[860]=(self.scalar_static_f64[859]).exp();
        self.scalar_static_f64[861]=(self.scalar_static_f64[315]*self.scalar_static_f64[860]);
        self.scalar_static_f64[862]=(self.scalar_static_f64[503]*self.scalar_static_f64[317]);
        self.scalar_static_f64[863]=(1.0+self.scalar_static_f64[862]);
        self.scalar_static_f64[864]=(self.scalar_static_f64[861]*self.scalar_static_f64[863]);
        self.scalar_static_f64[865]=(if self.scalar_static_bool[86]{self.scalar_static_f64[144]}else{self.scalar_static_f64[553]});
        self.scalar_static_f64[866]=(if self.scalar_static_bool[89]{self.scalar_static_f64[168]}else{self.scalar_static_f64[595]});
        self.scalar_static_f64[867]=(if self.scalar_static_bool[95]{self.scalar_static_f64[200]}else{self.scalar_static_f64[648]});
        self.scalar_static_f64[868]=(if self.scalar_static_bool[99]{self.scalar_static_f64[232]}else{self.scalar_static_f64[701]});
        self.scalar_static_f64[869]=(if self.scalar_static_bool[102]{self.scalar_static_f64[259]}else{self.scalar_static_f64[743]});
        self.scalar_static_f64[870]=(if self.scalar_static_bool[105]{2.4}else{self.scalar_static_f64[807]});
        self.scalar_static_f64[871]=(if self.scalar_static_bool[113]{self.scalar_static_f64[331]}else{self.scalar_static_f64[849]});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
