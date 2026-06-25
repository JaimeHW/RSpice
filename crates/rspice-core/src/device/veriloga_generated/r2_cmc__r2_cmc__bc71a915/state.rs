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
        params.p2 = 100.0;
        params.p3 = 1.0;
        params.p4 = 1.0;
        params.p5 = 0.0;
        params.p6 = 1.0;
        params.p7 = 1.0;
        params.p8 = 2.0;
        params.p9 = 1.0;
        params.p10 = 0.0;
        params.p11 = -100.0;
        params.p12 = 500.0;
        params.p13 = 0.001;
        params.p14 = 1002.0;
        params.p15 = 27.0;
        params.p16 = 100.0;
        params.p17 = 0.0;
        params.p18 = 9900000000.0;
        params.p19 = 0.0;
        params.p20 = 9900000000.0;
        params.p21 = 0.0;
        params.p22 = 0.0;
        params.p23 = 0.0;
        params.p24 = 0.0;
        params.p25 = 0.0;
        params.p26 = 0.0;
        params.p27 = 0.0;
        params.p28 = 0.0;
        params.p29 = 0.0;
        params.p30 = 2.0;
        params.p31 = 1.0;
        params.p32 = 0.0;
        params.p33 = 100.0;
        params.p34 = -100.0;
        params.p35 = 500.0;
        params.p36 = 0.0;
        params.p37 = 0.0;
        params.p38 = 0.0;
        params.p39 = 0.0;
        params.p40 = 0.0;
        params.p41 = 0.0;
        params.p42 = 0.0;
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
    pub nodes: [usize; 2],
    pub branches: [usize; 0],
    pub params: Parameters,
    pub(crate) param_given: [bool; 43],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 0],
    pub(crate) ddt_state_previous: [f64; 0],
    pub(crate) ddt_state_initialized: [bool; 0],
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
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 2;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 43;
    pub const VARIABLE_COUNT: usize = 86;
    pub const DDT_STATE_COUNT: usize = 0;
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
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "r" => { validate_parameter("r", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "c1" => { validate_parameter("c1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "c2" => { validate_parameter("c2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "dra" => { validate_finite_parameter("trise", value)?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "isnoisy" => { validate_parameter("isnoisy", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "rthresh" => { validate_parameter("rthresh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "level" => { validate_finite_parameter("level", value)?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "lmin" => { validate_parameter("lmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "lmax" => { validate_parameter("lmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "wmin" => { validate_parameter("wmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "wmax" => { validate_parameter("wmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "dxle" => { validate_finite_parameter("dxle", value)?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "sw_efgeo" => { validate_parameter("sw_efgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "q3" => { validate_parameter("q3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "p3" => { validate_parameter("p3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "q2" => { validate_parameter("q2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "p2" => { validate_parameter("p2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "sw_fngeo" => { validate_parameter("sw_fngeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "jmax" => { validate_parameter("jmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "tc1" => { validate_finite_parameter("tc1", value)?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "tc2" => { validate_finite_parameter("tc2", value)?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "tc1l" => { validate_finite_parameter("tc1l", value)?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "tc2l" => { validate_finite_parameter("tc2l", value)?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "tc1w" => { validate_finite_parameter("tc1w", value)?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "tc2w" => { validate_finite_parameter("tc2w", value)?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "tc1kfn" => { validate_finite_parameter("tc1kfn", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r2_cmc'", name)),
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

}
