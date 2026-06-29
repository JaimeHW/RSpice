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
    pub p128: f64,
    pub p129: f64,
    pub p130: f64,
    pub p131: f64,
    pub p132: f64,
    pub p133: f64,
    pub p134: f64,
    pub p135: f64,
    pub p136: f64,
    pub p137: f64,
    pub p138: f64,
    pub p139: f64,
    pub p140: f64,
    pub p141: f64,
    pub p142: f64,
    pub p143: f64,
    pub p144: f64,
    pub p145: f64,
    pub p146: f64,
    pub p147: f64,
    pub p148: f64,
    pub p149: f64,
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
            params.p0 = 310.0;
            params.p1 = 2e-30;
            params.p2 = 2e-14;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 1.0;
            params.p10 = 0.0;
            params.p11 = 1.0;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 1e-18;
            params.p15 = 1.0;
            params.p16 = 0.0;
            params.p17 = 2.0;
            params.p18 = 0.0;
            params.p19 = 1.0;
            params.p20 = 0.0;
            params.p21 = 2.0;
            params.p22 = 0.0;
            params.p23 = 1e-16;
            params.p24 = 1.0;
            params.p25 = 0.0;
            params.p26 = 1.0;
            params.p27 = 0.0;
            params.p28 = 40.0;
            params.p29 = 1.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = 0.0;
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 0.0;
            params.p38 = 40.0;
            params.p39 = 1e-20;
            params.p40 = 0.9;
            params.p41 = 0.5;
            params.p42 = 2.5;
            params.p43 = 1e-20;
            params.p44 = 0.9;
            params.p45 = 0.5;
            params.p46 = 2.5;
            params.p47 = 1e-20;
            params.p48 = 0.7;
            params.p49 = 0.4;
            params.p50 = 2.4;
            params.p51 = 100.0;
            params.p52 = 1e-20;
            params.p53 = 0.7;
            params.p54 = 0.4;
            params.p55 = 2.4;
            params.p56 = 100.0;
            params.p57 = 0.0;
            params.p58 = 0.6;
            params.p59 = 0.5;
            params.p60 = 2.4;
            params.p61 = 100.0;
            params.p62 = 0.0;
            params.p63 = 0.6;
            params.p64 = 0.5;
            params.p65 = 100.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 1.0;
            params.p71 = 0.0;
            params.p72 = 0.1;
            params.p73 = 0.0;
            params.p74 = 150.0;
            params.p75 = 0.5;
            params.p76 = 100.0;
            params.p77 = 2.0;
            params.p78 = 0.1;
            params.p79 = 0.0;
            params.p80 = 1.921812;
            params.p81 = 0.001;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.01;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.167;
            params.p88 = 0.333;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 0.6557;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 1.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 0.0;
            params.p98 = 1.0;
            params.p99 = 0.0;
            params.p100 = 1.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 1.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 2.0;
            params.p112 = -1.0;
            params.p113 = 0.0;
            params.p114 = 2.0;
            params.p115 = 0.0;
            params.p116 = 0.0;
            params.p117 = 1.17;
            params.p118 = 1.17;
            params.p119 = 1.17;
            params.p120 = 1.17;
            params.p121 = -0.000102377;
            params.p122 = 0.00043215;
            params.p123 = 3.0;
            params.p124 = 3.5;
            params.p125 = 0.0;
            params.p126 = 1.0;
            params.p127 = 1.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 0.0;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 0.0;
            params.p138 = 1.0;
            params.p139 = 0.0;
            params.p140 = 0.0;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = 0.0;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = 27.0;
            params.p147 = 0.0;
            params.p148 = 1.0;
            params.p149 = 0.001;
            validate_parameter("minr", params.p149, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
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
    pub nodes: [usize; 15],
    pub branches: [usize; 6],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 150]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 20]>,
    pub(crate) ddt_state_previous: Box<[f64; 20]>,
    pub(crate) ddt_state_initialized: Box<[bool; 20]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: bool,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: bool,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: bool,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: bool,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: bool,
    pub(crate) scalar_v37: bool,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: bool,
    pub(crate) scalar_v40: bool,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: bool,
    pub(crate) scalar_v52: bool,
    pub(crate) scalar_v53: bool,
    pub(crate) scalar_v54: bool,
    pub(crate) scalar_v55: bool,
    pub(crate) scalar_v72: bool,
    pub(crate) scalar_v76: bool,
    pub(crate) scalar_v77: bool,
    pub(crate) scalar_v78: bool,
    pub(crate) scalar_v79: bool,
    pub(crate) scalar_v80: bool,
    pub(crate) scalar_v81: bool,
    pub(crate) scalar_v82: bool,
    pub(crate) scalar_v83: bool,
    pub(crate) scalar_v84: bool,
    pub(crate) scalar_v85: bool,
    pub(crate) scalar_v86: bool,
    pub(crate) scalar_v87: bool,
    pub(crate) scalar_v89: bool,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: bool,
    pub(crate) scalar_v92: bool,
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: bool,
    pub(crate) scalar_v95: bool,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: bool,
    pub(crate) scalar_v101: bool,
    pub(crate) scalar_v102: bool,
    pub(crate) scalar_v103: bool,
    pub(crate) scalar_v104: bool,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v112: bool,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: bool,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v123: bool,
    pub(crate) scalar_v126: bool,
    pub(crate) scalar_v127: bool,
    pub(crate) scalar_v132: bool,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: bool,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v144: bool,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scratch: Option<Box<GenericScratch<572, 15, 6>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<572, 15, 6>>>,
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
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v45: self.scalar_v45,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v72: self.scalar_v72,
            scalar_v76: self.scalar_v76,
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v83: self.scalar_v83,
            scalar_v84: self.scalar_v84,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v87: self.scalar_v87,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v121: self.scalar_v121,
            scalar_v123: self.scalar_v123,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v144: self.scalar_v144,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 15;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["ci", "ei", "bp", "bi", "si", "xf1", "xf2", "xf", "n1", "n2"];

    pub const BRANCH_COUNT: usize = 6;
    pub const PARAMETER_COUNT: usize = 150;
    pub const VARIABLE_COUNT: usize = 572;
    pub const DDT_STATE_COUNT: usize = 20;
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
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v20: false,
            scalar_v21: 0.0,
            scalar_v22: 0.0,
            scalar_v23: false,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v32: false,
            scalar_v33: 0.0,
            scalar_v34: false,
            scalar_v35: 0.0,
            scalar_v36: false,
            scalar_v37: false,
            scalar_v38: 0.0,
            scalar_v39: false,
            scalar_v40: false,
            scalar_v41: false,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v51: false,
            scalar_v52: false,
            scalar_v53: false,
            scalar_v54: false,
            scalar_v55: false,
            scalar_v72: false,
            scalar_v76: false,
            scalar_v77: false,
            scalar_v78: false,
            scalar_v79: false,
            scalar_v80: false,
            scalar_v81: false,
            scalar_v82: false,
            scalar_v83: false,
            scalar_v84: false,
            scalar_v85: false,
            scalar_v86: false,
            scalar_v87: false,
            scalar_v89: false,
            scalar_v90: 0.0,
            scalar_v91: false,
            scalar_v92: false,
            scalar_v93: false,
            scalar_v94: false,
            scalar_v95: false,
            scalar_v96: false,
            scalar_v99: 0.0,
            scalar_v100: false,
            scalar_v101: false,
            scalar_v102: false,
            scalar_v103: false,
            scalar_v104: false,
            scalar_v105: 0.0,
            scalar_v108: false,
            scalar_v109: 0.0,
            scalar_v112: false,
            scalar_v113: 0.0,
            scalar_v114: false,
            scalar_v115: 0.0,
            scalar_v116: false,
            scalar_v117: 0.0,
            scalar_v121: 0.0,
            scalar_v123: false,
            scalar_v126: false,
            scalar_v127: false,
            scalar_v132: false,
            scalar_v133: 0.0,
            scalar_v134: false,
            scalar_v135: 0.0,
            scalar_v144: false,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v166: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: None,
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
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v72,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v108,
            scalar_v109,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v121,
            scalar_v123,
            scalar_v126,
            scalar_v127,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v144,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
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
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v72,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v108,
            scalar_v109,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v121,
            scalar_v123,
            scalar_v126,
            scalar_v127,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v144,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "c10" => { validate_parameter("c10", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "qp0" => { validate_parameter("qp0", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "hf0" => { validate_parameter("hf0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "hfe" => { validate_parameter("hfe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "hfb" => { validate_parameter("hfb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "hfc" => { validate_parameter("hfc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "hr0" => { validate_parameter("hr0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "hjei0" => { validate_parameter("hjei0", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "hjei" => { validate_parameter("hjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "ahjei" => { validate_parameter("ahjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "rhjei" => { validate_parameter("rhjei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "hjci" => { validate_parameter("hjci", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "ibeis" => { validate_parameter("ibeis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "mbei" => { validate_parameter("mbei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "ireis" => { validate_parameter("ireis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "mrei" => { validate_parameter("mrei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "ibeps" => { validate_parameter("ibeps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "mbep" => { validate_parameter("mbep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "ireps" => { validate_parameter("ireps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "mrep" => { validate_parameter("mrep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "tbhrec" => { validate_parameter("tbhrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "ibcis" => { validate_parameter("ibcis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "mbci" => { validate_parameter("mbci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ibcxs" => { validate_parameter("ibcxs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "mbcx" => { validate_parameter("mbcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "ibets" => { validate_parameter("ibets", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "abet" => { validate_parameter("abet", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "tunode" => { validate_parameter("tunode", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ibetat0" => { validate_parameter("ibetat0", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "vbetat" => { validate_parameter("vbetat", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "hcavl" => { validate_parameter("hcavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "hvdavl" => { validate_parameter("hvdavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ibcts" => { validate_parameter("ibcts", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "abct" => { validate_parameter("abct", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "cjei0" => { validate_parameter("cjei0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "vdei" => { validate_parameter("vdei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "zei" => { validate_parameter("zei", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "ajei" => { validate_parameter("ajei", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "cjep0" => { validate_parameter("cjep0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "vdep" => { validate_parameter("vdep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "zep" => { validate_parameter("zep", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "ajep" => { validate_parameter("ajep", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "ajci" => { validate_parameter("ajci", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "ajcx" => { validate_parameter("ajcx", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "ajs" => { validate_parameter("ajs", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "cscp0" => { validate_parameter("cscp0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "vdsp" => { validate_parameter("vdsp", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "zsp" => { validate_parameter("zsp", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "vptsp" => { validate_parameter("vptsp", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "tbvl" => { validate_finite_parameter("tbvl", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "gtfe" => { validate_parameter("gtfe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "fthc" => { validate_parameter("fthc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.2, "1.2")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "avcsm" => { validate_parameter("avcsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "vcbar" => { validate_parameter("vcbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "icbar" => { validate_parameter("icbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "acbar" => { validate_parameter("acbar", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "fdqr0" => { validate_parameter("fdqr0", value, Some((-0.5, "-0.5")), false, Some((100.0, "100.0")), false, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "fcrbi" => { validate_parameter("fcrbi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "fqi" => { validate_parameter("fqi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "tsf" => { validate_parameter("tsf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "rsu" => { validate_parameter("rsu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "csu" => { validate_parameter("csu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "fbepar" => { validate_parameter("fbepar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "fbcpar" => { validate_parameter("fbcpar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "ccepar" => { validate_parameter("ccepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "flcono" => { validate_parameter("flcono", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "cfbe" => { validate_parameter("cfbe", value, Some((-2.0, "-2.0")), false, Some((-1.0, "-1.0")), false, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "kfre" => { validate_parameter("kfre", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "afre" => { validate_parameter("afre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "latb" => { validate_parameter("latb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "latl" => { validate_parameter("latl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "f2vg" => { validate_finite_parameter("f2vg", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "zetact" => { validate_parameter("zetact", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "zetabet" => { validate_parameter("zetabet", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "dvgbe" => { validate_parameter("dvgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "zetahjei" => { validate_parameter("zetahjei", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "zetavgbe" => { validate_parameter("zetavgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "zetaci" => { validate_parameter("zetaci", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "zetarbi" => { validate_parameter("zetarbi", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "zetarbx" => { validate_parameter("zetarbx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "zetarcx" => { validate_parameter("zetarcx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "zetare" => { validate_parameter("zetare", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "zetacx" => { validate_parameter("zetacx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "zetarth" => { validate_parameter("zetarth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("tnom", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hicumL2va'", name)),
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
        let v9: f64 = p.p0;
        self.scalar_v9 = v9;
        let v14: f64 = p.p107;
        self.scalar_v14 = v14;
        let v15: f64 = (1.0 - p.p107);
        self.scalar_v15 = v15;
        let v16: f64 = p.p52;
        self.scalar_v16 = v16;
        let v17: f64 = p.p106;
        self.scalar_v17 = v17;
        let v18: f64 = (p.p52 + p.p106);
        self.scalar_v18 = v18;
        let v19: f64 = (v15 * v18);
        self.scalar_v19 = v19;
        let v20: bool = (v19 >= p.p106);
        self.scalar_v20 = v20;
        let v21: f64 = (if v20 { p.p106 } else { 0.0 });
        self.scalar_v21 = v21;
        let v22: f64 = (if v20 { 0.0 } else { 0.0 });
        self.scalar_v22 = v22;
        let v23: bool = (!v20);
        self.scalar_v23 = v23;
        let v24: f64 = (if v23 { v19 } else { v21 });
        self.scalar_v24 = v24;
        let v25: f64 = (p.p106 - v24);
        self.scalar_v25 = v25;
        let v26: f64 = (if v23 { v25 } else { v22 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p105;
        self.scalar_v27 = v27;
        let v28: f64 = p.p104;
        self.scalar_v28 = v28;
        let v29: f64 = (p.p105 * p.p104);
        self.scalar_v29 = v29;
        let v30: f64 = (p.p104 - v29);
        self.scalar_v30 = v30;
        let v31: f64 = p.p86;
        self.scalar_v31 = v31;
        let v32: bool = (0.0 != p.p86);
        self.scalar_v32 = v32;
        let v33: f64 = p.p88;
        self.scalar_v33 = v33;
        let v34: bool = (0.0 == p.p88);
        self.scalar_v34 = v34;
        let v35: f64 = p.p87;
        self.scalar_v35 = v35;
        let v36: bool = (0.0 == p.p87);
        self.scalar_v36 = v36;
        let v37: bool = (v34 && v36);
        self.scalar_v37 = v37;
        let v38: f64 = p.p66;
        self.scalar_v38 = v38;
        let v39: bool = (0.0 == p.p66);
        self.scalar_v39 = v39;
        let v40: bool = (v37 || v39);
        self.scalar_v40 = v40;
        let v41: bool = (v32 && v40);
        self.scalar_v41 = v41;
        let v42: f64 = (if v41 { 0.0 } else { p.p86 });
        self.scalar_v42 = v42;
        let v43: f64 = p.p89;
        self.scalar_v43 = v43;
        let v44: f64 = p.p99;
        self.scalar_v44 = v44;
        let v45: f64 = p.p96;
        self.scalar_v45 = v45;
        let v46: f64 = p.p90;
        self.scalar_v46 = v46;
        let v47: f64 = p.p95;
        self.scalar_v47 = v47;
        let v48: f64 = p.p142;
        self.scalar_v48 = v48;
        let v49: f64 = p.p141;
        self.scalar_v49 = v49;
        let v50: f64 = p.p149;
        self.scalar_v50 = v50;
        let v51: bool = (p.p142 >= p.p149);
        self.scalar_v51 = v51;
        let v52: bool = (p.p142 > 0.0);
        self.scalar_v52 = v52;
        let v53: bool = (p.p0 >= 310.0);
        self.scalar_v53 = v53;
        let v54: bool = (p.p99 > 0.0);
        self.scalar_v54 = v54;
        let v55: bool = (0.0 != v42);
        self.scalar_v55 = v55;
        let v72: bool = (!v55);
        self.scalar_v72 = v72;
        let v76: bool = (p.p89 >= p.p149);
        self.scalar_v76 = v76;
        let v77: bool = (p.p89 > 0.0);
        self.scalar_v77 = v77;
        let v78: bool = (v76 && v77);
        self.scalar_v78 = v78;
        let v79: bool = (p.p90 >= p.p149);
        self.scalar_v79 = v79;
        let v80: bool = (p.p90 > 0.0);
        self.scalar_v80 = v80;
        let v81: bool = (v79 && v80);
        self.scalar_v81 = v81;
        let v82: bool = (p.p95 >= p.p149);
        self.scalar_v82 = v82;
        let v83: bool = (p.p95 > 0.0);
        self.scalar_v83 = v83;
        let v84: bool = (v82 && v83);
        self.scalar_v84 = v84;
        let v85: bool = (p.p96 >= p.p149);
        self.scalar_v85 = v85;
        let v86: bool = (p.p96 > 0.0);
        self.scalar_v86 = v86;
        let v87: bool = (v85 && v86);
        self.scalar_v87 = v87;
        let v89: bool = (p.p0 >= 320.0);
        self.scalar_v89 = v89;
        let v90: f64 = p.p102;
        self.scalar_v90 = v90;
        let v91: bool = (p.p102 >= p.p149);
        self.scalar_v91 = v91;
        let v92: bool = (p.p102 > 0.0);
        self.scalar_v92 = v92;
        let v93: bool = (v91 && v92);
        self.scalar_v93 = v93;
        let v94: bool = (p.p141 >= 1.0);
        self.scalar_v94 = v94;
        let v95: bool = (v51 && v94);
        self.scalar_v95 = v95;
        let v96: bool = (v52 && v95);
        self.scalar_v96 = v96;
        let v99: f64 = p.p109;
        self.scalar_v99 = v99;
        let v100: bool = (1.0 == p.p109);
        self.scalar_v100 = v100;
        let v101: bool = (p.p88 > 0.0);
        self.scalar_v101 = v101;
        let v102: bool = (p.p87 > 0.0);
        self.scalar_v102 = v102;
        let v103: bool = (v101 && v102);
        self.scalar_v103 = v103;
        let v104: bool = (v100 && v103);
        self.scalar_v104 = v104;
        let v105: f64 = (if v104 { 1.0 } else { 0.0 });
        self.scalar_v105 = v105;
        let v108: bool = (!v78);
        self.scalar_v108 = v108;
        let v109: f64 = (if v108 { 0.0 } else { 0.0 });
        self.scalar_v109 = v109;
        let v112: bool = (!v81);
        self.scalar_v112 = v112;
        let v113: f64 = (if v112 { 0.0 } else { 0.0 });
        self.scalar_v113 = v113;
        let v114: bool = (!v84);
        self.scalar_v114 = v114;
        let v115: f64 = (if v114 { 0.0 } else { 0.0 });
        self.scalar_v115 = v115;
        let v116: bool = (!v87);
        self.scalar_v116 = v116;
        let v117: f64 = (if v116 { 0.0 } else { 0.0 });
        self.scalar_v117 = v117;
        let v121: f64 = p.p108;
        self.scalar_v121 = v121;
        let v123: bool = (v54 && v89);
        self.scalar_v123 = v123;
        let v126: bool = (!v89);
        self.scalar_v126 = v126;
        let v127: bool = (v53 && v126);
        self.scalar_v127 = v127;
        let v132: bool = (!v93);
        self.scalar_v132 = v132;
        let v133: f64 = (if v132 { 0.0 } else { 0.0 });
        self.scalar_v133 = v133;
        let v134: bool = (!v96);
        self.scalar_v134 = v134;
        let v135: f64 = (if v134 { 0.0 } else { 0.0 });
        self.scalar_v135 = v135;
        let v144: bool = (!v104);
        self.scalar_v144 = v144;
        let v147: f64 = (if v55 { 1.0 } else { 0.0 });
        self.scalar_v147 = v147;
        let v148: f64 = (p.p88 * v147);
        self.scalar_v148 = v148;
        let v149: f64 = (p.p66 * v148);
        self.scalar_v149 = v149;
        let v150: f64 = (if v55 { v149 } else { 0.0 });
        self.scalar_v150 = v150;
        let v151: f64 = (v148 / 3.0);
        self.scalar_v151 = v151;
        let v152: f64 = (p.p66 * v151);
        self.scalar_v152 = v152;
        let v153: f64 = (if v55 { v152 } else { 0.0 });
        self.scalar_v153 = v153;
        let v154: f64 = (p.p87 * v147);
        self.scalar_v154 = v154;
        let v155: f64 = (p.p66 * v154);
        self.scalar_v155 = v155;
        let v156: f64 = (if v55 { v155 } else { 0.0 });
        self.scalar_v156 = v156;
        let v157: f64 = (if v72 { 0.0 } else { v150 });
        self.scalar_v157 = v157;
        let v158: f64 = (if v72 { 0.0 } else { v153 });
        self.scalar_v158 = v158;
        let v159: f64 = (if v72 { 0.0 } else { v156 });
        self.scalar_v159 = v159;
        let v160: f64 = (-v26);
        self.scalar_v160 = v160;
        let v161: f64 = (-v24);
        self.scalar_v161 = v161;
        let v162: f64 = (-v29);
        self.scalar_v162 = v162;
        let v163: f64 = (-v30);
        self.scalar_v163 = v163;
        let v164: f64 = (-p.p108);
        self.scalar_v164 = v164;
        let v166: f64 = (if v123 { -0.0 } else { 0.0 });
        self.scalar_v166 = v166;
        let v167: f64 = (if v123 { 0.0 } else { 0.0 });
        self.scalar_v167 = v167;
        let v168: f64 = (if v127 { -0.0 } else { 0.0 });
        self.scalar_v168 = v168;
        let v169: f64 = (if v127 { 0.0 } else { 0.0 });
        self.scalar_v169 = v169;
        let v170: f64 = (-1.0 / p.p102);
        self.scalar_v170 = v170;
        let v171: f64 = (1.0 / p.p102);
        self.scalar_v171 = v171;
        let v172: f64 = (if v93 { v170 } else { 0.0 });
        self.scalar_v172 = v172;
        let v173: f64 = (if v93 { v171 } else { 0.0 });
        self.scalar_v173 = v173;
        let v174: f64 = (if v104 { -1.0 } else { 0.0 });
        self.scalar_v174 = v174;
        let v175: f64 = (if v144 { 1.0 } else { 0.0 });
        self.scalar_v175 = v175;
    }
}
