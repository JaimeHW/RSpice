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
    pub p147: f64,
    pub p148: f64,
    pub p149: f64,
    pub p150: f64,
    pub p151: f64,
    pub p152: f64,
    pub p153: f64,
    pub p154: f64,
    pub p155: f64,
    pub p156: f64,
    pub p157: f64,
    pub p158: f64,
    pub p159: f64,
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
            params.p146 = 300.0;
            params.p147 = 3.0000000000000004e-9;
            params.p148 = 0.0;
            params.p149 = 0.0;
            params.p150 = 2.0;
            params.p151 = 400.0;
            params.p152 = 1e-40;
            params.p153 = 1e-40;
            params.p154 = 0.001;
            validate_parameter("minr", params.p154, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p155 = 0.0;
            params.p156 = 1.0;
            params.p157 = 0.0;
            params.p158 = 0.16;
            params.p159 = 0.0;
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
    pub nodes: [usize; 13],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 160]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 11]>,
    pub(crate) ddt_state_previous: Box<[f64; 11]>,
    pub(crate) ddt_state_older: Box<[f64; 11]>,
    pub(crate) ddt_state_initialized: Box<[bool; 11]>,
    pub(crate) ddt_derivative_current: Box<[f64; 11]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 11]>,
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
    pub(crate) scalar_v30: f64,
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
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: bool,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: bool,
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
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: bool,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: bool,
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
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: bool,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v399: bool,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v430: bool,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v467: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v526: bool,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v536: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v572: f64,
    pub(crate) scalar_v573: f64,
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v634: f64,
    pub(crate) scalar_v637: f64,
    pub(crate) scalar_v640: f64,
    pub(crate) scalar_v641: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v646: f64,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v652: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v658: f64,
    pub(crate) scalar_v659: f64,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v664: f64,
    pub(crate) scalar_v668: f64,
    pub(crate) scalar_v669: f64,
    pub(crate) scalar_v673: f64,
    pub(crate) scalar_v674: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v692: f64,
    pub(crate) scalar_v694: f64,
    pub(crate) scalar_v695: f64,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v715: f64,
    pub(crate) scalar_v717: f64,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v724: bool,
    pub(crate) scalar_v730: bool,
    pub(crate) scalar_v732: bool,
    pub(crate) scalar_v738: bool,
    pub(crate) scalar_v740: bool,
    pub(crate) scalar_v746: bool,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v801: f64,
    pub(crate) scalar_v931: f64,
    pub(crate) scalar_v984: f64,
    pub(crate) scalar_v985: f64,
    pub(crate) scalar_v986: f64,
    pub(crate) scalar_v997: f64,
    pub(crate) scalar_v1018: f64,
    pub(crate) scalar_v1019: f64,
    pub(crate) scalar_v1020: f64,
    pub(crate) scalar_v1021: f64,
    pub(crate) scalar_v1022: f64,
    pub(crate) scalar_v1023: f64,
    pub(crate) scalar_v1070: f64,
    pub(crate) scalar_v1080: f64,
    pub(crate) scalar_v1093: f64,
    pub(crate) scalar_v1094: bool,
    pub(crate) scalar_v1098: bool,
    pub(crate) scalar_v1147: f64,
    pub(crate) scalar_v1148: f64,
    pub(crate) scalar_v1149: f64,
    pub(crate) scalar_v1171: f64,
    pub(crate) scalar_v1179: f64,
    pub(crate) scalar_v1180: bool,
    pub(crate) scalar_v1182: bool,
    pub(crate) scalar_v1183: bool,
    pub(crate) scalar_v1184: bool,
    pub(crate) scalar_v1187: bool,
    pub(crate) scalar_v1188: bool,
    pub(crate) scalar_v1193: f64,
    pub(crate) scalar_v1214: f64,
    pub(crate) scalar_v1216: f64,
    pub(crate) scalar_v1245: bool,
    pub(crate) scalar_v1251: bool,
    pub(crate) scalar_v1285: f64,
    pub(crate) scalar_v1307: f64,
    pub(crate) scalar_v1320: f64,
    pub(crate) scalar_v1338: f64,
    pub(crate) scalar_v1401: f64,
    pub(crate) scalar_v1402: bool,
    pub(crate) scalar_v1403: bool,
    pub(crate) scalar_v1404: bool,
    pub(crate) scalar_v1406: bool,
    pub(crate) scalar_v1407: bool,
    pub(crate) scalar_v1408: f64,
    pub(crate) scalar_v1501: bool,
    pub(crate) scalar_v1502: bool,
    pub(crate) scalar_v1503: bool,
    pub(crate) scalar_v1527: f64,
    pub(crate) scalar_v1529: f64,
    pub(crate) scalar_v1530: f64,
    pub(crate) scalar_v1532: f64,
    pub(crate) scalar_v1592: bool,
    pub(crate) scalar_v1593: bool,
    pub(crate) scalar_v1594: bool,
    pub(crate) scalar_v1620: f64,
    pub(crate) scalar_v1622: f64,
    pub(crate) scalar_v1623: f64,
    pub(crate) scalar_v1625: f64,
    pub(crate) scalar_v1702: f64,
    pub(crate) scalar_v1703: bool,
    pub(crate) scalar_v1704: f64,
    pub(crate) scalar_v1705: f64,
    pub(crate) scalar_v1711: f64,
    pub(crate) scalar_v1720: f64,
    pub(crate) scalar_v1721: f64,
    pub(crate) scalar_v1733: bool,
    pub(crate) scalar_v1752: f64,
    pub(crate) scalar_v1762: f64,
    pub(crate) scalar_v1763: bool,
    pub(crate) scalar_v1764: bool,
    pub(crate) scalar_v1765: bool,
    pub(crate) scalar_v1770: f64,
    pub(crate) scalar_v1780: bool,
    pub(crate) scalar_v1781: f64,
    pub(crate) scalar_v1782: f64,
    pub(crate) scalar_v1796: bool,
    pub(crate) scalar_v1804: bool,
    pub(crate) scalar_v1805: bool,
    pub(crate) scalar_v1818: f64,
    pub(crate) scalar_v1823: f64,
    pub(crate) scalar_v1840: bool,
    pub(crate) scalar_v1841: bool,
    pub(crate) scalar_v1847: f64,
    pub(crate) scalar_v1848: bool,
    pub(crate) scalar_v1851: f64,
    pub(crate) scalar_v1857: f64,
    pub(crate) scalar_v1868: f64,
    pub(crate) scalar_v1869: f64,
    pub(crate) scalar_v1870: f64,
    pub(crate) scalar_v1871: f64,
    pub(crate) scalar_v1872: f64,
    pub(crate) scalar_v1873: f64,
    pub(crate) scalar_v1874: f64,
    pub(crate) scalar_v1875: f64,
    pub(crate) scalar_v1876: f64,
    pub(crate) scalar_v1877: f64,
    pub(crate) scalar_v1878: f64,
    pub(crate) scalar_v1879: f64,
    pub(crate) scalar_v1880: f64,
    pub(crate) scalar_v1881: f64,
    pub(crate) scalar_v1882: f64,
    pub(crate) scalar_v1896: bool,
    pub(crate) scalar_v1923: f64,
    pub(crate) scalar_v1924: bool,
    pub(crate) scalar_v1925: f64,
    pub(crate) scalar_v1928: f64,
    pub(crate) scalar_v1947: f64,
    pub(crate) scalar_v1961: f64,
    pub(crate) scalar_v1966: bool,
    pub(crate) scalar_v1968: bool,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v1973: f64,
    pub(crate) scalar_v1974: f64,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1976: f64,
    pub(crate) scalar_v1985: f64,
    pub(crate) scalar_v1986: bool,
    pub(crate) scalar_v1989: bool,
    pub(crate) scalar_v2012: f64,
    pub(crate) scalar_v2013: f64,
    pub(crate) scalar_v2019: f64,
    pub(crate) scalar_v2020: f64,
    pub(crate) scalar_v2021: f64,
    pub(crate) scalar_v2069: bool,
    pub(crate) scalar_v2070: bool,
    pub(crate) scalar_v2075: f64,
    pub(crate) scalar_v2079: f64,
    pub(crate) scalar_v2086: f64,
    pub(crate) scalar_v2091: f64,
    pub(crate) scalar_v2111: f64,
    pub(crate) scalar_v2131: f64,
    pub(crate) scalar_v2132: bool,
    pub(crate) scalar_v2167: bool,
    pub(crate) scalar_v2173: bool,
    pub(crate) scalar_v2240: f64,
    pub(crate) scalar_v2241: f64,
    pub(crate) scalar_v2271: f64,
    pub(crate) scalar_v2309: f64,
    pub(crate) scalar_v2344: f64,
    pub(crate) scalar_v2345: f64,
    pub(crate) scalar_v2346: f64,
    pub(crate) scalar_v2365: f64,
    pub(crate) scalar_v2378: f64,
    pub(crate) scalar_v2379: f64,
    pub(crate) scalar_v2401: f64,
    pub(crate) scalar_v2402: bool,
    pub(crate) scalar_v2411: f64,
    pub(crate) scalar_v2415: bool,
    pub(crate) scalar_v2434: bool,
    pub(crate) scalar_v2435: bool,
    pub(crate) scalar_v2436: bool,
    pub(crate) scalar_v2439: bool,
    pub(crate) scalar_v2455: f64,
    pub(crate) scalar_v2466: bool,
    pub(crate) scalar_v2487: f64,
    pub(crate) scalar_v2488: bool,
    pub(crate) scalar_v2489: f64,
    pub(crate) scalar_v2527: f64,
    pub(crate) scalar_v2528: f64,
    pub(crate) scalar_v2534: f64,
    pub(crate) scalar_v2538: f64,
    pub(crate) scalar_v2541: bool,
    pub(crate) scalar_v2545: f64,
    pub(crate) scalar_v2549: f64,
    pub(crate) scalar_v2550: bool,
    pub(crate) scalar_v2551: f64,
    pub(crate) scalar_v2552: bool,
    pub(crate) scalar_v2553: bool,
    pub(crate) scalar_v2557: f64,
    pub(crate) scalar_v2558: bool,
    pub(crate) scalar_v2559: bool,
    pub(crate) scalar_v2560: bool,
    pub(crate) scalar_v2561: bool,
    pub(crate) scalar_v2569: bool,
    pub(crate) scalar_v2570: bool,
    pub(crate) scalar_v2578: bool,
    pub(crate) scalar_v2583: f64,
    pub(crate) scalar_v2584: bool,
    pub(crate) scalar_v2588: bool,
    pub(crate) scalar_v2598: f64,
    pub(crate) scalar_v2599: bool,
    pub(crate) scalar_v2602: bool,
    pub(crate) scalar_v2603: bool,
    pub(crate) scalar_v2604: bool,
    pub(crate) scalar_v2605: f64,
    pub(crate) scalar_v2608: bool,
    pub(crate) scalar_v2609: bool,
    pub(crate) scalar_v2619: f64,
    pub(crate) scalar_v2620: f64,
    pub(crate) scalar_v2669: f64,
    pub(crate) scalar_v2673: f64,
    pub(crate) scalar_v2917: f64,
    pub(crate) scalar_v3268: f64,
    pub(crate) scalar_v3269: f64,
    pub(crate) scalar_v3270: f64,
    pub(crate) scalar_v3271: f64,
    pub(crate) scalar_v4305: f64,
    pub(crate) scalar_v4329: f64,
    pub(crate) scalar_v4330: f64,
    pub(crate) scalar_v4347: f64,
    pub(crate) scalar_v4433: f64,
    pub(crate) scalar_v4452: f64,
    pub(crate) scalar_v4795: f64,
    pub(crate) scalar_v4796: f64,
    pub(crate) scalar_v4805: f64,
    pub(crate) scalar_v4806: f64,
    pub(crate) scalar_v4830: f64,
    pub(crate) scalar_v4831: f64,
    pub(crate) scalar_v4842: f64,
    pub(crate) scalar_v4843: f64,
    pub(crate) scalar_v5308: f64,
    pub(crate) scalar_v5365: f64,
    pub(crate) scalar_v5366: f64,
    pub(crate) scalar_v5429: f64,
    pub(crate) scalar_v5430: f64,
    pub(crate) scalar_v5563: f64,
    pub(crate) scalar_v5620: f64,
    pub(crate) scalar_v5621: f64,
    pub(crate) scalar_v6109: f64,
    pub(crate) scalar_v6110: f64,
    pub(crate) scalar_v6321: f64,
    pub(crate) scalar_v6322: f64,
    pub(crate) scalar_v6324: f64,
    pub(crate) scalar_v6325: f64,
    pub(crate) scalar_v6579: f64,
    pub(crate) scalar_v6580: f64,
    pub(crate) scalar_v6581: f64,
    pub(crate) scalar_v6582: f64,
    pub(crate) scalar_v6583: f64,
    pub(crate) scalar_v6584: f64,
    pub(crate) scalar_v7012: f64,
    pub(crate) scalar_v7619: f64,
    pub(crate) scalar_v7716: f64,
    pub(crate) scalar_v8029: f64,
    pub(crate) scalar_v8030: f64,
    pub(crate) scalar_v8031: f64,
    pub(crate) scalar_v8032: f64,
    pub(crate) scalar_v8033: f64,
    pub(crate) scalar_v8249: f64,
    pub(crate) scalar_v8250: f64,
    pub(crate) scalar_v8318: f64,
    pub(crate) scalar_v8874: f64,
    pub(crate) scalar_v8909: f64,
    pub(crate) scalar_v9019: f64,
    pub(crate) scalar_v9020: f64,
    pub(crate) scalar_v9021: f64,
    pub(crate) scalar_v9022: f64,
    pub(crate) scalar_v9337: f64,
    pub(crate) scalar_v9475: f64,
    pub(crate) scalar_v9476: f64,
    pub(crate) scalar_v9596: f64,
    pub(crate) scalar_v9602: f64,
    pub(crate) scalar_v9931: f64,
    pub(crate) scalar_v9932: f64,
    pub(crate) scalar_v10054: f64,
    pub(crate) scalar_v10055: f64,
    pub(crate) scalar_v10060: f64,
    pub(crate) scalar_v10061: f64,
    pub(crate) scalar_v10088: f64,
    pub(crate) scalar_v10089: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v720: f64,
    pub(crate) scalar_v722: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v2562: f64,
    pub(crate) scalar_v2563: f64,
    pub(crate) scalar_v2571: f64,
    pub(crate) scalar_v2572: f64,
    pub(crate) scalar_v2573: f64,
    pub(crate) scalar_v9589: f64,
    pub(crate) scalar_v9590: f64,
    pub(crate) scalar_v9591: f64,
    pub(crate) scalar_v9592: f64,
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
            scalar_v30: self.scalar_v30,
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
            scalar_v46: self.scalar_v46,
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
            scalar_v101: self.scalar_v101,
            scalar_v108: self.scalar_v108,
            scalar_v121: self.scalar_v121,
            scalar_v123: self.scalar_v123,
            scalar_v178: self.scalar_v178,
            scalar_v198: self.scalar_v198,
            scalar_v201: self.scalar_v201,
            scalar_v221: self.scalar_v221,
            scalar_v262: self.scalar_v262,
            scalar_v265: self.scalar_v265,
            scalar_v285: self.scalar_v285,
            scalar_v288: self.scalar_v288,
            scalar_v314: self.scalar_v314,
            scalar_v316: self.scalar_v316,
            scalar_v318: self.scalar_v318,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v328: self.scalar_v328,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v362: self.scalar_v362,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v371: self.scalar_v371,
            scalar_v399: self.scalar_v399,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v430: self.scalar_v430,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v451: self.scalar_v451,
            scalar_v453: self.scalar_v453,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v461: self.scalar_v461,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v477: self.scalar_v477,
            scalar_v479: self.scalar_v479,
            scalar_v480: self.scalar_v480,
            scalar_v481: self.scalar_v481,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v498: self.scalar_v498,
            scalar_v503: self.scalar_v503,
            scalar_v504: self.scalar_v504,
            scalar_v505: self.scalar_v505,
            scalar_v507: self.scalar_v507,
            scalar_v511: self.scalar_v511,
            scalar_v512: self.scalar_v512,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v525: self.scalar_v525,
            scalar_v526: self.scalar_v526,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v535: self.scalar_v535,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v544: self.scalar_v544,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v556: self.scalar_v556,
            scalar_v557: self.scalar_v557,
            scalar_v561: self.scalar_v561,
            scalar_v562: self.scalar_v562,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v571: self.scalar_v571,
            scalar_v572: self.scalar_v572,
            scalar_v573: self.scalar_v573,
            scalar_v580: self.scalar_v580,
            scalar_v583: self.scalar_v583,
            scalar_v591: self.scalar_v591,
            scalar_v600: self.scalar_v600,
            scalar_v613: self.scalar_v613,
            scalar_v622: self.scalar_v622,
            scalar_v634: self.scalar_v634,
            scalar_v637: self.scalar_v637,
            scalar_v640: self.scalar_v640,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v646: self.scalar_v646,
            scalar_v651: self.scalar_v651,
            scalar_v652: self.scalar_v652,
            scalar_v653: self.scalar_v653,
            scalar_v658: self.scalar_v658,
            scalar_v659: self.scalar_v659,
            scalar_v663: self.scalar_v663,
            scalar_v664: self.scalar_v664,
            scalar_v668: self.scalar_v668,
            scalar_v669: self.scalar_v669,
            scalar_v673: self.scalar_v673,
            scalar_v674: self.scalar_v674,
            scalar_v678: self.scalar_v678,
            scalar_v679: self.scalar_v679,
            scalar_v680: self.scalar_v680,
            scalar_v684: self.scalar_v684,
            scalar_v685: self.scalar_v685,
            scalar_v689: self.scalar_v689,
            scalar_v692: self.scalar_v692,
            scalar_v694: self.scalar_v694,
            scalar_v695: self.scalar_v695,
            scalar_v696: self.scalar_v696,
            scalar_v715: self.scalar_v715,
            scalar_v717: self.scalar_v717,
            scalar_v719: self.scalar_v719,
            scalar_v721: self.scalar_v721,
            scalar_v724: self.scalar_v724,
            scalar_v730: self.scalar_v730,
            scalar_v732: self.scalar_v732,
            scalar_v738: self.scalar_v738,
            scalar_v740: self.scalar_v740,
            scalar_v746: self.scalar_v746,
            scalar_v796: self.scalar_v796,
            scalar_v801: self.scalar_v801,
            scalar_v931: self.scalar_v931,
            scalar_v984: self.scalar_v984,
            scalar_v985: self.scalar_v985,
            scalar_v986: self.scalar_v986,
            scalar_v997: self.scalar_v997,
            scalar_v1018: self.scalar_v1018,
            scalar_v1019: self.scalar_v1019,
            scalar_v1020: self.scalar_v1020,
            scalar_v1021: self.scalar_v1021,
            scalar_v1022: self.scalar_v1022,
            scalar_v1023: self.scalar_v1023,
            scalar_v1070: self.scalar_v1070,
            scalar_v1080: self.scalar_v1080,
            scalar_v1093: self.scalar_v1093,
            scalar_v1094: self.scalar_v1094,
            scalar_v1098: self.scalar_v1098,
            scalar_v1147: self.scalar_v1147,
            scalar_v1148: self.scalar_v1148,
            scalar_v1149: self.scalar_v1149,
            scalar_v1171: self.scalar_v1171,
            scalar_v1179: self.scalar_v1179,
            scalar_v1180: self.scalar_v1180,
            scalar_v1182: self.scalar_v1182,
            scalar_v1183: self.scalar_v1183,
            scalar_v1184: self.scalar_v1184,
            scalar_v1187: self.scalar_v1187,
            scalar_v1188: self.scalar_v1188,
            scalar_v1193: self.scalar_v1193,
            scalar_v1214: self.scalar_v1214,
            scalar_v1216: self.scalar_v1216,
            scalar_v1245: self.scalar_v1245,
            scalar_v1251: self.scalar_v1251,
            scalar_v1285: self.scalar_v1285,
            scalar_v1307: self.scalar_v1307,
            scalar_v1320: self.scalar_v1320,
            scalar_v1338: self.scalar_v1338,
            scalar_v1401: self.scalar_v1401,
            scalar_v1402: self.scalar_v1402,
            scalar_v1403: self.scalar_v1403,
            scalar_v1404: self.scalar_v1404,
            scalar_v1406: self.scalar_v1406,
            scalar_v1407: self.scalar_v1407,
            scalar_v1408: self.scalar_v1408,
            scalar_v1501: self.scalar_v1501,
            scalar_v1502: self.scalar_v1502,
            scalar_v1503: self.scalar_v1503,
            scalar_v1527: self.scalar_v1527,
            scalar_v1529: self.scalar_v1529,
            scalar_v1530: self.scalar_v1530,
            scalar_v1532: self.scalar_v1532,
            scalar_v1592: self.scalar_v1592,
            scalar_v1593: self.scalar_v1593,
            scalar_v1594: self.scalar_v1594,
            scalar_v1620: self.scalar_v1620,
            scalar_v1622: self.scalar_v1622,
            scalar_v1623: self.scalar_v1623,
            scalar_v1625: self.scalar_v1625,
            scalar_v1702: self.scalar_v1702,
            scalar_v1703: self.scalar_v1703,
            scalar_v1704: self.scalar_v1704,
            scalar_v1705: self.scalar_v1705,
            scalar_v1711: self.scalar_v1711,
            scalar_v1720: self.scalar_v1720,
            scalar_v1721: self.scalar_v1721,
            scalar_v1733: self.scalar_v1733,
            scalar_v1752: self.scalar_v1752,
            scalar_v1762: self.scalar_v1762,
            scalar_v1763: self.scalar_v1763,
            scalar_v1764: self.scalar_v1764,
            scalar_v1765: self.scalar_v1765,
            scalar_v1770: self.scalar_v1770,
            scalar_v1780: self.scalar_v1780,
            scalar_v1781: self.scalar_v1781,
            scalar_v1782: self.scalar_v1782,
            scalar_v1796: self.scalar_v1796,
            scalar_v1804: self.scalar_v1804,
            scalar_v1805: self.scalar_v1805,
            scalar_v1818: self.scalar_v1818,
            scalar_v1823: self.scalar_v1823,
            scalar_v1840: self.scalar_v1840,
            scalar_v1841: self.scalar_v1841,
            scalar_v1847: self.scalar_v1847,
            scalar_v1848: self.scalar_v1848,
            scalar_v1851: self.scalar_v1851,
            scalar_v1857: self.scalar_v1857,
            scalar_v1868: self.scalar_v1868,
            scalar_v1869: self.scalar_v1869,
            scalar_v1870: self.scalar_v1870,
            scalar_v1871: self.scalar_v1871,
            scalar_v1872: self.scalar_v1872,
            scalar_v1873: self.scalar_v1873,
            scalar_v1874: self.scalar_v1874,
            scalar_v1875: self.scalar_v1875,
            scalar_v1876: self.scalar_v1876,
            scalar_v1877: self.scalar_v1877,
            scalar_v1878: self.scalar_v1878,
            scalar_v1879: self.scalar_v1879,
            scalar_v1880: self.scalar_v1880,
            scalar_v1881: self.scalar_v1881,
            scalar_v1882: self.scalar_v1882,
            scalar_v1896: self.scalar_v1896,
            scalar_v1923: self.scalar_v1923,
            scalar_v1924: self.scalar_v1924,
            scalar_v1925: self.scalar_v1925,
            scalar_v1928: self.scalar_v1928,
            scalar_v1947: self.scalar_v1947,
            scalar_v1961: self.scalar_v1961,
            scalar_v1966: self.scalar_v1966,
            scalar_v1968: self.scalar_v1968,
            scalar_v1972: self.scalar_v1972,
            scalar_v1973: self.scalar_v1973,
            scalar_v1974: self.scalar_v1974,
            scalar_v1975: self.scalar_v1975,
            scalar_v1976: self.scalar_v1976,
            scalar_v1985: self.scalar_v1985,
            scalar_v1986: self.scalar_v1986,
            scalar_v1989: self.scalar_v1989,
            scalar_v2012: self.scalar_v2012,
            scalar_v2013: self.scalar_v2013,
            scalar_v2019: self.scalar_v2019,
            scalar_v2020: self.scalar_v2020,
            scalar_v2021: self.scalar_v2021,
            scalar_v2069: self.scalar_v2069,
            scalar_v2070: self.scalar_v2070,
            scalar_v2075: self.scalar_v2075,
            scalar_v2079: self.scalar_v2079,
            scalar_v2086: self.scalar_v2086,
            scalar_v2091: self.scalar_v2091,
            scalar_v2111: self.scalar_v2111,
            scalar_v2131: self.scalar_v2131,
            scalar_v2132: self.scalar_v2132,
            scalar_v2167: self.scalar_v2167,
            scalar_v2173: self.scalar_v2173,
            scalar_v2240: self.scalar_v2240,
            scalar_v2241: self.scalar_v2241,
            scalar_v2271: self.scalar_v2271,
            scalar_v2309: self.scalar_v2309,
            scalar_v2344: self.scalar_v2344,
            scalar_v2345: self.scalar_v2345,
            scalar_v2346: self.scalar_v2346,
            scalar_v2365: self.scalar_v2365,
            scalar_v2378: self.scalar_v2378,
            scalar_v2379: self.scalar_v2379,
            scalar_v2401: self.scalar_v2401,
            scalar_v2402: self.scalar_v2402,
            scalar_v2411: self.scalar_v2411,
            scalar_v2415: self.scalar_v2415,
            scalar_v2434: self.scalar_v2434,
            scalar_v2435: self.scalar_v2435,
            scalar_v2436: self.scalar_v2436,
            scalar_v2439: self.scalar_v2439,
            scalar_v2455: self.scalar_v2455,
            scalar_v2466: self.scalar_v2466,
            scalar_v2487: self.scalar_v2487,
            scalar_v2488: self.scalar_v2488,
            scalar_v2489: self.scalar_v2489,
            scalar_v2527: self.scalar_v2527,
            scalar_v2528: self.scalar_v2528,
            scalar_v2534: self.scalar_v2534,
            scalar_v2538: self.scalar_v2538,
            scalar_v2541: self.scalar_v2541,
            scalar_v2545: self.scalar_v2545,
            scalar_v2549: self.scalar_v2549,
            scalar_v2550: self.scalar_v2550,
            scalar_v2551: self.scalar_v2551,
            scalar_v2552: self.scalar_v2552,
            scalar_v2553: self.scalar_v2553,
            scalar_v2557: self.scalar_v2557,
            scalar_v2558: self.scalar_v2558,
            scalar_v2559: self.scalar_v2559,
            scalar_v2560: self.scalar_v2560,
            scalar_v2561: self.scalar_v2561,
            scalar_v2569: self.scalar_v2569,
            scalar_v2570: self.scalar_v2570,
            scalar_v2578: self.scalar_v2578,
            scalar_v2583: self.scalar_v2583,
            scalar_v2584: self.scalar_v2584,
            scalar_v2588: self.scalar_v2588,
            scalar_v2598: self.scalar_v2598,
            scalar_v2599: self.scalar_v2599,
            scalar_v2602: self.scalar_v2602,
            scalar_v2603: self.scalar_v2603,
            scalar_v2604: self.scalar_v2604,
            scalar_v2605: self.scalar_v2605,
            scalar_v2608: self.scalar_v2608,
            scalar_v2609: self.scalar_v2609,
            scalar_v2619: self.scalar_v2619,
            scalar_v2620: self.scalar_v2620,
            scalar_v2669: self.scalar_v2669,
            scalar_v2673: self.scalar_v2673,
            scalar_v2917: self.scalar_v2917,
            scalar_v3268: self.scalar_v3268,
            scalar_v3269: self.scalar_v3269,
            scalar_v3270: self.scalar_v3270,
            scalar_v3271: self.scalar_v3271,
            scalar_v4305: self.scalar_v4305,
            scalar_v4329: self.scalar_v4329,
            scalar_v4330: self.scalar_v4330,
            scalar_v4347: self.scalar_v4347,
            scalar_v4433: self.scalar_v4433,
            scalar_v4452: self.scalar_v4452,
            scalar_v4795: self.scalar_v4795,
            scalar_v4796: self.scalar_v4796,
            scalar_v4805: self.scalar_v4805,
            scalar_v4806: self.scalar_v4806,
            scalar_v4830: self.scalar_v4830,
            scalar_v4831: self.scalar_v4831,
            scalar_v4842: self.scalar_v4842,
            scalar_v4843: self.scalar_v4843,
            scalar_v5308: self.scalar_v5308,
            scalar_v5365: self.scalar_v5365,
            scalar_v5366: self.scalar_v5366,
            scalar_v5429: self.scalar_v5429,
            scalar_v5430: self.scalar_v5430,
            scalar_v5563: self.scalar_v5563,
            scalar_v5620: self.scalar_v5620,
            scalar_v5621: self.scalar_v5621,
            scalar_v6109: self.scalar_v6109,
            scalar_v6110: self.scalar_v6110,
            scalar_v6321: self.scalar_v6321,
            scalar_v6322: self.scalar_v6322,
            scalar_v6324: self.scalar_v6324,
            scalar_v6325: self.scalar_v6325,
            scalar_v6579: self.scalar_v6579,
            scalar_v6580: self.scalar_v6580,
            scalar_v6581: self.scalar_v6581,
            scalar_v6582: self.scalar_v6582,
            scalar_v6583: self.scalar_v6583,
            scalar_v6584: self.scalar_v6584,
            scalar_v7012: self.scalar_v7012,
            scalar_v7619: self.scalar_v7619,
            scalar_v7716: self.scalar_v7716,
            scalar_v8029: self.scalar_v8029,
            scalar_v8030: self.scalar_v8030,
            scalar_v8031: self.scalar_v8031,
            scalar_v8032: self.scalar_v8032,
            scalar_v8033: self.scalar_v8033,
            scalar_v8249: self.scalar_v8249,
            scalar_v8250: self.scalar_v8250,
            scalar_v8318: self.scalar_v8318,
            scalar_v8874: self.scalar_v8874,
            scalar_v8909: self.scalar_v8909,
            scalar_v9019: self.scalar_v9019,
            scalar_v9020: self.scalar_v9020,
            scalar_v9021: self.scalar_v9021,
            scalar_v9022: self.scalar_v9022,
            scalar_v9337: self.scalar_v9337,
            scalar_v9475: self.scalar_v9475,
            scalar_v9476: self.scalar_v9476,
            scalar_v9596: self.scalar_v9596,
            scalar_v9602: self.scalar_v9602,
            scalar_v9931: self.scalar_v9931,
            scalar_v9932: self.scalar_v9932,
            scalar_v10054: self.scalar_v10054,
            scalar_v10055: self.scalar_v10055,
            scalar_v10060: self.scalar_v10060,
            scalar_v10061: self.scalar_v10061,
            scalar_v10088: self.scalar_v10088,
            scalar_v10089: self.scalar_v10089,
            scalar_v20: self.scalar_v20,
            scalar_v720: self.scalar_v720,
            scalar_v722: self.scalar_v722,
            scalar_v723: self.scalar_v723,
            scalar_v2562: self.scalar_v2562,
            scalar_v2563: self.scalar_v2563,
            scalar_v2571: self.scalar_v2571,
            scalar_v2572: self.scalar_v2572,
            scalar_v2573: self.scalar_v2573,
            scalar_v9589: self.scalar_v9589,
            scalar_v9590: self.scalar_v9590,
            scalar_v9591: self.scalar_v9591,
            scalar_v9592: self.scalar_v9592,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 160;
    pub const VARIABLE_COUNT: usize = 630;
    pub const DDT_STATE_COUNT: usize = 11;
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
            scalar_v30: 0.0,
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
            scalar_v44: 0.0,
            scalar_v46: 0.0,
            scalar_v48: 0.0,
            scalar_v49: false,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: false,
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
            scalar_v81: 0.0,
            scalar_v82: false,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: false,
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
            scalar_v101: 0.0,
            scalar_v108: 0.0,
            scalar_v121: 0.0,
            scalar_v123: 0.0,
            scalar_v178: 0.0,
            scalar_v198: 0.0,
            scalar_v201: 0.0,
            scalar_v221: 0.0,
            scalar_v262: 0.0,
            scalar_v265: 0.0,
            scalar_v285: 0.0,
            scalar_v288: 0.0,
            scalar_v314: 0.0,
            scalar_v316: 0.0,
            scalar_v318: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v328: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v345: 0.0,
            scalar_v346: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v357: 0.0,
            scalar_v358: 0.0,
            scalar_v362: 0.0,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v369: 0.0,
            scalar_v370: false,
            scalar_v371: 0.0,
            scalar_v399: false,
            scalar_v401: 0.0,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v430: false,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v451: 0.0,
            scalar_v453: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v456: 0.0,
            scalar_v461: 0.0,
            scalar_v466: 0.0,
            scalar_v467: 0.0,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v473: 0.0,
            scalar_v477: 0.0,
            scalar_v479: 0.0,
            scalar_v480: 0.0,
            scalar_v481: 0.0,
            scalar_v485: 0.0,
            scalar_v486: 0.0,
            scalar_v491: 0.0,
            scalar_v492: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v498: 0.0,
            scalar_v503: 0.0,
            scalar_v504: 0.0,
            scalar_v505: 0.0,
            scalar_v507: 0.0,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v525: 0.0,
            scalar_v526: false,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v535: 0.0,
            scalar_v536: 0.0,
            scalar_v537: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v544: 0.0,
            scalar_v550: 0.0,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v556: 0.0,
            scalar_v557: 0.0,
            scalar_v561: 0.0,
            scalar_v562: 0.0,
            scalar_v563: 0.0,
            scalar_v564: 0.0,
            scalar_v571: 0.0,
            scalar_v572: 0.0,
            scalar_v573: 0.0,
            scalar_v580: 0.0,
            scalar_v583: 0.0,
            scalar_v591: 0.0,
            scalar_v600: 0.0,
            scalar_v613: 0.0,
            scalar_v622: 0.0,
            scalar_v634: 0.0,
            scalar_v637: 0.0,
            scalar_v640: 0.0,
            scalar_v641: 0.0,
            scalar_v642: 0.0,
            scalar_v646: 0.0,
            scalar_v651: 0.0,
            scalar_v652: 0.0,
            scalar_v653: 0.0,
            scalar_v658: 0.0,
            scalar_v659: 0.0,
            scalar_v663: 0.0,
            scalar_v664: 0.0,
            scalar_v668: 0.0,
            scalar_v669: 0.0,
            scalar_v673: 0.0,
            scalar_v674: 0.0,
            scalar_v678: 0.0,
            scalar_v679: 0.0,
            scalar_v680: 0.0,
            scalar_v684: 0.0,
            scalar_v685: 0.0,
            scalar_v689: 0.0,
            scalar_v692: 0.0,
            scalar_v694: 0.0,
            scalar_v695: 0.0,
            scalar_v696: 0.0,
            scalar_v715: 0.0,
            scalar_v717: 0.0,
            scalar_v719: 0.0,
            scalar_v721: 0.0,
            scalar_v724: false,
            scalar_v730: false,
            scalar_v732: false,
            scalar_v738: false,
            scalar_v740: false,
            scalar_v746: false,
            scalar_v796: 0.0,
            scalar_v801: 0.0,
            scalar_v931: 0.0,
            scalar_v984: 0.0,
            scalar_v985: 0.0,
            scalar_v986: 0.0,
            scalar_v997: 0.0,
            scalar_v1018: 0.0,
            scalar_v1019: 0.0,
            scalar_v1020: 0.0,
            scalar_v1021: 0.0,
            scalar_v1022: 0.0,
            scalar_v1023: 0.0,
            scalar_v1070: 0.0,
            scalar_v1080: 0.0,
            scalar_v1093: 0.0,
            scalar_v1094: false,
            scalar_v1098: false,
            scalar_v1147: 0.0,
            scalar_v1148: 0.0,
            scalar_v1149: 0.0,
            scalar_v1171: 0.0,
            scalar_v1179: 0.0,
            scalar_v1180: false,
            scalar_v1182: false,
            scalar_v1183: false,
            scalar_v1184: false,
            scalar_v1187: false,
            scalar_v1188: false,
            scalar_v1193: 0.0,
            scalar_v1214: 0.0,
            scalar_v1216: 0.0,
            scalar_v1245: false,
            scalar_v1251: false,
            scalar_v1285: 0.0,
            scalar_v1307: 0.0,
            scalar_v1320: 0.0,
            scalar_v1338: 0.0,
            scalar_v1401: 0.0,
            scalar_v1402: false,
            scalar_v1403: false,
            scalar_v1404: false,
            scalar_v1406: false,
            scalar_v1407: false,
            scalar_v1408: 0.0,
            scalar_v1501: false,
            scalar_v1502: false,
            scalar_v1503: false,
            scalar_v1527: 0.0,
            scalar_v1529: 0.0,
            scalar_v1530: 0.0,
            scalar_v1532: 0.0,
            scalar_v1592: false,
            scalar_v1593: false,
            scalar_v1594: false,
            scalar_v1620: 0.0,
            scalar_v1622: 0.0,
            scalar_v1623: 0.0,
            scalar_v1625: 0.0,
            scalar_v1702: 0.0,
            scalar_v1703: false,
            scalar_v1704: 0.0,
            scalar_v1705: 0.0,
            scalar_v1711: 0.0,
            scalar_v1720: 0.0,
            scalar_v1721: 0.0,
            scalar_v1733: false,
            scalar_v1752: 0.0,
            scalar_v1762: 0.0,
            scalar_v1763: false,
            scalar_v1764: false,
            scalar_v1765: false,
            scalar_v1770: 0.0,
            scalar_v1780: false,
            scalar_v1781: 0.0,
            scalar_v1782: 0.0,
            scalar_v1796: false,
            scalar_v1804: false,
            scalar_v1805: false,
            scalar_v1818: 0.0,
            scalar_v1823: 0.0,
            scalar_v1840: false,
            scalar_v1841: false,
            scalar_v1847: 0.0,
            scalar_v1848: false,
            scalar_v1851: 0.0,
            scalar_v1857: 0.0,
            scalar_v1868: 0.0,
            scalar_v1869: 0.0,
            scalar_v1870: 0.0,
            scalar_v1871: 0.0,
            scalar_v1872: 0.0,
            scalar_v1873: 0.0,
            scalar_v1874: 0.0,
            scalar_v1875: 0.0,
            scalar_v1876: 0.0,
            scalar_v1877: 0.0,
            scalar_v1878: 0.0,
            scalar_v1879: 0.0,
            scalar_v1880: 0.0,
            scalar_v1881: 0.0,
            scalar_v1882: 0.0,
            scalar_v1896: false,
            scalar_v1923: 0.0,
            scalar_v1924: false,
            scalar_v1925: 0.0,
            scalar_v1928: 0.0,
            scalar_v1947: 0.0,
            scalar_v1961: 0.0,
            scalar_v1966: false,
            scalar_v1968: false,
            scalar_v1972: 0.0,
            scalar_v1973: 0.0,
            scalar_v1974: 0.0,
            scalar_v1975: 0.0,
            scalar_v1976: 0.0,
            scalar_v1985: 0.0,
            scalar_v1986: false,
            scalar_v1989: false,
            scalar_v2012: 0.0,
            scalar_v2013: 0.0,
            scalar_v2019: 0.0,
            scalar_v2020: 0.0,
            scalar_v2021: 0.0,
            scalar_v2069: false,
            scalar_v2070: false,
            scalar_v2075: 0.0,
            scalar_v2079: 0.0,
            scalar_v2086: 0.0,
            scalar_v2091: 0.0,
            scalar_v2111: 0.0,
            scalar_v2131: 0.0,
            scalar_v2132: false,
            scalar_v2167: false,
            scalar_v2173: false,
            scalar_v2240: 0.0,
            scalar_v2241: 0.0,
            scalar_v2271: 0.0,
            scalar_v2309: 0.0,
            scalar_v2344: 0.0,
            scalar_v2345: 0.0,
            scalar_v2346: 0.0,
            scalar_v2365: 0.0,
            scalar_v2378: 0.0,
            scalar_v2379: 0.0,
            scalar_v2401: 0.0,
            scalar_v2402: false,
            scalar_v2411: 0.0,
            scalar_v2415: false,
            scalar_v2434: false,
            scalar_v2435: false,
            scalar_v2436: false,
            scalar_v2439: false,
            scalar_v2455: 0.0,
            scalar_v2466: false,
            scalar_v2487: 0.0,
            scalar_v2488: false,
            scalar_v2489: 0.0,
            scalar_v2527: 0.0,
            scalar_v2528: 0.0,
            scalar_v2534: 0.0,
            scalar_v2538: 0.0,
            scalar_v2541: false,
            scalar_v2545: 0.0,
            scalar_v2549: 0.0,
            scalar_v2550: false,
            scalar_v2551: 0.0,
            scalar_v2552: false,
            scalar_v2553: false,
            scalar_v2557: 0.0,
            scalar_v2558: false,
            scalar_v2559: false,
            scalar_v2560: false,
            scalar_v2561: false,
            scalar_v2569: false,
            scalar_v2570: false,
            scalar_v2578: false,
            scalar_v2583: 0.0,
            scalar_v2584: false,
            scalar_v2588: false,
            scalar_v2598: 0.0,
            scalar_v2599: false,
            scalar_v2602: false,
            scalar_v2603: false,
            scalar_v2604: false,
            scalar_v2605: 0.0,
            scalar_v2608: false,
            scalar_v2609: false,
            scalar_v2619: 0.0,
            scalar_v2620: 0.0,
            scalar_v2669: 0.0,
            scalar_v2673: 0.0,
            scalar_v2917: 0.0,
            scalar_v3268: 0.0,
            scalar_v3269: 0.0,
            scalar_v3270: 0.0,
            scalar_v3271: 0.0,
            scalar_v4305: 0.0,
            scalar_v4329: 0.0,
            scalar_v4330: 0.0,
            scalar_v4347: 0.0,
            scalar_v4433: 0.0,
            scalar_v4452: 0.0,
            scalar_v4795: 0.0,
            scalar_v4796: 0.0,
            scalar_v4805: 0.0,
            scalar_v4806: 0.0,
            scalar_v4830: 0.0,
            scalar_v4831: 0.0,
            scalar_v4842: 0.0,
            scalar_v4843: 0.0,
            scalar_v5308: 0.0,
            scalar_v5365: 0.0,
            scalar_v5366: 0.0,
            scalar_v5429: 0.0,
            scalar_v5430: 0.0,
            scalar_v5563: 0.0,
            scalar_v5620: 0.0,
            scalar_v5621: 0.0,
            scalar_v6109: 0.0,
            scalar_v6110: 0.0,
            scalar_v6321: 0.0,
            scalar_v6322: 0.0,
            scalar_v6324: 0.0,
            scalar_v6325: 0.0,
            scalar_v6579: 0.0,
            scalar_v6580: 0.0,
            scalar_v6581: 0.0,
            scalar_v6582: 0.0,
            scalar_v6583: 0.0,
            scalar_v6584: 0.0,
            scalar_v7012: 0.0,
            scalar_v7619: 0.0,
            scalar_v7716: 0.0,
            scalar_v8029: 0.0,
            scalar_v8030: 0.0,
            scalar_v8031: 0.0,
            scalar_v8032: 0.0,
            scalar_v8033: 0.0,
            scalar_v8249: 0.0,
            scalar_v8250: 0.0,
            scalar_v8318: 0.0,
            scalar_v8874: 0.0,
            scalar_v8909: 0.0,
            scalar_v9019: 0.0,
            scalar_v9020: 0.0,
            scalar_v9021: 0.0,
            scalar_v9022: 0.0,
            scalar_v9337: 0.0,
            scalar_v9475: 0.0,
            scalar_v9476: 0.0,
            scalar_v9596: 0.0,
            scalar_v9602: 0.0,
            scalar_v9931: 0.0,
            scalar_v9932: 0.0,
            scalar_v10054: 0.0,
            scalar_v10055: 0.0,
            scalar_v10060: 0.0,
            scalar_v10061: 0.0,
            scalar_v10088: 0.0,
            scalar_v10089: 0.0,
            scalar_v20: 0.0,
            scalar_v720: 0.0,
            scalar_v722: 0.0,
            scalar_v723: 0.0,
            scalar_v2562: 0.0,
            scalar_v2563: 0.0,
            scalar_v2571: 0.0,
            scalar_v2572: 0.0,
            scalar_v2573: 0.0,
            scalar_v9589: 0.0,
            scalar_v9590: 0.0,
            scalar_v9591: 0.0,
            scalar_v9592: 0.0,
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
            scalar_v30,
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
            scalar_v46,
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
            scalar_v101,
            scalar_v108,
            scalar_v121,
            scalar_v123,
            scalar_v178,
            scalar_v198,
            scalar_v201,
            scalar_v221,
            scalar_v262,
            scalar_v265,
            scalar_v285,
            scalar_v288,
            scalar_v314,
            scalar_v316,
            scalar_v318,
            scalar_v321,
            scalar_v322,
            scalar_v328,
            scalar_v331,
            scalar_v332,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v345,
            scalar_v346,
            scalar_v352,
            scalar_v353,
            scalar_v357,
            scalar_v358,
            scalar_v362,
            scalar_v364,
            scalar_v365,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v399,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v430,
            scalar_v432,
            scalar_v433,
            scalar_v451,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v461,
            scalar_v466,
            scalar_v467,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v477,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v485,
            scalar_v486,
            scalar_v491,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v498,
            scalar_v503,
            scalar_v504,
            scalar_v505,
            scalar_v507,
            scalar_v511,
            scalar_v512,
            scalar_v517,
            scalar_v518,
            scalar_v525,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v556,
            scalar_v557,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v580,
            scalar_v583,
            scalar_v591,
            scalar_v600,
            scalar_v613,
            scalar_v622,
            scalar_v634,
            scalar_v637,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v646,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v658,
            scalar_v659,
            scalar_v663,
            scalar_v664,
            scalar_v668,
            scalar_v669,
            scalar_v673,
            scalar_v674,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v684,
            scalar_v685,
            scalar_v689,
            scalar_v692,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v715,
            scalar_v717,
            scalar_v719,
            scalar_v721,
            scalar_v724,
            scalar_v730,
            scalar_v732,
            scalar_v738,
            scalar_v740,
            scalar_v746,
            scalar_v796,
            scalar_v801,
            scalar_v931,
            scalar_v984,
            scalar_v985,
            scalar_v986,
            scalar_v997,
            scalar_v1018,
            scalar_v1019,
            scalar_v1020,
            scalar_v1021,
            scalar_v1022,
            scalar_v1023,
            scalar_v1070,
            scalar_v1080,
            scalar_v1093,
            scalar_v1094,
            scalar_v1098,
            scalar_v1147,
            scalar_v1148,
            scalar_v1149,
            scalar_v1171,
            scalar_v1179,
            scalar_v1180,
            scalar_v1182,
            scalar_v1183,
            scalar_v1184,
            scalar_v1187,
            scalar_v1188,
            scalar_v1193,
            scalar_v1214,
            scalar_v1216,
            scalar_v1245,
            scalar_v1251,
            scalar_v1285,
            scalar_v1307,
            scalar_v1320,
            scalar_v1338,
            scalar_v1401,
            scalar_v1402,
            scalar_v1403,
            scalar_v1404,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1501,
            scalar_v1502,
            scalar_v1503,
            scalar_v1527,
            scalar_v1529,
            scalar_v1530,
            scalar_v1532,
            scalar_v1592,
            scalar_v1593,
            scalar_v1594,
            scalar_v1620,
            scalar_v1622,
            scalar_v1623,
            scalar_v1625,
            scalar_v1702,
            scalar_v1703,
            scalar_v1704,
            scalar_v1705,
            scalar_v1711,
            scalar_v1720,
            scalar_v1721,
            scalar_v1733,
            scalar_v1752,
            scalar_v1762,
            scalar_v1763,
            scalar_v1764,
            scalar_v1765,
            scalar_v1770,
            scalar_v1780,
            scalar_v1781,
            scalar_v1782,
            scalar_v1796,
            scalar_v1804,
            scalar_v1805,
            scalar_v1818,
            scalar_v1823,
            scalar_v1840,
            scalar_v1841,
            scalar_v1847,
            scalar_v1848,
            scalar_v1851,
            scalar_v1857,
            scalar_v1868,
            scalar_v1869,
            scalar_v1870,
            scalar_v1871,
            scalar_v1872,
            scalar_v1873,
            scalar_v1874,
            scalar_v1875,
            scalar_v1876,
            scalar_v1877,
            scalar_v1878,
            scalar_v1879,
            scalar_v1880,
            scalar_v1881,
            scalar_v1882,
            scalar_v1896,
            scalar_v1923,
            scalar_v1924,
            scalar_v1925,
            scalar_v1928,
            scalar_v1947,
            scalar_v1961,
            scalar_v1966,
            scalar_v1968,
            scalar_v1972,
            scalar_v1973,
            scalar_v1974,
            scalar_v1975,
            scalar_v1976,
            scalar_v1985,
            scalar_v1986,
            scalar_v1989,
            scalar_v2012,
            scalar_v2013,
            scalar_v2019,
            scalar_v2020,
            scalar_v2021,
            scalar_v2069,
            scalar_v2070,
            scalar_v2075,
            scalar_v2079,
            scalar_v2086,
            scalar_v2091,
            scalar_v2111,
            scalar_v2131,
            scalar_v2132,
            scalar_v2167,
            scalar_v2173,
            scalar_v2240,
            scalar_v2241,
            scalar_v2271,
            scalar_v2309,
            scalar_v2344,
            scalar_v2345,
            scalar_v2346,
            scalar_v2365,
            scalar_v2378,
            scalar_v2379,
            scalar_v2401,
            scalar_v2402,
            scalar_v2411,
            scalar_v2415,
            scalar_v2434,
            scalar_v2435,
            scalar_v2436,
            scalar_v2439,
            scalar_v2455,
            scalar_v2466,
            scalar_v2487,
            scalar_v2488,
            scalar_v2489,
            scalar_v2527,
            scalar_v2528,
            scalar_v2534,
            scalar_v2538,
            scalar_v2541,
            scalar_v2545,
            scalar_v2549,
            scalar_v2550,
            scalar_v2551,
            scalar_v2552,
            scalar_v2553,
            scalar_v2557,
            scalar_v2558,
            scalar_v2559,
            scalar_v2560,
            scalar_v2561,
            scalar_v2569,
            scalar_v2570,
            scalar_v2578,
            scalar_v2583,
            scalar_v2584,
            scalar_v2588,
            scalar_v2598,
            scalar_v2599,
            scalar_v2602,
            scalar_v2603,
            scalar_v2604,
            scalar_v2605,
            scalar_v2608,
            scalar_v2609,
            scalar_v2619,
            scalar_v2620,
            scalar_v2669,
            scalar_v2673,
            scalar_v2917,
            scalar_v3268,
            scalar_v3269,
            scalar_v3270,
            scalar_v3271,
            scalar_v4305,
            scalar_v4329,
            scalar_v4330,
            scalar_v4347,
            scalar_v4433,
            scalar_v4452,
            scalar_v4795,
            scalar_v4796,
            scalar_v4805,
            scalar_v4806,
            scalar_v4830,
            scalar_v4831,
            scalar_v4842,
            scalar_v4843,
            scalar_v5308,
            scalar_v5365,
            scalar_v5366,
            scalar_v5429,
            scalar_v5430,
            scalar_v5563,
            scalar_v5620,
            scalar_v5621,
            scalar_v6109,
            scalar_v6110,
            scalar_v6321,
            scalar_v6322,
            scalar_v6324,
            scalar_v6325,
            scalar_v6579,
            scalar_v6580,
            scalar_v6581,
            scalar_v6582,
            scalar_v6583,
            scalar_v6584,
            scalar_v7012,
            scalar_v7619,
            scalar_v7716,
            scalar_v8029,
            scalar_v8030,
            scalar_v8031,
            scalar_v8032,
            scalar_v8033,
            scalar_v8249,
            scalar_v8250,
            scalar_v8318,
            scalar_v8874,
            scalar_v8909,
            scalar_v9019,
            scalar_v9020,
            scalar_v9021,
            scalar_v9022,
            scalar_v9337,
            scalar_v9475,
            scalar_v9476,
            scalar_v9596,
            scalar_v9602,
            scalar_v9931,
            scalar_v9932,
            scalar_v10054,
            scalar_v10055,
            scalar_v10060,
            scalar_v10061,
            scalar_v10088,
            scalar_v10089,
            scalar_v20,
            scalar_v720,
            scalar_v722,
            scalar_v723,
            scalar_v2562,
            scalar_v2563,
            scalar_v2571,
            scalar_v2572,
            scalar_v2573,
            scalar_v9589,
            scalar_v9590,
            scalar_v9591,
            scalar_v9592,
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
            scalar_v30,
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
            scalar_v46,
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
            scalar_v101,
            scalar_v108,
            scalar_v121,
            scalar_v123,
            scalar_v178,
            scalar_v198,
            scalar_v201,
            scalar_v221,
            scalar_v262,
            scalar_v265,
            scalar_v285,
            scalar_v288,
            scalar_v314,
            scalar_v316,
            scalar_v318,
            scalar_v321,
            scalar_v322,
            scalar_v328,
            scalar_v331,
            scalar_v332,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v345,
            scalar_v346,
            scalar_v352,
            scalar_v353,
            scalar_v357,
            scalar_v358,
            scalar_v362,
            scalar_v364,
            scalar_v365,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v399,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v430,
            scalar_v432,
            scalar_v433,
            scalar_v451,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v461,
            scalar_v466,
            scalar_v467,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v477,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v485,
            scalar_v486,
            scalar_v491,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v498,
            scalar_v503,
            scalar_v504,
            scalar_v505,
            scalar_v507,
            scalar_v511,
            scalar_v512,
            scalar_v517,
            scalar_v518,
            scalar_v525,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v556,
            scalar_v557,
            scalar_v561,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v580,
            scalar_v583,
            scalar_v591,
            scalar_v600,
            scalar_v613,
            scalar_v622,
            scalar_v634,
            scalar_v637,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v646,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v658,
            scalar_v659,
            scalar_v663,
            scalar_v664,
            scalar_v668,
            scalar_v669,
            scalar_v673,
            scalar_v674,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v684,
            scalar_v685,
            scalar_v689,
            scalar_v692,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v715,
            scalar_v717,
            scalar_v719,
            scalar_v721,
            scalar_v724,
            scalar_v730,
            scalar_v732,
            scalar_v738,
            scalar_v740,
            scalar_v746,
            scalar_v796,
            scalar_v801,
            scalar_v931,
            scalar_v984,
            scalar_v985,
            scalar_v986,
            scalar_v997,
            scalar_v1018,
            scalar_v1019,
            scalar_v1020,
            scalar_v1021,
            scalar_v1022,
            scalar_v1023,
            scalar_v1070,
            scalar_v1080,
            scalar_v1093,
            scalar_v1094,
            scalar_v1098,
            scalar_v1147,
            scalar_v1148,
            scalar_v1149,
            scalar_v1171,
            scalar_v1179,
            scalar_v1180,
            scalar_v1182,
            scalar_v1183,
            scalar_v1184,
            scalar_v1187,
            scalar_v1188,
            scalar_v1193,
            scalar_v1214,
            scalar_v1216,
            scalar_v1245,
            scalar_v1251,
            scalar_v1285,
            scalar_v1307,
            scalar_v1320,
            scalar_v1338,
            scalar_v1401,
            scalar_v1402,
            scalar_v1403,
            scalar_v1404,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1501,
            scalar_v1502,
            scalar_v1503,
            scalar_v1527,
            scalar_v1529,
            scalar_v1530,
            scalar_v1532,
            scalar_v1592,
            scalar_v1593,
            scalar_v1594,
            scalar_v1620,
            scalar_v1622,
            scalar_v1623,
            scalar_v1625,
            scalar_v1702,
            scalar_v1703,
            scalar_v1704,
            scalar_v1705,
            scalar_v1711,
            scalar_v1720,
            scalar_v1721,
            scalar_v1733,
            scalar_v1752,
            scalar_v1762,
            scalar_v1763,
            scalar_v1764,
            scalar_v1765,
            scalar_v1770,
            scalar_v1780,
            scalar_v1781,
            scalar_v1782,
            scalar_v1796,
            scalar_v1804,
            scalar_v1805,
            scalar_v1818,
            scalar_v1823,
            scalar_v1840,
            scalar_v1841,
            scalar_v1847,
            scalar_v1848,
            scalar_v1851,
            scalar_v1857,
            scalar_v1868,
            scalar_v1869,
            scalar_v1870,
            scalar_v1871,
            scalar_v1872,
            scalar_v1873,
            scalar_v1874,
            scalar_v1875,
            scalar_v1876,
            scalar_v1877,
            scalar_v1878,
            scalar_v1879,
            scalar_v1880,
            scalar_v1881,
            scalar_v1882,
            scalar_v1896,
            scalar_v1923,
            scalar_v1924,
            scalar_v1925,
            scalar_v1928,
            scalar_v1947,
            scalar_v1961,
            scalar_v1966,
            scalar_v1968,
            scalar_v1972,
            scalar_v1973,
            scalar_v1974,
            scalar_v1975,
            scalar_v1976,
            scalar_v1985,
            scalar_v1986,
            scalar_v1989,
            scalar_v2012,
            scalar_v2013,
            scalar_v2019,
            scalar_v2020,
            scalar_v2021,
            scalar_v2069,
            scalar_v2070,
            scalar_v2075,
            scalar_v2079,
            scalar_v2086,
            scalar_v2091,
            scalar_v2111,
            scalar_v2131,
            scalar_v2132,
            scalar_v2167,
            scalar_v2173,
            scalar_v2240,
            scalar_v2241,
            scalar_v2271,
            scalar_v2309,
            scalar_v2344,
            scalar_v2345,
            scalar_v2346,
            scalar_v2365,
            scalar_v2378,
            scalar_v2379,
            scalar_v2401,
            scalar_v2402,
            scalar_v2411,
            scalar_v2415,
            scalar_v2434,
            scalar_v2435,
            scalar_v2436,
            scalar_v2439,
            scalar_v2455,
            scalar_v2466,
            scalar_v2487,
            scalar_v2488,
            scalar_v2489,
            scalar_v2527,
            scalar_v2528,
            scalar_v2534,
            scalar_v2538,
            scalar_v2541,
            scalar_v2545,
            scalar_v2549,
            scalar_v2550,
            scalar_v2551,
            scalar_v2552,
            scalar_v2553,
            scalar_v2557,
            scalar_v2558,
            scalar_v2559,
            scalar_v2560,
            scalar_v2561,
            scalar_v2569,
            scalar_v2570,
            scalar_v2578,
            scalar_v2583,
            scalar_v2584,
            scalar_v2588,
            scalar_v2598,
            scalar_v2599,
            scalar_v2602,
            scalar_v2603,
            scalar_v2604,
            scalar_v2605,
            scalar_v2608,
            scalar_v2609,
            scalar_v2619,
            scalar_v2620,
            scalar_v2669,
            scalar_v2673,
            scalar_v2917,
            scalar_v3268,
            scalar_v3269,
            scalar_v3270,
            scalar_v3271,
            scalar_v4305,
            scalar_v4329,
            scalar_v4330,
            scalar_v4347,
            scalar_v4433,
            scalar_v4452,
            scalar_v4795,
            scalar_v4796,
            scalar_v4805,
            scalar_v4806,
            scalar_v4830,
            scalar_v4831,
            scalar_v4842,
            scalar_v4843,
            scalar_v5308,
            scalar_v5365,
            scalar_v5366,
            scalar_v5429,
            scalar_v5430,
            scalar_v5563,
            scalar_v5620,
            scalar_v5621,
            scalar_v6109,
            scalar_v6110,
            scalar_v6321,
            scalar_v6322,
            scalar_v6324,
            scalar_v6325,
            scalar_v6579,
            scalar_v6580,
            scalar_v6581,
            scalar_v6582,
            scalar_v6583,
            scalar_v6584,
            scalar_v7012,
            scalar_v7619,
            scalar_v7716,
            scalar_v8029,
            scalar_v8030,
            scalar_v8031,
            scalar_v8032,
            scalar_v8033,
            scalar_v8249,
            scalar_v8250,
            scalar_v8318,
            scalar_v8874,
            scalar_v8909,
            scalar_v9019,
            scalar_v9020,
            scalar_v9021,
            scalar_v9022,
            scalar_v9337,
            scalar_v9475,
            scalar_v9476,
            scalar_v9596,
            scalar_v9602,
            scalar_v9931,
            scalar_v9932,
            scalar_v10054,
            scalar_v10055,
            scalar_v10060,
            scalar_v10061,
            scalar_v10088,
            scalar_v10089,
            scalar_v20,
            scalar_v720,
            scalar_v722,
            scalar_v723,
            scalar_v2562,
            scalar_v2563,
            scalar_v2571,
            scalar_v2572,
            scalar_v2573,
            scalar_v9589,
            scalar_v9590,
            scalar_v9591,
            scalar_v9592,
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
            "swnlsh" => { validate_parameter("swnlsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ath" => { validate_finite_parameter("ath", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjt505t_va'", name)),
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
        let v21: f64 = p.p154;
        self.scalar_v21 = v21;
        let v22: bool = (0.0 == p.p154);
        self.scalar_v22 = v22;
        let v24: f64 = (if v22 { 1e-12 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: bool = (!v22);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p154 } else { v24 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p1;
        self.scalar_v27 = v27;
        let v28: f64 = (v26 * p.p1);
        self.scalar_v28 = v28;
        let v29: f64 = (1.0 / v28);
        self.scalar_v29 = v29;
        let v30: f64 = p.p134;
        self.scalar_v30 = v30;
        let v33: f64 = p.p67;
        self.scalar_v33 = v33;
        let v34: f64 = (2.0 - p.p67);
        self.scalar_v34 = v34;
        let v35: f64 = f64::powf(2.0, v34);
        self.scalar_v35 = v35;
        let v36: f64 = (1.0 / v35);
        self.scalar_v36 = v36;
        let v37: f64 = p.p114;
        self.scalar_v37 = v37;
        let v38: f64 = p.p115;
        self.scalar_v38 = v38;
        let v39: f64 = (v17 * p.p115);
        self.scalar_v39 = v39;
        let v40: f64 = (v17 * v39);
        self.scalar_v40 = v40;
        let v41: f64 = p.p116;
        self.scalar_v41 = v41;
        let v42: f64 = (v17 + p.p116);
        self.scalar_v42 = v42;
        let v43: f64 = (v40 / v42);
        self.scalar_v43 = v43;
        let v44: f64 = (p.p114 + v43);
        self.scalar_v44 = v44;
        let v46: f64 = (v44 - 0.05);
        self.scalar_v46 = v46;
        let v48: f64 = (v46 / 0.1);
        self.scalar_v48 = v48;
        let v49: bool = (v44 < 0.05);
        self.scalar_v49 = v49;
        let v50: f64 = ((v48) as f64).exp();
        self.scalar_v50 = v50;
        let v51: f64 = (1.0 + v50);
        self.scalar_v51 = v51;
        let v52: f64 = ((v51) as f64).ln();
        self.scalar_v52 = v52;
        let v53: f64 = (0.1 * v52);
        self.scalar_v53 = v53;
        let v54: f64 = (0.05 + v53);
        self.scalar_v54 = v54;
        let v55: f64 = (if v49 { v54 } else { 0.0 });
        self.scalar_v55 = v55;
        let v56: bool = (!v49);
        self.scalar_v56 = v56;
        let v57: f64 = (-v48);
        self.scalar_v57 = v57;
        let v58: f64 = ((v57) as f64).exp();
        self.scalar_v58 = v58;
        let v59: f64 = (1.0 + v58);
        self.scalar_v59 = v59;
        let v60: f64 = ((v59) as f64).ln();
        self.scalar_v60 = v60;
        let v61: f64 = (0.1 * v60);
        self.scalar_v61 = v61;
        let v62: f64 = (v44 + v61);
        self.scalar_v62 = v62;
        let v63: f64 = (if v56 { v62 } else { v55 });
        self.scalar_v63 = v63;
        let v64: f64 = (1.0 / p.p114);
        self.scalar_v64 = v64;
        let v65: f64 = p.p66;
        self.scalar_v65 = v65;
        let v66: f64 = (1.0 / p.p66);
        self.scalar_v66 = v66;
        let v67: f64 = p.p71;
        self.scalar_v67 = v67;
        let v68: f64 = p.p72;
        self.scalar_v68 = v68;
        let v69: f64 = (2.0 - p.p72);
        self.scalar_v69 = v69;
        let v70: f64 = f64::powf(2.0, v69);
        self.scalar_v70 = v70;
        let v71: f64 = (1.0 / v70);
        self.scalar_v71 = v71;
        let v72: f64 = p.p117;
        self.scalar_v72 = v72;
        let v73: f64 = p.p118;
        self.scalar_v73 = v73;
        let v74: f64 = (v17 * p.p118);
        self.scalar_v74 = v74;
        let v75: f64 = (v17 * v74);
        self.scalar_v75 = v75;
        let v76: f64 = p.p119;
        self.scalar_v76 = v76;
        let v77: f64 = (v17 + p.p119);
        self.scalar_v77 = v77;
        let v78: f64 = (v75 / v77);
        self.scalar_v78 = v78;
        let v79: f64 = (p.p117 + v78);
        self.scalar_v79 = v79;
        let v80: f64 = (v79 - 0.05);
        self.scalar_v80 = v80;
        let v81: f64 = (v80 / 0.1);
        self.scalar_v81 = v81;
        let v82: bool = (v79 < 0.05);
        self.scalar_v82 = v82;
        let v83: f64 = ((v81) as f64).exp();
        self.scalar_v83 = v83;
        let v84: f64 = (1.0 + v83);
        self.scalar_v84 = v84;
        let v85: f64 = ((v84) as f64).ln();
        self.scalar_v85 = v85;
        let v86: f64 = (0.1 * v85);
        self.scalar_v86 = v86;
        let v87: f64 = (0.05 + v86);
        self.scalar_v87 = v87;
        let v88: f64 = (if v82 { v87 } else { 0.0 });
        self.scalar_v88 = v88;
        let v89: bool = (!v82);
        self.scalar_v89 = v89;
        let v90: f64 = (-v81);
        self.scalar_v90 = v90;
        let v91: f64 = ((v90) as f64).exp();
        self.scalar_v91 = v91;
        let v92: f64 = (1.0 + v91);
        self.scalar_v92 = v92;
        let v93: f64 = ((v92) as f64).ln();
        self.scalar_v93 = v93;
        let v94: f64 = (0.1 * v93);
        self.scalar_v94 = v94;
        let v95: f64 = (v79 + v94);
        self.scalar_v95 = v95;
        let v96: f64 = (if v89 { v95 } else { v88 });
        self.scalar_v96 = v96;
        let v97: f64 = (1.0 / p.p117);
        self.scalar_v97 = v97;
        let v98: f64 = (1.0 / p.p71);
        self.scalar_v98 = v98;
        let v99: f64 = p.p83;
        self.scalar_v99 = v99;
        let v100: f64 = (1.0 / p.p83);
        self.scalar_v100 = v100;
        let v101: f64 = (1.0 - v100);
        self.scalar_v101 = v101;
        let v108: f64 = p.p125;
        self.scalar_v108 = v108;
        let v121: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v121 = v121;
        let v123: f64 = (1.0 / v121);
        self.scalar_v123 = v123;
        let v178: f64 = p.p105;
        self.scalar_v178 = v178;
        let v198: f64 = p.p64;
        self.scalar_v198 = v198;
        let v201: f64 = p.p110;
        self.scalar_v201 = v201;
        let v221: f64 = p.p80;
        self.scalar_v221 = v221;
        let v262: f64 = p.p27;
        self.scalar_v262 = v262;
        let v265: f64 = p.p109;
        self.scalar_v265 = v265;
        let v285: f64 = p.p138;
        self.scalar_v285 = v285;
        let v288: f64 = p.p140;
        self.scalar_v288 = v288;
        let v314: f64 = p.p65;
        self.scalar_v314 = v314;
        let v316: f64 = p.p137;
        self.scalar_v316 = v316;
        let v318: f64 = p.p139;
        self.scalar_v318 = v318;
        let v321: f64 = p.p75;
        self.scalar_v321 = v321;
        let v322: f64 = (1.0 - p.p75);
        self.scalar_v322 = v322;
        let v328: f64 = p.p70;
        self.scalar_v328 = v328;
        let v331: f64 = p.p54;
        self.scalar_v331 = v331;
        let v332: f64 = p.p97;
        self.scalar_v332 = v332;
        let v338: f64 = p.p56;
        self.scalar_v338 = v338;
        let v339: f64 = p.p98;
        self.scalar_v339 = v339;
        let v340: f64 = p.p96;
        self.scalar_v340 = v340;
        let v341: f64 = (p.p98 - p.p96);
        self.scalar_v341 = v341;
        let v345: f64 = p.p55;
        self.scalar_v345 = v345;
        let v346: f64 = p.p101;
        self.scalar_v346 = v346;
        let v352: f64 = p.p57;
        self.scalar_v352 = v352;
        let v353: f64 = p.p102;
        self.scalar_v353 = v353;
        let v357: f64 = p.p58;
        self.scalar_v357 = v357;
        let v358: f64 = p.p104;
        self.scalar_v358 = v358;
        let v362: f64 = p.p59;
        self.scalar_v362 = v362;
        let v364: f64 = p.p60;
        self.scalar_v364 = v364;
        let v365: f64 = p.p99;
        self.scalar_v365 = v365;
        let v369: f64 = p.p122;
        self.scalar_v369 = v369;
        let v370: bool = (0.0 != p.p122);
        self.scalar_v370 = v370;
        let v371: f64 = p.p10;
        self.scalar_v371 = v371;
        let v399: bool = (!v370);
        self.scalar_v399 = v399;
        let v401: f64 = p.p123;
        self.scalar_v401 = v401;
        let v402: bool = (0.0 != p.p123);
        self.scalar_v402 = v402;
        let v403: f64 = p.p11;
        self.scalar_v403 = v403;
        let v430: bool = (!v402);
        self.scalar_v430 = v430;
        let v432: f64 = p.p43;
        self.scalar_v432 = v432;
        let v433: f64 = p.p124;
        self.scalar_v433 = v433;
        let v451: f64 = p.p9;
        self.scalar_v451 = v451;
        let v453: f64 = (4.0 - p.p98);
        self.scalar_v453 = v453;
        let v454: f64 = (v453 - p.p96);
        self.scalar_v454 = v454;
        let v455: f64 = p.p121;
        self.scalar_v455 = v455;
        let v456: f64 = (v454 + p.p121);
        self.scalar_v456 = v456;
        let v461: f64 = (-p.p105);
        self.scalar_v461 = v461;
        let v466: f64 = p.p12;
        self.scalar_v466 = v466;
        let v467: f64 = (1.0 - p.p98);
        self.scalar_v467 = v467;
        let v471: f64 = p.p30;
        self.scalar_v471 = v471;
        let v472: f64 = p.p103;
        self.scalar_v472 = v472;
        let v473: f64 = (1.0 - p.p103);
        self.scalar_v473 = v473;
        let v477: f64 = p.p20;
        self.scalar_v477 = v477;
        let v479: f64 = p.p21;
        self.scalar_v479 = v479;
        let v480: f64 = (2.0 * p.p21);
        self.scalar_v480 = v480;
        let v481: f64 = (6.0 - v480);
        self.scalar_v481 = v481;
        let v485: f64 = p.p113;
        self.scalar_v485 = v485;
        let v486: f64 = (-p.p113);
        self.scalar_v486 = v486;
        let v491: f64 = p.p31;
        self.scalar_v491 = v491;
        let v492: f64 = p.p32;
        self.scalar_v492 = v492;
        let v493: f64 = (2.0 * p.p32);
        self.scalar_v493 = v493;
        let v494: f64 = (6.0 - v493);
        self.scalar_v494 = v494;
        let v498: f64 = (-p.p110);
        self.scalar_v498 = v498;
        let v503: f64 = p.p16;
        self.scalar_v503 = v503;
        let v504: f64 = (4.0 - p.p97);
        self.scalar_v504 = v504;
        let v505: f64 = (p.p121 + v504);
        self.scalar_v505 = v505;
        let v507: f64 = p.p17;
        self.scalar_v507 = v507;
        let v511: f64 = p.p111;
        self.scalar_v511 = v511;
        let v512: f64 = (-p.p111);
        self.scalar_v512 = v512;
        let v517: f64 = p.p18;
        self.scalar_v517 = v517;
        let v518: f64 = p.p19;
        self.scalar_v518 = v518;
        let v525: f64 = p.p24;
        self.scalar_v525 = v525;
        let v526: bool = (1.0 == p.p24);
        self.scalar_v526 = v526;
        let v527: f64 = p.p25;
        self.scalar_v527 = v527;
        let v528: f64 = p.p107;
        self.scalar_v528 = v528;
        let v529: f64 = (-p.p107);
        self.scalar_v529 = v529;
        let v535: f64 = p.p28;
        self.scalar_v535 = v535;
        let v536: f64 = p.p106;
        self.scalar_v536 = v536;
        let v537: f64 = (-p.p106);
        self.scalar_v537 = v537;
        let v542: f64 = p.p26;
        self.scalar_v542 = v542;
        let v543: f64 = p.p108;
        self.scalar_v543 = v543;
        let v544: f64 = (-p.p108);
        self.scalar_v544 = v544;
        let v550: f64 = p.p29;
        self.scalar_v550 = v550;
        let v551: f64 = (4.0 - p.p103);
        self.scalar_v551 = v551;
        let v552: f64 = (p.p121 + v551);
        self.scalar_v552 = v552;
        let v556: f64 = p.p112;
        self.scalar_v556 = v556;
        let v557: f64 = (-p.p112);
        self.scalar_v557 = v557;
        let v561: f64 = p.p22;
        self.scalar_v561 = v561;
        let v562: f64 = p.p23;
        self.scalar_v562 = v562;
        let v563: f64 = (2.0 * p.p23);
        self.scalar_v563 = v563;
        let v564: f64 = (6.0 - v563);
        self.scalar_v564 = v564;
        let v571: f64 = p.p149;
        self.scalar_v571 = v571;
        let v572: f64 = p.p150;
        self.scalar_v572 = v572;
        let v573: f64 = (4.0 / p.p150);
        self.scalar_v573 = v573;
        let v580: f64 = p.p155;
        self.scalar_v580 = v580;
        let v583: f64 = p.p157;
        self.scalar_v583 = v583;
        let v591: f64 = p.p35;
        self.scalar_v591 = v591;
        let v600: f64 = p.p34;
        self.scalar_v600 = v600;
        let v613: f64 = p.p37;
        self.scalar_v613 = v613;
        let v622: f64 = p.p36;
        self.scalar_v622 = v622;
        let v634: f64 = p.p14;
        self.scalar_v634 = v634;
        let v637: f64 = p.p13;
        self.scalar_v637 = v637;
        let v640: f64 = p.p133;
        self.scalar_v640 = v640;
        let v641: f64 = p.p141;
        self.scalar_v641 = v641;
        let v642: f64 = (4.0 - p.p141);
        self.scalar_v642 = v642;
        let v646: f64 = (-p.p140);
        self.scalar_v646 = v646;
        let v651: f64 = p.p142;
        self.scalar_v651 = v651;
        let v652: f64 = (0.5 * p.p142);
        self.scalar_v652 = v652;
        let v653: f64 = (3.5 - v652);
        self.scalar_v653 = v653;
        let v658: f64 = p.p135;
        self.scalar_v658 = v658;
        let v659: f64 = (1.0 - p.p141);
        self.scalar_v659 = v659;
        let v663: f64 = p.p136;
        self.scalar_v663 = v663;
        let v664: f64 = (1.0 - p.p142);
        self.scalar_v664 = v664;
        let v668: f64 = p.p86;
        self.scalar_v668 = v668;
        let v669: f64 = (p.p98 - 2.0);
        self.scalar_v669 = v669;
        let v673: f64 = p.p120;
        self.scalar_v673 = v673;
        let v674: f64 = (-p.p120);
        self.scalar_v674 = v674;
        let v678: f64 = p.p87;
        self.scalar_v678 = v678;
        let v679: f64 = (p.p98 + p.p96);
        self.scalar_v679 = v679;
        let v680: f64 = (v679 - 1.0);
        self.scalar_v680 = v680;
        let v684: f64 = p.p88;
        self.scalar_v684 = v684;
        let v685: f64 = (p.p99 - 1.0);
        self.scalar_v685 = v685;
        let v689: f64 = p.p89;
        self.scalar_v689 = v689;
        let v692: f64 = (p.p87 + p.p88);
        self.scalar_v692 = v692;
        let v694: f64 = p.p90;
        self.scalar_v694 = v694;
        let v695: f64 = p.p100;
        self.scalar_v695 = v695;
        let v696: f64 = (p.p100 - 1.0);
        self.scalar_v696 = v696;
        let v715: f64 = (v12 * 1.081);
        self.scalar_v715 = v715;
        let v717: f64 = p.p92;
        self.scalar_v717 = v717;
        let v719: f64 = p.p146;
        self.scalar_v719 = v719;
        let v721: f64 = p.p148;
        self.scalar_v721 = v721;
        let v724: bool = (p.p57 > 0.0);
        self.scalar_v724 = v724;
        let v730: bool = (!v724);
        self.scalar_v730 = v730;
        let v732: bool = (p.p58 > 0.0);
        self.scalar_v732 = v732;
        let v738: bool = (!v732);
        self.scalar_v738 = v738;
        let v740: bool = (p.p59 > 0.0);
        self.scalar_v740 = v740;
        let v746: bool = (!v740);
        self.scalar_v746 = v746;
        let v796: f64 = p.p151;
        self.scalar_v796 = v796;
        let v801: f64 = ((p.p151) as f64).exp();
        self.scalar_v801 = v801;
        let v931: f64 = p.p153;
        self.scalar_v931 = v931;
        let v984: f64 = p.p62;
        self.scalar_v984 = v984;
        let v985: f64 = p.p61;
        self.scalar_v985 = v985;
        let v986: f64 = (p.p62 * p.p61);
        self.scalar_v986 = v986;
        let v997: f64 = p.p63;
        self.scalar_v997 = v997;
        let v1018: f64 = (-1.0 / p.p63);
        self.scalar_v1018 = v1018;
        let v1019: f64 = ((v1018) as f64).exp();
        self.scalar_v1019 = v1019;
        let v1020: f64 = (1.0 + v1019);
        self.scalar_v1020 = v1020;
        let v1021: f64 = ((v1020) as f64).ln();
        self.scalar_v1021 = v1021;
        let v1022: f64 = (p.p63 * v1021);
        self.scalar_v1022 = v1022;
        let v1023: f64 = (1.0 + v1022);
        self.scalar_v1023 = v1023;
        let v1070: f64 = p.p152;
        self.scalar_v1070 = v1070;
        let v1080: f64 = (0.5 * p.p61);
        self.scalar_v1080 = v1080;
        let v1093: f64 = p.p73;
        self.scalar_v1093 = v1093;
        let v1094: bool = (0.0 == p.p73);
        self.scalar_v1094 = v1094;
        let v1098: bool = (!v1094);
        self.scalar_v1098 = v1098;
        let v1147: f64 = (-1.0 / p.p67);
        self.scalar_v1147 = v1147;
        let v1148: f64 = f64::powf(3.0, v1147);
        self.scalar_v1148 = v1148;
        let v1149: f64 = (1.0 - v1148);
        self.scalar_v1149 = v1149;
        let v1171: f64 = (1.0 - p.p67);
        self.scalar_v1171 = v1171;
        let v1179: f64 = p.p74;
        self.scalar_v1179 = v1179;
        let v1180: bool = (1.0 == p.p74);
        self.scalar_v1180 = v1180;
        let v1182: bool = (2.0 == p.p74);
        self.scalar_v1182 = v1182;
        let v1183: bool = (!v1180);
        self.scalar_v1183 = v1183;
        let v1184: bool = (v1182 && v1183);
        self.scalar_v1184 = v1184;
        let v1187: bool = (!v1182);
        self.scalar_v1187 = v1187;
        let v1188: bool = (v1183 && v1187);
        self.scalar_v1188 = v1188;
        let v1193: f64 = (-1.0 / p.p72);
        self.scalar_v1193 = v1193;
        let v1214: f64 = p.p76;
        self.scalar_v1214 = v1214;
        let v1216: f64 = (1.0 - p.p72);
        self.scalar_v1216 = v1216;
        let v1245: bool = (0.0 == p.p92);
        self.scalar_v1245 = v1245;
        let v1251: bool = (!v1245);
        self.scalar_v1251 = v1251;
        let v1285: f64 = p.p15;
        self.scalar_v1285 = v1285;
        let v1307: f64 = p.p156;
        self.scalar_v1307 = v1307;
        let v1320: f64 = p.p158;
        self.scalar_v1320 = v1320;
        let v1338: f64 = p.p159;
        self.scalar_v1338 = v1338;
        let v1401: f64 = p.p93;
        self.scalar_v1401 = v1401;
        let v1402: bool = (0.0 == p.p93);
        self.scalar_v1402 = v1402;
        let v1403: bool = (!v526);
        self.scalar_v1403 = v1403;
        let v1404: bool = (v1402 && v1403);
        self.scalar_v1404 = v1404;
        let v1406: bool = (!v1402);
        self.scalar_v1406 = v1406;
        let v1407: bool = (v1403 && v1406);
        self.scalar_v1407 = v1407;
        let v1408: f64 = (1.0 - p.p93);
        self.scalar_v1408 = v1408;
        let v1501: bool = (p.p34 > 0.0);
        self.scalar_v1501 = v1501;
        let v1502: bool = (p.p35 > 0.0);
        self.scalar_v1502 = v1502;
        let v1503: bool = (v1501 && v1502);
        self.scalar_v1503 = v1503;
        let v1527: f64 = (-2.0 - p.p67);
        self.scalar_v1527 = v1527;
        let v1529: f64 = (p.p67 * p.p67);
        self.scalar_v1529 = v1529;
        let v1530: f64 = (1.0 - v1529);
        self.scalar_v1530 = v1530;
        let v1532: f64 = (p.p67 - 1.0);
        self.scalar_v1532 = v1532;
        let v1592: bool = (p.p36 > 0.0);
        self.scalar_v1592 = v1592;
        let v1593: bool = (p.p37 > 0.0);
        self.scalar_v1593 = v1593;
        let v1594: bool = (v1592 && v1593);
        self.scalar_v1594 = v1594;
        let v1620: f64 = (-2.0 - p.p72);
        self.scalar_v1620 = v1620;
        let v1622: f64 = (p.p72 * p.p72);
        self.scalar_v1622 = v1622;
        let v1623: f64 = (1.0 - v1622);
        self.scalar_v1623 = v1623;
        let v1625: f64 = (p.p72 - 1.0);
        self.scalar_v1625 = v1625;
        let v1702: f64 = p.p8;
        self.scalar_v1702 = v1702;
        let v1703: bool = (1.0 == p.p8);
        self.scalar_v1703 = v1703;
        let v1704: f64 = p.p143;
        self.scalar_v1704 = v1704;
        let v1705: f64 = (2.0 * p.p143);
        self.scalar_v1705 = v1705;
        let v1711: f64 = p.p144;
        self.scalar_v1711 = v1711;
        let v1720: f64 = (1.0 - p.p143);
        self.scalar_v1720 = v1720;
        let v1721: f64 = (2.0 * v1720);
        self.scalar_v1721 = v1721;
        let v1733: bool = (!v1703);
        self.scalar_v1733 = v1733;
        let v1752: f64 = (4.0 * p.p144);
        self.scalar_v1752 = v1752;
        let v1762: f64 = p.p5;
        self.scalar_v1762 = v1762;
        let v1763: bool = (p.p5 > 0.0);
        self.scalar_v1763 = v1763;
        let v1764: bool = (p.p33 > 0.0);
        self.scalar_v1764 = v1764;
        let v1765: bool = (v1763 && v1764);
        self.scalar_v1765 = v1765;
        let v1770: f64 = (p.p33 * 2.0);
        self.scalar_v1770 = v1770;
        let v1780: bool = (v1703 && v1765);
        self.scalar_v1780 = v1780;
        let v1781: f64 = (p.p33 * v1720);
        self.scalar_v1781 = v1781;
        let v1782: f64 = (2.0 * v1781);
        self.scalar_v1782 = v1782;
        let v1796: bool = (v1733 && v1765);
        self.scalar_v1796 = v1796;
        let v1804: bool = (1.0 == p.p5);
        self.scalar_v1804 = v1804;
        let v1805: bool = (v1765 && v1804);
        self.scalar_v1805 = v1805;
        let v1818: f64 = (if v1805 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1818 = v1818;
        let v1823: f64 = (0.5 * v1818);
        self.scalar_v1823 = v1823;
        let v1840: bool = (!v1804);
        self.scalar_v1840 = v1840;
        let v1841: bool = (v1765 && v1840);
        self.scalar_v1841 = v1841;
        let v1847: f64 = p.p84;
        self.scalar_v1847 = v1847;
        let v1848: bool = (1.0 == p.p84);
        self.scalar_v1848 = v1848;
        let v1851: f64 = (if v1848 { 1e-12 } else { v1818 });
        self.scalar_v1851 = v1851;
        let v1857: f64 = (0.5 * v1851);
        self.scalar_v1857 = v1857;
        let v1868: f64 = p.p82;
        self.scalar_v1868 = v1868;
        let v1869: f64 = f64::powf(v101, p.p82);
        self.scalar_v1869 = v1869;
        let v1870: f64 = (1.0 - v1869);
        self.scalar_v1870 = v1870;
        let v1871: f64 = (1.0 / v1870);
        self.scalar_v1871 = v1871;
        let v1872: f64 = (if v1848 { v1871 } else { 0.0 });
        self.scalar_v1872 = v1872;
        let v1873: f64 = p.p81;
        self.scalar_v1873 = v1873;
        let v1874: f64 = (v101 * p.p81);
        self.scalar_v1874 = v1874;
        let v1875: f64 = (if v1848 { v1874 } else { 0.0 });
        self.scalar_v1875 = v1875;
        let v1876: f64 = (v1872 * v1872);
        self.scalar_v1876 = v1876;
        let v1877: f64 = (p.p82 - 1.0);
        self.scalar_v1877 = v1877;
        let v1878: f64 = f64::powf(v101, v1877);
        self.scalar_v1878 = v1878;
        let v1879: f64 = (v1876 * v1878);
        self.scalar_v1879 = v1879;
        let v1880: f64 = (p.p82 * v1879);
        self.scalar_v1880 = v1880;
        let v1881: f64 = (v1880 / p.p81);
        self.scalar_v1881 = v1881;
        let v1882: f64 = (if v1848 { v1881 } else { 0.0 });
        self.scalar_v1882 = v1882;
        let v1896: bool = (!v1848);
        self.scalar_v1896 = v1896;
        let v1923: f64 = p.p39;
        self.scalar_v1923 = v1923;
        let v1924: bool = (1.0 == p.p39);
        self.scalar_v1924 = v1924;
        let v1925: f64 = p.p44;
        self.scalar_v1925 = v1925;
        let v1928: f64 = p.p42;
        self.scalar_v1928 = v1928;
        let v1947: f64 = p.p41;
        self.scalar_v1947 = v1947;
        let v1961: f64 = p.p40;
        self.scalar_v1961 = v1961;
        let v1966: bool = (2.0 == p.p39);
        self.scalar_v1966 = v1966;
        let v1968: bool = (!v1924);
        self.scalar_v1968 = v1968;
        let v1972: f64 = p.p46;
        self.scalar_v1972 = v1972;
        let v1973: f64 = (2.0 * p.p46);
        self.scalar_v1973 = v1973;
        let v1974: f64 = p.p45;
        self.scalar_v1974 = v1974;
        let v1975: f64 = (p.p45 * p.p45);
        self.scalar_v1975 = v1975;
        let v1976: f64 = (v1973 / v1975);
        self.scalar_v1976 = v1976;
        let v1985: f64 = p.p7;
        self.scalar_v1985 = v1985;
        let v1986: bool = (0.0 == p.p7);
        self.scalar_v1986 = v1986;
        let v1989: bool = (!v1986);
        self.scalar_v1989 = v1989;
        let v2012: f64 = p.p47;
        self.scalar_v2012 = v2012;
        let v2013: f64 = (2.0 * p.p47);
        self.scalar_v2013 = v2013;
        let v2019: f64 = (1.0 + p.p47);
        self.scalar_v2019 = v2019;
        let v2020: f64 = (1.0 + v2013);
        self.scalar_v2020 = v2020;
        let v2021: f64 = (v2019 / v2020);
        self.scalar_v2021 = v2021;
        let v2069: bool = (3.0 == p.p39);
        self.scalar_v2069 = v2069;
        let v2070: bool = (!v1966);
        self.scalar_v2070 = v2070;
        let v2075: f64 = p.p48;
        self.scalar_v2075 = v2075;
        let v2079: f64 = p.p49;
        self.scalar_v2079 = v2079;
        let v2086: f64 = p.p52;
        self.scalar_v2086 = v2086;
        let v2091: f64 = p.p51;
        self.scalar_v2091 = v2091;
        let v2111: f64 = p.p50;
        self.scalar_v2111 = v2111;
        let v2131: f64 = p.p53;
        self.scalar_v2131 = v2131;
        let v2132: bool = (1.0 == p.p53);
        self.scalar_v2132 = v2132;
        let v2167: bool = (!v2069);
        self.scalar_v2167 = v2167;
        let v2173: bool = (!v2132);
        self.scalar_v2173 = v2173;
        let v2240: f64 = p.p68;
        self.scalar_v2240 = v2240;
        let v2241: f64 = (1.0 - p.p68);
        self.scalar_v2241 = v2241;
        let v2271: f64 = p.p77;
        self.scalar_v2271 = v2271;
        let v2309: f64 = (1.0 - p.p77);
        self.scalar_v2309 = v2309;
        let v2344: f64 = (-1.0 / p.p139);
        self.scalar_v2344 = v2344;
        let v2345: f64 = f64::powf(2.0, v2344);
        self.scalar_v2345 = v2345;
        let v2346: f64 = (1.0 - v2345);
        self.scalar_v2346 = v2346;
        let v2365: f64 = (1.0 - p.p139);
        self.scalar_v2365 = v2365;
        let v2378: f64 = p.p85;
        self.scalar_v2378 = v2378;
        let v2379: f64 = (1.0 / p.p85);
        self.scalar_v2379 = v2379;
        let v2401: f64 = p.p79;
        self.scalar_v2401 = v2401;
        let v2402: bool = (0.0 == p.p79);
        self.scalar_v2402 = v2402;
        let v2411: f64 = p.p91;
        self.scalar_v2411 = v2411;
        let v2415: bool = (!v2402);
        self.scalar_v2415 = v2415;
        let v2434: bool = (3.0 == p.p5);
        self.scalar_v2434 = v2434;
        let v2435: bool = (v1804 || v2434);
        self.scalar_v2435 = v2435;
        let v2436: bool = (v1764 && v2435);
        self.scalar_v2436 = v2436;
        let v2439: bool = (v2402 && v2436);
        self.scalar_v2439 = v2439;
        let v2455: f64 = (p.p33 * 0.5);
        self.scalar_v2455 = v2455;
        let v2466: bool = (v2415 && v2436);
        self.scalar_v2466 = v2466;
        let v2487: f64 = p.p6;
        self.scalar_v2487 = v2487;
        let v2488: bool = (1.0 == p.p6);
        self.scalar_v2488 = v2488;
        let v2489: f64 = (-p.p67);
        self.scalar_v2489 = v2489;
        let v2527: f64 = p.p95;
        self.scalar_v2527 = v2527;
        let v2528: f64 = (1.0 - p.p95);
        self.scalar_v2528 = v2528;
        let v2534: f64 = p.p94;
        self.scalar_v2534 = v2534;
        let v2538: f64 = (1.0 - p.p94);
        self.scalar_v2538 = v2538;
        let v2541: bool = (!v2488);
        self.scalar_v2541 = v2541;
        let v2545: f64 = p.p147;
        self.scalar_v2545 = v2545;
        let v2549: f64 = (1.0 - p.p148);
        self.scalar_v2549 = v2549;
        let v2550: bool = (p.p146 > v28);
        self.scalar_v2550 = v2550;
        let v2551: f64 = p.p145;
        self.scalar_v2551 = v2551;
        let v2552: bool = (0.0 == p.p145);
        self.scalar_v2552 = v2552;
        let v2553: bool = (v2550 && v2552);
        self.scalar_v2553 = v2553;
        let v2557: f64 = ((v2549) as f64).abs();
        self.scalar_v2557 = v2557;
        let v2558: bool = (v2557 < 1e-6);
        self.scalar_v2558 = v2558;
        let v2559: bool = (!v2552);
        self.scalar_v2559 = v2559;
        let v2560: bool = (v2550 && v2559);
        self.scalar_v2560 = v2560;
        let v2561: bool = (v2558 && v2560);
        self.scalar_v2561 = v2561;
        let v2569: bool = (!v2558);
        self.scalar_v2569 = v2569;
        let v2570: bool = (v2560 && v2569);
        self.scalar_v2570 = v2570;
        let v2578: bool = (!v2550);
        self.scalar_v2578 = v2578;
        let v2583: f64 = p.p130;
        self.scalar_v2583 = v2583;
        let v2584: bool = (p.p130 > 0.0);
        self.scalar_v2584 = v2584;
        let v2588: bool = (!v2584);
        self.scalar_v2588 = v2588;
        let v2598: f64 = p.p131;
        self.scalar_v2598 = v2598;
        let v2599: bool = (1.0 == p.p131);
        self.scalar_v2599 = v2599;
        let v2602: bool = (2.0 == p.p131);
        self.scalar_v2602 = v2602;
        let v2603: bool = (!v2599);
        self.scalar_v2603 = v2603;
        let v2604: bool = (v2602 && v2603);
        self.scalar_v2604 = v2604;
        let v2605: f64 = p.p132;
        self.scalar_v2605 = v2605;
        let v2608: bool = (!v2602);
        self.scalar_v2608 = v2608;
        let v2609: bool = (v2603 && v2608);
        self.scalar_v2609 = v2609;
        let v2619: f64 = p.p69;
        self.scalar_v2619 = v2619;
        let v2620: f64 = p.p78;
        self.scalar_v2620 = v2620;
        let v2669: f64 = (p.p3 * p.p69);
        self.scalar_v2669 = v2669;
        let v2673: f64 = (p.p3 * p.p78);
        self.scalar_v2673 = v2673;
        let v2917: f64 = (p.p139 - 1.0);
        self.scalar_v2917 = v2917;
        let v3268: f64 = (-p.p3);
        self.scalar_v3268 = v3268;
        let v3269: f64 = (p.p3 + v3268);
        self.scalar_v3269 = v3269;
        let v3270: f64 = (v3268 - v3268);
        self.scalar_v3270 = v3270;
        let v3271: f64 = (p.p3 + v3269);
        self.scalar_v3271 = v3271;
        let v4305: f64 = (v1171 - 1.0);
        self.scalar_v4305 = v4305;
        let v4329: f64 = (if v1180 { p.p3 } else { 0.0 });
        self.scalar_v4329 = v4329;
        let v4330: f64 = (if v1180 { v3268 } else { 0.0 });
        self.scalar_v4330 = v4330;
        let v4347: f64 = (v1193 - 1.0);
        self.scalar_v4347 = v4347;
        let v4433: f64 = (p.p76 - 1.0);
        self.scalar_v4433 = v4433;
        let v4452: f64 = (v1216 - 1.0);
        self.scalar_v4452 = v4452;
        let v4795: f64 = (v3268 / 0.0001);
        self.scalar_v4795 = v4795;
        let v4796: f64 = (p.p3 / 0.0001);
        self.scalar_v4796 = v4796;
        let v4805: f64 = (-v4795);
        self.scalar_v4805 = v4805;
        let v4806: f64 = (-v4796);
        self.scalar_v4806 = v4806;
        let v4830: f64 = (v3268 / 0.001);
        self.scalar_v4830 = v4830;
        let v4831: f64 = (p.p3 / 0.001);
        self.scalar_v4831 = v4831;
        let v4842: f64 = (-v4830);
        self.scalar_v4842 = v4842;
        let v4843: f64 = (-v4831);
        self.scalar_v4843 = v4843;
        let v5308: f64 = (v1527 - 1.0);
        self.scalar_v5308 = v5308;
        let v5365: f64 = (v35 * v3268);
        self.scalar_v5365 = v5365;
        let v5366: f64 = (p.p3 * v35);
        self.scalar_v5366 = v5366;
        let v5429: f64 = (0.5 * v3268);
        self.scalar_v5429 = v5429;
        let v5430: f64 = (p.p3 * 0.5);
        self.scalar_v5430 = v5430;
        let v5563: f64 = (v1620 - 1.0);
        self.scalar_v5563 = v5563;
        let v5620: f64 = (p.p3 * v70);
        self.scalar_v5620 = v5620;
        let v5621: f64 = (v70 * v3268);
        self.scalar_v5621 = v5621;
        let v6109: f64 = (p.p3 * 0.0);
        self.scalar_v6109 = v6109;
        let v6110: f64 = (0.0 * v3268);
        self.scalar_v6110 = v6110;
        let v6321: f64 = (if v1805 { v3269 } else { 0.0 });
        self.scalar_v6321 = v6321;
        let v6322: f64 = (if v1805 { v3271 } else { 0.0 });
        self.scalar_v6322 = v6322;
        let v6324: f64 = (if v1805 { v3270 } else { 0.0 });
        self.scalar_v6324 = v6324;
        let v6325: f64 = (if v1805 { v3268 } else { 0.0 });
        self.scalar_v6325 = v6325;
        let v6579: f64 = (if v1848 { p.p3 } else { 0.0 });
        self.scalar_v6579 = v6579;
        let v6580: f64 = (if v1848 { v3269 } else { 0.0 });
        self.scalar_v6580 = v6580;
        let v6581: f64 = (if v1848 { v3268 } else { 0.0 });
        self.scalar_v6581 = v6581;
        let v6582: f64 = (-v6579);
        self.scalar_v6582 = v6582;
        let v6583: f64 = (-v6580);
        self.scalar_v6583 = v6583;
        let v6584: f64 = (-v6581);
        self.scalar_v6584 = v6584;
        let v7012: f64 = (p.p41 - 1.0);
        self.scalar_v7012 = v7012;
        let v7619: f64 = (p.p49 - 1.0);
        self.scalar_v7619 = v7619;
        let v7716: f64 = (p.p50 - 1.0);
        self.scalar_v7716 = v7716;
        let v8029: f64 = (if v526 { p.p3 } else { 0.0 });
        self.scalar_v8029 = v8029;
        let v8030: f64 = (if v526 { v3268 } else { 0.0 });
        self.scalar_v8030 = v8030;
        let v8031: f64 = (if v1403 { p.p3 } else { v8029 });
        self.scalar_v8031 = v8031;
        let v8032: f64 = (if v1403 { 0.0 } else { v8030 });
        self.scalar_v8032 = v8032;
        let v8033: f64 = (if v1403 { v3268 } else { 0.0 });
        self.scalar_v8033 = v8033;
        let v8249: f64 = (0.0 * v3269);
        self.scalar_v8249 = v8249;
        let v8250: f64 = (0.0 * v3270);
        self.scalar_v8250 = v8250;
        let v8318: f64 = (v3270 - v3270);
        self.scalar_v8318 = v8318;
        let v8874: f64 = (v2365 - 1.0);
        self.scalar_v8874 = v8874;
        let v8909: f64 = (v2379 - 1.0);
        self.scalar_v8909 = v8909;
        let v9019: f64 = (p.p3 / p.p91);
        self.scalar_v9019 = v9019;
        let v9020: f64 = (v3269 / p.p91);
        self.scalar_v9020 = v9020;
        let v9021: f64 = (v3270 / p.p91);
        self.scalar_v9021 = v9021;
        let v9022: f64 = (v3268 / p.p91);
        self.scalar_v9022 = v9022;
        let v9337: f64 = (v2489 - 1.0);
        self.scalar_v9337 = v9337;
        let v9475: f64 = (p.p3 * 0.2);
        self.scalar_v9475 = v9475;
        let v9476: f64 = (0.2 * v3268);
        self.scalar_v9476 = v9476;
        let v9596: f64 = (v2549 - 1.0);
        self.scalar_v9596 = v9596;
        let v9602: f64 = (1.0 / v26);
        self.scalar_v9602 = v9602;
        let v9931: f64 = (p.p3 * p.p3);
        self.scalar_v9931 = v9931;
        let v9932: f64 = (p.p3 * v3268);
        self.scalar_v9932 = v9932;
        let v10054: f64 = (p.p3 * v2669);
        self.scalar_v10054 = v10054;
        let v10055: f64 = (v2669 * v3268);
        self.scalar_v10055 = v10055;
        let v10060: f64 = (v2673 * v3268);
        self.scalar_v10060 = v10060;
        let v10061: f64 = (p.p3 * v2673);
        self.scalar_v10061 = v10061;
        let v10088: f64 = (p.p3 * v3269);
        self.scalar_v10088 = v10088;
        let v10089: f64 = (p.p3 * v3270);
        self.scalar_v10089 = v10089;
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
        let v720: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v720 = v720;
        let v722: f64 = f64::powf(self.scalar_v720, self.scalar_v721);
        self.scalar_v722 = v722;
        let v723: f64 = (self.scalar_v719 * self.scalar_v722);
        self.scalar_v723 = v723;
        let v2562: f64 = (self.scalar_v20 / self.scalar_v723);
        self.scalar_v2562 = v2562;
        let v2563: f64 = (self.scalar_v27 * self.scalar_v2562);
        self.scalar_v2563 = v2563;
        let v2571: f64 = (self.scalar_v723 * self.scalar_v2549);
        self.scalar_v2571 = v2571;
        let v2572: f64 = (self.scalar_v20 / self.scalar_v2571);
        self.scalar_v2572 = v2572;
        let v2573: f64 = (self.scalar_v27 * self.scalar_v2572);
        self.scalar_v2573 = v2573;
        let v9589: f64 = (1.0 / self.scalar_v723);
        self.scalar_v9589 = v9589;
        let v9590: f64 = (self.scalar_v27 * self.scalar_v9589);
        self.scalar_v9590 = v9590;
        let v9591: f64 = (if self.scalar_v2553 { self.scalar_v9590 } else { 0.0 });
        self.scalar_v9591 = v9591;
        let v9592: f64 = (1.0 / self.scalar_v20);
        self.scalar_v9592 = v9592;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
