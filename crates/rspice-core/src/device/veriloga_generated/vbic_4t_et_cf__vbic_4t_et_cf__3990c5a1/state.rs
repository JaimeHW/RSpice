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
            params.p0 = 27.0;
            params.p1 = 0.0;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 1e-16;
            params.p12 = 1.0;
            params.p13 = 1.0;
            params.p14 = 0.9;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.75;
            params.p18 = 0.33;
            params.p19 = -0.5;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 0.75;
            params.p25 = 0.33;
            params.p26 = -0.5;
            params.p27 = 0.0;
            params.p28 = 0.75;
            params.p29 = 0.33;
            params.p30 = -0.5;
            params.p31 = 1e-18;
            params.p32 = 1.0;
            params.p33 = 1.0;
            params.p34 = 0.0;
            params.p35 = 2.0;
            params.p36 = 1e-16;
            params.p37 = 1.0;
            params.p38 = 0.0;
            params.p39 = 2.0;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = 0.0;
            params.p43 = 1.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 1.0;
            params.p49 = 0.0;
            params.p50 = 2.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 0.0;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 1.0;
            params.p65 = 1.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 1.12;
            params.p72 = 1.12;
            params.p73 = 1.12;
            params.p74 = 1.12;
            params.p75 = 1.12;
            params.p76 = 1.12;
            params.p77 = 1.12;
            params.p78 = 3.0;
            params.p79 = 3.0;
            params.p80 = 3.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.1;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 0.5;
            params.p90 = 0.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 1.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 1.12;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 1e-6;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 1.2;
            params.p107 = 0.0;
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
    pub nodes: [usize; 12],
    pub branches: [usize; 0],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 108]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 9]>,
    pub(crate) ddt_state_previous: Box<[f64; 9]>,
    pub(crate) ddt_state_older: Box<[f64; 9]>,
    pub(crate) ddt_state_initialized: Box<[bool; 9]>,
    pub(crate) ddt_derivative_current: Box<[f64; 9]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 9]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v1: f64,
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v305: bool,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: bool,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: bool,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: bool,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: bool,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v326: f64,
    pub(crate) scalar_v327: bool,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v329: f64,
    pub(crate) scalar_v330: f64,
    pub(crate) scalar_v331: bool,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: bool,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: bool,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v390: bool,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v491: f64,
    pub(crate) scalar_v492: bool,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v504: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v514: f64,
    pub(crate) scalar_v515: bool,
    pub(crate) scalar_v516: f64,
    pub(crate) scalar_v547: f64,
    pub(crate) scalar_v548: bool,
    pub(crate) scalar_v549: bool,
    pub(crate) scalar_v550: bool,
    pub(crate) scalar_v551: bool,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v565: f64,
    pub(crate) scalar_v566: f64,
    pub(crate) scalar_v616: f64,
    pub(crate) scalar_v635: bool,
    pub(crate) scalar_v636: bool,
    pub(crate) scalar_v665: f64,
    pub(crate) scalar_v817: bool,
    pub(crate) scalar_v821: f64,
    pub(crate) scalar_v822: bool,
    pub(crate) scalar_v823: bool,
    pub(crate) scalar_v830: f64,
    pub(crate) scalar_v831: f64,
    pub(crate) scalar_v837: f64,
    pub(crate) scalar_v840: f64,
    pub(crate) scalar_v859: bool,
    pub(crate) scalar_v860: bool,
    pub(crate) scalar_v862: f64,
    pub(crate) scalar_v863: f64,
    pub(crate) scalar_v891: f64,
    pub(crate) scalar_v892: f64,
    pub(crate) scalar_v899: bool,
    pub(crate) scalar_v929: f64,
    pub(crate) scalar_v930: bool,
    pub(crate) scalar_v931: f64,
    pub(crate) scalar_v932: f64,
    pub(crate) scalar_v940: bool,
    pub(crate) scalar_v949: bool,
    pub(crate) scalar_v959: f64,
    pub(crate) scalar_v961: f64,
    pub(crate) scalar_v985: bool,
    pub(crate) scalar_v989: f64,
    pub(crate) scalar_v990: bool,
    pub(crate) scalar_v1001: bool,
    pub(crate) scalar_v1002: bool,
    pub(crate) scalar_v1013: f64,
    pub(crate) scalar_v1018: bool,
    pub(crate) scalar_v1019: bool,
    pub(crate) scalar_v1021: bool,
    pub(crate) scalar_v1022: bool,
    pub(crate) scalar_v1023: bool,
    pub(crate) scalar_v1033: bool,
    pub(crate) scalar_v1048: bool,
    pub(crate) scalar_v1050: bool,
    pub(crate) scalar_v1051: bool,
    pub(crate) scalar_v1058: bool,
    pub(crate) scalar_v1072: bool,
    pub(crate) scalar_v1084: f64,
    pub(crate) scalar_v1108: bool,
    pub(crate) scalar_v1109: bool,
    pub(crate) scalar_v1110: bool,
    pub(crate) scalar_v1125: bool,
    pub(crate) scalar_v1127: f64,
    pub(crate) scalar_v1128: bool,
    pub(crate) scalar_v1139: f64,
    pub(crate) scalar_v1149: bool,
    pub(crate) scalar_v1152: bool,
    pub(crate) scalar_v1157: bool,
    pub(crate) scalar_v1169: bool,
    pub(crate) scalar_v1198: bool,
    pub(crate) scalar_v1200: bool,
    pub(crate) scalar_v1205: bool,
    pub(crate) scalar_v1207: bool,
    pub(crate) scalar_v1212: bool,
    pub(crate) scalar_v1214: bool,
    pub(crate) scalar_v1219: bool,
    pub(crate) scalar_v1221: bool,
    pub(crate) scalar_v1226: bool,
    pub(crate) scalar_v1228: bool,
    pub(crate) scalar_v1229: bool,
    pub(crate) scalar_v1230: bool,
    pub(crate) scalar_v1247: bool,
    pub(crate) scalar_v1249: bool,
    pub(crate) scalar_v1254: bool,
    pub(crate) scalar_v1262: f64,
    pub(crate) scalar_v1263: f64,
    pub(crate) scalar_v1267: f64,
    pub(crate) scalar_v1287: f64,
    pub(crate) scalar_v1290: f64,
    pub(crate) scalar_v1298: f64,
    pub(crate) scalar_v1302: f64,
    pub(crate) scalar_v1305: f64,
    pub(crate) scalar_v1337: f64,
    pub(crate) scalar_v1338: bool,
    pub(crate) scalar_v1341: bool,
    pub(crate) scalar_v1343: f64,
    pub(crate) scalar_v1346: f64,
    pub(crate) scalar_v1347: f64,
    pub(crate) scalar_v1352: f64,
    pub(crate) scalar_v1357: f64,
    pub(crate) scalar_v1362: f64,
    pub(crate) scalar_v1367: f64,
    pub(crate) scalar_v1372: f64,
    pub(crate) scalar_v1377: f64,
    pub(crate) scalar_v1382: f64,
    pub(crate) scalar_v1387: f64,
    pub(crate) scalar_v1391: f64,
    pub(crate) scalar_v1392: f64,
    pub(crate) scalar_v1402: f64,
    pub(crate) scalar_v1407: f64,
    pub(crate) scalar_v1411: f64,
    pub(crate) scalar_v1420: f64,
    pub(crate) scalar_v1425: f64,
    pub(crate) scalar_v1434: f64,
    pub(crate) scalar_v1439: f64,
    pub(crate) scalar_v1443: f64,
    pub(crate) scalar_v1452: f64,
    pub(crate) scalar_v1457: f64,
    pub(crate) scalar_v1461: f64,
    pub(crate) scalar_v1470: f64,
    pub(crate) scalar_v1475: f64,
    pub(crate) scalar_v1484: f64,
    pub(crate) scalar_v1489: f64,
    pub(crate) scalar_v1498: f64,
    pub(crate) scalar_v1505: f64,
    pub(crate) scalar_v1514: f64,
    pub(crate) scalar_v1519: f64,
    pub(crate) scalar_v1528: f64,
    pub(crate) scalar_v1533: f64,
    pub(crate) scalar_v1534: f64,
    pub(crate) scalar_v1535: f64,
    pub(crate) scalar_v1538: f64,
    pub(crate) scalar_v1545: f64,
    pub(crate) scalar_v1551: f64,
    pub(crate) scalar_v1571: f64,
    pub(crate) scalar_v1589: f64,
    pub(crate) scalar_v1595: f64,
    pub(crate) scalar_v1610: f64,
    pub(crate) scalar_v1627: f64,
    pub(crate) scalar_v1633: f64,
    pub(crate) scalar_v1648: f64,
    pub(crate) scalar_v1669: f64,
    pub(crate) scalar_v1687: f64,
    pub(crate) scalar_v1696: f64,
    pub(crate) scalar_v1722: f64,
    pub(crate) scalar_v1723: f64,
    pub(crate) scalar_v1728: f64,
    pub(crate) scalar_v1729: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1810: f64,
    pub(crate) scalar_v1811: f64,
    pub(crate) scalar_v1875: f64,
    pub(crate) scalar_v1876: f64,
    pub(crate) scalar_v1883: f64,
    pub(crate) scalar_v1884: f64,
    pub(crate) scalar_v1965: f64,
    pub(crate) scalar_v1966: f64,
    pub(crate) scalar_v2049: f64,
    pub(crate) scalar_v2050: f64,
    pub(crate) scalar_v2051: f64,
    pub(crate) scalar_v2052: f64,
    pub(crate) scalar_v2060: f64,
    pub(crate) scalar_v2061: f64,
    pub(crate) scalar_v2062: f64,
    pub(crate) scalar_v2063: f64,
    pub(crate) scalar_v2100: f64,
    pub(crate) scalar_v2104: f64,
    pub(crate) scalar_v2335: f64,
    pub(crate) scalar_v2422: f64,
    pub(crate) scalar_v2423: f64,
    pub(crate) scalar_v2424: f64,
    pub(crate) scalar_v2425: f64,
    pub(crate) scalar_v2523: f64,
    pub(crate) scalar_v2524: f64,
    pub(crate) scalar_v2525: f64,
    pub(crate) scalar_v2526: f64,
    pub(crate) scalar_v2535: f64,
    pub(crate) scalar_v2536: f64,
    pub(crate) scalar_v2537: f64,
    pub(crate) scalar_v2538: f64,
    pub(crate) scalar_v2917: f64,
    pub(crate) scalar_v2918: f64,
    pub(crate) scalar_v2919: f64,
    pub(crate) scalar_v2920: f64,
    pub(crate) scalar_v3038: f64,
    pub(crate) scalar_v3039: f64,
    pub(crate) scalar_v3040: f64,
    pub(crate) scalar_v3041: f64,
    pub(crate) scalar_v3042: f64,
    pub(crate) scalar_v3043: f64,
    pub(crate) scalar_v3053: f64,
    pub(crate) scalar_v3054: f64,
    pub(crate) scalar_v3055: f64,
    pub(crate) scalar_v3056: f64,
    pub(crate) scalar_v3057: f64,
    pub(crate) scalar_v3058: f64,
    pub(crate) scalar_v3112: f64,
    pub(crate) scalar_v3179: f64,
    pub(crate) scalar_v3180: f64,
    pub(crate) scalar_v3181: f64,
    pub(crate) scalar_v3182: f64,
    pub(crate) scalar_v3183: f64,
    pub(crate) scalar_v3184: f64,
    pub(crate) scalar_v3408: f64,
    pub(crate) scalar_v3424: f64,
    pub(crate) scalar_v3513: f64,
    pub(crate) scalar_v3686: f64,
    pub(crate) scalar_v3715: f64,
    pub(crate) scalar_v4115: f64,
    pub(crate) scalar_v4126: f64,
    pub(crate) scalar_v4246: f64,
    pub(crate) scalar_v4247: f64,
    pub(crate) scalar_v4598: f64,
    pub(crate) scalar_v4621: f64,
    pub(crate) scalar_v4720: f64,
    pub(crate) scalar_v4721: f64,
    pub(crate) scalar_v4722: f64,
    pub(crate) scalar_v4858: f64,
    pub(crate) scalar_v4861: f64,
    pub(crate) scalar_v4862: f64,
    pub(crate) scalar_v5045: f64,
    pub(crate) scalar_v5046: f64,
    pub(crate) scalar_v5047: f64,
    pub(crate) scalar_v5: f64,
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
            scalar_v1: self.scalar_v1,
            scalar_v2: self.scalar_v2,
            scalar_v4: self.scalar_v4,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v133: self.scalar_v133,
            scalar_v135: self.scalar_v135,
            scalar_v137: self.scalar_v137,
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v159: self.scalar_v159,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v192: self.scalar_v192,
            scalar_v221: self.scalar_v221,
            scalar_v222: self.scalar_v222,
            scalar_v226: self.scalar_v226,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v253: self.scalar_v253,
            scalar_v275: self.scalar_v275,
            scalar_v277: self.scalar_v277,
            scalar_v280: self.scalar_v280,
            scalar_v282: self.scalar_v282,
            scalar_v285: self.scalar_v285,
            scalar_v287: self.scalar_v287,
            scalar_v289: self.scalar_v289,
            scalar_v292: self.scalar_v292,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v303: self.scalar_v303,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
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
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v351: self.scalar_v351,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v359: self.scalar_v359,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v368: self.scalar_v368,
            scalar_v371: self.scalar_v371,
            scalar_v390: self.scalar_v390,
            scalar_v392: self.scalar_v392,
            scalar_v393: self.scalar_v393,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v491: self.scalar_v491,
            scalar_v492: self.scalar_v492,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v504: self.scalar_v504,
            scalar_v507: self.scalar_v507,
            scalar_v514: self.scalar_v514,
            scalar_v515: self.scalar_v515,
            scalar_v516: self.scalar_v516,
            scalar_v547: self.scalar_v547,
            scalar_v548: self.scalar_v548,
            scalar_v549: self.scalar_v549,
            scalar_v550: self.scalar_v550,
            scalar_v551: self.scalar_v551,
            scalar_v559: self.scalar_v559,
            scalar_v560: self.scalar_v560,
            scalar_v565: self.scalar_v565,
            scalar_v566: self.scalar_v566,
            scalar_v616: self.scalar_v616,
            scalar_v635: self.scalar_v635,
            scalar_v636: self.scalar_v636,
            scalar_v665: self.scalar_v665,
            scalar_v817: self.scalar_v817,
            scalar_v821: self.scalar_v821,
            scalar_v822: self.scalar_v822,
            scalar_v823: self.scalar_v823,
            scalar_v830: self.scalar_v830,
            scalar_v831: self.scalar_v831,
            scalar_v837: self.scalar_v837,
            scalar_v840: self.scalar_v840,
            scalar_v859: self.scalar_v859,
            scalar_v860: self.scalar_v860,
            scalar_v862: self.scalar_v862,
            scalar_v863: self.scalar_v863,
            scalar_v891: self.scalar_v891,
            scalar_v892: self.scalar_v892,
            scalar_v899: self.scalar_v899,
            scalar_v929: self.scalar_v929,
            scalar_v930: self.scalar_v930,
            scalar_v931: self.scalar_v931,
            scalar_v932: self.scalar_v932,
            scalar_v940: self.scalar_v940,
            scalar_v949: self.scalar_v949,
            scalar_v959: self.scalar_v959,
            scalar_v961: self.scalar_v961,
            scalar_v985: self.scalar_v985,
            scalar_v989: self.scalar_v989,
            scalar_v990: self.scalar_v990,
            scalar_v1001: self.scalar_v1001,
            scalar_v1002: self.scalar_v1002,
            scalar_v1013: self.scalar_v1013,
            scalar_v1018: self.scalar_v1018,
            scalar_v1019: self.scalar_v1019,
            scalar_v1021: self.scalar_v1021,
            scalar_v1022: self.scalar_v1022,
            scalar_v1023: self.scalar_v1023,
            scalar_v1033: self.scalar_v1033,
            scalar_v1048: self.scalar_v1048,
            scalar_v1050: self.scalar_v1050,
            scalar_v1051: self.scalar_v1051,
            scalar_v1058: self.scalar_v1058,
            scalar_v1072: self.scalar_v1072,
            scalar_v1084: self.scalar_v1084,
            scalar_v1108: self.scalar_v1108,
            scalar_v1109: self.scalar_v1109,
            scalar_v1110: self.scalar_v1110,
            scalar_v1125: self.scalar_v1125,
            scalar_v1127: self.scalar_v1127,
            scalar_v1128: self.scalar_v1128,
            scalar_v1139: self.scalar_v1139,
            scalar_v1149: self.scalar_v1149,
            scalar_v1152: self.scalar_v1152,
            scalar_v1157: self.scalar_v1157,
            scalar_v1169: self.scalar_v1169,
            scalar_v1198: self.scalar_v1198,
            scalar_v1200: self.scalar_v1200,
            scalar_v1205: self.scalar_v1205,
            scalar_v1207: self.scalar_v1207,
            scalar_v1212: self.scalar_v1212,
            scalar_v1214: self.scalar_v1214,
            scalar_v1219: self.scalar_v1219,
            scalar_v1221: self.scalar_v1221,
            scalar_v1226: self.scalar_v1226,
            scalar_v1228: self.scalar_v1228,
            scalar_v1229: self.scalar_v1229,
            scalar_v1230: self.scalar_v1230,
            scalar_v1247: self.scalar_v1247,
            scalar_v1249: self.scalar_v1249,
            scalar_v1254: self.scalar_v1254,
            scalar_v1262: self.scalar_v1262,
            scalar_v1263: self.scalar_v1263,
            scalar_v1267: self.scalar_v1267,
            scalar_v1287: self.scalar_v1287,
            scalar_v1290: self.scalar_v1290,
            scalar_v1298: self.scalar_v1298,
            scalar_v1302: self.scalar_v1302,
            scalar_v1305: self.scalar_v1305,
            scalar_v1337: self.scalar_v1337,
            scalar_v1338: self.scalar_v1338,
            scalar_v1341: self.scalar_v1341,
            scalar_v1343: self.scalar_v1343,
            scalar_v1346: self.scalar_v1346,
            scalar_v1347: self.scalar_v1347,
            scalar_v1352: self.scalar_v1352,
            scalar_v1357: self.scalar_v1357,
            scalar_v1362: self.scalar_v1362,
            scalar_v1367: self.scalar_v1367,
            scalar_v1372: self.scalar_v1372,
            scalar_v1377: self.scalar_v1377,
            scalar_v1382: self.scalar_v1382,
            scalar_v1387: self.scalar_v1387,
            scalar_v1391: self.scalar_v1391,
            scalar_v1392: self.scalar_v1392,
            scalar_v1402: self.scalar_v1402,
            scalar_v1407: self.scalar_v1407,
            scalar_v1411: self.scalar_v1411,
            scalar_v1420: self.scalar_v1420,
            scalar_v1425: self.scalar_v1425,
            scalar_v1434: self.scalar_v1434,
            scalar_v1439: self.scalar_v1439,
            scalar_v1443: self.scalar_v1443,
            scalar_v1452: self.scalar_v1452,
            scalar_v1457: self.scalar_v1457,
            scalar_v1461: self.scalar_v1461,
            scalar_v1470: self.scalar_v1470,
            scalar_v1475: self.scalar_v1475,
            scalar_v1484: self.scalar_v1484,
            scalar_v1489: self.scalar_v1489,
            scalar_v1498: self.scalar_v1498,
            scalar_v1505: self.scalar_v1505,
            scalar_v1514: self.scalar_v1514,
            scalar_v1519: self.scalar_v1519,
            scalar_v1528: self.scalar_v1528,
            scalar_v1533: self.scalar_v1533,
            scalar_v1534: self.scalar_v1534,
            scalar_v1535: self.scalar_v1535,
            scalar_v1538: self.scalar_v1538,
            scalar_v1545: self.scalar_v1545,
            scalar_v1551: self.scalar_v1551,
            scalar_v1571: self.scalar_v1571,
            scalar_v1589: self.scalar_v1589,
            scalar_v1595: self.scalar_v1595,
            scalar_v1610: self.scalar_v1610,
            scalar_v1627: self.scalar_v1627,
            scalar_v1633: self.scalar_v1633,
            scalar_v1648: self.scalar_v1648,
            scalar_v1669: self.scalar_v1669,
            scalar_v1687: self.scalar_v1687,
            scalar_v1696: self.scalar_v1696,
            scalar_v1722: self.scalar_v1722,
            scalar_v1723: self.scalar_v1723,
            scalar_v1728: self.scalar_v1728,
            scalar_v1729: self.scalar_v1729,
            scalar_v1759: self.scalar_v1759,
            scalar_v1810: self.scalar_v1810,
            scalar_v1811: self.scalar_v1811,
            scalar_v1875: self.scalar_v1875,
            scalar_v1876: self.scalar_v1876,
            scalar_v1883: self.scalar_v1883,
            scalar_v1884: self.scalar_v1884,
            scalar_v1965: self.scalar_v1965,
            scalar_v1966: self.scalar_v1966,
            scalar_v2049: self.scalar_v2049,
            scalar_v2050: self.scalar_v2050,
            scalar_v2051: self.scalar_v2051,
            scalar_v2052: self.scalar_v2052,
            scalar_v2060: self.scalar_v2060,
            scalar_v2061: self.scalar_v2061,
            scalar_v2062: self.scalar_v2062,
            scalar_v2063: self.scalar_v2063,
            scalar_v2100: self.scalar_v2100,
            scalar_v2104: self.scalar_v2104,
            scalar_v2335: self.scalar_v2335,
            scalar_v2422: self.scalar_v2422,
            scalar_v2423: self.scalar_v2423,
            scalar_v2424: self.scalar_v2424,
            scalar_v2425: self.scalar_v2425,
            scalar_v2523: self.scalar_v2523,
            scalar_v2524: self.scalar_v2524,
            scalar_v2525: self.scalar_v2525,
            scalar_v2526: self.scalar_v2526,
            scalar_v2535: self.scalar_v2535,
            scalar_v2536: self.scalar_v2536,
            scalar_v2537: self.scalar_v2537,
            scalar_v2538: self.scalar_v2538,
            scalar_v2917: self.scalar_v2917,
            scalar_v2918: self.scalar_v2918,
            scalar_v2919: self.scalar_v2919,
            scalar_v2920: self.scalar_v2920,
            scalar_v3038: self.scalar_v3038,
            scalar_v3039: self.scalar_v3039,
            scalar_v3040: self.scalar_v3040,
            scalar_v3041: self.scalar_v3041,
            scalar_v3042: self.scalar_v3042,
            scalar_v3043: self.scalar_v3043,
            scalar_v3053: self.scalar_v3053,
            scalar_v3054: self.scalar_v3054,
            scalar_v3055: self.scalar_v3055,
            scalar_v3056: self.scalar_v3056,
            scalar_v3057: self.scalar_v3057,
            scalar_v3058: self.scalar_v3058,
            scalar_v3112: self.scalar_v3112,
            scalar_v3179: self.scalar_v3179,
            scalar_v3180: self.scalar_v3180,
            scalar_v3181: self.scalar_v3181,
            scalar_v3182: self.scalar_v3182,
            scalar_v3183: self.scalar_v3183,
            scalar_v3184: self.scalar_v3184,
            scalar_v3408: self.scalar_v3408,
            scalar_v3424: self.scalar_v3424,
            scalar_v3513: self.scalar_v3513,
            scalar_v3686: self.scalar_v3686,
            scalar_v3715: self.scalar_v3715,
            scalar_v4115: self.scalar_v4115,
            scalar_v4126: self.scalar_v4126,
            scalar_v4246: self.scalar_v4246,
            scalar_v4247: self.scalar_v4247,
            scalar_v4598: self.scalar_v4598,
            scalar_v4621: self.scalar_v4621,
            scalar_v4720: self.scalar_v4720,
            scalar_v4721: self.scalar_v4721,
            scalar_v4722: self.scalar_v4722,
            scalar_v4858: self.scalar_v4858,
            scalar_v4861: self.scalar_v4861,
            scalar_v4862: self.scalar_v4862,
            scalar_v5045: self.scalar_v5045,
            scalar_v5046: self.scalar_v5046,
            scalar_v5047: self.scalar_v5047,
            scalar_v5: self.scalar_v5,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["cx", "ci", "bx", "bi", "ei", "bp", "si"];

    pub const BRANCH_COUNT: usize = 0;
    pub const PARAMETER_COUNT: usize = 108;
    pub const VARIABLE_COUNT: usize = 171;
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
            scalar_v1: 0.0,
            scalar_v2: 0.0,
            scalar_v4: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v34: 0.0,
            scalar_v35: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v64: 0.0,
            scalar_v65: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v133: 0.0,
            scalar_v135: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v148: 0.0,
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v159: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v192: 0.0,
            scalar_v221: 0.0,
            scalar_v222: 0.0,
            scalar_v226: 0.0,
            scalar_v248: 0.0,
            scalar_v249: 0.0,
            scalar_v253: 0.0,
            scalar_v275: 0.0,
            scalar_v277: 0.0,
            scalar_v280: 0.0,
            scalar_v282: 0.0,
            scalar_v285: 0.0,
            scalar_v287: 0.0,
            scalar_v289: 0.0,
            scalar_v292: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v303: 0.0,
            scalar_v305: false,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v309: false,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: false,
            scalar_v315: 0.0,
            scalar_v316: false,
            scalar_v317: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v320: false,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: false,
            scalar_v326: 0.0,
            scalar_v327: false,
            scalar_v328: 0.0,
            scalar_v329: 0.0,
            scalar_v330: 0.0,
            scalar_v331: false,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: false,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v338: 0.0,
            scalar_v351: 0.0,
            scalar_v353: 0.0,
            scalar_v354: false,
            scalar_v359: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v368: 0.0,
            scalar_v371: 0.0,
            scalar_v390: false,
            scalar_v392: 0.0,
            scalar_v393: 0.0,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v491: 0.0,
            scalar_v492: false,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v504: 0.0,
            scalar_v507: 0.0,
            scalar_v514: 0.0,
            scalar_v515: false,
            scalar_v516: 0.0,
            scalar_v547: 0.0,
            scalar_v548: false,
            scalar_v549: false,
            scalar_v550: false,
            scalar_v551: false,
            scalar_v559: 0.0,
            scalar_v560: 0.0,
            scalar_v565: 0.0,
            scalar_v566: 0.0,
            scalar_v616: 0.0,
            scalar_v635: false,
            scalar_v636: false,
            scalar_v665: 0.0,
            scalar_v817: false,
            scalar_v821: 0.0,
            scalar_v822: false,
            scalar_v823: false,
            scalar_v830: 0.0,
            scalar_v831: 0.0,
            scalar_v837: 0.0,
            scalar_v840: 0.0,
            scalar_v859: false,
            scalar_v860: false,
            scalar_v862: 0.0,
            scalar_v863: 0.0,
            scalar_v891: 0.0,
            scalar_v892: 0.0,
            scalar_v899: false,
            scalar_v929: 0.0,
            scalar_v930: false,
            scalar_v931: 0.0,
            scalar_v932: 0.0,
            scalar_v940: false,
            scalar_v949: false,
            scalar_v959: 0.0,
            scalar_v961: 0.0,
            scalar_v985: false,
            scalar_v989: 0.0,
            scalar_v990: false,
            scalar_v1001: false,
            scalar_v1002: false,
            scalar_v1013: 0.0,
            scalar_v1018: false,
            scalar_v1019: false,
            scalar_v1021: false,
            scalar_v1022: false,
            scalar_v1023: false,
            scalar_v1033: false,
            scalar_v1048: false,
            scalar_v1050: false,
            scalar_v1051: false,
            scalar_v1058: false,
            scalar_v1072: false,
            scalar_v1084: 0.0,
            scalar_v1108: false,
            scalar_v1109: false,
            scalar_v1110: false,
            scalar_v1125: false,
            scalar_v1127: 0.0,
            scalar_v1128: false,
            scalar_v1139: 0.0,
            scalar_v1149: false,
            scalar_v1152: false,
            scalar_v1157: false,
            scalar_v1169: false,
            scalar_v1198: false,
            scalar_v1200: false,
            scalar_v1205: false,
            scalar_v1207: false,
            scalar_v1212: false,
            scalar_v1214: false,
            scalar_v1219: false,
            scalar_v1221: false,
            scalar_v1226: false,
            scalar_v1228: false,
            scalar_v1229: false,
            scalar_v1230: false,
            scalar_v1247: false,
            scalar_v1249: false,
            scalar_v1254: false,
            scalar_v1262: 0.0,
            scalar_v1263: 0.0,
            scalar_v1267: 0.0,
            scalar_v1287: 0.0,
            scalar_v1290: 0.0,
            scalar_v1298: 0.0,
            scalar_v1302: 0.0,
            scalar_v1305: 0.0,
            scalar_v1337: 0.0,
            scalar_v1338: false,
            scalar_v1341: false,
            scalar_v1343: 0.0,
            scalar_v1346: 0.0,
            scalar_v1347: 0.0,
            scalar_v1352: 0.0,
            scalar_v1357: 0.0,
            scalar_v1362: 0.0,
            scalar_v1367: 0.0,
            scalar_v1372: 0.0,
            scalar_v1377: 0.0,
            scalar_v1382: 0.0,
            scalar_v1387: 0.0,
            scalar_v1391: 0.0,
            scalar_v1392: 0.0,
            scalar_v1402: 0.0,
            scalar_v1407: 0.0,
            scalar_v1411: 0.0,
            scalar_v1420: 0.0,
            scalar_v1425: 0.0,
            scalar_v1434: 0.0,
            scalar_v1439: 0.0,
            scalar_v1443: 0.0,
            scalar_v1452: 0.0,
            scalar_v1457: 0.0,
            scalar_v1461: 0.0,
            scalar_v1470: 0.0,
            scalar_v1475: 0.0,
            scalar_v1484: 0.0,
            scalar_v1489: 0.0,
            scalar_v1498: 0.0,
            scalar_v1505: 0.0,
            scalar_v1514: 0.0,
            scalar_v1519: 0.0,
            scalar_v1528: 0.0,
            scalar_v1533: 0.0,
            scalar_v1534: 0.0,
            scalar_v1535: 0.0,
            scalar_v1538: 0.0,
            scalar_v1545: 0.0,
            scalar_v1551: 0.0,
            scalar_v1571: 0.0,
            scalar_v1589: 0.0,
            scalar_v1595: 0.0,
            scalar_v1610: 0.0,
            scalar_v1627: 0.0,
            scalar_v1633: 0.0,
            scalar_v1648: 0.0,
            scalar_v1669: 0.0,
            scalar_v1687: 0.0,
            scalar_v1696: 0.0,
            scalar_v1722: 0.0,
            scalar_v1723: 0.0,
            scalar_v1728: 0.0,
            scalar_v1729: 0.0,
            scalar_v1759: 0.0,
            scalar_v1810: 0.0,
            scalar_v1811: 0.0,
            scalar_v1875: 0.0,
            scalar_v1876: 0.0,
            scalar_v1883: 0.0,
            scalar_v1884: 0.0,
            scalar_v1965: 0.0,
            scalar_v1966: 0.0,
            scalar_v2049: 0.0,
            scalar_v2050: 0.0,
            scalar_v2051: 0.0,
            scalar_v2052: 0.0,
            scalar_v2060: 0.0,
            scalar_v2061: 0.0,
            scalar_v2062: 0.0,
            scalar_v2063: 0.0,
            scalar_v2100: 0.0,
            scalar_v2104: 0.0,
            scalar_v2335: 0.0,
            scalar_v2422: 0.0,
            scalar_v2423: 0.0,
            scalar_v2424: 0.0,
            scalar_v2425: 0.0,
            scalar_v2523: 0.0,
            scalar_v2524: 0.0,
            scalar_v2525: 0.0,
            scalar_v2526: 0.0,
            scalar_v2535: 0.0,
            scalar_v2536: 0.0,
            scalar_v2537: 0.0,
            scalar_v2538: 0.0,
            scalar_v2917: 0.0,
            scalar_v2918: 0.0,
            scalar_v2919: 0.0,
            scalar_v2920: 0.0,
            scalar_v3038: 0.0,
            scalar_v3039: 0.0,
            scalar_v3040: 0.0,
            scalar_v3041: 0.0,
            scalar_v3042: 0.0,
            scalar_v3043: 0.0,
            scalar_v3053: 0.0,
            scalar_v3054: 0.0,
            scalar_v3055: 0.0,
            scalar_v3056: 0.0,
            scalar_v3057: 0.0,
            scalar_v3058: 0.0,
            scalar_v3112: 0.0,
            scalar_v3179: 0.0,
            scalar_v3180: 0.0,
            scalar_v3181: 0.0,
            scalar_v3182: 0.0,
            scalar_v3183: 0.0,
            scalar_v3184: 0.0,
            scalar_v3408: 0.0,
            scalar_v3424: 0.0,
            scalar_v3513: 0.0,
            scalar_v3686: 0.0,
            scalar_v3715: 0.0,
            scalar_v4115: 0.0,
            scalar_v4126: 0.0,
            scalar_v4246: 0.0,
            scalar_v4247: 0.0,
            scalar_v4598: 0.0,
            scalar_v4621: 0.0,
            scalar_v4720: 0.0,
            scalar_v4721: 0.0,
            scalar_v4722: 0.0,
            scalar_v4858: 0.0,
            scalar_v4861: 0.0,
            scalar_v4862: 0.0,
            scalar_v5045: 0.0,
            scalar_v5046: 0.0,
            scalar_v5047: 0.0,
            scalar_v5: 0.0,
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
            scalar_v1,
            scalar_v2,
            scalar_v4,
            scalar_v14,
            scalar_v15,
            scalar_v18,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v26,
            scalar_v27,
            scalar_v30,
            scalar_v31,
            scalar_v34,
            scalar_v35,
            scalar_v38,
            scalar_v39,
            scalar_v42,
            scalar_v43,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v57,
            scalar_v58,
            scalar_v61,
            scalar_v62,
            scalar_v64,
            scalar_v65,
            scalar_v70,
            scalar_v71,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v88,
            scalar_v89,
            scalar_v94,
            scalar_v95,
            scalar_v98,
            scalar_v99,
            scalar_v101,
            scalar_v102,
            scalar_v107,
            scalar_v108,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v118,
            scalar_v119,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v129,
            scalar_v130,
            scalar_v133,
            scalar_v135,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v144,
            scalar_v145,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v155,
            scalar_v156,
            scalar_v159,
            scalar_v164,
            scalar_v165,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v177,
            scalar_v178,
            scalar_v186,
            scalar_v187,
            scalar_v192,
            scalar_v221,
            scalar_v222,
            scalar_v226,
            scalar_v248,
            scalar_v249,
            scalar_v253,
            scalar_v275,
            scalar_v277,
            scalar_v280,
            scalar_v282,
            scalar_v285,
            scalar_v287,
            scalar_v289,
            scalar_v292,
            scalar_v295,
            scalar_v296,
            scalar_v303,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
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
            scalar_v337,
            scalar_v338,
            scalar_v351,
            scalar_v353,
            scalar_v354,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v368,
            scalar_v371,
            scalar_v390,
            scalar_v392,
            scalar_v393,
            scalar_v421,
            scalar_v422,
            scalar_v491,
            scalar_v492,
            scalar_v497,
            scalar_v498,
            scalar_v504,
            scalar_v507,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v559,
            scalar_v560,
            scalar_v565,
            scalar_v566,
            scalar_v616,
            scalar_v635,
            scalar_v636,
            scalar_v665,
            scalar_v817,
            scalar_v821,
            scalar_v822,
            scalar_v823,
            scalar_v830,
            scalar_v831,
            scalar_v837,
            scalar_v840,
            scalar_v859,
            scalar_v860,
            scalar_v862,
            scalar_v863,
            scalar_v891,
            scalar_v892,
            scalar_v899,
            scalar_v929,
            scalar_v930,
            scalar_v931,
            scalar_v932,
            scalar_v940,
            scalar_v949,
            scalar_v959,
            scalar_v961,
            scalar_v985,
            scalar_v989,
            scalar_v990,
            scalar_v1001,
            scalar_v1002,
            scalar_v1013,
            scalar_v1018,
            scalar_v1019,
            scalar_v1021,
            scalar_v1022,
            scalar_v1023,
            scalar_v1033,
            scalar_v1048,
            scalar_v1050,
            scalar_v1051,
            scalar_v1058,
            scalar_v1072,
            scalar_v1084,
            scalar_v1108,
            scalar_v1109,
            scalar_v1110,
            scalar_v1125,
            scalar_v1127,
            scalar_v1128,
            scalar_v1139,
            scalar_v1149,
            scalar_v1152,
            scalar_v1157,
            scalar_v1169,
            scalar_v1198,
            scalar_v1200,
            scalar_v1205,
            scalar_v1207,
            scalar_v1212,
            scalar_v1214,
            scalar_v1219,
            scalar_v1221,
            scalar_v1226,
            scalar_v1228,
            scalar_v1229,
            scalar_v1230,
            scalar_v1247,
            scalar_v1249,
            scalar_v1254,
            scalar_v1262,
            scalar_v1263,
            scalar_v1267,
            scalar_v1287,
            scalar_v1290,
            scalar_v1298,
            scalar_v1302,
            scalar_v1305,
            scalar_v1337,
            scalar_v1338,
            scalar_v1341,
            scalar_v1343,
            scalar_v1346,
            scalar_v1347,
            scalar_v1352,
            scalar_v1357,
            scalar_v1362,
            scalar_v1367,
            scalar_v1372,
            scalar_v1377,
            scalar_v1382,
            scalar_v1387,
            scalar_v1391,
            scalar_v1392,
            scalar_v1402,
            scalar_v1407,
            scalar_v1411,
            scalar_v1420,
            scalar_v1425,
            scalar_v1434,
            scalar_v1439,
            scalar_v1443,
            scalar_v1452,
            scalar_v1457,
            scalar_v1461,
            scalar_v1470,
            scalar_v1475,
            scalar_v1484,
            scalar_v1489,
            scalar_v1498,
            scalar_v1505,
            scalar_v1514,
            scalar_v1519,
            scalar_v1528,
            scalar_v1533,
            scalar_v1534,
            scalar_v1535,
            scalar_v1538,
            scalar_v1545,
            scalar_v1551,
            scalar_v1571,
            scalar_v1589,
            scalar_v1595,
            scalar_v1610,
            scalar_v1627,
            scalar_v1633,
            scalar_v1648,
            scalar_v1669,
            scalar_v1687,
            scalar_v1696,
            scalar_v1722,
            scalar_v1723,
            scalar_v1728,
            scalar_v1729,
            scalar_v1759,
            scalar_v1810,
            scalar_v1811,
            scalar_v1875,
            scalar_v1876,
            scalar_v1883,
            scalar_v1884,
            scalar_v1965,
            scalar_v1966,
            scalar_v2049,
            scalar_v2050,
            scalar_v2051,
            scalar_v2052,
            scalar_v2060,
            scalar_v2061,
            scalar_v2062,
            scalar_v2063,
            scalar_v2100,
            scalar_v2104,
            scalar_v2335,
            scalar_v2422,
            scalar_v2423,
            scalar_v2424,
            scalar_v2425,
            scalar_v2523,
            scalar_v2524,
            scalar_v2525,
            scalar_v2526,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v2538,
            scalar_v2917,
            scalar_v2918,
            scalar_v2919,
            scalar_v2920,
            scalar_v3038,
            scalar_v3039,
            scalar_v3040,
            scalar_v3041,
            scalar_v3042,
            scalar_v3043,
            scalar_v3053,
            scalar_v3054,
            scalar_v3055,
            scalar_v3056,
            scalar_v3057,
            scalar_v3058,
            scalar_v3112,
            scalar_v3179,
            scalar_v3180,
            scalar_v3181,
            scalar_v3182,
            scalar_v3183,
            scalar_v3184,
            scalar_v3408,
            scalar_v3424,
            scalar_v3513,
            scalar_v3686,
            scalar_v3715,
            scalar_v4115,
            scalar_v4126,
            scalar_v4246,
            scalar_v4247,
            scalar_v4598,
            scalar_v4621,
            scalar_v4720,
            scalar_v4721,
            scalar_v4722,
            scalar_v4858,
            scalar_v4861,
            scalar_v4862,
            scalar_v5045,
            scalar_v5046,
            scalar_v5047,
            scalar_v5,
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
            scalar_v1,
            scalar_v2,
            scalar_v4,
            scalar_v14,
            scalar_v15,
            scalar_v18,
            scalar_v19,
            scalar_v22,
            scalar_v23,
            scalar_v26,
            scalar_v27,
            scalar_v30,
            scalar_v31,
            scalar_v34,
            scalar_v35,
            scalar_v38,
            scalar_v39,
            scalar_v42,
            scalar_v43,
            scalar_v46,
            scalar_v47,
            scalar_v49,
            scalar_v50,
            scalar_v57,
            scalar_v58,
            scalar_v61,
            scalar_v62,
            scalar_v64,
            scalar_v65,
            scalar_v70,
            scalar_v71,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v81,
            scalar_v82,
            scalar_v85,
            scalar_v86,
            scalar_v88,
            scalar_v89,
            scalar_v94,
            scalar_v95,
            scalar_v98,
            scalar_v99,
            scalar_v101,
            scalar_v102,
            scalar_v107,
            scalar_v108,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v118,
            scalar_v119,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v129,
            scalar_v130,
            scalar_v133,
            scalar_v135,
            scalar_v137,
            scalar_v138,
            scalar_v139,
            scalar_v144,
            scalar_v145,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v155,
            scalar_v156,
            scalar_v159,
            scalar_v164,
            scalar_v165,
            scalar_v169,
            scalar_v170,
            scalar_v171,
            scalar_v177,
            scalar_v178,
            scalar_v186,
            scalar_v187,
            scalar_v192,
            scalar_v221,
            scalar_v222,
            scalar_v226,
            scalar_v248,
            scalar_v249,
            scalar_v253,
            scalar_v275,
            scalar_v277,
            scalar_v280,
            scalar_v282,
            scalar_v285,
            scalar_v287,
            scalar_v289,
            scalar_v292,
            scalar_v295,
            scalar_v296,
            scalar_v303,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
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
            scalar_v337,
            scalar_v338,
            scalar_v351,
            scalar_v353,
            scalar_v354,
            scalar_v359,
            scalar_v361,
            scalar_v362,
            scalar_v368,
            scalar_v371,
            scalar_v390,
            scalar_v392,
            scalar_v393,
            scalar_v421,
            scalar_v422,
            scalar_v491,
            scalar_v492,
            scalar_v497,
            scalar_v498,
            scalar_v504,
            scalar_v507,
            scalar_v514,
            scalar_v515,
            scalar_v516,
            scalar_v547,
            scalar_v548,
            scalar_v549,
            scalar_v550,
            scalar_v551,
            scalar_v559,
            scalar_v560,
            scalar_v565,
            scalar_v566,
            scalar_v616,
            scalar_v635,
            scalar_v636,
            scalar_v665,
            scalar_v817,
            scalar_v821,
            scalar_v822,
            scalar_v823,
            scalar_v830,
            scalar_v831,
            scalar_v837,
            scalar_v840,
            scalar_v859,
            scalar_v860,
            scalar_v862,
            scalar_v863,
            scalar_v891,
            scalar_v892,
            scalar_v899,
            scalar_v929,
            scalar_v930,
            scalar_v931,
            scalar_v932,
            scalar_v940,
            scalar_v949,
            scalar_v959,
            scalar_v961,
            scalar_v985,
            scalar_v989,
            scalar_v990,
            scalar_v1001,
            scalar_v1002,
            scalar_v1013,
            scalar_v1018,
            scalar_v1019,
            scalar_v1021,
            scalar_v1022,
            scalar_v1023,
            scalar_v1033,
            scalar_v1048,
            scalar_v1050,
            scalar_v1051,
            scalar_v1058,
            scalar_v1072,
            scalar_v1084,
            scalar_v1108,
            scalar_v1109,
            scalar_v1110,
            scalar_v1125,
            scalar_v1127,
            scalar_v1128,
            scalar_v1139,
            scalar_v1149,
            scalar_v1152,
            scalar_v1157,
            scalar_v1169,
            scalar_v1198,
            scalar_v1200,
            scalar_v1205,
            scalar_v1207,
            scalar_v1212,
            scalar_v1214,
            scalar_v1219,
            scalar_v1221,
            scalar_v1226,
            scalar_v1228,
            scalar_v1229,
            scalar_v1230,
            scalar_v1247,
            scalar_v1249,
            scalar_v1254,
            scalar_v1262,
            scalar_v1263,
            scalar_v1267,
            scalar_v1287,
            scalar_v1290,
            scalar_v1298,
            scalar_v1302,
            scalar_v1305,
            scalar_v1337,
            scalar_v1338,
            scalar_v1341,
            scalar_v1343,
            scalar_v1346,
            scalar_v1347,
            scalar_v1352,
            scalar_v1357,
            scalar_v1362,
            scalar_v1367,
            scalar_v1372,
            scalar_v1377,
            scalar_v1382,
            scalar_v1387,
            scalar_v1391,
            scalar_v1392,
            scalar_v1402,
            scalar_v1407,
            scalar_v1411,
            scalar_v1420,
            scalar_v1425,
            scalar_v1434,
            scalar_v1439,
            scalar_v1443,
            scalar_v1452,
            scalar_v1457,
            scalar_v1461,
            scalar_v1470,
            scalar_v1475,
            scalar_v1484,
            scalar_v1489,
            scalar_v1498,
            scalar_v1505,
            scalar_v1514,
            scalar_v1519,
            scalar_v1528,
            scalar_v1533,
            scalar_v1534,
            scalar_v1535,
            scalar_v1538,
            scalar_v1545,
            scalar_v1551,
            scalar_v1571,
            scalar_v1589,
            scalar_v1595,
            scalar_v1610,
            scalar_v1627,
            scalar_v1633,
            scalar_v1648,
            scalar_v1669,
            scalar_v1687,
            scalar_v1696,
            scalar_v1722,
            scalar_v1723,
            scalar_v1728,
            scalar_v1729,
            scalar_v1759,
            scalar_v1810,
            scalar_v1811,
            scalar_v1875,
            scalar_v1876,
            scalar_v1883,
            scalar_v1884,
            scalar_v1965,
            scalar_v1966,
            scalar_v2049,
            scalar_v2050,
            scalar_v2051,
            scalar_v2052,
            scalar_v2060,
            scalar_v2061,
            scalar_v2062,
            scalar_v2063,
            scalar_v2100,
            scalar_v2104,
            scalar_v2335,
            scalar_v2422,
            scalar_v2423,
            scalar_v2424,
            scalar_v2425,
            scalar_v2523,
            scalar_v2524,
            scalar_v2525,
            scalar_v2526,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v2538,
            scalar_v2917,
            scalar_v2918,
            scalar_v2919,
            scalar_v2920,
            scalar_v3038,
            scalar_v3039,
            scalar_v3040,
            scalar_v3041,
            scalar_v3042,
            scalar_v3043,
            scalar_v3053,
            scalar_v3054,
            scalar_v3055,
            scalar_v3056,
            scalar_v3057,
            scalar_v3058,
            scalar_v3112,
            scalar_v3179,
            scalar_v3180,
            scalar_v3181,
            scalar_v3182,
            scalar_v3183,
            scalar_v3184,
            scalar_v3408,
            scalar_v3424,
            scalar_v3513,
            scalar_v3686,
            scalar_v3715,
            scalar_v4115,
            scalar_v4126,
            scalar_v4246,
            scalar_v4247,
            scalar_v4598,
            scalar_v4621,
            scalar_v4720,
            scalar_v4721,
            scalar_v4722,
            scalar_v4858,
            scalar_v4861,
            scalar_v4862,
            scalar_v5045,
            scalar_v5046,
            scalar_v5047,
            scalar_v5,
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
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_finite_parameter("TNOM", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcx" => { validate_parameter("RCX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rci" => { validate_parameter("RCI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vo" => { validate_parameter("VO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "v0" => { validate_parameter("VO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamm" => { validate_parameter("GAMM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gamma" => { validate_parameter("GAMM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hrcf" => { validate_parameter("HRCF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbx" => { validate_parameter("RBX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbi" => { validate_parameter("RBI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("RE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbp" => { validate_parameter("RBP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("IS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nr" => { validate_parameter("NR", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fc" => { validate_parameter("FC", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("CBEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbe0" => { validate_parameter("CBEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("CJE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("PE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "me" => { validate_parameter("ME", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aje" => { validate_finite_parameter("AJE", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("CBCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbc0" => { validate_parameter("CBCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("CJC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qco" => { validate_parameter("QCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qc0" => { validate_parameter("QCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjep" => { validate_parameter("CJEP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("PC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("MC", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajc" => { validate_finite_parameter("AJC", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjcp" => { validate_parameter("CJCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ms" => { validate_parameter("MS", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ajs" => { validate_finite_parameter("AJS", value)?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibei" => { validate_parameter("IBEI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbe" => { validate_parameter("WBE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nei" => { validate_parameter("NEI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iben" => { validate_parameter("IBEN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nen" => { validate_parameter("NEN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibci" => { validate_parameter("IBCI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nci" => { validate_parameter("NCI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcn" => { validate_parameter("IBCN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncn" => { validate_parameter("NCN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc1" => { validate_parameter("AVC1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avc2" => { validate_parameter("AVC2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isp" => { validate_parameter("ISP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsp" => { validate_parameter("WSP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfp" => { validate_parameter("NFP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibeip" => { validate_parameter("IBEIP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibenp" => { validate_parameter("IBENP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcip" => { validate_parameter("IBCIP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncip" => { validate_parameter("NCIP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibcnp" => { validate_parameter("IBCNP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncnp" => { validate_parameter("NCNP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("VEF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("VER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikf" => { validate_parameter("IKF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikr" => { validate_parameter("IKR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikp" => { validate_parameter("IKP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tf" => { validate_parameter("TF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qtf" => { validate_parameter("QTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xtf" => { validate_parameter("XTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtf" => { validate_parameter("VTF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itf" => { validate_parameter("ITF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "td" => { validate_parameter("TD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("KFN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("AFN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bfn" => { validate_parameter("BFN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xre" => { validate_finite_parameter("XRE", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbi" => { validate_finite_parameter("XRBI", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrci" => { validate_finite_parameter("XRCI", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrs" => { validate_finite_parameter("XRS", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xvo" => { validate_finite_parameter("XVO", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xv0" => { validate_finite_parameter("XVO", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ea" => { validate_finite_parameter("EA", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaie" => { validate_finite_parameter("EAIE", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eaic" => { validate_finite_parameter("EAIC", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eais" => { validate_finite_parameter("EAIS", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eane" => { validate_finite_parameter("EANE", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eanc" => { validate_finite_parameter("EANC", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eans" => { validate_finite_parameter("EANS", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xis" => { validate_finite_parameter("XIS", value)?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xii" => { validate_finite_parameter("XII", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xin" => { validate_finite_parameter("XIN", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnf" => { validate_finite_parameter("TNF", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tavc" => { validate_finite_parameter("TAVC", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("RTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("CTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vrt" => { validate_parameter("VRT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "art" => { validate_parameter("ART", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccso" => { validate_parameter("CCSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccs0" => { validate_parameter("CCSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qbm" => { validate_finite_parameter("QBM", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nkf" => { validate_parameter("NKF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xikf" => { validate_finite_parameter("XIKF", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrcx" => { validate_finite_parameter("XRCX", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbx" => { validate_finite_parameter("XRBX", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrbp" => { validate_finite_parameter("XRBP", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isrr" => { validate_parameter("ISRR", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xisr" => { validate_finite_parameter("XISR", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dear" => { validate_finite_parameter("DEAR", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eap" => { validate_finite_parameter("EAP", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbbe" => { validate_finite_parameter("VBBE", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbbe" => { validate_parameter("NBBE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibbe" => { validate_finite_parameter("IBBE", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe1" => { validate_finite_parameter("TVBBE1", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvbbe2" => { validate_finite_parameter("TVBBE2", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnbbe" => { validate_finite_parameter("TNBBE", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ebbe" => { validate_finite_parameter("EBBE", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtmp" => { validate_finite_parameter("DTEMP", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vers" => { validate_finite_parameter("VERS", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERS", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vrev" => { validate_finite_parameter("VREV", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'vbic_4T_et_cf'", name)),
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
        let v1: f64 = p.p0;
        self.scalar_v1 = v1;
        let v2: f64 = (273.15 + p.p0);
        self.scalar_v2 = v2;
        let v4: f64 = p.p105;
        self.scalar_v4 = v4;
        let v14: f64 = p.p53;
        self.scalar_v14 = v14;
        let v15: f64 = p.p90;
        self.scalar_v15 = v15;
        let v18: f64 = p.p1;
        self.scalar_v18 = v18;
        let v19: f64 = p.p91;
        self.scalar_v19 = v19;
        let v22: f64 = p.p2;
        self.scalar_v22 = v22;
        let v23: f64 = p.p68;
        self.scalar_v23 = v23;
        let v26: f64 = p.p6;
        self.scalar_v26 = v26;
        let v27: f64 = p.p92;
        self.scalar_v27 = v27;
        let v30: f64 = p.p7;
        self.scalar_v30 = v30;
        let v31: f64 = p.p67;
        self.scalar_v31 = v31;
        let v34: f64 = p.p8;
        self.scalar_v34 = v34;
        let v35: f64 = p.p66;
        self.scalar_v35 = v35;
        let v38: f64 = p.p9;
        self.scalar_v38 = v38;
        let v39: f64 = p.p69;
        self.scalar_v39 = v39;
        let v42: f64 = p.p10;
        self.scalar_v42 = v42;
        let v43: f64 = p.p93;
        self.scalar_v43 = v43;
        let v46: f64 = p.p11;
        self.scalar_v46 = v46;
        let v47: f64 = p.p78;
        self.scalar_v47 = v47;
        let v49: f64 = p.p71;
        self.scalar_v49 = v49;
        let v50: f64 = (-p.p71);
        self.scalar_v50 = v50;
        let v57: f64 = p.p12;
        self.scalar_v57 = v57;
        let v58: f64 = (1.0 / p.p12);
        self.scalar_v58 = v58;
        let v61: f64 = p.p94;
        self.scalar_v61 = v61;
        let v62: f64 = p.p95;
        self.scalar_v62 = v62;
        let v64: f64 = p.p96;
        self.scalar_v64 = v64;
        let v65: f64 = (-p.p96);
        self.scalar_v65 = v65;
        let v70: f64 = p.p13;
        self.scalar_v70 = v70;
        let v71: f64 = (1.0 / p.p13);
        self.scalar_v71 = v71;
        let v74: f64 = p.p42;
        self.scalar_v74 = v74;
        let v75: f64 = p.p97;
        self.scalar_v75 = v75;
        let v76: f64 = (-p.p97);
        self.scalar_v76 = v76;
        let v81: f64 = p.p44;
        self.scalar_v81 = v81;
        let v82: f64 = (1.0 / p.p44);
        self.scalar_v82 = v82;
        let v85: f64 = p.p31;
        self.scalar_v85 = v85;
        let v86: f64 = p.p79;
        self.scalar_v86 = v86;
        let v88: f64 = p.p72;
        self.scalar_v88 = v88;
        let v89: f64 = (-p.p72);
        self.scalar_v89 = v89;
        let v94: f64 = p.p33;
        self.scalar_v94 = v94;
        let v95: f64 = (1.0 / p.p33);
        self.scalar_v95 = v95;
        let v98: f64 = p.p34;
        self.scalar_v98 = v98;
        let v99: f64 = p.p80;
        self.scalar_v99 = v99;
        let v101: f64 = p.p75;
        self.scalar_v101 = v101;
        let v102: f64 = (-p.p75);
        self.scalar_v102 = v102;
        let v107: f64 = p.p35;
        self.scalar_v107 = v107;
        let v108: f64 = (1.0 / p.p35);
        self.scalar_v108 = v108;
        let v111: f64 = p.p36;
        self.scalar_v111 = v111;
        let v112: f64 = p.p73;
        self.scalar_v112 = v112;
        let v113: f64 = (-p.p73);
        self.scalar_v113 = v113;
        let v118: f64 = p.p37;
        self.scalar_v118 = v118;
        let v119: f64 = (1.0 / p.p37);
        self.scalar_v119 = v119;
        let v122: f64 = p.p38;
        self.scalar_v122 = v122;
        let v123: f64 = p.p76;
        self.scalar_v123 = v123;
        let v124: f64 = (-p.p76);
        self.scalar_v124 = v124;
        let v129: f64 = p.p39;
        self.scalar_v129 = v129;
        let v130: f64 = (1.0 / p.p39);
        self.scalar_v130 = v130;
        let v133: f64 = p.p45;
        self.scalar_v133 = v133;
        let v135: f64 = p.p46;
        self.scalar_v135 = v135;
        let v137: f64 = p.p47;
        self.scalar_v137 = v137;
        let v138: f64 = p.p74;
        self.scalar_v138 = v138;
        let v139: f64 = (-p.p74);
        self.scalar_v139 = v139;
        let v144: f64 = p.p48;
        self.scalar_v144 = v144;
        let v145: f64 = (1.0 / p.p48);
        self.scalar_v145 = v145;
        let v148: f64 = p.p49;
        self.scalar_v148 = v148;
        let v149: f64 = p.p77;
        self.scalar_v149 = v149;
        let v150: f64 = (-p.p77);
        self.scalar_v150 = v150;
        let v155: f64 = p.p50;
        self.scalar_v155 = v155;
        let v156: f64 = (1.0 / p.p50);
        self.scalar_v156 = v156;
        let v159: f64 = p.p81;
        self.scalar_v159 = v159;
        let v164: f64 = p.p41;
        self.scalar_v164 = v164;
        let v165: f64 = p.p82;
        self.scalar_v165 = v165;
        let v169: f64 = p.p98;
        self.scalar_v169 = v169;
        let v170: f64 = p.p101;
        self.scalar_v170 = v170;
        let v171: f64 = p.p102;
        self.scalar_v171 = v171;
        let v177: f64 = p.p99;
        self.scalar_v177 = v177;
        let v178: f64 = p.p103;
        self.scalar_v178 = v178;
        let v186: f64 = p.p17;
        self.scalar_v186 = v186;
        let v187: f64 = (0.5 * p.p17);
        self.scalar_v187 = v187;
        let v192: f64 = (p.p17 * -0.5);
        self.scalar_v192 = v192;
        let v221: f64 = p.p24;
        self.scalar_v221 = v221;
        let v222: f64 = (0.5 * p.p24);
        self.scalar_v222 = v222;
        let v226: f64 = (-0.5 * p.p24);
        self.scalar_v226 = v226;
        let v248: f64 = p.p28;
        self.scalar_v248 = v248;
        let v249: f64 = (0.5 * p.p28);
        self.scalar_v249 = v249;
        let v253: f64 = (-0.5 * p.p28);
        self.scalar_v253 = v253;
        let v275: f64 = p.p16;
        self.scalar_v275 = v275;
        let v277: f64 = p.p18;
        self.scalar_v277 = v277;
        let v280: f64 = p.p21;
        self.scalar_v280 = v280;
        let v282: f64 = p.p25;
        self.scalar_v282 = v282;
        let v285: f64 = p.p23;
        self.scalar_v285 = v285;
        let v287: f64 = p.p27;
        self.scalar_v287 = v287;
        let v289: f64 = p.p29;
        self.scalar_v289 = v289;
        let v292: f64 = p.p4;
        self.scalar_v292 = v292;
        let v295: f64 = p.p3;
        self.scalar_v295 = v295;
        let v296: f64 = p.p70;
        self.scalar_v296 = v296;
        let v303: f64 = p.p51;
        self.scalar_v303 = v303;
        let v305: bool = (p.p51 > 0.0);
        self.scalar_v305 = v305;
        let v306: f64 = (1.0 / p.p51);
        self.scalar_v306 = v306;
        let v307: f64 = (if v305 { v306 } else { 0.0 });
        self.scalar_v307 = v307;
        let v308: f64 = p.p52;
        self.scalar_v308 = v308;
        let v309: bool = (p.p52 > 0.0);
        self.scalar_v309 = v309;
        let v310: f64 = (1.0 / p.p52);
        self.scalar_v310 = v310;
        let v311: f64 = (if v309 { v310 } else { 0.0 });
        self.scalar_v311 = v311;
        let v312: bool = (p.p53 > 0.0);
        self.scalar_v312 = v312;
        let v315: f64 = p.p54;
        self.scalar_v315 = v315;
        let v316: bool = (p.p54 > 0.0);
        self.scalar_v316 = v316;
        let v317: f64 = (1.0 / p.p54);
        self.scalar_v317 = v317;
        let v318: f64 = (if v316 { v317 } else { 0.0 });
        self.scalar_v318 = v318;
        let v319: f64 = p.p55;
        self.scalar_v319 = v319;
        let v320: bool = (p.p55 > 0.0);
        self.scalar_v320 = v320;
        let v321: f64 = (1.0 / p.p55);
        self.scalar_v321 = v321;
        let v322: f64 = (if v320 { v321 } else { 0.0 });
        self.scalar_v322 = v322;
        let v323: bool = (p.p3 > 0.0);
        self.scalar_v323 = v323;
        let v326: f64 = p.p5;
        self.scalar_v326 = v326;
        let v327: bool = (p.p5 > 0.0);
        self.scalar_v327 = v327;
        let v328: f64 = (1.0 / p.p5);
        self.scalar_v328 = v328;
        let v329: f64 = (if v327 { v328 } else { 0.0 });
        self.scalar_v329 = v329;
        let v330: f64 = p.p59;
        self.scalar_v330 = v330;
        let v331: bool = (p.p59 > 0.0);
        self.scalar_v331 = v331;
        let v332: f64 = (1.0 / p.p59);
        self.scalar_v332 = v332;
        let v333: f64 = (if v331 { v332 } else { 0.0 });
        self.scalar_v333 = v333;
        let v334: f64 = p.p60;
        self.scalar_v334 = v334;
        let v335: bool = (p.p60 > 0.0);
        self.scalar_v335 = v335;
        let v336: f64 = (1.0 / p.p60);
        self.scalar_v336 = v336;
        let v337: f64 = (if v335 { v336 } else { 0.0 });
        self.scalar_v337 = v337;
        let v338: f64 = (if v335 { 0.0 } else { 1.0 });
        self.scalar_v338 = v338;
        let v351: f64 = p.p14;
        self.scalar_v351 = v351;
        let v353: f64 = p.p19;
        self.scalar_v353 = v353;
        let v354: bool = (p.p19 <= 0.0);
        self.scalar_v354 = v354;
        let v359: f64 = (1.0 - p.p14);
        self.scalar_v359 = v359;
        let v361: f64 = (-1.0 - p.p18);
        self.scalar_v361 = v361;
        let v362: f64 = f64::powf(v359, v361);
        self.scalar_v362 = v362;
        let v368: f64 = (1.0 - p.p18);
        self.scalar_v368 = v368;
        let v371: f64 = (0.5 * p.p18);
        self.scalar_v371 = v371;
        let v390: bool = (!v354);
        self.scalar_v390 = v390;
        let v392: f64 = (4.0 * p.p19);
        self.scalar_v392 = v392;
        let v393: f64 = (p.p19 * v392);
        self.scalar_v393 = v393;
        let v421: f64 = (-p.p18);
        self.scalar_v421 = v421;
        let v422: f64 = f64::powf(v359, v421);
        self.scalar_v422 = v422;
        let v491: f64 = p.p26;
        self.scalar_v491 = v491;
        let v492: bool = (p.p26 <= 0.0);
        self.scalar_v492 = v492;
        let v497: f64 = (-1.0 - p.p25);
        self.scalar_v497 = v497;
        let v498: f64 = f64::powf(v359, v497);
        self.scalar_v498 = v498;
        let v504: f64 = (1.0 - p.p25);
        self.scalar_v504 = v504;
        let v507: f64 = (0.5 * p.p25);
        self.scalar_v507 = v507;
        let v514: f64 = p.p85;
        self.scalar_v514 = v514;
        let v515: bool = (p.p85 > 0.0);
        self.scalar_v515 = v515;
        let v516: f64 = (-p.p85);
        self.scalar_v516 = v516;
        let v547: f64 = p.p86;
        self.scalar_v547 = v547;
        let v548: bool = (p.p86 > 0.0);
        self.scalar_v548 = v548;
        let v549: bool = (v515 && v548);
        self.scalar_v549 = v549;
        let v550: bool = (!v492);
        self.scalar_v550 = v550;
        let v551: bool = (v549 && v550);
        self.scalar_v551 = v551;
        let v559: f64 = (4.0 * p.p26);
        self.scalar_v559 = v559;
        let v560: f64 = (p.p26 * v559);
        self.scalar_v560 = v560;
        let v565: f64 = (4.0 * p.p86);
        self.scalar_v565 = v565;
        let v566: f64 = (p.p86 * v565);
        self.scalar_v566 = v566;
        let v616: f64 = (-p.p25);
        self.scalar_v616 = v616;
        let v635: bool = (!v549);
        self.scalar_v635 = v635;
        let v636: bool = (v550 && v635);
        self.scalar_v636 = v636;
        let v665: f64 = f64::powf(v359, v616);
        self.scalar_v665 = v665;
        let v817: bool = (p.p27 > 0.0);
        self.scalar_v817 = v817;
        let v821: f64 = p.p30;
        self.scalar_v821 = v821;
        let v822: bool = (p.p30 <= 0.0);
        self.scalar_v822 = v822;
        let v823: bool = (v817 && v822);
        self.scalar_v823 = v823;
        let v830: f64 = (-1.0 - p.p29);
        self.scalar_v830 = v830;
        let v831: f64 = f64::powf(v359, v830);
        self.scalar_v831 = v831;
        let v837: f64 = (1.0 - p.p29);
        self.scalar_v837 = v837;
        let v840: f64 = (0.5 * p.p29);
        self.scalar_v840 = v840;
        let v859: bool = (!v822);
        self.scalar_v859 = v859;
        let v860: bool = (v817 && v859);
        self.scalar_v860 = v860;
        let v862: f64 = (4.0 * p.p30);
        self.scalar_v862 = v862;
        let v863: f64 = (p.p30 * v862);
        self.scalar_v863 = v863;
        let v891: f64 = (-p.p29);
        self.scalar_v891 = v891;
        let v892: f64 = f64::powf(v359, v891);
        self.scalar_v892 = v892;
        let v899: bool = (!v817);
        self.scalar_v899 = v899;
        let v929: f64 = p.p88;
        self.scalar_v929 = v929;
        let v930: bool = (p.p88 < 0.5);
        self.scalar_v930 = v930;
        let v931: f64 = p.p89;
        self.scalar_v931 = v931;
        let v932: f64 = (1.0 / p.p89);
        self.scalar_v932 = v932;
        let v940: bool = (!v930);
        self.scalar_v940 = v940;
        let v949: bool = (p.p42 > 0.0);
        self.scalar_v949 = v949;
        let v959: f64 = p.p43;
        self.scalar_v959 = v959;
        let v961: f64 = (1.0 - p.p43);
        self.scalar_v961 = v961;
        let v985: bool = (!v949);
        self.scalar_v985 = v985;
        let v989: f64 = p.p32;
        self.scalar_v989 = v989;
        let v990: bool = (1.0 == p.p32);
        self.scalar_v990 = v990;
        let v1001: bool = (p.p98 > 0.0);
        self.scalar_v1001 = v1001;
        let v1002: bool = (v990 && v1001);
        self.scalar_v1002 = v1002;
        let v1013: f64 = p.p100;
        self.scalar_v1013 = v1013;
        let v1018: bool = (!v1001);
        self.scalar_v1018 = v1018;
        let v1019: bool = (v990 && v1018);
        self.scalar_v1019 = v1019;
        let v1021: bool = (0.0 == p.p32);
        self.scalar_v1021 = v1021;
        let v1022: bool = (!v990);
        self.scalar_v1022 = v1022;
        let v1023: bool = (v1021 && v1022);
        self.scalar_v1023 = v1023;
        let v1033: bool = (v1001 && v1023);
        self.scalar_v1033 = v1033;
        let v1048: bool = (v1018 && v1023);
        self.scalar_v1048 = v1048;
        let v1050: bool = (!v1021);
        self.scalar_v1050 = v1050;
        let v1051: bool = (v1022 && v1050);
        self.scalar_v1051 = v1051;
        let v1058: bool = (v1001 && v1051);
        self.scalar_v1058 = v1058;
        let v1072: bool = (v1018 && v1051);
        self.scalar_v1072 = v1072;
        let v1084: f64 = (1.0 - p.p32);
        self.scalar_v1084 = v1084;
        let v1108: bool = (p.p45 > 0.0);
        self.scalar_v1108 = v1108;
        let v1109: bool = (p.p46 > 0.0);
        self.scalar_v1109 = v1109;
        let v1110: bool = (v1108 || v1109);
        self.scalar_v1110 = v1110;
        let v1125: bool = (!v1110);
        self.scalar_v1125 = v1125;
        let v1127: f64 = p.p40;
        self.scalar_v1127 = v1127;
        let v1128: bool = (p.p40 > 0.0);
        self.scalar_v1128 = v1128;
        let v1139: f64 = (p.p25 - 1.0);
        self.scalar_v1139 = v1139;
        let v1149: bool = (!v1128);
        self.scalar_v1149 = v1149;
        let v1152: bool = (p.p1 > 0.0);
        self.scalar_v1152 = v1152;
        let v1157: bool = (!v1152);
        self.scalar_v1157 = v1157;
        let v1169: bool = (p.p2 > 0.0);
        self.scalar_v1169 = v1169;
        let v1198: bool = (!v1169);
        self.scalar_v1198 = v1198;
        let v1200: bool = (p.p6 > 0.0);
        self.scalar_v1200 = v1200;
        let v1205: bool = (!v1200);
        self.scalar_v1205 = v1205;
        let v1207: bool = (p.p7 > 0.0);
        self.scalar_v1207 = v1207;
        let v1212: bool = (!v1207);
        self.scalar_v1212 = v1212;
        let v1214: bool = (p.p8 > 0.0);
        self.scalar_v1214 = v1214;
        let v1219: bool = (!v1214);
        self.scalar_v1219 = v1219;
        let v1221: bool = (p.p10 > 0.0);
        self.scalar_v1221 = v1221;
        let v1226: bool = (!v1221);
        self.scalar_v1226 = v1226;
        let v1228: bool = (p.p47 > 0.0);
        self.scalar_v1228 = v1228;
        let v1229: bool = (p.p49 > 0.0);
        self.scalar_v1229 = v1229;
        let v1230: bool = (v1228 || v1229);
        self.scalar_v1230 = v1230;
        let v1247: bool = (!v1230);
        self.scalar_v1247 = v1247;
        let v1249: bool = (p.p9 > 0.0);
        self.scalar_v1249 = v1249;
        let v1254: bool = (!v1249);
        self.scalar_v1254 = v1254;
        let v1262: f64 = p.p56;
        self.scalar_v1262 = v1262;
        let v1263: f64 = p.p57;
        self.scalar_v1263 = v1263;
        let v1267: f64 = p.p58;
        self.scalar_v1267 = v1267;
        let v1287: f64 = p.p61;
        self.scalar_v1287 = v1287;
        let v1290: f64 = p.p22;
        self.scalar_v1290 = v1290;
        let v1298: f64 = p.p87;
        self.scalar_v1298 = v1298;
        let v1302: f64 = p.p15;
        self.scalar_v1302 = v1302;
        let v1305: f64 = p.p20;
        self.scalar_v1305 = v1305;
        let v1337: f64 = p.p83;
        self.scalar_v1337 = v1337;
        let v1338: bool = (p.p83 > 0.0);
        self.scalar_v1338 = v1338;
        let v1341: bool = (!v1338);
        self.scalar_v1341 = v1341;
        let v1343: f64 = p.p84;
        self.scalar_v1343 = v1343;
        let v1346: f64 = (1.0 / v2);
        self.scalar_v1346 = v1346;
        let v1347: f64 = (p.p90 - 1.0);
        self.scalar_v1347 = v1347;
        let v1352: f64 = (p.p91 - 1.0);
        self.scalar_v1352 = v1352;
        let v1357: f64 = (p.p68 - 1.0);
        self.scalar_v1357 = v1357;
        let v1362: f64 = (p.p92 - 1.0);
        self.scalar_v1362 = v1362;
        let v1367: f64 = (p.p67 - 1.0);
        self.scalar_v1367 = v1367;
        let v1372: f64 = (p.p66 - 1.0);
        self.scalar_v1372 = v1372;
        let v1377: f64 = (p.p69 - 1.0);
        self.scalar_v1377 = v1377;
        let v1382: f64 = (p.p93 - 1.0);
        self.scalar_v1382 = v1382;
        let v1387: f64 = (p.p78 - 1.0);
        self.scalar_v1387 = v1387;
        let v1391: f64 = (-v1346);
        self.scalar_v1391 = v1391;
        let v1392: f64 = (v50 * v1391);
        self.scalar_v1392 = v1392;
        let v1402: f64 = (v58 - 1.0);
        self.scalar_v1402 = v1402;
        let v1407: f64 = (p.p95 - 1.0);
        self.scalar_v1407 = v1407;
        let v1411: f64 = (v65 * v1391);
        self.scalar_v1411 = v1411;
        let v1420: f64 = (v71 - 1.0);
        self.scalar_v1420 = v1420;
        let v1425: f64 = (v76 * v1391);
        self.scalar_v1425 = v1425;
        let v1434: f64 = (v82 - 1.0);
        self.scalar_v1434 = v1434;
        let v1439: f64 = (p.p79 - 1.0);
        self.scalar_v1439 = v1439;
        let v1443: f64 = (v89 * v1391);
        self.scalar_v1443 = v1443;
        let v1452: f64 = (v95 - 1.0);
        self.scalar_v1452 = v1452;
        let v1457: f64 = (p.p80 - 1.0);
        self.scalar_v1457 = v1457;
        let v1461: f64 = (v102 * v1391);
        self.scalar_v1461 = v1461;
        let v1470: f64 = (v108 - 1.0);
        self.scalar_v1470 = v1470;
        let v1475: f64 = (v113 * v1391);
        self.scalar_v1475 = v1475;
        let v1484: f64 = (v119 - 1.0);
        self.scalar_v1484 = v1484;
        let v1489: f64 = (v124 * v1391);
        self.scalar_v1489 = v1489;
        let v1498: f64 = (v130 - 1.0);
        self.scalar_v1498 = v1498;
        let v1505: f64 = (v139 * v1391);
        self.scalar_v1505 = v1505;
        let v1514: f64 = (v145 - 1.0);
        self.scalar_v1514 = v1514;
        let v1519: f64 = (v150 * v1391);
        self.scalar_v1519 = v1519;
        let v1528: f64 = (v156 - 1.0);
        self.scalar_v1528 = v1528;
        let v1533: f64 = (p.p12 * p.p81);
        self.scalar_v1533 = v1533;
        let v1534: f64 = (p.p13 * p.p81);
        self.scalar_v1534 = v1534;
        let v1535: f64 = (p.p41 * p.p82);
        self.scalar_v1535 = v1535;
        let v1538: f64 = (p.p99 * p.p103);
        self.scalar_v1538 = v1538;
        let v1545: f64 = (v187 * v1346);
        self.scalar_v1545 = v1545;
        let v1551: f64 = (v192 * v1346);
        self.scalar_v1551 = v1551;
        let v1571: f64 = (p.p72 * v1346);
        self.scalar_v1571 = v1571;
        let v1589: f64 = (v222 * v1346);
        self.scalar_v1589 = v1589;
        let v1595: f64 = (v226 * v1346);
        self.scalar_v1595 = v1595;
        let v1610: f64 = (p.p73 * v1346);
        self.scalar_v1610 = v1610;
        let v1627: f64 = (v249 * v1346);
        self.scalar_v1627 = v1627;
        let v1633: f64 = (v253 * v1346);
        self.scalar_v1633 = v1633;
        let v1648: f64 = (p.p74 * v1346);
        self.scalar_v1648 = v1648;
        let v1669: f64 = (p.p18 - 1.0);
        self.scalar_v1669 = v1669;
        let v1687: f64 = (p.p29 - 1.0);
        self.scalar_v1687 = v1687;
        let v1696: f64 = (p.p70 - 1.0);
        self.scalar_v1696 = v1696;
        let v1722: f64 = (if v354 { 1.0 } else { 0.0 });
        self.scalar_v1722 = v1722;
        let v1723: f64 = (if v354 { -1.0 } else { 0.0 });
        self.scalar_v1723 = v1723;
        let v1728: f64 = (v371 * v1722);
        self.scalar_v1728 = v1728;
        let v1729: f64 = (v371 * v1723);
        self.scalar_v1729 = v1729;
        let v1759: f64 = (v368 - 1.0);
        self.scalar_v1759 = v1759;
        let v1810: f64 = (if v390 { 1.0 } else { 0.0 });
        self.scalar_v1810 = v1810;
        let v1811: f64 = (if v390 { -1.0 } else { 0.0 });
        self.scalar_v1811 = v1811;
        let v1875: f64 = (if v354 { 0.0 } else { v1722 });
        self.scalar_v1875 = v1875;
        let v1876: f64 = (if v354 { -1.0 } else { v1723 });
        self.scalar_v1876 = v1876;
        let v1883: f64 = (v371 * v1875);
        self.scalar_v1883 = v1883;
        let v1884: f64 = (v371 * v1876);
        self.scalar_v1884 = v1884;
        let v1965: f64 = (if v390 { 0.0 } else { v1810 });
        self.scalar_v1965 = v1965;
        let v1966: f64 = (if v390 { -1.0 } else { v1811 });
        self.scalar_v1966 = v1966;
        let v2049: f64 = (if v492 { -1.0 } else { 0.0 });
        self.scalar_v2049 = v2049;
        let v2050: f64 = (if v492 { 0.0 } else { v1722 });
        self.scalar_v2050 = v2050;
        let v2051: f64 = (if v492 { 1.0 } else { v1875 });
        self.scalar_v2051 = v2051;
        let v2052: f64 = (if v492 { 0.0 } else { v1876 });
        self.scalar_v2052 = v2052;
        let v2060: f64 = (v507 * v2049);
        self.scalar_v2060 = v2060;
        let v2061: f64 = (v507 * v2050);
        self.scalar_v2061 = v2061;
        let v2062: f64 = (v507 * v2051);
        self.scalar_v2062 = v2062;
        let v2063: f64 = (v507 * v2052);
        self.scalar_v2063 = v2063;
        let v2100: f64 = (v504 - 1.0);
        self.scalar_v2100 = v2100;
        let v2104: f64 = (-v504);
        self.scalar_v2104 = v2104;
        let v2335: f64 = (v616 - 1.0);
        self.scalar_v2335 = v2335;
        let v2422: f64 = (if v636 { -1.0 } else { 0.0 });
        self.scalar_v2422 = v2422;
        let v2423: f64 = (if v636 { 0.0 } else { v1810 });
        self.scalar_v2423 = v2423;
        let v2424: f64 = (if v636 { 1.0 } else { v1965 });
        self.scalar_v2424 = v2424;
        let v2425: f64 = (if v636 { 0.0 } else { v1966 });
        self.scalar_v2425 = v2425;
        let v2523: f64 = (if v492 { 0.0 } else { v2049 });
        self.scalar_v2523 = v2523;
        let v2524: f64 = (if v492 { 1.0 } else { v2050 });
        self.scalar_v2524 = v2524;
        let v2525: f64 = (if v492 { 0.0 } else { v2051 });
        self.scalar_v2525 = v2525;
        let v2526: f64 = (if v492 { 0.0 } else { v2052 });
        self.scalar_v2526 = v2526;
        let v2535: f64 = (v507 * v2523);
        self.scalar_v2535 = v2535;
        let v2536: f64 = (v507 * v2524);
        self.scalar_v2536 = v2536;
        let v2537: f64 = (v507 * v2525);
        self.scalar_v2537 = v2537;
        let v2538: f64 = (v507 * v2526);
        self.scalar_v2538 = v2538;
        let v2917: f64 = (if v636 { 0.0 } else { v2422 });
        self.scalar_v2917 = v2917;
        let v2918: f64 = (if v636 { 1.0 } else { v2423 });
        self.scalar_v2918 = v2918;
        let v2919: f64 = (if v636 { 0.0 } else { v2424 });
        self.scalar_v2919 = v2919;
        let v2920: f64 = (if v636 { 0.0 } else { v2425 });
        self.scalar_v2920 = v2920;
        let v3038: f64 = (if v823 { 0.0 } else { v2523 });
        self.scalar_v3038 = v3038;
        let v3039: f64 = (if v823 { 0.0 } else { v2524 });
        self.scalar_v3039 = v3039;
        let v3040: f64 = (if v823 { 0.0 } else { v2525 });
        self.scalar_v3040 = v3040;
        let v3041: f64 = (if v823 { 0.0 } else { v2526 });
        self.scalar_v3041 = v3041;
        let v3042: f64 = (if v823 { -1.0 } else { v2049 });
        self.scalar_v3042 = v3042;
        let v3043: f64 = (if v823 { 1.0 } else { 0.0 });
        self.scalar_v3043 = v3043;
        let v3053: f64 = (v840 * v3038);
        self.scalar_v3053 = v3053;
        let v3054: f64 = (v840 * v3039);
        self.scalar_v3054 = v3054;
        let v3055: f64 = (v840 * v3040);
        self.scalar_v3055 = v3055;
        let v3056: f64 = (v840 * v3041);
        self.scalar_v3056 = v3056;
        let v3057: f64 = (v840 * v3042);
        self.scalar_v3057 = v3057;
        let v3058: f64 = (v840 * v3043);
        self.scalar_v3058 = v3058;
        let v3112: f64 = (v837 - 1.0);
        self.scalar_v3112 = v3112;
        let v3179: f64 = (if v860 { 0.0 } else { v2917 });
        self.scalar_v3179 = v3179;
        let v3180: f64 = (if v860 { 0.0 } else { v2918 });
        self.scalar_v3180 = v3180;
        let v3181: f64 = (if v860 { 0.0 } else { v2919 });
        self.scalar_v3181 = v3181;
        let v3182: f64 = (if v860 { 0.0 } else { v2920 });
        self.scalar_v3182 = v3182;
        let v3183: f64 = (if v860 { -1.0 } else { v2422 });
        self.scalar_v3183 = v3183;
        let v3184: f64 = (if v860 { 1.0 } else { 0.0 });
        self.scalar_v3184 = v3184;
        let v3408: f64 = (v932 - 1.0);
        self.scalar_v3408 = v3408;
        let v3424: f64 = (p.p89 - 1.0);
        self.scalar_v3424 = v3424;
        let v3513: f64 = (p.p44 * 8.617342301212761e-5);
        self.scalar_v3513 = v3513;
        let v3686: f64 = (p.p33 * 8.617342301212761e-5);
        self.scalar_v3686 = v3686;
        let v3715: f64 = (p.p35 * 8.617342301212761e-5);
        self.scalar_v3715 = v3715;
        let v4115: f64 = (p.p37 * 8.617342301212761e-5);
        self.scalar_v4115 = v4115;
        let v4126: f64 = (p.p39 * 8.617342301212761e-5);
        self.scalar_v4126 = v4126;
        let v4246: f64 = (-v1535);
        self.scalar_v4246 = v4246;
        let v4247: f64 = (v1139 - 1.0);
        self.scalar_v4247 = v4247;
        let v4598: f64 = (p.p48 * 8.617342301212761e-5);
        self.scalar_v4598 = v4598;
        let v4621: f64 = (p.p50 * 8.617342301212761e-5);
        self.scalar_v4621 = v4621;
        let v4720: f64 = (-v333);
        self.scalar_v4720 = v4720;
        let v4721: f64 = (v4720 / 1.44);
        self.scalar_v4721 = v4721;
        let v4722: f64 = (v333 / 1.44);
        self.scalar_v4722 = v4722;
        let v4858: f64 = (-p.p87);
        self.scalar_v4858 = v4858;
        let v4861: f64 = (-p.p15);
        self.scalar_v4861 = v4861;
        let v4862: f64 = (-p.p20);
        self.scalar_v4862 = v4862;
        let v5045: f64 = (1.0 / p.p83);
        self.scalar_v5045 = v5045;
        let v5046: f64 = (if v1338 { v5045 } else { 0.0 });
        self.scalar_v5046 = v5046;
        let v5047: f64 = (if v1341 { 0.0 } else { v5046 });
        self.scalar_v5047 = v5047;
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
        let v5: f64 = (temperature + self.scalar_v4);
        self.scalar_v5 = v5;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
