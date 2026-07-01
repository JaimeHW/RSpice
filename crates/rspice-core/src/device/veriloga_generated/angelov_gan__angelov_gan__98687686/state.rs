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
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: bool,
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: bool,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v97: bool,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: bool,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: bool,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v128: bool,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v192: bool,
    pub(crate) scalar_v194: bool,
    pub(crate) scalar_v196: bool,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v207: bool,
    pub(crate) scalar_v208: bool,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v259: bool,
    pub(crate) scalar_v260: bool,
    pub(crate) scalar_v261: bool,
    pub(crate) scalar_v299: bool,
    pub(crate) scalar_v300: bool,
    pub(crate) scalar_v301: bool,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: bool,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: bool,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v418: bool,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: bool,
    pub(crate) scalar_v432: bool,
    pub(crate) scalar_v437: bool,
    pub(crate) scalar_v438: bool,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v465: bool,
    pub(crate) scalar_v466: bool,
    pub(crate) scalar_v467: bool,
    pub(crate) scalar_v468: bool,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: bool,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: bool,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: bool,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v476: bool,
    pub(crate) scalar_v477: bool,
    pub(crate) scalar_v478: bool,
    pub(crate) scalar_v479: bool,
    pub(crate) scalar_v480: bool,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: bool,
    pub(crate) scalar_v483: bool,
    pub(crate) scalar_v484: bool,
    pub(crate) scalar_v485: bool,
    pub(crate) scalar_v486: f64,
    pub(crate) scalar_v487: bool,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v507: bool,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v512: f64,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v522: bool,
    pub(crate) scalar_v523: f64,
    pub(crate) scalar_v527: bool,
    pub(crate) scalar_v528: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v536: bool,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v542: bool,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v547: bool,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v549: bool,
    pub(crate) scalar_v550: f64,
    pub(crate) scalar_v551: bool,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: bool,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v576: bool,
    pub(crate) scalar_v579: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v599: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v754: f64,
    pub(crate) scalar_v795: f64,
    pub(crate) scalar_v826: f64,
    pub(crate) scalar_v896: f64,
    pub(crate) scalar_v1031: f64,
    pub(crate) scalar_v1234: f64,
    pub(crate) scalar_v1723: f64,
    pub(crate) scalar_v1724: f64,
    pub(crate) scalar_v1761: f64,
    pub(crate) scalar_v1762: f64,
    pub(crate) scalar_v1763: f64,
    pub(crate) scalar_v1768: f64,
    pub(crate) scalar_v1783: f64,
    pub(crate) scalar_v1784: f64,
    pub(crate) scalar_v1785: f64,
    pub(crate) scalar_v1786: f64,
    pub(crate) scalar_v1803: f64,
    pub(crate) scalar_v1809: f64,
    pub(crate) scalar_v1833: f64,
    pub(crate) scalar_v1834: f64,
    pub(crate) scalar_v1841: f64,
    pub(crate) scalar_v1842: f64,
    pub(crate) scalar_v1843: f64,
    pub(crate) scalar_v1844: f64,
    pub(crate) scalar_v1845: f64,
    pub(crate) scalar_v1846: f64,
    pub(crate) scalar_v1847: f64,
    pub(crate) scalar_v1848: f64,
    pub(crate) scalar_v1849: f64,
    pub(crate) scalar_v1850: f64,
    pub(crate) scalar_v1851: f64,
    pub(crate) scalar_v1852: f64,
    pub(crate) scalar_v1853: f64,
    pub(crate) scalar_v1854: f64,
    pub(crate) scalar_v1859: f64,
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
            scalar_v70: self.scalar_v70,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v99: self.scalar_v99,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v131: self.scalar_v131,
            scalar_v134: self.scalar_v134,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v144: self.scalar_v144,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v153: self.scalar_v153,
            scalar_v155: self.scalar_v155,
            scalar_v161: self.scalar_v161,
            scalar_v172: self.scalar_v172,
            scalar_v187: self.scalar_v187,
            scalar_v192: self.scalar_v192,
            scalar_v194: self.scalar_v194,
            scalar_v196: self.scalar_v196,
            scalar_v199: self.scalar_v199,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v227: self.scalar_v227,
            scalar_v235: self.scalar_v235,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v385: self.scalar_v385,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v410: self.scalar_v410,
            scalar_v415: self.scalar_v415,
            scalar_v418: self.scalar_v418,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v446: self.scalar_v446,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v465: self.scalar_v465,
            scalar_v466: self.scalar_v466,
            scalar_v467: self.scalar_v467,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
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
            scalar_v490: self.scalar_v490,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v507: self.scalar_v507,
            scalar_v509: self.scalar_v509,
            scalar_v512: self.scalar_v512,
            scalar_v516: self.scalar_v516,
            scalar_v522: self.scalar_v522,
            scalar_v523: self.scalar_v523,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
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
            scalar_v576: self.scalar_v576,
            scalar_v579: self.scalar_v579,
            scalar_v583: self.scalar_v583,
            scalar_v599: self.scalar_v599,
            scalar_v607: self.scalar_v607,
            scalar_v754: self.scalar_v754,
            scalar_v795: self.scalar_v795,
            scalar_v826: self.scalar_v826,
            scalar_v896: self.scalar_v896,
            scalar_v1031: self.scalar_v1031,
            scalar_v1234: self.scalar_v1234,
            scalar_v1723: self.scalar_v1723,
            scalar_v1724: self.scalar_v1724,
            scalar_v1761: self.scalar_v1761,
            scalar_v1762: self.scalar_v1762,
            scalar_v1763: self.scalar_v1763,
            scalar_v1768: self.scalar_v1768,
            scalar_v1783: self.scalar_v1783,
            scalar_v1784: self.scalar_v1784,
            scalar_v1785: self.scalar_v1785,
            scalar_v1786: self.scalar_v1786,
            scalar_v1803: self.scalar_v1803,
            scalar_v1809: self.scalar_v1809,
            scalar_v1833: self.scalar_v1833,
            scalar_v1834: self.scalar_v1834,
            scalar_v1841: self.scalar_v1841,
            scalar_v1842: self.scalar_v1842,
            scalar_v1843: self.scalar_v1843,
            scalar_v1844: self.scalar_v1844,
            scalar_v1845: self.scalar_v1845,
            scalar_v1846: self.scalar_v1846,
            scalar_v1847: self.scalar_v1847,
            scalar_v1848: self.scalar_v1848,
            scalar_v1849: self.scalar_v1849,
            scalar_v1850: self.scalar_v1850,
            scalar_v1851: self.scalar_v1851,
            scalar_v1852: self.scalar_v1852,
            scalar_v1853: self.scalar_v1853,
            scalar_v1854: self.scalar_v1854,
            scalar_v1859: self.scalar_v1859,
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
            scalar_v70: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v90: 0.0,
            scalar_v91: false,
            scalar_v93: false,
            scalar_v94: false,
            scalar_v95: 0.0,
            scalar_v96: false,
            scalar_v97: false,
            scalar_v99: 0.0,
            scalar_v105: 0.0,
            scalar_v106: false,
            scalar_v119: 0.0,
            scalar_v120: false,
            scalar_v121: 0.0,
            scalar_v122: false,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v128: false,
            scalar_v129: 0.0,
            scalar_v131: 0.0,
            scalar_v134: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v144: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v153: 0.0,
            scalar_v155: 0.0,
            scalar_v161: 0.0,
            scalar_v172: 0.0,
            scalar_v187: 0.0,
            scalar_v192: false,
            scalar_v194: false,
            scalar_v196: false,
            scalar_v199: 0.0,
            scalar_v207: false,
            scalar_v208: false,
            scalar_v227: 0.0,
            scalar_v235: 0.0,
            scalar_v259: false,
            scalar_v260: false,
            scalar_v261: false,
            scalar_v299: false,
            scalar_v300: false,
            scalar_v301: false,
            scalar_v374: false,
            scalar_v375: false,
            scalar_v376: false,
            scalar_v385: 0.0,
            scalar_v396: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v400: false,
            scalar_v410: 0.0,
            scalar_v415: 0.0,
            scalar_v418: false,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v428: 0.0,
            scalar_v429: 0.0,
            scalar_v430: 0.0,
            scalar_v431: false,
            scalar_v432: false,
            scalar_v437: false,
            scalar_v438: false,
            scalar_v446: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v465: false,
            scalar_v466: false,
            scalar_v467: false,
            scalar_v468: false,
            scalar_v469: 0.0,
            scalar_v470: false,
            scalar_v471: 0.0,
            scalar_v472: false,
            scalar_v473: 0.0,
            scalar_v474: false,
            scalar_v475: 0.0,
            scalar_v476: false,
            scalar_v477: false,
            scalar_v478: false,
            scalar_v479: false,
            scalar_v480: false,
            scalar_v481: 0.0,
            scalar_v482: false,
            scalar_v483: false,
            scalar_v484: false,
            scalar_v485: false,
            scalar_v486: 0.0,
            scalar_v487: false,
            scalar_v490: 0.0,
            scalar_v491: 0.0,
            scalar_v492: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v507: false,
            scalar_v509: 0.0,
            scalar_v512: 0.0,
            scalar_v516: 0.0,
            scalar_v522: false,
            scalar_v523: 0.0,
            scalar_v527: false,
            scalar_v528: 0.0,
            scalar_v529: 0.0,
            scalar_v536: false,
            scalar_v537: 0.0,
            scalar_v542: false,
            scalar_v543: 0.0,
            scalar_v547: false,
            scalar_v548: 0.0,
            scalar_v549: false,
            scalar_v550: 0.0,
            scalar_v551: false,
            scalar_v552: 0.0,
            scalar_v553: false,
            scalar_v554: 0.0,
            scalar_v576: false,
            scalar_v579: 0.0,
            scalar_v583: 0.0,
            scalar_v599: 0.0,
            scalar_v607: 0.0,
            scalar_v754: 0.0,
            scalar_v795: 0.0,
            scalar_v826: 0.0,
            scalar_v896: 0.0,
            scalar_v1031: 0.0,
            scalar_v1234: 0.0,
            scalar_v1723: 0.0,
            scalar_v1724: 0.0,
            scalar_v1761: 0.0,
            scalar_v1762: 0.0,
            scalar_v1763: 0.0,
            scalar_v1768: 0.0,
            scalar_v1783: 0.0,
            scalar_v1784: 0.0,
            scalar_v1785: 0.0,
            scalar_v1786: 0.0,
            scalar_v1803: 0.0,
            scalar_v1809: 0.0,
            scalar_v1833: 0.0,
            scalar_v1834: 0.0,
            scalar_v1841: 0.0,
            scalar_v1842: 0.0,
            scalar_v1843: 0.0,
            scalar_v1844: 0.0,
            scalar_v1845: 0.0,
            scalar_v1846: 0.0,
            scalar_v1847: 0.0,
            scalar_v1848: 0.0,
            scalar_v1849: 0.0,
            scalar_v1850: 0.0,
            scalar_v1851: 0.0,
            scalar_v1852: 0.0,
            scalar_v1853: 0.0,
            scalar_v1854: 0.0,
            scalar_v1859: 0.0,
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
            scalar_v70,
            scalar_v75,
            scalar_v76,
            scalar_v80,
            scalar_v81,
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v91,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v99,
            scalar_v105,
            scalar_v106,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v128,
            scalar_v129,
            scalar_v131,
            scalar_v134,
            scalar_v136,
            scalar_v137,
            scalar_v144,
            scalar_v148,
            scalar_v149,
            scalar_v153,
            scalar_v155,
            scalar_v161,
            scalar_v172,
            scalar_v187,
            scalar_v192,
            scalar_v194,
            scalar_v196,
            scalar_v199,
            scalar_v207,
            scalar_v208,
            scalar_v227,
            scalar_v235,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v385,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v410,
            scalar_v415,
            scalar_v418,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v446,
            scalar_v450,
            scalar_v451,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
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
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v495,
            scalar_v496,
            scalar_v507,
            scalar_v509,
            scalar_v512,
            scalar_v516,
            scalar_v522,
            scalar_v523,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v536,
            scalar_v537,
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
            scalar_v576,
            scalar_v579,
            scalar_v583,
            scalar_v599,
            scalar_v607,
            scalar_v754,
            scalar_v795,
            scalar_v826,
            scalar_v896,
            scalar_v1031,
            scalar_v1234,
            scalar_v1723,
            scalar_v1724,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1768,
            scalar_v1783,
            scalar_v1784,
            scalar_v1785,
            scalar_v1786,
            scalar_v1803,
            scalar_v1809,
            scalar_v1833,
            scalar_v1834,
            scalar_v1841,
            scalar_v1842,
            scalar_v1843,
            scalar_v1844,
            scalar_v1845,
            scalar_v1846,
            scalar_v1847,
            scalar_v1848,
            scalar_v1849,
            scalar_v1850,
            scalar_v1851,
            scalar_v1852,
            scalar_v1853,
            scalar_v1854,
            scalar_v1859,
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
            scalar_v70,
            scalar_v75,
            scalar_v76,
            scalar_v80,
            scalar_v81,
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v91,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v99,
            scalar_v105,
            scalar_v106,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v124,
            scalar_v125,
            scalar_v128,
            scalar_v129,
            scalar_v131,
            scalar_v134,
            scalar_v136,
            scalar_v137,
            scalar_v144,
            scalar_v148,
            scalar_v149,
            scalar_v153,
            scalar_v155,
            scalar_v161,
            scalar_v172,
            scalar_v187,
            scalar_v192,
            scalar_v194,
            scalar_v196,
            scalar_v199,
            scalar_v207,
            scalar_v208,
            scalar_v227,
            scalar_v235,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v385,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v410,
            scalar_v415,
            scalar_v418,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v446,
            scalar_v450,
            scalar_v451,
            scalar_v465,
            scalar_v466,
            scalar_v467,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v476,
            scalar_v477,
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
            scalar_v490,
            scalar_v491,
            scalar_v492,
            scalar_v495,
            scalar_v496,
            scalar_v507,
            scalar_v509,
            scalar_v512,
            scalar_v516,
            scalar_v522,
            scalar_v523,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v536,
            scalar_v537,
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
            scalar_v576,
            scalar_v579,
            scalar_v583,
            scalar_v599,
            scalar_v607,
            scalar_v754,
            scalar_v795,
            scalar_v826,
            scalar_v896,
            scalar_v1031,
            scalar_v1234,
            scalar_v1723,
            scalar_v1724,
            scalar_v1761,
            scalar_v1762,
            scalar_v1763,
            scalar_v1768,
            scalar_v1783,
            scalar_v1784,
            scalar_v1785,
            scalar_v1786,
            scalar_v1803,
            scalar_v1809,
            scalar_v1833,
            scalar_v1834,
            scalar_v1841,
            scalar_v1842,
            scalar_v1843,
            scalar_v1844,
            scalar_v1845,
            scalar_v1846,
            scalar_v1847,
            scalar_v1848,
            scalar_v1849,
            scalar_v1850,
            scalar_v1851,
            scalar_v1852,
            scalar_v1853,
            scalar_v1854,
            scalar_v1859,
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
        let v68: f64 = p.p58;
        self.scalar_v68 = v68;
        let v69: f64 = p.p59;
        self.scalar_v69 = v69;
        let v70: f64 = p.p75;
        self.scalar_v70 = v70;
        let v75: f64 = p.p9;
        self.scalar_v75 = v75;
        let v76: f64 = p.p78;
        self.scalar_v76 = v76;
        let v80: f64 = p.p45;
        self.scalar_v80 = v80;
        let v81: f64 = p.p79;
        self.scalar_v81 = v81;
        let v85: f64 = p.p21;
        self.scalar_v85 = v85;
        let v86: f64 = p.p81;
        self.scalar_v86 = v86;
        let v90: f64 = p.p4;
        self.scalar_v90 = v90;
        let v91: bool = (1.0 == p.p4);
        self.scalar_v91 = v91;
        let v93: bool = (p.p4 == 4.0);
        self.scalar_v93 = v93;
        let v94: bool = (v91 || v93);
        self.scalar_v94 = v94;
        let v95: f64 = p.p6;
        self.scalar_v95 = v95;
        let v96: bool = (4.0 == p.p6);
        self.scalar_v96 = v96;
        let v97: bool = (v94 && v96);
        self.scalar_v97 = v97;
        let v99: f64 = p.p62;
        self.scalar_v99 = v99;
        let v105: f64 = p.p63;
        self.scalar_v105 = v105;
        let v106: bool = (!v97);
        self.scalar_v106 = v106;
        let v119: f64 = if param_given[43] { 1.0 } else { 0.0 };
        self.scalar_v119 = v119;
        let v120: bool = (!(if param_given[43] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v120 = v120;
        let v121: f64 = if param_given[44] { 1.0 } else { 0.0 };
        self.scalar_v121 = v121;
        let v122: bool = (v120 && (if param_given[44] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v122 = v122;
        let v124: f64 = p.p44;
        self.scalar_v124 = v124;
        let v125: f64 = (0.5 / p.p44);
        self.scalar_v125 = v125;
        let v128: bool = (!v122);
        self.scalar_v128 = v128;
        let v129: f64 = p.p43;
        self.scalar_v129 = v129;
        let v131: f64 = p.p19;
        self.scalar_v131 = v131;
        let v134: f64 = p.p64;
        self.scalar_v134 = v134;
        let v136: f64 = p.p11;
        self.scalar_v136 = v136;
        let v137: f64 = p.p18;
        self.scalar_v137 = v137;
        let v144: f64 = p.p69;
        self.scalar_v144 = v144;
        let v148: f64 = p.p13;
        self.scalar_v148 = v148;
        let v149: f64 = p.p70;
        self.scalar_v149 = v149;
        let v153: f64 = p.p10;
        self.scalar_v153 = v153;
        let v155: f64 = p.p15;
        self.scalar_v155 = v155;
        let v161: f64 = p.p22;
        self.scalar_v161 = v161;
        let v172: f64 = p.p12;
        self.scalar_v172 = v172;
        let v187: f64 = p.p14;
        self.scalar_v187 = v187;
        let v192: bool = (0.0 == p.p4);
        self.scalar_v192 = v192;
        let v194: bool = (p.p4 == 2.0);
        self.scalar_v194 = v194;
        let v196: bool = (p.p4 == 3.0);
        self.scalar_v196 = v196;
        let v199: f64 = p.p16;
        self.scalar_v199 = v199;
        let v207: bool = (!v192);
        self.scalar_v207 = v207;
        let v208: bool = (v91 && v207);
        self.scalar_v208 = v208;
        let v227: f64 = p.p17;
        self.scalar_v227 = v227;
        let v235: f64 = p.p23;
        self.scalar_v235 = v235;
        let v259: bool = (v91 || v192);
        self.scalar_v259 = v259;
        let v260: bool = (!v259);
        self.scalar_v260 = v260;
        let v261: bool = (v194 && v260);
        self.scalar_v261 = v261;
        let v299: bool = (v194 || v259);
        self.scalar_v299 = v299;
        let v300: bool = (!v299);
        self.scalar_v300 = v300;
        let v301: bool = (v196 && v300);
        self.scalar_v301 = v301;
        let v374: bool = (v196 || v299);
        self.scalar_v374 = v374;
        let v375: bool = (!v374);
        self.scalar_v375 = v375;
        let v376: bool = (v93 && v375);
        self.scalar_v376 = v376;
        let v385: f64 = p.p65;
        self.scalar_v385 = v385;
        let v396: f64 = p.p47;
        self.scalar_v396 = v396;
        let v397: f64 = p.p48;
        self.scalar_v397 = v397;
        let v398: f64 = p.p50;
        self.scalar_v398 = v398;
        let v399: f64 = p.p5;
        self.scalar_v399 = v399;
        let v400: bool = (0.0 == p.p5);
        self.scalar_v400 = v400;
        let v410: f64 = p.p83;
        self.scalar_v410 = v410;
        let v415: f64 = p.p84;
        self.scalar_v415 = v415;
        let v418: bool = (!v400);
        self.scalar_v418 = v418;
        let v423: f64 = p.p85;
        self.scalar_v423 = v423;
        let v424: f64 = (-p.p85);
        self.scalar_v424 = v424;
        let v425: f64 = (p.p83 * v424);
        self.scalar_v425 = v425;
        let v426: f64 = { let limexp_arg = v425; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v426 = v426;
        let v427: f64 = (if v418 { v426 } else { 0.0 });
        self.scalar_v427 = v427;
        let v428: f64 = (p.p84 * v424);
        self.scalar_v428 = v428;
        let v429: f64 = { let limexp_arg = v428; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v429 = v429;
        let v430: f64 = (if v418 { v429 } else { 0.0 });
        self.scalar_v430 = v430;
        let v431: bool = (1.0 == p.p5);
        self.scalar_v431 = v431;
        let v432: bool = (v418 && v431);
        self.scalar_v432 = v432;
        let v437: bool = (!v431);
        self.scalar_v437 = v437;
        let v438: bool = (v418 && v437);
        self.scalar_v438 = v438;
        let v446: f64 = p.p42;
        self.scalar_v446 = v446;
        let v450: f64 = p.p82;
        self.scalar_v450 = v450;
        let v451: f64 = (0.001 * p.p82);
        self.scalar_v451 = v451;
        let v465: bool = (p.p58 > 0.0);
        self.scalar_v465 = v465;
        let v466: bool = (p.p63 > 0.0);
        self.scalar_v466 = v466;
        let v467: bool = (p.p62 > 0.0);
        self.scalar_v467 = v467;
        let v468: bool = (v466 || v467);
        self.scalar_v468 = v468;
        let v469: f64 = p.p60;
        self.scalar_v469 = v469;
        let v470: bool = (p.p60 > 0.0);
        self.scalar_v470 = v470;
        let v471: f64 = p.p51;
        self.scalar_v471 = v471;
        let v472: bool = (p.p51 > 0.0);
        self.scalar_v472 = v472;
        let v473: f64 = p.p49;
        self.scalar_v473 = v473;
        let v474: bool = (p.p49 > 0.0);
        self.scalar_v474 = v474;
        let v475: f64 = p.p46;
        self.scalar_v475 = v475;
        let v476: bool = (p.p46 > 0.0);
        self.scalar_v476 = v476;
        let v477: bool = (p.p50 > 0.0);
        self.scalar_v477 = v477;
        let v478: bool = (p.p47 > 0.0);
        self.scalar_v478 = v478;
        let v479: bool = (p.p48 > 0.0);
        self.scalar_v479 = v479;
        let v480: bool = (v478 || v479);
        self.scalar_v480 = v480;
        let v481: f64 = p.p7;
        self.scalar_v481 = v481;
        let v482: bool = (0.0 == p.p7);
        self.scalar_v482 = v482;
        let v483: bool = (1.0 == p.p7);
        self.scalar_v483 = v483;
        let v484: bool = (!v482);
        self.scalar_v484 = v484;
        let v485: bool = (v483 && v484);
        self.scalar_v485 = v485;
        let v486: f64 = p.p0;
        self.scalar_v486 = v486;
        let v487: bool = (v485 && (p.p0 != 0.0));
        self.scalar_v487 = v487;
        let v490: f64 = p.p87;
        self.scalar_v490 = v490;
        let v491: f64 = p.p86;
        self.scalar_v491 = v491;
        let v492: f64 = p.p88;
        self.scalar_v492 = v492;
        let v495: f64 = (p.p87 * p.p86);
        self.scalar_v495 = v495;
        let v496: f64 = ((v495) as f64).sqrt();
        self.scalar_v496 = v496;
        let v507: bool = (p.p1 == 1.0);
        self.scalar_v507 = v507;
        let v509: f64 = p.p56;
        self.scalar_v509 = v509;
        let v512: f64 = p.p28;
        self.scalar_v512 = v512;
        let v516: f64 = p.p24;
        self.scalar_v516 = v516;
        let v522: bool = (!v465);
        self.scalar_v522 = v522;
        let v523: f64 = (if v522 { 0.0 } else { 0.0 });
        self.scalar_v523 = v523;
        let v527: bool = (!v468);
        self.scalar_v527 = v527;
        let v528: f64 = (if v527 { 0.0 } else { 0.0 });
        self.scalar_v528 = v528;
        let v529: f64 = p.p61;
        self.scalar_v529 = v529;
        let v536: bool = (!v470);
        self.scalar_v536 = v536;
        let v537: f64 = (if v536 { 0.0 } else { 0.0 });
        self.scalar_v537 = v537;
        let v542: bool = (!v472);
        self.scalar_v542 = v542;
        let v543: f64 = (if v542 { 0.0 } else { 0.0 });
        self.scalar_v543 = v543;
        let v547: bool = (!v474);
        self.scalar_v547 = v547;
        let v548: f64 = (if v547 { 0.0 } else { 0.0 });
        self.scalar_v548 = v548;
        let v549: bool = (!v476);
        self.scalar_v549 = v549;
        let v550: f64 = (if v549 { 0.0 } else { 0.0 });
        self.scalar_v550 = v550;
        let v551: bool = (!v477);
        self.scalar_v551 = v551;
        let v552: f64 = (if v551 { 0.0 } else { 0.0 });
        self.scalar_v552 = v552;
        let v553: bool = (!v480);
        self.scalar_v553 = v553;
        let v554: f64 = (if v553 { 0.0 } else { 0.0 });
        self.scalar_v554 = v554;
        let v576: bool = (!v507);
        self.scalar_v576 = v576;
        let v579: f64 = (-p.p19);
        self.scalar_v579 = v579;
        let v583: f64 = (-p.p64);
        self.scalar_v583 = v583;
        let v599: f64 = (-p.p15);
        self.scalar_v599 = v599;
        let v607: f64 = (-p.p22);
        self.scalar_v607 = v607;
        let v754: f64 = (-p.p16);
        self.scalar_v754 = v754;
        let v795: f64 = (if v208 { 0.0 } else { 1.0 });
        self.scalar_v795 = v795;
        let v826: f64 = (p.p12 * v795);
        self.scalar_v826 = v826;
        let v896: f64 = (-p.p23);
        self.scalar_v896 = v896;
        let v1031: f64 = (if v261 { 1.0 } else { 0.0 });
        self.scalar_v1031 = v1031;
        let v1234: f64 = (if v301 { 1.0 } else { v1031 });
        self.scalar_v1234 = v1234;
        let v1723: f64 = (-p.p65);
        self.scalar_v1723 = v1723;
        let v1724: f64 = (-1.0 + v1723);
        self.scalar_v1724 = v1724;
        let v1761: f64 = (if v400 { 0.0 } else { v1234 });
        self.scalar_v1761 = v1761;
        let v1762: f64 = (if v400 { -1.0 } else { 0.0 });
        self.scalar_v1762 = v1762;
        let v1763: f64 = (if v400 { 1.0 } else { 0.0 });
        self.scalar_v1763 = v1763;
        let v1768: f64 = (if v418 { 0.0 } else { v1761 });
        self.scalar_v1768 = v1768;
        let v1783: f64 = (if v418 { 1.0 } else { v1763 });
        self.scalar_v1783 = v1783;
        let v1784: f64 = (if v418 { -1.0 } else { v1762 });
        self.scalar_v1784 = v1784;
        let v1785: f64 = (p.p85 * v1783);
        self.scalar_v1785 = v1785;
        let v1786: f64 = (p.p85 * v1784);
        self.scalar_v1786 = v1786;
        let v1803: f64 = (-v1768);
        self.scalar_v1803 = v1803;
        let v1809: f64 = (p.p42 * v1803);
        self.scalar_v1809 = v1809;
        let v1833: f64 = (-p.p28);
        self.scalar_v1833 = v1833;
        let v1834: f64 = (-p.p24);
        self.scalar_v1834 = v1834;
        let v1841: f64 = (-p.p61);
        self.scalar_v1841 = v1841;
        let v1842: f64 = (-1.0 / p.p60);
        self.scalar_v1842 = v1842;
        let v1843: f64 = (1.0 / p.p60);
        self.scalar_v1843 = v1843;
        let v1844: f64 = (if v470 { v1842 } else { 0.0 });
        self.scalar_v1844 = v1844;
        let v1845: f64 = (if v470 { v1843 } else { 0.0 });
        self.scalar_v1845 = v1845;
        let v1846: f64 = (-1.0 / p.p51);
        self.scalar_v1846 = v1846;
        let v1847: f64 = (1.0 / p.p51);
        self.scalar_v1847 = v1847;
        let v1848: f64 = (if v472 { v1846 } else { 0.0 });
        self.scalar_v1848 = v1848;
        let v1849: f64 = (if v472 { v1847 } else { 0.0 });
        self.scalar_v1849 = v1849;
        let v1850: f64 = (-1.0 / p.p49);
        self.scalar_v1850 = v1850;
        let v1851: f64 = (1.0 / p.p49);
        self.scalar_v1851 = v1851;
        let v1852: f64 = (if v474 { v1850 } else { 0.0 });
        self.scalar_v1852 = v1852;
        let v1853: f64 = (if v474 { v1851 } else { 0.0 });
        self.scalar_v1853 = v1853;
        let v1854: f64 = (if v487 { 1.0 } else { 0.0 });
        self.scalar_v1854 = v1854;
        let v1859: f64 = (if v576 { 1e-12 } else { 0.0 });
        self.scalar_v1859 = v1859;
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
