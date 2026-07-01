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
            params.p0 = 1.0;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 1e21;
            params.p4 = 1e21;
            params.p5 = 1e-5;
            params.p6 = 1e-5;
            params.p7 = 1.0;
            params.p8 = 1.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.002;
            params.p14 = 3e-7;
            params.p15 = 0.5;
            params.p16 = 0.001;
            params.p17 = 0.7;
            params.p18 = 0.5;
            params.p19 = 0.00015;
            params.p20 = -1.5;
            params.p21 = 0.0;
            params.p22 = 100000000.0;
            params.p23 = 2000000.0;
            params.p24 = 0.8;
            params.p25 = 0.8;
            params.p26 = -1e-8;
            params.p27 = -1e-8;
            params.p28 = 0.2;
            params.p29 = 0.3;
            params.p30 = 0.00023;
            params.p31 = 4e-7;
            params.p32 = 500000000.0;
            params.p33 = 400000000.0;
            params.p34 = 0.0009;
            params.p35 = 1.0;
            params.p36 = 0.0;
            params.p37 = 5e-7;
            params.p38 = 1e-6;
            params.p39 = 1e-6;
            params.p40 = 1e-6;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 1.0;
            params.p44 = 1e-9;
            params.p45 = 1e-12;
            params.p46 = 1e-12;
            params.p47 = 0.9;
            params.p48 = 0.7;
            params.p49 = 0.7;
            params.p50 = 0.8;
            params.p51 = 0.6;
            params.p52 = 0.6;
            params.p53 = 1e-9;
            params.p54 = 1e-12;
            params.p55 = 1e-12;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 10.0;
            params.p59 = 1.0;
            params.p60 = 1.0;
            params.p61 = 1.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 3.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
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
    pub nodes: [usize; 4],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 75]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 5]>,
    pub(crate) ddt_state_previous: Box<[f64; 5]>,
    pub(crate) ddt_state_older: Box<[f64; 5]>,
    pub(crate) ddt_state_initialized: Box<[bool; 5]>,
    pub(crate) ddt_derivative_current: Box<[f64; 5]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 5]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v10: f64,
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
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: bool,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v30: bool,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v35: bool,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: bool,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: bool,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v131: bool,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v137: bool,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: bool,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: bool,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v160: bool,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: bool,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v512: bool,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v530: bool,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v739: f64,
    pub(crate) scalar_v740: f64,
    pub(crate) scalar_v741: f64,
    pub(crate) scalar_v742: f64,
    pub(crate) scalar_v743: f64,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v828: f64,
    pub(crate) scalar_v829: f64,
    pub(crate) scalar_v830: bool,
    pub(crate) scalar_v831: bool,
    pub(crate) scalar_v832: bool,
    pub(crate) scalar_v833: f64,
    pub(crate) scalar_v834: f64,
    pub(crate) scalar_v835: f64,
    pub(crate) scalar_v836: bool,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v838: f64,
    pub(crate) scalar_v839: bool,
    pub(crate) scalar_v840: bool,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v842: f64,
    pub(crate) scalar_v843: f64,
    pub(crate) scalar_v844: bool,
    pub(crate) scalar_v845: f64,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v847: bool,
    pub(crate) scalar_v848: bool,
    pub(crate) scalar_v849: f64,
    pub(crate) scalar_v850: bool,
    pub(crate) scalar_v851: f64,
    pub(crate) scalar_v852: f64,
    pub(crate) scalar_v853: bool,
    pub(crate) scalar_v854: bool,
    pub(crate) scalar_v855: f64,
    pub(crate) scalar_v856: bool,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v858: f64,
    pub(crate) scalar_v859: f64,
    pub(crate) scalar_v862: f64,
    pub(crate) scalar_v865: f64,
    pub(crate) scalar_v868: f64,
    pub(crate) scalar_v870: f64,
    pub(crate) scalar_v872: f64,
    pub(crate) scalar_v874: f64,
    pub(crate) scalar_v875: f64,
    pub(crate) scalar_v878: f64,
    pub(crate) scalar_v879: f64,
    pub(crate) scalar_v882: f64,
    pub(crate) scalar_v883: f64,
    pub(crate) scalar_v886: f64,
    pub(crate) scalar_v887: f64,
    pub(crate) scalar_v891: f64,
    pub(crate) scalar_v892: f64,
    pub(crate) scalar_v896: f64,
    pub(crate) scalar_v897: f64,
    pub(crate) scalar_v901: f64,
    pub(crate) scalar_v903: f64,
    pub(crate) scalar_v907: f64,
    pub(crate) scalar_v908: f64,
    pub(crate) scalar_v912: f64,
    pub(crate) scalar_v913: f64,
    pub(crate) scalar_v929: f64,
    pub(crate) scalar_v937: f64,
    pub(crate) scalar_v943: f64,
    pub(crate) scalar_v948: f64,
    pub(crate) scalar_v960: f64,
    pub(crate) scalar_v972: f64,
    pub(crate) scalar_v1034: f64,
    pub(crate) scalar_v1035: f64,
    pub(crate) scalar_v1044: f64,
    pub(crate) scalar_v1045: f64,
    pub(crate) scalar_v1054: f64,
    pub(crate) scalar_v1055: f64,
    pub(crate) scalar_v1139: f64,
    pub(crate) scalar_v1144: f64,
    pub(crate) scalar_v1165: f64,
    pub(crate) scalar_v4361: f64,
    pub(crate) scalar_v4362: f64,
    pub(crate) scalar_v4371: f64,
    pub(crate) scalar_v4372: f64,
    pub(crate) scalar_v4381: f64,
    pub(crate) scalar_v4382: f64,
    pub(crate) scalar_v4511: f64,
    pub(crate) scalar_v4512: f64,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v579: f64,
    pub(crate) scalar_v753: bool,
    pub(crate) scalar_v757: f64,
    pub(crate) scalar_v860: f64,
    pub(crate) scalar_v861: f64,
    pub(crate) scalar_v863: f64,
    pub(crate) scalar_v864: f64,
    pub(crate) scalar_v866: f64,
    pub(crate) scalar_v867: f64,
    pub(crate) scalar_v869: f64,
    pub(crate) scalar_v871: f64,
    pub(crate) scalar_v873: f64,
    pub(crate) scalar_v876: f64,
    pub(crate) scalar_v877: f64,
    pub(crate) scalar_v880: f64,
    pub(crate) scalar_v881: f64,
    pub(crate) scalar_v884: f64,
    pub(crate) scalar_v885: f64,
    pub(crate) scalar_v888: f64,
    pub(crate) scalar_v889: f64,
    pub(crate) scalar_v890: f64,
    pub(crate) scalar_v893: f64,
    pub(crate) scalar_v894: f64,
    pub(crate) scalar_v895: f64,
    pub(crate) scalar_v898: f64,
    pub(crate) scalar_v899: f64,
    pub(crate) scalar_v900: f64,
    pub(crate) scalar_v902: f64,
    pub(crate) scalar_v904: f64,
    pub(crate) scalar_v905: f64,
    pub(crate) scalar_v906: f64,
    pub(crate) scalar_v909: f64,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v911: f64,
    pub(crate) scalar_v914: f64,
    pub(crate) scalar_v915: f64,
    pub(crate) scalar_v916: f64,
    pub(crate) scalar_v917: f64,
    pub(crate) scalar_v918: f64,
    pub(crate) scalar_v919: f64,
    pub(crate) scalar_v920: f64,
    pub(crate) scalar_v921: f64,
    pub(crate) scalar_v924: f64,
    pub(crate) scalar_v944: f64,
    pub(crate) scalar_v946: f64,
    pub(crate) scalar_v958: f64,
    pub(crate) scalar_v970: f64,
    pub(crate) scalar_v982: f64,
    pub(crate) scalar_v983: f64,
    pub(crate) scalar_v984: f64,
    pub(crate) scalar_v985: f64,
    pub(crate) scalar_v1033: f64,
    pub(crate) scalar_v1043: f64,
    pub(crate) scalar_v1053: f64,
    pub(crate) scalar_v1083: f64,
    pub(crate) scalar_v1091: f64,
    pub(crate) scalar_v4193: f64,
    pub(crate) scalar_v4194: f64,
    pub(crate) scalar_v4195: f64,
    pub(crate) scalar_v4196: f64,
    pub(crate) scalar_v4199: f64,
    pub(crate) scalar_v4200: f64,
    pub(crate) scalar_v4207: f64,
    pub(crate) scalar_v4208: f64,
    pub(crate) scalar_v4209: f64,
    pub(crate) scalar_v4210: f64,
    pub(crate) scalar_v4226: f64,
    pub(crate) scalar_v4227: f64,
    pub(crate) scalar_v4228: f64,
    pub(crate) scalar_v4229: f64,
    pub(crate) scalar_v4247: f64,
    pub(crate) scalar_v4248: f64,
    pub(crate) scalar_v4249: f64,
    pub(crate) scalar_v4250: f64,
    pub(crate) scalar_v4325: f64,
    pub(crate) scalar_v4326: f64,
    pub(crate) scalar_v4337: f64,
    pub(crate) scalar_v4338: f64,
    pub(crate) scalar_v4349: f64,
    pub(crate) scalar_v4350: f64,
    pub(crate) scalar_v4363: f64,
    pub(crate) scalar_v4364: f64,
    pub(crate) scalar_v4365: f64,
    pub(crate) scalar_v4366: f64,
    pub(crate) scalar_v4367: f64,
    pub(crate) scalar_v4368: f64,
    pub(crate) scalar_v4373: f64,
    pub(crate) scalar_v4374: f64,
    pub(crate) scalar_v4375: f64,
    pub(crate) scalar_v4376: f64,
    pub(crate) scalar_v4377: f64,
    pub(crate) scalar_v4378: f64,
    pub(crate) scalar_v4383: f64,
    pub(crate) scalar_v4384: f64,
    pub(crate) scalar_v4385: f64,
    pub(crate) scalar_v4386: f64,
    pub(crate) scalar_v4387: f64,
    pub(crate) scalar_v4388: f64,
    pub(crate) scalar_v4431: f64,
    pub(crate) scalar_v4432: f64,
    pub(crate) scalar_v4435: f64,
    pub(crate) scalar_v4436: f64,
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
            scalar_v2: self.scalar_v2,
            scalar_v3: self.scalar_v3,
            scalar_v4: self.scalar_v4,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v10: self.scalar_v10,
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
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v30: self.scalar_v30,
            scalar_v32: self.scalar_v32,
            scalar_v35: self.scalar_v35,
            scalar_v37: self.scalar_v37,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v45: self.scalar_v45,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v90: self.scalar_v90,
            scalar_v111: self.scalar_v111,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
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
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v233: self.scalar_v233,
            scalar_v234: self.scalar_v234,
            scalar_v238: self.scalar_v238,
            scalar_v424: self.scalar_v424,
            scalar_v426: self.scalar_v426,
            scalar_v508: self.scalar_v508,
            scalar_v512: self.scalar_v512,
            scalar_v520: self.scalar_v520,
            scalar_v530: self.scalar_v530,
            scalar_v559: self.scalar_v559,
            scalar_v721: self.scalar_v721,
            scalar_v739: self.scalar_v739,
            scalar_v740: self.scalar_v740,
            scalar_v741: self.scalar_v741,
            scalar_v742: self.scalar_v742,
            scalar_v743: self.scalar_v743,
            scalar_v773: self.scalar_v773,
            scalar_v828: self.scalar_v828,
            scalar_v829: self.scalar_v829,
            scalar_v830: self.scalar_v830,
            scalar_v831: self.scalar_v831,
            scalar_v832: self.scalar_v832,
            scalar_v833: self.scalar_v833,
            scalar_v834: self.scalar_v834,
            scalar_v835: self.scalar_v835,
            scalar_v836: self.scalar_v836,
            scalar_v837: self.scalar_v837,
            scalar_v838: self.scalar_v838,
            scalar_v839: self.scalar_v839,
            scalar_v840: self.scalar_v840,
            scalar_v841: self.scalar_v841,
            scalar_v842: self.scalar_v842,
            scalar_v843: self.scalar_v843,
            scalar_v844: self.scalar_v844,
            scalar_v845: self.scalar_v845,
            scalar_v846: self.scalar_v846,
            scalar_v847: self.scalar_v847,
            scalar_v848: self.scalar_v848,
            scalar_v849: self.scalar_v849,
            scalar_v850: self.scalar_v850,
            scalar_v851: self.scalar_v851,
            scalar_v852: self.scalar_v852,
            scalar_v853: self.scalar_v853,
            scalar_v854: self.scalar_v854,
            scalar_v855: self.scalar_v855,
            scalar_v856: self.scalar_v856,
            scalar_v857: self.scalar_v857,
            scalar_v858: self.scalar_v858,
            scalar_v859: self.scalar_v859,
            scalar_v862: self.scalar_v862,
            scalar_v865: self.scalar_v865,
            scalar_v868: self.scalar_v868,
            scalar_v870: self.scalar_v870,
            scalar_v872: self.scalar_v872,
            scalar_v874: self.scalar_v874,
            scalar_v875: self.scalar_v875,
            scalar_v878: self.scalar_v878,
            scalar_v879: self.scalar_v879,
            scalar_v882: self.scalar_v882,
            scalar_v883: self.scalar_v883,
            scalar_v886: self.scalar_v886,
            scalar_v887: self.scalar_v887,
            scalar_v891: self.scalar_v891,
            scalar_v892: self.scalar_v892,
            scalar_v896: self.scalar_v896,
            scalar_v897: self.scalar_v897,
            scalar_v901: self.scalar_v901,
            scalar_v903: self.scalar_v903,
            scalar_v907: self.scalar_v907,
            scalar_v908: self.scalar_v908,
            scalar_v912: self.scalar_v912,
            scalar_v913: self.scalar_v913,
            scalar_v929: self.scalar_v929,
            scalar_v937: self.scalar_v937,
            scalar_v943: self.scalar_v943,
            scalar_v948: self.scalar_v948,
            scalar_v960: self.scalar_v960,
            scalar_v972: self.scalar_v972,
            scalar_v1034: self.scalar_v1034,
            scalar_v1035: self.scalar_v1035,
            scalar_v1044: self.scalar_v1044,
            scalar_v1045: self.scalar_v1045,
            scalar_v1054: self.scalar_v1054,
            scalar_v1055: self.scalar_v1055,
            scalar_v1139: self.scalar_v1139,
            scalar_v1144: self.scalar_v1144,
            scalar_v1165: self.scalar_v1165,
            scalar_v4361: self.scalar_v4361,
            scalar_v4362: self.scalar_v4362,
            scalar_v4371: self.scalar_v4371,
            scalar_v4372: self.scalar_v4372,
            scalar_v4381: self.scalar_v4381,
            scalar_v4382: self.scalar_v4382,
            scalar_v4511: self.scalar_v4511,
            scalar_v4512: self.scalar_v4512,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v38: self.scalar_v38,
            scalar_v47: self.scalar_v47,
            scalar_v49: self.scalar_v49,
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v57: self.scalar_v57,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v71: self.scalar_v71,
            scalar_v72: self.scalar_v72,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v83: self.scalar_v83,
            scalar_v84: self.scalar_v84,
            scalar_v87: self.scalar_v87,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
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
            scalar_v112: self.scalar_v112,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v159: self.scalar_v159,
            scalar_v208: self.scalar_v208,
            scalar_v322: self.scalar_v322,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v578: self.scalar_v578,
            scalar_v579: self.scalar_v579,
            scalar_v753: self.scalar_v753,
            scalar_v757: self.scalar_v757,
            scalar_v860: self.scalar_v860,
            scalar_v861: self.scalar_v861,
            scalar_v863: self.scalar_v863,
            scalar_v864: self.scalar_v864,
            scalar_v866: self.scalar_v866,
            scalar_v867: self.scalar_v867,
            scalar_v869: self.scalar_v869,
            scalar_v871: self.scalar_v871,
            scalar_v873: self.scalar_v873,
            scalar_v876: self.scalar_v876,
            scalar_v877: self.scalar_v877,
            scalar_v880: self.scalar_v880,
            scalar_v881: self.scalar_v881,
            scalar_v884: self.scalar_v884,
            scalar_v885: self.scalar_v885,
            scalar_v888: self.scalar_v888,
            scalar_v889: self.scalar_v889,
            scalar_v890: self.scalar_v890,
            scalar_v893: self.scalar_v893,
            scalar_v894: self.scalar_v894,
            scalar_v895: self.scalar_v895,
            scalar_v898: self.scalar_v898,
            scalar_v899: self.scalar_v899,
            scalar_v900: self.scalar_v900,
            scalar_v902: self.scalar_v902,
            scalar_v904: self.scalar_v904,
            scalar_v905: self.scalar_v905,
            scalar_v906: self.scalar_v906,
            scalar_v909: self.scalar_v909,
            scalar_v910: self.scalar_v910,
            scalar_v911: self.scalar_v911,
            scalar_v914: self.scalar_v914,
            scalar_v915: self.scalar_v915,
            scalar_v916: self.scalar_v916,
            scalar_v917: self.scalar_v917,
            scalar_v918: self.scalar_v918,
            scalar_v919: self.scalar_v919,
            scalar_v920: self.scalar_v920,
            scalar_v921: self.scalar_v921,
            scalar_v924: self.scalar_v924,
            scalar_v944: self.scalar_v944,
            scalar_v946: self.scalar_v946,
            scalar_v958: self.scalar_v958,
            scalar_v970: self.scalar_v970,
            scalar_v982: self.scalar_v982,
            scalar_v983: self.scalar_v983,
            scalar_v984: self.scalar_v984,
            scalar_v985: self.scalar_v985,
            scalar_v1033: self.scalar_v1033,
            scalar_v1043: self.scalar_v1043,
            scalar_v1053: self.scalar_v1053,
            scalar_v1083: self.scalar_v1083,
            scalar_v1091: self.scalar_v1091,
            scalar_v4193: self.scalar_v4193,
            scalar_v4194: self.scalar_v4194,
            scalar_v4195: self.scalar_v4195,
            scalar_v4196: self.scalar_v4196,
            scalar_v4199: self.scalar_v4199,
            scalar_v4200: self.scalar_v4200,
            scalar_v4207: self.scalar_v4207,
            scalar_v4208: self.scalar_v4208,
            scalar_v4209: self.scalar_v4209,
            scalar_v4210: self.scalar_v4210,
            scalar_v4226: self.scalar_v4226,
            scalar_v4227: self.scalar_v4227,
            scalar_v4228: self.scalar_v4228,
            scalar_v4229: self.scalar_v4229,
            scalar_v4247: self.scalar_v4247,
            scalar_v4248: self.scalar_v4248,
            scalar_v4249: self.scalar_v4249,
            scalar_v4250: self.scalar_v4250,
            scalar_v4325: self.scalar_v4325,
            scalar_v4326: self.scalar_v4326,
            scalar_v4337: self.scalar_v4337,
            scalar_v4338: self.scalar_v4338,
            scalar_v4349: self.scalar_v4349,
            scalar_v4350: self.scalar_v4350,
            scalar_v4363: self.scalar_v4363,
            scalar_v4364: self.scalar_v4364,
            scalar_v4365: self.scalar_v4365,
            scalar_v4366: self.scalar_v4366,
            scalar_v4367: self.scalar_v4367,
            scalar_v4368: self.scalar_v4368,
            scalar_v4373: self.scalar_v4373,
            scalar_v4374: self.scalar_v4374,
            scalar_v4375: self.scalar_v4375,
            scalar_v4376: self.scalar_v4376,
            scalar_v4377: self.scalar_v4377,
            scalar_v4378: self.scalar_v4378,
            scalar_v4383: self.scalar_v4383,
            scalar_v4384: self.scalar_v4384,
            scalar_v4385: self.scalar_v4385,
            scalar_v4386: self.scalar_v4386,
            scalar_v4387: self.scalar_v4387,
            scalar_v4388: self.scalar_v4388,
            scalar_v4431: self.scalar_v4431,
            scalar_v4432: self.scalar_v4432,
            scalar_v4435: self.scalar_v4435,
            scalar_v4436: self.scalar_v4436,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 0;
    pub const NODE_COUNT: usize = 4;
    pub const INTERNAL_NODE_NAMES: [&str; 0] = [];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 75;
    pub const VARIABLE_COUNT: usize = 271;
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
            scalar_v3: 0.0,
            scalar_v4: 0.0,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v10: 0.0,
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
            scalar_v21: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v24: false,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v30: false,
            scalar_v32: 0.0,
            scalar_v35: false,
            scalar_v37: 0.0,
            scalar_v39: 0.0,
            scalar_v40: false,
            scalar_v42: 0.0,
            scalar_v43: false,
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v90: 0.0,
            scalar_v111: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v131: false,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v137: false,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v144: 0.0,
            scalar_v145: false,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v152: 0.0,
            scalar_v153: false,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v160: false,
            scalar_v161: 0.0,
            scalar_v162: false,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v166: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v238: 0.0,
            scalar_v424: 0.0,
            scalar_v426: 0.0,
            scalar_v508: 0.0,
            scalar_v512: false,
            scalar_v520: 0.0,
            scalar_v530: false,
            scalar_v559: 0.0,
            scalar_v721: 0.0,
            scalar_v739: 0.0,
            scalar_v740: 0.0,
            scalar_v741: 0.0,
            scalar_v742: 0.0,
            scalar_v743: 0.0,
            scalar_v773: 0.0,
            scalar_v828: 0.0,
            scalar_v829: 0.0,
            scalar_v830: false,
            scalar_v831: false,
            scalar_v832: false,
            scalar_v833: 0.0,
            scalar_v834: 0.0,
            scalar_v835: 0.0,
            scalar_v836: false,
            scalar_v837: 0.0,
            scalar_v838: 0.0,
            scalar_v839: false,
            scalar_v840: false,
            scalar_v841: 0.0,
            scalar_v842: 0.0,
            scalar_v843: 0.0,
            scalar_v844: false,
            scalar_v845: 0.0,
            scalar_v846: 0.0,
            scalar_v847: false,
            scalar_v848: false,
            scalar_v849: 0.0,
            scalar_v850: false,
            scalar_v851: 0.0,
            scalar_v852: 0.0,
            scalar_v853: false,
            scalar_v854: false,
            scalar_v855: 0.0,
            scalar_v856: false,
            scalar_v857: 0.0,
            scalar_v858: 0.0,
            scalar_v859: 0.0,
            scalar_v862: 0.0,
            scalar_v865: 0.0,
            scalar_v868: 0.0,
            scalar_v870: 0.0,
            scalar_v872: 0.0,
            scalar_v874: 0.0,
            scalar_v875: 0.0,
            scalar_v878: 0.0,
            scalar_v879: 0.0,
            scalar_v882: 0.0,
            scalar_v883: 0.0,
            scalar_v886: 0.0,
            scalar_v887: 0.0,
            scalar_v891: 0.0,
            scalar_v892: 0.0,
            scalar_v896: 0.0,
            scalar_v897: 0.0,
            scalar_v901: 0.0,
            scalar_v903: 0.0,
            scalar_v907: 0.0,
            scalar_v908: 0.0,
            scalar_v912: 0.0,
            scalar_v913: 0.0,
            scalar_v929: 0.0,
            scalar_v937: 0.0,
            scalar_v943: 0.0,
            scalar_v948: 0.0,
            scalar_v960: 0.0,
            scalar_v972: 0.0,
            scalar_v1034: 0.0,
            scalar_v1035: 0.0,
            scalar_v1044: 0.0,
            scalar_v1045: 0.0,
            scalar_v1054: 0.0,
            scalar_v1055: 0.0,
            scalar_v1139: 0.0,
            scalar_v1144: 0.0,
            scalar_v1165: 0.0,
            scalar_v4361: 0.0,
            scalar_v4362: 0.0,
            scalar_v4371: 0.0,
            scalar_v4372: 0.0,
            scalar_v4381: 0.0,
            scalar_v4382: 0.0,
            scalar_v4511: 0.0,
            scalar_v4512: 0.0,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v38: 0.0,
            scalar_v47: 0.0,
            scalar_v49: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v57: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v65: 0.0,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v112: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v122: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v159: 0.0,
            scalar_v208: 0.0,
            scalar_v322: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v578: 0.0,
            scalar_v579: 0.0,
            scalar_v753: false,
            scalar_v757: 0.0,
            scalar_v860: 0.0,
            scalar_v861: 0.0,
            scalar_v863: 0.0,
            scalar_v864: 0.0,
            scalar_v866: 0.0,
            scalar_v867: 0.0,
            scalar_v869: 0.0,
            scalar_v871: 0.0,
            scalar_v873: 0.0,
            scalar_v876: 0.0,
            scalar_v877: 0.0,
            scalar_v880: 0.0,
            scalar_v881: 0.0,
            scalar_v884: 0.0,
            scalar_v885: 0.0,
            scalar_v888: 0.0,
            scalar_v889: 0.0,
            scalar_v890: 0.0,
            scalar_v893: 0.0,
            scalar_v894: 0.0,
            scalar_v895: 0.0,
            scalar_v898: 0.0,
            scalar_v899: 0.0,
            scalar_v900: 0.0,
            scalar_v902: 0.0,
            scalar_v904: 0.0,
            scalar_v905: 0.0,
            scalar_v906: 0.0,
            scalar_v909: 0.0,
            scalar_v910: 0.0,
            scalar_v911: 0.0,
            scalar_v914: 0.0,
            scalar_v915: 0.0,
            scalar_v916: 0.0,
            scalar_v917: 0.0,
            scalar_v918: 0.0,
            scalar_v919: 0.0,
            scalar_v920: 0.0,
            scalar_v921: 0.0,
            scalar_v924: 0.0,
            scalar_v944: 0.0,
            scalar_v946: 0.0,
            scalar_v958: 0.0,
            scalar_v970: 0.0,
            scalar_v982: 0.0,
            scalar_v983: 0.0,
            scalar_v984: 0.0,
            scalar_v985: 0.0,
            scalar_v1033: 0.0,
            scalar_v1043: 0.0,
            scalar_v1053: 0.0,
            scalar_v1083: 0.0,
            scalar_v1091: 0.0,
            scalar_v4193: 0.0,
            scalar_v4194: 0.0,
            scalar_v4195: 0.0,
            scalar_v4196: 0.0,
            scalar_v4199: 0.0,
            scalar_v4200: 0.0,
            scalar_v4207: 0.0,
            scalar_v4208: 0.0,
            scalar_v4209: 0.0,
            scalar_v4210: 0.0,
            scalar_v4226: 0.0,
            scalar_v4227: 0.0,
            scalar_v4228: 0.0,
            scalar_v4229: 0.0,
            scalar_v4247: 0.0,
            scalar_v4248: 0.0,
            scalar_v4249: 0.0,
            scalar_v4250: 0.0,
            scalar_v4325: 0.0,
            scalar_v4326: 0.0,
            scalar_v4337: 0.0,
            scalar_v4338: 0.0,
            scalar_v4349: 0.0,
            scalar_v4350: 0.0,
            scalar_v4363: 0.0,
            scalar_v4364: 0.0,
            scalar_v4365: 0.0,
            scalar_v4366: 0.0,
            scalar_v4367: 0.0,
            scalar_v4368: 0.0,
            scalar_v4373: 0.0,
            scalar_v4374: 0.0,
            scalar_v4375: 0.0,
            scalar_v4376: 0.0,
            scalar_v4377: 0.0,
            scalar_v4378: 0.0,
            scalar_v4383: 0.0,
            scalar_v4384: 0.0,
            scalar_v4385: 0.0,
            scalar_v4386: 0.0,
            scalar_v4387: 0.0,
            scalar_v4388: 0.0,
            scalar_v4431: 0.0,
            scalar_v4432: 0.0,
            scalar_v4435: 0.0,
            scalar_v4436: 0.0,
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
            scalar_v2,
            scalar_v3,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
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
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v27,
            scalar_v28,
            scalar_v30,
            scalar_v32,
            scalar_v35,
            scalar_v37,
            scalar_v39,
            scalar_v40,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v73,
            scalar_v74,
            scalar_v77,
            scalar_v78,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v111,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
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
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v233,
            scalar_v234,
            scalar_v238,
            scalar_v424,
            scalar_v426,
            scalar_v508,
            scalar_v512,
            scalar_v520,
            scalar_v530,
            scalar_v559,
            scalar_v721,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v773,
            scalar_v828,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v832,
            scalar_v833,
            scalar_v834,
            scalar_v835,
            scalar_v836,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v844,
            scalar_v845,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v849,
            scalar_v850,
            scalar_v851,
            scalar_v852,
            scalar_v853,
            scalar_v854,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v862,
            scalar_v865,
            scalar_v868,
            scalar_v870,
            scalar_v872,
            scalar_v874,
            scalar_v875,
            scalar_v878,
            scalar_v879,
            scalar_v882,
            scalar_v883,
            scalar_v886,
            scalar_v887,
            scalar_v891,
            scalar_v892,
            scalar_v896,
            scalar_v897,
            scalar_v901,
            scalar_v903,
            scalar_v907,
            scalar_v908,
            scalar_v912,
            scalar_v913,
            scalar_v929,
            scalar_v937,
            scalar_v943,
            scalar_v948,
            scalar_v960,
            scalar_v972,
            scalar_v1034,
            scalar_v1035,
            scalar_v1044,
            scalar_v1045,
            scalar_v1054,
            scalar_v1055,
            scalar_v1139,
            scalar_v1144,
            scalar_v1165,
            scalar_v4361,
            scalar_v4362,
            scalar_v4371,
            scalar_v4372,
            scalar_v4381,
            scalar_v4382,
            scalar_v4511,
            scalar_v4512,
            scalar_v33,
            scalar_v34,
            scalar_v38,
            scalar_v47,
            scalar_v49,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v57,
            scalar_v60,
            scalar_v61,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v71,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v79,
            scalar_v80,
            scalar_v83,
            scalar_v84,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
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
            scalar_v112,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v159,
            scalar_v208,
            scalar_v322,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v578,
            scalar_v579,
            scalar_v753,
            scalar_v757,
            scalar_v860,
            scalar_v861,
            scalar_v863,
            scalar_v864,
            scalar_v866,
            scalar_v867,
            scalar_v869,
            scalar_v871,
            scalar_v873,
            scalar_v876,
            scalar_v877,
            scalar_v880,
            scalar_v881,
            scalar_v884,
            scalar_v885,
            scalar_v888,
            scalar_v889,
            scalar_v890,
            scalar_v893,
            scalar_v894,
            scalar_v895,
            scalar_v898,
            scalar_v899,
            scalar_v900,
            scalar_v902,
            scalar_v904,
            scalar_v905,
            scalar_v906,
            scalar_v909,
            scalar_v910,
            scalar_v911,
            scalar_v914,
            scalar_v915,
            scalar_v916,
            scalar_v917,
            scalar_v918,
            scalar_v919,
            scalar_v920,
            scalar_v921,
            scalar_v924,
            scalar_v944,
            scalar_v946,
            scalar_v958,
            scalar_v970,
            scalar_v982,
            scalar_v983,
            scalar_v984,
            scalar_v985,
            scalar_v1033,
            scalar_v1043,
            scalar_v1053,
            scalar_v1083,
            scalar_v1091,
            scalar_v4193,
            scalar_v4194,
            scalar_v4195,
            scalar_v4196,
            scalar_v4199,
            scalar_v4200,
            scalar_v4207,
            scalar_v4208,
            scalar_v4209,
            scalar_v4210,
            scalar_v4226,
            scalar_v4227,
            scalar_v4228,
            scalar_v4229,
            scalar_v4247,
            scalar_v4248,
            scalar_v4249,
            scalar_v4250,
            scalar_v4325,
            scalar_v4326,
            scalar_v4337,
            scalar_v4338,
            scalar_v4349,
            scalar_v4350,
            scalar_v4363,
            scalar_v4364,
            scalar_v4365,
            scalar_v4366,
            scalar_v4367,
            scalar_v4368,
            scalar_v4373,
            scalar_v4374,
            scalar_v4375,
            scalar_v4376,
            scalar_v4377,
            scalar_v4378,
            scalar_v4383,
            scalar_v4384,
            scalar_v4385,
            scalar_v4386,
            scalar_v4387,
            scalar_v4388,
            scalar_v4431,
            scalar_v4432,
            scalar_v4435,
            scalar_v4436,
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
            scalar_v2,
            scalar_v3,
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v10,
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
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v27,
            scalar_v28,
            scalar_v30,
            scalar_v32,
            scalar_v35,
            scalar_v37,
            scalar_v39,
            scalar_v40,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v73,
            scalar_v74,
            scalar_v77,
            scalar_v78,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v111,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v171,
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
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v233,
            scalar_v234,
            scalar_v238,
            scalar_v424,
            scalar_v426,
            scalar_v508,
            scalar_v512,
            scalar_v520,
            scalar_v530,
            scalar_v559,
            scalar_v721,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v773,
            scalar_v828,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v832,
            scalar_v833,
            scalar_v834,
            scalar_v835,
            scalar_v836,
            scalar_v837,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v844,
            scalar_v845,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v849,
            scalar_v850,
            scalar_v851,
            scalar_v852,
            scalar_v853,
            scalar_v854,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v862,
            scalar_v865,
            scalar_v868,
            scalar_v870,
            scalar_v872,
            scalar_v874,
            scalar_v875,
            scalar_v878,
            scalar_v879,
            scalar_v882,
            scalar_v883,
            scalar_v886,
            scalar_v887,
            scalar_v891,
            scalar_v892,
            scalar_v896,
            scalar_v897,
            scalar_v901,
            scalar_v903,
            scalar_v907,
            scalar_v908,
            scalar_v912,
            scalar_v913,
            scalar_v929,
            scalar_v937,
            scalar_v943,
            scalar_v948,
            scalar_v960,
            scalar_v972,
            scalar_v1034,
            scalar_v1035,
            scalar_v1044,
            scalar_v1045,
            scalar_v1054,
            scalar_v1055,
            scalar_v1139,
            scalar_v1144,
            scalar_v1165,
            scalar_v4361,
            scalar_v4362,
            scalar_v4371,
            scalar_v4372,
            scalar_v4381,
            scalar_v4382,
            scalar_v4511,
            scalar_v4512,
            scalar_v33,
            scalar_v34,
            scalar_v38,
            scalar_v47,
            scalar_v49,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v57,
            scalar_v60,
            scalar_v61,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v71,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v79,
            scalar_v80,
            scalar_v83,
            scalar_v84,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
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
            scalar_v112,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v159,
            scalar_v208,
            scalar_v322,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v578,
            scalar_v579,
            scalar_v753,
            scalar_v757,
            scalar_v860,
            scalar_v861,
            scalar_v863,
            scalar_v864,
            scalar_v866,
            scalar_v867,
            scalar_v869,
            scalar_v871,
            scalar_v873,
            scalar_v876,
            scalar_v877,
            scalar_v880,
            scalar_v881,
            scalar_v884,
            scalar_v885,
            scalar_v888,
            scalar_v889,
            scalar_v890,
            scalar_v893,
            scalar_v894,
            scalar_v895,
            scalar_v898,
            scalar_v899,
            scalar_v900,
            scalar_v902,
            scalar_v904,
            scalar_v905,
            scalar_v906,
            scalar_v909,
            scalar_v910,
            scalar_v911,
            scalar_v914,
            scalar_v915,
            scalar_v916,
            scalar_v917,
            scalar_v918,
            scalar_v919,
            scalar_v920,
            scalar_v921,
            scalar_v924,
            scalar_v944,
            scalar_v946,
            scalar_v958,
            scalar_v970,
            scalar_v982,
            scalar_v983,
            scalar_v984,
            scalar_v985,
            scalar_v1033,
            scalar_v1043,
            scalar_v1053,
            scalar_v1083,
            scalar_v1091,
            scalar_v4193,
            scalar_v4194,
            scalar_v4195,
            scalar_v4196,
            scalar_v4199,
            scalar_v4200,
            scalar_v4207,
            scalar_v4208,
            scalar_v4209,
            scalar_v4210,
            scalar_v4226,
            scalar_v4227,
            scalar_v4228,
            scalar_v4229,
            scalar_v4247,
            scalar_v4248,
            scalar_v4249,
            scalar_v4250,
            scalar_v4325,
            scalar_v4326,
            scalar_v4337,
            scalar_v4338,
            scalar_v4349,
            scalar_v4350,
            scalar_v4363,
            scalar_v4364,
            scalar_v4365,
            scalar_v4366,
            scalar_v4367,
            scalar_v4368,
            scalar_v4373,
            scalar_v4374,
            scalar_v4375,
            scalar_v4376,
            scalar_v4377,
            scalar_v4378,
            scalar_v4383,
            scalar_v4384,
            scalar_v4385,
            scalar_v4386,
            scalar_v4387,
            scalar_v4388,
            scalar_v4431,
            scalar_v4432,
            scalar_v4435,
            scalar_v4436,
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
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "temp" => { validate_parameter("TEMP", value, Some((273.15, "273.15")), false, None, false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "m" => { validate_parameter("M", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ns" => { validate_parameter("NS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cox" => { validate_parameter("COX", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xj" => { validate_parameter("XJ", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vto" => { validate_finite_parameter("VTO", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcv" => { validate_finite_parameter("TCV", value)?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamma" => { validate_parameter("GAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "phi" => { validate_parameter("PHI", value, Some((0.2, "0.2")), false, None, false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kp" => { validate_parameter("KP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bex" => { validate_finite_parameter("BEX", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "theta" => { validate_parameter("THETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "e0" => { validate_finite_parameter("E0", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ucrit" => { validate_parameter("UCRIT", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ucex" => { validate_finite_parameter("UCEX", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lambda" => { validate_parameter("LAMBDA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dl" => { validate_finite_parameter("DL", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dw" => { validate_finite_parameter("DW", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "weta" => { validate_parameter("WETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "leta" => { validate_parameter("LETA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "q0" => { validate_parameter("Q0", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lk" => { validate_parameter("LK", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iba" => { validate_parameter("IBA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibb" => { validate_parameter("IBB", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibbt" => { validate_finite_parameter("IBBT", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibn" => { validate_parameter("IBN", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hdif" => { validate_parameter("HDIF", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avto" => { validate_parameter("AVTO", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "akp" => { validate_parameter("AKP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agamma" => { validate_parameter("AGAMMA", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("AF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("KF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_n" => { validate_parameter("xd_n", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_js" => { validate_parameter("xd_js", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_jsw" => { validate_parameter("xd_jsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_jswg" => { validate_parameter("xd_jswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mj" => { validate_parameter("xd_mj", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mjsw" => { validate_parameter("xd_mjsw", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_mjswg" => { validate_parameter("xd_mjswg", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pb" => { validate_parameter("xd_pb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pbsw" => { validate_parameter("xd_pbsw", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_pbswg" => { validate_parameter("xd_pbswg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cj" => { validate_parameter("xd_cj", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cjsw" => { validate_parameter("xd_cjsw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_cjswg" => { validate_parameter("xd_cjswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_gmin" => { validate_parameter("xd_gmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_xjbv" => { validate_parameter("xd_xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_bv" => { validate_parameter("xd_bv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njts" => { validate_parameter("xd_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njtssw" => { validate_parameter("xd_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_njtsswg" => { validate_parameter("xd_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vts" => { validate_parameter("xd_vts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vtssw" => { validate_parameter("xd_vtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xd_vtsswg" => { validate_parameter("xd_vtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_xti" => { validate_finite_parameter("tp_xti", value)?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cj" => { validate_finite_parameter("tp_cj", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cjsw" => { validate_finite_parameter("tp_cjsw", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_cjswg" => { validate_finite_parameter("tp_cjswg", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pb" => { validate_finite_parameter("tp_pb", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pbsw" => { validate_finite_parameter("tp_pbsw", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_pbswg" => { validate_finite_parameter("tp_pbswg", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njts" => { validate_parameter("tp_njts", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njtssw" => { validate_parameter("tp_njtssw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tp_njtsswg" => { validate_parameter("tp_njtsswg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'ekv_va'", name)),
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
        let v2: f64 = p.p13;
        self.scalar_v2 = v2;
        let v3: f64 = (1.0359399871014713e-10 / p.p13);
        self.scalar_v3 = v3;
        let v4: f64 = p.p14;
        self.scalar_v4 = v4;
        let v5: f64 = (v3 * p.p14);
        self.scalar_v5 = v5;
        let v6: f64 = ((v5) as f64).sqrt();
        self.scalar_v6 = v6;
        let v7: f64 = p.p25;
        self.scalar_v7 = v7;
        let v8: f64 = (v6 * p.p25);
        self.scalar_v8 = v8;
        let v10: f64 = (v3 * 3.0);
        self.scalar_v10 = v10;
        let v11: f64 = p.p28;
        self.scalar_v11 = v11;
        let v12: f64 = (v10 * p.p28);
        self.scalar_v12 = v12;
        let v13: f64 = p.p29;
        self.scalar_v13 = v13;
        let v14: f64 = (v3 * p.p29);
        self.scalar_v14 = v14;
        let v15: f64 = p.p35;
        self.scalar_v15 = v15;
        let v16: f64 = (p.p35 + p.p35);
        self.scalar_v16 = v16;
        let v17: f64 = p.p22;
        self.scalar_v17 = v17;
        let v18: f64 = (1.0359399871014713e-10 * p.p22);
        self.scalar_v18 = v18;
        let v19: f64 = (p.p13 / v18);
        self.scalar_v19 = v19;
        let v20: f64 = p.p30;
        self.scalar_v20 = v20;
        let v21: f64 = (p.p30 + p.p30);
        self.scalar_v21 = v21;
        let v22: f64 = (v21 / p.p13);
        self.scalar_v22 = v22;
        let v23: f64 = p.p0;
        self.scalar_v23 = v23;
        let v24: bool = (p.p0 > 0.0);
        self.scalar_v24 = v24;
        let v27: f64 = (if v24 { 0.5 } else { 0.3333333333333 });
        self.scalar_v27 = v27;
        let v28: f64 = p.p3;
        self.scalar_v28 = v28;
        let v30: bool = (p.p3 == 1e21);
        self.scalar_v30 = v30;
        let v32: f64 = p.p2;
        self.scalar_v32 = v32;
        let v35: bool = (!v30);
        self.scalar_v35 = v35;
        let v37: f64 = (p.p3 + 273.15);
        self.scalar_v37 = v37;
        let v39: f64 = p.p4;
        self.scalar_v39 = v39;
        let v40: bool = (1e21 == p.p4);
        self.scalar_v40 = v40;
        let v42: f64 = (if v40 { 298.15 } else { 0.0 });
        self.scalar_v42 = v42;
        let v43: bool = (!v40);
        self.scalar_v43 = v43;
        let v44: f64 = (273.15 + p.p4);
        self.scalar_v44 = v44;
        let v45: f64 = (if v43 { v44 } else { v42 });
        self.scalar_v45 = v45;
        let v66: f64 = (v45 * 0.000702);
        self.scalar_v66 = v66;
        let v67: f64 = (v45 * v66);
        self.scalar_v67 = v67;
        let v68: f64 = (v45 + 1108.0);
        self.scalar_v68 = v68;
        let v69: f64 = (v67 / v68);
        self.scalar_v69 = v69;
        let v70: f64 = (1.16 - v69);
        self.scalar_v70 = v70;
        let v73: f64 = p.p15;
        self.scalar_v73 = v73;
        let v74: f64 = p.p16;
        self.scalar_v74 = v74;
        let v77: f64 = p.p19;
        self.scalar_v77 = v77;
        let v78: f64 = p.p20;
        self.scalar_v78 = v78;
        let v81: f64 = p.p23;
        self.scalar_v81 = v81;
        let v82: f64 = p.p24;
        self.scalar_v82 = v82;
        let v85: f64 = p.p33;
        self.scalar_v85 = v85;
        let v86: f64 = p.p34;
        self.scalar_v86 = v86;
        let v90: f64 = p.p18;
        self.scalar_v90 = v90;
        let v111: f64 = p.p32;
        self.scalar_v111 = v111;
        let v113: f64 = p.p5;
        self.scalar_v113 = v113;
        let v114: f64 = p.p26;
        self.scalar_v114 = v114;
        let v115: f64 = (p.p5 + p.p26);
        self.scalar_v115 = v115;
        let v116: f64 = p.p6;
        self.scalar_v116 = v116;
        let v117: f64 = p.p27;
        self.scalar_v117 = v117;
        let v118: f64 = (p.p6 + p.p27);
        self.scalar_v118 = v118;
        let v126: f64 = (v115 * v118);
        self.scalar_v126 = v126;
        let v127: f64 = ((v126) as f64).sqrt();
        self.scalar_v127 = v127;
        let v128: f64 = (1.0 / v127);
        self.scalar_v128 = v128;
        let v129: f64 = p.p38;
        self.scalar_v129 = v129;
        let v131: bool = (p.p38 != 1e-6);
        self.scalar_v131 = v131;
        let v132: f64 = (p.p38 - 1e-6);
        self.scalar_v132 = v132;
        let v133: f64 = (v128 * v132);
        self.scalar_v133 = v133;
        let v137: bool = (!v24);
        self.scalar_v137 = v137;
        let v138: f64 = (1e-6 - p.p38);
        self.scalar_v138 = v138;
        let v139: f64 = (v128 * v138);
        self.scalar_v139 = v139;
        let v144: f64 = p.p39;
        self.scalar_v144 = v144;
        let v145: bool = (1e-6 != p.p39);
        self.scalar_v145 = v145;
        let v146: f64 = (p.p39 - 1e-6);
        self.scalar_v146 = v146;
        let v147: f64 = (v128 * v146);
        self.scalar_v147 = v147;
        let v148: f64 = (1.0 + v147);
        self.scalar_v148 = v148;
        let v152: f64 = p.p40;
        self.scalar_v152 = v152;
        let v153: bool = (1e-6 != p.p40);
        self.scalar_v153 = v153;
        let v154: f64 = p.p17;
        self.scalar_v154 = v154;
        let v155: f64 = (p.p40 - 1e-6);
        self.scalar_v155 = v155;
        let v156: f64 = (v128 * v155);
        self.scalar_v156 = v156;
        let v157: f64 = (p.p17 + v156);
        self.scalar_v157 = v157;
        let v158: f64 = (if v153 { v157 } else { p.p17 });
        self.scalar_v158 = v158;
        let v160: bool = (0.0 == v22);
        self.scalar_v160 = v160;
        let v161: f64 = (if v160 { 0.0 } else { 0.0 });
        self.scalar_v161 = v161;
        let v162: bool = (!v160);
        self.scalar_v162 = v162;
        let v164: f64 = p.p31;
        self.scalar_v164 = v164;
        let v165: f64 = p.p8;
        self.scalar_v165 = v165;
        let v166: f64 = (p.p31 * p.p8);
        self.scalar_v166 = v166;
        let v167: f64 = (v115 / v166);
        self.scalar_v167 = v167;
        let v168: f64 = (v167 - 0.1);
        self.scalar_v168 = v168;
        let v169: f64 = (0.28 * v168);
        self.scalar_v169 = v169;
        let v170: f64 = (if v162 { v169 } else { 0.0 });
        self.scalar_v170 = v170;
        let v171: f64 = (v170 * v170);
        self.scalar_v171 = v171;
        let v173: f64 = (v171 + 0.001936);
        self.scalar_v173 = v173;
        let v174: f64 = ((v173) as f64).sqrt();
        self.scalar_v174 = v174;
        let v175: f64 = (v170 + v174);
        self.scalar_v175 = v175;
        let v176: f64 = (0.5 * v175);
        self.scalar_v176 = v176;
        let v177: f64 = (1.0 + v176);
        self.scalar_v177 = v177;
        let v178: f64 = (1.0 / v177);
        self.scalar_v178 = v178;
        let v179: f64 = (if v162 { v178 } else { 0.0 });
        self.scalar_v179 = v179;
        let v180: f64 = (v22 * v179);
        self.scalar_v180 = v180;
        let v181: f64 = (v179 * v180);
        self.scalar_v181 = v181;
        let v182: f64 = (if v162 { v181 } else { v161 });
        self.scalar_v182 = v182;
        let v227: f64 = p.p7;
        self.scalar_v227 = v227;
        let v228: f64 = (v12 * p.p7);
        self.scalar_v228 = v228;
        let v229: f64 = (v228 / v118);
        self.scalar_v229 = v229;
        let v230: f64 = (v14 * p.p8);
        self.scalar_v230 = v230;
        let v231: f64 = (v230 / v115);
        self.scalar_v231 = v231;
        let v233: f64 = (v158 * 0.25);
        self.scalar_v233 = v233;
        let v234: f64 = (v158 * v233);
        self.scalar_v234 = v234;
        let v238: f64 = (0.5 * v158);
        self.scalar_v238 = v238;
        let v424: f64 = (0.1 * v115);
        self.scalar_v424 = v424;
        let v426: f64 = (v424 * v424);
        self.scalar_v426 = v426;
        let v508: f64 = (v158 * -0.5);
        self.scalar_v508 = v508;
        let v512: bool = (0.0 == p.p22);
        self.scalar_v512 = v512;
        let v520: f64 = p.p21;
        self.scalar_v520 = v520;
        let v530: bool = (!v512);
        self.scalar_v530 = v530;
        let v559: f64 = (-v231);
        self.scalar_v559 = v559;
        let v721: f64 = (-v158);
        self.scalar_v721 = v721;
        let v739: f64 = p.p36;
        self.scalar_v739 = v739;
        let v740: f64 = p.p37;
        self.scalar_v740 = v740;
        let v741: f64 = (p.p36 * p.p37);
        self.scalar_v741 = v741;
        let v742: f64 = (v118 - p.p27);
        self.scalar_v742 = v742;
        let v743: f64 = (v741 / v742);
        self.scalar_v743 = v743;
        let v773: f64 = (p.p13 * v126);
        self.scalar_v773 = v773;
        let v828: f64 = p.p1;
        self.scalar_v828 = v828;
        let v829: f64 = p.p9;
        self.scalar_v829 = v829;
        let v830: bool = (0.0 == p.p9);
        self.scalar_v830 = v830;
        let v831: bool = (p.p37 > 0.0);
        self.scalar_v831 = v831;
        let v832: bool = (v830 && v831);
        self.scalar_v832 = v832;
        let v833: f64 = (2.0 * p.p37);
        self.scalar_v833 = v833;
        let v834: f64 = (v118 * v833);
        self.scalar_v834 = v834;
        let v835: f64 = (if v832 { v834 } else { 0.0 });
        self.scalar_v835 = v835;
        let v836: bool = (!v832);
        self.scalar_v836 = v836;
        let v837: f64 = (if v836 { p.p9 } else { v835 });
        self.scalar_v837 = v837;
        let v838: f64 = p.p11;
        self.scalar_v838 = v838;
        let v839: bool = (0.0 == p.p11);
        self.scalar_v839 = v839;
        let v840: bool = (v831 && v839);
        self.scalar_v840 = v840;
        let v841: f64 = (4.0 * p.p37);
        self.scalar_v841 = v841;
        let v842: f64 = (v118 + v841);
        self.scalar_v842 = v842;
        let v843: f64 = (if v840 { v842 } else { 0.0 });
        self.scalar_v843 = v843;
        let v844: bool = (!v840);
        self.scalar_v844 = v844;
        let v845: f64 = (if v844 { p.p11 } else { v843 });
        self.scalar_v845 = v845;
        let v846: f64 = p.p10;
        self.scalar_v846 = v846;
        let v847: bool = (0.0 == p.p10);
        self.scalar_v847 = v847;
        let v848: bool = (v831 && v847);
        self.scalar_v848 = v848;
        let v849: f64 = (if v848 { v834 } else { 0.0 });
        self.scalar_v849 = v849;
        let v850: bool = (!v848);
        self.scalar_v850 = v850;
        let v851: f64 = (if v850 { p.p10 } else { v849 });
        self.scalar_v851 = v851;
        let v852: f64 = p.p12;
        self.scalar_v852 = v852;
        let v853: bool = (0.0 == p.p12);
        self.scalar_v853 = v853;
        let v854: bool = (v831 && v853);
        self.scalar_v854 = v854;
        let v855: f64 = (if v854 { v842 } else { 0.0 });
        self.scalar_v855 = v855;
        let v856: bool = (!v854);
        self.scalar_v856 = v856;
        let v857: f64 = (if v856 { p.p12 } else { v855 });
        self.scalar_v857 = v857;
        let v858: f64 = (v45 * 8.617333262145179e-5);
        self.scalar_v858 = v858;
        let v859: f64 = (v70 / v858);
        self.scalar_v859 = v859;
        let v862: f64 = p.p65;
        self.scalar_v862 = v862;
        let v865: f64 = p.p43;
        self.scalar_v865 = v865;
        let v868: f64 = p.p44;
        self.scalar_v868 = v868;
        let v870: f64 = p.p45;
        self.scalar_v870 = v870;
        let v872: f64 = p.p46;
        self.scalar_v872 = v872;
        let v874: f64 = p.p50;
        self.scalar_v874 = v874;
        let v875: f64 = p.p69;
        self.scalar_v875 = v875;
        let v878: f64 = p.p51;
        self.scalar_v878 = v878;
        let v879: f64 = p.p70;
        self.scalar_v879 = v879;
        let v882: f64 = p.p52;
        self.scalar_v882 = v882;
        let v883: f64 = p.p71;
        self.scalar_v883 = v883;
        let v886: f64 = p.p53;
        self.scalar_v886 = v886;
        let v887: f64 = p.p66;
        self.scalar_v887 = v887;
        let v891: f64 = p.p54;
        self.scalar_v891 = v891;
        let v892: f64 = p.p67;
        self.scalar_v892 = v892;
        let v896: f64 = p.p55;
        self.scalar_v896 = v896;
        let v897: f64 = p.p68;
        self.scalar_v897 = v897;
        let v901: f64 = p.p59;
        self.scalar_v901 = v901;
        let v903: f64 = p.p72;
        self.scalar_v903 = v903;
        let v907: f64 = p.p60;
        self.scalar_v907 = v907;
        let v908: f64 = p.p73;
        self.scalar_v908 = v908;
        let v912: f64 = p.p61;
        self.scalar_v912 = v912;
        let v913: f64 = p.p74;
        self.scalar_v913 = v913;
        let v929: f64 = p.p58;
        self.scalar_v929 = v929;
        let v937: f64 = p.p57;
        self.scalar_v937 = v937;
        let v943: f64 = (-v118);
        self.scalar_v943 = v943;
        let v948: f64 = p.p64;
        self.scalar_v948 = v948;
        let v960: f64 = p.p63;
        self.scalar_v960 = v960;
        let v972: f64 = p.p62;
        self.scalar_v972 = v972;
        let v1034: f64 = p.p47;
        self.scalar_v1034 = v1034;
        let v1035: f64 = (-p.p47);
        self.scalar_v1035 = v1035;
        let v1044: f64 = p.p48;
        self.scalar_v1044 = v1044;
        let v1045: f64 = (-p.p48);
        self.scalar_v1045 = v1045;
        let v1054: f64 = p.p49;
        self.scalar_v1054 = v1054;
        let v1055: f64 = (-p.p49);
        self.scalar_v1055 = v1055;
        let v1139: f64 = (if (p.p1 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v1139 = v1139;
        let v1144: f64 = p.p56;
        self.scalar_v1144 = v1144;
        let v1165: f64 = (-p.p0);
        self.scalar_v1165 = v1165;
        let v4361: f64 = (p.p0 * p.p47);
        self.scalar_v4361 = v4361;
        let v4362: f64 = (p.p47 * v1165);
        self.scalar_v4362 = v4362;
        let v4371: f64 = (p.p0 * p.p48);
        self.scalar_v4371 = v4371;
        let v4372: f64 = (p.p48 * v1165);
        self.scalar_v4372 = v4372;
        let v4381: f64 = (p.p0 * p.p49);
        self.scalar_v4381 = v4381;
        let v4382: f64 = (p.p49 * v1165);
        self.scalar_v4382 = v4382;
        let v4511: f64 = (p.p0 * p.p56);
        self.scalar_v4511 = v4511;
        let v4512: f64 = (p.p56 * v1165);
        self.scalar_v4512 = v4512;
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
        let v33: f64 = (temperature + self.scalar_v32);
        self.scalar_v33 = v33;
        let v34: f64 = (if self.scalar_v30 { self.scalar_v33 } else { 0.0 });
        self.scalar_v34 = v34;
        let v38: f64 = (if self.scalar_v35 { self.scalar_v37 } else { self.scalar_v34 });
        self.scalar_v38 = v38;
        let v47: f64 = (self.scalar_v38 * 8.617333262145179e-5);
        self.scalar_v47 = v47;
        let v49: f64 = (self.scalar_v47 * 0.1);
        self.scalar_v49 = v49;
        let v51: f64 = (1.0 / self.scalar_v47);
        self.scalar_v51 = v51;
        let v52: f64 = (self.scalar_v47 + self.scalar_v47);
        self.scalar_v52 = v52;
        let v53: f64 = (self.scalar_v52 + self.scalar_v52);
        self.scalar_v53 = v53;
        let v54: f64 = (self.scalar_v47 * self.scalar_v47);
        self.scalar_v54 = v54;
        let v55: f64 = (self.scalar_v54 + self.scalar_v54);
        self.scalar_v55 = v55;
        let v57: f64 = (self.scalar_v54 * 16.0);
        self.scalar_v57 = v57;
        let v60: f64 = (self.scalar_v38 * 0.000702);
        self.scalar_v60 = v60;
        let v61: f64 = (self.scalar_v38 * self.scalar_v60);
        self.scalar_v61 = v61;
        let v63: f64 = (self.scalar_v38 + 1108.0);
        self.scalar_v63 = v63;
        let v64: f64 = (self.scalar_v61 / self.scalar_v63);
        self.scalar_v64 = v64;
        let v65: f64 = (1.16 - self.scalar_v64);
        self.scalar_v65 = v65;
        let v71: f64 = (self.scalar_v38 - self.scalar_v45);
        self.scalar_v71 = v71;
        let v72: f64 = (self.scalar_v38 / self.scalar_v45);
        self.scalar_v72 = v72;
        let v75: f64 = (self.scalar_v71 * self.scalar_v74);
        self.scalar_v75 = v75;
        let v76: f64 = (self.scalar_v73 - self.scalar_v75);
        self.scalar_v76 = v76;
        let v79: f64 = f64::powf(self.scalar_v72, self.scalar_v78);
        self.scalar_v79 = v79;
        let v80: f64 = (self.scalar_v77 * self.scalar_v79);
        self.scalar_v80 = v80;
        let v83: f64 = f64::powf(self.scalar_v72, self.scalar_v82);
        self.scalar_v83 = v83;
        let v84: f64 = (self.scalar_v81 * self.scalar_v83);
        self.scalar_v84 = v84;
        let v87: f64 = (self.scalar_v71 * self.scalar_v86);
        self.scalar_v87 = v87;
        let v88: f64 = (1.0 + self.scalar_v87);
        self.scalar_v88 = v88;
        let v89: f64 = (self.scalar_v85 * self.scalar_v88);
        self.scalar_v89 = v89;
        let v91: f64 = (self.scalar_v72 * self.scalar_v90);
        self.scalar_v91 = v91;
        let v92: f64 = (3.0 * self.scalar_v47);
        self.scalar_v92 = v92;
        let v93: f64 = ((self.scalar_v72) as f64).ln();
        self.scalar_v93 = v93;
        let v94: f64 = (self.scalar_v92 * self.scalar_v93);
        self.scalar_v94 = v94;
        let v95: f64 = (self.scalar_v91 - self.scalar_v94);
        self.scalar_v95 = v95;
        let v96: f64 = (self.scalar_v70 * self.scalar_v72);
        self.scalar_v96 = v96;
        let v97: f64 = (self.scalar_v95 - self.scalar_v96);
        self.scalar_v97 = v97;
        let v98: f64 = (self.scalar_v65 + self.scalar_v97);
        self.scalar_v98 = v98;
        let v100: f64 = (self.scalar_v98 - 0.2);
        self.scalar_v100 = v100;
        let v101: f64 = (self.scalar_v100 * self.scalar_v100);
        self.scalar_v101 = v101;
        let v102: f64 = (self.scalar_v54 + self.scalar_v101);
        self.scalar_v102 = v102;
        let v103: f64 = ((self.scalar_v102) as f64).sqrt();
        self.scalar_v103 = v103;
        let v104: f64 = (self.scalar_v100 + self.scalar_v103);
        self.scalar_v104 = v104;
        let v105: f64 = (0.5 * self.scalar_v104);
        self.scalar_v105 = v105;
        let v106: f64 = (0.2 + self.scalar_v105);
        self.scalar_v106 = v106;
        let v107: f64 = ((self.scalar_v106) as f64).sqrt();
        self.scalar_v107 = v107;
        let v108: f64 = (1.0 / self.scalar_v84);
        self.scalar_v108 = v108;
        let v109: f64 = (self.scalar_v6 * self.scalar_v84);
        self.scalar_v109 = v109;
        let v110: f64 = (self.scalar_v6 * self.scalar_v89);
        self.scalar_v110 = v110;
        let v112: f64 = (self.scalar_v111 / self.scalar_v89);
        self.scalar_v112 = v112;
        let v119: f64 = (self.scalar_v84 * self.scalar_v115);
        self.scalar_v119 = v119;
        let v120: f64 = (0.5 * self.scalar_v119);
        self.scalar_v120 = v120;
        let v121: f64 = (self.scalar_v51 * self.scalar_v120);
        self.scalar_v121 = v121;
        let v122: f64 = ((self.scalar_v121) as f64).ln();
        self.scalar_v122 = v122;
        let v124: f64 = (self.scalar_v122 - 0.6);
        self.scalar_v124 = v124;
        let v125: f64 = (self.scalar_v47 * self.scalar_v124);
        self.scalar_v125 = v125;
        let v134: f64 = (self.scalar_v76 + self.scalar_v133);
        self.scalar_v134 = v134;
        let v135: f64 = (if self.scalar_v131 { self.scalar_v134 } else { self.scalar_v76 });
        self.scalar_v135 = v135;
        let v136: f64 = (if self.scalar_v24 { self.scalar_v135 } else { 0.0 });
        self.scalar_v136 = v136;
        let v140: f64 = (self.scalar_v139 - self.scalar_v76);
        self.scalar_v140 = v140;
        let v141: f64 = (-self.scalar_v76);
        self.scalar_v141 = v141;
        let v142: f64 = (if self.scalar_v131 { self.scalar_v140 } else { self.scalar_v141 });
        self.scalar_v142 = v142;
        let v143: f64 = (if self.scalar_v137 { self.scalar_v142 } else { self.scalar_v136 });
        self.scalar_v143 = v143;
        let v149: f64 = (self.scalar_v80 * self.scalar_v148);
        self.scalar_v149 = v149;
        let v150: f64 = (if self.scalar_v145 { self.scalar_v149 } else { self.scalar_v80 });
        self.scalar_v150 = v150;
        let v151: f64 = (self.scalar_v118 * self.scalar_v150);
        self.scalar_v151 = v151;
        let v159: f64 = (self.scalar_v107 * self.scalar_v158);
        self.scalar_v159 = v159;
        let v208: f64 = (self.scalar_v57 * 2.0);
        self.scalar_v208 = v208;
        let v322: f64 = (self.scalar_v47 / self.scalar_v119);
        self.scalar_v322 = v322;
        let v539: f64 = (self.scalar_v19 * self.scalar_v159);
        self.scalar_v539 = v539;
        let v540: f64 = (1.0 + self.scalar_v539);
        self.scalar_v540 = v540;
        let v541: f64 = (if self.scalar_v530 { self.scalar_v540 } else { 0.0 });
        self.scalar_v541 = v541;
        let v542: f64 = (self.scalar_v151 * self.scalar_v541);
        self.scalar_v542 = v542;
        let v578: f64 = (self.scalar_v53 + self.scalar_v53);
        self.scalar_v578 = v578;
        let v579: f64 = (self.scalar_v7 * self.scalar_v578);
        self.scalar_v579 = v579;
        let v753: bool = (self.scalar_v112 > 0.0);
        self.scalar_v753 = v753;
        let v757: f64 = (-self.scalar_v110);
        self.scalar_v757 = v757;
        let v860: f64 = (self.scalar_v65 / self.scalar_v47);
        self.scalar_v860 = v860;
        let v861: f64 = (self.scalar_v859 - self.scalar_v860);
        self.scalar_v861 = v861;
        let v863: f64 = (self.scalar_v93 * self.scalar_v862);
        self.scalar_v863 = v863;
        let v864: f64 = (self.scalar_v861 + self.scalar_v863);
        self.scalar_v864 = v864;
        let v866: f64 = (self.scalar_v864 / self.scalar_v865);
        self.scalar_v866 = v866;
        let v867: f64 = ((self.scalar_v866) as f64).exp();
        self.scalar_v867 = v867;
        let v869: f64 = (self.scalar_v867 * self.scalar_v868);
        self.scalar_v869 = v869;
        let v871: f64 = (self.scalar_v867 * self.scalar_v870);
        self.scalar_v871 = v871;
        let v873: f64 = (self.scalar_v867 * self.scalar_v872);
        self.scalar_v873 = v873;
        let v876: f64 = (self.scalar_v71 * self.scalar_v875);
        self.scalar_v876 = v876;
        let v877: f64 = (self.scalar_v874 - self.scalar_v876);
        self.scalar_v877 = v877;
        let v880: f64 = (self.scalar_v71 * self.scalar_v879);
        self.scalar_v880 = v880;
        let v881: f64 = (self.scalar_v878 - self.scalar_v880);
        self.scalar_v881 = v881;
        let v884: f64 = (self.scalar_v71 * self.scalar_v883);
        self.scalar_v884 = v884;
        let v885: f64 = (self.scalar_v882 - self.scalar_v884);
        self.scalar_v885 = v885;
        let v888: f64 = (self.scalar_v71 * self.scalar_v887);
        self.scalar_v888 = v888;
        let v889: f64 = (1.0 + self.scalar_v888);
        self.scalar_v889 = v889;
        let v890: f64 = (self.scalar_v886 * self.scalar_v889);
        self.scalar_v890 = v890;
        let v893: f64 = (self.scalar_v71 * self.scalar_v892);
        self.scalar_v893 = v893;
        let v894: f64 = (1.0 + self.scalar_v893);
        self.scalar_v894 = v894;
        let v895: f64 = (self.scalar_v891 * self.scalar_v894);
        self.scalar_v895 = v895;
        let v898: f64 = (self.scalar_v71 * self.scalar_v897);
        self.scalar_v898 = v898;
        let v899: f64 = (1.0 + self.scalar_v898);
        self.scalar_v899 = v899;
        let v900: f64 = (self.scalar_v896 * self.scalar_v899);
        self.scalar_v900 = v900;
        let v902: f64 = (self.scalar_v72 - 1.0);
        self.scalar_v902 = v902;
        let v904: f64 = (self.scalar_v902 * self.scalar_v903);
        self.scalar_v904 = v904;
        let v905: f64 = (1.0 + self.scalar_v904);
        self.scalar_v905 = v905;
        let v906: f64 = (self.scalar_v901 * self.scalar_v905);
        self.scalar_v906 = v906;
        let v909: f64 = (self.scalar_v902 * self.scalar_v908);
        self.scalar_v909 = v909;
        let v910: f64 = (1.0 + self.scalar_v909);
        self.scalar_v910 = v910;
        let v911: f64 = (self.scalar_v907 * self.scalar_v910);
        self.scalar_v911 = v911;
        let v914: f64 = (self.scalar_v902 * self.scalar_v913);
        self.scalar_v914 = v914;
        let v915: f64 = (1.0 + self.scalar_v914);
        self.scalar_v915 = v915;
        let v916: f64 = (self.scalar_v912 * self.scalar_v915);
        self.scalar_v916 = v916;
        let v917: f64 = (self.scalar_v851 * self.scalar_v869);
        self.scalar_v917 = v917;
        let v918: f64 = (self.scalar_v857 * self.scalar_v871);
        self.scalar_v918 = v918;
        let v919: f64 = (self.scalar_v917 + self.scalar_v918);
        self.scalar_v919 = v919;
        let v920: f64 = (self.scalar_v118 * self.scalar_v873);
        self.scalar_v920 = v920;
        let v921: f64 = (self.scalar_v919 + self.scalar_v920);
        self.scalar_v921 = v921;
        let v924: f64 = (self.scalar_v47 * self.scalar_v865);
        self.scalar_v924 = v924;
        let v944: f64 = (self.scalar_v873 * self.scalar_v943);
        self.scalar_v944 = v944;
        let v946: f64 = (self.scalar_v47 * self.scalar_v916);
        self.scalar_v946 = v946;
        let v958: f64 = (self.scalar_v47 * self.scalar_v911);
        self.scalar_v958 = v958;
        let v970: f64 = (self.scalar_v47 * self.scalar_v906);
        self.scalar_v970 = v970;
        let v982: f64 = (self.scalar_v837 * self.scalar_v869);
        self.scalar_v982 = v982;
        let v983: f64 = (self.scalar_v845 * self.scalar_v871);
        self.scalar_v983 = v983;
        let v984: f64 = (self.scalar_v982 + self.scalar_v983);
        self.scalar_v984 = v984;
        let v985: f64 = (self.scalar_v920 + self.scalar_v984);
        self.scalar_v985 = v985;
        let v1033: f64 = (self.scalar_v851 * self.scalar_v890);
        self.scalar_v1033 = v1033;
        let v1043: f64 = (self.scalar_v857 * self.scalar_v895);
        self.scalar_v1043 = v1043;
        let v1053: f64 = (self.scalar_v118 * self.scalar_v900);
        self.scalar_v1053 = v1053;
        let v1083: f64 = (self.scalar_v837 * self.scalar_v890);
        self.scalar_v1083 = v1083;
        let v1091: f64 = (self.scalar_v845 * self.scalar_v895);
        self.scalar_v1091 = v1091;
        let v4193: f64 = (self.scalar_v72 * self.scalar_v1165);
        self.scalar_v4193 = v4193;
        let v4194: f64 = (self.scalar_v23 * self.scalar_v72);
        self.scalar_v4194 = v4194;
        let v4195: f64 = (self.scalar_v4193 / self.scalar_v924);
        self.scalar_v4195 = v4195;
        let v4196: f64 = (self.scalar_v4194 / self.scalar_v924);
        self.scalar_v4196 = v4196;
        let v4199: f64 = (-self.scalar_v4195);
        self.scalar_v4199 = v4199;
        let v4200: f64 = (-self.scalar_v4196);
        self.scalar_v4200 = v4200;
        let v4207: f64 = (self.scalar_v4194 / self.scalar_v946);
        self.scalar_v4207 = v4207;
        let v4208: f64 = (self.scalar_v4193 / self.scalar_v946);
        self.scalar_v4208 = v4208;
        let v4209: f64 = (self.scalar_v948 * self.scalar_v4207);
        self.scalar_v4209 = v4209;
        let v4210: f64 = (self.scalar_v948 * self.scalar_v4208);
        self.scalar_v4210 = v4210;
        let v4226: f64 = (self.scalar_v4194 / self.scalar_v958);
        self.scalar_v4226 = v4226;
        let v4227: f64 = (self.scalar_v4193 / self.scalar_v958);
        self.scalar_v4227 = v4227;
        let v4228: f64 = (self.scalar_v960 * self.scalar_v4226);
        self.scalar_v4228 = v4228;
        let v4229: f64 = (self.scalar_v960 * self.scalar_v4227);
        self.scalar_v4229 = v4229;
        let v4247: f64 = (self.scalar_v4194 / self.scalar_v970);
        self.scalar_v4247 = v4247;
        let v4248: f64 = (self.scalar_v4193 / self.scalar_v970);
        self.scalar_v4248 = v4248;
        let v4249: f64 = (self.scalar_v972 * self.scalar_v4247);
        self.scalar_v4249 = v4249;
        let v4250: f64 = (self.scalar_v972 * self.scalar_v4248);
        self.scalar_v4250 = v4250;
        let v4325: f64 = (self.scalar_v23 / self.scalar_v877);
        self.scalar_v4325 = v4325;
        let v4326: f64 = (self.scalar_v1165 / self.scalar_v877);
        self.scalar_v4326 = v4326;
        let v4337: f64 = (self.scalar_v23 / self.scalar_v881);
        self.scalar_v4337 = v4337;
        let v4338: f64 = (self.scalar_v1165 / self.scalar_v881);
        self.scalar_v4338 = v4338;
        let v4349: f64 = (self.scalar_v23 / self.scalar_v885);
        self.scalar_v4349 = v4349;
        let v4350: f64 = (self.scalar_v1165 / self.scalar_v885);
        self.scalar_v4350 = v4350;
        let v4363: f64 = (self.scalar_v4361 / self.scalar_v877);
        self.scalar_v4363 = v4363;
        let v4364: f64 = (self.scalar_v4362 / self.scalar_v877);
        self.scalar_v4364 = v4364;
        let v4365: f64 = (-self.scalar_v4363);
        self.scalar_v4365 = v4365;
        let v4366: f64 = (-self.scalar_v4364);
        self.scalar_v4366 = v4366;
        let v4367: f64 = (self.scalar_v1033 * self.scalar_v4365);
        self.scalar_v4367 = v4367;
        let v4368: f64 = (self.scalar_v1033 * self.scalar_v4366);
        self.scalar_v4368 = v4368;
        let v4373: f64 = (self.scalar_v4371 / self.scalar_v881);
        self.scalar_v4373 = v4373;
        let v4374: f64 = (self.scalar_v4372 / self.scalar_v881);
        self.scalar_v4374 = v4374;
        let v4375: f64 = (-self.scalar_v4373);
        self.scalar_v4375 = v4375;
        let v4376: f64 = (-self.scalar_v4374);
        self.scalar_v4376 = v4376;
        let v4377: f64 = (self.scalar_v1043 * self.scalar_v4375);
        self.scalar_v4377 = v4377;
        let v4378: f64 = (self.scalar_v1043 * self.scalar_v4376);
        self.scalar_v4378 = v4378;
        let v4383: f64 = (self.scalar_v4381 / self.scalar_v885);
        self.scalar_v4383 = v4383;
        let v4384: f64 = (self.scalar_v4382 / self.scalar_v885);
        self.scalar_v4384 = v4384;
        let v4385: f64 = (-self.scalar_v4383);
        self.scalar_v4385 = v4385;
        let v4386: f64 = (-self.scalar_v4384);
        self.scalar_v4386 = v4386;
        let v4387: f64 = (self.scalar_v1053 * self.scalar_v4385);
        self.scalar_v4387 = v4387;
        let v4388: f64 = (self.scalar_v1053 * self.scalar_v4386);
        self.scalar_v4388 = v4388;
        let v4431: f64 = (self.scalar_v1083 * self.scalar_v4365);
        self.scalar_v4431 = v4431;
        let v4432: f64 = (self.scalar_v1083 * self.scalar_v4366);
        self.scalar_v4432 = v4432;
        let v4435: f64 = (self.scalar_v1091 * self.scalar_v4375);
        self.scalar_v4435 = v4435;
        let v4436: f64 = (self.scalar_v1091 * self.scalar_v4376);
        self.scalar_v4436 = v4436;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
