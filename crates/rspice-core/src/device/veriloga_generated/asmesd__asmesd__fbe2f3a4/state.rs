#![allow(dead_code, unused_parens, unused_variables)]

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

impl Default for Parameters {
    fn default() -> Self {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut params: Self = unsafe { std::mem::zeroed::<Self>() };
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
        params
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
pub struct Instance {
    pub nodes: [usize; 10],
    pub branches: [usize; 8],
    pub params: Parameters,
    pub(crate) param_given: [bool; 85],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 13],
    pub(crate) ddt_state_previous: [f64; 13],
    pub(crate) ddt_state_initialized: [bool; 13],
    pub(crate) idt_state_current: [f64; 0],
    pub(crate) idt_state_previous: [f64; 0],
    pub(crate) idt_state_initialized: [bool; 0],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
}

impl Copy for Instance {}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self { *self }
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
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::default(),
            param_given: [false; Self::PARAMETER_COUNT],
            multiplicity: 1.0,
            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],
            idt_state_current: [0.0; Self::IDT_STATE_COUNT],
            idt_state_previous: [0.0; Self::IDT_STATE_COUNT],
            idt_state_initialized: [false; Self::IDT_STATE_COUNT],
            time: 0.0,
            timestep: 0.0,
        }
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "isr" => { validate_parameter("isr", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "ntr" => { validate_parameter("ntr", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "vtr" => { validate_parameter("vtr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "bvr" => { validate_parameter("bvr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "xbvr" => { validate_finite_parameter("xbvr", value)?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "xjbv" => { validate_parameter("xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "ther" => { validate_parameter("ther", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "theexp" => { validate_parameter("theexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "xtheexp" => { validate_finite_parameter("xtheexp", value)?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "rb" => { validate_parameter("rb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "rbe" => { validate_parameter("rbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "ree" => { validate_parameter("ree", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "vje" => { validate_parameter("vje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "mje" => { validate_parameter("mje", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "qtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "vtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "eg" => { validate_parameter("eg", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "xtir" => { validate_parameter("xtir", value, Some((-20.0, "-20.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-40.0, "-40.0")), false, Some((125.0, "125.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "tfail" => { validate_parameter("tfail", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "extmod" => { validate_parameter("extmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "rbmod" => { validate_parameter("rbmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "rth1" => { validate_parameter("rth1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "cth1" => { validate_parameter("cth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "arb" => { validate_parameter("arb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "are" => { validate_parameter("are", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "texp" => { validate_parameter("texp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "vtf0" => { validate_parameter("vtf0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "atff" => { validate_parameter("atff", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "n" => { validate_parameter("n", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "qexp" => { validate_parameter("qexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "ijbv" => { validate_parameter("ijbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "vsatb" => { validate_parameter("vsatb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "mexp" => { validate_parameter("mexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "vsate" => { validate_parameter("vsate", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "mexpe" => { validate_parameter("mexpe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "bf" => { validate_parameter("bf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "vaf" => { validate_parameter("vaf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "ikf" => { validate_parameter("ikf", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "xjbvc" => { validate_parameter("xjbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "ijbvc" => { validate_parameter("ijbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "nbvc" => { validate_parameter("nbvc", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "ise" => { validate_parameter("ise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "ne" => { validate_parameter("ne", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "br" => { validate_parameter("br", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "nr" => { validate_parameter("nr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "var" => { validate_parameter("var", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "ikr" => { validate_parameter("ikr", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "isc" => { validate_parameter("isc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "nc" => { validate_parameter("nc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "rc" => { validate_parameter("rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "rce" => { validate_parameter("rce", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "ptf" => { validate_parameter("ptf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "vjc" => { validate_parameter("vjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "mjc" => { validate_parameter("mjc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "vjs" => { validate_parameter("vjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "mjs" => { validate_parameter("mjs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "xtb" => { validate_parameter("xtb", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "arc" => { validate_parameter("arc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "kbwm" => { validate_parameter("kbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "xbwm" => { validate_parameter("xbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "ikbwm" => { validate_parameter("ikbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "xkf" => { validate_parameter("xkf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "cthbb" => { validate_parameter("cthbb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "cdelay" => { validate_parameter("cdelay", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
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
    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {
        self.time = time;
        self.timestep = timestep;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_previous[index] = self.ddt_state_current[index];
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
        self.ddt_state_current[slot] = value;
        if self.timestep.abs() > Self::DDT_EPSILON {
            (value - previous) / self.timestep
        } else {
            self.ddt_state_previous[slot] = value;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative / self.timestep
        } else {
            0.0
        }
    }
}
