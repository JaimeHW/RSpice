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
            params.p0 = 200.0;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 1e-12;
            params.p4 = 1e-6;
            params.p5 = 1e-6;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 0.0;
            params.p10 = 1.0;
            params.p11 = 1.0;
            params.p12 = 1000.0;
            params.p13 = 21.0;
            params.p14 = 1000.0;
            params.p15 = 0.001;
            params.p16 = 1e-9;
            params.p17 = 1e-9;
            params.p18 = 1.0;
            params.p19 = 1.0;
            params.p20 = 1.0;
            params.p21 = 0.5;
            params.p22 = 0.5;
            params.p23 = 0.5;
            params.p24 = 1.16;
            params.p25 = 1.16;
            params.p26 = 1.16;
            params.p27 = 1e-12;
            params.p28 = 1e-18;
            params.p29 = 1e-18;
            params.p30 = 100.0;
            params.p31 = 0.0001;
            params.p32 = 0.0001;
            params.p33 = 1e-7;
            params.p34 = 1e-7;
            params.p35 = 100.0;
            params.p36 = 0.0001;
            params.p37 = 0.0001;
            params.p38 = 0.25;
            params.p39 = 0.25;
            params.p40 = 0.25;
            params.p41 = 1e-12;
            params.p42 = 1e-18;
            params.p43 = 1e-18;
            params.p44 = 1000000000.0;
            params.p45 = 1000000000.0;
            params.p46 = 1000000000.0;
            params.p47 = -0.001;
            params.p48 = -0.001;
            params.p49 = -0.001;
            params.p50 = 10.0;
            params.p51 = 10.0;
            params.p52 = 10.0;
            params.p53 = 4.0;
            params.p54 = 4.0;
            params.p55 = 4.0;
            params.p56 = 1.0;
            params.p57 = 1.0;
            params.p58 = 1.0;
            params.p59 = 1.0;
            params.p60 = -1.0;
            params.p61 = 0.1;
            params.p62 = 0.0;
            params.p63 = 2.5;
            params.p64 = 0.03;
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
    pub nodes: [usize; 2],
    pub branches: [usize; 0],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 65]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 1]>,
    pub(crate) ddt_state_previous: Box<[f64; 1]>,
    pub(crate) ddt_state_initialized: Box<[bool; 1]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<668, 2, 0>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<668, 2, 0>>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 2;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 65;
    pub const VARIABLE_COUNT: usize = 668;
    pub const DDT_STATE_COUNT: usize = 1;
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
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "ab" => { validate_parameter("AB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "ls" => { validate_parameter("LS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "lg" => { validate_parameter("LG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "mult" => { validate_parameter("MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "trise" => { validate_finite_parameter("TRISE", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "dtemp" => { validate_finite_parameter("TRISE", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "ifactor" => { validate_parameter("IFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "cfactor" => { validate_parameter("CFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "imax" => { validate_parameter("IMAX", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "trj" => { validate_parameter("TRJ", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "tref" => { validate_parameter("TRJ", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "frev" => { validate_parameter("FREV", value, Some((10.0, "10.0")), false, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "cjorbot" => { validate_parameter("CJORBOT", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "cjorsti" => { validate_parameter("CJORSTI", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "cjorgat" => { validate_parameter("CJORGAT", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "vbirbot" => { validate_parameter("VBIRBOT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "vbirsti" => { validate_parameter("VBIRSTI", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "vbirgat" => { validate_parameter("VBIRGAT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "pbot" => { validate_parameter("PBOT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "psti" => { validate_parameter("PSTI", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "pgat" => { validate_parameter("PGAT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "phigbot" => { validate_finite_parameter("PHIGBOT", value)?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "phigsti" => { validate_finite_parameter("PHIGSTI", value)?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "phiggat" => { validate_finite_parameter("PHIGGAT", value)?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "idsatrbot" => { validate_parameter("IDSATRBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "idsatrsti" => { validate_parameter("IDSATRSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "idsatrgat" => { validate_parameter("IDSATRGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "csrhbot" => { validate_parameter("CSRHBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "csrhsti" => { validate_parameter("CSRHSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "csrhgat" => { validate_parameter("CSRHGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "xjunsti" => { validate_parameter("XJUNSTI", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "xjungat" => { validate_parameter("XJUNGAT", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "ctatbot" => { validate_parameter("CTATBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "ctatsti" => { validate_parameter("CTATSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "ctatgat" => { validate_parameter("CTATGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "mefftatbot" => { validate_parameter("MEFFTATBOT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "mefftatsti" => { validate_parameter("MEFFTATSTI", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "mefftatgat" => { validate_parameter("MEFFTATGAT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "cbbtbot" => { validate_parameter("CBBTBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "cbbtsti" => { validate_parameter("CBBTSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "cbbtgat" => { validate_parameter("CBBTGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "fbbtrbot" => { validate_finite_parameter("FBBTRBOT", value)?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "fbbtrsti" => { validate_finite_parameter("FBBTRSTI", value)?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "fbbtrgat" => { validate_finite_parameter("FBBTRGAT", value)?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "stfbbtbot" => { validate_finite_parameter("STFBBTBOT", value)?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "stfbbtsti" => { validate_finite_parameter("STFBBTSTI", value)?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "stfbbtgat" => { validate_finite_parameter("STFBBTGAT", value)?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "vbrbot" => { validate_parameter("VBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "vbrsti" => { validate_parameter("VBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "vbrgat" => { validate_parameter("VBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "pbrbot" => { validate_parameter("PBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "pbrsti" => { validate_parameter("PBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "pbrgat" => { validate_parameter("PBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "fcjorgat2" => { validate_parameter("FCJORGAT2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "fvbirgat2" => { validate_parameter("FVBIRGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "fpgat2" => { validate_parameter("FPGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "fphiggat2" => { validate_parameter("FPHIGGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "vtrgat" => { validate_parameter("VTRGAT", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "anugat" => { validate_parameter("ANUGAT", value, Some((0.001, "0.001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "swjunexp" => { validate_parameter("SWJUNEXP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "vjunref" => { validate_parameter("VJUNREF", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "fjunq" => { validate_parameter("FJUNQ", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'JUNCAP200'", name)),
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
