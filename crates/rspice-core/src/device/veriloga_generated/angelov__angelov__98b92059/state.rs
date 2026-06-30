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
            params.p21 = 20.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 1.0;
            params.p31 = 0.0;
            params.p32 = 0.2;
            params.p33 = 0.0;
            params.p34 = 0.2;
            params.p35 = 0.0;
            params.p36 = 1.0;
            params.p37 = 0.0;
            params.p38 = 5e-5;
            params.p39 = 15.0;
            params.p40 = 1.0;
            params.p41 = 0.7;
            params.p42 = 0.05;
            params.p43 = 0.05;
            params.p44 = 0.0;
            params.p45 = 0.05;
            params.p46 = 0.05;
            params.p47 = 0.05;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 1000.0;
            params.p53 = 10000.0;
            params.p54 = 0.0;
            params.p55 = 100000.0;
            params.p56 = 0.0;
            params.p57 = 0.001;
            params.p58 = 0.0001;
            params.p59 = -0.002;
            params.p60 = -0.002;
            params.p61 = 0.002;
            params.p62 = 0.002;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.003;
            params.p67 = 0.001;
            params.p68 = 0.001;
            params.p69 = -0.001;
            params.p70 = 0.0;
            params.p71 = 0.5;
            params.p72 = 1.0;
            params.p73 = 0.9;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 1.0;
            params.p77 = 1.0;
            params.p78 = 25.0;
            params.p79 = 0.1;
            params.p80 = 1.0;
            params.p81 = 1e-14;
            params.p82 = 60000.0;
            params.p83 = 0.3;
            params.p84 = 0.1;
            params.p85 = 25.0;
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
    pub nodes: [usize; 16],
    pub branches: [usize; 19],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 86]>,
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
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: bool,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: bool,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: bool,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: bool,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v113: bool,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: bool,
    pub(crate) scalar_v163: bool,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v178: bool,
    pub(crate) scalar_v179: bool,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v228: bool,
    pub(crate) scalar_v229: bool,
    pub(crate) scalar_v230: bool,
    pub(crate) scalar_v265: bool,
    pub(crate) scalar_v266: bool,
    pub(crate) scalar_v267: bool,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: bool,
    pub(crate) scalar_v364: bool,
    pub(crate) scalar_v369: bool,
    pub(crate) scalar_v370: bool,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: bool,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v388: bool,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v390: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v392: bool,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: bool,
    pub(crate) scalar_v395: f64,
    pub(crate) scalar_v396: bool,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: bool,
    pub(crate) scalar_v399: bool,
    pub(crate) scalar_v400: bool,
    pub(crate) scalar_v401: bool,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: bool,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: bool,
    pub(crate) scalar_v407: bool,
    pub(crate) scalar_v408: bool,
    pub(crate) scalar_v409: bool,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: bool,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v431: bool,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v448: bool,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v457: bool,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v462: bool,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v467: bool,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v469: bool,
    pub(crate) scalar_v470: bool,
    pub(crate) scalar_v471: bool,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: bool,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: bool,
    pub(crate) scalar_v476: bool,
    pub(crate) scalar_v477: bool,
    pub(crate) scalar_v478: f64,
    pub(crate) scalar_v496: bool,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v642: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v702: f64,
    pub(crate) scalar_v868: f64,
    pub(crate) scalar_v1030: f64,
    pub(crate) scalar_v1406: f64,
    pub(crate) scalar_v1407: f64,
    pub(crate) scalar_v1408: f64,
    pub(crate) scalar_v1412: f64,
    pub(crate) scalar_v1447: f64,
    pub(crate) scalar_v1451: f64,
    pub(crate) scalar_v1456: f64,
    pub(crate) scalar_v1457: f64,
    pub(crate) scalar_v1479: f64,
    pub(crate) scalar_v1480: f64,
    pub(crate) scalar_v1481: f64,
    pub(crate) scalar_v1482: f64,
    pub(crate) scalar_v1483: f64,
    pub(crate) scalar_v1484: f64,
    pub(crate) scalar_v1485: f64,
    pub(crate) scalar_v1486: f64,
    pub(crate) scalar_v1487: f64,
    pub(crate) scalar_v1488: f64,
    pub(crate) scalar_v1489: f64,
    pub(crate) scalar_v1490: f64,
    pub(crate) scalar_v1491: f64,
    pub(crate) scalar_v1492: f64,
    pub(crate) scalar_v1495: f64,
    pub(crate) scalar_v1496: f64,
    pub(crate) scalar_v1497: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
    pub(crate) scratch: Option<Box<GenericScratch<125, 16, 19>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<125, 16, 19>>>,
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
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v19: self.scalar_v19,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v84: self.scalar_v84,
            scalar_v85: self.scalar_v85,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v116: self.scalar_v116,
            scalar_v119: self.scalar_v119,
            scalar_v124: self.scalar_v124,
            scalar_v126: self.scalar_v126,
            scalar_v131: self.scalar_v131,
            scalar_v140: self.scalar_v140,
            scalar_v143: self.scalar_v143,
            scalar_v156: self.scalar_v156,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v165: self.scalar_v165,
            scalar_v167: self.scalar_v167,
            scalar_v170: self.scalar_v170,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v198: self.scalar_v198,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v340: self.scalar_v340,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v364: self.scalar_v364,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v379: self.scalar_v379,
            scalar_v388: self.scalar_v388,
            scalar_v389: self.scalar_v389,
            scalar_v390: self.scalar_v390,
            scalar_v391: self.scalar_v391,
            scalar_v392: self.scalar_v392,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v395: self.scalar_v395,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
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
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v414: self.scalar_v414,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v431: self.scalar_v431,
            scalar_v433: self.scalar_v433,
            scalar_v436: self.scalar_v436,
            scalar_v440: self.scalar_v440,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
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
            scalar_v496: self.scalar_v496,
            scalar_v500: self.scalar_v500,
            scalar_v517: self.scalar_v517,
            scalar_v524: self.scalar_v524,
            scalar_v642: self.scalar_v642,
            scalar_v677: self.scalar_v677,
            scalar_v702: self.scalar_v702,
            scalar_v868: self.scalar_v868,
            scalar_v1030: self.scalar_v1030,
            scalar_v1406: self.scalar_v1406,
            scalar_v1407: self.scalar_v1407,
            scalar_v1408: self.scalar_v1408,
            scalar_v1412: self.scalar_v1412,
            scalar_v1447: self.scalar_v1447,
            scalar_v1451: self.scalar_v1451,
            scalar_v1456: self.scalar_v1456,
            scalar_v1457: self.scalar_v1457,
            scalar_v1479: self.scalar_v1479,
            scalar_v1480: self.scalar_v1480,
            scalar_v1481: self.scalar_v1481,
            scalar_v1482: self.scalar_v1482,
            scalar_v1483: self.scalar_v1483,
            scalar_v1484: self.scalar_v1484,
            scalar_v1485: self.scalar_v1485,
            scalar_v1486: self.scalar_v1486,
            scalar_v1487: self.scalar_v1487,
            scalar_v1488: self.scalar_v1488,
            scalar_v1489: self.scalar_v1489,
            scalar_v1490: self.scalar_v1490,
            scalar_v1491: self.scalar_v1491,
            scalar_v1492: self.scalar_v1492,
            scalar_v1495: self.scalar_v1495,
            scalar_v1496: self.scalar_v1496,
            scalar_v1497: self.scalar_v1497,
            scalar_v20: self.scalar_v20,
            scalar_v21: self.scalar_v21,
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
    pub const INTERNAL_NODE_COUNT: usize = 13;
    pub const NODE_COUNT: usize = 16;
    pub const INTERNAL_NODE_NAMES: [&str; 13] = ["di", "gi", "si", "sii", "gdi", "gsi", "bi", "rf", "t", "xt1", "xt2", "ia", "ib"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 86;
    pub const VARIABLE_COUNT: usize = 125;
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
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: false,
            scalar_v19: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: false,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v39: 0.0,
            scalar_v40: false,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v104: 0.0,
            scalar_v105: false,
            scalar_v106: 0.0,
            scalar_v107: false,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v113: false,
            scalar_v114: 0.0,
            scalar_v116: 0.0,
            scalar_v119: 0.0,
            scalar_v124: 0.0,
            scalar_v126: 0.0,
            scalar_v131: 0.0,
            scalar_v140: 0.0,
            scalar_v143: 0.0,
            scalar_v156: 0.0,
            scalar_v161: 0.0,
            scalar_v162: false,
            scalar_v163: false,
            scalar_v165: false,
            scalar_v167: false,
            scalar_v170: 0.0,
            scalar_v178: false,
            scalar_v179: false,
            scalar_v198: 0.0,
            scalar_v228: false,
            scalar_v229: false,
            scalar_v230: false,
            scalar_v265: false,
            scalar_v266: false,
            scalar_v267: false,
            scalar_v340: 0.0,
            scalar_v345: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v352: 0.0,
            scalar_v353: false,
            scalar_v364: false,
            scalar_v369: false,
            scalar_v370: false,
            scalar_v375: false,
            scalar_v376: false,
            scalar_v379: 0.0,
            scalar_v388: false,
            scalar_v389: 0.0,
            scalar_v390: false,
            scalar_v391: 0.0,
            scalar_v392: false,
            scalar_v393: 0.0,
            scalar_v394: false,
            scalar_v395: 0.0,
            scalar_v396: false,
            scalar_v397: 0.0,
            scalar_v398: false,
            scalar_v399: false,
            scalar_v400: false,
            scalar_v401: false,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v404: false,
            scalar_v405: 0.0,
            scalar_v406: false,
            scalar_v407: false,
            scalar_v408: false,
            scalar_v409: false,
            scalar_v410: 0.0,
            scalar_v411: false,
            scalar_v414: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v431: false,
            scalar_v433: 0.0,
            scalar_v436: 0.0,
            scalar_v440: 0.0,
            scalar_v448: false,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v457: false,
            scalar_v458: 0.0,
            scalar_v462: false,
            scalar_v463: 0.0,
            scalar_v467: false,
            scalar_v468: 0.0,
            scalar_v469: false,
            scalar_v470: false,
            scalar_v471: false,
            scalar_v472: 0.0,
            scalar_v473: false,
            scalar_v474: 0.0,
            scalar_v475: false,
            scalar_v476: false,
            scalar_v477: false,
            scalar_v478: 0.0,
            scalar_v496: false,
            scalar_v500: 0.0,
            scalar_v517: 0.0,
            scalar_v524: 0.0,
            scalar_v642: 0.0,
            scalar_v677: 0.0,
            scalar_v702: 0.0,
            scalar_v868: 0.0,
            scalar_v1030: 0.0,
            scalar_v1406: 0.0,
            scalar_v1407: 0.0,
            scalar_v1408: 0.0,
            scalar_v1412: 0.0,
            scalar_v1447: 0.0,
            scalar_v1451: 0.0,
            scalar_v1456: 0.0,
            scalar_v1457: 0.0,
            scalar_v1479: 0.0,
            scalar_v1480: 0.0,
            scalar_v1481: 0.0,
            scalar_v1482: 0.0,
            scalar_v1483: 0.0,
            scalar_v1484: 0.0,
            scalar_v1485: 0.0,
            scalar_v1486: 0.0,
            scalar_v1487: 0.0,
            scalar_v1488: 0.0,
            scalar_v1489: 0.0,
            scalar_v1490: 0.0,
            scalar_v1491: 0.0,
            scalar_v1492: 0.0,
            scalar_v1495: 0.0,
            scalar_v1496: 0.0,
            scalar_v1497: 0.0,
            scalar_v20: 0.0,
            scalar_v21: 0.0,
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
            scalar_v12,
            scalar_v13,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v39,
            scalar_v40,
            scalar_v43,
            scalar_v44,
            scalar_v49,
            scalar_v50,
            scalar_v55,
            scalar_v56,
            scalar_v61,
            scalar_v62,
            scalar_v67,
            scalar_v68,
            scalar_v73,
            scalar_v74,
            scalar_v79,
            scalar_v80,
            scalar_v84,
            scalar_v85,
            scalar_v89,
            scalar_v90,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v109,
            scalar_v110,
            scalar_v113,
            scalar_v114,
            scalar_v116,
            scalar_v119,
            scalar_v124,
            scalar_v126,
            scalar_v131,
            scalar_v140,
            scalar_v143,
            scalar_v156,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v165,
            scalar_v167,
            scalar_v170,
            scalar_v178,
            scalar_v179,
            scalar_v198,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v340,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v352,
            scalar_v353,
            scalar_v364,
            scalar_v369,
            scalar_v370,
            scalar_v375,
            scalar_v376,
            scalar_v379,
            scalar_v388,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v396,
            scalar_v397,
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
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v414,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v431,
            scalar_v433,
            scalar_v436,
            scalar_v440,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v457,
            scalar_v458,
            scalar_v462,
            scalar_v463,
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
            scalar_v496,
            scalar_v500,
            scalar_v517,
            scalar_v524,
            scalar_v642,
            scalar_v677,
            scalar_v702,
            scalar_v868,
            scalar_v1030,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1412,
            scalar_v1447,
            scalar_v1451,
            scalar_v1456,
            scalar_v1457,
            scalar_v1479,
            scalar_v1480,
            scalar_v1481,
            scalar_v1482,
            scalar_v1483,
            scalar_v1484,
            scalar_v1485,
            scalar_v1486,
            scalar_v1487,
            scalar_v1488,
            scalar_v1489,
            scalar_v1490,
            scalar_v1491,
            scalar_v1492,
            scalar_v1495,
            scalar_v1496,
            scalar_v1497,
            scalar_v20,
            scalar_v21,
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
            scalar_v12,
            scalar_v13,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v39,
            scalar_v40,
            scalar_v43,
            scalar_v44,
            scalar_v49,
            scalar_v50,
            scalar_v55,
            scalar_v56,
            scalar_v61,
            scalar_v62,
            scalar_v67,
            scalar_v68,
            scalar_v73,
            scalar_v74,
            scalar_v79,
            scalar_v80,
            scalar_v84,
            scalar_v85,
            scalar_v89,
            scalar_v90,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v109,
            scalar_v110,
            scalar_v113,
            scalar_v114,
            scalar_v116,
            scalar_v119,
            scalar_v124,
            scalar_v126,
            scalar_v131,
            scalar_v140,
            scalar_v143,
            scalar_v156,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v165,
            scalar_v167,
            scalar_v170,
            scalar_v178,
            scalar_v179,
            scalar_v198,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v340,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v352,
            scalar_v353,
            scalar_v364,
            scalar_v369,
            scalar_v370,
            scalar_v375,
            scalar_v376,
            scalar_v379,
            scalar_v388,
            scalar_v389,
            scalar_v390,
            scalar_v391,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v396,
            scalar_v397,
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
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v414,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v431,
            scalar_v433,
            scalar_v436,
            scalar_v440,
            scalar_v448,
            scalar_v449,
            scalar_v450,
            scalar_v457,
            scalar_v458,
            scalar_v462,
            scalar_v463,
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
            scalar_v496,
            scalar_v500,
            scalar_v517,
            scalar_v524,
            scalar_v642,
            scalar_v677,
            scalar_v702,
            scalar_v868,
            scalar_v1030,
            scalar_v1406,
            scalar_v1407,
            scalar_v1408,
            scalar_v1412,
            scalar_v1447,
            scalar_v1451,
            scalar_v1456,
            scalar_v1457,
            scalar_v1479,
            scalar_v1480,
            scalar_v1481,
            scalar_v1482,
            scalar_v1483,
            scalar_v1484,
            scalar_v1485,
            scalar_v1486,
            scalar_v1487,
            scalar_v1488,
            scalar_v1489,
            scalar_v1490,
            scalar_v1491,
            scalar_v1492,
            scalar_v1495,
            scalar_v1496,
            scalar_v1497,
            scalar_v20,
            scalar_v21,
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
            "noise" => { validate_parameter("Noise", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "selft" => { validate_parameter("Selft", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("Trise", value)?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "temp" => { validate_parameter("Temp", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "idsmod" => { validate_parameter("Idsmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igmod" => { validate_parameter("Igmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "capmod" => { validate_parameter("Capmod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
            "cds" => { validate_parameter("Cds", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgspi" => { validate_finite_parameter("Cgspi", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgs0" => { validate_finite_parameter("Cgs0", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdpi" => { validate_finite_parameter("Cgdpi", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdpe" => { validate_parameter("Cgdpe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgd0" => { validate_finite_parameter("Cgd0", value)?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p10" => { validate_parameter("P10", value, Some((-2.0, "-2.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p11" => { validate_parameter("P11", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p20" => { validate_parameter("P20", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p21" => { validate_parameter("P21", value, Some((0.01, "0.01")), false, Some((5.0, "5.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p30" => { validate_parameter("P30", value, Some((-2.0, "-2.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p31" => { validate_parameter("P31", value, Some((0.1, "0.1")), false, Some((5.0, "5.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p40" => { validate_parameter("P40", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p41" => { validate_parameter("P41", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p111" => { validate_parameter("P111", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ij" => { validate_parameter("Ij", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg" => { validate_parameter("Pg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ne" => { validate_parameter("Ne", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vjg" => { validate_parameter("Vjg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rg" => { validate_parameter("Rg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd" => { validate_parameter("Rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd2" => { validate_parameter("Rd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ri" => { validate_parameter("Ri", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("Rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rgd" => { validate_parameter("Rgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ld" => { validate_parameter("Ld", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ls" => { validate_parameter("Ls", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lg" => { validate_parameter("Lg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tau" => { validate_parameter("Tau", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcmin" => { validate_parameter("Rcmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rc" => { validate_parameter("Rc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "crf" => { validate_finite_parameter("Crf", value)?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcin" => { validate_parameter("Rcin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "crfin" => { validate_parameter("Crfin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rtherm" => { validate_parameter("Rth", value, Some((1e-7, "1e-7")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctherm" => { validate_parameter("Cth", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcipk0" => { validate_parameter("Tcipk0", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcp1" => { validate_parameter("Tcp1", value, Some((-0.003, "-0.003")), false, Some((0.0, "0.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccgs0" => { validate_parameter("Tccgs0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccgd0" => { validate_parameter("Tccgd0", value, Some((-0.002, "-0.002")), false, Some((0.002, "0.002")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tclsb0" => { validate_parameter("Tclsb0", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrc" => { validate_finite_parameter("Tcrc", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccrf" => { validate_finite_parameter("Tccrf", value)?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrs" => { validate_parameter("Tcrs", value, Some((0.0, "0.0")), false, Some((0.1, "0.1")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrtherm" => { validate_parameter("TcRtherm", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvpk" => { validate_parameter("TcVpk", value, Some((-0.1, "-0.1")), false, Some((0.1, "0.1")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvjg" => { validate_finite_parameter("TcVjg", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvtr" => { validate_parameter("TcVtr", value, Some((0.0, "0.0")), false, Some((0.01, "0.01")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noiser" => { validate_parameter("NoiseR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noisep" => { validate_parameter("NoiseP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noisec" => { validate_parameter("NoiseC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnc" => { validate_parameter("Fnc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("Kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("Af", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ffe" => { validate_parameter("Ffe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td" => { validate_parameter("Td", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td1" => { validate_finite_parameter("Td1", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmn" => { validate_finite_parameter("Tmn", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "klf" => { validate_finite_parameter("Klf", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgr" => { validate_finite_parameter("Fgr", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "np" => { validate_finite_parameter("Np", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lw" => { validate_finite_parameter("Lw", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("Tnom", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'angelov'", name)),
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
        let v12: f64 = if param_given[3] { 1.0 } else { 0.0 };
        self.scalar_v12 = v12;
        let v13: f64 = p.p3;
        self.scalar_v13 = v13;
        let v15: f64 = (p.p3 + 273.15);
        self.scalar_v15 = v15;
        let v16: f64 = (if (if param_given[3] { 1.0 } else { 0.0 } != 0.0) { v15 } else { 0.0 });
        self.scalar_v16 = v16;
        let v17: bool = (!(if param_given[3] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v17 = v17;
        let v19: f64 = p.p2;
        self.scalar_v19 = v19;
        let v22: f64 = if param_given[85] { 1.0 } else { 0.0 };
        self.scalar_v22 = v22;
        let v23: f64 = p.p85;
        self.scalar_v23 = v23;
        let v24: f64 = (273.15 + p.p85);
        self.scalar_v24 = v24;
        let v25: f64 = (if (if param_given[85] { 1.0 } else { 0.0 } != 0.0) { v24 } else { 0.0 });
        self.scalar_v25 = v25;
        let v26: bool = (!(if param_given[85] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v26 = v26;
        let v28: f64 = (if v26 { 300.15 } else { v25 });
        self.scalar_v28 = v28;
        let v29: f64 = p.p1;
        self.scalar_v29 = v29;
        let v39: f64 = p.p57;
        self.scalar_v39 = v39;
        let v40: bool = (p.p57 > 0.0);
        self.scalar_v40 = v40;
        let v43: f64 = p.p8;
        self.scalar_v43 = v43;
        let v44: f64 = p.p59;
        self.scalar_v44 = v44;
        let v49: f64 = p.p11;
        self.scalar_v49 = v49;
        let v50: f64 = p.p60;
        self.scalar_v50 = v50;
        let v55: f64 = p.p20;
        self.scalar_v55 = v55;
        let v56: f64 = p.p63;
        self.scalar_v56 = v56;
        let v61: f64 = p.p25;
        self.scalar_v61 = v61;
        let v62: f64 = p.p61;
        self.scalar_v62 = v62;
        let v67: f64 = p.p53;
        self.scalar_v67 = v67;
        let v68: f64 = p.p64;
        self.scalar_v68 = v68;
        let v73: f64 = p.p54;
        self.scalar_v73 = v73;
        let v74: f64 = p.p65;
        self.scalar_v74 = v74;
        let v79: f64 = p.p9;
        self.scalar_v79 = v79;
        let v80: f64 = p.p68;
        self.scalar_v80 = v80;
        let v84: f64 = p.p41;
        self.scalar_v84 = v84;
        let v85: f64 = p.p69;
        self.scalar_v85 = v85;
        let v89: f64 = p.p21;
        self.scalar_v89 = v89;
        let v90: f64 = p.p70;
        self.scalar_v90 = v90;
        let v104: f64 = if param_given[39] { 1.0 } else { 0.0 };
        self.scalar_v104 = v104;
        let v105: bool = (!(if param_given[39] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v105 = v105;
        let v106: f64 = if param_given[40] { 1.0 } else { 0.0 };
        self.scalar_v106 = v106;
        let v107: bool = (v105 && (if param_given[40] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v107 = v107;
        let v109: f64 = p.p40;
        self.scalar_v109 = v109;
        let v110: f64 = (0.5 / p.p40);
        self.scalar_v110 = v110;
        let v113: bool = (!v107);
        self.scalar_v113 = v113;
        let v114: f64 = p.p39;
        self.scalar_v114 = v114;
        let v116: f64 = p.p19;
        self.scalar_v116 = v116;
        let v119: f64 = p.p18;
        self.scalar_v119 = v119;
        let v124: f64 = p.p10;
        self.scalar_v124 = v124;
        let v126: f64 = p.p15;
        self.scalar_v126 = v126;
        let v131: f64 = p.p22;
        self.scalar_v131 = v131;
        let v140: f64 = p.p12;
        self.scalar_v140 = v140;
        let v143: f64 = p.p13;
        self.scalar_v143 = v143;
        let v156: f64 = p.p14;
        self.scalar_v156 = v156;
        let v161: f64 = p.p4;
        self.scalar_v161 = v161;
        let v162: bool = (0.0 == p.p4);
        self.scalar_v162 = v162;
        let v163: bool = (1.0 == p.p4);
        self.scalar_v163 = v163;
        let v165: bool = (p.p4 == 2.0);
        self.scalar_v165 = v165;
        let v167: bool = (p.p4 == 3.0);
        self.scalar_v167 = v167;
        let v170: f64 = p.p16;
        self.scalar_v170 = v170;
        let v178: bool = (!v162);
        self.scalar_v178 = v178;
        let v179: bool = (v163 && v178);
        self.scalar_v179 = v179;
        let v198: f64 = p.p17;
        self.scalar_v198 = v198;
        let v228: bool = (v162 || v163);
        self.scalar_v228 = v228;
        let v229: bool = (!v228);
        self.scalar_v229 = v229;
        let v230: bool = (v165 && v229);
        self.scalar_v230 = v230;
        let v265: bool = (v165 || v228);
        self.scalar_v265 = v265;
        let v266: bool = (!v265);
        self.scalar_v266 = v266;
        let v267: bool = (v167 && v266);
        self.scalar_v267 = v267;
        let v340: f64 = p.p52;
        self.scalar_v340 = v340;
        let v345: f64 = p.p43;
        self.scalar_v345 = v345;
        let v346: f64 = p.p44;
        self.scalar_v346 = v346;
        let v347: f64 = p.p46;
        self.scalar_v347 = v347;
        let v352: f64 = p.p5;
        self.scalar_v352 = v352;
        let v353: bool = (0.0 == p.p5);
        self.scalar_v353 = v353;
        let v364: bool = (!v353);
        self.scalar_v364 = v364;
        let v369: bool = (1.0 == p.p5);
        self.scalar_v369 = v369;
        let v370: bool = (v364 && v369);
        self.scalar_v370 = v370;
        let v375: bool = (!v369);
        self.scalar_v375 = v375;
        let v376: bool = (v364 && v375);
        self.scalar_v376 = v376;
        let v379: f64 = p.p38;
        self.scalar_v379 = v379;
        let v388: bool = (p.p53 > 0.0);
        self.scalar_v388 = v388;
        let v389: f64 = p.p55;
        self.scalar_v389 = v389;
        let v390: bool = (p.p55 > 0.0);
        self.scalar_v390 = v390;
        let v391: f64 = p.p47;
        self.scalar_v391 = v391;
        let v392: bool = (p.p47 > 0.0);
        self.scalar_v392 = v392;
        let v393: f64 = p.p45;
        self.scalar_v393 = v393;
        let v394: bool = (p.p45 > 0.0);
        self.scalar_v394 = v394;
        let v395: f64 = p.p42;
        self.scalar_v395 = v395;
        let v396: bool = (p.p42 > 0.0);
        self.scalar_v396 = v396;
        let v397: f64 = p.p50;
        self.scalar_v397 = v397;
        let v398: bool = (p.p50 > 0.0);
        self.scalar_v398 = v398;
        let v399: bool = (p.p46 > 0.0);
        self.scalar_v399 = v399;
        let v400: bool = (p.p43 > 0.0);
        self.scalar_v400 = v400;
        let v401: bool = (p.p44 > 0.0);
        self.scalar_v401 = v401;
        let v402: bool = (v400 || v401);
        self.scalar_v402 = v402;
        let v403: f64 = p.p48;
        self.scalar_v403 = v403;
        let v404: bool = (p.p48 > 0.0);
        self.scalar_v404 = v404;
        let v405: f64 = p.p7;
        self.scalar_v405 = v405;
        let v406: bool = (0.0 == p.p7);
        self.scalar_v406 = v406;
        let v407: bool = (1.0 == p.p7);
        self.scalar_v407 = v407;
        let v408: bool = (!v406);
        self.scalar_v408 = v408;
        let v409: bool = (v407 && v408);
        self.scalar_v409 = v409;
        let v410: f64 = p.p0;
        self.scalar_v410 = v410;
        let v411: bool = (v409 && (p.p0 != 0.0));
        self.scalar_v411 = v411;
        let v414: f64 = p.p73;
        self.scalar_v414 = v414;
        let v417: f64 = p.p72;
        self.scalar_v417 = v417;
        let v418: f64 = p.p71;
        self.scalar_v418 = v418;
        let v419: f64 = (p.p72 * p.p71);
        self.scalar_v419 = v419;
        let v420: f64 = ((v419) as f64).sqrt();
        self.scalar_v420 = v420;
        let v431: bool = ((p.p1 != 0.0) && (p.p57 != 0.0));
        self.scalar_v431 = v431;
        let v433: f64 = p.p51;
        self.scalar_v433 = v433;
        let v436: f64 = p.p27;
        self.scalar_v436 = v436;
        let v440: f64 = p.p23;
        self.scalar_v440 = v440;
        let v448: bool = (!v388);
        self.scalar_v448 = v448;
        let v449: f64 = (if v448 { 0.0 } else { 0.0 });
        self.scalar_v449 = v449;
        let v450: f64 = p.p56;
        self.scalar_v450 = v450;
        let v457: bool = (!v390);
        self.scalar_v457 = v457;
        let v458: f64 = (if v457 { 0.0 } else { 0.0 });
        self.scalar_v458 = v458;
        let v462: bool = (!v392);
        self.scalar_v462 = v462;
        let v463: f64 = (if v462 { 0.0 } else { 0.0 });
        self.scalar_v463 = v463;
        let v467: bool = (!v394);
        self.scalar_v467 = v467;
        let v468: f64 = (if v467 { 0.0 } else { 0.0 });
        self.scalar_v468 = v468;
        let v469: bool = (!v396);
        self.scalar_v469 = v469;
        let v470: bool = (!v398);
        self.scalar_v470 = v470;
        let v471: bool = (v469 && v470);
        self.scalar_v471 = v471;
        let v472: f64 = (if v471 { 0.0 } else { 0.0 });
        self.scalar_v472 = v472;
        let v473: bool = (!v399);
        self.scalar_v473 = v473;
        let v474: f64 = (if v473 { 0.0 } else { 0.0 });
        self.scalar_v474 = v474;
        let v475: bool = (!v402);
        self.scalar_v475 = v475;
        let v476: bool = (!v404);
        self.scalar_v476 = v476;
        let v477: bool = (v475 && v476);
        self.scalar_v477 = v477;
        let v478: f64 = (if v477 { 0.0 } else { 0.0 });
        self.scalar_v478 = v478;
        let v496: bool = (!v431);
        self.scalar_v496 = v496;
        let v500: f64 = (-p.p19);
        self.scalar_v500 = v500;
        let v517: f64 = (-p.p15);
        self.scalar_v517 = v517;
        let v524: f64 = (-p.p22);
        self.scalar_v524 = v524;
        let v642: f64 = (-p.p16);
        self.scalar_v642 = v642;
        let v677: f64 = (if v179 { 0.0 } else { 1.0 });
        self.scalar_v677 = v677;
        let v702: f64 = (p.p12 * v677);
        self.scalar_v702 = v702;
        let v868: f64 = (if v230 { 1.0 } else { 0.0 });
        self.scalar_v868 = v868;
        let v1030: f64 = (if v267 { 1.0 } else { v868 });
        self.scalar_v1030 = v1030;
        let v1406: f64 = (if v353 { 0.0 } else { v1030 });
        self.scalar_v1406 = v1406;
        let v1407: f64 = (if v353 { -1.0 } else { 0.0 });
        self.scalar_v1407 = v1407;
        let v1408: f64 = (if v353 { 1.0 } else { 0.0 });
        self.scalar_v1408 = v1408;
        let v1412: f64 = (if v364 { 0.0 } else { v1406 });
        self.scalar_v1412 = v1412;
        let v1447: f64 = (-v1412);
        self.scalar_v1447 = v1447;
        let v1451: f64 = (p.p38 * v1447);
        self.scalar_v1451 = v1451;
        let v1456: f64 = (-p.p27);
        self.scalar_v1456 = v1456;
        let v1457: f64 = (-p.p23);
        self.scalar_v1457 = v1457;
        let v1479: f64 = (-p.p56);
        self.scalar_v1479 = v1479;
        let v1480: f64 = (-1.0 / p.p55);
        self.scalar_v1480 = v1480;
        let v1481: f64 = (1.0 / p.p55);
        self.scalar_v1481 = v1481;
        let v1482: f64 = (if v390 { v1480 } else { 0.0 });
        self.scalar_v1482 = v1482;
        let v1483: f64 = (if v390 { v1481 } else { 0.0 });
        self.scalar_v1483 = v1483;
        let v1484: f64 = (1.0 / p.p47);
        self.scalar_v1484 = v1484;
        let v1485: f64 = (-1.0 / p.p47);
        self.scalar_v1485 = v1485;
        let v1486: f64 = (if v392 { v1484 } else { 0.0 });
        self.scalar_v1486 = v1486;
        let v1487: f64 = (if v392 { v1485 } else { 0.0 });
        self.scalar_v1487 = v1487;
        let v1488: f64 = (1.0 / p.p45);
        self.scalar_v1488 = v1488;
        let v1489: f64 = (-1.0 / p.p45);
        self.scalar_v1489 = v1489;
        let v1490: f64 = (if v394 { v1488 } else { 0.0 });
        self.scalar_v1490 = v1490;
        let v1491: f64 = (if v394 { v1489 } else { 0.0 });
        self.scalar_v1491 = v1491;
        let v1492: f64 = (if v411 { 1.0 } else { 0.0 });
        self.scalar_v1492 = v1492;
        let v1495: f64 = (1.0 / p.p57);
        self.scalar_v1495 = v1495;
        let v1496: f64 = (if v431 { v1495 } else { 0.0 });
        self.scalar_v1496 = v1496;
        let v1497: f64 = (if v496 { 1e-12 } else { 0.0 });
        self.scalar_v1497 = v1497;
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
        let v20: f64 = (temperature + self.scalar_v19);
        self.scalar_v20 = v20;
        let v21: f64 = (if self.scalar_v17 { self.scalar_v20 } else { self.scalar_v16 });
        self.scalar_v21 = v21;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
