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
    pub p150: f64,
    pub p151: f64,
    pub p152: f64,
    pub p153: f64,
    pub p154: f64,
    pub p155: f64,
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
            params.p0 = 0.0;
            params.p1 = 1.0;
            params.p2 = 505.5;
            params.p3 = 1.0;
            params.p4 = 25.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 1.0;
            params.p9 = 2.2e-17;
            params.p10 = 1.0;
            params.p11 = 1.0;
            params.p12 = 0.1;
            params.p13 = 2.5;
            params.p14 = 44.0;
            params.p15 = 1.0;
            params.p16 = 1.0000000000000001e-19;
            params.p17 = 1.0;
            params.p18 = 0.0;
            params.p19 = 1.0;
            params.p20 = 2.7000000000000005e-15;
            params.p21 = 2.0;
            params.p22 = 0.0;
            params.p23 = 2.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.68;
            params.p28 = 0.0;
            params.p29 = 3.1400000000000002e-18;
            params.p30 = 0.014289999999999999;
            params.p31 = 1e-15;
            params.p32 = 2.0;
            params.p33 = 0.63;
            params.p34 = 0.0;
            params.p35 = 22.0;
            params.p36 = 0.0;
            params.p37 = 22.0;
            params.p38 = 1e-6;
            params.p39 = 1.0;
            params.p40 = 400.0;
            params.p41 = -0.37;
            params.p42 = 0.5;
            params.p43 = 25.0;
            params.p44 = 0.1;
            params.p45 = 1.1e-6;
            params.p46 = 3.0;
            params.p47 = 0.3;
            params.p48 = 0.004;
            params.p49 = -0.37;
            params.p50 = -0.37;
            params.p51 = 0.3;
            params.p52 = 0.004;
            params.p53 = 1.0;
            params.p54 = 5.0;
            params.p55 = 23.0;
            params.p56 = 18.0;
            params.p57 = 12.0;
            params.p58 = 0.0;
            params.p59 = 0.0;
            params.p60 = 150.0;
            params.p61 = 1250.0;
            params.p62 = 0.004;
            params.p63 = 0.3;
            params.p64 = 0.68;
            params.p65 = 7.3e-14;
            params.p66 = 0.95;
            params.p67 = 0.4;
            params.p68 = 0.4;
            params.p69 = 0.0;
            params.p70 = 7.800000000000001e-14;
            params.p71 = 0.68;
            params.p72 = 0.5;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 0.35;
            params.p76 = 0.5;
            params.p77 = 0.032;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.68;
            params.p81 = 100.0;
            params.p82 = 4.0;
            params.p83 = 1000.0;
            params.p84 = 0.0;
            params.p85 = 1.0;
            params.p86 = 2e-12;
            params.p87 = 4.2e-12;
            params.p88 = 4.1e-11;
            params.p89 = 5.2e-10;
            params.p90 = 1e-11;
            params.p91 = 1.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.3333333333333333;
            params.p95 = 0.0;
            params.p96 = 0.3;
            params.p97 = 0.0;
            params.p98 = 1.0;
            params.p99 = 2.5;
            params.p100 = 2.5;
            params.p101 = 0.62;
            params.p102 = 2.0;
            params.p103 = 1.3;
            params.p104 = 2.0;
            params.p105 = 1.17;
            params.p106 = 1.12;
            params.p107 = 1.12;
            params.p108 = 1.12;
            params.p109 = 1.12;
            params.p110 = 1.18;
            params.p111 = 1.12;
            params.p112 = 1.125;
            params.p113 = 1.15;
            params.p114 = 1.15;
            params.p115 = 0.000473;
            params.p116 = 636.0;
            params.p117 = 1.15;
            params.p118 = 0.000473;
            params.p119 = 636.0;
            params.p120 = 0.05;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0005;
            params.p125 = 200.0;
            params.p126 = 2.0;
            params.p127 = 2.0;
            params.p128 = 2e-11;
            params.p129 = 2e-11;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 4.8000000000000003e-17;
            params.p134 = 0.0;
            params.p135 = 0.0005455;
            params.p136 = 4.9999999999999996e-5;
            params.p137 = 3.15e-13;
            params.p138 = 0.62;
            params.p139 = 0.34;
            params.p140 = 1.2;
            params.p141 = 1.58;
            params.p142 = 2.0;
            params.p143 = 0.0;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = 2.0;
            params.p147 = 400.0;
            params.p148 = 1e-40;
            params.p149 = 1e-40;
            params.p150 = 0.001;
            validate_parameter("minr", params.p150, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p151 = 0.0;
            params.p152 = 1.0;
            params.p153 = 0.0;
            params.p154 = 0.16;
            params.p155 = 0.0;
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
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 156]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 10]>,
    pub(crate) ddt_state_previous: Box<[f64; 10]>,
    pub(crate) ddt_state_initialized: Box<[bool; 10]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v2: bool,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: bool,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: bool,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: bool,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: bool,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: bool,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: bool,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: bool,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: bool,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: bool,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v343: bool,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: bool,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v395: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: bool,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v566: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v585: f64,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v590: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v629: f64,
    pub(crate) scalar_v631: bool,
    pub(crate) scalar_v637: bool,
    pub(crate) scalar_v639: bool,
    pub(crate) scalar_v645: bool,
    pub(crate) scalar_v647: bool,
    pub(crate) scalar_v653: bool,
    pub(crate) scalar_v701: f64,
    pub(crate) scalar_v706: f64,
    pub(crate) scalar_v824: f64,
    pub(crate) scalar_v878: f64,
    pub(crate) scalar_v879: f64,
    pub(crate) scalar_v880: f64,
    pub(crate) scalar_v891: f64,
    pub(crate) scalar_v912: f64,
    pub(crate) scalar_v913: f64,
    pub(crate) scalar_v914: f64,
    pub(crate) scalar_v915: f64,
    pub(crate) scalar_v916: f64,
    pub(crate) scalar_v917: f64,
    pub(crate) scalar_v964: f64,
    pub(crate) scalar_v974: f64,
    pub(crate) scalar_v987: f64,
    pub(crate) scalar_v988: bool,
    pub(crate) scalar_v992: bool,
    pub(crate) scalar_v1043: f64,
    pub(crate) scalar_v1044: f64,
    pub(crate) scalar_v1045: f64,
    pub(crate) scalar_v1067: f64,
    pub(crate) scalar_v1075: f64,
    pub(crate) scalar_v1076: bool,
    pub(crate) scalar_v1078: bool,
    pub(crate) scalar_v1079: bool,
    pub(crate) scalar_v1080: bool,
    pub(crate) scalar_v1083: bool,
    pub(crate) scalar_v1084: bool,
    pub(crate) scalar_v1089: f64,
    pub(crate) scalar_v1110: f64,
    pub(crate) scalar_v1112: f64,
    pub(crate) scalar_v1141: bool,
    pub(crate) scalar_v1147: bool,
    pub(crate) scalar_v1182: f64,
    pub(crate) scalar_v1206: f64,
    pub(crate) scalar_v1219: f64,
    pub(crate) scalar_v1237: f64,
    pub(crate) scalar_v1300: f64,
    pub(crate) scalar_v1301: bool,
    pub(crate) scalar_v1302: bool,
    pub(crate) scalar_v1303: bool,
    pub(crate) scalar_v1305: bool,
    pub(crate) scalar_v1306: bool,
    pub(crate) scalar_v1307: f64,
    pub(crate) scalar_v1400: bool,
    pub(crate) scalar_v1401: bool,
    pub(crate) scalar_v1402: bool,
    pub(crate) scalar_v1426: f64,
    pub(crate) scalar_v1428: f64,
    pub(crate) scalar_v1429: f64,
    pub(crate) scalar_v1431: f64,
    pub(crate) scalar_v1491: bool,
    pub(crate) scalar_v1492: bool,
    pub(crate) scalar_v1493: bool,
    pub(crate) scalar_v1519: f64,
    pub(crate) scalar_v1521: f64,
    pub(crate) scalar_v1522: f64,
    pub(crate) scalar_v1524: f64,
    pub(crate) scalar_v1590: f64,
    pub(crate) scalar_v1591: bool,
    pub(crate) scalar_v1592: f64,
    pub(crate) scalar_v1593: f64,
    pub(crate) scalar_v1599: f64,
    pub(crate) scalar_v1608: f64,
    pub(crate) scalar_v1609: f64,
    pub(crate) scalar_v1621: bool,
    pub(crate) scalar_v1640: f64,
    pub(crate) scalar_v1650: f64,
    pub(crate) scalar_v1651: bool,
    pub(crate) scalar_v1652: bool,
    pub(crate) scalar_v1653: bool,
    pub(crate) scalar_v1658: f64,
    pub(crate) scalar_v1668: bool,
    pub(crate) scalar_v1669: f64,
    pub(crate) scalar_v1670: f64,
    pub(crate) scalar_v1684: bool,
    pub(crate) scalar_v1692: bool,
    pub(crate) scalar_v1693: bool,
    pub(crate) scalar_v1706: f64,
    pub(crate) scalar_v1711: f64,
    pub(crate) scalar_v1728: bool,
    pub(crate) scalar_v1729: bool,
    pub(crate) scalar_v1735: f64,
    pub(crate) scalar_v1736: bool,
    pub(crate) scalar_v1739: f64,
    pub(crate) scalar_v1746: f64,
    pub(crate) scalar_v1757: f64,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1760: f64,
    pub(crate) scalar_v1761: f64,
    pub(crate) scalar_v1762: f64,
    pub(crate) scalar_v1763: f64,
    pub(crate) scalar_v1764: f64,
    pub(crate) scalar_v1765: f64,
    pub(crate) scalar_v1766: f64,
    pub(crate) scalar_v1767: f64,
    pub(crate) scalar_v1768: f64,
    pub(crate) scalar_v1769: f64,
    pub(crate) scalar_v1770: f64,
    pub(crate) scalar_v1771: f64,
    pub(crate) scalar_v1785: bool,
    pub(crate) scalar_v1812: f64,
    pub(crate) scalar_v1813: bool,
    pub(crate) scalar_v1814: f64,
    pub(crate) scalar_v1817: f64,
    pub(crate) scalar_v1836: f64,
    pub(crate) scalar_v1850: f64,
    pub(crate) scalar_v1855: bool,
    pub(crate) scalar_v1857: bool,
    pub(crate) scalar_v1861: f64,
    pub(crate) scalar_v1862: f64,
    pub(crate) scalar_v1863: f64,
    pub(crate) scalar_v1864: f64,
    pub(crate) scalar_v1865: f64,
    pub(crate) scalar_v1874: f64,
    pub(crate) scalar_v1875: bool,
    pub(crate) scalar_v1878: bool,
    pub(crate) scalar_v1901: f64,
    pub(crate) scalar_v1902: f64,
    pub(crate) scalar_v1908: f64,
    pub(crate) scalar_v1909: f64,
    pub(crate) scalar_v1910: f64,
    pub(crate) scalar_v1958: bool,
    pub(crate) scalar_v1959: bool,
    pub(crate) scalar_v1964: f64,
    pub(crate) scalar_v1968: f64,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1980: f64,
    pub(crate) scalar_v2000: f64,
    pub(crate) scalar_v2020: f64,
    pub(crate) scalar_v2021: bool,
    pub(crate) scalar_v2056: bool,
    pub(crate) scalar_v2062: bool,
    pub(crate) scalar_v2067: f64,
    pub(crate) scalar_v2068: bool,
    pub(crate) scalar_v2072: bool,
    pub(crate) scalar_v2128: f64,
    pub(crate) scalar_v2133: f64,
    pub(crate) scalar_v2136: f64,
    pub(crate) scalar_v2137: f64,
    pub(crate) scalar_v2138: f64,
    pub(crate) scalar_v2139: f64,
    pub(crate) scalar_v2140: f64,
    pub(crate) scalar_v2141: f64,
    pub(crate) scalar_v2142: f64,
    pub(crate) scalar_v2143: f64,
    pub(crate) scalar_v2144: f64,
    pub(crate) scalar_v2858: f64,
    pub(crate) scalar_v2873: f64,
    pub(crate) scalar_v2874: f64,
    pub(crate) scalar_v2941: f64,
    pub(crate) scalar_v2953: f64,
    pub(crate) scalar_v3178: f64,
    pub(crate) scalar_v3179: f64,
    pub(crate) scalar_v3188: f64,
    pub(crate) scalar_v3189: f64,
    pub(crate) scalar_v3212: f64,
    pub(crate) scalar_v3213: f64,
    pub(crate) scalar_v3224: f64,
    pub(crate) scalar_v3225: f64,
    pub(crate) scalar_v3561: f64,
    pub(crate) scalar_v3600: f64,
    pub(crate) scalar_v3601: f64,
    pub(crate) scalar_v3644: f64,
    pub(crate) scalar_v3645: f64,
    pub(crate) scalar_v3734: f64,
    pub(crate) scalar_v3773: f64,
    pub(crate) scalar_v3774: f64,
    pub(crate) scalar_v4065: f64,
    pub(crate) scalar_v4066: f64,
    pub(crate) scalar_v4262: f64,
    pub(crate) scalar_v4263: f64,
    pub(crate) scalar_v4264: f64,
    pub(crate) scalar_v4265: f64,
    pub(crate) scalar_v4266: f64,
    pub(crate) scalar_v4267: f64,
    pub(crate) scalar_v4530: f64,
    pub(crate) scalar_v4531: f64,
    pub(crate) scalar_v4532: f64,
    pub(crate) scalar_v4533: f64,
    pub(crate) scalar_v4534: f64,
    pub(crate) scalar_v4535: f64,
    pub(crate) scalar_v4536: f64,
    pub(crate) scalar_v4537: f64,
    pub(crate) scalar_v4538: f64,
    pub(crate) scalar_v4931: f64,
    pub(crate) scalar_v5403: f64,
    pub(crate) scalar_v5482: f64,
    pub(crate) scalar_v5718: f64,
    pub(crate) scalar_v5719: f64,
    pub(crate) scalar_v5720: f64,
    pub(crate) scalar_v5721: f64,
    pub(crate) scalar_v5722: f64,
    pub(crate) scalar_v5723: f64,
    pub(crate) scalar_v5884: f64,
    pub(crate) scalar_v5885: f64,
    pub(crate) scalar_v5914: f64,
    pub(crate) scalar_v5915: f64,
    pub(crate) scalar_v5916: f64,
    pub(crate) scalar_v5917: f64,
    pub(crate) scalar_v5918: f64,
    pub(crate) scalar_v5919: f64,
    pub(crate) scratch: Option<Box<GenericScratch<616, 12, 2>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<616, 12, 2>>>,
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
            scalar_v0: self.scalar_v0,
            scalar_v2: self.scalar_v2,
            scalar_v5: self.scalar_v5,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v10: self.scalar_v10,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v17: self.scalar_v17,
            scalar_v19: self.scalar_v19,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
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
            scalar_v50: self.scalar_v50,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v72: self.scalar_v72,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
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
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v110: self.scalar_v110,
            scalar_v112: self.scalar_v112,
            scalar_v167: self.scalar_v167,
            scalar_v187: self.scalar_v187,
            scalar_v190: self.scalar_v190,
            scalar_v230: self.scalar_v230,
            scalar_v233: self.scalar_v233,
            scalar_v253: self.scalar_v253,
            scalar_v256: self.scalar_v256,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v275: self.scalar_v275,
            scalar_v276: self.scalar_v276,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v306: self.scalar_v306,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
            scalar_v313: self.scalar_v313,
            scalar_v314: self.scalar_v314,
            scalar_v315: self.scalar_v315,
            scalar_v343: self.scalar_v343,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v374: self.scalar_v374,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v395: self.scalar_v395,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v405: self.scalar_v405,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v421: self.scalar_v421,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v442: self.scalar_v442,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v451: self.scalar_v451,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v479: self.scalar_v479,
            scalar_v480: self.scalar_v480,
            scalar_v481: self.scalar_v481,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v500: self.scalar_v500,
            scalar_v501: self.scalar_v501,
            scalar_v505: self.scalar_v505,
            scalar_v506: self.scalar_v506,
            scalar_v507: self.scalar_v507,
            scalar_v508: self.scalar_v508,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v517: self.scalar_v517,
            scalar_v524: self.scalar_v524,
            scalar_v527: self.scalar_v527,
            scalar_v535: self.scalar_v535,
            scalar_v544: self.scalar_v544,
            scalar_v557: self.scalar_v557,
            scalar_v566: self.scalar_v566,
            scalar_v578: self.scalar_v578,
            scalar_v581: self.scalar_v581,
            scalar_v584: self.scalar_v584,
            scalar_v585: self.scalar_v585,
            scalar_v586: self.scalar_v586,
            scalar_v590: self.scalar_v590,
            scalar_v595: self.scalar_v595,
            scalar_v596: self.scalar_v596,
            scalar_v597: self.scalar_v597,
            scalar_v602: self.scalar_v602,
            scalar_v603: self.scalar_v603,
            scalar_v607: self.scalar_v607,
            scalar_v608: self.scalar_v608,
            scalar_v627: self.scalar_v627,
            scalar_v629: self.scalar_v629,
            scalar_v631: self.scalar_v631,
            scalar_v637: self.scalar_v637,
            scalar_v639: self.scalar_v639,
            scalar_v645: self.scalar_v645,
            scalar_v647: self.scalar_v647,
            scalar_v653: self.scalar_v653,
            scalar_v701: self.scalar_v701,
            scalar_v706: self.scalar_v706,
            scalar_v824: self.scalar_v824,
            scalar_v878: self.scalar_v878,
            scalar_v879: self.scalar_v879,
            scalar_v880: self.scalar_v880,
            scalar_v891: self.scalar_v891,
            scalar_v912: self.scalar_v912,
            scalar_v913: self.scalar_v913,
            scalar_v914: self.scalar_v914,
            scalar_v915: self.scalar_v915,
            scalar_v916: self.scalar_v916,
            scalar_v917: self.scalar_v917,
            scalar_v964: self.scalar_v964,
            scalar_v974: self.scalar_v974,
            scalar_v987: self.scalar_v987,
            scalar_v988: self.scalar_v988,
            scalar_v992: self.scalar_v992,
            scalar_v1043: self.scalar_v1043,
            scalar_v1044: self.scalar_v1044,
            scalar_v1045: self.scalar_v1045,
            scalar_v1067: self.scalar_v1067,
            scalar_v1075: self.scalar_v1075,
            scalar_v1076: self.scalar_v1076,
            scalar_v1078: self.scalar_v1078,
            scalar_v1079: self.scalar_v1079,
            scalar_v1080: self.scalar_v1080,
            scalar_v1083: self.scalar_v1083,
            scalar_v1084: self.scalar_v1084,
            scalar_v1089: self.scalar_v1089,
            scalar_v1110: self.scalar_v1110,
            scalar_v1112: self.scalar_v1112,
            scalar_v1141: self.scalar_v1141,
            scalar_v1147: self.scalar_v1147,
            scalar_v1182: self.scalar_v1182,
            scalar_v1206: self.scalar_v1206,
            scalar_v1219: self.scalar_v1219,
            scalar_v1237: self.scalar_v1237,
            scalar_v1300: self.scalar_v1300,
            scalar_v1301: self.scalar_v1301,
            scalar_v1302: self.scalar_v1302,
            scalar_v1303: self.scalar_v1303,
            scalar_v1305: self.scalar_v1305,
            scalar_v1306: self.scalar_v1306,
            scalar_v1307: self.scalar_v1307,
            scalar_v1400: self.scalar_v1400,
            scalar_v1401: self.scalar_v1401,
            scalar_v1402: self.scalar_v1402,
            scalar_v1426: self.scalar_v1426,
            scalar_v1428: self.scalar_v1428,
            scalar_v1429: self.scalar_v1429,
            scalar_v1431: self.scalar_v1431,
            scalar_v1491: self.scalar_v1491,
            scalar_v1492: self.scalar_v1492,
            scalar_v1493: self.scalar_v1493,
            scalar_v1519: self.scalar_v1519,
            scalar_v1521: self.scalar_v1521,
            scalar_v1522: self.scalar_v1522,
            scalar_v1524: self.scalar_v1524,
            scalar_v1590: self.scalar_v1590,
            scalar_v1591: self.scalar_v1591,
            scalar_v1592: self.scalar_v1592,
            scalar_v1593: self.scalar_v1593,
            scalar_v1599: self.scalar_v1599,
            scalar_v1608: self.scalar_v1608,
            scalar_v1609: self.scalar_v1609,
            scalar_v1621: self.scalar_v1621,
            scalar_v1640: self.scalar_v1640,
            scalar_v1650: self.scalar_v1650,
            scalar_v1651: self.scalar_v1651,
            scalar_v1652: self.scalar_v1652,
            scalar_v1653: self.scalar_v1653,
            scalar_v1658: self.scalar_v1658,
            scalar_v1668: self.scalar_v1668,
            scalar_v1669: self.scalar_v1669,
            scalar_v1670: self.scalar_v1670,
            scalar_v1684: self.scalar_v1684,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1706: self.scalar_v1706,
            scalar_v1711: self.scalar_v1711,
            scalar_v1728: self.scalar_v1728,
            scalar_v1729: self.scalar_v1729,
            scalar_v1735: self.scalar_v1735,
            scalar_v1736: self.scalar_v1736,
            scalar_v1739: self.scalar_v1739,
            scalar_v1746: self.scalar_v1746,
            scalar_v1757: self.scalar_v1757,
            scalar_v1758: self.scalar_v1758,
            scalar_v1759: self.scalar_v1759,
            scalar_v1760: self.scalar_v1760,
            scalar_v1761: self.scalar_v1761,
            scalar_v1762: self.scalar_v1762,
            scalar_v1763: self.scalar_v1763,
            scalar_v1764: self.scalar_v1764,
            scalar_v1765: self.scalar_v1765,
            scalar_v1766: self.scalar_v1766,
            scalar_v1767: self.scalar_v1767,
            scalar_v1768: self.scalar_v1768,
            scalar_v1769: self.scalar_v1769,
            scalar_v1770: self.scalar_v1770,
            scalar_v1771: self.scalar_v1771,
            scalar_v1785: self.scalar_v1785,
            scalar_v1812: self.scalar_v1812,
            scalar_v1813: self.scalar_v1813,
            scalar_v1814: self.scalar_v1814,
            scalar_v1817: self.scalar_v1817,
            scalar_v1836: self.scalar_v1836,
            scalar_v1850: self.scalar_v1850,
            scalar_v1855: self.scalar_v1855,
            scalar_v1857: self.scalar_v1857,
            scalar_v1861: self.scalar_v1861,
            scalar_v1862: self.scalar_v1862,
            scalar_v1863: self.scalar_v1863,
            scalar_v1864: self.scalar_v1864,
            scalar_v1865: self.scalar_v1865,
            scalar_v1874: self.scalar_v1874,
            scalar_v1875: self.scalar_v1875,
            scalar_v1878: self.scalar_v1878,
            scalar_v1901: self.scalar_v1901,
            scalar_v1902: self.scalar_v1902,
            scalar_v1908: self.scalar_v1908,
            scalar_v1909: self.scalar_v1909,
            scalar_v1910: self.scalar_v1910,
            scalar_v1958: self.scalar_v1958,
            scalar_v1959: self.scalar_v1959,
            scalar_v1964: self.scalar_v1964,
            scalar_v1968: self.scalar_v1968,
            scalar_v1975: self.scalar_v1975,
            scalar_v1980: self.scalar_v1980,
            scalar_v2000: self.scalar_v2000,
            scalar_v2020: self.scalar_v2020,
            scalar_v2021: self.scalar_v2021,
            scalar_v2056: self.scalar_v2056,
            scalar_v2062: self.scalar_v2062,
            scalar_v2067: self.scalar_v2067,
            scalar_v2068: self.scalar_v2068,
            scalar_v2072: self.scalar_v2072,
            scalar_v2128: self.scalar_v2128,
            scalar_v2133: self.scalar_v2133,
            scalar_v2136: self.scalar_v2136,
            scalar_v2137: self.scalar_v2137,
            scalar_v2138: self.scalar_v2138,
            scalar_v2139: self.scalar_v2139,
            scalar_v2140: self.scalar_v2140,
            scalar_v2141: self.scalar_v2141,
            scalar_v2142: self.scalar_v2142,
            scalar_v2143: self.scalar_v2143,
            scalar_v2144: self.scalar_v2144,
            scalar_v2858: self.scalar_v2858,
            scalar_v2873: self.scalar_v2873,
            scalar_v2874: self.scalar_v2874,
            scalar_v2941: self.scalar_v2941,
            scalar_v2953: self.scalar_v2953,
            scalar_v3178: self.scalar_v3178,
            scalar_v3179: self.scalar_v3179,
            scalar_v3188: self.scalar_v3188,
            scalar_v3189: self.scalar_v3189,
            scalar_v3212: self.scalar_v3212,
            scalar_v3213: self.scalar_v3213,
            scalar_v3224: self.scalar_v3224,
            scalar_v3225: self.scalar_v3225,
            scalar_v3561: self.scalar_v3561,
            scalar_v3600: self.scalar_v3600,
            scalar_v3601: self.scalar_v3601,
            scalar_v3644: self.scalar_v3644,
            scalar_v3645: self.scalar_v3645,
            scalar_v3734: self.scalar_v3734,
            scalar_v3773: self.scalar_v3773,
            scalar_v3774: self.scalar_v3774,
            scalar_v4065: self.scalar_v4065,
            scalar_v4066: self.scalar_v4066,
            scalar_v4262: self.scalar_v4262,
            scalar_v4263: self.scalar_v4263,
            scalar_v4264: self.scalar_v4264,
            scalar_v4265: self.scalar_v4265,
            scalar_v4266: self.scalar_v4266,
            scalar_v4267: self.scalar_v4267,
            scalar_v4530: self.scalar_v4530,
            scalar_v4531: self.scalar_v4531,
            scalar_v4532: self.scalar_v4532,
            scalar_v4533: self.scalar_v4533,
            scalar_v4534: self.scalar_v4534,
            scalar_v4535: self.scalar_v4535,
            scalar_v4536: self.scalar_v4536,
            scalar_v4537: self.scalar_v4537,
            scalar_v4538: self.scalar_v4538,
            scalar_v4931: self.scalar_v4931,
            scalar_v5403: self.scalar_v5403,
            scalar_v5482: self.scalar_v5482,
            scalar_v5718: self.scalar_v5718,
            scalar_v5719: self.scalar_v5719,
            scalar_v5720: self.scalar_v5720,
            scalar_v5721: self.scalar_v5721,
            scalar_v5722: self.scalar_v5722,
            scalar_v5723: self.scalar_v5723,
            scalar_v5884: self.scalar_v5884,
            scalar_v5885: self.scalar_v5885,
            scalar_v5914: self.scalar_v5914,
            scalar_v5915: self.scalar_v5915,
            scalar_v5916: self.scalar_v5916,
            scalar_v5917: self.scalar_v5917,
            scalar_v5918: self.scalar_v5918,
            scalar_v5919: self.scalar_v5919,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 156;
    pub const VARIABLE_COUNT: usize = 616;
    pub const DDT_STATE_COUNT: usize = 10;
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
            scalar_v0: 0.0,
            scalar_v2: false,
            scalar_v5: 0.0,
            scalar_v7: 0.0,
            scalar_v8: false,
            scalar_v10: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v17: 0.0,
            scalar_v19: 0.0,
            scalar_v21: 0.0,
            scalar_v22: false,
            scalar_v24: 0.0,
            scalar_v25: false,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: false,
            scalar_v32: 0.0,
            scalar_v33: false,
            scalar_v34: 0.0,
            scalar_v37: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v40: 0.0,
            scalar_v41: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v50: 0.0,
            scalar_v52: 0.0,
            scalar_v53: false,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: false,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v65: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: false,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: false,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v110: 0.0,
            scalar_v112: 0.0,
            scalar_v167: 0.0,
            scalar_v187: 0.0,
            scalar_v190: 0.0,
            scalar_v230: 0.0,
            scalar_v233: 0.0,
            scalar_v253: 0.0,
            scalar_v256: 0.0,
            scalar_v267: 0.0,
            scalar_v268: 0.0,
            scalar_v275: 0.0,
            scalar_v276: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v306: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v313: 0.0,
            scalar_v314: false,
            scalar_v315: 0.0,
            scalar_v343: false,
            scalar_v345: 0.0,
            scalar_v346: false,
            scalar_v347: 0.0,
            scalar_v374: false,
            scalar_v376: 0.0,
            scalar_v377: 0.0,
            scalar_v395: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v405: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v421: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v429: 0.0,
            scalar_v430: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v437: 0.0,
            scalar_v438: 0.0,
            scalar_v442: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v451: 0.0,
            scalar_v455: 0.0,
            scalar_v456: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v469: 0.0,
            scalar_v470: false,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v473: 0.0,
            scalar_v479: 0.0,
            scalar_v480: 0.0,
            scalar_v481: 0.0,
            scalar_v486: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v500: 0.0,
            scalar_v501: 0.0,
            scalar_v505: 0.0,
            scalar_v506: 0.0,
            scalar_v507: 0.0,
            scalar_v508: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v517: 0.0,
            scalar_v524: 0.0,
            scalar_v527: 0.0,
            scalar_v535: 0.0,
            scalar_v544: 0.0,
            scalar_v557: 0.0,
            scalar_v566: 0.0,
            scalar_v578: 0.0,
            scalar_v581: 0.0,
            scalar_v584: 0.0,
            scalar_v585: 0.0,
            scalar_v586: 0.0,
            scalar_v590: 0.0,
            scalar_v595: 0.0,
            scalar_v596: 0.0,
            scalar_v597: 0.0,
            scalar_v602: 0.0,
            scalar_v603: 0.0,
            scalar_v607: 0.0,
            scalar_v608: 0.0,
            scalar_v627: 0.0,
            scalar_v629: 0.0,
            scalar_v631: false,
            scalar_v637: false,
            scalar_v639: false,
            scalar_v645: false,
            scalar_v647: false,
            scalar_v653: false,
            scalar_v701: 0.0,
            scalar_v706: 0.0,
            scalar_v824: 0.0,
            scalar_v878: 0.0,
            scalar_v879: 0.0,
            scalar_v880: 0.0,
            scalar_v891: 0.0,
            scalar_v912: 0.0,
            scalar_v913: 0.0,
            scalar_v914: 0.0,
            scalar_v915: 0.0,
            scalar_v916: 0.0,
            scalar_v917: 0.0,
            scalar_v964: 0.0,
            scalar_v974: 0.0,
            scalar_v987: 0.0,
            scalar_v988: false,
            scalar_v992: false,
            scalar_v1043: 0.0,
            scalar_v1044: 0.0,
            scalar_v1045: 0.0,
            scalar_v1067: 0.0,
            scalar_v1075: 0.0,
            scalar_v1076: false,
            scalar_v1078: false,
            scalar_v1079: false,
            scalar_v1080: false,
            scalar_v1083: false,
            scalar_v1084: false,
            scalar_v1089: 0.0,
            scalar_v1110: 0.0,
            scalar_v1112: 0.0,
            scalar_v1141: false,
            scalar_v1147: false,
            scalar_v1182: 0.0,
            scalar_v1206: 0.0,
            scalar_v1219: 0.0,
            scalar_v1237: 0.0,
            scalar_v1300: 0.0,
            scalar_v1301: false,
            scalar_v1302: false,
            scalar_v1303: false,
            scalar_v1305: false,
            scalar_v1306: false,
            scalar_v1307: 0.0,
            scalar_v1400: false,
            scalar_v1401: false,
            scalar_v1402: false,
            scalar_v1426: 0.0,
            scalar_v1428: 0.0,
            scalar_v1429: 0.0,
            scalar_v1431: 0.0,
            scalar_v1491: false,
            scalar_v1492: false,
            scalar_v1493: false,
            scalar_v1519: 0.0,
            scalar_v1521: 0.0,
            scalar_v1522: 0.0,
            scalar_v1524: 0.0,
            scalar_v1590: 0.0,
            scalar_v1591: false,
            scalar_v1592: 0.0,
            scalar_v1593: 0.0,
            scalar_v1599: 0.0,
            scalar_v1608: 0.0,
            scalar_v1609: 0.0,
            scalar_v1621: false,
            scalar_v1640: 0.0,
            scalar_v1650: 0.0,
            scalar_v1651: false,
            scalar_v1652: false,
            scalar_v1653: false,
            scalar_v1658: 0.0,
            scalar_v1668: false,
            scalar_v1669: 0.0,
            scalar_v1670: 0.0,
            scalar_v1684: false,
            scalar_v1692: false,
            scalar_v1693: false,
            scalar_v1706: 0.0,
            scalar_v1711: 0.0,
            scalar_v1728: false,
            scalar_v1729: false,
            scalar_v1735: 0.0,
            scalar_v1736: false,
            scalar_v1739: 0.0,
            scalar_v1746: 0.0,
            scalar_v1757: 0.0,
            scalar_v1758: 0.0,
            scalar_v1759: 0.0,
            scalar_v1760: 0.0,
            scalar_v1761: 0.0,
            scalar_v1762: 0.0,
            scalar_v1763: 0.0,
            scalar_v1764: 0.0,
            scalar_v1765: 0.0,
            scalar_v1766: 0.0,
            scalar_v1767: 0.0,
            scalar_v1768: 0.0,
            scalar_v1769: 0.0,
            scalar_v1770: 0.0,
            scalar_v1771: 0.0,
            scalar_v1785: false,
            scalar_v1812: 0.0,
            scalar_v1813: false,
            scalar_v1814: 0.0,
            scalar_v1817: 0.0,
            scalar_v1836: 0.0,
            scalar_v1850: 0.0,
            scalar_v1855: false,
            scalar_v1857: false,
            scalar_v1861: 0.0,
            scalar_v1862: 0.0,
            scalar_v1863: 0.0,
            scalar_v1864: 0.0,
            scalar_v1865: 0.0,
            scalar_v1874: 0.0,
            scalar_v1875: false,
            scalar_v1878: false,
            scalar_v1901: 0.0,
            scalar_v1902: 0.0,
            scalar_v1908: 0.0,
            scalar_v1909: 0.0,
            scalar_v1910: 0.0,
            scalar_v1958: false,
            scalar_v1959: false,
            scalar_v1964: 0.0,
            scalar_v1968: 0.0,
            scalar_v1975: 0.0,
            scalar_v1980: 0.0,
            scalar_v2000: 0.0,
            scalar_v2020: 0.0,
            scalar_v2021: false,
            scalar_v2056: false,
            scalar_v2062: false,
            scalar_v2067: 0.0,
            scalar_v2068: false,
            scalar_v2072: false,
            scalar_v2128: 0.0,
            scalar_v2133: 0.0,
            scalar_v2136: 0.0,
            scalar_v2137: 0.0,
            scalar_v2138: 0.0,
            scalar_v2139: 0.0,
            scalar_v2140: 0.0,
            scalar_v2141: 0.0,
            scalar_v2142: 0.0,
            scalar_v2143: 0.0,
            scalar_v2144: 0.0,
            scalar_v2858: 0.0,
            scalar_v2873: 0.0,
            scalar_v2874: 0.0,
            scalar_v2941: 0.0,
            scalar_v2953: 0.0,
            scalar_v3178: 0.0,
            scalar_v3179: 0.0,
            scalar_v3188: 0.0,
            scalar_v3189: 0.0,
            scalar_v3212: 0.0,
            scalar_v3213: 0.0,
            scalar_v3224: 0.0,
            scalar_v3225: 0.0,
            scalar_v3561: 0.0,
            scalar_v3600: 0.0,
            scalar_v3601: 0.0,
            scalar_v3644: 0.0,
            scalar_v3645: 0.0,
            scalar_v3734: 0.0,
            scalar_v3773: 0.0,
            scalar_v3774: 0.0,
            scalar_v4065: 0.0,
            scalar_v4066: 0.0,
            scalar_v4262: 0.0,
            scalar_v4263: 0.0,
            scalar_v4264: 0.0,
            scalar_v4265: 0.0,
            scalar_v4266: 0.0,
            scalar_v4267: 0.0,
            scalar_v4530: 0.0,
            scalar_v4531: 0.0,
            scalar_v4532: 0.0,
            scalar_v4533: 0.0,
            scalar_v4534: 0.0,
            scalar_v4535: 0.0,
            scalar_v4536: 0.0,
            scalar_v4537: 0.0,
            scalar_v4538: 0.0,
            scalar_v4931: 0.0,
            scalar_v5403: 0.0,
            scalar_v5482: 0.0,
            scalar_v5718: 0.0,
            scalar_v5719: 0.0,
            scalar_v5720: 0.0,
            scalar_v5721: 0.0,
            scalar_v5722: 0.0,
            scalar_v5723: 0.0,
            scalar_v5884: 0.0,
            scalar_v5885: 0.0,
            scalar_v5914: 0.0,
            scalar_v5915: 0.0,
            scalar_v5916: 0.0,
            scalar_v5917: 0.0,
            scalar_v5918: 0.0,
            scalar_v5919: 0.0,
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
            scalar_v0,
            scalar_v2,
            scalar_v5,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
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
            scalar_v50,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v110,
            scalar_v112,
            scalar_v167,
            scalar_v187,
            scalar_v190,
            scalar_v230,
            scalar_v233,
            scalar_v253,
            scalar_v256,
            scalar_v267,
            scalar_v268,
            scalar_v275,
            scalar_v276,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v289,
            scalar_v290,
            scalar_v296,
            scalar_v297,
            scalar_v301,
            scalar_v302,
            scalar_v306,
            scalar_v308,
            scalar_v309,
            scalar_v313,
            scalar_v314,
            scalar_v315,
            scalar_v343,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v374,
            scalar_v376,
            scalar_v377,
            scalar_v395,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v405,
            scalar_v410,
            scalar_v411,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v438,
            scalar_v442,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v451,
            scalar_v455,
            scalar_v456,
            scalar_v461,
            scalar_v462,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v500,
            scalar_v501,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v515,
            scalar_v516,
            scalar_v517,
            scalar_v524,
            scalar_v527,
            scalar_v535,
            scalar_v544,
            scalar_v557,
            scalar_v566,
            scalar_v578,
            scalar_v581,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v590,
            scalar_v595,
            scalar_v596,
            scalar_v597,
            scalar_v602,
            scalar_v603,
            scalar_v607,
            scalar_v608,
            scalar_v627,
            scalar_v629,
            scalar_v631,
            scalar_v637,
            scalar_v639,
            scalar_v645,
            scalar_v647,
            scalar_v653,
            scalar_v701,
            scalar_v706,
            scalar_v824,
            scalar_v878,
            scalar_v879,
            scalar_v880,
            scalar_v891,
            scalar_v912,
            scalar_v913,
            scalar_v914,
            scalar_v915,
            scalar_v916,
            scalar_v917,
            scalar_v964,
            scalar_v974,
            scalar_v987,
            scalar_v988,
            scalar_v992,
            scalar_v1043,
            scalar_v1044,
            scalar_v1045,
            scalar_v1067,
            scalar_v1075,
            scalar_v1076,
            scalar_v1078,
            scalar_v1079,
            scalar_v1080,
            scalar_v1083,
            scalar_v1084,
            scalar_v1089,
            scalar_v1110,
            scalar_v1112,
            scalar_v1141,
            scalar_v1147,
            scalar_v1182,
            scalar_v1206,
            scalar_v1219,
            scalar_v1237,
            scalar_v1300,
            scalar_v1301,
            scalar_v1302,
            scalar_v1303,
            scalar_v1305,
            scalar_v1306,
            scalar_v1307,
            scalar_v1400,
            scalar_v1401,
            scalar_v1402,
            scalar_v1426,
            scalar_v1428,
            scalar_v1429,
            scalar_v1431,
            scalar_v1491,
            scalar_v1492,
            scalar_v1493,
            scalar_v1519,
            scalar_v1521,
            scalar_v1522,
            scalar_v1524,
            scalar_v1590,
            scalar_v1591,
            scalar_v1592,
            scalar_v1593,
            scalar_v1599,
            scalar_v1608,
            scalar_v1609,
            scalar_v1621,
            scalar_v1640,
            scalar_v1650,
            scalar_v1651,
            scalar_v1652,
            scalar_v1653,
            scalar_v1658,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1684,
            scalar_v1692,
            scalar_v1693,
            scalar_v1706,
            scalar_v1711,
            scalar_v1728,
            scalar_v1729,
            scalar_v1735,
            scalar_v1736,
            scalar_v1739,
            scalar_v1746,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1764,
            scalar_v1765,
            scalar_v1766,
            scalar_v1767,
            scalar_v1768,
            scalar_v1769,
            scalar_v1770,
            scalar_v1771,
            scalar_v1785,
            scalar_v1812,
            scalar_v1813,
            scalar_v1814,
            scalar_v1817,
            scalar_v1836,
            scalar_v1850,
            scalar_v1855,
            scalar_v1857,
            scalar_v1861,
            scalar_v1862,
            scalar_v1863,
            scalar_v1864,
            scalar_v1865,
            scalar_v1874,
            scalar_v1875,
            scalar_v1878,
            scalar_v1901,
            scalar_v1902,
            scalar_v1908,
            scalar_v1909,
            scalar_v1910,
            scalar_v1958,
            scalar_v1959,
            scalar_v1964,
            scalar_v1968,
            scalar_v1975,
            scalar_v1980,
            scalar_v2000,
            scalar_v2020,
            scalar_v2021,
            scalar_v2056,
            scalar_v2062,
            scalar_v2067,
            scalar_v2068,
            scalar_v2072,
            scalar_v2128,
            scalar_v2133,
            scalar_v2136,
            scalar_v2137,
            scalar_v2138,
            scalar_v2139,
            scalar_v2140,
            scalar_v2141,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2858,
            scalar_v2873,
            scalar_v2874,
            scalar_v2941,
            scalar_v2953,
            scalar_v3178,
            scalar_v3179,
            scalar_v3188,
            scalar_v3189,
            scalar_v3212,
            scalar_v3213,
            scalar_v3224,
            scalar_v3225,
            scalar_v3561,
            scalar_v3600,
            scalar_v3601,
            scalar_v3644,
            scalar_v3645,
            scalar_v3734,
            scalar_v3773,
            scalar_v3774,
            scalar_v4065,
            scalar_v4066,
            scalar_v4262,
            scalar_v4263,
            scalar_v4264,
            scalar_v4265,
            scalar_v4266,
            scalar_v4267,
            scalar_v4530,
            scalar_v4531,
            scalar_v4532,
            scalar_v4533,
            scalar_v4534,
            scalar_v4535,
            scalar_v4536,
            scalar_v4537,
            scalar_v4538,
            scalar_v4931,
            scalar_v5403,
            scalar_v5482,
            scalar_v5718,
            scalar_v5719,
            scalar_v5720,
            scalar_v5721,
            scalar_v5722,
            scalar_v5723,
            scalar_v5884,
            scalar_v5885,
            scalar_v5914,
            scalar_v5915,
            scalar_v5916,
            scalar_v5917,
            scalar_v5918,
            scalar_v5919,
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
            scalar_v0,
            scalar_v2,
            scalar_v5,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
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
            scalar_v50,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v110,
            scalar_v112,
            scalar_v167,
            scalar_v187,
            scalar_v190,
            scalar_v230,
            scalar_v233,
            scalar_v253,
            scalar_v256,
            scalar_v267,
            scalar_v268,
            scalar_v275,
            scalar_v276,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v289,
            scalar_v290,
            scalar_v296,
            scalar_v297,
            scalar_v301,
            scalar_v302,
            scalar_v306,
            scalar_v308,
            scalar_v309,
            scalar_v313,
            scalar_v314,
            scalar_v315,
            scalar_v343,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v374,
            scalar_v376,
            scalar_v377,
            scalar_v395,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v405,
            scalar_v410,
            scalar_v411,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v438,
            scalar_v442,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v451,
            scalar_v455,
            scalar_v456,
            scalar_v461,
            scalar_v462,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v500,
            scalar_v501,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v515,
            scalar_v516,
            scalar_v517,
            scalar_v524,
            scalar_v527,
            scalar_v535,
            scalar_v544,
            scalar_v557,
            scalar_v566,
            scalar_v578,
            scalar_v581,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v590,
            scalar_v595,
            scalar_v596,
            scalar_v597,
            scalar_v602,
            scalar_v603,
            scalar_v607,
            scalar_v608,
            scalar_v627,
            scalar_v629,
            scalar_v631,
            scalar_v637,
            scalar_v639,
            scalar_v645,
            scalar_v647,
            scalar_v653,
            scalar_v701,
            scalar_v706,
            scalar_v824,
            scalar_v878,
            scalar_v879,
            scalar_v880,
            scalar_v891,
            scalar_v912,
            scalar_v913,
            scalar_v914,
            scalar_v915,
            scalar_v916,
            scalar_v917,
            scalar_v964,
            scalar_v974,
            scalar_v987,
            scalar_v988,
            scalar_v992,
            scalar_v1043,
            scalar_v1044,
            scalar_v1045,
            scalar_v1067,
            scalar_v1075,
            scalar_v1076,
            scalar_v1078,
            scalar_v1079,
            scalar_v1080,
            scalar_v1083,
            scalar_v1084,
            scalar_v1089,
            scalar_v1110,
            scalar_v1112,
            scalar_v1141,
            scalar_v1147,
            scalar_v1182,
            scalar_v1206,
            scalar_v1219,
            scalar_v1237,
            scalar_v1300,
            scalar_v1301,
            scalar_v1302,
            scalar_v1303,
            scalar_v1305,
            scalar_v1306,
            scalar_v1307,
            scalar_v1400,
            scalar_v1401,
            scalar_v1402,
            scalar_v1426,
            scalar_v1428,
            scalar_v1429,
            scalar_v1431,
            scalar_v1491,
            scalar_v1492,
            scalar_v1493,
            scalar_v1519,
            scalar_v1521,
            scalar_v1522,
            scalar_v1524,
            scalar_v1590,
            scalar_v1591,
            scalar_v1592,
            scalar_v1593,
            scalar_v1599,
            scalar_v1608,
            scalar_v1609,
            scalar_v1621,
            scalar_v1640,
            scalar_v1650,
            scalar_v1651,
            scalar_v1652,
            scalar_v1653,
            scalar_v1658,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1684,
            scalar_v1692,
            scalar_v1693,
            scalar_v1706,
            scalar_v1711,
            scalar_v1728,
            scalar_v1729,
            scalar_v1735,
            scalar_v1736,
            scalar_v1739,
            scalar_v1746,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1764,
            scalar_v1765,
            scalar_v1766,
            scalar_v1767,
            scalar_v1768,
            scalar_v1769,
            scalar_v1770,
            scalar_v1771,
            scalar_v1785,
            scalar_v1812,
            scalar_v1813,
            scalar_v1814,
            scalar_v1817,
            scalar_v1836,
            scalar_v1850,
            scalar_v1855,
            scalar_v1857,
            scalar_v1861,
            scalar_v1862,
            scalar_v1863,
            scalar_v1864,
            scalar_v1865,
            scalar_v1874,
            scalar_v1875,
            scalar_v1878,
            scalar_v1901,
            scalar_v1902,
            scalar_v1908,
            scalar_v1909,
            scalar_v1910,
            scalar_v1958,
            scalar_v1959,
            scalar_v1964,
            scalar_v1968,
            scalar_v1975,
            scalar_v1980,
            scalar_v2000,
            scalar_v2020,
            scalar_v2021,
            scalar_v2056,
            scalar_v2062,
            scalar_v2067,
            scalar_v2068,
            scalar_v2072,
            scalar_v2128,
            scalar_v2133,
            scalar_v2136,
            scalar_v2137,
            scalar_v2138,
            scalar_v2139,
            scalar_v2140,
            scalar_v2141,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2858,
            scalar_v2873,
            scalar_v2874,
            scalar_v2941,
            scalar_v2953,
            scalar_v3178,
            scalar_v3179,
            scalar_v3188,
            scalar_v3189,
            scalar_v3212,
            scalar_v3213,
            scalar_v3224,
            scalar_v3225,
            scalar_v3561,
            scalar_v3600,
            scalar_v3601,
            scalar_v3644,
            scalar_v3645,
            scalar_v3734,
            scalar_v3773,
            scalar_v3774,
            scalar_v4065,
            scalar_v4066,
            scalar_v4262,
            scalar_v4263,
            scalar_v4264,
            scalar_v4265,
            scalar_v4266,
            scalar_v4267,
            scalar_v4530,
            scalar_v4531,
            scalar_v4532,
            scalar_v4533,
            scalar_v4534,
            scalar_v4535,
            scalar_v4536,
            scalar_v4537,
            scalar_v4538,
            scalar_v4931,
            scalar_v5403,
            scalar_v5482,
            scalar_v5718,
            scalar_v5719,
            scalar_v5720,
            scalar_v5721,
            scalar_v5722,
            scalar_v5723,
            scalar_v5884,
            scalar_v5885,
            scalar_v5914,
            scalar_v5915,
            scalar_v5916,
            scalar_v5917,
            scalar_v5918,
            scalar_v5919,
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
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "mult" => { validate_parameter("mult", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((505.5, "505.5")), false, Some((505.51, "505.51")), true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "tref" => { validate_parameter("tref", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "exmod" => { validate_parameter("exmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "exphi" => { validate_parameter("exphi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "exavl" => { validate_parameter("exavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "exsub" => { validate_parameter("exsub", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "iss" => { validate_parameter("iss", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "icss" => { validate_parameter("icss", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "iks" => { validate_parameter("iks", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "ikcs" => { validate_parameter("ikcs", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.05, "0.05")), true, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "ps" => { validate_parameter("ps", value, Some((0.01, "0.01")), true, Some((0.99, "0.99")), true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "as" => { validate_finite_parameter("as", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "asub" => { validate_finite_parameter("asub", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "xisubi" => { validate_parameter("xisubi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "swvsch" => { validate_parameter("swvsch", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjt505_va'", name)),
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
        let v0: f64 = p.p3;
        self.scalar_v0 = v0;
        let v2: bool = (p.p3 == 1.0);
        self.scalar_v2 = v2;
        let v5: f64 = (if v2 { 70300000.0 } else { 0.0 });
        self.scalar_v5 = v5;
        let v7: f64 = (if v2 { 123000000.0 } else { 0.0 });
        self.scalar_v7 = v7;
        let v8: bool = (!v2);
        self.scalar_v8 = v8;
        let v10: f64 = (if v8 { 158000000.0 } else { v5 });
        self.scalar_v10 = v10;
        let v12: f64 = (if v8 { 204000000.0 } else { v7 });
        self.scalar_v12 = v12;
        let v13: f64 = p.p33;
        self.scalar_v13 = v13;
        let v14: f64 = (1.0 - p.p33);
        self.scalar_v14 = v14;
        let v15: f64 = p.p4;
        self.scalar_v15 = v15;
        let v17: f64 = (p.p4 + 273.15);
        self.scalar_v17 = v17;
        let v19: f64 = p.p0;
        self.scalar_v19 = v19;
        let v21: f64 = p.p150;
        self.scalar_v21 = v21;
        let v22: bool = (p.p150 == 0.0);
        self.scalar_v22 = v22;
        let v24: f64 = (if v22 { 1e-12 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: bool = (!v22);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p150 } else { v24 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p1;
        self.scalar_v27 = v27;
        let v28: f64 = (v26 * p.p1);
        self.scalar_v28 = v28;
        let v29: f64 = (1.0 / v28);
        self.scalar_v29 = v29;
        let v30: f64 = p.p134;
        self.scalar_v30 = v30;
        let v31: bool = (p.p134 > 0.0);
        self.scalar_v31 = v31;
        let v32: f64 = (if v31 { 0.0 } else { 0.0 });
        self.scalar_v32 = v32;
        let v33: bool = (!v31);
        self.scalar_v33 = v33;
        let v34: f64 = (if v33 { 0.0 } else { v32 });
        self.scalar_v34 = v34;
        let v37: f64 = p.p67;
        self.scalar_v37 = v37;
        let v38: f64 = (2.0 - p.p67);
        self.scalar_v38 = v38;
        let v39: f64 = f64::powf(2.0, v38);
        self.scalar_v39 = v39;
        let v40: f64 = (1.0 / v39);
        self.scalar_v40 = v40;
        let v41: f64 = p.p114;
        self.scalar_v41 = v41;
        let v42: f64 = p.p115;
        self.scalar_v42 = v42;
        let v43: f64 = (p.p115 * v17);
        self.scalar_v43 = v43;
        let v44: f64 = (v43 * v17);
        self.scalar_v44 = v44;
        let v45: f64 = p.p116;
        self.scalar_v45 = v45;
        let v46: f64 = (v17 + p.p116);
        self.scalar_v46 = v46;
        let v47: f64 = (v44 / v46);
        self.scalar_v47 = v47;
        let v48: f64 = (p.p114 + v47);
        self.scalar_v48 = v48;
        let v50: f64 = (v48 - 0.05);
        self.scalar_v50 = v50;
        let v52: f64 = (v50 / 0.1);
        self.scalar_v52 = v52;
        let v53: bool = (v48 < 0.05);
        self.scalar_v53 = v53;
        let v54: f64 = v52.exp();
        self.scalar_v54 = v54;
        let v55: f64 = (1.0 + v54);
        self.scalar_v55 = v55;
        let v56: f64 = v55.ln();
        self.scalar_v56 = v56;
        let v57: f64 = (0.1 * v56);
        self.scalar_v57 = v57;
        let v58: f64 = (0.05 + v57);
        self.scalar_v58 = v58;
        let v59: f64 = (if v53 { v58 } else { 0.0 });
        self.scalar_v59 = v59;
        let v60: bool = (!v53);
        self.scalar_v60 = v60;
        let v61: f64 = (-v52);
        self.scalar_v61 = v61;
        let v62: f64 = v61.exp();
        self.scalar_v62 = v62;
        let v63: f64 = (1.0 + v62);
        self.scalar_v63 = v63;
        let v64: f64 = v63.ln();
        self.scalar_v64 = v64;
        let v65: f64 = (0.1 * v64);
        self.scalar_v65 = v65;
        let v66: f64 = (v48 + v65);
        self.scalar_v66 = v66;
        let v67: f64 = (if v60 { v66 } else { v59 });
        self.scalar_v67 = v67;
        let v68: f64 = (1.0 / p.p114);
        self.scalar_v68 = v68;
        let v69: f64 = p.p66;
        self.scalar_v69 = v69;
        let v70: f64 = (1.0 / p.p66);
        self.scalar_v70 = v70;
        let v71: f64 = p.p71;
        self.scalar_v71 = v71;
        let v72: f64 = p.p72;
        self.scalar_v72 = v72;
        let v73: f64 = (2.0 - p.p72);
        self.scalar_v73 = v73;
        let v74: f64 = f64::powf(2.0, v73);
        self.scalar_v74 = v74;
        let v75: f64 = (1.0 / v74);
        self.scalar_v75 = v75;
        let v76: f64 = p.p117;
        self.scalar_v76 = v76;
        let v77: f64 = p.p118;
        self.scalar_v77 = v77;
        let v78: f64 = (p.p118 * v17);
        self.scalar_v78 = v78;
        let v79: f64 = (v78 * v17);
        self.scalar_v79 = v79;
        let v80: f64 = p.p119;
        self.scalar_v80 = v80;
        let v81: f64 = (v17 + p.p119);
        self.scalar_v81 = v81;
        let v82: f64 = (v79 / v81);
        self.scalar_v82 = v82;
        let v83: f64 = (p.p117 + v82);
        self.scalar_v83 = v83;
        let v84: f64 = (v83 - 0.05);
        self.scalar_v84 = v84;
        let v85: f64 = (v84 / 0.1);
        self.scalar_v85 = v85;
        let v86: bool = (v83 < 0.05);
        self.scalar_v86 = v86;
        let v87: f64 = v85.exp();
        self.scalar_v87 = v87;
        let v88: f64 = (1.0 + v87);
        self.scalar_v88 = v88;
        let v89: f64 = v88.ln();
        self.scalar_v89 = v89;
        let v90: f64 = (0.1 * v89);
        self.scalar_v90 = v90;
        let v91: f64 = (0.05 + v90);
        self.scalar_v91 = v91;
        let v92: f64 = (if v86 { v91 } else { 0.0 });
        self.scalar_v92 = v92;
        let v93: bool = (!v86);
        self.scalar_v93 = v93;
        let v94: f64 = (-v85);
        self.scalar_v94 = v94;
        let v95: f64 = v94.exp();
        self.scalar_v95 = v95;
        let v96: f64 = (1.0 + v95);
        self.scalar_v96 = v96;
        let v97: f64 = v96.ln();
        self.scalar_v97 = v97;
        let v98: f64 = (0.1 * v97);
        self.scalar_v98 = v98;
        let v99: f64 = (v83 + v98);
        self.scalar_v99 = v99;
        let v100: f64 = (if v93 { v99 } else { v92 });
        self.scalar_v100 = v100;
        let v101: f64 = (1.0 / p.p117);
        self.scalar_v101 = v101;
        let v102: f64 = (1.0 / p.p71);
        self.scalar_v102 = v102;
        let v103: f64 = p.p83;
        self.scalar_v103 = v103;
        let v104: f64 = (1.0 / p.p83);
        self.scalar_v104 = v104;
        let v105: f64 = (1.0 - v104);
        self.scalar_v105 = v105;
        let v110: f64 = (8.617086918058125e-5 * v17);
        self.scalar_v110 = v110;
        let v112: f64 = (1.0 / v110);
        self.scalar_v112 = v112;
        let v167: f64 = p.p105;
        self.scalar_v167 = v167;
        let v187: f64 = p.p64;
        self.scalar_v187 = v187;
        let v190: f64 = p.p110;
        self.scalar_v190 = v190;
        let v230: f64 = p.p27;
        self.scalar_v230 = v230;
        let v233: f64 = p.p109;
        self.scalar_v233 = v233;
        let v253: f64 = p.p138;
        self.scalar_v253 = v253;
        let v256: f64 = p.p140;
        self.scalar_v256 = v256;
        let v267: f64 = p.p75;
        self.scalar_v267 = v267;
        let v268: f64 = (1.0 - p.p75);
        self.scalar_v268 = v268;
        let v275: f64 = p.p54;
        self.scalar_v275 = v275;
        let v276: f64 = p.p97;
        self.scalar_v276 = v276;
        let v282: f64 = p.p56;
        self.scalar_v282 = v282;
        let v283: f64 = p.p98;
        self.scalar_v283 = v283;
        let v284: f64 = p.p96;
        self.scalar_v284 = v284;
        let v285: f64 = (p.p98 - p.p96);
        self.scalar_v285 = v285;
        let v289: f64 = p.p55;
        self.scalar_v289 = v289;
        let v290: f64 = p.p101;
        self.scalar_v290 = v290;
        let v296: f64 = p.p57;
        self.scalar_v296 = v296;
        let v297: f64 = p.p102;
        self.scalar_v297 = v297;
        let v301: f64 = p.p58;
        self.scalar_v301 = v301;
        let v302: f64 = p.p104;
        self.scalar_v302 = v302;
        let v306: f64 = p.p59;
        self.scalar_v306 = v306;
        let v308: f64 = p.p60;
        self.scalar_v308 = v308;
        let v309: f64 = p.p99;
        self.scalar_v309 = v309;
        let v313: f64 = p.p122;
        self.scalar_v313 = v313;
        let v314: bool = (p.p122 != 0.0);
        self.scalar_v314 = v314;
        let v315: f64 = p.p10;
        self.scalar_v315 = v315;
        let v343: bool = (!v314);
        self.scalar_v343 = v343;
        let v345: f64 = p.p123;
        self.scalar_v345 = v345;
        let v346: bool = (p.p123 != 0.0);
        self.scalar_v346 = v346;
        let v347: f64 = p.p11;
        self.scalar_v347 = v347;
        let v374: bool = (!v346);
        self.scalar_v374 = v374;
        let v376: f64 = p.p43;
        self.scalar_v376 = v376;
        let v377: f64 = p.p124;
        self.scalar_v377 = v377;
        let v395: f64 = p.p9;
        self.scalar_v395 = v395;
        let v397: f64 = (4.0 - p.p98);
        self.scalar_v397 = v397;
        let v398: f64 = (v397 - p.p96);
        self.scalar_v398 = v398;
        let v399: f64 = p.p121;
        self.scalar_v399 = v399;
        let v400: f64 = (v398 + p.p121);
        self.scalar_v400 = v400;
        let v405: f64 = (-p.p105);
        self.scalar_v405 = v405;
        let v410: f64 = p.p12;
        self.scalar_v410 = v410;
        let v411: f64 = (1.0 - p.p98);
        self.scalar_v411 = v411;
        let v415: f64 = p.p30;
        self.scalar_v415 = v415;
        let v416: f64 = p.p103;
        self.scalar_v416 = v416;
        let v417: f64 = (1.0 - p.p103);
        self.scalar_v417 = v417;
        let v421: f64 = p.p20;
        self.scalar_v421 = v421;
        let v423: f64 = p.p21;
        self.scalar_v423 = v423;
        let v424: f64 = (2.0 * p.p21);
        self.scalar_v424 = v424;
        let v425: f64 = (6.0 - v424);
        self.scalar_v425 = v425;
        let v429: f64 = p.p113;
        self.scalar_v429 = v429;
        let v430: f64 = (-p.p113);
        self.scalar_v430 = v430;
        let v435: f64 = p.p31;
        self.scalar_v435 = v435;
        let v436: f64 = p.p32;
        self.scalar_v436 = v436;
        let v437: f64 = (2.0 * p.p32);
        self.scalar_v437 = v437;
        let v438: f64 = (6.0 - v437);
        self.scalar_v438 = v438;
        let v442: f64 = (-p.p110);
        self.scalar_v442 = v442;
        let v447: f64 = p.p16;
        self.scalar_v447 = v447;
        let v448: f64 = (4.0 - p.p97);
        self.scalar_v448 = v448;
        let v449: f64 = (v448 + p.p121);
        self.scalar_v449 = v449;
        let v451: f64 = p.p17;
        self.scalar_v451 = v451;
        let v455: f64 = p.p111;
        self.scalar_v455 = v455;
        let v456: f64 = (-p.p111);
        self.scalar_v456 = v456;
        let v461: f64 = p.p18;
        self.scalar_v461 = v461;
        let v462: f64 = p.p19;
        self.scalar_v462 = v462;
        let v469: f64 = p.p24;
        self.scalar_v469 = v469;
        let v470: bool = (p.p24 == 1.0);
        self.scalar_v470 = v470;
        let v471: f64 = p.p25;
        self.scalar_v471 = v471;
        let v472: f64 = p.p107;
        self.scalar_v472 = v472;
        let v473: f64 = (-p.p107);
        self.scalar_v473 = v473;
        let v479: f64 = p.p28;
        self.scalar_v479 = v479;
        let v480: f64 = p.p106;
        self.scalar_v480 = v480;
        let v481: f64 = (-p.p106);
        self.scalar_v481 = v481;
        let v486: f64 = p.p26;
        self.scalar_v486 = v486;
        let v487: f64 = p.p108;
        self.scalar_v487 = v487;
        let v488: f64 = (-p.p108);
        self.scalar_v488 = v488;
        let v494: f64 = p.p29;
        self.scalar_v494 = v494;
        let v495: f64 = (4.0 - p.p103);
        self.scalar_v495 = v495;
        let v496: f64 = (v495 + p.p121);
        self.scalar_v496 = v496;
        let v500: f64 = p.p112;
        self.scalar_v500 = v500;
        let v501: f64 = (-p.p112);
        self.scalar_v501 = v501;
        let v505: f64 = p.p22;
        self.scalar_v505 = v505;
        let v506: f64 = p.p23;
        self.scalar_v506 = v506;
        let v507: f64 = (2.0 * p.p23);
        self.scalar_v507 = v507;
        let v508: f64 = (6.0 - v507);
        self.scalar_v508 = v508;
        let v515: f64 = p.p145;
        self.scalar_v515 = v515;
        let v516: f64 = p.p146;
        self.scalar_v516 = v516;
        let v517: f64 = (4.0 / p.p146);
        self.scalar_v517 = v517;
        let v524: f64 = p.p151;
        self.scalar_v524 = v524;
        let v527: f64 = p.p153;
        self.scalar_v527 = v527;
        let v535: f64 = p.p35;
        self.scalar_v535 = v535;
        let v544: f64 = p.p34;
        self.scalar_v544 = v544;
        let v557: f64 = p.p37;
        self.scalar_v557 = v557;
        let v566: f64 = p.p36;
        self.scalar_v566 = v566;
        let v578: f64 = p.p14;
        self.scalar_v578 = v578;
        let v581: f64 = p.p13;
        self.scalar_v581 = v581;
        let v584: f64 = p.p133;
        self.scalar_v584 = v584;
        let v585: f64 = p.p141;
        self.scalar_v585 = v585;
        let v586: f64 = (4.0 - p.p141);
        self.scalar_v586 = v586;
        let v590: f64 = (-p.p140);
        self.scalar_v590 = v590;
        let v595: f64 = p.p142;
        self.scalar_v595 = v595;
        let v596: f64 = (0.5 * p.p142);
        self.scalar_v596 = v596;
        let v597: f64 = (3.5 - v596);
        self.scalar_v597 = v597;
        let v602: f64 = p.p135;
        self.scalar_v602 = v602;
        let v603: f64 = (1.0 - p.p141);
        self.scalar_v603 = v603;
        let v607: f64 = p.p136;
        self.scalar_v607 = v607;
        let v608: f64 = (1.0 - p.p142);
        self.scalar_v608 = v608;
        let v627: f64 = (v12 * 1.081);
        self.scalar_v627 = v627;
        let v629: f64 = p.p92;
        self.scalar_v629 = v629;
        let v631: bool = (p.p57 > 0.0);
        self.scalar_v631 = v631;
        let v637: bool = (!v631);
        self.scalar_v637 = v637;
        let v639: bool = (p.p58 > 0.0);
        self.scalar_v639 = v639;
        let v645: bool = (!v639);
        self.scalar_v645 = v645;
        let v647: bool = (p.p59 > 0.0);
        self.scalar_v647 = v647;
        let v653: bool = (!v647);
        self.scalar_v653 = v653;
        let v701: f64 = p.p147;
        self.scalar_v701 = v701;
        let v706: f64 = p.p147.exp();
        self.scalar_v706 = v706;
        let v824: f64 = p.p149;
        self.scalar_v824 = v824;
        let v878: f64 = p.p62;
        self.scalar_v878 = v878;
        let v879: f64 = p.p61;
        self.scalar_v879 = v879;
        let v880: f64 = (p.p62 * p.p61);
        self.scalar_v880 = v880;
        let v891: f64 = p.p63;
        self.scalar_v891 = v891;
        let v912: f64 = (-1.0 / p.p63);
        self.scalar_v912 = v912;
        let v913: f64 = v912.exp();
        self.scalar_v913 = v913;
        let v914: f64 = (1.0 + v913);
        self.scalar_v914 = v914;
        let v915: f64 = v914.ln();
        self.scalar_v915 = v915;
        let v916: f64 = (p.p63 * v915);
        self.scalar_v916 = v916;
        let v917: f64 = (1.0 + v916);
        self.scalar_v917 = v917;
        let v964: f64 = p.p148;
        self.scalar_v964 = v964;
        let v974: f64 = (0.5 * p.p61);
        self.scalar_v974 = v974;
        let v987: f64 = p.p73;
        self.scalar_v987 = v987;
        let v988: bool = (p.p73 == 0.0);
        self.scalar_v988 = v988;
        let v992: bool = (!v988);
        self.scalar_v992 = v992;
        let v1043: f64 = (-1.0 / p.p67);
        self.scalar_v1043 = v1043;
        let v1044: f64 = f64::powf(3.0, v1043);
        self.scalar_v1044 = v1044;
        let v1045: f64 = (1.0 - v1044);
        self.scalar_v1045 = v1045;
        let v1067: f64 = (1.0 - p.p67);
        self.scalar_v1067 = v1067;
        let v1075: f64 = p.p74;
        self.scalar_v1075 = v1075;
        let v1076: bool = (p.p74 == 1.0);
        self.scalar_v1076 = v1076;
        let v1078: bool = (p.p74 == 2.0);
        self.scalar_v1078 = v1078;
        let v1079: bool = (!v1076);
        self.scalar_v1079 = v1079;
        let v1080: bool = (v1079 && v1078);
        self.scalar_v1080 = v1080;
        let v1083: bool = (!v1078);
        self.scalar_v1083 = v1083;
        let v1084: bool = (v1079 && v1083);
        self.scalar_v1084 = v1084;
        let v1089: f64 = (-1.0 / p.p72);
        self.scalar_v1089 = v1089;
        let v1110: f64 = p.p76;
        self.scalar_v1110 = v1110;
        let v1112: f64 = (1.0 - p.p72);
        self.scalar_v1112 = v1112;
        let v1141: bool = (p.p92 == 0.0);
        self.scalar_v1141 = v1141;
        let v1147: bool = (!v1141);
        self.scalar_v1147 = v1147;
        let v1182: f64 = p.p15;
        self.scalar_v1182 = v1182;
        let v1206: f64 = p.p152;
        self.scalar_v1206 = v1206;
        let v1219: f64 = p.p154;
        self.scalar_v1219 = v1219;
        let v1237: f64 = p.p155;
        self.scalar_v1237 = v1237;
        let v1300: f64 = p.p93;
        self.scalar_v1300 = v1300;
        let v1301: bool = (p.p93 == 0.0);
        self.scalar_v1301 = v1301;
        let v1302: bool = (!v470);
        self.scalar_v1302 = v1302;
        let v1303: bool = (v1302 && v1301);
        self.scalar_v1303 = v1303;
        let v1305: bool = (!v1301);
        self.scalar_v1305 = v1305;
        let v1306: bool = (v1302 && v1305);
        self.scalar_v1306 = v1306;
        let v1307: f64 = (1.0 - p.p93);
        self.scalar_v1307 = v1307;
        let v1400: bool = (p.p34 > 0.0);
        self.scalar_v1400 = v1400;
        let v1401: bool = (p.p35 > 0.0);
        self.scalar_v1401 = v1401;
        let v1402: bool = (v1400 && v1401);
        self.scalar_v1402 = v1402;
        let v1426: f64 = (-2.0 - p.p67);
        self.scalar_v1426 = v1426;
        let v1428: f64 = (p.p67 * p.p67);
        self.scalar_v1428 = v1428;
        let v1429: f64 = (1.0 - v1428);
        self.scalar_v1429 = v1429;
        let v1431: f64 = (p.p67 - 1.0);
        self.scalar_v1431 = v1431;
        let v1491: bool = (p.p36 > 0.0);
        self.scalar_v1491 = v1491;
        let v1492: bool = (p.p37 > 0.0);
        self.scalar_v1492 = v1492;
        let v1493: bool = (v1491 && v1492);
        self.scalar_v1493 = v1493;
        let v1519: f64 = (-2.0 - p.p72);
        self.scalar_v1519 = v1519;
        let v1521: f64 = (p.p72 * p.p72);
        self.scalar_v1521 = v1521;
        let v1522: f64 = (1.0 - v1521);
        self.scalar_v1522 = v1522;
        let v1524: f64 = (p.p72 - 1.0);
        self.scalar_v1524 = v1524;
        let v1590: f64 = p.p8;
        self.scalar_v1590 = v1590;
        let v1591: bool = (p.p8 == 1.0);
        self.scalar_v1591 = v1591;
        let v1592: f64 = p.p143;
        self.scalar_v1592 = v1592;
        let v1593: f64 = (p.p143 * 2.0);
        self.scalar_v1593 = v1593;
        let v1599: f64 = p.p144;
        self.scalar_v1599 = v1599;
        let v1608: f64 = (1.0 - p.p143);
        self.scalar_v1608 = v1608;
        let v1609: f64 = (v1608 * 2.0);
        self.scalar_v1609 = v1609;
        let v1621: bool = (!v1591);
        self.scalar_v1621 = v1621;
        let v1640: f64 = (p.p144 * 4.0);
        self.scalar_v1640 = v1640;
        let v1650: f64 = p.p5;
        self.scalar_v1650 = v1650;
        let v1651: bool = (p.p5 > 0.0);
        self.scalar_v1651 = v1651;
        let v1652: bool = (p.p33 > 0.0);
        self.scalar_v1652 = v1652;
        let v1653: bool = (v1651 && v1652);
        self.scalar_v1653 = v1653;
        let v1658: f64 = (p.p33 * 2.0);
        self.scalar_v1658 = v1658;
        let v1668: bool = (v1653 && v1591);
        self.scalar_v1668 = v1668;
        let v1669: f64 = (v1608 * p.p33);
        self.scalar_v1669 = v1669;
        let v1670: f64 = (v1669 * 2.0);
        self.scalar_v1670 = v1670;
        let v1684: bool = (v1653 && v1621);
        self.scalar_v1684 = v1684;
        let v1692: bool = (p.p5 == 1.0);
        self.scalar_v1692 = v1692;
        let v1693: bool = (v1653 && v1692);
        self.scalar_v1693 = v1693;
        let v1706: f64 = (if v1693 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1706 = v1706;
        let v1711: f64 = (0.5 * v1706);
        self.scalar_v1711 = v1711;
        let v1728: bool = (!v1692);
        self.scalar_v1728 = v1728;
        let v1729: bool = (v1653 && v1728);
        self.scalar_v1729 = v1729;
        let v1735: f64 = p.p84;
        self.scalar_v1735 = v1735;
        let v1736: bool = (p.p84 == 1.0);
        self.scalar_v1736 = v1736;
        let v1739: f64 = (if v1736 { 1e-12 } else { v1706 });
        self.scalar_v1739 = v1739;
        let v1746: f64 = (0.5 * v1739);
        self.scalar_v1746 = v1746;
        let v1757: f64 = p.p82;
        self.scalar_v1757 = v1757;
        let v1758: f64 = f64::powf(v105, p.p82);
        self.scalar_v1758 = v1758;
        let v1759: f64 = (1.0 - v1758);
        self.scalar_v1759 = v1759;
        let v1760: f64 = (1.0 / v1759);
        self.scalar_v1760 = v1760;
        let v1761: f64 = (if v1736 { v1760 } else { 0.0 });
        self.scalar_v1761 = v1761;
        let v1762: f64 = p.p81;
        self.scalar_v1762 = v1762;
        let v1763: f64 = (v105 * p.p81);
        self.scalar_v1763 = v1763;
        let v1764: f64 = (if v1736 { v1763 } else { 0.0 });
        self.scalar_v1764 = v1764;
        let v1765: f64 = (v1761 * v1761);
        self.scalar_v1765 = v1765;
        let v1766: f64 = (p.p82 - 1.0);
        self.scalar_v1766 = v1766;
        let v1767: f64 = f64::powf(v105, v1766);
        self.scalar_v1767 = v1767;
        let v1768: f64 = (v1765 * v1767);
        self.scalar_v1768 = v1768;
        let v1769: f64 = (v1768 * p.p82);
        self.scalar_v1769 = v1769;
        let v1770: f64 = (v1769 / p.p81);
        self.scalar_v1770 = v1770;
        let v1771: f64 = (if v1736 { v1770 } else { 0.0 });
        self.scalar_v1771 = v1771;
        let v1785: bool = (!v1736);
        self.scalar_v1785 = v1785;
        let v1812: f64 = p.p39;
        self.scalar_v1812 = v1812;
        let v1813: bool = (p.p39 == 1.0);
        self.scalar_v1813 = v1813;
        let v1814: f64 = p.p44;
        self.scalar_v1814 = v1814;
        let v1817: f64 = p.p42;
        self.scalar_v1817 = v1817;
        let v1836: f64 = p.p41;
        self.scalar_v1836 = v1836;
        let v1850: f64 = p.p40;
        self.scalar_v1850 = v1850;
        let v1855: bool = (p.p39 == 2.0);
        self.scalar_v1855 = v1855;
        let v1857: bool = (!v1813);
        self.scalar_v1857 = v1857;
        let v1861: f64 = p.p46;
        self.scalar_v1861 = v1861;
        let v1862: f64 = (2.0 * p.p46);
        self.scalar_v1862 = v1862;
        let v1863: f64 = p.p45;
        self.scalar_v1863 = v1863;
        let v1864: f64 = (p.p45 * p.p45);
        self.scalar_v1864 = v1864;
        let v1865: f64 = (v1862 / v1864);
        self.scalar_v1865 = v1865;
        let v1874: f64 = p.p7;
        self.scalar_v1874 = v1874;
        let v1875: bool = (p.p7 == 0.0);
        self.scalar_v1875 = v1875;
        let v1878: bool = (!v1875);
        self.scalar_v1878 = v1878;
        let v1901: f64 = p.p47;
        self.scalar_v1901 = v1901;
        let v1902: f64 = (2.0 * p.p47);
        self.scalar_v1902 = v1902;
        let v1908: f64 = (1.0 + p.p47);
        self.scalar_v1908 = v1908;
        let v1909: f64 = (1.0 + v1902);
        self.scalar_v1909 = v1909;
        let v1910: f64 = (v1908 / v1909);
        self.scalar_v1910 = v1910;
        let v1958: bool = (p.p39 == 3.0);
        self.scalar_v1958 = v1958;
        let v1959: bool = (!v1855);
        self.scalar_v1959 = v1959;
        let v1964: f64 = p.p48;
        self.scalar_v1964 = v1964;
        let v1968: f64 = p.p49;
        self.scalar_v1968 = v1968;
        let v1975: f64 = p.p52;
        self.scalar_v1975 = v1975;
        let v1980: f64 = p.p51;
        self.scalar_v1980 = v1980;
        let v2000: f64 = p.p50;
        self.scalar_v2000 = v2000;
        let v2020: f64 = p.p53;
        self.scalar_v2020 = v2020;
        let v2021: bool = (p.p53 == 1.0);
        self.scalar_v2021 = v2021;
        let v2056: bool = (!v1958);
        self.scalar_v2056 = v2056;
        let v2062: bool = (!v2021);
        self.scalar_v2062 = v2062;
        let v2067: f64 = p.p130;
        self.scalar_v2067 = v2067;
        let v2068: bool = (p.p130 > 0.0);
        self.scalar_v2068 = v2068;
        let v2072: bool = (!v2068);
        self.scalar_v2072 = v2072;
        let v2128: f64 = (if v645 { 0.0 } else { 0.0 });
        self.scalar_v2128 = v2128;
        let v2133: f64 = (if v653 { 0.0 } else { 0.0 });
        self.scalar_v2133 = v2133;
        let v2136: f64 = (p.p3 * -1.0);
        self.scalar_v2136 = v2136;
        let v2137: f64 = (v2136 + p.p3);
        self.scalar_v2137 = v2137;
        let v2138: f64 = (-p.p3);
        self.scalar_v2138 = v2138;
        let v2139: f64 = (v2136 - v2136);
        self.scalar_v2139 = v2139;
        let v2140: f64 = (v2138 - v2136);
        self.scalar_v2140 = v2140;
        let v2141: f64 = (-v2136);
        self.scalar_v2141 = v2141;
        let v2142: f64 = (v2138 + p.p3);
        self.scalar_v2142 = v2142;
        let v2143: f64 = (v2136 + v2141);
        self.scalar_v2143 = v2143;
        let v2144: f64 = (p.p3 + v2142);
        self.scalar_v2144 = v2144;
        let v2858: f64 = (v1067 - 1.0);
        self.scalar_v2858 = v2858;
        let v2873: f64 = (if v1076 { p.p3 } else { 0.0 });
        self.scalar_v2873 = v2873;
        let v2874: f64 = (if v1076 { v2136 } else { 0.0 });
        self.scalar_v2874 = v2874;
        let v2941: f64 = (p.p76 - 1.0);
        self.scalar_v2941 = v2941;
        let v2953: f64 = (v1112 - 1.0);
        self.scalar_v2953 = v2953;
        let v3178: f64 = (v2136 / 0.0001);
        self.scalar_v3178 = v3178;
        let v3179: f64 = (p.p3 / 0.0001);
        self.scalar_v3179 = v3179;
        let v3188: f64 = (-v3178);
        self.scalar_v3188 = v3188;
        let v3189: f64 = (-v3179);
        self.scalar_v3189 = v3189;
        let v3212: f64 = (v2136 / 0.001);
        self.scalar_v3212 = v3212;
        let v3213: f64 = (p.p3 / 0.001);
        self.scalar_v3213 = v3213;
        let v3224: f64 = (-v3212);
        self.scalar_v3224 = v3224;
        let v3225: f64 = (-v3213);
        self.scalar_v3225 = v3225;
        let v3561: f64 = (v1426 - 1.0);
        self.scalar_v3561 = v3561;
        let v3600: f64 = (v2136 * v39);
        self.scalar_v3600 = v3600;
        let v3601: f64 = (p.p3 * v39);
        self.scalar_v3601 = v3601;
        let v3644: f64 = (v2136 * 0.5);
        self.scalar_v3644 = v3644;
        let v3645: f64 = (p.p3 * 0.5);
        self.scalar_v3645 = v3645;
        let v3734: f64 = (v1519 - 1.0);
        self.scalar_v3734 = v3734;
        let v3773: f64 = (p.p3 * v74);
        self.scalar_v3773 = v3773;
        let v3774: f64 = (v2136 * v74);
        self.scalar_v3774 = v3774;
        let v4065: f64 = (p.p3 * v34);
        self.scalar_v4065 = v4065;
        let v4066: f64 = (v2136 * v34);
        self.scalar_v4066 = v4066;
        let v4262: f64 = (if v1693 { v2143 } else { 0.0 });
        self.scalar_v4262 = v4262;
        let v4263: f64 = (if v1693 { v2144 } else { 0.0 });
        self.scalar_v4263 = v4263;
        let v4264: f64 = (if v1693 { v2137 } else { 0.0 });
        self.scalar_v4264 = v4264;
        let v4265: f64 = (if v1693 { v2140 } else { 0.0 });
        self.scalar_v4265 = v4265;
        let v4266: f64 = (if v1693 { v2139 } else { 0.0 });
        self.scalar_v4266 = v4266;
        let v4267: f64 = (if v1693 { v2138 } else { 0.0 });
        self.scalar_v4267 = v4267;
        let v4530: f64 = (if v1736 { p.p3 } else { 0.0 });
        self.scalar_v4530 = v4530;
        let v4531: f64 = (if v1736 { v2137 } else { 0.0 });
        self.scalar_v4531 = v4531;
        let v4532: f64 = (if v1736 { v2136 } else { 0.0 });
        self.scalar_v4532 = v4532;
        let v4533: f64 = (-1.0 * v4530);
        self.scalar_v4533 = v4533;
        let v4534: f64 = (-1.0 * v4531);
        self.scalar_v4534 = v4534;
        let v4535: f64 = (-1.0 * v4532);
        self.scalar_v4535 = v4535;
        let v4536: f64 = (v4533 * -1.0);
        self.scalar_v4536 = v4536;
        let v4537: f64 = (v4534 * -1.0);
        self.scalar_v4537 = v4537;
        let v4538: f64 = (v4535 * -1.0);
        self.scalar_v4538 = v4538;
        let v4931: f64 = (p.p41 - 1.0);
        self.scalar_v4931 = v4931;
        let v5403: f64 = (p.p49 - 1.0);
        self.scalar_v5403 = v5403;
        let v5482: f64 = (p.p50 - 1.0);
        self.scalar_v5482 = v5482;
        let v5718: f64 = (0.0 * v2136);
        self.scalar_v5718 = v5718;
        let v5719: f64 = (0.0 * p.p3);
        self.scalar_v5719 = v5719;
        let v5720: f64 = (0.0 * v2137);
        self.scalar_v5720 = v5720;
        let v5721: f64 = (0.0 * v2140);
        self.scalar_v5721 = v5721;
        let v5722: f64 = (0.0 * v2139);
        self.scalar_v5722 = v5722;
        let v5723: f64 = (0.0 * v2138);
        self.scalar_v5723 = v5723;
        let v5884: f64 = (p.p3 * p.p3);
        self.scalar_v5884 = v5884;
        let v5885: f64 = (p.p3 * v2136);
        self.scalar_v5885 = v5885;
        let v5914: f64 = (p.p3 * v2141);
        self.scalar_v5914 = v5914;
        let v5915: f64 = (p.p3 * v2142);
        self.scalar_v5915 = v5915;
        let v5916: f64 = (p.p3 * v2137);
        self.scalar_v5916 = v5916;
        let v5917: f64 = (p.p3 * v2140);
        self.scalar_v5917 = v5917;
        let v5918: f64 = (p.p3 * v2139);
        self.scalar_v5918 = v5918;
        let v5919: f64 = (p.p3 * v2138);
        self.scalar_v5919 = v5919;
    }
}
