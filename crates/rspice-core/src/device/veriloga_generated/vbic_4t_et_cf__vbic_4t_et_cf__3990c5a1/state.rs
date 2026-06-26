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
            params.p0 = 27.0;
            params.p1 = 0.0;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 1e-16;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 0.9;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.75;
            params.p18 = 0.33;
            params.p19 = -0.5;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.75;
            params.p25 = 0.33;
            params.p26 = -0.5;
            params.p27 = 0.0;
            params.p28 = 0.75;
            params.p29 = 0.33;
            params.p30 = -0.5;
            params.p31 = 1e-18;
            params.p32 = 1.0;
            params.p33 = 1.0;
            params.p34 = 0.0;
            params.p35 = 2.0;
            params.p36 = 1e-16;
            params.p37 = 1.0;
            params.p38 = 0.0;
            params.p39 = 2.0;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 1.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 1.0;
            params.p49 = 0.0;
            params.p50 = 2.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 0.0;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 1.0;
            params.p65 = 1.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 1.12;
            params.p72 = 1.12;
            params.p73 = 1.12;
            params.p74 = 1.12;
            params.p75 = 1.12;
            params.p76 = 1.12;
            params.p77 = 1.12;
            params.p78 = 3.0;
            params.p79 = 3.0;
            params.p80 = 3.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.1;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 0.5;
            params.p90 = 0.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 1.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 1.12;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 1e-6;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 1.2;
            params.p107 = 0.0;
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
    pub branches: [usize; 0],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 108]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 9]>,
    pub(crate) ddt_state_previous: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<171, 12, 0>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<171, 12, 0>>>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["cx", "ci", "bx", "bi", "ei", "bp", "si"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 108;
    pub const VARIABLE_COUNT: usize = 171;
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
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "tref" => { validate_finite_parameter("TNOM", value)?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "rcx" => { validate_parameter("RCX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "rci" => { validate_parameter("RCI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "vo" => { validate_parameter("VO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "v0" => { validate_parameter("VO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "gamm" => { validate_parameter("GAMM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "gamma" => { validate_parameter("GAMM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "hrcf" => { validate_parameter("HRCF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "rbx" => { validate_parameter("RBX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "rbi" => { validate_parameter("RBI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "re" => { validate_parameter("RE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "rbp" => { validate_parameter("RBP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "is" => { validate_parameter("IS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "nr" => { validate_parameter("NR", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "fc" => { validate_parameter("FC", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "cbeo" => { validate_parameter("CBEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "cbe0" => { validate_parameter("CBEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "cje" => { validate_parameter("CJE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "pe" => { validate_parameter("PE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "me" => { validate_parameter("ME", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "aje" => { validate_finite_parameter("AJE", value)?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "cbco" => { validate_parameter("CBCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "cbc0" => { validate_parameter("CBCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "cjc" => { validate_parameter("CJC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "qco" => { validate_parameter("QCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "qc0" => { validate_parameter("QCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "cjep" => { validate_parameter("CJEP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "pc" => { validate_parameter("PC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "mc" => { validate_parameter("MC", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "ajc" => { validate_finite_parameter("AJC", value)?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "cjcp" => { validate_parameter("CJCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "ms" => { validate_parameter("MS", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "ajs" => { validate_finite_parameter("AJS", value)?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "ibei" => { validate_parameter("IBEI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "wbe" => { validate_parameter("WBE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "nei" => { validate_parameter("NEI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "iben" => { validate_parameter("IBEN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "nen" => { validate_parameter("NEN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "ibci" => { validate_parameter("IBCI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "nci" => { validate_parameter("NCI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "ibcn" => { validate_parameter("IBCN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "ncn" => { validate_parameter("NCN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "avc1" => { validate_parameter("AVC1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "avc2" => { validate_parameter("AVC2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "isp" => { validate_parameter("ISP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "wsp" => { validate_parameter("WSP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "nfp" => { validate_parameter("NFP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "ibeip" => { validate_parameter("IBEIP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "ibenp" => { validate_parameter("IBENP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "ibcip" => { validate_parameter("IBCIP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "ncip" => { validate_parameter("NCIP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "ibcnp" => { validate_parameter("IBCNP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "ncnp" => { validate_parameter("NCNP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "vef" => { validate_parameter("VEF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "ver" => { validate_parameter("VER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "ikf" => { validate_parameter("IKF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "ikr" => { validate_parameter("IKR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "ikp" => { validate_parameter("IKP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "tf" => { validate_parameter("TF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "qtf" => { validate_parameter("QTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "xtf" => { validate_parameter("XTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "vtf" => { validate_parameter("VTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "itf" => { validate_parameter("ITF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "td" => { validate_parameter("TD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "kfn" => { validate_parameter("KFN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "afn" => { validate_parameter("AFN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "bfn" => { validate_parameter("BFN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "xre" => { validate_finite_parameter("XRE", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "xrbi" => { validate_finite_parameter("XRBI", value)?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "xrci" => { validate_finite_parameter("XRCI", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "xrs" => { validate_finite_parameter("XRS", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "xvo" => { validate_finite_parameter("XVO", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "xv0" => { validate_finite_parameter("XVO", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "ea" => { validate_finite_parameter("EA", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "eaie" => { validate_finite_parameter("EAIE", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "eaic" => { validate_finite_parameter("EAIC", value)?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "eais" => { validate_finite_parameter("EAIS", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "eane" => { validate_finite_parameter("EANE", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "eanc" => { validate_finite_parameter("EANC", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "eans" => { validate_finite_parameter("EANS", value)?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "xis" => { validate_finite_parameter("XIS", value)?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "xii" => { validate_finite_parameter("XII", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "xin" => { validate_finite_parameter("XIN", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "tnf" => { validate_finite_parameter("TNF", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "tavc" => { validate_finite_parameter("TAVC", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "rth" => { validate_parameter("RTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "cth" => { validate_parameter("CTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "vrt" => { validate_parameter("VRT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "art" => { validate_parameter("ART", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "ccso" => { validate_parameter("CCSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "ccs0" => { validate_parameter("CCSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "qbm" => { validate_finite_parameter("QBM", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "nkf" => { validate_parameter("NKF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "xikf" => { validate_finite_parameter("XIKF", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "xrcx" => { validate_finite_parameter("XRCX", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "xrbx" => { validate_finite_parameter("XRBX", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "xrbp" => { validate_finite_parameter("XRBP", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "isrr" => { validate_parameter("ISRR", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "xisr" => { validate_finite_parameter("XISR", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "dear" => { validate_finite_parameter("DEAR", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "eap" => { validate_finite_parameter("EAP", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "vbbe" => { validate_finite_parameter("VBBE", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "nbbe" => { validate_parameter("NBBE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "ibbe" => { validate_finite_parameter("IBBE", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "tvbbe1" => { validate_finite_parameter("TVBBE1", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "tvbbe2" => { validate_finite_parameter("TVBBE2", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "tnbbe" => { validate_finite_parameter("TNBBE", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "ebbe" => { validate_finite_parameter("EBBE", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "dtmp" => { validate_finite_parameter("DTEMP", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "vers" => { validate_finite_parameter("VERS", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "version" => { validate_finite_parameter("VERS", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "vrev" => { validate_finite_parameter("VREV", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic_4T_et_cf'", name)),
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
