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
    pub nodes: [usize; 7],
    pub branches: [usize; 4],
    pub params: Parameters,
    pub(crate) param_given: [bool; 67],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 3],
    pub(crate) ddt_state_previous: [f64; 3],
    pub(crate) ddt_state_initialized: [bool; 3],
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
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "subversion" => { validate_finite_parameter("SUBVERSION", value)?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "revision" => { validate_finite_parameter("REVISION", value)?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((-273.0, "-273.0")), false, Some((21.0, "21.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((21.0, "21.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "lmin" => { validate_parameter("LMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "lmax" => { validate_parameter("LMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "wmin" => { validate_parameter("WMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "wmax" => { validate_parameter("WMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "swres" => { validate_parameter("SWRES", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "typep" => { validate_parameter("TYPEP", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "toxo" => { validate_parameter("TOXO", value, Some((5e-10, "5e-10")), false, Some((2e-6, "2e-6")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "epsroxo" => { validate_parameter("EPSROXO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "swqinv" => { validate_parameter("SWQINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "tau" => { validate_parameter("TAU", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "nsubo" => { validate_parameter("NSUBO", value, Some((1e18, "1e18")), false, Some((1e25, "1e25")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "mnsubo" => { validate_parameter("MNSUBO", value, Some((1.0, "1.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "dnsubo" => { validate_parameter("DNSUBO", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "vnsubo" => { validate_parameter("VNSUBO", value, Some((-5.0, "-5.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "nslpo" => { validate_parameter("NSLPO", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((1e24, "1e24")), false, Some((1e27, "1e27")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "dwr" => { validate_finite_parameter("DWR", value)?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "cfrl" => { validate_parameter("CFRL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "cfrw" => { validate_parameter("CFRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "rpv" => { validate_parameter("RPV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "rend" => { validate_parameter("REND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "uac" => { validate_parameter("UAC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "uacred" => { validate_parameter("UACRED", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "strshg" => { validate_finite_parameter("STRSHG", value)?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "strpv" => { validate_finite_parameter("STRPV", value)?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "strend" => { validate_finite_parameter("STREND", value)?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "strshs" => { validate_finite_parameter("STRSHS", value)?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "stuac" => { validate_finite_parameter("STUAC", value)?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "chibpo" => { validate_parameter("CHIBPO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "lov" => { validate_parameter("LOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1e22, "1e22")), false, Some((1e26, "1e26")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "igovw" => { validate_parameter("IGOVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "gc2o" => { validate_parameter("GC2O", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "gc3o" => { validate_parameter("GC3O", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "igchvlw" => { validate_parameter("IGCHVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "igovhvw" => { validate_parameter("IGOVHVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "gcohvo" => { validate_parameter("GCOHVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "gc2hvo" => { validate_parameter("GC2HVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "gc3hvo" => { validate_parameter("GC3HVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "igmax" => { validate_parameter("IGMAX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "racnoise" => { validate_parameter("RACNOISE", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
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
