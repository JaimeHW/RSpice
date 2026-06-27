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
    pub p103: f64,
    pub p104: f64,
    pub p105: f64,
    pub p106: f64,
    pub p107: f64,
    pub p108: f64,
    pub p109: f64,
    pub p110: f64,
    pub p111: f64,
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
            params.p0 = 210.0;
            params.p1 = 1e-16;
            params.p2 = 0.0;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 1000000.0;
            params.p6 = 1000000.0;
            params.p7 = 0.0;
            params.p8 = 2.0;
            params.p9 = 1000000.0;
            params.p10 = if (params.p0 <= 200.0) { 1.0 } else { 0.0 };
            validate_parameter("fiqf", params.p10, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p11 = 1000000.0;
            params.p12 = 1000000.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 1e-18;
            params.p16 = 1.0;
            params.p17 = 0.0;
            params.p18 = 2.0;
            params.p19 = if (params.p0 <= 200.0) { 0.0 } else { 1e-16 };
            validate_parameter("ibcs", params.p19, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p20 = 1.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 2.5;
            params.p25 = 1000000.0;
            params.p26 = 0.0;
            params.p27 = 0.656;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 1.0;
            params.p32 = 0.0;
            params.p33 = 1.0;
            params.p34 = 1e-20;
            params.p35 = 0.9;
            params.p36 = 0.5;
            params.p37 = 2.5;
            params.p38 = 0.9;
            params.p39 = 0.5;
            params.p40 = 2.5;
            params.p41 = 1e-20;
            params.p42 = 0.7;
            params.p43 = 0.333;
            params.p44 = 100.0;
            params.p45 = 1e-20;
            params.p46 = 0.7;
            params.p47 = 0.333;
            params.p48 = 100.0;
            params.p49 = 1.0;
            params.p50 = 1e-20;
            params.p51 = 0.3;
            params.p52 = 0.3;
            params.p53 = 100.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 1.0;
            params.p59 = 0.0;
            params.p60 = 0.1;
            params.p61 = 150.0;
            params.p62 = 0.5;
            params.p63 = 100.0;
            params.p64 = 0.1;
            params.p65 = 0.0;
            params.p66 = 0.001;
            params.p67 = 2.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.167;
            params.p72 = 0.333;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 2.0;
            params.p76 = 1.2;
            params.p77 = 1.17;
            params.p78 = 1.17;
            params.p79 = 1.17;
            params.p80 = -0.000102377;
            params.p81 = 3.0;
            params.p82 = 3.5;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = if (params.p0 <= 200.0) { 1.0 } else { 0.0 };
            validate_parameter("flteft", params.p96, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p97 = -1.0;
            params.p98 = 0.0;
            params.p99 = 0.0;
            params.p100 = 0.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 27.0;
            params.p109 = 0.0;
            params.p110 = 1.0;
            params.p111 = 0.001;
            validate_parameter("minr", params.p111, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
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
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 112]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 9]>,
    pub(crate) ddt_state_previous: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
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
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: bool,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: bool,
    pub(crate) scalar_v65: bool,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: bool,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: bool,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v248: bool,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v261: bool,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v269: bool,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v276: bool,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v288: bool,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v383: bool,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v391: bool,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: bool,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: bool,
    pub(crate) scalar_v431: bool,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v603: bool,
    pub(crate) scalar_v609: bool,
    pub(crate) scalar_v619: bool,
    pub(crate) scalar_v626: bool,
    pub(crate) scalar_v632: bool,
    pub(crate) scalar_v641: bool,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v652: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v654: f64,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v682: f64,
    pub(crate) scalar_v683: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v731: bool,
    pub(crate) scalar_v737: bool,
    pub(crate) scalar_v747: f64,
    pub(crate) scalar_v769: f64,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v776: f64,
    pub(crate) scalar_v777: bool,
    pub(crate) scalar_v781: f64,
    pub(crate) scalar_v787: f64,
    pub(crate) scalar_v788: f64,
    pub(crate) scalar_v789: f64,
    pub(crate) scalar_v859: f64,
    pub(crate) scalar_v863: f64,
    pub(crate) scalar_v907: bool,
    pub(crate) scalar_v952: f64,
    pub(crate) scalar_v953: bool,
    pub(crate) scalar_v957: f64,
    pub(crate) scalar_v961: f64,
    pub(crate) scalar_v962: f64,
    pub(crate) scalar_v963: f64,
    pub(crate) scalar_v1030: f64,
    pub(crate) scalar_v1034: f64,
    pub(crate) scalar_v1078: bool,
    pub(crate) scalar_v1466: f64,
    pub(crate) scalar_v1476: f64,
    pub(crate) scalar_v1479: f64,
    pub(crate) scalar_v1522: f64,
    pub(crate) scalar_v1527: f64,
    pub(crate) scalar_v1570: f64,
    pub(crate) scalar_v1575: f64,
    pub(crate) scalar_v1593: f64,
    pub(crate) scalar_v1596: f64,
    pub(crate) scalar_v1597: bool,
    pub(crate) scalar_v1598: f64,
    pub(crate) scalar_v1599: bool,
    pub(crate) scalar_v1600: f64,
    pub(crate) scalar_v1638: f64,
    pub(crate) scalar_v1652: f64,
    pub(crate) scalar_v1656: f64,
    pub(crate) scalar_v1661: f64,
    pub(crate) scalar_v1662: bool,
    pub(crate) scalar_v1669: bool,
    pub(crate) scalar_v1671: f64,
    pub(crate) scalar_v1672: f64,
    pub(crate) scalar_v1685: f64,
    pub(crate) scalar_v1698: bool,
    pub(crate) scalar_v1716: bool,
    pub(crate) scalar_v1730: f64,
    pub(crate) scalar_v1752: f64,
    pub(crate) scalar_v1753: bool,
    pub(crate) scalar_v1754: bool,
    pub(crate) scalar_v1760: bool,
    pub(crate) scalar_v1766: bool,
    pub(crate) scalar_v1768: f64,
    pub(crate) scalar_v1773: bool,
    pub(crate) scalar_v1774: bool,
    pub(crate) scalar_v1775: bool,
    pub(crate) scalar_v1776: bool,
    pub(crate) scalar_v1777: f64,
    pub(crate) scalar_v1778: bool,
    pub(crate) scalar_v1779: bool,
    pub(crate) scalar_v1908: f64,
    pub(crate) scalar_v1912: f64,
    pub(crate) scalar_v1913: f64,
    pub(crate) scalar_v1914: f64,
    pub(crate) scalar_v1919: f64,
    pub(crate) scalar_v1926: f64,
    pub(crate) scalar_v1930: f64,
    pub(crate) scalar_v1932: bool,
    pub(crate) scalar_v1933: f64,
    pub(crate) scalar_v1951: bool,
    pub(crate) scalar_v1953: bool,
    pub(crate) scalar_v1954: f64,
    pub(crate) scalar_v1972: bool,
    pub(crate) scalar_v1975: bool,
    pub(crate) scalar_v1976: f64,
    pub(crate) scalar_v1994: bool,
    pub(crate) scalar_v2180: f64,
    pub(crate) scalar_v2183: f64,
    pub(crate) scalar_v2201: f64,
    pub(crate) scalar_v2223: bool,
    pub(crate) scalar_v2224: f64,
    pub(crate) scalar_v2236: bool,
    pub(crate) scalar_v2238: bool,
    pub(crate) scalar_v2239: f64,
    pub(crate) scalar_v2257: bool,
    pub(crate) scalar_v2259: f64,
    pub(crate) scalar_v2260: bool,
    pub(crate) scalar_v2263: f64,
    pub(crate) scalar_v2267: f64,
    pub(crate) scalar_v2268: f64,
    pub(crate) scalar_v2269: f64,
    pub(crate) scalar_v2330: f64,
    pub(crate) scalar_v2361: bool,
    pub(crate) scalar_v2394: bool,
    pub(crate) scalar_v2395: bool,
    pub(crate) scalar_v2400: f64,
    pub(crate) scalar_v2401: bool,
    pub(crate) scalar_v2402: bool,
    pub(crate) scalar_v2403: bool,
    pub(crate) scalar_v2408: f64,
    pub(crate) scalar_v2416: f64,
    pub(crate) scalar_v2420: bool,
    pub(crate) scalar_v2426: f64,
    pub(crate) scalar_v2428: f64,
    pub(crate) scalar_v2443: bool,
    pub(crate) scalar_v2444: bool,
    pub(crate) scalar_v2445: bool,
    pub(crate) scalar_v2446: bool,
    pub(crate) scalar_v2447: bool,
    pub(crate) scalar_v2448: bool,
    pub(crate) scalar_v2449: bool,
    pub(crate) scalar_v2450: bool,
    pub(crate) scalar_v2456: bool,
    pub(crate) scalar_v2457: f64,
    pub(crate) scalar_v2460: bool,
    pub(crate) scalar_v2461: f64,
    pub(crate) scalar_v2464: bool,
    pub(crate) scalar_v2465: f64,
    pub(crate) scalar_v2466: f64,
    pub(crate) scalar_v2467: bool,
    pub(crate) scalar_v2472: f64,
    pub(crate) scalar_v2473: f64,
    pub(crate) scalar_v2474: f64,
    pub(crate) scalar_v2475: f64,
    pub(crate) scalar_v2816: f64,
    pub(crate) scalar_v5075: f64,
    pub(crate) scalar_v5076: f64,
    pub(crate) scalar_v5079: f64,
    pub(crate) scalar_v5080: f64,
    pub(crate) scalar_v5081: f64,
    pub(crate) scalar_v7908: f64,
    pub(crate) scalar_v7909: f64,
    pub(crate) scalar_v8931: f64,
    pub(crate) scalar_v8942: f64,
    pub(crate) scalar_v8948: f64,
    pub(crate) scalar_v8954: f64,
    pub(crate) scalar_v8960: f64,
    pub(crate) scalar_v8981: f64,
    pub(crate) scalar_v8987: f64,
    pub(crate) scalar_v8993: f64,
    pub(crate) scalar_v8999: f64,
    pub(crate) scalar_v9005: f64,
    pub(crate) scalar_v9016: f64,
    pub(crate) scalar_v9022: f64,
    pub(crate) scalar_v9023: f64,
    pub(crate) scalar_v9024: f64,
    pub(crate) scalar_v9025: f64,
    pub(crate) scalar_v9050: f64,
    pub(crate) scalar_v9051: f64,
    pub(crate) scalar_v9052: f64,
    pub(crate) scalar_v9053: f64,
    pub(crate) scalar_v9072: f64,
    pub(crate) scratch: Option<Box<GenericScratch<386, 10, 4>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<386, 10, 4>>>,
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
            scalar_v0: self.scalar_v0,
            scalar_v22: self.scalar_v22,
            scalar_v24: self.scalar_v24,
            scalar_v27: self.scalar_v27,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
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
            scalar_v44: self.scalar_v44,
            scalar_v45: self.scalar_v45,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
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
            scalar_v120: self.scalar_v120,
            scalar_v126: self.scalar_v126,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v152: self.scalar_v152,
            scalar_v158: self.scalar_v158,
            scalar_v161: self.scalar_v161,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v170: self.scalar_v170,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
            scalar_v199: self.scalar_v199,
            scalar_v205: self.scalar_v205,
            scalar_v211: self.scalar_v211,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
            scalar_v220: self.scalar_v220,
            scalar_v225: self.scalar_v225,
            scalar_v226: self.scalar_v226,
            scalar_v230: self.scalar_v230,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v248: self.scalar_v248,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v255: self.scalar_v255,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v269: self.scalar_v269,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v288: self.scalar_v288,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v350: self.scalar_v350,
            scalar_v351: self.scalar_v351,
            scalar_v357: self.scalar_v357,
            scalar_v363: self.scalar_v363,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v372: self.scalar_v372,
            scalar_v374: self.scalar_v374,
            scalar_v381: self.scalar_v381,
            scalar_v383: self.scalar_v383,
            scalar_v384: self.scalar_v384,
            scalar_v385: self.scalar_v385,
            scalar_v391: self.scalar_v391,
            scalar_v392: self.scalar_v392,
            scalar_v396: self.scalar_v396,
            scalar_v398: self.scalar_v398,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v423: self.scalar_v423,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v499: self.scalar_v499,
            scalar_v500: self.scalar_v500,
            scalar_v501: self.scalar_v501,
            scalar_v502: self.scalar_v502,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v547: self.scalar_v547,
            scalar_v603: self.scalar_v603,
            scalar_v609: self.scalar_v609,
            scalar_v619: self.scalar_v619,
            scalar_v626: self.scalar_v626,
            scalar_v632: self.scalar_v632,
            scalar_v641: self.scalar_v641,
            scalar_v648: self.scalar_v648,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v652: self.scalar_v652,
            scalar_v653: self.scalar_v653,
            scalar_v654: self.scalar_v654,
            scalar_v655: self.scalar_v655,
            scalar_v678: self.scalar_v678,
            scalar_v679: self.scalar_v679,
            scalar_v680: self.scalar_v680,
            scalar_v681: self.scalar_v681,
            scalar_v682: self.scalar_v682,
            scalar_v683: self.scalar_v683,
            scalar_v684: self.scalar_v684,
            scalar_v685: self.scalar_v685,
            scalar_v731: self.scalar_v731,
            scalar_v737: self.scalar_v737,
            scalar_v747: self.scalar_v747,
            scalar_v769: self.scalar_v769,
            scalar_v773: self.scalar_v773,
            scalar_v776: self.scalar_v776,
            scalar_v777: self.scalar_v777,
            scalar_v781: self.scalar_v781,
            scalar_v787: self.scalar_v787,
            scalar_v788: self.scalar_v788,
            scalar_v789: self.scalar_v789,
            scalar_v859: self.scalar_v859,
            scalar_v863: self.scalar_v863,
            scalar_v907: self.scalar_v907,
            scalar_v952: self.scalar_v952,
            scalar_v953: self.scalar_v953,
            scalar_v957: self.scalar_v957,
            scalar_v961: self.scalar_v961,
            scalar_v962: self.scalar_v962,
            scalar_v963: self.scalar_v963,
            scalar_v1030: self.scalar_v1030,
            scalar_v1034: self.scalar_v1034,
            scalar_v1078: self.scalar_v1078,
            scalar_v1466: self.scalar_v1466,
            scalar_v1476: self.scalar_v1476,
            scalar_v1479: self.scalar_v1479,
            scalar_v1522: self.scalar_v1522,
            scalar_v1527: self.scalar_v1527,
            scalar_v1570: self.scalar_v1570,
            scalar_v1575: self.scalar_v1575,
            scalar_v1593: self.scalar_v1593,
            scalar_v1596: self.scalar_v1596,
            scalar_v1597: self.scalar_v1597,
            scalar_v1598: self.scalar_v1598,
            scalar_v1599: self.scalar_v1599,
            scalar_v1600: self.scalar_v1600,
            scalar_v1638: self.scalar_v1638,
            scalar_v1652: self.scalar_v1652,
            scalar_v1656: self.scalar_v1656,
            scalar_v1661: self.scalar_v1661,
            scalar_v1662: self.scalar_v1662,
            scalar_v1669: self.scalar_v1669,
            scalar_v1671: self.scalar_v1671,
            scalar_v1672: self.scalar_v1672,
            scalar_v1685: self.scalar_v1685,
            scalar_v1698: self.scalar_v1698,
            scalar_v1716: self.scalar_v1716,
            scalar_v1730: self.scalar_v1730,
            scalar_v1752: self.scalar_v1752,
            scalar_v1753: self.scalar_v1753,
            scalar_v1754: self.scalar_v1754,
            scalar_v1760: self.scalar_v1760,
            scalar_v1766: self.scalar_v1766,
            scalar_v1768: self.scalar_v1768,
            scalar_v1773: self.scalar_v1773,
            scalar_v1774: self.scalar_v1774,
            scalar_v1775: self.scalar_v1775,
            scalar_v1776: self.scalar_v1776,
            scalar_v1777: self.scalar_v1777,
            scalar_v1778: self.scalar_v1778,
            scalar_v1779: self.scalar_v1779,
            scalar_v1908: self.scalar_v1908,
            scalar_v1912: self.scalar_v1912,
            scalar_v1913: self.scalar_v1913,
            scalar_v1914: self.scalar_v1914,
            scalar_v1919: self.scalar_v1919,
            scalar_v1926: self.scalar_v1926,
            scalar_v1930: self.scalar_v1930,
            scalar_v1932: self.scalar_v1932,
            scalar_v1933: self.scalar_v1933,
            scalar_v1951: self.scalar_v1951,
            scalar_v1953: self.scalar_v1953,
            scalar_v1954: self.scalar_v1954,
            scalar_v1972: self.scalar_v1972,
            scalar_v1975: self.scalar_v1975,
            scalar_v1976: self.scalar_v1976,
            scalar_v1994: self.scalar_v1994,
            scalar_v2180: self.scalar_v2180,
            scalar_v2183: self.scalar_v2183,
            scalar_v2201: self.scalar_v2201,
            scalar_v2223: self.scalar_v2223,
            scalar_v2224: self.scalar_v2224,
            scalar_v2236: self.scalar_v2236,
            scalar_v2238: self.scalar_v2238,
            scalar_v2239: self.scalar_v2239,
            scalar_v2257: self.scalar_v2257,
            scalar_v2259: self.scalar_v2259,
            scalar_v2260: self.scalar_v2260,
            scalar_v2263: self.scalar_v2263,
            scalar_v2267: self.scalar_v2267,
            scalar_v2268: self.scalar_v2268,
            scalar_v2269: self.scalar_v2269,
            scalar_v2330: self.scalar_v2330,
            scalar_v2361: self.scalar_v2361,
            scalar_v2394: self.scalar_v2394,
            scalar_v2395: self.scalar_v2395,
            scalar_v2400: self.scalar_v2400,
            scalar_v2401: self.scalar_v2401,
            scalar_v2402: self.scalar_v2402,
            scalar_v2403: self.scalar_v2403,
            scalar_v2408: self.scalar_v2408,
            scalar_v2416: self.scalar_v2416,
            scalar_v2420: self.scalar_v2420,
            scalar_v2426: self.scalar_v2426,
            scalar_v2428: self.scalar_v2428,
            scalar_v2443: self.scalar_v2443,
            scalar_v2444: self.scalar_v2444,
            scalar_v2445: self.scalar_v2445,
            scalar_v2446: self.scalar_v2446,
            scalar_v2447: self.scalar_v2447,
            scalar_v2448: self.scalar_v2448,
            scalar_v2449: self.scalar_v2449,
            scalar_v2450: self.scalar_v2450,
            scalar_v2456: self.scalar_v2456,
            scalar_v2457: self.scalar_v2457,
            scalar_v2460: self.scalar_v2460,
            scalar_v2461: self.scalar_v2461,
            scalar_v2464: self.scalar_v2464,
            scalar_v2465: self.scalar_v2465,
            scalar_v2466: self.scalar_v2466,
            scalar_v2467: self.scalar_v2467,
            scalar_v2472: self.scalar_v2472,
            scalar_v2473: self.scalar_v2473,
            scalar_v2474: self.scalar_v2474,
            scalar_v2475: self.scalar_v2475,
            scalar_v2816: self.scalar_v2816,
            scalar_v5075: self.scalar_v5075,
            scalar_v5076: self.scalar_v5076,
            scalar_v5079: self.scalar_v5079,
            scalar_v5080: self.scalar_v5080,
            scalar_v5081: self.scalar_v5081,
            scalar_v7908: self.scalar_v7908,
            scalar_v7909: self.scalar_v7909,
            scalar_v8931: self.scalar_v8931,
            scalar_v8942: self.scalar_v8942,
            scalar_v8948: self.scalar_v8948,
            scalar_v8954: self.scalar_v8954,
            scalar_v8960: self.scalar_v8960,
            scalar_v8981: self.scalar_v8981,
            scalar_v8987: self.scalar_v8987,
            scalar_v8993: self.scalar_v8993,
            scalar_v8999: self.scalar_v8999,
            scalar_v9005: self.scalar_v9005,
            scalar_v9016: self.scalar_v9016,
            scalar_v9022: self.scalar_v9022,
            scalar_v9023: self.scalar_v9023,
            scalar_v9024: self.scalar_v9024,
            scalar_v9025: self.scalar_v9025,
            scalar_v9050: self.scalar_v9050,
            scalar_v9051: self.scalar_v9051,
            scalar_v9052: self.scalar_v9052,
            scalar_v9053: self.scalar_v9053,
            scalar_v9072: self.scalar_v9072,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 5;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 5] = ["ci", "bi", "ei", "nd_qf_nqs", "nd_itf_nqs"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 112;
    pub const VARIABLE_COUNT: usize = 386;
    pub const DDT_STATE_COUNT: usize = 9;
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
            scalar_v0: 0.0,
            scalar_v22: 0.0,
            scalar_v24: 0.0,
            scalar_v27: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
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
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v61: 0.0,
            scalar_v62: false,
            scalar_v63: 0.0,
            scalar_v64: false,
            scalar_v65: false,
            scalar_v66: 0.0,
            scalar_v67: false,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v120: 0.0,
            scalar_v126: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v152: 0.0,
            scalar_v158: 0.0,
            scalar_v161: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v170: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v199: 0.0,
            scalar_v205: 0.0,
            scalar_v211: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v220: 0.0,
            scalar_v225: 0.0,
            scalar_v226: 0.0,
            scalar_v230: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v240: 0.0,
            scalar_v241: false,
            scalar_v242: 0.0,
            scalar_v248: false,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v255: 0.0,
            scalar_v260: 0.0,
            scalar_v261: false,
            scalar_v262: 0.0,
            scalar_v269: false,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v276: false,
            scalar_v277: 0.0,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v288: false,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v350: 0.0,
            scalar_v351: 0.0,
            scalar_v357: 0.0,
            scalar_v363: 0.0,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v372: 0.0,
            scalar_v374: 0.0,
            scalar_v381: 0.0,
            scalar_v383: false,
            scalar_v384: 0.0,
            scalar_v385: 0.0,
            scalar_v391: false,
            scalar_v392: 0.0,
            scalar_v396: 0.0,
            scalar_v398: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v423: 0.0,
            scalar_v427: 0.0,
            scalar_v428: false,
            scalar_v429: 0.0,
            scalar_v430: false,
            scalar_v431: false,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v499: 0.0,
            scalar_v500: 0.0,
            scalar_v501: 0.0,
            scalar_v502: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v544: 0.0,
            scalar_v545: 0.0,
            scalar_v546: 0.0,
            scalar_v547: 0.0,
            scalar_v603: false,
            scalar_v609: false,
            scalar_v619: false,
            scalar_v626: false,
            scalar_v632: false,
            scalar_v641: false,
            scalar_v648: 0.0,
            scalar_v649: 0.0,
            scalar_v650: 0.0,
            scalar_v651: 0.0,
            scalar_v652: 0.0,
            scalar_v653: 0.0,
            scalar_v654: 0.0,
            scalar_v655: 0.0,
            scalar_v678: 0.0,
            scalar_v679: 0.0,
            scalar_v680: 0.0,
            scalar_v681: 0.0,
            scalar_v682: 0.0,
            scalar_v683: 0.0,
            scalar_v684: 0.0,
            scalar_v685: 0.0,
            scalar_v731: false,
            scalar_v737: false,
            scalar_v747: 0.0,
            scalar_v769: 0.0,
            scalar_v773: 0.0,
            scalar_v776: 0.0,
            scalar_v777: false,
            scalar_v781: 0.0,
            scalar_v787: 0.0,
            scalar_v788: 0.0,
            scalar_v789: 0.0,
            scalar_v859: 0.0,
            scalar_v863: 0.0,
            scalar_v907: false,
            scalar_v952: 0.0,
            scalar_v953: false,
            scalar_v957: 0.0,
            scalar_v961: 0.0,
            scalar_v962: 0.0,
            scalar_v963: 0.0,
            scalar_v1030: 0.0,
            scalar_v1034: 0.0,
            scalar_v1078: false,
            scalar_v1466: 0.0,
            scalar_v1476: 0.0,
            scalar_v1479: 0.0,
            scalar_v1522: 0.0,
            scalar_v1527: 0.0,
            scalar_v1570: 0.0,
            scalar_v1575: 0.0,
            scalar_v1593: 0.0,
            scalar_v1596: 0.0,
            scalar_v1597: false,
            scalar_v1598: 0.0,
            scalar_v1599: false,
            scalar_v1600: 0.0,
            scalar_v1638: 0.0,
            scalar_v1652: 0.0,
            scalar_v1656: 0.0,
            scalar_v1661: 0.0,
            scalar_v1662: false,
            scalar_v1669: false,
            scalar_v1671: 0.0,
            scalar_v1672: 0.0,
            scalar_v1685: 0.0,
            scalar_v1698: false,
            scalar_v1716: false,
            scalar_v1730: 0.0,
            scalar_v1752: 0.0,
            scalar_v1753: false,
            scalar_v1754: false,
            scalar_v1760: false,
            scalar_v1766: false,
            scalar_v1768: 0.0,
            scalar_v1773: false,
            scalar_v1774: false,
            scalar_v1775: false,
            scalar_v1776: false,
            scalar_v1777: 0.0,
            scalar_v1778: false,
            scalar_v1779: false,
            scalar_v1908: 0.0,
            scalar_v1912: 0.0,
            scalar_v1913: 0.0,
            scalar_v1914: 0.0,
            scalar_v1919: 0.0,
            scalar_v1926: 0.0,
            scalar_v1930: 0.0,
            scalar_v1932: false,
            scalar_v1933: 0.0,
            scalar_v1951: false,
            scalar_v1953: false,
            scalar_v1954: 0.0,
            scalar_v1972: false,
            scalar_v1975: false,
            scalar_v1976: 0.0,
            scalar_v1994: false,
            scalar_v2180: 0.0,
            scalar_v2183: 0.0,
            scalar_v2201: 0.0,
            scalar_v2223: false,
            scalar_v2224: 0.0,
            scalar_v2236: false,
            scalar_v2238: false,
            scalar_v2239: 0.0,
            scalar_v2257: false,
            scalar_v2259: 0.0,
            scalar_v2260: false,
            scalar_v2263: 0.0,
            scalar_v2267: 0.0,
            scalar_v2268: 0.0,
            scalar_v2269: 0.0,
            scalar_v2330: 0.0,
            scalar_v2361: false,
            scalar_v2394: false,
            scalar_v2395: false,
            scalar_v2400: 0.0,
            scalar_v2401: false,
            scalar_v2402: false,
            scalar_v2403: false,
            scalar_v2408: 0.0,
            scalar_v2416: 0.0,
            scalar_v2420: false,
            scalar_v2426: 0.0,
            scalar_v2428: 0.0,
            scalar_v2443: false,
            scalar_v2444: false,
            scalar_v2445: false,
            scalar_v2446: false,
            scalar_v2447: false,
            scalar_v2448: false,
            scalar_v2449: false,
            scalar_v2450: false,
            scalar_v2456: false,
            scalar_v2457: 0.0,
            scalar_v2460: false,
            scalar_v2461: 0.0,
            scalar_v2464: false,
            scalar_v2465: 0.0,
            scalar_v2466: 0.0,
            scalar_v2467: false,
            scalar_v2472: 0.0,
            scalar_v2473: 0.0,
            scalar_v2474: 0.0,
            scalar_v2475: 0.0,
            scalar_v2816: 0.0,
            scalar_v5075: 0.0,
            scalar_v5076: 0.0,
            scalar_v5079: 0.0,
            scalar_v5080: 0.0,
            scalar_v5081: 0.0,
            scalar_v7908: 0.0,
            scalar_v7909: 0.0,
            scalar_v8931: 0.0,
            scalar_v8942: 0.0,
            scalar_v8948: 0.0,
            scalar_v8954: 0.0,
            scalar_v8960: 0.0,
            scalar_v8981: 0.0,
            scalar_v8987: 0.0,
            scalar_v8993: 0.0,
            scalar_v8999: 0.0,
            scalar_v9005: 0.0,
            scalar_v9016: 0.0,
            scalar_v9022: 0.0,
            scalar_v9023: 0.0,
            scalar_v9024: 0.0,
            scalar_v9025: 0.0,
            scalar_v9050: 0.0,
            scalar_v9051: 0.0,
            scalar_v9052: 0.0,
            scalar_v9053: 0.0,
            scalar_v9072: 0.0,
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
            scalar_v0,
            scalar_v22,
            scalar_v24,
            scalar_v27,
            scalar_v29,
            scalar_v30,
            scalar_v31,
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
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
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
            scalar_v120,
            scalar_v126,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v152,
            scalar_v158,
            scalar_v161,
            scalar_v167,
            scalar_v168,
            scalar_v170,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v199,
            scalar_v205,
            scalar_v211,
            scalar_v217,
            scalar_v218,
            scalar_v220,
            scalar_v225,
            scalar_v226,
            scalar_v230,
            scalar_v235,
            scalar_v236,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v248,
            scalar_v251,
            scalar_v252,
            scalar_v255,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v269,
            scalar_v271,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v282,
            scalar_v283,
            scalar_v288,
            scalar_v291,
            scalar_v292,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v319,
            scalar_v320,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v350,
            scalar_v351,
            scalar_v357,
            scalar_v363,
            scalar_v367,
            scalar_v368,
            scalar_v372,
            scalar_v374,
            scalar_v381,
            scalar_v383,
            scalar_v384,
            scalar_v385,
            scalar_v391,
            scalar_v392,
            scalar_v396,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v414,
            scalar_v418,
            scalar_v419,
            scalar_v423,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v603,
            scalar_v609,
            scalar_v619,
            scalar_v626,
            scalar_v632,
            scalar_v641,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v654,
            scalar_v655,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v685,
            scalar_v731,
            scalar_v737,
            scalar_v747,
            scalar_v769,
            scalar_v773,
            scalar_v776,
            scalar_v777,
            scalar_v781,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v859,
            scalar_v863,
            scalar_v907,
            scalar_v952,
            scalar_v953,
            scalar_v957,
            scalar_v961,
            scalar_v962,
            scalar_v963,
            scalar_v1030,
            scalar_v1034,
            scalar_v1078,
            scalar_v1466,
            scalar_v1476,
            scalar_v1479,
            scalar_v1522,
            scalar_v1527,
            scalar_v1570,
            scalar_v1575,
            scalar_v1593,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1600,
            scalar_v1638,
            scalar_v1652,
            scalar_v1656,
            scalar_v1661,
            scalar_v1662,
            scalar_v1669,
            scalar_v1671,
            scalar_v1672,
            scalar_v1685,
            scalar_v1698,
            scalar_v1716,
            scalar_v1730,
            scalar_v1752,
            scalar_v1753,
            scalar_v1754,
            scalar_v1760,
            scalar_v1766,
            scalar_v1768,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1779,
            scalar_v1908,
            scalar_v1912,
            scalar_v1913,
            scalar_v1914,
            scalar_v1919,
            scalar_v1926,
            scalar_v1930,
            scalar_v1932,
            scalar_v1933,
            scalar_v1951,
            scalar_v1953,
            scalar_v1954,
            scalar_v1972,
            scalar_v1975,
            scalar_v1976,
            scalar_v1994,
            scalar_v2180,
            scalar_v2183,
            scalar_v2201,
            scalar_v2223,
            scalar_v2224,
            scalar_v2236,
            scalar_v2238,
            scalar_v2239,
            scalar_v2257,
            scalar_v2259,
            scalar_v2260,
            scalar_v2263,
            scalar_v2267,
            scalar_v2268,
            scalar_v2269,
            scalar_v2330,
            scalar_v2361,
            scalar_v2394,
            scalar_v2395,
            scalar_v2400,
            scalar_v2401,
            scalar_v2402,
            scalar_v2403,
            scalar_v2408,
            scalar_v2416,
            scalar_v2420,
            scalar_v2426,
            scalar_v2428,
            scalar_v2443,
            scalar_v2444,
            scalar_v2445,
            scalar_v2446,
            scalar_v2447,
            scalar_v2448,
            scalar_v2449,
            scalar_v2450,
            scalar_v2456,
            scalar_v2457,
            scalar_v2460,
            scalar_v2461,
            scalar_v2464,
            scalar_v2465,
            scalar_v2466,
            scalar_v2467,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v2816,
            scalar_v5075,
            scalar_v5076,
            scalar_v5079,
            scalar_v5080,
            scalar_v5081,
            scalar_v7908,
            scalar_v7909,
            scalar_v8931,
            scalar_v8942,
            scalar_v8948,
            scalar_v8954,
            scalar_v8960,
            scalar_v8981,
            scalar_v8987,
            scalar_v8993,
            scalar_v8999,
            scalar_v9005,
            scalar_v9016,
            scalar_v9022,
            scalar_v9023,
            scalar_v9024,
            scalar_v9025,
            scalar_v9050,
            scalar_v9051,
            scalar_v9052,
            scalar_v9053,
            scalar_v9072,
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
            scalar_v0,
            scalar_v22,
            scalar_v24,
            scalar_v27,
            scalar_v29,
            scalar_v30,
            scalar_v31,
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
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
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
            scalar_v120,
            scalar_v126,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v152,
            scalar_v158,
            scalar_v161,
            scalar_v167,
            scalar_v168,
            scalar_v170,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
            scalar_v199,
            scalar_v205,
            scalar_v211,
            scalar_v217,
            scalar_v218,
            scalar_v220,
            scalar_v225,
            scalar_v226,
            scalar_v230,
            scalar_v235,
            scalar_v236,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v248,
            scalar_v251,
            scalar_v252,
            scalar_v255,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v269,
            scalar_v271,
            scalar_v272,
            scalar_v276,
            scalar_v277,
            scalar_v282,
            scalar_v283,
            scalar_v288,
            scalar_v291,
            scalar_v292,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v319,
            scalar_v320,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v350,
            scalar_v351,
            scalar_v357,
            scalar_v363,
            scalar_v367,
            scalar_v368,
            scalar_v372,
            scalar_v374,
            scalar_v381,
            scalar_v383,
            scalar_v384,
            scalar_v385,
            scalar_v391,
            scalar_v392,
            scalar_v396,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v414,
            scalar_v418,
            scalar_v419,
            scalar_v423,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v464,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v502,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v547,
            scalar_v603,
            scalar_v609,
            scalar_v619,
            scalar_v626,
            scalar_v632,
            scalar_v641,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v654,
            scalar_v655,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v685,
            scalar_v731,
            scalar_v737,
            scalar_v747,
            scalar_v769,
            scalar_v773,
            scalar_v776,
            scalar_v777,
            scalar_v781,
            scalar_v787,
            scalar_v788,
            scalar_v789,
            scalar_v859,
            scalar_v863,
            scalar_v907,
            scalar_v952,
            scalar_v953,
            scalar_v957,
            scalar_v961,
            scalar_v962,
            scalar_v963,
            scalar_v1030,
            scalar_v1034,
            scalar_v1078,
            scalar_v1466,
            scalar_v1476,
            scalar_v1479,
            scalar_v1522,
            scalar_v1527,
            scalar_v1570,
            scalar_v1575,
            scalar_v1593,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1600,
            scalar_v1638,
            scalar_v1652,
            scalar_v1656,
            scalar_v1661,
            scalar_v1662,
            scalar_v1669,
            scalar_v1671,
            scalar_v1672,
            scalar_v1685,
            scalar_v1698,
            scalar_v1716,
            scalar_v1730,
            scalar_v1752,
            scalar_v1753,
            scalar_v1754,
            scalar_v1760,
            scalar_v1766,
            scalar_v1768,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1779,
            scalar_v1908,
            scalar_v1912,
            scalar_v1913,
            scalar_v1914,
            scalar_v1919,
            scalar_v1926,
            scalar_v1930,
            scalar_v1932,
            scalar_v1933,
            scalar_v1951,
            scalar_v1953,
            scalar_v1954,
            scalar_v1972,
            scalar_v1975,
            scalar_v1976,
            scalar_v1994,
            scalar_v2180,
            scalar_v2183,
            scalar_v2201,
            scalar_v2223,
            scalar_v2224,
            scalar_v2236,
            scalar_v2238,
            scalar_v2239,
            scalar_v2257,
            scalar_v2259,
            scalar_v2260,
            scalar_v2263,
            scalar_v2267,
            scalar_v2268,
            scalar_v2269,
            scalar_v2330,
            scalar_v2361,
            scalar_v2394,
            scalar_v2395,
            scalar_v2400,
            scalar_v2401,
            scalar_v2402,
            scalar_v2403,
            scalar_v2408,
            scalar_v2416,
            scalar_v2420,
            scalar_v2426,
            scalar_v2428,
            scalar_v2443,
            scalar_v2444,
            scalar_v2445,
            scalar_v2446,
            scalar_v2447,
            scalar_v2448,
            scalar_v2449,
            scalar_v2450,
            scalar_v2456,
            scalar_v2457,
            scalar_v2460,
            scalar_v2461,
            scalar_v2464,
            scalar_v2465,
            scalar_v2466,
            scalar_v2467,
            scalar_v2472,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v2816,
            scalar_v5075,
            scalar_v5076,
            scalar_v5079,
            scalar_v5080,
            scalar_v5081,
            scalar_v7908,
            scalar_v7909,
            scalar_v8931,
            scalar_v8942,
            scalar_v8948,
            scalar_v8954,
            scalar_v8960,
            scalar_v8981,
            scalar_v8987,
            scalar_v8993,
            scalar_v8999,
            scalar_v9005,
            scalar_v9016,
            scalar_v9022,
            scalar_v9023,
            scalar_v9024,
            scalar_v9025,
            scalar_v9050,
            scalar_v9051,
            scalar_v9052,
            scalar_v9053,
            scalar_v9072,
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "flitm" => { validate_parameter("flitm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "mcr" => { validate_parameter("mcr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "aver" => { validate_parameter("aver", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "rver" => { validate_parameter("rver", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "iqf" => { validate_parameter("iqf", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "fiqf" => { validate_parameter("fiqf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "iqr" => { validate_parameter("iqr", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "iqfh" => { validate_parameter("iqfh", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "tfh" => { validate_parameter("tfh", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "ahq" => { validate_parameter("ahq", value, Some((-0.9, "-0.9")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ibes" => { validate_parameter("ibes", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "mbe" => { validate_parameter("mbe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "ires" => { validate_parameter("ires", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "mre" => { validate_parameter("mre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "ibcs" => { validate_parameter("ibcs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "mbc" => { validate_parameter("mbc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "vr0e" => { validate_parameter("vr0e", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "vr0c" => { validate_parameter("vr0c", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "cje0" => { validate_parameter("cje0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "ze" => { validate_parameter("ze", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "aje" => { validate_parameter("aje", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "vdedc" => { validate_parameter("vdedc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "zedc" => { validate_parameter("zedc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "ajedc" => { validate_parameter("ajedc", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "fbc" => { validate_parameter("fbc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "tbvl" => { validate_parameter("tbvl", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "gte" => { validate_parameter("gte", value, Some((0.0, "0.0")), true, Some((20.0, "20.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "zetact" => { validate_finite_parameter("zetact", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "zetabet" => { validate_finite_parameter("zetabet", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "dvgbe" => { validate_finite_parameter("dvgbe", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "zetavgbe" => { validate_finite_parameter("zetavgbe", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "zetaci" => { validate_finite_parameter("zetaci", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "zetarbi" => { validate_finite_parameter("zetarbi", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "zetarbx" => { validate_finite_parameter("zetarbx", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "zetarcx" => { validate_finite_parameter("zetarcx", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "zetare" => { validate_finite_parameter("zetare", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "zetaiqf" => { validate_finite_parameter("zetaiqf", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "flteft" => { validate_parameter("flteft", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "zetaver" => { validate_finite_parameter("zetaver", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "zetaiqfh" => { validate_finite_parameter("zetaiqfh", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "aliqfh" => { validate_finite_parameter("aliqfh", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "kiqfh" => { validate_finite_parameter("kiqfh", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "zetarth" => { validate_finite_parameter("zetarth", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), true, Some((600.0, "600.0")), false, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hicumL0va'", name)),
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
        let v0: f64 = p.p110;
        self.scalar_v0 = v0;
        let v22: f64 = p.p108;
        self.scalar_v22 = v22;
        let v24: f64 = (p.p108 + 273.15);
        self.scalar_v24 = v24;
        let v27: f64 = (1.3806226e-23 * v24);
        self.scalar_v27 = v27;
        let v29: f64 = (v27 / 1.602176462e-19);
        self.scalar_v29 = v29;
        let v30: f64 = p.p88;
        self.scalar_v30 = v30;
        let v31: f64 = (p.p88 * v24);
        self.scalar_v31 = v31;
        let v33: f64 = p.p76;
        self.scalar_v33 = v33;
        let v34: f64 = p.p77;
        self.scalar_v34 = v34;
        let v35: f64 = (p.p76 + p.p77);
        self.scalar_v35 = v35;
        let v36: f64 = (0.5 * v35);
        self.scalar_v36 = v36;
        let v37: f64 = p.p78;
        self.scalar_v37 = v37;
        let v38: f64 = (p.p76 + p.p78);
        self.scalar_v38 = v38;
        let v39: f64 = (0.5 * v38);
        self.scalar_v39 = v39;
        let v40: f64 = p.p79;
        self.scalar_v40 = v40;
        let v41: f64 = (p.p79 + p.p78);
        self.scalar_v41 = v41;
        let v42: f64 = (0.5 * v41);
        self.scalar_v42 = v42;
        let v44: f64 = p.p80;
        self.scalar_v44 = v44;
        let v45: f64 = (1.602176462e-19 * p.p80);
        self.scalar_v45 = v45;
        let v46: f64 = (v45 / 1.3806226e-23);
        self.scalar_v46 = v46;
        let v47: f64 = (3.0 - v46);
        self.scalar_v47 = v47;
        let v49: f64 = (v47 + 1.0);
        self.scalar_v49 = v49;
        let v50: f64 = p.p87;
        self.scalar_v50 = v50;
        let v51: f64 = (v49 - p.p87);
        self.scalar_v51 = v51;
        let v53: f64 = (v47 - 1.5);
        self.scalar_v53 = v53;
        let v54: f64 = p.p82;
        self.scalar_v54 = v54;
        let v55: f64 = p.p81;
        self.scalar_v55 = v55;
        let v56: f64 = (p.p82 - p.p81);
        self.scalar_v56 = v56;
        let v57: f64 = (v56 - 0.5);
        self.scalar_v57 = v57;
        let v58: f64 = (p.p76 - p.p77);
        self.scalar_v58 = v58;
        let v59: f64 = p.p34;
        self.scalar_v59 = v59;
        let v61: f64 = p.p21;
        self.scalar_v61 = v61;
        let v62: bool = (p.p21 > 0.0);
        self.scalar_v62 = v62;
        let v63: f64 = p.p41;
        self.scalar_v63 = v63;
        let v64: bool = (p.p41 > 0.0);
        self.scalar_v64 = v64;
        let v65: bool = (v62 && v64);
        self.scalar_v65 = v65;
        let v66: f64 = (if v65 { 1.0 } else { 0.0 });
        self.scalar_v66 = v66;
        let v67: bool = (!v65);
        self.scalar_v67 = v67;
        let v68: f64 = (if v67 { 0.0 } else { v66 });
        self.scalar_v68 = v68;
        let v69: f64 = p.p109;
        self.scalar_v69 = v69;
        let v89: f64 = p.p35;
        self.scalar_v89 = v89;
        let v90: f64 = (0.5 * p.p35);
        self.scalar_v90 = v90;
        let v91: f64 = (v90 / v29);
        self.scalar_v91 = v91;
        let v93: f64 = (2.0 * v29);
        self.scalar_v93 = v93;
        let v94: f64 = v91.exp();
        self.scalar_v94 = v94;
        let v95: f64 = (-v91);
        self.scalar_v95 = v95;
        let v96: f64 = v95.exp();
        self.scalar_v96 = v96;
        let v97: f64 = (v94 - v96);
        self.scalar_v97 = v97;
        let v98: f64 = v97.ln();
        self.scalar_v98 = v98;
        let v99: f64 = (v93 * v98);
        self.scalar_v99 = v99;
        let v120: f64 = p.p36;
        self.scalar_v120 = v120;
        let v126: f64 = p.p37;
        self.scalar_v126 = v126;
        let v129: f64 = p.p38;
        self.scalar_v129 = v129;
        let v130: f64 = (0.5 * p.p38);
        self.scalar_v130 = v130;
        let v131: f64 = (v130 / v29);
        self.scalar_v131 = v131;
        let v132: f64 = v131.exp();
        self.scalar_v132 = v132;
        let v133: f64 = (-v131);
        self.scalar_v133 = v133;
        let v134: f64 = v133.exp();
        self.scalar_v134 = v134;
        let v135: f64 = (v132 - v134);
        self.scalar_v135 = v135;
        let v136: f64 = v135.ln();
        self.scalar_v136 = v136;
        let v137: f64 = (v93 * v136);
        self.scalar_v137 = v137;
        let v152: f64 = p.p39;
        self.scalar_v152 = v152;
        let v158: f64 = p.p40;
        self.scalar_v158 = v158;
        let v161: f64 = p.p15;
        self.scalar_v161 = v161;
        let v167: f64 = p.p17;
        self.scalar_v167 = v167;
        let v168: f64 = (0.5 * v47);
        self.scalar_v168 = v168;
        let v170: f64 = (0.5 * v36);
        self.scalar_v170 = v170;
        let v175: f64 = p.p42;
        self.scalar_v175 = v175;
        let v176: f64 = (0.5 * p.p42);
        self.scalar_v176 = v176;
        let v177: f64 = (v176 / v29);
        self.scalar_v177 = v177;
        let v178: f64 = v177.exp();
        self.scalar_v178 = v178;
        let v179: f64 = (-v177);
        self.scalar_v179 = v179;
        let v180: f64 = v179.exp();
        self.scalar_v180 = v180;
        let v181: f64 = (v178 - v180);
        self.scalar_v181 = v181;
        let v182: f64 = v181.ln();
        self.scalar_v182 = v182;
        let v183: f64 = (v93 * v182);
        self.scalar_v183 = v183;
        let v199: f64 = p.p43;
        self.scalar_v199 = v199;
        let v205: f64 = p.p19;
        self.scalar_v205 = v205;
        let v211: f64 = p.p1;
        self.scalar_v211 = v211;
        let v217: f64 = p.p9;
        self.scalar_v217 = v217;
        let v218: f64 = p.p95;
        self.scalar_v218 = v218;
        let v220: f64 = p.p83;
        self.scalar_v220 = v220;
        let v225: f64 = p.p62;
        self.scalar_v225 = v225;
        let v226: f64 = (p.p87 - v31);
        self.scalar_v226 = v226;
        let v230: f64 = p.p61;
        self.scalar_v230 = v230;
        let v235: f64 = p.p64;
        self.scalar_v235 = v235;
        let v236: f64 = p.p89;
        self.scalar_v236 = v236;
        let v240: f64 = p.p65;
        self.scalar_v240 = v240;
        let v241: bool = (p.p65 > 0.0);
        self.scalar_v241 = v241;
        let v242: f64 = p.p90;
        self.scalar_v242 = v242;
        let v248: bool = (!v241);
        self.scalar_v248 = v248;
        let v251: f64 = p.p54;
        self.scalar_v251 = v251;
        let v252: f64 = p.p85;
        self.scalar_v252 = v252;
        let v255: f64 = p.p86;
        self.scalar_v255 = v255;
        let v260: f64 = p.p96;
        self.scalar_v260 = v260;
        let v261: bool = (p.p96 == 1.0);
        self.scalar_v261 = v261;
        let v262: f64 = p.p57;
        self.scalar_v262 = v262;
        let v269: bool = (!v261);
        self.scalar_v269 = v269;
        let v271: f64 = p.p59;
        self.scalar_v271 = v271;
        let v272: f64 = (p.p87 - 1.0);
        self.scalar_v272 = v272;
        let v276: bool = (v68 == 1.0);
        self.scalar_v276 = v276;
        let v277: f64 = p.p99;
        self.scalar_v277 = v277;
        let v282: f64 = p.p22;
        self.scalar_v282 = v282;
        let v283: f64 = p.p100;
        self.scalar_v283 = v283;
        let v288: bool = (!v276);
        self.scalar_v288 = v288;
        let v291: f64 = p.p23;
        self.scalar_v291 = v291;
        let v292: f64 = p.p91;
        self.scalar_v292 = v292;
        let v296: f64 = p.p46;
        self.scalar_v296 = v296;
        let v297: f64 = (0.5 * p.p46);
        self.scalar_v297 = v297;
        let v298: f64 = (v297 / v29);
        self.scalar_v298 = v298;
        let v299: f64 = v298.exp();
        self.scalar_v299 = v299;
        let v300: f64 = (-v298);
        self.scalar_v300 = v300;
        let v301: f64 = v300.exp();
        self.scalar_v301 = v301;
        let v302: f64 = (v299 - v301);
        self.scalar_v302 = v302;
        let v303: f64 = v302.ln();
        self.scalar_v303 = v303;
        let v304: f64 = (v93 * v303);
        self.scalar_v304 = v304;
        let v319: f64 = p.p45;
        self.scalar_v319 = v319;
        let v320: f64 = p.p47;
        self.scalar_v320 = v320;
        let v326: f64 = p.p51;
        self.scalar_v326 = v326;
        let v327: f64 = (0.5 * p.p51);
        self.scalar_v327 = v327;
        let v328: f64 = (v327 / v29);
        self.scalar_v328 = v328;
        let v329: f64 = v328.exp();
        self.scalar_v329 = v329;
        let v330: f64 = (-v328);
        self.scalar_v330 = v330;
        let v331: f64 = v330.exp();
        self.scalar_v331 = v331;
        let v332: f64 = (v329 - v331);
        self.scalar_v332 = v332;
        let v333: f64 = v332.ln();
        self.scalar_v333 = v333;
        let v334: f64 = (v93 * v333);
        self.scalar_v334 = v334;
        let v350: f64 = p.p50;
        self.scalar_v350 = v350;
        let v351: f64 = p.p52;
        self.scalar_v351 = v351;
        let v357: f64 = p.p32;
        self.scalar_v357 = v357;
        let v363: f64 = p.p30;
        self.scalar_v363 = v363;
        let v367: f64 = p.p7;
        self.scalar_v367 = v367;
        let v368: f64 = p.p97;
        self.scalar_v368 = v368;
        let v372: f64 = p.p6;
        self.scalar_v372 = v372;
        let v374: f64 = p.p84;
        self.scalar_v374 = v374;
        let v381: f64 = p.p0;
        self.scalar_v381 = v381;
        let v383: bool = (p.p0 <= 200.0);
        self.scalar_v383 = v383;
        let v384: f64 = p.p101;
        self.scalar_v384 = v384;
        let v385: f64 = p.p102;
        self.scalar_v385 = v385;
        let v391: bool = (!v383);
        self.scalar_v391 = v391;
        let v392: f64 = p.p98;
        self.scalar_v392 = v392;
        let v396: f64 = p.p12;
        self.scalar_v396 = v396;
        let v398: f64 = p.p13;
        self.scalar_v398 = v398;
        let v402: f64 = p.p14;
        self.scalar_v402 = v402;
        let v403: f64 = p.p29;
        self.scalar_v403 = v403;
        let v404: f64 = p.p93;
        self.scalar_v404 = v404;
        let v408: f64 = p.p26;
        self.scalar_v408 = v408;
        let v409: f64 = p.p92;
        self.scalar_v409 = v409;
        let v413: f64 = p.p28;
        self.scalar_v413 = v413;
        let v414: f64 = p.p94;
        self.scalar_v414 = v414;
        let v418: f64 = p.p104;
        self.scalar_v418 = v418;
        let v419: f64 = p.p105;
        self.scalar_v419 = v419;
        let v423: f64 = p.p106;
        self.scalar_v423 = v423;
        let v427: f64 = p.p103;
        self.scalar_v427 = v427;
        let v428: bool = (p.p103 != 0.0);
        self.scalar_v428 = v428;
        let v429: f64 = p.p111;
        self.scalar_v429 = v429;
        let v430: bool = (p.p104 >= p.p111);
        self.scalar_v430 = v430;
        let v431: bool = (v428 && v430);
        self.scalar_v431 = v431;
        let v457: f64 = (if v431 { v91 } else { v328 });
        self.scalar_v457 = v457;
        let v458: f64 = v457.exp();
        self.scalar_v458 = v458;
        let v459: f64 = (-v457);
        self.scalar_v459 = v459;
        let v460: f64 = v459.exp();
        self.scalar_v460 = v460;
        let v461: f64 = (v458 - v460);
        self.scalar_v461 = v461;
        let v462: f64 = v461.ln();
        self.scalar_v462 = v462;
        let v463: f64 = (v93 * v462);
        self.scalar_v463 = v463;
        let v464: f64 = (if v431 { v463 } else { v334 });
        self.scalar_v464 = v464;
        let v495: f64 = (if v431 { v131 } else { v457 });
        self.scalar_v495 = v495;
        let v496: f64 = v495.exp();
        self.scalar_v496 = v496;
        let v497: f64 = (-v495);
        self.scalar_v497 = v497;
        let v498: f64 = v497.exp();
        self.scalar_v498 = v498;
        let v499: f64 = (v496 - v498);
        self.scalar_v499 = v499;
        let v500: f64 = v499.ln();
        self.scalar_v500 = v500;
        let v501: f64 = (v93 * v500);
        self.scalar_v501 = v501;
        let v502: f64 = (if v431 { v501 } else { v464 });
        self.scalar_v502 = v502;
        let v540: f64 = (if v431 { v177 } else { v495 });
        self.scalar_v540 = v540;
        let v541: f64 = v540.exp();
        self.scalar_v541 = v541;
        let v542: f64 = (-v540);
        self.scalar_v542 = v542;
        let v543: f64 = v542.exp();
        self.scalar_v543 = v543;
        let v544: f64 = (v541 - v543);
        self.scalar_v544 = v544;
        let v545: f64 = v544.ln();
        self.scalar_v545 = v545;
        let v546: f64 = (v93 * v545);
        self.scalar_v546 = v546;
        let v547: f64 = (if v431 { v546 } else { v502 });
        self.scalar_v547 = v547;
        let v603: bool = (v431 && v241);
        self.scalar_v603 = v603;
        let v609: bool = (v431 && v248);
        self.scalar_v609 = v609;
        let v619: bool = (v431 && v261);
        self.scalar_v619 = v619;
        let v626: bool = (v431 && v269);
        self.scalar_v626 = v626;
        let v632: bool = (v431 && v276);
        self.scalar_v632 = v632;
        let v641: bool = (v431 && v288);
        self.scalar_v641 = v641;
        let v648: f64 = (if v431 { v298 } else { v540 });
        self.scalar_v648 = v648;
        let v649: f64 = v648.exp();
        self.scalar_v649 = v649;
        let v650: f64 = (-v648);
        self.scalar_v650 = v650;
        let v651: f64 = v650.exp();
        self.scalar_v651 = v651;
        let v652: f64 = (v649 - v651);
        self.scalar_v652 = v652;
        let v653: f64 = v652.ln();
        self.scalar_v653 = v653;
        let v654: f64 = (v93 * v653);
        self.scalar_v654 = v654;
        let v655: f64 = (if v431 { v654 } else { v547 });
        self.scalar_v655 = v655;
        let v678: f64 = (if v431 { v328 } else { v648 });
        self.scalar_v678 = v678;
        let v679: f64 = v678.exp();
        self.scalar_v679 = v679;
        let v680: f64 = (-v678);
        self.scalar_v680 = v680;
        let v681: f64 = v680.exp();
        self.scalar_v681 = v681;
        let v682: f64 = (v679 - v681);
        self.scalar_v682 = v682;
        let v683: f64 = v682.ln();
        self.scalar_v683 = v683;
        let v684: f64 = (v93 * v683);
        self.scalar_v684 = v684;
        let v685: f64 = (if v431 { v684 } else { v655 });
        self.scalar_v685 = v685;
        let v731: bool = (v431 && v383);
        self.scalar_v731 = v731;
        let v737: bool = (v431 && v391);
        self.scalar_v737 = v737;
        let v747: f64 = (if v431 { p.p14 } else { p.p14 });
        self.scalar_v747 = v747;
        let v769: f64 = p.p49;
        self.scalar_v769 = v769;
        let v773: f64 = (1.0 - p.p49);
        self.scalar_v773 = v773;
        let v776: f64 = p.p44;
        self.scalar_v776 = v776;
        let v777: bool = (p.p44 < 100.0);
        self.scalar_v777 = v777;
        let v781: f64 = (p.p43 / 4.0);
        self.scalar_v781 = v781;
        let v787: f64 = (-0.8754687373538999 / p.p43);
        self.scalar_v787 = v787;
        let v788: f64 = v787.exp();
        self.scalar_v788 = v788;
        let v789: f64 = (1.0 - v788);
        self.scalar_v789 = v789;
        let v859: f64 = (1.0 - p.p43);
        self.scalar_v859 = v859;
        let v863: f64 = (-p.p43);
        self.scalar_v863 = v863;
        let v907: bool = (!v777);
        self.scalar_v907 = v907;
        let v952: f64 = p.p48;
        self.scalar_v952 = v952;
        let v953: bool = (p.p48 < 100.0);
        self.scalar_v953 = v953;
        let v957: f64 = (p.p47 / 4.0);
        self.scalar_v957 = v957;
        let v961: f64 = (-0.8754687373538999 / p.p47);
        self.scalar_v961 = v961;
        let v962: f64 = v961.exp();
        self.scalar_v962 = v962;
        let v963: f64 = (1.0 - v962);
        self.scalar_v963 = v963;
        let v1030: f64 = (1.0 - p.p47);
        self.scalar_v1030 = v1030;
        let v1034: f64 = (-p.p47);
        self.scalar_v1034 = v1034;
        let v1078: bool = (!v953);
        self.scalar_v1078 = v1078;
        let v1466: f64 = p.p67;
        self.scalar_v1466 = v1466;
        let v1476: f64 = p.p63;
        self.scalar_v1476 = v1476;
        let v1479: f64 = p.p66;
        self.scalar_v1479 = v1479;
        let v1522: f64 = (-p.p36);
        self.scalar_v1522 = v1522;
        let v1527: f64 = (1.0 - p.p36);
        self.scalar_v1527 = v1527;
        let v1570: f64 = (-p.p39);
        self.scalar_v1570 = v1570;
        let v1575: f64 = (1.0 - p.p39);
        self.scalar_v1575 = v1575;
        let v1593: f64 = (if v383 { p.p39 } else { 0.0 });
        self.scalar_v1593 = v1593;
        let v1596: f64 = (if v391 { p.p36 } else { v1593 });
        self.scalar_v1596 = v1596;
        let v1597: bool = (p.p7 == 0.0);
        self.scalar_v1597 = v1597;
        let v1598: f64 = (if v1597 { 1.0 } else { 0.0 });
        self.scalar_v1598 = v1598;
        let v1599: bool = (!v1597);
        self.scalar_v1599 = v1599;
        let v1600: f64 = p.p8;
        self.scalar_v1600 = v1600;
        let v1638: f64 = p.p5;
        self.scalar_v1638 = v1638;
        let v1652: f64 = p.p55;
        self.scalar_v1652 = v1652;
        let v1656: f64 = p.p56;
        self.scalar_v1656 = v1656;
        let v1661: f64 = p.p10;
        self.scalar_v1661 = v1661;
        let v1662: bool = (p.p10 == 1.0);
        self.scalar_v1662 = v1662;
        let v1669: bool = (!v1662);
        self.scalar_v1669 = v1669;
        let v1671: f64 = p.p11;
        self.scalar_v1671 = v1671;
        let v1672: f64 = p.p3;
        self.scalar_v1672 = v1672;
        let v1685: f64 = p.p4;
        self.scalar_v1685 = v1685;
        let v1698: bool = (p.p13 != 0.0);
        self.scalar_v1698 = v1698;
        let v1716: bool = (!v1698);
        self.scalar_v1716 = v1716;
        let v1730: f64 = (1.0 + v747);
        self.scalar_v1730 = v1730;
        let v1752: f64 = p.p2;
        self.scalar_v1752 = v1752;
        let v1753: bool = (p.p2 == 0.0);
        self.scalar_v1753 = v1753;
        let v1754: bool = (v1753 && v1698);
        self.scalar_v1754 = v1754;
        let v1760: bool = (v1753 && v1716);
        self.scalar_v1760 = v1760;
        let v1766: bool = (!v1753);
        self.scalar_v1766 = v1766;
        let v1768: f64 = (if v1766 { 0.3333333333333333 } else { 0.0 });
        self.scalar_v1768 = v1768;
        let v1773: bool = (p.p9 == 1000000.0);
        self.scalar_v1773 = v1773;
        let v1774: bool = (p.p12 == 1000000.0);
        self.scalar_v1774 = v1774;
        let v1775: bool = (v1773 && v1774);
        self.scalar_v1775 = v1775;
        let v1776: bool = (v1766 && v1775);
        self.scalar_v1776 = v1776;
        let v1777: f64 = (if v1776 { 0.0 } else { 0.0 });
        self.scalar_v1777 = v1777;
        let v1778: bool = (!v1775);
        self.scalar_v1778 = v1778;
        let v1779: bool = (v1766 && v1778);
        self.scalar_v1779 = v1779;
        let v1908: f64 = p.p60;
        self.scalar_v1908 = v1908;
        let v1912: f64 = (1.0 + p.p60);
        self.scalar_v1912 = v1912;
        let v1913: f64 = v1912.sqrt();
        self.scalar_v1913 = v1913;
        let v1914: f64 = (1.0 + v1913);
        self.scalar_v1914 = v1914;
        let v1919: f64 = p.p58;
        self.scalar_v1919 = v1919;
        let v1926: f64 = (p.p58 + 1.0);
        self.scalar_v1926 = v1926;
        let v1930: f64 = p.p68;
        self.scalar_v1930 = v1930;
        let v1932: bool = (p.p15 > 0.0);
        self.scalar_v1932 = v1932;
        let v1933: f64 = p.p16;
        self.scalar_v1933 = v1933;
        let v1951: bool = (!v1932);
        self.scalar_v1951 = v1951;
        let v1953: bool = (p.p17 > 0.0);
        self.scalar_v1953 = v1953;
        let v1954: f64 = p.p18;
        self.scalar_v1954 = v1954;
        let v1972: bool = (!v1953);
        self.scalar_v1972 = v1972;
        let v1975: bool = (p.p19 > 0.0);
        self.scalar_v1975 = v1975;
        let v1976: f64 = p.p20;
        self.scalar_v1976 = v1976;
        let v1994: bool = (!v1975);
        self.scalar_v1994 = v1994;
        let v2180: f64 = p.p24;
        self.scalar_v2180 = v2180;
        let v2183: f64 = p.p25;
        self.scalar_v2183 = v2183;
        let v2201: f64 = p.p27;
        self.scalar_v2201 = v2201;
        let v2223: bool = (p.p30 > 0.0);
        self.scalar_v2223 = v2223;
        let v2224: f64 = p.p31;
        self.scalar_v2224 = v2224;
        let v2236: bool = (!v2223);
        self.scalar_v2236 = v2236;
        let v2238: bool = (p.p32 > 0.0);
        self.scalar_v2238 = v2238;
        let v2239: f64 = p.p33;
        self.scalar_v2239 = v2239;
        let v2257: bool = (!v2238);
        self.scalar_v2257 = v2257;
        let v2259: f64 = p.p53;
        self.scalar_v2259 = v2259;
        let v2260: bool = (p.p53 < 100.0);
        self.scalar_v2260 = v2260;
        let v2263: f64 = (p.p52 / 4.0);
        self.scalar_v2263 = v2263;
        let v2267: f64 = (-0.8754687373538999 / p.p52);
        self.scalar_v2267 = v2267;
        let v2268: f64 = v2267.exp();
        self.scalar_v2268 = v2268;
        let v2269: f64 = (1.0 - v2268);
        self.scalar_v2269 = v2269;
        let v2330: f64 = (1.0 - p.p52);
        self.scalar_v2330 = v2330;
        let v2361: bool = (!v2260);
        self.scalar_v2361 = v2361;
        let v2394: bool = (p.p103 == 1.0);
        self.scalar_v2394 = v2394;
        let v2395: bool = (v2394 && v430);
        self.scalar_v2395 = v2395;
        let v2400: f64 = p.p73;
        self.scalar_v2400 = v2400;
        let v2401: bool = (p.p73 != 0.0);
        self.scalar_v2401 = v2401;
        let v2402: bool = (p.p54 != 0.0);
        self.scalar_v2402 = v2402;
        let v2403: bool = (v2401 && v2402);
        self.scalar_v2403 = v2403;
        let v2408: f64 = p.p71;
        self.scalar_v2408 = v2408;
        let v2416: f64 = p.p72;
        self.scalar_v2416 = v2416;
        let v2420: bool = (!v2403);
        self.scalar_v2420 = v2420;
        let v2426: f64 = p.p70;
        self.scalar_v2426 = v2426;
        let v2428: f64 = p.p69;
        self.scalar_v2428 = v2428;
        let v2443: bool = (p.p28 >= p.p111);
        self.scalar_v2443 = v2443;
        let v2444: bool = (p.p29 >= p.p111);
        self.scalar_v2444 = v2444;
        let v2445: bool = (p.p23 >= p.p111);
        self.scalar_v2445 = v2445;
        let v2446: bool = (p.p26 >= p.p111);
        self.scalar_v2446 = v2446;
        let v2447: bool = (v2445 || v2446);
        self.scalar_v2447 = v2447;
        let v2448: bool = (p.p103 == 0.0);
        self.scalar_v2448 = v2448;
        let v2449: bool = (p.p104 < p.p111);
        self.scalar_v2449 = v2449;
        let v2450: bool = (v2448 || v2449);
        self.scalar_v2450 = v2450;
        let v2456: bool = (!v2443);
        self.scalar_v2456 = v2456;
        let v2457: f64 = (if v2456 { 0.0 } else { 0.0 });
        self.scalar_v2457 = v2457;
        let v2460: bool = (!v2444);
        self.scalar_v2460 = v2460;
        let v2461: f64 = (if v2460 { 0.0 } else { 0.0 });
        self.scalar_v2461 = v2461;
        let v2464: bool = (!v2447);
        self.scalar_v2464 = v2464;
        let v2465: f64 = (if v2464 { 0.0 } else { 0.0 });
        self.scalar_v2465 = v2465;
        let v2466: f64 = (if v2450 { 0.0 } else { 0.0 });
        self.scalar_v2466 = v2466;
        let v2467: bool = (!v2450);
        self.scalar_v2467 = v2467;
        let v2472: f64 = (p.p110 * -1.0);
        self.scalar_v2472 = v2472;
        let v2473: f64 = (-v2472);
        self.scalar_v2473 = v2473;
        let v2474: f64 = (p.p110 - p.p110);
        self.scalar_v2474 = v2474;
        let v2475: f64 = (if v431 { 1.0 } else { 0.0 });
        self.scalar_v2475 = v2475;
        let v2816: f64 = (-p.p110);
        self.scalar_v2816 = v2816;
        let v5075: f64 = (if v241 { v2473 } else { 0.0 });
        self.scalar_v5075 = v5075;
        let v5076: f64 = (if v241 { v2816 } else { 0.0 });
        self.scalar_v5076 = v5076;
        let v5079: f64 = (if v248 { v2473 } else { v5075 });
        self.scalar_v5079 = v5079;
        let v5080: f64 = (if v248 { v2474 } else { v5076 });
        self.scalar_v5080 = v5080;
        let v5081: f64 = (if v248 { v2472 } else { 0.0 });
        self.scalar_v5081 = v5081;
        let v7908: f64 = (if v276 { v2473 } else { 0.0 });
        self.scalar_v7908 = v7908;
        let v7909: f64 = (if v276 { v2816 } else { 0.0 });
        self.scalar_v7909 = v7909;
        let v8931: f64 = (if v2403 { 1.0 } else { 0.0 });
        self.scalar_v8931 = v8931;
        let v8942: f64 = (if v2403 { v8931 } else { 0.0 });
        self.scalar_v8942 = v8942;
        let v8948: f64 = (p.p71 * v8931);
        self.scalar_v8948 = v8948;
        let v8954: f64 = (v8948 * p.p54);
        self.scalar_v8954 = v8954;
        let v8960: f64 = (if v2403 { v8954 } else { 0.0 });
        self.scalar_v8960 = v8960;
        let v8981: f64 = (p.p72 * v8931);
        self.scalar_v8981 = v8981;
        let v8987: f64 = (v8981 * p.p54);
        self.scalar_v8987 = v8987;
        let v8993: f64 = (if v2403 { v8987 } else { 0.0 });
        self.scalar_v8993 = v8993;
        let v8999: f64 = (if v2420 { 1.0 } else { v8942 });
        self.scalar_v8999 = v8999;
        let v9005: f64 = (if v2420 { 0.0 } else { v8960 });
        self.scalar_v9005 = v9005;
        let v9016: f64 = (if v2420 { 0.0 } else { v8993 });
        self.scalar_v9016 = v9016;
        let v9022: f64 = (p.p70 * p.p110);
        self.scalar_v9022 = v9022;
        let v9023: f64 = (p.p70 * v2472);
        self.scalar_v9023 = v9023;
        let v9024: f64 = (p.p69 * p.p110);
        self.scalar_v9024 = v9024;
        let v9025: f64 = (p.p69 * v2472);
        self.scalar_v9025 = v9025;
        let v9050: f64 = (p.p110 * v9022);
        self.scalar_v9050 = v9050;
        let v9051: f64 = (p.p110 * v9023);
        self.scalar_v9051 = v9051;
        let v9052: f64 = (p.p110 * v9024);
        self.scalar_v9052 = v9052;
        let v9053: f64 = (p.p110 * v9025);
        self.scalar_v9053 = v9053;
        let v9072: f64 = (p.p110 * v8931);
        self.scalar_v9072 = v9072;
    }
}
