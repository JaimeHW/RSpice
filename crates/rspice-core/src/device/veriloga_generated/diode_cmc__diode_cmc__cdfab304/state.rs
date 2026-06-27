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
            params.p0 = 2002.0;
            params.p1 = 3.0;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.001;
            validate_parameter("minr", params.p4, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p5 = 1000.0;
            params.p6 = 21.0;
            params.p7 = 1000.0;
            params.p8 = 0.001;
            params.p9 = 1e-9;
            params.p10 = 1e-9;
            params.p11 = 1.0;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 0.5;
            params.p15 = 0.5;
            params.p16 = 0.5;
            params.p17 = 1.16;
            params.p18 = 1.16;
            params.p19 = 1.16;
            params.p20 = 1e-12;
            params.p21 = 1e-18;
            params.p22 = 1e-18;
            params.p23 = 100.0;
            params.p24 = 0.0001;
            params.p25 = 0.0001;
            params.p26 = 1e-7;
            params.p27 = 1e-7;
            params.p28 = 100.0;
            params.p29 = 0.0001;
            params.p30 = 0.0001;
            params.p31 = 0.25;
            params.p32 = 0.25;
            params.p33 = 0.25;
            params.p34 = 1e-12;
            params.p35 = 1e-18;
            params.p36 = 1e-18;
            params.p37 = 1000000000.0;
            params.p38 = 1000000000.0;
            params.p39 = 1000000000.0;
            params.p40 = -0.001;
            params.p41 = -0.001;
            params.p42 = -0.001;
            params.p43 = 10.0;
            params.p44 = 10.0;
            params.p45 = 10.0;
            params.p46 = 4.0;
            params.p47 = 4.0;
            params.p48 = 4.0;
            params.p49 = 0.0;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 1.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 0.0;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 1.0;
            params.p64 = 1.0;
            params.p65 = 1.0;
            params.p66 = 0.0;
            params.p67 = 1.0;
            params.p68 = 0.0;
            params.p69 = 1.0;
            params.p70 = 0.0;
            params.p71 = 1.0;
            params.p72 = -55.0;
            params.p73 = 155.0;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 3.0;
            params.p77 = 1.0;
            params.p78 = 0.0;
            params.p79 = 1e20;
            params.p80 = 1.0;
            params.p81 = 0.0;
            params.p82 = 2.5;
            params.p83 = 0.03;
            params.p84 = 0.0;
            params.p85 = 1.0;
            params.p86 = 0.1;
            params.p87 = 1e16;
            params.p88 = 1e16;
            params.p89 = 1e16;
            params.p90 = 1.0;
            params.p91 = 10.0;
            params.p92 = 5e-9;
            params.p93 = 2e-7;
            params.p94 = 5e-6;
            params.p95 = 0.0;
            params.p96 = 21.0;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 1e-12;
            params.p100 = 1e-6;
            params.p101 = 0.0;
            params.p102 = 0.0;
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
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 103]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 5]>,
    pub(crate) ddt_state_previous: Box<[f64; 5]>,
    pub(crate) ddt_state_initialized: Box<[bool; 5]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v1: f64,
    pub(crate) scalar_v3: bool,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: bool,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: bool,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: bool,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v15: bool,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: bool,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v22: bool,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: bool,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v29: bool,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: bool,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: bool,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: bool,
    pub(crate) scalar_v40: bool,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: bool,
    pub(crate) scalar_v45: bool,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: bool,
    pub(crate) scalar_v50: bool,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v58: bool,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: bool,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: bool,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: bool,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: bool,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: bool,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v77: bool,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: bool,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: bool,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: bool,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: bool,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: bool,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: bool,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v118: bool,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: bool,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: bool,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: bool,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: bool,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: bool,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: bool,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: bool,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: bool,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: bool,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: bool,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: bool,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: bool,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: bool,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: bool,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: bool,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v180: bool,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: bool,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: bool,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: bool,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
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
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v375: f64,
    pub(crate) scalar_v376: f64,
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
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: bool,
    pub(crate) scalar_v511: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v513: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v517: bool,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v521: f64,
    pub(crate) scalar_v522: bool,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v596: bool,
    pub(crate) scalar_v600: bool,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v606: bool,
    pub(crate) scalar_v609: bool,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v615: bool,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v619: f64,
    pub(crate) scalar_v620: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v629: bool,
    pub(crate) scalar_v630: f64,
    pub(crate) scalar_v631: bool,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v634: f64,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v636: f64,
    pub(crate) scalar_v638: bool,
    pub(crate) scalar_v639: f64,
    pub(crate) scalar_v640: bool,
    pub(crate) scalar_v641: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v668: f64,
    pub(crate) scalar_v670: f64,
    pub(crate) scalar_v671: f64,
    pub(crate) scalar_v672: f64,
    pub(crate) scalar_v673: f64,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: bool,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v682: f64,
    pub(crate) scalar_v683: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v686: f64,
    pub(crate) scalar_v687: f64,
    pub(crate) scalar_v688: bool,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v690: f64,
    pub(crate) scalar_v692: bool,
    pub(crate) scalar_v693: bool,
    pub(crate) scalar_v694: bool,
    pub(crate) scalar_v695: bool,
    pub(crate) scalar_v696: bool,
    pub(crate) scalar_v697: f64,
    pub(crate) scalar_v698: f64,
    pub(crate) scalar_v699: bool,
    pub(crate) scalar_v700: bool,
    pub(crate) scalar_v701: bool,
    pub(crate) scalar_v702: bool,
    pub(crate) scalar_v703: bool,
    pub(crate) scalar_v704: f64,
    pub(crate) scalar_v705: f64,
    pub(crate) scalar_v706: bool,
    pub(crate) scalar_v707: bool,
    pub(crate) scalar_v708: bool,
    pub(crate) scalar_v709: bool,
    pub(crate) scalar_v710: bool,
    pub(crate) scalar_v711: f64,
    pub(crate) scalar_v712: bool,
    pub(crate) scalar_v713: bool,
    pub(crate) scalar_v714: bool,
    pub(crate) scalar_v715: f64,
    pub(crate) scalar_v716: bool,
    pub(crate) scalar_v717: f64,
    pub(crate) scalar_v718: bool,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v720: bool,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v725: f64,
    pub(crate) scalar_v727: f64,
    pub(crate) scalar_v728: f64,
    pub(crate) scalar_v729: f64,
    pub(crate) scalar_v730: f64,
    pub(crate) scalar_v731: f64,
    pub(crate) scalar_v732: f64,
    pub(crate) scalar_v733: f64,
    pub(crate) scalar_v734: f64,
    pub(crate) scalar_v735: f64,
    pub(crate) scalar_v736: f64,
    pub(crate) scalar_v737: f64,
    pub(crate) scalar_v739: f64,
    pub(crate) scalar_v740: bool,
    pub(crate) scalar_v741: bool,
    pub(crate) scalar_v742: bool,
    pub(crate) scalar_v743: bool,
    pub(crate) scalar_v805: f64,
    pub(crate) scalar_v806: bool,
    pub(crate) scalar_v808: f64,
    pub(crate) scalar_v819: f64,
    pub(crate) scalar_v820: f64,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v838: f64,
    pub(crate) scalar_v884: bool,
    pub(crate) scalar_v933: bool,
    pub(crate) scalar_v961: f64,
    pub(crate) scalar_v962: f64,
    pub(crate) scalar_v1008: bool,
    pub(crate) scalar_v1057: bool,
    pub(crate) scalar_v1085: f64,
    pub(crate) scalar_v1086: f64,
    pub(crate) scalar_v1132: bool,
    pub(crate) scalar_v1619: bool,
    pub(crate) scalar_v1620: bool,
    pub(crate) scalar_v1631: bool,
    pub(crate) scalar_v1632: bool,
    pub(crate) scalar_v1633: f64,
    pub(crate) scalar_v1659: f64,
    pub(crate) scalar_v1660: f64,
    pub(crate) scalar_v1661: f64,
    pub(crate) scalar_v1662: f64,
    pub(crate) scalar_v1663: f64,
    pub(crate) scalar_v1664: f64,
    pub(crate) scalar_v1665: f64,
    pub(crate) scalar_v1666: f64,
    pub(crate) scalar_v1667: f64,
    pub(crate) scalar_v1668: f64,
    pub(crate) scalar_v1669: f64,
    pub(crate) scalar_v1670: f64,
    pub(crate) scalar_v1671: f64,
    pub(crate) scalar_v1674: f64,
    pub(crate) scalar_v1675: f64,
    pub(crate) scalar_v1676: f64,
    pub(crate) scalar_v1677: f64,
    pub(crate) scalar_v1678: f64,
    pub(crate) scalar_v1679: bool,
    pub(crate) scalar_v1680: bool,
    pub(crate) scalar_v1688: f64,
    pub(crate) scalar_v1689: f64,
    pub(crate) scalar_v1690: bool,
    pub(crate) scalar_v1691: f64,
    pub(crate) scalar_v1692: bool,
    pub(crate) scalar_v1693: bool,
    pub(crate) scalar_v1694: bool,
    pub(crate) scalar_v1695: bool,
    pub(crate) scalar_v1700: bool,
    pub(crate) scalar_v1701: bool,
    pub(crate) scalar_v1712: bool,
    pub(crate) scalar_v1713: bool,
    pub(crate) scalar_v1714: bool,
    pub(crate) scalar_v1715: bool,
    pub(crate) scalar_v1716: f64,
    pub(crate) scalar_v1717: bool,
    pub(crate) scalar_v1718: bool,
    pub(crate) scalar_v1726: bool,
    pub(crate) scalar_v1727: bool,
    pub(crate) scalar_v1728: f64,
    pub(crate) scalar_v1729: bool,
    pub(crate) scalar_v1730: bool,
    pub(crate) scalar_v1737: f64,
    pub(crate) scalar_v1738: f64,
    pub(crate) scalar_v1757: bool,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1759: bool,
    pub(crate) scalar_v1760: bool,
    pub(crate) scalar_v1781: f64,
    pub(crate) scalar_v1782: f64,
    pub(crate) scalar_v1783: bool,
    pub(crate) scalar_v1784: bool,
    pub(crate) scalar_v1789: bool,
    pub(crate) scalar_v1790: bool,
    pub(crate) scalar_v1885: bool,
    pub(crate) scalar_v1886: bool,
    pub(crate) scalar_v1887: f64,
    pub(crate) scalar_v1888: bool,
    pub(crate) scalar_v1889: bool,
    pub(crate) scalar_v1890: bool,
    pub(crate) scalar_v1891: f64,
    pub(crate) scalar_v1892: f64,
    pub(crate) scalar_v1893: f64,
    pub(crate) scalar_v1895: bool,
    pub(crate) scalar_v1896: f64,
    pub(crate) scalar_v1898: f64,
    pub(crate) scalar_v1941: f64,
    pub(crate) scalar_v1942: bool,
    pub(crate) scalar_v1946: f64,
    pub(crate) scalar_v1949: bool,
    pub(crate) scalar_v1960: bool,
    pub(crate) scalar_v1983: bool,
    pub(crate) scalar_v1984: f64,
    pub(crate) scalar_v1985: bool,
    pub(crate) scalar_v1986: bool,
    pub(crate) scalar_v1987: bool,
    pub(crate) scalar_v1988: bool,
    pub(crate) scalar_v1993: bool,
    pub(crate) scalar_v1994: bool,
    pub(crate) scalar_v2004: bool,
    pub(crate) scalar_v2005: bool,
    pub(crate) scalar_v2006: bool,
    pub(crate) scalar_v2007: bool,
    pub(crate) scalar_v2013: bool,
    pub(crate) scalar_v2014: bool,
    pub(crate) scalar_v2022: bool,
    pub(crate) scalar_v2023: bool,
    pub(crate) scalar_v2025: bool,
    pub(crate) scalar_v2026: bool,
    pub(crate) scalar_v2033: f64,
    pub(crate) scalar_v2034: f64,
    pub(crate) scalar_v2052: bool,
    pub(crate) scalar_v2054: bool,
    pub(crate) scalar_v2055: bool,
    pub(crate) scalar_v2075: f64,
    pub(crate) scalar_v2076: f64,
    pub(crate) scalar_v2077: bool,
    pub(crate) scalar_v2078: bool,
    pub(crate) scalar_v2083: bool,
    pub(crate) scalar_v2084: bool,
    pub(crate) scalar_v2177: bool,
    pub(crate) scalar_v2178: bool,
    pub(crate) scalar_v2180: bool,
    pub(crate) scalar_v2181: bool,
    pub(crate) scalar_v2182: bool,
    pub(crate) scalar_v2183: f64,
    pub(crate) scalar_v2184: f64,
    pub(crate) scalar_v2185: f64,
    pub(crate) scalar_v2187: bool,
    pub(crate) scalar_v2188: f64,
    pub(crate) scalar_v2190: f64,
    pub(crate) scalar_v2238: bool,
    pub(crate) scalar_v2249: bool,
    pub(crate) scalar_v2272: bool,
    pub(crate) scalar_v2273: f64,
    pub(crate) scalar_v2274: bool,
    pub(crate) scalar_v2275: bool,
    pub(crate) scalar_v2276: bool,
    pub(crate) scalar_v2277: bool,
    pub(crate) scalar_v2282: bool,
    pub(crate) scalar_v2283: bool,
    pub(crate) scalar_v2293: bool,
    pub(crate) scalar_v2294: bool,
    pub(crate) scalar_v2295: bool,
    pub(crate) scalar_v2296: bool,
    pub(crate) scalar_v2302: bool,
    pub(crate) scalar_v2303: bool,
    pub(crate) scalar_v2311: bool,
    pub(crate) scalar_v2312: bool,
    pub(crate) scalar_v2314: bool,
    pub(crate) scalar_v2315: bool,
    pub(crate) scalar_v2322: f64,
    pub(crate) scalar_v2323: f64,
    pub(crate) scalar_v2341: bool,
    pub(crate) scalar_v2343: bool,
    pub(crate) scalar_v2344: bool,
    pub(crate) scalar_v2364: f64,
    pub(crate) scalar_v2365: f64,
    pub(crate) scalar_v2366: bool,
    pub(crate) scalar_v2367: bool,
    pub(crate) scalar_v2372: bool,
    pub(crate) scalar_v2373: bool,
    pub(crate) scalar_v2466: bool,
    pub(crate) scalar_v2467: bool,
    pub(crate) scalar_v2469: bool,
    pub(crate) scalar_v2470: bool,
    pub(crate) scalar_v2471: bool,
    pub(crate) scalar_v2472: f64,
    pub(crate) scalar_v2473: f64,
    pub(crate) scalar_v2474: f64,
    pub(crate) scalar_v2476: bool,
    pub(crate) scalar_v2477: f64,
    pub(crate) scalar_v2479: f64,
    pub(crate) scalar_v2527: bool,
    pub(crate) scalar_v2538: bool,
    pub(crate) scalar_v3419: bool,
    pub(crate) scalar_v3420: bool,
    pub(crate) scalar_v3431: bool,
    pub(crate) scalar_v3432: bool,
    pub(crate) scalar_v3433: f64,
    pub(crate) scalar_v3457: f64,
    pub(crate) scalar_v3458: f64,
    pub(crate) scalar_v3459: f64,
    pub(crate) scalar_v3460: f64,
    pub(crate) scalar_v3461: f64,
    pub(crate) scalar_v3462: f64,
    pub(crate) scalar_v3463: f64,
    pub(crate) scalar_v3464: f64,
    pub(crate) scalar_v3465: f64,
    pub(crate) scalar_v3466: f64,
    pub(crate) scalar_v3467: f64,
    pub(crate) scalar_v3468: f64,
    pub(crate) scalar_v3469: f64,
    pub(crate) scalar_v3470: f64,
    pub(crate) scalar_v3471: f64,
    pub(crate) scalar_v3472: f64,
    pub(crate) scalar_v3480: f64,
    pub(crate) scalar_v3481: f64,
    pub(crate) scalar_v3652: f64,
    pub(crate) scalar_v3653: f64,
    pub(crate) scalar_v3654: f64,
    pub(crate) scalar_v3656: f64,
    pub(crate) scalar_v3658: f64,
    pub(crate) scalar_v3898: f64,
    pub(crate) scalar_v3899: f64,
    pub(crate) scalar_v3900: f64,
    pub(crate) scalar_v3902: f64,
    pub(crate) scalar_v3904: f64,
    pub(crate) scalar_v4144: f64,
    pub(crate) scalar_v4145: f64,
    pub(crate) scalar_v4146: f64,
    pub(crate) scalar_v4148: f64,
    pub(crate) scalar_v4150: f64,
    pub(crate) scalar_v5080: bool,
    pub(crate) scalar_v5081: bool,
    pub(crate) scalar_v5092: bool,
    pub(crate) scalar_v5093: bool,
    pub(crate) scalar_v5094: f64,
    pub(crate) scalar_v5118: f64,
    pub(crate) scalar_v5119: f64,
    pub(crate) scalar_v5120: f64,
    pub(crate) scalar_v5121: f64,
    pub(crate) scalar_v5122: f64,
    pub(crate) scalar_v5123: f64,
    pub(crate) scalar_v5124: f64,
    pub(crate) scalar_v5125: f64,
    pub(crate) scalar_v5126: f64,
    pub(crate) scalar_v5127: f64,
    pub(crate) scalar_v5128: f64,
    pub(crate) scalar_v5129: f64,
    pub(crate) scalar_v5130: f64,
    pub(crate) scalar_v5131: f64,
    pub(crate) scalar_v5132: f64,
    pub(crate) scalar_v5133: f64,
    pub(crate) scalar_v5141: f64,
    pub(crate) scalar_v5142: f64,
    pub(crate) scalar_v5313: f64,
    pub(crate) scalar_v5314: f64,
    pub(crate) scalar_v5315: f64,
    pub(crate) scalar_v5317: f64,
    pub(crate) scalar_v5319: f64,
    pub(crate) scalar_v5559: f64,
    pub(crate) scalar_v5560: f64,
    pub(crate) scalar_v5561: f64,
    pub(crate) scalar_v5563: f64,
    pub(crate) scalar_v5565: f64,
    pub(crate) scalar_v5805: f64,
    pub(crate) scalar_v5806: f64,
    pub(crate) scalar_v5807: f64,
    pub(crate) scalar_v5809: f64,
    pub(crate) scalar_v5811: f64,
    pub(crate) scalar_v6741: bool,
    pub(crate) scalar_v6742: bool,
    pub(crate) scalar_v6753: bool,
    pub(crate) scalar_v6754: bool,
    pub(crate) scalar_v6755: f64,
    pub(crate) scalar_v6779: f64,
    pub(crate) scalar_v6780: f64,
    pub(crate) scalar_v6781: f64,
    pub(crate) scalar_v6782: f64,
    pub(crate) scalar_v6783: f64,
    pub(crate) scalar_v6784: f64,
    pub(crate) scalar_v6785: f64,
    pub(crate) scalar_v6786: f64,
    pub(crate) scalar_v6787: f64,
    pub(crate) scalar_v6788: f64,
    pub(crate) scalar_v6789: f64,
    pub(crate) scalar_v6790: f64,
    pub(crate) scalar_v6791: f64,
    pub(crate) scalar_v6792: f64,
    pub(crate) scalar_v6793: f64,
    pub(crate) scalar_v6794: f64,
    pub(crate) scalar_v6802: f64,
    pub(crate) scalar_v6803: f64,
    pub(crate) scalar_v6974: f64,
    pub(crate) scalar_v6975: f64,
    pub(crate) scalar_v6976: f64,
    pub(crate) scalar_v6978: f64,
    pub(crate) scalar_v6980: f64,
    pub(crate) scalar_v7220: f64,
    pub(crate) scalar_v7221: f64,
    pub(crate) scalar_v7222: f64,
    pub(crate) scalar_v7224: f64,
    pub(crate) scalar_v7226: f64,
    pub(crate) scalar_v7466: f64,
    pub(crate) scalar_v7467: f64,
    pub(crate) scalar_v7468: f64,
    pub(crate) scalar_v7470: f64,
    pub(crate) scalar_v7472: f64,
    pub(crate) scalar_v8402: bool,
    pub(crate) scalar_v8403: bool,
    pub(crate) scalar_v8414: bool,
    pub(crate) scalar_v8415: bool,
    pub(crate) scalar_v8416: f64,
    pub(crate) scalar_v8440: f64,
    pub(crate) scalar_v8441: f64,
    pub(crate) scalar_v8442: f64,
    pub(crate) scalar_v8443: f64,
    pub(crate) scalar_v8444: f64,
    pub(crate) scalar_v8445: f64,
    pub(crate) scalar_v8446: f64,
    pub(crate) scalar_v8447: f64,
    pub(crate) scalar_v8448: f64,
    pub(crate) scalar_v8449: f64,
    pub(crate) scalar_v8450: f64,
    pub(crate) scalar_v8451: f64,
    pub(crate) scalar_v8452: f64,
    pub(crate) scalar_v8453: f64,
    pub(crate) scalar_v8454: f64,
    pub(crate) scalar_v8455: f64,
    pub(crate) scalar_v8463: f64,
    pub(crate) scalar_v8464: f64,
    pub(crate) scalar_v8635: f64,
    pub(crate) scalar_v8636: f64,
    pub(crate) scalar_v8637: f64,
    pub(crate) scalar_v8639: f64,
    pub(crate) scalar_v8641: f64,
    pub(crate) scalar_v8881: f64,
    pub(crate) scalar_v8882: f64,
    pub(crate) scalar_v8883: f64,
    pub(crate) scalar_v8885: f64,
    pub(crate) scalar_v8887: f64,
    pub(crate) scalar_v9127: f64,
    pub(crate) scalar_v9128: f64,
    pub(crate) scalar_v9129: f64,
    pub(crate) scalar_v9131: f64,
    pub(crate) scalar_v9133: f64,
    pub(crate) scalar_v9242: f64,
    pub(crate) scalar_v9309: f64,
    pub(crate) scalar_v9312: f64,
    pub(crate) scalar_v9313: f64,
    pub(crate) scalar_v9321: f64,
    pub(crate) scalar_v9338: f64,
    pub(crate) scalar_v9376: f64,
    pub(crate) scalar_v9556: bool,
    pub(crate) scalar_v9557: bool,
    pub(crate) scalar_v10461: bool,
    pub(crate) scalar_v10471: bool,
    pub(crate) scalar_v10475: bool,
    pub(crate) scalar_v10476: bool,
    pub(crate) scalar_v10481: bool,
    pub(crate) scalar_v10492: bool,
    pub(crate) scalar_v10493: f64,
    pub(crate) scalar_v10494: bool,
    pub(crate) scalar_v10502: bool,
    pub(crate) scalar_v10503: f64,
    pub(crate) scalar_v10504: bool,
    pub(crate) scalar_v10529: bool,
    pub(crate) scalar_v10530: f64,
    pub(crate) scalar_v10531: bool,
    pub(crate) scalar_v10550: bool,
    pub(crate) scalar_v10555: bool,
    pub(crate) scalar_v10648: bool,
    pub(crate) scalar_v10649: f64,
    pub(crate) scalar_v10650: bool,
    pub(crate) scalar_v10651: bool,
    pub(crate) scalar_v10656: bool,
    pub(crate) scalar_v10733: bool,
    pub(crate) scalar_v10737: bool,
    pub(crate) scalar_v10738: bool,
    pub(crate) scalar_v10743: bool,
    pub(crate) scalar_v10753: bool,
    pub(crate) scalar_v10759: bool,
    pub(crate) scalar_v10767: bool,
    pub(crate) scalar_v10769: bool,
    pub(crate) scalar_v10793: bool,
    pub(crate) scalar_v10795: bool,
    pub(crate) scalar_v10814: bool,
    pub(crate) scalar_v10819: bool,
    pub(crate) scalar_v10912: bool,
    pub(crate) scalar_v10914: bool,
    pub(crate) scalar_v10915: bool,
    pub(crate) scalar_v10920: bool,
    pub(crate) scalar_v10997: bool,
    pub(crate) scalar_v11001: bool,
    pub(crate) scalar_v11002: bool,
    pub(crate) scalar_v11007: bool,
    pub(crate) scalar_v11017: bool,
    pub(crate) scalar_v11023: bool,
    pub(crate) scalar_v11031: bool,
    pub(crate) scalar_v11033: bool,
    pub(crate) scalar_v11057: bool,
    pub(crate) scalar_v11059: bool,
    pub(crate) scalar_v11078: bool,
    pub(crate) scalar_v11083: bool,
    pub(crate) scalar_v11176: bool,
    pub(crate) scalar_v11178: bool,
    pub(crate) scalar_v11179: bool,
    pub(crate) scalar_v11184: bool,
    pub(crate) scalar_v11280: f64,
    pub(crate) scalar_v11281: bool,
    pub(crate) scalar_v11282: bool,
    pub(crate) scalar_v11354: bool,
    pub(crate) scalar_v11405: f64,
    pub(crate) scalar_v11406: bool,
    pub(crate) scalar_v11410: f64,
    pub(crate) scalar_v11415: f64,
    pub(crate) scalar_v11419: f64,
    pub(crate) scalar_v11428: f64,
    pub(crate) scalar_v11437: f64,
    pub(crate) scalar_v11438: bool,
    pub(crate) scalar_v11439: bool,
    pub(crate) scalar_v11441: f64,
    pub(crate) scalar_v11448: bool,
    pub(crate) scalar_v11449: bool,
    pub(crate) scalar_v11513: f64,
    pub(crate) scalar_v11514: bool,
    pub(crate) scalar_v11515: bool,
    pub(crate) scalar_v11516: f64,
    pub(crate) scalar_v11523: bool,
    pub(crate) scalar_v11524: bool,
    pub(crate) scalar_v11527: f64,
    pub(crate) scalar_v11528: f64,
    pub(crate) scalar_v11529: f64,
    pub(crate) scalar_v11530: f64,
    pub(crate) scalar_v11531: f64,
    pub(crate) scalar_v11533: f64,
    pub(crate) scalar_v11556: f64,
    pub(crate) scalar_v11560: f64,
    pub(crate) scalar_v11568: bool,
    pub(crate) scalar_v11569: f64,
    pub(crate) scalar_v11570: bool,
    pub(crate) scalar_v11571: f64,
    pub(crate) scalar_v11671: f64,
    pub(crate) scalar_v11672: f64,
    pub(crate) scalar_v11673: f64,
    pub(crate) scalar_v11674: f64,
    pub(crate) scalar_v11675: f64,
    pub(crate) scalar_v11676: f64,
    pub(crate) scalar_v11677: f64,
    pub(crate) scalar_v11678: f64,
    pub(crate) scalar_v11715: f64,
    pub(crate) scalar_v11745: f64,
    pub(crate) scalar_v11773: f64,
    pub(crate) scalar_v11792: f64,
    pub(crate) scalar_v11793: f64,
    pub(crate) scalar_v11794: f64,
    pub(crate) scalar_v11795: f64,
    pub(crate) scalar_v11796: f64,
    pub(crate) scalar_v11797: f64,
    pub(crate) scalar_v11798: f64,
    pub(crate) scalar_v11799: f64,
    pub(crate) scalar_v11878: f64,
    pub(crate) scalar_v13605: f64,
    pub(crate) scalar_v13704: f64,
    pub(crate) scalar_v14185: f64,
    pub(crate) scalar_v14286: f64,
    pub(crate) scalar_v14769: f64,
    pub(crate) scalar_v14870: f64,
    pub(crate) scalar_v15282: f64,
    pub(crate) scalar_v15283: f64,
    pub(crate) scalar_v15286: f64,
    pub(crate) scalar_v15287: f64,
    pub(crate) scalar_v15508: f64,
    pub(crate) scalar_v15541: f64,
    pub(crate) scalar_v15542: f64,
    pub(crate) scalar_v15543: f64,
    pub(crate) scalar_v15548: f64,
    pub(crate) scalar_v15589: f64,
    pub(crate) scalar_v15590: f64,
    pub(crate) scalar_v15656: f64,
    pub(crate) scalar_v15657: f64,
    pub(crate) scalar_v15658: f64,
    pub(crate) scalar_v15663: f64,
    pub(crate) scalar_v15669: f64,
    pub(crate) scalar_v15695: f64,
    pub(crate) scratch: Option<Box<GenericScratch<962, 6, 4>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<962, 6, 4>>>,
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
            scalar_v1: self.scalar_v1,
            scalar_v3: self.scalar_v3,
            scalar_v4: self.scalar_v4,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v9: self.scalar_v9,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
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
            scalar_v116: self.scalar_v116,
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
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
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
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v180: self.scalar_v180,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
            scalar_v190: self.scalar_v190,
            scalar_v192: self.scalar_v192,
            scalar_v194: self.scalar_v194,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v225: self.scalar_v225,
            scalar_v227: self.scalar_v227,
            scalar_v233: self.scalar_v233,
            scalar_v239: self.scalar_v239,
            scalar_v245: self.scalar_v245,
            scalar_v250: self.scalar_v250,
            scalar_v255: self.scalar_v255,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
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
            scalar_v356: self.scalar_v356,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
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
            scalar_v408: self.scalar_v408,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v471: self.scalar_v471,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v495: self.scalar_v495,
            scalar_v497: self.scalar_v497,
            scalar_v505: self.scalar_v505,
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
            scalar_v525: self.scalar_v525,
            scalar_v596: self.scalar_v596,
            scalar_v600: self.scalar_v600,
            scalar_v601: self.scalar_v601,
            scalar_v602: self.scalar_v602,
            scalar_v603: self.scalar_v603,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v606: self.scalar_v606,
            scalar_v609: self.scalar_v609,
            scalar_v610: self.scalar_v610,
            scalar_v611: self.scalar_v611,
            scalar_v612: self.scalar_v612,
            scalar_v613: self.scalar_v613,
            scalar_v614: self.scalar_v614,
            scalar_v615: self.scalar_v615,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v629: self.scalar_v629,
            scalar_v630: self.scalar_v630,
            scalar_v631: self.scalar_v631,
            scalar_v632: self.scalar_v632,
            scalar_v634: self.scalar_v634,
            scalar_v635: self.scalar_v635,
            scalar_v636: self.scalar_v636,
            scalar_v638: self.scalar_v638,
            scalar_v639: self.scalar_v639,
            scalar_v640: self.scalar_v640,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v668: self.scalar_v668,
            scalar_v670: self.scalar_v670,
            scalar_v671: self.scalar_v671,
            scalar_v672: self.scalar_v672,
            scalar_v673: self.scalar_v673,
            scalar_v675: self.scalar_v675,
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
            scalar_v692: self.scalar_v692,
            scalar_v693: self.scalar_v693,
            scalar_v694: self.scalar_v694,
            scalar_v695: self.scalar_v695,
            scalar_v696: self.scalar_v696,
            scalar_v697: self.scalar_v697,
            scalar_v698: self.scalar_v698,
            scalar_v699: self.scalar_v699,
            scalar_v700: self.scalar_v700,
            scalar_v701: self.scalar_v701,
            scalar_v702: self.scalar_v702,
            scalar_v703: self.scalar_v703,
            scalar_v704: self.scalar_v704,
            scalar_v705: self.scalar_v705,
            scalar_v706: self.scalar_v706,
            scalar_v707: self.scalar_v707,
            scalar_v708: self.scalar_v708,
            scalar_v709: self.scalar_v709,
            scalar_v710: self.scalar_v710,
            scalar_v711: self.scalar_v711,
            scalar_v712: self.scalar_v712,
            scalar_v713: self.scalar_v713,
            scalar_v714: self.scalar_v714,
            scalar_v715: self.scalar_v715,
            scalar_v716: self.scalar_v716,
            scalar_v717: self.scalar_v717,
            scalar_v718: self.scalar_v718,
            scalar_v719: self.scalar_v719,
            scalar_v720: self.scalar_v720,
            scalar_v721: self.scalar_v721,
            scalar_v723: self.scalar_v723,
            scalar_v725: self.scalar_v725,
            scalar_v727: self.scalar_v727,
            scalar_v728: self.scalar_v728,
            scalar_v729: self.scalar_v729,
            scalar_v730: self.scalar_v730,
            scalar_v731: self.scalar_v731,
            scalar_v732: self.scalar_v732,
            scalar_v733: self.scalar_v733,
            scalar_v734: self.scalar_v734,
            scalar_v735: self.scalar_v735,
            scalar_v736: self.scalar_v736,
            scalar_v737: self.scalar_v737,
            scalar_v739: self.scalar_v739,
            scalar_v740: self.scalar_v740,
            scalar_v741: self.scalar_v741,
            scalar_v742: self.scalar_v742,
            scalar_v743: self.scalar_v743,
            scalar_v805: self.scalar_v805,
            scalar_v806: self.scalar_v806,
            scalar_v808: self.scalar_v808,
            scalar_v819: self.scalar_v819,
            scalar_v820: self.scalar_v820,
            scalar_v837: self.scalar_v837,
            scalar_v838: self.scalar_v838,
            scalar_v884: self.scalar_v884,
            scalar_v933: self.scalar_v933,
            scalar_v961: self.scalar_v961,
            scalar_v962: self.scalar_v962,
            scalar_v1008: self.scalar_v1008,
            scalar_v1057: self.scalar_v1057,
            scalar_v1085: self.scalar_v1085,
            scalar_v1086: self.scalar_v1086,
            scalar_v1132: self.scalar_v1132,
            scalar_v1619: self.scalar_v1619,
            scalar_v1620: self.scalar_v1620,
            scalar_v1631: self.scalar_v1631,
            scalar_v1632: self.scalar_v1632,
            scalar_v1633: self.scalar_v1633,
            scalar_v1659: self.scalar_v1659,
            scalar_v1660: self.scalar_v1660,
            scalar_v1661: self.scalar_v1661,
            scalar_v1662: self.scalar_v1662,
            scalar_v1663: self.scalar_v1663,
            scalar_v1664: self.scalar_v1664,
            scalar_v1665: self.scalar_v1665,
            scalar_v1666: self.scalar_v1666,
            scalar_v1667: self.scalar_v1667,
            scalar_v1668: self.scalar_v1668,
            scalar_v1669: self.scalar_v1669,
            scalar_v1670: self.scalar_v1670,
            scalar_v1671: self.scalar_v1671,
            scalar_v1674: self.scalar_v1674,
            scalar_v1675: self.scalar_v1675,
            scalar_v1676: self.scalar_v1676,
            scalar_v1677: self.scalar_v1677,
            scalar_v1678: self.scalar_v1678,
            scalar_v1679: self.scalar_v1679,
            scalar_v1680: self.scalar_v1680,
            scalar_v1688: self.scalar_v1688,
            scalar_v1689: self.scalar_v1689,
            scalar_v1690: self.scalar_v1690,
            scalar_v1691: self.scalar_v1691,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1694: self.scalar_v1694,
            scalar_v1695: self.scalar_v1695,
            scalar_v1700: self.scalar_v1700,
            scalar_v1701: self.scalar_v1701,
            scalar_v1712: self.scalar_v1712,
            scalar_v1713: self.scalar_v1713,
            scalar_v1714: self.scalar_v1714,
            scalar_v1715: self.scalar_v1715,
            scalar_v1716: self.scalar_v1716,
            scalar_v1717: self.scalar_v1717,
            scalar_v1718: self.scalar_v1718,
            scalar_v1726: self.scalar_v1726,
            scalar_v1727: self.scalar_v1727,
            scalar_v1728: self.scalar_v1728,
            scalar_v1729: self.scalar_v1729,
            scalar_v1730: self.scalar_v1730,
            scalar_v1737: self.scalar_v1737,
            scalar_v1738: self.scalar_v1738,
            scalar_v1757: self.scalar_v1757,
            scalar_v1758: self.scalar_v1758,
            scalar_v1759: self.scalar_v1759,
            scalar_v1760: self.scalar_v1760,
            scalar_v1781: self.scalar_v1781,
            scalar_v1782: self.scalar_v1782,
            scalar_v1783: self.scalar_v1783,
            scalar_v1784: self.scalar_v1784,
            scalar_v1789: self.scalar_v1789,
            scalar_v1790: self.scalar_v1790,
            scalar_v1885: self.scalar_v1885,
            scalar_v1886: self.scalar_v1886,
            scalar_v1887: self.scalar_v1887,
            scalar_v1888: self.scalar_v1888,
            scalar_v1889: self.scalar_v1889,
            scalar_v1890: self.scalar_v1890,
            scalar_v1891: self.scalar_v1891,
            scalar_v1892: self.scalar_v1892,
            scalar_v1893: self.scalar_v1893,
            scalar_v1895: self.scalar_v1895,
            scalar_v1896: self.scalar_v1896,
            scalar_v1898: self.scalar_v1898,
            scalar_v1941: self.scalar_v1941,
            scalar_v1942: self.scalar_v1942,
            scalar_v1946: self.scalar_v1946,
            scalar_v1949: self.scalar_v1949,
            scalar_v1960: self.scalar_v1960,
            scalar_v1983: self.scalar_v1983,
            scalar_v1984: self.scalar_v1984,
            scalar_v1985: self.scalar_v1985,
            scalar_v1986: self.scalar_v1986,
            scalar_v1987: self.scalar_v1987,
            scalar_v1988: self.scalar_v1988,
            scalar_v1993: self.scalar_v1993,
            scalar_v1994: self.scalar_v1994,
            scalar_v2004: self.scalar_v2004,
            scalar_v2005: self.scalar_v2005,
            scalar_v2006: self.scalar_v2006,
            scalar_v2007: self.scalar_v2007,
            scalar_v2013: self.scalar_v2013,
            scalar_v2014: self.scalar_v2014,
            scalar_v2022: self.scalar_v2022,
            scalar_v2023: self.scalar_v2023,
            scalar_v2025: self.scalar_v2025,
            scalar_v2026: self.scalar_v2026,
            scalar_v2033: self.scalar_v2033,
            scalar_v2034: self.scalar_v2034,
            scalar_v2052: self.scalar_v2052,
            scalar_v2054: self.scalar_v2054,
            scalar_v2055: self.scalar_v2055,
            scalar_v2075: self.scalar_v2075,
            scalar_v2076: self.scalar_v2076,
            scalar_v2077: self.scalar_v2077,
            scalar_v2078: self.scalar_v2078,
            scalar_v2083: self.scalar_v2083,
            scalar_v2084: self.scalar_v2084,
            scalar_v2177: self.scalar_v2177,
            scalar_v2178: self.scalar_v2178,
            scalar_v2180: self.scalar_v2180,
            scalar_v2181: self.scalar_v2181,
            scalar_v2182: self.scalar_v2182,
            scalar_v2183: self.scalar_v2183,
            scalar_v2184: self.scalar_v2184,
            scalar_v2185: self.scalar_v2185,
            scalar_v2187: self.scalar_v2187,
            scalar_v2188: self.scalar_v2188,
            scalar_v2190: self.scalar_v2190,
            scalar_v2238: self.scalar_v2238,
            scalar_v2249: self.scalar_v2249,
            scalar_v2272: self.scalar_v2272,
            scalar_v2273: self.scalar_v2273,
            scalar_v2274: self.scalar_v2274,
            scalar_v2275: self.scalar_v2275,
            scalar_v2276: self.scalar_v2276,
            scalar_v2277: self.scalar_v2277,
            scalar_v2282: self.scalar_v2282,
            scalar_v2283: self.scalar_v2283,
            scalar_v2293: self.scalar_v2293,
            scalar_v2294: self.scalar_v2294,
            scalar_v2295: self.scalar_v2295,
            scalar_v2296: self.scalar_v2296,
            scalar_v2302: self.scalar_v2302,
            scalar_v2303: self.scalar_v2303,
            scalar_v2311: self.scalar_v2311,
            scalar_v2312: self.scalar_v2312,
            scalar_v2314: self.scalar_v2314,
            scalar_v2315: self.scalar_v2315,
            scalar_v2322: self.scalar_v2322,
            scalar_v2323: self.scalar_v2323,
            scalar_v2341: self.scalar_v2341,
            scalar_v2343: self.scalar_v2343,
            scalar_v2344: self.scalar_v2344,
            scalar_v2364: self.scalar_v2364,
            scalar_v2365: self.scalar_v2365,
            scalar_v2366: self.scalar_v2366,
            scalar_v2367: self.scalar_v2367,
            scalar_v2372: self.scalar_v2372,
            scalar_v2373: self.scalar_v2373,
            scalar_v2466: self.scalar_v2466,
            scalar_v2467: self.scalar_v2467,
            scalar_v2469: self.scalar_v2469,
            scalar_v2470: self.scalar_v2470,
            scalar_v2471: self.scalar_v2471,
            scalar_v2472: self.scalar_v2472,
            scalar_v2473: self.scalar_v2473,
            scalar_v2474: self.scalar_v2474,
            scalar_v2476: self.scalar_v2476,
            scalar_v2477: self.scalar_v2477,
            scalar_v2479: self.scalar_v2479,
            scalar_v2527: self.scalar_v2527,
            scalar_v2538: self.scalar_v2538,
            scalar_v3419: self.scalar_v3419,
            scalar_v3420: self.scalar_v3420,
            scalar_v3431: self.scalar_v3431,
            scalar_v3432: self.scalar_v3432,
            scalar_v3433: self.scalar_v3433,
            scalar_v3457: self.scalar_v3457,
            scalar_v3458: self.scalar_v3458,
            scalar_v3459: self.scalar_v3459,
            scalar_v3460: self.scalar_v3460,
            scalar_v3461: self.scalar_v3461,
            scalar_v3462: self.scalar_v3462,
            scalar_v3463: self.scalar_v3463,
            scalar_v3464: self.scalar_v3464,
            scalar_v3465: self.scalar_v3465,
            scalar_v3466: self.scalar_v3466,
            scalar_v3467: self.scalar_v3467,
            scalar_v3468: self.scalar_v3468,
            scalar_v3469: self.scalar_v3469,
            scalar_v3470: self.scalar_v3470,
            scalar_v3471: self.scalar_v3471,
            scalar_v3472: self.scalar_v3472,
            scalar_v3480: self.scalar_v3480,
            scalar_v3481: self.scalar_v3481,
            scalar_v3652: self.scalar_v3652,
            scalar_v3653: self.scalar_v3653,
            scalar_v3654: self.scalar_v3654,
            scalar_v3656: self.scalar_v3656,
            scalar_v3658: self.scalar_v3658,
            scalar_v3898: self.scalar_v3898,
            scalar_v3899: self.scalar_v3899,
            scalar_v3900: self.scalar_v3900,
            scalar_v3902: self.scalar_v3902,
            scalar_v3904: self.scalar_v3904,
            scalar_v4144: self.scalar_v4144,
            scalar_v4145: self.scalar_v4145,
            scalar_v4146: self.scalar_v4146,
            scalar_v4148: self.scalar_v4148,
            scalar_v4150: self.scalar_v4150,
            scalar_v5080: self.scalar_v5080,
            scalar_v5081: self.scalar_v5081,
            scalar_v5092: self.scalar_v5092,
            scalar_v5093: self.scalar_v5093,
            scalar_v5094: self.scalar_v5094,
            scalar_v5118: self.scalar_v5118,
            scalar_v5119: self.scalar_v5119,
            scalar_v5120: self.scalar_v5120,
            scalar_v5121: self.scalar_v5121,
            scalar_v5122: self.scalar_v5122,
            scalar_v5123: self.scalar_v5123,
            scalar_v5124: self.scalar_v5124,
            scalar_v5125: self.scalar_v5125,
            scalar_v5126: self.scalar_v5126,
            scalar_v5127: self.scalar_v5127,
            scalar_v5128: self.scalar_v5128,
            scalar_v5129: self.scalar_v5129,
            scalar_v5130: self.scalar_v5130,
            scalar_v5131: self.scalar_v5131,
            scalar_v5132: self.scalar_v5132,
            scalar_v5133: self.scalar_v5133,
            scalar_v5141: self.scalar_v5141,
            scalar_v5142: self.scalar_v5142,
            scalar_v5313: self.scalar_v5313,
            scalar_v5314: self.scalar_v5314,
            scalar_v5315: self.scalar_v5315,
            scalar_v5317: self.scalar_v5317,
            scalar_v5319: self.scalar_v5319,
            scalar_v5559: self.scalar_v5559,
            scalar_v5560: self.scalar_v5560,
            scalar_v5561: self.scalar_v5561,
            scalar_v5563: self.scalar_v5563,
            scalar_v5565: self.scalar_v5565,
            scalar_v5805: self.scalar_v5805,
            scalar_v5806: self.scalar_v5806,
            scalar_v5807: self.scalar_v5807,
            scalar_v5809: self.scalar_v5809,
            scalar_v5811: self.scalar_v5811,
            scalar_v6741: self.scalar_v6741,
            scalar_v6742: self.scalar_v6742,
            scalar_v6753: self.scalar_v6753,
            scalar_v6754: self.scalar_v6754,
            scalar_v6755: self.scalar_v6755,
            scalar_v6779: self.scalar_v6779,
            scalar_v6780: self.scalar_v6780,
            scalar_v6781: self.scalar_v6781,
            scalar_v6782: self.scalar_v6782,
            scalar_v6783: self.scalar_v6783,
            scalar_v6784: self.scalar_v6784,
            scalar_v6785: self.scalar_v6785,
            scalar_v6786: self.scalar_v6786,
            scalar_v6787: self.scalar_v6787,
            scalar_v6788: self.scalar_v6788,
            scalar_v6789: self.scalar_v6789,
            scalar_v6790: self.scalar_v6790,
            scalar_v6791: self.scalar_v6791,
            scalar_v6792: self.scalar_v6792,
            scalar_v6793: self.scalar_v6793,
            scalar_v6794: self.scalar_v6794,
            scalar_v6802: self.scalar_v6802,
            scalar_v6803: self.scalar_v6803,
            scalar_v6974: self.scalar_v6974,
            scalar_v6975: self.scalar_v6975,
            scalar_v6976: self.scalar_v6976,
            scalar_v6978: self.scalar_v6978,
            scalar_v6980: self.scalar_v6980,
            scalar_v7220: self.scalar_v7220,
            scalar_v7221: self.scalar_v7221,
            scalar_v7222: self.scalar_v7222,
            scalar_v7224: self.scalar_v7224,
            scalar_v7226: self.scalar_v7226,
            scalar_v7466: self.scalar_v7466,
            scalar_v7467: self.scalar_v7467,
            scalar_v7468: self.scalar_v7468,
            scalar_v7470: self.scalar_v7470,
            scalar_v7472: self.scalar_v7472,
            scalar_v8402: self.scalar_v8402,
            scalar_v8403: self.scalar_v8403,
            scalar_v8414: self.scalar_v8414,
            scalar_v8415: self.scalar_v8415,
            scalar_v8416: self.scalar_v8416,
            scalar_v8440: self.scalar_v8440,
            scalar_v8441: self.scalar_v8441,
            scalar_v8442: self.scalar_v8442,
            scalar_v8443: self.scalar_v8443,
            scalar_v8444: self.scalar_v8444,
            scalar_v8445: self.scalar_v8445,
            scalar_v8446: self.scalar_v8446,
            scalar_v8447: self.scalar_v8447,
            scalar_v8448: self.scalar_v8448,
            scalar_v8449: self.scalar_v8449,
            scalar_v8450: self.scalar_v8450,
            scalar_v8451: self.scalar_v8451,
            scalar_v8452: self.scalar_v8452,
            scalar_v8453: self.scalar_v8453,
            scalar_v8454: self.scalar_v8454,
            scalar_v8455: self.scalar_v8455,
            scalar_v8463: self.scalar_v8463,
            scalar_v8464: self.scalar_v8464,
            scalar_v8635: self.scalar_v8635,
            scalar_v8636: self.scalar_v8636,
            scalar_v8637: self.scalar_v8637,
            scalar_v8639: self.scalar_v8639,
            scalar_v8641: self.scalar_v8641,
            scalar_v8881: self.scalar_v8881,
            scalar_v8882: self.scalar_v8882,
            scalar_v8883: self.scalar_v8883,
            scalar_v8885: self.scalar_v8885,
            scalar_v8887: self.scalar_v8887,
            scalar_v9127: self.scalar_v9127,
            scalar_v9128: self.scalar_v9128,
            scalar_v9129: self.scalar_v9129,
            scalar_v9131: self.scalar_v9131,
            scalar_v9133: self.scalar_v9133,
            scalar_v9242: self.scalar_v9242,
            scalar_v9309: self.scalar_v9309,
            scalar_v9312: self.scalar_v9312,
            scalar_v9313: self.scalar_v9313,
            scalar_v9321: self.scalar_v9321,
            scalar_v9338: self.scalar_v9338,
            scalar_v9376: self.scalar_v9376,
            scalar_v9556: self.scalar_v9556,
            scalar_v9557: self.scalar_v9557,
            scalar_v10461: self.scalar_v10461,
            scalar_v10471: self.scalar_v10471,
            scalar_v10475: self.scalar_v10475,
            scalar_v10476: self.scalar_v10476,
            scalar_v10481: self.scalar_v10481,
            scalar_v10492: self.scalar_v10492,
            scalar_v10493: self.scalar_v10493,
            scalar_v10494: self.scalar_v10494,
            scalar_v10502: self.scalar_v10502,
            scalar_v10503: self.scalar_v10503,
            scalar_v10504: self.scalar_v10504,
            scalar_v10529: self.scalar_v10529,
            scalar_v10530: self.scalar_v10530,
            scalar_v10531: self.scalar_v10531,
            scalar_v10550: self.scalar_v10550,
            scalar_v10555: self.scalar_v10555,
            scalar_v10648: self.scalar_v10648,
            scalar_v10649: self.scalar_v10649,
            scalar_v10650: self.scalar_v10650,
            scalar_v10651: self.scalar_v10651,
            scalar_v10656: self.scalar_v10656,
            scalar_v10733: self.scalar_v10733,
            scalar_v10737: self.scalar_v10737,
            scalar_v10738: self.scalar_v10738,
            scalar_v10743: self.scalar_v10743,
            scalar_v10753: self.scalar_v10753,
            scalar_v10759: self.scalar_v10759,
            scalar_v10767: self.scalar_v10767,
            scalar_v10769: self.scalar_v10769,
            scalar_v10793: self.scalar_v10793,
            scalar_v10795: self.scalar_v10795,
            scalar_v10814: self.scalar_v10814,
            scalar_v10819: self.scalar_v10819,
            scalar_v10912: self.scalar_v10912,
            scalar_v10914: self.scalar_v10914,
            scalar_v10915: self.scalar_v10915,
            scalar_v10920: self.scalar_v10920,
            scalar_v10997: self.scalar_v10997,
            scalar_v11001: self.scalar_v11001,
            scalar_v11002: self.scalar_v11002,
            scalar_v11007: self.scalar_v11007,
            scalar_v11017: self.scalar_v11017,
            scalar_v11023: self.scalar_v11023,
            scalar_v11031: self.scalar_v11031,
            scalar_v11033: self.scalar_v11033,
            scalar_v11057: self.scalar_v11057,
            scalar_v11059: self.scalar_v11059,
            scalar_v11078: self.scalar_v11078,
            scalar_v11083: self.scalar_v11083,
            scalar_v11176: self.scalar_v11176,
            scalar_v11178: self.scalar_v11178,
            scalar_v11179: self.scalar_v11179,
            scalar_v11184: self.scalar_v11184,
            scalar_v11280: self.scalar_v11280,
            scalar_v11281: self.scalar_v11281,
            scalar_v11282: self.scalar_v11282,
            scalar_v11354: self.scalar_v11354,
            scalar_v11405: self.scalar_v11405,
            scalar_v11406: self.scalar_v11406,
            scalar_v11410: self.scalar_v11410,
            scalar_v11415: self.scalar_v11415,
            scalar_v11419: self.scalar_v11419,
            scalar_v11428: self.scalar_v11428,
            scalar_v11437: self.scalar_v11437,
            scalar_v11438: self.scalar_v11438,
            scalar_v11439: self.scalar_v11439,
            scalar_v11441: self.scalar_v11441,
            scalar_v11448: self.scalar_v11448,
            scalar_v11449: self.scalar_v11449,
            scalar_v11513: self.scalar_v11513,
            scalar_v11514: self.scalar_v11514,
            scalar_v11515: self.scalar_v11515,
            scalar_v11516: self.scalar_v11516,
            scalar_v11523: self.scalar_v11523,
            scalar_v11524: self.scalar_v11524,
            scalar_v11527: self.scalar_v11527,
            scalar_v11528: self.scalar_v11528,
            scalar_v11529: self.scalar_v11529,
            scalar_v11530: self.scalar_v11530,
            scalar_v11531: self.scalar_v11531,
            scalar_v11533: self.scalar_v11533,
            scalar_v11556: self.scalar_v11556,
            scalar_v11560: self.scalar_v11560,
            scalar_v11568: self.scalar_v11568,
            scalar_v11569: self.scalar_v11569,
            scalar_v11570: self.scalar_v11570,
            scalar_v11571: self.scalar_v11571,
            scalar_v11671: self.scalar_v11671,
            scalar_v11672: self.scalar_v11672,
            scalar_v11673: self.scalar_v11673,
            scalar_v11674: self.scalar_v11674,
            scalar_v11675: self.scalar_v11675,
            scalar_v11676: self.scalar_v11676,
            scalar_v11677: self.scalar_v11677,
            scalar_v11678: self.scalar_v11678,
            scalar_v11715: self.scalar_v11715,
            scalar_v11745: self.scalar_v11745,
            scalar_v11773: self.scalar_v11773,
            scalar_v11792: self.scalar_v11792,
            scalar_v11793: self.scalar_v11793,
            scalar_v11794: self.scalar_v11794,
            scalar_v11795: self.scalar_v11795,
            scalar_v11796: self.scalar_v11796,
            scalar_v11797: self.scalar_v11797,
            scalar_v11798: self.scalar_v11798,
            scalar_v11799: self.scalar_v11799,
            scalar_v11878: self.scalar_v11878,
            scalar_v13605: self.scalar_v13605,
            scalar_v13704: self.scalar_v13704,
            scalar_v14185: self.scalar_v14185,
            scalar_v14286: self.scalar_v14286,
            scalar_v14769: self.scalar_v14769,
            scalar_v14870: self.scalar_v14870,
            scalar_v15282: self.scalar_v15282,
            scalar_v15283: self.scalar_v15283,
            scalar_v15286: self.scalar_v15286,
            scalar_v15287: self.scalar_v15287,
            scalar_v15508: self.scalar_v15508,
            scalar_v15541: self.scalar_v15541,
            scalar_v15542: self.scalar_v15542,
            scalar_v15543: self.scalar_v15543,
            scalar_v15548: self.scalar_v15548,
            scalar_v15589: self.scalar_v15589,
            scalar_v15590: self.scalar_v15590,
            scalar_v15656: self.scalar_v15656,
            scalar_v15657: self.scalar_v15657,
            scalar_v15658: self.scalar_v15658,
            scalar_v15663: self.scalar_v15663,
            scalar_v15669: self.scalar_v15669,
            scalar_v15695: self.scalar_v15695,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 6;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["aik", "charge_a", "charge_k", "depl_a"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 103;
    pub const VARIABLE_COUNT: usize = 962;
    pub const DDT_STATE_COUNT: usize = 5;
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
            scalar_v1: 0.0,
            scalar_v3: false,
            scalar_v4: 0.0,
            scalar_v5: 0.0,
            scalar_v6: false,
            scalar_v7: 0.0,
            scalar_v8: false,
            scalar_v9: 0.0,
            scalar_v10: false,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v15: false,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: false,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v22: false,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: false,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v29: false,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v32: false,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v35: false,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
            scalar_v38: false,
            scalar_v40: false,
            scalar_v41: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v44: false,
            scalar_v45: false,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v49: false,
            scalar_v50: false,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v58: false,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: false,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: false,
            scalar_v65: 0.0,
            scalar_v66: 0.0,
            scalar_v67: false,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: false,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v73: false,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v77: false,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: false,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v83: false,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: false,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: false,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v93: false,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: false,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: false,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: false,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: false,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v108: false,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v118: false,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: false,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: false,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: false,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v130: false,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: false,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: false,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v140: false,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: false,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: false,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: false,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v152: false,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v161: false,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: false,
            scalar_v165: 0.0,
            scalar_v166: 0.0,
            scalar_v167: false,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: false,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: false,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: false,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v180: false,
            scalar_v182: 0.0,
            scalar_v183: false,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: false,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: false,
            scalar_v190: 0.0,
            scalar_v192: 0.0,
            scalar_v194: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v225: 0.0,
            scalar_v227: 0.0,
            scalar_v233: 0.0,
            scalar_v239: 0.0,
            scalar_v245: 0.0,
            scalar_v250: 0.0,
            scalar_v255: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
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
            scalar_v356: 0.0,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v366: 0.0,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v374: 0.0,
            scalar_v375: 0.0,
            scalar_v376: 0.0,
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
            scalar_v408: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v445: 0.0,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v454: 0.0,
            scalar_v455: 0.0,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v471: 0.0,
            scalar_v473: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v495: 0.0,
            scalar_v497: 0.0,
            scalar_v505: 0.0,
            scalar_v509: 0.0,
            scalar_v510: false,
            scalar_v511: 0.0,
            scalar_v512: 0.0,
            scalar_v513: 0.0,
            scalar_v514: 0.0,
            scalar_v515: 0.0,
            scalar_v516: 0.0,
            scalar_v517: false,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v520: 0.0,
            scalar_v521: 0.0,
            scalar_v522: false,
            scalar_v523: 0.0,
            scalar_v524: 0.0,
            scalar_v525: 0.0,
            scalar_v596: false,
            scalar_v600: false,
            scalar_v601: 0.0,
            scalar_v602: 0.0,
            scalar_v603: 0.0,
            scalar_v604: 0.0,
            scalar_v605: 0.0,
            scalar_v606: false,
            scalar_v609: false,
            scalar_v610: 0.0,
            scalar_v611: 0.0,
            scalar_v612: 0.0,
            scalar_v613: 0.0,
            scalar_v614: 0.0,
            scalar_v615: false,
            scalar_v618: false,
            scalar_v619: 0.0,
            scalar_v620: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v629: false,
            scalar_v630: 0.0,
            scalar_v631: false,
            scalar_v632: 0.0,
            scalar_v634: 0.0,
            scalar_v635: 0.0,
            scalar_v636: 0.0,
            scalar_v638: false,
            scalar_v639: 0.0,
            scalar_v640: false,
            scalar_v641: 0.0,
            scalar_v642: 0.0,
            scalar_v668: 0.0,
            scalar_v670: 0.0,
            scalar_v671: 0.0,
            scalar_v672: 0.0,
            scalar_v673: 0.0,
            scalar_v675: 0.0,
            scalar_v677: 0.0,
            scalar_v678: 0.0,
            scalar_v679: false,
            scalar_v680: 0.0,
            scalar_v681: 0.0,
            scalar_v682: 0.0,
            scalar_v683: 0.0,
            scalar_v684: 0.0,
            scalar_v685: 0.0,
            scalar_v686: 0.0,
            scalar_v687: 0.0,
            scalar_v688: false,
            scalar_v689: 0.0,
            scalar_v690: 0.0,
            scalar_v692: false,
            scalar_v693: false,
            scalar_v694: false,
            scalar_v695: false,
            scalar_v696: false,
            scalar_v697: 0.0,
            scalar_v698: 0.0,
            scalar_v699: false,
            scalar_v700: false,
            scalar_v701: false,
            scalar_v702: false,
            scalar_v703: false,
            scalar_v704: 0.0,
            scalar_v705: 0.0,
            scalar_v706: false,
            scalar_v707: false,
            scalar_v708: false,
            scalar_v709: false,
            scalar_v710: false,
            scalar_v711: 0.0,
            scalar_v712: false,
            scalar_v713: false,
            scalar_v714: false,
            scalar_v715: 0.0,
            scalar_v716: false,
            scalar_v717: 0.0,
            scalar_v718: false,
            scalar_v719: 0.0,
            scalar_v720: false,
            scalar_v721: 0.0,
            scalar_v723: 0.0,
            scalar_v725: 0.0,
            scalar_v727: 0.0,
            scalar_v728: 0.0,
            scalar_v729: 0.0,
            scalar_v730: 0.0,
            scalar_v731: 0.0,
            scalar_v732: 0.0,
            scalar_v733: 0.0,
            scalar_v734: 0.0,
            scalar_v735: 0.0,
            scalar_v736: 0.0,
            scalar_v737: 0.0,
            scalar_v739: 0.0,
            scalar_v740: false,
            scalar_v741: false,
            scalar_v742: false,
            scalar_v743: false,
            scalar_v805: 0.0,
            scalar_v806: false,
            scalar_v808: 0.0,
            scalar_v819: 0.0,
            scalar_v820: 0.0,
            scalar_v837: 0.0,
            scalar_v838: 0.0,
            scalar_v884: false,
            scalar_v933: false,
            scalar_v961: 0.0,
            scalar_v962: 0.0,
            scalar_v1008: false,
            scalar_v1057: false,
            scalar_v1085: 0.0,
            scalar_v1086: 0.0,
            scalar_v1132: false,
            scalar_v1619: false,
            scalar_v1620: false,
            scalar_v1631: false,
            scalar_v1632: false,
            scalar_v1633: 0.0,
            scalar_v1659: 0.0,
            scalar_v1660: 0.0,
            scalar_v1661: 0.0,
            scalar_v1662: 0.0,
            scalar_v1663: 0.0,
            scalar_v1664: 0.0,
            scalar_v1665: 0.0,
            scalar_v1666: 0.0,
            scalar_v1667: 0.0,
            scalar_v1668: 0.0,
            scalar_v1669: 0.0,
            scalar_v1670: 0.0,
            scalar_v1671: 0.0,
            scalar_v1674: 0.0,
            scalar_v1675: 0.0,
            scalar_v1676: 0.0,
            scalar_v1677: 0.0,
            scalar_v1678: 0.0,
            scalar_v1679: false,
            scalar_v1680: false,
            scalar_v1688: 0.0,
            scalar_v1689: 0.0,
            scalar_v1690: false,
            scalar_v1691: 0.0,
            scalar_v1692: false,
            scalar_v1693: false,
            scalar_v1694: false,
            scalar_v1695: false,
            scalar_v1700: false,
            scalar_v1701: false,
            scalar_v1712: false,
            scalar_v1713: false,
            scalar_v1714: false,
            scalar_v1715: false,
            scalar_v1716: 0.0,
            scalar_v1717: false,
            scalar_v1718: false,
            scalar_v1726: false,
            scalar_v1727: false,
            scalar_v1728: 0.0,
            scalar_v1729: false,
            scalar_v1730: false,
            scalar_v1737: 0.0,
            scalar_v1738: 0.0,
            scalar_v1757: false,
            scalar_v1758: 0.0,
            scalar_v1759: false,
            scalar_v1760: false,
            scalar_v1781: 0.0,
            scalar_v1782: 0.0,
            scalar_v1783: false,
            scalar_v1784: false,
            scalar_v1789: false,
            scalar_v1790: false,
            scalar_v1885: false,
            scalar_v1886: false,
            scalar_v1887: 0.0,
            scalar_v1888: false,
            scalar_v1889: false,
            scalar_v1890: false,
            scalar_v1891: 0.0,
            scalar_v1892: 0.0,
            scalar_v1893: 0.0,
            scalar_v1895: false,
            scalar_v1896: 0.0,
            scalar_v1898: 0.0,
            scalar_v1941: 0.0,
            scalar_v1942: false,
            scalar_v1946: 0.0,
            scalar_v1949: false,
            scalar_v1960: false,
            scalar_v1983: false,
            scalar_v1984: 0.0,
            scalar_v1985: false,
            scalar_v1986: false,
            scalar_v1987: false,
            scalar_v1988: false,
            scalar_v1993: false,
            scalar_v1994: false,
            scalar_v2004: false,
            scalar_v2005: false,
            scalar_v2006: false,
            scalar_v2007: false,
            scalar_v2013: false,
            scalar_v2014: false,
            scalar_v2022: false,
            scalar_v2023: false,
            scalar_v2025: false,
            scalar_v2026: false,
            scalar_v2033: 0.0,
            scalar_v2034: 0.0,
            scalar_v2052: false,
            scalar_v2054: false,
            scalar_v2055: false,
            scalar_v2075: 0.0,
            scalar_v2076: 0.0,
            scalar_v2077: false,
            scalar_v2078: false,
            scalar_v2083: false,
            scalar_v2084: false,
            scalar_v2177: false,
            scalar_v2178: false,
            scalar_v2180: false,
            scalar_v2181: false,
            scalar_v2182: false,
            scalar_v2183: 0.0,
            scalar_v2184: 0.0,
            scalar_v2185: 0.0,
            scalar_v2187: false,
            scalar_v2188: 0.0,
            scalar_v2190: 0.0,
            scalar_v2238: false,
            scalar_v2249: false,
            scalar_v2272: false,
            scalar_v2273: 0.0,
            scalar_v2274: false,
            scalar_v2275: false,
            scalar_v2276: false,
            scalar_v2277: false,
            scalar_v2282: false,
            scalar_v2283: false,
            scalar_v2293: false,
            scalar_v2294: false,
            scalar_v2295: false,
            scalar_v2296: false,
            scalar_v2302: false,
            scalar_v2303: false,
            scalar_v2311: false,
            scalar_v2312: false,
            scalar_v2314: false,
            scalar_v2315: false,
            scalar_v2322: 0.0,
            scalar_v2323: 0.0,
            scalar_v2341: false,
            scalar_v2343: false,
            scalar_v2344: false,
            scalar_v2364: 0.0,
            scalar_v2365: 0.0,
            scalar_v2366: false,
            scalar_v2367: false,
            scalar_v2372: false,
            scalar_v2373: false,
            scalar_v2466: false,
            scalar_v2467: false,
            scalar_v2469: false,
            scalar_v2470: false,
            scalar_v2471: false,
            scalar_v2472: 0.0,
            scalar_v2473: 0.0,
            scalar_v2474: 0.0,
            scalar_v2476: false,
            scalar_v2477: 0.0,
            scalar_v2479: 0.0,
            scalar_v2527: false,
            scalar_v2538: false,
            scalar_v3419: false,
            scalar_v3420: false,
            scalar_v3431: false,
            scalar_v3432: false,
            scalar_v3433: 0.0,
            scalar_v3457: 0.0,
            scalar_v3458: 0.0,
            scalar_v3459: 0.0,
            scalar_v3460: 0.0,
            scalar_v3461: 0.0,
            scalar_v3462: 0.0,
            scalar_v3463: 0.0,
            scalar_v3464: 0.0,
            scalar_v3465: 0.0,
            scalar_v3466: 0.0,
            scalar_v3467: 0.0,
            scalar_v3468: 0.0,
            scalar_v3469: 0.0,
            scalar_v3470: 0.0,
            scalar_v3471: 0.0,
            scalar_v3472: 0.0,
            scalar_v3480: 0.0,
            scalar_v3481: 0.0,
            scalar_v3652: 0.0,
            scalar_v3653: 0.0,
            scalar_v3654: 0.0,
            scalar_v3656: 0.0,
            scalar_v3658: 0.0,
            scalar_v3898: 0.0,
            scalar_v3899: 0.0,
            scalar_v3900: 0.0,
            scalar_v3902: 0.0,
            scalar_v3904: 0.0,
            scalar_v4144: 0.0,
            scalar_v4145: 0.0,
            scalar_v4146: 0.0,
            scalar_v4148: 0.0,
            scalar_v4150: 0.0,
            scalar_v5080: false,
            scalar_v5081: false,
            scalar_v5092: false,
            scalar_v5093: false,
            scalar_v5094: 0.0,
            scalar_v5118: 0.0,
            scalar_v5119: 0.0,
            scalar_v5120: 0.0,
            scalar_v5121: 0.0,
            scalar_v5122: 0.0,
            scalar_v5123: 0.0,
            scalar_v5124: 0.0,
            scalar_v5125: 0.0,
            scalar_v5126: 0.0,
            scalar_v5127: 0.0,
            scalar_v5128: 0.0,
            scalar_v5129: 0.0,
            scalar_v5130: 0.0,
            scalar_v5131: 0.0,
            scalar_v5132: 0.0,
            scalar_v5133: 0.0,
            scalar_v5141: 0.0,
            scalar_v5142: 0.0,
            scalar_v5313: 0.0,
            scalar_v5314: 0.0,
            scalar_v5315: 0.0,
            scalar_v5317: 0.0,
            scalar_v5319: 0.0,
            scalar_v5559: 0.0,
            scalar_v5560: 0.0,
            scalar_v5561: 0.0,
            scalar_v5563: 0.0,
            scalar_v5565: 0.0,
            scalar_v5805: 0.0,
            scalar_v5806: 0.0,
            scalar_v5807: 0.0,
            scalar_v5809: 0.0,
            scalar_v5811: 0.0,
            scalar_v6741: false,
            scalar_v6742: false,
            scalar_v6753: false,
            scalar_v6754: false,
            scalar_v6755: 0.0,
            scalar_v6779: 0.0,
            scalar_v6780: 0.0,
            scalar_v6781: 0.0,
            scalar_v6782: 0.0,
            scalar_v6783: 0.0,
            scalar_v6784: 0.0,
            scalar_v6785: 0.0,
            scalar_v6786: 0.0,
            scalar_v6787: 0.0,
            scalar_v6788: 0.0,
            scalar_v6789: 0.0,
            scalar_v6790: 0.0,
            scalar_v6791: 0.0,
            scalar_v6792: 0.0,
            scalar_v6793: 0.0,
            scalar_v6794: 0.0,
            scalar_v6802: 0.0,
            scalar_v6803: 0.0,
            scalar_v6974: 0.0,
            scalar_v6975: 0.0,
            scalar_v6976: 0.0,
            scalar_v6978: 0.0,
            scalar_v6980: 0.0,
            scalar_v7220: 0.0,
            scalar_v7221: 0.0,
            scalar_v7222: 0.0,
            scalar_v7224: 0.0,
            scalar_v7226: 0.0,
            scalar_v7466: 0.0,
            scalar_v7467: 0.0,
            scalar_v7468: 0.0,
            scalar_v7470: 0.0,
            scalar_v7472: 0.0,
            scalar_v8402: false,
            scalar_v8403: false,
            scalar_v8414: false,
            scalar_v8415: false,
            scalar_v8416: 0.0,
            scalar_v8440: 0.0,
            scalar_v8441: 0.0,
            scalar_v8442: 0.0,
            scalar_v8443: 0.0,
            scalar_v8444: 0.0,
            scalar_v8445: 0.0,
            scalar_v8446: 0.0,
            scalar_v8447: 0.0,
            scalar_v8448: 0.0,
            scalar_v8449: 0.0,
            scalar_v8450: 0.0,
            scalar_v8451: 0.0,
            scalar_v8452: 0.0,
            scalar_v8453: 0.0,
            scalar_v8454: 0.0,
            scalar_v8455: 0.0,
            scalar_v8463: 0.0,
            scalar_v8464: 0.0,
            scalar_v8635: 0.0,
            scalar_v8636: 0.0,
            scalar_v8637: 0.0,
            scalar_v8639: 0.0,
            scalar_v8641: 0.0,
            scalar_v8881: 0.0,
            scalar_v8882: 0.0,
            scalar_v8883: 0.0,
            scalar_v8885: 0.0,
            scalar_v8887: 0.0,
            scalar_v9127: 0.0,
            scalar_v9128: 0.0,
            scalar_v9129: 0.0,
            scalar_v9131: 0.0,
            scalar_v9133: 0.0,
            scalar_v9242: 0.0,
            scalar_v9309: 0.0,
            scalar_v9312: 0.0,
            scalar_v9313: 0.0,
            scalar_v9321: 0.0,
            scalar_v9338: 0.0,
            scalar_v9376: 0.0,
            scalar_v9556: false,
            scalar_v9557: false,
            scalar_v10461: false,
            scalar_v10471: false,
            scalar_v10475: false,
            scalar_v10476: false,
            scalar_v10481: false,
            scalar_v10492: false,
            scalar_v10493: 0.0,
            scalar_v10494: false,
            scalar_v10502: false,
            scalar_v10503: 0.0,
            scalar_v10504: false,
            scalar_v10529: false,
            scalar_v10530: 0.0,
            scalar_v10531: false,
            scalar_v10550: false,
            scalar_v10555: false,
            scalar_v10648: false,
            scalar_v10649: 0.0,
            scalar_v10650: false,
            scalar_v10651: false,
            scalar_v10656: false,
            scalar_v10733: false,
            scalar_v10737: false,
            scalar_v10738: false,
            scalar_v10743: false,
            scalar_v10753: false,
            scalar_v10759: false,
            scalar_v10767: false,
            scalar_v10769: false,
            scalar_v10793: false,
            scalar_v10795: false,
            scalar_v10814: false,
            scalar_v10819: false,
            scalar_v10912: false,
            scalar_v10914: false,
            scalar_v10915: false,
            scalar_v10920: false,
            scalar_v10997: false,
            scalar_v11001: false,
            scalar_v11002: false,
            scalar_v11007: false,
            scalar_v11017: false,
            scalar_v11023: false,
            scalar_v11031: false,
            scalar_v11033: false,
            scalar_v11057: false,
            scalar_v11059: false,
            scalar_v11078: false,
            scalar_v11083: false,
            scalar_v11176: false,
            scalar_v11178: false,
            scalar_v11179: false,
            scalar_v11184: false,
            scalar_v11280: 0.0,
            scalar_v11281: false,
            scalar_v11282: false,
            scalar_v11354: false,
            scalar_v11405: 0.0,
            scalar_v11406: false,
            scalar_v11410: 0.0,
            scalar_v11415: 0.0,
            scalar_v11419: 0.0,
            scalar_v11428: 0.0,
            scalar_v11437: 0.0,
            scalar_v11438: false,
            scalar_v11439: false,
            scalar_v11441: 0.0,
            scalar_v11448: false,
            scalar_v11449: false,
            scalar_v11513: 0.0,
            scalar_v11514: false,
            scalar_v11515: false,
            scalar_v11516: 0.0,
            scalar_v11523: false,
            scalar_v11524: false,
            scalar_v11527: 0.0,
            scalar_v11528: 0.0,
            scalar_v11529: 0.0,
            scalar_v11530: 0.0,
            scalar_v11531: 0.0,
            scalar_v11533: 0.0,
            scalar_v11556: 0.0,
            scalar_v11560: 0.0,
            scalar_v11568: false,
            scalar_v11569: 0.0,
            scalar_v11570: false,
            scalar_v11571: 0.0,
            scalar_v11671: 0.0,
            scalar_v11672: 0.0,
            scalar_v11673: 0.0,
            scalar_v11674: 0.0,
            scalar_v11675: 0.0,
            scalar_v11676: 0.0,
            scalar_v11677: 0.0,
            scalar_v11678: 0.0,
            scalar_v11715: 0.0,
            scalar_v11745: 0.0,
            scalar_v11773: 0.0,
            scalar_v11792: 0.0,
            scalar_v11793: 0.0,
            scalar_v11794: 0.0,
            scalar_v11795: 0.0,
            scalar_v11796: 0.0,
            scalar_v11797: 0.0,
            scalar_v11798: 0.0,
            scalar_v11799: 0.0,
            scalar_v11878: 0.0,
            scalar_v13605: 0.0,
            scalar_v13704: 0.0,
            scalar_v14185: 0.0,
            scalar_v14286: 0.0,
            scalar_v14769: 0.0,
            scalar_v14870: 0.0,
            scalar_v15282: 0.0,
            scalar_v15283: 0.0,
            scalar_v15286: 0.0,
            scalar_v15287: 0.0,
            scalar_v15508: 0.0,
            scalar_v15541: 0.0,
            scalar_v15542: 0.0,
            scalar_v15543: 0.0,
            scalar_v15548: 0.0,
            scalar_v15589: 0.0,
            scalar_v15590: 0.0,
            scalar_v15656: 0.0,
            scalar_v15657: 0.0,
            scalar_v15658: 0.0,
            scalar_v15663: 0.0,
            scalar_v15669: 0.0,
            scalar_v15695: 0.0,
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
            scalar_v1,
            scalar_v3,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
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
            scalar_v116,
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
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
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
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v180,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v192,
            scalar_v194,
            scalar_v202,
            scalar_v203,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v225,
            scalar_v227,
            scalar_v233,
            scalar_v239,
            scalar_v245,
            scalar_v250,
            scalar_v255,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
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
            scalar_v356,
            scalar_v358,
            scalar_v359,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v374,
            scalar_v375,
            scalar_v376,
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
            scalar_v408,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v452,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v471,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v495,
            scalar_v497,
            scalar_v505,
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
            scalar_v525,
            scalar_v596,
            scalar_v600,
            scalar_v601,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v609,
            scalar_v610,
            scalar_v611,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v629,
            scalar_v630,
            scalar_v631,
            scalar_v632,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v668,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v675,
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
            scalar_v692,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v701,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v705,
            scalar_v706,
            scalar_v707,
            scalar_v708,
            scalar_v709,
            scalar_v710,
            scalar_v711,
            scalar_v712,
            scalar_v713,
            scalar_v714,
            scalar_v715,
            scalar_v716,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v720,
            scalar_v721,
            scalar_v723,
            scalar_v725,
            scalar_v727,
            scalar_v728,
            scalar_v729,
            scalar_v730,
            scalar_v731,
            scalar_v732,
            scalar_v733,
            scalar_v734,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v805,
            scalar_v806,
            scalar_v808,
            scalar_v819,
            scalar_v820,
            scalar_v837,
            scalar_v838,
            scalar_v884,
            scalar_v933,
            scalar_v961,
            scalar_v962,
            scalar_v1008,
            scalar_v1057,
            scalar_v1085,
            scalar_v1086,
            scalar_v1132,
            scalar_v1619,
            scalar_v1620,
            scalar_v1631,
            scalar_v1632,
            scalar_v1633,
            scalar_v1659,
            scalar_v1660,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1664,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1671,
            scalar_v1674,
            scalar_v1675,
            scalar_v1676,
            scalar_v1677,
            scalar_v1678,
            scalar_v1679,
            scalar_v1680,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1700,
            scalar_v1701,
            scalar_v1712,
            scalar_v1713,
            scalar_v1714,
            scalar_v1715,
            scalar_v1716,
            scalar_v1717,
            scalar_v1718,
            scalar_v1726,
            scalar_v1727,
            scalar_v1728,
            scalar_v1729,
            scalar_v1730,
            scalar_v1737,
            scalar_v1738,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1781,
            scalar_v1782,
            scalar_v1783,
            scalar_v1784,
            scalar_v1789,
            scalar_v1790,
            scalar_v1885,
            scalar_v1886,
            scalar_v1887,
            scalar_v1888,
            scalar_v1889,
            scalar_v1890,
            scalar_v1891,
            scalar_v1892,
            scalar_v1893,
            scalar_v1895,
            scalar_v1896,
            scalar_v1898,
            scalar_v1941,
            scalar_v1942,
            scalar_v1946,
            scalar_v1949,
            scalar_v1960,
            scalar_v1983,
            scalar_v1984,
            scalar_v1985,
            scalar_v1986,
            scalar_v1987,
            scalar_v1988,
            scalar_v1993,
            scalar_v1994,
            scalar_v2004,
            scalar_v2005,
            scalar_v2006,
            scalar_v2007,
            scalar_v2013,
            scalar_v2014,
            scalar_v2022,
            scalar_v2023,
            scalar_v2025,
            scalar_v2026,
            scalar_v2033,
            scalar_v2034,
            scalar_v2052,
            scalar_v2054,
            scalar_v2055,
            scalar_v2075,
            scalar_v2076,
            scalar_v2077,
            scalar_v2078,
            scalar_v2083,
            scalar_v2084,
            scalar_v2177,
            scalar_v2178,
            scalar_v2180,
            scalar_v2181,
            scalar_v2182,
            scalar_v2183,
            scalar_v2184,
            scalar_v2185,
            scalar_v2187,
            scalar_v2188,
            scalar_v2190,
            scalar_v2238,
            scalar_v2249,
            scalar_v2272,
            scalar_v2273,
            scalar_v2274,
            scalar_v2275,
            scalar_v2276,
            scalar_v2277,
            scalar_v2282,
            scalar_v2283,
            scalar_v2293,
            scalar_v2294,
            scalar_v2295,
            scalar_v2296,
            scalar_v2302,
            scalar_v2303,
            scalar_v2311,
            scalar_v2312,
            scalar_v2314,
            scalar_v2315,
            scalar_v2322,
            scalar_v2323,
            scalar_v2341,
            scalar_v2343,
            scalar_v2344,
            scalar_v2364,
            scalar_v2365,
            scalar_v2366,
            scalar_v2367,
            scalar_v2372,
            scalar_v2373,
            scalar_v2466,
            scalar_v2467,
            scalar_v2469,
            scalar_v2470,
            scalar_v2471,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2476,
            scalar_v2477,
            scalar_v2479,
            scalar_v2527,
            scalar_v2538,
            scalar_v3419,
            scalar_v3420,
            scalar_v3431,
            scalar_v3432,
            scalar_v3433,
            scalar_v3457,
            scalar_v3458,
            scalar_v3459,
            scalar_v3460,
            scalar_v3461,
            scalar_v3462,
            scalar_v3463,
            scalar_v3464,
            scalar_v3465,
            scalar_v3466,
            scalar_v3467,
            scalar_v3468,
            scalar_v3469,
            scalar_v3470,
            scalar_v3471,
            scalar_v3472,
            scalar_v3480,
            scalar_v3481,
            scalar_v3652,
            scalar_v3653,
            scalar_v3654,
            scalar_v3656,
            scalar_v3658,
            scalar_v3898,
            scalar_v3899,
            scalar_v3900,
            scalar_v3902,
            scalar_v3904,
            scalar_v4144,
            scalar_v4145,
            scalar_v4146,
            scalar_v4148,
            scalar_v4150,
            scalar_v5080,
            scalar_v5081,
            scalar_v5092,
            scalar_v5093,
            scalar_v5094,
            scalar_v5118,
            scalar_v5119,
            scalar_v5120,
            scalar_v5121,
            scalar_v5122,
            scalar_v5123,
            scalar_v5124,
            scalar_v5125,
            scalar_v5126,
            scalar_v5127,
            scalar_v5128,
            scalar_v5129,
            scalar_v5130,
            scalar_v5131,
            scalar_v5132,
            scalar_v5133,
            scalar_v5141,
            scalar_v5142,
            scalar_v5313,
            scalar_v5314,
            scalar_v5315,
            scalar_v5317,
            scalar_v5319,
            scalar_v5559,
            scalar_v5560,
            scalar_v5561,
            scalar_v5563,
            scalar_v5565,
            scalar_v5805,
            scalar_v5806,
            scalar_v5807,
            scalar_v5809,
            scalar_v5811,
            scalar_v6741,
            scalar_v6742,
            scalar_v6753,
            scalar_v6754,
            scalar_v6755,
            scalar_v6779,
            scalar_v6780,
            scalar_v6781,
            scalar_v6782,
            scalar_v6783,
            scalar_v6784,
            scalar_v6785,
            scalar_v6786,
            scalar_v6787,
            scalar_v6788,
            scalar_v6789,
            scalar_v6790,
            scalar_v6791,
            scalar_v6792,
            scalar_v6793,
            scalar_v6794,
            scalar_v6802,
            scalar_v6803,
            scalar_v6974,
            scalar_v6975,
            scalar_v6976,
            scalar_v6978,
            scalar_v6980,
            scalar_v7220,
            scalar_v7221,
            scalar_v7222,
            scalar_v7224,
            scalar_v7226,
            scalar_v7466,
            scalar_v7467,
            scalar_v7468,
            scalar_v7470,
            scalar_v7472,
            scalar_v8402,
            scalar_v8403,
            scalar_v8414,
            scalar_v8415,
            scalar_v8416,
            scalar_v8440,
            scalar_v8441,
            scalar_v8442,
            scalar_v8443,
            scalar_v8444,
            scalar_v8445,
            scalar_v8446,
            scalar_v8447,
            scalar_v8448,
            scalar_v8449,
            scalar_v8450,
            scalar_v8451,
            scalar_v8452,
            scalar_v8453,
            scalar_v8454,
            scalar_v8455,
            scalar_v8463,
            scalar_v8464,
            scalar_v8635,
            scalar_v8636,
            scalar_v8637,
            scalar_v8639,
            scalar_v8641,
            scalar_v8881,
            scalar_v8882,
            scalar_v8883,
            scalar_v8885,
            scalar_v8887,
            scalar_v9127,
            scalar_v9128,
            scalar_v9129,
            scalar_v9131,
            scalar_v9133,
            scalar_v9242,
            scalar_v9309,
            scalar_v9312,
            scalar_v9313,
            scalar_v9321,
            scalar_v9338,
            scalar_v9376,
            scalar_v9556,
            scalar_v9557,
            scalar_v10461,
            scalar_v10471,
            scalar_v10475,
            scalar_v10476,
            scalar_v10481,
            scalar_v10492,
            scalar_v10493,
            scalar_v10494,
            scalar_v10502,
            scalar_v10503,
            scalar_v10504,
            scalar_v10529,
            scalar_v10530,
            scalar_v10531,
            scalar_v10550,
            scalar_v10555,
            scalar_v10648,
            scalar_v10649,
            scalar_v10650,
            scalar_v10651,
            scalar_v10656,
            scalar_v10733,
            scalar_v10737,
            scalar_v10738,
            scalar_v10743,
            scalar_v10753,
            scalar_v10759,
            scalar_v10767,
            scalar_v10769,
            scalar_v10793,
            scalar_v10795,
            scalar_v10814,
            scalar_v10819,
            scalar_v10912,
            scalar_v10914,
            scalar_v10915,
            scalar_v10920,
            scalar_v10997,
            scalar_v11001,
            scalar_v11002,
            scalar_v11007,
            scalar_v11017,
            scalar_v11023,
            scalar_v11031,
            scalar_v11033,
            scalar_v11057,
            scalar_v11059,
            scalar_v11078,
            scalar_v11083,
            scalar_v11176,
            scalar_v11178,
            scalar_v11179,
            scalar_v11184,
            scalar_v11280,
            scalar_v11281,
            scalar_v11282,
            scalar_v11354,
            scalar_v11405,
            scalar_v11406,
            scalar_v11410,
            scalar_v11415,
            scalar_v11419,
            scalar_v11428,
            scalar_v11437,
            scalar_v11438,
            scalar_v11439,
            scalar_v11441,
            scalar_v11448,
            scalar_v11449,
            scalar_v11513,
            scalar_v11514,
            scalar_v11515,
            scalar_v11516,
            scalar_v11523,
            scalar_v11524,
            scalar_v11527,
            scalar_v11528,
            scalar_v11529,
            scalar_v11530,
            scalar_v11531,
            scalar_v11533,
            scalar_v11556,
            scalar_v11560,
            scalar_v11568,
            scalar_v11569,
            scalar_v11570,
            scalar_v11571,
            scalar_v11671,
            scalar_v11672,
            scalar_v11673,
            scalar_v11674,
            scalar_v11675,
            scalar_v11676,
            scalar_v11677,
            scalar_v11678,
            scalar_v11715,
            scalar_v11745,
            scalar_v11773,
            scalar_v11792,
            scalar_v11793,
            scalar_v11794,
            scalar_v11795,
            scalar_v11796,
            scalar_v11797,
            scalar_v11798,
            scalar_v11799,
            scalar_v11878,
            scalar_v13605,
            scalar_v13704,
            scalar_v14185,
            scalar_v14286,
            scalar_v14769,
            scalar_v14870,
            scalar_v15282,
            scalar_v15283,
            scalar_v15286,
            scalar_v15287,
            scalar_v15508,
            scalar_v15541,
            scalar_v15542,
            scalar_v15543,
            scalar_v15548,
            scalar_v15589,
            scalar_v15590,
            scalar_v15656,
            scalar_v15657,
            scalar_v15658,
            scalar_v15663,
            scalar_v15669,
            scalar_v15695,
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
            scalar_v1,
            scalar_v3,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
            scalar_v37,
            scalar_v38,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
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
            scalar_v116,
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
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
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
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v180,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v192,
            scalar_v194,
            scalar_v202,
            scalar_v203,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v225,
            scalar_v227,
            scalar_v233,
            scalar_v239,
            scalar_v245,
            scalar_v250,
            scalar_v255,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
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
            scalar_v356,
            scalar_v358,
            scalar_v359,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v374,
            scalar_v375,
            scalar_v376,
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
            scalar_v408,
            scalar_v443,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v452,
            scalar_v453,
            scalar_v454,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v471,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v495,
            scalar_v497,
            scalar_v505,
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
            scalar_v525,
            scalar_v596,
            scalar_v600,
            scalar_v601,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v609,
            scalar_v610,
            scalar_v611,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v629,
            scalar_v630,
            scalar_v631,
            scalar_v632,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v668,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v675,
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
            scalar_v692,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v701,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v705,
            scalar_v706,
            scalar_v707,
            scalar_v708,
            scalar_v709,
            scalar_v710,
            scalar_v711,
            scalar_v712,
            scalar_v713,
            scalar_v714,
            scalar_v715,
            scalar_v716,
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v720,
            scalar_v721,
            scalar_v723,
            scalar_v725,
            scalar_v727,
            scalar_v728,
            scalar_v729,
            scalar_v730,
            scalar_v731,
            scalar_v732,
            scalar_v733,
            scalar_v734,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v805,
            scalar_v806,
            scalar_v808,
            scalar_v819,
            scalar_v820,
            scalar_v837,
            scalar_v838,
            scalar_v884,
            scalar_v933,
            scalar_v961,
            scalar_v962,
            scalar_v1008,
            scalar_v1057,
            scalar_v1085,
            scalar_v1086,
            scalar_v1132,
            scalar_v1619,
            scalar_v1620,
            scalar_v1631,
            scalar_v1632,
            scalar_v1633,
            scalar_v1659,
            scalar_v1660,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1664,
            scalar_v1665,
            scalar_v1666,
            scalar_v1667,
            scalar_v1668,
            scalar_v1669,
            scalar_v1670,
            scalar_v1671,
            scalar_v1674,
            scalar_v1675,
            scalar_v1676,
            scalar_v1677,
            scalar_v1678,
            scalar_v1679,
            scalar_v1680,
            scalar_v1688,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1695,
            scalar_v1700,
            scalar_v1701,
            scalar_v1712,
            scalar_v1713,
            scalar_v1714,
            scalar_v1715,
            scalar_v1716,
            scalar_v1717,
            scalar_v1718,
            scalar_v1726,
            scalar_v1727,
            scalar_v1728,
            scalar_v1729,
            scalar_v1730,
            scalar_v1737,
            scalar_v1738,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1760,
            scalar_v1781,
            scalar_v1782,
            scalar_v1783,
            scalar_v1784,
            scalar_v1789,
            scalar_v1790,
            scalar_v1885,
            scalar_v1886,
            scalar_v1887,
            scalar_v1888,
            scalar_v1889,
            scalar_v1890,
            scalar_v1891,
            scalar_v1892,
            scalar_v1893,
            scalar_v1895,
            scalar_v1896,
            scalar_v1898,
            scalar_v1941,
            scalar_v1942,
            scalar_v1946,
            scalar_v1949,
            scalar_v1960,
            scalar_v1983,
            scalar_v1984,
            scalar_v1985,
            scalar_v1986,
            scalar_v1987,
            scalar_v1988,
            scalar_v1993,
            scalar_v1994,
            scalar_v2004,
            scalar_v2005,
            scalar_v2006,
            scalar_v2007,
            scalar_v2013,
            scalar_v2014,
            scalar_v2022,
            scalar_v2023,
            scalar_v2025,
            scalar_v2026,
            scalar_v2033,
            scalar_v2034,
            scalar_v2052,
            scalar_v2054,
            scalar_v2055,
            scalar_v2075,
            scalar_v2076,
            scalar_v2077,
            scalar_v2078,
            scalar_v2083,
            scalar_v2084,
            scalar_v2177,
            scalar_v2178,
            scalar_v2180,
            scalar_v2181,
            scalar_v2182,
            scalar_v2183,
            scalar_v2184,
            scalar_v2185,
            scalar_v2187,
            scalar_v2188,
            scalar_v2190,
            scalar_v2238,
            scalar_v2249,
            scalar_v2272,
            scalar_v2273,
            scalar_v2274,
            scalar_v2275,
            scalar_v2276,
            scalar_v2277,
            scalar_v2282,
            scalar_v2283,
            scalar_v2293,
            scalar_v2294,
            scalar_v2295,
            scalar_v2296,
            scalar_v2302,
            scalar_v2303,
            scalar_v2311,
            scalar_v2312,
            scalar_v2314,
            scalar_v2315,
            scalar_v2322,
            scalar_v2323,
            scalar_v2341,
            scalar_v2343,
            scalar_v2344,
            scalar_v2364,
            scalar_v2365,
            scalar_v2366,
            scalar_v2367,
            scalar_v2372,
            scalar_v2373,
            scalar_v2466,
            scalar_v2467,
            scalar_v2469,
            scalar_v2470,
            scalar_v2471,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2476,
            scalar_v2477,
            scalar_v2479,
            scalar_v2527,
            scalar_v2538,
            scalar_v3419,
            scalar_v3420,
            scalar_v3431,
            scalar_v3432,
            scalar_v3433,
            scalar_v3457,
            scalar_v3458,
            scalar_v3459,
            scalar_v3460,
            scalar_v3461,
            scalar_v3462,
            scalar_v3463,
            scalar_v3464,
            scalar_v3465,
            scalar_v3466,
            scalar_v3467,
            scalar_v3468,
            scalar_v3469,
            scalar_v3470,
            scalar_v3471,
            scalar_v3472,
            scalar_v3480,
            scalar_v3481,
            scalar_v3652,
            scalar_v3653,
            scalar_v3654,
            scalar_v3656,
            scalar_v3658,
            scalar_v3898,
            scalar_v3899,
            scalar_v3900,
            scalar_v3902,
            scalar_v3904,
            scalar_v4144,
            scalar_v4145,
            scalar_v4146,
            scalar_v4148,
            scalar_v4150,
            scalar_v5080,
            scalar_v5081,
            scalar_v5092,
            scalar_v5093,
            scalar_v5094,
            scalar_v5118,
            scalar_v5119,
            scalar_v5120,
            scalar_v5121,
            scalar_v5122,
            scalar_v5123,
            scalar_v5124,
            scalar_v5125,
            scalar_v5126,
            scalar_v5127,
            scalar_v5128,
            scalar_v5129,
            scalar_v5130,
            scalar_v5131,
            scalar_v5132,
            scalar_v5133,
            scalar_v5141,
            scalar_v5142,
            scalar_v5313,
            scalar_v5314,
            scalar_v5315,
            scalar_v5317,
            scalar_v5319,
            scalar_v5559,
            scalar_v5560,
            scalar_v5561,
            scalar_v5563,
            scalar_v5565,
            scalar_v5805,
            scalar_v5806,
            scalar_v5807,
            scalar_v5809,
            scalar_v5811,
            scalar_v6741,
            scalar_v6742,
            scalar_v6753,
            scalar_v6754,
            scalar_v6755,
            scalar_v6779,
            scalar_v6780,
            scalar_v6781,
            scalar_v6782,
            scalar_v6783,
            scalar_v6784,
            scalar_v6785,
            scalar_v6786,
            scalar_v6787,
            scalar_v6788,
            scalar_v6789,
            scalar_v6790,
            scalar_v6791,
            scalar_v6792,
            scalar_v6793,
            scalar_v6794,
            scalar_v6802,
            scalar_v6803,
            scalar_v6974,
            scalar_v6975,
            scalar_v6976,
            scalar_v6978,
            scalar_v6980,
            scalar_v7220,
            scalar_v7221,
            scalar_v7222,
            scalar_v7224,
            scalar_v7226,
            scalar_v7466,
            scalar_v7467,
            scalar_v7468,
            scalar_v7470,
            scalar_v7472,
            scalar_v8402,
            scalar_v8403,
            scalar_v8414,
            scalar_v8415,
            scalar_v8416,
            scalar_v8440,
            scalar_v8441,
            scalar_v8442,
            scalar_v8443,
            scalar_v8444,
            scalar_v8445,
            scalar_v8446,
            scalar_v8447,
            scalar_v8448,
            scalar_v8449,
            scalar_v8450,
            scalar_v8451,
            scalar_v8452,
            scalar_v8453,
            scalar_v8454,
            scalar_v8455,
            scalar_v8463,
            scalar_v8464,
            scalar_v8635,
            scalar_v8636,
            scalar_v8637,
            scalar_v8639,
            scalar_v8641,
            scalar_v8881,
            scalar_v8882,
            scalar_v8883,
            scalar_v8885,
            scalar_v8887,
            scalar_v9127,
            scalar_v9128,
            scalar_v9129,
            scalar_v9131,
            scalar_v9133,
            scalar_v9242,
            scalar_v9309,
            scalar_v9312,
            scalar_v9313,
            scalar_v9321,
            scalar_v9338,
            scalar_v9376,
            scalar_v9556,
            scalar_v9557,
            scalar_v10461,
            scalar_v10471,
            scalar_v10475,
            scalar_v10476,
            scalar_v10481,
            scalar_v10492,
            scalar_v10493,
            scalar_v10494,
            scalar_v10502,
            scalar_v10503,
            scalar_v10504,
            scalar_v10529,
            scalar_v10530,
            scalar_v10531,
            scalar_v10550,
            scalar_v10555,
            scalar_v10648,
            scalar_v10649,
            scalar_v10650,
            scalar_v10651,
            scalar_v10656,
            scalar_v10733,
            scalar_v10737,
            scalar_v10738,
            scalar_v10743,
            scalar_v10753,
            scalar_v10759,
            scalar_v10767,
            scalar_v10769,
            scalar_v10793,
            scalar_v10795,
            scalar_v10814,
            scalar_v10819,
            scalar_v10912,
            scalar_v10914,
            scalar_v10915,
            scalar_v10920,
            scalar_v10997,
            scalar_v11001,
            scalar_v11002,
            scalar_v11007,
            scalar_v11017,
            scalar_v11023,
            scalar_v11031,
            scalar_v11033,
            scalar_v11057,
            scalar_v11059,
            scalar_v11078,
            scalar_v11083,
            scalar_v11176,
            scalar_v11178,
            scalar_v11179,
            scalar_v11184,
            scalar_v11280,
            scalar_v11281,
            scalar_v11282,
            scalar_v11354,
            scalar_v11405,
            scalar_v11406,
            scalar_v11410,
            scalar_v11415,
            scalar_v11419,
            scalar_v11428,
            scalar_v11437,
            scalar_v11438,
            scalar_v11439,
            scalar_v11441,
            scalar_v11448,
            scalar_v11449,
            scalar_v11513,
            scalar_v11514,
            scalar_v11515,
            scalar_v11516,
            scalar_v11523,
            scalar_v11524,
            scalar_v11527,
            scalar_v11528,
            scalar_v11529,
            scalar_v11530,
            scalar_v11531,
            scalar_v11533,
            scalar_v11556,
            scalar_v11560,
            scalar_v11568,
            scalar_v11569,
            scalar_v11570,
            scalar_v11571,
            scalar_v11671,
            scalar_v11672,
            scalar_v11673,
            scalar_v11674,
            scalar_v11675,
            scalar_v11676,
            scalar_v11677,
            scalar_v11678,
            scalar_v11715,
            scalar_v11745,
            scalar_v11773,
            scalar_v11792,
            scalar_v11793,
            scalar_v11794,
            scalar_v11795,
            scalar_v11796,
            scalar_v11797,
            scalar_v11798,
            scalar_v11799,
            scalar_v11878,
            scalar_v13605,
            scalar_v13704,
            scalar_v14185,
            scalar_v14286,
            scalar_v14769,
            scalar_v14870,
            scalar_v15282,
            scalar_v15283,
            scalar_v15286,
            scalar_v15287,
            scalar_v15508,
            scalar_v15541,
            scalar_v15542,
            scalar_v15543,
            scalar_v15548,
            scalar_v15589,
            scalar_v15590,
            scalar_v15656,
            scalar_v15657,
            scalar_v15658,
            scalar_v15663,
            scalar_v15669,
            scalar_v15695,
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
            "level" => { validate_finite_parameter("level", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_finite_parameter("version", value)?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "subversion" => { validate_finite_parameter("subversion", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "revision" => { validate_finite_parameter("revision", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "imax" => { validate_parameter("imax", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "trj" => { validate_parameter("trj", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "frev" => { validate_parameter("frev", value, Some((1000.0, "1000.0")), false, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "cjorbot" => { validate_parameter("cjorbot", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "cjorsti" => { validate_parameter("cjorsti", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "cjorgat" => { validate_parameter("cjorgat", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "vbirbot" => { validate_parameter("vbirbot", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "vbirsti" => { validate_parameter("vbirsti", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "vbirgat" => { validate_parameter("vbirgat", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "pbot" => { validate_parameter("pbot", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "psti" => { validate_parameter("psti", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "pgat" => { validate_parameter("pgat", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "phigbot" => { validate_finite_parameter("phigbot", value)?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "phigsti" => { validate_finite_parameter("phigsti", value)?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "phiggat" => { validate_finite_parameter("phiggat", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "idsatrbot" => { validate_parameter("idsatrbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "idsatrsti" => { validate_parameter("idsatrsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "idsatrgat" => { validate_parameter("idsatrgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "csrhbot" => { validate_parameter("csrhbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "csrhsti" => { validate_parameter("csrhsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "csrhgat" => { validate_parameter("csrhgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "xjunsti" => { validate_parameter("xjunsti", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "xjungat" => { validate_parameter("xjungat", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "ctatbot" => { validate_parameter("ctatbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "ctatsti" => { validate_parameter("ctatsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ctatgat" => { validate_parameter("ctatgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "mefftatbot" => { validate_parameter("mefftatbot", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "mefftatsti" => { validate_parameter("mefftatsti", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "mefftatgat" => { validate_parameter("mefftatgat", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cbbtbot" => { validate_parameter("cbbtbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "cbbtsti" => { validate_parameter("cbbtsti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cbbtgat" => { validate_parameter("cbbtgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "fbbtrbot" => { validate_finite_parameter("fbbtrbot", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "fbbtrsti" => { validate_finite_parameter("fbbtrsti", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "fbbtrgat" => { validate_finite_parameter("fbbtrgat", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "stfbbtbot" => { validate_finite_parameter("stfbbtbot", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "stfbbtsti" => { validate_finite_parameter("stfbbtsti", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "stfbbtgat" => { validate_finite_parameter("stfbbtgat", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "vbrbot" => { validate_parameter("vbrbot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "vbrsti" => { validate_parameter("vbrsti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "vbrgat" => { validate_parameter("vbrgat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "pbrbot" => { validate_parameter("pbrbot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "pbrsti" => { validate_parameter("pbrsti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "pbrgat" => { validate_parameter("pbrgat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "rsbot" => { validate_parameter("rsbot", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "rssti" => { validate_parameter("rssti", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "rsgat" => { validate_parameter("rsgat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "rscom" => { validate_parameter("rscom", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "strs" => { validate_parameter("strs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "tt" => { validate_parameter("tt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "stvbrbot1" => { validate_finite_parameter("stvbrbot1", value)?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "stvbrbot2" => { validate_finite_parameter("stvbrbot2", value)?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "stvbrsti1" => { validate_finite_parameter("stvbrsti1", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "stvbrsti2" => { validate_finite_parameter("stvbrsti2", value)?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "stvbrgat1" => { validate_finite_parameter("stvbrgat1", value)?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "stvbrgat2" => { validate_finite_parameter("stvbrgat2", value)?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "nfabot" => { validate_parameter("nfabot", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "nfasti" => { validate_parameter("nfasti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "nfagat" => { validate_parameter("nfagat", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "abmin" => { validate_parameter("abmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "abmax" => { validate_parameter("abmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "lsmin" => { validate_parameter("lsmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "lsmax" => { validate_parameter("lsmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "lgmin" => { validate_parameter("lgmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "lgmax" => { validate_parameter("lgmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "tempmin" => { validate_parameter("tempmin", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "tempmax" => { validate_parameter("tempmax", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "vfmax" => { validate_parameter("vfmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "vrmax" => { validate_parameter("vrmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "pt" => { validate_parameter("xti", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "expceil" => { validate_parameter("expceil", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "swbv" => { validate_parameter("swbv", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "bv_enable" => { validate_parameter("swbv", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "swjunexp" => { validate_parameter("swjunexp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "vjunref" => { validate_parameter("vjunref", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "fjunq" => { validate_parameter("fjunq", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "corecovery" => { validate_parameter("corecovery", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "njh" => { validate_parameter("njh", value, Some((0.5, "0.5")), false, Some((5.0, "5.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "njdv" => { validate_parameter("njdv", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "ndibot" => { validate_parameter("ndibot", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "ndigat" => { validate_parameter("ndigat", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "ndisti" => { validate_parameter("ndisti", value, Some((1.0, "1.0")), false, Some((1e23, "1e23")), false, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "inj1" => { validate_parameter("inj1", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "inj2" => { validate_parameter("inj2", value, Some((0.0, "0.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "nqs" => { validate_parameter("nqs", value, Some((0.0, "0.0")), false, Some((0.001, "0.001")), false, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "tau" => { validate_parameter("tau", value, Some((1e-12, "1e-12")), false, Some((0.001, "0.001")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "wi" => { validate_parameter("wi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "depnqs" => { validate_parameter("depnqs", value, Some((0.0, "0.0")), false, Some((0.001, "0.001")), false, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "taut" => { validate_parameter("taut", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "injt" => { validate_parameter("injt", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), false, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "ab" => { validate_parameter("ab", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "area" => { validate_parameter("ab", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "ls" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "perim" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "pj" => { validate_parameter("ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "lg" => { validate_parameter("lg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'DIODE_CMC'", name)),
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
        let param_given = self.param_given.as_ref();
        let v1: f64 = p.p6;
        self.scalar_v1 = v1;
        let v3: bool = (p.p6 > -250.0);
        self.scalar_v3 = v3;
        let v4: f64 = (if v3 { p.p6 } else { -250.0 });
        self.scalar_v4 = v4;
        let v5: f64 = if param_given[6] { 1.0 } else { 0.0 };
        self.scalar_v5 = v5;
        let v6: bool = (!(if param_given[6] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v6 = v6;
        let v7: f64 = if param_given[96] { 1.0 } else { 0.0 };
        self.scalar_v7 = v7;
        let v8: bool = (v6 && (if param_given[96] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v8 = v8;
        let v9: f64 = p.p96;
        self.scalar_v9 = v9;
        let v10: bool = (p.p96 > -250.0);
        self.scalar_v10 = v10;
        let v11: f64 = (if v10 { p.p96 } else { -250.0 });
        self.scalar_v11 = v11;
        let v12: f64 = (if v8 { v11 } else { v4 });
        self.scalar_v12 = v12;
        let v13: f64 = p.p5;
        self.scalar_v13 = v13;
        let v15: bool = (p.p5 > 1e-12);
        self.scalar_v15 = v15;
        let v16: f64 = (if v15 { p.p5 } else { 1e-12 });
        self.scalar_v16 = v16;
        let v17: f64 = p.p8;
        self.scalar_v17 = v17;
        let v18: bool = (p.p8 > 1e-12);
        self.scalar_v18 = v18;
        let v19: f64 = (if v18 { p.p8 } else { 1e-12 });
        self.scalar_v19 = v19;
        let v20: f64 = p.p9;
        self.scalar_v20 = v20;
        let v22: bool = (p.p9 > 1e-18);
        self.scalar_v22 = v22;
        let v23: f64 = (if v22 { p.p9 } else { 1e-18 });
        self.scalar_v23 = v23;
        let v24: f64 = p.p10;
        self.scalar_v24 = v24;
        let v25: bool = (p.p10 > 1e-18);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p10 } else { 1e-18 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p11;
        self.scalar_v27 = v27;
        let v29: bool = (p.p11 > 0.05);
        self.scalar_v29 = v29;
        let v30: f64 = (if v29 { p.p11 } else { 0.05 });
        self.scalar_v30 = v30;
        let v31: f64 = p.p12;
        self.scalar_v31 = v31;
        let v32: bool = (p.p12 > 0.05);
        self.scalar_v32 = v32;
        let v33: f64 = (if v32 { p.p12 } else { 0.05 });
        self.scalar_v33 = v33;
        let v34: f64 = p.p13;
        self.scalar_v34 = v34;
        let v35: bool = (p.p13 > 0.05);
        self.scalar_v35 = v35;
        let v36: f64 = (if v35 { p.p13 } else { 0.05 });
        self.scalar_v36 = v36;
        let v37: f64 = p.p14;
        self.scalar_v37 = v37;
        let v38: bool = (p.p14 > 0.05);
        self.scalar_v38 = v38;
        let v40: bool = (p.p14 < 0.95);
        self.scalar_v40 = v40;
        let v41: f64 = (if v40 { p.p14 } else { 0.95 });
        self.scalar_v41 = v41;
        let v42: f64 = (if v38 { v41 } else { 0.05 });
        self.scalar_v42 = v42;
        let v43: f64 = p.p15;
        self.scalar_v43 = v43;
        let v44: bool = (p.p15 > 0.05);
        self.scalar_v44 = v44;
        let v45: bool = (p.p15 < 0.95);
        self.scalar_v45 = v45;
        let v46: f64 = (if v45 { p.p15 } else { 0.95 });
        self.scalar_v46 = v46;
        let v47: f64 = (if v44 { v46 } else { 0.05 });
        self.scalar_v47 = v47;
        let v48: f64 = p.p16;
        self.scalar_v48 = v48;
        let v49: bool = (p.p16 > 0.05);
        self.scalar_v49 = v49;
        let v50: bool = (p.p16 < 0.95);
        self.scalar_v50 = v50;
        let v51: f64 = (if v50 { p.p16 } else { 0.95 });
        self.scalar_v51 = v51;
        let v52: f64 = (if v49 { v51 } else { 0.05 });
        self.scalar_v52 = v52;
        let v53: f64 = p.p17;
        self.scalar_v53 = v53;
        let v54: f64 = p.p18;
        self.scalar_v54 = v54;
        let v55: f64 = p.p19;
        self.scalar_v55 = v55;
        let v56: f64 = p.p20;
        self.scalar_v56 = v56;
        let v58: bool = (p.p20 > 0.0);
        self.scalar_v58 = v58;
        let v59: f64 = (if v58 { p.p20 } else { 0.0 });
        self.scalar_v59 = v59;
        let v60: f64 = p.p21;
        self.scalar_v60 = v60;
        let v61: bool = (p.p21 > 0.0);
        self.scalar_v61 = v61;
        let v62: f64 = (if v61 { p.p21 } else { 0.0 });
        self.scalar_v62 = v62;
        let v63: f64 = p.p22;
        self.scalar_v63 = v63;
        let v64: bool = (p.p22 > 0.0);
        self.scalar_v64 = v64;
        let v65: f64 = (if v64 { p.p22 } else { 0.0 });
        self.scalar_v65 = v65;
        let v66: f64 = p.p23;
        self.scalar_v66 = v66;
        let v67: bool = (p.p23 > 0.0);
        self.scalar_v67 = v67;
        let v68: f64 = (if v67 { p.p23 } else { 0.0 });
        self.scalar_v68 = v68;
        let v69: f64 = p.p24;
        self.scalar_v69 = v69;
        let v70: bool = (p.p24 > 0.0);
        self.scalar_v70 = v70;
        let v71: f64 = (if v70 { p.p24 } else { 0.0 });
        self.scalar_v71 = v71;
        let v72: f64 = p.p25;
        self.scalar_v72 = v72;
        let v73: bool = (p.p25 > 0.0);
        self.scalar_v73 = v73;
        let v74: f64 = (if v73 { p.p25 } else { 0.0 });
        self.scalar_v74 = v74;
        let v75: f64 = p.p26;
        self.scalar_v75 = v75;
        let v77: bool = (p.p26 > 1e-9);
        self.scalar_v77 = v77;
        let v78: f64 = (if v77 { p.p26 } else { 1e-9 });
        self.scalar_v78 = v78;
        let v79: f64 = p.p27;
        self.scalar_v79 = v79;
        let v80: bool = (p.p27 > 1e-9);
        self.scalar_v80 = v80;
        let v81: f64 = (if v80 { p.p27 } else { 1e-9 });
        self.scalar_v81 = v81;
        let v82: f64 = p.p28;
        self.scalar_v82 = v82;
        let v83: bool = (p.p28 > 0.0);
        self.scalar_v83 = v83;
        let v84: f64 = (if v83 { p.p28 } else { 0.0 });
        self.scalar_v84 = v84;
        let v85: f64 = p.p29;
        self.scalar_v85 = v85;
        let v86: bool = (p.p29 > 0.0);
        self.scalar_v86 = v86;
        let v87: f64 = (if v86 { p.p29 } else { 0.0 });
        self.scalar_v87 = v87;
        let v88: f64 = p.p30;
        self.scalar_v88 = v88;
        let v89: bool = (p.p30 > 0.0);
        self.scalar_v89 = v89;
        let v90: f64 = (if v89 { p.p30 } else { 0.0 });
        self.scalar_v90 = v90;
        let v91: f64 = p.p31;
        self.scalar_v91 = v91;
        let v93: bool = (p.p31 > 0.01);
        self.scalar_v93 = v93;
        let v94: f64 = (if v93 { p.p31 } else { 0.01 });
        self.scalar_v94 = v94;
        let v95: f64 = p.p32;
        self.scalar_v95 = v95;
        let v96: bool = (p.p32 > 0.01);
        self.scalar_v96 = v96;
        let v97: f64 = (if v96 { p.p32 } else { 0.01 });
        self.scalar_v97 = v97;
        let v98: f64 = p.p33;
        self.scalar_v98 = v98;
        let v99: bool = (p.p33 > 0.01);
        self.scalar_v99 = v99;
        let v100: f64 = (if v99 { p.p33 } else { 0.01 });
        self.scalar_v100 = v100;
        let v101: f64 = p.p34;
        self.scalar_v101 = v101;
        let v102: bool = (p.p34 > 0.0);
        self.scalar_v102 = v102;
        let v103: f64 = (if v102 { p.p34 } else { 0.0 });
        self.scalar_v103 = v103;
        let v104: f64 = p.p35;
        self.scalar_v104 = v104;
        let v105: bool = (p.p35 > 0.0);
        self.scalar_v105 = v105;
        let v106: f64 = (if v105 { p.p35 } else { 0.0 });
        self.scalar_v106 = v106;
        let v107: f64 = p.p36;
        self.scalar_v107 = v107;
        let v108: bool = (p.p36 > 0.0);
        self.scalar_v108 = v108;
        let v109: f64 = (if v108 { p.p36 } else { 0.0 });
        self.scalar_v109 = v109;
        let v110: f64 = p.p37;
        self.scalar_v110 = v110;
        let v111: f64 = p.p38;
        self.scalar_v111 = v111;
        let v112: f64 = p.p39;
        self.scalar_v112 = v112;
        let v113: f64 = p.p40;
        self.scalar_v113 = v113;
        let v114: f64 = p.p41;
        self.scalar_v114 = v114;
        let v115: f64 = p.p42;
        self.scalar_v115 = v115;
        let v116: f64 = p.p43;
        self.scalar_v116 = v116;
        let v118: bool = (p.p43 > 0.1);
        self.scalar_v118 = v118;
        let v119: f64 = (if v118 { p.p43 } else { 0.1 });
        self.scalar_v119 = v119;
        let v120: f64 = p.p44;
        self.scalar_v120 = v120;
        let v121: bool = (p.p44 > 0.1);
        self.scalar_v121 = v121;
        let v122: f64 = (if v121 { p.p44 } else { 0.1 });
        self.scalar_v122 = v122;
        let v123: f64 = p.p45;
        self.scalar_v123 = v123;
        let v124: bool = (p.p45 > 0.1);
        self.scalar_v124 = v124;
        let v125: f64 = (if v124 { p.p45 } else { 0.1 });
        self.scalar_v125 = v125;
        let v126: f64 = p.p46;
        self.scalar_v126 = v126;
        let v127: bool = (p.p46 > 0.1);
        self.scalar_v127 = v127;
        let v128: f64 = (if v127 { p.p46 } else { 0.1 });
        self.scalar_v128 = v128;
        let v129: f64 = p.p47;
        self.scalar_v129 = v129;
        let v130: bool = (p.p47 > 0.1);
        self.scalar_v130 = v130;
        let v131: f64 = (if v130 { p.p47 } else { 0.1 });
        self.scalar_v131 = v131;
        let v132: f64 = p.p48;
        self.scalar_v132 = v132;
        let v133: bool = (p.p48 > 0.1);
        self.scalar_v133 = v133;
        let v134: f64 = (if v133 { p.p48 } else { 0.1 });
        self.scalar_v134 = v134;
        let v135: f64 = p.p7;
        self.scalar_v135 = v135;
        let v136: f64 = p.p49;
        self.scalar_v136 = v136;
        let v137: bool = (p.p49 > 0.0);
        self.scalar_v137 = v137;
        let v138: f64 = (if v137 { p.p49 } else { 0.0 });
        self.scalar_v138 = v138;
        let v139: f64 = p.p50;
        self.scalar_v139 = v139;
        let v140: bool = (p.p50 > 0.0);
        self.scalar_v140 = v140;
        let v141: f64 = (if v140 { p.p50 } else { 0.0 });
        self.scalar_v141 = v141;
        let v142: f64 = p.p51;
        self.scalar_v142 = v142;
        let v143: bool = (p.p51 > 0.0);
        self.scalar_v143 = v143;
        let v144: f64 = (if v143 { p.p51 } else { 0.0 });
        self.scalar_v144 = v144;
        let v145: f64 = p.p52;
        self.scalar_v145 = v145;
        let v146: bool = (p.p52 > 0.0);
        self.scalar_v146 = v146;
        let v147: f64 = (if v146 { p.p52 } else { 0.0 });
        self.scalar_v147 = v147;
        let v148: f64 = p.p53;
        self.scalar_v148 = v148;
        let v149: bool = (p.p53 > 0.0);
        self.scalar_v149 = v149;
        let v150: f64 = (if v149 { p.p53 } else { 0.0 });
        self.scalar_v150 = v150;
        let v151: f64 = p.p56;
        self.scalar_v151 = v151;
        let v152: bool = (p.p56 > 0.0);
        self.scalar_v152 = v152;
        let v153: f64 = (if v152 { p.p56 } else { 0.0 });
        self.scalar_v153 = v153;
        let v154: f64 = p.p57;
        self.scalar_v154 = v154;
        let v155: f64 = p.p58;
        self.scalar_v155 = v155;
        let v156: f64 = p.p59;
        self.scalar_v156 = v156;
        let v157: f64 = p.p60;
        self.scalar_v157 = v157;
        let v158: f64 = p.p61;
        self.scalar_v158 = v158;
        let v159: f64 = p.p62;
        self.scalar_v159 = v159;
        let v160: f64 = p.p63;
        self.scalar_v160 = v160;
        let v161: bool = (p.p63 > 0.1);
        self.scalar_v161 = v161;
        let v162: f64 = (if v161 { p.p63 } else { 0.1 });
        self.scalar_v162 = v162;
        let v163: f64 = p.p64;
        self.scalar_v163 = v163;
        let v164: bool = (p.p64 > 0.1);
        self.scalar_v164 = v164;
        let v165: f64 = (if v164 { p.p64 } else { 0.1 });
        self.scalar_v165 = v165;
        let v166: f64 = p.p65;
        self.scalar_v166 = v166;
        let v167: bool = (p.p65 > 0.1);
        self.scalar_v167 = v167;
        let v168: f64 = (if v167 { p.p65 } else { 0.1 });
        self.scalar_v168 = v168;
        let v169: f64 = p.p76;
        self.scalar_v169 = v169;
        let v170: bool = (p.p76 > 0.1);
        self.scalar_v170 = v170;
        let v171: f64 = (if v170 { p.p76 } else { 0.1 });
        self.scalar_v171 = v171;
        let v172: f64 = p.p77;
        self.scalar_v172 = v172;
        let v173: bool = (p.p77 > 0.0);
        self.scalar_v173 = v173;
        let v174: f64 = (if v173 { p.p77 } else { 0.0 });
        self.scalar_v174 = v174;
        let v175: f64 = p.p78;
        self.scalar_v175 = v175;
        let v176: bool = (p.p78 > 0.0);
        self.scalar_v176 = v176;
        let v177: f64 = (if v176 { p.p78 } else { 0.0 });
        self.scalar_v177 = v177;
        let v178: f64 = p.p81;
        self.scalar_v178 = v178;
        let v180: bool = (p.p81 > 0.5);
        self.scalar_v180 = v180;
        let v182: f64 = (if v180 { 1.0 } else { 0.0 });
        self.scalar_v182 = v182;
        let v183: bool = (!v180);
        self.scalar_v183 = v183;
        let v184: f64 = (if v183 { 0.0 } else { v182 });
        self.scalar_v184 = v184;
        let v185: f64 = p.p82;
        self.scalar_v185 = v185;
        let v186: bool = (p.p82 > 0.5);
        self.scalar_v186 = v186;
        let v187: f64 = (if v186 { p.p82 } else { 0.5 });
        self.scalar_v187 = v187;
        let v188: f64 = p.p83;
        self.scalar_v188 = v188;
        let v189: bool = (p.p83 > 0.0);
        self.scalar_v189 = v189;
        let v190: f64 = (if v189 { p.p83 } else { 0.0 });
        self.scalar_v190 = v190;
        let v192: f64 = (273.15 + v12);
        self.scalar_v192 = v192;
        let v194: f64 = p.p102;
        self.scalar_v194 = v194;
        let v202: f64 = (8.61726105451295e-5 * v192);
        self.scalar_v202 = v202;
        let v203: f64 = (1.0 / v202);
        self.scalar_v203 = v203;
        let v207: f64 = (0.000702 * v192);
        self.scalar_v207 = v207;
        let v208: f64 = (v207 * v192);
        self.scalar_v208 = v208;
        let v209: f64 = (-v208);
        self.scalar_v209 = v209;
        let v211: f64 = (1108.0 + v192);
        self.scalar_v211 = v211;
        let v212: f64 = (v209 / v211);
        self.scalar_v212 = v212;
        let v213: f64 = (p.p17 + v212);
        self.scalar_v213 = v213;
        let v214: f64 = (p.p18 + v212);
        self.scalar_v214 = v214;
        let v215: f64 = (p.p19 + v212);
        self.scalar_v215 = v215;
        let v225: f64 = (v171 / 2.0);
        self.scalar_v225 = v225;
        let v227: f64 = (v213 * v203);
        self.scalar_v227 = v227;
        let v233: f64 = (v214 * v203);
        self.scalar_v233 = v233;
        let v239: f64 = (v215 * v203);
        self.scalar_v239 = v239;
        let v245: f64 = (v225 / v162);
        self.scalar_v245 = v245;
        let v250: f64 = (v225 / v165);
        self.scalar_v250 = v250;
        let v255: f64 = (v225 / v168);
        self.scalar_v255 = v255;
        let v303: f64 = (1.0 - v42);
        self.scalar_v303 = v303;
        let v304: f64 = (1.0 - v47);
        self.scalar_v304 = v304;
        let v305: f64 = (1.0 - v52);
        self.scalar_v305 = v305;
        let v306: f64 = (1.0 / v303);
        self.scalar_v306 = v306;
        let v307: f64 = (1.0 / v304);
        self.scalar_v307 = v307;
        let v308: f64 = (1.0 / v305);
        self.scalar_v308 = v308;
        let v327: f64 = (1.0447941624768001e-10 / v19);
        self.scalar_v327 = v327;
        let v328: f64 = (v78 * 1.0447941624768001e-10);
        self.scalar_v328 = v328;
        let v329: f64 = (v328 / v23);
        self.scalar_v329 = v329;
        let v330: f64 = (v81 * 1.0447941624768001e-10);
        self.scalar_v330 = v330;
        let v331: f64 = (v330 / v26);
        self.scalar_v331 = v331;
        let v332: f64 = (1.0 / v327);
        self.scalar_v332 = v332;
        let v333: f64 = (1.0 / v329);
        self.scalar_v333 = v333;
        let v334: f64 = (1.0 / v331);
        self.scalar_v334 = v334;
        let v335: f64 = (1.0 / v30);
        self.scalar_v335 = v335;
        let v336: f64 = (1.0 / v33);
        self.scalar_v336 = v336;
        let v337: f64 = (1.0 / v36);
        self.scalar_v337 = v337;
        let v356: f64 = (32.0 * v94);
        self.scalar_v356 = v356;
        let v358: f64 = (v356 * 9.1093826e-31);
        self.scalar_v358 = v358;
        let v359: f64 = (v358 * 1.6021918e-19);
        self.scalar_v359 = v359;
        let v366: f64 = (32.0 * v97);
        self.scalar_v366 = v366;
        let v367: f64 = (v366 * 9.1093826e-31);
        self.scalar_v367 = v367;
        let v368: f64 = (v367 * 1.6021918e-19);
        self.scalar_v368 = v368;
        let v374: f64 = (32.0 * v100);
        self.scalar_v374 = v374;
        let v375: f64 = (v374 * 9.1093826e-31);
        self.scalar_v375 = v375;
        let v376: f64 = (v375 * 1.6021918e-19);
        self.scalar_v376 = v376;
        let v398: f64 = (p.p7 - 1.0);
        self.scalar_v398 = v398;
        let v399: f64 = (v398 / p.p7);
        self.scalar_v399 = v399;
        let v400: f64 = f64::powf(v399, v128);
        self.scalar_v400 = v400;
        let v401: f64 = (1.0 - v400);
        self.scalar_v401 = v401;
        let v402: f64 = (1.0 / v401);
        self.scalar_v402 = v402;
        let v403: f64 = f64::powf(v399, v131);
        self.scalar_v403 = v403;
        let v404: f64 = (1.0 - v403);
        self.scalar_v404 = v404;
        let v405: f64 = (1.0 / v404);
        self.scalar_v405 = v405;
        let v406: f64 = f64::powf(v399, v134);
        self.scalar_v406 = v406;
        let v407: f64 = (1.0 - v406);
        self.scalar_v407 = v407;
        let v408: f64 = (1.0 / v407);
        self.scalar_v408 = v408;
        let v443: f64 = (0.01 * v177);
        self.scalar_v443 = v443;
        let v444: f64 = (1.0 - v443);
        self.scalar_v444 = v444;
        let v445: f64 = (v402 * v402);
        self.scalar_v445 = v445;
        let v446: f64 = (v128 - 1.0);
        self.scalar_v446 = v446;
        let v447: f64 = f64::powf(v399, v446);
        self.scalar_v447 = v447;
        let v448: f64 = (v445 * v447);
        self.scalar_v448 = v448;
        let v449: f64 = (-v448);
        self.scalar_v449 = v449;
        let v450: f64 = (v449 * v128);
        self.scalar_v450 = v450;
        let v452: f64 = (v405 * v405);
        self.scalar_v452 = v452;
        let v453: f64 = (v131 - 1.0);
        self.scalar_v453 = v453;
        let v454: f64 = f64::powf(v399, v453);
        self.scalar_v454 = v454;
        let v455: f64 = (v452 * v454);
        self.scalar_v455 = v455;
        let v456: f64 = (-v455);
        self.scalar_v456 = v456;
        let v457: f64 = (v456 * v131);
        self.scalar_v457 = v457;
        let v459: f64 = (v408 * v408);
        self.scalar_v459 = v459;
        let v460: f64 = (v134 - 1.0);
        self.scalar_v460 = v460;
        let v461: f64 = f64::powf(v399, v460);
        self.scalar_v461 = v461;
        let v462: f64 = (v459 * v461);
        self.scalar_v462 = v462;
        let v463: f64 = (-v462);
        self.scalar_v463 = v463;
        let v464: f64 = (v463 * v134);
        self.scalar_v464 = v464;
        let v471: f64 = p.p87;
        self.scalar_v471 = v471;
        let v473: f64 = (p.p87 * 1000000.0);
        self.scalar_v473 = v473;
        let v474: f64 = p.p89;
        self.scalar_v474 = v474;
        let v475: f64 = (p.p89 * 1000000.0);
        self.scalar_v475 = v475;
        let v476: f64 = p.p88;
        self.scalar_v476 = v476;
        let v477: f64 = (p.p88 * 1000000.0);
        self.scalar_v477 = v477;
        let v495: f64 = p.p97;
        self.scalar_v495 = v495;
        let v497: f64 = p.p93;
        self.scalar_v497 = v497;
        let v505: f64 = p.p94;
        self.scalar_v505 = v505;
        let v509: f64 = p.p99;
        self.scalar_v509 = v509;
        let v510: bool = (p.p99 > 0.0);
        self.scalar_v510 = v510;
        let v511: f64 = (if v510 { p.p99 } else { 0.0 });
        self.scalar_v511 = v511;
        let v512: f64 = (v511 * v174);
        self.scalar_v512 = v512;
        let v513: f64 = (v512 * v174);
        self.scalar_v513 = v513;
        let v514: f64 = (v513 * v444);
        self.scalar_v514 = v514;
        let v515: f64 = (v514 * v444);
        self.scalar_v515 = v515;
        let v516: f64 = p.p100;
        self.scalar_v516 = v516;
        let v517: bool = (p.p100 > 0.0);
        self.scalar_v517 = v517;
        let v518: f64 = (if v517 { p.p100 } else { 0.0 });
        self.scalar_v518 = v518;
        let v519: f64 = (v518 * v174);
        self.scalar_v519 = v519;
        let v520: f64 = (v519 * v444);
        self.scalar_v520 = v520;
        let v521: f64 = p.p101;
        self.scalar_v521 = v521;
        let v522: bool = (p.p101 > 0.0);
        self.scalar_v522 = v522;
        let v523: f64 = (if v522 { p.p101 } else { 0.0 });
        self.scalar_v523 = v523;
        let v524: f64 = (v523 * v174);
        self.scalar_v524 = v524;
        let v525: f64 = (v524 * v444);
        self.scalar_v525 = v525;
        let v596: bool = (v515 == 0.0);
        self.scalar_v596 = v596;
        let v600: bool = (v47 < v52);
        self.scalar_v600 = v600;
        let v601: f64 = (if v600 { v47 } else { v52 });
        self.scalar_v601 = v601;
        let v602: f64 = (0.9 * v601);
        self.scalar_v602 = v602;
        let v603: f64 = (if v596 { v602 } else { v42 });
        self.scalar_v603 = v603;
        let v604: f64 = (v33 + v36);
        self.scalar_v604 = v604;
        let v605: f64 = (if v596 { v604 } else { v30 });
        self.scalar_v605 = v605;
        let v606: bool = (v520 == 0.0);
        self.scalar_v606 = v606;
        let v609: bool = (v42 < v52);
        self.scalar_v609 = v609;
        let v610: f64 = (if v609 { v42 } else { v52 });
        self.scalar_v610 = v610;
        let v611: f64 = (0.9 * v610);
        self.scalar_v611 = v611;
        let v612: f64 = (if v606 { v611 } else { v47 });
        self.scalar_v612 = v612;
        let v613: f64 = (v30 + v36);
        self.scalar_v613 = v613;
        let v614: f64 = (if v606 { v613 } else { v33 });
        self.scalar_v614 = v614;
        let v615: bool = (v525 == 0.0);
        self.scalar_v615 = v615;
        let v618: bool = (v42 < v47);
        self.scalar_v618 = v618;
        let v619: f64 = (if v618 { v42 } else { v47 });
        self.scalar_v619 = v619;
        let v620: f64 = (0.9 * v619);
        self.scalar_v620 = v620;
        let v621: f64 = (if v615 { v620 } else { v52 });
        self.scalar_v621 = v621;
        let v622: f64 = (v30 + v33);
        self.scalar_v622 = v622;
        let v623: f64 = (if v615 { v622 } else { v36 });
        self.scalar_v623 = v623;
        let v629: bool = (v603 > v612);
        self.scalar_v629 = v629;
        let v630: f64 = (if v629 { v603 } else { v612 });
        self.scalar_v630 = v630;
        let v631: bool = (v630 > v621);
        self.scalar_v631 = v631;
        let v632: f64 = (if v631 { v630 } else { v621 });
        self.scalar_v632 = v632;
        let v634: f64 = (-1.0 / v632);
        self.scalar_v634 = v634;
        let v635: f64 = f64::powf(2.0, v634);
        self.scalar_v635 = v635;
        let v636: f64 = (1.0 - v635);
        self.scalar_v636 = v636;
        let v638: bool = (v605 < v614);
        self.scalar_v638 = v638;
        let v639: f64 = (if v638 { v605 } else { v614 });
        self.scalar_v639 = v639;
        let v640: bool = (v639 < v623);
        self.scalar_v640 = v640;
        let v641: f64 = (if v640 { v639 } else { v623 });
        self.scalar_v641 = v641;
        let v642: f64 = (v641 - 0.05);
        self.scalar_v642 = v642;
        let v668: f64 = (1.6021918e-19 * v515);
        self.scalar_v668 = v668;
        let v670: f64 = (1.6021918e-19 * v473);
        self.scalar_v670 = v670;
        let v671: f64 = (2.0895883249536002e-10 / v670);
        self.scalar_v671 = v671;
        let v672: f64 = v671.sqrt();
        self.scalar_v672 = v672;
        let v673: f64 = (p.p94 - v672);
        self.scalar_v673 = v673;
        let v675: f64 = (v673 - 1e-7);
        self.scalar_v675 = v675;
        let v677: f64 = (4.0 * p.p94);
        self.scalar_v677 = v677;
        let v678: f64 = (v677 * 1e-7);
        self.scalar_v678 = v678;
        let v679: bool = (v678 > 0.0);
        self.scalar_v679 = v679;
        let v680: f64 = (-v678);
        self.scalar_v680 = v680;
        let v681: f64 = (if v679 { v678 } else { v680 });
        self.scalar_v681 = v681;
        let v682: f64 = (v675 * v675);
        self.scalar_v682 = v682;
        let v683: f64 = (v682 + v681);
        self.scalar_v683 = v683;
        let v684: f64 = v683.sqrt();
        self.scalar_v684 = v684;
        let v685: f64 = (v675 + v684);
        self.scalar_v685 = v685;
        let v686: f64 = (0.5 * v685);
        self.scalar_v686 = v686;
        let v687: f64 = (p.p94 - v686);
        self.scalar_v687 = v687;
        let v688: bool = (v184 > 0.9);
        self.scalar_v688 = v688;
        let v689: f64 = (v162 - v168);
        self.scalar_v689 = v689;
        let v690: f64 = v689.abs();
        self.scalar_v690 = v690;
        let v692: bool = (v690 > 1e-6);
        self.scalar_v692 = v692;
        let v693: bool = (v515 > 0.0);
        self.scalar_v693 = v693;
        let v694: bool = (v692 && v693);
        self.scalar_v694 = v694;
        let v695: bool = (v525 > 0.0);
        self.scalar_v695 = v695;
        let v696: bool = (v694 && v695);
        self.scalar_v696 = v696;
        let v697: f64 = (v162 - v165);
        self.scalar_v697 = v697;
        let v698: f64 = v697.abs();
        self.scalar_v698 = v698;
        let v699: bool = (v698 > 1e-6);
        self.scalar_v699 = v699;
        let v700: bool = (v699 && v693);
        self.scalar_v700 = v700;
        let v701: bool = (v520 > 0.0);
        self.scalar_v701 = v701;
        let v702: bool = (v700 && v701);
        self.scalar_v702 = v702;
        let v703: bool = (v696 || v702);
        self.scalar_v703 = v703;
        let v704: f64 = (v168 - v165);
        self.scalar_v704 = v704;
        let v705: f64 = v704.abs();
        self.scalar_v705 = v705;
        let v706: bool = (v705 > 1e-6);
        self.scalar_v706 = v706;
        let v707: bool = (v706 && v695);
        self.scalar_v707 = v707;
        let v708: bool = (v707 && v701);
        self.scalar_v708 = v708;
        let v709: bool = (v703 || v708);
        self.scalar_v709 = v709;
        let v710: bool = (v688 && v709);
        self.scalar_v710 = v710;
        let v711: f64 = (if v710 { 0.0 } else { v184 });
        self.scalar_v711 = v711;
        let v712: bool = (!v709);
        self.scalar_v712 = v712;
        let v713: bool = (v688 && v712);
        self.scalar_v713 = v713;
        let v714: bool = (v713 && v693);
        self.scalar_v714 = v714;
        let v715: f64 = (if v714 { v162 } else { 1.0 });
        self.scalar_v715 = v715;
        let v716: bool = (v713 && v695);
        self.scalar_v716 = v716;
        let v717: f64 = (if v716 { v168 } else { v715 });
        self.scalar_v717 = v717;
        let v718: bool = (v713 && v701);
        self.scalar_v718 = v718;
        let v719: f64 = (if v718 { v165 } else { v717 });
        self.scalar_v719 = v719;
        let v720: bool = (v711 == 1.0);
        self.scalar_v720 = v720;
        let v721: f64 = (if v720 { 0.0 } else { 0.0 });
        self.scalar_v721 = v721;
        let v723: f64 = (if v720 { 0.4 } else { 0.0 });
        self.scalar_v723 = v723;
        let v725: f64 = (if v720 { 0.65 } else { 0.0 });
        self.scalar_v725 = v725;
        let v727: f64 = (if v720 { 0.8 } else { 0.0 });
        self.scalar_v727 = v727;
        let v728: f64 = (-v723);
        self.scalar_v728 = v728;
        let v729: f64 = (v728 * v187);
        self.scalar_v729 = v729;
        let v730: f64 = (if v720 { v729 } else { 0.0 });
        self.scalar_v730 = v730;
        let v731: f64 = (-v725);
        self.scalar_v731 = v731;
        let v732: f64 = (v731 * v187);
        self.scalar_v732 = v732;
        let v733: f64 = (if v720 { v732 } else { 0.0 });
        self.scalar_v733 = v733;
        let v734: f64 = (-v727);
        self.scalar_v734 = v734;
        let v735: f64 = (v734 * v187);
        self.scalar_v735 = v735;
        let v736: f64 = (if v720 { v735 } else { 0.0 });
        self.scalar_v736 = v736;
        let v737: f64 = (if v720 { 0.1 } else { 0.0 });
        self.scalar_v737 = v737;
        let v739: f64 = (if v720 { 0.2 } else { 0.0 });
        self.scalar_v739 = v739;
        let v740: bool = (v596 && v606);
        self.scalar_v740 = v740;
        let v741: bool = (v740 && v615);
        self.scalar_v741 = v741;
        let v742: bool = (!v741);
        self.scalar_v742 = v742;
        let v743: bool = (v720 && v742);
        self.scalar_v743 = v743;
        let v805: f64 = p.p85;
        self.scalar_v805 = v805;
        let v806: bool = (v162 < p.p85);
        self.scalar_v806 = v806;
        let v808: f64 = p.p86;
        self.scalar_v808 = v808;
        let v819: f64 = (4.0 * p.p85);
        self.scalar_v819 = v819;
        let v820: f64 = (v819 * 0.01);
        self.scalar_v820 = v820;
        let v837: f64 = (4.0 * v162);
        self.scalar_v837 = v837;
        let v838: f64 = (v837 * 0.01);
        self.scalar_v838 = v838;
        let v884: bool = (!v806);
        self.scalar_v884 = v884;
        let v933: bool = (v165 < p.p85);
        self.scalar_v933 = v933;
        let v961: f64 = (4.0 * v165);
        self.scalar_v961 = v961;
        let v962: f64 = (v961 * 0.01);
        self.scalar_v962 = v962;
        let v1008: bool = (!v933);
        self.scalar_v1008 = v1008;
        let v1057: bool = (v168 < p.p85);
        self.scalar_v1057 = v1057;
        let v1085: f64 = (4.0 * v168);
        self.scalar_v1085 = v1085;
        let v1086: f64 = (v1085 * 0.01);
        self.scalar_v1086 = v1086;
        let v1132: bool = (!v1057);
        self.scalar_v1132 = v1132;
        let v1619: bool = (v730 > 0.0);
        self.scalar_v1619 = v1619;
        let v1620: bool = (v743 && v1619);
        self.scalar_v1620 = v1620;
        let v1631: bool = (!v1619);
        self.scalar_v1631 = v1631;
        let v1632: bool = (v743 && v1631);
        self.scalar_v1632 = v1632;
        let v1633: f64 = (-v730);
        self.scalar_v1633 = v1633;
        let v1659: f64 = (v730 + v642);
        self.scalar_v1659 = v1659;
        let v1660: f64 = (v730 - v642);
        self.scalar_v1660 = v1660;
        let v1661: f64 = (v1660 * v1660);
        self.scalar_v1661 = v1661;
        let v1662: f64 = (4.0 * v202);
        self.scalar_v1662 = v1662;
        let v1663: f64 = (v1662 * v202);
        self.scalar_v1663 = v1663;
        let v1664: f64 = (v1661 + v1663);
        self.scalar_v1664 = v1664;
        let v1665: f64 = v1664.sqrt();
        self.scalar_v1665 = v1665;
        let v1666: f64 = (v1659 - v1665);
        self.scalar_v1666 = v1666;
        let v1667: f64 = (0.5 * v1666);
        self.scalar_v1667 = v1667;
        let v1668: f64 = (if v743 { v1667 } else { v721 });
        self.scalar_v1668 = v1668;
        let v1669: f64 = (v730 + 0.0);
        self.scalar_v1669 = v1669;
        let v1670: f64 = (v730 - 0.0);
        self.scalar_v1670 = v1670;
        let v1671: f64 = (v1670 * v1670);
        self.scalar_v1671 = v1671;
        let v1674: f64 = (v1671 + 4e-12);
        self.scalar_v1674 = v1674;
        let v1675: f64 = v1674.sqrt();
        self.scalar_v1675 = v1675;
        let v1676: f64 = (v1669 - v1675);
        self.scalar_v1676 = v1676;
        let v1677: f64 = (0.5 * v1676);
        self.scalar_v1677 = v1677;
        let v1678: f64 = (if v743 { v1677 } else { v721 });
        self.scalar_v1678 = v1678;
        let v1679: bool = (!v742);
        self.scalar_v1679 = v1679;
        let v1680: bool = (v720 && v1679);
        self.scalar_v1680 = v1680;
        let v1688: f64 = (if v1680 { 0.0 } else { v1668 });
        self.scalar_v1688 = v1688;
        let v1689: f64 = (if v1680 { 0.0 } else { v1678 });
        self.scalar_v1689 = v1689;
        let v1690: bool = (v720 && v596);
        self.scalar_v1690 = v1690;
        let v1691: f64 = (if v1690 { 0.0 } else { 0.0 });
        self.scalar_v1691 = v1691;
        let v1692: bool = (v303 == 0.5);
        self.scalar_v1692 = v1692;
        let v1693: bool = (!v596);
        self.scalar_v1693 = v1693;
        let v1694: bool = (v720 && v1693);
        self.scalar_v1694 = v1694;
        let v1695: bool = (v1694 && v1692);
        self.scalar_v1695 = v1695;
        let v1700: bool = (!v1692);
        self.scalar_v1700 = v1700;
        let v1701: bool = (v1694 && v1700);
        self.scalar_v1701 = v1701;
        let v1712: bool = (v68 == 0.0);
        self.scalar_v1712 = v1712;
        let v1713: bool = (v84 == 0.0);
        self.scalar_v1713 = v1713;
        let v1714: bool = (v1712 && v1713);
        self.scalar_v1714 = v1714;
        let v1715: bool = (v1694 && v1714);
        self.scalar_v1715 = v1715;
        let v1716: f64 = (if v1715 { 0.0 } else { v721 });
        self.scalar_v1716 = v1716;
        let v1717: bool = (!v1714);
        self.scalar_v1717 = v1717;
        let v1718: bool = (v1694 && v1717);
        self.scalar_v1718 = v1718;
        let v1726: bool = (v42 == 0.5);
        self.scalar_v1726 = v1726;
        let v1727: bool = (v1718 && v1726);
        self.scalar_v1727 = v1727;
        let v1728: f64 = (if v1727 { 0.0 } else { v721 });
        self.scalar_v1728 = v1728;
        let v1729: bool = (!v1726);
        self.scalar_v1729 = v1729;
        let v1730: bool = (v1718 && v1729);
        self.scalar_v1730 = v1730;
        let v1737: f64 = (2.0 * v42);
        self.scalar_v1737 = v1737;
        let v1738: f64 = (1.0 - v1737);
        self.scalar_v1738 = v1738;
        let v1757: bool = (v1694 && v1713);
        self.scalar_v1757 = v1757;
        let v1758: f64 = (if v1757 { 0.0 } else { v721 });
        self.scalar_v1758 = v1758;
        let v1759: bool = (!v1713);
        self.scalar_v1759 = v1759;
        let v1760: bool = (v1694 && v1759);
        self.scalar_v1760 = v1760;
        let v1781: f64 = (-v42);
        self.scalar_v1781 = v1781;
        let v1782: f64 = (v1781 * v306);
        self.scalar_v1782 = v1782;
        let v1783: bool = (v1782 == -1.0);
        self.scalar_v1783 = v1783;
        let v1784: bool = (v1760 && v1783);
        self.scalar_v1784 = v1784;
        let v1789: bool = (!v1783);
        self.scalar_v1789 = v1789;
        let v1790: bool = (v1760 && v1789);
        self.scalar_v1790 = v1790;
        let v1885: bool = (v103 == 0.0);
        self.scalar_v1885 = v1885;
        let v1886: bool = (v1694 && v1885);
        self.scalar_v1886 = v1886;
        let v1887: f64 = (if v1886 { 0.0 } else { v721 });
        self.scalar_v1887 = v1887;
        let v1888: bool = (!v1885);
        self.scalar_v1888 = v1888;
        let v1889: bool = (v1694 && v1888);
        self.scalar_v1889 = v1889;
        let v1890: bool = (v1889 && v1726);
        self.scalar_v1890 = v1890;
        let v1891: f64 = (v30 - v1688);
        self.scalar_v1891 = v1891;
        let v1892: f64 = (v1891 * v335);
        self.scalar_v1892 = v1892;
        let v1893: f64 = v1892.sqrt();
        self.scalar_v1893 = v1893;
        let v1895: bool = (v1889 && v1729);
        self.scalar_v1895 = v1895;
        let v1896: f64 = f64::powf(v1892, v42);
        self.scalar_v1896 = v1896;
        let v1898: f64 = (v1891 * v332);
        self.scalar_v1898 = v1898;
        let v1941: f64 = p.p80;
        self.scalar_v1941 = v1941;
        let v1942: bool = (p.p80 == 0.0);
        self.scalar_v1942 = v1942;
        let v1946: f64 = (-v399);
        self.scalar_v1946 = v1946;
        let v1949: bool = (v128 == 4.0);
        self.scalar_v1949 = v1949;
        let v1960: bool = (!v1949);
        self.scalar_v1960 = v1960;
        let v1983: bool = (v720 && v606);
        self.scalar_v1983 = v1983;
        let v1984: f64 = (if v1983 { 0.0 } else { 0.0 });
        self.scalar_v1984 = v1984;
        let v1985: bool = (v304 == 0.5);
        self.scalar_v1985 = v1985;
        let v1986: bool = (!v606);
        self.scalar_v1986 = v1986;
        let v1987: bool = (v720 && v1986);
        self.scalar_v1987 = v1987;
        let v1988: bool = (v1987 && v1985);
        self.scalar_v1988 = v1988;
        let v1993: bool = (!v1985);
        self.scalar_v1993 = v1993;
        let v1994: bool = (v1987 && v1993);
        self.scalar_v1994 = v1994;
        let v2004: bool = (v71 == 0.0);
        self.scalar_v2004 = v2004;
        let v2005: bool = (v87 == 0.0);
        self.scalar_v2005 = v2005;
        let v2006: bool = (v2004 && v2005);
        self.scalar_v2006 = v2006;
        let v2007: bool = (v1987 && v2006);
        self.scalar_v2007 = v2007;
        let v2013: bool = (!v2006);
        self.scalar_v2013 = v2013;
        let v2014: bool = (v1987 && v2013);
        self.scalar_v2014 = v2014;
        let v2022: bool = (v47 == 0.5);
        self.scalar_v2022 = v2022;
        let v2023: bool = (v2014 && v2022);
        self.scalar_v2023 = v2023;
        let v2025: bool = (!v2022);
        self.scalar_v2025 = v2025;
        let v2026: bool = (v2014 && v2025);
        self.scalar_v2026 = v2026;
        let v2033: f64 = (2.0 * v47);
        self.scalar_v2033 = v2033;
        let v2034: f64 = (1.0 - v2033);
        self.scalar_v2034 = v2034;
        let v2052: bool = (v1987 && v2005);
        self.scalar_v2052 = v2052;
        let v2054: bool = (!v2005);
        self.scalar_v2054 = v2054;
        let v2055: bool = (v1987 && v2054);
        self.scalar_v2055 = v2055;
        let v2075: f64 = (-v47);
        self.scalar_v2075 = v2075;
        let v2076: f64 = (v2075 * v307);
        self.scalar_v2076 = v2076;
        let v2077: bool = (v2076 == -1.0);
        self.scalar_v2077 = v2077;
        let v2078: bool = (v2055 && v2077);
        self.scalar_v2078 = v2078;
        let v2083: bool = (!v2077);
        self.scalar_v2083 = v2083;
        let v2084: bool = (v2055 && v2083);
        self.scalar_v2084 = v2084;
        let v2177: bool = (v106 == 0.0);
        self.scalar_v2177 = v2177;
        let v2178: bool = (v1987 && v2177);
        self.scalar_v2178 = v2178;
        let v2180: bool = (!v2177);
        self.scalar_v2180 = v2180;
        let v2181: bool = (v1987 && v2180);
        self.scalar_v2181 = v2181;
        let v2182: bool = (v2181 && v2022);
        self.scalar_v2182 = v2182;
        let v2183: f64 = (v33 - v1688);
        self.scalar_v2183 = v2183;
        let v2184: f64 = (v2183 * v336);
        self.scalar_v2184 = v2184;
        let v2185: f64 = v2184.sqrt();
        self.scalar_v2185 = v2185;
        let v2187: bool = (v2181 && v2025);
        self.scalar_v2187 = v2187;
        let v2188: f64 = f64::powf(v2184, v47);
        self.scalar_v2188 = v2188;
        let v2190: f64 = (v2183 * v333);
        self.scalar_v2190 = v2190;
        let v2238: bool = (v131 == 4.0);
        self.scalar_v2238 = v2238;
        let v2249: bool = (!v2238);
        self.scalar_v2249 = v2249;
        let v2272: bool = (v720 && v615);
        self.scalar_v2272 = v2272;
        let v2273: f64 = (if v2272 { 0.0 } else { 0.0 });
        self.scalar_v2273 = v2273;
        let v2274: bool = (v305 == 0.5);
        self.scalar_v2274 = v2274;
        let v2275: bool = (!v615);
        self.scalar_v2275 = v2275;
        let v2276: bool = (v720 && v2275);
        self.scalar_v2276 = v2276;
        let v2277: bool = (v2276 && v2274);
        self.scalar_v2277 = v2277;
        let v2282: bool = (!v2274);
        self.scalar_v2282 = v2282;
        let v2283: bool = (v2276 && v2282);
        self.scalar_v2283 = v2283;
        let v2293: bool = (v74 == 0.0);
        self.scalar_v2293 = v2293;
        let v2294: bool = (v90 == 0.0);
        self.scalar_v2294 = v2294;
        let v2295: bool = (v2293 && v2294);
        self.scalar_v2295 = v2295;
        let v2296: bool = (v2276 && v2295);
        self.scalar_v2296 = v2296;
        let v2302: bool = (!v2295);
        self.scalar_v2302 = v2302;
        let v2303: bool = (v2276 && v2302);
        self.scalar_v2303 = v2303;
        let v2311: bool = (v52 == 0.5);
        self.scalar_v2311 = v2311;
        let v2312: bool = (v2303 && v2311);
        self.scalar_v2312 = v2312;
        let v2314: bool = (!v2311);
        self.scalar_v2314 = v2314;
        let v2315: bool = (v2303 && v2314);
        self.scalar_v2315 = v2315;
        let v2322: f64 = (2.0 * v52);
        self.scalar_v2322 = v2322;
        let v2323: f64 = (1.0 - v2322);
        self.scalar_v2323 = v2323;
        let v2341: bool = (v2276 && v2294);
        self.scalar_v2341 = v2341;
        let v2343: bool = (!v2294);
        self.scalar_v2343 = v2343;
        let v2344: bool = (v2276 && v2343);
        self.scalar_v2344 = v2344;
        let v2364: f64 = (-v52);
        self.scalar_v2364 = v2364;
        let v2365: f64 = (v2364 * v308);
        self.scalar_v2365 = v2365;
        let v2366: bool = (v2365 == -1.0);
        self.scalar_v2366 = v2366;
        let v2367: bool = (v2344 && v2366);
        self.scalar_v2367 = v2367;
        let v2372: bool = (!v2366);
        self.scalar_v2372 = v2372;
        let v2373: bool = (v2344 && v2372);
        self.scalar_v2373 = v2373;
        let v2466: bool = (v109 == 0.0);
        self.scalar_v2466 = v2466;
        let v2467: bool = (v2276 && v2466);
        self.scalar_v2467 = v2467;
        let v2469: bool = (!v2466);
        self.scalar_v2469 = v2469;
        let v2470: bool = (v2276 && v2469);
        self.scalar_v2470 = v2470;
        let v2471: bool = (v2470 && v2311);
        self.scalar_v2471 = v2471;
        let v2472: f64 = (v36 - v1688);
        self.scalar_v2472 = v2472;
        let v2473: f64 = (v2472 * v337);
        self.scalar_v2473 = v2473;
        let v2474: f64 = v2473.sqrt();
        self.scalar_v2474 = v2474;
        let v2476: bool = (v2470 && v2314);
        self.scalar_v2476 = v2476;
        let v2477: f64 = f64::powf(v2473, v52);
        self.scalar_v2477 = v2477;
        let v2479: f64 = (v2472 * v334);
        self.scalar_v2479 = v2479;
        let v2527: bool = (v134 == 4.0);
        self.scalar_v2527 = v2527;
        let v2538: bool = (!v2527);
        self.scalar_v2538 = v2538;
        let v3419: bool = (v733 > 0.0);
        self.scalar_v3419 = v3419;
        let v3420: bool = (v743 && v3419);
        self.scalar_v3420 = v3420;
        let v3431: bool = (!v3419);
        self.scalar_v3431 = v3431;
        let v3432: bool = (v743 && v3431);
        self.scalar_v3432 = v3432;
        let v3433: f64 = (-v733);
        self.scalar_v3433 = v3433;
        let v3457: f64 = (v733 + v642);
        self.scalar_v3457 = v3457;
        let v3458: f64 = (v733 - v642);
        self.scalar_v3458 = v3458;
        let v3459: f64 = (v3458 * v3458);
        self.scalar_v3459 = v3459;
        let v3460: f64 = (v3459 + v1663);
        self.scalar_v3460 = v3460;
        let v3461: f64 = v3460.sqrt();
        self.scalar_v3461 = v3461;
        let v3462: f64 = (v3457 - v3461);
        self.scalar_v3462 = v3462;
        let v3463: f64 = (0.5 * v3462);
        self.scalar_v3463 = v3463;
        let v3464: f64 = (if v743 { v3463 } else { v1688 });
        self.scalar_v3464 = v3464;
        let v3465: f64 = (v733 + 0.0);
        self.scalar_v3465 = v3465;
        let v3466: f64 = (v733 - 0.0);
        self.scalar_v3466 = v3466;
        let v3467: f64 = (v3466 * v3466);
        self.scalar_v3467 = v3467;
        let v3468: f64 = (v3467 + 4e-12);
        self.scalar_v3468 = v3468;
        let v3469: f64 = v3468.sqrt();
        self.scalar_v3469 = v3469;
        let v3470: f64 = (v3465 - v3469);
        self.scalar_v3470 = v3470;
        let v3471: f64 = (0.5 * v3470);
        self.scalar_v3471 = v3471;
        let v3472: f64 = (if v743 { v3471 } else { v1689 });
        self.scalar_v3472 = v3472;
        let v3480: f64 = (if v1680 { 0.0 } else { v3464 });
        self.scalar_v3480 = v3480;
        let v3481: f64 = (if v1680 { 0.0 } else { v3472 });
        self.scalar_v3481 = v3481;
        let v3652: f64 = (v30 - v3480);
        self.scalar_v3652 = v3652;
        let v3653: f64 = (v3652 * v335);
        self.scalar_v3653 = v3653;
        let v3654: f64 = v3653.sqrt();
        self.scalar_v3654 = v3654;
        let v3656: f64 = f64::powf(v3653, v42);
        self.scalar_v3656 = v3656;
        let v3658: f64 = (v3652 * v332);
        self.scalar_v3658 = v3658;
        let v3898: f64 = (v33 - v3480);
        self.scalar_v3898 = v3898;
        let v3899: f64 = (v3898 * v336);
        self.scalar_v3899 = v3899;
        let v3900: f64 = v3899.sqrt();
        self.scalar_v3900 = v3900;
        let v3902: f64 = f64::powf(v3899, v47);
        self.scalar_v3902 = v3902;
        let v3904: f64 = (v3898 * v333);
        self.scalar_v3904 = v3904;
        let v4144: f64 = (v36 - v3480);
        self.scalar_v4144 = v4144;
        let v4145: f64 = (v4144 * v337);
        self.scalar_v4145 = v4145;
        let v4146: f64 = v4145.sqrt();
        self.scalar_v4146 = v4146;
        let v4148: f64 = f64::powf(v4145, v52);
        self.scalar_v4148 = v4148;
        let v4150: f64 = (v4144 * v334);
        self.scalar_v4150 = v4150;
        let v5080: bool = (v736 > 0.0);
        self.scalar_v5080 = v5080;
        let v5081: bool = (v743 && v5080);
        self.scalar_v5081 = v5081;
        let v5092: bool = (!v5080);
        self.scalar_v5092 = v5092;
        let v5093: bool = (v743 && v5092);
        self.scalar_v5093 = v5093;
        let v5094: f64 = (-v736);
        self.scalar_v5094 = v5094;
        let v5118: f64 = (v736 + v642);
        self.scalar_v5118 = v5118;
        let v5119: f64 = (v736 - v642);
        self.scalar_v5119 = v5119;
        let v5120: f64 = (v5119 * v5119);
        self.scalar_v5120 = v5120;
        let v5121: f64 = (v5120 + v1663);
        self.scalar_v5121 = v5121;
        let v5122: f64 = v5121.sqrt();
        self.scalar_v5122 = v5122;
        let v5123: f64 = (v5118 - v5122);
        self.scalar_v5123 = v5123;
        let v5124: f64 = (0.5 * v5123);
        self.scalar_v5124 = v5124;
        let v5125: f64 = (if v743 { v5124 } else { v3480 });
        self.scalar_v5125 = v5125;
        let v5126: f64 = (v736 + 0.0);
        self.scalar_v5126 = v5126;
        let v5127: f64 = (v736 - 0.0);
        self.scalar_v5127 = v5127;
        let v5128: f64 = (v5127 * v5127);
        self.scalar_v5128 = v5128;
        let v5129: f64 = (v5128 + 4e-12);
        self.scalar_v5129 = v5129;
        let v5130: f64 = v5129.sqrt();
        self.scalar_v5130 = v5130;
        let v5131: f64 = (v5126 - v5130);
        self.scalar_v5131 = v5131;
        let v5132: f64 = (0.5 * v5131);
        self.scalar_v5132 = v5132;
        let v5133: f64 = (if v743 { v5132 } else { v3481 });
        self.scalar_v5133 = v5133;
        let v5141: f64 = (if v1680 { 0.0 } else { v5125 });
        self.scalar_v5141 = v5141;
        let v5142: f64 = (if v1680 { 0.0 } else { v5133 });
        self.scalar_v5142 = v5142;
        let v5313: f64 = (v30 - v5141);
        self.scalar_v5313 = v5313;
        let v5314: f64 = (v5313 * v335);
        self.scalar_v5314 = v5314;
        let v5315: f64 = v5314.sqrt();
        self.scalar_v5315 = v5315;
        let v5317: f64 = f64::powf(v5314, v42);
        self.scalar_v5317 = v5317;
        let v5319: f64 = (v5313 * v332);
        self.scalar_v5319 = v5319;
        let v5559: f64 = (v33 - v5141);
        self.scalar_v5559 = v5559;
        let v5560: f64 = (v5559 * v336);
        self.scalar_v5560 = v5560;
        let v5561: f64 = v5560.sqrt();
        self.scalar_v5561 = v5561;
        let v5563: f64 = f64::powf(v5560, v47);
        self.scalar_v5563 = v5563;
        let v5565: f64 = (v5559 * v333);
        self.scalar_v5565 = v5565;
        let v5805: f64 = (v36 - v5141);
        self.scalar_v5805 = v5805;
        let v5806: f64 = (v5805 * v337);
        self.scalar_v5806 = v5806;
        let v5807: f64 = v5806.sqrt();
        self.scalar_v5807 = v5807;
        let v5809: f64 = f64::powf(v5806, v52);
        self.scalar_v5809 = v5809;
        let v5811: f64 = (v5805 * v334);
        self.scalar_v5811 = v5811;
        let v6741: bool = (v737 > 0.0);
        self.scalar_v6741 = v6741;
        let v6742: bool = (v743 && v6741);
        self.scalar_v6742 = v6742;
        let v6753: bool = (!v6741);
        self.scalar_v6753 = v6753;
        let v6754: bool = (v743 && v6753);
        self.scalar_v6754 = v6754;
        let v6755: f64 = (-v737);
        self.scalar_v6755 = v6755;
        let v6779: f64 = (v737 + v642);
        self.scalar_v6779 = v6779;
        let v6780: f64 = (v737 - v642);
        self.scalar_v6780 = v6780;
        let v6781: f64 = (v6780 * v6780);
        self.scalar_v6781 = v6781;
        let v6782: f64 = (v6781 + v1663);
        self.scalar_v6782 = v6782;
        let v6783: f64 = v6782.sqrt();
        self.scalar_v6783 = v6783;
        let v6784: f64 = (v6779 - v6783);
        self.scalar_v6784 = v6784;
        let v6785: f64 = (0.5 * v6784);
        self.scalar_v6785 = v6785;
        let v6786: f64 = (if v743 { v6785 } else { v5141 });
        self.scalar_v6786 = v6786;
        let v6787: f64 = (v737 + 0.0);
        self.scalar_v6787 = v6787;
        let v6788: f64 = (v737 - 0.0);
        self.scalar_v6788 = v6788;
        let v6789: f64 = (v6788 * v6788);
        self.scalar_v6789 = v6789;
        let v6790: f64 = (v6789 + 4e-12);
        self.scalar_v6790 = v6790;
        let v6791: f64 = v6790.sqrt();
        self.scalar_v6791 = v6791;
        let v6792: f64 = (v6787 - v6791);
        self.scalar_v6792 = v6792;
        let v6793: f64 = (0.5 * v6792);
        self.scalar_v6793 = v6793;
        let v6794: f64 = (if v743 { v6793 } else { v5142 });
        self.scalar_v6794 = v6794;
        let v6802: f64 = (if v1680 { 0.0 } else { v6786 });
        self.scalar_v6802 = v6802;
        let v6803: f64 = (if v1680 { 0.0 } else { v6794 });
        self.scalar_v6803 = v6803;
        let v6974: f64 = (v30 - v6802);
        self.scalar_v6974 = v6974;
        let v6975: f64 = (v6974 * v335);
        self.scalar_v6975 = v6975;
        let v6976: f64 = v6975.sqrt();
        self.scalar_v6976 = v6976;
        let v6978: f64 = f64::powf(v6975, v42);
        self.scalar_v6978 = v6978;
        let v6980: f64 = (v6974 * v332);
        self.scalar_v6980 = v6980;
        let v7220: f64 = (v33 - v6802);
        self.scalar_v7220 = v7220;
        let v7221: f64 = (v7220 * v336);
        self.scalar_v7221 = v7221;
        let v7222: f64 = v7221.sqrt();
        self.scalar_v7222 = v7222;
        let v7224: f64 = f64::powf(v7221, v47);
        self.scalar_v7224 = v7224;
        let v7226: f64 = (v7220 * v333);
        self.scalar_v7226 = v7226;
        let v7466: f64 = (v36 - v6802);
        self.scalar_v7466 = v7466;
        let v7467: f64 = (v7466 * v337);
        self.scalar_v7467 = v7467;
        let v7468: f64 = v7467.sqrt();
        self.scalar_v7468 = v7468;
        let v7470: f64 = f64::powf(v7467, v52);
        self.scalar_v7470 = v7470;
        let v7472: f64 = (v7466 * v334);
        self.scalar_v7472 = v7472;
        let v8402: bool = (v739 > 0.0);
        self.scalar_v8402 = v8402;
        let v8403: bool = (v743 && v8402);
        self.scalar_v8403 = v8403;
        let v8414: bool = (!v8402);
        self.scalar_v8414 = v8414;
        let v8415: bool = (v743 && v8414);
        self.scalar_v8415 = v8415;
        let v8416: f64 = (-v739);
        self.scalar_v8416 = v8416;
        let v8440: f64 = (v739 + v642);
        self.scalar_v8440 = v8440;
        let v8441: f64 = (v739 - v642);
        self.scalar_v8441 = v8441;
        let v8442: f64 = (v8441 * v8441);
        self.scalar_v8442 = v8442;
        let v8443: f64 = (v8442 + v1663);
        self.scalar_v8443 = v8443;
        let v8444: f64 = v8443.sqrt();
        self.scalar_v8444 = v8444;
        let v8445: f64 = (v8440 - v8444);
        self.scalar_v8445 = v8445;
        let v8446: f64 = (0.5 * v8445);
        self.scalar_v8446 = v8446;
        let v8447: f64 = (if v743 { v8446 } else { v6802 });
        self.scalar_v8447 = v8447;
        let v8448: f64 = (v739 + 0.0);
        self.scalar_v8448 = v8448;
        let v8449: f64 = (v739 - 0.0);
        self.scalar_v8449 = v8449;
        let v8450: f64 = (v8449 * v8449);
        self.scalar_v8450 = v8450;
        let v8451: f64 = (v8450 + 4e-12);
        self.scalar_v8451 = v8451;
        let v8452: f64 = v8451.sqrt();
        self.scalar_v8452 = v8452;
        let v8453: f64 = (v8448 - v8452);
        self.scalar_v8453 = v8453;
        let v8454: f64 = (0.5 * v8453);
        self.scalar_v8454 = v8454;
        let v8455: f64 = (if v743 { v8454 } else { v6803 });
        self.scalar_v8455 = v8455;
        let v8463: f64 = (if v1680 { 0.0 } else { v8447 });
        self.scalar_v8463 = v8463;
        let v8464: f64 = (if v1680 { 0.0 } else { v8455 });
        self.scalar_v8464 = v8464;
        let v8635: f64 = (v30 - v8463);
        self.scalar_v8635 = v8635;
        let v8636: f64 = (v8635 * v335);
        self.scalar_v8636 = v8636;
        let v8637: f64 = v8636.sqrt();
        self.scalar_v8637 = v8637;
        let v8639: f64 = f64::powf(v8636, v42);
        self.scalar_v8639 = v8639;
        let v8641: f64 = (v8635 * v332);
        self.scalar_v8641 = v8641;
        let v8881: f64 = (v33 - v8463);
        self.scalar_v8881 = v8881;
        let v8882: f64 = (v8881 * v336);
        self.scalar_v8882 = v8882;
        let v8883: f64 = v8882.sqrt();
        self.scalar_v8883 = v8883;
        let v8885: f64 = f64::powf(v8882, v47);
        self.scalar_v8885 = v8885;
        let v8887: f64 = (v8881 * v333);
        self.scalar_v8887 = v8887;
        let v9127: f64 = (v36 - v8463);
        self.scalar_v9127 = v9127;
        let v9128: f64 = (v9127 * v337);
        self.scalar_v9128 = v9128;
        let v9129: f64 = v9128.sqrt();
        self.scalar_v9129 = v9129;
        let v9131: f64 = f64::powf(v9128, v52);
        self.scalar_v9131 = v9131;
        let v9133: f64 = (v9127 * v334);
        self.scalar_v9133 = v9133;
        let v9242: f64 = (v737 - v739);
        self.scalar_v9242 = v9242;
        let v9309: f64 = (v730 - v733);
        self.scalar_v9309 = v9309;
        let v9312: f64 = (v733 - v730);
        self.scalar_v9312 = v9312;
        let v9313: f64 = (v733 / v9312);
        self.scalar_v9313 = v9313;
        let v9321: f64 = (v730 / v9309);
        self.scalar_v9321 = v9321;
        let v9338: f64 = (1.0 / v736);
        self.scalar_v9338 = v9338;
        let v9376: f64 = (0.5 * v16);
        self.scalar_v9376 = v9376;
        let v9556: bool = (!v720);
        self.scalar_v9556 = v9556;
        let v9557: bool = (v9556 && v742);
        self.scalar_v9557 = v9557;
        let v10461: bool = (v9556 && v1679);
        self.scalar_v10461 = v10461;
        let v10471: bool = (v9556 && v596);
        self.scalar_v10471 = v10471;
        let v10475: bool = (v9556 && v1693);
        self.scalar_v10475 = v10475;
        let v10476: bool = (v10475 && v1692);
        self.scalar_v10476 = v10476;
        let v10481: bool = (v10475 && v1700);
        self.scalar_v10481 = v10481;
        let v10492: bool = (v10475 && v1714);
        self.scalar_v10492 = v10492;
        let v10493: f64 = (if v10492 { 0.0 } else { 0.0 });
        self.scalar_v10493 = v10493;
        let v10494: bool = (v10475 && v1717);
        self.scalar_v10494 = v10494;
        let v10502: bool = (v10494 && v1726);
        self.scalar_v10502 = v10502;
        let v10503: f64 = (if v10502 { 0.0 } else { 0.0 });
        self.scalar_v10503 = v10503;
        let v10504: bool = (v10494 && v1729);
        self.scalar_v10504 = v10504;
        let v10529: bool = (v10475 && v1713);
        self.scalar_v10529 = v10529;
        let v10530: f64 = (if v10529 { 0.0 } else { 0.0 });
        self.scalar_v10530 = v10530;
        let v10531: bool = (v10475 && v1759);
        self.scalar_v10531 = v10531;
        let v10550: bool = (v10531 && v1783);
        self.scalar_v10550 = v10550;
        let v10555: bool = (v10531 && v1789);
        self.scalar_v10555 = v10555;
        let v10648: bool = (v10475 && v1885);
        self.scalar_v10648 = v10648;
        let v10649: f64 = (if v10648 { 0.0 } else { 0.0 });
        self.scalar_v10649 = v10649;
        let v10650: bool = (v10475 && v1888);
        self.scalar_v10650 = v10650;
        let v10651: bool = (v10650 && v1726);
        self.scalar_v10651 = v10651;
        let v10656: bool = (v10650 && v1729);
        self.scalar_v10656 = v10656;
        let v10733: bool = (v9556 && v606);
        self.scalar_v10733 = v10733;
        let v10737: bool = (v9556 && v1986);
        self.scalar_v10737 = v10737;
        let v10738: bool = (v10737 && v1985);
        self.scalar_v10738 = v10738;
        let v10743: bool = (v10737 && v1993);
        self.scalar_v10743 = v10743;
        let v10753: bool = (v10737 && v2006);
        self.scalar_v10753 = v10753;
        let v10759: bool = (v10737 && v2013);
        self.scalar_v10759 = v10759;
        let v10767: bool = (v10759 && v2022);
        self.scalar_v10767 = v10767;
        let v10769: bool = (v10759 && v2025);
        self.scalar_v10769 = v10769;
        let v10793: bool = (v10737 && v2005);
        self.scalar_v10793 = v10793;
        let v10795: bool = (v10737 && v2054);
        self.scalar_v10795 = v10795;
        let v10814: bool = (v10795 && v2077);
        self.scalar_v10814 = v10814;
        let v10819: bool = (v10795 && v2083);
        self.scalar_v10819 = v10819;
        let v10912: bool = (v10737 && v2177);
        self.scalar_v10912 = v10912;
        let v10914: bool = (v10737 && v2180);
        self.scalar_v10914 = v10914;
        let v10915: bool = (v10914 && v2022);
        self.scalar_v10915 = v10915;
        let v10920: bool = (v10914 && v2025);
        self.scalar_v10920 = v10920;
        let v10997: bool = (v9556 && v615);
        self.scalar_v10997 = v10997;
        let v11001: bool = (v9556 && v2275);
        self.scalar_v11001 = v11001;
        let v11002: bool = (v11001 && v2274);
        self.scalar_v11002 = v11002;
        let v11007: bool = (v11001 && v2282);
        self.scalar_v11007 = v11007;
        let v11017: bool = (v11001 && v2295);
        self.scalar_v11017 = v11017;
        let v11023: bool = (v11001 && v2302);
        self.scalar_v11023 = v11023;
        let v11031: bool = (v11023 && v2311);
        self.scalar_v11031 = v11031;
        let v11033: bool = (v11023 && v2314);
        self.scalar_v11033 = v11033;
        let v11057: bool = (v11001 && v2294);
        self.scalar_v11057 = v11057;
        let v11059: bool = (v11001 && v2343);
        self.scalar_v11059 = v11059;
        let v11078: bool = (v11059 && v2366);
        self.scalar_v11078 = v11078;
        let v11083: bool = (v11059 && v2372);
        self.scalar_v11083 = v11083;
        let v11176: bool = (v11001 && v2466);
        self.scalar_v11176 = v11176;
        let v11178: bool = (v11001 && v2469);
        self.scalar_v11178 = v11178;
        let v11179: bool = (v11178 && v2311);
        self.scalar_v11179 = v11179;
        let v11184: bool = (v11178 && v2314);
        self.scalar_v11184 = v11184;
        let v11280: f64 = p.p84;
        self.scalar_v11280 = v11280;
        let v11281: bool = (p.p84 > 0.0);
        self.scalar_v11281 = v11281;
        let v11282: bool = (v11281 && v806);
        self.scalar_v11282 = v11282;
        let v11354: bool = (v11281 && v884);
        self.scalar_v11354 = v11354;
        let v11405: f64 = p.p91;
        self.scalar_v11405 = v11405;
        let v11406: bool = (p.p91 == 0.0);
        self.scalar_v11406 = v11406;
        let v11410: f64 = p.p90;
        self.scalar_v11410 = v11410;
        let v11415: f64 = (-p.p91);
        self.scalar_v11415 = v11415;
        let v11419: f64 = p.p98;
        self.scalar_v11419 = v11419;
        let v11428: f64 = p.p79;
        self.scalar_v11428 = v11428;
        let v11437: f64 = p.p92;
        self.scalar_v11437 = v11437;
        let v11438: bool = (p.p92 > 0.0);
        self.scalar_v11438 = v11438;
        let v11439: bool = (v11281 && v11438);
        self.scalar_v11439 = v11439;
        let v11441: f64 = (1e-23 / v668);
        self.scalar_v11441 = v11441;
        let v11448: bool = (!v11438);
        self.scalar_v11448 = v11448;
        let v11449: bool = (v11281 && v11448);
        self.scalar_v11449 = v11449;
        let v11513: f64 = p.p95;
        self.scalar_v11513 = v11513;
        let v11514: bool = (p.p95 > 0.0);
        self.scalar_v11514 = v11514;
        let v11515: bool = (v11281 && v11514);
        self.scalar_v11515 = v11515;
        let v11516: f64 = (1.0 / v687);
        self.scalar_v11516 = v11516;
        let v11523: bool = (!v11514);
        self.scalar_v11523 = v11523;
        let v11524: bool = (v11281 && v11523);
        self.scalar_v11524 = v11524;
        let v11527: f64 = (v473 * v515);
        self.scalar_v11527 = v11527;
        let v11528: f64 = (v11527 * 1.6021918e-19);
        self.scalar_v11528 = v11528;
        let v11529: f64 = (-v11528);
        self.scalar_v11529 = v11529;
        let v11530: f64 = (v11529 * p.p94);
        self.scalar_v11530 = v11530;
        let v11531: f64 = (if v11281 { v11530 } else { 0.0 });
        self.scalar_v11531 = v11531;
        let v11533: f64 = (-p.p94);
        self.scalar_v11533 = v11533;
        let v11556: f64 = (if v11281 { 0.0 } else { v153 });
        self.scalar_v11556 = v11556;
        let v11560: f64 = p.p4;
        self.scalar_v11560 = v11560;
        let v11568: bool = (!v11439);
        self.scalar_v11568 = v11568;
        let v11569: f64 = (if v11568 { 0.0 } else { 0.0 });
        self.scalar_v11569 = v11569;
        let v11570: bool = (!v11515);
        self.scalar_v11570 = v11570;
        let v11571: f64 = (if v11570 { 0.0 } else { 0.0 });
        self.scalar_v11571 = v11571;
        let v11671: f64 = (if v720 { 1.0 } else { 0.0 });
        self.scalar_v11671 = v11671;
        let v11672: f64 = (if v720 { -1.0 } else { 0.0 });
        self.scalar_v11672 = v11672;
        let v11673: f64 = (if v720 { v11671 } else { 0.0 });
        self.scalar_v11673 = v11673;
        let v11674: f64 = (if v720 { v11672 } else { 0.0 });
        self.scalar_v11674 = v11674;
        let v11675: f64 = (-v11671);
        self.scalar_v11675 = v11675;
        let v11676: f64 = (-v11672);
        self.scalar_v11676 = v11676;
        let v11677: f64 = (if v720 { v11675 } else { 0.0 });
        self.scalar_v11677 = v11677;
        let v11678: f64 = (if v720 { v11676 } else { 0.0 });
        self.scalar_v11678 = v11678;
        let v11715: f64 = (v303 - 1.0);
        self.scalar_v11715 = v11715;
        let v11745: f64 = (v304 - 1.0);
        self.scalar_v11745 = v11745;
        let v11773: f64 = (v305 - 1.0);
        self.scalar_v11773 = v11773;
        let v11792: f64 = (if v9557 { 1.0 } else { v11671 });
        self.scalar_v11792 = v11792;
        let v11793: f64 = (if v9557 { -1.0 } else { v11672 });
        self.scalar_v11793 = v11793;
        let v11794: f64 = (if v9557 { v11792 } else { v11673 });
        self.scalar_v11794 = v11794;
        let v11795: f64 = (if v9557 { v11793 } else { v11674 });
        self.scalar_v11795 = v11795;
        let v11796: f64 = (-v11792);
        self.scalar_v11796 = v11796;
        let v11797: f64 = (-v11793);
        self.scalar_v11797 = v11797;
        let v11798: f64 = (if v9557 { v11796 } else { v11677 });
        self.scalar_v11798 = v11798;
        let v11799: f64 = (if v9557 { v11797 } else { v11678 });
        self.scalar_v11799 = v11799;
        let v11878: f64 = (p.p86 * -1.0);
        self.scalar_v11878 = v11878;
        let v13605: f64 = (v42 - 1.0);
        self.scalar_v13605 = v13605;
        let v13704: f64 = (v1782 - 1.0);
        self.scalar_v13704 = v13704;
        let v14185: f64 = (v47 - 1.0);
        self.scalar_v14185 = v14185;
        let v14286: f64 = (v2076 - 1.0);
        self.scalar_v14286 = v14286;
        let v14769: f64 = (v52 - 1.0);
        self.scalar_v14769 = v14769;
        let v14870: f64 = (v2365 - 1.0);
        self.scalar_v14870 = v14870;
        let v15282: f64 = (if v11282 { p.p86 } else { 0.0 });
        self.scalar_v15282 = v15282;
        let v15283: f64 = (if v11282 { v11878 } else { 0.0 });
        self.scalar_v15283 = v15283;
        let v15286: f64 = (-v15282);
        self.scalar_v15286 = v15286;
        let v15287: f64 = (-v15283);
        self.scalar_v15287 = v15287;
        let v15508: f64 = (v11415 * -1.0);
        self.scalar_v15508 = v15508;
        let v15541: f64 = (if v11439 { 1.0 } else { 0.0 });
        self.scalar_v15541 = v15541;
        let v15542: f64 = (v15541 / v11441);
        self.scalar_v15542 = v15542;
        let v15543: f64 = (if v11439 { v15542 } else { 0.0 });
        self.scalar_v15543 = v15543;
        let v15548: f64 = (if v11449 { 0.0 } else { v15543 });
        self.scalar_v15548 = v15548;
        let v15589: f64 = (if v11281 { -1.0 } else { 0.0 });
        self.scalar_v15589 = v15589;
        let v15590: f64 = (if v11281 { 1.0 } else { 0.0 });
        self.scalar_v15590 = v15590;
        let v15656: f64 = (if v11515 { 1.0 } else { 0.0 });
        self.scalar_v15656 = v15656;
        let v15657: f64 = (v15656 / v11516);
        self.scalar_v15657 = v15657;
        let v15658: f64 = (if v11515 { v15657 } else { 0.0 });
        self.scalar_v15658 = v15658;
        let v15663: f64 = (if v11524 { 0.0 } else { v15658 });
        self.scalar_v15663 = v15663;
        let v15669: f64 = (-v15663);
        self.scalar_v15669 = v15669;
        let v15695: f64 = (-v15669);
        self.scalar_v15695 = v15695;
    }
}
