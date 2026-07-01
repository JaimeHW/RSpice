#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

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
    pub(crate) ddt_state_older: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
    pub(crate) ddt_derivative_current: Box<[f64; 9]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 9]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
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
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v354: bool,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: bool,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v385: bool,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v388: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v467: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v481: bool,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v538: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v568: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v589: f64,
    pub(crate) scalar_v592: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v619: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v646: bool,
    pub(crate) scalar_v652: bool,
    pub(crate) scalar_v654: bool,
    pub(crate) scalar_v660: bool,
    pub(crate) scalar_v662: bool,
    pub(crate) scalar_v668: bool,
    pub(crate) scalar_v713: f64,
    pub(crate) scalar_v718: f64,
    pub(crate) scalar_v818: f64,
    pub(crate) scalar_v871: f64,
    pub(crate) scalar_v872: f64,
    pub(crate) scalar_v873: f64,
    pub(crate) scalar_v884: f64,
    pub(crate) scalar_v905: f64,
    pub(crate) scalar_v906: f64,
    pub(crate) scalar_v907: f64,
    pub(crate) scalar_v908: f64,
    pub(crate) scalar_v909: f64,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v957: f64,
    pub(crate) scalar_v967: f64,
    pub(crate) scalar_v980: f64,
    pub(crate) scalar_v981: bool,
    pub(crate) scalar_v985: bool,
    pub(crate) scalar_v1034: f64,
    pub(crate) scalar_v1035: f64,
    pub(crate) scalar_v1036: f64,
    pub(crate) scalar_v1058: f64,
    pub(crate) scalar_v1066: f64,
    pub(crate) scalar_v1067: bool,
    pub(crate) scalar_v1069: bool,
    pub(crate) scalar_v1070: bool,
    pub(crate) scalar_v1071: bool,
    pub(crate) scalar_v1074: bool,
    pub(crate) scalar_v1075: bool,
    pub(crate) scalar_v1080: f64,
    pub(crate) scalar_v1101: f64,
    pub(crate) scalar_v1103: f64,
    pub(crate) scalar_v1132: bool,
    pub(crate) scalar_v1138: bool,
    pub(crate) scalar_v1172: f64,
    pub(crate) scalar_v1194: f64,
    pub(crate) scalar_v1207: f64,
    pub(crate) scalar_v1225: f64,
    pub(crate) scalar_v1288: f64,
    pub(crate) scalar_v1289: bool,
    pub(crate) scalar_v1290: bool,
    pub(crate) scalar_v1291: bool,
    pub(crate) scalar_v1293: bool,
    pub(crate) scalar_v1294: bool,
    pub(crate) scalar_v1295: f64,
    pub(crate) scalar_v1388: bool,
    pub(crate) scalar_v1389: bool,
    pub(crate) scalar_v1390: bool,
    pub(crate) scalar_v1414: f64,
    pub(crate) scalar_v1416: f64,
    pub(crate) scalar_v1417: f64,
    pub(crate) scalar_v1419: f64,
    pub(crate) scalar_v1479: bool,
    pub(crate) scalar_v1480: bool,
    pub(crate) scalar_v1481: bool,
    pub(crate) scalar_v1507: f64,
    pub(crate) scalar_v1509: f64,
    pub(crate) scalar_v1510: f64,
    pub(crate) scalar_v1512: f64,
    pub(crate) scalar_v1589: f64,
    pub(crate) scalar_v1590: bool,
    pub(crate) scalar_v1591: bool,
    pub(crate) scalar_v1592: bool,
    pub(crate) scalar_v1595: f64,
    pub(crate) scalar_v1605: bool,
    pub(crate) scalar_v1606: bool,
    pub(crate) scalar_v1618: f64,
    pub(crate) scalar_v1623: f64,
    pub(crate) scalar_v1639: bool,
    pub(crate) scalar_v1640: bool,
    pub(crate) scalar_v1644: f64,
    pub(crate) scalar_v1645: bool,
    pub(crate) scalar_v1648: f64,
    pub(crate) scalar_v1654: f64,
    pub(crate) scalar_v1665: f64,
    pub(crate) scalar_v1666: f64,
    pub(crate) scalar_v1667: f64,
    pub(crate) scalar_v1668: f64,
    pub(crate) scalar_v1669: f64,
    pub(crate) scalar_v1670: f64,
    pub(crate) scalar_v1671: f64,
    pub(crate) scalar_v1672: f64,
    pub(crate) scalar_v1673: f64,
    pub(crate) scalar_v1674: f64,
    pub(crate) scalar_v1675: f64,
    pub(crate) scalar_v1676: f64,
    pub(crate) scalar_v1677: f64,
    pub(crate) scalar_v1678: f64,
    pub(crate) scalar_v1679: f64,
    pub(crate) scalar_v1693: bool,
    pub(crate) scalar_v1720: f64,
    pub(crate) scalar_v1721: bool,
    pub(crate) scalar_v1722: f64,
    pub(crate) scalar_v1725: f64,
    pub(crate) scalar_v1744: f64,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1763: bool,
    pub(crate) scalar_v1765: bool,
    pub(crate) scalar_v1769: f64,
    pub(crate) scalar_v1770: f64,
    pub(crate) scalar_v1771: f64,
    pub(crate) scalar_v1772: f64,
    pub(crate) scalar_v1773: f64,
    pub(crate) scalar_v1782: f64,
    pub(crate) scalar_v1783: bool,
    pub(crate) scalar_v1786: bool,
    pub(crate) scalar_v1809: f64,
    pub(crate) scalar_v1810: f64,
    pub(crate) scalar_v1816: f64,
    pub(crate) scalar_v1817: f64,
    pub(crate) scalar_v1818: f64,
    pub(crate) scalar_v1866: bool,
    pub(crate) scalar_v1867: bool,
    pub(crate) scalar_v1872: f64,
    pub(crate) scalar_v1876: f64,
    pub(crate) scalar_v1883: f64,
    pub(crate) scalar_v1888: f64,
    pub(crate) scalar_v1908: f64,
    pub(crate) scalar_v1928: f64,
    pub(crate) scalar_v1929: bool,
    pub(crate) scalar_v1964: bool,
    pub(crate) scalar_v1970: bool,
    pub(crate) scalar_v1973: f64,
    pub(crate) scalar_v1974: f64,
    pub(crate) scalar_v2004: f64,
    pub(crate) scalar_v2042: f64,
    pub(crate) scalar_v2078: f64,
    pub(crate) scalar_v2079: f64,
    pub(crate) scalar_v2101: f64,
    pub(crate) scalar_v2102: bool,
    pub(crate) scalar_v2111: f64,
    pub(crate) scalar_v2115: bool,
    pub(crate) scalar_v2134: bool,
    pub(crate) scalar_v2135: bool,
    pub(crate) scalar_v2136: bool,
    pub(crate) scalar_v2139: bool,
    pub(crate) scalar_v2155: f64,
    pub(crate) scalar_v2166: bool,
    pub(crate) scalar_v2187: f64,
    pub(crate) scalar_v2188: bool,
    pub(crate) scalar_v2189: f64,
    pub(crate) scalar_v2227: f64,
    pub(crate) scalar_v2228: f64,
    pub(crate) scalar_v2234: f64,
    pub(crate) scalar_v2238: f64,
    pub(crate) scalar_v2241: bool,
    pub(crate) scalar_v2247: f64,
    pub(crate) scalar_v2248: bool,
    pub(crate) scalar_v2252: bool,
    pub(crate) scalar_v2262: f64,
    pub(crate) scalar_v2263: bool,
    pub(crate) scalar_v2266: bool,
    pub(crate) scalar_v2267: bool,
    pub(crate) scalar_v2268: bool,
    pub(crate) scalar_v2269: f64,
    pub(crate) scalar_v2272: bool,
    pub(crate) scalar_v2273: bool,
    pub(crate) scalar_v2288: f64,
    pub(crate) scalar_v2289: f64,
    pub(crate) scalar_v2329: f64,
    pub(crate) scalar_v2333: f64,
    pub(crate) scalar_v2363: f64,
    pub(crate) scalar_v2364: f64,
    pub(crate) scalar_v2365: f64,
    pub(crate) scalar_v2366: f64,
    pub(crate) scalar_v3051: f64,
    pub(crate) scalar_v3066: f64,
    pub(crate) scalar_v3067: f64,
    pub(crate) scalar_v3134: f64,
    pub(crate) scalar_v3146: f64,
    pub(crate) scalar_v3367: f64,
    pub(crate) scalar_v3368: f64,
    pub(crate) scalar_v3377: f64,
    pub(crate) scalar_v3378: f64,
    pub(crate) scalar_v3401: f64,
    pub(crate) scalar_v3402: f64,
    pub(crate) scalar_v3413: f64,
    pub(crate) scalar_v3414: f64,
    pub(crate) scalar_v3739: f64,
    pub(crate) scalar_v3778: f64,
    pub(crate) scalar_v3779: f64,
    pub(crate) scalar_v3822: f64,
    pub(crate) scalar_v3823: f64,
    pub(crate) scalar_v3910: f64,
    pub(crate) scalar_v3949: f64,
    pub(crate) scalar_v3950: f64,
    pub(crate) scalar_v4163: f64,
    pub(crate) scalar_v4164: f64,
    pub(crate) scalar_v4165: f64,
    pub(crate) scalar_v4166: f64,
    pub(crate) scalar_v4342: f64,
    pub(crate) scalar_v4343: f64,
    pub(crate) scalar_v4344: f64,
    pub(crate) scalar_v4345: f64,
    pub(crate) scalar_v4346: f64,
    pub(crate) scalar_v4347: f64,
    pub(crate) scalar_v4711: f64,
    pub(crate) scalar_v5173: f64,
    pub(crate) scalar_v5252: f64,
    pub(crate) scalar_v5806: f64,
    pub(crate) scalar_v5807: f64,
    pub(crate) scalar_v5808: f64,
    pub(crate) scalar_v5809: f64,
    pub(crate) scalar_v6041: f64,
    pub(crate) scalar_v6131: f64,
    pub(crate) scalar_v6132: f64,
    pub(crate) scalar_v6332: f64,
    pub(crate) scalar_v6333: f64,
    pub(crate) scalar_v6334: f64,
    pub(crate) scalar_v6335: f64,
    pub(crate) scalar_v6479: f64,
    pub(crate) scalar_v6480: f64,
    pub(crate) scalar_v6550: f64,
    pub(crate) scalar_v6551: f64,
    pub(crate) scalar_v6556: f64,
    pub(crate) scalar_v6557: f64,
    pub(crate) scalar_v6580: f64,
    pub(crate) scalar_v6581: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: bool,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: bool,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: bool,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: bool,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: bool,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: bool,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: bool,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: bool,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v210: bool,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: bool,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: bool,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
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
    pub(crate) scalar_v244: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: bool,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v259: f64,
    pub(crate) scalar_v260: bool,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: bool,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: bool,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: bool,
    pub(crate) scalar_v335: bool,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: bool,
    pub(crate) scalar_v343: bool,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v366: bool,
    pub(crate) scalar_v367: bool,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v383: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v390: f64,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: bool,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v530: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v536: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v549: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v558: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: f64,
    pub(crate) scalar_v565: f64,
    pub(crate) scalar_v566: f64,
    pub(crate) scalar_v567: f64,
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
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v582: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v585: f64,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v587: f64,
    pub(crate) scalar_v588: f64,
    pub(crate) scalar_v590: f64,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v593: f64,
    pub(crate) scalar_v594: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v598: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v609: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v615: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v618: f64,
    pub(crate) scalar_v620: f64,
    pub(crate) scalar_v624: f64,
    pub(crate) scalar_v625: f64,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v628: f64,
    pub(crate) scalar_v630: bool,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v633: f64,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v636: f64,
    pub(crate) scalar_v637: f64,
    pub(crate) scalar_v638: f64,
    pub(crate) scalar_v639: f64,
    pub(crate) scalar_v640: bool,
    pub(crate) scalar_v643: f64,
    pub(crate) scalar_v645: f64,
    pub(crate) scalar_v647: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v649: bool,
    pub(crate) scalar_v650: bool,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v656: f64,
    pub(crate) scalar_v657: bool,
    pub(crate) scalar_v658: bool,
    pub(crate) scalar_v659: f64,
    pub(crate) scalar_v661: f64,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v664: f64,
    pub(crate) scalar_v665: bool,
    pub(crate) scalar_v666: bool,
    pub(crate) scalar_v667: f64,
    pub(crate) scalar_v669: f64,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v852: f64,
    pub(crate) scalar_v876: f64,
    pub(crate) scalar_v963: f64,
    pub(crate) scalar_v964: f64,
    pub(crate) scalar_v971: f64,
    pub(crate) scalar_v972: f64,
    pub(crate) scalar_v983: f64,
    pub(crate) scalar_v1006: f64,
    pub(crate) scalar_v1010: f64,
    pub(crate) scalar_v1037: f64,
    pub(crate) scalar_v1038: f64,
    pub(crate) scalar_v1060: f64,
    pub(crate) scalar_v1077: f64,
    pub(crate) scalar_v1078: f64,
    pub(crate) scalar_v1079: f64,
    pub(crate) scalar_v1081: f64,
    pub(crate) scalar_v1082: f64,
    pub(crate) scalar_v1083: f64,
    pub(crate) scalar_v1104: f64,
    pub(crate) scalar_v1118: f64,
    pub(crate) scalar_v1119: f64,
    pub(crate) scalar_v1125: f64,
    pub(crate) scalar_v1150: f64,
    pub(crate) scalar_v1151: f64,
    pub(crate) scalar_v1152: f64,
    pub(crate) scalar_v1173: f64,
    pub(crate) scalar_v1271: f64,
    pub(crate) scalar_v1330: f64,
    pub(crate) scalar_v1470: f64,
    pub(crate) scalar_v1559: f64,
    pub(crate) scalar_v1579: f64,
    pub(crate) scalar_v1582: f64,
    pub(crate) scalar_v1583: f64,
    pub(crate) scalar_v1596: f64,
    pub(crate) scalar_v1607: f64,
    pub(crate) scalar_v1608: f64,
    pub(crate) scalar_v1609: f64,
    pub(crate) scalar_v1610: f64,
    pub(crate) scalar_v1611: f64,
    pub(crate) scalar_v1612: f64,
    pub(crate) scalar_v1613: f64,
    pub(crate) scalar_v1614: f64,
    pub(crate) scalar_v1743: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1848: f64,
    pub(crate) scalar_v1851: f64,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1994: f64,
    pub(crate) scalar_v2005: f64,
    pub(crate) scalar_v2007: f64,
    pub(crate) scalar_v2008: f64,
    pub(crate) scalar_v2076: f64,
    pub(crate) scalar_v2077: f64,
    pub(crate) scalar_v2080: f64,
    pub(crate) scalar_v2081: f64,
    pub(crate) scalar_v2082: f64,
    pub(crate) scalar_v2094: f64,
    pub(crate) scalar_v2095: f64,
    pub(crate) scalar_v2096: f64,
    pub(crate) scalar_v2097: f64,
    pub(crate) scalar_v2103: f64,
    pub(crate) scalar_v2126: f64,
    pub(crate) scalar_v2156: f64,
    pub(crate) scalar_v2177: f64,
    pub(crate) scalar_v2367: f64,
    pub(crate) scalar_v2368: f64,
    pub(crate) scalar_v2377: f64,
    pub(crate) scalar_v2378: f64,
    pub(crate) scalar_v2387: f64,
    pub(crate) scalar_v2388: f64,
    pub(crate) scalar_v2413: f64,
    pub(crate) scalar_v3023: f64,
    pub(crate) scalar_v3024: f64,
    pub(crate) scalar_v3035: f64,
    pub(crate) scalar_v3036: f64,
    pub(crate) scalar_v3188: f64,
    pub(crate) scalar_v3189: f64,
    pub(crate) scalar_v3206: f64,
    pub(crate) scalar_v3439: f64,
    pub(crate) scalar_v3440: f64,
    pub(crate) scalar_v3572: f64,
    pub(crate) scalar_v3573: f64,
    pub(crate) scalar_v3629: f64,
    pub(crate) scalar_v3630: f64,
    pub(crate) scalar_v3644: f64,
    pub(crate) scalar_v3645: f64,
    pub(crate) scalar_v3659: f64,
    pub(crate) scalar_v3660: f64,
    pub(crate) scalar_v3661: f64,
    pub(crate) scalar_v3662: f64,
    pub(crate) scalar_v3686: f64,
    pub(crate) scalar_v3687: f64,
    pub(crate) scalar_v3728: f64,
    pub(crate) scalar_v3729: f64,
    pub(crate) scalar_v3780: f64,
    pub(crate) scalar_v3781: f64,
    pub(crate) scalar_v3870: f64,
    pub(crate) scalar_v3871: f64,
    pub(crate) scalar_v3872: f64,
    pub(crate) scalar_v3873: f64,
    pub(crate) scalar_v3951: f64,
    pub(crate) scalar_v3952: f64,
    pub(crate) scalar_v5295: f64,
    pub(crate) scalar_v5296: f64,
    pub(crate) scalar_v5548: f64,
    pub(crate) scalar_v5549: f64,
    pub(crate) scalar_v5550: f64,
    pub(crate) scalar_v5551: f64,
    pub(crate) scalar_v5572: f64,
    pub(crate) scalar_v5573: f64,
    pub(crate) scalar_v5574: f64,
    pub(crate) scalar_v5575: f64,
    pub(crate) scalar_v5634: f64,
    pub(crate) scalar_v5635: f64,
    pub(crate) scalar_v5652: f64,
    pub(crate) scalar_v5673: f64,
    pub(crate) scalar_v5732: f64,
    pub(crate) scalar_v5749: f64,
    pub(crate) scalar_v5750: f64,
    pub(crate) scalar_v5810: f64,
    pub(crate) scalar_v5811: f64,
    pub(crate) scalar_v5812: f64,
    pub(crate) scalar_v5813: f64,
    pub(crate) scalar_v6048: f64,
    pub(crate) scalar_v6049: f64,
    pub(crate) scalar_v6059: f64,
    pub(crate) scalar_v6060: f64,
    pub(crate) scalar_v6481: f64,
    pub(crate) scalar_v6482: f64,
    pub(crate) scalar_v6483: f64,
    pub(crate) scalar_v6484: f64,
    pub(crate) scalar_v6485: f64,
    pub(crate) scalar_v6486: f64,
    pub(crate) scalar_v6487: f64,
    pub(crate) scalar_v6488: f64,
    pub(crate) scalar_v6582: f64,
    pub(crate) scalar_v6583: f64,
    pub(crate) scalar_v6584: f64,
    pub(crate) scalar_v6585: f64,
    pub(crate) scalar_v6586: f64,
    pub(crate) scalar_v6587: f64,
    pub(crate) scalar_v6588: f64,
    pub(crate) scalar_v6589: f64,
    pub(crate) scalar_v6649: f64,
    pub(crate) scalar_v6650: f64,
    pub(crate) scalar_v6651: f64,
    pub(crate) scalar_v6652: f64,
    pub(crate) scalar_v6653: f64,
    pub(crate) scalar_v6654: f64,
    pub(crate) scalar_v6655: f64,
    pub(crate) scalar_v6656: f64,
    pub(crate) scalar_v6657: f64,
    pub(crate) scalar_v6658: f64,
    pub(crate) scalar_v6659: f64,
    pub(crate) scalar_v6660: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
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
            ddt_state_older: self.ddt_state_older.clone(),
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            ddt_derivative_current: self.ddt_derivative_current.clone(),
            ddt_derivative_previous: self.ddt_derivative_previous.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
            ddt_coefficients: self.ddt_coefficients,
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
            scalar_v104: self.scalar_v104,
            scalar_v106: self.scalar_v106,
            scalar_v161: self.scalar_v161,
            scalar_v181: self.scalar_v181,
            scalar_v184: self.scalar_v184,
            scalar_v204: self.scalar_v204,
            scalar_v245: self.scalar_v245,
            scalar_v248: self.scalar_v248,
            scalar_v274: self.scalar_v274,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v283: self.scalar_v283,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v317: self.scalar_v317,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v324: self.scalar_v324,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v354: self.scalar_v354,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v385: self.scalar_v385,
            scalar_v387: self.scalar_v387,
            scalar_v388: self.scalar_v388,
            scalar_v406: self.scalar_v406,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v416: self.scalar_v416,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v432: self.scalar_v432,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v440: self.scalar_v440,
            scalar_v441: self.scalar_v441,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v453: self.scalar_v453,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v462: self.scalar_v462,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v480: self.scalar_v480,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v484: self.scalar_v484,
            scalar_v490: self.scalar_v490,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v499: self.scalar_v499,
            scalar_v505: self.scalar_v505,
            scalar_v506: self.scalar_v506,
            scalar_v507: self.scalar_v507,
            scalar_v511: self.scalar_v511,
            scalar_v512: self.scalar_v512,
            scalar_v516: self.scalar_v516,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v519: self.scalar_v519,
            scalar_v526: self.scalar_v526,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v535: self.scalar_v535,
            scalar_v538: self.scalar_v538,
            scalar_v546: self.scalar_v546,
            scalar_v555: self.scalar_v555,
            scalar_v568: self.scalar_v568,
            scalar_v577: self.scalar_v577,
            scalar_v589: self.scalar_v589,
            scalar_v592: self.scalar_v592,
            scalar_v595: self.scalar_v595,
            scalar_v596: self.scalar_v596,
            scalar_v600: self.scalar_v600,
            scalar_v601: self.scalar_v601,
            scalar_v605: self.scalar_v605,
            scalar_v606: self.scalar_v606,
            scalar_v607: self.scalar_v607,
            scalar_v611: self.scalar_v611,
            scalar_v612: self.scalar_v612,
            scalar_v616: self.scalar_v616,
            scalar_v619: self.scalar_v619,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v642: self.scalar_v642,
            scalar_v644: self.scalar_v644,
            scalar_v646: self.scalar_v646,
            scalar_v652: self.scalar_v652,
            scalar_v654: self.scalar_v654,
            scalar_v660: self.scalar_v660,
            scalar_v662: self.scalar_v662,
            scalar_v668: self.scalar_v668,
            scalar_v713: self.scalar_v713,
            scalar_v718: self.scalar_v718,
            scalar_v818: self.scalar_v818,
            scalar_v871: self.scalar_v871,
            scalar_v872: self.scalar_v872,
            scalar_v873: self.scalar_v873,
            scalar_v884: self.scalar_v884,
            scalar_v905: self.scalar_v905,
            scalar_v906: self.scalar_v906,
            scalar_v907: self.scalar_v907,
            scalar_v908: self.scalar_v908,
            scalar_v909: self.scalar_v909,
            scalar_v910: self.scalar_v910,
            scalar_v957: self.scalar_v957,
            scalar_v967: self.scalar_v967,
            scalar_v980: self.scalar_v980,
            scalar_v981: self.scalar_v981,
            scalar_v985: self.scalar_v985,
            scalar_v1034: self.scalar_v1034,
            scalar_v1035: self.scalar_v1035,
            scalar_v1036: self.scalar_v1036,
            scalar_v1058: self.scalar_v1058,
            scalar_v1066: self.scalar_v1066,
            scalar_v1067: self.scalar_v1067,
            scalar_v1069: self.scalar_v1069,
            scalar_v1070: self.scalar_v1070,
            scalar_v1071: self.scalar_v1071,
            scalar_v1074: self.scalar_v1074,
            scalar_v1075: self.scalar_v1075,
            scalar_v1080: self.scalar_v1080,
            scalar_v1101: self.scalar_v1101,
            scalar_v1103: self.scalar_v1103,
            scalar_v1132: self.scalar_v1132,
            scalar_v1138: self.scalar_v1138,
            scalar_v1172: self.scalar_v1172,
            scalar_v1194: self.scalar_v1194,
            scalar_v1207: self.scalar_v1207,
            scalar_v1225: self.scalar_v1225,
            scalar_v1288: self.scalar_v1288,
            scalar_v1289: self.scalar_v1289,
            scalar_v1290: self.scalar_v1290,
            scalar_v1291: self.scalar_v1291,
            scalar_v1293: self.scalar_v1293,
            scalar_v1294: self.scalar_v1294,
            scalar_v1295: self.scalar_v1295,
            scalar_v1388: self.scalar_v1388,
            scalar_v1389: self.scalar_v1389,
            scalar_v1390: self.scalar_v1390,
            scalar_v1414: self.scalar_v1414,
            scalar_v1416: self.scalar_v1416,
            scalar_v1417: self.scalar_v1417,
            scalar_v1419: self.scalar_v1419,
            scalar_v1479: self.scalar_v1479,
            scalar_v1480: self.scalar_v1480,
            scalar_v1481: self.scalar_v1481,
            scalar_v1507: self.scalar_v1507,
            scalar_v1509: self.scalar_v1509,
            scalar_v1510: self.scalar_v1510,
            scalar_v1512: self.scalar_v1512,
            scalar_v1589: self.scalar_v1589,
            scalar_v1590: self.scalar_v1590,
            scalar_v1591: self.scalar_v1591,
            scalar_v1592: self.scalar_v1592,
            scalar_v1595: self.scalar_v1595,
            scalar_v1605: self.scalar_v1605,
            scalar_v1606: self.scalar_v1606,
            scalar_v1618: self.scalar_v1618,
            scalar_v1623: self.scalar_v1623,
            scalar_v1639: self.scalar_v1639,
            scalar_v1640: self.scalar_v1640,
            scalar_v1644: self.scalar_v1644,
            scalar_v1645: self.scalar_v1645,
            scalar_v1648: self.scalar_v1648,
            scalar_v1654: self.scalar_v1654,
            scalar_v1665: self.scalar_v1665,
            scalar_v1666: self.scalar_v1666,
            scalar_v1667: self.scalar_v1667,
            scalar_v1668: self.scalar_v1668,
            scalar_v1669: self.scalar_v1669,
            scalar_v1670: self.scalar_v1670,
            scalar_v1671: self.scalar_v1671,
            scalar_v1672: self.scalar_v1672,
            scalar_v1673: self.scalar_v1673,
            scalar_v1674: self.scalar_v1674,
            scalar_v1675: self.scalar_v1675,
            scalar_v1676: self.scalar_v1676,
            scalar_v1677: self.scalar_v1677,
            scalar_v1678: self.scalar_v1678,
            scalar_v1679: self.scalar_v1679,
            scalar_v1693: self.scalar_v1693,
            scalar_v1720: self.scalar_v1720,
            scalar_v1721: self.scalar_v1721,
            scalar_v1722: self.scalar_v1722,
            scalar_v1725: self.scalar_v1725,
            scalar_v1744: self.scalar_v1744,
            scalar_v1758: self.scalar_v1758,
            scalar_v1763: self.scalar_v1763,
            scalar_v1765: self.scalar_v1765,
            scalar_v1769: self.scalar_v1769,
            scalar_v1770: self.scalar_v1770,
            scalar_v1771: self.scalar_v1771,
            scalar_v1772: self.scalar_v1772,
            scalar_v1773: self.scalar_v1773,
            scalar_v1782: self.scalar_v1782,
            scalar_v1783: self.scalar_v1783,
            scalar_v1786: self.scalar_v1786,
            scalar_v1809: self.scalar_v1809,
            scalar_v1810: self.scalar_v1810,
            scalar_v1816: self.scalar_v1816,
            scalar_v1817: self.scalar_v1817,
            scalar_v1818: self.scalar_v1818,
            scalar_v1866: self.scalar_v1866,
            scalar_v1867: self.scalar_v1867,
            scalar_v1872: self.scalar_v1872,
            scalar_v1876: self.scalar_v1876,
            scalar_v1883: self.scalar_v1883,
            scalar_v1888: self.scalar_v1888,
            scalar_v1908: self.scalar_v1908,
            scalar_v1928: self.scalar_v1928,
            scalar_v1929: self.scalar_v1929,
            scalar_v1964: self.scalar_v1964,
            scalar_v1970: self.scalar_v1970,
            scalar_v1973: self.scalar_v1973,
            scalar_v1974: self.scalar_v1974,
            scalar_v2004: self.scalar_v2004,
            scalar_v2042: self.scalar_v2042,
            scalar_v2078: self.scalar_v2078,
            scalar_v2079: self.scalar_v2079,
            scalar_v2101: self.scalar_v2101,
            scalar_v2102: self.scalar_v2102,
            scalar_v2111: self.scalar_v2111,
            scalar_v2115: self.scalar_v2115,
            scalar_v2134: self.scalar_v2134,
            scalar_v2135: self.scalar_v2135,
            scalar_v2136: self.scalar_v2136,
            scalar_v2139: self.scalar_v2139,
            scalar_v2155: self.scalar_v2155,
            scalar_v2166: self.scalar_v2166,
            scalar_v2187: self.scalar_v2187,
            scalar_v2188: self.scalar_v2188,
            scalar_v2189: self.scalar_v2189,
            scalar_v2227: self.scalar_v2227,
            scalar_v2228: self.scalar_v2228,
            scalar_v2234: self.scalar_v2234,
            scalar_v2238: self.scalar_v2238,
            scalar_v2241: self.scalar_v2241,
            scalar_v2247: self.scalar_v2247,
            scalar_v2248: self.scalar_v2248,
            scalar_v2252: self.scalar_v2252,
            scalar_v2262: self.scalar_v2262,
            scalar_v2263: self.scalar_v2263,
            scalar_v2266: self.scalar_v2266,
            scalar_v2267: self.scalar_v2267,
            scalar_v2268: self.scalar_v2268,
            scalar_v2269: self.scalar_v2269,
            scalar_v2272: self.scalar_v2272,
            scalar_v2273: self.scalar_v2273,
            scalar_v2288: self.scalar_v2288,
            scalar_v2289: self.scalar_v2289,
            scalar_v2329: self.scalar_v2329,
            scalar_v2333: self.scalar_v2333,
            scalar_v2363: self.scalar_v2363,
            scalar_v2364: self.scalar_v2364,
            scalar_v2365: self.scalar_v2365,
            scalar_v2366: self.scalar_v2366,
            scalar_v3051: self.scalar_v3051,
            scalar_v3066: self.scalar_v3066,
            scalar_v3067: self.scalar_v3067,
            scalar_v3134: self.scalar_v3134,
            scalar_v3146: self.scalar_v3146,
            scalar_v3367: self.scalar_v3367,
            scalar_v3368: self.scalar_v3368,
            scalar_v3377: self.scalar_v3377,
            scalar_v3378: self.scalar_v3378,
            scalar_v3401: self.scalar_v3401,
            scalar_v3402: self.scalar_v3402,
            scalar_v3413: self.scalar_v3413,
            scalar_v3414: self.scalar_v3414,
            scalar_v3739: self.scalar_v3739,
            scalar_v3778: self.scalar_v3778,
            scalar_v3779: self.scalar_v3779,
            scalar_v3822: self.scalar_v3822,
            scalar_v3823: self.scalar_v3823,
            scalar_v3910: self.scalar_v3910,
            scalar_v3949: self.scalar_v3949,
            scalar_v3950: self.scalar_v3950,
            scalar_v4163: self.scalar_v4163,
            scalar_v4164: self.scalar_v4164,
            scalar_v4165: self.scalar_v4165,
            scalar_v4166: self.scalar_v4166,
            scalar_v4342: self.scalar_v4342,
            scalar_v4343: self.scalar_v4343,
            scalar_v4344: self.scalar_v4344,
            scalar_v4345: self.scalar_v4345,
            scalar_v4346: self.scalar_v4346,
            scalar_v4347: self.scalar_v4347,
            scalar_v4711: self.scalar_v4711,
            scalar_v5173: self.scalar_v5173,
            scalar_v5252: self.scalar_v5252,
            scalar_v5806: self.scalar_v5806,
            scalar_v5807: self.scalar_v5807,
            scalar_v5808: self.scalar_v5808,
            scalar_v5809: self.scalar_v5809,
            scalar_v6041: self.scalar_v6041,
            scalar_v6131: self.scalar_v6131,
            scalar_v6132: self.scalar_v6132,
            scalar_v6332: self.scalar_v6332,
            scalar_v6333: self.scalar_v6333,
            scalar_v6334: self.scalar_v6334,
            scalar_v6335: self.scalar_v6335,
            scalar_v6479: self.scalar_v6479,
            scalar_v6480: self.scalar_v6480,
            scalar_v6550: self.scalar_v6550,
            scalar_v6551: self.scalar_v6551,
            scalar_v6556: self.scalar_v6556,
            scalar_v6557: self.scalar_v6557,
            scalar_v6580: self.scalar_v6580,
            scalar_v6581: self.scalar_v6581,
            scalar_v20: self.scalar_v20,
            scalar_v101: self.scalar_v101,
            scalar_v103: self.scalar_v103,
            scalar_v105: self.scalar_v105,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
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
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
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
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
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
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
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
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
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
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v275: self.scalar_v275,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v314: self.scalar_v314,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v318: self.scalar_v318,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
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
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v355: self.scalar_v355,
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
            scalar_v373: self.scalar_v373,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v378: self.scalar_v378,
            scalar_v379: self.scalar_v379,
            scalar_v380: self.scalar_v380,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v383: self.scalar_v383,
            scalar_v384: self.scalar_v384,
            scalar_v386: self.scalar_v386,
            scalar_v389: self.scalar_v389,
            scalar_v390: self.scalar_v390,
            scalar_v391: self.scalar_v391,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v405: self.scalar_v405,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v461: self.scalar_v461,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v465: self.scalar_v465,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v478: self.scalar_v478,
            scalar_v479: self.scalar_v479,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v500: self.scalar_v500,
            scalar_v501: self.scalar_v501,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v504: self.scalar_v504,
            scalar_v508: self.scalar_v508,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v513: self.scalar_v513,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v520: self.scalar_v520,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v523: self.scalar_v523,
            scalar_v524: self.scalar_v524,
            scalar_v525: self.scalar_v525,
            scalar_v529: self.scalar_v529,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v534: self.scalar_v534,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v549: self.scalar_v549,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v553: self.scalar_v553,
            scalar_v554: self.scalar_v554,
            scalar_v556: self.scalar_v556,
            scalar_v557: self.scalar_v557,
            scalar_v558: self.scalar_v558,
            scalar_v559: self.scalar_v559,
            scalar_v560: self.scalar_v560,
            scalar_v561: self.scalar_v561,
            scalar_v562: self.scalar_v562,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v565: self.scalar_v565,
            scalar_v566: self.scalar_v566,
            scalar_v567: self.scalar_v567,
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
            scalar_v580: self.scalar_v580,
            scalar_v581: self.scalar_v581,
            scalar_v582: self.scalar_v582,
            scalar_v583: self.scalar_v583,
            scalar_v584: self.scalar_v584,
            scalar_v585: self.scalar_v585,
            scalar_v586: self.scalar_v586,
            scalar_v587: self.scalar_v587,
            scalar_v588: self.scalar_v588,
            scalar_v590: self.scalar_v590,
            scalar_v591: self.scalar_v591,
            scalar_v593: self.scalar_v593,
            scalar_v594: self.scalar_v594,
            scalar_v597: self.scalar_v597,
            scalar_v598: self.scalar_v598,
            scalar_v599: self.scalar_v599,
            scalar_v602: self.scalar_v602,
            scalar_v603: self.scalar_v603,
            scalar_v604: self.scalar_v604,
            scalar_v608: self.scalar_v608,
            scalar_v609: self.scalar_v609,
            scalar_v610: self.scalar_v610,
            scalar_v613: self.scalar_v613,
            scalar_v614: self.scalar_v614,
            scalar_v615: self.scalar_v615,
            scalar_v617: self.scalar_v617,
            scalar_v618: self.scalar_v618,
            scalar_v620: self.scalar_v620,
            scalar_v624: self.scalar_v624,
            scalar_v625: self.scalar_v625,
            scalar_v626: self.scalar_v626,
            scalar_v628: self.scalar_v628,
            scalar_v630: self.scalar_v630,
            scalar_v632: self.scalar_v632,
            scalar_v633: self.scalar_v633,
            scalar_v635: self.scalar_v635,
            scalar_v636: self.scalar_v636,
            scalar_v637: self.scalar_v637,
            scalar_v638: self.scalar_v638,
            scalar_v639: self.scalar_v639,
            scalar_v640: self.scalar_v640,
            scalar_v643: self.scalar_v643,
            scalar_v645: self.scalar_v645,
            scalar_v647: self.scalar_v647,
            scalar_v648: self.scalar_v648,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v653: self.scalar_v653,
            scalar_v655: self.scalar_v655,
            scalar_v656: self.scalar_v656,
            scalar_v657: self.scalar_v657,
            scalar_v658: self.scalar_v658,
            scalar_v659: self.scalar_v659,
            scalar_v661: self.scalar_v661,
            scalar_v663: self.scalar_v663,
            scalar_v664: self.scalar_v664,
            scalar_v665: self.scalar_v665,
            scalar_v666: self.scalar_v666,
            scalar_v667: self.scalar_v667,
            scalar_v669: self.scalar_v669,
            scalar_v841: self.scalar_v841,
            scalar_v852: self.scalar_v852,
            scalar_v876: self.scalar_v876,
            scalar_v963: self.scalar_v963,
            scalar_v964: self.scalar_v964,
            scalar_v971: self.scalar_v971,
            scalar_v972: self.scalar_v972,
            scalar_v983: self.scalar_v983,
            scalar_v1006: self.scalar_v1006,
            scalar_v1010: self.scalar_v1010,
            scalar_v1037: self.scalar_v1037,
            scalar_v1038: self.scalar_v1038,
            scalar_v1060: self.scalar_v1060,
            scalar_v1077: self.scalar_v1077,
            scalar_v1078: self.scalar_v1078,
            scalar_v1079: self.scalar_v1079,
            scalar_v1081: self.scalar_v1081,
            scalar_v1082: self.scalar_v1082,
            scalar_v1083: self.scalar_v1083,
            scalar_v1104: self.scalar_v1104,
            scalar_v1118: self.scalar_v1118,
            scalar_v1119: self.scalar_v1119,
            scalar_v1125: self.scalar_v1125,
            scalar_v1150: self.scalar_v1150,
            scalar_v1151: self.scalar_v1151,
            scalar_v1152: self.scalar_v1152,
            scalar_v1173: self.scalar_v1173,
            scalar_v1271: self.scalar_v1271,
            scalar_v1330: self.scalar_v1330,
            scalar_v1470: self.scalar_v1470,
            scalar_v1559: self.scalar_v1559,
            scalar_v1579: self.scalar_v1579,
            scalar_v1582: self.scalar_v1582,
            scalar_v1583: self.scalar_v1583,
            scalar_v1596: self.scalar_v1596,
            scalar_v1607: self.scalar_v1607,
            scalar_v1608: self.scalar_v1608,
            scalar_v1609: self.scalar_v1609,
            scalar_v1610: self.scalar_v1610,
            scalar_v1611: self.scalar_v1611,
            scalar_v1612: self.scalar_v1612,
            scalar_v1613: self.scalar_v1613,
            scalar_v1614: self.scalar_v1614,
            scalar_v1743: self.scalar_v1743,
            scalar_v1759: self.scalar_v1759,
            scalar_v1848: self.scalar_v1848,
            scalar_v1851: self.scalar_v1851,
            scalar_v1975: self.scalar_v1975,
            scalar_v1994: self.scalar_v1994,
            scalar_v2005: self.scalar_v2005,
            scalar_v2007: self.scalar_v2007,
            scalar_v2008: self.scalar_v2008,
            scalar_v2076: self.scalar_v2076,
            scalar_v2077: self.scalar_v2077,
            scalar_v2080: self.scalar_v2080,
            scalar_v2081: self.scalar_v2081,
            scalar_v2082: self.scalar_v2082,
            scalar_v2094: self.scalar_v2094,
            scalar_v2095: self.scalar_v2095,
            scalar_v2096: self.scalar_v2096,
            scalar_v2097: self.scalar_v2097,
            scalar_v2103: self.scalar_v2103,
            scalar_v2126: self.scalar_v2126,
            scalar_v2156: self.scalar_v2156,
            scalar_v2177: self.scalar_v2177,
            scalar_v2367: self.scalar_v2367,
            scalar_v2368: self.scalar_v2368,
            scalar_v2377: self.scalar_v2377,
            scalar_v2378: self.scalar_v2378,
            scalar_v2387: self.scalar_v2387,
            scalar_v2388: self.scalar_v2388,
            scalar_v2413: self.scalar_v2413,
            scalar_v3023: self.scalar_v3023,
            scalar_v3024: self.scalar_v3024,
            scalar_v3035: self.scalar_v3035,
            scalar_v3036: self.scalar_v3036,
            scalar_v3188: self.scalar_v3188,
            scalar_v3189: self.scalar_v3189,
            scalar_v3206: self.scalar_v3206,
            scalar_v3439: self.scalar_v3439,
            scalar_v3440: self.scalar_v3440,
            scalar_v3572: self.scalar_v3572,
            scalar_v3573: self.scalar_v3573,
            scalar_v3629: self.scalar_v3629,
            scalar_v3630: self.scalar_v3630,
            scalar_v3644: self.scalar_v3644,
            scalar_v3645: self.scalar_v3645,
            scalar_v3659: self.scalar_v3659,
            scalar_v3660: self.scalar_v3660,
            scalar_v3661: self.scalar_v3661,
            scalar_v3662: self.scalar_v3662,
            scalar_v3686: self.scalar_v3686,
            scalar_v3687: self.scalar_v3687,
            scalar_v3728: self.scalar_v3728,
            scalar_v3729: self.scalar_v3729,
            scalar_v3780: self.scalar_v3780,
            scalar_v3781: self.scalar_v3781,
            scalar_v3870: self.scalar_v3870,
            scalar_v3871: self.scalar_v3871,
            scalar_v3872: self.scalar_v3872,
            scalar_v3873: self.scalar_v3873,
            scalar_v3951: self.scalar_v3951,
            scalar_v3952: self.scalar_v3952,
            scalar_v5295: self.scalar_v5295,
            scalar_v5296: self.scalar_v5296,
            scalar_v5548: self.scalar_v5548,
            scalar_v5549: self.scalar_v5549,
            scalar_v5550: self.scalar_v5550,
            scalar_v5551: self.scalar_v5551,
            scalar_v5572: self.scalar_v5572,
            scalar_v5573: self.scalar_v5573,
            scalar_v5574: self.scalar_v5574,
            scalar_v5575: self.scalar_v5575,
            scalar_v5634: self.scalar_v5634,
            scalar_v5635: self.scalar_v5635,
            scalar_v5652: self.scalar_v5652,
            scalar_v5673: self.scalar_v5673,
            scalar_v5732: self.scalar_v5732,
            scalar_v5749: self.scalar_v5749,
            scalar_v5750: self.scalar_v5750,
            scalar_v5810: self.scalar_v5810,
            scalar_v5811: self.scalar_v5811,
            scalar_v5812: self.scalar_v5812,
            scalar_v5813: self.scalar_v5813,
            scalar_v6048: self.scalar_v6048,
            scalar_v6049: self.scalar_v6049,
            scalar_v6059: self.scalar_v6059,
            scalar_v6060: self.scalar_v6060,
            scalar_v6481: self.scalar_v6481,
            scalar_v6482: self.scalar_v6482,
            scalar_v6483: self.scalar_v6483,
            scalar_v6484: self.scalar_v6484,
            scalar_v6485: self.scalar_v6485,
            scalar_v6486: self.scalar_v6486,
            scalar_v6487: self.scalar_v6487,
            scalar_v6488: self.scalar_v6488,
            scalar_v6582: self.scalar_v6582,
            scalar_v6583: self.scalar_v6583,
            scalar_v6584: self.scalar_v6584,
            scalar_v6585: self.scalar_v6585,
            scalar_v6586: self.scalar_v6586,
            scalar_v6587: self.scalar_v6587,
            scalar_v6588: self.scalar_v6588,
            scalar_v6589: self.scalar_v6589,
            scalar_v6649: self.scalar_v6649,
            scalar_v6650: self.scalar_v6650,
            scalar_v6651: self.scalar_v6651,
            scalar_v6652: self.scalar_v6652,
            scalar_v6653: self.scalar_v6653,
            scalar_v6654: self.scalar_v6654,
            scalar_v6655: self.scalar_v6655,
            scalar_v6656: self.scalar_v6656,
            scalar_v6657: self.scalar_v6657,
            scalar_v6658: self.scalar_v6658,
            scalar_v6659: self.scalar_v6659,
            scalar_v6660: self.scalar_v6660,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
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
            ddt_state_older: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_derivative_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            ddt_coefficients: GeneratedDdtCoefficients::inactive(),
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
            scalar_v104: 0.0,
            scalar_v106: 0.0,
            scalar_v161: 0.0,
            scalar_v181: 0.0,
            scalar_v184: 0.0,
            scalar_v204: 0.0,
            scalar_v245: 0.0,
            scalar_v248: 0.0,
            scalar_v274: 0.0,
            scalar_v276: 0.0,
            scalar_v277: 0.0,
            scalar_v283: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v317: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v324: 0.0,
            scalar_v325: false,
            scalar_v326: 0.0,
            scalar_v354: false,
            scalar_v356: 0.0,
            scalar_v357: false,
            scalar_v358: 0.0,
            scalar_v385: false,
            scalar_v387: 0.0,
            scalar_v388: 0.0,
            scalar_v406: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v416: 0.0,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v428: 0.0,
            scalar_v432: 0.0,
            scalar_v434: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v440: 0.0,
            scalar_v441: 0.0,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v453: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v462: 0.0,
            scalar_v466: 0.0,
            scalar_v467: 0.0,
            scalar_v472: 0.0,
            scalar_v473: 0.0,
            scalar_v480: 0.0,
            scalar_v481: false,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v484: 0.0,
            scalar_v490: 0.0,
            scalar_v491: 0.0,
            scalar_v492: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v499: 0.0,
            scalar_v505: 0.0,
            scalar_v506: 0.0,
            scalar_v507: 0.0,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v516: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v526: 0.0,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v535: 0.0,
            scalar_v538: 0.0,
            scalar_v546: 0.0,
            scalar_v555: 0.0,
            scalar_v568: 0.0,
            scalar_v577: 0.0,
            scalar_v589: 0.0,
            scalar_v592: 0.0,
            scalar_v595: 0.0,
            scalar_v596: 0.0,
            scalar_v600: 0.0,
            scalar_v601: 0.0,
            scalar_v605: 0.0,
            scalar_v606: 0.0,
            scalar_v607: 0.0,
            scalar_v611: 0.0,
            scalar_v612: 0.0,
            scalar_v616: 0.0,
            scalar_v619: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v642: 0.0,
            scalar_v644: 0.0,
            scalar_v646: false,
            scalar_v652: false,
            scalar_v654: false,
            scalar_v660: false,
            scalar_v662: false,
            scalar_v668: false,
            scalar_v713: 0.0,
            scalar_v718: 0.0,
            scalar_v818: 0.0,
            scalar_v871: 0.0,
            scalar_v872: 0.0,
            scalar_v873: 0.0,
            scalar_v884: 0.0,
            scalar_v905: 0.0,
            scalar_v906: 0.0,
            scalar_v907: 0.0,
            scalar_v908: 0.0,
            scalar_v909: 0.0,
            scalar_v910: 0.0,
            scalar_v957: 0.0,
            scalar_v967: 0.0,
            scalar_v980: 0.0,
            scalar_v981: false,
            scalar_v985: false,
            scalar_v1034: 0.0,
            scalar_v1035: 0.0,
            scalar_v1036: 0.0,
            scalar_v1058: 0.0,
            scalar_v1066: 0.0,
            scalar_v1067: false,
            scalar_v1069: false,
            scalar_v1070: false,
            scalar_v1071: false,
            scalar_v1074: false,
            scalar_v1075: false,
            scalar_v1080: 0.0,
            scalar_v1101: 0.0,
            scalar_v1103: 0.0,
            scalar_v1132: false,
            scalar_v1138: false,
            scalar_v1172: 0.0,
            scalar_v1194: 0.0,
            scalar_v1207: 0.0,
            scalar_v1225: 0.0,
            scalar_v1288: 0.0,
            scalar_v1289: false,
            scalar_v1290: false,
            scalar_v1291: false,
            scalar_v1293: false,
            scalar_v1294: false,
            scalar_v1295: 0.0,
            scalar_v1388: false,
            scalar_v1389: false,
            scalar_v1390: false,
            scalar_v1414: 0.0,
            scalar_v1416: 0.0,
            scalar_v1417: 0.0,
            scalar_v1419: 0.0,
            scalar_v1479: false,
            scalar_v1480: false,
            scalar_v1481: false,
            scalar_v1507: 0.0,
            scalar_v1509: 0.0,
            scalar_v1510: 0.0,
            scalar_v1512: 0.0,
            scalar_v1589: 0.0,
            scalar_v1590: false,
            scalar_v1591: false,
            scalar_v1592: false,
            scalar_v1595: 0.0,
            scalar_v1605: false,
            scalar_v1606: false,
            scalar_v1618: 0.0,
            scalar_v1623: 0.0,
            scalar_v1639: false,
            scalar_v1640: false,
            scalar_v1644: 0.0,
            scalar_v1645: false,
            scalar_v1648: 0.0,
            scalar_v1654: 0.0,
            scalar_v1665: 0.0,
            scalar_v1666: 0.0,
            scalar_v1667: 0.0,
            scalar_v1668: 0.0,
            scalar_v1669: 0.0,
            scalar_v1670: 0.0,
            scalar_v1671: 0.0,
            scalar_v1672: 0.0,
            scalar_v1673: 0.0,
            scalar_v1674: 0.0,
            scalar_v1675: 0.0,
            scalar_v1676: 0.0,
            scalar_v1677: 0.0,
            scalar_v1678: 0.0,
            scalar_v1679: 0.0,
            scalar_v1693: false,
            scalar_v1720: 0.0,
            scalar_v1721: false,
            scalar_v1722: 0.0,
            scalar_v1725: 0.0,
            scalar_v1744: 0.0,
            scalar_v1758: 0.0,
            scalar_v1763: false,
            scalar_v1765: false,
            scalar_v1769: 0.0,
            scalar_v1770: 0.0,
            scalar_v1771: 0.0,
            scalar_v1772: 0.0,
            scalar_v1773: 0.0,
            scalar_v1782: 0.0,
            scalar_v1783: false,
            scalar_v1786: false,
            scalar_v1809: 0.0,
            scalar_v1810: 0.0,
            scalar_v1816: 0.0,
            scalar_v1817: 0.0,
            scalar_v1818: 0.0,
            scalar_v1866: false,
            scalar_v1867: false,
            scalar_v1872: 0.0,
            scalar_v1876: 0.0,
            scalar_v1883: 0.0,
            scalar_v1888: 0.0,
            scalar_v1908: 0.0,
            scalar_v1928: 0.0,
            scalar_v1929: false,
            scalar_v1964: false,
            scalar_v1970: false,
            scalar_v1973: 0.0,
            scalar_v1974: 0.0,
            scalar_v2004: 0.0,
            scalar_v2042: 0.0,
            scalar_v2078: 0.0,
            scalar_v2079: 0.0,
            scalar_v2101: 0.0,
            scalar_v2102: false,
            scalar_v2111: 0.0,
            scalar_v2115: false,
            scalar_v2134: false,
            scalar_v2135: false,
            scalar_v2136: false,
            scalar_v2139: false,
            scalar_v2155: 0.0,
            scalar_v2166: false,
            scalar_v2187: 0.0,
            scalar_v2188: false,
            scalar_v2189: 0.0,
            scalar_v2227: 0.0,
            scalar_v2228: 0.0,
            scalar_v2234: 0.0,
            scalar_v2238: 0.0,
            scalar_v2241: false,
            scalar_v2247: 0.0,
            scalar_v2248: false,
            scalar_v2252: false,
            scalar_v2262: 0.0,
            scalar_v2263: false,
            scalar_v2266: false,
            scalar_v2267: false,
            scalar_v2268: false,
            scalar_v2269: 0.0,
            scalar_v2272: false,
            scalar_v2273: false,
            scalar_v2288: 0.0,
            scalar_v2289: 0.0,
            scalar_v2329: 0.0,
            scalar_v2333: 0.0,
            scalar_v2363: 0.0,
            scalar_v2364: 0.0,
            scalar_v2365: 0.0,
            scalar_v2366: 0.0,
            scalar_v3051: 0.0,
            scalar_v3066: 0.0,
            scalar_v3067: 0.0,
            scalar_v3134: 0.0,
            scalar_v3146: 0.0,
            scalar_v3367: 0.0,
            scalar_v3368: 0.0,
            scalar_v3377: 0.0,
            scalar_v3378: 0.0,
            scalar_v3401: 0.0,
            scalar_v3402: 0.0,
            scalar_v3413: 0.0,
            scalar_v3414: 0.0,
            scalar_v3739: 0.0,
            scalar_v3778: 0.0,
            scalar_v3779: 0.0,
            scalar_v3822: 0.0,
            scalar_v3823: 0.0,
            scalar_v3910: 0.0,
            scalar_v3949: 0.0,
            scalar_v3950: 0.0,
            scalar_v4163: 0.0,
            scalar_v4164: 0.0,
            scalar_v4165: 0.0,
            scalar_v4166: 0.0,
            scalar_v4342: 0.0,
            scalar_v4343: 0.0,
            scalar_v4344: 0.0,
            scalar_v4345: 0.0,
            scalar_v4346: 0.0,
            scalar_v4347: 0.0,
            scalar_v4711: 0.0,
            scalar_v5173: 0.0,
            scalar_v5252: 0.0,
            scalar_v5806: 0.0,
            scalar_v5807: 0.0,
            scalar_v5808: 0.0,
            scalar_v5809: 0.0,
            scalar_v6041: 0.0,
            scalar_v6131: 0.0,
            scalar_v6132: 0.0,
            scalar_v6332: 0.0,
            scalar_v6333: 0.0,
            scalar_v6334: 0.0,
            scalar_v6335: 0.0,
            scalar_v6479: 0.0,
            scalar_v6480: 0.0,
            scalar_v6550: 0.0,
            scalar_v6551: 0.0,
            scalar_v6556: 0.0,
            scalar_v6557: 0.0,
            scalar_v6580: 0.0,
            scalar_v6581: 0.0,
            scalar_v20: 0.0,
            scalar_v101: 0.0,
            scalar_v103: 0.0,
            scalar_v105: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: false,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: false,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: false,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: false,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v166: false,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: false,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: false,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v196: false,
            scalar_v197: 0.0,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v210: false,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: false,
            scalar_v218: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v221: 0.0,
            scalar_v222: 0.0,
            scalar_v223: 0.0,
            scalar_v224: 0.0,
            scalar_v225: 0.0,
            scalar_v226: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: false,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
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
            scalar_v244: 0.0,
            scalar_v246: 0.0,
            scalar_v247: 0.0,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: false,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v257: 0.0,
            scalar_v258: 0.0,
            scalar_v259: 0.0,
            scalar_v260: false,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v266: 0.0,
            scalar_v267: 0.0,
            scalar_v268: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v275: 0.0,
            scalar_v278: 0.0,
            scalar_v279: 0.0,
            scalar_v280: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v291: false,
            scalar_v292: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v305: false,
            scalar_v306: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v314: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v318: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: false,
            scalar_v335: false,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v342: false,
            scalar_v343: false,
            scalar_v344: 0.0,
            scalar_v345: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v355: 0.0,
            scalar_v359: 0.0,
            scalar_v360: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v366: false,
            scalar_v367: false,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v370: 0.0,
            scalar_v371: 0.0,
            scalar_v372: 0.0,
            scalar_v373: 0.0,
            scalar_v374: false,
            scalar_v375: false,
            scalar_v376: 0.0,
            scalar_v377: 0.0,
            scalar_v378: 0.0,
            scalar_v379: 0.0,
            scalar_v380: 0.0,
            scalar_v381: 0.0,
            scalar_v382: 0.0,
            scalar_v383: 0.0,
            scalar_v384: 0.0,
            scalar_v386: 0.0,
            scalar_v389: 0.0,
            scalar_v390: 0.0,
            scalar_v391: 0.0,
            scalar_v393: 0.0,
            scalar_v394: false,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v401: 0.0,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v405: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v429: 0.0,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v437: 0.0,
            scalar_v438: 0.0,
            scalar_v439: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v445: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v461: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v465: 0.0,
            scalar_v468: 0.0,
            scalar_v469: 0.0,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v478: 0.0,
            scalar_v479: 0.0,
            scalar_v485: 0.0,
            scalar_v486: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v500: 0.0,
            scalar_v501: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v504: 0.0,
            scalar_v508: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v513: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v520: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v523: 0.0,
            scalar_v524: 0.0,
            scalar_v525: 0.0,
            scalar_v529: 0.0,
            scalar_v530: 0.0,
            scalar_v531: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v534: 0.0,
            scalar_v536: 0.0,
            scalar_v537: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v544: 0.0,
            scalar_v545: 0.0,
            scalar_v547: 0.0,
            scalar_v548: 0.0,
            scalar_v549: 0.0,
            scalar_v550: 0.0,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v554: 0.0,
            scalar_v556: 0.0,
            scalar_v557: 0.0,
            scalar_v558: 0.0,
            scalar_v559: 0.0,
            scalar_v560: 0.0,
            scalar_v561: 0.0,
            scalar_v562: 0.0,
            scalar_v563: 0.0,
            scalar_v564: 0.0,
            scalar_v565: 0.0,
            scalar_v566: 0.0,
            scalar_v567: 0.0,
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
            scalar_v580: 0.0,
            scalar_v581: 0.0,
            scalar_v582: 0.0,
            scalar_v583: 0.0,
            scalar_v584: 0.0,
            scalar_v585: 0.0,
            scalar_v586: 0.0,
            scalar_v587: 0.0,
            scalar_v588: 0.0,
            scalar_v590: 0.0,
            scalar_v591: 0.0,
            scalar_v593: 0.0,
            scalar_v594: 0.0,
            scalar_v597: 0.0,
            scalar_v598: 0.0,
            scalar_v599: 0.0,
            scalar_v602: 0.0,
            scalar_v603: 0.0,
            scalar_v604: 0.0,
            scalar_v608: 0.0,
            scalar_v609: 0.0,
            scalar_v610: 0.0,
            scalar_v613: 0.0,
            scalar_v614: 0.0,
            scalar_v615: 0.0,
            scalar_v617: 0.0,
            scalar_v618: 0.0,
            scalar_v620: 0.0,
            scalar_v624: 0.0,
            scalar_v625: 0.0,
            scalar_v626: 0.0,
            scalar_v628: 0.0,
            scalar_v630: false,
            scalar_v632: 0.0,
            scalar_v633: 0.0,
            scalar_v635: 0.0,
            scalar_v636: 0.0,
            scalar_v637: 0.0,
            scalar_v638: 0.0,
            scalar_v639: 0.0,
            scalar_v640: false,
            scalar_v643: 0.0,
            scalar_v645: 0.0,
            scalar_v647: 0.0,
            scalar_v648: 0.0,
            scalar_v649: false,
            scalar_v650: false,
            scalar_v651: 0.0,
            scalar_v653: 0.0,
            scalar_v655: 0.0,
            scalar_v656: 0.0,
            scalar_v657: false,
            scalar_v658: false,
            scalar_v659: 0.0,
            scalar_v661: 0.0,
            scalar_v663: 0.0,
            scalar_v664: 0.0,
            scalar_v665: false,
            scalar_v666: false,
            scalar_v667: 0.0,
            scalar_v669: 0.0,
            scalar_v841: 0.0,
            scalar_v852: 0.0,
            scalar_v876: 0.0,
            scalar_v963: 0.0,
            scalar_v964: 0.0,
            scalar_v971: 0.0,
            scalar_v972: 0.0,
            scalar_v983: 0.0,
            scalar_v1006: 0.0,
            scalar_v1010: 0.0,
            scalar_v1037: 0.0,
            scalar_v1038: 0.0,
            scalar_v1060: 0.0,
            scalar_v1077: 0.0,
            scalar_v1078: 0.0,
            scalar_v1079: 0.0,
            scalar_v1081: 0.0,
            scalar_v1082: 0.0,
            scalar_v1083: 0.0,
            scalar_v1104: 0.0,
            scalar_v1118: 0.0,
            scalar_v1119: 0.0,
            scalar_v1125: 0.0,
            scalar_v1150: 0.0,
            scalar_v1151: 0.0,
            scalar_v1152: 0.0,
            scalar_v1173: 0.0,
            scalar_v1271: 0.0,
            scalar_v1330: 0.0,
            scalar_v1470: 0.0,
            scalar_v1559: 0.0,
            scalar_v1579: 0.0,
            scalar_v1582: 0.0,
            scalar_v1583: 0.0,
            scalar_v1596: 0.0,
            scalar_v1607: 0.0,
            scalar_v1608: 0.0,
            scalar_v1609: 0.0,
            scalar_v1610: 0.0,
            scalar_v1611: 0.0,
            scalar_v1612: 0.0,
            scalar_v1613: 0.0,
            scalar_v1614: 0.0,
            scalar_v1743: 0.0,
            scalar_v1759: 0.0,
            scalar_v1848: 0.0,
            scalar_v1851: 0.0,
            scalar_v1975: 0.0,
            scalar_v1994: 0.0,
            scalar_v2005: 0.0,
            scalar_v2007: 0.0,
            scalar_v2008: 0.0,
            scalar_v2076: 0.0,
            scalar_v2077: 0.0,
            scalar_v2080: 0.0,
            scalar_v2081: 0.0,
            scalar_v2082: 0.0,
            scalar_v2094: 0.0,
            scalar_v2095: 0.0,
            scalar_v2096: 0.0,
            scalar_v2097: 0.0,
            scalar_v2103: 0.0,
            scalar_v2126: 0.0,
            scalar_v2156: 0.0,
            scalar_v2177: 0.0,
            scalar_v2367: 0.0,
            scalar_v2368: 0.0,
            scalar_v2377: 0.0,
            scalar_v2378: 0.0,
            scalar_v2387: 0.0,
            scalar_v2388: 0.0,
            scalar_v2413: 0.0,
            scalar_v3023: 0.0,
            scalar_v3024: 0.0,
            scalar_v3035: 0.0,
            scalar_v3036: 0.0,
            scalar_v3188: 0.0,
            scalar_v3189: 0.0,
            scalar_v3206: 0.0,
            scalar_v3439: 0.0,
            scalar_v3440: 0.0,
            scalar_v3572: 0.0,
            scalar_v3573: 0.0,
            scalar_v3629: 0.0,
            scalar_v3630: 0.0,
            scalar_v3644: 0.0,
            scalar_v3645: 0.0,
            scalar_v3659: 0.0,
            scalar_v3660: 0.0,
            scalar_v3661: 0.0,
            scalar_v3662: 0.0,
            scalar_v3686: 0.0,
            scalar_v3687: 0.0,
            scalar_v3728: 0.0,
            scalar_v3729: 0.0,
            scalar_v3780: 0.0,
            scalar_v3781: 0.0,
            scalar_v3870: 0.0,
            scalar_v3871: 0.0,
            scalar_v3872: 0.0,
            scalar_v3873: 0.0,
            scalar_v3951: 0.0,
            scalar_v3952: 0.0,
            scalar_v5295: 0.0,
            scalar_v5296: 0.0,
            scalar_v5548: 0.0,
            scalar_v5549: 0.0,
            scalar_v5550: 0.0,
            scalar_v5551: 0.0,
            scalar_v5572: 0.0,
            scalar_v5573: 0.0,
            scalar_v5574: 0.0,
            scalar_v5575: 0.0,
            scalar_v5634: 0.0,
            scalar_v5635: 0.0,
            scalar_v5652: 0.0,
            scalar_v5673: 0.0,
            scalar_v5732: 0.0,
            scalar_v5749: 0.0,
            scalar_v5750: 0.0,
            scalar_v5810: 0.0,
            scalar_v5811: 0.0,
            scalar_v5812: 0.0,
            scalar_v5813: 0.0,
            scalar_v6048: 0.0,
            scalar_v6049: 0.0,
            scalar_v6059: 0.0,
            scalar_v6060: 0.0,
            scalar_v6481: 0.0,
            scalar_v6482: 0.0,
            scalar_v6483: 0.0,
            scalar_v6484: 0.0,
            scalar_v6485: 0.0,
            scalar_v6486: 0.0,
            scalar_v6487: 0.0,
            scalar_v6488: 0.0,
            scalar_v6582: 0.0,
            scalar_v6583: 0.0,
            scalar_v6584: 0.0,
            scalar_v6585: 0.0,
            scalar_v6586: 0.0,
            scalar_v6587: 0.0,
            scalar_v6588: 0.0,
            scalar_v6589: 0.0,
            scalar_v6649: 0.0,
            scalar_v6650: 0.0,
            scalar_v6651: 0.0,
            scalar_v6652: 0.0,
            scalar_v6653: 0.0,
            scalar_v6654: 0.0,
            scalar_v6655: 0.0,
            scalar_v6656: 0.0,
            scalar_v6657: 0.0,
            scalar_v6658: 0.0,
            scalar_v6659: 0.0,
            scalar_v6660: 0.0,
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
        };
        instance.recompute_instance_static();
        instance
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
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
            scalar_v104,
            scalar_v106,
            scalar_v161,
            scalar_v181,
            scalar_v184,
            scalar_v204,
            scalar_v245,
            scalar_v248,
            scalar_v274,
            scalar_v276,
            scalar_v277,
            scalar_v283,
            scalar_v286,
            scalar_v287,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v307,
            scalar_v308,
            scalar_v312,
            scalar_v313,
            scalar_v317,
            scalar_v319,
            scalar_v320,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v354,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v385,
            scalar_v387,
            scalar_v388,
            scalar_v406,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v416,
            scalar_v421,
            scalar_v422,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v432,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v440,
            scalar_v441,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v453,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v462,
            scalar_v466,
            scalar_v467,
            scalar_v472,
            scalar_v473,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v511,
            scalar_v512,
            scalar_v516,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v535,
            scalar_v538,
            scalar_v546,
            scalar_v555,
            scalar_v568,
            scalar_v577,
            scalar_v589,
            scalar_v592,
            scalar_v595,
            scalar_v596,
            scalar_v600,
            scalar_v601,
            scalar_v605,
            scalar_v606,
            scalar_v607,
            scalar_v611,
            scalar_v612,
            scalar_v616,
            scalar_v619,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v642,
            scalar_v644,
            scalar_v646,
            scalar_v652,
            scalar_v654,
            scalar_v660,
            scalar_v662,
            scalar_v668,
            scalar_v713,
            scalar_v718,
            scalar_v818,
            scalar_v871,
            scalar_v872,
            scalar_v873,
            scalar_v884,
            scalar_v905,
            scalar_v906,
            scalar_v907,
            scalar_v908,
            scalar_v909,
            scalar_v910,
            scalar_v957,
            scalar_v967,
            scalar_v980,
            scalar_v981,
            scalar_v985,
            scalar_v1034,
            scalar_v1035,
            scalar_v1036,
            scalar_v1058,
            scalar_v1066,
            scalar_v1067,
            scalar_v1069,
            scalar_v1070,
            scalar_v1071,
            scalar_v1074,
            scalar_v1075,
            scalar_v1080,
            scalar_v1101,
            scalar_v1103,
            scalar_v1132,
            scalar_v1138,
            scalar_v1172,
            scalar_v1194,
            scalar_v1207,
            scalar_v1225,
            scalar_v1288,
            scalar_v1289,
            scalar_v1290,
            scalar_v1291,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1388,
            scalar_v1389,
            scalar_v1390,
            scalar_v1414,
            scalar_v1416,
            scalar_v1417,
            scalar_v1419,
            scalar_v1479,
            scalar_v1480,
            scalar_v1481,
            scalar_v1507,
            scalar_v1509,
            scalar_v1510,
            scalar_v1512,
            scalar_v1589,
            scalar_v1590,
            scalar_v1591,
            scalar_v1592,
            scalar_v1595,
            scalar_v1605,
            scalar_v1606,
            scalar_v1618,
            scalar_v1623,
            scalar_v1639,
            scalar_v1640,
            scalar_v1644,
            scalar_v1645,
            scalar_v1648,
            scalar_v1654,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1671,
            scalar_v1672,
            scalar_v1673,
            scalar_v1674,
            scalar_v1675,
            scalar_v1676,
            scalar_v1677,
            scalar_v1678,
            scalar_v1679,
            scalar_v1693,
            scalar_v1720,
            scalar_v1721,
            scalar_v1722,
            scalar_v1725,
            scalar_v1744,
            scalar_v1758,
            scalar_v1763,
            scalar_v1765,
            scalar_v1769,
            scalar_v1770,
            scalar_v1771,
            scalar_v1772,
            scalar_v1773,
            scalar_v1782,
            scalar_v1783,
            scalar_v1786,
            scalar_v1809,
            scalar_v1810,
            scalar_v1816,
            scalar_v1817,
            scalar_v1818,
            scalar_v1866,
            scalar_v1867,
            scalar_v1872,
            scalar_v1876,
            scalar_v1883,
            scalar_v1888,
            scalar_v1908,
            scalar_v1928,
            scalar_v1929,
            scalar_v1964,
            scalar_v1970,
            scalar_v1973,
            scalar_v1974,
            scalar_v2004,
            scalar_v2042,
            scalar_v2078,
            scalar_v2079,
            scalar_v2101,
            scalar_v2102,
            scalar_v2111,
            scalar_v2115,
            scalar_v2134,
            scalar_v2135,
            scalar_v2136,
            scalar_v2139,
            scalar_v2155,
            scalar_v2166,
            scalar_v2187,
            scalar_v2188,
            scalar_v2189,
            scalar_v2227,
            scalar_v2228,
            scalar_v2234,
            scalar_v2238,
            scalar_v2241,
            scalar_v2247,
            scalar_v2248,
            scalar_v2252,
            scalar_v2262,
            scalar_v2263,
            scalar_v2266,
            scalar_v2267,
            scalar_v2268,
            scalar_v2269,
            scalar_v2272,
            scalar_v2273,
            scalar_v2288,
            scalar_v2289,
            scalar_v2329,
            scalar_v2333,
            scalar_v2363,
            scalar_v2364,
            scalar_v2365,
            scalar_v2366,
            scalar_v3051,
            scalar_v3066,
            scalar_v3067,
            scalar_v3134,
            scalar_v3146,
            scalar_v3367,
            scalar_v3368,
            scalar_v3377,
            scalar_v3378,
            scalar_v3401,
            scalar_v3402,
            scalar_v3413,
            scalar_v3414,
            scalar_v3739,
            scalar_v3778,
            scalar_v3779,
            scalar_v3822,
            scalar_v3823,
            scalar_v3910,
            scalar_v3949,
            scalar_v3950,
            scalar_v4163,
            scalar_v4164,
            scalar_v4165,
            scalar_v4166,
            scalar_v4342,
            scalar_v4343,
            scalar_v4344,
            scalar_v4345,
            scalar_v4346,
            scalar_v4347,
            scalar_v4711,
            scalar_v5173,
            scalar_v5252,
            scalar_v5806,
            scalar_v5807,
            scalar_v5808,
            scalar_v5809,
            scalar_v6041,
            scalar_v6131,
            scalar_v6132,
            scalar_v6332,
            scalar_v6333,
            scalar_v6334,
            scalar_v6335,
            scalar_v6479,
            scalar_v6480,
            scalar_v6550,
            scalar_v6551,
            scalar_v6556,
            scalar_v6557,
            scalar_v6580,
            scalar_v6581,
            scalar_v20,
            scalar_v101,
            scalar_v103,
            scalar_v105,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
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
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
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
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v182,
            scalar_v183,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
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
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
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
            scalar_v246,
            scalar_v247,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
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
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v275,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v284,
            scalar_v285,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v318,
            scalar_v321,
            scalar_v322,
            scalar_v323,
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
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v355,
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
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v386,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v393,
            scalar_v394,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v437,
            scalar_v438,
            scalar_v439,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v461,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v478,
            scalar_v479,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v524,
            scalar_v525,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v536,
            scalar_v537,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v544,
            scalar_v545,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v556,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v565,
            scalar_v566,
            scalar_v567,
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
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v590,
            scalar_v591,
            scalar_v593,
            scalar_v594,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v608,
            scalar_v609,
            scalar_v610,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v617,
            scalar_v618,
            scalar_v620,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v628,
            scalar_v630,
            scalar_v632,
            scalar_v633,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v643,
            scalar_v645,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v655,
            scalar_v656,
            scalar_v657,
            scalar_v658,
            scalar_v659,
            scalar_v661,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v666,
            scalar_v667,
            scalar_v669,
            scalar_v841,
            scalar_v852,
            scalar_v876,
            scalar_v963,
            scalar_v964,
            scalar_v971,
            scalar_v972,
            scalar_v983,
            scalar_v1006,
            scalar_v1010,
            scalar_v1037,
            scalar_v1038,
            scalar_v1060,
            scalar_v1077,
            scalar_v1078,
            scalar_v1079,
            scalar_v1081,
            scalar_v1082,
            scalar_v1083,
            scalar_v1104,
            scalar_v1118,
            scalar_v1119,
            scalar_v1125,
            scalar_v1150,
            scalar_v1151,
            scalar_v1152,
            scalar_v1173,
            scalar_v1271,
            scalar_v1330,
            scalar_v1470,
            scalar_v1559,
            scalar_v1579,
            scalar_v1582,
            scalar_v1583,
            scalar_v1596,
            scalar_v1607,
            scalar_v1608,
            scalar_v1609,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1743,
            scalar_v1759,
            scalar_v1848,
            scalar_v1851,
            scalar_v1975,
            scalar_v1994,
            scalar_v2005,
            scalar_v2007,
            scalar_v2008,
            scalar_v2076,
            scalar_v2077,
            scalar_v2080,
            scalar_v2081,
            scalar_v2082,
            scalar_v2094,
            scalar_v2095,
            scalar_v2096,
            scalar_v2097,
            scalar_v2103,
            scalar_v2126,
            scalar_v2156,
            scalar_v2177,
            scalar_v2367,
            scalar_v2368,
            scalar_v2377,
            scalar_v2378,
            scalar_v2387,
            scalar_v2388,
            scalar_v2413,
            scalar_v3023,
            scalar_v3024,
            scalar_v3035,
            scalar_v3036,
            scalar_v3188,
            scalar_v3189,
            scalar_v3206,
            scalar_v3439,
            scalar_v3440,
            scalar_v3572,
            scalar_v3573,
            scalar_v3629,
            scalar_v3630,
            scalar_v3644,
            scalar_v3645,
            scalar_v3659,
            scalar_v3660,
            scalar_v3661,
            scalar_v3662,
            scalar_v3686,
            scalar_v3687,
            scalar_v3728,
            scalar_v3729,
            scalar_v3780,
            scalar_v3781,
            scalar_v3870,
            scalar_v3871,
            scalar_v3872,
            scalar_v3873,
            scalar_v3951,
            scalar_v3952,
            scalar_v5295,
            scalar_v5296,
            scalar_v5548,
            scalar_v5549,
            scalar_v5550,
            scalar_v5551,
            scalar_v5572,
            scalar_v5573,
            scalar_v5574,
            scalar_v5575,
            scalar_v5634,
            scalar_v5635,
            scalar_v5652,
            scalar_v5673,
            scalar_v5732,
            scalar_v5749,
            scalar_v5750,
            scalar_v5810,
            scalar_v5811,
            scalar_v5812,
            scalar_v5813,
            scalar_v6048,
            scalar_v6049,
            scalar_v6059,
            scalar_v6060,
            scalar_v6481,
            scalar_v6482,
            scalar_v6483,
            scalar_v6484,
            scalar_v6485,
            scalar_v6486,
            scalar_v6487,
            scalar_v6488,
            scalar_v6582,
            scalar_v6583,
            scalar_v6584,
            scalar_v6585,
            scalar_v6586,
            scalar_v6587,
            scalar_v6588,
            scalar_v6589,
            scalar_v6649,
            scalar_v6650,
            scalar_v6651,
            scalar_v6652,
            scalar_v6653,
            scalar_v6654,
            scalar_v6655,
            scalar_v6656,
            scalar_v6657,
            scalar_v6658,
            scalar_v6659,
            scalar_v6660,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
        } = snapshot;
        *self = Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_older,
            ddt_state_initialized,
            ddt_derivative_current,
            ddt_derivative_previous,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            ddt_coefficients,
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
            scalar_v104,
            scalar_v106,
            scalar_v161,
            scalar_v181,
            scalar_v184,
            scalar_v204,
            scalar_v245,
            scalar_v248,
            scalar_v274,
            scalar_v276,
            scalar_v277,
            scalar_v283,
            scalar_v286,
            scalar_v287,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v300,
            scalar_v301,
            scalar_v307,
            scalar_v308,
            scalar_v312,
            scalar_v313,
            scalar_v317,
            scalar_v319,
            scalar_v320,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v354,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v385,
            scalar_v387,
            scalar_v388,
            scalar_v406,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v416,
            scalar_v421,
            scalar_v422,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v432,
            scalar_v434,
            scalar_v435,
            scalar_v436,
            scalar_v440,
            scalar_v441,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v453,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v462,
            scalar_v466,
            scalar_v467,
            scalar_v472,
            scalar_v473,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v511,
            scalar_v512,
            scalar_v516,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v535,
            scalar_v538,
            scalar_v546,
            scalar_v555,
            scalar_v568,
            scalar_v577,
            scalar_v589,
            scalar_v592,
            scalar_v595,
            scalar_v596,
            scalar_v600,
            scalar_v601,
            scalar_v605,
            scalar_v606,
            scalar_v607,
            scalar_v611,
            scalar_v612,
            scalar_v616,
            scalar_v619,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v642,
            scalar_v644,
            scalar_v646,
            scalar_v652,
            scalar_v654,
            scalar_v660,
            scalar_v662,
            scalar_v668,
            scalar_v713,
            scalar_v718,
            scalar_v818,
            scalar_v871,
            scalar_v872,
            scalar_v873,
            scalar_v884,
            scalar_v905,
            scalar_v906,
            scalar_v907,
            scalar_v908,
            scalar_v909,
            scalar_v910,
            scalar_v957,
            scalar_v967,
            scalar_v980,
            scalar_v981,
            scalar_v985,
            scalar_v1034,
            scalar_v1035,
            scalar_v1036,
            scalar_v1058,
            scalar_v1066,
            scalar_v1067,
            scalar_v1069,
            scalar_v1070,
            scalar_v1071,
            scalar_v1074,
            scalar_v1075,
            scalar_v1080,
            scalar_v1101,
            scalar_v1103,
            scalar_v1132,
            scalar_v1138,
            scalar_v1172,
            scalar_v1194,
            scalar_v1207,
            scalar_v1225,
            scalar_v1288,
            scalar_v1289,
            scalar_v1290,
            scalar_v1291,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1388,
            scalar_v1389,
            scalar_v1390,
            scalar_v1414,
            scalar_v1416,
            scalar_v1417,
            scalar_v1419,
            scalar_v1479,
            scalar_v1480,
            scalar_v1481,
            scalar_v1507,
            scalar_v1509,
            scalar_v1510,
            scalar_v1512,
            scalar_v1589,
            scalar_v1590,
            scalar_v1591,
            scalar_v1592,
            scalar_v1595,
            scalar_v1605,
            scalar_v1606,
            scalar_v1618,
            scalar_v1623,
            scalar_v1639,
            scalar_v1640,
            scalar_v1644,
            scalar_v1645,
            scalar_v1648,
            scalar_v1654,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1671,
            scalar_v1672,
            scalar_v1673,
            scalar_v1674,
            scalar_v1675,
            scalar_v1676,
            scalar_v1677,
            scalar_v1678,
            scalar_v1679,
            scalar_v1693,
            scalar_v1720,
            scalar_v1721,
            scalar_v1722,
            scalar_v1725,
            scalar_v1744,
            scalar_v1758,
            scalar_v1763,
            scalar_v1765,
            scalar_v1769,
            scalar_v1770,
            scalar_v1771,
            scalar_v1772,
            scalar_v1773,
            scalar_v1782,
            scalar_v1783,
            scalar_v1786,
            scalar_v1809,
            scalar_v1810,
            scalar_v1816,
            scalar_v1817,
            scalar_v1818,
            scalar_v1866,
            scalar_v1867,
            scalar_v1872,
            scalar_v1876,
            scalar_v1883,
            scalar_v1888,
            scalar_v1908,
            scalar_v1928,
            scalar_v1929,
            scalar_v1964,
            scalar_v1970,
            scalar_v1973,
            scalar_v1974,
            scalar_v2004,
            scalar_v2042,
            scalar_v2078,
            scalar_v2079,
            scalar_v2101,
            scalar_v2102,
            scalar_v2111,
            scalar_v2115,
            scalar_v2134,
            scalar_v2135,
            scalar_v2136,
            scalar_v2139,
            scalar_v2155,
            scalar_v2166,
            scalar_v2187,
            scalar_v2188,
            scalar_v2189,
            scalar_v2227,
            scalar_v2228,
            scalar_v2234,
            scalar_v2238,
            scalar_v2241,
            scalar_v2247,
            scalar_v2248,
            scalar_v2252,
            scalar_v2262,
            scalar_v2263,
            scalar_v2266,
            scalar_v2267,
            scalar_v2268,
            scalar_v2269,
            scalar_v2272,
            scalar_v2273,
            scalar_v2288,
            scalar_v2289,
            scalar_v2329,
            scalar_v2333,
            scalar_v2363,
            scalar_v2364,
            scalar_v2365,
            scalar_v2366,
            scalar_v3051,
            scalar_v3066,
            scalar_v3067,
            scalar_v3134,
            scalar_v3146,
            scalar_v3367,
            scalar_v3368,
            scalar_v3377,
            scalar_v3378,
            scalar_v3401,
            scalar_v3402,
            scalar_v3413,
            scalar_v3414,
            scalar_v3739,
            scalar_v3778,
            scalar_v3779,
            scalar_v3822,
            scalar_v3823,
            scalar_v3910,
            scalar_v3949,
            scalar_v3950,
            scalar_v4163,
            scalar_v4164,
            scalar_v4165,
            scalar_v4166,
            scalar_v4342,
            scalar_v4343,
            scalar_v4344,
            scalar_v4345,
            scalar_v4346,
            scalar_v4347,
            scalar_v4711,
            scalar_v5173,
            scalar_v5252,
            scalar_v5806,
            scalar_v5807,
            scalar_v5808,
            scalar_v5809,
            scalar_v6041,
            scalar_v6131,
            scalar_v6132,
            scalar_v6332,
            scalar_v6333,
            scalar_v6334,
            scalar_v6335,
            scalar_v6479,
            scalar_v6480,
            scalar_v6550,
            scalar_v6551,
            scalar_v6556,
            scalar_v6557,
            scalar_v6580,
            scalar_v6581,
            scalar_v20,
            scalar_v101,
            scalar_v103,
            scalar_v105,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
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
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
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
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v182,
            scalar_v183,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
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
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
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
            scalar_v246,
            scalar_v247,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
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
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v275,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v284,
            scalar_v285,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v318,
            scalar_v321,
            scalar_v322,
            scalar_v323,
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
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v355,
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
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v386,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v393,
            scalar_v394,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v437,
            scalar_v438,
            scalar_v439,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v461,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v478,
            scalar_v479,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v524,
            scalar_v525,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v536,
            scalar_v537,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v544,
            scalar_v545,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v556,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v565,
            scalar_v566,
            scalar_v567,
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
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v590,
            scalar_v591,
            scalar_v593,
            scalar_v594,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v608,
            scalar_v609,
            scalar_v610,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v617,
            scalar_v618,
            scalar_v620,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v628,
            scalar_v630,
            scalar_v632,
            scalar_v633,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v643,
            scalar_v645,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v655,
            scalar_v656,
            scalar_v657,
            scalar_v658,
            scalar_v659,
            scalar_v661,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v666,
            scalar_v667,
            scalar_v669,
            scalar_v841,
            scalar_v852,
            scalar_v876,
            scalar_v963,
            scalar_v964,
            scalar_v971,
            scalar_v972,
            scalar_v983,
            scalar_v1006,
            scalar_v1010,
            scalar_v1037,
            scalar_v1038,
            scalar_v1060,
            scalar_v1077,
            scalar_v1078,
            scalar_v1079,
            scalar_v1081,
            scalar_v1082,
            scalar_v1083,
            scalar_v1104,
            scalar_v1118,
            scalar_v1119,
            scalar_v1125,
            scalar_v1150,
            scalar_v1151,
            scalar_v1152,
            scalar_v1173,
            scalar_v1271,
            scalar_v1330,
            scalar_v1470,
            scalar_v1559,
            scalar_v1579,
            scalar_v1582,
            scalar_v1583,
            scalar_v1596,
            scalar_v1607,
            scalar_v1608,
            scalar_v1609,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1743,
            scalar_v1759,
            scalar_v1848,
            scalar_v1851,
            scalar_v1975,
            scalar_v1994,
            scalar_v2005,
            scalar_v2007,
            scalar_v2008,
            scalar_v2076,
            scalar_v2077,
            scalar_v2080,
            scalar_v2081,
            scalar_v2082,
            scalar_v2094,
            scalar_v2095,
            scalar_v2096,
            scalar_v2097,
            scalar_v2103,
            scalar_v2126,
            scalar_v2156,
            scalar_v2177,
            scalar_v2367,
            scalar_v2368,
            scalar_v2377,
            scalar_v2378,
            scalar_v2387,
            scalar_v2388,
            scalar_v2413,
            scalar_v3023,
            scalar_v3024,
            scalar_v3035,
            scalar_v3036,
            scalar_v3188,
            scalar_v3189,
            scalar_v3206,
            scalar_v3439,
            scalar_v3440,
            scalar_v3572,
            scalar_v3573,
            scalar_v3629,
            scalar_v3630,
            scalar_v3644,
            scalar_v3645,
            scalar_v3659,
            scalar_v3660,
            scalar_v3661,
            scalar_v3662,
            scalar_v3686,
            scalar_v3687,
            scalar_v3728,
            scalar_v3729,
            scalar_v3780,
            scalar_v3781,
            scalar_v3870,
            scalar_v3871,
            scalar_v3872,
            scalar_v3873,
            scalar_v3951,
            scalar_v3952,
            scalar_v5295,
            scalar_v5296,
            scalar_v5548,
            scalar_v5549,
            scalar_v5550,
            scalar_v5551,
            scalar_v5572,
            scalar_v5573,
            scalar_v5574,
            scalar_v5575,
            scalar_v5634,
            scalar_v5635,
            scalar_v5652,
            scalar_v5673,
            scalar_v5732,
            scalar_v5749,
            scalar_v5750,
            scalar_v5810,
            scalar_v5811,
            scalar_v5812,
            scalar_v5813,
            scalar_v6048,
            scalar_v6049,
            scalar_v6059,
            scalar_v6060,
            scalar_v6481,
            scalar_v6482,
            scalar_v6483,
            scalar_v6484,
            scalar_v6485,
            scalar_v6486,
            scalar_v6487,
            scalar_v6488,
            scalar_v6582,
            scalar_v6583,
            scalar_v6584,
            scalar_v6585,
            scalar_v6586,
            scalar_v6587,
            scalar_v6588,
            scalar_v6589,
            scalar_v6649,
            scalar_v6650,
            scalar_v6651,
            scalar_v6652,
            scalar_v6653,
            scalar_v6654,
            scalar_v6655,
            scalar_v6656,
            scalar_v6657,
            scalar_v6658,
            scalar_v6659,
            scalar_v6660,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
    pub fn set_timepoint(&mut self, time: f64, timestep: f64, ddt_coefficients: GeneratedDdtCoefficients) {
        self.time = time;
        self.timestep = timestep;
        self.ddt_coefficients = ddt_coefficients;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_older[index] = self.ddt_state_previous[index];
            self.ddt_state_previous[index] = self.ddt_state_current[index];
            self.ddt_derivative_previous[index] = self.ddt_derivative_current[index];
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
        let older = if self.ddt_state_initialized[slot] {
            self.ddt_state_older[slot]
        } else {
            value
        };
        self.ddt_state_current[slot] = value;
        if self.ddt_coefficients.active {
            let result = value * self.ddt_coefficients.derivative_scale
                - previous * self.ddt_coefficients.previous_value_scale
                - older * self.ddt_coefficients.older_value_scale
                - self.ddt_derivative_previous[slot] * self.ddt_coefficients.previous_derivative_scale;
            self.ddt_derivative_current[slot] = result;
            result
        } else {
            self.ddt_state_current[slot] = value;
            self.ddt_state_previous[slot] = value;
            self.ddt_state_older[slot] = value;
            self.ddt_derivative_current[slot] = 0.0;
            self.ddt_derivative_previous[slot] = 0.0;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.ddt_coefficients.active {
            derivative * self.ddt_coefficients.derivative_scale
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
        let v22: bool = (0.0 == p.p137);
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
        let v38: f64 = (v17 * p.p114);
        self.scalar_v38 = v38;
        let v39: f64 = (v17 * v38);
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
        let v49: f64 = ((v47) as f64).exp();
        self.scalar_v49 = v49;
        let v50: f64 = (1.0 + v49);
        self.scalar_v50 = v50;
        let v51: f64 = ((v50) as f64).ln();
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
        let v57: f64 = ((v56) as f64).exp();
        self.scalar_v57 = v57;
        let v58: f64 = (1.0 + v57);
        self.scalar_v58 = v58;
        let v59: f64 = ((v58) as f64).ln();
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
        let v73: f64 = (v17 * p.p117);
        self.scalar_v73 = v73;
        let v74: f64 = (v17 * v73);
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
        let v82: f64 = ((v80) as f64).exp();
        self.scalar_v82 = v82;
        let v83: f64 = (1.0 + v82);
        self.scalar_v83 = v83;
        let v84: f64 = ((v83) as f64).ln();
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
        let v90: f64 = ((v89) as f64).exp();
        self.scalar_v90 = v90;
        let v91: f64 = (1.0 + v90);
        self.scalar_v91 = v91;
        let v92: f64 = ((v91) as f64).ln();
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
        let v104: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v104 = v104;
        let v106: f64 = (1.0 / v104);
        self.scalar_v106 = v106;
        let v161: f64 = p.p104;
        self.scalar_v161 = v161;
        let v181: f64 = p.p63;
        self.scalar_v181 = v181;
        let v184: f64 = p.p109;
        self.scalar_v184 = v184;
        let v204: f64 = p.p79;
        self.scalar_v204 = v204;
        let v245: f64 = p.p26;
        self.scalar_v245 = v245;
        let v248: f64 = p.p108;
        self.scalar_v248 = v248;
        let v274: f64 = p.p64;
        self.scalar_v274 = v274;
        let v276: f64 = p.p74;
        self.scalar_v276 = v276;
        let v277: f64 = (1.0 - p.p74);
        self.scalar_v277 = v277;
        let v283: f64 = p.p69;
        self.scalar_v283 = v283;
        let v286: f64 = p.p53;
        self.scalar_v286 = v286;
        let v287: f64 = p.p96;
        self.scalar_v287 = v287;
        let v293: f64 = p.p55;
        self.scalar_v293 = v293;
        let v294: f64 = p.p97;
        self.scalar_v294 = v294;
        let v295: f64 = p.p95;
        self.scalar_v295 = v295;
        let v296: f64 = (p.p97 - p.p95);
        self.scalar_v296 = v296;
        let v300: f64 = p.p54;
        self.scalar_v300 = v300;
        let v301: f64 = p.p100;
        self.scalar_v301 = v301;
        let v307: f64 = p.p56;
        self.scalar_v307 = v307;
        let v308: f64 = p.p101;
        self.scalar_v308 = v308;
        let v312: f64 = p.p57;
        self.scalar_v312 = v312;
        let v313: f64 = p.p103;
        self.scalar_v313 = v313;
        let v317: f64 = p.p58;
        self.scalar_v317 = v317;
        let v319: f64 = p.p59;
        self.scalar_v319 = v319;
        let v320: f64 = p.p98;
        self.scalar_v320 = v320;
        let v324: f64 = p.p121;
        self.scalar_v324 = v324;
        let v325: bool = (0.0 != p.p121);
        self.scalar_v325 = v325;
        let v326: f64 = p.p9;
        self.scalar_v326 = v326;
        let v354: bool = (!v325);
        self.scalar_v354 = v354;
        let v356: f64 = p.p122;
        self.scalar_v356 = v356;
        let v357: bool = (0.0 != p.p122);
        self.scalar_v357 = v357;
        let v358: f64 = p.p10;
        self.scalar_v358 = v358;
        let v385: bool = (!v357);
        self.scalar_v385 = v385;
        let v387: f64 = p.p42;
        self.scalar_v387 = v387;
        let v388: f64 = p.p123;
        self.scalar_v388 = v388;
        let v406: f64 = p.p8;
        self.scalar_v406 = v406;
        let v408: f64 = (4.0 - p.p97);
        self.scalar_v408 = v408;
        let v409: f64 = (v408 - p.p95);
        self.scalar_v409 = v409;
        let v410: f64 = p.p120;
        self.scalar_v410 = v410;
        let v411: f64 = (v409 + p.p120);
        self.scalar_v411 = v411;
        let v416: f64 = (-p.p104);
        self.scalar_v416 = v416;
        let v421: f64 = p.p11;
        self.scalar_v421 = v421;
        let v422: f64 = (1.0 - p.p97);
        self.scalar_v422 = v422;
        let v426: f64 = p.p29;
        self.scalar_v426 = v426;
        let v427: f64 = p.p102;
        self.scalar_v427 = v427;
        let v428: f64 = (1.0 - p.p102);
        self.scalar_v428 = v428;
        let v432: f64 = p.p19;
        self.scalar_v432 = v432;
        let v434: f64 = p.p20;
        self.scalar_v434 = v434;
        let v435: f64 = (2.0 * p.p20);
        self.scalar_v435 = v435;
        let v436: f64 = (6.0 - v435);
        self.scalar_v436 = v436;
        let v440: f64 = p.p112;
        self.scalar_v440 = v440;
        let v441: f64 = (-p.p112);
        self.scalar_v441 = v441;
        let v446: f64 = p.p30;
        self.scalar_v446 = v446;
        let v447: f64 = p.p31;
        self.scalar_v447 = v447;
        let v448: f64 = (2.0 * p.p31);
        self.scalar_v448 = v448;
        let v449: f64 = (6.0 - v448);
        self.scalar_v449 = v449;
        let v453: f64 = (-p.p109);
        self.scalar_v453 = v453;
        let v458: f64 = p.p15;
        self.scalar_v458 = v458;
        let v459: f64 = (4.0 - p.p96);
        self.scalar_v459 = v459;
        let v460: f64 = (p.p120 + v459);
        self.scalar_v460 = v460;
        let v462: f64 = p.p16;
        self.scalar_v462 = v462;
        let v466: f64 = p.p110;
        self.scalar_v466 = v466;
        let v467: f64 = (-p.p110);
        self.scalar_v467 = v467;
        let v472: f64 = p.p17;
        self.scalar_v472 = v472;
        let v473: f64 = p.p18;
        self.scalar_v473 = v473;
        let v480: f64 = p.p23;
        self.scalar_v480 = v480;
        let v481: bool = (1.0 == p.p23);
        self.scalar_v481 = v481;
        let v482: f64 = p.p24;
        self.scalar_v482 = v482;
        let v483: f64 = p.p106;
        self.scalar_v483 = v483;
        let v484: f64 = (-p.p106);
        self.scalar_v484 = v484;
        let v490: f64 = p.p27;
        self.scalar_v490 = v490;
        let v491: f64 = p.p105;
        self.scalar_v491 = v491;
        let v492: f64 = (-p.p105);
        self.scalar_v492 = v492;
        let v497: f64 = p.p25;
        self.scalar_v497 = v497;
        let v498: f64 = p.p107;
        self.scalar_v498 = v498;
        let v499: f64 = (-p.p107);
        self.scalar_v499 = v499;
        let v505: f64 = p.p28;
        self.scalar_v505 = v505;
        let v506: f64 = (4.0 - p.p102);
        self.scalar_v506 = v506;
        let v507: f64 = (p.p120 + v506);
        self.scalar_v507 = v507;
        let v511: f64 = p.p111;
        self.scalar_v511 = v511;
        let v512: f64 = (-p.p111);
        self.scalar_v512 = v512;
        let v516: f64 = p.p21;
        self.scalar_v516 = v516;
        let v517: f64 = p.p22;
        self.scalar_v517 = v517;
        let v518: f64 = (2.0 * p.p22);
        self.scalar_v518 = v518;
        let v519: f64 = (6.0 - v518);
        self.scalar_v519 = v519;
        let v526: f64 = p.p132;
        self.scalar_v526 = v526;
        let v527: f64 = p.p133;
        self.scalar_v527 = v527;
        let v528: f64 = (4.0 / p.p133);
        self.scalar_v528 = v528;
        let v535: f64 = p.p138;
        self.scalar_v535 = v535;
        let v538: f64 = p.p140;
        self.scalar_v538 = v538;
        let v546: f64 = p.p34;
        self.scalar_v546 = v546;
        let v555: f64 = p.p33;
        self.scalar_v555 = v555;
        let v568: f64 = p.p36;
        self.scalar_v568 = v568;
        let v577: f64 = p.p35;
        self.scalar_v577 = v577;
        let v589: f64 = p.p13;
        self.scalar_v589 = v589;
        let v592: f64 = p.p12;
        self.scalar_v592 = v592;
        let v595: f64 = p.p85;
        self.scalar_v595 = v595;
        let v596: f64 = (p.p97 - 2.0);
        self.scalar_v596 = v596;
        let v600: f64 = p.p119;
        self.scalar_v600 = v600;
        let v601: f64 = (-p.p119);
        self.scalar_v601 = v601;
        let v605: f64 = p.p86;
        self.scalar_v605 = v605;
        let v606: f64 = (p.p97 + p.p95);
        self.scalar_v606 = v606;
        let v607: f64 = (v606 - 1.0);
        self.scalar_v607 = v607;
        let v611: f64 = p.p87;
        self.scalar_v611 = v611;
        let v612: f64 = (p.p98 - 1.0);
        self.scalar_v612 = v612;
        let v616: f64 = p.p88;
        self.scalar_v616 = v616;
        let v619: f64 = (p.p86 + p.p87);
        self.scalar_v619 = v619;
        let v621: f64 = p.p89;
        self.scalar_v621 = v621;
        let v622: f64 = p.p99;
        self.scalar_v622 = v622;
        let v623: f64 = (p.p99 - 1.0);
        self.scalar_v623 = v623;
        let v642: f64 = (v12 * 1.081);
        self.scalar_v642 = v642;
        let v644: f64 = p.p91;
        self.scalar_v644 = v644;
        let v646: bool = (p.p56 > 0.0);
        self.scalar_v646 = v646;
        let v652: bool = (!v646);
        self.scalar_v652 = v652;
        let v654: bool = (p.p57 > 0.0);
        self.scalar_v654 = v654;
        let v660: bool = (!v654);
        self.scalar_v660 = v660;
        let v662: bool = (p.p58 > 0.0);
        self.scalar_v662 = v662;
        let v668: bool = (!v662);
        self.scalar_v668 = v668;
        let v713: f64 = p.p134;
        self.scalar_v713 = v713;
        let v718: f64 = ((p.p134) as f64).exp();
        self.scalar_v718 = v718;
        let v818: f64 = p.p136;
        self.scalar_v818 = v818;
        let v871: f64 = p.p61;
        self.scalar_v871 = v871;
        let v872: f64 = p.p60;
        self.scalar_v872 = v872;
        let v873: f64 = (p.p61 * p.p60);
        self.scalar_v873 = v873;
        let v884: f64 = p.p62;
        self.scalar_v884 = v884;
        let v905: f64 = (-1.0 / p.p62);
        self.scalar_v905 = v905;
        let v906: f64 = ((v905) as f64).exp();
        self.scalar_v906 = v906;
        let v907: f64 = (1.0 + v906);
        self.scalar_v907 = v907;
        let v908: f64 = ((v907) as f64).ln();
        self.scalar_v908 = v908;
        let v909: f64 = (p.p62 * v908);
        self.scalar_v909 = v909;
        let v910: f64 = (1.0 + v909);
        self.scalar_v910 = v910;
        let v957: f64 = p.p135;
        self.scalar_v957 = v957;
        let v967: f64 = (0.5 * p.p60);
        self.scalar_v967 = v967;
        let v980: f64 = p.p72;
        self.scalar_v980 = v980;
        let v981: bool = (0.0 == p.p72);
        self.scalar_v981 = v981;
        let v985: bool = (!v981);
        self.scalar_v985 = v985;
        let v1034: f64 = (-1.0 / p.p66);
        self.scalar_v1034 = v1034;
        let v1035: f64 = f64::powf(3.0, v1034);
        self.scalar_v1035 = v1035;
        let v1036: f64 = (1.0 - v1035);
        self.scalar_v1036 = v1036;
        let v1058: f64 = (1.0 - p.p66);
        self.scalar_v1058 = v1058;
        let v1066: f64 = p.p73;
        self.scalar_v1066 = v1066;
        let v1067: bool = (1.0 == p.p73);
        self.scalar_v1067 = v1067;
        let v1069: bool = (2.0 == p.p73);
        self.scalar_v1069 = v1069;
        let v1070: bool = (!v1067);
        self.scalar_v1070 = v1070;
        let v1071: bool = (v1069 && v1070);
        self.scalar_v1071 = v1071;
        let v1074: bool = (!v1069);
        self.scalar_v1074 = v1074;
        let v1075: bool = (v1070 && v1074);
        self.scalar_v1075 = v1075;
        let v1080: f64 = (-1.0 / p.p71);
        self.scalar_v1080 = v1080;
        let v1101: f64 = p.p75;
        self.scalar_v1101 = v1101;
        let v1103: f64 = (1.0 - p.p71);
        self.scalar_v1103 = v1103;
        let v1132: bool = (0.0 == p.p91);
        self.scalar_v1132 = v1132;
        let v1138: bool = (!v1132);
        self.scalar_v1138 = v1138;
        let v1172: f64 = p.p14;
        self.scalar_v1172 = v1172;
        let v1194: f64 = p.p139;
        self.scalar_v1194 = v1194;
        let v1207: f64 = p.p141;
        self.scalar_v1207 = v1207;
        let v1225: f64 = p.p142;
        self.scalar_v1225 = v1225;
        let v1288: f64 = p.p92;
        self.scalar_v1288 = v1288;
        let v1289: bool = (0.0 == p.p92);
        self.scalar_v1289 = v1289;
        let v1290: bool = (!v481);
        self.scalar_v1290 = v1290;
        let v1291: bool = (v1289 && v1290);
        self.scalar_v1291 = v1291;
        let v1293: bool = (!v1289);
        self.scalar_v1293 = v1293;
        let v1294: bool = (v1290 && v1293);
        self.scalar_v1294 = v1294;
        let v1295: f64 = (1.0 - p.p92);
        self.scalar_v1295 = v1295;
        let v1388: bool = (p.p33 > 0.0);
        self.scalar_v1388 = v1388;
        let v1389: bool = (p.p34 > 0.0);
        self.scalar_v1389 = v1389;
        let v1390: bool = (v1388 && v1389);
        self.scalar_v1390 = v1390;
        let v1414: f64 = (-2.0 - p.p66);
        self.scalar_v1414 = v1414;
        let v1416: f64 = (p.p66 * p.p66);
        self.scalar_v1416 = v1416;
        let v1417: f64 = (1.0 - v1416);
        self.scalar_v1417 = v1417;
        let v1419: f64 = (p.p66 - 1.0);
        self.scalar_v1419 = v1419;
        let v1479: bool = (p.p35 > 0.0);
        self.scalar_v1479 = v1479;
        let v1480: bool = (p.p36 > 0.0);
        self.scalar_v1480 = v1480;
        let v1481: bool = (v1479 && v1480);
        self.scalar_v1481 = v1481;
        let v1507: f64 = (-2.0 - p.p71);
        self.scalar_v1507 = v1507;
        let v1509: f64 = (p.p71 * p.p71);
        self.scalar_v1509 = v1509;
        let v1510: f64 = (1.0 - v1509);
        self.scalar_v1510 = v1510;
        let v1512: f64 = (p.p71 - 1.0);
        self.scalar_v1512 = v1512;
        let v1589: f64 = p.p5;
        self.scalar_v1589 = v1589;
        let v1590: bool = (p.p5 > 0.0);
        self.scalar_v1590 = v1590;
        let v1591: bool = (p.p32 > 0.0);
        self.scalar_v1591 = v1591;
        let v1592: bool = (v1590 && v1591);
        self.scalar_v1592 = v1592;
        let v1595: f64 = (p.p32 * 2.0);
        self.scalar_v1595 = v1595;
        let v1605: bool = (1.0 == p.p5);
        self.scalar_v1605 = v1605;
        let v1606: bool = (v1592 && v1605);
        self.scalar_v1606 = v1606;
        let v1618: f64 = (if v1606 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1618 = v1618;
        let v1623: f64 = (0.5 * v1618);
        self.scalar_v1623 = v1623;
        let v1639: bool = (!v1605);
        self.scalar_v1639 = v1639;
        let v1640: bool = (v1592 && v1639);
        self.scalar_v1640 = v1640;
        let v1644: f64 = p.p83;
        self.scalar_v1644 = v1644;
        let v1645: bool = (1.0 == p.p83);
        self.scalar_v1645 = v1645;
        let v1648: f64 = (if v1645 { 1e-12 } else { v1618 });
        self.scalar_v1648 = v1648;
        let v1654: f64 = (0.5 * v1648);
        self.scalar_v1654 = v1654;
        let v1665: f64 = p.p81;
        self.scalar_v1665 = v1665;
        let v1666: f64 = f64::powf(v100, p.p81);
        self.scalar_v1666 = v1666;
        let v1667: f64 = (1.0 - v1666);
        self.scalar_v1667 = v1667;
        let v1668: f64 = (1.0 / v1667);
        self.scalar_v1668 = v1668;
        let v1669: f64 = (if v1645 { v1668 } else { 0.0 });
        self.scalar_v1669 = v1669;
        let v1670: f64 = p.p80;
        self.scalar_v1670 = v1670;
        let v1671: f64 = (v100 * p.p80);
        self.scalar_v1671 = v1671;
        let v1672: f64 = (if v1645 { v1671 } else { 0.0 });
        self.scalar_v1672 = v1672;
        let v1673: f64 = (v1669 * v1669);
        self.scalar_v1673 = v1673;
        let v1674: f64 = (p.p81 - 1.0);
        self.scalar_v1674 = v1674;
        let v1675: f64 = f64::powf(v100, v1674);
        self.scalar_v1675 = v1675;
        let v1676: f64 = (v1673 * v1675);
        self.scalar_v1676 = v1676;
        let v1677: f64 = (p.p81 * v1676);
        self.scalar_v1677 = v1677;
        let v1678: f64 = (v1677 / p.p80);
        self.scalar_v1678 = v1678;
        let v1679: f64 = (if v1645 { v1678 } else { 0.0 });
        self.scalar_v1679 = v1679;
        let v1693: bool = (!v1645);
        self.scalar_v1693 = v1693;
        let v1720: f64 = p.p38;
        self.scalar_v1720 = v1720;
        let v1721: bool = (1.0 == p.p38);
        self.scalar_v1721 = v1721;
        let v1722: f64 = p.p43;
        self.scalar_v1722 = v1722;
        let v1725: f64 = p.p41;
        self.scalar_v1725 = v1725;
        let v1744: f64 = p.p40;
        self.scalar_v1744 = v1744;
        let v1758: f64 = p.p39;
        self.scalar_v1758 = v1758;
        let v1763: bool = (2.0 == p.p38);
        self.scalar_v1763 = v1763;
        let v1765: bool = (!v1721);
        self.scalar_v1765 = v1765;
        let v1769: f64 = p.p45;
        self.scalar_v1769 = v1769;
        let v1770: f64 = (2.0 * p.p45);
        self.scalar_v1770 = v1770;
        let v1771: f64 = p.p44;
        self.scalar_v1771 = v1771;
        let v1772: f64 = (p.p44 * p.p44);
        self.scalar_v1772 = v1772;
        let v1773: f64 = (v1770 / v1772);
        self.scalar_v1773 = v1773;
        let v1782: f64 = p.p7;
        self.scalar_v1782 = v1782;
        let v1783: bool = (0.0 == p.p7);
        self.scalar_v1783 = v1783;
        let v1786: bool = (!v1783);
        self.scalar_v1786 = v1786;
        let v1809: f64 = p.p46;
        self.scalar_v1809 = v1809;
        let v1810: f64 = (2.0 * p.p46);
        self.scalar_v1810 = v1810;
        let v1816: f64 = (1.0 + p.p46);
        self.scalar_v1816 = v1816;
        let v1817: f64 = (1.0 + v1810);
        self.scalar_v1817 = v1817;
        let v1818: f64 = (v1816 / v1817);
        self.scalar_v1818 = v1818;
        let v1866: bool = (3.0 == p.p38);
        self.scalar_v1866 = v1866;
        let v1867: bool = (!v1763);
        self.scalar_v1867 = v1867;
        let v1872: f64 = p.p47;
        self.scalar_v1872 = v1872;
        let v1876: f64 = p.p48;
        self.scalar_v1876 = v1876;
        let v1883: f64 = p.p51;
        self.scalar_v1883 = v1883;
        let v1888: f64 = p.p50;
        self.scalar_v1888 = v1888;
        let v1908: f64 = p.p49;
        self.scalar_v1908 = v1908;
        let v1928: f64 = p.p52;
        self.scalar_v1928 = v1928;
        let v1929: bool = (1.0 == p.p52);
        self.scalar_v1929 = v1929;
        let v1964: bool = (!v1866);
        self.scalar_v1964 = v1964;
        let v1970: bool = (!v1929);
        self.scalar_v1970 = v1970;
        let v1973: f64 = p.p67;
        self.scalar_v1973 = v1973;
        let v1974: f64 = (1.0 - p.p67);
        self.scalar_v1974 = v1974;
        let v2004: f64 = p.p76;
        self.scalar_v2004 = v2004;
        let v2042: f64 = (1.0 - p.p76);
        self.scalar_v2042 = v2042;
        let v2078: f64 = p.p84;
        self.scalar_v2078 = v2078;
        let v2079: f64 = (1.0 / p.p84);
        self.scalar_v2079 = v2079;
        let v2101: f64 = p.p78;
        self.scalar_v2101 = v2101;
        let v2102: bool = (0.0 == p.p78);
        self.scalar_v2102 = v2102;
        let v2111: f64 = p.p90;
        self.scalar_v2111 = v2111;
        let v2115: bool = (!v2102);
        self.scalar_v2115 = v2115;
        let v2134: bool = (3.0 == p.p5);
        self.scalar_v2134 = v2134;
        let v2135: bool = (v1605 || v2134);
        self.scalar_v2135 = v2135;
        let v2136: bool = (v1591 && v2135);
        self.scalar_v2136 = v2136;
        let v2139: bool = (v2102 && v2136);
        self.scalar_v2139 = v2139;
        let v2155: f64 = (p.p32 * 0.5);
        self.scalar_v2155 = v2155;
        let v2166: bool = (v2115 && v2136);
        self.scalar_v2166 = v2166;
        let v2187: f64 = p.p6;
        self.scalar_v2187 = v2187;
        let v2188: bool = (1.0 == p.p6);
        self.scalar_v2188 = v2188;
        let v2189: f64 = (-p.p66);
        self.scalar_v2189 = v2189;
        let v2227: f64 = p.p94;
        self.scalar_v2227 = v2227;
        let v2228: f64 = (1.0 - p.p94);
        self.scalar_v2228 = v2228;
        let v2234: f64 = p.p93;
        self.scalar_v2234 = v2234;
        let v2238: f64 = (1.0 - p.p93);
        self.scalar_v2238 = v2238;
        let v2241: bool = (!v2188);
        self.scalar_v2241 = v2241;
        let v2247: f64 = p.p129;
        self.scalar_v2247 = v2247;
        let v2248: bool = (p.p129 > 0.0);
        self.scalar_v2248 = v2248;
        let v2252: bool = (!v2248);
        self.scalar_v2252 = v2252;
        let v2262: f64 = p.p130;
        self.scalar_v2262 = v2262;
        let v2263: bool = (1.0 == p.p130);
        self.scalar_v2263 = v2263;
        let v2266: bool = (2.0 == p.p130);
        self.scalar_v2266 = v2266;
        let v2267: bool = (!v2263);
        self.scalar_v2267 = v2267;
        let v2268: bool = (v2266 && v2267);
        self.scalar_v2268 = v2268;
        let v2269: f64 = p.p131;
        self.scalar_v2269 = v2269;
        let v2272: bool = (!v2266);
        self.scalar_v2272 = v2272;
        let v2273: bool = (v2267 && v2272);
        self.scalar_v2273 = v2273;
        let v2288: f64 = p.p68;
        self.scalar_v2288 = v2288;
        let v2289: f64 = p.p77;
        self.scalar_v2289 = v2289;
        let v2329: f64 = (p.p3 * p.p68);
        self.scalar_v2329 = v2329;
        let v2333: f64 = (p.p3 * p.p77);
        self.scalar_v2333 = v2333;
        let v2363: f64 = (-p.p3);
        self.scalar_v2363 = v2363;
        let v2364: f64 = (p.p3 + v2363);
        self.scalar_v2364 = v2364;
        let v2365: f64 = (v2363 - v2363);
        self.scalar_v2365 = v2365;
        let v2366: f64 = (p.p3 + v2364);
        self.scalar_v2366 = v2366;
        let v3051: f64 = (v1058 - 1.0);
        self.scalar_v3051 = v3051;
        let v3066: f64 = (if v1067 { p.p3 } else { 0.0 });
        self.scalar_v3066 = v3066;
        let v3067: f64 = (if v1067 { v2363 } else { 0.0 });
        self.scalar_v3067 = v3067;
        let v3134: f64 = (p.p75 - 1.0);
        self.scalar_v3134 = v3134;
        let v3146: f64 = (v1103 - 1.0);
        self.scalar_v3146 = v3146;
        let v3367: f64 = (v2363 / 0.0001);
        self.scalar_v3367 = v3367;
        let v3368: f64 = (p.p3 / 0.0001);
        self.scalar_v3368 = v3368;
        let v3377: f64 = (-v3367);
        self.scalar_v3377 = v3377;
        let v3378: f64 = (-v3368);
        self.scalar_v3378 = v3378;
        let v3401: f64 = (v2363 / 0.001);
        self.scalar_v3401 = v3401;
        let v3402: f64 = (p.p3 / 0.001);
        self.scalar_v3402 = v3402;
        let v3413: f64 = (-v3401);
        self.scalar_v3413 = v3413;
        let v3414: f64 = (-v3402);
        self.scalar_v3414 = v3414;
        let v3739: f64 = (v1414 - 1.0);
        self.scalar_v3739 = v3739;
        let v3778: f64 = (v34 * v2363);
        self.scalar_v3778 = v3778;
        let v3779: f64 = (p.p3 * v34);
        self.scalar_v3779 = v3779;
        let v3822: f64 = (0.5 * v2363);
        self.scalar_v3822 = v3822;
        let v3823: f64 = (p.p3 * 0.5);
        self.scalar_v3823 = v3823;
        let v3910: f64 = (v1507 - 1.0);
        self.scalar_v3910 = v3910;
        let v3949: f64 = (p.p3 * v69);
        self.scalar_v3949 = v3949;
        let v3950: f64 = (v69 * v2363);
        self.scalar_v3950 = v3950;
        let v4163: f64 = (if v1606 { v2364 } else { 0.0 });
        self.scalar_v4163 = v4163;
        let v4164: f64 = (if v1606 { v2366 } else { 0.0 });
        self.scalar_v4164 = v4164;
        let v4165: f64 = (if v1606 { v2365 } else { 0.0 });
        self.scalar_v4165 = v4165;
        let v4166: f64 = (if v1606 { v2363 } else { 0.0 });
        self.scalar_v4166 = v4166;
        let v4342: f64 = (if v1645 { p.p3 } else { 0.0 });
        self.scalar_v4342 = v4342;
        let v4343: f64 = (if v1645 { v2364 } else { 0.0 });
        self.scalar_v4343 = v4343;
        let v4344: f64 = (if v1645 { v2363 } else { 0.0 });
        self.scalar_v4344 = v4344;
        let v4345: f64 = (-v4342);
        self.scalar_v4345 = v4345;
        let v4346: f64 = (-v4343);
        self.scalar_v4346 = v4346;
        let v4347: f64 = (-v4344);
        self.scalar_v4347 = v4347;
        let v4711: f64 = (p.p40 - 1.0);
        self.scalar_v4711 = v4711;
        let v5173: f64 = (p.p48 - 1.0);
        self.scalar_v5173 = v5173;
        let v5252: f64 = (p.p49 - 1.0);
        self.scalar_v5252 = v5252;
        let v5806: f64 = (p.p3 / p.p90);
        self.scalar_v5806 = v5806;
        let v5807: f64 = (v2364 / p.p90);
        self.scalar_v5807 = v5807;
        let v5808: f64 = (v2365 / p.p90);
        self.scalar_v5808 = v5808;
        let v5809: f64 = (v2363 / p.p90);
        self.scalar_v5809 = v5809;
        let v6041: f64 = (v2189 - 1.0);
        self.scalar_v6041 = v6041;
        let v6131: f64 = (p.p3 * 0.2);
        self.scalar_v6131 = v6131;
        let v6132: f64 = (0.2 * v2363);
        self.scalar_v6132 = v6132;
        let v6332: f64 = (0.0 * v2363);
        self.scalar_v6332 = v6332;
        let v6333: f64 = (p.p3 * 0.0);
        self.scalar_v6333 = v6333;
        let v6334: f64 = (0.0 * v2364);
        self.scalar_v6334 = v6334;
        let v6335: f64 = (0.0 * v2365);
        self.scalar_v6335 = v6335;
        let v6479: f64 = (p.p3 * p.p3);
        self.scalar_v6479 = v6479;
        let v6480: f64 = (p.p3 * v2363);
        self.scalar_v6480 = v6480;
        let v6550: f64 = (p.p3 * v2329);
        self.scalar_v6550 = v6550;
        let v6551: f64 = (v2329 * v2363);
        self.scalar_v6551 = v6551;
        let v6556: f64 = (v2333 * v2363);
        self.scalar_v6556 = v6556;
        let v6557: f64 = (p.p3 * v2333);
        self.scalar_v6557 = v6557;
        let v6580: f64 = (p.p3 * v2364);
        self.scalar_v6580 = v6580;
        let v6581: f64 = (p.p3 * v2365);
        self.scalar_v6581 = v6581;
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
        let v101: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v101 = v101;
        let v103: f64 = (self.scalar_v20 * 8.617086918058125e-5);
        self.scalar_v103 = v103;
        let v105: f64 = (1.0 / self.scalar_v103);
        self.scalar_v105 = v105;
        let v107: f64 = (self.scalar_v105 - self.scalar_v106);
        self.scalar_v107 = v107;
        let v108: f64 = (self.scalar_v20 - self.scalar_v17);
        self.scalar_v108 = v108;
        let v109: f64 = ((self.scalar_v101) as f64).ln();
        self.scalar_v109 = v109;
        let v110: f64 = (self.scalar_v20 * self.scalar_v37);
        self.scalar_v110 = v110;
        let v111: f64 = (self.scalar_v20 * self.scalar_v110);
        self.scalar_v111 = v111;
        let v112: f64 = (self.scalar_v20 + self.scalar_v40);
        self.scalar_v112 = v112;
        let v113: f64 = (self.scalar_v111 / self.scalar_v112);
        self.scalar_v113 = v113;
        let v114: f64 = (self.scalar_v62 - self.scalar_v113);
        self.scalar_v114 = v114;
        let v115: f64 = (self.scalar_v114 - 0.05);
        self.scalar_v115 = v115;
        let v116: f64 = (self.scalar_v115 / 0.1);
        self.scalar_v116 = v116;
        let v117: bool = (self.scalar_v114 < 0.05);
        self.scalar_v117 = v117;
        let v118: f64 = ((self.scalar_v116) as f64).exp();
        self.scalar_v118 = v118;
        let v119: f64 = (1.0 + self.scalar_v118);
        self.scalar_v119 = v119;
        let v120: f64 = ((self.scalar_v119) as f64).ln();
        self.scalar_v120 = v120;
        let v121: f64 = (0.1 * self.scalar_v120);
        self.scalar_v121 = v121;
        let v122: f64 = (0.05 + self.scalar_v121);
        self.scalar_v122 = v122;
        let v123: f64 = (if self.scalar_v117 { self.scalar_v122 } else { 0.0 });
        self.scalar_v123 = v123;
        let v124: bool = (!self.scalar_v117);
        self.scalar_v124 = v124;
        let v125: f64 = (-self.scalar_v116);
        self.scalar_v125 = v125;
        let v126: f64 = ((self.scalar_v125) as f64).exp();
        self.scalar_v126 = v126;
        let v127: f64 = (1.0 + self.scalar_v126);
        self.scalar_v127 = v127;
        let v128: f64 = ((self.scalar_v127) as f64).ln();
        self.scalar_v128 = v128;
        let v129: f64 = (0.1 * self.scalar_v128);
        self.scalar_v129 = v129;
        let v130: f64 = (self.scalar_v114 + self.scalar_v129);
        self.scalar_v130 = v130;
        let v131: f64 = (if self.scalar_v124 { self.scalar_v130 } else { self.scalar_v123 });
        self.scalar_v131 = v131;
        let v132: f64 = (self.scalar_v20 * self.scalar_v72);
        self.scalar_v132 = v132;
        let v133: f64 = (self.scalar_v20 * self.scalar_v132);
        self.scalar_v133 = v133;
        let v134: f64 = (self.scalar_v20 + self.scalar_v75);
        self.scalar_v134 = v134;
        let v135: f64 = (self.scalar_v133 / self.scalar_v134);
        self.scalar_v135 = v135;
        let v136: f64 = (self.scalar_v95 - self.scalar_v135);
        self.scalar_v136 = v136;
        let v137: f64 = (self.scalar_v136 - 0.05);
        self.scalar_v137 = v137;
        let v138: f64 = (self.scalar_v137 / 0.1);
        self.scalar_v138 = v138;
        let v139: bool = (self.scalar_v136 < 0.05);
        self.scalar_v139 = v139;
        let v140: f64 = ((self.scalar_v138) as f64).exp();
        self.scalar_v140 = v140;
        let v141: f64 = (1.0 + self.scalar_v140);
        self.scalar_v141 = v141;
        let v142: f64 = ((self.scalar_v141) as f64).ln();
        self.scalar_v142 = v142;
        let v143: f64 = (0.1 * self.scalar_v142);
        self.scalar_v143 = v143;
        let v144: f64 = (0.05 + self.scalar_v143);
        self.scalar_v144 = v144;
        let v145: f64 = (if self.scalar_v139 { self.scalar_v144 } else { 0.0 });
        self.scalar_v145 = v145;
        let v146: bool = (!self.scalar_v139);
        self.scalar_v146 = v146;
        let v147: f64 = (-self.scalar_v138);
        self.scalar_v147 = v147;
        let v148: f64 = ((self.scalar_v147) as f64).exp();
        self.scalar_v148 = v148;
        let v149: f64 = (1.0 + self.scalar_v148);
        self.scalar_v149 = v149;
        let v150: f64 = ((self.scalar_v149) as f64).ln();
        self.scalar_v150 = v150;
        let v151: f64 = (0.1 * self.scalar_v150);
        self.scalar_v151 = v151;
        let v152: f64 = (self.scalar_v136 + self.scalar_v151);
        self.scalar_v152 = v152;
        let v153: f64 = (if self.scalar_v146 { self.scalar_v152 } else { self.scalar_v145 });
        self.scalar_v153 = v153;
        let v156: f64 = (self.scalar_v103 * -3.0);
        self.scalar_v156 = v156;
        let v157: f64 = (self.scalar_v109 * self.scalar_v156);
        self.scalar_v157 = v157;
        let v158: f64 = (self.scalar_v64 * self.scalar_v101);
        self.scalar_v158 = v158;
        let v159: f64 = (self.scalar_v157 + self.scalar_v158);
        self.scalar_v159 = v159;
        let v160: f64 = (1.0 - self.scalar_v101);
        self.scalar_v160 = v160;
        let v162: f64 = (self.scalar_v160 * self.scalar_v161);
        self.scalar_v162 = v162;
        let v163: f64 = (self.scalar_v159 + self.scalar_v162);
        self.scalar_v163 = v163;
        let v164: f64 = (0.05 - self.scalar_v163);
        self.scalar_v164 = v164;
        let v165: f64 = (self.scalar_v164 / self.scalar_v103);
        self.scalar_v165 = v165;
        let v166: bool = (0.05 < self.scalar_v163);
        self.scalar_v166 = v166;
        let v167: f64 = ((self.scalar_v165) as f64).exp();
        self.scalar_v167 = v167;
        let v168: f64 = (1.0 + self.scalar_v167);
        self.scalar_v168 = v168;
        let v169: f64 = ((self.scalar_v168) as f64).ln();
        self.scalar_v169 = v169;
        let v170: f64 = (self.scalar_v103 * self.scalar_v169);
        self.scalar_v170 = v170;
        let v171: f64 = (self.scalar_v163 + self.scalar_v170);
        self.scalar_v171 = v171;
        let v172: f64 = (if self.scalar_v166 { self.scalar_v171 } else { 0.0 });
        self.scalar_v172 = v172;
        let v173: bool = (!self.scalar_v166);
        self.scalar_v173 = v173;
        let v174: f64 = (-self.scalar_v165);
        self.scalar_v174 = v174;
        let v175: f64 = ((self.scalar_v174) as f64).exp();
        self.scalar_v175 = v175;
        let v176: f64 = (1.0 + self.scalar_v175);
        self.scalar_v176 = v176;
        let v177: f64 = ((self.scalar_v176) as f64).ln();
        self.scalar_v177 = v177;
        let v178: f64 = (self.scalar_v103 * self.scalar_v177);
        self.scalar_v178 = v178;
        let v179: f64 = (0.05 + self.scalar_v178);
        self.scalar_v179 = v179;
        let v180: f64 = (if self.scalar_v173 { self.scalar_v179 } else { self.scalar_v172 });
        self.scalar_v180 = v180;
        let v182: f64 = (self.scalar_v101 * self.scalar_v181);
        self.scalar_v182 = v182;
        let v183: f64 = (self.scalar_v157 + self.scalar_v182);
        self.scalar_v183 = v183;
        let v185: f64 = (self.scalar_v160 * self.scalar_v184);
        self.scalar_v185 = v185;
        let v186: f64 = (self.scalar_v183 + self.scalar_v185);
        self.scalar_v186 = v186;
        let v187: f64 = (0.05 - self.scalar_v186);
        self.scalar_v187 = v187;
        let v188: f64 = (self.scalar_v187 / self.scalar_v103);
        self.scalar_v188 = v188;
        let v189: bool = (0.05 < self.scalar_v186);
        self.scalar_v189 = v189;
        let v190: f64 = ((self.scalar_v188) as f64).exp();
        self.scalar_v190 = v190;
        let v191: f64 = (1.0 + self.scalar_v190);
        self.scalar_v191 = v191;
        let v192: f64 = ((self.scalar_v191) as f64).ln();
        self.scalar_v192 = v192;
        let v193: f64 = (self.scalar_v103 * self.scalar_v192);
        self.scalar_v193 = v193;
        let v194: f64 = (self.scalar_v186 + self.scalar_v193);
        self.scalar_v194 = v194;
        let v195: f64 = (if self.scalar_v189 { self.scalar_v194 } else { 0.0 });
        self.scalar_v195 = v195;
        let v196: bool = (!self.scalar_v189);
        self.scalar_v196 = v196;
        let v197: f64 = (-self.scalar_v188);
        self.scalar_v197 = v197;
        let v198: f64 = ((self.scalar_v197) as f64).exp();
        self.scalar_v198 = v198;
        let v199: f64 = (1.0 + self.scalar_v198);
        self.scalar_v199 = v199;
        let v200: f64 = ((self.scalar_v199) as f64).ln();
        self.scalar_v200 = v200;
        let v201: f64 = (self.scalar_v103 * self.scalar_v200);
        self.scalar_v201 = v201;
        let v202: f64 = (0.05 + self.scalar_v201);
        self.scalar_v202 = v202;
        let v203: f64 = (if self.scalar_v196 { self.scalar_v202 } else { self.scalar_v195 });
        self.scalar_v203 = v203;
        let v205: f64 = (self.scalar_v101 * self.scalar_v204);
        self.scalar_v205 = v205;
        let v206: f64 = (self.scalar_v157 + self.scalar_v205);
        self.scalar_v206 = v206;
        let v207: f64 = (self.scalar_v185 + self.scalar_v206);
        self.scalar_v207 = v207;
        let v208: f64 = (0.05 - self.scalar_v207);
        self.scalar_v208 = v208;
        let v209: f64 = (self.scalar_v208 / self.scalar_v103);
        self.scalar_v209 = v209;
        let v210: bool = (0.05 < self.scalar_v207);
        self.scalar_v210 = v210;
        let v211: f64 = ((self.scalar_v209) as f64).exp();
        self.scalar_v211 = v211;
        let v212: f64 = (1.0 + self.scalar_v211);
        self.scalar_v212 = v212;
        let v213: f64 = ((self.scalar_v212) as f64).ln();
        self.scalar_v213 = v213;
        let v214: f64 = (self.scalar_v103 * self.scalar_v213);
        self.scalar_v214 = v214;
        let v215: f64 = (self.scalar_v207 + self.scalar_v214);
        self.scalar_v215 = v215;
        let v216: f64 = (if self.scalar_v210 { self.scalar_v215 } else { 0.0 });
        self.scalar_v216 = v216;
        let v217: bool = (!self.scalar_v210);
        self.scalar_v217 = v217;
        let v218: f64 = (-self.scalar_v209);
        self.scalar_v218 = v218;
        let v219: f64 = ((self.scalar_v218) as f64).exp();
        self.scalar_v219 = v219;
        let v220: f64 = (1.0 + self.scalar_v219);
        self.scalar_v220 = v220;
        let v221: f64 = ((self.scalar_v220) as f64).ln();
        self.scalar_v221 = v221;
        let v222: f64 = (self.scalar_v103 * self.scalar_v221);
        self.scalar_v222 = v222;
        let v223: f64 = (0.05 + self.scalar_v222);
        self.scalar_v223 = v223;
        let v224: f64 = (if self.scalar_v217 { self.scalar_v223 } else { self.scalar_v216 });
        self.scalar_v224 = v224;
        let v225: f64 = (self.scalar_v66 * self.scalar_v101);
        self.scalar_v225 = v225;
        let v226: f64 = (self.scalar_v157 + self.scalar_v225);
        self.scalar_v226 = v226;
        let v227: f64 = (self.scalar_v185 + self.scalar_v226);
        self.scalar_v227 = v227;
        let v228: f64 = (0.05 - self.scalar_v227);
        self.scalar_v228 = v228;
        let v229: f64 = (self.scalar_v228 / self.scalar_v103);
        self.scalar_v229 = v229;
        let v230: bool = (0.05 < self.scalar_v227);
        self.scalar_v230 = v230;
        let v231: f64 = ((self.scalar_v229) as f64).exp();
        self.scalar_v231 = v231;
        let v232: f64 = (1.0 + self.scalar_v231);
        self.scalar_v232 = v232;
        let v233: f64 = ((self.scalar_v232) as f64).ln();
        self.scalar_v233 = v233;
        let v234: f64 = (self.scalar_v103 * self.scalar_v233);
        self.scalar_v234 = v234;
        let v235: f64 = (self.scalar_v227 + self.scalar_v234);
        self.scalar_v235 = v235;
        let v236: f64 = (if self.scalar_v230 { self.scalar_v235 } else { 0.0 });
        self.scalar_v236 = v236;
        let v237: bool = (!self.scalar_v230);
        self.scalar_v237 = v237;
        let v238: f64 = (-self.scalar_v229);
        self.scalar_v238 = v238;
        let v239: f64 = ((self.scalar_v238) as f64).exp();
        self.scalar_v239 = v239;
        let v240: f64 = (1.0 + self.scalar_v239);
        self.scalar_v240 = v240;
        let v241: f64 = ((self.scalar_v240) as f64).ln();
        self.scalar_v241 = v241;
        let v242: f64 = (self.scalar_v103 * self.scalar_v241);
        self.scalar_v242 = v242;
        let v243: f64 = (0.05 + self.scalar_v242);
        self.scalar_v243 = v243;
        let v244: f64 = (if self.scalar_v237 { self.scalar_v243 } else { self.scalar_v236 });
        self.scalar_v244 = v244;
        let v246: f64 = (self.scalar_v101 * self.scalar_v245);
        self.scalar_v246 = v246;
        let v247: f64 = (self.scalar_v157 + self.scalar_v246);
        self.scalar_v247 = v247;
        let v249: f64 = (self.scalar_v160 * self.scalar_v248);
        self.scalar_v249 = v249;
        let v250: f64 = (self.scalar_v247 + self.scalar_v249);
        self.scalar_v250 = v250;
        let v251: f64 = (0.05 - self.scalar_v250);
        self.scalar_v251 = v251;
        let v252: f64 = (self.scalar_v251 / self.scalar_v103);
        self.scalar_v252 = v252;
        let v253: bool = (0.05 < self.scalar_v250);
        self.scalar_v253 = v253;
        let v254: f64 = ((self.scalar_v252) as f64).exp();
        self.scalar_v254 = v254;
        let v255: f64 = (1.0 + self.scalar_v254);
        self.scalar_v255 = v255;
        let v256: f64 = ((self.scalar_v255) as f64).ln();
        self.scalar_v256 = v256;
        let v257: f64 = (self.scalar_v103 * self.scalar_v256);
        self.scalar_v257 = v257;
        let v258: f64 = (self.scalar_v250 + self.scalar_v257);
        self.scalar_v258 = v258;
        let v259: f64 = (if self.scalar_v253 { self.scalar_v258 } else { 0.0 });
        self.scalar_v259 = v259;
        let v260: bool = (!self.scalar_v253);
        self.scalar_v260 = v260;
        let v261: f64 = (-self.scalar_v252);
        self.scalar_v261 = v261;
        let v262: f64 = ((self.scalar_v261) as f64).exp();
        self.scalar_v262 = v262;
        let v263: f64 = (1.0 + self.scalar_v262);
        self.scalar_v263 = v263;
        let v264: f64 = ((self.scalar_v263) as f64).ln();
        self.scalar_v264 = v264;
        let v265: f64 = (self.scalar_v103 * self.scalar_v264);
        self.scalar_v265 = v265;
        let v266: f64 = (0.05 + self.scalar_v265);
        self.scalar_v266 = v266;
        let v267: f64 = (if self.scalar_v260 { self.scalar_v266 } else { self.scalar_v259 });
        self.scalar_v267 = v267;
        let v268: f64 = (1.0 / self.scalar_v180);
        self.scalar_v268 = v268;
        let v269: f64 = (1.0 / self.scalar_v244);
        self.scalar_v269 = v269;
        let v270: f64 = (self.scalar_v64 * self.scalar_v268);
        self.scalar_v270 = v270;
        let v271: f64 = f64::powf(self.scalar_v270, self.scalar_v32);
        self.scalar_v271 = v271;
        let v272: f64 = (self.scalar_v66 * self.scalar_v269);
        self.scalar_v272 = v272;
        let v273: f64 = f64::powf(self.scalar_v272, self.scalar_v67);
        self.scalar_v273 = v273;
        let v275: f64 = (self.scalar_v271 * self.scalar_v274);
        self.scalar_v275 = v275;
        let v278: f64 = (self.scalar_v66 / self.scalar_v244);
        self.scalar_v278 = v278;
        let v279: f64 = f64::powf(self.scalar_v278, self.scalar_v67);
        self.scalar_v279 = v279;
        let v280: f64 = (self.scalar_v277 * self.scalar_v279);
        self.scalar_v280 = v280;
        let v281: f64 = (self.scalar_v276 + self.scalar_v280);
        self.scalar_v281 = v281;
        let v282: f64 = (1.0 / self.scalar_v281);
        self.scalar_v282 = v282;
        let v284: f64 = (self.scalar_v281 * self.scalar_v283);
        self.scalar_v284 = v284;
        let v285: f64 = (self.scalar_v276 * self.scalar_v282);
        self.scalar_v285 = v285;
        let v288: f64 = (self.scalar_v109 * self.scalar_v287);
        self.scalar_v288 = v288;
        let v289: f64 = ((self.scalar_v288) as f64).exp();
        self.scalar_v289 = v289;
        let v290: f64 = (self.scalar_v286 * self.scalar_v289);
        self.scalar_v290 = v290;
        let v291: bool = (self.scalar_v290 < self.scalar_v28);
        self.scalar_v291 = v291;
        let v292: f64 = (if self.scalar_v291 { self.scalar_v28 } else { self.scalar_v290 });
        self.scalar_v292 = v292;
        let v297: f64 = (self.scalar_v109 * self.scalar_v296);
        self.scalar_v297 = v297;
        let v298: f64 = ((self.scalar_v297) as f64).exp();
        self.scalar_v298 = v298;
        let v299: f64 = (self.scalar_v293 * self.scalar_v298);
        self.scalar_v299 = v299;
        let v302: f64 = (self.scalar_v109 * self.scalar_v301);
        self.scalar_v302 = v302;
        let v303: f64 = ((self.scalar_v302) as f64).exp();
        self.scalar_v303 = v303;
        let v304: f64 = (self.scalar_v300 * self.scalar_v303);
        self.scalar_v304 = v304;
        let v305: bool = (self.scalar_v304 < self.scalar_v28);
        self.scalar_v305 = v305;
        let v306: f64 = (if self.scalar_v305 { self.scalar_v28 } else { self.scalar_v304 });
        self.scalar_v306 = v306;
        let v309: f64 = (self.scalar_v109 * self.scalar_v308);
        self.scalar_v309 = v309;
        let v310: f64 = ((self.scalar_v309) as f64).exp();
        self.scalar_v310 = v310;
        let v311: f64 = (self.scalar_v307 * self.scalar_v310);
        self.scalar_v311 = v311;
        let v314: f64 = (self.scalar_v109 * self.scalar_v313);
        self.scalar_v314 = v314;
        let v315: f64 = ((self.scalar_v314) as f64).exp();
        self.scalar_v315 = v315;
        let v316: f64 = (self.scalar_v312 * self.scalar_v315);
        self.scalar_v316 = v316;
        let v318: f64 = (self.scalar_v315 * self.scalar_v317);
        self.scalar_v318 = v318;
        let v321: f64 = (self.scalar_v109 * self.scalar_v320);
        self.scalar_v321 = v321;
        let v322: f64 = ((self.scalar_v321) as f64).exp();
        self.scalar_v322 = v322;
        let v323: f64 = (self.scalar_v319 * self.scalar_v322);
        self.scalar_v323 = v323;
        let v327: f64 = (self.scalar_v108 * self.scalar_v324);
        self.scalar_v327 = v327;
        let v328: f64 = (1.0 + self.scalar_v327);
        self.scalar_v328 = v328;
        let v329: f64 = (self.scalar_v326 * self.scalar_v328);
        self.scalar_v329 = v329;
        let v330: f64 = (if self.scalar_v325 { self.scalar_v329 } else { 0.0 });
        self.scalar_v330 = v330;
        let v331: f64 = (self.scalar_v330 - 1.0);
        self.scalar_v331 = v331;
        let v332: f64 = (self.scalar_v331 / 0.001);
        self.scalar_v332 = v332;
        let v333: f64 = (if self.scalar_v325 { self.scalar_v332 } else { self.scalar_v252 });
        self.scalar_v333 = v333;
        let v334: bool = (self.scalar_v330 < 1.0);
        self.scalar_v334 = v334;
        let v335: bool = (self.scalar_v325 && self.scalar_v334);
        self.scalar_v335 = v335;
        let v336: f64 = ((self.scalar_v333) as f64).exp();
        self.scalar_v336 = v336;
        let v337: f64 = (1.0 + self.scalar_v336);
        self.scalar_v337 = v337;
        let v338: f64 = ((self.scalar_v337) as f64).ln();
        self.scalar_v338 = v338;
        let v339: f64 = (0.001 * self.scalar_v338);
        self.scalar_v339 = v339;
        let v340: f64 = (1.0 + self.scalar_v339);
        self.scalar_v340 = v340;
        let v341: f64 = (if self.scalar_v335 { self.scalar_v340 } else { self.scalar_v330 });
        self.scalar_v341 = v341;
        let v342: bool = (!self.scalar_v334);
        self.scalar_v342 = v342;
        let v343: bool = (self.scalar_v325 && self.scalar_v342);
        self.scalar_v343 = v343;
        let v344: f64 = (-self.scalar_v333);
        self.scalar_v344 = v344;
        let v345: f64 = ((self.scalar_v344) as f64).exp();
        self.scalar_v345 = v345;
        let v346: f64 = (1.0 + self.scalar_v345);
        self.scalar_v346 = v346;
        let v347: f64 = ((self.scalar_v346) as f64).ln();
        self.scalar_v347 = v347;
        let v348: f64 = (0.001 * self.scalar_v347);
        self.scalar_v348 = v348;
        let v349: f64 = (self.scalar_v341 + self.scalar_v348);
        self.scalar_v349 = v349;
        let v350: f64 = (if self.scalar_v343 { self.scalar_v349 } else { self.scalar_v341 });
        self.scalar_v350 = v350;
        let v352: f64 = (self.scalar_v350 - 0.0006931471805599453);
        self.scalar_v352 = v352;
        let v353: f64 = (if self.scalar_v325 { self.scalar_v352 } else { 0.0 });
        self.scalar_v353 = v353;
        let v355: f64 = (if self.scalar_v354 { self.scalar_v326 } else { self.scalar_v353 });
        self.scalar_v355 = v355;
        let v359: f64 = (self.scalar_v108 * self.scalar_v356);
        self.scalar_v359 = v359;
        let v360: f64 = (1.0 + self.scalar_v359);
        self.scalar_v360 = v360;
        let v361: f64 = (self.scalar_v358 * self.scalar_v360);
        self.scalar_v361 = v361;
        let v362: f64 = (if self.scalar_v357 { self.scalar_v361 } else { 0.0 });
        self.scalar_v362 = v362;
        let v363: f64 = (self.scalar_v362 - 1.0);
        self.scalar_v363 = v363;
        let v364: f64 = (self.scalar_v363 / 0.001);
        self.scalar_v364 = v364;
        let v365: f64 = (if self.scalar_v357 { self.scalar_v364 } else { self.scalar_v333 });
        self.scalar_v365 = v365;
        let v366: bool = (self.scalar_v362 < 1.0);
        self.scalar_v366 = v366;
        let v367: bool = (self.scalar_v357 && self.scalar_v366);
        self.scalar_v367 = v367;
        let v368: f64 = ((self.scalar_v365) as f64).exp();
        self.scalar_v368 = v368;
        let v369: f64 = (1.0 + self.scalar_v368);
        self.scalar_v369 = v369;
        let v370: f64 = ((self.scalar_v369) as f64).ln();
        self.scalar_v370 = v370;
        let v371: f64 = (0.001 * self.scalar_v370);
        self.scalar_v371 = v371;
        let v372: f64 = (1.0 + self.scalar_v371);
        self.scalar_v372 = v372;
        let v373: f64 = (if self.scalar_v367 { self.scalar_v372 } else { self.scalar_v362 });
        self.scalar_v373 = v373;
        let v374: bool = (!self.scalar_v366);
        self.scalar_v374 = v374;
        let v375: bool = (self.scalar_v357 && self.scalar_v374);
        self.scalar_v375 = v375;
        let v376: f64 = (-self.scalar_v365);
        self.scalar_v376 = v376;
        let v377: f64 = ((self.scalar_v376) as f64).exp();
        self.scalar_v377 = v377;
        let v378: f64 = (1.0 + self.scalar_v377);
        self.scalar_v378 = v378;
        let v379: f64 = ((self.scalar_v378) as f64).ln();
        self.scalar_v379 = v379;
        let v380: f64 = (0.001 * self.scalar_v379);
        self.scalar_v380 = v380;
        let v381: f64 = (self.scalar_v373 + self.scalar_v380);
        self.scalar_v381 = v381;
        let v382: f64 = (if self.scalar_v375 { self.scalar_v381 } else { self.scalar_v373 });
        self.scalar_v382 = v382;
        let v383: f64 = (self.scalar_v382 - 0.0006931471805599453);
        self.scalar_v383 = v383;
        let v384: f64 = (if self.scalar_v357 { self.scalar_v383 } else { 0.0 });
        self.scalar_v384 = v384;
        let v386: f64 = (if self.scalar_v385 { self.scalar_v358 } else { self.scalar_v384 });
        self.scalar_v386 = v386;
        let v389: f64 = (self.scalar_v108 * self.scalar_v388);
        self.scalar_v389 = v389;
        let v390: f64 = (1.0 + self.scalar_v389);
        self.scalar_v390 = v390;
        let v391: f64 = (self.scalar_v387 * self.scalar_v390);
        self.scalar_v391 = v391;
        let v393: f64 = (self.scalar_v391 * self.scalar_v391);
        self.scalar_v393 = v393;
        let v394: bool = (self.scalar_v391 < 0.0);
        self.scalar_v394 = v394;
        let v397: f64 = (1e-6 + self.scalar_v393);
        self.scalar_v397 = v397;
        let v398: f64 = ((self.scalar_v397) as f64).sqrt();
        self.scalar_v398 = v398;
        let v399: f64 = (self.scalar_v398 - self.scalar_v391);
        self.scalar_v399 = v399;
        let v400: f64 = (5e-7 / self.scalar_v399);
        self.scalar_v400 = v400;
        let v401: f64 = (if self.scalar_v394 { self.scalar_v400 } else { 0.0 });
        self.scalar_v401 = v401;
        let v402: bool = (!self.scalar_v394);
        self.scalar_v402 = v402;
        let v403: f64 = (self.scalar_v391 + self.scalar_v398);
        self.scalar_v403 = v403;
        let v404: f64 = (0.5 * self.scalar_v403);
        self.scalar_v404 = v404;
        let v405: f64 = (if self.scalar_v402 { self.scalar_v404 } else { self.scalar_v401 });
        self.scalar_v405 = v405;
        let v412: f64 = (self.scalar_v109 * self.scalar_v411);
        self.scalar_v412 = v412;
        let v413: f64 = (self.scalar_v412 / self.scalar_v355);
        self.scalar_v413 = v413;
        let v414: f64 = ((self.scalar_v413) as f64).exp();
        self.scalar_v414 = v414;
        let v415: f64 = (self.scalar_v406 * self.scalar_v414);
        self.scalar_v415 = v415;
        let v417: f64 = (self.scalar_v107 * self.scalar_v416);
        self.scalar_v417 = v417;
        let v418: f64 = (self.scalar_v417 / self.scalar_v355);
        self.scalar_v418 = v418;
        let v419: f64 = ((self.scalar_v418) as f64).exp();
        self.scalar_v419 = v419;
        let v420: f64 = (self.scalar_v415 * self.scalar_v419);
        self.scalar_v420 = v420;
        let v423: f64 = (self.scalar_v109 * self.scalar_v422);
        self.scalar_v423 = v423;
        let v424: f64 = ((self.scalar_v423) as f64).exp();
        self.scalar_v424 = v424;
        let v425: f64 = (self.scalar_v421 * self.scalar_v424);
        self.scalar_v425 = v425;
        let v429: f64 = (self.scalar_v109 * self.scalar_v428);
        self.scalar_v429 = v429;
        let v430: f64 = ((self.scalar_v429) as f64).exp();
        self.scalar_v430 = v430;
        let v431: f64 = (self.scalar_v426 * self.scalar_v430);
        self.scalar_v431 = v431;
        let v437: f64 = (self.scalar_v109 * self.scalar_v436);
        self.scalar_v437 = v437;
        let v438: f64 = ((self.scalar_v437) as f64).exp();
        self.scalar_v438 = v438;
        let v439: f64 = (self.scalar_v432 * self.scalar_v438);
        self.scalar_v439 = v439;
        let v442: f64 = (self.scalar_v107 * self.scalar_v441);
        self.scalar_v442 = v442;
        let v443: f64 = (self.scalar_v442 / self.scalar_v434);
        self.scalar_v443 = v443;
        let v444: f64 = ((self.scalar_v443) as f64).exp();
        self.scalar_v444 = v444;
        let v445: f64 = (self.scalar_v439 * self.scalar_v444);
        self.scalar_v445 = v445;
        let v450: f64 = (self.scalar_v109 * self.scalar_v449);
        self.scalar_v450 = v450;
        let v451: f64 = ((self.scalar_v450) as f64).exp();
        self.scalar_v451 = v451;
        let v452: f64 = (self.scalar_v446 * self.scalar_v451);
        self.scalar_v452 = v452;
        let v454: f64 = (self.scalar_v107 * self.scalar_v453);
        self.scalar_v454 = v454;
        let v455: f64 = (self.scalar_v454 / self.scalar_v447);
        self.scalar_v455 = v455;
        let v456: f64 = ((self.scalar_v455) as f64).exp();
        self.scalar_v456 = v456;
        let v457: f64 = (self.scalar_v452 * self.scalar_v456);
        self.scalar_v457 = v457;
        let v461: f64 = (self.scalar_v109 * self.scalar_v460);
        self.scalar_v461 = v461;
        let v463: f64 = (self.scalar_v461 / self.scalar_v462);
        self.scalar_v463 = v463;
        let v464: f64 = ((self.scalar_v463) as f64).exp();
        self.scalar_v464 = v464;
        let v465: f64 = (self.scalar_v458 * self.scalar_v464);
        self.scalar_v465 = v465;
        let v468: f64 = (self.scalar_v107 * self.scalar_v467);
        self.scalar_v468 = v468;
        let v469: f64 = (self.scalar_v468 / self.scalar_v462);
        self.scalar_v469 = v469;
        let v470: f64 = ((self.scalar_v469) as f64).exp();
        self.scalar_v470 = v470;
        let v471: f64 = (self.scalar_v465 * self.scalar_v470);
        self.scalar_v471 = v471;
        let v474: f64 = (self.scalar_v461 / self.scalar_v473);
        self.scalar_v474 = v474;
        let v475: f64 = ((self.scalar_v474) as f64).exp();
        self.scalar_v475 = v475;
        let v476: f64 = (self.scalar_v472 * self.scalar_v475);
        self.scalar_v476 = v476;
        let v477: f64 = (self.scalar_v468 / self.scalar_v473);
        self.scalar_v477 = v477;
        let v478: f64 = ((self.scalar_v477) as f64).exp();
        self.scalar_v478 = v478;
        let v479: f64 = (self.scalar_v476 * self.scalar_v478);
        self.scalar_v479 = v479;
        let v485: f64 = (self.scalar_v107 * self.scalar_v484);
        self.scalar_v485 = v485;
        let v486: f64 = (self.scalar_v485 / self.scalar_v462);
        self.scalar_v486 = v486;
        let v487: f64 = ((self.scalar_v486) as f64).exp();
        self.scalar_v487 = v487;
        let v488: f64 = (self.scalar_v482 * self.scalar_v487);
        self.scalar_v488 = v488;
        let v489: f64 = (if self.scalar_v481 { self.scalar_v488 } else { 0.0 });
        self.scalar_v489 = v489;
        let v493: f64 = (self.scalar_v107 * self.scalar_v492);
        self.scalar_v493 = v493;
        let v494: f64 = ((self.scalar_v493) as f64).exp();
        self.scalar_v494 = v494;
        let v495: f64 = (self.scalar_v490 * self.scalar_v494);
        self.scalar_v495 = v495;
        let v496: f64 = (if self.scalar_v481 { self.scalar_v495 } else { 0.0 });
        self.scalar_v496 = v496;
        let v500: f64 = (self.scalar_v107 * self.scalar_v499);
        self.scalar_v500 = v500;
        let v501: f64 = (self.scalar_v500 / self.scalar_v473);
        self.scalar_v501 = v501;
        let v502: f64 = ((self.scalar_v501) as f64).exp();
        self.scalar_v502 = v502;
        let v503: f64 = (self.scalar_v497 * self.scalar_v502);
        self.scalar_v503 = v503;
        let v504: f64 = (if self.scalar_v481 { self.scalar_v503 } else { 0.0 });
        self.scalar_v504 = v504;
        let v508: f64 = (self.scalar_v109 * self.scalar_v507);
        self.scalar_v508 = v508;
        let v509: f64 = ((self.scalar_v508) as f64).exp();
        self.scalar_v509 = v509;
        let v510: f64 = (self.scalar_v505 * self.scalar_v509);
        self.scalar_v510 = v510;
        let v513: f64 = (self.scalar_v107 * self.scalar_v512);
        self.scalar_v513 = v513;
        let v514: f64 = ((self.scalar_v513) as f64).exp();
        self.scalar_v514 = v514;
        let v515: f64 = (self.scalar_v510 * self.scalar_v514);
        self.scalar_v515 = v515;
        let v520: f64 = (self.scalar_v109 * self.scalar_v519);
        self.scalar_v520 = v520;
        let v521: f64 = ((self.scalar_v520) as f64).exp();
        self.scalar_v521 = v521;
        let v522: f64 = (self.scalar_v516 * self.scalar_v521);
        self.scalar_v522 = v522;
        let v523: f64 = (self.scalar_v442 / self.scalar_v517);
        self.scalar_v523 = v523;
        let v524: f64 = ((self.scalar_v523) as f64).exp();
        self.scalar_v524 = v524;
        let v525: f64 = (self.scalar_v522 * self.scalar_v524);
        self.scalar_v525 = v525;
        let v529: f64 = (self.scalar_v109 * self.scalar_v528);
        self.scalar_v529 = v529;
        let v530: f64 = ((self.scalar_v529) as f64).exp();
        self.scalar_v530 = v530;
        let v531: f64 = (self.scalar_v526 * self.scalar_v530);
        self.scalar_v531 = v531;
        let v532: f64 = (self.scalar_v442 / self.scalar_v527);
        self.scalar_v532 = v532;
        let v533: f64 = ((self.scalar_v532) as f64).exp();
        self.scalar_v533 = v533;
        let v534: f64 = (self.scalar_v531 * self.scalar_v533);
        self.scalar_v534 = v534;
        let v536: f64 = ((self.scalar_v101) as f64).sqrt();
        self.scalar_v536 = v536;
        let v537: f64 = (self.scalar_v535 * self.scalar_v536);
        self.scalar_v537 = v537;
        let v539: f64 = (self.scalar_v108 * self.scalar_v538);
        self.scalar_v539 = v539;
        let v540: f64 = ((self.scalar_v539) as f64).exp();
        self.scalar_v540 = v540;
        let v541: f64 = (self.scalar_v537 * self.scalar_v540);
        self.scalar_v541 = v541;
        let v542: f64 = (self.scalar_v63 * self.scalar_v131);
        self.scalar_v542 = v542;
        let v544: f64 = f64::powf(self.scalar_v542, -0.5);
        self.scalar_v544 = v544;
        let v545: f64 = (1.0 / self.scalar_v271);
        self.scalar_v545 = v545;
        let v547: f64 = (self.scalar_v131 * self.scalar_v546);
        self.scalar_v547 = v547;
        let v548: f64 = (self.scalar_v131 * self.scalar_v547);
        self.scalar_v548 = v548;
        let v549: f64 = (self.scalar_v544 * self.scalar_v548);
        self.scalar_v549 = v549;
        let v550: f64 = (self.scalar_v545 * self.scalar_v549);
        self.scalar_v550 = v550;
        let v551: f64 = (self.scalar_v64 * self.scalar_v550);
        self.scalar_v551 = v551;
        let v552: f64 = (self.scalar_v268 * self.scalar_v551);
        self.scalar_v552 = v552;
        let v553: f64 = (self.scalar_v63 * self.scalar_v552);
        self.scalar_v553 = v553;
        let v554: f64 = (self.scalar_v63 * self.scalar_v553);
        self.scalar_v554 = v554;
        let v556: f64 = (self.scalar_v544 * self.scalar_v555);
        self.scalar_v556 = v556;
        let v557: f64 = (self.scalar_v180 * self.scalar_v556);
        self.scalar_v557 = v557;
        let v558: f64 = (self.scalar_v180 * self.scalar_v557);
        self.scalar_v558 = v558;
        let v559: f64 = (self.scalar_v65 * self.scalar_v558);
        self.scalar_v559 = v559;
        let v560: f64 = (self.scalar_v65 * self.scalar_v559);
        self.scalar_v560 = v560;
        let v561: f64 = (self.scalar_v271 * self.scalar_v560);
        self.scalar_v561 = v561;
        let v562: f64 = (self.scalar_v546 - self.scalar_v554);
        self.scalar_v562 = v562;
        let v563: f64 = ((self.scalar_v562) as f64).exp();
        self.scalar_v563 = v563;
        let v564: f64 = (self.scalar_v561 * self.scalar_v563);
        self.scalar_v564 = v564;
        let v565: f64 = (self.scalar_v96 * self.scalar_v153);
        self.scalar_v565 = v565;
        let v566: f64 = f64::powf(self.scalar_v565, -0.5);
        self.scalar_v566 = v566;
        let v567: f64 = (1.0 / self.scalar_v273);
        self.scalar_v567 = v567;
        let v569: f64 = (self.scalar_v153 * self.scalar_v568);
        self.scalar_v569 = v569;
        let v570: f64 = (self.scalar_v153 * self.scalar_v569);
        self.scalar_v570 = v570;
        let v571: f64 = (self.scalar_v566 * self.scalar_v570);
        self.scalar_v571 = v571;
        let v572: f64 = (self.scalar_v567 * self.scalar_v571);
        self.scalar_v572 = v572;
        let v573: f64 = (self.scalar_v66 * self.scalar_v572);
        self.scalar_v573 = v573;
        let v574: f64 = (self.scalar_v269 * self.scalar_v573);
        self.scalar_v574 = v574;
        let v575: f64 = (self.scalar_v96 * self.scalar_v574);
        self.scalar_v575 = v575;
        let v576: f64 = (self.scalar_v96 * self.scalar_v575);
        self.scalar_v576 = v576;
        let v578: f64 = (self.scalar_v566 * self.scalar_v577);
        self.scalar_v578 = v578;
        let v579: f64 = (self.scalar_v244 * self.scalar_v578);
        self.scalar_v579 = v579;
        let v580: f64 = (self.scalar_v244 * self.scalar_v579);
        self.scalar_v580 = v580;
        let v581: f64 = (self.scalar_v97 * self.scalar_v580);
        self.scalar_v581 = v581;
        let v582: f64 = (self.scalar_v97 * self.scalar_v581);
        self.scalar_v582 = v582;
        let v583: f64 = (self.scalar_v273 * self.scalar_v582);
        self.scalar_v583 = v583;
        let v584: f64 = (self.scalar_v568 - self.scalar_v576);
        self.scalar_v584 = v584;
        let v585: f64 = ((self.scalar_v584) as f64).exp();
        self.scalar_v585 = v585;
        let v586: f64 = (self.scalar_v583 * self.scalar_v585);
        self.scalar_v586 = v586;
        let v587: f64 = (self.scalar_v109 * self.scalar_v295);
        self.scalar_v587 = v587;
        let v588: f64 = ((self.scalar_v587) as f64).exp();
        self.scalar_v588 = v588;
        let v590: f64 = (self.scalar_v588 * self.scalar_v589);
        self.scalar_v590 = v590;
        let v591: f64 = (self.scalar_v282 * self.scalar_v590);
        self.scalar_v591 = v591;
        let v593: f64 = (self.scalar_v588 * self.scalar_v592);
        self.scalar_v593 = v593;
        let v594: f64 = (self.scalar_v545 * self.scalar_v593);
        self.scalar_v594 = v594;
        let v597: f64 = (self.scalar_v109 * self.scalar_v596);
        self.scalar_v597 = v597;
        let v598: f64 = ((self.scalar_v597) as f64).exp();
        self.scalar_v598 = v598;
        let v599: f64 = (self.scalar_v595 * self.scalar_v598);
        self.scalar_v599 = v599;
        let v602: f64 = (self.scalar_v107 * self.scalar_v601);
        self.scalar_v602 = v602;
        let v603: f64 = ((self.scalar_v602) as f64).exp();
        self.scalar_v603 = v603;
        let v604: f64 = (self.scalar_v599 * self.scalar_v603);
        self.scalar_v604 = v604;
        let v608: f64 = (self.scalar_v109 * self.scalar_v607);
        self.scalar_v608 = v608;
        let v609: f64 = ((self.scalar_v608) as f64).exp();
        self.scalar_v609 = v609;
        let v610: f64 = (self.scalar_v605 * self.scalar_v609);
        self.scalar_v610 = v610;
        let v613: f64 = (self.scalar_v109 * self.scalar_v612);
        self.scalar_v613 = v613;
        let v614: f64 = ((self.scalar_v613) as f64).exp();
        self.scalar_v614 = v614;
        let v615: f64 = (self.scalar_v611 * self.scalar_v614);
        self.scalar_v615 = v615;
        let v617: f64 = (self.scalar_v610 + self.scalar_v615);
        self.scalar_v617 = v617;
        let v618: f64 = (self.scalar_v616 * self.scalar_v617);
        self.scalar_v618 = v618;
        let v620: f64 = (self.scalar_v618 / self.scalar_v619);
        self.scalar_v620 = v620;
        let v624: f64 = (self.scalar_v109 * self.scalar_v623);
        self.scalar_v624 = v624;
        let v625: f64 = ((self.scalar_v624) as f64).exp();
        self.scalar_v625 = v625;
        let v626: f64 = (self.scalar_v621 * self.scalar_v625);
        self.scalar_v626 = v626;
        let v628: f64 = (self.scalar_v20 - 300.0);
        self.scalar_v628 = v628;
        let v630: bool = (self.scalar_v20 < 525.0);
        self.scalar_v630 = v630;
        let v632: f64 = (self.scalar_v628 * 0.00072);
        self.scalar_v632 = v632;
        let v633: f64 = (1.0 + self.scalar_v632);
        self.scalar_v633 = v633;
        let v635: f64 = (self.scalar_v628 * 1.6e-6);
        self.scalar_v635 = v635;
        let v636: f64 = (self.scalar_v628 * self.scalar_v635);
        self.scalar_v636 = v636;
        let v637: f64 = (self.scalar_v633 - self.scalar_v636);
        self.scalar_v637 = v637;
        let v638: f64 = (self.scalar_v12 * self.scalar_v637);
        self.scalar_v638 = v638;
        let v639: f64 = (if self.scalar_v630 { self.scalar_v638 } else { 0.0 });
        self.scalar_v639 = v639;
        let v640: bool = (!self.scalar_v630);
        self.scalar_v640 = v640;
        let v643: f64 = (if self.scalar_v640 { self.scalar_v642 } else { self.scalar_v639 });
        self.scalar_v643 = v643;
        let v645: f64 = (self.scalar_v588 * self.scalar_v644);
        self.scalar_v645 = v645;
        let v647: f64 = (1.0 / self.scalar_v311);
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
        let v655: f64 = (1.0 / self.scalar_v316);
        self.scalar_v655 = v655;
        let v656: f64 = (if self.scalar_v654 { self.scalar_v655 } else { 0.0 });
        self.scalar_v656 = v656;
        let v657: bool = (self.scalar_v656 > self.scalar_v29);
        self.scalar_v657 = v657;
        let v658: bool = (self.scalar_v654 && self.scalar_v657);
        self.scalar_v658 = v658;
        let v659: f64 = (if self.scalar_v658 { self.scalar_v29 } else { self.scalar_v656 });
        self.scalar_v659 = v659;
        let v661: f64 = (if self.scalar_v660 { 0.0 } else { self.scalar_v659 });
        self.scalar_v661 = v661;
        let v663: f64 = (1.0 / self.scalar_v318);
        self.scalar_v663 = v663;
        let v664: f64 = (if self.scalar_v662 { self.scalar_v663 } else { 0.0 });
        self.scalar_v664 = v664;
        let v665: bool = (self.scalar_v664 > self.scalar_v29);
        self.scalar_v665 = v665;
        let v666: bool = (self.scalar_v662 && self.scalar_v665);
        self.scalar_v666 = v666;
        let v667: f64 = (if self.scalar_v666 { self.scalar_v29 } else { self.scalar_v664 });
        self.scalar_v667 = v667;
        let v669: f64 = (if self.scalar_v668 { 0.0 } else { self.scalar_v667 });
        self.scalar_v669 = v669;
        let v841: f64 = (2.0 * self.scalar_v103);
        self.scalar_v841 = v841;
        let v852: f64 = (self.scalar_v203 * 0.2);
        self.scalar_v852 = v852;
        let v876: f64 = (self.scalar_v323 * self.scalar_v871);
        self.scalar_v876 = v876;
        let v963: f64 = (self.scalar_v105 * self.scalar_v203);
        self.scalar_v963 = v963;
        let v964: f64 = ((self.scalar_v963) as f64).exp();
        self.scalar_v964 = v964;
        let v971: f64 = (self.scalar_v323 * self.scalar_v872);
        self.scalar_v971 = v971;
        let v972: f64 = (self.scalar_v871 * self.scalar_v971);
        self.scalar_v972 = v972;
        let v983: f64 = (0.1 * self.scalar_v244);
        self.scalar_v983 = v983;
        let v1006: f64 = (self.scalar_v103 * 1e-5);
        self.scalar_v1006 = v1006;
        let v1010: f64 = (self.scalar_v103 * 1e-40);
        self.scalar_v1010 = v1010;
        let v1037: f64 = (self.scalar_v180 * self.scalar_v1036);
        self.scalar_v1037 = v1037;
        let v1038: f64 = (0.1 * self.scalar_v180);
        self.scalar_v1038 = v1038;
        let v1060: f64 = (self.scalar_v180 / self.scalar_v1058);
        self.scalar_v1060 = v1060;
        let v1077: f64 = (2.0 - self.scalar_v285);
        self.scalar_v1077 = v1077;
        let v1078: f64 = (1.0 - self.scalar_v285);
        self.scalar_v1078 = v1078;
        let v1079: f64 = (self.scalar_v1077 / self.scalar_v1078);
        self.scalar_v1079 = v1079;
        let v1081: f64 = f64::powf(self.scalar_v1079, self.scalar_v1080);
        self.scalar_v1081 = v1081;
        let v1082: f64 = (1.0 - self.scalar_v1081);
        self.scalar_v1082 = v1082;
        let v1083: f64 = (self.scalar_v244 * self.scalar_v1082);
        self.scalar_v1083 = v1083;
        let v1104: f64 = (self.scalar_v244 / self.scalar_v1103);
        self.scalar_v1104 = v1104;
        let v1118: f64 = (4.0 * self.scalar_v420);
        self.scalar_v1118 = v1118;
        let v1119: f64 = (self.scalar_v1118 / self.scalar_v425);
        self.scalar_v1119 = v1119;
        let v1125: f64 = (1.0 / self.scalar_v386);
        self.scalar_v1125 = v1125;
        let v1150: f64 = (self.scalar_v105 * self.scalar_v645);
        self.scalar_v1150 = v1150;
        let v1151: f64 = ((self.scalar_v1150) as f64).exp();
        self.scalar_v1151 = v1151;
        let v1152: f64 = (self.scalar_v1151 - 1.0);
        self.scalar_v1152 = v1152;
        let v1173: f64 = (self.scalar_v420 * self.scalar_v1172);
        self.scalar_v1173 = v1173;
        let v1271: f64 = (2.0 * self.scalar_v489);
        self.scalar_v1271 = v1271;
        let v1330: f64 = (2.0 * self.scalar_v504);
        self.scalar_v1330 = v1330;
        let v1470: f64 = (2.0 * self.scalar_v564);
        self.scalar_v1470 = v1470;
        let v1559: f64 = (2.0 * self.scalar_v586);
        self.scalar_v1559 = v1559;
        let v1579: f64 = (2.0 * self.scalar_v515);
        self.scalar_v1579 = v1579;
        let v1582: f64 = (4.0 * self.scalar_v515);
        self.scalar_v1582 = v1582;
        let v1583: f64 = (self.scalar_v1582 / self.scalar_v431);
        self.scalar_v1583 = v1583;
        let v1596: f64 = (self.scalar_v515 * self.scalar_v1595);
        self.scalar_v1596 = v1596;
        let v1607: f64 = (self.scalar_v13 * self.scalar_v515);
        self.scalar_v1607 = v1607;
        let v1608: f64 = (self.scalar_v311 * self.scalar_v1607);
        self.scalar_v1608 = v1608;
        let v1609: f64 = (if self.scalar_v1606 { self.scalar_v1608 } else { 0.0 });
        self.scalar_v1609 = v1609;
        let v1610: f64 = (self.scalar_v105 * self.scalar_v1609);
        self.scalar_v1610 = v1610;
        let v1611: f64 = ((self.scalar_v1610) as f64).ln();
        self.scalar_v1611 = v1611;
        let v1612: f64 = (2.0 - self.scalar_v1611);
        self.scalar_v1612 = v1612;
        let v1613: f64 = (self.scalar_v103 * self.scalar_v1612);
        self.scalar_v1613 = v1613;
        let v1614: f64 = (if self.scalar_v1606 { self.scalar_v1613 } else { 0.0 });
        self.scalar_v1614 = v1614;
        let v1743: f64 = (-self.scalar_v405);
        self.scalar_v1743 = v1743;
        let v1759: f64 = (self.scalar_v1758 / self.scalar_v405);
        self.scalar_v1759 = v1759;
        let v1848: f64 = (self.scalar_v10 / self.scalar_v643);
        self.scalar_v1848 = v1848;
        let v1851: f64 = (-self.scalar_v643);
        self.scalar_v1851 = v1851;
        let v1975: f64 = (self.scalar_v275 * self.scalar_v1974);
        self.scalar_v1975 = v1975;
        let v1994: f64 = (self.scalar_v275 * self.scalar_v1973);
        self.scalar_v1994 = v1994;
        let v2005: f64 = (self.scalar_v284 * self.scalar_v2004);
        self.scalar_v2005 = v2005;
        let v2007: f64 = (self.scalar_v425 * self.scalar_v610);
        self.scalar_v2007 = v2007;
        let v2008: f64 = (0.5 * self.scalar_v2007);
        self.scalar_v2008 = v2008;
        let v2076: f64 = (self.scalar_v425 * self.scalar_v604);
        self.scalar_v2076 = v2076;
        let v2077: f64 = (self.scalar_v420 / self.scalar_v425);
        self.scalar_v2077 = v2077;
        let v2080: f64 = f64::powf(self.scalar_v2077, self.scalar_v2079);
        self.scalar_v2080 = v2080;
        let v2081: f64 = (self.scalar_v2076 * self.scalar_v2080);
        self.scalar_v2081 = v2081;
        let v2082: f64 = (self.scalar_v103 * self.scalar_v2078);
        self.scalar_v2082 = v2082;
        let v2094: f64 = (4.0 * self.scalar_v615);
        self.scalar_v2094 = v2094;
        let v2095: f64 = (self.scalar_v103 * self.scalar_v2094);
        self.scalar_v2095 = v2095;
        let v2096: f64 = (self.scalar_v2095 / self.scalar_v323);
        self.scalar_v2096 = v2096;
        let v2097: f64 = (0.5 * self.scalar_v2096);
        self.scalar_v2097 = v2097;
        let v2103: f64 = (0.5 * self.scalar_v620);
        self.scalar_v2103 = v2103;
        let v2126: f64 = (self.scalar_v626 * self.scalar_v1579);
        self.scalar_v2126 = v2126;
        let v2156: f64 = (self.scalar_v620 * self.scalar_v2155);
        self.scalar_v2156 = v2156;
        let v2177: f64 = (self.scalar_v626 * self.scalar_v1596);
        self.scalar_v2177 = v2177;
        let v2367: f64 = (self.scalar_v0 * self.scalar_v105);
        self.scalar_v2367 = v2367;
        let v2368: f64 = (self.scalar_v105 * self.scalar_v2363);
        self.scalar_v2368 = v2368;
        let v2377: f64 = (self.scalar_v2368 / self.scalar_v355);
        self.scalar_v2377 = v2377;
        let v2378: f64 = (self.scalar_v2367 / self.scalar_v355);
        self.scalar_v2378 = v2378;
        let v2387: f64 = (self.scalar_v105 * self.scalar_v2364);
        self.scalar_v2387 = v2387;
        let v2388: f64 = (self.scalar_v105 * self.scalar_v2365);
        self.scalar_v2388 = v2388;
        let v2413: f64 = (self.scalar_v105 * self.scalar_v2366);
        self.scalar_v2413 = v2413;
        let v3023: f64 = (self.scalar_v2363 / self.scalar_v1038);
        self.scalar_v3023 = v3023;
        let v3024: f64 = (self.scalar_v0 / self.scalar_v1038);
        self.scalar_v3024 = v3024;
        let v3035: f64 = (-self.scalar_v3023);
        self.scalar_v3035 = v3035;
        let v3036: f64 = (-self.scalar_v3024);
        self.scalar_v3036 = v3036;
        let v3188: f64 = (self.scalar_v0 * self.scalar_v285);
        self.scalar_v3188 = v3188;
        let v3189: f64 = (self.scalar_v285 * self.scalar_v2363);
        self.scalar_v3189 = v3189;
        let v3206: f64 = (self.scalar_v1125 - 1.0);
        self.scalar_v3206 = v3206;
        let v3439: f64 = (self.scalar_v2368 / self.scalar_v462);
        self.scalar_v3439 = v3439;
        let v3440: f64 = (self.scalar_v2367 / self.scalar_v462);
        self.scalar_v3440 = v3440;
        let v3572: f64 = (self.scalar_v2368 / self.scalar_v473);
        self.scalar_v3572 = v3572;
        let v3573: f64 = (self.scalar_v2367 / self.scalar_v473);
        self.scalar_v3573 = v3573;
        let v3629: f64 = (self.scalar_v2368 / self.scalar_v434);
        self.scalar_v3629 = v3629;
        let v3630: f64 = (self.scalar_v2367 / self.scalar_v434);
        self.scalar_v3630 = v3630;
        let v3644: f64 = (self.scalar_v2368 / self.scalar_v517);
        self.scalar_v3644 = v3644;
        let v3645: f64 = (self.scalar_v2367 / self.scalar_v517);
        self.scalar_v3645 = v3645;
        let v3659: f64 = (self.scalar_v2367 / self.scalar_v447);
        self.scalar_v3659 = v3659;
        let v3660: f64 = (self.scalar_v2387 / self.scalar_v447);
        self.scalar_v3660 = v3660;
        let v3661: f64 = (self.scalar_v2388 / self.scalar_v447);
        self.scalar_v3661 = v3661;
        let v3662: f64 = (self.scalar_v2368 / self.scalar_v447);
        self.scalar_v3662 = v3662;
        let v3686: f64 = (self.scalar_v2368 / self.scalar_v527);
        self.scalar_v3686 = v3686;
        let v3687: f64 = (self.scalar_v2367 / self.scalar_v527);
        self.scalar_v3687 = v3687;
        let v3728: f64 = (self.scalar_v268 * self.scalar_v2363);
        self.scalar_v3728 = v3728;
        let v3729: f64 = (self.scalar_v0 * self.scalar_v268);
        self.scalar_v3729 = v3729;
        let v3780: f64 = (self.scalar_v554 * self.scalar_v3778);
        self.scalar_v3780 = v3780;
        let v3781: f64 = (self.scalar_v554 * self.scalar_v3779);
        self.scalar_v3781 = v3781;
        let v3870: f64 = (self.scalar_v0 * self.scalar_v269);
        self.scalar_v3870 = v3870;
        let v3871: f64 = (self.scalar_v269 * self.scalar_v2363);
        self.scalar_v3871 = v3871;
        let v3872: f64 = (-self.scalar_v3870);
        self.scalar_v3872 = v3872;
        let v3873: f64 = (-self.scalar_v3871);
        self.scalar_v3873 = v3873;
        let v3951: f64 = (self.scalar_v576 * self.scalar_v3949);
        self.scalar_v3951 = v3951;
        let v3952: f64 = (self.scalar_v576 * self.scalar_v3950);
        self.scalar_v3952 = v3952;
        let v5295: f64 = (self.scalar_v1759 * self.scalar_v2363);
        self.scalar_v5295 = v5295;
        let v5296: f64 = (self.scalar_v0 * self.scalar_v1759);
        self.scalar_v5296 = v5296;
        let v5548: f64 = (self.scalar_v0 / self.scalar_v983);
        self.scalar_v5548 = v5548;
        let v5549: f64 = (self.scalar_v2364 / self.scalar_v983);
        self.scalar_v5549 = v5549;
        let v5550: f64 = (self.scalar_v2365 / self.scalar_v983);
        self.scalar_v5550 = v5550;
        let v5551: f64 = (self.scalar_v2363 / self.scalar_v983);
        self.scalar_v5551 = v5551;
        let v5572: f64 = (-self.scalar_v5548);
        self.scalar_v5572 = v5572;
        let v5573: f64 = (-self.scalar_v5549);
        self.scalar_v5573 = v5573;
        let v5574: f64 = (-self.scalar_v5550);
        self.scalar_v5574 = v5574;
        let v5575: f64 = (-self.scalar_v5551);
        self.scalar_v5575 = v5575;
        let v5634: f64 = (self.scalar_v285 * self.scalar_v2364);
        self.scalar_v5634 = v5634;
        let v5635: f64 = (self.scalar_v285 * self.scalar_v2365);
        self.scalar_v5635 = v5635;
        let v5652: f64 = (self.scalar_v2366 / self.scalar_v983);
        self.scalar_v5652 = v5652;
        let v5673: f64 = (-self.scalar_v5652);
        self.scalar_v5673 = v5673;
        let v5732: f64 = (self.scalar_v285 * self.scalar_v2366);
        self.scalar_v5732 = v5732;
        let v5749: f64 = (self.scalar_v2363 / self.scalar_v2082);
        self.scalar_v5749 = v5749;
        let v5750: f64 = (self.scalar_v0 / self.scalar_v2082);
        self.scalar_v5750 = v5750;
        let v5810: f64 = (self.scalar_v105 * self.scalar_v5806);
        self.scalar_v5810 = v5810;
        let v5811: f64 = (self.scalar_v105 * self.scalar_v5807);
        self.scalar_v5811 = v5811;
        let v5812: f64 = (self.scalar_v105 * self.scalar_v5808);
        self.scalar_v5812 = v5812;
        let v5813: f64 = (self.scalar_v105 * self.scalar_v5809);
        self.scalar_v5813 = v5813;
        let v6048: f64 = (if self.scalar_v2188 { self.scalar_v3023 } else { 0.0 });
        self.scalar_v6048 = v6048;
        let v6049: f64 = (if self.scalar_v2188 { self.scalar_v3024 } else { 0.0 });
        self.scalar_v6049 = v6049;
        let v6059: f64 = (-self.scalar_v6048);
        self.scalar_v6059 = v6059;
        let v6060: f64 = (-self.scalar_v6049);
        self.scalar_v6060 = v6060;
        let v6481: f64 = (self.scalar_v6479 / self.scalar_v292);
        self.scalar_v6481 = v6481;
        let v6482: f64 = (self.scalar_v6480 / self.scalar_v292);
        self.scalar_v6482 = v6482;
        let v6483: f64 = (self.scalar_v27 * self.scalar_v6481);
        self.scalar_v6483 = v6483;
        let v6484: f64 = (self.scalar_v27 * self.scalar_v6482);
        self.scalar_v6484 = v6484;
        let v6485: f64 = (self.scalar_v6479 / self.scalar_v306);
        self.scalar_v6485 = v6485;
        let v6486: f64 = (self.scalar_v6480 / self.scalar_v306);
        self.scalar_v6486 = v6486;
        let v6487: f64 = (self.scalar_v27 * self.scalar_v6485);
        self.scalar_v6487 = v6487;
        let v6488: f64 = (self.scalar_v27 * self.scalar_v6486);
        self.scalar_v6488 = v6488;
        let v6582: f64 = (self.scalar_v653 * self.scalar_v6479);
        self.scalar_v6582 = v6582;
        let v6583: f64 = (self.scalar_v653 * self.scalar_v6580);
        self.scalar_v6583 = v6583;
        let v6584: f64 = (self.scalar_v653 * self.scalar_v6581);
        self.scalar_v6584 = v6584;
        let v6585: f64 = (self.scalar_v653 * self.scalar_v6480);
        self.scalar_v6585 = v6585;
        let v6586: f64 = (self.scalar_v27 * self.scalar_v6582);
        self.scalar_v6586 = v6586;
        let v6587: f64 = (self.scalar_v27 * self.scalar_v6583);
        self.scalar_v6587 = v6587;
        let v6588: f64 = (self.scalar_v27 * self.scalar_v6584);
        self.scalar_v6588 = v6588;
        let v6589: f64 = (self.scalar_v27 * self.scalar_v6585);
        self.scalar_v6589 = v6589;
        let v6649: f64 = (self.scalar_v661 * self.scalar_v6479);
        self.scalar_v6649 = v6649;
        let v6650: f64 = (self.scalar_v661 * self.scalar_v6480);
        self.scalar_v6650 = v6650;
        let v6651: f64 = (self.scalar_v27 * self.scalar_v6649);
        self.scalar_v6651 = v6651;
        let v6652: f64 = (self.scalar_v27 * self.scalar_v6650);
        self.scalar_v6652 = v6652;
        let v6653: f64 = (if self.scalar_v654 { self.scalar_v6651 } else { 0.0 });
        self.scalar_v6653 = v6653;
        let v6654: f64 = (if self.scalar_v654 { self.scalar_v6652 } else { 0.0 });
        self.scalar_v6654 = v6654;
        let v6655: f64 = (self.scalar_v669 * self.scalar_v6480);
        self.scalar_v6655 = v6655;
        let v6656: f64 = (self.scalar_v669 * self.scalar_v6479);
        self.scalar_v6656 = v6656;
        let v6657: f64 = (self.scalar_v27 * self.scalar_v6655);
        self.scalar_v6657 = v6657;
        let v6658: f64 = (self.scalar_v27 * self.scalar_v6656);
        self.scalar_v6658 = v6658;
        let v6659: f64 = (if self.scalar_v662 { self.scalar_v6657 } else { 0.0 });
        self.scalar_v6659 = v6659;
        let v6660: f64 = (if self.scalar_v662 { self.scalar_v6658 } else { 0.0 });
        self.scalar_v6660 = v6660;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
