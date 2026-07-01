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
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: bool,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: bool,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: bool,
    pub(crate) scalar_v112: bool,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v123: bool,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: bool,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: bool,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v215: bool,
    pub(crate) scalar_v217: bool,
    pub(crate) scalar_v219: bool,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v230: bool,
    pub(crate) scalar_v231: bool,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v282: bool,
    pub(crate) scalar_v283: bool,
    pub(crate) scalar_v284: bool,
    pub(crate) scalar_v322: bool,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v324: bool,
    pub(crate) scalar_v397: bool,
    pub(crate) scalar_v398: bool,
    pub(crate) scalar_v399: bool,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: bool,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v441: bool,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v448: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v454: bool,
    pub(crate) scalar_v455: bool,
    pub(crate) scalar_v460: bool,
    pub(crate) scalar_v461: bool,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v692: bool,
    pub(crate) scalar_v693: bool,
    pub(crate) scalar_v694: bool,
    pub(crate) scalar_v695: bool,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v697: bool,
    pub(crate) scalar_v698: f64,
    pub(crate) scalar_v699: bool,
    pub(crate) scalar_v700: f64,
    pub(crate) scalar_v701: bool,
    pub(crate) scalar_v702: f64,
    pub(crate) scalar_v703: bool,
    pub(crate) scalar_v704: bool,
    pub(crate) scalar_v705: bool,
    pub(crate) scalar_v706: bool,
    pub(crate) scalar_v707: bool,
    pub(crate) scalar_v708: f64,
    pub(crate) scalar_v709: bool,
    pub(crate) scalar_v710: bool,
    pub(crate) scalar_v711: bool,
    pub(crate) scalar_v712: bool,
    pub(crate) scalar_v713: f64,
    pub(crate) scalar_v714: bool,
    pub(crate) scalar_v717: f64,
    pub(crate) scalar_v718: f64,
    pub(crate) scalar_v719: f64,
    pub(crate) scalar_v722: f64,
    pub(crate) scalar_v723: f64,
    pub(crate) scalar_v736: f64,
    pub(crate) scalar_v737: bool,
    pub(crate) scalar_v738: bool,
    pub(crate) scalar_v740: f64,
    pub(crate) scalar_v754: f64,
    pub(crate) scalar_v758: f64,
    pub(crate) scalar_v764: bool,
    pub(crate) scalar_v765: f64,
    pub(crate) scalar_v772: bool,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v774: f64,
    pub(crate) scalar_v781: bool,
    pub(crate) scalar_v782: f64,
    pub(crate) scalar_v787: bool,
    pub(crate) scalar_v788: f64,
    pub(crate) scalar_v789: f64,
    pub(crate) scalar_v793: bool,
    pub(crate) scalar_v794: f64,
    pub(crate) scalar_v795: bool,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v797: bool,
    pub(crate) scalar_v798: f64,
    pub(crate) scalar_v799: bool,
    pub(crate) scalar_v800: f64,
    pub(crate) scalar_v801: bool,
    pub(crate) scalar_v802: f64,
    pub(crate) scalar_v803: bool,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v805: bool,
    pub(crate) scalar_v806: f64,
    pub(crate) scalar_v811: bool,
    pub(crate) scalar_v812: f64,
    pub(crate) scalar_v813: f64,
    pub(crate) scalar_v826: bool,
    pub(crate) scalar_v827: f64,
    pub(crate) scalar_v828: bool,
    pub(crate) scalar_v829: f64,
    pub(crate) scalar_v843: bool,
    pub(crate) scalar_v846: f64,
    pub(crate) scalar_v850: f64,
    pub(crate) scalar_v866: f64,
    pub(crate) scalar_v874: f64,
    pub(crate) scalar_v1021: f64,
    pub(crate) scalar_v1062: f64,
    pub(crate) scalar_v1093: f64,
    pub(crate) scalar_v1163: f64,
    pub(crate) scalar_v1298: f64,
    pub(crate) scalar_v1501: f64,
    pub(crate) scalar_v1990: f64,
    pub(crate) scalar_v1991: f64,
    pub(crate) scalar_v2028: f64,
    pub(crate) scalar_v2029: f64,
    pub(crate) scalar_v2030: f64,
    pub(crate) scalar_v2035: f64,
    pub(crate) scalar_v2050: f64,
    pub(crate) scalar_v2051: f64,
    pub(crate) scalar_v2052: f64,
    pub(crate) scalar_v2053: f64,
    pub(crate) scalar_v2070: f64,
    pub(crate) scalar_v2076: f64,
    pub(crate) scalar_v2520: f64,
    pub(crate) scalar_v2521: f64,
    pub(crate) scalar_v2533: f64,
    pub(crate) scalar_v2534: f64,
    pub(crate) scalar_v2535: f64,
    pub(crate) scalar_v2536: f64,
    pub(crate) scalar_v2537: f64,
    pub(crate) scalar_v2538: f64,
    pub(crate) scalar_v2539: f64,
    pub(crate) scalar_v2540: f64,
    pub(crate) scalar_v2541: f64,
    pub(crate) scalar_v2542: f64,
    pub(crate) scalar_v2543: f64,
    pub(crate) scalar_v2544: f64,
    pub(crate) scalar_v2545: f64,
    pub(crate) scalar_v2546: f64,
    pub(crate) scalar_v2555: f64,
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
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v114: self.scalar_v114,
            scalar_v120: self.scalar_v120,
            scalar_v123: self.scalar_v123,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v154: self.scalar_v154,
            scalar_v157: self.scalar_v157,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v167: self.scalar_v167,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v176: self.scalar_v176,
            scalar_v178: self.scalar_v178,
            scalar_v184: self.scalar_v184,
            scalar_v195: self.scalar_v195,
            scalar_v210: self.scalar_v210,
            scalar_v215: self.scalar_v215,
            scalar_v217: self.scalar_v217,
            scalar_v219: self.scalar_v219,
            scalar_v222: self.scalar_v222,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v250: self.scalar_v250,
            scalar_v258: self.scalar_v258,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v408: self.scalar_v408,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v433: self.scalar_v433,
            scalar_v438: self.scalar_v438,
            scalar_v441: self.scalar_v441,
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
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v469: self.scalar_v469,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
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
            scalar_v717: self.scalar_v717,
            scalar_v718: self.scalar_v718,
            scalar_v719: self.scalar_v719,
            scalar_v722: self.scalar_v722,
            scalar_v723: self.scalar_v723,
            scalar_v736: self.scalar_v736,
            scalar_v737: self.scalar_v737,
            scalar_v738: self.scalar_v738,
            scalar_v740: self.scalar_v740,
            scalar_v754: self.scalar_v754,
            scalar_v758: self.scalar_v758,
            scalar_v764: self.scalar_v764,
            scalar_v765: self.scalar_v765,
            scalar_v772: self.scalar_v772,
            scalar_v773: self.scalar_v773,
            scalar_v774: self.scalar_v774,
            scalar_v781: self.scalar_v781,
            scalar_v782: self.scalar_v782,
            scalar_v787: self.scalar_v787,
            scalar_v788: self.scalar_v788,
            scalar_v789: self.scalar_v789,
            scalar_v793: self.scalar_v793,
            scalar_v794: self.scalar_v794,
            scalar_v795: self.scalar_v795,
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
            scalar_v806: self.scalar_v806,
            scalar_v811: self.scalar_v811,
            scalar_v812: self.scalar_v812,
            scalar_v813: self.scalar_v813,
            scalar_v826: self.scalar_v826,
            scalar_v827: self.scalar_v827,
            scalar_v828: self.scalar_v828,
            scalar_v829: self.scalar_v829,
            scalar_v843: self.scalar_v843,
            scalar_v846: self.scalar_v846,
            scalar_v850: self.scalar_v850,
            scalar_v866: self.scalar_v866,
            scalar_v874: self.scalar_v874,
            scalar_v1021: self.scalar_v1021,
            scalar_v1062: self.scalar_v1062,
            scalar_v1093: self.scalar_v1093,
            scalar_v1163: self.scalar_v1163,
            scalar_v1298: self.scalar_v1298,
            scalar_v1501: self.scalar_v1501,
            scalar_v1990: self.scalar_v1990,
            scalar_v1991: self.scalar_v1991,
            scalar_v2028: self.scalar_v2028,
            scalar_v2029: self.scalar_v2029,
            scalar_v2030: self.scalar_v2030,
            scalar_v2035: self.scalar_v2035,
            scalar_v2050: self.scalar_v2050,
            scalar_v2051: self.scalar_v2051,
            scalar_v2052: self.scalar_v2052,
            scalar_v2053: self.scalar_v2053,
            scalar_v2070: self.scalar_v2070,
            scalar_v2076: self.scalar_v2076,
            scalar_v2520: self.scalar_v2520,
            scalar_v2521: self.scalar_v2521,
            scalar_v2533: self.scalar_v2533,
            scalar_v2534: self.scalar_v2534,
            scalar_v2535: self.scalar_v2535,
            scalar_v2536: self.scalar_v2536,
            scalar_v2537: self.scalar_v2537,
            scalar_v2538: self.scalar_v2538,
            scalar_v2539: self.scalar_v2539,
            scalar_v2540: self.scalar_v2540,
            scalar_v2541: self.scalar_v2541,
            scalar_v2542: self.scalar_v2542,
            scalar_v2543: self.scalar_v2543,
            scalar_v2544: self.scalar_v2544,
            scalar_v2545: self.scalar_v2545,
            scalar_v2546: self.scalar_v2546,
            scalar_v2555: self.scalar_v2555,
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
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v105: 0.0,
            scalar_v106: false,
            scalar_v108: false,
            scalar_v109: false,
            scalar_v110: 0.0,
            scalar_v111: false,
            scalar_v112: false,
            scalar_v114: 0.0,
            scalar_v120: 0.0,
            scalar_v123: false,
            scalar_v142: 0.0,
            scalar_v143: false,
            scalar_v144: 0.0,
            scalar_v145: false,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v151: false,
            scalar_v152: 0.0,
            scalar_v154: 0.0,
            scalar_v157: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v167: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v176: 0.0,
            scalar_v178: 0.0,
            scalar_v184: 0.0,
            scalar_v195: 0.0,
            scalar_v210: 0.0,
            scalar_v215: false,
            scalar_v217: false,
            scalar_v219: false,
            scalar_v222: 0.0,
            scalar_v230: false,
            scalar_v231: false,
            scalar_v250: 0.0,
            scalar_v258: 0.0,
            scalar_v282: false,
            scalar_v283: false,
            scalar_v284: false,
            scalar_v322: false,
            scalar_v323: false,
            scalar_v324: false,
            scalar_v397: false,
            scalar_v398: false,
            scalar_v399: false,
            scalar_v408: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v423: false,
            scalar_v433: 0.0,
            scalar_v438: 0.0,
            scalar_v441: false,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v448: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v453: 0.0,
            scalar_v454: false,
            scalar_v455: false,
            scalar_v460: false,
            scalar_v461: false,
            scalar_v469: 0.0,
            scalar_v473: 0.0,
            scalar_v474: 0.0,
            scalar_v692: false,
            scalar_v693: false,
            scalar_v694: false,
            scalar_v695: false,
            scalar_v696: 0.0,
            scalar_v697: false,
            scalar_v698: 0.0,
            scalar_v699: false,
            scalar_v700: 0.0,
            scalar_v701: false,
            scalar_v702: 0.0,
            scalar_v703: false,
            scalar_v704: false,
            scalar_v705: false,
            scalar_v706: false,
            scalar_v707: false,
            scalar_v708: 0.0,
            scalar_v709: false,
            scalar_v710: false,
            scalar_v711: false,
            scalar_v712: false,
            scalar_v713: 0.0,
            scalar_v714: false,
            scalar_v717: 0.0,
            scalar_v718: 0.0,
            scalar_v719: 0.0,
            scalar_v722: 0.0,
            scalar_v723: 0.0,
            scalar_v736: 0.0,
            scalar_v737: false,
            scalar_v738: false,
            scalar_v740: 0.0,
            scalar_v754: 0.0,
            scalar_v758: 0.0,
            scalar_v764: false,
            scalar_v765: 0.0,
            scalar_v772: false,
            scalar_v773: 0.0,
            scalar_v774: 0.0,
            scalar_v781: false,
            scalar_v782: 0.0,
            scalar_v787: false,
            scalar_v788: 0.0,
            scalar_v789: 0.0,
            scalar_v793: false,
            scalar_v794: 0.0,
            scalar_v795: false,
            scalar_v796: 0.0,
            scalar_v797: false,
            scalar_v798: 0.0,
            scalar_v799: false,
            scalar_v800: 0.0,
            scalar_v801: false,
            scalar_v802: 0.0,
            scalar_v803: false,
            scalar_v804: 0.0,
            scalar_v805: false,
            scalar_v806: 0.0,
            scalar_v811: false,
            scalar_v812: 0.0,
            scalar_v813: 0.0,
            scalar_v826: false,
            scalar_v827: 0.0,
            scalar_v828: false,
            scalar_v829: 0.0,
            scalar_v843: false,
            scalar_v846: 0.0,
            scalar_v850: 0.0,
            scalar_v866: 0.0,
            scalar_v874: 0.0,
            scalar_v1021: 0.0,
            scalar_v1062: 0.0,
            scalar_v1093: 0.0,
            scalar_v1163: 0.0,
            scalar_v1298: 0.0,
            scalar_v1501: 0.0,
            scalar_v1990: 0.0,
            scalar_v1991: 0.0,
            scalar_v2028: 0.0,
            scalar_v2029: 0.0,
            scalar_v2030: 0.0,
            scalar_v2035: 0.0,
            scalar_v2050: 0.0,
            scalar_v2051: 0.0,
            scalar_v2052: 0.0,
            scalar_v2053: 0.0,
            scalar_v2070: 0.0,
            scalar_v2076: 0.0,
            scalar_v2520: 0.0,
            scalar_v2521: 0.0,
            scalar_v2533: 0.0,
            scalar_v2534: 0.0,
            scalar_v2535: 0.0,
            scalar_v2536: 0.0,
            scalar_v2537: 0.0,
            scalar_v2538: 0.0,
            scalar_v2539: 0.0,
            scalar_v2540: 0.0,
            scalar_v2541: 0.0,
            scalar_v2542: 0.0,
            scalar_v2543: 0.0,
            scalar_v2544: 0.0,
            scalar_v2545: 0.0,
            scalar_v2546: 0.0,
            scalar_v2555: 0.0,
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
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v95,
            scalar_v96,
            scalar_v100,
            scalar_v101,
            scalar_v105,
            scalar_v106,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v114,
            scalar_v120,
            scalar_v123,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v148,
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v157,
            scalar_v159,
            scalar_v160,
            scalar_v167,
            scalar_v171,
            scalar_v172,
            scalar_v176,
            scalar_v178,
            scalar_v184,
            scalar_v195,
            scalar_v210,
            scalar_v215,
            scalar_v217,
            scalar_v219,
            scalar_v222,
            scalar_v230,
            scalar_v231,
            scalar_v250,
            scalar_v258,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v408,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v433,
            scalar_v438,
            scalar_v441,
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
            scalar_v460,
            scalar_v461,
            scalar_v469,
            scalar_v473,
            scalar_v474,
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
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v722,
            scalar_v723,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v740,
            scalar_v754,
            scalar_v758,
            scalar_v764,
            scalar_v765,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v781,
            scalar_v782,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v793,
            scalar_v794,
            scalar_v795,
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
            scalar_v806,
            scalar_v811,
            scalar_v812,
            scalar_v813,
            scalar_v826,
            scalar_v827,
            scalar_v828,
            scalar_v829,
            scalar_v843,
            scalar_v846,
            scalar_v850,
            scalar_v866,
            scalar_v874,
            scalar_v1021,
            scalar_v1062,
            scalar_v1093,
            scalar_v1163,
            scalar_v1298,
            scalar_v1501,
            scalar_v1990,
            scalar_v1991,
            scalar_v2028,
            scalar_v2029,
            scalar_v2030,
            scalar_v2035,
            scalar_v2050,
            scalar_v2051,
            scalar_v2052,
            scalar_v2053,
            scalar_v2070,
            scalar_v2076,
            scalar_v2520,
            scalar_v2521,
            scalar_v2533,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v2538,
            scalar_v2539,
            scalar_v2540,
            scalar_v2541,
            scalar_v2542,
            scalar_v2543,
            scalar_v2544,
            scalar_v2545,
            scalar_v2546,
            scalar_v2555,
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
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v95,
            scalar_v96,
            scalar_v100,
            scalar_v101,
            scalar_v105,
            scalar_v106,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v114,
            scalar_v120,
            scalar_v123,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v148,
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v157,
            scalar_v159,
            scalar_v160,
            scalar_v167,
            scalar_v171,
            scalar_v172,
            scalar_v176,
            scalar_v178,
            scalar_v184,
            scalar_v195,
            scalar_v210,
            scalar_v215,
            scalar_v217,
            scalar_v219,
            scalar_v222,
            scalar_v230,
            scalar_v231,
            scalar_v250,
            scalar_v258,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v408,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v433,
            scalar_v438,
            scalar_v441,
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
            scalar_v460,
            scalar_v461,
            scalar_v469,
            scalar_v473,
            scalar_v474,
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
            scalar_v717,
            scalar_v718,
            scalar_v719,
            scalar_v722,
            scalar_v723,
            scalar_v736,
            scalar_v737,
            scalar_v738,
            scalar_v740,
            scalar_v754,
            scalar_v758,
            scalar_v764,
            scalar_v765,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v781,
            scalar_v782,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v793,
            scalar_v794,
            scalar_v795,
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
            scalar_v806,
            scalar_v811,
            scalar_v812,
            scalar_v813,
            scalar_v826,
            scalar_v827,
            scalar_v828,
            scalar_v829,
            scalar_v843,
            scalar_v846,
            scalar_v850,
            scalar_v866,
            scalar_v874,
            scalar_v1021,
            scalar_v1062,
            scalar_v1093,
            scalar_v1163,
            scalar_v1298,
            scalar_v1501,
            scalar_v1990,
            scalar_v1991,
            scalar_v2028,
            scalar_v2029,
            scalar_v2030,
            scalar_v2035,
            scalar_v2050,
            scalar_v2051,
            scalar_v2052,
            scalar_v2053,
            scalar_v2070,
            scalar_v2076,
            scalar_v2520,
            scalar_v2521,
            scalar_v2533,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v2538,
            scalar_v2539,
            scalar_v2540,
            scalar_v2541,
            scalar_v2542,
            scalar_v2543,
            scalar_v2544,
            scalar_v2545,
            scalar_v2546,
            scalar_v2555,
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
        let v74: f64 = p.p58;
        self.scalar_v74 = v74;
        let v75: f64 = p.p59;
        self.scalar_v75 = v75;
        let v76: f64 = p.p75;
        self.scalar_v76 = v76;
        let v81: f64 = p.p9;
        self.scalar_v81 = v81;
        let v82: f64 = p.p78;
        self.scalar_v82 = v82;
        let v95: f64 = p.p45;
        self.scalar_v95 = v95;
        let v96: f64 = p.p79;
        self.scalar_v96 = v96;
        let v100: f64 = p.p21;
        self.scalar_v100 = v100;
        let v101: f64 = p.p81;
        self.scalar_v101 = v101;
        let v105: f64 = p.p4;
        self.scalar_v105 = v105;
        let v106: bool = (1.0 == p.p4);
        self.scalar_v106 = v106;
        let v108: bool = (p.p4 == 4.0);
        self.scalar_v108 = v108;
        let v109: bool = (v106 || v108);
        self.scalar_v109 = v109;
        let v110: f64 = p.p6;
        self.scalar_v110 = v110;
        let v111: bool = (4.0 == p.p6);
        self.scalar_v111 = v111;
        let v112: bool = (v109 && v111);
        self.scalar_v112 = v112;
        let v114: f64 = p.p62;
        self.scalar_v114 = v114;
        let v120: f64 = p.p63;
        self.scalar_v120 = v120;
        let v123: bool = (!v112);
        self.scalar_v123 = v123;
        let v142: f64 = if param_given[43] { 1.0 } else { 0.0 };
        self.scalar_v142 = v142;
        let v143: bool = (!(if param_given[43] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v143 = v143;
        let v144: f64 = if param_given[44] { 1.0 } else { 0.0 };
        self.scalar_v144 = v144;
        let v145: bool = (v143 && (if param_given[44] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v145 = v145;
        let v147: f64 = p.p44;
        self.scalar_v147 = v147;
        let v148: f64 = (0.5 / p.p44);
        self.scalar_v148 = v148;
        let v151: bool = (!v145);
        self.scalar_v151 = v151;
        let v152: f64 = p.p43;
        self.scalar_v152 = v152;
        let v154: f64 = p.p19;
        self.scalar_v154 = v154;
        let v157: f64 = p.p64;
        self.scalar_v157 = v157;
        let v159: f64 = p.p11;
        self.scalar_v159 = v159;
        let v160: f64 = p.p18;
        self.scalar_v160 = v160;
        let v167: f64 = p.p69;
        self.scalar_v167 = v167;
        let v171: f64 = p.p13;
        self.scalar_v171 = v171;
        let v172: f64 = p.p70;
        self.scalar_v172 = v172;
        let v176: f64 = p.p10;
        self.scalar_v176 = v176;
        let v178: f64 = p.p15;
        self.scalar_v178 = v178;
        let v184: f64 = p.p22;
        self.scalar_v184 = v184;
        let v195: f64 = p.p12;
        self.scalar_v195 = v195;
        let v210: f64 = p.p14;
        self.scalar_v210 = v210;
        let v215: bool = (0.0 == p.p4);
        self.scalar_v215 = v215;
        let v217: bool = (p.p4 == 2.0);
        self.scalar_v217 = v217;
        let v219: bool = (p.p4 == 3.0);
        self.scalar_v219 = v219;
        let v222: f64 = p.p16;
        self.scalar_v222 = v222;
        let v230: bool = (!v215);
        self.scalar_v230 = v230;
        let v231: bool = (v106 && v230);
        self.scalar_v231 = v231;
        let v250: f64 = p.p17;
        self.scalar_v250 = v250;
        let v258: f64 = p.p23;
        self.scalar_v258 = v258;
        let v282: bool = (v106 || v215);
        self.scalar_v282 = v282;
        let v283: bool = (!v282);
        self.scalar_v283 = v283;
        let v284: bool = (v217 && v283);
        self.scalar_v284 = v284;
        let v322: bool = (v217 || v282);
        self.scalar_v322 = v322;
        let v323: bool = (!v322);
        self.scalar_v323 = v323;
        let v324: bool = (v219 && v323);
        self.scalar_v324 = v324;
        let v397: bool = (v219 || v322);
        self.scalar_v397 = v397;
        let v398: bool = (!v397);
        self.scalar_v398 = v398;
        let v399: bool = (v108 && v398);
        self.scalar_v399 = v399;
        let v408: f64 = p.p65;
        self.scalar_v408 = v408;
        let v419: f64 = p.p47;
        self.scalar_v419 = v419;
        let v420: f64 = p.p48;
        self.scalar_v420 = v420;
        let v421: f64 = p.p50;
        self.scalar_v421 = v421;
        let v422: f64 = p.p5;
        self.scalar_v422 = v422;
        let v423: bool = (0.0 == p.p5);
        self.scalar_v423 = v423;
        let v433: f64 = p.p83;
        self.scalar_v433 = v433;
        let v438: f64 = p.p84;
        self.scalar_v438 = v438;
        let v441: bool = (!v423);
        self.scalar_v441 = v441;
        let v446: f64 = p.p85;
        self.scalar_v446 = v446;
        let v447: f64 = (-p.p85);
        self.scalar_v447 = v447;
        let v448: f64 = (p.p83 * v447);
        self.scalar_v448 = v448;
        let v449: f64 = { let limexp_arg = v448; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v449 = v449;
        let v450: f64 = (if v441 { v449 } else { 0.0 });
        self.scalar_v450 = v450;
        let v451: f64 = (p.p84 * v447);
        self.scalar_v451 = v451;
        let v452: f64 = { let limexp_arg = v451; if limexp_arg < 80.0 { limexp_arg.exp() } else { 5.54062238439351e34 * (1.0 + (limexp_arg - 80.0)) } };
        self.scalar_v452 = v452;
        let v453: f64 = (if v441 { v452 } else { 0.0 });
        self.scalar_v453 = v453;
        let v454: bool = (1.0 == p.p5);
        self.scalar_v454 = v454;
        let v455: bool = (v441 && v454);
        self.scalar_v455 = v455;
        let v460: bool = (!v454);
        self.scalar_v460 = v460;
        let v461: bool = (v441 && v460);
        self.scalar_v461 = v461;
        let v469: f64 = p.p42;
        self.scalar_v469 = v469;
        let v473: f64 = p.p82;
        self.scalar_v473 = v473;
        let v474: f64 = (0.001 * p.p82);
        self.scalar_v474 = v474;
        let v692: bool = (p.p58 > 0.0);
        self.scalar_v692 = v692;
        let v693: bool = (p.p63 > 0.0);
        self.scalar_v693 = v693;
        let v694: bool = (p.p62 > 0.0);
        self.scalar_v694 = v694;
        let v695: bool = (v693 || v694);
        self.scalar_v695 = v695;
        let v696: f64 = p.p60;
        self.scalar_v696 = v696;
        let v697: bool = (p.p60 > 0.0);
        self.scalar_v697 = v697;
        let v698: f64 = p.p51;
        self.scalar_v698 = v698;
        let v699: bool = (p.p51 > 0.0);
        self.scalar_v699 = v699;
        let v700: f64 = p.p49;
        self.scalar_v700 = v700;
        let v701: bool = (p.p49 > 0.0);
        self.scalar_v701 = v701;
        let v702: f64 = p.p46;
        self.scalar_v702 = v702;
        let v703: bool = (p.p46 > 0.0);
        self.scalar_v703 = v703;
        let v704: bool = (p.p50 > 0.0);
        self.scalar_v704 = v704;
        let v705: bool = (p.p47 > 0.0);
        self.scalar_v705 = v705;
        let v706: bool = (p.p48 > 0.0);
        self.scalar_v706 = v706;
        let v707: bool = (v705 || v706);
        self.scalar_v707 = v707;
        let v708: f64 = p.p7;
        self.scalar_v708 = v708;
        let v709: bool = (0.0 == p.p7);
        self.scalar_v709 = v709;
        let v710: bool = (1.0 == p.p7);
        self.scalar_v710 = v710;
        let v711: bool = (!v709);
        self.scalar_v711 = v711;
        let v712: bool = (v710 && v711);
        self.scalar_v712 = v712;
        let v713: f64 = p.p0;
        self.scalar_v713 = v713;
        let v714: bool = (v712 && (p.p0 != 0.0));
        self.scalar_v714 = v714;
        let v717: f64 = p.p87;
        self.scalar_v717 = v717;
        let v718: f64 = p.p86;
        self.scalar_v718 = v718;
        let v719: f64 = p.p88;
        self.scalar_v719 = v719;
        let v722: f64 = (p.p87 * p.p86);
        self.scalar_v722 = v722;
        let v723: f64 = ((v722) as f64).sqrt();
        self.scalar_v723 = v723;
        let v736: f64 = p.p90;
        self.scalar_v736 = v736;
        let v737: bool = (p.p90 > 0.0);
        self.scalar_v737 = v737;
        let v738: bool = (p.p1 == 1.0);
        self.scalar_v738 = v738;
        let v740: f64 = p.p56;
        self.scalar_v740 = v740;
        let v754: f64 = p.p28;
        self.scalar_v754 = v754;
        let v758: f64 = p.p24;
        self.scalar_v758 = v758;
        let v764: bool = (!v692);
        self.scalar_v764 = v764;
        let v765: f64 = (if v764 { 0.0 } else { 0.0 });
        self.scalar_v765 = v765;
        let v772: bool = (!v695);
        self.scalar_v772 = v772;
        let v773: f64 = (if v772 { 0.0 } else { 0.0 });
        self.scalar_v773 = v773;
        let v774: f64 = p.p61;
        self.scalar_v774 = v774;
        let v781: bool = (!v697);
        self.scalar_v781 = v781;
        let v782: f64 = (if v781 { 0.0 } else { 0.0 });
        self.scalar_v782 = v782;
        let v787: bool = (!v699);
        self.scalar_v787 = v787;
        let v788: f64 = (if v787 { 0.0 } else { 0.0 });
        self.scalar_v788 = v788;
        let v789: f64 = (if (p.p0 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v789 = v789;
        let v793: bool = (!v701);
        self.scalar_v793 = v793;
        let v794: f64 = (if v793 { 0.0 } else { 0.0 });
        self.scalar_v794 = v794;
        let v795: bool = (v703 && (p.p0 != 0.0));
        self.scalar_v795 = v795;
        let v796: f64 = (if v795 { 0.0 } else { 0.0 });
        self.scalar_v796 = v796;
        let v797: bool = (!v703);
        self.scalar_v797 = v797;
        let v798: f64 = (if v797 { 0.0 } else { 0.0 });
        self.scalar_v798 = v798;
        let v799: bool = (v704 && (p.p0 != 0.0));
        self.scalar_v799 = v799;
        let v800: f64 = (if v799 { 0.0 } else { 0.0 });
        self.scalar_v800 = v800;
        let v801: bool = (!v704);
        self.scalar_v801 = v801;
        let v802: f64 = (if v801 { 0.0 } else { 0.0 });
        self.scalar_v802 = v802;
        let v803: bool = (v707 && (p.p0 != 0.0));
        self.scalar_v803 = v803;
        let v804: f64 = (if v803 { 0.0 } else { 0.0 });
        self.scalar_v804 = v804;
        let v805: bool = (!v707);
        self.scalar_v805 = v805;
        let v806: f64 = (if v805 { 0.0 } else { 0.0 });
        self.scalar_v806 = v806;
        let v811: bool = (v709 && (p.p0 != 0.0));
        self.scalar_v811 = v811;
        let v812: f64 = (if v811 { 0.0 } else { 0.0 });
        self.scalar_v812 = v812;
        let v813: f64 = (if v714 { 0.0 } else { 0.0 });
        self.scalar_v813 = v813;
        let v826: bool = (v714 && v737);
        self.scalar_v826 = v826;
        let v827: f64 = (if v826 { 0.0 } else { 0.0 });
        self.scalar_v827 = v827;
        let v828: bool = ((p.p0 != 0.0) && v737);
        self.scalar_v828 = v828;
        let v829: f64 = (if v828 { 0.0 } else { 0.0 });
        self.scalar_v829 = v829;
        let v843: bool = (!v738);
        self.scalar_v843 = v843;
        let v846: f64 = (-p.p19);
        self.scalar_v846 = v846;
        let v850: f64 = (-p.p64);
        self.scalar_v850 = v850;
        let v866: f64 = (-p.p15);
        self.scalar_v866 = v866;
        let v874: f64 = (-p.p22);
        self.scalar_v874 = v874;
        let v1021: f64 = (-p.p16);
        self.scalar_v1021 = v1021;
        let v1062: f64 = (if v231 { 0.0 } else { 1.0 });
        self.scalar_v1062 = v1062;
        let v1093: f64 = (p.p12 * v1062);
        self.scalar_v1093 = v1093;
        let v1163: f64 = (-p.p23);
        self.scalar_v1163 = v1163;
        let v1298: f64 = (if v284 { 1.0 } else { 0.0 });
        self.scalar_v1298 = v1298;
        let v1501: f64 = (if v324 { 1.0 } else { v1298 });
        self.scalar_v1501 = v1501;
        let v1990: f64 = (-p.p65);
        self.scalar_v1990 = v1990;
        let v1991: f64 = (-1.0 + v1990);
        self.scalar_v1991 = v1991;
        let v2028: f64 = (if v423 { 0.0 } else { v1501 });
        self.scalar_v2028 = v2028;
        let v2029: f64 = (if v423 { -1.0 } else { 0.0 });
        self.scalar_v2029 = v2029;
        let v2030: f64 = (if v423 { 1.0 } else { 0.0 });
        self.scalar_v2030 = v2030;
        let v2035: f64 = (if v441 { 0.0 } else { v2028 });
        self.scalar_v2035 = v2035;
        let v2050: f64 = (if v441 { 1.0 } else { v2030 });
        self.scalar_v2050 = v2050;
        let v2051: f64 = (if v441 { -1.0 } else { v2029 });
        self.scalar_v2051 = v2051;
        let v2052: f64 = (p.p85 * v2050);
        self.scalar_v2052 = v2052;
        let v2053: f64 = (p.p85 * v2051);
        self.scalar_v2053 = v2053;
        let v2070: f64 = (-v2035);
        self.scalar_v2070 = v2070;
        let v2076: f64 = (p.p42 * v2070);
        self.scalar_v2076 = v2076;
        let v2520: f64 = (-p.p28);
        self.scalar_v2520 = v2520;
        let v2521: f64 = (-p.p24);
        self.scalar_v2521 = v2521;
        let v2533: f64 = (-p.p61);
        self.scalar_v2533 = v2533;
        let v2534: f64 = (-1.0 / p.p60);
        self.scalar_v2534 = v2534;
        let v2535: f64 = (1.0 / p.p60);
        self.scalar_v2535 = v2535;
        let v2536: f64 = (if v697 { v2534 } else { 0.0 });
        self.scalar_v2536 = v2536;
        let v2537: f64 = (if v697 { v2535 } else { 0.0 });
        self.scalar_v2537 = v2537;
        let v2538: f64 = (-1.0 / p.p51);
        self.scalar_v2538 = v2538;
        let v2539: f64 = (1.0 / p.p51);
        self.scalar_v2539 = v2539;
        let v2540: f64 = (if v699 { v2538 } else { 0.0 });
        self.scalar_v2540 = v2540;
        let v2541: f64 = (if v699 { v2539 } else { 0.0 });
        self.scalar_v2541 = v2541;
        let v2542: f64 = (-1.0 / p.p49);
        self.scalar_v2542 = v2542;
        let v2543: f64 = (1.0 / p.p49);
        self.scalar_v2543 = v2543;
        let v2544: f64 = (if v701 { v2542 } else { 0.0 });
        self.scalar_v2544 = v2544;
        let v2545: f64 = (if v701 { v2543 } else { 0.0 });
        self.scalar_v2545 = v2545;
        let v2546: f64 = (if v714 { 1.0 } else { 0.0 });
        self.scalar_v2546 = v2546;
        let v2555: f64 = (if v843 { 1e-12 } else { 0.0 });
        self.scalar_v2555 = v2555;
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
