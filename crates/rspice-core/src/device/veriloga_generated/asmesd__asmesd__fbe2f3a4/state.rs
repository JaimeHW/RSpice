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
            params.p0 = 1e-17;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 5.0;
            params.p4 = 10.0;
            params.p5 = 10.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.01;
            params.p9 = 1.11;
            params.p10 = 0.0;
            params.p11 = 10.0;
            params.p12 = 1e-5;
            params.p13 = 0.0;
            params.p14 = 1e-6;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.75;
            params.p18 = 0.33;
            params.p19 = 0.0;
            params.p20 = 0.001;
            params.p21 = 1.11;
            params.p22 = 3.0;
            params.p23 = 0.5;
            params.p24 = 0.5;
            params.p25 = 25.0;
            params.p26 = 1000.0;
            params.p27 = 0.0;
            params.p28 = 1.0;
            params.p29 = 1.0;
            params.p30 = 2.0;
            params.p31 = 0.0;
            params.p32 = 1.0;
            params.p33 = 0.0005;
            params.p34 = 0.0005;
            params.p35 = 5e-6;
            params.p36 = 1e-7;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 2.0;
            params.p40 = 100.0;
            params.p41 = 0.0;
            params.p42 = 1e-5;
            params.p43 = 1.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.001;
            validate_parameter("minr", params.p46, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p47 = 5.0;
            params.p48 = 100.0;
            params.p49 = 2.0;
            params.p50 = 100.0;
            params.p51 = 2.0;
            params.p52 = 0.1;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 5.0;
            params.p56 = 0.0;
            params.p57 = 20.0;
            params.p58 = 0.0;
            params.p59 = 1.5;
            params.p60 = 1.0;
            params.p61 = 10.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 2.0;
            params.p66 = 1e-6;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.75;
            params.p71 = 0.33;
            params.p72 = 1.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 0.75;
            params.p76 = 0.33;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 1.0;
            params.p81 = 0.0;
            params.p82 = 0.9;
            params.p83 = 1e-8;
            params.p84 = 0.0;
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
    pub branches: [usize; 8],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 85]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 13]>,
    pub(crate) ddt_state_previous: Box<[f64; 13]>,
    pub(crate) ddt_state_older: Box<[f64; 13]>,
    pub(crate) ddt_state_initialized: Box<[bool; 13]>,
    pub(crate) ddt_derivative_current: Box<[f64; 13]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 13]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 156]>,
    pub(crate) scalar_static_bool: Box<[bool; 32]>,
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
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 6;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 6] = ["ci", "bi", "ei", "dt1", "tt", "tbb"];

    pub const BRANCH_COUNT: usize = 8;
    pub const PARAMETER_COUNT: usize = 85;
    pub const VARIABLE_COUNT: usize = 128;
    pub const DDT_STATE_COUNT: usize = 13;
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
            scalar_static_f64: boxed_zero_f64_array::<156>(),
            scalar_static_bool: boxed_zero_bool_array::<32>(),
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
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "isr" => { validate_parameter("isr", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "ntr" => { validate_parameter("ntr", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "vtr" => { validate_parameter("vtr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "bvr" => { validate_parameter("bvr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "xbvr" => { validate_finite_parameter("xbvr", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "xjbv" => { validate_parameter("xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "ther" => { validate_parameter("ther", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "theexp" => { validate_parameter("theexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "xtheexp" => { validate_finite_parameter("xtheexp", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "rb" => { validate_parameter("rb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "rbe" => { validate_parameter("rbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ree" => { validate_parameter("ree", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "vje" => { validate_parameter("vje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "mje" => { validate_parameter("mje", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "qtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "vtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "eg" => { validate_parameter("eg", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "xtir" => { validate_parameter("xtir", value, Some((-20.0, "-20.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-40.0, "-40.0")), false, Some((125.0, "125.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "tfail" => { validate_parameter("tfail", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "extmod" => { validate_parameter("extmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "rbmod" => { validate_parameter("rbmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "rth1" => { validate_parameter("rth1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cth1" => { validate_parameter("cth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "arb" => { validate_parameter("arb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "are" => { validate_parameter("are", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "texp" => { validate_parameter("texp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "vtf0" => { validate_parameter("vtf0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "atff" => { validate_parameter("atff", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "n" => { validate_parameter("n", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "qexp" => { validate_parameter("qexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ijbv" => { validate_parameter("ijbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "vsatb" => { validate_parameter("vsatb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "mexp" => { validate_parameter("mexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "vsate" => { validate_parameter("vsate", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "mexpe" => { validate_parameter("mexpe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "bf" => { validate_parameter("bf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "vaf" => { validate_parameter("vaf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "ikf" => { validate_parameter("ikf", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "xjbvc" => { validate_parameter("xjbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "ijbvc" => { validate_parameter("ijbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "nbvc" => { validate_parameter("nbvc", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "ise" => { validate_parameter("ise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "ne" => { validate_parameter("ne", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "br" => { validate_parameter("br", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "nr" => { validate_parameter("nr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "var" => { validate_parameter("var", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "ikr" => { validate_parameter("ikr", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "isc" => { validate_parameter("isc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "nc" => { validate_parameter("nc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "rc" => { validate_parameter("rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "rce" => { validate_parameter("rce", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "ptf" => { validate_parameter("ptf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "vjc" => { validate_parameter("vjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "mjc" => { validate_parameter("mjc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "vjs" => { validate_parameter("vjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "mjs" => { validate_parameter("mjs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "xtb" => { validate_parameter("xtb", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "arc" => { validate_parameter("arc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "kbwm" => { validate_parameter("kbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "xbwm" => { validate_parameter("xbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "ikbwm" => { validate_parameter("ikbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "xkf" => { validate_parameter("xkf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "cthbb" => { validate_parameter("cthbb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "cdelay" => { validate_parameter("cdelay", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmesd'", name)),
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
        self.scalar_static_f64[0]=p.p45;
        self.scalar_static_f64[1]=p.p43;
        self.scalar_static_f64[2]=p.p42;
        self.scalar_static_f64[3]=(self.scalar_static_f64[1]*self.scalar_static_f64[2]);
        self.scalar_static_f64[4]=p.p29;
        self.scalar_static_f64[5]=p.p79;
        self.scalar_static_f64[6]=p.p80;
        self.scalar_static_f64[7]=p.p25;
        self.scalar_static_f64[8]=(273.15+self.scalar_static_f64[7]);
        self.scalar_static_f64[9]=p.p77;
        self.scalar_static_f64[10]=p.p52;
        self.scalar_static_f64[11]=p.p60;
        self.scalar_static_f64[12]=p.p53;
        self.scalar_static_bool[0]=(self.scalar_static_f64[12]>0.0);
        self.scalar_static_f64[13]=(1.0/self.scalar_static_f64[12]);
        self.scalar_static_f64[14]=(if self.scalar_static_bool[0]{self.scalar_static_f64[13]}else{0.0});
        self.scalar_static_f64[15]=p.p62;
        self.scalar_static_bool[1]=(self.scalar_static_f64[15]>0.0);
        self.scalar_static_f64[16]=(1.0/self.scalar_static_f64[15]);
        self.scalar_static_f64[17]=(if self.scalar_static_bool[1]{self.scalar_static_f64[16]}else{0.0});
        self.scalar_static_f64[18]=p.p54;
        self.scalar_static_bool[2]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_f64[19]=(1.0/self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=(if self.scalar_static_bool[2]{self.scalar_static_f64[19]}else{0.0});
        self.scalar_static_f64[21]=p.p63;
        self.scalar_static_bool[3]=(self.scalar_static_f64[21]>0.0);
        self.scalar_static_f64[22]=(1.0/self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[3]{self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=p.p22;
        self.scalar_static_f64[25]=p.p21;
        self.scalar_static_f64[26]=p.p23;
        self.scalar_static_f64[27]=p.p0;
        self.scalar_static_f64[28]=p.p2;
        self.scalar_static_f64[29]=p.p58;
        self.scalar_static_f64[30]=p.p59;
        self.scalar_static_f64[31]=p.p64;
        self.scalar_static_f64[32]=p.p65;
        self.scalar_static_f64[33]=p.p47;
        self.scalar_static_f64[34]=p.p7;
        self.scalar_static_f64[35]=p.p5;
        self.scalar_static_f64[36]=p.p6;
        self.scalar_static_f64[37]=p.p9;
        self.scalar_static_f64[38]=p.p10;
        self.scalar_static_f64[39]=p.p56;
        self.scalar_static_f64[40]=p.p55;
        self.scalar_static_f64[41]=p.p16;
        self.scalar_static_f64[42]=p.p69;
        self.scalar_static_f64[43]=p.p74;
        self.scalar_static_f64[44]=(self.scalar_static_f64[8]/300.15);
        self.scalar_static_f64[45]=p.p17;
        self.scalar_static_f64[46]=p.p18;
        self.scalar_static_f64[47]=(self.scalar_static_f64[8]-300.15);
        self.scalar_static_f64[48]=(0.0004*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p70;
        self.scalar_static_f64[50]=p.p71;
        self.scalar_static_f64[51]=p.p75;
        self.scalar_static_f64[52]=p.p76;
        self.scalar_static_f64[53]=p.p1;
        self.scalar_static_f64[54]=p.p11;
        self.scalar_static_f64[55]=p.p8;
        self.scalar_static_f64[56]=p.p4;
        self.scalar_static_f64[57]=p.p3;
        self.scalar_static_f64[58]=p.p57;
        self.scalar_static_f64[59]=p.p61;
        self.scalar_static_f64[60]=p.p81;
        self.scalar_static_f64[61]=p.p82;
        self.scalar_static_f64[62]=p.p84;
        self.scalar_static_f64[63]=(1.0-self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=p.p48;
        self.scalar_static_f64[65]=p.p49;
        self.scalar_static_f64[66]=p.p50;
        self.scalar_static_f64[67]=p.p51;
        self.scalar_static_f64[68]=p.p12;
        self.scalar_static_f64[69]=p.p37;
        self.scalar_static_f64[70]=(1.0/self.scalar_static_f64[65]);
        self.scalar_static_f64[71]=p.p66;
        self.scalar_static_f64[72]=p.p78;
        self.scalar_static_f64[73]=p.p14;
        self.scalar_static_f64[74]=p.p38;
        self.scalar_static_f64[75]=(1.0/self.scalar_static_f64[67]);
        self.scalar_static_f64[76]=p.p40;
        self.scalar_static_f64[77]=p.p39;
        self.scalar_static_f64[78]=(1.0/self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=p.p19;
        self.scalar_static_f64[80]=p.p41;
        self.scalar_static_f64[81]=p.p73;
        self.scalar_static_f64[82]=p.p32;
        self.scalar_static_bool[4]=(1.0==self.scalar_static_f64[82]);
        self.scalar_static_f64[83]=p.p20;
        self.scalar_static_f64[84]=p.p44;
        self.scalar_static_f64[85]=p.p31;
        self.scalar_static_bool[5]=(1.0==self.scalar_static_f64[85]);
        self.scalar_static_f64[86]=p.p13;
        self.scalar_static_f64[87]=p.p67;
        self.scalar_static_f64[88]=p.p15;
        self.scalar_static_f64[89]=(1.0-self.scalar_static_f64[52]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[52]*0.5);
        self.scalar_static_f64[91]=p.p24;
        self.scalar_static_f64[92]=(-1.0-self.scalar_static_f64[46]);
        self.scalar_static_f64[93]=(1.0-self.scalar_static_f64[91]);
        self.scalar_static_f64[94]=(self.scalar_static_f64[93]).ln();
        self.scalar_static_f64[95]=(self.scalar_static_f64[92]*self.scalar_static_f64[94]);
        self.scalar_static_f64[96]=(self.scalar_static_f64[95]).exp();
        self.scalar_static_f64[97]=(1.0-self.scalar_static_f64[46]);
        self.scalar_static_f64[98]=(self.scalar_static_f64[46]*0.5);
        self.scalar_static_f64[99]=(-1.0-self.scalar_static_f64[50]);
        self.scalar_static_f64[100]=(self.scalar_static_f64[94]*self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=(self.scalar_static_f64[100]).exp();
        self.scalar_static_f64[102]=(1.0-self.scalar_static_f64[50]);
        self.scalar_static_f64[103]=(self.scalar_static_f64[50]*0.5);
        self.scalar_static_f64[104]=p.p72;
        self.scalar_static_f64[105]=(1.0-self.scalar_static_f64[104]);
        self.scalar_static_f64[106]=p.p68;
        self.scalar_static_bool[6]=(0.0!=self.scalar_static_f64[106]);
        self.scalar_static_bool[7]=(0.0!=self.scalar_static_f64[79]);
        self.scalar_static_bool[8]=(self.scalar_static_bool[6]&&self.scalar_static_bool[7]);
        self.scalar_static_f64[107]=(self.scalar_static_f64[4]*self.scalar_static_f64[106]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[107]*3.141592653589793);
        self.scalar_static_f64[109]=(self.scalar_static_f64[108]/180.0);
        self.scalar_static_f64[110]=(self.scalar_static_f64[79]*self.scalar_static_f64[109]);
        self.scalar_static_bool[9]=(!self.scalar_static_bool[8]);
        self.scalar_static_f64[111]=p.p30;
        self.scalar_static_bool[10]=(1.0==self.scalar_static_f64[111]);
        self.scalar_static_f64[112]=p.p33;
        self.scalar_static_bool[11]=(self.scalar_static_f64[112]>0.0);
        self.scalar_static_bool[12]=(self.scalar_static_bool[10]&&self.scalar_static_bool[11]);
        self.scalar_static_bool[13]=(2.0==self.scalar_static_f64[111]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[11]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[113]=p.p35;
        self.scalar_static_bool[15]=(self.scalar_static_f64[113]>0.0);
        self.scalar_static_bool[16]=(self.scalar_static_bool[14]&&self.scalar_static_bool[15]);
        self.scalar_static_bool[17]=(-1.0==self.scalar_static_f64[111]);
        self.scalar_static_f64[114]=(self.scalar_static_f64[85]*self.scalar_static_f64[86]);
        self.scalar_static_f64[115]=(self.scalar_static_f64[68]+self.scalar_static_f64[114]);
        self.scalar_static_f64[116]=(self.scalar_static_f64[115]/self.scalar_static_f64[3]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[85]*self.scalar_static_f64[88]);
        self.scalar_static_f64[118]=(self.scalar_static_f64[73]+self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=(self.scalar_static_f64[118]/self.scalar_static_f64[3]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[85]*self.scalar_static_f64[87]);
        self.scalar_static_f64[121]=(self.scalar_static_f64[71]+self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[121]/self.scalar_static_f64[3]);
        self.scalar_static_bool[18]=(self.scalar_static_f64[116]>0.0);
        self.scalar_static_f64[123]=p.p46;
        self.scalar_static_bool[19]=(self.scalar_static_f64[116]>=self.scalar_static_f64[123]);
        self.scalar_static_bool[20]=(self.scalar_static_bool[18]&&self.scalar_static_bool[19]);
        self.scalar_static_bool[21]=(self.scalar_static_f64[119]>0.0);
        self.scalar_static_bool[22]=(self.scalar_static_f64[119]>=self.scalar_static_f64[123]);
        self.scalar_static_bool[23]=(self.scalar_static_bool[21]&&self.scalar_static_bool[22]);
        self.scalar_static_bool[24]=(self.scalar_static_f64[122]>0.0);
        self.scalar_static_bool[25]=(self.scalar_static_f64[122]>=self.scalar_static_f64[123]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[24]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[124]=p.p83;
        self.scalar_static_f64[125]=p.p34;
        self.scalar_static_bool[27]=(!self.scalar_static_bool[12]);
        self.scalar_static_bool[28]=(self.scalar_static_bool[16]&&self.scalar_static_bool[27]);
        self.scalar_static_f64[126]=p.p36;
        self.scalar_static_bool[29]=(!self.scalar_static_bool[16]);
        self.scalar_static_bool[30]=(self.scalar_static_bool[27]&&self.scalar_static_bool[29]);
        self.scalar_static_bool[31]=(self.scalar_static_bool[17]&&self.scalar_static_bool[30]);
        self.scalar_static_f64[127]=(-self.scalar_static_f64[4]);
        self.scalar_static_f64[128]=(self.scalar_static_f64[6]-1.0);
        self.scalar_static_f64[129]=(self.scalar_static_f64[56]*self.scalar_static_f64[127]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[4]*self.scalar_static_f64[56]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[4]*self.scalar_static_f64[17]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[17]*self.scalar_static_f64[127]);
        self.scalar_static_f64[133]=(-self.scalar_static_f64[131]);
        self.scalar_static_f64[134]=(-self.scalar_static_f64[132]);
        self.scalar_static_f64[135]=(self.scalar_static_f64[14]*self.scalar_static_f64[127]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[4]*self.scalar_static_f64[14]);
        self.scalar_static_f64[137]=(-self.scalar_static_f64[135]);
        self.scalar_static_f64[138]=(self.scalar_static_f64[133]-self.scalar_static_f64[136]);
        self.scalar_static_f64[139]=(2.0*self.scalar_static_f64[137]);
        self.scalar_static_f64[140]=(2.0*self.scalar_static_f64[138]);
        self.scalar_static_f64[141]=(2.0*self.scalar_static_f64[134]);
        self.scalar_static_f64[142]=(self.scalar_static_f64[4]*self.scalar_static_f64[90]);
        self.scalar_static_f64[143]=(self.scalar_static_f64[90]*self.scalar_static_f64[127]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[4]*self.scalar_static_f64[98]);
        self.scalar_static_f64[145]=(self.scalar_static_f64[98]*self.scalar_static_f64[127]);
        self.scalar_static_f64[146]=(self.scalar_static_f64[4]*self.scalar_static_f64[103]);
        self.scalar_static_f64[147]=(self.scalar_static_f64[103]*self.scalar_static_f64[127]);
        self.scalar_static_f64[148]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[149]=(1.0/self.scalar_static_f64[112]);
        self.scalar_static_f64[150]=(if self.scalar_static_bool[12]{self.scalar_static_f64[149]}else{0.0});
        self.scalar_static_f64[151]=(-1.0/self.scalar_static_f64[112]);
        self.scalar_static_f64[152]=(if self.scalar_static_bool[28]{self.scalar_static_f64[149]}else{0.0});
        self.scalar_static_f64[153]=(if self.scalar_static_bool[28]{self.scalar_static_f64[151]}else{0.0});
        self.scalar_static_f64[154]=(1.0/self.scalar_static_f64[113]);
        self.scalar_static_f64[155]=(if self.scalar_static_bool[28]{self.scalar_static_f64[154]}else{0.0});
    }
}
