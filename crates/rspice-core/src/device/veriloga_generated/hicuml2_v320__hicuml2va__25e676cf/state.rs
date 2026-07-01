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
    pub(crate) ddt_state_older: Box<[f64; 20]>,
    pub(crate) ddt_state_initialized: Box<[bool; 20]>,
    pub(crate) ddt_derivative_current: Box<[f64; 20]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 20]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v26: bool,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: bool,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
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
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
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
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: bool,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: bool,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: bool,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: bool,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: bool,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: bool,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: bool,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: bool,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: bool,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: bool,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: bool,
    pub(crate) scalar_v132: bool,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: bool,
    pub(crate) scalar_v135: bool,
    pub(crate) scalar_v136: bool,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v140: bool,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: bool,
    pub(crate) scalar_v143: bool,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: bool,
    pub(crate) scalar_v148: bool,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: bool,
    pub(crate) scalar_v156: bool,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: bool,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: bool,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: bool,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v267: bool,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v272: bool,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: bool,
    pub(crate) scalar_v334: bool,
    pub(crate) scalar_v338: bool,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: bool,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v388: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: bool,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: bool,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v448: bool,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v460: bool,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: bool,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v470: bool,
    pub(crate) scalar_v471: bool,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: bool,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v535: bool,
    pub(crate) scalar_v536: bool,
    pub(crate) scalar_v540: bool,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: bool,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: bool,
    pub(crate) scalar_v565: bool,
    pub(crate) scalar_v566: bool,
    pub(crate) scalar_v567: bool,
    pub(crate) scalar_v583: bool,
    pub(crate) scalar_v584: bool,
    pub(crate) scalar_v585: bool,
    pub(crate) scalar_v586: bool,
    pub(crate) scalar_v587: bool,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v618: f64,
    pub(crate) scalar_v619: f64,
    pub(crate) scalar_v620: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v624: f64,
    pub(crate) scalar_v625: f64,
    pub(crate) scalar_v640: f64,
    pub(crate) scalar_v645: f64,
    pub(crate) scalar_v646: f64,
    pub(crate) scalar_v647: bool,
    pub(crate) scalar_v648: bool,
    pub(crate) scalar_v656: f64,
    pub(crate) scalar_v661: f64,
    pub(crate) scalar_v662: bool,
    pub(crate) scalar_v663: bool,
    pub(crate) scalar_v664: f64,
    pub(crate) scalar_v665: f64,
    pub(crate) scalar_v666: f64,
    pub(crate) scalar_v667: f64,
    pub(crate) scalar_v668: f64,
    pub(crate) scalar_v669: f64,
    pub(crate) scalar_v670: f64,
    pub(crate) scalar_v671: f64,
    pub(crate) scalar_v672: f64,
    pub(crate) scalar_v673: f64,
    pub(crate) scalar_v674: f64,
    pub(crate) scalar_v692: f64,
    pub(crate) scalar_v700: f64,
    pub(crate) scalar_v701: bool,
    pub(crate) scalar_v705: bool,
    pub(crate) scalar_v706: bool,
    pub(crate) scalar_v710: f64,
    pub(crate) scalar_v711: bool,
    pub(crate) scalar_v712: f64,
    pub(crate) scalar_v735: f64,
    pub(crate) scalar_v736: f64,
    pub(crate) scalar_v737: f64,
    pub(crate) scalar_v739: bool,
    pub(crate) scalar_v740: bool,
    pub(crate) scalar_v744: bool,
    pub(crate) scalar_v748: f64,
    pub(crate) scalar_v749: f64,
    pub(crate) scalar_v751: f64,
    pub(crate) scalar_v756: f64,
    pub(crate) scalar_v760: f64,
    pub(crate) scalar_v761: f64,
    pub(crate) scalar_v765: f64,
    pub(crate) scalar_v766: bool,
    pub(crate) scalar_v767: f64,
    pub(crate) scalar_v768: bool,
    pub(crate) scalar_v769: bool,
    pub(crate) scalar_v770: f64,
    pub(crate) scalar_v771: f64,
    pub(crate) scalar_v772: f64,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v774: f64,
    pub(crate) scalar_v775: f64,
    pub(crate) scalar_v776: f64,
    pub(crate) scalar_v777: f64,
    pub(crate) scalar_v778: f64,
    pub(crate) scalar_v779: f64,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v803: f64,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v805: f64,
    pub(crate) scalar_v806: bool,
    pub(crate) scalar_v807: bool,
    pub(crate) scalar_v811: bool,
    pub(crate) scalar_v812: bool,
    pub(crate) scalar_v816: bool,
    pub(crate) scalar_v820: f64,
    pub(crate) scalar_v821: f64,
    pub(crate) scalar_v825: f64,
    pub(crate) scalar_v826: f64,
    pub(crate) scalar_v830: f64,
    pub(crate) scalar_v831: f64,
    pub(crate) scalar_v835: f64,
    pub(crate) scalar_v836: f64,
    pub(crate) scalar_v840: f64,
    pub(crate) scalar_v844: f64,
    pub(crate) scalar_v845: bool,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v847: bool,
    pub(crate) scalar_v848: bool,
    pub(crate) scalar_v849: bool,
    pub(crate) scalar_v850: bool,
    pub(crate) scalar_v895: bool,
    pub(crate) scalar_v896: f64,
    pub(crate) scalar_v925: bool,
    pub(crate) scalar_v929: bool,
    pub(crate) scalar_v947: bool,
    pub(crate) scalar_v948: f64,
    pub(crate) scalar_v973: bool,
    pub(crate) scalar_v977: bool,
    pub(crate) scalar_v981: bool,
    pub(crate) scalar_v1006: bool,
    pub(crate) scalar_v1015: bool,
    pub(crate) scalar_v1040: bool,
    pub(crate) scalar_v1046: bool,
    pub(crate) scalar_v1063: bool,
    pub(crate) scalar_v1072: bool,
    pub(crate) scalar_v1101: bool,
    pub(crate) scalar_v1102: f64,
    pub(crate) scalar_v1126: bool,
    pub(crate) scalar_v1130: bool,
    pub(crate) scalar_v1186: bool,
    pub(crate) scalar_v1187: f64,
    pub(crate) scalar_v1210: bool,
    pub(crate) scalar_v1214: bool,
    pub(crate) scalar_v1228: bool,
    pub(crate) scalar_v1229: f64,
    pub(crate) scalar_v1254: bool,
    pub(crate) scalar_v1258: bool,
    pub(crate) scalar_v1262: f64,
    pub(crate) scalar_v1263: bool,
    pub(crate) scalar_v1264: bool,
    pub(crate) scalar_v1265: f64,
    pub(crate) scalar_v1289: bool,
    pub(crate) scalar_v1293: bool,
    pub(crate) scalar_v1297: f64,
    pub(crate) scalar_v1312: bool,
    pub(crate) scalar_v1313: bool,
    pub(crate) scalar_v1314: f64,
    pub(crate) scalar_v1337: f64,
    pub(crate) scalar_v1338: f64,
    pub(crate) scalar_v1340: bool,
    pub(crate) scalar_v1341: bool,
    pub(crate) scalar_v1345: bool,
    pub(crate) scalar_v1349: bool,
    pub(crate) scalar_v1372: bool,
    pub(crate) scalar_v1373: f64,
    pub(crate) scalar_v1392: bool,
    pub(crate) scalar_v1394: bool,
    pub(crate) scalar_v1412: bool,
    pub(crate) scalar_v1415: f64,
    pub(crate) scalar_v1450: f64,
    pub(crate) scalar_v1460: f64,
    pub(crate) scalar_v1475: f64,
    pub(crate) scalar_v1477: bool,
    pub(crate) scalar_v1480: f64,
    pub(crate) scalar_v1557: f64,
    pub(crate) scalar_v1561: f64,
    pub(crate) scalar_v1609: bool,
    pub(crate) scalar_v1654: bool,
    pub(crate) scalar_v1655: f64,
    pub(crate) scalar_v1692: bool,
    pub(crate) scalar_v1696: f64,
    pub(crate) scalar_v1711: f64,
    pub(crate) scalar_v1712: f64,
    pub(crate) scalar_v1713: f64,
    pub(crate) scalar_v1734: f64,
    pub(crate) scalar_v1739: f64,
    pub(crate) scalar_v1761: f64,
    pub(crate) scalar_v1770: f64,
    pub(crate) scalar_v1780: f64,
    pub(crate) scalar_v1783: f64,
    pub(crate) scalar_v1791: f64,
    pub(crate) scalar_v1792: bool,
    pub(crate) scalar_v1816: bool,
    pub(crate) scalar_v1820: bool,
    pub(crate) scalar_v1829: bool,
    pub(crate) scalar_v1833: f64,
    pub(crate) scalar_v1840: f64,
    pub(crate) scalar_v1843: f64,
    pub(crate) scalar_v1844: f64,
    pub(crate) scalar_v1845: f64,
    pub(crate) scalar_v1846: bool,
    pub(crate) scalar_v1847: bool,
    pub(crate) scalar_v1857: f64,
    pub(crate) scalar_v1861: f64,
    pub(crate) scalar_v1873: f64,
    pub(crate) scalar_v1874: f64,
    pub(crate) scalar_v1891: f64,
    pub(crate) scalar_v1895: f64,
    pub(crate) scalar_v1896: f64,
    pub(crate) scalar_v1897: f64,
    pub(crate) scalar_v1916: bool,
    pub(crate) scalar_v1917: bool,
    pub(crate) scalar_v1918: bool,
    pub(crate) scalar_v1942: f64,
    pub(crate) scalar_v1943: bool,
    pub(crate) scalar_v1948: bool,
    pub(crate) scalar_v1958: f64,
    pub(crate) scalar_v1969: f64,
    pub(crate) scalar_v1978: bool,
    pub(crate) scalar_v1991: f64,
    pub(crate) scalar_v2011: f64,
    pub(crate) scalar_v2029: f64,
    pub(crate) scalar_v2040: bool,
    pub(crate) scalar_v2051: f64,
    pub(crate) scalar_v2099: f64,
    pub(crate) scalar_v2144: f64,
    pub(crate) scalar_v2504: f64,
    pub(crate) scalar_v2855: f64,
    pub(crate) scalar_v2862: bool,
    pub(crate) scalar_v2863: f64,
    pub(crate) scalar_v2881: bool,
    pub(crate) scalar_v2886: f64,
    pub(crate) scalar_v2887: f64,
    pub(crate) scalar_v2910: f64,
    pub(crate) scalar_v2911: bool,
    pub(crate) scalar_v2918: f64,
    pub(crate) scalar_v2935: bool,
    pub(crate) scalar_v2965: f64,
    pub(crate) scalar_v2966: bool,
    pub(crate) scalar_v2982: bool,
    pub(crate) scalar_v2990: f64,
    pub(crate) scalar_v2991: f64,
    pub(crate) scalar_v3011: f64,
    pub(crate) scalar_v3030: f64,
    pub(crate) scalar_v3039: bool,
    pub(crate) scalar_v3040: f64,
    pub(crate) scalar_v3058: bool,
    pub(crate) scalar_v3060: bool,
    pub(crate) scalar_v3078: bool,
    pub(crate) scalar_v3107: f64,
    pub(crate) scalar_v3117: f64,
    pub(crate) scalar_v3136: f64,
    pub(crate) scalar_v3137: f64,
    pub(crate) scalar_v3159: f64,
    pub(crate) scalar_v3160: f64,
    pub(crate) scalar_v3183: f64,
    pub(crate) scalar_v3184: bool,
    pub(crate) scalar_v3187: f64,
    pub(crate) scalar_v3256: f64,
    pub(crate) scalar_v3287: bool,
    pub(crate) scalar_v3320: bool,
    pub(crate) scalar_v3321: f64,
    pub(crate) scalar_v3339: bool,
    pub(crate) scalar_v3464: f64,
    pub(crate) scalar_v3465: bool,
    pub(crate) scalar_v3468: f64,
    pub(crate) scalar_v3537: f64,
    pub(crate) scalar_v3568: bool,
    pub(crate) scalar_v3601: f64,
    pub(crate) scalar_v3602: bool,
    pub(crate) scalar_v3604: bool,
    pub(crate) scalar_v3606: f64,
    pub(crate) scalar_v3675: f64,
    pub(crate) scalar_v3706: bool,
    pub(crate) scalar_v3707: bool,
    pub(crate) scalar_v3742: bool,
    pub(crate) scalar_v3743: f64,
    pub(crate) scalar_v3755: bool,
    pub(crate) scalar_v3756: bool,
    pub(crate) scalar_v3760: bool,
    pub(crate) scalar_v3761: bool,
    pub(crate) scalar_v3763: bool,
    pub(crate) scalar_v3766: bool,
    pub(crate) scalar_v3767: f64,
    pub(crate) scalar_v3785: bool,
    pub(crate) scalar_v3787: bool,
    pub(crate) scalar_v3788: bool,
    pub(crate) scalar_v3789: bool,
    pub(crate) scalar_v3794: bool,
    pub(crate) scalar_v3795: bool,
    pub(crate) scalar_v3796: bool,
    pub(crate) scalar_v3797: bool,
    pub(crate) scalar_v3845: bool,
    pub(crate) scalar_v3846: bool,
    pub(crate) scalar_v3848: bool,
    pub(crate) scalar_v3880: bool,
    pub(crate) scalar_v3887: bool,
    pub(crate) scalar_v3888: bool,
    pub(crate) scalar_v3889: bool,
    pub(crate) scalar_v3890: bool,
    pub(crate) scalar_v3891: bool,
    pub(crate) scalar_v3892: bool,
    pub(crate) scalar_v3893: bool,
    pub(crate) scalar_v3894: bool,
    pub(crate) scalar_v3895: bool,
    pub(crate) scalar_v3896: bool,
    pub(crate) scalar_v3897: bool,
    pub(crate) scalar_v3898: bool,
    pub(crate) scalar_v3899: bool,
    pub(crate) scalar_v3900: f64,
    pub(crate) scalar_v3901: bool,
    pub(crate) scalar_v3902: bool,
    pub(crate) scalar_v3903: bool,
    pub(crate) scalar_v3904: f64,
    pub(crate) scalar_v3905: bool,
    pub(crate) scalar_v3906: bool,
    pub(crate) scalar_v3907: bool,
    pub(crate) scalar_v3908: bool,
    pub(crate) scalar_v3909: f64,
    pub(crate) scalar_v3910: bool,
    pub(crate) scalar_v3914: f64,
    pub(crate) scalar_v3915: bool,
    pub(crate) scalar_v3916: bool,
    pub(crate) scalar_v3917: bool,
    pub(crate) scalar_v3918: bool,
    pub(crate) scalar_v3919: bool,
    pub(crate) scalar_v3926: f64,
    pub(crate) scalar_v3929: f64,
    pub(crate) scalar_v3930: f64,
    pub(crate) scalar_v3931: f64,
    pub(crate) scalar_v3944: f64,
    pub(crate) scalar_v3963: bool,
    pub(crate) scalar_v3968: bool,
    pub(crate) scalar_v3987: f64,
    pub(crate) scalar_v3990: bool,
    pub(crate) scalar_v3995: bool,
    pub(crate) scalar_v3997: bool,
    pub(crate) scalar_v4004: bool,
    pub(crate) scalar_v4011: bool,
    pub(crate) scalar_v4033: bool,
    pub(crate) scalar_v4036: f64,
    pub(crate) scalar_v4037: f64,
    pub(crate) scalar_v5647: f64,
    pub(crate) scalar_v5648: f64,
    pub(crate) scalar_v5651: f64,
    pub(crate) scalar_v5652: f64,
    pub(crate) scalar_v5653: f64,
    pub(crate) scalar_v5698: f64,
    pub(crate) scalar_v5699: f64,
    pub(crate) scalar_v5700: f64,
    pub(crate) scalar_v23551: f64,
    pub(crate) scalar_v23552: f64,
    pub(crate) scalar_v26252: f64,
    pub(crate) scalar_v26253: f64,
    pub(crate) scalar_v28544: f64,
    pub(crate) scalar_v28545: f64,
    pub(crate) scalar_v29048: f64,
    pub(crate) scalar_v29197: f64,
    pub(crate) scalar_v29305: f64,
    pub(crate) scalar_v29306: f64,
    pub(crate) scalar_v29307: f64,
    pub(crate) scalar_v29308: f64,
    pub(crate) scalar_v29309: f64,
    pub(crate) scalar_v29310: f64,
    pub(crate) scalar_v29521: f64,
    pub(crate) scalar_v29522: f64,
    pub(crate) scalar_v29523: f64,
    pub(crate) scalar_v29587: f64,
    pub(crate) scalar_v29588: f64,
    pub(crate) scalar_v29610: f64,
    pub(crate) scalar_v30261: f64,
    pub(crate) scalar_v30268: f64,
    pub(crate) scalar_v30285: f64,
    pub(crate) scalar_v30286: f64,
    pub(crate) scalar_v30287: f64,
    pub(crate) scalar_v30304: f64,
    pub(crate) scalar_v30311: f64,
    pub(crate) scalar_v30328: f64,
    pub(crate) scalar_v30329: f64,
    pub(crate) scalar_v30330: f64,
    pub(crate) scalar_v30331: f64,
    pub(crate) scalar_v30332: f64,
    pub(crate) scalar_v30385: f64,
    pub(crate) scalar_v30517: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v192: bool,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v195: bool,
    pub(crate) scalar_v196: bool,
    pub(crate) scalar_v197: bool,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: f64,
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
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: f64,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v259: f64,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v325: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v383: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v390: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: f64,
    pub(crate) scalar_v395: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v530: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v538: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v549: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v569: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v609: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v628: f64,
    pub(crate) scalar_v629: f64,
    pub(crate) scalar_v630: f64,
    pub(crate) scalar_v631: f64,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v633: f64,
    pub(crate) scalar_v634: f64,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v636: f64,
    pub(crate) scalar_v637: f64,
    pub(crate) scalar_v638: f64,
    pub(crate) scalar_v639: f64,
    pub(crate) scalar_v641: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v643: f64,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v654: f64,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v657: f64,
    pub(crate) scalar_v658: f64,
    pub(crate) scalar_v659: f64,
    pub(crate) scalar_v660: f64,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v676: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v682: f64,
    pub(crate) scalar_v683: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v686: f64,
    pub(crate) scalar_v687: f64,
    pub(crate) scalar_v688: f64,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v690: f64,
    pub(crate) scalar_v691: f64,
    pub(crate) scalar_v693: f64,
    pub(crate) scalar_v694: f64,
    pub(crate) scalar_v695: f64,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v697: f64,
    pub(crate) scalar_v698: f64,
    pub(crate) scalar_v702: f64,
    pub(crate) scalar_v703: f64,
    pub(crate) scalar_v704: f64,
    pub(crate) scalar_v707: f64,
    pub(crate) scalar_v708: f64,
    pub(crate) scalar_v709: f64,
    pub(crate) scalar_v713: f64,
    pub(crate) scalar_v714: f64,
    pub(crate) scalar_v715: f64,
    pub(crate) scalar_v716: f64,
    pub(crate) scalar_v717: f64,
    pub(crate) scalar_v718: f64,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v720: f64,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v722: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v724: f64,
    pub(crate) scalar_v725: f64,
    pub(crate) scalar_v726: f64,
    pub(crate) scalar_v727: f64,
    pub(crate) scalar_v728: f64,
    pub(crate) scalar_v729: f64,
    pub(crate) scalar_v730: f64,
    pub(crate) scalar_v731: f64,
    pub(crate) scalar_v732: f64,
    pub(crate) scalar_v733: f64,
    pub(crate) scalar_v734: f64,
    pub(crate) scalar_v738: f64,
    pub(crate) scalar_v741: f64,
    pub(crate) scalar_v742: f64,
    pub(crate) scalar_v743: f64,
    pub(crate) scalar_v745: f64,
    pub(crate) scalar_v746: f64,
    pub(crate) scalar_v747: f64,
    pub(crate) scalar_v750: f64,
    pub(crate) scalar_v752: f64,
    pub(crate) scalar_v753: f64,
    pub(crate) scalar_v754: f64,
    pub(crate) scalar_v755: f64,
    pub(crate) scalar_v757: f64,
    pub(crate) scalar_v758: f64,
    pub(crate) scalar_v759: f64,
    pub(crate) scalar_v762: f64,
    pub(crate) scalar_v763: f64,
    pub(crate) scalar_v764: f64,
    pub(crate) scalar_v780: f64,
    pub(crate) scalar_v781: f64,
    pub(crate) scalar_v782: f64,
    pub(crate) scalar_v783: f64,
    pub(crate) scalar_v784: f64,
    pub(crate) scalar_v785: f64,
    pub(crate) scalar_v786: f64,
    pub(crate) scalar_v787: f64,
    pub(crate) scalar_v788: f64,
    pub(crate) scalar_v789: f64,
    pub(crate) scalar_v790: f64,
    pub(crate) scalar_v791: f64,
    pub(crate) scalar_v792: f64,
    pub(crate) scalar_v793: f64,
    pub(crate) scalar_v794: f64,
    pub(crate) scalar_v795: f64,
    pub(crate) scalar_v797: f64,
    pub(crate) scalar_v798: f64,
    pub(crate) scalar_v799: f64,
    pub(crate) scalar_v800: f64,
    pub(crate) scalar_v801: f64,
    pub(crate) scalar_v802: f64,
    pub(crate) scalar_v808: f64,
    pub(crate) scalar_v809: f64,
    pub(crate) scalar_v810: f64,
    pub(crate) scalar_v813: f64,
    pub(crate) scalar_v814: f64,
    pub(crate) scalar_v815: f64,
    pub(crate) scalar_v817: f64,
    pub(crate) scalar_v818: f64,
    pub(crate) scalar_v819: f64,
    pub(crate) scalar_v822: f64,
    pub(crate) scalar_v823: f64,
    pub(crate) scalar_v824: f64,
    pub(crate) scalar_v827: f64,
    pub(crate) scalar_v828: f64,
    pub(crate) scalar_v829: f64,
    pub(crate) scalar_v832: f64,
    pub(crate) scalar_v833: f64,
    pub(crate) scalar_v834: f64,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v838: f64,
    pub(crate) scalar_v839: f64,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v842: f64,
    pub(crate) scalar_v843: f64,
    pub(crate) scalar_v924: f64,
    pub(crate) scalar_v972: f64,
    pub(crate) scalar_v1045: f64,
    pub(crate) scalar_v1125: f64,
    pub(crate) scalar_v1209: f64,
    pub(crate) scalar_v1253: f64,
    pub(crate) scalar_v1339: f64,
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
            scalar_v24: self.scalar_v24,
            scalar_v26: self.scalar_v26,
            scalar_v29: self.scalar_v29,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v34: self.scalar_v34,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v39: self.scalar_v39,
            scalar_v41: self.scalar_v41,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
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
            scalar_v56: self.scalar_v56,
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
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v83: self.scalar_v83,
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
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
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
            scalar_v158: self.scalar_v158,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
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
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
            scalar_v219: self.scalar_v219,
            scalar_v220: self.scalar_v220,
            scalar_v222: self.scalar_v222,
            scalar_v223: self.scalar_v223,
            scalar_v224: self.scalar_v224,
            scalar_v225: self.scalar_v225,
            scalar_v226: self.scalar_v226,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v233: self.scalar_v233,
            scalar_v234: self.scalar_v234,
            scalar_v257: self.scalar_v257,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v272: self.scalar_v272,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v279: self.scalar_v279,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v289: self.scalar_v289,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v323: self.scalar_v323,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v338: self.scalar_v338,
            scalar_v344: self.scalar_v344,
            scalar_v346: self.scalar_v346,
            scalar_v351: self.scalar_v351,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v361: self.scalar_v361,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v371: self.scalar_v371,
            scalar_v372: self.scalar_v372,
            scalar_v373: self.scalar_v373,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v378: self.scalar_v378,
            scalar_v380: self.scalar_v380,
            scalar_v388: self.scalar_v388,
            scalar_v391: self.scalar_v391,
            scalar_v392: self.scalar_v392,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v413: self.scalar_v413,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v434: self.scalar_v434,
            scalar_v437: self.scalar_v437,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v460: self.scalar_v460,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v468: self.scalar_v468,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v499: self.scalar_v499,
            scalar_v500: self.scalar_v500,
            scalar_v501: self.scalar_v501,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v504: self.scalar_v504,
            scalar_v505: self.scalar_v505,
            scalar_v506: self.scalar_v506,
            scalar_v507: self.scalar_v507,
            scalar_v508: self.scalar_v508,
            scalar_v525: self.scalar_v525,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v534: self.scalar_v534,
            scalar_v535: self.scalar_v535,
            scalar_v536: self.scalar_v536,
            scalar_v540: self.scalar_v540,
            scalar_v544: self.scalar_v544,
            scalar_v546: self.scalar_v546,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v554: self.scalar_v554,
            scalar_v555: self.scalar_v555,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v565: self.scalar_v565,
            scalar_v566: self.scalar_v566,
            scalar_v567: self.scalar_v567,
            scalar_v583: self.scalar_v583,
            scalar_v584: self.scalar_v584,
            scalar_v585: self.scalar_v585,
            scalar_v586: self.scalar_v586,
            scalar_v587: self.scalar_v587,
            scalar_v602: self.scalar_v602,
            scalar_v608: self.scalar_v608,
            scalar_v611: self.scalar_v611,
            scalar_v616: self.scalar_v616,
            scalar_v617: self.scalar_v617,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v624: self.scalar_v624,
            scalar_v625: self.scalar_v625,
            scalar_v640: self.scalar_v640,
            scalar_v645: self.scalar_v645,
            scalar_v646: self.scalar_v646,
            scalar_v647: self.scalar_v647,
            scalar_v648: self.scalar_v648,
            scalar_v656: self.scalar_v656,
            scalar_v661: self.scalar_v661,
            scalar_v662: self.scalar_v662,
            scalar_v663: self.scalar_v663,
            scalar_v664: self.scalar_v664,
            scalar_v665: self.scalar_v665,
            scalar_v666: self.scalar_v666,
            scalar_v667: self.scalar_v667,
            scalar_v668: self.scalar_v668,
            scalar_v669: self.scalar_v669,
            scalar_v670: self.scalar_v670,
            scalar_v671: self.scalar_v671,
            scalar_v672: self.scalar_v672,
            scalar_v673: self.scalar_v673,
            scalar_v674: self.scalar_v674,
            scalar_v692: self.scalar_v692,
            scalar_v700: self.scalar_v700,
            scalar_v701: self.scalar_v701,
            scalar_v705: self.scalar_v705,
            scalar_v706: self.scalar_v706,
            scalar_v710: self.scalar_v710,
            scalar_v711: self.scalar_v711,
            scalar_v712: self.scalar_v712,
            scalar_v735: self.scalar_v735,
            scalar_v736: self.scalar_v736,
            scalar_v737: self.scalar_v737,
            scalar_v739: self.scalar_v739,
            scalar_v740: self.scalar_v740,
            scalar_v744: self.scalar_v744,
            scalar_v748: self.scalar_v748,
            scalar_v749: self.scalar_v749,
            scalar_v751: self.scalar_v751,
            scalar_v756: self.scalar_v756,
            scalar_v760: self.scalar_v760,
            scalar_v761: self.scalar_v761,
            scalar_v765: self.scalar_v765,
            scalar_v766: self.scalar_v766,
            scalar_v767: self.scalar_v767,
            scalar_v768: self.scalar_v768,
            scalar_v769: self.scalar_v769,
            scalar_v770: self.scalar_v770,
            scalar_v771: self.scalar_v771,
            scalar_v772: self.scalar_v772,
            scalar_v773: self.scalar_v773,
            scalar_v774: self.scalar_v774,
            scalar_v775: self.scalar_v775,
            scalar_v776: self.scalar_v776,
            scalar_v777: self.scalar_v777,
            scalar_v778: self.scalar_v778,
            scalar_v779: self.scalar_v779,
            scalar_v796: self.scalar_v796,
            scalar_v803: self.scalar_v803,
            scalar_v804: self.scalar_v804,
            scalar_v805: self.scalar_v805,
            scalar_v806: self.scalar_v806,
            scalar_v807: self.scalar_v807,
            scalar_v811: self.scalar_v811,
            scalar_v812: self.scalar_v812,
            scalar_v816: self.scalar_v816,
            scalar_v820: self.scalar_v820,
            scalar_v821: self.scalar_v821,
            scalar_v825: self.scalar_v825,
            scalar_v826: self.scalar_v826,
            scalar_v830: self.scalar_v830,
            scalar_v831: self.scalar_v831,
            scalar_v835: self.scalar_v835,
            scalar_v836: self.scalar_v836,
            scalar_v840: self.scalar_v840,
            scalar_v844: self.scalar_v844,
            scalar_v845: self.scalar_v845,
            scalar_v846: self.scalar_v846,
            scalar_v847: self.scalar_v847,
            scalar_v848: self.scalar_v848,
            scalar_v849: self.scalar_v849,
            scalar_v850: self.scalar_v850,
            scalar_v895: self.scalar_v895,
            scalar_v896: self.scalar_v896,
            scalar_v925: self.scalar_v925,
            scalar_v929: self.scalar_v929,
            scalar_v947: self.scalar_v947,
            scalar_v948: self.scalar_v948,
            scalar_v973: self.scalar_v973,
            scalar_v977: self.scalar_v977,
            scalar_v981: self.scalar_v981,
            scalar_v1006: self.scalar_v1006,
            scalar_v1015: self.scalar_v1015,
            scalar_v1040: self.scalar_v1040,
            scalar_v1046: self.scalar_v1046,
            scalar_v1063: self.scalar_v1063,
            scalar_v1072: self.scalar_v1072,
            scalar_v1101: self.scalar_v1101,
            scalar_v1102: self.scalar_v1102,
            scalar_v1126: self.scalar_v1126,
            scalar_v1130: self.scalar_v1130,
            scalar_v1186: self.scalar_v1186,
            scalar_v1187: self.scalar_v1187,
            scalar_v1210: self.scalar_v1210,
            scalar_v1214: self.scalar_v1214,
            scalar_v1228: self.scalar_v1228,
            scalar_v1229: self.scalar_v1229,
            scalar_v1254: self.scalar_v1254,
            scalar_v1258: self.scalar_v1258,
            scalar_v1262: self.scalar_v1262,
            scalar_v1263: self.scalar_v1263,
            scalar_v1264: self.scalar_v1264,
            scalar_v1265: self.scalar_v1265,
            scalar_v1289: self.scalar_v1289,
            scalar_v1293: self.scalar_v1293,
            scalar_v1297: self.scalar_v1297,
            scalar_v1312: self.scalar_v1312,
            scalar_v1313: self.scalar_v1313,
            scalar_v1314: self.scalar_v1314,
            scalar_v1337: self.scalar_v1337,
            scalar_v1338: self.scalar_v1338,
            scalar_v1340: self.scalar_v1340,
            scalar_v1341: self.scalar_v1341,
            scalar_v1345: self.scalar_v1345,
            scalar_v1349: self.scalar_v1349,
            scalar_v1372: self.scalar_v1372,
            scalar_v1373: self.scalar_v1373,
            scalar_v1392: self.scalar_v1392,
            scalar_v1394: self.scalar_v1394,
            scalar_v1412: self.scalar_v1412,
            scalar_v1415: self.scalar_v1415,
            scalar_v1450: self.scalar_v1450,
            scalar_v1460: self.scalar_v1460,
            scalar_v1475: self.scalar_v1475,
            scalar_v1477: self.scalar_v1477,
            scalar_v1480: self.scalar_v1480,
            scalar_v1557: self.scalar_v1557,
            scalar_v1561: self.scalar_v1561,
            scalar_v1609: self.scalar_v1609,
            scalar_v1654: self.scalar_v1654,
            scalar_v1655: self.scalar_v1655,
            scalar_v1692: self.scalar_v1692,
            scalar_v1696: self.scalar_v1696,
            scalar_v1711: self.scalar_v1711,
            scalar_v1712: self.scalar_v1712,
            scalar_v1713: self.scalar_v1713,
            scalar_v1734: self.scalar_v1734,
            scalar_v1739: self.scalar_v1739,
            scalar_v1761: self.scalar_v1761,
            scalar_v1770: self.scalar_v1770,
            scalar_v1780: self.scalar_v1780,
            scalar_v1783: self.scalar_v1783,
            scalar_v1791: self.scalar_v1791,
            scalar_v1792: self.scalar_v1792,
            scalar_v1816: self.scalar_v1816,
            scalar_v1820: self.scalar_v1820,
            scalar_v1829: self.scalar_v1829,
            scalar_v1833: self.scalar_v1833,
            scalar_v1840: self.scalar_v1840,
            scalar_v1843: self.scalar_v1843,
            scalar_v1844: self.scalar_v1844,
            scalar_v1845: self.scalar_v1845,
            scalar_v1846: self.scalar_v1846,
            scalar_v1847: self.scalar_v1847,
            scalar_v1857: self.scalar_v1857,
            scalar_v1861: self.scalar_v1861,
            scalar_v1873: self.scalar_v1873,
            scalar_v1874: self.scalar_v1874,
            scalar_v1891: self.scalar_v1891,
            scalar_v1895: self.scalar_v1895,
            scalar_v1896: self.scalar_v1896,
            scalar_v1897: self.scalar_v1897,
            scalar_v1916: self.scalar_v1916,
            scalar_v1917: self.scalar_v1917,
            scalar_v1918: self.scalar_v1918,
            scalar_v1942: self.scalar_v1942,
            scalar_v1943: self.scalar_v1943,
            scalar_v1948: self.scalar_v1948,
            scalar_v1958: self.scalar_v1958,
            scalar_v1969: self.scalar_v1969,
            scalar_v1978: self.scalar_v1978,
            scalar_v1991: self.scalar_v1991,
            scalar_v2011: self.scalar_v2011,
            scalar_v2029: self.scalar_v2029,
            scalar_v2040: self.scalar_v2040,
            scalar_v2051: self.scalar_v2051,
            scalar_v2099: self.scalar_v2099,
            scalar_v2144: self.scalar_v2144,
            scalar_v2504: self.scalar_v2504,
            scalar_v2855: self.scalar_v2855,
            scalar_v2862: self.scalar_v2862,
            scalar_v2863: self.scalar_v2863,
            scalar_v2881: self.scalar_v2881,
            scalar_v2886: self.scalar_v2886,
            scalar_v2887: self.scalar_v2887,
            scalar_v2910: self.scalar_v2910,
            scalar_v2911: self.scalar_v2911,
            scalar_v2918: self.scalar_v2918,
            scalar_v2935: self.scalar_v2935,
            scalar_v2965: self.scalar_v2965,
            scalar_v2966: self.scalar_v2966,
            scalar_v2982: self.scalar_v2982,
            scalar_v2990: self.scalar_v2990,
            scalar_v2991: self.scalar_v2991,
            scalar_v3011: self.scalar_v3011,
            scalar_v3030: self.scalar_v3030,
            scalar_v3039: self.scalar_v3039,
            scalar_v3040: self.scalar_v3040,
            scalar_v3058: self.scalar_v3058,
            scalar_v3060: self.scalar_v3060,
            scalar_v3078: self.scalar_v3078,
            scalar_v3107: self.scalar_v3107,
            scalar_v3117: self.scalar_v3117,
            scalar_v3136: self.scalar_v3136,
            scalar_v3137: self.scalar_v3137,
            scalar_v3159: self.scalar_v3159,
            scalar_v3160: self.scalar_v3160,
            scalar_v3183: self.scalar_v3183,
            scalar_v3184: self.scalar_v3184,
            scalar_v3187: self.scalar_v3187,
            scalar_v3256: self.scalar_v3256,
            scalar_v3287: self.scalar_v3287,
            scalar_v3320: self.scalar_v3320,
            scalar_v3321: self.scalar_v3321,
            scalar_v3339: self.scalar_v3339,
            scalar_v3464: self.scalar_v3464,
            scalar_v3465: self.scalar_v3465,
            scalar_v3468: self.scalar_v3468,
            scalar_v3537: self.scalar_v3537,
            scalar_v3568: self.scalar_v3568,
            scalar_v3601: self.scalar_v3601,
            scalar_v3602: self.scalar_v3602,
            scalar_v3604: self.scalar_v3604,
            scalar_v3606: self.scalar_v3606,
            scalar_v3675: self.scalar_v3675,
            scalar_v3706: self.scalar_v3706,
            scalar_v3707: self.scalar_v3707,
            scalar_v3742: self.scalar_v3742,
            scalar_v3743: self.scalar_v3743,
            scalar_v3755: self.scalar_v3755,
            scalar_v3756: self.scalar_v3756,
            scalar_v3760: self.scalar_v3760,
            scalar_v3761: self.scalar_v3761,
            scalar_v3763: self.scalar_v3763,
            scalar_v3766: self.scalar_v3766,
            scalar_v3767: self.scalar_v3767,
            scalar_v3785: self.scalar_v3785,
            scalar_v3787: self.scalar_v3787,
            scalar_v3788: self.scalar_v3788,
            scalar_v3789: self.scalar_v3789,
            scalar_v3794: self.scalar_v3794,
            scalar_v3795: self.scalar_v3795,
            scalar_v3796: self.scalar_v3796,
            scalar_v3797: self.scalar_v3797,
            scalar_v3845: self.scalar_v3845,
            scalar_v3846: self.scalar_v3846,
            scalar_v3848: self.scalar_v3848,
            scalar_v3880: self.scalar_v3880,
            scalar_v3887: self.scalar_v3887,
            scalar_v3888: self.scalar_v3888,
            scalar_v3889: self.scalar_v3889,
            scalar_v3890: self.scalar_v3890,
            scalar_v3891: self.scalar_v3891,
            scalar_v3892: self.scalar_v3892,
            scalar_v3893: self.scalar_v3893,
            scalar_v3894: self.scalar_v3894,
            scalar_v3895: self.scalar_v3895,
            scalar_v3896: self.scalar_v3896,
            scalar_v3897: self.scalar_v3897,
            scalar_v3898: self.scalar_v3898,
            scalar_v3899: self.scalar_v3899,
            scalar_v3900: self.scalar_v3900,
            scalar_v3901: self.scalar_v3901,
            scalar_v3902: self.scalar_v3902,
            scalar_v3903: self.scalar_v3903,
            scalar_v3904: self.scalar_v3904,
            scalar_v3905: self.scalar_v3905,
            scalar_v3906: self.scalar_v3906,
            scalar_v3907: self.scalar_v3907,
            scalar_v3908: self.scalar_v3908,
            scalar_v3909: self.scalar_v3909,
            scalar_v3910: self.scalar_v3910,
            scalar_v3914: self.scalar_v3914,
            scalar_v3915: self.scalar_v3915,
            scalar_v3916: self.scalar_v3916,
            scalar_v3917: self.scalar_v3917,
            scalar_v3918: self.scalar_v3918,
            scalar_v3919: self.scalar_v3919,
            scalar_v3926: self.scalar_v3926,
            scalar_v3929: self.scalar_v3929,
            scalar_v3930: self.scalar_v3930,
            scalar_v3931: self.scalar_v3931,
            scalar_v3944: self.scalar_v3944,
            scalar_v3963: self.scalar_v3963,
            scalar_v3968: self.scalar_v3968,
            scalar_v3987: self.scalar_v3987,
            scalar_v3990: self.scalar_v3990,
            scalar_v3995: self.scalar_v3995,
            scalar_v3997: self.scalar_v3997,
            scalar_v4004: self.scalar_v4004,
            scalar_v4011: self.scalar_v4011,
            scalar_v4033: self.scalar_v4033,
            scalar_v4036: self.scalar_v4036,
            scalar_v4037: self.scalar_v4037,
            scalar_v5647: self.scalar_v5647,
            scalar_v5648: self.scalar_v5648,
            scalar_v5651: self.scalar_v5651,
            scalar_v5652: self.scalar_v5652,
            scalar_v5653: self.scalar_v5653,
            scalar_v5698: self.scalar_v5698,
            scalar_v5699: self.scalar_v5699,
            scalar_v5700: self.scalar_v5700,
            scalar_v23551: self.scalar_v23551,
            scalar_v23552: self.scalar_v23552,
            scalar_v26252: self.scalar_v26252,
            scalar_v26253: self.scalar_v26253,
            scalar_v28544: self.scalar_v28544,
            scalar_v28545: self.scalar_v28545,
            scalar_v29048: self.scalar_v29048,
            scalar_v29197: self.scalar_v29197,
            scalar_v29305: self.scalar_v29305,
            scalar_v29306: self.scalar_v29306,
            scalar_v29307: self.scalar_v29307,
            scalar_v29308: self.scalar_v29308,
            scalar_v29309: self.scalar_v29309,
            scalar_v29310: self.scalar_v29310,
            scalar_v29521: self.scalar_v29521,
            scalar_v29522: self.scalar_v29522,
            scalar_v29523: self.scalar_v29523,
            scalar_v29587: self.scalar_v29587,
            scalar_v29588: self.scalar_v29588,
            scalar_v29610: self.scalar_v29610,
            scalar_v30261: self.scalar_v30261,
            scalar_v30268: self.scalar_v30268,
            scalar_v30285: self.scalar_v30285,
            scalar_v30286: self.scalar_v30286,
            scalar_v30287: self.scalar_v30287,
            scalar_v30304: self.scalar_v30304,
            scalar_v30311: self.scalar_v30311,
            scalar_v30328: self.scalar_v30328,
            scalar_v30329: self.scalar_v30329,
            scalar_v30330: self.scalar_v30330,
            scalar_v30331: self.scalar_v30331,
            scalar_v30332: self.scalar_v30332,
            scalar_v30385: self.scalar_v30385,
            scalar_v30517: self.scalar_v30517,
            scalar_v190: self.scalar_v190,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
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
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v238: self.scalar_v238,
            scalar_v239: self.scalar_v239,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v258: self.scalar_v258,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v278: self.scalar_v278,
            scalar_v280: self.scalar_v280,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v288: self.scalar_v288,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v306: self.scalar_v306,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
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
            scalar_v324: self.scalar_v324,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v343: self.scalar_v343,
            scalar_v345: self.scalar_v345,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v360: self.scalar_v360,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v379: self.scalar_v379,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v383: self.scalar_v383,
            scalar_v384: self.scalar_v384,
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v389: self.scalar_v389,
            scalar_v390: self.scalar_v390,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v395: self.scalar_v395,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v407: self.scalar_v407,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v441: self.scalar_v441,
            scalar_v445: self.scalar_v445,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v473: self.scalar_v473,
            scalar_v475: self.scalar_v475,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v511: self.scalar_v511,
            scalar_v512: self.scalar_v512,
            scalar_v513: self.scalar_v513,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v519: self.scalar_v519,
            scalar_v520: self.scalar_v520,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v523: self.scalar_v523,
            scalar_v524: self.scalar_v524,
            scalar_v526: self.scalar_v526,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v537: self.scalar_v537,
            scalar_v538: self.scalar_v538,
            scalar_v539: self.scalar_v539,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v545: self.scalar_v545,
            scalar_v549: self.scalar_v549,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v553: self.scalar_v553,
            scalar_v561: self.scalar_v561,
            scalar_v569: self.scalar_v569,
            scalar_v571: self.scalar_v571,
            scalar_v577: self.scalar_v577,
            scalar_v591: self.scalar_v591,
            scalar_v596: self.scalar_v596,
            scalar_v609: self.scalar_v609,
            scalar_v610: self.scalar_v610,
            scalar_v612: self.scalar_v612,
            scalar_v613: self.scalar_v613,
            scalar_v614: self.scalar_v614,
            scalar_v626: self.scalar_v626,
            scalar_v627: self.scalar_v627,
            scalar_v628: self.scalar_v628,
            scalar_v629: self.scalar_v629,
            scalar_v630: self.scalar_v630,
            scalar_v631: self.scalar_v631,
            scalar_v632: self.scalar_v632,
            scalar_v633: self.scalar_v633,
            scalar_v634: self.scalar_v634,
            scalar_v635: self.scalar_v635,
            scalar_v636: self.scalar_v636,
            scalar_v637: self.scalar_v637,
            scalar_v638: self.scalar_v638,
            scalar_v639: self.scalar_v639,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v643: self.scalar_v643,
            scalar_v644: self.scalar_v644,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v653: self.scalar_v653,
            scalar_v654: self.scalar_v654,
            scalar_v655: self.scalar_v655,
            scalar_v657: self.scalar_v657,
            scalar_v658: self.scalar_v658,
            scalar_v659: self.scalar_v659,
            scalar_v660: self.scalar_v660,
            scalar_v675: self.scalar_v675,
            scalar_v676: self.scalar_v676,
            scalar_v677: self.scalar_v677,
            scalar_v678: self.scalar_v678,
            scalar_v679: self.scalar_v679,
            scalar_v680: self.scalar_v680,
            scalar_v681: self.scalar_v681,
            scalar_v682: self.scalar_v682,
            scalar_v683: self.scalar_v683,
            scalar_v684: self.scalar_v684,
            scalar_v685: self.scalar_v685,
            scalar_v686: self.scalar_v686,
            scalar_v687: self.scalar_v687,
            scalar_v688: self.scalar_v688,
            scalar_v689: self.scalar_v689,
            scalar_v690: self.scalar_v690,
            scalar_v691: self.scalar_v691,
            scalar_v693: self.scalar_v693,
            scalar_v694: self.scalar_v694,
            scalar_v695: self.scalar_v695,
            scalar_v696: self.scalar_v696,
            scalar_v697: self.scalar_v697,
            scalar_v698: self.scalar_v698,
            scalar_v702: self.scalar_v702,
            scalar_v703: self.scalar_v703,
            scalar_v704: self.scalar_v704,
            scalar_v707: self.scalar_v707,
            scalar_v708: self.scalar_v708,
            scalar_v709: self.scalar_v709,
            scalar_v713: self.scalar_v713,
            scalar_v714: self.scalar_v714,
            scalar_v715: self.scalar_v715,
            scalar_v716: self.scalar_v716,
            scalar_v717: self.scalar_v717,
            scalar_v718: self.scalar_v718,
            scalar_v719: self.scalar_v719,
            scalar_v720: self.scalar_v720,
            scalar_v721: self.scalar_v721,
            scalar_v722: self.scalar_v722,
            scalar_v723: self.scalar_v723,
            scalar_v724: self.scalar_v724,
            scalar_v725: self.scalar_v725,
            scalar_v726: self.scalar_v726,
            scalar_v727: self.scalar_v727,
            scalar_v728: self.scalar_v728,
            scalar_v729: self.scalar_v729,
            scalar_v730: self.scalar_v730,
            scalar_v731: self.scalar_v731,
            scalar_v732: self.scalar_v732,
            scalar_v733: self.scalar_v733,
            scalar_v734: self.scalar_v734,
            scalar_v738: self.scalar_v738,
            scalar_v741: self.scalar_v741,
            scalar_v742: self.scalar_v742,
            scalar_v743: self.scalar_v743,
            scalar_v745: self.scalar_v745,
            scalar_v746: self.scalar_v746,
            scalar_v747: self.scalar_v747,
            scalar_v750: self.scalar_v750,
            scalar_v752: self.scalar_v752,
            scalar_v753: self.scalar_v753,
            scalar_v754: self.scalar_v754,
            scalar_v755: self.scalar_v755,
            scalar_v757: self.scalar_v757,
            scalar_v758: self.scalar_v758,
            scalar_v759: self.scalar_v759,
            scalar_v762: self.scalar_v762,
            scalar_v763: self.scalar_v763,
            scalar_v764: self.scalar_v764,
            scalar_v780: self.scalar_v780,
            scalar_v781: self.scalar_v781,
            scalar_v782: self.scalar_v782,
            scalar_v783: self.scalar_v783,
            scalar_v784: self.scalar_v784,
            scalar_v785: self.scalar_v785,
            scalar_v786: self.scalar_v786,
            scalar_v787: self.scalar_v787,
            scalar_v788: self.scalar_v788,
            scalar_v789: self.scalar_v789,
            scalar_v790: self.scalar_v790,
            scalar_v791: self.scalar_v791,
            scalar_v792: self.scalar_v792,
            scalar_v793: self.scalar_v793,
            scalar_v794: self.scalar_v794,
            scalar_v795: self.scalar_v795,
            scalar_v797: self.scalar_v797,
            scalar_v798: self.scalar_v798,
            scalar_v799: self.scalar_v799,
            scalar_v800: self.scalar_v800,
            scalar_v801: self.scalar_v801,
            scalar_v802: self.scalar_v802,
            scalar_v808: self.scalar_v808,
            scalar_v809: self.scalar_v809,
            scalar_v810: self.scalar_v810,
            scalar_v813: self.scalar_v813,
            scalar_v814: self.scalar_v814,
            scalar_v815: self.scalar_v815,
            scalar_v817: self.scalar_v817,
            scalar_v818: self.scalar_v818,
            scalar_v819: self.scalar_v819,
            scalar_v822: self.scalar_v822,
            scalar_v823: self.scalar_v823,
            scalar_v824: self.scalar_v824,
            scalar_v827: self.scalar_v827,
            scalar_v828: self.scalar_v828,
            scalar_v829: self.scalar_v829,
            scalar_v832: self.scalar_v832,
            scalar_v833: self.scalar_v833,
            scalar_v834: self.scalar_v834,
            scalar_v837: self.scalar_v837,
            scalar_v838: self.scalar_v838,
            scalar_v839: self.scalar_v839,
            scalar_v841: self.scalar_v841,
            scalar_v842: self.scalar_v842,
            scalar_v843: self.scalar_v843,
            scalar_v924: self.scalar_v924,
            scalar_v972: self.scalar_v972,
            scalar_v1045: self.scalar_v1045,
            scalar_v1125: self.scalar_v1125,
            scalar_v1209: self.scalar_v1209,
            scalar_v1253: self.scalar_v1253,
            scalar_v1339: self.scalar_v1339,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
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
            scalar_v24: 0.0,
            scalar_v26: false,
            scalar_v29: 0.0,
            scalar_v31: 0.0,
            scalar_v32: false,
            scalar_v34: 0.0,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
            scalar_v39: 0.0,
            scalar_v41: 0.0,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
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
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
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
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: false,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: false,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: false,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: false,
            scalar_v113: 0.0,
            scalar_v114: false,
            scalar_v115: false,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v119: false,
            scalar_v120: 0.0,
            scalar_v121: false,
            scalar_v122: false,
            scalar_v123: 0.0,
            scalar_v124: false,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: false,
            scalar_v128: 0.0,
            scalar_v129: false,
            scalar_v130: 0.0,
            scalar_v131: false,
            scalar_v132: false,
            scalar_v133: 0.0,
            scalar_v134: false,
            scalar_v135: false,
            scalar_v136: false,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v140: false,
            scalar_v141: 0.0,
            scalar_v142: false,
            scalar_v143: false,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: false,
            scalar_v148: false,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: false,
            scalar_v152: false,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: false,
            scalar_v156: false,
            scalar_v158: 0.0,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: false,
            scalar_v165: false,
            scalar_v166: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: false,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: 0.0,
            scalar_v219: 0.0,
            scalar_v220: false,
            scalar_v222: 0.0,
            scalar_v223: 0.0,
            scalar_v224: 0.0,
            scalar_v225: 0.0,
            scalar_v226: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v257: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v266: 0.0,
            scalar_v267: false,
            scalar_v268: false,
            scalar_v272: false,
            scalar_v276: 0.0,
            scalar_v277: 0.0,
            scalar_v279: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v289: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v323: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: false,
            scalar_v334: false,
            scalar_v338: false,
            scalar_v344: 0.0,
            scalar_v346: 0.0,
            scalar_v351: 0.0,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v361: 0.0,
            scalar_v366: 0.0,
            scalar_v367: 0.0,
            scalar_v371: 0.0,
            scalar_v372: 0.0,
            scalar_v373: 0.0,
            scalar_v375: false,
            scalar_v376: false,
            scalar_v377: 0.0,
            scalar_v378: 0.0,
            scalar_v380: 0.0,
            scalar_v388: false,
            scalar_v391: 0.0,
            scalar_v392: 0.0,
            scalar_v396: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v413: 0.0,
            scalar_v418: 0.0,
            scalar_v419: false,
            scalar_v420: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: false,
            scalar_v428: 0.0,
            scalar_v434: 0.0,
            scalar_v437: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v448: false,
            scalar_v449: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v460: false,
            scalar_v463: 0.0,
            scalar_v464: false,
            scalar_v468: 0.0,
            scalar_v470: false,
            scalar_v471: false,
            scalar_v491: 0.0,
            scalar_v492: 0.0,
            scalar_v496: 0.0,
            scalar_v497: false,
            scalar_v498: 0.0,
            scalar_v499: 0.0,
            scalar_v500: 0.0,
            scalar_v501: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v504: 0.0,
            scalar_v505: 0.0,
            scalar_v506: 0.0,
            scalar_v507: 0.0,
            scalar_v508: 0.0,
            scalar_v525: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v534: 0.0,
            scalar_v535: false,
            scalar_v536: false,
            scalar_v540: false,
            scalar_v544: 0.0,
            scalar_v546: 0.0,
            scalar_v547: 0.0,
            scalar_v548: 0.0,
            scalar_v554: 0.0,
            scalar_v555: false,
            scalar_v563: 0.0,
            scalar_v564: false,
            scalar_v565: false,
            scalar_v566: false,
            scalar_v567: false,
            scalar_v583: false,
            scalar_v584: false,
            scalar_v585: false,
            scalar_v586: false,
            scalar_v587: false,
            scalar_v602: 0.0,
            scalar_v608: 0.0,
            scalar_v611: 0.0,
            scalar_v616: 0.0,
            scalar_v617: 0.0,
            scalar_v618: 0.0,
            scalar_v619: 0.0,
            scalar_v620: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v624: 0.0,
            scalar_v625: 0.0,
            scalar_v640: 0.0,
            scalar_v645: 0.0,
            scalar_v646: 0.0,
            scalar_v647: false,
            scalar_v648: false,
            scalar_v656: 0.0,
            scalar_v661: 0.0,
            scalar_v662: false,
            scalar_v663: false,
            scalar_v664: 0.0,
            scalar_v665: 0.0,
            scalar_v666: 0.0,
            scalar_v667: 0.0,
            scalar_v668: 0.0,
            scalar_v669: 0.0,
            scalar_v670: 0.0,
            scalar_v671: 0.0,
            scalar_v672: 0.0,
            scalar_v673: 0.0,
            scalar_v674: 0.0,
            scalar_v692: 0.0,
            scalar_v700: 0.0,
            scalar_v701: false,
            scalar_v705: false,
            scalar_v706: false,
            scalar_v710: 0.0,
            scalar_v711: false,
            scalar_v712: 0.0,
            scalar_v735: 0.0,
            scalar_v736: 0.0,
            scalar_v737: 0.0,
            scalar_v739: false,
            scalar_v740: false,
            scalar_v744: false,
            scalar_v748: 0.0,
            scalar_v749: 0.0,
            scalar_v751: 0.0,
            scalar_v756: 0.0,
            scalar_v760: 0.0,
            scalar_v761: 0.0,
            scalar_v765: 0.0,
            scalar_v766: false,
            scalar_v767: 0.0,
            scalar_v768: false,
            scalar_v769: false,
            scalar_v770: 0.0,
            scalar_v771: 0.0,
            scalar_v772: 0.0,
            scalar_v773: 0.0,
            scalar_v774: 0.0,
            scalar_v775: 0.0,
            scalar_v776: 0.0,
            scalar_v777: 0.0,
            scalar_v778: 0.0,
            scalar_v779: 0.0,
            scalar_v796: 0.0,
            scalar_v803: 0.0,
            scalar_v804: 0.0,
            scalar_v805: 0.0,
            scalar_v806: false,
            scalar_v807: false,
            scalar_v811: false,
            scalar_v812: false,
            scalar_v816: false,
            scalar_v820: 0.0,
            scalar_v821: 0.0,
            scalar_v825: 0.0,
            scalar_v826: 0.0,
            scalar_v830: 0.0,
            scalar_v831: 0.0,
            scalar_v835: 0.0,
            scalar_v836: 0.0,
            scalar_v840: 0.0,
            scalar_v844: 0.0,
            scalar_v845: false,
            scalar_v846: 0.0,
            scalar_v847: false,
            scalar_v848: false,
            scalar_v849: false,
            scalar_v850: false,
            scalar_v895: false,
            scalar_v896: 0.0,
            scalar_v925: false,
            scalar_v929: false,
            scalar_v947: false,
            scalar_v948: 0.0,
            scalar_v973: false,
            scalar_v977: false,
            scalar_v981: false,
            scalar_v1006: false,
            scalar_v1015: false,
            scalar_v1040: false,
            scalar_v1046: false,
            scalar_v1063: false,
            scalar_v1072: false,
            scalar_v1101: false,
            scalar_v1102: 0.0,
            scalar_v1126: false,
            scalar_v1130: false,
            scalar_v1186: false,
            scalar_v1187: 0.0,
            scalar_v1210: false,
            scalar_v1214: false,
            scalar_v1228: false,
            scalar_v1229: 0.0,
            scalar_v1254: false,
            scalar_v1258: false,
            scalar_v1262: 0.0,
            scalar_v1263: false,
            scalar_v1264: false,
            scalar_v1265: 0.0,
            scalar_v1289: false,
            scalar_v1293: false,
            scalar_v1297: 0.0,
            scalar_v1312: false,
            scalar_v1313: false,
            scalar_v1314: 0.0,
            scalar_v1337: 0.0,
            scalar_v1338: 0.0,
            scalar_v1340: false,
            scalar_v1341: false,
            scalar_v1345: false,
            scalar_v1349: false,
            scalar_v1372: false,
            scalar_v1373: 0.0,
            scalar_v1392: false,
            scalar_v1394: false,
            scalar_v1412: false,
            scalar_v1415: 0.0,
            scalar_v1450: 0.0,
            scalar_v1460: 0.0,
            scalar_v1475: 0.0,
            scalar_v1477: false,
            scalar_v1480: 0.0,
            scalar_v1557: 0.0,
            scalar_v1561: 0.0,
            scalar_v1609: false,
            scalar_v1654: false,
            scalar_v1655: 0.0,
            scalar_v1692: false,
            scalar_v1696: 0.0,
            scalar_v1711: 0.0,
            scalar_v1712: 0.0,
            scalar_v1713: 0.0,
            scalar_v1734: 0.0,
            scalar_v1739: 0.0,
            scalar_v1761: 0.0,
            scalar_v1770: 0.0,
            scalar_v1780: 0.0,
            scalar_v1783: 0.0,
            scalar_v1791: 0.0,
            scalar_v1792: false,
            scalar_v1816: false,
            scalar_v1820: false,
            scalar_v1829: false,
            scalar_v1833: 0.0,
            scalar_v1840: 0.0,
            scalar_v1843: 0.0,
            scalar_v1844: 0.0,
            scalar_v1845: 0.0,
            scalar_v1846: false,
            scalar_v1847: false,
            scalar_v1857: 0.0,
            scalar_v1861: 0.0,
            scalar_v1873: 0.0,
            scalar_v1874: 0.0,
            scalar_v1891: 0.0,
            scalar_v1895: 0.0,
            scalar_v1896: 0.0,
            scalar_v1897: 0.0,
            scalar_v1916: false,
            scalar_v1917: false,
            scalar_v1918: false,
            scalar_v1942: 0.0,
            scalar_v1943: false,
            scalar_v1948: false,
            scalar_v1958: 0.0,
            scalar_v1969: 0.0,
            scalar_v1978: false,
            scalar_v1991: 0.0,
            scalar_v2011: 0.0,
            scalar_v2029: 0.0,
            scalar_v2040: false,
            scalar_v2051: 0.0,
            scalar_v2099: 0.0,
            scalar_v2144: 0.0,
            scalar_v2504: 0.0,
            scalar_v2855: 0.0,
            scalar_v2862: false,
            scalar_v2863: 0.0,
            scalar_v2881: false,
            scalar_v2886: 0.0,
            scalar_v2887: 0.0,
            scalar_v2910: 0.0,
            scalar_v2911: false,
            scalar_v2918: 0.0,
            scalar_v2935: false,
            scalar_v2965: 0.0,
            scalar_v2966: false,
            scalar_v2982: false,
            scalar_v2990: 0.0,
            scalar_v2991: 0.0,
            scalar_v3011: 0.0,
            scalar_v3030: 0.0,
            scalar_v3039: false,
            scalar_v3040: 0.0,
            scalar_v3058: false,
            scalar_v3060: false,
            scalar_v3078: false,
            scalar_v3107: 0.0,
            scalar_v3117: 0.0,
            scalar_v3136: 0.0,
            scalar_v3137: 0.0,
            scalar_v3159: 0.0,
            scalar_v3160: 0.0,
            scalar_v3183: 0.0,
            scalar_v3184: false,
            scalar_v3187: 0.0,
            scalar_v3256: 0.0,
            scalar_v3287: false,
            scalar_v3320: false,
            scalar_v3321: 0.0,
            scalar_v3339: false,
            scalar_v3464: 0.0,
            scalar_v3465: false,
            scalar_v3468: 0.0,
            scalar_v3537: 0.0,
            scalar_v3568: false,
            scalar_v3601: 0.0,
            scalar_v3602: false,
            scalar_v3604: false,
            scalar_v3606: 0.0,
            scalar_v3675: 0.0,
            scalar_v3706: false,
            scalar_v3707: false,
            scalar_v3742: false,
            scalar_v3743: 0.0,
            scalar_v3755: false,
            scalar_v3756: false,
            scalar_v3760: false,
            scalar_v3761: false,
            scalar_v3763: false,
            scalar_v3766: false,
            scalar_v3767: 0.0,
            scalar_v3785: false,
            scalar_v3787: false,
            scalar_v3788: false,
            scalar_v3789: false,
            scalar_v3794: false,
            scalar_v3795: false,
            scalar_v3796: false,
            scalar_v3797: false,
            scalar_v3845: false,
            scalar_v3846: false,
            scalar_v3848: false,
            scalar_v3880: false,
            scalar_v3887: false,
            scalar_v3888: false,
            scalar_v3889: false,
            scalar_v3890: false,
            scalar_v3891: false,
            scalar_v3892: false,
            scalar_v3893: false,
            scalar_v3894: false,
            scalar_v3895: false,
            scalar_v3896: false,
            scalar_v3897: false,
            scalar_v3898: false,
            scalar_v3899: false,
            scalar_v3900: 0.0,
            scalar_v3901: false,
            scalar_v3902: false,
            scalar_v3903: false,
            scalar_v3904: 0.0,
            scalar_v3905: false,
            scalar_v3906: false,
            scalar_v3907: false,
            scalar_v3908: false,
            scalar_v3909: 0.0,
            scalar_v3910: false,
            scalar_v3914: 0.0,
            scalar_v3915: false,
            scalar_v3916: false,
            scalar_v3917: false,
            scalar_v3918: false,
            scalar_v3919: false,
            scalar_v3926: 0.0,
            scalar_v3929: 0.0,
            scalar_v3930: 0.0,
            scalar_v3931: 0.0,
            scalar_v3944: 0.0,
            scalar_v3963: false,
            scalar_v3968: false,
            scalar_v3987: 0.0,
            scalar_v3990: false,
            scalar_v3995: false,
            scalar_v3997: false,
            scalar_v4004: false,
            scalar_v4011: false,
            scalar_v4033: false,
            scalar_v4036: 0.0,
            scalar_v4037: 0.0,
            scalar_v5647: 0.0,
            scalar_v5648: 0.0,
            scalar_v5651: 0.0,
            scalar_v5652: 0.0,
            scalar_v5653: 0.0,
            scalar_v5698: 0.0,
            scalar_v5699: 0.0,
            scalar_v5700: 0.0,
            scalar_v23551: 0.0,
            scalar_v23552: 0.0,
            scalar_v26252: 0.0,
            scalar_v26253: 0.0,
            scalar_v28544: 0.0,
            scalar_v28545: 0.0,
            scalar_v29048: 0.0,
            scalar_v29197: 0.0,
            scalar_v29305: 0.0,
            scalar_v29306: 0.0,
            scalar_v29307: 0.0,
            scalar_v29308: 0.0,
            scalar_v29309: 0.0,
            scalar_v29310: 0.0,
            scalar_v29521: 0.0,
            scalar_v29522: 0.0,
            scalar_v29523: 0.0,
            scalar_v29587: 0.0,
            scalar_v29588: 0.0,
            scalar_v29610: 0.0,
            scalar_v30261: 0.0,
            scalar_v30268: 0.0,
            scalar_v30285: 0.0,
            scalar_v30286: 0.0,
            scalar_v30287: 0.0,
            scalar_v30304: 0.0,
            scalar_v30311: 0.0,
            scalar_v30328: 0.0,
            scalar_v30329: 0.0,
            scalar_v30330: 0.0,
            scalar_v30331: 0.0,
            scalar_v30332: 0.0,
            scalar_v30385: 0.0,
            scalar_v30517: 0.0,
            scalar_v190: 0.0,
            scalar_v192: false,
            scalar_v193: 0.0,
            scalar_v195: false,
            scalar_v196: false,
            scalar_v197: false,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v201: 0.0,
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
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v237: 0.0,
            scalar_v238: 0.0,
            scalar_v239: 0.0,
            scalar_v240: 0.0,
            scalar_v241: 0.0,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v245: 0.0,
            scalar_v246: 0.0,
            scalar_v247: 0.0,
            scalar_v248: 0.0,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v258: 0.0,
            scalar_v259: 0.0,
            scalar_v260: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v278: 0.0,
            scalar_v280: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v288: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v314: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v324: 0.0,
            scalar_v325: 0.0,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v343: 0.0,
            scalar_v345: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v354: 0.0,
            scalar_v355: 0.0,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v360: 0.0,
            scalar_v362: 0.0,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v370: 0.0,
            scalar_v379: 0.0,
            scalar_v381: 0.0,
            scalar_v382: 0.0,
            scalar_v383: 0.0,
            scalar_v384: 0.0,
            scalar_v385: 0.0,
            scalar_v386: 0.0,
            scalar_v387: 0.0,
            scalar_v389: 0.0,
            scalar_v390: 0.0,
            scalar_v393: 0.0,
            scalar_v394: 0.0,
            scalar_v395: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v401: 0.0,
            scalar_v405: 0.0,
            scalar_v406: 0.0,
            scalar_v407: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v429: 0.0,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v438: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v441: 0.0,
            scalar_v445: 0.0,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v473: 0.0,
            scalar_v475: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v513: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v520: 0.0,
            scalar_v521: 0.0,
            scalar_v522: 0.0,
            scalar_v523: 0.0,
            scalar_v524: 0.0,
            scalar_v526: 0.0,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v530: 0.0,
            scalar_v531: 0.0,
            scalar_v537: 0.0,
            scalar_v538: 0.0,
            scalar_v539: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v545: 0.0,
            scalar_v549: 0.0,
            scalar_v550: 0.0,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v561: 0.0,
            scalar_v569: 0.0,
            scalar_v571: 0.0,
            scalar_v577: 0.0,
            scalar_v591: 0.0,
            scalar_v596: 0.0,
            scalar_v609: 0.0,
            scalar_v610: 0.0,
            scalar_v612: 0.0,
            scalar_v613: 0.0,
            scalar_v614: 0.0,
            scalar_v626: 0.0,
            scalar_v627: 0.0,
            scalar_v628: 0.0,
            scalar_v629: 0.0,
            scalar_v630: 0.0,
            scalar_v631: 0.0,
            scalar_v632: 0.0,
            scalar_v633: 0.0,
            scalar_v634: 0.0,
            scalar_v635: 0.0,
            scalar_v636: 0.0,
            scalar_v637: 0.0,
            scalar_v638: 0.0,
            scalar_v639: 0.0,
            scalar_v641: 0.0,
            scalar_v642: 0.0,
            scalar_v643: 0.0,
            scalar_v644: 0.0,
            scalar_v649: 0.0,
            scalar_v650: 0.0,
            scalar_v651: 0.0,
            scalar_v653: 0.0,
            scalar_v654: 0.0,
            scalar_v655: 0.0,
            scalar_v657: 0.0,
            scalar_v658: 0.0,
            scalar_v659: 0.0,
            scalar_v660: 0.0,
            scalar_v675: 0.0,
            scalar_v676: 0.0,
            scalar_v677: 0.0,
            scalar_v678: 0.0,
            scalar_v679: 0.0,
            scalar_v680: 0.0,
            scalar_v681: 0.0,
            scalar_v682: 0.0,
            scalar_v683: 0.0,
            scalar_v684: 0.0,
            scalar_v685: 0.0,
            scalar_v686: 0.0,
            scalar_v687: 0.0,
            scalar_v688: 0.0,
            scalar_v689: 0.0,
            scalar_v690: 0.0,
            scalar_v691: 0.0,
            scalar_v693: 0.0,
            scalar_v694: 0.0,
            scalar_v695: 0.0,
            scalar_v696: 0.0,
            scalar_v697: 0.0,
            scalar_v698: 0.0,
            scalar_v702: 0.0,
            scalar_v703: 0.0,
            scalar_v704: 0.0,
            scalar_v707: 0.0,
            scalar_v708: 0.0,
            scalar_v709: 0.0,
            scalar_v713: 0.0,
            scalar_v714: 0.0,
            scalar_v715: 0.0,
            scalar_v716: 0.0,
            scalar_v717: 0.0,
            scalar_v718: 0.0,
            scalar_v719: 0.0,
            scalar_v720: 0.0,
            scalar_v721: 0.0,
            scalar_v722: 0.0,
            scalar_v723: 0.0,
            scalar_v724: 0.0,
            scalar_v725: 0.0,
            scalar_v726: 0.0,
            scalar_v727: 0.0,
            scalar_v728: 0.0,
            scalar_v729: 0.0,
            scalar_v730: 0.0,
            scalar_v731: 0.0,
            scalar_v732: 0.0,
            scalar_v733: 0.0,
            scalar_v734: 0.0,
            scalar_v738: 0.0,
            scalar_v741: 0.0,
            scalar_v742: 0.0,
            scalar_v743: 0.0,
            scalar_v745: 0.0,
            scalar_v746: 0.0,
            scalar_v747: 0.0,
            scalar_v750: 0.0,
            scalar_v752: 0.0,
            scalar_v753: 0.0,
            scalar_v754: 0.0,
            scalar_v755: 0.0,
            scalar_v757: 0.0,
            scalar_v758: 0.0,
            scalar_v759: 0.0,
            scalar_v762: 0.0,
            scalar_v763: 0.0,
            scalar_v764: 0.0,
            scalar_v780: 0.0,
            scalar_v781: 0.0,
            scalar_v782: 0.0,
            scalar_v783: 0.0,
            scalar_v784: 0.0,
            scalar_v785: 0.0,
            scalar_v786: 0.0,
            scalar_v787: 0.0,
            scalar_v788: 0.0,
            scalar_v789: 0.0,
            scalar_v790: 0.0,
            scalar_v791: 0.0,
            scalar_v792: 0.0,
            scalar_v793: 0.0,
            scalar_v794: 0.0,
            scalar_v795: 0.0,
            scalar_v797: 0.0,
            scalar_v798: 0.0,
            scalar_v799: 0.0,
            scalar_v800: 0.0,
            scalar_v801: 0.0,
            scalar_v802: 0.0,
            scalar_v808: 0.0,
            scalar_v809: 0.0,
            scalar_v810: 0.0,
            scalar_v813: 0.0,
            scalar_v814: 0.0,
            scalar_v815: 0.0,
            scalar_v817: 0.0,
            scalar_v818: 0.0,
            scalar_v819: 0.0,
            scalar_v822: 0.0,
            scalar_v823: 0.0,
            scalar_v824: 0.0,
            scalar_v827: 0.0,
            scalar_v828: 0.0,
            scalar_v829: 0.0,
            scalar_v832: 0.0,
            scalar_v833: 0.0,
            scalar_v834: 0.0,
            scalar_v837: 0.0,
            scalar_v838: 0.0,
            scalar_v839: 0.0,
            scalar_v841: 0.0,
            scalar_v842: 0.0,
            scalar_v843: 0.0,
            scalar_v924: 0.0,
            scalar_v972: 0.0,
            scalar_v1045: 0.0,
            scalar_v1125: 0.0,
            scalar_v1209: 0.0,
            scalar_v1253: 0.0,
            scalar_v1339: 0.0,
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
            scalar_v24,
            scalar_v26,
            scalar_v29,
            scalar_v31,
            scalar_v32,
            scalar_v34,
            scalar_v36,
            scalar_v37,
            scalar_v39,
            scalar_v41,
            scalar_v43,
            scalar_v44,
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
            scalar_v56,
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
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
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
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
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
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
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
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v219,
            scalar_v220,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v234,
            scalar_v257,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v279,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v289,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v323,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v338,
            scalar_v344,
            scalar_v346,
            scalar_v351,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v372,
            scalar_v373,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v380,
            scalar_v388,
            scalar_v391,
            scalar_v392,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v434,
            scalar_v437,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v454,
            scalar_v455,
            scalar_v460,
            scalar_v463,
            scalar_v464,
            scalar_v468,
            scalar_v470,
            scalar_v471,
            scalar_v491,
            scalar_v492,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v525,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v540,
            scalar_v544,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v555,
            scalar_v563,
            scalar_v564,
            scalar_v565,
            scalar_v566,
            scalar_v567,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v602,
            scalar_v608,
            scalar_v611,
            scalar_v616,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v640,
            scalar_v645,
            scalar_v646,
            scalar_v647,
            scalar_v648,
            scalar_v656,
            scalar_v661,
            scalar_v662,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v666,
            scalar_v667,
            scalar_v668,
            scalar_v669,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v674,
            scalar_v692,
            scalar_v700,
            scalar_v701,
            scalar_v705,
            scalar_v706,
            scalar_v710,
            scalar_v711,
            scalar_v712,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v739,
            scalar_v740,
            scalar_v744,
            scalar_v748,
            scalar_v749,
            scalar_v751,
            scalar_v756,
            scalar_v760,
            scalar_v761,
            scalar_v765,
            scalar_v766,
            scalar_v767,
            scalar_v768,
            scalar_v769,
            scalar_v770,
            scalar_v771,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v775,
            scalar_v776,
            scalar_v777,
            scalar_v778,
            scalar_v779,
            scalar_v796,
            scalar_v803,
            scalar_v804,
            scalar_v805,
            scalar_v806,
            scalar_v807,
            scalar_v811,
            scalar_v812,
            scalar_v816,
            scalar_v820,
            scalar_v821,
            scalar_v825,
            scalar_v826,
            scalar_v830,
            scalar_v831,
            scalar_v835,
            scalar_v836,
            scalar_v840,
            scalar_v844,
            scalar_v845,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v849,
            scalar_v850,
            scalar_v895,
            scalar_v896,
            scalar_v925,
            scalar_v929,
            scalar_v947,
            scalar_v948,
            scalar_v973,
            scalar_v977,
            scalar_v981,
            scalar_v1006,
            scalar_v1015,
            scalar_v1040,
            scalar_v1046,
            scalar_v1063,
            scalar_v1072,
            scalar_v1101,
            scalar_v1102,
            scalar_v1126,
            scalar_v1130,
            scalar_v1186,
            scalar_v1187,
            scalar_v1210,
            scalar_v1214,
            scalar_v1228,
            scalar_v1229,
            scalar_v1254,
            scalar_v1258,
            scalar_v1262,
            scalar_v1263,
            scalar_v1264,
            scalar_v1265,
            scalar_v1289,
            scalar_v1293,
            scalar_v1297,
            scalar_v1312,
            scalar_v1313,
            scalar_v1314,
            scalar_v1337,
            scalar_v1338,
            scalar_v1340,
            scalar_v1341,
            scalar_v1345,
            scalar_v1349,
            scalar_v1372,
            scalar_v1373,
            scalar_v1392,
            scalar_v1394,
            scalar_v1412,
            scalar_v1415,
            scalar_v1450,
            scalar_v1460,
            scalar_v1475,
            scalar_v1477,
            scalar_v1480,
            scalar_v1557,
            scalar_v1561,
            scalar_v1609,
            scalar_v1654,
            scalar_v1655,
            scalar_v1692,
            scalar_v1696,
            scalar_v1711,
            scalar_v1712,
            scalar_v1713,
            scalar_v1734,
            scalar_v1739,
            scalar_v1761,
            scalar_v1770,
            scalar_v1780,
            scalar_v1783,
            scalar_v1791,
            scalar_v1792,
            scalar_v1816,
            scalar_v1820,
            scalar_v1829,
            scalar_v1833,
            scalar_v1840,
            scalar_v1843,
            scalar_v1844,
            scalar_v1845,
            scalar_v1846,
            scalar_v1847,
            scalar_v1857,
            scalar_v1861,
            scalar_v1873,
            scalar_v1874,
            scalar_v1891,
            scalar_v1895,
            scalar_v1896,
            scalar_v1897,
            scalar_v1916,
            scalar_v1917,
            scalar_v1918,
            scalar_v1942,
            scalar_v1943,
            scalar_v1948,
            scalar_v1958,
            scalar_v1969,
            scalar_v1978,
            scalar_v1991,
            scalar_v2011,
            scalar_v2029,
            scalar_v2040,
            scalar_v2051,
            scalar_v2099,
            scalar_v2144,
            scalar_v2504,
            scalar_v2855,
            scalar_v2862,
            scalar_v2863,
            scalar_v2881,
            scalar_v2886,
            scalar_v2887,
            scalar_v2910,
            scalar_v2911,
            scalar_v2918,
            scalar_v2935,
            scalar_v2965,
            scalar_v2966,
            scalar_v2982,
            scalar_v2990,
            scalar_v2991,
            scalar_v3011,
            scalar_v3030,
            scalar_v3039,
            scalar_v3040,
            scalar_v3058,
            scalar_v3060,
            scalar_v3078,
            scalar_v3107,
            scalar_v3117,
            scalar_v3136,
            scalar_v3137,
            scalar_v3159,
            scalar_v3160,
            scalar_v3183,
            scalar_v3184,
            scalar_v3187,
            scalar_v3256,
            scalar_v3287,
            scalar_v3320,
            scalar_v3321,
            scalar_v3339,
            scalar_v3464,
            scalar_v3465,
            scalar_v3468,
            scalar_v3537,
            scalar_v3568,
            scalar_v3601,
            scalar_v3602,
            scalar_v3604,
            scalar_v3606,
            scalar_v3675,
            scalar_v3706,
            scalar_v3707,
            scalar_v3742,
            scalar_v3743,
            scalar_v3755,
            scalar_v3756,
            scalar_v3760,
            scalar_v3761,
            scalar_v3763,
            scalar_v3766,
            scalar_v3767,
            scalar_v3785,
            scalar_v3787,
            scalar_v3788,
            scalar_v3789,
            scalar_v3794,
            scalar_v3795,
            scalar_v3796,
            scalar_v3797,
            scalar_v3845,
            scalar_v3846,
            scalar_v3848,
            scalar_v3880,
            scalar_v3887,
            scalar_v3888,
            scalar_v3889,
            scalar_v3890,
            scalar_v3891,
            scalar_v3892,
            scalar_v3893,
            scalar_v3894,
            scalar_v3895,
            scalar_v3896,
            scalar_v3897,
            scalar_v3898,
            scalar_v3899,
            scalar_v3900,
            scalar_v3901,
            scalar_v3902,
            scalar_v3903,
            scalar_v3904,
            scalar_v3905,
            scalar_v3906,
            scalar_v3907,
            scalar_v3908,
            scalar_v3909,
            scalar_v3910,
            scalar_v3914,
            scalar_v3915,
            scalar_v3916,
            scalar_v3917,
            scalar_v3918,
            scalar_v3919,
            scalar_v3926,
            scalar_v3929,
            scalar_v3930,
            scalar_v3931,
            scalar_v3944,
            scalar_v3963,
            scalar_v3968,
            scalar_v3987,
            scalar_v3990,
            scalar_v3995,
            scalar_v3997,
            scalar_v4004,
            scalar_v4011,
            scalar_v4033,
            scalar_v4036,
            scalar_v4037,
            scalar_v5647,
            scalar_v5648,
            scalar_v5651,
            scalar_v5652,
            scalar_v5653,
            scalar_v5698,
            scalar_v5699,
            scalar_v5700,
            scalar_v23551,
            scalar_v23552,
            scalar_v26252,
            scalar_v26253,
            scalar_v28544,
            scalar_v28545,
            scalar_v29048,
            scalar_v29197,
            scalar_v29305,
            scalar_v29306,
            scalar_v29307,
            scalar_v29308,
            scalar_v29309,
            scalar_v29310,
            scalar_v29521,
            scalar_v29522,
            scalar_v29523,
            scalar_v29587,
            scalar_v29588,
            scalar_v29610,
            scalar_v30261,
            scalar_v30268,
            scalar_v30285,
            scalar_v30286,
            scalar_v30287,
            scalar_v30304,
            scalar_v30311,
            scalar_v30328,
            scalar_v30329,
            scalar_v30330,
            scalar_v30331,
            scalar_v30332,
            scalar_v30385,
            scalar_v30517,
            scalar_v190,
            scalar_v192,
            scalar_v193,
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
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v278,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
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
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v343,
            scalar_v345,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v360,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v379,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v389,
            scalar_v390,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v435,
            scalar_v436,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v461,
            scalar_v462,
            scalar_v473,
            scalar_v475,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v524,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v545,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v561,
            scalar_v569,
            scalar_v571,
            scalar_v577,
            scalar_v591,
            scalar_v596,
            scalar_v609,
            scalar_v610,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v630,
            scalar_v631,
            scalar_v632,
            scalar_v633,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v644,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v654,
            scalar_v655,
            scalar_v657,
            scalar_v658,
            scalar_v659,
            scalar_v660,
            scalar_v675,
            scalar_v676,
            scalar_v677,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v685,
            scalar_v686,
            scalar_v687,
            scalar_v688,
            scalar_v689,
            scalar_v690,
            scalar_v691,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v707,
            scalar_v708,
            scalar_v709,
            scalar_v713,
            scalar_v714,
            scalar_v715,
            scalar_v716,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v720,
            scalar_v721,
            scalar_v722,
            scalar_v723,
            scalar_v724,
            scalar_v725,
            scalar_v726,
            scalar_v727,
            scalar_v728,
            scalar_v729,
            scalar_v730,
            scalar_v731,
            scalar_v732,
            scalar_v733,
            scalar_v734,
            scalar_v738,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v745,
            scalar_v746,
            scalar_v747,
            scalar_v750,
            scalar_v752,
            scalar_v753,
            scalar_v754,
            scalar_v755,
            scalar_v757,
            scalar_v758,
            scalar_v759,
            scalar_v762,
            scalar_v763,
            scalar_v764,
            scalar_v780,
            scalar_v781,
            scalar_v782,
            scalar_v783,
            scalar_v784,
            scalar_v785,
            scalar_v786,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v790,
            scalar_v791,
            scalar_v792,
            scalar_v793,
            scalar_v794,
            scalar_v795,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v808,
            scalar_v809,
            scalar_v810,
            scalar_v813,
            scalar_v814,
            scalar_v815,
            scalar_v817,
            scalar_v818,
            scalar_v819,
            scalar_v822,
            scalar_v823,
            scalar_v824,
            scalar_v827,
            scalar_v828,
            scalar_v829,
            scalar_v832,
            scalar_v833,
            scalar_v834,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v924,
            scalar_v972,
            scalar_v1045,
            scalar_v1125,
            scalar_v1209,
            scalar_v1253,
            scalar_v1339,
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
            scalar_v24,
            scalar_v26,
            scalar_v29,
            scalar_v31,
            scalar_v32,
            scalar_v34,
            scalar_v36,
            scalar_v37,
            scalar_v39,
            scalar_v41,
            scalar_v43,
            scalar_v44,
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
            scalar_v56,
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
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
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
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
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
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
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
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v219,
            scalar_v220,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v234,
            scalar_v257,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v279,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v289,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v323,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v338,
            scalar_v344,
            scalar_v346,
            scalar_v351,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v372,
            scalar_v373,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v380,
            scalar_v388,
            scalar_v391,
            scalar_v392,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v434,
            scalar_v437,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v454,
            scalar_v455,
            scalar_v460,
            scalar_v463,
            scalar_v464,
            scalar_v468,
            scalar_v470,
            scalar_v471,
            scalar_v491,
            scalar_v492,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v503,
            scalar_v504,
            scalar_v505,
            scalar_v506,
            scalar_v507,
            scalar_v508,
            scalar_v525,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v540,
            scalar_v544,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v555,
            scalar_v563,
            scalar_v564,
            scalar_v565,
            scalar_v566,
            scalar_v567,
            scalar_v583,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v602,
            scalar_v608,
            scalar_v611,
            scalar_v616,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v640,
            scalar_v645,
            scalar_v646,
            scalar_v647,
            scalar_v648,
            scalar_v656,
            scalar_v661,
            scalar_v662,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v666,
            scalar_v667,
            scalar_v668,
            scalar_v669,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v674,
            scalar_v692,
            scalar_v700,
            scalar_v701,
            scalar_v705,
            scalar_v706,
            scalar_v710,
            scalar_v711,
            scalar_v712,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v739,
            scalar_v740,
            scalar_v744,
            scalar_v748,
            scalar_v749,
            scalar_v751,
            scalar_v756,
            scalar_v760,
            scalar_v761,
            scalar_v765,
            scalar_v766,
            scalar_v767,
            scalar_v768,
            scalar_v769,
            scalar_v770,
            scalar_v771,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v775,
            scalar_v776,
            scalar_v777,
            scalar_v778,
            scalar_v779,
            scalar_v796,
            scalar_v803,
            scalar_v804,
            scalar_v805,
            scalar_v806,
            scalar_v807,
            scalar_v811,
            scalar_v812,
            scalar_v816,
            scalar_v820,
            scalar_v821,
            scalar_v825,
            scalar_v826,
            scalar_v830,
            scalar_v831,
            scalar_v835,
            scalar_v836,
            scalar_v840,
            scalar_v844,
            scalar_v845,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v849,
            scalar_v850,
            scalar_v895,
            scalar_v896,
            scalar_v925,
            scalar_v929,
            scalar_v947,
            scalar_v948,
            scalar_v973,
            scalar_v977,
            scalar_v981,
            scalar_v1006,
            scalar_v1015,
            scalar_v1040,
            scalar_v1046,
            scalar_v1063,
            scalar_v1072,
            scalar_v1101,
            scalar_v1102,
            scalar_v1126,
            scalar_v1130,
            scalar_v1186,
            scalar_v1187,
            scalar_v1210,
            scalar_v1214,
            scalar_v1228,
            scalar_v1229,
            scalar_v1254,
            scalar_v1258,
            scalar_v1262,
            scalar_v1263,
            scalar_v1264,
            scalar_v1265,
            scalar_v1289,
            scalar_v1293,
            scalar_v1297,
            scalar_v1312,
            scalar_v1313,
            scalar_v1314,
            scalar_v1337,
            scalar_v1338,
            scalar_v1340,
            scalar_v1341,
            scalar_v1345,
            scalar_v1349,
            scalar_v1372,
            scalar_v1373,
            scalar_v1392,
            scalar_v1394,
            scalar_v1412,
            scalar_v1415,
            scalar_v1450,
            scalar_v1460,
            scalar_v1475,
            scalar_v1477,
            scalar_v1480,
            scalar_v1557,
            scalar_v1561,
            scalar_v1609,
            scalar_v1654,
            scalar_v1655,
            scalar_v1692,
            scalar_v1696,
            scalar_v1711,
            scalar_v1712,
            scalar_v1713,
            scalar_v1734,
            scalar_v1739,
            scalar_v1761,
            scalar_v1770,
            scalar_v1780,
            scalar_v1783,
            scalar_v1791,
            scalar_v1792,
            scalar_v1816,
            scalar_v1820,
            scalar_v1829,
            scalar_v1833,
            scalar_v1840,
            scalar_v1843,
            scalar_v1844,
            scalar_v1845,
            scalar_v1846,
            scalar_v1847,
            scalar_v1857,
            scalar_v1861,
            scalar_v1873,
            scalar_v1874,
            scalar_v1891,
            scalar_v1895,
            scalar_v1896,
            scalar_v1897,
            scalar_v1916,
            scalar_v1917,
            scalar_v1918,
            scalar_v1942,
            scalar_v1943,
            scalar_v1948,
            scalar_v1958,
            scalar_v1969,
            scalar_v1978,
            scalar_v1991,
            scalar_v2011,
            scalar_v2029,
            scalar_v2040,
            scalar_v2051,
            scalar_v2099,
            scalar_v2144,
            scalar_v2504,
            scalar_v2855,
            scalar_v2862,
            scalar_v2863,
            scalar_v2881,
            scalar_v2886,
            scalar_v2887,
            scalar_v2910,
            scalar_v2911,
            scalar_v2918,
            scalar_v2935,
            scalar_v2965,
            scalar_v2966,
            scalar_v2982,
            scalar_v2990,
            scalar_v2991,
            scalar_v3011,
            scalar_v3030,
            scalar_v3039,
            scalar_v3040,
            scalar_v3058,
            scalar_v3060,
            scalar_v3078,
            scalar_v3107,
            scalar_v3117,
            scalar_v3136,
            scalar_v3137,
            scalar_v3159,
            scalar_v3160,
            scalar_v3183,
            scalar_v3184,
            scalar_v3187,
            scalar_v3256,
            scalar_v3287,
            scalar_v3320,
            scalar_v3321,
            scalar_v3339,
            scalar_v3464,
            scalar_v3465,
            scalar_v3468,
            scalar_v3537,
            scalar_v3568,
            scalar_v3601,
            scalar_v3602,
            scalar_v3604,
            scalar_v3606,
            scalar_v3675,
            scalar_v3706,
            scalar_v3707,
            scalar_v3742,
            scalar_v3743,
            scalar_v3755,
            scalar_v3756,
            scalar_v3760,
            scalar_v3761,
            scalar_v3763,
            scalar_v3766,
            scalar_v3767,
            scalar_v3785,
            scalar_v3787,
            scalar_v3788,
            scalar_v3789,
            scalar_v3794,
            scalar_v3795,
            scalar_v3796,
            scalar_v3797,
            scalar_v3845,
            scalar_v3846,
            scalar_v3848,
            scalar_v3880,
            scalar_v3887,
            scalar_v3888,
            scalar_v3889,
            scalar_v3890,
            scalar_v3891,
            scalar_v3892,
            scalar_v3893,
            scalar_v3894,
            scalar_v3895,
            scalar_v3896,
            scalar_v3897,
            scalar_v3898,
            scalar_v3899,
            scalar_v3900,
            scalar_v3901,
            scalar_v3902,
            scalar_v3903,
            scalar_v3904,
            scalar_v3905,
            scalar_v3906,
            scalar_v3907,
            scalar_v3908,
            scalar_v3909,
            scalar_v3910,
            scalar_v3914,
            scalar_v3915,
            scalar_v3916,
            scalar_v3917,
            scalar_v3918,
            scalar_v3919,
            scalar_v3926,
            scalar_v3929,
            scalar_v3930,
            scalar_v3931,
            scalar_v3944,
            scalar_v3963,
            scalar_v3968,
            scalar_v3987,
            scalar_v3990,
            scalar_v3995,
            scalar_v3997,
            scalar_v4004,
            scalar_v4011,
            scalar_v4033,
            scalar_v4036,
            scalar_v4037,
            scalar_v5647,
            scalar_v5648,
            scalar_v5651,
            scalar_v5652,
            scalar_v5653,
            scalar_v5698,
            scalar_v5699,
            scalar_v5700,
            scalar_v23551,
            scalar_v23552,
            scalar_v26252,
            scalar_v26253,
            scalar_v28544,
            scalar_v28545,
            scalar_v29048,
            scalar_v29197,
            scalar_v29305,
            scalar_v29306,
            scalar_v29307,
            scalar_v29308,
            scalar_v29309,
            scalar_v29310,
            scalar_v29521,
            scalar_v29522,
            scalar_v29523,
            scalar_v29587,
            scalar_v29588,
            scalar_v29610,
            scalar_v30261,
            scalar_v30268,
            scalar_v30285,
            scalar_v30286,
            scalar_v30287,
            scalar_v30304,
            scalar_v30311,
            scalar_v30328,
            scalar_v30329,
            scalar_v30330,
            scalar_v30331,
            scalar_v30332,
            scalar_v30385,
            scalar_v30517,
            scalar_v190,
            scalar_v192,
            scalar_v193,
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
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v278,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v288,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
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
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v343,
            scalar_v345,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v360,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v379,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v389,
            scalar_v390,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v435,
            scalar_v436,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v461,
            scalar_v462,
            scalar_v473,
            scalar_v475,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v520,
            scalar_v521,
            scalar_v522,
            scalar_v523,
            scalar_v524,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v545,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v561,
            scalar_v569,
            scalar_v571,
            scalar_v577,
            scalar_v591,
            scalar_v596,
            scalar_v609,
            scalar_v610,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v630,
            scalar_v631,
            scalar_v632,
            scalar_v633,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v638,
            scalar_v639,
            scalar_v641,
            scalar_v642,
            scalar_v643,
            scalar_v644,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v653,
            scalar_v654,
            scalar_v655,
            scalar_v657,
            scalar_v658,
            scalar_v659,
            scalar_v660,
            scalar_v675,
            scalar_v676,
            scalar_v677,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v685,
            scalar_v686,
            scalar_v687,
            scalar_v688,
            scalar_v689,
            scalar_v690,
            scalar_v691,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v707,
            scalar_v708,
            scalar_v709,
            scalar_v713,
            scalar_v714,
            scalar_v715,
            scalar_v716,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v720,
            scalar_v721,
            scalar_v722,
            scalar_v723,
            scalar_v724,
            scalar_v725,
            scalar_v726,
            scalar_v727,
            scalar_v728,
            scalar_v729,
            scalar_v730,
            scalar_v731,
            scalar_v732,
            scalar_v733,
            scalar_v734,
            scalar_v738,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v745,
            scalar_v746,
            scalar_v747,
            scalar_v750,
            scalar_v752,
            scalar_v753,
            scalar_v754,
            scalar_v755,
            scalar_v757,
            scalar_v758,
            scalar_v759,
            scalar_v762,
            scalar_v763,
            scalar_v764,
            scalar_v780,
            scalar_v781,
            scalar_v782,
            scalar_v783,
            scalar_v784,
            scalar_v785,
            scalar_v786,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v790,
            scalar_v791,
            scalar_v792,
            scalar_v793,
            scalar_v794,
            scalar_v795,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v808,
            scalar_v809,
            scalar_v810,
            scalar_v813,
            scalar_v814,
            scalar_v815,
            scalar_v817,
            scalar_v818,
            scalar_v819,
            scalar_v822,
            scalar_v823,
            scalar_v824,
            scalar_v827,
            scalar_v828,
            scalar_v829,
            scalar_v832,
            scalar_v833,
            scalar_v834,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v924,
            scalar_v972,
            scalar_v1045,
            scalar_v1125,
            scalar_v1209,
            scalar_v1253,
            scalar_v1339,
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "c10" => { validate_parameter("c10", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qp0" => { validate_parameter("qp0", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hf0" => { validate_parameter("hf0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfe" => { validate_parameter("hfe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfb" => { validate_parameter("hfb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hfc" => { validate_parameter("hfc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hr0" => { validate_parameter("hr0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjei0" => { validate_parameter("hjei0", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjei" => { validate_parameter("hjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahjei" => { validate_parameter("ahjei", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rhjei" => { validate_parameter("rhjei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hjci" => { validate_parameter("hjci", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeis" => { validate_parameter("ibeis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbei" => { validate_parameter("mbei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ireis" => { validate_parameter("ireis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mrei" => { validate_parameter("mrei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeps" => { validate_parameter("ibeps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbep" => { validate_parameter("mbep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ireps" => { validate_parameter("ireps", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mrep" => { validate_parameter("mrep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbhrec" => { validate_parameter("tbhrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcis" => { validate_parameter("ibcis", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbci" => { validate_parameter("mbci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcxs" => { validate_parameter("ibcxs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbcx" => { validate_parameter("mbcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibets" => { validate_parameter("ibets", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abet" => { validate_parameter("abet", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tunode" => { validate_parameter("tunode", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibetat0" => { validate_parameter("ibetat0", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbetat" => { validate_parameter("vbetat", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hcavl" => { validate_parameter("hcavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hvdavl" => { validate_parameter("hvdavl", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcts" => { validate_parameter("ibcts", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abct" => { validate_parameter("abct", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjei0" => { validate_parameter("cjei0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdei" => { validate_parameter("vdei", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zei" => { validate_parameter("zei", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajei" => { validate_parameter("ajei", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjep0" => { validate_parameter("cjep0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdep" => { validate_parameter("vdep", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zep" => { validate_parameter("zep", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajep" => { validate_parameter("ajep", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajci" => { validate_parameter("ajci", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajcx" => { validate_parameter("ajcx", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajs" => { validate_parameter("ajs", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cscp0" => { validate_parameter("cscp0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdsp" => { validate_parameter("vdsp", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zsp" => { validate_parameter("zsp", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptsp" => { validate_parameter("vptsp", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbvl" => { validate_finite_parameter("tbvl", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gtfe" => { validate_parameter("gtfe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fthc" => { validate_parameter("fthc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.2, "1.2")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcsm" => { validate_parameter("avcsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vcbar" => { validate_parameter("vcbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "icbar" => { validate_parameter("icbar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbar" => { validate_parameter("acbar", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fdqr0" => { validate_parameter("fdqr0", value, Some((-0.5, "-0.5")), false, Some((100.0, "100.0")), false, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fcrbi" => { validate_parameter("fcrbi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fqi" => { validate_parameter("fqi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tsf" => { validate_parameter("tsf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsu" => { validate_parameter("rsu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csu" => { validate_parameter("csu", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbepar" => { validate_parameter("fbepar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbcpar" => { validate_parameter("fbcpar", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccepar" => { validate_parameter("ccepar", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flcono" => { validate_parameter("flcono", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfbe" => { validate_parameter("cfbe", value, Some((-2.0, "-2.0")), false, Some((-1.0, "-1.0")), false, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfre" => { validate_parameter("kfre", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afre" => { validate_parameter("afre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "latb" => { validate_parameter("latb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "latl" => { validate_parameter("latl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f2vg" => { validate_finite_parameter("f2vg", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetact" => { validate_parameter("zetact", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetabet" => { validate_parameter("zetabet", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgbe" => { validate_parameter("dvgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetahjei" => { validate_parameter("zetahjei", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetavgbe" => { validate_parameter("zetavgbe", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaci" => { validate_parameter("zetaci", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbi" => { validate_parameter("zetarbi", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbx" => { validate_parameter("zetarbx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarcx" => { validate_parameter("zetarcx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetare" => { validate_parameter("zetare", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetacx" => { validate_parameter("zetacx", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarth" => { validate_parameter("zetarth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("tnom", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
        let v0: f64 = p.p148;
        self.scalar_v0 = v0;
        let v24: f64 = p.p0;
        self.scalar_v24 = v24;
        let v26: bool = (p.p0 <= 310.0);
        self.scalar_v26 = v26;
        let v29: f64 = (if v26 { 1.6021918e-19 } else { 0.0 });
        self.scalar_v29 = v29;
        let v31: f64 = (if v26 { 1.3806226e-23 } else { 0.0 });
        self.scalar_v31 = v31;
        let v32: bool = (!v26);
        self.scalar_v32 = v32;
        let v34: f64 = (if v32 { 1.602176634e-19 } else { v29 });
        self.scalar_v34 = v34;
        let v36: f64 = (if v32 { 1.380649e-23 } else { v31 });
        self.scalar_v36 = v36;
        let v37: f64 = p.p146;
        self.scalar_v37 = v37;
        let v39: f64 = (p.p146 + 273.15);
        self.scalar_v39 = v39;
        let v41: f64 = (v36 / v34);
        self.scalar_v41 = v41;
        let v43: f64 = (v41 * 300.0);
        self.scalar_v43 = v43;
        let v44: f64 = (v39 * v41);
        self.scalar_v44 = v44;
        let v46: f64 = (1.0 / v44);
        self.scalar_v46 = v46;
        let v47: f64 = p.p121;
        self.scalar_v47 = v47;
        let v48: f64 = (v39 * p.p121);
        self.scalar_v48 = v48;
        let v49: f64 = ((v39) as f64).ln();
        self.scalar_v49 = v49;
        let v50: f64 = (v48 * v49);
        self.scalar_v50 = v50;
        let v51: f64 = p.p122;
        self.scalar_v51 = v51;
        let v52: f64 = (v39 * p.p122);
        self.scalar_v52 = v52;
        let v53: f64 = p.p131;
        self.scalar_v53 = v53;
        let v54: f64 = (v39 * p.p131);
        self.scalar_v54 = v54;
        let v55: f64 = p.p117;
        self.scalar_v55 = v55;
        let v56: f64 = (v50 + p.p117);
        self.scalar_v56 = v56;
        let v57: f64 = (v52 + v56);
        self.scalar_v57 = v57;
        let v58: f64 = p.p118;
        self.scalar_v58 = v58;
        let v59: f64 = (v50 + p.p118);
        self.scalar_v59 = v59;
        let v60: f64 = (v52 + v59);
        self.scalar_v60 = v60;
        let v61: f64 = p.p119;
        self.scalar_v61 = v61;
        let v62: f64 = (v50 + p.p119);
        self.scalar_v62 = v62;
        let v63: f64 = (v52 + v62);
        self.scalar_v63 = v63;
        let v64: f64 = (v57 + v60);
        self.scalar_v64 = v64;
        let v66: f64 = (v64 * 0.5);
        self.scalar_v66 = v66;
        let v67: f64 = (v57 + v63);
        self.scalar_v67 = v67;
        let v68: f64 = (0.5 * v67);
        self.scalar_v68 = v68;
        let v69: f64 = (p.p117 + p.p118);
        self.scalar_v69 = v69;
        let v70: f64 = (0.5 * v69);
        self.scalar_v70 = v70;
        let v71: f64 = (p.p117 + p.p119);
        self.scalar_v71 = v71;
        let v72: f64 = (0.5 * v71);
        self.scalar_v72 = v72;
        let v73: f64 = p.p120;
        self.scalar_v73 = v73;
        let v74: f64 = (p.p119 + p.p120);
        self.scalar_v74 = v74;
        let v75: f64 = (0.5 * v74);
        self.scalar_v75 = v75;
        let v77: f64 = (p.p121 / v41);
        self.scalar_v77 = v77;
        let v78: f64 = (3.0 - v77);
        self.scalar_v78 = v78;
        let v79: f64 = (1.0 + v78);
        self.scalar_v79 = v79;
        let v80: f64 = p.p130;
        self.scalar_v80 = v80;
        let v81: f64 = (v79 - p.p130);
        self.scalar_v81 = v81;
        let v82: f64 = p.p138;
        self.scalar_v82 = v82;
        let v83: f64 = (v79 - p.p138);
        self.scalar_v83 = v83;
        let v85: f64 = (v78 - 1.5);
        self.scalar_v85 = v85;
        let v86: f64 = p.p107;
        self.scalar_v86 = v86;
        let v87: f64 = (1.0 - p.p107);
        self.scalar_v87 = v87;
        let v88: f64 = p.p52;
        self.scalar_v88 = v88;
        let v89: f64 = p.p106;
        self.scalar_v89 = v89;
        let v90: f64 = (p.p52 + p.p106);
        self.scalar_v90 = v90;
        let v91: f64 = (v87 * v90);
        self.scalar_v91 = v91;
        let v92: bool = (v91 >= p.p106);
        self.scalar_v92 = v92;
        let v93: f64 = (if v92 { p.p106 } else { 0.0 });
        self.scalar_v93 = v93;
        let v94: f64 = (v91 - p.p106);
        self.scalar_v94 = v94;
        let v95: f64 = (if v92 { v94 } else { 0.0 });
        self.scalar_v95 = v95;
        let v96: f64 = (p.p52 - v95);
        self.scalar_v96 = v96;
        let v97: f64 = (if v92 { v96 } else { 0.0 });
        self.scalar_v97 = v97;
        let v98: bool = (!v92);
        self.scalar_v98 = v98;
        let v99: f64 = (if v98 { v91 } else { v93 });
        self.scalar_v99 = v99;
        let v100: f64 = (p.p106 - v99);
        self.scalar_v100 = v100;
        let v101: f64 = (if v98 { v100 } else { 0.0 });
        self.scalar_v101 = v101;
        let v102: f64 = (if v98 { 0.0 } else { v95 });
        self.scalar_v102 = v102;
        let v103: f64 = (if v98 { p.p52 } else { v97 });
        self.scalar_v103 = v103;
        let v104: f64 = p.p105;
        self.scalar_v104 = v104;
        let v105: f64 = p.p104;
        self.scalar_v105 = v105;
        let v106: f64 = (p.p105 * p.p104);
        self.scalar_v106 = v106;
        let v107: f64 = (p.p104 - v106);
        self.scalar_v107 = v107;
        let v108: f64 = p.p22;
        self.scalar_v108 = v108;
        let v109: bool = (0.0 != p.p22);
        self.scalar_v109 = v109;
        let v110: f64 = (1.0 / p.p22);
        self.scalar_v110 = v110;
        let v111: f64 = (if v109 { v110 } else { 0.0 });
        self.scalar_v111 = v111;
        let v112: bool = (!v109);
        self.scalar_v112 = v112;
        let v113: f64 = (if v112 { 0.0 } else { v111 });
        self.scalar_v113 = v113;
        let v114: bool = (p.p0 <= 300.0);
        self.scalar_v114 = v114;
        let v115: bool = (!v114);
        self.scalar_v115 = v115;
        let v117: f64 = (if v115 { 0.7 } else { 0.0 });
        self.scalar_v117 = v117;
        let v118: f64 = p.p32;
        self.scalar_v118 = v118;
        let v119: bool = (p.p32 > 0.0);
        self.scalar_v119 = v119;
        let v120: f64 = p.p47;
        self.scalar_v120 = v120;
        let v121: bool = (p.p47 > 0.0);
        self.scalar_v121 = v121;
        let v122: bool = (v119 && v121);
        self.scalar_v122 = v122;
        let v123: f64 = (if v122 { 1.0 } else { 0.0 });
        self.scalar_v123 = v123;
        let v124: bool = (!v122);
        self.scalar_v124 = v124;
        let v125: f64 = (if v124 { 0.0 } else { v123 });
        self.scalar_v125 = v125;
        let v126: f64 = p.p86;
        self.scalar_v126 = v126;
        let v127: bool = (0.0 != p.p86);
        self.scalar_v127 = v127;
        let v128: f64 = p.p88;
        self.scalar_v128 = v128;
        let v129: bool = (0.0 == p.p88);
        self.scalar_v129 = v129;
        let v130: f64 = p.p87;
        self.scalar_v130 = v130;
        let v131: bool = (0.0 == p.p87);
        self.scalar_v131 = v131;
        let v132: bool = (v129 && v131);
        self.scalar_v132 = v132;
        let v133: f64 = p.p66;
        self.scalar_v133 = v133;
        let v134: bool = (0.0 == p.p66);
        self.scalar_v134 = v134;
        let v135: bool = (v132 || v134);
        self.scalar_v135 = v135;
        let v136: bool = (v127 && v135);
        self.scalar_v136 = v136;
        let v137: f64 = (if v136 { 0.0 } else { p.p86 });
        self.scalar_v137 = v137;
        let v138: f64 = p.p115;
        self.scalar_v138 = v138;
        let v140: bool = (p.p115 >= 0.01);
        self.scalar_v140 = v140;
        let v141: f64 = p.p116;
        self.scalar_v141 = v141;
        let v142: bool = (p.p116 >= 0.01);
        self.scalar_v142 = v142;
        let v143: bool = (v140 || v142);
        self.scalar_v143 = v143;
        let v144: f64 = (p.p115 - p.p116);
        self.scalar_v144 = v144;
        let v145: f64 = (0.5 * v144);
        self.scalar_v145 = v145;
        let v146: f64 = (if v143 { v145 } else { 0.0 });
        self.scalar_v146 = v146;
        let v147: bool = (p.p116 < p.p115);
        self.scalar_v147 = v147;
        let v148: bool = (v143 && v147);
        self.scalar_v148 = v148;
        let v149: f64 = (if v148 { p.p116 } else { 0.0 });
        self.scalar_v149 = v149;
        let v150: f64 = (if v148 { p.p115 } else { 0.0 });
        self.scalar_v150 = v150;
        let v151: bool = (!v147);
        self.scalar_v151 = v151;
        let v152: bool = (v143 && v151);
        self.scalar_v152 = v152;
        let v153: f64 = (if v152 { p.p115 } else { v149 });
        self.scalar_v153 = v153;
        let v154: f64 = (if v152 { p.p116 } else { v150 });
        self.scalar_v154 = v154;
        let v155: bool = (v153 < 0.01);
        self.scalar_v155 = v155;
        let v156: bool = (v143 && v155);
        self.scalar_v156 = v156;
        let v158: f64 = (if v156 { 1000000000.0 } else { 0.0 });
        self.scalar_v158 = v158;
        let v160: f64 = (if v156 { 170000000.0 } else { 0.0 });
        self.scalar_v160 = v160;
        let v161: f64 = (1.0 + v154);
        self.scalar_v161 = v161;
        let v162: f64 = ((v161) as f64).ln();
        self.scalar_v162 = v162;
        let v163: f64 = (if v156 { v162 } else { 0.0 });
        self.scalar_v163 = v163;
        let v164: bool = (!v155);
        self.scalar_v164 = v164;
        let v165: bool = (v143 && v164);
        self.scalar_v165 = v165;
        let v166: f64 = (1.0 / p.p115);
        self.scalar_v166 = v166;
        let v167: f64 = (if v165 { v166 } else { v158 });
        self.scalar_v167 = v167;
        let v168: f64 = (1.0 / p.p116);
        self.scalar_v168 = v168;
        let v169: f64 = (if v165 { v168 } else { v158 });
        self.scalar_v169 = v169;
        let v171: f64 = (p.p115 / 6.0);
        self.scalar_v171 = v171;
        let v172: f64 = (if v165 { v171 } else { v160 });
        self.scalar_v172 = v172;
        let v173: f64 = (p.p116 / 6.0);
        self.scalar_v173 = v173;
        let v174: f64 = (if v165 { v173 } else { v160 });
        self.scalar_v174 = v174;
        let v175: f64 = (1.0 + p.p115);
        self.scalar_v175 = v175;
        let v176: f64 = (1.0 + p.p116);
        self.scalar_v176 = v176;
        let v177: f64 = (v175 / v176);
        self.scalar_v177 = v177;
        let v178: f64 = ((v177) as f64).ln();
        self.scalar_v178 = v178;
        let v179: f64 = (if v165 { v178 } else { v163 });
        self.scalar_v179 = v179;
        let v180: bool = (!v143);
        self.scalar_v180 = v180;
        let v181: f64 = (if v180 { 0.0 } else { v146 });
        self.scalar_v181 = v181;
        let v182: f64 = (if v180 { 1000000000.0 } else { v167 });
        self.scalar_v182 = v182;
        let v183: f64 = (if v180 { 1000000000.0 } else { v169 });
        self.scalar_v183 = v183;
        let v184: f64 = (if v180 { 170000000.0 } else { v172 });
        self.scalar_v184 = v184;
        let v185: f64 = (if v180 { 170000000.0 } else { v174 });
        self.scalar_v185 = v185;
        let v186: f64 = (if v180 { p.p116 } else { v153 });
        self.scalar_v186 = v186;
        let v187: f64 = (if v180 { p.p115 } else { v154 });
        self.scalar_v187 = v187;
        let v188: f64 = (if v180 { 0.0 } else { v179 });
        self.scalar_v188 = v188;
        let v189: f64 = p.p147;
        self.scalar_v189 = v189;
        let v219: f64 = p.p39;
        self.scalar_v219 = v219;
        let v220: bool = (p.p39 > 0.0);
        self.scalar_v220 = v220;
        let v222: f64 = (v44 * 2.0);
        self.scalar_v222 = v222;
        let v223: f64 = p.p40;
        self.scalar_v223 = v223;
        let v224: f64 = (0.5 * p.p40);
        self.scalar_v224 = v224;
        let v225: f64 = (v46 * v224);
        self.scalar_v225 = v225;
        let v226: f64 = ((v225) as f64).exp();
        self.scalar_v226 = v226;
        let v228: f64 = (p.p40 * -0.5);
        self.scalar_v228 = v228;
        let v229: f64 = (v46 * v228);
        self.scalar_v229 = v229;
        let v230: f64 = ((v229) as f64).exp();
        self.scalar_v230 = v230;
        let v231: f64 = (v226 - v230);
        self.scalar_v231 = v231;
        let v232: f64 = ((v231) as f64).ln();
        self.scalar_v232 = v232;
        let v233: f64 = (v222 * v232);
        self.scalar_v233 = v233;
        let v234: f64 = (if v220 { v233 } else { 0.0 });
        self.scalar_v234 = v234;
        let v257: f64 = p.p41;
        self.scalar_v257 = v257;
        let v264: f64 = p.p42;
        self.scalar_v264 = v264;
        let v265: f64 = ((p.p42) as f64).abs();
        self.scalar_v265 = v265;
        let v266: f64 = (if v220 { v265 } else { 0.0 });
        self.scalar_v266 = v266;
        let v267: bool = (p.p42 > 0.0);
        self.scalar_v267 = v267;
        let v268: bool = (v220 && v267);
        self.scalar_v268 = v268;
        let v272: bool = (!v220);
        self.scalar_v272 = v272;
        let v276: f64 = p.p14;
        self.scalar_v276 = v276;
        let v277: f64 = p.p124;
        self.scalar_v277 = v277;
        let v279: f64 = (v46 * p.p118);
        self.scalar_v279 = v279;
        let v285: f64 = p.p16;
        self.scalar_v285 = v285;
        let v286: f64 = p.p17;
        self.scalar_v286 = v286;
        let v287: f64 = (v78 / p.p17);
        self.scalar_v287 = v287;
        let v289: f64 = (v46 * v70);
        self.scalar_v289 = v289;
        let v295: f64 = p.p48;
        self.scalar_v295 = v295;
        let v296: f64 = (0.5 * p.p48);
        self.scalar_v296 = v296;
        let v297: f64 = (v46 * v296);
        self.scalar_v297 = v297;
        let v298: f64 = ((v297) as f64).exp();
        self.scalar_v298 = v298;
        let v299: f64 = (-0.5 * p.p48);
        self.scalar_v299 = v299;
        let v300: f64 = (v46 * v299);
        self.scalar_v300 = v300;
        let v301: f64 = ((v300) as f64).exp();
        self.scalar_v301 = v301;
        let v302: f64 = (v298 - v301);
        self.scalar_v302 = v302;
        let v303: f64 = ((v302) as f64).ln();
        self.scalar_v303 = v303;
        let v304: f64 = (v222 * v303);
        self.scalar_v304 = v304;
        let v305: f64 = (if v121 { v304 } else { v234 });
        self.scalar_v305 = v305;
        let v323: f64 = p.p49;
        self.scalar_v323 = v323;
        let v330: f64 = p.p50;
        self.scalar_v330 = v330;
        let v331: f64 = ((p.p50) as f64).abs();
        self.scalar_v331 = v331;
        let v332: f64 = (if v121 { v331 } else { 0.0 });
        self.scalar_v332 = v332;
        let v333: bool = (p.p50 > 0.0);
        self.scalar_v333 = v333;
        let v334: bool = (v121 && v333);
        self.scalar_v334 = v334;
        let v338: bool = (!v121);
        self.scalar_v338 = v338;
        let v344: f64 = p.p23;
        self.scalar_v344 = v344;
        let v346: f64 = (v46 * p.p119);
        self.scalar_v346 = v346;
        let v351: f64 = p.p2;
        self.scalar_v351 = v351;
        let v358: f64 = p.p1;
        self.scalar_v358 = v358;
        let v359: f64 = p.p123;
        self.scalar_v359 = v359;
        let v361: f64 = (v46 * p.p117);
        self.scalar_v361 = v361;
        let v366: f64 = p.p10;
        self.scalar_v366 = v366;
        let v367: f64 = p.p126;
        self.scalar_v367 = v367;
        let v371: f64 = p.p8;
        self.scalar_v371 = v371;
        let v372: f64 = (p.p8 - 1.0);
        self.scalar_v372 = v372;
        let v373: f64 = ((v372) as f64).abs();
        self.scalar_v373 = v373;
        let v375: bool = (v373 < 1e-5);
        self.scalar_v375 = v375;
        let v376: bool = (v114 && v375);
        self.scalar_v376 = v376;
        let v377: f64 = p.p9;
        self.scalar_v377 = v377;
        let v378: f64 = p.p125;
        self.scalar_v378 = v378;
        let v380: f64 = p.p127;
        self.scalar_v380 = v380;
        let v388: bool = (!v376);
        self.scalar_v388 = v388;
        let v391: f64 = p.p3;
        self.scalar_v391 = v391;
        let v392: f64 = (v46 * p.p125);
        self.scalar_v392 = v392;
        let v396: f64 = p.p4;
        self.scalar_v396 = v396;
        let v397: f64 = (p.p117 - p.p118);
        self.scalar_v397 = v397;
        let v398: f64 = (v46 * v397);
        self.scalar_v398 = v398;
        let v402: f64 = p.p6;
        self.scalar_v402 = v402;
        let v403: f64 = (p.p117 - p.p119);
        self.scalar_v403 = v403;
        let v404: f64 = (v46 * v403);
        self.scalar_v404 = v404;
        let v408: f64 = p.p75;
        self.scalar_v408 = v408;
        let v409: f64 = (p.p130 - v54);
        self.scalar_v409 = v409;
        let v413: f64 = p.p74;
        self.scalar_v413 = v413;
        let v418: f64 = p.p79;
        self.scalar_v418 = v418;
        let v419: bool = (p.p79 > 0.0);
        self.scalar_v419 = v419;
        let v420: f64 = p.p133;
        self.scalar_v420 = v420;
        let v425: f64 = p.p78;
        self.scalar_v425 = v425;
        let v426: f64 = (if v419 { p.p78 } else { 0.0 });
        self.scalar_v426 = v426;
        let v427: bool = (!v419);
        self.scalar_v427 = v427;
        let v428: f64 = p.p132;
        self.scalar_v428 = v428;
        let v434: f64 = p.p128;
        self.scalar_v434 = v434;
        let v437: f64 = p.p129;
        self.scalar_v437 = v437;
        let v442: f64 = p.p69;
        self.scalar_v442 = v442;
        let v443: f64 = p.p71;
        self.scalar_v443 = v443;
        let v444: f64 = (p.p130 - 1.0);
        self.scalar_v444 = v444;
        let v448: bool = (1.0 == v125);
        self.scalar_v448 = v448;
        let v449: f64 = p.p139;
        self.scalar_v449 = v449;
        let v454: f64 = p.p33;
        self.scalar_v454 = v454;
        let v455: f64 = p.p140;
        self.scalar_v455 = v455;
        let v460: bool = (!v448);
        self.scalar_v460 = v460;
        let v463: f64 = p.p37;
        self.scalar_v463 = v463;
        let v464: bool = (p.p37 > 0.0);
        self.scalar_v464 = v464;
        let v468: f64 = p.p38;
        self.scalar_v468 = v468;
        let v470: bool = (p.p48 > 0.0);
        self.scalar_v470 = v470;
        let v471: bool = (v121 && v470);
        self.scalar_v471 = v471;
        let v491: f64 = p.p89;
        self.scalar_v491 = v491;
        let v492: f64 = p.p134;
        self.scalar_v492 = v492;
        let v496: f64 = p.p43;
        self.scalar_v496 = v496;
        let v497: bool = (p.p43 > 0.0);
        self.scalar_v497 = v497;
        let v498: f64 = p.p44;
        self.scalar_v498 = v498;
        let v499: f64 = (0.5 * p.p44);
        self.scalar_v499 = v499;
        let v500: f64 = (v46 * v499);
        self.scalar_v500 = v500;
        let v501: f64 = ((v500) as f64).exp();
        self.scalar_v501 = v501;
        let v502: f64 = (-0.5 * p.p44);
        self.scalar_v502 = v502;
        let v503: f64 = (v46 * v502);
        self.scalar_v503 = v503;
        let v504: f64 = ((v503) as f64).exp();
        self.scalar_v504 = v504;
        let v505: f64 = (v501 - v504);
        self.scalar_v505 = v505;
        let v506: f64 = ((v505) as f64).ln();
        self.scalar_v506 = v506;
        let v507: f64 = (v222 * v506);
        self.scalar_v507 = v507;
        let v508: f64 = (if v497 { v507 } else { v305 });
        self.scalar_v508 = v508;
        let v525: f64 = p.p45;
        self.scalar_v525 = v525;
        let v532: f64 = p.p46;
        self.scalar_v532 = v532;
        let v533: f64 = ((p.p46) as f64).abs();
        self.scalar_v533 = v533;
        let v534: f64 = (if v497 { v533 } else { 0.0 });
        self.scalar_v534 = v534;
        let v535: bool = (p.p46 > 0.0);
        self.scalar_v535 = v535;
        let v536: bool = (v497 && v535);
        self.scalar_v536 = v536;
        let v540: bool = (!v497);
        self.scalar_v540 = v540;
        let v544: f64 = p.p18;
        self.scalar_v544 = v544;
        let v546: f64 = p.p20;
        self.scalar_v546 = v546;
        let v547: f64 = p.p21;
        self.scalar_v547 = v547;
        let v548: f64 = (v78 / p.p21);
        self.scalar_v548 = v548;
        let v554: f64 = p.p27;
        self.scalar_v554 = v554;
        let v555: bool = (p.p27 > 0.0);
        self.scalar_v555 = v555;
        let v563: f64 = p.p29;
        self.scalar_v563 = v563;
        let v564: bool = (1.0 == p.p29);
        self.scalar_v564 = v564;
        let v565: bool = (v497 && v564);
        self.scalar_v565 = v565;
        let v566: bool = (p.p44 > 0.0);
        self.scalar_v566 = v566;
        let v567: bool = (v565 && v566);
        self.scalar_v567 = v567;
        let v583: bool = (0.0 == p.p29);
        self.scalar_v583 = v583;
        let v584: bool = (v220 && v583);
        self.scalar_v584 = v584;
        let v585: bool = (p.p40 > 0.0);
        self.scalar_v585 = v585;
        let v586: bool = (v584 && v585);
        self.scalar_v586 = v586;
        let v587: bool = (!v567);
        self.scalar_v587 = v587;
        let v602: f64 = p.p28;
        self.scalar_v602 = v602;
        let v608: f64 = p.p30;
        self.scalar_v608 = v608;
        let v611: f64 = p.p31;
        self.scalar_v611 = v611;
        let v616: f64 = p.p53;
        self.scalar_v616 = v616;
        let v617: f64 = (0.5 * p.p53);
        self.scalar_v617 = v617;
        let v618: f64 = (v46 * v617);
        self.scalar_v618 = v618;
        let v619: f64 = ((v618) as f64).exp();
        self.scalar_v619 = v619;
        let v620: f64 = (-0.5 * p.p53);
        self.scalar_v620 = v620;
        let v621: f64 = (v46 * v620);
        self.scalar_v621 = v621;
        let v622: f64 = ((v621) as f64).exp();
        self.scalar_v622 = v622;
        let v623: f64 = (v619 - v622);
        self.scalar_v623 = v623;
        let v624: f64 = ((v623) as f64).ln();
        self.scalar_v624 = v624;
        let v625: f64 = (v222 * v624);
        self.scalar_v625 = v625;
        let v640: f64 = p.p54;
        self.scalar_v640 = v640;
        let v645: f64 = p.p55;
        self.scalar_v645 = v645;
        let v646: f64 = ((p.p55) as f64).abs();
        self.scalar_v646 = v646;
        let v647: bool = (p.p55 > 0.0);
        self.scalar_v647 = v647;
        let v648: bool = (true && v647);
        self.scalar_v648 = v648;
        let v656: f64 = p.p25;
        self.scalar_v656 = v656;
        let v661: f64 = p.p57;
        self.scalar_v661 = v661;
        let v662: bool = (p.p57 > 0.0);
        self.scalar_v662 = v662;
        let v663: bool = (v114 && v662);
        self.scalar_v663 = v663;
        let v664: f64 = p.p58;
        self.scalar_v664 = v664;
        let v665: f64 = (0.5 * p.p58);
        self.scalar_v665 = v665;
        let v666: f64 = (v46 * v665);
        self.scalar_v666 = v666;
        let v667: f64 = ((v666) as f64).exp();
        self.scalar_v667 = v667;
        let v668: f64 = (-0.5 * p.p58);
        self.scalar_v668 = v668;
        let v669: f64 = (v46 * v668);
        self.scalar_v669 = v669;
        let v670: f64 = ((v669) as f64).exp();
        self.scalar_v670 = v670;
        let v671: f64 = (v667 - v670);
        self.scalar_v671 = v671;
        let v672: f64 = ((v671) as f64).ln();
        self.scalar_v672 = v672;
        let v673: f64 = (v222 * v672);
        self.scalar_v673 = v673;
        let v674: f64 = (if v663 { v673 } else { v625 });
        self.scalar_v674 = v674;
        let v692: f64 = p.p59;
        self.scalar_v692 = v692;
        let v700: f64 = (if v663 { 2.4 } else { 0.0 });
        self.scalar_v700 = v700;
        let v701: bool = (false && v663);
        self.scalar_v701 = v701;
        let v705: bool = (!v662);
        self.scalar_v705 = v705;
        let v706: bool = (v114 && v705);
        self.scalar_v706 = v706;
        let v710: f64 = (if v114 { 2.4 } else { 0.0 });
        self.scalar_v710 = v710;
        let v711: bool = (v115 && v662);
        self.scalar_v711 = v711;
        let v712: f64 = (if v711 { v673 } else { v674 });
        self.scalar_v712 = v712;
        let v735: f64 = p.p60;
        self.scalar_v735 = v735;
        let v736: f64 = (-p.p60);
        self.scalar_v736 = v736;
        let v737: f64 = ((v736) as f64).abs();
        self.scalar_v737 = v737;
        let v739: bool = (v736 > 0.0);
        self.scalar_v739 = v739;
        let v740: bool = (v711 && v739);
        self.scalar_v740 = v740;
        let v744: bool = (v115 && v705);
        self.scalar_v744 = v744;
        let v748: f64 = (if v115 { p.p60 } else { v710 });
        self.scalar_v748 = v748;
        let v749: f64 = p.p99;
        self.scalar_v749 = v749;
        let v751: f64 = (v46 * p.p120);
        self.scalar_v751 = v751;
        let v756: f64 = p.p97;
        self.scalar_v756 = v756;
        let v760: f64 = p.p101;
        self.scalar_v760 = v760;
        let v761: f64 = (p.p138 - 1.0);
        self.scalar_v761 = v761;
        let v765: f64 = p.p63;
        self.scalar_v765 = v765;
        let v766: bool = (p.p63 > 0.0);
        self.scalar_v766 = v766;
        let v767: f64 = p.p62;
        self.scalar_v767 = v767;
        let v768: bool = (p.p62 > 0.0);
        self.scalar_v768 = v768;
        let v769: bool = (v766 && v768);
        self.scalar_v769 = v769;
        let v770: f64 = (0.5 * p.p63);
        self.scalar_v770 = v770;
        let v771: f64 = (v46 * v770);
        self.scalar_v771 = v771;
        let v772: f64 = ((v771) as f64).exp();
        self.scalar_v772 = v772;
        let v773: f64 = (-0.5 * p.p63);
        self.scalar_v773 = v773;
        let v774: f64 = (v46 * v773);
        self.scalar_v774 = v774;
        let v775: f64 = ((v774) as f64).exp();
        self.scalar_v775 = v775;
        let v776: f64 = (v772 - v775);
        self.scalar_v776 = v776;
        let v777: f64 = ((v776) as f64).ln();
        self.scalar_v777 = v777;
        let v778: f64 = (v222 * v777);
        self.scalar_v778 = v778;
        let v779: f64 = (if v769 { v778 } else { v712 });
        self.scalar_v779 = v779;
        let v796: f64 = p.p64;
        self.scalar_v796 = v796;
        let v803: f64 = (-v748);
        self.scalar_v803 = v803;
        let v804: f64 = ((v803) as f64).abs();
        self.scalar_v804 = v804;
        let v805: f64 = (if v769 { v804 } else { 0.0 });
        self.scalar_v805 = v805;
        let v806: bool = (v803 > 0.0);
        self.scalar_v806 = v806;
        let v807: bool = (v769 && v806);
        self.scalar_v807 = v807;
        let v811: bool = (!v768);
        self.scalar_v811 = v811;
        let v812: bool = (v766 && v811);
        self.scalar_v812 = v812;
        let v816: bool = (!v766);
        self.scalar_v816 = v816;
        let v820: f64 = p.p96;
        self.scalar_v820 = v820;
        let v821: f64 = p.p136;
        self.scalar_v821 = v821;
        let v825: f64 = p.p90;
        self.scalar_v825 = v825;
        let v826: f64 = p.p135;
        self.scalar_v826 = v826;
        let v830: f64 = p.p95;
        self.scalar_v830 = v830;
        let v831: f64 = p.p137;
        self.scalar_v831 = v831;
        let v835: f64 = p.p142;
        self.scalar_v835 = v835;
        let v836: f64 = p.p143;
        self.scalar_v836 = v836;
        let v840: f64 = p.p144;
        self.scalar_v840 = v840;
        let v844: f64 = p.p141;
        self.scalar_v844 = v844;
        let v845: bool = (0.0 != p.p141);
        self.scalar_v845 = v845;
        let v846: f64 = p.p149;
        self.scalar_v846 = v846;
        let v847: bool = (p.p142 >= p.p149);
        self.scalar_v847 = v847;
        let v848: bool = (v845 && v847);
        self.scalar_v848 = v848;
        let v849: bool = (p.p142 > 0.0);
        self.scalar_v849 = v849;
        let v850: bool = (v848 && v849);
        self.scalar_v850 = v850;
        let v895: bool = (v220 && v850);
        self.scalar_v895 = v895;
        let v896: f64 = (if v895 { v233 } else { v779 });
        self.scalar_v896 = v896;
        let v925: bool = (v267 && v895);
        self.scalar_v925 = v925;
        let v929: bool = (v272 && v850);
        self.scalar_v929 = v929;
        let v947: bool = (v121 && v850);
        self.scalar_v947 = v947;
        let v948: f64 = (if v947 { v304 } else { v896 });
        self.scalar_v948 = v948;
        let v973: bool = (v333 && v947);
        self.scalar_v973 = v973;
        let v977: bool = (v338 && v850);
        self.scalar_v977 = v977;
        let v981: bool = (v114 && v850);
        self.scalar_v981 = v981;
        let v1006: bool = (v376 && v850);
        self.scalar_v1006 = v1006;
        let v1015: bool = (v388 && v850);
        self.scalar_v1015 = v1015;
        let v1040: bool = (v419 && v850);
        self.scalar_v1040 = v1040;
        let v1046: bool = (v427 && v850);
        self.scalar_v1046 = v1046;
        let v1063: bool = (v448 && v850);
        self.scalar_v1063 = v1063;
        let v1072: bool = (v460 && v850);
        self.scalar_v1072 = v1072;
        let v1101: bool = (v497 && v850);
        self.scalar_v1101 = v1101;
        let v1102: f64 = (if v1101 { v507 } else { v948 });
        self.scalar_v1102 = v1102;
        let v1126: bool = (v535 && v1101);
        self.scalar_v1126 = v1126;
        let v1130: bool = (v540 && v850);
        self.scalar_v1130 = v1130;
        let v1186: bool = (true && v850);
        self.scalar_v1186 = v1186;
        let v1187: f64 = (if v1186 { v625 } else { v1102 });
        self.scalar_v1187 = v1187;
        let v1210: bool = (v647 && v1186);
        self.scalar_v1210 = v1210;
        let v1214: bool = (false && v850);
        self.scalar_v1214 = v1214;
        let v1228: bool = (v662 && v981);
        self.scalar_v1228 = v1228;
        let v1229: f64 = (if v1228 { v673 } else { v1187 });
        self.scalar_v1229 = v1229;
        let v1254: bool = (false && v1228);
        self.scalar_v1254 = v1254;
        let v1258: bool = (v705 && v981);
        self.scalar_v1258 = v1258;
        let v1262: f64 = (if v981 { 2.4 } else { v748 });
        self.scalar_v1262 = v1262;
        let v1263: bool = (v115 && v850);
        self.scalar_v1263 = v1263;
        let v1264: bool = (v662 && v1263);
        self.scalar_v1264 = v1264;
        let v1265: f64 = (if v1264 { v673 } else { v1229 });
        self.scalar_v1265 = v1265;
        let v1289: bool = (v739 && v1264);
        self.scalar_v1289 = v1289;
        let v1293: bool = (v705 && v1263);
        self.scalar_v1293 = v1293;
        let v1297: f64 = (if v1263 { p.p60 } else { v1262 });
        self.scalar_v1297 = v1297;
        let v1312: bool = (v766 && v850);
        self.scalar_v1312 = v1312;
        let v1313: bool = (v768 && v1312);
        self.scalar_v1313 = v1313;
        let v1314: f64 = (if v1313 { v778 } else { v1265 });
        self.scalar_v1314 = v1314;
        let v1337: f64 = (-v1297);
        self.scalar_v1337 = v1337;
        let v1338: f64 = ((v1337) as f64).abs();
        self.scalar_v1338 = v1338;
        let v1340: bool = (v1337 > 0.0);
        self.scalar_v1340 = v1340;
        let v1341: bool = (v1313 && v1340);
        self.scalar_v1341 = v1341;
        let v1345: bool = (v811 && v1312);
        self.scalar_v1345 = v1345;
        let v1349: bool = (v816 && v850);
        self.scalar_v1349 = v1349;
        let v1372: bool = (p.p14 > 0.0);
        self.scalar_v1372 = v1372;
        let v1373: f64 = p.p15;
        self.scalar_v1373 = v1373;
        let v1392: bool = (!v1372);
        self.scalar_v1392 = v1392;
        let v1394: bool = (p.p16 > 0.0);
        self.scalar_v1394 = v1394;
        let v1412: bool = (!v1394);
        self.scalar_v1412 = v1412;
        let v1415: f64 = p.p13;
        self.scalar_v1415 = v1415;
        let v1450: f64 = (-p.p41);
        self.scalar_v1450 = v1450;
        let v1460: f64 = (1.0 - p.p41);
        self.scalar_v1460 = v1460;
        let v1475: f64 = p.p51;
        self.scalar_v1475 = v1475;
        let v1477: bool = (p.p51 < 100.0);
        self.scalar_v1477 = v1477;
        let v1480: f64 = (p.p49 / 4.0);
        self.scalar_v1480 = v1480;
        let v1557: f64 = (1.0 - p.p49);
        self.scalar_v1557 = v1557;
        let v1561: f64 = (-p.p49);
        self.scalar_v1561 = v1561;
        let v1609: bool = (!v1477);
        self.scalar_v1609 = v1609;
        let v1654: bool = (p.p10 > 0.0);
        self.scalar_v1654 = v1654;
        let v1655: f64 = p.p11;
        self.scalar_v1655 = v1655;
        let v1692: bool = (!v1654);
        self.scalar_v1692 = v1692;
        let v1696: f64 = p.p12;
        self.scalar_v1696 = v1696;
        let v1711: f64 = (-0.8754687373538999 / p.p49);
        self.scalar_v1711 = v1711;
        let v1712: f64 = ((v1711) as f64).exp();
        self.scalar_v1712 = v1712;
        let v1713: f64 = (1.0 - v1712);
        self.scalar_v1713 = v1713;
        let v1734: f64 = p.p67;
        self.scalar_v1734 = v1734;
        let v1739: f64 = p.p68;
        self.scalar_v1739 = v1739;
        let v1761: f64 = p.p80;
        self.scalar_v1761 = v1761;
        let v1770: f64 = p.p77;
        self.scalar_v1770 = v1770;
        let v1780: f64 = p.p76;
        self.scalar_v1780 = v1780;
        let v1783: f64 = p.p81;
        self.scalar_v1783 = v1783;
        let v1791: f64 = p.p85;
        self.scalar_v1791 = v1791;
        let v1792: bool = (p.p85 > 0.0);
        self.scalar_v1792 = v1792;
        let v1816: bool = (p.p0 >= 310.0);
        self.scalar_v1816 = v1816;
        let v1820: bool = (!v1816);
        self.scalar_v1820 = v1820;
        let v1829: bool = (p.p0 >= 320.0);
        self.scalar_v1829 = v1829;
        let v1833: f64 = p.p70;
        self.scalar_v1833 = v1833;
        let v1840: f64 = (1.0 + p.p70);
        self.scalar_v1840 = v1840;
        let v1843: f64 = p.p83;
        self.scalar_v1843 = v1843;
        let v1844: f64 = (p.p75 / p.p74);
        self.scalar_v1844 = v1844;
        let v1845: f64 = (0.05 * v1844);
        self.scalar_v1845 = v1845;
        let v1846: bool = (p.p83 < v1845);
        self.scalar_v1846 = v1846;
        let v1847: bool = (!v1846);
        self.scalar_v1847 = v1847;
        let v1857: f64 = p.p84;
        self.scalar_v1857 = v1857;
        let v1861: f64 = p.p82;
        self.scalar_v1861 = v1861;
        let v1873: f64 = p.p73;
        self.scalar_v1873 = v1873;
        let v1874: f64 = (1.0 - p.p73);
        self.scalar_v1874 = v1874;
        let v1891: f64 = p.p72;
        self.scalar_v1891 = v1891;
        let v1895: f64 = (1.0 + p.p72);
        self.scalar_v1895 = v1895;
        let v1896: f64 = ((v1895) as f64).sqrt();
        self.scalar_v1896 = v1896;
        let v1897: f64 = (1.0 + v1896);
        self.scalar_v1897 = v1897;
        let v1916: bool = (p.p115 < 0.01);
        self.scalar_v1916 = v1916;
        let v1917: bool = (p.p116 < 0.01);
        self.scalar_v1917 = v1917;
        let v1918: bool = (v1916 && v1917);
        self.scalar_v1918 = v1918;
        let v1942: f64 = ((v181) as f64).abs();
        self.scalar_v1942 = v1942;
        let v1943: bool = (v1942 > 0.001);
        self.scalar_v1943 = v1943;
        let v1948: bool = (v186 < 0.01);
        self.scalar_v1948 = v1948;
        let v1958: f64 = (v187 * 0.25);
        self.scalar_v1958 = v1958;
        let v1969: f64 = (-v188);
        self.scalar_v1969 = v1969;
        let v1978: bool = (!v1948);
        self.scalar_v1978 = v1978;
        let v1991: f64 = (v183 * v184);
        self.scalar_v1991 = v1991;
        let v2011: f64 = (v182 * v185);
        self.scalar_v2011 = v2011;
        let v2029: f64 = (v181 * -2.0);
        self.scalar_v2029 = v2029;
        let v2040: bool = (!v1943);
        self.scalar_v2040 = v2040;
        let v2051: f64 = (v184 * 2.0);
        self.scalar_v2051 = v2051;
        let v2099: f64 = p.p5;
        self.scalar_v2099 = v2099;
        let v2144: f64 = p.p7;
        self.scalar_v2144 = v2144;
        let v2504: f64 = (p.p85 * p.p7);
        self.scalar_v2504 = v2504;
        let v2855: f64 = p.p93;
        self.scalar_v2855 = v2855;
        let v2862: bool = (p.p23 > 0.0);
        self.scalar_v2862 = v2862;
        let v2863: f64 = p.p24;
        self.scalar_v2863 = v2863;
        let v2881: bool = (!v2862);
        self.scalar_v2881 = v2881;
        let v2886: f64 = (1.0 / p.p49);
        self.scalar_v2886 = v2886;
        let v2887: f64 = (v2886 - 1.0);
        self.scalar_v2887 = v2887;
        let v2910: f64 = p.p35;
        self.scalar_v2910 = v2910;
        let v2911: bool = (p.p35 > 0.0);
        self.scalar_v2911 = v2911;
        let v2918: f64 = p.p36;
        self.scalar_v2918 = v2918;
        let v2935: bool = (!v2911);
        self.scalar_v2935 = v2935;
        let v2965: f64 = p.p34;
        self.scalar_v2965 = v2965;
        let v2966: bool = (p.p34 > 0.0);
        self.scalar_v2966 = v2966;
        let v2982: bool = (!v2966);
        self.scalar_v2982 = v2982;
        let v2990: f64 = p.p92;
        self.scalar_v2990 = v2990;
        let v2991: f64 = (1.0 + p.p92);
        self.scalar_v2991 = v2991;
        let v3011: f64 = p.p91;
        self.scalar_v3011 = v3011;
        let v3030: f64 = p.p94;
        self.scalar_v3030 = v3030;
        let v3039: bool = (p.p18 > 0.0);
        self.scalar_v3039 = v3039;
        let v3040: f64 = p.p19;
        self.scalar_v3040 = v3040;
        let v3058: bool = (!v3039);
        self.scalar_v3058 = v3058;
        let v3060: bool = (p.p20 > 0.0);
        self.scalar_v3060 = v3060;
        let v3078: bool = (!v3060);
        self.scalar_v3078 = v3078;
        let v3107: f64 = (-p.p45);
        self.scalar_v3107 = v3107;
        let v3117: f64 = (1.0 - p.p45);
        self.scalar_v3117 = v3117;
        let v3136: f64 = (1.0 / p.p45);
        self.scalar_v3136 = v3136;
        let v3137: f64 = (1.0 - v3136);
        self.scalar_v3137 = v3137;
        let v3159: f64 = (1.0 / p.p41);
        self.scalar_v3159 = v3159;
        let v3160: f64 = (1.0 - v3159);
        self.scalar_v3160 = v3160;
        let v3183: f64 = p.p56;
        self.scalar_v3183 = v3183;
        let v3184: bool = (p.p56 < 100.0);
        self.scalar_v3184 = v3184;
        let v3187: f64 = (p.p54 / 4.0);
        self.scalar_v3187 = v3187;
        let v3256: f64 = (1.0 - p.p54);
        self.scalar_v3256 = v3256;
        let v3287: bool = (!v3184);
        self.scalar_v3287 = v3287;
        let v3320: bool = (p.p25 > 0.0);
        self.scalar_v3320 = v3320;
        let v3321: f64 = p.p26;
        self.scalar_v3321 = v3321;
        let v3339: bool = (!v3320);
        self.scalar_v3339 = v3339;
        let v3464: f64 = p.p61;
        self.scalar_v3464 = v3464;
        let v3465: bool = (p.p61 < 100.0);
        self.scalar_v3465 = v3465;
        let v3468: f64 = (p.p59 / 4.0);
        self.scalar_v3468 = v3468;
        let v3537: f64 = (1.0 - p.p59);
        self.scalar_v3537 = v3537;
        let v3568: bool = (!v3465);
        self.scalar_v3568 = v3568;
        let v3601: f64 = p.p65;
        self.scalar_v3601 = v3601;
        let v3602: bool = (p.p65 < 100.0);
        self.scalar_v3602 = v3602;
        let v3604: bool = (v766 && v3602);
        self.scalar_v3604 = v3604;
        let v3606: f64 = (p.p64 / 4.0);
        self.scalar_v3606 = v3606;
        let v3675: f64 = (1.0 - p.p64);
        self.scalar_v3675 = v3675;
        let v3706: bool = (!v3602);
        self.scalar_v3706 = v3706;
        let v3707: bool = (v766 && v3706);
        self.scalar_v3707 = v3707;
        let v3742: bool = (p.p97 > 0.0);
        self.scalar_v3742 = v3742;
        let v3743: f64 = p.p98;
        self.scalar_v3743 = v3743;
        let v3755: bool = (p.p101 > 0.0);
        self.scalar_v3755 = v3755;
        let v3756: bool = (v3742 && v3755);
        self.scalar_v3756 = v3756;
        let v3760: bool = (!v3755);
        self.scalar_v3760 = v3760;
        let v3761: bool = (v3742 && v3760);
        self.scalar_v3761 = v3761;
        let v3763: bool = (!v3742);
        self.scalar_v3763 = v3763;
        let v3766: bool = (p.p99 > 0.0);
        self.scalar_v3766 = v3766;
        let v3767: f64 = p.p100;
        self.scalar_v3767 = v3767;
        let v3785: bool = (!v3766);
        self.scalar_v3785 = v3785;
        let v3787: bool = (v847 && v849);
        self.scalar_v3787 = v3787;
        let v3788: bool = (1.0 == p.p141);
        self.scalar_v3788 = v3788;
        let v3789: bool = (v3787 && v3788);
        self.scalar_v3789 = v3789;
        let v3794: bool = (2.0 == p.p141);
        self.scalar_v3794 = v3794;
        let v3795: bool = (!v3788);
        self.scalar_v3795 = v3795;
        let v3796: bool = (v3787 && v3795);
        self.scalar_v3796 = v3796;
        let v3797: bool = (v3794 && v3796);
        self.scalar_v3797 = v3797;
        let v3845: bool = (!v3794);
        self.scalar_v3845 = v3845;
        let v3846: bool = (v3796 && v3845);
        self.scalar_v3846 = v3846;
        let v3848: bool = (0.0 != v137);
        self.scalar_v3848 = v3848;
        let v3880: bool = (!v3848);
        self.scalar_v3880 = v3880;
        let v3887: bool = (p.p89 >= p.p149);
        self.scalar_v3887 = v3887;
        let v3888: bool = (p.p89 > 0.0);
        self.scalar_v3888 = v3888;
        let v3889: bool = (v3887 && v3888);
        self.scalar_v3889 = v3889;
        let v3890: bool = (p.p93 > 0.0);
        self.scalar_v3890 = v3890;
        let v3891: bool = (p.p90 >= p.p149);
        self.scalar_v3891 = v3891;
        let v3892: bool = (p.p90 > 0.0);
        self.scalar_v3892 = v3892;
        let v3893: bool = (v3891 && v3892);
        self.scalar_v3893 = v3893;
        let v3894: bool = (p.p95 >= p.p149);
        self.scalar_v3894 = v3894;
        let v3895: bool = (p.p95 > 0.0);
        self.scalar_v3895 = v3895;
        let v3896: bool = (v3894 && v3895);
        self.scalar_v3896 = v3896;
        let v3897: bool = (p.p96 >= p.p149);
        self.scalar_v3897 = v3897;
        let v3898: bool = (p.p96 > 0.0);
        self.scalar_v3898 = v3898;
        let v3899: bool = (v3897 && v3898);
        self.scalar_v3899 = v3899;
        let v3900: f64 = p.p102;
        self.scalar_v3900 = v3900;
        let v3901: bool = (p.p102 >= p.p149);
        self.scalar_v3901 = v3901;
        let v3902: bool = (p.p102 > 0.0);
        self.scalar_v3902 = v3902;
        let v3903: bool = (v3901 && v3902);
        self.scalar_v3903 = v3903;
        let v3904: f64 = p.p103;
        self.scalar_v3904 = v3904;
        let v3905: bool = (p.p103 > 0.0);
        self.scalar_v3905 = v3905;
        let v3906: bool = (p.p141 >= 1.0);
        self.scalar_v3906 = v3906;
        let v3907: bool = (v847 && v3906);
        self.scalar_v3907 = v3907;
        let v3908: bool = (v849 && v3907);
        self.scalar_v3908 = v3908;
        let v3909: f64 = p.p145;
        self.scalar_v3909 = v3909;
        let v3910: bool = (p.p145 > 0.0);
        self.scalar_v3910 = v3910;
        let v3914: f64 = p.p109;
        self.scalar_v3914 = v3914;
        let v3915: bool = (1.0 == p.p109);
        self.scalar_v3915 = v3915;
        let v3916: bool = (p.p88 > 0.0);
        self.scalar_v3916 = v3916;
        let v3917: bool = (p.p87 > 0.0);
        self.scalar_v3917 = v3917;
        let v3918: bool = (v3916 && v3917);
        self.scalar_v3918 = v3918;
        let v3919: bool = (v3915 && v3918);
        self.scalar_v3919 = v3919;
        let v3926: f64 = (if v3919 { 1.0 } else { 0.0 });
        self.scalar_v3926 = v3926;
        let v3929: f64 = (p.p87 * 2.0);
        self.scalar_v3929 = v3929;
        let v3930: f64 = (p.p88 * p.p88);
        self.scalar_v3930 = v3930;
        let v3931: f64 = (v3929 - v3930);
        self.scalar_v3931 = v3931;
        let v3944: f64 = (-p.p148);
        self.scalar_v3944 = v3944;
        let v3963: bool = (v3889 && v3890);
        self.scalar_v3963 = v3963;
        let v3968: bool = (!v564);
        self.scalar_v3968 = v3968;
        let v3987: f64 = p.p108;
        self.scalar_v3987 = v3987;
        let v3990: bool = (v1829 && v3766);
        self.scalar_v3990 = v3990;
        let v3995: bool = (!v1829);
        self.scalar_v3995 = v3995;
        let v3997: bool = (v1816 && v3995);
        self.scalar_v3997 = v3997;
        let v4004: bool = (v3903 && v3905);
        self.scalar_v4004 = v4004;
        let v4011: bool = (v3908 && v3910);
        self.scalar_v4011 = v4011;
        let v4033: bool = (!v3919);
        self.scalar_v4033 = v4033;
        let v4036: f64 = (p.p148 - p.p148);
        self.scalar_v4036 = v4036;
        let v4037: f64 = (if v850 { 1.0 } else { 0.0 });
        self.scalar_v4037 = v4037;
        let v5647: f64 = (if v419 { p.p148 } else { 0.0 });
        self.scalar_v5647 = v5647;
        let v5648: f64 = (if v419 { v3944 } else { 0.0 });
        self.scalar_v5648 = v5648;
        let v5651: f64 = (if v427 { p.p148 } else { v5647 });
        self.scalar_v5651 = v5651;
        let v5652: f64 = (if v427 { v3944 } else { 0.0 });
        self.scalar_v5652 = v5652;
        let v5653: f64 = (if v427 { v4036 } else { v5648 });
        self.scalar_v5653 = v5653;
        let v5698: f64 = (v5651 / v43);
        self.scalar_v5698 = v5698;
        let v5699: f64 = (v5652 / v43);
        self.scalar_v5699 = v5699;
        let v5700: f64 = (v5653 / v43);
        self.scalar_v5700 = v5700;
        let v23551: f64 = (if v448 { p.p148 } else { 0.0 });
        self.scalar_v23551 = v23551;
        let v23552: f64 = (if v448 { v3944 } else { 0.0 });
        self.scalar_v23552 = v23552;
        let v26252: f64 = (v3944 / p.p31);
        self.scalar_v26252 = v26252;
        let v26253: f64 = (p.p148 / p.p31);
        self.scalar_v26253 = v26253;
        let v28544: f64 = (p.p62 * v3944);
        self.scalar_v28544 = v28544;
        let v28545: f64 = (p.p148 * p.p62);
        self.scalar_v28545 = v28545;
        let v29048: f64 = (if v3848 { 1.0 } else { 0.0 });
        self.scalar_v29048 = v29048;
        let v29197: f64 = (-v29048);
        self.scalar_v29197 = v29197;
        let v29305: f64 = (p.p88 * v29048);
        self.scalar_v29305 = v29305;
        let v29306: f64 = (p.p66 * v29305);
        self.scalar_v29306 = v29306;
        let v29307: f64 = (if v3848 { v29306 } else { 0.0 });
        self.scalar_v29307 = v29307;
        let v29308: f64 = (v29305 / 3.0);
        self.scalar_v29308 = v29308;
        let v29309: f64 = (p.p66 * v29308);
        self.scalar_v29309 = v29309;
        let v29310: f64 = (if v3848 { v29309 } else { 0.0 });
        self.scalar_v29310 = v29310;
        let v29521: f64 = (p.p87 * v29048);
        self.scalar_v29521 = v29521;
        let v29522: f64 = (p.p66 * v29521);
        self.scalar_v29522 = v29522;
        let v29523: f64 = (if v3848 { v29522 } else { 0.0 });
        self.scalar_v29523 = v29523;
        let v29587: f64 = (if v3880 { 0.0 } else { v29307 });
        self.scalar_v29587 = v29587;
        let v29588: f64 = (if v3880 { 0.0 } else { v29310 });
        self.scalar_v29588 = v29588;
        let v29610: f64 = (if v3880 { 0.0 } else { v29523 });
        self.scalar_v29610 = v29610;
        let v30261: f64 = (-v101);
        self.scalar_v30261 = v30261;
        let v30268: f64 = (-v99);
        self.scalar_v30268 = v30268;
        let v30285: f64 = (-v106);
        self.scalar_v30285 = v30285;
        let v30286: f64 = (-v107);
        self.scalar_v30286 = v30286;
        let v30287: f64 = (-p.p108);
        self.scalar_v30287 = v30287;
        let v30304: f64 = (if v3990 { -0.0 } else { 0.0 });
        self.scalar_v30304 = v30304;
        let v30311: f64 = (if v3997 { -0.0 } else { 0.0 });
        self.scalar_v30311 = v30311;
        let v30328: f64 = (-1.0 / p.p102);
        self.scalar_v30328 = v30328;
        let v30329: f64 = (1.0 / p.p102);
        self.scalar_v30329 = v30329;
        let v30330: f64 = (if v3903 { v30328 } else { 0.0 });
        self.scalar_v30330 = v30330;
        let v30331: f64 = (if v3903 { v30329 } else { 0.0 });
        self.scalar_v30331 = v30331;
        let v30332: f64 = (-p.p103);
        self.scalar_v30332 = v30332;
        let v30385: f64 = (if v3919 { -1.0 } else { 0.0 });
        self.scalar_v30385 = v30385;
        let v30517: f64 = (if v4033 { 1.0 } else { 0.0 });
        self.scalar_v30517 = v30517;
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
        let v190: f64 = (temperature + self.scalar_v189);
        self.scalar_v190 = v190;
        let v192: bool = (self.scalar_v190 < 73.14999999999998);
        self.scalar_v192 = v192;
        let v193: f64 = (if self.scalar_v192 { 73.14999999999998 } else { self.scalar_v190 });
        self.scalar_v193 = v193;
        let v195: bool = (self.scalar_v193 > 600.0);
        self.scalar_v195 = v195;
        let v196: bool = (!self.scalar_v192);
        self.scalar_v196 = v196;
        let v197: bool = (self.scalar_v195 && self.scalar_v196);
        self.scalar_v197 = v197;
        let v198: f64 = (if self.scalar_v197 { 600.0 } else { self.scalar_v193 });
        self.scalar_v198 = v198;
        let v199: f64 = (self.scalar_v41 * self.scalar_v198);
        self.scalar_v199 = v199;
        let v200: f64 = (1.0 / self.scalar_v199);
        self.scalar_v200 = v200;
        let v201: f64 = (self.scalar_v198 - self.scalar_v39);
        self.scalar_v201 = v201;
        let v202: f64 = (self.scalar_v39 / self.scalar_v198);
        self.scalar_v202 = v202;
        let v203: f64 = (self.scalar_v198 / self.scalar_v39);
        self.scalar_v203 = v203;
        let v204: f64 = ((self.scalar_v203) as f64).ln();
        self.scalar_v204 = v204;
        let v205: f64 = (self.scalar_v47 * self.scalar_v198);
        self.scalar_v205 = v205;
        let v206: f64 = ((self.scalar_v198) as f64).ln();
        self.scalar_v206 = v206;
        let v207: f64 = (self.scalar_v205 * self.scalar_v206);
        self.scalar_v207 = v207;
        let v208: f64 = (self.scalar_v51 * self.scalar_v198);
        self.scalar_v208 = v208;
        let v209: f64 = (self.scalar_v55 + self.scalar_v207);
        self.scalar_v209 = v209;
        let v210: f64 = (self.scalar_v208 + self.scalar_v209);
        self.scalar_v210 = v210;
        let v211: f64 = (self.scalar_v58 + self.scalar_v207);
        self.scalar_v211 = v211;
        let v212: f64 = (self.scalar_v208 + self.scalar_v211);
        self.scalar_v212 = v212;
        let v213: f64 = (self.scalar_v61 + self.scalar_v207);
        self.scalar_v213 = v213;
        let v214: f64 = (self.scalar_v208 + self.scalar_v213);
        self.scalar_v214 = v214;
        let v215: f64 = (self.scalar_v210 + self.scalar_v212);
        self.scalar_v215 = v215;
        let v216: f64 = (0.5 * self.scalar_v215);
        self.scalar_v216 = v216;
        let v217: f64 = (self.scalar_v210 + self.scalar_v214);
        self.scalar_v217 = v217;
        let v218: f64 = (0.5 * self.scalar_v217);
        self.scalar_v218 = v218;
        let v235: f64 = (self.scalar_v203 * self.scalar_v234);
        self.scalar_v235 = v235;
        let v236: f64 = (1.0 - self.scalar_v203);
        self.scalar_v236 = v236;
        let v237: f64 = (self.scalar_v70 * self.scalar_v236);
        self.scalar_v237 = v237;
        let v238: f64 = (self.scalar_v235 + self.scalar_v237);
        self.scalar_v238 = v238;
        let v239: f64 = (self.scalar_v78 * self.scalar_v199);
        self.scalar_v239 = v239;
        let v240: f64 = (self.scalar_v204 * self.scalar_v239);
        self.scalar_v240 = v240;
        let v241: f64 = (self.scalar_v238 - self.scalar_v240);
        self.scalar_v241 = v241;
        let v242: f64 = (if self.scalar_v220 { self.scalar_v241 } else { 0.0 });
        self.scalar_v242 = v242;
        let v243: f64 = (self.scalar_v199 * 2.0);
        self.scalar_v243 = v243;
        let v245: f64 = (-self.scalar_v242);
        self.scalar_v245 = v245;
        let v246: f64 = (self.scalar_v200 * self.scalar_v245);
        self.scalar_v246 = v246;
        let v247: f64 = ((self.scalar_v246) as f64).exp();
        self.scalar_v247 = v247;
        let v248: f64 = (4.0 * self.scalar_v247);
        self.scalar_v248 = v248;
        let v249: f64 = (1.0 + self.scalar_v248);
        self.scalar_v249 = v249;
        let v250: f64 = ((self.scalar_v249) as f64).sqrt();
        self.scalar_v250 = v250;
        let v251: f64 = (1.0 + self.scalar_v250);
        self.scalar_v251 = v251;
        let v252: f64 = (0.5 * self.scalar_v251);
        self.scalar_v252 = v252;
        let v253: f64 = ((self.scalar_v252) as f64).ln();
        self.scalar_v253 = v253;
        let v254: f64 = (self.scalar_v243 * self.scalar_v253);
        self.scalar_v254 = v254;
        let v255: f64 = (self.scalar_v242 + self.scalar_v254);
        self.scalar_v255 = v255;
        let v256: f64 = (if self.scalar_v220 { self.scalar_v255 } else { 0.0 });
        self.scalar_v256 = v256;
        let v258: f64 = (self.scalar_v223 / self.scalar_v256);
        self.scalar_v258 = v258;
        let v259: f64 = ((self.scalar_v258) as f64).ln();
        self.scalar_v259 = v259;
        let v260: f64 = (self.scalar_v257 * self.scalar_v259);
        self.scalar_v260 = v260;
        let v261: f64 = ((self.scalar_v260) as f64).exp();
        self.scalar_v261 = v261;
        let v262: f64 = (self.scalar_v219 * self.scalar_v261);
        self.scalar_v262 = v262;
        let v263: f64 = (if self.scalar_v220 { self.scalar_v262 } else { 0.0 });
        self.scalar_v263 = v263;
        let v269: f64 = (self.scalar_v256 * self.scalar_v264);
        self.scalar_v269 = v269;
        let v270: f64 = (self.scalar_v269 / self.scalar_v223);
        self.scalar_v270 = v270;
        let v271: f64 = (if self.scalar_v268 { self.scalar_v270 } else { self.scalar_v266 });
        self.scalar_v271 = v271;
        let v273: f64 = (if self.scalar_v272 { self.scalar_v219 } else { self.scalar_v263 });
        self.scalar_v273 = v273;
        let v274: f64 = (if self.scalar_v272 { self.scalar_v223 } else { self.scalar_v256 });
        self.scalar_v274 = v274;
        let v275: f64 = (if self.scalar_v272 { self.scalar_v264 } else { self.scalar_v271 });
        self.scalar_v275 = v275;
        let v278: f64 = (self.scalar_v204 * self.scalar_v277);
        self.scalar_v278 = v278;
        let v280: f64 = (1.0 - self.scalar_v202);
        self.scalar_v280 = v280;
        let v281: f64 = (self.scalar_v279 * self.scalar_v280);
        self.scalar_v281 = v281;
        let v282: f64 = (self.scalar_v278 + self.scalar_v281);
        self.scalar_v282 = v282;
        let v283: f64 = ((self.scalar_v282) as f64).exp();
        self.scalar_v283 = v283;
        let v284: f64 = (self.scalar_v276 * self.scalar_v283);
        self.scalar_v284 = v284;
        let v288: f64 = (self.scalar_v204 * self.scalar_v287);
        self.scalar_v288 = v288;
        let v290: f64 = (self.scalar_v280 * self.scalar_v289);
        self.scalar_v290 = v290;
        let v291: f64 = (self.scalar_v290 / self.scalar_v286);
        self.scalar_v291 = v291;
        let v292: f64 = (self.scalar_v288 + self.scalar_v291);
        self.scalar_v292 = v292;
        let v293: f64 = ((self.scalar_v292) as f64).exp();
        self.scalar_v293 = v293;
        let v294: f64 = (self.scalar_v285 * self.scalar_v293);
        self.scalar_v294 = v294;
        let v306: f64 = (self.scalar_v203 * self.scalar_v305);
        self.scalar_v306 = v306;
        let v307: f64 = (self.scalar_v72 * self.scalar_v236);
        self.scalar_v307 = v307;
        let v308: f64 = (self.scalar_v306 + self.scalar_v307);
        self.scalar_v308 = v308;
        let v309: f64 = (self.scalar_v308 - self.scalar_v240);
        self.scalar_v309 = v309;
        let v310: f64 = (if self.scalar_v121 { self.scalar_v309 } else { self.scalar_v242 });
        self.scalar_v310 = v310;
        let v311: f64 = (-self.scalar_v310);
        self.scalar_v311 = v311;
        let v312: f64 = (self.scalar_v200 * self.scalar_v311);
        self.scalar_v312 = v312;
        let v313: f64 = ((self.scalar_v312) as f64).exp();
        self.scalar_v313 = v313;
        let v314: f64 = (4.0 * self.scalar_v313);
        self.scalar_v314 = v314;
        let v315: f64 = (1.0 + self.scalar_v314);
        self.scalar_v315 = v315;
        let v316: f64 = ((self.scalar_v315) as f64).sqrt();
        self.scalar_v316 = v316;
        let v317: f64 = (1.0 + self.scalar_v316);
        self.scalar_v317 = v317;
        let v318: f64 = (0.5 * self.scalar_v317);
        self.scalar_v318 = v318;
        let v319: f64 = ((self.scalar_v318) as f64).ln();
        self.scalar_v319 = v319;
        let v320: f64 = (self.scalar_v243 * self.scalar_v319);
        self.scalar_v320 = v320;
        let v321: f64 = (self.scalar_v310 + self.scalar_v320);
        self.scalar_v321 = v321;
        let v322: f64 = (if self.scalar_v121 { self.scalar_v321 } else { 0.0 });
        self.scalar_v322 = v322;
        let v324: f64 = (self.scalar_v295 / self.scalar_v322);
        self.scalar_v324 = v324;
        let v325: f64 = ((self.scalar_v324) as f64).ln();
        self.scalar_v325 = v325;
        let v326: f64 = (self.scalar_v323 * self.scalar_v325);
        self.scalar_v326 = v326;
        let v327: f64 = ((self.scalar_v326) as f64).exp();
        self.scalar_v327 = v327;
        let v328: f64 = (self.scalar_v120 * self.scalar_v327);
        self.scalar_v328 = v328;
        let v329: f64 = (if self.scalar_v121 { self.scalar_v328 } else { 0.0 });
        self.scalar_v329 = v329;
        let v335: f64 = (self.scalar_v322 * self.scalar_v330);
        self.scalar_v335 = v335;
        let v336: f64 = (self.scalar_v335 / self.scalar_v295);
        self.scalar_v336 = v336;
        let v337: f64 = (if self.scalar_v334 { self.scalar_v336 } else { self.scalar_v332 });
        self.scalar_v337 = v337;
        let v339: f64 = (if self.scalar_v338 { self.scalar_v120 } else { self.scalar_v329 });
        self.scalar_v339 = v339;
        let v340: f64 = (if self.scalar_v338 { self.scalar_v295 } else { self.scalar_v322 });
        self.scalar_v340 = v340;
        let v341: f64 = (if self.scalar_v338 { self.scalar_v330 } else { self.scalar_v337 });
        self.scalar_v341 = v341;
        let v343: f64 = (if self.scalar_v114 { 2.4 } else { self.scalar_v341 });
        self.scalar_v343 = v343;
        let v345: f64 = (self.scalar_v81 * self.scalar_v204);
        self.scalar_v345 = v345;
        let v347: f64 = (self.scalar_v280 * self.scalar_v346);
        self.scalar_v347 = v347;
        let v348: f64 = (self.scalar_v345 + self.scalar_v347);
        self.scalar_v348 = v348;
        let v349: f64 = ((self.scalar_v348) as f64).exp();
        self.scalar_v349 = v349;
        let v350: f64 = (self.scalar_v344 * self.scalar_v349);
        self.scalar_v350 = v350;
        let v352: f64 = (self.scalar_v274 / self.scalar_v223);
        self.scalar_v352 = v352;
        let v353: f64 = ((self.scalar_v352) as f64).ln();
        self.scalar_v353 = v353;
        let v354: f64 = (self.scalar_v257 * self.scalar_v353);
        self.scalar_v354 = v354;
        let v355: f64 = ((self.scalar_v354) as f64).exp();
        self.scalar_v355 = v355;
        let v356: f64 = (2.0 - self.scalar_v355);
        self.scalar_v356 = v356;
        let v357: f64 = (self.scalar_v351 * self.scalar_v356);
        self.scalar_v357 = v357;
        let v360: f64 = (self.scalar_v204 * self.scalar_v359);
        self.scalar_v360 = v360;
        let v362: f64 = (self.scalar_v280 * self.scalar_v361);
        self.scalar_v362 = v362;
        let v363: f64 = (self.scalar_v360 + self.scalar_v362);
        self.scalar_v363 = v363;
        let v364: f64 = ((self.scalar_v363) as f64).exp();
        self.scalar_v364 = v364;
        let v365: f64 = (self.scalar_v358 * self.scalar_v364);
        self.scalar_v365 = v365;
        let v368: f64 = (self.scalar_v204 * self.scalar_v367);
        self.scalar_v368 = v368;
        let v369: f64 = ((self.scalar_v368) as f64).exp();
        self.scalar_v369 = v369;
        let v370: f64 = (self.scalar_v366 * self.scalar_v369);
        self.scalar_v370 = v370;
        let v379: f64 = (self.scalar_v200 * self.scalar_v378);
        self.scalar_v379 = v379;
        let v381: f64 = (self.scalar_v204 * self.scalar_v380);
        self.scalar_v381 = v381;
        let v382: f64 = ((self.scalar_v381) as f64).exp();
        self.scalar_v382 = v382;
        let v383: f64 = (self.scalar_v382 - 1.0);
        self.scalar_v383 = v383;
        let v384: f64 = (self.scalar_v379 * self.scalar_v383);
        self.scalar_v384 = v384;
        let v385: f64 = ((self.scalar_v384) as f64).exp();
        self.scalar_v385 = v385;
        let v386: f64 = (self.scalar_v377 * self.scalar_v385);
        self.scalar_v386 = v386;
        let v387: f64 = (if self.scalar_v376 { self.scalar_v386 } else { 0.0 });
        self.scalar_v387 = v387;
        let v389: f64 = (self.scalar_v371 * self.scalar_v385);
        self.scalar_v389 = v389;
        let v390: f64 = (if self.scalar_v388 { self.scalar_v389 } else { self.scalar_v387 });
        self.scalar_v390 = v390;
        let v393: f64 = (self.scalar_v280 * self.scalar_v392);
        self.scalar_v393 = v393;
        let v394: f64 = ((self.scalar_v393) as f64).exp();
        self.scalar_v394 = v394;
        let v395: f64 = (self.scalar_v391 * self.scalar_v394);
        self.scalar_v395 = v395;
        let v399: f64 = (self.scalar_v280 * self.scalar_v398);
        self.scalar_v399 = v399;
        let v400: f64 = ((self.scalar_v399) as f64).exp();
        self.scalar_v400 = v400;
        let v401: f64 = (self.scalar_v396 * self.scalar_v400);
        self.scalar_v401 = v401;
        let v405: f64 = (self.scalar_v280 * self.scalar_v404);
        self.scalar_v405 = v405;
        let v406: f64 = ((self.scalar_v405) as f64).exp();
        self.scalar_v406 = v406;
        let v407: f64 = (self.scalar_v402 * self.scalar_v406);
        self.scalar_v407 = v407;
        let v410: f64 = (self.scalar_v204 * self.scalar_v409);
        self.scalar_v410 = v410;
        let v411: f64 = ((self.scalar_v410) as f64).exp();
        self.scalar_v411 = v411;
        let v412: f64 = (self.scalar_v408 * self.scalar_v411);
        self.scalar_v412 = v412;
        let v414: f64 = (self.scalar_v80 * self.scalar_v204);
        self.scalar_v414 = v414;
        let v415: f64 = ((self.scalar_v414) as f64).exp();
        self.scalar_v415 = v415;
        let v416: f64 = (self.scalar_v413 * self.scalar_v415);
        self.scalar_v416 = v416;
        let v417: f64 = (1.0 / self.scalar_v416);
        self.scalar_v417 = v417;
        let v421: f64 = (self.scalar_v201 * self.scalar_v420);
        self.scalar_v421 = v421;
        let v422: f64 = (1.0 - self.scalar_v421);
        self.scalar_v422 = v422;
        let v423: f64 = (self.scalar_v418 * self.scalar_v422);
        self.scalar_v423 = v423;
        let v424: f64 = (if self.scalar_v419 { self.scalar_v423 } else { 0.0 });
        self.scalar_v424 = v424;
        let v429: f64 = (self.scalar_v201 * self.scalar_v428);
        self.scalar_v429 = v429;
        let v430: f64 = (1.0 + self.scalar_v429);
        self.scalar_v430 = v430;
        let v431: f64 = (self.scalar_v425 * self.scalar_v430);
        self.scalar_v431 = v431;
        let v432: f64 = (if self.scalar_v427 { self.scalar_v431 } else { self.scalar_v426 });
        self.scalar_v432 = v432;
        let v433: f64 = (if self.scalar_v427 { self.scalar_v418 } else { self.scalar_v424 });
        self.scalar_v433 = v433;
        let v435: f64 = (self.scalar_v201 * self.scalar_v434);
        self.scalar_v435 = v435;
        let v436: f64 = (1.0 + self.scalar_v435);
        self.scalar_v436 = v436;
        let v438: f64 = (self.scalar_v201 * self.scalar_v437);
        self.scalar_v438 = v438;
        let v439: f64 = (self.scalar_v201 * self.scalar_v438);
        self.scalar_v439 = v439;
        let v440: f64 = (self.scalar_v436 + self.scalar_v439);
        self.scalar_v440 = v440;
        let v441: f64 = (self.scalar_v133 * self.scalar_v440);
        self.scalar_v441 = v441;
        let v445: f64 = (self.scalar_v204 * self.scalar_v444);
        self.scalar_v445 = v445;
        let v446: f64 = ((self.scalar_v445) as f64).exp();
        self.scalar_v446 = v446;
        let v447: f64 = (self.scalar_v443 * self.scalar_v446);
        self.scalar_v447 = v447;
        let v450: f64 = (self.scalar_v201 * self.scalar_v449);
        self.scalar_v450 = v450;
        let v451: f64 = ((self.scalar_v450) as f64).exp();
        self.scalar_v451 = v451;
        let v452: f64 = (self.scalar_v118 * self.scalar_v451);
        self.scalar_v452 = v452;
        let v453: f64 = (if self.scalar_v448 { self.scalar_v452 } else { 0.0 });
        self.scalar_v453 = v453;
        let v456: f64 = (self.scalar_v201 * self.scalar_v455);
        self.scalar_v456 = v456;
        let v457: f64 = ((self.scalar_v456) as f64).exp();
        self.scalar_v457 = v457;
        let v458: f64 = (self.scalar_v454 * self.scalar_v457);
        self.scalar_v458 = v458;
        let v459: f64 = (if self.scalar_v448 { self.scalar_v458 } else { 0.0 });
        self.scalar_v459 = v459;
        let v461: f64 = (if self.scalar_v460 { self.scalar_v118 } else { self.scalar_v453 });
        self.scalar_v461 = v461;
        let v462: f64 = (if self.scalar_v460 { self.scalar_v454 } else { self.scalar_v459 });
        self.scalar_v462 = v462;
        let v473: f64 = (self.scalar_v68 / self.scalar_v218);
        self.scalar_v473 = v473;
        let v475: f64 = (self.scalar_v340 / self.scalar_v295);
        self.scalar_v475 = v475;
        let v493: f64 = (self.scalar_v204 * self.scalar_v492);
        self.scalar_v493 = v493;
        let v494: f64 = ((self.scalar_v493) as f64).exp();
        self.scalar_v494 = v494;
        let v495: f64 = (self.scalar_v491 * self.scalar_v494);
        self.scalar_v495 = v495;
        let v509: f64 = (self.scalar_v203 * self.scalar_v508);
        self.scalar_v509 = v509;
        let v510: f64 = (self.scalar_v237 + self.scalar_v509);
        self.scalar_v510 = v510;
        let v511: f64 = (self.scalar_v510 - self.scalar_v240);
        self.scalar_v511 = v511;
        let v512: f64 = (if self.scalar_v497 { self.scalar_v511 } else { self.scalar_v310 });
        self.scalar_v512 = v512;
        let v513: f64 = (-self.scalar_v512);
        self.scalar_v513 = v513;
        let v514: f64 = (self.scalar_v200 * self.scalar_v513);
        self.scalar_v514 = v514;
        let v515: f64 = ((self.scalar_v514) as f64).exp();
        self.scalar_v515 = v515;
        let v516: f64 = (4.0 * self.scalar_v515);
        self.scalar_v516 = v516;
        let v517: f64 = (1.0 + self.scalar_v516);
        self.scalar_v517 = v517;
        let v518: f64 = ((self.scalar_v517) as f64).sqrt();
        self.scalar_v518 = v518;
        let v519: f64 = (1.0 + self.scalar_v518);
        self.scalar_v519 = v519;
        let v520: f64 = (0.5 * self.scalar_v519);
        self.scalar_v520 = v520;
        let v521: f64 = ((self.scalar_v520) as f64).ln();
        self.scalar_v521 = v521;
        let v522: f64 = (self.scalar_v243 * self.scalar_v521);
        self.scalar_v522 = v522;
        let v523: f64 = (self.scalar_v512 + self.scalar_v522);
        self.scalar_v523 = v523;
        let v524: f64 = (if self.scalar_v497 { self.scalar_v523 } else { 0.0 });
        self.scalar_v524 = v524;
        let v526: f64 = (self.scalar_v498 / self.scalar_v524);
        self.scalar_v526 = v526;
        let v527: f64 = ((self.scalar_v526) as f64).ln();
        self.scalar_v527 = v527;
        let v528: f64 = (self.scalar_v525 * self.scalar_v527);
        self.scalar_v528 = v528;
        let v529: f64 = ((self.scalar_v528) as f64).exp();
        self.scalar_v529 = v529;
        let v530: f64 = (self.scalar_v496 * self.scalar_v529);
        self.scalar_v530 = v530;
        let v531: f64 = (if self.scalar_v497 { self.scalar_v530 } else { 0.0 });
        self.scalar_v531 = v531;
        let v537: f64 = (self.scalar_v524 * self.scalar_v532);
        self.scalar_v537 = v537;
        let v538: f64 = (self.scalar_v537 / self.scalar_v498);
        self.scalar_v538 = v538;
        let v539: f64 = (if self.scalar_v536 { self.scalar_v538 } else { self.scalar_v534 });
        self.scalar_v539 = v539;
        let v541: f64 = (if self.scalar_v540 { self.scalar_v496 } else { self.scalar_v531 });
        self.scalar_v541 = v541;
        let v542: f64 = (if self.scalar_v540 { self.scalar_v498 } else { self.scalar_v524 });
        self.scalar_v542 = v542;
        let v543: f64 = (if self.scalar_v540 { self.scalar_v532 } else { self.scalar_v539 });
        self.scalar_v543 = v543;
        let v545: f64 = (self.scalar_v283 * self.scalar_v544);
        self.scalar_v545 = v545;
        let v549: f64 = (self.scalar_v204 * self.scalar_v548);
        self.scalar_v549 = v549;
        let v550: f64 = (self.scalar_v290 / self.scalar_v547);
        self.scalar_v550 = v550;
        let v551: f64 = (self.scalar_v549 + self.scalar_v550);
        self.scalar_v551 = v551;
        let v552: f64 = ((self.scalar_v551) as f64).exp();
        self.scalar_v552 = v552;
        let v553: f64 = (self.scalar_v546 * self.scalar_v552);
        self.scalar_v553 = v553;
        let v561: f64 = (self.scalar_v66 / self.scalar_v216);
        self.scalar_v561 = v561;
        let v569: f64 = (self.scalar_v542 / self.scalar_v498);
        self.scalar_v569 = v569;
        let v571: f64 = (self.scalar_v541 / self.scalar_v496);
        self.scalar_v571 = v571;
        let v577: f64 = (self.scalar_v496 / self.scalar_v541);
        self.scalar_v577 = v577;
        let v591: f64 = (self.scalar_v273 / self.scalar_v219);
        self.scalar_v591 = v591;
        let v596: f64 = (self.scalar_v219 / self.scalar_v273);
        self.scalar_v596 = v596;
        let v609: f64 = (self.scalar_v274 - self.scalar_v223);
        self.scalar_v609 = v609;
        let v610: f64 = (-self.scalar_v609);
        self.scalar_v610 = v610;
        let v612: f64 = (self.scalar_v610 / self.scalar_v611);
        self.scalar_v612 = v612;
        let v613: f64 = ((self.scalar_v612) as f64).exp();
        self.scalar_v613 = v613;
        let v614: f64 = (self.scalar_v608 * self.scalar_v613);
        self.scalar_v614 = v614;
        let v626: f64 = (self.scalar_v203 * self.scalar_v625);
        self.scalar_v626 = v626;
        let v627: f64 = (self.scalar_v307 + self.scalar_v626);
        self.scalar_v627 = v627;
        let v628: f64 = (self.scalar_v627 - self.scalar_v240);
        self.scalar_v628 = v628;
        let v629: f64 = (-self.scalar_v628);
        self.scalar_v629 = v629;
        let v630: f64 = (self.scalar_v200 * self.scalar_v629);
        self.scalar_v630 = v630;
        let v631: f64 = ((self.scalar_v630) as f64).exp();
        self.scalar_v631 = v631;
        let v632: f64 = (4.0 * self.scalar_v631);
        self.scalar_v632 = v632;
        let v633: f64 = (1.0 + self.scalar_v632);
        self.scalar_v633 = v633;
        let v634: f64 = ((self.scalar_v633) as f64).sqrt();
        self.scalar_v634 = v634;
        let v635: f64 = (1.0 + self.scalar_v634);
        self.scalar_v635 = v635;
        let v636: f64 = (0.5 * self.scalar_v635);
        self.scalar_v636 = v636;
        let v637: f64 = ((self.scalar_v636) as f64).ln();
        self.scalar_v637 = v637;
        let v638: f64 = (self.scalar_v243 * self.scalar_v637);
        self.scalar_v638 = v638;
        let v639: f64 = (self.scalar_v628 + self.scalar_v638);
        self.scalar_v639 = v639;
        let v641: f64 = (self.scalar_v616 / self.scalar_v639);
        self.scalar_v641 = v641;
        let v642: f64 = ((self.scalar_v641) as f64).ln();
        self.scalar_v642 = v642;
        let v643: f64 = (self.scalar_v640 * self.scalar_v642);
        self.scalar_v643 = v643;
        let v644: f64 = ((self.scalar_v643) as f64).exp();
        self.scalar_v644 = v644;
        let v649: f64 = (self.scalar_v639 * self.scalar_v645);
        self.scalar_v649 = v649;
        let v650: f64 = (self.scalar_v649 / self.scalar_v616);
        self.scalar_v650 = v650;
        let v651: f64 = (if self.scalar_v648 { self.scalar_v650 } else { self.scalar_v646 });
        self.scalar_v651 = v651;
        let v653: f64 = (if self.scalar_v114 { 2.4 } else { self.scalar_v651 });
        self.scalar_v653 = v653;
        let v654: f64 = (self.scalar_v102 * self.scalar_v644);
        self.scalar_v654 = v654;
        let v655: f64 = (self.scalar_v103 * self.scalar_v644);
        self.scalar_v655 = v655;
        let v657: f64 = (self.scalar_v83 * self.scalar_v204);
        self.scalar_v657 = v657;
        let v658: f64 = (self.scalar_v347 + self.scalar_v657);
        self.scalar_v658 = v658;
        let v659: f64 = ((self.scalar_v658) as f64).exp();
        self.scalar_v659 = v659;
        let v660: f64 = (self.scalar_v656 * self.scalar_v659);
        self.scalar_v660 = v660;
        let v675: f64 = (self.scalar_v203 * self.scalar_v674);
        self.scalar_v675 = v675;
        let v676: f64 = (self.scalar_v75 * self.scalar_v236);
        self.scalar_v676 = v676;
        let v677: f64 = (self.scalar_v675 + self.scalar_v676);
        self.scalar_v677 = v677;
        let v678: f64 = (self.scalar_v677 - self.scalar_v240);
        self.scalar_v678 = v678;
        let v679: f64 = (if self.scalar_v663 { self.scalar_v678 } else { self.scalar_v628 });
        self.scalar_v679 = v679;
        let v680: f64 = (-self.scalar_v679);
        self.scalar_v680 = v680;
        let v681: f64 = (self.scalar_v200 * self.scalar_v680);
        self.scalar_v681 = v681;
        let v682: f64 = ((self.scalar_v681) as f64).exp();
        self.scalar_v682 = v682;
        let v683: f64 = (4.0 * self.scalar_v682);
        self.scalar_v683 = v683;
        let v684: f64 = (1.0 + self.scalar_v683);
        self.scalar_v684 = v684;
        let v685: f64 = ((self.scalar_v684) as f64).sqrt();
        self.scalar_v685 = v685;
        let v686: f64 = (1.0 + self.scalar_v685);
        self.scalar_v686 = v686;
        let v687: f64 = (0.5 * self.scalar_v686);
        self.scalar_v687 = v687;
        let v688: f64 = ((self.scalar_v687) as f64).ln();
        self.scalar_v688 = v688;
        let v689: f64 = (self.scalar_v243 * self.scalar_v688);
        self.scalar_v689 = v689;
        let v690: f64 = (self.scalar_v679 + self.scalar_v689);
        self.scalar_v690 = v690;
        let v691: f64 = (if self.scalar_v663 { self.scalar_v690 } else { 0.0 });
        self.scalar_v691 = v691;
        let v693: f64 = (self.scalar_v664 / self.scalar_v691);
        self.scalar_v693 = v693;
        let v694: f64 = ((self.scalar_v693) as f64).ln();
        self.scalar_v694 = v694;
        let v695: f64 = (self.scalar_v692 * self.scalar_v694);
        self.scalar_v695 = v695;
        let v696: f64 = ((self.scalar_v695) as f64).exp();
        self.scalar_v696 = v696;
        let v697: f64 = (self.scalar_v661 * self.scalar_v696);
        self.scalar_v697 = v697;
        let v698: f64 = (if self.scalar_v663 { self.scalar_v697 } else { 0.0 });
        self.scalar_v698 = v698;
        let v702: f64 = (self.scalar_v691 * -2.4);
        self.scalar_v702 = v702;
        let v703: f64 = (self.scalar_v702 / self.scalar_v664);
        self.scalar_v703 = v703;
        let v704: f64 = (if self.scalar_v701 { self.scalar_v703 } else { self.scalar_v700 });
        self.scalar_v704 = v704;
        let v707: f64 = (if self.scalar_v706 { self.scalar_v661 } else { self.scalar_v698 });
        self.scalar_v707 = v707;
        let v708: f64 = (if self.scalar_v706 { self.scalar_v664 } else { self.scalar_v691 });
        self.scalar_v708 = v708;
        let v709: f64 = (if self.scalar_v706 { -2.4 } else { self.scalar_v704 });
        self.scalar_v709 = v709;
        let v713: f64 = (self.scalar_v203 * self.scalar_v712);
        self.scalar_v713 = v713;
        let v714: f64 = (self.scalar_v676 + self.scalar_v713);
        self.scalar_v714 = v714;
        let v715: f64 = (self.scalar_v714 - self.scalar_v240);
        self.scalar_v715 = v715;
        let v716: f64 = (if self.scalar_v711 { self.scalar_v715 } else { self.scalar_v679 });
        self.scalar_v716 = v716;
        let v717: f64 = (-self.scalar_v716);
        self.scalar_v717 = v717;
        let v718: f64 = (self.scalar_v200 * self.scalar_v717);
        self.scalar_v718 = v718;
        let v719: f64 = ((self.scalar_v718) as f64).exp();
        self.scalar_v719 = v719;
        let v720: f64 = (4.0 * self.scalar_v719);
        self.scalar_v720 = v720;
        let v721: f64 = (1.0 + self.scalar_v720);
        self.scalar_v721 = v721;
        let v722: f64 = ((self.scalar_v721) as f64).sqrt();
        self.scalar_v722 = v722;
        let v723: f64 = (1.0 + self.scalar_v722);
        self.scalar_v723 = v723;
        let v724: f64 = (0.5 * self.scalar_v723);
        self.scalar_v724 = v724;
        let v725: f64 = ((self.scalar_v724) as f64).ln();
        self.scalar_v725 = v725;
        let v726: f64 = (self.scalar_v243 * self.scalar_v725);
        self.scalar_v726 = v726;
        let v727: f64 = (self.scalar_v716 + self.scalar_v726);
        self.scalar_v727 = v727;
        let v728: f64 = (if self.scalar_v711 { self.scalar_v727 } else { self.scalar_v708 });
        self.scalar_v728 = v728;
        let v729: f64 = (self.scalar_v664 / self.scalar_v728);
        self.scalar_v729 = v729;
        let v730: f64 = ((self.scalar_v729) as f64).ln();
        self.scalar_v730 = v730;
        let v731: f64 = (self.scalar_v692 * self.scalar_v730);
        self.scalar_v731 = v731;
        let v732: f64 = ((self.scalar_v731) as f64).exp();
        self.scalar_v732 = v732;
        let v733: f64 = (self.scalar_v661 * self.scalar_v732);
        self.scalar_v733 = v733;
        let v734: f64 = (if self.scalar_v711 { self.scalar_v733 } else { self.scalar_v707 });
        self.scalar_v734 = v734;
        let v738: f64 = (if self.scalar_v711 { self.scalar_v737 } else { self.scalar_v709 });
        self.scalar_v738 = v738;
        let v741: f64 = (self.scalar_v728 * self.scalar_v736);
        self.scalar_v741 = v741;
        let v742: f64 = (self.scalar_v741 / self.scalar_v664);
        self.scalar_v742 = v742;
        let v743: f64 = (if self.scalar_v740 { self.scalar_v742 } else { self.scalar_v738 });
        self.scalar_v743 = v743;
        let v745: f64 = (if self.scalar_v744 { self.scalar_v661 } else { self.scalar_v734 });
        self.scalar_v745 = v745;
        let v746: f64 = (if self.scalar_v744 { self.scalar_v664 } else { self.scalar_v728 });
        self.scalar_v746 = v746;
        let v747: f64 = (if self.scalar_v744 { self.scalar_v736 } else { self.scalar_v743 });
        self.scalar_v747 = v747;
        let v750: f64 = (self.scalar_v85 * self.scalar_v204);
        self.scalar_v750 = v750;
        let v752: f64 = (self.scalar_v280 * self.scalar_v751);
        self.scalar_v752 = v752;
        let v753: f64 = (self.scalar_v750 + self.scalar_v752);
        self.scalar_v753 = v753;
        let v754: f64 = ((self.scalar_v753) as f64).exp();
        self.scalar_v754 = v754;
        let v755: f64 = (self.scalar_v749 * self.scalar_v754);
        self.scalar_v755 = v755;
        let v757: f64 = (self.scalar_v347 + self.scalar_v750);
        self.scalar_v757 = v757;
        let v758: f64 = ((self.scalar_v757) as f64).exp();
        self.scalar_v758 = v758;
        let v759: f64 = (self.scalar_v756 * self.scalar_v758);
        self.scalar_v759 = v759;
        let v762: f64 = (self.scalar_v204 * self.scalar_v761);
        self.scalar_v762 = v762;
        let v763: f64 = ((self.scalar_v762) as f64).exp();
        self.scalar_v763 = v763;
        let v764: f64 = (self.scalar_v760 * self.scalar_v763);
        self.scalar_v764 = v764;
        let v780: f64 = (self.scalar_v203 * self.scalar_v779);
        self.scalar_v780 = v780;
        let v781: f64 = (self.scalar_v676 + self.scalar_v780);
        self.scalar_v781 = v781;
        let v782: f64 = (self.scalar_v781 - self.scalar_v240);
        self.scalar_v782 = v782;
        let v783: f64 = (if self.scalar_v769 { self.scalar_v782 } else { self.scalar_v716 });
        self.scalar_v783 = v783;
        let v784: f64 = (-self.scalar_v783);
        self.scalar_v784 = v784;
        let v785: f64 = (self.scalar_v200 * self.scalar_v784);
        self.scalar_v785 = v785;
        let v786: f64 = ((self.scalar_v785) as f64).exp();
        self.scalar_v786 = v786;
        let v787: f64 = (4.0 * self.scalar_v786);
        self.scalar_v787 = v787;
        let v788: f64 = (1.0 + self.scalar_v787);
        self.scalar_v788 = v788;
        let v789: f64 = ((self.scalar_v788) as f64).sqrt();
        self.scalar_v789 = v789;
        let v790: f64 = (1.0 + self.scalar_v789);
        self.scalar_v790 = v790;
        let v791: f64 = (0.5 * self.scalar_v790);
        self.scalar_v791 = v791;
        let v792: f64 = ((self.scalar_v791) as f64).ln();
        self.scalar_v792 = v792;
        let v793: f64 = (self.scalar_v243 * self.scalar_v792);
        self.scalar_v793 = v793;
        let v794: f64 = (self.scalar_v783 + self.scalar_v793);
        self.scalar_v794 = v794;
        let v795: f64 = (if self.scalar_v769 { self.scalar_v794 } else { 0.0 });
        self.scalar_v795 = v795;
        let v797: f64 = (self.scalar_v765 / self.scalar_v795);
        self.scalar_v797 = v797;
        let v798: f64 = ((self.scalar_v797) as f64).ln();
        self.scalar_v798 = v798;
        let v799: f64 = (self.scalar_v796 * self.scalar_v798);
        self.scalar_v799 = v799;
        let v800: f64 = ((self.scalar_v799) as f64).exp();
        self.scalar_v800 = v800;
        let v801: f64 = (self.scalar_v767 * self.scalar_v800);
        self.scalar_v801 = v801;
        let v802: f64 = (if self.scalar_v769 { self.scalar_v801 } else { 0.0 });
        self.scalar_v802 = v802;
        let v808: f64 = (self.scalar_v795 * self.scalar_v803);
        self.scalar_v808 = v808;
        let v809: f64 = (self.scalar_v808 / self.scalar_v765);
        self.scalar_v809 = v809;
        let v810: f64 = (if self.scalar_v807 { self.scalar_v809 } else { self.scalar_v805 });
        self.scalar_v810 = v810;
        let v813: f64 = (if self.scalar_v812 { self.scalar_v767 } else { self.scalar_v802 });
        self.scalar_v813 = v813;
        let v814: f64 = (if self.scalar_v812 { self.scalar_v765 } else { self.scalar_v795 });
        self.scalar_v814 = v814;
        let v815: f64 = (if self.scalar_v812 { self.scalar_v803 } else { self.scalar_v810 });
        self.scalar_v815 = v815;
        let v817: f64 = (if self.scalar_v816 { self.scalar_v767 } else { self.scalar_v813 });
        self.scalar_v817 = v817;
        let v818: f64 = (if self.scalar_v816 { self.scalar_v765 } else { self.scalar_v814 });
        self.scalar_v818 = v818;
        let v819: f64 = (if self.scalar_v816 { self.scalar_v748 } else { self.scalar_v815 });
        self.scalar_v819 = v819;
        let v822: f64 = (self.scalar_v204 * self.scalar_v821);
        self.scalar_v822 = v822;
        let v823: f64 = ((self.scalar_v822) as f64).exp();
        self.scalar_v823 = v823;
        let v824: f64 = (self.scalar_v820 * self.scalar_v823);
        self.scalar_v824 = v824;
        let v827: f64 = (self.scalar_v204 * self.scalar_v826);
        self.scalar_v827 = v827;
        let v828: f64 = ((self.scalar_v827) as f64).exp();
        self.scalar_v828 = v828;
        let v829: f64 = (self.scalar_v825 * self.scalar_v828);
        self.scalar_v829 = v829;
        let v832: f64 = (self.scalar_v204 * self.scalar_v831);
        self.scalar_v832 = v832;
        let v833: f64 = ((self.scalar_v832) as f64).exp();
        self.scalar_v833 = v833;
        let v834: f64 = (self.scalar_v830 * self.scalar_v833);
        self.scalar_v834 = v834;
        let v837: f64 = (self.scalar_v204 * self.scalar_v836);
        self.scalar_v837 = v837;
        let v838: f64 = ((self.scalar_v837) as f64).exp();
        self.scalar_v838 = v838;
        let v839: f64 = (self.scalar_v835 * self.scalar_v838);
        self.scalar_v839 = v839;
        let v841: f64 = (self.scalar_v201 * self.scalar_v840);
        self.scalar_v841 = v841;
        let v842: f64 = (1.0 + self.scalar_v841);
        self.scalar_v842 = v842;
        let v843: f64 = (self.scalar_v839 * self.scalar_v842);
        self.scalar_v843 = v843;
        let v924: f64 = (if self.scalar_v895 { self.scalar_v265 } else { self.scalar_v275 });
        self.scalar_v924 = v924;
        let v972: f64 = (if self.scalar_v947 { self.scalar_v331 } else { self.scalar_v343 });
        self.scalar_v972 = v972;
        let v1045: f64 = (if self.scalar_v1040 { self.scalar_v425 } else { self.scalar_v432 });
        self.scalar_v1045 = v1045;
        let v1125: f64 = (if self.scalar_v1101 { self.scalar_v533 } else { self.scalar_v543 });
        self.scalar_v1125 = v1125;
        let v1209: f64 = (if self.scalar_v1186 { self.scalar_v646 } else { self.scalar_v653 });
        self.scalar_v1209 = v1209;
        let v1253: f64 = (if self.scalar_v1228 { 2.4 } else { self.scalar_v747 });
        self.scalar_v1253 = v1253;
        let v1339: f64 = (if self.scalar_v1313 { self.scalar_v1338 } else { self.scalar_v819 });
        self.scalar_v1339 = v1339;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
