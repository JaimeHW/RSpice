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
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v325: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v375: f64,
    pub(crate) scalar_v403: bool,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: bool,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v434: bool,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v530: bool,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v565: f64,
    pub(crate) scalar_v566: f64,
    pub(crate) scalar_v567: f64,
    pub(crate) scalar_v568: f64,
    pub(crate) scalar_v575: f64,
    pub(crate) scalar_v576: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v587: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v638: f64,
    pub(crate) scalar_v641: f64,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v645: f64,
    pub(crate) scalar_v646: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v656: f64,
    pub(crate) scalar_v657: f64,
    pub(crate) scalar_v662: f64,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v667: f64,
    pub(crate) scalar_v668: f64,
    pub(crate) scalar_v672: f64,
    pub(crate) scalar_v673: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v682: f64,
    pub(crate) scalar_v683: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v688: f64,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v693: f64,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v698: f64,
    pub(crate) scalar_v699: f64,
    pub(crate) scalar_v700: f64,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v725: f64,
    pub(crate) scalar_v728: bool,
    pub(crate) scalar_v734: bool,
    pub(crate) scalar_v736: bool,
    pub(crate) scalar_v742: bool,
    pub(crate) scalar_v744: bool,
    pub(crate) scalar_v750: bool,
    pub(crate) scalar_v800: f64,
    pub(crate) scalar_v805: f64,
    pub(crate) scalar_v935: f64,
    pub(crate) scalar_v988: f64,
    pub(crate) scalar_v989: f64,
    pub(crate) scalar_v990: f64,
    pub(crate) scalar_v1001: f64,
    pub(crate) scalar_v1022: f64,
    pub(crate) scalar_v1023: f64,
    pub(crate) scalar_v1024: f64,
    pub(crate) scalar_v1025: f64,
    pub(crate) scalar_v1026: f64,
    pub(crate) scalar_v1027: f64,
    pub(crate) scalar_v1074: f64,
    pub(crate) scalar_v1084: f64,
    pub(crate) scalar_v1097: f64,
    pub(crate) scalar_v1098: bool,
    pub(crate) scalar_v1102: bool,
    pub(crate) scalar_v1151: f64,
    pub(crate) scalar_v1152: f64,
    pub(crate) scalar_v1153: f64,
    pub(crate) scalar_v1175: f64,
    pub(crate) scalar_v1183: f64,
    pub(crate) scalar_v1184: bool,
    pub(crate) scalar_v1186: bool,
    pub(crate) scalar_v1187: bool,
    pub(crate) scalar_v1188: bool,
    pub(crate) scalar_v1191: bool,
    pub(crate) scalar_v1192: bool,
    pub(crate) scalar_v1197: f64,
    pub(crate) scalar_v1218: f64,
    pub(crate) scalar_v1220: f64,
    pub(crate) scalar_v1249: bool,
    pub(crate) scalar_v1255: bool,
    pub(crate) scalar_v1289: f64,
    pub(crate) scalar_v1311: f64,
    pub(crate) scalar_v1324: f64,
    pub(crate) scalar_v1342: f64,
    pub(crate) scalar_v1405: f64,
    pub(crate) scalar_v1406: bool,
    pub(crate) scalar_v1407: bool,
    pub(crate) scalar_v1408: bool,
    pub(crate) scalar_v1410: bool,
    pub(crate) scalar_v1411: bool,
    pub(crate) scalar_v1412: f64,
    pub(crate) scalar_v1505: bool,
    pub(crate) scalar_v1506: bool,
    pub(crate) scalar_v1507: bool,
    pub(crate) scalar_v1531: f64,
    pub(crate) scalar_v1533: f64,
    pub(crate) scalar_v1534: f64,
    pub(crate) scalar_v1536: f64,
    pub(crate) scalar_v1596: bool,
    pub(crate) scalar_v1597: bool,
    pub(crate) scalar_v1598: bool,
    pub(crate) scalar_v1624: f64,
    pub(crate) scalar_v1626: f64,
    pub(crate) scalar_v1627: f64,
    pub(crate) scalar_v1629: f64,
    pub(crate) scalar_v1706: f64,
    pub(crate) scalar_v1707: bool,
    pub(crate) scalar_v1708: f64,
    pub(crate) scalar_v1709: f64,
    pub(crate) scalar_v1715: f64,
    pub(crate) scalar_v1724: f64,
    pub(crate) scalar_v1725: f64,
    pub(crate) scalar_v1737: bool,
    pub(crate) scalar_v1756: f64,
    pub(crate) scalar_v1766: f64,
    pub(crate) scalar_v1767: bool,
    pub(crate) scalar_v1768: bool,
    pub(crate) scalar_v1769: bool,
    pub(crate) scalar_v1774: f64,
    pub(crate) scalar_v1784: bool,
    pub(crate) scalar_v1785: f64,
    pub(crate) scalar_v1786: f64,
    pub(crate) scalar_v1800: bool,
    pub(crate) scalar_v1808: bool,
    pub(crate) scalar_v1809: bool,
    pub(crate) scalar_v1822: f64,
    pub(crate) scalar_v1827: f64,
    pub(crate) scalar_v1844: bool,
    pub(crate) scalar_v1845: bool,
    pub(crate) scalar_v1851: f64,
    pub(crate) scalar_v1852: bool,
    pub(crate) scalar_v1855: f64,
    pub(crate) scalar_v1861: f64,
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
    pub(crate) scalar_v1883: f64,
    pub(crate) scalar_v1884: f64,
    pub(crate) scalar_v1885: f64,
    pub(crate) scalar_v1886: f64,
    pub(crate) scalar_v1900: bool,
    pub(crate) scalar_v1927: f64,
    pub(crate) scalar_v1928: bool,
    pub(crate) scalar_v1929: f64,
    pub(crate) scalar_v1932: f64,
    pub(crate) scalar_v1951: f64,
    pub(crate) scalar_v1965: f64,
    pub(crate) scalar_v1970: bool,
    pub(crate) scalar_v1972: bool,
    pub(crate) scalar_v1976: f64,
    pub(crate) scalar_v1977: f64,
    pub(crate) scalar_v1978: f64,
    pub(crate) scalar_v1979: f64,
    pub(crate) scalar_v1980: f64,
    pub(crate) scalar_v1989: f64,
    pub(crate) scalar_v1990: bool,
    pub(crate) scalar_v1993: bool,
    pub(crate) scalar_v2016: f64,
    pub(crate) scalar_v2017: f64,
    pub(crate) scalar_v2023: f64,
    pub(crate) scalar_v2024: f64,
    pub(crate) scalar_v2025: f64,
    pub(crate) scalar_v2073: bool,
    pub(crate) scalar_v2074: bool,
    pub(crate) scalar_v2079: f64,
    pub(crate) scalar_v2083: f64,
    pub(crate) scalar_v2090: f64,
    pub(crate) scalar_v2095: f64,
    pub(crate) scalar_v2115: f64,
    pub(crate) scalar_v2135: f64,
    pub(crate) scalar_v2136: bool,
    pub(crate) scalar_v2171: bool,
    pub(crate) scalar_v2177: bool,
    pub(crate) scalar_v2244: f64,
    pub(crate) scalar_v2245: f64,
    pub(crate) scalar_v2275: f64,
    pub(crate) scalar_v2313: f64,
    pub(crate) scalar_v2348: f64,
    pub(crate) scalar_v2349: f64,
    pub(crate) scalar_v2350: f64,
    pub(crate) scalar_v2369: f64,
    pub(crate) scalar_v2382: f64,
    pub(crate) scalar_v2383: f64,
    pub(crate) scalar_v2405: f64,
    pub(crate) scalar_v2406: bool,
    pub(crate) scalar_v2415: f64,
    pub(crate) scalar_v2419: bool,
    pub(crate) scalar_v2438: bool,
    pub(crate) scalar_v2439: bool,
    pub(crate) scalar_v2440: bool,
    pub(crate) scalar_v2443: bool,
    pub(crate) scalar_v2459: f64,
    pub(crate) scalar_v2470: bool,
    pub(crate) scalar_v2491: f64,
    pub(crate) scalar_v2492: bool,
    pub(crate) scalar_v2493: f64,
    pub(crate) scalar_v2531: f64,
    pub(crate) scalar_v2532: f64,
    pub(crate) scalar_v2538: f64,
    pub(crate) scalar_v2542: f64,
    pub(crate) scalar_v2545: bool,
    pub(crate) scalar_v2549: f64,
    pub(crate) scalar_v2553: f64,
    pub(crate) scalar_v2554: bool,
    pub(crate) scalar_v2555: f64,
    pub(crate) scalar_v2556: bool,
    pub(crate) scalar_v2557: bool,
    pub(crate) scalar_v2561: f64,
    pub(crate) scalar_v2562: bool,
    pub(crate) scalar_v2563: bool,
    pub(crate) scalar_v2564: bool,
    pub(crate) scalar_v2565: bool,
    pub(crate) scalar_v2573: bool,
    pub(crate) scalar_v2574: bool,
    pub(crate) scalar_v2582: bool,
    pub(crate) scalar_v2587: f64,
    pub(crate) scalar_v2588: bool,
    pub(crate) scalar_v2592: bool,
    pub(crate) scalar_v2602: f64,
    pub(crate) scalar_v2603: bool,
    pub(crate) scalar_v2606: bool,
    pub(crate) scalar_v2607: bool,
    pub(crate) scalar_v2608: bool,
    pub(crate) scalar_v2609: f64,
    pub(crate) scalar_v2612: bool,
    pub(crate) scalar_v2613: bool,
    pub(crate) scalar_v2623: f64,
    pub(crate) scalar_v2624: f64,
    pub(crate) scalar_v2673: f64,
    pub(crate) scalar_v2677: f64,
    pub(crate) scalar_v2699: f64,
    pub(crate) scalar_v2704: f64,
    pub(crate) scalar_v2709: f64,
    pub(crate) scalar_v2710: f64,
    pub(crate) scalar_v2711: bool,
    pub(crate) scalar_v2712: f64,
    pub(crate) scalar_v2713: bool,
    pub(crate) scalar_v2714: f64,
    pub(crate) scalar_v2715: bool,
    pub(crate) scalar_v2716: f64,
    pub(crate) scalar_v2717: bool,
    pub(crate) scalar_v2718: f64,
    pub(crate) scalar_v2933: f64,
    pub(crate) scalar_v3284: f64,
    pub(crate) scalar_v3285: f64,
    pub(crate) scalar_v3286: f64,
    pub(crate) scalar_v3287: f64,
    pub(crate) scalar_v4321: f64,
    pub(crate) scalar_v4345: f64,
    pub(crate) scalar_v4346: f64,
    pub(crate) scalar_v4363: f64,
    pub(crate) scalar_v4449: f64,
    pub(crate) scalar_v4468: f64,
    pub(crate) scalar_v4811: f64,
    pub(crate) scalar_v4812: f64,
    pub(crate) scalar_v4821: f64,
    pub(crate) scalar_v4822: f64,
    pub(crate) scalar_v4846: f64,
    pub(crate) scalar_v4847: f64,
    pub(crate) scalar_v4858: f64,
    pub(crate) scalar_v4859: f64,
    pub(crate) scalar_v5324: f64,
    pub(crate) scalar_v5381: f64,
    pub(crate) scalar_v5382: f64,
    pub(crate) scalar_v5445: f64,
    pub(crate) scalar_v5446: f64,
    pub(crate) scalar_v5579: f64,
    pub(crate) scalar_v5636: f64,
    pub(crate) scalar_v5637: f64,
    pub(crate) scalar_v6125: f64,
    pub(crate) scalar_v6126: f64,
    pub(crate) scalar_v6337: f64,
    pub(crate) scalar_v6338: f64,
    pub(crate) scalar_v6340: f64,
    pub(crate) scalar_v6341: f64,
    pub(crate) scalar_v6595: f64,
    pub(crate) scalar_v6596: f64,
    pub(crate) scalar_v6597: f64,
    pub(crate) scalar_v6598: f64,
    pub(crate) scalar_v6599: f64,
    pub(crate) scalar_v6600: f64,
    pub(crate) scalar_v7028: f64,
    pub(crate) scalar_v7635: f64,
    pub(crate) scalar_v7732: f64,
    pub(crate) scalar_v8045: f64,
    pub(crate) scalar_v8046: f64,
    pub(crate) scalar_v8047: f64,
    pub(crate) scalar_v8048: f64,
    pub(crate) scalar_v8049: f64,
    pub(crate) scalar_v8180: f64,
    pub(crate) scalar_v8181: f64,
    pub(crate) scalar_v8267: f64,
    pub(crate) scalar_v8268: f64,
    pub(crate) scalar_v8336: f64,
    pub(crate) scalar_v8892: f64,
    pub(crate) scalar_v8927: f64,
    pub(crate) scalar_v9037: f64,
    pub(crate) scalar_v9038: f64,
    pub(crate) scalar_v9039: f64,
    pub(crate) scalar_v9040: f64,
    pub(crate) scalar_v9355: f64,
    pub(crate) scalar_v9493: f64,
    pub(crate) scalar_v9494: f64,
    pub(crate) scalar_v9614: f64,
    pub(crate) scalar_v9620: f64,
    pub(crate) scalar_v9949: f64,
    pub(crate) scalar_v9950: f64,
    pub(crate) scalar_v10072: f64,
    pub(crate) scalar_v10073: f64,
    pub(crate) scalar_v10078: f64,
    pub(crate) scalar_v10079: f64,
    pub(crate) scalar_v10106: f64,
    pub(crate) scalar_v10107: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v724: f64,
    pub(crate) scalar_v726: f64,
    pub(crate) scalar_v727: f64,
    pub(crate) scalar_v2566: f64,
    pub(crate) scalar_v2567: f64,
    pub(crate) scalar_v2575: f64,
    pub(crate) scalar_v2576: f64,
    pub(crate) scalar_v2577: f64,
    pub(crate) scalar_v9607: f64,
    pub(crate) scalar_v9608: f64,
    pub(crate) scalar_v9609: f64,
    pub(crate) scalar_v9610: f64,
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
            scalar_v225: self.scalar_v225,
            scalar_v266: self.scalar_v266,
            scalar_v269: self.scalar_v269,
            scalar_v289: self.scalar_v289,
            scalar_v292: self.scalar_v292,
            scalar_v318: self.scalar_v318,
            scalar_v320: self.scalar_v320,
            scalar_v322: self.scalar_v322,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v332: self.scalar_v332,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v345: self.scalar_v345,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v366: self.scalar_v366,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v373: self.scalar_v373,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v403: self.scalar_v403,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v407: self.scalar_v407,
            scalar_v434: self.scalar_v434,
            scalar_v436: self.scalar_v436,
            scalar_v437: self.scalar_v437,
            scalar_v455: self.scalar_v455,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v465: self.scalar_v465,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v481: self.scalar_v481,
            scalar_v483: self.scalar_v483,
            scalar_v484: self.scalar_v484,
            scalar_v485: self.scalar_v485,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v502: self.scalar_v502,
            scalar_v507: self.scalar_v507,
            scalar_v508: self.scalar_v508,
            scalar_v509: self.scalar_v509,
            scalar_v511: self.scalar_v511,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v529: self.scalar_v529,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v546: self.scalar_v546,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v554: self.scalar_v554,
            scalar_v555: self.scalar_v555,
            scalar_v556: self.scalar_v556,
            scalar_v560: self.scalar_v560,
            scalar_v561: self.scalar_v561,
            scalar_v565: self.scalar_v565,
            scalar_v566: self.scalar_v566,
            scalar_v567: self.scalar_v567,
            scalar_v568: self.scalar_v568,
            scalar_v575: self.scalar_v575,
            scalar_v576: self.scalar_v576,
            scalar_v577: self.scalar_v577,
            scalar_v584: self.scalar_v584,
            scalar_v587: self.scalar_v587,
            scalar_v595: self.scalar_v595,
            scalar_v604: self.scalar_v604,
            scalar_v617: self.scalar_v617,
            scalar_v626: self.scalar_v626,
            scalar_v638: self.scalar_v638,
            scalar_v641: self.scalar_v641,
            scalar_v644: self.scalar_v644,
            scalar_v645: self.scalar_v645,
            scalar_v646: self.scalar_v646,
            scalar_v650: self.scalar_v650,
            scalar_v655: self.scalar_v655,
            scalar_v656: self.scalar_v656,
            scalar_v657: self.scalar_v657,
            scalar_v662: self.scalar_v662,
            scalar_v663: self.scalar_v663,
            scalar_v667: self.scalar_v667,
            scalar_v668: self.scalar_v668,
            scalar_v672: self.scalar_v672,
            scalar_v673: self.scalar_v673,
            scalar_v677: self.scalar_v677,
            scalar_v678: self.scalar_v678,
            scalar_v682: self.scalar_v682,
            scalar_v683: self.scalar_v683,
            scalar_v684: self.scalar_v684,
            scalar_v688: self.scalar_v688,
            scalar_v689: self.scalar_v689,
            scalar_v693: self.scalar_v693,
            scalar_v696: self.scalar_v696,
            scalar_v698: self.scalar_v698,
            scalar_v699: self.scalar_v699,
            scalar_v700: self.scalar_v700,
            scalar_v719: self.scalar_v719,
            scalar_v721: self.scalar_v721,
            scalar_v723: self.scalar_v723,
            scalar_v725: self.scalar_v725,
            scalar_v728: self.scalar_v728,
            scalar_v734: self.scalar_v734,
            scalar_v736: self.scalar_v736,
            scalar_v742: self.scalar_v742,
            scalar_v744: self.scalar_v744,
            scalar_v750: self.scalar_v750,
            scalar_v800: self.scalar_v800,
            scalar_v805: self.scalar_v805,
            scalar_v935: self.scalar_v935,
            scalar_v988: self.scalar_v988,
            scalar_v989: self.scalar_v989,
            scalar_v990: self.scalar_v990,
            scalar_v1001: self.scalar_v1001,
            scalar_v1022: self.scalar_v1022,
            scalar_v1023: self.scalar_v1023,
            scalar_v1024: self.scalar_v1024,
            scalar_v1025: self.scalar_v1025,
            scalar_v1026: self.scalar_v1026,
            scalar_v1027: self.scalar_v1027,
            scalar_v1074: self.scalar_v1074,
            scalar_v1084: self.scalar_v1084,
            scalar_v1097: self.scalar_v1097,
            scalar_v1098: self.scalar_v1098,
            scalar_v1102: self.scalar_v1102,
            scalar_v1151: self.scalar_v1151,
            scalar_v1152: self.scalar_v1152,
            scalar_v1153: self.scalar_v1153,
            scalar_v1175: self.scalar_v1175,
            scalar_v1183: self.scalar_v1183,
            scalar_v1184: self.scalar_v1184,
            scalar_v1186: self.scalar_v1186,
            scalar_v1187: self.scalar_v1187,
            scalar_v1188: self.scalar_v1188,
            scalar_v1191: self.scalar_v1191,
            scalar_v1192: self.scalar_v1192,
            scalar_v1197: self.scalar_v1197,
            scalar_v1218: self.scalar_v1218,
            scalar_v1220: self.scalar_v1220,
            scalar_v1249: self.scalar_v1249,
            scalar_v1255: self.scalar_v1255,
            scalar_v1289: self.scalar_v1289,
            scalar_v1311: self.scalar_v1311,
            scalar_v1324: self.scalar_v1324,
            scalar_v1342: self.scalar_v1342,
            scalar_v1405: self.scalar_v1405,
            scalar_v1406: self.scalar_v1406,
            scalar_v1407: self.scalar_v1407,
            scalar_v1408: self.scalar_v1408,
            scalar_v1410: self.scalar_v1410,
            scalar_v1411: self.scalar_v1411,
            scalar_v1412: self.scalar_v1412,
            scalar_v1505: self.scalar_v1505,
            scalar_v1506: self.scalar_v1506,
            scalar_v1507: self.scalar_v1507,
            scalar_v1531: self.scalar_v1531,
            scalar_v1533: self.scalar_v1533,
            scalar_v1534: self.scalar_v1534,
            scalar_v1536: self.scalar_v1536,
            scalar_v1596: self.scalar_v1596,
            scalar_v1597: self.scalar_v1597,
            scalar_v1598: self.scalar_v1598,
            scalar_v1624: self.scalar_v1624,
            scalar_v1626: self.scalar_v1626,
            scalar_v1627: self.scalar_v1627,
            scalar_v1629: self.scalar_v1629,
            scalar_v1706: self.scalar_v1706,
            scalar_v1707: self.scalar_v1707,
            scalar_v1708: self.scalar_v1708,
            scalar_v1709: self.scalar_v1709,
            scalar_v1715: self.scalar_v1715,
            scalar_v1724: self.scalar_v1724,
            scalar_v1725: self.scalar_v1725,
            scalar_v1737: self.scalar_v1737,
            scalar_v1756: self.scalar_v1756,
            scalar_v1766: self.scalar_v1766,
            scalar_v1767: self.scalar_v1767,
            scalar_v1768: self.scalar_v1768,
            scalar_v1769: self.scalar_v1769,
            scalar_v1774: self.scalar_v1774,
            scalar_v1784: self.scalar_v1784,
            scalar_v1785: self.scalar_v1785,
            scalar_v1786: self.scalar_v1786,
            scalar_v1800: self.scalar_v1800,
            scalar_v1808: self.scalar_v1808,
            scalar_v1809: self.scalar_v1809,
            scalar_v1822: self.scalar_v1822,
            scalar_v1827: self.scalar_v1827,
            scalar_v1844: self.scalar_v1844,
            scalar_v1845: self.scalar_v1845,
            scalar_v1851: self.scalar_v1851,
            scalar_v1852: self.scalar_v1852,
            scalar_v1855: self.scalar_v1855,
            scalar_v1861: self.scalar_v1861,
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
            scalar_v1883: self.scalar_v1883,
            scalar_v1884: self.scalar_v1884,
            scalar_v1885: self.scalar_v1885,
            scalar_v1886: self.scalar_v1886,
            scalar_v1900: self.scalar_v1900,
            scalar_v1927: self.scalar_v1927,
            scalar_v1928: self.scalar_v1928,
            scalar_v1929: self.scalar_v1929,
            scalar_v1932: self.scalar_v1932,
            scalar_v1951: self.scalar_v1951,
            scalar_v1965: self.scalar_v1965,
            scalar_v1970: self.scalar_v1970,
            scalar_v1972: self.scalar_v1972,
            scalar_v1976: self.scalar_v1976,
            scalar_v1977: self.scalar_v1977,
            scalar_v1978: self.scalar_v1978,
            scalar_v1979: self.scalar_v1979,
            scalar_v1980: self.scalar_v1980,
            scalar_v1989: self.scalar_v1989,
            scalar_v1990: self.scalar_v1990,
            scalar_v1993: self.scalar_v1993,
            scalar_v2016: self.scalar_v2016,
            scalar_v2017: self.scalar_v2017,
            scalar_v2023: self.scalar_v2023,
            scalar_v2024: self.scalar_v2024,
            scalar_v2025: self.scalar_v2025,
            scalar_v2073: self.scalar_v2073,
            scalar_v2074: self.scalar_v2074,
            scalar_v2079: self.scalar_v2079,
            scalar_v2083: self.scalar_v2083,
            scalar_v2090: self.scalar_v2090,
            scalar_v2095: self.scalar_v2095,
            scalar_v2115: self.scalar_v2115,
            scalar_v2135: self.scalar_v2135,
            scalar_v2136: self.scalar_v2136,
            scalar_v2171: self.scalar_v2171,
            scalar_v2177: self.scalar_v2177,
            scalar_v2244: self.scalar_v2244,
            scalar_v2245: self.scalar_v2245,
            scalar_v2275: self.scalar_v2275,
            scalar_v2313: self.scalar_v2313,
            scalar_v2348: self.scalar_v2348,
            scalar_v2349: self.scalar_v2349,
            scalar_v2350: self.scalar_v2350,
            scalar_v2369: self.scalar_v2369,
            scalar_v2382: self.scalar_v2382,
            scalar_v2383: self.scalar_v2383,
            scalar_v2405: self.scalar_v2405,
            scalar_v2406: self.scalar_v2406,
            scalar_v2415: self.scalar_v2415,
            scalar_v2419: self.scalar_v2419,
            scalar_v2438: self.scalar_v2438,
            scalar_v2439: self.scalar_v2439,
            scalar_v2440: self.scalar_v2440,
            scalar_v2443: self.scalar_v2443,
            scalar_v2459: self.scalar_v2459,
            scalar_v2470: self.scalar_v2470,
            scalar_v2491: self.scalar_v2491,
            scalar_v2492: self.scalar_v2492,
            scalar_v2493: self.scalar_v2493,
            scalar_v2531: self.scalar_v2531,
            scalar_v2532: self.scalar_v2532,
            scalar_v2538: self.scalar_v2538,
            scalar_v2542: self.scalar_v2542,
            scalar_v2545: self.scalar_v2545,
            scalar_v2549: self.scalar_v2549,
            scalar_v2553: self.scalar_v2553,
            scalar_v2554: self.scalar_v2554,
            scalar_v2555: self.scalar_v2555,
            scalar_v2556: self.scalar_v2556,
            scalar_v2557: self.scalar_v2557,
            scalar_v2561: self.scalar_v2561,
            scalar_v2562: self.scalar_v2562,
            scalar_v2563: self.scalar_v2563,
            scalar_v2564: self.scalar_v2564,
            scalar_v2565: self.scalar_v2565,
            scalar_v2573: self.scalar_v2573,
            scalar_v2574: self.scalar_v2574,
            scalar_v2582: self.scalar_v2582,
            scalar_v2587: self.scalar_v2587,
            scalar_v2588: self.scalar_v2588,
            scalar_v2592: self.scalar_v2592,
            scalar_v2602: self.scalar_v2602,
            scalar_v2603: self.scalar_v2603,
            scalar_v2606: self.scalar_v2606,
            scalar_v2607: self.scalar_v2607,
            scalar_v2608: self.scalar_v2608,
            scalar_v2609: self.scalar_v2609,
            scalar_v2612: self.scalar_v2612,
            scalar_v2613: self.scalar_v2613,
            scalar_v2623: self.scalar_v2623,
            scalar_v2624: self.scalar_v2624,
            scalar_v2673: self.scalar_v2673,
            scalar_v2677: self.scalar_v2677,
            scalar_v2699: self.scalar_v2699,
            scalar_v2704: self.scalar_v2704,
            scalar_v2709: self.scalar_v2709,
            scalar_v2710: self.scalar_v2710,
            scalar_v2711: self.scalar_v2711,
            scalar_v2712: self.scalar_v2712,
            scalar_v2713: self.scalar_v2713,
            scalar_v2714: self.scalar_v2714,
            scalar_v2715: self.scalar_v2715,
            scalar_v2716: self.scalar_v2716,
            scalar_v2717: self.scalar_v2717,
            scalar_v2718: self.scalar_v2718,
            scalar_v2933: self.scalar_v2933,
            scalar_v3284: self.scalar_v3284,
            scalar_v3285: self.scalar_v3285,
            scalar_v3286: self.scalar_v3286,
            scalar_v3287: self.scalar_v3287,
            scalar_v4321: self.scalar_v4321,
            scalar_v4345: self.scalar_v4345,
            scalar_v4346: self.scalar_v4346,
            scalar_v4363: self.scalar_v4363,
            scalar_v4449: self.scalar_v4449,
            scalar_v4468: self.scalar_v4468,
            scalar_v4811: self.scalar_v4811,
            scalar_v4812: self.scalar_v4812,
            scalar_v4821: self.scalar_v4821,
            scalar_v4822: self.scalar_v4822,
            scalar_v4846: self.scalar_v4846,
            scalar_v4847: self.scalar_v4847,
            scalar_v4858: self.scalar_v4858,
            scalar_v4859: self.scalar_v4859,
            scalar_v5324: self.scalar_v5324,
            scalar_v5381: self.scalar_v5381,
            scalar_v5382: self.scalar_v5382,
            scalar_v5445: self.scalar_v5445,
            scalar_v5446: self.scalar_v5446,
            scalar_v5579: self.scalar_v5579,
            scalar_v5636: self.scalar_v5636,
            scalar_v5637: self.scalar_v5637,
            scalar_v6125: self.scalar_v6125,
            scalar_v6126: self.scalar_v6126,
            scalar_v6337: self.scalar_v6337,
            scalar_v6338: self.scalar_v6338,
            scalar_v6340: self.scalar_v6340,
            scalar_v6341: self.scalar_v6341,
            scalar_v6595: self.scalar_v6595,
            scalar_v6596: self.scalar_v6596,
            scalar_v6597: self.scalar_v6597,
            scalar_v6598: self.scalar_v6598,
            scalar_v6599: self.scalar_v6599,
            scalar_v6600: self.scalar_v6600,
            scalar_v7028: self.scalar_v7028,
            scalar_v7635: self.scalar_v7635,
            scalar_v7732: self.scalar_v7732,
            scalar_v8045: self.scalar_v8045,
            scalar_v8046: self.scalar_v8046,
            scalar_v8047: self.scalar_v8047,
            scalar_v8048: self.scalar_v8048,
            scalar_v8049: self.scalar_v8049,
            scalar_v8180: self.scalar_v8180,
            scalar_v8181: self.scalar_v8181,
            scalar_v8267: self.scalar_v8267,
            scalar_v8268: self.scalar_v8268,
            scalar_v8336: self.scalar_v8336,
            scalar_v8892: self.scalar_v8892,
            scalar_v8927: self.scalar_v8927,
            scalar_v9037: self.scalar_v9037,
            scalar_v9038: self.scalar_v9038,
            scalar_v9039: self.scalar_v9039,
            scalar_v9040: self.scalar_v9040,
            scalar_v9355: self.scalar_v9355,
            scalar_v9493: self.scalar_v9493,
            scalar_v9494: self.scalar_v9494,
            scalar_v9614: self.scalar_v9614,
            scalar_v9620: self.scalar_v9620,
            scalar_v9949: self.scalar_v9949,
            scalar_v9950: self.scalar_v9950,
            scalar_v10072: self.scalar_v10072,
            scalar_v10073: self.scalar_v10073,
            scalar_v10078: self.scalar_v10078,
            scalar_v10079: self.scalar_v10079,
            scalar_v10106: self.scalar_v10106,
            scalar_v10107: self.scalar_v10107,
            scalar_v20: self.scalar_v20,
            scalar_v724: self.scalar_v724,
            scalar_v726: self.scalar_v726,
            scalar_v727: self.scalar_v727,
            scalar_v2566: self.scalar_v2566,
            scalar_v2567: self.scalar_v2567,
            scalar_v2575: self.scalar_v2575,
            scalar_v2576: self.scalar_v2576,
            scalar_v2577: self.scalar_v2577,
            scalar_v9607: self.scalar_v9607,
            scalar_v9608: self.scalar_v9608,
            scalar_v9609: self.scalar_v9609,
            scalar_v9610: self.scalar_v9610,
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
            scalar_v225: 0.0,
            scalar_v266: 0.0,
            scalar_v269: 0.0,
            scalar_v289: 0.0,
            scalar_v292: 0.0,
            scalar_v318: 0.0,
            scalar_v320: 0.0,
            scalar_v322: 0.0,
            scalar_v325: 0.0,
            scalar_v326: 0.0,
            scalar_v332: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v342: 0.0,
            scalar_v343: 0.0,
            scalar_v344: 0.0,
            scalar_v345: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v366: 0.0,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v373: 0.0,
            scalar_v374: false,
            scalar_v375: 0.0,
            scalar_v403: false,
            scalar_v405: 0.0,
            scalar_v406: false,
            scalar_v407: 0.0,
            scalar_v434: false,
            scalar_v436: 0.0,
            scalar_v437: 0.0,
            scalar_v455: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v465: 0.0,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v481: 0.0,
            scalar_v483: 0.0,
            scalar_v484: 0.0,
            scalar_v485: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v502: 0.0,
            scalar_v507: 0.0,
            scalar_v508: 0.0,
            scalar_v509: 0.0,
            scalar_v511: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v529: 0.0,
            scalar_v530: false,
            scalar_v531: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v546: 0.0,
            scalar_v547: 0.0,
            scalar_v548: 0.0,
            scalar_v554: 0.0,
            scalar_v555: 0.0,
            scalar_v556: 0.0,
            scalar_v560: 0.0,
            scalar_v561: 0.0,
            scalar_v565: 0.0,
            scalar_v566: 0.0,
            scalar_v567: 0.0,
            scalar_v568: 0.0,
            scalar_v575: 0.0,
            scalar_v576: 0.0,
            scalar_v577: 0.0,
            scalar_v584: 0.0,
            scalar_v587: 0.0,
            scalar_v595: 0.0,
            scalar_v604: 0.0,
            scalar_v617: 0.0,
            scalar_v626: 0.0,
            scalar_v638: 0.0,
            scalar_v641: 0.0,
            scalar_v644: 0.0,
            scalar_v645: 0.0,
            scalar_v646: 0.0,
            scalar_v650: 0.0,
            scalar_v655: 0.0,
            scalar_v656: 0.0,
            scalar_v657: 0.0,
            scalar_v662: 0.0,
            scalar_v663: 0.0,
            scalar_v667: 0.0,
            scalar_v668: 0.0,
            scalar_v672: 0.0,
            scalar_v673: 0.0,
            scalar_v677: 0.0,
            scalar_v678: 0.0,
            scalar_v682: 0.0,
            scalar_v683: 0.0,
            scalar_v684: 0.0,
            scalar_v688: 0.0,
            scalar_v689: 0.0,
            scalar_v693: 0.0,
            scalar_v696: 0.0,
            scalar_v698: 0.0,
            scalar_v699: 0.0,
            scalar_v700: 0.0,
            scalar_v719: 0.0,
            scalar_v721: 0.0,
            scalar_v723: 0.0,
            scalar_v725: 0.0,
            scalar_v728: false,
            scalar_v734: false,
            scalar_v736: false,
            scalar_v742: false,
            scalar_v744: false,
            scalar_v750: false,
            scalar_v800: 0.0,
            scalar_v805: 0.0,
            scalar_v935: 0.0,
            scalar_v988: 0.0,
            scalar_v989: 0.0,
            scalar_v990: 0.0,
            scalar_v1001: 0.0,
            scalar_v1022: 0.0,
            scalar_v1023: 0.0,
            scalar_v1024: 0.0,
            scalar_v1025: 0.0,
            scalar_v1026: 0.0,
            scalar_v1027: 0.0,
            scalar_v1074: 0.0,
            scalar_v1084: 0.0,
            scalar_v1097: 0.0,
            scalar_v1098: false,
            scalar_v1102: false,
            scalar_v1151: 0.0,
            scalar_v1152: 0.0,
            scalar_v1153: 0.0,
            scalar_v1175: 0.0,
            scalar_v1183: 0.0,
            scalar_v1184: false,
            scalar_v1186: false,
            scalar_v1187: false,
            scalar_v1188: false,
            scalar_v1191: false,
            scalar_v1192: false,
            scalar_v1197: 0.0,
            scalar_v1218: 0.0,
            scalar_v1220: 0.0,
            scalar_v1249: false,
            scalar_v1255: false,
            scalar_v1289: 0.0,
            scalar_v1311: 0.0,
            scalar_v1324: 0.0,
            scalar_v1342: 0.0,
            scalar_v1405: 0.0,
            scalar_v1406: false,
            scalar_v1407: false,
            scalar_v1408: false,
            scalar_v1410: false,
            scalar_v1411: false,
            scalar_v1412: 0.0,
            scalar_v1505: false,
            scalar_v1506: false,
            scalar_v1507: false,
            scalar_v1531: 0.0,
            scalar_v1533: 0.0,
            scalar_v1534: 0.0,
            scalar_v1536: 0.0,
            scalar_v1596: false,
            scalar_v1597: false,
            scalar_v1598: false,
            scalar_v1624: 0.0,
            scalar_v1626: 0.0,
            scalar_v1627: 0.0,
            scalar_v1629: 0.0,
            scalar_v1706: 0.0,
            scalar_v1707: false,
            scalar_v1708: 0.0,
            scalar_v1709: 0.0,
            scalar_v1715: 0.0,
            scalar_v1724: 0.0,
            scalar_v1725: 0.0,
            scalar_v1737: false,
            scalar_v1756: 0.0,
            scalar_v1766: 0.0,
            scalar_v1767: false,
            scalar_v1768: false,
            scalar_v1769: false,
            scalar_v1774: 0.0,
            scalar_v1784: false,
            scalar_v1785: 0.0,
            scalar_v1786: 0.0,
            scalar_v1800: false,
            scalar_v1808: false,
            scalar_v1809: false,
            scalar_v1822: 0.0,
            scalar_v1827: 0.0,
            scalar_v1844: false,
            scalar_v1845: false,
            scalar_v1851: 0.0,
            scalar_v1852: false,
            scalar_v1855: 0.0,
            scalar_v1861: 0.0,
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
            scalar_v1883: 0.0,
            scalar_v1884: 0.0,
            scalar_v1885: 0.0,
            scalar_v1886: 0.0,
            scalar_v1900: false,
            scalar_v1927: 0.0,
            scalar_v1928: false,
            scalar_v1929: 0.0,
            scalar_v1932: 0.0,
            scalar_v1951: 0.0,
            scalar_v1965: 0.0,
            scalar_v1970: false,
            scalar_v1972: false,
            scalar_v1976: 0.0,
            scalar_v1977: 0.0,
            scalar_v1978: 0.0,
            scalar_v1979: 0.0,
            scalar_v1980: 0.0,
            scalar_v1989: 0.0,
            scalar_v1990: false,
            scalar_v1993: false,
            scalar_v2016: 0.0,
            scalar_v2017: 0.0,
            scalar_v2023: 0.0,
            scalar_v2024: 0.0,
            scalar_v2025: 0.0,
            scalar_v2073: false,
            scalar_v2074: false,
            scalar_v2079: 0.0,
            scalar_v2083: 0.0,
            scalar_v2090: 0.0,
            scalar_v2095: 0.0,
            scalar_v2115: 0.0,
            scalar_v2135: 0.0,
            scalar_v2136: false,
            scalar_v2171: false,
            scalar_v2177: false,
            scalar_v2244: 0.0,
            scalar_v2245: 0.0,
            scalar_v2275: 0.0,
            scalar_v2313: 0.0,
            scalar_v2348: 0.0,
            scalar_v2349: 0.0,
            scalar_v2350: 0.0,
            scalar_v2369: 0.0,
            scalar_v2382: 0.0,
            scalar_v2383: 0.0,
            scalar_v2405: 0.0,
            scalar_v2406: false,
            scalar_v2415: 0.0,
            scalar_v2419: false,
            scalar_v2438: false,
            scalar_v2439: false,
            scalar_v2440: false,
            scalar_v2443: false,
            scalar_v2459: 0.0,
            scalar_v2470: false,
            scalar_v2491: 0.0,
            scalar_v2492: false,
            scalar_v2493: 0.0,
            scalar_v2531: 0.0,
            scalar_v2532: 0.0,
            scalar_v2538: 0.0,
            scalar_v2542: 0.0,
            scalar_v2545: false,
            scalar_v2549: 0.0,
            scalar_v2553: 0.0,
            scalar_v2554: false,
            scalar_v2555: 0.0,
            scalar_v2556: false,
            scalar_v2557: false,
            scalar_v2561: 0.0,
            scalar_v2562: false,
            scalar_v2563: false,
            scalar_v2564: false,
            scalar_v2565: false,
            scalar_v2573: false,
            scalar_v2574: false,
            scalar_v2582: false,
            scalar_v2587: 0.0,
            scalar_v2588: false,
            scalar_v2592: false,
            scalar_v2602: 0.0,
            scalar_v2603: false,
            scalar_v2606: false,
            scalar_v2607: false,
            scalar_v2608: false,
            scalar_v2609: 0.0,
            scalar_v2612: false,
            scalar_v2613: false,
            scalar_v2623: 0.0,
            scalar_v2624: 0.0,
            scalar_v2673: 0.0,
            scalar_v2677: 0.0,
            scalar_v2699: 0.0,
            scalar_v2704: 0.0,
            scalar_v2709: 0.0,
            scalar_v2710: 0.0,
            scalar_v2711: false,
            scalar_v2712: 0.0,
            scalar_v2713: false,
            scalar_v2714: 0.0,
            scalar_v2715: false,
            scalar_v2716: 0.0,
            scalar_v2717: false,
            scalar_v2718: 0.0,
            scalar_v2933: 0.0,
            scalar_v3284: 0.0,
            scalar_v3285: 0.0,
            scalar_v3286: 0.0,
            scalar_v3287: 0.0,
            scalar_v4321: 0.0,
            scalar_v4345: 0.0,
            scalar_v4346: 0.0,
            scalar_v4363: 0.0,
            scalar_v4449: 0.0,
            scalar_v4468: 0.0,
            scalar_v4811: 0.0,
            scalar_v4812: 0.0,
            scalar_v4821: 0.0,
            scalar_v4822: 0.0,
            scalar_v4846: 0.0,
            scalar_v4847: 0.0,
            scalar_v4858: 0.0,
            scalar_v4859: 0.0,
            scalar_v5324: 0.0,
            scalar_v5381: 0.0,
            scalar_v5382: 0.0,
            scalar_v5445: 0.0,
            scalar_v5446: 0.0,
            scalar_v5579: 0.0,
            scalar_v5636: 0.0,
            scalar_v5637: 0.0,
            scalar_v6125: 0.0,
            scalar_v6126: 0.0,
            scalar_v6337: 0.0,
            scalar_v6338: 0.0,
            scalar_v6340: 0.0,
            scalar_v6341: 0.0,
            scalar_v6595: 0.0,
            scalar_v6596: 0.0,
            scalar_v6597: 0.0,
            scalar_v6598: 0.0,
            scalar_v6599: 0.0,
            scalar_v6600: 0.0,
            scalar_v7028: 0.0,
            scalar_v7635: 0.0,
            scalar_v7732: 0.0,
            scalar_v8045: 0.0,
            scalar_v8046: 0.0,
            scalar_v8047: 0.0,
            scalar_v8048: 0.0,
            scalar_v8049: 0.0,
            scalar_v8180: 0.0,
            scalar_v8181: 0.0,
            scalar_v8267: 0.0,
            scalar_v8268: 0.0,
            scalar_v8336: 0.0,
            scalar_v8892: 0.0,
            scalar_v8927: 0.0,
            scalar_v9037: 0.0,
            scalar_v9038: 0.0,
            scalar_v9039: 0.0,
            scalar_v9040: 0.0,
            scalar_v9355: 0.0,
            scalar_v9493: 0.0,
            scalar_v9494: 0.0,
            scalar_v9614: 0.0,
            scalar_v9620: 0.0,
            scalar_v9949: 0.0,
            scalar_v9950: 0.0,
            scalar_v10072: 0.0,
            scalar_v10073: 0.0,
            scalar_v10078: 0.0,
            scalar_v10079: 0.0,
            scalar_v10106: 0.0,
            scalar_v10107: 0.0,
            scalar_v20: 0.0,
            scalar_v724: 0.0,
            scalar_v726: 0.0,
            scalar_v727: 0.0,
            scalar_v2566: 0.0,
            scalar_v2567: 0.0,
            scalar_v2575: 0.0,
            scalar_v2576: 0.0,
            scalar_v2577: 0.0,
            scalar_v9607: 0.0,
            scalar_v9608: 0.0,
            scalar_v9609: 0.0,
            scalar_v9610: 0.0,
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
            scalar_v225,
            scalar_v266,
            scalar_v269,
            scalar_v289,
            scalar_v292,
            scalar_v318,
            scalar_v320,
            scalar_v322,
            scalar_v325,
            scalar_v326,
            scalar_v332,
            scalar_v335,
            scalar_v336,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v349,
            scalar_v350,
            scalar_v356,
            scalar_v357,
            scalar_v361,
            scalar_v362,
            scalar_v366,
            scalar_v368,
            scalar_v369,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v403,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v434,
            scalar_v436,
            scalar_v437,
            scalar_v455,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v465,
            scalar_v470,
            scalar_v471,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v481,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v489,
            scalar_v490,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v502,
            scalar_v507,
            scalar_v508,
            scalar_v509,
            scalar_v511,
            scalar_v515,
            scalar_v516,
            scalar_v521,
            scalar_v522,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v555,
            scalar_v556,
            scalar_v560,
            scalar_v561,
            scalar_v565,
            scalar_v566,
            scalar_v567,
            scalar_v568,
            scalar_v575,
            scalar_v576,
            scalar_v577,
            scalar_v584,
            scalar_v587,
            scalar_v595,
            scalar_v604,
            scalar_v617,
            scalar_v626,
            scalar_v638,
            scalar_v641,
            scalar_v644,
            scalar_v645,
            scalar_v646,
            scalar_v650,
            scalar_v655,
            scalar_v656,
            scalar_v657,
            scalar_v662,
            scalar_v663,
            scalar_v667,
            scalar_v668,
            scalar_v672,
            scalar_v673,
            scalar_v677,
            scalar_v678,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v688,
            scalar_v689,
            scalar_v693,
            scalar_v696,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v719,
            scalar_v721,
            scalar_v723,
            scalar_v725,
            scalar_v728,
            scalar_v734,
            scalar_v736,
            scalar_v742,
            scalar_v744,
            scalar_v750,
            scalar_v800,
            scalar_v805,
            scalar_v935,
            scalar_v988,
            scalar_v989,
            scalar_v990,
            scalar_v1001,
            scalar_v1022,
            scalar_v1023,
            scalar_v1024,
            scalar_v1025,
            scalar_v1026,
            scalar_v1027,
            scalar_v1074,
            scalar_v1084,
            scalar_v1097,
            scalar_v1098,
            scalar_v1102,
            scalar_v1151,
            scalar_v1152,
            scalar_v1153,
            scalar_v1175,
            scalar_v1183,
            scalar_v1184,
            scalar_v1186,
            scalar_v1187,
            scalar_v1188,
            scalar_v1191,
            scalar_v1192,
            scalar_v1197,
            scalar_v1218,
            scalar_v1220,
            scalar_v1249,
            scalar_v1255,
            scalar_v1289,
            scalar_v1311,
            scalar_v1324,
            scalar_v1342,
            scalar_v1405,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1410,
            scalar_v1411,
            scalar_v1412,
            scalar_v1505,
            scalar_v1506,
            scalar_v1507,
            scalar_v1531,
            scalar_v1533,
            scalar_v1534,
            scalar_v1536,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1624,
            scalar_v1626,
            scalar_v1627,
            scalar_v1629,
            scalar_v1706,
            scalar_v1707,
            scalar_v1708,
            scalar_v1709,
            scalar_v1715,
            scalar_v1724,
            scalar_v1725,
            scalar_v1737,
            scalar_v1756,
            scalar_v1766,
            scalar_v1767,
            scalar_v1768,
            scalar_v1769,
            scalar_v1774,
            scalar_v1784,
            scalar_v1785,
            scalar_v1786,
            scalar_v1800,
            scalar_v1808,
            scalar_v1809,
            scalar_v1822,
            scalar_v1827,
            scalar_v1844,
            scalar_v1845,
            scalar_v1851,
            scalar_v1852,
            scalar_v1855,
            scalar_v1861,
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
            scalar_v1883,
            scalar_v1884,
            scalar_v1885,
            scalar_v1886,
            scalar_v1900,
            scalar_v1927,
            scalar_v1928,
            scalar_v1929,
            scalar_v1932,
            scalar_v1951,
            scalar_v1965,
            scalar_v1970,
            scalar_v1972,
            scalar_v1976,
            scalar_v1977,
            scalar_v1978,
            scalar_v1979,
            scalar_v1980,
            scalar_v1989,
            scalar_v1990,
            scalar_v1993,
            scalar_v2016,
            scalar_v2017,
            scalar_v2023,
            scalar_v2024,
            scalar_v2025,
            scalar_v2073,
            scalar_v2074,
            scalar_v2079,
            scalar_v2083,
            scalar_v2090,
            scalar_v2095,
            scalar_v2115,
            scalar_v2135,
            scalar_v2136,
            scalar_v2171,
            scalar_v2177,
            scalar_v2244,
            scalar_v2245,
            scalar_v2275,
            scalar_v2313,
            scalar_v2348,
            scalar_v2349,
            scalar_v2350,
            scalar_v2369,
            scalar_v2382,
            scalar_v2383,
            scalar_v2405,
            scalar_v2406,
            scalar_v2415,
            scalar_v2419,
            scalar_v2438,
            scalar_v2439,
            scalar_v2440,
            scalar_v2443,
            scalar_v2459,
            scalar_v2470,
            scalar_v2491,
            scalar_v2492,
            scalar_v2493,
            scalar_v2531,
            scalar_v2532,
            scalar_v2538,
            scalar_v2542,
            scalar_v2545,
            scalar_v2549,
            scalar_v2553,
            scalar_v2554,
            scalar_v2555,
            scalar_v2556,
            scalar_v2557,
            scalar_v2561,
            scalar_v2562,
            scalar_v2563,
            scalar_v2564,
            scalar_v2565,
            scalar_v2573,
            scalar_v2574,
            scalar_v2582,
            scalar_v2587,
            scalar_v2588,
            scalar_v2592,
            scalar_v2602,
            scalar_v2603,
            scalar_v2606,
            scalar_v2607,
            scalar_v2608,
            scalar_v2609,
            scalar_v2612,
            scalar_v2613,
            scalar_v2623,
            scalar_v2624,
            scalar_v2673,
            scalar_v2677,
            scalar_v2699,
            scalar_v2704,
            scalar_v2709,
            scalar_v2710,
            scalar_v2711,
            scalar_v2712,
            scalar_v2713,
            scalar_v2714,
            scalar_v2715,
            scalar_v2716,
            scalar_v2717,
            scalar_v2718,
            scalar_v2933,
            scalar_v3284,
            scalar_v3285,
            scalar_v3286,
            scalar_v3287,
            scalar_v4321,
            scalar_v4345,
            scalar_v4346,
            scalar_v4363,
            scalar_v4449,
            scalar_v4468,
            scalar_v4811,
            scalar_v4812,
            scalar_v4821,
            scalar_v4822,
            scalar_v4846,
            scalar_v4847,
            scalar_v4858,
            scalar_v4859,
            scalar_v5324,
            scalar_v5381,
            scalar_v5382,
            scalar_v5445,
            scalar_v5446,
            scalar_v5579,
            scalar_v5636,
            scalar_v5637,
            scalar_v6125,
            scalar_v6126,
            scalar_v6337,
            scalar_v6338,
            scalar_v6340,
            scalar_v6341,
            scalar_v6595,
            scalar_v6596,
            scalar_v6597,
            scalar_v6598,
            scalar_v6599,
            scalar_v6600,
            scalar_v7028,
            scalar_v7635,
            scalar_v7732,
            scalar_v8045,
            scalar_v8046,
            scalar_v8047,
            scalar_v8048,
            scalar_v8049,
            scalar_v8180,
            scalar_v8181,
            scalar_v8267,
            scalar_v8268,
            scalar_v8336,
            scalar_v8892,
            scalar_v8927,
            scalar_v9037,
            scalar_v9038,
            scalar_v9039,
            scalar_v9040,
            scalar_v9355,
            scalar_v9493,
            scalar_v9494,
            scalar_v9614,
            scalar_v9620,
            scalar_v9949,
            scalar_v9950,
            scalar_v10072,
            scalar_v10073,
            scalar_v10078,
            scalar_v10079,
            scalar_v10106,
            scalar_v10107,
            scalar_v20,
            scalar_v724,
            scalar_v726,
            scalar_v727,
            scalar_v2566,
            scalar_v2567,
            scalar_v2575,
            scalar_v2576,
            scalar_v2577,
            scalar_v9607,
            scalar_v9608,
            scalar_v9609,
            scalar_v9610,
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
            scalar_v225,
            scalar_v266,
            scalar_v269,
            scalar_v289,
            scalar_v292,
            scalar_v318,
            scalar_v320,
            scalar_v322,
            scalar_v325,
            scalar_v326,
            scalar_v332,
            scalar_v335,
            scalar_v336,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v349,
            scalar_v350,
            scalar_v356,
            scalar_v357,
            scalar_v361,
            scalar_v362,
            scalar_v366,
            scalar_v368,
            scalar_v369,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v403,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v434,
            scalar_v436,
            scalar_v437,
            scalar_v455,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v465,
            scalar_v470,
            scalar_v471,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v481,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v489,
            scalar_v490,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v502,
            scalar_v507,
            scalar_v508,
            scalar_v509,
            scalar_v511,
            scalar_v515,
            scalar_v516,
            scalar_v521,
            scalar_v522,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v555,
            scalar_v556,
            scalar_v560,
            scalar_v561,
            scalar_v565,
            scalar_v566,
            scalar_v567,
            scalar_v568,
            scalar_v575,
            scalar_v576,
            scalar_v577,
            scalar_v584,
            scalar_v587,
            scalar_v595,
            scalar_v604,
            scalar_v617,
            scalar_v626,
            scalar_v638,
            scalar_v641,
            scalar_v644,
            scalar_v645,
            scalar_v646,
            scalar_v650,
            scalar_v655,
            scalar_v656,
            scalar_v657,
            scalar_v662,
            scalar_v663,
            scalar_v667,
            scalar_v668,
            scalar_v672,
            scalar_v673,
            scalar_v677,
            scalar_v678,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v688,
            scalar_v689,
            scalar_v693,
            scalar_v696,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v719,
            scalar_v721,
            scalar_v723,
            scalar_v725,
            scalar_v728,
            scalar_v734,
            scalar_v736,
            scalar_v742,
            scalar_v744,
            scalar_v750,
            scalar_v800,
            scalar_v805,
            scalar_v935,
            scalar_v988,
            scalar_v989,
            scalar_v990,
            scalar_v1001,
            scalar_v1022,
            scalar_v1023,
            scalar_v1024,
            scalar_v1025,
            scalar_v1026,
            scalar_v1027,
            scalar_v1074,
            scalar_v1084,
            scalar_v1097,
            scalar_v1098,
            scalar_v1102,
            scalar_v1151,
            scalar_v1152,
            scalar_v1153,
            scalar_v1175,
            scalar_v1183,
            scalar_v1184,
            scalar_v1186,
            scalar_v1187,
            scalar_v1188,
            scalar_v1191,
            scalar_v1192,
            scalar_v1197,
            scalar_v1218,
            scalar_v1220,
            scalar_v1249,
            scalar_v1255,
            scalar_v1289,
            scalar_v1311,
            scalar_v1324,
            scalar_v1342,
            scalar_v1405,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1410,
            scalar_v1411,
            scalar_v1412,
            scalar_v1505,
            scalar_v1506,
            scalar_v1507,
            scalar_v1531,
            scalar_v1533,
            scalar_v1534,
            scalar_v1536,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1624,
            scalar_v1626,
            scalar_v1627,
            scalar_v1629,
            scalar_v1706,
            scalar_v1707,
            scalar_v1708,
            scalar_v1709,
            scalar_v1715,
            scalar_v1724,
            scalar_v1725,
            scalar_v1737,
            scalar_v1756,
            scalar_v1766,
            scalar_v1767,
            scalar_v1768,
            scalar_v1769,
            scalar_v1774,
            scalar_v1784,
            scalar_v1785,
            scalar_v1786,
            scalar_v1800,
            scalar_v1808,
            scalar_v1809,
            scalar_v1822,
            scalar_v1827,
            scalar_v1844,
            scalar_v1845,
            scalar_v1851,
            scalar_v1852,
            scalar_v1855,
            scalar_v1861,
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
            scalar_v1883,
            scalar_v1884,
            scalar_v1885,
            scalar_v1886,
            scalar_v1900,
            scalar_v1927,
            scalar_v1928,
            scalar_v1929,
            scalar_v1932,
            scalar_v1951,
            scalar_v1965,
            scalar_v1970,
            scalar_v1972,
            scalar_v1976,
            scalar_v1977,
            scalar_v1978,
            scalar_v1979,
            scalar_v1980,
            scalar_v1989,
            scalar_v1990,
            scalar_v1993,
            scalar_v2016,
            scalar_v2017,
            scalar_v2023,
            scalar_v2024,
            scalar_v2025,
            scalar_v2073,
            scalar_v2074,
            scalar_v2079,
            scalar_v2083,
            scalar_v2090,
            scalar_v2095,
            scalar_v2115,
            scalar_v2135,
            scalar_v2136,
            scalar_v2171,
            scalar_v2177,
            scalar_v2244,
            scalar_v2245,
            scalar_v2275,
            scalar_v2313,
            scalar_v2348,
            scalar_v2349,
            scalar_v2350,
            scalar_v2369,
            scalar_v2382,
            scalar_v2383,
            scalar_v2405,
            scalar_v2406,
            scalar_v2415,
            scalar_v2419,
            scalar_v2438,
            scalar_v2439,
            scalar_v2440,
            scalar_v2443,
            scalar_v2459,
            scalar_v2470,
            scalar_v2491,
            scalar_v2492,
            scalar_v2493,
            scalar_v2531,
            scalar_v2532,
            scalar_v2538,
            scalar_v2542,
            scalar_v2545,
            scalar_v2549,
            scalar_v2553,
            scalar_v2554,
            scalar_v2555,
            scalar_v2556,
            scalar_v2557,
            scalar_v2561,
            scalar_v2562,
            scalar_v2563,
            scalar_v2564,
            scalar_v2565,
            scalar_v2573,
            scalar_v2574,
            scalar_v2582,
            scalar_v2587,
            scalar_v2588,
            scalar_v2592,
            scalar_v2602,
            scalar_v2603,
            scalar_v2606,
            scalar_v2607,
            scalar_v2608,
            scalar_v2609,
            scalar_v2612,
            scalar_v2613,
            scalar_v2623,
            scalar_v2624,
            scalar_v2673,
            scalar_v2677,
            scalar_v2699,
            scalar_v2704,
            scalar_v2709,
            scalar_v2710,
            scalar_v2711,
            scalar_v2712,
            scalar_v2713,
            scalar_v2714,
            scalar_v2715,
            scalar_v2716,
            scalar_v2717,
            scalar_v2718,
            scalar_v2933,
            scalar_v3284,
            scalar_v3285,
            scalar_v3286,
            scalar_v3287,
            scalar_v4321,
            scalar_v4345,
            scalar_v4346,
            scalar_v4363,
            scalar_v4449,
            scalar_v4468,
            scalar_v4811,
            scalar_v4812,
            scalar_v4821,
            scalar_v4822,
            scalar_v4846,
            scalar_v4847,
            scalar_v4858,
            scalar_v4859,
            scalar_v5324,
            scalar_v5381,
            scalar_v5382,
            scalar_v5445,
            scalar_v5446,
            scalar_v5579,
            scalar_v5636,
            scalar_v5637,
            scalar_v6125,
            scalar_v6126,
            scalar_v6337,
            scalar_v6338,
            scalar_v6340,
            scalar_v6341,
            scalar_v6595,
            scalar_v6596,
            scalar_v6597,
            scalar_v6598,
            scalar_v6599,
            scalar_v6600,
            scalar_v7028,
            scalar_v7635,
            scalar_v7732,
            scalar_v8045,
            scalar_v8046,
            scalar_v8047,
            scalar_v8048,
            scalar_v8049,
            scalar_v8180,
            scalar_v8181,
            scalar_v8267,
            scalar_v8268,
            scalar_v8336,
            scalar_v8892,
            scalar_v8927,
            scalar_v9037,
            scalar_v9038,
            scalar_v9039,
            scalar_v9040,
            scalar_v9355,
            scalar_v9493,
            scalar_v9494,
            scalar_v9614,
            scalar_v9620,
            scalar_v9949,
            scalar_v9950,
            scalar_v10072,
            scalar_v10073,
            scalar_v10078,
            scalar_v10079,
            scalar_v10106,
            scalar_v10107,
            scalar_v20,
            scalar_v724,
            scalar_v726,
            scalar_v727,
            scalar_v2566,
            scalar_v2567,
            scalar_v2575,
            scalar_v2576,
            scalar_v2577,
            scalar_v9607,
            scalar_v9608,
            scalar_v9609,
            scalar_v9610,
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
        let v225: f64 = p.p80;
        self.scalar_v225 = v225;
        let v266: f64 = p.p27;
        self.scalar_v266 = v266;
        let v269: f64 = p.p109;
        self.scalar_v269 = v269;
        let v289: f64 = p.p138;
        self.scalar_v289 = v289;
        let v292: f64 = p.p140;
        self.scalar_v292 = v292;
        let v318: f64 = p.p65;
        self.scalar_v318 = v318;
        let v320: f64 = p.p137;
        self.scalar_v320 = v320;
        let v322: f64 = p.p139;
        self.scalar_v322 = v322;
        let v325: f64 = p.p75;
        self.scalar_v325 = v325;
        let v326: f64 = (1.0 - p.p75);
        self.scalar_v326 = v326;
        let v332: f64 = p.p70;
        self.scalar_v332 = v332;
        let v335: f64 = p.p54;
        self.scalar_v335 = v335;
        let v336: f64 = p.p97;
        self.scalar_v336 = v336;
        let v342: f64 = p.p56;
        self.scalar_v342 = v342;
        let v343: f64 = p.p98;
        self.scalar_v343 = v343;
        let v344: f64 = p.p96;
        self.scalar_v344 = v344;
        let v345: f64 = (p.p98 - p.p96);
        self.scalar_v345 = v345;
        let v349: f64 = p.p55;
        self.scalar_v349 = v349;
        let v350: f64 = p.p101;
        self.scalar_v350 = v350;
        let v356: f64 = p.p57;
        self.scalar_v356 = v356;
        let v357: f64 = p.p102;
        self.scalar_v357 = v357;
        let v361: f64 = p.p58;
        self.scalar_v361 = v361;
        let v362: f64 = p.p104;
        self.scalar_v362 = v362;
        let v366: f64 = p.p59;
        self.scalar_v366 = v366;
        let v368: f64 = p.p60;
        self.scalar_v368 = v368;
        let v369: f64 = p.p99;
        self.scalar_v369 = v369;
        let v373: f64 = p.p122;
        self.scalar_v373 = v373;
        let v374: bool = (0.0 != p.p122);
        self.scalar_v374 = v374;
        let v375: f64 = p.p10;
        self.scalar_v375 = v375;
        let v403: bool = (!v374);
        self.scalar_v403 = v403;
        let v405: f64 = p.p123;
        self.scalar_v405 = v405;
        let v406: bool = (0.0 != p.p123);
        self.scalar_v406 = v406;
        let v407: f64 = p.p11;
        self.scalar_v407 = v407;
        let v434: bool = (!v406);
        self.scalar_v434 = v434;
        let v436: f64 = p.p43;
        self.scalar_v436 = v436;
        let v437: f64 = p.p124;
        self.scalar_v437 = v437;
        let v455: f64 = p.p9;
        self.scalar_v455 = v455;
        let v457: f64 = (4.0 - p.p98);
        self.scalar_v457 = v457;
        let v458: f64 = (v457 - p.p96);
        self.scalar_v458 = v458;
        let v459: f64 = p.p121;
        self.scalar_v459 = v459;
        let v460: f64 = (v458 + p.p121);
        self.scalar_v460 = v460;
        let v465: f64 = (-p.p105);
        self.scalar_v465 = v465;
        let v470: f64 = p.p12;
        self.scalar_v470 = v470;
        let v471: f64 = (1.0 - p.p98);
        self.scalar_v471 = v471;
        let v475: f64 = p.p30;
        self.scalar_v475 = v475;
        let v476: f64 = p.p103;
        self.scalar_v476 = v476;
        let v477: f64 = (1.0 - p.p103);
        self.scalar_v477 = v477;
        let v481: f64 = p.p20;
        self.scalar_v481 = v481;
        let v483: f64 = p.p21;
        self.scalar_v483 = v483;
        let v484: f64 = (2.0 * p.p21);
        self.scalar_v484 = v484;
        let v485: f64 = (6.0 - v484);
        self.scalar_v485 = v485;
        let v489: f64 = p.p113;
        self.scalar_v489 = v489;
        let v490: f64 = (-p.p113);
        self.scalar_v490 = v490;
        let v495: f64 = p.p31;
        self.scalar_v495 = v495;
        let v496: f64 = p.p32;
        self.scalar_v496 = v496;
        let v497: f64 = (2.0 * p.p32);
        self.scalar_v497 = v497;
        let v498: f64 = (6.0 - v497);
        self.scalar_v498 = v498;
        let v502: f64 = (-p.p110);
        self.scalar_v502 = v502;
        let v507: f64 = p.p16;
        self.scalar_v507 = v507;
        let v508: f64 = (4.0 - p.p97);
        self.scalar_v508 = v508;
        let v509: f64 = (p.p121 + v508);
        self.scalar_v509 = v509;
        let v511: f64 = p.p17;
        self.scalar_v511 = v511;
        let v515: f64 = p.p111;
        self.scalar_v515 = v515;
        let v516: f64 = (-p.p111);
        self.scalar_v516 = v516;
        let v521: f64 = p.p18;
        self.scalar_v521 = v521;
        let v522: f64 = p.p19;
        self.scalar_v522 = v522;
        let v529: f64 = p.p24;
        self.scalar_v529 = v529;
        let v530: bool = (1.0 == p.p24);
        self.scalar_v530 = v530;
        let v531: f64 = p.p25;
        self.scalar_v531 = v531;
        let v532: f64 = p.p107;
        self.scalar_v532 = v532;
        let v533: f64 = (-p.p107);
        self.scalar_v533 = v533;
        let v539: f64 = p.p28;
        self.scalar_v539 = v539;
        let v540: f64 = p.p106;
        self.scalar_v540 = v540;
        let v541: f64 = (-p.p106);
        self.scalar_v541 = v541;
        let v546: f64 = p.p26;
        self.scalar_v546 = v546;
        let v547: f64 = p.p108;
        self.scalar_v547 = v547;
        let v548: f64 = (-p.p108);
        self.scalar_v548 = v548;
        let v554: f64 = p.p29;
        self.scalar_v554 = v554;
        let v555: f64 = (4.0 - p.p103);
        self.scalar_v555 = v555;
        let v556: f64 = (p.p121 + v555);
        self.scalar_v556 = v556;
        let v560: f64 = p.p112;
        self.scalar_v560 = v560;
        let v561: f64 = (-p.p112);
        self.scalar_v561 = v561;
        let v565: f64 = p.p22;
        self.scalar_v565 = v565;
        let v566: f64 = p.p23;
        self.scalar_v566 = v566;
        let v567: f64 = (2.0 * p.p23);
        self.scalar_v567 = v567;
        let v568: f64 = (6.0 - v567);
        self.scalar_v568 = v568;
        let v575: f64 = p.p149;
        self.scalar_v575 = v575;
        let v576: f64 = p.p150;
        self.scalar_v576 = v576;
        let v577: f64 = (4.0 / p.p150);
        self.scalar_v577 = v577;
        let v584: f64 = p.p155;
        self.scalar_v584 = v584;
        let v587: f64 = p.p157;
        self.scalar_v587 = v587;
        let v595: f64 = p.p35;
        self.scalar_v595 = v595;
        let v604: f64 = p.p34;
        self.scalar_v604 = v604;
        let v617: f64 = p.p37;
        self.scalar_v617 = v617;
        let v626: f64 = p.p36;
        self.scalar_v626 = v626;
        let v638: f64 = p.p14;
        self.scalar_v638 = v638;
        let v641: f64 = p.p13;
        self.scalar_v641 = v641;
        let v644: f64 = p.p133;
        self.scalar_v644 = v644;
        let v645: f64 = p.p141;
        self.scalar_v645 = v645;
        let v646: f64 = (4.0 - p.p141);
        self.scalar_v646 = v646;
        let v650: f64 = (-p.p140);
        self.scalar_v650 = v650;
        let v655: f64 = p.p142;
        self.scalar_v655 = v655;
        let v656: f64 = (0.5 * p.p142);
        self.scalar_v656 = v656;
        let v657: f64 = (3.5 - v656);
        self.scalar_v657 = v657;
        let v662: f64 = p.p135;
        self.scalar_v662 = v662;
        let v663: f64 = (1.0 - p.p141);
        self.scalar_v663 = v663;
        let v667: f64 = p.p136;
        self.scalar_v667 = v667;
        let v668: f64 = (1.0 - p.p142);
        self.scalar_v668 = v668;
        let v672: f64 = p.p86;
        self.scalar_v672 = v672;
        let v673: f64 = (p.p98 - 2.0);
        self.scalar_v673 = v673;
        let v677: f64 = p.p120;
        self.scalar_v677 = v677;
        let v678: f64 = (-p.p120);
        self.scalar_v678 = v678;
        let v682: f64 = p.p87;
        self.scalar_v682 = v682;
        let v683: f64 = (p.p98 + p.p96);
        self.scalar_v683 = v683;
        let v684: f64 = (v683 - 1.0);
        self.scalar_v684 = v684;
        let v688: f64 = p.p88;
        self.scalar_v688 = v688;
        let v689: f64 = (p.p99 - 1.0);
        self.scalar_v689 = v689;
        let v693: f64 = p.p89;
        self.scalar_v693 = v693;
        let v696: f64 = (p.p87 + p.p88);
        self.scalar_v696 = v696;
        let v698: f64 = p.p90;
        self.scalar_v698 = v698;
        let v699: f64 = p.p100;
        self.scalar_v699 = v699;
        let v700: f64 = (p.p100 - 1.0);
        self.scalar_v700 = v700;
        let v719: f64 = (v12 * 1.081);
        self.scalar_v719 = v719;
        let v721: f64 = p.p92;
        self.scalar_v721 = v721;
        let v723: f64 = p.p146;
        self.scalar_v723 = v723;
        let v725: f64 = p.p148;
        self.scalar_v725 = v725;
        let v728: bool = (p.p57 > 0.0);
        self.scalar_v728 = v728;
        let v734: bool = (!v728);
        self.scalar_v734 = v734;
        let v736: bool = (p.p58 > 0.0);
        self.scalar_v736 = v736;
        let v742: bool = (!v736);
        self.scalar_v742 = v742;
        let v744: bool = (p.p59 > 0.0);
        self.scalar_v744 = v744;
        let v750: bool = (!v744);
        self.scalar_v750 = v750;
        let v800: f64 = p.p151;
        self.scalar_v800 = v800;
        let v805: f64 = ((p.p151) as f64).exp();
        self.scalar_v805 = v805;
        let v935: f64 = p.p153;
        self.scalar_v935 = v935;
        let v988: f64 = p.p62;
        self.scalar_v988 = v988;
        let v989: f64 = p.p61;
        self.scalar_v989 = v989;
        let v990: f64 = (p.p62 * p.p61);
        self.scalar_v990 = v990;
        let v1001: f64 = p.p63;
        self.scalar_v1001 = v1001;
        let v1022: f64 = (-1.0 / p.p63);
        self.scalar_v1022 = v1022;
        let v1023: f64 = ((v1022) as f64).exp();
        self.scalar_v1023 = v1023;
        let v1024: f64 = (1.0 + v1023);
        self.scalar_v1024 = v1024;
        let v1025: f64 = ((v1024) as f64).ln();
        self.scalar_v1025 = v1025;
        let v1026: f64 = (p.p63 * v1025);
        self.scalar_v1026 = v1026;
        let v1027: f64 = (1.0 + v1026);
        self.scalar_v1027 = v1027;
        let v1074: f64 = p.p152;
        self.scalar_v1074 = v1074;
        let v1084: f64 = (0.5 * p.p61);
        self.scalar_v1084 = v1084;
        let v1097: f64 = p.p73;
        self.scalar_v1097 = v1097;
        let v1098: bool = (0.0 == p.p73);
        self.scalar_v1098 = v1098;
        let v1102: bool = (!v1098);
        self.scalar_v1102 = v1102;
        let v1151: f64 = (-1.0 / p.p67);
        self.scalar_v1151 = v1151;
        let v1152: f64 = f64::powf(3.0, v1151);
        self.scalar_v1152 = v1152;
        let v1153: f64 = (1.0 - v1152);
        self.scalar_v1153 = v1153;
        let v1175: f64 = (1.0 - p.p67);
        self.scalar_v1175 = v1175;
        let v1183: f64 = p.p74;
        self.scalar_v1183 = v1183;
        let v1184: bool = (1.0 == p.p74);
        self.scalar_v1184 = v1184;
        let v1186: bool = (2.0 == p.p74);
        self.scalar_v1186 = v1186;
        let v1187: bool = (!v1184);
        self.scalar_v1187 = v1187;
        let v1188: bool = (v1186 && v1187);
        self.scalar_v1188 = v1188;
        let v1191: bool = (!v1186);
        self.scalar_v1191 = v1191;
        let v1192: bool = (v1187 && v1191);
        self.scalar_v1192 = v1192;
        let v1197: f64 = (-1.0 / p.p72);
        self.scalar_v1197 = v1197;
        let v1218: f64 = p.p76;
        self.scalar_v1218 = v1218;
        let v1220: f64 = (1.0 - p.p72);
        self.scalar_v1220 = v1220;
        let v1249: bool = (0.0 == p.p92);
        self.scalar_v1249 = v1249;
        let v1255: bool = (!v1249);
        self.scalar_v1255 = v1255;
        let v1289: f64 = p.p15;
        self.scalar_v1289 = v1289;
        let v1311: f64 = p.p156;
        self.scalar_v1311 = v1311;
        let v1324: f64 = p.p158;
        self.scalar_v1324 = v1324;
        let v1342: f64 = p.p159;
        self.scalar_v1342 = v1342;
        let v1405: f64 = p.p93;
        self.scalar_v1405 = v1405;
        let v1406: bool = (0.0 == p.p93);
        self.scalar_v1406 = v1406;
        let v1407: bool = (!v530);
        self.scalar_v1407 = v1407;
        let v1408: bool = (v1406 && v1407);
        self.scalar_v1408 = v1408;
        let v1410: bool = (!v1406);
        self.scalar_v1410 = v1410;
        let v1411: bool = (v1407 && v1410);
        self.scalar_v1411 = v1411;
        let v1412: f64 = (1.0 - p.p93);
        self.scalar_v1412 = v1412;
        let v1505: bool = (p.p34 > 0.0);
        self.scalar_v1505 = v1505;
        let v1506: bool = (p.p35 > 0.0);
        self.scalar_v1506 = v1506;
        let v1507: bool = (v1505 && v1506);
        self.scalar_v1507 = v1507;
        let v1531: f64 = (-2.0 - p.p67);
        self.scalar_v1531 = v1531;
        let v1533: f64 = (p.p67 * p.p67);
        self.scalar_v1533 = v1533;
        let v1534: f64 = (1.0 - v1533);
        self.scalar_v1534 = v1534;
        let v1536: f64 = (p.p67 - 1.0);
        self.scalar_v1536 = v1536;
        let v1596: bool = (p.p36 > 0.0);
        self.scalar_v1596 = v1596;
        let v1597: bool = (p.p37 > 0.0);
        self.scalar_v1597 = v1597;
        let v1598: bool = (v1596 && v1597);
        self.scalar_v1598 = v1598;
        let v1624: f64 = (-2.0 - p.p72);
        self.scalar_v1624 = v1624;
        let v1626: f64 = (p.p72 * p.p72);
        self.scalar_v1626 = v1626;
        let v1627: f64 = (1.0 - v1626);
        self.scalar_v1627 = v1627;
        let v1629: f64 = (p.p72 - 1.0);
        self.scalar_v1629 = v1629;
        let v1706: f64 = p.p8;
        self.scalar_v1706 = v1706;
        let v1707: bool = (1.0 == p.p8);
        self.scalar_v1707 = v1707;
        let v1708: f64 = p.p143;
        self.scalar_v1708 = v1708;
        let v1709: f64 = (2.0 * p.p143);
        self.scalar_v1709 = v1709;
        let v1715: f64 = p.p144;
        self.scalar_v1715 = v1715;
        let v1724: f64 = (1.0 - p.p143);
        self.scalar_v1724 = v1724;
        let v1725: f64 = (2.0 * v1724);
        self.scalar_v1725 = v1725;
        let v1737: bool = (!v1707);
        self.scalar_v1737 = v1737;
        let v1756: f64 = (4.0 * p.p144);
        self.scalar_v1756 = v1756;
        let v1766: f64 = p.p5;
        self.scalar_v1766 = v1766;
        let v1767: bool = (p.p5 > 0.0);
        self.scalar_v1767 = v1767;
        let v1768: bool = (p.p33 > 0.0);
        self.scalar_v1768 = v1768;
        let v1769: bool = (v1767 && v1768);
        self.scalar_v1769 = v1769;
        let v1774: f64 = (p.p33 * 2.0);
        self.scalar_v1774 = v1774;
        let v1784: bool = (v1707 && v1769);
        self.scalar_v1784 = v1784;
        let v1785: f64 = (p.p33 * v1724);
        self.scalar_v1785 = v1785;
        let v1786: f64 = (2.0 * v1785);
        self.scalar_v1786 = v1786;
        let v1800: bool = (v1737 && v1769);
        self.scalar_v1800 = v1800;
        let v1808: bool = (1.0 == p.p5);
        self.scalar_v1808 = v1808;
        let v1809: bool = (v1769 && v1808);
        self.scalar_v1809 = v1809;
        let v1822: f64 = (if v1809 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1822 = v1822;
        let v1827: f64 = (0.5 * v1822);
        self.scalar_v1827 = v1827;
        let v1844: bool = (!v1808);
        self.scalar_v1844 = v1844;
        let v1845: bool = (v1769 && v1844);
        self.scalar_v1845 = v1845;
        let v1851: f64 = p.p84;
        self.scalar_v1851 = v1851;
        let v1852: bool = (1.0 == p.p84);
        self.scalar_v1852 = v1852;
        let v1855: f64 = (if v1852 { 1e-12 } else { v1822 });
        self.scalar_v1855 = v1855;
        let v1861: f64 = (0.5 * v1855);
        self.scalar_v1861 = v1861;
        let v1872: f64 = p.p82;
        self.scalar_v1872 = v1872;
        let v1873: f64 = f64::powf(v105, p.p82);
        self.scalar_v1873 = v1873;
        let v1874: f64 = (1.0 - v1873);
        self.scalar_v1874 = v1874;
        let v1875: f64 = (1.0 / v1874);
        self.scalar_v1875 = v1875;
        let v1876: f64 = (if v1852 { v1875 } else { 0.0 });
        self.scalar_v1876 = v1876;
        let v1877: f64 = p.p81;
        self.scalar_v1877 = v1877;
        let v1878: f64 = (v105 * p.p81);
        self.scalar_v1878 = v1878;
        let v1879: f64 = (if v1852 { v1878 } else { 0.0 });
        self.scalar_v1879 = v1879;
        let v1880: f64 = (v1876 * v1876);
        self.scalar_v1880 = v1880;
        let v1881: f64 = (p.p82 - 1.0);
        self.scalar_v1881 = v1881;
        let v1882: f64 = f64::powf(v105, v1881);
        self.scalar_v1882 = v1882;
        let v1883: f64 = (v1880 * v1882);
        self.scalar_v1883 = v1883;
        let v1884: f64 = (p.p82 * v1883);
        self.scalar_v1884 = v1884;
        let v1885: f64 = (v1884 / p.p81);
        self.scalar_v1885 = v1885;
        let v1886: f64 = (if v1852 { v1885 } else { 0.0 });
        self.scalar_v1886 = v1886;
        let v1900: bool = (!v1852);
        self.scalar_v1900 = v1900;
        let v1927: f64 = p.p39;
        self.scalar_v1927 = v1927;
        let v1928: bool = (1.0 == p.p39);
        self.scalar_v1928 = v1928;
        let v1929: f64 = p.p44;
        self.scalar_v1929 = v1929;
        let v1932: f64 = p.p42;
        self.scalar_v1932 = v1932;
        let v1951: f64 = p.p41;
        self.scalar_v1951 = v1951;
        let v1965: f64 = p.p40;
        self.scalar_v1965 = v1965;
        let v1970: bool = (2.0 == p.p39);
        self.scalar_v1970 = v1970;
        let v1972: bool = (!v1928);
        self.scalar_v1972 = v1972;
        let v1976: f64 = p.p46;
        self.scalar_v1976 = v1976;
        let v1977: f64 = (2.0 * p.p46);
        self.scalar_v1977 = v1977;
        let v1978: f64 = p.p45;
        self.scalar_v1978 = v1978;
        let v1979: f64 = (p.p45 * p.p45);
        self.scalar_v1979 = v1979;
        let v1980: f64 = (v1977 / v1979);
        self.scalar_v1980 = v1980;
        let v1989: f64 = p.p7;
        self.scalar_v1989 = v1989;
        let v1990: bool = (0.0 == p.p7);
        self.scalar_v1990 = v1990;
        let v1993: bool = (!v1990);
        self.scalar_v1993 = v1993;
        let v2016: f64 = p.p47;
        self.scalar_v2016 = v2016;
        let v2017: f64 = (2.0 * p.p47);
        self.scalar_v2017 = v2017;
        let v2023: f64 = (1.0 + p.p47);
        self.scalar_v2023 = v2023;
        let v2024: f64 = (1.0 + v2017);
        self.scalar_v2024 = v2024;
        let v2025: f64 = (v2023 / v2024);
        self.scalar_v2025 = v2025;
        let v2073: bool = (3.0 == p.p39);
        self.scalar_v2073 = v2073;
        let v2074: bool = (!v1970);
        self.scalar_v2074 = v2074;
        let v2079: f64 = p.p48;
        self.scalar_v2079 = v2079;
        let v2083: f64 = p.p49;
        self.scalar_v2083 = v2083;
        let v2090: f64 = p.p52;
        self.scalar_v2090 = v2090;
        let v2095: f64 = p.p51;
        self.scalar_v2095 = v2095;
        let v2115: f64 = p.p50;
        self.scalar_v2115 = v2115;
        let v2135: f64 = p.p53;
        self.scalar_v2135 = v2135;
        let v2136: bool = (1.0 == p.p53);
        self.scalar_v2136 = v2136;
        let v2171: bool = (!v2073);
        self.scalar_v2171 = v2171;
        let v2177: bool = (!v2136);
        self.scalar_v2177 = v2177;
        let v2244: f64 = p.p68;
        self.scalar_v2244 = v2244;
        let v2245: f64 = (1.0 - p.p68);
        self.scalar_v2245 = v2245;
        let v2275: f64 = p.p77;
        self.scalar_v2275 = v2275;
        let v2313: f64 = (1.0 - p.p77);
        self.scalar_v2313 = v2313;
        let v2348: f64 = (-1.0 / p.p139);
        self.scalar_v2348 = v2348;
        let v2349: f64 = f64::powf(2.0, v2348);
        self.scalar_v2349 = v2349;
        let v2350: f64 = (1.0 - v2349);
        self.scalar_v2350 = v2350;
        let v2369: f64 = (1.0 - p.p139);
        self.scalar_v2369 = v2369;
        let v2382: f64 = p.p85;
        self.scalar_v2382 = v2382;
        let v2383: f64 = (1.0 / p.p85);
        self.scalar_v2383 = v2383;
        let v2405: f64 = p.p79;
        self.scalar_v2405 = v2405;
        let v2406: bool = (0.0 == p.p79);
        self.scalar_v2406 = v2406;
        let v2415: f64 = p.p91;
        self.scalar_v2415 = v2415;
        let v2419: bool = (!v2406);
        self.scalar_v2419 = v2419;
        let v2438: bool = (3.0 == p.p5);
        self.scalar_v2438 = v2438;
        let v2439: bool = (v1808 || v2438);
        self.scalar_v2439 = v2439;
        let v2440: bool = (v1768 && v2439);
        self.scalar_v2440 = v2440;
        let v2443: bool = (v2406 && v2440);
        self.scalar_v2443 = v2443;
        let v2459: f64 = (p.p33 * 0.5);
        self.scalar_v2459 = v2459;
        let v2470: bool = (v2419 && v2440);
        self.scalar_v2470 = v2470;
        let v2491: f64 = p.p6;
        self.scalar_v2491 = v2491;
        let v2492: bool = (1.0 == p.p6);
        self.scalar_v2492 = v2492;
        let v2493: f64 = (-p.p67);
        self.scalar_v2493 = v2493;
        let v2531: f64 = p.p95;
        self.scalar_v2531 = v2531;
        let v2532: f64 = (1.0 - p.p95);
        self.scalar_v2532 = v2532;
        let v2538: f64 = p.p94;
        self.scalar_v2538 = v2538;
        let v2542: f64 = (1.0 - p.p94);
        self.scalar_v2542 = v2542;
        let v2545: bool = (!v2492);
        self.scalar_v2545 = v2545;
        let v2549: f64 = p.p147;
        self.scalar_v2549 = v2549;
        let v2553: f64 = (1.0 - p.p148);
        self.scalar_v2553 = v2553;
        let v2554: bool = (p.p146 > v28);
        self.scalar_v2554 = v2554;
        let v2555: f64 = p.p145;
        self.scalar_v2555 = v2555;
        let v2556: bool = (0.0 == p.p145);
        self.scalar_v2556 = v2556;
        let v2557: bool = (v2554 && v2556);
        self.scalar_v2557 = v2557;
        let v2561: f64 = ((v2553) as f64).abs();
        self.scalar_v2561 = v2561;
        let v2562: bool = (v2561 < 1e-6);
        self.scalar_v2562 = v2562;
        let v2563: bool = (!v2556);
        self.scalar_v2563 = v2563;
        let v2564: bool = (v2554 && v2563);
        self.scalar_v2564 = v2564;
        let v2565: bool = (v2562 && v2564);
        self.scalar_v2565 = v2565;
        let v2573: bool = (!v2562);
        self.scalar_v2573 = v2573;
        let v2574: bool = (v2564 && v2573);
        self.scalar_v2574 = v2574;
        let v2582: bool = (!v2554);
        self.scalar_v2582 = v2582;
        let v2587: f64 = p.p130;
        self.scalar_v2587 = v2587;
        let v2588: bool = (p.p130 > 0.0);
        self.scalar_v2588 = v2588;
        let v2592: bool = (!v2588);
        self.scalar_v2592 = v2592;
        let v2602: f64 = p.p131;
        self.scalar_v2602 = v2602;
        let v2603: bool = (1.0 == p.p131);
        self.scalar_v2603 = v2603;
        let v2606: bool = (2.0 == p.p131);
        self.scalar_v2606 = v2606;
        let v2607: bool = (!v2603);
        self.scalar_v2607 = v2607;
        let v2608: bool = (v2606 && v2607);
        self.scalar_v2608 = v2608;
        let v2609: f64 = p.p132;
        self.scalar_v2609 = v2609;
        let v2612: bool = (!v2606);
        self.scalar_v2612 = v2612;
        let v2613: bool = (v2607 && v2612);
        self.scalar_v2613 = v2613;
        let v2623: f64 = p.p69;
        self.scalar_v2623 = v2623;
        let v2624: f64 = p.p78;
        self.scalar_v2624 = v2624;
        let v2673: f64 = (p.p3 * p.p69);
        self.scalar_v2673 = v2673;
        let v2677: f64 = (p.p3 * p.p78);
        self.scalar_v2677 = v2677;
        let v2699: f64 = (if v742 { 0.0 } else { 0.0 });
        self.scalar_v2699 = v2699;
        let v2704: f64 = (if v750 { 0.0 } else { 0.0 });
        self.scalar_v2704 = v2704;
        let v2709: f64 = (if v530 { 0.0 } else { 0.0 });
        self.scalar_v2709 = v2709;
        let v2710: f64 = (if v1407 { 0.0 } else { 0.0 });
        self.scalar_v2710 = v2710;
        let v2711: bool = (v736 && v744);
        self.scalar_v2711 = v2711;
        let v2712: f64 = (if v2711 { 0.0 } else { 0.0 });
        self.scalar_v2712 = v2712;
        let v2713: bool = (v736 && v750);
        self.scalar_v2713 = v2713;
        let v2714: f64 = (if v2713 { 0.0 } else { 0.0 });
        self.scalar_v2714 = v2714;
        let v2715: bool = (v742 && v744);
        self.scalar_v2715 = v2715;
        let v2716: f64 = (if v2715 { 0.0 } else { 0.0 });
        self.scalar_v2716 = v2716;
        let v2717: bool = (v742 && v750);
        self.scalar_v2717 = v2717;
        let v2718: f64 = (if v2717 { 0.0 } else { 0.0 });
        self.scalar_v2718 = v2718;
        let v2933: f64 = (p.p139 - 1.0);
        self.scalar_v2933 = v2933;
        let v3284: f64 = (-p.p3);
        self.scalar_v3284 = v3284;
        let v3285: f64 = (p.p3 + v3284);
        self.scalar_v3285 = v3285;
        let v3286: f64 = (v3284 - v3284);
        self.scalar_v3286 = v3286;
        let v3287: f64 = (p.p3 + v3285);
        self.scalar_v3287 = v3287;
        let v4321: f64 = (v1175 - 1.0);
        self.scalar_v4321 = v4321;
        let v4345: f64 = (if v1184 { p.p3 } else { 0.0 });
        self.scalar_v4345 = v4345;
        let v4346: f64 = (if v1184 { v3284 } else { 0.0 });
        self.scalar_v4346 = v4346;
        let v4363: f64 = (v1197 - 1.0);
        self.scalar_v4363 = v4363;
        let v4449: f64 = (p.p76 - 1.0);
        self.scalar_v4449 = v4449;
        let v4468: f64 = (v1220 - 1.0);
        self.scalar_v4468 = v4468;
        let v4811: f64 = (v3284 / 0.0001);
        self.scalar_v4811 = v4811;
        let v4812: f64 = (p.p3 / 0.0001);
        self.scalar_v4812 = v4812;
        let v4821: f64 = (-v4811);
        self.scalar_v4821 = v4821;
        let v4822: f64 = (-v4812);
        self.scalar_v4822 = v4822;
        let v4846: f64 = (v3284 / 0.001);
        self.scalar_v4846 = v4846;
        let v4847: f64 = (p.p3 / 0.001);
        self.scalar_v4847 = v4847;
        let v4858: f64 = (-v4846);
        self.scalar_v4858 = v4858;
        let v4859: f64 = (-v4847);
        self.scalar_v4859 = v4859;
        let v5324: f64 = (v1531 - 1.0);
        self.scalar_v5324 = v5324;
        let v5381: f64 = (v39 * v3284);
        self.scalar_v5381 = v5381;
        let v5382: f64 = (p.p3 * v39);
        self.scalar_v5382 = v5382;
        let v5445: f64 = (0.5 * v3284);
        self.scalar_v5445 = v5445;
        let v5446: f64 = (p.p3 * 0.5);
        self.scalar_v5446 = v5446;
        let v5579: f64 = (v1624 - 1.0);
        self.scalar_v5579 = v5579;
        let v5636: f64 = (p.p3 * v74);
        self.scalar_v5636 = v5636;
        let v5637: f64 = (v74 * v3284);
        self.scalar_v5637 = v5637;
        let v6125: f64 = (p.p3 * v34);
        self.scalar_v6125 = v6125;
        let v6126: f64 = (v34 * v3284);
        self.scalar_v6126 = v6126;
        let v6337: f64 = (if v1809 { v3285 } else { 0.0 });
        self.scalar_v6337 = v6337;
        let v6338: f64 = (if v1809 { v3287 } else { 0.0 });
        self.scalar_v6338 = v6338;
        let v6340: f64 = (if v1809 { v3286 } else { 0.0 });
        self.scalar_v6340 = v6340;
        let v6341: f64 = (if v1809 { v3284 } else { 0.0 });
        self.scalar_v6341 = v6341;
        let v6595: f64 = (if v1852 { p.p3 } else { 0.0 });
        self.scalar_v6595 = v6595;
        let v6596: f64 = (if v1852 { v3285 } else { 0.0 });
        self.scalar_v6596 = v6596;
        let v6597: f64 = (if v1852 { v3284 } else { 0.0 });
        self.scalar_v6597 = v6597;
        let v6598: f64 = (-v6595);
        self.scalar_v6598 = v6598;
        let v6599: f64 = (-v6596);
        self.scalar_v6599 = v6599;
        let v6600: f64 = (-v6597);
        self.scalar_v6600 = v6600;
        let v7028: f64 = (p.p41 - 1.0);
        self.scalar_v7028 = v7028;
        let v7635: f64 = (p.p49 - 1.0);
        self.scalar_v7635 = v7635;
        let v7732: f64 = (p.p50 - 1.0);
        self.scalar_v7732 = v7732;
        let v8045: f64 = (if v530 { p.p3 } else { 0.0 });
        self.scalar_v8045 = v8045;
        let v8046: f64 = (if v530 { v3284 } else { 0.0 });
        self.scalar_v8046 = v8046;
        let v8047: f64 = (if v1407 { p.p3 } else { v8045 });
        self.scalar_v8047 = v8047;
        let v8048: f64 = (if v1407 { 0.0 } else { v8046 });
        self.scalar_v8048 = v8048;
        let v8049: f64 = (if v1407 { v3284 } else { 0.0 });
        self.scalar_v8049 = v8049;
        let v8180: f64 = (0.0 * v3284);
        self.scalar_v8180 = v8180;
        let v8181: f64 = (p.p3 * 0.0);
        self.scalar_v8181 = v8181;
        let v8267: f64 = (0.0 * v3285);
        self.scalar_v8267 = v8267;
        let v8268: f64 = (0.0 * v3286);
        self.scalar_v8268 = v8268;
        let v8336: f64 = (v3286 - v3286);
        self.scalar_v8336 = v8336;
        let v8892: f64 = (v2369 - 1.0);
        self.scalar_v8892 = v8892;
        let v8927: f64 = (v2383 - 1.0);
        self.scalar_v8927 = v8927;
        let v9037: f64 = (p.p3 / p.p91);
        self.scalar_v9037 = v9037;
        let v9038: f64 = (v3285 / p.p91);
        self.scalar_v9038 = v9038;
        let v9039: f64 = (v3286 / p.p91);
        self.scalar_v9039 = v9039;
        let v9040: f64 = (v3284 / p.p91);
        self.scalar_v9040 = v9040;
        let v9355: f64 = (v2493 - 1.0);
        self.scalar_v9355 = v9355;
        let v9493: f64 = (p.p3 * 0.2);
        self.scalar_v9493 = v9493;
        let v9494: f64 = (0.2 * v3284);
        self.scalar_v9494 = v9494;
        let v9614: f64 = (v2553 - 1.0);
        self.scalar_v9614 = v9614;
        let v9620: f64 = (1.0 / v26);
        self.scalar_v9620 = v9620;
        let v9949: f64 = (p.p3 * p.p3);
        self.scalar_v9949 = v9949;
        let v9950: f64 = (p.p3 * v3284);
        self.scalar_v9950 = v9950;
        let v10072: f64 = (p.p3 * v2673);
        self.scalar_v10072 = v10072;
        let v10073: f64 = (v2673 * v3284);
        self.scalar_v10073 = v10073;
        let v10078: f64 = (v2677 * v3284);
        self.scalar_v10078 = v10078;
        let v10079: f64 = (p.p3 * v2677);
        self.scalar_v10079 = v10079;
        let v10106: f64 = (p.p3 * v3285);
        self.scalar_v10106 = v10106;
        let v10107: f64 = (p.p3 * v3286);
        self.scalar_v10107 = v10107;
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
        let v724: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v724 = v724;
        let v726: f64 = f64::powf(self.scalar_v724, self.scalar_v725);
        self.scalar_v726 = v726;
        let v727: f64 = (self.scalar_v723 * self.scalar_v726);
        self.scalar_v727 = v727;
        let v2566: f64 = (self.scalar_v20 / self.scalar_v727);
        self.scalar_v2566 = v2566;
        let v2567: f64 = (self.scalar_v27 * self.scalar_v2566);
        self.scalar_v2567 = v2567;
        let v2575: f64 = (self.scalar_v727 * self.scalar_v2553);
        self.scalar_v2575 = v2575;
        let v2576: f64 = (self.scalar_v20 / self.scalar_v2575);
        self.scalar_v2576 = v2576;
        let v2577: f64 = (self.scalar_v27 * self.scalar_v2576);
        self.scalar_v2577 = v2577;
        let v9607: f64 = (1.0 / self.scalar_v727);
        self.scalar_v9607 = v9607;
        let v9608: f64 = (self.scalar_v27 * self.scalar_v9607);
        self.scalar_v9608 = v9608;
        let v9609: f64 = (if self.scalar_v2557 { self.scalar_v9608 } else { 0.0 });
        self.scalar_v9609 = v9609;
        let v9610: f64 = (1.0 / self.scalar_v20);
        self.scalar_v9610 = v9610;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
