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
    pub(crate) ddt_state_initialized: Box<[bool; 3]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
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
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v388: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v834: bool,
    pub(crate) scalar_v835: f64,
    pub(crate) scalar_v836: f64,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v1517: bool,
    pub(crate) scalar_v1519: f64,
    pub(crate) scalar_v1520: bool,
    pub(crate) scalar_v2417: f64,
    pub(crate) scalar_v2418: f64,
    pub(crate) scalar_v2475: bool,
    pub(crate) scalar_v2522: f64,
    pub(crate) scalar_v2526: f64,
    pub(crate) scalar_v2527: bool,
    pub(crate) scalar_v2535: bool,
    pub(crate) scalar_v2750: bool,
    pub(crate) scalar_v3276: f64,
    pub(crate) scalar_v3300: f64,
    pub(crate) scalar_v3314: f64,
    pub(crate) scalar_v3333: f64,
    pub(crate) scalar_v3334: f64,
    pub(crate) scalar_v10897: f64,
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
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
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
            scalar_v380: self.scalar_v380,
            scalar_v388: self.scalar_v388,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v405: self.scalar_v405,
            scalar_v444: self.scalar_v444,
            scalar_v834: self.scalar_v834,
            scalar_v835: self.scalar_v835,
            scalar_v836: self.scalar_v836,
            scalar_v837: self.scalar_v837,
            scalar_v1517: self.scalar_v1517,
            scalar_v1519: self.scalar_v1519,
            scalar_v1520: self.scalar_v1520,
            scalar_v2417: self.scalar_v2417,
            scalar_v2418: self.scalar_v2418,
            scalar_v2475: self.scalar_v2475,
            scalar_v2522: self.scalar_v2522,
            scalar_v2526: self.scalar_v2526,
            scalar_v2527: self.scalar_v2527,
            scalar_v2535: self.scalar_v2535,
            scalar_v2750: self.scalar_v2750,
            scalar_v3276: self.scalar_v3276,
            scalar_v3300: self.scalar_v3300,
            scalar_v3314: self.scalar_v3314,
            scalar_v3333: self.scalar_v3333,
            scalar_v3334: self.scalar_v3334,
            scalar_v10897: self.scalar_v10897,
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
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
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
            scalar_v380: 0.0,
            scalar_v388: 0.0,
            scalar_v396: 0.0,
            scalar_v397: 0.0,
            scalar_v405: 0.0,
            scalar_v444: 0.0,
            scalar_v834: false,
            scalar_v835: 0.0,
            scalar_v836: 0.0,
            scalar_v837: 0.0,
            scalar_v1517: false,
            scalar_v1519: 0.0,
            scalar_v1520: false,
            scalar_v2417: 0.0,
            scalar_v2418: 0.0,
            scalar_v2475: false,
            scalar_v2522: 0.0,
            scalar_v2526: 0.0,
            scalar_v2527: false,
            scalar_v2535: false,
            scalar_v2750: false,
            scalar_v3276: 0.0,
            scalar_v3300: 0.0,
            scalar_v3314: 0.0,
            scalar_v3333: 0.0,
            scalar_v3334: 0.0,
            scalar_v10897: 0.0,
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
            scalar_v380,
            scalar_v388,
            scalar_v396,
            scalar_v397,
            scalar_v405,
            scalar_v444,
            scalar_v834,
            scalar_v835,
            scalar_v836,
            scalar_v837,
            scalar_v1517,
            scalar_v1519,
            scalar_v1520,
            scalar_v2417,
            scalar_v2418,
            scalar_v2475,
            scalar_v2522,
            scalar_v2526,
            scalar_v2527,
            scalar_v2535,
            scalar_v2750,
            scalar_v3276,
            scalar_v3300,
            scalar_v3314,
            scalar_v3333,
            scalar_v3334,
            scalar_v10897,
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
            scalar_v380,
            scalar_v388,
            scalar_v396,
            scalar_v397,
            scalar_v405,
            scalar_v444,
            scalar_v834,
            scalar_v835,
            scalar_v836,
            scalar_v837,
            scalar_v1517,
            scalar_v1519,
            scalar_v1520,
            scalar_v2417,
            scalar_v2418,
            scalar_v2475,
            scalar_v2522,
            scalar_v2526,
            scalar_v2527,
            scalar_v2535,
            scalar_v2750,
            scalar_v3276,
            scalar_v3300,
            scalar_v3314,
            scalar_v3333,
            scalar_v3334,
            scalar_v10897,
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
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTA", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "subversion" => { validate_finite_parameter("SUBVERSION", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "revision" => { validate_finite_parameter("REVISION", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((-273.0, "-273.0")), false, Some((21.0, "21.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((21.0, "21.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "lmin" => { validate_parameter("LMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "lmax" => { validate_parameter("LMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "wmin" => { validate_parameter("WMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "wmax" => { validate_parameter("WMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "swres" => { validate_parameter("SWRES", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "typep" => { validate_parameter("TYPEP", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "toxo" => { validate_parameter("TOXO", value, Some((5e-10, "5e-10")), false, Some((2e-6, "2e-6")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "epsroxo" => { validate_parameter("EPSROXO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "swqinv" => { validate_parameter("SWQINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "tau" => { validate_parameter("TAU", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "nsubo" => { validate_parameter("NSUBO", value, Some((1e18, "1e18")), false, Some((1e25, "1e25")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "mnsubo" => { validate_parameter("MNSUBO", value, Some((1.0, "1.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "dnsubo" => { validate_parameter("DNSUBO", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "vnsubo" => { validate_parameter("VNSUBO", value, Some((-5.0, "-5.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "nslpo" => { validate_parameter("NSLPO", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((1e24, "1e24")), false, Some((1e27, "1e27")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "dwr" => { validate_finite_parameter("DWR", value)?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cfrl" => { validate_parameter("CFRL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "cfrw" => { validate_parameter("CFRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "rpv" => { validate_parameter("RPV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "rend" => { validate_parameter("REND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "uac" => { validate_parameter("UAC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "uacred" => { validate_parameter("UACRED", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "strshg" => { validate_finite_parameter("STRSHG", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "strpv" => { validate_finite_parameter("STRPV", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "strend" => { validate_finite_parameter("STREND", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "strshs" => { validate_finite_parameter("STRSHS", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "stuac" => { validate_finite_parameter("STUAC", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "chibpo" => { validate_parameter("CHIBPO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "lov" => { validate_parameter("LOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1e22, "1e22")), false, Some((1e26, "1e26")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "igovw" => { validate_parameter("IGOVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "gc2o" => { validate_parameter("GC2O", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "gc3o" => { validate_parameter("GC3O", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "igchvlw" => { validate_parameter("IGCHVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "igovhvw" => { validate_parameter("IGOVHVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "gcohvo" => { validate_parameter("GCOHVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "gc2hvo" => { validate_parameter("GC2HVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "gc3hvo" => { validate_parameter("GC3HVO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "igmax" => { validate_parameter("IGMAX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "racnoise" => { validate_parameter("RACNOISE", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
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
        let v14: f64 = v13.sqrt();
        self.scalar_v14 = v14;
        let v15: f64 = (v14 / v7);
        self.scalar_v15 = v15;
        let v16: f64 = p.p54;
        self.scalar_v16 = v16;
        let v17: f64 = (3.348580862e-29 * p.p54);
        self.scalar_v17 = v17;
        let v18: f64 = v17.sqrt();
        self.scalar_v18 = v18;
        let v19: f64 = (v18 / v7);
        self.scalar_v19 = v19;
        let v20: f64 = p.p30;
        self.scalar_v20 = v20;
        let v22: bool = (p.p30 > 0.0);
        self.scalar_v22 = v22;
        let v24: f64 = (2.3807972 * p.p30);
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
        let v33: f64 = (1.2514650134837189 * v28);
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
        let v43: f64 = (0.5 * p.p48);
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
        let v52: f64 = (273.15 + v50);
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
        let v182: f64 = (p.p35 * p.p0);
        self.scalar_v182 = v182;
        let v183: f64 = p.p34;
        self.scalar_v183 = v183;
        let v184: f64 = (p.p34 * p.p1);
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
        let v195: f64 = (v194 * p.p1);
        self.scalar_v195 = v195;
        let v198: f64 = (p.p0 * p.p1);
        self.scalar_v198 = v198;
        let v201: f64 = p.p33;
        self.scalar_v201 = v201;
        let v202: f64 = (p.p0 + p.p33);
        self.scalar_v202 = v202;
        let v203: f64 = (2.0 * v202);
        self.scalar_v203 = v203;
        let v208: f64 = (12.0 * v202);
        self.scalar_v208 = v208;
        let v249: bool = (!(p.p16 != 0.0));
        self.scalar_v249 = v249;
        let v255: f64 = p.p49;
        self.scalar_v255 = v255;
        let v256: f64 = p.p55;
        self.scalar_v256 = v256;
        let v257: f64 = (p.p55 * v100);
        self.scalar_v257 = v257;
        let v258: f64 = (v257 * v98);
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
        let v266: f64 = (v265 * v100);
        self.scalar_v266 = v266;
        let v267: f64 = (v266 * 1000000000000.0);
        self.scalar_v267 = v267;
        let v268: f64 = (if (p.p49 != 0.0) { v267 } else { 0.0 });
        self.scalar_v268 = v268;
        let v269: f64 = p.p60;
        self.scalar_v269 = v269;
        let v270: f64 = (p.p60 * v100);
        self.scalar_v270 = v270;
        let v271: f64 = (v270 * v98);
        self.scalar_v271 = v271;
        let v272: f64 = (v271 * 1000000000000.0);
        self.scalar_v272 = v272;
        let v273: f64 = (if (p.p49 != 0.0) { v272 } else { 0.0 });
        self.scalar_v273 = v273;
        let v274: f64 = p.p61;
        self.scalar_v274 = v274;
        let v275: f64 = (2.0 * p.p61);
        self.scalar_v275 = v275;
        let v276: f64 = (v275 * p.p53);
        self.scalar_v276 = v276;
        let v277: f64 = (v276 * v100);
        self.scalar_v277 = v277;
        let v278: f64 = (v277 * 1000000000000.0);
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
        let v299: f64 = (2.918995620956536e-49 * p.p50);
        self.scalar_v299 = v299;
        let v300: f64 = v299.sqrt();
        self.scalar_v300 = v300;
        let v301: f64 = (1.3333333333333333 * v300);
        self.scalar_v301 = v301;
        let v303: f64 = (v301 / 1.05457168e-34);
        self.scalar_v303 = v303;
        let v304: f64 = (if (p.p49 != 0.0) { v303 } else { 0.0 });
        self.scalar_v304 = v304;
        let v305: f64 = (v304 * p.p19);
        self.scalar_v305 = v305;
        let v306: f64 = (if (p.p49 != 0.0) { v305 } else { 0.0 });
        self.scalar_v306 = v306;
        let v307: f64 = (if (p.p49 != 0.0) { v306 } else { 0.0 });
        self.scalar_v307 = v307;
        let v308: f64 = (2.918995620956536e-49 * p.p51);
        self.scalar_v308 = v308;
        let v309: f64 = v308.sqrt();
        self.scalar_v309 = v309;
        let v310: f64 = (1.3333333333333333 * v309);
        self.scalar_v310 = v310;
        let v311: f64 = (v310 / 1.05457168e-34);
        self.scalar_v311 = v311;
        let v312: f64 = (if (p.p49 != 0.0) { v311 } else { v304 });
        self.scalar_v312 = v312;
        let v313: f64 = (v312 * p.p19);
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
        let v380: f64 = p.p28;
        self.scalar_v380 = v380;
        let v388: f64 = (0.5 * p.p28);
        self.scalar_v388 = v388;
        let v396: f64 = (1e-32 + p.p28);
        self.scalar_v396 = v396;
        let v397: f64 = v396.sqrt();
        self.scalar_v397 = v397;
        let v405: f64 = p.p25;
        self.scalar_v405 = v405;
        let v444: f64 = (0.75 * v36);
        self.scalar_v444 = v444;
        let v834: bool = (p.p29 < 1e27);
        self.scalar_v834 = v834;
        let v835: f64 = (-p.p17);
        self.scalar_v835 = v835;
        let v836: f64 = p.p18;
        self.scalar_v836 = v836;
        let v837: f64 = (v835 * p.p18);
        self.scalar_v837 = v837;
        let v1517: bool = (!v834);
        self.scalar_v1517 = v1517;
        let v1519: f64 = p.p21;
        self.scalar_v1519 = v1519;
        let v1520: bool = (p.p21 < 1.0);
        self.scalar_v1520 = v1520;
        let v2417: f64 = (0.37 * v46);
        self.scalar_v2417 = v2417;
        let v2418: f64 = (1.0 + v2417);
        self.scalar_v2418 = v2418;
        let v2475: bool = (v36 > 0.0);
        self.scalar_v2475 = v2475;
        let v2522: f64 = p.p41;
        self.scalar_v2522 = v2522;
        let v2526: f64 = (p.p18 * p.p17);
        self.scalar_v2526 = v2526;
        let v2527: bool = (v2526 == -1.0);
        self.scalar_v2527 = v2527;
        let v2535: bool = (p.p49 != 0.0);
        self.scalar_v2535 = v2535;
        let v2750: bool = (p.p18 == 1.0);
        self.scalar_v2750 = v2750;
        let v3276: f64 = p.p22;
        self.scalar_v3276 = v3276;
        let v3300: f64 = (p.p17 * -1.0);
        self.scalar_v3300 = v3300;
        let v3314: f64 = (-v3300);
        self.scalar_v3314 = v3314;
        let v3333: f64 = (0.5 * p.p17);
        self.scalar_v3333 = v3333;
        let v3334: f64 = (0.5 * v3300);
        self.scalar_v3334 = v3334;
        let v10897: f64 = (v186 * -1.0);
        self.scalar_v10897 = v10897;
    }
}
