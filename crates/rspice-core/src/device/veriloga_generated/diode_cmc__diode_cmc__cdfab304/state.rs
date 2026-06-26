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
            params.p0 = 2002.0;
            params.p1 = 3.0;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.001;
            validate_parameter("minr", params.p4, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p5 = 1000.0;
            params.p6 = 21.0;
            params.p7 = 1000.0;
            params.p8 = 0.001;
            params.p9 = 1e-9;
            params.p10 = 1e-9;
            params.p11 = 1.0;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 0.5;
            params.p15 = 0.5;
            params.p16 = 0.5;
            params.p17 = 1.16;
            params.p18 = 1.16;
            params.p19 = 1.16;
            params.p20 = 1e-12;
            params.p21 = 1e-18;
            params.p22 = 1e-18;
            params.p23 = 100.0;
            params.p24 = 0.0001;
            params.p25 = 0.0001;
            params.p26 = 1e-7;
            params.p27 = 1e-7;
            params.p28 = 100.0;
            params.p29 = 0.0001;
            params.p30 = 0.0001;
            params.p31 = 0.25;
            params.p32 = 0.25;
            params.p33 = 0.25;
            params.p34 = 1e-12;
            params.p35 = 1e-18;
            params.p36 = 1e-18;
            params.p37 = 1000000000.0;
            params.p38 = 1000000000.0;
            params.p39 = 1000000000.0;
            params.p40 = -0.001;
            params.p41 = -0.001;
            params.p42 = -0.001;
            params.p43 = 10.0;
            params.p44 = 10.0;
            params.p45 = 10.0;
            params.p46 = 4.0;
            params.p47 = 4.0;
            params.p48 = 4.0;
            params.p49 = 0.0;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 1.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 0.0;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 1.0;
            params.p64 = 1.0;
            params.p65 = 1.0;
            params.p66 = 0.0;
            params.p67 = 1.0;
            params.p68 = 0.0;
            params.p69 = 1.0;
            params.p70 = 0.0;
            params.p71 = 1.0;
            params.p72 = -55.0;
            params.p73 = 155.0;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 3.0;
            params.p77 = 1.0;
            params.p78 = 0.0;
            params.p79 = 1e20;
            params.p80 = 1.0;
            params.p81 = 0.0;
            params.p82 = 2.5;
            params.p83 = 0.03;
            params.p84 = 0.0;
            params.p85 = 1.0;
            params.p86 = 0.1;
            params.p87 = 1e16;
            params.p88 = 1e16;
            params.p89 = 1e16;
            params.p90 = 1.0;
            params.p91 = 10.0;
            params.p92 = 5e-9;
            params.p93 = 2e-7;
            params.p94 = 5e-6;
            params.p95 = 0.0;
            params.p96 = 21.0;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 1e-12;
            params.p100 = 1e-6;
            params.p101 = 0.0;
            params.p102 = 0.0;
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
    pub branches: [usize; 4],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 103]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 5]>,
    pub(crate) ddt_state_previous: Box<[f64; 5]>,
    pub(crate) ddt_state_initialized: Box<[bool; 5]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<962, 6, 4>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<962, 6, 4>>>,
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
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["aik", "charge_a", "charge_k", "depl_a"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 103;
    pub const VARIABLE_COUNT: usize = 962;
    pub const DDT_STATE_COUNT: usize = 5;
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
            reactive_scratch: Some(GenericReactiveScratch::new_box()),
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
            "level" => { validate_finite_parameter("level", value)?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "subversion" => { validate_finite_parameter("subversion", value)?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "imax" => { validate_parameter("imax", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "trj" => { validate_parameter("trj", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "frev" => { validate_parameter("frev", value, Some((1000.0, "1000.0")), false, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "cjorbot" => { validate_parameter("cjorbot", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "cjorsti" => { validate_parameter("cjorsti", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "cjorgat" => { validate_parameter("cjorgat", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "vbirbot" => { validate_parameter("vbirbot", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "vbirsti" => { validate_parameter("vbirsti", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "vbirgat" => { validate_parameter("vbirgat", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "pbot" => { validate_parameter("pbot", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "psti" => { validate_parameter("psti", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "pgat" => { validate_parameter("pgat", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "phigbot" => { validate_finite_parameter("phigbot", value)?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "phigsti" => { validate_finite_parameter("phigsti", value)?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "phiggat" => { validate_finite_parameter("phiggat", value)?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "idsatrbot" => { validate_parameter("idsatrbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "idsatrsti" => { validate_parameter("idsatrsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "idsatrgat" => { validate_parameter("idsatrgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "csrhbot" => { validate_parameter("csrhbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "csrhsti" => { validate_parameter("csrhsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "csrhgat" => { validate_parameter("csrhgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "xjunsti" => { validate_parameter("xjunsti", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "xjungat" => { validate_parameter("xjungat", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "ctatbot" => { validate_parameter("ctatbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "ctatsti" => { validate_parameter("ctatsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "ctatgat" => { validate_parameter("ctatgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "mefftatbot" => { validate_parameter("mefftatbot", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "mefftatsti" => { validate_parameter("mefftatsti", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "mefftatgat" => { validate_parameter("mefftatgat", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "cbbtbot" => { validate_parameter("cbbtbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "cbbtsti" => { validate_parameter("cbbtsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "cbbtgat" => { validate_parameter("cbbtgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "fbbtrbot" => { validate_finite_parameter("fbbtrbot", value)?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "fbbtrsti" => { validate_finite_parameter("fbbtrsti", value)?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "fbbtrgat" => { validate_finite_parameter("fbbtrgat", value)?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "stfbbtbot" => { validate_finite_parameter("stfbbtbot", value)?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "stfbbtsti" => { validate_finite_parameter("stfbbtsti", value)?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "stfbbtgat" => { validate_finite_parameter("stfbbtgat", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "vbrbot" => { validate_parameter("vbrbot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "vbrsti" => { validate_parameter("vbrsti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "vbrgat" => { validate_parameter("vbrgat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "pbrbot" => { validate_parameter("pbrbot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "pbrsti" => { validate_parameter("pbrsti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "pbrgat" => { validate_parameter("pbrgat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "rsbot" => { validate_parameter("rsbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "rssti" => { validate_parameter("rssti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "rsgat" => { validate_parameter("rsgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "rscom" => { validate_parameter("rscom", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "strs" => { validate_parameter("strs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "tt" => { validate_parameter("tt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "stvbrbot1" => { validate_finite_parameter("stvbrbot1", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "stvbrbot2" => { validate_finite_parameter("stvbrbot2", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "stvbrsti1" => { validate_finite_parameter("stvbrsti1", value)?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "stvbrsti2" => { validate_finite_parameter("stvbrsti2", value)?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "stvbrgat1" => { validate_finite_parameter("stvbrgat1", value)?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "stvbrgat2" => { validate_finite_parameter("stvbrgat2", value)?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "nfabot" => { validate_parameter("nfabot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "nfasti" => { validate_parameter("nfasti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "nfagat" => { validate_parameter("nfagat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "abmin" => { validate_parameter("abmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "abmax" => { validate_parameter("abmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "lsmin" => { validate_parameter("lsmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "lsmax" => { validate_parameter("lsmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "lgmin" => { validate_parameter("lgmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "lgmax" => { validate_parameter("lgmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "tempmin" => { validate_parameter("tempmin", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "tempmax" => { validate_parameter("tempmax", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "vfmax" => { validate_parameter("vfmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "vrmax" => { validate_parameter("vrmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "pt" => { validate_parameter("xti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "expceil" => { validate_parameter("expceil", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "swbv" => { validate_parameter("swbv", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "bv_enable" => { validate_parameter("swbv", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "swjunexp" => { validate_parameter("swjunexp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "vjunref" => { validate_parameter("vjunref", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "fjunq" => { validate_parameter("fjunq", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "corecovery" => { validate_parameter("corecovery", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "njh" => { validate_parameter("njh", value, Some((0.5, "0.5")), false, Some((5.0, "5.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "njdv" => { validate_parameter("njdv", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "ndibot" => { validate_parameter("ndibot", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "ndigat" => { validate_parameter("ndigat", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "ndisti" => { validate_parameter("ndisti", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "inj1" => { validate_parameter("inj1", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "inj2" => { validate_parameter("inj2", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "nqs" => { validate_parameter("nqs", value, Some((0.0, "0.0")), false, Some((0.001, "0.001")), false, &[])?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "tau" => { validate_parameter("tau", value, Some((1e-12, "1e-12")), false, Some((0.001, "0.001")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "wi" => { validate_parameter("wi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "depnqs" => { validate_parameter("depnqs", value, Some((0.0, "0.0")), false, Some((0.001, "0.001")), false, &[])?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "taut" => { validate_parameter("taut", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "injt" => { validate_parameter("injt", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), false, &[])?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "ab" => { validate_parameter("ab", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "area" => { validate_parameter("ab", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "ls" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "perim" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "pj" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "lg" => { validate_parameter("lg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'DIODE_CMC'", name)),
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
