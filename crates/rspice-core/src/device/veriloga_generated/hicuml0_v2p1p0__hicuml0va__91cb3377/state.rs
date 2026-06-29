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
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: bool,
    pub(crate) scalar_v241: f64,
    pub(crate) scalar_v247: bool,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v259: f64,
    pub(crate) scalar_v260: bool,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v275: bool,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v287: bool,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v325: f64,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v382: bool,
    pub(crate) scalar_v383: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v390: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v395: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: bool,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: bool,
    pub(crate) scalar_v430: bool,
    pub(crate) scalar_v456: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v461: f64,
    pub(crate) scalar_v462: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v494: f64,
    pub(crate) scalar_v495: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v499: f64,
    pub(crate) scalar_v500: f64,
    pub(crate) scalar_v501: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v544: f64,
    pub(crate) scalar_v545: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v602: bool,
    pub(crate) scalar_v608: bool,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v625: bool,
    pub(crate) scalar_v631: bool,
    pub(crate) scalar_v640: bool,
    pub(crate) scalar_v647: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v652: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v654: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v682: f64,
    pub(crate) scalar_v683: f64,
    pub(crate) scalar_v684: f64,
    pub(crate) scalar_v730: bool,
    pub(crate) scalar_v736: bool,
    pub(crate) scalar_v746: f64,
    pub(crate) scalar_v768: f64,
    pub(crate) scalar_v772: f64,
    pub(crate) scalar_v775: f64,
    pub(crate) scalar_v776: bool,
    pub(crate) scalar_v780: f64,
    pub(crate) scalar_v786: f64,
    pub(crate) scalar_v787: f64,
    pub(crate) scalar_v788: f64,
    pub(crate) scalar_v858: f64,
    pub(crate) scalar_v862: f64,
    pub(crate) scalar_v906: bool,
    pub(crate) scalar_v951: f64,
    pub(crate) scalar_v952: bool,
    pub(crate) scalar_v956: f64,
    pub(crate) scalar_v960: f64,
    pub(crate) scalar_v961: f64,
    pub(crate) scalar_v962: f64,
    pub(crate) scalar_v1029: f64,
    pub(crate) scalar_v1033: f64,
    pub(crate) scalar_v1077: bool,
    pub(crate) scalar_v1465: f64,
    pub(crate) scalar_v1475: f64,
    pub(crate) scalar_v1478: f64,
    pub(crate) scalar_v1521: f64,
    pub(crate) scalar_v1526: f64,
    pub(crate) scalar_v1569: f64,
    pub(crate) scalar_v1574: f64,
    pub(crate) scalar_v1592: f64,
    pub(crate) scalar_v1595: f64,
    pub(crate) scalar_v1596: bool,
    pub(crate) scalar_v1597: f64,
    pub(crate) scalar_v1598: bool,
    pub(crate) scalar_v1599: f64,
    pub(crate) scalar_v1637: f64,
    pub(crate) scalar_v1651: f64,
    pub(crate) scalar_v1655: f64,
    pub(crate) scalar_v1660: f64,
    pub(crate) scalar_v1661: bool,
    pub(crate) scalar_v1668: bool,
    pub(crate) scalar_v1670: f64,
    pub(crate) scalar_v1671: f64,
    pub(crate) scalar_v1684: f64,
    pub(crate) scalar_v1697: bool,
    pub(crate) scalar_v1715: bool,
    pub(crate) scalar_v1729: f64,
    pub(crate) scalar_v1751: f64,
    pub(crate) scalar_v1752: bool,
    pub(crate) scalar_v1753: bool,
    pub(crate) scalar_v1759: bool,
    pub(crate) scalar_v1765: bool,
    pub(crate) scalar_v1767: f64,
    pub(crate) scalar_v1772: bool,
    pub(crate) scalar_v1773: bool,
    pub(crate) scalar_v1774: bool,
    pub(crate) scalar_v1775: bool,
    pub(crate) scalar_v1776: f64,
    pub(crate) scalar_v1777: bool,
    pub(crate) scalar_v1778: bool,
    pub(crate) scalar_v1907: f64,
    pub(crate) scalar_v1911: f64,
    pub(crate) scalar_v1912: f64,
    pub(crate) scalar_v1913: f64,
    pub(crate) scalar_v1918: f64,
    pub(crate) scalar_v1925: f64,
    pub(crate) scalar_v1929: f64,
    pub(crate) scalar_v1931: bool,
    pub(crate) scalar_v1932: f64,
    pub(crate) scalar_v1950: bool,
    pub(crate) scalar_v1952: bool,
    pub(crate) scalar_v1953: f64,
    pub(crate) scalar_v1971: bool,
    pub(crate) scalar_v1974: bool,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1993: bool,
    pub(crate) scalar_v2179: f64,
    pub(crate) scalar_v2182: f64,
    pub(crate) scalar_v2200: f64,
    pub(crate) scalar_v2222: bool,
    pub(crate) scalar_v2223: f64,
    pub(crate) scalar_v2235: bool,
    pub(crate) scalar_v2237: bool,
    pub(crate) scalar_v2238: f64,
    pub(crate) scalar_v2256: bool,
    pub(crate) scalar_v2258: f64,
    pub(crate) scalar_v2259: bool,
    pub(crate) scalar_v2262: f64,
    pub(crate) scalar_v2266: f64,
    pub(crate) scalar_v2267: f64,
    pub(crate) scalar_v2268: f64,
    pub(crate) scalar_v2329: f64,
    pub(crate) scalar_v2360: bool,
    pub(crate) scalar_v2393: bool,
    pub(crate) scalar_v2394: bool,
    pub(crate) scalar_v2399: f64,
    pub(crate) scalar_v2400: bool,
    pub(crate) scalar_v2401: bool,
    pub(crate) scalar_v2402: bool,
    pub(crate) scalar_v2407: f64,
    pub(crate) scalar_v2415: f64,
    pub(crate) scalar_v2419: bool,
    pub(crate) scalar_v2425: f64,
    pub(crate) scalar_v2427: f64,
    pub(crate) scalar_v2444: bool,
    pub(crate) scalar_v2445: bool,
    pub(crate) scalar_v2446: bool,
    pub(crate) scalar_v2447: bool,
    pub(crate) scalar_v2448: bool,
    pub(crate) scalar_v2449: bool,
    pub(crate) scalar_v2450: bool,
    pub(crate) scalar_v2451: bool,
    pub(crate) scalar_v2457: bool,
    pub(crate) scalar_v2458: f64,
    pub(crate) scalar_v2461: bool,
    pub(crate) scalar_v2462: f64,
    pub(crate) scalar_v2465: bool,
    pub(crate) scalar_v2466: f64,
    pub(crate) scalar_v2467: f64,
    pub(crate) scalar_v2468: bool,
    pub(crate) scalar_v2473: f64,
    pub(crate) scalar_v2474: f64,
    pub(crate) scalar_v2475: f64,
    pub(crate) scalar_v5056: f64,
    pub(crate) scalar_v5057: f64,
    pub(crate) scalar_v5060: f64,
    pub(crate) scalar_v5061: f64,
    pub(crate) scalar_v5062: f64,
    pub(crate) scalar_v7815: f64,
    pub(crate) scalar_v7816: f64,
    pub(crate) scalar_v8825: f64,
    pub(crate) scalar_v8836: f64,
    pub(crate) scalar_v8842: f64,
    pub(crate) scalar_v8848: f64,
    pub(crate) scalar_v8854: f64,
    pub(crate) scalar_v8875: f64,
    pub(crate) scalar_v8881: f64,
    pub(crate) scalar_v8887: f64,
    pub(crate) scalar_v8893: f64,
    pub(crate) scalar_v8899: f64,
    pub(crate) scalar_v8910: f64,
    pub(crate) scalar_v8916: f64,
    pub(crate) scalar_v8917: f64,
    pub(crate) scalar_v8918: f64,
    pub(crate) scalar_v8919: f64,
    pub(crate) scalar_v8944: f64,
    pub(crate) scalar_v8945: f64,
    pub(crate) scalar_v8946: f64,
    pub(crate) scalar_v8947: f64,
    pub(crate) scalar_v8966: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v73: bool,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v76: bool,
    pub(crate) scalar_v77: bool,
    pub(crate) scalar_v78: bool,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: f64,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
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
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v375: f64,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v386: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v388: f64,
    pub(crate) scalar_v389: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
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
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v119: self.scalar_v119,
            scalar_v125: self.scalar_v125,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v151: self.scalar_v151,
            scalar_v157: self.scalar_v157,
            scalar_v160: self.scalar_v160,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v169: self.scalar_v169,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v198: self.scalar_v198,
            scalar_v204: self.scalar_v204,
            scalar_v210: self.scalar_v210,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v219: self.scalar_v219,
            scalar_v224: self.scalar_v224,
            scalar_v225: self.scalar_v225,
            scalar_v229: self.scalar_v229,
            scalar_v234: self.scalar_v234,
            scalar_v235: self.scalar_v235,
            scalar_v239: self.scalar_v239,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v247: self.scalar_v247,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v254: self.scalar_v254,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v268: self.scalar_v268,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v275: self.scalar_v275,
            scalar_v276: self.scalar_v276,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v287: self.scalar_v287,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v356: self.scalar_v356,
            scalar_v362: self.scalar_v362,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v371: self.scalar_v371,
            scalar_v373: self.scalar_v373,
            scalar_v380: self.scalar_v380,
            scalar_v382: self.scalar_v382,
            scalar_v383: self.scalar_v383,
            scalar_v384: self.scalar_v384,
            scalar_v390: self.scalar_v390,
            scalar_v391: self.scalar_v391,
            scalar_v395: self.scalar_v395,
            scalar_v397: self.scalar_v397,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v407: self.scalar_v407,
            scalar_v408: self.scalar_v408,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v422: self.scalar_v422,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v456: self.scalar_v456,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v461: self.scalar_v461,
            scalar_v462: self.scalar_v462,
            scalar_v463: self.scalar_v463,
            scalar_v494: self.scalar_v494,
            scalar_v495: self.scalar_v495,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v499: self.scalar_v499,
            scalar_v500: self.scalar_v500,
            scalar_v501: self.scalar_v501,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v544: self.scalar_v544,
            scalar_v545: self.scalar_v545,
            scalar_v546: self.scalar_v546,
            scalar_v602: self.scalar_v602,
            scalar_v608: self.scalar_v608,
            scalar_v618: self.scalar_v618,
            scalar_v625: self.scalar_v625,
            scalar_v631: self.scalar_v631,
            scalar_v640: self.scalar_v640,
            scalar_v647: self.scalar_v647,
            scalar_v648: self.scalar_v648,
            scalar_v649: self.scalar_v649,
            scalar_v650: self.scalar_v650,
            scalar_v651: self.scalar_v651,
            scalar_v652: self.scalar_v652,
            scalar_v653: self.scalar_v653,
            scalar_v654: self.scalar_v654,
            scalar_v677: self.scalar_v677,
            scalar_v678: self.scalar_v678,
            scalar_v679: self.scalar_v679,
            scalar_v680: self.scalar_v680,
            scalar_v681: self.scalar_v681,
            scalar_v682: self.scalar_v682,
            scalar_v683: self.scalar_v683,
            scalar_v684: self.scalar_v684,
            scalar_v730: self.scalar_v730,
            scalar_v736: self.scalar_v736,
            scalar_v746: self.scalar_v746,
            scalar_v768: self.scalar_v768,
            scalar_v772: self.scalar_v772,
            scalar_v775: self.scalar_v775,
            scalar_v776: self.scalar_v776,
            scalar_v780: self.scalar_v780,
            scalar_v786: self.scalar_v786,
            scalar_v787: self.scalar_v787,
            scalar_v788: self.scalar_v788,
            scalar_v858: self.scalar_v858,
            scalar_v862: self.scalar_v862,
            scalar_v906: self.scalar_v906,
            scalar_v951: self.scalar_v951,
            scalar_v952: self.scalar_v952,
            scalar_v956: self.scalar_v956,
            scalar_v960: self.scalar_v960,
            scalar_v961: self.scalar_v961,
            scalar_v962: self.scalar_v962,
            scalar_v1029: self.scalar_v1029,
            scalar_v1033: self.scalar_v1033,
            scalar_v1077: self.scalar_v1077,
            scalar_v1465: self.scalar_v1465,
            scalar_v1475: self.scalar_v1475,
            scalar_v1478: self.scalar_v1478,
            scalar_v1521: self.scalar_v1521,
            scalar_v1526: self.scalar_v1526,
            scalar_v1569: self.scalar_v1569,
            scalar_v1574: self.scalar_v1574,
            scalar_v1592: self.scalar_v1592,
            scalar_v1595: self.scalar_v1595,
            scalar_v1596: self.scalar_v1596,
            scalar_v1597: self.scalar_v1597,
            scalar_v1598: self.scalar_v1598,
            scalar_v1599: self.scalar_v1599,
            scalar_v1637: self.scalar_v1637,
            scalar_v1651: self.scalar_v1651,
            scalar_v1655: self.scalar_v1655,
            scalar_v1660: self.scalar_v1660,
            scalar_v1661: self.scalar_v1661,
            scalar_v1668: self.scalar_v1668,
            scalar_v1670: self.scalar_v1670,
            scalar_v1671: self.scalar_v1671,
            scalar_v1684: self.scalar_v1684,
            scalar_v1697: self.scalar_v1697,
            scalar_v1715: self.scalar_v1715,
            scalar_v1729: self.scalar_v1729,
            scalar_v1751: self.scalar_v1751,
            scalar_v1752: self.scalar_v1752,
            scalar_v1753: self.scalar_v1753,
            scalar_v1759: self.scalar_v1759,
            scalar_v1765: self.scalar_v1765,
            scalar_v1767: self.scalar_v1767,
            scalar_v1772: self.scalar_v1772,
            scalar_v1773: self.scalar_v1773,
            scalar_v1774: self.scalar_v1774,
            scalar_v1775: self.scalar_v1775,
            scalar_v1776: self.scalar_v1776,
            scalar_v1777: self.scalar_v1777,
            scalar_v1778: self.scalar_v1778,
            scalar_v1907: self.scalar_v1907,
            scalar_v1911: self.scalar_v1911,
            scalar_v1912: self.scalar_v1912,
            scalar_v1913: self.scalar_v1913,
            scalar_v1918: self.scalar_v1918,
            scalar_v1925: self.scalar_v1925,
            scalar_v1929: self.scalar_v1929,
            scalar_v1931: self.scalar_v1931,
            scalar_v1932: self.scalar_v1932,
            scalar_v1950: self.scalar_v1950,
            scalar_v1952: self.scalar_v1952,
            scalar_v1953: self.scalar_v1953,
            scalar_v1971: self.scalar_v1971,
            scalar_v1974: self.scalar_v1974,
            scalar_v1975: self.scalar_v1975,
            scalar_v1993: self.scalar_v1993,
            scalar_v2179: self.scalar_v2179,
            scalar_v2182: self.scalar_v2182,
            scalar_v2200: self.scalar_v2200,
            scalar_v2222: self.scalar_v2222,
            scalar_v2223: self.scalar_v2223,
            scalar_v2235: self.scalar_v2235,
            scalar_v2237: self.scalar_v2237,
            scalar_v2238: self.scalar_v2238,
            scalar_v2256: self.scalar_v2256,
            scalar_v2258: self.scalar_v2258,
            scalar_v2259: self.scalar_v2259,
            scalar_v2262: self.scalar_v2262,
            scalar_v2266: self.scalar_v2266,
            scalar_v2267: self.scalar_v2267,
            scalar_v2268: self.scalar_v2268,
            scalar_v2329: self.scalar_v2329,
            scalar_v2360: self.scalar_v2360,
            scalar_v2393: self.scalar_v2393,
            scalar_v2394: self.scalar_v2394,
            scalar_v2399: self.scalar_v2399,
            scalar_v2400: self.scalar_v2400,
            scalar_v2401: self.scalar_v2401,
            scalar_v2402: self.scalar_v2402,
            scalar_v2407: self.scalar_v2407,
            scalar_v2415: self.scalar_v2415,
            scalar_v2419: self.scalar_v2419,
            scalar_v2425: self.scalar_v2425,
            scalar_v2427: self.scalar_v2427,
            scalar_v2444: self.scalar_v2444,
            scalar_v2445: self.scalar_v2445,
            scalar_v2446: self.scalar_v2446,
            scalar_v2447: self.scalar_v2447,
            scalar_v2448: self.scalar_v2448,
            scalar_v2449: self.scalar_v2449,
            scalar_v2450: self.scalar_v2450,
            scalar_v2451: self.scalar_v2451,
            scalar_v2457: self.scalar_v2457,
            scalar_v2458: self.scalar_v2458,
            scalar_v2461: self.scalar_v2461,
            scalar_v2462: self.scalar_v2462,
            scalar_v2465: self.scalar_v2465,
            scalar_v2466: self.scalar_v2466,
            scalar_v2467: self.scalar_v2467,
            scalar_v2468: self.scalar_v2468,
            scalar_v2473: self.scalar_v2473,
            scalar_v2474: self.scalar_v2474,
            scalar_v2475: self.scalar_v2475,
            scalar_v5056: self.scalar_v5056,
            scalar_v5057: self.scalar_v5057,
            scalar_v5060: self.scalar_v5060,
            scalar_v5061: self.scalar_v5061,
            scalar_v5062: self.scalar_v5062,
            scalar_v7815: self.scalar_v7815,
            scalar_v7816: self.scalar_v7816,
            scalar_v8825: self.scalar_v8825,
            scalar_v8836: self.scalar_v8836,
            scalar_v8842: self.scalar_v8842,
            scalar_v8848: self.scalar_v8848,
            scalar_v8854: self.scalar_v8854,
            scalar_v8875: self.scalar_v8875,
            scalar_v8881: self.scalar_v8881,
            scalar_v8887: self.scalar_v8887,
            scalar_v8893: self.scalar_v8893,
            scalar_v8899: self.scalar_v8899,
            scalar_v8910: self.scalar_v8910,
            scalar_v8916: self.scalar_v8916,
            scalar_v8917: self.scalar_v8917,
            scalar_v8918: self.scalar_v8918,
            scalar_v8919: self.scalar_v8919,
            scalar_v8944: self.scalar_v8944,
            scalar_v8945: self.scalar_v8945,
            scalar_v8946: self.scalar_v8946,
            scalar_v8947: self.scalar_v8947,
            scalar_v8966: self.scalar_v8966,
            scalar_v70: self.scalar_v70,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
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
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
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
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v168: self.scalar_v168,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v183: self.scalar_v183,
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v218: self.scalar_v218,
            scalar_v220: self.scalar_v220,
            scalar_v221: self.scalar_v221,
            scalar_v222: self.scalar_v222,
            scalar_v223: self.scalar_v223,
            scalar_v226: self.scalar_v226,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v233: self.scalar_v233,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v238: self.scalar_v238,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v269: self.scalar_v269,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
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
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v360: self.scalar_v360,
            scalar_v361: self.scalar_v361,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v365: self.scalar_v365,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v372: self.scalar_v372,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v378: self.scalar_v378,
            scalar_v379: self.scalar_v379,
            scalar_v385: self.scalar_v385,
            scalar_v386: self.scalar_v386,
            scalar_v387: self.scalar_v387,
            scalar_v388: self.scalar_v388,
            scalar_v389: self.scalar_v389,
            scalar_v392: self.scalar_v392,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v396: self.scalar_v396,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v404: self.scalar_v404,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v421: self.scalar_v421,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
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
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v119: 0.0,
            scalar_v125: 0.0,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v151: 0.0,
            scalar_v157: 0.0,
            scalar_v160: 0.0,
            scalar_v166: 0.0,
            scalar_v167: 0.0,
            scalar_v169: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v198: 0.0,
            scalar_v204: 0.0,
            scalar_v210: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v219: 0.0,
            scalar_v224: 0.0,
            scalar_v225: 0.0,
            scalar_v229: 0.0,
            scalar_v234: 0.0,
            scalar_v235: 0.0,
            scalar_v239: 0.0,
            scalar_v240: false,
            scalar_v241: 0.0,
            scalar_v247: false,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v254: 0.0,
            scalar_v259: 0.0,
            scalar_v260: false,
            scalar_v261: 0.0,
            scalar_v268: false,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v275: false,
            scalar_v276: 0.0,
            scalar_v281: 0.0,
            scalar_v282: 0.0,
            scalar_v287: false,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v325: 0.0,
            scalar_v326: 0.0,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v356: 0.0,
            scalar_v362: 0.0,
            scalar_v366: 0.0,
            scalar_v367: 0.0,
            scalar_v371: 0.0,
            scalar_v373: 0.0,
            scalar_v380: 0.0,
            scalar_v382: false,
            scalar_v383: 0.0,
            scalar_v384: 0.0,
            scalar_v390: false,
            scalar_v391: 0.0,
            scalar_v395: 0.0,
            scalar_v397: 0.0,
            scalar_v401: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v407: 0.0,
            scalar_v408: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v422: 0.0,
            scalar_v426: 0.0,
            scalar_v427: false,
            scalar_v428: 0.0,
            scalar_v429: false,
            scalar_v430: false,
            scalar_v456: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v461: 0.0,
            scalar_v462: 0.0,
            scalar_v463: 0.0,
            scalar_v494: 0.0,
            scalar_v495: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v499: 0.0,
            scalar_v500: 0.0,
            scalar_v501: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v544: 0.0,
            scalar_v545: 0.0,
            scalar_v546: 0.0,
            scalar_v602: false,
            scalar_v608: false,
            scalar_v618: false,
            scalar_v625: false,
            scalar_v631: false,
            scalar_v640: false,
            scalar_v647: 0.0,
            scalar_v648: 0.0,
            scalar_v649: 0.0,
            scalar_v650: 0.0,
            scalar_v651: 0.0,
            scalar_v652: 0.0,
            scalar_v653: 0.0,
            scalar_v654: 0.0,
            scalar_v677: 0.0,
            scalar_v678: 0.0,
            scalar_v679: 0.0,
            scalar_v680: 0.0,
            scalar_v681: 0.0,
            scalar_v682: 0.0,
            scalar_v683: 0.0,
            scalar_v684: 0.0,
            scalar_v730: false,
            scalar_v736: false,
            scalar_v746: 0.0,
            scalar_v768: 0.0,
            scalar_v772: 0.0,
            scalar_v775: 0.0,
            scalar_v776: false,
            scalar_v780: 0.0,
            scalar_v786: 0.0,
            scalar_v787: 0.0,
            scalar_v788: 0.0,
            scalar_v858: 0.0,
            scalar_v862: 0.0,
            scalar_v906: false,
            scalar_v951: 0.0,
            scalar_v952: false,
            scalar_v956: 0.0,
            scalar_v960: 0.0,
            scalar_v961: 0.0,
            scalar_v962: 0.0,
            scalar_v1029: 0.0,
            scalar_v1033: 0.0,
            scalar_v1077: false,
            scalar_v1465: 0.0,
            scalar_v1475: 0.0,
            scalar_v1478: 0.0,
            scalar_v1521: 0.0,
            scalar_v1526: 0.0,
            scalar_v1569: 0.0,
            scalar_v1574: 0.0,
            scalar_v1592: 0.0,
            scalar_v1595: 0.0,
            scalar_v1596: false,
            scalar_v1597: 0.0,
            scalar_v1598: false,
            scalar_v1599: 0.0,
            scalar_v1637: 0.0,
            scalar_v1651: 0.0,
            scalar_v1655: 0.0,
            scalar_v1660: 0.0,
            scalar_v1661: false,
            scalar_v1668: false,
            scalar_v1670: 0.0,
            scalar_v1671: 0.0,
            scalar_v1684: 0.0,
            scalar_v1697: false,
            scalar_v1715: false,
            scalar_v1729: 0.0,
            scalar_v1751: 0.0,
            scalar_v1752: false,
            scalar_v1753: false,
            scalar_v1759: false,
            scalar_v1765: false,
            scalar_v1767: 0.0,
            scalar_v1772: false,
            scalar_v1773: false,
            scalar_v1774: false,
            scalar_v1775: false,
            scalar_v1776: 0.0,
            scalar_v1777: false,
            scalar_v1778: false,
            scalar_v1907: 0.0,
            scalar_v1911: 0.0,
            scalar_v1912: 0.0,
            scalar_v1913: 0.0,
            scalar_v1918: 0.0,
            scalar_v1925: 0.0,
            scalar_v1929: 0.0,
            scalar_v1931: false,
            scalar_v1932: 0.0,
            scalar_v1950: false,
            scalar_v1952: false,
            scalar_v1953: 0.0,
            scalar_v1971: false,
            scalar_v1974: false,
            scalar_v1975: 0.0,
            scalar_v1993: false,
            scalar_v2179: 0.0,
            scalar_v2182: 0.0,
            scalar_v2200: 0.0,
            scalar_v2222: false,
            scalar_v2223: 0.0,
            scalar_v2235: false,
            scalar_v2237: false,
            scalar_v2238: 0.0,
            scalar_v2256: false,
            scalar_v2258: 0.0,
            scalar_v2259: false,
            scalar_v2262: 0.0,
            scalar_v2266: 0.0,
            scalar_v2267: 0.0,
            scalar_v2268: 0.0,
            scalar_v2329: 0.0,
            scalar_v2360: false,
            scalar_v2393: false,
            scalar_v2394: false,
            scalar_v2399: 0.0,
            scalar_v2400: false,
            scalar_v2401: false,
            scalar_v2402: false,
            scalar_v2407: 0.0,
            scalar_v2415: 0.0,
            scalar_v2419: false,
            scalar_v2425: 0.0,
            scalar_v2427: 0.0,
            scalar_v2444: false,
            scalar_v2445: false,
            scalar_v2446: false,
            scalar_v2447: false,
            scalar_v2448: false,
            scalar_v2449: false,
            scalar_v2450: false,
            scalar_v2451: false,
            scalar_v2457: false,
            scalar_v2458: 0.0,
            scalar_v2461: false,
            scalar_v2462: 0.0,
            scalar_v2465: false,
            scalar_v2466: 0.0,
            scalar_v2467: 0.0,
            scalar_v2468: false,
            scalar_v2473: 0.0,
            scalar_v2474: 0.0,
            scalar_v2475: 0.0,
            scalar_v5056: 0.0,
            scalar_v5057: 0.0,
            scalar_v5060: 0.0,
            scalar_v5061: 0.0,
            scalar_v5062: 0.0,
            scalar_v7815: 0.0,
            scalar_v7816: 0.0,
            scalar_v8825: 0.0,
            scalar_v8836: 0.0,
            scalar_v8842: 0.0,
            scalar_v8848: 0.0,
            scalar_v8854: 0.0,
            scalar_v8875: 0.0,
            scalar_v8881: 0.0,
            scalar_v8887: 0.0,
            scalar_v8893: 0.0,
            scalar_v8899: 0.0,
            scalar_v8910: 0.0,
            scalar_v8916: 0.0,
            scalar_v8917: 0.0,
            scalar_v8918: 0.0,
            scalar_v8919: 0.0,
            scalar_v8944: 0.0,
            scalar_v8945: 0.0,
            scalar_v8946: 0.0,
            scalar_v8947: 0.0,
            scalar_v8966: 0.0,
            scalar_v70: 0.0,
            scalar_v73: false,
            scalar_v74: 0.0,
            scalar_v76: false,
            scalar_v77: false,
            scalar_v78: false,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v118: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v168: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v218: 0.0,
            scalar_v220: 0.0,
            scalar_v221: 0.0,
            scalar_v222: 0.0,
            scalar_v223: 0.0,
            scalar_v226: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v233: 0.0,
            scalar_v236: 0.0,
            scalar_v237: 0.0,
            scalar_v238: 0.0,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v244: 0.0,
            scalar_v245: 0.0,
            scalar_v246: 0.0,
            scalar_v248: 0.0,
            scalar_v249: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v257: 0.0,
            scalar_v258: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v264: 0.0,
            scalar_v265: 0.0,
            scalar_v266: 0.0,
            scalar_v267: 0.0,
            scalar_v269: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v277: 0.0,
            scalar_v278: 0.0,
            scalar_v279: 0.0,
            scalar_v280: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
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
            scalar_v317: 0.0,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: 0.0,
            scalar_v324: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v342: 0.0,
            scalar_v343: 0.0,
            scalar_v344: 0.0,
            scalar_v345: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v351: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v354: 0.0,
            scalar_v355: 0.0,
            scalar_v357: 0.0,
            scalar_v358: 0.0,
            scalar_v359: 0.0,
            scalar_v360: 0.0,
            scalar_v361: 0.0,
            scalar_v363: 0.0,
            scalar_v364: 0.0,
            scalar_v365: 0.0,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v370: 0.0,
            scalar_v372: 0.0,
            scalar_v374: 0.0,
            scalar_v375: 0.0,
            scalar_v376: 0.0,
            scalar_v377: 0.0,
            scalar_v378: 0.0,
            scalar_v379: 0.0,
            scalar_v385: 0.0,
            scalar_v386: 0.0,
            scalar_v387: 0.0,
            scalar_v388: 0.0,
            scalar_v389: 0.0,
            scalar_v392: 0.0,
            scalar_v393: 0.0,
            scalar_v394: 0.0,
            scalar_v396: 0.0,
            scalar_v398: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v404: 0.0,
            scalar_v405: 0.0,
            scalar_v406: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v421: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v119,
            scalar_v125,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v151,
            scalar_v157,
            scalar_v160,
            scalar_v166,
            scalar_v167,
            scalar_v169,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v198,
            scalar_v204,
            scalar_v210,
            scalar_v216,
            scalar_v217,
            scalar_v219,
            scalar_v224,
            scalar_v225,
            scalar_v229,
            scalar_v234,
            scalar_v235,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v247,
            scalar_v250,
            scalar_v251,
            scalar_v254,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v268,
            scalar_v270,
            scalar_v271,
            scalar_v275,
            scalar_v276,
            scalar_v281,
            scalar_v282,
            scalar_v287,
            scalar_v290,
            scalar_v291,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v318,
            scalar_v319,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v349,
            scalar_v350,
            scalar_v356,
            scalar_v362,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v373,
            scalar_v380,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v390,
            scalar_v391,
            scalar_v395,
            scalar_v397,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v407,
            scalar_v408,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v422,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v602,
            scalar_v608,
            scalar_v618,
            scalar_v625,
            scalar_v631,
            scalar_v640,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v654,
            scalar_v677,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v730,
            scalar_v736,
            scalar_v746,
            scalar_v768,
            scalar_v772,
            scalar_v775,
            scalar_v776,
            scalar_v780,
            scalar_v786,
            scalar_v787,
            scalar_v788,
            scalar_v858,
            scalar_v862,
            scalar_v906,
            scalar_v951,
            scalar_v952,
            scalar_v956,
            scalar_v960,
            scalar_v961,
            scalar_v962,
            scalar_v1029,
            scalar_v1033,
            scalar_v1077,
            scalar_v1465,
            scalar_v1475,
            scalar_v1478,
            scalar_v1521,
            scalar_v1526,
            scalar_v1569,
            scalar_v1574,
            scalar_v1592,
            scalar_v1595,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1637,
            scalar_v1651,
            scalar_v1655,
            scalar_v1660,
            scalar_v1661,
            scalar_v1668,
            scalar_v1670,
            scalar_v1671,
            scalar_v1684,
            scalar_v1697,
            scalar_v1715,
            scalar_v1729,
            scalar_v1751,
            scalar_v1752,
            scalar_v1753,
            scalar_v1759,
            scalar_v1765,
            scalar_v1767,
            scalar_v1772,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1907,
            scalar_v1911,
            scalar_v1912,
            scalar_v1913,
            scalar_v1918,
            scalar_v1925,
            scalar_v1929,
            scalar_v1931,
            scalar_v1932,
            scalar_v1950,
            scalar_v1952,
            scalar_v1953,
            scalar_v1971,
            scalar_v1974,
            scalar_v1975,
            scalar_v1993,
            scalar_v2179,
            scalar_v2182,
            scalar_v2200,
            scalar_v2222,
            scalar_v2223,
            scalar_v2235,
            scalar_v2237,
            scalar_v2238,
            scalar_v2256,
            scalar_v2258,
            scalar_v2259,
            scalar_v2262,
            scalar_v2266,
            scalar_v2267,
            scalar_v2268,
            scalar_v2329,
            scalar_v2360,
            scalar_v2393,
            scalar_v2394,
            scalar_v2399,
            scalar_v2400,
            scalar_v2401,
            scalar_v2402,
            scalar_v2407,
            scalar_v2415,
            scalar_v2419,
            scalar_v2425,
            scalar_v2427,
            scalar_v2444,
            scalar_v2445,
            scalar_v2446,
            scalar_v2447,
            scalar_v2448,
            scalar_v2449,
            scalar_v2450,
            scalar_v2451,
            scalar_v2457,
            scalar_v2458,
            scalar_v2461,
            scalar_v2462,
            scalar_v2465,
            scalar_v2466,
            scalar_v2467,
            scalar_v2468,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v5056,
            scalar_v5057,
            scalar_v5060,
            scalar_v5061,
            scalar_v5062,
            scalar_v7815,
            scalar_v7816,
            scalar_v8825,
            scalar_v8836,
            scalar_v8842,
            scalar_v8848,
            scalar_v8854,
            scalar_v8875,
            scalar_v8881,
            scalar_v8887,
            scalar_v8893,
            scalar_v8899,
            scalar_v8910,
            scalar_v8916,
            scalar_v8917,
            scalar_v8918,
            scalar_v8919,
            scalar_v8944,
            scalar_v8945,
            scalar_v8946,
            scalar_v8947,
            scalar_v8966,
            scalar_v70,
            scalar_v73,
            scalar_v74,
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
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
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
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v126,
            scalar_v127,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v158,
            scalar_v159,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v168,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v218,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v248,
            scalar_v249,
            scalar_v252,
            scalar_v253,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v269,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v288,
            scalar_v289,
            scalar_v292,
            scalar_v293,
            scalar_v294,
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
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v372,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v388,
            scalar_v389,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v396,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v119,
            scalar_v125,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v151,
            scalar_v157,
            scalar_v160,
            scalar_v166,
            scalar_v167,
            scalar_v169,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v198,
            scalar_v204,
            scalar_v210,
            scalar_v216,
            scalar_v217,
            scalar_v219,
            scalar_v224,
            scalar_v225,
            scalar_v229,
            scalar_v234,
            scalar_v235,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v247,
            scalar_v250,
            scalar_v251,
            scalar_v254,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v268,
            scalar_v270,
            scalar_v271,
            scalar_v275,
            scalar_v276,
            scalar_v281,
            scalar_v282,
            scalar_v287,
            scalar_v290,
            scalar_v291,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v318,
            scalar_v319,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v349,
            scalar_v350,
            scalar_v356,
            scalar_v362,
            scalar_v366,
            scalar_v367,
            scalar_v371,
            scalar_v373,
            scalar_v380,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v390,
            scalar_v391,
            scalar_v395,
            scalar_v397,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v407,
            scalar_v408,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v422,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v456,
            scalar_v457,
            scalar_v458,
            scalar_v459,
            scalar_v460,
            scalar_v461,
            scalar_v462,
            scalar_v463,
            scalar_v494,
            scalar_v495,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v499,
            scalar_v500,
            scalar_v501,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v542,
            scalar_v543,
            scalar_v544,
            scalar_v545,
            scalar_v546,
            scalar_v602,
            scalar_v608,
            scalar_v618,
            scalar_v625,
            scalar_v631,
            scalar_v640,
            scalar_v647,
            scalar_v648,
            scalar_v649,
            scalar_v650,
            scalar_v651,
            scalar_v652,
            scalar_v653,
            scalar_v654,
            scalar_v677,
            scalar_v678,
            scalar_v679,
            scalar_v680,
            scalar_v681,
            scalar_v682,
            scalar_v683,
            scalar_v684,
            scalar_v730,
            scalar_v736,
            scalar_v746,
            scalar_v768,
            scalar_v772,
            scalar_v775,
            scalar_v776,
            scalar_v780,
            scalar_v786,
            scalar_v787,
            scalar_v788,
            scalar_v858,
            scalar_v862,
            scalar_v906,
            scalar_v951,
            scalar_v952,
            scalar_v956,
            scalar_v960,
            scalar_v961,
            scalar_v962,
            scalar_v1029,
            scalar_v1033,
            scalar_v1077,
            scalar_v1465,
            scalar_v1475,
            scalar_v1478,
            scalar_v1521,
            scalar_v1526,
            scalar_v1569,
            scalar_v1574,
            scalar_v1592,
            scalar_v1595,
            scalar_v1596,
            scalar_v1597,
            scalar_v1598,
            scalar_v1599,
            scalar_v1637,
            scalar_v1651,
            scalar_v1655,
            scalar_v1660,
            scalar_v1661,
            scalar_v1668,
            scalar_v1670,
            scalar_v1671,
            scalar_v1684,
            scalar_v1697,
            scalar_v1715,
            scalar_v1729,
            scalar_v1751,
            scalar_v1752,
            scalar_v1753,
            scalar_v1759,
            scalar_v1765,
            scalar_v1767,
            scalar_v1772,
            scalar_v1773,
            scalar_v1774,
            scalar_v1775,
            scalar_v1776,
            scalar_v1777,
            scalar_v1778,
            scalar_v1907,
            scalar_v1911,
            scalar_v1912,
            scalar_v1913,
            scalar_v1918,
            scalar_v1925,
            scalar_v1929,
            scalar_v1931,
            scalar_v1932,
            scalar_v1950,
            scalar_v1952,
            scalar_v1953,
            scalar_v1971,
            scalar_v1974,
            scalar_v1975,
            scalar_v1993,
            scalar_v2179,
            scalar_v2182,
            scalar_v2200,
            scalar_v2222,
            scalar_v2223,
            scalar_v2235,
            scalar_v2237,
            scalar_v2238,
            scalar_v2256,
            scalar_v2258,
            scalar_v2259,
            scalar_v2262,
            scalar_v2266,
            scalar_v2267,
            scalar_v2268,
            scalar_v2329,
            scalar_v2360,
            scalar_v2393,
            scalar_v2394,
            scalar_v2399,
            scalar_v2400,
            scalar_v2401,
            scalar_v2402,
            scalar_v2407,
            scalar_v2415,
            scalar_v2419,
            scalar_v2425,
            scalar_v2427,
            scalar_v2444,
            scalar_v2445,
            scalar_v2446,
            scalar_v2447,
            scalar_v2448,
            scalar_v2449,
            scalar_v2450,
            scalar_v2451,
            scalar_v2457,
            scalar_v2458,
            scalar_v2461,
            scalar_v2462,
            scalar_v2465,
            scalar_v2466,
            scalar_v2467,
            scalar_v2468,
            scalar_v2473,
            scalar_v2474,
            scalar_v2475,
            scalar_v5056,
            scalar_v5057,
            scalar_v5060,
            scalar_v5061,
            scalar_v5062,
            scalar_v7815,
            scalar_v7816,
            scalar_v8825,
            scalar_v8836,
            scalar_v8842,
            scalar_v8848,
            scalar_v8854,
            scalar_v8875,
            scalar_v8881,
            scalar_v8887,
            scalar_v8893,
            scalar_v8899,
            scalar_v8910,
            scalar_v8916,
            scalar_v8917,
            scalar_v8918,
            scalar_v8919,
            scalar_v8944,
            scalar_v8945,
            scalar_v8946,
            scalar_v8947,
            scalar_v8966,
            scalar_v70,
            scalar_v73,
            scalar_v74,
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
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
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
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v126,
            scalar_v127,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v152,
            scalar_v153,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v158,
            scalar_v159,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v168,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v183,
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v215,
            scalar_v218,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v248,
            scalar_v249,
            scalar_v252,
            scalar_v253,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v269,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v288,
            scalar_v289,
            scalar_v292,
            scalar_v293,
            scalar_v294,
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
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v363,
            scalar_v364,
            scalar_v365,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v372,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v385,
            scalar_v386,
            scalar_v387,
            scalar_v388,
            scalar_v389,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v396,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v423,
            scalar_v424,
            scalar_v425,
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
            "flcomp" => { validate_parameter("flcomp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flitm" => { validate_parameter("flitm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcf" => { validate_parameter("mcf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcr" => { validate_parameter("mcr", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aver" => { validate_parameter("aver", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rver" => { validate_parameter("rver", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqf" => { validate_parameter("iqf", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fiqf" => { validate_parameter("fiqf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqr" => { validate_parameter("iqr", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iqfh" => { validate_parameter("iqfh", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tfh" => { validate_parameter("tfh", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahq" => { validate_parameter("ahq", value, Some((-0.9, "-0.9")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibes" => { validate_parameter("ibes", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbe" => { validate_parameter("mbe", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ires" => { validate_parameter("ires", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mre" => { validate_parameter("mre", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcs" => { validate_parameter("ibcs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mbc" => { validate_parameter("mbc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "favl" => { validate_parameter("favl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qavl" => { validate_parameter("qavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi0" => { validate_parameter("rbi0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vr0e" => { validate_parameter("vr0e", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vr0c" => { validate_parameter("vr0c", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fgeo" => { validate_parameter("fgeo", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itss" => { validate_parameter("itss", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msf" => { validate_parameter("msf", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iscs" => { validate_parameter("iscs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "msc" => { validate_parameter("msc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje0" => { validate_parameter("cje0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ze" => { validate_parameter("ze", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aje" => { validate_parameter("aje", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdedc" => { validate_parameter("vdedc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zedc" => { validate_parameter("zedc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajedc" => { validate_parameter("ajedc", value, Some((1.0, "1.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjci0" => { validate_parameter("cjci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdci" => { validate_parameter("vdci", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zci" => { validate_parameter("zci", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptci" => { validate_parameter("vptci", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcx0" => { validate_parameter("cjcx0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcx" => { validate_parameter("vdcx", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zcx" => { validate_parameter("zcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vptcx" => { validate_parameter("vptcx", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbc" => { validate_parameter("fbc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs0" => { validate_parameter("cjs0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vds" => { validate_parameter("vds", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zs" => { validate_parameter("zs", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpts" => { validate_parameter("vpts", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "t0" => { validate_parameter("t0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt0h" => { validate_finite_parameter("dt0h", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbvl" => { validate_parameter("tbvl", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tef0" => { validate_parameter("tef0", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gte" => { validate_parameter("gte", value, Some((0.0, "0.0")), true, Some((20.0, "20.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thcs" => { validate_parameter("thcs", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ahc" => { validate_parameter("ahc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci0" => { validate_parameter("rci0", value, Some((0.0, "0.0")), true, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vlim" => { validate_parameter("vlim", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpt" => { validate_parameter("vpt", value, Some((0.0, "0.0")), true, Some((100.0, "100.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vces" => { validate_parameter("vces", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdck" => { validate_parameter("vdck", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aick" => { validate_parameter("aick", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delck" => { validate_parameter("delck", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbepar" => { validate_parameter("cbepar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbcpar" => { validate_parameter("cbcpar", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqf" => { validate_parameter("alqf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alit" => { validate_parameter("alit", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flnqs" => { validate_parameter("flnqs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgs" => { validate_parameter("vgs", value, Some((0.0, "0.0")), true, Some((10.0, "10.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "f1vg" => { validate_finite_parameter("f1vg", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetact" => { validate_finite_parameter("zetact", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetabet" => { validate_finite_parameter("zetabet", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgbe" => { validate_finite_parameter("dvgbe", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetavgbe" => { validate_finite_parameter("zetavgbe", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alt0" => { validate_finite_parameter("alt0", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kt0" => { validate_finite_parameter("kt0", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaci" => { validate_finite_parameter("zetaci", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alvs" => { validate_finite_parameter("alvs", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alces" => { validate_finite_parameter("alces", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aldck" => { validate_finite_parameter("aldck", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbi" => { validate_finite_parameter("zetarbi", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarbx" => { validate_finite_parameter("zetarbx", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarcx" => { validate_finite_parameter("zetarcx", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetare" => { validate_finite_parameter("zetare", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaiqf" => { validate_finite_parameter("zetaiqf", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flteft" => { validate_parameter("flteft", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaver" => { validate_finite_parameter("zetaver", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetaiqfh" => { validate_finite_parameter("zetaiqfh", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alfav" => { validate_finite_parameter("alfav", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alqav" => { validate_finite_parameter("alqav", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aliqfh" => { validate_finite_parameter("aliqfh", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kiqfh" => { validate_finite_parameter("kiqfh", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flsh" => { validate_parameter("flsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "zetarth" => { validate_finite_parameter("zetarth", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alrth" => { validate_parameter("alrth", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, Some((1000000.0, "1000000.0")), true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), true, Some((600.0, "600.0")), false, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dt" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dt", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
        let v27: f64 = (v24 * 1.3806226e-23);
        self.scalar_v27 = v27;
        let v29: f64 = (v27 / 1.602176462e-19);
        self.scalar_v29 = v29;
        let v30: f64 = p.p88;
        self.scalar_v30 = v30;
        let v31: f64 = (v24 * p.p88);
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
        let v41: f64 = (p.p78 + p.p79);
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
        let v88: f64 = p.p35;
        self.scalar_v88 = v88;
        let v89: f64 = (0.5 * p.p35);
        self.scalar_v89 = v89;
        let v90: f64 = (v89 / v29);
        self.scalar_v90 = v90;
        let v92: f64 = (v29 * 2.0);
        self.scalar_v92 = v92;
        let v93: f64 = ((v90) as f64).exp();
        self.scalar_v93 = v93;
        let v94: f64 = (-v90);
        self.scalar_v94 = v94;
        let v95: f64 = ((v94) as f64).exp();
        self.scalar_v95 = v95;
        let v96: f64 = (v93 - v95);
        self.scalar_v96 = v96;
        let v97: f64 = ((v96) as f64).ln();
        self.scalar_v97 = v97;
        let v98: f64 = (v92 * v97);
        self.scalar_v98 = v98;
        let v119: f64 = p.p36;
        self.scalar_v119 = v119;
        let v125: f64 = p.p37;
        self.scalar_v125 = v125;
        let v128: f64 = p.p38;
        self.scalar_v128 = v128;
        let v129: f64 = (0.5 * p.p38);
        self.scalar_v129 = v129;
        let v130: f64 = (v129 / v29);
        self.scalar_v130 = v130;
        let v131: f64 = ((v130) as f64).exp();
        self.scalar_v131 = v131;
        let v132: f64 = (-v130);
        self.scalar_v132 = v132;
        let v133: f64 = ((v132) as f64).exp();
        self.scalar_v133 = v133;
        let v134: f64 = (v131 - v133);
        self.scalar_v134 = v134;
        let v135: f64 = ((v134) as f64).ln();
        self.scalar_v135 = v135;
        let v136: f64 = (v92 * v135);
        self.scalar_v136 = v136;
        let v151: f64 = p.p39;
        self.scalar_v151 = v151;
        let v157: f64 = p.p40;
        self.scalar_v157 = v157;
        let v160: f64 = p.p15;
        self.scalar_v160 = v160;
        let v166: f64 = p.p17;
        self.scalar_v166 = v166;
        let v167: f64 = (0.5 * v47);
        self.scalar_v167 = v167;
        let v169: f64 = (0.5 * v36);
        self.scalar_v169 = v169;
        let v174: f64 = p.p42;
        self.scalar_v174 = v174;
        let v175: f64 = (0.5 * p.p42);
        self.scalar_v175 = v175;
        let v176: f64 = (v175 / v29);
        self.scalar_v176 = v176;
        let v177: f64 = ((v176) as f64).exp();
        self.scalar_v177 = v177;
        let v178: f64 = (-v176);
        self.scalar_v178 = v178;
        let v179: f64 = ((v178) as f64).exp();
        self.scalar_v179 = v179;
        let v180: f64 = (v177 - v179);
        self.scalar_v180 = v180;
        let v181: f64 = ((v180) as f64).ln();
        self.scalar_v181 = v181;
        let v182: f64 = (v92 * v181);
        self.scalar_v182 = v182;
        let v198: f64 = p.p43;
        self.scalar_v198 = v198;
        let v204: f64 = p.p19;
        self.scalar_v204 = v204;
        let v210: f64 = p.p1;
        self.scalar_v210 = v210;
        let v216: f64 = p.p9;
        self.scalar_v216 = v216;
        let v217: f64 = p.p95;
        self.scalar_v217 = v217;
        let v219: f64 = p.p83;
        self.scalar_v219 = v219;
        let v224: f64 = p.p62;
        self.scalar_v224 = v224;
        let v225: f64 = (p.p87 - v31);
        self.scalar_v225 = v225;
        let v229: f64 = p.p61;
        self.scalar_v229 = v229;
        let v234: f64 = p.p64;
        self.scalar_v234 = v234;
        let v235: f64 = p.p89;
        self.scalar_v235 = v235;
        let v239: f64 = p.p65;
        self.scalar_v239 = v239;
        let v240: bool = (p.p65 > 0.0);
        self.scalar_v240 = v240;
        let v241: f64 = p.p90;
        self.scalar_v241 = v241;
        let v247: bool = (!v240);
        self.scalar_v247 = v247;
        let v250: f64 = p.p54;
        self.scalar_v250 = v250;
        let v251: f64 = p.p85;
        self.scalar_v251 = v251;
        let v254: f64 = p.p86;
        self.scalar_v254 = v254;
        let v259: f64 = p.p96;
        self.scalar_v259 = v259;
        let v260: bool = (1.0 == p.p96);
        self.scalar_v260 = v260;
        let v261: f64 = p.p57;
        self.scalar_v261 = v261;
        let v268: bool = (!v260);
        self.scalar_v268 = v268;
        let v270: f64 = p.p59;
        self.scalar_v270 = v270;
        let v271: f64 = (p.p87 - 1.0);
        self.scalar_v271 = v271;
        let v275: bool = (1.0 == v68);
        self.scalar_v275 = v275;
        let v276: f64 = p.p99;
        self.scalar_v276 = v276;
        let v281: f64 = p.p22;
        self.scalar_v281 = v281;
        let v282: f64 = p.p100;
        self.scalar_v282 = v282;
        let v287: bool = (!v275);
        self.scalar_v287 = v287;
        let v290: f64 = p.p23;
        self.scalar_v290 = v290;
        let v291: f64 = p.p91;
        self.scalar_v291 = v291;
        let v295: f64 = p.p46;
        self.scalar_v295 = v295;
        let v296: f64 = (0.5 * p.p46);
        self.scalar_v296 = v296;
        let v297: f64 = (v296 / v29);
        self.scalar_v297 = v297;
        let v298: f64 = ((v297) as f64).exp();
        self.scalar_v298 = v298;
        let v299: f64 = (-v297);
        self.scalar_v299 = v299;
        let v300: f64 = ((v299) as f64).exp();
        self.scalar_v300 = v300;
        let v301: f64 = (v298 - v300);
        self.scalar_v301 = v301;
        let v302: f64 = ((v301) as f64).ln();
        self.scalar_v302 = v302;
        let v303: f64 = (v92 * v302);
        self.scalar_v303 = v303;
        let v318: f64 = p.p45;
        self.scalar_v318 = v318;
        let v319: f64 = p.p47;
        self.scalar_v319 = v319;
        let v325: f64 = p.p51;
        self.scalar_v325 = v325;
        let v326: f64 = (0.5 * p.p51);
        self.scalar_v326 = v326;
        let v327: f64 = (v326 / v29);
        self.scalar_v327 = v327;
        let v328: f64 = ((v327) as f64).exp();
        self.scalar_v328 = v328;
        let v329: f64 = (-v327);
        self.scalar_v329 = v329;
        let v330: f64 = ((v329) as f64).exp();
        self.scalar_v330 = v330;
        let v331: f64 = (v328 - v330);
        self.scalar_v331 = v331;
        let v332: f64 = ((v331) as f64).ln();
        self.scalar_v332 = v332;
        let v333: f64 = (v92 * v332);
        self.scalar_v333 = v333;
        let v349: f64 = p.p50;
        self.scalar_v349 = v349;
        let v350: f64 = p.p52;
        self.scalar_v350 = v350;
        let v356: f64 = p.p32;
        self.scalar_v356 = v356;
        let v362: f64 = p.p30;
        self.scalar_v362 = v362;
        let v366: f64 = p.p7;
        self.scalar_v366 = v366;
        let v367: f64 = p.p97;
        self.scalar_v367 = v367;
        let v371: f64 = p.p6;
        self.scalar_v371 = v371;
        let v373: f64 = p.p84;
        self.scalar_v373 = v373;
        let v380: f64 = p.p0;
        self.scalar_v380 = v380;
        let v382: bool = (p.p0 <= 200.0);
        self.scalar_v382 = v382;
        let v383: f64 = p.p101;
        self.scalar_v383 = v383;
        let v384: f64 = p.p102;
        self.scalar_v384 = v384;
        let v390: bool = (!v382);
        self.scalar_v390 = v390;
        let v391: f64 = p.p98;
        self.scalar_v391 = v391;
        let v395: f64 = p.p12;
        self.scalar_v395 = v395;
        let v397: f64 = p.p13;
        self.scalar_v397 = v397;
        let v401: f64 = p.p14;
        self.scalar_v401 = v401;
        let v402: f64 = p.p29;
        self.scalar_v402 = v402;
        let v403: f64 = p.p93;
        self.scalar_v403 = v403;
        let v407: f64 = p.p26;
        self.scalar_v407 = v407;
        let v408: f64 = p.p92;
        self.scalar_v408 = v408;
        let v412: f64 = p.p28;
        self.scalar_v412 = v412;
        let v413: f64 = p.p94;
        self.scalar_v413 = v413;
        let v417: f64 = p.p104;
        self.scalar_v417 = v417;
        let v418: f64 = p.p105;
        self.scalar_v418 = v418;
        let v422: f64 = p.p106;
        self.scalar_v422 = v422;
        let v426: f64 = p.p103;
        self.scalar_v426 = v426;
        let v427: bool = (0.0 != p.p103);
        self.scalar_v427 = v427;
        let v428: f64 = p.p111;
        self.scalar_v428 = v428;
        let v429: bool = (p.p104 >= p.p111);
        self.scalar_v429 = v429;
        let v430: bool = (v427 && v429);
        self.scalar_v430 = v430;
        let v456: f64 = (if v430 { v90 } else { v327 });
        self.scalar_v456 = v456;
        let v457: f64 = ((v456) as f64).exp();
        self.scalar_v457 = v457;
        let v458: f64 = (-v456);
        self.scalar_v458 = v458;
        let v459: f64 = ((v458) as f64).exp();
        self.scalar_v459 = v459;
        let v460: f64 = (v457 - v459);
        self.scalar_v460 = v460;
        let v461: f64 = ((v460) as f64).ln();
        self.scalar_v461 = v461;
        let v462: f64 = (v92 * v461);
        self.scalar_v462 = v462;
        let v463: f64 = (if v430 { v462 } else { v333 });
        self.scalar_v463 = v463;
        let v494: f64 = (if v430 { v130 } else { v456 });
        self.scalar_v494 = v494;
        let v495: f64 = ((v494) as f64).exp();
        self.scalar_v495 = v495;
        let v496: f64 = (-v494);
        self.scalar_v496 = v496;
        let v497: f64 = ((v496) as f64).exp();
        self.scalar_v497 = v497;
        let v498: f64 = (v495 - v497);
        self.scalar_v498 = v498;
        let v499: f64 = ((v498) as f64).ln();
        self.scalar_v499 = v499;
        let v500: f64 = (v92 * v499);
        self.scalar_v500 = v500;
        let v501: f64 = (if v430 { v500 } else { v463 });
        self.scalar_v501 = v501;
        let v539: f64 = (if v430 { v176 } else { v494 });
        self.scalar_v539 = v539;
        let v540: f64 = ((v539) as f64).exp();
        self.scalar_v540 = v540;
        let v541: f64 = (-v539);
        self.scalar_v541 = v541;
        let v542: f64 = ((v541) as f64).exp();
        self.scalar_v542 = v542;
        let v543: f64 = (v540 - v542);
        self.scalar_v543 = v543;
        let v544: f64 = ((v543) as f64).ln();
        self.scalar_v544 = v544;
        let v545: f64 = (v92 * v544);
        self.scalar_v545 = v545;
        let v546: f64 = (if v430 { v545 } else { v501 });
        self.scalar_v546 = v546;
        let v602: bool = (v240 && v430);
        self.scalar_v602 = v602;
        let v608: bool = (v247 && v430);
        self.scalar_v608 = v608;
        let v618: bool = (v260 && v430);
        self.scalar_v618 = v618;
        let v625: bool = (v268 && v430);
        self.scalar_v625 = v625;
        let v631: bool = (v275 && v430);
        self.scalar_v631 = v631;
        let v640: bool = (v287 && v430);
        self.scalar_v640 = v640;
        let v647: f64 = (if v430 { v297 } else { v539 });
        self.scalar_v647 = v647;
        let v648: f64 = ((v647) as f64).exp();
        self.scalar_v648 = v648;
        let v649: f64 = (-v647);
        self.scalar_v649 = v649;
        let v650: f64 = ((v649) as f64).exp();
        self.scalar_v650 = v650;
        let v651: f64 = (v648 - v650);
        self.scalar_v651 = v651;
        let v652: f64 = ((v651) as f64).ln();
        self.scalar_v652 = v652;
        let v653: f64 = (v92 * v652);
        self.scalar_v653 = v653;
        let v654: f64 = (if v430 { v653 } else { v546 });
        self.scalar_v654 = v654;
        let v677: f64 = (if v430 { v327 } else { v647 });
        self.scalar_v677 = v677;
        let v678: f64 = ((v677) as f64).exp();
        self.scalar_v678 = v678;
        let v679: f64 = (-v677);
        self.scalar_v679 = v679;
        let v680: f64 = ((v679) as f64).exp();
        self.scalar_v680 = v680;
        let v681: f64 = (v678 - v680);
        self.scalar_v681 = v681;
        let v682: f64 = ((v681) as f64).ln();
        self.scalar_v682 = v682;
        let v683: f64 = (v92 * v682);
        self.scalar_v683 = v683;
        let v684: f64 = (if v430 { v683 } else { v654 });
        self.scalar_v684 = v684;
        let v730: bool = (v382 && v430);
        self.scalar_v730 = v730;
        let v736: bool = (v390 && v430);
        self.scalar_v736 = v736;
        let v746: f64 = (if v430 { p.p14 } else { p.p14 });
        self.scalar_v746 = v746;
        let v768: f64 = p.p49;
        self.scalar_v768 = v768;
        let v772: f64 = (1.0 - p.p49);
        self.scalar_v772 = v772;
        let v775: f64 = p.p44;
        self.scalar_v775 = v775;
        let v776: bool = (p.p44 < 100.0);
        self.scalar_v776 = v776;
        let v780: f64 = (p.p43 / 4.0);
        self.scalar_v780 = v780;
        let v786: f64 = (-0.8754687373538999 / p.p43);
        self.scalar_v786 = v786;
        let v787: f64 = ((v786) as f64).exp();
        self.scalar_v787 = v787;
        let v788: f64 = (1.0 - v787);
        self.scalar_v788 = v788;
        let v858: f64 = (1.0 - p.p43);
        self.scalar_v858 = v858;
        let v862: f64 = (-p.p43);
        self.scalar_v862 = v862;
        let v906: bool = (!v776);
        self.scalar_v906 = v906;
        let v951: f64 = p.p48;
        self.scalar_v951 = v951;
        let v952: bool = (p.p48 < 100.0);
        self.scalar_v952 = v952;
        let v956: f64 = (p.p47 / 4.0);
        self.scalar_v956 = v956;
        let v960: f64 = (-0.8754687373538999 / p.p47);
        self.scalar_v960 = v960;
        let v961: f64 = ((v960) as f64).exp();
        self.scalar_v961 = v961;
        let v962: f64 = (1.0 - v961);
        self.scalar_v962 = v962;
        let v1029: f64 = (1.0 - p.p47);
        self.scalar_v1029 = v1029;
        let v1033: f64 = (-p.p47);
        self.scalar_v1033 = v1033;
        let v1077: bool = (!v952);
        self.scalar_v1077 = v1077;
        let v1465: f64 = p.p67;
        self.scalar_v1465 = v1465;
        let v1475: f64 = p.p63;
        self.scalar_v1475 = v1475;
        let v1478: f64 = p.p66;
        self.scalar_v1478 = v1478;
        let v1521: f64 = (-p.p36);
        self.scalar_v1521 = v1521;
        let v1526: f64 = (1.0 - p.p36);
        self.scalar_v1526 = v1526;
        let v1569: f64 = (-p.p39);
        self.scalar_v1569 = v1569;
        let v1574: f64 = (1.0 - p.p39);
        self.scalar_v1574 = v1574;
        let v1592: f64 = (if v382 { p.p39 } else { 0.0 });
        self.scalar_v1592 = v1592;
        let v1595: f64 = (if v390 { p.p36 } else { v1592 });
        self.scalar_v1595 = v1595;
        let v1596: bool = (0.0 == p.p7);
        self.scalar_v1596 = v1596;
        let v1597: f64 = (if v1596 { 1.0 } else { 0.0 });
        self.scalar_v1597 = v1597;
        let v1598: bool = (!v1596);
        self.scalar_v1598 = v1598;
        let v1599: f64 = p.p8;
        self.scalar_v1599 = v1599;
        let v1637: f64 = p.p5;
        self.scalar_v1637 = v1637;
        let v1651: f64 = p.p55;
        self.scalar_v1651 = v1651;
        let v1655: f64 = p.p56;
        self.scalar_v1655 = v1655;
        let v1660: f64 = p.p10;
        self.scalar_v1660 = v1660;
        let v1661: bool = (1.0 == p.p10);
        self.scalar_v1661 = v1661;
        let v1668: bool = (!v1661);
        self.scalar_v1668 = v1668;
        let v1670: f64 = p.p11;
        self.scalar_v1670 = v1670;
        let v1671: f64 = p.p3;
        self.scalar_v1671 = v1671;
        let v1684: f64 = p.p4;
        self.scalar_v1684 = v1684;
        let v1697: bool = (0.0 != p.p13);
        self.scalar_v1697 = v1697;
        let v1715: bool = (!v1697);
        self.scalar_v1715 = v1715;
        let v1729: f64 = (1.0 + v746);
        self.scalar_v1729 = v1729;
        let v1751: f64 = p.p2;
        self.scalar_v1751 = v1751;
        let v1752: bool = (0.0 == p.p2);
        self.scalar_v1752 = v1752;
        let v1753: bool = (v1697 && v1752);
        self.scalar_v1753 = v1753;
        let v1759: bool = (v1715 && v1752);
        self.scalar_v1759 = v1759;
        let v1765: bool = (!v1752);
        self.scalar_v1765 = v1765;
        let v1767: f64 = (if v1765 { 0.3333333333333333 } else { 0.0 });
        self.scalar_v1767 = v1767;
        let v1772: bool = (p.p9 == 1000000.0);
        self.scalar_v1772 = v1772;
        let v1773: bool = (p.p12 == 1000000.0);
        self.scalar_v1773 = v1773;
        let v1774: bool = (v1772 && v1773);
        self.scalar_v1774 = v1774;
        let v1775: bool = (v1765 && v1774);
        self.scalar_v1775 = v1775;
        let v1776: f64 = (if v1775 { 0.0 } else { 0.0 });
        self.scalar_v1776 = v1776;
        let v1777: bool = (!v1774);
        self.scalar_v1777 = v1777;
        let v1778: bool = (v1765 && v1777);
        self.scalar_v1778 = v1778;
        let v1907: f64 = p.p60;
        self.scalar_v1907 = v1907;
        let v1911: f64 = (1.0 + p.p60);
        self.scalar_v1911 = v1911;
        let v1912: f64 = ((v1911) as f64).sqrt();
        self.scalar_v1912 = v1912;
        let v1913: f64 = (1.0 + v1912);
        self.scalar_v1913 = v1913;
        let v1918: f64 = p.p58;
        self.scalar_v1918 = v1918;
        let v1925: f64 = (1.0 + p.p58);
        self.scalar_v1925 = v1925;
        let v1929: f64 = p.p68;
        self.scalar_v1929 = v1929;
        let v1931: bool = (p.p15 > 0.0);
        self.scalar_v1931 = v1931;
        let v1932: f64 = p.p16;
        self.scalar_v1932 = v1932;
        let v1950: bool = (!v1931);
        self.scalar_v1950 = v1950;
        let v1952: bool = (p.p17 > 0.0);
        self.scalar_v1952 = v1952;
        let v1953: f64 = p.p18;
        self.scalar_v1953 = v1953;
        let v1971: bool = (!v1952);
        self.scalar_v1971 = v1971;
        let v1974: bool = (p.p19 > 0.0);
        self.scalar_v1974 = v1974;
        let v1975: f64 = p.p20;
        self.scalar_v1975 = v1975;
        let v1993: bool = (!v1974);
        self.scalar_v1993 = v1993;
        let v2179: f64 = p.p24;
        self.scalar_v2179 = v2179;
        let v2182: f64 = p.p25;
        self.scalar_v2182 = v2182;
        let v2200: f64 = p.p27;
        self.scalar_v2200 = v2200;
        let v2222: bool = (p.p30 > 0.0);
        self.scalar_v2222 = v2222;
        let v2223: f64 = p.p31;
        self.scalar_v2223 = v2223;
        let v2235: bool = (!v2222);
        self.scalar_v2235 = v2235;
        let v2237: bool = (p.p32 > 0.0);
        self.scalar_v2237 = v2237;
        let v2238: f64 = p.p33;
        self.scalar_v2238 = v2238;
        let v2256: bool = (!v2237);
        self.scalar_v2256 = v2256;
        let v2258: f64 = p.p53;
        self.scalar_v2258 = v2258;
        let v2259: bool = (p.p53 < 100.0);
        self.scalar_v2259 = v2259;
        let v2262: f64 = (p.p52 / 4.0);
        self.scalar_v2262 = v2262;
        let v2266: f64 = (-0.8754687373538999 / p.p52);
        self.scalar_v2266 = v2266;
        let v2267: f64 = ((v2266) as f64).exp();
        self.scalar_v2267 = v2267;
        let v2268: f64 = (1.0 - v2267);
        self.scalar_v2268 = v2268;
        let v2329: f64 = (1.0 - p.p52);
        self.scalar_v2329 = v2329;
        let v2360: bool = (!v2259);
        self.scalar_v2360 = v2360;
        let v2393: bool = (1.0 == p.p103);
        self.scalar_v2393 = v2393;
        let v2394: bool = (v429 && v2393);
        self.scalar_v2394 = v2394;
        let v2399: f64 = p.p73;
        self.scalar_v2399 = v2399;
        let v2400: bool = (0.0 != p.p73);
        self.scalar_v2400 = v2400;
        let v2401: bool = (0.0 != p.p54);
        self.scalar_v2401 = v2401;
        let v2402: bool = (v2400 && v2401);
        self.scalar_v2402 = v2402;
        let v2407: f64 = p.p71;
        self.scalar_v2407 = v2407;
        let v2415: f64 = p.p72;
        self.scalar_v2415 = v2415;
        let v2419: bool = (!v2402);
        self.scalar_v2419 = v2419;
        let v2425: f64 = p.p70;
        self.scalar_v2425 = v2425;
        let v2427: f64 = p.p69;
        self.scalar_v2427 = v2427;
        let v2444: bool = (p.p28 >= p.p111);
        self.scalar_v2444 = v2444;
        let v2445: bool = (p.p29 >= p.p111);
        self.scalar_v2445 = v2445;
        let v2446: bool = (p.p23 >= p.p111);
        self.scalar_v2446 = v2446;
        let v2447: bool = (p.p26 >= p.p111);
        self.scalar_v2447 = v2447;
        let v2448: bool = (v2446 || v2447);
        self.scalar_v2448 = v2448;
        let v2449: bool = (0.0 == p.p103);
        self.scalar_v2449 = v2449;
        let v2450: bool = (p.p104 < p.p111);
        self.scalar_v2450 = v2450;
        let v2451: bool = (v2449 || v2450);
        self.scalar_v2451 = v2451;
        let v2457: bool = (!v2444);
        self.scalar_v2457 = v2457;
        let v2458: f64 = (if v2457 { 0.0 } else { 0.0 });
        self.scalar_v2458 = v2458;
        let v2461: bool = (!v2445);
        self.scalar_v2461 = v2461;
        let v2462: f64 = (if v2461 { 0.0 } else { 0.0 });
        self.scalar_v2462 = v2462;
        let v2465: bool = (!v2448);
        self.scalar_v2465 = v2465;
        let v2466: f64 = (if v2465 { 0.0 } else { 0.0 });
        self.scalar_v2466 = v2466;
        let v2467: f64 = (if v2451 { 0.0 } else { 0.0 });
        self.scalar_v2467 = v2467;
        let v2468: bool = (!v2451);
        self.scalar_v2468 = v2468;
        let v2473: f64 = (-p.p110);
        self.scalar_v2473 = v2473;
        let v2474: f64 = (p.p110 - p.p110);
        self.scalar_v2474 = v2474;
        let v2475: f64 = (if v430 { 1.0 } else { 0.0 });
        self.scalar_v2475 = v2475;
        let v5056: f64 = (if v240 { p.p110 } else { 0.0 });
        self.scalar_v5056 = v5056;
        let v5057: f64 = (if v240 { v2473 } else { 0.0 });
        self.scalar_v5057 = v5057;
        let v5060: f64 = (if v247 { p.p110 } else { v5056 });
        self.scalar_v5060 = v5060;
        let v5061: f64 = (if v247 { v2474 } else { v5057 });
        self.scalar_v5061 = v5061;
        let v5062: f64 = (if v247 { v2473 } else { 0.0 });
        self.scalar_v5062 = v5062;
        let v7815: f64 = (if v275 { p.p110 } else { 0.0 });
        self.scalar_v7815 = v7815;
        let v7816: f64 = (if v275 { v2473 } else { 0.0 });
        self.scalar_v7816 = v7816;
        let v8825: f64 = (if v2402 { 1.0 } else { 0.0 });
        self.scalar_v8825 = v8825;
        let v8836: f64 = (if v2402 { v8825 } else { 0.0 });
        self.scalar_v8836 = v8836;
        let v8842: f64 = (p.p71 * v8825);
        self.scalar_v8842 = v8842;
        let v8848: f64 = (p.p54 * v8842);
        self.scalar_v8848 = v8848;
        let v8854: f64 = (if v2402 { v8848 } else { 0.0 });
        self.scalar_v8854 = v8854;
        let v8875: f64 = (p.p72 * v8825);
        self.scalar_v8875 = v8875;
        let v8881: f64 = (p.p54 * v8875);
        self.scalar_v8881 = v8881;
        let v8887: f64 = (if v2402 { v8881 } else { 0.0 });
        self.scalar_v8887 = v8887;
        let v8893: f64 = (if v2419 { 1.0 } else { v8836 });
        self.scalar_v8893 = v8893;
        let v8899: f64 = (if v2419 { 0.0 } else { v8854 });
        self.scalar_v8899 = v8899;
        let v8910: f64 = (if v2419 { 0.0 } else { v8887 });
        self.scalar_v8910 = v8910;
        let v8916: f64 = (p.p110 * p.p70);
        self.scalar_v8916 = v8916;
        let v8917: f64 = (p.p70 * v2473);
        self.scalar_v8917 = v8917;
        let v8918: f64 = (p.p110 * p.p69);
        self.scalar_v8918 = v8918;
        let v8919: f64 = (p.p69 * v2473);
        self.scalar_v8919 = v8919;
        let v8944: f64 = (p.p110 * v8916);
        self.scalar_v8944 = v8944;
        let v8945: f64 = (p.p110 * v8917);
        self.scalar_v8945 = v8945;
        let v8946: f64 = (p.p110 * v8918);
        self.scalar_v8946 = v8946;
        let v8947: f64 = (p.p110 * v8919);
        self.scalar_v8947 = v8947;
        let v8966: f64 = (p.p110 * v8825);
        self.scalar_v8966 = v8966;
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
        let v70: f64 = (temperature + self.scalar_v69);
        self.scalar_v70 = v70;
        let v73: bool = (self.scalar_v70 < 173.14999999999998);
        self.scalar_v73 = v73;
        let v74: f64 = (if self.scalar_v73 { 173.14999999999998 } else { self.scalar_v70 });
        self.scalar_v74 = v74;
        let v76: bool = (self.scalar_v74 > 600.0);
        self.scalar_v76 = v76;
        let v77: bool = (!self.scalar_v73);
        self.scalar_v77 = v77;
        let v78: bool = (self.scalar_v76 && self.scalar_v77);
        self.scalar_v78 = v78;
        let v79: f64 = (if self.scalar_v78 { 600.0 } else { self.scalar_v74 });
        self.scalar_v79 = v79;
        let v80: f64 = (1.3806226e-23 * self.scalar_v79);
        self.scalar_v80 = v80;
        let v81: f64 = (self.scalar_v80 / 1.602176462e-19);
        self.scalar_v81 = v81;
        let v82: f64 = (1.0 / self.scalar_v81);
        self.scalar_v82 = v82;
        let v83: f64 = (self.scalar_v79 - self.scalar_v24);
        self.scalar_v83 = v83;
        let v84: f64 = (self.scalar_v79 / self.scalar_v24);
        self.scalar_v84 = v84;
        let v85: f64 = ((self.scalar_v84) as f64).ln();
        self.scalar_v85 = v85;
        let v86: f64 = (self.scalar_v84 - 1.0);
        self.scalar_v86 = v86;
        let v87: f64 = (self.scalar_v82 * self.scalar_v86);
        self.scalar_v87 = v87;
        let v99: f64 = (self.scalar_v84 * self.scalar_v98);
        self.scalar_v99 = v99;
        let v100: f64 = (1.0 - self.scalar_v84);
        self.scalar_v100 = v100;
        let v101: f64 = (self.scalar_v36 * self.scalar_v100);
        self.scalar_v101 = v101;
        let v102: f64 = (self.scalar_v99 + self.scalar_v101);
        self.scalar_v102 = v102;
        let v103: f64 = (self.scalar_v47 * self.scalar_v81);
        self.scalar_v103 = v103;
        let v104: f64 = (self.scalar_v85 * self.scalar_v103);
        self.scalar_v104 = v104;
        let v105: f64 = (self.scalar_v102 - self.scalar_v104);
        self.scalar_v105 = v105;
        let v106: f64 = (self.scalar_v81 * 2.0);
        self.scalar_v106 = v106;
        let v108: f64 = (-self.scalar_v105);
        self.scalar_v108 = v108;
        let v109: f64 = (self.scalar_v82 * self.scalar_v108);
        self.scalar_v109 = v109;
        let v110: f64 = ((self.scalar_v109) as f64).exp();
        self.scalar_v110 = v110;
        let v111: f64 = (4.0 * self.scalar_v110);
        self.scalar_v111 = v111;
        let v112: f64 = (1.0 + self.scalar_v111);
        self.scalar_v112 = v112;
        let v113: f64 = ((self.scalar_v112) as f64).sqrt();
        self.scalar_v113 = v113;
        let v114: f64 = (1.0 + self.scalar_v113);
        self.scalar_v114 = v114;
        let v115: f64 = (0.5 * self.scalar_v114);
        self.scalar_v115 = v115;
        let v116: f64 = ((self.scalar_v115) as f64).ln();
        self.scalar_v116 = v116;
        let v117: f64 = (self.scalar_v106 * self.scalar_v116);
        self.scalar_v117 = v117;
        let v118: f64 = (self.scalar_v105 + self.scalar_v117);
        self.scalar_v118 = v118;
        let v120: f64 = (self.scalar_v88 / self.scalar_v118);
        self.scalar_v120 = v120;
        let v121: f64 = ((self.scalar_v120) as f64).ln();
        self.scalar_v121 = v121;
        let v122: f64 = (self.scalar_v119 * self.scalar_v121);
        self.scalar_v122 = v122;
        let v123: f64 = ((self.scalar_v122) as f64).exp();
        self.scalar_v123 = v123;
        let v124: f64 = (self.scalar_v59 * self.scalar_v123);
        self.scalar_v124 = v124;
        let v126: f64 = (self.scalar_v118 * self.scalar_v125);
        self.scalar_v126 = v126;
        let v127: f64 = (self.scalar_v126 / self.scalar_v88);
        self.scalar_v127 = v127;
        let v137: f64 = (self.scalar_v84 * self.scalar_v136);
        self.scalar_v137 = v137;
        let v138: f64 = (self.scalar_v101 + self.scalar_v137);
        self.scalar_v138 = v138;
        let v139: f64 = (self.scalar_v138 - self.scalar_v104);
        self.scalar_v139 = v139;
        let v140: f64 = (-self.scalar_v139);
        self.scalar_v140 = v140;
        let v141: f64 = (self.scalar_v82 * self.scalar_v140);
        self.scalar_v141 = v141;
        let v142: f64 = ((self.scalar_v141) as f64).exp();
        self.scalar_v142 = v142;
        let v143: f64 = (4.0 * self.scalar_v142);
        self.scalar_v143 = v143;
        let v144: f64 = (1.0 + self.scalar_v143);
        self.scalar_v144 = v144;
        let v145: f64 = ((self.scalar_v144) as f64).sqrt();
        self.scalar_v145 = v145;
        let v146: f64 = (1.0 + self.scalar_v145);
        self.scalar_v146 = v146;
        let v147: f64 = (0.5 * self.scalar_v146);
        self.scalar_v147 = v147;
        let v148: f64 = ((self.scalar_v147) as f64).ln();
        self.scalar_v148 = v148;
        let v149: f64 = (self.scalar_v106 * self.scalar_v148);
        self.scalar_v149 = v149;
        let v150: f64 = (self.scalar_v139 + self.scalar_v149);
        self.scalar_v150 = v150;
        let v152: f64 = (self.scalar_v128 / self.scalar_v150);
        self.scalar_v152 = v152;
        let v153: f64 = ((self.scalar_v152) as f64).ln();
        self.scalar_v153 = v153;
        let v154: f64 = (self.scalar_v151 * self.scalar_v153);
        self.scalar_v154 = v154;
        let v155: f64 = ((self.scalar_v154) as f64).exp();
        self.scalar_v155 = v155;
        let v156: f64 = (self.scalar_v59 * self.scalar_v155);
        self.scalar_v156 = v156;
        let v158: f64 = (self.scalar_v150 * self.scalar_v157);
        self.scalar_v158 = v158;
        let v159: f64 = (self.scalar_v158 / self.scalar_v128);
        self.scalar_v159 = v159;
        let v161: f64 = (self.scalar_v54 * self.scalar_v85);
        self.scalar_v161 = v161;
        let v162: f64 = (self.scalar_v34 * self.scalar_v87);
        self.scalar_v162 = v162;
        let v163: f64 = (self.scalar_v161 + self.scalar_v162);
        self.scalar_v163 = v163;
        let v164: f64 = ((self.scalar_v163) as f64).exp();
        self.scalar_v164 = v164;
        let v165: f64 = (self.scalar_v160 * self.scalar_v164);
        self.scalar_v165 = v165;
        let v168: f64 = (self.scalar_v85 * self.scalar_v167);
        self.scalar_v168 = v168;
        let v170: f64 = (self.scalar_v87 * self.scalar_v169);
        self.scalar_v170 = v170;
        let v171: f64 = (self.scalar_v168 + self.scalar_v170);
        self.scalar_v171 = v171;
        let v172: f64 = ((self.scalar_v171) as f64).exp();
        self.scalar_v172 = v172;
        let v173: f64 = (self.scalar_v166 * self.scalar_v172);
        self.scalar_v173 = v173;
        let v183: f64 = (self.scalar_v84 * self.scalar_v182);
        self.scalar_v183 = v183;
        let v184: f64 = (self.scalar_v39 * self.scalar_v100);
        self.scalar_v184 = v184;
        let v185: f64 = (self.scalar_v183 + self.scalar_v184);
        self.scalar_v185 = v185;
        let v186: f64 = (self.scalar_v185 - self.scalar_v104);
        self.scalar_v186 = v186;
        let v187: f64 = (-self.scalar_v186);
        self.scalar_v187 = v187;
        let v188: f64 = (self.scalar_v82 * self.scalar_v187);
        self.scalar_v188 = v188;
        let v189: f64 = ((self.scalar_v188) as f64).exp();
        self.scalar_v189 = v189;
        let v190: f64 = (4.0 * self.scalar_v189);
        self.scalar_v190 = v190;
        let v191: f64 = (1.0 + self.scalar_v190);
        self.scalar_v191 = v191;
        let v192: f64 = ((self.scalar_v191) as f64).sqrt();
        self.scalar_v192 = v192;
        let v193: f64 = (1.0 + self.scalar_v192);
        self.scalar_v193 = v193;
        let v194: f64 = (0.5 * self.scalar_v193);
        self.scalar_v194 = v194;
        let v195: f64 = ((self.scalar_v194) as f64).ln();
        self.scalar_v195 = v195;
        let v196: f64 = (self.scalar_v106 * self.scalar_v195);
        self.scalar_v196 = v196;
        let v197: f64 = (self.scalar_v186 + self.scalar_v196);
        self.scalar_v197 = v197;
        let v199: f64 = (self.scalar_v174 / self.scalar_v197);
        self.scalar_v199 = v199;
        let v200: f64 = ((self.scalar_v199) as f64).ln();
        self.scalar_v200 = v200;
        let v201: f64 = (self.scalar_v198 * self.scalar_v200);
        self.scalar_v201 = v201;
        let v202: f64 = ((self.scalar_v201) as f64).exp();
        self.scalar_v202 = v202;
        let v203: f64 = (self.scalar_v63 * self.scalar_v202);
        self.scalar_v203 = v203;
        let v205: f64 = (self.scalar_v51 * self.scalar_v85);
        self.scalar_v205 = v205;
        let v206: f64 = (self.scalar_v37 * self.scalar_v87);
        self.scalar_v206 = v206;
        let v207: f64 = (self.scalar_v205 + self.scalar_v206);
        self.scalar_v207 = v207;
        let v208: f64 = ((self.scalar_v207) as f64).exp();
        self.scalar_v208 = v208;
        let v209: f64 = (self.scalar_v204 * self.scalar_v208);
        self.scalar_v209 = v209;
        let v211: f64 = (self.scalar_v55 * self.scalar_v85);
        self.scalar_v211 = v211;
        let v212: f64 = (self.scalar_v33 * self.scalar_v87);
        self.scalar_v212 = v212;
        let v213: f64 = (self.scalar_v211 + self.scalar_v212);
        self.scalar_v213 = v213;
        let v214: f64 = ((self.scalar_v213) as f64).exp();
        self.scalar_v214 = v214;
        let v215: f64 = (self.scalar_v210 * self.scalar_v214);
        self.scalar_v215 = v215;
        let v218: f64 = (self.scalar_v85 * self.scalar_v217);
        self.scalar_v218 = v218;
        let v220: f64 = (self.scalar_v87 * self.scalar_v219);
        self.scalar_v220 = v220;
        let v221: f64 = (self.scalar_v218 - self.scalar_v220);
        self.scalar_v221 = v221;
        let v222: f64 = ((self.scalar_v221) as f64).exp();
        self.scalar_v222 = v222;
        let v223: f64 = (self.scalar_v216 * self.scalar_v222);
        self.scalar_v223 = v223;
        let v226: f64 = (self.scalar_v85 * self.scalar_v225);
        self.scalar_v226 = v226;
        let v227: f64 = ((self.scalar_v226) as f64).exp();
        self.scalar_v227 = v227;
        let v228: f64 = (self.scalar_v224 * self.scalar_v227);
        self.scalar_v228 = v228;
        let v230: f64 = (self.scalar_v50 * self.scalar_v85);
        self.scalar_v230 = v230;
        let v231: f64 = ((self.scalar_v230) as f64).exp();
        self.scalar_v231 = v231;
        let v232: f64 = (self.scalar_v229 * self.scalar_v231);
        self.scalar_v232 = v232;
        let v233: f64 = (1.0 / self.scalar_v232);
        self.scalar_v233 = v233;
        let v236: f64 = (self.scalar_v83 * self.scalar_v235);
        self.scalar_v236 = v236;
        let v237: f64 = (1.0 + self.scalar_v236);
        self.scalar_v237 = v237;
        let v238: f64 = (self.scalar_v234 * self.scalar_v237);
        self.scalar_v238 = v238;
        let v242: f64 = (self.scalar_v83 * self.scalar_v241);
        self.scalar_v242 = v242;
        let v243: f64 = (1.0 - self.scalar_v242);
        self.scalar_v243 = v243;
        let v244: f64 = (self.scalar_v239 * self.scalar_v243);
        self.scalar_v244 = v244;
        let v245: f64 = (if self.scalar_v240 { self.scalar_v244 } else { 0.0 });
        self.scalar_v245 = v245;
        let v246: f64 = (if self.scalar_v240 { self.scalar_v234 } else { self.scalar_v238 });
        self.scalar_v246 = v246;
        let v248: f64 = (if self.scalar_v247 { self.scalar_v238 } else { self.scalar_v246 });
        self.scalar_v248 = v248;
        let v249: f64 = (if self.scalar_v247 { self.scalar_v239 } else { self.scalar_v245 });
        self.scalar_v249 = v249;
        let v252: f64 = (self.scalar_v83 * self.scalar_v251);
        self.scalar_v252 = v252;
        let v253: f64 = (1.0 + self.scalar_v252);
        self.scalar_v253 = v253;
        let v255: f64 = (self.scalar_v83 * self.scalar_v254);
        self.scalar_v255 = v255;
        let v256: f64 = (self.scalar_v83 * self.scalar_v255);
        self.scalar_v256 = v256;
        let v257: f64 = (self.scalar_v253 + self.scalar_v256);
        self.scalar_v257 = v257;
        let v258: f64 = (self.scalar_v250 * self.scalar_v257);
        self.scalar_v258 = v258;
        let v262: f64 = (self.scalar_v57 * self.scalar_v85);
        self.scalar_v262 = v262;
        let v263: f64 = (self.scalar_v58 * self.scalar_v87);
        self.scalar_v263 = v263;
        let v264: f64 = (self.scalar_v262 - self.scalar_v263);
        self.scalar_v264 = v264;
        let v265: f64 = ((self.scalar_v264) as f64).exp();
        self.scalar_v265 = v265;
        let v266: f64 = (self.scalar_v261 * self.scalar_v265);
        self.scalar_v266 = v266;
        let v267: f64 = (if self.scalar_v260 { self.scalar_v266 } else { 0.0 });
        self.scalar_v267 = v267;
        let v269: f64 = (if self.scalar_v268 { self.scalar_v261 } else { self.scalar_v267 });
        self.scalar_v269 = v269;
        let v272: f64 = (self.scalar_v85 * self.scalar_v271);
        self.scalar_v272 = v272;
        let v273: f64 = ((self.scalar_v272) as f64).exp();
        self.scalar_v273 = v273;
        let v274: f64 = (self.scalar_v270 * self.scalar_v273);
        self.scalar_v274 = v274;
        let v277: f64 = (self.scalar_v83 * self.scalar_v276);
        self.scalar_v277 = v277;
        let v278: f64 = ((self.scalar_v277) as f64).exp();
        self.scalar_v278 = v278;
        let v279: f64 = (self.scalar_v61 * self.scalar_v278);
        self.scalar_v279 = v279;
        let v280: f64 = (if self.scalar_v275 { self.scalar_v279 } else { 0.0 });
        self.scalar_v280 = v280;
        let v283: f64 = (self.scalar_v83 * self.scalar_v282);
        self.scalar_v283 = v283;
        let v284: f64 = ((self.scalar_v283) as f64).exp();
        self.scalar_v284 = v284;
        let v285: f64 = (self.scalar_v281 * self.scalar_v284);
        self.scalar_v285 = v285;
        let v286: f64 = (if self.scalar_v275 { self.scalar_v285 } else { 0.0 });
        self.scalar_v286 = v286;
        let v288: f64 = (if self.scalar_v287 { self.scalar_v61 } else { self.scalar_v280 });
        self.scalar_v288 = v288;
        let v289: f64 = (if self.scalar_v287 { self.scalar_v281 } else { self.scalar_v286 });
        self.scalar_v289 = v289;
        let v292: f64 = (self.scalar_v85 * self.scalar_v291);
        self.scalar_v292 = v292;
        let v293: f64 = ((self.scalar_v292) as f64).exp();
        self.scalar_v293 = v293;
        let v294: f64 = (self.scalar_v290 * self.scalar_v293);
        self.scalar_v294 = v294;
        let v304: f64 = (self.scalar_v84 * self.scalar_v303);
        self.scalar_v304 = v304;
        let v305: f64 = (self.scalar_v184 + self.scalar_v304);
        self.scalar_v305 = v305;
        let v306: f64 = (self.scalar_v305 - self.scalar_v104);
        self.scalar_v306 = v306;
        let v307: f64 = (-self.scalar_v306);
        self.scalar_v307 = v307;
        let v308: f64 = (self.scalar_v82 * self.scalar_v307);
        self.scalar_v308 = v308;
        let v309: f64 = ((self.scalar_v308) as f64).exp();
        self.scalar_v309 = v309;
        let v310: f64 = (4.0 * self.scalar_v309);
        self.scalar_v310 = v310;
        let v311: f64 = (1.0 + self.scalar_v310);
        self.scalar_v311 = v311;
        let v312: f64 = ((self.scalar_v311) as f64).sqrt();
        self.scalar_v312 = v312;
        let v313: f64 = (1.0 + self.scalar_v312);
        self.scalar_v313 = v313;
        let v314: f64 = (0.5 * self.scalar_v313);
        self.scalar_v314 = v314;
        let v315: f64 = ((self.scalar_v314) as f64).ln();
        self.scalar_v315 = v315;
        let v316: f64 = (self.scalar_v106 * self.scalar_v315);
        self.scalar_v316 = v316;
        let v317: f64 = (self.scalar_v306 + self.scalar_v316);
        self.scalar_v317 = v317;
        let v320: f64 = (self.scalar_v295 / self.scalar_v317);
        self.scalar_v320 = v320;
        let v321: f64 = ((self.scalar_v320) as f64).ln();
        self.scalar_v321 = v321;
        let v322: f64 = (self.scalar_v319 * self.scalar_v321);
        self.scalar_v322 = v322;
        let v323: f64 = ((self.scalar_v322) as f64).exp();
        self.scalar_v323 = v323;
        let v324: f64 = (self.scalar_v318 * self.scalar_v323);
        self.scalar_v324 = v324;
        let v334: f64 = (self.scalar_v84 * self.scalar_v333);
        self.scalar_v334 = v334;
        let v335: f64 = (self.scalar_v42 * self.scalar_v100);
        self.scalar_v335 = v335;
        let v336: f64 = (self.scalar_v334 + self.scalar_v335);
        self.scalar_v336 = v336;
        let v337: f64 = (self.scalar_v336 - self.scalar_v104);
        self.scalar_v337 = v337;
        let v338: f64 = (-self.scalar_v337);
        self.scalar_v338 = v338;
        let v339: f64 = (self.scalar_v82 * self.scalar_v338);
        self.scalar_v339 = v339;
        let v340: f64 = ((self.scalar_v339) as f64).exp();
        self.scalar_v340 = v340;
        let v341: f64 = (4.0 * self.scalar_v340);
        self.scalar_v341 = v341;
        let v342: f64 = (1.0 + self.scalar_v341);
        self.scalar_v342 = v342;
        let v343: f64 = ((self.scalar_v342) as f64).sqrt();
        self.scalar_v343 = v343;
        let v344: f64 = (1.0 + self.scalar_v343);
        self.scalar_v344 = v344;
        let v345: f64 = (0.5 * self.scalar_v344);
        self.scalar_v345 = v345;
        let v346: f64 = ((self.scalar_v345) as f64).ln();
        self.scalar_v346 = v346;
        let v347: f64 = (self.scalar_v106 * self.scalar_v346);
        self.scalar_v347 = v347;
        let v348: f64 = (self.scalar_v337 + self.scalar_v347);
        self.scalar_v348 = v348;
        let v351: f64 = (self.scalar_v325 / self.scalar_v348);
        self.scalar_v351 = v351;
        let v352: f64 = ((self.scalar_v351) as f64).ln();
        self.scalar_v352 = v352;
        let v353: f64 = (self.scalar_v350 * self.scalar_v352);
        self.scalar_v353 = v353;
        let v354: f64 = ((self.scalar_v353) as f64).exp();
        self.scalar_v354 = v354;
        let v355: f64 = (self.scalar_v349 * self.scalar_v354);
        self.scalar_v355 = v355;
        let v357: f64 = (self.scalar_v53 * self.scalar_v85);
        self.scalar_v357 = v357;
        let v358: f64 = (self.scalar_v40 * self.scalar_v87);
        self.scalar_v358 = v358;
        let v359: f64 = (self.scalar_v357 + self.scalar_v358);
        self.scalar_v359 = v359;
        let v360: f64 = ((self.scalar_v359) as f64).exp();
        self.scalar_v360 = v360;
        let v361: f64 = (self.scalar_v356 * self.scalar_v360);
        self.scalar_v361 = v361;
        let v363: f64 = (self.scalar_v206 + self.scalar_v357);
        self.scalar_v363 = v363;
        let v364: f64 = ((self.scalar_v363) as f64).exp();
        self.scalar_v364 = v364;
        let v365: f64 = (self.scalar_v362 * self.scalar_v364);
        self.scalar_v365 = v365;
        let v368: f64 = (self.scalar_v85 * self.scalar_v367);
        self.scalar_v368 = v368;
        let v369: f64 = ((self.scalar_v368) as f64).exp();
        self.scalar_v369 = v369;
        let v370: f64 = (self.scalar_v366 * self.scalar_v369);
        self.scalar_v370 = v370;
        let v372: f64 = (self.scalar_v82 * self.scalar_v219);
        self.scalar_v372 = v372;
        let v374: f64 = (self.scalar_v85 * self.scalar_v373);
        self.scalar_v374 = v374;
        let v375: f64 = ((self.scalar_v374) as f64).exp();
        self.scalar_v375 = v375;
        let v376: f64 = (self.scalar_v375 - 1.0);
        self.scalar_v376 = v376;
        let v377: f64 = (self.scalar_v372 * self.scalar_v376);
        self.scalar_v377 = v377;
        let v378: f64 = ((self.scalar_v377) as f64).exp();
        self.scalar_v378 = v378;
        let v379: f64 = (self.scalar_v371 / self.scalar_v378);
        self.scalar_v379 = v379;
        let v385: f64 = (self.scalar_v83 * self.scalar_v384);
        self.scalar_v385 = v385;
        let v386: f64 = (self.scalar_v383 + self.scalar_v385);
        self.scalar_v386 = v386;
        let v387: f64 = (self.scalar_v83 * self.scalar_v386);
        self.scalar_v387 = v387;
        let v388: f64 = (1.0 + self.scalar_v387);
        self.scalar_v388 = v388;
        let v389: f64 = (if self.scalar_v382 { self.scalar_v388 } else { 0.0 });
        self.scalar_v389 = v389;
        let v392: f64 = (self.scalar_v85 * self.scalar_v391);
        self.scalar_v392 = v392;
        let v393: f64 = ((self.scalar_v392) as f64).exp();
        self.scalar_v393 = v393;
        let v394: f64 = (if self.scalar_v390 { self.scalar_v393 } else { self.scalar_v389 });
        self.scalar_v394 = v394;
        let v396: f64 = (self.scalar_v394 * self.scalar_v395);
        self.scalar_v396 = v396;
        let v398: f64 = (self.scalar_v394 * self.scalar_v397);
        self.scalar_v398 = v398;
        let v399: f64 = ((self.scalar_v263) as f64).exp();
        self.scalar_v399 = v399;
        let v400: f64 = (self.scalar_v398 * self.scalar_v399);
        self.scalar_v400 = v400;
        let v404: f64 = (self.scalar_v85 * self.scalar_v403);
        self.scalar_v404 = v404;
        let v405: f64 = ((self.scalar_v404) as f64).exp();
        self.scalar_v405 = v405;
        let v406: f64 = (self.scalar_v402 * self.scalar_v405);
        self.scalar_v406 = v406;
        let v409: f64 = (self.scalar_v85 * self.scalar_v408);
        self.scalar_v409 = v409;
        let v410: f64 = ((self.scalar_v409) as f64).exp();
        self.scalar_v410 = v410;
        let v411: f64 = (self.scalar_v407 * self.scalar_v410);
        self.scalar_v411 = v411;
        let v414: f64 = (self.scalar_v85 * self.scalar_v413);
        self.scalar_v414 = v414;
        let v415: f64 = ((self.scalar_v414) as f64).exp();
        self.scalar_v415 = v415;
        let v416: f64 = (self.scalar_v412 * self.scalar_v415);
        self.scalar_v416 = v416;
        let v419: f64 = (self.scalar_v85 * self.scalar_v418);
        self.scalar_v419 = v419;
        let v420: f64 = ((self.scalar_v419) as f64).exp();
        self.scalar_v420 = v420;
        let v421: f64 = (self.scalar_v417 * self.scalar_v420);
        self.scalar_v421 = v421;
        let v423: f64 = (self.scalar_v83 * self.scalar_v422);
        self.scalar_v423 = v423;
        let v424: f64 = (1.0 + self.scalar_v423);
        self.scalar_v424 = v424;
        let v425: f64 = (self.scalar_v421 * self.scalar_v424);
        self.scalar_v425 = v425;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
