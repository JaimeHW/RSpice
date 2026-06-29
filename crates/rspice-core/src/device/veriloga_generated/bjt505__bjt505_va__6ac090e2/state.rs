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
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: bool,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v342: bool,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v345: bool,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v373: bool,
    pub(crate) scalar_v375: f64,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v394: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v469: bool,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v565: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v585: f64,
    pub(crate) scalar_v589: f64,
    pub(crate) scalar_v594: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v628: f64,
    pub(crate) scalar_v630: bool,
    pub(crate) scalar_v636: bool,
    pub(crate) scalar_v638: bool,
    pub(crate) scalar_v644: bool,
    pub(crate) scalar_v646: bool,
    pub(crate) scalar_v652: bool,
    pub(crate) scalar_v700: f64,
    pub(crate) scalar_v705: f64,
    pub(crate) scalar_v823: f64,
    pub(crate) scalar_v876: f64,
    pub(crate) scalar_v877: f64,
    pub(crate) scalar_v878: f64,
    pub(crate) scalar_v889: f64,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v911: f64,
    pub(crate) scalar_v912: f64,
    pub(crate) scalar_v913: f64,
    pub(crate) scalar_v914: f64,
    pub(crate) scalar_v915: f64,
    pub(crate) scalar_v962: f64,
    pub(crate) scalar_v972: f64,
    pub(crate) scalar_v985: f64,
    pub(crate) scalar_v986: bool,
    pub(crate) scalar_v990: bool,
    pub(crate) scalar_v1039: f64,
    pub(crate) scalar_v1040: f64,
    pub(crate) scalar_v1041: f64,
    pub(crate) scalar_v1063: f64,
    pub(crate) scalar_v1071: f64,
    pub(crate) scalar_v1072: bool,
    pub(crate) scalar_v1074: bool,
    pub(crate) scalar_v1075: bool,
    pub(crate) scalar_v1076: bool,
    pub(crate) scalar_v1079: bool,
    pub(crate) scalar_v1080: bool,
    pub(crate) scalar_v1085: f64,
    pub(crate) scalar_v1106: f64,
    pub(crate) scalar_v1108: f64,
    pub(crate) scalar_v1137: bool,
    pub(crate) scalar_v1143: bool,
    pub(crate) scalar_v1177: f64,
    pub(crate) scalar_v1199: f64,
    pub(crate) scalar_v1212: f64,
    pub(crate) scalar_v1230: f64,
    pub(crate) scalar_v1293: f64,
    pub(crate) scalar_v1294: bool,
    pub(crate) scalar_v1295: bool,
    pub(crate) scalar_v1296: bool,
    pub(crate) scalar_v1298: bool,
    pub(crate) scalar_v1299: bool,
    pub(crate) scalar_v1300: f64,
    pub(crate) scalar_v1393: bool,
    pub(crate) scalar_v1394: bool,
    pub(crate) scalar_v1395: bool,
    pub(crate) scalar_v1419: f64,
    pub(crate) scalar_v1421: f64,
    pub(crate) scalar_v1422: f64,
    pub(crate) scalar_v1424: f64,
    pub(crate) scalar_v1484: bool,
    pub(crate) scalar_v1485: bool,
    pub(crate) scalar_v1486: bool,
    pub(crate) scalar_v1512: f64,
    pub(crate) scalar_v1514: f64,
    pub(crate) scalar_v1515: f64,
    pub(crate) scalar_v1517: f64,
    pub(crate) scalar_v1583: f64,
    pub(crate) scalar_v1584: bool,
    pub(crate) scalar_v1585: f64,
    pub(crate) scalar_v1586: f64,
    pub(crate) scalar_v1592: f64,
    pub(crate) scalar_v1601: f64,
    pub(crate) scalar_v1602: f64,
    pub(crate) scalar_v1614: bool,
    pub(crate) scalar_v1633: f64,
    pub(crate) scalar_v1643: f64,
    pub(crate) scalar_v1644: bool,
    pub(crate) scalar_v1645: bool,
    pub(crate) scalar_v1646: bool,
    pub(crate) scalar_v1651: f64,
    pub(crate) scalar_v1661: bool,
    pub(crate) scalar_v1662: f64,
    pub(crate) scalar_v1663: f64,
    pub(crate) scalar_v1677: bool,
    pub(crate) scalar_v1685: bool,
    pub(crate) scalar_v1686: bool,
    pub(crate) scalar_v1699: f64,
    pub(crate) scalar_v1704: f64,
    pub(crate) scalar_v1721: bool,
    pub(crate) scalar_v1722: bool,
    pub(crate) scalar_v1728: f64,
    pub(crate) scalar_v1729: bool,
    pub(crate) scalar_v1732: f64,
    pub(crate) scalar_v1738: f64,
    pub(crate) scalar_v1749: f64,
    pub(crate) scalar_v1750: f64,
    pub(crate) scalar_v1751: f64,
    pub(crate) scalar_v1752: f64,
    pub(crate) scalar_v1753: f64,
    pub(crate) scalar_v1754: f64,
    pub(crate) scalar_v1755: f64,
    pub(crate) scalar_v1756: f64,
    pub(crate) scalar_v1757: f64,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1760: f64,
    pub(crate) scalar_v1761: f64,
    pub(crate) scalar_v1762: f64,
    pub(crate) scalar_v1763: f64,
    pub(crate) scalar_v1777: bool,
    pub(crate) scalar_v1804: f64,
    pub(crate) scalar_v1805: bool,
    pub(crate) scalar_v1806: f64,
    pub(crate) scalar_v1809: f64,
    pub(crate) scalar_v1828: f64,
    pub(crate) scalar_v1842: f64,
    pub(crate) scalar_v1847: bool,
    pub(crate) scalar_v1849: bool,
    pub(crate) scalar_v1853: f64,
    pub(crate) scalar_v1854: f64,
    pub(crate) scalar_v1855: f64,
    pub(crate) scalar_v1856: f64,
    pub(crate) scalar_v1857: f64,
    pub(crate) scalar_v1866: f64,
    pub(crate) scalar_v1867: bool,
    pub(crate) scalar_v1870: bool,
    pub(crate) scalar_v1893: f64,
    pub(crate) scalar_v1894: f64,
    pub(crate) scalar_v1900: f64,
    pub(crate) scalar_v1901: f64,
    pub(crate) scalar_v1902: f64,
    pub(crate) scalar_v1950: bool,
    pub(crate) scalar_v1951: bool,
    pub(crate) scalar_v1956: f64,
    pub(crate) scalar_v1960: f64,
    pub(crate) scalar_v1967: f64,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v1992: f64,
    pub(crate) scalar_v2012: f64,
    pub(crate) scalar_v2013: bool,
    pub(crate) scalar_v2048: bool,
    pub(crate) scalar_v2054: bool,
    pub(crate) scalar_v2059: f64,
    pub(crate) scalar_v2060: bool,
    pub(crate) scalar_v2064: bool,
    pub(crate) scalar_v2120: f64,
    pub(crate) scalar_v2125: f64,
    pub(crate) scalar_v2128: f64,
    pub(crate) scalar_v2129: f64,
    pub(crate) scalar_v2130: f64,
    pub(crate) scalar_v2131: f64,
    pub(crate) scalar_v2816: f64,
    pub(crate) scalar_v2831: f64,
    pub(crate) scalar_v2832: f64,
    pub(crate) scalar_v2899: f64,
    pub(crate) scalar_v2911: f64,
    pub(crate) scalar_v3132: f64,
    pub(crate) scalar_v3133: f64,
    pub(crate) scalar_v3142: f64,
    pub(crate) scalar_v3143: f64,
    pub(crate) scalar_v3166: f64,
    pub(crate) scalar_v3167: f64,
    pub(crate) scalar_v3178: f64,
    pub(crate) scalar_v3179: f64,
    pub(crate) scalar_v3504: f64,
    pub(crate) scalar_v3543: f64,
    pub(crate) scalar_v3544: f64,
    pub(crate) scalar_v3587: f64,
    pub(crate) scalar_v3588: f64,
    pub(crate) scalar_v3675: f64,
    pub(crate) scalar_v3714: f64,
    pub(crate) scalar_v3715: f64,
    pub(crate) scalar_v3992: f64,
    pub(crate) scalar_v3993: f64,
    pub(crate) scalar_v4138: f64,
    pub(crate) scalar_v4139: f64,
    pub(crate) scalar_v4140: f64,
    pub(crate) scalar_v4141: f64,
    pub(crate) scalar_v4363: f64,
    pub(crate) scalar_v4364: f64,
    pub(crate) scalar_v4365: f64,
    pub(crate) scalar_v4366: f64,
    pub(crate) scalar_v4367: f64,
    pub(crate) scalar_v4368: f64,
    pub(crate) scalar_v4733: f64,
    pub(crate) scalar_v5195: f64,
    pub(crate) scalar_v5274: f64,
    pub(crate) scalar_v5510: f64,
    pub(crate) scalar_v5511: f64,
    pub(crate) scalar_v5512: f64,
    pub(crate) scalar_v5513: f64,
    pub(crate) scalar_v5670: f64,
    pub(crate) scalar_v5671: f64,
    pub(crate) scalar_v5700: f64,
    pub(crate) scalar_v5701: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: bool,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: bool,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: bool,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: bool,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: bool,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: bool,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: bool,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: bool,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: bool,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: f64,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: bool,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v259: f64,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v279: bool,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: bool,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: bool,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v325: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: bool,
    pub(crate) scalar_v331: bool,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: bool,
    pub(crate) scalar_v355: bool,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: bool,
    pub(crate) scalar_v363: bool,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v382: bool,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v388: f64,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v390: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v467: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v530: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v536: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v538: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v549: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v558: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: f64,
    pub(crate) scalar_v566: f64,
    pub(crate) scalar_v567: f64,
    pub(crate) scalar_v568: f64,
    pub(crate) scalar_v569: f64,
    pub(crate) scalar_v570: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v572: f64,
    pub(crate) scalar_v573: f64,
    pub(crate) scalar_v574: f64,
    pub(crate) scalar_v575: f64,
    pub(crate) scalar_v576: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v579: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v582: f64,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v587: f64,
    pub(crate) scalar_v588: f64,
    pub(crate) scalar_v590: f64,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v592: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v598: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v609: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v614: bool,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v619: f64,
    pub(crate) scalar_v620: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v624: bool,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v629: f64,
    pub(crate) scalar_v631: f64,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v633: bool,
    pub(crate) scalar_v634: bool,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v637: f64,
    pub(crate) scalar_v639: f64,
    pub(crate) scalar_v640: f64,
    pub(crate) scalar_v641: bool,
    pub(crate) scalar_v642: bool,
    pub(crate) scalar_v643: f64,
    pub(crate) scalar_v645: f64,
    pub(crate) scalar_v647: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v649: bool,
    pub(crate) scalar_v650: bool,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v881: f64,
    pub(crate) scalar_v968: f64,
    pub(crate) scalar_v969: f64,
    pub(crate) scalar_v976: f64,
    pub(crate) scalar_v977: f64,
    pub(crate) scalar_v988: f64,
    pub(crate) scalar_v1011: f64,
    pub(crate) scalar_v1015: f64,
    pub(crate) scalar_v1042: f64,
    pub(crate) scalar_v1043: f64,
    pub(crate) scalar_v1065: f64,
    pub(crate) scalar_v1082: f64,
    pub(crate) scalar_v1083: f64,
    pub(crate) scalar_v1084: f64,
    pub(crate) scalar_v1086: f64,
    pub(crate) scalar_v1087: f64,
    pub(crate) scalar_v1088: f64,
    pub(crate) scalar_v1109: f64,
    pub(crate) scalar_v1123: f64,
    pub(crate) scalar_v1124: f64,
    pub(crate) scalar_v1130: f64,
    pub(crate) scalar_v1155: f64,
    pub(crate) scalar_v1156: f64,
    pub(crate) scalar_v1157: f64,
    pub(crate) scalar_v1178: f64,
    pub(crate) scalar_v1276: f64,
    pub(crate) scalar_v1335: f64,
    pub(crate) scalar_v1475: f64,
    pub(crate) scalar_v1564: f64,
    pub(crate) scalar_v1573: f64,
    pub(crate) scalar_v1576: f64,
    pub(crate) scalar_v1577: f64,
    pub(crate) scalar_v1587: f64,
    pub(crate) scalar_v1590: f64,
    pub(crate) scalar_v1591: f64,
    pub(crate) scalar_v1603: f64,
    pub(crate) scalar_v1630: f64,
    pub(crate) scalar_v1634: f64,
    pub(crate) scalar_v1635: f64,
    pub(crate) scalar_v1652: f64,
    pub(crate) scalar_v1664: f64,
    pub(crate) scalar_v1667: f64,
    pub(crate) scalar_v1668: f64,
    pub(crate) scalar_v1687: f64,
    pub(crate) scalar_v1688: f64,
    pub(crate) scalar_v1689: f64,
    pub(crate) scalar_v1690: f64,
    pub(crate) scalar_v1691: f64,
    pub(crate) scalar_v1692: f64,
    pub(crate) scalar_v1693: f64,
    pub(crate) scalar_v1694: f64,
    pub(crate) scalar_v1695: f64,
    pub(crate) scalar_v1827: f64,
    pub(crate) scalar_v1843: f64,
    pub(crate) scalar_v1932: f64,
    pub(crate) scalar_v1935: f64,
    pub(crate) scalar_v2132: f64,
    pub(crate) scalar_v2133: f64,
    pub(crate) scalar_v2142: f64,
    pub(crate) scalar_v2143: f64,
    pub(crate) scalar_v2152: f64,
    pub(crate) scalar_v2153: f64,
    pub(crate) scalar_v2178: f64,
    pub(crate) scalar_v2788: f64,
    pub(crate) scalar_v2789: f64,
    pub(crate) scalar_v2800: f64,
    pub(crate) scalar_v2801: f64,
    pub(crate) scalar_v2953: f64,
    pub(crate) scalar_v2954: f64,
    pub(crate) scalar_v2971: f64,
    pub(crate) scalar_v3204: f64,
    pub(crate) scalar_v3205: f64,
    pub(crate) scalar_v3337: f64,
    pub(crate) scalar_v3338: f64,
    pub(crate) scalar_v3394: f64,
    pub(crate) scalar_v3395: f64,
    pub(crate) scalar_v3409: f64,
    pub(crate) scalar_v3410: f64,
    pub(crate) scalar_v3424: f64,
    pub(crate) scalar_v3425: f64,
    pub(crate) scalar_v3426: f64,
    pub(crate) scalar_v3427: f64,
    pub(crate) scalar_v3451: f64,
    pub(crate) scalar_v3452: f64,
    pub(crate) scalar_v3493: f64,
    pub(crate) scalar_v3494: f64,
    pub(crate) scalar_v3545: f64,
    pub(crate) scalar_v3546: f64,
    pub(crate) scalar_v3635: f64,
    pub(crate) scalar_v3636: f64,
    pub(crate) scalar_v3637: f64,
    pub(crate) scalar_v3638: f64,
    pub(crate) scalar_v3716: f64,
    pub(crate) scalar_v3717: f64,
    pub(crate) scalar_v5317: f64,
    pub(crate) scalar_v5318: f64,
    pub(crate) scalar_v5672: f64,
    pub(crate) scalar_v5673: f64,
    pub(crate) scalar_v5674: f64,
    pub(crate) scalar_v5675: f64,
    pub(crate) scalar_v5676: f64,
    pub(crate) scalar_v5677: f64,
    pub(crate) scalar_v5678: f64,
    pub(crate) scalar_v5679: f64,
    pub(crate) scalar_v5702: f64,
    pub(crate) scalar_v5703: f64,
    pub(crate) scalar_v5704: f64,
    pub(crate) scalar_v5705: f64,
    pub(crate) scalar_v5706: f64,
    pub(crate) scalar_v5707: f64,
    pub(crate) scalar_v5708: f64,
    pub(crate) scalar_v5709: f64,
    pub(crate) scalar_v5737: f64,
    pub(crate) scalar_v5738: f64,
    pub(crate) scalar_v5739: f64,
    pub(crate) scalar_v5740: f64,
    pub(crate) scalar_v5741: f64,
    pub(crate) scalar_v5742: f64,
    pub(crate) scalar_v5743: f64,
    pub(crate) scalar_v5744: f64,
    pub(crate) scalar_v5745: f64,
    pub(crate) scalar_v5746: f64,
    pub(crate) scalar_v5747: f64,
    pub(crate) scalar_v5748: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
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
            scalar_v109: self.scalar_v109,
            scalar_v111: self.scalar_v111,
            scalar_v166: self.scalar_v166,
            scalar_v186: self.scalar_v186,
            scalar_v189: self.scalar_v189,
            scalar_v229: self.scalar_v229,
            scalar_v232: self.scalar_v232,
            scalar_v252: self.scalar_v252,
            scalar_v255: self.scalar_v255,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v305: self.scalar_v305,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v314: self.scalar_v314,
            scalar_v342: self.scalar_v342,
            scalar_v344: self.scalar_v344,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v373: self.scalar_v373,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v394: self.scalar_v394,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v404: self.scalar_v404,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v420: self.scalar_v420,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v437: self.scalar_v437,
            scalar_v441: self.scalar_v441,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v450: self.scalar_v450,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v478: self.scalar_v478,
            scalar_v479: self.scalar_v479,
            scalar_v480: self.scalar_v480,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v499: self.scalar_v499,
            scalar_v500: self.scalar_v500,
            scalar_v504: self.scalar_v504,
            scalar_v505: self.scalar_v505,
            scalar_v506: self.scalar_v506,
            scalar_v507: self.scalar_v507,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v523: self.scalar_v523,
            scalar_v526: self.scalar_v526,
            scalar_v534: self.scalar_v534,
            scalar_v543: self.scalar_v543,
            scalar_v556: self.scalar_v556,
            scalar_v565: self.scalar_v565,
            scalar_v577: self.scalar_v577,
            scalar_v580: self.scalar_v580,
            scalar_v583: self.scalar_v583,
            scalar_v584: self.scalar_v584,
            scalar_v585: self.scalar_v585,
            scalar_v589: self.scalar_v589,
            scalar_v594: self.scalar_v594,
            scalar_v595: self.scalar_v595,
            scalar_v596: self.scalar_v596,
            scalar_v601: self.scalar_v601,
            scalar_v602: self.scalar_v602,
            scalar_v606: self.scalar_v606,
            scalar_v607: self.scalar_v607,
            scalar_v626: self.scalar_v626,
            scalar_v628: self.scalar_v628,
            scalar_v630: self.scalar_v630,
            scalar_v636: self.scalar_v636,
            scalar_v638: self.scalar_v638,
            scalar_v644: self.scalar_v644,
            scalar_v646: self.scalar_v646,
            scalar_v652: self.scalar_v652,
            scalar_v700: self.scalar_v700,
            scalar_v705: self.scalar_v705,
            scalar_v823: self.scalar_v823,
            scalar_v876: self.scalar_v876,
            scalar_v877: self.scalar_v877,
            scalar_v878: self.scalar_v878,
            scalar_v889: self.scalar_v889,
            scalar_v910: self.scalar_v910,
            scalar_v911: self.scalar_v911,
            scalar_v912: self.scalar_v912,
            scalar_v913: self.scalar_v913,
            scalar_v914: self.scalar_v914,
            scalar_v915: self.scalar_v915,
            scalar_v962: self.scalar_v962,
            scalar_v972: self.scalar_v972,
            scalar_v985: self.scalar_v985,
            scalar_v986: self.scalar_v986,
            scalar_v990: self.scalar_v990,
            scalar_v1039: self.scalar_v1039,
            scalar_v1040: self.scalar_v1040,
            scalar_v1041: self.scalar_v1041,
            scalar_v1063: self.scalar_v1063,
            scalar_v1071: self.scalar_v1071,
            scalar_v1072: self.scalar_v1072,
            scalar_v1074: self.scalar_v1074,
            scalar_v1075: self.scalar_v1075,
            scalar_v1076: self.scalar_v1076,
            scalar_v1079: self.scalar_v1079,
            scalar_v1080: self.scalar_v1080,
            scalar_v1085: self.scalar_v1085,
            scalar_v1106: self.scalar_v1106,
            scalar_v1108: self.scalar_v1108,
            scalar_v1137: self.scalar_v1137,
            scalar_v1143: self.scalar_v1143,
            scalar_v1177: self.scalar_v1177,
            scalar_v1199: self.scalar_v1199,
            scalar_v1212: self.scalar_v1212,
            scalar_v1230: self.scalar_v1230,
            scalar_v1293: self.scalar_v1293,
            scalar_v1294: self.scalar_v1294,
            scalar_v1295: self.scalar_v1295,
            scalar_v1296: self.scalar_v1296,
            scalar_v1298: self.scalar_v1298,
            scalar_v1299: self.scalar_v1299,
            scalar_v1300: self.scalar_v1300,
            scalar_v1393: self.scalar_v1393,
            scalar_v1394: self.scalar_v1394,
            scalar_v1395: self.scalar_v1395,
            scalar_v1419: self.scalar_v1419,
            scalar_v1421: self.scalar_v1421,
            scalar_v1422: self.scalar_v1422,
            scalar_v1424: self.scalar_v1424,
            scalar_v1484: self.scalar_v1484,
            scalar_v1485: self.scalar_v1485,
            scalar_v1486: self.scalar_v1486,
            scalar_v1512: self.scalar_v1512,
            scalar_v1514: self.scalar_v1514,
            scalar_v1515: self.scalar_v1515,
            scalar_v1517: self.scalar_v1517,
            scalar_v1583: self.scalar_v1583,
            scalar_v1584: self.scalar_v1584,
            scalar_v1585: self.scalar_v1585,
            scalar_v1586: self.scalar_v1586,
            scalar_v1592: self.scalar_v1592,
            scalar_v1601: self.scalar_v1601,
            scalar_v1602: self.scalar_v1602,
            scalar_v1614: self.scalar_v1614,
            scalar_v1633: self.scalar_v1633,
            scalar_v1643: self.scalar_v1643,
            scalar_v1644: self.scalar_v1644,
            scalar_v1645: self.scalar_v1645,
            scalar_v1646: self.scalar_v1646,
            scalar_v1651: self.scalar_v1651,
            scalar_v1661: self.scalar_v1661,
            scalar_v1662: self.scalar_v1662,
            scalar_v1663: self.scalar_v1663,
            scalar_v1677: self.scalar_v1677,
            scalar_v1685: self.scalar_v1685,
            scalar_v1686: self.scalar_v1686,
            scalar_v1699: self.scalar_v1699,
            scalar_v1704: self.scalar_v1704,
            scalar_v1721: self.scalar_v1721,
            scalar_v1722: self.scalar_v1722,
            scalar_v1728: self.scalar_v1728,
            scalar_v1729: self.scalar_v1729,
            scalar_v1732: self.scalar_v1732,
            scalar_v1738: self.scalar_v1738,
            scalar_v1749: self.scalar_v1749,
            scalar_v1750: self.scalar_v1750,
            scalar_v1751: self.scalar_v1751,
            scalar_v1752: self.scalar_v1752,
            scalar_v1753: self.scalar_v1753,
            scalar_v1754: self.scalar_v1754,
            scalar_v1755: self.scalar_v1755,
            scalar_v1756: self.scalar_v1756,
            scalar_v1757: self.scalar_v1757,
            scalar_v1758: self.scalar_v1758,
            scalar_v1759: self.scalar_v1759,
            scalar_v1760: self.scalar_v1760,
            scalar_v1761: self.scalar_v1761,
            scalar_v1762: self.scalar_v1762,
            scalar_v1763: self.scalar_v1763,
            scalar_v1777: self.scalar_v1777,
            scalar_v1804: self.scalar_v1804,
            scalar_v1805: self.scalar_v1805,
            scalar_v1806: self.scalar_v1806,
            scalar_v1809: self.scalar_v1809,
            scalar_v1828: self.scalar_v1828,
            scalar_v1842: self.scalar_v1842,
            scalar_v1847: self.scalar_v1847,
            scalar_v1849: self.scalar_v1849,
            scalar_v1853: self.scalar_v1853,
            scalar_v1854: self.scalar_v1854,
            scalar_v1855: self.scalar_v1855,
            scalar_v1856: self.scalar_v1856,
            scalar_v1857: self.scalar_v1857,
            scalar_v1866: self.scalar_v1866,
            scalar_v1867: self.scalar_v1867,
            scalar_v1870: self.scalar_v1870,
            scalar_v1893: self.scalar_v1893,
            scalar_v1894: self.scalar_v1894,
            scalar_v1900: self.scalar_v1900,
            scalar_v1901: self.scalar_v1901,
            scalar_v1902: self.scalar_v1902,
            scalar_v1950: self.scalar_v1950,
            scalar_v1951: self.scalar_v1951,
            scalar_v1956: self.scalar_v1956,
            scalar_v1960: self.scalar_v1960,
            scalar_v1967: self.scalar_v1967,
            scalar_v1972: self.scalar_v1972,
            scalar_v1992: self.scalar_v1992,
            scalar_v2012: self.scalar_v2012,
            scalar_v2013: self.scalar_v2013,
            scalar_v2048: self.scalar_v2048,
            scalar_v2054: self.scalar_v2054,
            scalar_v2059: self.scalar_v2059,
            scalar_v2060: self.scalar_v2060,
            scalar_v2064: self.scalar_v2064,
            scalar_v2120: self.scalar_v2120,
            scalar_v2125: self.scalar_v2125,
            scalar_v2128: self.scalar_v2128,
            scalar_v2129: self.scalar_v2129,
            scalar_v2130: self.scalar_v2130,
            scalar_v2131: self.scalar_v2131,
            scalar_v2816: self.scalar_v2816,
            scalar_v2831: self.scalar_v2831,
            scalar_v2832: self.scalar_v2832,
            scalar_v2899: self.scalar_v2899,
            scalar_v2911: self.scalar_v2911,
            scalar_v3132: self.scalar_v3132,
            scalar_v3133: self.scalar_v3133,
            scalar_v3142: self.scalar_v3142,
            scalar_v3143: self.scalar_v3143,
            scalar_v3166: self.scalar_v3166,
            scalar_v3167: self.scalar_v3167,
            scalar_v3178: self.scalar_v3178,
            scalar_v3179: self.scalar_v3179,
            scalar_v3504: self.scalar_v3504,
            scalar_v3543: self.scalar_v3543,
            scalar_v3544: self.scalar_v3544,
            scalar_v3587: self.scalar_v3587,
            scalar_v3588: self.scalar_v3588,
            scalar_v3675: self.scalar_v3675,
            scalar_v3714: self.scalar_v3714,
            scalar_v3715: self.scalar_v3715,
            scalar_v3992: self.scalar_v3992,
            scalar_v3993: self.scalar_v3993,
            scalar_v4138: self.scalar_v4138,
            scalar_v4139: self.scalar_v4139,
            scalar_v4140: self.scalar_v4140,
            scalar_v4141: self.scalar_v4141,
            scalar_v4363: self.scalar_v4363,
            scalar_v4364: self.scalar_v4364,
            scalar_v4365: self.scalar_v4365,
            scalar_v4366: self.scalar_v4366,
            scalar_v4367: self.scalar_v4367,
            scalar_v4368: self.scalar_v4368,
            scalar_v4733: self.scalar_v4733,
            scalar_v5195: self.scalar_v5195,
            scalar_v5274: self.scalar_v5274,
            scalar_v5510: self.scalar_v5510,
            scalar_v5511: self.scalar_v5511,
            scalar_v5512: self.scalar_v5512,
            scalar_v5513: self.scalar_v5513,
            scalar_v5670: self.scalar_v5670,
            scalar_v5671: self.scalar_v5671,
            scalar_v5700: self.scalar_v5700,
            scalar_v5701: self.scalar_v5701,
            scalar_v20: self.scalar_v20,
            scalar_v106: self.scalar_v106,
            scalar_v108: self.scalar_v108,
            scalar_v110: self.scalar_v110,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
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
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v198: self.scalar_v198,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v204: self.scalar_v204,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v210: self.scalar_v210,
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
            scalar_v219: self.scalar_v219,
            scalar_v220: self.scalar_v220,
            scalar_v221: self.scalar_v221,
            scalar_v222: self.scalar_v222,
            scalar_v223: self.scalar_v223,
            scalar_v224: self.scalar_v224,
            scalar_v225: self.scalar_v225,
            scalar_v226: self.scalar_v226,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v233: self.scalar_v233,
            scalar_v234: self.scalar_v234,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v238: self.scalar_v238,
            scalar_v239: self.scalar_v239,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v268: self.scalar_v268,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v306: self.scalar_v306,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v343: self.scalar_v343,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v360: self.scalar_v360,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v371: self.scalar_v371,
            scalar_v372: self.scalar_v372,
            scalar_v374: self.scalar_v374,
            scalar_v377: self.scalar_v377,
            scalar_v378: self.scalar_v378,
            scalar_v379: self.scalar_v379,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v388: self.scalar_v388,
            scalar_v389: self.scalar_v389,
            scalar_v390: self.scalar_v390,
            scalar_v391: self.scalar_v391,
            scalar_v392: self.scalar_v392,
            scalar_v393: self.scalar_v393,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v407: self.scalar_v407,
            scalar_v408: self.scalar_v408,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v449: self.scalar_v449,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v465: self.scalar_v465,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v484: self.scalar_v484,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v501: self.scalar_v501,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v508: self.scalar_v508,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v511: self.scalar_v511,
            scalar_v512: self.scalar_v512,
            scalar_v513: self.scalar_v513,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v519: self.scalar_v519,
            scalar_v520: self.scalar_v520,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v524: self.scalar_v524,
            scalar_v525: self.scalar_v525,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v530: self.scalar_v530,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v535: self.scalar_v535,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v538: self.scalar_v538,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v549: self.scalar_v549,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v553: self.scalar_v553,
            scalar_v554: self.scalar_v554,
            scalar_v555: self.scalar_v555,
            scalar_v557: self.scalar_v557,
            scalar_v558: self.scalar_v558,
            scalar_v559: self.scalar_v559,
            scalar_v560: self.scalar_v560,
            scalar_v561: self.scalar_v561,
            scalar_v562: self.scalar_v562,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v566: self.scalar_v566,
            scalar_v567: self.scalar_v567,
            scalar_v568: self.scalar_v568,
            scalar_v569: self.scalar_v569,
            scalar_v570: self.scalar_v570,
            scalar_v571: self.scalar_v571,
            scalar_v572: self.scalar_v572,
            scalar_v573: self.scalar_v573,
            scalar_v574: self.scalar_v574,
            scalar_v575: self.scalar_v575,
            scalar_v576: self.scalar_v576,
            scalar_v578: self.scalar_v578,
            scalar_v579: self.scalar_v579,
            scalar_v581: self.scalar_v581,
            scalar_v582: self.scalar_v582,
            scalar_v586: self.scalar_v586,
            scalar_v587: self.scalar_v587,
            scalar_v588: self.scalar_v588,
            scalar_v590: self.scalar_v590,
            scalar_v591: self.scalar_v591,
            scalar_v592: self.scalar_v592,
            scalar_v597: self.scalar_v597,
            scalar_v598: self.scalar_v598,
            scalar_v599: self.scalar_v599,
            scalar_v600: self.scalar_v600,
            scalar_v603: self.scalar_v603,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v608: self.scalar_v608,
            scalar_v609: self.scalar_v609,
            scalar_v610: self.scalar_v610,
            scalar_v612: self.scalar_v612,
            scalar_v614: self.scalar_v614,
            scalar_v616: self.scalar_v616,
            scalar_v617: self.scalar_v617,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v624: self.scalar_v624,
            scalar_v627: self.scalar_v627,
            scalar_v629: self.scalar_v629,
            scalar_v631: self.scalar_v631,
            scalar_v632: self.scalar_v632,
            scalar_v633: self.scalar_v633,
            scalar_v634: self.scalar_v634,
            scalar_v635: self.scalar_v635,
            scalar_v637: self.scalar_v637,
            scalar_v639: self.scalar_v639,
            scalar_v640: self.scalar_v640,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v643: self.scalar_v643,
            scalar_v645: self.scalar_v645,
            scalar_v647: self.scalar_v647,
            scalar_v648: self.scalar_v648,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v653: self.scalar_v653,
            scalar_v846: self.scalar_v846,
            scalar_v857: self.scalar_v857,
            scalar_v881: self.scalar_v881,
            scalar_v968: self.scalar_v968,
            scalar_v969: self.scalar_v969,
            scalar_v976: self.scalar_v976,
            scalar_v977: self.scalar_v977,
            scalar_v988: self.scalar_v988,
            scalar_v1011: self.scalar_v1011,
            scalar_v1015: self.scalar_v1015,
            scalar_v1042: self.scalar_v1042,
            scalar_v1043: self.scalar_v1043,
            scalar_v1065: self.scalar_v1065,
            scalar_v1082: self.scalar_v1082,
            scalar_v1083: self.scalar_v1083,
            scalar_v1084: self.scalar_v1084,
            scalar_v1086: self.scalar_v1086,
            scalar_v1087: self.scalar_v1087,
            scalar_v1088: self.scalar_v1088,
            scalar_v1109: self.scalar_v1109,
            scalar_v1123: self.scalar_v1123,
            scalar_v1124: self.scalar_v1124,
            scalar_v1130: self.scalar_v1130,
            scalar_v1155: self.scalar_v1155,
            scalar_v1156: self.scalar_v1156,
            scalar_v1157: self.scalar_v1157,
            scalar_v1178: self.scalar_v1178,
            scalar_v1276: self.scalar_v1276,
            scalar_v1335: self.scalar_v1335,
            scalar_v1475: self.scalar_v1475,
            scalar_v1564: self.scalar_v1564,
            scalar_v1573: self.scalar_v1573,
            scalar_v1576: self.scalar_v1576,
            scalar_v1577: self.scalar_v1577,
            scalar_v1587: self.scalar_v1587,
            scalar_v1590: self.scalar_v1590,
            scalar_v1591: self.scalar_v1591,
            scalar_v1603: self.scalar_v1603,
            scalar_v1630: self.scalar_v1630,
            scalar_v1634: self.scalar_v1634,
            scalar_v1635: self.scalar_v1635,
            scalar_v1652: self.scalar_v1652,
            scalar_v1664: self.scalar_v1664,
            scalar_v1667: self.scalar_v1667,
            scalar_v1668: self.scalar_v1668,
            scalar_v1687: self.scalar_v1687,
            scalar_v1688: self.scalar_v1688,
            scalar_v1689: self.scalar_v1689,
            scalar_v1690: self.scalar_v1690,
            scalar_v1691: self.scalar_v1691,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1694: self.scalar_v1694,
            scalar_v1695: self.scalar_v1695,
            scalar_v1827: self.scalar_v1827,
            scalar_v1843: self.scalar_v1843,
            scalar_v1932: self.scalar_v1932,
            scalar_v1935: self.scalar_v1935,
            scalar_v2132: self.scalar_v2132,
            scalar_v2133: self.scalar_v2133,
            scalar_v2142: self.scalar_v2142,
            scalar_v2143: self.scalar_v2143,
            scalar_v2152: self.scalar_v2152,
            scalar_v2153: self.scalar_v2153,
            scalar_v2178: self.scalar_v2178,
            scalar_v2788: self.scalar_v2788,
            scalar_v2789: self.scalar_v2789,
            scalar_v2800: self.scalar_v2800,
            scalar_v2801: self.scalar_v2801,
            scalar_v2953: self.scalar_v2953,
            scalar_v2954: self.scalar_v2954,
            scalar_v2971: self.scalar_v2971,
            scalar_v3204: self.scalar_v3204,
            scalar_v3205: self.scalar_v3205,
            scalar_v3337: self.scalar_v3337,
            scalar_v3338: self.scalar_v3338,
            scalar_v3394: self.scalar_v3394,
            scalar_v3395: self.scalar_v3395,
            scalar_v3409: self.scalar_v3409,
            scalar_v3410: self.scalar_v3410,
            scalar_v3424: self.scalar_v3424,
            scalar_v3425: self.scalar_v3425,
            scalar_v3426: self.scalar_v3426,
            scalar_v3427: self.scalar_v3427,
            scalar_v3451: self.scalar_v3451,
            scalar_v3452: self.scalar_v3452,
            scalar_v3493: self.scalar_v3493,
            scalar_v3494: self.scalar_v3494,
            scalar_v3545: self.scalar_v3545,
            scalar_v3546: self.scalar_v3546,
            scalar_v3635: self.scalar_v3635,
            scalar_v3636: self.scalar_v3636,
            scalar_v3637: self.scalar_v3637,
            scalar_v3638: self.scalar_v3638,
            scalar_v3716: self.scalar_v3716,
            scalar_v3717: self.scalar_v3717,
            scalar_v5317: self.scalar_v5317,
            scalar_v5318: self.scalar_v5318,
            scalar_v5672: self.scalar_v5672,
            scalar_v5673: self.scalar_v5673,
            scalar_v5674: self.scalar_v5674,
            scalar_v5675: self.scalar_v5675,
            scalar_v5676: self.scalar_v5676,
            scalar_v5677: self.scalar_v5677,
            scalar_v5678: self.scalar_v5678,
            scalar_v5679: self.scalar_v5679,
            scalar_v5702: self.scalar_v5702,
            scalar_v5703: self.scalar_v5703,
            scalar_v5704: self.scalar_v5704,
            scalar_v5705: self.scalar_v5705,
            scalar_v5706: self.scalar_v5706,
            scalar_v5707: self.scalar_v5707,
            scalar_v5708: self.scalar_v5708,
            scalar_v5709: self.scalar_v5709,
            scalar_v5737: self.scalar_v5737,
            scalar_v5738: self.scalar_v5738,
            scalar_v5739: self.scalar_v5739,
            scalar_v5740: self.scalar_v5740,
            scalar_v5741: self.scalar_v5741,
            scalar_v5742: self.scalar_v5742,
            scalar_v5743: self.scalar_v5743,
            scalar_v5744: self.scalar_v5744,
            scalar_v5745: self.scalar_v5745,
            scalar_v5746: self.scalar_v5746,
            scalar_v5747: self.scalar_v5747,
            scalar_v5748: self.scalar_v5748,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
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
            scalar_v109: 0.0,
            scalar_v111: 0.0,
            scalar_v166: 0.0,
            scalar_v186: 0.0,
            scalar_v189: 0.0,
            scalar_v229: 0.0,
            scalar_v232: 0.0,
            scalar_v252: 0.0,
            scalar_v255: 0.0,
            scalar_v266: 0.0,
            scalar_v267: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v305: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v312: 0.0,
            scalar_v313: false,
            scalar_v314: 0.0,
            scalar_v342: false,
            scalar_v344: 0.0,
            scalar_v345: false,
            scalar_v346: 0.0,
            scalar_v373: false,
            scalar_v375: 0.0,
            scalar_v376: 0.0,
            scalar_v394: 0.0,
            scalar_v396: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v404: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v420: 0.0,
            scalar_v422: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v428: 0.0,
            scalar_v429: 0.0,
            scalar_v434: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v437: 0.0,
            scalar_v441: 0.0,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v450: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v468: 0.0,
            scalar_v469: false,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v478: 0.0,
            scalar_v479: 0.0,
            scalar_v480: 0.0,
            scalar_v485: 0.0,
            scalar_v486: 0.0,
            scalar_v487: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v499: 0.0,
            scalar_v500: 0.0,
            scalar_v504: 0.0,
            scalar_v505: 0.0,
            scalar_v506: 0.0,
            scalar_v507: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v523: 0.0,
            scalar_v526: 0.0,
            scalar_v534: 0.0,
            scalar_v543: 0.0,
            scalar_v556: 0.0,
            scalar_v565: 0.0,
            scalar_v577: 0.0,
            scalar_v580: 0.0,
            scalar_v583: 0.0,
            scalar_v584: 0.0,
            scalar_v585: 0.0,
            scalar_v589: 0.0,
            scalar_v594: 0.0,
            scalar_v595: 0.0,
            scalar_v596: 0.0,
            scalar_v601: 0.0,
            scalar_v602: 0.0,
            scalar_v606: 0.0,
            scalar_v607: 0.0,
            scalar_v626: 0.0,
            scalar_v628: 0.0,
            scalar_v630: false,
            scalar_v636: false,
            scalar_v638: false,
            scalar_v644: false,
            scalar_v646: false,
            scalar_v652: false,
            scalar_v700: 0.0,
            scalar_v705: 0.0,
            scalar_v823: 0.0,
            scalar_v876: 0.0,
            scalar_v877: 0.0,
            scalar_v878: 0.0,
            scalar_v889: 0.0,
            scalar_v910: 0.0,
            scalar_v911: 0.0,
            scalar_v912: 0.0,
            scalar_v913: 0.0,
            scalar_v914: 0.0,
            scalar_v915: 0.0,
            scalar_v962: 0.0,
            scalar_v972: 0.0,
            scalar_v985: 0.0,
            scalar_v986: false,
            scalar_v990: false,
            scalar_v1039: 0.0,
            scalar_v1040: 0.0,
            scalar_v1041: 0.0,
            scalar_v1063: 0.0,
            scalar_v1071: 0.0,
            scalar_v1072: false,
            scalar_v1074: false,
            scalar_v1075: false,
            scalar_v1076: false,
            scalar_v1079: false,
            scalar_v1080: false,
            scalar_v1085: 0.0,
            scalar_v1106: 0.0,
            scalar_v1108: 0.0,
            scalar_v1137: false,
            scalar_v1143: false,
            scalar_v1177: 0.0,
            scalar_v1199: 0.0,
            scalar_v1212: 0.0,
            scalar_v1230: 0.0,
            scalar_v1293: 0.0,
            scalar_v1294: false,
            scalar_v1295: false,
            scalar_v1296: false,
            scalar_v1298: false,
            scalar_v1299: false,
            scalar_v1300: 0.0,
            scalar_v1393: false,
            scalar_v1394: false,
            scalar_v1395: false,
            scalar_v1419: 0.0,
            scalar_v1421: 0.0,
            scalar_v1422: 0.0,
            scalar_v1424: 0.0,
            scalar_v1484: false,
            scalar_v1485: false,
            scalar_v1486: false,
            scalar_v1512: 0.0,
            scalar_v1514: 0.0,
            scalar_v1515: 0.0,
            scalar_v1517: 0.0,
            scalar_v1583: 0.0,
            scalar_v1584: false,
            scalar_v1585: 0.0,
            scalar_v1586: 0.0,
            scalar_v1592: 0.0,
            scalar_v1601: 0.0,
            scalar_v1602: 0.0,
            scalar_v1614: false,
            scalar_v1633: 0.0,
            scalar_v1643: 0.0,
            scalar_v1644: false,
            scalar_v1645: false,
            scalar_v1646: false,
            scalar_v1651: 0.0,
            scalar_v1661: false,
            scalar_v1662: 0.0,
            scalar_v1663: 0.0,
            scalar_v1677: false,
            scalar_v1685: false,
            scalar_v1686: false,
            scalar_v1699: 0.0,
            scalar_v1704: 0.0,
            scalar_v1721: false,
            scalar_v1722: false,
            scalar_v1728: 0.0,
            scalar_v1729: false,
            scalar_v1732: 0.0,
            scalar_v1738: 0.0,
            scalar_v1749: 0.0,
            scalar_v1750: 0.0,
            scalar_v1751: 0.0,
            scalar_v1752: 0.0,
            scalar_v1753: 0.0,
            scalar_v1754: 0.0,
            scalar_v1755: 0.0,
            scalar_v1756: 0.0,
            scalar_v1757: 0.0,
            scalar_v1758: 0.0,
            scalar_v1759: 0.0,
            scalar_v1760: 0.0,
            scalar_v1761: 0.0,
            scalar_v1762: 0.0,
            scalar_v1763: 0.0,
            scalar_v1777: false,
            scalar_v1804: 0.0,
            scalar_v1805: false,
            scalar_v1806: 0.0,
            scalar_v1809: 0.0,
            scalar_v1828: 0.0,
            scalar_v1842: 0.0,
            scalar_v1847: false,
            scalar_v1849: false,
            scalar_v1853: 0.0,
            scalar_v1854: 0.0,
            scalar_v1855: 0.0,
            scalar_v1856: 0.0,
            scalar_v1857: 0.0,
            scalar_v1866: 0.0,
            scalar_v1867: false,
            scalar_v1870: false,
            scalar_v1893: 0.0,
            scalar_v1894: 0.0,
            scalar_v1900: 0.0,
            scalar_v1901: 0.0,
            scalar_v1902: 0.0,
            scalar_v1950: false,
            scalar_v1951: false,
            scalar_v1956: 0.0,
            scalar_v1960: 0.0,
            scalar_v1967: 0.0,
            scalar_v1972: 0.0,
            scalar_v1992: 0.0,
            scalar_v2012: 0.0,
            scalar_v2013: false,
            scalar_v2048: false,
            scalar_v2054: false,
            scalar_v2059: 0.0,
            scalar_v2060: false,
            scalar_v2064: false,
            scalar_v2120: 0.0,
            scalar_v2125: 0.0,
            scalar_v2128: 0.0,
            scalar_v2129: 0.0,
            scalar_v2130: 0.0,
            scalar_v2131: 0.0,
            scalar_v2816: 0.0,
            scalar_v2831: 0.0,
            scalar_v2832: 0.0,
            scalar_v2899: 0.0,
            scalar_v2911: 0.0,
            scalar_v3132: 0.0,
            scalar_v3133: 0.0,
            scalar_v3142: 0.0,
            scalar_v3143: 0.0,
            scalar_v3166: 0.0,
            scalar_v3167: 0.0,
            scalar_v3178: 0.0,
            scalar_v3179: 0.0,
            scalar_v3504: 0.0,
            scalar_v3543: 0.0,
            scalar_v3544: 0.0,
            scalar_v3587: 0.0,
            scalar_v3588: 0.0,
            scalar_v3675: 0.0,
            scalar_v3714: 0.0,
            scalar_v3715: 0.0,
            scalar_v3992: 0.0,
            scalar_v3993: 0.0,
            scalar_v4138: 0.0,
            scalar_v4139: 0.0,
            scalar_v4140: 0.0,
            scalar_v4141: 0.0,
            scalar_v4363: 0.0,
            scalar_v4364: 0.0,
            scalar_v4365: 0.0,
            scalar_v4366: 0.0,
            scalar_v4367: 0.0,
            scalar_v4368: 0.0,
            scalar_v4733: 0.0,
            scalar_v5195: 0.0,
            scalar_v5274: 0.0,
            scalar_v5510: 0.0,
            scalar_v5511: 0.0,
            scalar_v5512: 0.0,
            scalar_v5513: 0.0,
            scalar_v5670: 0.0,
            scalar_v5671: 0.0,
            scalar_v5700: 0.0,
            scalar_v5701: 0.0,
            scalar_v20: 0.0,
            scalar_v106: 0.0,
            scalar_v108: 0.0,
            scalar_v110: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v122: false,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v128: 0.0,
            scalar_v129: false,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: false,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: false,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: false,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: false,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: false,
            scalar_v195: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v201: false,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v204: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v210: 0.0,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: false,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v221: false,
            scalar_v222: 0.0,
            scalar_v223: 0.0,
            scalar_v224: 0.0,
            scalar_v225: 0.0,
            scalar_v226: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v237: false,
            scalar_v238: 0.0,
            scalar_v239: 0.0,
            scalar_v240: 0.0,
            scalar_v241: 0.0,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v244: false,
            scalar_v245: 0.0,
            scalar_v246: 0.0,
            scalar_v247: 0.0,
            scalar_v248: 0.0,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v256: 0.0,
            scalar_v257: 0.0,
            scalar_v258: 0.0,
            scalar_v259: 0.0,
            scalar_v260: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v268: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v276: 0.0,
            scalar_v277: 0.0,
            scalar_v278: 0.0,
            scalar_v279: false,
            scalar_v280: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: false,
            scalar_v294: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v306: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: false,
            scalar_v323: false,
            scalar_v324: 0.0,
            scalar_v325: 0.0,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: false,
            scalar_v331: false,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v343: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v351: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v354: false,
            scalar_v355: false,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v360: 0.0,
            scalar_v361: 0.0,
            scalar_v362: false,
            scalar_v363: false,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v366: 0.0,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v370: 0.0,
            scalar_v371: 0.0,
            scalar_v372: 0.0,
            scalar_v374: 0.0,
            scalar_v377: 0.0,
            scalar_v378: 0.0,
            scalar_v379: 0.0,
            scalar_v381: 0.0,
            scalar_v382: false,
            scalar_v385: 0.0,
            scalar_v386: 0.0,
            scalar_v387: 0.0,
            scalar_v388: 0.0,
            scalar_v389: 0.0,
            scalar_v390: false,
            scalar_v391: 0.0,
            scalar_v392: 0.0,
            scalar_v393: 0.0,
            scalar_v400: 0.0,
            scalar_v401: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v405: 0.0,
            scalar_v406: 0.0,
            scalar_v407: 0.0,
            scalar_v408: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v438: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v445: 0.0,
            scalar_v449: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v465: 0.0,
            scalar_v466: 0.0,
            scalar_v467: 0.0,
            scalar_v473: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v481: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v484: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v491: 0.0,
            scalar_v492: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v501: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v508: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v513: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v520: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v524: 0.0,
            scalar_v525: 0.0,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v530: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v535: 0.0,
            scalar_v536: 0.0,
            scalar_v537: 0.0,
            scalar_v538: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v544: 0.0,
            scalar_v545: 0.0,
            scalar_v546: 0.0,
            scalar_v547: 0.0,
            scalar_v548: 0.0,
            scalar_v549: 0.0,
            scalar_v550: 0.0,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v554: 0.0,
            scalar_v555: 0.0,
            scalar_v557: 0.0,
            scalar_v558: 0.0,
            scalar_v559: 0.0,
            scalar_v560: 0.0,
            scalar_v561: 0.0,
            scalar_v562: 0.0,
            scalar_v563: 0.0,
            scalar_v564: 0.0,
            scalar_v566: 0.0,
            scalar_v567: 0.0,
            scalar_v568: 0.0,
            scalar_v569: 0.0,
            scalar_v570: 0.0,
            scalar_v571: 0.0,
            scalar_v572: 0.0,
            scalar_v573: 0.0,
            scalar_v574: 0.0,
            scalar_v575: 0.0,
            scalar_v576: 0.0,
            scalar_v578: 0.0,
            scalar_v579: 0.0,
            scalar_v581: 0.0,
            scalar_v582: 0.0,
            scalar_v586: 0.0,
            scalar_v587: 0.0,
            scalar_v588: 0.0,
            scalar_v590: 0.0,
            scalar_v591: 0.0,
            scalar_v592: 0.0,
            scalar_v597: 0.0,
            scalar_v598: 0.0,
            scalar_v599: 0.0,
            scalar_v600: 0.0,
            scalar_v603: 0.0,
            scalar_v604: 0.0,
            scalar_v605: 0.0,
            scalar_v608: 0.0,
            scalar_v609: 0.0,
            scalar_v610: 0.0,
            scalar_v612: 0.0,
            scalar_v614: false,
            scalar_v616: 0.0,
            scalar_v617: 0.0,
            scalar_v619: 0.0,
            scalar_v620: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v624: false,
            scalar_v627: 0.0,
            scalar_v629: 0.0,
            scalar_v631: 0.0,
            scalar_v632: 0.0,
            scalar_v633: false,
            scalar_v634: false,
            scalar_v635: 0.0,
            scalar_v637: 0.0,
            scalar_v639: 0.0,
            scalar_v640: 0.0,
            scalar_v641: false,
            scalar_v642: false,
            scalar_v643: 0.0,
            scalar_v645: 0.0,
            scalar_v647: 0.0,
            scalar_v648: 0.0,
            scalar_v649: false,
            scalar_v650: false,
            scalar_v651: 0.0,
            scalar_v653: 0.0,
            scalar_v846: 0.0,
            scalar_v857: 0.0,
            scalar_v881: 0.0,
            scalar_v968: 0.0,
            scalar_v969: 0.0,
            scalar_v976: 0.0,
            scalar_v977: 0.0,
            scalar_v988: 0.0,
            scalar_v1011: 0.0,
            scalar_v1015: 0.0,
            scalar_v1042: 0.0,
            scalar_v1043: 0.0,
            scalar_v1065: 0.0,
            scalar_v1082: 0.0,
            scalar_v1083: 0.0,
            scalar_v1084: 0.0,
            scalar_v1086: 0.0,
            scalar_v1087: 0.0,
            scalar_v1088: 0.0,
            scalar_v1109: 0.0,
            scalar_v1123: 0.0,
            scalar_v1124: 0.0,
            scalar_v1130: 0.0,
            scalar_v1155: 0.0,
            scalar_v1156: 0.0,
            scalar_v1157: 0.0,
            scalar_v1178: 0.0,
            scalar_v1276: 0.0,
            scalar_v1335: 0.0,
            scalar_v1475: 0.0,
            scalar_v1564: 0.0,
            scalar_v1573: 0.0,
            scalar_v1576: 0.0,
            scalar_v1577: 0.0,
            scalar_v1587: 0.0,
            scalar_v1590: 0.0,
            scalar_v1591: 0.0,
            scalar_v1603: 0.0,
            scalar_v1630: 0.0,
            scalar_v1634: 0.0,
            scalar_v1635: 0.0,
            scalar_v1652: 0.0,
            scalar_v1664: 0.0,
            scalar_v1667: 0.0,
            scalar_v1668: 0.0,
            scalar_v1687: 0.0,
            scalar_v1688: 0.0,
            scalar_v1689: 0.0,
            scalar_v1690: 0.0,
            scalar_v1691: 0.0,
            scalar_v1692: 0.0,
            scalar_v1693: 0.0,
            scalar_v1694: 0.0,
            scalar_v1695: 0.0,
            scalar_v1827: 0.0,
            scalar_v1843: 0.0,
            scalar_v1932: 0.0,
            scalar_v1935: 0.0,
            scalar_v2132: 0.0,
            scalar_v2133: 0.0,
            scalar_v2142: 0.0,
            scalar_v2143: 0.0,
            scalar_v2152: 0.0,
            scalar_v2153: 0.0,
            scalar_v2178: 0.0,
            scalar_v2788: 0.0,
            scalar_v2789: 0.0,
            scalar_v2800: 0.0,
            scalar_v2801: 0.0,
            scalar_v2953: 0.0,
            scalar_v2954: 0.0,
            scalar_v2971: 0.0,
            scalar_v3204: 0.0,
            scalar_v3205: 0.0,
            scalar_v3337: 0.0,
            scalar_v3338: 0.0,
            scalar_v3394: 0.0,
            scalar_v3395: 0.0,
            scalar_v3409: 0.0,
            scalar_v3410: 0.0,
            scalar_v3424: 0.0,
            scalar_v3425: 0.0,
            scalar_v3426: 0.0,
            scalar_v3427: 0.0,
            scalar_v3451: 0.0,
            scalar_v3452: 0.0,
            scalar_v3493: 0.0,
            scalar_v3494: 0.0,
            scalar_v3545: 0.0,
            scalar_v3546: 0.0,
            scalar_v3635: 0.0,
            scalar_v3636: 0.0,
            scalar_v3637: 0.0,
            scalar_v3638: 0.0,
            scalar_v3716: 0.0,
            scalar_v3717: 0.0,
            scalar_v5317: 0.0,
            scalar_v5318: 0.0,
            scalar_v5672: 0.0,
            scalar_v5673: 0.0,
            scalar_v5674: 0.0,
            scalar_v5675: 0.0,
            scalar_v5676: 0.0,
            scalar_v5677: 0.0,
            scalar_v5678: 0.0,
            scalar_v5679: 0.0,
            scalar_v5702: 0.0,
            scalar_v5703: 0.0,
            scalar_v5704: 0.0,
            scalar_v5705: 0.0,
            scalar_v5706: 0.0,
            scalar_v5707: 0.0,
            scalar_v5708: 0.0,
            scalar_v5709: 0.0,
            scalar_v5737: 0.0,
            scalar_v5738: 0.0,
            scalar_v5739: 0.0,
            scalar_v5740: 0.0,
            scalar_v5741: 0.0,
            scalar_v5742: 0.0,
            scalar_v5743: 0.0,
            scalar_v5744: 0.0,
            scalar_v5745: 0.0,
            scalar_v5746: 0.0,
            scalar_v5747: 0.0,
            scalar_v5748: 0.0,
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
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
            scalar_v109,
            scalar_v111,
            scalar_v166,
            scalar_v186,
            scalar_v189,
            scalar_v229,
            scalar_v232,
            scalar_v252,
            scalar_v255,
            scalar_v266,
            scalar_v267,
            scalar_v274,
            scalar_v275,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v289,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v305,
            scalar_v307,
            scalar_v308,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v342,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v373,
            scalar_v375,
            scalar_v376,
            scalar_v394,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v404,
            scalar_v409,
            scalar_v410,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v420,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v428,
            scalar_v429,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v441,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v450,
            scalar_v454,
            scalar_v455,
            scalar_v460,
            scalar_v461,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v499,
            scalar_v500,
            scalar_v504,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v523,
            scalar_v526,
            scalar_v534,
            scalar_v543,
            scalar_v556,
            scalar_v565,
            scalar_v577,
            scalar_v580,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v589,
            scalar_v594,
            scalar_v595,
            scalar_v596,
            scalar_v601,
            scalar_v602,
            scalar_v606,
            scalar_v607,
            scalar_v626,
            scalar_v628,
            scalar_v630,
            scalar_v636,
            scalar_v638,
            scalar_v644,
            scalar_v646,
            scalar_v652,
            scalar_v700,
            scalar_v705,
            scalar_v823,
            scalar_v876,
            scalar_v877,
            scalar_v878,
            scalar_v889,
            scalar_v910,
            scalar_v911,
            scalar_v912,
            scalar_v913,
            scalar_v914,
            scalar_v915,
            scalar_v962,
            scalar_v972,
            scalar_v985,
            scalar_v986,
            scalar_v990,
            scalar_v1039,
            scalar_v1040,
            scalar_v1041,
            scalar_v1063,
            scalar_v1071,
            scalar_v1072,
            scalar_v1074,
            scalar_v1075,
            scalar_v1076,
            scalar_v1079,
            scalar_v1080,
            scalar_v1085,
            scalar_v1106,
            scalar_v1108,
            scalar_v1137,
            scalar_v1143,
            scalar_v1177,
            scalar_v1199,
            scalar_v1212,
            scalar_v1230,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1296,
            scalar_v1298,
            scalar_v1299,
            scalar_v1300,
            scalar_v1393,
            scalar_v1394,
            scalar_v1395,
            scalar_v1419,
            scalar_v1421,
            scalar_v1422,
            scalar_v1424,
            scalar_v1484,
            scalar_v1485,
            scalar_v1486,
            scalar_v1512,
            scalar_v1514,
            scalar_v1515,
            scalar_v1517,
            scalar_v1583,
            scalar_v1584,
            scalar_v1585,
            scalar_v1586,
            scalar_v1592,
            scalar_v1601,
            scalar_v1602,
            scalar_v1614,
            scalar_v1633,
            scalar_v1643,
            scalar_v1644,
            scalar_v1645,
            scalar_v1646,
            scalar_v1651,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1677,
            scalar_v1685,
            scalar_v1686,
            scalar_v1699,
            scalar_v1704,
            scalar_v1721,
            scalar_v1722,
            scalar_v1728,
            scalar_v1729,
            scalar_v1732,
            scalar_v1738,
            scalar_v1749,
            scalar_v1750,
            scalar_v1751,
            scalar_v1752,
            scalar_v1753,
            scalar_v1754,
            scalar_v1755,
            scalar_v1756,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1777,
            scalar_v1804,
            scalar_v1805,
            scalar_v1806,
            scalar_v1809,
            scalar_v1828,
            scalar_v1842,
            scalar_v1847,
            scalar_v1849,
            scalar_v1853,
            scalar_v1854,
            scalar_v1855,
            scalar_v1856,
            scalar_v1857,
            scalar_v1866,
            scalar_v1867,
            scalar_v1870,
            scalar_v1893,
            scalar_v1894,
            scalar_v1900,
            scalar_v1901,
            scalar_v1902,
            scalar_v1950,
            scalar_v1951,
            scalar_v1956,
            scalar_v1960,
            scalar_v1967,
            scalar_v1972,
            scalar_v1992,
            scalar_v2012,
            scalar_v2013,
            scalar_v2048,
            scalar_v2054,
            scalar_v2059,
            scalar_v2060,
            scalar_v2064,
            scalar_v2120,
            scalar_v2125,
            scalar_v2128,
            scalar_v2129,
            scalar_v2130,
            scalar_v2131,
            scalar_v2816,
            scalar_v2831,
            scalar_v2832,
            scalar_v2899,
            scalar_v2911,
            scalar_v3132,
            scalar_v3133,
            scalar_v3142,
            scalar_v3143,
            scalar_v3166,
            scalar_v3167,
            scalar_v3178,
            scalar_v3179,
            scalar_v3504,
            scalar_v3543,
            scalar_v3544,
            scalar_v3587,
            scalar_v3588,
            scalar_v3675,
            scalar_v3714,
            scalar_v3715,
            scalar_v3992,
            scalar_v3993,
            scalar_v4138,
            scalar_v4139,
            scalar_v4140,
            scalar_v4141,
            scalar_v4363,
            scalar_v4364,
            scalar_v4365,
            scalar_v4366,
            scalar_v4367,
            scalar_v4368,
            scalar_v4733,
            scalar_v5195,
            scalar_v5274,
            scalar_v5510,
            scalar_v5511,
            scalar_v5512,
            scalar_v5513,
            scalar_v5670,
            scalar_v5671,
            scalar_v5700,
            scalar_v5701,
            scalar_v20,
            scalar_v106,
            scalar_v108,
            scalar_v110,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
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
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v187,
            scalar_v188,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v230,
            scalar_v231,
            scalar_v233,
            scalar_v234,
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v253,
            scalar_v254,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v306,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v340,
            scalar_v341,
            scalar_v343,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v372,
            scalar_v374,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v381,
            scalar_v382,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v388,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v392,
            scalar_v393,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v408,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v449,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v524,
            scalar_v525,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v532,
            scalar_v533,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v566,
            scalar_v567,
            scalar_v568,
            scalar_v569,
            scalar_v570,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v574,
            scalar_v575,
            scalar_v576,
            scalar_v578,
            scalar_v579,
            scalar_v581,
            scalar_v582,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v590,
            scalar_v591,
            scalar_v592,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v600,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v608,
            scalar_v609,
            scalar_v610,
            scalar_v612,
            scalar_v614,
            scalar_v616,
            scalar_v617,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v627,
            scalar_v629,
            scalar_v631,
            scalar_v632,
            scalar_v633,
            scalar_v634,
            scalar_v635,
            scalar_v637,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v645,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v846,
            scalar_v857,
            scalar_v881,
            scalar_v968,
            scalar_v969,
            scalar_v976,
            scalar_v977,
            scalar_v988,
            scalar_v1011,
            scalar_v1015,
            scalar_v1042,
            scalar_v1043,
            scalar_v1065,
            scalar_v1082,
            scalar_v1083,
            scalar_v1084,
            scalar_v1086,
            scalar_v1087,
            scalar_v1088,
            scalar_v1109,
            scalar_v1123,
            scalar_v1124,
            scalar_v1130,
            scalar_v1155,
            scalar_v1156,
            scalar_v1157,
            scalar_v1178,
            scalar_v1276,
            scalar_v1335,
            scalar_v1475,
            scalar_v1564,
            scalar_v1573,
            scalar_v1576,
            scalar_v1577,
            scalar_v1587,
            scalar_v1590,
            scalar_v1591,
            scalar_v1603,
            scalar_v1630,
            scalar_v1634,
            scalar_v1635,
            scalar_v1652,
            scalar_v1664,
            scalar_v1667,
            scalar_v1668,
            scalar_v1687,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1827,
            scalar_v1843,
            scalar_v1932,
            scalar_v1935,
            scalar_v2132,
            scalar_v2133,
            scalar_v2142,
            scalar_v2143,
            scalar_v2152,
            scalar_v2153,
            scalar_v2178,
            scalar_v2788,
            scalar_v2789,
            scalar_v2800,
            scalar_v2801,
            scalar_v2953,
            scalar_v2954,
            scalar_v2971,
            scalar_v3204,
            scalar_v3205,
            scalar_v3337,
            scalar_v3338,
            scalar_v3394,
            scalar_v3395,
            scalar_v3409,
            scalar_v3410,
            scalar_v3424,
            scalar_v3425,
            scalar_v3426,
            scalar_v3427,
            scalar_v3451,
            scalar_v3452,
            scalar_v3493,
            scalar_v3494,
            scalar_v3545,
            scalar_v3546,
            scalar_v3635,
            scalar_v3636,
            scalar_v3637,
            scalar_v3638,
            scalar_v3716,
            scalar_v3717,
            scalar_v5317,
            scalar_v5318,
            scalar_v5672,
            scalar_v5673,
            scalar_v5674,
            scalar_v5675,
            scalar_v5676,
            scalar_v5677,
            scalar_v5678,
            scalar_v5679,
            scalar_v5702,
            scalar_v5703,
            scalar_v5704,
            scalar_v5705,
            scalar_v5706,
            scalar_v5707,
            scalar_v5708,
            scalar_v5709,
            scalar_v5737,
            scalar_v5738,
            scalar_v5739,
            scalar_v5740,
            scalar_v5741,
            scalar_v5742,
            scalar_v5743,
            scalar_v5744,
            scalar_v5745,
            scalar_v5746,
            scalar_v5747,
            scalar_v5748,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            scalar_v109,
            scalar_v111,
            scalar_v166,
            scalar_v186,
            scalar_v189,
            scalar_v229,
            scalar_v232,
            scalar_v252,
            scalar_v255,
            scalar_v266,
            scalar_v267,
            scalar_v274,
            scalar_v275,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v289,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v305,
            scalar_v307,
            scalar_v308,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v342,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v373,
            scalar_v375,
            scalar_v376,
            scalar_v394,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v404,
            scalar_v409,
            scalar_v410,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v420,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v428,
            scalar_v429,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v441,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v450,
            scalar_v454,
            scalar_v455,
            scalar_v460,
            scalar_v461,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v499,
            scalar_v500,
            scalar_v504,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v523,
            scalar_v526,
            scalar_v534,
            scalar_v543,
            scalar_v556,
            scalar_v565,
            scalar_v577,
            scalar_v580,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v589,
            scalar_v594,
            scalar_v595,
            scalar_v596,
            scalar_v601,
            scalar_v602,
            scalar_v606,
            scalar_v607,
            scalar_v626,
            scalar_v628,
            scalar_v630,
            scalar_v636,
            scalar_v638,
            scalar_v644,
            scalar_v646,
            scalar_v652,
            scalar_v700,
            scalar_v705,
            scalar_v823,
            scalar_v876,
            scalar_v877,
            scalar_v878,
            scalar_v889,
            scalar_v910,
            scalar_v911,
            scalar_v912,
            scalar_v913,
            scalar_v914,
            scalar_v915,
            scalar_v962,
            scalar_v972,
            scalar_v985,
            scalar_v986,
            scalar_v990,
            scalar_v1039,
            scalar_v1040,
            scalar_v1041,
            scalar_v1063,
            scalar_v1071,
            scalar_v1072,
            scalar_v1074,
            scalar_v1075,
            scalar_v1076,
            scalar_v1079,
            scalar_v1080,
            scalar_v1085,
            scalar_v1106,
            scalar_v1108,
            scalar_v1137,
            scalar_v1143,
            scalar_v1177,
            scalar_v1199,
            scalar_v1212,
            scalar_v1230,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1296,
            scalar_v1298,
            scalar_v1299,
            scalar_v1300,
            scalar_v1393,
            scalar_v1394,
            scalar_v1395,
            scalar_v1419,
            scalar_v1421,
            scalar_v1422,
            scalar_v1424,
            scalar_v1484,
            scalar_v1485,
            scalar_v1486,
            scalar_v1512,
            scalar_v1514,
            scalar_v1515,
            scalar_v1517,
            scalar_v1583,
            scalar_v1584,
            scalar_v1585,
            scalar_v1586,
            scalar_v1592,
            scalar_v1601,
            scalar_v1602,
            scalar_v1614,
            scalar_v1633,
            scalar_v1643,
            scalar_v1644,
            scalar_v1645,
            scalar_v1646,
            scalar_v1651,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1677,
            scalar_v1685,
            scalar_v1686,
            scalar_v1699,
            scalar_v1704,
            scalar_v1721,
            scalar_v1722,
            scalar_v1728,
            scalar_v1729,
            scalar_v1732,
            scalar_v1738,
            scalar_v1749,
            scalar_v1750,
            scalar_v1751,
            scalar_v1752,
            scalar_v1753,
            scalar_v1754,
            scalar_v1755,
            scalar_v1756,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1777,
            scalar_v1804,
            scalar_v1805,
            scalar_v1806,
            scalar_v1809,
            scalar_v1828,
            scalar_v1842,
            scalar_v1847,
            scalar_v1849,
            scalar_v1853,
            scalar_v1854,
            scalar_v1855,
            scalar_v1856,
            scalar_v1857,
            scalar_v1866,
            scalar_v1867,
            scalar_v1870,
            scalar_v1893,
            scalar_v1894,
            scalar_v1900,
            scalar_v1901,
            scalar_v1902,
            scalar_v1950,
            scalar_v1951,
            scalar_v1956,
            scalar_v1960,
            scalar_v1967,
            scalar_v1972,
            scalar_v1992,
            scalar_v2012,
            scalar_v2013,
            scalar_v2048,
            scalar_v2054,
            scalar_v2059,
            scalar_v2060,
            scalar_v2064,
            scalar_v2120,
            scalar_v2125,
            scalar_v2128,
            scalar_v2129,
            scalar_v2130,
            scalar_v2131,
            scalar_v2816,
            scalar_v2831,
            scalar_v2832,
            scalar_v2899,
            scalar_v2911,
            scalar_v3132,
            scalar_v3133,
            scalar_v3142,
            scalar_v3143,
            scalar_v3166,
            scalar_v3167,
            scalar_v3178,
            scalar_v3179,
            scalar_v3504,
            scalar_v3543,
            scalar_v3544,
            scalar_v3587,
            scalar_v3588,
            scalar_v3675,
            scalar_v3714,
            scalar_v3715,
            scalar_v3992,
            scalar_v3993,
            scalar_v4138,
            scalar_v4139,
            scalar_v4140,
            scalar_v4141,
            scalar_v4363,
            scalar_v4364,
            scalar_v4365,
            scalar_v4366,
            scalar_v4367,
            scalar_v4368,
            scalar_v4733,
            scalar_v5195,
            scalar_v5274,
            scalar_v5510,
            scalar_v5511,
            scalar_v5512,
            scalar_v5513,
            scalar_v5670,
            scalar_v5671,
            scalar_v5700,
            scalar_v5701,
            scalar_v20,
            scalar_v106,
            scalar_v108,
            scalar_v110,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
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
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v187,
            scalar_v188,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v230,
            scalar_v231,
            scalar_v233,
            scalar_v234,
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v253,
            scalar_v254,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v306,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v340,
            scalar_v341,
            scalar_v343,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v372,
            scalar_v374,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v381,
            scalar_v382,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v388,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v392,
            scalar_v393,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v408,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v449,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v524,
            scalar_v525,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v532,
            scalar_v533,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v566,
            scalar_v567,
            scalar_v568,
            scalar_v569,
            scalar_v570,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v574,
            scalar_v575,
            scalar_v576,
            scalar_v578,
            scalar_v579,
            scalar_v581,
            scalar_v582,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v590,
            scalar_v591,
            scalar_v592,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v600,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v608,
            scalar_v609,
            scalar_v610,
            scalar_v612,
            scalar_v614,
            scalar_v616,
            scalar_v617,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v627,
            scalar_v629,
            scalar_v631,
            scalar_v632,
            scalar_v633,
            scalar_v634,
            scalar_v635,
            scalar_v637,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v645,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v846,
            scalar_v857,
            scalar_v881,
            scalar_v968,
            scalar_v969,
            scalar_v976,
            scalar_v977,
            scalar_v988,
            scalar_v1011,
            scalar_v1015,
            scalar_v1042,
            scalar_v1043,
            scalar_v1065,
            scalar_v1082,
            scalar_v1083,
            scalar_v1084,
            scalar_v1086,
            scalar_v1087,
            scalar_v1088,
            scalar_v1109,
            scalar_v1123,
            scalar_v1124,
            scalar_v1130,
            scalar_v1155,
            scalar_v1156,
            scalar_v1157,
            scalar_v1178,
            scalar_v1276,
            scalar_v1335,
            scalar_v1475,
            scalar_v1564,
            scalar_v1573,
            scalar_v1576,
            scalar_v1577,
            scalar_v1587,
            scalar_v1590,
            scalar_v1591,
            scalar_v1603,
            scalar_v1630,
            scalar_v1634,
            scalar_v1635,
            scalar_v1652,
            scalar_v1664,
            scalar_v1667,
            scalar_v1668,
            scalar_v1687,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1827,
            scalar_v1843,
            scalar_v1932,
            scalar_v1935,
            scalar_v2132,
            scalar_v2133,
            scalar_v2142,
            scalar_v2143,
            scalar_v2152,
            scalar_v2153,
            scalar_v2178,
            scalar_v2788,
            scalar_v2789,
            scalar_v2800,
            scalar_v2801,
            scalar_v2953,
            scalar_v2954,
            scalar_v2971,
            scalar_v3204,
            scalar_v3205,
            scalar_v3337,
            scalar_v3338,
            scalar_v3394,
            scalar_v3395,
            scalar_v3409,
            scalar_v3410,
            scalar_v3424,
            scalar_v3425,
            scalar_v3426,
            scalar_v3427,
            scalar_v3451,
            scalar_v3452,
            scalar_v3493,
            scalar_v3494,
            scalar_v3545,
            scalar_v3546,
            scalar_v3635,
            scalar_v3636,
            scalar_v3637,
            scalar_v3638,
            scalar_v3716,
            scalar_v3717,
            scalar_v5317,
            scalar_v5318,
            scalar_v5672,
            scalar_v5673,
            scalar_v5674,
            scalar_v5675,
            scalar_v5676,
            scalar_v5677,
            scalar_v5678,
            scalar_v5679,
            scalar_v5702,
            scalar_v5703,
            scalar_v5704,
            scalar_v5705,
            scalar_v5706,
            scalar_v5707,
            scalar_v5708,
            scalar_v5709,
            scalar_v5737,
            scalar_v5738,
            scalar_v5739,
            scalar_v5740,
            scalar_v5741,
            scalar_v5742,
            scalar_v5743,
            scalar_v5744,
            scalar_v5745,
            scalar_v5746,
            scalar_v5747,
            scalar_v5748,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult" => { validate_parameter("mult", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((505.5, "505.5")), false, Some((505.51, "505.51")), true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("tref", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exmod" => { validate_parameter("exmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exphi" => { validate_parameter("exphi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exavl" => { validate_parameter("exavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exsub" => { validate_parameter("exsub", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iss" => { validate_parameter("iss", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "icss" => { validate_parameter("icss", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iks" => { validate_parameter("iks", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikcs" => { validate_parameter("ikcs", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.05, "0.05")), true, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("ps", value, Some((0.01, "0.01")), true, Some((0.99, "0.99")), true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "as" => { validate_finite_parameter("as", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "asub" => { validate_finite_parameter("asub", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xisubi" => { validate_parameter("xisubi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvsch" => { validate_parameter("swvsch", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
        let v22: bool = (0.0 == p.p150);
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
        let v43: f64 = (v17 * p.p115);
        self.scalar_v43 = v43;
        let v44: f64 = (v17 * v43);
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
        let v54: f64 = ((v52) as f64).exp();
        self.scalar_v54 = v54;
        let v55: f64 = (1.0 + v54);
        self.scalar_v55 = v55;
        let v56: f64 = ((v55) as f64).ln();
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
        let v62: f64 = ((v61) as f64).exp();
        self.scalar_v62 = v62;
        let v63: f64 = (1.0 + v62);
        self.scalar_v63 = v63;
        let v64: f64 = ((v63) as f64).ln();
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
        let v78: f64 = (v17 * p.p118);
        self.scalar_v78 = v78;
        let v79: f64 = (v17 * v78);
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
        let v87: f64 = ((v85) as f64).exp();
        self.scalar_v87 = v87;
        let v88: f64 = (1.0 + v87);
        self.scalar_v88 = v88;
        let v89: f64 = ((v88) as f64).ln();
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
        let v95: f64 = ((v94) as f64).exp();
        self.scalar_v95 = v95;
        let v96: f64 = (1.0 + v95);
        self.scalar_v96 = v96;
        let v97: f64 = ((v96) as f64).ln();
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
        let v109: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v109 = v109;
        let v111: f64 = (1.0 / v109);
        self.scalar_v111 = v111;
        let v166: f64 = p.p105;
        self.scalar_v166 = v166;
        let v186: f64 = p.p64;
        self.scalar_v186 = v186;
        let v189: f64 = p.p110;
        self.scalar_v189 = v189;
        let v229: f64 = p.p27;
        self.scalar_v229 = v229;
        let v232: f64 = p.p109;
        self.scalar_v232 = v232;
        let v252: f64 = p.p138;
        self.scalar_v252 = v252;
        let v255: f64 = p.p140;
        self.scalar_v255 = v255;
        let v266: f64 = p.p75;
        self.scalar_v266 = v266;
        let v267: f64 = (1.0 - p.p75);
        self.scalar_v267 = v267;
        let v274: f64 = p.p54;
        self.scalar_v274 = v274;
        let v275: f64 = p.p97;
        self.scalar_v275 = v275;
        let v281: f64 = p.p56;
        self.scalar_v281 = v281;
        let v282: f64 = p.p98;
        self.scalar_v282 = v282;
        let v283: f64 = p.p96;
        self.scalar_v283 = v283;
        let v284: f64 = (p.p98 - p.p96);
        self.scalar_v284 = v284;
        let v288: f64 = p.p55;
        self.scalar_v288 = v288;
        let v289: f64 = p.p101;
        self.scalar_v289 = v289;
        let v295: f64 = p.p57;
        self.scalar_v295 = v295;
        let v296: f64 = p.p102;
        self.scalar_v296 = v296;
        let v300: f64 = p.p58;
        self.scalar_v300 = v300;
        let v301: f64 = p.p104;
        self.scalar_v301 = v301;
        let v305: f64 = p.p59;
        self.scalar_v305 = v305;
        let v307: f64 = p.p60;
        self.scalar_v307 = v307;
        let v308: f64 = p.p99;
        self.scalar_v308 = v308;
        let v312: f64 = p.p122;
        self.scalar_v312 = v312;
        let v313: bool = (0.0 != p.p122);
        self.scalar_v313 = v313;
        let v314: f64 = p.p10;
        self.scalar_v314 = v314;
        let v342: bool = (!v313);
        self.scalar_v342 = v342;
        let v344: f64 = p.p123;
        self.scalar_v344 = v344;
        let v345: bool = (0.0 != p.p123);
        self.scalar_v345 = v345;
        let v346: f64 = p.p11;
        self.scalar_v346 = v346;
        let v373: bool = (!v345);
        self.scalar_v373 = v373;
        let v375: f64 = p.p43;
        self.scalar_v375 = v375;
        let v376: f64 = p.p124;
        self.scalar_v376 = v376;
        let v394: f64 = p.p9;
        self.scalar_v394 = v394;
        let v396: f64 = (4.0 - p.p98);
        self.scalar_v396 = v396;
        let v397: f64 = (v396 - p.p96);
        self.scalar_v397 = v397;
        let v398: f64 = p.p121;
        self.scalar_v398 = v398;
        let v399: f64 = (v397 + p.p121);
        self.scalar_v399 = v399;
        let v404: f64 = (-p.p105);
        self.scalar_v404 = v404;
        let v409: f64 = p.p12;
        self.scalar_v409 = v409;
        let v410: f64 = (1.0 - p.p98);
        self.scalar_v410 = v410;
        let v414: f64 = p.p30;
        self.scalar_v414 = v414;
        let v415: f64 = p.p103;
        self.scalar_v415 = v415;
        let v416: f64 = (1.0 - p.p103);
        self.scalar_v416 = v416;
        let v420: f64 = p.p20;
        self.scalar_v420 = v420;
        let v422: f64 = p.p21;
        self.scalar_v422 = v422;
        let v423: f64 = (2.0 * p.p21);
        self.scalar_v423 = v423;
        let v424: f64 = (6.0 - v423);
        self.scalar_v424 = v424;
        let v428: f64 = p.p113;
        self.scalar_v428 = v428;
        let v429: f64 = (-p.p113);
        self.scalar_v429 = v429;
        let v434: f64 = p.p31;
        self.scalar_v434 = v434;
        let v435: f64 = p.p32;
        self.scalar_v435 = v435;
        let v436: f64 = (2.0 * p.p32);
        self.scalar_v436 = v436;
        let v437: f64 = (6.0 - v436);
        self.scalar_v437 = v437;
        let v441: f64 = (-p.p110);
        self.scalar_v441 = v441;
        let v446: f64 = p.p16;
        self.scalar_v446 = v446;
        let v447: f64 = (4.0 - p.p97);
        self.scalar_v447 = v447;
        let v448: f64 = (p.p121 + v447);
        self.scalar_v448 = v448;
        let v450: f64 = p.p17;
        self.scalar_v450 = v450;
        let v454: f64 = p.p111;
        self.scalar_v454 = v454;
        let v455: f64 = (-p.p111);
        self.scalar_v455 = v455;
        let v460: f64 = p.p18;
        self.scalar_v460 = v460;
        let v461: f64 = p.p19;
        self.scalar_v461 = v461;
        let v468: f64 = p.p24;
        self.scalar_v468 = v468;
        let v469: bool = (1.0 == p.p24);
        self.scalar_v469 = v469;
        let v470: f64 = p.p25;
        self.scalar_v470 = v470;
        let v471: f64 = p.p107;
        self.scalar_v471 = v471;
        let v472: f64 = (-p.p107);
        self.scalar_v472 = v472;
        let v478: f64 = p.p28;
        self.scalar_v478 = v478;
        let v479: f64 = p.p106;
        self.scalar_v479 = v479;
        let v480: f64 = (-p.p106);
        self.scalar_v480 = v480;
        let v485: f64 = p.p26;
        self.scalar_v485 = v485;
        let v486: f64 = p.p108;
        self.scalar_v486 = v486;
        let v487: f64 = (-p.p108);
        self.scalar_v487 = v487;
        let v493: f64 = p.p29;
        self.scalar_v493 = v493;
        let v494: f64 = (4.0 - p.p103);
        self.scalar_v494 = v494;
        let v495: f64 = (p.p121 + v494);
        self.scalar_v495 = v495;
        let v499: f64 = p.p112;
        self.scalar_v499 = v499;
        let v500: f64 = (-p.p112);
        self.scalar_v500 = v500;
        let v504: f64 = p.p22;
        self.scalar_v504 = v504;
        let v505: f64 = p.p23;
        self.scalar_v505 = v505;
        let v506: f64 = (2.0 * p.p23);
        self.scalar_v506 = v506;
        let v507: f64 = (6.0 - v506);
        self.scalar_v507 = v507;
        let v514: f64 = p.p145;
        self.scalar_v514 = v514;
        let v515: f64 = p.p146;
        self.scalar_v515 = v515;
        let v516: f64 = (4.0 / p.p146);
        self.scalar_v516 = v516;
        let v523: f64 = p.p151;
        self.scalar_v523 = v523;
        let v526: f64 = p.p153;
        self.scalar_v526 = v526;
        let v534: f64 = p.p35;
        self.scalar_v534 = v534;
        let v543: f64 = p.p34;
        self.scalar_v543 = v543;
        let v556: f64 = p.p37;
        self.scalar_v556 = v556;
        let v565: f64 = p.p36;
        self.scalar_v565 = v565;
        let v577: f64 = p.p14;
        self.scalar_v577 = v577;
        let v580: f64 = p.p13;
        self.scalar_v580 = v580;
        let v583: f64 = p.p133;
        self.scalar_v583 = v583;
        let v584: f64 = p.p141;
        self.scalar_v584 = v584;
        let v585: f64 = (4.0 - p.p141);
        self.scalar_v585 = v585;
        let v589: f64 = (-p.p140);
        self.scalar_v589 = v589;
        let v594: f64 = p.p142;
        self.scalar_v594 = v594;
        let v595: f64 = (0.5 * p.p142);
        self.scalar_v595 = v595;
        let v596: f64 = (3.5 - v595);
        self.scalar_v596 = v596;
        let v601: f64 = p.p135;
        self.scalar_v601 = v601;
        let v602: f64 = (1.0 - p.p141);
        self.scalar_v602 = v602;
        let v606: f64 = p.p136;
        self.scalar_v606 = v606;
        let v607: f64 = (1.0 - p.p142);
        self.scalar_v607 = v607;
        let v626: f64 = (v12 * 1.081);
        self.scalar_v626 = v626;
        let v628: f64 = p.p92;
        self.scalar_v628 = v628;
        let v630: bool = (p.p57 > 0.0);
        self.scalar_v630 = v630;
        let v636: bool = (!v630);
        self.scalar_v636 = v636;
        let v638: bool = (p.p58 > 0.0);
        self.scalar_v638 = v638;
        let v644: bool = (!v638);
        self.scalar_v644 = v644;
        let v646: bool = (p.p59 > 0.0);
        self.scalar_v646 = v646;
        let v652: bool = (!v646);
        self.scalar_v652 = v652;
        let v700: f64 = p.p147;
        self.scalar_v700 = v700;
        let v705: f64 = ((p.p147) as f64).exp();
        self.scalar_v705 = v705;
        let v823: f64 = p.p149;
        self.scalar_v823 = v823;
        let v876: f64 = p.p62;
        self.scalar_v876 = v876;
        let v877: f64 = p.p61;
        self.scalar_v877 = v877;
        let v878: f64 = (p.p62 * p.p61);
        self.scalar_v878 = v878;
        let v889: f64 = p.p63;
        self.scalar_v889 = v889;
        let v910: f64 = (-1.0 / p.p63);
        self.scalar_v910 = v910;
        let v911: f64 = ((v910) as f64).exp();
        self.scalar_v911 = v911;
        let v912: f64 = (1.0 + v911);
        self.scalar_v912 = v912;
        let v913: f64 = ((v912) as f64).ln();
        self.scalar_v913 = v913;
        let v914: f64 = (p.p63 * v913);
        self.scalar_v914 = v914;
        let v915: f64 = (1.0 + v914);
        self.scalar_v915 = v915;
        let v962: f64 = p.p148;
        self.scalar_v962 = v962;
        let v972: f64 = (0.5 * p.p61);
        self.scalar_v972 = v972;
        let v985: f64 = p.p73;
        self.scalar_v985 = v985;
        let v986: bool = (0.0 == p.p73);
        self.scalar_v986 = v986;
        let v990: bool = (!v986);
        self.scalar_v990 = v990;
        let v1039: f64 = (-1.0 / p.p67);
        self.scalar_v1039 = v1039;
        let v1040: f64 = f64::powf(3.0, v1039);
        self.scalar_v1040 = v1040;
        let v1041: f64 = (1.0 - v1040);
        self.scalar_v1041 = v1041;
        let v1063: f64 = (1.0 - p.p67);
        self.scalar_v1063 = v1063;
        let v1071: f64 = p.p74;
        self.scalar_v1071 = v1071;
        let v1072: bool = (1.0 == p.p74);
        self.scalar_v1072 = v1072;
        let v1074: bool = (2.0 == p.p74);
        self.scalar_v1074 = v1074;
        let v1075: bool = (!v1072);
        self.scalar_v1075 = v1075;
        let v1076: bool = (v1074 && v1075);
        self.scalar_v1076 = v1076;
        let v1079: bool = (!v1074);
        self.scalar_v1079 = v1079;
        let v1080: bool = (v1075 && v1079);
        self.scalar_v1080 = v1080;
        let v1085: f64 = (-1.0 / p.p72);
        self.scalar_v1085 = v1085;
        let v1106: f64 = p.p76;
        self.scalar_v1106 = v1106;
        let v1108: f64 = (1.0 - p.p72);
        self.scalar_v1108 = v1108;
        let v1137: bool = (0.0 == p.p92);
        self.scalar_v1137 = v1137;
        let v1143: bool = (!v1137);
        self.scalar_v1143 = v1143;
        let v1177: f64 = p.p15;
        self.scalar_v1177 = v1177;
        let v1199: f64 = p.p152;
        self.scalar_v1199 = v1199;
        let v1212: f64 = p.p154;
        self.scalar_v1212 = v1212;
        let v1230: f64 = p.p155;
        self.scalar_v1230 = v1230;
        let v1293: f64 = p.p93;
        self.scalar_v1293 = v1293;
        let v1294: bool = (0.0 == p.p93);
        self.scalar_v1294 = v1294;
        let v1295: bool = (!v469);
        self.scalar_v1295 = v1295;
        let v1296: bool = (v1294 && v1295);
        self.scalar_v1296 = v1296;
        let v1298: bool = (!v1294);
        self.scalar_v1298 = v1298;
        let v1299: bool = (v1295 && v1298);
        self.scalar_v1299 = v1299;
        let v1300: f64 = (1.0 - p.p93);
        self.scalar_v1300 = v1300;
        let v1393: bool = (p.p34 > 0.0);
        self.scalar_v1393 = v1393;
        let v1394: bool = (p.p35 > 0.0);
        self.scalar_v1394 = v1394;
        let v1395: bool = (v1393 && v1394);
        self.scalar_v1395 = v1395;
        let v1419: f64 = (-2.0 - p.p67);
        self.scalar_v1419 = v1419;
        let v1421: f64 = (p.p67 * p.p67);
        self.scalar_v1421 = v1421;
        let v1422: f64 = (1.0 - v1421);
        self.scalar_v1422 = v1422;
        let v1424: f64 = (p.p67 - 1.0);
        self.scalar_v1424 = v1424;
        let v1484: bool = (p.p36 > 0.0);
        self.scalar_v1484 = v1484;
        let v1485: bool = (p.p37 > 0.0);
        self.scalar_v1485 = v1485;
        let v1486: bool = (v1484 && v1485);
        self.scalar_v1486 = v1486;
        let v1512: f64 = (-2.0 - p.p72);
        self.scalar_v1512 = v1512;
        let v1514: f64 = (p.p72 * p.p72);
        self.scalar_v1514 = v1514;
        let v1515: f64 = (1.0 - v1514);
        self.scalar_v1515 = v1515;
        let v1517: f64 = (p.p72 - 1.0);
        self.scalar_v1517 = v1517;
        let v1583: f64 = p.p8;
        self.scalar_v1583 = v1583;
        let v1584: bool = (1.0 == p.p8);
        self.scalar_v1584 = v1584;
        let v1585: f64 = p.p143;
        self.scalar_v1585 = v1585;
        let v1586: f64 = (2.0 * p.p143);
        self.scalar_v1586 = v1586;
        let v1592: f64 = p.p144;
        self.scalar_v1592 = v1592;
        let v1601: f64 = (1.0 - p.p143);
        self.scalar_v1601 = v1601;
        let v1602: f64 = (2.0 * v1601);
        self.scalar_v1602 = v1602;
        let v1614: bool = (!v1584);
        self.scalar_v1614 = v1614;
        let v1633: f64 = (4.0 * p.p144);
        self.scalar_v1633 = v1633;
        let v1643: f64 = p.p5;
        self.scalar_v1643 = v1643;
        let v1644: bool = (p.p5 > 0.0);
        self.scalar_v1644 = v1644;
        let v1645: bool = (p.p33 > 0.0);
        self.scalar_v1645 = v1645;
        let v1646: bool = (v1644 && v1645);
        self.scalar_v1646 = v1646;
        let v1651: f64 = (p.p33 * 2.0);
        self.scalar_v1651 = v1651;
        let v1661: bool = (v1584 && v1646);
        self.scalar_v1661 = v1661;
        let v1662: f64 = (p.p33 * v1601);
        self.scalar_v1662 = v1662;
        let v1663: f64 = (2.0 * v1662);
        self.scalar_v1663 = v1663;
        let v1677: bool = (v1614 && v1646);
        self.scalar_v1677 = v1677;
        let v1685: bool = (1.0 == p.p5);
        self.scalar_v1685 = v1685;
        let v1686: bool = (v1646 && v1685);
        self.scalar_v1686 = v1686;
        let v1699: f64 = (if v1686 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1699 = v1699;
        let v1704: f64 = (0.5 * v1699);
        self.scalar_v1704 = v1704;
        let v1721: bool = (!v1685);
        self.scalar_v1721 = v1721;
        let v1722: bool = (v1646 && v1721);
        self.scalar_v1722 = v1722;
        let v1728: f64 = p.p84;
        self.scalar_v1728 = v1728;
        let v1729: bool = (1.0 == p.p84);
        self.scalar_v1729 = v1729;
        let v1732: f64 = (if v1729 { 1e-12 } else { v1699 });
        self.scalar_v1732 = v1732;
        let v1738: f64 = (0.5 * v1732);
        self.scalar_v1738 = v1738;
        let v1749: f64 = p.p82;
        self.scalar_v1749 = v1749;
        let v1750: f64 = f64::powf(v105, p.p82);
        self.scalar_v1750 = v1750;
        let v1751: f64 = (1.0 - v1750);
        self.scalar_v1751 = v1751;
        let v1752: f64 = (1.0 / v1751);
        self.scalar_v1752 = v1752;
        let v1753: f64 = (if v1729 { v1752 } else { 0.0 });
        self.scalar_v1753 = v1753;
        let v1754: f64 = p.p81;
        self.scalar_v1754 = v1754;
        let v1755: f64 = (v105 * p.p81);
        self.scalar_v1755 = v1755;
        let v1756: f64 = (if v1729 { v1755 } else { 0.0 });
        self.scalar_v1756 = v1756;
        let v1757: f64 = (v1753 * v1753);
        self.scalar_v1757 = v1757;
        let v1758: f64 = (p.p82 - 1.0);
        self.scalar_v1758 = v1758;
        let v1759: f64 = f64::powf(v105, v1758);
        self.scalar_v1759 = v1759;
        let v1760: f64 = (v1757 * v1759);
        self.scalar_v1760 = v1760;
        let v1761: f64 = (p.p82 * v1760);
        self.scalar_v1761 = v1761;
        let v1762: f64 = (v1761 / p.p81);
        self.scalar_v1762 = v1762;
        let v1763: f64 = (if v1729 { v1762 } else { 0.0 });
        self.scalar_v1763 = v1763;
        let v1777: bool = (!v1729);
        self.scalar_v1777 = v1777;
        let v1804: f64 = p.p39;
        self.scalar_v1804 = v1804;
        let v1805: bool = (1.0 == p.p39);
        self.scalar_v1805 = v1805;
        let v1806: f64 = p.p44;
        self.scalar_v1806 = v1806;
        let v1809: f64 = p.p42;
        self.scalar_v1809 = v1809;
        let v1828: f64 = p.p41;
        self.scalar_v1828 = v1828;
        let v1842: f64 = p.p40;
        self.scalar_v1842 = v1842;
        let v1847: bool = (2.0 == p.p39);
        self.scalar_v1847 = v1847;
        let v1849: bool = (!v1805);
        self.scalar_v1849 = v1849;
        let v1853: f64 = p.p46;
        self.scalar_v1853 = v1853;
        let v1854: f64 = (2.0 * p.p46);
        self.scalar_v1854 = v1854;
        let v1855: f64 = p.p45;
        self.scalar_v1855 = v1855;
        let v1856: f64 = (p.p45 * p.p45);
        self.scalar_v1856 = v1856;
        let v1857: f64 = (v1854 / v1856);
        self.scalar_v1857 = v1857;
        let v1866: f64 = p.p7;
        self.scalar_v1866 = v1866;
        let v1867: bool = (0.0 == p.p7);
        self.scalar_v1867 = v1867;
        let v1870: bool = (!v1867);
        self.scalar_v1870 = v1870;
        let v1893: f64 = p.p47;
        self.scalar_v1893 = v1893;
        let v1894: f64 = (2.0 * p.p47);
        self.scalar_v1894 = v1894;
        let v1900: f64 = (1.0 + p.p47);
        self.scalar_v1900 = v1900;
        let v1901: f64 = (1.0 + v1894);
        self.scalar_v1901 = v1901;
        let v1902: f64 = (v1900 / v1901);
        self.scalar_v1902 = v1902;
        let v1950: bool = (3.0 == p.p39);
        self.scalar_v1950 = v1950;
        let v1951: bool = (!v1847);
        self.scalar_v1951 = v1951;
        let v1956: f64 = p.p48;
        self.scalar_v1956 = v1956;
        let v1960: f64 = p.p49;
        self.scalar_v1960 = v1960;
        let v1967: f64 = p.p52;
        self.scalar_v1967 = v1967;
        let v1972: f64 = p.p51;
        self.scalar_v1972 = v1972;
        let v1992: f64 = p.p50;
        self.scalar_v1992 = v1992;
        let v2012: f64 = p.p53;
        self.scalar_v2012 = v2012;
        let v2013: bool = (1.0 == p.p53);
        self.scalar_v2013 = v2013;
        let v2048: bool = (!v1950);
        self.scalar_v2048 = v2048;
        let v2054: bool = (!v2013);
        self.scalar_v2054 = v2054;
        let v2059: f64 = p.p130;
        self.scalar_v2059 = v2059;
        let v2060: bool = (p.p130 > 0.0);
        self.scalar_v2060 = v2060;
        let v2064: bool = (!v2060);
        self.scalar_v2064 = v2064;
        let v2120: f64 = (if v644 { 0.0 } else { 0.0 });
        self.scalar_v2120 = v2120;
        let v2125: f64 = (if v652 { 0.0 } else { 0.0 });
        self.scalar_v2125 = v2125;
        let v2128: f64 = (-p.p3);
        self.scalar_v2128 = v2128;
        let v2129: f64 = (p.p3 + v2128);
        self.scalar_v2129 = v2129;
        let v2130: f64 = (v2128 - v2128);
        self.scalar_v2130 = v2130;
        let v2131: f64 = (p.p3 + v2129);
        self.scalar_v2131 = v2131;
        let v2816: f64 = (v1063 - 1.0);
        self.scalar_v2816 = v2816;
        let v2831: f64 = (if v1072 { p.p3 } else { 0.0 });
        self.scalar_v2831 = v2831;
        let v2832: f64 = (if v1072 { v2128 } else { 0.0 });
        self.scalar_v2832 = v2832;
        let v2899: f64 = (p.p76 - 1.0);
        self.scalar_v2899 = v2899;
        let v2911: f64 = (v1108 - 1.0);
        self.scalar_v2911 = v2911;
        let v3132: f64 = (v2128 / 0.0001);
        self.scalar_v3132 = v3132;
        let v3133: f64 = (p.p3 / 0.0001);
        self.scalar_v3133 = v3133;
        let v3142: f64 = (-v3132);
        self.scalar_v3142 = v3142;
        let v3143: f64 = (-v3133);
        self.scalar_v3143 = v3143;
        let v3166: f64 = (v2128 / 0.001);
        self.scalar_v3166 = v3166;
        let v3167: f64 = (p.p3 / 0.001);
        self.scalar_v3167 = v3167;
        let v3178: f64 = (-v3166);
        self.scalar_v3178 = v3178;
        let v3179: f64 = (-v3167);
        self.scalar_v3179 = v3179;
        let v3504: f64 = (v1419 - 1.0);
        self.scalar_v3504 = v3504;
        let v3543: f64 = (v39 * v2128);
        self.scalar_v3543 = v3543;
        let v3544: f64 = (p.p3 * v39);
        self.scalar_v3544 = v3544;
        let v3587: f64 = (0.5 * v2128);
        self.scalar_v3587 = v3587;
        let v3588: f64 = (p.p3 * 0.5);
        self.scalar_v3588 = v3588;
        let v3675: f64 = (v1512 - 1.0);
        self.scalar_v3675 = v3675;
        let v3714: f64 = (p.p3 * v74);
        self.scalar_v3714 = v3714;
        let v3715: f64 = (v74 * v2128);
        self.scalar_v3715 = v3715;
        let v3992: f64 = (p.p3 * v34);
        self.scalar_v3992 = v3992;
        let v3993: f64 = (v34 * v2128);
        self.scalar_v3993 = v3993;
        let v4138: f64 = (if v1686 { v2129 } else { 0.0 });
        self.scalar_v4138 = v4138;
        let v4139: f64 = (if v1686 { v2131 } else { 0.0 });
        self.scalar_v4139 = v4139;
        let v4140: f64 = (if v1686 { v2130 } else { 0.0 });
        self.scalar_v4140 = v4140;
        let v4141: f64 = (if v1686 { v2128 } else { 0.0 });
        self.scalar_v4141 = v4141;
        let v4363: f64 = (if v1729 { p.p3 } else { 0.0 });
        self.scalar_v4363 = v4363;
        let v4364: f64 = (if v1729 { v2129 } else { 0.0 });
        self.scalar_v4364 = v4364;
        let v4365: f64 = (if v1729 { v2128 } else { 0.0 });
        self.scalar_v4365 = v4365;
        let v4366: f64 = (-v4363);
        self.scalar_v4366 = v4366;
        let v4367: f64 = (-v4364);
        self.scalar_v4367 = v4367;
        let v4368: f64 = (-v4365);
        self.scalar_v4368 = v4368;
        let v4733: f64 = (p.p41 - 1.0);
        self.scalar_v4733 = v4733;
        let v5195: f64 = (p.p49 - 1.0);
        self.scalar_v5195 = v5195;
        let v5274: f64 = (p.p50 - 1.0);
        self.scalar_v5274 = v5274;
        let v5510: f64 = (0.0 * v2128);
        self.scalar_v5510 = v5510;
        let v5511: f64 = (p.p3 * 0.0);
        self.scalar_v5511 = v5511;
        let v5512: f64 = (0.0 * v2129);
        self.scalar_v5512 = v5512;
        let v5513: f64 = (0.0 * v2130);
        self.scalar_v5513 = v5513;
        let v5670: f64 = (p.p3 * p.p3);
        self.scalar_v5670 = v5670;
        let v5671: f64 = (p.p3 * v2128);
        self.scalar_v5671 = v5671;
        let v5700: f64 = (p.p3 * v2129);
        self.scalar_v5700 = v5700;
        let v5701: f64 = (p.p3 * v2130);
        self.scalar_v5701 = v5701;
    }

    #[inline]
    fn invalidate_temperature_static(&mut self) {
        self.scalar_temperature_static_valid = false;
    }

    #[inline]
    pub(super) fn ensure_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        if !self.scalar_temperature_static_valid
            || self.scalar_temperature_static_temperature.to_bits() != temperature.to_bits()
            || self.scalar_temperature_static_thermal_voltage.to_bits() != thermal_voltage.to_bits()
        {
            self.recompute_temperature_static(temperature, thermal_voltage);
        }
    }

    #[inline]
    fn recompute_temperature_static(&mut self, temperature: f64, thermal_voltage: f64) {
        let p = &(*self.params);
        let v20: f64 = (temperature + self.scalar_v19);
        self.scalar_v20 = v20;
        let v106: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v106 = v106;
        let v108: f64 = (self.scalar_v20 * 8.617086918058125e-5);
        self.scalar_v108 = v108;
        let v110: f64 = (1.0 / self.scalar_v108);
        self.scalar_v110 = v110;
        let v112: f64 = (self.scalar_v110 - self.scalar_v111);
        self.scalar_v112 = v112;
        let v113: f64 = (self.scalar_v20 - self.scalar_v17);
        self.scalar_v113 = v113;
        let v114: f64 = ((self.scalar_v106) as f64).ln();
        self.scalar_v114 = v114;
        let v115: f64 = (self.scalar_v20 * self.scalar_v42);
        self.scalar_v115 = v115;
        let v116: f64 = (self.scalar_v20 * self.scalar_v115);
        self.scalar_v116 = v116;
        let v117: f64 = (self.scalar_v20 + self.scalar_v45);
        self.scalar_v117 = v117;
        let v118: f64 = (self.scalar_v116 / self.scalar_v117);
        self.scalar_v118 = v118;
        let v119: f64 = (self.scalar_v67 - self.scalar_v118);
        self.scalar_v119 = v119;
        let v120: f64 = (self.scalar_v119 - 0.05);
        self.scalar_v120 = v120;
        let v121: f64 = (self.scalar_v120 / 0.1);
        self.scalar_v121 = v121;
        let v122: bool = (self.scalar_v119 < 0.05);
        self.scalar_v122 = v122;
        let v123: f64 = ((self.scalar_v121) as f64).exp();
        self.scalar_v123 = v123;
        let v124: f64 = (1.0 + self.scalar_v123);
        self.scalar_v124 = v124;
        let v125: f64 = ((self.scalar_v124) as f64).ln();
        self.scalar_v125 = v125;
        let v126: f64 = (0.1 * self.scalar_v125);
        self.scalar_v126 = v126;
        let v127: f64 = (0.05 + self.scalar_v126);
        self.scalar_v127 = v127;
        let v128: f64 = (if self.scalar_v122 { self.scalar_v127 } else { 0.0 });
        self.scalar_v128 = v128;
        let v129: bool = (!self.scalar_v122);
        self.scalar_v129 = v129;
        let v130: f64 = (-self.scalar_v121);
        self.scalar_v130 = v130;
        let v131: f64 = ((self.scalar_v130) as f64).exp();
        self.scalar_v131 = v131;
        let v132: f64 = (1.0 + self.scalar_v131);
        self.scalar_v132 = v132;
        let v133: f64 = ((self.scalar_v132) as f64).ln();
        self.scalar_v133 = v133;
        let v134: f64 = (0.1 * self.scalar_v133);
        self.scalar_v134 = v134;
        let v135: f64 = (self.scalar_v119 + self.scalar_v134);
        self.scalar_v135 = v135;
        let v136: f64 = (if self.scalar_v129 { self.scalar_v135 } else { self.scalar_v128 });
        self.scalar_v136 = v136;
        let v137: f64 = (self.scalar_v20 * self.scalar_v77);
        self.scalar_v137 = v137;
        let v138: f64 = (self.scalar_v20 * self.scalar_v137);
        self.scalar_v138 = v138;
        let v139: f64 = (self.scalar_v20 + self.scalar_v80);
        self.scalar_v139 = v139;
        let v140: f64 = (self.scalar_v138 / self.scalar_v139);
        self.scalar_v140 = v140;
        let v141: f64 = (self.scalar_v100 - self.scalar_v140);
        self.scalar_v141 = v141;
        let v142: f64 = (self.scalar_v141 - 0.05);
        self.scalar_v142 = v142;
        let v143: f64 = (self.scalar_v142 / 0.1);
        self.scalar_v143 = v143;
        let v144: bool = (self.scalar_v141 < 0.05);
        self.scalar_v144 = v144;
        let v145: f64 = ((self.scalar_v143) as f64).exp();
        self.scalar_v145 = v145;
        let v146: f64 = (1.0 + self.scalar_v145);
        self.scalar_v146 = v146;
        let v147: f64 = ((self.scalar_v146) as f64).ln();
        self.scalar_v147 = v147;
        let v148: f64 = (0.1 * self.scalar_v147);
        self.scalar_v148 = v148;
        let v149: f64 = (0.05 + self.scalar_v148);
        self.scalar_v149 = v149;
        let v150: f64 = (if self.scalar_v144 { self.scalar_v149 } else { 0.0 });
        self.scalar_v150 = v150;
        let v151: bool = (!self.scalar_v144);
        self.scalar_v151 = v151;
        let v152: f64 = (-self.scalar_v143);
        self.scalar_v152 = v152;
        let v153: f64 = ((self.scalar_v152) as f64).exp();
        self.scalar_v153 = v153;
        let v154: f64 = (1.0 + self.scalar_v153);
        self.scalar_v154 = v154;
        let v155: f64 = ((self.scalar_v154) as f64).ln();
        self.scalar_v155 = v155;
        let v156: f64 = (0.1 * self.scalar_v155);
        self.scalar_v156 = v156;
        let v157: f64 = (self.scalar_v141 + self.scalar_v156);
        self.scalar_v157 = v157;
        let v158: f64 = (if self.scalar_v151 { self.scalar_v157 } else { self.scalar_v150 });
        self.scalar_v158 = v158;
        let v161: f64 = (self.scalar_v108 * -3.0);
        self.scalar_v161 = v161;
        let v162: f64 = (self.scalar_v114 * self.scalar_v161);
        self.scalar_v162 = v162;
        let v163: f64 = (self.scalar_v69 * self.scalar_v106);
        self.scalar_v163 = v163;
        let v164: f64 = (self.scalar_v162 + self.scalar_v163);
        self.scalar_v164 = v164;
        let v165: f64 = (1.0 - self.scalar_v106);
        self.scalar_v165 = v165;
        let v167: f64 = (self.scalar_v165 * self.scalar_v166);
        self.scalar_v167 = v167;
        let v168: f64 = (self.scalar_v164 + self.scalar_v167);
        self.scalar_v168 = v168;
        let v169: f64 = (0.05 - self.scalar_v168);
        self.scalar_v169 = v169;
        let v170: f64 = (self.scalar_v169 / self.scalar_v108);
        self.scalar_v170 = v170;
        let v171: bool = (0.05 < self.scalar_v168);
        self.scalar_v171 = v171;
        let v172: f64 = ((self.scalar_v170) as f64).exp();
        self.scalar_v172 = v172;
        let v173: f64 = (1.0 + self.scalar_v172);
        self.scalar_v173 = v173;
        let v174: f64 = ((self.scalar_v173) as f64).ln();
        self.scalar_v174 = v174;
        let v175: f64 = (self.scalar_v108 * self.scalar_v174);
        self.scalar_v175 = v175;
        let v176: f64 = (self.scalar_v168 + self.scalar_v175);
        self.scalar_v176 = v176;
        let v177: f64 = (if self.scalar_v171 { self.scalar_v176 } else { 0.0 });
        self.scalar_v177 = v177;
        let v178: bool = (!self.scalar_v171);
        self.scalar_v178 = v178;
        let v179: f64 = (-self.scalar_v170);
        self.scalar_v179 = v179;
        let v180: f64 = ((self.scalar_v179) as f64).exp();
        self.scalar_v180 = v180;
        let v181: f64 = (1.0 + self.scalar_v180);
        self.scalar_v181 = v181;
        let v182: f64 = ((self.scalar_v181) as f64).ln();
        self.scalar_v182 = v182;
        let v183: f64 = (self.scalar_v108 * self.scalar_v182);
        self.scalar_v183 = v183;
        let v184: f64 = (0.05 + self.scalar_v183);
        self.scalar_v184 = v184;
        let v185: f64 = (if self.scalar_v178 { self.scalar_v184 } else { self.scalar_v177 });
        self.scalar_v185 = v185;
        let v187: f64 = (self.scalar_v106 * self.scalar_v186);
        self.scalar_v187 = v187;
        let v188: f64 = (self.scalar_v162 + self.scalar_v187);
        self.scalar_v188 = v188;
        let v190: f64 = (self.scalar_v165 * self.scalar_v189);
        self.scalar_v190 = v190;
        let v191: f64 = (self.scalar_v188 + self.scalar_v190);
        self.scalar_v191 = v191;
        let v192: f64 = (0.05 - self.scalar_v191);
        self.scalar_v192 = v192;
        let v193: f64 = (self.scalar_v192 / self.scalar_v108);
        self.scalar_v193 = v193;
        let v194: bool = (0.05 < self.scalar_v191);
        self.scalar_v194 = v194;
        let v195: f64 = ((self.scalar_v193) as f64).exp();
        self.scalar_v195 = v195;
        let v196: f64 = (1.0 + self.scalar_v195);
        self.scalar_v196 = v196;
        let v197: f64 = ((self.scalar_v196) as f64).ln();
        self.scalar_v197 = v197;
        let v198: f64 = (self.scalar_v108 * self.scalar_v197);
        self.scalar_v198 = v198;
        let v199: f64 = (self.scalar_v191 + self.scalar_v198);
        self.scalar_v199 = v199;
        let v200: f64 = (if self.scalar_v194 { self.scalar_v199 } else { 0.0 });
        self.scalar_v200 = v200;
        let v201: bool = (!self.scalar_v194);
        self.scalar_v201 = v201;
        let v202: f64 = (-self.scalar_v193);
        self.scalar_v202 = v202;
        let v203: f64 = ((self.scalar_v202) as f64).exp();
        self.scalar_v203 = v203;
        let v204: f64 = (1.0 + self.scalar_v203);
        self.scalar_v204 = v204;
        let v205: f64 = ((self.scalar_v204) as f64).ln();
        self.scalar_v205 = v205;
        let v206: f64 = (self.scalar_v108 * self.scalar_v205);
        self.scalar_v206 = v206;
        let v207: f64 = (0.05 + self.scalar_v206);
        self.scalar_v207 = v207;
        let v208: f64 = (if self.scalar_v201 { self.scalar_v207 } else { self.scalar_v200 });
        self.scalar_v208 = v208;
        let v209: f64 = (self.scalar_v71 * self.scalar_v106);
        self.scalar_v209 = v209;
        let v210: f64 = (self.scalar_v162 + self.scalar_v209);
        self.scalar_v210 = v210;
        let v211: f64 = (self.scalar_v190 + self.scalar_v210);
        self.scalar_v211 = v211;
        let v212: f64 = (0.05 - self.scalar_v211);
        self.scalar_v212 = v212;
        let v213: f64 = (self.scalar_v212 / self.scalar_v108);
        self.scalar_v213 = v213;
        let v214: bool = (0.05 < self.scalar_v211);
        self.scalar_v214 = v214;
        let v215: f64 = ((self.scalar_v213) as f64).exp();
        self.scalar_v215 = v215;
        let v216: f64 = (1.0 + self.scalar_v215);
        self.scalar_v216 = v216;
        let v217: f64 = ((self.scalar_v216) as f64).ln();
        self.scalar_v217 = v217;
        let v218: f64 = (self.scalar_v108 * self.scalar_v217);
        self.scalar_v218 = v218;
        let v219: f64 = (self.scalar_v211 + self.scalar_v218);
        self.scalar_v219 = v219;
        let v220: f64 = (if self.scalar_v214 { self.scalar_v219 } else { 0.0 });
        self.scalar_v220 = v220;
        let v221: bool = (!self.scalar_v214);
        self.scalar_v221 = v221;
        let v222: f64 = (-self.scalar_v213);
        self.scalar_v222 = v222;
        let v223: f64 = ((self.scalar_v222) as f64).exp();
        self.scalar_v223 = v223;
        let v224: f64 = (1.0 + self.scalar_v223);
        self.scalar_v224 = v224;
        let v225: f64 = ((self.scalar_v224) as f64).ln();
        self.scalar_v225 = v225;
        let v226: f64 = (self.scalar_v108 * self.scalar_v225);
        self.scalar_v226 = v226;
        let v227: f64 = (0.05 + self.scalar_v226);
        self.scalar_v227 = v227;
        let v228: f64 = (if self.scalar_v221 { self.scalar_v227 } else { self.scalar_v220 });
        self.scalar_v228 = v228;
        let v230: f64 = (self.scalar_v106 * self.scalar_v229);
        self.scalar_v230 = v230;
        let v231: f64 = (self.scalar_v162 + self.scalar_v230);
        self.scalar_v231 = v231;
        let v233: f64 = (self.scalar_v165 * self.scalar_v232);
        self.scalar_v233 = v233;
        let v234: f64 = (self.scalar_v231 + self.scalar_v233);
        self.scalar_v234 = v234;
        let v235: f64 = (0.05 - self.scalar_v234);
        self.scalar_v235 = v235;
        let v236: f64 = (self.scalar_v235 / self.scalar_v108);
        self.scalar_v236 = v236;
        let v237: bool = (0.05 < self.scalar_v234);
        self.scalar_v237 = v237;
        let v238: f64 = ((self.scalar_v236) as f64).exp();
        self.scalar_v238 = v238;
        let v239: f64 = (1.0 + self.scalar_v238);
        self.scalar_v239 = v239;
        let v240: f64 = ((self.scalar_v239) as f64).ln();
        self.scalar_v240 = v240;
        let v241: f64 = (self.scalar_v108 * self.scalar_v240);
        self.scalar_v241 = v241;
        let v242: f64 = (self.scalar_v234 + self.scalar_v241);
        self.scalar_v242 = v242;
        let v243: f64 = (if self.scalar_v237 { self.scalar_v242 } else { 0.0 });
        self.scalar_v243 = v243;
        let v244: bool = (!self.scalar_v237);
        self.scalar_v244 = v244;
        let v245: f64 = (-self.scalar_v236);
        self.scalar_v245 = v245;
        let v246: f64 = ((self.scalar_v245) as f64).exp();
        self.scalar_v246 = v246;
        let v247: f64 = (1.0 + self.scalar_v246);
        self.scalar_v247 = v247;
        let v248: f64 = ((self.scalar_v247) as f64).ln();
        self.scalar_v248 = v248;
        let v249: f64 = (self.scalar_v108 * self.scalar_v248);
        self.scalar_v249 = v249;
        let v250: f64 = (0.05 + self.scalar_v249);
        self.scalar_v250 = v250;
        let v251: f64 = (if self.scalar_v244 { self.scalar_v250 } else { self.scalar_v243 });
        self.scalar_v251 = v251;
        let v253: f64 = (self.scalar_v106 * self.scalar_v252);
        self.scalar_v253 = v253;
        let v254: f64 = (self.scalar_v162 + self.scalar_v253);
        self.scalar_v254 = v254;
        let v256: f64 = (self.scalar_v165 * self.scalar_v255);
        self.scalar_v256 = v256;
        let v257: f64 = (self.scalar_v254 + self.scalar_v256);
        self.scalar_v257 = v257;
        let v258: f64 = (0.05 - self.scalar_v257);
        self.scalar_v258 = v258;
        let v259: f64 = (self.scalar_v258 / self.scalar_v108);
        self.scalar_v259 = v259;
        let v260: f64 = (1.0 / self.scalar_v185);
        self.scalar_v260 = v260;
        let v261: f64 = (1.0 / self.scalar_v228);
        self.scalar_v261 = v261;
        let v262: f64 = (self.scalar_v69 * self.scalar_v260);
        self.scalar_v262 = v262;
        let v263: f64 = f64::powf(self.scalar_v262, self.scalar_v37);
        self.scalar_v263 = v263;
        let v264: f64 = (self.scalar_v71 * self.scalar_v261);
        self.scalar_v264 = v264;
        let v265: f64 = f64::powf(self.scalar_v264, self.scalar_v72);
        self.scalar_v265 = v265;
        let v268: f64 = (self.scalar_v71 / self.scalar_v228);
        self.scalar_v268 = v268;
        let v269: f64 = f64::powf(self.scalar_v268, self.scalar_v72);
        self.scalar_v269 = v269;
        let v270: f64 = (self.scalar_v267 * self.scalar_v269);
        self.scalar_v270 = v270;
        let v271: f64 = (self.scalar_v266 + self.scalar_v270);
        self.scalar_v271 = v271;
        let v272: f64 = (1.0 / self.scalar_v271);
        self.scalar_v272 = v272;
        let v273: f64 = (self.scalar_v266 * self.scalar_v272);
        self.scalar_v273 = v273;
        let v276: f64 = (self.scalar_v114 * self.scalar_v275);
        self.scalar_v276 = v276;
        let v277: f64 = ((self.scalar_v276) as f64).exp();
        self.scalar_v277 = v277;
        let v278: f64 = (self.scalar_v274 * self.scalar_v277);
        self.scalar_v278 = v278;
        let v279: bool = (self.scalar_v278 < self.scalar_v28);
        self.scalar_v279 = v279;
        let v280: f64 = (if self.scalar_v279 { self.scalar_v28 } else { self.scalar_v278 });
        self.scalar_v280 = v280;
        let v285: f64 = (self.scalar_v114 * self.scalar_v284);
        self.scalar_v285 = v285;
        let v286: f64 = ((self.scalar_v285) as f64).exp();
        self.scalar_v286 = v286;
        let v287: f64 = (self.scalar_v281 * self.scalar_v286);
        self.scalar_v287 = v287;
        let v290: f64 = (self.scalar_v114 * self.scalar_v289);
        self.scalar_v290 = v290;
        let v291: f64 = ((self.scalar_v290) as f64).exp();
        self.scalar_v291 = v291;
        let v292: f64 = (self.scalar_v288 * self.scalar_v291);
        self.scalar_v292 = v292;
        let v293: bool = (self.scalar_v292 < self.scalar_v28);
        self.scalar_v293 = v293;
        let v294: f64 = (if self.scalar_v293 { self.scalar_v28 } else { self.scalar_v292 });
        self.scalar_v294 = v294;
        let v297: f64 = (self.scalar_v114 * self.scalar_v296);
        self.scalar_v297 = v297;
        let v298: f64 = ((self.scalar_v297) as f64).exp();
        self.scalar_v298 = v298;
        let v299: f64 = (self.scalar_v295 * self.scalar_v298);
        self.scalar_v299 = v299;
        let v302: f64 = (self.scalar_v114 * self.scalar_v301);
        self.scalar_v302 = v302;
        let v303: f64 = ((self.scalar_v302) as f64).exp();
        self.scalar_v303 = v303;
        let v304: f64 = (self.scalar_v300 * self.scalar_v303);
        self.scalar_v304 = v304;
        let v306: f64 = (self.scalar_v303 * self.scalar_v305);
        self.scalar_v306 = v306;
        let v309: f64 = (self.scalar_v114 * self.scalar_v308);
        self.scalar_v309 = v309;
        let v310: f64 = ((self.scalar_v309) as f64).exp();
        self.scalar_v310 = v310;
        let v311: f64 = (self.scalar_v307 * self.scalar_v310);
        self.scalar_v311 = v311;
        let v315: f64 = (self.scalar_v113 * self.scalar_v312);
        self.scalar_v315 = v315;
        let v316: f64 = (1.0 + self.scalar_v315);
        self.scalar_v316 = v316;
        let v317: f64 = (self.scalar_v314 * self.scalar_v316);
        self.scalar_v317 = v317;
        let v318: f64 = (if self.scalar_v313 { self.scalar_v317 } else { 0.0 });
        self.scalar_v318 = v318;
        let v319: f64 = (self.scalar_v318 - 1.0);
        self.scalar_v319 = v319;
        let v320: f64 = (self.scalar_v319 / 0.001);
        self.scalar_v320 = v320;
        let v321: f64 = (if self.scalar_v313 { self.scalar_v320 } else { self.scalar_v259 });
        self.scalar_v321 = v321;
        let v322: bool = (self.scalar_v318 < 1.0);
        self.scalar_v322 = v322;
        let v323: bool = (self.scalar_v313 && self.scalar_v322);
        self.scalar_v323 = v323;
        let v324: f64 = ((self.scalar_v321) as f64).exp();
        self.scalar_v324 = v324;
        let v325: f64 = (1.0 + self.scalar_v324);
        self.scalar_v325 = v325;
        let v326: f64 = ((self.scalar_v325) as f64).ln();
        self.scalar_v326 = v326;
        let v327: f64 = (0.001 * self.scalar_v326);
        self.scalar_v327 = v327;
        let v328: f64 = (1.0 + self.scalar_v327);
        self.scalar_v328 = v328;
        let v329: f64 = (if self.scalar_v323 { self.scalar_v328 } else { self.scalar_v318 });
        self.scalar_v329 = v329;
        let v330: bool = (!self.scalar_v322);
        self.scalar_v330 = v330;
        let v331: bool = (self.scalar_v313 && self.scalar_v330);
        self.scalar_v331 = v331;
        let v332: f64 = (-self.scalar_v321);
        self.scalar_v332 = v332;
        let v333: f64 = ((self.scalar_v332) as f64).exp();
        self.scalar_v333 = v333;
        let v334: f64 = (1.0 + self.scalar_v333);
        self.scalar_v334 = v334;
        let v335: f64 = ((self.scalar_v334) as f64).ln();
        self.scalar_v335 = v335;
        let v336: f64 = (0.001 * self.scalar_v335);
        self.scalar_v336 = v336;
        let v337: f64 = (self.scalar_v329 + self.scalar_v336);
        self.scalar_v337 = v337;
        let v338: f64 = (if self.scalar_v331 { self.scalar_v337 } else { self.scalar_v329 });
        self.scalar_v338 = v338;
        let v340: f64 = (self.scalar_v338 - 0.0006931471805599453);
        self.scalar_v340 = v340;
        let v341: f64 = (if self.scalar_v313 { self.scalar_v340 } else { 0.0 });
        self.scalar_v341 = v341;
        let v343: f64 = (if self.scalar_v342 { self.scalar_v314 } else { self.scalar_v341 });
        self.scalar_v343 = v343;
        let v347: f64 = (self.scalar_v113 * self.scalar_v344);
        self.scalar_v347 = v347;
        let v348: f64 = (1.0 + self.scalar_v347);
        self.scalar_v348 = v348;
        let v349: f64 = (self.scalar_v346 * self.scalar_v348);
        self.scalar_v349 = v349;
        let v350: f64 = (if self.scalar_v345 { self.scalar_v349 } else { 0.0 });
        self.scalar_v350 = v350;
        let v351: f64 = (self.scalar_v350 - 1.0);
        self.scalar_v351 = v351;
        let v352: f64 = (self.scalar_v351 / 0.001);
        self.scalar_v352 = v352;
        let v353: f64 = (if self.scalar_v345 { self.scalar_v352 } else { self.scalar_v321 });
        self.scalar_v353 = v353;
        let v354: bool = (self.scalar_v350 < 1.0);
        self.scalar_v354 = v354;
        let v355: bool = (self.scalar_v345 && self.scalar_v354);
        self.scalar_v355 = v355;
        let v356: f64 = ((self.scalar_v353) as f64).exp();
        self.scalar_v356 = v356;
        let v357: f64 = (1.0 + self.scalar_v356);
        self.scalar_v357 = v357;
        let v358: f64 = ((self.scalar_v357) as f64).ln();
        self.scalar_v358 = v358;
        let v359: f64 = (0.001 * self.scalar_v358);
        self.scalar_v359 = v359;
        let v360: f64 = (1.0 + self.scalar_v359);
        self.scalar_v360 = v360;
        let v361: f64 = (if self.scalar_v355 { self.scalar_v360 } else { self.scalar_v350 });
        self.scalar_v361 = v361;
        let v362: bool = (!self.scalar_v354);
        self.scalar_v362 = v362;
        let v363: bool = (self.scalar_v345 && self.scalar_v362);
        self.scalar_v363 = v363;
        let v364: f64 = (-self.scalar_v353);
        self.scalar_v364 = v364;
        let v365: f64 = ((self.scalar_v364) as f64).exp();
        self.scalar_v365 = v365;
        let v366: f64 = (1.0 + self.scalar_v365);
        self.scalar_v366 = v366;
        let v367: f64 = ((self.scalar_v366) as f64).ln();
        self.scalar_v367 = v367;
        let v368: f64 = (0.001 * self.scalar_v367);
        self.scalar_v368 = v368;
        let v369: f64 = (self.scalar_v361 + self.scalar_v368);
        self.scalar_v369 = v369;
        let v370: f64 = (if self.scalar_v363 { self.scalar_v369 } else { self.scalar_v361 });
        self.scalar_v370 = v370;
        let v371: f64 = (self.scalar_v370 - 0.0006931471805599453);
        self.scalar_v371 = v371;
        let v372: f64 = (if self.scalar_v345 { self.scalar_v371 } else { 0.0 });
        self.scalar_v372 = v372;
        let v374: f64 = (if self.scalar_v373 { self.scalar_v346 } else { self.scalar_v372 });
        self.scalar_v374 = v374;
        let v377: f64 = (self.scalar_v113 * self.scalar_v376);
        self.scalar_v377 = v377;
        let v378: f64 = (1.0 + self.scalar_v377);
        self.scalar_v378 = v378;
        let v379: f64 = (self.scalar_v375 * self.scalar_v378);
        self.scalar_v379 = v379;
        let v381: f64 = (self.scalar_v379 * self.scalar_v379);
        self.scalar_v381 = v381;
        let v382: bool = (self.scalar_v379 < 0.0);
        self.scalar_v382 = v382;
        let v385: f64 = (1e-6 + self.scalar_v381);
        self.scalar_v385 = v385;
        let v386: f64 = ((self.scalar_v385) as f64).sqrt();
        self.scalar_v386 = v386;
        let v387: f64 = (self.scalar_v386 - self.scalar_v379);
        self.scalar_v387 = v387;
        let v388: f64 = (5e-7 / self.scalar_v387);
        self.scalar_v388 = v388;
        let v389: f64 = (if self.scalar_v382 { self.scalar_v388 } else { 0.0 });
        self.scalar_v389 = v389;
        let v390: bool = (!self.scalar_v382);
        self.scalar_v390 = v390;
        let v391: f64 = (self.scalar_v379 + self.scalar_v386);
        self.scalar_v391 = v391;
        let v392: f64 = (0.5 * self.scalar_v391);
        self.scalar_v392 = v392;
        let v393: f64 = (if self.scalar_v390 { self.scalar_v392 } else { self.scalar_v389 });
        self.scalar_v393 = v393;
        let v400: f64 = (self.scalar_v114 * self.scalar_v399);
        self.scalar_v400 = v400;
        let v401: f64 = (self.scalar_v400 / self.scalar_v343);
        self.scalar_v401 = v401;
        let v402: f64 = ((self.scalar_v401) as f64).exp();
        self.scalar_v402 = v402;
        let v403: f64 = (self.scalar_v394 * self.scalar_v402);
        self.scalar_v403 = v403;
        let v405: f64 = (self.scalar_v112 * self.scalar_v404);
        self.scalar_v405 = v405;
        let v406: f64 = (self.scalar_v405 / self.scalar_v343);
        self.scalar_v406 = v406;
        let v407: f64 = ((self.scalar_v406) as f64).exp();
        self.scalar_v407 = v407;
        let v408: f64 = (self.scalar_v403 * self.scalar_v407);
        self.scalar_v408 = v408;
        let v411: f64 = (self.scalar_v114 * self.scalar_v410);
        self.scalar_v411 = v411;
        let v412: f64 = ((self.scalar_v411) as f64).exp();
        self.scalar_v412 = v412;
        let v413: f64 = (self.scalar_v409 * self.scalar_v412);
        self.scalar_v413 = v413;
        let v417: f64 = (self.scalar_v114 * self.scalar_v416);
        self.scalar_v417 = v417;
        let v418: f64 = ((self.scalar_v417) as f64).exp();
        self.scalar_v418 = v418;
        let v419: f64 = (self.scalar_v414 * self.scalar_v418);
        self.scalar_v419 = v419;
        let v425: f64 = (self.scalar_v114 * self.scalar_v424);
        self.scalar_v425 = v425;
        let v426: f64 = ((self.scalar_v425) as f64).exp();
        self.scalar_v426 = v426;
        let v427: f64 = (self.scalar_v420 * self.scalar_v426);
        self.scalar_v427 = v427;
        let v430: f64 = (self.scalar_v112 * self.scalar_v429);
        self.scalar_v430 = v430;
        let v431: f64 = (self.scalar_v430 / self.scalar_v422);
        self.scalar_v431 = v431;
        let v432: f64 = ((self.scalar_v431) as f64).exp();
        self.scalar_v432 = v432;
        let v433: f64 = (self.scalar_v427 * self.scalar_v432);
        self.scalar_v433 = v433;
        let v438: f64 = (self.scalar_v114 * self.scalar_v437);
        self.scalar_v438 = v438;
        let v439: f64 = ((self.scalar_v438) as f64).exp();
        self.scalar_v439 = v439;
        let v440: f64 = (self.scalar_v434 * self.scalar_v439);
        self.scalar_v440 = v440;
        let v442: f64 = (self.scalar_v112 * self.scalar_v441);
        self.scalar_v442 = v442;
        let v443: f64 = (self.scalar_v442 / self.scalar_v435);
        self.scalar_v443 = v443;
        let v444: f64 = ((self.scalar_v443) as f64).exp();
        self.scalar_v444 = v444;
        let v445: f64 = (self.scalar_v440 * self.scalar_v444);
        self.scalar_v445 = v445;
        let v449: f64 = (self.scalar_v114 * self.scalar_v448);
        self.scalar_v449 = v449;
        let v451: f64 = (self.scalar_v449 / self.scalar_v450);
        self.scalar_v451 = v451;
        let v452: f64 = ((self.scalar_v451) as f64).exp();
        self.scalar_v452 = v452;
        let v453: f64 = (self.scalar_v446 * self.scalar_v452);
        self.scalar_v453 = v453;
        let v456: f64 = (self.scalar_v112 * self.scalar_v455);
        self.scalar_v456 = v456;
        let v457: f64 = (self.scalar_v456 / self.scalar_v450);
        self.scalar_v457 = v457;
        let v458: f64 = ((self.scalar_v457) as f64).exp();
        self.scalar_v458 = v458;
        let v459: f64 = (self.scalar_v453 * self.scalar_v458);
        self.scalar_v459 = v459;
        let v462: f64 = (self.scalar_v449 / self.scalar_v461);
        self.scalar_v462 = v462;
        let v463: f64 = ((self.scalar_v462) as f64).exp();
        self.scalar_v463 = v463;
        let v464: f64 = (self.scalar_v460 * self.scalar_v463);
        self.scalar_v464 = v464;
        let v465: f64 = (self.scalar_v456 / self.scalar_v461);
        self.scalar_v465 = v465;
        let v466: f64 = ((self.scalar_v465) as f64).exp();
        self.scalar_v466 = v466;
        let v467: f64 = (self.scalar_v464 * self.scalar_v466);
        self.scalar_v467 = v467;
        let v473: f64 = (self.scalar_v112 * self.scalar_v472);
        self.scalar_v473 = v473;
        let v474: f64 = (self.scalar_v473 / self.scalar_v450);
        self.scalar_v474 = v474;
        let v475: f64 = ((self.scalar_v474) as f64).exp();
        self.scalar_v475 = v475;
        let v476: f64 = (self.scalar_v470 * self.scalar_v475);
        self.scalar_v476 = v476;
        let v477: f64 = (if self.scalar_v469 { self.scalar_v476 } else { 0.0 });
        self.scalar_v477 = v477;
        let v481: f64 = (self.scalar_v112 * self.scalar_v480);
        self.scalar_v481 = v481;
        let v482: f64 = ((self.scalar_v481) as f64).exp();
        self.scalar_v482 = v482;
        let v483: f64 = (self.scalar_v478 * self.scalar_v482);
        self.scalar_v483 = v483;
        let v484: f64 = (if self.scalar_v469 { self.scalar_v483 } else { 0.0 });
        self.scalar_v484 = v484;
        let v488: f64 = (self.scalar_v112 * self.scalar_v487);
        self.scalar_v488 = v488;
        let v489: f64 = (self.scalar_v488 / self.scalar_v461);
        self.scalar_v489 = v489;
        let v490: f64 = ((self.scalar_v489) as f64).exp();
        self.scalar_v490 = v490;
        let v491: f64 = (self.scalar_v485 * self.scalar_v490);
        self.scalar_v491 = v491;
        let v492: f64 = (if self.scalar_v469 { self.scalar_v491 } else { 0.0 });
        self.scalar_v492 = v492;
        let v496: f64 = (self.scalar_v114 * self.scalar_v495);
        self.scalar_v496 = v496;
        let v497: f64 = ((self.scalar_v496) as f64).exp();
        self.scalar_v497 = v497;
        let v498: f64 = (self.scalar_v493 * self.scalar_v497);
        self.scalar_v498 = v498;
        let v501: f64 = (self.scalar_v112 * self.scalar_v500);
        self.scalar_v501 = v501;
        let v502: f64 = ((self.scalar_v501) as f64).exp();
        self.scalar_v502 = v502;
        let v503: f64 = (self.scalar_v498 * self.scalar_v502);
        self.scalar_v503 = v503;
        let v508: f64 = (self.scalar_v114 * self.scalar_v507);
        self.scalar_v508 = v508;
        let v509: f64 = ((self.scalar_v508) as f64).exp();
        self.scalar_v509 = v509;
        let v510: f64 = (self.scalar_v504 * self.scalar_v509);
        self.scalar_v510 = v510;
        let v511: f64 = (self.scalar_v430 / self.scalar_v505);
        self.scalar_v511 = v511;
        let v512: f64 = ((self.scalar_v511) as f64).exp();
        self.scalar_v512 = v512;
        let v513: f64 = (self.scalar_v510 * self.scalar_v512);
        self.scalar_v513 = v513;
        let v517: f64 = (self.scalar_v114 * self.scalar_v516);
        self.scalar_v517 = v517;
        let v518: f64 = ((self.scalar_v517) as f64).exp();
        self.scalar_v518 = v518;
        let v519: f64 = (self.scalar_v514 * self.scalar_v518);
        self.scalar_v519 = v519;
        let v520: f64 = (self.scalar_v430 / self.scalar_v515);
        self.scalar_v520 = v520;
        let v521: f64 = ((self.scalar_v520) as f64).exp();
        self.scalar_v521 = v521;
        let v522: f64 = (self.scalar_v519 * self.scalar_v521);
        self.scalar_v522 = v522;
        let v524: f64 = ((self.scalar_v106) as f64).sqrt();
        self.scalar_v524 = v524;
        let v525: f64 = (self.scalar_v523 * self.scalar_v524);
        self.scalar_v525 = v525;
        let v527: f64 = (self.scalar_v113 * self.scalar_v526);
        self.scalar_v527 = v527;
        let v528: f64 = ((self.scalar_v527) as f64).exp();
        self.scalar_v528 = v528;
        let v529: f64 = (self.scalar_v525 * self.scalar_v528);
        self.scalar_v529 = v529;
        let v530: f64 = (self.scalar_v68 * self.scalar_v136);
        self.scalar_v530 = v530;
        let v532: f64 = f64::powf(self.scalar_v530, -0.5);
        self.scalar_v532 = v532;
        let v533: f64 = (1.0 / self.scalar_v263);
        self.scalar_v533 = v533;
        let v535: f64 = (self.scalar_v136 * self.scalar_v534);
        self.scalar_v535 = v535;
        let v536: f64 = (self.scalar_v136 * self.scalar_v535);
        self.scalar_v536 = v536;
        let v537: f64 = (self.scalar_v532 * self.scalar_v536);
        self.scalar_v537 = v537;
        let v538: f64 = (self.scalar_v533 * self.scalar_v537);
        self.scalar_v538 = v538;
        let v539: f64 = (self.scalar_v69 * self.scalar_v538);
        self.scalar_v539 = v539;
        let v540: f64 = (self.scalar_v260 * self.scalar_v539);
        self.scalar_v540 = v540;
        let v541: f64 = (self.scalar_v68 * self.scalar_v540);
        self.scalar_v541 = v541;
        let v542: f64 = (self.scalar_v68 * self.scalar_v541);
        self.scalar_v542 = v542;
        let v544: f64 = (self.scalar_v532 * self.scalar_v543);
        self.scalar_v544 = v544;
        let v545: f64 = (self.scalar_v185 * self.scalar_v544);
        self.scalar_v545 = v545;
        let v546: f64 = (self.scalar_v185 * self.scalar_v545);
        self.scalar_v546 = v546;
        let v547: f64 = (self.scalar_v70 * self.scalar_v546);
        self.scalar_v547 = v547;
        let v548: f64 = (self.scalar_v70 * self.scalar_v547);
        self.scalar_v548 = v548;
        let v549: f64 = (self.scalar_v263 * self.scalar_v548);
        self.scalar_v549 = v549;
        let v550: f64 = (self.scalar_v534 - self.scalar_v542);
        self.scalar_v550 = v550;
        let v551: f64 = ((self.scalar_v550) as f64).exp();
        self.scalar_v551 = v551;
        let v552: f64 = (self.scalar_v549 * self.scalar_v551);
        self.scalar_v552 = v552;
        let v553: f64 = (self.scalar_v101 * self.scalar_v158);
        self.scalar_v553 = v553;
        let v554: f64 = f64::powf(self.scalar_v553, -0.5);
        self.scalar_v554 = v554;
        let v555: f64 = (1.0 / self.scalar_v265);
        self.scalar_v555 = v555;
        let v557: f64 = (self.scalar_v158 * self.scalar_v556);
        self.scalar_v557 = v557;
        let v558: f64 = (self.scalar_v158 * self.scalar_v557);
        self.scalar_v558 = v558;
        let v559: f64 = (self.scalar_v554 * self.scalar_v558);
        self.scalar_v559 = v559;
        let v560: f64 = (self.scalar_v555 * self.scalar_v559);
        self.scalar_v560 = v560;
        let v561: f64 = (self.scalar_v71 * self.scalar_v560);
        self.scalar_v561 = v561;
        let v562: f64 = (self.scalar_v261 * self.scalar_v561);
        self.scalar_v562 = v562;
        let v563: f64 = (self.scalar_v101 * self.scalar_v562);
        self.scalar_v563 = v563;
        let v564: f64 = (self.scalar_v101 * self.scalar_v563);
        self.scalar_v564 = v564;
        let v566: f64 = (self.scalar_v554 * self.scalar_v565);
        self.scalar_v566 = v566;
        let v567: f64 = (self.scalar_v228 * self.scalar_v566);
        self.scalar_v567 = v567;
        let v568: f64 = (self.scalar_v228 * self.scalar_v567);
        self.scalar_v568 = v568;
        let v569: f64 = (self.scalar_v102 * self.scalar_v568);
        self.scalar_v569 = v569;
        let v570: f64 = (self.scalar_v102 * self.scalar_v569);
        self.scalar_v570 = v570;
        let v571: f64 = (self.scalar_v265 * self.scalar_v570);
        self.scalar_v571 = v571;
        let v572: f64 = (self.scalar_v556 - self.scalar_v564);
        self.scalar_v572 = v572;
        let v573: f64 = ((self.scalar_v572) as f64).exp();
        self.scalar_v573 = v573;
        let v574: f64 = (self.scalar_v571 * self.scalar_v573);
        self.scalar_v574 = v574;
        let v575: f64 = (self.scalar_v114 * self.scalar_v283);
        self.scalar_v575 = v575;
        let v576: f64 = ((self.scalar_v575) as f64).exp();
        self.scalar_v576 = v576;
        let v578: f64 = (self.scalar_v576 * self.scalar_v577);
        self.scalar_v578 = v578;
        let v579: f64 = (self.scalar_v272 * self.scalar_v578);
        self.scalar_v579 = v579;
        let v581: f64 = (self.scalar_v576 * self.scalar_v580);
        self.scalar_v581 = v581;
        let v582: f64 = (self.scalar_v533 * self.scalar_v581);
        self.scalar_v582 = v582;
        let v586: f64 = (self.scalar_v114 * self.scalar_v585);
        self.scalar_v586 = v586;
        let v587: f64 = ((self.scalar_v586) as f64).exp();
        self.scalar_v587 = v587;
        let v588: f64 = (self.scalar_v583 * self.scalar_v587);
        self.scalar_v588 = v588;
        let v590: f64 = (self.scalar_v112 * self.scalar_v589);
        self.scalar_v590 = v590;
        let v591: f64 = ((self.scalar_v590) as f64).exp();
        self.scalar_v591 = v591;
        let v592: f64 = (self.scalar_v588 * self.scalar_v591);
        self.scalar_v592 = v592;
        let v597: f64 = (self.scalar_v114 * self.scalar_v596);
        self.scalar_v597 = v597;
        let v598: f64 = ((self.scalar_v597) as f64).exp();
        self.scalar_v598 = v598;
        let v599: f64 = (self.scalar_v30 * self.scalar_v598);
        self.scalar_v599 = v599;
        let v600: f64 = (self.scalar_v591 * self.scalar_v599);
        self.scalar_v600 = v600;
        let v603: f64 = (self.scalar_v114 * self.scalar_v602);
        self.scalar_v603 = v603;
        let v604: f64 = ((self.scalar_v603) as f64).exp();
        self.scalar_v604 = v604;
        let v605: f64 = (self.scalar_v601 * self.scalar_v604);
        self.scalar_v605 = v605;
        let v608: f64 = (self.scalar_v114 * self.scalar_v607);
        self.scalar_v608 = v608;
        let v609: f64 = ((self.scalar_v608) as f64).exp();
        self.scalar_v609 = v609;
        let v610: f64 = (self.scalar_v606 * self.scalar_v609);
        self.scalar_v610 = v610;
        let v612: f64 = (self.scalar_v20 - 300.0);
        self.scalar_v612 = v612;
        let v614: bool = (self.scalar_v20 < 525.0);
        self.scalar_v614 = v614;
        let v616: f64 = (self.scalar_v612 * 0.00072);
        self.scalar_v616 = v616;
        let v617: f64 = (1.0 + self.scalar_v616);
        self.scalar_v617 = v617;
        let v619: f64 = (self.scalar_v612 * 1.6e-6);
        self.scalar_v619 = v619;
        let v620: f64 = (self.scalar_v612 * self.scalar_v619);
        self.scalar_v620 = v620;
        let v621: f64 = (self.scalar_v617 - self.scalar_v620);
        self.scalar_v621 = v621;
        let v622: f64 = (self.scalar_v12 * self.scalar_v621);
        self.scalar_v622 = v622;
        let v623: f64 = (if self.scalar_v614 { self.scalar_v622 } else { 0.0 });
        self.scalar_v623 = v623;
        let v624: bool = (!self.scalar_v614);
        self.scalar_v624 = v624;
        let v627: f64 = (if self.scalar_v624 { self.scalar_v626 } else { self.scalar_v623 });
        self.scalar_v627 = v627;
        let v629: f64 = (self.scalar_v576 * self.scalar_v628);
        self.scalar_v629 = v629;
        let v631: f64 = (1.0 / self.scalar_v299);
        self.scalar_v631 = v631;
        let v632: f64 = (if self.scalar_v630 { self.scalar_v631 } else { 0.0 });
        self.scalar_v632 = v632;
        let v633: bool = (self.scalar_v632 > self.scalar_v29);
        self.scalar_v633 = v633;
        let v634: bool = (self.scalar_v630 && self.scalar_v633);
        self.scalar_v634 = v634;
        let v635: f64 = (if self.scalar_v634 { self.scalar_v29 } else { self.scalar_v632 });
        self.scalar_v635 = v635;
        let v637: f64 = (if self.scalar_v636 { 0.0 } else { self.scalar_v635 });
        self.scalar_v637 = v637;
        let v639: f64 = (1.0 / self.scalar_v304);
        self.scalar_v639 = v639;
        let v640: f64 = (if self.scalar_v638 { self.scalar_v639 } else { 0.0 });
        self.scalar_v640 = v640;
        let v641: bool = (self.scalar_v640 > self.scalar_v29);
        self.scalar_v641 = v641;
        let v642: bool = (self.scalar_v638 && self.scalar_v641);
        self.scalar_v642 = v642;
        let v643: f64 = (if self.scalar_v642 { self.scalar_v29 } else { self.scalar_v640 });
        self.scalar_v643 = v643;
        let v645: f64 = (if self.scalar_v644 { 0.0 } else { self.scalar_v643 });
        self.scalar_v645 = v645;
        let v647: f64 = (1.0 / self.scalar_v306);
        self.scalar_v647 = v647;
        let v648: f64 = (if self.scalar_v646 { self.scalar_v647 } else { 0.0 });
        self.scalar_v648 = v648;
        let v649: bool = (self.scalar_v648 > self.scalar_v29);
        self.scalar_v649 = v649;
        let v650: bool = (self.scalar_v646 && self.scalar_v649);
        self.scalar_v650 = v650;
        let v651: f64 = (if self.scalar_v650 { self.scalar_v29 } else { self.scalar_v648 });
        self.scalar_v651 = v651;
        let v653: f64 = (if self.scalar_v652 { 0.0 } else { self.scalar_v651 });
        self.scalar_v653 = v653;
        let v846: f64 = (2.0 * self.scalar_v108);
        self.scalar_v846 = v846;
        let v857: f64 = (self.scalar_v208 * 0.2);
        self.scalar_v857 = v857;
        let v881: f64 = (self.scalar_v311 * self.scalar_v876);
        self.scalar_v881 = v881;
        let v968: f64 = (self.scalar_v110 * self.scalar_v208);
        self.scalar_v968 = v968;
        let v969: f64 = ((self.scalar_v968) as f64).exp();
        self.scalar_v969 = v969;
        let v976: f64 = (self.scalar_v311 * self.scalar_v877);
        self.scalar_v976 = v976;
        let v977: f64 = (self.scalar_v876 * self.scalar_v976);
        self.scalar_v977 = v977;
        let v988: f64 = (0.1 * self.scalar_v228);
        self.scalar_v988 = v988;
        let v1011: f64 = (self.scalar_v108 * 1e-5);
        self.scalar_v1011 = v1011;
        let v1015: f64 = (self.scalar_v108 * 1e-40);
        self.scalar_v1015 = v1015;
        let v1042: f64 = (self.scalar_v185 * self.scalar_v1041);
        self.scalar_v1042 = v1042;
        let v1043: f64 = (0.1 * self.scalar_v185);
        self.scalar_v1043 = v1043;
        let v1065: f64 = (self.scalar_v185 / self.scalar_v1063);
        self.scalar_v1065 = v1065;
        let v1082: f64 = (2.0 - self.scalar_v273);
        self.scalar_v1082 = v1082;
        let v1083: f64 = (1.0 - self.scalar_v273);
        self.scalar_v1083 = v1083;
        let v1084: f64 = (self.scalar_v1082 / self.scalar_v1083);
        self.scalar_v1084 = v1084;
        let v1086: f64 = f64::powf(self.scalar_v1084, self.scalar_v1085);
        self.scalar_v1086 = v1086;
        let v1087: f64 = (1.0 - self.scalar_v1086);
        self.scalar_v1087 = v1087;
        let v1088: f64 = (self.scalar_v228 * self.scalar_v1087);
        self.scalar_v1088 = v1088;
        let v1109: f64 = (self.scalar_v228 / self.scalar_v1108);
        self.scalar_v1109 = v1109;
        let v1123: f64 = (4.0 * self.scalar_v408);
        self.scalar_v1123 = v1123;
        let v1124: f64 = (self.scalar_v1123 / self.scalar_v413);
        self.scalar_v1124 = v1124;
        let v1130: f64 = (1.0 / self.scalar_v374);
        self.scalar_v1130 = v1130;
        let v1155: f64 = (self.scalar_v110 * self.scalar_v629);
        self.scalar_v1155 = v1155;
        let v1156: f64 = ((self.scalar_v1155) as f64).exp();
        self.scalar_v1156 = v1156;
        let v1157: f64 = (self.scalar_v1156 - 1.0);
        self.scalar_v1157 = v1157;
        let v1178: f64 = (self.scalar_v408 * self.scalar_v1177);
        self.scalar_v1178 = v1178;
        let v1276: f64 = (2.0 * self.scalar_v477);
        self.scalar_v1276 = v1276;
        let v1335: f64 = (2.0 * self.scalar_v492);
        self.scalar_v1335 = v1335;
        let v1475: f64 = (2.0 * self.scalar_v552);
        self.scalar_v1475 = v1475;
        let v1564: f64 = (2.0 * self.scalar_v574);
        self.scalar_v1564 = v1564;
        let v1573: f64 = (2.0 * self.scalar_v503);
        self.scalar_v1573 = v1573;
        let v1576: f64 = (4.0 * self.scalar_v503);
        self.scalar_v1576 = v1576;
        let v1577: f64 = (self.scalar_v1576 / self.scalar_v419);
        self.scalar_v1577 = v1577;
        let v1587: f64 = (self.scalar_v592 * self.scalar_v1586);
        self.scalar_v1587 = v1587;
        let v1590: f64 = (self.scalar_v592 / self.scalar_v605);
        self.scalar_v1590 = v1590;
        let v1591: f64 = (4.0 * self.scalar_v1590);
        self.scalar_v1591 = v1591;
        let v1603: f64 = (self.scalar_v592 * self.scalar_v1602);
        self.scalar_v1603 = v1603;
        let v1630: f64 = (2.0 * self.scalar_v600);
        self.scalar_v1630 = v1630;
        let v1634: f64 = (self.scalar_v600 / self.scalar_v610);
        self.scalar_v1634 = v1634;
        let v1635: f64 = (self.scalar_v1633 * self.scalar_v1634);
        self.scalar_v1635 = v1635;
        let v1652: f64 = (self.scalar_v503 * self.scalar_v1651);
        self.scalar_v1652 = v1652;
        let v1664: f64 = (self.scalar_v592 * self.scalar_v1663);
        self.scalar_v1664 = v1664;
        let v1667: f64 = (4.0 * self.scalar_v592);
        self.scalar_v1667 = v1667;
        let v1668: f64 = (self.scalar_v1667 / self.scalar_v605);
        self.scalar_v1668 = v1668;
        let v1687: f64 = (self.scalar_v503 + self.scalar_v592);
        self.scalar_v1687 = v1687;
        let v1688: f64 = (self.scalar_v13 * self.scalar_v1687);
        self.scalar_v1688 = v1688;
        let v1689: f64 = (self.scalar_v299 * self.scalar_v1688);
        self.scalar_v1689 = v1689;
        let v1690: f64 = (if self.scalar_v1686 { self.scalar_v1689 } else { 0.0 });
        self.scalar_v1690 = v1690;
        let v1691: f64 = (self.scalar_v110 * self.scalar_v1690);
        self.scalar_v1691 = v1691;
        let v1692: f64 = ((self.scalar_v1691) as f64).ln();
        self.scalar_v1692 = v1692;
        let v1693: f64 = (2.0 - self.scalar_v1692);
        self.scalar_v1693 = v1693;
        let v1694: f64 = (self.scalar_v108 * self.scalar_v1693);
        self.scalar_v1694 = v1694;
        let v1695: f64 = (if self.scalar_v1686 { self.scalar_v1694 } else { 0.0 });
        self.scalar_v1695 = v1695;
        let v1827: f64 = (-self.scalar_v393);
        self.scalar_v1827 = v1827;
        let v1843: f64 = (self.scalar_v1842 / self.scalar_v393);
        self.scalar_v1843 = v1843;
        let v1932: f64 = (self.scalar_v10 / self.scalar_v627);
        self.scalar_v1932 = v1932;
        let v1935: f64 = (-self.scalar_v627);
        self.scalar_v1935 = v1935;
        let v2132: f64 = (self.scalar_v0 * self.scalar_v110);
        self.scalar_v2132 = v2132;
        let v2133: f64 = (self.scalar_v110 * self.scalar_v2128);
        self.scalar_v2133 = v2133;
        let v2142: f64 = (self.scalar_v2133 / self.scalar_v343);
        self.scalar_v2142 = v2142;
        let v2143: f64 = (self.scalar_v2132 / self.scalar_v343);
        self.scalar_v2143 = v2143;
        let v2152: f64 = (self.scalar_v110 * self.scalar_v2129);
        self.scalar_v2152 = v2152;
        let v2153: f64 = (self.scalar_v110 * self.scalar_v2130);
        self.scalar_v2153 = v2153;
        let v2178: f64 = (self.scalar_v110 * self.scalar_v2131);
        self.scalar_v2178 = v2178;
        let v2788: f64 = (self.scalar_v2128 / self.scalar_v1043);
        self.scalar_v2788 = v2788;
        let v2789: f64 = (self.scalar_v0 / self.scalar_v1043);
        self.scalar_v2789 = v2789;
        let v2800: f64 = (-self.scalar_v2788);
        self.scalar_v2800 = v2800;
        let v2801: f64 = (-self.scalar_v2789);
        self.scalar_v2801 = v2801;
        let v2953: f64 = (self.scalar_v0 * self.scalar_v273);
        self.scalar_v2953 = v2953;
        let v2954: f64 = (self.scalar_v273 * self.scalar_v2128);
        self.scalar_v2954 = v2954;
        let v2971: f64 = (self.scalar_v1130 - 1.0);
        self.scalar_v2971 = v2971;
        let v3204: f64 = (self.scalar_v2133 / self.scalar_v450);
        self.scalar_v3204 = v3204;
        let v3205: f64 = (self.scalar_v2132 / self.scalar_v450);
        self.scalar_v3205 = v3205;
        let v3337: f64 = (self.scalar_v2133 / self.scalar_v461);
        self.scalar_v3337 = v3337;
        let v3338: f64 = (self.scalar_v2132 / self.scalar_v461);
        self.scalar_v3338 = v3338;
        let v3394: f64 = (self.scalar_v2133 / self.scalar_v422);
        self.scalar_v3394 = v3394;
        let v3395: f64 = (self.scalar_v2132 / self.scalar_v422);
        self.scalar_v3395 = v3395;
        let v3409: f64 = (self.scalar_v2133 / self.scalar_v505);
        self.scalar_v3409 = v3409;
        let v3410: f64 = (self.scalar_v2132 / self.scalar_v505);
        self.scalar_v3410 = v3410;
        let v3424: f64 = (self.scalar_v2132 / self.scalar_v435);
        self.scalar_v3424 = v3424;
        let v3425: f64 = (self.scalar_v2152 / self.scalar_v435);
        self.scalar_v3425 = v3425;
        let v3426: f64 = (self.scalar_v2153 / self.scalar_v435);
        self.scalar_v3426 = v3426;
        let v3427: f64 = (self.scalar_v2133 / self.scalar_v435);
        self.scalar_v3427 = v3427;
        let v3451: f64 = (self.scalar_v2133 / self.scalar_v515);
        self.scalar_v3451 = v3451;
        let v3452: f64 = (self.scalar_v2132 / self.scalar_v515);
        self.scalar_v3452 = v3452;
        let v3493: f64 = (self.scalar_v260 * self.scalar_v2128);
        self.scalar_v3493 = v3493;
        let v3494: f64 = (self.scalar_v0 * self.scalar_v260);
        self.scalar_v3494 = v3494;
        let v3545: f64 = (self.scalar_v542 * self.scalar_v3543);
        self.scalar_v3545 = v3545;
        let v3546: f64 = (self.scalar_v542 * self.scalar_v3544);
        self.scalar_v3546 = v3546;
        let v3635: f64 = (self.scalar_v0 * self.scalar_v261);
        self.scalar_v3635 = v3635;
        let v3636: f64 = (self.scalar_v261 * self.scalar_v2128);
        self.scalar_v3636 = v3636;
        let v3637: f64 = (-self.scalar_v3635);
        self.scalar_v3637 = v3637;
        let v3638: f64 = (-self.scalar_v3636);
        self.scalar_v3638 = v3638;
        let v3716: f64 = (self.scalar_v564 * self.scalar_v3714);
        self.scalar_v3716 = v3716;
        let v3717: f64 = (self.scalar_v564 * self.scalar_v3715);
        self.scalar_v3717 = v3717;
        let v5317: f64 = (self.scalar_v1843 * self.scalar_v2128);
        self.scalar_v5317 = v5317;
        let v5318: f64 = (self.scalar_v0 * self.scalar_v1843);
        self.scalar_v5318 = v5318;
        let v5672: f64 = (self.scalar_v5670 / self.scalar_v280);
        self.scalar_v5672 = v5672;
        let v5673: f64 = (self.scalar_v5671 / self.scalar_v280);
        self.scalar_v5673 = v5673;
        let v5674: f64 = (self.scalar_v27 * self.scalar_v5672);
        self.scalar_v5674 = v5674;
        let v5675: f64 = (self.scalar_v27 * self.scalar_v5673);
        self.scalar_v5675 = v5675;
        let v5676: f64 = (self.scalar_v5670 / self.scalar_v294);
        self.scalar_v5676 = v5676;
        let v5677: f64 = (self.scalar_v5671 / self.scalar_v294);
        self.scalar_v5677 = v5677;
        let v5678: f64 = (self.scalar_v27 * self.scalar_v5676);
        self.scalar_v5678 = v5678;
        let v5679: f64 = (self.scalar_v27 * self.scalar_v5677);
        self.scalar_v5679 = v5679;
        let v5702: f64 = (self.scalar_v637 * self.scalar_v5670);
        self.scalar_v5702 = v5702;
        let v5703: f64 = (self.scalar_v637 * self.scalar_v5700);
        self.scalar_v5703 = v5703;
        let v5704: f64 = (self.scalar_v637 * self.scalar_v5701);
        self.scalar_v5704 = v5704;
        let v5705: f64 = (self.scalar_v637 * self.scalar_v5671);
        self.scalar_v5705 = v5705;
        let v5706: f64 = (self.scalar_v27 * self.scalar_v5702);
        self.scalar_v5706 = v5706;
        let v5707: f64 = (self.scalar_v27 * self.scalar_v5703);
        self.scalar_v5707 = v5707;
        let v5708: f64 = (self.scalar_v27 * self.scalar_v5704);
        self.scalar_v5708 = v5708;
        let v5709: f64 = (self.scalar_v27 * self.scalar_v5705);
        self.scalar_v5709 = v5709;
        let v5737: f64 = (self.scalar_v645 * self.scalar_v5670);
        self.scalar_v5737 = v5737;
        let v5738: f64 = (self.scalar_v645 * self.scalar_v5671);
        self.scalar_v5738 = v5738;
        let v5739: f64 = (self.scalar_v27 * self.scalar_v5737);
        self.scalar_v5739 = v5739;
        let v5740: f64 = (self.scalar_v27 * self.scalar_v5738);
        self.scalar_v5740 = v5740;
        let v5741: f64 = (if self.scalar_v638 { self.scalar_v5739 } else { 0.0 });
        self.scalar_v5741 = v5741;
        let v5742: f64 = (if self.scalar_v638 { self.scalar_v5740 } else { 0.0 });
        self.scalar_v5742 = v5742;
        let v5743: f64 = (self.scalar_v653 * self.scalar_v5671);
        self.scalar_v5743 = v5743;
        let v5744: f64 = (self.scalar_v653 * self.scalar_v5670);
        self.scalar_v5744 = v5744;
        let v5745: f64 = (self.scalar_v27 * self.scalar_v5743);
        self.scalar_v5745 = v5745;
        let v5746: f64 = (self.scalar_v27 * self.scalar_v5744);
        self.scalar_v5746 = v5746;
        let v5747: f64 = (if self.scalar_v646 { self.scalar_v5745 } else { 0.0 });
        self.scalar_v5747 = v5747;
        let v5748: f64 = (if self.scalar_v646 { self.scalar_v5746 } else { 0.0 });
        self.scalar_v5748 = v5748;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
