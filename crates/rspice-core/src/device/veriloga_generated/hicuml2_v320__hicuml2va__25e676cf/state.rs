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
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v25: bool,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: bool,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v42: f64,
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
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
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
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
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
    pub(crate) scalar_v103: bool,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: bool,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: bool,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: bool,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: bool,
    pub(crate) scalar_v116: bool,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: bool,
    pub(crate) scalar_v119: bool,
    pub(crate) scalar_v120: bool,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: bool,
    pub(crate) scalar_v200: bool,
    pub(crate) scalar_v204: bool,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
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
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: bool,
    pub(crate) scalar_v259: bool,
    pub(crate) scalar_v263: bool,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: bool,
    pub(crate) scalar_v277: bool,
    pub(crate) scalar_v278: bool,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: bool,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: bool,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: bool,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: bool,
    pub(crate) scalar_v354: bool,
    pub(crate) scalar_v355: bool,
    pub(crate) scalar_v356: bool,
    pub(crate) scalar_v372: bool,
    pub(crate) scalar_v373: bool,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: bool,
    pub(crate) scalar_v430: bool,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: bool,
    pub(crate) scalar_v445: bool,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: bool,
    pub(crate) scalar_v487: bool,
    pub(crate) scalar_v488: bool,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v493: bool,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v521: bool,
    pub(crate) scalar_v522: bool,
    pub(crate) scalar_v526: bool,
    pub(crate) scalar_v530: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v538: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: bool,
    pub(crate) scalar_v549: f64,
    pub(crate) scalar_v550: bool,
    pub(crate) scalar_v551: bool,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v558: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v585: f64,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v587: f64,
    pub(crate) scalar_v588: bool,
    pub(crate) scalar_v589: bool,
    pub(crate) scalar_v593: bool,
    pub(crate) scalar_v594: bool,
    pub(crate) scalar_v598: bool,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v618: f64,
    pub(crate) scalar_v619: bool,
    pub(crate) scalar_v620: f64,
    pub(crate) scalar_v621: bool,
    pub(crate) scalar_v622: bool,
    pub(crate) scalar_v623: bool,
    pub(crate) scalar_v624: bool,
    pub(crate) scalar_v667: bool,
    pub(crate) scalar_v668: f64,
    pub(crate) scalar_v697: bool,
    pub(crate) scalar_v701: bool,
    pub(crate) scalar_v711: bool,
    pub(crate) scalar_v712: f64,
    pub(crate) scalar_v737: bool,
    pub(crate) scalar_v741: bool,
    pub(crate) scalar_v745: bool,
    pub(crate) scalar_v755: bool,
    pub(crate) scalar_v756: f64,
    pub(crate) scalar_v780: bool,
    pub(crate) scalar_v784: bool,
    pub(crate) scalar_v834: bool,
    pub(crate) scalar_v835: f64,
    pub(crate) scalar_v858: bool,
    pub(crate) scalar_v862: bool,
    pub(crate) scalar_v876: bool,
    pub(crate) scalar_v877: f64,
    pub(crate) scalar_v902: bool,
    pub(crate) scalar_v906: bool,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v911: bool,
    pub(crate) scalar_v912: bool,
    pub(crate) scalar_v913: f64,
    pub(crate) scalar_v937: bool,
    pub(crate) scalar_v941: bool,
    pub(crate) scalar_v945: f64,
    pub(crate) scalar_v960: bool,
    pub(crate) scalar_v961: bool,
    pub(crate) scalar_v962: f64,
    pub(crate) scalar_v985: f64,
    pub(crate) scalar_v986: f64,
    pub(crate) scalar_v988: bool,
    pub(crate) scalar_v989: bool,
    pub(crate) scalar_v993: bool,
    pub(crate) scalar_v997: bool,
    pub(crate) scalar_v1013: bool,
    pub(crate) scalar_v1014: f64,
    pub(crate) scalar_v1028: bool,
    pub(crate) scalar_v1069: f64,
    pub(crate) scalar_v1079: f64,
    pub(crate) scalar_v1088: f64,
    pub(crate) scalar_v1090: bool,
    pub(crate) scalar_v1093: f64,
    pub(crate) scalar_v1164: f64,
    pub(crate) scalar_v1168: f64,
    pub(crate) scalar_v1187: bool,
    pub(crate) scalar_v1219: bool,
    pub(crate) scalar_v1221: bool,
    pub(crate) scalar_v1222: bool,
    pub(crate) scalar_v1223: f64,
    pub(crate) scalar_v1236: bool,
    pub(crate) scalar_v1237: f64,
    pub(crate) scalar_v1255: bool,
    pub(crate) scalar_v1257: bool,
    pub(crate) scalar_v1275: bool,
    pub(crate) scalar_v1304: f64,
    pub(crate) scalar_v1314: f64,
    pub(crate) scalar_v1333: f64,
    pub(crate) scalar_v1334: f64,
    pub(crate) scalar_v1356: f64,
    pub(crate) scalar_v1357: f64,
    pub(crate) scalar_v1376: f64,
    pub(crate) scalar_v1377: bool,
    pub(crate) scalar_v1380: f64,
    pub(crate) scalar_v1449: f64,
    pub(crate) scalar_v1480: bool,
    pub(crate) scalar_v1513: bool,
    pub(crate) scalar_v1514: f64,
    pub(crate) scalar_v1532: bool,
    pub(crate) scalar_v1657: f64,
    pub(crate) scalar_v1658: bool,
    pub(crate) scalar_v1661: f64,
    pub(crate) scalar_v1730: f64,
    pub(crate) scalar_v1761: bool,
    pub(crate) scalar_v1794: f64,
    pub(crate) scalar_v1795: bool,
    pub(crate) scalar_v1797: bool,
    pub(crate) scalar_v1799: f64,
    pub(crate) scalar_v1868: f64,
    pub(crate) scalar_v1899: bool,
    pub(crate) scalar_v1900: bool,
    pub(crate) scalar_v1935: bool,
    pub(crate) scalar_v1936: f64,
    pub(crate) scalar_v1948: bool,
    pub(crate) scalar_v1949: bool,
    pub(crate) scalar_v1953: bool,
    pub(crate) scalar_v1954: bool,
    pub(crate) scalar_v1956: bool,
    pub(crate) scalar_v1959: bool,
    pub(crate) scalar_v1960: f64,
    pub(crate) scalar_v1978: bool,
    pub(crate) scalar_v1980: bool,
    pub(crate) scalar_v1997: bool,
    pub(crate) scalar_v2001: bool,
    pub(crate) scalar_v2002: bool,
    pub(crate) scalar_v2003: bool,
    pub(crate) scalar_v2004: bool,
    pub(crate) scalar_v2005: bool,
    pub(crate) scalar_v2006: bool,
    pub(crate) scalar_v2007: bool,
    pub(crate) scalar_v2008: bool,
    pub(crate) scalar_v2009: bool,
    pub(crate) scalar_v2010: bool,
    pub(crate) scalar_v2011: bool,
    pub(crate) scalar_v2012: bool,
    pub(crate) scalar_v2013: f64,
    pub(crate) scalar_v2014: bool,
    pub(crate) scalar_v2015: bool,
    pub(crate) scalar_v2016: bool,
    pub(crate) scalar_v2017: bool,
    pub(crate) scalar_v2018: bool,
    pub(crate) scalar_v2019: bool,
    pub(crate) scalar_v2025: f64,
    pub(crate) scalar_v2026: bool,
    pub(crate) scalar_v2027: bool,
    pub(crate) scalar_v2028: bool,
    pub(crate) scalar_v2029: bool,
    pub(crate) scalar_v2030: bool,
    pub(crate) scalar_v2031: f64,
    pub(crate) scalar_v2034: f64,
    pub(crate) scalar_v2035: bool,
    pub(crate) scalar_v2036: f64,
    pub(crate) scalar_v2039: bool,
    pub(crate) scalar_v2053: bool,
    pub(crate) scalar_v2054: f64,
    pub(crate) scalar_v2055: bool,
    pub(crate) scalar_v2056: f64,
    pub(crate) scalar_v2060: bool,
    pub(crate) scalar_v2061: f64,
    pub(crate) scalar_v2065: f64,
    pub(crate) scalar_v2068: bool,
    pub(crate) scalar_v2073: bool,
    pub(crate) scalar_v2075: bool,
    pub(crate) scalar_v2082: bool,
    pub(crate) scalar_v2083: f64,
    pub(crate) scalar_v2084: bool,
    pub(crate) scalar_v2085: f64,
    pub(crate) scalar_v2094: bool,
    pub(crate) scalar_v2097: f64,
    pub(crate) scalar_v5912: f64,
    pub(crate) scalar_v5913: f64,
    pub(crate) scalar_v6058: f64,
    pub(crate) scalar_v6059: f64,
    pub(crate) scalar_v6060: f64,
    pub(crate) scalar_v6061: f64,
    pub(crate) scalar_v6062: f64,
    pub(crate) scalar_v6063: f64,
    pub(crate) scalar_v6064: f64,
    pub(crate) scalar_v6065: f64,
    pub(crate) scalar_v6066: f64,
    pub(crate) scalar_v6067: f64,
    pub(crate) scalar_v6068: f64,
    pub(crate) scalar_v6069: f64,
    pub(crate) scalar_v6070: f64,
    pub(crate) scalar_v6123: f64,
    pub(crate) scalar_v6130: f64,
    pub(crate) scalar_v6149: f64,
    pub(crate) scalar_v6150: f64,
    pub(crate) scalar_v6151: f64,
    pub(crate) scalar_v6169: f64,
    pub(crate) scalar_v6170: f64,
    pub(crate) scalar_v6177: f64,
    pub(crate) scalar_v6178: f64,
    pub(crate) scalar_v6195: f64,
    pub(crate) scalar_v6196: f64,
    pub(crate) scalar_v6197: f64,
    pub(crate) scalar_v6198: f64,
    pub(crate) scalar_v6199: f64,
    pub(crate) scalar_v6200: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v125: bool,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v128: bool,
    pub(crate) scalar_v129: bool,
    pub(crate) scalar_v130: bool,
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
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: f64,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v467: f64,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
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
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v536: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: f64,
    pub(crate) scalar_v565: f64,
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
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v579: f64,
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v582: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v590: f64,
    pub(crate) scalar_v591: f64,
    pub(crate) scalar_v592: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v609: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v615: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v736: f64,
    pub(crate) scalar_v779: f64,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v901: f64,
    pub(crate) scalar_v987: f64,
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
            scalar_v23: self.scalar_v23,
            scalar_v25: self.scalar_v25,
            scalar_v28: self.scalar_v28,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v33: self.scalar_v33,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v38: self.scalar_v38,
            scalar_v40: self.scalar_v40,
            scalar_v42: self.scalar_v42,
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
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
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
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v77: self.scalar_v77,
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
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v189: self.scalar_v189,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v198: self.scalar_v198,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v204: self.scalar_v204,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v211: self.scalar_v211,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
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
            scalar_v248: self.scalar_v248,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v259: self.scalar_v259,
            scalar_v263: self.scalar_v263,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v314: self.scalar_v314,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v325: self.scalar_v325,
            scalar_v329: self.scalar_v329,
            scalar_v333: self.scalar_v333,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v372: self.scalar_v372,
            scalar_v373: self.scalar_v373,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v391: self.scalar_v391,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v407: self.scalar_v407,
            scalar_v422: self.scalar_v422,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v438: self.scalar_v438,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v474: self.scalar_v474,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v492: self.scalar_v492,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v519: self.scalar_v519,
            scalar_v521: self.scalar_v521,
            scalar_v522: self.scalar_v522,
            scalar_v526: self.scalar_v526,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v533: self.scalar_v533,
            scalar_v538: self.scalar_v538,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v549: self.scalar_v549,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v553: self.scalar_v553,
            scalar_v554: self.scalar_v554,
            scalar_v555: self.scalar_v555,
            scalar_v556: self.scalar_v556,
            scalar_v557: self.scalar_v557,
            scalar_v558: self.scalar_v558,
            scalar_v559: self.scalar_v559,
            scalar_v560: self.scalar_v560,
            scalar_v561: self.scalar_v561,
            scalar_v578: self.scalar_v578,
            scalar_v585: self.scalar_v585,
            scalar_v586: self.scalar_v586,
            scalar_v587: self.scalar_v587,
            scalar_v588: self.scalar_v588,
            scalar_v589: self.scalar_v589,
            scalar_v593: self.scalar_v593,
            scalar_v594: self.scalar_v594,
            scalar_v598: self.scalar_v598,
            scalar_v602: self.scalar_v602,
            scalar_v603: self.scalar_v603,
            scalar_v607: self.scalar_v607,
            scalar_v608: self.scalar_v608,
            scalar_v612: self.scalar_v612,
            scalar_v613: self.scalar_v613,
            scalar_v617: self.scalar_v617,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v624: self.scalar_v624,
            scalar_v667: self.scalar_v667,
            scalar_v668: self.scalar_v668,
            scalar_v697: self.scalar_v697,
            scalar_v701: self.scalar_v701,
            scalar_v711: self.scalar_v711,
            scalar_v712: self.scalar_v712,
            scalar_v737: self.scalar_v737,
            scalar_v741: self.scalar_v741,
            scalar_v745: self.scalar_v745,
            scalar_v755: self.scalar_v755,
            scalar_v756: self.scalar_v756,
            scalar_v780: self.scalar_v780,
            scalar_v784: self.scalar_v784,
            scalar_v834: self.scalar_v834,
            scalar_v835: self.scalar_v835,
            scalar_v858: self.scalar_v858,
            scalar_v862: self.scalar_v862,
            scalar_v876: self.scalar_v876,
            scalar_v877: self.scalar_v877,
            scalar_v902: self.scalar_v902,
            scalar_v906: self.scalar_v906,
            scalar_v910: self.scalar_v910,
            scalar_v911: self.scalar_v911,
            scalar_v912: self.scalar_v912,
            scalar_v913: self.scalar_v913,
            scalar_v937: self.scalar_v937,
            scalar_v941: self.scalar_v941,
            scalar_v945: self.scalar_v945,
            scalar_v960: self.scalar_v960,
            scalar_v961: self.scalar_v961,
            scalar_v962: self.scalar_v962,
            scalar_v985: self.scalar_v985,
            scalar_v986: self.scalar_v986,
            scalar_v988: self.scalar_v988,
            scalar_v989: self.scalar_v989,
            scalar_v993: self.scalar_v993,
            scalar_v997: self.scalar_v997,
            scalar_v1013: self.scalar_v1013,
            scalar_v1014: self.scalar_v1014,
            scalar_v1028: self.scalar_v1028,
            scalar_v1069: self.scalar_v1069,
            scalar_v1079: self.scalar_v1079,
            scalar_v1088: self.scalar_v1088,
            scalar_v1090: self.scalar_v1090,
            scalar_v1093: self.scalar_v1093,
            scalar_v1164: self.scalar_v1164,
            scalar_v1168: self.scalar_v1168,
            scalar_v1187: self.scalar_v1187,
            scalar_v1219: self.scalar_v1219,
            scalar_v1221: self.scalar_v1221,
            scalar_v1222: self.scalar_v1222,
            scalar_v1223: self.scalar_v1223,
            scalar_v1236: self.scalar_v1236,
            scalar_v1237: self.scalar_v1237,
            scalar_v1255: self.scalar_v1255,
            scalar_v1257: self.scalar_v1257,
            scalar_v1275: self.scalar_v1275,
            scalar_v1304: self.scalar_v1304,
            scalar_v1314: self.scalar_v1314,
            scalar_v1333: self.scalar_v1333,
            scalar_v1334: self.scalar_v1334,
            scalar_v1356: self.scalar_v1356,
            scalar_v1357: self.scalar_v1357,
            scalar_v1376: self.scalar_v1376,
            scalar_v1377: self.scalar_v1377,
            scalar_v1380: self.scalar_v1380,
            scalar_v1449: self.scalar_v1449,
            scalar_v1480: self.scalar_v1480,
            scalar_v1513: self.scalar_v1513,
            scalar_v1514: self.scalar_v1514,
            scalar_v1532: self.scalar_v1532,
            scalar_v1657: self.scalar_v1657,
            scalar_v1658: self.scalar_v1658,
            scalar_v1661: self.scalar_v1661,
            scalar_v1730: self.scalar_v1730,
            scalar_v1761: self.scalar_v1761,
            scalar_v1794: self.scalar_v1794,
            scalar_v1795: self.scalar_v1795,
            scalar_v1797: self.scalar_v1797,
            scalar_v1799: self.scalar_v1799,
            scalar_v1868: self.scalar_v1868,
            scalar_v1899: self.scalar_v1899,
            scalar_v1900: self.scalar_v1900,
            scalar_v1935: self.scalar_v1935,
            scalar_v1936: self.scalar_v1936,
            scalar_v1948: self.scalar_v1948,
            scalar_v1949: self.scalar_v1949,
            scalar_v1953: self.scalar_v1953,
            scalar_v1954: self.scalar_v1954,
            scalar_v1956: self.scalar_v1956,
            scalar_v1959: self.scalar_v1959,
            scalar_v1960: self.scalar_v1960,
            scalar_v1978: self.scalar_v1978,
            scalar_v1980: self.scalar_v1980,
            scalar_v1997: self.scalar_v1997,
            scalar_v2001: self.scalar_v2001,
            scalar_v2002: self.scalar_v2002,
            scalar_v2003: self.scalar_v2003,
            scalar_v2004: self.scalar_v2004,
            scalar_v2005: self.scalar_v2005,
            scalar_v2006: self.scalar_v2006,
            scalar_v2007: self.scalar_v2007,
            scalar_v2008: self.scalar_v2008,
            scalar_v2009: self.scalar_v2009,
            scalar_v2010: self.scalar_v2010,
            scalar_v2011: self.scalar_v2011,
            scalar_v2012: self.scalar_v2012,
            scalar_v2013: self.scalar_v2013,
            scalar_v2014: self.scalar_v2014,
            scalar_v2015: self.scalar_v2015,
            scalar_v2016: self.scalar_v2016,
            scalar_v2017: self.scalar_v2017,
            scalar_v2018: self.scalar_v2018,
            scalar_v2019: self.scalar_v2019,
            scalar_v2025: self.scalar_v2025,
            scalar_v2026: self.scalar_v2026,
            scalar_v2027: self.scalar_v2027,
            scalar_v2028: self.scalar_v2028,
            scalar_v2029: self.scalar_v2029,
            scalar_v2030: self.scalar_v2030,
            scalar_v2031: self.scalar_v2031,
            scalar_v2034: self.scalar_v2034,
            scalar_v2035: self.scalar_v2035,
            scalar_v2036: self.scalar_v2036,
            scalar_v2039: self.scalar_v2039,
            scalar_v2053: self.scalar_v2053,
            scalar_v2054: self.scalar_v2054,
            scalar_v2055: self.scalar_v2055,
            scalar_v2056: self.scalar_v2056,
            scalar_v2060: self.scalar_v2060,
            scalar_v2061: self.scalar_v2061,
            scalar_v2065: self.scalar_v2065,
            scalar_v2068: self.scalar_v2068,
            scalar_v2073: self.scalar_v2073,
            scalar_v2075: self.scalar_v2075,
            scalar_v2082: self.scalar_v2082,
            scalar_v2083: self.scalar_v2083,
            scalar_v2084: self.scalar_v2084,
            scalar_v2085: self.scalar_v2085,
            scalar_v2094: self.scalar_v2094,
            scalar_v2097: self.scalar_v2097,
            scalar_v5912: self.scalar_v5912,
            scalar_v5913: self.scalar_v5913,
            scalar_v6058: self.scalar_v6058,
            scalar_v6059: self.scalar_v6059,
            scalar_v6060: self.scalar_v6060,
            scalar_v6061: self.scalar_v6061,
            scalar_v6062: self.scalar_v6062,
            scalar_v6063: self.scalar_v6063,
            scalar_v6064: self.scalar_v6064,
            scalar_v6065: self.scalar_v6065,
            scalar_v6066: self.scalar_v6066,
            scalar_v6067: self.scalar_v6067,
            scalar_v6068: self.scalar_v6068,
            scalar_v6069: self.scalar_v6069,
            scalar_v6070: self.scalar_v6070,
            scalar_v6123: self.scalar_v6123,
            scalar_v6130: self.scalar_v6130,
            scalar_v6149: self.scalar_v6149,
            scalar_v6150: self.scalar_v6150,
            scalar_v6151: self.scalar_v6151,
            scalar_v6169: self.scalar_v6169,
            scalar_v6170: self.scalar_v6170,
            scalar_v6177: self.scalar_v6177,
            scalar_v6178: self.scalar_v6178,
            scalar_v6195: self.scalar_v6195,
            scalar_v6196: self.scalar_v6196,
            scalar_v6197: self.scalar_v6197,
            scalar_v6198: self.scalar_v6198,
            scalar_v6199: self.scalar_v6199,
            scalar_v6200: self.scalar_v6200,
            scalar_v123: self.scalar_v123,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
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
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
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
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
            scalar_v207: self.scalar_v207,
            scalar_v210: self.scalar_v210,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v219: self.scalar_v219,
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
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v268: self.scalar_v268,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v280: self.scalar_v280,
            scalar_v282: self.scalar_v282,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v334: self.scalar_v334,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v350: self.scalar_v350,
            scalar_v358: self.scalar_v358,
            scalar_v360: self.scalar_v360,
            scalar_v366: self.scalar_v366,
            scalar_v380: self.scalar_v380,
            scalar_v385: self.scalar_v385,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v421: self.scalar_v421,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v435: self.scalar_v435,
            scalar_v436: self.scalar_v436,
            scalar_v437: self.scalar_v437,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v441: self.scalar_v441,
            scalar_v442: self.scalar_v442,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v465: self.scalar_v465,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v478: self.scalar_v478,
            scalar_v479: self.scalar_v479,
            scalar_v480: self.scalar_v480,
            scalar_v484: self.scalar_v484,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v491: self.scalar_v491,
            scalar_v495: self.scalar_v495,
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
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v511: self.scalar_v511,
            scalar_v512: self.scalar_v512,
            scalar_v513: self.scalar_v513,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v520: self.scalar_v520,
            scalar_v523: self.scalar_v523,
            scalar_v524: self.scalar_v524,
            scalar_v525: self.scalar_v525,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v532: self.scalar_v532,
            scalar_v534: self.scalar_v534,
            scalar_v535: self.scalar_v535,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v562: self.scalar_v562,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v565: self.scalar_v565,
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
            scalar_v577: self.scalar_v577,
            scalar_v579: self.scalar_v579,
            scalar_v580: self.scalar_v580,
            scalar_v581: self.scalar_v581,
            scalar_v582: self.scalar_v582,
            scalar_v583: self.scalar_v583,
            scalar_v584: self.scalar_v584,
            scalar_v590: self.scalar_v590,
            scalar_v591: self.scalar_v591,
            scalar_v592: self.scalar_v592,
            scalar_v595: self.scalar_v595,
            scalar_v596: self.scalar_v596,
            scalar_v597: self.scalar_v597,
            scalar_v599: self.scalar_v599,
            scalar_v600: self.scalar_v600,
            scalar_v601: self.scalar_v601,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v606: self.scalar_v606,
            scalar_v609: self.scalar_v609,
            scalar_v610: self.scalar_v610,
            scalar_v611: self.scalar_v611,
            scalar_v614: self.scalar_v614,
            scalar_v615: self.scalar_v615,
            scalar_v616: self.scalar_v616,
            scalar_v696: self.scalar_v696,
            scalar_v736: self.scalar_v736,
            scalar_v779: self.scalar_v779,
            scalar_v857: self.scalar_v857,
            scalar_v901: self.scalar_v901,
            scalar_v987: self.scalar_v987,
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
            scalar_v23: 0.0,
            scalar_v25: false,
            scalar_v28: 0.0,
            scalar_v30: 0.0,
            scalar_v31: false,
            scalar_v33: 0.0,
            scalar_v35: 0.0,
            scalar_v36: 0.0,
            scalar_v38: 0.0,
            scalar_v40: 0.0,
            scalar_v42: 0.0,
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
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
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
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
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
            scalar_v103: false,
            scalar_v104: 0.0,
            scalar_v105: false,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: false,
            scalar_v110: 0.0,
            scalar_v111: false,
            scalar_v112: 0.0,
            scalar_v113: false,
            scalar_v114: 0.0,
            scalar_v115: false,
            scalar_v116: false,
            scalar_v117: 0.0,
            scalar_v118: false,
            scalar_v119: false,
            scalar_v120: false,
            scalar_v121: 0.0,
            scalar_v122: 0.0,
            scalar_v151: 0.0,
            scalar_v152: false,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v166: 0.0,
            scalar_v189: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v198: 0.0,
            scalar_v199: false,
            scalar_v200: false,
            scalar_v204: false,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v211: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
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
            scalar_v230: 0.0,
            scalar_v248: 0.0,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v257: 0.0,
            scalar_v258: false,
            scalar_v259: false,
            scalar_v263: false,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v273: 0.0,
            scalar_v274: false,
            scalar_v277: false,
            scalar_v278: false,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: false,
            scalar_v287: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v314: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: 0.0,
            scalar_v324: false,
            scalar_v325: false,
            scalar_v329: false,
            scalar_v333: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v343: 0.0,
            scalar_v344: false,
            scalar_v352: 0.0,
            scalar_v353: false,
            scalar_v354: false,
            scalar_v355: false,
            scalar_v356: false,
            scalar_v372: false,
            scalar_v373: false,
            scalar_v374: false,
            scalar_v375: false,
            scalar_v376: false,
            scalar_v391: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v401: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v405: 0.0,
            scalar_v406: 0.0,
            scalar_v407: 0.0,
            scalar_v422: 0.0,
            scalar_v427: 0.0,
            scalar_v428: 0.0,
            scalar_v429: false,
            scalar_v430: false,
            scalar_v438: 0.0,
            scalar_v443: 0.0,
            scalar_v444: false,
            scalar_v445: false,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v456: 0.0,
            scalar_v474: 0.0,
            scalar_v482: 0.0,
            scalar_v483: false,
            scalar_v487: false,
            scalar_v488: false,
            scalar_v492: 0.0,
            scalar_v493: false,
            scalar_v494: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v521: false,
            scalar_v522: false,
            scalar_v526: false,
            scalar_v530: 0.0,
            scalar_v531: 0.0,
            scalar_v533: 0.0,
            scalar_v538: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v547: 0.0,
            scalar_v548: false,
            scalar_v549: 0.0,
            scalar_v550: false,
            scalar_v551: false,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v554: 0.0,
            scalar_v555: 0.0,
            scalar_v556: 0.0,
            scalar_v557: 0.0,
            scalar_v558: 0.0,
            scalar_v559: 0.0,
            scalar_v560: 0.0,
            scalar_v561: 0.0,
            scalar_v578: 0.0,
            scalar_v585: 0.0,
            scalar_v586: 0.0,
            scalar_v587: 0.0,
            scalar_v588: false,
            scalar_v589: false,
            scalar_v593: false,
            scalar_v594: false,
            scalar_v598: false,
            scalar_v602: 0.0,
            scalar_v603: 0.0,
            scalar_v607: 0.0,
            scalar_v608: 0.0,
            scalar_v612: 0.0,
            scalar_v613: 0.0,
            scalar_v617: 0.0,
            scalar_v618: 0.0,
            scalar_v619: false,
            scalar_v620: 0.0,
            scalar_v621: false,
            scalar_v622: false,
            scalar_v623: false,
            scalar_v624: false,
            scalar_v667: false,
            scalar_v668: 0.0,
            scalar_v697: false,
            scalar_v701: false,
            scalar_v711: false,
            scalar_v712: 0.0,
            scalar_v737: false,
            scalar_v741: false,
            scalar_v745: false,
            scalar_v755: false,
            scalar_v756: 0.0,
            scalar_v780: false,
            scalar_v784: false,
            scalar_v834: false,
            scalar_v835: 0.0,
            scalar_v858: false,
            scalar_v862: false,
            scalar_v876: false,
            scalar_v877: 0.0,
            scalar_v902: false,
            scalar_v906: false,
            scalar_v910: 0.0,
            scalar_v911: false,
            scalar_v912: false,
            scalar_v913: 0.0,
            scalar_v937: false,
            scalar_v941: false,
            scalar_v945: 0.0,
            scalar_v960: false,
            scalar_v961: false,
            scalar_v962: 0.0,
            scalar_v985: 0.0,
            scalar_v986: 0.0,
            scalar_v988: false,
            scalar_v989: false,
            scalar_v993: false,
            scalar_v997: false,
            scalar_v1013: false,
            scalar_v1014: 0.0,
            scalar_v1028: false,
            scalar_v1069: 0.0,
            scalar_v1079: 0.0,
            scalar_v1088: 0.0,
            scalar_v1090: false,
            scalar_v1093: 0.0,
            scalar_v1164: 0.0,
            scalar_v1168: 0.0,
            scalar_v1187: false,
            scalar_v1219: false,
            scalar_v1221: false,
            scalar_v1222: false,
            scalar_v1223: 0.0,
            scalar_v1236: false,
            scalar_v1237: 0.0,
            scalar_v1255: false,
            scalar_v1257: false,
            scalar_v1275: false,
            scalar_v1304: 0.0,
            scalar_v1314: 0.0,
            scalar_v1333: 0.0,
            scalar_v1334: 0.0,
            scalar_v1356: 0.0,
            scalar_v1357: 0.0,
            scalar_v1376: 0.0,
            scalar_v1377: false,
            scalar_v1380: 0.0,
            scalar_v1449: 0.0,
            scalar_v1480: false,
            scalar_v1513: false,
            scalar_v1514: 0.0,
            scalar_v1532: false,
            scalar_v1657: 0.0,
            scalar_v1658: false,
            scalar_v1661: 0.0,
            scalar_v1730: 0.0,
            scalar_v1761: false,
            scalar_v1794: 0.0,
            scalar_v1795: false,
            scalar_v1797: false,
            scalar_v1799: 0.0,
            scalar_v1868: 0.0,
            scalar_v1899: false,
            scalar_v1900: false,
            scalar_v1935: false,
            scalar_v1936: 0.0,
            scalar_v1948: false,
            scalar_v1949: false,
            scalar_v1953: false,
            scalar_v1954: false,
            scalar_v1956: false,
            scalar_v1959: false,
            scalar_v1960: 0.0,
            scalar_v1978: false,
            scalar_v1980: false,
            scalar_v1997: false,
            scalar_v2001: false,
            scalar_v2002: false,
            scalar_v2003: false,
            scalar_v2004: false,
            scalar_v2005: false,
            scalar_v2006: false,
            scalar_v2007: false,
            scalar_v2008: false,
            scalar_v2009: false,
            scalar_v2010: false,
            scalar_v2011: false,
            scalar_v2012: false,
            scalar_v2013: 0.0,
            scalar_v2014: false,
            scalar_v2015: false,
            scalar_v2016: false,
            scalar_v2017: false,
            scalar_v2018: false,
            scalar_v2019: false,
            scalar_v2025: 0.0,
            scalar_v2026: false,
            scalar_v2027: false,
            scalar_v2028: false,
            scalar_v2029: false,
            scalar_v2030: false,
            scalar_v2031: 0.0,
            scalar_v2034: 0.0,
            scalar_v2035: false,
            scalar_v2036: 0.0,
            scalar_v2039: false,
            scalar_v2053: false,
            scalar_v2054: 0.0,
            scalar_v2055: false,
            scalar_v2056: 0.0,
            scalar_v2060: false,
            scalar_v2061: 0.0,
            scalar_v2065: 0.0,
            scalar_v2068: false,
            scalar_v2073: false,
            scalar_v2075: false,
            scalar_v2082: false,
            scalar_v2083: 0.0,
            scalar_v2084: false,
            scalar_v2085: 0.0,
            scalar_v2094: false,
            scalar_v2097: 0.0,
            scalar_v5912: 0.0,
            scalar_v5913: 0.0,
            scalar_v6058: 0.0,
            scalar_v6059: 0.0,
            scalar_v6060: 0.0,
            scalar_v6061: 0.0,
            scalar_v6062: 0.0,
            scalar_v6063: 0.0,
            scalar_v6064: 0.0,
            scalar_v6065: 0.0,
            scalar_v6066: 0.0,
            scalar_v6067: 0.0,
            scalar_v6068: 0.0,
            scalar_v6069: 0.0,
            scalar_v6070: 0.0,
            scalar_v6123: 0.0,
            scalar_v6130: 0.0,
            scalar_v6149: 0.0,
            scalar_v6150: 0.0,
            scalar_v6151: 0.0,
            scalar_v6169: 0.0,
            scalar_v6170: 0.0,
            scalar_v6177: 0.0,
            scalar_v6178: 0.0,
            scalar_v6195: 0.0,
            scalar_v6196: 0.0,
            scalar_v6197: 0.0,
            scalar_v6198: 0.0,
            scalar_v6199: 0.0,
            scalar_v6200: 0.0,
            scalar_v123: 0.0,
            scalar_v125: false,
            scalar_v126: 0.0,
            scalar_v128: false,
            scalar_v129: false,
            scalar_v130: false,
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
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v210: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v219: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v237: 0.0,
            scalar_v238: 0.0,
            scalar_v239: 0.0,
            scalar_v240: 0.0,
            scalar_v241: 0.0,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v244: 0.0,
            scalar_v245: 0.0,
            scalar_v246: 0.0,
            scalar_v247: 0.0,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v260: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v266: 0.0,
            scalar_v268: 0.0,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v280: 0.0,
            scalar_v282: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v334: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v342: 0.0,
            scalar_v350: 0.0,
            scalar_v358: 0.0,
            scalar_v360: 0.0,
            scalar_v366: 0.0,
            scalar_v380: 0.0,
            scalar_v385: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v421: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v435: 0.0,
            scalar_v436: 0.0,
            scalar_v437: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v441: 0.0,
            scalar_v442: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v465: 0.0,
            scalar_v466: 0.0,
            scalar_v467: 0.0,
            scalar_v468: 0.0,
            scalar_v469: 0.0,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v473: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v478: 0.0,
            scalar_v479: 0.0,
            scalar_v480: 0.0,
            scalar_v484: 0.0,
            scalar_v485: 0.0,
            scalar_v486: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v491: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
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
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v513: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v520: 0.0,
            scalar_v523: 0.0,
            scalar_v524: 0.0,
            scalar_v525: 0.0,
            scalar_v527: 0.0,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v532: 0.0,
            scalar_v534: 0.0,
            scalar_v535: 0.0,
            scalar_v536: 0.0,
            scalar_v537: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v544: 0.0,
            scalar_v545: 0.0,
            scalar_v546: 0.0,
            scalar_v562: 0.0,
            scalar_v563: 0.0,
            scalar_v564: 0.0,
            scalar_v565: 0.0,
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
            scalar_v577: 0.0,
            scalar_v579: 0.0,
            scalar_v580: 0.0,
            scalar_v581: 0.0,
            scalar_v582: 0.0,
            scalar_v583: 0.0,
            scalar_v584: 0.0,
            scalar_v590: 0.0,
            scalar_v591: 0.0,
            scalar_v592: 0.0,
            scalar_v595: 0.0,
            scalar_v596: 0.0,
            scalar_v597: 0.0,
            scalar_v599: 0.0,
            scalar_v600: 0.0,
            scalar_v601: 0.0,
            scalar_v604: 0.0,
            scalar_v605: 0.0,
            scalar_v606: 0.0,
            scalar_v609: 0.0,
            scalar_v610: 0.0,
            scalar_v611: 0.0,
            scalar_v614: 0.0,
            scalar_v615: 0.0,
            scalar_v616: 0.0,
            scalar_v696: 0.0,
            scalar_v736: 0.0,
            scalar_v779: 0.0,
            scalar_v857: 0.0,
            scalar_v901: 0.0,
            scalar_v987: 0.0,
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
            scalar_v23,
            scalar_v25,
            scalar_v28,
            scalar_v30,
            scalar_v31,
            scalar_v33,
            scalar_v35,
            scalar_v36,
            scalar_v38,
            scalar_v40,
            scalar_v42,
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
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
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
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
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
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v189,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v216,
            scalar_v217,
            scalar_v218,
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
            scalar_v248,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v314,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v329,
            scalar_v333,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v343,
            scalar_v344,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v391,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v422,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v438,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v474,
            scalar_v482,
            scalar_v483,
            scalar_v487,
            scalar_v488,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v521,
            scalar_v522,
            scalar_v526,
            scalar_v530,
            scalar_v531,
            scalar_v533,
            scalar_v538,
            scalar_v542,
            scalar_v543,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v556,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v578,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v589,
            scalar_v593,
            scalar_v594,
            scalar_v598,
            scalar_v602,
            scalar_v603,
            scalar_v607,
            scalar_v608,
            scalar_v612,
            scalar_v613,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v667,
            scalar_v668,
            scalar_v697,
            scalar_v701,
            scalar_v711,
            scalar_v712,
            scalar_v737,
            scalar_v741,
            scalar_v745,
            scalar_v755,
            scalar_v756,
            scalar_v780,
            scalar_v784,
            scalar_v834,
            scalar_v835,
            scalar_v858,
            scalar_v862,
            scalar_v876,
            scalar_v877,
            scalar_v902,
            scalar_v906,
            scalar_v910,
            scalar_v911,
            scalar_v912,
            scalar_v913,
            scalar_v937,
            scalar_v941,
            scalar_v945,
            scalar_v960,
            scalar_v961,
            scalar_v962,
            scalar_v985,
            scalar_v986,
            scalar_v988,
            scalar_v989,
            scalar_v993,
            scalar_v997,
            scalar_v1013,
            scalar_v1014,
            scalar_v1028,
            scalar_v1069,
            scalar_v1079,
            scalar_v1088,
            scalar_v1090,
            scalar_v1093,
            scalar_v1164,
            scalar_v1168,
            scalar_v1187,
            scalar_v1219,
            scalar_v1221,
            scalar_v1222,
            scalar_v1223,
            scalar_v1236,
            scalar_v1237,
            scalar_v1255,
            scalar_v1257,
            scalar_v1275,
            scalar_v1304,
            scalar_v1314,
            scalar_v1333,
            scalar_v1334,
            scalar_v1356,
            scalar_v1357,
            scalar_v1376,
            scalar_v1377,
            scalar_v1380,
            scalar_v1449,
            scalar_v1480,
            scalar_v1513,
            scalar_v1514,
            scalar_v1532,
            scalar_v1657,
            scalar_v1658,
            scalar_v1661,
            scalar_v1730,
            scalar_v1761,
            scalar_v1794,
            scalar_v1795,
            scalar_v1797,
            scalar_v1799,
            scalar_v1868,
            scalar_v1899,
            scalar_v1900,
            scalar_v1935,
            scalar_v1936,
            scalar_v1948,
            scalar_v1949,
            scalar_v1953,
            scalar_v1954,
            scalar_v1956,
            scalar_v1959,
            scalar_v1960,
            scalar_v1978,
            scalar_v1980,
            scalar_v1997,
            scalar_v2001,
            scalar_v2002,
            scalar_v2003,
            scalar_v2004,
            scalar_v2005,
            scalar_v2006,
            scalar_v2007,
            scalar_v2008,
            scalar_v2009,
            scalar_v2010,
            scalar_v2011,
            scalar_v2012,
            scalar_v2013,
            scalar_v2014,
            scalar_v2015,
            scalar_v2016,
            scalar_v2017,
            scalar_v2018,
            scalar_v2019,
            scalar_v2025,
            scalar_v2026,
            scalar_v2027,
            scalar_v2028,
            scalar_v2029,
            scalar_v2030,
            scalar_v2031,
            scalar_v2034,
            scalar_v2035,
            scalar_v2036,
            scalar_v2039,
            scalar_v2053,
            scalar_v2054,
            scalar_v2055,
            scalar_v2056,
            scalar_v2060,
            scalar_v2061,
            scalar_v2065,
            scalar_v2068,
            scalar_v2073,
            scalar_v2075,
            scalar_v2082,
            scalar_v2083,
            scalar_v2084,
            scalar_v2085,
            scalar_v2094,
            scalar_v2097,
            scalar_v5912,
            scalar_v5913,
            scalar_v6058,
            scalar_v6059,
            scalar_v6060,
            scalar_v6061,
            scalar_v6062,
            scalar_v6063,
            scalar_v6064,
            scalar_v6065,
            scalar_v6066,
            scalar_v6067,
            scalar_v6068,
            scalar_v6069,
            scalar_v6070,
            scalar_v6123,
            scalar_v6130,
            scalar_v6149,
            scalar_v6150,
            scalar_v6151,
            scalar_v6169,
            scalar_v6170,
            scalar_v6177,
            scalar_v6178,
            scalar_v6195,
            scalar_v6196,
            scalar_v6197,
            scalar_v6198,
            scalar_v6199,
            scalar_v6200,
            scalar_v123,
            scalar_v125,
            scalar_v126,
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
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
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
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v210,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v219,
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
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v268,
            scalar_v271,
            scalar_v272,
            scalar_v280,
            scalar_v282,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v334,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v350,
            scalar_v358,
            scalar_v360,
            scalar_v366,
            scalar_v380,
            scalar_v385,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v442,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v495,
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
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v520,
            scalar_v523,
            scalar_v524,
            scalar_v525,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v532,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v565,
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
            scalar_v577,
            scalar_v579,
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v583,
            scalar_v584,
            scalar_v590,
            scalar_v591,
            scalar_v592,
            scalar_v595,
            scalar_v596,
            scalar_v597,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v609,
            scalar_v610,
            scalar_v611,
            scalar_v614,
            scalar_v615,
            scalar_v616,
            scalar_v696,
            scalar_v736,
            scalar_v779,
            scalar_v857,
            scalar_v901,
            scalar_v987,
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
            scalar_v23,
            scalar_v25,
            scalar_v28,
            scalar_v30,
            scalar_v31,
            scalar_v33,
            scalar_v35,
            scalar_v36,
            scalar_v38,
            scalar_v40,
            scalar_v42,
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
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
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
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
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
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v189,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v216,
            scalar_v217,
            scalar_v218,
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
            scalar_v248,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v263,
            scalar_v269,
            scalar_v270,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v314,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v329,
            scalar_v333,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v343,
            scalar_v344,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v391,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v422,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v438,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v474,
            scalar_v482,
            scalar_v483,
            scalar_v487,
            scalar_v488,
            scalar_v492,
            scalar_v493,
            scalar_v494,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v521,
            scalar_v522,
            scalar_v526,
            scalar_v530,
            scalar_v531,
            scalar_v533,
            scalar_v538,
            scalar_v542,
            scalar_v543,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v555,
            scalar_v556,
            scalar_v557,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v561,
            scalar_v578,
            scalar_v585,
            scalar_v586,
            scalar_v587,
            scalar_v588,
            scalar_v589,
            scalar_v593,
            scalar_v594,
            scalar_v598,
            scalar_v602,
            scalar_v603,
            scalar_v607,
            scalar_v608,
            scalar_v612,
            scalar_v613,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v667,
            scalar_v668,
            scalar_v697,
            scalar_v701,
            scalar_v711,
            scalar_v712,
            scalar_v737,
            scalar_v741,
            scalar_v745,
            scalar_v755,
            scalar_v756,
            scalar_v780,
            scalar_v784,
            scalar_v834,
            scalar_v835,
            scalar_v858,
            scalar_v862,
            scalar_v876,
            scalar_v877,
            scalar_v902,
            scalar_v906,
            scalar_v910,
            scalar_v911,
            scalar_v912,
            scalar_v913,
            scalar_v937,
            scalar_v941,
            scalar_v945,
            scalar_v960,
            scalar_v961,
            scalar_v962,
            scalar_v985,
            scalar_v986,
            scalar_v988,
            scalar_v989,
            scalar_v993,
            scalar_v997,
            scalar_v1013,
            scalar_v1014,
            scalar_v1028,
            scalar_v1069,
            scalar_v1079,
            scalar_v1088,
            scalar_v1090,
            scalar_v1093,
            scalar_v1164,
            scalar_v1168,
            scalar_v1187,
            scalar_v1219,
            scalar_v1221,
            scalar_v1222,
            scalar_v1223,
            scalar_v1236,
            scalar_v1237,
            scalar_v1255,
            scalar_v1257,
            scalar_v1275,
            scalar_v1304,
            scalar_v1314,
            scalar_v1333,
            scalar_v1334,
            scalar_v1356,
            scalar_v1357,
            scalar_v1376,
            scalar_v1377,
            scalar_v1380,
            scalar_v1449,
            scalar_v1480,
            scalar_v1513,
            scalar_v1514,
            scalar_v1532,
            scalar_v1657,
            scalar_v1658,
            scalar_v1661,
            scalar_v1730,
            scalar_v1761,
            scalar_v1794,
            scalar_v1795,
            scalar_v1797,
            scalar_v1799,
            scalar_v1868,
            scalar_v1899,
            scalar_v1900,
            scalar_v1935,
            scalar_v1936,
            scalar_v1948,
            scalar_v1949,
            scalar_v1953,
            scalar_v1954,
            scalar_v1956,
            scalar_v1959,
            scalar_v1960,
            scalar_v1978,
            scalar_v1980,
            scalar_v1997,
            scalar_v2001,
            scalar_v2002,
            scalar_v2003,
            scalar_v2004,
            scalar_v2005,
            scalar_v2006,
            scalar_v2007,
            scalar_v2008,
            scalar_v2009,
            scalar_v2010,
            scalar_v2011,
            scalar_v2012,
            scalar_v2013,
            scalar_v2014,
            scalar_v2015,
            scalar_v2016,
            scalar_v2017,
            scalar_v2018,
            scalar_v2019,
            scalar_v2025,
            scalar_v2026,
            scalar_v2027,
            scalar_v2028,
            scalar_v2029,
            scalar_v2030,
            scalar_v2031,
            scalar_v2034,
            scalar_v2035,
            scalar_v2036,
            scalar_v2039,
            scalar_v2053,
            scalar_v2054,
            scalar_v2055,
            scalar_v2056,
            scalar_v2060,
            scalar_v2061,
            scalar_v2065,
            scalar_v2068,
            scalar_v2073,
            scalar_v2075,
            scalar_v2082,
            scalar_v2083,
            scalar_v2084,
            scalar_v2085,
            scalar_v2094,
            scalar_v2097,
            scalar_v5912,
            scalar_v5913,
            scalar_v6058,
            scalar_v6059,
            scalar_v6060,
            scalar_v6061,
            scalar_v6062,
            scalar_v6063,
            scalar_v6064,
            scalar_v6065,
            scalar_v6066,
            scalar_v6067,
            scalar_v6068,
            scalar_v6069,
            scalar_v6070,
            scalar_v6123,
            scalar_v6130,
            scalar_v6149,
            scalar_v6150,
            scalar_v6151,
            scalar_v6169,
            scalar_v6170,
            scalar_v6177,
            scalar_v6178,
            scalar_v6195,
            scalar_v6196,
            scalar_v6197,
            scalar_v6198,
            scalar_v6199,
            scalar_v6200,
            scalar_v123,
            scalar_v125,
            scalar_v126,
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
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
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
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v210,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v219,
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
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v268,
            scalar_v271,
            scalar_v272,
            scalar_v280,
            scalar_v282,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v334,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v350,
            scalar_v358,
            scalar_v360,
            scalar_v366,
            scalar_v380,
            scalar_v385,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v435,
            scalar_v436,
            scalar_v437,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v442,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v495,
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
            scalar_v509,
            scalar_v510,
            scalar_v511,
            scalar_v512,
            scalar_v513,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v520,
            scalar_v523,
            scalar_v524,
            scalar_v525,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v532,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v562,
            scalar_v563,
            scalar_v564,
            scalar_v565,
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
            scalar_v577,
            scalar_v579,
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v583,
            scalar_v584,
            scalar_v590,
            scalar_v591,
            scalar_v592,
            scalar_v595,
            scalar_v596,
            scalar_v597,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v609,
            scalar_v610,
            scalar_v611,
            scalar_v614,
            scalar_v615,
            scalar_v616,
            scalar_v696,
            scalar_v736,
            scalar_v779,
            scalar_v857,
            scalar_v901,
            scalar_v987,
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
        let v23: f64 = p.p0;
        self.scalar_v23 = v23;
        let v25: bool = (p.p0 <= 310.0);
        self.scalar_v25 = v25;
        let v28: f64 = (if v25 { 1.6021918e-19 } else { 0.0 });
        self.scalar_v28 = v28;
        let v30: f64 = (if v25 { 1.3806226e-23 } else { 0.0 });
        self.scalar_v30 = v30;
        let v31: bool = (!v25);
        self.scalar_v31 = v31;
        let v33: f64 = (if v31 { 1.602176634e-19 } else { v28 });
        self.scalar_v33 = v33;
        let v35: f64 = (if v31 { 1.380649e-23 } else { v30 });
        self.scalar_v35 = v35;
        let v36: f64 = p.p146;
        self.scalar_v36 = v36;
        let v38: f64 = (p.p146 + 273.15);
        self.scalar_v38 = v38;
        let v40: f64 = (v35 / v33);
        self.scalar_v40 = v40;
        let v42: f64 = (v38 * v40);
        self.scalar_v42 = v42;
        let v44: f64 = (1.0 / v42);
        self.scalar_v44 = v44;
        let v45: f64 = p.p121;
        self.scalar_v45 = v45;
        let v46: f64 = (v38 * p.p121);
        self.scalar_v46 = v46;
        let v47: f64 = ((v38) as f64).ln();
        self.scalar_v47 = v47;
        let v48: f64 = (v46 * v47);
        self.scalar_v48 = v48;
        let v49: f64 = p.p122;
        self.scalar_v49 = v49;
        let v50: f64 = (v38 * p.p122);
        self.scalar_v50 = v50;
        let v51: f64 = p.p117;
        self.scalar_v51 = v51;
        let v52: f64 = (v48 + p.p117);
        self.scalar_v52 = v52;
        let v53: f64 = (v50 + v52);
        self.scalar_v53 = v53;
        let v54: f64 = p.p118;
        self.scalar_v54 = v54;
        let v55: f64 = (v48 + p.p118);
        self.scalar_v55 = v55;
        let v56: f64 = (v50 + v55);
        self.scalar_v56 = v56;
        let v57: f64 = p.p119;
        self.scalar_v57 = v57;
        let v58: f64 = (v48 + p.p119);
        self.scalar_v58 = v58;
        let v59: f64 = (v50 + v58);
        self.scalar_v59 = v59;
        let v60: f64 = (v53 + v56);
        self.scalar_v60 = v60;
        let v62: f64 = (v60 * 0.5);
        self.scalar_v62 = v62;
        let v63: f64 = (v53 + v59);
        self.scalar_v63 = v63;
        let v64: f64 = (0.5 * v63);
        self.scalar_v64 = v64;
        let v65: f64 = (p.p117 + p.p118);
        self.scalar_v65 = v65;
        let v66: f64 = (0.5 * v65);
        self.scalar_v66 = v66;
        let v67: f64 = (p.p117 + p.p119);
        self.scalar_v67 = v67;
        let v68: f64 = (0.5 * v67);
        self.scalar_v68 = v68;
        let v69: f64 = p.p120;
        self.scalar_v69 = v69;
        let v70: f64 = (p.p119 + p.p120);
        self.scalar_v70 = v70;
        let v71: f64 = (0.5 * v70);
        self.scalar_v71 = v71;
        let v73: f64 = (p.p121 / v40);
        self.scalar_v73 = v73;
        let v74: f64 = (3.0 - v73);
        self.scalar_v74 = v74;
        let v75: f64 = (1.0 + v74);
        self.scalar_v75 = v75;
        let v76: f64 = p.p138;
        self.scalar_v76 = v76;
        let v77: f64 = (v75 - p.p138);
        self.scalar_v77 = v77;
        let v79: f64 = (v74 - 1.5);
        self.scalar_v79 = v79;
        let v80: f64 = p.p107;
        self.scalar_v80 = v80;
        let v81: f64 = (1.0 - p.p107);
        self.scalar_v81 = v81;
        let v82: f64 = p.p52;
        self.scalar_v82 = v82;
        let v83: f64 = p.p106;
        self.scalar_v83 = v83;
        let v84: f64 = (p.p52 + p.p106);
        self.scalar_v84 = v84;
        let v85: f64 = (v81 * v84);
        self.scalar_v85 = v85;
        let v86: bool = (v85 >= p.p106);
        self.scalar_v86 = v86;
        let v87: f64 = (if v86 { p.p106 } else { 0.0 });
        self.scalar_v87 = v87;
        let v88: f64 = (if v86 { 0.0 } else { 0.0 });
        self.scalar_v88 = v88;
        let v89: f64 = (v85 - p.p106);
        self.scalar_v89 = v89;
        let v90: f64 = (if v86 { v89 } else { 0.0 });
        self.scalar_v90 = v90;
        let v91: f64 = (p.p52 - v90);
        self.scalar_v91 = v91;
        let v92: f64 = (if v86 { v91 } else { 0.0 });
        self.scalar_v92 = v92;
        let v93: bool = (!v86);
        self.scalar_v93 = v93;
        let v94: f64 = (if v93 { v85 } else { v87 });
        self.scalar_v94 = v94;
        let v95: f64 = (p.p106 - v94);
        self.scalar_v95 = v95;
        let v96: f64 = (if v93 { v95 } else { v88 });
        self.scalar_v96 = v96;
        let v97: f64 = (if v93 { 0.0 } else { v90 });
        self.scalar_v97 = v97;
        let v98: f64 = (if v93 { p.p52 } else { v92 });
        self.scalar_v98 = v98;
        let v99: f64 = p.p105;
        self.scalar_v99 = v99;
        let v100: f64 = p.p104;
        self.scalar_v100 = v100;
        let v101: f64 = (p.p105 * p.p104);
        self.scalar_v101 = v101;
        let v102: f64 = (p.p104 - v101);
        self.scalar_v102 = v102;
        let v103: bool = (p.p0 <= 300.0);
        self.scalar_v103 = v103;
        let v104: f64 = (if v103 { 0.0 } else { 0.0 });
        self.scalar_v104 = v104;
        let v105: bool = (!v103);
        self.scalar_v105 = v105;
        let v107: f64 = (if v105 { 0.7 } else { v104 });
        self.scalar_v107 = v107;
        let v108: f64 = p.p47;
        self.scalar_v108 = v108;
        let v109: bool = (p.p47 > 0.0);
        self.scalar_v109 = v109;
        let v110: f64 = p.p86;
        self.scalar_v110 = v110;
        let v111: bool = (0.0 != p.p86);
        self.scalar_v111 = v111;
        let v112: f64 = p.p88;
        self.scalar_v112 = v112;
        let v113: bool = (0.0 == p.p88);
        self.scalar_v113 = v113;
        let v114: f64 = p.p87;
        self.scalar_v114 = v114;
        let v115: bool = (0.0 == p.p87);
        self.scalar_v115 = v115;
        let v116: bool = (v113 && v115);
        self.scalar_v116 = v116;
        let v117: f64 = p.p66;
        self.scalar_v117 = v117;
        let v118: bool = (0.0 == p.p66);
        self.scalar_v118 = v118;
        let v119: bool = (v116 || v118);
        self.scalar_v119 = v119;
        let v120: bool = (v111 && v119);
        self.scalar_v120 = v120;
        let v121: f64 = (if v120 { 0.0 } else { p.p86 });
        self.scalar_v121 = v121;
        let v122: f64 = p.p147;
        self.scalar_v122 = v122;
        let v151: f64 = p.p39;
        self.scalar_v151 = v151;
        let v152: bool = (p.p39 > 0.0);
        self.scalar_v152 = v152;
        let v154: f64 = (v42 * 2.0);
        self.scalar_v154 = v154;
        let v155: f64 = p.p40;
        self.scalar_v155 = v155;
        let v156: f64 = (0.5 * p.p40);
        self.scalar_v156 = v156;
        let v157: f64 = (v44 * v156);
        self.scalar_v157 = v157;
        let v158: f64 = ((v157) as f64).exp();
        self.scalar_v158 = v158;
        let v160: f64 = (p.p40 * -0.5);
        self.scalar_v160 = v160;
        let v161: f64 = (v44 * v160);
        self.scalar_v161 = v161;
        let v162: f64 = ((v161) as f64).exp();
        self.scalar_v162 = v162;
        let v163: f64 = (v158 - v162);
        self.scalar_v163 = v163;
        let v164: f64 = ((v163) as f64).ln();
        self.scalar_v164 = v164;
        let v165: f64 = (v154 * v164);
        self.scalar_v165 = v165;
        let v166: f64 = (if v152 { v165 } else { 0.0 });
        self.scalar_v166 = v166;
        let v189: f64 = p.p41;
        self.scalar_v189 = v189;
        let v196: f64 = p.p42;
        self.scalar_v196 = v196;
        let v197: f64 = ((p.p42) as f64).abs();
        self.scalar_v197 = v197;
        let v198: f64 = (if v152 { v197 } else { 0.0 });
        self.scalar_v198 = v198;
        let v199: bool = (p.p42 > 0.0);
        self.scalar_v199 = v199;
        let v200: bool = (v152 && v199);
        self.scalar_v200 = v200;
        let v204: bool = (!v152);
        self.scalar_v204 = v204;
        let v208: f64 = p.p14;
        self.scalar_v208 = v208;
        let v209: f64 = p.p124;
        self.scalar_v209 = v209;
        let v211: f64 = (v44 * p.p118);
        self.scalar_v211 = v211;
        let v216: f64 = p.p16;
        self.scalar_v216 = v216;
        let v217: f64 = p.p17;
        self.scalar_v217 = v217;
        let v218: f64 = (v44 * v66);
        self.scalar_v218 = v218;
        let v220: f64 = p.p48;
        self.scalar_v220 = v220;
        let v221: f64 = (0.5 * p.p48);
        self.scalar_v221 = v221;
        let v222: f64 = (v44 * v221);
        self.scalar_v222 = v222;
        let v223: f64 = ((v222) as f64).exp();
        self.scalar_v223 = v223;
        let v224: f64 = (-0.5 * p.p48);
        self.scalar_v224 = v224;
        let v225: f64 = (v44 * v224);
        self.scalar_v225 = v225;
        let v226: f64 = ((v225) as f64).exp();
        self.scalar_v226 = v226;
        let v227: f64 = (v223 - v226);
        self.scalar_v227 = v227;
        let v228: f64 = ((v227) as f64).ln();
        self.scalar_v228 = v228;
        let v229: f64 = (v154 * v228);
        self.scalar_v229 = v229;
        let v230: f64 = (if v109 { v229 } else { v166 });
        self.scalar_v230 = v230;
        let v248: f64 = p.p49;
        self.scalar_v248 = v248;
        let v255: f64 = p.p50;
        self.scalar_v255 = v255;
        let v256: f64 = ((p.p50) as f64).abs();
        self.scalar_v256 = v256;
        let v257: f64 = (if v109 { v256 } else { 0.0 });
        self.scalar_v257 = v257;
        let v258: bool = (p.p50 > 0.0);
        self.scalar_v258 = v258;
        let v259: bool = (v109 && v258);
        self.scalar_v259 = v259;
        let v263: bool = (!v109);
        self.scalar_v263 = v263;
        let v269: f64 = p.p23;
        self.scalar_v269 = v269;
        let v270: f64 = (v44 * p.p119);
        self.scalar_v270 = v270;
        let v273: f64 = p.p37;
        self.scalar_v273 = v273;
        let v274: bool = (p.p37 > 0.0);
        self.scalar_v274 = v274;
        let v277: bool = (p.p48 > 0.0);
        self.scalar_v277 = v277;
        let v278: bool = (v109 && v277);
        self.scalar_v278 = v278;
        let v284: f64 = p.p89;
        self.scalar_v284 = v284;
        let v285: f64 = p.p43;
        self.scalar_v285 = v285;
        let v286: bool = (p.p43 > 0.0);
        self.scalar_v286 = v286;
        let v287: f64 = p.p44;
        self.scalar_v287 = v287;
        let v288: f64 = (0.5 * p.p44);
        self.scalar_v288 = v288;
        let v289: f64 = (v44 * v288);
        self.scalar_v289 = v289;
        let v290: f64 = ((v289) as f64).exp();
        self.scalar_v290 = v290;
        let v291: f64 = (-0.5 * p.p44);
        self.scalar_v291 = v291;
        let v292: f64 = (v44 * v291);
        self.scalar_v292 = v292;
        let v293: f64 = ((v292) as f64).exp();
        self.scalar_v293 = v293;
        let v294: f64 = (v290 - v293);
        self.scalar_v294 = v294;
        let v295: f64 = ((v294) as f64).ln();
        self.scalar_v295 = v295;
        let v296: f64 = (v154 * v295);
        self.scalar_v296 = v296;
        let v297: f64 = (if v286 { v296 } else { v230 });
        self.scalar_v297 = v297;
        let v314: f64 = p.p45;
        self.scalar_v314 = v314;
        let v321: f64 = p.p46;
        self.scalar_v321 = v321;
        let v322: f64 = ((p.p46) as f64).abs();
        self.scalar_v322 = v322;
        let v323: f64 = (if v286 { v322 } else { 0.0 });
        self.scalar_v323 = v323;
        let v324: bool = (p.p46 > 0.0);
        self.scalar_v324 = v324;
        let v325: bool = (v286 && v324);
        self.scalar_v325 = v325;
        let v329: bool = (!v286);
        self.scalar_v329 = v329;
        let v333: f64 = p.p18;
        self.scalar_v333 = v333;
        let v335: f64 = p.p20;
        self.scalar_v335 = v335;
        let v336: f64 = p.p21;
        self.scalar_v336 = v336;
        let v337: f64 = (v74 / p.p21);
        self.scalar_v337 = v337;
        let v343: f64 = p.p27;
        self.scalar_v343 = v343;
        let v344: bool = (p.p27 > 0.0);
        self.scalar_v344 = v344;
        let v352: f64 = p.p29;
        self.scalar_v352 = v352;
        let v353: bool = (1.0 == p.p29);
        self.scalar_v353 = v353;
        let v354: bool = (v286 && v353);
        self.scalar_v354 = v354;
        let v355: bool = (p.p44 > 0.0);
        self.scalar_v355 = v355;
        let v356: bool = (v354 && v355);
        self.scalar_v356 = v356;
        let v372: bool = (0.0 == p.p29);
        self.scalar_v372 = v372;
        let v373: bool = (v152 && v372);
        self.scalar_v373 = v373;
        let v374: bool = (p.p40 > 0.0);
        self.scalar_v374 = v374;
        let v375: bool = (v373 && v374);
        self.scalar_v375 = v375;
        let v376: bool = (!v356);
        self.scalar_v376 = v376;
        let v391: f64 = p.p28;
        self.scalar_v391 = v391;
        let v398: f64 = p.p53;
        self.scalar_v398 = v398;
        let v399: f64 = (0.5 * p.p53);
        self.scalar_v399 = v399;
        let v400: f64 = (v44 * v399);
        self.scalar_v400 = v400;
        let v401: f64 = ((v400) as f64).exp();
        self.scalar_v401 = v401;
        let v402: f64 = (-0.5 * p.p53);
        self.scalar_v402 = v402;
        let v403: f64 = (v44 * v402);
        self.scalar_v403 = v403;
        let v404: f64 = ((v403) as f64).exp();
        self.scalar_v404 = v404;
        let v405: f64 = (v401 - v404);
        self.scalar_v405 = v405;
        let v406: f64 = ((v405) as f64).ln();
        self.scalar_v406 = v406;
        let v407: f64 = (v154 * v406);
        self.scalar_v407 = v407;
        let v422: f64 = p.p54;
        self.scalar_v422 = v422;
        let v427: f64 = p.p55;
        self.scalar_v427 = v427;
        let v428: f64 = ((p.p55) as f64).abs();
        self.scalar_v428 = v428;
        let v429: bool = (p.p55 > 0.0);
        self.scalar_v429 = v429;
        let v430: bool = (true && v429);
        self.scalar_v430 = v430;
        let v438: f64 = p.p25;
        self.scalar_v438 = v438;
        let v443: f64 = p.p57;
        self.scalar_v443 = v443;
        let v444: bool = (p.p57 > 0.0);
        self.scalar_v444 = v444;
        let v445: bool = (v103 && v444);
        self.scalar_v445 = v445;
        let v446: f64 = p.p58;
        self.scalar_v446 = v446;
        let v447: f64 = (0.5 * p.p58);
        self.scalar_v447 = v447;
        let v448: f64 = (v44 * v447);
        self.scalar_v448 = v448;
        let v449: f64 = ((v448) as f64).exp();
        self.scalar_v449 = v449;
        let v450: f64 = (-0.5 * p.p58);
        self.scalar_v450 = v450;
        let v451: f64 = (v44 * v450);
        self.scalar_v451 = v451;
        let v452: f64 = ((v451) as f64).exp();
        self.scalar_v452 = v452;
        let v453: f64 = (v449 - v452);
        self.scalar_v453 = v453;
        let v454: f64 = ((v453) as f64).ln();
        self.scalar_v454 = v454;
        let v455: f64 = (v154 * v454);
        self.scalar_v455 = v455;
        let v456: f64 = (if v445 { v455 } else { v407 });
        self.scalar_v456 = v456;
        let v474: f64 = p.p59;
        self.scalar_v474 = v474;
        let v482: f64 = (if v445 { 2.4 } else { 0.0 });
        self.scalar_v482 = v482;
        let v483: bool = (false && v445);
        self.scalar_v483 = v483;
        let v487: bool = (!v444);
        self.scalar_v487 = v487;
        let v488: bool = (v103 && v487);
        self.scalar_v488 = v488;
        let v492: f64 = (if v103 { 2.4 } else { 0.0 });
        self.scalar_v492 = v492;
        let v493: bool = (v105 && v444);
        self.scalar_v493 = v493;
        let v494: f64 = (if v493 { v455 } else { v456 });
        self.scalar_v494 = v494;
        let v517: f64 = p.p60;
        self.scalar_v517 = v517;
        let v518: f64 = (-p.p60);
        self.scalar_v518 = v518;
        let v519: f64 = ((v518) as f64).abs();
        self.scalar_v519 = v519;
        let v521: bool = (v518 > 0.0);
        self.scalar_v521 = v521;
        let v522: bool = (v493 && v521);
        self.scalar_v522 = v522;
        let v526: bool = (v105 && v487);
        self.scalar_v526 = v526;
        let v530: f64 = (if v105 { p.p60 } else { v492 });
        self.scalar_v530 = v530;
        let v531: f64 = p.p99;
        self.scalar_v531 = v531;
        let v533: f64 = (v44 * p.p120);
        self.scalar_v533 = v533;
        let v538: f64 = p.p97;
        self.scalar_v538 = v538;
        let v542: f64 = p.p101;
        self.scalar_v542 = v542;
        let v543: f64 = (p.p138 - 1.0);
        self.scalar_v543 = v543;
        let v547: f64 = p.p63;
        self.scalar_v547 = v547;
        let v548: bool = (p.p63 > 0.0);
        self.scalar_v548 = v548;
        let v549: f64 = p.p62;
        self.scalar_v549 = v549;
        let v550: bool = (p.p62 > 0.0);
        self.scalar_v550 = v550;
        let v551: bool = (v548 && v550);
        self.scalar_v551 = v551;
        let v552: f64 = (0.5 * p.p63);
        self.scalar_v552 = v552;
        let v553: f64 = (v44 * v552);
        self.scalar_v553 = v553;
        let v554: f64 = ((v553) as f64).exp();
        self.scalar_v554 = v554;
        let v555: f64 = (-0.5 * p.p63);
        self.scalar_v555 = v555;
        let v556: f64 = (v44 * v555);
        self.scalar_v556 = v556;
        let v557: f64 = ((v556) as f64).exp();
        self.scalar_v557 = v557;
        let v558: f64 = (v554 - v557);
        self.scalar_v558 = v558;
        let v559: f64 = ((v558) as f64).ln();
        self.scalar_v559 = v559;
        let v560: f64 = (v154 * v559);
        self.scalar_v560 = v560;
        let v561: f64 = (if v551 { v560 } else { v494 });
        self.scalar_v561 = v561;
        let v578: f64 = p.p64;
        self.scalar_v578 = v578;
        let v585: f64 = (-v530);
        self.scalar_v585 = v585;
        let v586: f64 = ((v585) as f64).abs();
        self.scalar_v586 = v586;
        let v587: f64 = (if v551 { v586 } else { 0.0 });
        self.scalar_v587 = v587;
        let v588: bool = (v585 > 0.0);
        self.scalar_v588 = v588;
        let v589: bool = (v551 && v588);
        self.scalar_v589 = v589;
        let v593: bool = (!v550);
        self.scalar_v593 = v593;
        let v594: bool = (v548 && v593);
        self.scalar_v594 = v594;
        let v598: bool = (!v548);
        self.scalar_v598 = v598;
        let v602: f64 = p.p96;
        self.scalar_v602 = v602;
        let v603: f64 = p.p136;
        self.scalar_v603 = v603;
        let v607: f64 = p.p90;
        self.scalar_v607 = v607;
        let v608: f64 = p.p135;
        self.scalar_v608 = v608;
        let v612: f64 = p.p95;
        self.scalar_v612 = v612;
        let v613: f64 = p.p137;
        self.scalar_v613 = v613;
        let v617: f64 = p.p142;
        self.scalar_v617 = v617;
        let v618: f64 = p.p141;
        self.scalar_v618 = v618;
        let v619: bool = (0.0 != p.p141);
        self.scalar_v619 = v619;
        let v620: f64 = p.p149;
        self.scalar_v620 = v620;
        let v621: bool = (p.p142 >= p.p149);
        self.scalar_v621 = v621;
        let v622: bool = (v619 && v621);
        self.scalar_v622 = v622;
        let v623: bool = (p.p142 > 0.0);
        self.scalar_v623 = v623;
        let v624: bool = (v622 && v623);
        self.scalar_v624 = v624;
        let v667: bool = (v152 && v624);
        self.scalar_v667 = v667;
        let v668: f64 = (if v667 { v165 } else { v561 });
        self.scalar_v668 = v668;
        let v697: bool = (v199 && v667);
        self.scalar_v697 = v697;
        let v701: bool = (v204 && v624);
        self.scalar_v701 = v701;
        let v711: bool = (v109 && v624);
        self.scalar_v711 = v711;
        let v712: f64 = (if v711 { v229 } else { v668 });
        self.scalar_v712 = v712;
        let v737: bool = (v258 && v711);
        self.scalar_v737 = v737;
        let v741: bool = (v263 && v624);
        self.scalar_v741 = v741;
        let v745: bool = (v103 && v624);
        self.scalar_v745 = v745;
        let v755: bool = (v286 && v624);
        self.scalar_v755 = v755;
        let v756: f64 = (if v755 { v296 } else { v712 });
        self.scalar_v756 = v756;
        let v780: bool = (v324 && v755);
        self.scalar_v780 = v780;
        let v784: bool = (v329 && v624);
        self.scalar_v784 = v784;
        let v834: bool = (true && v624);
        self.scalar_v834 = v834;
        let v835: f64 = (if v834 { v407 } else { v756 });
        self.scalar_v835 = v835;
        let v858: bool = (v429 && v834);
        self.scalar_v858 = v858;
        let v862: bool = (false && v624);
        self.scalar_v862 = v862;
        let v876: bool = (v444 && v745);
        self.scalar_v876 = v876;
        let v877: f64 = (if v876 { v455 } else { v835 });
        self.scalar_v877 = v877;
        let v902: bool = (false && v876);
        self.scalar_v902 = v902;
        let v906: bool = (v487 && v745);
        self.scalar_v906 = v906;
        let v910: f64 = (if v745 { 2.4 } else { v530 });
        self.scalar_v910 = v910;
        let v911: bool = (v105 && v624);
        self.scalar_v911 = v911;
        let v912: bool = (v444 && v911);
        self.scalar_v912 = v912;
        let v913: f64 = (if v912 { v455 } else { v877 });
        self.scalar_v913 = v913;
        let v937: bool = (v521 && v912);
        self.scalar_v937 = v937;
        let v941: bool = (v487 && v911);
        self.scalar_v941 = v941;
        let v945: f64 = (if v911 { p.p60 } else { v910 });
        self.scalar_v945 = v945;
        let v960: bool = (v548 && v624);
        self.scalar_v960 = v960;
        let v961: bool = (v550 && v960);
        self.scalar_v961 = v961;
        let v962: f64 = (if v961 { v560 } else { v913 });
        self.scalar_v962 = v962;
        let v985: f64 = (-v945);
        self.scalar_v985 = v985;
        let v986: f64 = ((v985) as f64).abs();
        self.scalar_v986 = v986;
        let v988: bool = (v985 > 0.0);
        self.scalar_v988 = v988;
        let v989: bool = (v961 && v988);
        self.scalar_v989 = v989;
        let v993: bool = (v593 && v960);
        self.scalar_v993 = v993;
        let v997: bool = (v598 && v624);
        self.scalar_v997 = v997;
        let v1013: bool = (p.p14 > 0.0);
        self.scalar_v1013 = v1013;
        let v1014: f64 = p.p15;
        self.scalar_v1014 = v1014;
        let v1028: bool = (p.p16 > 0.0);
        self.scalar_v1028 = v1028;
        let v1069: f64 = (-p.p41);
        self.scalar_v1069 = v1069;
        let v1079: f64 = (1.0 - p.p41);
        self.scalar_v1079 = v1079;
        let v1088: f64 = p.p51;
        self.scalar_v1088 = v1088;
        let v1090: bool = (p.p51 < 100.0);
        self.scalar_v1090 = v1090;
        let v1093: f64 = (p.p49 / 4.0);
        self.scalar_v1093 = v1093;
        let v1164: f64 = (1.0 - p.p49);
        self.scalar_v1164 = v1164;
        let v1168: f64 = (-p.p49);
        self.scalar_v1168 = v1168;
        let v1187: bool = (!v1090);
        self.scalar_v1187 = v1187;
        let v1219: bool = (p.p0 >= 310.0);
        self.scalar_v1219 = v1219;
        let v1221: bool = (p.p0 >= 320.0);
        self.scalar_v1221 = v1221;
        let v1222: bool = (p.p23 > 0.0);
        self.scalar_v1222 = v1222;
        let v1223: f64 = p.p24;
        self.scalar_v1223 = v1223;
        let v1236: bool = (p.p18 > 0.0);
        self.scalar_v1236 = v1236;
        let v1237: f64 = p.p19;
        self.scalar_v1237 = v1237;
        let v1255: bool = (!v1236);
        self.scalar_v1255 = v1255;
        let v1257: bool = (p.p20 > 0.0);
        self.scalar_v1257 = v1257;
        let v1275: bool = (!v1257);
        self.scalar_v1275 = v1275;
        let v1304: f64 = (-p.p45);
        self.scalar_v1304 = v1304;
        let v1314: f64 = (1.0 - p.p45);
        self.scalar_v1314 = v1314;
        let v1333: f64 = (1.0 / p.p45);
        self.scalar_v1333 = v1333;
        let v1334: f64 = (1.0 - v1333);
        self.scalar_v1334 = v1334;
        let v1356: f64 = (1.0 / p.p41);
        self.scalar_v1356 = v1356;
        let v1357: f64 = (1.0 - v1356);
        self.scalar_v1357 = v1357;
        let v1376: f64 = p.p56;
        self.scalar_v1376 = v1376;
        let v1377: bool = (p.p56 < 100.0);
        self.scalar_v1377 = v1377;
        let v1380: f64 = (p.p54 / 4.0);
        self.scalar_v1380 = v1380;
        let v1449: f64 = (1.0 - p.p54);
        self.scalar_v1449 = v1449;
        let v1480: bool = (!v1377);
        self.scalar_v1480 = v1480;
        let v1513: bool = (p.p25 > 0.0);
        self.scalar_v1513 = v1513;
        let v1514: f64 = p.p26;
        self.scalar_v1514 = v1514;
        let v1532: bool = (!v1513);
        self.scalar_v1532 = v1532;
        let v1657: f64 = p.p61;
        self.scalar_v1657 = v1657;
        let v1658: bool = (p.p61 < 100.0);
        self.scalar_v1658 = v1658;
        let v1661: f64 = (p.p59 / 4.0);
        self.scalar_v1661 = v1661;
        let v1730: f64 = (1.0 - p.p59);
        self.scalar_v1730 = v1730;
        let v1761: bool = (!v1658);
        self.scalar_v1761 = v1761;
        let v1794: f64 = p.p65;
        self.scalar_v1794 = v1794;
        let v1795: bool = (p.p65 < 100.0);
        self.scalar_v1795 = v1795;
        let v1797: bool = (v548 && v1795);
        self.scalar_v1797 = v1797;
        let v1799: f64 = (p.p64 / 4.0);
        self.scalar_v1799 = v1799;
        let v1868: f64 = (1.0 - p.p64);
        self.scalar_v1868 = v1868;
        let v1899: bool = (!v1795);
        self.scalar_v1899 = v1899;
        let v1900: bool = (v548 && v1899);
        self.scalar_v1900 = v1900;
        let v1935: bool = (p.p97 > 0.0);
        self.scalar_v1935 = v1935;
        let v1936: f64 = p.p98;
        self.scalar_v1936 = v1936;
        let v1948: bool = (p.p101 > 0.0);
        self.scalar_v1948 = v1948;
        let v1949: bool = (v1935 && v1948);
        self.scalar_v1949 = v1949;
        let v1953: bool = (!v1948);
        self.scalar_v1953 = v1953;
        let v1954: bool = (v1935 && v1953);
        self.scalar_v1954 = v1954;
        let v1956: bool = (!v1935);
        self.scalar_v1956 = v1956;
        let v1959: bool = (p.p99 > 0.0);
        self.scalar_v1959 = v1959;
        let v1960: f64 = p.p100;
        self.scalar_v1960 = v1960;
        let v1978: bool = (!v1959);
        self.scalar_v1978 = v1978;
        let v1980: bool = (0.0 != v121);
        self.scalar_v1980 = v1980;
        let v1997: bool = (!v1980);
        self.scalar_v1997 = v1997;
        let v2001: bool = (p.p89 >= p.p149);
        self.scalar_v2001 = v2001;
        let v2002: bool = (p.p89 > 0.0);
        self.scalar_v2002 = v2002;
        let v2003: bool = (v2001 && v2002);
        self.scalar_v2003 = v2003;
        let v2004: bool = (p.p90 >= p.p149);
        self.scalar_v2004 = v2004;
        let v2005: bool = (p.p90 > 0.0);
        self.scalar_v2005 = v2005;
        let v2006: bool = (v2004 && v2005);
        self.scalar_v2006 = v2006;
        let v2007: bool = (p.p95 >= p.p149);
        self.scalar_v2007 = v2007;
        let v2008: bool = (p.p95 > 0.0);
        self.scalar_v2008 = v2008;
        let v2009: bool = (v2007 && v2008);
        self.scalar_v2009 = v2009;
        let v2010: bool = (p.p96 >= p.p149);
        self.scalar_v2010 = v2010;
        let v2011: bool = (p.p96 > 0.0);
        self.scalar_v2011 = v2011;
        let v2012: bool = (v2010 && v2011);
        self.scalar_v2012 = v2012;
        let v2013: f64 = p.p102;
        self.scalar_v2013 = v2013;
        let v2014: bool = (p.p102 >= p.p149);
        self.scalar_v2014 = v2014;
        let v2015: bool = (p.p102 > 0.0);
        self.scalar_v2015 = v2015;
        let v2016: bool = (v2014 && v2015);
        self.scalar_v2016 = v2016;
        let v2017: bool = (p.p141 >= 1.0);
        self.scalar_v2017 = v2017;
        let v2018: bool = (v621 && v2017);
        self.scalar_v2018 = v2018;
        let v2019: bool = (v623 && v2018);
        self.scalar_v2019 = v2019;
        let v2025: f64 = p.p109;
        self.scalar_v2025 = v2025;
        let v2026: bool = (1.0 == p.p109);
        self.scalar_v2026 = v2026;
        let v2027: bool = (p.p88 > 0.0);
        self.scalar_v2027 = v2027;
        let v2028: bool = (p.p87 > 0.0);
        self.scalar_v2028 = v2028;
        let v2029: bool = (v2027 && v2028);
        self.scalar_v2029 = v2029;
        let v2030: bool = (v2026 && v2029);
        self.scalar_v2030 = v2030;
        let v2031: f64 = (if v2030 { 1.0 } else { 0.0 });
        self.scalar_v2031 = v2031;
        let v2034: f64 = (-p.p148);
        self.scalar_v2034 = v2034;
        let v2035: bool = (!v2003);
        self.scalar_v2035 = v2035;
        let v2036: f64 = (if v2035 { 0.0 } else { 0.0 });
        self.scalar_v2036 = v2036;
        let v2039: bool = (!v353);
        self.scalar_v2039 = v2039;
        let v2053: bool = (!v2006);
        self.scalar_v2053 = v2053;
        let v2054: f64 = (if v2053 { 0.0 } else { 0.0 });
        self.scalar_v2054 = v2054;
        let v2055: bool = (!v2009);
        self.scalar_v2055 = v2055;
        let v2056: f64 = (if v2055 { 0.0 } else { 0.0 });
        self.scalar_v2056 = v2056;
        let v2060: bool = (!v2012);
        self.scalar_v2060 = v2060;
        let v2061: f64 = (if v2060 { 0.0 } else { 0.0 });
        self.scalar_v2061 = v2061;
        let v2065: f64 = p.p108;
        self.scalar_v2065 = v2065;
        let v2068: bool = (v1221 && v1959);
        self.scalar_v2068 = v2068;
        let v2073: bool = (!v1221);
        self.scalar_v2073 = v2073;
        let v2075: bool = (v1219 && v2073);
        self.scalar_v2075 = v2075;
        let v2082: bool = (!v2016);
        self.scalar_v2082 = v2082;
        let v2083: f64 = (if v2082 { 0.0 } else { 0.0 });
        self.scalar_v2083 = v2083;
        let v2084: bool = (!v2019);
        self.scalar_v2084 = v2084;
        let v2085: f64 = (if v2084 { 0.0 } else { 0.0 });
        self.scalar_v2085 = v2085;
        let v2094: bool = (!v2030);
        self.scalar_v2094 = v2094;
        let v2097: f64 = (if v624 { 1.0 } else { 0.0 });
        self.scalar_v2097 = v2097;
        let v5912: f64 = (p.p62 * v2034);
        self.scalar_v5912 = v5912;
        let v5913: f64 = (p.p148 * p.p62);
        self.scalar_v5913 = v5913;
        let v6058: f64 = (if v1980 { 1.0 } else { 0.0 });
        self.scalar_v6058 = v6058;
        let v6059: f64 = (p.p88 * v6058);
        self.scalar_v6059 = v6059;
        let v6060: f64 = (p.p66 * v6059);
        self.scalar_v6060 = v6060;
        let v6061: f64 = (if v1980 { v6060 } else { 0.0 });
        self.scalar_v6061 = v6061;
        let v6062: f64 = (v6059 / 3.0);
        self.scalar_v6062 = v6062;
        let v6063: f64 = (p.p66 * v6062);
        self.scalar_v6063 = v6063;
        let v6064: f64 = (if v1980 { v6063 } else { 0.0 });
        self.scalar_v6064 = v6064;
        let v6065: f64 = (p.p87 * v6058);
        self.scalar_v6065 = v6065;
        let v6066: f64 = (p.p66 * v6065);
        self.scalar_v6066 = v6066;
        let v6067: f64 = (if v1980 { v6066 } else { 0.0 });
        self.scalar_v6067 = v6067;
        let v6068: f64 = (if v1997 { 0.0 } else { v6061 });
        self.scalar_v6068 = v6068;
        let v6069: f64 = (if v1997 { 0.0 } else { v6064 });
        self.scalar_v6069 = v6069;
        let v6070: f64 = (if v1997 { 0.0 } else { v6067 });
        self.scalar_v6070 = v6070;
        let v6123: f64 = (-v96);
        self.scalar_v6123 = v6123;
        let v6130: f64 = (-v94);
        self.scalar_v6130 = v6130;
        let v6149: f64 = (-v101);
        self.scalar_v6149 = v6149;
        let v6150: f64 = (-v102);
        self.scalar_v6150 = v6150;
        let v6151: f64 = (-p.p108);
        self.scalar_v6151 = v6151;
        let v6169: f64 = (if v2068 { -0.0 } else { 0.0 });
        self.scalar_v6169 = v6169;
        let v6170: f64 = (if v2068 { 0.0 } else { 0.0 });
        self.scalar_v6170 = v6170;
        let v6177: f64 = (if v2075 { -0.0 } else { 0.0 });
        self.scalar_v6177 = v6177;
        let v6178: f64 = (if v2075 { 0.0 } else { 0.0 });
        self.scalar_v6178 = v6178;
        let v6195: f64 = (-1.0 / p.p102);
        self.scalar_v6195 = v6195;
        let v6196: f64 = (1.0 / p.p102);
        self.scalar_v6196 = v6196;
        let v6197: f64 = (if v2016 { v6195 } else { 0.0 });
        self.scalar_v6197 = v6197;
        let v6198: f64 = (if v2016 { v6196 } else { 0.0 });
        self.scalar_v6198 = v6198;
        let v6199: f64 = (if v2030 { -1.0 } else { 0.0 });
        self.scalar_v6199 = v6199;
        let v6200: f64 = (if v2094 { 1.0 } else { 0.0 });
        self.scalar_v6200 = v6200;
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
        let v123: f64 = (temperature + self.scalar_v122);
        self.scalar_v123 = v123;
        let v125: bool = (self.scalar_v123 < 73.14999999999998);
        self.scalar_v125 = v125;
        let v126: f64 = (if self.scalar_v125 { 73.14999999999998 } else { self.scalar_v123 });
        self.scalar_v126 = v126;
        let v128: bool = (self.scalar_v126 > 600.0);
        self.scalar_v128 = v128;
        let v129: bool = (!self.scalar_v125);
        self.scalar_v129 = v129;
        let v130: bool = (self.scalar_v128 && self.scalar_v129);
        self.scalar_v130 = v130;
        let v131: f64 = (if self.scalar_v130 { 600.0 } else { self.scalar_v126 });
        self.scalar_v131 = v131;
        let v132: f64 = (self.scalar_v40 * self.scalar_v131);
        self.scalar_v132 = v132;
        let v133: f64 = (1.0 / self.scalar_v132);
        self.scalar_v133 = v133;
        let v134: f64 = (self.scalar_v38 / self.scalar_v131);
        self.scalar_v134 = v134;
        let v135: f64 = (self.scalar_v131 / self.scalar_v38);
        self.scalar_v135 = v135;
        let v136: f64 = ((self.scalar_v135) as f64).ln();
        self.scalar_v136 = v136;
        let v137: f64 = (self.scalar_v45 * self.scalar_v131);
        self.scalar_v137 = v137;
        let v138: f64 = ((self.scalar_v131) as f64).ln();
        self.scalar_v138 = v138;
        let v139: f64 = (self.scalar_v137 * self.scalar_v138);
        self.scalar_v139 = v139;
        let v140: f64 = (self.scalar_v49 * self.scalar_v131);
        self.scalar_v140 = v140;
        let v141: f64 = (self.scalar_v51 + self.scalar_v139);
        self.scalar_v141 = v141;
        let v142: f64 = (self.scalar_v140 + self.scalar_v141);
        self.scalar_v142 = v142;
        let v143: f64 = (self.scalar_v54 + self.scalar_v139);
        self.scalar_v143 = v143;
        let v144: f64 = (self.scalar_v140 + self.scalar_v143);
        self.scalar_v144 = v144;
        let v145: f64 = (self.scalar_v57 + self.scalar_v139);
        self.scalar_v145 = v145;
        let v146: f64 = (self.scalar_v140 + self.scalar_v145);
        self.scalar_v146 = v146;
        let v147: f64 = (self.scalar_v142 + self.scalar_v144);
        self.scalar_v147 = v147;
        let v148: f64 = (0.5 * self.scalar_v147);
        self.scalar_v148 = v148;
        let v149: f64 = (self.scalar_v142 + self.scalar_v146);
        self.scalar_v149 = v149;
        let v150: f64 = (0.5 * self.scalar_v149);
        self.scalar_v150 = v150;
        let v167: f64 = (self.scalar_v135 * self.scalar_v166);
        self.scalar_v167 = v167;
        let v168: f64 = (1.0 - self.scalar_v135);
        self.scalar_v168 = v168;
        let v169: f64 = (self.scalar_v66 * self.scalar_v168);
        self.scalar_v169 = v169;
        let v170: f64 = (self.scalar_v167 + self.scalar_v169);
        self.scalar_v170 = v170;
        let v171: f64 = (self.scalar_v74 * self.scalar_v132);
        self.scalar_v171 = v171;
        let v172: f64 = (self.scalar_v136 * self.scalar_v171);
        self.scalar_v172 = v172;
        let v173: f64 = (self.scalar_v170 - self.scalar_v172);
        self.scalar_v173 = v173;
        let v174: f64 = (if self.scalar_v152 { self.scalar_v173 } else { 0.0 });
        self.scalar_v174 = v174;
        let v175: f64 = (self.scalar_v132 * 2.0);
        self.scalar_v175 = v175;
        let v177: f64 = (-self.scalar_v174);
        self.scalar_v177 = v177;
        let v178: f64 = (self.scalar_v133 * self.scalar_v177);
        self.scalar_v178 = v178;
        let v179: f64 = ((self.scalar_v178) as f64).exp();
        self.scalar_v179 = v179;
        let v180: f64 = (4.0 * self.scalar_v179);
        self.scalar_v180 = v180;
        let v181: f64 = (1.0 + self.scalar_v180);
        self.scalar_v181 = v181;
        let v182: f64 = ((self.scalar_v181) as f64).sqrt();
        self.scalar_v182 = v182;
        let v183: f64 = (1.0 + self.scalar_v182);
        self.scalar_v183 = v183;
        let v184: f64 = (0.5 * self.scalar_v183);
        self.scalar_v184 = v184;
        let v185: f64 = ((self.scalar_v184) as f64).ln();
        self.scalar_v185 = v185;
        let v186: f64 = (self.scalar_v175 * self.scalar_v185);
        self.scalar_v186 = v186;
        let v187: f64 = (self.scalar_v174 + self.scalar_v186);
        self.scalar_v187 = v187;
        let v188: f64 = (if self.scalar_v152 { self.scalar_v187 } else { 0.0 });
        self.scalar_v188 = v188;
        let v190: f64 = (self.scalar_v155 / self.scalar_v188);
        self.scalar_v190 = v190;
        let v191: f64 = ((self.scalar_v190) as f64).ln();
        self.scalar_v191 = v191;
        let v192: f64 = (self.scalar_v189 * self.scalar_v191);
        self.scalar_v192 = v192;
        let v193: f64 = ((self.scalar_v192) as f64).exp();
        self.scalar_v193 = v193;
        let v194: f64 = (self.scalar_v151 * self.scalar_v193);
        self.scalar_v194 = v194;
        let v195: f64 = (if self.scalar_v152 { self.scalar_v194 } else { 0.0 });
        self.scalar_v195 = v195;
        let v201: f64 = (self.scalar_v188 * self.scalar_v196);
        self.scalar_v201 = v201;
        let v202: f64 = (self.scalar_v201 / self.scalar_v155);
        self.scalar_v202 = v202;
        let v203: f64 = (if self.scalar_v200 { self.scalar_v202 } else { self.scalar_v198 });
        self.scalar_v203 = v203;
        let v205: f64 = (if self.scalar_v204 { self.scalar_v151 } else { self.scalar_v195 });
        self.scalar_v205 = v205;
        let v206: f64 = (if self.scalar_v204 { self.scalar_v155 } else { self.scalar_v188 });
        self.scalar_v206 = v206;
        let v207: f64 = (if self.scalar_v204 { self.scalar_v196 } else { self.scalar_v203 });
        self.scalar_v207 = v207;
        let v210: f64 = (self.scalar_v136 * self.scalar_v209);
        self.scalar_v210 = v210;
        let v212: f64 = (1.0 - self.scalar_v134);
        self.scalar_v212 = v212;
        let v213: f64 = (self.scalar_v211 * self.scalar_v212);
        self.scalar_v213 = v213;
        let v214: f64 = (self.scalar_v210 + self.scalar_v213);
        self.scalar_v214 = v214;
        let v215: f64 = ((self.scalar_v214) as f64).exp();
        self.scalar_v215 = v215;
        let v219: f64 = (self.scalar_v212 * self.scalar_v218);
        self.scalar_v219 = v219;
        let v231: f64 = (self.scalar_v135 * self.scalar_v230);
        self.scalar_v231 = v231;
        let v232: f64 = (self.scalar_v68 * self.scalar_v168);
        self.scalar_v232 = v232;
        let v233: f64 = (self.scalar_v231 + self.scalar_v232);
        self.scalar_v233 = v233;
        let v234: f64 = (self.scalar_v233 - self.scalar_v172);
        self.scalar_v234 = v234;
        let v235: f64 = (if self.scalar_v109 { self.scalar_v234 } else { self.scalar_v174 });
        self.scalar_v235 = v235;
        let v236: f64 = (-self.scalar_v235);
        self.scalar_v236 = v236;
        let v237: f64 = (self.scalar_v133 * self.scalar_v236);
        self.scalar_v237 = v237;
        let v238: f64 = ((self.scalar_v237) as f64).exp();
        self.scalar_v238 = v238;
        let v239: f64 = (4.0 * self.scalar_v238);
        self.scalar_v239 = v239;
        let v240: f64 = (1.0 + self.scalar_v239);
        self.scalar_v240 = v240;
        let v241: f64 = ((self.scalar_v240) as f64).sqrt();
        self.scalar_v241 = v241;
        let v242: f64 = (1.0 + self.scalar_v241);
        self.scalar_v242 = v242;
        let v243: f64 = (0.5 * self.scalar_v242);
        self.scalar_v243 = v243;
        let v244: f64 = ((self.scalar_v243) as f64).ln();
        self.scalar_v244 = v244;
        let v245: f64 = (self.scalar_v175 * self.scalar_v244);
        self.scalar_v245 = v245;
        let v246: f64 = (self.scalar_v235 + self.scalar_v245);
        self.scalar_v246 = v246;
        let v247: f64 = (if self.scalar_v109 { self.scalar_v246 } else { 0.0 });
        self.scalar_v247 = v247;
        let v249: f64 = (self.scalar_v220 / self.scalar_v247);
        self.scalar_v249 = v249;
        let v250: f64 = ((self.scalar_v249) as f64).ln();
        self.scalar_v250 = v250;
        let v251: f64 = (self.scalar_v248 * self.scalar_v250);
        self.scalar_v251 = v251;
        let v252: f64 = ((self.scalar_v251) as f64).exp();
        self.scalar_v252 = v252;
        let v253: f64 = (self.scalar_v108 * self.scalar_v252);
        self.scalar_v253 = v253;
        let v254: f64 = (if self.scalar_v109 { self.scalar_v253 } else { 0.0 });
        self.scalar_v254 = v254;
        let v260: f64 = (self.scalar_v247 * self.scalar_v255);
        self.scalar_v260 = v260;
        let v261: f64 = (self.scalar_v260 / self.scalar_v220);
        self.scalar_v261 = v261;
        let v262: f64 = (if self.scalar_v259 { self.scalar_v261 } else { self.scalar_v257 });
        self.scalar_v262 = v262;
        let v264: f64 = (if self.scalar_v263 { self.scalar_v108 } else { self.scalar_v254 });
        self.scalar_v264 = v264;
        let v265: f64 = (if self.scalar_v263 { self.scalar_v220 } else { self.scalar_v247 });
        self.scalar_v265 = v265;
        let v266: f64 = (if self.scalar_v263 { self.scalar_v255 } else { self.scalar_v262 });
        self.scalar_v266 = v266;
        let v268: f64 = (if self.scalar_v103 { 2.4 } else { self.scalar_v266 });
        self.scalar_v268 = v268;
        let v271: f64 = (self.scalar_v212 * self.scalar_v270);
        self.scalar_v271 = v271;
        let v272: f64 = (self.scalar_v206 / self.scalar_v155);
        self.scalar_v272 = v272;
        let v280: f64 = (self.scalar_v64 / self.scalar_v150);
        self.scalar_v280 = v280;
        let v282: f64 = (self.scalar_v265 / self.scalar_v220);
        self.scalar_v282 = v282;
        let v298: f64 = (self.scalar_v135 * self.scalar_v297);
        self.scalar_v298 = v298;
        let v299: f64 = (self.scalar_v169 + self.scalar_v298);
        self.scalar_v299 = v299;
        let v300: f64 = (self.scalar_v299 - self.scalar_v172);
        self.scalar_v300 = v300;
        let v301: f64 = (if self.scalar_v286 { self.scalar_v300 } else { self.scalar_v235 });
        self.scalar_v301 = v301;
        let v302: f64 = (-self.scalar_v301);
        self.scalar_v302 = v302;
        let v303: f64 = (self.scalar_v133 * self.scalar_v302);
        self.scalar_v303 = v303;
        let v304: f64 = ((self.scalar_v303) as f64).exp();
        self.scalar_v304 = v304;
        let v305: f64 = (4.0 * self.scalar_v304);
        self.scalar_v305 = v305;
        let v306: f64 = (1.0 + self.scalar_v305);
        self.scalar_v306 = v306;
        let v307: f64 = ((self.scalar_v306) as f64).sqrt();
        self.scalar_v307 = v307;
        let v308: f64 = (1.0 + self.scalar_v307);
        self.scalar_v308 = v308;
        let v309: f64 = (0.5 * self.scalar_v308);
        self.scalar_v309 = v309;
        let v310: f64 = ((self.scalar_v309) as f64).ln();
        self.scalar_v310 = v310;
        let v311: f64 = (self.scalar_v175 * self.scalar_v310);
        self.scalar_v311 = v311;
        let v312: f64 = (self.scalar_v301 + self.scalar_v311);
        self.scalar_v312 = v312;
        let v313: f64 = (if self.scalar_v286 { self.scalar_v312 } else { 0.0 });
        self.scalar_v313 = v313;
        let v315: f64 = (self.scalar_v287 / self.scalar_v313);
        self.scalar_v315 = v315;
        let v316: f64 = ((self.scalar_v315) as f64).ln();
        self.scalar_v316 = v316;
        let v317: f64 = (self.scalar_v314 * self.scalar_v316);
        self.scalar_v317 = v317;
        let v318: f64 = ((self.scalar_v317) as f64).exp();
        self.scalar_v318 = v318;
        let v319: f64 = (self.scalar_v285 * self.scalar_v318);
        self.scalar_v319 = v319;
        let v320: f64 = (if self.scalar_v286 { self.scalar_v319 } else { 0.0 });
        self.scalar_v320 = v320;
        let v326: f64 = (self.scalar_v313 * self.scalar_v321);
        self.scalar_v326 = v326;
        let v327: f64 = (self.scalar_v326 / self.scalar_v287);
        self.scalar_v327 = v327;
        let v328: f64 = (if self.scalar_v325 { self.scalar_v327 } else { self.scalar_v323 });
        self.scalar_v328 = v328;
        let v330: f64 = (if self.scalar_v329 { self.scalar_v285 } else { self.scalar_v320 });
        self.scalar_v330 = v330;
        let v331: f64 = (if self.scalar_v329 { self.scalar_v287 } else { self.scalar_v313 });
        self.scalar_v331 = v331;
        let v332: f64 = (if self.scalar_v329 { self.scalar_v321 } else { self.scalar_v328 });
        self.scalar_v332 = v332;
        let v334: f64 = (self.scalar_v215 * self.scalar_v333);
        self.scalar_v334 = v334;
        let v338: f64 = (self.scalar_v136 * self.scalar_v337);
        self.scalar_v338 = v338;
        let v339: f64 = (self.scalar_v219 / self.scalar_v336);
        self.scalar_v339 = v339;
        let v340: f64 = (self.scalar_v338 + self.scalar_v339);
        self.scalar_v340 = v340;
        let v341: f64 = ((self.scalar_v340) as f64).exp();
        self.scalar_v341 = v341;
        let v342: f64 = (self.scalar_v335 * self.scalar_v341);
        self.scalar_v342 = v342;
        let v350: f64 = (self.scalar_v62 / self.scalar_v148);
        self.scalar_v350 = v350;
        let v358: f64 = (self.scalar_v331 / self.scalar_v287);
        self.scalar_v358 = v358;
        let v360: f64 = (self.scalar_v330 / self.scalar_v285);
        self.scalar_v360 = v360;
        let v366: f64 = (self.scalar_v285 / self.scalar_v330);
        self.scalar_v366 = v366;
        let v380: f64 = (self.scalar_v205 / self.scalar_v151);
        self.scalar_v380 = v380;
        let v385: f64 = (self.scalar_v151 / self.scalar_v205);
        self.scalar_v385 = v385;
        let v408: f64 = (self.scalar_v135 * self.scalar_v407);
        self.scalar_v408 = v408;
        let v409: f64 = (self.scalar_v232 + self.scalar_v408);
        self.scalar_v409 = v409;
        let v410: f64 = (self.scalar_v409 - self.scalar_v172);
        self.scalar_v410 = v410;
        let v411: f64 = (-self.scalar_v410);
        self.scalar_v411 = v411;
        let v412: f64 = (self.scalar_v133 * self.scalar_v411);
        self.scalar_v412 = v412;
        let v413: f64 = ((self.scalar_v412) as f64).exp();
        self.scalar_v413 = v413;
        let v414: f64 = (4.0 * self.scalar_v413);
        self.scalar_v414 = v414;
        let v415: f64 = (1.0 + self.scalar_v414);
        self.scalar_v415 = v415;
        let v416: f64 = ((self.scalar_v415) as f64).sqrt();
        self.scalar_v416 = v416;
        let v417: f64 = (1.0 + self.scalar_v416);
        self.scalar_v417 = v417;
        let v418: f64 = (0.5 * self.scalar_v417);
        self.scalar_v418 = v418;
        let v419: f64 = ((self.scalar_v418) as f64).ln();
        self.scalar_v419 = v419;
        let v420: f64 = (self.scalar_v175 * self.scalar_v419);
        self.scalar_v420 = v420;
        let v421: f64 = (self.scalar_v410 + self.scalar_v420);
        self.scalar_v421 = v421;
        let v423: f64 = (self.scalar_v398 / self.scalar_v421);
        self.scalar_v423 = v423;
        let v424: f64 = ((self.scalar_v423) as f64).ln();
        self.scalar_v424 = v424;
        let v425: f64 = (self.scalar_v422 * self.scalar_v424);
        self.scalar_v425 = v425;
        let v426: f64 = ((self.scalar_v425) as f64).exp();
        self.scalar_v426 = v426;
        let v431: f64 = (self.scalar_v421 * self.scalar_v427);
        self.scalar_v431 = v431;
        let v432: f64 = (self.scalar_v431 / self.scalar_v398);
        self.scalar_v432 = v432;
        let v433: f64 = (if self.scalar_v430 { self.scalar_v432 } else { self.scalar_v428 });
        self.scalar_v433 = v433;
        let v435: f64 = (if self.scalar_v103 { 2.4 } else { self.scalar_v433 });
        self.scalar_v435 = v435;
        let v436: f64 = (self.scalar_v97 * self.scalar_v426);
        self.scalar_v436 = v436;
        let v437: f64 = (self.scalar_v98 * self.scalar_v426);
        self.scalar_v437 = v437;
        let v439: f64 = (self.scalar_v77 * self.scalar_v136);
        self.scalar_v439 = v439;
        let v440: f64 = (self.scalar_v271 + self.scalar_v439);
        self.scalar_v440 = v440;
        let v441: f64 = ((self.scalar_v440) as f64).exp();
        self.scalar_v441 = v441;
        let v442: f64 = (self.scalar_v438 * self.scalar_v441);
        self.scalar_v442 = v442;
        let v457: f64 = (self.scalar_v135 * self.scalar_v456);
        self.scalar_v457 = v457;
        let v458: f64 = (self.scalar_v71 * self.scalar_v168);
        self.scalar_v458 = v458;
        let v459: f64 = (self.scalar_v457 + self.scalar_v458);
        self.scalar_v459 = v459;
        let v460: f64 = (self.scalar_v459 - self.scalar_v172);
        self.scalar_v460 = v460;
        let v461: f64 = (if self.scalar_v445 { self.scalar_v460 } else { self.scalar_v410 });
        self.scalar_v461 = v461;
        let v462: f64 = (-self.scalar_v461);
        self.scalar_v462 = v462;
        let v463: f64 = (self.scalar_v133 * self.scalar_v462);
        self.scalar_v463 = v463;
        let v464: f64 = ((self.scalar_v463) as f64).exp();
        self.scalar_v464 = v464;
        let v465: f64 = (4.0 * self.scalar_v464);
        self.scalar_v465 = v465;
        let v466: f64 = (1.0 + self.scalar_v465);
        self.scalar_v466 = v466;
        let v467: f64 = ((self.scalar_v466) as f64).sqrt();
        self.scalar_v467 = v467;
        let v468: f64 = (1.0 + self.scalar_v467);
        self.scalar_v468 = v468;
        let v469: f64 = (0.5 * self.scalar_v468);
        self.scalar_v469 = v469;
        let v470: f64 = ((self.scalar_v469) as f64).ln();
        self.scalar_v470 = v470;
        let v471: f64 = (self.scalar_v175 * self.scalar_v470);
        self.scalar_v471 = v471;
        let v472: f64 = (self.scalar_v461 + self.scalar_v471);
        self.scalar_v472 = v472;
        let v473: f64 = (if self.scalar_v445 { self.scalar_v472 } else { 0.0 });
        self.scalar_v473 = v473;
        let v475: f64 = (self.scalar_v446 / self.scalar_v473);
        self.scalar_v475 = v475;
        let v476: f64 = ((self.scalar_v475) as f64).ln();
        self.scalar_v476 = v476;
        let v477: f64 = (self.scalar_v474 * self.scalar_v476);
        self.scalar_v477 = v477;
        let v478: f64 = ((self.scalar_v477) as f64).exp();
        self.scalar_v478 = v478;
        let v479: f64 = (self.scalar_v443 * self.scalar_v478);
        self.scalar_v479 = v479;
        let v480: f64 = (if self.scalar_v445 { self.scalar_v479 } else { 0.0 });
        self.scalar_v480 = v480;
        let v484: f64 = (self.scalar_v473 * -2.4);
        self.scalar_v484 = v484;
        let v485: f64 = (self.scalar_v484 / self.scalar_v446);
        self.scalar_v485 = v485;
        let v486: f64 = (if self.scalar_v483 { self.scalar_v485 } else { self.scalar_v482 });
        self.scalar_v486 = v486;
        let v489: f64 = (if self.scalar_v488 { self.scalar_v443 } else { self.scalar_v480 });
        self.scalar_v489 = v489;
        let v490: f64 = (if self.scalar_v488 { self.scalar_v446 } else { self.scalar_v473 });
        self.scalar_v490 = v490;
        let v491: f64 = (if self.scalar_v488 { -2.4 } else { self.scalar_v486 });
        self.scalar_v491 = v491;
        let v495: f64 = (self.scalar_v135 * self.scalar_v494);
        self.scalar_v495 = v495;
        let v496: f64 = (self.scalar_v458 + self.scalar_v495);
        self.scalar_v496 = v496;
        let v497: f64 = (self.scalar_v496 - self.scalar_v172);
        self.scalar_v497 = v497;
        let v498: f64 = (if self.scalar_v493 { self.scalar_v497 } else { self.scalar_v461 });
        self.scalar_v498 = v498;
        let v499: f64 = (-self.scalar_v498);
        self.scalar_v499 = v499;
        let v500: f64 = (self.scalar_v133 * self.scalar_v499);
        self.scalar_v500 = v500;
        let v501: f64 = ((self.scalar_v500) as f64).exp();
        self.scalar_v501 = v501;
        let v502: f64 = (4.0 * self.scalar_v501);
        self.scalar_v502 = v502;
        let v503: f64 = (1.0 + self.scalar_v502);
        self.scalar_v503 = v503;
        let v504: f64 = ((self.scalar_v503) as f64).sqrt();
        self.scalar_v504 = v504;
        let v505: f64 = (1.0 + self.scalar_v504);
        self.scalar_v505 = v505;
        let v506: f64 = (0.5 * self.scalar_v505);
        self.scalar_v506 = v506;
        let v507: f64 = ((self.scalar_v506) as f64).ln();
        self.scalar_v507 = v507;
        let v508: f64 = (self.scalar_v175 * self.scalar_v507);
        self.scalar_v508 = v508;
        let v509: f64 = (self.scalar_v498 + self.scalar_v508);
        self.scalar_v509 = v509;
        let v510: f64 = (if self.scalar_v493 { self.scalar_v509 } else { self.scalar_v490 });
        self.scalar_v510 = v510;
        let v511: f64 = (self.scalar_v446 / self.scalar_v510);
        self.scalar_v511 = v511;
        let v512: f64 = ((self.scalar_v511) as f64).ln();
        self.scalar_v512 = v512;
        let v513: f64 = (self.scalar_v474 * self.scalar_v512);
        self.scalar_v513 = v513;
        let v514: f64 = ((self.scalar_v513) as f64).exp();
        self.scalar_v514 = v514;
        let v515: f64 = (self.scalar_v443 * self.scalar_v514);
        self.scalar_v515 = v515;
        let v516: f64 = (if self.scalar_v493 { self.scalar_v515 } else { self.scalar_v489 });
        self.scalar_v516 = v516;
        let v520: f64 = (if self.scalar_v493 { self.scalar_v519 } else { self.scalar_v491 });
        self.scalar_v520 = v520;
        let v523: f64 = (self.scalar_v510 * self.scalar_v518);
        self.scalar_v523 = v523;
        let v524: f64 = (self.scalar_v523 / self.scalar_v446);
        self.scalar_v524 = v524;
        let v525: f64 = (if self.scalar_v522 { self.scalar_v524 } else { self.scalar_v520 });
        self.scalar_v525 = v525;
        let v527: f64 = (if self.scalar_v526 { self.scalar_v443 } else { self.scalar_v516 });
        self.scalar_v527 = v527;
        let v528: f64 = (if self.scalar_v526 { self.scalar_v446 } else { self.scalar_v510 });
        self.scalar_v528 = v528;
        let v529: f64 = (if self.scalar_v526 { self.scalar_v518 } else { self.scalar_v525 });
        self.scalar_v529 = v529;
        let v532: f64 = (self.scalar_v79 * self.scalar_v136);
        self.scalar_v532 = v532;
        let v534: f64 = (self.scalar_v212 * self.scalar_v533);
        self.scalar_v534 = v534;
        let v535: f64 = (self.scalar_v532 + self.scalar_v534);
        self.scalar_v535 = v535;
        let v536: f64 = ((self.scalar_v535) as f64).exp();
        self.scalar_v536 = v536;
        let v537: f64 = (self.scalar_v531 * self.scalar_v536);
        self.scalar_v537 = v537;
        let v539: f64 = (self.scalar_v271 + self.scalar_v532);
        self.scalar_v539 = v539;
        let v540: f64 = ((self.scalar_v539) as f64).exp();
        self.scalar_v540 = v540;
        let v541: f64 = (self.scalar_v538 * self.scalar_v540);
        self.scalar_v541 = v541;
        let v544: f64 = (self.scalar_v136 * self.scalar_v543);
        self.scalar_v544 = v544;
        let v545: f64 = ((self.scalar_v544) as f64).exp();
        self.scalar_v545 = v545;
        let v546: f64 = (self.scalar_v542 * self.scalar_v545);
        self.scalar_v546 = v546;
        let v562: f64 = (self.scalar_v135 * self.scalar_v561);
        self.scalar_v562 = v562;
        let v563: f64 = (self.scalar_v458 + self.scalar_v562);
        self.scalar_v563 = v563;
        let v564: f64 = (self.scalar_v563 - self.scalar_v172);
        self.scalar_v564 = v564;
        let v565: f64 = (if self.scalar_v551 { self.scalar_v564 } else { self.scalar_v498 });
        self.scalar_v565 = v565;
        let v566: f64 = (-self.scalar_v565);
        self.scalar_v566 = v566;
        let v567: f64 = (self.scalar_v133 * self.scalar_v566);
        self.scalar_v567 = v567;
        let v568: f64 = ((self.scalar_v567) as f64).exp();
        self.scalar_v568 = v568;
        let v569: f64 = (4.0 * self.scalar_v568);
        self.scalar_v569 = v569;
        let v570: f64 = (1.0 + self.scalar_v569);
        self.scalar_v570 = v570;
        let v571: f64 = ((self.scalar_v570) as f64).sqrt();
        self.scalar_v571 = v571;
        let v572: f64 = (1.0 + self.scalar_v571);
        self.scalar_v572 = v572;
        let v573: f64 = (0.5 * self.scalar_v572);
        self.scalar_v573 = v573;
        let v574: f64 = ((self.scalar_v573) as f64).ln();
        self.scalar_v574 = v574;
        let v575: f64 = (self.scalar_v175 * self.scalar_v574);
        self.scalar_v575 = v575;
        let v576: f64 = (self.scalar_v565 + self.scalar_v575);
        self.scalar_v576 = v576;
        let v577: f64 = (if self.scalar_v551 { self.scalar_v576 } else { 0.0 });
        self.scalar_v577 = v577;
        let v579: f64 = (self.scalar_v547 / self.scalar_v577);
        self.scalar_v579 = v579;
        let v580: f64 = ((self.scalar_v579) as f64).ln();
        self.scalar_v580 = v580;
        let v581: f64 = (self.scalar_v578 * self.scalar_v580);
        self.scalar_v581 = v581;
        let v582: f64 = ((self.scalar_v581) as f64).exp();
        self.scalar_v582 = v582;
        let v583: f64 = (self.scalar_v549 * self.scalar_v582);
        self.scalar_v583 = v583;
        let v584: f64 = (if self.scalar_v551 { self.scalar_v583 } else { 0.0 });
        self.scalar_v584 = v584;
        let v590: f64 = (self.scalar_v577 * self.scalar_v585);
        self.scalar_v590 = v590;
        let v591: f64 = (self.scalar_v590 / self.scalar_v547);
        self.scalar_v591 = v591;
        let v592: f64 = (if self.scalar_v589 { self.scalar_v591 } else { self.scalar_v587 });
        self.scalar_v592 = v592;
        let v595: f64 = (if self.scalar_v594 { self.scalar_v549 } else { self.scalar_v584 });
        self.scalar_v595 = v595;
        let v596: f64 = (if self.scalar_v594 { self.scalar_v547 } else { self.scalar_v577 });
        self.scalar_v596 = v596;
        let v597: f64 = (if self.scalar_v594 { self.scalar_v585 } else { self.scalar_v592 });
        self.scalar_v597 = v597;
        let v599: f64 = (if self.scalar_v598 { self.scalar_v549 } else { self.scalar_v595 });
        self.scalar_v599 = v599;
        let v600: f64 = (if self.scalar_v598 { self.scalar_v547 } else { self.scalar_v596 });
        self.scalar_v600 = v600;
        let v601: f64 = (if self.scalar_v598 { self.scalar_v530 } else { self.scalar_v597 });
        self.scalar_v601 = v601;
        let v604: f64 = (self.scalar_v136 * self.scalar_v603);
        self.scalar_v604 = v604;
        let v605: f64 = ((self.scalar_v604) as f64).exp();
        self.scalar_v605 = v605;
        let v606: f64 = (self.scalar_v602 * self.scalar_v605);
        self.scalar_v606 = v606;
        let v609: f64 = (self.scalar_v136 * self.scalar_v608);
        self.scalar_v609 = v609;
        let v610: f64 = ((self.scalar_v609) as f64).exp();
        self.scalar_v610 = v610;
        let v611: f64 = (self.scalar_v607 * self.scalar_v610);
        self.scalar_v611 = v611;
        let v614: f64 = (self.scalar_v136 * self.scalar_v613);
        self.scalar_v614 = v614;
        let v615: f64 = ((self.scalar_v614) as f64).exp();
        self.scalar_v615 = v615;
        let v616: f64 = (self.scalar_v612 * self.scalar_v615);
        self.scalar_v616 = v616;
        let v696: f64 = (if self.scalar_v667 { self.scalar_v197 } else { self.scalar_v207 });
        self.scalar_v696 = v696;
        let v736: f64 = (if self.scalar_v711 { self.scalar_v256 } else { self.scalar_v268 });
        self.scalar_v736 = v736;
        let v779: f64 = (if self.scalar_v755 { self.scalar_v322 } else { self.scalar_v332 });
        self.scalar_v779 = v779;
        let v857: f64 = (if self.scalar_v834 { self.scalar_v428 } else { self.scalar_v435 });
        self.scalar_v857 = v857;
        let v901: f64 = (if self.scalar_v876 { 2.4 } else { self.scalar_v529 });
        self.scalar_v901 = v901;
        let v987: f64 = (if self.scalar_v961 { self.scalar_v986 } else { self.scalar_v601 });
        self.scalar_v987 = v987;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
