#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;
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
            params.p2 = 1.0;
            params.p3 = 0.0;
            params.p4 = 1.4;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 1000.0;
            params.p8 = -100.0;
            params.p9 = 500.0;
            params.p10 = 10000.0;
            params.p11 = 21.0;
            params.p12 = 1e-8;
            params.p13 = 9900000000.0;
            params.p14 = 1e-8;
            params.p15 = 9900000000.0;
            params.p16 = 1.0;
            params.p17 = -1.0;
            params.p18 = -1.0;
            params.p19 = 2e-9;
            params.p20 = 3.9;
            params.p21 = 1.0;
            params.p22 = 0.1;
            params.p23 = 0.0;
            params.p24 = 3e23;
            params.p25 = 1.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.1;
            params.p29 = 1e27;
            params.p30 = 1.0;
            params.p31 = 0.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = 0.0;
            params.p35 = 0.0;
            params.p36 = 1.0;
            params.p37 = 0.0;
            params.p38 = 0.0001;
            params.p39 = 1000.0;
            params.p40 = 0.05;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 1.0;
            params.p49 = 0.0;
            params.p50 = 3.1;
            params.p51 = 4.5;
            params.p52 = 2.0;
            params.p53 = 0.0;
            params.p54 = 5e25;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.375;
            params.p59 = 0.063;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 0.375;
            params.p64 = 0.063;
            params.p65 = 1e-5;
            params.p66 = 1.0;
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
    pub nodes: [usize; 7],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 67]>,
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
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v22: bool,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: bool,
    pub(crate) scalar_v31: bool,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: bool,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v49: bool,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v249: bool,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v260: f64,
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
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
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
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: bool,
    pub(crate) scalar_v318: bool,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: bool,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: bool,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: bool,
    pub(crate) scalar_v335: bool,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v351: bool,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v829: bool,
    pub(crate) scalar_v830: f64,
    pub(crate) scalar_v831: f64,
    pub(crate) scalar_v832: f64,
    pub(crate) scalar_v1511: bool,
    pub(crate) scalar_v1513: f64,
    pub(crate) scalar_v1514: bool,
    pub(crate) scalar_v2409: f64,
    pub(crate) scalar_v2410: f64,
    pub(crate) scalar_v2467: bool,
    pub(crate) scalar_v2514: f64,
    pub(crate) scalar_v2518: f64,
    pub(crate) scalar_v2519: bool,
    pub(crate) scalar_v2527: bool,
    pub(crate) scalar_v2742: bool,
    pub(crate) scalar_v3264: f64,
    pub(crate) scalar_v3284: f64,
    pub(crate) scalar_v3314: f64,
    pub(crate) scalar_v3315: f64,
    pub(crate) scalar_v10576: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v124: bool,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
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
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: bool,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v211: bool,
    pub(crate) scalar_v212: bool,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: bool,
    pub(crate) scalar_v217: bool,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: bool,
    pub(crate) scalar_v222: bool,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: bool,
    pub(crate) scalar_v227: bool,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: bool,
    pub(crate) scalar_v233: bool,
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
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v842: f64,
    pub(crate) scalar_v843: f64,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v853: f64,
    pub(crate) scalar_v969: f64,
    pub(crate) scalar_v970: f64,
    pub(crate) scalar_v971: f64,
    pub(crate) scalar_v973: f64,
    pub(crate) scalar_v1001: f64,
    pub(crate) scalar_v1003: f64,
    pub(crate) scalar_v1010: f64,
    pub(crate) scalar_v1108: f64,
    pub(crate) scalar_v2414: f64,
    pub(crate) scalar_v2520: f64,
    pub(crate) scalar_v2521: f64,
    pub(crate) scalar_v2528: bool,
    pub(crate) scalar_v2529: bool,
    pub(crate) scalar_v2530: bool,
    pub(crate) scalar_v2531: bool,
    pub(crate) scalar_v2541: f64,
    pub(crate) scalar_v2542: f64,
    pub(crate) scalar_v2543: f64,
    pub(crate) scalar_v2544: f64,
    pub(crate) scalar_v2569: f64,
    pub(crate) scalar_v2571: f64,
    pub(crate) scalar_v2735: bool,
    pub(crate) scalar_v2738: bool,
    pub(crate) scalar_v2741: f64,
    pub(crate) scalar_v2743: bool,
    pub(crate) scalar_v2744: bool,
    pub(crate) scalar_v2774: bool,
    pub(crate) scalar_v2798: bool,
    pub(crate) scalar_v2800: f64,
    pub(crate) scalar_v2807: bool,
    pub(crate) scalar_v2808: f64,
    pub(crate) scalar_v2876: bool,
    pub(crate) scalar_v2902: bool,
    pub(crate) scalar_v2925: bool,
    pub(crate) scalar_v2930: bool,
    pub(crate) scalar_v2998: bool,
    pub(crate) scalar_v2999: bool,
    pub(crate) scalar_v3000: bool,
    pub(crate) scalar_v3001: bool,
    pub(crate) scalar_v3007: f64,
    pub(crate) scalar_v3008: bool,
    pub(crate) scalar_v3009: bool,
    pub(crate) scalar_v3039: bool,
    pub(crate) scalar_v3062: bool,
    pub(crate) scalar_v3069: bool,
    pub(crate) scalar_v3136: bool,
    pub(crate) scalar_v3162: bool,
    pub(crate) scalar_v3185: bool,
    pub(crate) scalar_v3190: bool,
    pub(crate) scalar_v3480: f64,
    pub(crate) scalar_v3481: f64,
    pub(crate) scalar_v3526: f64,
    pub(crate) scalar_v3527: f64,
    pub(crate) scalar_v6465: f64,
    pub(crate) scalar_v9095: f64,
    pub(crate) scalar_v9096: f64,
    pub(crate) scalar_v9524: f64,
    pub(crate) scalar_v9525: f64,
    pub(crate) scalar_v9647: f64,
    pub(crate) scalar_v9648: f64,
    pub(crate) scalar_v9649: f64,
    pub(crate) scalar_v9650: f64,
    pub(crate) scalar_v9937: f64,
    pub(crate) scalar_v9938: f64,
    pub(crate) scalar_v10125: f64,
    pub(crate) scalar_v10126: f64,
    pub(crate) scalar_v10127: f64,
    pub(crate) scalar_v10128: f64,
    pub(crate) scalar_v10577: f64,
    pub(crate) scalar_v10578: f64,
    pub(crate) scalar_v10579: f64,
    pub(crate) scalar_v10580: f64,
    pub(crate) scalar_v10581: f64,
    pub(crate) scalar_v10582: f64,
    pub(crate) scalar_v10592: f64,
    pub(crate) scalar_v10593: f64,
    pub(crate) scalar_v10594: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
    pub(crate) scratch: Option<Box<GenericScratch<432, 7, 4>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<432, 7, 4>>>,
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
            scalar_v2: self.scalar_v2,
            scalar_v4: self.scalar_v4,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v11: self.scalar_v11,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v22: self.scalar_v22,
            scalar_v24: self.scalar_v24,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v52: self.scalar_v52,
            scalar_v54: self.scalar_v54,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v74: self.scalar_v74,
            scalar_v76: self.scalar_v76,
            scalar_v78: self.scalar_v78,
            scalar_v80: self.scalar_v80,
            scalar_v82: self.scalar_v82,
            scalar_v84: self.scalar_v84,
            scalar_v86: self.scalar_v86,
            scalar_v88: self.scalar_v88,
            scalar_v90: self.scalar_v90,
            scalar_v92: self.scalar_v92,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v198: self.scalar_v198,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v208: self.scalar_v208,
            scalar_v249: self.scalar_v249,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
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
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
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
            scalar_v314: self.scalar_v314,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v318: self.scalar_v318,
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
            scalar_v345: self.scalar_v345,
            scalar_v348: self.scalar_v348,
            scalar_v351: self.scalar_v351,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v366: self.scalar_v366,
            scalar_v369: self.scalar_v369,
            scalar_v373: self.scalar_v373,
            scalar_v379: self.scalar_v379,
            scalar_v386: self.scalar_v386,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v401: self.scalar_v401,
            scalar_v440: self.scalar_v440,
            scalar_v829: self.scalar_v829,
            scalar_v830: self.scalar_v830,
            scalar_v831: self.scalar_v831,
            scalar_v832: self.scalar_v832,
            scalar_v1511: self.scalar_v1511,
            scalar_v1513: self.scalar_v1513,
            scalar_v1514: self.scalar_v1514,
            scalar_v2409: self.scalar_v2409,
            scalar_v2410: self.scalar_v2410,
            scalar_v2467: self.scalar_v2467,
            scalar_v2514: self.scalar_v2514,
            scalar_v2518: self.scalar_v2518,
            scalar_v2519: self.scalar_v2519,
            scalar_v2527: self.scalar_v2527,
            scalar_v2742: self.scalar_v2742,
            scalar_v3264: self.scalar_v3264,
            scalar_v3284: self.scalar_v3284,
            scalar_v3314: self.scalar_v3314,
            scalar_v3315: self.scalar_v3315,
            scalar_v10576: self.scalar_v10576,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v69: self.scalar_v69,
            scalar_v72: self.scalar_v72,
            scalar_v73: self.scalar_v73,
            scalar_v75: self.scalar_v75,
            scalar_v77: self.scalar_v77,
            scalar_v79: self.scalar_v79,
            scalar_v81: self.scalar_v81,
            scalar_v83: self.scalar_v83,
            scalar_v85: self.scalar_v85,
            scalar_v87: self.scalar_v87,
            scalar_v89: self.scalar_v89,
            scalar_v91: self.scalar_v91,
            scalar_v93: self.scalar_v93,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v122: self.scalar_v122,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
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
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
            scalar_v147: self.scalar_v147,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
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
            scalar_v188: self.scalar_v188,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v204: self.scalar_v204,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
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
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v841: self.scalar_v841,
            scalar_v842: self.scalar_v842,
            scalar_v843: self.scalar_v843,
            scalar_v846: self.scalar_v846,
            scalar_v853: self.scalar_v853,
            scalar_v969: self.scalar_v969,
            scalar_v970: self.scalar_v970,
            scalar_v971: self.scalar_v971,
            scalar_v973: self.scalar_v973,
            scalar_v1001: self.scalar_v1001,
            scalar_v1003: self.scalar_v1003,
            scalar_v1010: self.scalar_v1010,
            scalar_v1108: self.scalar_v1108,
            scalar_v2414: self.scalar_v2414,
            scalar_v2520: self.scalar_v2520,
            scalar_v2521: self.scalar_v2521,
            scalar_v2528: self.scalar_v2528,
            scalar_v2529: self.scalar_v2529,
            scalar_v2530: self.scalar_v2530,
            scalar_v2531: self.scalar_v2531,
            scalar_v2541: self.scalar_v2541,
            scalar_v2542: self.scalar_v2542,
            scalar_v2543: self.scalar_v2543,
            scalar_v2544: self.scalar_v2544,
            scalar_v2569: self.scalar_v2569,
            scalar_v2571: self.scalar_v2571,
            scalar_v2735: self.scalar_v2735,
            scalar_v2738: self.scalar_v2738,
            scalar_v2741: self.scalar_v2741,
            scalar_v2743: self.scalar_v2743,
            scalar_v2744: self.scalar_v2744,
            scalar_v2774: self.scalar_v2774,
            scalar_v2798: self.scalar_v2798,
            scalar_v2800: self.scalar_v2800,
            scalar_v2807: self.scalar_v2807,
            scalar_v2808: self.scalar_v2808,
            scalar_v2876: self.scalar_v2876,
            scalar_v2902: self.scalar_v2902,
            scalar_v2925: self.scalar_v2925,
            scalar_v2930: self.scalar_v2930,
            scalar_v2998: self.scalar_v2998,
            scalar_v2999: self.scalar_v2999,
            scalar_v3000: self.scalar_v3000,
            scalar_v3001: self.scalar_v3001,
            scalar_v3007: self.scalar_v3007,
            scalar_v3008: self.scalar_v3008,
            scalar_v3009: self.scalar_v3009,
            scalar_v3039: self.scalar_v3039,
            scalar_v3062: self.scalar_v3062,
            scalar_v3069: self.scalar_v3069,
            scalar_v3136: self.scalar_v3136,
            scalar_v3162: self.scalar_v3162,
            scalar_v3185: self.scalar_v3185,
            scalar_v3190: self.scalar_v3190,
            scalar_v3480: self.scalar_v3480,
            scalar_v3481: self.scalar_v3481,
            scalar_v3526: self.scalar_v3526,
            scalar_v3527: self.scalar_v3527,
            scalar_v6465: self.scalar_v6465,
            scalar_v9095: self.scalar_v9095,
            scalar_v9096: self.scalar_v9096,
            scalar_v9524: self.scalar_v9524,
            scalar_v9525: self.scalar_v9525,
            scalar_v9647: self.scalar_v9647,
            scalar_v9648: self.scalar_v9648,
            scalar_v9649: self.scalar_v9649,
            scalar_v9650: self.scalar_v9650,
            scalar_v9937: self.scalar_v9937,
            scalar_v9938: self.scalar_v9938,
            scalar_v10125: self.scalar_v10125,
            scalar_v10126: self.scalar_v10126,
            scalar_v10127: self.scalar_v10127,
            scalar_v10128: self.scalar_v10128,
            scalar_v10577: self.scalar_v10577,
            scalar_v10578: self.scalar_v10578,
            scalar_v10579: self.scalar_v10579,
            scalar_v10580: self.scalar_v10580,
            scalar_v10581: self.scalar_v10581,
            scalar_v10582: self.scalar_v10582,
            scalar_v10592: self.scalar_v10592,
            scalar_v10593: self.scalar_v10593,
            scalar_v10594: self.scalar_v10594,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 3;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 7;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["gii", "gi", "ci", "n"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 67;
    pub const VARIABLE_COUNT: usize = 432;
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
            scalar_v2: 0.0,
            scalar_v4: 0.0,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v22: false,
            scalar_v24: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: false,
            scalar_v31: false,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v35: false,
            scalar_v36: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v40: 0.0,
            scalar_v41: false,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v49: false,
            scalar_v50: 0.0,
            scalar_v52: 0.0,
            scalar_v54: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v74: 0.0,
            scalar_v76: 0.0,
            scalar_v78: 0.0,
            scalar_v80: 0.0,
            scalar_v82: 0.0,
            scalar_v84: 0.0,
            scalar_v86: 0.0,
            scalar_v88: 0.0,
            scalar_v90: 0.0,
            scalar_v92: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v198: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v208: 0.0,
            scalar_v249: false,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v257: 0.0,
            scalar_v258: 0.0,
            scalar_v260: 0.0,
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
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v276: 0.0,
            scalar_v277: 0.0,
            scalar_v278: 0.0,
            scalar_v279: 0.0,
            scalar_v280: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
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
            scalar_v314: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: false,
            scalar_v318: false,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: 0.0,
            scalar_v324: false,
            scalar_v325: false,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: false,
            scalar_v329: false,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: false,
            scalar_v335: false,
            scalar_v336: 0.0,
            scalar_v345: 0.0,
            scalar_v348: 0.0,
            scalar_v351: false,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v366: 0.0,
            scalar_v369: 0.0,
            scalar_v373: 0.0,
            scalar_v379: 0.0,
            scalar_v386: 0.0,
            scalar_v393: 0.0,
            scalar_v394: 0.0,
            scalar_v401: 0.0,
            scalar_v440: 0.0,
            scalar_v829: false,
            scalar_v830: 0.0,
            scalar_v831: 0.0,
            scalar_v832: 0.0,
            scalar_v1511: false,
            scalar_v1513: 0.0,
            scalar_v1514: false,
            scalar_v2409: 0.0,
            scalar_v2410: 0.0,
            scalar_v2467: false,
            scalar_v2514: 0.0,
            scalar_v2518: 0.0,
            scalar_v2519: false,
            scalar_v2527: false,
            scalar_v2742: false,
            scalar_v3264: 0.0,
            scalar_v3284: 0.0,
            scalar_v3314: 0.0,
            scalar_v3315: 0.0,
            scalar_v10576: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v69: 0.0,
            scalar_v72: 0.0,
            scalar_v73: 0.0,
            scalar_v75: 0.0,
            scalar_v77: 0.0,
            scalar_v79: 0.0,
            scalar_v81: 0.0,
            scalar_v83: 0.0,
            scalar_v85: 0.0,
            scalar_v87: 0.0,
            scalar_v89: 0.0,
            scalar_v91: 0.0,
            scalar_v93: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v122: 0.0,
            scalar_v124: false,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
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
            scalar_v139: 0.0,
            scalar_v140: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v165: false,
            scalar_v166: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: false,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v188: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v204: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v209: 0.0,
            scalar_v210: 0.0,
            scalar_v211: false,
            scalar_v212: false,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v216: false,
            scalar_v217: false,
            scalar_v218: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v221: false,
            scalar_v222: false,
            scalar_v223: 0.0,
            scalar_v224: 0.0,
            scalar_v225: 0.0,
            scalar_v226: false,
            scalar_v227: false,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v231: false,
            scalar_v233: false,
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
            scalar_v248: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v342: 0.0,
            scalar_v343: 0.0,
            scalar_v344: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v354: 0.0,
            scalar_v355: 0.0,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v841: 0.0,
            scalar_v842: 0.0,
            scalar_v843: 0.0,
            scalar_v846: 0.0,
            scalar_v853: 0.0,
            scalar_v969: 0.0,
            scalar_v970: 0.0,
            scalar_v971: 0.0,
            scalar_v973: 0.0,
            scalar_v1001: 0.0,
            scalar_v1003: 0.0,
            scalar_v1010: 0.0,
            scalar_v1108: 0.0,
            scalar_v2414: 0.0,
            scalar_v2520: 0.0,
            scalar_v2521: 0.0,
            scalar_v2528: false,
            scalar_v2529: false,
            scalar_v2530: false,
            scalar_v2531: false,
            scalar_v2541: 0.0,
            scalar_v2542: 0.0,
            scalar_v2543: 0.0,
            scalar_v2544: 0.0,
            scalar_v2569: 0.0,
            scalar_v2571: 0.0,
            scalar_v2735: false,
            scalar_v2738: false,
            scalar_v2741: 0.0,
            scalar_v2743: false,
            scalar_v2744: false,
            scalar_v2774: false,
            scalar_v2798: false,
            scalar_v2800: 0.0,
            scalar_v2807: false,
            scalar_v2808: 0.0,
            scalar_v2876: false,
            scalar_v2902: false,
            scalar_v2925: false,
            scalar_v2930: false,
            scalar_v2998: false,
            scalar_v2999: false,
            scalar_v3000: false,
            scalar_v3001: false,
            scalar_v3007: 0.0,
            scalar_v3008: false,
            scalar_v3009: false,
            scalar_v3039: false,
            scalar_v3062: false,
            scalar_v3069: false,
            scalar_v3136: false,
            scalar_v3162: false,
            scalar_v3185: false,
            scalar_v3190: false,
            scalar_v3480: 0.0,
            scalar_v3481: 0.0,
            scalar_v3526: 0.0,
            scalar_v3527: 0.0,
            scalar_v6465: 0.0,
            scalar_v9095: 0.0,
            scalar_v9096: 0.0,
            scalar_v9524: 0.0,
            scalar_v9525: 0.0,
            scalar_v9647: 0.0,
            scalar_v9648: 0.0,
            scalar_v9649: 0.0,
            scalar_v9650: 0.0,
            scalar_v9937: 0.0,
            scalar_v9938: 0.0,
            scalar_v10125: 0.0,
            scalar_v10126: 0.0,
            scalar_v10127: 0.0,
            scalar_v10128: 0.0,
            scalar_v10577: 0.0,
            scalar_v10578: 0.0,
            scalar_v10579: 0.0,
            scalar_v10580: 0.0,
            scalar_v10581: 0.0,
            scalar_v10582: 0.0,
            scalar_v10592: 0.0,
            scalar_v10593: 0.0,
            scalar_v10594: 0.0,
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: None,
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
            scalar_v2,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v24,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v43,
            scalar_v44,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v52,
            scalar_v54,
            scalar_v70,
            scalar_v71,
            scalar_v74,
            scalar_v76,
            scalar_v78,
            scalar_v80,
            scalar_v82,
            scalar_v84,
            scalar_v86,
            scalar_v88,
            scalar_v90,
            scalar_v92,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v190,
            scalar_v191,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v198,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v208,
            scalar_v249,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
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
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v299,
            scalar_v300,
            scalar_v301,
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
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
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
            scalar_v345,
            scalar_v348,
            scalar_v351,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v369,
            scalar_v373,
            scalar_v379,
            scalar_v386,
            scalar_v393,
            scalar_v394,
            scalar_v401,
            scalar_v440,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v832,
            scalar_v1511,
            scalar_v1513,
            scalar_v1514,
            scalar_v2409,
            scalar_v2410,
            scalar_v2467,
            scalar_v2514,
            scalar_v2518,
            scalar_v2519,
            scalar_v2527,
            scalar_v2742,
            scalar_v3264,
            scalar_v3284,
            scalar_v3314,
            scalar_v3315,
            scalar_v10576,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v63,
            scalar_v64,
            scalar_v66,
            scalar_v67,
            scalar_v69,
            scalar_v72,
            scalar_v73,
            scalar_v75,
            scalar_v77,
            scalar_v79,
            scalar_v81,
            scalar_v83,
            scalar_v85,
            scalar_v87,
            scalar_v89,
            scalar_v91,
            scalar_v93,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v110,
            scalar_v111,
            scalar_v114,
            scalar_v115,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
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
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v162,
            scalar_v163,
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
            scalar_v188,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v205,
            scalar_v206,
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
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v367,
            scalar_v368,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v846,
            scalar_v853,
            scalar_v969,
            scalar_v970,
            scalar_v971,
            scalar_v973,
            scalar_v1001,
            scalar_v1003,
            scalar_v1010,
            scalar_v1108,
            scalar_v2414,
            scalar_v2520,
            scalar_v2521,
            scalar_v2528,
            scalar_v2529,
            scalar_v2530,
            scalar_v2531,
            scalar_v2541,
            scalar_v2542,
            scalar_v2543,
            scalar_v2544,
            scalar_v2569,
            scalar_v2571,
            scalar_v2735,
            scalar_v2738,
            scalar_v2741,
            scalar_v2743,
            scalar_v2744,
            scalar_v2774,
            scalar_v2798,
            scalar_v2800,
            scalar_v2807,
            scalar_v2808,
            scalar_v2876,
            scalar_v2902,
            scalar_v2925,
            scalar_v2930,
            scalar_v2998,
            scalar_v2999,
            scalar_v3000,
            scalar_v3001,
            scalar_v3007,
            scalar_v3008,
            scalar_v3009,
            scalar_v3039,
            scalar_v3062,
            scalar_v3069,
            scalar_v3136,
            scalar_v3162,
            scalar_v3185,
            scalar_v3190,
            scalar_v3480,
            scalar_v3481,
            scalar_v3526,
            scalar_v3527,
            scalar_v6465,
            scalar_v9095,
            scalar_v9096,
            scalar_v9524,
            scalar_v9525,
            scalar_v9647,
            scalar_v9648,
            scalar_v9649,
            scalar_v9650,
            scalar_v9937,
            scalar_v9938,
            scalar_v10125,
            scalar_v10126,
            scalar_v10127,
            scalar_v10128,
            scalar_v10577,
            scalar_v10578,
            scalar_v10579,
            scalar_v10580,
            scalar_v10581,
            scalar_v10582,
            scalar_v10592,
            scalar_v10593,
            scalar_v10594,
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
            scalar_v2,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v24,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v43,
            scalar_v44,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v52,
            scalar_v54,
            scalar_v70,
            scalar_v71,
            scalar_v74,
            scalar_v76,
            scalar_v78,
            scalar_v80,
            scalar_v82,
            scalar_v84,
            scalar_v86,
            scalar_v88,
            scalar_v90,
            scalar_v92,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v190,
            scalar_v191,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v198,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v208,
            scalar_v249,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
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
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v299,
            scalar_v300,
            scalar_v301,
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
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
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
            scalar_v345,
            scalar_v348,
            scalar_v351,
            scalar_v358,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v366,
            scalar_v369,
            scalar_v373,
            scalar_v379,
            scalar_v386,
            scalar_v393,
            scalar_v394,
            scalar_v401,
            scalar_v440,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v832,
            scalar_v1511,
            scalar_v1513,
            scalar_v1514,
            scalar_v2409,
            scalar_v2410,
            scalar_v2467,
            scalar_v2514,
            scalar_v2518,
            scalar_v2519,
            scalar_v2527,
            scalar_v2742,
            scalar_v3264,
            scalar_v3284,
            scalar_v3314,
            scalar_v3315,
            scalar_v10576,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v63,
            scalar_v64,
            scalar_v66,
            scalar_v67,
            scalar_v69,
            scalar_v72,
            scalar_v73,
            scalar_v75,
            scalar_v77,
            scalar_v79,
            scalar_v81,
            scalar_v83,
            scalar_v85,
            scalar_v87,
            scalar_v89,
            scalar_v91,
            scalar_v93,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v110,
            scalar_v111,
            scalar_v114,
            scalar_v115,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v127,
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
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v162,
            scalar_v163,
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
            scalar_v188,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v204,
            scalar_v205,
            scalar_v206,
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
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v349,
            scalar_v350,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v367,
            scalar_v368,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v846,
            scalar_v853,
            scalar_v969,
            scalar_v970,
            scalar_v971,
            scalar_v973,
            scalar_v1001,
            scalar_v1003,
            scalar_v1010,
            scalar_v1108,
            scalar_v2414,
            scalar_v2520,
            scalar_v2521,
            scalar_v2528,
            scalar_v2529,
            scalar_v2530,
            scalar_v2531,
            scalar_v2541,
            scalar_v2542,
            scalar_v2543,
            scalar_v2544,
            scalar_v2569,
            scalar_v2571,
            scalar_v2735,
            scalar_v2738,
            scalar_v2741,
            scalar_v2743,
            scalar_v2744,
            scalar_v2774,
            scalar_v2798,
            scalar_v2800,
            scalar_v2807,
            scalar_v2808,
            scalar_v2876,
            scalar_v2902,
            scalar_v2925,
            scalar_v2930,
            scalar_v2998,
            scalar_v2999,
            scalar_v3000,
            scalar_v3001,
            scalar_v3007,
            scalar_v3008,
            scalar_v3009,
            scalar_v3039,
            scalar_v3062,
            scalar_v3069,
            scalar_v3136,
            scalar_v3162,
            scalar_v3185,
            scalar_v3190,
            scalar_v3480,
            scalar_v3481,
            scalar_v3526,
            scalar_v3527,
            scalar_v6465,
            scalar_v9095,
            scalar_v9096,
            scalar_v9524,
            scalar_v9525,
            scalar_v9647,
            scalar_v9648,
            scalar_v9649,
            scalar_v9650,
            scalar_v9937,
            scalar_v9938,
            scalar_v10125,
            scalar_v10126,
            scalar_v10127,
            scalar_v10128,
            scalar_v10577,
            scalar_v10578,
            scalar_v10579,
            scalar_v10580,
            scalar_v10581,
            scalar_v10582,
            scalar_v10592,
            scalar_v10593,
            scalar_v10594,
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
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subversion" => { validate_finite_parameter("SUBVERSION", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "revision" => { validate_finite_parameter("REVISION", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((-273.0, "-273.0")), false, Some((21.0, "21.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((21.0, "21.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_parameter("LMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_parameter("LMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_parameter("WMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_parameter("WMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swres" => { validate_parameter("SWRES", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "typep" => { validate_parameter("TYPEP", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxo" => { validate_parameter("TOXO", value, Some((5e-10, "5e-10")), false, Some((2e-6, "2e-6")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "epsroxo" => { validate_parameter("EPSROXO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqinv" => { validate_parameter("SWQINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tau" => { validate_parameter("TAU", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubo" => { validate_parameter("NSUBO", value, Some((1e18, "1e18")), false, Some((1e25, "1e25")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mnsubo" => { validate_parameter("MNSUBO", value, Some((1.0, "1.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dnsubo" => { validate_parameter("DNSUBO", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vnsubo" => { validate_parameter("VNSUBO", value, Some((-5.0, "-5.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nslpo" => { validate_parameter("NSLPO", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((1e24, "1e24")), false, Some((1e27, "1e27")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dwr" => { validate_finite_parameter("DWR", value)?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrl" => { validate_parameter("CFRL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrw" => { validate_parameter("CFRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rpv" => { validate_parameter("RPV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rend" => { validate_parameter("REND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "uac" => { validate_parameter("UAC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "uacred" => { validate_parameter("UACRED", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strshg" => { validate_finite_parameter("STRSHG", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strpv" => { validate_finite_parameter("STRPV", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strend" => { validate_finite_parameter("STREND", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strshs" => { validate_finite_parameter("STRSHS", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stuac" => { validate_finite_parameter("STUAC", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chibpo" => { validate_parameter("CHIBPO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lov" => { validate_parameter("LOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1e22, "1e22")), false, Some((1e26, "1e26")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovw" => { validate_parameter("IGOVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2o" => { validate_parameter("GC2O", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3o" => { validate_parameter("GC3O", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igchvlw" => { validate_parameter("IGCHVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovhvw" => { validate_parameter("IGOVHVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcohvo" => { validate_parameter("GCOHVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2hvo" => { validate_parameter("GC2HVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3hvo" => { validate_parameter("GC3HVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igmax" => { validate_parameter("IGMAX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "racnoise" => { validate_parameter("RACNOISE", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'mosvar'", name)),
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
        let v2: f64 = p.p20;
        self.scalar_v2 = v2;
        let v4: f64 = (p.p20 / 3.9);
        self.scalar_v4 = v4;
        let v5: f64 = (3.453e-11 * v4);
        self.scalar_v5 = v5;
        let v6: f64 = p.p19;
        self.scalar_v6 = v6;
        let v7: f64 = (v5 / p.p19);
        self.scalar_v7 = v7;
        let v11: f64 = p.p24;
        self.scalar_v11 = v11;
        let v12: f64 = p.p29;
        self.scalar_v12 = v12;
        let v13: f64 = (3.348580862e-29 * p.p29);
        self.scalar_v13 = v13;
        let v14: f64 = ((v13) as f64).sqrt();
        self.scalar_v14 = v14;
        let v15: f64 = (v14 / v7);
        self.scalar_v15 = v15;
        let v16: f64 = p.p54;
        self.scalar_v16 = v16;
        let v17: f64 = (3.348580862e-29 * p.p54);
        self.scalar_v17 = v17;
        let v18: f64 = ((v17) as f64).sqrt();
        self.scalar_v18 = v18;
        let v19: f64 = (v18 / v7);
        self.scalar_v19 = v19;
        let v20: f64 = p.p30;
        self.scalar_v20 = v20;
        let v22: bool = (p.p30 > 0.0);
        self.scalar_v22 = v22;
        let v24: f64 = (p.p30 * 2.3807972);
        self.scalar_v24 = v24;
        let v26: f64 = f64::powf(v7, 0.6666666666666666);
        self.scalar_v26 = v26;
        let v27: f64 = (v24 * v26);
        self.scalar_v27 = v27;
        let v28: f64 = (if v22 { v27 } else { 0.0 });
        self.scalar_v28 = v28;
        let v29: f64 = p.p17;
        self.scalar_v29 = v29;
        let v30: bool = (p.p17 < 0.0);
        self.scalar_v30 = v30;
        let v31: bool = (v22 && v30);
        self.scalar_v31 = v31;
        let v33: f64 = (v28 * 1.2514650134837189);
        self.scalar_v33 = v33;
        let v34: f64 = (if v31 { v33 } else { v28 });
        self.scalar_v34 = v34;
        let v35: bool = (!v22);
        self.scalar_v35 = v35;
        let v36: f64 = (if v35 { 0.0 } else { v34 });
        self.scalar_v36 = v36;
        let v38: f64 = p.p48;
        self.scalar_v38 = v38;
        let v39: f64 = (0.3333333333333333 * p.p48);
        self.scalar_v39 = v39;
        let v40: f64 = (if v30 { v39 } else { 0.0 });
        self.scalar_v40 = v40;
        let v41: bool = (!v30);
        self.scalar_v41 = v41;
        let v43: f64 = (p.p48 * 0.5);
        self.scalar_v43 = v43;
        let v44: f64 = (if v41 { v43 } else { v40 });
        self.scalar_v44 = v44;
        let v46: f64 = (p.p19 / 1e-9);
        self.scalar_v46 = v46;
        let v47: f64 = p.p11;
        self.scalar_v47 = v47;
        let v49: bool = (p.p11 > -273.0);
        self.scalar_v49 = v49;
        let v50: f64 = (if v49 { p.p11 } else { -273.0 });
        self.scalar_v50 = v50;
        let v52: f64 = (v50 + 273.15);
        self.scalar_v52 = v52;
        let v54: f64 = p.p3;
        self.scalar_v54 = v54;
        let v70: f64 = p.p23;
        self.scalar_v70 = v70;
        let v71: f64 = p.p42;
        self.scalar_v71 = v71;
        let v74: f64 = p.p43;
        self.scalar_v74 = v74;
        let v76: f64 = p.p36;
        self.scalar_v76 = v76;
        let v78: f64 = p.p44;
        self.scalar_v78 = v78;
        let v80: f64 = p.p37;
        self.scalar_v80 = v80;
        let v82: f64 = p.p45;
        self.scalar_v82 = v82;
        let v84: f64 = p.p38;
        self.scalar_v84 = v84;
        let v86: f64 = p.p46;
        self.scalar_v86 = v86;
        let v88: f64 = p.p39;
        self.scalar_v88 = v88;
        let v90: f64 = p.p47;
        self.scalar_v90 = v90;
        let v92: f64 = p.p40;
        self.scalar_v92 = v92;
        let v95: f64 = p.p1;
        self.scalar_v95 = v95;
        let v96: f64 = p.p0;
        self.scalar_v96 = v96;
        let v97: f64 = p.p31;
        self.scalar_v97 = v97;
        let v98: f64 = (p.p1 + p.p31);
        self.scalar_v98 = v98;
        let v99: f64 = p.p32;
        self.scalar_v99 = v99;
        let v100: f64 = (p.p0 + p.p32);
        self.scalar_v100 = v100;
        let v181: f64 = p.p35;
        self.scalar_v181 = v181;
        let v182: f64 = (p.p0 * p.p35);
        self.scalar_v182 = v182;
        let v183: f64 = p.p34;
        self.scalar_v183 = v183;
        let v184: f64 = (p.p1 * p.p34);
        self.scalar_v184 = v184;
        let v185: f64 = (v182 + v184);
        self.scalar_v185 = v185;
        let v186: f64 = (2.0 * v185);
        self.scalar_v186 = v186;
        let v187: f64 = p.p16;
        self.scalar_v187 = v187;
        let v190: f64 = p.p2;
        self.scalar_v190 = v190;
        let v191: f64 = (p.p2 - 1.0);
        self.scalar_v191 = v191;
        let v193: f64 = (v191 * 9.0);
        self.scalar_v193 = v193;
        let v194: f64 = (3.0 + v193);
        self.scalar_v194 = v194;
        let v195: f64 = (p.p1 * v194);
        self.scalar_v195 = v195;
        let v198: f64 = (p.p1 * p.p0);
        self.scalar_v198 = v198;
        let v201: f64 = p.p33;
        self.scalar_v201 = v201;
        let v202: f64 = (p.p0 + p.p33);
        self.scalar_v202 = v202;
        let v203: f64 = (2.0 * v202);
        self.scalar_v203 = v203;
        let v208: f64 = (v202 * 12.0);
        self.scalar_v208 = v208;
        let v249: bool = (!(p.p16 != 0.0));
        self.scalar_v249 = v249;
        let v255: f64 = p.p49;
        self.scalar_v255 = v255;
        let v256: f64 = p.p55;
        self.scalar_v256 = v256;
        let v257: f64 = (v100 * p.p55);
        self.scalar_v257 = v257;
        let v258: f64 = (v98 * v257);
        self.scalar_v258 = v258;
        let v260: f64 = (v258 * 1000000000000.0);
        self.scalar_v260 = v260;
        let v261: f64 = (if (p.p49 != 0.0) { v260 } else { 0.0 });
        self.scalar_v261 = v261;
        let v262: f64 = p.p56;
        self.scalar_v262 = v262;
        let v263: f64 = (2.0 * p.p56);
        self.scalar_v263 = v263;
        let v264: f64 = p.p53;
        self.scalar_v264 = v264;
        let v265: f64 = (v263 * p.p53);
        self.scalar_v265 = v265;
        let v266: f64 = (v100 * v265);
        self.scalar_v266 = v266;
        let v267: f64 = (1000000000000.0 * v266);
        self.scalar_v267 = v267;
        let v268: f64 = (if (p.p49 != 0.0) { v267 } else { 0.0 });
        self.scalar_v268 = v268;
        let v269: f64 = p.p60;
        self.scalar_v269 = v269;
        let v270: f64 = (v100 * p.p60);
        self.scalar_v270 = v270;
        let v271: f64 = (v98 * v270);
        self.scalar_v271 = v271;
        let v272: f64 = (1000000000000.0 * v271);
        self.scalar_v272 = v272;
        let v273: f64 = (if (p.p49 != 0.0) { v272 } else { 0.0 });
        self.scalar_v273 = v273;
        let v274: f64 = p.p61;
        self.scalar_v274 = v274;
        let v275: f64 = (2.0 * p.p61);
        self.scalar_v275 = v275;
        let v276: f64 = (p.p53 * v275);
        self.scalar_v276 = v276;
        let v277: f64 = (v100 * v276);
        self.scalar_v277 = v277;
        let v278: f64 = (1000000000000.0 * v277);
        self.scalar_v278 = v278;
        let v279: f64 = (if (p.p49 != 0.0) { v278 } else { 0.0 });
        self.scalar_v279 = v279;
        let v280: f64 = p.p52;
        self.scalar_v280 = v280;
        let v291: f64 = p.p50;
        self.scalar_v291 = v291;
        let v292: f64 = (1.0 / p.p50);
        self.scalar_v292 = v292;
        let v293: f64 = (if (p.p49 != 0.0) { v292 } else { 0.0 });
        self.scalar_v293 = v293;
        let v294: f64 = p.p51;
        self.scalar_v294 = v294;
        let v295: f64 = (1.0 / p.p51);
        self.scalar_v295 = v295;
        let v296: f64 = (if (p.p49 != 0.0) { v295 } else { 0.0 });
        self.scalar_v296 = v296;
        let v299: f64 = (p.p50 * 2.918995620956536e-49);
        self.scalar_v299 = v299;
        let v300: f64 = ((v299) as f64).sqrt();
        self.scalar_v300 = v300;
        let v301: f64 = (1.3333333333333333 * v300);
        self.scalar_v301 = v301;
        let v303: f64 = (v301 / 1.05457168e-34);
        self.scalar_v303 = v303;
        let v304: f64 = (if (p.p49 != 0.0) { v303 } else { 0.0 });
        self.scalar_v304 = v304;
        let v305: f64 = (p.p19 * v304);
        self.scalar_v305 = v305;
        let v306: f64 = (if (p.p49 != 0.0) { v305 } else { 0.0 });
        self.scalar_v306 = v306;
        let v307: f64 = (if (p.p49 != 0.0) { v306 } else { 0.0 });
        self.scalar_v307 = v307;
        let v308: f64 = (p.p51 * 2.918995620956536e-49);
        self.scalar_v308 = v308;
        let v309: f64 = ((v308) as f64).sqrt();
        self.scalar_v309 = v309;
        let v310: f64 = (1.3333333333333333 * v309);
        self.scalar_v310 = v310;
        let v311: f64 = (v310 / 1.05457168e-34);
        self.scalar_v311 = v311;
        let v312: f64 = (if (p.p49 != 0.0) { v311 } else { v304 });
        self.scalar_v312 = v312;
        let v313: f64 = (p.p19 * v312);
        self.scalar_v313 = v313;
        let v314: f64 = (if (p.p49 != 0.0) { v313 } else { 0.0 });
        self.scalar_v314 = v314;
        let v315: f64 = (if (p.p49 != 0.0) { v314 } else { 0.0 });
        self.scalar_v315 = v315;
        let v316: f64 = p.p59;
        self.scalar_v316 = v316;
        let v317: bool = (p.p59 < 0.0);
        self.scalar_v317 = v317;
        let v318: bool = ((p.p49 != 0.0) && v317);
        self.scalar_v318 = v318;
        let v320: f64 = p.p58;
        self.scalar_v320 = v320;
        let v321: f64 = (-0.495 * p.p58);
        self.scalar_v321 = v321;
        let v322: f64 = (v321 / p.p59);
        self.scalar_v322 = v322;
        let v323: f64 = (if v318 { v322 } else { 0.0 });
        self.scalar_v323 = v323;
        let v324: bool = (!v317);
        self.scalar_v324 = v324;
        let v325: bool = ((p.p49 != 0.0) && v324);
        self.scalar_v325 = v325;
        let v326: f64 = (if v325 { 0.0 } else { v323 });
        self.scalar_v326 = v326;
        let v327: f64 = p.p64;
        self.scalar_v327 = v327;
        let v328: bool = (p.p64 < 0.0);
        self.scalar_v328 = v328;
        let v329: bool = ((p.p49 != 0.0) && v328);
        self.scalar_v329 = v329;
        let v330: f64 = p.p63;
        self.scalar_v330 = v330;
        let v331: f64 = (-0.495 * p.p63);
        self.scalar_v331 = v331;
        let v332: f64 = (v331 / p.p64);
        self.scalar_v332 = v332;
        let v333: f64 = (if v329 { v332 } else { 0.0 });
        self.scalar_v333 = v333;
        let v334: bool = (!v328);
        self.scalar_v334 = v334;
        let v335: bool = ((p.p49 != 0.0) && v334);
        self.scalar_v335 = v335;
        let v336: f64 = (if v335 { 0.0 } else { v333 });
        self.scalar_v336 = v336;
        let v345: f64 = p.p57;
        self.scalar_v345 = v345;
        let v348: f64 = p.p62;
        self.scalar_v348 = v348;
        let v351: bool = (!(p.p49 != 0.0));
        self.scalar_v351 = v351;
        let v358: f64 = (if v351 { 0.0 } else { v326 });
        self.scalar_v358 = v358;
        let v359: f64 = (if v351 { 0.0 } else { v336 });
        self.scalar_v359 = v359;
        let v361: f64 = (if v351 { 0.1 } else { v293 });
        self.scalar_v361 = v361;
        let v362: f64 = (if v351 { 0.1 } else { v296 });
        self.scalar_v362 = v362;
        let v363: f64 = (if v351 { 0.0 } else { v306 });
        self.scalar_v363 = v363;
        let v364: f64 = (if v351 { 0.0 } else { v307 });
        self.scalar_v364 = v364;
        let v365: f64 = (if v351 { 0.0 } else { v314 });
        self.scalar_v365 = v365;
        let v366: f64 = (if v351 { 0.0 } else { v315 });
        self.scalar_v366 = v366;
        let v369: f64 = p.p26;
        self.scalar_v369 = v369;
        let v373: f64 = p.p27;
        self.scalar_v373 = v373;
        let v379: f64 = p.p28;
        self.scalar_v379 = v379;
        let v386: f64 = (0.5 * p.p28);
        self.scalar_v386 = v386;
        let v393: f64 = (p.p28 + 1e-32);
        self.scalar_v393 = v393;
        let v394: f64 = ((v393) as f64).sqrt();
        self.scalar_v394 = v394;
        let v401: f64 = p.p25;
        self.scalar_v401 = v401;
        let v440: f64 = (v36 * 0.75);
        self.scalar_v440 = v440;
        let v829: bool = (p.p29 < 1e27);
        self.scalar_v829 = v829;
        let v830: f64 = (-p.p17);
        self.scalar_v830 = v830;
        let v831: f64 = p.p18;
        self.scalar_v831 = v831;
        let v832: f64 = (v830 * p.p18);
        self.scalar_v832 = v832;
        let v1511: bool = (!v829);
        self.scalar_v1511 = v1511;
        let v1513: f64 = p.p21;
        self.scalar_v1513 = v1513;
        let v1514: bool = (p.p21 < 1.0);
        self.scalar_v1514 = v1514;
        let v2409: f64 = (v46 * 0.37);
        self.scalar_v2409 = v2409;
        let v2410: f64 = (1.0 + v2409);
        self.scalar_v2410 = v2410;
        let v2467: bool = (v36 > 0.0);
        self.scalar_v2467 = v2467;
        let v2514: f64 = p.p41;
        self.scalar_v2514 = v2514;
        let v2518: f64 = (p.p17 * p.p18);
        self.scalar_v2518 = v2518;
        let v2519: bool = (-1.0 == v2518);
        self.scalar_v2519 = v2519;
        let v2527: bool = (0.0 != p.p49);
        self.scalar_v2527 = v2527;
        let v2742: bool = (1.0 == p.p18);
        self.scalar_v2742 = v2742;
        let v3264: f64 = p.p22;
        self.scalar_v3264 = v3264;
        let v3284: f64 = (if v249 { 0.0 } else { 0.0 });
        self.scalar_v3284 = v3284;
        let v3314: f64 = (p.p17 * 0.5);
        self.scalar_v3314 = v3314;
        let v3315: f64 = (0.5 * v830);
        self.scalar_v3315 = v3315;
        let v10576: f64 = (-v186);
        self.scalar_v10576 = v10576;
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
        let v55: f64 = (temperature + self.scalar_v54);
        self.scalar_v55 = v55;
        let v56: f64 = (self.scalar_v55 - 273.15);
        self.scalar_v56 = v56;
        let v57: f64 = (273.15 + self.scalar_v56);
        self.scalar_v57 = v57;
        let v58: f64 = (self.scalar_v57 * self.scalar_v57);
        self.scalar_v58 = v58;
        let v59: f64 = (self.scalar_v57 - self.scalar_v52);
        self.scalar_v59 = v59;
        let v60: f64 = (self.scalar_v57 / self.scalar_v52);
        self.scalar_v60 = v60;
        let v61: f64 = (self.scalar_v52 / self.scalar_v57);
        self.scalar_v61 = v61;
        let v63: f64 = (self.scalar_v57 * 1.3806505e-23);
        self.scalar_v63 = v63;
        let v64: f64 = (self.scalar_v63 / 1.6021918e-19);
        self.scalar_v64 = v64;
        let v66: f64 = (self.scalar_v64 * 100.0);
        self.scalar_v66 = v66;
        let v67: f64 = (self.scalar_v64 * self.scalar_v66);
        self.scalar_v67 = v67;
        let v69: f64 = (1.0 / self.scalar_v64);
        self.scalar_v69 = v69;
        let v72: f64 = (self.scalar_v59 * self.scalar_v71);
        self.scalar_v72 = v72;
        let v73: f64 = (self.scalar_v70 + self.scalar_v72);
        self.scalar_v73 = v73;
        let v75: f64 = f64::powf(self.scalar_v61, self.scalar_v74);
        self.scalar_v75 = v75;
        let v77: f64 = (self.scalar_v75 * self.scalar_v76);
        self.scalar_v77 = v77;
        let v79: f64 = f64::powf(self.scalar_v61, self.scalar_v78);
        self.scalar_v79 = v79;
        let v81: f64 = (self.scalar_v79 * self.scalar_v80);
        self.scalar_v81 = v81;
        let v83: f64 = f64::powf(self.scalar_v61, self.scalar_v82);
        self.scalar_v83 = v83;
        let v85: f64 = (self.scalar_v83 * self.scalar_v84);
        self.scalar_v85 = v85;
        let v87: f64 = f64::powf(self.scalar_v61, self.scalar_v86);
        self.scalar_v87 = v87;
        let v89: f64 = (self.scalar_v87 * self.scalar_v88);
        self.scalar_v89 = v89;
        let v91: f64 = f64::powf(self.scalar_v60, self.scalar_v90);
        self.scalar_v91 = v91;
        let v93: f64 = (self.scalar_v91 * self.scalar_v92);
        self.scalar_v93 = v93;
        let v104: f64 = (self.scalar_v57 * 3.05e-7);
        self.scalar_v104 = v104;
        let v105: f64 = (9.025e-5 + self.scalar_v104);
        self.scalar_v105 = v105;
        let v106: f64 = (self.scalar_v57 * self.scalar_v105);
        self.scalar_v106 = v106;
        let v107: f64 = (1.179 - self.scalar_v106);
        self.scalar_v107 = v107;
        let v110: f64 = (self.scalar_v57 * 0.00045);
        self.scalar_v110 = v110;
        let v111: f64 = (1.045 + self.scalar_v110);
        self.scalar_v111 = v111;
        let v114: f64 = (self.scalar_v57 * 0.0014);
        self.scalar_v114 = v114;
        let v115: f64 = (0.523 + self.scalar_v114);
        self.scalar_v115 = v115;
        let v117: f64 = (self.scalar_v58 * 1.48e-6);
        self.scalar_v117 = v117;
        let v118: f64 = (self.scalar_v115 - self.scalar_v117);
        self.scalar_v118 = v118;
        let v119: f64 = (self.scalar_v111 * self.scalar_v118);
        self.scalar_v119 = v119;
        let v120: f64 = (self.scalar_v58 * self.scalar_v119);
        self.scalar_v120 = v120;
        let v122: f64 = (self.scalar_v120 / 90000.0);
        self.scalar_v122 = v122;
        let v124: bool = (self.scalar_v122 > 0.001);
        self.scalar_v124 = v124;
        let v125: f64 = (if self.scalar_v124 { self.scalar_v122 } else { 0.001 });
        self.scalar_v125 = v125;
        let v126: f64 = ((self.scalar_v125) as f64).sqrt();
        self.scalar_v126 = v126;
        let v127: f64 = ((self.scalar_v126) as f64).sqrt();
        self.scalar_v127 = v127;
        let v129: f64 = (self.scalar_v126 * 2.5e25);
        self.scalar_v129 = v129;
        let v130: f64 = (self.scalar_v127 * self.scalar_v129);
        self.scalar_v130 = v130;
        let v131: f64 = (1.0 / self.scalar_v130);
        self.scalar_v131 = v131;
        let v132: f64 = (2.0 * self.scalar_v64);
        self.scalar_v132 = v132;
        let v133: f64 = (self.scalar_v11 * self.scalar_v131);
        self.scalar_v133 = v133;
        let v134: f64 = ((self.scalar_v133) as f64).ln();
        self.scalar_v134 = v134;
        let v135: f64 = (self.scalar_v132 * self.scalar_v134);
        self.scalar_v135 = v135;
        let v136: f64 = (self.scalar_v107 + self.scalar_v135);
        self.scalar_v136 = v136;
        let v137: f64 = (self.scalar_v12 * self.scalar_v131);
        self.scalar_v137 = v137;
        let v138: f64 = ((self.scalar_v137) as f64).ln();
        self.scalar_v138 = v138;
        let v139: f64 = (self.scalar_v132 * self.scalar_v138);
        self.scalar_v139 = v139;
        let v140: f64 = (self.scalar_v107 + self.scalar_v139);
        self.scalar_v140 = v140;
        let v142: f64 = (self.scalar_v64 * 6.0);
        self.scalar_v142 = v142;
        let v143: f64 = (self.scalar_v107 + self.scalar_v142);
        self.scalar_v143 = v143;
        let v144: f64 = ((self.scalar_v69) as f64).sqrt();
        self.scalar_v144 = v144;
        let v145: f64 = (self.scalar_v15 * self.scalar_v144);
        self.scalar_v145 = v145;
        let v146: f64 = (self.scalar_v145 * self.scalar_v145);
        self.scalar_v146 = v146;
        let v147: f64 = (1.0 / self.scalar_v146);
        self.scalar_v147 = v147;
        let v149: f64 = (self.scalar_v145 * 0.7071067811865475);
        self.scalar_v149 = v149;
        let v150: f64 = (1.0 + self.scalar_v149);
        self.scalar_v150 = v150;
        let v151: f64 = (1.0 / self.scalar_v150);
        self.scalar_v151 = v151;
        let v153: f64 = (self.scalar_v150 * 1e-5);
        self.scalar_v153 = v153;
        let v154: f64 = (self.scalar_v69 * self.scalar_v140);
        self.scalar_v154 = v154;
        let v155: f64 = (self.scalar_v19 * self.scalar_v144);
        self.scalar_v155 = v155;
        let v156: f64 = (self.scalar_v155 * self.scalar_v155);
        self.scalar_v156 = v156;
        let v157: f64 = (0.7071067811865475 * self.scalar_v155);
        self.scalar_v157 = v157;
        let v158: f64 = (1.0 + self.scalar_v157);
        self.scalar_v158 = v158;
        let v159: f64 = (1e-5 * self.scalar_v158);
        self.scalar_v159 = v159;
        let v162: f64 = (self.scalar_v155 * 0.7324648775608221);
        self.scalar_v162 = v162;
        let v163: f64 = (1.25 + self.scalar_v162);
        self.scalar_v163 = v163;
        let v165: bool = (self.scalar_v154 < 460.51701859880916);
        self.scalar_v165 = v165;
        let v166: f64 = (-self.scalar_v154);
        self.scalar_v166 = v166;
        let v167: f64 = ((self.scalar_v166) as f64).exp();
        self.scalar_v167 = v167;
        let v168: f64 = (if self.scalar_v165 { self.scalar_v167 } else { 0.0 });
        self.scalar_v168 = v168;
        let v169: bool = (!self.scalar_v165);
        self.scalar_v169 = v169;
        let v171: f64 = (self.scalar_v154 - 460.51701859880916);
        self.scalar_v171 = v171;
        let v172: f64 = (0.5 * self.scalar_v171);
        self.scalar_v172 = v172;
        let v173: f64 = (0.3333333333333333 * self.scalar_v171);
        self.scalar_v173 = v173;
        let v174: f64 = (1.0 + self.scalar_v173);
        self.scalar_v174 = v174;
        let v175: f64 = (self.scalar_v172 * self.scalar_v174);
        self.scalar_v175 = v175;
        let v176: f64 = (1.0 + self.scalar_v175);
        self.scalar_v176 = v176;
        let v177: f64 = (self.scalar_v171 * self.scalar_v176);
        self.scalar_v177 = v177;
        let v178: f64 = (1.0 + self.scalar_v177);
        self.scalar_v178 = v178;
        let v179: f64 = (1e-200 / self.scalar_v178);
        self.scalar_v179 = v179;
        let v180: f64 = (if self.scalar_v169 { self.scalar_v179 } else { self.scalar_v168 });
        self.scalar_v180 = v180;
        let v188: f64 = (self.scalar_v77 * self.scalar_v96);
        self.scalar_v188 = v188;
        let v196: f64 = (self.scalar_v188 / self.scalar_v195);
        self.scalar_v196 = v196;
        let v197: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v196 } else { 0.0 });
        self.scalar_v197 = v197;
        let v199: f64 = (self.scalar_v81 / self.scalar_v198);
        self.scalar_v199 = v199;
        let v200: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v199 } else { 0.0 });
        self.scalar_v200 = v200;
        let v204: f64 = (self.scalar_v85 / self.scalar_v203);
        self.scalar_v204 = v204;
        let v205: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v204 } else { 0.0 });
        self.scalar_v205 = v205;
        let v206: f64 = (self.scalar_v89 * self.scalar_v95);
        self.scalar_v206 = v206;
        let v209: f64 = (self.scalar_v206 / self.scalar_v208);
        self.scalar_v209 = v209;
        let v210: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v209 } else { 0.0 });
        self.scalar_v210 = v210;
        let v211: bool = (self.scalar_v197 > 0.001);
        self.scalar_v211 = v211;
        let v212: bool = (self.scalar_v197 < 1000.0);
        self.scalar_v212 = v212;
        let v213: f64 = (if self.scalar_v212 { self.scalar_v197 } else { 1000.0 });
        self.scalar_v213 = v213;
        let v214: f64 = (if self.scalar_v211 { self.scalar_v213 } else { 0.001 });
        self.scalar_v214 = v214;
        let v215: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v214 } else { self.scalar_v197 });
        self.scalar_v215 = v215;
        let v216: bool = (self.scalar_v200 > 0.001);
        self.scalar_v216 = v216;
        let v217: bool = (self.scalar_v200 < 100.0);
        self.scalar_v217 = v217;
        let v218: f64 = (if self.scalar_v217 { self.scalar_v200 } else { 100.0 });
        self.scalar_v218 = v218;
        let v219: f64 = (if self.scalar_v216 { self.scalar_v218 } else { 0.001 });
        self.scalar_v219 = v219;
        let v220: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v219 } else { self.scalar_v200 });
        self.scalar_v220 = v220;
        let v221: bool = (self.scalar_v205 > 0.001);
        self.scalar_v221 = v221;
        let v222: bool = (self.scalar_v205 < 1000.0);
        self.scalar_v222 = v222;
        let v223: f64 = (if self.scalar_v222 { self.scalar_v205 } else { 1000.0 });
        self.scalar_v223 = v223;
        let v224: f64 = (if self.scalar_v221 { self.scalar_v223 } else { 0.001 });
        self.scalar_v224 = v224;
        let v225: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v224 } else { self.scalar_v205 });
        self.scalar_v225 = v225;
        let v226: bool = (self.scalar_v210 > 0.001);
        self.scalar_v226 = v226;
        let v227: bool = (self.scalar_v210 < 1000.0);
        self.scalar_v227 = v227;
        let v228: f64 = (if self.scalar_v227 { self.scalar_v210 } else { 1000.0 });
        self.scalar_v228 = v228;
        let v229: f64 = (if self.scalar_v226 { self.scalar_v228 } else { 0.001 });
        self.scalar_v229 = v229;
        let v230: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v229 } else { self.scalar_v210 });
        self.scalar_v230 = v230;
        let v231: bool = (self.scalar_v93 > 0.001);
        self.scalar_v231 = v231;
        let v233: bool = (self.scalar_v93 < 20.0);
        self.scalar_v233 = v233;
        let v234: f64 = (if self.scalar_v233 { self.scalar_v93 } else { 20.0 });
        self.scalar_v234 = v234;
        let v235: f64 = (if self.scalar_v231 { self.scalar_v234 } else { 0.001 });
        self.scalar_v235 = v235;
        let v236: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v235 } else { self.scalar_v93 });
        self.scalar_v236 = v236;
        let v237: f64 = (1.0 / self.scalar_v215);
        self.scalar_v237 = v237;
        let v238: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v237 } else { 0.0 });
        self.scalar_v238 = v238;
        let v239: f64 = (1.0 / self.scalar_v220);
        self.scalar_v239 = v239;
        let v240: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v239 } else { 0.0 });
        self.scalar_v240 = v240;
        let v241: f64 = (1.0 / self.scalar_v225);
        self.scalar_v241 = v241;
        let v242: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v241 } else { 0.0 });
        self.scalar_v242 = v242;
        let v243: f64 = (1.0 / self.scalar_v230);
        self.scalar_v243 = v243;
        let v244: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v243 } else { 0.0 });
        self.scalar_v244 = v244;
        let v245: f64 = (12.0 * self.scalar_v236);
        self.scalar_v245 = v245;
        let v246: f64 = (self.scalar_v96 * self.scalar_v245);
        self.scalar_v246 = v246;
        let v247: f64 = (self.scalar_v246 / self.scalar_v95);
        self.scalar_v247 = v247;
        let v248: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v247 } else { 0.0 });
        self.scalar_v248 = v248;
        let v250: f64 = (if self.scalar_v249 { 0.0 } else { self.scalar_v238 });
        self.scalar_v250 = v250;
        let v251: f64 = (if self.scalar_v249 { 0.0 } else { self.scalar_v240 });
        self.scalar_v251 = v251;
        let v252: f64 = (if self.scalar_v249 { 0.0 } else { self.scalar_v242 });
        self.scalar_v252 = v252;
        let v253: f64 = (if self.scalar_v249 { 0.0 } else { self.scalar_v244 });
        self.scalar_v253 = v253;
        let v254: f64 = (if self.scalar_v249 { 0.0 } else { self.scalar_v248 });
        self.scalar_v254 = v254;
        let v281: f64 = f64::powf(self.scalar_v60, self.scalar_v280);
        self.scalar_v281 = v281;
        let v282: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v281 } else { 0.0 });
        self.scalar_v282 = v282;
        let v283: f64 = (self.scalar_v261 * self.scalar_v282);
        self.scalar_v283 = v283;
        let v284: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v283 } else { self.scalar_v261 });
        self.scalar_v284 = v284;
        let v285: f64 = (self.scalar_v268 * self.scalar_v282);
        self.scalar_v285 = v285;
        let v286: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v285 } else { self.scalar_v268 });
        self.scalar_v286 = v286;
        let v287: f64 = (self.scalar_v273 * self.scalar_v282);
        self.scalar_v287 = v287;
        let v288: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v287 } else { self.scalar_v273 });
        self.scalar_v288 = v288;
        let v289: f64 = (self.scalar_v279 * self.scalar_v282);
        self.scalar_v289 = v289;
        let v290: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v289 } else { self.scalar_v279 });
        self.scalar_v290 = v290;
        let v337: f64 = (self.scalar_v29 * self.scalar_v136);
        self.scalar_v337 = v337;
        let v338: f64 = (self.scalar_v107 + self.scalar_v337);
        self.scalar_v338 = v338;
        let v339: f64 = (0.5 * self.scalar_v338);
        self.scalar_v339 = v339;
        let v340: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v339 } else { 0.0 });
        self.scalar_v340 = v340;
        let v341: f64 = (self.scalar_v29 * self.scalar_v143);
        self.scalar_v341 = v341;
        let v342: f64 = (self.scalar_v107 + self.scalar_v341);
        self.scalar_v342 = v342;
        let v343: f64 = (0.5 * self.scalar_v342);
        self.scalar_v343 = v343;
        let v344: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v343 } else { 0.0 });
        self.scalar_v344 = v344;
        let v346: f64 = (self.scalar_v64 * self.scalar_v345);
        self.scalar_v346 = v346;
        let v347: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v346 } else { 0.0 });
        self.scalar_v347 = v347;
        let v349: f64 = (self.scalar_v64 * self.scalar_v348);
        self.scalar_v349 = v349;
        let v350: f64 = (if (self.scalar_v255 != 0.0) { self.scalar_v349 } else { 0.0 });
        self.scalar_v350 = v350;
        let v352: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v284 });
        self.scalar_v352 = v352;
        let v353: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v286 });
        self.scalar_v353 = v353;
        let v354: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v288 });
        self.scalar_v354 = v354;
        let v355: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v290 });
        self.scalar_v355 = v355;
        let v356: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v347 });
        self.scalar_v356 = v356;
        let v357: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v350 });
        self.scalar_v357 = v357;
        let v367: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v340 });
        self.scalar_v367 = v367;
        let v368: f64 = (if self.scalar_v351 { 0.0 } else { self.scalar_v344 });
        self.scalar_v368 = v368;
        let v841: f64 = (self.scalar_v151 * self.scalar_v151);
        self.scalar_v841 = v841;
        let v842: f64 = (0.1666666666666667 * self.scalar_v841);
        self.scalar_v842 = v842;
        let v843: f64 = (0.7071067811865475 * self.scalar_v842);
        self.scalar_v843 = v843;
        let v846: f64 = (1.0 - self.scalar_v180);
        self.scalar_v846 = v846;
        let v853: f64 = (-self.scalar_v153);
        self.scalar_v853 = v853;
        let v969: f64 = (self.scalar_v145 * 0.7324648775608221);
        self.scalar_v969 = v969;
        let v970: f64 = (1.25 + self.scalar_v969);
        self.scalar_v970 = v970;
        let v971: f64 = (1.0 / self.scalar_v970);
        self.scalar_v971 = v971;
        let v973: f64 = (self.scalar_v150 * 1.25);
        self.scalar_v973 = v973;
        let v1001: f64 = (0.5 * self.scalar_v146);
        self.scalar_v1001 = v1001;
        let v1003: f64 = (self.scalar_v146 * 0.25);
        self.scalar_v1003 = v1003;
        let v1010: f64 = (self.scalar_v154 + 3.0);
        self.scalar_v1010 = v1010;
        let v1108: f64 = (self.scalar_v154 - 230.25850929940458);
        self.scalar_v1108 = v1108;
        let v2414: f64 = ((self.scalar_v61) as f64).sqrt();
        self.scalar_v2414 = v2414;
        let v2520: f64 = (self.scalar_v107 * self.scalar_v831);
        self.scalar_v2520 = v2520;
        let v2521: f64 = (if self.scalar_v2519 { self.scalar_v2520 } else { 0.0 });
        self.scalar_v2521 = v2521;
        let v2528: bool = (self.scalar_v353 > 0.0);
        self.scalar_v2528 = v2528;
        let v2529: bool = (self.scalar_v355 > 0.0);
        self.scalar_v2529 = v2529;
        let v2530: bool = (self.scalar_v2528 || self.scalar_v2529);
        self.scalar_v2530 = v2530;
        let v2531: bool = (self.scalar_v2527 && self.scalar_v2530);
        self.scalar_v2531 = v2531;
        let v2541: f64 = (self.scalar_v158 * 1.25);
        self.scalar_v2541 = v2541;
        let v2542: f64 = (self.scalar_v2541 / self.scalar_v163);
        self.scalar_v2542 = v2542;
        let v2543: f64 = (self.scalar_v2542 - 1.0);
        self.scalar_v2543 = v2543;
        let v2544: f64 = (self.scalar_v2543 / self.scalar_v163);
        self.scalar_v2544 = v2544;
        let v2569: f64 = (0.5 * self.scalar_v156);
        self.scalar_v2569 = v2569;
        let v2571: f64 = (self.scalar_v156 * 0.25);
        self.scalar_v2571 = v2571;
        let v2735: bool = (!self.scalar_v2531);
        self.scalar_v2735 = v2735;
        let v2738: bool = ((self.scalar_v255 != 0.0) && self.scalar_v2530);
        self.scalar_v2738 = v2738;
        let v2741: f64 = (if self.scalar_v2738 { 0.0 } else { 0.0 });
        self.scalar_v2741 = v2741;
        let v2743: bool = (self.scalar_v2529 && self.scalar_v2742);
        self.scalar_v2743 = v2743;
        let v2744: bool = (self.scalar_v2738 && self.scalar_v2743);
        self.scalar_v2744 = v2744;
        let v2774: bool = (self.scalar_v328 && self.scalar_v2744);
        self.scalar_v2774 = v2774;
        let v2798: bool = (self.scalar_v2744 && true);
        self.scalar_v2798 = v2798;
        let v2800: f64 = (self.scalar_v107 - self.scalar_v368);
        self.scalar_v2800 = v2800;
        let v2807: bool = (self.scalar_v2744 && false);
        self.scalar_v2807 = v2807;
        let v2808: f64 = (self.scalar_v107 - self.scalar_v367);
        self.scalar_v2808 = v2808;
        let v2876: bool = (self.scalar_v2528 && self.scalar_v2738);
        self.scalar_v2876 = v2876;
        let v2902: bool = (self.scalar_v317 && self.scalar_v2876);
        self.scalar_v2902 = v2902;
        let v2925: bool = (true && self.scalar_v2876);
        self.scalar_v2925 = v2925;
        let v2930: bool = (false && self.scalar_v2876);
        self.scalar_v2930 = v2930;
        let v2998: bool = (self.scalar_v352 > 0.0);
        self.scalar_v2998 = v2998;
        let v2999: bool = (self.scalar_v354 > 0.0);
        self.scalar_v2999 = v2999;
        let v3000: bool = (self.scalar_v2998 || self.scalar_v2999);
        self.scalar_v3000 = v3000;
        let v3001: bool = ((self.scalar_v255 != 0.0) && self.scalar_v3000);
        self.scalar_v3001 = v3001;
        let v3007: f64 = (if self.scalar_v3001 { 0.0 } else { 0.0 });
        self.scalar_v3007 = v3007;
        let v3008: bool = (self.scalar_v2742 && self.scalar_v2999);
        self.scalar_v3008 = v3008;
        let v3009: bool = (self.scalar_v3001 && self.scalar_v3008);
        self.scalar_v3009 = v3009;
        let v3039: bool = (self.scalar_v328 && self.scalar_v3009);
        self.scalar_v3039 = v3039;
        let v3062: bool = (false && self.scalar_v3009);
        self.scalar_v3062 = v3062;
        let v3069: bool = (true && self.scalar_v3009);
        self.scalar_v3069 = v3069;
        let v3136: bool = (self.scalar_v2998 && self.scalar_v3001);
        self.scalar_v3136 = v3136;
        let v3162: bool = (self.scalar_v317 && self.scalar_v3136);
        self.scalar_v3162 = v3162;
        let v3185: bool = (false && self.scalar_v3136);
        self.scalar_v3185 = v3185;
        let v3190: bool = (true && self.scalar_v3136);
        self.scalar_v3190 = v3190;
        let v3480: f64 = (self.scalar_v29 * self.scalar_v69);
        self.scalar_v3480 = v3480;
        let v3481: f64 = (self.scalar_v69 * self.scalar_v830);
        self.scalar_v3481 = v3481;
        let v3526: f64 = (-self.scalar_v3480);
        self.scalar_v3526 = v3526;
        let v3527: f64 = (-self.scalar_v3481);
        self.scalar_v3527 = v3527;
        let v6465: f64 = (-self.scalar_v69);
        self.scalar_v6465 = v6465;
        let v9095: f64 = (self.scalar_v3481 / self.scalar_v158);
        self.scalar_v9095 = v9095;
        let v9096: f64 = (self.scalar_v3480 / self.scalar_v158);
        self.scalar_v9096 = v9096;
        let v9524: f64 = (if self.scalar_v2738 { self.scalar_v830 } else { 0.0 });
        self.scalar_v9524 = v9524;
        let v9525: f64 = (if self.scalar_v2738 { self.scalar_v29 } else { 0.0 });
        self.scalar_v9525 = v9525;
        let v9647: f64 = (self.scalar_v29 * self.scalar_v9524);
        self.scalar_v9647 = v9647;
        let v9648: f64 = (self.scalar_v29 * self.scalar_v9525);
        self.scalar_v9648 = v9648;
        let v9649: f64 = (self.scalar_v69 * self.scalar_v9647);
        self.scalar_v9649 = v9649;
        let v9650: f64 = (self.scalar_v69 * self.scalar_v9648);
        self.scalar_v9650 = v9650;
        let v9937: f64 = (if self.scalar_v3001 { self.scalar_v29 } else { 0.0 });
        self.scalar_v9937 = v9937;
        let v9938: f64 = (if self.scalar_v3001 { self.scalar_v830 } else { 0.0 });
        self.scalar_v9938 = v9938;
        let v10125: f64 = (self.scalar_v29 * self.scalar_v9937);
        self.scalar_v10125 = v10125;
        let v10126: f64 = (self.scalar_v29 * self.scalar_v9938);
        self.scalar_v10126 = v10126;
        let v10127: f64 = (self.scalar_v69 * self.scalar_v10125);
        self.scalar_v10127 = v10127;
        let v10128: f64 = (self.scalar_v69 * self.scalar_v10126);
        self.scalar_v10128 = v10128;
        let v10577: f64 = (-self.scalar_v250);
        self.scalar_v10577 = v10577;
        let v10578: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v250 } else { 0.0 });
        self.scalar_v10578 = v10578;
        let v10579: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v10577 } else { 0.0 });
        self.scalar_v10579 = v10579;
        let v10580: f64 = (-self.scalar_v251);
        self.scalar_v10580 = v10580;
        let v10581: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v251 } else { 0.0 });
        self.scalar_v10581 = v10581;
        let v10582: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v10580 } else { 0.0 });
        self.scalar_v10582 = v10582;
        let v10592: f64 = (-self.scalar_v252);
        self.scalar_v10592 = v10592;
        let v10593: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v252 } else { 0.0 });
        self.scalar_v10593 = v10593;
        let v10594: f64 = (if (self.scalar_v187 != 0.0) { self.scalar_v10592 } else { 0.0 });
        self.scalar_v10594 = v10594;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
