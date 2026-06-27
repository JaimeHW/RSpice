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
            params.p1 = 0.0;
            params.p2 = 0.0;
            params.p3 = 25.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 2.0;
            params.p7 = 0.0;
            params.p8 = 0.05;
            params.p9 = -0.2;
            params.p10 = 0.2;
            params.p11 = 0.8;
            params.p12 = 0.0;
            params.p13 = 0.0;
            params.p14 = 0.1;
            params.p15 = 1.0;
            params.p16 = 0.001;
            params.p17 = 0.0;
            params.p18 = 0.1;
            params.p19 = 4.0;
            params.p20 = 0.0;
            params.p21 = 50.0;
            params.p22 = 0.0;
            params.p23 = 0.2;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 0.2;
            params.p34 = 0.0;
            params.p35 = 0.2;
            params.p36 = 0.0;
            params.p37 = 1.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 1.0;
            params.p41 = 0.5;
            params.p42 = 5e-5;
            params.p43 = 15.0;
            params.p44 = 1.0;
            params.p45 = 0.8;
            params.p46 = 0.05;
            params.p47 = 0.05;
            params.p48 = 0.0;
            params.p49 = 0.05;
            params.p50 = 0.05;
            params.p51 = 0.05;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.1;
            params.p56 = 0.0;
            params.p57 = 1000.0;
            params.p58 = 10000.0;
            params.p59 = 0.0;
            params.p60 = 100000.0;
            params.p61 = 0.0;
            params.p62 = 1.0;
            params.p63 = 1e-15;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.001;
            params.p67 = 1e-6;
            params.p68 = -0.002;
            params.p69 = -0.002;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.001;
            params.p73 = 0.001;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.002;
            params.p77 = 0.001;
            params.p78 = 0.001;
            params.p79 = -0.001;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 10.0;
            params.p84 = 100.0;
            params.p85 = 0.5;
            params.p86 = 0.5;
            params.p87 = 1.0;
            params.p88 = 0.9;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 1.0;
            params.p92 = 1.0;
            params.p93 = 25.0;
            params.p94 = 0.1;
            params.p95 = 1.0;
            params.p96 = 1e-14;
            params.p97 = 60000.0;
            params.p98 = 0.3;
            params.p99 = 0.1;
            params.p100 = 27.0;
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
    pub nodes: [usize; 19],
    pub branches: [usize; 19],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 101]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 17]>,
    pub(crate) ddt_state_previous: Box<[f64; 17]>,
    pub(crate) ddt_state_initialized: Box<[bool; 17]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: bool,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: bool,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: bool,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: bool,
    pub(crate) scalar_v37: bool,
    pub(crate) scalar_v38: bool,
    pub(crate) scalar_v39: bool,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v42: bool,
    pub(crate) scalar_v94: bool,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scratch: Option<Box<GenericScratch<145, 19, 19>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<145, 19, 19>>>,
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
            scalar_v9: self.scalar_v9,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v94: self.scalar_v94,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 14;
    pub const NODE_COUNT: usize = 19;
    pub const INTERNAL_NODE_NAMES: [&str; 14] = ["di", "dii", "gi", "si", "sii", "gdi", "gsi", "gii", "ggi", "bi", "xt1", "xt2", "ia", "ib"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 101;
    pub const VARIABLE_COUNT: usize = 145;
    pub const DDT_STATE_COUNT: usize = 17;
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
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            scalar_v9: 0.0,
            scalar_v23: 0.0,
            scalar_v24: false,
            scalar_v25: 0.0,
            scalar_v26: false,
            scalar_v27: 0.0,
            scalar_v28: false,
            scalar_v35: 0.0,
            scalar_v36: false,
            scalar_v37: false,
            scalar_v38: false,
            scalar_v39: false,
            scalar_v40: 0.0,
            scalar_v41: false,
            scalar_v42: false,
            scalar_v94: false,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: Some(GenericReactiveScratch::new_box()),
        };
        instance.recompute_instance_static();
        instance
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
            scalar_v9,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v94,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
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
            scalar_v9,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v94,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
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
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "selft" => { validate_parameter("Selft", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "temp" => { validate_parameter("Temp", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "idsmod" => { validate_parameter("Idsmod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "igmod" => { validate_parameter("Igmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "capmod" => { validate_parameter("Capmod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "noimod" => { validate_parameter("Noimod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "ipk0" => { validate_finite_parameter("Ipk0", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "vpks" => { validate_finite_parameter("Vpks", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "dvpks" => { validate_finite_parameter("Dvpks", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "p1" => { validate_finite_parameter("P1", value)?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "p2" => { validate_finite_parameter("P2", value)?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "p3" => { validate_finite_parameter("P3", value)?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "alphar" => { validate_finite_parameter("Alphar", value)?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "alphas" => { validate_finite_parameter("Alphas", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "lambda" => { validate_finite_parameter("Lambda", value)?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "lvg" => { validate_finite_parameter("Lvg", value)?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "b1" => { validate_finite_parameter("B1", value)?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "b2" => { validate_finite_parameter("B2", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "lsb0" => { validate_finite_parameter("Lsb0", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "vtr" => { validate_finite_parameter("Vtr", value)?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "vsb2" => { validate_finite_parameter("Vsb2", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "ebd" => { validate_finite_parameter("Ebd", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "cds" => { validate_parameter("Cds", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "cgspi" => { validate_finite_parameter("Cgspi", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "cgs0" => { validate_finite_parameter("Cgs0", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "cgdpi" => { validate_finite_parameter("Cgdpi", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "cgdpe" => { validate_parameter("Cgdpe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "cgd0" => { validate_finite_parameter("Cgd0", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "p10" => { validate_parameter("P10", value, Some((-100.0, "-100.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "p11" => { validate_parameter("P11", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "p20" => { validate_parameter("P20", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "p21" => { validate_parameter("P21", value, Some((0.01, "0.01")), false, Some((5.0, "5.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "p30" => { validate_parameter("P30", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "p31" => { validate_parameter("P31", value, Some((0.01, "0.01")), false, Some((5.0, "5.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "p40" => { validate_parameter("P40", value, Some((-100.0, "-100.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "p41" => { validate_parameter("P41", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "p111" => { validate_parameter("P111", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "p222" => { validate_parameter("P222", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "p10pk" => { validate_parameter("P10pk", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[(0.0, "0.0")])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "m" => { validate_finite_parameter("m", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "ij" => { validate_parameter("Ij", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "pg" => { validate_parameter("Pg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "ne" => { validate_parameter("Ne", value, Some((1.0, "1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "vjg" => { validate_parameter("Vjg", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[(0.0, "0.0")])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "rg" => { validate_parameter("Rg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "rd" => { validate_parameter("Rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "rd2" => { validate_parameter("Rd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "ri" => { validate_parameter("Ri", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "rs" => { validate_parameter("Rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "rgd" => { validate_parameter("Rgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "ld" => { validate_parameter("Ld", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "ls" => { validate_parameter("Ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "lg" => { validate_parameter("Lg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "ldc" => { validate_parameter("Ldc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "tau" => { validate_parameter("Tau", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "rcmin" => { validate_parameter("Rcmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "rc" => { validate_parameter("Rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "crf" => { validate_finite_parameter("Crf", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "rcin" => { validate_parameter("Rcin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "crfin" => { validate_parameter("Crfin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "rdel" => { validate_parameter("Rdel", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "cdel" => { validate_parameter("Cdel", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "kbgate" => { validate_finite_parameter("Kbgate", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "krfdc" => { validate_parameter("KRFDC", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "rtherm" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "ctherm" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "tcipk0" => { validate_parameter("Tcipk0", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "tcp1" => { validate_parameter("Tcp1", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "tcp3" => { validate_parameter("Tcp3", value, Some((-0.05, "-0.05")), false, Some((0.05, "0.05")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "tcp10" => { validate_parameter("Tcp10", value, Some((-0.01, "-0.01")), false, Some((0.01, "0.01")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "tccgs0" => { validate_parameter("Tccgs0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "tccgd0" => { validate_parameter("Tccgd0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "tcrc" => { validate_finite_parameter("Tcrc", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "tccrf" => { validate_finite_parameter("Tccrf", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "tcrs" => { validate_parameter("Tcrs", value, Some((0.0, "0.0")), false, Some((0.1, "0.1")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "tcrtherm" => { validate_parameter("TcRtherm", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "tcvpk" => { validate_parameter("TcVpk", value, Some((-0.1, "-0.1")), false, Some((0.1, "0.1")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "tcvjg" => { validate_finite_parameter("TcVjg", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "tclsb0" => { validate_parameter("Tclsb0", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "tcvtr" => { validate_parameter("TcVtr", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "kbdgate" => { validate_parameter("Kbdgate", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "vbdgs" => { validate_parameter("Vbdgs", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "vbdgd" => { validate_parameter("Vbdgd", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "pbdg" => { validate_parameter("Pbdg", value, Some((0.4, "0.4")), false, Some((1.0, "1.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "noiser" => { validate_parameter("NoiseR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "noisep" => { validate_parameter("NoiseP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "noisec" => { validate_parameter("NoiseC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "fnc" => { validate_parameter("Fnc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("Kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("Af", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "ffe" => { validate_parameter("Ffe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "td" => { validate_parameter("Td", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "td1" => { validate_finite_parameter("Td1", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "tmn" => { validate_finite_parameter("Tmn", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "klf" => { validate_finite_parameter("Klf", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "fgr" => { validate_finite_parameter("Fgr", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "np" => { validate_finite_parameter("Np", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "lw" => { validate_finite_parameter("Lw", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("Tnom", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'angelov_gan'", name)),
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

    #[inline]
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        let v9: f64 = p.p1;
        self.scalar_v9 = v9;
        let v23: f64 = p.p60;
        self.scalar_v23 = v23;
        let v24: bool = (p.p60 > 0.0);
        self.scalar_v24 = v24;
        let v25: f64 = p.p51;
        self.scalar_v25 = v25;
        let v26: bool = (p.p51 > 0.0);
        self.scalar_v26 = v26;
        let v27: f64 = p.p49;
        self.scalar_v27 = v27;
        let v28: bool = (p.p49 > 0.0);
        self.scalar_v28 = v28;
        let v35: f64 = p.p7;
        self.scalar_v35 = v35;
        let v36: bool = (p.p7 == 0.0);
        self.scalar_v36 = v36;
        let v37: bool = (p.p7 == 1.0);
        self.scalar_v37 = v37;
        let v38: bool = (!v36);
        self.scalar_v38 = v38;
        let v39: bool = (v37 && v38);
        self.scalar_v39 = v39;
        let v40: f64 = p.p0;
        self.scalar_v40 = v40;
        let v41: bool = (v39 && (p.p0 != 0.0));
        self.scalar_v41 = v41;
        let v42: bool = (p.p1 == 1.0);
        self.scalar_v42 = v42;
        let v94: bool = (!v42);
        self.scalar_v94 = v94;
        let v101: f64 = (-1.0 / p.p60);
        self.scalar_v101 = v101;
        let v102: f64 = (1.0 / p.p60);
        self.scalar_v102 = v102;
        let v103: f64 = (if v24 { v101 } else { 0.0 });
        self.scalar_v103 = v103;
        let v104: f64 = (if v24 { v102 } else { 0.0 });
        self.scalar_v104 = v104;
        let v105: f64 = (-1.0 / p.p51);
        self.scalar_v105 = v105;
        let v106: f64 = (1.0 / p.p51);
        self.scalar_v106 = v106;
        let v107: f64 = (if v26 { v105 } else { 0.0 });
        self.scalar_v107 = v107;
        let v108: f64 = (if v26 { v106 } else { 0.0 });
        self.scalar_v108 = v108;
        let v109: f64 = (-1.0 / p.p49);
        self.scalar_v109 = v109;
        let v110: f64 = (1.0 / p.p49);
        self.scalar_v110 = v110;
        let v111: f64 = (if v28 { v109 } else { 0.0 });
        self.scalar_v111 = v111;
        let v112: f64 = (if v28 { v110 } else { 0.0 });
        self.scalar_v112 = v112;
        let v113: f64 = (if v41 { 1.0 } else { 0.0 });
        self.scalar_v113 = v113;
        let v114: f64 = (if v94 { 1e-12 } else { 0.0 });
        self.scalar_v114 = v114;
    }
}
