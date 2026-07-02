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
            params.p2 = 505.5;
            params.p3 = 1.0;
            params.p4 = 25.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 2.2e-17;
            params.p9 = 1.0;
            params.p10 = 1.0;
            params.p11 = 0.1;
            params.p12 = 2.5;
            params.p13 = 44.0;
            params.p14 = 1.0;
            params.p15 = 1.0000000000000001e-19;
            params.p16 = 1.0;
            params.p17 = 0.0;
            params.p18 = 1.0;
            params.p19 = 2.7000000000000005e-15;
            params.p20 = 2.0;
            params.p21 = 0.0;
            params.p22 = 2.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.68;
            params.p27 = 0.0;
            params.p28 = 3.1400000000000002e-18;
            params.p29 = 0.014289999999999999;
            params.p30 = 1e-15;
            params.p31 = 2.0;
            params.p32 = 0.63;
            params.p33 = 0.0;
            params.p34 = 22.0;
            params.p35 = 0.0;
            params.p36 = 22.0;
            params.p37 = 1e-6;
            params.p38 = 1.0;
            params.p39 = 400.0;
            params.p40 = -0.37;
            params.p41 = 0.5;
            params.p42 = 25.0;
            params.p43 = 0.1;
            params.p44 = 1.1e-6;
            params.p45 = 3.0;
            params.p46 = 0.3;
            params.p47 = 0.004;
            params.p48 = -0.37;
            params.p49 = -0.37;
            params.p50 = 0.3;
            params.p51 = 0.004;
            params.p52 = 1.0;
            params.p53 = 5.0;
            params.p54 = 23.0;
            params.p55 = 18.0;
            params.p56 = 12.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 150.0;
            params.p60 = 1250.0;
            params.p61 = 0.004;
            params.p62 = 0.3;
            params.p63 = 0.68;
            params.p64 = 7.3e-14;
            params.p65 = 0.95;
            params.p66 = 0.4;
            params.p67 = 0.4;
            params.p68 = 0.0;
            params.p69 = 7.800000000000001e-14;
            params.p70 = 0.68;
            params.p71 = 0.5;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.35;
            params.p75 = 0.5;
            params.p76 = 0.032;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.68;
            params.p80 = 100.0;
            params.p81 = 4.0;
            params.p82 = 1000.0;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 2e-12;
            params.p86 = 4.2e-12;
            params.p87 = 4.1e-11;
            params.p88 = 5.2e-10;
            params.p89 = 1e-11;
            params.p90 = 1.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.3333333333333333;
            params.p94 = 0.0;
            params.p95 = 0.3;
            params.p96 = 0.0;
            params.p97 = 1.0;
            params.p98 = 2.5;
            params.p99 = 2.5;
            params.p100 = 0.62;
            params.p101 = 2.0;
            params.p102 = 1.3;
            params.p103 = 2.0;
            params.p104 = 1.17;
            params.p105 = 1.12;
            params.p106 = 1.12;
            params.p107 = 1.12;
            params.p108 = 1.12;
            params.p109 = 1.18;
            params.p110 = 1.12;
            params.p111 = 1.125;
            params.p112 = 1.15;
            params.p113 = 1.15;
            params.p114 = 0.000473;
            params.p115 = 636.0;
            params.p116 = 1.15;
            params.p117 = 0.000473;
            params.p118 = 636.0;
            params.p119 = 0.05;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0005;
            params.p124 = 200.0;
            params.p125 = 2.0;
            params.p126 = 2.0;
            params.p127 = 2e-11;
            params.p128 = 2e-11;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 300.0;
            params.p134 = 3.0000000000000004e-9;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 2.0;
            params.p138 = 400.0;
            params.p139 = 1e-40;
            params.p140 = 1e-40;
            params.p141 = 0.001;
            validate_parameter("minr", params.p141, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p142 = 0.0;
            params.p143 = 1.0;
            params.p144 = 0.0;
            params.p145 = 0.16;
            params.p146 = 0.0;
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
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 147]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 10]>,
    pub(crate) ddt_state_previous: Box<[f64; 10]>,
    pub(crate) ddt_state_older: Box<[f64; 10]>,
    pub(crate) ddt_state_initialized: Box<[bool; 10]>,
    pub(crate) ddt_derivative_current: Box<[f64; 10]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 10]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 378]>,
    pub(crate) scalar_static_bool: Box<[bool; 86]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 147;
    pub const VARIABLE_COUNT: usize = 585;
    pub const DDT_STATE_COUNT: usize = 10;
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
            scalar_static_f64: boxed_zero_f64_array::<378>(),
            scalar_static_bool: boxed_zero_bool_array::<86>(),
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
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult" => { validate_parameter("mult", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((505.5, "505.5")), false, Some((505.51, "505.51")), true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("tref", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exmod" => { validate_parameter("exmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exphi" => { validate_parameter("exphi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exavl" => { validate_parameter("exavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swnlsh" => { validate_parameter("swnlsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ath" => { validate_finite_parameter("ath", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505t_va'", name)),
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
        self.scalar_static_f64[0]=p.p3;
        self.scalar_static_bool[0]=(self.scalar_static_f64[0]==1.0);
        self.scalar_static_f64[1]=(if self.scalar_static_bool[0]{70300000.0}else{0.0});
        self.scalar_static_f64[2]=(if self.scalar_static_bool[0]{123000000.0}else{0.0});
        self.scalar_static_bool[1]=(!self.scalar_static_bool[0]);
        self.scalar_static_f64[3]=(if self.scalar_static_bool[1]{158000000.0}else{self.scalar_static_f64[1]});
        self.scalar_static_f64[4]=(if self.scalar_static_bool[1]{204000000.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[5]=p.p32;
        self.scalar_static_f64[6]=(1.0-self.scalar_static_f64[5]);
        self.scalar_static_f64[7]=p.p4;
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]+273.15);
        self.scalar_static_f64[9]=p.p0;
        self.scalar_static_f64[10]=p.p141;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[10]);
        self.scalar_static_f64[11]=(if self.scalar_static_bool[2]{1e-12}else{0.0});
        self.scalar_static_bool[3]=(!self.scalar_static_bool[2]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[3]{self.scalar_static_f64[10]}else{self.scalar_static_f64[11]});
        self.scalar_static_f64[13]=p.p1;
        self.scalar_static_f64[14]=(self.scalar_static_f64[12]*self.scalar_static_f64[13]);
        self.scalar_static_f64[15]=(1.0/self.scalar_static_f64[14]);
        self.scalar_static_f64[16]=p.p66;
        self.scalar_static_f64[17]=(2.0-self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=f64::powf(2.0,self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(1.0/self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=p.p113;
        self.scalar_static_f64[21]=p.p114;
        self.scalar_static_f64[22]=(self.scalar_static_f64[8]*self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(self.scalar_static_f64[8]*self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=p.p115;
        self.scalar_static_f64[25]=(self.scalar_static_f64[8]+self.scalar_static_f64[24]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[23]/self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=(self.scalar_static_f64[20]+self.scalar_static_f64[26]);
        self.scalar_static_f64[28]=(self.scalar_static_f64[27]-0.05);
        self.scalar_static_f64[29]=(self.scalar_static_f64[28]/0.1);
        self.scalar_static_bool[4]=(self.scalar_static_f64[27]<0.05);
        self.scalar_static_f64[30]=(self.scalar_static_f64[29]).exp();
        self.scalar_static_f64[31]=(1.0+self.scalar_static_f64[30]);
        self.scalar_static_f64[32]=(self.scalar_static_f64[31]).ln();
        self.scalar_static_f64[33]=(0.1*self.scalar_static_f64[32]);
        self.scalar_static_f64[34]=(0.05+self.scalar_static_f64[33]);
        self.scalar_static_f64[35]=(if self.scalar_static_bool[4]{self.scalar_static_f64[34]}else{0.0});
        self.scalar_static_bool[5]=(!self.scalar_static_bool[4]);
        self.scalar_static_f64[36]=(-self.scalar_static_f64[29]);
        self.scalar_static_f64[37]=(self.scalar_static_f64[36]).exp();
        self.scalar_static_f64[38]=(1.0+self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=(self.scalar_static_f64[38]).ln();
        self.scalar_static_f64[40]=(0.1*self.scalar_static_f64[39]);
        self.scalar_static_f64[41]=(self.scalar_static_f64[27]+self.scalar_static_f64[40]);
        self.scalar_static_f64[42]=(if self.scalar_static_bool[5]{self.scalar_static_f64[41]}else{self.scalar_static_f64[35]});
        self.scalar_static_f64[43]=(1.0/self.scalar_static_f64[20]);
        self.scalar_static_f64[44]=p.p65;
        self.scalar_static_f64[45]=(1.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=p.p70;
        self.scalar_static_f64[47]=p.p71;
        self.scalar_static_f64[48]=(2.0-self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=f64::powf(2.0,self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=(1.0/self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=p.p116;
        self.scalar_static_f64[52]=p.p117;
        self.scalar_static_f64[53]=(self.scalar_static_f64[8]*self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(self.scalar_static_f64[8]*self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=p.p118;
        self.scalar_static_f64[56]=(self.scalar_static_f64[8]+self.scalar_static_f64[55]);
        self.scalar_static_f64[57]=(self.scalar_static_f64[54]/self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=(self.scalar_static_f64[51]+self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(self.scalar_static_f64[58]-0.05);
        self.scalar_static_f64[60]=(self.scalar_static_f64[59]/0.1);
        self.scalar_static_bool[6]=(self.scalar_static_f64[58]<0.05);
        self.scalar_static_f64[61]=(self.scalar_static_f64[60]).exp();
        self.scalar_static_f64[62]=(1.0+self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(self.scalar_static_f64[62]).ln();
        self.scalar_static_f64[64]=(0.1*self.scalar_static_f64[63]);
        self.scalar_static_f64[65]=(0.05+self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(if self.scalar_static_bool[6]{self.scalar_static_f64[65]}else{0.0});
        self.scalar_static_bool[7]=(!self.scalar_static_bool[6]);
        self.scalar_static_f64[67]=(-self.scalar_static_f64[60]);
        self.scalar_static_f64[68]=(self.scalar_static_f64[67]).exp();
        self.scalar_static_f64[69]=(1.0+self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(self.scalar_static_f64[69]).ln();
        self.scalar_static_f64[71]=(0.1*self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=(self.scalar_static_f64[58]+self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=(if self.scalar_static_bool[7]{self.scalar_static_f64[72]}else{self.scalar_static_f64[66]});
        self.scalar_static_f64[74]=(1.0/self.scalar_static_f64[51]);
        self.scalar_static_f64[75]=(1.0/self.scalar_static_f64[46]);
        self.scalar_static_f64[76]=p.p82;
        self.scalar_static_f64[77]=(1.0/self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(1.0-self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=p.p124;
        self.scalar_static_f64[80]=(self.scalar_static_f64[8]*8.617086918058125e-5);
        self.scalar_static_f64[81]=(1.0/self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=p.p104;
        self.scalar_static_f64[83]=p.p63;
        self.scalar_static_f64[84]=p.p109;
        self.scalar_static_f64[85]=p.p79;
        self.scalar_static_f64[86]=p.p26;
        self.scalar_static_f64[87]=p.p108;
        self.scalar_static_f64[88]=p.p64;
        self.scalar_static_f64[89]=p.p74;
        self.scalar_static_f64[90]=(1.0-self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=p.p69;
        self.scalar_static_f64[92]=p.p53;
        self.scalar_static_f64[93]=p.p96;
        self.scalar_static_f64[94]=p.p55;
        self.scalar_static_f64[95]=p.p97;
        self.scalar_static_f64[96]=p.p95;
        self.scalar_static_f64[97]=(self.scalar_static_f64[95]-self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=p.p54;
        self.scalar_static_f64[99]=p.p100;
        self.scalar_static_f64[100]=p.p56;
        self.scalar_static_f64[101]=p.p101;
        self.scalar_static_f64[102]=p.p57;
        self.scalar_static_f64[103]=p.p103;
        self.scalar_static_f64[104]=p.p58;
        self.scalar_static_f64[105]=p.p59;
        self.scalar_static_f64[106]=p.p98;
        self.scalar_static_f64[107]=p.p121;
        self.scalar_static_bool[8]=(0.0!=self.scalar_static_f64[107]);
        self.scalar_static_f64[108]=p.p9;
        self.scalar_static_bool[9]=(!self.scalar_static_bool[8]);
        self.scalar_static_f64[109]=p.p122;
        self.scalar_static_bool[10]=(0.0!=self.scalar_static_f64[109]);
        self.scalar_static_f64[110]=p.p10;
        self.scalar_static_bool[11]=(!self.scalar_static_bool[10]);
        self.scalar_static_f64[111]=p.p42;
        self.scalar_static_f64[112]=p.p123;
        self.scalar_static_f64[113]=p.p8;
        self.scalar_static_f64[114]=(4.0-self.scalar_static_f64[95]);
        self.scalar_static_f64[115]=(self.scalar_static_f64[114]-self.scalar_static_f64[96]);
        self.scalar_static_f64[116]=p.p120;
        self.scalar_static_f64[117]=(self.scalar_static_f64[115]+self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(-self.scalar_static_f64[82]);
        self.scalar_static_f64[119]=p.p11;
        self.scalar_static_f64[120]=(1.0-self.scalar_static_f64[95]);
        self.scalar_static_f64[121]=p.p29;
        self.scalar_static_f64[122]=p.p102;
        self.scalar_static_f64[123]=(1.0-self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=p.p19;
        self.scalar_static_f64[125]=p.p20;
        self.scalar_static_f64[126]=(2.0*self.scalar_static_f64[125]);
        self.scalar_static_f64[127]=(6.0-self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=p.p112;
        self.scalar_static_f64[129]=(-self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=p.p30;
        self.scalar_static_f64[131]=p.p31;
        self.scalar_static_f64[132]=(2.0*self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(6.0-self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=(-self.scalar_static_f64[84]);
        self.scalar_static_f64[135]=p.p15;
        self.scalar_static_f64[136]=(4.0-self.scalar_static_f64[93]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[116]+self.scalar_static_f64[136]);
        self.scalar_static_f64[138]=p.p16;
        self.scalar_static_f64[139]=p.p110;
        self.scalar_static_f64[140]=(-self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=p.p17;
        self.scalar_static_f64[142]=p.p18;
        self.scalar_static_f64[143]=p.p23;
        self.scalar_static_bool[12]=(1.0==self.scalar_static_f64[143]);
        self.scalar_static_f64[144]=p.p24;
        self.scalar_static_f64[145]=p.p106;
        self.scalar_static_f64[146]=(-self.scalar_static_f64[145]);
        self.scalar_static_f64[147]=p.p27;
        self.scalar_static_f64[148]=p.p105;
        self.scalar_static_f64[149]=(-self.scalar_static_f64[148]);
        self.scalar_static_f64[150]=p.p25;
        self.scalar_static_f64[151]=p.p107;
        self.scalar_static_f64[152]=(-self.scalar_static_f64[151]);
        self.scalar_static_f64[153]=p.p28;
        self.scalar_static_f64[154]=(4.0-self.scalar_static_f64[122]);
        self.scalar_static_f64[155]=(self.scalar_static_f64[116]+self.scalar_static_f64[154]);
        self.scalar_static_f64[156]=p.p111;
        self.scalar_static_f64[157]=(-self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=p.p21;
        self.scalar_static_f64[159]=p.p22;
        self.scalar_static_f64[160]=(2.0*self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(6.0-self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=p.p136;
        self.scalar_static_f64[163]=p.p137;
        self.scalar_static_f64[164]=(4.0/self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=p.p142;
        self.scalar_static_f64[166]=p.p144;
        self.scalar_static_f64[167]=p.p34;
        self.scalar_static_f64[168]=p.p33;
        self.scalar_static_f64[169]=p.p36;
        self.scalar_static_f64[170]=p.p35;
        self.scalar_static_f64[171]=p.p13;
        self.scalar_static_f64[172]=p.p12;
        self.scalar_static_f64[173]=p.p85;
        self.scalar_static_f64[174]=(self.scalar_static_f64[95]-2.0);
        self.scalar_static_f64[175]=p.p119;
        self.scalar_static_f64[176]=(-self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=p.p86;
        self.scalar_static_f64[178]=(self.scalar_static_f64[95]+self.scalar_static_f64[96]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[178]-1.0);
        self.scalar_static_f64[180]=p.p87;
        self.scalar_static_f64[181]=(self.scalar_static_f64[106]-1.0);
        self.scalar_static_f64[182]=p.p88;
        self.scalar_static_f64[183]=(self.scalar_static_f64[177]+self.scalar_static_f64[180]);
        self.scalar_static_f64[184]=p.p89;
        self.scalar_static_f64[185]=p.p99;
        self.scalar_static_f64[186]=(self.scalar_static_f64[185]-1.0);
        self.scalar_static_f64[187]=(self.scalar_static_f64[4]*1.081);
        self.scalar_static_f64[188]=p.p91;
        self.scalar_static_f64[189]=p.p133;
        self.scalar_static_f64[190]=p.p135;
        self.scalar_static_bool[13]=(self.scalar_static_f64[100]>0.0);
        self.scalar_static_bool[14]=(!self.scalar_static_bool[13]);
        self.scalar_static_bool[15]=(self.scalar_static_f64[102]>0.0);
        self.scalar_static_bool[16]=(!self.scalar_static_bool[15]);
        self.scalar_static_bool[17]=(self.scalar_static_f64[104]>0.0);
        self.scalar_static_bool[18]=(!self.scalar_static_bool[17]);
        self.scalar_static_f64[191]=p.p138;
        self.scalar_static_f64[192]=(self.scalar_static_f64[191]).exp();
        self.scalar_static_f64[193]=p.p140;
        self.scalar_static_f64[194]=p.p61;
        self.scalar_static_f64[195]=p.p60;
        self.scalar_static_f64[196]=(self.scalar_static_f64[194]*self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=p.p62;
        self.scalar_static_f64[198]=(-1.0/self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=(self.scalar_static_f64[198]).exp();
        self.scalar_static_f64[200]=(1.0+self.scalar_static_f64[199]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[200]).ln();
        self.scalar_static_f64[202]=(self.scalar_static_f64[197]*self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=(1.0+self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=p.p139;
        self.scalar_static_f64[205]=(0.5*self.scalar_static_f64[195]);
        self.scalar_static_f64[206]=p.p72;
        self.scalar_static_bool[19]=(0.0==self.scalar_static_f64[206]);
        self.scalar_static_bool[20]=(!self.scalar_static_bool[19]);
        self.scalar_static_f64[207]=(-1.0/self.scalar_static_f64[16]);
        self.scalar_static_f64[208]=f64::powf(3.0,self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=(1.0-self.scalar_static_f64[208]);
        self.scalar_static_f64[210]=(1.0-self.scalar_static_f64[16]);
        self.scalar_static_f64[211]=p.p73;
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[211]);
        self.scalar_static_bool[22]=(2.0==self.scalar_static_f64[211]);
        self.scalar_static_bool[23]=(!self.scalar_static_bool[21]);
        self.scalar_static_bool[24]=(self.scalar_static_bool[22]&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=(!self.scalar_static_bool[22]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[23]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[212]=(-1.0/self.scalar_static_f64[47]);
        self.scalar_static_f64[213]=p.p75;
        self.scalar_static_f64[214]=(1.0-self.scalar_static_f64[47]);
        self.scalar_static_bool[27]=(0.0==self.scalar_static_f64[188]);
        self.scalar_static_bool[28]=(!self.scalar_static_bool[27]);
        self.scalar_static_f64[215]=p.p14;
        self.scalar_static_f64[216]=p.p143;
        self.scalar_static_f64[217]=p.p145;
        self.scalar_static_f64[218]=p.p146;
        self.scalar_static_f64[219]=p.p92;
        self.scalar_static_bool[29]=(0.0==self.scalar_static_f64[219]);
        self.scalar_static_bool[30]=(!self.scalar_static_bool[12]);
        self.scalar_static_bool[31]=(self.scalar_static_bool[29]&&self.scalar_static_bool[30]);
        self.scalar_static_bool[32]=(!self.scalar_static_bool[29]);
        self.scalar_static_bool[33]=(self.scalar_static_bool[30]&&self.scalar_static_bool[32]);
        self.scalar_static_f64[220]=(1.0-self.scalar_static_f64[219]);
        self.scalar_static_bool[34]=(self.scalar_static_f64[168]>0.0);
        self.scalar_static_bool[35]=(self.scalar_static_f64[167]>0.0);
        self.scalar_static_bool[36]=(self.scalar_static_bool[34]&&self.scalar_static_bool[35]);
        self.scalar_static_f64[221]=(-2.0-self.scalar_static_f64[16]);
        self.scalar_static_f64[222]=(self.scalar_static_f64[16]*self.scalar_static_f64[16]);
        self.scalar_static_f64[223]=(1.0-self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[16]-1.0);
        self.scalar_static_bool[37]=(self.scalar_static_f64[170]>0.0);
        self.scalar_static_bool[38]=(self.scalar_static_f64[169]>0.0);
        self.scalar_static_bool[39]=(self.scalar_static_bool[37]&&self.scalar_static_bool[38]);
        self.scalar_static_f64[225]=(-2.0-self.scalar_static_f64[47]);
        self.scalar_static_f64[226]=(self.scalar_static_f64[47]*self.scalar_static_f64[47]);
        self.scalar_static_f64[227]=(1.0-self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=(self.scalar_static_f64[47]-1.0);
        self.scalar_static_f64[229]=p.p5;
        self.scalar_static_bool[40]=(self.scalar_static_f64[229]>0.0);
        self.scalar_static_bool[41]=(self.scalar_static_f64[5]>0.0);
        self.scalar_static_bool[42]=(self.scalar_static_bool[40]&&self.scalar_static_bool[41]);
        self.scalar_static_f64[230]=(self.scalar_static_f64[5]*2.0);
        self.scalar_static_bool[43]=(1.0==self.scalar_static_f64[229]);
        self.scalar_static_bool[44]=(self.scalar_static_bool[42]&&self.scalar_static_bool[43]);
        self.scalar_static_f64[231]=(if self.scalar_static_bool[44]{0.0121}else{0.010000000000000002});
        self.scalar_static_f64[232]=(0.5*self.scalar_static_f64[231]);
        self.scalar_static_bool[45]=(!self.scalar_static_bool[43]);
        self.scalar_static_bool[46]=(self.scalar_static_bool[42]&&self.scalar_static_bool[45]);
        self.scalar_static_f64[233]=p.p83;
        self.scalar_static_bool[47]=(1.0==self.scalar_static_f64[233]);
        self.scalar_static_f64[234]=(if self.scalar_static_bool[47]{1e-12}else{self.scalar_static_f64[231]});
        self.scalar_static_f64[235]=(0.5*self.scalar_static_f64[234]);
        self.scalar_static_f64[236]=p.p81;
        self.scalar_static_f64[237]=f64::powf(self.scalar_static_f64[78],self.scalar_static_f64[236]);
        self.scalar_static_f64[238]=(1.0-self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=(1.0/self.scalar_static_f64[238]);
        self.scalar_static_f64[240]=(if self.scalar_static_bool[47]{self.scalar_static_f64[239]}else{0.0});
        self.scalar_static_f64[241]=p.p80;
        self.scalar_static_f64[242]=(self.scalar_static_f64[78]*self.scalar_static_f64[241]);
        self.scalar_static_f64[243]=(if self.scalar_static_bool[47]{self.scalar_static_f64[242]}else{0.0});
        self.scalar_static_f64[244]=(self.scalar_static_f64[240]*self.scalar_static_f64[240]);
        self.scalar_static_f64[245]=(self.scalar_static_f64[236]-1.0);
        self.scalar_static_f64[246]=f64::powf(self.scalar_static_f64[78],self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=(self.scalar_static_f64[244]*self.scalar_static_f64[246]);
        self.scalar_static_f64[248]=(self.scalar_static_f64[236]*self.scalar_static_f64[247]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]/self.scalar_static_f64[241]);
        self.scalar_static_f64[250]=(if self.scalar_static_bool[47]{self.scalar_static_f64[249]}else{0.0});
        self.scalar_static_bool[48]=(!self.scalar_static_bool[47]);
        self.scalar_static_f64[251]=p.p38;
        self.scalar_static_bool[49]=(1.0==self.scalar_static_f64[251]);
        self.scalar_static_f64[252]=p.p43;
        self.scalar_static_f64[253]=p.p41;
        self.scalar_static_f64[254]=p.p40;
        self.scalar_static_f64[255]=p.p39;
        self.scalar_static_bool[50]=(2.0==self.scalar_static_f64[251]);
        self.scalar_static_bool[51]=(!self.scalar_static_bool[49]);
        self.scalar_static_f64[256]=p.p45;
        self.scalar_static_f64[257]=(2.0*self.scalar_static_f64[256]);
        self.scalar_static_f64[258]=p.p44;
        self.scalar_static_f64[259]=(self.scalar_static_f64[258]*self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[257]/self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=p.p7;
        self.scalar_static_bool[52]=(0.0==self.scalar_static_f64[261]);
        self.scalar_static_bool[53]=(!self.scalar_static_bool[52]);
        self.scalar_static_f64[262]=p.p46;
        self.scalar_static_f64[263]=(2.0*self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=(1.0+self.scalar_static_f64[262]);
        self.scalar_static_f64[265]=(1.0+self.scalar_static_f64[263]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[264]/self.scalar_static_f64[265]);
        self.scalar_static_bool[54]=(3.0==self.scalar_static_f64[251]);
        self.scalar_static_bool[55]=(!self.scalar_static_bool[50]);
        self.scalar_static_f64[267]=p.p47;
        self.scalar_static_f64[268]=p.p48;
        self.scalar_static_f64[269]=p.p51;
        self.scalar_static_f64[270]=p.p50;
        self.scalar_static_f64[271]=p.p49;
        self.scalar_static_f64[272]=p.p52;
        self.scalar_static_bool[56]=(1.0==self.scalar_static_f64[272]);
        self.scalar_static_bool[57]=(!self.scalar_static_bool[54]);
        self.scalar_static_bool[58]=(!self.scalar_static_bool[56]);
        self.scalar_static_f64[273]=p.p67;
        self.scalar_static_f64[274]=(1.0-self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=p.p76;
        self.scalar_static_f64[276]=(1.0-self.scalar_static_f64[275]);
        self.scalar_static_f64[277]=p.p84;
        self.scalar_static_f64[278]=(1.0/self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=p.p78;
        self.scalar_static_bool[59]=(0.0==self.scalar_static_f64[279]);
        self.scalar_static_f64[280]=p.p90;
        self.scalar_static_bool[60]=(!self.scalar_static_bool[59]);
        self.scalar_static_bool[61]=(3.0==self.scalar_static_f64[229]);
        self.scalar_static_bool[62]=(self.scalar_static_bool[43]||self.scalar_static_bool[61]);
        self.scalar_static_bool[63]=(self.scalar_static_bool[41]&&self.scalar_static_bool[62]);
        self.scalar_static_bool[64]=(self.scalar_static_bool[59]&&self.scalar_static_bool[63]);
        self.scalar_static_f64[281]=(self.scalar_static_f64[5]*0.5);
        self.scalar_static_bool[65]=(self.scalar_static_bool[60]&&self.scalar_static_bool[63]);
        self.scalar_static_f64[282]=p.p6;
        self.scalar_static_bool[66]=(1.0==self.scalar_static_f64[282]);
        self.scalar_static_f64[283]=(-self.scalar_static_f64[16]);
        self.scalar_static_f64[284]=p.p94;
        self.scalar_static_f64[285]=(1.0-self.scalar_static_f64[284]);
        self.scalar_static_f64[286]=p.p93;
        self.scalar_static_f64[287]=(1.0-self.scalar_static_f64[286]);
        self.scalar_static_bool[67]=(!self.scalar_static_bool[66]);
        self.scalar_static_f64[288]=p.p134;
        self.scalar_static_f64[289]=(1.0-self.scalar_static_f64[190]);
        self.scalar_static_bool[68]=(self.scalar_static_f64[189]>self.scalar_static_f64[14]);
        self.scalar_static_f64[290]=p.p132;
        self.scalar_static_bool[69]=(0.0==self.scalar_static_f64[290]);
        self.scalar_static_bool[70]=(self.scalar_static_bool[68]&&self.scalar_static_bool[69]);
        self.scalar_static_f64[291]=(self.scalar_static_f64[289]).abs();
        self.scalar_static_bool[71]=(self.scalar_static_f64[291]<1e-6);
        self.scalar_static_bool[72]=(!self.scalar_static_bool[69]);
        self.scalar_static_bool[73]=(self.scalar_static_bool[68]&&self.scalar_static_bool[72]);
        self.scalar_static_bool[74]=(self.scalar_static_bool[71]&&self.scalar_static_bool[73]);
        self.scalar_static_bool[75]=(!self.scalar_static_bool[71]);
        self.scalar_static_bool[76]=(self.scalar_static_bool[73]&&self.scalar_static_bool[75]);
        self.scalar_static_bool[77]=(!self.scalar_static_bool[68]);
        self.scalar_static_f64[292]=p.p129;
        self.scalar_static_bool[78]=(self.scalar_static_f64[292]>0.0);
        self.scalar_static_bool[79]=(!self.scalar_static_bool[78]);
        self.scalar_static_f64[293]=p.p130;
        self.scalar_static_bool[80]=(1.0==self.scalar_static_f64[293]);
        self.scalar_static_bool[81]=(2.0==self.scalar_static_f64[293]);
        self.scalar_static_bool[82]=(!self.scalar_static_bool[80]);
        self.scalar_static_bool[83]=(self.scalar_static_bool[81]&&self.scalar_static_bool[82]);
        self.scalar_static_f64[294]=p.p131;
        self.scalar_static_bool[84]=(!self.scalar_static_bool[81]);
        self.scalar_static_bool[85]=(self.scalar_static_bool[82]&&self.scalar_static_bool[84]);
        self.scalar_static_f64[295]=p.p68;
        self.scalar_static_f64[296]=p.p77;
        self.scalar_static_f64[297]=(self.scalar_static_f64[0]*self.scalar_static_f64[295]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[0]*self.scalar_static_f64[296]);
        self.scalar_static_f64[299]=(-self.scalar_static_f64[0]);
        self.scalar_static_f64[300]=(self.scalar_static_f64[0]+self.scalar_static_f64[299]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[299]-self.scalar_static_f64[299]);
        self.scalar_static_f64[302]=(self.scalar_static_f64[0]+self.scalar_static_f64[300]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[210]-1.0);
        self.scalar_static_f64[304]=(if self.scalar_static_bool[21]{self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[305]=(if self.scalar_static_bool[21]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[306]=(self.scalar_static_f64[212]-1.0);
        self.scalar_static_f64[307]=(self.scalar_static_f64[213]-1.0);
        self.scalar_static_f64[308]=(self.scalar_static_f64[214]-1.0);
        self.scalar_static_f64[309]=(self.scalar_static_f64[299]/0.0001);
        self.scalar_static_f64[310]=(self.scalar_static_f64[0]/0.0001);
        self.scalar_static_f64[311]=(-self.scalar_static_f64[309]);
        self.scalar_static_f64[312]=(-self.scalar_static_f64[310]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[299]/0.001);
        self.scalar_static_f64[314]=(self.scalar_static_f64[0]/0.001);
        self.scalar_static_f64[315]=(-self.scalar_static_f64[313]);
        self.scalar_static_f64[316]=(-self.scalar_static_f64[314]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[221]-1.0);
        self.scalar_static_f64[318]=(self.scalar_static_f64[18]*self.scalar_static_f64[299]);
        self.scalar_static_f64[319]=(self.scalar_static_f64[0]*self.scalar_static_f64[18]);
        self.scalar_static_f64[320]=(0.5*self.scalar_static_f64[299]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[0]*0.5);
        self.scalar_static_f64[322]=(self.scalar_static_f64[225]-1.0);
        self.scalar_static_f64[323]=(self.scalar_static_f64[0]*self.scalar_static_f64[49]);
        self.scalar_static_f64[324]=(self.scalar_static_f64[49]*self.scalar_static_f64[299]);
        self.scalar_static_f64[325]=(if self.scalar_static_bool[44]{self.scalar_static_f64[300]}else{0.0});
        self.scalar_static_f64[326]=(if self.scalar_static_bool[44]{self.scalar_static_f64[302]}else{0.0});
        self.scalar_static_f64[327]=(if self.scalar_static_bool[44]{self.scalar_static_f64[301]}else{0.0});
        self.scalar_static_f64[328]=(if self.scalar_static_bool[44]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[329]=(if self.scalar_static_bool[47]{self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[330]=(if self.scalar_static_bool[47]{self.scalar_static_f64[300]}else{0.0});
        self.scalar_static_f64[331]=(if self.scalar_static_bool[47]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[332]=(-self.scalar_static_f64[329]);
        self.scalar_static_f64[333]=(-self.scalar_static_f64[330]);
        self.scalar_static_f64[334]=(-self.scalar_static_f64[331]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[254]-1.0);
        self.scalar_static_f64[336]=(self.scalar_static_f64[268]-1.0);
        self.scalar_static_f64[337]=(self.scalar_static_f64[271]-1.0);
        self.scalar_static_f64[338]=(if self.scalar_static_bool[12]{self.scalar_static_f64[0]}else{0.0});
        self.scalar_static_f64[339]=(if self.scalar_static_bool[12]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[340]=(if self.scalar_static_bool[30]{self.scalar_static_f64[0]}else{self.scalar_static_f64[338]});
        self.scalar_static_f64[341]=(if self.scalar_static_bool[30]{0.0}else{self.scalar_static_f64[339]});
        self.scalar_static_f64[342]=(if self.scalar_static_bool[30]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[343]=(0.0*self.scalar_static_f64[299]);
        self.scalar_static_f64[344]=(self.scalar_static_f64[0]*0.0);
        self.scalar_static_f64[345]=(0.0*self.scalar_static_f64[300]);
        self.scalar_static_f64[346]=(0.0*self.scalar_static_f64[301]);
        self.scalar_static_f64[347]=(self.scalar_static_f64[278]-1.0);
        self.scalar_static_f64[348]=(self.scalar_static_f64[0]/self.scalar_static_f64[280]);
        self.scalar_static_f64[349]=(self.scalar_static_f64[300]/self.scalar_static_f64[280]);
        self.scalar_static_f64[350]=(self.scalar_static_f64[301]/self.scalar_static_f64[280]);
        self.scalar_static_f64[351]=(self.scalar_static_f64[299]/self.scalar_static_f64[280]);
        self.scalar_static_f64[352]=(self.scalar_static_f64[283]-1.0);
        self.scalar_static_f64[353]=(self.scalar_static_f64[0]*0.2);
        self.scalar_static_f64[354]=(0.2*self.scalar_static_f64[299]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[289]-1.0);
        self.scalar_static_f64[356]=(1.0/self.scalar_static_f64[12]);
        self.scalar_static_f64[357]=(self.scalar_static_f64[0]*self.scalar_static_f64[0]);
        self.scalar_static_f64[358]=(self.scalar_static_f64[0]*self.scalar_static_f64[299]);
        self.scalar_static_f64[359]=(self.scalar_static_f64[0]*self.scalar_static_f64[297]);
        self.scalar_static_f64[360]=(self.scalar_static_f64[297]*self.scalar_static_f64[299]);
        self.scalar_static_f64[361]=(self.scalar_static_f64[298]*self.scalar_static_f64[299]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[0]*self.scalar_static_f64[298]);
        self.scalar_static_f64[363]=(self.scalar_static_f64[0]*self.scalar_static_f64[300]);
        self.scalar_static_f64[364]=(self.scalar_static_f64[0]*self.scalar_static_f64[301]);
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
        self.scalar_static_f64[365]=(temperature+self.scalar_static_f64[9]);
        self.scalar_static_f64[366]=(self.scalar_static_f64[365]/self.scalar_static_f64[8]);
        self.scalar_static_f64[367]=f64::powf(self.scalar_static_f64[366],self.scalar_static_f64[190]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[189]*self.scalar_static_f64[367]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[365]/self.scalar_static_f64[368]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[13]*self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[368]*self.scalar_static_f64[289]);
        self.scalar_static_f64[372]=(self.scalar_static_f64[365]/self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=(self.scalar_static_f64[13]*self.scalar_static_f64[372]);
        self.scalar_static_f64[374]=(1.0/self.scalar_static_f64[368]);
        self.scalar_static_f64[375]=(self.scalar_static_f64[13]*self.scalar_static_f64[374]);
        self.scalar_static_f64[376]=(if self.scalar_static_bool[70]{self.scalar_static_f64[375]}else{0.0});
        self.scalar_static_f64[377]=(1.0/self.scalar_static_f64[365]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
