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
    pub p112: f64,
    pub p113: f64,
    pub p114: f64,
    pub p115: f64,
    pub p116: f64,
    pub p117: f64,
    pub p118: f64,
    pub p119: f64,
    pub p120: f64,
    pub p121: f64,
    pub p122: f64,
    pub p123: f64,
    pub p124: f64,
    pub p125: f64,
    pub p126: f64,
    pub p127: f64,
    pub p128: f64,
    pub p129: f64,
    pub p130: f64,
    pub p131: f64,
    pub p132: f64,
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
            params.p0 = 0.0;
            params.p1 = 1.0;
            params.p2 = 1.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = -100.0;
            params.p9 = 500.0;
            params.p10 = 1e-12;
            params.p11 = 1.0;
            params.p12 = 1e22;
            params.p13 = 27.0;
            params.p14 = -100.0;
            params.p15 = 500.0;
            params.p16 = 0.0;
            params.p17 = 0.0;
            params.p18 = 0.0;
            params.p19 = 0.0;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 1e-16;
            params.p27 = 1.0;
            params.p28 = 1.0;
            params.p29 = 1.0;
            params.p30 = 0.0;
            params.p31 = 0.0;
            params.p32 = 1.0;
            params.p33 = 1.0;
            params.p34 = 0.9;
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 0.75;
            params.p38 = 0.33;
            params.p39 = -0.5;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.75;
            params.p43 = 0.33;
            params.p44 = -0.5;
            params.p45 = 0.0;
            params.p46 = 0.1;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 0.75;
            params.p51 = 0.33;
            params.p52 = -0.5;
            params.p53 = 0.0;
            params.p54 = 1e-18;
            params.p55 = 1.0;
            params.p56 = 1.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 2.0;
            params.p60 = 1e-16;
            params.p61 = 1.0;
            params.p62 = 0.0;
            params.p63 = 2.0;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.0;
            params.p67 = 1.0;
            params.p68 = 0.0;
            params.p69 = 2.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.5;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.0;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.33;
            params.p88 = 0.0;
            params.p89 = 1.0;
            params.p90 = 1e-6;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 1.0;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 1.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 0.0;
            params.p112 = 0.0;
            params.p113 = 1.12;
            params.p114 = 1.12;
            params.p115 = 1.12;
            params.p116 = 1.12;
            params.p117 = 1.12;
            params.p118 = 1.12;
            params.p119 = 1.12;
            params.p120 = 1.12;
            params.p121 = 0.0;
            params.p122 = 3.0;
            params.p123 = 3.0;
            params.p124 = 3.0;
            params.p125 = 0.0;
            params.p126 = 0.0;
            params.p127 = 0.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
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
    pub nodes: [usize; 14],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 133]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 11]>,
    pub(crate) ddt_state_previous: Box<[f64; 11]>,
    pub(crate) ddt_state_older: Box<[f64; 11]>,
    pub(crate) ddt_state_initialized: Box<[bool; 11]>,
    pub(crate) ddt_derivative_current: Box<[f64; 11]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 11]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v8: bool,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v16: bool,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: bool,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: bool,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v34: bool,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: bool,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: bool,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: bool,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: bool,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: bool,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v113: bool,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: bool,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v398: f64,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v413: bool,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v422: bool,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v433: bool,
    pub(crate) scalar_v436: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v441: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v445: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v450: bool,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v454: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v531: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v536: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v555: f64,
    pub(crate) scalar_v556: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v589: f64,
    pub(crate) scalar_v590: f64,
    pub(crate) scalar_v594: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v617: f64,
    pub(crate) scalar_v621: f64,
    pub(crate) scalar_v643: f64,
    pub(crate) scalar_v645: f64,
    pub(crate) scalar_v648: f64,
    pub(crate) scalar_v650: f64,
    pub(crate) scalar_v653: f64,
    pub(crate) scalar_v655: f64,
    pub(crate) scalar_v657: f64,
    pub(crate) scalar_v660: f64,
    pub(crate) scalar_v666: f64,
    pub(crate) scalar_v667: f64,
    pub(crate) scalar_v674: f64,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v679: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v764: f64,
    pub(crate) scalar_v766: f64,
    pub(crate) scalar_v767: bool,
    pub(crate) scalar_v772: f64,
    pub(crate) scalar_v773: f64,
    pub(crate) scalar_v774: f64,
    pub(crate) scalar_v779: f64,
    pub(crate) scalar_v782: f64,
    pub(crate) scalar_v802: bool,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v805: f64,
    pub(crate) scalar_v845: f64,
    pub(crate) scalar_v846: bool,
    pub(crate) scalar_v851: f64,
    pub(crate) scalar_v852: f64,
    pub(crate) scalar_v858: f64,
    pub(crate) scalar_v861: f64,
    pub(crate) scalar_v868: f64,
    pub(crate) scalar_v869: bool,
    pub(crate) scalar_v870: f64,
    pub(crate) scalar_v901: f64,
    pub(crate) scalar_v902: bool,
    pub(crate) scalar_v903: bool,
    pub(crate) scalar_v904: bool,
    pub(crate) scalar_v905: bool,
    pub(crate) scalar_v913: f64,
    pub(crate) scalar_v914: f64,
    pub(crate) scalar_v919: f64,
    pub(crate) scalar_v920: f64,
    pub(crate) scalar_v970: f64,
    pub(crate) scalar_v989: bool,
    pub(crate) scalar_v990: bool,
    pub(crate) scalar_v1019: f64,
    pub(crate) scalar_v1075: f64,
    pub(crate) scalar_v1076: bool,
    pub(crate) scalar_v1077: f64,
    pub(crate) scalar_v1090: f64,
    pub(crate) scalar_v1094: bool,
    pub(crate) scalar_v1106: f64,
    pub(crate) scalar_v1111: bool,
    pub(crate) scalar_v1140: f64,
    pub(crate) scalar_v1142: f64,
    pub(crate) scalar_v1181: bool,
    pub(crate) scalar_v1185: f64,
    pub(crate) scalar_v1186: bool,
    pub(crate) scalar_v1219: f64,
    pub(crate) scalar_v1220: bool,
    pub(crate) scalar_v1221: bool,
    pub(crate) scalar_v1232: bool,
    pub(crate) scalar_v1233: bool,
    pub(crate) scalar_v1237: bool,
    pub(crate) scalar_v1238: bool,
    pub(crate) scalar_v1261: bool,
    pub(crate) scalar_v1262: bool,
    pub(crate) scalar_v1263: bool,
    pub(crate) scalar_v1301: bool,
    pub(crate) scalar_v1322: bool,
    pub(crate) scalar_v1323: bool,
    pub(crate) scalar_v1348: bool,
    pub(crate) scalar_v1356: bool,
    pub(crate) scalar_v1361: bool,
    pub(crate) scalar_v1378: f64,
    pub(crate) scalar_v1407: f64,
    pub(crate) scalar_v1431: f64,
    pub(crate) scalar_v1467: bool,
    pub(crate) scalar_v1468: bool,
    pub(crate) scalar_v1469: bool,
    pub(crate) scalar_v1506: bool,
    pub(crate) scalar_v1564: f64,
    pub(crate) scalar_v1565: bool,
    pub(crate) scalar_v1570: f64,
    pub(crate) scalar_v1571: f64,
    pub(crate) scalar_v1584: f64,
    pub(crate) scalar_v1606: bool,
    pub(crate) scalar_v1608: f64,
    pub(crate) scalar_v1609: bool,
    pub(crate) scalar_v1612: f64,
    pub(crate) scalar_v1613: f64,
    pub(crate) scalar_v1614: f64,
    pub(crate) scalar_v1627: f64,
    pub(crate) scalar_v1648: bool,
    pub(crate) scalar_v1650: f64,
    pub(crate) scalar_v1651: bool,
    pub(crate) scalar_v1652: f64,
    pub(crate) scalar_v1653: bool,
    pub(crate) scalar_v1654: bool,
    pub(crate) scalar_v1655: f64,
    pub(crate) scalar_v1656: bool,
    pub(crate) scalar_v1657: bool,
    pub(crate) scalar_v1672: bool,
    pub(crate) scalar_v1673: bool,
    pub(crate) scalar_v1677: f64,
    pub(crate) scalar_v1681: bool,
    pub(crate) scalar_v1685: bool,
    pub(crate) scalar_v1686: bool,
    pub(crate) scalar_v1687: bool,
    pub(crate) scalar_v1726: bool,
    pub(crate) scalar_v1755: f64,
    pub(crate) scalar_v1756: f64,
    pub(crate) scalar_v1783: bool,
    pub(crate) scalar_v1787: f64,
    pub(crate) scalar_v1788: bool,
    pub(crate) scalar_v1789: bool,
    pub(crate) scalar_v1794: f64,
    pub(crate) scalar_v1795: f64,
    pub(crate) scalar_v1800: f64,
    pub(crate) scalar_v1803: f64,
    pub(crate) scalar_v1823: bool,
    pub(crate) scalar_v1824: bool,
    pub(crate) scalar_v1826: f64,
    pub(crate) scalar_v1827: f64,
    pub(crate) scalar_v1865: bool,
    pub(crate) scalar_v2045: f64,
    pub(crate) scalar_v2046: f64,
    pub(crate) scalar_v2050: f64,
    pub(crate) scalar_v2066: f64,
    pub(crate) scalar_v2069: f64,
    pub(crate) scalar_v2077: f64,
    pub(crate) scalar_v2080: f64,
    pub(crate) scalar_v2082: f64,
    pub(crate) scalar_v2084: f64,
    pub(crate) scalar_v2086: f64,
    pub(crate) scalar_v2105: f64,
    pub(crate) scalar_v2110: f64,
    pub(crate) scalar_v2116: f64,
    pub(crate) scalar_v2122: f64,
    pub(crate) scalar_v2130: f64,
    pub(crate) scalar_v2136: f64,
    pub(crate) scalar_v2142: f64,
    pub(crate) scalar_v2150: f64,
    pub(crate) scalar_v2155: f64,
    pub(crate) scalar_v2160: f64,
    pub(crate) scalar_v2170: f64,
    pub(crate) scalar_v2187: f64,
    pub(crate) scalar_v2203: f64,
    pub(crate) scalar_v2219: f64,
    pub(crate) scalar_v2235: f64,
    pub(crate) scalar_v2251: f64,
    pub(crate) scalar_v2267: f64,
    pub(crate) scalar_v2291: f64,
    pub(crate) scalar_v2307: f64,
    pub(crate) scalar_v2468: f64,
    pub(crate) scalar_v2486: f64,
    pub(crate) scalar_v2491: f64,
    pub(crate) scalar_v2504: f64,
    pub(crate) scalar_v2614: f64,
    pub(crate) scalar_v2782: f64,
    pub(crate) scalar_v2993: f64,
    pub(crate) scalar_v3247: f64,
    pub(crate) scalar_v3266: f64,
    pub(crate) scalar_v4595: f64,
    pub(crate) scalar_v4622: f64,
    pub(crate) scalar_v4704: f64,
    pub(crate) scalar_v4731: f64,
    pub(crate) scalar_v4833: f64,
    pub(crate) scalar_v5291: f64,
    pub(crate) scalar_v6021: f64,
    pub(crate) scalar_v6022: f64,
    pub(crate) scalar_v6023: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v77: bool,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v85: bool,
    pub(crate) scalar_v86: bool,
    pub(crate) scalar_v87: bool,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v359: f64,
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
            scalar_v4: self.scalar_v4,
            scalar_v6: self.scalar_v6,
            scalar_v8: self.scalar_v8,
            scalar_v12: self.scalar_v12,
            scalar_v14: self.scalar_v14,
            scalar_v16: self.scalar_v16,
            scalar_v19: self.scalar_v19,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v32: self.scalar_v32,
            scalar_v34: self.scalar_v34,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v45: self.scalar_v45,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v65: self.scalar_v65,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v72: self.scalar_v72,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v83: self.scalar_v83,
            scalar_v84: self.scalar_v84,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v113: self.scalar_v113,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v200: self.scalar_v200,
            scalar_v201: self.scalar_v201,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v233: self.scalar_v233,
            scalar_v234: self.scalar_v234,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v309: self.scalar_v309,
            scalar_v320: self.scalar_v320,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v393: self.scalar_v393,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v407: self.scalar_v407,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v413: self.scalar_v413,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v433: self.scalar_v433,
            scalar_v436: self.scalar_v436,
            scalar_v437: self.scalar_v437,
            scalar_v440: self.scalar_v440,
            scalar_v441: self.scalar_v441,
            scalar_v444: self.scalar_v444,
            scalar_v445: self.scalar_v445,
            scalar_v446: self.scalar_v446,
            scalar_v450: self.scalar_v450,
            scalar_v453: self.scalar_v453,
            scalar_v454: self.scalar_v454,
            scalar_v526: self.scalar_v526,
            scalar_v531: self.scalar_v531,
            scalar_v532: self.scalar_v532,
            scalar_v536: self.scalar_v536,
            scalar_v537: self.scalar_v537,
            scalar_v541: self.scalar_v541,
            scalar_v542: self.scalar_v542,
            scalar_v548: self.scalar_v548,
            scalar_v555: self.scalar_v555,
            scalar_v556: self.scalar_v556,
            scalar_v561: self.scalar_v561,
            scalar_v589: self.scalar_v589,
            scalar_v590: self.scalar_v590,
            scalar_v594: self.scalar_v594,
            scalar_v616: self.scalar_v616,
            scalar_v617: self.scalar_v617,
            scalar_v621: self.scalar_v621,
            scalar_v643: self.scalar_v643,
            scalar_v645: self.scalar_v645,
            scalar_v648: self.scalar_v648,
            scalar_v650: self.scalar_v650,
            scalar_v653: self.scalar_v653,
            scalar_v655: self.scalar_v655,
            scalar_v657: self.scalar_v657,
            scalar_v660: self.scalar_v660,
            scalar_v666: self.scalar_v666,
            scalar_v667: self.scalar_v667,
            scalar_v674: self.scalar_v674,
            scalar_v675: self.scalar_v675,
            scalar_v679: self.scalar_v679,
            scalar_v680: self.scalar_v680,
            scalar_v764: self.scalar_v764,
            scalar_v766: self.scalar_v766,
            scalar_v767: self.scalar_v767,
            scalar_v772: self.scalar_v772,
            scalar_v773: self.scalar_v773,
            scalar_v774: self.scalar_v774,
            scalar_v779: self.scalar_v779,
            scalar_v782: self.scalar_v782,
            scalar_v802: self.scalar_v802,
            scalar_v804: self.scalar_v804,
            scalar_v805: self.scalar_v805,
            scalar_v845: self.scalar_v845,
            scalar_v846: self.scalar_v846,
            scalar_v851: self.scalar_v851,
            scalar_v852: self.scalar_v852,
            scalar_v858: self.scalar_v858,
            scalar_v861: self.scalar_v861,
            scalar_v868: self.scalar_v868,
            scalar_v869: self.scalar_v869,
            scalar_v870: self.scalar_v870,
            scalar_v901: self.scalar_v901,
            scalar_v902: self.scalar_v902,
            scalar_v903: self.scalar_v903,
            scalar_v904: self.scalar_v904,
            scalar_v905: self.scalar_v905,
            scalar_v913: self.scalar_v913,
            scalar_v914: self.scalar_v914,
            scalar_v919: self.scalar_v919,
            scalar_v920: self.scalar_v920,
            scalar_v970: self.scalar_v970,
            scalar_v989: self.scalar_v989,
            scalar_v990: self.scalar_v990,
            scalar_v1019: self.scalar_v1019,
            scalar_v1075: self.scalar_v1075,
            scalar_v1076: self.scalar_v1076,
            scalar_v1077: self.scalar_v1077,
            scalar_v1090: self.scalar_v1090,
            scalar_v1094: self.scalar_v1094,
            scalar_v1106: self.scalar_v1106,
            scalar_v1111: self.scalar_v1111,
            scalar_v1140: self.scalar_v1140,
            scalar_v1142: self.scalar_v1142,
            scalar_v1181: self.scalar_v1181,
            scalar_v1185: self.scalar_v1185,
            scalar_v1186: self.scalar_v1186,
            scalar_v1219: self.scalar_v1219,
            scalar_v1220: self.scalar_v1220,
            scalar_v1221: self.scalar_v1221,
            scalar_v1232: self.scalar_v1232,
            scalar_v1233: self.scalar_v1233,
            scalar_v1237: self.scalar_v1237,
            scalar_v1238: self.scalar_v1238,
            scalar_v1261: self.scalar_v1261,
            scalar_v1262: self.scalar_v1262,
            scalar_v1263: self.scalar_v1263,
            scalar_v1301: self.scalar_v1301,
            scalar_v1322: self.scalar_v1322,
            scalar_v1323: self.scalar_v1323,
            scalar_v1348: self.scalar_v1348,
            scalar_v1356: self.scalar_v1356,
            scalar_v1361: self.scalar_v1361,
            scalar_v1378: self.scalar_v1378,
            scalar_v1407: self.scalar_v1407,
            scalar_v1431: self.scalar_v1431,
            scalar_v1467: self.scalar_v1467,
            scalar_v1468: self.scalar_v1468,
            scalar_v1469: self.scalar_v1469,
            scalar_v1506: self.scalar_v1506,
            scalar_v1564: self.scalar_v1564,
            scalar_v1565: self.scalar_v1565,
            scalar_v1570: self.scalar_v1570,
            scalar_v1571: self.scalar_v1571,
            scalar_v1584: self.scalar_v1584,
            scalar_v1606: self.scalar_v1606,
            scalar_v1608: self.scalar_v1608,
            scalar_v1609: self.scalar_v1609,
            scalar_v1612: self.scalar_v1612,
            scalar_v1613: self.scalar_v1613,
            scalar_v1614: self.scalar_v1614,
            scalar_v1627: self.scalar_v1627,
            scalar_v1648: self.scalar_v1648,
            scalar_v1650: self.scalar_v1650,
            scalar_v1651: self.scalar_v1651,
            scalar_v1652: self.scalar_v1652,
            scalar_v1653: self.scalar_v1653,
            scalar_v1654: self.scalar_v1654,
            scalar_v1655: self.scalar_v1655,
            scalar_v1656: self.scalar_v1656,
            scalar_v1657: self.scalar_v1657,
            scalar_v1672: self.scalar_v1672,
            scalar_v1673: self.scalar_v1673,
            scalar_v1677: self.scalar_v1677,
            scalar_v1681: self.scalar_v1681,
            scalar_v1685: self.scalar_v1685,
            scalar_v1686: self.scalar_v1686,
            scalar_v1687: self.scalar_v1687,
            scalar_v1726: self.scalar_v1726,
            scalar_v1755: self.scalar_v1755,
            scalar_v1756: self.scalar_v1756,
            scalar_v1783: self.scalar_v1783,
            scalar_v1787: self.scalar_v1787,
            scalar_v1788: self.scalar_v1788,
            scalar_v1789: self.scalar_v1789,
            scalar_v1794: self.scalar_v1794,
            scalar_v1795: self.scalar_v1795,
            scalar_v1800: self.scalar_v1800,
            scalar_v1803: self.scalar_v1803,
            scalar_v1823: self.scalar_v1823,
            scalar_v1824: self.scalar_v1824,
            scalar_v1826: self.scalar_v1826,
            scalar_v1827: self.scalar_v1827,
            scalar_v1865: self.scalar_v1865,
            scalar_v2045: self.scalar_v2045,
            scalar_v2046: self.scalar_v2046,
            scalar_v2050: self.scalar_v2050,
            scalar_v2066: self.scalar_v2066,
            scalar_v2069: self.scalar_v2069,
            scalar_v2077: self.scalar_v2077,
            scalar_v2080: self.scalar_v2080,
            scalar_v2082: self.scalar_v2082,
            scalar_v2084: self.scalar_v2084,
            scalar_v2086: self.scalar_v2086,
            scalar_v2105: self.scalar_v2105,
            scalar_v2110: self.scalar_v2110,
            scalar_v2116: self.scalar_v2116,
            scalar_v2122: self.scalar_v2122,
            scalar_v2130: self.scalar_v2130,
            scalar_v2136: self.scalar_v2136,
            scalar_v2142: self.scalar_v2142,
            scalar_v2150: self.scalar_v2150,
            scalar_v2155: self.scalar_v2155,
            scalar_v2160: self.scalar_v2160,
            scalar_v2170: self.scalar_v2170,
            scalar_v2187: self.scalar_v2187,
            scalar_v2203: self.scalar_v2203,
            scalar_v2219: self.scalar_v2219,
            scalar_v2235: self.scalar_v2235,
            scalar_v2251: self.scalar_v2251,
            scalar_v2267: self.scalar_v2267,
            scalar_v2291: self.scalar_v2291,
            scalar_v2307: self.scalar_v2307,
            scalar_v2468: self.scalar_v2468,
            scalar_v2486: self.scalar_v2486,
            scalar_v2491: self.scalar_v2491,
            scalar_v2504: self.scalar_v2504,
            scalar_v2614: self.scalar_v2614,
            scalar_v2782: self.scalar_v2782,
            scalar_v2993: self.scalar_v2993,
            scalar_v3247: self.scalar_v3247,
            scalar_v3266: self.scalar_v3266,
            scalar_v4595: self.scalar_v4595,
            scalar_v4622: self.scalar_v4622,
            scalar_v4704: self.scalar_v4704,
            scalar_v4731: self.scalar_v4731,
            scalar_v4833: self.scalar_v4833,
            scalar_v5291: self.scalar_v5291,
            scalar_v6021: self.scalar_v6021,
            scalar_v6022: self.scalar_v6022,
            scalar_v6023: self.scalar_v6023,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v87: self.scalar_v87,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v95: self.scalar_v95,
            scalar_v97: self.scalar_v97,
            scalar_v103: self.scalar_v103,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v124: self.scalar_v124,
            scalar_v167: self.scalar_v167,
            scalar_v203: self.scalar_v203,
            scalar_v236: self.scalar_v236,
            scalar_v257: self.scalar_v257,
            scalar_v277: self.scalar_v277,
            scalar_v297: self.scalar_v297,
            scalar_v339: self.scalar_v339,
            scalar_v359: self.scalar_v359,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 10;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 10] = ["dt", "cx", "ci", "bx", "bi", "ei", "bp", "si", "xf1", "xf2"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 133;
    pub const VARIABLE_COUNT: usize = 359;
    pub const DDT_STATE_COUNT: usize = 11;
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
            scalar_v4: 0.0,
            scalar_v6: 0.0,
            scalar_v8: false,
            scalar_v12: 0.0,
            scalar_v14: 0.0,
            scalar_v16: false,
            scalar_v19: 0.0,
            scalar_v22: 0.0,
            scalar_v23: false,
            scalar_v28: 0.0,
            scalar_v29: false,
            scalar_v32: 0.0,
            scalar_v34: false,
            scalar_v37: 0.0,
            scalar_v38: 0.0,
            scalar_v40: 0.0,
            scalar_v41: false,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v45: 0.0,
            scalar_v46: false,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v50: 0.0,
            scalar_v51: false,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v55: 0.0,
            scalar_v56: false,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v60: 0.0,
            scalar_v61: false,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v65: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v72: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: false,
            scalar_v102: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v113: false,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v129: 0.0,
            scalar_v130: false,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v195: 0.0,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
            scalar_v200: 0.0,
            scalar_v201: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v248: 0.0,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v294: 0.0,
            scalar_v295: 0.0,
            scalar_v309: 0.0,
            scalar_v320: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v351: 0.0,
            scalar_v352: 0.0,
            scalar_v353: 0.0,
            scalar_v356: 0.0,
            scalar_v357: 0.0,
            scalar_v393: 0.0,
            scalar_v396: 0.0,
            scalar_v397: 0.0,
            scalar_v398: 0.0,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v407: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v413: false,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v422: false,
            scalar_v423: 0.0,
            scalar_v427: 0.0,
            scalar_v428: 0.0,
            scalar_v429: 0.0,
            scalar_v433: false,
            scalar_v436: 0.0,
            scalar_v437: 0.0,
            scalar_v440: 0.0,
            scalar_v441: 0.0,
            scalar_v444: 0.0,
            scalar_v445: 0.0,
            scalar_v446: 0.0,
            scalar_v450: false,
            scalar_v453: 0.0,
            scalar_v454: 0.0,
            scalar_v526: 0.0,
            scalar_v531: 0.0,
            scalar_v532: 0.0,
            scalar_v536: 0.0,
            scalar_v537: 0.0,
            scalar_v541: 0.0,
            scalar_v542: 0.0,
            scalar_v548: 0.0,
            scalar_v555: 0.0,
            scalar_v556: 0.0,
            scalar_v561: 0.0,
            scalar_v589: 0.0,
            scalar_v590: 0.0,
            scalar_v594: 0.0,
            scalar_v616: 0.0,
            scalar_v617: 0.0,
            scalar_v621: 0.0,
            scalar_v643: 0.0,
            scalar_v645: 0.0,
            scalar_v648: 0.0,
            scalar_v650: 0.0,
            scalar_v653: 0.0,
            scalar_v655: 0.0,
            scalar_v657: 0.0,
            scalar_v660: 0.0,
            scalar_v666: 0.0,
            scalar_v667: 0.0,
            scalar_v674: 0.0,
            scalar_v675: 0.0,
            scalar_v679: 0.0,
            scalar_v680: 0.0,
            scalar_v764: 0.0,
            scalar_v766: 0.0,
            scalar_v767: false,
            scalar_v772: 0.0,
            scalar_v773: 0.0,
            scalar_v774: 0.0,
            scalar_v779: 0.0,
            scalar_v782: 0.0,
            scalar_v802: false,
            scalar_v804: 0.0,
            scalar_v805: 0.0,
            scalar_v845: 0.0,
            scalar_v846: false,
            scalar_v851: 0.0,
            scalar_v852: 0.0,
            scalar_v858: 0.0,
            scalar_v861: 0.0,
            scalar_v868: 0.0,
            scalar_v869: false,
            scalar_v870: 0.0,
            scalar_v901: 0.0,
            scalar_v902: false,
            scalar_v903: false,
            scalar_v904: false,
            scalar_v905: false,
            scalar_v913: 0.0,
            scalar_v914: 0.0,
            scalar_v919: 0.0,
            scalar_v920: 0.0,
            scalar_v970: 0.0,
            scalar_v989: false,
            scalar_v990: false,
            scalar_v1019: 0.0,
            scalar_v1075: 0.0,
            scalar_v1076: false,
            scalar_v1077: 0.0,
            scalar_v1090: 0.0,
            scalar_v1094: false,
            scalar_v1106: 0.0,
            scalar_v1111: false,
            scalar_v1140: 0.0,
            scalar_v1142: 0.0,
            scalar_v1181: false,
            scalar_v1185: 0.0,
            scalar_v1186: false,
            scalar_v1219: 0.0,
            scalar_v1220: false,
            scalar_v1221: false,
            scalar_v1232: false,
            scalar_v1233: false,
            scalar_v1237: false,
            scalar_v1238: false,
            scalar_v1261: false,
            scalar_v1262: false,
            scalar_v1263: false,
            scalar_v1301: false,
            scalar_v1322: false,
            scalar_v1323: false,
            scalar_v1348: false,
            scalar_v1356: false,
            scalar_v1361: false,
            scalar_v1378: 0.0,
            scalar_v1407: 0.0,
            scalar_v1431: 0.0,
            scalar_v1467: false,
            scalar_v1468: false,
            scalar_v1469: false,
            scalar_v1506: false,
            scalar_v1564: 0.0,
            scalar_v1565: false,
            scalar_v1570: 0.0,
            scalar_v1571: 0.0,
            scalar_v1584: 0.0,
            scalar_v1606: false,
            scalar_v1608: 0.0,
            scalar_v1609: false,
            scalar_v1612: 0.0,
            scalar_v1613: 0.0,
            scalar_v1614: 0.0,
            scalar_v1627: 0.0,
            scalar_v1648: false,
            scalar_v1650: 0.0,
            scalar_v1651: false,
            scalar_v1652: 0.0,
            scalar_v1653: false,
            scalar_v1654: false,
            scalar_v1655: 0.0,
            scalar_v1656: false,
            scalar_v1657: false,
            scalar_v1672: false,
            scalar_v1673: false,
            scalar_v1677: 0.0,
            scalar_v1681: false,
            scalar_v1685: false,
            scalar_v1686: false,
            scalar_v1687: false,
            scalar_v1726: false,
            scalar_v1755: 0.0,
            scalar_v1756: 0.0,
            scalar_v1783: false,
            scalar_v1787: 0.0,
            scalar_v1788: false,
            scalar_v1789: false,
            scalar_v1794: 0.0,
            scalar_v1795: 0.0,
            scalar_v1800: 0.0,
            scalar_v1803: 0.0,
            scalar_v1823: false,
            scalar_v1824: false,
            scalar_v1826: 0.0,
            scalar_v1827: 0.0,
            scalar_v1865: false,
            scalar_v2045: 0.0,
            scalar_v2046: 0.0,
            scalar_v2050: 0.0,
            scalar_v2066: 0.0,
            scalar_v2069: 0.0,
            scalar_v2077: 0.0,
            scalar_v2080: 0.0,
            scalar_v2082: 0.0,
            scalar_v2084: 0.0,
            scalar_v2086: 0.0,
            scalar_v2105: 0.0,
            scalar_v2110: 0.0,
            scalar_v2116: 0.0,
            scalar_v2122: 0.0,
            scalar_v2130: 0.0,
            scalar_v2136: 0.0,
            scalar_v2142: 0.0,
            scalar_v2150: 0.0,
            scalar_v2155: 0.0,
            scalar_v2160: 0.0,
            scalar_v2170: 0.0,
            scalar_v2187: 0.0,
            scalar_v2203: 0.0,
            scalar_v2219: 0.0,
            scalar_v2235: 0.0,
            scalar_v2251: 0.0,
            scalar_v2267: 0.0,
            scalar_v2291: 0.0,
            scalar_v2307: 0.0,
            scalar_v2468: 0.0,
            scalar_v2486: 0.0,
            scalar_v2491: 0.0,
            scalar_v2504: 0.0,
            scalar_v2614: 0.0,
            scalar_v2782: 0.0,
            scalar_v2993: 0.0,
            scalar_v3247: 0.0,
            scalar_v3266: 0.0,
            scalar_v4595: 0.0,
            scalar_v4622: 0.0,
            scalar_v4704: 0.0,
            scalar_v4731: 0.0,
            scalar_v4833: 0.0,
            scalar_v5291: 0.0,
            scalar_v6021: 0.0,
            scalar_v6022: 0.0,
            scalar_v6023: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v77: false,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v85: false,
            scalar_v86: false,
            scalar_v87: false,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v95: 0.0,
            scalar_v97: 0.0,
            scalar_v103: 0.0,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v124: 0.0,
            scalar_v167: 0.0,
            scalar_v203: 0.0,
            scalar_v236: 0.0,
            scalar_v257: 0.0,
            scalar_v277: 0.0,
            scalar_v297: 0.0,
            scalar_v339: 0.0,
            scalar_v359: 0.0,
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
            scalar_v4,
            scalar_v6,
            scalar_v8,
            scalar_v12,
            scalar_v14,
            scalar_v16,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v28,
            scalar_v29,
            scalar_v32,
            scalar_v34,
            scalar_v37,
            scalar_v38,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v65,
            scalar_v68,
            scalar_v69,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v83,
            scalar_v84,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v104,
            scalar_v105,
            scalar_v113,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v120,
            scalar_v121,
            scalar_v129,
            scalar_v130,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v141,
            scalar_v142,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v164,
            scalar_v165,
            scalar_v176,
            scalar_v177,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v200,
            scalar_v201,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v233,
            scalar_v234,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v254,
            scalar_v255,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v274,
            scalar_v275,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v294,
            scalar_v295,
            scalar_v309,
            scalar_v320,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v336,
            scalar_v337,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v356,
            scalar_v357,
            scalar_v393,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v407,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v422,
            scalar_v423,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v433,
            scalar_v436,
            scalar_v437,
            scalar_v440,
            scalar_v441,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v450,
            scalar_v453,
            scalar_v454,
            scalar_v526,
            scalar_v531,
            scalar_v532,
            scalar_v536,
            scalar_v537,
            scalar_v541,
            scalar_v542,
            scalar_v548,
            scalar_v555,
            scalar_v556,
            scalar_v561,
            scalar_v589,
            scalar_v590,
            scalar_v594,
            scalar_v616,
            scalar_v617,
            scalar_v621,
            scalar_v643,
            scalar_v645,
            scalar_v648,
            scalar_v650,
            scalar_v653,
            scalar_v655,
            scalar_v657,
            scalar_v660,
            scalar_v666,
            scalar_v667,
            scalar_v674,
            scalar_v675,
            scalar_v679,
            scalar_v680,
            scalar_v764,
            scalar_v766,
            scalar_v767,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v779,
            scalar_v782,
            scalar_v802,
            scalar_v804,
            scalar_v805,
            scalar_v845,
            scalar_v846,
            scalar_v851,
            scalar_v852,
            scalar_v858,
            scalar_v861,
            scalar_v868,
            scalar_v869,
            scalar_v870,
            scalar_v901,
            scalar_v902,
            scalar_v903,
            scalar_v904,
            scalar_v905,
            scalar_v913,
            scalar_v914,
            scalar_v919,
            scalar_v920,
            scalar_v970,
            scalar_v989,
            scalar_v990,
            scalar_v1019,
            scalar_v1075,
            scalar_v1076,
            scalar_v1077,
            scalar_v1090,
            scalar_v1094,
            scalar_v1106,
            scalar_v1111,
            scalar_v1140,
            scalar_v1142,
            scalar_v1181,
            scalar_v1185,
            scalar_v1186,
            scalar_v1219,
            scalar_v1220,
            scalar_v1221,
            scalar_v1232,
            scalar_v1233,
            scalar_v1237,
            scalar_v1238,
            scalar_v1261,
            scalar_v1262,
            scalar_v1263,
            scalar_v1301,
            scalar_v1322,
            scalar_v1323,
            scalar_v1348,
            scalar_v1356,
            scalar_v1361,
            scalar_v1378,
            scalar_v1407,
            scalar_v1431,
            scalar_v1467,
            scalar_v1468,
            scalar_v1469,
            scalar_v1506,
            scalar_v1564,
            scalar_v1565,
            scalar_v1570,
            scalar_v1571,
            scalar_v1584,
            scalar_v1606,
            scalar_v1608,
            scalar_v1609,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1627,
            scalar_v1648,
            scalar_v1650,
            scalar_v1651,
            scalar_v1652,
            scalar_v1653,
            scalar_v1654,
            scalar_v1655,
            scalar_v1656,
            scalar_v1657,
            scalar_v1672,
            scalar_v1673,
            scalar_v1677,
            scalar_v1681,
            scalar_v1685,
            scalar_v1686,
            scalar_v1687,
            scalar_v1726,
            scalar_v1755,
            scalar_v1756,
            scalar_v1783,
            scalar_v1787,
            scalar_v1788,
            scalar_v1789,
            scalar_v1794,
            scalar_v1795,
            scalar_v1800,
            scalar_v1803,
            scalar_v1823,
            scalar_v1824,
            scalar_v1826,
            scalar_v1827,
            scalar_v1865,
            scalar_v2045,
            scalar_v2046,
            scalar_v2050,
            scalar_v2066,
            scalar_v2069,
            scalar_v2077,
            scalar_v2080,
            scalar_v2082,
            scalar_v2084,
            scalar_v2086,
            scalar_v2105,
            scalar_v2110,
            scalar_v2116,
            scalar_v2122,
            scalar_v2130,
            scalar_v2136,
            scalar_v2142,
            scalar_v2150,
            scalar_v2155,
            scalar_v2160,
            scalar_v2170,
            scalar_v2187,
            scalar_v2203,
            scalar_v2219,
            scalar_v2235,
            scalar_v2251,
            scalar_v2267,
            scalar_v2291,
            scalar_v2307,
            scalar_v2468,
            scalar_v2486,
            scalar_v2491,
            scalar_v2504,
            scalar_v2614,
            scalar_v2782,
            scalar_v2993,
            scalar_v3247,
            scalar_v3266,
            scalar_v4595,
            scalar_v4622,
            scalar_v4704,
            scalar_v4731,
            scalar_v4833,
            scalar_v5291,
            scalar_v6021,
            scalar_v6022,
            scalar_v6023,
            scalar_v73,
            scalar_v74,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v95,
            scalar_v97,
            scalar_v103,
            scalar_v106,
            scalar_v107,
            scalar_v124,
            scalar_v167,
            scalar_v203,
            scalar_v236,
            scalar_v257,
            scalar_v277,
            scalar_v297,
            scalar_v339,
            scalar_v359,
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
            scalar_v4,
            scalar_v6,
            scalar_v8,
            scalar_v12,
            scalar_v14,
            scalar_v16,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v28,
            scalar_v29,
            scalar_v32,
            scalar_v34,
            scalar_v37,
            scalar_v38,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v65,
            scalar_v68,
            scalar_v69,
            scalar_v72,
            scalar_v75,
            scalar_v76,
            scalar_v83,
            scalar_v84,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v104,
            scalar_v105,
            scalar_v113,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v120,
            scalar_v121,
            scalar_v129,
            scalar_v130,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v141,
            scalar_v142,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v164,
            scalar_v165,
            scalar_v176,
            scalar_v177,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v200,
            scalar_v201,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v233,
            scalar_v234,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v254,
            scalar_v255,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v274,
            scalar_v275,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v294,
            scalar_v295,
            scalar_v309,
            scalar_v320,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v336,
            scalar_v337,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v356,
            scalar_v357,
            scalar_v393,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v402,
            scalar_v403,
            scalar_v407,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v422,
            scalar_v423,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v433,
            scalar_v436,
            scalar_v437,
            scalar_v440,
            scalar_v441,
            scalar_v444,
            scalar_v445,
            scalar_v446,
            scalar_v450,
            scalar_v453,
            scalar_v454,
            scalar_v526,
            scalar_v531,
            scalar_v532,
            scalar_v536,
            scalar_v537,
            scalar_v541,
            scalar_v542,
            scalar_v548,
            scalar_v555,
            scalar_v556,
            scalar_v561,
            scalar_v589,
            scalar_v590,
            scalar_v594,
            scalar_v616,
            scalar_v617,
            scalar_v621,
            scalar_v643,
            scalar_v645,
            scalar_v648,
            scalar_v650,
            scalar_v653,
            scalar_v655,
            scalar_v657,
            scalar_v660,
            scalar_v666,
            scalar_v667,
            scalar_v674,
            scalar_v675,
            scalar_v679,
            scalar_v680,
            scalar_v764,
            scalar_v766,
            scalar_v767,
            scalar_v772,
            scalar_v773,
            scalar_v774,
            scalar_v779,
            scalar_v782,
            scalar_v802,
            scalar_v804,
            scalar_v805,
            scalar_v845,
            scalar_v846,
            scalar_v851,
            scalar_v852,
            scalar_v858,
            scalar_v861,
            scalar_v868,
            scalar_v869,
            scalar_v870,
            scalar_v901,
            scalar_v902,
            scalar_v903,
            scalar_v904,
            scalar_v905,
            scalar_v913,
            scalar_v914,
            scalar_v919,
            scalar_v920,
            scalar_v970,
            scalar_v989,
            scalar_v990,
            scalar_v1019,
            scalar_v1075,
            scalar_v1076,
            scalar_v1077,
            scalar_v1090,
            scalar_v1094,
            scalar_v1106,
            scalar_v1111,
            scalar_v1140,
            scalar_v1142,
            scalar_v1181,
            scalar_v1185,
            scalar_v1186,
            scalar_v1219,
            scalar_v1220,
            scalar_v1221,
            scalar_v1232,
            scalar_v1233,
            scalar_v1237,
            scalar_v1238,
            scalar_v1261,
            scalar_v1262,
            scalar_v1263,
            scalar_v1301,
            scalar_v1322,
            scalar_v1323,
            scalar_v1348,
            scalar_v1356,
            scalar_v1361,
            scalar_v1378,
            scalar_v1407,
            scalar_v1431,
            scalar_v1467,
            scalar_v1468,
            scalar_v1469,
            scalar_v1506,
            scalar_v1564,
            scalar_v1565,
            scalar_v1570,
            scalar_v1571,
            scalar_v1584,
            scalar_v1606,
            scalar_v1608,
            scalar_v1609,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1627,
            scalar_v1648,
            scalar_v1650,
            scalar_v1651,
            scalar_v1652,
            scalar_v1653,
            scalar_v1654,
            scalar_v1655,
            scalar_v1656,
            scalar_v1657,
            scalar_v1672,
            scalar_v1673,
            scalar_v1677,
            scalar_v1681,
            scalar_v1685,
            scalar_v1686,
            scalar_v1687,
            scalar_v1726,
            scalar_v1755,
            scalar_v1756,
            scalar_v1783,
            scalar_v1787,
            scalar_v1788,
            scalar_v1789,
            scalar_v1794,
            scalar_v1795,
            scalar_v1800,
            scalar_v1803,
            scalar_v1823,
            scalar_v1824,
            scalar_v1826,
            scalar_v1827,
            scalar_v1865,
            scalar_v2045,
            scalar_v2046,
            scalar_v2050,
            scalar_v2066,
            scalar_v2069,
            scalar_v2077,
            scalar_v2080,
            scalar_v2082,
            scalar_v2084,
            scalar_v2086,
            scalar_v2105,
            scalar_v2110,
            scalar_v2116,
            scalar_v2122,
            scalar_v2130,
            scalar_v2136,
            scalar_v2142,
            scalar_v2150,
            scalar_v2155,
            scalar_v2160,
            scalar_v2170,
            scalar_v2187,
            scalar_v2203,
            scalar_v2219,
            scalar_v2235,
            scalar_v2251,
            scalar_v2267,
            scalar_v2291,
            scalar_v2307,
            scalar_v2468,
            scalar_v2486,
            scalar_v2491,
            scalar_v2504,
            scalar_v2614,
            scalar_v2782,
            scalar_v2993,
            scalar_v3247,
            scalar_v3266,
            scalar_v4595,
            scalar_v4622,
            scalar_v4704,
            scalar_v4731,
            scalar_v4833,
            scalar_v5291,
            scalar_v6021,
            scalar_v6022,
            scalar_v6023,
            scalar_v73,
            scalar_v74,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v95,
            scalar_v97,
            scalar_v103,
            scalar_v106,
            scalar_v107,
            scalar_v124,
            scalar_v167,
            scalar_v203,
            scalar_v236,
            scalar_v257,
            scalar_v277,
            scalar_v297,
            scalar_v339,
            scalar_v359,
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
            "trise" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dta" => { validate_finite_parameter("trise", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_noise" => { validate_parameter("sw_noise", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sw_et" => { validate_parameter("sw_et", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npn" => { validate_finite_parameter("npn", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnp" => { validate_finite_parameter("pnp", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scale" => { validate_parameter("scale", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shrink" => { validate_parameter("shrink", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("tmin", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("tmax", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gmin" => { validate_parameter("gmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnjmaxi" => { validate_parameter("pnjmaxi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "maxexp" => { validate_parameter("maxexp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("tnom", value, Some((-250.0, "-250.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tminclip" => { validate_parameter("tminclip", value, Some((-250.0, "-250.0")), false, Some((27.0, "27.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmaxclip" => { validate_parameter("tmaxclip", value, Some((27.0, "27.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("rcx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci" => { validate_parameter("rci", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vo" => { validate_parameter("vo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamm" => { validate_parameter("gamm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hrcf" => { validate_parameter("hrcf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("rbx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi" => { validate_parameter("rbi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("rs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbp" => { validate_parameter("rbp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isrr" => { validate_parameter("isrr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nr" => { validate_parameter("nr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qbm" => { validate_parameter("qbm", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isp" => { validate_parameter("isp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsp" => { validate_parameter("wsp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfp" => { validate_parameter("nfp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "me" => { validate_parameter("me", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aje" => { validate_finite_parameter("aje", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajc" => { validate_finite_parameter("ajc", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vrt" => { validate_parameter("vrt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "art" => { validate_parameter("art", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qco" => { validate_parameter("qco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjep" => { validate_parameter("cjep", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcp" => { validate_parameter("cjcp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("ps", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ms" => { validate_parameter("ms", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajs" => { validate_finite_parameter("ajs", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccso" => { validate_parameter("ccso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibei" => { validate_parameter("ibei", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbe" => { validate_parameter("wbe", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nei" => { validate_parameter("nei", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qnibeir" => { validate_parameter("qnibeir", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iben" => { validate_parameter("iben", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nen" => { validate_finite_parameter("nen", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibci" => { validate_parameter("ibci", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nci" => { validate_parameter("nci", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcn" => { validate_parameter("ibcn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncn" => { validate_finite_parameter("ncn", value)?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeip" => { validate_parameter("ibeip", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibenp" => { validate_parameter("ibenp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcip" => { validate_parameter("ibcip", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncip" => { validate_parameter("ncip", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcnp" => { validate_parameter("ibcnp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncnp" => { validate_finite_parameter("ncnp", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikf" => { validate_parameter("ikf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nkf" => { validate_parameter("nkf", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikr" => { validate_parameter("ikr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikp" => { validate_parameter("ikp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tf" => { validate_parameter("tf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qtf" => { validate_parameter("qtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xtf" => { validate_parameter("xtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtf" => { validate_parameter("vtf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itf" => { validate_parameter("itf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("tr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td" => { validate_parameter("td", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc1" => { validate_parameter("avc1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc2" => { validate_parameter("avc2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcx1" => { validate_parameter("avcx1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avcx2" => { validate_parameter("avcx2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mcx" => { validate_parameter("mcx", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), false, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbbe" => { validate_parameter("vbbe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbbe" => { validate_parameter("nbbe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibbe" => { validate_parameter("ibbe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe1" => { validate_finite_parameter("tvbbe1", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe2" => { validate_finite_parameter("tvbbe2", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnbbe" => { validate_finite_parameter("tnbbe", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpte" => { validate_parameter("vpte", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibk0" => { validate_parameter("ibk0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "abk" => { validate_parameter("abk", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bbk" => { validate_parameter("bbk", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("bfn", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xre" => { validate_finite_parameter("xre", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrb" => { validate_finite_parameter("xrb", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbi" => { validate_finite_parameter("xrbi", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbx" => { validate_finite_parameter("xrbx", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrc" => { validate_finite_parameter("xrc", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrci" => { validate_finite_parameter("xrci", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrcx" => { validate_finite_parameter("xrcx", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbp" => { validate_finite_parameter("xrbp", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrs" => { validate_finite_parameter("xrs", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xvo" => { validate_finite_parameter("xvo", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaie" => { validate_finite_parameter("eaie", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaic" => { validate_finite_parameter("eaic", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eais" => { validate_finite_parameter("eais", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eane" => { validate_finite_parameter("eane", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eanc" => { validate_finite_parameter("eanc", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eans" => { validate_finite_parameter("eans", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eap" => { validate_finite_parameter("eap", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dear" => { validate_finite_parameter("dear", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xis" => { validate_finite_parameter("xis", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xii" => { validate_finite_parameter("xii", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xin" => { validate_finite_parameter("xin", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xisr" => { validate_finite_parameter("xisr", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xikf" => { validate_finite_parameter("xikf", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tavc" => { validate_finite_parameter("tavc", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tavcx" => { validate_finite_parameter("tavcx", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnf" => { validate_finite_parameter("tnf", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcvef" => { validate_finite_parameter("tcvef", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcver" => { validate_finite_parameter("tcver", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcrth" => { validate_finite_parameter("tcrth", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic13_4t'", name)),
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
        let v4: f64 = if param_given[10] { 1.0 } else { 0.0 };
        self.scalar_v4 = v4;
        let v6: f64 = p.p10;
        self.scalar_v6 = v6;
        let v8: bool = (!(if param_given[10] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v8 = v8;
        let v12: f64 = if param_given[11] { 1.0 } else { 0.0 };
        self.scalar_v12 = v12;
        let v14: f64 = p.p11;
        self.scalar_v14 = v14;
        let v16: bool = (!(if param_given[11] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v16 = v16;
        let v19: f64 = if param_given[3] { 1.0 } else { 0.0 };
        self.scalar_v19 = v19;
        let v22: f64 = if param_given[4] { 1.0 } else { 0.0 };
        self.scalar_v22 = v22;
        let v23: bool = (!(if param_given[3] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v23 = v23;
        let v28: f64 = if param_given[5] { 1.0 } else { 0.0 };
        self.scalar_v28 = v28;
        let v29: bool = (!(if param_given[4] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v29 = v29;
        let v32: f64 = p.p5;
        self.scalar_v32 = v32;
        let v34: bool = (!(if param_given[5] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v34 = v34;
        let v37: f64 = p.p12;
        self.scalar_v37 = v37;
        let v38: f64 = ((p.p12) as f64).ln();
        self.scalar_v38 = v38;
        let v40: f64 = p.p74;
        self.scalar_v40 = v40;
        let v41: bool = (p.p74 > 0.0);
        self.scalar_v41 = v41;
        let v42: f64 = (1.0 / p.p74);
        self.scalar_v42 = v42;
        let v43: f64 = (if v41 { v42 } else { 0.0 });
        self.scalar_v43 = v43;
        let v45: f64 = p.p75;
        self.scalar_v45 = v45;
        let v46: bool = (p.p75 > 0.0);
        self.scalar_v46 = v46;
        let v47: f64 = (1.0 / p.p75);
        self.scalar_v47 = v47;
        let v48: f64 = (if v46 { v47 } else { 0.0 });
        self.scalar_v48 = v48;
        let v50: f64 = p.p20;
        self.scalar_v50 = v50;
        let v51: bool = (p.p20 > 0.0);
        self.scalar_v51 = v51;
        let v52: f64 = (1.0 / p.p20);
        self.scalar_v52 = v52;
        let v53: f64 = (if v51 { v52 } else { 0.0 });
        self.scalar_v53 = v53;
        let v55: f64 = p.p79;
        self.scalar_v55 = v55;
        let v56: bool = (p.p79 > 0.0);
        self.scalar_v56 = v56;
        let v57: f64 = (1.0 / p.p79);
        self.scalar_v57 = v57;
        let v58: f64 = (if v56 { v57 } else { 0.0 });
        self.scalar_v58 = v58;
        let v60: f64 = p.p80;
        self.scalar_v60 = v60;
        let v61: bool = (p.p80 > 0.0);
        self.scalar_v61 = v61;
        let v62: f64 = (1.0 / p.p80);
        self.scalar_v62 = v62;
        let v63: f64 = (if v61 { v62 } else { 0.0 });
        self.scalar_v63 = v63;
        let v65: f64 = (if v61 { 0.0 } else { 1.0 });
        self.scalar_v65 = v65;
        let v68: f64 = p.p13;
        self.scalar_v68 = v68;
        let v69: f64 = (273.15 + p.p13);
        self.scalar_v69 = v69;
        let v72: f64 = p.p0;
        self.scalar_v72 = v72;
        let v75: f64 = p.p14;
        self.scalar_v75 = v75;
        let v76: f64 = (1.0 + p.p14);
        self.scalar_v76 = v76;
        let v83: f64 = p.p15;
        self.scalar_v83 = v83;
        let v84: f64 = (p.p15 - 1.0);
        self.scalar_v84 = v84;
        let v99: f64 = p.p26;
        self.scalar_v99 = v99;
        let v100: f64 = p.p90;
        self.scalar_v100 = v100;
        let v101: bool = (p.p90 > 0.0);
        self.scalar_v101 = v101;
        let v102: f64 = p.p89;
        self.scalar_v102 = v102;
        let v104: f64 = p.p88;
        self.scalar_v104 = v104;
        let v105: f64 = (-p.p88);
        self.scalar_v105 = v105;
        let v113: bool = (!v101);
        self.scalar_v113 = v113;
        let v115: f64 = p.p122;
        self.scalar_v115 = v115;
        let v116: f64 = p.p28;
        self.scalar_v116 = v116;
        let v117: f64 = (p.p122 / p.p28);
        self.scalar_v117 = v117;
        let v120: f64 = p.p113;
        self.scalar_v120 = v120;
        let v121: f64 = (-p.p113);
        self.scalar_v121 = v121;
        let v129: f64 = p.p72;
        self.scalar_v129 = v129;
        let v130: bool = (p.p72 > 0.0);
        self.scalar_v130 = v130;
        let v137: f64 = (4.0 / p.p72);
        self.scalar_v137 = v137;
        let v138: f64 = p.p73;
        self.scalar_v138 = v138;
        let v139: f64 = f64::powf(v137, p.p73);
        self.scalar_v139 = v139;
        let v141: f64 = (1.0 - p.p73);
        self.scalar_v141 = v141;
        let v142: f64 = (1.0 / v141);
        self.scalar_v142 = v142;
        let v158: f64 = p.p27;
        self.scalar_v158 = v158;
        let v159: f64 = p.p125;
        self.scalar_v159 = v159;
        let v160: f64 = p.p29;
        self.scalar_v160 = v160;
        let v161: f64 = (p.p125 / p.p29);
        self.scalar_v161 = v161;
        let v164: f64 = p.p121;
        self.scalar_v164 = v164;
        let v165: f64 = (-p.p121);
        self.scalar_v165 = v165;
        let v176: f64 = (4.0 / p.p74);
        self.scalar_v176 = v176;
        let v177: f64 = f64::powf(v176, p.p73);
        self.scalar_v177 = v177;
        let v195: f64 = p.p31;
        self.scalar_v195 = v195;
        let v196: f64 = p.p33;
        self.scalar_v196 = v196;
        let v197: f64 = (p.p122 / p.p33);
        self.scalar_v197 = v197;
        let v200: f64 = p.p120;
        self.scalar_v200 = v200;
        let v201: f64 = (-p.p120);
        self.scalar_v201 = v201;
        let v227: f64 = p.p54;
        self.scalar_v227 = v227;
        let v228: f64 = p.p123;
        self.scalar_v228 = v228;
        let v229: f64 = p.p56;
        self.scalar_v229 = v229;
        let v230: f64 = (p.p123 / p.p56);
        self.scalar_v230 = v230;
        let v233: f64 = p.p114;
        self.scalar_v233 = v233;
        let v234: f64 = (-p.p114);
        self.scalar_v234 = v234;
        let v248: f64 = p.p58;
        self.scalar_v248 = v248;
        let v249: f64 = p.p124;
        self.scalar_v249 = v249;
        let v250: f64 = p.p59;
        self.scalar_v250 = v250;
        let v251: f64 = (p.p124 / p.p59);
        self.scalar_v251 = v251;
        let v254: f64 = p.p117;
        self.scalar_v254 = v254;
        let v255: f64 = (-p.p117);
        self.scalar_v255 = v255;
        let v269: f64 = p.p60;
        self.scalar_v269 = v269;
        let v270: f64 = p.p61;
        self.scalar_v270 = v270;
        let v271: f64 = (p.p123 / p.p61);
        self.scalar_v271 = v271;
        let v274: f64 = p.p115;
        self.scalar_v274 = v274;
        let v275: f64 = (-p.p115);
        self.scalar_v275 = v275;
        let v289: f64 = p.p62;
        self.scalar_v289 = v289;
        let v290: f64 = p.p63;
        self.scalar_v290 = v290;
        let v291: f64 = (p.p124 / p.p63);
        self.scalar_v291 = v291;
        let v294: f64 = p.p118;
        self.scalar_v294 = v294;
        let v295: f64 = (-p.p118);
        self.scalar_v295 = v295;
        let v309: f64 = p.p64;
        self.scalar_v309 = v309;
        let v320: f64 = p.p65;
        self.scalar_v320 = v320;
        let v331: f64 = p.p66;
        self.scalar_v331 = v331;
        let v332: f64 = p.p67;
        self.scalar_v332 = v332;
        let v333: f64 = (p.p123 / p.p67);
        self.scalar_v333 = v333;
        let v336: f64 = p.p116;
        self.scalar_v336 = v336;
        let v337: f64 = (-p.p116);
        self.scalar_v337 = v337;
        let v351: f64 = p.p68;
        self.scalar_v351 = v351;
        let v352: f64 = p.p69;
        self.scalar_v352 = v352;
        let v353: f64 = (p.p124 / p.p69);
        self.scalar_v353 = v353;
        let v356: f64 = p.p119;
        self.scalar_v356 = v356;
        let v357: f64 = (-p.p119);
        self.scalar_v357 = v357;
        let v393: f64 = p.p126;
        self.scalar_v393 = v393;
        let v396: f64 = if param_given[109] { 1.0 } else { 0.0 };
        self.scalar_v396 = v396;
        let v397: f64 = p.p16;
        self.scalar_v397 = v397;
        let v398: f64 = p.p109;
        self.scalar_v398 = v398;
        let v402: bool = (!(if param_given[109] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v402 = v402;
        let v403: f64 = p.p107;
        self.scalar_v403 = v403;
        let v407: f64 = if param_given[108] { 1.0 } else { 0.0 };
        self.scalar_v407 = v407;
        let v408: f64 = p.p17;
        self.scalar_v408 = v408;
        let v409: f64 = p.p108;
        self.scalar_v409 = v409;
        let v413: bool = (!(if param_given[108] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v413 = v413;
        let v416: f64 = if param_given[106] { 1.0 } else { 0.0 };
        self.scalar_v416 = v416;
        let v417: f64 = p.p21;
        self.scalar_v417 = v417;
        let v418: f64 = p.p106;
        self.scalar_v418 = v418;
        let v422: bool = (!(if param_given[106] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v422 = v422;
        let v423: f64 = p.p104;
        self.scalar_v423 = v423;
        let v427: f64 = if param_given[105] { 1.0 } else { 0.0 };
        self.scalar_v427 = v427;
        let v428: f64 = p.p22;
        self.scalar_v428 = v428;
        let v429: f64 = p.p105;
        self.scalar_v429 = v429;
        let v433: bool = (!(if param_given[105] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v433 = v433;
        let v436: f64 = p.p23;
        self.scalar_v436 = v436;
        let v437: f64 = p.p103;
        self.scalar_v437 = v437;
        let v440: f64 = p.p24;
        self.scalar_v440 = v440;
        let v441: f64 = p.p111;
        self.scalar_v441 = v441;
        let v444: f64 = if param_given[110] { 1.0 } else { 0.0 };
        self.scalar_v444 = v444;
        let v445: f64 = p.p25;
        self.scalar_v445 = v445;
        let v446: f64 = p.p110;
        self.scalar_v446 = v446;
        let v450: bool = (!(if param_given[110] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v450 = v450;
        let v453: f64 = p.p101;
        self.scalar_v453 = v453;
        let v454: f64 = p.p132;
        self.scalar_v454 = v454;
        let v526: f64 = p.p129;
        self.scalar_v526 = v526;
        let v531: f64 = p.p84;
        self.scalar_v531 = v531;
        let v532: f64 = p.p127;
        self.scalar_v532 = v532;
        let v536: f64 = p.p86;
        self.scalar_v536 = v536;
        let v537: f64 = p.p128;
        self.scalar_v537 = v537;
        let v541: f64 = p.p91;
        self.scalar_v541 = v541;
        let v542: f64 = p.p92;
        self.scalar_v542 = v542;
        let v548: f64 = p.p93;
        self.scalar_v548 = v548;
        let v555: f64 = p.p37;
        self.scalar_v555 = v555;
        let v556: f64 = (0.5 * p.p37);
        self.scalar_v556 = v556;
        let v561: f64 = (p.p37 * -0.5);
        self.scalar_v561 = v561;
        let v589: f64 = p.p42;
        self.scalar_v589 = v589;
        let v590: f64 = (0.5 * p.p42);
        self.scalar_v590 = v590;
        let v594: f64 = (-0.5 * p.p42);
        self.scalar_v594 = v594;
        let v616: f64 = p.p50;
        self.scalar_v616 = v616;
        let v617: f64 = (0.5 * p.p50);
        self.scalar_v617 = v617;
        let v621: f64 = (-0.5 * p.p50);
        self.scalar_v621 = v621;
        let v643: f64 = p.p36;
        self.scalar_v643 = v643;
        let v645: f64 = p.p38;
        self.scalar_v645 = v645;
        let v648: f64 = p.p41;
        self.scalar_v648 = v648;
        let v650: f64 = p.p43;
        self.scalar_v650 = v650;
        let v653: f64 = p.p48;
        self.scalar_v653 = v653;
        let v655: f64 = p.p49;
        self.scalar_v655 = v655;
        let v657: f64 = p.p51;
        self.scalar_v657 = v657;
        let v660: f64 = p.p19;
        self.scalar_v660 = v660;
        let v666: f64 = p.p18;
        self.scalar_v666 = v666;
        let v667: f64 = p.p112;
        self.scalar_v667 = v667;
        let v674: f64 = p.p70;
        self.scalar_v674 = v674;
        let v675: f64 = p.p130;
        self.scalar_v675 = v675;
        let v679: f64 = p.p71;
        self.scalar_v679 = v679;
        let v680: f64 = p.p131;
        self.scalar_v680 = v680;
        let v764: f64 = p.p34;
        self.scalar_v764 = v764;
        let v766: f64 = p.p39;
        self.scalar_v766 = v766;
        let v767: bool = (p.p39 <= 0.0);
        self.scalar_v767 = v767;
        let v772: f64 = (1.0 - p.p34);
        self.scalar_v772 = v772;
        let v773: f64 = (-p.p38);
        self.scalar_v773 = v773;
        let v774: f64 = f64::powf(v772, v773);
        self.scalar_v774 = v774;
        let v779: f64 = (1.0 - p.p38);
        self.scalar_v779 = v779;
        let v782: f64 = (0.5 * p.p38);
        self.scalar_v782 = v782;
        let v802: bool = (!v767);
        self.scalar_v802 = v802;
        let v804: f64 = (4.0 * p.p39);
        self.scalar_v804 = v804;
        let v805: f64 = (p.p39 * v804);
        self.scalar_v805 = v805;
        let v845: f64 = p.p44;
        self.scalar_v845 = v845;
        let v846: bool = (p.p44 <= 0.0);
        self.scalar_v846 = v846;
        let v851: f64 = (-1.0 - p.p43);
        self.scalar_v851 = v851;
        let v852: f64 = f64::powf(v772, v851);
        self.scalar_v852 = v852;
        let v858: f64 = (1.0 - p.p43);
        self.scalar_v858 = v858;
        let v861: f64 = (0.5 * p.p43);
        self.scalar_v861 = v861;
        let v868: f64 = p.p45;
        self.scalar_v868 = v868;
        let v869: bool = (p.p45 > 0.0);
        self.scalar_v869 = v869;
        let v870: f64 = (-p.p45);
        self.scalar_v870 = v870;
        let v901: f64 = p.p46;
        self.scalar_v901 = v901;
        let v902: bool = (p.p46 > 0.0);
        self.scalar_v902 = v902;
        let v903: bool = (v869 && v902);
        self.scalar_v903 = v903;
        let v904: bool = (!v846);
        self.scalar_v904 = v904;
        let v905: bool = (v903 && v904);
        self.scalar_v905 = v905;
        let v913: f64 = (4.0 * p.p44);
        self.scalar_v913 = v913;
        let v914: f64 = (p.p44 * v913);
        self.scalar_v914 = v914;
        let v919: f64 = (4.0 * p.p46);
        self.scalar_v919 = v919;
        let v920: f64 = (p.p46 * v919);
        self.scalar_v920 = v920;
        let v970: f64 = (-p.p43);
        self.scalar_v970 = v970;
        let v989: bool = (!v903);
        self.scalar_v989 = v989;
        let v990: bool = (v904 && v989);
        self.scalar_v990 = v990;
        let v1019: f64 = f64::powf(v772, v970);
        self.scalar_v1019 = v1019;
        let v1075: f64 = p.p30;
        self.scalar_v1075 = v1075;
        let v1076: bool = (p.p30 < 0.5);
        self.scalar_v1076 = v1076;
        let v1077: f64 = (1.0 / p.p73);
        self.scalar_v1077 = v1077;
        let v1090: f64 = f64::powf(1e-8, p.p73);
        self.scalar_v1090 = v1090;
        let v1094: bool = (!v1076);
        self.scalar_v1094 = v1094;
        let v1106: f64 = (1.0 + v1090);
        self.scalar_v1106 = v1106;
        let v1111: bool = (p.p31 > 0.0);
        self.scalar_v1111 = v1111;
        let v1140: f64 = p.p32;
        self.scalar_v1140 = v1140;
        let v1142: f64 = (1.0 - p.p32);
        self.scalar_v1142 = v1142;
        let v1181: bool = (!v1111);
        self.scalar_v1181 = v1181;
        let v1185: f64 = p.p55;
        self.scalar_v1185 = v1185;
        let v1186: bool = (1.0 == p.p55);
        self.scalar_v1186 = v1186;
        let v1219: f64 = p.p57;
        self.scalar_v1219 = v1219;
        let v1220: bool = (p.p57 > 0.0);
        self.scalar_v1220 = v1220;
        let v1221: bool = (v1186 && v1220);
        self.scalar_v1221 = v1221;
        let v1232: bool = (!v1220);
        self.scalar_v1232 = v1232;
        let v1233: bool = (v1186 && v1232);
        self.scalar_v1233 = v1233;
        let v1237: bool = (p.p88 > 0.0);
        self.scalar_v1237 = v1237;
        let v1238: bool = (v1186 && v1237);
        self.scalar_v1238 = v1238;
        let v1261: bool = (0.0 == p.p55);
        self.scalar_v1261 = v1261;
        let v1262: bool = (!v1186);
        self.scalar_v1262 = v1262;
        let v1263: bool = (v1261 && v1262);
        self.scalar_v1263 = v1263;
        let v1301: bool = (v1237 && v1263);
        self.scalar_v1301 = v1301;
        let v1322: bool = (!v1261);
        self.scalar_v1322 = v1322;
        let v1323: bool = (v1262 && v1322);
        self.scalar_v1323 = v1323;
        let v1348: bool = (v1220 && v1323);
        self.scalar_v1348 = v1348;
        let v1356: bool = (v1232 && v1323);
        self.scalar_v1356 = v1356;
        let v1361: bool = (v1237 && v1323);
        self.scalar_v1361 = v1361;
        let v1378: f64 = (p.p90 * p.p55);
        self.scalar_v1378 = v1378;
        let v1407: f64 = (1.0 - p.p55);
        self.scalar_v1407 = v1407;
        let v1431: f64 = (p.p90 * v1407);
        self.scalar_v1431 = v1431;
        let v1467: bool = (p.p64 > 0.0);
        self.scalar_v1467 = v1467;
        let v1468: bool = (p.p65 > 0.0);
        self.scalar_v1468 = v1468;
        let v1469: bool = (v1467 || v1468);
        self.scalar_v1469 = v1469;
        let v1506: bool = (!v1469);
        self.scalar_v1506 = v1506;
        let v1564: f64 = p.p83;
        self.scalar_v1564 = v1564;
        let v1565: bool = (p.p83 > 0.0);
        self.scalar_v1565 = v1565;
        let v1570: f64 = (1.01 - p.p43);
        self.scalar_v1570 = v1570;
        let v1571: f64 = (1.0 / v1570);
        self.scalar_v1571 = v1571;
        let v1584: f64 = (p.p43 - 1.0);
        self.scalar_v1584 = v1584;
        let v1606: bool = (!v1565);
        self.scalar_v1606 = v1606;
        let v1608: f64 = p.p85;
        self.scalar_v1608 = v1608;
        let v1609: bool = (p.p85 > 0.0);
        self.scalar_v1609 = v1609;
        let v1612: f64 = p.p87;
        self.scalar_v1612 = v1612;
        let v1613: f64 = (1.01 - p.p87);
        self.scalar_v1613 = v1613;
        let v1614: f64 = (1.0 / v1613);
        self.scalar_v1614 = v1614;
        let v1627: f64 = (p.p87 - 1.0);
        self.scalar_v1627 = v1627;
        let v1648: bool = (!v1609);
        self.scalar_v1648 = v1648;
        let v1650: f64 = p.p97;
        self.scalar_v1650 = v1650;
        let v1651: bool = (p.p97 > 0.0);
        self.scalar_v1651 = v1651;
        let v1652: f64 = p.p95;
        self.scalar_v1652 = v1652;
        let v1653: bool = (p.p95 > 0.0);
        self.scalar_v1653 = v1653;
        let v1654: bool = (v1651 && v1653);
        self.scalar_v1654 = v1654;
        let v1655: f64 = p.p94;
        self.scalar_v1655 = v1655;
        let v1656: bool = (p.p94 > 0.0);
        self.scalar_v1656 = v1656;
        let v1657: bool = (v1654 && v1656);
        self.scalar_v1657 = v1657;
        let v1672: bool = (!v1656);
        self.scalar_v1672 = v1672;
        let v1673: bool = (v1654 && v1672);
        self.scalar_v1673 = v1673;
        let v1677: f64 = p.p96;
        self.scalar_v1677 = v1677;
        let v1681: bool = (!v1654);
        self.scalar_v1681 = v1681;
        let v1685: bool = (p.p66 > 0.0);
        self.scalar_v1685 = v1685;
        let v1686: bool = (p.p68 > 0.0);
        self.scalar_v1686 = v1686;
        let v1687: bool = (v1685 || v1686);
        self.scalar_v1687 = v1687;
        let v1726: bool = (!v1687);
        self.scalar_v1726 = v1726;
        let v1755: f64 = p.p2;
        self.scalar_v1755 = v1755;
        let v1756: f64 = (-p.p2);
        self.scalar_v1756 = v1756;
        let v1783: bool = (p.p49 > 0.0);
        self.scalar_v1783 = v1783;
        let v1787: f64 = p.p52;
        self.scalar_v1787 = v1787;
        let v1788: bool = (p.p52 <= 0.0);
        self.scalar_v1788 = v1788;
        let v1789: bool = (v1783 && v1788);
        self.scalar_v1789 = v1789;
        let v1794: f64 = (-p.p51);
        self.scalar_v1794 = v1794;
        let v1795: f64 = f64::powf(v772, v1794);
        self.scalar_v1795 = v1795;
        let v1800: f64 = (1.0 - p.p51);
        self.scalar_v1800 = v1800;
        let v1803: f64 = (0.5 * p.p51);
        self.scalar_v1803 = v1803;
        let v1823: bool = (!v1788);
        self.scalar_v1823 = v1823;
        let v1824: bool = (v1783 && v1823);
        self.scalar_v1824 = v1824;
        let v1826: f64 = (4.0 * p.p52);
        self.scalar_v1826 = v1826;
        let v1827: f64 = (p.p52 * v1826);
        self.scalar_v1827 = v1827;
        let v1865: bool = (!v1783);
        self.scalar_v1865 = v1865;
        let v2045: f64 = p.p76;
        self.scalar_v2045 = v2045;
        let v2046: f64 = p.p77;
        self.scalar_v2046 = v2046;
        let v2050: f64 = p.p78;
        self.scalar_v2050 = v2050;
        let v2066: f64 = p.p81;
        self.scalar_v2066 = v2066;
        let v2069: f64 = p.p47;
        self.scalar_v2069 = v2069;
        let v2077: f64 = p.p53;
        self.scalar_v2077 = v2077;
        let v2080: f64 = p.p35;
        self.scalar_v2080 = v2080;
        let v2082: f64 = p.p40;
        self.scalar_v2082 = v2082;
        let v2084: f64 = p.p102;
        self.scalar_v2084 = v2084;
        let v2086: f64 = p.p82;
        self.scalar_v2086 = v2086;
        let v2105: f64 = (p.p126 - 1.0);
        self.scalar_v2105 = v2105;
        let v2110: f64 = (p.p109 - 1.0);
        self.scalar_v2110 = v2110;
        let v2116: f64 = (p.p107 - 1.0);
        self.scalar_v2116 = v2116;
        let v2122: f64 = (p.p108 - 1.0);
        self.scalar_v2122 = v2122;
        let v2130: f64 = (p.p106 - 1.0);
        self.scalar_v2130 = v2130;
        let v2136: f64 = (p.p104 - 1.0);
        self.scalar_v2136 = v2136;
        let v2142: f64 = (p.p105 - 1.0);
        self.scalar_v2142 = v2142;
        let v2150: f64 = (p.p103 - 1.0);
        self.scalar_v2150 = v2150;
        let v2155: f64 = (p.p111 - 1.0);
        self.scalar_v2155 = v2155;
        let v2160: f64 = (p.p110 - 1.0);
        self.scalar_v2160 = v2160;
        let v2170: f64 = (v117 - 1.0);
        self.scalar_v2170 = v2170;
        let v2187: f64 = (v161 - 1.0);
        self.scalar_v2187 = v2187;
        let v2203: f64 = (v197 - 1.0);
        self.scalar_v2203 = v2203;
        let v2219: f64 = (v230 - 1.0);
        self.scalar_v2219 = v2219;
        let v2235: f64 = (v251 - 1.0);
        self.scalar_v2235 = v2235;
        let v2251: f64 = (v271 - 1.0);
        self.scalar_v2251 = v2251;
        let v2267: f64 = (v291 - 1.0);
        self.scalar_v2267 = v2267;
        let v2291: f64 = (v333 - 1.0);
        self.scalar_v2291 = v2291;
        let v2307: f64 = (v353 - 1.0);
        self.scalar_v2307 = v2307;
        let v2468: f64 = (p.p38 - 1.0);
        self.scalar_v2468 = v2468;
        let v2486: f64 = (p.p51 - 1.0);
        self.scalar_v2486 = v2486;
        let v2491: f64 = (p.p122 - 1.0);
        self.scalar_v2491 = v2491;
        let v2504: f64 = (p.p112 - 1.0);
        self.scalar_v2504 = v2504;
        let v2614: f64 = (v779 - 1.0);
        self.scalar_v2614 = v2614;
        let v2782: f64 = (v858 - 1.0);
        self.scalar_v2782 = v2782;
        let v2993: f64 = (v970 - 1.0);
        self.scalar_v2993 = v2993;
        let v3247: f64 = (v1077 - 1.0);
        self.scalar_v3247 = v3247;
        let v3266: f64 = (p.p73 - 1.0);
        self.scalar_v3266 = v3266;
        let v4595: f64 = (v1571 - 1.0);
        self.scalar_v4595 = v4595;
        let v4622: f64 = (v1584 - 1.0);
        self.scalar_v4622 = v4622;
        let v4704: f64 = (v1614 - 1.0);
        self.scalar_v4704 = v4704;
        let v4731: f64 = (v1627 - 1.0);
        self.scalar_v4731 = v4731;
        let v4833: f64 = (p.p96 - 1.0);
        self.scalar_v4833 = v4833;
        let v5291: f64 = (v1800 - 1.0);
        self.scalar_v5291 = v5291;
        let v6021: f64 = (-p.p35);
        self.scalar_v6021 = v6021;
        let v6022: f64 = (-p.p40);
        self.scalar_v6022 = v6022;
        let v6023: f64 = (p.p82 * 0.3333333333333333);
        self.scalar_v6023 = v6023;
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
        let v73: f64 = (temperature + self.scalar_v72);
        self.scalar_v73 = v73;
        let v74: f64 = (self.scalar_v73 - 273.15);
        self.scalar_v74 = v74;
        let v77: bool = (self.scalar_v74 < self.scalar_v76);
        self.scalar_v77 = v77;
        let v78: f64 = (self.scalar_v74 - self.scalar_v75);
        self.scalar_v78 = v78;
        let v79: f64 = (self.scalar_v78 - 1.0);
        self.scalar_v79 = v79;
        let v80: f64 = ((self.scalar_v79) as f64).exp();
        self.scalar_v80 = v80;
        let v81: f64 = (self.scalar_v75 + self.scalar_v80);
        self.scalar_v81 = v81;
        let v82: f64 = (if self.scalar_v77 { self.scalar_v81 } else { self.scalar_v74 });
        self.scalar_v82 = v82;
        let v85: bool = (self.scalar_v82 > self.scalar_v84);
        self.scalar_v85 = v85;
        let v86: bool = (!self.scalar_v77);
        self.scalar_v86 = v86;
        let v87: bool = (self.scalar_v85 && self.scalar_v86);
        self.scalar_v87 = v87;
        let v88: f64 = (self.scalar_v83 - self.scalar_v82);
        self.scalar_v88 = v88;
        let v89: f64 = (self.scalar_v88 - 1.0);
        self.scalar_v89 = v89;
        let v90: f64 = ((self.scalar_v89) as f64).exp();
        self.scalar_v90 = v90;
        let v91: f64 = (self.scalar_v83 - self.scalar_v90);
        self.scalar_v91 = v91;
        let v92: f64 = (if self.scalar_v87 { self.scalar_v91 } else { self.scalar_v82 });
        self.scalar_v92 = v92;
        let v93: f64 = (273.15 + self.scalar_v92);
        self.scalar_v93 = v93;
        let v95: f64 = (self.scalar_v93 * 1.380662e-23);
        self.scalar_v95 = v95;
        let v97: f64 = (self.scalar_v95 / 1.602189e-19);
        self.scalar_v97 = v97;
        let v103: f64 = (self.scalar_v97 * self.scalar_v102);
        self.scalar_v103 = v103;
        let v106: f64 = (self.scalar_v105 / self.scalar_v103);
        self.scalar_v106 = v106;
        let v107: f64 = ((self.scalar_v106) as f64).exp();
        self.scalar_v107 = v107;
        let v124: f64 = (self.scalar_v97 * self.scalar_v116);
        self.scalar_v124 = v124;
        let v167: f64 = (self.scalar_v97 * self.scalar_v160);
        self.scalar_v167 = v167;
        let v203: f64 = (self.scalar_v97 * self.scalar_v196);
        self.scalar_v203 = v203;
        let v236: f64 = (self.scalar_v97 * self.scalar_v229);
        self.scalar_v236 = v236;
        let v257: f64 = (self.scalar_v97 * self.scalar_v250);
        self.scalar_v257 = v257;
        let v277: f64 = (self.scalar_v97 * self.scalar_v270);
        self.scalar_v277 = v277;
        let v297: f64 = (self.scalar_v97 * self.scalar_v290);
        self.scalar_v297 = v297;
        let v339: f64 = (self.scalar_v97 * self.scalar_v332);
        self.scalar_v339 = v339;
        let v359: f64 = (self.scalar_v97 * self.scalar_v352);
        self.scalar_v359 = v359;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
