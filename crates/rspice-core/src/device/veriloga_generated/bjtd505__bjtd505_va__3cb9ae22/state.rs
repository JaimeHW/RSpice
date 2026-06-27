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
            params.p8 = 2.2e-17;
            params.p9 = 1.0;
            params.p10 = 1.0;
            params.p11 = 0.1;
            params.p12 = 2.5;
            params.p13 = 44.0;
            params.p14 = 1.0;
            params.p15 = 1.0000000000000001e-19;
            params.p16 = 1.0;
            params.p17 = 0.0;
            params.p18 = 1.0;
            params.p19 = 2.7000000000000005e-15;
            params.p20 = 2.0;
            params.p21 = 0.0;
            params.p22 = 2.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.68;
            params.p27 = 0.0;
            params.p28 = 3.1400000000000002e-18;
            params.p29 = 0.014289999999999999;
            params.p30 = 1e-15;
            params.p31 = 2.0;
            params.p32 = 0.63;
            params.p33 = 0.0;
            params.p34 = 22.0;
            params.p35 = 0.0;
            params.p36 = 22.0;
            params.p37 = 1e-6;
            params.p38 = 1.0;
            params.p39 = 400.0;
            params.p40 = -0.37;
            params.p41 = 0.5;
            params.p42 = 25.0;
            params.p43 = 0.1;
            params.p44 = 1.1e-6;
            params.p45 = 3.0;
            params.p46 = 0.3;
            params.p47 = 0.004;
            params.p48 = -0.37;
            params.p49 = -0.37;
            params.p50 = 0.3;
            params.p51 = 0.004;
            params.p52 = 1.0;
            params.p53 = 5.0;
            params.p54 = 23.0;
            params.p55 = 18.0;
            params.p56 = 12.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 150.0;
            params.p60 = 1250.0;
            params.p61 = 0.004;
            params.p62 = 0.3;
            params.p63 = 0.68;
            params.p64 = 7.3e-14;
            params.p65 = 0.95;
            params.p66 = 0.4;
            params.p67 = 0.4;
            params.p68 = 0.0;
            params.p69 = 7.800000000000001e-14;
            params.p70 = 0.68;
            params.p71 = 0.5;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.35;
            params.p75 = 0.5;
            params.p76 = 0.032;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.68;
            params.p80 = 100.0;
            params.p81 = 4.0;
            params.p82 = 1000.0;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 2e-12;
            params.p86 = 4.2e-12;
            params.p87 = 4.1e-11;
            params.p88 = 5.2e-10;
            params.p89 = 1e-11;
            params.p90 = 1.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.3333333333333333;
            params.p94 = 0.0;
            params.p95 = 0.3;
            params.p96 = 0.0;
            params.p97 = 1.0;
            params.p98 = 2.5;
            params.p99 = 2.5;
            params.p100 = 0.62;
            params.p101 = 2.0;
            params.p102 = 1.3;
            params.p103 = 2.0;
            params.p104 = 1.17;
            params.p105 = 1.12;
            params.p106 = 1.12;
            params.p107 = 1.12;
            params.p108 = 1.12;
            params.p109 = 1.18;
            params.p110 = 1.12;
            params.p111 = 1.125;
            params.p112 = 1.15;
            params.p113 = 1.15;
            params.p114 = 0.000473;
            params.p115 = 636.0;
            params.p116 = 1.15;
            params.p117 = 0.000473;
            params.p118 = 636.0;
            params.p119 = 0.05;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0005;
            params.p124 = 200.0;
            params.p125 = 2.0;
            params.p126 = 2.0;
            params.p127 = 2e-11;
            params.p128 = 2e-11;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 2.0;
            params.p134 = 400.0;
            params.p135 = 1e-40;
            params.p136 = 1e-40;
            params.p137 = 0.001;
            validate_parameter("minr", params.p137, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p138 = 0.0;
            params.p139 = 1.0;
            params.p140 = 0.0;
            params.p141 = 0.16;
            params.p142 = 0.0;
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
    pub nodes: [usize; 11],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 143]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 9]>,
    pub(crate) ddt_state_previous: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
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
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: bool,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: bool,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
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
    pub(crate) scalar_v81: bool,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: bool,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: bool,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v330: bool,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: bool,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v361: bool,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: bool,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v467: f64,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v565: f64,
    pub(crate) scalar_v568: f64,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v588: f64,
    pub(crate) scalar_v590: bool,
    pub(crate) scalar_v596: bool,
    pub(crate) scalar_v598: bool,
    pub(crate) scalar_v604: bool,
    pub(crate) scalar_v606: bool,
    pub(crate) scalar_v612: bool,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v660: f64,
    pub(crate) scalar_v748: f64,
    pub(crate) scalar_v802: f64,
    pub(crate) scalar_v803: f64,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v815: f64,
    pub(crate) scalar_v836: f64,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v838: f64,
    pub(crate) scalar_v839: f64,
    pub(crate) scalar_v840: f64,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v888: f64,
    pub(crate) scalar_v898: f64,
    pub(crate) scalar_v911: f64,
    pub(crate) scalar_v912: bool,
    pub(crate) scalar_v916: bool,
    pub(crate) scalar_v967: f64,
    pub(crate) scalar_v968: f64,
    pub(crate) scalar_v969: f64,
    pub(crate) scalar_v991: f64,
    pub(crate) scalar_v999: f64,
    pub(crate) scalar_v1000: bool,
    pub(crate) scalar_v1002: bool,
    pub(crate) scalar_v1003: bool,
    pub(crate) scalar_v1004: bool,
    pub(crate) scalar_v1007: bool,
    pub(crate) scalar_v1008: bool,
    pub(crate) scalar_v1013: f64,
    pub(crate) scalar_v1034: f64,
    pub(crate) scalar_v1036: f64,
    pub(crate) scalar_v1065: bool,
    pub(crate) scalar_v1071: bool,
    pub(crate) scalar_v1106: f64,
    pub(crate) scalar_v1130: f64,
    pub(crate) scalar_v1143: f64,
    pub(crate) scalar_v1161: f64,
    pub(crate) scalar_v1224: f64,
    pub(crate) scalar_v1225: bool,
    pub(crate) scalar_v1226: bool,
    pub(crate) scalar_v1227: bool,
    pub(crate) scalar_v1229: bool,
    pub(crate) scalar_v1230: bool,
    pub(crate) scalar_v1231: f64,
    pub(crate) scalar_v1324: bool,
    pub(crate) scalar_v1325: bool,
    pub(crate) scalar_v1326: bool,
    pub(crate) scalar_v1350: f64,
    pub(crate) scalar_v1352: f64,
    pub(crate) scalar_v1353: f64,
    pub(crate) scalar_v1355: f64,
    pub(crate) scalar_v1415: bool,
    pub(crate) scalar_v1416: bool,
    pub(crate) scalar_v1417: bool,
    pub(crate) scalar_v1443: f64,
    pub(crate) scalar_v1445: f64,
    pub(crate) scalar_v1446: f64,
    pub(crate) scalar_v1448: f64,
    pub(crate) scalar_v1514: f64,
    pub(crate) scalar_v1515: bool,
    pub(crate) scalar_v1516: bool,
    pub(crate) scalar_v1517: bool,
    pub(crate) scalar_v1520: f64,
    pub(crate) scalar_v1530: f64,
    pub(crate) scalar_v1531: bool,
    pub(crate) scalar_v1532: bool,
    pub(crate) scalar_v1544: f64,
    pub(crate) scalar_v1549: f64,
    pub(crate) scalar_v1566: bool,
    pub(crate) scalar_v1567: bool,
    pub(crate) scalar_v1571: f64,
    pub(crate) scalar_v1572: bool,
    pub(crate) scalar_v1575: f64,
    pub(crate) scalar_v1582: f64,
    pub(crate) scalar_v1593: f64,
    pub(crate) scalar_v1594: f64,
    pub(crate) scalar_v1595: f64,
    pub(crate) scalar_v1596: f64,
    pub(crate) scalar_v1597: f64,
    pub(crate) scalar_v1598: f64,
    pub(crate) scalar_v1599: f64,
    pub(crate) scalar_v1600: f64,
    pub(crate) scalar_v1601: f64,
    pub(crate) scalar_v1602: f64,
    pub(crate) scalar_v1603: f64,
    pub(crate) scalar_v1604: f64,
    pub(crate) scalar_v1605: f64,
    pub(crate) scalar_v1606: f64,
    pub(crate) scalar_v1607: f64,
    pub(crate) scalar_v1621: bool,
    pub(crate) scalar_v1648: f64,
    pub(crate) scalar_v1649: bool,
    pub(crate) scalar_v1650: f64,
    pub(crate) scalar_v1653: f64,
    pub(crate) scalar_v1672: f64,
    pub(crate) scalar_v1686: f64,
    pub(crate) scalar_v1691: bool,
    pub(crate) scalar_v1693: bool,
    pub(crate) scalar_v1697: f64,
    pub(crate) scalar_v1698: f64,
    pub(crate) scalar_v1699: f64,
    pub(crate) scalar_v1700: f64,
    pub(crate) scalar_v1701: f64,
    pub(crate) scalar_v1710: f64,
    pub(crate) scalar_v1711: bool,
    pub(crate) scalar_v1714: bool,
    pub(crate) scalar_v1737: f64,
    pub(crate) scalar_v1738: f64,
    pub(crate) scalar_v1744: f64,
    pub(crate) scalar_v1745: f64,
    pub(crate) scalar_v1746: f64,
    pub(crate) scalar_v1794: bool,
    pub(crate) scalar_v1795: bool,
    pub(crate) scalar_v1800: f64,
    pub(crate) scalar_v1804: f64,
    pub(crate) scalar_v1811: f64,
    pub(crate) scalar_v1816: f64,
    pub(crate) scalar_v1836: f64,
    pub(crate) scalar_v1856: f64,
    pub(crate) scalar_v1857: bool,
    pub(crate) scalar_v1892: bool,
    pub(crate) scalar_v1898: bool,
    pub(crate) scalar_v1903: f64,
    pub(crate) scalar_v1904: bool,
    pub(crate) scalar_v1908: bool,
    pub(crate) scalar_v1964: f64,
    pub(crate) scalar_v1965: f64,
    pub(crate) scalar_v1966: f64,
    pub(crate) scalar_v1967: f64,
    pub(crate) scalar_v1968: f64,
    pub(crate) scalar_v1969: f64,
    pub(crate) scalar_v1970: f64,
    pub(crate) scalar_v1971: f64,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v2650: f64,
    pub(crate) scalar_v2665: f64,
    pub(crate) scalar_v2666: f64,
    pub(crate) scalar_v2733: f64,
    pub(crate) scalar_v2745: f64,
    pub(crate) scalar_v2970: f64,
    pub(crate) scalar_v2971: f64,
    pub(crate) scalar_v2980: f64,
    pub(crate) scalar_v2981: f64,
    pub(crate) scalar_v3004: f64,
    pub(crate) scalar_v3005: f64,
    pub(crate) scalar_v3016: f64,
    pub(crate) scalar_v3017: f64,
    pub(crate) scalar_v3353: f64,
    pub(crate) scalar_v3392: f64,
    pub(crate) scalar_v3393: f64,
    pub(crate) scalar_v3436: f64,
    pub(crate) scalar_v3437: f64,
    pub(crate) scalar_v3526: f64,
    pub(crate) scalar_v3565: f64,
    pub(crate) scalar_v3566: f64,
    pub(crate) scalar_v3752: f64,
    pub(crate) scalar_v3753: f64,
    pub(crate) scalar_v3754: f64,
    pub(crate) scalar_v3755: f64,
    pub(crate) scalar_v3756: f64,
    pub(crate) scalar_v3757: f64,
    pub(crate) scalar_v3967: f64,
    pub(crate) scalar_v3968: f64,
    pub(crate) scalar_v3969: f64,
    pub(crate) scalar_v3970: f64,
    pub(crate) scalar_v3971: f64,
    pub(crate) scalar_v3972: f64,
    pub(crate) scalar_v3973: f64,
    pub(crate) scalar_v3974: f64,
    pub(crate) scalar_v3975: f64,
    pub(crate) scalar_v4367: f64,
    pub(crate) scalar_v4839: f64,
    pub(crate) scalar_v4918: f64,
    pub(crate) scalar_v5154: f64,
    pub(crate) scalar_v5155: f64,
    pub(crate) scalar_v5156: f64,
    pub(crate) scalar_v5157: f64,
    pub(crate) scalar_v5158: f64,
    pub(crate) scalar_v5159: f64,
    pub(crate) scalar_v5276: f64,
    pub(crate) scalar_v5277: f64,
    pub(crate) scalar_v5304: f64,
    pub(crate) scalar_v5305: f64,
    pub(crate) scalar_v5306: f64,
    pub(crate) scalar_v5307: f64,
    pub(crate) scalar_v5308: f64,
    pub(crate) scalar_v5309: f64,
    pub(crate) scratch: Option<Box<GenericScratch<571, 11, 2>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<571, 11, 2>>>,
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
            scalar_v45: self.scalar_v45,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
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
            scalar_v105: self.scalar_v105,
            scalar_v107: self.scalar_v107,
            scalar_v162: self.scalar_v162,
            scalar_v182: self.scalar_v182,
            scalar_v185: self.scalar_v185,
            scalar_v225: self.scalar_v225,
            scalar_v228: self.scalar_v228,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v293: self.scalar_v293,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v330: self.scalar_v330,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v361: self.scalar_v361,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v382: self.scalar_v382,
            scalar_v384: self.scalar_v384,
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v392: self.scalar_v392,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v408: self.scalar_v408,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v429: self.scalar_v429,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v438: self.scalar_v438,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v468: self.scalar_v468,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v492: self.scalar_v492,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v504: self.scalar_v504,
            scalar_v511: self.scalar_v511,
            scalar_v514: self.scalar_v514,
            scalar_v522: self.scalar_v522,
            scalar_v531: self.scalar_v531,
            scalar_v544: self.scalar_v544,
            scalar_v553: self.scalar_v553,
            scalar_v565: self.scalar_v565,
            scalar_v568: self.scalar_v568,
            scalar_v586: self.scalar_v586,
            scalar_v588: self.scalar_v588,
            scalar_v590: self.scalar_v590,
            scalar_v596: self.scalar_v596,
            scalar_v598: self.scalar_v598,
            scalar_v604: self.scalar_v604,
            scalar_v606: self.scalar_v606,
            scalar_v612: self.scalar_v612,
            scalar_v655: self.scalar_v655,
            scalar_v660: self.scalar_v660,
            scalar_v748: self.scalar_v748,
            scalar_v802: self.scalar_v802,
            scalar_v803: self.scalar_v803,
            scalar_v804: self.scalar_v804,
            scalar_v815: self.scalar_v815,
            scalar_v836: self.scalar_v836,
            scalar_v837: self.scalar_v837,
            scalar_v838: self.scalar_v838,
            scalar_v839: self.scalar_v839,
            scalar_v840: self.scalar_v840,
            scalar_v841: self.scalar_v841,
            scalar_v888: self.scalar_v888,
            scalar_v898: self.scalar_v898,
            scalar_v911: self.scalar_v911,
            scalar_v912: self.scalar_v912,
            scalar_v916: self.scalar_v916,
            scalar_v967: self.scalar_v967,
            scalar_v968: self.scalar_v968,
            scalar_v969: self.scalar_v969,
            scalar_v991: self.scalar_v991,
            scalar_v999: self.scalar_v999,
            scalar_v1000: self.scalar_v1000,
            scalar_v1002: self.scalar_v1002,
            scalar_v1003: self.scalar_v1003,
            scalar_v1004: self.scalar_v1004,
            scalar_v1007: self.scalar_v1007,
            scalar_v1008: self.scalar_v1008,
            scalar_v1013: self.scalar_v1013,
            scalar_v1034: self.scalar_v1034,
            scalar_v1036: self.scalar_v1036,
            scalar_v1065: self.scalar_v1065,
            scalar_v1071: self.scalar_v1071,
            scalar_v1106: self.scalar_v1106,
            scalar_v1130: self.scalar_v1130,
            scalar_v1143: self.scalar_v1143,
            scalar_v1161: self.scalar_v1161,
            scalar_v1224: self.scalar_v1224,
            scalar_v1225: self.scalar_v1225,
            scalar_v1226: self.scalar_v1226,
            scalar_v1227: self.scalar_v1227,
            scalar_v1229: self.scalar_v1229,
            scalar_v1230: self.scalar_v1230,
            scalar_v1231: self.scalar_v1231,
            scalar_v1324: self.scalar_v1324,
            scalar_v1325: self.scalar_v1325,
            scalar_v1326: self.scalar_v1326,
            scalar_v1350: self.scalar_v1350,
            scalar_v1352: self.scalar_v1352,
            scalar_v1353: self.scalar_v1353,
            scalar_v1355: self.scalar_v1355,
            scalar_v1415: self.scalar_v1415,
            scalar_v1416: self.scalar_v1416,
            scalar_v1417: self.scalar_v1417,
            scalar_v1443: self.scalar_v1443,
            scalar_v1445: self.scalar_v1445,
            scalar_v1446: self.scalar_v1446,
            scalar_v1448: self.scalar_v1448,
            scalar_v1514: self.scalar_v1514,
            scalar_v1515: self.scalar_v1515,
            scalar_v1516: self.scalar_v1516,
            scalar_v1517: self.scalar_v1517,
            scalar_v1520: self.scalar_v1520,
            scalar_v1530: self.scalar_v1530,
            scalar_v1531: self.scalar_v1531,
            scalar_v1532: self.scalar_v1532,
            scalar_v1544: self.scalar_v1544,
            scalar_v1549: self.scalar_v1549,
            scalar_v1566: self.scalar_v1566,
            scalar_v1567: self.scalar_v1567,
            scalar_v1571: self.scalar_v1571,
            scalar_v1572: self.scalar_v1572,
            scalar_v1575: self.scalar_v1575,
            scalar_v1582: self.scalar_v1582,
            scalar_v1593: self.scalar_v1593,
            scalar_v1594: self.scalar_v1594,
            scalar_v1595: self.scalar_v1595,
            scalar_v1596: self.scalar_v1596,
            scalar_v1597: self.scalar_v1597,
            scalar_v1598: self.scalar_v1598,
            scalar_v1599: self.scalar_v1599,
            scalar_v1600: self.scalar_v1600,
            scalar_v1601: self.scalar_v1601,
            scalar_v1602: self.scalar_v1602,
            scalar_v1603: self.scalar_v1603,
            scalar_v1604: self.scalar_v1604,
            scalar_v1605: self.scalar_v1605,
            scalar_v1606: self.scalar_v1606,
            scalar_v1607: self.scalar_v1607,
            scalar_v1621: self.scalar_v1621,
            scalar_v1648: self.scalar_v1648,
            scalar_v1649: self.scalar_v1649,
            scalar_v1650: self.scalar_v1650,
            scalar_v1653: self.scalar_v1653,
            scalar_v1672: self.scalar_v1672,
            scalar_v1686: self.scalar_v1686,
            scalar_v1691: self.scalar_v1691,
            scalar_v1693: self.scalar_v1693,
            scalar_v1697: self.scalar_v1697,
            scalar_v1698: self.scalar_v1698,
            scalar_v1699: self.scalar_v1699,
            scalar_v1700: self.scalar_v1700,
            scalar_v1701: self.scalar_v1701,
            scalar_v1710: self.scalar_v1710,
            scalar_v1711: self.scalar_v1711,
            scalar_v1714: self.scalar_v1714,
            scalar_v1737: self.scalar_v1737,
            scalar_v1738: self.scalar_v1738,
            scalar_v1744: self.scalar_v1744,
            scalar_v1745: self.scalar_v1745,
            scalar_v1746: self.scalar_v1746,
            scalar_v1794: self.scalar_v1794,
            scalar_v1795: self.scalar_v1795,
            scalar_v1800: self.scalar_v1800,
            scalar_v1804: self.scalar_v1804,
            scalar_v1811: self.scalar_v1811,
            scalar_v1816: self.scalar_v1816,
            scalar_v1836: self.scalar_v1836,
            scalar_v1856: self.scalar_v1856,
            scalar_v1857: self.scalar_v1857,
            scalar_v1892: self.scalar_v1892,
            scalar_v1898: self.scalar_v1898,
            scalar_v1903: self.scalar_v1903,
            scalar_v1904: self.scalar_v1904,
            scalar_v1908: self.scalar_v1908,
            scalar_v1964: self.scalar_v1964,
            scalar_v1965: self.scalar_v1965,
            scalar_v1966: self.scalar_v1966,
            scalar_v1967: self.scalar_v1967,
            scalar_v1968: self.scalar_v1968,
            scalar_v1969: self.scalar_v1969,
            scalar_v1970: self.scalar_v1970,
            scalar_v1971: self.scalar_v1971,
            scalar_v1972: self.scalar_v1972,
            scalar_v2650: self.scalar_v2650,
            scalar_v2665: self.scalar_v2665,
            scalar_v2666: self.scalar_v2666,
            scalar_v2733: self.scalar_v2733,
            scalar_v2745: self.scalar_v2745,
            scalar_v2970: self.scalar_v2970,
            scalar_v2971: self.scalar_v2971,
            scalar_v2980: self.scalar_v2980,
            scalar_v2981: self.scalar_v2981,
            scalar_v3004: self.scalar_v3004,
            scalar_v3005: self.scalar_v3005,
            scalar_v3016: self.scalar_v3016,
            scalar_v3017: self.scalar_v3017,
            scalar_v3353: self.scalar_v3353,
            scalar_v3392: self.scalar_v3392,
            scalar_v3393: self.scalar_v3393,
            scalar_v3436: self.scalar_v3436,
            scalar_v3437: self.scalar_v3437,
            scalar_v3526: self.scalar_v3526,
            scalar_v3565: self.scalar_v3565,
            scalar_v3566: self.scalar_v3566,
            scalar_v3752: self.scalar_v3752,
            scalar_v3753: self.scalar_v3753,
            scalar_v3754: self.scalar_v3754,
            scalar_v3755: self.scalar_v3755,
            scalar_v3756: self.scalar_v3756,
            scalar_v3757: self.scalar_v3757,
            scalar_v3967: self.scalar_v3967,
            scalar_v3968: self.scalar_v3968,
            scalar_v3969: self.scalar_v3969,
            scalar_v3970: self.scalar_v3970,
            scalar_v3971: self.scalar_v3971,
            scalar_v3972: self.scalar_v3972,
            scalar_v3973: self.scalar_v3973,
            scalar_v3974: self.scalar_v3974,
            scalar_v3975: self.scalar_v3975,
            scalar_v4367: self.scalar_v4367,
            scalar_v4839: self.scalar_v4839,
            scalar_v4918: self.scalar_v4918,
            scalar_v5154: self.scalar_v5154,
            scalar_v5155: self.scalar_v5155,
            scalar_v5156: self.scalar_v5156,
            scalar_v5157: self.scalar_v5157,
            scalar_v5158: self.scalar_v5158,
            scalar_v5159: self.scalar_v5159,
            scalar_v5276: self.scalar_v5276,
            scalar_v5277: self.scalar_v5277,
            scalar_v5304: self.scalar_v5304,
            scalar_v5305: self.scalar_v5305,
            scalar_v5306: self.scalar_v5306,
            scalar_v5307: self.scalar_v5307,
            scalar_v5308: self.scalar_v5308,
            scalar_v5309: self.scalar_v5309,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 11;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 143;
    pub const VARIABLE_COUNT: usize = 571;
    pub const DDT_STATE_COUNT: usize = 9;
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
            scalar_v32: 0.0,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v35: 0.0,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v40: 0.0,
            scalar_v41: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v45: 0.0,
            scalar_v47: 0.0,
            scalar_v48: false,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: false,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
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
            scalar_v81: false,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: false,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v105: 0.0,
            scalar_v107: 0.0,
            scalar_v162: 0.0,
            scalar_v182: 0.0,
            scalar_v185: 0.0,
            scalar_v225: 0.0,
            scalar_v228: 0.0,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v276: 0.0,
            scalar_v277: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v293: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v300: 0.0,
            scalar_v301: false,
            scalar_v302: 0.0,
            scalar_v330: false,
            scalar_v332: 0.0,
            scalar_v333: false,
            scalar_v334: 0.0,
            scalar_v361: false,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v382: 0.0,
            scalar_v384: 0.0,
            scalar_v385: 0.0,
            scalar_v386: 0.0,
            scalar_v387: 0.0,
            scalar_v392: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v408: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v422: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v429: 0.0,
            scalar_v434: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v438: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v456: 0.0,
            scalar_v457: false,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v466: 0.0,
            scalar_v467: 0.0,
            scalar_v468: 0.0,
            scalar_v473: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v481: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v492: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v504: 0.0,
            scalar_v511: 0.0,
            scalar_v514: 0.0,
            scalar_v522: 0.0,
            scalar_v531: 0.0,
            scalar_v544: 0.0,
            scalar_v553: 0.0,
            scalar_v565: 0.0,
            scalar_v568: 0.0,
            scalar_v586: 0.0,
            scalar_v588: 0.0,
            scalar_v590: false,
            scalar_v596: false,
            scalar_v598: false,
            scalar_v604: false,
            scalar_v606: false,
            scalar_v612: false,
            scalar_v655: 0.0,
            scalar_v660: 0.0,
            scalar_v748: 0.0,
            scalar_v802: 0.0,
            scalar_v803: 0.0,
            scalar_v804: 0.0,
            scalar_v815: 0.0,
            scalar_v836: 0.0,
            scalar_v837: 0.0,
            scalar_v838: 0.0,
            scalar_v839: 0.0,
            scalar_v840: 0.0,
            scalar_v841: 0.0,
            scalar_v888: 0.0,
            scalar_v898: 0.0,
            scalar_v911: 0.0,
            scalar_v912: false,
            scalar_v916: false,
            scalar_v967: 0.0,
            scalar_v968: 0.0,
            scalar_v969: 0.0,
            scalar_v991: 0.0,
            scalar_v999: 0.0,
            scalar_v1000: false,
            scalar_v1002: false,
            scalar_v1003: false,
            scalar_v1004: false,
            scalar_v1007: false,
            scalar_v1008: false,
            scalar_v1013: 0.0,
            scalar_v1034: 0.0,
            scalar_v1036: 0.0,
            scalar_v1065: false,
            scalar_v1071: false,
            scalar_v1106: 0.0,
            scalar_v1130: 0.0,
            scalar_v1143: 0.0,
            scalar_v1161: 0.0,
            scalar_v1224: 0.0,
            scalar_v1225: false,
            scalar_v1226: false,
            scalar_v1227: false,
            scalar_v1229: false,
            scalar_v1230: false,
            scalar_v1231: 0.0,
            scalar_v1324: false,
            scalar_v1325: false,
            scalar_v1326: false,
            scalar_v1350: 0.0,
            scalar_v1352: 0.0,
            scalar_v1353: 0.0,
            scalar_v1355: 0.0,
            scalar_v1415: false,
            scalar_v1416: false,
            scalar_v1417: false,
            scalar_v1443: 0.0,
            scalar_v1445: 0.0,
            scalar_v1446: 0.0,
            scalar_v1448: 0.0,
            scalar_v1514: 0.0,
            scalar_v1515: false,
            scalar_v1516: false,
            scalar_v1517: false,
            scalar_v1520: 0.0,
            scalar_v1530: 0.0,
            scalar_v1531: false,
            scalar_v1532: false,
            scalar_v1544: 0.0,
            scalar_v1549: 0.0,
            scalar_v1566: false,
            scalar_v1567: false,
            scalar_v1571: 0.0,
            scalar_v1572: false,
            scalar_v1575: 0.0,
            scalar_v1582: 0.0,
            scalar_v1593: 0.0,
            scalar_v1594: 0.0,
            scalar_v1595: 0.0,
            scalar_v1596: 0.0,
            scalar_v1597: 0.0,
            scalar_v1598: 0.0,
            scalar_v1599: 0.0,
            scalar_v1600: 0.0,
            scalar_v1601: 0.0,
            scalar_v1602: 0.0,
            scalar_v1603: 0.0,
            scalar_v1604: 0.0,
            scalar_v1605: 0.0,
            scalar_v1606: 0.0,
            scalar_v1607: 0.0,
            scalar_v1621: false,
            scalar_v1648: 0.0,
            scalar_v1649: false,
            scalar_v1650: 0.0,
            scalar_v1653: 0.0,
            scalar_v1672: 0.0,
            scalar_v1686: 0.0,
            scalar_v1691: false,
            scalar_v1693: false,
            scalar_v1697: 0.0,
            scalar_v1698: 0.0,
            scalar_v1699: 0.0,
            scalar_v1700: 0.0,
            scalar_v1701: 0.0,
            scalar_v1710: 0.0,
            scalar_v1711: false,
            scalar_v1714: false,
            scalar_v1737: 0.0,
            scalar_v1738: 0.0,
            scalar_v1744: 0.0,
            scalar_v1745: 0.0,
            scalar_v1746: 0.0,
            scalar_v1794: false,
            scalar_v1795: false,
            scalar_v1800: 0.0,
            scalar_v1804: 0.0,
            scalar_v1811: 0.0,
            scalar_v1816: 0.0,
            scalar_v1836: 0.0,
            scalar_v1856: 0.0,
            scalar_v1857: false,
            scalar_v1892: false,
            scalar_v1898: false,
            scalar_v1903: 0.0,
            scalar_v1904: false,
            scalar_v1908: false,
            scalar_v1964: 0.0,
            scalar_v1965: 0.0,
            scalar_v1966: 0.0,
            scalar_v1967: 0.0,
            scalar_v1968: 0.0,
            scalar_v1969: 0.0,
            scalar_v1970: 0.0,
            scalar_v1971: 0.0,
            scalar_v1972: 0.0,
            scalar_v2650: 0.0,
            scalar_v2665: 0.0,
            scalar_v2666: 0.0,
            scalar_v2733: 0.0,
            scalar_v2745: 0.0,
            scalar_v2970: 0.0,
            scalar_v2971: 0.0,
            scalar_v2980: 0.0,
            scalar_v2981: 0.0,
            scalar_v3004: 0.0,
            scalar_v3005: 0.0,
            scalar_v3016: 0.0,
            scalar_v3017: 0.0,
            scalar_v3353: 0.0,
            scalar_v3392: 0.0,
            scalar_v3393: 0.0,
            scalar_v3436: 0.0,
            scalar_v3437: 0.0,
            scalar_v3526: 0.0,
            scalar_v3565: 0.0,
            scalar_v3566: 0.0,
            scalar_v3752: 0.0,
            scalar_v3753: 0.0,
            scalar_v3754: 0.0,
            scalar_v3755: 0.0,
            scalar_v3756: 0.0,
            scalar_v3757: 0.0,
            scalar_v3967: 0.0,
            scalar_v3968: 0.0,
            scalar_v3969: 0.0,
            scalar_v3970: 0.0,
            scalar_v3971: 0.0,
            scalar_v3972: 0.0,
            scalar_v3973: 0.0,
            scalar_v3974: 0.0,
            scalar_v3975: 0.0,
            scalar_v4367: 0.0,
            scalar_v4839: 0.0,
            scalar_v4918: 0.0,
            scalar_v5154: 0.0,
            scalar_v5155: 0.0,
            scalar_v5156: 0.0,
            scalar_v5157: 0.0,
            scalar_v5158: 0.0,
            scalar_v5159: 0.0,
            scalar_v5276: 0.0,
            scalar_v5277: 0.0,
            scalar_v5304: 0.0,
            scalar_v5305: 0.0,
            scalar_v5306: 0.0,
            scalar_v5307: 0.0,
            scalar_v5308: 0.0,
            scalar_v5309: 0.0,
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
            scalar_v45,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
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
            scalar_v105,
            scalar_v107,
            scalar_v162,
            scalar_v182,
            scalar_v185,
            scalar_v225,
            scalar_v228,
            scalar_v254,
            scalar_v255,
            scalar_v262,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v289,
            scalar_v293,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v330,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v361,
            scalar_v363,
            scalar_v364,
            scalar_v382,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v392,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v416,
            scalar_v417,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v438,
            scalar_v442,
            scalar_v443,
            scalar_v448,
            scalar_v449,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v487,
            scalar_v488,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v511,
            scalar_v514,
            scalar_v522,
            scalar_v531,
            scalar_v544,
            scalar_v553,
            scalar_v565,
            scalar_v568,
            scalar_v586,
            scalar_v588,
            scalar_v590,
            scalar_v596,
            scalar_v598,
            scalar_v604,
            scalar_v606,
            scalar_v612,
            scalar_v655,
            scalar_v660,
            scalar_v748,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v815,
            scalar_v836,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v841,
            scalar_v888,
            scalar_v898,
            scalar_v911,
            scalar_v912,
            scalar_v916,
            scalar_v967,
            scalar_v968,
            scalar_v969,
            scalar_v991,
            scalar_v999,
            scalar_v1000,
            scalar_v1002,
            scalar_v1003,
            scalar_v1004,
            scalar_v1007,
            scalar_v1008,
            scalar_v1013,
            scalar_v1034,
            scalar_v1036,
            scalar_v1065,
            scalar_v1071,
            scalar_v1106,
            scalar_v1130,
            scalar_v1143,
            scalar_v1161,
            scalar_v1224,
            scalar_v1225,
            scalar_v1226,
            scalar_v1227,
            scalar_v1229,
            scalar_v1230,
            scalar_v1231,
            scalar_v1324,
            scalar_v1325,
            scalar_v1326,
            scalar_v1350,
            scalar_v1352,
            scalar_v1353,
            scalar_v1355,
            scalar_v1415,
            scalar_v1416,
            scalar_v1417,
            scalar_v1443,
            scalar_v1445,
            scalar_v1446,
            scalar_v1448,
            scalar_v1514,
            scalar_v1515,
            scalar_v1516,
            scalar_v1517,
            scalar_v1520,
            scalar_v1530,
            scalar_v1531,
            scalar_v1532,
            scalar_v1544,
            scalar_v1549,
            scalar_v1566,
            scalar_v1567,
            scalar_v1571,
            scalar_v1572,
            scalar_v1575,
            scalar_v1582,
            scalar_v1593,
            scalar_v1594,
            scalar_v1595,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1600,
            scalar_v1601,
            scalar_v1602,
            scalar_v1603,
            scalar_v1604,
            scalar_v1605,
            scalar_v1606,
            scalar_v1607,
            scalar_v1621,
            scalar_v1648,
            scalar_v1649,
            scalar_v1650,
            scalar_v1653,
            scalar_v1672,
            scalar_v1686,
            scalar_v1691,
            scalar_v1693,
            scalar_v1697,
            scalar_v1698,
            scalar_v1699,
            scalar_v1700,
            scalar_v1701,
            scalar_v1710,
            scalar_v1711,
            scalar_v1714,
            scalar_v1737,
            scalar_v1738,
            scalar_v1744,
            scalar_v1745,
            scalar_v1746,
            scalar_v1794,
            scalar_v1795,
            scalar_v1800,
            scalar_v1804,
            scalar_v1811,
            scalar_v1816,
            scalar_v1836,
            scalar_v1856,
            scalar_v1857,
            scalar_v1892,
            scalar_v1898,
            scalar_v1903,
            scalar_v1904,
            scalar_v1908,
            scalar_v1964,
            scalar_v1965,
            scalar_v1966,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1972,
            scalar_v2650,
            scalar_v2665,
            scalar_v2666,
            scalar_v2733,
            scalar_v2745,
            scalar_v2970,
            scalar_v2971,
            scalar_v2980,
            scalar_v2981,
            scalar_v3004,
            scalar_v3005,
            scalar_v3016,
            scalar_v3017,
            scalar_v3353,
            scalar_v3392,
            scalar_v3393,
            scalar_v3436,
            scalar_v3437,
            scalar_v3526,
            scalar_v3565,
            scalar_v3566,
            scalar_v3752,
            scalar_v3753,
            scalar_v3754,
            scalar_v3755,
            scalar_v3756,
            scalar_v3757,
            scalar_v3967,
            scalar_v3968,
            scalar_v3969,
            scalar_v3970,
            scalar_v3971,
            scalar_v3972,
            scalar_v3973,
            scalar_v3974,
            scalar_v3975,
            scalar_v4367,
            scalar_v4839,
            scalar_v4918,
            scalar_v5154,
            scalar_v5155,
            scalar_v5156,
            scalar_v5157,
            scalar_v5158,
            scalar_v5159,
            scalar_v5276,
            scalar_v5277,
            scalar_v5304,
            scalar_v5305,
            scalar_v5306,
            scalar_v5307,
            scalar_v5308,
            scalar_v5309,
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
            scalar_v45,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
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
            scalar_v105,
            scalar_v107,
            scalar_v162,
            scalar_v182,
            scalar_v185,
            scalar_v225,
            scalar_v228,
            scalar_v254,
            scalar_v255,
            scalar_v262,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v289,
            scalar_v293,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v330,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v361,
            scalar_v363,
            scalar_v364,
            scalar_v382,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v392,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v416,
            scalar_v417,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v438,
            scalar_v442,
            scalar_v443,
            scalar_v448,
            scalar_v449,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v487,
            scalar_v488,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v511,
            scalar_v514,
            scalar_v522,
            scalar_v531,
            scalar_v544,
            scalar_v553,
            scalar_v565,
            scalar_v568,
            scalar_v586,
            scalar_v588,
            scalar_v590,
            scalar_v596,
            scalar_v598,
            scalar_v604,
            scalar_v606,
            scalar_v612,
            scalar_v655,
            scalar_v660,
            scalar_v748,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v815,
            scalar_v836,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v841,
            scalar_v888,
            scalar_v898,
            scalar_v911,
            scalar_v912,
            scalar_v916,
            scalar_v967,
            scalar_v968,
            scalar_v969,
            scalar_v991,
            scalar_v999,
            scalar_v1000,
            scalar_v1002,
            scalar_v1003,
            scalar_v1004,
            scalar_v1007,
            scalar_v1008,
            scalar_v1013,
            scalar_v1034,
            scalar_v1036,
            scalar_v1065,
            scalar_v1071,
            scalar_v1106,
            scalar_v1130,
            scalar_v1143,
            scalar_v1161,
            scalar_v1224,
            scalar_v1225,
            scalar_v1226,
            scalar_v1227,
            scalar_v1229,
            scalar_v1230,
            scalar_v1231,
            scalar_v1324,
            scalar_v1325,
            scalar_v1326,
            scalar_v1350,
            scalar_v1352,
            scalar_v1353,
            scalar_v1355,
            scalar_v1415,
            scalar_v1416,
            scalar_v1417,
            scalar_v1443,
            scalar_v1445,
            scalar_v1446,
            scalar_v1448,
            scalar_v1514,
            scalar_v1515,
            scalar_v1516,
            scalar_v1517,
            scalar_v1520,
            scalar_v1530,
            scalar_v1531,
            scalar_v1532,
            scalar_v1544,
            scalar_v1549,
            scalar_v1566,
            scalar_v1567,
            scalar_v1571,
            scalar_v1572,
            scalar_v1575,
            scalar_v1582,
            scalar_v1593,
            scalar_v1594,
            scalar_v1595,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1600,
            scalar_v1601,
            scalar_v1602,
            scalar_v1603,
            scalar_v1604,
            scalar_v1605,
            scalar_v1606,
            scalar_v1607,
            scalar_v1621,
            scalar_v1648,
            scalar_v1649,
            scalar_v1650,
            scalar_v1653,
            scalar_v1672,
            scalar_v1686,
            scalar_v1691,
            scalar_v1693,
            scalar_v1697,
            scalar_v1698,
            scalar_v1699,
            scalar_v1700,
            scalar_v1701,
            scalar_v1710,
            scalar_v1711,
            scalar_v1714,
            scalar_v1737,
            scalar_v1738,
            scalar_v1744,
            scalar_v1745,
            scalar_v1746,
            scalar_v1794,
            scalar_v1795,
            scalar_v1800,
            scalar_v1804,
            scalar_v1811,
            scalar_v1816,
            scalar_v1836,
            scalar_v1856,
            scalar_v1857,
            scalar_v1892,
            scalar_v1898,
            scalar_v1903,
            scalar_v1904,
            scalar_v1908,
            scalar_v1964,
            scalar_v1965,
            scalar_v1966,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1972,
            scalar_v2650,
            scalar_v2665,
            scalar_v2666,
            scalar_v2733,
            scalar_v2745,
            scalar_v2970,
            scalar_v2971,
            scalar_v2980,
            scalar_v2981,
            scalar_v3004,
            scalar_v3005,
            scalar_v3016,
            scalar_v3017,
            scalar_v3353,
            scalar_v3392,
            scalar_v3393,
            scalar_v3436,
            scalar_v3437,
            scalar_v3526,
            scalar_v3565,
            scalar_v3566,
            scalar_v3752,
            scalar_v3753,
            scalar_v3754,
            scalar_v3755,
            scalar_v3756,
            scalar_v3757,
            scalar_v3967,
            scalar_v3968,
            scalar_v3969,
            scalar_v3970,
            scalar_v3971,
            scalar_v3972,
            scalar_v3973,
            scalar_v3974,
            scalar_v3975,
            scalar_v4367,
            scalar_v4839,
            scalar_v4918,
            scalar_v5154,
            scalar_v5155,
            scalar_v5156,
            scalar_v5157,
            scalar_v5158,
            scalar_v5159,
            scalar_v5276,
            scalar_v5277,
            scalar_v5304,
            scalar_v5305,
            scalar_v5306,
            scalar_v5307,
            scalar_v5308,
            scalar_v5309,
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
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505_va'", name)),
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
        let v13: f64 = p.p32;
        self.scalar_v13 = v13;
        let v14: f64 = (1.0 - p.p32);
        self.scalar_v14 = v14;
        let v15: f64 = p.p4;
        self.scalar_v15 = v15;
        let v17: f64 = (p.p4 + 273.15);
        self.scalar_v17 = v17;
        let v19: f64 = p.p0;
        self.scalar_v19 = v19;
        let v21: f64 = p.p137;
        self.scalar_v21 = v21;
        let v22: bool = (p.p137 == 0.0);
        self.scalar_v22 = v22;
        let v24: f64 = (if v22 { 1e-12 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: bool = (!v22);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p137 } else { v24 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p1;
        self.scalar_v27 = v27;
        let v28: f64 = (v26 * p.p1);
        self.scalar_v28 = v28;
        let v29: f64 = (1.0 / v28);
        self.scalar_v29 = v29;
        let v32: f64 = p.p66;
        self.scalar_v32 = v32;
        let v33: f64 = (2.0 - p.p66);
        self.scalar_v33 = v33;
        let v34: f64 = f64::powf(2.0, v33);
        self.scalar_v34 = v34;
        let v35: f64 = (1.0 / v34);
        self.scalar_v35 = v35;
        let v36: f64 = p.p113;
        self.scalar_v36 = v36;
        let v37: f64 = p.p114;
        self.scalar_v37 = v37;
        let v38: f64 = (p.p114 * v17);
        self.scalar_v38 = v38;
        let v39: f64 = (v38 * v17);
        self.scalar_v39 = v39;
        let v40: f64 = p.p115;
        self.scalar_v40 = v40;
        let v41: f64 = (v17 + p.p115);
        self.scalar_v41 = v41;
        let v42: f64 = (v39 / v41);
        self.scalar_v42 = v42;
        let v43: f64 = (p.p113 + v42);
        self.scalar_v43 = v43;
        let v45: f64 = (v43 - 0.05);
        self.scalar_v45 = v45;
        let v47: f64 = (v45 / 0.1);
        self.scalar_v47 = v47;
        let v48: bool = (v43 < 0.05);
        self.scalar_v48 = v48;
        let v49: f64 = v47.exp();
        self.scalar_v49 = v49;
        let v50: f64 = (1.0 + v49);
        self.scalar_v50 = v50;
        let v51: f64 = v50.ln();
        self.scalar_v51 = v51;
        let v52: f64 = (0.1 * v51);
        self.scalar_v52 = v52;
        let v53: f64 = (0.05 + v52);
        self.scalar_v53 = v53;
        let v54: f64 = (if v48 { v53 } else { 0.0 });
        self.scalar_v54 = v54;
        let v55: bool = (!v48);
        self.scalar_v55 = v55;
        let v56: f64 = (-v47);
        self.scalar_v56 = v56;
        let v57: f64 = v56.exp();
        self.scalar_v57 = v57;
        let v58: f64 = (1.0 + v57);
        self.scalar_v58 = v58;
        let v59: f64 = v58.ln();
        self.scalar_v59 = v59;
        let v60: f64 = (0.1 * v59);
        self.scalar_v60 = v60;
        let v61: f64 = (v43 + v60);
        self.scalar_v61 = v61;
        let v62: f64 = (if v55 { v61 } else { v54 });
        self.scalar_v62 = v62;
        let v63: f64 = (1.0 / p.p113);
        self.scalar_v63 = v63;
        let v64: f64 = p.p65;
        self.scalar_v64 = v64;
        let v65: f64 = (1.0 / p.p65);
        self.scalar_v65 = v65;
        let v66: f64 = p.p70;
        self.scalar_v66 = v66;
        let v67: f64 = p.p71;
        self.scalar_v67 = v67;
        let v68: f64 = (2.0 - p.p71);
        self.scalar_v68 = v68;
        let v69: f64 = f64::powf(2.0, v68);
        self.scalar_v69 = v69;
        let v70: f64 = (1.0 / v69);
        self.scalar_v70 = v70;
        let v71: f64 = p.p116;
        self.scalar_v71 = v71;
        let v72: f64 = p.p117;
        self.scalar_v72 = v72;
        let v73: f64 = (p.p117 * v17);
        self.scalar_v73 = v73;
        let v74: f64 = (v73 * v17);
        self.scalar_v74 = v74;
        let v75: f64 = p.p118;
        self.scalar_v75 = v75;
        let v76: f64 = (v17 + p.p118);
        self.scalar_v76 = v76;
        let v77: f64 = (v74 / v76);
        self.scalar_v77 = v77;
        let v78: f64 = (p.p116 + v77);
        self.scalar_v78 = v78;
        let v79: f64 = (v78 - 0.05);
        self.scalar_v79 = v79;
        let v80: f64 = (v79 / 0.1);
        self.scalar_v80 = v80;
        let v81: bool = (v78 < 0.05);
        self.scalar_v81 = v81;
        let v82: f64 = v80.exp();
        self.scalar_v82 = v82;
        let v83: f64 = (1.0 + v82);
        self.scalar_v83 = v83;
        let v84: f64 = v83.ln();
        self.scalar_v84 = v84;
        let v85: f64 = (0.1 * v84);
        self.scalar_v85 = v85;
        let v86: f64 = (0.05 + v85);
        self.scalar_v86 = v86;
        let v87: f64 = (if v81 { v86 } else { 0.0 });
        self.scalar_v87 = v87;
        let v88: bool = (!v81);
        self.scalar_v88 = v88;
        let v89: f64 = (-v80);
        self.scalar_v89 = v89;
        let v90: f64 = v89.exp();
        self.scalar_v90 = v90;
        let v91: f64 = (1.0 + v90);
        self.scalar_v91 = v91;
        let v92: f64 = v91.ln();
        self.scalar_v92 = v92;
        let v93: f64 = (0.1 * v92);
        self.scalar_v93 = v93;
        let v94: f64 = (v78 + v93);
        self.scalar_v94 = v94;
        let v95: f64 = (if v88 { v94 } else { v87 });
        self.scalar_v95 = v95;
        let v96: f64 = (1.0 / p.p116);
        self.scalar_v96 = v96;
        let v97: f64 = (1.0 / p.p70);
        self.scalar_v97 = v97;
        let v98: f64 = p.p82;
        self.scalar_v98 = v98;
        let v99: f64 = (1.0 / p.p82);
        self.scalar_v99 = v99;
        let v100: f64 = (1.0 - v99);
        self.scalar_v100 = v100;
        let v105: f64 = (8.617086918058125e-5 * v17);
        self.scalar_v105 = v105;
        let v107: f64 = (1.0 / v105);
        self.scalar_v107 = v107;
        let v162: f64 = p.p104;
        self.scalar_v162 = v162;
        let v182: f64 = p.p63;
        self.scalar_v182 = v182;
        let v185: f64 = p.p109;
        self.scalar_v185 = v185;
        let v225: f64 = p.p26;
        self.scalar_v225 = v225;
        let v228: f64 = p.p108;
        self.scalar_v228 = v228;
        let v254: f64 = p.p74;
        self.scalar_v254 = v254;
        let v255: f64 = (1.0 - p.p74);
        self.scalar_v255 = v255;
        let v262: f64 = p.p53;
        self.scalar_v262 = v262;
        let v263: f64 = p.p96;
        self.scalar_v263 = v263;
        let v269: f64 = p.p55;
        self.scalar_v269 = v269;
        let v270: f64 = p.p97;
        self.scalar_v270 = v270;
        let v271: f64 = p.p95;
        self.scalar_v271 = v271;
        let v272: f64 = (p.p97 - p.p95);
        self.scalar_v272 = v272;
        let v276: f64 = p.p54;
        self.scalar_v276 = v276;
        let v277: f64 = p.p100;
        self.scalar_v277 = v277;
        let v283: f64 = p.p56;
        self.scalar_v283 = v283;
        let v284: f64 = p.p101;
        self.scalar_v284 = v284;
        let v288: f64 = p.p57;
        self.scalar_v288 = v288;
        let v289: f64 = p.p103;
        self.scalar_v289 = v289;
        let v293: f64 = p.p58;
        self.scalar_v293 = v293;
        let v295: f64 = p.p59;
        self.scalar_v295 = v295;
        let v296: f64 = p.p98;
        self.scalar_v296 = v296;
        let v300: f64 = p.p121;
        self.scalar_v300 = v300;
        let v301: bool = (p.p121 != 0.0);
        self.scalar_v301 = v301;
        let v302: f64 = p.p9;
        self.scalar_v302 = v302;
        let v330: bool = (!v301);
        self.scalar_v330 = v330;
        let v332: f64 = p.p122;
        self.scalar_v332 = v332;
        let v333: bool = (p.p122 != 0.0);
        self.scalar_v333 = v333;
        let v334: f64 = p.p10;
        self.scalar_v334 = v334;
        let v361: bool = (!v333);
        self.scalar_v361 = v361;
        let v363: f64 = p.p42;
        self.scalar_v363 = v363;
        let v364: f64 = p.p123;
        self.scalar_v364 = v364;
        let v382: f64 = p.p8;
        self.scalar_v382 = v382;
        let v384: f64 = (4.0 - p.p97);
        self.scalar_v384 = v384;
        let v385: f64 = (v384 - p.p95);
        self.scalar_v385 = v385;
        let v386: f64 = p.p120;
        self.scalar_v386 = v386;
        let v387: f64 = (v385 + p.p120);
        self.scalar_v387 = v387;
        let v392: f64 = (-p.p104);
        self.scalar_v392 = v392;
        let v397: f64 = p.p11;
        self.scalar_v397 = v397;
        let v398: f64 = (1.0 - p.p97);
        self.scalar_v398 = v398;
        let v402: f64 = p.p29;
        self.scalar_v402 = v402;
        let v403: f64 = p.p102;
        self.scalar_v403 = v403;
        let v404: f64 = (1.0 - p.p102);
        self.scalar_v404 = v404;
        let v408: f64 = p.p19;
        self.scalar_v408 = v408;
        let v410: f64 = p.p20;
        self.scalar_v410 = v410;
        let v411: f64 = (2.0 * p.p20);
        self.scalar_v411 = v411;
        let v412: f64 = (6.0 - v411);
        self.scalar_v412 = v412;
        let v416: f64 = p.p112;
        self.scalar_v416 = v416;
        let v417: f64 = (-p.p112);
        self.scalar_v417 = v417;
        let v422: f64 = p.p30;
        self.scalar_v422 = v422;
        let v423: f64 = p.p31;
        self.scalar_v423 = v423;
        let v424: f64 = (2.0 * p.p31);
        self.scalar_v424 = v424;
        let v425: f64 = (6.0 - v424);
        self.scalar_v425 = v425;
        let v429: f64 = (-p.p109);
        self.scalar_v429 = v429;
        let v434: f64 = p.p15;
        self.scalar_v434 = v434;
        let v435: f64 = (4.0 - p.p96);
        self.scalar_v435 = v435;
        let v436: f64 = (v435 + p.p120);
        self.scalar_v436 = v436;
        let v438: f64 = p.p16;
        self.scalar_v438 = v438;
        let v442: f64 = p.p110;
        self.scalar_v442 = v442;
        let v443: f64 = (-p.p110);
        self.scalar_v443 = v443;
        let v448: f64 = p.p17;
        self.scalar_v448 = v448;
        let v449: f64 = p.p18;
        self.scalar_v449 = v449;
        let v456: f64 = p.p23;
        self.scalar_v456 = v456;
        let v457: bool = (p.p23 == 1.0);
        self.scalar_v457 = v457;
        let v458: f64 = p.p24;
        self.scalar_v458 = v458;
        let v459: f64 = p.p106;
        self.scalar_v459 = v459;
        let v460: f64 = (-p.p106);
        self.scalar_v460 = v460;
        let v466: f64 = p.p27;
        self.scalar_v466 = v466;
        let v467: f64 = p.p105;
        self.scalar_v467 = v467;
        let v468: f64 = (-p.p105);
        self.scalar_v468 = v468;
        let v473: f64 = p.p25;
        self.scalar_v473 = v473;
        let v474: f64 = p.p107;
        self.scalar_v474 = v474;
        let v475: f64 = (-p.p107);
        self.scalar_v475 = v475;
        let v481: f64 = p.p28;
        self.scalar_v481 = v481;
        let v482: f64 = (4.0 - p.p102);
        self.scalar_v482 = v482;
        let v483: f64 = (v482 + p.p120);
        self.scalar_v483 = v483;
        let v487: f64 = p.p111;
        self.scalar_v487 = v487;
        let v488: f64 = (-p.p111);
        self.scalar_v488 = v488;
        let v492: f64 = p.p21;
        self.scalar_v492 = v492;
        let v493: f64 = p.p22;
        self.scalar_v493 = v493;
        let v494: f64 = (2.0 * p.p22);
        self.scalar_v494 = v494;
        let v495: f64 = (6.0 - v494);
        self.scalar_v495 = v495;
        let v502: f64 = p.p132;
        self.scalar_v502 = v502;
        let v503: f64 = p.p133;
        self.scalar_v503 = v503;
        let v504: f64 = (4.0 / p.p133);
        self.scalar_v504 = v504;
        let v511: f64 = p.p138;
        self.scalar_v511 = v511;
        let v514: f64 = p.p140;
        self.scalar_v514 = v514;
        let v522: f64 = p.p34;
        self.scalar_v522 = v522;
        let v531: f64 = p.p33;
        self.scalar_v531 = v531;
        let v544: f64 = p.p36;
        self.scalar_v544 = v544;
        let v553: f64 = p.p35;
        self.scalar_v553 = v553;
        let v565: f64 = p.p13;
        self.scalar_v565 = v565;
        let v568: f64 = p.p12;
        self.scalar_v568 = v568;
        let v586: f64 = (v12 * 1.081);
        self.scalar_v586 = v586;
        let v588: f64 = p.p91;
        self.scalar_v588 = v588;
        let v590: bool = (p.p56 > 0.0);
        self.scalar_v590 = v590;
        let v596: bool = (!v590);
        self.scalar_v596 = v596;
        let v598: bool = (p.p57 > 0.0);
        self.scalar_v598 = v598;
        let v604: bool = (!v598);
        self.scalar_v604 = v604;
        let v606: bool = (p.p58 > 0.0);
        self.scalar_v606 = v606;
        let v612: bool = (!v606);
        self.scalar_v612 = v612;
        let v655: f64 = p.p134;
        self.scalar_v655 = v655;
        let v660: f64 = p.p134.exp();
        self.scalar_v660 = v660;
        let v748: f64 = p.p136;
        self.scalar_v748 = v748;
        let v802: f64 = p.p61;
        self.scalar_v802 = v802;
        let v803: f64 = p.p60;
        self.scalar_v803 = v803;
        let v804: f64 = (p.p61 * p.p60);
        self.scalar_v804 = v804;
        let v815: f64 = p.p62;
        self.scalar_v815 = v815;
        let v836: f64 = (-1.0 / p.p62);
        self.scalar_v836 = v836;
        let v837: f64 = v836.exp();
        self.scalar_v837 = v837;
        let v838: f64 = (1.0 + v837);
        self.scalar_v838 = v838;
        let v839: f64 = v838.ln();
        self.scalar_v839 = v839;
        let v840: f64 = (p.p62 * v839);
        self.scalar_v840 = v840;
        let v841: f64 = (1.0 + v840);
        self.scalar_v841 = v841;
        let v888: f64 = p.p135;
        self.scalar_v888 = v888;
        let v898: f64 = (0.5 * p.p60);
        self.scalar_v898 = v898;
        let v911: f64 = p.p72;
        self.scalar_v911 = v911;
        let v912: bool = (p.p72 == 0.0);
        self.scalar_v912 = v912;
        let v916: bool = (!v912);
        self.scalar_v916 = v916;
        let v967: f64 = (-1.0 / p.p66);
        self.scalar_v967 = v967;
        let v968: f64 = f64::powf(3.0, v967);
        self.scalar_v968 = v968;
        let v969: f64 = (1.0 - v968);
        self.scalar_v969 = v969;
        let v991: f64 = (1.0 - p.p66);
        self.scalar_v991 = v991;
        let v999: f64 = p.p73;
        self.scalar_v999 = v999;
        let v1000: bool = (p.p73 == 1.0);
        self.scalar_v1000 = v1000;
        let v1002: bool = (p.p73 == 2.0);
        self.scalar_v1002 = v1002;
        let v1003: bool = (!v1000);
        self.scalar_v1003 = v1003;
        let v1004: bool = (v1003 && v1002);
        self.scalar_v1004 = v1004;
        let v1007: bool = (!v1002);
        self.scalar_v1007 = v1007;
        let v1008: bool = (v1003 && v1007);
        self.scalar_v1008 = v1008;
        let v1013: f64 = (-1.0 / p.p71);
        self.scalar_v1013 = v1013;
        let v1034: f64 = p.p75;
        self.scalar_v1034 = v1034;
        let v1036: f64 = (1.0 - p.p71);
        self.scalar_v1036 = v1036;
        let v1065: bool = (p.p91 == 0.0);
        self.scalar_v1065 = v1065;
        let v1071: bool = (!v1065);
        self.scalar_v1071 = v1071;
        let v1106: f64 = p.p14;
        self.scalar_v1106 = v1106;
        let v1130: f64 = p.p139;
        self.scalar_v1130 = v1130;
        let v1143: f64 = p.p141;
        self.scalar_v1143 = v1143;
        let v1161: f64 = p.p142;
        self.scalar_v1161 = v1161;
        let v1224: f64 = p.p92;
        self.scalar_v1224 = v1224;
        let v1225: bool = (p.p92 == 0.0);
        self.scalar_v1225 = v1225;
        let v1226: bool = (!v457);
        self.scalar_v1226 = v1226;
        let v1227: bool = (v1226 && v1225);
        self.scalar_v1227 = v1227;
        let v1229: bool = (!v1225);
        self.scalar_v1229 = v1229;
        let v1230: bool = (v1226 && v1229);
        self.scalar_v1230 = v1230;
        let v1231: f64 = (1.0 - p.p92);
        self.scalar_v1231 = v1231;
        let v1324: bool = (p.p33 > 0.0);
        self.scalar_v1324 = v1324;
        let v1325: bool = (p.p34 > 0.0);
        self.scalar_v1325 = v1325;
        let v1326: bool = (v1324 && v1325);
        self.scalar_v1326 = v1326;
        let v1350: f64 = (-2.0 - p.p66);
        self.scalar_v1350 = v1350;
        let v1352: f64 = (p.p66 * p.p66);
        self.scalar_v1352 = v1352;
        let v1353: f64 = (1.0 - v1352);
        self.scalar_v1353 = v1353;
        let v1355: f64 = (p.p66 - 1.0);
        self.scalar_v1355 = v1355;
        let v1415: bool = (p.p35 > 0.0);
        self.scalar_v1415 = v1415;
        let v1416: bool = (p.p36 > 0.0);
        self.scalar_v1416 = v1416;
        let v1417: bool = (v1415 && v1416);
        self.scalar_v1417 = v1417;
        let v1443: f64 = (-2.0 - p.p71);
        self.scalar_v1443 = v1443;
        let v1445: f64 = (p.p71 * p.p71);
        self.scalar_v1445 = v1445;
        let v1446: f64 = (1.0 - v1445);
        self.scalar_v1446 = v1446;
        let v1448: f64 = (p.p71 - 1.0);
        self.scalar_v1448 = v1448;
        let v1514: f64 = p.p5;
        self.scalar_v1514 = v1514;
        let v1515: bool = (p.p5 > 0.0);
        self.scalar_v1515 = v1515;
        let v1516: bool = (p.p32 > 0.0);
        self.scalar_v1516 = v1516;
        let v1517: bool = (v1515 && v1516);
        self.scalar_v1517 = v1517;
        let v1520: f64 = (p.p32 * 2.0);
        self.scalar_v1520 = v1520;
        let v1530: f64 = (if v1517 { 0.0 } else { 0.0 });
        self.scalar_v1530 = v1530;
        let v1531: bool = (p.p5 == 1.0);
        self.scalar_v1531 = v1531;
        let v1532: bool = (v1517 && v1531);
        self.scalar_v1532 = v1532;
        let v1544: f64 = (if v1532 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1544 = v1544;
        let v1549: f64 = (0.5 * v1544);
        self.scalar_v1549 = v1549;
        let v1566: bool = (!v1531);
        self.scalar_v1566 = v1566;
        let v1567: bool = (v1517 && v1566);
        self.scalar_v1567 = v1567;
        let v1571: f64 = p.p83;
        self.scalar_v1571 = v1571;
        let v1572: bool = (p.p83 == 1.0);
        self.scalar_v1572 = v1572;
        let v1575: f64 = (if v1572 { 1e-12 } else { v1544 });
        self.scalar_v1575 = v1575;
        let v1582: f64 = (0.5 * v1575);
        self.scalar_v1582 = v1582;
        let v1593: f64 = p.p81;
        self.scalar_v1593 = v1593;
        let v1594: f64 = f64::powf(v100, p.p81);
        self.scalar_v1594 = v1594;
        let v1595: f64 = (1.0 - v1594);
        self.scalar_v1595 = v1595;
        let v1596: f64 = (1.0 / v1595);
        self.scalar_v1596 = v1596;
        let v1597: f64 = (if v1572 { v1596 } else { 0.0 });
        self.scalar_v1597 = v1597;
        let v1598: f64 = p.p80;
        self.scalar_v1598 = v1598;
        let v1599: f64 = (v100 * p.p80);
        self.scalar_v1599 = v1599;
        let v1600: f64 = (if v1572 { v1599 } else { 0.0 });
        self.scalar_v1600 = v1600;
        let v1601: f64 = (v1597 * v1597);
        self.scalar_v1601 = v1601;
        let v1602: f64 = (p.p81 - 1.0);
        self.scalar_v1602 = v1602;
        let v1603: f64 = f64::powf(v100, v1602);
        self.scalar_v1603 = v1603;
        let v1604: f64 = (v1601 * v1603);
        self.scalar_v1604 = v1604;
        let v1605: f64 = (v1604 * p.p81);
        self.scalar_v1605 = v1605;
        let v1606: f64 = (v1605 / p.p80);
        self.scalar_v1606 = v1606;
        let v1607: f64 = (if v1572 { v1606 } else { 0.0 });
        self.scalar_v1607 = v1607;
        let v1621: bool = (!v1572);
        self.scalar_v1621 = v1621;
        let v1648: f64 = p.p38;
        self.scalar_v1648 = v1648;
        let v1649: bool = (p.p38 == 1.0);
        self.scalar_v1649 = v1649;
        let v1650: f64 = p.p43;
        self.scalar_v1650 = v1650;
        let v1653: f64 = p.p41;
        self.scalar_v1653 = v1653;
        let v1672: f64 = p.p40;
        self.scalar_v1672 = v1672;
        let v1686: f64 = p.p39;
        self.scalar_v1686 = v1686;
        let v1691: bool = (p.p38 == 2.0);
        self.scalar_v1691 = v1691;
        let v1693: bool = (!v1649);
        self.scalar_v1693 = v1693;
        let v1697: f64 = p.p45;
        self.scalar_v1697 = v1697;
        let v1698: f64 = (2.0 * p.p45);
        self.scalar_v1698 = v1698;
        let v1699: f64 = p.p44;
        self.scalar_v1699 = v1699;
        let v1700: f64 = (p.p44 * p.p44);
        self.scalar_v1700 = v1700;
        let v1701: f64 = (v1698 / v1700);
        self.scalar_v1701 = v1701;
        let v1710: f64 = p.p7;
        self.scalar_v1710 = v1710;
        let v1711: bool = (p.p7 == 0.0);
        self.scalar_v1711 = v1711;
        let v1714: bool = (!v1711);
        self.scalar_v1714 = v1714;
        let v1737: f64 = p.p46;
        self.scalar_v1737 = v1737;
        let v1738: f64 = (2.0 * p.p46);
        self.scalar_v1738 = v1738;
        let v1744: f64 = (1.0 + p.p46);
        self.scalar_v1744 = v1744;
        let v1745: f64 = (1.0 + v1738);
        self.scalar_v1745 = v1745;
        let v1746: f64 = (v1744 / v1745);
        self.scalar_v1746 = v1746;
        let v1794: bool = (p.p38 == 3.0);
        self.scalar_v1794 = v1794;
        let v1795: bool = (!v1691);
        self.scalar_v1795 = v1795;
        let v1800: f64 = p.p47;
        self.scalar_v1800 = v1800;
        let v1804: f64 = p.p48;
        self.scalar_v1804 = v1804;
        let v1811: f64 = p.p51;
        self.scalar_v1811 = v1811;
        let v1816: f64 = p.p50;
        self.scalar_v1816 = v1816;
        let v1836: f64 = p.p49;
        self.scalar_v1836 = v1836;
        let v1856: f64 = p.p52;
        self.scalar_v1856 = v1856;
        let v1857: bool = (p.p52 == 1.0);
        self.scalar_v1857 = v1857;
        let v1892: bool = (!v1794);
        self.scalar_v1892 = v1892;
        let v1898: bool = (!v1857);
        self.scalar_v1898 = v1898;
        let v1903: f64 = p.p129;
        self.scalar_v1903 = v1903;
        let v1904: bool = (p.p129 > 0.0);
        self.scalar_v1904 = v1904;
        let v1908: bool = (!v1904);
        self.scalar_v1908 = v1908;
        let v1964: f64 = (p.p3 * -1.0);
        self.scalar_v1964 = v1964;
        let v1965: f64 = (v1964 + p.p3);
        self.scalar_v1965 = v1965;
        let v1966: f64 = (-p.p3);
        self.scalar_v1966 = v1966;
        let v1967: f64 = (v1964 - v1964);
        self.scalar_v1967 = v1967;
        let v1968: f64 = (v1966 - v1964);
        self.scalar_v1968 = v1968;
        let v1969: f64 = (-v1964);
        self.scalar_v1969 = v1969;
        let v1970: f64 = (v1966 + p.p3);
        self.scalar_v1970 = v1970;
        let v1971: f64 = (v1964 + v1969);
        self.scalar_v1971 = v1971;
        let v1972: f64 = (p.p3 + v1970);
        self.scalar_v1972 = v1972;
        let v2650: f64 = (v991 - 1.0);
        self.scalar_v2650 = v2650;
        let v2665: f64 = (if v1000 { p.p3 } else { 0.0 });
        self.scalar_v2665 = v2665;
        let v2666: f64 = (if v1000 { v1964 } else { 0.0 });
        self.scalar_v2666 = v2666;
        let v2733: f64 = (p.p75 - 1.0);
        self.scalar_v2733 = v2733;
        let v2745: f64 = (v1036 - 1.0);
        self.scalar_v2745 = v2745;
        let v2970: f64 = (v1964 / 0.0001);
        self.scalar_v2970 = v2970;
        let v2971: f64 = (p.p3 / 0.0001);
        self.scalar_v2971 = v2971;
        let v2980: f64 = (-v2970);
        self.scalar_v2980 = v2980;
        let v2981: f64 = (-v2971);
        self.scalar_v2981 = v2981;
        let v3004: f64 = (v1964 / 0.001);
        self.scalar_v3004 = v3004;
        let v3005: f64 = (p.p3 / 0.001);
        self.scalar_v3005 = v3005;
        let v3016: f64 = (-v3004);
        self.scalar_v3016 = v3016;
        let v3017: f64 = (-v3005);
        self.scalar_v3017 = v3017;
        let v3353: f64 = (v1350 - 1.0);
        self.scalar_v3353 = v3353;
        let v3392: f64 = (v1964 * v34);
        self.scalar_v3392 = v3392;
        let v3393: f64 = (p.p3 * v34);
        self.scalar_v3393 = v3393;
        let v3436: f64 = (v1964 * 0.5);
        self.scalar_v3436 = v3436;
        let v3437: f64 = (p.p3 * 0.5);
        self.scalar_v3437 = v3437;
        let v3526: f64 = (v1443 - 1.0);
        self.scalar_v3526 = v3526;
        let v3565: f64 = (p.p3 * v69);
        self.scalar_v3565 = v3565;
        let v3566: f64 = (v1964 * v69);
        self.scalar_v3566 = v3566;
        let v3752: f64 = (if v1532 { v1971 } else { 0.0 });
        self.scalar_v3752 = v3752;
        let v3753: f64 = (if v1532 { v1972 } else { 0.0 });
        self.scalar_v3753 = v3753;
        let v3754: f64 = (if v1532 { v1965 } else { 0.0 });
        self.scalar_v3754 = v3754;
        let v3755: f64 = (if v1532 { v1968 } else { 0.0 });
        self.scalar_v3755 = v3755;
        let v3756: f64 = (if v1532 { v1967 } else { 0.0 });
        self.scalar_v3756 = v3756;
        let v3757: f64 = (if v1532 { v1966 } else { 0.0 });
        self.scalar_v3757 = v3757;
        let v3967: f64 = (if v1572 { p.p3 } else { 0.0 });
        self.scalar_v3967 = v3967;
        let v3968: f64 = (if v1572 { v1965 } else { 0.0 });
        self.scalar_v3968 = v3968;
        let v3969: f64 = (if v1572 { v1964 } else { 0.0 });
        self.scalar_v3969 = v3969;
        let v3970: f64 = (-1.0 * v3967);
        self.scalar_v3970 = v3970;
        let v3971: f64 = (-1.0 * v3968);
        self.scalar_v3971 = v3971;
        let v3972: f64 = (-1.0 * v3969);
        self.scalar_v3972 = v3972;
        let v3973: f64 = (v3970 * -1.0);
        self.scalar_v3973 = v3973;
        let v3974: f64 = (v3971 * -1.0);
        self.scalar_v3974 = v3974;
        let v3975: f64 = (v3972 * -1.0);
        self.scalar_v3975 = v3975;
        let v4367: f64 = (p.p40 - 1.0);
        self.scalar_v4367 = v4367;
        let v4839: f64 = (p.p48 - 1.0);
        self.scalar_v4839 = v4839;
        let v4918: f64 = (p.p49 - 1.0);
        self.scalar_v4918 = v4918;
        let v5154: f64 = (0.0 * v1964);
        self.scalar_v5154 = v5154;
        let v5155: f64 = (0.0 * p.p3);
        self.scalar_v5155 = v5155;
        let v5156: f64 = (0.0 * v1965);
        self.scalar_v5156 = v5156;
        let v5157: f64 = (0.0 * v1968);
        self.scalar_v5157 = v5157;
        let v5158: f64 = (0.0 * v1967);
        self.scalar_v5158 = v5158;
        let v5159: f64 = (0.0 * v1966);
        self.scalar_v5159 = v5159;
        let v5276: f64 = (p.p3 * p.p3);
        self.scalar_v5276 = v5276;
        let v5277: f64 = (p.p3 * v1964);
        self.scalar_v5277 = v5277;
        let v5304: f64 = (p.p3 * v1969);
        self.scalar_v5304 = v5304;
        let v5305: f64 = (p.p3 * v1970);
        self.scalar_v5305 = v5305;
        let v5306: f64 = (p.p3 * v1965);
        self.scalar_v5306 = v5306;
        let v5307: f64 = (p.p3 * v1968);
        self.scalar_v5307 = v5307;
        let v5308: f64 = (p.p3 * v1967);
        self.scalar_v5308 = v5308;
        let v5309: f64 = (p.p3 * v1966);
        self.scalar_v5309 = v5309;
    }
}
