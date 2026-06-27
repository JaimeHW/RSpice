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
            params.p0 = 200.0;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 1e-12;
            params.p4 = 1e-6;
            params.p5 = 1e-6;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 0.0;
            params.p10 = 1.0;
            params.p11 = 1.0;
            params.p12 = 1000.0;
            params.p13 = 21.0;
            params.p14 = 1000.0;
            params.p15 = 0.001;
            params.p16 = 1e-9;
            params.p17 = 1e-9;
            params.p18 = 1.0;
            params.p19 = 1.0;
            params.p20 = 1.0;
            params.p21 = 0.5;
            params.p22 = 0.5;
            params.p23 = 0.5;
            params.p24 = 1.16;
            params.p25 = 1.16;
            params.p26 = 1.16;
            params.p27 = 1e-12;
            params.p28 = 1e-18;
            params.p29 = 1e-18;
            params.p30 = 100.0;
            params.p31 = 0.0001;
            params.p32 = 0.0001;
            params.p33 = 1e-7;
            params.p34 = 1e-7;
            params.p35 = 100.0;
            params.p36 = 0.0001;
            params.p37 = 0.0001;
            params.p38 = 0.25;
            params.p39 = 0.25;
            params.p40 = 0.25;
            params.p41 = 1e-12;
            params.p42 = 1e-18;
            params.p43 = 1e-18;
            params.p44 = 1000000000.0;
            params.p45 = 1000000000.0;
            params.p46 = 1000000000.0;
            params.p47 = -0.001;
            params.p48 = -0.001;
            params.p49 = -0.001;
            params.p50 = 10.0;
            params.p51 = 10.0;
            params.p52 = 10.0;
            params.p53 = 4.0;
            params.p54 = 4.0;
            params.p55 = 4.0;
            params.p56 = 1.0;
            params.p57 = 1.0;
            params.p58 = 1.0;
            params.p59 = 1.0;
            params.p60 = -1.0;
            params.p61 = 0.1;
            params.p62 = 0.0;
            params.p63 = 2.5;
            params.p64 = 0.03;
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
    pub nodes: [usize; 2],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 65]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 1]>,
    pub(crate) ddt_state_previous: Box<[f64; 1]>,
    pub(crate) ddt_state_initialized: Box<[bool; 1]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v4: bool,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: bool,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
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
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
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
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: bool,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: bool,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: bool,
    pub(crate) scalar_v112: bool,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: bool,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: bool,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: bool,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v127: bool,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: bool,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v134: bool,
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
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: bool,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: bool,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: bool,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: bool,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v425: bool,
    pub(crate) scalar_v429: bool,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: bool,
    pub(crate) scalar_v438: bool,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: bool,
    pub(crate) scalar_v447: bool,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v459: bool,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: bool,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v466: f64,
    pub(crate) scalar_v468: bool,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: bool,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: bool,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v493: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: bool,
    pub(crate) scalar_v496: bool,
    pub(crate) scalar_v497: bool,
    pub(crate) scalar_v498: bool,
    pub(crate) scalar_v556: bool,
    pub(crate) scalar_v557: bool,
    pub(crate) scalar_v568: bool,
    pub(crate) scalar_v569: bool,
    pub(crate) scalar_v570: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v598: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v602: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v604: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v611: f64,
    pub(crate) scalar_v612: f64,
    pub(crate) scalar_v613: f64,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v615: f64,
    pub(crate) scalar_v616: bool,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v619: bool,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: bool,
    pub(crate) scalar_v624: f64,
    pub(crate) scalar_v625: bool,
    pub(crate) scalar_v626: bool,
    pub(crate) scalar_v627: bool,
    pub(crate) scalar_v628: f64,
    pub(crate) scalar_v629: bool,
    pub(crate) scalar_v630: bool,
    pub(crate) scalar_v638: bool,
    pub(crate) scalar_v639: bool,
    pub(crate) scalar_v640: f64,
    pub(crate) scalar_v641: bool,
    pub(crate) scalar_v642: bool,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v669: bool,
    pub(crate) scalar_v670: f64,
    pub(crate) scalar_v671: bool,
    pub(crate) scalar_v672: bool,
    pub(crate) scalar_v692: f64,
    pub(crate) scalar_v693: f64,
    pub(crate) scalar_v694: bool,
    pub(crate) scalar_v695: bool,
    pub(crate) scalar_v700: bool,
    pub(crate) scalar_v701: bool,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v797: bool,
    pub(crate) scalar_v798: bool,
    pub(crate) scalar_v799: f64,
    pub(crate) scalar_v800: bool,
    pub(crate) scalar_v801: bool,
    pub(crate) scalar_v802: bool,
    pub(crate) scalar_v803: f64,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v805: f64,
    pub(crate) scalar_v807: bool,
    pub(crate) scalar_v808: f64,
    pub(crate) scalar_v810: f64,
    pub(crate) scalar_v853: bool,
    pub(crate) scalar_v854: bool,
    pub(crate) scalar_v855: f64,
    pub(crate) scalar_v856: f64,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v858: bool,
    pub(crate) scalar_v859: bool,
    pub(crate) scalar_v860: bool,
    pub(crate) scalar_v861: bool,
    pub(crate) scalar_v862: bool,
    pub(crate) scalar_v863: bool,
    pub(crate) scalar_v864: f64,
    pub(crate) scalar_v865: f64,
    pub(crate) scalar_v866: f64,
    pub(crate) scalar_v867: f64,
    pub(crate) scalar_v869: bool,
    pub(crate) scalar_v870: bool,
    pub(crate) scalar_v871: f64,
    pub(crate) scalar_v872: f64,
    pub(crate) scalar_v877: bool,
    pub(crate) scalar_v878: bool,
    pub(crate) scalar_v879: f64,
    pub(crate) scalar_v880: f64,
    pub(crate) scalar_v881: f64,
    pub(crate) scalar_v882: f64,
    pub(crate) scalar_v884: f64,
    pub(crate) scalar_v891: bool,
    pub(crate) scalar_v892: f64,
    pub(crate) scalar_v893: bool,
    pub(crate) scalar_v894: bool,
    pub(crate) scalar_v897: f64,
    pub(crate) scalar_v898: bool,
    pub(crate) scalar_v899: f64,
    pub(crate) scalar_v900: bool,
    pub(crate) scalar_v901: bool,
    pub(crate) scalar_v902: bool,
    pub(crate) scalar_v904: bool,
    pub(crate) scalar_v905: bool,
    pub(crate) scalar_v913: bool,
    pub(crate) scalar_v914: bool,
    pub(crate) scalar_v916: bool,
    pub(crate) scalar_v917: bool,
    pub(crate) scalar_v924: f64,
    pub(crate) scalar_v925: f64,
    pub(crate) scalar_v943: bool,
    pub(crate) scalar_v945: bool,
    pub(crate) scalar_v946: bool,
    pub(crate) scalar_v965: f64,
    pub(crate) scalar_v966: f64,
    pub(crate) scalar_v967: bool,
    pub(crate) scalar_v968: bool,
    pub(crate) scalar_v973: bool,
    pub(crate) scalar_v974: bool,
    pub(crate) scalar_v1067: f64,
    pub(crate) scalar_v1068: bool,
    pub(crate) scalar_v1069: bool,
    pub(crate) scalar_v1071: bool,
    pub(crate) scalar_v1072: bool,
    pub(crate) scalar_v1073: bool,
    pub(crate) scalar_v1074: f64,
    pub(crate) scalar_v1075: f64,
    pub(crate) scalar_v1076: f64,
    pub(crate) scalar_v1078: bool,
    pub(crate) scalar_v1079: f64,
    pub(crate) scalar_v1081: f64,
    pub(crate) scalar_v1123: bool,
    pub(crate) scalar_v1124: bool,
    pub(crate) scalar_v1126: f64,
    pub(crate) scalar_v1127: bool,
    pub(crate) scalar_v1128: bool,
    pub(crate) scalar_v1129: bool,
    pub(crate) scalar_v1130: bool,
    pub(crate) scalar_v1131: bool,
    pub(crate) scalar_v1132: bool,
    pub(crate) scalar_v1133: f64,
    pub(crate) scalar_v1134: f64,
    pub(crate) scalar_v1135: f64,
    pub(crate) scalar_v1136: f64,
    pub(crate) scalar_v1138: bool,
    pub(crate) scalar_v1139: bool,
    pub(crate) scalar_v1140: f64,
    pub(crate) scalar_v1141: f64,
    pub(crate) scalar_v1146: bool,
    pub(crate) scalar_v1147: bool,
    pub(crate) scalar_v1148: f64,
    pub(crate) scalar_v1149: f64,
    pub(crate) scalar_v1150: f64,
    pub(crate) scalar_v1151: f64,
    pub(crate) scalar_v1159: bool,
    pub(crate) scalar_v1160: f64,
    pub(crate) scalar_v1161: bool,
    pub(crate) scalar_v1162: bool,
    pub(crate) scalar_v1165: f64,
    pub(crate) scalar_v1166: bool,
    pub(crate) scalar_v1167: f64,
    pub(crate) scalar_v1168: bool,
    pub(crate) scalar_v1169: bool,
    pub(crate) scalar_v1170: bool,
    pub(crate) scalar_v1172: bool,
    pub(crate) scalar_v1173: bool,
    pub(crate) scalar_v1181: bool,
    pub(crate) scalar_v1182: bool,
    pub(crate) scalar_v1184: bool,
    pub(crate) scalar_v1185: bool,
    pub(crate) scalar_v1192: f64,
    pub(crate) scalar_v1193: f64,
    pub(crate) scalar_v1211: bool,
    pub(crate) scalar_v1213: bool,
    pub(crate) scalar_v1214: bool,
    pub(crate) scalar_v1233: f64,
    pub(crate) scalar_v1234: f64,
    pub(crate) scalar_v1235: bool,
    pub(crate) scalar_v1236: bool,
    pub(crate) scalar_v1241: bool,
    pub(crate) scalar_v1242: bool,
    pub(crate) scalar_v1335: f64,
    pub(crate) scalar_v1336: bool,
    pub(crate) scalar_v1337: bool,
    pub(crate) scalar_v1339: bool,
    pub(crate) scalar_v1340: bool,
    pub(crate) scalar_v1341: bool,
    pub(crate) scalar_v1342: f64,
    pub(crate) scalar_v1343: f64,
    pub(crate) scalar_v1344: f64,
    pub(crate) scalar_v1346: bool,
    pub(crate) scalar_v1347: f64,
    pub(crate) scalar_v1349: f64,
    pub(crate) scalar_v1391: bool,
    pub(crate) scalar_v1392: bool,
    pub(crate) scalar_v1394: f64,
    pub(crate) scalar_v1395: bool,
    pub(crate) scalar_v1396: bool,
    pub(crate) scalar_v1397: bool,
    pub(crate) scalar_v1398: bool,
    pub(crate) scalar_v1399: bool,
    pub(crate) scalar_v1400: bool,
    pub(crate) scalar_v1401: f64,
    pub(crate) scalar_v1402: f64,
    pub(crate) scalar_v1403: f64,
    pub(crate) scalar_v1404: f64,
    pub(crate) scalar_v1406: bool,
    pub(crate) scalar_v1407: bool,
    pub(crate) scalar_v1408: f64,
    pub(crate) scalar_v1409: f64,
    pub(crate) scalar_v1414: bool,
    pub(crate) scalar_v1415: bool,
    pub(crate) scalar_v1416: f64,
    pub(crate) scalar_v1417: f64,
    pub(crate) scalar_v1418: f64,
    pub(crate) scalar_v1419: f64,
    pub(crate) scalar_v1433: f64,
    pub(crate) scalar_v1487: bool,
    pub(crate) scalar_v1488: bool,
    pub(crate) scalar_v1499: bool,
    pub(crate) scalar_v1500: bool,
    pub(crate) scalar_v1501: f64,
    pub(crate) scalar_v1525: f64,
    pub(crate) scalar_v1526: f64,
    pub(crate) scalar_v1527: f64,
    pub(crate) scalar_v1528: f64,
    pub(crate) scalar_v1529: f64,
    pub(crate) scalar_v1530: f64,
    pub(crate) scalar_v1531: f64,
    pub(crate) scalar_v1532: f64,
    pub(crate) scalar_v1533: f64,
    pub(crate) scalar_v1534: f64,
    pub(crate) scalar_v1535: f64,
    pub(crate) scalar_v1536: f64,
    pub(crate) scalar_v1537: f64,
    pub(crate) scalar_v1538: f64,
    pub(crate) scalar_v1539: f64,
    pub(crate) scalar_v1540: f64,
    pub(crate) scalar_v1692: f64,
    pub(crate) scalar_v1693: f64,
    pub(crate) scalar_v1694: f64,
    pub(crate) scalar_v1696: f64,
    pub(crate) scalar_v1698: f64,
    pub(crate) scalar_v1740: bool,
    pub(crate) scalar_v1741: bool,
    pub(crate) scalar_v1742: bool,
    pub(crate) scalar_v1743: f64,
    pub(crate) scalar_v1744: f64,
    pub(crate) scalar_v1745: f64,
    pub(crate) scalar_v1746: f64,
    pub(crate) scalar_v1748: bool,
    pub(crate) scalar_v1749: f64,
    pub(crate) scalar_v1750: f64,
    pub(crate) scalar_v1755: bool,
    pub(crate) scalar_v1756: bool,
    pub(crate) scalar_v1757: f64,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1917: f64,
    pub(crate) scalar_v1918: f64,
    pub(crate) scalar_v1919: f64,
    pub(crate) scalar_v1921: f64,
    pub(crate) scalar_v1923: f64,
    pub(crate) scalar_v1965: bool,
    pub(crate) scalar_v1966: bool,
    pub(crate) scalar_v1967: bool,
    pub(crate) scalar_v1968: f64,
    pub(crate) scalar_v1969: f64,
    pub(crate) scalar_v1970: f64,
    pub(crate) scalar_v1971: f64,
    pub(crate) scalar_v1973: bool,
    pub(crate) scalar_v1974: f64,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1980: bool,
    pub(crate) scalar_v1981: bool,
    pub(crate) scalar_v1982: f64,
    pub(crate) scalar_v1983: f64,
    pub(crate) scalar_v1984: f64,
    pub(crate) scalar_v2142: f64,
    pub(crate) scalar_v2143: f64,
    pub(crate) scalar_v2144: f64,
    pub(crate) scalar_v2146: f64,
    pub(crate) scalar_v2148: f64,
    pub(crate) scalar_v2190: bool,
    pub(crate) scalar_v2191: bool,
    pub(crate) scalar_v2192: bool,
    pub(crate) scalar_v2193: f64,
    pub(crate) scalar_v2194: f64,
    pub(crate) scalar_v2195: f64,
    pub(crate) scalar_v2196: f64,
    pub(crate) scalar_v2198: bool,
    pub(crate) scalar_v2199: f64,
    pub(crate) scalar_v2200: f64,
    pub(crate) scalar_v2205: bool,
    pub(crate) scalar_v2206: bool,
    pub(crate) scalar_v2207: f64,
    pub(crate) scalar_v2208: f64,
    pub(crate) scalar_v2209: f64,
    pub(crate) scalar_v2223: f64,
    pub(crate) scalar_v2277: bool,
    pub(crate) scalar_v2278: bool,
    pub(crate) scalar_v2289: bool,
    pub(crate) scalar_v2290: bool,
    pub(crate) scalar_v2291: f64,
    pub(crate) scalar_v2315: f64,
    pub(crate) scalar_v2316: f64,
    pub(crate) scalar_v2317: f64,
    pub(crate) scalar_v2318: f64,
    pub(crate) scalar_v2319: f64,
    pub(crate) scalar_v2320: f64,
    pub(crate) scalar_v2321: f64,
    pub(crate) scalar_v2322: f64,
    pub(crate) scalar_v2323: f64,
    pub(crate) scalar_v2324: f64,
    pub(crate) scalar_v2325: f64,
    pub(crate) scalar_v2326: f64,
    pub(crate) scalar_v2327: f64,
    pub(crate) scalar_v2328: f64,
    pub(crate) scalar_v2329: f64,
    pub(crate) scalar_v2330: f64,
    pub(crate) scalar_v2482: f64,
    pub(crate) scalar_v2483: f64,
    pub(crate) scalar_v2484: f64,
    pub(crate) scalar_v2486: f64,
    pub(crate) scalar_v2488: f64,
    pub(crate) scalar_v2530: bool,
    pub(crate) scalar_v2531: bool,
    pub(crate) scalar_v2532: bool,
    pub(crate) scalar_v2533: f64,
    pub(crate) scalar_v2534: f64,
    pub(crate) scalar_v2535: f64,
    pub(crate) scalar_v2536: f64,
    pub(crate) scalar_v2538: bool,
    pub(crate) scalar_v2539: f64,
    pub(crate) scalar_v2540: f64,
    pub(crate) scalar_v2545: bool,
    pub(crate) scalar_v2546: bool,
    pub(crate) scalar_v2547: f64,
    pub(crate) scalar_v2548: f64,
    pub(crate) scalar_v2549: f64,
    pub(crate) scalar_v2707: f64,
    pub(crate) scalar_v2708: f64,
    pub(crate) scalar_v2709: f64,
    pub(crate) scalar_v2711: f64,
    pub(crate) scalar_v2713: f64,
    pub(crate) scalar_v2755: bool,
    pub(crate) scalar_v2756: bool,
    pub(crate) scalar_v2757: bool,
    pub(crate) scalar_v2758: f64,
    pub(crate) scalar_v2759: f64,
    pub(crate) scalar_v2760: f64,
    pub(crate) scalar_v2761: f64,
    pub(crate) scalar_v2763: bool,
    pub(crate) scalar_v2764: f64,
    pub(crate) scalar_v2765: f64,
    pub(crate) scalar_v2770: bool,
    pub(crate) scalar_v2771: bool,
    pub(crate) scalar_v2772: f64,
    pub(crate) scalar_v2773: f64,
    pub(crate) scalar_v2774: f64,
    pub(crate) scalar_v2932: f64,
    pub(crate) scalar_v2933: f64,
    pub(crate) scalar_v2934: f64,
    pub(crate) scalar_v2936: f64,
    pub(crate) scalar_v2938: f64,
    pub(crate) scalar_v2980: bool,
    pub(crate) scalar_v2981: bool,
    pub(crate) scalar_v2982: bool,
    pub(crate) scalar_v2983: f64,
    pub(crate) scalar_v2984: f64,
    pub(crate) scalar_v2985: f64,
    pub(crate) scalar_v2986: f64,
    pub(crate) scalar_v2988: bool,
    pub(crate) scalar_v2989: f64,
    pub(crate) scalar_v2990: f64,
    pub(crate) scalar_v2995: bool,
    pub(crate) scalar_v2996: bool,
    pub(crate) scalar_v2997: f64,
    pub(crate) scalar_v2998: f64,
    pub(crate) scalar_v2999: f64,
    pub(crate) scalar_v3013: f64,
    pub(crate) scalar_v3067: bool,
    pub(crate) scalar_v3068: bool,
    pub(crate) scalar_v3079: bool,
    pub(crate) scalar_v3080: bool,
    pub(crate) scalar_v3081: f64,
    pub(crate) scalar_v3105: f64,
    pub(crate) scalar_v3106: f64,
    pub(crate) scalar_v3107: f64,
    pub(crate) scalar_v3108: f64,
    pub(crate) scalar_v3109: f64,
    pub(crate) scalar_v3110: f64,
    pub(crate) scalar_v3111: f64,
    pub(crate) scalar_v3112: f64,
    pub(crate) scalar_v3113: f64,
    pub(crate) scalar_v3114: f64,
    pub(crate) scalar_v3115: f64,
    pub(crate) scalar_v3116: f64,
    pub(crate) scalar_v3117: f64,
    pub(crate) scalar_v3118: f64,
    pub(crate) scalar_v3119: f64,
    pub(crate) scalar_v3120: f64,
    pub(crate) scalar_v3272: f64,
    pub(crate) scalar_v3273: f64,
    pub(crate) scalar_v3274: f64,
    pub(crate) scalar_v3276: f64,
    pub(crate) scalar_v3278: f64,
    pub(crate) scalar_v3320: bool,
    pub(crate) scalar_v3321: bool,
    pub(crate) scalar_v3322: bool,
    pub(crate) scalar_v3323: f64,
    pub(crate) scalar_v3324: f64,
    pub(crate) scalar_v3325: f64,
    pub(crate) scalar_v3326: f64,
    pub(crate) scalar_v3328: bool,
    pub(crate) scalar_v3329: f64,
    pub(crate) scalar_v3330: f64,
    pub(crate) scalar_v3335: bool,
    pub(crate) scalar_v3336: bool,
    pub(crate) scalar_v3337: f64,
    pub(crate) scalar_v3338: f64,
    pub(crate) scalar_v3339: f64,
    pub(crate) scalar_v3497: f64,
    pub(crate) scalar_v3498: f64,
    pub(crate) scalar_v3499: f64,
    pub(crate) scalar_v3501: f64,
    pub(crate) scalar_v3503: f64,
    pub(crate) scalar_v3545: bool,
    pub(crate) scalar_v3546: bool,
    pub(crate) scalar_v3547: bool,
    pub(crate) scalar_v3548: f64,
    pub(crate) scalar_v3549: f64,
    pub(crate) scalar_v3550: f64,
    pub(crate) scalar_v3551: f64,
    pub(crate) scalar_v3553: bool,
    pub(crate) scalar_v3554: f64,
    pub(crate) scalar_v3555: f64,
    pub(crate) scalar_v3560: bool,
    pub(crate) scalar_v3561: bool,
    pub(crate) scalar_v3562: f64,
    pub(crate) scalar_v3563: f64,
    pub(crate) scalar_v3564: f64,
    pub(crate) scalar_v3722: f64,
    pub(crate) scalar_v3723: f64,
    pub(crate) scalar_v3724: f64,
    pub(crate) scalar_v3726: f64,
    pub(crate) scalar_v3728: f64,
    pub(crate) scalar_v3770: bool,
    pub(crate) scalar_v3771: bool,
    pub(crate) scalar_v3772: bool,
    pub(crate) scalar_v3773: f64,
    pub(crate) scalar_v3774: f64,
    pub(crate) scalar_v3775: f64,
    pub(crate) scalar_v3776: f64,
    pub(crate) scalar_v3778: bool,
    pub(crate) scalar_v3779: f64,
    pub(crate) scalar_v3780: f64,
    pub(crate) scalar_v3785: bool,
    pub(crate) scalar_v3786: bool,
    pub(crate) scalar_v3787: f64,
    pub(crate) scalar_v3788: f64,
    pub(crate) scalar_v3789: f64,
    pub(crate) scalar_v3803: f64,
    pub(crate) scalar_v3857: bool,
    pub(crate) scalar_v3858: bool,
    pub(crate) scalar_v3869: bool,
    pub(crate) scalar_v3870: bool,
    pub(crate) scalar_v3871: f64,
    pub(crate) scalar_v3895: f64,
    pub(crate) scalar_v3896: f64,
    pub(crate) scalar_v3897: f64,
    pub(crate) scalar_v3898: f64,
    pub(crate) scalar_v3899: f64,
    pub(crate) scalar_v3900: f64,
    pub(crate) scalar_v3901: f64,
    pub(crate) scalar_v3902: f64,
    pub(crate) scalar_v3903: f64,
    pub(crate) scalar_v3904: f64,
    pub(crate) scalar_v3905: f64,
    pub(crate) scalar_v3906: f64,
    pub(crate) scalar_v3907: f64,
    pub(crate) scalar_v3908: f64,
    pub(crate) scalar_v3909: f64,
    pub(crate) scalar_v3910: f64,
    pub(crate) scalar_v4062: f64,
    pub(crate) scalar_v4063: f64,
    pub(crate) scalar_v4064: f64,
    pub(crate) scalar_v4066: f64,
    pub(crate) scalar_v4068: f64,
    pub(crate) scalar_v4110: bool,
    pub(crate) scalar_v4111: bool,
    pub(crate) scalar_v4112: bool,
    pub(crate) scalar_v4113: f64,
    pub(crate) scalar_v4114: f64,
    pub(crate) scalar_v4115: f64,
    pub(crate) scalar_v4116: f64,
    pub(crate) scalar_v4118: bool,
    pub(crate) scalar_v4119: f64,
    pub(crate) scalar_v4120: f64,
    pub(crate) scalar_v4125: bool,
    pub(crate) scalar_v4126: bool,
    pub(crate) scalar_v4127: f64,
    pub(crate) scalar_v4128: f64,
    pub(crate) scalar_v4129: f64,
    pub(crate) scalar_v4287: f64,
    pub(crate) scalar_v4288: f64,
    pub(crate) scalar_v4289: f64,
    pub(crate) scalar_v4291: f64,
    pub(crate) scalar_v4293: f64,
    pub(crate) scalar_v4335: bool,
    pub(crate) scalar_v4336: bool,
    pub(crate) scalar_v4337: bool,
    pub(crate) scalar_v4338: f64,
    pub(crate) scalar_v4339: f64,
    pub(crate) scalar_v4340: f64,
    pub(crate) scalar_v4341: f64,
    pub(crate) scalar_v4343: bool,
    pub(crate) scalar_v4344: f64,
    pub(crate) scalar_v4345: f64,
    pub(crate) scalar_v4350: bool,
    pub(crate) scalar_v4351: bool,
    pub(crate) scalar_v4352: f64,
    pub(crate) scalar_v4353: f64,
    pub(crate) scalar_v4354: f64,
    pub(crate) scalar_v4512: f64,
    pub(crate) scalar_v4513: f64,
    pub(crate) scalar_v4514: f64,
    pub(crate) scalar_v4516: f64,
    pub(crate) scalar_v4518: f64,
    pub(crate) scalar_v4560: bool,
    pub(crate) scalar_v4561: bool,
    pub(crate) scalar_v4562: bool,
    pub(crate) scalar_v4563: f64,
    pub(crate) scalar_v4564: f64,
    pub(crate) scalar_v4565: f64,
    pub(crate) scalar_v4566: f64,
    pub(crate) scalar_v4568: bool,
    pub(crate) scalar_v4569: f64,
    pub(crate) scalar_v4570: f64,
    pub(crate) scalar_v4575: bool,
    pub(crate) scalar_v4576: bool,
    pub(crate) scalar_v4577: f64,
    pub(crate) scalar_v4578: f64,
    pub(crate) scalar_v4579: f64,
    pub(crate) scalar_v4630: f64,
    pub(crate) scalar_v4694: f64,
    pub(crate) scalar_v4697: f64,
    pub(crate) scalar_v4698: f64,
    pub(crate) scalar_v4706: f64,
    pub(crate) scalar_v4723: f64,
    pub(crate) scalar_v4744: f64,
    pub(crate) scalar_v4761: f64,
    pub(crate) scalar_v4791: f64,
    pub(crate) scalar_v4880: bool,
    pub(crate) scalar_v4887: bool,
    pub(crate) scalar_v4898: bool,
    pub(crate) scalar_v4905: bool,
    pub(crate) scalar_v4915: bool,
    pub(crate) scalar_v4922: bool,
    pub(crate) scalar_v4931: bool,
    pub(crate) scalar_v4932: f64,
    pub(crate) scalar_v4933: bool,
    pub(crate) scalar_v5055: bool,
    pub(crate) scalar_v5056: f64,
    pub(crate) scalar_v5058: bool,
    pub(crate) scalar_v5061: bool,
    pub(crate) scalar_v5062: f64,
    pub(crate) scalar_v5063: bool,
    pub(crate) scalar_v5071: bool,
    pub(crate) scalar_v5072: f64,
    pub(crate) scalar_v5073: bool,
    pub(crate) scalar_v5098: bool,
    pub(crate) scalar_v5099: f64,
    pub(crate) scalar_v5100: bool,
    pub(crate) scalar_v5118: bool,
    pub(crate) scalar_v5123: bool,
    pub(crate) scalar_v5216: bool,
    pub(crate) scalar_v5217: f64,
    pub(crate) scalar_v5218: bool,
    pub(crate) scalar_v5219: bool,
    pub(crate) scalar_v5224: bool,
    pub(crate) scalar_v5268: bool,
    pub(crate) scalar_v5269: f64,
    pub(crate) scalar_v5271: bool,
    pub(crate) scalar_v5298: bool,
    pub(crate) scalar_v5303: bool,
    pub(crate) scalar_v5306: f64,
    pub(crate) scalar_v5314: bool,
    pub(crate) scalar_v5315: f64,
    pub(crate) scalar_v5317: bool,
    pub(crate) scalar_v5320: bool,
    pub(crate) scalar_v5322: bool,
    pub(crate) scalar_v5330: bool,
    pub(crate) scalar_v5332: bool,
    pub(crate) scalar_v5356: bool,
    pub(crate) scalar_v5358: bool,
    pub(crate) scalar_v5376: bool,
    pub(crate) scalar_v5381: bool,
    pub(crate) scalar_v5474: bool,
    pub(crate) scalar_v5476: bool,
    pub(crate) scalar_v5477: bool,
    pub(crate) scalar_v5482: bool,
    pub(crate) scalar_v5526: bool,
    pub(crate) scalar_v5529: bool,
    pub(crate) scalar_v5556: bool,
    pub(crate) scalar_v5561: bool,
    pub(crate) scalar_v5570: bool,
    pub(crate) scalar_v5571: f64,
    pub(crate) scalar_v5573: bool,
    pub(crate) scalar_v5576: bool,
    pub(crate) scalar_v5578: bool,
    pub(crate) scalar_v5586: bool,
    pub(crate) scalar_v5588: bool,
    pub(crate) scalar_v5612: bool,
    pub(crate) scalar_v5614: bool,
    pub(crate) scalar_v5632: bool,
    pub(crate) scalar_v5637: bool,
    pub(crate) scalar_v5730: bool,
    pub(crate) scalar_v5732: bool,
    pub(crate) scalar_v5733: bool,
    pub(crate) scalar_v5738: bool,
    pub(crate) scalar_v5782: bool,
    pub(crate) scalar_v5785: bool,
    pub(crate) scalar_v5812: bool,
    pub(crate) scalar_v5813: f64,
    pub(crate) scalar_v5816: f64,
    pub(crate) scalar_v5856: bool,
    pub(crate) scalar_v5861: bool,
    pub(crate) scalar_v5892: bool,
    pub(crate) scalar_v5893: bool,
    pub(crate) scalar_v5898: bool,
    pub(crate) scalar_v5899: bool,
    pub(crate) scalar_v5911: bool,
    pub(crate) scalar_v5912: bool,
    pub(crate) scalar_v5913: bool,
    pub(crate) scalar_v5918: bool,
    pub(crate) scalar_v5938: f64,
    pub(crate) scalar_v5939: f64,
    pub(crate) scalar_v5940: f64,
    pub(crate) scalar_v5941: f64,
    pub(crate) scalar_v5942: f64,
    pub(crate) scalar_v5945: f64,
    pub(crate) scalar_v6010: f64,
    pub(crate) scalar_v6011: f64,
    pub(crate) scalar_v6047: f64,
    pub(crate) scalar_v6048: f64,
    pub(crate) scalar_v6049: f64,
    pub(crate) scalar_v6050: f64,
    pub(crate) scalar_v6051: f64,
    pub(crate) scalar_v6052: f64,
    pub(crate) scalar_v6053: f64,
    pub(crate) scalar_v6054: f64,
    pub(crate) scalar_v6092: f64,
    pub(crate) scalar_v6120: f64,
    pub(crate) scalar_v6146: f64,
    pub(crate) scalar_v6163: f64,
    pub(crate) scalar_v6164: f64,
    pub(crate) scalar_v6165: f64,
    pub(crate) scalar_v6166: f64,
    pub(crate) scalar_v6167: f64,
    pub(crate) scalar_v6168: f64,
    pub(crate) scalar_v6169: f64,
    pub(crate) scalar_v6170: f64,
    pub(crate) scalar_v6450: f64,
    pub(crate) scalar_v6558: f64,
    pub(crate) scalar_v7061: f64,
    pub(crate) scalar_v7171: f64,
    pub(crate) scalar_v7674: f64,
    pub(crate) scalar_v7784: f64,
    pub(crate) scalar_v8188: f64,
    pub(crate) scalar_v8189: f64,
    pub(crate) scalar_v8198: f64,
    pub(crate) scalar_v8199: f64,
    pub(crate) scalar_v8328: f64,
    pub(crate) scratch: Option<Box<GenericScratch<668, 2, 0>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<668, 2, 0>>>,
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
            scalar_v2: self.scalar_v2,
            scalar_v4: self.scalar_v4,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
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
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
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
            scalar_v146: self.scalar_v146,
            scalar_v148: self.scalar_v148,
            scalar_v166: self.scalar_v166,
            scalar_v172: self.scalar_v172,
            scalar_v178: self.scalar_v178,
            scalar_v184: self.scalar_v184,
            scalar_v187: self.scalar_v187,
            scalar_v190: self.scalar_v190,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v314: self.scalar_v314,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v359: self.scalar_v359,
            scalar_v425: self.scalar_v425,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v441: self.scalar_v441,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v447: self.scalar_v447,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v464: self.scalar_v464,
            scalar_v465: self.scalar_v465,
            scalar_v466: self.scalar_v466,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v476: self.scalar_v476,
            scalar_v478: self.scalar_v478,
            scalar_v480: self.scalar_v480,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v484: self.scalar_v484,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v491: self.scalar_v491,
            scalar_v493: self.scalar_v493,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v556: self.scalar_v556,
            scalar_v557: self.scalar_v557,
            scalar_v568: self.scalar_v568,
            scalar_v569: self.scalar_v569,
            scalar_v570: self.scalar_v570,
            scalar_v596: self.scalar_v596,
            scalar_v597: self.scalar_v597,
            scalar_v598: self.scalar_v598,
            scalar_v599: self.scalar_v599,
            scalar_v600: self.scalar_v600,
            scalar_v601: self.scalar_v601,
            scalar_v602: self.scalar_v602,
            scalar_v603: self.scalar_v603,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v606: self.scalar_v606,
            scalar_v607: self.scalar_v607,
            scalar_v608: self.scalar_v608,
            scalar_v611: self.scalar_v611,
            scalar_v612: self.scalar_v612,
            scalar_v613: self.scalar_v613,
            scalar_v614: self.scalar_v614,
            scalar_v615: self.scalar_v615,
            scalar_v616: self.scalar_v616,
            scalar_v617: self.scalar_v617,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v624: self.scalar_v624,
            scalar_v625: self.scalar_v625,
            scalar_v626: self.scalar_v626,
            scalar_v627: self.scalar_v627,
            scalar_v628: self.scalar_v628,
            scalar_v629: self.scalar_v629,
            scalar_v630: self.scalar_v630,
            scalar_v638: self.scalar_v638,
            scalar_v639: self.scalar_v639,
            scalar_v640: self.scalar_v640,
            scalar_v641: self.scalar_v641,
            scalar_v642: self.scalar_v642,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v669: self.scalar_v669,
            scalar_v670: self.scalar_v670,
            scalar_v671: self.scalar_v671,
            scalar_v672: self.scalar_v672,
            scalar_v692: self.scalar_v692,
            scalar_v693: self.scalar_v693,
            scalar_v694: self.scalar_v694,
            scalar_v695: self.scalar_v695,
            scalar_v700: self.scalar_v700,
            scalar_v701: self.scalar_v701,
            scalar_v796: self.scalar_v796,
            scalar_v797: self.scalar_v797,
            scalar_v798: self.scalar_v798,
            scalar_v799: self.scalar_v799,
            scalar_v800: self.scalar_v800,
            scalar_v801: self.scalar_v801,
            scalar_v802: self.scalar_v802,
            scalar_v803: self.scalar_v803,
            scalar_v804: self.scalar_v804,
            scalar_v805: self.scalar_v805,
            scalar_v807: self.scalar_v807,
            scalar_v808: self.scalar_v808,
            scalar_v810: self.scalar_v810,
            scalar_v853: self.scalar_v853,
            scalar_v854: self.scalar_v854,
            scalar_v855: self.scalar_v855,
            scalar_v856: self.scalar_v856,
            scalar_v857: self.scalar_v857,
            scalar_v858: self.scalar_v858,
            scalar_v859: self.scalar_v859,
            scalar_v860: self.scalar_v860,
            scalar_v861: self.scalar_v861,
            scalar_v862: self.scalar_v862,
            scalar_v863: self.scalar_v863,
            scalar_v864: self.scalar_v864,
            scalar_v865: self.scalar_v865,
            scalar_v866: self.scalar_v866,
            scalar_v867: self.scalar_v867,
            scalar_v869: self.scalar_v869,
            scalar_v870: self.scalar_v870,
            scalar_v871: self.scalar_v871,
            scalar_v872: self.scalar_v872,
            scalar_v877: self.scalar_v877,
            scalar_v878: self.scalar_v878,
            scalar_v879: self.scalar_v879,
            scalar_v880: self.scalar_v880,
            scalar_v881: self.scalar_v881,
            scalar_v882: self.scalar_v882,
            scalar_v884: self.scalar_v884,
            scalar_v891: self.scalar_v891,
            scalar_v892: self.scalar_v892,
            scalar_v893: self.scalar_v893,
            scalar_v894: self.scalar_v894,
            scalar_v897: self.scalar_v897,
            scalar_v898: self.scalar_v898,
            scalar_v899: self.scalar_v899,
            scalar_v900: self.scalar_v900,
            scalar_v901: self.scalar_v901,
            scalar_v902: self.scalar_v902,
            scalar_v904: self.scalar_v904,
            scalar_v905: self.scalar_v905,
            scalar_v913: self.scalar_v913,
            scalar_v914: self.scalar_v914,
            scalar_v916: self.scalar_v916,
            scalar_v917: self.scalar_v917,
            scalar_v924: self.scalar_v924,
            scalar_v925: self.scalar_v925,
            scalar_v943: self.scalar_v943,
            scalar_v945: self.scalar_v945,
            scalar_v946: self.scalar_v946,
            scalar_v965: self.scalar_v965,
            scalar_v966: self.scalar_v966,
            scalar_v967: self.scalar_v967,
            scalar_v968: self.scalar_v968,
            scalar_v973: self.scalar_v973,
            scalar_v974: self.scalar_v974,
            scalar_v1067: self.scalar_v1067,
            scalar_v1068: self.scalar_v1068,
            scalar_v1069: self.scalar_v1069,
            scalar_v1071: self.scalar_v1071,
            scalar_v1072: self.scalar_v1072,
            scalar_v1073: self.scalar_v1073,
            scalar_v1074: self.scalar_v1074,
            scalar_v1075: self.scalar_v1075,
            scalar_v1076: self.scalar_v1076,
            scalar_v1078: self.scalar_v1078,
            scalar_v1079: self.scalar_v1079,
            scalar_v1081: self.scalar_v1081,
            scalar_v1123: self.scalar_v1123,
            scalar_v1124: self.scalar_v1124,
            scalar_v1126: self.scalar_v1126,
            scalar_v1127: self.scalar_v1127,
            scalar_v1128: self.scalar_v1128,
            scalar_v1129: self.scalar_v1129,
            scalar_v1130: self.scalar_v1130,
            scalar_v1131: self.scalar_v1131,
            scalar_v1132: self.scalar_v1132,
            scalar_v1133: self.scalar_v1133,
            scalar_v1134: self.scalar_v1134,
            scalar_v1135: self.scalar_v1135,
            scalar_v1136: self.scalar_v1136,
            scalar_v1138: self.scalar_v1138,
            scalar_v1139: self.scalar_v1139,
            scalar_v1140: self.scalar_v1140,
            scalar_v1141: self.scalar_v1141,
            scalar_v1146: self.scalar_v1146,
            scalar_v1147: self.scalar_v1147,
            scalar_v1148: self.scalar_v1148,
            scalar_v1149: self.scalar_v1149,
            scalar_v1150: self.scalar_v1150,
            scalar_v1151: self.scalar_v1151,
            scalar_v1159: self.scalar_v1159,
            scalar_v1160: self.scalar_v1160,
            scalar_v1161: self.scalar_v1161,
            scalar_v1162: self.scalar_v1162,
            scalar_v1165: self.scalar_v1165,
            scalar_v1166: self.scalar_v1166,
            scalar_v1167: self.scalar_v1167,
            scalar_v1168: self.scalar_v1168,
            scalar_v1169: self.scalar_v1169,
            scalar_v1170: self.scalar_v1170,
            scalar_v1172: self.scalar_v1172,
            scalar_v1173: self.scalar_v1173,
            scalar_v1181: self.scalar_v1181,
            scalar_v1182: self.scalar_v1182,
            scalar_v1184: self.scalar_v1184,
            scalar_v1185: self.scalar_v1185,
            scalar_v1192: self.scalar_v1192,
            scalar_v1193: self.scalar_v1193,
            scalar_v1211: self.scalar_v1211,
            scalar_v1213: self.scalar_v1213,
            scalar_v1214: self.scalar_v1214,
            scalar_v1233: self.scalar_v1233,
            scalar_v1234: self.scalar_v1234,
            scalar_v1235: self.scalar_v1235,
            scalar_v1236: self.scalar_v1236,
            scalar_v1241: self.scalar_v1241,
            scalar_v1242: self.scalar_v1242,
            scalar_v1335: self.scalar_v1335,
            scalar_v1336: self.scalar_v1336,
            scalar_v1337: self.scalar_v1337,
            scalar_v1339: self.scalar_v1339,
            scalar_v1340: self.scalar_v1340,
            scalar_v1341: self.scalar_v1341,
            scalar_v1342: self.scalar_v1342,
            scalar_v1343: self.scalar_v1343,
            scalar_v1344: self.scalar_v1344,
            scalar_v1346: self.scalar_v1346,
            scalar_v1347: self.scalar_v1347,
            scalar_v1349: self.scalar_v1349,
            scalar_v1391: self.scalar_v1391,
            scalar_v1392: self.scalar_v1392,
            scalar_v1394: self.scalar_v1394,
            scalar_v1395: self.scalar_v1395,
            scalar_v1396: self.scalar_v1396,
            scalar_v1397: self.scalar_v1397,
            scalar_v1398: self.scalar_v1398,
            scalar_v1399: self.scalar_v1399,
            scalar_v1400: self.scalar_v1400,
            scalar_v1401: self.scalar_v1401,
            scalar_v1402: self.scalar_v1402,
            scalar_v1403: self.scalar_v1403,
            scalar_v1404: self.scalar_v1404,
            scalar_v1406: self.scalar_v1406,
            scalar_v1407: self.scalar_v1407,
            scalar_v1408: self.scalar_v1408,
            scalar_v1409: self.scalar_v1409,
            scalar_v1414: self.scalar_v1414,
            scalar_v1415: self.scalar_v1415,
            scalar_v1416: self.scalar_v1416,
            scalar_v1417: self.scalar_v1417,
            scalar_v1418: self.scalar_v1418,
            scalar_v1419: self.scalar_v1419,
            scalar_v1433: self.scalar_v1433,
            scalar_v1487: self.scalar_v1487,
            scalar_v1488: self.scalar_v1488,
            scalar_v1499: self.scalar_v1499,
            scalar_v1500: self.scalar_v1500,
            scalar_v1501: self.scalar_v1501,
            scalar_v1525: self.scalar_v1525,
            scalar_v1526: self.scalar_v1526,
            scalar_v1527: self.scalar_v1527,
            scalar_v1528: self.scalar_v1528,
            scalar_v1529: self.scalar_v1529,
            scalar_v1530: self.scalar_v1530,
            scalar_v1531: self.scalar_v1531,
            scalar_v1532: self.scalar_v1532,
            scalar_v1533: self.scalar_v1533,
            scalar_v1534: self.scalar_v1534,
            scalar_v1535: self.scalar_v1535,
            scalar_v1536: self.scalar_v1536,
            scalar_v1537: self.scalar_v1537,
            scalar_v1538: self.scalar_v1538,
            scalar_v1539: self.scalar_v1539,
            scalar_v1540: self.scalar_v1540,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1694: self.scalar_v1694,
            scalar_v1696: self.scalar_v1696,
            scalar_v1698: self.scalar_v1698,
            scalar_v1740: self.scalar_v1740,
            scalar_v1741: self.scalar_v1741,
            scalar_v1742: self.scalar_v1742,
            scalar_v1743: self.scalar_v1743,
            scalar_v1744: self.scalar_v1744,
            scalar_v1745: self.scalar_v1745,
            scalar_v1746: self.scalar_v1746,
            scalar_v1748: self.scalar_v1748,
            scalar_v1749: self.scalar_v1749,
            scalar_v1750: self.scalar_v1750,
            scalar_v1755: self.scalar_v1755,
            scalar_v1756: self.scalar_v1756,
            scalar_v1757: self.scalar_v1757,
            scalar_v1758: self.scalar_v1758,
            scalar_v1759: self.scalar_v1759,
            scalar_v1917: self.scalar_v1917,
            scalar_v1918: self.scalar_v1918,
            scalar_v1919: self.scalar_v1919,
            scalar_v1921: self.scalar_v1921,
            scalar_v1923: self.scalar_v1923,
            scalar_v1965: self.scalar_v1965,
            scalar_v1966: self.scalar_v1966,
            scalar_v1967: self.scalar_v1967,
            scalar_v1968: self.scalar_v1968,
            scalar_v1969: self.scalar_v1969,
            scalar_v1970: self.scalar_v1970,
            scalar_v1971: self.scalar_v1971,
            scalar_v1973: self.scalar_v1973,
            scalar_v1974: self.scalar_v1974,
            scalar_v1975: self.scalar_v1975,
            scalar_v1980: self.scalar_v1980,
            scalar_v1981: self.scalar_v1981,
            scalar_v1982: self.scalar_v1982,
            scalar_v1983: self.scalar_v1983,
            scalar_v1984: self.scalar_v1984,
            scalar_v2142: self.scalar_v2142,
            scalar_v2143: self.scalar_v2143,
            scalar_v2144: self.scalar_v2144,
            scalar_v2146: self.scalar_v2146,
            scalar_v2148: self.scalar_v2148,
            scalar_v2190: self.scalar_v2190,
            scalar_v2191: self.scalar_v2191,
            scalar_v2192: self.scalar_v2192,
            scalar_v2193: self.scalar_v2193,
            scalar_v2194: self.scalar_v2194,
            scalar_v2195: self.scalar_v2195,
            scalar_v2196: self.scalar_v2196,
            scalar_v2198: self.scalar_v2198,
            scalar_v2199: self.scalar_v2199,
            scalar_v2200: self.scalar_v2200,
            scalar_v2205: self.scalar_v2205,
            scalar_v2206: self.scalar_v2206,
            scalar_v2207: self.scalar_v2207,
            scalar_v2208: self.scalar_v2208,
            scalar_v2209: self.scalar_v2209,
            scalar_v2223: self.scalar_v2223,
            scalar_v2277: self.scalar_v2277,
            scalar_v2278: self.scalar_v2278,
            scalar_v2289: self.scalar_v2289,
            scalar_v2290: self.scalar_v2290,
            scalar_v2291: self.scalar_v2291,
            scalar_v2315: self.scalar_v2315,
            scalar_v2316: self.scalar_v2316,
            scalar_v2317: self.scalar_v2317,
            scalar_v2318: self.scalar_v2318,
            scalar_v2319: self.scalar_v2319,
            scalar_v2320: self.scalar_v2320,
            scalar_v2321: self.scalar_v2321,
            scalar_v2322: self.scalar_v2322,
            scalar_v2323: self.scalar_v2323,
            scalar_v2324: self.scalar_v2324,
            scalar_v2325: self.scalar_v2325,
            scalar_v2326: self.scalar_v2326,
            scalar_v2327: self.scalar_v2327,
            scalar_v2328: self.scalar_v2328,
            scalar_v2329: self.scalar_v2329,
            scalar_v2330: self.scalar_v2330,
            scalar_v2482: self.scalar_v2482,
            scalar_v2483: self.scalar_v2483,
            scalar_v2484: self.scalar_v2484,
            scalar_v2486: self.scalar_v2486,
            scalar_v2488: self.scalar_v2488,
            scalar_v2530: self.scalar_v2530,
            scalar_v2531: self.scalar_v2531,
            scalar_v2532: self.scalar_v2532,
            scalar_v2533: self.scalar_v2533,
            scalar_v2534: self.scalar_v2534,
            scalar_v2535: self.scalar_v2535,
            scalar_v2536: self.scalar_v2536,
            scalar_v2538: self.scalar_v2538,
            scalar_v2539: self.scalar_v2539,
            scalar_v2540: self.scalar_v2540,
            scalar_v2545: self.scalar_v2545,
            scalar_v2546: self.scalar_v2546,
            scalar_v2547: self.scalar_v2547,
            scalar_v2548: self.scalar_v2548,
            scalar_v2549: self.scalar_v2549,
            scalar_v2707: self.scalar_v2707,
            scalar_v2708: self.scalar_v2708,
            scalar_v2709: self.scalar_v2709,
            scalar_v2711: self.scalar_v2711,
            scalar_v2713: self.scalar_v2713,
            scalar_v2755: self.scalar_v2755,
            scalar_v2756: self.scalar_v2756,
            scalar_v2757: self.scalar_v2757,
            scalar_v2758: self.scalar_v2758,
            scalar_v2759: self.scalar_v2759,
            scalar_v2760: self.scalar_v2760,
            scalar_v2761: self.scalar_v2761,
            scalar_v2763: self.scalar_v2763,
            scalar_v2764: self.scalar_v2764,
            scalar_v2765: self.scalar_v2765,
            scalar_v2770: self.scalar_v2770,
            scalar_v2771: self.scalar_v2771,
            scalar_v2772: self.scalar_v2772,
            scalar_v2773: self.scalar_v2773,
            scalar_v2774: self.scalar_v2774,
            scalar_v2932: self.scalar_v2932,
            scalar_v2933: self.scalar_v2933,
            scalar_v2934: self.scalar_v2934,
            scalar_v2936: self.scalar_v2936,
            scalar_v2938: self.scalar_v2938,
            scalar_v2980: self.scalar_v2980,
            scalar_v2981: self.scalar_v2981,
            scalar_v2982: self.scalar_v2982,
            scalar_v2983: self.scalar_v2983,
            scalar_v2984: self.scalar_v2984,
            scalar_v2985: self.scalar_v2985,
            scalar_v2986: self.scalar_v2986,
            scalar_v2988: self.scalar_v2988,
            scalar_v2989: self.scalar_v2989,
            scalar_v2990: self.scalar_v2990,
            scalar_v2995: self.scalar_v2995,
            scalar_v2996: self.scalar_v2996,
            scalar_v2997: self.scalar_v2997,
            scalar_v2998: self.scalar_v2998,
            scalar_v2999: self.scalar_v2999,
            scalar_v3013: self.scalar_v3013,
            scalar_v3067: self.scalar_v3067,
            scalar_v3068: self.scalar_v3068,
            scalar_v3079: self.scalar_v3079,
            scalar_v3080: self.scalar_v3080,
            scalar_v3081: self.scalar_v3081,
            scalar_v3105: self.scalar_v3105,
            scalar_v3106: self.scalar_v3106,
            scalar_v3107: self.scalar_v3107,
            scalar_v3108: self.scalar_v3108,
            scalar_v3109: self.scalar_v3109,
            scalar_v3110: self.scalar_v3110,
            scalar_v3111: self.scalar_v3111,
            scalar_v3112: self.scalar_v3112,
            scalar_v3113: self.scalar_v3113,
            scalar_v3114: self.scalar_v3114,
            scalar_v3115: self.scalar_v3115,
            scalar_v3116: self.scalar_v3116,
            scalar_v3117: self.scalar_v3117,
            scalar_v3118: self.scalar_v3118,
            scalar_v3119: self.scalar_v3119,
            scalar_v3120: self.scalar_v3120,
            scalar_v3272: self.scalar_v3272,
            scalar_v3273: self.scalar_v3273,
            scalar_v3274: self.scalar_v3274,
            scalar_v3276: self.scalar_v3276,
            scalar_v3278: self.scalar_v3278,
            scalar_v3320: self.scalar_v3320,
            scalar_v3321: self.scalar_v3321,
            scalar_v3322: self.scalar_v3322,
            scalar_v3323: self.scalar_v3323,
            scalar_v3324: self.scalar_v3324,
            scalar_v3325: self.scalar_v3325,
            scalar_v3326: self.scalar_v3326,
            scalar_v3328: self.scalar_v3328,
            scalar_v3329: self.scalar_v3329,
            scalar_v3330: self.scalar_v3330,
            scalar_v3335: self.scalar_v3335,
            scalar_v3336: self.scalar_v3336,
            scalar_v3337: self.scalar_v3337,
            scalar_v3338: self.scalar_v3338,
            scalar_v3339: self.scalar_v3339,
            scalar_v3497: self.scalar_v3497,
            scalar_v3498: self.scalar_v3498,
            scalar_v3499: self.scalar_v3499,
            scalar_v3501: self.scalar_v3501,
            scalar_v3503: self.scalar_v3503,
            scalar_v3545: self.scalar_v3545,
            scalar_v3546: self.scalar_v3546,
            scalar_v3547: self.scalar_v3547,
            scalar_v3548: self.scalar_v3548,
            scalar_v3549: self.scalar_v3549,
            scalar_v3550: self.scalar_v3550,
            scalar_v3551: self.scalar_v3551,
            scalar_v3553: self.scalar_v3553,
            scalar_v3554: self.scalar_v3554,
            scalar_v3555: self.scalar_v3555,
            scalar_v3560: self.scalar_v3560,
            scalar_v3561: self.scalar_v3561,
            scalar_v3562: self.scalar_v3562,
            scalar_v3563: self.scalar_v3563,
            scalar_v3564: self.scalar_v3564,
            scalar_v3722: self.scalar_v3722,
            scalar_v3723: self.scalar_v3723,
            scalar_v3724: self.scalar_v3724,
            scalar_v3726: self.scalar_v3726,
            scalar_v3728: self.scalar_v3728,
            scalar_v3770: self.scalar_v3770,
            scalar_v3771: self.scalar_v3771,
            scalar_v3772: self.scalar_v3772,
            scalar_v3773: self.scalar_v3773,
            scalar_v3774: self.scalar_v3774,
            scalar_v3775: self.scalar_v3775,
            scalar_v3776: self.scalar_v3776,
            scalar_v3778: self.scalar_v3778,
            scalar_v3779: self.scalar_v3779,
            scalar_v3780: self.scalar_v3780,
            scalar_v3785: self.scalar_v3785,
            scalar_v3786: self.scalar_v3786,
            scalar_v3787: self.scalar_v3787,
            scalar_v3788: self.scalar_v3788,
            scalar_v3789: self.scalar_v3789,
            scalar_v3803: self.scalar_v3803,
            scalar_v3857: self.scalar_v3857,
            scalar_v3858: self.scalar_v3858,
            scalar_v3869: self.scalar_v3869,
            scalar_v3870: self.scalar_v3870,
            scalar_v3871: self.scalar_v3871,
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
            scalar_v4062: self.scalar_v4062,
            scalar_v4063: self.scalar_v4063,
            scalar_v4064: self.scalar_v4064,
            scalar_v4066: self.scalar_v4066,
            scalar_v4068: self.scalar_v4068,
            scalar_v4110: self.scalar_v4110,
            scalar_v4111: self.scalar_v4111,
            scalar_v4112: self.scalar_v4112,
            scalar_v4113: self.scalar_v4113,
            scalar_v4114: self.scalar_v4114,
            scalar_v4115: self.scalar_v4115,
            scalar_v4116: self.scalar_v4116,
            scalar_v4118: self.scalar_v4118,
            scalar_v4119: self.scalar_v4119,
            scalar_v4120: self.scalar_v4120,
            scalar_v4125: self.scalar_v4125,
            scalar_v4126: self.scalar_v4126,
            scalar_v4127: self.scalar_v4127,
            scalar_v4128: self.scalar_v4128,
            scalar_v4129: self.scalar_v4129,
            scalar_v4287: self.scalar_v4287,
            scalar_v4288: self.scalar_v4288,
            scalar_v4289: self.scalar_v4289,
            scalar_v4291: self.scalar_v4291,
            scalar_v4293: self.scalar_v4293,
            scalar_v4335: self.scalar_v4335,
            scalar_v4336: self.scalar_v4336,
            scalar_v4337: self.scalar_v4337,
            scalar_v4338: self.scalar_v4338,
            scalar_v4339: self.scalar_v4339,
            scalar_v4340: self.scalar_v4340,
            scalar_v4341: self.scalar_v4341,
            scalar_v4343: self.scalar_v4343,
            scalar_v4344: self.scalar_v4344,
            scalar_v4345: self.scalar_v4345,
            scalar_v4350: self.scalar_v4350,
            scalar_v4351: self.scalar_v4351,
            scalar_v4352: self.scalar_v4352,
            scalar_v4353: self.scalar_v4353,
            scalar_v4354: self.scalar_v4354,
            scalar_v4512: self.scalar_v4512,
            scalar_v4513: self.scalar_v4513,
            scalar_v4514: self.scalar_v4514,
            scalar_v4516: self.scalar_v4516,
            scalar_v4518: self.scalar_v4518,
            scalar_v4560: self.scalar_v4560,
            scalar_v4561: self.scalar_v4561,
            scalar_v4562: self.scalar_v4562,
            scalar_v4563: self.scalar_v4563,
            scalar_v4564: self.scalar_v4564,
            scalar_v4565: self.scalar_v4565,
            scalar_v4566: self.scalar_v4566,
            scalar_v4568: self.scalar_v4568,
            scalar_v4569: self.scalar_v4569,
            scalar_v4570: self.scalar_v4570,
            scalar_v4575: self.scalar_v4575,
            scalar_v4576: self.scalar_v4576,
            scalar_v4577: self.scalar_v4577,
            scalar_v4578: self.scalar_v4578,
            scalar_v4579: self.scalar_v4579,
            scalar_v4630: self.scalar_v4630,
            scalar_v4694: self.scalar_v4694,
            scalar_v4697: self.scalar_v4697,
            scalar_v4698: self.scalar_v4698,
            scalar_v4706: self.scalar_v4706,
            scalar_v4723: self.scalar_v4723,
            scalar_v4744: self.scalar_v4744,
            scalar_v4761: self.scalar_v4761,
            scalar_v4791: self.scalar_v4791,
            scalar_v4880: self.scalar_v4880,
            scalar_v4887: self.scalar_v4887,
            scalar_v4898: self.scalar_v4898,
            scalar_v4905: self.scalar_v4905,
            scalar_v4915: self.scalar_v4915,
            scalar_v4922: self.scalar_v4922,
            scalar_v4931: self.scalar_v4931,
            scalar_v4932: self.scalar_v4932,
            scalar_v4933: self.scalar_v4933,
            scalar_v5055: self.scalar_v5055,
            scalar_v5056: self.scalar_v5056,
            scalar_v5058: self.scalar_v5058,
            scalar_v5061: self.scalar_v5061,
            scalar_v5062: self.scalar_v5062,
            scalar_v5063: self.scalar_v5063,
            scalar_v5071: self.scalar_v5071,
            scalar_v5072: self.scalar_v5072,
            scalar_v5073: self.scalar_v5073,
            scalar_v5098: self.scalar_v5098,
            scalar_v5099: self.scalar_v5099,
            scalar_v5100: self.scalar_v5100,
            scalar_v5118: self.scalar_v5118,
            scalar_v5123: self.scalar_v5123,
            scalar_v5216: self.scalar_v5216,
            scalar_v5217: self.scalar_v5217,
            scalar_v5218: self.scalar_v5218,
            scalar_v5219: self.scalar_v5219,
            scalar_v5224: self.scalar_v5224,
            scalar_v5268: self.scalar_v5268,
            scalar_v5269: self.scalar_v5269,
            scalar_v5271: self.scalar_v5271,
            scalar_v5298: self.scalar_v5298,
            scalar_v5303: self.scalar_v5303,
            scalar_v5306: self.scalar_v5306,
            scalar_v5314: self.scalar_v5314,
            scalar_v5315: self.scalar_v5315,
            scalar_v5317: self.scalar_v5317,
            scalar_v5320: self.scalar_v5320,
            scalar_v5322: self.scalar_v5322,
            scalar_v5330: self.scalar_v5330,
            scalar_v5332: self.scalar_v5332,
            scalar_v5356: self.scalar_v5356,
            scalar_v5358: self.scalar_v5358,
            scalar_v5376: self.scalar_v5376,
            scalar_v5381: self.scalar_v5381,
            scalar_v5474: self.scalar_v5474,
            scalar_v5476: self.scalar_v5476,
            scalar_v5477: self.scalar_v5477,
            scalar_v5482: self.scalar_v5482,
            scalar_v5526: self.scalar_v5526,
            scalar_v5529: self.scalar_v5529,
            scalar_v5556: self.scalar_v5556,
            scalar_v5561: self.scalar_v5561,
            scalar_v5570: self.scalar_v5570,
            scalar_v5571: self.scalar_v5571,
            scalar_v5573: self.scalar_v5573,
            scalar_v5576: self.scalar_v5576,
            scalar_v5578: self.scalar_v5578,
            scalar_v5586: self.scalar_v5586,
            scalar_v5588: self.scalar_v5588,
            scalar_v5612: self.scalar_v5612,
            scalar_v5614: self.scalar_v5614,
            scalar_v5632: self.scalar_v5632,
            scalar_v5637: self.scalar_v5637,
            scalar_v5730: self.scalar_v5730,
            scalar_v5732: self.scalar_v5732,
            scalar_v5733: self.scalar_v5733,
            scalar_v5738: self.scalar_v5738,
            scalar_v5782: self.scalar_v5782,
            scalar_v5785: self.scalar_v5785,
            scalar_v5812: self.scalar_v5812,
            scalar_v5813: self.scalar_v5813,
            scalar_v5816: self.scalar_v5816,
            scalar_v5856: self.scalar_v5856,
            scalar_v5861: self.scalar_v5861,
            scalar_v5892: self.scalar_v5892,
            scalar_v5893: self.scalar_v5893,
            scalar_v5898: self.scalar_v5898,
            scalar_v5899: self.scalar_v5899,
            scalar_v5911: self.scalar_v5911,
            scalar_v5912: self.scalar_v5912,
            scalar_v5913: self.scalar_v5913,
            scalar_v5918: self.scalar_v5918,
            scalar_v5938: self.scalar_v5938,
            scalar_v5939: self.scalar_v5939,
            scalar_v5940: self.scalar_v5940,
            scalar_v5941: self.scalar_v5941,
            scalar_v5942: self.scalar_v5942,
            scalar_v5945: self.scalar_v5945,
            scalar_v6010: self.scalar_v6010,
            scalar_v6011: self.scalar_v6011,
            scalar_v6047: self.scalar_v6047,
            scalar_v6048: self.scalar_v6048,
            scalar_v6049: self.scalar_v6049,
            scalar_v6050: self.scalar_v6050,
            scalar_v6051: self.scalar_v6051,
            scalar_v6052: self.scalar_v6052,
            scalar_v6053: self.scalar_v6053,
            scalar_v6054: self.scalar_v6054,
            scalar_v6092: self.scalar_v6092,
            scalar_v6120: self.scalar_v6120,
            scalar_v6146: self.scalar_v6146,
            scalar_v6163: self.scalar_v6163,
            scalar_v6164: self.scalar_v6164,
            scalar_v6165: self.scalar_v6165,
            scalar_v6166: self.scalar_v6166,
            scalar_v6167: self.scalar_v6167,
            scalar_v6168: self.scalar_v6168,
            scalar_v6169: self.scalar_v6169,
            scalar_v6170: self.scalar_v6170,
            scalar_v6450: self.scalar_v6450,
            scalar_v6558: self.scalar_v6558,
            scalar_v7061: self.scalar_v7061,
            scalar_v7171: self.scalar_v7171,
            scalar_v7674: self.scalar_v7674,
            scalar_v7784: self.scalar_v7784,
            scalar_v8188: self.scalar_v8188,
            scalar_v8189: self.scalar_v8189,
            scalar_v8198: self.scalar_v8198,
            scalar_v8199: self.scalar_v8199,
            scalar_v8328: self.scalar_v8328,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 2;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 2;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 65;
    pub const VARIABLE_COUNT: usize = 668;
    pub const DDT_STATE_COUNT: usize = 1;
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
            scalar_v2: 0.0,
            scalar_v4: false,
            scalar_v6: 0.0,
            scalar_v7: false,
            scalar_v8: 0.0,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v21: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
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
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
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
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: false,
            scalar_v107: 0.0,
            scalar_v108: false,
            scalar_v109: false,
            scalar_v110: 0.0,
            scalar_v111: false,
            scalar_v112: false,
            scalar_v113: 0.0,
            scalar_v114: false,
            scalar_v115: false,
            scalar_v116: 0.0,
            scalar_v117: false,
            scalar_v118: 0.0,
            scalar_v119: false,
            scalar_v120: 0.0,
            scalar_v122: false,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v127: false,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: false,
            scalar_v132: 0.0,
            scalar_v134: false,
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
            scalar_v146: 0.0,
            scalar_v148: 0.0,
            scalar_v166: 0.0,
            scalar_v172: 0.0,
            scalar_v178: 0.0,
            scalar_v184: 0.0,
            scalar_v187: 0.0,
            scalar_v190: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v314: 0.0,
            scalar_v345: 0.0,
            scalar_v346: false,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: false,
            scalar_v350: 0.0,
            scalar_v351: 0.0,
            scalar_v352: false,
            scalar_v353: 0.0,
            scalar_v354: 0.0,
            scalar_v355: false,
            scalar_v356: 0.0,
            scalar_v359: 0.0,
            scalar_v425: false,
            scalar_v429: false,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v434: 0.0,
            scalar_v435: false,
            scalar_v438: false,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v441: 0.0,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v444: false,
            scalar_v447: false,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v459: false,
            scalar_v460: 0.0,
            scalar_v461: false,
            scalar_v462: 0.0,
            scalar_v464: 0.0,
            scalar_v465: 0.0,
            scalar_v466: 0.0,
            scalar_v468: false,
            scalar_v469: 0.0,
            scalar_v470: false,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v473: false,
            scalar_v474: 0.0,
            scalar_v476: 0.0,
            scalar_v478: 0.0,
            scalar_v480: 0.0,
            scalar_v481: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v484: 0.0,
            scalar_v485: 0.0,
            scalar_v486: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v491: 0.0,
            scalar_v493: 0.0,
            scalar_v494: 0.0,
            scalar_v495: false,
            scalar_v496: false,
            scalar_v497: false,
            scalar_v498: false,
            scalar_v556: false,
            scalar_v557: false,
            scalar_v568: false,
            scalar_v569: false,
            scalar_v570: 0.0,
            scalar_v596: 0.0,
            scalar_v597: 0.0,
            scalar_v598: 0.0,
            scalar_v599: 0.0,
            scalar_v600: 0.0,
            scalar_v601: 0.0,
            scalar_v602: 0.0,
            scalar_v603: 0.0,
            scalar_v604: 0.0,
            scalar_v605: 0.0,
            scalar_v606: 0.0,
            scalar_v607: 0.0,
            scalar_v608: 0.0,
            scalar_v611: 0.0,
            scalar_v612: 0.0,
            scalar_v613: 0.0,
            scalar_v614: 0.0,
            scalar_v615: 0.0,
            scalar_v616: false,
            scalar_v617: 0.0,
            scalar_v618: false,
            scalar_v619: false,
            scalar_v622: 0.0,
            scalar_v623: false,
            scalar_v624: 0.0,
            scalar_v625: false,
            scalar_v626: false,
            scalar_v627: false,
            scalar_v628: 0.0,
            scalar_v629: false,
            scalar_v630: false,
            scalar_v638: false,
            scalar_v639: false,
            scalar_v640: 0.0,
            scalar_v641: false,
            scalar_v642: false,
            scalar_v649: 0.0,
            scalar_v650: 0.0,
            scalar_v669: false,
            scalar_v670: 0.0,
            scalar_v671: false,
            scalar_v672: false,
            scalar_v692: 0.0,
            scalar_v693: 0.0,
            scalar_v694: false,
            scalar_v695: false,
            scalar_v700: false,
            scalar_v701: false,
            scalar_v796: 0.0,
            scalar_v797: false,
            scalar_v798: false,
            scalar_v799: 0.0,
            scalar_v800: false,
            scalar_v801: false,
            scalar_v802: false,
            scalar_v803: 0.0,
            scalar_v804: 0.0,
            scalar_v805: 0.0,
            scalar_v807: false,
            scalar_v808: 0.0,
            scalar_v810: 0.0,
            scalar_v853: false,
            scalar_v854: false,
            scalar_v855: 0.0,
            scalar_v856: 0.0,
            scalar_v857: 0.0,
            scalar_v858: false,
            scalar_v859: false,
            scalar_v860: false,
            scalar_v861: false,
            scalar_v862: false,
            scalar_v863: false,
            scalar_v864: 0.0,
            scalar_v865: 0.0,
            scalar_v866: 0.0,
            scalar_v867: 0.0,
            scalar_v869: false,
            scalar_v870: false,
            scalar_v871: 0.0,
            scalar_v872: 0.0,
            scalar_v877: false,
            scalar_v878: false,
            scalar_v879: 0.0,
            scalar_v880: 0.0,
            scalar_v881: 0.0,
            scalar_v882: 0.0,
            scalar_v884: 0.0,
            scalar_v891: false,
            scalar_v892: 0.0,
            scalar_v893: false,
            scalar_v894: false,
            scalar_v897: 0.0,
            scalar_v898: false,
            scalar_v899: 0.0,
            scalar_v900: false,
            scalar_v901: false,
            scalar_v902: false,
            scalar_v904: false,
            scalar_v905: false,
            scalar_v913: false,
            scalar_v914: false,
            scalar_v916: false,
            scalar_v917: false,
            scalar_v924: 0.0,
            scalar_v925: 0.0,
            scalar_v943: false,
            scalar_v945: false,
            scalar_v946: false,
            scalar_v965: 0.0,
            scalar_v966: 0.0,
            scalar_v967: false,
            scalar_v968: false,
            scalar_v973: false,
            scalar_v974: false,
            scalar_v1067: 0.0,
            scalar_v1068: false,
            scalar_v1069: false,
            scalar_v1071: false,
            scalar_v1072: false,
            scalar_v1073: false,
            scalar_v1074: 0.0,
            scalar_v1075: 0.0,
            scalar_v1076: 0.0,
            scalar_v1078: false,
            scalar_v1079: 0.0,
            scalar_v1081: 0.0,
            scalar_v1123: false,
            scalar_v1124: false,
            scalar_v1126: 0.0,
            scalar_v1127: false,
            scalar_v1128: false,
            scalar_v1129: false,
            scalar_v1130: false,
            scalar_v1131: false,
            scalar_v1132: false,
            scalar_v1133: 0.0,
            scalar_v1134: 0.0,
            scalar_v1135: 0.0,
            scalar_v1136: 0.0,
            scalar_v1138: false,
            scalar_v1139: false,
            scalar_v1140: 0.0,
            scalar_v1141: 0.0,
            scalar_v1146: false,
            scalar_v1147: false,
            scalar_v1148: 0.0,
            scalar_v1149: 0.0,
            scalar_v1150: 0.0,
            scalar_v1151: 0.0,
            scalar_v1159: false,
            scalar_v1160: 0.0,
            scalar_v1161: false,
            scalar_v1162: false,
            scalar_v1165: 0.0,
            scalar_v1166: false,
            scalar_v1167: 0.0,
            scalar_v1168: false,
            scalar_v1169: false,
            scalar_v1170: false,
            scalar_v1172: false,
            scalar_v1173: false,
            scalar_v1181: false,
            scalar_v1182: false,
            scalar_v1184: false,
            scalar_v1185: false,
            scalar_v1192: 0.0,
            scalar_v1193: 0.0,
            scalar_v1211: false,
            scalar_v1213: false,
            scalar_v1214: false,
            scalar_v1233: 0.0,
            scalar_v1234: 0.0,
            scalar_v1235: false,
            scalar_v1236: false,
            scalar_v1241: false,
            scalar_v1242: false,
            scalar_v1335: 0.0,
            scalar_v1336: false,
            scalar_v1337: false,
            scalar_v1339: false,
            scalar_v1340: false,
            scalar_v1341: false,
            scalar_v1342: 0.0,
            scalar_v1343: 0.0,
            scalar_v1344: 0.0,
            scalar_v1346: false,
            scalar_v1347: 0.0,
            scalar_v1349: 0.0,
            scalar_v1391: false,
            scalar_v1392: false,
            scalar_v1394: 0.0,
            scalar_v1395: false,
            scalar_v1396: false,
            scalar_v1397: false,
            scalar_v1398: false,
            scalar_v1399: false,
            scalar_v1400: false,
            scalar_v1401: 0.0,
            scalar_v1402: 0.0,
            scalar_v1403: 0.0,
            scalar_v1404: 0.0,
            scalar_v1406: false,
            scalar_v1407: false,
            scalar_v1408: 0.0,
            scalar_v1409: 0.0,
            scalar_v1414: false,
            scalar_v1415: false,
            scalar_v1416: 0.0,
            scalar_v1417: 0.0,
            scalar_v1418: 0.0,
            scalar_v1419: 0.0,
            scalar_v1433: 0.0,
            scalar_v1487: false,
            scalar_v1488: false,
            scalar_v1499: false,
            scalar_v1500: false,
            scalar_v1501: 0.0,
            scalar_v1525: 0.0,
            scalar_v1526: 0.0,
            scalar_v1527: 0.0,
            scalar_v1528: 0.0,
            scalar_v1529: 0.0,
            scalar_v1530: 0.0,
            scalar_v1531: 0.0,
            scalar_v1532: 0.0,
            scalar_v1533: 0.0,
            scalar_v1534: 0.0,
            scalar_v1535: 0.0,
            scalar_v1536: 0.0,
            scalar_v1537: 0.0,
            scalar_v1538: 0.0,
            scalar_v1539: 0.0,
            scalar_v1540: 0.0,
            scalar_v1692: 0.0,
            scalar_v1693: 0.0,
            scalar_v1694: 0.0,
            scalar_v1696: 0.0,
            scalar_v1698: 0.0,
            scalar_v1740: false,
            scalar_v1741: false,
            scalar_v1742: false,
            scalar_v1743: 0.0,
            scalar_v1744: 0.0,
            scalar_v1745: 0.0,
            scalar_v1746: 0.0,
            scalar_v1748: false,
            scalar_v1749: 0.0,
            scalar_v1750: 0.0,
            scalar_v1755: false,
            scalar_v1756: false,
            scalar_v1757: 0.0,
            scalar_v1758: 0.0,
            scalar_v1759: 0.0,
            scalar_v1917: 0.0,
            scalar_v1918: 0.0,
            scalar_v1919: 0.0,
            scalar_v1921: 0.0,
            scalar_v1923: 0.0,
            scalar_v1965: false,
            scalar_v1966: false,
            scalar_v1967: false,
            scalar_v1968: 0.0,
            scalar_v1969: 0.0,
            scalar_v1970: 0.0,
            scalar_v1971: 0.0,
            scalar_v1973: false,
            scalar_v1974: 0.0,
            scalar_v1975: 0.0,
            scalar_v1980: false,
            scalar_v1981: false,
            scalar_v1982: 0.0,
            scalar_v1983: 0.0,
            scalar_v1984: 0.0,
            scalar_v2142: 0.0,
            scalar_v2143: 0.0,
            scalar_v2144: 0.0,
            scalar_v2146: 0.0,
            scalar_v2148: 0.0,
            scalar_v2190: false,
            scalar_v2191: false,
            scalar_v2192: false,
            scalar_v2193: 0.0,
            scalar_v2194: 0.0,
            scalar_v2195: 0.0,
            scalar_v2196: 0.0,
            scalar_v2198: false,
            scalar_v2199: 0.0,
            scalar_v2200: 0.0,
            scalar_v2205: false,
            scalar_v2206: false,
            scalar_v2207: 0.0,
            scalar_v2208: 0.0,
            scalar_v2209: 0.0,
            scalar_v2223: 0.0,
            scalar_v2277: false,
            scalar_v2278: false,
            scalar_v2289: false,
            scalar_v2290: false,
            scalar_v2291: 0.0,
            scalar_v2315: 0.0,
            scalar_v2316: 0.0,
            scalar_v2317: 0.0,
            scalar_v2318: 0.0,
            scalar_v2319: 0.0,
            scalar_v2320: 0.0,
            scalar_v2321: 0.0,
            scalar_v2322: 0.0,
            scalar_v2323: 0.0,
            scalar_v2324: 0.0,
            scalar_v2325: 0.0,
            scalar_v2326: 0.0,
            scalar_v2327: 0.0,
            scalar_v2328: 0.0,
            scalar_v2329: 0.0,
            scalar_v2330: 0.0,
            scalar_v2482: 0.0,
            scalar_v2483: 0.0,
            scalar_v2484: 0.0,
            scalar_v2486: 0.0,
            scalar_v2488: 0.0,
            scalar_v2530: false,
            scalar_v2531: false,
            scalar_v2532: false,
            scalar_v2533: 0.0,
            scalar_v2534: 0.0,
            scalar_v2535: 0.0,
            scalar_v2536: 0.0,
            scalar_v2538: false,
            scalar_v2539: 0.0,
            scalar_v2540: 0.0,
            scalar_v2545: false,
            scalar_v2546: false,
            scalar_v2547: 0.0,
            scalar_v2548: 0.0,
            scalar_v2549: 0.0,
            scalar_v2707: 0.0,
            scalar_v2708: 0.0,
            scalar_v2709: 0.0,
            scalar_v2711: 0.0,
            scalar_v2713: 0.0,
            scalar_v2755: false,
            scalar_v2756: false,
            scalar_v2757: false,
            scalar_v2758: 0.0,
            scalar_v2759: 0.0,
            scalar_v2760: 0.0,
            scalar_v2761: 0.0,
            scalar_v2763: false,
            scalar_v2764: 0.0,
            scalar_v2765: 0.0,
            scalar_v2770: false,
            scalar_v2771: false,
            scalar_v2772: 0.0,
            scalar_v2773: 0.0,
            scalar_v2774: 0.0,
            scalar_v2932: 0.0,
            scalar_v2933: 0.0,
            scalar_v2934: 0.0,
            scalar_v2936: 0.0,
            scalar_v2938: 0.0,
            scalar_v2980: false,
            scalar_v2981: false,
            scalar_v2982: false,
            scalar_v2983: 0.0,
            scalar_v2984: 0.0,
            scalar_v2985: 0.0,
            scalar_v2986: 0.0,
            scalar_v2988: false,
            scalar_v2989: 0.0,
            scalar_v2990: 0.0,
            scalar_v2995: false,
            scalar_v2996: false,
            scalar_v2997: 0.0,
            scalar_v2998: 0.0,
            scalar_v2999: 0.0,
            scalar_v3013: 0.0,
            scalar_v3067: false,
            scalar_v3068: false,
            scalar_v3079: false,
            scalar_v3080: false,
            scalar_v3081: 0.0,
            scalar_v3105: 0.0,
            scalar_v3106: 0.0,
            scalar_v3107: 0.0,
            scalar_v3108: 0.0,
            scalar_v3109: 0.0,
            scalar_v3110: 0.0,
            scalar_v3111: 0.0,
            scalar_v3112: 0.0,
            scalar_v3113: 0.0,
            scalar_v3114: 0.0,
            scalar_v3115: 0.0,
            scalar_v3116: 0.0,
            scalar_v3117: 0.0,
            scalar_v3118: 0.0,
            scalar_v3119: 0.0,
            scalar_v3120: 0.0,
            scalar_v3272: 0.0,
            scalar_v3273: 0.0,
            scalar_v3274: 0.0,
            scalar_v3276: 0.0,
            scalar_v3278: 0.0,
            scalar_v3320: false,
            scalar_v3321: false,
            scalar_v3322: false,
            scalar_v3323: 0.0,
            scalar_v3324: 0.0,
            scalar_v3325: 0.0,
            scalar_v3326: 0.0,
            scalar_v3328: false,
            scalar_v3329: 0.0,
            scalar_v3330: 0.0,
            scalar_v3335: false,
            scalar_v3336: false,
            scalar_v3337: 0.0,
            scalar_v3338: 0.0,
            scalar_v3339: 0.0,
            scalar_v3497: 0.0,
            scalar_v3498: 0.0,
            scalar_v3499: 0.0,
            scalar_v3501: 0.0,
            scalar_v3503: 0.0,
            scalar_v3545: false,
            scalar_v3546: false,
            scalar_v3547: false,
            scalar_v3548: 0.0,
            scalar_v3549: 0.0,
            scalar_v3550: 0.0,
            scalar_v3551: 0.0,
            scalar_v3553: false,
            scalar_v3554: 0.0,
            scalar_v3555: 0.0,
            scalar_v3560: false,
            scalar_v3561: false,
            scalar_v3562: 0.0,
            scalar_v3563: 0.0,
            scalar_v3564: 0.0,
            scalar_v3722: 0.0,
            scalar_v3723: 0.0,
            scalar_v3724: 0.0,
            scalar_v3726: 0.0,
            scalar_v3728: 0.0,
            scalar_v3770: false,
            scalar_v3771: false,
            scalar_v3772: false,
            scalar_v3773: 0.0,
            scalar_v3774: 0.0,
            scalar_v3775: 0.0,
            scalar_v3776: 0.0,
            scalar_v3778: false,
            scalar_v3779: 0.0,
            scalar_v3780: 0.0,
            scalar_v3785: false,
            scalar_v3786: false,
            scalar_v3787: 0.0,
            scalar_v3788: 0.0,
            scalar_v3789: 0.0,
            scalar_v3803: 0.0,
            scalar_v3857: false,
            scalar_v3858: false,
            scalar_v3869: false,
            scalar_v3870: false,
            scalar_v3871: 0.0,
            scalar_v3895: 0.0,
            scalar_v3896: 0.0,
            scalar_v3897: 0.0,
            scalar_v3898: 0.0,
            scalar_v3899: 0.0,
            scalar_v3900: 0.0,
            scalar_v3901: 0.0,
            scalar_v3902: 0.0,
            scalar_v3903: 0.0,
            scalar_v3904: 0.0,
            scalar_v3905: 0.0,
            scalar_v3906: 0.0,
            scalar_v3907: 0.0,
            scalar_v3908: 0.0,
            scalar_v3909: 0.0,
            scalar_v3910: 0.0,
            scalar_v4062: 0.0,
            scalar_v4063: 0.0,
            scalar_v4064: 0.0,
            scalar_v4066: 0.0,
            scalar_v4068: 0.0,
            scalar_v4110: false,
            scalar_v4111: false,
            scalar_v4112: false,
            scalar_v4113: 0.0,
            scalar_v4114: 0.0,
            scalar_v4115: 0.0,
            scalar_v4116: 0.0,
            scalar_v4118: false,
            scalar_v4119: 0.0,
            scalar_v4120: 0.0,
            scalar_v4125: false,
            scalar_v4126: false,
            scalar_v4127: 0.0,
            scalar_v4128: 0.0,
            scalar_v4129: 0.0,
            scalar_v4287: 0.0,
            scalar_v4288: 0.0,
            scalar_v4289: 0.0,
            scalar_v4291: 0.0,
            scalar_v4293: 0.0,
            scalar_v4335: false,
            scalar_v4336: false,
            scalar_v4337: false,
            scalar_v4338: 0.0,
            scalar_v4339: 0.0,
            scalar_v4340: 0.0,
            scalar_v4341: 0.0,
            scalar_v4343: false,
            scalar_v4344: 0.0,
            scalar_v4345: 0.0,
            scalar_v4350: false,
            scalar_v4351: false,
            scalar_v4352: 0.0,
            scalar_v4353: 0.0,
            scalar_v4354: 0.0,
            scalar_v4512: 0.0,
            scalar_v4513: 0.0,
            scalar_v4514: 0.0,
            scalar_v4516: 0.0,
            scalar_v4518: 0.0,
            scalar_v4560: false,
            scalar_v4561: false,
            scalar_v4562: false,
            scalar_v4563: 0.0,
            scalar_v4564: 0.0,
            scalar_v4565: 0.0,
            scalar_v4566: 0.0,
            scalar_v4568: false,
            scalar_v4569: 0.0,
            scalar_v4570: 0.0,
            scalar_v4575: false,
            scalar_v4576: false,
            scalar_v4577: 0.0,
            scalar_v4578: 0.0,
            scalar_v4579: 0.0,
            scalar_v4630: 0.0,
            scalar_v4694: 0.0,
            scalar_v4697: 0.0,
            scalar_v4698: 0.0,
            scalar_v4706: 0.0,
            scalar_v4723: 0.0,
            scalar_v4744: 0.0,
            scalar_v4761: 0.0,
            scalar_v4791: 0.0,
            scalar_v4880: false,
            scalar_v4887: false,
            scalar_v4898: false,
            scalar_v4905: false,
            scalar_v4915: false,
            scalar_v4922: false,
            scalar_v4931: false,
            scalar_v4932: 0.0,
            scalar_v4933: false,
            scalar_v5055: false,
            scalar_v5056: 0.0,
            scalar_v5058: false,
            scalar_v5061: false,
            scalar_v5062: 0.0,
            scalar_v5063: false,
            scalar_v5071: false,
            scalar_v5072: 0.0,
            scalar_v5073: false,
            scalar_v5098: false,
            scalar_v5099: 0.0,
            scalar_v5100: false,
            scalar_v5118: false,
            scalar_v5123: false,
            scalar_v5216: false,
            scalar_v5217: 0.0,
            scalar_v5218: false,
            scalar_v5219: false,
            scalar_v5224: false,
            scalar_v5268: false,
            scalar_v5269: 0.0,
            scalar_v5271: false,
            scalar_v5298: false,
            scalar_v5303: false,
            scalar_v5306: 0.0,
            scalar_v5314: false,
            scalar_v5315: 0.0,
            scalar_v5317: false,
            scalar_v5320: false,
            scalar_v5322: false,
            scalar_v5330: false,
            scalar_v5332: false,
            scalar_v5356: false,
            scalar_v5358: false,
            scalar_v5376: false,
            scalar_v5381: false,
            scalar_v5474: false,
            scalar_v5476: false,
            scalar_v5477: false,
            scalar_v5482: false,
            scalar_v5526: false,
            scalar_v5529: false,
            scalar_v5556: false,
            scalar_v5561: false,
            scalar_v5570: false,
            scalar_v5571: 0.0,
            scalar_v5573: false,
            scalar_v5576: false,
            scalar_v5578: false,
            scalar_v5586: false,
            scalar_v5588: false,
            scalar_v5612: false,
            scalar_v5614: false,
            scalar_v5632: false,
            scalar_v5637: false,
            scalar_v5730: false,
            scalar_v5732: false,
            scalar_v5733: false,
            scalar_v5738: false,
            scalar_v5782: false,
            scalar_v5785: false,
            scalar_v5812: false,
            scalar_v5813: 0.0,
            scalar_v5816: 0.0,
            scalar_v5856: false,
            scalar_v5861: false,
            scalar_v5892: false,
            scalar_v5893: false,
            scalar_v5898: false,
            scalar_v5899: false,
            scalar_v5911: false,
            scalar_v5912: false,
            scalar_v5913: false,
            scalar_v5918: false,
            scalar_v5938: 0.0,
            scalar_v5939: 0.0,
            scalar_v5940: 0.0,
            scalar_v5941: 0.0,
            scalar_v5942: 0.0,
            scalar_v5945: 0.0,
            scalar_v6010: 0.0,
            scalar_v6011: 0.0,
            scalar_v6047: 0.0,
            scalar_v6048: 0.0,
            scalar_v6049: 0.0,
            scalar_v6050: 0.0,
            scalar_v6051: 0.0,
            scalar_v6052: 0.0,
            scalar_v6053: 0.0,
            scalar_v6054: 0.0,
            scalar_v6092: 0.0,
            scalar_v6120: 0.0,
            scalar_v6146: 0.0,
            scalar_v6163: 0.0,
            scalar_v6164: 0.0,
            scalar_v6165: 0.0,
            scalar_v6166: 0.0,
            scalar_v6167: 0.0,
            scalar_v6168: 0.0,
            scalar_v6169: 0.0,
            scalar_v6170: 0.0,
            scalar_v6450: 0.0,
            scalar_v6558: 0.0,
            scalar_v7061: 0.0,
            scalar_v7171: 0.0,
            scalar_v7674: 0.0,
            scalar_v7784: 0.0,
            scalar_v8188: 0.0,
            scalar_v8189: 0.0,
            scalar_v8198: 0.0,
            scalar_v8199: 0.0,
            scalar_v8328: 0.0,
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
            scalar_v2,
            scalar_v4,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v11,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v23,
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
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
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
            scalar_v146,
            scalar_v148,
            scalar_v166,
            scalar_v172,
            scalar_v178,
            scalar_v184,
            scalar_v187,
            scalar_v190,
            scalar_v261,
            scalar_v262,
            scalar_v264,
            scalar_v265,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v290,
            scalar_v291,
            scalar_v296,
            scalar_v297,
            scalar_v301,
            scalar_v302,
            scalar_v314,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v359,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v476,
            scalar_v478,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v556,
            scalar_v557,
            scalar_v568,
            scalar_v569,
            scalar_v570,
            scalar_v596,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v607,
            scalar_v608,
            scalar_v611,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v616,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v630,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v649,
            scalar_v650,
            scalar_v669,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v692,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v700,
            scalar_v701,
            scalar_v796,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v805,
            scalar_v807,
            scalar_v808,
            scalar_v810,
            scalar_v853,
            scalar_v854,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v860,
            scalar_v861,
            scalar_v862,
            scalar_v863,
            scalar_v864,
            scalar_v865,
            scalar_v866,
            scalar_v867,
            scalar_v869,
            scalar_v870,
            scalar_v871,
            scalar_v872,
            scalar_v877,
            scalar_v878,
            scalar_v879,
            scalar_v880,
            scalar_v881,
            scalar_v882,
            scalar_v884,
            scalar_v891,
            scalar_v892,
            scalar_v893,
            scalar_v894,
            scalar_v897,
            scalar_v898,
            scalar_v899,
            scalar_v900,
            scalar_v901,
            scalar_v902,
            scalar_v904,
            scalar_v905,
            scalar_v913,
            scalar_v914,
            scalar_v916,
            scalar_v917,
            scalar_v924,
            scalar_v925,
            scalar_v943,
            scalar_v945,
            scalar_v946,
            scalar_v965,
            scalar_v966,
            scalar_v967,
            scalar_v968,
            scalar_v973,
            scalar_v974,
            scalar_v1067,
            scalar_v1068,
            scalar_v1069,
            scalar_v1071,
            scalar_v1072,
            scalar_v1073,
            scalar_v1074,
            scalar_v1075,
            scalar_v1076,
            scalar_v1078,
            scalar_v1079,
            scalar_v1081,
            scalar_v1123,
            scalar_v1124,
            scalar_v1126,
            scalar_v1127,
            scalar_v1128,
            scalar_v1129,
            scalar_v1130,
            scalar_v1131,
            scalar_v1132,
            scalar_v1133,
            scalar_v1134,
            scalar_v1135,
            scalar_v1136,
            scalar_v1138,
            scalar_v1139,
            scalar_v1140,
            scalar_v1141,
            scalar_v1146,
            scalar_v1147,
            scalar_v1148,
            scalar_v1149,
            scalar_v1150,
            scalar_v1151,
            scalar_v1159,
            scalar_v1160,
            scalar_v1161,
            scalar_v1162,
            scalar_v1165,
            scalar_v1166,
            scalar_v1167,
            scalar_v1168,
            scalar_v1169,
            scalar_v1170,
            scalar_v1172,
            scalar_v1173,
            scalar_v1181,
            scalar_v1182,
            scalar_v1184,
            scalar_v1185,
            scalar_v1192,
            scalar_v1193,
            scalar_v1211,
            scalar_v1213,
            scalar_v1214,
            scalar_v1233,
            scalar_v1234,
            scalar_v1235,
            scalar_v1236,
            scalar_v1241,
            scalar_v1242,
            scalar_v1335,
            scalar_v1336,
            scalar_v1337,
            scalar_v1339,
            scalar_v1340,
            scalar_v1341,
            scalar_v1342,
            scalar_v1343,
            scalar_v1344,
            scalar_v1346,
            scalar_v1347,
            scalar_v1349,
            scalar_v1391,
            scalar_v1392,
            scalar_v1394,
            scalar_v1395,
            scalar_v1396,
            scalar_v1397,
            scalar_v1398,
            scalar_v1399,
            scalar_v1400,
            scalar_v1401,
            scalar_v1402,
            scalar_v1403,
            scalar_v1404,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1409,
            scalar_v1414,
            scalar_v1415,
            scalar_v1416,
            scalar_v1417,
            scalar_v1418,
            scalar_v1419,
            scalar_v1433,
            scalar_v1487,
            scalar_v1488,
            scalar_v1499,
            scalar_v1500,
            scalar_v1501,
            scalar_v1525,
            scalar_v1526,
            scalar_v1527,
            scalar_v1528,
            scalar_v1529,
            scalar_v1530,
            scalar_v1531,
            scalar_v1532,
            scalar_v1533,
            scalar_v1534,
            scalar_v1535,
            scalar_v1536,
            scalar_v1537,
            scalar_v1538,
            scalar_v1539,
            scalar_v1540,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1696,
            scalar_v1698,
            scalar_v1740,
            scalar_v1741,
            scalar_v1742,
            scalar_v1743,
            scalar_v1744,
            scalar_v1745,
            scalar_v1746,
            scalar_v1748,
            scalar_v1749,
            scalar_v1750,
            scalar_v1755,
            scalar_v1756,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1917,
            scalar_v1918,
            scalar_v1919,
            scalar_v1921,
            scalar_v1923,
            scalar_v1965,
            scalar_v1966,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1973,
            scalar_v1974,
            scalar_v1975,
            scalar_v1980,
            scalar_v1981,
            scalar_v1982,
            scalar_v1983,
            scalar_v1984,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2146,
            scalar_v2148,
            scalar_v2190,
            scalar_v2191,
            scalar_v2192,
            scalar_v2193,
            scalar_v2194,
            scalar_v2195,
            scalar_v2196,
            scalar_v2198,
            scalar_v2199,
            scalar_v2200,
            scalar_v2205,
            scalar_v2206,
            scalar_v2207,
            scalar_v2208,
            scalar_v2209,
            scalar_v2223,
            scalar_v2277,
            scalar_v2278,
            scalar_v2289,
            scalar_v2290,
            scalar_v2291,
            scalar_v2315,
            scalar_v2316,
            scalar_v2317,
            scalar_v2318,
            scalar_v2319,
            scalar_v2320,
            scalar_v2321,
            scalar_v2322,
            scalar_v2323,
            scalar_v2324,
            scalar_v2325,
            scalar_v2326,
            scalar_v2327,
            scalar_v2328,
            scalar_v2329,
            scalar_v2330,
            scalar_v2482,
            scalar_v2483,
            scalar_v2484,
            scalar_v2486,
            scalar_v2488,
            scalar_v2530,
            scalar_v2531,
            scalar_v2532,
            scalar_v2533,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2538,
            scalar_v2539,
            scalar_v2540,
            scalar_v2545,
            scalar_v2546,
            scalar_v2547,
            scalar_v2548,
            scalar_v2549,
            scalar_v2707,
            scalar_v2708,
            scalar_v2709,
            scalar_v2711,
            scalar_v2713,
            scalar_v2755,
            scalar_v2756,
            scalar_v2757,
            scalar_v2758,
            scalar_v2759,
            scalar_v2760,
            scalar_v2761,
            scalar_v2763,
            scalar_v2764,
            scalar_v2765,
            scalar_v2770,
            scalar_v2771,
            scalar_v2772,
            scalar_v2773,
            scalar_v2774,
            scalar_v2932,
            scalar_v2933,
            scalar_v2934,
            scalar_v2936,
            scalar_v2938,
            scalar_v2980,
            scalar_v2981,
            scalar_v2982,
            scalar_v2983,
            scalar_v2984,
            scalar_v2985,
            scalar_v2986,
            scalar_v2988,
            scalar_v2989,
            scalar_v2990,
            scalar_v2995,
            scalar_v2996,
            scalar_v2997,
            scalar_v2998,
            scalar_v2999,
            scalar_v3013,
            scalar_v3067,
            scalar_v3068,
            scalar_v3079,
            scalar_v3080,
            scalar_v3081,
            scalar_v3105,
            scalar_v3106,
            scalar_v3107,
            scalar_v3108,
            scalar_v3109,
            scalar_v3110,
            scalar_v3111,
            scalar_v3112,
            scalar_v3113,
            scalar_v3114,
            scalar_v3115,
            scalar_v3116,
            scalar_v3117,
            scalar_v3118,
            scalar_v3119,
            scalar_v3120,
            scalar_v3272,
            scalar_v3273,
            scalar_v3274,
            scalar_v3276,
            scalar_v3278,
            scalar_v3320,
            scalar_v3321,
            scalar_v3322,
            scalar_v3323,
            scalar_v3324,
            scalar_v3325,
            scalar_v3326,
            scalar_v3328,
            scalar_v3329,
            scalar_v3330,
            scalar_v3335,
            scalar_v3336,
            scalar_v3337,
            scalar_v3338,
            scalar_v3339,
            scalar_v3497,
            scalar_v3498,
            scalar_v3499,
            scalar_v3501,
            scalar_v3503,
            scalar_v3545,
            scalar_v3546,
            scalar_v3547,
            scalar_v3548,
            scalar_v3549,
            scalar_v3550,
            scalar_v3551,
            scalar_v3553,
            scalar_v3554,
            scalar_v3555,
            scalar_v3560,
            scalar_v3561,
            scalar_v3562,
            scalar_v3563,
            scalar_v3564,
            scalar_v3722,
            scalar_v3723,
            scalar_v3724,
            scalar_v3726,
            scalar_v3728,
            scalar_v3770,
            scalar_v3771,
            scalar_v3772,
            scalar_v3773,
            scalar_v3774,
            scalar_v3775,
            scalar_v3776,
            scalar_v3778,
            scalar_v3779,
            scalar_v3780,
            scalar_v3785,
            scalar_v3786,
            scalar_v3787,
            scalar_v3788,
            scalar_v3789,
            scalar_v3803,
            scalar_v3857,
            scalar_v3858,
            scalar_v3869,
            scalar_v3870,
            scalar_v3871,
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
            scalar_v4062,
            scalar_v4063,
            scalar_v4064,
            scalar_v4066,
            scalar_v4068,
            scalar_v4110,
            scalar_v4111,
            scalar_v4112,
            scalar_v4113,
            scalar_v4114,
            scalar_v4115,
            scalar_v4116,
            scalar_v4118,
            scalar_v4119,
            scalar_v4120,
            scalar_v4125,
            scalar_v4126,
            scalar_v4127,
            scalar_v4128,
            scalar_v4129,
            scalar_v4287,
            scalar_v4288,
            scalar_v4289,
            scalar_v4291,
            scalar_v4293,
            scalar_v4335,
            scalar_v4336,
            scalar_v4337,
            scalar_v4338,
            scalar_v4339,
            scalar_v4340,
            scalar_v4341,
            scalar_v4343,
            scalar_v4344,
            scalar_v4345,
            scalar_v4350,
            scalar_v4351,
            scalar_v4352,
            scalar_v4353,
            scalar_v4354,
            scalar_v4512,
            scalar_v4513,
            scalar_v4514,
            scalar_v4516,
            scalar_v4518,
            scalar_v4560,
            scalar_v4561,
            scalar_v4562,
            scalar_v4563,
            scalar_v4564,
            scalar_v4565,
            scalar_v4566,
            scalar_v4568,
            scalar_v4569,
            scalar_v4570,
            scalar_v4575,
            scalar_v4576,
            scalar_v4577,
            scalar_v4578,
            scalar_v4579,
            scalar_v4630,
            scalar_v4694,
            scalar_v4697,
            scalar_v4698,
            scalar_v4706,
            scalar_v4723,
            scalar_v4744,
            scalar_v4761,
            scalar_v4791,
            scalar_v4880,
            scalar_v4887,
            scalar_v4898,
            scalar_v4905,
            scalar_v4915,
            scalar_v4922,
            scalar_v4931,
            scalar_v4932,
            scalar_v4933,
            scalar_v5055,
            scalar_v5056,
            scalar_v5058,
            scalar_v5061,
            scalar_v5062,
            scalar_v5063,
            scalar_v5071,
            scalar_v5072,
            scalar_v5073,
            scalar_v5098,
            scalar_v5099,
            scalar_v5100,
            scalar_v5118,
            scalar_v5123,
            scalar_v5216,
            scalar_v5217,
            scalar_v5218,
            scalar_v5219,
            scalar_v5224,
            scalar_v5268,
            scalar_v5269,
            scalar_v5271,
            scalar_v5298,
            scalar_v5303,
            scalar_v5306,
            scalar_v5314,
            scalar_v5315,
            scalar_v5317,
            scalar_v5320,
            scalar_v5322,
            scalar_v5330,
            scalar_v5332,
            scalar_v5356,
            scalar_v5358,
            scalar_v5376,
            scalar_v5381,
            scalar_v5474,
            scalar_v5476,
            scalar_v5477,
            scalar_v5482,
            scalar_v5526,
            scalar_v5529,
            scalar_v5556,
            scalar_v5561,
            scalar_v5570,
            scalar_v5571,
            scalar_v5573,
            scalar_v5576,
            scalar_v5578,
            scalar_v5586,
            scalar_v5588,
            scalar_v5612,
            scalar_v5614,
            scalar_v5632,
            scalar_v5637,
            scalar_v5730,
            scalar_v5732,
            scalar_v5733,
            scalar_v5738,
            scalar_v5782,
            scalar_v5785,
            scalar_v5812,
            scalar_v5813,
            scalar_v5816,
            scalar_v5856,
            scalar_v5861,
            scalar_v5892,
            scalar_v5893,
            scalar_v5898,
            scalar_v5899,
            scalar_v5911,
            scalar_v5912,
            scalar_v5913,
            scalar_v5918,
            scalar_v5938,
            scalar_v5939,
            scalar_v5940,
            scalar_v5941,
            scalar_v5942,
            scalar_v5945,
            scalar_v6010,
            scalar_v6011,
            scalar_v6047,
            scalar_v6048,
            scalar_v6049,
            scalar_v6050,
            scalar_v6051,
            scalar_v6052,
            scalar_v6053,
            scalar_v6054,
            scalar_v6092,
            scalar_v6120,
            scalar_v6146,
            scalar_v6163,
            scalar_v6164,
            scalar_v6165,
            scalar_v6166,
            scalar_v6167,
            scalar_v6168,
            scalar_v6169,
            scalar_v6170,
            scalar_v6450,
            scalar_v6558,
            scalar_v7061,
            scalar_v7171,
            scalar_v7674,
            scalar_v7784,
            scalar_v8188,
            scalar_v8189,
            scalar_v8198,
            scalar_v8199,
            scalar_v8328,
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
            scalar_v2,
            scalar_v4,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v11,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v23,
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
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
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
            scalar_v146,
            scalar_v148,
            scalar_v166,
            scalar_v172,
            scalar_v178,
            scalar_v184,
            scalar_v187,
            scalar_v190,
            scalar_v261,
            scalar_v262,
            scalar_v264,
            scalar_v265,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v290,
            scalar_v291,
            scalar_v296,
            scalar_v297,
            scalar_v301,
            scalar_v302,
            scalar_v314,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v359,
            scalar_v425,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v441,
            scalar_v442,
            scalar_v443,
            scalar_v444,
            scalar_v447,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v464,
            scalar_v465,
            scalar_v466,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v476,
            scalar_v478,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v491,
            scalar_v493,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v556,
            scalar_v557,
            scalar_v568,
            scalar_v569,
            scalar_v570,
            scalar_v596,
            scalar_v597,
            scalar_v598,
            scalar_v599,
            scalar_v600,
            scalar_v601,
            scalar_v602,
            scalar_v603,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v607,
            scalar_v608,
            scalar_v611,
            scalar_v612,
            scalar_v613,
            scalar_v614,
            scalar_v615,
            scalar_v616,
            scalar_v617,
            scalar_v618,
            scalar_v619,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v626,
            scalar_v627,
            scalar_v628,
            scalar_v629,
            scalar_v630,
            scalar_v638,
            scalar_v639,
            scalar_v640,
            scalar_v641,
            scalar_v642,
            scalar_v649,
            scalar_v650,
            scalar_v669,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v692,
            scalar_v693,
            scalar_v694,
            scalar_v695,
            scalar_v700,
            scalar_v701,
            scalar_v796,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v805,
            scalar_v807,
            scalar_v808,
            scalar_v810,
            scalar_v853,
            scalar_v854,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v860,
            scalar_v861,
            scalar_v862,
            scalar_v863,
            scalar_v864,
            scalar_v865,
            scalar_v866,
            scalar_v867,
            scalar_v869,
            scalar_v870,
            scalar_v871,
            scalar_v872,
            scalar_v877,
            scalar_v878,
            scalar_v879,
            scalar_v880,
            scalar_v881,
            scalar_v882,
            scalar_v884,
            scalar_v891,
            scalar_v892,
            scalar_v893,
            scalar_v894,
            scalar_v897,
            scalar_v898,
            scalar_v899,
            scalar_v900,
            scalar_v901,
            scalar_v902,
            scalar_v904,
            scalar_v905,
            scalar_v913,
            scalar_v914,
            scalar_v916,
            scalar_v917,
            scalar_v924,
            scalar_v925,
            scalar_v943,
            scalar_v945,
            scalar_v946,
            scalar_v965,
            scalar_v966,
            scalar_v967,
            scalar_v968,
            scalar_v973,
            scalar_v974,
            scalar_v1067,
            scalar_v1068,
            scalar_v1069,
            scalar_v1071,
            scalar_v1072,
            scalar_v1073,
            scalar_v1074,
            scalar_v1075,
            scalar_v1076,
            scalar_v1078,
            scalar_v1079,
            scalar_v1081,
            scalar_v1123,
            scalar_v1124,
            scalar_v1126,
            scalar_v1127,
            scalar_v1128,
            scalar_v1129,
            scalar_v1130,
            scalar_v1131,
            scalar_v1132,
            scalar_v1133,
            scalar_v1134,
            scalar_v1135,
            scalar_v1136,
            scalar_v1138,
            scalar_v1139,
            scalar_v1140,
            scalar_v1141,
            scalar_v1146,
            scalar_v1147,
            scalar_v1148,
            scalar_v1149,
            scalar_v1150,
            scalar_v1151,
            scalar_v1159,
            scalar_v1160,
            scalar_v1161,
            scalar_v1162,
            scalar_v1165,
            scalar_v1166,
            scalar_v1167,
            scalar_v1168,
            scalar_v1169,
            scalar_v1170,
            scalar_v1172,
            scalar_v1173,
            scalar_v1181,
            scalar_v1182,
            scalar_v1184,
            scalar_v1185,
            scalar_v1192,
            scalar_v1193,
            scalar_v1211,
            scalar_v1213,
            scalar_v1214,
            scalar_v1233,
            scalar_v1234,
            scalar_v1235,
            scalar_v1236,
            scalar_v1241,
            scalar_v1242,
            scalar_v1335,
            scalar_v1336,
            scalar_v1337,
            scalar_v1339,
            scalar_v1340,
            scalar_v1341,
            scalar_v1342,
            scalar_v1343,
            scalar_v1344,
            scalar_v1346,
            scalar_v1347,
            scalar_v1349,
            scalar_v1391,
            scalar_v1392,
            scalar_v1394,
            scalar_v1395,
            scalar_v1396,
            scalar_v1397,
            scalar_v1398,
            scalar_v1399,
            scalar_v1400,
            scalar_v1401,
            scalar_v1402,
            scalar_v1403,
            scalar_v1404,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1409,
            scalar_v1414,
            scalar_v1415,
            scalar_v1416,
            scalar_v1417,
            scalar_v1418,
            scalar_v1419,
            scalar_v1433,
            scalar_v1487,
            scalar_v1488,
            scalar_v1499,
            scalar_v1500,
            scalar_v1501,
            scalar_v1525,
            scalar_v1526,
            scalar_v1527,
            scalar_v1528,
            scalar_v1529,
            scalar_v1530,
            scalar_v1531,
            scalar_v1532,
            scalar_v1533,
            scalar_v1534,
            scalar_v1535,
            scalar_v1536,
            scalar_v1537,
            scalar_v1538,
            scalar_v1539,
            scalar_v1540,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1696,
            scalar_v1698,
            scalar_v1740,
            scalar_v1741,
            scalar_v1742,
            scalar_v1743,
            scalar_v1744,
            scalar_v1745,
            scalar_v1746,
            scalar_v1748,
            scalar_v1749,
            scalar_v1750,
            scalar_v1755,
            scalar_v1756,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1917,
            scalar_v1918,
            scalar_v1919,
            scalar_v1921,
            scalar_v1923,
            scalar_v1965,
            scalar_v1966,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1973,
            scalar_v1974,
            scalar_v1975,
            scalar_v1980,
            scalar_v1981,
            scalar_v1982,
            scalar_v1983,
            scalar_v1984,
            scalar_v2142,
            scalar_v2143,
            scalar_v2144,
            scalar_v2146,
            scalar_v2148,
            scalar_v2190,
            scalar_v2191,
            scalar_v2192,
            scalar_v2193,
            scalar_v2194,
            scalar_v2195,
            scalar_v2196,
            scalar_v2198,
            scalar_v2199,
            scalar_v2200,
            scalar_v2205,
            scalar_v2206,
            scalar_v2207,
            scalar_v2208,
            scalar_v2209,
            scalar_v2223,
            scalar_v2277,
            scalar_v2278,
            scalar_v2289,
            scalar_v2290,
            scalar_v2291,
            scalar_v2315,
            scalar_v2316,
            scalar_v2317,
            scalar_v2318,
            scalar_v2319,
            scalar_v2320,
            scalar_v2321,
            scalar_v2322,
            scalar_v2323,
            scalar_v2324,
            scalar_v2325,
            scalar_v2326,
            scalar_v2327,
            scalar_v2328,
            scalar_v2329,
            scalar_v2330,
            scalar_v2482,
            scalar_v2483,
            scalar_v2484,
            scalar_v2486,
            scalar_v2488,
            scalar_v2530,
            scalar_v2531,
            scalar_v2532,
            scalar_v2533,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2538,
            scalar_v2539,
            scalar_v2540,
            scalar_v2545,
            scalar_v2546,
            scalar_v2547,
            scalar_v2548,
            scalar_v2549,
            scalar_v2707,
            scalar_v2708,
            scalar_v2709,
            scalar_v2711,
            scalar_v2713,
            scalar_v2755,
            scalar_v2756,
            scalar_v2757,
            scalar_v2758,
            scalar_v2759,
            scalar_v2760,
            scalar_v2761,
            scalar_v2763,
            scalar_v2764,
            scalar_v2765,
            scalar_v2770,
            scalar_v2771,
            scalar_v2772,
            scalar_v2773,
            scalar_v2774,
            scalar_v2932,
            scalar_v2933,
            scalar_v2934,
            scalar_v2936,
            scalar_v2938,
            scalar_v2980,
            scalar_v2981,
            scalar_v2982,
            scalar_v2983,
            scalar_v2984,
            scalar_v2985,
            scalar_v2986,
            scalar_v2988,
            scalar_v2989,
            scalar_v2990,
            scalar_v2995,
            scalar_v2996,
            scalar_v2997,
            scalar_v2998,
            scalar_v2999,
            scalar_v3013,
            scalar_v3067,
            scalar_v3068,
            scalar_v3079,
            scalar_v3080,
            scalar_v3081,
            scalar_v3105,
            scalar_v3106,
            scalar_v3107,
            scalar_v3108,
            scalar_v3109,
            scalar_v3110,
            scalar_v3111,
            scalar_v3112,
            scalar_v3113,
            scalar_v3114,
            scalar_v3115,
            scalar_v3116,
            scalar_v3117,
            scalar_v3118,
            scalar_v3119,
            scalar_v3120,
            scalar_v3272,
            scalar_v3273,
            scalar_v3274,
            scalar_v3276,
            scalar_v3278,
            scalar_v3320,
            scalar_v3321,
            scalar_v3322,
            scalar_v3323,
            scalar_v3324,
            scalar_v3325,
            scalar_v3326,
            scalar_v3328,
            scalar_v3329,
            scalar_v3330,
            scalar_v3335,
            scalar_v3336,
            scalar_v3337,
            scalar_v3338,
            scalar_v3339,
            scalar_v3497,
            scalar_v3498,
            scalar_v3499,
            scalar_v3501,
            scalar_v3503,
            scalar_v3545,
            scalar_v3546,
            scalar_v3547,
            scalar_v3548,
            scalar_v3549,
            scalar_v3550,
            scalar_v3551,
            scalar_v3553,
            scalar_v3554,
            scalar_v3555,
            scalar_v3560,
            scalar_v3561,
            scalar_v3562,
            scalar_v3563,
            scalar_v3564,
            scalar_v3722,
            scalar_v3723,
            scalar_v3724,
            scalar_v3726,
            scalar_v3728,
            scalar_v3770,
            scalar_v3771,
            scalar_v3772,
            scalar_v3773,
            scalar_v3774,
            scalar_v3775,
            scalar_v3776,
            scalar_v3778,
            scalar_v3779,
            scalar_v3780,
            scalar_v3785,
            scalar_v3786,
            scalar_v3787,
            scalar_v3788,
            scalar_v3789,
            scalar_v3803,
            scalar_v3857,
            scalar_v3858,
            scalar_v3869,
            scalar_v3870,
            scalar_v3871,
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
            scalar_v4062,
            scalar_v4063,
            scalar_v4064,
            scalar_v4066,
            scalar_v4068,
            scalar_v4110,
            scalar_v4111,
            scalar_v4112,
            scalar_v4113,
            scalar_v4114,
            scalar_v4115,
            scalar_v4116,
            scalar_v4118,
            scalar_v4119,
            scalar_v4120,
            scalar_v4125,
            scalar_v4126,
            scalar_v4127,
            scalar_v4128,
            scalar_v4129,
            scalar_v4287,
            scalar_v4288,
            scalar_v4289,
            scalar_v4291,
            scalar_v4293,
            scalar_v4335,
            scalar_v4336,
            scalar_v4337,
            scalar_v4338,
            scalar_v4339,
            scalar_v4340,
            scalar_v4341,
            scalar_v4343,
            scalar_v4344,
            scalar_v4345,
            scalar_v4350,
            scalar_v4351,
            scalar_v4352,
            scalar_v4353,
            scalar_v4354,
            scalar_v4512,
            scalar_v4513,
            scalar_v4514,
            scalar_v4516,
            scalar_v4518,
            scalar_v4560,
            scalar_v4561,
            scalar_v4562,
            scalar_v4563,
            scalar_v4564,
            scalar_v4565,
            scalar_v4566,
            scalar_v4568,
            scalar_v4569,
            scalar_v4570,
            scalar_v4575,
            scalar_v4576,
            scalar_v4577,
            scalar_v4578,
            scalar_v4579,
            scalar_v4630,
            scalar_v4694,
            scalar_v4697,
            scalar_v4698,
            scalar_v4706,
            scalar_v4723,
            scalar_v4744,
            scalar_v4761,
            scalar_v4791,
            scalar_v4880,
            scalar_v4887,
            scalar_v4898,
            scalar_v4905,
            scalar_v4915,
            scalar_v4922,
            scalar_v4931,
            scalar_v4932,
            scalar_v4933,
            scalar_v5055,
            scalar_v5056,
            scalar_v5058,
            scalar_v5061,
            scalar_v5062,
            scalar_v5063,
            scalar_v5071,
            scalar_v5072,
            scalar_v5073,
            scalar_v5098,
            scalar_v5099,
            scalar_v5100,
            scalar_v5118,
            scalar_v5123,
            scalar_v5216,
            scalar_v5217,
            scalar_v5218,
            scalar_v5219,
            scalar_v5224,
            scalar_v5268,
            scalar_v5269,
            scalar_v5271,
            scalar_v5298,
            scalar_v5303,
            scalar_v5306,
            scalar_v5314,
            scalar_v5315,
            scalar_v5317,
            scalar_v5320,
            scalar_v5322,
            scalar_v5330,
            scalar_v5332,
            scalar_v5356,
            scalar_v5358,
            scalar_v5376,
            scalar_v5381,
            scalar_v5474,
            scalar_v5476,
            scalar_v5477,
            scalar_v5482,
            scalar_v5526,
            scalar_v5529,
            scalar_v5556,
            scalar_v5561,
            scalar_v5570,
            scalar_v5571,
            scalar_v5573,
            scalar_v5576,
            scalar_v5578,
            scalar_v5586,
            scalar_v5588,
            scalar_v5612,
            scalar_v5614,
            scalar_v5632,
            scalar_v5637,
            scalar_v5730,
            scalar_v5732,
            scalar_v5733,
            scalar_v5738,
            scalar_v5782,
            scalar_v5785,
            scalar_v5812,
            scalar_v5813,
            scalar_v5816,
            scalar_v5856,
            scalar_v5861,
            scalar_v5892,
            scalar_v5893,
            scalar_v5898,
            scalar_v5899,
            scalar_v5911,
            scalar_v5912,
            scalar_v5913,
            scalar_v5918,
            scalar_v5938,
            scalar_v5939,
            scalar_v5940,
            scalar_v5941,
            scalar_v5942,
            scalar_v5945,
            scalar_v6010,
            scalar_v6011,
            scalar_v6047,
            scalar_v6048,
            scalar_v6049,
            scalar_v6050,
            scalar_v6051,
            scalar_v6052,
            scalar_v6053,
            scalar_v6054,
            scalar_v6092,
            scalar_v6120,
            scalar_v6146,
            scalar_v6163,
            scalar_v6164,
            scalar_v6165,
            scalar_v6166,
            scalar_v6167,
            scalar_v6168,
            scalar_v6169,
            scalar_v6170,
            scalar_v6450,
            scalar_v6558,
            scalar_v7061,
            scalar_v7171,
            scalar_v7674,
            scalar_v7784,
            scalar_v8188,
            scalar_v8189,
            scalar_v8198,
            scalar_v8199,
            scalar_v8328,
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
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "ab" => { validate_parameter("AB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "ls" => { validate_parameter("LS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "lg" => { validate_parameter("LG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "mult" => { validate_parameter("MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("TRISE", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("TRISE", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "ifactor" => { validate_parameter("IFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "cfactor" => { validate_parameter("CFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "imax" => { validate_parameter("IMAX", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "trj" => { validate_parameter("TRJ", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "tref" => { validate_parameter("TRJ", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "frev" => { validate_parameter("FREV", value, Some((10.0, "10.0")), false, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "cjorbot" => { validate_parameter("CJORBOT", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cjorsti" => { validate_parameter("CJORSTI", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "cjorgat" => { validate_parameter("CJORGAT", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "vbirbot" => { validate_parameter("VBIRBOT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "vbirsti" => { validate_parameter("VBIRSTI", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "vbirgat" => { validate_parameter("VBIRGAT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "pbot" => { validate_parameter("PBOT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "psti" => { validate_parameter("PSTI", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "pgat" => { validate_parameter("PGAT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "phigbot" => { validate_finite_parameter("PHIGBOT", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "phigsti" => { validate_finite_parameter("PHIGSTI", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "phiggat" => { validate_finite_parameter("PHIGGAT", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "idsatrbot" => { validate_parameter("IDSATRBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "idsatrsti" => { validate_parameter("IDSATRSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "idsatrgat" => { validate_parameter("IDSATRGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "csrhbot" => { validate_parameter("CSRHBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "csrhsti" => { validate_parameter("CSRHSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "csrhgat" => { validate_parameter("CSRHGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "xjunsti" => { validate_parameter("XJUNSTI", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "xjungat" => { validate_parameter("XJUNGAT", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "ctatbot" => { validate_parameter("CTATBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "ctatsti" => { validate_parameter("CTATSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ctatgat" => { validate_parameter("CTATGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "mefftatbot" => { validate_parameter("MEFFTATBOT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "mefftatsti" => { validate_parameter("MEFFTATSTI", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "mefftatgat" => { validate_parameter("MEFFTATGAT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "cbbtbot" => { validate_parameter("CBBTBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "cbbtsti" => { validate_parameter("CBBTSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "cbbtgat" => { validate_parameter("CBBTGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "fbbtrbot" => { validate_finite_parameter("FBBTRBOT", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "fbbtrsti" => { validate_finite_parameter("FBBTRSTI", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "fbbtrgat" => { validate_finite_parameter("FBBTRGAT", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "stfbbtbot" => { validate_finite_parameter("STFBBTBOT", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "stfbbtsti" => { validate_finite_parameter("STFBBTSTI", value)?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "stfbbtgat" => { validate_finite_parameter("STFBBTGAT", value)?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "vbrbot" => { validate_parameter("VBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "vbrsti" => { validate_parameter("VBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "vbrgat" => { validate_parameter("VBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "pbrbot" => { validate_parameter("PBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "pbrsti" => { validate_parameter("PBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "pbrgat" => { validate_parameter("PBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "fcjorgat2" => { validate_parameter("FCJORGAT2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "fvbirgat2" => { validate_parameter("FVBIRGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "fpgat2" => { validate_parameter("FPGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "fphiggat2" => { validate_parameter("FPHIGGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "vtrgat" => { validate_parameter("VTRGAT", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "anugat" => { validate_parameter("ANUGAT", value, Some((0.001, "0.001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "swjunexp" => { validate_parameter("SWJUNEXP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "vjunref" => { validate_parameter("VJUNREF", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "fjunq" => { validate_parameter("FJUNQ", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'JUNCAP200'", name)),
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
        let v2: f64 = p.p62;
        self.scalar_v2 = v2;
        let v4: bool = (p.p62 > 0.5);
        self.scalar_v4 = v4;
        let v6: f64 = (if v4 { 1.0 } else { 0.0 });
        self.scalar_v6 = v6;
        let v7: bool = (!v4);
        self.scalar_v7 = v7;
        let v8: f64 = (if v7 { 0.0 } else { v6 });
        self.scalar_v8 = v8;
        let v10: f64 = p.p13;
        self.scalar_v10 = v10;
        let v11: f64 = (273.15 + p.p13);
        self.scalar_v11 = v11;
        let v14: f64 = (8.61726105451295e-5 * v11);
        self.scalar_v14 = v14;
        let v15: f64 = (1.0 / v14);
        self.scalar_v15 = v15;
        let v17: f64 = (0.000702 * v11);
        self.scalar_v17 = v17;
        let v18: f64 = (v17 * v11);
        self.scalar_v18 = v18;
        let v19: f64 = (-v18);
        self.scalar_v19 = v19;
        let v21: f64 = (1108.0 + v11);
        self.scalar_v21 = v21;
        let v22: f64 = (v19 / v21);
        self.scalar_v22 = v22;
        let v23: f64 = p.p24;
        self.scalar_v23 = v23;
        let v24: f64 = (p.p24 + v22);
        self.scalar_v24 = v24;
        let v25: f64 = p.p25;
        self.scalar_v25 = v25;
        let v26: f64 = (p.p25 + v22);
        self.scalar_v26 = v26;
        let v27: f64 = p.p26;
        self.scalar_v27 = v27;
        let v28: f64 = (p.p26 + v22);
        self.scalar_v28 = v28;
        let v29: f64 = p.p21;
        self.scalar_v29 = v29;
        let v30: f64 = (1.0 - p.p21);
        self.scalar_v30 = v30;
        let v31: f64 = p.p22;
        self.scalar_v31 = v31;
        let v32: f64 = (1.0 - p.p22);
        self.scalar_v32 = v32;
        let v33: f64 = p.p23;
        self.scalar_v33 = v33;
        let v34: f64 = (1.0 - p.p23);
        self.scalar_v34 = v34;
        let v35: f64 = (1.0 / v30);
        self.scalar_v35 = v35;
        let v36: f64 = (1.0 / v32);
        self.scalar_v36 = v36;
        let v37: f64 = (1.0 / v34);
        self.scalar_v37 = v37;
        let v38: f64 = p.p15;
        self.scalar_v38 = v38;
        let v39: f64 = (1.0447941624768001e-10 / p.p15);
        self.scalar_v39 = v39;
        let v40: f64 = p.p33;
        self.scalar_v40 = v40;
        let v41: f64 = (p.p33 * 1.0447941624768001e-10);
        self.scalar_v41 = v41;
        let v42: f64 = p.p16;
        self.scalar_v42 = v42;
        let v43: f64 = (v41 / p.p16);
        self.scalar_v43 = v43;
        let v44: f64 = p.p34;
        self.scalar_v44 = v44;
        let v45: f64 = (p.p34 * 1.0447941624768001e-10);
        self.scalar_v45 = v45;
        let v46: f64 = p.p17;
        self.scalar_v46 = v46;
        let v47: f64 = (v45 / p.p17);
        self.scalar_v47 = v47;
        let v48: f64 = (1.0 / v39);
        self.scalar_v48 = v48;
        let v49: f64 = (1.0 / v43);
        self.scalar_v49 = v49;
        let v50: f64 = (1.0 / v47);
        self.scalar_v50 = v50;
        let v51: f64 = p.p18;
        self.scalar_v51 = v51;
        let v52: f64 = (1.0 / p.p18);
        self.scalar_v52 = v52;
        let v53: f64 = p.p19;
        self.scalar_v53 = v53;
        let v54: f64 = (1.0 / p.p19);
        self.scalar_v54 = v54;
        let v55: f64 = p.p20;
        self.scalar_v55 = v55;
        let v56: f64 = (1.0 / p.p20);
        self.scalar_v56 = v56;
        let v63: f64 = p.p14;
        self.scalar_v63 = v63;
        let v64: f64 = (1.0 / p.p14);
        self.scalar_v64 = v64;
        let v65: f64 = (1.0 - v64);
        self.scalar_v65 = v65;
        let v66: f64 = p.p53;
        self.scalar_v66 = v66;
        let v67: f64 = f64::powf(v65, p.p53);
        self.scalar_v67 = v67;
        let v68: f64 = (1.0 - v67);
        self.scalar_v68 = v68;
        let v69: f64 = (1.0 / v68);
        self.scalar_v69 = v69;
        let v70: f64 = p.p54;
        self.scalar_v70 = v70;
        let v71: f64 = f64::powf(v65, p.p54);
        self.scalar_v71 = v71;
        let v72: f64 = (1.0 - v71);
        self.scalar_v72 = v72;
        let v73: f64 = (1.0 / v72);
        self.scalar_v73 = v73;
        let v74: f64 = p.p55;
        self.scalar_v74 = v74;
        let v75: f64 = f64::powf(v65, p.p55);
        self.scalar_v75 = v75;
        let v76: f64 = (1.0 - v75);
        self.scalar_v76 = v76;
        let v77: f64 = (1.0 / v76);
        self.scalar_v77 = v77;
        let v78: f64 = p.p50;
        self.scalar_v78 = v78;
        let v79: f64 = (1.0 / p.p50);
        self.scalar_v79 = v79;
        let v80: f64 = p.p51;
        self.scalar_v80 = v80;
        let v81: f64 = (1.0 / p.p51);
        self.scalar_v81 = v81;
        let v82: f64 = p.p52;
        self.scalar_v82 = v82;
        let v83: f64 = (1.0 / p.p52);
        self.scalar_v83 = v83;
        let v84: f64 = (v69 * v69);
        self.scalar_v84 = v84;
        let v85: f64 = (p.p53 - 1.0);
        self.scalar_v85 = v85;
        let v86: f64 = f64::powf(v65, v85);
        self.scalar_v86 = v86;
        let v87: f64 = (v84 * v86);
        self.scalar_v87 = v87;
        let v88: f64 = (-v87);
        self.scalar_v88 = v88;
        let v89: f64 = (v88 * p.p53);
        self.scalar_v89 = v89;
        let v90: f64 = (v89 * v79);
        self.scalar_v90 = v90;
        let v91: f64 = (v73 * v73);
        self.scalar_v91 = v91;
        let v92: f64 = (p.p54 - 1.0);
        self.scalar_v92 = v92;
        let v93: f64 = f64::powf(v65, v92);
        self.scalar_v93 = v93;
        let v94: f64 = (v91 * v93);
        self.scalar_v94 = v94;
        let v95: f64 = (-v94);
        self.scalar_v95 = v95;
        let v96: f64 = (v95 * p.p54);
        self.scalar_v96 = v96;
        let v97: f64 = (v96 * v81);
        self.scalar_v97 = v97;
        let v98: f64 = (v77 * v77);
        self.scalar_v98 = v98;
        let v99: f64 = (p.p55 - 1.0);
        self.scalar_v99 = v99;
        let v100: f64 = f64::powf(v65, v99);
        self.scalar_v100 = v100;
        let v101: f64 = (v98 * v100);
        self.scalar_v101 = v101;
        let v102: f64 = (-v101);
        self.scalar_v102 = v102;
        let v103: f64 = (v102 * p.p55);
        self.scalar_v103 = v103;
        let v104: f64 = (v103 * v83);
        self.scalar_v104 = v104;
        let v105: f64 = p.p56;
        self.scalar_v105 = v105;
        let v106: bool = (p.p56 != 1.0);
        self.scalar_v106 = v106;
        let v107: f64 = p.p57;
        self.scalar_v107 = v107;
        let v108: bool = (p.p57 != 1.0);
        self.scalar_v108 = v108;
        let v109: bool = (v106 || v108);
        self.scalar_v109 = v109;
        let v110: f64 = p.p58;
        self.scalar_v110 = v110;
        let v111: bool = (p.p58 != 1.0);
        self.scalar_v111 = v111;
        let v112: bool = (v109 || v111);
        self.scalar_v112 = v112;
        let v113: f64 = p.p59;
        self.scalar_v113 = v113;
        let v114: bool = (p.p59 != 1.0);
        self.scalar_v114 = v114;
        let v115: bool = (v112 || v114);
        self.scalar_v115 = v115;
        let v116: f64 = (if v115 { 1.0 } else { 0.0 });
        self.scalar_v116 = v116;
        let v117: bool = (!v115);
        self.scalar_v117 = v117;
        let v118: f64 = (if v117 { 0.0 } else { v116 });
        self.scalar_v118 = v118;
        let v119: bool = (v118 == 1.0);
        self.scalar_v119 = v119;
        let v120: f64 = (p.p17 * p.p56);
        self.scalar_v120 = v120;
        let v122: bool = (v120 > 1e-18);
        self.scalar_v122 = v122;
        let v123: f64 = (if v122 { v120 } else { 1e-18 });
        self.scalar_v123 = v123;
        let v124: f64 = (if v119 { v123 } else { 0.0 });
        self.scalar_v124 = v124;
        let v125: f64 = (p.p20 * p.p57);
        self.scalar_v125 = v125;
        let v127: bool = (v125 > 0.05);
        self.scalar_v127 = v127;
        let v128: f64 = (if v127 { v125 } else { 0.05 });
        self.scalar_v128 = v128;
        let v129: f64 = (if v119 { v128 } else { 0.0 });
        self.scalar_v129 = v129;
        let v130: f64 = (p.p23 * p.p58);
        self.scalar_v130 = v130;
        let v131: bool = (v130 > 0.05);
        self.scalar_v131 = v131;
        let v132: f64 = (if v131 { v130 } else { 0.05 });
        self.scalar_v132 = v132;
        let v134: bool = (v132 < 0.95);
        self.scalar_v134 = v134;
        let v135: f64 = (if v134 { v132 } else { 0.95 });
        self.scalar_v135 = v135;
        let v136: f64 = (if v119 { v135 } else { 0.0 });
        self.scalar_v136 = v136;
        let v137: f64 = (p.p26 * p.p59);
        self.scalar_v137 = v137;
        let v138: f64 = (if v119 { v137 } else { 0.0 });
        self.scalar_v138 = v138;
        let v139: f64 = (v138 + v22);
        self.scalar_v139 = v139;
        let v140: f64 = (if v119 { v139 } else { 0.0 });
        self.scalar_v140 = v140;
        let v141: f64 = (1.0 - v136);
        self.scalar_v141 = v141;
        let v142: f64 = (if v119 { v141 } else { 0.0 });
        self.scalar_v142 = v142;
        let v143: f64 = (1.0 / v142);
        self.scalar_v143 = v143;
        let v144: f64 = (if v119 { v143 } else { 0.0 });
        self.scalar_v144 = v144;
        let v146: f64 = p.p2;
        self.scalar_v146 = v146;
        let v148: f64 = p.p9;
        self.scalar_v148 = v148;
        let v166: f64 = (v24 * v15);
        self.scalar_v166 = v166;
        let v172: f64 = (v26 * v15);
        self.scalar_v172 = v172;
        let v178: f64 = (v28 * v15);
        self.scalar_v178 = v178;
        let v184: f64 = p.p27;
        self.scalar_v184 = v184;
        let v187: f64 = p.p28;
        self.scalar_v187 = v187;
        let v190: f64 = p.p29;
        self.scalar_v190 = v190;
        let v261: f64 = p.p38;
        self.scalar_v261 = v261;
        let v262: f64 = (32.0 * p.p38);
        self.scalar_v262 = v262;
        let v264: f64 = (v262 * 9.1093826e-31);
        self.scalar_v264 = v264;
        let v265: f64 = (v264 * 1.6021918e-19);
        self.scalar_v265 = v265;
        let v272: f64 = p.p39;
        self.scalar_v272 = v272;
        let v273: f64 = (32.0 * p.p39);
        self.scalar_v273 = v273;
        let v274: f64 = (v273 * 9.1093826e-31);
        self.scalar_v274 = v274;
        let v275: f64 = (v274 * 1.6021918e-19);
        self.scalar_v275 = v275;
        let v281: f64 = p.p40;
        self.scalar_v281 = v281;
        let v282: f64 = (32.0 * p.p40);
        self.scalar_v282 = v282;
        let v283: f64 = (v282 * 9.1093826e-31);
        self.scalar_v283 = v283;
        let v284: f64 = (v283 * 1.6021918e-19);
        self.scalar_v284 = v284;
        let v290: f64 = p.p44;
        self.scalar_v290 = v290;
        let v291: f64 = p.p47;
        self.scalar_v291 = v291;
        let v296: f64 = p.p45;
        self.scalar_v296 = v296;
        let v297: f64 = p.p48;
        self.scalar_v297 = v297;
        let v301: f64 = p.p46;
        self.scalar_v301 = v301;
        let v302: f64 = p.p49;
        self.scalar_v302 = v302;
        let v314: f64 = (v140 * v15);
        self.scalar_v314 = v314;
        let v345: f64 = p.p3;
        self.scalar_v345 = v345;
        let v346: bool = (p.p3 > 0.0);
        self.scalar_v346 = v346;
        let v347: f64 = (if v346 { p.p3 } else { 0.0 });
        self.scalar_v347 = v347;
        let v348: f64 = p.p4;
        self.scalar_v348 = v348;
        let v349: bool = (p.p4 > 0.0);
        self.scalar_v349 = v349;
        let v350: f64 = (if v349 { p.p4 } else { 0.0 });
        self.scalar_v350 = v350;
        let v351: f64 = p.p5;
        self.scalar_v351 = v351;
        let v352: bool = (p.p5 > 0.0);
        self.scalar_v352 = v352;
        let v353: f64 = (if v352 { p.p5 } else { 0.0 });
        self.scalar_v353 = v353;
        let v354: f64 = p.p6;
        self.scalar_v354 = v354;
        let v355: bool = (p.p6 > 0.0);
        self.scalar_v355 = v355;
        let v356: f64 = (if v355 { p.p6 } else { 0.0 });
        self.scalar_v356 = v356;
        let v359: f64 = p.p12;
        self.scalar_v359 = v359;
        let v425: bool = (v347 == 0.0);
        self.scalar_v425 = v425;
        let v429: bool = (p.p22 < p.p23);
        self.scalar_v429 = v429;
        let v430: f64 = (if v429 { p.p22 } else { p.p23 });
        self.scalar_v430 = v430;
        let v431: f64 = (0.9 * v430);
        self.scalar_v431 = v431;
        let v432: f64 = (if v425 { v431 } else { p.p21 });
        self.scalar_v432 = v432;
        let v433: f64 = (p.p19 + p.p20);
        self.scalar_v433 = v433;
        let v434: f64 = (if v425 { v433 } else { p.p18 });
        self.scalar_v434 = v434;
        let v435: bool = (v350 == 0.0);
        self.scalar_v435 = v435;
        let v438: bool = (p.p21 < p.p23);
        self.scalar_v438 = v438;
        let v439: f64 = (if v438 { p.p21 } else { p.p23 });
        self.scalar_v439 = v439;
        let v440: f64 = (0.9 * v439);
        self.scalar_v440 = v440;
        let v441: f64 = (if v435 { v440 } else { p.p22 });
        self.scalar_v441 = v441;
        let v442: f64 = (p.p18 + p.p20);
        self.scalar_v442 = v442;
        let v443: f64 = (if v435 { v442 } else { p.p19 });
        self.scalar_v443 = v443;
        let v444: bool = (v353 == 0.0);
        self.scalar_v444 = v444;
        let v447: bool = (p.p21 < p.p22);
        self.scalar_v447 = v447;
        let v448: f64 = (if v447 { p.p21 } else { p.p22 });
        self.scalar_v448 = v448;
        let v449: f64 = (0.9 * v448);
        self.scalar_v449 = v449;
        let v450: f64 = (if v444 { v449 } else { p.p23 });
        self.scalar_v450 = v450;
        let v451: f64 = (p.p18 + p.p19);
        self.scalar_v451 = v451;
        let v452: f64 = (if v444 { v451 } else { p.p20 });
        self.scalar_v452 = v452;
        let v459: bool = (v432 > v441);
        self.scalar_v459 = v459;
        let v460: f64 = (if v459 { v432 } else { v441 });
        self.scalar_v460 = v460;
        let v461: bool = (v460 > v450);
        self.scalar_v461 = v461;
        let v462: f64 = (if v461 { v460 } else { v450 });
        self.scalar_v462 = v462;
        let v464: f64 = (-1.0 / v462);
        self.scalar_v464 = v464;
        let v465: f64 = f64::powf(2.0, v464);
        self.scalar_v465 = v465;
        let v466: f64 = (1.0 - v465);
        self.scalar_v466 = v466;
        let v468: bool = (v434 < v443);
        self.scalar_v468 = v468;
        let v469: f64 = (if v468 { v434 } else { v443 });
        self.scalar_v469 = v469;
        let v470: bool = (v469 < v452);
        self.scalar_v470 = v470;
        let v471: f64 = (if v470 { v469 } else { v452 });
        self.scalar_v471 = v471;
        let v472: f64 = (v471 - 0.05);
        self.scalar_v472 = v472;
        let v473: bool = (v8 == 1.0);
        self.scalar_v473 = v473;
        let v474: f64 = (if v473 { 0.0 } else { 0.0 });
        self.scalar_v474 = v474;
        let v476: f64 = (if v473 { 0.4 } else { 0.0 });
        self.scalar_v476 = v476;
        let v478: f64 = (if v473 { 0.65 } else { 0.0 });
        self.scalar_v478 = v478;
        let v480: f64 = (if v473 { 0.8 } else { 0.0 });
        self.scalar_v480 = v480;
        let v481: f64 = (-v476);
        self.scalar_v481 = v481;
        let v482: f64 = p.p63;
        self.scalar_v482 = v482;
        let v483: f64 = (v481 * p.p63);
        self.scalar_v483 = v483;
        let v484: f64 = (if v473 { v483 } else { 0.0 });
        self.scalar_v484 = v484;
        let v485: f64 = (-v478);
        self.scalar_v485 = v485;
        let v486: f64 = (v485 * p.p63);
        self.scalar_v486 = v486;
        let v487: f64 = (if v473 { v486 } else { 0.0 });
        self.scalar_v487 = v487;
        let v488: f64 = (-v480);
        self.scalar_v488 = v488;
        let v489: f64 = (v488 * p.p63);
        self.scalar_v489 = v489;
        let v490: f64 = (if v473 { v489 } else { 0.0 });
        self.scalar_v490 = v490;
        let v491: f64 = (if v473 { 0.1 } else { 0.0 });
        self.scalar_v491 = v491;
        let v493: f64 = (if v473 { 0.2 } else { 0.0 });
        self.scalar_v493 = v493;
        let v494: f64 = (if v473 { 0.0 } else { v474 });
        self.scalar_v494 = v494;
        let v495: bool = (v425 && v435);
        self.scalar_v495 = v495;
        let v496: bool = (v495 && v444);
        self.scalar_v496 = v496;
        let v497: bool = (!v496);
        self.scalar_v497 = v497;
        let v498: bool = (v473 && v497);
        self.scalar_v498 = v498;
        let v556: bool = (v484 > 0.0);
        self.scalar_v556 = v556;
        let v557: bool = (v498 && v556);
        self.scalar_v557 = v557;
        let v568: bool = (!v556);
        self.scalar_v568 = v568;
        let v569: bool = (v498 && v568);
        self.scalar_v569 = v569;
        let v570: f64 = (-v484);
        self.scalar_v570 = v570;
        let v596: f64 = (v484 + v472);
        self.scalar_v596 = v596;
        let v597: f64 = (v484 - v472);
        self.scalar_v597 = v597;
        let v598: f64 = (v597 * v597);
        self.scalar_v598 = v598;
        let v599: f64 = (4.0 * v14);
        self.scalar_v599 = v599;
        let v600: f64 = (v599 * v14);
        self.scalar_v600 = v600;
        let v601: f64 = (v598 + v600);
        self.scalar_v601 = v601;
        let v602: f64 = v601.sqrt();
        self.scalar_v602 = v602;
        let v603: f64 = (v596 - v602);
        self.scalar_v603 = v603;
        let v604: f64 = (0.5 * v603);
        self.scalar_v604 = v604;
        let v605: f64 = (if v498 { v604 } else { v494 });
        self.scalar_v605 = v605;
        let v606: f64 = (v484 + 0.0);
        self.scalar_v606 = v606;
        let v607: f64 = (v484 - 0.0);
        self.scalar_v607 = v607;
        let v608: f64 = (v607 * v607);
        self.scalar_v608 = v608;
        let v611: f64 = (v608 + 4e-12);
        self.scalar_v611 = v611;
        let v612: f64 = v611.sqrt();
        self.scalar_v612 = v612;
        let v613: f64 = (v606 - v612);
        self.scalar_v613 = v613;
        let v614: f64 = (0.5 * v613);
        self.scalar_v614 = v614;
        let v615: f64 = (if v498 { v614 } else { v474 });
        self.scalar_v615 = v615;
        let v616: bool = (v473 && v425);
        self.scalar_v616 = v616;
        let v617: f64 = (if v616 { 0.0 } else { 0.0 });
        self.scalar_v617 = v617;
        let v618: bool = (!v425);
        self.scalar_v618 = v618;
        let v619: bool = (v473 && v618);
        self.scalar_v619 = v619;
        let v622: f64 = p.p30;
        self.scalar_v622 = v622;
        let v623: bool = (p.p30 == 0.0);
        self.scalar_v623 = v623;
        let v624: f64 = p.p35;
        self.scalar_v624 = v624;
        let v625: bool = (p.p35 == 0.0);
        self.scalar_v625 = v625;
        let v626: bool = (v623 && v625);
        self.scalar_v626 = v626;
        let v627: bool = (v619 && v626);
        self.scalar_v627 = v627;
        let v628: f64 = (if v627 { 0.0 } else { v474 });
        self.scalar_v628 = v628;
        let v629: bool = (!v626);
        self.scalar_v629 = v629;
        let v630: bool = (v619 && v629);
        self.scalar_v630 = v630;
        let v638: bool = (p.p21 == 0.5);
        self.scalar_v638 = v638;
        let v639: bool = (v630 && v638);
        self.scalar_v639 = v639;
        let v640: f64 = (if v639 { 0.0 } else { v474 });
        self.scalar_v640 = v640;
        let v641: bool = (!v638);
        self.scalar_v641 = v641;
        let v642: bool = (v630 && v641);
        self.scalar_v642 = v642;
        let v649: f64 = (2.0 * p.p21);
        self.scalar_v649 = v649;
        let v650: f64 = (1.0 - v649);
        self.scalar_v650 = v650;
        let v669: bool = (v619 && v625);
        self.scalar_v669 = v669;
        let v670: f64 = (if v669 { 0.0 } else { v474 });
        self.scalar_v670 = v670;
        let v671: bool = (!v625);
        self.scalar_v671 = v671;
        let v672: bool = (v619 && v671);
        self.scalar_v672 = v672;
        let v692: f64 = (-p.p21);
        self.scalar_v692 = v692;
        let v693: f64 = (v692 * v35);
        self.scalar_v693 = v693;
        let v694: bool = (v693 == -1.0);
        self.scalar_v694 = v694;
        let v695: bool = (v672 && v694);
        self.scalar_v695 = v695;
        let v700: bool = (!v694);
        self.scalar_v700 = v700;
        let v701: bool = (v672 && v700);
        self.scalar_v701 = v701;
        let v796: f64 = p.p41;
        self.scalar_v796 = v796;
        let v797: bool = (p.p41 == 0.0);
        self.scalar_v797 = v797;
        let v798: bool = (v619 && v797);
        self.scalar_v798 = v798;
        let v799: f64 = (if v798 { 0.0 } else { v474 });
        self.scalar_v799 = v799;
        let v800: bool = (!v797);
        self.scalar_v800 = v800;
        let v801: bool = (v619 && v800);
        self.scalar_v801 = v801;
        let v802: bool = (v801 && v638);
        self.scalar_v802 = v802;
        let v803: f64 = (p.p18 - v605);
        self.scalar_v803 = v803;
        let v804: f64 = (v803 * v52);
        self.scalar_v804 = v804;
        let v805: f64 = v804.sqrt();
        self.scalar_v805 = v805;
        let v807: bool = (v801 && v641);
        self.scalar_v807 = v807;
        let v808: f64 = f64::powf(v804, p.p21);
        self.scalar_v808 = v808;
        let v810: f64 = (v803 * v48);
        self.scalar_v810 = v810;
        let v853: bool = (p.p50 > 1000.0);
        self.scalar_v853 = v853;
        let v854: bool = (v619 && v853);
        self.scalar_v854 = v854;
        let v855: f64 = (if v854 { 1.0 } else { v474 });
        self.scalar_v855 = v855;
        let v856: f64 = (-v65);
        self.scalar_v856 = v856;
        let v857: f64 = (v856 * p.p50);
        self.scalar_v857 = v857;
        let v858: bool = (v615 > v857);
        self.scalar_v858 = v858;
        let v859: bool = (p.p53 == 4.0);
        self.scalar_v859 = v859;
        let v860: bool = (!v853);
        self.scalar_v860 = v860;
        let v861: bool = (v619 && v860);
        self.scalar_v861 = v861;
        let v862: bool = (v861 && v858);
        self.scalar_v862 = v862;
        let v863: bool = (v862 && v859);
        self.scalar_v863 = v863;
        let v864: f64 = (v615 * v79);
        self.scalar_v864 = v864;
        let v865: f64 = (v864 * v864);
        self.scalar_v865 = v865;
        let v866: f64 = (v865 * v864);
        self.scalar_v866 = v866;
        let v867: f64 = (v866 * v864);
        self.scalar_v867 = v867;
        let v869: bool = (!v859);
        self.scalar_v869 = v869;
        let v870: bool = (v862 && v869);
        self.scalar_v870 = v870;
        let v871: f64 = v864.abs();
        self.scalar_v871 = v871;
        let v872: f64 = f64::powf(v871, p.p53);
        self.scalar_v872 = v872;
        let v877: bool = (!v858);
        self.scalar_v877 = v877;
        let v878: bool = (v861 && v877);
        self.scalar_v878 = v878;
        let v879: f64 = (v65 * p.p50);
        self.scalar_v879 = v879;
        let v880: f64 = (v615 + v879);
        self.scalar_v880 = v880;
        let v881: f64 = (v880 * v90);
        self.scalar_v881 = v881;
        let v882: f64 = (v69 + v881);
        self.scalar_v882 = v882;
        let v884: f64 = p.p10;
        self.scalar_v884 = v884;
        let v891: bool = (v473 && v435);
        self.scalar_v891 = v891;
        let v892: f64 = (if v891 { 0.0 } else { 0.0 });
        self.scalar_v892 = v892;
        let v893: bool = (!v435);
        self.scalar_v893 = v893;
        let v894: bool = (v473 && v893);
        self.scalar_v894 = v894;
        let v897: f64 = p.p31;
        self.scalar_v897 = v897;
        let v898: bool = (p.p31 == 0.0);
        self.scalar_v898 = v898;
        let v899: f64 = p.p36;
        self.scalar_v899 = v899;
        let v900: bool = (p.p36 == 0.0);
        self.scalar_v900 = v900;
        let v901: bool = (v898 && v900);
        self.scalar_v901 = v901;
        let v902: bool = (v894 && v901);
        self.scalar_v902 = v902;
        let v904: bool = (!v901);
        self.scalar_v904 = v904;
        let v905: bool = (v894 && v904);
        self.scalar_v905 = v905;
        let v913: bool = (p.p22 == 0.5);
        self.scalar_v913 = v913;
        let v914: bool = (v905 && v913);
        self.scalar_v914 = v914;
        let v916: bool = (!v913);
        self.scalar_v916 = v916;
        let v917: bool = (v905 && v916);
        self.scalar_v917 = v917;
        let v924: f64 = (2.0 * p.p22);
        self.scalar_v924 = v924;
        let v925: f64 = (1.0 - v924);
        self.scalar_v925 = v925;
        let v943: bool = (v894 && v900);
        self.scalar_v943 = v943;
        let v945: bool = (!v900);
        self.scalar_v945 = v945;
        let v946: bool = (v894 && v945);
        self.scalar_v946 = v946;
        let v965: f64 = (-p.p22);
        self.scalar_v965 = v965;
        let v966: f64 = (v965 * v36);
        self.scalar_v966 = v966;
        let v967: bool = (v966 == -1.0);
        self.scalar_v967 = v967;
        let v968: bool = (v946 && v967);
        self.scalar_v968 = v968;
        let v973: bool = (!v967);
        self.scalar_v973 = v973;
        let v974: bool = (v946 && v973);
        self.scalar_v974 = v974;
        let v1067: f64 = p.p42;
        self.scalar_v1067 = v1067;
        let v1068: bool = (p.p42 == 0.0);
        self.scalar_v1068 = v1068;
        let v1069: bool = (v894 && v1068);
        self.scalar_v1069 = v1069;
        let v1071: bool = (!v1068);
        self.scalar_v1071 = v1071;
        let v1072: bool = (v894 && v1071);
        self.scalar_v1072 = v1072;
        let v1073: bool = (v1072 && v913);
        self.scalar_v1073 = v1073;
        let v1074: f64 = (p.p19 - v605);
        self.scalar_v1074 = v1074;
        let v1075: f64 = (v1074 * v54);
        self.scalar_v1075 = v1075;
        let v1076: f64 = v1075.sqrt();
        self.scalar_v1076 = v1076;
        let v1078: bool = (v1072 && v916);
        self.scalar_v1078 = v1078;
        let v1079: f64 = f64::powf(v1075, p.p22);
        self.scalar_v1079 = v1079;
        let v1081: f64 = (v1074 * v49);
        self.scalar_v1081 = v1081;
        let v1123: bool = (p.p51 > 1000.0);
        self.scalar_v1123 = v1123;
        let v1124: bool = (v894 && v1123);
        self.scalar_v1124 = v1124;
        let v1126: f64 = (v856 * p.p51);
        self.scalar_v1126 = v1126;
        let v1127: bool = (v615 > v1126);
        self.scalar_v1127 = v1127;
        let v1128: bool = (p.p54 == 4.0);
        self.scalar_v1128 = v1128;
        let v1129: bool = (!v1123);
        self.scalar_v1129 = v1129;
        let v1130: bool = (v894 && v1129);
        self.scalar_v1130 = v1130;
        let v1131: bool = (v1130 && v1127);
        self.scalar_v1131 = v1131;
        let v1132: bool = (v1131 && v1128);
        self.scalar_v1132 = v1132;
        let v1133: f64 = (v615 * v81);
        self.scalar_v1133 = v1133;
        let v1134: f64 = (v1133 * v1133);
        self.scalar_v1134 = v1134;
        let v1135: f64 = (v1134 * v1133);
        self.scalar_v1135 = v1135;
        let v1136: f64 = (v1135 * v1133);
        self.scalar_v1136 = v1136;
        let v1138: bool = (!v1128);
        self.scalar_v1138 = v1138;
        let v1139: bool = (v1131 && v1138);
        self.scalar_v1139 = v1139;
        let v1140: f64 = v1133.abs();
        self.scalar_v1140 = v1140;
        let v1141: f64 = f64::powf(v1140, p.p54);
        self.scalar_v1141 = v1141;
        let v1146: bool = (!v1127);
        self.scalar_v1146 = v1146;
        let v1147: bool = (v1130 && v1146);
        self.scalar_v1147 = v1147;
        let v1148: f64 = (v65 * p.p51);
        self.scalar_v1148 = v1148;
        let v1149: f64 = (v615 + v1148);
        self.scalar_v1149 = v1149;
        let v1150: f64 = (v1149 * v97);
        self.scalar_v1150 = v1150;
        let v1151: f64 = (v73 + v1150);
        self.scalar_v1151 = v1151;
        let v1159: bool = (v473 && v444);
        self.scalar_v1159 = v1159;
        let v1160: f64 = (if v1159 { 0.0 } else { 0.0 });
        self.scalar_v1160 = v1160;
        let v1161: bool = (!v444);
        self.scalar_v1161 = v1161;
        let v1162: bool = (v473 && v1161);
        self.scalar_v1162 = v1162;
        let v1165: f64 = p.p32;
        self.scalar_v1165 = v1165;
        let v1166: bool = (p.p32 == 0.0);
        self.scalar_v1166 = v1166;
        let v1167: f64 = p.p37;
        self.scalar_v1167 = v1167;
        let v1168: bool = (p.p37 == 0.0);
        self.scalar_v1168 = v1168;
        let v1169: bool = (v1166 && v1168);
        self.scalar_v1169 = v1169;
        let v1170: bool = (v1162 && v1169);
        self.scalar_v1170 = v1170;
        let v1172: bool = (!v1169);
        self.scalar_v1172 = v1172;
        let v1173: bool = (v1162 && v1172);
        self.scalar_v1173 = v1173;
        let v1181: bool = (p.p23 == 0.5);
        self.scalar_v1181 = v1181;
        let v1182: bool = (v1173 && v1181);
        self.scalar_v1182 = v1182;
        let v1184: bool = (!v1181);
        self.scalar_v1184 = v1184;
        let v1185: bool = (v1173 && v1184);
        self.scalar_v1185 = v1185;
        let v1192: f64 = (2.0 * p.p23);
        self.scalar_v1192 = v1192;
        let v1193: f64 = (1.0 - v1192);
        self.scalar_v1193 = v1193;
        let v1211: bool = (v1162 && v1168);
        self.scalar_v1211 = v1211;
        let v1213: bool = (!v1168);
        self.scalar_v1213 = v1213;
        let v1214: bool = (v1162 && v1213);
        self.scalar_v1214 = v1214;
        let v1233: f64 = (-p.p23);
        self.scalar_v1233 = v1233;
        let v1234: f64 = (v1233 * v37);
        self.scalar_v1234 = v1234;
        let v1235: bool = (v1234 == -1.0);
        self.scalar_v1235 = v1235;
        let v1236: bool = (v1214 && v1235);
        self.scalar_v1236 = v1236;
        let v1241: bool = (!v1235);
        self.scalar_v1241 = v1241;
        let v1242: bool = (v1214 && v1241);
        self.scalar_v1242 = v1242;
        let v1335: f64 = p.p43;
        self.scalar_v1335 = v1335;
        let v1336: bool = (p.p43 == 0.0);
        self.scalar_v1336 = v1336;
        let v1337: bool = (v1162 && v1336);
        self.scalar_v1337 = v1337;
        let v1339: bool = (!v1336);
        self.scalar_v1339 = v1339;
        let v1340: bool = (v1162 && v1339);
        self.scalar_v1340 = v1340;
        let v1341: bool = (v1340 && v1181);
        self.scalar_v1341 = v1341;
        let v1342: f64 = (p.p20 - v605);
        self.scalar_v1342 = v1342;
        let v1343: f64 = (v1342 * v56);
        self.scalar_v1343 = v1343;
        let v1344: f64 = v1343.sqrt();
        self.scalar_v1344 = v1344;
        let v1346: bool = (v1340 && v1184);
        self.scalar_v1346 = v1346;
        let v1347: f64 = f64::powf(v1343, p.p23);
        self.scalar_v1347 = v1347;
        let v1349: f64 = (v1342 * v50);
        self.scalar_v1349 = v1349;
        let v1391: bool = (p.p52 > 1000.0);
        self.scalar_v1391 = v1391;
        let v1392: bool = (v1162 && v1391);
        self.scalar_v1392 = v1392;
        let v1394: f64 = (v856 * p.p52);
        self.scalar_v1394 = v1394;
        let v1395: bool = (v615 > v1394);
        self.scalar_v1395 = v1395;
        let v1396: bool = (p.p55 == 4.0);
        self.scalar_v1396 = v1396;
        let v1397: bool = (!v1391);
        self.scalar_v1397 = v1397;
        let v1398: bool = (v1162 && v1397);
        self.scalar_v1398 = v1398;
        let v1399: bool = (v1398 && v1395);
        self.scalar_v1399 = v1399;
        let v1400: bool = (v1399 && v1396);
        self.scalar_v1400 = v1400;
        let v1401: f64 = (v615 * v83);
        self.scalar_v1401 = v1401;
        let v1402: f64 = (v1401 * v1401);
        self.scalar_v1402 = v1402;
        let v1403: f64 = (v1402 * v1401);
        self.scalar_v1403 = v1403;
        let v1404: f64 = (v1403 * v1401);
        self.scalar_v1404 = v1404;
        let v1406: bool = (!v1396);
        self.scalar_v1406 = v1406;
        let v1407: bool = (v1399 && v1406);
        self.scalar_v1407 = v1407;
        let v1408: f64 = v1401.abs();
        self.scalar_v1408 = v1408;
        let v1409: f64 = f64::powf(v1408, p.p55);
        self.scalar_v1409 = v1409;
        let v1414: bool = (!v1395);
        self.scalar_v1414 = v1414;
        let v1415: bool = (v1398 && v1414);
        self.scalar_v1415 = v1415;
        let v1416: f64 = (v65 * p.p52);
        self.scalar_v1416 = v1416;
        let v1417: f64 = (v615 + v1416);
        self.scalar_v1417 = v1417;
        let v1418: f64 = (v1417 * v104);
        self.scalar_v1418 = v1418;
        let v1419: f64 = (v77 + v1418);
        self.scalar_v1419 = v1419;
        let v1433: f64 = (if v473 { 0.0 } else { v605 });
        self.scalar_v1433 = v1433;
        let v1487: bool = (v487 > 0.0);
        self.scalar_v1487 = v1487;
        let v1488: bool = (v498 && v1487);
        self.scalar_v1488 = v1488;
        let v1499: bool = (!v1487);
        self.scalar_v1499 = v1499;
        let v1500: bool = (v498 && v1499);
        self.scalar_v1500 = v1500;
        let v1501: f64 = (-v487);
        self.scalar_v1501 = v1501;
        let v1525: f64 = (v487 + v472);
        self.scalar_v1525 = v1525;
        let v1526: f64 = (v487 - v472);
        self.scalar_v1526 = v1526;
        let v1527: f64 = (v1526 * v1526);
        self.scalar_v1527 = v1527;
        let v1528: f64 = (v1527 + v600);
        self.scalar_v1528 = v1528;
        let v1529: f64 = v1528.sqrt();
        self.scalar_v1529 = v1529;
        let v1530: f64 = (v1525 - v1529);
        self.scalar_v1530 = v1530;
        let v1531: f64 = (0.5 * v1530);
        self.scalar_v1531 = v1531;
        let v1532: f64 = (if v498 { v1531 } else { v1433 });
        self.scalar_v1532 = v1532;
        let v1533: f64 = (v487 + 0.0);
        self.scalar_v1533 = v1533;
        let v1534: f64 = (v487 - 0.0);
        self.scalar_v1534 = v1534;
        let v1535: f64 = (v1534 * v1534);
        self.scalar_v1535 = v1535;
        let v1536: f64 = (v1535 + 4e-12);
        self.scalar_v1536 = v1536;
        let v1537: f64 = v1536.sqrt();
        self.scalar_v1537 = v1537;
        let v1538: f64 = (v1533 - v1537);
        self.scalar_v1538 = v1538;
        let v1539: f64 = (0.5 * v1538);
        self.scalar_v1539 = v1539;
        let v1540: f64 = (if v498 { v1539 } else { v615 });
        self.scalar_v1540 = v1540;
        let v1692: f64 = (p.p18 - v1532);
        self.scalar_v1692 = v1692;
        let v1693: f64 = (v1692 * v52);
        self.scalar_v1693 = v1693;
        let v1694: f64 = v1693.sqrt();
        self.scalar_v1694 = v1694;
        let v1696: f64 = f64::powf(v1693, p.p21);
        self.scalar_v1696 = v1696;
        let v1698: f64 = (v1692 * v48);
        self.scalar_v1698 = v1698;
        let v1740: bool = (v1540 > v857);
        self.scalar_v1740 = v1740;
        let v1741: bool = (v861 && v1740);
        self.scalar_v1741 = v1741;
        let v1742: bool = (v1741 && v859);
        self.scalar_v1742 = v1742;
        let v1743: f64 = (v1540 * v79);
        self.scalar_v1743 = v1743;
        let v1744: f64 = (v1743 * v1743);
        self.scalar_v1744 = v1744;
        let v1745: f64 = (v1744 * v1743);
        self.scalar_v1745 = v1745;
        let v1746: f64 = (v1745 * v1743);
        self.scalar_v1746 = v1746;
        let v1748: bool = (v1741 && v869);
        self.scalar_v1748 = v1748;
        let v1749: f64 = v1743.abs();
        self.scalar_v1749 = v1749;
        let v1750: f64 = f64::powf(v1749, p.p53);
        self.scalar_v1750 = v1750;
        let v1755: bool = (!v1740);
        self.scalar_v1755 = v1755;
        let v1756: bool = (v861 && v1755);
        self.scalar_v1756 = v1756;
        let v1757: f64 = (v1540 + v879);
        self.scalar_v1757 = v1757;
        let v1758: f64 = (v1757 * v90);
        self.scalar_v1758 = v1758;
        let v1759: f64 = (v69 + v1758);
        self.scalar_v1759 = v1759;
        let v1917: f64 = (p.p19 - v1532);
        self.scalar_v1917 = v1917;
        let v1918: f64 = (v1917 * v54);
        self.scalar_v1918 = v1918;
        let v1919: f64 = v1918.sqrt();
        self.scalar_v1919 = v1919;
        let v1921: f64 = f64::powf(v1918, p.p22);
        self.scalar_v1921 = v1921;
        let v1923: f64 = (v1917 * v49);
        self.scalar_v1923 = v1923;
        let v1965: bool = (v1540 > v1126);
        self.scalar_v1965 = v1965;
        let v1966: bool = (v1130 && v1965);
        self.scalar_v1966 = v1966;
        let v1967: bool = (v1966 && v1128);
        self.scalar_v1967 = v1967;
        let v1968: f64 = (v1540 * v81);
        self.scalar_v1968 = v1968;
        let v1969: f64 = (v1968 * v1968);
        self.scalar_v1969 = v1969;
        let v1970: f64 = (v1969 * v1968);
        self.scalar_v1970 = v1970;
        let v1971: f64 = (v1970 * v1968);
        self.scalar_v1971 = v1971;
        let v1973: bool = (v1966 && v1138);
        self.scalar_v1973 = v1973;
        let v1974: f64 = v1968.abs();
        self.scalar_v1974 = v1974;
        let v1975: f64 = f64::powf(v1974, p.p54);
        self.scalar_v1975 = v1975;
        let v1980: bool = (!v1965);
        self.scalar_v1980 = v1980;
        let v1981: bool = (v1130 && v1980);
        self.scalar_v1981 = v1981;
        let v1982: f64 = (v1540 + v1148);
        self.scalar_v1982 = v1982;
        let v1983: f64 = (v1982 * v97);
        self.scalar_v1983 = v1983;
        let v1984: f64 = (v73 + v1983);
        self.scalar_v1984 = v1984;
        let v2142: f64 = (p.p20 - v1532);
        self.scalar_v2142 = v2142;
        let v2143: f64 = (v2142 * v56);
        self.scalar_v2143 = v2143;
        let v2144: f64 = v2143.sqrt();
        self.scalar_v2144 = v2144;
        let v2146: f64 = f64::powf(v2143, p.p23);
        self.scalar_v2146 = v2146;
        let v2148: f64 = (v2142 * v50);
        self.scalar_v2148 = v2148;
        let v2190: bool = (v1540 > v1394);
        self.scalar_v2190 = v2190;
        let v2191: bool = (v1398 && v2190);
        self.scalar_v2191 = v2191;
        let v2192: bool = (v2191 && v1396);
        self.scalar_v2192 = v2192;
        let v2193: f64 = (v1540 * v83);
        self.scalar_v2193 = v2193;
        let v2194: f64 = (v2193 * v2193);
        self.scalar_v2194 = v2194;
        let v2195: f64 = (v2194 * v2193);
        self.scalar_v2195 = v2195;
        let v2196: f64 = (v2195 * v2193);
        self.scalar_v2196 = v2196;
        let v2198: bool = (v2191 && v1406);
        self.scalar_v2198 = v2198;
        let v2199: f64 = v2193.abs();
        self.scalar_v2199 = v2199;
        let v2200: f64 = f64::powf(v2199, p.p55);
        self.scalar_v2200 = v2200;
        let v2205: bool = (!v2190);
        self.scalar_v2205 = v2205;
        let v2206: bool = (v1398 && v2205);
        self.scalar_v2206 = v2206;
        let v2207: f64 = (v1540 + v1416);
        self.scalar_v2207 = v2207;
        let v2208: f64 = (v2207 * v104);
        self.scalar_v2208 = v2208;
        let v2209: f64 = (v77 + v2208);
        self.scalar_v2209 = v2209;
        let v2223: f64 = (if v473 { 0.0 } else { v1532 });
        self.scalar_v2223 = v2223;
        let v2277: bool = (v490 > 0.0);
        self.scalar_v2277 = v2277;
        let v2278: bool = (v498 && v2277);
        self.scalar_v2278 = v2278;
        let v2289: bool = (!v2277);
        self.scalar_v2289 = v2289;
        let v2290: bool = (v498 && v2289);
        self.scalar_v2290 = v2290;
        let v2291: f64 = (-v490);
        self.scalar_v2291 = v2291;
        let v2315: f64 = (v490 + v472);
        self.scalar_v2315 = v2315;
        let v2316: f64 = (v490 - v472);
        self.scalar_v2316 = v2316;
        let v2317: f64 = (v2316 * v2316);
        self.scalar_v2317 = v2317;
        let v2318: f64 = (v2317 + v600);
        self.scalar_v2318 = v2318;
        let v2319: f64 = v2318.sqrt();
        self.scalar_v2319 = v2319;
        let v2320: f64 = (v2315 - v2319);
        self.scalar_v2320 = v2320;
        let v2321: f64 = (0.5 * v2320);
        self.scalar_v2321 = v2321;
        let v2322: f64 = (if v498 { v2321 } else { v2223 });
        self.scalar_v2322 = v2322;
        let v2323: f64 = (v490 + 0.0);
        self.scalar_v2323 = v2323;
        let v2324: f64 = (v490 - 0.0);
        self.scalar_v2324 = v2324;
        let v2325: f64 = (v2324 * v2324);
        self.scalar_v2325 = v2325;
        let v2326: f64 = (v2325 + 4e-12);
        self.scalar_v2326 = v2326;
        let v2327: f64 = v2326.sqrt();
        self.scalar_v2327 = v2327;
        let v2328: f64 = (v2323 - v2327);
        self.scalar_v2328 = v2328;
        let v2329: f64 = (0.5 * v2328);
        self.scalar_v2329 = v2329;
        let v2330: f64 = (if v498 { v2329 } else { v1540 });
        self.scalar_v2330 = v2330;
        let v2482: f64 = (p.p18 - v2322);
        self.scalar_v2482 = v2482;
        let v2483: f64 = (v2482 * v52);
        self.scalar_v2483 = v2483;
        let v2484: f64 = v2483.sqrt();
        self.scalar_v2484 = v2484;
        let v2486: f64 = f64::powf(v2483, p.p21);
        self.scalar_v2486 = v2486;
        let v2488: f64 = (v2482 * v48);
        self.scalar_v2488 = v2488;
        let v2530: bool = (v2330 > v857);
        self.scalar_v2530 = v2530;
        let v2531: bool = (v861 && v2530);
        self.scalar_v2531 = v2531;
        let v2532: bool = (v2531 && v859);
        self.scalar_v2532 = v2532;
        let v2533: f64 = (v2330 * v79);
        self.scalar_v2533 = v2533;
        let v2534: f64 = (v2533 * v2533);
        self.scalar_v2534 = v2534;
        let v2535: f64 = (v2534 * v2533);
        self.scalar_v2535 = v2535;
        let v2536: f64 = (v2535 * v2533);
        self.scalar_v2536 = v2536;
        let v2538: bool = (v2531 && v869);
        self.scalar_v2538 = v2538;
        let v2539: f64 = v2533.abs();
        self.scalar_v2539 = v2539;
        let v2540: f64 = f64::powf(v2539, p.p53);
        self.scalar_v2540 = v2540;
        let v2545: bool = (!v2530);
        self.scalar_v2545 = v2545;
        let v2546: bool = (v861 && v2545);
        self.scalar_v2546 = v2546;
        let v2547: f64 = (v2330 + v879);
        self.scalar_v2547 = v2547;
        let v2548: f64 = (v2547 * v90);
        self.scalar_v2548 = v2548;
        let v2549: f64 = (v69 + v2548);
        self.scalar_v2549 = v2549;
        let v2707: f64 = (p.p19 - v2322);
        self.scalar_v2707 = v2707;
        let v2708: f64 = (v2707 * v54);
        self.scalar_v2708 = v2708;
        let v2709: f64 = v2708.sqrt();
        self.scalar_v2709 = v2709;
        let v2711: f64 = f64::powf(v2708, p.p22);
        self.scalar_v2711 = v2711;
        let v2713: f64 = (v2707 * v49);
        self.scalar_v2713 = v2713;
        let v2755: bool = (v2330 > v1126);
        self.scalar_v2755 = v2755;
        let v2756: bool = (v1130 && v2755);
        self.scalar_v2756 = v2756;
        let v2757: bool = (v2756 && v1128);
        self.scalar_v2757 = v2757;
        let v2758: f64 = (v2330 * v81);
        self.scalar_v2758 = v2758;
        let v2759: f64 = (v2758 * v2758);
        self.scalar_v2759 = v2759;
        let v2760: f64 = (v2759 * v2758);
        self.scalar_v2760 = v2760;
        let v2761: f64 = (v2760 * v2758);
        self.scalar_v2761 = v2761;
        let v2763: bool = (v2756 && v1138);
        self.scalar_v2763 = v2763;
        let v2764: f64 = v2758.abs();
        self.scalar_v2764 = v2764;
        let v2765: f64 = f64::powf(v2764, p.p54);
        self.scalar_v2765 = v2765;
        let v2770: bool = (!v2755);
        self.scalar_v2770 = v2770;
        let v2771: bool = (v1130 && v2770);
        self.scalar_v2771 = v2771;
        let v2772: f64 = (v2330 + v1148);
        self.scalar_v2772 = v2772;
        let v2773: f64 = (v2772 * v97);
        self.scalar_v2773 = v2773;
        let v2774: f64 = (v73 + v2773);
        self.scalar_v2774 = v2774;
        let v2932: f64 = (p.p20 - v2322);
        self.scalar_v2932 = v2932;
        let v2933: f64 = (v2932 * v56);
        self.scalar_v2933 = v2933;
        let v2934: f64 = v2933.sqrt();
        self.scalar_v2934 = v2934;
        let v2936: f64 = f64::powf(v2933, p.p23);
        self.scalar_v2936 = v2936;
        let v2938: f64 = (v2932 * v50);
        self.scalar_v2938 = v2938;
        let v2980: bool = (v2330 > v1394);
        self.scalar_v2980 = v2980;
        let v2981: bool = (v1398 && v2980);
        self.scalar_v2981 = v2981;
        let v2982: bool = (v2981 && v1396);
        self.scalar_v2982 = v2982;
        let v2983: f64 = (v2330 * v83);
        self.scalar_v2983 = v2983;
        let v2984: f64 = (v2983 * v2983);
        self.scalar_v2984 = v2984;
        let v2985: f64 = (v2984 * v2983);
        self.scalar_v2985 = v2985;
        let v2986: f64 = (v2985 * v2983);
        self.scalar_v2986 = v2986;
        let v2988: bool = (v2981 && v1406);
        self.scalar_v2988 = v2988;
        let v2989: f64 = v2983.abs();
        self.scalar_v2989 = v2989;
        let v2990: f64 = f64::powf(v2989, p.p55);
        self.scalar_v2990 = v2990;
        let v2995: bool = (!v2980);
        self.scalar_v2995 = v2995;
        let v2996: bool = (v1398 && v2995);
        self.scalar_v2996 = v2996;
        let v2997: f64 = (v2330 + v1416);
        self.scalar_v2997 = v2997;
        let v2998: f64 = (v2997 * v104);
        self.scalar_v2998 = v2998;
        let v2999: f64 = (v77 + v2998);
        self.scalar_v2999 = v2999;
        let v3013: f64 = (if v473 { 0.0 } else { v2322 });
        self.scalar_v3013 = v3013;
        let v3067: bool = (v491 > 0.0);
        self.scalar_v3067 = v3067;
        let v3068: bool = (v498 && v3067);
        self.scalar_v3068 = v3068;
        let v3079: bool = (!v3067);
        self.scalar_v3079 = v3079;
        let v3080: bool = (v498 && v3079);
        self.scalar_v3080 = v3080;
        let v3081: f64 = (-v491);
        self.scalar_v3081 = v3081;
        let v3105: f64 = (v491 + v472);
        self.scalar_v3105 = v3105;
        let v3106: f64 = (v491 - v472);
        self.scalar_v3106 = v3106;
        let v3107: f64 = (v3106 * v3106);
        self.scalar_v3107 = v3107;
        let v3108: f64 = (v3107 + v600);
        self.scalar_v3108 = v3108;
        let v3109: f64 = v3108.sqrt();
        self.scalar_v3109 = v3109;
        let v3110: f64 = (v3105 - v3109);
        self.scalar_v3110 = v3110;
        let v3111: f64 = (0.5 * v3110);
        self.scalar_v3111 = v3111;
        let v3112: f64 = (if v498 { v3111 } else { v3013 });
        self.scalar_v3112 = v3112;
        let v3113: f64 = (v491 + 0.0);
        self.scalar_v3113 = v3113;
        let v3114: f64 = (v491 - 0.0);
        self.scalar_v3114 = v3114;
        let v3115: f64 = (v3114 * v3114);
        self.scalar_v3115 = v3115;
        let v3116: f64 = (v3115 + 4e-12);
        self.scalar_v3116 = v3116;
        let v3117: f64 = v3116.sqrt();
        self.scalar_v3117 = v3117;
        let v3118: f64 = (v3113 - v3117);
        self.scalar_v3118 = v3118;
        let v3119: f64 = (0.5 * v3118);
        self.scalar_v3119 = v3119;
        let v3120: f64 = (if v498 { v3119 } else { v2330 });
        self.scalar_v3120 = v3120;
        let v3272: f64 = (p.p18 - v3112);
        self.scalar_v3272 = v3272;
        let v3273: f64 = (v3272 * v52);
        self.scalar_v3273 = v3273;
        let v3274: f64 = v3273.sqrt();
        self.scalar_v3274 = v3274;
        let v3276: f64 = f64::powf(v3273, p.p21);
        self.scalar_v3276 = v3276;
        let v3278: f64 = (v3272 * v48);
        self.scalar_v3278 = v3278;
        let v3320: bool = (v3120 > v857);
        self.scalar_v3320 = v3320;
        let v3321: bool = (v861 && v3320);
        self.scalar_v3321 = v3321;
        let v3322: bool = (v3321 && v859);
        self.scalar_v3322 = v3322;
        let v3323: f64 = (v3120 * v79);
        self.scalar_v3323 = v3323;
        let v3324: f64 = (v3323 * v3323);
        self.scalar_v3324 = v3324;
        let v3325: f64 = (v3324 * v3323);
        self.scalar_v3325 = v3325;
        let v3326: f64 = (v3325 * v3323);
        self.scalar_v3326 = v3326;
        let v3328: bool = (v3321 && v869);
        self.scalar_v3328 = v3328;
        let v3329: f64 = v3323.abs();
        self.scalar_v3329 = v3329;
        let v3330: f64 = f64::powf(v3329, p.p53);
        self.scalar_v3330 = v3330;
        let v3335: bool = (!v3320);
        self.scalar_v3335 = v3335;
        let v3336: bool = (v861 && v3335);
        self.scalar_v3336 = v3336;
        let v3337: f64 = (v3120 + v879);
        self.scalar_v3337 = v3337;
        let v3338: f64 = (v3337 * v90);
        self.scalar_v3338 = v3338;
        let v3339: f64 = (v69 + v3338);
        self.scalar_v3339 = v3339;
        let v3497: f64 = (p.p19 - v3112);
        self.scalar_v3497 = v3497;
        let v3498: f64 = (v3497 * v54);
        self.scalar_v3498 = v3498;
        let v3499: f64 = v3498.sqrt();
        self.scalar_v3499 = v3499;
        let v3501: f64 = f64::powf(v3498, p.p22);
        self.scalar_v3501 = v3501;
        let v3503: f64 = (v3497 * v49);
        self.scalar_v3503 = v3503;
        let v3545: bool = (v3120 > v1126);
        self.scalar_v3545 = v3545;
        let v3546: bool = (v1130 && v3545);
        self.scalar_v3546 = v3546;
        let v3547: bool = (v3546 && v1128);
        self.scalar_v3547 = v3547;
        let v3548: f64 = (v3120 * v81);
        self.scalar_v3548 = v3548;
        let v3549: f64 = (v3548 * v3548);
        self.scalar_v3549 = v3549;
        let v3550: f64 = (v3549 * v3548);
        self.scalar_v3550 = v3550;
        let v3551: f64 = (v3550 * v3548);
        self.scalar_v3551 = v3551;
        let v3553: bool = (v3546 && v1138);
        self.scalar_v3553 = v3553;
        let v3554: f64 = v3548.abs();
        self.scalar_v3554 = v3554;
        let v3555: f64 = f64::powf(v3554, p.p54);
        self.scalar_v3555 = v3555;
        let v3560: bool = (!v3545);
        self.scalar_v3560 = v3560;
        let v3561: bool = (v1130 && v3560);
        self.scalar_v3561 = v3561;
        let v3562: f64 = (v3120 + v1148);
        self.scalar_v3562 = v3562;
        let v3563: f64 = (v3562 * v97);
        self.scalar_v3563 = v3563;
        let v3564: f64 = (v73 + v3563);
        self.scalar_v3564 = v3564;
        let v3722: f64 = (p.p20 - v3112);
        self.scalar_v3722 = v3722;
        let v3723: f64 = (v3722 * v56);
        self.scalar_v3723 = v3723;
        let v3724: f64 = v3723.sqrt();
        self.scalar_v3724 = v3724;
        let v3726: f64 = f64::powf(v3723, p.p23);
        self.scalar_v3726 = v3726;
        let v3728: f64 = (v3722 * v50);
        self.scalar_v3728 = v3728;
        let v3770: bool = (v3120 > v1394);
        self.scalar_v3770 = v3770;
        let v3771: bool = (v1398 && v3770);
        self.scalar_v3771 = v3771;
        let v3772: bool = (v3771 && v1396);
        self.scalar_v3772 = v3772;
        let v3773: f64 = (v3120 * v83);
        self.scalar_v3773 = v3773;
        let v3774: f64 = (v3773 * v3773);
        self.scalar_v3774 = v3774;
        let v3775: f64 = (v3774 * v3773);
        self.scalar_v3775 = v3775;
        let v3776: f64 = (v3775 * v3773);
        self.scalar_v3776 = v3776;
        let v3778: bool = (v3771 && v1406);
        self.scalar_v3778 = v3778;
        let v3779: f64 = v3773.abs();
        self.scalar_v3779 = v3779;
        let v3780: f64 = f64::powf(v3779, p.p55);
        self.scalar_v3780 = v3780;
        let v3785: bool = (!v3770);
        self.scalar_v3785 = v3785;
        let v3786: bool = (v1398 && v3785);
        self.scalar_v3786 = v3786;
        let v3787: f64 = (v3120 + v1416);
        self.scalar_v3787 = v3787;
        let v3788: f64 = (v3787 * v104);
        self.scalar_v3788 = v3788;
        let v3789: f64 = (v77 + v3788);
        self.scalar_v3789 = v3789;
        let v3803: f64 = (if v473 { 0.0 } else { v3112 });
        self.scalar_v3803 = v3803;
        let v3857: bool = (v493 > 0.0);
        self.scalar_v3857 = v3857;
        let v3858: bool = (v498 && v3857);
        self.scalar_v3858 = v3858;
        let v3869: bool = (!v3857);
        self.scalar_v3869 = v3869;
        let v3870: bool = (v498 && v3869);
        self.scalar_v3870 = v3870;
        let v3871: f64 = (-v493);
        self.scalar_v3871 = v3871;
        let v3895: f64 = (v493 + v472);
        self.scalar_v3895 = v3895;
        let v3896: f64 = (v493 - v472);
        self.scalar_v3896 = v3896;
        let v3897: f64 = (v3896 * v3896);
        self.scalar_v3897 = v3897;
        let v3898: f64 = (v3897 + v600);
        self.scalar_v3898 = v3898;
        let v3899: f64 = v3898.sqrt();
        self.scalar_v3899 = v3899;
        let v3900: f64 = (v3895 - v3899);
        self.scalar_v3900 = v3900;
        let v3901: f64 = (0.5 * v3900);
        self.scalar_v3901 = v3901;
        let v3902: f64 = (if v498 { v3901 } else { v3803 });
        self.scalar_v3902 = v3902;
        let v3903: f64 = (v493 + 0.0);
        self.scalar_v3903 = v3903;
        let v3904: f64 = (v493 - 0.0);
        self.scalar_v3904 = v3904;
        let v3905: f64 = (v3904 * v3904);
        self.scalar_v3905 = v3905;
        let v3906: f64 = (v3905 + 4e-12);
        self.scalar_v3906 = v3906;
        let v3907: f64 = v3906.sqrt();
        self.scalar_v3907 = v3907;
        let v3908: f64 = (v3903 - v3907);
        self.scalar_v3908 = v3908;
        let v3909: f64 = (0.5 * v3908);
        self.scalar_v3909 = v3909;
        let v3910: f64 = (if v498 { v3909 } else { v3120 });
        self.scalar_v3910 = v3910;
        let v4062: f64 = (p.p18 - v3902);
        self.scalar_v4062 = v4062;
        let v4063: f64 = (v4062 * v52);
        self.scalar_v4063 = v4063;
        let v4064: f64 = v4063.sqrt();
        self.scalar_v4064 = v4064;
        let v4066: f64 = f64::powf(v4063, p.p21);
        self.scalar_v4066 = v4066;
        let v4068: f64 = (v4062 * v48);
        self.scalar_v4068 = v4068;
        let v4110: bool = (v3910 > v857);
        self.scalar_v4110 = v4110;
        let v4111: bool = (v861 && v4110);
        self.scalar_v4111 = v4111;
        let v4112: bool = (v4111 && v859);
        self.scalar_v4112 = v4112;
        let v4113: f64 = (v3910 * v79);
        self.scalar_v4113 = v4113;
        let v4114: f64 = (v4113 * v4113);
        self.scalar_v4114 = v4114;
        let v4115: f64 = (v4114 * v4113);
        self.scalar_v4115 = v4115;
        let v4116: f64 = (v4115 * v4113);
        self.scalar_v4116 = v4116;
        let v4118: bool = (v4111 && v869);
        self.scalar_v4118 = v4118;
        let v4119: f64 = v4113.abs();
        self.scalar_v4119 = v4119;
        let v4120: f64 = f64::powf(v4119, p.p53);
        self.scalar_v4120 = v4120;
        let v4125: bool = (!v4110);
        self.scalar_v4125 = v4125;
        let v4126: bool = (v861 && v4125);
        self.scalar_v4126 = v4126;
        let v4127: f64 = (v3910 + v879);
        self.scalar_v4127 = v4127;
        let v4128: f64 = (v4127 * v90);
        self.scalar_v4128 = v4128;
        let v4129: f64 = (v69 + v4128);
        self.scalar_v4129 = v4129;
        let v4287: f64 = (p.p19 - v3902);
        self.scalar_v4287 = v4287;
        let v4288: f64 = (v4287 * v54);
        self.scalar_v4288 = v4288;
        let v4289: f64 = v4288.sqrt();
        self.scalar_v4289 = v4289;
        let v4291: f64 = f64::powf(v4288, p.p22);
        self.scalar_v4291 = v4291;
        let v4293: f64 = (v4287 * v49);
        self.scalar_v4293 = v4293;
        let v4335: bool = (v3910 > v1126);
        self.scalar_v4335 = v4335;
        let v4336: bool = (v1130 && v4335);
        self.scalar_v4336 = v4336;
        let v4337: bool = (v4336 && v1128);
        self.scalar_v4337 = v4337;
        let v4338: f64 = (v3910 * v81);
        self.scalar_v4338 = v4338;
        let v4339: f64 = (v4338 * v4338);
        self.scalar_v4339 = v4339;
        let v4340: f64 = (v4339 * v4338);
        self.scalar_v4340 = v4340;
        let v4341: f64 = (v4340 * v4338);
        self.scalar_v4341 = v4341;
        let v4343: bool = (v4336 && v1138);
        self.scalar_v4343 = v4343;
        let v4344: f64 = v4338.abs();
        self.scalar_v4344 = v4344;
        let v4345: f64 = f64::powf(v4344, p.p54);
        self.scalar_v4345 = v4345;
        let v4350: bool = (!v4335);
        self.scalar_v4350 = v4350;
        let v4351: bool = (v1130 && v4350);
        self.scalar_v4351 = v4351;
        let v4352: f64 = (v3910 + v1148);
        self.scalar_v4352 = v4352;
        let v4353: f64 = (v4352 * v97);
        self.scalar_v4353 = v4353;
        let v4354: f64 = (v73 + v4353);
        self.scalar_v4354 = v4354;
        let v4512: f64 = (p.p20 - v3902);
        self.scalar_v4512 = v4512;
        let v4513: f64 = (v4512 * v56);
        self.scalar_v4513 = v4513;
        let v4514: f64 = v4513.sqrt();
        self.scalar_v4514 = v4514;
        let v4516: f64 = f64::powf(v4513, p.p23);
        self.scalar_v4516 = v4516;
        let v4518: f64 = (v4512 * v50);
        self.scalar_v4518 = v4518;
        let v4560: bool = (v3910 > v1394);
        self.scalar_v4560 = v4560;
        let v4561: bool = (v1398 && v4560);
        self.scalar_v4561 = v4561;
        let v4562: bool = (v4561 && v1396);
        self.scalar_v4562 = v4562;
        let v4563: f64 = (v3910 * v83);
        self.scalar_v4563 = v4563;
        let v4564: f64 = (v4563 * v4563);
        self.scalar_v4564 = v4564;
        let v4565: f64 = (v4564 * v4563);
        self.scalar_v4565 = v4565;
        let v4566: f64 = (v4565 * v4563);
        self.scalar_v4566 = v4566;
        let v4568: bool = (v4561 && v1406);
        self.scalar_v4568 = v4568;
        let v4569: f64 = v4563.abs();
        self.scalar_v4569 = v4569;
        let v4570: f64 = f64::powf(v4569, p.p55);
        self.scalar_v4570 = v4570;
        let v4575: bool = (!v4560);
        self.scalar_v4575 = v4575;
        let v4576: bool = (v1398 && v4575);
        self.scalar_v4576 = v4576;
        let v4577: f64 = (v3910 + v1416);
        self.scalar_v4577 = v4577;
        let v4578: f64 = (v4577 * v104);
        self.scalar_v4578 = v4578;
        let v4579: f64 = (v77 + v4578);
        self.scalar_v4579 = v4579;
        let v4630: f64 = (v491 - v493);
        self.scalar_v4630 = v4630;
        let v4694: f64 = (v484 - v487);
        self.scalar_v4694 = v4694;
        let v4697: f64 = (v487 - v484);
        self.scalar_v4697 = v4697;
        let v4698: f64 = (v487 / v4697);
        self.scalar_v4698 = v4698;
        let v4706: f64 = (v484 / v4694);
        self.scalar_v4706 = v4706;
        let v4723: f64 = (1.0 / v490);
        self.scalar_v4723 = v4723;
        let v4744: f64 = p.p64;
        self.scalar_v4744 = v4744;
        let v4761: f64 = (0.5 * p.p12);
        self.scalar_v4761 = v4761;
        let v4791: f64 = p.p1;
        self.scalar_v4791 = v4791;
        let v4880: bool = (v30 == 0.5);
        self.scalar_v4880 = v4880;
        let v4887: bool = (!v4880);
        self.scalar_v4887 = v4887;
        let v4898: bool = (v32 == 0.5);
        self.scalar_v4898 = v4898;
        let v4905: bool = (!v4898);
        self.scalar_v4905 = v4905;
        let v4915: bool = (v34 == 0.5);
        self.scalar_v4915 = v4915;
        let v4922: bool = (!v4915);
        self.scalar_v4922 = v4922;
        let v4931: bool = (!v473);
        self.scalar_v4931 = v4931;
        let v4932: f64 = (if v4931 { 0.0 } else { 0.0 });
        self.scalar_v4932 = v4932;
        let v4933: bool = (v4931 && v497);
        self.scalar_v4933 = v4933;
        let v5055: bool = (v4931 && v425);
        self.scalar_v5055 = v5055;
        let v5056: f64 = (if v5055 { 0.0 } else { 0.0 });
        self.scalar_v5056 = v5056;
        let v5058: bool = (v4931 && v618);
        self.scalar_v5058 = v5058;
        let v5061: bool = (v5058 && v626);
        self.scalar_v5061 = v5061;
        let v5062: f64 = (if v5061 { 0.0 } else { 0.0 });
        self.scalar_v5062 = v5062;
        let v5063: bool = (v5058 && v629);
        self.scalar_v5063 = v5063;
        let v5071: bool = (v5063 && v638);
        self.scalar_v5071 = v5071;
        let v5072: f64 = (if v5071 { 0.0 } else { 0.0 });
        self.scalar_v5072 = v5072;
        let v5073: bool = (v5063 && v641);
        self.scalar_v5073 = v5073;
        let v5098: bool = (v5058 && v625);
        self.scalar_v5098 = v5098;
        let v5099: f64 = (if v5098 { 0.0 } else { 0.0 });
        self.scalar_v5099 = v5099;
        let v5100: bool = (v5058 && v671);
        self.scalar_v5100 = v5100;
        let v5118: bool = (v5100 && v694);
        self.scalar_v5118 = v5118;
        let v5123: bool = (v5100 && v700);
        self.scalar_v5123 = v5123;
        let v5216: bool = (v5058 && v797);
        self.scalar_v5216 = v5216;
        let v5217: f64 = (if v5216 { 0.0 } else { 0.0 });
        self.scalar_v5217 = v5217;
        let v5218: bool = (v5058 && v800);
        self.scalar_v5218 = v5218;
        let v5219: bool = (v5218 && v638);
        self.scalar_v5219 = v5219;
        let v5224: bool = (v5218 && v641);
        self.scalar_v5224 = v5224;
        let v5268: bool = (v5058 && v853);
        self.scalar_v5268 = v5268;
        let v5269: f64 = (if v5268 { 1.0 } else { 0.0 });
        self.scalar_v5269 = v5269;
        let v5271: bool = (v5058 && v860);
        self.scalar_v5271 = v5271;
        let v5298: bool = (v5058 && v4880);
        self.scalar_v5298 = v5298;
        let v5303: bool = (v5058 && v4887);
        self.scalar_v5303 = v5303;
        let v5306: f64 = p.p11;
        self.scalar_v5306 = v5306;
        let v5314: bool = (v4931 && v435);
        self.scalar_v5314 = v5314;
        let v5315: f64 = (if v5314 { 0.0 } else { 0.0 });
        self.scalar_v5315 = v5315;
        let v5317: bool = (v4931 && v893);
        self.scalar_v5317 = v5317;
        let v5320: bool = (v5317 && v901);
        self.scalar_v5320 = v5320;
        let v5322: bool = (v5317 && v904);
        self.scalar_v5322 = v5322;
        let v5330: bool = (v5322 && v913);
        self.scalar_v5330 = v5330;
        let v5332: bool = (v5322 && v916);
        self.scalar_v5332 = v5332;
        let v5356: bool = (v5317 && v900);
        self.scalar_v5356 = v5356;
        let v5358: bool = (v5317 && v945);
        self.scalar_v5358 = v5358;
        let v5376: bool = (v5358 && v967);
        self.scalar_v5376 = v5376;
        let v5381: bool = (v5358 && v973);
        self.scalar_v5381 = v5381;
        let v5474: bool = (v5317 && v1068);
        self.scalar_v5474 = v5474;
        let v5476: bool = (v5317 && v1071);
        self.scalar_v5476 = v5476;
        let v5477: bool = (v5476 && v913);
        self.scalar_v5477 = v5477;
        let v5482: bool = (v5476 && v916);
        self.scalar_v5482 = v5482;
        let v5526: bool = (v5317 && v1123);
        self.scalar_v5526 = v5526;
        let v5529: bool = (v5317 && v1129);
        self.scalar_v5529 = v5529;
        let v5556: bool = (v5317 && v4898);
        self.scalar_v5556 = v5556;
        let v5561: bool = (v5317 && v4905);
        self.scalar_v5561 = v5561;
        let v5570: bool = (v4931 && v444);
        self.scalar_v5570 = v5570;
        let v5571: f64 = (if v5570 { 0.0 } else { 0.0 });
        self.scalar_v5571 = v5571;
        let v5573: bool = (v4931 && v1161);
        self.scalar_v5573 = v5573;
        let v5576: bool = (v5573 && v1169);
        self.scalar_v5576 = v5576;
        let v5578: bool = (v5573 && v1172);
        self.scalar_v5578 = v5578;
        let v5586: bool = (v5578 && v1181);
        self.scalar_v5586 = v5586;
        let v5588: bool = (v5578 && v1184);
        self.scalar_v5588 = v5588;
        let v5612: bool = (v5573 && v1168);
        self.scalar_v5612 = v5612;
        let v5614: bool = (v5573 && v1213);
        self.scalar_v5614 = v5614;
        let v5632: bool = (v5614 && v1235);
        self.scalar_v5632 = v5632;
        let v5637: bool = (v5614 && v1241);
        self.scalar_v5637 = v5637;
        let v5730: bool = (v5573 && v1336);
        self.scalar_v5730 = v5730;
        let v5732: bool = (v5573 && v1339);
        self.scalar_v5732 = v5732;
        let v5733: bool = (v5732 && v1181);
        self.scalar_v5733 = v5733;
        let v5738: bool = (v5732 && v1184);
        self.scalar_v5738 = v5738;
        let v5782: bool = (v5573 && v1391);
        self.scalar_v5782 = v5782;
        let v5785: bool = (v5573 && v1397);
        self.scalar_v5785 = v5785;
        let v5812: bool = (v5573 && v119);
        self.scalar_v5812 = v5812;
        let v5813: f64 = p.p60;
        self.scalar_v5813 = v5813;
        let v5816: f64 = p.p61;
        self.scalar_v5816 = v5816;
        let v5856: bool = (v5812 && v4915);
        self.scalar_v5856 = v5856;
        let v5861: bool = (v5812 && v4922);
        self.scalar_v5861 = v5861;
        let v5892: bool = (v142 == 0.5);
        self.scalar_v5892 = v5892;
        let v5893: bool = (v5812 && v5892);
        self.scalar_v5893 = v5893;
        let v5898: bool = (!v5892);
        self.scalar_v5898 = v5898;
        let v5899: bool = (v5812 && v5898);
        self.scalar_v5899 = v5899;
        let v5911: bool = (!v119);
        self.scalar_v5911 = v5911;
        let v5912: bool = (v5573 && v5911);
        self.scalar_v5912 = v5912;
        let v5913: bool = (v5912 && v4915);
        self.scalar_v5913 = v5913;
        let v5918: bool = (v5912 && v4922);
        self.scalar_v5918 = v5918;
        let v5938: f64 = (p.p1 * v356);
        self.scalar_v5938 = v5938;
        let v5939: f64 = p.p8;
        self.scalar_v5939 = v5939;
        let v5940: f64 = (v5938 * p.p8);
        self.scalar_v5940 = v5940;
        let v5941: f64 = p.p7;
        self.scalar_v5941 = v5941;
        let v5942: f64 = (v5938 * p.p7);
        self.scalar_v5942 = v5942;
        let v5945: f64 = (p.p1 * -1.0);
        self.scalar_v5945 = v5945;
        let v6010: f64 = (-p.p1);
        self.scalar_v6010 = v6010;
        let v6011: f64 = (-v5945);
        self.scalar_v6011 = v6011;
        let v6047: f64 = (if v473 { p.p1 } else { 0.0 });
        self.scalar_v6047 = v6047;
        let v6048: f64 = (if v473 { v5945 } else { 0.0 });
        self.scalar_v6048 = v6048;
        let v6049: f64 = (if v473 { v6047 } else { 0.0 });
        self.scalar_v6049 = v6049;
        let v6050: f64 = (if v473 { v6048 } else { 0.0 });
        self.scalar_v6050 = v6050;
        let v6051: f64 = (-v6047);
        self.scalar_v6051 = v6051;
        let v6052: f64 = (-v6048);
        self.scalar_v6052 = v6052;
        let v6053: f64 = (if v473 { v6051 } else { 0.0 });
        self.scalar_v6053 = v6053;
        let v6054: f64 = (if v473 { v6052 } else { 0.0 });
        self.scalar_v6054 = v6054;
        let v6092: f64 = (v30 - 1.0);
        self.scalar_v6092 = v6092;
        let v6120: f64 = (v32 - 1.0);
        self.scalar_v6120 = v6120;
        let v6146: f64 = (v34 - 1.0);
        self.scalar_v6146 = v6146;
        let v6163: f64 = (if v4933 { p.p1 } else { v6047 });
        self.scalar_v6163 = v6163;
        let v6164: f64 = (if v4933 { v5945 } else { v6048 });
        self.scalar_v6164 = v6164;
        let v6165: f64 = (if v4933 { v6163 } else { v6049 });
        self.scalar_v6165 = v6165;
        let v6166: f64 = (if v4933 { v6164 } else { v6050 });
        self.scalar_v6166 = v6166;
        let v6167: f64 = (-v6163);
        self.scalar_v6167 = v6167;
        let v6168: f64 = (-v6164);
        self.scalar_v6168 = v6168;
        let v6169: f64 = (if v4933 { v6167 } else { v6053 });
        self.scalar_v6169 = v6169;
        let v6170: f64 = (if v4933 { v6168 } else { v6054 });
        self.scalar_v6170 = v6170;
        let v6450: f64 = (p.p21 - 1.0);
        self.scalar_v6450 = v6450;
        let v6558: f64 = (v693 - 1.0);
        self.scalar_v6558 = v6558;
        let v7061: f64 = (p.p22 - 1.0);
        self.scalar_v7061 = v7061;
        let v7171: f64 = (v966 - 1.0);
        self.scalar_v7171 = v7171;
        let v7674: f64 = (p.p23 - 1.0);
        self.scalar_v7674 = v7674;
        let v7784: f64 = (v1234 - 1.0);
        self.scalar_v7784 = v7784;
        let v8188: f64 = (p.p1 / p.p61);
        self.scalar_v8188 = v8188;
        let v8189: f64 = (v5945 / p.p61);
        self.scalar_v8189 = v8189;
        let v8198: f64 = (v6010 / p.p61);
        self.scalar_v8198 = v8198;
        let v8199: f64 = (v6011 / p.p61);
        self.scalar_v8199 = v8199;
        let v8328: f64 = (v142 - 1.0);
        self.scalar_v8328 = v8328;
    }
}
