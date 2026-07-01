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
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: bool,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v116: bool,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: bool,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v408: bool,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v419: bool,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v428: bool,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v439: bool,
    pub(crate) scalar_v442: f64,
    pub(crate) scalar_v443: f64,
    pub(crate) scalar_v446: f64,
    pub(crate) scalar_v447: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v452: f64,
    pub(crate) scalar_v456: bool,
    pub(crate) scalar_v459: f64,
    pub(crate) scalar_v460: f64,
    pub(crate) scalar_v532: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v538: f64,
    pub(crate) scalar_v542: f64,
    pub(crate) scalar_v543: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: f64,
    pub(crate) scalar_v554: f64,
    pub(crate) scalar_v561: f64,
    pub(crate) scalar_v562: f64,
    pub(crate) scalar_v567: f64,
    pub(crate) scalar_v595: f64,
    pub(crate) scalar_v596: f64,
    pub(crate) scalar_v600: f64,
    pub(crate) scalar_v622: f64,
    pub(crate) scalar_v623: f64,
    pub(crate) scalar_v627: f64,
    pub(crate) scalar_v649: f64,
    pub(crate) scalar_v651: f64,
    pub(crate) scalar_v654: f64,
    pub(crate) scalar_v656: f64,
    pub(crate) scalar_v659: f64,
    pub(crate) scalar_v661: f64,
    pub(crate) scalar_v663: f64,
    pub(crate) scalar_v666: f64,
    pub(crate) scalar_v672: f64,
    pub(crate) scalar_v673: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v681: f64,
    pub(crate) scalar_v685: f64,
    pub(crate) scalar_v686: f64,
    pub(crate) scalar_v770: f64,
    pub(crate) scalar_v772: f64,
    pub(crate) scalar_v773: bool,
    pub(crate) scalar_v778: f64,
    pub(crate) scalar_v779: f64,
    pub(crate) scalar_v780: f64,
    pub(crate) scalar_v785: f64,
    pub(crate) scalar_v788: f64,
    pub(crate) scalar_v808: bool,
    pub(crate) scalar_v810: f64,
    pub(crate) scalar_v811: f64,
    pub(crate) scalar_v851: f64,
    pub(crate) scalar_v852: bool,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v858: f64,
    pub(crate) scalar_v864: f64,
    pub(crate) scalar_v867: f64,
    pub(crate) scalar_v874: f64,
    pub(crate) scalar_v875: bool,
    pub(crate) scalar_v876: f64,
    pub(crate) scalar_v907: f64,
    pub(crate) scalar_v908: bool,
    pub(crate) scalar_v909: bool,
    pub(crate) scalar_v910: bool,
    pub(crate) scalar_v911: bool,
    pub(crate) scalar_v919: f64,
    pub(crate) scalar_v920: f64,
    pub(crate) scalar_v925: f64,
    pub(crate) scalar_v926: f64,
    pub(crate) scalar_v976: f64,
    pub(crate) scalar_v995: bool,
    pub(crate) scalar_v996: bool,
    pub(crate) scalar_v1025: f64,
    pub(crate) scalar_v1081: f64,
    pub(crate) scalar_v1082: bool,
    pub(crate) scalar_v1083: f64,
    pub(crate) scalar_v1096: f64,
    pub(crate) scalar_v1100: bool,
    pub(crate) scalar_v1112: f64,
    pub(crate) scalar_v1117: bool,
    pub(crate) scalar_v1146: f64,
    pub(crate) scalar_v1148: f64,
    pub(crate) scalar_v1187: bool,
    pub(crate) scalar_v1191: f64,
    pub(crate) scalar_v1192: bool,
    pub(crate) scalar_v1225: f64,
    pub(crate) scalar_v1226: bool,
    pub(crate) scalar_v1227: bool,
    pub(crate) scalar_v1238: bool,
    pub(crate) scalar_v1239: bool,
    pub(crate) scalar_v1243: bool,
    pub(crate) scalar_v1244: bool,
    pub(crate) scalar_v1267: f64,
    pub(crate) scalar_v1268: bool,
    pub(crate) scalar_v1269: bool,
    pub(crate) scalar_v1270: bool,
    pub(crate) scalar_v1308: bool,
    pub(crate) scalar_v1329: bool,
    pub(crate) scalar_v1330: bool,
    pub(crate) scalar_v1355: bool,
    pub(crate) scalar_v1363: bool,
    pub(crate) scalar_v1368: bool,
    pub(crate) scalar_v1385: f64,
    pub(crate) scalar_v1414: f64,
    pub(crate) scalar_v1438: f64,
    pub(crate) scalar_v1474: bool,
    pub(crate) scalar_v1475: bool,
    pub(crate) scalar_v1476: bool,
    pub(crate) scalar_v1513: bool,
    pub(crate) scalar_v1571: f64,
    pub(crate) scalar_v1572: bool,
    pub(crate) scalar_v1577: f64,
    pub(crate) scalar_v1578: f64,
    pub(crate) scalar_v1591: f64,
    pub(crate) scalar_v1613: bool,
    pub(crate) scalar_v1615: f64,
    pub(crate) scalar_v1616: bool,
    pub(crate) scalar_v1619: f64,
    pub(crate) scalar_v1620: f64,
    pub(crate) scalar_v1621: f64,
    pub(crate) scalar_v1634: f64,
    pub(crate) scalar_v1655: bool,
    pub(crate) scalar_v1657: f64,
    pub(crate) scalar_v1658: bool,
    pub(crate) scalar_v1659: f64,
    pub(crate) scalar_v1660: bool,
    pub(crate) scalar_v1661: bool,
    pub(crate) scalar_v1662: f64,
    pub(crate) scalar_v1663: bool,
    pub(crate) scalar_v1664: bool,
    pub(crate) scalar_v1679: bool,
    pub(crate) scalar_v1680: bool,
    pub(crate) scalar_v1684: f64,
    pub(crate) scalar_v1688: bool,
    pub(crate) scalar_v1692: bool,
    pub(crate) scalar_v1693: bool,
    pub(crate) scalar_v1694: bool,
    pub(crate) scalar_v1733: bool,
    pub(crate) scalar_v1762: f64,
    pub(crate) scalar_v1763: f64,
    pub(crate) scalar_v1790: bool,
    pub(crate) scalar_v1794: f64,
    pub(crate) scalar_v1795: bool,
    pub(crate) scalar_v1796: bool,
    pub(crate) scalar_v1801: f64,
    pub(crate) scalar_v1802: f64,
    pub(crate) scalar_v1807: f64,
    pub(crate) scalar_v1810: f64,
    pub(crate) scalar_v1830: bool,
    pub(crate) scalar_v1831: bool,
    pub(crate) scalar_v1833: f64,
    pub(crate) scalar_v1834: f64,
    pub(crate) scalar_v1872: bool,
    pub(crate) scalar_v2052: f64,
    pub(crate) scalar_v2053: f64,
    pub(crate) scalar_v2057: f64,
    pub(crate) scalar_v2073: f64,
    pub(crate) scalar_v2076: f64,
    pub(crate) scalar_v2084: f64,
    pub(crate) scalar_v2087: f64,
    pub(crate) scalar_v2089: f64,
    pub(crate) scalar_v2091: f64,
    pub(crate) scalar_v2093: f64,
    pub(crate) scalar_v2104: f64,
    pub(crate) scalar_v2105: f64,
    pub(crate) scalar_v2115: f64,
    pub(crate) scalar_v2120: f64,
    pub(crate) scalar_v2126: f64,
    pub(crate) scalar_v2132: f64,
    pub(crate) scalar_v2140: f64,
    pub(crate) scalar_v2146: f64,
    pub(crate) scalar_v2152: f64,
    pub(crate) scalar_v2160: f64,
    pub(crate) scalar_v2165: f64,
    pub(crate) scalar_v2170: f64,
    pub(crate) scalar_v2180: f64,
    pub(crate) scalar_v2197: f64,
    pub(crate) scalar_v2213: f64,
    pub(crate) scalar_v2229: f64,
    pub(crate) scalar_v2245: f64,
    pub(crate) scalar_v2261: f64,
    pub(crate) scalar_v2277: f64,
    pub(crate) scalar_v2301: f64,
    pub(crate) scalar_v2317: f64,
    pub(crate) scalar_v2478: f64,
    pub(crate) scalar_v2496: f64,
    pub(crate) scalar_v2501: f64,
    pub(crate) scalar_v2514: f64,
    pub(crate) scalar_v2624: f64,
    pub(crate) scalar_v2792: f64,
    pub(crate) scalar_v3003: f64,
    pub(crate) scalar_v3257: f64,
    pub(crate) scalar_v3276: f64,
    pub(crate) scalar_v4617: f64,
    pub(crate) scalar_v4644: f64,
    pub(crate) scalar_v4726: f64,
    pub(crate) scalar_v4753: f64,
    pub(crate) scalar_v4855: f64,
    pub(crate) scalar_v5313: f64,
    pub(crate) scalar_v6043: f64,
    pub(crate) scalar_v6044: f64,
    pub(crate) scalar_v6045: f64,
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
    pub(crate) scalar_v93: bool,
    pub(crate) scalar_v94: bool,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v362: f64,
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
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v116: self.scalar_v116,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v198: self.scalar_v198,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v203: self.scalar_v203,
            scalar_v204: self.scalar_v204,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v233: self.scalar_v233,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v312: self.scalar_v312,
            scalar_v323: self.scalar_v323,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v359: self.scalar_v359,
            scalar_v360: self.scalar_v360,
            scalar_v399: self.scalar_v399,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v419: self.scalar_v419,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v433: self.scalar_v433,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v439: self.scalar_v439,
            scalar_v442: self.scalar_v442,
            scalar_v443: self.scalar_v443,
            scalar_v446: self.scalar_v446,
            scalar_v447: self.scalar_v447,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v452: self.scalar_v452,
            scalar_v456: self.scalar_v456,
            scalar_v459: self.scalar_v459,
            scalar_v460: self.scalar_v460,
            scalar_v532: self.scalar_v532,
            scalar_v537: self.scalar_v537,
            scalar_v538: self.scalar_v538,
            scalar_v542: self.scalar_v542,
            scalar_v543: self.scalar_v543,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v554: self.scalar_v554,
            scalar_v561: self.scalar_v561,
            scalar_v562: self.scalar_v562,
            scalar_v567: self.scalar_v567,
            scalar_v595: self.scalar_v595,
            scalar_v596: self.scalar_v596,
            scalar_v600: self.scalar_v600,
            scalar_v622: self.scalar_v622,
            scalar_v623: self.scalar_v623,
            scalar_v627: self.scalar_v627,
            scalar_v649: self.scalar_v649,
            scalar_v651: self.scalar_v651,
            scalar_v654: self.scalar_v654,
            scalar_v656: self.scalar_v656,
            scalar_v659: self.scalar_v659,
            scalar_v661: self.scalar_v661,
            scalar_v663: self.scalar_v663,
            scalar_v666: self.scalar_v666,
            scalar_v672: self.scalar_v672,
            scalar_v673: self.scalar_v673,
            scalar_v680: self.scalar_v680,
            scalar_v681: self.scalar_v681,
            scalar_v685: self.scalar_v685,
            scalar_v686: self.scalar_v686,
            scalar_v770: self.scalar_v770,
            scalar_v772: self.scalar_v772,
            scalar_v773: self.scalar_v773,
            scalar_v778: self.scalar_v778,
            scalar_v779: self.scalar_v779,
            scalar_v780: self.scalar_v780,
            scalar_v785: self.scalar_v785,
            scalar_v788: self.scalar_v788,
            scalar_v808: self.scalar_v808,
            scalar_v810: self.scalar_v810,
            scalar_v811: self.scalar_v811,
            scalar_v851: self.scalar_v851,
            scalar_v852: self.scalar_v852,
            scalar_v857: self.scalar_v857,
            scalar_v858: self.scalar_v858,
            scalar_v864: self.scalar_v864,
            scalar_v867: self.scalar_v867,
            scalar_v874: self.scalar_v874,
            scalar_v875: self.scalar_v875,
            scalar_v876: self.scalar_v876,
            scalar_v907: self.scalar_v907,
            scalar_v908: self.scalar_v908,
            scalar_v909: self.scalar_v909,
            scalar_v910: self.scalar_v910,
            scalar_v911: self.scalar_v911,
            scalar_v919: self.scalar_v919,
            scalar_v920: self.scalar_v920,
            scalar_v925: self.scalar_v925,
            scalar_v926: self.scalar_v926,
            scalar_v976: self.scalar_v976,
            scalar_v995: self.scalar_v995,
            scalar_v996: self.scalar_v996,
            scalar_v1025: self.scalar_v1025,
            scalar_v1081: self.scalar_v1081,
            scalar_v1082: self.scalar_v1082,
            scalar_v1083: self.scalar_v1083,
            scalar_v1096: self.scalar_v1096,
            scalar_v1100: self.scalar_v1100,
            scalar_v1112: self.scalar_v1112,
            scalar_v1117: self.scalar_v1117,
            scalar_v1146: self.scalar_v1146,
            scalar_v1148: self.scalar_v1148,
            scalar_v1187: self.scalar_v1187,
            scalar_v1191: self.scalar_v1191,
            scalar_v1192: self.scalar_v1192,
            scalar_v1225: self.scalar_v1225,
            scalar_v1226: self.scalar_v1226,
            scalar_v1227: self.scalar_v1227,
            scalar_v1238: self.scalar_v1238,
            scalar_v1239: self.scalar_v1239,
            scalar_v1243: self.scalar_v1243,
            scalar_v1244: self.scalar_v1244,
            scalar_v1267: self.scalar_v1267,
            scalar_v1268: self.scalar_v1268,
            scalar_v1269: self.scalar_v1269,
            scalar_v1270: self.scalar_v1270,
            scalar_v1308: self.scalar_v1308,
            scalar_v1329: self.scalar_v1329,
            scalar_v1330: self.scalar_v1330,
            scalar_v1355: self.scalar_v1355,
            scalar_v1363: self.scalar_v1363,
            scalar_v1368: self.scalar_v1368,
            scalar_v1385: self.scalar_v1385,
            scalar_v1414: self.scalar_v1414,
            scalar_v1438: self.scalar_v1438,
            scalar_v1474: self.scalar_v1474,
            scalar_v1475: self.scalar_v1475,
            scalar_v1476: self.scalar_v1476,
            scalar_v1513: self.scalar_v1513,
            scalar_v1571: self.scalar_v1571,
            scalar_v1572: self.scalar_v1572,
            scalar_v1577: self.scalar_v1577,
            scalar_v1578: self.scalar_v1578,
            scalar_v1591: self.scalar_v1591,
            scalar_v1613: self.scalar_v1613,
            scalar_v1615: self.scalar_v1615,
            scalar_v1616: self.scalar_v1616,
            scalar_v1619: self.scalar_v1619,
            scalar_v1620: self.scalar_v1620,
            scalar_v1621: self.scalar_v1621,
            scalar_v1634: self.scalar_v1634,
            scalar_v1655: self.scalar_v1655,
            scalar_v1657: self.scalar_v1657,
            scalar_v1658: self.scalar_v1658,
            scalar_v1659: self.scalar_v1659,
            scalar_v1660: self.scalar_v1660,
            scalar_v1661: self.scalar_v1661,
            scalar_v1662: self.scalar_v1662,
            scalar_v1663: self.scalar_v1663,
            scalar_v1664: self.scalar_v1664,
            scalar_v1679: self.scalar_v1679,
            scalar_v1680: self.scalar_v1680,
            scalar_v1684: self.scalar_v1684,
            scalar_v1688: self.scalar_v1688,
            scalar_v1692: self.scalar_v1692,
            scalar_v1693: self.scalar_v1693,
            scalar_v1694: self.scalar_v1694,
            scalar_v1733: self.scalar_v1733,
            scalar_v1762: self.scalar_v1762,
            scalar_v1763: self.scalar_v1763,
            scalar_v1790: self.scalar_v1790,
            scalar_v1794: self.scalar_v1794,
            scalar_v1795: self.scalar_v1795,
            scalar_v1796: self.scalar_v1796,
            scalar_v1801: self.scalar_v1801,
            scalar_v1802: self.scalar_v1802,
            scalar_v1807: self.scalar_v1807,
            scalar_v1810: self.scalar_v1810,
            scalar_v1830: self.scalar_v1830,
            scalar_v1831: self.scalar_v1831,
            scalar_v1833: self.scalar_v1833,
            scalar_v1834: self.scalar_v1834,
            scalar_v1872: self.scalar_v1872,
            scalar_v2052: self.scalar_v2052,
            scalar_v2053: self.scalar_v2053,
            scalar_v2057: self.scalar_v2057,
            scalar_v2073: self.scalar_v2073,
            scalar_v2076: self.scalar_v2076,
            scalar_v2084: self.scalar_v2084,
            scalar_v2087: self.scalar_v2087,
            scalar_v2089: self.scalar_v2089,
            scalar_v2091: self.scalar_v2091,
            scalar_v2093: self.scalar_v2093,
            scalar_v2104: self.scalar_v2104,
            scalar_v2105: self.scalar_v2105,
            scalar_v2115: self.scalar_v2115,
            scalar_v2120: self.scalar_v2120,
            scalar_v2126: self.scalar_v2126,
            scalar_v2132: self.scalar_v2132,
            scalar_v2140: self.scalar_v2140,
            scalar_v2146: self.scalar_v2146,
            scalar_v2152: self.scalar_v2152,
            scalar_v2160: self.scalar_v2160,
            scalar_v2165: self.scalar_v2165,
            scalar_v2170: self.scalar_v2170,
            scalar_v2180: self.scalar_v2180,
            scalar_v2197: self.scalar_v2197,
            scalar_v2213: self.scalar_v2213,
            scalar_v2229: self.scalar_v2229,
            scalar_v2245: self.scalar_v2245,
            scalar_v2261: self.scalar_v2261,
            scalar_v2277: self.scalar_v2277,
            scalar_v2301: self.scalar_v2301,
            scalar_v2317: self.scalar_v2317,
            scalar_v2478: self.scalar_v2478,
            scalar_v2496: self.scalar_v2496,
            scalar_v2501: self.scalar_v2501,
            scalar_v2514: self.scalar_v2514,
            scalar_v2624: self.scalar_v2624,
            scalar_v2792: self.scalar_v2792,
            scalar_v3003: self.scalar_v3003,
            scalar_v3257: self.scalar_v3257,
            scalar_v3276: self.scalar_v3276,
            scalar_v4617: self.scalar_v4617,
            scalar_v4644: self.scalar_v4644,
            scalar_v4726: self.scalar_v4726,
            scalar_v4753: self.scalar_v4753,
            scalar_v4855: self.scalar_v4855,
            scalar_v5313: self.scalar_v5313,
            scalar_v6043: self.scalar_v6043,
            scalar_v6044: self.scalar_v6044,
            scalar_v6045: self.scalar_v6045,
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
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v98: self.scalar_v98,
            scalar_v100: self.scalar_v100,
            scalar_v106: self.scalar_v106,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v127: self.scalar_v127,
            scalar_v170: self.scalar_v170,
            scalar_v206: self.scalar_v206,
            scalar_v239: self.scalar_v239,
            scalar_v260: self.scalar_v260,
            scalar_v280: self.scalar_v280,
            scalar_v300: self.scalar_v300,
            scalar_v342: self.scalar_v342,
            scalar_v362: self.scalar_v362,
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
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: false,
            scalar_v105: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v116: false,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v132: 0.0,
            scalar_v133: false,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v203: 0.0,
            scalar_v204: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v233: 0.0,
            scalar_v236: 0.0,
            scalar_v237: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v257: 0.0,
            scalar_v258: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v277: 0.0,
            scalar_v278: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v312: 0.0,
            scalar_v323: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v354: 0.0,
            scalar_v355: 0.0,
            scalar_v356: 0.0,
            scalar_v359: 0.0,
            scalar_v360: 0.0,
            scalar_v399: 0.0,
            scalar_v402: 0.0,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v408: false,
            scalar_v409: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v419: false,
            scalar_v422: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v428: false,
            scalar_v429: 0.0,
            scalar_v433: 0.0,
            scalar_v434: 0.0,
            scalar_v435: 0.0,
            scalar_v439: false,
            scalar_v442: 0.0,
            scalar_v443: 0.0,
            scalar_v446: 0.0,
            scalar_v447: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v452: 0.0,
            scalar_v456: false,
            scalar_v459: 0.0,
            scalar_v460: 0.0,
            scalar_v532: 0.0,
            scalar_v537: 0.0,
            scalar_v538: 0.0,
            scalar_v542: 0.0,
            scalar_v543: 0.0,
            scalar_v547: 0.0,
            scalar_v548: 0.0,
            scalar_v554: 0.0,
            scalar_v561: 0.0,
            scalar_v562: 0.0,
            scalar_v567: 0.0,
            scalar_v595: 0.0,
            scalar_v596: 0.0,
            scalar_v600: 0.0,
            scalar_v622: 0.0,
            scalar_v623: 0.0,
            scalar_v627: 0.0,
            scalar_v649: 0.0,
            scalar_v651: 0.0,
            scalar_v654: 0.0,
            scalar_v656: 0.0,
            scalar_v659: 0.0,
            scalar_v661: 0.0,
            scalar_v663: 0.0,
            scalar_v666: 0.0,
            scalar_v672: 0.0,
            scalar_v673: 0.0,
            scalar_v680: 0.0,
            scalar_v681: 0.0,
            scalar_v685: 0.0,
            scalar_v686: 0.0,
            scalar_v770: 0.0,
            scalar_v772: 0.0,
            scalar_v773: false,
            scalar_v778: 0.0,
            scalar_v779: 0.0,
            scalar_v780: 0.0,
            scalar_v785: 0.0,
            scalar_v788: 0.0,
            scalar_v808: false,
            scalar_v810: 0.0,
            scalar_v811: 0.0,
            scalar_v851: 0.0,
            scalar_v852: false,
            scalar_v857: 0.0,
            scalar_v858: 0.0,
            scalar_v864: 0.0,
            scalar_v867: 0.0,
            scalar_v874: 0.0,
            scalar_v875: false,
            scalar_v876: 0.0,
            scalar_v907: 0.0,
            scalar_v908: false,
            scalar_v909: false,
            scalar_v910: false,
            scalar_v911: false,
            scalar_v919: 0.0,
            scalar_v920: 0.0,
            scalar_v925: 0.0,
            scalar_v926: 0.0,
            scalar_v976: 0.0,
            scalar_v995: false,
            scalar_v996: false,
            scalar_v1025: 0.0,
            scalar_v1081: 0.0,
            scalar_v1082: false,
            scalar_v1083: 0.0,
            scalar_v1096: 0.0,
            scalar_v1100: false,
            scalar_v1112: 0.0,
            scalar_v1117: false,
            scalar_v1146: 0.0,
            scalar_v1148: 0.0,
            scalar_v1187: false,
            scalar_v1191: 0.0,
            scalar_v1192: false,
            scalar_v1225: 0.0,
            scalar_v1226: false,
            scalar_v1227: false,
            scalar_v1238: false,
            scalar_v1239: false,
            scalar_v1243: false,
            scalar_v1244: false,
            scalar_v1267: 0.0,
            scalar_v1268: false,
            scalar_v1269: false,
            scalar_v1270: false,
            scalar_v1308: false,
            scalar_v1329: false,
            scalar_v1330: false,
            scalar_v1355: false,
            scalar_v1363: false,
            scalar_v1368: false,
            scalar_v1385: 0.0,
            scalar_v1414: 0.0,
            scalar_v1438: 0.0,
            scalar_v1474: false,
            scalar_v1475: false,
            scalar_v1476: false,
            scalar_v1513: false,
            scalar_v1571: 0.0,
            scalar_v1572: false,
            scalar_v1577: 0.0,
            scalar_v1578: 0.0,
            scalar_v1591: 0.0,
            scalar_v1613: false,
            scalar_v1615: 0.0,
            scalar_v1616: false,
            scalar_v1619: 0.0,
            scalar_v1620: 0.0,
            scalar_v1621: 0.0,
            scalar_v1634: 0.0,
            scalar_v1655: false,
            scalar_v1657: 0.0,
            scalar_v1658: false,
            scalar_v1659: 0.0,
            scalar_v1660: false,
            scalar_v1661: false,
            scalar_v1662: 0.0,
            scalar_v1663: false,
            scalar_v1664: false,
            scalar_v1679: false,
            scalar_v1680: false,
            scalar_v1684: 0.0,
            scalar_v1688: false,
            scalar_v1692: false,
            scalar_v1693: false,
            scalar_v1694: false,
            scalar_v1733: false,
            scalar_v1762: 0.0,
            scalar_v1763: 0.0,
            scalar_v1790: false,
            scalar_v1794: 0.0,
            scalar_v1795: false,
            scalar_v1796: false,
            scalar_v1801: 0.0,
            scalar_v1802: 0.0,
            scalar_v1807: 0.0,
            scalar_v1810: 0.0,
            scalar_v1830: false,
            scalar_v1831: false,
            scalar_v1833: 0.0,
            scalar_v1834: 0.0,
            scalar_v1872: false,
            scalar_v2052: 0.0,
            scalar_v2053: 0.0,
            scalar_v2057: 0.0,
            scalar_v2073: 0.0,
            scalar_v2076: 0.0,
            scalar_v2084: 0.0,
            scalar_v2087: 0.0,
            scalar_v2089: 0.0,
            scalar_v2091: 0.0,
            scalar_v2093: 0.0,
            scalar_v2104: 0.0,
            scalar_v2105: 0.0,
            scalar_v2115: 0.0,
            scalar_v2120: 0.0,
            scalar_v2126: 0.0,
            scalar_v2132: 0.0,
            scalar_v2140: 0.0,
            scalar_v2146: 0.0,
            scalar_v2152: 0.0,
            scalar_v2160: 0.0,
            scalar_v2165: 0.0,
            scalar_v2170: 0.0,
            scalar_v2180: 0.0,
            scalar_v2197: 0.0,
            scalar_v2213: 0.0,
            scalar_v2229: 0.0,
            scalar_v2245: 0.0,
            scalar_v2261: 0.0,
            scalar_v2277: 0.0,
            scalar_v2301: 0.0,
            scalar_v2317: 0.0,
            scalar_v2478: 0.0,
            scalar_v2496: 0.0,
            scalar_v2501: 0.0,
            scalar_v2514: 0.0,
            scalar_v2624: 0.0,
            scalar_v2792: 0.0,
            scalar_v3003: 0.0,
            scalar_v3257: 0.0,
            scalar_v3276: 0.0,
            scalar_v4617: 0.0,
            scalar_v4644: 0.0,
            scalar_v4726: 0.0,
            scalar_v4753: 0.0,
            scalar_v4855: 0.0,
            scalar_v5313: 0.0,
            scalar_v6043: 0.0,
            scalar_v6044: 0.0,
            scalar_v6045: 0.0,
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
            scalar_v93: false,
            scalar_v94: false,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v98: 0.0,
            scalar_v100: 0.0,
            scalar_v106: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v127: 0.0,
            scalar_v170: 0.0,
            scalar_v206: 0.0,
            scalar_v239: 0.0,
            scalar_v260: 0.0,
            scalar_v280: 0.0,
            scalar_v300: 0.0,
            scalar_v342: 0.0,
            scalar_v362: 0.0,
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
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v107,
            scalar_v108,
            scalar_v116,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v123,
            scalar_v124,
            scalar_v132,
            scalar_v133,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v144,
            scalar_v145,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v167,
            scalar_v168,
            scalar_v179,
            scalar_v180,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v203,
            scalar_v204,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v236,
            scalar_v237,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v257,
            scalar_v258,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v297,
            scalar_v298,
            scalar_v312,
            scalar_v323,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v339,
            scalar_v340,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v359,
            scalar_v360,
            scalar_v399,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v419,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v428,
            scalar_v429,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v439,
            scalar_v442,
            scalar_v443,
            scalar_v446,
            scalar_v447,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v456,
            scalar_v459,
            scalar_v460,
            scalar_v532,
            scalar_v537,
            scalar_v538,
            scalar_v542,
            scalar_v543,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v561,
            scalar_v562,
            scalar_v567,
            scalar_v595,
            scalar_v596,
            scalar_v600,
            scalar_v622,
            scalar_v623,
            scalar_v627,
            scalar_v649,
            scalar_v651,
            scalar_v654,
            scalar_v656,
            scalar_v659,
            scalar_v661,
            scalar_v663,
            scalar_v666,
            scalar_v672,
            scalar_v673,
            scalar_v680,
            scalar_v681,
            scalar_v685,
            scalar_v686,
            scalar_v770,
            scalar_v772,
            scalar_v773,
            scalar_v778,
            scalar_v779,
            scalar_v780,
            scalar_v785,
            scalar_v788,
            scalar_v808,
            scalar_v810,
            scalar_v811,
            scalar_v851,
            scalar_v852,
            scalar_v857,
            scalar_v858,
            scalar_v864,
            scalar_v867,
            scalar_v874,
            scalar_v875,
            scalar_v876,
            scalar_v907,
            scalar_v908,
            scalar_v909,
            scalar_v910,
            scalar_v911,
            scalar_v919,
            scalar_v920,
            scalar_v925,
            scalar_v926,
            scalar_v976,
            scalar_v995,
            scalar_v996,
            scalar_v1025,
            scalar_v1081,
            scalar_v1082,
            scalar_v1083,
            scalar_v1096,
            scalar_v1100,
            scalar_v1112,
            scalar_v1117,
            scalar_v1146,
            scalar_v1148,
            scalar_v1187,
            scalar_v1191,
            scalar_v1192,
            scalar_v1225,
            scalar_v1226,
            scalar_v1227,
            scalar_v1238,
            scalar_v1239,
            scalar_v1243,
            scalar_v1244,
            scalar_v1267,
            scalar_v1268,
            scalar_v1269,
            scalar_v1270,
            scalar_v1308,
            scalar_v1329,
            scalar_v1330,
            scalar_v1355,
            scalar_v1363,
            scalar_v1368,
            scalar_v1385,
            scalar_v1414,
            scalar_v1438,
            scalar_v1474,
            scalar_v1475,
            scalar_v1476,
            scalar_v1513,
            scalar_v1571,
            scalar_v1572,
            scalar_v1577,
            scalar_v1578,
            scalar_v1591,
            scalar_v1613,
            scalar_v1615,
            scalar_v1616,
            scalar_v1619,
            scalar_v1620,
            scalar_v1621,
            scalar_v1634,
            scalar_v1655,
            scalar_v1657,
            scalar_v1658,
            scalar_v1659,
            scalar_v1660,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1664,
            scalar_v1679,
            scalar_v1680,
            scalar_v1684,
            scalar_v1688,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1733,
            scalar_v1762,
            scalar_v1763,
            scalar_v1790,
            scalar_v1794,
            scalar_v1795,
            scalar_v1796,
            scalar_v1801,
            scalar_v1802,
            scalar_v1807,
            scalar_v1810,
            scalar_v1830,
            scalar_v1831,
            scalar_v1833,
            scalar_v1834,
            scalar_v1872,
            scalar_v2052,
            scalar_v2053,
            scalar_v2057,
            scalar_v2073,
            scalar_v2076,
            scalar_v2084,
            scalar_v2087,
            scalar_v2089,
            scalar_v2091,
            scalar_v2093,
            scalar_v2104,
            scalar_v2105,
            scalar_v2115,
            scalar_v2120,
            scalar_v2126,
            scalar_v2132,
            scalar_v2140,
            scalar_v2146,
            scalar_v2152,
            scalar_v2160,
            scalar_v2165,
            scalar_v2170,
            scalar_v2180,
            scalar_v2197,
            scalar_v2213,
            scalar_v2229,
            scalar_v2245,
            scalar_v2261,
            scalar_v2277,
            scalar_v2301,
            scalar_v2317,
            scalar_v2478,
            scalar_v2496,
            scalar_v2501,
            scalar_v2514,
            scalar_v2624,
            scalar_v2792,
            scalar_v3003,
            scalar_v3257,
            scalar_v3276,
            scalar_v4617,
            scalar_v4644,
            scalar_v4726,
            scalar_v4753,
            scalar_v4855,
            scalar_v5313,
            scalar_v6043,
            scalar_v6044,
            scalar_v6045,
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
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v98,
            scalar_v100,
            scalar_v106,
            scalar_v109,
            scalar_v110,
            scalar_v127,
            scalar_v170,
            scalar_v206,
            scalar_v239,
            scalar_v260,
            scalar_v280,
            scalar_v300,
            scalar_v342,
            scalar_v362,
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
            scalar_v102,
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v107,
            scalar_v108,
            scalar_v116,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v123,
            scalar_v124,
            scalar_v132,
            scalar_v133,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v144,
            scalar_v145,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v167,
            scalar_v168,
            scalar_v179,
            scalar_v180,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v203,
            scalar_v204,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v236,
            scalar_v237,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v257,
            scalar_v258,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v277,
            scalar_v278,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v297,
            scalar_v298,
            scalar_v312,
            scalar_v323,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v339,
            scalar_v340,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v359,
            scalar_v360,
            scalar_v399,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v408,
            scalar_v409,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v419,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v428,
            scalar_v429,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v439,
            scalar_v442,
            scalar_v443,
            scalar_v446,
            scalar_v447,
            scalar_v450,
            scalar_v451,
            scalar_v452,
            scalar_v456,
            scalar_v459,
            scalar_v460,
            scalar_v532,
            scalar_v537,
            scalar_v538,
            scalar_v542,
            scalar_v543,
            scalar_v547,
            scalar_v548,
            scalar_v554,
            scalar_v561,
            scalar_v562,
            scalar_v567,
            scalar_v595,
            scalar_v596,
            scalar_v600,
            scalar_v622,
            scalar_v623,
            scalar_v627,
            scalar_v649,
            scalar_v651,
            scalar_v654,
            scalar_v656,
            scalar_v659,
            scalar_v661,
            scalar_v663,
            scalar_v666,
            scalar_v672,
            scalar_v673,
            scalar_v680,
            scalar_v681,
            scalar_v685,
            scalar_v686,
            scalar_v770,
            scalar_v772,
            scalar_v773,
            scalar_v778,
            scalar_v779,
            scalar_v780,
            scalar_v785,
            scalar_v788,
            scalar_v808,
            scalar_v810,
            scalar_v811,
            scalar_v851,
            scalar_v852,
            scalar_v857,
            scalar_v858,
            scalar_v864,
            scalar_v867,
            scalar_v874,
            scalar_v875,
            scalar_v876,
            scalar_v907,
            scalar_v908,
            scalar_v909,
            scalar_v910,
            scalar_v911,
            scalar_v919,
            scalar_v920,
            scalar_v925,
            scalar_v926,
            scalar_v976,
            scalar_v995,
            scalar_v996,
            scalar_v1025,
            scalar_v1081,
            scalar_v1082,
            scalar_v1083,
            scalar_v1096,
            scalar_v1100,
            scalar_v1112,
            scalar_v1117,
            scalar_v1146,
            scalar_v1148,
            scalar_v1187,
            scalar_v1191,
            scalar_v1192,
            scalar_v1225,
            scalar_v1226,
            scalar_v1227,
            scalar_v1238,
            scalar_v1239,
            scalar_v1243,
            scalar_v1244,
            scalar_v1267,
            scalar_v1268,
            scalar_v1269,
            scalar_v1270,
            scalar_v1308,
            scalar_v1329,
            scalar_v1330,
            scalar_v1355,
            scalar_v1363,
            scalar_v1368,
            scalar_v1385,
            scalar_v1414,
            scalar_v1438,
            scalar_v1474,
            scalar_v1475,
            scalar_v1476,
            scalar_v1513,
            scalar_v1571,
            scalar_v1572,
            scalar_v1577,
            scalar_v1578,
            scalar_v1591,
            scalar_v1613,
            scalar_v1615,
            scalar_v1616,
            scalar_v1619,
            scalar_v1620,
            scalar_v1621,
            scalar_v1634,
            scalar_v1655,
            scalar_v1657,
            scalar_v1658,
            scalar_v1659,
            scalar_v1660,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1664,
            scalar_v1679,
            scalar_v1680,
            scalar_v1684,
            scalar_v1688,
            scalar_v1692,
            scalar_v1693,
            scalar_v1694,
            scalar_v1733,
            scalar_v1762,
            scalar_v1763,
            scalar_v1790,
            scalar_v1794,
            scalar_v1795,
            scalar_v1796,
            scalar_v1801,
            scalar_v1802,
            scalar_v1807,
            scalar_v1810,
            scalar_v1830,
            scalar_v1831,
            scalar_v1833,
            scalar_v1834,
            scalar_v1872,
            scalar_v2052,
            scalar_v2053,
            scalar_v2057,
            scalar_v2073,
            scalar_v2076,
            scalar_v2084,
            scalar_v2087,
            scalar_v2089,
            scalar_v2091,
            scalar_v2093,
            scalar_v2104,
            scalar_v2105,
            scalar_v2115,
            scalar_v2120,
            scalar_v2126,
            scalar_v2132,
            scalar_v2140,
            scalar_v2146,
            scalar_v2152,
            scalar_v2160,
            scalar_v2165,
            scalar_v2170,
            scalar_v2180,
            scalar_v2197,
            scalar_v2213,
            scalar_v2229,
            scalar_v2245,
            scalar_v2261,
            scalar_v2277,
            scalar_v2301,
            scalar_v2317,
            scalar_v2478,
            scalar_v2496,
            scalar_v2501,
            scalar_v2514,
            scalar_v2624,
            scalar_v2792,
            scalar_v3003,
            scalar_v3257,
            scalar_v3276,
            scalar_v4617,
            scalar_v4644,
            scalar_v4726,
            scalar_v4753,
            scalar_v4855,
            scalar_v5313,
            scalar_v6043,
            scalar_v6044,
            scalar_v6045,
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
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v98,
            scalar_v100,
            scalar_v106,
            scalar_v109,
            scalar_v110,
            scalar_v127,
            scalar_v170,
            scalar_v206,
            scalar_v239,
            scalar_v260,
            scalar_v280,
            scalar_v300,
            scalar_v342,
            scalar_v362,
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
        let v102: f64 = p.p26;
        self.scalar_v102 = v102;
        let v103: f64 = p.p90;
        self.scalar_v103 = v103;
        let v104: bool = (p.p90 > 0.0);
        self.scalar_v104 = v104;
        let v105: f64 = p.p89;
        self.scalar_v105 = v105;
        let v107: f64 = p.p88;
        self.scalar_v107 = v107;
        let v108: f64 = (-p.p88);
        self.scalar_v108 = v108;
        let v116: bool = (!v104);
        self.scalar_v116 = v116;
        let v118: f64 = p.p122;
        self.scalar_v118 = v118;
        let v119: f64 = p.p28;
        self.scalar_v119 = v119;
        let v120: f64 = (p.p122 / p.p28);
        self.scalar_v120 = v120;
        let v123: f64 = p.p113;
        self.scalar_v123 = v123;
        let v124: f64 = (-p.p113);
        self.scalar_v124 = v124;
        let v132: f64 = p.p72;
        self.scalar_v132 = v132;
        let v133: bool = (p.p72 > 0.0);
        self.scalar_v133 = v133;
        let v140: f64 = (4.0 / p.p72);
        self.scalar_v140 = v140;
        let v141: f64 = p.p73;
        self.scalar_v141 = v141;
        let v142: f64 = f64::powf(v140, p.p73);
        self.scalar_v142 = v142;
        let v144: f64 = (1.0 - p.p73);
        self.scalar_v144 = v144;
        let v145: f64 = (1.0 / v144);
        self.scalar_v145 = v145;
        let v161: f64 = p.p27;
        self.scalar_v161 = v161;
        let v162: f64 = p.p125;
        self.scalar_v162 = v162;
        let v163: f64 = p.p29;
        self.scalar_v163 = v163;
        let v164: f64 = (p.p125 / p.p29);
        self.scalar_v164 = v164;
        let v167: f64 = p.p121;
        self.scalar_v167 = v167;
        let v168: f64 = (-p.p121);
        self.scalar_v168 = v168;
        let v179: f64 = (4.0 / p.p74);
        self.scalar_v179 = v179;
        let v180: f64 = f64::powf(v179, p.p73);
        self.scalar_v180 = v180;
        let v198: f64 = p.p31;
        self.scalar_v198 = v198;
        let v199: f64 = p.p33;
        self.scalar_v199 = v199;
        let v200: f64 = (p.p122 / p.p33);
        self.scalar_v200 = v200;
        let v203: f64 = p.p120;
        self.scalar_v203 = v203;
        let v204: f64 = (-p.p120);
        self.scalar_v204 = v204;
        let v230: f64 = p.p54;
        self.scalar_v230 = v230;
        let v231: f64 = p.p123;
        self.scalar_v231 = v231;
        let v232: f64 = p.p56;
        self.scalar_v232 = v232;
        let v233: f64 = (p.p123 / p.p56);
        self.scalar_v233 = v233;
        let v236: f64 = p.p114;
        self.scalar_v236 = v236;
        let v237: f64 = (-p.p114);
        self.scalar_v237 = v237;
        let v251: f64 = p.p58;
        self.scalar_v251 = v251;
        let v252: f64 = p.p124;
        self.scalar_v252 = v252;
        let v253: f64 = p.p59;
        self.scalar_v253 = v253;
        let v254: f64 = (p.p124 / p.p59);
        self.scalar_v254 = v254;
        let v257: f64 = p.p117;
        self.scalar_v257 = v257;
        let v258: f64 = (-p.p117);
        self.scalar_v258 = v258;
        let v272: f64 = p.p60;
        self.scalar_v272 = v272;
        let v273: f64 = p.p61;
        self.scalar_v273 = v273;
        let v274: f64 = (p.p123 / p.p61);
        self.scalar_v274 = v274;
        let v277: f64 = p.p115;
        self.scalar_v277 = v277;
        let v278: f64 = (-p.p115);
        self.scalar_v278 = v278;
        let v292: f64 = p.p62;
        self.scalar_v292 = v292;
        let v293: f64 = p.p63;
        self.scalar_v293 = v293;
        let v294: f64 = (p.p124 / p.p63);
        self.scalar_v294 = v294;
        let v297: f64 = p.p118;
        self.scalar_v297 = v297;
        let v298: f64 = (-p.p118);
        self.scalar_v298 = v298;
        let v312: f64 = p.p64;
        self.scalar_v312 = v312;
        let v323: f64 = p.p65;
        self.scalar_v323 = v323;
        let v334: f64 = p.p66;
        self.scalar_v334 = v334;
        let v335: f64 = p.p67;
        self.scalar_v335 = v335;
        let v336: f64 = (p.p123 / p.p67);
        self.scalar_v336 = v336;
        let v339: f64 = p.p116;
        self.scalar_v339 = v339;
        let v340: f64 = (-p.p116);
        self.scalar_v340 = v340;
        let v354: f64 = p.p68;
        self.scalar_v354 = v354;
        let v355: f64 = p.p69;
        self.scalar_v355 = v355;
        let v356: f64 = (p.p124 / p.p69);
        self.scalar_v356 = v356;
        let v359: f64 = p.p119;
        self.scalar_v359 = v359;
        let v360: f64 = (-p.p119);
        self.scalar_v360 = v360;
        let v399: f64 = p.p126;
        self.scalar_v399 = v399;
        let v402: f64 = if param_given[109] { 1.0 } else { 0.0 };
        self.scalar_v402 = v402;
        let v403: f64 = p.p16;
        self.scalar_v403 = v403;
        let v404: f64 = p.p109;
        self.scalar_v404 = v404;
        let v408: bool = (!(if param_given[109] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v408 = v408;
        let v409: f64 = p.p107;
        self.scalar_v409 = v409;
        let v413: f64 = if param_given[108] { 1.0 } else { 0.0 };
        self.scalar_v413 = v413;
        let v414: f64 = p.p17;
        self.scalar_v414 = v414;
        let v415: f64 = p.p108;
        self.scalar_v415 = v415;
        let v419: bool = (!(if param_given[108] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v419 = v419;
        let v422: f64 = if param_given[106] { 1.0 } else { 0.0 };
        self.scalar_v422 = v422;
        let v423: f64 = p.p21;
        self.scalar_v423 = v423;
        let v424: f64 = p.p106;
        self.scalar_v424 = v424;
        let v428: bool = (!(if param_given[106] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v428 = v428;
        let v429: f64 = p.p104;
        self.scalar_v429 = v429;
        let v433: f64 = if param_given[105] { 1.0 } else { 0.0 };
        self.scalar_v433 = v433;
        let v434: f64 = p.p22;
        self.scalar_v434 = v434;
        let v435: f64 = p.p105;
        self.scalar_v435 = v435;
        let v439: bool = (!(if param_given[105] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v439 = v439;
        let v442: f64 = p.p23;
        self.scalar_v442 = v442;
        let v443: f64 = p.p103;
        self.scalar_v443 = v443;
        let v446: f64 = p.p24;
        self.scalar_v446 = v446;
        let v447: f64 = p.p111;
        self.scalar_v447 = v447;
        let v450: f64 = if param_given[110] { 1.0 } else { 0.0 };
        self.scalar_v450 = v450;
        let v451: f64 = p.p25;
        self.scalar_v451 = v451;
        let v452: f64 = p.p110;
        self.scalar_v452 = v452;
        let v456: bool = (!(if param_given[110] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v456 = v456;
        let v459: f64 = p.p101;
        self.scalar_v459 = v459;
        let v460: f64 = p.p132;
        self.scalar_v460 = v460;
        let v532: f64 = p.p129;
        self.scalar_v532 = v532;
        let v537: f64 = p.p84;
        self.scalar_v537 = v537;
        let v538: f64 = p.p127;
        self.scalar_v538 = v538;
        let v542: f64 = p.p86;
        self.scalar_v542 = v542;
        let v543: f64 = p.p128;
        self.scalar_v543 = v543;
        let v547: f64 = p.p91;
        self.scalar_v547 = v547;
        let v548: f64 = p.p92;
        self.scalar_v548 = v548;
        let v554: f64 = p.p93;
        self.scalar_v554 = v554;
        let v561: f64 = p.p37;
        self.scalar_v561 = v561;
        let v562: f64 = (0.5 * p.p37);
        self.scalar_v562 = v562;
        let v567: f64 = (p.p37 * -0.5);
        self.scalar_v567 = v567;
        let v595: f64 = p.p42;
        self.scalar_v595 = v595;
        let v596: f64 = (0.5 * p.p42);
        self.scalar_v596 = v596;
        let v600: f64 = (-0.5 * p.p42);
        self.scalar_v600 = v600;
        let v622: f64 = p.p50;
        self.scalar_v622 = v622;
        let v623: f64 = (0.5 * p.p50);
        self.scalar_v623 = v623;
        let v627: f64 = (-0.5 * p.p50);
        self.scalar_v627 = v627;
        let v649: f64 = p.p36;
        self.scalar_v649 = v649;
        let v651: f64 = p.p38;
        self.scalar_v651 = v651;
        let v654: f64 = p.p41;
        self.scalar_v654 = v654;
        let v656: f64 = p.p43;
        self.scalar_v656 = v656;
        let v659: f64 = p.p48;
        self.scalar_v659 = v659;
        let v661: f64 = p.p49;
        self.scalar_v661 = v661;
        let v663: f64 = p.p51;
        self.scalar_v663 = v663;
        let v666: f64 = p.p19;
        self.scalar_v666 = v666;
        let v672: f64 = p.p18;
        self.scalar_v672 = v672;
        let v673: f64 = p.p112;
        self.scalar_v673 = v673;
        let v680: f64 = p.p70;
        self.scalar_v680 = v680;
        let v681: f64 = p.p130;
        self.scalar_v681 = v681;
        let v685: f64 = p.p71;
        self.scalar_v685 = v685;
        let v686: f64 = p.p131;
        self.scalar_v686 = v686;
        let v770: f64 = p.p34;
        self.scalar_v770 = v770;
        let v772: f64 = p.p39;
        self.scalar_v772 = v772;
        let v773: bool = (p.p39 <= 0.0);
        self.scalar_v773 = v773;
        let v778: f64 = (1.0 - p.p34);
        self.scalar_v778 = v778;
        let v779: f64 = (-p.p38);
        self.scalar_v779 = v779;
        let v780: f64 = f64::powf(v778, v779);
        self.scalar_v780 = v780;
        let v785: f64 = (1.0 - p.p38);
        self.scalar_v785 = v785;
        let v788: f64 = (0.5 * p.p38);
        self.scalar_v788 = v788;
        let v808: bool = (!v773);
        self.scalar_v808 = v808;
        let v810: f64 = (4.0 * p.p39);
        self.scalar_v810 = v810;
        let v811: f64 = (p.p39 * v810);
        self.scalar_v811 = v811;
        let v851: f64 = p.p44;
        self.scalar_v851 = v851;
        let v852: bool = (p.p44 <= 0.0);
        self.scalar_v852 = v852;
        let v857: f64 = (-1.0 - p.p43);
        self.scalar_v857 = v857;
        let v858: f64 = f64::powf(v778, v857);
        self.scalar_v858 = v858;
        let v864: f64 = (1.0 - p.p43);
        self.scalar_v864 = v864;
        let v867: f64 = (0.5 * p.p43);
        self.scalar_v867 = v867;
        let v874: f64 = p.p45;
        self.scalar_v874 = v874;
        let v875: bool = (p.p45 > 0.0);
        self.scalar_v875 = v875;
        let v876: f64 = (-p.p45);
        self.scalar_v876 = v876;
        let v907: f64 = p.p46;
        self.scalar_v907 = v907;
        let v908: bool = (p.p46 > 0.0);
        self.scalar_v908 = v908;
        let v909: bool = (v875 && v908);
        self.scalar_v909 = v909;
        let v910: bool = (!v852);
        self.scalar_v910 = v910;
        let v911: bool = (v909 && v910);
        self.scalar_v911 = v911;
        let v919: f64 = (4.0 * p.p44);
        self.scalar_v919 = v919;
        let v920: f64 = (p.p44 * v919);
        self.scalar_v920 = v920;
        let v925: f64 = (4.0 * p.p46);
        self.scalar_v925 = v925;
        let v926: f64 = (p.p46 * v925);
        self.scalar_v926 = v926;
        let v976: f64 = (-p.p43);
        self.scalar_v976 = v976;
        let v995: bool = (!v909);
        self.scalar_v995 = v995;
        let v996: bool = (v910 && v995);
        self.scalar_v996 = v996;
        let v1025: f64 = f64::powf(v778, v976);
        self.scalar_v1025 = v1025;
        let v1081: f64 = p.p30;
        self.scalar_v1081 = v1081;
        let v1082: bool = (p.p30 < 0.5);
        self.scalar_v1082 = v1082;
        let v1083: f64 = (1.0 / p.p73);
        self.scalar_v1083 = v1083;
        let v1096: f64 = f64::powf(1e-8, p.p73);
        self.scalar_v1096 = v1096;
        let v1100: bool = (!v1082);
        self.scalar_v1100 = v1100;
        let v1112: f64 = (1.0 + v1096);
        self.scalar_v1112 = v1112;
        let v1117: bool = (p.p31 > 0.0);
        self.scalar_v1117 = v1117;
        let v1146: f64 = p.p32;
        self.scalar_v1146 = v1146;
        let v1148: f64 = (1.0 - p.p32);
        self.scalar_v1148 = v1148;
        let v1187: bool = (!v1117);
        self.scalar_v1187 = v1187;
        let v1191: f64 = p.p55;
        self.scalar_v1191 = v1191;
        let v1192: bool = (1.0 == p.p55);
        self.scalar_v1192 = v1192;
        let v1225: f64 = p.p57;
        self.scalar_v1225 = v1225;
        let v1226: bool = (p.p57 > 0.0);
        self.scalar_v1226 = v1226;
        let v1227: bool = (v1192 && v1226);
        self.scalar_v1227 = v1227;
        let v1238: bool = (!v1226);
        self.scalar_v1238 = v1238;
        let v1239: bool = (v1192 && v1238);
        self.scalar_v1239 = v1239;
        let v1243: bool = (p.p88 > 0.0);
        self.scalar_v1243 = v1243;
        let v1244: bool = (v1192 && v1243);
        self.scalar_v1244 = v1244;
        let v1267: f64 = (if v1192 { 0.0 } else { 0.0 });
        self.scalar_v1267 = v1267;
        let v1268: bool = (0.0 == p.p55);
        self.scalar_v1268 = v1268;
        let v1269: bool = (!v1192);
        self.scalar_v1269 = v1269;
        let v1270: bool = (v1268 && v1269);
        self.scalar_v1270 = v1270;
        let v1308: bool = (v1243 && v1270);
        self.scalar_v1308 = v1308;
        let v1329: bool = (!v1268);
        self.scalar_v1329 = v1329;
        let v1330: bool = (v1269 && v1329);
        self.scalar_v1330 = v1330;
        let v1355: bool = (v1226 && v1330);
        self.scalar_v1355 = v1355;
        let v1363: bool = (v1238 && v1330);
        self.scalar_v1363 = v1363;
        let v1368: bool = (v1243 && v1330);
        self.scalar_v1368 = v1368;
        let v1385: f64 = (p.p90 * p.p55);
        self.scalar_v1385 = v1385;
        let v1414: f64 = (1.0 - p.p55);
        self.scalar_v1414 = v1414;
        let v1438: f64 = (p.p90 * v1414);
        self.scalar_v1438 = v1438;
        let v1474: bool = (p.p64 > 0.0);
        self.scalar_v1474 = v1474;
        let v1475: bool = (p.p65 > 0.0);
        self.scalar_v1475 = v1475;
        let v1476: bool = (v1474 || v1475);
        self.scalar_v1476 = v1476;
        let v1513: bool = (!v1476);
        self.scalar_v1513 = v1513;
        let v1571: f64 = p.p83;
        self.scalar_v1571 = v1571;
        let v1572: bool = (p.p83 > 0.0);
        self.scalar_v1572 = v1572;
        let v1577: f64 = (1.01 - p.p43);
        self.scalar_v1577 = v1577;
        let v1578: f64 = (1.0 / v1577);
        self.scalar_v1578 = v1578;
        let v1591: f64 = (p.p43 - 1.0);
        self.scalar_v1591 = v1591;
        let v1613: bool = (!v1572);
        self.scalar_v1613 = v1613;
        let v1615: f64 = p.p85;
        self.scalar_v1615 = v1615;
        let v1616: bool = (p.p85 > 0.0);
        self.scalar_v1616 = v1616;
        let v1619: f64 = p.p87;
        self.scalar_v1619 = v1619;
        let v1620: f64 = (1.01 - p.p87);
        self.scalar_v1620 = v1620;
        let v1621: f64 = (1.0 / v1620);
        self.scalar_v1621 = v1621;
        let v1634: f64 = (p.p87 - 1.0);
        self.scalar_v1634 = v1634;
        let v1655: bool = (!v1616);
        self.scalar_v1655 = v1655;
        let v1657: f64 = p.p97;
        self.scalar_v1657 = v1657;
        let v1658: bool = (p.p97 > 0.0);
        self.scalar_v1658 = v1658;
        let v1659: f64 = p.p95;
        self.scalar_v1659 = v1659;
        let v1660: bool = (p.p95 > 0.0);
        self.scalar_v1660 = v1660;
        let v1661: bool = (v1658 && v1660);
        self.scalar_v1661 = v1661;
        let v1662: f64 = p.p94;
        self.scalar_v1662 = v1662;
        let v1663: bool = (p.p94 > 0.0);
        self.scalar_v1663 = v1663;
        let v1664: bool = (v1661 && v1663);
        self.scalar_v1664 = v1664;
        let v1679: bool = (!v1663);
        self.scalar_v1679 = v1679;
        let v1680: bool = (v1661 && v1679);
        self.scalar_v1680 = v1680;
        let v1684: f64 = p.p96;
        self.scalar_v1684 = v1684;
        let v1688: bool = (!v1661);
        self.scalar_v1688 = v1688;
        let v1692: bool = (p.p66 > 0.0);
        self.scalar_v1692 = v1692;
        let v1693: bool = (p.p68 > 0.0);
        self.scalar_v1693 = v1693;
        let v1694: bool = (v1692 || v1693);
        self.scalar_v1694 = v1694;
        let v1733: bool = (!v1694);
        self.scalar_v1733 = v1733;
        let v1762: f64 = p.p2;
        self.scalar_v1762 = v1762;
        let v1763: f64 = (-p.p2);
        self.scalar_v1763 = v1763;
        let v1790: bool = (p.p49 > 0.0);
        self.scalar_v1790 = v1790;
        let v1794: f64 = p.p52;
        self.scalar_v1794 = v1794;
        let v1795: bool = (p.p52 <= 0.0);
        self.scalar_v1795 = v1795;
        let v1796: bool = (v1790 && v1795);
        self.scalar_v1796 = v1796;
        let v1801: f64 = (-p.p51);
        self.scalar_v1801 = v1801;
        let v1802: f64 = f64::powf(v778, v1801);
        self.scalar_v1802 = v1802;
        let v1807: f64 = (1.0 - p.p51);
        self.scalar_v1807 = v1807;
        let v1810: f64 = (0.5 * p.p51);
        self.scalar_v1810 = v1810;
        let v1830: bool = (!v1795);
        self.scalar_v1830 = v1830;
        let v1831: bool = (v1790 && v1830);
        self.scalar_v1831 = v1831;
        let v1833: f64 = (4.0 * p.p52);
        self.scalar_v1833 = v1833;
        let v1834: f64 = (p.p52 * v1833);
        self.scalar_v1834 = v1834;
        let v1872: bool = (!v1790);
        self.scalar_v1872 = v1872;
        let v2052: f64 = p.p76;
        self.scalar_v2052 = v2052;
        let v2053: f64 = p.p77;
        self.scalar_v2053 = v2053;
        let v2057: f64 = p.p78;
        self.scalar_v2057 = v2057;
        let v2073: f64 = p.p81;
        self.scalar_v2073 = v2073;
        let v2076: f64 = p.p47;
        self.scalar_v2076 = v2076;
        let v2084: f64 = p.p53;
        self.scalar_v2084 = v2084;
        let v2087: f64 = p.p35;
        self.scalar_v2087 = v2087;
        let v2089: f64 = p.p40;
        self.scalar_v2089 = v2089;
        let v2091: f64 = p.p102;
        self.scalar_v2091 = v2091;
        let v2093: f64 = p.p82;
        self.scalar_v2093 = v2093;
        let v2104: f64 = p.p1;
        self.scalar_v2104 = v2104;
        let v2105: f64 = (if (p.p1 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v2105 = v2105;
        let v2115: f64 = (p.p126 - 1.0);
        self.scalar_v2115 = v2115;
        let v2120: f64 = (p.p109 - 1.0);
        self.scalar_v2120 = v2120;
        let v2126: f64 = (p.p107 - 1.0);
        self.scalar_v2126 = v2126;
        let v2132: f64 = (p.p108 - 1.0);
        self.scalar_v2132 = v2132;
        let v2140: f64 = (p.p106 - 1.0);
        self.scalar_v2140 = v2140;
        let v2146: f64 = (p.p104 - 1.0);
        self.scalar_v2146 = v2146;
        let v2152: f64 = (p.p105 - 1.0);
        self.scalar_v2152 = v2152;
        let v2160: f64 = (p.p103 - 1.0);
        self.scalar_v2160 = v2160;
        let v2165: f64 = (p.p111 - 1.0);
        self.scalar_v2165 = v2165;
        let v2170: f64 = (p.p110 - 1.0);
        self.scalar_v2170 = v2170;
        let v2180: f64 = (v120 - 1.0);
        self.scalar_v2180 = v2180;
        let v2197: f64 = (v164 - 1.0);
        self.scalar_v2197 = v2197;
        let v2213: f64 = (v200 - 1.0);
        self.scalar_v2213 = v2213;
        let v2229: f64 = (v233 - 1.0);
        self.scalar_v2229 = v2229;
        let v2245: f64 = (v254 - 1.0);
        self.scalar_v2245 = v2245;
        let v2261: f64 = (v274 - 1.0);
        self.scalar_v2261 = v2261;
        let v2277: f64 = (v294 - 1.0);
        self.scalar_v2277 = v2277;
        let v2301: f64 = (v336 - 1.0);
        self.scalar_v2301 = v2301;
        let v2317: f64 = (v356 - 1.0);
        self.scalar_v2317 = v2317;
        let v2478: f64 = (p.p38 - 1.0);
        self.scalar_v2478 = v2478;
        let v2496: f64 = (p.p51 - 1.0);
        self.scalar_v2496 = v2496;
        let v2501: f64 = (p.p122 - 1.0);
        self.scalar_v2501 = v2501;
        let v2514: f64 = (p.p112 - 1.0);
        self.scalar_v2514 = v2514;
        let v2624: f64 = (v785 - 1.0);
        self.scalar_v2624 = v2624;
        let v2792: f64 = (v864 - 1.0);
        self.scalar_v2792 = v2792;
        let v3003: f64 = (v976 - 1.0);
        self.scalar_v3003 = v3003;
        let v3257: f64 = (v1083 - 1.0);
        self.scalar_v3257 = v3257;
        let v3276: f64 = (p.p73 - 1.0);
        self.scalar_v3276 = v3276;
        let v4617: f64 = (v1578 - 1.0);
        self.scalar_v4617 = v4617;
        let v4644: f64 = (v1591 - 1.0);
        self.scalar_v4644 = v4644;
        let v4726: f64 = (v1621 - 1.0);
        self.scalar_v4726 = v4726;
        let v4753: f64 = (v1634 - 1.0);
        self.scalar_v4753 = v4753;
        let v4855: f64 = (p.p96 - 1.0);
        self.scalar_v4855 = v4855;
        let v5313: f64 = (v1807 - 1.0);
        self.scalar_v5313 = v5313;
        let v6043: f64 = (-p.p35);
        self.scalar_v6043 = v6043;
        let v6044: f64 = (-p.p40);
        self.scalar_v6044 = v6044;
        let v6045: f64 = (p.p82 * 0.3333333333333333);
        self.scalar_v6045 = v6045;
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
        let v93: bool = (!self.scalar_v85);
        self.scalar_v93 = v93;
        let v94: bool = (self.scalar_v86 && self.scalar_v93);
        self.scalar_v94 = v94;
        let v95: f64 = (if self.scalar_v94 { self.scalar_v92 } else { self.scalar_v92 });
        self.scalar_v95 = v95;
        let v96: f64 = (273.15 + self.scalar_v95);
        self.scalar_v96 = v96;
        let v98: f64 = (self.scalar_v96 * 1.380662e-23);
        self.scalar_v98 = v98;
        let v100: f64 = (self.scalar_v98 / 1.602189e-19);
        self.scalar_v100 = v100;
        let v106: f64 = (self.scalar_v100 * self.scalar_v105);
        self.scalar_v106 = v106;
        let v109: f64 = (self.scalar_v108 / self.scalar_v106);
        self.scalar_v109 = v109;
        let v110: f64 = ((self.scalar_v109) as f64).exp();
        self.scalar_v110 = v110;
        let v127: f64 = (self.scalar_v100 * self.scalar_v119);
        self.scalar_v127 = v127;
        let v170: f64 = (self.scalar_v100 * self.scalar_v163);
        self.scalar_v170 = v170;
        let v206: f64 = (self.scalar_v100 * self.scalar_v199);
        self.scalar_v206 = v206;
        let v239: f64 = (self.scalar_v100 * self.scalar_v232);
        self.scalar_v239 = v239;
        let v260: f64 = (self.scalar_v100 * self.scalar_v253);
        self.scalar_v260 = v260;
        let v280: f64 = (self.scalar_v100 * self.scalar_v273);
        self.scalar_v280 = v280;
        let v300: f64 = (self.scalar_v100 * self.scalar_v293);
        self.scalar_v300 = v300;
        let v342: f64 = (self.scalar_v100 * self.scalar_v335);
        self.scalar_v342 = v342;
        let v362: f64 = (self.scalar_v100 * self.scalar_v355);
        self.scalar_v362 = v362;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
