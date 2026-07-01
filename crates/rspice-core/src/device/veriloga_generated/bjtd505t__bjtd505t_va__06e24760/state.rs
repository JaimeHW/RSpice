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
    pub p143: f64,
    pub p144: f64,
    pub p145: f64,
    pub p146: f64,
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
            params.p133 = 300.0;
            params.p134 = 3.0000000000000004e-9;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 2.0;
            params.p138 = 400.0;
            params.p139 = 1e-40;
            params.p140 = 1e-40;
            params.p141 = 0.001;
            validate_parameter("minr", params.p141, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p142 = 0.0;
            params.p143 = 1.0;
            params.p144 = 0.0;
            params.p145 = 0.16;
            params.p146 = 0.0;
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
    pub(crate) param_given: Box<[bool; 147]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 10]>,
    pub(crate) ddt_state_previous: Box<[f64; 10]>,
    pub(crate) ddt_state_older: Box<[f64; 10]>,
    pub(crate) ddt_state_initialized: Box<[bool; 10]>,
    pub(crate) ddt_derivative_current: Box<[f64; 10]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 10]>,
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
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: bool,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v370: bool,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v373: bool,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v401: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: bool,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v593: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v628: f64,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v637: f64,
    pub(crate) scalar_v638: f64,
    pub(crate) scalar_v639: f64,
    pub(crate) scalar_v658: f64,
    pub(crate) scalar_v660: f64,
    pub(crate) scalar_v662: f64,
    pub(crate) scalar_v664: f64,
    pub(crate) scalar_v667: bool,
    pub(crate) scalar_v673: bool,
    pub(crate) scalar_v675: bool,
    pub(crate) scalar_v681: bool,
    pub(crate) scalar_v683: bool,
    pub(crate) scalar_v689: bool,
    pub(crate) scalar_v734: f64,
    pub(crate) scalar_v739: f64,
    pub(crate) scalar_v839: f64,
    pub(crate) scalar_v892: f64,
    pub(crate) scalar_v893: f64,
    pub(crate) scalar_v894: f64,
    pub(crate) scalar_v905: f64,
    pub(crate) scalar_v926: f64,
    pub(crate) scalar_v927: f64,
    pub(crate) scalar_v928: f64,
    pub(crate) scalar_v929: f64,
    pub(crate) scalar_v930: f64,
    pub(crate) scalar_v931: f64,
    pub(crate) scalar_v978: f64,
    pub(crate) scalar_v988: f64,
    pub(crate) scalar_v1001: f64,
    pub(crate) scalar_v1002: bool,
    pub(crate) scalar_v1006: bool,
    pub(crate) scalar_v1055: f64,
    pub(crate) scalar_v1056: f64,
    pub(crate) scalar_v1057: f64,
    pub(crate) scalar_v1079: f64,
    pub(crate) scalar_v1087: f64,
    pub(crate) scalar_v1088: bool,
    pub(crate) scalar_v1090: bool,
    pub(crate) scalar_v1091: bool,
    pub(crate) scalar_v1092: bool,
    pub(crate) scalar_v1095: bool,
    pub(crate) scalar_v1096: bool,
    pub(crate) scalar_v1101: f64,
    pub(crate) scalar_v1122: f64,
    pub(crate) scalar_v1124: f64,
    pub(crate) scalar_v1153: bool,
    pub(crate) scalar_v1159: bool,
    pub(crate) scalar_v1193: f64,
    pub(crate) scalar_v1215: f64,
    pub(crate) scalar_v1228: f64,
    pub(crate) scalar_v1246: f64,
    pub(crate) scalar_v1309: f64,
    pub(crate) scalar_v1310: bool,
    pub(crate) scalar_v1311: bool,
    pub(crate) scalar_v1312: bool,
    pub(crate) scalar_v1314: bool,
    pub(crate) scalar_v1315: bool,
    pub(crate) scalar_v1316: f64,
    pub(crate) scalar_v1409: bool,
    pub(crate) scalar_v1410: bool,
    pub(crate) scalar_v1411: bool,
    pub(crate) scalar_v1435: f64,
    pub(crate) scalar_v1437: f64,
    pub(crate) scalar_v1438: f64,
    pub(crate) scalar_v1440: f64,
    pub(crate) scalar_v1500: bool,
    pub(crate) scalar_v1501: bool,
    pub(crate) scalar_v1502: bool,
    pub(crate) scalar_v1528: f64,
    pub(crate) scalar_v1530: f64,
    pub(crate) scalar_v1531: f64,
    pub(crate) scalar_v1533: f64,
    pub(crate) scalar_v1610: f64,
    pub(crate) scalar_v1611: bool,
    pub(crate) scalar_v1612: bool,
    pub(crate) scalar_v1613: bool,
    pub(crate) scalar_v1616: f64,
    pub(crate) scalar_v1626: f64,
    pub(crate) scalar_v1627: bool,
    pub(crate) scalar_v1628: bool,
    pub(crate) scalar_v1640: f64,
    pub(crate) scalar_v1645: f64,
    pub(crate) scalar_v1662: bool,
    pub(crate) scalar_v1663: bool,
    pub(crate) scalar_v1667: f64,
    pub(crate) scalar_v1668: bool,
    pub(crate) scalar_v1671: f64,
    pub(crate) scalar_v1677: f64,
    pub(crate) scalar_v1688: f64,
    pub(crate) scalar_v1689: f64,
    pub(crate) scalar_v1690: f64,
    pub(crate) scalar_v1691: f64,
    pub(crate) scalar_v1692: f64,
    pub(crate) scalar_v1693: f64,
    pub(crate) scalar_v1694: f64,
    pub(crate) scalar_v1695: f64,
    pub(crate) scalar_v1696: f64,
    pub(crate) scalar_v1697: f64,
    pub(crate) scalar_v1698: f64,
    pub(crate) scalar_v1699: f64,
    pub(crate) scalar_v1700: f64,
    pub(crate) scalar_v1701: f64,
    pub(crate) scalar_v1702: f64,
    pub(crate) scalar_v1716: bool,
    pub(crate) scalar_v1743: f64,
    pub(crate) scalar_v1744: bool,
    pub(crate) scalar_v1745: f64,
    pub(crate) scalar_v1748: f64,
    pub(crate) scalar_v1767: f64,
    pub(crate) scalar_v1781: f64,
    pub(crate) scalar_v1786: bool,
    pub(crate) scalar_v1788: bool,
    pub(crate) scalar_v1792: f64,
    pub(crate) scalar_v1793: f64,
    pub(crate) scalar_v1794: f64,
    pub(crate) scalar_v1795: f64,
    pub(crate) scalar_v1796: f64,
    pub(crate) scalar_v1805: f64,
    pub(crate) scalar_v1806: bool,
    pub(crate) scalar_v1809: bool,
    pub(crate) scalar_v1832: f64,
    pub(crate) scalar_v1833: f64,
    pub(crate) scalar_v1839: f64,
    pub(crate) scalar_v1840: f64,
    pub(crate) scalar_v1841: f64,
    pub(crate) scalar_v1889: bool,
    pub(crate) scalar_v1890: bool,
    pub(crate) scalar_v1895: f64,
    pub(crate) scalar_v1899: f64,
    pub(crate) scalar_v1906: f64,
    pub(crate) scalar_v1911: f64,
    pub(crate) scalar_v1931: f64,
    pub(crate) scalar_v1951: f64,
    pub(crate) scalar_v1952: bool,
    pub(crate) scalar_v1987: bool,
    pub(crate) scalar_v1993: bool,
    pub(crate) scalar_v2049: f64,
    pub(crate) scalar_v2050: f64,
    pub(crate) scalar_v2080: f64,
    pub(crate) scalar_v2118: f64,
    pub(crate) scalar_v2154: f64,
    pub(crate) scalar_v2155: f64,
    pub(crate) scalar_v2177: f64,
    pub(crate) scalar_v2178: bool,
    pub(crate) scalar_v2187: f64,
    pub(crate) scalar_v2191: bool,
    pub(crate) scalar_v2210: bool,
    pub(crate) scalar_v2211: bool,
    pub(crate) scalar_v2212: bool,
    pub(crate) scalar_v2215: bool,
    pub(crate) scalar_v2231: f64,
    pub(crate) scalar_v2242: bool,
    pub(crate) scalar_v2263: f64,
    pub(crate) scalar_v2264: bool,
    pub(crate) scalar_v2265: f64,
    pub(crate) scalar_v2303: f64,
    pub(crate) scalar_v2304: f64,
    pub(crate) scalar_v2310: f64,
    pub(crate) scalar_v2314: f64,
    pub(crate) scalar_v2317: bool,
    pub(crate) scalar_v2321: f64,
    pub(crate) scalar_v2325: f64,
    pub(crate) scalar_v2326: bool,
    pub(crate) scalar_v2327: f64,
    pub(crate) scalar_v2328: bool,
    pub(crate) scalar_v2329: bool,
    pub(crate) scalar_v2333: f64,
    pub(crate) scalar_v2334: bool,
    pub(crate) scalar_v2335: bool,
    pub(crate) scalar_v2336: bool,
    pub(crate) scalar_v2337: bool,
    pub(crate) scalar_v2345: bool,
    pub(crate) scalar_v2346: bool,
    pub(crate) scalar_v2354: bool,
    pub(crate) scalar_v2359: f64,
    pub(crate) scalar_v2360: bool,
    pub(crate) scalar_v2364: bool,
    pub(crate) scalar_v2374: f64,
    pub(crate) scalar_v2375: bool,
    pub(crate) scalar_v2378: bool,
    pub(crate) scalar_v2379: bool,
    pub(crate) scalar_v2380: bool,
    pub(crate) scalar_v2381: f64,
    pub(crate) scalar_v2384: bool,
    pub(crate) scalar_v2385: bool,
    pub(crate) scalar_v2395: f64,
    pub(crate) scalar_v2396: f64,
    pub(crate) scalar_v2434: f64,
    pub(crate) scalar_v2438: f64,
    pub(crate) scalar_v2460: f64,
    pub(crate) scalar_v2465: f64,
    pub(crate) scalar_v2470: f64,
    pub(crate) scalar_v2471: f64,
    pub(crate) scalar_v2472: bool,
    pub(crate) scalar_v2473: f64,
    pub(crate) scalar_v2474: bool,
    pub(crate) scalar_v2475: f64,
    pub(crate) scalar_v2476: bool,
    pub(crate) scalar_v2477: f64,
    pub(crate) scalar_v2478: bool,
    pub(crate) scalar_v2479: f64,
    pub(crate) scalar_v2993: f64,
    pub(crate) scalar_v2994: f64,
    pub(crate) scalar_v2995: f64,
    pub(crate) scalar_v2996: f64,
    pub(crate) scalar_v3983: f64,
    pub(crate) scalar_v4007: f64,
    pub(crate) scalar_v4008: f64,
    pub(crate) scalar_v4025: f64,
    pub(crate) scalar_v4111: f64,
    pub(crate) scalar_v4130: f64,
    pub(crate) scalar_v4473: f64,
    pub(crate) scalar_v4474: f64,
    pub(crate) scalar_v4483: f64,
    pub(crate) scalar_v4484: f64,
    pub(crate) scalar_v4508: f64,
    pub(crate) scalar_v4509: f64,
    pub(crate) scalar_v4520: f64,
    pub(crate) scalar_v4521: f64,
    pub(crate) scalar_v4986: f64,
    pub(crate) scalar_v5043: f64,
    pub(crate) scalar_v5044: f64,
    pub(crate) scalar_v5107: f64,
    pub(crate) scalar_v5108: f64,
    pub(crate) scalar_v5241: f64,
    pub(crate) scalar_v5298: f64,
    pub(crate) scalar_v5299: f64,
    pub(crate) scalar_v5621: f64,
    pub(crate) scalar_v5622: f64,
    pub(crate) scalar_v5624: f64,
    pub(crate) scalar_v5625: f64,
    pub(crate) scalar_v5828: f64,
    pub(crate) scalar_v5829: f64,
    pub(crate) scalar_v5830: f64,
    pub(crate) scalar_v5831: f64,
    pub(crate) scalar_v5832: f64,
    pub(crate) scalar_v5833: f64,
    pub(crate) scalar_v6260: f64,
    pub(crate) scalar_v6867: f64,
    pub(crate) scalar_v6964: f64,
    pub(crate) scalar_v7277: f64,
    pub(crate) scalar_v7278: f64,
    pub(crate) scalar_v7279: f64,
    pub(crate) scalar_v7280: f64,
    pub(crate) scalar_v7281: f64,
    pub(crate) scalar_v7412: f64,
    pub(crate) scalar_v7413: f64,
    pub(crate) scalar_v7499: f64,
    pub(crate) scalar_v7500: f64,
    pub(crate) scalar_v7990: f64,
    pub(crate) scalar_v8100: f64,
    pub(crate) scalar_v8101: f64,
    pub(crate) scalar_v8102: f64,
    pub(crate) scalar_v8103: f64,
    pub(crate) scalar_v8416: f64,
    pub(crate) scalar_v8554: f64,
    pub(crate) scalar_v8555: f64,
    pub(crate) scalar_v8675: f64,
    pub(crate) scalar_v8681: f64,
    pub(crate) scalar_v8960: f64,
    pub(crate) scalar_v8961: f64,
    pub(crate) scalar_v9072: f64,
    pub(crate) scalar_v9073: f64,
    pub(crate) scalar_v9078: f64,
    pub(crate) scalar_v9079: f64,
    pub(crate) scalar_v9104: f64,
    pub(crate) scalar_v9105: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v665: f64,
    pub(crate) scalar_v666: f64,
    pub(crate) scalar_v2338: f64,
    pub(crate) scalar_v2339: f64,
    pub(crate) scalar_v2347: f64,
    pub(crate) scalar_v2348: f64,
    pub(crate) scalar_v2349: f64,
    pub(crate) scalar_v8668: f64,
    pub(crate) scalar_v8669: f64,
    pub(crate) scalar_v8670: f64,
    pub(crate) scalar_v8671: f64,
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
            scalar_v107: self.scalar_v107,
            scalar_v120: self.scalar_v120,
            scalar_v122: self.scalar_v122,
            scalar_v177: self.scalar_v177,
            scalar_v197: self.scalar_v197,
            scalar_v200: self.scalar_v200,
            scalar_v220: self.scalar_v220,
            scalar_v261: self.scalar_v261,
            scalar_v264: self.scalar_v264,
            scalar_v290: self.scalar_v290,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v299: self.scalar_v299,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v333: self.scalar_v333,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v370: self.scalar_v370,
            scalar_v372: self.scalar_v372,
            scalar_v373: self.scalar_v373,
            scalar_v374: self.scalar_v374,
            scalar_v401: self.scalar_v401,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v422: self.scalar_v422,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v432: self.scalar_v432,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v448: self.scalar_v448,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v465: self.scalar_v465,
            scalar_v469: self.scalar_v469,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v478: self.scalar_v478,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v499: self.scalar_v499,
            scalar_v500: self.scalar_v500,
            scalar_v506: self.scalar_v506,
            scalar_v507: self.scalar_v507,
            scalar_v508: self.scalar_v508,
            scalar_v513: self.scalar_v513,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v523: self.scalar_v523,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v534: self.scalar_v534,
            scalar_v535: self.scalar_v535,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v544: self.scalar_v544,
            scalar_v551: self.scalar_v551,
            scalar_v554: self.scalar_v554,
            scalar_v562: self.scalar_v562,
            scalar_v571: self.scalar_v571,
            scalar_v584: self.scalar_v584,
            scalar_v593: self.scalar_v593,
            scalar_v605: self.scalar_v605,
            scalar_v608: self.scalar_v608,
            scalar_v611: self.scalar_v611,
            scalar_v612: self.scalar_v612,
            scalar_v616: self.scalar_v616,
            scalar_v617: self.scalar_v617,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v627: self.scalar_v627,
            scalar_v628: self.scalar_v628,
            scalar_v632: self.scalar_v632,
            scalar_v635: self.scalar_v635,
            scalar_v637: self.scalar_v637,
            scalar_v638: self.scalar_v638,
            scalar_v639: self.scalar_v639,
            scalar_v658: self.scalar_v658,
            scalar_v660: self.scalar_v660,
            scalar_v662: self.scalar_v662,
            scalar_v664: self.scalar_v664,
            scalar_v667: self.scalar_v667,
            scalar_v673: self.scalar_v673,
            scalar_v675: self.scalar_v675,
            scalar_v681: self.scalar_v681,
            scalar_v683: self.scalar_v683,
            scalar_v689: self.scalar_v689,
            scalar_v734: self.scalar_v734,
            scalar_v739: self.scalar_v739,
            scalar_v839: self.scalar_v839,
            scalar_v892: self.scalar_v892,
            scalar_v893: self.scalar_v893,
            scalar_v894: self.scalar_v894,
            scalar_v905: self.scalar_v905,
            scalar_v926: self.scalar_v926,
            scalar_v927: self.scalar_v927,
            scalar_v928: self.scalar_v928,
            scalar_v929: self.scalar_v929,
            scalar_v930: self.scalar_v930,
            scalar_v931: self.scalar_v931,
            scalar_v978: self.scalar_v978,
            scalar_v988: self.scalar_v988,
            scalar_v1001: self.scalar_v1001,
            scalar_v1002: self.scalar_v1002,
            scalar_v1006: self.scalar_v1006,
            scalar_v1055: self.scalar_v1055,
            scalar_v1056: self.scalar_v1056,
            scalar_v1057: self.scalar_v1057,
            scalar_v1079: self.scalar_v1079,
            scalar_v1087: self.scalar_v1087,
            scalar_v1088: self.scalar_v1088,
            scalar_v1090: self.scalar_v1090,
            scalar_v1091: self.scalar_v1091,
            scalar_v1092: self.scalar_v1092,
            scalar_v1095: self.scalar_v1095,
            scalar_v1096: self.scalar_v1096,
            scalar_v1101: self.scalar_v1101,
            scalar_v1122: self.scalar_v1122,
            scalar_v1124: self.scalar_v1124,
            scalar_v1153: self.scalar_v1153,
            scalar_v1159: self.scalar_v1159,
            scalar_v1193: self.scalar_v1193,
            scalar_v1215: self.scalar_v1215,
            scalar_v1228: self.scalar_v1228,
            scalar_v1246: self.scalar_v1246,
            scalar_v1309: self.scalar_v1309,
            scalar_v1310: self.scalar_v1310,
            scalar_v1311: self.scalar_v1311,
            scalar_v1312: self.scalar_v1312,
            scalar_v1314: self.scalar_v1314,
            scalar_v1315: self.scalar_v1315,
            scalar_v1316: self.scalar_v1316,
            scalar_v1409: self.scalar_v1409,
            scalar_v1410: self.scalar_v1410,
            scalar_v1411: self.scalar_v1411,
            scalar_v1435: self.scalar_v1435,
            scalar_v1437: self.scalar_v1437,
            scalar_v1438: self.scalar_v1438,
            scalar_v1440: self.scalar_v1440,
            scalar_v1500: self.scalar_v1500,
            scalar_v1501: self.scalar_v1501,
            scalar_v1502: self.scalar_v1502,
            scalar_v1528: self.scalar_v1528,
            scalar_v1530: self.scalar_v1530,
            scalar_v1531: self.scalar_v1531,
            scalar_v1533: self.scalar_v1533,
            scalar_v1610: self.scalar_v1610,
            scalar_v1611: self.scalar_v1611,
            scalar_v1612: self.scalar_v1612,
            scalar_v1613: self.scalar_v1613,
            scalar_v1616: self.scalar_v1616,
            scalar_v1626: self.scalar_v1626,
            scalar_v1627: self.scalar_v1627,
            scalar_v1628: self.scalar_v1628,
            scalar_v1640: self.scalar_v1640,
            scalar_v1645: self.scalar_v1645,
            scalar_v1662: self.scalar_v1662,
            scalar_v1663: self.scalar_v1663,
            scalar_v1667: self.scalar_v1667,
            scalar_v1668: self.scalar_v1668,
            scalar_v1671: self.scalar_v1671,
            scalar_v1677: self.scalar_v1677,
            scalar_v1688: self.scalar_v1688,
            scalar_v1689: self.scalar_v1689,
            scalar_v1690: self.scalar_v1690,
            scalar_v1691: self.scalar_v1691,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1694: self.scalar_v1694,
            scalar_v1695: self.scalar_v1695,
            scalar_v1696: self.scalar_v1696,
            scalar_v1697: self.scalar_v1697,
            scalar_v1698: self.scalar_v1698,
            scalar_v1699: self.scalar_v1699,
            scalar_v1700: self.scalar_v1700,
            scalar_v1701: self.scalar_v1701,
            scalar_v1702: self.scalar_v1702,
            scalar_v1716: self.scalar_v1716,
            scalar_v1743: self.scalar_v1743,
            scalar_v1744: self.scalar_v1744,
            scalar_v1745: self.scalar_v1745,
            scalar_v1748: self.scalar_v1748,
            scalar_v1767: self.scalar_v1767,
            scalar_v1781: self.scalar_v1781,
            scalar_v1786: self.scalar_v1786,
            scalar_v1788: self.scalar_v1788,
            scalar_v1792: self.scalar_v1792,
            scalar_v1793: self.scalar_v1793,
            scalar_v1794: self.scalar_v1794,
            scalar_v1795: self.scalar_v1795,
            scalar_v1796: self.scalar_v1796,
            scalar_v1805: self.scalar_v1805,
            scalar_v1806: self.scalar_v1806,
            scalar_v1809: self.scalar_v1809,
            scalar_v1832: self.scalar_v1832,
            scalar_v1833: self.scalar_v1833,
            scalar_v1839: self.scalar_v1839,
            scalar_v1840: self.scalar_v1840,
            scalar_v1841: self.scalar_v1841,
            scalar_v1889: self.scalar_v1889,
            scalar_v1890: self.scalar_v1890,
            scalar_v1895: self.scalar_v1895,
            scalar_v1899: self.scalar_v1899,
            scalar_v1906: self.scalar_v1906,
            scalar_v1911: self.scalar_v1911,
            scalar_v1931: self.scalar_v1931,
            scalar_v1951: self.scalar_v1951,
            scalar_v1952: self.scalar_v1952,
            scalar_v1987: self.scalar_v1987,
            scalar_v1993: self.scalar_v1993,
            scalar_v2049: self.scalar_v2049,
            scalar_v2050: self.scalar_v2050,
            scalar_v2080: self.scalar_v2080,
            scalar_v2118: self.scalar_v2118,
            scalar_v2154: self.scalar_v2154,
            scalar_v2155: self.scalar_v2155,
            scalar_v2177: self.scalar_v2177,
            scalar_v2178: self.scalar_v2178,
            scalar_v2187: self.scalar_v2187,
            scalar_v2191: self.scalar_v2191,
            scalar_v2210: self.scalar_v2210,
            scalar_v2211: self.scalar_v2211,
            scalar_v2212: self.scalar_v2212,
            scalar_v2215: self.scalar_v2215,
            scalar_v2231: self.scalar_v2231,
            scalar_v2242: self.scalar_v2242,
            scalar_v2263: self.scalar_v2263,
            scalar_v2264: self.scalar_v2264,
            scalar_v2265: self.scalar_v2265,
            scalar_v2303: self.scalar_v2303,
            scalar_v2304: self.scalar_v2304,
            scalar_v2310: self.scalar_v2310,
            scalar_v2314: self.scalar_v2314,
            scalar_v2317: self.scalar_v2317,
            scalar_v2321: self.scalar_v2321,
            scalar_v2325: self.scalar_v2325,
            scalar_v2326: self.scalar_v2326,
            scalar_v2327: self.scalar_v2327,
            scalar_v2328: self.scalar_v2328,
            scalar_v2329: self.scalar_v2329,
            scalar_v2333: self.scalar_v2333,
            scalar_v2334: self.scalar_v2334,
            scalar_v2335: self.scalar_v2335,
            scalar_v2336: self.scalar_v2336,
            scalar_v2337: self.scalar_v2337,
            scalar_v2345: self.scalar_v2345,
            scalar_v2346: self.scalar_v2346,
            scalar_v2354: self.scalar_v2354,
            scalar_v2359: self.scalar_v2359,
            scalar_v2360: self.scalar_v2360,
            scalar_v2364: self.scalar_v2364,
            scalar_v2374: self.scalar_v2374,
            scalar_v2375: self.scalar_v2375,
            scalar_v2378: self.scalar_v2378,
            scalar_v2379: self.scalar_v2379,
            scalar_v2380: self.scalar_v2380,
            scalar_v2381: self.scalar_v2381,
            scalar_v2384: self.scalar_v2384,
            scalar_v2385: self.scalar_v2385,
            scalar_v2395: self.scalar_v2395,
            scalar_v2396: self.scalar_v2396,
            scalar_v2434: self.scalar_v2434,
            scalar_v2438: self.scalar_v2438,
            scalar_v2460: self.scalar_v2460,
            scalar_v2465: self.scalar_v2465,
            scalar_v2470: self.scalar_v2470,
            scalar_v2471: self.scalar_v2471,
            scalar_v2472: self.scalar_v2472,
            scalar_v2473: self.scalar_v2473,
            scalar_v2474: self.scalar_v2474,
            scalar_v2475: self.scalar_v2475,
            scalar_v2476: self.scalar_v2476,
            scalar_v2477: self.scalar_v2477,
            scalar_v2478: self.scalar_v2478,
            scalar_v2479: self.scalar_v2479,
            scalar_v2993: self.scalar_v2993,
            scalar_v2994: self.scalar_v2994,
            scalar_v2995: self.scalar_v2995,
            scalar_v2996: self.scalar_v2996,
            scalar_v3983: self.scalar_v3983,
            scalar_v4007: self.scalar_v4007,
            scalar_v4008: self.scalar_v4008,
            scalar_v4025: self.scalar_v4025,
            scalar_v4111: self.scalar_v4111,
            scalar_v4130: self.scalar_v4130,
            scalar_v4473: self.scalar_v4473,
            scalar_v4474: self.scalar_v4474,
            scalar_v4483: self.scalar_v4483,
            scalar_v4484: self.scalar_v4484,
            scalar_v4508: self.scalar_v4508,
            scalar_v4509: self.scalar_v4509,
            scalar_v4520: self.scalar_v4520,
            scalar_v4521: self.scalar_v4521,
            scalar_v4986: self.scalar_v4986,
            scalar_v5043: self.scalar_v5043,
            scalar_v5044: self.scalar_v5044,
            scalar_v5107: self.scalar_v5107,
            scalar_v5108: self.scalar_v5108,
            scalar_v5241: self.scalar_v5241,
            scalar_v5298: self.scalar_v5298,
            scalar_v5299: self.scalar_v5299,
            scalar_v5621: self.scalar_v5621,
            scalar_v5622: self.scalar_v5622,
            scalar_v5624: self.scalar_v5624,
            scalar_v5625: self.scalar_v5625,
            scalar_v5828: self.scalar_v5828,
            scalar_v5829: self.scalar_v5829,
            scalar_v5830: self.scalar_v5830,
            scalar_v5831: self.scalar_v5831,
            scalar_v5832: self.scalar_v5832,
            scalar_v5833: self.scalar_v5833,
            scalar_v6260: self.scalar_v6260,
            scalar_v6867: self.scalar_v6867,
            scalar_v6964: self.scalar_v6964,
            scalar_v7277: self.scalar_v7277,
            scalar_v7278: self.scalar_v7278,
            scalar_v7279: self.scalar_v7279,
            scalar_v7280: self.scalar_v7280,
            scalar_v7281: self.scalar_v7281,
            scalar_v7412: self.scalar_v7412,
            scalar_v7413: self.scalar_v7413,
            scalar_v7499: self.scalar_v7499,
            scalar_v7500: self.scalar_v7500,
            scalar_v7990: self.scalar_v7990,
            scalar_v8100: self.scalar_v8100,
            scalar_v8101: self.scalar_v8101,
            scalar_v8102: self.scalar_v8102,
            scalar_v8103: self.scalar_v8103,
            scalar_v8416: self.scalar_v8416,
            scalar_v8554: self.scalar_v8554,
            scalar_v8555: self.scalar_v8555,
            scalar_v8675: self.scalar_v8675,
            scalar_v8681: self.scalar_v8681,
            scalar_v8960: self.scalar_v8960,
            scalar_v8961: self.scalar_v8961,
            scalar_v9072: self.scalar_v9072,
            scalar_v9073: self.scalar_v9073,
            scalar_v9078: self.scalar_v9078,
            scalar_v9079: self.scalar_v9079,
            scalar_v9104: self.scalar_v9104,
            scalar_v9105: self.scalar_v9105,
            scalar_v20: self.scalar_v20,
            scalar_v663: self.scalar_v663,
            scalar_v665: self.scalar_v665,
            scalar_v666: self.scalar_v666,
            scalar_v2338: self.scalar_v2338,
            scalar_v2339: self.scalar_v2339,
            scalar_v2347: self.scalar_v2347,
            scalar_v2348: self.scalar_v2348,
            scalar_v2349: self.scalar_v2349,
            scalar_v8668: self.scalar_v8668,
            scalar_v8669: self.scalar_v8669,
            scalar_v8670: self.scalar_v8670,
            scalar_v8671: self.scalar_v8671,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 147;
    pub const VARIABLE_COUNT: usize = 585;
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
            scalar_v107: 0.0,
            scalar_v120: 0.0,
            scalar_v122: 0.0,
            scalar_v177: 0.0,
            scalar_v197: 0.0,
            scalar_v200: 0.0,
            scalar_v220: 0.0,
            scalar_v261: 0.0,
            scalar_v264: 0.0,
            scalar_v290: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v299: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v323: 0.0,
            scalar_v324: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v333: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v340: 0.0,
            scalar_v341: false,
            scalar_v342: 0.0,
            scalar_v370: false,
            scalar_v372: 0.0,
            scalar_v373: false,
            scalar_v374: 0.0,
            scalar_v401: false,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v422: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v432: 0.0,
            scalar_v437: 0.0,
            scalar_v438: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v448: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v465: 0.0,
            scalar_v469: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v478: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v496: 0.0,
            scalar_v497: false,
            scalar_v498: 0.0,
            scalar_v499: 0.0,
            scalar_v500: 0.0,
            scalar_v506: 0.0,
            scalar_v507: 0.0,
            scalar_v508: 0.0,
            scalar_v513: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v523: 0.0,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v534: 0.0,
            scalar_v535: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v544: 0.0,
            scalar_v551: 0.0,
            scalar_v554: 0.0,
            scalar_v562: 0.0,
            scalar_v571: 0.0,
            scalar_v584: 0.0,
            scalar_v593: 0.0,
            scalar_v605: 0.0,
            scalar_v608: 0.0,
            scalar_v611: 0.0,
            scalar_v612: 0.0,
            scalar_v616: 0.0,
            scalar_v617: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v627: 0.0,
            scalar_v628: 0.0,
            scalar_v632: 0.0,
            scalar_v635: 0.0,
            scalar_v637: 0.0,
            scalar_v638: 0.0,
            scalar_v639: 0.0,
            scalar_v658: 0.0,
            scalar_v660: 0.0,
            scalar_v662: 0.0,
            scalar_v664: 0.0,
            scalar_v667: false,
            scalar_v673: false,
            scalar_v675: false,
            scalar_v681: false,
            scalar_v683: false,
            scalar_v689: false,
            scalar_v734: 0.0,
            scalar_v739: 0.0,
            scalar_v839: 0.0,
            scalar_v892: 0.0,
            scalar_v893: 0.0,
            scalar_v894: 0.0,
            scalar_v905: 0.0,
            scalar_v926: 0.0,
            scalar_v927: 0.0,
            scalar_v928: 0.0,
            scalar_v929: 0.0,
            scalar_v930: 0.0,
            scalar_v931: 0.0,
            scalar_v978: 0.0,
            scalar_v988: 0.0,
            scalar_v1001: 0.0,
            scalar_v1002: false,
            scalar_v1006: false,
            scalar_v1055: 0.0,
            scalar_v1056: 0.0,
            scalar_v1057: 0.0,
            scalar_v1079: 0.0,
            scalar_v1087: 0.0,
            scalar_v1088: false,
            scalar_v1090: false,
            scalar_v1091: false,
            scalar_v1092: false,
            scalar_v1095: false,
            scalar_v1096: false,
            scalar_v1101: 0.0,
            scalar_v1122: 0.0,
            scalar_v1124: 0.0,
            scalar_v1153: false,
            scalar_v1159: false,
            scalar_v1193: 0.0,
            scalar_v1215: 0.0,
            scalar_v1228: 0.0,
            scalar_v1246: 0.0,
            scalar_v1309: 0.0,
            scalar_v1310: false,
            scalar_v1311: false,
            scalar_v1312: false,
            scalar_v1314: false,
            scalar_v1315: false,
            scalar_v1316: 0.0,
            scalar_v1409: false,
            scalar_v1410: false,
            scalar_v1411: false,
            scalar_v1435: 0.0,
            scalar_v1437: 0.0,
            scalar_v1438: 0.0,
            scalar_v1440: 0.0,
            scalar_v1500: false,
            scalar_v1501: false,
            scalar_v1502: false,
            scalar_v1528: 0.0,
            scalar_v1530: 0.0,
            scalar_v1531: 0.0,
            scalar_v1533: 0.0,
            scalar_v1610: 0.0,
            scalar_v1611: false,
            scalar_v1612: false,
            scalar_v1613: false,
            scalar_v1616: 0.0,
            scalar_v1626: 0.0,
            scalar_v1627: false,
            scalar_v1628: false,
            scalar_v1640: 0.0,
            scalar_v1645: 0.0,
            scalar_v1662: false,
            scalar_v1663: false,
            scalar_v1667: 0.0,
            scalar_v1668: false,
            scalar_v1671: 0.0,
            scalar_v1677: 0.0,
            scalar_v1688: 0.0,
            scalar_v1689: 0.0,
            scalar_v1690: 0.0,
            scalar_v1691: 0.0,
            scalar_v1692: 0.0,
            scalar_v1693: 0.0,
            scalar_v1694: 0.0,
            scalar_v1695: 0.0,
            scalar_v1696: 0.0,
            scalar_v1697: 0.0,
            scalar_v1698: 0.0,
            scalar_v1699: 0.0,
            scalar_v1700: 0.0,
            scalar_v1701: 0.0,
            scalar_v1702: 0.0,
            scalar_v1716: false,
            scalar_v1743: 0.0,
            scalar_v1744: false,
            scalar_v1745: 0.0,
            scalar_v1748: 0.0,
            scalar_v1767: 0.0,
            scalar_v1781: 0.0,
            scalar_v1786: false,
            scalar_v1788: false,
            scalar_v1792: 0.0,
            scalar_v1793: 0.0,
            scalar_v1794: 0.0,
            scalar_v1795: 0.0,
            scalar_v1796: 0.0,
            scalar_v1805: 0.0,
            scalar_v1806: false,
            scalar_v1809: false,
            scalar_v1832: 0.0,
            scalar_v1833: 0.0,
            scalar_v1839: 0.0,
            scalar_v1840: 0.0,
            scalar_v1841: 0.0,
            scalar_v1889: false,
            scalar_v1890: false,
            scalar_v1895: 0.0,
            scalar_v1899: 0.0,
            scalar_v1906: 0.0,
            scalar_v1911: 0.0,
            scalar_v1931: 0.0,
            scalar_v1951: 0.0,
            scalar_v1952: false,
            scalar_v1987: false,
            scalar_v1993: false,
            scalar_v2049: 0.0,
            scalar_v2050: 0.0,
            scalar_v2080: 0.0,
            scalar_v2118: 0.0,
            scalar_v2154: 0.0,
            scalar_v2155: 0.0,
            scalar_v2177: 0.0,
            scalar_v2178: false,
            scalar_v2187: 0.0,
            scalar_v2191: false,
            scalar_v2210: false,
            scalar_v2211: false,
            scalar_v2212: false,
            scalar_v2215: false,
            scalar_v2231: 0.0,
            scalar_v2242: false,
            scalar_v2263: 0.0,
            scalar_v2264: false,
            scalar_v2265: 0.0,
            scalar_v2303: 0.0,
            scalar_v2304: 0.0,
            scalar_v2310: 0.0,
            scalar_v2314: 0.0,
            scalar_v2317: false,
            scalar_v2321: 0.0,
            scalar_v2325: 0.0,
            scalar_v2326: false,
            scalar_v2327: 0.0,
            scalar_v2328: false,
            scalar_v2329: false,
            scalar_v2333: 0.0,
            scalar_v2334: false,
            scalar_v2335: false,
            scalar_v2336: false,
            scalar_v2337: false,
            scalar_v2345: false,
            scalar_v2346: false,
            scalar_v2354: false,
            scalar_v2359: 0.0,
            scalar_v2360: false,
            scalar_v2364: false,
            scalar_v2374: 0.0,
            scalar_v2375: false,
            scalar_v2378: false,
            scalar_v2379: false,
            scalar_v2380: false,
            scalar_v2381: 0.0,
            scalar_v2384: false,
            scalar_v2385: false,
            scalar_v2395: 0.0,
            scalar_v2396: 0.0,
            scalar_v2434: 0.0,
            scalar_v2438: 0.0,
            scalar_v2460: 0.0,
            scalar_v2465: 0.0,
            scalar_v2470: 0.0,
            scalar_v2471: 0.0,
            scalar_v2472: false,
            scalar_v2473: 0.0,
            scalar_v2474: false,
            scalar_v2475: 0.0,
            scalar_v2476: false,
            scalar_v2477: 0.0,
            scalar_v2478: false,
            scalar_v2479: 0.0,
            scalar_v2993: 0.0,
            scalar_v2994: 0.0,
            scalar_v2995: 0.0,
            scalar_v2996: 0.0,
            scalar_v3983: 0.0,
            scalar_v4007: 0.0,
            scalar_v4008: 0.0,
            scalar_v4025: 0.0,
            scalar_v4111: 0.0,
            scalar_v4130: 0.0,
            scalar_v4473: 0.0,
            scalar_v4474: 0.0,
            scalar_v4483: 0.0,
            scalar_v4484: 0.0,
            scalar_v4508: 0.0,
            scalar_v4509: 0.0,
            scalar_v4520: 0.0,
            scalar_v4521: 0.0,
            scalar_v4986: 0.0,
            scalar_v5043: 0.0,
            scalar_v5044: 0.0,
            scalar_v5107: 0.0,
            scalar_v5108: 0.0,
            scalar_v5241: 0.0,
            scalar_v5298: 0.0,
            scalar_v5299: 0.0,
            scalar_v5621: 0.0,
            scalar_v5622: 0.0,
            scalar_v5624: 0.0,
            scalar_v5625: 0.0,
            scalar_v5828: 0.0,
            scalar_v5829: 0.0,
            scalar_v5830: 0.0,
            scalar_v5831: 0.0,
            scalar_v5832: 0.0,
            scalar_v5833: 0.0,
            scalar_v6260: 0.0,
            scalar_v6867: 0.0,
            scalar_v6964: 0.0,
            scalar_v7277: 0.0,
            scalar_v7278: 0.0,
            scalar_v7279: 0.0,
            scalar_v7280: 0.0,
            scalar_v7281: 0.0,
            scalar_v7412: 0.0,
            scalar_v7413: 0.0,
            scalar_v7499: 0.0,
            scalar_v7500: 0.0,
            scalar_v7990: 0.0,
            scalar_v8100: 0.0,
            scalar_v8101: 0.0,
            scalar_v8102: 0.0,
            scalar_v8103: 0.0,
            scalar_v8416: 0.0,
            scalar_v8554: 0.0,
            scalar_v8555: 0.0,
            scalar_v8675: 0.0,
            scalar_v8681: 0.0,
            scalar_v8960: 0.0,
            scalar_v8961: 0.0,
            scalar_v9072: 0.0,
            scalar_v9073: 0.0,
            scalar_v9078: 0.0,
            scalar_v9079: 0.0,
            scalar_v9104: 0.0,
            scalar_v9105: 0.0,
            scalar_v20: 0.0,
            scalar_v663: 0.0,
            scalar_v665: 0.0,
            scalar_v666: 0.0,
            scalar_v2338: 0.0,
            scalar_v2339: 0.0,
            scalar_v2347: 0.0,
            scalar_v2348: 0.0,
            scalar_v2349: 0.0,
            scalar_v8668: 0.0,
            scalar_v8669: 0.0,
            scalar_v8670: 0.0,
            scalar_v8671: 0.0,
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
            scalar_v107,
            scalar_v120,
            scalar_v122,
            scalar_v177,
            scalar_v197,
            scalar_v200,
            scalar_v220,
            scalar_v261,
            scalar_v264,
            scalar_v290,
            scalar_v292,
            scalar_v293,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v316,
            scalar_v317,
            scalar_v323,
            scalar_v324,
            scalar_v328,
            scalar_v329,
            scalar_v333,
            scalar_v335,
            scalar_v336,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v370,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v401,
            scalar_v403,
            scalar_v404,
            scalar_v422,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v448,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v456,
            scalar_v457,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v469,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v478,
            scalar_v482,
            scalar_v483,
            scalar_v488,
            scalar_v489,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v527,
            scalar_v528,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v551,
            scalar_v554,
            scalar_v562,
            scalar_v571,
            scalar_v584,
            scalar_v593,
            scalar_v605,
            scalar_v608,
            scalar_v611,
            scalar_v612,
            scalar_v616,
            scalar_v617,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v627,
            scalar_v628,
            scalar_v632,
            scalar_v635,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v658,
            scalar_v660,
            scalar_v662,
            scalar_v664,
            scalar_v667,
            scalar_v673,
            scalar_v675,
            scalar_v681,
            scalar_v683,
            scalar_v689,
            scalar_v734,
            scalar_v739,
            scalar_v839,
            scalar_v892,
            scalar_v893,
            scalar_v894,
            scalar_v905,
            scalar_v926,
            scalar_v927,
            scalar_v928,
            scalar_v929,
            scalar_v930,
            scalar_v931,
            scalar_v978,
            scalar_v988,
            scalar_v1001,
            scalar_v1002,
            scalar_v1006,
            scalar_v1055,
            scalar_v1056,
            scalar_v1057,
            scalar_v1079,
            scalar_v1087,
            scalar_v1088,
            scalar_v1090,
            scalar_v1091,
            scalar_v1092,
            scalar_v1095,
            scalar_v1096,
            scalar_v1101,
            scalar_v1122,
            scalar_v1124,
            scalar_v1153,
            scalar_v1159,
            scalar_v1193,
            scalar_v1215,
            scalar_v1228,
            scalar_v1246,
            scalar_v1309,
            scalar_v1310,
            scalar_v1311,
            scalar_v1312,
            scalar_v1314,
            scalar_v1315,
            scalar_v1316,
            scalar_v1409,
            scalar_v1410,
            scalar_v1411,
            scalar_v1435,
            scalar_v1437,
            scalar_v1438,
            scalar_v1440,
            scalar_v1500,
            scalar_v1501,
            scalar_v1502,
            scalar_v1528,
            scalar_v1530,
            scalar_v1531,
            scalar_v1533,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1616,
            scalar_v1626,
            scalar_v1627,
            scalar_v1628,
            scalar_v1640,
            scalar_v1645,
            scalar_v1662,
            scalar_v1663,
            scalar_v1667,
            scalar_v1668,
            scalar_v1671,
            scalar_v1677,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1696,
            scalar_v1697,
            scalar_v1698,
            scalar_v1699,
            scalar_v1700,
            scalar_v1701,
            scalar_v1702,
            scalar_v1716,
            scalar_v1743,
            scalar_v1744,
            scalar_v1745,
            scalar_v1748,
            scalar_v1767,
            scalar_v1781,
            scalar_v1786,
            scalar_v1788,
            scalar_v1792,
            scalar_v1793,
            scalar_v1794,
            scalar_v1795,
            scalar_v1796,
            scalar_v1805,
            scalar_v1806,
            scalar_v1809,
            scalar_v1832,
            scalar_v1833,
            scalar_v1839,
            scalar_v1840,
            scalar_v1841,
            scalar_v1889,
            scalar_v1890,
            scalar_v1895,
            scalar_v1899,
            scalar_v1906,
            scalar_v1911,
            scalar_v1931,
            scalar_v1951,
            scalar_v1952,
            scalar_v1987,
            scalar_v1993,
            scalar_v2049,
            scalar_v2050,
            scalar_v2080,
            scalar_v2118,
            scalar_v2154,
            scalar_v2155,
            scalar_v2177,
            scalar_v2178,
            scalar_v2187,
            scalar_v2191,
            scalar_v2210,
            scalar_v2211,
            scalar_v2212,
            scalar_v2215,
            scalar_v2231,
            scalar_v2242,
            scalar_v2263,
            scalar_v2264,
            scalar_v2265,
            scalar_v2303,
            scalar_v2304,
            scalar_v2310,
            scalar_v2314,
            scalar_v2317,
            scalar_v2321,
            scalar_v2325,
            scalar_v2326,
            scalar_v2327,
            scalar_v2328,
            scalar_v2329,
            scalar_v2333,
            scalar_v2334,
            scalar_v2335,
            scalar_v2336,
            scalar_v2337,
            scalar_v2345,
            scalar_v2346,
            scalar_v2354,
            scalar_v2359,
            scalar_v2360,
            scalar_v2364,
            scalar_v2374,
            scalar_v2375,
            scalar_v2378,
            scalar_v2379,
            scalar_v2380,
            scalar_v2381,
            scalar_v2384,
            scalar_v2385,
            scalar_v2395,
            scalar_v2396,
            scalar_v2434,
            scalar_v2438,
            scalar_v2460,
            scalar_v2465,
            scalar_v2470,
            scalar_v2471,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v2476,
            scalar_v2477,
            scalar_v2478,
            scalar_v2479,
            scalar_v2993,
            scalar_v2994,
            scalar_v2995,
            scalar_v2996,
            scalar_v3983,
            scalar_v4007,
            scalar_v4008,
            scalar_v4025,
            scalar_v4111,
            scalar_v4130,
            scalar_v4473,
            scalar_v4474,
            scalar_v4483,
            scalar_v4484,
            scalar_v4508,
            scalar_v4509,
            scalar_v4520,
            scalar_v4521,
            scalar_v4986,
            scalar_v5043,
            scalar_v5044,
            scalar_v5107,
            scalar_v5108,
            scalar_v5241,
            scalar_v5298,
            scalar_v5299,
            scalar_v5621,
            scalar_v5622,
            scalar_v5624,
            scalar_v5625,
            scalar_v5828,
            scalar_v5829,
            scalar_v5830,
            scalar_v5831,
            scalar_v5832,
            scalar_v5833,
            scalar_v6260,
            scalar_v6867,
            scalar_v6964,
            scalar_v7277,
            scalar_v7278,
            scalar_v7279,
            scalar_v7280,
            scalar_v7281,
            scalar_v7412,
            scalar_v7413,
            scalar_v7499,
            scalar_v7500,
            scalar_v7990,
            scalar_v8100,
            scalar_v8101,
            scalar_v8102,
            scalar_v8103,
            scalar_v8416,
            scalar_v8554,
            scalar_v8555,
            scalar_v8675,
            scalar_v8681,
            scalar_v8960,
            scalar_v8961,
            scalar_v9072,
            scalar_v9073,
            scalar_v9078,
            scalar_v9079,
            scalar_v9104,
            scalar_v9105,
            scalar_v20,
            scalar_v663,
            scalar_v665,
            scalar_v666,
            scalar_v2338,
            scalar_v2339,
            scalar_v2347,
            scalar_v2348,
            scalar_v2349,
            scalar_v8668,
            scalar_v8669,
            scalar_v8670,
            scalar_v8671,
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
            scalar_v107,
            scalar_v120,
            scalar_v122,
            scalar_v177,
            scalar_v197,
            scalar_v200,
            scalar_v220,
            scalar_v261,
            scalar_v264,
            scalar_v290,
            scalar_v292,
            scalar_v293,
            scalar_v299,
            scalar_v302,
            scalar_v303,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v316,
            scalar_v317,
            scalar_v323,
            scalar_v324,
            scalar_v328,
            scalar_v329,
            scalar_v333,
            scalar_v335,
            scalar_v336,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v370,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v401,
            scalar_v403,
            scalar_v404,
            scalar_v422,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v448,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v456,
            scalar_v457,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v469,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v478,
            scalar_v482,
            scalar_v483,
            scalar_v488,
            scalar_v489,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v527,
            scalar_v528,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v551,
            scalar_v554,
            scalar_v562,
            scalar_v571,
            scalar_v584,
            scalar_v593,
            scalar_v605,
            scalar_v608,
            scalar_v611,
            scalar_v612,
            scalar_v616,
            scalar_v617,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v627,
            scalar_v628,
            scalar_v632,
            scalar_v635,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v658,
            scalar_v660,
            scalar_v662,
            scalar_v664,
            scalar_v667,
            scalar_v673,
            scalar_v675,
            scalar_v681,
            scalar_v683,
            scalar_v689,
            scalar_v734,
            scalar_v739,
            scalar_v839,
            scalar_v892,
            scalar_v893,
            scalar_v894,
            scalar_v905,
            scalar_v926,
            scalar_v927,
            scalar_v928,
            scalar_v929,
            scalar_v930,
            scalar_v931,
            scalar_v978,
            scalar_v988,
            scalar_v1001,
            scalar_v1002,
            scalar_v1006,
            scalar_v1055,
            scalar_v1056,
            scalar_v1057,
            scalar_v1079,
            scalar_v1087,
            scalar_v1088,
            scalar_v1090,
            scalar_v1091,
            scalar_v1092,
            scalar_v1095,
            scalar_v1096,
            scalar_v1101,
            scalar_v1122,
            scalar_v1124,
            scalar_v1153,
            scalar_v1159,
            scalar_v1193,
            scalar_v1215,
            scalar_v1228,
            scalar_v1246,
            scalar_v1309,
            scalar_v1310,
            scalar_v1311,
            scalar_v1312,
            scalar_v1314,
            scalar_v1315,
            scalar_v1316,
            scalar_v1409,
            scalar_v1410,
            scalar_v1411,
            scalar_v1435,
            scalar_v1437,
            scalar_v1438,
            scalar_v1440,
            scalar_v1500,
            scalar_v1501,
            scalar_v1502,
            scalar_v1528,
            scalar_v1530,
            scalar_v1531,
            scalar_v1533,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1616,
            scalar_v1626,
            scalar_v1627,
            scalar_v1628,
            scalar_v1640,
            scalar_v1645,
            scalar_v1662,
            scalar_v1663,
            scalar_v1667,
            scalar_v1668,
            scalar_v1671,
            scalar_v1677,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1696,
            scalar_v1697,
            scalar_v1698,
            scalar_v1699,
            scalar_v1700,
            scalar_v1701,
            scalar_v1702,
            scalar_v1716,
            scalar_v1743,
            scalar_v1744,
            scalar_v1745,
            scalar_v1748,
            scalar_v1767,
            scalar_v1781,
            scalar_v1786,
            scalar_v1788,
            scalar_v1792,
            scalar_v1793,
            scalar_v1794,
            scalar_v1795,
            scalar_v1796,
            scalar_v1805,
            scalar_v1806,
            scalar_v1809,
            scalar_v1832,
            scalar_v1833,
            scalar_v1839,
            scalar_v1840,
            scalar_v1841,
            scalar_v1889,
            scalar_v1890,
            scalar_v1895,
            scalar_v1899,
            scalar_v1906,
            scalar_v1911,
            scalar_v1931,
            scalar_v1951,
            scalar_v1952,
            scalar_v1987,
            scalar_v1993,
            scalar_v2049,
            scalar_v2050,
            scalar_v2080,
            scalar_v2118,
            scalar_v2154,
            scalar_v2155,
            scalar_v2177,
            scalar_v2178,
            scalar_v2187,
            scalar_v2191,
            scalar_v2210,
            scalar_v2211,
            scalar_v2212,
            scalar_v2215,
            scalar_v2231,
            scalar_v2242,
            scalar_v2263,
            scalar_v2264,
            scalar_v2265,
            scalar_v2303,
            scalar_v2304,
            scalar_v2310,
            scalar_v2314,
            scalar_v2317,
            scalar_v2321,
            scalar_v2325,
            scalar_v2326,
            scalar_v2327,
            scalar_v2328,
            scalar_v2329,
            scalar_v2333,
            scalar_v2334,
            scalar_v2335,
            scalar_v2336,
            scalar_v2337,
            scalar_v2345,
            scalar_v2346,
            scalar_v2354,
            scalar_v2359,
            scalar_v2360,
            scalar_v2364,
            scalar_v2374,
            scalar_v2375,
            scalar_v2378,
            scalar_v2379,
            scalar_v2380,
            scalar_v2381,
            scalar_v2384,
            scalar_v2385,
            scalar_v2395,
            scalar_v2396,
            scalar_v2434,
            scalar_v2438,
            scalar_v2460,
            scalar_v2465,
            scalar_v2470,
            scalar_v2471,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v2476,
            scalar_v2477,
            scalar_v2478,
            scalar_v2479,
            scalar_v2993,
            scalar_v2994,
            scalar_v2995,
            scalar_v2996,
            scalar_v3983,
            scalar_v4007,
            scalar_v4008,
            scalar_v4025,
            scalar_v4111,
            scalar_v4130,
            scalar_v4473,
            scalar_v4474,
            scalar_v4483,
            scalar_v4484,
            scalar_v4508,
            scalar_v4509,
            scalar_v4520,
            scalar_v4521,
            scalar_v4986,
            scalar_v5043,
            scalar_v5044,
            scalar_v5107,
            scalar_v5108,
            scalar_v5241,
            scalar_v5298,
            scalar_v5299,
            scalar_v5621,
            scalar_v5622,
            scalar_v5624,
            scalar_v5625,
            scalar_v5828,
            scalar_v5829,
            scalar_v5830,
            scalar_v5831,
            scalar_v5832,
            scalar_v5833,
            scalar_v6260,
            scalar_v6867,
            scalar_v6964,
            scalar_v7277,
            scalar_v7278,
            scalar_v7279,
            scalar_v7280,
            scalar_v7281,
            scalar_v7412,
            scalar_v7413,
            scalar_v7499,
            scalar_v7500,
            scalar_v7990,
            scalar_v8100,
            scalar_v8101,
            scalar_v8102,
            scalar_v8103,
            scalar_v8416,
            scalar_v8554,
            scalar_v8555,
            scalar_v8675,
            scalar_v8681,
            scalar_v8960,
            scalar_v8961,
            scalar_v9072,
            scalar_v9073,
            scalar_v9078,
            scalar_v9079,
            scalar_v9104,
            scalar_v9105,
            scalar_v20,
            scalar_v663,
            scalar_v665,
            scalar_v666,
            scalar_v2338,
            scalar_v2339,
            scalar_v2347,
            scalar_v2348,
            scalar_v2349,
            scalar_v8668,
            scalar_v8669,
            scalar_v8670,
            scalar_v8671,
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
            "swnlsh" => { validate_parameter("swnlsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ath" => { validate_finite_parameter("ath", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505t_va'", name)),
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
        let v21: f64 = p.p141;
        self.scalar_v21 = v21;
        let v22: bool = (0.0 == p.p141);
        self.scalar_v22 = v22;
        let v24: f64 = (if v22 { 1e-12 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: bool = (!v22);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p141 } else { v24 });
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
        let v107: f64 = p.p124;
        self.scalar_v107 = v107;
        let v120: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v120 = v120;
        let v122: f64 = (1.0 / v120);
        self.scalar_v122 = v122;
        let v177: f64 = p.p104;
        self.scalar_v177 = v177;
        let v197: f64 = p.p63;
        self.scalar_v197 = v197;
        let v200: f64 = p.p109;
        self.scalar_v200 = v200;
        let v220: f64 = p.p79;
        self.scalar_v220 = v220;
        let v261: f64 = p.p26;
        self.scalar_v261 = v261;
        let v264: f64 = p.p108;
        self.scalar_v264 = v264;
        let v290: f64 = p.p64;
        self.scalar_v290 = v290;
        let v292: f64 = p.p74;
        self.scalar_v292 = v292;
        let v293: f64 = (1.0 - p.p74);
        self.scalar_v293 = v293;
        let v299: f64 = p.p69;
        self.scalar_v299 = v299;
        let v302: f64 = p.p53;
        self.scalar_v302 = v302;
        let v303: f64 = p.p96;
        self.scalar_v303 = v303;
        let v309: f64 = p.p55;
        self.scalar_v309 = v309;
        let v310: f64 = p.p97;
        self.scalar_v310 = v310;
        let v311: f64 = p.p95;
        self.scalar_v311 = v311;
        let v312: f64 = (p.p97 - p.p95);
        self.scalar_v312 = v312;
        let v316: f64 = p.p54;
        self.scalar_v316 = v316;
        let v317: f64 = p.p100;
        self.scalar_v317 = v317;
        let v323: f64 = p.p56;
        self.scalar_v323 = v323;
        let v324: f64 = p.p101;
        self.scalar_v324 = v324;
        let v328: f64 = p.p57;
        self.scalar_v328 = v328;
        let v329: f64 = p.p103;
        self.scalar_v329 = v329;
        let v333: f64 = p.p58;
        self.scalar_v333 = v333;
        let v335: f64 = p.p59;
        self.scalar_v335 = v335;
        let v336: f64 = p.p98;
        self.scalar_v336 = v336;
        let v340: f64 = p.p121;
        self.scalar_v340 = v340;
        let v341: bool = (0.0 != p.p121);
        self.scalar_v341 = v341;
        let v342: f64 = p.p9;
        self.scalar_v342 = v342;
        let v370: bool = (!v341);
        self.scalar_v370 = v370;
        let v372: f64 = p.p122;
        self.scalar_v372 = v372;
        let v373: bool = (0.0 != p.p122);
        self.scalar_v373 = v373;
        let v374: f64 = p.p10;
        self.scalar_v374 = v374;
        let v401: bool = (!v373);
        self.scalar_v401 = v401;
        let v403: f64 = p.p42;
        self.scalar_v403 = v403;
        let v404: f64 = p.p123;
        self.scalar_v404 = v404;
        let v422: f64 = p.p8;
        self.scalar_v422 = v422;
        let v424: f64 = (4.0 - p.p97);
        self.scalar_v424 = v424;
        let v425: f64 = (v424 - p.p95);
        self.scalar_v425 = v425;
        let v426: f64 = p.p120;
        self.scalar_v426 = v426;
        let v427: f64 = (v425 + p.p120);
        self.scalar_v427 = v427;
        let v432: f64 = (-p.p104);
        self.scalar_v432 = v432;
        let v437: f64 = p.p11;
        self.scalar_v437 = v437;
        let v438: f64 = (1.0 - p.p97);
        self.scalar_v438 = v438;
        let v442: f64 = p.p29;
        self.scalar_v442 = v442;
        let v443: f64 = p.p102;
        self.scalar_v443 = v443;
        let v444: f64 = (1.0 - p.p102);
        self.scalar_v444 = v444;
        let v448: f64 = p.p19;
        self.scalar_v448 = v448;
        let v450: f64 = p.p20;
        self.scalar_v450 = v450;
        let v451: f64 = (2.0 * p.p20);
        self.scalar_v451 = v451;
        let v452: f64 = (6.0 - v451);
        self.scalar_v452 = v452;
        let v456: f64 = p.p112;
        self.scalar_v456 = v456;
        let v457: f64 = (-p.p112);
        self.scalar_v457 = v457;
        let v462: f64 = p.p30;
        self.scalar_v462 = v462;
        let v463: f64 = p.p31;
        self.scalar_v463 = v463;
        let v464: f64 = (2.0 * p.p31);
        self.scalar_v464 = v464;
        let v465: f64 = (6.0 - v464);
        self.scalar_v465 = v465;
        let v469: f64 = (-p.p109);
        self.scalar_v469 = v469;
        let v474: f64 = p.p15;
        self.scalar_v474 = v474;
        let v475: f64 = (4.0 - p.p96);
        self.scalar_v475 = v475;
        let v476: f64 = (p.p120 + v475);
        self.scalar_v476 = v476;
        let v478: f64 = p.p16;
        self.scalar_v478 = v478;
        let v482: f64 = p.p110;
        self.scalar_v482 = v482;
        let v483: f64 = (-p.p110);
        self.scalar_v483 = v483;
        let v488: f64 = p.p17;
        self.scalar_v488 = v488;
        let v489: f64 = p.p18;
        self.scalar_v489 = v489;
        let v496: f64 = p.p23;
        self.scalar_v496 = v496;
        let v497: bool = (1.0 == p.p23);
        self.scalar_v497 = v497;
        let v498: f64 = p.p24;
        self.scalar_v498 = v498;
        let v499: f64 = p.p106;
        self.scalar_v499 = v499;
        let v500: f64 = (-p.p106);
        self.scalar_v500 = v500;
        let v506: f64 = p.p27;
        self.scalar_v506 = v506;
        let v507: f64 = p.p105;
        self.scalar_v507 = v507;
        let v508: f64 = (-p.p105);
        self.scalar_v508 = v508;
        let v513: f64 = p.p25;
        self.scalar_v513 = v513;
        let v514: f64 = p.p107;
        self.scalar_v514 = v514;
        let v515: f64 = (-p.p107);
        self.scalar_v515 = v515;
        let v521: f64 = p.p28;
        self.scalar_v521 = v521;
        let v522: f64 = (4.0 - p.p102);
        self.scalar_v522 = v522;
        let v523: f64 = (p.p120 + v522);
        self.scalar_v523 = v523;
        let v527: f64 = p.p111;
        self.scalar_v527 = v527;
        let v528: f64 = (-p.p111);
        self.scalar_v528 = v528;
        let v532: f64 = p.p21;
        self.scalar_v532 = v532;
        let v533: f64 = p.p22;
        self.scalar_v533 = v533;
        let v534: f64 = (2.0 * p.p22);
        self.scalar_v534 = v534;
        let v535: f64 = (6.0 - v534);
        self.scalar_v535 = v535;
        let v542: f64 = p.p136;
        self.scalar_v542 = v542;
        let v543: f64 = p.p137;
        self.scalar_v543 = v543;
        let v544: f64 = (4.0 / p.p137);
        self.scalar_v544 = v544;
        let v551: f64 = p.p142;
        self.scalar_v551 = v551;
        let v554: f64 = p.p144;
        self.scalar_v554 = v554;
        let v562: f64 = p.p34;
        self.scalar_v562 = v562;
        let v571: f64 = p.p33;
        self.scalar_v571 = v571;
        let v584: f64 = p.p36;
        self.scalar_v584 = v584;
        let v593: f64 = p.p35;
        self.scalar_v593 = v593;
        let v605: f64 = p.p13;
        self.scalar_v605 = v605;
        let v608: f64 = p.p12;
        self.scalar_v608 = v608;
        let v611: f64 = p.p85;
        self.scalar_v611 = v611;
        let v612: f64 = (p.p97 - 2.0);
        self.scalar_v612 = v612;
        let v616: f64 = p.p119;
        self.scalar_v616 = v616;
        let v617: f64 = (-p.p119);
        self.scalar_v617 = v617;
        let v621: f64 = p.p86;
        self.scalar_v621 = v621;
        let v622: f64 = (p.p97 + p.p95);
        self.scalar_v622 = v622;
        let v623: f64 = (v622 - 1.0);
        self.scalar_v623 = v623;
        let v627: f64 = p.p87;
        self.scalar_v627 = v627;
        let v628: f64 = (p.p98 - 1.0);
        self.scalar_v628 = v628;
        let v632: f64 = p.p88;
        self.scalar_v632 = v632;
        let v635: f64 = (p.p86 + p.p87);
        self.scalar_v635 = v635;
        let v637: f64 = p.p89;
        self.scalar_v637 = v637;
        let v638: f64 = p.p99;
        self.scalar_v638 = v638;
        let v639: f64 = (p.p99 - 1.0);
        self.scalar_v639 = v639;
        let v658: f64 = (v12 * 1.081);
        self.scalar_v658 = v658;
        let v660: f64 = p.p91;
        self.scalar_v660 = v660;
        let v662: f64 = p.p133;
        self.scalar_v662 = v662;
        let v664: f64 = p.p135;
        self.scalar_v664 = v664;
        let v667: bool = (p.p56 > 0.0);
        self.scalar_v667 = v667;
        let v673: bool = (!v667);
        self.scalar_v673 = v673;
        let v675: bool = (p.p57 > 0.0);
        self.scalar_v675 = v675;
        let v681: bool = (!v675);
        self.scalar_v681 = v681;
        let v683: bool = (p.p58 > 0.0);
        self.scalar_v683 = v683;
        let v689: bool = (!v683);
        self.scalar_v689 = v689;
        let v734: f64 = p.p138;
        self.scalar_v734 = v734;
        let v739: f64 = ((p.p138) as f64).exp();
        self.scalar_v739 = v739;
        let v839: f64 = p.p140;
        self.scalar_v839 = v839;
        let v892: f64 = p.p61;
        self.scalar_v892 = v892;
        let v893: f64 = p.p60;
        self.scalar_v893 = v893;
        let v894: f64 = (p.p61 * p.p60);
        self.scalar_v894 = v894;
        let v905: f64 = p.p62;
        self.scalar_v905 = v905;
        let v926: f64 = (-1.0 / p.p62);
        self.scalar_v926 = v926;
        let v927: f64 = ((v926) as f64).exp();
        self.scalar_v927 = v927;
        let v928: f64 = (1.0 + v927);
        self.scalar_v928 = v928;
        let v929: f64 = ((v928) as f64).ln();
        self.scalar_v929 = v929;
        let v930: f64 = (p.p62 * v929);
        self.scalar_v930 = v930;
        let v931: f64 = (1.0 + v930);
        self.scalar_v931 = v931;
        let v978: f64 = p.p139;
        self.scalar_v978 = v978;
        let v988: f64 = (0.5 * p.p60);
        self.scalar_v988 = v988;
        let v1001: f64 = p.p72;
        self.scalar_v1001 = v1001;
        let v1002: bool = (0.0 == p.p72);
        self.scalar_v1002 = v1002;
        let v1006: bool = (!v1002);
        self.scalar_v1006 = v1006;
        let v1055: f64 = (-1.0 / p.p66);
        self.scalar_v1055 = v1055;
        let v1056: f64 = f64::powf(3.0, v1055);
        self.scalar_v1056 = v1056;
        let v1057: f64 = (1.0 - v1056);
        self.scalar_v1057 = v1057;
        let v1079: f64 = (1.0 - p.p66);
        self.scalar_v1079 = v1079;
        let v1087: f64 = p.p73;
        self.scalar_v1087 = v1087;
        let v1088: bool = (1.0 == p.p73);
        self.scalar_v1088 = v1088;
        let v1090: bool = (2.0 == p.p73);
        self.scalar_v1090 = v1090;
        let v1091: bool = (!v1088);
        self.scalar_v1091 = v1091;
        let v1092: bool = (v1090 && v1091);
        self.scalar_v1092 = v1092;
        let v1095: bool = (!v1090);
        self.scalar_v1095 = v1095;
        let v1096: bool = (v1091 && v1095);
        self.scalar_v1096 = v1096;
        let v1101: f64 = (-1.0 / p.p71);
        self.scalar_v1101 = v1101;
        let v1122: f64 = p.p75;
        self.scalar_v1122 = v1122;
        let v1124: f64 = (1.0 - p.p71);
        self.scalar_v1124 = v1124;
        let v1153: bool = (0.0 == p.p91);
        self.scalar_v1153 = v1153;
        let v1159: bool = (!v1153);
        self.scalar_v1159 = v1159;
        let v1193: f64 = p.p14;
        self.scalar_v1193 = v1193;
        let v1215: f64 = p.p143;
        self.scalar_v1215 = v1215;
        let v1228: f64 = p.p145;
        self.scalar_v1228 = v1228;
        let v1246: f64 = p.p146;
        self.scalar_v1246 = v1246;
        let v1309: f64 = p.p92;
        self.scalar_v1309 = v1309;
        let v1310: bool = (0.0 == p.p92);
        self.scalar_v1310 = v1310;
        let v1311: bool = (!v497);
        self.scalar_v1311 = v1311;
        let v1312: bool = (v1310 && v1311);
        self.scalar_v1312 = v1312;
        let v1314: bool = (!v1310);
        self.scalar_v1314 = v1314;
        let v1315: bool = (v1311 && v1314);
        self.scalar_v1315 = v1315;
        let v1316: f64 = (1.0 - p.p92);
        self.scalar_v1316 = v1316;
        let v1409: bool = (p.p33 > 0.0);
        self.scalar_v1409 = v1409;
        let v1410: bool = (p.p34 > 0.0);
        self.scalar_v1410 = v1410;
        let v1411: bool = (v1409 && v1410);
        self.scalar_v1411 = v1411;
        let v1435: f64 = (-2.0 - p.p66);
        self.scalar_v1435 = v1435;
        let v1437: f64 = (p.p66 * p.p66);
        self.scalar_v1437 = v1437;
        let v1438: f64 = (1.0 - v1437);
        self.scalar_v1438 = v1438;
        let v1440: f64 = (p.p66 - 1.0);
        self.scalar_v1440 = v1440;
        let v1500: bool = (p.p35 > 0.0);
        self.scalar_v1500 = v1500;
        let v1501: bool = (p.p36 > 0.0);
        self.scalar_v1501 = v1501;
        let v1502: bool = (v1500 && v1501);
        self.scalar_v1502 = v1502;
        let v1528: f64 = (-2.0 - p.p71);
        self.scalar_v1528 = v1528;
        let v1530: f64 = (p.p71 * p.p71);
        self.scalar_v1530 = v1530;
        let v1531: f64 = (1.0 - v1530);
        self.scalar_v1531 = v1531;
        let v1533: f64 = (p.p71 - 1.0);
        self.scalar_v1533 = v1533;
        let v1610: f64 = p.p5;
        self.scalar_v1610 = v1610;
        let v1611: bool = (p.p5 > 0.0);
        self.scalar_v1611 = v1611;
        let v1612: bool = (p.p32 > 0.0);
        self.scalar_v1612 = v1612;
        let v1613: bool = (v1611 && v1612);
        self.scalar_v1613 = v1613;
        let v1616: f64 = (p.p32 * 2.0);
        self.scalar_v1616 = v1616;
        let v1626: f64 = (if v1613 { 0.0 } else { 0.0 });
        self.scalar_v1626 = v1626;
        let v1627: bool = (1.0 == p.p5);
        self.scalar_v1627 = v1627;
        let v1628: bool = (v1613 && v1627);
        self.scalar_v1628 = v1628;
        let v1640: f64 = (if v1628 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1640 = v1640;
        let v1645: f64 = (0.5 * v1640);
        self.scalar_v1645 = v1645;
        let v1662: bool = (!v1627);
        self.scalar_v1662 = v1662;
        let v1663: bool = (v1613 && v1662);
        self.scalar_v1663 = v1663;
        let v1667: f64 = p.p83;
        self.scalar_v1667 = v1667;
        let v1668: bool = (1.0 == p.p83);
        self.scalar_v1668 = v1668;
        let v1671: f64 = (if v1668 { 1e-12 } else { v1640 });
        self.scalar_v1671 = v1671;
        let v1677: f64 = (0.5 * v1671);
        self.scalar_v1677 = v1677;
        let v1688: f64 = p.p81;
        self.scalar_v1688 = v1688;
        let v1689: f64 = f64::powf(v100, p.p81);
        self.scalar_v1689 = v1689;
        let v1690: f64 = (1.0 - v1689);
        self.scalar_v1690 = v1690;
        let v1691: f64 = (1.0 / v1690);
        self.scalar_v1691 = v1691;
        let v1692: f64 = (if v1668 { v1691 } else { 0.0 });
        self.scalar_v1692 = v1692;
        let v1693: f64 = p.p80;
        self.scalar_v1693 = v1693;
        let v1694: f64 = (v100 * p.p80);
        self.scalar_v1694 = v1694;
        let v1695: f64 = (if v1668 { v1694 } else { 0.0 });
        self.scalar_v1695 = v1695;
        let v1696: f64 = (v1692 * v1692);
        self.scalar_v1696 = v1696;
        let v1697: f64 = (p.p81 - 1.0);
        self.scalar_v1697 = v1697;
        let v1698: f64 = f64::powf(v100, v1697);
        self.scalar_v1698 = v1698;
        let v1699: f64 = (v1696 * v1698);
        self.scalar_v1699 = v1699;
        let v1700: f64 = (p.p81 * v1699);
        self.scalar_v1700 = v1700;
        let v1701: f64 = (v1700 / p.p80);
        self.scalar_v1701 = v1701;
        let v1702: f64 = (if v1668 { v1701 } else { 0.0 });
        self.scalar_v1702 = v1702;
        let v1716: bool = (!v1668);
        self.scalar_v1716 = v1716;
        let v1743: f64 = p.p38;
        self.scalar_v1743 = v1743;
        let v1744: bool = (1.0 == p.p38);
        self.scalar_v1744 = v1744;
        let v1745: f64 = p.p43;
        self.scalar_v1745 = v1745;
        let v1748: f64 = p.p41;
        self.scalar_v1748 = v1748;
        let v1767: f64 = p.p40;
        self.scalar_v1767 = v1767;
        let v1781: f64 = p.p39;
        self.scalar_v1781 = v1781;
        let v1786: bool = (2.0 == p.p38);
        self.scalar_v1786 = v1786;
        let v1788: bool = (!v1744);
        self.scalar_v1788 = v1788;
        let v1792: f64 = p.p45;
        self.scalar_v1792 = v1792;
        let v1793: f64 = (2.0 * p.p45);
        self.scalar_v1793 = v1793;
        let v1794: f64 = p.p44;
        self.scalar_v1794 = v1794;
        let v1795: f64 = (p.p44 * p.p44);
        self.scalar_v1795 = v1795;
        let v1796: f64 = (v1793 / v1795);
        self.scalar_v1796 = v1796;
        let v1805: f64 = p.p7;
        self.scalar_v1805 = v1805;
        let v1806: bool = (0.0 == p.p7);
        self.scalar_v1806 = v1806;
        let v1809: bool = (!v1806);
        self.scalar_v1809 = v1809;
        let v1832: f64 = p.p46;
        self.scalar_v1832 = v1832;
        let v1833: f64 = (2.0 * p.p46);
        self.scalar_v1833 = v1833;
        let v1839: f64 = (1.0 + p.p46);
        self.scalar_v1839 = v1839;
        let v1840: f64 = (1.0 + v1833);
        self.scalar_v1840 = v1840;
        let v1841: f64 = (v1839 / v1840);
        self.scalar_v1841 = v1841;
        let v1889: bool = (3.0 == p.p38);
        self.scalar_v1889 = v1889;
        let v1890: bool = (!v1786);
        self.scalar_v1890 = v1890;
        let v1895: f64 = p.p47;
        self.scalar_v1895 = v1895;
        let v1899: f64 = p.p48;
        self.scalar_v1899 = v1899;
        let v1906: f64 = p.p51;
        self.scalar_v1906 = v1906;
        let v1911: f64 = p.p50;
        self.scalar_v1911 = v1911;
        let v1931: f64 = p.p49;
        self.scalar_v1931 = v1931;
        let v1951: f64 = p.p52;
        self.scalar_v1951 = v1951;
        let v1952: bool = (1.0 == p.p52);
        self.scalar_v1952 = v1952;
        let v1987: bool = (!v1889);
        self.scalar_v1987 = v1987;
        let v1993: bool = (!v1952);
        self.scalar_v1993 = v1993;
        let v2049: f64 = p.p67;
        self.scalar_v2049 = v2049;
        let v2050: f64 = (1.0 - p.p67);
        self.scalar_v2050 = v2050;
        let v2080: f64 = p.p76;
        self.scalar_v2080 = v2080;
        let v2118: f64 = (1.0 - p.p76);
        self.scalar_v2118 = v2118;
        let v2154: f64 = p.p84;
        self.scalar_v2154 = v2154;
        let v2155: f64 = (1.0 / p.p84);
        self.scalar_v2155 = v2155;
        let v2177: f64 = p.p78;
        self.scalar_v2177 = v2177;
        let v2178: bool = (0.0 == p.p78);
        self.scalar_v2178 = v2178;
        let v2187: f64 = p.p90;
        self.scalar_v2187 = v2187;
        let v2191: bool = (!v2178);
        self.scalar_v2191 = v2191;
        let v2210: bool = (3.0 == p.p5);
        self.scalar_v2210 = v2210;
        let v2211: bool = (v1627 || v2210);
        self.scalar_v2211 = v2211;
        let v2212: bool = (v1612 && v2211);
        self.scalar_v2212 = v2212;
        let v2215: bool = (v2178 && v2212);
        self.scalar_v2215 = v2215;
        let v2231: f64 = (p.p32 * 0.5);
        self.scalar_v2231 = v2231;
        let v2242: bool = (v2191 && v2212);
        self.scalar_v2242 = v2242;
        let v2263: f64 = p.p6;
        self.scalar_v2263 = v2263;
        let v2264: bool = (1.0 == p.p6);
        self.scalar_v2264 = v2264;
        let v2265: f64 = (-p.p66);
        self.scalar_v2265 = v2265;
        let v2303: f64 = p.p94;
        self.scalar_v2303 = v2303;
        let v2304: f64 = (1.0 - p.p94);
        self.scalar_v2304 = v2304;
        let v2310: f64 = p.p93;
        self.scalar_v2310 = v2310;
        let v2314: f64 = (1.0 - p.p93);
        self.scalar_v2314 = v2314;
        let v2317: bool = (!v2264);
        self.scalar_v2317 = v2317;
        let v2321: f64 = p.p134;
        self.scalar_v2321 = v2321;
        let v2325: f64 = (1.0 - p.p135);
        self.scalar_v2325 = v2325;
        let v2326: bool = (p.p133 > v28);
        self.scalar_v2326 = v2326;
        let v2327: f64 = p.p132;
        self.scalar_v2327 = v2327;
        let v2328: bool = (0.0 == p.p132);
        self.scalar_v2328 = v2328;
        let v2329: bool = (v2326 && v2328);
        self.scalar_v2329 = v2329;
        let v2333: f64 = ((v2325) as f64).abs();
        self.scalar_v2333 = v2333;
        let v2334: bool = (v2333 < 1e-6);
        self.scalar_v2334 = v2334;
        let v2335: bool = (!v2328);
        self.scalar_v2335 = v2335;
        let v2336: bool = (v2326 && v2335);
        self.scalar_v2336 = v2336;
        let v2337: bool = (v2334 && v2336);
        self.scalar_v2337 = v2337;
        let v2345: bool = (!v2334);
        self.scalar_v2345 = v2345;
        let v2346: bool = (v2336 && v2345);
        self.scalar_v2346 = v2346;
        let v2354: bool = (!v2326);
        self.scalar_v2354 = v2354;
        let v2359: f64 = p.p129;
        self.scalar_v2359 = v2359;
        let v2360: bool = (p.p129 > 0.0);
        self.scalar_v2360 = v2360;
        let v2364: bool = (!v2360);
        self.scalar_v2364 = v2364;
        let v2374: f64 = p.p130;
        self.scalar_v2374 = v2374;
        let v2375: bool = (1.0 == p.p130);
        self.scalar_v2375 = v2375;
        let v2378: bool = (2.0 == p.p130);
        self.scalar_v2378 = v2378;
        let v2379: bool = (!v2375);
        self.scalar_v2379 = v2379;
        let v2380: bool = (v2378 && v2379);
        self.scalar_v2380 = v2380;
        let v2381: f64 = p.p131;
        self.scalar_v2381 = v2381;
        let v2384: bool = (!v2378);
        self.scalar_v2384 = v2384;
        let v2385: bool = (v2379 && v2384);
        self.scalar_v2385 = v2385;
        let v2395: f64 = p.p68;
        self.scalar_v2395 = v2395;
        let v2396: f64 = p.p77;
        self.scalar_v2396 = v2396;
        let v2434: f64 = (p.p3 * p.p68);
        self.scalar_v2434 = v2434;
        let v2438: f64 = (p.p3 * p.p77);
        self.scalar_v2438 = v2438;
        let v2460: f64 = (if v681 { 0.0 } else { 0.0 });
        self.scalar_v2460 = v2460;
        let v2465: f64 = (if v689 { 0.0 } else { 0.0 });
        self.scalar_v2465 = v2465;
        let v2470: f64 = (if v497 { 0.0 } else { 0.0 });
        self.scalar_v2470 = v2470;
        let v2471: f64 = (if v1311 { 0.0 } else { 0.0 });
        self.scalar_v2471 = v2471;
        let v2472: bool = (v675 && v683);
        self.scalar_v2472 = v2472;
        let v2473: f64 = (if v2472 { 0.0 } else { 0.0 });
        self.scalar_v2473 = v2473;
        let v2474: bool = (v675 && v689);
        self.scalar_v2474 = v2474;
        let v2475: f64 = (if v2474 { 0.0 } else { 0.0 });
        self.scalar_v2475 = v2475;
        let v2476: bool = (v681 && v683);
        self.scalar_v2476 = v2476;
        let v2477: f64 = (if v2476 { 0.0 } else { 0.0 });
        self.scalar_v2477 = v2477;
        let v2478: bool = (v681 && v689);
        self.scalar_v2478 = v2478;
        let v2479: f64 = (if v2478 { 0.0 } else { 0.0 });
        self.scalar_v2479 = v2479;
        let v2993: f64 = (-p.p3);
        self.scalar_v2993 = v2993;
        let v2994: f64 = (p.p3 + v2993);
        self.scalar_v2994 = v2994;
        let v2995: f64 = (v2993 - v2993);
        self.scalar_v2995 = v2995;
        let v2996: f64 = (p.p3 + v2994);
        self.scalar_v2996 = v2996;
        let v3983: f64 = (v1079 - 1.0);
        self.scalar_v3983 = v3983;
        let v4007: f64 = (if v1088 { p.p3 } else { 0.0 });
        self.scalar_v4007 = v4007;
        let v4008: f64 = (if v1088 { v2993 } else { 0.0 });
        self.scalar_v4008 = v4008;
        let v4025: f64 = (v1101 - 1.0);
        self.scalar_v4025 = v4025;
        let v4111: f64 = (p.p75 - 1.0);
        self.scalar_v4111 = v4111;
        let v4130: f64 = (v1124 - 1.0);
        self.scalar_v4130 = v4130;
        let v4473: f64 = (v2993 / 0.0001);
        self.scalar_v4473 = v4473;
        let v4474: f64 = (p.p3 / 0.0001);
        self.scalar_v4474 = v4474;
        let v4483: f64 = (-v4473);
        self.scalar_v4483 = v4483;
        let v4484: f64 = (-v4474);
        self.scalar_v4484 = v4484;
        let v4508: f64 = (v2993 / 0.001);
        self.scalar_v4508 = v4508;
        let v4509: f64 = (p.p3 / 0.001);
        self.scalar_v4509 = v4509;
        let v4520: f64 = (-v4508);
        self.scalar_v4520 = v4520;
        let v4521: f64 = (-v4509);
        self.scalar_v4521 = v4521;
        let v4986: f64 = (v1435 - 1.0);
        self.scalar_v4986 = v4986;
        let v5043: f64 = (v34 * v2993);
        self.scalar_v5043 = v5043;
        let v5044: f64 = (p.p3 * v34);
        self.scalar_v5044 = v5044;
        let v5107: f64 = (0.5 * v2993);
        self.scalar_v5107 = v5107;
        let v5108: f64 = (p.p3 * 0.5);
        self.scalar_v5108 = v5108;
        let v5241: f64 = (v1528 - 1.0);
        self.scalar_v5241 = v5241;
        let v5298: f64 = (p.p3 * v69);
        self.scalar_v5298 = v5298;
        let v5299: f64 = (v69 * v2993);
        self.scalar_v5299 = v5299;
        let v5621: f64 = (if v1628 { v2994 } else { 0.0 });
        self.scalar_v5621 = v5621;
        let v5622: f64 = (if v1628 { v2996 } else { 0.0 });
        self.scalar_v5622 = v5622;
        let v5624: f64 = (if v1628 { v2995 } else { 0.0 });
        self.scalar_v5624 = v5624;
        let v5625: f64 = (if v1628 { v2993 } else { 0.0 });
        self.scalar_v5625 = v5625;
        let v5828: f64 = (if v1668 { p.p3 } else { 0.0 });
        self.scalar_v5828 = v5828;
        let v5829: f64 = (if v1668 { v2994 } else { 0.0 });
        self.scalar_v5829 = v5829;
        let v5830: f64 = (if v1668 { v2993 } else { 0.0 });
        self.scalar_v5830 = v5830;
        let v5831: f64 = (-v5828);
        self.scalar_v5831 = v5831;
        let v5832: f64 = (-v5829);
        self.scalar_v5832 = v5832;
        let v5833: f64 = (-v5830);
        self.scalar_v5833 = v5833;
        let v6260: f64 = (p.p40 - 1.0);
        self.scalar_v6260 = v6260;
        let v6867: f64 = (p.p48 - 1.0);
        self.scalar_v6867 = v6867;
        let v6964: f64 = (p.p49 - 1.0);
        self.scalar_v6964 = v6964;
        let v7277: f64 = (if v497 { p.p3 } else { 0.0 });
        self.scalar_v7277 = v7277;
        let v7278: f64 = (if v497 { v2993 } else { 0.0 });
        self.scalar_v7278 = v7278;
        let v7279: f64 = (if v1311 { p.p3 } else { v7277 });
        self.scalar_v7279 = v7279;
        let v7280: f64 = (if v1311 { 0.0 } else { v7278 });
        self.scalar_v7280 = v7280;
        let v7281: f64 = (if v1311 { v2993 } else { 0.0 });
        self.scalar_v7281 = v7281;
        let v7412: f64 = (0.0 * v2993);
        self.scalar_v7412 = v7412;
        let v7413: f64 = (p.p3 * 0.0);
        self.scalar_v7413 = v7413;
        let v7499: f64 = (0.0 * v2994);
        self.scalar_v7499 = v7499;
        let v7500: f64 = (0.0 * v2995);
        self.scalar_v7500 = v7500;
        let v7990: f64 = (v2155 - 1.0);
        self.scalar_v7990 = v7990;
        let v8100: f64 = (p.p3 / p.p90);
        self.scalar_v8100 = v8100;
        let v8101: f64 = (v2994 / p.p90);
        self.scalar_v8101 = v8101;
        let v8102: f64 = (v2995 / p.p90);
        self.scalar_v8102 = v8102;
        let v8103: f64 = (v2993 / p.p90);
        self.scalar_v8103 = v8103;
        let v8416: f64 = (v2265 - 1.0);
        self.scalar_v8416 = v8416;
        let v8554: f64 = (p.p3 * 0.2);
        self.scalar_v8554 = v8554;
        let v8555: f64 = (0.2 * v2993);
        self.scalar_v8555 = v8555;
        let v8675: f64 = (v2325 - 1.0);
        self.scalar_v8675 = v8675;
        let v8681: f64 = (1.0 / v26);
        self.scalar_v8681 = v8681;
        let v8960: f64 = (p.p3 * p.p3);
        self.scalar_v8960 = v8960;
        let v8961: f64 = (p.p3 * v2993);
        self.scalar_v8961 = v8961;
        let v9072: f64 = (p.p3 * v2434);
        self.scalar_v9072 = v9072;
        let v9073: f64 = (v2434 * v2993);
        self.scalar_v9073 = v9073;
        let v9078: f64 = (v2438 * v2993);
        self.scalar_v9078 = v9078;
        let v9079: f64 = (p.p3 * v2438);
        self.scalar_v9079 = v9079;
        let v9104: f64 = (p.p3 * v2994);
        self.scalar_v9104 = v9104;
        let v9105: f64 = (p.p3 * v2995);
        self.scalar_v9105 = v9105;
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
        let v663: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v663 = v663;
        let v665: f64 = f64::powf(self.scalar_v663, self.scalar_v664);
        self.scalar_v665 = v665;
        let v666: f64 = (self.scalar_v662 * self.scalar_v665);
        self.scalar_v666 = v666;
        let v2338: f64 = (self.scalar_v20 / self.scalar_v666);
        self.scalar_v2338 = v2338;
        let v2339: f64 = (self.scalar_v27 * self.scalar_v2338);
        self.scalar_v2339 = v2339;
        let v2347: f64 = (self.scalar_v666 * self.scalar_v2325);
        self.scalar_v2347 = v2347;
        let v2348: f64 = (self.scalar_v20 / self.scalar_v2347);
        self.scalar_v2348 = v2348;
        let v2349: f64 = (self.scalar_v27 * self.scalar_v2348);
        self.scalar_v2349 = v2349;
        let v8668: f64 = (1.0 / self.scalar_v666);
        self.scalar_v8668 = v8668;
        let v8669: f64 = (self.scalar_v27 * self.scalar_v8668);
        self.scalar_v8669 = v8669;
        let v8670: f64 = (if self.scalar_v2329 { self.scalar_v8669 } else { 0.0 });
        self.scalar_v8670 = v8670;
        let v8671: f64 = (1.0 / self.scalar_v20);
        self.scalar_v8671 = v8671;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
