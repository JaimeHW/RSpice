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
            params.p0 = 1e-6;
            params.p1 = 1e-6;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 1.0;
            params.p14 = 1.0;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 1.0;
            params.p18 = 1.0;
            params.p19 = 2.0;
            params.p20 = 1003.0;
            params.p21 = -1.0;
            params.p22 = 1.0;
            validate_parameter("scale", params.p22, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p23 = 0.0;
            validate_parameter("shrink", params.p23, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p24 = -100.0;
            params.p25 = 500.0;
            params.p26 = 0.001;
            validate_parameter("rthresh", params.p26, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p27 = 1.0;
            validate_parameter("imax", params.p27, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p28 = 27.0;
            params.p29 = 0.0;
            params.p30 = 9900000000.0;
            params.p31 = 0.0;
            params.p32 = 9900000000.0;
            params.p33 = 100.0;
            params.p34 = 9900000000.0;
            params.p35 = -100.0;
            params.p36 = 500.0;
            params.p37 = 100.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = 1.0;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.01;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 1.0;
            params.p54 = 2.0;
            params.p55 = 0.0;
            params.p56 = 0.5;
            params.p57 = 0.0;
            params.p58 = 2.0;
            params.p59 = 0.0;
            params.p60 = 4.0;
            params.p61 = 0.4;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 1e-12;
            params.p65 = 0.02;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.9;
            params.p69 = 0.0;
            params.p70 = 1.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.75;
            params.p74 = 0.33;
            params.p75 = -0.5;
            params.p76 = 0.0;
            params.p77 = 1.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.75;
            params.p81 = 0.33;
            params.p82 = -0.5;
            params.p83 = 0.0;
            params.p84 = 1e-6;
            params.p85 = 1.0;
            params.p86 = 0.0;
            params.p87 = 2.0;
            params.p88 = 1.0;
            params.p89 = 0.0;
            params.p90 = 1.12;
            params.p91 = 3.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 0.0;
            params.p100 = 0.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 1000000.0;
            params.p111 = 0.0;
            params.p112 = 0.0;
            params.p113 = 0.0;
            params.p114 = 0.0;
            params.p115 = 0.0;
            params.p116 = 0.0;
            params.p117 = 0.0;
            params.p118 = 0.0;
            params.p119 = 0.0;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = 0.0;
            params.p126 = 0.0;
            params.p127 = 0.0;
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
    pub nodes: [usize; 6],
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 128]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 3]>,
    pub(crate) ddt_state_previous: Box<[f64; 3]>,
    pub(crate) ddt_state_older: Box<[f64; 3]>,
    pub(crate) ddt_state_initialized: Box<[bool; 3]>,
    pub(crate) ddt_derivative_current: Box<[f64; 3]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 3]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: bool,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: bool,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
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
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: bool,
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
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v135: bool,
    pub(crate) scalar_v136: bool,
    pub(crate) scalar_v137: bool,
    pub(crate) scalar_v138: bool,
    pub(crate) scalar_v139: bool,
    pub(crate) scalar_v140: bool,
    pub(crate) scalar_v141: bool,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v153: bool,
    pub(crate) scalar_v154: bool,
    pub(crate) scalar_v155: bool,
    pub(crate) scalar_v156: bool,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v166: bool,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v168: bool,
    pub(crate) scalar_v169: bool,
    pub(crate) scalar_v170: bool,
    pub(crate) scalar_v171: bool,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v182: bool,
    pub(crate) scalar_v183: bool,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v190: bool,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: bool,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v263: bool,
    pub(crate) scalar_v264: bool,
    pub(crate) scalar_v265: bool,
    pub(crate) scalar_v270: bool,
    pub(crate) scalar_v271: bool,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v282: bool,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: bool,
    pub(crate) scalar_v312: bool,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: bool,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: bool,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v375: f64,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v383: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v390: f64,
    pub(crate) scalar_v439: bool,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: bool,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v482: bool,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: bool,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v498: bool,
    pub(crate) scalar_v506: bool,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v550: bool,
    pub(crate) scalar_v553: bool,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v588: bool,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v592: bool,
    pub(crate) scalar_v593: f64,
    pub(crate) scalar_v594: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v615: f64,
    pub(crate) scalar_v620: bool,
    pub(crate) scalar_v624: f64,
    pub(crate) scalar_v625: bool,
    pub(crate) scalar_v626: bool,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v628: bool,
    pub(crate) scalar_v629: f64,
    pub(crate) scalar_v636: bool,
    pub(crate) scalar_v637: bool,
    pub(crate) scalar_v641: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v643: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v666: bool,
    pub(crate) scalar_v1165: f64,
    pub(crate) scalar_v1278: f64,
    pub(crate) scalar_v1283: bool,
    pub(crate) scalar_v1287: bool,
    pub(crate) scalar_v1308: f64,
    pub(crate) scalar_v1309: f64,
    pub(crate) scalar_v1310: bool,
    pub(crate) scalar_v1320: bool,
    pub(crate) scalar_v1322: f64,
    pub(crate) scalar_v1335: bool,
    pub(crate) scalar_v1336: bool,
    pub(crate) scalar_v1346: bool,
    pub(crate) scalar_v1355: f64,
    pub(crate) scalar_v1358: f64,
    pub(crate) scalar_v1359: bool,
    pub(crate) scalar_v1365: f64,
    pub(crate) scalar_v1366: f64,
    pub(crate) scalar_v1367: f64,
    pub(crate) scalar_v1372: f64,
    pub(crate) scalar_v1375: f64,
    pub(crate) scalar_v1395: bool,
    pub(crate) scalar_v1398: f64,
    pub(crate) scalar_v1399: f64,
    pub(crate) scalar_v1438: f64,
    pub(crate) scalar_v1439: bool,
    pub(crate) scalar_v1445: f64,
    pub(crate) scalar_v1446: f64,
    pub(crate) scalar_v1451: f64,
    pub(crate) scalar_v1454: f64,
    pub(crate) scalar_v1474: bool,
    pub(crate) scalar_v1477: f64,
    pub(crate) scalar_v1478: f64,
    pub(crate) scalar_v1516: bool,
    pub(crate) scalar_v1518: bool,
    pub(crate) scalar_v1519: bool,
    pub(crate) scalar_v1528: bool,
    pub(crate) scalar_v1671: bool,
    pub(crate) scalar_v1681: f64,
    pub(crate) scalar_v1735: f64,
    pub(crate) scalar_v1840: f64,
    pub(crate) scalar_v1892: f64,
    pub(crate) scalar_v3662: f64,
    pub(crate) scalar_v3663: f64,
    pub(crate) scalar_v3853: f64,
    pub(crate) scalar_v3947: f64,
    pub(crate) scalar_v4113: f64,
    pub(crate) scalar_v4702: f64,
    pub(crate) scalar_v4703: f64,
    pub(crate) scalar_v4706: f64,
    pub(crate) scalar_v4707: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v21: bool,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v29: bool,
    pub(crate) scalar_v30: bool,
    pub(crate) scalar_v31: bool,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v1277: bool,
    pub(crate) scalar_v1279: bool,
    pub(crate) scalar_v1280: bool,
    pub(crate) scalar_v1284: bool,
    pub(crate) scalar_v1288: bool,
    pub(crate) scalar_v1289: f64,
    pub(crate) scalar_v1290: bool,
    pub(crate) scalar_v1291: bool,
    pub(crate) scalar_v1292: f64,
    pub(crate) scalar_v1293: f64,
    pub(crate) scalar_v1294: f64,
    pub(crate) scalar_v1295: f64,
    pub(crate) scalar_v1296: f64,
    pub(crate) scalar_v1297: bool,
    pub(crate) scalar_v1298: bool,
    pub(crate) scalar_v1299: bool,
    pub(crate) scalar_v1300: bool,
    pub(crate) scalar_v1301: f64,
    pub(crate) scalar_v1302: f64,
    pub(crate) scalar_v1303: f64,
    pub(crate) scalar_v1304: f64,
    pub(crate) scalar_v1305: f64,
    pub(crate) scalar_v1306: f64,
    pub(crate) scalar_v1307: f64,
    pub(crate) scalar_v1311: bool,
    pub(crate) scalar_v1312: f64,
    pub(crate) scalar_v1321: bool,
    pub(crate) scalar_v1328: bool,
    pub(crate) scalar_v3851: f64,
    pub(crate) scalar_v3852: f64,
    pub(crate) scalar_v3860: f64,
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
            scalar_v4: self.scalar_v4,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v16: self.scalar_v16,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
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
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
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
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v144: self.scalar_v144,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v159: self.scalar_v159,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v185: self.scalar_v185,
            scalar_v187: self.scalar_v187,
            scalar_v190: self.scalar_v190,
            scalar_v193: self.scalar_v193,
            scalar_v196: self.scalar_v196,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v204: self.scalar_v204,
            scalar_v208: self.scalar_v208,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v231: self.scalar_v231,
            scalar_v234: self.scalar_v234,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v279: self.scalar_v279,
            scalar_v282: self.scalar_v282,
            scalar_v294: self.scalar_v294,
            scalar_v303: self.scalar_v303,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v314: self.scalar_v314,
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
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
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
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v389: self.scalar_v389,
            scalar_v390: self.scalar_v390,
            scalar_v439: self.scalar_v439,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v457: self.scalar_v457,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v466: self.scalar_v466,
            scalar_v470: self.scalar_v470,
            scalar_v476: self.scalar_v476,
            scalar_v482: self.scalar_v482,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v498: self.scalar_v498,
            scalar_v506: self.scalar_v506,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v515: self.scalar_v515,
            scalar_v546: self.scalar_v546,
            scalar_v550: self.scalar_v550,
            scalar_v553: self.scalar_v553,
            scalar_v554: self.scalar_v554,
            scalar_v555: self.scalar_v555,
            scalar_v559: self.scalar_v559,
            scalar_v584: self.scalar_v584,
            scalar_v588: self.scalar_v588,
            scalar_v591: self.scalar_v591,
            scalar_v592: self.scalar_v592,
            scalar_v593: self.scalar_v593,
            scalar_v594: self.scalar_v594,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v614: self.scalar_v614,
            scalar_v615: self.scalar_v615,
            scalar_v620: self.scalar_v620,
            scalar_v624: self.scalar_v624,
            scalar_v625: self.scalar_v625,
            scalar_v626: self.scalar_v626,
            scalar_v627: self.scalar_v627,
            scalar_v628: self.scalar_v628,
            scalar_v629: self.scalar_v629,
            scalar_v636: self.scalar_v636,
            scalar_v637: self.scalar_v637,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v643: self.scalar_v643,
            scalar_v648: self.scalar_v648,
            scalar_v666: self.scalar_v666,
            scalar_v1165: self.scalar_v1165,
            scalar_v1278: self.scalar_v1278,
            scalar_v1283: self.scalar_v1283,
            scalar_v1287: self.scalar_v1287,
            scalar_v1308: self.scalar_v1308,
            scalar_v1309: self.scalar_v1309,
            scalar_v1310: self.scalar_v1310,
            scalar_v1320: self.scalar_v1320,
            scalar_v1322: self.scalar_v1322,
            scalar_v1335: self.scalar_v1335,
            scalar_v1336: self.scalar_v1336,
            scalar_v1346: self.scalar_v1346,
            scalar_v1355: self.scalar_v1355,
            scalar_v1358: self.scalar_v1358,
            scalar_v1359: self.scalar_v1359,
            scalar_v1365: self.scalar_v1365,
            scalar_v1366: self.scalar_v1366,
            scalar_v1367: self.scalar_v1367,
            scalar_v1372: self.scalar_v1372,
            scalar_v1375: self.scalar_v1375,
            scalar_v1395: self.scalar_v1395,
            scalar_v1398: self.scalar_v1398,
            scalar_v1399: self.scalar_v1399,
            scalar_v1438: self.scalar_v1438,
            scalar_v1439: self.scalar_v1439,
            scalar_v1445: self.scalar_v1445,
            scalar_v1446: self.scalar_v1446,
            scalar_v1451: self.scalar_v1451,
            scalar_v1454: self.scalar_v1454,
            scalar_v1474: self.scalar_v1474,
            scalar_v1477: self.scalar_v1477,
            scalar_v1478: self.scalar_v1478,
            scalar_v1516: self.scalar_v1516,
            scalar_v1518: self.scalar_v1518,
            scalar_v1519: self.scalar_v1519,
            scalar_v1528: self.scalar_v1528,
            scalar_v1671: self.scalar_v1671,
            scalar_v1681: self.scalar_v1681,
            scalar_v1735: self.scalar_v1735,
            scalar_v1840: self.scalar_v1840,
            scalar_v1892: self.scalar_v1892,
            scalar_v3662: self.scalar_v3662,
            scalar_v3663: self.scalar_v3663,
            scalar_v3853: self.scalar_v3853,
            scalar_v3947: self.scalar_v3947,
            scalar_v4113: self.scalar_v4113,
            scalar_v4702: self.scalar_v4702,
            scalar_v4703: self.scalar_v4703,
            scalar_v4706: self.scalar_v4706,
            scalar_v4707: self.scalar_v4707,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v39: self.scalar_v39,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v215: self.scalar_v215,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
            scalar_v280: self.scalar_v280,
            scalar_v281: self.scalar_v281,
            scalar_v284: self.scalar_v284,
            scalar_v291: self.scalar_v291,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v301: self.scalar_v301,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v1277: self.scalar_v1277,
            scalar_v1279: self.scalar_v1279,
            scalar_v1280: self.scalar_v1280,
            scalar_v1284: self.scalar_v1284,
            scalar_v1288: self.scalar_v1288,
            scalar_v1289: self.scalar_v1289,
            scalar_v1290: self.scalar_v1290,
            scalar_v1291: self.scalar_v1291,
            scalar_v1292: self.scalar_v1292,
            scalar_v1293: self.scalar_v1293,
            scalar_v1294: self.scalar_v1294,
            scalar_v1295: self.scalar_v1295,
            scalar_v1296: self.scalar_v1296,
            scalar_v1297: self.scalar_v1297,
            scalar_v1298: self.scalar_v1298,
            scalar_v1299: self.scalar_v1299,
            scalar_v1300: self.scalar_v1300,
            scalar_v1301: self.scalar_v1301,
            scalar_v1302: self.scalar_v1302,
            scalar_v1303: self.scalar_v1303,
            scalar_v1304: self.scalar_v1304,
            scalar_v1305: self.scalar_v1305,
            scalar_v1306: self.scalar_v1306,
            scalar_v1307: self.scalar_v1307,
            scalar_v1311: self.scalar_v1311,
            scalar_v1312: self.scalar_v1312,
            scalar_v1321: self.scalar_v1321,
            scalar_v1328: self.scalar_v1328,
            scalar_v3851: self.scalar_v3851,
            scalar_v3852: self.scalar_v3852,
            scalar_v3860: self.scalar_v3860,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 2;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 2] = ["i1", "i2"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 128;
    pub const VARIABLE_COUNT: usize = 329;
    pub const DDT_STATE_COUNT: usize = 3;
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
            scalar_v4: 0.0,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v16: 0.0,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: false,
            scalar_v60: 0.0,
            scalar_v61: false,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
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
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: false,
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
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v135: false,
            scalar_v136: false,
            scalar_v137: false,
            scalar_v138: false,
            scalar_v139: false,
            scalar_v140: false,
            scalar_v141: false,
            scalar_v144: 0.0,
            scalar_v151: false,
            scalar_v152: false,
            scalar_v153: false,
            scalar_v154: false,
            scalar_v155: false,
            scalar_v156: false,
            scalar_v159: 0.0,
            scalar_v166: false,
            scalar_v167: false,
            scalar_v168: false,
            scalar_v169: false,
            scalar_v170: false,
            scalar_v171: false,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v182: false,
            scalar_v183: false,
            scalar_v185: 0.0,
            scalar_v187: 0.0,
            scalar_v190: false,
            scalar_v193: 0.0,
            scalar_v196: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v204: 0.0,
            scalar_v208: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v231: 0.0,
            scalar_v234: 0.0,
            scalar_v252: 0.0,
            scalar_v253: false,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v263: false,
            scalar_v264: false,
            scalar_v265: false,
            scalar_v270: false,
            scalar_v271: false,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v279: 0.0,
            scalar_v282: false,
            scalar_v294: 0.0,
            scalar_v303: 0.0,
            scalar_v310: 0.0,
            scalar_v311: false,
            scalar_v312: false,
            scalar_v313: 0.0,
            scalar_v314: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v318: false,
            scalar_v319: 0.0,
            scalar_v320: false,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: false,
            scalar_v324: 0.0,
            scalar_v325: false,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v341: 0.0,
            scalar_v342: 0.0,
            scalar_v343: 0.0,
            scalar_v344: 0.0,
            scalar_v345: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v351: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v374: 0.0,
            scalar_v375: 0.0,
            scalar_v376: 0.0,
            scalar_v377: 0.0,
            scalar_v378: 0.0,
            scalar_v379: 0.0,
            scalar_v380: 0.0,
            scalar_v381: 0.0,
            scalar_v382: 0.0,
            scalar_v383: 0.0,
            scalar_v384: 0.0,
            scalar_v385: 0.0,
            scalar_v386: 0.0,
            scalar_v387: 0.0,
            scalar_v389: 0.0,
            scalar_v390: 0.0,
            scalar_v439: false,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v457: 0.0,
            scalar_v459: 0.0,
            scalar_v460: false,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v466: 0.0,
            scalar_v470: 0.0,
            scalar_v476: 0.0,
            scalar_v482: false,
            scalar_v485: 0.0,
            scalar_v486: false,
            scalar_v487: 0.0,
            scalar_v498: false,
            scalar_v506: false,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v515: 0.0,
            scalar_v546: 0.0,
            scalar_v550: false,
            scalar_v553: false,
            scalar_v554: 0.0,
            scalar_v555: 0.0,
            scalar_v559: 0.0,
            scalar_v584: 0.0,
            scalar_v588: false,
            scalar_v591: 0.0,
            scalar_v592: false,
            scalar_v593: 0.0,
            scalar_v594: 0.0,
            scalar_v604: 0.0,
            scalar_v605: 0.0,
            scalar_v614: 0.0,
            scalar_v615: 0.0,
            scalar_v620: false,
            scalar_v624: 0.0,
            scalar_v625: false,
            scalar_v626: false,
            scalar_v627: 0.0,
            scalar_v628: false,
            scalar_v629: 0.0,
            scalar_v636: false,
            scalar_v637: false,
            scalar_v641: 0.0,
            scalar_v642: 0.0,
            scalar_v643: 0.0,
            scalar_v648: 0.0,
            scalar_v666: false,
            scalar_v1165: 0.0,
            scalar_v1278: 0.0,
            scalar_v1283: false,
            scalar_v1287: false,
            scalar_v1308: 0.0,
            scalar_v1309: 0.0,
            scalar_v1310: false,
            scalar_v1320: false,
            scalar_v1322: 0.0,
            scalar_v1335: false,
            scalar_v1336: false,
            scalar_v1346: false,
            scalar_v1355: 0.0,
            scalar_v1358: 0.0,
            scalar_v1359: false,
            scalar_v1365: 0.0,
            scalar_v1366: 0.0,
            scalar_v1367: 0.0,
            scalar_v1372: 0.0,
            scalar_v1375: 0.0,
            scalar_v1395: false,
            scalar_v1398: 0.0,
            scalar_v1399: 0.0,
            scalar_v1438: 0.0,
            scalar_v1439: false,
            scalar_v1445: 0.0,
            scalar_v1446: 0.0,
            scalar_v1451: 0.0,
            scalar_v1454: 0.0,
            scalar_v1474: false,
            scalar_v1477: 0.0,
            scalar_v1478: 0.0,
            scalar_v1516: false,
            scalar_v1518: false,
            scalar_v1519: false,
            scalar_v1528: false,
            scalar_v1671: false,
            scalar_v1681: 0.0,
            scalar_v1735: 0.0,
            scalar_v1840: 0.0,
            scalar_v1892: 0.0,
            scalar_v3662: 0.0,
            scalar_v3663: 0.0,
            scalar_v3853: 0.0,
            scalar_v3947: 0.0,
            scalar_v4113: 0.0,
            scalar_v4702: 0.0,
            scalar_v4703: 0.0,
            scalar_v4706: 0.0,
            scalar_v4707: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v21: false,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v29: false,
            scalar_v30: false,
            scalar_v31: false,
            scalar_v32: 0.0,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v35: 0.0,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
            scalar_v39: 0.0,
            scalar_v41: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v280: 0.0,
            scalar_v281: 0.0,
            scalar_v284: 0.0,
            scalar_v291: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v301: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v1277: false,
            scalar_v1279: false,
            scalar_v1280: false,
            scalar_v1284: false,
            scalar_v1288: false,
            scalar_v1289: 0.0,
            scalar_v1290: false,
            scalar_v1291: false,
            scalar_v1292: 0.0,
            scalar_v1293: 0.0,
            scalar_v1294: 0.0,
            scalar_v1295: 0.0,
            scalar_v1296: 0.0,
            scalar_v1297: false,
            scalar_v1298: false,
            scalar_v1299: false,
            scalar_v1300: false,
            scalar_v1301: 0.0,
            scalar_v1302: 0.0,
            scalar_v1303: 0.0,
            scalar_v1304: 0.0,
            scalar_v1305: 0.0,
            scalar_v1306: 0.0,
            scalar_v1307: 0.0,
            scalar_v1311: false,
            scalar_v1312: 0.0,
            scalar_v1321: false,
            scalar_v1328: false,
            scalar_v3851: 0.0,
            scalar_v3852: 0.0,
            scalar_v3860: 0.0,
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
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v11,
            scalar_v13,
            scalar_v14,
            scalar_v16,
            scalar_v19,
            scalar_v20,
            scalar_v27,
            scalar_v28,
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
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
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
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v144,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v159,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v174,
            scalar_v175,
            scalar_v182,
            scalar_v183,
            scalar_v185,
            scalar_v187,
            scalar_v190,
            scalar_v193,
            scalar_v196,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v208,
            scalar_v213,
            scalar_v214,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v231,
            scalar_v234,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v270,
            scalar_v271,
            scalar_v274,
            scalar_v275,
            scalar_v279,
            scalar_v282,
            scalar_v294,
            scalar_v303,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
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
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v356,
            scalar_v357,
            scalar_v363,
            scalar_v364,
            scalar_v367,
            scalar_v368,
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
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v389,
            scalar_v390,
            scalar_v439,
            scalar_v443,
            scalar_v444,
            scalar_v457,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v466,
            scalar_v470,
            scalar_v476,
            scalar_v482,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v498,
            scalar_v506,
            scalar_v509,
            scalar_v510,
            scalar_v515,
            scalar_v546,
            scalar_v550,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v559,
            scalar_v584,
            scalar_v588,
            scalar_v591,
            scalar_v592,
            scalar_v593,
            scalar_v594,
            scalar_v604,
            scalar_v605,
            scalar_v614,
            scalar_v615,
            scalar_v620,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v636,
            scalar_v637,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v648,
            scalar_v666,
            scalar_v1165,
            scalar_v1278,
            scalar_v1283,
            scalar_v1287,
            scalar_v1308,
            scalar_v1309,
            scalar_v1310,
            scalar_v1320,
            scalar_v1322,
            scalar_v1335,
            scalar_v1336,
            scalar_v1346,
            scalar_v1355,
            scalar_v1358,
            scalar_v1359,
            scalar_v1365,
            scalar_v1366,
            scalar_v1367,
            scalar_v1372,
            scalar_v1375,
            scalar_v1395,
            scalar_v1398,
            scalar_v1399,
            scalar_v1438,
            scalar_v1439,
            scalar_v1445,
            scalar_v1446,
            scalar_v1451,
            scalar_v1454,
            scalar_v1474,
            scalar_v1477,
            scalar_v1478,
            scalar_v1516,
            scalar_v1518,
            scalar_v1519,
            scalar_v1528,
            scalar_v1671,
            scalar_v1681,
            scalar_v1735,
            scalar_v1840,
            scalar_v1892,
            scalar_v3662,
            scalar_v3663,
            scalar_v3853,
            scalar_v3947,
            scalar_v4113,
            scalar_v4702,
            scalar_v4703,
            scalar_v4706,
            scalar_v4707,
            scalar_v17,
            scalar_v18,
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v39,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v280,
            scalar_v281,
            scalar_v284,
            scalar_v291,
            scalar_v295,
            scalar_v296,
            scalar_v301,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v1277,
            scalar_v1279,
            scalar_v1280,
            scalar_v1284,
            scalar_v1288,
            scalar_v1289,
            scalar_v1290,
            scalar_v1291,
            scalar_v1292,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1296,
            scalar_v1297,
            scalar_v1298,
            scalar_v1299,
            scalar_v1300,
            scalar_v1301,
            scalar_v1302,
            scalar_v1303,
            scalar_v1304,
            scalar_v1305,
            scalar_v1306,
            scalar_v1307,
            scalar_v1311,
            scalar_v1312,
            scalar_v1321,
            scalar_v1328,
            scalar_v3851,
            scalar_v3852,
            scalar_v3860,
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
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v11,
            scalar_v13,
            scalar_v14,
            scalar_v16,
            scalar_v19,
            scalar_v20,
            scalar_v27,
            scalar_v28,
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
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
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
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v144,
            scalar_v151,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v159,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v174,
            scalar_v175,
            scalar_v182,
            scalar_v183,
            scalar_v185,
            scalar_v187,
            scalar_v190,
            scalar_v193,
            scalar_v196,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v208,
            scalar_v213,
            scalar_v214,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v231,
            scalar_v234,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v270,
            scalar_v271,
            scalar_v274,
            scalar_v275,
            scalar_v279,
            scalar_v282,
            scalar_v294,
            scalar_v303,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
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
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v356,
            scalar_v357,
            scalar_v363,
            scalar_v364,
            scalar_v367,
            scalar_v368,
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
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v389,
            scalar_v390,
            scalar_v439,
            scalar_v443,
            scalar_v444,
            scalar_v457,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v466,
            scalar_v470,
            scalar_v476,
            scalar_v482,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v498,
            scalar_v506,
            scalar_v509,
            scalar_v510,
            scalar_v515,
            scalar_v546,
            scalar_v550,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v559,
            scalar_v584,
            scalar_v588,
            scalar_v591,
            scalar_v592,
            scalar_v593,
            scalar_v594,
            scalar_v604,
            scalar_v605,
            scalar_v614,
            scalar_v615,
            scalar_v620,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v636,
            scalar_v637,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v648,
            scalar_v666,
            scalar_v1165,
            scalar_v1278,
            scalar_v1283,
            scalar_v1287,
            scalar_v1308,
            scalar_v1309,
            scalar_v1310,
            scalar_v1320,
            scalar_v1322,
            scalar_v1335,
            scalar_v1336,
            scalar_v1346,
            scalar_v1355,
            scalar_v1358,
            scalar_v1359,
            scalar_v1365,
            scalar_v1366,
            scalar_v1367,
            scalar_v1372,
            scalar_v1375,
            scalar_v1395,
            scalar_v1398,
            scalar_v1399,
            scalar_v1438,
            scalar_v1439,
            scalar_v1445,
            scalar_v1446,
            scalar_v1451,
            scalar_v1454,
            scalar_v1474,
            scalar_v1477,
            scalar_v1478,
            scalar_v1516,
            scalar_v1518,
            scalar_v1519,
            scalar_v1528,
            scalar_v1671,
            scalar_v1681,
            scalar_v1735,
            scalar_v1840,
            scalar_v1892,
            scalar_v3662,
            scalar_v3663,
            scalar_v3853,
            scalar_v3947,
            scalar_v4113,
            scalar_v4702,
            scalar_v4703,
            scalar_v4706,
            scalar_v4707,
            scalar_v17,
            scalar_v18,
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v39,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v280,
            scalar_v281,
            scalar_v284,
            scalar_v291,
            scalar_v295,
            scalar_v296,
            scalar_v301,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v1277,
            scalar_v1279,
            scalar_v1280,
            scalar_v1284,
            scalar_v1288,
            scalar_v1289,
            scalar_v1290,
            scalar_v1291,
            scalar_v1292,
            scalar_v1293,
            scalar_v1294,
            scalar_v1295,
            scalar_v1296,
            scalar_v1297,
            scalar_v1298,
            scalar_v1299,
            scalar_v1300,
            scalar_v1301,
            scalar_v1302,
            scalar_v1303,
            scalar_v1304,
            scalar_v1305,
            scalar_v1306,
            scalar_v1307,
            scalar_v1311,
            scalar_v1312,
            scalar_v1321,
            scalar_v1328,
            scalar_v3851,
            scalar_v3852,
            scalar_v3860,
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
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wd" => { validate_parameter("wd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a1" => { validate_parameter("a1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p1" => { validate_parameter("p1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c1" => { validate_parameter("c1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a2" => { validate_parameter("a2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p2" => { validate_parameter("p2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c2" => { validate_parameter("c2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dta" => { validate_finite_parameter("trise", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsmm_rsh" => { validate_finite_parameter("nsmm_rsh", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsmm_w" => { validate_finite_parameter("nsmm_w", value)?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsmm_l" => { validate_finite_parameter("nsmm_l", value)?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_noise" => { validate_parameter("sw_noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_et" => { validate_parameter("sw_et", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_lin" => { validate_parameter("sw_lin", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_mman" => { validate_parameter("sw_mman", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subversion" => { validate_finite_parameter("subversion", value)?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "level" => { validate_finite_parameter("level", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthresh" => { validate_parameter("rthresh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "imax" => { validate_parameter("imax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_parameter("lmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_finite_parameter("lmax", value)?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_parameter("wmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_finite_parameter("wmax", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "jmax" => { validate_parameter("jmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmax" => { validate_parameter("vmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nwxw" => { validate_finite_parameter("nwxw", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wexw" => { validate_finite_parameter("wexw", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fdrw" => { validate_parameter("fdrw", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fdxwinf" => { validate_finite_parameter("fdxwinf", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xlw" => { validate_finite_parameter("xlw", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dxlsat" => { validate_finite_parameter("dxlsat", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nst" => { validate_parameter("nst", value, Some((0.1, "0.1")), false, Some((5.0, "5.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ats" => { validate_parameter("ats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "atsinf" => { validate_parameter("ats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "atsl" => { validate_parameter("atsl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dfinf" => { validate_parameter("dfinf", value, Some((0.0001, "0.0001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dfw" => { validate_finite_parameter("dfw", value)?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dfl" => { validate_finite_parameter("dfl", value)?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dfwl" => { validate_finite_parameter("dfwl", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_dfgeo" => { validate_parameter("sw_dfgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dp" => { validate_parameter("dp", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dpinf" => { validate_parameter("dp", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dpw" => { validate_finite_parameter("dpw", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dpwe" => { validate_finite_parameter("dpwe", value)?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dpl" => { validate_finite_parameter("dpl", value)?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dple" => { validate_finite_parameter("dple", value)?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dpwl" => { validate_finite_parameter("dpwl", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ecrit" => { validate_parameter("ecrit", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ecorn" => { validate_parameter("ecorn", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_vsatt" => { validate_parameter("sw_vsatt", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_accpo" => { validate_parameter("sw_accpo", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "grpo" => { validate_parameter("grpo", value, Some((0.0, "0.0")), true, Some((0.1, "0.1")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "du" => { validate_parameter("du", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rc" => { validate_parameter("rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcw" => { validate_parameter("rcw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isa" => { validate_parameter("isa", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "na" => { validate_parameter("na", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ca" => { validate_parameter("ca", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cja" => { validate_parameter("cja", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pa" => { validate_parameter("pa", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ma" => { validate_parameter("ma", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aja" => { validate_finite_parameter("aja", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isp" => { validate_parameter("isp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "np" => { validate_parameter("np", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cp" => { validate_parameter("cp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjp" => { validate_parameter("cjp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pp" => { validate_parameter("pp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mp" => { validate_parameter("mp", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajp" => { validate_finite_parameter("ajp", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbv" => { validate_parameter("vbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibv" => { validate_parameter("ibv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_fngeo" => { validate_parameter("sw_fngeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xis" => { validate_finite_parameter("xis", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xvsat" => { validate_finite_parameter("xvsat", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1" => { validate_finite_parameter("tc1", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2" => { validate_finite_parameter("tc2", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1l" => { validate_finite_parameter("tc1l", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2l" => { validate_finite_parameter("tc2l", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1w" => { validate_finite_parameter("tc1w", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2w" => { validate_finite_parameter("tc2w", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1wl" => { validate_finite_parameter("tc1wl", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2wl" => { validate_finite_parameter("tc2wl", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1rc" => { validate_finite_parameter("tc1rc", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2rc" => { validate_finite_parameter("tc2rc", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1dp" => { validate_finite_parameter("tc1dp", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2dp" => { validate_finite_parameter("tc2dp", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1vbv" => { validate_finite_parameter("tc1vbv", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc2vbv" => { validate_finite_parameter("tc2vbv", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1nbv" => { validate_finite_parameter("tc1nbv", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tc1kfn" => { validate_finite_parameter("tc1kfn", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tegth" => { validate_parameter("tegth", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gth0" => { validate_parameter("gth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gthp" => { validate_parameter("gthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gtha" => { validate_parameter("gtha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gthc" => { validate_parameter("gthc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cthp" => { validate_parameter("cthp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctha" => { validate_parameter("ctha", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cthc" => { validate_parameter("cthc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsig_rsh" => { validate_finite_parameter("nsig_rsh", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsig_w" => { validate_finite_parameter("nsig_w", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsig_l" => { validate_finite_parameter("nsig_l", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sig_rsh" => { validate_parameter("sig_rsh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sig_w" => { validate_parameter("sig_w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sig_l" => { validate_parameter("sig_l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "smm_rsh" => { validate_parameter("smm_rsh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "smm_w" => { validate_parameter("smm_w", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "smm_l" => { validate_parameter("smm_l", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_mmgeo" => { validate_parameter("sw_mmgeo", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'r3_cmc'", name)),
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
        let v4: f64 = p.p23;
        self.scalar_v4 = v4;
        let v5: f64 = (0.01 * p.p23);
        self.scalar_v5 = v5;
        let v6: f64 = (1.0 - v5);
        self.scalar_v6 = v6;
        let v7: f64 = p.p22;
        self.scalar_v7 = v7;
        let v8: f64 = (v6 * p.p22);
        self.scalar_v8 = v8;
        let v10: f64 = (v8 * 1000000.0);
        self.scalar_v10 = v10;
        let v11: f64 = (v10 * v10);
        self.scalar_v11 = v11;
        let v13: f64 = p.p28;
        self.scalar_v13 = v13;
        let v14: f64 = (273.15 + p.p28);
        self.scalar_v14 = v14;
        let v16: f64 = p.p9;
        self.scalar_v16 = v16;
        let v19: f64 = p.p35;
        self.scalar_v19 = v19;
        let v20: f64 = (1.0 + p.p35);
        self.scalar_v20 = v20;
        let v27: f64 = p.p36;
        self.scalar_v27 = v27;
        let v28: f64 = (p.p36 - 1.0);
        self.scalar_v28 = v28;
        let v44: f64 = p.p0;
        self.scalar_v44 = v44;
        let v45: f64 = (v10 * p.p0);
        self.scalar_v45 = v45;
        let v46: f64 = p.p1;
        self.scalar_v46 = v46;
        let v47: f64 = (v10 * p.p1);
        self.scalar_v47 = v47;
        let v48: f64 = p.p2;
        self.scalar_v48 = v48;
        let v49: f64 = (v10 * p.p2);
        self.scalar_v49 = v49;
        let v50: f64 = (0.0 * v11);
        self.scalar_v50 = v50;
        let v51: f64 = p.p4;
        self.scalar_v51 = v51;
        let v52: f64 = (v10 * p.p4);
        self.scalar_v52 = v52;
        let v53: f64 = p.p7;
        self.scalar_v53 = v53;
        let v54: f64 = (v10 * p.p7);
        self.scalar_v54 = v54;
        let v55: f64 = (v45 * v47);
        self.scalar_v55 = v55;
        let v57: f64 = (v47 * 2.0);
        self.scalar_v57 = v57;
        let v58: f64 = p.p5;
        self.scalar_v58 = v58;
        let v59: bool = (p.p5 > 0.0);
        self.scalar_v59 = v59;
        let v60: f64 = p.p8;
        self.scalar_v60 = v60;
        let v61: bool = (p.p8 > 0.0);
        self.scalar_v61 = v61;
        let v62: f64 = (v59 + v61);
        self.scalar_v62 = v62;
        let v63: f64 = (v45 * v62);
        self.scalar_v63 = v63;
        let v64: f64 = (v57 + v63);
        self.scalar_v64 = v64;
        let v66: f64 = (v62 * 0.5);
        self.scalar_v66 = v66;
        let v67: f64 = p.p43;
        self.scalar_v67 = v67;
        let v68: f64 = p.p44;
        self.scalar_v68 = v68;
        let v69: f64 = (p.p44 / v45);
        self.scalar_v69 = v69;
        let v70: f64 = (p.p43 + v69);
        self.scalar_v70 = v70;
        let v71: f64 = (v66 * v70);
        self.scalar_v71 = v71;
        let v72: f64 = p.p38;
        self.scalar_v72 = v72;
        let v73: f64 = (v45 + p.p38);
        self.scalar_v73 = v73;
        let v74: f64 = p.p39;
        self.scalar_v74 = v74;
        let v75: f64 = (p.p39 / v45);
        self.scalar_v75 = v75;
        let v76: f64 = (v73 + v75);
        self.scalar_v76 = v76;
        let v77: f64 = p.p42;
        self.scalar_v77 = v77;
        let v78: f64 = (-v45);
        self.scalar_v78 = v78;
        let v79: f64 = p.p41;
        self.scalar_v79 = v79;
        let v80: f64 = (v78 / p.p41);
        self.scalar_v80 = v80;
        let v81: f64 = ((v80) as f64).exp();
        self.scalar_v81 = v81;
        let v82: f64 = (1.0 - v81);
        self.scalar_v82 = v82;
        let v83: f64 = (p.p42 * v82);
        self.scalar_v83 = v83;
        let v84: f64 = (v76 + v83);
        self.scalar_v84 = v84;
        let v85: f64 = p.p40;
        self.scalar_v85 = v85;
        let v86: f64 = (v49 * p.p40);
        self.scalar_v86 = v86;
        let v87: f64 = (v86 / v55);
        self.scalar_v87 = v87;
        let v88: f64 = (1.0 - v87);
        self.scalar_v88 = v88;
        let v89: f64 = (v84 / v88);
        self.scalar_v89 = v89;
        let v90: f64 = (v47 + v71);
        self.scalar_v90 = v90;
        let v91: f64 = p.p127;
        self.scalar_v91 = v91;
        let v92: f64 = (if (p.p127 != 0.0) { v89 } else { 0.0 });
        self.scalar_v92 = v92;
        let v93: f64 = (if (p.p127 != 0.0) { v90 } else { 0.0 });
        self.scalar_v93 = v93;
        let v94: bool = (!(p.p127 != 0.0));
        self.scalar_v94 = v94;
        let v95: f64 = (if v94 { v45 } else { v92 });
        self.scalar_v95 = v95;
        let v96: f64 = (if v94 { v47 } else { v93 });
        self.scalar_v96 = v96;
        let v97: f64 = p.p16;
        self.scalar_v97 = v97;
        let v98: f64 = p.p119;
        self.scalar_v98 = v98;
        let v99: f64 = p.p122;
        self.scalar_v99 = v99;
        let v100: f64 = (p.p119 * p.p122);
        self.scalar_v100 = v100;
        let v101: f64 = (v89 + v100);
        self.scalar_v101 = v101;
        let v102: f64 = p.p11;
        self.scalar_v102 = v102;
        let v103: f64 = p.p125;
        self.scalar_v103 = v103;
        let v104: f64 = (p.p11 * p.p125);
        self.scalar_v104 = v104;
        let v110: f64 = p.p120;
        self.scalar_v110 = v110;
        let v111: f64 = p.p123;
        self.scalar_v111 = v111;
        let v112: f64 = (p.p120 * p.p123);
        self.scalar_v112 = v112;
        let v113: f64 = (v90 + v112);
        self.scalar_v113 = v113;
        let v114: f64 = p.p12;
        self.scalar_v114 = v114;
        let v115: f64 = p.p126;
        self.scalar_v115 = v115;
        let v116: f64 = (p.p12 * p.p126);
        self.scalar_v116 = v116;
        let v122: f64 = p.p118;
        self.scalar_v122 = v122;
        let v123: f64 = p.p121;
        self.scalar_v123 = v123;
        let v124: f64 = (p.p118 * p.p121);
        self.scalar_v124 = v124;
        let v125: f64 = p.p10;
        self.scalar_v125 = v125;
        let v126: f64 = p.p124;
        self.scalar_v126 = v126;
        let v127: f64 = (p.p10 * p.p124);
        self.scalar_v127 = v127;
        let v135: bool = (0.0 != p.p119);
        self.scalar_v135 = v135;
        let v136: bool = (p.p125 > 0.0);
        self.scalar_v136 = v136;
        let v137: bool = (p.p122 > 0.0);
        self.scalar_v137 = v137;
        let v138: bool = (v136 || v137);
        self.scalar_v138 = v138;
        let v139: bool = (v135 && v138);
        self.scalar_v139 = v139;
        let v140: bool = (!(p.p16 != 0.0));
        self.scalar_v140 = v140;
        let v141: bool = (v139 && v140);
        self.scalar_v141 = v141;
        let v144: f64 = (p.p122 * p.p122);
        self.scalar_v144 = v144;
        let v151: bool = (0.0 != p.p120);
        self.scalar_v151 = v151;
        let v152: bool = (p.p126 > 0.0);
        self.scalar_v152 = v152;
        let v153: bool = (p.p123 > 0.0);
        self.scalar_v153 = v153;
        let v154: bool = (v152 || v153);
        self.scalar_v154 = v154;
        let v155: bool = (v151 && v154);
        self.scalar_v155 = v155;
        let v156: bool = (v140 && v155);
        self.scalar_v156 = v156;
        let v159: f64 = (p.p123 * p.p123);
        self.scalar_v159 = v159;
        let v166: bool = (0.0 != p.p118);
        self.scalar_v166 = v166;
        let v167: bool = (p.p124 > 0.0);
        self.scalar_v167 = v167;
        let v168: bool = (p.p121 > 0.0);
        self.scalar_v168 = v168;
        let v169: bool = (v167 || v168);
        self.scalar_v169 = v169;
        let v170: bool = (v166 && v169);
        self.scalar_v170 = v170;
        let v171: bool = (v140 && v170);
        self.scalar_v171 = v171;
        let v174: f64 = (0.01 * p.p118);
        self.scalar_v174 = v174;
        let v175: f64 = (p.p121 * p.p121);
        self.scalar_v175 = v175;
        let v182: bool = (!v170);
        self.scalar_v182 = v182;
        let v183: bool = (v140 && v182);
        self.scalar_v183 = v183;
        let v185: f64 = p.p45;
        self.scalar_v185 = v185;
        let v187: f64 = p.p53;
        self.scalar_v187 = v187;
        let v190: bool = (!(p.p53 != 0.0));
        self.scalar_v190 = v190;
        let v193: f64 = p.p56;
        self.scalar_v193 = v193;
        let v196: f64 = p.p58;
        self.scalar_v196 = v196;
        let v199: f64 = p.p54;
        self.scalar_v199 = v199;
        let v200: f64 = p.p55;
        self.scalar_v200 = v200;
        let v204: f64 = p.p57;
        self.scalar_v204 = v204;
        let v208: f64 = p.p59;
        self.scalar_v208 = v208;
        let v213: f64 = p.p103;
        self.scalar_v213 = v213;
        let v214: f64 = p.p104;
        self.scalar_v214 = v214;
        let v227: f64 = p.p15;
        self.scalar_v227 = v227;
        let v228: f64 = p.p49;
        self.scalar_v228 = v228;
        let v229: f64 = p.p50;
        self.scalar_v229 = v229;
        let v231: f64 = p.p51;
        self.scalar_v231 = v231;
        let v234: f64 = p.p52;
        self.scalar_v234 = v234;
        let v252: f64 = p.p63;
        self.scalar_v252 = v252;
        let v253: bool = (p.p63 > 1.0);
        self.scalar_v253 = v253;
        let v254: f64 = p.p64;
        self.scalar_v254 = v254;
        let v255: f64 = (2.0 * p.p64);
        self.scalar_v255 = v255;
        let v263: bool = (p.p63 > 0.0);
        self.scalar_v263 = v263;
        let v264: bool = (!v253);
        self.scalar_v264 = v264;
        let v265: bool = (v263 && v264);
        self.scalar_v265 = v265;
        let v270: bool = (!v263);
        self.scalar_v270 = v270;
        let v271: bool = (v264 && v270);
        self.scalar_v271 = v271;
        let v274: f64 = p.p47;
        self.scalar_v274 = v274;
        let v275: f64 = p.p48;
        self.scalar_v275 = v275;
        let v279: f64 = p.p46;
        self.scalar_v279 = v279;
        let v282: bool = (p.p63 > 2.0);
        self.scalar_v282 = v282;
        let v294: f64 = (2.0 * p.p46);
        self.scalar_v294 = v294;
        let v303: f64 = p.p37;
        self.scalar_v303 = v303;
        let v310: f64 = p.p66;
        self.scalar_v310 = v310;
        let v311: bool = (p.p66 > 0.0);
        self.scalar_v311 = v311;
        let v312: bool = (v59 && v311);
        self.scalar_v312 = v312;
        let v313: f64 = p.p67;
        self.scalar_v313 = v313;
        let v314: f64 = (p.p67 / v45);
        self.scalar_v314 = v314;
        let v315: f64 = (p.p66 + v314);
        self.scalar_v315 = v315;
        let v316: f64 = (v315 / p.p5);
        self.scalar_v316 = v316;
        let v317: f64 = (if v312 { v316 } else { 0.0 });
        self.scalar_v317 = v317;
        let v318: bool = (!v312);
        self.scalar_v318 = v318;
        let v319: f64 = (if v318 { 0.0 } else { v317 });
        self.scalar_v319 = v319;
        let v320: bool = (v61 && v311);
        self.scalar_v320 = v320;
        let v321: f64 = (v315 / p.p8);
        self.scalar_v321 = v321;
        let v322: f64 = (if v320 { v321 } else { 0.0 });
        self.scalar_v322 = v322;
        let v323: bool = (!v320);
        self.scalar_v323 = v323;
        let v324: f64 = (if v323 { 0.0 } else { v322 });
        self.scalar_v324 = v324;
        let v325: bool = (!(p.p15 != 0.0));
        self.scalar_v325 = v325;
        let v326: f64 = p.p110;
        self.scalar_v326 = v326;
        let v327: f64 = p.p111;
        self.scalar_v327 = v327;
        let v328: f64 = (v64 * p.p111);
        self.scalar_v328 = v328;
        let v329: f64 = (p.p110 + v328);
        self.scalar_v329 = v329;
        let v330: f64 = p.p112;
        self.scalar_v330 = v330;
        let v331: f64 = (v55 * p.p112);
        self.scalar_v331 = v331;
        let v332: f64 = (v329 + v331);
        self.scalar_v332 = v332;
        let v333: f64 = p.p113;
        self.scalar_v333 = v333;
        let v334: f64 = (p.p5 + p.p8);
        self.scalar_v334 = v334;
        let v335: f64 = (p.p113 * v334);
        self.scalar_v335 = v335;
        let v336: f64 = (v332 + v335);
        self.scalar_v336 = v336;
        let v337: f64 = p.p109;
        self.scalar_v337 = v337;
        let v341: f64 = p.p114;
        self.scalar_v341 = v341;
        let v342: f64 = p.p115;
        self.scalar_v342 = v342;
        let v343: f64 = (v64 * p.p115);
        self.scalar_v343 = v343;
        let v344: f64 = (p.p114 + v343);
        self.scalar_v344 = v344;
        let v345: f64 = p.p116;
        self.scalar_v345 = v345;
        let v346: f64 = (v55 * p.p116);
        self.scalar_v346 = v346;
        let v347: f64 = (v344 + v346);
        self.scalar_v347 = v347;
        let v348: f64 = p.p117;
        self.scalar_v348 = v348;
        let v349: f64 = (v334 * p.p117);
        self.scalar_v349 = v349;
        let v350: f64 = (v347 + v349);
        self.scalar_v350 = v350;
        let v351: f64 = (if v325 { v350 } else { 0.0 });
        self.scalar_v351 = v351;
        let v352: f64 = p.p93;
        self.scalar_v352 = v352;
        let v353: f64 = p.p97;
        self.scalar_v353 = v353;
        let v356: f64 = p.p95;
        self.scalar_v356 = v356;
        let v357: f64 = p.p99;
        self.scalar_v357 = v357;
        let v363: f64 = p.p94;
        self.scalar_v363 = v363;
        let v364: f64 = p.p98;
        self.scalar_v364 = v364;
        let v367: f64 = p.p96;
        self.scalar_v367 = v367;
        let v368: f64 = p.p100;
        self.scalar_v368 = v368;
        let v374: f64 = p.p71;
        self.scalar_v374 = v374;
        let v375: f64 = (v50 * p.p71);
        self.scalar_v375 = v375;
        let v376: f64 = p.p78;
        self.scalar_v376 = v376;
        let v377: f64 = (v52 * p.p78);
        self.scalar_v377 = v377;
        let v378: f64 = (v375 + v377);
        self.scalar_v378 = v378;
        let v379: f64 = (v54 * p.p78);
        self.scalar_v379 = v379;
        let v380: f64 = (v375 + v379);
        self.scalar_v380 = v380;
        let v381: f64 = p.p72;
        self.scalar_v381 = v381;
        let v382: f64 = (v50 * p.p72);
        self.scalar_v382 = v382;
        let v383: f64 = p.p79;
        self.scalar_v383 = v383;
        let v384: f64 = (v52 * p.p79);
        self.scalar_v384 = v384;
        let v385: f64 = (v382 + v384);
        self.scalar_v385 = v385;
        let v386: f64 = (v54 * p.p79);
        self.scalar_v386 = v386;
        let v387: f64 = (v382 + v386);
        self.scalar_v387 = v387;
        let v389: f64 = p.p21;
        self.scalar_v389 = v389;
        let v390: f64 = (-p.p21);
        self.scalar_v390 = v390;
        let v439: bool = (!(p.p63 != 0.0));
        self.scalar_v439 = v439;
        let v443: f64 = p.p101;
        self.scalar_v443 = v443;
        let v444: f64 = p.p102;
        self.scalar_v444 = v444;
        let v457: f64 = p.p92;
        self.scalar_v457 = v457;
        let v459: f64 = p.p69;
        self.scalar_v459 = v459;
        let v460: bool = (p.p69 > 0.0);
        self.scalar_v460 = v460;
        let v461: f64 = p.p90;
        self.scalar_v461 = v461;
        let v462: f64 = (-p.p90);
        self.scalar_v462 = v462;
        let v466: f64 = p.p91;
        self.scalar_v466 = v466;
        let v470: f64 = p.p70;
        self.scalar_v470 = v470;
        let v476: f64 = p.p27;
        self.scalar_v476 = v476;
        let v482: bool = (!v460);
        self.scalar_v482 = v482;
        let v485: f64 = p.p76;
        self.scalar_v485 = v485;
        let v486: bool = (p.p76 > 0.0);
        self.scalar_v486 = v486;
        let v487: f64 = p.p77;
        self.scalar_v487 = v487;
        let v498: bool = (!v486);
        self.scalar_v498 = v498;
        let v506: bool = (p.p72 > 0.0);
        self.scalar_v506 = v506;
        let v509: f64 = p.p73;
        self.scalar_v509 = v509;
        let v510: f64 = (0.5 * p.p73);
        self.scalar_v510 = v510;
        let v515: f64 = (p.p73 * -0.5);
        self.scalar_v515 = v515;
        let v546: f64 = p.p74;
        self.scalar_v546 = v546;
        let v550: bool = (!v506);
        self.scalar_v550 = v550;
        let v553: bool = (p.p79 > 0.0);
        self.scalar_v553 = v553;
        let v554: f64 = p.p80;
        self.scalar_v554 = v554;
        let v555: f64 = (0.5 * p.p80);
        self.scalar_v555 = v555;
        let v559: f64 = (-0.5 * p.p80);
        self.scalar_v559 = v559;
        let v584: f64 = p.p81;
        self.scalar_v584 = v584;
        let v588: bool = (!v553);
        self.scalar_v588 = v588;
        let v591: f64 = p.p83;
        self.scalar_v591 = v591;
        let v592: bool = (p.p83 > 0.0);
        self.scalar_v592 = v592;
        let v593: f64 = p.p105;
        self.scalar_v593 = v593;
        let v594: f64 = p.p106;
        self.scalar_v594 = v594;
        let v604: f64 = p.p85;
        self.scalar_v604 = v604;
        let v605: f64 = p.p107;
        self.scalar_v605 = v605;
        let v614: f64 = p.p84;
        self.scalar_v614 = v614;
        let v615: f64 = (p.p27 / p.p84);
        self.scalar_v615 = v615;
        let v620: bool = (!v592);
        self.scalar_v620 = v620;
        let v624: f64 = p.p60;
        self.scalar_v624 = v624;
        let v625: bool = (p.p60 > 0.0);
        self.scalar_v625 = v625;
        let v626: bool = (v325 && v625);
        self.scalar_v626 = v626;
        let v627: f64 = p.p62;
        self.scalar_v627 = v627;
        let v628: bool = (v626 && (p.p62 != 0.0));
        self.scalar_v628 = v628;
        let v629: f64 = p.p61;
        self.scalar_v629 = v629;
        let v636: bool = (!(p.p62 != 0.0));
        self.scalar_v636 = v636;
        let v637: bool = (v626 && v636);
        self.scalar_v637 = v637;
        let v641: f64 = p.p65;
        self.scalar_v641 = v641;
        let v642: f64 = (4.0 * p.p65);
        self.scalar_v642 = v642;
        let v643: f64 = (p.p65 * v642);
        self.scalar_v643 = v643;
        let v648: f64 = (2.0 * p.p65);
        self.scalar_v648 = v648;
        let v666: bool = (!v626);
        self.scalar_v666 = v666;
        let v1165: f64 = (-p.p84);
        self.scalar_v1165 = v1165;
        let v1278: f64 = p.p14;
        self.scalar_v1278 = v1278;
        let v1283: bool = (0.0 == p.p109);
        self.scalar_v1283 = v1283;
        let v1287: bool = (!v1283);
        self.scalar_v1287 = v1287;
        let v1308: f64 = (1.0 + p.p109);
        self.scalar_v1308 = v1308;
        let v1309: f64 = ((v1308) as f64).abs();
        self.scalar_v1309 = v1309;
        let v1310: bool = (v1309 > 0.1);
        self.scalar_v1310 = v1310;
        let v1320: bool = (!v1310);
        self.scalar_v1320 = v1320;
        let v1322: f64 = (0.5 * p.p109);
        self.scalar_v1322 = v1322;
        let v1335: bool = (v385 > 0.0);
        self.scalar_v1335 = v1335;
        let v1336: bool = ((p.p63 != 0.0) && v1335);
        self.scalar_v1336 = v1336;
        let v1346: bool = (v439 && v1335);
        self.scalar_v1346 = v1346;
        let v1355: f64 = p.p68;
        self.scalar_v1355 = v1355;
        let v1358: f64 = p.p75;
        self.scalar_v1358 = v1358;
        let v1359: bool = (p.p75 <= 0.0);
        self.scalar_v1359 = v1359;
        let v1365: f64 = (1.0 - p.p68);
        self.scalar_v1365 = v1365;
        let v1366: f64 = (-p.p74);
        self.scalar_v1366 = v1366;
        let v1367: f64 = f64::powf(v1365, v1366);
        self.scalar_v1367 = v1367;
        let v1372: f64 = (1.0 - p.p74);
        self.scalar_v1372 = v1372;
        let v1375: f64 = (0.5 * p.p74);
        self.scalar_v1375 = v1375;
        let v1395: bool = (!v1359);
        self.scalar_v1395 = v1395;
        let v1398: f64 = (4.0 * p.p75);
        self.scalar_v1398 = v1398;
        let v1399: f64 = (p.p75 * v1398);
        self.scalar_v1399 = v1399;
        let v1438: f64 = p.p82;
        self.scalar_v1438 = v1438;
        let v1439: bool = (p.p82 <= 0.0);
        self.scalar_v1439 = v1439;
        let v1445: f64 = (-p.p81);
        self.scalar_v1445 = v1445;
        let v1446: f64 = f64::powf(v1365, v1445);
        self.scalar_v1446 = v1446;
        let v1451: f64 = (1.0 - p.p81);
        self.scalar_v1451 = v1451;
        let v1454: f64 = (0.5 * p.p81);
        self.scalar_v1454 = v1454;
        let v1474: bool = (!v1439);
        self.scalar_v1474 = v1474;
        let v1477: f64 = (4.0 * p.p82);
        self.scalar_v1477 = v1477;
        let v1478: f64 = (p.p82 * v1477);
        self.scalar_v1478 = v1478;
        let v1516: bool = (!v1335);
        self.scalar_v1516 = v1516;
        let v1518: bool = (v387 > 0.0);
        self.scalar_v1518 = v1518;
        let v1519: bool = ((p.p63 != 0.0) && v1518);
        self.scalar_v1519 = v1519;
        let v1528: bool = (v439 && v1518);
        self.scalar_v1528 = v1528;
        let v1671: bool = (!v1518);
        self.scalar_v1671 = v1671;
        let v1681: f64 = p.p26;
        self.scalar_v1681 = v1681;
        let v1735: f64 = (p.p92 - 1.0);
        self.scalar_v1735 = v1735;
        let v1840: f64 = (p.p74 - 1.0);
        self.scalar_v1840 = v1840;
        let v1892: f64 = (p.p81 - 1.0);
        self.scalar_v1892 = v1892;
        let v3662: f64 = (0.0 * v390);
        self.scalar_v3662 = v3662;
        let v3663: f64 = (0.0 * p.p21);
        self.scalar_v3663 = v3663;
        let v3853: f64 = (v1308 - 1.0);
        self.scalar_v3853 = v3853;
        let v3947: f64 = (v1372 - 1.0);
        self.scalar_v3947 = v3947;
        let v4113: f64 = (v1451 - 1.0);
        self.scalar_v4113 = v4113;
        let v4702: f64 = (v378 * v390);
        self.scalar_v4702 = v4702;
        let v4703: f64 = (v378 * p.p21);
        self.scalar_v4703 = v4703;
        let v4706: f64 = (v380 * v390);
        self.scalar_v4706 = v4706;
        let v4707: f64 = (v380 * p.p21);
        self.scalar_v4707 = v4707;
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
        let v17: f64 = (temperature + self.scalar_v16);
        self.scalar_v17 = v17;
        let v18: f64 = (self.scalar_v17 - 273.15);
        self.scalar_v18 = v18;
        let v21: bool = (self.scalar_v18 < self.scalar_v20);
        self.scalar_v21 = v21;
        let v22: f64 = (self.scalar_v18 - self.scalar_v19);
        self.scalar_v22 = v22;
        let v23: f64 = (self.scalar_v22 - 1.0);
        self.scalar_v23 = v23;
        let v24: f64 = ((self.scalar_v23) as f64).exp();
        self.scalar_v24 = v24;
        let v25: f64 = (self.scalar_v19 + self.scalar_v24);
        self.scalar_v25 = v25;
        let v26: f64 = (if self.scalar_v21 { self.scalar_v25 } else { self.scalar_v18 });
        self.scalar_v26 = v26;
        let v29: bool = (self.scalar_v26 > self.scalar_v28);
        self.scalar_v29 = v29;
        let v30: bool = (!self.scalar_v21);
        self.scalar_v30 = v30;
        let v31: bool = (self.scalar_v29 && self.scalar_v30);
        self.scalar_v31 = v31;
        let v32: f64 = (self.scalar_v27 - self.scalar_v26);
        self.scalar_v32 = v32;
        let v33: f64 = (self.scalar_v32 - 1.0);
        self.scalar_v33 = v33;
        let v34: f64 = ((self.scalar_v33) as f64).exp();
        self.scalar_v34 = v34;
        let v35: f64 = (self.scalar_v27 - self.scalar_v34);
        self.scalar_v35 = v35;
        let v36: f64 = (if self.scalar_v31 { self.scalar_v35 } else { self.scalar_v26 });
        self.scalar_v36 = v36;
        let v37: f64 = (273.15 + self.scalar_v36);
        self.scalar_v37 = v37;
        let v39: f64 = (self.scalar_v37 * 1.3806505e-23);
        self.scalar_v39 = v39;
        let v41: f64 = (self.scalar_v39 / 1.60217653e-19);
        self.scalar_v41 = v41;
        let v42: f64 = (self.scalar_v37 / self.scalar_v14);
        self.scalar_v42 = v42;
        let v43: f64 = (self.scalar_v37 - self.scalar_v14);
        self.scalar_v43 = v43;
        let v215: f64 = (self.scalar_v43 * self.scalar_v214);
        self.scalar_v215 = v215;
        let v216: f64 = (self.scalar_v213 + self.scalar_v215);
        self.scalar_v216 = v216;
        let v217: f64 = (self.scalar_v43 * self.scalar_v216);
        self.scalar_v217 = v217;
        let v218: f64 = (1.0 + self.scalar_v217);
        self.scalar_v218 = v218;
        let v280: f64 = (self.scalar_v41 * self.scalar_v279);
        self.scalar_v280 = v280;
        let v281: f64 = (if self.scalar_v253 { self.scalar_v280 } else { 0.0 });
        self.scalar_v281 = v281;
        let v284: f64 = (self.scalar_v41 * 0.55);
        self.scalar_v284 = v284;
        let v291: f64 = (self.scalar_v41 * 1.1);
        self.scalar_v291 = v291;
        let v295: f64 = (self.scalar_v41 * self.scalar_v294);
        self.scalar_v295 = v295;
        let v296: f64 = (if self.scalar_v265 { self.scalar_v295 } else { self.scalar_v281 });
        self.scalar_v296 = v296;
        let v301: f64 = (if self.scalar_v271 { self.scalar_v280 } else { self.scalar_v296 });
        self.scalar_v301 = v301;
        let v338: f64 = f64::powf(self.scalar_v42, self.scalar_v337);
        self.scalar_v338 = v338;
        let v339: f64 = (self.scalar_v336 * self.scalar_v338);
        self.scalar_v339 = v339;
        let v340: f64 = (if self.scalar_v325 { self.scalar_v339 } else { 0.0 });
        self.scalar_v340 = v340;
        let v1277: bool = (self.scalar_v340 > 0.0);
        self.scalar_v1277 = v1277;
        let v1279: bool = (self.scalar_v1277 && (self.scalar_v1278 != 0.0));
        self.scalar_v1279 = v1279;
        let v1280: bool = (self.scalar_v325 && self.scalar_v1279);
        self.scalar_v1280 = v1280;
        let v1284: bool = (self.scalar_v1280 && self.scalar_v1283);
        self.scalar_v1284 = v1284;
        let v1288: bool = (self.scalar_v1280 && self.scalar_v1287);
        self.scalar_v1288 = v1288;
        let v1289: f64 = (if self.scalar_v1288 { self.scalar_v18 } else { 0.0 });
        self.scalar_v1289 = v1289;
        let v1290: bool = (self.scalar_v1289 < self.scalar_v20);
        self.scalar_v1290 = v1290;
        let v1291: bool = (self.scalar_v1288 && self.scalar_v1290);
        self.scalar_v1291 = v1291;
        let v1292: f64 = (self.scalar_v1289 - self.scalar_v19);
        self.scalar_v1292 = v1292;
        let v1293: f64 = (self.scalar_v1292 - 1.0);
        self.scalar_v1293 = v1293;
        let v1294: f64 = ((self.scalar_v1293) as f64).exp();
        self.scalar_v1294 = v1294;
        let v1295: f64 = (self.scalar_v19 + self.scalar_v1294);
        self.scalar_v1295 = v1295;
        let v1296: f64 = (if self.scalar_v1291 { self.scalar_v1295 } else { self.scalar_v1289 });
        self.scalar_v1296 = v1296;
        let v1297: bool = (self.scalar_v1296 > self.scalar_v28);
        self.scalar_v1297 = v1297;
        let v1298: bool = (!self.scalar_v1290);
        self.scalar_v1298 = v1298;
        let v1299: bool = (self.scalar_v1288 && self.scalar_v1298);
        self.scalar_v1299 = v1299;
        let v1300: bool = (self.scalar_v1297 && self.scalar_v1299);
        self.scalar_v1300 = v1300;
        let v1301: f64 = (self.scalar_v27 - self.scalar_v1296);
        self.scalar_v1301 = v1301;
        let v1302: f64 = (self.scalar_v1301 - 1.0);
        self.scalar_v1302 = v1302;
        let v1303: f64 = ((self.scalar_v1302) as f64).exp();
        self.scalar_v1303 = v1303;
        let v1304: f64 = (self.scalar_v27 - self.scalar_v1303);
        self.scalar_v1304 = v1304;
        let v1305: f64 = (if self.scalar_v1300 { self.scalar_v1304 } else { self.scalar_v1296 });
        self.scalar_v1305 = v1305;
        let v1306: f64 = (273.15 + self.scalar_v1305);
        self.scalar_v1306 = v1306;
        let v1307: f64 = (if self.scalar_v1288 { self.scalar_v1306 } else { 0.0 });
        self.scalar_v1307 = v1307;
        let v1311: bool = (self.scalar_v1288 && self.scalar_v1310);
        self.scalar_v1311 = v1311;
        let v1312: f64 = (self.scalar_v340 * self.scalar_v1307);
        self.scalar_v1312 = v1312;
        let v1321: bool = (self.scalar_v1288 && self.scalar_v1320);
        self.scalar_v1321 = v1321;
        let v1328: bool = (!self.scalar_v1280);
        self.scalar_v1328 = v1328;
        let v3851: f64 = (if self.scalar_v1284 { self.scalar_v340 } else { 0.0 });
        self.scalar_v3851 = v3851;
        let v3852: f64 = (1.0 / self.scalar_v1307);
        self.scalar_v3852 = v3852;
        let v3860: f64 = (self.scalar_v1322 / self.scalar_v1307);
        self.scalar_v3860 = v3860;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
