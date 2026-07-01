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
            params.p0 = 1e-17;
            params.p1 = 1.0;
            params.p2 = 0.0;
            params.p3 = 5.0;
            params.p4 = 10.0;
            params.p5 = 10.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.01;
            params.p9 = 1.11;
            params.p10 = 0.0;
            params.p11 = 10.0;
            params.p12 = 1e-5;
            params.p13 = 0.0;
            params.p14 = 1e-6;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.75;
            params.p18 = 0.33;
            params.p19 = 0.0;
            params.p20 = 0.001;
            params.p21 = 1.11;
            params.p22 = 3.0;
            params.p23 = 0.5;
            params.p24 = 0.5;
            params.p25 = 25.0;
            params.p26 = 1000.0;
            params.p27 = 0.0;
            params.p28 = 1.0;
            params.p29 = 1.0;
            params.p30 = 2.0;
            params.p31 = 0.0;
            params.p32 = 1.0;
            params.p33 = 0.0005;
            params.p34 = 0.0005;
            params.p35 = 5e-6;
            params.p36 = 1e-7;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 2.0;
            params.p40 = 100.0;
            params.p41 = 0.0;
            params.p42 = 1e-5;
            params.p43 = 1.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.001;
            validate_parameter("minr", params.p46, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p47 = 5.0;
            params.p48 = 100.0;
            params.p49 = 2.0;
            params.p50 = 100.0;
            params.p51 = 2.0;
            params.p52 = 0.1;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 5.0;
            params.p56 = 0.0;
            params.p57 = 20.0;
            params.p58 = 0.0;
            params.p59 = 1.5;
            params.p60 = 1.0;
            params.p61 = 10.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 2.0;
            params.p66 = 1e-6;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.75;
            params.p71 = 0.33;
            params.p72 = 1.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 0.75;
            params.p76 = 0.33;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 1.0;
            params.p81 = 0.0;
            params.p82 = 0.9;
            params.p83 = 1e-8;
            params.p84 = 0.0;
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
    pub nodes: [usize; 10],
    pub branches: [usize; 8],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 85]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 13]>,
    pub(crate) ddt_state_previous: Box<[f64; 13]>,
    pub(crate) ddt_state_older: Box<[f64; 13]>,
    pub(crate) ddt_state_initialized: Box<[bool; 13]>,
    pub(crate) ddt_derivative_current: Box<[f64; 13]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 13]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: bool,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: bool,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: bool,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: bool,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v563: f64,
    pub(crate) scalar_v564: f64,
    pub(crate) scalar_v569: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v572: bool,
    pub(crate) scalar_v575: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v582: bool,
    pub(crate) scalar_v584: f64,
    pub(crate) scalar_v585: bool,
    pub(crate) scalar_v586: f64,
    pub(crate) scalar_v589: f64,
    pub(crate) scalar_v592: f64,
    pub(crate) scalar_v597: f64,
    pub(crate) scalar_v610: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v624: f64,
    pub(crate) scalar_v625: f64,
    pub(crate) scalar_v631: f64,
    pub(crate) scalar_v634: f64,
    pub(crate) scalar_v658: f64,
    pub(crate) scalar_v659: f64,
    pub(crate) scalar_v660: f64,
    pub(crate) scalar_v666: f64,
    pub(crate) scalar_v669: f64,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v690: f64,
    pub(crate) scalar_v721: f64,
    pub(crate) scalar_v722: bool,
    pub(crate) scalar_v723: bool,
    pub(crate) scalar_v724: bool,
    pub(crate) scalar_v725: f64,
    pub(crate) scalar_v727: f64,
    pub(crate) scalar_v729: f64,
    pub(crate) scalar_v730: f64,
    pub(crate) scalar_v733: bool,
    pub(crate) scalar_v735: f64,
    pub(crate) scalar_v736: bool,
    pub(crate) scalar_v737: f64,
    pub(crate) scalar_v738: bool,
    pub(crate) scalar_v739: bool,
    pub(crate) scalar_v740: bool,
    pub(crate) scalar_v741: bool,
    pub(crate) scalar_v742: f64,
    pub(crate) scalar_v743: bool,
    pub(crate) scalar_v744: bool,
    pub(crate) scalar_v745: bool,
    pub(crate) scalar_v746: f64,
    pub(crate) scalar_v747: f64,
    pub(crate) scalar_v748: f64,
    pub(crate) scalar_v749: f64,
    pub(crate) scalar_v750: f64,
    pub(crate) scalar_v751: f64,
    pub(crate) scalar_v752: f64,
    pub(crate) scalar_v753: f64,
    pub(crate) scalar_v754: f64,
    pub(crate) scalar_v755: bool,
    pub(crate) scalar_v756: f64,
    pub(crate) scalar_v757: bool,
    pub(crate) scalar_v758: bool,
    pub(crate) scalar_v760: bool,
    pub(crate) scalar_v761: bool,
    pub(crate) scalar_v762: bool,
    pub(crate) scalar_v764: bool,
    pub(crate) scalar_v765: bool,
    pub(crate) scalar_v766: bool,
    pub(crate) scalar_v772: f64,
    pub(crate) scalar_v783: f64,
    pub(crate) scalar_v795: f64,
    pub(crate) scalar_v799: f64,
    pub(crate) scalar_v800: bool,
    pub(crate) scalar_v801: bool,
    pub(crate) scalar_v811: f64,
    pub(crate) scalar_v815: bool,
    pub(crate) scalar_v816: bool,
    pub(crate) scalar_v817: bool,
    pub(crate) scalar_v819: f64,
    pub(crate) scalar_v820: bool,
    pub(crate) scalar_v821: bool,
    pub(crate) scalar_v822: f64,
    pub(crate) scalar_v831: f64,
    pub(crate) scalar_v832: bool,
    pub(crate) scalar_v833: f64,
    pub(crate) scalar_v838: f64,
    pub(crate) scalar_v839: bool,
    pub(crate) scalar_v840: f64,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v847: bool,
    pub(crate) scalar_v848: f64,
    pub(crate) scalar_v875: f64,
    pub(crate) scalar_v880: f64,
    pub(crate) scalar_v1144: f64,
    pub(crate) scalar_v1145: f64,
    pub(crate) scalar_v1548: f64,
    pub(crate) scalar_v1549: f64,
    pub(crate) scalar_v1550: f64,
    pub(crate) scalar_v1551: f64,
    pub(crate) scalar_v1552: f64,
    pub(crate) scalar_v1553: f64,
    pub(crate) scalar_v1554: f64,
    pub(crate) scalar_v1555: f64,
    pub(crate) scalar_v1556: f64,
    pub(crate) scalar_v1557: f64,
    pub(crate) scalar_v1558: f64,
    pub(crate) scalar_v1665: f64,
    pub(crate) scalar_v1666: f64,
    pub(crate) scalar_v1690: f64,
    pub(crate) scalar_v1691: f64,
    pub(crate) scalar_v1763: f64,
    pub(crate) scalar_v1765: f64,
    pub(crate) scalar_v1964: f64,
    pub(crate) scalar_v1967: f64,
    pub(crate) scalar_v1968: f64,
    pub(crate) scalar_v1971: f64,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v1973: f64,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1976: f64,
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
            scalar_v3: self.scalar_v3,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v22: self.scalar_v22,
            scalar_v26: self.scalar_v26,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v36: self.scalar_v36,
            scalar_v39: self.scalar_v39,
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
            scalar_v67: self.scalar_v67,
            scalar_v69: self.scalar_v69,
            scalar_v72: self.scalar_v72,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v87: self.scalar_v87,
            scalar_v88: self.scalar_v88,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v111: self.scalar_v111,
            scalar_v137: self.scalar_v137,
            scalar_v142: self.scalar_v142,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v160: self.scalar_v160,
            scalar_v165: self.scalar_v165,
            scalar_v178: self.scalar_v178,
            scalar_v183: self.scalar_v183,
            scalar_v210: self.scalar_v210,
            scalar_v216: self.scalar_v216,
            scalar_v267: self.scalar_v267,
            scalar_v278: self.scalar_v278,
            scalar_v286: self.scalar_v286,
            scalar_v312: self.scalar_v312,
            scalar_v364: self.scalar_v364,
            scalar_v490: self.scalar_v490,
            scalar_v505: self.scalar_v505,
            scalar_v514: self.scalar_v514,
            scalar_v516: self.scalar_v516,
            scalar_v520: self.scalar_v520,
            scalar_v523: self.scalar_v523,
            scalar_v526: self.scalar_v526,
            scalar_v529: self.scalar_v529,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v537: self.scalar_v537,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v550: self.scalar_v550,
            scalar_v554: self.scalar_v554,
            scalar_v557: self.scalar_v557,
            scalar_v560: self.scalar_v560,
            scalar_v563: self.scalar_v563,
            scalar_v564: self.scalar_v564,
            scalar_v569: self.scalar_v569,
            scalar_v571: self.scalar_v571,
            scalar_v572: self.scalar_v572,
            scalar_v575: self.scalar_v575,
            scalar_v577: self.scalar_v577,
            scalar_v582: self.scalar_v582,
            scalar_v584: self.scalar_v584,
            scalar_v585: self.scalar_v585,
            scalar_v586: self.scalar_v586,
            scalar_v589: self.scalar_v589,
            scalar_v592: self.scalar_v592,
            scalar_v597: self.scalar_v597,
            scalar_v610: self.scalar_v610,
            scalar_v617: self.scalar_v617,
            scalar_v621: self.scalar_v621,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v624: self.scalar_v624,
            scalar_v625: self.scalar_v625,
            scalar_v631: self.scalar_v631,
            scalar_v634: self.scalar_v634,
            scalar_v658: self.scalar_v658,
            scalar_v659: self.scalar_v659,
            scalar_v660: self.scalar_v660,
            scalar_v666: self.scalar_v666,
            scalar_v669: self.scalar_v669,
            scalar_v689: self.scalar_v689,
            scalar_v690: self.scalar_v690,
            scalar_v721: self.scalar_v721,
            scalar_v722: self.scalar_v722,
            scalar_v723: self.scalar_v723,
            scalar_v724: self.scalar_v724,
            scalar_v725: self.scalar_v725,
            scalar_v727: self.scalar_v727,
            scalar_v729: self.scalar_v729,
            scalar_v730: self.scalar_v730,
            scalar_v733: self.scalar_v733,
            scalar_v735: self.scalar_v735,
            scalar_v736: self.scalar_v736,
            scalar_v737: self.scalar_v737,
            scalar_v738: self.scalar_v738,
            scalar_v739: self.scalar_v739,
            scalar_v740: self.scalar_v740,
            scalar_v741: self.scalar_v741,
            scalar_v742: self.scalar_v742,
            scalar_v743: self.scalar_v743,
            scalar_v744: self.scalar_v744,
            scalar_v745: self.scalar_v745,
            scalar_v746: self.scalar_v746,
            scalar_v747: self.scalar_v747,
            scalar_v748: self.scalar_v748,
            scalar_v749: self.scalar_v749,
            scalar_v750: self.scalar_v750,
            scalar_v751: self.scalar_v751,
            scalar_v752: self.scalar_v752,
            scalar_v753: self.scalar_v753,
            scalar_v754: self.scalar_v754,
            scalar_v755: self.scalar_v755,
            scalar_v756: self.scalar_v756,
            scalar_v757: self.scalar_v757,
            scalar_v758: self.scalar_v758,
            scalar_v760: self.scalar_v760,
            scalar_v761: self.scalar_v761,
            scalar_v762: self.scalar_v762,
            scalar_v764: self.scalar_v764,
            scalar_v765: self.scalar_v765,
            scalar_v766: self.scalar_v766,
            scalar_v772: self.scalar_v772,
            scalar_v783: self.scalar_v783,
            scalar_v795: self.scalar_v795,
            scalar_v799: self.scalar_v799,
            scalar_v800: self.scalar_v800,
            scalar_v801: self.scalar_v801,
            scalar_v811: self.scalar_v811,
            scalar_v815: self.scalar_v815,
            scalar_v816: self.scalar_v816,
            scalar_v817: self.scalar_v817,
            scalar_v819: self.scalar_v819,
            scalar_v820: self.scalar_v820,
            scalar_v821: self.scalar_v821,
            scalar_v822: self.scalar_v822,
            scalar_v831: self.scalar_v831,
            scalar_v832: self.scalar_v832,
            scalar_v833: self.scalar_v833,
            scalar_v838: self.scalar_v838,
            scalar_v839: self.scalar_v839,
            scalar_v840: self.scalar_v840,
            scalar_v846: self.scalar_v846,
            scalar_v847: self.scalar_v847,
            scalar_v848: self.scalar_v848,
            scalar_v875: self.scalar_v875,
            scalar_v880: self.scalar_v880,
            scalar_v1144: self.scalar_v1144,
            scalar_v1145: self.scalar_v1145,
            scalar_v1548: self.scalar_v1548,
            scalar_v1549: self.scalar_v1549,
            scalar_v1550: self.scalar_v1550,
            scalar_v1551: self.scalar_v1551,
            scalar_v1552: self.scalar_v1552,
            scalar_v1553: self.scalar_v1553,
            scalar_v1554: self.scalar_v1554,
            scalar_v1555: self.scalar_v1555,
            scalar_v1556: self.scalar_v1556,
            scalar_v1557: self.scalar_v1557,
            scalar_v1558: self.scalar_v1558,
            scalar_v1665: self.scalar_v1665,
            scalar_v1666: self.scalar_v1666,
            scalar_v1690: self.scalar_v1690,
            scalar_v1691: self.scalar_v1691,
            scalar_v1763: self.scalar_v1763,
            scalar_v1765: self.scalar_v1765,
            scalar_v1964: self.scalar_v1964,
            scalar_v1967: self.scalar_v1967,
            scalar_v1968: self.scalar_v1968,
            scalar_v1971: self.scalar_v1971,
            scalar_v1972: self.scalar_v1972,
            scalar_v1973: self.scalar_v1973,
            scalar_v1975: self.scalar_v1975,
            scalar_v1976: self.scalar_v1976,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 6;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 6] = ["ci", "bi", "ei", "dt1", "tt", "tbb"];

    pub const BRANCH_COUNT: usize = 8;
    pub const PARAMETER_COUNT: usize = 85;
    pub const VARIABLE_COUNT: usize = 128;
    pub const DDT_STATE_COUNT: usize = 13;
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
            scalar_v3: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v22: 0.0,
            scalar_v26: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v36: 0.0,
            scalar_v39: 0.0,
            scalar_v42: 0.0,
            scalar_v44: 0.0,
            scalar_v45: false,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v49: false,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: false,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v57: false,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v62: 0.0,
            scalar_v67: 0.0,
            scalar_v69: 0.0,
            scalar_v72: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v111: 0.0,
            scalar_v137: 0.0,
            scalar_v142: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v160: 0.0,
            scalar_v165: 0.0,
            scalar_v178: 0.0,
            scalar_v183: 0.0,
            scalar_v210: 0.0,
            scalar_v216: 0.0,
            scalar_v267: 0.0,
            scalar_v278: 0.0,
            scalar_v286: 0.0,
            scalar_v312: 0.0,
            scalar_v364: 0.0,
            scalar_v490: 0.0,
            scalar_v505: 0.0,
            scalar_v514: 0.0,
            scalar_v516: 0.0,
            scalar_v520: 0.0,
            scalar_v523: 0.0,
            scalar_v526: 0.0,
            scalar_v529: 0.0,
            scalar_v532: 0.0,
            scalar_v533: 0.0,
            scalar_v537: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v545: 0.0,
            scalar_v546: 0.0,
            scalar_v550: 0.0,
            scalar_v554: 0.0,
            scalar_v557: 0.0,
            scalar_v560: 0.0,
            scalar_v563: 0.0,
            scalar_v564: 0.0,
            scalar_v569: 0.0,
            scalar_v571: 0.0,
            scalar_v572: false,
            scalar_v575: 0.0,
            scalar_v577: 0.0,
            scalar_v582: false,
            scalar_v584: 0.0,
            scalar_v585: false,
            scalar_v586: 0.0,
            scalar_v589: 0.0,
            scalar_v592: 0.0,
            scalar_v597: 0.0,
            scalar_v610: 0.0,
            scalar_v617: 0.0,
            scalar_v621: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v624: 0.0,
            scalar_v625: 0.0,
            scalar_v631: 0.0,
            scalar_v634: 0.0,
            scalar_v658: 0.0,
            scalar_v659: 0.0,
            scalar_v660: 0.0,
            scalar_v666: 0.0,
            scalar_v669: 0.0,
            scalar_v689: 0.0,
            scalar_v690: 0.0,
            scalar_v721: 0.0,
            scalar_v722: false,
            scalar_v723: false,
            scalar_v724: false,
            scalar_v725: 0.0,
            scalar_v727: 0.0,
            scalar_v729: 0.0,
            scalar_v730: 0.0,
            scalar_v733: false,
            scalar_v735: 0.0,
            scalar_v736: false,
            scalar_v737: 0.0,
            scalar_v738: false,
            scalar_v739: false,
            scalar_v740: false,
            scalar_v741: false,
            scalar_v742: 0.0,
            scalar_v743: false,
            scalar_v744: false,
            scalar_v745: false,
            scalar_v746: 0.0,
            scalar_v747: 0.0,
            scalar_v748: 0.0,
            scalar_v749: 0.0,
            scalar_v750: 0.0,
            scalar_v751: 0.0,
            scalar_v752: 0.0,
            scalar_v753: 0.0,
            scalar_v754: 0.0,
            scalar_v755: false,
            scalar_v756: 0.0,
            scalar_v757: false,
            scalar_v758: false,
            scalar_v760: false,
            scalar_v761: false,
            scalar_v762: false,
            scalar_v764: false,
            scalar_v765: false,
            scalar_v766: false,
            scalar_v772: 0.0,
            scalar_v783: 0.0,
            scalar_v795: 0.0,
            scalar_v799: 0.0,
            scalar_v800: false,
            scalar_v801: false,
            scalar_v811: 0.0,
            scalar_v815: false,
            scalar_v816: false,
            scalar_v817: false,
            scalar_v819: 0.0,
            scalar_v820: false,
            scalar_v821: false,
            scalar_v822: 0.0,
            scalar_v831: 0.0,
            scalar_v832: false,
            scalar_v833: 0.0,
            scalar_v838: 0.0,
            scalar_v839: false,
            scalar_v840: 0.0,
            scalar_v846: 0.0,
            scalar_v847: false,
            scalar_v848: 0.0,
            scalar_v875: 0.0,
            scalar_v880: 0.0,
            scalar_v1144: 0.0,
            scalar_v1145: 0.0,
            scalar_v1548: 0.0,
            scalar_v1549: 0.0,
            scalar_v1550: 0.0,
            scalar_v1551: 0.0,
            scalar_v1552: 0.0,
            scalar_v1553: 0.0,
            scalar_v1554: 0.0,
            scalar_v1555: 0.0,
            scalar_v1556: 0.0,
            scalar_v1557: 0.0,
            scalar_v1558: 0.0,
            scalar_v1665: 0.0,
            scalar_v1666: 0.0,
            scalar_v1690: 0.0,
            scalar_v1691: 0.0,
            scalar_v1763: 0.0,
            scalar_v1765: 0.0,
            scalar_v1964: 0.0,
            scalar_v1967: 0.0,
            scalar_v1968: 0.0,
            scalar_v1971: 0.0,
            scalar_v1972: 0.0,
            scalar_v1973: 0.0,
            scalar_v1975: 0.0,
            scalar_v1976: 0.0,
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
            scalar_v3,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v22,
            scalar_v26,
            scalar_v30,
            scalar_v31,
            scalar_v36,
            scalar_v39,
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
            scalar_v67,
            scalar_v69,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v87,
            scalar_v88,
            scalar_v92,
            scalar_v93,
            scalar_v97,
            scalar_v98,
            scalar_v102,
            scalar_v103,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v111,
            scalar_v137,
            scalar_v142,
            scalar_v144,
            scalar_v145,
            scalar_v160,
            scalar_v165,
            scalar_v178,
            scalar_v183,
            scalar_v210,
            scalar_v216,
            scalar_v267,
            scalar_v278,
            scalar_v286,
            scalar_v312,
            scalar_v364,
            scalar_v490,
            scalar_v505,
            scalar_v514,
            scalar_v516,
            scalar_v520,
            scalar_v523,
            scalar_v526,
            scalar_v529,
            scalar_v532,
            scalar_v533,
            scalar_v537,
            scalar_v540,
            scalar_v541,
            scalar_v545,
            scalar_v546,
            scalar_v550,
            scalar_v554,
            scalar_v557,
            scalar_v560,
            scalar_v563,
            scalar_v564,
            scalar_v569,
            scalar_v571,
            scalar_v572,
            scalar_v575,
            scalar_v577,
            scalar_v582,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v589,
            scalar_v592,
            scalar_v597,
            scalar_v610,
            scalar_v617,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v631,
            scalar_v634,
            scalar_v658,
            scalar_v659,
            scalar_v660,
            scalar_v666,
            scalar_v669,
            scalar_v689,
            scalar_v690,
            scalar_v721,
            scalar_v722,
            scalar_v723,
            scalar_v724,
            scalar_v725,
            scalar_v727,
            scalar_v729,
            scalar_v730,
            scalar_v733,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v744,
            scalar_v745,
            scalar_v746,
            scalar_v747,
            scalar_v748,
            scalar_v749,
            scalar_v750,
            scalar_v751,
            scalar_v752,
            scalar_v753,
            scalar_v754,
            scalar_v755,
            scalar_v756,
            scalar_v757,
            scalar_v758,
            scalar_v760,
            scalar_v761,
            scalar_v762,
            scalar_v764,
            scalar_v765,
            scalar_v766,
            scalar_v772,
            scalar_v783,
            scalar_v795,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v811,
            scalar_v815,
            scalar_v816,
            scalar_v817,
            scalar_v819,
            scalar_v820,
            scalar_v821,
            scalar_v822,
            scalar_v831,
            scalar_v832,
            scalar_v833,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v875,
            scalar_v880,
            scalar_v1144,
            scalar_v1145,
            scalar_v1548,
            scalar_v1549,
            scalar_v1550,
            scalar_v1551,
            scalar_v1552,
            scalar_v1553,
            scalar_v1554,
            scalar_v1555,
            scalar_v1556,
            scalar_v1557,
            scalar_v1558,
            scalar_v1665,
            scalar_v1666,
            scalar_v1690,
            scalar_v1691,
            scalar_v1763,
            scalar_v1765,
            scalar_v1964,
            scalar_v1967,
            scalar_v1968,
            scalar_v1971,
            scalar_v1972,
            scalar_v1973,
            scalar_v1975,
            scalar_v1976,
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
            scalar_v3,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v22,
            scalar_v26,
            scalar_v30,
            scalar_v31,
            scalar_v36,
            scalar_v39,
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
            scalar_v67,
            scalar_v69,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v87,
            scalar_v88,
            scalar_v92,
            scalar_v93,
            scalar_v97,
            scalar_v98,
            scalar_v102,
            scalar_v103,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v111,
            scalar_v137,
            scalar_v142,
            scalar_v144,
            scalar_v145,
            scalar_v160,
            scalar_v165,
            scalar_v178,
            scalar_v183,
            scalar_v210,
            scalar_v216,
            scalar_v267,
            scalar_v278,
            scalar_v286,
            scalar_v312,
            scalar_v364,
            scalar_v490,
            scalar_v505,
            scalar_v514,
            scalar_v516,
            scalar_v520,
            scalar_v523,
            scalar_v526,
            scalar_v529,
            scalar_v532,
            scalar_v533,
            scalar_v537,
            scalar_v540,
            scalar_v541,
            scalar_v545,
            scalar_v546,
            scalar_v550,
            scalar_v554,
            scalar_v557,
            scalar_v560,
            scalar_v563,
            scalar_v564,
            scalar_v569,
            scalar_v571,
            scalar_v572,
            scalar_v575,
            scalar_v577,
            scalar_v582,
            scalar_v584,
            scalar_v585,
            scalar_v586,
            scalar_v589,
            scalar_v592,
            scalar_v597,
            scalar_v610,
            scalar_v617,
            scalar_v621,
            scalar_v622,
            scalar_v623,
            scalar_v624,
            scalar_v625,
            scalar_v631,
            scalar_v634,
            scalar_v658,
            scalar_v659,
            scalar_v660,
            scalar_v666,
            scalar_v669,
            scalar_v689,
            scalar_v690,
            scalar_v721,
            scalar_v722,
            scalar_v723,
            scalar_v724,
            scalar_v725,
            scalar_v727,
            scalar_v729,
            scalar_v730,
            scalar_v733,
            scalar_v735,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v739,
            scalar_v740,
            scalar_v741,
            scalar_v742,
            scalar_v743,
            scalar_v744,
            scalar_v745,
            scalar_v746,
            scalar_v747,
            scalar_v748,
            scalar_v749,
            scalar_v750,
            scalar_v751,
            scalar_v752,
            scalar_v753,
            scalar_v754,
            scalar_v755,
            scalar_v756,
            scalar_v757,
            scalar_v758,
            scalar_v760,
            scalar_v761,
            scalar_v762,
            scalar_v764,
            scalar_v765,
            scalar_v766,
            scalar_v772,
            scalar_v783,
            scalar_v795,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v811,
            scalar_v815,
            scalar_v816,
            scalar_v817,
            scalar_v819,
            scalar_v820,
            scalar_v821,
            scalar_v822,
            scalar_v831,
            scalar_v832,
            scalar_v833,
            scalar_v838,
            scalar_v839,
            scalar_v840,
            scalar_v846,
            scalar_v847,
            scalar_v848,
            scalar_v875,
            scalar_v880,
            scalar_v1144,
            scalar_v1145,
            scalar_v1548,
            scalar_v1549,
            scalar_v1550,
            scalar_v1551,
            scalar_v1552,
            scalar_v1553,
            scalar_v1554,
            scalar_v1555,
            scalar_v1556,
            scalar_v1557,
            scalar_v1558,
            scalar_v1665,
            scalar_v1666,
            scalar_v1690,
            scalar_v1691,
            scalar_v1763,
            scalar_v1765,
            scalar_v1964,
            scalar_v1967,
            scalar_v1968,
            scalar_v1971,
            scalar_v1972,
            scalar_v1973,
            scalar_v1975,
            scalar_v1976,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "isr" => { validate_parameter("isr", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "ntr" => { validate_parameter("ntr", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "vtr" => { validate_parameter("vtr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "bvr" => { validate_parameter("bvr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "xbvr" => { validate_finite_parameter("xbvr", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "xjbv" => { validate_parameter("xjbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "ther" => { validate_parameter("ther", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "theexp" => { validate_parameter("theexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "xtheexp" => { validate_finite_parameter("xtheexp", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "nbv" => { validate_parameter("nbv", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "rb" => { validate_parameter("rb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "rbe" => { validate_parameter("rbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ree" => { validate_parameter("ree", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "vje" => { validate_parameter("vje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "mje" => { validate_parameter("mje", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "qtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "vtt0" => { validate_parameter("qtt0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "eg" => { validate_parameter("eg", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "xti" => { validate_parameter("xti", value, Some((0.0, "0.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "xtir" => { validate_parameter("xtir", value, Some((-20.0, "-20.0")), false, Some((20.0, "20.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-40.0, "-40.0")), false, Some((125.0, "125.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "tfail" => { validate_parameter("tfail", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "extmod" => { validate_parameter("extmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "rbmod" => { validate_parameter("rbmod", value, Some((-1.0, "-1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "rth1" => { validate_parameter("rth1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "cth1" => { validate_parameter("cth1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "arb" => { validate_parameter("arb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "are" => { validate_parameter("are", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "texp" => { validate_parameter("texp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "vtf0" => { validate_parameter("vtf0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "atff" => { validate_parameter("atff", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "n" => { validate_parameter("n", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "qexp" => { validate_parameter("qexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ijbv" => { validate_parameter("ijbv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "vsatb" => { validate_parameter("vsatb", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "mexp" => { validate_parameter("mexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "vsate" => { validate_parameter("vsate", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "mexpe" => { validate_parameter("mexpe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "bf" => { validate_parameter("bf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "vaf" => { validate_parameter("vaf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "ikf" => { validate_parameter("ikf", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "xjbvc" => { validate_parameter("xjbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "ijbvc" => { validate_parameter("ijbvc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "nbvc" => { validate_parameter("nbvc", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "ise" => { validate_parameter("ise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "ne" => { validate_parameter("ne", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "br" => { validate_parameter("br", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "nr" => { validate_parameter("nr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "var" => { validate_parameter("var", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "ikr" => { validate_parameter("ikr", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "isc" => { validate_parameter("isc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "nc" => { validate_parameter("nc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "rc" => { validate_parameter("rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "rce" => { validate_parameter("rce", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "ptf" => { validate_parameter("ptf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "vjc" => { validate_parameter("vjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "mjc" => { validate_parameter("mjc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "vjs" => { validate_parameter("vjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "mjs" => { validate_parameter("mjs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "xtb" => { validate_parameter("xtb", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "arc" => { validate_parameter("arc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "kbwm" => { validate_parameter("kbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "xbwm" => { validate_parameter("xbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "ikbwm" => { validate_parameter("ikbwm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "xkf" => { validate_parameter("xkf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "cthbb" => { validate_parameter("cthbb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "cdelay" => { validate_parameter("cdelay", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmesd'", name)),
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
        let v3: f64 = p.p45;
        self.scalar_v3 = v3;
        let v14: f64 = p.p43;
        self.scalar_v14 = v14;
        let v15: f64 = p.p42;
        self.scalar_v15 = v15;
        let v16: f64 = (p.p43 * p.p42);
        self.scalar_v16 = v16;
        let v17: f64 = p.p29;
        self.scalar_v17 = v17;
        let v22: f64 = p.p79;
        self.scalar_v22 = v22;
        let v26: f64 = p.p80;
        self.scalar_v26 = v26;
        let v30: f64 = p.p25;
        self.scalar_v30 = v30;
        let v31: f64 = (273.15 + p.p25);
        self.scalar_v31 = v31;
        let v36: f64 = p.p77;
        self.scalar_v36 = v36;
        let v39: f64 = p.p52;
        self.scalar_v39 = v39;
        let v42: f64 = p.p60;
        self.scalar_v42 = v42;
        let v44: f64 = p.p53;
        self.scalar_v44 = v44;
        let v45: bool = (p.p53 > 0.0);
        self.scalar_v45 = v45;
        let v46: f64 = (1.0 / p.p53);
        self.scalar_v46 = v46;
        let v47: f64 = (if v45 { v46 } else { 0.0 });
        self.scalar_v47 = v47;
        let v48: f64 = p.p62;
        self.scalar_v48 = v48;
        let v49: bool = (p.p62 > 0.0);
        self.scalar_v49 = v49;
        let v50: f64 = (1.0 / p.p62);
        self.scalar_v50 = v50;
        let v51: f64 = (if v49 { v50 } else { 0.0 });
        self.scalar_v51 = v51;
        let v52: f64 = p.p54;
        self.scalar_v52 = v52;
        let v53: bool = (p.p54 > 0.0);
        self.scalar_v53 = v53;
        let v54: f64 = (1.0 / p.p54);
        self.scalar_v54 = v54;
        let v55: f64 = (if v53 { v54 } else { 0.0 });
        self.scalar_v55 = v55;
        let v56: f64 = p.p63;
        self.scalar_v56 = v56;
        let v57: bool = (p.p63 > 0.0);
        self.scalar_v57 = v57;
        let v58: f64 = (1.0 / p.p63);
        self.scalar_v58 = v58;
        let v59: f64 = (if v57 { v58 } else { 0.0 });
        self.scalar_v59 = v59;
        let v60: f64 = p.p22;
        self.scalar_v60 = v60;
        let v62: f64 = p.p21;
        self.scalar_v62 = v62;
        let v67: f64 = p.p23;
        self.scalar_v67 = v67;
        let v69: f64 = p.p0;
        self.scalar_v69 = v69;
        let v72: f64 = p.p2;
        self.scalar_v72 = v72;
        let v75: f64 = p.p58;
        self.scalar_v75 = v75;
        let v76: f64 = p.p59;
        self.scalar_v76 = v76;
        let v81: f64 = p.p64;
        self.scalar_v81 = v81;
        let v82: f64 = p.p65;
        self.scalar_v82 = v82;
        let v87: f64 = p.p47;
        self.scalar_v87 = v87;
        let v88: f64 = p.p7;
        self.scalar_v88 = v88;
        let v92: f64 = p.p5;
        self.scalar_v92 = v92;
        let v93: f64 = p.p6;
        self.scalar_v93 = v93;
        let v97: f64 = p.p9;
        self.scalar_v97 = v97;
        let v98: f64 = p.p10;
        self.scalar_v98 = v98;
        let v102: f64 = p.p56;
        self.scalar_v102 = v102;
        let v103: f64 = p.p55;
        self.scalar_v103 = v103;
        let v107: f64 = p.p16;
        self.scalar_v107 = v107;
        let v108: f64 = p.p69;
        self.scalar_v108 = v108;
        let v109: f64 = p.p74;
        self.scalar_v109 = v109;
        let v111: f64 = (v31 / 300.15);
        self.scalar_v111 = v111;
        let v137: f64 = p.p17;
        self.scalar_v137 = v137;
        let v142: f64 = p.p18;
        self.scalar_v142 = v142;
        let v144: f64 = (v31 - 300.15);
        self.scalar_v144 = v144;
        let v145: f64 = (0.0004 * v144);
        self.scalar_v145 = v145;
        let v160: f64 = p.p70;
        self.scalar_v160 = v160;
        let v165: f64 = p.p71;
        self.scalar_v165 = v165;
        let v178: f64 = p.p75;
        self.scalar_v178 = v178;
        let v183: f64 = p.p76;
        self.scalar_v183 = v183;
        let v210: f64 = p.p1;
        self.scalar_v210 = v210;
        let v216: f64 = p.p11;
        self.scalar_v216 = v216;
        let v267: f64 = p.p8;
        self.scalar_v267 = v267;
        let v278: f64 = p.p4;
        self.scalar_v278 = v278;
        let v286: f64 = p.p3;
        self.scalar_v286 = v286;
        let v312: f64 = p.p57;
        self.scalar_v312 = v312;
        let v364: f64 = p.p61;
        self.scalar_v364 = v364;
        let v490: f64 = p.p81;
        self.scalar_v490 = v490;
        let v505: f64 = p.p82;
        self.scalar_v505 = v505;
        let v514: f64 = p.p84;
        self.scalar_v514 = v514;
        let v516: f64 = (1.0 - p.p84);
        self.scalar_v516 = v516;
        let v520: f64 = p.p48;
        self.scalar_v520 = v520;
        let v523: f64 = p.p49;
        self.scalar_v523 = v523;
        let v526: f64 = p.p50;
        self.scalar_v526 = v526;
        let v529: f64 = p.p51;
        self.scalar_v529 = v529;
        let v532: f64 = p.p12;
        self.scalar_v532 = v532;
        let v533: f64 = p.p37;
        self.scalar_v533 = v533;
        let v537: f64 = (1.0 / p.p49);
        self.scalar_v537 = v537;
        let v540: f64 = p.p66;
        self.scalar_v540 = v540;
        let v541: f64 = p.p78;
        self.scalar_v541 = v541;
        let v545: f64 = p.p14;
        self.scalar_v545 = v545;
        let v546: f64 = p.p38;
        self.scalar_v546 = v546;
        let v550: f64 = (1.0 / p.p51);
        self.scalar_v550 = v550;
        let v554: f64 = p.p40;
        self.scalar_v554 = v554;
        let v557: f64 = p.p39;
        self.scalar_v557 = v557;
        let v560: f64 = (1.0 / p.p39);
        self.scalar_v560 = v560;
        let v563: f64 = p.p19;
        self.scalar_v563 = v563;
        let v564: f64 = p.p41;
        self.scalar_v564 = v564;
        let v569: f64 = p.p73;
        self.scalar_v569 = v569;
        let v571: f64 = p.p32;
        self.scalar_v571 = v571;
        let v572: bool = (1.0 == p.p32);
        self.scalar_v572 = v572;
        let v575: f64 = p.p20;
        self.scalar_v575 = v575;
        let v577: f64 = p.p44;
        self.scalar_v577 = v577;
        let v582: bool = (!v572);
        self.scalar_v582 = v582;
        let v584: f64 = p.p31;
        self.scalar_v584 = v584;
        let v585: bool = (1.0 == p.p31);
        self.scalar_v585 = v585;
        let v586: f64 = p.p13;
        self.scalar_v586 = v586;
        let v589: f64 = p.p67;
        self.scalar_v589 = v589;
        let v592: f64 = p.p15;
        self.scalar_v592 = v592;
        let v597: f64 = (1.0 - p.p76);
        self.scalar_v597 = v597;
        let v610: f64 = (p.p76 * 0.5);
        self.scalar_v610 = v610;
        let v617: f64 = p.p24;
        self.scalar_v617 = v617;
        let v621: f64 = (-1.0 - p.p18);
        self.scalar_v621 = v621;
        let v622: f64 = (1.0 - p.p24);
        self.scalar_v622 = v622;
        let v623: f64 = ((v622) as f64).ln();
        self.scalar_v623 = v623;
        let v624: f64 = (v621 * v623);
        self.scalar_v624 = v624;
        let v625: f64 = ((v624) as f64).exp();
        self.scalar_v625 = v625;
        let v631: f64 = (1.0 - p.p18);
        self.scalar_v631 = v631;
        let v634: f64 = (p.p18 * 0.5);
        self.scalar_v634 = v634;
        let v658: f64 = (-1.0 - p.p71);
        self.scalar_v658 = v658;
        let v659: f64 = (v623 * v658);
        self.scalar_v659 = v659;
        let v660: f64 = ((v659) as f64).exp();
        self.scalar_v660 = v660;
        let v666: f64 = (1.0 - p.p71);
        self.scalar_v666 = v666;
        let v669: f64 = (p.p71 * 0.5);
        self.scalar_v669 = v669;
        let v689: f64 = p.p72;
        self.scalar_v689 = v689;
        let v690: f64 = (1.0 - p.p72);
        self.scalar_v690 = v690;
        let v721: f64 = p.p68;
        self.scalar_v721 = v721;
        let v722: bool = (0.0 != p.p68);
        self.scalar_v722 = v722;
        let v723: bool = (0.0 != p.p19);
        self.scalar_v723 = v723;
        let v724: bool = (v722 && v723);
        self.scalar_v724 = v724;
        let v725: f64 = (p.p29 * p.p68);
        self.scalar_v725 = v725;
        let v727: f64 = (v725 * 3.141592653589793);
        self.scalar_v727 = v727;
        let v729: f64 = (v727 / 180.0);
        self.scalar_v729 = v729;
        let v730: f64 = (p.p19 * v729);
        self.scalar_v730 = v730;
        let v733: bool = (!v724);
        self.scalar_v733 = v733;
        let v735: f64 = p.p30;
        self.scalar_v735 = v735;
        let v736: bool = (1.0 == p.p30);
        self.scalar_v736 = v736;
        let v737: f64 = p.p33;
        self.scalar_v737 = v737;
        let v738: bool = (p.p33 > 0.0);
        self.scalar_v738 = v738;
        let v739: bool = (v736 && v738);
        self.scalar_v739 = v739;
        let v740: bool = (2.0 == p.p30);
        self.scalar_v740 = v740;
        let v741: bool = (v738 && v740);
        self.scalar_v741 = v741;
        let v742: f64 = p.p35;
        self.scalar_v742 = v742;
        let v743: bool = (p.p35 > 0.0);
        self.scalar_v743 = v743;
        let v744: bool = (v741 && v743);
        self.scalar_v744 = v744;
        let v745: bool = (-1.0 == p.p30);
        self.scalar_v745 = v745;
        let v746: f64 = (p.p31 * p.p13);
        self.scalar_v746 = v746;
        let v747: f64 = (p.p12 + v746);
        self.scalar_v747 = v747;
        let v748: f64 = (v747 / v16);
        self.scalar_v748 = v748;
        let v749: f64 = (p.p31 * p.p15);
        self.scalar_v749 = v749;
        let v750: f64 = (p.p14 + v749);
        self.scalar_v750 = v750;
        let v751: f64 = (v750 / v16);
        self.scalar_v751 = v751;
        let v752: f64 = (p.p31 * p.p67);
        self.scalar_v752 = v752;
        let v753: f64 = (p.p66 + v752);
        self.scalar_v753 = v753;
        let v754: f64 = (v753 / v16);
        self.scalar_v754 = v754;
        let v755: bool = (v748 > 0.0);
        self.scalar_v755 = v755;
        let v756: f64 = p.p46;
        self.scalar_v756 = v756;
        let v757: bool = (v748 >= p.p46);
        self.scalar_v757 = v757;
        let v758: bool = (v755 && v757);
        self.scalar_v758 = v758;
        let v760: bool = (v751 > 0.0);
        self.scalar_v760 = v760;
        let v761: bool = (v751 >= p.p46);
        self.scalar_v761 = v761;
        let v762: bool = (v760 && v761);
        self.scalar_v762 = v762;
        let v764: bool = (v754 > 0.0);
        self.scalar_v764 = v764;
        let v765: bool = (v754 >= p.p46);
        self.scalar_v765 = v765;
        let v766: bool = (v764 && v765);
        self.scalar_v766 = v766;
        let v772: f64 = p.p83;
        self.scalar_v772 = v772;
        let v783: f64 = (if v582 { 0.0 } else { 0.0 });
        self.scalar_v783 = v783;
        let v795: f64 = p.p34;
        self.scalar_v795 = v795;
        let v799: f64 = (if v739 { 0.0 } else { 0.0 });
        self.scalar_v799 = v799;
        let v800: bool = (!v739);
        self.scalar_v800 = v800;
        let v801: bool = (v744 && v800);
        self.scalar_v801 = v801;
        let v811: f64 = p.p36;
        self.scalar_v811 = v811;
        let v815: bool = (!v744);
        self.scalar_v815 = v815;
        let v816: bool = (v800 && v815);
        self.scalar_v816 = v816;
        let v817: bool = (v745 && v816);
        self.scalar_v817 = v817;
        let v819: f64 = (if v817 { 0.0 } else { 0.0 });
        self.scalar_v819 = v819;
        let v820: bool = (!v745);
        self.scalar_v820 = v820;
        let v821: bool = (v816 && v820);
        self.scalar_v821 = v821;
        let v822: f64 = (if v821 { 0.0 } else { 0.0 });
        self.scalar_v822 = v822;
        let v831: f64 = (if v758 { 0.0 } else { 0.0 });
        self.scalar_v831 = v831;
        let v832: bool = (!v758);
        self.scalar_v832 = v832;
        let v833: f64 = (if v832 { 0.0 } else { 0.0 });
        self.scalar_v833 = v833;
        let v838: f64 = (if v762 { 0.0 } else { 0.0 });
        self.scalar_v838 = v838;
        let v839: bool = (!v762);
        self.scalar_v839 = v839;
        let v840: f64 = (if v839 { 0.0 } else { 0.0 });
        self.scalar_v840 = v840;
        let v846: f64 = (if v766 { 0.0 } else { 0.0 });
        self.scalar_v846 = v846;
        let v847: bool = (!v766);
        self.scalar_v847 = v847;
        let v848: f64 = (if v847 { 0.0 } else { 0.0 });
        self.scalar_v848 = v848;
        let v875: f64 = (-p.p29);
        self.scalar_v875 = v875;
        let v880: f64 = (p.p80 - 1.0);
        self.scalar_v880 = v880;
        let v1144: f64 = (p.p4 * v875);
        self.scalar_v1144 = v1144;
        let v1145: f64 = (p.p29 * p.p4);
        self.scalar_v1145 = v1145;
        let v1548: f64 = (p.p29 * v51);
        self.scalar_v1548 = v1548;
        let v1549: f64 = (v51 * v875);
        self.scalar_v1549 = v1549;
        let v1550: f64 = (-v1548);
        self.scalar_v1550 = v1550;
        let v1551: f64 = (-v1549);
        self.scalar_v1551 = v1551;
        let v1552: f64 = (v47 * v875);
        self.scalar_v1552 = v1552;
        let v1553: f64 = (p.p29 * v47);
        self.scalar_v1553 = v1553;
        let v1554: f64 = (-v1552);
        self.scalar_v1554 = v1554;
        let v1555: f64 = (v1550 - v1553);
        self.scalar_v1555 = v1555;
        let v1556: f64 = (2.0 * v1554);
        self.scalar_v1556 = v1556;
        let v1557: f64 = (2.0 * v1555);
        self.scalar_v1557 = v1557;
        let v1558: f64 = (2.0 * v1551);
        self.scalar_v1558 = v1558;
        let v1665: f64 = (p.p29 * v610);
        self.scalar_v1665 = v1665;
        let v1666: f64 = (v610 * v875);
        self.scalar_v1666 = v1666;
        let v1690: f64 = (p.p29 * v634);
        self.scalar_v1690 = v1690;
        let v1691: f64 = (v634 * v875);
        self.scalar_v1691 = v1691;
        let v1763: f64 = (p.p29 * v669);
        self.scalar_v1763 = v1763;
        let v1765: f64 = (v669 * v875);
        self.scalar_v1765 = v1765;
        let v1964: f64 = (if v572 { 1.0 } else { 0.0 });
        self.scalar_v1964 = v1964;
        let v1967: f64 = (1.0 / p.p33);
        self.scalar_v1967 = v1967;
        let v1968: f64 = (if v739 { v1967 } else { 0.0 });
        self.scalar_v1968 = v1968;
        let v1971: f64 = (-1.0 / p.p33);
        self.scalar_v1971 = v1971;
        let v1972: f64 = (if v801 { v1967 } else { 0.0 });
        self.scalar_v1972 = v1972;
        let v1973: f64 = (if v801 { v1971 } else { 0.0 });
        self.scalar_v1973 = v1973;
        let v1975: f64 = (1.0 / p.p35);
        self.scalar_v1975 = v1975;
        let v1976: f64 = (if v801 { v1975 } else { 0.0 });
        self.scalar_v1976 = v1976;
    }
}
