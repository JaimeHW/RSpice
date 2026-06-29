#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::support::{ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

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
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 1.0;
            params.p14 = 1.0;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 1.0;
            params.p18 = 1.0;
            params.p19 = 2.0;
            params.p20 = 1003.0;
            params.p21 = -1.0;
            params.p22 = 1.0;
            validate_parameter("scale", params.p22, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p23 = 0.0;
            validate_parameter("shrink", params.p23, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p24 = -100.0;
            params.p25 = 500.0;
            params.p26 = 0.001;
            validate_parameter("rthresh", params.p26, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p27 = 1.0;
            validate_parameter("imax", params.p27, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p28 = 27.0;
            params.p29 = 0.0;
            params.p30 = 9900000000.0;
            params.p31 = 0.0;
            params.p32 = 9900000000.0;
            params.p33 = 100.0;
            params.p34 = 9900000000.0;
            params.p35 = -100.0;
            params.p36 = 500.0;
            params.p37 = 100.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = 1.0;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.01;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 1.0;
            params.p54 = 2.0;
            params.p55 = 0.0;
            params.p56 = 0.5;
            params.p57 = 0.0;
            params.p58 = 2.0;
            params.p59 = 0.0;
            params.p60 = 4.0;
            params.p61 = 0.4;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 1e-12;
            params.p65 = 0.02;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.9;
            params.p69 = 0.0;
            params.p70 = 1.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.75;
            params.p74 = 0.33;
            params.p75 = -0.5;
            params.p76 = 0.0;
            params.p77 = 1.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.75;
            params.p81 = 0.33;
            params.p82 = -0.5;
            params.p83 = 0.0;
            params.p84 = 1e-6;
            params.p85 = 1.0;
            params.p86 = 0.0;
            params.p87 = 2.0;
            params.p88 = 1.0;
            params.p89 = 0.0;
            params.p90 = 1.12;
            params.p91 = 3.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 0.0;
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
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 1000000.0;
            params.p111 = 0.0;
            params.p112 = 0.0;
            params.p113 = 0.0;
            params.p114 = 0.0;
            params.p115 = 0.0;
            params.p116 = 0.0;
            params.p117 = 0.0;
            params.p118 = 0.0;
            params.p119 = 0.0;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = 0.0;
            params.p126 = 0.0;
            params.p127 = 0.0;
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
    pub nodes: [usize; 6],
    pub branches: [usize; 2],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 128]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 3]>,
    pub(crate) ddt_state_previous: Box<[f64; 3]>,
    pub(crate) ddt_state_initialized: Box<[bool; 3]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<329, 6, 2>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<329, 6, 2>>>,
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
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 2;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 2] = ["i1", "i2"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 128;
    pub const VARIABLE_COUNT: usize = 329;
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
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            ddt_state_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: None,
        }
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let scratch = self.scratch.take();
        let reactive_scratch = self.reactive_scratch.take();
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            scratch: _,
            reactive_scratch: _,
        } = snapshot;
        *self = Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            scratch,
            reactive_scratch,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "wd" => { validate_parameter("wd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "a1" => { validate_parameter("a1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "p1" => { validate_parameter("p1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "c1" => { validate_parameter("c1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "a2" => { validate_parameter("a2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "p2" => { validate_parameter("p2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "c2" => { validate_parameter("c2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "dta" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "nsmm_rsh" => { validate_finite_parameter("nsmm_rsh", value)?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "nsmm_w" => { validate_finite_parameter("nsmm_w", value)?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "nsmm_l" => { validate_finite_parameter("nsmm_l", value)?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "sw_noise" => { validate_parameter("sw_noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "sw_et" => { validate_parameter("sw_et", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "sw_lin" => { validate_parameter("sw_lin", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "sw_mman" => { validate_parameter("sw_mman", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "subversion" => { validate_finite_parameter("subversion", value)?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "level" => { validate_finite_parameter("level", value)?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "rthresh" => { validate_parameter("rthresh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "imax" => { validate_parameter("imax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "lmin" => { validate_parameter("lmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "lmax" => { validate_finite_parameter("lmax", value)?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "wmin" => { validate_parameter("wmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "wmax" => { validate_finite_parameter("wmax", value)?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "jmax" => { validate_parameter("jmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "vmax" => { validate_parameter("vmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "nwxw" => { validate_finite_parameter("nwxw", value)?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "wexw" => { validate_finite_parameter("wexw", value)?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "fdrw" => { validate_parameter("fdrw", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "fdxwinf" => { validate_finite_parameter("fdxwinf", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "xlw" => { validate_finite_parameter("xlw", value)?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "dxlsat" => { validate_finite_parameter("dxlsat", value)?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "nst" => { validate_parameter("nst", value, Some((0.1, "0.1")), false, Some((5.0, "5.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "ats" => { validate_parameter("ats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "atsinf" => { validate_parameter("ats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "atsl" => { validate_parameter("atsl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "dfinf" => { validate_parameter("dfinf", value, Some((0.0001, "0.0001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "dfw" => { validate_finite_parameter("dfw", value)?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "dfl" => { validate_finite_parameter("dfl", value)?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "dfwl" => { validate_finite_parameter("dfwl", value)?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "sw_dfgeo" => { validate_parameter("sw_dfgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "dp" => { validate_parameter("dp", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "dpinf" => { validate_parameter("dp", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "dpw" => { validate_finite_parameter("dpw", value)?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "dpwe" => { validate_finite_parameter("dpwe", value)?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "dpl" => { validate_finite_parameter("dpl", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "dple" => { validate_finite_parameter("dple", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "dpwl" => { validate_finite_parameter("dpwl", value)?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "ecrit" => { validate_parameter("ecrit", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "ecorn" => { validate_parameter("ecorn", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "sw_vsatt" => { validate_parameter("sw_vsatt", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "sw_accpo" => { validate_parameter("sw_accpo", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "grpo" => { validate_parameter("grpo", value, Some((0.0, "0.0")), true, Some((0.1, "0.1")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "du" => { validate_parameter("du", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "rc" => { validate_parameter("rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "rcw" => { validate_parameter("rcw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "isa" => { validate_parameter("isa", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "na" => { validate_parameter("na", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "ca" => { validate_parameter("ca", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "cja" => { validate_parameter("cja", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "pa" => { validate_parameter("pa", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "ma" => { validate_parameter("ma", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "aja" => { validate_finite_parameter("aja", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "isp" => { validate_parameter("isp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "np" => { validate_parameter("np", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "cp" => { validate_parameter("cp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "cjp" => { validate_parameter("cjp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "pp" => { validate_parameter("pp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "mp" => { validate_parameter("mp", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "ajp" => { validate_finite_parameter("ajp", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "vbv" => { validate_parameter("vbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "ibv" => { validate_parameter("ibv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "sw_fngeo" => { validate_parameter("sw_fngeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "xis" => { validate_finite_parameter("xis", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "xvsat" => { validate_finite_parameter("xvsat", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "tc1" => { validate_finite_parameter("tc1", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "tc2" => { validate_finite_parameter("tc2", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "tc1l" => { validate_finite_parameter("tc1l", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "tc2l" => { validate_finite_parameter("tc2l", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "tc1w" => { validate_finite_parameter("tc1w", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "tc2w" => { validate_finite_parameter("tc2w", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "tc1wl" => { validate_finite_parameter("tc1wl", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "tc2wl" => { validate_finite_parameter("tc2wl", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "tc1rc" => { validate_finite_parameter("tc1rc", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "tc2rc" => { validate_finite_parameter("tc2rc", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "tc1dp" => { validate_finite_parameter("tc1dp", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "tc2dp" => { validate_finite_parameter("tc2dp", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "tc1vbv" => { validate_finite_parameter("tc1vbv", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "tc2vbv" => { validate_finite_parameter("tc2vbv", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "tc1nbv" => { validate_finite_parameter("tc1nbv", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "tc1kfn" => { validate_finite_parameter("tc1kfn", value)?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "tegth" => { validate_parameter("tegth", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "gth0" => { validate_parameter("gth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "gthp" => { validate_parameter("gthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "gtha" => { validate_parameter("gtha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "gthc" => { validate_parameter("gthc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "cthp" => { validate_parameter("cthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "ctha" => { validate_parameter("ctha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "cthc" => { validate_parameter("cthc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "nsig_rsh" => { validate_finite_parameter("nsig_rsh", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "nsig_w" => { validate_finite_parameter("nsig_w", value)?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "nsig_l" => { validate_finite_parameter("nsig_l", value)?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "sig_rsh" => { validate_parameter("sig_rsh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "sig_w" => { validate_parameter("sig_w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "sig_l" => { validate_parameter("sig_l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "smm_rsh" => { validate_parameter("smm_rsh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "smm_w" => { validate_parameter("smm_w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "smm_l" => { validate_parameter("smm_l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "sw_mmgeo" => { validate_parameter("sw_mmgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r3_cmc'", name)),
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
