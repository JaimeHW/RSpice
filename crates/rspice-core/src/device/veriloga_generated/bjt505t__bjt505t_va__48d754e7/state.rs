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
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v358: bool,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: bool,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v389: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v485: bool,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v530: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v572: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v593: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v618: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v646: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v651: bool,
    pub(crate) scalar_v657: bool,
    pub(crate) scalar_v659: bool,
    pub(crate) scalar_v665: bool,
    pub(crate) scalar_v667: bool,
    pub(crate) scalar_v673: bool,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v726: f64,
    pub(crate) scalar_v844: f64,
    pub(crate) scalar_v897: f64,
    pub(crate) scalar_v898: f64,
    pub(crate) scalar_v899: f64,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v931: f64,
    pub(crate) scalar_v932: f64,
    pub(crate) scalar_v933: f64,
    pub(crate) scalar_v934: f64,
    pub(crate) scalar_v935: f64,
    pub(crate) scalar_v936: f64,
    pub(crate) scalar_v983: f64,
    pub(crate) scalar_v993: f64,
    pub(crate) scalar_v1006: f64,
    pub(crate) scalar_v1007: bool,
    pub(crate) scalar_v1011: bool,
    pub(crate) scalar_v1060: f64,
    pub(crate) scalar_v1061: f64,
    pub(crate) scalar_v1062: f64,
    pub(crate) scalar_v1084: f64,
    pub(crate) scalar_v1092: f64,
    pub(crate) scalar_v1093: bool,
    pub(crate) scalar_v1095: bool,
    pub(crate) scalar_v1096: bool,
    pub(crate) scalar_v1097: bool,
    pub(crate) scalar_v1100: bool,
    pub(crate) scalar_v1101: bool,
    pub(crate) scalar_v1106: f64,
    pub(crate) scalar_v1127: f64,
    pub(crate) scalar_v1129: f64,
    pub(crate) scalar_v1158: bool,
    pub(crate) scalar_v1164: bool,
    pub(crate) scalar_v1198: f64,
    pub(crate) scalar_v1220: f64,
    pub(crate) scalar_v1233: f64,
    pub(crate) scalar_v1251: f64,
    pub(crate) scalar_v1314: f64,
    pub(crate) scalar_v1315: bool,
    pub(crate) scalar_v1316: bool,
    pub(crate) scalar_v1317: bool,
    pub(crate) scalar_v1319: bool,
    pub(crate) scalar_v1320: bool,
    pub(crate) scalar_v1321: f64,
    pub(crate) scalar_v1414: bool,
    pub(crate) scalar_v1415: bool,
    pub(crate) scalar_v1416: bool,
    pub(crate) scalar_v1440: f64,
    pub(crate) scalar_v1442: f64,
    pub(crate) scalar_v1443: f64,
    pub(crate) scalar_v1445: f64,
    pub(crate) scalar_v1505: bool,
    pub(crate) scalar_v1506: bool,
    pub(crate) scalar_v1507: bool,
    pub(crate) scalar_v1533: f64,
    pub(crate) scalar_v1535: f64,
    pub(crate) scalar_v1536: f64,
    pub(crate) scalar_v1538: f64,
    pub(crate) scalar_v1604: f64,
    pub(crate) scalar_v1605: bool,
    pub(crate) scalar_v1606: f64,
    pub(crate) scalar_v1607: f64,
    pub(crate) scalar_v1613: f64,
    pub(crate) scalar_v1622: f64,
    pub(crate) scalar_v1623: f64,
    pub(crate) scalar_v1635: bool,
    pub(crate) scalar_v1654: f64,
    pub(crate) scalar_v1664: f64,
    pub(crate) scalar_v1665: bool,
    pub(crate) scalar_v1666: bool,
    pub(crate) scalar_v1667: bool,
    pub(crate) scalar_v1672: f64,
    pub(crate) scalar_v1682: bool,
    pub(crate) scalar_v1683: f64,
    pub(crate) scalar_v1684: f64,
    pub(crate) scalar_v1698: bool,
    pub(crate) scalar_v1706: bool,
    pub(crate) scalar_v1707: bool,
    pub(crate) scalar_v1720: f64,
    pub(crate) scalar_v1725: f64,
    pub(crate) scalar_v1742: bool,
    pub(crate) scalar_v1743: bool,
    pub(crate) scalar_v1749: f64,
    pub(crate) scalar_v1750: bool,
    pub(crate) scalar_v1753: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1770: f64,
    pub(crate) scalar_v1771: f64,
    pub(crate) scalar_v1772: f64,
    pub(crate) scalar_v1773: f64,
    pub(crate) scalar_v1774: f64,
    pub(crate) scalar_v1775: f64,
    pub(crate) scalar_v1776: f64,
    pub(crate) scalar_v1777: f64,
    pub(crate) scalar_v1778: f64,
    pub(crate) scalar_v1779: f64,
    pub(crate) scalar_v1780: f64,
    pub(crate) scalar_v1781: f64,
    pub(crate) scalar_v1782: f64,
    pub(crate) scalar_v1783: f64,
    pub(crate) scalar_v1784: f64,
    pub(crate) scalar_v1798: bool,
    pub(crate) scalar_v1825: f64,
    pub(crate) scalar_v1826: bool,
    pub(crate) scalar_v1827: f64,
    pub(crate) scalar_v1830: f64,
    pub(crate) scalar_v1849: f64,
    pub(crate) scalar_v1863: f64,
    pub(crate) scalar_v1868: bool,
    pub(crate) scalar_v1870: bool,
    pub(crate) scalar_v1874: f64,
    pub(crate) scalar_v1875: f64,
    pub(crate) scalar_v1876: f64,
    pub(crate) scalar_v1877: f64,
    pub(crate) scalar_v1878: f64,
    pub(crate) scalar_v1887: f64,
    pub(crate) scalar_v1888: bool,
    pub(crate) scalar_v1891: bool,
    pub(crate) scalar_v1914: f64,
    pub(crate) scalar_v1915: f64,
    pub(crate) scalar_v1921: f64,
    pub(crate) scalar_v1922: f64,
    pub(crate) scalar_v1923: f64,
    pub(crate) scalar_v1971: bool,
    pub(crate) scalar_v1972: bool,
    pub(crate) scalar_v1977: f64,
    pub(crate) scalar_v1981: f64,
    pub(crate) scalar_v1988: f64,
    pub(crate) scalar_v1993: f64,
    pub(crate) scalar_v2013: f64,
    pub(crate) scalar_v2033: f64,
    pub(crate) scalar_v2034: bool,
    pub(crate) scalar_v2069: bool,
    pub(crate) scalar_v2075: bool,
    pub(crate) scalar_v2142: f64,
    pub(crate) scalar_v2143: bool,
    pub(crate) scalar_v2144: f64,
    pub(crate) scalar_v2145: bool,
    pub(crate) scalar_v2146: bool,
    pub(crate) scalar_v2150: f64,
    pub(crate) scalar_v2151: bool,
    pub(crate) scalar_v2152: bool,
    pub(crate) scalar_v2153: bool,
    pub(crate) scalar_v2154: bool,
    pub(crate) scalar_v2162: bool,
    pub(crate) scalar_v2163: bool,
    pub(crate) scalar_v2171: bool,
    pub(crate) scalar_v2176: f64,
    pub(crate) scalar_v2177: bool,
    pub(crate) scalar_v2181: bool,
    pub(crate) scalar_v2230: f64,
    pub(crate) scalar_v2235: f64,
    pub(crate) scalar_v2736: f64,
    pub(crate) scalar_v2737: f64,
    pub(crate) scalar_v2738: f64,
    pub(crate) scalar_v2739: f64,
    pub(crate) scalar_v3729: f64,
    pub(crate) scalar_v3753: f64,
    pub(crate) scalar_v3754: f64,
    pub(crate) scalar_v3771: f64,
    pub(crate) scalar_v3857: f64,
    pub(crate) scalar_v3876: f64,
    pub(crate) scalar_v4219: f64,
    pub(crate) scalar_v4220: f64,
    pub(crate) scalar_v4229: f64,
    pub(crate) scalar_v4230: f64,
    pub(crate) scalar_v4254: f64,
    pub(crate) scalar_v4255: f64,
    pub(crate) scalar_v4266: f64,
    pub(crate) scalar_v4267: f64,
    pub(crate) scalar_v4732: f64,
    pub(crate) scalar_v4789: f64,
    pub(crate) scalar_v4790: f64,
    pub(crate) scalar_v4853: f64,
    pub(crate) scalar_v4854: f64,
    pub(crate) scalar_v4987: f64,
    pub(crate) scalar_v5044: f64,
    pub(crate) scalar_v5045: f64,
    pub(crate) scalar_v5466: f64,
    pub(crate) scalar_v5467: f64,
    pub(crate) scalar_v5678: f64,
    pub(crate) scalar_v5679: f64,
    pub(crate) scalar_v5681: f64,
    pub(crate) scalar_v5682: f64,
    pub(crate) scalar_v5936: f64,
    pub(crate) scalar_v5937: f64,
    pub(crate) scalar_v5938: f64,
    pub(crate) scalar_v5939: f64,
    pub(crate) scalar_v5940: f64,
    pub(crate) scalar_v5941: f64,
    pub(crate) scalar_v6369: f64,
    pub(crate) scalar_v6976: f64,
    pub(crate) scalar_v7073: f64,
    pub(crate) scalar_v7386: f64,
    pub(crate) scalar_v7387: f64,
    pub(crate) scalar_v7388: f64,
    pub(crate) scalar_v7389: f64,
    pub(crate) scalar_v7390: f64,
    pub(crate) scalar_v7521: f64,
    pub(crate) scalar_v7522: f64,
    pub(crate) scalar_v7608: f64,
    pub(crate) scalar_v7609: f64,
    pub(crate) scalar_v7677: f64,
    pub(crate) scalar_v7767: f64,
    pub(crate) scalar_v7773: f64,
    pub(crate) scalar_v7949: f64,
    pub(crate) scalar_v7950: f64,
    pub(crate) scalar_v8013: f64,
    pub(crate) scalar_v8014: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v647: f64,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v2155: f64,
    pub(crate) scalar_v2156: f64,
    pub(crate) scalar_v2164: f64,
    pub(crate) scalar_v2165: f64,
    pub(crate) scalar_v2166: f64,
    pub(crate) scalar_v7760: f64,
    pub(crate) scalar_v7761: f64,
    pub(crate) scalar_v7762: f64,
    pub(crate) scalar_v7763: f64,
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
            scalar_v112: self.scalar_v112,
            scalar_v125: self.scalar_v125,
            scalar_v127: self.scalar_v127,
            scalar_v182: self.scalar_v182,
            scalar_v202: self.scalar_v202,
            scalar_v205: self.scalar_v205,
            scalar_v245: self.scalar_v245,
            scalar_v248: self.scalar_v248,
            scalar_v268: self.scalar_v268,
            scalar_v271: self.scalar_v271,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v321: self.scalar_v321,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v358: self.scalar_v358,
            scalar_v360: self.scalar_v360,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v389: self.scalar_v389,
            scalar_v391: self.scalar_v391,
            scalar_v392: self.scalar_v392,
            scalar_v410: self.scalar_v410,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v420: self.scalar_v420,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v436: self.scalar_v436,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v457: self.scalar_v457,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v466: self.scalar_v466,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v484: self.scalar_v484,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v501: self.scalar_v501,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v511: self.scalar_v511,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v520: self.scalar_v520,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v523: self.scalar_v523,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v532: self.scalar_v532,
            scalar_v539: self.scalar_v539,
            scalar_v542: self.scalar_v542,
            scalar_v550: self.scalar_v550,
            scalar_v559: self.scalar_v559,
            scalar_v572: self.scalar_v572,
            scalar_v581: self.scalar_v581,
            scalar_v593: self.scalar_v593,
            scalar_v596: self.scalar_v596,
            scalar_v599: self.scalar_v599,
            scalar_v600: self.scalar_v600,
            scalar_v601: self.scalar_v601,
            scalar_v605: self.scalar_v605,
            scalar_v610: self.scalar_v610,
            scalar_v611: self.scalar_v611,
            scalar_v612: self.scalar_v612,
            scalar_v617: self.scalar_v617,
            scalar_v618: self.scalar_v618,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v642: self.scalar_v642,
            scalar_v644: self.scalar_v644,
            scalar_v646: self.scalar_v646,
            scalar_v648: self.scalar_v648,
            scalar_v651: self.scalar_v651,
            scalar_v657: self.scalar_v657,
            scalar_v659: self.scalar_v659,
            scalar_v665: self.scalar_v665,
            scalar_v667: self.scalar_v667,
            scalar_v673: self.scalar_v673,
            scalar_v721: self.scalar_v721,
            scalar_v726: self.scalar_v726,
            scalar_v844: self.scalar_v844,
            scalar_v897: self.scalar_v897,
            scalar_v898: self.scalar_v898,
            scalar_v899: self.scalar_v899,
            scalar_v910: self.scalar_v910,
            scalar_v931: self.scalar_v931,
            scalar_v932: self.scalar_v932,
            scalar_v933: self.scalar_v933,
            scalar_v934: self.scalar_v934,
            scalar_v935: self.scalar_v935,
            scalar_v936: self.scalar_v936,
            scalar_v983: self.scalar_v983,
            scalar_v993: self.scalar_v993,
            scalar_v1006: self.scalar_v1006,
            scalar_v1007: self.scalar_v1007,
            scalar_v1011: self.scalar_v1011,
            scalar_v1060: self.scalar_v1060,
            scalar_v1061: self.scalar_v1061,
            scalar_v1062: self.scalar_v1062,
            scalar_v1084: self.scalar_v1084,
            scalar_v1092: self.scalar_v1092,
            scalar_v1093: self.scalar_v1093,
            scalar_v1095: self.scalar_v1095,
            scalar_v1096: self.scalar_v1096,
            scalar_v1097: self.scalar_v1097,
            scalar_v1100: self.scalar_v1100,
            scalar_v1101: self.scalar_v1101,
            scalar_v1106: self.scalar_v1106,
            scalar_v1127: self.scalar_v1127,
            scalar_v1129: self.scalar_v1129,
            scalar_v1158: self.scalar_v1158,
            scalar_v1164: self.scalar_v1164,
            scalar_v1198: self.scalar_v1198,
            scalar_v1220: self.scalar_v1220,
            scalar_v1233: self.scalar_v1233,
            scalar_v1251: self.scalar_v1251,
            scalar_v1314: self.scalar_v1314,
            scalar_v1315: self.scalar_v1315,
            scalar_v1316: self.scalar_v1316,
            scalar_v1317: self.scalar_v1317,
            scalar_v1319: self.scalar_v1319,
            scalar_v1320: self.scalar_v1320,
            scalar_v1321: self.scalar_v1321,
            scalar_v1414: self.scalar_v1414,
            scalar_v1415: self.scalar_v1415,
            scalar_v1416: self.scalar_v1416,
            scalar_v1440: self.scalar_v1440,
            scalar_v1442: self.scalar_v1442,
            scalar_v1443: self.scalar_v1443,
            scalar_v1445: self.scalar_v1445,
            scalar_v1505: self.scalar_v1505,
            scalar_v1506: self.scalar_v1506,
            scalar_v1507: self.scalar_v1507,
            scalar_v1533: self.scalar_v1533,
            scalar_v1535: self.scalar_v1535,
            scalar_v1536: self.scalar_v1536,
            scalar_v1538: self.scalar_v1538,
            scalar_v1604: self.scalar_v1604,
            scalar_v1605: self.scalar_v1605,
            scalar_v1606: self.scalar_v1606,
            scalar_v1607: self.scalar_v1607,
            scalar_v1613: self.scalar_v1613,
            scalar_v1622: self.scalar_v1622,
            scalar_v1623: self.scalar_v1623,
            scalar_v1635: self.scalar_v1635,
            scalar_v1654: self.scalar_v1654,
            scalar_v1664: self.scalar_v1664,
            scalar_v1665: self.scalar_v1665,
            scalar_v1666: self.scalar_v1666,
            scalar_v1667: self.scalar_v1667,
            scalar_v1672: self.scalar_v1672,
            scalar_v1682: self.scalar_v1682,
            scalar_v1683: self.scalar_v1683,
            scalar_v1684: self.scalar_v1684,
            scalar_v1698: self.scalar_v1698,
            scalar_v1706: self.scalar_v1706,
            scalar_v1707: self.scalar_v1707,
            scalar_v1720: self.scalar_v1720,
            scalar_v1725: self.scalar_v1725,
            scalar_v1742: self.scalar_v1742,
            scalar_v1743: self.scalar_v1743,
            scalar_v1749: self.scalar_v1749,
            scalar_v1750: self.scalar_v1750,
            scalar_v1753: self.scalar_v1753,
            scalar_v1759: self.scalar_v1759,
            scalar_v1770: self.scalar_v1770,
            scalar_v1771: self.scalar_v1771,
            scalar_v1772: self.scalar_v1772,
            scalar_v1773: self.scalar_v1773,
            scalar_v1774: self.scalar_v1774,
            scalar_v1775: self.scalar_v1775,
            scalar_v1776: self.scalar_v1776,
            scalar_v1777: self.scalar_v1777,
            scalar_v1778: self.scalar_v1778,
            scalar_v1779: self.scalar_v1779,
            scalar_v1780: self.scalar_v1780,
            scalar_v1781: self.scalar_v1781,
            scalar_v1782: self.scalar_v1782,
            scalar_v1783: self.scalar_v1783,
            scalar_v1784: self.scalar_v1784,
            scalar_v1798: self.scalar_v1798,
            scalar_v1825: self.scalar_v1825,
            scalar_v1826: self.scalar_v1826,
            scalar_v1827: self.scalar_v1827,
            scalar_v1830: self.scalar_v1830,
            scalar_v1849: self.scalar_v1849,
            scalar_v1863: self.scalar_v1863,
            scalar_v1868: self.scalar_v1868,
            scalar_v1870: self.scalar_v1870,
            scalar_v1874: self.scalar_v1874,
            scalar_v1875: self.scalar_v1875,
            scalar_v1876: self.scalar_v1876,
            scalar_v1877: self.scalar_v1877,
            scalar_v1878: self.scalar_v1878,
            scalar_v1887: self.scalar_v1887,
            scalar_v1888: self.scalar_v1888,
            scalar_v1891: self.scalar_v1891,
            scalar_v1914: self.scalar_v1914,
            scalar_v1915: self.scalar_v1915,
            scalar_v1921: self.scalar_v1921,
            scalar_v1922: self.scalar_v1922,
            scalar_v1923: self.scalar_v1923,
            scalar_v1971: self.scalar_v1971,
            scalar_v1972: self.scalar_v1972,
            scalar_v1977: self.scalar_v1977,
            scalar_v1981: self.scalar_v1981,
            scalar_v1988: self.scalar_v1988,
            scalar_v1993: self.scalar_v1993,
            scalar_v2013: self.scalar_v2013,
            scalar_v2033: self.scalar_v2033,
            scalar_v2034: self.scalar_v2034,
            scalar_v2069: self.scalar_v2069,
            scalar_v2075: self.scalar_v2075,
            scalar_v2142: self.scalar_v2142,
            scalar_v2143: self.scalar_v2143,
            scalar_v2144: self.scalar_v2144,
            scalar_v2145: self.scalar_v2145,
            scalar_v2146: self.scalar_v2146,
            scalar_v2150: self.scalar_v2150,
            scalar_v2151: self.scalar_v2151,
            scalar_v2152: self.scalar_v2152,
            scalar_v2153: self.scalar_v2153,
            scalar_v2154: self.scalar_v2154,
            scalar_v2162: self.scalar_v2162,
            scalar_v2163: self.scalar_v2163,
            scalar_v2171: self.scalar_v2171,
            scalar_v2176: self.scalar_v2176,
            scalar_v2177: self.scalar_v2177,
            scalar_v2181: self.scalar_v2181,
            scalar_v2230: self.scalar_v2230,
            scalar_v2235: self.scalar_v2235,
            scalar_v2736: self.scalar_v2736,
            scalar_v2737: self.scalar_v2737,
            scalar_v2738: self.scalar_v2738,
            scalar_v2739: self.scalar_v2739,
            scalar_v3729: self.scalar_v3729,
            scalar_v3753: self.scalar_v3753,
            scalar_v3754: self.scalar_v3754,
            scalar_v3771: self.scalar_v3771,
            scalar_v3857: self.scalar_v3857,
            scalar_v3876: self.scalar_v3876,
            scalar_v4219: self.scalar_v4219,
            scalar_v4220: self.scalar_v4220,
            scalar_v4229: self.scalar_v4229,
            scalar_v4230: self.scalar_v4230,
            scalar_v4254: self.scalar_v4254,
            scalar_v4255: self.scalar_v4255,
            scalar_v4266: self.scalar_v4266,
            scalar_v4267: self.scalar_v4267,
            scalar_v4732: self.scalar_v4732,
            scalar_v4789: self.scalar_v4789,
            scalar_v4790: self.scalar_v4790,
            scalar_v4853: self.scalar_v4853,
            scalar_v4854: self.scalar_v4854,
            scalar_v4987: self.scalar_v4987,
            scalar_v5044: self.scalar_v5044,
            scalar_v5045: self.scalar_v5045,
            scalar_v5466: self.scalar_v5466,
            scalar_v5467: self.scalar_v5467,
            scalar_v5678: self.scalar_v5678,
            scalar_v5679: self.scalar_v5679,
            scalar_v5681: self.scalar_v5681,
            scalar_v5682: self.scalar_v5682,
            scalar_v5936: self.scalar_v5936,
            scalar_v5937: self.scalar_v5937,
            scalar_v5938: self.scalar_v5938,
            scalar_v5939: self.scalar_v5939,
            scalar_v5940: self.scalar_v5940,
            scalar_v5941: self.scalar_v5941,
            scalar_v6369: self.scalar_v6369,
            scalar_v6976: self.scalar_v6976,
            scalar_v7073: self.scalar_v7073,
            scalar_v7386: self.scalar_v7386,
            scalar_v7387: self.scalar_v7387,
            scalar_v7388: self.scalar_v7388,
            scalar_v7389: self.scalar_v7389,
            scalar_v7390: self.scalar_v7390,
            scalar_v7521: self.scalar_v7521,
            scalar_v7522: self.scalar_v7522,
            scalar_v7608: self.scalar_v7608,
            scalar_v7609: self.scalar_v7609,
            scalar_v7677: self.scalar_v7677,
            scalar_v7767: self.scalar_v7767,
            scalar_v7773: self.scalar_v7773,
            scalar_v7949: self.scalar_v7949,
            scalar_v7950: self.scalar_v7950,
            scalar_v8013: self.scalar_v8013,
            scalar_v8014: self.scalar_v8014,
            scalar_v20: self.scalar_v20,
            scalar_v647: self.scalar_v647,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v2155: self.scalar_v2155,
            scalar_v2156: self.scalar_v2156,
            scalar_v2164: self.scalar_v2164,
            scalar_v2165: self.scalar_v2165,
            scalar_v2166: self.scalar_v2166,
            scalar_v7760: self.scalar_v7760,
            scalar_v7761: self.scalar_v7761,
            scalar_v7762: self.scalar_v7762,
            scalar_v7763: self.scalar_v7763,
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
            scalar_v112: 0.0,
            scalar_v125: 0.0,
            scalar_v127: 0.0,
            scalar_v182: 0.0,
            scalar_v202: 0.0,
            scalar_v205: 0.0,
            scalar_v245: 0.0,
            scalar_v248: 0.0,
            scalar_v268: 0.0,
            scalar_v271: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v321: 0.0,
            scalar_v323: 0.0,
            scalar_v324: 0.0,
            scalar_v328: 0.0,
            scalar_v329: false,
            scalar_v330: 0.0,
            scalar_v358: false,
            scalar_v360: 0.0,
            scalar_v361: false,
            scalar_v362: 0.0,
            scalar_v389: false,
            scalar_v391: 0.0,
            scalar_v392: 0.0,
            scalar_v410: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v420: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v436: 0.0,
            scalar_v438: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v444: 0.0,
            scalar_v445: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v457: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v466: 0.0,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v484: 0.0,
            scalar_v485: false,
            scalar_v486: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v501: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v511: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v520: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v523: 0.0,
            scalar_v530: 0.0,
            scalar_v531: 0.0,
            scalar_v532: 0.0,
            scalar_v539: 0.0,
            scalar_v542: 0.0,
            scalar_v550: 0.0,
            scalar_v559: 0.0,
            scalar_v572: 0.0,
            scalar_v581: 0.0,
            scalar_v593: 0.0,
            scalar_v596: 0.0,
            scalar_v599: 0.0,
            scalar_v600: 0.0,
            scalar_v601: 0.0,
            scalar_v605: 0.0,
            scalar_v610: 0.0,
            scalar_v611: 0.0,
            scalar_v612: 0.0,
            scalar_v617: 0.0,
            scalar_v618: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v642: 0.0,
            scalar_v644: 0.0,
            scalar_v646: 0.0,
            scalar_v648: 0.0,
            scalar_v651: false,
            scalar_v657: false,
            scalar_v659: false,
            scalar_v665: false,
            scalar_v667: false,
            scalar_v673: false,
            scalar_v721: 0.0,
            scalar_v726: 0.0,
            scalar_v844: 0.0,
            scalar_v897: 0.0,
            scalar_v898: 0.0,
            scalar_v899: 0.0,
            scalar_v910: 0.0,
            scalar_v931: 0.0,
            scalar_v932: 0.0,
            scalar_v933: 0.0,
            scalar_v934: 0.0,
            scalar_v935: 0.0,
            scalar_v936: 0.0,
            scalar_v983: 0.0,
            scalar_v993: 0.0,
            scalar_v1006: 0.0,
            scalar_v1007: false,
            scalar_v1011: false,
            scalar_v1060: 0.0,
            scalar_v1061: 0.0,
            scalar_v1062: 0.0,
            scalar_v1084: 0.0,
            scalar_v1092: 0.0,
            scalar_v1093: false,
            scalar_v1095: false,
            scalar_v1096: false,
            scalar_v1097: false,
            scalar_v1100: false,
            scalar_v1101: false,
            scalar_v1106: 0.0,
            scalar_v1127: 0.0,
            scalar_v1129: 0.0,
            scalar_v1158: false,
            scalar_v1164: false,
            scalar_v1198: 0.0,
            scalar_v1220: 0.0,
            scalar_v1233: 0.0,
            scalar_v1251: 0.0,
            scalar_v1314: 0.0,
            scalar_v1315: false,
            scalar_v1316: false,
            scalar_v1317: false,
            scalar_v1319: false,
            scalar_v1320: false,
            scalar_v1321: 0.0,
            scalar_v1414: false,
            scalar_v1415: false,
            scalar_v1416: false,
            scalar_v1440: 0.0,
            scalar_v1442: 0.0,
            scalar_v1443: 0.0,
            scalar_v1445: 0.0,
            scalar_v1505: false,
            scalar_v1506: false,
            scalar_v1507: false,
            scalar_v1533: 0.0,
            scalar_v1535: 0.0,
            scalar_v1536: 0.0,
            scalar_v1538: 0.0,
            scalar_v1604: 0.0,
            scalar_v1605: false,
            scalar_v1606: 0.0,
            scalar_v1607: 0.0,
            scalar_v1613: 0.0,
            scalar_v1622: 0.0,
            scalar_v1623: 0.0,
            scalar_v1635: false,
            scalar_v1654: 0.0,
            scalar_v1664: 0.0,
            scalar_v1665: false,
            scalar_v1666: false,
            scalar_v1667: false,
            scalar_v1672: 0.0,
            scalar_v1682: false,
            scalar_v1683: 0.0,
            scalar_v1684: 0.0,
            scalar_v1698: false,
            scalar_v1706: false,
            scalar_v1707: false,
            scalar_v1720: 0.0,
            scalar_v1725: 0.0,
            scalar_v1742: false,
            scalar_v1743: false,
            scalar_v1749: 0.0,
            scalar_v1750: false,
            scalar_v1753: 0.0,
            scalar_v1759: 0.0,
            scalar_v1770: 0.0,
            scalar_v1771: 0.0,
            scalar_v1772: 0.0,
            scalar_v1773: 0.0,
            scalar_v1774: 0.0,
            scalar_v1775: 0.0,
            scalar_v1776: 0.0,
            scalar_v1777: 0.0,
            scalar_v1778: 0.0,
            scalar_v1779: 0.0,
            scalar_v1780: 0.0,
            scalar_v1781: 0.0,
            scalar_v1782: 0.0,
            scalar_v1783: 0.0,
            scalar_v1784: 0.0,
            scalar_v1798: false,
            scalar_v1825: 0.0,
            scalar_v1826: false,
            scalar_v1827: 0.0,
            scalar_v1830: 0.0,
            scalar_v1849: 0.0,
            scalar_v1863: 0.0,
            scalar_v1868: false,
            scalar_v1870: false,
            scalar_v1874: 0.0,
            scalar_v1875: 0.0,
            scalar_v1876: 0.0,
            scalar_v1877: 0.0,
            scalar_v1878: 0.0,
            scalar_v1887: 0.0,
            scalar_v1888: false,
            scalar_v1891: false,
            scalar_v1914: 0.0,
            scalar_v1915: 0.0,
            scalar_v1921: 0.0,
            scalar_v1922: 0.0,
            scalar_v1923: 0.0,
            scalar_v1971: false,
            scalar_v1972: false,
            scalar_v1977: 0.0,
            scalar_v1981: 0.0,
            scalar_v1988: 0.0,
            scalar_v1993: 0.0,
            scalar_v2013: 0.0,
            scalar_v2033: 0.0,
            scalar_v2034: false,
            scalar_v2069: false,
            scalar_v2075: false,
            scalar_v2142: 0.0,
            scalar_v2143: false,
            scalar_v2144: 0.0,
            scalar_v2145: false,
            scalar_v2146: false,
            scalar_v2150: 0.0,
            scalar_v2151: false,
            scalar_v2152: false,
            scalar_v2153: false,
            scalar_v2154: false,
            scalar_v2162: false,
            scalar_v2163: false,
            scalar_v2171: false,
            scalar_v2176: 0.0,
            scalar_v2177: false,
            scalar_v2181: false,
            scalar_v2230: 0.0,
            scalar_v2235: 0.0,
            scalar_v2736: 0.0,
            scalar_v2737: 0.0,
            scalar_v2738: 0.0,
            scalar_v2739: 0.0,
            scalar_v3729: 0.0,
            scalar_v3753: 0.0,
            scalar_v3754: 0.0,
            scalar_v3771: 0.0,
            scalar_v3857: 0.0,
            scalar_v3876: 0.0,
            scalar_v4219: 0.0,
            scalar_v4220: 0.0,
            scalar_v4229: 0.0,
            scalar_v4230: 0.0,
            scalar_v4254: 0.0,
            scalar_v4255: 0.0,
            scalar_v4266: 0.0,
            scalar_v4267: 0.0,
            scalar_v4732: 0.0,
            scalar_v4789: 0.0,
            scalar_v4790: 0.0,
            scalar_v4853: 0.0,
            scalar_v4854: 0.0,
            scalar_v4987: 0.0,
            scalar_v5044: 0.0,
            scalar_v5045: 0.0,
            scalar_v5466: 0.0,
            scalar_v5467: 0.0,
            scalar_v5678: 0.0,
            scalar_v5679: 0.0,
            scalar_v5681: 0.0,
            scalar_v5682: 0.0,
            scalar_v5936: 0.0,
            scalar_v5937: 0.0,
            scalar_v5938: 0.0,
            scalar_v5939: 0.0,
            scalar_v5940: 0.0,
            scalar_v5941: 0.0,
            scalar_v6369: 0.0,
            scalar_v6976: 0.0,
            scalar_v7073: 0.0,
            scalar_v7386: 0.0,
            scalar_v7387: 0.0,
            scalar_v7388: 0.0,
            scalar_v7389: 0.0,
            scalar_v7390: 0.0,
            scalar_v7521: 0.0,
            scalar_v7522: 0.0,
            scalar_v7608: 0.0,
            scalar_v7609: 0.0,
            scalar_v7677: 0.0,
            scalar_v7767: 0.0,
            scalar_v7773: 0.0,
            scalar_v7949: 0.0,
            scalar_v7950: 0.0,
            scalar_v8013: 0.0,
            scalar_v8014: 0.0,
            scalar_v20: 0.0,
            scalar_v647: 0.0,
            scalar_v649: 0.0,
            scalar_v650: 0.0,
            scalar_v2155: 0.0,
            scalar_v2156: 0.0,
            scalar_v2164: 0.0,
            scalar_v2165: 0.0,
            scalar_v2166: 0.0,
            scalar_v7760: 0.0,
            scalar_v7761: 0.0,
            scalar_v7762: 0.0,
            scalar_v7763: 0.0,
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
            scalar_v112,
            scalar_v125,
            scalar_v127,
            scalar_v182,
            scalar_v202,
            scalar_v205,
            scalar_v245,
            scalar_v248,
            scalar_v268,
            scalar_v271,
            scalar_v282,
            scalar_v283,
            scalar_v290,
            scalar_v291,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v304,
            scalar_v305,
            scalar_v311,
            scalar_v312,
            scalar_v316,
            scalar_v317,
            scalar_v321,
            scalar_v323,
            scalar_v324,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v358,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v389,
            scalar_v391,
            scalar_v392,
            scalar_v410,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v420,
            scalar_v425,
            scalar_v426,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v436,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v445,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v457,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v466,
            scalar_v470,
            scalar_v471,
            scalar_v476,
            scalar_v477,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v515,
            scalar_v516,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v539,
            scalar_v542,
            scalar_v550,
            scalar_v559,
            scalar_v572,
            scalar_v581,
            scalar_v593,
            scalar_v596,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v605,
            scalar_v610,
            scalar_v611,
            scalar_v612,
            scalar_v617,
            scalar_v618,
            scalar_v622,
            scalar_v623,
            scalar_v642,
            scalar_v644,
            scalar_v646,
            scalar_v648,
            scalar_v651,
            scalar_v657,
            scalar_v659,
            scalar_v665,
            scalar_v667,
            scalar_v673,
            scalar_v721,
            scalar_v726,
            scalar_v844,
            scalar_v897,
            scalar_v898,
            scalar_v899,
            scalar_v910,
            scalar_v931,
            scalar_v932,
            scalar_v933,
            scalar_v934,
            scalar_v935,
            scalar_v936,
            scalar_v983,
            scalar_v993,
            scalar_v1006,
            scalar_v1007,
            scalar_v1011,
            scalar_v1060,
            scalar_v1061,
            scalar_v1062,
            scalar_v1084,
            scalar_v1092,
            scalar_v1093,
            scalar_v1095,
            scalar_v1096,
            scalar_v1097,
            scalar_v1100,
            scalar_v1101,
            scalar_v1106,
            scalar_v1127,
            scalar_v1129,
            scalar_v1158,
            scalar_v1164,
            scalar_v1198,
            scalar_v1220,
            scalar_v1233,
            scalar_v1251,
            scalar_v1314,
            scalar_v1315,
            scalar_v1316,
            scalar_v1317,
            scalar_v1319,
            scalar_v1320,
            scalar_v1321,
            scalar_v1414,
            scalar_v1415,
            scalar_v1416,
            scalar_v1440,
            scalar_v1442,
            scalar_v1443,
            scalar_v1445,
            scalar_v1505,
            scalar_v1506,
            scalar_v1507,
            scalar_v1533,
            scalar_v1535,
            scalar_v1536,
            scalar_v1538,
            scalar_v1604,
            scalar_v1605,
            scalar_v1606,
            scalar_v1607,
            scalar_v1613,
            scalar_v1622,
            scalar_v1623,
            scalar_v1635,
            scalar_v1654,
            scalar_v1664,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1672,
            scalar_v1682,
            scalar_v1683,
            scalar_v1684,
            scalar_v1698,
            scalar_v1706,
            scalar_v1707,
            scalar_v1720,
            scalar_v1725,
            scalar_v1742,
            scalar_v1743,
            scalar_v1749,
            scalar_v1750,
            scalar_v1753,
            scalar_v1759,
            scalar_v1770,
            scalar_v1771,
            scalar_v1772,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1779,
            scalar_v1780,
            scalar_v1781,
            scalar_v1782,
            scalar_v1783,
            scalar_v1784,
            scalar_v1798,
            scalar_v1825,
            scalar_v1826,
            scalar_v1827,
            scalar_v1830,
            scalar_v1849,
            scalar_v1863,
            scalar_v1868,
            scalar_v1870,
            scalar_v1874,
            scalar_v1875,
            scalar_v1876,
            scalar_v1877,
            scalar_v1878,
            scalar_v1887,
            scalar_v1888,
            scalar_v1891,
            scalar_v1914,
            scalar_v1915,
            scalar_v1921,
            scalar_v1922,
            scalar_v1923,
            scalar_v1971,
            scalar_v1972,
            scalar_v1977,
            scalar_v1981,
            scalar_v1988,
            scalar_v1993,
            scalar_v2013,
            scalar_v2033,
            scalar_v2034,
            scalar_v2069,
            scalar_v2075,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2145,
            scalar_v2146,
            scalar_v2150,
            scalar_v2151,
            scalar_v2152,
            scalar_v2153,
            scalar_v2154,
            scalar_v2162,
            scalar_v2163,
            scalar_v2171,
            scalar_v2176,
            scalar_v2177,
            scalar_v2181,
            scalar_v2230,
            scalar_v2235,
            scalar_v2736,
            scalar_v2737,
            scalar_v2738,
            scalar_v2739,
            scalar_v3729,
            scalar_v3753,
            scalar_v3754,
            scalar_v3771,
            scalar_v3857,
            scalar_v3876,
            scalar_v4219,
            scalar_v4220,
            scalar_v4229,
            scalar_v4230,
            scalar_v4254,
            scalar_v4255,
            scalar_v4266,
            scalar_v4267,
            scalar_v4732,
            scalar_v4789,
            scalar_v4790,
            scalar_v4853,
            scalar_v4854,
            scalar_v4987,
            scalar_v5044,
            scalar_v5045,
            scalar_v5466,
            scalar_v5467,
            scalar_v5678,
            scalar_v5679,
            scalar_v5681,
            scalar_v5682,
            scalar_v5936,
            scalar_v5937,
            scalar_v5938,
            scalar_v5939,
            scalar_v5940,
            scalar_v5941,
            scalar_v6369,
            scalar_v6976,
            scalar_v7073,
            scalar_v7386,
            scalar_v7387,
            scalar_v7388,
            scalar_v7389,
            scalar_v7390,
            scalar_v7521,
            scalar_v7522,
            scalar_v7608,
            scalar_v7609,
            scalar_v7677,
            scalar_v7767,
            scalar_v7773,
            scalar_v7949,
            scalar_v7950,
            scalar_v8013,
            scalar_v8014,
            scalar_v20,
            scalar_v647,
            scalar_v649,
            scalar_v650,
            scalar_v2155,
            scalar_v2156,
            scalar_v2164,
            scalar_v2165,
            scalar_v2166,
            scalar_v7760,
            scalar_v7761,
            scalar_v7762,
            scalar_v7763,
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
            scalar_v112,
            scalar_v125,
            scalar_v127,
            scalar_v182,
            scalar_v202,
            scalar_v205,
            scalar_v245,
            scalar_v248,
            scalar_v268,
            scalar_v271,
            scalar_v282,
            scalar_v283,
            scalar_v290,
            scalar_v291,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v304,
            scalar_v305,
            scalar_v311,
            scalar_v312,
            scalar_v316,
            scalar_v317,
            scalar_v321,
            scalar_v323,
            scalar_v324,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v358,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v389,
            scalar_v391,
            scalar_v392,
            scalar_v410,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v420,
            scalar_v425,
            scalar_v426,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v436,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v445,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v457,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v466,
            scalar_v470,
            scalar_v471,
            scalar_v476,
            scalar_v477,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v515,
            scalar_v516,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v539,
            scalar_v542,
            scalar_v550,
            scalar_v559,
            scalar_v572,
            scalar_v581,
            scalar_v593,
            scalar_v596,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v605,
            scalar_v610,
            scalar_v611,
            scalar_v612,
            scalar_v617,
            scalar_v618,
            scalar_v622,
            scalar_v623,
            scalar_v642,
            scalar_v644,
            scalar_v646,
            scalar_v648,
            scalar_v651,
            scalar_v657,
            scalar_v659,
            scalar_v665,
            scalar_v667,
            scalar_v673,
            scalar_v721,
            scalar_v726,
            scalar_v844,
            scalar_v897,
            scalar_v898,
            scalar_v899,
            scalar_v910,
            scalar_v931,
            scalar_v932,
            scalar_v933,
            scalar_v934,
            scalar_v935,
            scalar_v936,
            scalar_v983,
            scalar_v993,
            scalar_v1006,
            scalar_v1007,
            scalar_v1011,
            scalar_v1060,
            scalar_v1061,
            scalar_v1062,
            scalar_v1084,
            scalar_v1092,
            scalar_v1093,
            scalar_v1095,
            scalar_v1096,
            scalar_v1097,
            scalar_v1100,
            scalar_v1101,
            scalar_v1106,
            scalar_v1127,
            scalar_v1129,
            scalar_v1158,
            scalar_v1164,
            scalar_v1198,
            scalar_v1220,
            scalar_v1233,
            scalar_v1251,
            scalar_v1314,
            scalar_v1315,
            scalar_v1316,
            scalar_v1317,
            scalar_v1319,
            scalar_v1320,
            scalar_v1321,
            scalar_v1414,
            scalar_v1415,
            scalar_v1416,
            scalar_v1440,
            scalar_v1442,
            scalar_v1443,
            scalar_v1445,
            scalar_v1505,
            scalar_v1506,
            scalar_v1507,
            scalar_v1533,
            scalar_v1535,
            scalar_v1536,
            scalar_v1538,
            scalar_v1604,
            scalar_v1605,
            scalar_v1606,
            scalar_v1607,
            scalar_v1613,
            scalar_v1622,
            scalar_v1623,
            scalar_v1635,
            scalar_v1654,
            scalar_v1664,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1672,
            scalar_v1682,
            scalar_v1683,
            scalar_v1684,
            scalar_v1698,
            scalar_v1706,
            scalar_v1707,
            scalar_v1720,
            scalar_v1725,
            scalar_v1742,
            scalar_v1743,
            scalar_v1749,
            scalar_v1750,
            scalar_v1753,
            scalar_v1759,
            scalar_v1770,
            scalar_v1771,
            scalar_v1772,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1779,
            scalar_v1780,
            scalar_v1781,
            scalar_v1782,
            scalar_v1783,
            scalar_v1784,
            scalar_v1798,
            scalar_v1825,
            scalar_v1826,
            scalar_v1827,
            scalar_v1830,
            scalar_v1849,
            scalar_v1863,
            scalar_v1868,
            scalar_v1870,
            scalar_v1874,
            scalar_v1875,
            scalar_v1876,
            scalar_v1877,
            scalar_v1878,
            scalar_v1887,
            scalar_v1888,
            scalar_v1891,
            scalar_v1914,
            scalar_v1915,
            scalar_v1921,
            scalar_v1922,
            scalar_v1923,
            scalar_v1971,
            scalar_v1972,
            scalar_v1977,
            scalar_v1981,
            scalar_v1988,
            scalar_v1993,
            scalar_v2013,
            scalar_v2033,
            scalar_v2034,
            scalar_v2069,
            scalar_v2075,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2145,
            scalar_v2146,
            scalar_v2150,
            scalar_v2151,
            scalar_v2152,
            scalar_v2153,
            scalar_v2154,
            scalar_v2162,
            scalar_v2163,
            scalar_v2171,
            scalar_v2176,
            scalar_v2177,
            scalar_v2181,
            scalar_v2230,
            scalar_v2235,
            scalar_v2736,
            scalar_v2737,
            scalar_v2738,
            scalar_v2739,
            scalar_v3729,
            scalar_v3753,
            scalar_v3754,
            scalar_v3771,
            scalar_v3857,
            scalar_v3876,
            scalar_v4219,
            scalar_v4220,
            scalar_v4229,
            scalar_v4230,
            scalar_v4254,
            scalar_v4255,
            scalar_v4266,
            scalar_v4267,
            scalar_v4732,
            scalar_v4789,
            scalar_v4790,
            scalar_v4853,
            scalar_v4854,
            scalar_v4987,
            scalar_v5044,
            scalar_v5045,
            scalar_v5466,
            scalar_v5467,
            scalar_v5678,
            scalar_v5679,
            scalar_v5681,
            scalar_v5682,
            scalar_v5936,
            scalar_v5937,
            scalar_v5938,
            scalar_v5939,
            scalar_v5940,
            scalar_v5941,
            scalar_v6369,
            scalar_v6976,
            scalar_v7073,
            scalar_v7386,
            scalar_v7387,
            scalar_v7388,
            scalar_v7389,
            scalar_v7390,
            scalar_v7521,
            scalar_v7522,
            scalar_v7608,
            scalar_v7609,
            scalar_v7677,
            scalar_v7767,
            scalar_v7773,
            scalar_v7949,
            scalar_v7950,
            scalar_v8013,
            scalar_v8014,
            scalar_v20,
            scalar_v647,
            scalar_v649,
            scalar_v650,
            scalar_v2155,
            scalar_v2156,
            scalar_v2164,
            scalar_v2165,
            scalar_v2166,
            scalar_v7760,
            scalar_v7761,
            scalar_v7762,
            scalar_v7763,
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
        let v112: f64 = p.p125;
        self.scalar_v112 = v112;
        let v125: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v125 = v125;
        let v127: f64 = (1.0 / v125);
        self.scalar_v127 = v127;
        let v182: f64 = p.p105;
        self.scalar_v182 = v182;
        let v202: f64 = p.p64;
        self.scalar_v202 = v202;
        let v205: f64 = p.p110;
        self.scalar_v205 = v205;
        let v245: f64 = p.p27;
        self.scalar_v245 = v245;
        let v248: f64 = p.p109;
        self.scalar_v248 = v248;
        let v268: f64 = p.p138;
        self.scalar_v268 = v268;
        let v271: f64 = p.p140;
        self.scalar_v271 = v271;
        let v282: f64 = p.p75;
        self.scalar_v282 = v282;
        let v283: f64 = (1.0 - p.p75);
        self.scalar_v283 = v283;
        let v290: f64 = p.p54;
        self.scalar_v290 = v290;
        let v291: f64 = p.p97;
        self.scalar_v291 = v291;
        let v297: f64 = p.p56;
        self.scalar_v297 = v297;
        let v298: f64 = p.p98;
        self.scalar_v298 = v298;
        let v299: f64 = p.p96;
        self.scalar_v299 = v299;
        let v300: f64 = (p.p98 - p.p96);
        self.scalar_v300 = v300;
        let v304: f64 = p.p55;
        self.scalar_v304 = v304;
        let v305: f64 = p.p101;
        self.scalar_v305 = v305;
        let v311: f64 = p.p57;
        self.scalar_v311 = v311;
        let v312: f64 = p.p102;
        self.scalar_v312 = v312;
        let v316: f64 = p.p58;
        self.scalar_v316 = v316;
        let v317: f64 = p.p104;
        self.scalar_v317 = v317;
        let v321: f64 = p.p59;
        self.scalar_v321 = v321;
        let v323: f64 = p.p60;
        self.scalar_v323 = v323;
        let v324: f64 = p.p99;
        self.scalar_v324 = v324;
        let v328: f64 = p.p122;
        self.scalar_v328 = v328;
        let v329: bool = (0.0 != p.p122);
        self.scalar_v329 = v329;
        let v330: f64 = p.p10;
        self.scalar_v330 = v330;
        let v358: bool = (!v329);
        self.scalar_v358 = v358;
        let v360: f64 = p.p123;
        self.scalar_v360 = v360;
        let v361: bool = (0.0 != p.p123);
        self.scalar_v361 = v361;
        let v362: f64 = p.p11;
        self.scalar_v362 = v362;
        let v389: bool = (!v361);
        self.scalar_v389 = v389;
        let v391: f64 = p.p43;
        self.scalar_v391 = v391;
        let v392: f64 = p.p124;
        self.scalar_v392 = v392;
        let v410: f64 = p.p9;
        self.scalar_v410 = v410;
        let v412: f64 = (4.0 - p.p98);
        self.scalar_v412 = v412;
        let v413: f64 = (v412 - p.p96);
        self.scalar_v413 = v413;
        let v414: f64 = p.p121;
        self.scalar_v414 = v414;
        let v415: f64 = (v413 + p.p121);
        self.scalar_v415 = v415;
        let v420: f64 = (-p.p105);
        self.scalar_v420 = v420;
        let v425: f64 = p.p12;
        self.scalar_v425 = v425;
        let v426: f64 = (1.0 - p.p98);
        self.scalar_v426 = v426;
        let v430: f64 = p.p30;
        self.scalar_v430 = v430;
        let v431: f64 = p.p103;
        self.scalar_v431 = v431;
        let v432: f64 = (1.0 - p.p103);
        self.scalar_v432 = v432;
        let v436: f64 = p.p20;
        self.scalar_v436 = v436;
        let v438: f64 = p.p21;
        self.scalar_v438 = v438;
        let v439: f64 = (2.0 * p.p21);
        self.scalar_v439 = v439;
        let v440: f64 = (6.0 - v439);
        self.scalar_v440 = v440;
        let v444: f64 = p.p113;
        self.scalar_v444 = v444;
        let v445: f64 = (-p.p113);
        self.scalar_v445 = v445;
        let v450: f64 = p.p31;
        self.scalar_v450 = v450;
        let v451: f64 = p.p32;
        self.scalar_v451 = v451;
        let v452: f64 = (2.0 * p.p32);
        self.scalar_v452 = v452;
        let v453: f64 = (6.0 - v452);
        self.scalar_v453 = v453;
        let v457: f64 = (-p.p110);
        self.scalar_v457 = v457;
        let v462: f64 = p.p16;
        self.scalar_v462 = v462;
        let v463: f64 = (4.0 - p.p97);
        self.scalar_v463 = v463;
        let v464: f64 = (p.p121 + v463);
        self.scalar_v464 = v464;
        let v466: f64 = p.p17;
        self.scalar_v466 = v466;
        let v470: f64 = p.p111;
        self.scalar_v470 = v470;
        let v471: f64 = (-p.p111);
        self.scalar_v471 = v471;
        let v476: f64 = p.p18;
        self.scalar_v476 = v476;
        let v477: f64 = p.p19;
        self.scalar_v477 = v477;
        let v484: f64 = p.p24;
        self.scalar_v484 = v484;
        let v485: bool = (1.0 == p.p24);
        self.scalar_v485 = v485;
        let v486: f64 = p.p25;
        self.scalar_v486 = v486;
        let v487: f64 = p.p107;
        self.scalar_v487 = v487;
        let v488: f64 = (-p.p107);
        self.scalar_v488 = v488;
        let v494: f64 = p.p28;
        self.scalar_v494 = v494;
        let v495: f64 = p.p106;
        self.scalar_v495 = v495;
        let v496: f64 = (-p.p106);
        self.scalar_v496 = v496;
        let v501: f64 = p.p26;
        self.scalar_v501 = v501;
        let v502: f64 = p.p108;
        self.scalar_v502 = v502;
        let v503: f64 = (-p.p108);
        self.scalar_v503 = v503;
        let v509: f64 = p.p29;
        self.scalar_v509 = v509;
        let v510: f64 = (4.0 - p.p103);
        self.scalar_v510 = v510;
        let v511: f64 = (p.p121 + v510);
        self.scalar_v511 = v511;
        let v515: f64 = p.p112;
        self.scalar_v515 = v515;
        let v516: f64 = (-p.p112);
        self.scalar_v516 = v516;
        let v520: f64 = p.p22;
        self.scalar_v520 = v520;
        let v521: f64 = p.p23;
        self.scalar_v521 = v521;
        let v522: f64 = (2.0 * p.p23);
        self.scalar_v522 = v522;
        let v523: f64 = (6.0 - v522);
        self.scalar_v523 = v523;
        let v530: f64 = p.p149;
        self.scalar_v530 = v530;
        let v531: f64 = p.p150;
        self.scalar_v531 = v531;
        let v532: f64 = (4.0 / p.p150);
        self.scalar_v532 = v532;
        let v539: f64 = p.p155;
        self.scalar_v539 = v539;
        let v542: f64 = p.p157;
        self.scalar_v542 = v542;
        let v550: f64 = p.p35;
        self.scalar_v550 = v550;
        let v559: f64 = p.p34;
        self.scalar_v559 = v559;
        let v572: f64 = p.p37;
        self.scalar_v572 = v572;
        let v581: f64 = p.p36;
        self.scalar_v581 = v581;
        let v593: f64 = p.p14;
        self.scalar_v593 = v593;
        let v596: f64 = p.p13;
        self.scalar_v596 = v596;
        let v599: f64 = p.p133;
        self.scalar_v599 = v599;
        let v600: f64 = p.p141;
        self.scalar_v600 = v600;
        let v601: f64 = (4.0 - p.p141);
        self.scalar_v601 = v601;
        let v605: f64 = (-p.p140);
        self.scalar_v605 = v605;
        let v610: f64 = p.p142;
        self.scalar_v610 = v610;
        let v611: f64 = (0.5 * p.p142);
        self.scalar_v611 = v611;
        let v612: f64 = (3.5 - v611);
        self.scalar_v612 = v612;
        let v617: f64 = p.p135;
        self.scalar_v617 = v617;
        let v618: f64 = (1.0 - p.p141);
        self.scalar_v618 = v618;
        let v622: f64 = p.p136;
        self.scalar_v622 = v622;
        let v623: f64 = (1.0 - p.p142);
        self.scalar_v623 = v623;
        let v642: f64 = (v12 * 1.081);
        self.scalar_v642 = v642;
        let v644: f64 = p.p92;
        self.scalar_v644 = v644;
        let v646: f64 = p.p146;
        self.scalar_v646 = v646;
        let v648: f64 = p.p148;
        self.scalar_v648 = v648;
        let v651: bool = (p.p57 > 0.0);
        self.scalar_v651 = v651;
        let v657: bool = (!v651);
        self.scalar_v657 = v657;
        let v659: bool = (p.p58 > 0.0);
        self.scalar_v659 = v659;
        let v665: bool = (!v659);
        self.scalar_v665 = v665;
        let v667: bool = (p.p59 > 0.0);
        self.scalar_v667 = v667;
        let v673: bool = (!v667);
        self.scalar_v673 = v673;
        let v721: f64 = p.p151;
        self.scalar_v721 = v721;
        let v726: f64 = ((p.p151) as f64).exp();
        self.scalar_v726 = v726;
        let v844: f64 = p.p153;
        self.scalar_v844 = v844;
        let v897: f64 = p.p62;
        self.scalar_v897 = v897;
        let v898: f64 = p.p61;
        self.scalar_v898 = v898;
        let v899: f64 = (p.p62 * p.p61);
        self.scalar_v899 = v899;
        let v910: f64 = p.p63;
        self.scalar_v910 = v910;
        let v931: f64 = (-1.0 / p.p63);
        self.scalar_v931 = v931;
        let v932: f64 = ((v931) as f64).exp();
        self.scalar_v932 = v932;
        let v933: f64 = (1.0 + v932);
        self.scalar_v933 = v933;
        let v934: f64 = ((v933) as f64).ln();
        self.scalar_v934 = v934;
        let v935: f64 = (p.p63 * v934);
        self.scalar_v935 = v935;
        let v936: f64 = (1.0 + v935);
        self.scalar_v936 = v936;
        let v983: f64 = p.p152;
        self.scalar_v983 = v983;
        let v993: f64 = (0.5 * p.p61);
        self.scalar_v993 = v993;
        let v1006: f64 = p.p73;
        self.scalar_v1006 = v1006;
        let v1007: bool = (0.0 == p.p73);
        self.scalar_v1007 = v1007;
        let v1011: bool = (!v1007);
        self.scalar_v1011 = v1011;
        let v1060: f64 = (-1.0 / p.p67);
        self.scalar_v1060 = v1060;
        let v1061: f64 = f64::powf(3.0, v1060);
        self.scalar_v1061 = v1061;
        let v1062: f64 = (1.0 - v1061);
        self.scalar_v1062 = v1062;
        let v1084: f64 = (1.0 - p.p67);
        self.scalar_v1084 = v1084;
        let v1092: f64 = p.p74;
        self.scalar_v1092 = v1092;
        let v1093: bool = (1.0 == p.p74);
        self.scalar_v1093 = v1093;
        let v1095: bool = (2.0 == p.p74);
        self.scalar_v1095 = v1095;
        let v1096: bool = (!v1093);
        self.scalar_v1096 = v1096;
        let v1097: bool = (v1095 && v1096);
        self.scalar_v1097 = v1097;
        let v1100: bool = (!v1095);
        self.scalar_v1100 = v1100;
        let v1101: bool = (v1096 && v1100);
        self.scalar_v1101 = v1101;
        let v1106: f64 = (-1.0 / p.p72);
        self.scalar_v1106 = v1106;
        let v1127: f64 = p.p76;
        self.scalar_v1127 = v1127;
        let v1129: f64 = (1.0 - p.p72);
        self.scalar_v1129 = v1129;
        let v1158: bool = (0.0 == p.p92);
        self.scalar_v1158 = v1158;
        let v1164: bool = (!v1158);
        self.scalar_v1164 = v1164;
        let v1198: f64 = p.p15;
        self.scalar_v1198 = v1198;
        let v1220: f64 = p.p156;
        self.scalar_v1220 = v1220;
        let v1233: f64 = p.p158;
        self.scalar_v1233 = v1233;
        let v1251: f64 = p.p159;
        self.scalar_v1251 = v1251;
        let v1314: f64 = p.p93;
        self.scalar_v1314 = v1314;
        let v1315: bool = (0.0 == p.p93);
        self.scalar_v1315 = v1315;
        let v1316: bool = (!v485);
        self.scalar_v1316 = v1316;
        let v1317: bool = (v1315 && v1316);
        self.scalar_v1317 = v1317;
        let v1319: bool = (!v1315);
        self.scalar_v1319 = v1319;
        let v1320: bool = (v1316 && v1319);
        self.scalar_v1320 = v1320;
        let v1321: f64 = (1.0 - p.p93);
        self.scalar_v1321 = v1321;
        let v1414: bool = (p.p34 > 0.0);
        self.scalar_v1414 = v1414;
        let v1415: bool = (p.p35 > 0.0);
        self.scalar_v1415 = v1415;
        let v1416: bool = (v1414 && v1415);
        self.scalar_v1416 = v1416;
        let v1440: f64 = (-2.0 - p.p67);
        self.scalar_v1440 = v1440;
        let v1442: f64 = (p.p67 * p.p67);
        self.scalar_v1442 = v1442;
        let v1443: f64 = (1.0 - v1442);
        self.scalar_v1443 = v1443;
        let v1445: f64 = (p.p67 - 1.0);
        self.scalar_v1445 = v1445;
        let v1505: bool = (p.p36 > 0.0);
        self.scalar_v1505 = v1505;
        let v1506: bool = (p.p37 > 0.0);
        self.scalar_v1506 = v1506;
        let v1507: bool = (v1505 && v1506);
        self.scalar_v1507 = v1507;
        let v1533: f64 = (-2.0 - p.p72);
        self.scalar_v1533 = v1533;
        let v1535: f64 = (p.p72 * p.p72);
        self.scalar_v1535 = v1535;
        let v1536: f64 = (1.0 - v1535);
        self.scalar_v1536 = v1536;
        let v1538: f64 = (p.p72 - 1.0);
        self.scalar_v1538 = v1538;
        let v1604: f64 = p.p8;
        self.scalar_v1604 = v1604;
        let v1605: bool = (1.0 == p.p8);
        self.scalar_v1605 = v1605;
        let v1606: f64 = p.p143;
        self.scalar_v1606 = v1606;
        let v1607: f64 = (2.0 * p.p143);
        self.scalar_v1607 = v1607;
        let v1613: f64 = p.p144;
        self.scalar_v1613 = v1613;
        let v1622: f64 = (1.0 - p.p143);
        self.scalar_v1622 = v1622;
        let v1623: f64 = (2.0 * v1622);
        self.scalar_v1623 = v1623;
        let v1635: bool = (!v1605);
        self.scalar_v1635 = v1635;
        let v1654: f64 = (4.0 * p.p144);
        self.scalar_v1654 = v1654;
        let v1664: f64 = p.p5;
        self.scalar_v1664 = v1664;
        let v1665: bool = (p.p5 > 0.0);
        self.scalar_v1665 = v1665;
        let v1666: bool = (p.p33 > 0.0);
        self.scalar_v1666 = v1666;
        let v1667: bool = (v1665 && v1666);
        self.scalar_v1667 = v1667;
        let v1672: f64 = (p.p33 * 2.0);
        self.scalar_v1672 = v1672;
        let v1682: bool = (v1605 && v1667);
        self.scalar_v1682 = v1682;
        let v1683: f64 = (p.p33 * v1622);
        self.scalar_v1683 = v1683;
        let v1684: f64 = (2.0 * v1683);
        self.scalar_v1684 = v1684;
        let v1698: bool = (v1635 && v1667);
        self.scalar_v1698 = v1698;
        let v1706: bool = (1.0 == p.p5);
        self.scalar_v1706 = v1706;
        let v1707: bool = (v1667 && v1706);
        self.scalar_v1707 = v1707;
        let v1720: f64 = (if v1707 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1720 = v1720;
        let v1725: f64 = (0.5 * v1720);
        self.scalar_v1725 = v1725;
        let v1742: bool = (!v1706);
        self.scalar_v1742 = v1742;
        let v1743: bool = (v1667 && v1742);
        self.scalar_v1743 = v1743;
        let v1749: f64 = p.p84;
        self.scalar_v1749 = v1749;
        let v1750: bool = (1.0 == p.p84);
        self.scalar_v1750 = v1750;
        let v1753: f64 = (if v1750 { 1e-12 } else { v1720 });
        self.scalar_v1753 = v1753;
        let v1759: f64 = (0.5 * v1753);
        self.scalar_v1759 = v1759;
        let v1770: f64 = p.p82;
        self.scalar_v1770 = v1770;
        let v1771: f64 = f64::powf(v105, p.p82);
        self.scalar_v1771 = v1771;
        let v1772: f64 = (1.0 - v1771);
        self.scalar_v1772 = v1772;
        let v1773: f64 = (1.0 / v1772);
        self.scalar_v1773 = v1773;
        let v1774: f64 = (if v1750 { v1773 } else { 0.0 });
        self.scalar_v1774 = v1774;
        let v1775: f64 = p.p81;
        self.scalar_v1775 = v1775;
        let v1776: f64 = (v105 * p.p81);
        self.scalar_v1776 = v1776;
        let v1777: f64 = (if v1750 { v1776 } else { 0.0 });
        self.scalar_v1777 = v1777;
        let v1778: f64 = (v1774 * v1774);
        self.scalar_v1778 = v1778;
        let v1779: f64 = (p.p82 - 1.0);
        self.scalar_v1779 = v1779;
        let v1780: f64 = f64::powf(v105, v1779);
        self.scalar_v1780 = v1780;
        let v1781: f64 = (v1778 * v1780);
        self.scalar_v1781 = v1781;
        let v1782: f64 = (p.p82 * v1781);
        self.scalar_v1782 = v1782;
        let v1783: f64 = (v1782 / p.p81);
        self.scalar_v1783 = v1783;
        let v1784: f64 = (if v1750 { v1783 } else { 0.0 });
        self.scalar_v1784 = v1784;
        let v1798: bool = (!v1750);
        self.scalar_v1798 = v1798;
        let v1825: f64 = p.p39;
        self.scalar_v1825 = v1825;
        let v1826: bool = (1.0 == p.p39);
        self.scalar_v1826 = v1826;
        let v1827: f64 = p.p44;
        self.scalar_v1827 = v1827;
        let v1830: f64 = p.p42;
        self.scalar_v1830 = v1830;
        let v1849: f64 = p.p41;
        self.scalar_v1849 = v1849;
        let v1863: f64 = p.p40;
        self.scalar_v1863 = v1863;
        let v1868: bool = (2.0 == p.p39);
        self.scalar_v1868 = v1868;
        let v1870: bool = (!v1826);
        self.scalar_v1870 = v1870;
        let v1874: f64 = p.p46;
        self.scalar_v1874 = v1874;
        let v1875: f64 = (2.0 * p.p46);
        self.scalar_v1875 = v1875;
        let v1876: f64 = p.p45;
        self.scalar_v1876 = v1876;
        let v1877: f64 = (p.p45 * p.p45);
        self.scalar_v1877 = v1877;
        let v1878: f64 = (v1875 / v1877);
        self.scalar_v1878 = v1878;
        let v1887: f64 = p.p7;
        self.scalar_v1887 = v1887;
        let v1888: bool = (0.0 == p.p7);
        self.scalar_v1888 = v1888;
        let v1891: bool = (!v1888);
        self.scalar_v1891 = v1891;
        let v1914: f64 = p.p47;
        self.scalar_v1914 = v1914;
        let v1915: f64 = (2.0 * p.p47);
        self.scalar_v1915 = v1915;
        let v1921: f64 = (1.0 + p.p47);
        self.scalar_v1921 = v1921;
        let v1922: f64 = (1.0 + v1915);
        self.scalar_v1922 = v1922;
        let v1923: f64 = (v1921 / v1922);
        self.scalar_v1923 = v1923;
        let v1971: bool = (3.0 == p.p39);
        self.scalar_v1971 = v1971;
        let v1972: bool = (!v1868);
        self.scalar_v1972 = v1972;
        let v1977: f64 = p.p48;
        self.scalar_v1977 = v1977;
        let v1981: f64 = p.p49;
        self.scalar_v1981 = v1981;
        let v1988: f64 = p.p52;
        self.scalar_v1988 = v1988;
        let v1993: f64 = p.p51;
        self.scalar_v1993 = v1993;
        let v2013: f64 = p.p50;
        self.scalar_v2013 = v2013;
        let v2033: f64 = p.p53;
        self.scalar_v2033 = v2033;
        let v2034: bool = (1.0 == p.p53);
        self.scalar_v2034 = v2034;
        let v2069: bool = (!v1971);
        self.scalar_v2069 = v2069;
        let v2075: bool = (!v2034);
        self.scalar_v2075 = v2075;
        let v2142: f64 = (1.0 - p.p148);
        self.scalar_v2142 = v2142;
        let v2143: bool = (p.p146 > v28);
        self.scalar_v2143 = v2143;
        let v2144: f64 = p.p145;
        self.scalar_v2144 = v2144;
        let v2145: bool = (0.0 == p.p145);
        self.scalar_v2145 = v2145;
        let v2146: bool = (v2143 && v2145);
        self.scalar_v2146 = v2146;
        let v2150: f64 = ((v2142) as f64).abs();
        self.scalar_v2150 = v2150;
        let v2151: bool = (v2150 < 1e-6);
        self.scalar_v2151 = v2151;
        let v2152: bool = (!v2145);
        self.scalar_v2152 = v2152;
        let v2153: bool = (v2143 && v2152);
        self.scalar_v2153 = v2153;
        let v2154: bool = (v2151 && v2153);
        self.scalar_v2154 = v2154;
        let v2162: bool = (!v2151);
        self.scalar_v2162 = v2162;
        let v2163: bool = (v2153 && v2162);
        self.scalar_v2163 = v2163;
        let v2171: bool = (!v2143);
        self.scalar_v2171 = v2171;
        let v2176: f64 = p.p130;
        self.scalar_v2176 = v2176;
        let v2177: bool = (p.p130 > 0.0);
        self.scalar_v2177 = v2177;
        let v2181: bool = (!v2177);
        self.scalar_v2181 = v2181;
        let v2230: f64 = (if v665 { 0.0 } else { 0.0 });
        self.scalar_v2230 = v2230;
        let v2235: f64 = (if v673 { 0.0 } else { 0.0 });
        self.scalar_v2235 = v2235;
        let v2736: f64 = (-p.p3);
        self.scalar_v2736 = v2736;
        let v2737: f64 = (p.p3 + v2736);
        self.scalar_v2737 = v2737;
        let v2738: f64 = (v2736 - v2736);
        self.scalar_v2738 = v2738;
        let v2739: f64 = (p.p3 + v2737);
        self.scalar_v2739 = v2739;
        let v3729: f64 = (v1084 - 1.0);
        self.scalar_v3729 = v3729;
        let v3753: f64 = (if v1093 { p.p3 } else { 0.0 });
        self.scalar_v3753 = v3753;
        let v3754: f64 = (if v1093 { v2736 } else { 0.0 });
        self.scalar_v3754 = v3754;
        let v3771: f64 = (v1106 - 1.0);
        self.scalar_v3771 = v3771;
        let v3857: f64 = (p.p76 - 1.0);
        self.scalar_v3857 = v3857;
        let v3876: f64 = (v1129 - 1.0);
        self.scalar_v3876 = v3876;
        let v4219: f64 = (v2736 / 0.0001);
        self.scalar_v4219 = v4219;
        let v4220: f64 = (p.p3 / 0.0001);
        self.scalar_v4220 = v4220;
        let v4229: f64 = (-v4219);
        self.scalar_v4229 = v4229;
        let v4230: f64 = (-v4220);
        self.scalar_v4230 = v4230;
        let v4254: f64 = (v2736 / 0.001);
        self.scalar_v4254 = v4254;
        let v4255: f64 = (p.p3 / 0.001);
        self.scalar_v4255 = v4255;
        let v4266: f64 = (-v4254);
        self.scalar_v4266 = v4266;
        let v4267: f64 = (-v4255);
        self.scalar_v4267 = v4267;
        let v4732: f64 = (v1440 - 1.0);
        self.scalar_v4732 = v4732;
        let v4789: f64 = (v39 * v2736);
        self.scalar_v4789 = v4789;
        let v4790: f64 = (p.p3 * v39);
        self.scalar_v4790 = v4790;
        let v4853: f64 = (0.5 * v2736);
        self.scalar_v4853 = v4853;
        let v4854: f64 = (p.p3 * 0.5);
        self.scalar_v4854 = v4854;
        let v4987: f64 = (v1533 - 1.0);
        self.scalar_v4987 = v4987;
        let v5044: f64 = (p.p3 * v74);
        self.scalar_v5044 = v5044;
        let v5045: f64 = (v74 * v2736);
        self.scalar_v5045 = v5045;
        let v5466: f64 = (p.p3 * v34);
        self.scalar_v5466 = v5466;
        let v5467: f64 = (v34 * v2736);
        self.scalar_v5467 = v5467;
        let v5678: f64 = (if v1707 { v2737 } else { 0.0 });
        self.scalar_v5678 = v5678;
        let v5679: f64 = (if v1707 { v2739 } else { 0.0 });
        self.scalar_v5679 = v5679;
        let v5681: f64 = (if v1707 { v2738 } else { 0.0 });
        self.scalar_v5681 = v5681;
        let v5682: f64 = (if v1707 { v2736 } else { 0.0 });
        self.scalar_v5682 = v5682;
        let v5936: f64 = (if v1750 { p.p3 } else { 0.0 });
        self.scalar_v5936 = v5936;
        let v5937: f64 = (if v1750 { v2737 } else { 0.0 });
        self.scalar_v5937 = v5937;
        let v5938: f64 = (if v1750 { v2736 } else { 0.0 });
        self.scalar_v5938 = v5938;
        let v5939: f64 = (-v5936);
        self.scalar_v5939 = v5939;
        let v5940: f64 = (-v5937);
        self.scalar_v5940 = v5940;
        let v5941: f64 = (-v5938);
        self.scalar_v5941 = v5941;
        let v6369: f64 = (p.p41 - 1.0);
        self.scalar_v6369 = v6369;
        let v6976: f64 = (p.p49 - 1.0);
        self.scalar_v6976 = v6976;
        let v7073: f64 = (p.p50 - 1.0);
        self.scalar_v7073 = v7073;
        let v7386: f64 = (if v485 { p.p3 } else { 0.0 });
        self.scalar_v7386 = v7386;
        let v7387: f64 = (if v485 { v2736 } else { 0.0 });
        self.scalar_v7387 = v7387;
        let v7388: f64 = (if v1316 { p.p3 } else { v7386 });
        self.scalar_v7388 = v7388;
        let v7389: f64 = (if v1316 { 0.0 } else { v7387 });
        self.scalar_v7389 = v7389;
        let v7390: f64 = (if v1316 { v2736 } else { 0.0 });
        self.scalar_v7390 = v7390;
        let v7521: f64 = (0.0 * v2736);
        self.scalar_v7521 = v7521;
        let v7522: f64 = (p.p3 * 0.0);
        self.scalar_v7522 = v7522;
        let v7608: f64 = (0.0 * v2737);
        self.scalar_v7608 = v7608;
        let v7609: f64 = (0.0 * v2738);
        self.scalar_v7609 = v7609;
        let v7677: f64 = (v2738 - v2738);
        self.scalar_v7677 = v7677;
        let v7767: f64 = (v2142 - 1.0);
        self.scalar_v7767 = v7767;
        let v7773: f64 = (1.0 / v26);
        self.scalar_v7773 = v7773;
        let v7949: f64 = (p.p3 * p.p3);
        self.scalar_v7949 = v7949;
        let v7950: f64 = (p.p3 * v2736);
        self.scalar_v7950 = v7950;
        let v8013: f64 = (p.p3 * v2737);
        self.scalar_v8013 = v8013;
        let v8014: f64 = (p.p3 * v2738);
        self.scalar_v8014 = v8014;
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
        let v647: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v647 = v647;
        let v649: f64 = f64::powf(self.scalar_v647, self.scalar_v648);
        self.scalar_v649 = v649;
        let v650: f64 = (self.scalar_v646 * self.scalar_v649);
        self.scalar_v650 = v650;
        let v2155: f64 = (self.scalar_v20 / self.scalar_v650);
        self.scalar_v2155 = v2155;
        let v2156: f64 = (self.scalar_v27 * self.scalar_v2155);
        self.scalar_v2156 = v2156;
        let v2164: f64 = (self.scalar_v650 * self.scalar_v2142);
        self.scalar_v2164 = v2164;
        let v2165: f64 = (self.scalar_v20 / self.scalar_v2164);
        self.scalar_v2165 = v2165;
        let v2166: f64 = (self.scalar_v27 * self.scalar_v2165);
        self.scalar_v2166 = v2166;
        let v7760: f64 = (1.0 / self.scalar_v650);
        self.scalar_v7760 = v7760;
        let v7761: f64 = (self.scalar_v27 * self.scalar_v7760);
        self.scalar_v7761 = v7761;
        let v7762: f64 = (if self.scalar_v2146 { self.scalar_v7761 } else { 0.0 });
        self.scalar_v7762 = v7762;
        let v7763: f64 = (1.0 / self.scalar_v20);
        self.scalar_v7763 = v7763;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
