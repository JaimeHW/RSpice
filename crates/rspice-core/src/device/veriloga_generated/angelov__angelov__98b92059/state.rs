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
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: bool,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: bool,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v134: bool,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: bool,
    pub(crate) scalar_v184: bool,
    pub(crate) scalar_v186: bool,
    pub(crate) scalar_v188: bool,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v199: bool,
    pub(crate) scalar_v200: bool,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v249: bool,
    pub(crate) scalar_v250: bool,
    pub(crate) scalar_v251: bool,
    pub(crate) scalar_v286: bool,
    pub(crate) scalar_v287: bool,
    pub(crate) scalar_v288: bool,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v394: f64,
    pub(crate) scalar_v395: bool,
    pub(crate) scalar_v406: bool,
    pub(crate) scalar_v411: bool,
    pub(crate) scalar_v412: bool,
    pub(crate) scalar_v417: bool,
    pub(crate) scalar_v418: bool,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v455: f64,
    pub(crate) scalar_v456: bool,
    pub(crate) scalar_v457: bool,
    pub(crate) scalar_v458: bool,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: bool,
    pub(crate) scalar_v464: bool,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v475: bool,
    pub(crate) scalar_v476: bool,
    pub(crate) scalar_v477: bool,
    pub(crate) scalar_v526: bool,
    pub(crate) scalar_v527: f64,
    pub(crate) scalar_v528: bool,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v530: bool,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v532: bool,
    pub(crate) scalar_v533: f64,
    pub(crate) scalar_v534: bool,
    pub(crate) scalar_v535: f64,
    pub(crate) scalar_v536: bool,
    pub(crate) scalar_v537: bool,
    pub(crate) scalar_v538: bool,
    pub(crate) scalar_v539: bool,
    pub(crate) scalar_v540: bool,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: bool,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v544: bool,
    pub(crate) scalar_v545: bool,
    pub(crate) scalar_v546: bool,
    pub(crate) scalar_v547: bool,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v549: bool,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v553: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v557: f64,
    pub(crate) scalar_v558: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v572: bool,
    pub(crate) scalar_v573: bool,
    pub(crate) scalar_v575: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v585: bool,
    pub(crate) scalar_v592: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v604: bool,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v613: bool,
    pub(crate) scalar_v614: f64,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v619: f64,
    pub(crate) scalar_v620: bool,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v625: bool,
    pub(crate) scalar_v626: f64,
    pub(crate) scalar_v634: bool,
    pub(crate) scalar_v635: f64,
    pub(crate) scalar_v636: bool,
    pub(crate) scalar_v637: bool,
    pub(crate) scalar_v642: bool,
    pub(crate) scalar_v643: bool,
    pub(crate) scalar_v644: f64,
    pub(crate) scalar_v648: bool,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: bool,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v652: f64,
    pub(crate) scalar_v662: bool,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v664: bool,
    pub(crate) scalar_v665: bool,
    pub(crate) scalar_v670: bool,
    pub(crate) scalar_v671: bool,
    pub(crate) scalar_v672: f64,
    pub(crate) scalar_v673: bool,
    pub(crate) scalar_v674: f64,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v688: bool,
    pub(crate) scalar_v689: f64,
    pub(crate) scalar_v690: f64,
    pub(crate) scalar_v691: bool,
    pub(crate) scalar_v692: f64,
    pub(crate) scalar_v693: f64,
    pub(crate) scalar_v706: bool,
    pub(crate) scalar_v710: f64,
    pub(crate) scalar_v727: f64,
    pub(crate) scalar_v734: f64,
    pub(crate) scalar_v852: f64,
    pub(crate) scalar_v887: f64,
    pub(crate) scalar_v912: f64,
    pub(crate) scalar_v1078: f64,
    pub(crate) scalar_v1240: f64,
    pub(crate) scalar_v1644: f64,
    pub(crate) scalar_v1645: f64,
    pub(crate) scalar_v1646: f64,
    pub(crate) scalar_v1650: f64,
    pub(crate) scalar_v1685: f64,
    pub(crate) scalar_v1689: f64,
    pub(crate) scalar_v1690: f64,
    pub(crate) scalar_v1691: f64,
    pub(crate) scalar_v1692: f64,
    pub(crate) scalar_v1698: f64,
    pub(crate) scalar_v1703: f64,
    pub(crate) scalar_v1708: f64,
    pub(crate) scalar_v1709: f64,
    pub(crate) scalar_v1784: f64,
    pub(crate) scalar_v1790: f64,
    pub(crate) scalar_v1848: f64,
    pub(crate) scalar_v1904: f64,
    pub(crate) scalar_v1905: f64,
    pub(crate) scalar_v1927: f64,
    pub(crate) scalar_v1928: f64,
    pub(crate) scalar_v1929: f64,
    pub(crate) scalar_v1930: f64,
    pub(crate) scalar_v1931: f64,
    pub(crate) scalar_v1932: f64,
    pub(crate) scalar_v1933: f64,
    pub(crate) scalar_v1934: f64,
    pub(crate) scalar_v1935: f64,
    pub(crate) scalar_v1936: f64,
    pub(crate) scalar_v1937: f64,
    pub(crate) scalar_v1938: f64,
    pub(crate) scalar_v1939: f64,
    pub(crate) scalar_v1940: f64,
    pub(crate) scalar_v1965: f64,
    pub(crate) scalar_v1972: f64,
    pub(crate) scalar_v1973: f64,
    pub(crate) scalar_v1974: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v21: f64,
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
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v137: self.scalar_v137,
            scalar_v140: self.scalar_v140,
            scalar_v145: self.scalar_v145,
            scalar_v147: self.scalar_v147,
            scalar_v152: self.scalar_v152,
            scalar_v161: self.scalar_v161,
            scalar_v164: self.scalar_v164,
            scalar_v177: self.scalar_v177,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v186: self.scalar_v186,
            scalar_v188: self.scalar_v188,
            scalar_v191: self.scalar_v191,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v219: self.scalar_v219,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v361: self.scalar_v361,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v371: self.scalar_v371,
            scalar_v384: self.scalar_v384,
            scalar_v394: self.scalar_v394,
            scalar_v395: self.scalar_v395,
            scalar_v406: self.scalar_v406,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v421: self.scalar_v421,
            scalar_v432: self.scalar_v432,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v443: self.scalar_v443,
            scalar_v444: self.scalar_v444,
            scalar_v455: self.scalar_v455,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v470: self.scalar_v470,
            scalar_v475: self.scalar_v475,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v526: self.scalar_v526,
            scalar_v527: self.scalar_v527,
            scalar_v528: self.scalar_v528,
            scalar_v529: self.scalar_v529,
            scalar_v530: self.scalar_v530,
            scalar_v531: self.scalar_v531,
            scalar_v532: self.scalar_v532,
            scalar_v533: self.scalar_v533,
            scalar_v534: self.scalar_v534,
            scalar_v535: self.scalar_v535,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v538: self.scalar_v538,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v549: self.scalar_v549,
            scalar_v552: self.scalar_v552,
            scalar_v553: self.scalar_v553,
            scalar_v554: self.scalar_v554,
            scalar_v557: self.scalar_v557,
            scalar_v558: self.scalar_v558,
            scalar_v571: self.scalar_v571,
            scalar_v572: self.scalar_v572,
            scalar_v573: self.scalar_v573,
            scalar_v575: self.scalar_v575,
            scalar_v578: self.scalar_v578,
            scalar_v585: self.scalar_v585,
            scalar_v592: self.scalar_v592,
            scalar_v596: self.scalar_v596,
            scalar_v604: self.scalar_v604,
            scalar_v605: self.scalar_v605,
            scalar_v606: self.scalar_v606,
            scalar_v613: self.scalar_v613,
            scalar_v614: self.scalar_v614,
            scalar_v618: self.scalar_v618,
            scalar_v619: self.scalar_v619,
            scalar_v620: self.scalar_v620,
            scalar_v621: self.scalar_v621,
            scalar_v625: self.scalar_v625,
            scalar_v626: self.scalar_v626,
            scalar_v634: self.scalar_v634,
            scalar_v635: self.scalar_v635,
            scalar_v636: self.scalar_v636,
            scalar_v637: self.scalar_v637,
            scalar_v642: self.scalar_v642,
            scalar_v643: self.scalar_v643,
            scalar_v644: self.scalar_v644,
            scalar_v648: self.scalar_v648,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v652: self.scalar_v652,
            scalar_v662: self.scalar_v662,
            scalar_v663: self.scalar_v663,
            scalar_v664: self.scalar_v664,
            scalar_v665: self.scalar_v665,
            scalar_v670: self.scalar_v670,
            scalar_v671: self.scalar_v671,
            scalar_v672: self.scalar_v672,
            scalar_v673: self.scalar_v673,
            scalar_v674: self.scalar_v674,
            scalar_v675: self.scalar_v675,
            scalar_v688: self.scalar_v688,
            scalar_v689: self.scalar_v689,
            scalar_v690: self.scalar_v690,
            scalar_v691: self.scalar_v691,
            scalar_v692: self.scalar_v692,
            scalar_v693: self.scalar_v693,
            scalar_v706: self.scalar_v706,
            scalar_v710: self.scalar_v710,
            scalar_v727: self.scalar_v727,
            scalar_v734: self.scalar_v734,
            scalar_v852: self.scalar_v852,
            scalar_v887: self.scalar_v887,
            scalar_v912: self.scalar_v912,
            scalar_v1078: self.scalar_v1078,
            scalar_v1240: self.scalar_v1240,
            scalar_v1644: self.scalar_v1644,
            scalar_v1645: self.scalar_v1645,
            scalar_v1646: self.scalar_v1646,
            scalar_v1650: self.scalar_v1650,
            scalar_v1685: self.scalar_v1685,
            scalar_v1689: self.scalar_v1689,
            scalar_v1690: self.scalar_v1690,
            scalar_v1691: self.scalar_v1691,
            scalar_v1692: self.scalar_v1692,
            scalar_v1698: self.scalar_v1698,
            scalar_v1703: self.scalar_v1703,
            scalar_v1708: self.scalar_v1708,
            scalar_v1709: self.scalar_v1709,
            scalar_v1784: self.scalar_v1784,
            scalar_v1790: self.scalar_v1790,
            scalar_v1848: self.scalar_v1848,
            scalar_v1904: self.scalar_v1904,
            scalar_v1905: self.scalar_v1905,
            scalar_v1927: self.scalar_v1927,
            scalar_v1928: self.scalar_v1928,
            scalar_v1929: self.scalar_v1929,
            scalar_v1930: self.scalar_v1930,
            scalar_v1931: self.scalar_v1931,
            scalar_v1932: self.scalar_v1932,
            scalar_v1933: self.scalar_v1933,
            scalar_v1934: self.scalar_v1934,
            scalar_v1935: self.scalar_v1935,
            scalar_v1936: self.scalar_v1936,
            scalar_v1937: self.scalar_v1937,
            scalar_v1938: self.scalar_v1938,
            scalar_v1939: self.scalar_v1939,
            scalar_v1940: self.scalar_v1940,
            scalar_v1965: self.scalar_v1965,
            scalar_v1972: self.scalar_v1972,
            scalar_v1973: self.scalar_v1973,
            scalar_v1974: self.scalar_v1974,
            scalar_v20: self.scalar_v20,
            scalar_v21: self.scalar_v21,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
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
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v125: 0.0,
            scalar_v126: false,
            scalar_v127: 0.0,
            scalar_v128: false,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v134: false,
            scalar_v135: 0.0,
            scalar_v137: 0.0,
            scalar_v140: 0.0,
            scalar_v145: 0.0,
            scalar_v147: 0.0,
            scalar_v152: 0.0,
            scalar_v161: 0.0,
            scalar_v164: 0.0,
            scalar_v177: 0.0,
            scalar_v182: 0.0,
            scalar_v183: false,
            scalar_v184: false,
            scalar_v186: false,
            scalar_v188: false,
            scalar_v191: 0.0,
            scalar_v199: false,
            scalar_v200: false,
            scalar_v219: 0.0,
            scalar_v249: false,
            scalar_v250: false,
            scalar_v251: false,
            scalar_v286: false,
            scalar_v287: false,
            scalar_v288: false,
            scalar_v361: 0.0,
            scalar_v366: 0.0,
            scalar_v367: 0.0,
            scalar_v371: 0.0,
            scalar_v384: 0.0,
            scalar_v394: 0.0,
            scalar_v395: false,
            scalar_v406: false,
            scalar_v411: false,
            scalar_v412: false,
            scalar_v417: false,
            scalar_v418: false,
            scalar_v421: 0.0,
            scalar_v432: 0.0,
            scalar_v437: 0.0,
            scalar_v438: 0.0,
            scalar_v443: 0.0,
            scalar_v444: 0.0,
            scalar_v455: 0.0,
            scalar_v456: false,
            scalar_v457: false,
            scalar_v458: false,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v463: false,
            scalar_v464: false,
            scalar_v470: 0.0,
            scalar_v475: false,
            scalar_v476: false,
            scalar_v477: false,
            scalar_v526: false,
            scalar_v527: 0.0,
            scalar_v528: false,
            scalar_v529: 0.0,
            scalar_v530: false,
            scalar_v531: 0.0,
            scalar_v532: false,
            scalar_v533: 0.0,
            scalar_v534: false,
            scalar_v535: 0.0,
            scalar_v536: false,
            scalar_v537: false,
            scalar_v538: false,
            scalar_v539: false,
            scalar_v540: false,
            scalar_v541: 0.0,
            scalar_v542: false,
            scalar_v543: 0.0,
            scalar_v544: false,
            scalar_v545: false,
            scalar_v546: false,
            scalar_v547: false,
            scalar_v548: 0.0,
            scalar_v549: false,
            scalar_v552: 0.0,
            scalar_v553: 0.0,
            scalar_v554: 0.0,
            scalar_v557: 0.0,
            scalar_v558: 0.0,
            scalar_v571: 0.0,
            scalar_v572: false,
            scalar_v573: false,
            scalar_v575: 0.0,
            scalar_v578: 0.0,
            scalar_v585: false,
            scalar_v592: 0.0,
            scalar_v596: 0.0,
            scalar_v604: false,
            scalar_v605: 0.0,
            scalar_v606: 0.0,
            scalar_v613: false,
            scalar_v614: 0.0,
            scalar_v618: false,
            scalar_v619: 0.0,
            scalar_v620: false,
            scalar_v621: 0.0,
            scalar_v625: false,
            scalar_v626: 0.0,
            scalar_v634: false,
            scalar_v635: 0.0,
            scalar_v636: false,
            scalar_v637: false,
            scalar_v642: false,
            scalar_v643: false,
            scalar_v644: 0.0,
            scalar_v648: false,
            scalar_v649: 0.0,
            scalar_v650: false,
            scalar_v651: 0.0,
            scalar_v652: 0.0,
            scalar_v662: false,
            scalar_v663: 0.0,
            scalar_v664: false,
            scalar_v665: false,
            scalar_v670: false,
            scalar_v671: false,
            scalar_v672: 0.0,
            scalar_v673: false,
            scalar_v674: 0.0,
            scalar_v675: 0.0,
            scalar_v688: false,
            scalar_v689: 0.0,
            scalar_v690: 0.0,
            scalar_v691: false,
            scalar_v692: 0.0,
            scalar_v693: 0.0,
            scalar_v706: false,
            scalar_v710: 0.0,
            scalar_v727: 0.0,
            scalar_v734: 0.0,
            scalar_v852: 0.0,
            scalar_v887: 0.0,
            scalar_v912: 0.0,
            scalar_v1078: 0.0,
            scalar_v1240: 0.0,
            scalar_v1644: 0.0,
            scalar_v1645: 0.0,
            scalar_v1646: 0.0,
            scalar_v1650: 0.0,
            scalar_v1685: 0.0,
            scalar_v1689: 0.0,
            scalar_v1690: 0.0,
            scalar_v1691: 0.0,
            scalar_v1692: 0.0,
            scalar_v1698: 0.0,
            scalar_v1703: 0.0,
            scalar_v1708: 0.0,
            scalar_v1709: 0.0,
            scalar_v1784: 0.0,
            scalar_v1790: 0.0,
            scalar_v1848: 0.0,
            scalar_v1904: 0.0,
            scalar_v1905: 0.0,
            scalar_v1927: 0.0,
            scalar_v1928: 0.0,
            scalar_v1929: 0.0,
            scalar_v1930: 0.0,
            scalar_v1931: 0.0,
            scalar_v1932: 0.0,
            scalar_v1933: 0.0,
            scalar_v1934: 0.0,
            scalar_v1935: 0.0,
            scalar_v1936: 0.0,
            scalar_v1937: 0.0,
            scalar_v1938: 0.0,
            scalar_v1939: 0.0,
            scalar_v1940: 0.0,
            scalar_v1965: 0.0,
            scalar_v1972: 0.0,
            scalar_v1973: 0.0,
            scalar_v1974: 0.0,
            scalar_v20: 0.0,
            scalar_v21: 0.0,
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
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v102,
            scalar_v103,
            scalar_v107,
            scalar_v108,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v130,
            scalar_v131,
            scalar_v134,
            scalar_v135,
            scalar_v137,
            scalar_v140,
            scalar_v145,
            scalar_v147,
            scalar_v152,
            scalar_v161,
            scalar_v164,
            scalar_v177,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v186,
            scalar_v188,
            scalar_v191,
            scalar_v199,
            scalar_v200,
            scalar_v219,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v361,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v384,
            scalar_v394,
            scalar_v395,
            scalar_v406,
            scalar_v411,
            scalar_v412,
            scalar_v417,
            scalar_v418,
            scalar_v421,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v443,
            scalar_v444,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v470,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v557,
            scalar_v558,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v575,
            scalar_v578,
            scalar_v585,
            scalar_v592,
            scalar_v596,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v613,
            scalar_v614,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v625,
            scalar_v626,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v642,
            scalar_v643,
            scalar_v644,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v662,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v674,
            scalar_v675,
            scalar_v688,
            scalar_v689,
            scalar_v690,
            scalar_v691,
            scalar_v692,
            scalar_v693,
            scalar_v706,
            scalar_v710,
            scalar_v727,
            scalar_v734,
            scalar_v852,
            scalar_v887,
            scalar_v912,
            scalar_v1078,
            scalar_v1240,
            scalar_v1644,
            scalar_v1645,
            scalar_v1646,
            scalar_v1650,
            scalar_v1685,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1698,
            scalar_v1703,
            scalar_v1708,
            scalar_v1709,
            scalar_v1784,
            scalar_v1790,
            scalar_v1848,
            scalar_v1904,
            scalar_v1905,
            scalar_v1927,
            scalar_v1928,
            scalar_v1929,
            scalar_v1930,
            scalar_v1931,
            scalar_v1932,
            scalar_v1933,
            scalar_v1934,
            scalar_v1935,
            scalar_v1936,
            scalar_v1937,
            scalar_v1938,
            scalar_v1939,
            scalar_v1940,
            scalar_v1965,
            scalar_v1972,
            scalar_v1973,
            scalar_v1974,
            scalar_v20,
            scalar_v21,
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
            scalar_v85,
            scalar_v86,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v102,
            scalar_v103,
            scalar_v107,
            scalar_v108,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v130,
            scalar_v131,
            scalar_v134,
            scalar_v135,
            scalar_v137,
            scalar_v140,
            scalar_v145,
            scalar_v147,
            scalar_v152,
            scalar_v161,
            scalar_v164,
            scalar_v177,
            scalar_v182,
            scalar_v183,
            scalar_v184,
            scalar_v186,
            scalar_v188,
            scalar_v191,
            scalar_v199,
            scalar_v200,
            scalar_v219,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v361,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v384,
            scalar_v394,
            scalar_v395,
            scalar_v406,
            scalar_v411,
            scalar_v412,
            scalar_v417,
            scalar_v418,
            scalar_v421,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v443,
            scalar_v444,
            scalar_v455,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v470,
            scalar_v475,
            scalar_v476,
            scalar_v477,
            scalar_v526,
            scalar_v527,
            scalar_v528,
            scalar_v529,
            scalar_v530,
            scalar_v531,
            scalar_v532,
            scalar_v533,
            scalar_v534,
            scalar_v535,
            scalar_v536,
            scalar_v537,
            scalar_v538,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v552,
            scalar_v553,
            scalar_v554,
            scalar_v557,
            scalar_v558,
            scalar_v571,
            scalar_v572,
            scalar_v573,
            scalar_v575,
            scalar_v578,
            scalar_v585,
            scalar_v592,
            scalar_v596,
            scalar_v604,
            scalar_v605,
            scalar_v606,
            scalar_v613,
            scalar_v614,
            scalar_v618,
            scalar_v619,
            scalar_v620,
            scalar_v621,
            scalar_v625,
            scalar_v626,
            scalar_v634,
            scalar_v635,
            scalar_v636,
            scalar_v637,
            scalar_v642,
            scalar_v643,
            scalar_v644,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v662,
            scalar_v663,
            scalar_v664,
            scalar_v665,
            scalar_v670,
            scalar_v671,
            scalar_v672,
            scalar_v673,
            scalar_v674,
            scalar_v675,
            scalar_v688,
            scalar_v689,
            scalar_v690,
            scalar_v691,
            scalar_v692,
            scalar_v693,
            scalar_v706,
            scalar_v710,
            scalar_v727,
            scalar_v734,
            scalar_v852,
            scalar_v887,
            scalar_v912,
            scalar_v1078,
            scalar_v1240,
            scalar_v1644,
            scalar_v1645,
            scalar_v1646,
            scalar_v1650,
            scalar_v1685,
            scalar_v1689,
            scalar_v1690,
            scalar_v1691,
            scalar_v1692,
            scalar_v1698,
            scalar_v1703,
            scalar_v1708,
            scalar_v1709,
            scalar_v1784,
            scalar_v1790,
            scalar_v1848,
            scalar_v1904,
            scalar_v1905,
            scalar_v1927,
            scalar_v1928,
            scalar_v1929,
            scalar_v1930,
            scalar_v1931,
            scalar_v1932,
            scalar_v1933,
            scalar_v1934,
            scalar_v1935,
            scalar_v1936,
            scalar_v1937,
            scalar_v1938,
            scalar_v1939,
            scalar_v1940,
            scalar_v1965,
            scalar_v1972,
            scalar_v1973,
            scalar_v1974,
            scalar_v20,
            scalar_v21,
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
        let v67: f64 = p.p28;
        self.scalar_v67 = v67;
        let v68: f64 = p.p62;
        self.scalar_v68 = v68;
        let v73: f64 = p.p53;
        self.scalar_v73 = v73;
        let v74: f64 = p.p64;
        self.scalar_v74 = v74;
        let v79: f64 = p.p54;
        self.scalar_v79 = v79;
        let v80: f64 = p.p65;
        self.scalar_v80 = v80;
        let v85: f64 = p.p9;
        self.scalar_v85 = v85;
        let v86: f64 = p.p68;
        self.scalar_v86 = v86;
        let v90: f64 = p.p29;
        self.scalar_v90 = v90;
        let v91: f64 = p.p30;
        self.scalar_v91 = v91;
        let v92: f64 = (p.p68 * p.p30);
        self.scalar_v92 = v92;
        let v96: f64 = p.p35;
        self.scalar_v96 = v96;
        let v97: f64 = p.p36;
        self.scalar_v97 = v97;
        let v98: f64 = (p.p68 * p.p36);
        self.scalar_v98 = v98;
        let v102: f64 = p.p41;
        self.scalar_v102 = v102;
        let v103: f64 = p.p69;
        self.scalar_v103 = v103;
        let v107: f64 = p.p21;
        self.scalar_v107 = v107;
        let v108: f64 = p.p70;
        self.scalar_v108 = v108;
        let v125: f64 = if param_given[39] { 1.0 } else { 0.0 };
        self.scalar_v125 = v125;
        let v126: bool = (!(if param_given[39] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v126 = v126;
        let v127: f64 = if param_given[40] { 1.0 } else { 0.0 };
        self.scalar_v127 = v127;
        let v128: bool = (v126 && (if param_given[40] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v128 = v128;
        let v130: f64 = p.p40;
        self.scalar_v130 = v130;
        let v131: f64 = (0.5 / p.p40);
        self.scalar_v131 = v131;
        let v134: bool = (!v128);
        self.scalar_v134 = v134;
        let v135: f64 = p.p39;
        self.scalar_v135 = v135;
        let v137: f64 = p.p19;
        self.scalar_v137 = v137;
        let v140: f64 = p.p18;
        self.scalar_v140 = v140;
        let v145: f64 = p.p10;
        self.scalar_v145 = v145;
        let v147: f64 = p.p15;
        self.scalar_v147 = v147;
        let v152: f64 = p.p22;
        self.scalar_v152 = v152;
        let v161: f64 = p.p12;
        self.scalar_v161 = v161;
        let v164: f64 = p.p13;
        self.scalar_v164 = v164;
        let v177: f64 = p.p14;
        self.scalar_v177 = v177;
        let v182: f64 = p.p4;
        self.scalar_v182 = v182;
        let v183: bool = (0.0 == p.p4);
        self.scalar_v183 = v183;
        let v184: bool = (1.0 == p.p4);
        self.scalar_v184 = v184;
        let v186: bool = (p.p4 == 2.0);
        self.scalar_v186 = v186;
        let v188: bool = (p.p4 == 3.0);
        self.scalar_v188 = v188;
        let v191: f64 = p.p16;
        self.scalar_v191 = v191;
        let v199: bool = (!v183);
        self.scalar_v199 = v199;
        let v200: bool = (v184 && v199);
        self.scalar_v200 = v200;
        let v219: f64 = p.p17;
        self.scalar_v219 = v219;
        let v249: bool = (v183 || v184);
        self.scalar_v249 = v249;
        let v250: bool = (!v249);
        self.scalar_v250 = v250;
        let v251: bool = (v186 && v250);
        self.scalar_v251 = v251;
        let v286: bool = (v186 || v249);
        self.scalar_v286 = v286;
        let v287: bool = (!v286);
        self.scalar_v287 = v287;
        let v288: bool = (v188 && v287);
        self.scalar_v288 = v288;
        let v361: f64 = p.p52;
        self.scalar_v361 = v361;
        let v366: f64 = p.p43;
        self.scalar_v366 = v366;
        let v367: f64 = p.p44;
        self.scalar_v367 = v367;
        let v371: f64 = p.p46;
        self.scalar_v371 = v371;
        let v384: f64 = p.p66;
        self.scalar_v384 = v384;
        let v394: f64 = p.p5;
        self.scalar_v394 = v394;
        let v395: bool = (0.0 == p.p5);
        self.scalar_v395 = v395;
        let v406: bool = (!v395);
        self.scalar_v406 = v406;
        let v411: bool = (1.0 == p.p5);
        self.scalar_v411 = v411;
        let v412: bool = (v406 && v411);
        self.scalar_v412 = v412;
        let v417: bool = (!v411);
        self.scalar_v417 = v417;
        let v418: bool = (v406 && v417);
        self.scalar_v418 = v418;
        let v421: f64 = p.p38;
        self.scalar_v421 = v421;
        let v432: f64 = p.p37;
        self.scalar_v432 = v432;
        let v437: f64 = p.p31;
        self.scalar_v437 = v437;
        let v438: f64 = p.p32;
        self.scalar_v438 = v438;
        let v443: f64 = p.p33;
        self.scalar_v443 = v443;
        let v444: f64 = p.p34;
        self.scalar_v444 = v444;
        let v455: f64 = p.p6;
        self.scalar_v455 = v455;
        let v456: bool = (0.0 == p.p6);
        self.scalar_v456 = v456;
        let v457: bool = (1.0 == p.p6);
        self.scalar_v457 = v457;
        let v458: bool = (2.0 == p.p6);
        self.scalar_v458 = v458;
        let v459: f64 = p.p24;
        self.scalar_v459 = v459;
        let v460: f64 = (if v456 { p.p24 } else { 0.0 });
        self.scalar_v460 = v460;
        let v461: f64 = p.p26;
        self.scalar_v461 = v461;
        let v462: f64 = (if v456 { p.p26 } else { 0.0 });
        self.scalar_v462 = v462;
        let v463: bool = (!v456);
        self.scalar_v463 = v463;
        let v464: bool = (v457 && v463);
        self.scalar_v464 = v464;
        let v470: f64 = (2.0 * p.p37);
        self.scalar_v470 = v470;
        let v475: bool = (v456 || v457);
        self.scalar_v475 = v475;
        let v476: bool = (!v475);
        self.scalar_v476 = v476;
        let v477: bool = (v458 && v476);
        self.scalar_v477 = v477;
        let v526: bool = (p.p53 > 0.0);
        self.scalar_v526 = v526;
        let v527: f64 = p.p55;
        self.scalar_v527 = v527;
        let v528: bool = (p.p55 > 0.0);
        self.scalar_v528 = v528;
        let v529: f64 = p.p47;
        self.scalar_v529 = v529;
        let v530: bool = (p.p47 > 0.0);
        self.scalar_v530 = v530;
        let v531: f64 = p.p45;
        self.scalar_v531 = v531;
        let v532: bool = (p.p45 > 0.0);
        self.scalar_v532 = v532;
        let v533: f64 = p.p42;
        self.scalar_v533 = v533;
        let v534: bool = (p.p42 > 0.0);
        self.scalar_v534 = v534;
        let v535: f64 = p.p50;
        self.scalar_v535 = v535;
        let v536: bool = (p.p50 > 0.0);
        self.scalar_v536 = v536;
        let v537: bool = (p.p46 > 0.0);
        self.scalar_v537 = v537;
        let v538: bool = (p.p43 > 0.0);
        self.scalar_v538 = v538;
        let v539: bool = (p.p44 > 0.0);
        self.scalar_v539 = v539;
        let v540: bool = (v538 || v539);
        self.scalar_v540 = v540;
        let v541: f64 = p.p48;
        self.scalar_v541 = v541;
        let v542: bool = (p.p48 > 0.0);
        self.scalar_v542 = v542;
        let v543: f64 = p.p7;
        self.scalar_v543 = v543;
        let v544: bool = (0.0 == p.p7);
        self.scalar_v544 = v544;
        let v545: bool = (1.0 == p.p7);
        self.scalar_v545 = v545;
        let v546: bool = (!v544);
        self.scalar_v546 = v546;
        let v547: bool = (v545 && v546);
        self.scalar_v547 = v547;
        let v548: f64 = p.p0;
        self.scalar_v548 = v548;
        let v549: bool = (v547 && (p.p0 != 0.0));
        self.scalar_v549 = v549;
        let v552: f64 = p.p72;
        self.scalar_v552 = v552;
        let v553: f64 = p.p71;
        self.scalar_v553 = v553;
        let v554: f64 = p.p73;
        self.scalar_v554 = v554;
        let v557: f64 = (p.p72 * p.p71);
        self.scalar_v557 = v557;
        let v558: f64 = ((v557) as f64).sqrt();
        self.scalar_v558 = v558;
        let v571: f64 = p.p75;
        self.scalar_v571 = v571;
        let v572: bool = (p.p75 > 0.0);
        self.scalar_v572 = v572;
        let v573: bool = ((p.p1 != 0.0) && (p.p57 != 0.0));
        self.scalar_v573 = v573;
        let v575: f64 = p.p51;
        self.scalar_v575 = v575;
        let v578: f64 = (p.p51 / 3.0);
        self.scalar_v578 = v578;
        let v585: bool = (!v458);
        self.scalar_v585 = v585;
        let v592: f64 = p.p27;
        self.scalar_v592 = v592;
        let v596: f64 = p.p23;
        self.scalar_v596 = v596;
        let v604: bool = (!v526);
        self.scalar_v604 = v604;
        let v605: f64 = (if v604 { 0.0 } else { 0.0 });
        self.scalar_v605 = v605;
        let v606: f64 = p.p56;
        self.scalar_v606 = v606;
        let v613: bool = (!v528);
        self.scalar_v613 = v613;
        let v614: f64 = (if v613 { 0.0 } else { 0.0 });
        self.scalar_v614 = v614;
        let v618: bool = (v530 && (p.p0 != 0.0));
        self.scalar_v618 = v618;
        let v619: f64 = (if v618 { 0.0 } else { 0.0 });
        self.scalar_v619 = v619;
        let v620: bool = (!v530);
        self.scalar_v620 = v620;
        let v621: f64 = (if v620 { 0.0 } else { 0.0 });
        self.scalar_v621 = v621;
        let v625: bool = (!v532);
        self.scalar_v625 = v625;
        let v626: f64 = (if v625 { 0.0 } else { 0.0 });
        self.scalar_v626 = v626;
        let v634: bool = (v534 && (p.p0 != 0.0));
        self.scalar_v634 = v634;
        let v635: f64 = (if v634 { 0.0 } else { 0.0 });
        self.scalar_v635 = v635;
        let v636: bool = (!v534);
        self.scalar_v636 = v636;
        let v637: bool = (v536 && v636);
        self.scalar_v637 = v637;
        let v642: bool = (!v536);
        self.scalar_v642 = v642;
        let v643: bool = (v636 && v642);
        self.scalar_v643 = v643;
        let v644: f64 = (if v643 { 0.0 } else { 0.0 });
        self.scalar_v644 = v644;
        let v648: bool = (v537 && (p.p0 != 0.0));
        self.scalar_v648 = v648;
        let v649: f64 = (if v648 { 0.0 } else { 0.0 });
        self.scalar_v649 = v649;
        let v650: bool = (!v537);
        self.scalar_v650 = v650;
        let v651: f64 = (if v650 { 0.0 } else { 0.0 });
        self.scalar_v651 = v651;
        let v652: f64 = p.p49;
        self.scalar_v652 = v652;
        let v662: bool = (v540 && (p.p0 != 0.0));
        self.scalar_v662 = v662;
        let v663: f64 = (if v662 { 0.0 } else { 0.0 });
        self.scalar_v663 = v663;
        let v664: bool = (!v540);
        self.scalar_v664 = v664;
        let v665: bool = (v542 && v664);
        self.scalar_v665 = v665;
        let v670: bool = (!v542);
        self.scalar_v670 = v670;
        let v671: bool = (v664 && v670);
        self.scalar_v671 = v671;
        let v672: f64 = (if v671 { 0.0 } else { 0.0 });
        self.scalar_v672 = v672;
        let v673: bool = (v544 && (p.p0 != 0.0));
        self.scalar_v673 = v673;
        let v674: f64 = (if v673 { 0.0 } else { 0.0 });
        self.scalar_v674 = v674;
        let v675: f64 = (if v549 { 0.0 } else { 0.0 });
        self.scalar_v675 = v675;
        let v688: bool = (v549 && v572);
        self.scalar_v688 = v688;
        let v689: f64 = (if v688 { 0.0 } else { 0.0 });
        self.scalar_v689 = v689;
        let v690: f64 = (if (p.p0 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v690 = v690;
        let v691: bool = ((p.p0 != 0.0) && v572);
        self.scalar_v691 = v691;
        let v692: f64 = (if v691 { 0.0 } else { 0.0 });
        self.scalar_v692 = v692;
        let v693: f64 = p.p58;
        self.scalar_v693 = v693;
        let v706: bool = (!v573);
        self.scalar_v706 = v706;
        let v710: f64 = (-p.p19);
        self.scalar_v710 = v710;
        let v727: f64 = (-p.p15);
        self.scalar_v727 = v727;
        let v734: f64 = (-p.p22);
        self.scalar_v734 = v734;
        let v852: f64 = (-p.p16);
        self.scalar_v852 = v852;
        let v887: f64 = (if v200 { 0.0 } else { 1.0 });
        self.scalar_v887 = v887;
        let v912: f64 = (p.p12 * v887);
        self.scalar_v912 = v912;
        let v1078: f64 = (if v251 { 1.0 } else { 0.0 });
        self.scalar_v1078 = v1078;
        let v1240: f64 = (if v288 { 1.0 } else { v1078 });
        self.scalar_v1240 = v1240;
        let v1644: f64 = (if v395 { 0.0 } else { v1240 });
        self.scalar_v1644 = v1644;
        let v1645: f64 = (if v395 { -1.0 } else { 0.0 });
        self.scalar_v1645 = v1645;
        let v1646: f64 = (if v395 { 1.0 } else { 0.0 });
        self.scalar_v1646 = v1646;
        let v1650: f64 = (if v406 { 0.0 } else { v1644 });
        self.scalar_v1650 = v1650;
        let v1685: f64 = (-v1650);
        self.scalar_v1685 = v1685;
        let v1689: f64 = (p.p38 * v1685);
        self.scalar_v1689 = v1689;
        let v1690: f64 = (-p.p30);
        self.scalar_v1690 = v1690;
        let v1691: f64 = (-p.p37);
        self.scalar_v1691 = v1691;
        let v1692: f64 = (v1690 + v1691);
        self.scalar_v1692 = v1692;
        let v1698: f64 = (-p.p32);
        self.scalar_v1698 = v1698;
        let v1703: f64 = (-p.p34);
        self.scalar_v1703 = v1703;
        let v1708: f64 = (-p.p36);
        self.scalar_v1708 = v1708;
        let v1709: f64 = (v1708 - p.p37);
        self.scalar_v1709 = v1709;
        let v1784: f64 = (-v470);
        self.scalar_v1784 = v1784;
        let v1790: f64 = (-p.p24);
        self.scalar_v1790 = v1790;
        let v1848: f64 = (-p.p26);
        self.scalar_v1848 = v1848;
        let v1904: f64 = (-p.p27);
        self.scalar_v1904 = v1904;
        let v1905: f64 = (-p.p23);
        self.scalar_v1905 = v1905;
        let v1927: f64 = (-p.p56);
        self.scalar_v1927 = v1927;
        let v1928: f64 = (-1.0 / p.p55);
        self.scalar_v1928 = v1928;
        let v1929: f64 = (1.0 / p.p55);
        self.scalar_v1929 = v1929;
        let v1930: f64 = (if v528 { v1928 } else { 0.0 });
        self.scalar_v1930 = v1930;
        let v1931: f64 = (if v528 { v1929 } else { 0.0 });
        self.scalar_v1931 = v1931;
        let v1932: f64 = (1.0 / p.p47);
        self.scalar_v1932 = v1932;
        let v1933: f64 = (-1.0 / p.p47);
        self.scalar_v1933 = v1933;
        let v1934: f64 = (if v530 { v1932 } else { 0.0 });
        self.scalar_v1934 = v1934;
        let v1935: f64 = (if v530 { v1933 } else { 0.0 });
        self.scalar_v1935 = v1935;
        let v1936: f64 = (1.0 / p.p45);
        self.scalar_v1936 = v1936;
        let v1937: f64 = (-1.0 / p.p45);
        self.scalar_v1937 = v1937;
        let v1938: f64 = (if v532 { v1936 } else { 0.0 });
        self.scalar_v1938 = v1938;
        let v1939: f64 = (if v532 { v1937 } else { 0.0 });
        self.scalar_v1939 = v1939;
        let v1940: f64 = (if v534 { p.p42 } else { 0.0 });
        self.scalar_v1940 = v1940;
        let v1965: f64 = (if v549 { 1.0 } else { 0.0 });
        self.scalar_v1965 = v1965;
        let v1972: f64 = (1.0 / p.p57);
        self.scalar_v1972 = v1972;
        let v1973: f64 = (if v573 { v1972 } else { 0.0 });
        self.scalar_v1973 = v1973;
        let v1974: f64 = (if v706 { 1e-12 } else { 0.0 });
        self.scalar_v1974 = v1974;
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
