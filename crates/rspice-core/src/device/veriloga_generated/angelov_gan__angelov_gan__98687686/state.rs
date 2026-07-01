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
            params.p1 = 0.0;
            params.p2 = 0.0;
            params.p3 = 25.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 2.0;
            params.p7 = 0.0;
            params.p8 = 0.05;
            params.p9 = -0.2;
            params.p10 = 0.2;
            params.p11 = 0.8;
            params.p12 = 0.0;
            params.p13 = 0.0;
            params.p14 = 0.1;
            params.p15 = 1.0;
            params.p16 = 0.001;
            params.p17 = 0.0;
            params.p18 = 0.1;
            params.p19 = 4.0;
            params.p20 = 0.0;
            params.p21 = 50.0;
            params.p22 = 0.0;
            params.p23 = 0.2;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 0.2;
            params.p34 = 0.0;
            params.p35 = 0.2;
            params.p36 = 0.0;
            params.p37 = 1.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 1.0;
            params.p41 = 0.5;
            params.p42 = 5e-5;
            params.p43 = 15.0;
            params.p44 = 1.0;
            params.p45 = 0.8;
            params.p46 = 0.05;
            params.p47 = 0.05;
            params.p48 = 0.0;
            params.p49 = 0.05;
            params.p50 = 0.05;
            params.p51 = 0.05;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.1;
            params.p56 = 0.0;
            params.p57 = 1000.0;
            params.p58 = 10000.0;
            params.p59 = 0.0;
            params.p60 = 100000.0;
            params.p61 = 0.0;
            params.p62 = 1.0;
            params.p63 = 1e-15;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.001;
            params.p67 = 1e-6;
            params.p68 = -0.002;
            params.p69 = -0.002;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.001;
            params.p73 = 0.001;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.002;
            params.p77 = 0.001;
            params.p78 = 0.001;
            params.p79 = -0.001;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 10.0;
            params.p84 = 100.0;
            params.p85 = 0.5;
            params.p86 = 0.5;
            params.p87 = 1.0;
            params.p88 = 0.9;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 1.0;
            params.p92 = 1.0;
            params.p93 = 25.0;
            params.p94 = 0.1;
            params.p95 = 1.0;
            params.p96 = 1e-14;
            params.p97 = 60000.0;
            params.p98 = 0.3;
            params.p99 = 0.1;
            params.p100 = 27.0;
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
    pub nodes: [usize; 19],
    pub branches: [usize; 19],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 101]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 17]>,
    pub(crate) ddt_state_previous: Box<[f64; 17]>,
    pub(crate) ddt_state_older: Box<[f64; 17]>,
    pub(crate) ddt_state_initialized: Box<[bool; 17]>,
    pub(crate) ddt_derivative_current: Box<[f64; 17]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 17]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: bool,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: bool,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: bool,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: bool,
    pub(crate) scalar_v113: bool,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: bool,
    pub(crate) scalar_v117: bool,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v128: bool,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: bool,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v157: bool,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v221: bool,
    pub(crate) scalar_v223: bool,
    pub(crate) scalar_v225: bool,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v236: bool,
    pub(crate) scalar_v237: bool,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v288: bool,
    pub(crate) scalar_v289: bool,
    pub(crate) scalar_v290: bool,
    pub(crate) scalar_v328: bool,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v330: bool,
    pub(crate) scalar_v403: bool,
    pub(crate) scalar_v404: bool,
    pub(crate) scalar_v405: bool,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v425: bool,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v439: bool,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v455: bool,
    pub(crate) scalar_v465: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v473: bool,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v479: f64,
    pub(crate) scalar_v480: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v484: f64,
    pub(crate) scalar_v485: f64,
    pub(crate) scalar_v486: bool,
    pub(crate) scalar_v487: bool,
    pub(crate) scalar_v492: bool,
    pub(crate) scalar_v493: bool,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v505: f64,
    pub(crate) scalar_v506: f64,
    pub(crate) scalar_v520: f64,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v534: f64,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v547: bool,
    pub(crate) scalar_v548: bool,
    pub(crate) scalar_v549: bool,
    pub(crate) scalar_v550: bool,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v555: bool,
    pub(crate) scalar_v556: bool,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v567: bool,
    pub(crate) scalar_v568: bool,
    pub(crate) scalar_v569: bool,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v619: bool,
    pub(crate) scalar_v620: bool,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v625: f64,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v629: f64,
    pub(crate) scalar_v631: f64,
    pub(crate) scalar_v632: f64,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v654: f64,
    pub(crate) scalar_v666: bool,
    pub(crate) scalar_v667: bool,
    pub(crate) scalar_v668: bool,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v686: f64,
    pub(crate) scalar_v687: f64,
    pub(crate) scalar_v688: f64,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v723: bool,
    pub(crate) scalar_v724: f64,
    pub(crate) scalar_v725: bool,
    pub(crate) scalar_v726: bool,
    pub(crate) scalar_v727: bool,
    pub(crate) scalar_v728: bool,
    pub(crate) scalar_v729: f64,
    pub(crate) scalar_v730: bool,
    pub(crate) scalar_v731: f64,
    pub(crate) scalar_v732: bool,
    pub(crate) scalar_v733: f64,
    pub(crate) scalar_v734: bool,
    pub(crate) scalar_v735: f64,
    pub(crate) scalar_v736: bool,
    pub(crate) scalar_v737: bool,
    pub(crate) scalar_v738: bool,
    pub(crate) scalar_v739: bool,
    pub(crate) scalar_v740: bool,
    pub(crate) scalar_v741: f64,
    pub(crate) scalar_v742: bool,
    pub(crate) scalar_v743: bool,
    pub(crate) scalar_v744: bool,
    pub(crate) scalar_v745: bool,
    pub(crate) scalar_v746: f64,
    pub(crate) scalar_v747: bool,
    pub(crate) scalar_v750: f64,
    pub(crate) scalar_v751: f64,
    pub(crate) scalar_v752: f64,
    pub(crate) scalar_v755: f64,
    pub(crate) scalar_v756: f64,
    pub(crate) scalar_v769: f64,
    pub(crate) scalar_v770: bool,
    pub(crate) scalar_v771: bool,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v776: f64,
    pub(crate) scalar_v783: bool,
    pub(crate) scalar_v790: f64,
    pub(crate) scalar_v794: f64,
    pub(crate) scalar_v806: bool,
    pub(crate) scalar_v807: f64,
    pub(crate) scalar_v814: bool,
    pub(crate) scalar_v815: f64,
    pub(crate) scalar_v816: f64,
    pub(crate) scalar_v823: bool,
    pub(crate) scalar_v824: f64,
    pub(crate) scalar_v829: bool,
    pub(crate) scalar_v830: f64,
    pub(crate) scalar_v831: f64,
    pub(crate) scalar_v835: bool,
    pub(crate) scalar_v836: f64,
    pub(crate) scalar_v840: bool,
    pub(crate) scalar_v841: f64,
    pub(crate) scalar_v842: bool,
    pub(crate) scalar_v843: f64,
    pub(crate) scalar_v844: f64,
    pub(crate) scalar_v850: bool,
    pub(crate) scalar_v851: f64,
    pub(crate) scalar_v852: bool,
    pub(crate) scalar_v853: f64,
    pub(crate) scalar_v854: f64,
    pub(crate) scalar_v860: bool,
    pub(crate) scalar_v861: f64,
    pub(crate) scalar_v862: bool,
    pub(crate) scalar_v863: f64,
    pub(crate) scalar_v864: f64,
    pub(crate) scalar_v871: bool,
    pub(crate) scalar_v872: f64,
    pub(crate) scalar_v873: f64,
    pub(crate) scalar_v886: bool,
    pub(crate) scalar_v887: f64,
    pub(crate) scalar_v888: bool,
    pub(crate) scalar_v889: f64,
    pub(crate) scalar_v899: f64,
    pub(crate) scalar_v903: bool,
    pub(crate) scalar_v906: f64,
    pub(crate) scalar_v910: f64,
    pub(crate) scalar_v926: f64,
    pub(crate) scalar_v934: f64,
    pub(crate) scalar_v1081: f64,
    pub(crate) scalar_v1122: f64,
    pub(crate) scalar_v1153: f64,
    pub(crate) scalar_v1223: f64,
    pub(crate) scalar_v1358: f64,
    pub(crate) scalar_v1561: f64,
    pub(crate) scalar_v2050: f64,
    pub(crate) scalar_v2051: f64,
    pub(crate) scalar_v2155: f64,
    pub(crate) scalar_v2156: f64,
    pub(crate) scalar_v2157: f64,
    pub(crate) scalar_v2162: f64,
    pub(crate) scalar_v2177: f64,
    pub(crate) scalar_v2178: f64,
    pub(crate) scalar_v2179: f64,
    pub(crate) scalar_v2180: f64,
    pub(crate) scalar_v2197: f64,
    pub(crate) scalar_v2203: f64,
    pub(crate) scalar_v2222: f64,
    pub(crate) scalar_v2223: f64,
    pub(crate) scalar_v2224: f64,
    pub(crate) scalar_v2230: f64,
    pub(crate) scalar_v2235: f64,
    pub(crate) scalar_v2240: f64,
    pub(crate) scalar_v2241: f64,
    pub(crate) scalar_v2316: f64,
    pub(crate) scalar_v2322: f64,
    pub(crate) scalar_v2380: f64,
    pub(crate) scalar_v2393: f64,
    pub(crate) scalar_v2394: f64,
    pub(crate) scalar_v2395: f64,
    pub(crate) scalar_v2396: f64,
    pub(crate) scalar_v2401: f64,
    pub(crate) scalar_v2416: f64,
    pub(crate) scalar_v2417: f64,
    pub(crate) scalar_v2418: f64,
    pub(crate) scalar_v2431: f64,
    pub(crate) scalar_v2432: f64,
    pub(crate) scalar_v2433: f64,
    pub(crate) scalar_v2434: f64,
    pub(crate) scalar_v2491: f64,
    pub(crate) scalar_v2496: f64,
    pub(crate) scalar_v2647: f64,
    pub(crate) scalar_v2648: f64,
    pub(crate) scalar_v2673: f64,
    pub(crate) scalar_v2674: f64,
    pub(crate) scalar_v2675: f64,
    pub(crate) scalar_v2676: f64,
    pub(crate) scalar_v2677: f64,
    pub(crate) scalar_v2678: f64,
    pub(crate) scalar_v2679: f64,
    pub(crate) scalar_v2680: f64,
    pub(crate) scalar_v2681: f64,
    pub(crate) scalar_v2682: f64,
    pub(crate) scalar_v2683: f64,
    pub(crate) scalar_v2684: f64,
    pub(crate) scalar_v2685: f64,
    pub(crate) scalar_v2686: f64,
    pub(crate) scalar_v2709: f64,
    pub(crate) scalar_v2718: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
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
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v21: self.scalar_v21,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v45: self.scalar_v45,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v86: self.scalar_v86,
            scalar_v87: self.scalar_v87,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v97: self.scalar_v97,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v119: self.scalar_v119,
            scalar_v125: self.scalar_v125,
            scalar_v128: self.scalar_v128,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v160: self.scalar_v160,
            scalar_v163: self.scalar_v163,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v173: self.scalar_v173,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v182: self.scalar_v182,
            scalar_v184: self.scalar_v184,
            scalar_v190: self.scalar_v190,
            scalar_v201: self.scalar_v201,
            scalar_v216: self.scalar_v216,
            scalar_v221: self.scalar_v221,
            scalar_v223: self.scalar_v223,
            scalar_v225: self.scalar_v225,
            scalar_v228: self.scalar_v228,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v256: self.scalar_v256,
            scalar_v264: self.scalar_v264,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v405: self.scalar_v405,
            scalar_v414: self.scalar_v414,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v436: self.scalar_v436,
            scalar_v439: self.scalar_v439,
            scalar_v449: self.scalar_v449,
            scalar_v454: self.scalar_v454,
            scalar_v455: self.scalar_v455,
            scalar_v465: self.scalar_v465,
            scalar_v470: self.scalar_v470,
            scalar_v473: self.scalar_v473,
            scalar_v478: self.scalar_v478,
            scalar_v479: self.scalar_v479,
            scalar_v480: self.scalar_v480,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v484: self.scalar_v484,
            scalar_v485: self.scalar_v485,
            scalar_v486: self.scalar_v486,
            scalar_v487: self.scalar_v487,
            scalar_v492: self.scalar_v492,
            scalar_v493: self.scalar_v493,
            scalar_v501: self.scalar_v501,
            scalar_v505: self.scalar_v505,
            scalar_v506: self.scalar_v506,
            scalar_v520: self.scalar_v520,
            scalar_v523: self.scalar_v523,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v534: self.scalar_v534,
            scalar_v535: self.scalar_v535,
            scalar_v541: self.scalar_v541,
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
            scalar_v562: self.scalar_v562,
            scalar_v567: self.scalar_v567,
            scalar_v568: self.scalar_v568,
            scalar_v569: self.scalar_v569,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v625: self.scalar_v625,
            scalar_v626: self.scalar_v626,
            scalar_v629: self.scalar_v629,
            scalar_v631: self.scalar_v631,
            scalar_v632: self.scalar_v632,
            scalar_v644: self.scalar_v644,
            scalar_v654: self.scalar_v654,
            scalar_v666: self.scalar_v666,
            scalar_v667: self.scalar_v667,
            scalar_v668: self.scalar_v668,
            scalar_v675: self.scalar_v675,
            scalar_v681: self.scalar_v681,
            scalar_v685: self.scalar_v685,
            scalar_v686: self.scalar_v686,
            scalar_v687: self.scalar_v687,
            scalar_v688: self.scalar_v688,
            scalar_v689: self.scalar_v689,
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
            scalar_v750: self.scalar_v750,
            scalar_v751: self.scalar_v751,
            scalar_v752: self.scalar_v752,
            scalar_v755: self.scalar_v755,
            scalar_v756: self.scalar_v756,
            scalar_v769: self.scalar_v769,
            scalar_v770: self.scalar_v770,
            scalar_v771: self.scalar_v771,
            scalar_v773: self.scalar_v773,
            scalar_v776: self.scalar_v776,
            scalar_v783: self.scalar_v783,
            scalar_v790: self.scalar_v790,
            scalar_v794: self.scalar_v794,
            scalar_v806: self.scalar_v806,
            scalar_v807: self.scalar_v807,
            scalar_v814: self.scalar_v814,
            scalar_v815: self.scalar_v815,
            scalar_v816: self.scalar_v816,
            scalar_v823: self.scalar_v823,
            scalar_v824: self.scalar_v824,
            scalar_v829: self.scalar_v829,
            scalar_v830: self.scalar_v830,
            scalar_v831: self.scalar_v831,
            scalar_v835: self.scalar_v835,
            scalar_v836: self.scalar_v836,
            scalar_v840: self.scalar_v840,
            scalar_v841: self.scalar_v841,
            scalar_v842: self.scalar_v842,
            scalar_v843: self.scalar_v843,
            scalar_v844: self.scalar_v844,
            scalar_v850: self.scalar_v850,
            scalar_v851: self.scalar_v851,
            scalar_v852: self.scalar_v852,
            scalar_v853: self.scalar_v853,
            scalar_v854: self.scalar_v854,
            scalar_v860: self.scalar_v860,
            scalar_v861: self.scalar_v861,
            scalar_v862: self.scalar_v862,
            scalar_v863: self.scalar_v863,
            scalar_v864: self.scalar_v864,
            scalar_v871: self.scalar_v871,
            scalar_v872: self.scalar_v872,
            scalar_v873: self.scalar_v873,
            scalar_v886: self.scalar_v886,
            scalar_v887: self.scalar_v887,
            scalar_v888: self.scalar_v888,
            scalar_v889: self.scalar_v889,
            scalar_v899: self.scalar_v899,
            scalar_v903: self.scalar_v903,
            scalar_v906: self.scalar_v906,
            scalar_v910: self.scalar_v910,
            scalar_v926: self.scalar_v926,
            scalar_v934: self.scalar_v934,
            scalar_v1081: self.scalar_v1081,
            scalar_v1122: self.scalar_v1122,
            scalar_v1153: self.scalar_v1153,
            scalar_v1223: self.scalar_v1223,
            scalar_v1358: self.scalar_v1358,
            scalar_v1561: self.scalar_v1561,
            scalar_v2050: self.scalar_v2050,
            scalar_v2051: self.scalar_v2051,
            scalar_v2155: self.scalar_v2155,
            scalar_v2156: self.scalar_v2156,
            scalar_v2157: self.scalar_v2157,
            scalar_v2162: self.scalar_v2162,
            scalar_v2177: self.scalar_v2177,
            scalar_v2178: self.scalar_v2178,
            scalar_v2179: self.scalar_v2179,
            scalar_v2180: self.scalar_v2180,
            scalar_v2197: self.scalar_v2197,
            scalar_v2203: self.scalar_v2203,
            scalar_v2222: self.scalar_v2222,
            scalar_v2223: self.scalar_v2223,
            scalar_v2224: self.scalar_v2224,
            scalar_v2230: self.scalar_v2230,
            scalar_v2235: self.scalar_v2235,
            scalar_v2240: self.scalar_v2240,
            scalar_v2241: self.scalar_v2241,
            scalar_v2316: self.scalar_v2316,
            scalar_v2322: self.scalar_v2322,
            scalar_v2380: self.scalar_v2380,
            scalar_v2393: self.scalar_v2393,
            scalar_v2394: self.scalar_v2394,
            scalar_v2395: self.scalar_v2395,
            scalar_v2396: self.scalar_v2396,
            scalar_v2401: self.scalar_v2401,
            scalar_v2416: self.scalar_v2416,
            scalar_v2417: self.scalar_v2417,
            scalar_v2418: self.scalar_v2418,
            scalar_v2431: self.scalar_v2431,
            scalar_v2432: self.scalar_v2432,
            scalar_v2433: self.scalar_v2433,
            scalar_v2434: self.scalar_v2434,
            scalar_v2491: self.scalar_v2491,
            scalar_v2496: self.scalar_v2496,
            scalar_v2647: self.scalar_v2647,
            scalar_v2648: self.scalar_v2648,
            scalar_v2673: self.scalar_v2673,
            scalar_v2674: self.scalar_v2674,
            scalar_v2675: self.scalar_v2675,
            scalar_v2676: self.scalar_v2676,
            scalar_v2677: self.scalar_v2677,
            scalar_v2678: self.scalar_v2678,
            scalar_v2679: self.scalar_v2679,
            scalar_v2680: self.scalar_v2680,
            scalar_v2681: self.scalar_v2681,
            scalar_v2682: self.scalar_v2682,
            scalar_v2683: self.scalar_v2683,
            scalar_v2684: self.scalar_v2684,
            scalar_v2685: self.scalar_v2685,
            scalar_v2686: self.scalar_v2686,
            scalar_v2709: self.scalar_v2709,
            scalar_v2718: self.scalar_v2718,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 14;
    pub const NODE_COUNT: usize = 19;
    pub const INTERNAL_NODE_NAMES: [&str; 14] = ["di", "dii", "gi", "si", "sii", "gdi", "gsi", "gii", "ggi", "bi", "xt1", "xt2", "ia", "ib"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 101;
    pub const VARIABLE_COUNT: usize = 145;
    pub const DDT_STATE_COUNT: usize = 17;
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
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: false,
            scalar_v21: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: false,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v41: 0.0,
            scalar_v42: false,
            scalar_v45: 0.0,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v97: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v110: 0.0,
            scalar_v111: false,
            scalar_v113: false,
            scalar_v114: false,
            scalar_v115: 0.0,
            scalar_v116: false,
            scalar_v117: false,
            scalar_v119: 0.0,
            scalar_v125: 0.0,
            scalar_v128: false,
            scalar_v148: 0.0,
            scalar_v149: false,
            scalar_v150: 0.0,
            scalar_v151: false,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v157: false,
            scalar_v158: 0.0,
            scalar_v160: 0.0,
            scalar_v163: 0.0,
            scalar_v165: 0.0,
            scalar_v166: 0.0,
            scalar_v173: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v182: 0.0,
            scalar_v184: 0.0,
            scalar_v190: 0.0,
            scalar_v201: 0.0,
            scalar_v216: 0.0,
            scalar_v221: false,
            scalar_v223: false,
            scalar_v225: false,
            scalar_v228: 0.0,
            scalar_v236: false,
            scalar_v237: false,
            scalar_v256: 0.0,
            scalar_v264: 0.0,
            scalar_v288: false,
            scalar_v289: false,
            scalar_v290: false,
            scalar_v328: false,
            scalar_v329: false,
            scalar_v330: false,
            scalar_v403: false,
            scalar_v404: false,
            scalar_v405: false,
            scalar_v414: 0.0,
            scalar_v425: false,
            scalar_v426: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v436: 0.0,
            scalar_v439: false,
            scalar_v449: 0.0,
            scalar_v454: 0.0,
            scalar_v455: false,
            scalar_v465: 0.0,
            scalar_v470: 0.0,
            scalar_v473: false,
            scalar_v478: 0.0,
            scalar_v479: 0.0,
            scalar_v480: 0.0,
            scalar_v481: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v484: 0.0,
            scalar_v485: 0.0,
            scalar_v486: false,
            scalar_v487: false,
            scalar_v492: false,
            scalar_v493: false,
            scalar_v501: 0.0,
            scalar_v505: 0.0,
            scalar_v506: 0.0,
            scalar_v520: 0.0,
            scalar_v523: 0.0,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v534: 0.0,
            scalar_v535: 0.0,
            scalar_v541: 0.0,
            scalar_v547: false,
            scalar_v548: false,
            scalar_v549: false,
            scalar_v550: false,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v554: 0.0,
            scalar_v555: false,
            scalar_v556: false,
            scalar_v562: 0.0,
            scalar_v567: false,
            scalar_v568: false,
            scalar_v569: false,
            scalar_v618: false,
            scalar_v619: false,
            scalar_v620: false,
            scalar_v621: 0.0,
            scalar_v625: 0.0,
            scalar_v626: 0.0,
            scalar_v629: 0.0,
            scalar_v631: 0.0,
            scalar_v632: 0.0,
            scalar_v644: 0.0,
            scalar_v654: 0.0,
            scalar_v666: false,
            scalar_v667: false,
            scalar_v668: false,
            scalar_v675: 0.0,
            scalar_v681: 0.0,
            scalar_v685: 0.0,
            scalar_v686: 0.0,
            scalar_v687: 0.0,
            scalar_v688: 0.0,
            scalar_v689: 0.0,
            scalar_v723: false,
            scalar_v724: 0.0,
            scalar_v725: false,
            scalar_v726: false,
            scalar_v727: false,
            scalar_v728: false,
            scalar_v729: 0.0,
            scalar_v730: false,
            scalar_v731: 0.0,
            scalar_v732: false,
            scalar_v733: 0.0,
            scalar_v734: false,
            scalar_v735: 0.0,
            scalar_v736: false,
            scalar_v737: false,
            scalar_v738: false,
            scalar_v739: false,
            scalar_v740: false,
            scalar_v741: 0.0,
            scalar_v742: false,
            scalar_v743: false,
            scalar_v744: false,
            scalar_v745: false,
            scalar_v746: 0.0,
            scalar_v747: false,
            scalar_v750: 0.0,
            scalar_v751: 0.0,
            scalar_v752: 0.0,
            scalar_v755: 0.0,
            scalar_v756: 0.0,
            scalar_v769: 0.0,
            scalar_v770: false,
            scalar_v771: false,
            scalar_v773: 0.0,
            scalar_v776: 0.0,
            scalar_v783: false,
            scalar_v790: 0.0,
            scalar_v794: 0.0,
            scalar_v806: false,
            scalar_v807: 0.0,
            scalar_v814: false,
            scalar_v815: 0.0,
            scalar_v816: 0.0,
            scalar_v823: false,
            scalar_v824: 0.0,
            scalar_v829: false,
            scalar_v830: 0.0,
            scalar_v831: 0.0,
            scalar_v835: false,
            scalar_v836: 0.0,
            scalar_v840: false,
            scalar_v841: 0.0,
            scalar_v842: false,
            scalar_v843: 0.0,
            scalar_v844: 0.0,
            scalar_v850: false,
            scalar_v851: 0.0,
            scalar_v852: false,
            scalar_v853: 0.0,
            scalar_v854: 0.0,
            scalar_v860: false,
            scalar_v861: 0.0,
            scalar_v862: false,
            scalar_v863: 0.0,
            scalar_v864: 0.0,
            scalar_v871: false,
            scalar_v872: 0.0,
            scalar_v873: 0.0,
            scalar_v886: false,
            scalar_v887: 0.0,
            scalar_v888: false,
            scalar_v889: 0.0,
            scalar_v899: 0.0,
            scalar_v903: false,
            scalar_v906: 0.0,
            scalar_v910: 0.0,
            scalar_v926: 0.0,
            scalar_v934: 0.0,
            scalar_v1081: 0.0,
            scalar_v1122: 0.0,
            scalar_v1153: 0.0,
            scalar_v1223: 0.0,
            scalar_v1358: 0.0,
            scalar_v1561: 0.0,
            scalar_v2050: 0.0,
            scalar_v2051: 0.0,
            scalar_v2155: 0.0,
            scalar_v2156: 0.0,
            scalar_v2157: 0.0,
            scalar_v2162: 0.0,
            scalar_v2177: 0.0,
            scalar_v2178: 0.0,
            scalar_v2179: 0.0,
            scalar_v2180: 0.0,
            scalar_v2197: 0.0,
            scalar_v2203: 0.0,
            scalar_v2222: 0.0,
            scalar_v2223: 0.0,
            scalar_v2224: 0.0,
            scalar_v2230: 0.0,
            scalar_v2235: 0.0,
            scalar_v2240: 0.0,
            scalar_v2241: 0.0,
            scalar_v2316: 0.0,
            scalar_v2322: 0.0,
            scalar_v2380: 0.0,
            scalar_v2393: 0.0,
            scalar_v2394: 0.0,
            scalar_v2395: 0.0,
            scalar_v2396: 0.0,
            scalar_v2401: 0.0,
            scalar_v2416: 0.0,
            scalar_v2417: 0.0,
            scalar_v2418: 0.0,
            scalar_v2431: 0.0,
            scalar_v2432: 0.0,
            scalar_v2433: 0.0,
            scalar_v2434: 0.0,
            scalar_v2491: 0.0,
            scalar_v2496: 0.0,
            scalar_v2647: 0.0,
            scalar_v2648: 0.0,
            scalar_v2673: 0.0,
            scalar_v2674: 0.0,
            scalar_v2675: 0.0,
            scalar_v2676: 0.0,
            scalar_v2677: 0.0,
            scalar_v2678: 0.0,
            scalar_v2679: 0.0,
            scalar_v2680: 0.0,
            scalar_v2681: 0.0,
            scalar_v2682: 0.0,
            scalar_v2683: 0.0,
            scalar_v2684: 0.0,
            scalar_v2685: 0.0,
            scalar_v2686: 0.0,
            scalar_v2709: 0.0,
            scalar_v2718: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
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
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v21,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v30,
            scalar_v31,
            scalar_v41,
            scalar_v42,
            scalar_v45,
            scalar_v50,
            scalar_v51,
            scalar_v56,
            scalar_v57,
            scalar_v62,
            scalar_v63,
            scalar_v68,
            scalar_v69,
            scalar_v74,
            scalar_v75,
            scalar_v80,
            scalar_v81,
            scalar_v86,
            scalar_v87,
            scalar_v91,
            scalar_v92,
            scalar_v97,
            scalar_v100,
            scalar_v101,
            scalar_v105,
            scalar_v106,
            scalar_v110,
            scalar_v111,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v119,
            scalar_v125,
            scalar_v128,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v153,
            scalar_v154,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v163,
            scalar_v165,
            scalar_v166,
            scalar_v173,
            scalar_v177,
            scalar_v178,
            scalar_v182,
            scalar_v184,
            scalar_v190,
            scalar_v201,
            scalar_v216,
            scalar_v221,
            scalar_v223,
            scalar_v225,
            scalar_v228,
            scalar_v236,
            scalar_v237,
            scalar_v256,
            scalar_v264,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v414,
            scalar_v425,
            scalar_v426,
            scalar_v431,
            scalar_v432,
            scalar_v436,
            scalar_v439,
            scalar_v449,
            scalar_v454,
            scalar_v455,
            scalar_v465,
            scalar_v470,
            scalar_v473,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v492,
            scalar_v493,
            scalar_v501,
            scalar_v505,
            scalar_v506,
            scalar_v520,
            scalar_v523,
            scalar_v528,
            scalar_v529,
            scalar_v534,
            scalar_v535,
            scalar_v541,
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
            scalar_v562,
            scalar_v567,
            scalar_v568,
            scalar_v569,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v625,
            scalar_v626,
            scalar_v629,
            scalar_v631,
            scalar_v632,
            scalar_v644,
            scalar_v654,
            scalar_v666,
            scalar_v667,
            scalar_v668,
            scalar_v675,
            scalar_v681,
            scalar_v685,
            scalar_v686,
            scalar_v687,
            scalar_v688,
            scalar_v689,
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
            scalar_v750,
            scalar_v751,
            scalar_v752,
            scalar_v755,
            scalar_v756,
            scalar_v769,
            scalar_v770,
            scalar_v771,
            scalar_v773,
            scalar_v776,
            scalar_v783,
            scalar_v790,
            scalar_v794,
            scalar_v806,
            scalar_v807,
            scalar_v814,
            scalar_v815,
            scalar_v816,
            scalar_v823,
            scalar_v824,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v835,
            scalar_v836,
            scalar_v840,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v844,
            scalar_v850,
            scalar_v851,
            scalar_v852,
            scalar_v853,
            scalar_v854,
            scalar_v860,
            scalar_v861,
            scalar_v862,
            scalar_v863,
            scalar_v864,
            scalar_v871,
            scalar_v872,
            scalar_v873,
            scalar_v886,
            scalar_v887,
            scalar_v888,
            scalar_v889,
            scalar_v899,
            scalar_v903,
            scalar_v906,
            scalar_v910,
            scalar_v926,
            scalar_v934,
            scalar_v1081,
            scalar_v1122,
            scalar_v1153,
            scalar_v1223,
            scalar_v1358,
            scalar_v1561,
            scalar_v2050,
            scalar_v2051,
            scalar_v2155,
            scalar_v2156,
            scalar_v2157,
            scalar_v2162,
            scalar_v2177,
            scalar_v2178,
            scalar_v2179,
            scalar_v2180,
            scalar_v2197,
            scalar_v2203,
            scalar_v2222,
            scalar_v2223,
            scalar_v2224,
            scalar_v2230,
            scalar_v2235,
            scalar_v2240,
            scalar_v2241,
            scalar_v2316,
            scalar_v2322,
            scalar_v2380,
            scalar_v2393,
            scalar_v2394,
            scalar_v2395,
            scalar_v2396,
            scalar_v2401,
            scalar_v2416,
            scalar_v2417,
            scalar_v2418,
            scalar_v2431,
            scalar_v2432,
            scalar_v2433,
            scalar_v2434,
            scalar_v2491,
            scalar_v2496,
            scalar_v2647,
            scalar_v2648,
            scalar_v2673,
            scalar_v2674,
            scalar_v2675,
            scalar_v2676,
            scalar_v2677,
            scalar_v2678,
            scalar_v2679,
            scalar_v2680,
            scalar_v2681,
            scalar_v2682,
            scalar_v2683,
            scalar_v2684,
            scalar_v2685,
            scalar_v2686,
            scalar_v2709,
            scalar_v2718,
            scalar_v22,
            scalar_v23,
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
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v21,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v30,
            scalar_v31,
            scalar_v41,
            scalar_v42,
            scalar_v45,
            scalar_v50,
            scalar_v51,
            scalar_v56,
            scalar_v57,
            scalar_v62,
            scalar_v63,
            scalar_v68,
            scalar_v69,
            scalar_v74,
            scalar_v75,
            scalar_v80,
            scalar_v81,
            scalar_v86,
            scalar_v87,
            scalar_v91,
            scalar_v92,
            scalar_v97,
            scalar_v100,
            scalar_v101,
            scalar_v105,
            scalar_v106,
            scalar_v110,
            scalar_v111,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v119,
            scalar_v125,
            scalar_v128,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v153,
            scalar_v154,
            scalar_v157,
            scalar_v158,
            scalar_v160,
            scalar_v163,
            scalar_v165,
            scalar_v166,
            scalar_v173,
            scalar_v177,
            scalar_v178,
            scalar_v182,
            scalar_v184,
            scalar_v190,
            scalar_v201,
            scalar_v216,
            scalar_v221,
            scalar_v223,
            scalar_v225,
            scalar_v228,
            scalar_v236,
            scalar_v237,
            scalar_v256,
            scalar_v264,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v414,
            scalar_v425,
            scalar_v426,
            scalar_v431,
            scalar_v432,
            scalar_v436,
            scalar_v439,
            scalar_v449,
            scalar_v454,
            scalar_v455,
            scalar_v465,
            scalar_v470,
            scalar_v473,
            scalar_v478,
            scalar_v479,
            scalar_v480,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v484,
            scalar_v485,
            scalar_v486,
            scalar_v487,
            scalar_v492,
            scalar_v493,
            scalar_v501,
            scalar_v505,
            scalar_v506,
            scalar_v520,
            scalar_v523,
            scalar_v528,
            scalar_v529,
            scalar_v534,
            scalar_v535,
            scalar_v541,
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
            scalar_v562,
            scalar_v567,
            scalar_v568,
            scalar_v569,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v625,
            scalar_v626,
            scalar_v629,
            scalar_v631,
            scalar_v632,
            scalar_v644,
            scalar_v654,
            scalar_v666,
            scalar_v667,
            scalar_v668,
            scalar_v675,
            scalar_v681,
            scalar_v685,
            scalar_v686,
            scalar_v687,
            scalar_v688,
            scalar_v689,
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
            scalar_v750,
            scalar_v751,
            scalar_v752,
            scalar_v755,
            scalar_v756,
            scalar_v769,
            scalar_v770,
            scalar_v771,
            scalar_v773,
            scalar_v776,
            scalar_v783,
            scalar_v790,
            scalar_v794,
            scalar_v806,
            scalar_v807,
            scalar_v814,
            scalar_v815,
            scalar_v816,
            scalar_v823,
            scalar_v824,
            scalar_v829,
            scalar_v830,
            scalar_v831,
            scalar_v835,
            scalar_v836,
            scalar_v840,
            scalar_v841,
            scalar_v842,
            scalar_v843,
            scalar_v844,
            scalar_v850,
            scalar_v851,
            scalar_v852,
            scalar_v853,
            scalar_v854,
            scalar_v860,
            scalar_v861,
            scalar_v862,
            scalar_v863,
            scalar_v864,
            scalar_v871,
            scalar_v872,
            scalar_v873,
            scalar_v886,
            scalar_v887,
            scalar_v888,
            scalar_v889,
            scalar_v899,
            scalar_v903,
            scalar_v906,
            scalar_v910,
            scalar_v926,
            scalar_v934,
            scalar_v1081,
            scalar_v1122,
            scalar_v1153,
            scalar_v1223,
            scalar_v1358,
            scalar_v1561,
            scalar_v2050,
            scalar_v2051,
            scalar_v2155,
            scalar_v2156,
            scalar_v2157,
            scalar_v2162,
            scalar_v2177,
            scalar_v2178,
            scalar_v2179,
            scalar_v2180,
            scalar_v2197,
            scalar_v2203,
            scalar_v2222,
            scalar_v2223,
            scalar_v2224,
            scalar_v2230,
            scalar_v2235,
            scalar_v2240,
            scalar_v2241,
            scalar_v2316,
            scalar_v2322,
            scalar_v2380,
            scalar_v2393,
            scalar_v2394,
            scalar_v2395,
            scalar_v2396,
            scalar_v2401,
            scalar_v2416,
            scalar_v2417,
            scalar_v2418,
            scalar_v2431,
            scalar_v2432,
            scalar_v2433,
            scalar_v2434,
            scalar_v2491,
            scalar_v2496,
            scalar_v2647,
            scalar_v2648,
            scalar_v2673,
            scalar_v2674,
            scalar_v2675,
            scalar_v2676,
            scalar_v2677,
            scalar_v2678,
            scalar_v2679,
            scalar_v2680,
            scalar_v2681,
            scalar_v2682,
            scalar_v2683,
            scalar_v2684,
            scalar_v2685,
            scalar_v2686,
            scalar_v2709,
            scalar_v2718,
            scalar_v22,
            scalar_v23,
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
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "selft" => { validate_parameter("Selft", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "temp" => { validate_parameter("Temp", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "idsmod" => { validate_parameter("Idsmod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igmod" => { validate_parameter("Igmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "capmod" => { validate_parameter("Capmod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noimod" => { validate_parameter("Noimod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ipk0" => { validate_finite_parameter("Ipk0", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpks" => { validate_finite_parameter("Vpks", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvpks" => { validate_finite_parameter("Dvpks", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p1" => { validate_finite_parameter("P1", value)?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p2" => { validate_finite_parameter("P2", value)?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p3" => { validate_finite_parameter("P3", value)?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphar" => { validate_finite_parameter("Alphar", value)?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphas" => { validate_finite_parameter("Alphas", value)?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lambda" => { validate_finite_parameter("Lambda", value)?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvg" => { validate_finite_parameter("Lvg", value)?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "b1" => { validate_finite_parameter("B1", value)?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "b2" => { validate_finite_parameter("B2", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsb0" => { validate_finite_parameter("Lsb0", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtr" => { validate_finite_parameter("Vtr", value)?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vsb2" => { validate_finite_parameter("Vsb2", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ebd" => { validate_finite_parameter("Ebd", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cds" => { validate_parameter("Cds", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgspi" => { validate_finite_parameter("Cgspi", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgs0" => { validate_finite_parameter("Cgs0", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdpi" => { validate_finite_parameter("Cgdpi", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdpe" => { validate_parameter("Cgdpe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgd0" => { validate_finite_parameter("Cgd0", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p10" => { validate_parameter("P10", value, Some((-100.0, "-100.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p11" => { validate_parameter("P11", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p20" => { validate_parameter("P20", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p21" => { validate_parameter("P21", value, Some((0.01, "0.01")), false, Some((5.0, "5.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p30" => { validate_parameter("P30", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p31" => { validate_parameter("P31", value, Some((0.01, "0.01")), false, Some((5.0, "5.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p40" => { validate_parameter("P40", value, Some((-100.0, "-100.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p41" => { validate_parameter("P41", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p111" => { validate_parameter("P111", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p222" => { validate_parameter("P222", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p10pk" => { validate_parameter("P10pk", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[(0.0, "0.0")])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "m" => { validate_finite_parameter("m", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ij" => { validate_parameter("Ij", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg" => { validate_parameter("Pg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ne" => { validate_parameter("Ne", value, Some((1.0, "1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vjg" => { validate_parameter("Vjg", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[(0.0, "0.0")])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rg" => { validate_parameter("Rg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd" => { validate_parameter("Rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd2" => { validate_parameter("Rd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ri" => { validate_parameter("Ri", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("Rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rgd" => { validate_parameter("Rgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ld" => { validate_parameter("Ld", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ls" => { validate_parameter("Ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lg" => { validate_parameter("Lg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldc" => { validate_parameter("Ldc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tau" => { validate_parameter("Tau", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcmin" => { validate_parameter("Rcmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rc" => { validate_parameter("Rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "crf" => { validate_finite_parameter("Crf", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcin" => { validate_parameter("Rcin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "crfin" => { validate_parameter("Crfin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdel" => { validate_parameter("Rdel", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cdel" => { validate_parameter("Cdel", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbgate" => { validate_finite_parameter("Kbgate", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "krfdc" => { validate_parameter("KRFDC", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rtherm" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctherm" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcipk0" => { validate_parameter("Tcipk0", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcp1" => { validate_parameter("Tcp1", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcp3" => { validate_parameter("Tcp3", value, Some((-0.05, "-0.05")), false, Some((0.05, "0.05")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcp10" => { validate_parameter("Tcp10", value, Some((-0.01, "-0.01")), false, Some((0.01, "0.01")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccgs0" => { validate_parameter("Tccgs0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccgd0" => { validate_parameter("Tccgd0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrc" => { validate_finite_parameter("Tcrc", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccrf" => { validate_finite_parameter("Tccrf", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrs" => { validate_parameter("Tcrs", value, Some((0.0, "0.0")), false, Some((0.1, "0.1")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrtherm" => { validate_parameter("TcRtherm", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvpk" => { validate_parameter("TcVpk", value, Some((-0.1, "-0.1")), false, Some((0.1, "0.1")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvjg" => { validate_finite_parameter("TcVjg", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tclsb0" => { validate_parameter("Tclsb0", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvtr" => { validate_parameter("TcVtr", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbdgate" => { validate_parameter("Kbdgate", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbdgs" => { validate_parameter("Vbdgs", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbdgd" => { validate_parameter("Vbdgd", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbdg" => { validate_parameter("Pbdg", value, Some((0.4, "0.4")), false, Some((1.0, "1.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noiser" => { validate_parameter("NoiseR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noisep" => { validate_parameter("NoiseP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noisec" => { validate_parameter("NoiseC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnc" => { validate_parameter("Fnc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("Kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("Af", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ffe" => { validate_parameter("Ffe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td" => { validate_parameter("Td", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td1" => { validate_finite_parameter("Td1", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmn" => { validate_finite_parameter("Tmn", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "klf" => { validate_finite_parameter("Klf", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgr" => { validate_finite_parameter("Fgr", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "np" => { validate_finite_parameter("Np", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lw" => { validate_finite_parameter("Lw", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("Tnom", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'angelov_gan'", name)),
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
        let param_given = self.param_given.as_ref();
        let v14: f64 = if param_given[3] { 1.0 } else { 0.0 };
        self.scalar_v14 = v14;
        let v15: f64 = p.p3;
        self.scalar_v15 = v15;
        let v17: f64 = (p.p3 + 273.15);
        self.scalar_v17 = v17;
        let v18: f64 = (if (if param_given[3] { 1.0 } else { 0.0 } != 0.0) { v17 } else { 0.0 });
        self.scalar_v18 = v18;
        let v19: bool = (!(if param_given[3] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v19 = v19;
        let v21: f64 = p.p2;
        self.scalar_v21 = v21;
        let v24: f64 = if param_given[100] { 1.0 } else { 0.0 };
        self.scalar_v24 = v24;
        let v25: f64 = p.p100;
        self.scalar_v25 = v25;
        let v26: f64 = (273.15 + p.p100);
        self.scalar_v26 = v26;
        let v27: f64 = (if (if param_given[100] { 1.0 } else { 0.0 } != 0.0) { v26 } else { 0.0 });
        self.scalar_v27 = v27;
        let v28: bool = (!(if param_given[100] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v28 = v28;
        let v30: f64 = (if v28 { 300.15 } else { v27 });
        self.scalar_v30 = v30;
        let v31: f64 = p.p1;
        self.scalar_v31 = v31;
        let v41: f64 = p.p66;
        self.scalar_v41 = v41;
        let v42: bool = (p.p66 > 0.0);
        self.scalar_v42 = v42;
        let v45: f64 = p.p77;
        self.scalar_v45 = v45;
        let v50: f64 = p.p8;
        self.scalar_v50 = v50;
        let v51: f64 = p.p68;
        self.scalar_v51 = v51;
        let v56: f64 = p.p20;
        self.scalar_v56 = v56;
        let v57: f64 = p.p80;
        self.scalar_v57 = v57;
        let v62: f64 = p.p26;
        self.scalar_v62 = v62;
        let v63: f64 = p.p72;
        self.scalar_v63 = v63;
        let v68: f64 = p.p29;
        self.scalar_v68 = v68;
        let v69: f64 = p.p73;
        self.scalar_v69 = v69;
        let v74: f64 = p.p58;
        self.scalar_v74 = v74;
        let v75: f64 = p.p74;
        self.scalar_v75 = v75;
        let v80: f64 = p.p59;
        self.scalar_v80 = v80;
        let v81: f64 = p.p75;
        self.scalar_v81 = v81;
        let v86: f64 = p.p9;
        self.scalar_v86 = v86;
        let v87: f64 = p.p78;
        self.scalar_v87 = v87;
        let v91: f64 = p.p30;
        self.scalar_v91 = v91;
        let v92: f64 = p.p71;
        self.scalar_v92 = v92;
        let v97: f64 = p.p36;
        self.scalar_v97 = v97;
        let v100: f64 = p.p45;
        self.scalar_v100 = v100;
        let v101: f64 = p.p79;
        self.scalar_v101 = v101;
        let v105: f64 = p.p21;
        self.scalar_v105 = v105;
        let v106: f64 = p.p81;
        self.scalar_v106 = v106;
        let v110: f64 = p.p4;
        self.scalar_v110 = v110;
        let v111: bool = (1.0 == p.p4);
        self.scalar_v111 = v111;
        let v113: bool = (p.p4 == 4.0);
        self.scalar_v113 = v113;
        let v114: bool = (v111 || v113);
        self.scalar_v114 = v114;
        let v115: f64 = p.p6;
        self.scalar_v115 = v115;
        let v116: bool = (4.0 == p.p6);
        self.scalar_v116 = v116;
        let v117: bool = (v114 && v116);
        self.scalar_v117 = v117;
        let v119: f64 = p.p62;
        self.scalar_v119 = v119;
        let v125: f64 = p.p63;
        self.scalar_v125 = v125;
        let v128: bool = (!v117);
        self.scalar_v128 = v128;
        let v148: f64 = if param_given[43] { 1.0 } else { 0.0 };
        self.scalar_v148 = v148;
        let v149: bool = (!(if param_given[43] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v149 = v149;
        let v150: f64 = if param_given[44] { 1.0 } else { 0.0 };
        self.scalar_v150 = v150;
        let v151: bool = (v149 && (if param_given[44] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v151 = v151;
        let v153: f64 = p.p44;
        self.scalar_v153 = v153;
        let v154: f64 = (0.5 / p.p44);
        self.scalar_v154 = v154;
        let v157: bool = (!v151);
        self.scalar_v157 = v157;
        let v158: f64 = p.p43;
        self.scalar_v158 = v158;
        let v160: f64 = p.p19;
        self.scalar_v160 = v160;
        let v163: f64 = p.p64;
        self.scalar_v163 = v163;
        let v165: f64 = p.p11;
        self.scalar_v165 = v165;
        let v166: f64 = p.p18;
        self.scalar_v166 = v166;
        let v173: f64 = p.p69;
        self.scalar_v173 = v173;
        let v177: f64 = p.p13;
        self.scalar_v177 = v177;
        let v178: f64 = p.p70;
        self.scalar_v178 = v178;
        let v182: f64 = p.p10;
        self.scalar_v182 = v182;
        let v184: f64 = p.p15;
        self.scalar_v184 = v184;
        let v190: f64 = p.p22;
        self.scalar_v190 = v190;
        let v201: f64 = p.p12;
        self.scalar_v201 = v201;
        let v216: f64 = p.p14;
        self.scalar_v216 = v216;
        let v221: bool = (0.0 == p.p4);
        self.scalar_v221 = v221;
        let v223: bool = (p.p4 == 2.0);
        self.scalar_v223 = v223;
        let v225: bool = (p.p4 == 3.0);
        self.scalar_v225 = v225;
        let v228: f64 = p.p16;
        self.scalar_v228 = v228;
        let v236: bool = (!v221);
        self.scalar_v236 = v236;
        let v237: bool = (v111 && v236);
        self.scalar_v237 = v237;
        let v256: f64 = p.p17;
        self.scalar_v256 = v256;
        let v264: f64 = p.p23;
        self.scalar_v264 = v264;
        let v288: bool = (v111 || v221);
        self.scalar_v288 = v288;
        let v289: bool = (!v288);
        self.scalar_v289 = v289;
        let v290: bool = (v223 && v289);
        self.scalar_v290 = v290;
        let v328: bool = (v223 || v288);
        self.scalar_v328 = v328;
        let v329: bool = (!v328);
        self.scalar_v329 = v329;
        let v330: bool = (v225 && v329);
        self.scalar_v330 = v330;
        let v403: bool = (v225 || v328);
        self.scalar_v403 = v403;
        let v404: bool = (!v403);
        self.scalar_v404 = v404;
        let v405: bool = (v113 && v404);
        self.scalar_v405 = v405;
        let v414: f64 = p.p65;
        self.scalar_v414 = v414;
        let v425: bool = (v113 || v288);
        self.scalar_v425 = v425;
        let v426: f64 = p.p57;
        self.scalar_v426 = v426;
        let v431: f64 = p.p47;
        self.scalar_v431 = v431;
        let v432: f64 = p.p48;
        self.scalar_v432 = v432;
        let v436: f64 = p.p50;
        self.scalar_v436 = v436;
        let v439: bool = (!v425);
        self.scalar_v439 = v439;
        let v449: f64 = p.p76;
        self.scalar_v449 = v449;
        let v454: f64 = p.p5;
        self.scalar_v454 = v454;
        let v455: bool = (0.0 == p.p5);
        self.scalar_v455 = v455;
        let v465: f64 = p.p83;
        self.scalar_v465 = v465;
        let v470: f64 = p.p84;
        self.scalar_v470 = v470;
        let v473: bool = (!v455);
        self.scalar_v473 = v473;
        let v478: f64 = p.p85;
        self.scalar_v478 = v478;
        let v479: f64 = (-p.p85);
        self.scalar_v479 = v479;
        let v480: f64 = (p.p83 * v479);
        self.scalar_v480 = v480;
        let v481: f64 = { let limexp_arg = v480; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v481 = v481;
        let v482: f64 = (if v473 { v481 } else { 0.0 });
        self.scalar_v482 = v482;
        let v483: f64 = (p.p84 * v479);
        self.scalar_v483 = v483;
        let v484: f64 = { let limexp_arg = v483; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v484 = v484;
        let v485: f64 = (if v473 { v484 } else { 0.0 });
        self.scalar_v485 = v485;
        let v486: bool = (1.0 == p.p5);
        self.scalar_v486 = v486;
        let v487: bool = (v473 && v486);
        self.scalar_v487 = v487;
        let v492: bool = (!v486);
        self.scalar_v492 = v492;
        let v493: bool = (v473 && v492);
        self.scalar_v493 = v493;
        let v501: f64 = p.p42;
        self.scalar_v501 = v501;
        let v505: f64 = p.p82;
        self.scalar_v505 = v505;
        let v506: f64 = (0.001 * p.p82);
        self.scalar_v506 = v506;
        let v520: f64 = p.p31;
        self.scalar_v520 = v520;
        let v523: f64 = p.p38;
        self.scalar_v523 = v523;
        let v528: f64 = p.p32;
        self.scalar_v528 = v528;
        let v529: f64 = p.p33;
        self.scalar_v529 = v529;
        let v534: f64 = p.p34;
        self.scalar_v534 = v534;
        let v535: f64 = p.p35;
        self.scalar_v535 = v535;
        let v541: f64 = p.p37;
        self.scalar_v541 = v541;
        let v547: bool = (0.0 == p.p6);
        self.scalar_v547 = v547;
        let v548: bool = (1.0 == p.p6);
        self.scalar_v548 = v548;
        let v549: bool = (p.p6 == 2.0);
        self.scalar_v549 = v549;
        let v550: bool = (p.p6 == 3.0);
        self.scalar_v550 = v550;
        let v551: f64 = p.p25;
        self.scalar_v551 = v551;
        let v552: f64 = (if v547 { p.p25 } else { 0.0 });
        self.scalar_v552 = v552;
        let v553: f64 = p.p27;
        self.scalar_v553 = v553;
        let v554: f64 = (if v547 { p.p27 } else { 0.0 });
        self.scalar_v554 = v554;
        let v555: bool = (!v547);
        self.scalar_v555 = v555;
        let v556: bool = (v548 && v555);
        self.scalar_v556 = v556;
        let v562: f64 = (2.0 * p.p38);
        self.scalar_v562 = v562;
        let v567: bool = (v547 || v548);
        self.scalar_v567 = v567;
        let v568: bool = (!v567);
        self.scalar_v568 = v568;
        let v569: bool = (v549 && v568);
        self.scalar_v569 = v569;
        let v618: bool = (v549 || v567);
        self.scalar_v618 = v618;
        let v619: bool = (!v618);
        self.scalar_v619 = v619;
        let v620: bool = (v550 && v619);
        self.scalar_v620 = v620;
        let v621: f64 = p.p40;
        self.scalar_v621 = v621;
        let v625: f64 = (if v620 { 0.5 } else { 0.0 });
        self.scalar_v625 = v625;
        let v626: f64 = p.p41;
        self.scalar_v626 = v626;
        let v629: f64 = (-1.0 - v625);
        self.scalar_v629 = v629;
        let v631: f64 = (2.0 * v625);
        self.scalar_v631 = v631;
        let v632: f64 = (1.0 - v631);
        self.scalar_v632 = v632;
        let v644: f64 = (1.0 - p.p38);
        self.scalar_v644 = v644;
        let v654: f64 = p.p39;
        self.scalar_v654 = v654;
        let v666: bool = (v550 || v618);
        self.scalar_v666 = v666;
        let v667: bool = (!v666);
        self.scalar_v667 = v667;
        let v668: bool = (v116 && v667);
        self.scalar_v668 = v668;
        let v675: f64 = (if v668 { 0.5 } else { v625 });
        self.scalar_v675 = v675;
        let v681: f64 = (-v675);
        self.scalar_v681 = v681;
        let v685: f64 = (p.p40 * p.p39);
        self.scalar_v685 = v685;
        let v686: f64 = (1.0 + p.p41);
        self.scalar_v686 = v686;
        let v687: f64 = f64::powf(v686, v681);
        self.scalar_v687 = v687;
        let v688: f64 = (v685 * v687);
        self.scalar_v688 = v688;
        let v689: f64 = (if v668 { v688 } else { 0.0 });
        self.scalar_v689 = v689;
        let v723: bool = (v116 || v549);
        self.scalar_v723 = v723;
        let v724: f64 = p.p55;
        self.scalar_v724 = v724;
        let v725: bool = (p.p58 > 0.0);
        self.scalar_v725 = v725;
        let v726: bool = (p.p63 > 0.0);
        self.scalar_v726 = v726;
        let v727: bool = (p.p62 > 0.0);
        self.scalar_v727 = v727;
        let v728: bool = (v726 || v727);
        self.scalar_v728 = v728;
        let v729: f64 = p.p60;
        self.scalar_v729 = v729;
        let v730: bool = (p.p60 > 0.0);
        self.scalar_v730 = v730;
        let v731: f64 = p.p51;
        self.scalar_v731 = v731;
        let v732: bool = (p.p51 > 0.0);
        self.scalar_v732 = v732;
        let v733: f64 = p.p49;
        self.scalar_v733 = v733;
        let v734: bool = (p.p49 > 0.0);
        self.scalar_v734 = v734;
        let v735: f64 = p.p46;
        self.scalar_v735 = v735;
        let v736: bool = (p.p46 > 0.0);
        self.scalar_v736 = v736;
        let v737: bool = (p.p50 > 0.0);
        self.scalar_v737 = v737;
        let v738: bool = (p.p47 > 0.0);
        self.scalar_v738 = v738;
        let v739: bool = (p.p48 > 0.0);
        self.scalar_v739 = v739;
        let v740: bool = (v738 || v739);
        self.scalar_v740 = v740;
        let v741: f64 = p.p7;
        self.scalar_v741 = v741;
        let v742: bool = (0.0 == p.p7);
        self.scalar_v742 = v742;
        let v743: bool = (1.0 == p.p7);
        self.scalar_v743 = v743;
        let v744: bool = (!v742);
        self.scalar_v744 = v744;
        let v745: bool = (v743 && v744);
        self.scalar_v745 = v745;
        let v746: f64 = p.p0;
        self.scalar_v746 = v746;
        let v747: bool = (v745 && (p.p0 != 0.0));
        self.scalar_v747 = v747;
        let v750: f64 = p.p87;
        self.scalar_v750 = v750;
        let v751: f64 = p.p86;
        self.scalar_v751 = v751;
        let v752: f64 = p.p88;
        self.scalar_v752 = v752;
        let v755: f64 = (p.p87 * p.p86);
        self.scalar_v755 = v755;
        let v756: f64 = ((v755) as f64).sqrt();
        self.scalar_v756 = v756;
        let v769: f64 = p.p90;
        self.scalar_v769 = v769;
        let v770: bool = (p.p90 > 0.0);
        self.scalar_v770 = v770;
        let v771: bool = (p.p1 == 1.0);
        self.scalar_v771 = v771;
        let v773: f64 = p.p56;
        self.scalar_v773 = v773;
        let v776: f64 = (p.p56 / 3.0);
        self.scalar_v776 = v776;
        let v783: bool = (!v723);
        self.scalar_v783 = v783;
        let v790: f64 = p.p28;
        self.scalar_v790 = v790;
        let v794: f64 = p.p24;
        self.scalar_v794 = v794;
        let v806: bool = (!v725);
        self.scalar_v806 = v806;
        let v807: f64 = (if v806 { 0.0 } else { 0.0 });
        self.scalar_v807 = v807;
        let v814: bool = (!v728);
        self.scalar_v814 = v814;
        let v815: f64 = (if v814 { 0.0 } else { 0.0 });
        self.scalar_v815 = v815;
        let v816: f64 = p.p61;
        self.scalar_v816 = v816;
        let v823: bool = (!v730);
        self.scalar_v823 = v823;
        let v824: f64 = (if v823 { 0.0 } else { 0.0 });
        self.scalar_v824 = v824;
        let v829: bool = (!v732);
        self.scalar_v829 = v829;
        let v830: f64 = (if v829 { 0.0 } else { 0.0 });
        self.scalar_v830 = v830;
        let v831: f64 = (if (p.p0 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v831 = v831;
        let v835: bool = (!v734);
        self.scalar_v835 = v835;
        let v836: f64 = (if v835 { 0.0 } else { 0.0 });
        self.scalar_v836 = v836;
        let v840: bool = (v736 && (p.p0 != 0.0));
        self.scalar_v840 = v840;
        let v841: f64 = (if v840 { 0.0 } else { 0.0 });
        self.scalar_v841 = v841;
        let v842: bool = (!v736);
        self.scalar_v842 = v842;
        let v843: f64 = (if v842 { 0.0 } else { 0.0 });
        self.scalar_v843 = v843;
        let v844: f64 = p.p54;
        self.scalar_v844 = v844;
        let v850: bool = (v737 && (p.p0 != 0.0));
        self.scalar_v850 = v850;
        let v851: f64 = (if v850 { 0.0 } else { 0.0 });
        self.scalar_v851 = v851;
        let v852: bool = (!v737);
        self.scalar_v852 = v852;
        let v853: f64 = (if v852 { 0.0 } else { 0.0 });
        self.scalar_v853 = v853;
        let v854: f64 = p.p53;
        self.scalar_v854 = v854;
        let v860: bool = (v740 && (p.p0 != 0.0));
        self.scalar_v860 = v860;
        let v861: f64 = (if v860 { 0.0 } else { 0.0 });
        self.scalar_v861 = v861;
        let v862: bool = (!v740);
        self.scalar_v862 = v862;
        let v863: f64 = (if v862 { 0.0 } else { 0.0 });
        self.scalar_v863 = v863;
        let v864: f64 = p.p52;
        self.scalar_v864 = v864;
        let v871: bool = (v742 && (p.p0 != 0.0));
        self.scalar_v871 = v871;
        let v872: f64 = (if v871 { 0.0 } else { 0.0 });
        self.scalar_v872 = v872;
        let v873: f64 = (if v747 { 0.0 } else { 0.0 });
        self.scalar_v873 = v873;
        let v886: bool = (v747 && v770);
        self.scalar_v886 = v886;
        let v887: f64 = (if v886 { 0.0 } else { 0.0 });
        self.scalar_v887 = v887;
        let v888: bool = ((p.p0 != 0.0) && v770);
        self.scalar_v888 = v888;
        let v889: f64 = (if v888 { 0.0 } else { 0.0 });
        self.scalar_v889 = v889;
        let v899: f64 = p.p67;
        self.scalar_v899 = v899;
        let v903: bool = (!v771);
        self.scalar_v903 = v903;
        let v906: f64 = (-p.p19);
        self.scalar_v906 = v906;
        let v910: f64 = (-p.p64);
        self.scalar_v910 = v910;
        let v926: f64 = (-p.p15);
        self.scalar_v926 = v926;
        let v934: f64 = (-p.p22);
        self.scalar_v934 = v934;
        let v1081: f64 = (-p.p16);
        self.scalar_v1081 = v1081;
        let v1122: f64 = (if v237 { 0.0 } else { 1.0 });
        self.scalar_v1122 = v1122;
        let v1153: f64 = (p.p12 * v1122);
        self.scalar_v1153 = v1153;
        let v1223: f64 = (-p.p23);
        self.scalar_v1223 = v1223;
        let v1358: f64 = (if v290 { 1.0 } else { 0.0 });
        self.scalar_v1358 = v1358;
        let v1561: f64 = (if v330 { 1.0 } else { v1358 });
        self.scalar_v1561 = v1561;
        let v2050: f64 = (-p.p65);
        self.scalar_v2050 = v2050;
        let v2051: f64 = (-1.0 + v2050);
        self.scalar_v2051 = v2051;
        let v2155: f64 = (if v455 { 0.0 } else { v1561 });
        self.scalar_v2155 = v2155;
        let v2156: f64 = (if v455 { -1.0 } else { 0.0 });
        self.scalar_v2156 = v2156;
        let v2157: f64 = (if v455 { 1.0 } else { 0.0 });
        self.scalar_v2157 = v2157;
        let v2162: f64 = (if v473 { 0.0 } else { v2155 });
        self.scalar_v2162 = v2162;
        let v2177: f64 = (if v473 { 1.0 } else { v2157 });
        self.scalar_v2177 = v2177;
        let v2178: f64 = (if v473 { -1.0 } else { v2156 });
        self.scalar_v2178 = v2178;
        let v2179: f64 = (p.p85 * v2177);
        self.scalar_v2179 = v2179;
        let v2180: f64 = (p.p85 * v2178);
        self.scalar_v2180 = v2180;
        let v2197: f64 = (-v2162);
        self.scalar_v2197 = v2197;
        let v2203: f64 = (p.p42 * v2197);
        self.scalar_v2203 = v2203;
        let v2222: f64 = (-p.p31);
        self.scalar_v2222 = v2222;
        let v2223: f64 = (-p.p38);
        self.scalar_v2223 = v2223;
        let v2224: f64 = (v2222 + v2223);
        self.scalar_v2224 = v2224;
        let v2230: f64 = (-p.p33);
        self.scalar_v2230 = v2230;
        let v2235: f64 = (-p.p35);
        self.scalar_v2235 = v2235;
        let v2240: f64 = (-p.p37);
        self.scalar_v2240 = v2240;
        let v2241: f64 = (v2240 - p.p38);
        self.scalar_v2241 = v2241;
        let v2316: f64 = (-v562);
        self.scalar_v2316 = v2316;
        let v2322: f64 = (-p.p25);
        self.scalar_v2322 = v2322;
        let v2380: f64 = (-p.p27);
        self.scalar_v2380 = v2380;
        let v2393: f64 = (-1.0 / p.p40);
        self.scalar_v2393 = v2393;
        let v2394: f64 = (1.0 / p.p40);
        self.scalar_v2394 = v2394;
        let v2395: f64 = (if v620 { v2393 } else { 0.0 });
        self.scalar_v2395 = v2395;
        let v2396: f64 = (if v620 { v2394 } else { 0.0 });
        self.scalar_v2396 = v2396;
        let v2401: f64 = (v629 - 1.0);
        self.scalar_v2401 = v2401;
        let v2416: f64 = (-1.0 + v2223);
        self.scalar_v2416 = v2416;
        let v2417: f64 = (p.p31 * p.p38);
        self.scalar_v2417 = v2417;
        let v2418: f64 = (p.p31 * v2416);
        self.scalar_v2418 = v2418;
        let v2431: f64 = (-v644);
        self.scalar_v2431 = v2431;
        let v2432: f64 = (-1.0 + v644);
        self.scalar_v2432 = v2432;
        let v2433: f64 = (p.p37 * v2432);
        self.scalar_v2433 = v2433;
        let v2434: f64 = (p.p37 * v2431);
        self.scalar_v2434 = v2434;
        let v2491: f64 = (-p.p39);
        self.scalar_v2491 = v2491;
        let v2496: f64 = (v681 - 1.0);
        self.scalar_v2496 = v2496;
        let v2647: f64 = (-p.p28);
        self.scalar_v2647 = v2647;
        let v2648: f64 = (-p.p24);
        self.scalar_v2648 = v2648;
        let v2673: f64 = (-p.p61);
        self.scalar_v2673 = v2673;
        let v2674: f64 = (-1.0 / p.p60);
        self.scalar_v2674 = v2674;
        let v2675: f64 = (1.0 / p.p60);
        self.scalar_v2675 = v2675;
        let v2676: f64 = (if v730 { v2674 } else { 0.0 });
        self.scalar_v2676 = v2676;
        let v2677: f64 = (if v730 { v2675 } else { 0.0 });
        self.scalar_v2677 = v2677;
        let v2678: f64 = (-1.0 / p.p51);
        self.scalar_v2678 = v2678;
        let v2679: f64 = (1.0 / p.p51);
        self.scalar_v2679 = v2679;
        let v2680: f64 = (if v732 { v2678 } else { 0.0 });
        self.scalar_v2680 = v2680;
        let v2681: f64 = (if v732 { v2679 } else { 0.0 });
        self.scalar_v2681 = v2681;
        let v2682: f64 = (-1.0 / p.p49);
        self.scalar_v2682 = v2682;
        let v2683: f64 = (1.0 / p.p49);
        self.scalar_v2683 = v2683;
        let v2684: f64 = (if v734 { v2682 } else { 0.0 });
        self.scalar_v2684 = v2684;
        let v2685: f64 = (if v734 { v2683 } else { 0.0 });
        self.scalar_v2685 = v2685;
        let v2686: f64 = (if v736 { p.p46 } else { 0.0 });
        self.scalar_v2686 = v2686;
        let v2709: f64 = (if v747 { 1.0 } else { 0.0 });
        self.scalar_v2709 = v2709;
        let v2718: f64 = (if v903 { 1e-12 } else { 0.0 });
        self.scalar_v2718 = v2718;
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
        let v22: f64 = (temperature + self.scalar_v21);
        self.scalar_v22 = v22;
        let v23: f64 = (if self.scalar_v19 { self.scalar_v22 } else { self.scalar_v18 });
        self.scalar_v23 = v23;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
