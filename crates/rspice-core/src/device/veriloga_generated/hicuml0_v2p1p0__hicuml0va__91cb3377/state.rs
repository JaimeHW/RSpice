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

impl Default for Parameters {
    fn default() -> Self {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut params: Self = unsafe { std::mem::zeroed::<Self>() };
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
    pub branches: [usize; 4],
    pub params: Parameters,
    pub(crate) param_given: [bool; 112],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 9],
    pub(crate) ddt_state_previous: [f64; 9],
    pub(crate) ddt_state_initialized: [bool; 9],
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "flitm" => { validate_parameter("flitm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "mcr" => { validate_parameter("mcr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "aver" => { validate_parameter("aver", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "rver" => { validate_parameter("rver", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "iqf" => { validate_parameter("iqf", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "fiqf" => { validate_parameter("fiqf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "iqr" => { validate_parameter("iqr", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "iqfh" => { validate_parameter("iqfh", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "tfh" => { validate_parameter("tfh", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "ahq" => { validate_parameter("ahq", value, Some((-0.9, "-0.9")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "ibes" => { validate_parameter("ibes", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "mbe" => { validate_parameter("mbe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "ires" => { validate_parameter("ires", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "mre" => { validate_parameter("mre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "ibcs" => { validate_parameter("ibcs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "mbc" => { validate_parameter("mbc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "vr0e" => { validate_parameter("vr0e", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "vr0c" => { validate_parameter("vr0c", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "cje0" => { validate_parameter("cje0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "ze" => { validate_parameter("ze", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "aje" => { validate_parameter("aje", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "vdedc" => { validate_parameter("vdedc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "zedc" => { validate_parameter("zedc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "ajedc" => { validate_parameter("ajedc", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "fbc" => { validate_parameter("fbc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "tbvl" => { validate_parameter("tbvl", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "gte" => { validate_parameter("gte", value, Some((0.0, "0.0")), true, Some((20.0, "20.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "zetact" => { validate_finite_parameter("zetact", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "zetabet" => { validate_finite_parameter("zetabet", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "dvgbe" => { validate_finite_parameter("dvgbe", value)?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "zetavgbe" => { validate_finite_parameter("zetavgbe", value)?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "zetaci" => { validate_finite_parameter("zetaci", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "zetarbi" => { validate_finite_parameter("zetarbi", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "zetarbx" => { validate_finite_parameter("zetarbx", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "zetarcx" => { validate_finite_parameter("zetarcx", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "zetare" => { validate_finite_parameter("zetare", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "zetaiqf" => { validate_finite_parameter("zetaiqf", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "flteft" => { validate_parameter("flteft", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "zetaver" => { validate_finite_parameter("zetaver", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "zetaiqfh" => { validate_finite_parameter("zetaiqfh", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "aliqfh" => { validate_finite_parameter("aliqfh", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "kiqfh" => { validate_finite_parameter("kiqfh", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "zetarth" => { validate_finite_parameter("zetarth", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), true, Some((600.0, "600.0")), false, &[])?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
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
