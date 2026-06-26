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
            params.p0 = 1.0;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 1e21;
            params.p4 = 1e21;
            params.p5 = 1e-5;
            params.p6 = 1e-5;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.002;
            params.p14 = 3e-7;
            params.p15 = 0.5;
            params.p16 = 0.001;
            params.p17 = 0.7;
            params.p18 = 0.5;
            params.p19 = 0.00015;
            params.p20 = -1.5;
            params.p21 = 0.0;
            params.p22 = 100000000.0;
            params.p23 = 2000000.0;
            params.p24 = 0.8;
            params.p25 = 0.8;
            params.p26 = -1e-8;
            params.p27 = -1e-8;
            params.p28 = 0.2;
            params.p29 = 0.3;
            params.p30 = 0.00023;
            params.p31 = 4e-7;
            params.p32 = 500000000.0;
            params.p33 = 400000000.0;
            params.p34 = 0.0009;
            params.p35 = 1.0;
            params.p36 = 0.0;
            params.p37 = 5e-7;
            params.p38 = 1e-6;
            params.p39 = 1e-6;
            params.p40 = 1e-6;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 1.0;
            params.p44 = 1e-9;
            params.p45 = 1e-12;
            params.p46 = 1e-12;
            params.p47 = 0.9;
            params.p48 = 0.7;
            params.p49 = 0.7;
            params.p50 = 0.8;
            params.p51 = 0.6;
            params.p52 = 0.6;
            params.p53 = 1e-9;
            params.p54 = 1e-12;
            params.p55 = 1e-12;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 10.0;
            params.p59 = 1.0;
            params.p60 = 1.0;
            params.p61 = 1.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 3.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
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
    pub nodes: [usize; 4],
    pub branches: [usize; 0],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 75]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 5]>,
    pub(crate) ddt_state_previous: Box<[f64; 5]>,
    pub(crate) ddt_state_initialized: Box<[bool; 5]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<271, 4, 0>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<271, 4, 0>>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 4;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 75;
    pub const VARIABLE_COUNT: usize = 271;
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
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "temp" => { validate_parameter("TEMP", value, Some((273.15, "273.15")), false, None, false, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "m" => { validate_parameter("M", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "ns" => { validate_parameter("NS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "cox" => { validate_parameter("COX", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "xj" => { validate_parameter("XJ", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "vto" => { validate_finite_parameter("VTO", value)?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "tcv" => { validate_finite_parameter("TCV", value)?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "gamma" => { validate_parameter("GAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "phi" => { validate_parameter("PHI", value, Some((0.2, "0.2")), false, None, false, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "kp" => { validate_parameter("KP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "bex" => { validate_finite_parameter("BEX", value)?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "theta" => { validate_parameter("THETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "e0" => { validate_finite_parameter("E0", value)?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "ucrit" => { validate_parameter("UCRIT", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "ucex" => { validate_finite_parameter("UCEX", value)?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "lambda" => { validate_parameter("LAMBDA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "dl" => { validate_finite_parameter("DL", value)?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "dw" => { validate_finite_parameter("DW", value)?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "weta" => { validate_parameter("WETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "leta" => { validate_parameter("LETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "q0" => { validate_parameter("Q0", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "lk" => { validate_parameter("LK", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "iba" => { validate_parameter("IBA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "ibb" => { validate_parameter("IBB", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "ibbt" => { validate_finite_parameter("IBBT", value)?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "ibn" => { validate_parameter("IBN", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "hdif" => { validate_parameter("HDIF", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "avto" => { validate_parameter("AVTO", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "akp" => { validate_parameter("AKP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "agamma" => { validate_parameter("AGAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "af" => { validate_parameter("AF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "kf" => { validate_parameter("KF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "xd_n" => { validate_parameter("xd_n", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "xd_js" => { validate_parameter("xd_js", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "xd_jsw" => { validate_parameter("xd_jsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "xd_jswg" => { validate_parameter("xd_jswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "xd_mj" => { validate_parameter("xd_mj", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "xd_mjsw" => { validate_parameter("xd_mjsw", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "xd_mjswg" => { validate_parameter("xd_mjswg", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "xd_pb" => { validate_parameter("xd_pb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "xd_pbsw" => { validate_parameter("xd_pbsw", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "xd_pbswg" => { validate_parameter("xd_pbswg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "xd_cj" => { validate_parameter("xd_cj", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "xd_cjsw" => { validate_parameter("xd_cjsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "xd_cjswg" => { validate_parameter("xd_cjswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "xd_gmin" => { validate_parameter("xd_gmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "xd_xjbv" => { validate_parameter("xd_xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "xd_bv" => { validate_parameter("xd_bv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "xd_njts" => { validate_parameter("xd_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "xd_njtssw" => { validate_parameter("xd_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "xd_njtsswg" => { validate_parameter("xd_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "xd_vts" => { validate_parameter("xd_vts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "xd_vtssw" => { validate_parameter("xd_vtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "xd_vtsswg" => { validate_parameter("xd_vtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "tp_xti" => { validate_finite_parameter("tp_xti", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "tp_cj" => { validate_finite_parameter("tp_cj", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "tp_cjsw" => { validate_finite_parameter("tp_cjsw", value)?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "tp_cjswg" => { validate_finite_parameter("tp_cjswg", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "tp_pb" => { validate_finite_parameter("tp_pb", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "tp_pbsw" => { validate_finite_parameter("tp_pbsw", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "tp_pbswg" => { validate_finite_parameter("tp_pbswg", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "tp_njts" => { validate_parameter("tp_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "tp_njtssw" => { validate_parameter("tp_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "tp_njtsswg" => { validate_parameter("tp_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name)),
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
