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
    pub p133: f64,
    pub p134: f64,
    pub p135: f64,
    pub p136: f64,
    pub p137: f64,
    pub p138: f64,
    pub p139: f64,
    pub p140: f64,
    pub p141: f64,
    pub p142: f64,
    pub p143: f64,
    pub p144: f64,
    pub p145: f64,
    pub p146: f64,
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
            params.p2 = 505.5;
            params.p3 = 1.0;
            params.p4 = 25.0;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 2.2e-17;
            params.p9 = 1.0;
            params.p10 = 1.0;
            params.p11 = 0.1;
            params.p12 = 2.5;
            params.p13 = 44.0;
            params.p14 = 1.0;
            params.p15 = 1.0000000000000001e-19;
            params.p16 = 1.0;
            params.p17 = 0.0;
            params.p18 = 1.0;
            params.p19 = 2.7000000000000005e-15;
            params.p20 = 2.0;
            params.p21 = 0.0;
            params.p22 = 2.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.68;
            params.p27 = 0.0;
            params.p28 = 3.1400000000000002e-18;
            params.p29 = 0.014289999999999999;
            params.p30 = 1e-15;
            params.p31 = 2.0;
            params.p32 = 0.63;
            params.p33 = 0.0;
            params.p34 = 22.0;
            params.p35 = 0.0;
            params.p36 = 22.0;
            params.p37 = 1e-6;
            params.p38 = 1.0;
            params.p39 = 400.0;
            params.p40 = -0.37;
            params.p41 = 0.5;
            params.p42 = 25.0;
            params.p43 = 0.1;
            params.p44 = 1.1e-6;
            params.p45 = 3.0;
            params.p46 = 0.3;
            params.p47 = 0.004;
            params.p48 = -0.37;
            params.p49 = -0.37;
            params.p50 = 0.3;
            params.p51 = 0.004;
            params.p52 = 1.0;
            params.p53 = 5.0;
            params.p54 = 23.0;
            params.p55 = 18.0;
            params.p56 = 12.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 150.0;
            params.p60 = 1250.0;
            params.p61 = 0.004;
            params.p62 = 0.3;
            params.p63 = 0.68;
            params.p64 = 7.3e-14;
            params.p65 = 0.95;
            params.p66 = 0.4;
            params.p67 = 0.4;
            params.p68 = 0.0;
            params.p69 = 7.800000000000001e-14;
            params.p70 = 0.68;
            params.p71 = 0.5;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.35;
            params.p75 = 0.5;
            params.p76 = 0.032;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.68;
            params.p80 = 100.0;
            params.p81 = 4.0;
            params.p82 = 1000.0;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 2e-12;
            params.p86 = 4.2e-12;
            params.p87 = 4.1e-11;
            params.p88 = 5.2e-10;
            params.p89 = 1e-11;
            params.p90 = 1.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.3333333333333333;
            params.p94 = 0.0;
            params.p95 = 0.3;
            params.p96 = 0.0;
            params.p97 = 1.0;
            params.p98 = 2.5;
            params.p99 = 2.5;
            params.p100 = 0.62;
            params.p101 = 2.0;
            params.p102 = 1.3;
            params.p103 = 2.0;
            params.p104 = 1.17;
            params.p105 = 1.12;
            params.p106 = 1.12;
            params.p107 = 1.12;
            params.p108 = 1.12;
            params.p109 = 1.18;
            params.p110 = 1.12;
            params.p111 = 1.125;
            params.p112 = 1.15;
            params.p113 = 1.15;
            params.p114 = 0.000473;
            params.p115 = 636.0;
            params.p116 = 1.15;
            params.p117 = 0.000473;
            params.p118 = 636.0;
            params.p119 = 0.05;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0005;
            params.p124 = 200.0;
            params.p125 = 2.0;
            params.p126 = 2.0;
            params.p127 = 2e-11;
            params.p128 = 2e-11;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 300.0;
            params.p134 = 3.0000000000000004e-9;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 2.0;
            params.p138 = 400.0;
            params.p139 = 1e-40;
            params.p140 = 1e-40;
            params.p141 = 0.001;
            validate_parameter("minr", params.p141, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p142 = 0.0;
            params.p143 = 1.0;
            params.p144 = 0.0;
            params.p145 = 0.16;
            params.p146 = 0.0;
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
    pub branches: [usize; 2],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 147]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 10]>,
    pub(crate) ddt_state_previous: Box<[f64; 10]>,
    pub(crate) ddt_state_older: Box<[f64; 10]>,
    pub(crate) ddt_state_initialized: Box<[bool; 10]>,
    pub(crate) ddt_derivative_current: Box<[f64; 10]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 10]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v2: bool,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: bool,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: bool,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: bool,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v32: f64,
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
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: bool,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: bool,
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: bool,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: bool,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v197: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v277: f64,
    pub(crate) scalar_v278: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: bool,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v345: bool,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: bool,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v376: bool,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: f64,
    pub(crate) scalar_v397: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v437: f64,
    pub(crate) scalar_v438: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v450: f64,
    pub(crate) scalar_v451: f64,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v457: f64,
    pub(crate) scalar_v458: f64,
    pub(crate) scalar_v463: f64,
    pub(crate) scalar_v464: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: bool,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v474: f64,
    pub(crate) scalar_v475: f64,
    pub(crate) scalar_v481: f64,
    pub(crate) scalar_v482: f64,
    pub(crate) scalar_v483: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v496: f64,
    pub(crate) scalar_v497: f64,
    pub(crate) scalar_v498: f64,
    pub(crate) scalar_v502: f64,
    pub(crate) scalar_v503: f64,
    pub(crate) scalar_v507: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v517: f64,
    pub(crate) scalar_v518: f64,
    pub(crate) scalar_v519: f64,
    pub(crate) scalar_v526: f64,
    pub(crate) scalar_v529: f64,
    pub(crate) scalar_v537: f64,
    pub(crate) scalar_v546: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v568: f64,
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v583: f64,
    pub(crate) scalar_v601: f64,
    pub(crate) scalar_v603: f64,
    pub(crate) scalar_v605: f64,
    pub(crate) scalar_v607: f64,
    pub(crate) scalar_v610: bool,
    pub(crate) scalar_v616: bool,
    pub(crate) scalar_v618: bool,
    pub(crate) scalar_v624: bool,
    pub(crate) scalar_v626: bool,
    pub(crate) scalar_v632: bool,
    pub(crate) scalar_v675: f64,
    pub(crate) scalar_v680: f64,
    pub(crate) scalar_v768: f64,
    pub(crate) scalar_v821: f64,
    pub(crate) scalar_v822: f64,
    pub(crate) scalar_v823: f64,
    pub(crate) scalar_v834: f64,
    pub(crate) scalar_v855: f64,
    pub(crate) scalar_v856: f64,
    pub(crate) scalar_v857: f64,
    pub(crate) scalar_v858: f64,
    pub(crate) scalar_v859: f64,
    pub(crate) scalar_v860: f64,
    pub(crate) scalar_v907: f64,
    pub(crate) scalar_v917: f64,
    pub(crate) scalar_v930: f64,
    pub(crate) scalar_v931: bool,
    pub(crate) scalar_v935: bool,
    pub(crate) scalar_v984: f64,
    pub(crate) scalar_v985: f64,
    pub(crate) scalar_v986: f64,
    pub(crate) scalar_v1008: f64,
    pub(crate) scalar_v1016: f64,
    pub(crate) scalar_v1017: bool,
    pub(crate) scalar_v1019: bool,
    pub(crate) scalar_v1020: bool,
    pub(crate) scalar_v1021: bool,
    pub(crate) scalar_v1024: bool,
    pub(crate) scalar_v1025: bool,
    pub(crate) scalar_v1030: f64,
    pub(crate) scalar_v1051: f64,
    pub(crate) scalar_v1053: f64,
    pub(crate) scalar_v1082: bool,
    pub(crate) scalar_v1088: bool,
    pub(crate) scalar_v1122: f64,
    pub(crate) scalar_v1144: f64,
    pub(crate) scalar_v1157: f64,
    pub(crate) scalar_v1175: f64,
    pub(crate) scalar_v1238: f64,
    pub(crate) scalar_v1239: bool,
    pub(crate) scalar_v1240: bool,
    pub(crate) scalar_v1241: bool,
    pub(crate) scalar_v1243: bool,
    pub(crate) scalar_v1244: bool,
    pub(crate) scalar_v1245: f64,
    pub(crate) scalar_v1338: bool,
    pub(crate) scalar_v1339: bool,
    pub(crate) scalar_v1340: bool,
    pub(crate) scalar_v1364: f64,
    pub(crate) scalar_v1366: f64,
    pub(crate) scalar_v1367: f64,
    pub(crate) scalar_v1369: f64,
    pub(crate) scalar_v1429: bool,
    pub(crate) scalar_v1430: bool,
    pub(crate) scalar_v1431: bool,
    pub(crate) scalar_v1457: f64,
    pub(crate) scalar_v1459: f64,
    pub(crate) scalar_v1460: f64,
    pub(crate) scalar_v1462: f64,
    pub(crate) scalar_v1528: f64,
    pub(crate) scalar_v1529: bool,
    pub(crate) scalar_v1530: bool,
    pub(crate) scalar_v1531: bool,
    pub(crate) scalar_v1534: f64,
    pub(crate) scalar_v1544: f64,
    pub(crate) scalar_v1545: bool,
    pub(crate) scalar_v1546: bool,
    pub(crate) scalar_v1558: f64,
    pub(crate) scalar_v1563: f64,
    pub(crate) scalar_v1580: bool,
    pub(crate) scalar_v1581: bool,
    pub(crate) scalar_v1585: f64,
    pub(crate) scalar_v1586: bool,
    pub(crate) scalar_v1589: f64,
    pub(crate) scalar_v1595: f64,
    pub(crate) scalar_v1606: f64,
    pub(crate) scalar_v1607: f64,
    pub(crate) scalar_v1608: f64,
    pub(crate) scalar_v1609: f64,
    pub(crate) scalar_v1610: f64,
    pub(crate) scalar_v1611: f64,
    pub(crate) scalar_v1612: f64,
    pub(crate) scalar_v1613: f64,
    pub(crate) scalar_v1614: f64,
    pub(crate) scalar_v1615: f64,
    pub(crate) scalar_v1616: f64,
    pub(crate) scalar_v1617: f64,
    pub(crate) scalar_v1618: f64,
    pub(crate) scalar_v1619: f64,
    pub(crate) scalar_v1620: f64,
    pub(crate) scalar_v1634: bool,
    pub(crate) scalar_v1661: f64,
    pub(crate) scalar_v1662: bool,
    pub(crate) scalar_v1663: f64,
    pub(crate) scalar_v1666: f64,
    pub(crate) scalar_v1685: f64,
    pub(crate) scalar_v1699: f64,
    pub(crate) scalar_v1704: bool,
    pub(crate) scalar_v1706: bool,
    pub(crate) scalar_v1710: f64,
    pub(crate) scalar_v1711: f64,
    pub(crate) scalar_v1712: f64,
    pub(crate) scalar_v1713: f64,
    pub(crate) scalar_v1714: f64,
    pub(crate) scalar_v1723: f64,
    pub(crate) scalar_v1724: bool,
    pub(crate) scalar_v1727: bool,
    pub(crate) scalar_v1750: f64,
    pub(crate) scalar_v1751: f64,
    pub(crate) scalar_v1757: f64,
    pub(crate) scalar_v1758: f64,
    pub(crate) scalar_v1759: f64,
    pub(crate) scalar_v1807: bool,
    pub(crate) scalar_v1808: bool,
    pub(crate) scalar_v1813: f64,
    pub(crate) scalar_v1817: f64,
    pub(crate) scalar_v1824: f64,
    pub(crate) scalar_v1829: f64,
    pub(crate) scalar_v1849: f64,
    pub(crate) scalar_v1869: f64,
    pub(crate) scalar_v1870: bool,
    pub(crate) scalar_v1905: bool,
    pub(crate) scalar_v1911: bool,
    pub(crate) scalar_v1967: f64,
    pub(crate) scalar_v1968: bool,
    pub(crate) scalar_v1969: f64,
    pub(crate) scalar_v1970: bool,
    pub(crate) scalar_v1971: bool,
    pub(crate) scalar_v1975: f64,
    pub(crate) scalar_v1976: bool,
    pub(crate) scalar_v1977: bool,
    pub(crate) scalar_v1978: bool,
    pub(crate) scalar_v1979: bool,
    pub(crate) scalar_v1987: bool,
    pub(crate) scalar_v1988: bool,
    pub(crate) scalar_v1996: bool,
    pub(crate) scalar_v2001: f64,
    pub(crate) scalar_v2002: bool,
    pub(crate) scalar_v2006: bool,
    pub(crate) scalar_v2047: f64,
    pub(crate) scalar_v2052: f64,
    pub(crate) scalar_v2055: f64,
    pub(crate) scalar_v2056: f64,
    pub(crate) scalar_v2057: bool,
    pub(crate) scalar_v2058: f64,
    pub(crate) scalar_v2059: bool,
    pub(crate) scalar_v2060: f64,
    pub(crate) scalar_v2061: bool,
    pub(crate) scalar_v2062: f64,
    pub(crate) scalar_v2063: bool,
    pub(crate) scalar_v2064: f64,
    pub(crate) scalar_v2534: f64,
    pub(crate) scalar_v2535: f64,
    pub(crate) scalar_v2536: f64,
    pub(crate) scalar_v2537: f64,
    pub(crate) scalar_v3480: f64,
    pub(crate) scalar_v3504: f64,
    pub(crate) scalar_v3505: f64,
    pub(crate) scalar_v3522: f64,
    pub(crate) scalar_v3608: f64,
    pub(crate) scalar_v3627: f64,
    pub(crate) scalar_v3970: f64,
    pub(crate) scalar_v3971: f64,
    pub(crate) scalar_v3980: f64,
    pub(crate) scalar_v3981: f64,
    pub(crate) scalar_v4005: f64,
    pub(crate) scalar_v4006: f64,
    pub(crate) scalar_v4017: f64,
    pub(crate) scalar_v4018: f64,
    pub(crate) scalar_v4483: f64,
    pub(crate) scalar_v4540: f64,
    pub(crate) scalar_v4541: f64,
    pub(crate) scalar_v4604: f64,
    pub(crate) scalar_v4605: f64,
    pub(crate) scalar_v4738: f64,
    pub(crate) scalar_v4795: f64,
    pub(crate) scalar_v4796: f64,
    pub(crate) scalar_v5051: f64,
    pub(crate) scalar_v5052: f64,
    pub(crate) scalar_v5054: f64,
    pub(crate) scalar_v5055: f64,
    pub(crate) scalar_v5258: f64,
    pub(crate) scalar_v5259: f64,
    pub(crate) scalar_v5260: f64,
    pub(crate) scalar_v5261: f64,
    pub(crate) scalar_v5262: f64,
    pub(crate) scalar_v5263: f64,
    pub(crate) scalar_v5690: f64,
    pub(crate) scalar_v6297: f64,
    pub(crate) scalar_v6394: f64,
    pub(crate) scalar_v6707: f64,
    pub(crate) scalar_v6708: f64,
    pub(crate) scalar_v6709: f64,
    pub(crate) scalar_v6710: f64,
    pub(crate) scalar_v6711: f64,
    pub(crate) scalar_v6842: f64,
    pub(crate) scalar_v6843: f64,
    pub(crate) scalar_v6929: f64,
    pub(crate) scalar_v6930: f64,
    pub(crate) scalar_v7004: f64,
    pub(crate) scalar_v7010: f64,
    pub(crate) scalar_v7136: f64,
    pub(crate) scalar_v7137: f64,
    pub(crate) scalar_v7196: f64,
    pub(crate) scalar_v7197: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v606: f64,
    pub(crate) scalar_v608: f64,
    pub(crate) scalar_v609: f64,
    pub(crate) scalar_v1980: f64,
    pub(crate) scalar_v1981: f64,
    pub(crate) scalar_v1989: f64,
    pub(crate) scalar_v1990: f64,
    pub(crate) scalar_v1991: f64,
    pub(crate) scalar_v6997: f64,
    pub(crate) scalar_v6998: f64,
    pub(crate) scalar_v6999: f64,
    pub(crate) scalar_v7000: f64,
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
            scalar_v0: self.scalar_v0,
            scalar_v2: self.scalar_v2,
            scalar_v5: self.scalar_v5,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v10: self.scalar_v10,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v17: self.scalar_v17,
            scalar_v19: self.scalar_v19,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v32: self.scalar_v32,
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
            scalar_v43: self.scalar_v43,
            scalar_v45: self.scalar_v45,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v72: self.scalar_v72,
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
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
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v107: self.scalar_v107,
            scalar_v120: self.scalar_v120,
            scalar_v122: self.scalar_v122,
            scalar_v177: self.scalar_v177,
            scalar_v197: self.scalar_v197,
            scalar_v200: self.scalar_v200,
            scalar_v240: self.scalar_v240,
            scalar_v243: self.scalar_v243,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v308: self.scalar_v308,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v345: self.scalar_v345,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v376: self.scalar_v376,
            scalar_v378: self.scalar_v378,
            scalar_v379: self.scalar_v379,
            scalar_v397: self.scalar_v397,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v407: self.scalar_v407,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v423: self.scalar_v423,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v437: self.scalar_v437,
            scalar_v438: self.scalar_v438,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v444: self.scalar_v444,
            scalar_v449: self.scalar_v449,
            scalar_v450: self.scalar_v450,
            scalar_v451: self.scalar_v451,
            scalar_v453: self.scalar_v453,
            scalar_v457: self.scalar_v457,
            scalar_v458: self.scalar_v458,
            scalar_v463: self.scalar_v463,
            scalar_v464: self.scalar_v464,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v474: self.scalar_v474,
            scalar_v475: self.scalar_v475,
            scalar_v481: self.scalar_v481,
            scalar_v482: self.scalar_v482,
            scalar_v483: self.scalar_v483,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v496: self.scalar_v496,
            scalar_v497: self.scalar_v497,
            scalar_v498: self.scalar_v498,
            scalar_v502: self.scalar_v502,
            scalar_v503: self.scalar_v503,
            scalar_v507: self.scalar_v507,
            scalar_v508: self.scalar_v508,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v517: self.scalar_v517,
            scalar_v518: self.scalar_v518,
            scalar_v519: self.scalar_v519,
            scalar_v526: self.scalar_v526,
            scalar_v529: self.scalar_v529,
            scalar_v537: self.scalar_v537,
            scalar_v546: self.scalar_v546,
            scalar_v559: self.scalar_v559,
            scalar_v568: self.scalar_v568,
            scalar_v580: self.scalar_v580,
            scalar_v583: self.scalar_v583,
            scalar_v601: self.scalar_v601,
            scalar_v603: self.scalar_v603,
            scalar_v605: self.scalar_v605,
            scalar_v607: self.scalar_v607,
            scalar_v610: self.scalar_v610,
            scalar_v616: self.scalar_v616,
            scalar_v618: self.scalar_v618,
            scalar_v624: self.scalar_v624,
            scalar_v626: self.scalar_v626,
            scalar_v632: self.scalar_v632,
            scalar_v675: self.scalar_v675,
            scalar_v680: self.scalar_v680,
            scalar_v768: self.scalar_v768,
            scalar_v821: self.scalar_v821,
            scalar_v822: self.scalar_v822,
            scalar_v823: self.scalar_v823,
            scalar_v834: self.scalar_v834,
            scalar_v855: self.scalar_v855,
            scalar_v856: self.scalar_v856,
            scalar_v857: self.scalar_v857,
            scalar_v858: self.scalar_v858,
            scalar_v859: self.scalar_v859,
            scalar_v860: self.scalar_v860,
            scalar_v907: self.scalar_v907,
            scalar_v917: self.scalar_v917,
            scalar_v930: self.scalar_v930,
            scalar_v931: self.scalar_v931,
            scalar_v935: self.scalar_v935,
            scalar_v984: self.scalar_v984,
            scalar_v985: self.scalar_v985,
            scalar_v986: self.scalar_v986,
            scalar_v1008: self.scalar_v1008,
            scalar_v1016: self.scalar_v1016,
            scalar_v1017: self.scalar_v1017,
            scalar_v1019: self.scalar_v1019,
            scalar_v1020: self.scalar_v1020,
            scalar_v1021: self.scalar_v1021,
            scalar_v1024: self.scalar_v1024,
            scalar_v1025: self.scalar_v1025,
            scalar_v1030: self.scalar_v1030,
            scalar_v1051: self.scalar_v1051,
            scalar_v1053: self.scalar_v1053,
            scalar_v1082: self.scalar_v1082,
            scalar_v1088: self.scalar_v1088,
            scalar_v1122: self.scalar_v1122,
            scalar_v1144: self.scalar_v1144,
            scalar_v1157: self.scalar_v1157,
            scalar_v1175: self.scalar_v1175,
            scalar_v1238: self.scalar_v1238,
            scalar_v1239: self.scalar_v1239,
            scalar_v1240: self.scalar_v1240,
            scalar_v1241: self.scalar_v1241,
            scalar_v1243: self.scalar_v1243,
            scalar_v1244: self.scalar_v1244,
            scalar_v1245: self.scalar_v1245,
            scalar_v1338: self.scalar_v1338,
            scalar_v1339: self.scalar_v1339,
            scalar_v1340: self.scalar_v1340,
            scalar_v1364: self.scalar_v1364,
            scalar_v1366: self.scalar_v1366,
            scalar_v1367: self.scalar_v1367,
            scalar_v1369: self.scalar_v1369,
            scalar_v1429: self.scalar_v1429,
            scalar_v1430: self.scalar_v1430,
            scalar_v1431: self.scalar_v1431,
            scalar_v1457: self.scalar_v1457,
            scalar_v1459: self.scalar_v1459,
            scalar_v1460: self.scalar_v1460,
            scalar_v1462: self.scalar_v1462,
            scalar_v1528: self.scalar_v1528,
            scalar_v1529: self.scalar_v1529,
            scalar_v1530: self.scalar_v1530,
            scalar_v1531: self.scalar_v1531,
            scalar_v1534: self.scalar_v1534,
            scalar_v1544: self.scalar_v1544,
            scalar_v1545: self.scalar_v1545,
            scalar_v1546: self.scalar_v1546,
            scalar_v1558: self.scalar_v1558,
            scalar_v1563: self.scalar_v1563,
            scalar_v1580: self.scalar_v1580,
            scalar_v1581: self.scalar_v1581,
            scalar_v1585: self.scalar_v1585,
            scalar_v1586: self.scalar_v1586,
            scalar_v1589: self.scalar_v1589,
            scalar_v1595: self.scalar_v1595,
            scalar_v1606: self.scalar_v1606,
            scalar_v1607: self.scalar_v1607,
            scalar_v1608: self.scalar_v1608,
            scalar_v1609: self.scalar_v1609,
            scalar_v1610: self.scalar_v1610,
            scalar_v1611: self.scalar_v1611,
            scalar_v1612: self.scalar_v1612,
            scalar_v1613: self.scalar_v1613,
            scalar_v1614: self.scalar_v1614,
            scalar_v1615: self.scalar_v1615,
            scalar_v1616: self.scalar_v1616,
            scalar_v1617: self.scalar_v1617,
            scalar_v1618: self.scalar_v1618,
            scalar_v1619: self.scalar_v1619,
            scalar_v1620: self.scalar_v1620,
            scalar_v1634: self.scalar_v1634,
            scalar_v1661: self.scalar_v1661,
            scalar_v1662: self.scalar_v1662,
            scalar_v1663: self.scalar_v1663,
            scalar_v1666: self.scalar_v1666,
            scalar_v1685: self.scalar_v1685,
            scalar_v1699: self.scalar_v1699,
            scalar_v1704: self.scalar_v1704,
            scalar_v1706: self.scalar_v1706,
            scalar_v1710: self.scalar_v1710,
            scalar_v1711: self.scalar_v1711,
            scalar_v1712: self.scalar_v1712,
            scalar_v1713: self.scalar_v1713,
            scalar_v1714: self.scalar_v1714,
            scalar_v1723: self.scalar_v1723,
            scalar_v1724: self.scalar_v1724,
            scalar_v1727: self.scalar_v1727,
            scalar_v1750: self.scalar_v1750,
            scalar_v1751: self.scalar_v1751,
            scalar_v1757: self.scalar_v1757,
            scalar_v1758: self.scalar_v1758,
            scalar_v1759: self.scalar_v1759,
            scalar_v1807: self.scalar_v1807,
            scalar_v1808: self.scalar_v1808,
            scalar_v1813: self.scalar_v1813,
            scalar_v1817: self.scalar_v1817,
            scalar_v1824: self.scalar_v1824,
            scalar_v1829: self.scalar_v1829,
            scalar_v1849: self.scalar_v1849,
            scalar_v1869: self.scalar_v1869,
            scalar_v1870: self.scalar_v1870,
            scalar_v1905: self.scalar_v1905,
            scalar_v1911: self.scalar_v1911,
            scalar_v1967: self.scalar_v1967,
            scalar_v1968: self.scalar_v1968,
            scalar_v1969: self.scalar_v1969,
            scalar_v1970: self.scalar_v1970,
            scalar_v1971: self.scalar_v1971,
            scalar_v1975: self.scalar_v1975,
            scalar_v1976: self.scalar_v1976,
            scalar_v1977: self.scalar_v1977,
            scalar_v1978: self.scalar_v1978,
            scalar_v1979: self.scalar_v1979,
            scalar_v1987: self.scalar_v1987,
            scalar_v1988: self.scalar_v1988,
            scalar_v1996: self.scalar_v1996,
            scalar_v2001: self.scalar_v2001,
            scalar_v2002: self.scalar_v2002,
            scalar_v2006: self.scalar_v2006,
            scalar_v2047: self.scalar_v2047,
            scalar_v2052: self.scalar_v2052,
            scalar_v2055: self.scalar_v2055,
            scalar_v2056: self.scalar_v2056,
            scalar_v2057: self.scalar_v2057,
            scalar_v2058: self.scalar_v2058,
            scalar_v2059: self.scalar_v2059,
            scalar_v2060: self.scalar_v2060,
            scalar_v2061: self.scalar_v2061,
            scalar_v2062: self.scalar_v2062,
            scalar_v2063: self.scalar_v2063,
            scalar_v2064: self.scalar_v2064,
            scalar_v2534: self.scalar_v2534,
            scalar_v2535: self.scalar_v2535,
            scalar_v2536: self.scalar_v2536,
            scalar_v2537: self.scalar_v2537,
            scalar_v3480: self.scalar_v3480,
            scalar_v3504: self.scalar_v3504,
            scalar_v3505: self.scalar_v3505,
            scalar_v3522: self.scalar_v3522,
            scalar_v3608: self.scalar_v3608,
            scalar_v3627: self.scalar_v3627,
            scalar_v3970: self.scalar_v3970,
            scalar_v3971: self.scalar_v3971,
            scalar_v3980: self.scalar_v3980,
            scalar_v3981: self.scalar_v3981,
            scalar_v4005: self.scalar_v4005,
            scalar_v4006: self.scalar_v4006,
            scalar_v4017: self.scalar_v4017,
            scalar_v4018: self.scalar_v4018,
            scalar_v4483: self.scalar_v4483,
            scalar_v4540: self.scalar_v4540,
            scalar_v4541: self.scalar_v4541,
            scalar_v4604: self.scalar_v4604,
            scalar_v4605: self.scalar_v4605,
            scalar_v4738: self.scalar_v4738,
            scalar_v4795: self.scalar_v4795,
            scalar_v4796: self.scalar_v4796,
            scalar_v5051: self.scalar_v5051,
            scalar_v5052: self.scalar_v5052,
            scalar_v5054: self.scalar_v5054,
            scalar_v5055: self.scalar_v5055,
            scalar_v5258: self.scalar_v5258,
            scalar_v5259: self.scalar_v5259,
            scalar_v5260: self.scalar_v5260,
            scalar_v5261: self.scalar_v5261,
            scalar_v5262: self.scalar_v5262,
            scalar_v5263: self.scalar_v5263,
            scalar_v5690: self.scalar_v5690,
            scalar_v6297: self.scalar_v6297,
            scalar_v6394: self.scalar_v6394,
            scalar_v6707: self.scalar_v6707,
            scalar_v6708: self.scalar_v6708,
            scalar_v6709: self.scalar_v6709,
            scalar_v6710: self.scalar_v6710,
            scalar_v6711: self.scalar_v6711,
            scalar_v6842: self.scalar_v6842,
            scalar_v6843: self.scalar_v6843,
            scalar_v6929: self.scalar_v6929,
            scalar_v6930: self.scalar_v6930,
            scalar_v7004: self.scalar_v7004,
            scalar_v7010: self.scalar_v7010,
            scalar_v7136: self.scalar_v7136,
            scalar_v7137: self.scalar_v7137,
            scalar_v7196: self.scalar_v7196,
            scalar_v7197: self.scalar_v7197,
            scalar_v20: self.scalar_v20,
            scalar_v606: self.scalar_v606,
            scalar_v608: self.scalar_v608,
            scalar_v609: self.scalar_v609,
            scalar_v1980: self.scalar_v1980,
            scalar_v1981: self.scalar_v1981,
            scalar_v1989: self.scalar_v1989,
            scalar_v1990: self.scalar_v1990,
            scalar_v1991: self.scalar_v1991,
            scalar_v6997: self.scalar_v6997,
            scalar_v6998: self.scalar_v6998,
            scalar_v6999: self.scalar_v6999,
            scalar_v7000: self.scalar_v7000,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["e1", "b1", "b2", "c1", "c2", "c3", "c4", "noi"];

    pub const BRANCH_COUNT: usize = 2;
    pub const PARAMETER_COUNT: usize = 147;
    pub const VARIABLE_COUNT: usize = 585;
    pub const DDT_STATE_COUNT: usize = 10;
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
            scalar_v0: 0.0,
            scalar_v2: false,
            scalar_v5: 0.0,
            scalar_v7: 0.0,
            scalar_v8: false,
            scalar_v10: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v17: 0.0,
            scalar_v19: 0.0,
            scalar_v21: 0.0,
            scalar_v22: false,
            scalar_v24: 0.0,
            scalar_v25: false,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v32: 0.0,
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
            scalar_v43: 0.0,
            scalar_v45: 0.0,
            scalar_v47: 0.0,
            scalar_v48: false,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: false,
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v65: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: false,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: false,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v107: 0.0,
            scalar_v120: 0.0,
            scalar_v122: 0.0,
            scalar_v177: 0.0,
            scalar_v197: 0.0,
            scalar_v200: 0.0,
            scalar_v240: 0.0,
            scalar_v243: 0.0,
            scalar_v269: 0.0,
            scalar_v270: 0.0,
            scalar_v277: 0.0,
            scalar_v278: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v308: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v315: 0.0,
            scalar_v316: false,
            scalar_v317: 0.0,
            scalar_v345: false,
            scalar_v347: 0.0,
            scalar_v348: false,
            scalar_v349: 0.0,
            scalar_v376: false,
            scalar_v378: 0.0,
            scalar_v379: 0.0,
            scalar_v397: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v401: 0.0,
            scalar_v402: 0.0,
            scalar_v407: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v423: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v437: 0.0,
            scalar_v438: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v444: 0.0,
            scalar_v449: 0.0,
            scalar_v450: 0.0,
            scalar_v451: 0.0,
            scalar_v453: 0.0,
            scalar_v457: 0.0,
            scalar_v458: 0.0,
            scalar_v463: 0.0,
            scalar_v464: 0.0,
            scalar_v471: 0.0,
            scalar_v472: false,
            scalar_v473: 0.0,
            scalar_v474: 0.0,
            scalar_v475: 0.0,
            scalar_v481: 0.0,
            scalar_v482: 0.0,
            scalar_v483: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v496: 0.0,
            scalar_v497: 0.0,
            scalar_v498: 0.0,
            scalar_v502: 0.0,
            scalar_v503: 0.0,
            scalar_v507: 0.0,
            scalar_v508: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v517: 0.0,
            scalar_v518: 0.0,
            scalar_v519: 0.0,
            scalar_v526: 0.0,
            scalar_v529: 0.0,
            scalar_v537: 0.0,
            scalar_v546: 0.0,
            scalar_v559: 0.0,
            scalar_v568: 0.0,
            scalar_v580: 0.0,
            scalar_v583: 0.0,
            scalar_v601: 0.0,
            scalar_v603: 0.0,
            scalar_v605: 0.0,
            scalar_v607: 0.0,
            scalar_v610: false,
            scalar_v616: false,
            scalar_v618: false,
            scalar_v624: false,
            scalar_v626: false,
            scalar_v632: false,
            scalar_v675: 0.0,
            scalar_v680: 0.0,
            scalar_v768: 0.0,
            scalar_v821: 0.0,
            scalar_v822: 0.0,
            scalar_v823: 0.0,
            scalar_v834: 0.0,
            scalar_v855: 0.0,
            scalar_v856: 0.0,
            scalar_v857: 0.0,
            scalar_v858: 0.0,
            scalar_v859: 0.0,
            scalar_v860: 0.0,
            scalar_v907: 0.0,
            scalar_v917: 0.0,
            scalar_v930: 0.0,
            scalar_v931: false,
            scalar_v935: false,
            scalar_v984: 0.0,
            scalar_v985: 0.0,
            scalar_v986: 0.0,
            scalar_v1008: 0.0,
            scalar_v1016: 0.0,
            scalar_v1017: false,
            scalar_v1019: false,
            scalar_v1020: false,
            scalar_v1021: false,
            scalar_v1024: false,
            scalar_v1025: false,
            scalar_v1030: 0.0,
            scalar_v1051: 0.0,
            scalar_v1053: 0.0,
            scalar_v1082: false,
            scalar_v1088: false,
            scalar_v1122: 0.0,
            scalar_v1144: 0.0,
            scalar_v1157: 0.0,
            scalar_v1175: 0.0,
            scalar_v1238: 0.0,
            scalar_v1239: false,
            scalar_v1240: false,
            scalar_v1241: false,
            scalar_v1243: false,
            scalar_v1244: false,
            scalar_v1245: 0.0,
            scalar_v1338: false,
            scalar_v1339: false,
            scalar_v1340: false,
            scalar_v1364: 0.0,
            scalar_v1366: 0.0,
            scalar_v1367: 0.0,
            scalar_v1369: 0.0,
            scalar_v1429: false,
            scalar_v1430: false,
            scalar_v1431: false,
            scalar_v1457: 0.0,
            scalar_v1459: 0.0,
            scalar_v1460: 0.0,
            scalar_v1462: 0.0,
            scalar_v1528: 0.0,
            scalar_v1529: false,
            scalar_v1530: false,
            scalar_v1531: false,
            scalar_v1534: 0.0,
            scalar_v1544: 0.0,
            scalar_v1545: false,
            scalar_v1546: false,
            scalar_v1558: 0.0,
            scalar_v1563: 0.0,
            scalar_v1580: false,
            scalar_v1581: false,
            scalar_v1585: 0.0,
            scalar_v1586: false,
            scalar_v1589: 0.0,
            scalar_v1595: 0.0,
            scalar_v1606: 0.0,
            scalar_v1607: 0.0,
            scalar_v1608: 0.0,
            scalar_v1609: 0.0,
            scalar_v1610: 0.0,
            scalar_v1611: 0.0,
            scalar_v1612: 0.0,
            scalar_v1613: 0.0,
            scalar_v1614: 0.0,
            scalar_v1615: 0.0,
            scalar_v1616: 0.0,
            scalar_v1617: 0.0,
            scalar_v1618: 0.0,
            scalar_v1619: 0.0,
            scalar_v1620: 0.0,
            scalar_v1634: false,
            scalar_v1661: 0.0,
            scalar_v1662: false,
            scalar_v1663: 0.0,
            scalar_v1666: 0.0,
            scalar_v1685: 0.0,
            scalar_v1699: 0.0,
            scalar_v1704: false,
            scalar_v1706: false,
            scalar_v1710: 0.0,
            scalar_v1711: 0.0,
            scalar_v1712: 0.0,
            scalar_v1713: 0.0,
            scalar_v1714: 0.0,
            scalar_v1723: 0.0,
            scalar_v1724: false,
            scalar_v1727: false,
            scalar_v1750: 0.0,
            scalar_v1751: 0.0,
            scalar_v1757: 0.0,
            scalar_v1758: 0.0,
            scalar_v1759: 0.0,
            scalar_v1807: false,
            scalar_v1808: false,
            scalar_v1813: 0.0,
            scalar_v1817: 0.0,
            scalar_v1824: 0.0,
            scalar_v1829: 0.0,
            scalar_v1849: 0.0,
            scalar_v1869: 0.0,
            scalar_v1870: false,
            scalar_v1905: false,
            scalar_v1911: false,
            scalar_v1967: 0.0,
            scalar_v1968: false,
            scalar_v1969: 0.0,
            scalar_v1970: false,
            scalar_v1971: false,
            scalar_v1975: 0.0,
            scalar_v1976: false,
            scalar_v1977: false,
            scalar_v1978: false,
            scalar_v1979: false,
            scalar_v1987: false,
            scalar_v1988: false,
            scalar_v1996: false,
            scalar_v2001: 0.0,
            scalar_v2002: false,
            scalar_v2006: false,
            scalar_v2047: 0.0,
            scalar_v2052: 0.0,
            scalar_v2055: 0.0,
            scalar_v2056: 0.0,
            scalar_v2057: false,
            scalar_v2058: 0.0,
            scalar_v2059: false,
            scalar_v2060: 0.0,
            scalar_v2061: false,
            scalar_v2062: 0.0,
            scalar_v2063: false,
            scalar_v2064: 0.0,
            scalar_v2534: 0.0,
            scalar_v2535: 0.0,
            scalar_v2536: 0.0,
            scalar_v2537: 0.0,
            scalar_v3480: 0.0,
            scalar_v3504: 0.0,
            scalar_v3505: 0.0,
            scalar_v3522: 0.0,
            scalar_v3608: 0.0,
            scalar_v3627: 0.0,
            scalar_v3970: 0.0,
            scalar_v3971: 0.0,
            scalar_v3980: 0.0,
            scalar_v3981: 0.0,
            scalar_v4005: 0.0,
            scalar_v4006: 0.0,
            scalar_v4017: 0.0,
            scalar_v4018: 0.0,
            scalar_v4483: 0.0,
            scalar_v4540: 0.0,
            scalar_v4541: 0.0,
            scalar_v4604: 0.0,
            scalar_v4605: 0.0,
            scalar_v4738: 0.0,
            scalar_v4795: 0.0,
            scalar_v4796: 0.0,
            scalar_v5051: 0.0,
            scalar_v5052: 0.0,
            scalar_v5054: 0.0,
            scalar_v5055: 0.0,
            scalar_v5258: 0.0,
            scalar_v5259: 0.0,
            scalar_v5260: 0.0,
            scalar_v5261: 0.0,
            scalar_v5262: 0.0,
            scalar_v5263: 0.0,
            scalar_v5690: 0.0,
            scalar_v6297: 0.0,
            scalar_v6394: 0.0,
            scalar_v6707: 0.0,
            scalar_v6708: 0.0,
            scalar_v6709: 0.0,
            scalar_v6710: 0.0,
            scalar_v6711: 0.0,
            scalar_v6842: 0.0,
            scalar_v6843: 0.0,
            scalar_v6929: 0.0,
            scalar_v6930: 0.0,
            scalar_v7004: 0.0,
            scalar_v7010: 0.0,
            scalar_v7136: 0.0,
            scalar_v7137: 0.0,
            scalar_v7196: 0.0,
            scalar_v7197: 0.0,
            scalar_v20: 0.0,
            scalar_v606: 0.0,
            scalar_v608: 0.0,
            scalar_v609: 0.0,
            scalar_v1980: 0.0,
            scalar_v1981: 0.0,
            scalar_v1989: 0.0,
            scalar_v1990: 0.0,
            scalar_v1991: 0.0,
            scalar_v6997: 0.0,
            scalar_v6998: 0.0,
            scalar_v6999: 0.0,
            scalar_v7000: 0.0,
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
            scalar_v0,
            scalar_v2,
            scalar_v5,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v32,
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
            scalar_v43,
            scalar_v45,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v107,
            scalar_v120,
            scalar_v122,
            scalar_v177,
            scalar_v197,
            scalar_v200,
            scalar_v240,
            scalar_v243,
            scalar_v269,
            scalar_v270,
            scalar_v277,
            scalar_v278,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v291,
            scalar_v292,
            scalar_v298,
            scalar_v299,
            scalar_v303,
            scalar_v304,
            scalar_v308,
            scalar_v310,
            scalar_v311,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v345,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v376,
            scalar_v378,
            scalar_v379,
            scalar_v397,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v407,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v423,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v431,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v453,
            scalar_v457,
            scalar_v458,
            scalar_v463,
            scalar_v464,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v502,
            scalar_v503,
            scalar_v507,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v526,
            scalar_v529,
            scalar_v537,
            scalar_v546,
            scalar_v559,
            scalar_v568,
            scalar_v580,
            scalar_v583,
            scalar_v601,
            scalar_v603,
            scalar_v605,
            scalar_v607,
            scalar_v610,
            scalar_v616,
            scalar_v618,
            scalar_v624,
            scalar_v626,
            scalar_v632,
            scalar_v675,
            scalar_v680,
            scalar_v768,
            scalar_v821,
            scalar_v822,
            scalar_v823,
            scalar_v834,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v860,
            scalar_v907,
            scalar_v917,
            scalar_v930,
            scalar_v931,
            scalar_v935,
            scalar_v984,
            scalar_v985,
            scalar_v986,
            scalar_v1008,
            scalar_v1016,
            scalar_v1017,
            scalar_v1019,
            scalar_v1020,
            scalar_v1021,
            scalar_v1024,
            scalar_v1025,
            scalar_v1030,
            scalar_v1051,
            scalar_v1053,
            scalar_v1082,
            scalar_v1088,
            scalar_v1122,
            scalar_v1144,
            scalar_v1157,
            scalar_v1175,
            scalar_v1238,
            scalar_v1239,
            scalar_v1240,
            scalar_v1241,
            scalar_v1243,
            scalar_v1244,
            scalar_v1245,
            scalar_v1338,
            scalar_v1339,
            scalar_v1340,
            scalar_v1364,
            scalar_v1366,
            scalar_v1367,
            scalar_v1369,
            scalar_v1429,
            scalar_v1430,
            scalar_v1431,
            scalar_v1457,
            scalar_v1459,
            scalar_v1460,
            scalar_v1462,
            scalar_v1528,
            scalar_v1529,
            scalar_v1530,
            scalar_v1531,
            scalar_v1534,
            scalar_v1544,
            scalar_v1545,
            scalar_v1546,
            scalar_v1558,
            scalar_v1563,
            scalar_v1580,
            scalar_v1581,
            scalar_v1585,
            scalar_v1586,
            scalar_v1589,
            scalar_v1595,
            scalar_v1606,
            scalar_v1607,
            scalar_v1608,
            scalar_v1609,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1615,
            scalar_v1616,
            scalar_v1617,
            scalar_v1618,
            scalar_v1619,
            scalar_v1620,
            scalar_v1634,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1666,
            scalar_v1685,
            scalar_v1699,
            scalar_v1704,
            scalar_v1706,
            scalar_v1710,
            scalar_v1711,
            scalar_v1712,
            scalar_v1713,
            scalar_v1714,
            scalar_v1723,
            scalar_v1724,
            scalar_v1727,
            scalar_v1750,
            scalar_v1751,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1807,
            scalar_v1808,
            scalar_v1813,
            scalar_v1817,
            scalar_v1824,
            scalar_v1829,
            scalar_v1849,
            scalar_v1869,
            scalar_v1870,
            scalar_v1905,
            scalar_v1911,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1975,
            scalar_v1976,
            scalar_v1977,
            scalar_v1978,
            scalar_v1979,
            scalar_v1987,
            scalar_v1988,
            scalar_v1996,
            scalar_v2001,
            scalar_v2002,
            scalar_v2006,
            scalar_v2047,
            scalar_v2052,
            scalar_v2055,
            scalar_v2056,
            scalar_v2057,
            scalar_v2058,
            scalar_v2059,
            scalar_v2060,
            scalar_v2061,
            scalar_v2062,
            scalar_v2063,
            scalar_v2064,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v3480,
            scalar_v3504,
            scalar_v3505,
            scalar_v3522,
            scalar_v3608,
            scalar_v3627,
            scalar_v3970,
            scalar_v3971,
            scalar_v3980,
            scalar_v3981,
            scalar_v4005,
            scalar_v4006,
            scalar_v4017,
            scalar_v4018,
            scalar_v4483,
            scalar_v4540,
            scalar_v4541,
            scalar_v4604,
            scalar_v4605,
            scalar_v4738,
            scalar_v4795,
            scalar_v4796,
            scalar_v5051,
            scalar_v5052,
            scalar_v5054,
            scalar_v5055,
            scalar_v5258,
            scalar_v5259,
            scalar_v5260,
            scalar_v5261,
            scalar_v5262,
            scalar_v5263,
            scalar_v5690,
            scalar_v6297,
            scalar_v6394,
            scalar_v6707,
            scalar_v6708,
            scalar_v6709,
            scalar_v6710,
            scalar_v6711,
            scalar_v6842,
            scalar_v6843,
            scalar_v6929,
            scalar_v6930,
            scalar_v7004,
            scalar_v7010,
            scalar_v7136,
            scalar_v7137,
            scalar_v7196,
            scalar_v7197,
            scalar_v20,
            scalar_v606,
            scalar_v608,
            scalar_v609,
            scalar_v1980,
            scalar_v1981,
            scalar_v1989,
            scalar_v1990,
            scalar_v1991,
            scalar_v6997,
            scalar_v6998,
            scalar_v6999,
            scalar_v7000,
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
            scalar_v0,
            scalar_v2,
            scalar_v5,
            scalar_v7,
            scalar_v8,
            scalar_v10,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
            scalar_v29,
            scalar_v32,
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
            scalar_v43,
            scalar_v45,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v73,
            scalar_v74,
            scalar_v75,
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
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v107,
            scalar_v120,
            scalar_v122,
            scalar_v177,
            scalar_v197,
            scalar_v200,
            scalar_v240,
            scalar_v243,
            scalar_v269,
            scalar_v270,
            scalar_v277,
            scalar_v278,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v291,
            scalar_v292,
            scalar_v298,
            scalar_v299,
            scalar_v303,
            scalar_v304,
            scalar_v308,
            scalar_v310,
            scalar_v311,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v345,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v376,
            scalar_v378,
            scalar_v379,
            scalar_v397,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v407,
            scalar_v412,
            scalar_v413,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v423,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v431,
            scalar_v432,
            scalar_v437,
            scalar_v438,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v449,
            scalar_v450,
            scalar_v451,
            scalar_v453,
            scalar_v457,
            scalar_v458,
            scalar_v463,
            scalar_v464,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v474,
            scalar_v475,
            scalar_v481,
            scalar_v482,
            scalar_v483,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v496,
            scalar_v497,
            scalar_v498,
            scalar_v502,
            scalar_v503,
            scalar_v507,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v517,
            scalar_v518,
            scalar_v519,
            scalar_v526,
            scalar_v529,
            scalar_v537,
            scalar_v546,
            scalar_v559,
            scalar_v568,
            scalar_v580,
            scalar_v583,
            scalar_v601,
            scalar_v603,
            scalar_v605,
            scalar_v607,
            scalar_v610,
            scalar_v616,
            scalar_v618,
            scalar_v624,
            scalar_v626,
            scalar_v632,
            scalar_v675,
            scalar_v680,
            scalar_v768,
            scalar_v821,
            scalar_v822,
            scalar_v823,
            scalar_v834,
            scalar_v855,
            scalar_v856,
            scalar_v857,
            scalar_v858,
            scalar_v859,
            scalar_v860,
            scalar_v907,
            scalar_v917,
            scalar_v930,
            scalar_v931,
            scalar_v935,
            scalar_v984,
            scalar_v985,
            scalar_v986,
            scalar_v1008,
            scalar_v1016,
            scalar_v1017,
            scalar_v1019,
            scalar_v1020,
            scalar_v1021,
            scalar_v1024,
            scalar_v1025,
            scalar_v1030,
            scalar_v1051,
            scalar_v1053,
            scalar_v1082,
            scalar_v1088,
            scalar_v1122,
            scalar_v1144,
            scalar_v1157,
            scalar_v1175,
            scalar_v1238,
            scalar_v1239,
            scalar_v1240,
            scalar_v1241,
            scalar_v1243,
            scalar_v1244,
            scalar_v1245,
            scalar_v1338,
            scalar_v1339,
            scalar_v1340,
            scalar_v1364,
            scalar_v1366,
            scalar_v1367,
            scalar_v1369,
            scalar_v1429,
            scalar_v1430,
            scalar_v1431,
            scalar_v1457,
            scalar_v1459,
            scalar_v1460,
            scalar_v1462,
            scalar_v1528,
            scalar_v1529,
            scalar_v1530,
            scalar_v1531,
            scalar_v1534,
            scalar_v1544,
            scalar_v1545,
            scalar_v1546,
            scalar_v1558,
            scalar_v1563,
            scalar_v1580,
            scalar_v1581,
            scalar_v1585,
            scalar_v1586,
            scalar_v1589,
            scalar_v1595,
            scalar_v1606,
            scalar_v1607,
            scalar_v1608,
            scalar_v1609,
            scalar_v1610,
            scalar_v1611,
            scalar_v1612,
            scalar_v1613,
            scalar_v1614,
            scalar_v1615,
            scalar_v1616,
            scalar_v1617,
            scalar_v1618,
            scalar_v1619,
            scalar_v1620,
            scalar_v1634,
            scalar_v1661,
            scalar_v1662,
            scalar_v1663,
            scalar_v1666,
            scalar_v1685,
            scalar_v1699,
            scalar_v1704,
            scalar_v1706,
            scalar_v1710,
            scalar_v1711,
            scalar_v1712,
            scalar_v1713,
            scalar_v1714,
            scalar_v1723,
            scalar_v1724,
            scalar_v1727,
            scalar_v1750,
            scalar_v1751,
            scalar_v1757,
            scalar_v1758,
            scalar_v1759,
            scalar_v1807,
            scalar_v1808,
            scalar_v1813,
            scalar_v1817,
            scalar_v1824,
            scalar_v1829,
            scalar_v1849,
            scalar_v1869,
            scalar_v1870,
            scalar_v1905,
            scalar_v1911,
            scalar_v1967,
            scalar_v1968,
            scalar_v1969,
            scalar_v1970,
            scalar_v1971,
            scalar_v1975,
            scalar_v1976,
            scalar_v1977,
            scalar_v1978,
            scalar_v1979,
            scalar_v1987,
            scalar_v1988,
            scalar_v1996,
            scalar_v2001,
            scalar_v2002,
            scalar_v2006,
            scalar_v2047,
            scalar_v2052,
            scalar_v2055,
            scalar_v2056,
            scalar_v2057,
            scalar_v2058,
            scalar_v2059,
            scalar_v2060,
            scalar_v2061,
            scalar_v2062,
            scalar_v2063,
            scalar_v2064,
            scalar_v2534,
            scalar_v2535,
            scalar_v2536,
            scalar_v2537,
            scalar_v3480,
            scalar_v3504,
            scalar_v3505,
            scalar_v3522,
            scalar_v3608,
            scalar_v3627,
            scalar_v3970,
            scalar_v3971,
            scalar_v3980,
            scalar_v3981,
            scalar_v4005,
            scalar_v4006,
            scalar_v4017,
            scalar_v4018,
            scalar_v4483,
            scalar_v4540,
            scalar_v4541,
            scalar_v4604,
            scalar_v4605,
            scalar_v4738,
            scalar_v4795,
            scalar_v4796,
            scalar_v5051,
            scalar_v5052,
            scalar_v5054,
            scalar_v5055,
            scalar_v5258,
            scalar_v5259,
            scalar_v5260,
            scalar_v5261,
            scalar_v5262,
            scalar_v5263,
            scalar_v5690,
            scalar_v6297,
            scalar_v6394,
            scalar_v6707,
            scalar_v6708,
            scalar_v6709,
            scalar_v6710,
            scalar_v6711,
            scalar_v6842,
            scalar_v6843,
            scalar_v6929,
            scalar_v6930,
            scalar_v7004,
            scalar_v7010,
            scalar_v7136,
            scalar_v7137,
            scalar_v7196,
            scalar_v7197,
            scalar_v20,
            scalar_v606,
            scalar_v608,
            scalar_v609,
            scalar_v1980,
            scalar_v1981,
            scalar_v1989,
            scalar_v1990,
            scalar_v1991,
            scalar_v6997,
            scalar_v6998,
            scalar_v6999,
            scalar_v7000,
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
            "dta" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trise" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dta", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult" => { validate_parameter("mult", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((505.5, "505.5")), false, Some((505.51, "505.51")), true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("tref", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exmod" => { validate_parameter("exmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exphi" => { validate_parameter("exphi", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "exavl" => { validate_parameter("exavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "is" => { validate_parameter("is", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nff" => { validate_parameter("nff", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfr" => { validate_parameter("nfr", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ik" => { validate_parameter("ik", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ver" => { validate_parameter("ver", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vef" => { validate_parameter("vef", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "issr" => { validate_parameter("issr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibi" => { validate_parameter("ibi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbi" => { validate_parameter("nbi", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibis" => { validate_parameter("ibis", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nbis" => { validate_parameter("nbis", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibf" => { validate_parameter("ibf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlf" => { validate_parameter("mlf", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibfs" => { validate_parameter("ibfs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlfs" => { validate_parameter("mlfs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swib1" => { validate_parameter("swib1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbr" => { validate_parameter("ibinbr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrs" => { validate_parameter("ibinbrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vknbr" => { validate_parameter("vknbr", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibinbrqs" => { validate_parameter("ibinbrqs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibx" => { validate_parameter("ibx", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ikbx" => { validate_parameter("ikbx", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibr" => { validate_parameter("ibr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mlr" => { validate_parameter("mlr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xext" => { validate_parameter("xext", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izeb" => { validate_parameter("izeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzeb" => { validate_parameter("nzeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "izcb" => { validate_parameter("izcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nzcb" => { validate_parameter("nzcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzmin" => { validate_parameter("vzmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swavl" => { validate_parameter("swavl", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aavl" => { validate_parameter("aavl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cavl" => { validate_parameter("cavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "itoavl" => { validate_parameter("itoavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bavl" => { validate_parameter("bavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcavl" => { validate_finite_parameter("vdcavl", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wavl" => { validate_parameter("wavl", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vavl" => { validate_parameter("vavl", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfh" => { validate_parameter("sfh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihcavl" => { validate_parameter("ihcavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "davl" => { validate_parameter("davl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eavl" => { validate_parameter("eavl", value, None, true, Some((0.0, "0.0")), true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aexavl" => { validate_parameter("aexavl", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ionexavl" => { validate_parameter("ionexavl", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swgemlim" => { validate_parameter("swgemlim", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "re" => { validate_parameter("re", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbc" => { validate_parameter("rbc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbv" => { validate_parameter("rbv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcc" => { validate_parameter("rcc", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcblx" => { validate_parameter("rcblx", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcbli" => { validate_parameter("rcbli", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcv" => { validate_parameter("rcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scrcv" => { validate_parameter("scrcv", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ihc" => { validate_parameter("ihc", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axi" => { validate_parameter("axi", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdc" => { validate_parameter("vdc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cje" => { validate_parameter("cje", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vde" => { validate_parameter("vde", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pe" => { validate_parameter("pe", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcje" => { validate_parameter("xcje", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbeo" => { validate_parameter("cbeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjc" => { validate_parameter("cjc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcctc" => { validate_parameter("vdcctc", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pc" => { validate_parameter("pc", value, Some((0.01, "0.01")), false, Some((0.99, "0.99")), true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvchc" => { validate_parameter("swvchc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swvjunc" => { validate_parameter("swvjunc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xp" => { validate_parameter("xp", value, Some((0.0, "0.0")), false, Some((0.99, "0.99")), true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mc" => { validate_parameter("mc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcjc" => { validate_parameter("xcjc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbco" => { validate_parameter("cbco", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqex" => { validate_parameter("swqex", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdcex" => { validate_parameter("vdcex", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbrcb" => { validate_parameter("vbrcb", value, Some((0.0, "0.0")), true, Some((2000.0, "2000.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbrcb" => { validate_parameter("pbrcb", value, Some((0.0, "0.0")), true, Some((500.0, "500.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "frevcb" => { validate_parameter("frevcb", value, Some((10.0, "10.0")), true, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swjbrcb" => { validate_parameter("swjbrcb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mtau" => { validate_parameter("mtau", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taue" => { validate_parameter("taue", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taub" => { validate_parameter("taub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taur" => { validate_parameter("taur", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tauex" => { validate_parameter("tauex", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nex" => { validate_parameter("nex", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deg" => { validate_finite_parameter("deg", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xrec" => { validate_parameter("xrec", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqb" => { validate_parameter("xqb", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ke" => { validate_parameter("ke", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aqbo" => { validate_finite_parameter("aqbo", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ae" => { validate_finite_parameter("ae", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ab" => { validate_finite_parameter("ab", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepi" => { validate_finite_parameter("aepi", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aepiex" => { validate_finite_parameter("aepiex", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "aex" => { validate_finite_parameter("aex", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ac" => { validate_finite_parameter("ac", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acx" => { validate_finite_parameter("acx", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "acbl" => { validate_parameter("acbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgb" => { validate_parameter("vgb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrqs" => { validate_parameter("vgbnbrqs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbr" => { validate_parameter("vgbnbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgbnbrs" => { validate_parameter("vgbnbrs", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgknbr" => { validate_parameter("vgknbr", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgc" => { validate_parameter("vgc", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vge" => { validate_parameter("vge", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgcx" => { validate_parameter("vgcx", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgj" => { validate_parameter("vgj", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzeb" => { validate_parameter("vgzeb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgeb" => { validate_finite_parameter("avgeb", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgeb" => { validate_parameter("tvgeb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgzcb" => { validate_parameter("vgzcb", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "avgcb" => { validate_finite_parameter("avgcb", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tvgcb" => { validate_parameter("tvgcb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvgte" => { validate_finite_parameter("dvgte", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dais" => { validate_finite_parameter("dais", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnff" => { validate_finite_parameter("tnff", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnfr" => { validate_finite_parameter("tnfr", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbavl" => { validate_finite_parameter("tbavl", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtmax" => { validate_parameter("dtmax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "afn" => { validate_parameter("afn", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfn" => { validate_parameter("kfn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kavl" => { validate_parameter("kavl", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kc" => { validate_parameter("kc", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ftaun" => { validate_parameter("ftaun", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swnlsh" => { validate_parameter("swnlsh", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ath" => { validate_finite_parameter("ath", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isibrel" => { validate_parameter("isibrel", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfibrel" => { validate_parameter("nfibrel", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vexlim" => { validate_parameter("vexlim", value, Some((40.0, "40.0")), false, Some((400.0, "400.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "p0starlim" => { validate_parameter("p0starlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwlim" => { validate_parameter("pwlim", value, Some((0.0, "0.0")), false, Some((1e-20, "1e-20")), false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "istat" => { validate_parameter("istat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtat" => { validate_parameter("vtat", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ktat" => { validate_finite_parameter("ktat", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbtbt" => { validate_parameter("vbtbt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbtbt" => { validate_finite_parameter("kbtbt", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bjtd505t_va'", name)),
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
        let v0: f64 = p.p3;
        self.scalar_v0 = v0;
        let v2: bool = (p.p3 == 1.0);
        self.scalar_v2 = v2;
        let v5: f64 = (if v2 { 70300000.0 } else { 0.0 });
        self.scalar_v5 = v5;
        let v7: f64 = (if v2 { 123000000.0 } else { 0.0 });
        self.scalar_v7 = v7;
        let v8: bool = (!v2);
        self.scalar_v8 = v8;
        let v10: f64 = (if v8 { 158000000.0 } else { v5 });
        self.scalar_v10 = v10;
        let v12: f64 = (if v8 { 204000000.0 } else { v7 });
        self.scalar_v12 = v12;
        let v13: f64 = p.p32;
        self.scalar_v13 = v13;
        let v14: f64 = (1.0 - p.p32);
        self.scalar_v14 = v14;
        let v15: f64 = p.p4;
        self.scalar_v15 = v15;
        let v17: f64 = (p.p4 + 273.15);
        self.scalar_v17 = v17;
        let v19: f64 = p.p0;
        self.scalar_v19 = v19;
        let v21: f64 = p.p141;
        self.scalar_v21 = v21;
        let v22: bool = (0.0 == p.p141);
        self.scalar_v22 = v22;
        let v24: f64 = (if v22 { 1e-12 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: bool = (!v22);
        self.scalar_v25 = v25;
        let v26: f64 = (if v25 { p.p141 } else { v24 });
        self.scalar_v26 = v26;
        let v27: f64 = p.p1;
        self.scalar_v27 = v27;
        let v28: f64 = (v26 * p.p1);
        self.scalar_v28 = v28;
        let v29: f64 = (1.0 / v28);
        self.scalar_v29 = v29;
        let v32: f64 = p.p66;
        self.scalar_v32 = v32;
        let v33: f64 = (2.0 - p.p66);
        self.scalar_v33 = v33;
        let v34: f64 = f64::powf(2.0, v33);
        self.scalar_v34 = v34;
        let v35: f64 = (1.0 / v34);
        self.scalar_v35 = v35;
        let v36: f64 = p.p113;
        self.scalar_v36 = v36;
        let v37: f64 = p.p114;
        self.scalar_v37 = v37;
        let v38: f64 = (v17 * p.p114);
        self.scalar_v38 = v38;
        let v39: f64 = (v17 * v38);
        self.scalar_v39 = v39;
        let v40: f64 = p.p115;
        self.scalar_v40 = v40;
        let v41: f64 = (v17 + p.p115);
        self.scalar_v41 = v41;
        let v42: f64 = (v39 / v41);
        self.scalar_v42 = v42;
        let v43: f64 = (p.p113 + v42);
        self.scalar_v43 = v43;
        let v45: f64 = (v43 - 0.05);
        self.scalar_v45 = v45;
        let v47: f64 = (v45 / 0.1);
        self.scalar_v47 = v47;
        let v48: bool = (v43 < 0.05);
        self.scalar_v48 = v48;
        let v49: f64 = ((v47) as f64).exp();
        self.scalar_v49 = v49;
        let v50: f64 = (1.0 + v49);
        self.scalar_v50 = v50;
        let v51: f64 = ((v50) as f64).ln();
        self.scalar_v51 = v51;
        let v52: f64 = (0.1 * v51);
        self.scalar_v52 = v52;
        let v53: f64 = (0.05 + v52);
        self.scalar_v53 = v53;
        let v54: f64 = (if v48 { v53 } else { 0.0 });
        self.scalar_v54 = v54;
        let v55: bool = (!v48);
        self.scalar_v55 = v55;
        let v56: f64 = (-v47);
        self.scalar_v56 = v56;
        let v57: f64 = ((v56) as f64).exp();
        self.scalar_v57 = v57;
        let v58: f64 = (1.0 + v57);
        self.scalar_v58 = v58;
        let v59: f64 = ((v58) as f64).ln();
        self.scalar_v59 = v59;
        let v60: f64 = (0.1 * v59);
        self.scalar_v60 = v60;
        let v61: f64 = (v43 + v60);
        self.scalar_v61 = v61;
        let v62: f64 = (if v55 { v61 } else { v54 });
        self.scalar_v62 = v62;
        let v63: f64 = (1.0 / p.p113);
        self.scalar_v63 = v63;
        let v64: f64 = p.p65;
        self.scalar_v64 = v64;
        let v65: f64 = (1.0 / p.p65);
        self.scalar_v65 = v65;
        let v66: f64 = p.p70;
        self.scalar_v66 = v66;
        let v67: f64 = p.p71;
        self.scalar_v67 = v67;
        let v68: f64 = (2.0 - p.p71);
        self.scalar_v68 = v68;
        let v69: f64 = f64::powf(2.0, v68);
        self.scalar_v69 = v69;
        let v70: f64 = (1.0 / v69);
        self.scalar_v70 = v70;
        let v71: f64 = p.p116;
        self.scalar_v71 = v71;
        let v72: f64 = p.p117;
        self.scalar_v72 = v72;
        let v73: f64 = (v17 * p.p117);
        self.scalar_v73 = v73;
        let v74: f64 = (v17 * v73);
        self.scalar_v74 = v74;
        let v75: f64 = p.p118;
        self.scalar_v75 = v75;
        let v76: f64 = (v17 + p.p118);
        self.scalar_v76 = v76;
        let v77: f64 = (v74 / v76);
        self.scalar_v77 = v77;
        let v78: f64 = (p.p116 + v77);
        self.scalar_v78 = v78;
        let v79: f64 = (v78 - 0.05);
        self.scalar_v79 = v79;
        let v80: f64 = (v79 / 0.1);
        self.scalar_v80 = v80;
        let v81: bool = (v78 < 0.05);
        self.scalar_v81 = v81;
        let v82: f64 = ((v80) as f64).exp();
        self.scalar_v82 = v82;
        let v83: f64 = (1.0 + v82);
        self.scalar_v83 = v83;
        let v84: f64 = ((v83) as f64).ln();
        self.scalar_v84 = v84;
        let v85: f64 = (0.1 * v84);
        self.scalar_v85 = v85;
        let v86: f64 = (0.05 + v85);
        self.scalar_v86 = v86;
        let v87: f64 = (if v81 { v86 } else { 0.0 });
        self.scalar_v87 = v87;
        let v88: bool = (!v81);
        self.scalar_v88 = v88;
        let v89: f64 = (-v80);
        self.scalar_v89 = v89;
        let v90: f64 = ((v89) as f64).exp();
        self.scalar_v90 = v90;
        let v91: f64 = (1.0 + v90);
        self.scalar_v91 = v91;
        let v92: f64 = ((v91) as f64).ln();
        self.scalar_v92 = v92;
        let v93: f64 = (0.1 * v92);
        self.scalar_v93 = v93;
        let v94: f64 = (v78 + v93);
        self.scalar_v94 = v94;
        let v95: f64 = (if v88 { v94 } else { v87 });
        self.scalar_v95 = v95;
        let v96: f64 = (1.0 / p.p116);
        self.scalar_v96 = v96;
        let v97: f64 = (1.0 / p.p70);
        self.scalar_v97 = v97;
        let v98: f64 = p.p82;
        self.scalar_v98 = v98;
        let v99: f64 = (1.0 / p.p82);
        self.scalar_v99 = v99;
        let v100: f64 = (1.0 - v99);
        self.scalar_v100 = v100;
        let v107: f64 = p.p124;
        self.scalar_v107 = v107;
        let v120: f64 = (v17 * 8.617086918058125e-5);
        self.scalar_v120 = v120;
        let v122: f64 = (1.0 / v120);
        self.scalar_v122 = v122;
        let v177: f64 = p.p104;
        self.scalar_v177 = v177;
        let v197: f64 = p.p63;
        self.scalar_v197 = v197;
        let v200: f64 = p.p109;
        self.scalar_v200 = v200;
        let v240: f64 = p.p26;
        self.scalar_v240 = v240;
        let v243: f64 = p.p108;
        self.scalar_v243 = v243;
        let v269: f64 = p.p74;
        self.scalar_v269 = v269;
        let v270: f64 = (1.0 - p.p74);
        self.scalar_v270 = v270;
        let v277: f64 = p.p53;
        self.scalar_v277 = v277;
        let v278: f64 = p.p96;
        self.scalar_v278 = v278;
        let v284: f64 = p.p55;
        self.scalar_v284 = v284;
        let v285: f64 = p.p97;
        self.scalar_v285 = v285;
        let v286: f64 = p.p95;
        self.scalar_v286 = v286;
        let v287: f64 = (p.p97 - p.p95);
        self.scalar_v287 = v287;
        let v291: f64 = p.p54;
        self.scalar_v291 = v291;
        let v292: f64 = p.p100;
        self.scalar_v292 = v292;
        let v298: f64 = p.p56;
        self.scalar_v298 = v298;
        let v299: f64 = p.p101;
        self.scalar_v299 = v299;
        let v303: f64 = p.p57;
        self.scalar_v303 = v303;
        let v304: f64 = p.p103;
        self.scalar_v304 = v304;
        let v308: f64 = p.p58;
        self.scalar_v308 = v308;
        let v310: f64 = p.p59;
        self.scalar_v310 = v310;
        let v311: f64 = p.p98;
        self.scalar_v311 = v311;
        let v315: f64 = p.p121;
        self.scalar_v315 = v315;
        let v316: bool = (0.0 != p.p121);
        self.scalar_v316 = v316;
        let v317: f64 = p.p9;
        self.scalar_v317 = v317;
        let v345: bool = (!v316);
        self.scalar_v345 = v345;
        let v347: f64 = p.p122;
        self.scalar_v347 = v347;
        let v348: bool = (0.0 != p.p122);
        self.scalar_v348 = v348;
        let v349: f64 = p.p10;
        self.scalar_v349 = v349;
        let v376: bool = (!v348);
        self.scalar_v376 = v376;
        let v378: f64 = p.p42;
        self.scalar_v378 = v378;
        let v379: f64 = p.p123;
        self.scalar_v379 = v379;
        let v397: f64 = p.p8;
        self.scalar_v397 = v397;
        let v399: f64 = (4.0 - p.p97);
        self.scalar_v399 = v399;
        let v400: f64 = (v399 - p.p95);
        self.scalar_v400 = v400;
        let v401: f64 = p.p120;
        self.scalar_v401 = v401;
        let v402: f64 = (v400 + p.p120);
        self.scalar_v402 = v402;
        let v407: f64 = (-p.p104);
        self.scalar_v407 = v407;
        let v412: f64 = p.p11;
        self.scalar_v412 = v412;
        let v413: f64 = (1.0 - p.p97);
        self.scalar_v413 = v413;
        let v417: f64 = p.p29;
        self.scalar_v417 = v417;
        let v418: f64 = p.p102;
        self.scalar_v418 = v418;
        let v419: f64 = (1.0 - p.p102);
        self.scalar_v419 = v419;
        let v423: f64 = p.p19;
        self.scalar_v423 = v423;
        let v425: f64 = p.p20;
        self.scalar_v425 = v425;
        let v426: f64 = (2.0 * p.p20);
        self.scalar_v426 = v426;
        let v427: f64 = (6.0 - v426);
        self.scalar_v427 = v427;
        let v431: f64 = p.p112;
        self.scalar_v431 = v431;
        let v432: f64 = (-p.p112);
        self.scalar_v432 = v432;
        let v437: f64 = p.p30;
        self.scalar_v437 = v437;
        let v438: f64 = p.p31;
        self.scalar_v438 = v438;
        let v439: f64 = (2.0 * p.p31);
        self.scalar_v439 = v439;
        let v440: f64 = (6.0 - v439);
        self.scalar_v440 = v440;
        let v444: f64 = (-p.p109);
        self.scalar_v444 = v444;
        let v449: f64 = p.p15;
        self.scalar_v449 = v449;
        let v450: f64 = (4.0 - p.p96);
        self.scalar_v450 = v450;
        let v451: f64 = (p.p120 + v450);
        self.scalar_v451 = v451;
        let v453: f64 = p.p16;
        self.scalar_v453 = v453;
        let v457: f64 = p.p110;
        self.scalar_v457 = v457;
        let v458: f64 = (-p.p110);
        self.scalar_v458 = v458;
        let v463: f64 = p.p17;
        self.scalar_v463 = v463;
        let v464: f64 = p.p18;
        self.scalar_v464 = v464;
        let v471: f64 = p.p23;
        self.scalar_v471 = v471;
        let v472: bool = (1.0 == p.p23);
        self.scalar_v472 = v472;
        let v473: f64 = p.p24;
        self.scalar_v473 = v473;
        let v474: f64 = p.p106;
        self.scalar_v474 = v474;
        let v475: f64 = (-p.p106);
        self.scalar_v475 = v475;
        let v481: f64 = p.p27;
        self.scalar_v481 = v481;
        let v482: f64 = p.p105;
        self.scalar_v482 = v482;
        let v483: f64 = (-p.p105);
        self.scalar_v483 = v483;
        let v488: f64 = p.p25;
        self.scalar_v488 = v488;
        let v489: f64 = p.p107;
        self.scalar_v489 = v489;
        let v490: f64 = (-p.p107);
        self.scalar_v490 = v490;
        let v496: f64 = p.p28;
        self.scalar_v496 = v496;
        let v497: f64 = (4.0 - p.p102);
        self.scalar_v497 = v497;
        let v498: f64 = (p.p120 + v497);
        self.scalar_v498 = v498;
        let v502: f64 = p.p111;
        self.scalar_v502 = v502;
        let v503: f64 = (-p.p111);
        self.scalar_v503 = v503;
        let v507: f64 = p.p21;
        self.scalar_v507 = v507;
        let v508: f64 = p.p22;
        self.scalar_v508 = v508;
        let v509: f64 = (2.0 * p.p22);
        self.scalar_v509 = v509;
        let v510: f64 = (6.0 - v509);
        self.scalar_v510 = v510;
        let v517: f64 = p.p136;
        self.scalar_v517 = v517;
        let v518: f64 = p.p137;
        self.scalar_v518 = v518;
        let v519: f64 = (4.0 / p.p137);
        self.scalar_v519 = v519;
        let v526: f64 = p.p142;
        self.scalar_v526 = v526;
        let v529: f64 = p.p144;
        self.scalar_v529 = v529;
        let v537: f64 = p.p34;
        self.scalar_v537 = v537;
        let v546: f64 = p.p33;
        self.scalar_v546 = v546;
        let v559: f64 = p.p36;
        self.scalar_v559 = v559;
        let v568: f64 = p.p35;
        self.scalar_v568 = v568;
        let v580: f64 = p.p13;
        self.scalar_v580 = v580;
        let v583: f64 = p.p12;
        self.scalar_v583 = v583;
        let v601: f64 = (v12 * 1.081);
        self.scalar_v601 = v601;
        let v603: f64 = p.p91;
        self.scalar_v603 = v603;
        let v605: f64 = p.p133;
        self.scalar_v605 = v605;
        let v607: f64 = p.p135;
        self.scalar_v607 = v607;
        let v610: bool = (p.p56 > 0.0);
        self.scalar_v610 = v610;
        let v616: bool = (!v610);
        self.scalar_v616 = v616;
        let v618: bool = (p.p57 > 0.0);
        self.scalar_v618 = v618;
        let v624: bool = (!v618);
        self.scalar_v624 = v624;
        let v626: bool = (p.p58 > 0.0);
        self.scalar_v626 = v626;
        let v632: bool = (!v626);
        self.scalar_v632 = v632;
        let v675: f64 = p.p138;
        self.scalar_v675 = v675;
        let v680: f64 = ((p.p138) as f64).exp();
        self.scalar_v680 = v680;
        let v768: f64 = p.p140;
        self.scalar_v768 = v768;
        let v821: f64 = p.p61;
        self.scalar_v821 = v821;
        let v822: f64 = p.p60;
        self.scalar_v822 = v822;
        let v823: f64 = (p.p61 * p.p60);
        self.scalar_v823 = v823;
        let v834: f64 = p.p62;
        self.scalar_v834 = v834;
        let v855: f64 = (-1.0 / p.p62);
        self.scalar_v855 = v855;
        let v856: f64 = ((v855) as f64).exp();
        self.scalar_v856 = v856;
        let v857: f64 = (1.0 + v856);
        self.scalar_v857 = v857;
        let v858: f64 = ((v857) as f64).ln();
        self.scalar_v858 = v858;
        let v859: f64 = (p.p62 * v858);
        self.scalar_v859 = v859;
        let v860: f64 = (1.0 + v859);
        self.scalar_v860 = v860;
        let v907: f64 = p.p139;
        self.scalar_v907 = v907;
        let v917: f64 = (0.5 * p.p60);
        self.scalar_v917 = v917;
        let v930: f64 = p.p72;
        self.scalar_v930 = v930;
        let v931: bool = (0.0 == p.p72);
        self.scalar_v931 = v931;
        let v935: bool = (!v931);
        self.scalar_v935 = v935;
        let v984: f64 = (-1.0 / p.p66);
        self.scalar_v984 = v984;
        let v985: f64 = f64::powf(3.0, v984);
        self.scalar_v985 = v985;
        let v986: f64 = (1.0 - v985);
        self.scalar_v986 = v986;
        let v1008: f64 = (1.0 - p.p66);
        self.scalar_v1008 = v1008;
        let v1016: f64 = p.p73;
        self.scalar_v1016 = v1016;
        let v1017: bool = (1.0 == p.p73);
        self.scalar_v1017 = v1017;
        let v1019: bool = (2.0 == p.p73);
        self.scalar_v1019 = v1019;
        let v1020: bool = (!v1017);
        self.scalar_v1020 = v1020;
        let v1021: bool = (v1019 && v1020);
        self.scalar_v1021 = v1021;
        let v1024: bool = (!v1019);
        self.scalar_v1024 = v1024;
        let v1025: bool = (v1020 && v1024);
        self.scalar_v1025 = v1025;
        let v1030: f64 = (-1.0 / p.p71);
        self.scalar_v1030 = v1030;
        let v1051: f64 = p.p75;
        self.scalar_v1051 = v1051;
        let v1053: f64 = (1.0 - p.p71);
        self.scalar_v1053 = v1053;
        let v1082: bool = (0.0 == p.p91);
        self.scalar_v1082 = v1082;
        let v1088: bool = (!v1082);
        self.scalar_v1088 = v1088;
        let v1122: f64 = p.p14;
        self.scalar_v1122 = v1122;
        let v1144: f64 = p.p143;
        self.scalar_v1144 = v1144;
        let v1157: f64 = p.p145;
        self.scalar_v1157 = v1157;
        let v1175: f64 = p.p146;
        self.scalar_v1175 = v1175;
        let v1238: f64 = p.p92;
        self.scalar_v1238 = v1238;
        let v1239: bool = (0.0 == p.p92);
        self.scalar_v1239 = v1239;
        let v1240: bool = (!v472);
        self.scalar_v1240 = v1240;
        let v1241: bool = (v1239 && v1240);
        self.scalar_v1241 = v1241;
        let v1243: bool = (!v1239);
        self.scalar_v1243 = v1243;
        let v1244: bool = (v1240 && v1243);
        self.scalar_v1244 = v1244;
        let v1245: f64 = (1.0 - p.p92);
        self.scalar_v1245 = v1245;
        let v1338: bool = (p.p33 > 0.0);
        self.scalar_v1338 = v1338;
        let v1339: bool = (p.p34 > 0.0);
        self.scalar_v1339 = v1339;
        let v1340: bool = (v1338 && v1339);
        self.scalar_v1340 = v1340;
        let v1364: f64 = (-2.0 - p.p66);
        self.scalar_v1364 = v1364;
        let v1366: f64 = (p.p66 * p.p66);
        self.scalar_v1366 = v1366;
        let v1367: f64 = (1.0 - v1366);
        self.scalar_v1367 = v1367;
        let v1369: f64 = (p.p66 - 1.0);
        self.scalar_v1369 = v1369;
        let v1429: bool = (p.p35 > 0.0);
        self.scalar_v1429 = v1429;
        let v1430: bool = (p.p36 > 0.0);
        self.scalar_v1430 = v1430;
        let v1431: bool = (v1429 && v1430);
        self.scalar_v1431 = v1431;
        let v1457: f64 = (-2.0 - p.p71);
        self.scalar_v1457 = v1457;
        let v1459: f64 = (p.p71 * p.p71);
        self.scalar_v1459 = v1459;
        let v1460: f64 = (1.0 - v1459);
        self.scalar_v1460 = v1460;
        let v1462: f64 = (p.p71 - 1.0);
        self.scalar_v1462 = v1462;
        let v1528: f64 = p.p5;
        self.scalar_v1528 = v1528;
        let v1529: bool = (p.p5 > 0.0);
        self.scalar_v1529 = v1529;
        let v1530: bool = (p.p32 > 0.0);
        self.scalar_v1530 = v1530;
        let v1531: bool = (v1529 && v1530);
        self.scalar_v1531 = v1531;
        let v1534: f64 = (p.p32 * 2.0);
        self.scalar_v1534 = v1534;
        let v1544: f64 = (if v1531 { 0.0 } else { 0.0 });
        self.scalar_v1544 = v1544;
        let v1545: bool = (1.0 == p.p5);
        self.scalar_v1545 = v1545;
        let v1546: bool = (v1531 && v1545);
        self.scalar_v1546 = v1546;
        let v1558: f64 = (if v1546 { 0.0121 } else { 0.010000000000000002 });
        self.scalar_v1558 = v1558;
        let v1563: f64 = (0.5 * v1558);
        self.scalar_v1563 = v1563;
        let v1580: bool = (!v1545);
        self.scalar_v1580 = v1580;
        let v1581: bool = (v1531 && v1580);
        self.scalar_v1581 = v1581;
        let v1585: f64 = p.p83;
        self.scalar_v1585 = v1585;
        let v1586: bool = (1.0 == p.p83);
        self.scalar_v1586 = v1586;
        let v1589: f64 = (if v1586 { 1e-12 } else { v1558 });
        self.scalar_v1589 = v1589;
        let v1595: f64 = (0.5 * v1589);
        self.scalar_v1595 = v1595;
        let v1606: f64 = p.p81;
        self.scalar_v1606 = v1606;
        let v1607: f64 = f64::powf(v100, p.p81);
        self.scalar_v1607 = v1607;
        let v1608: f64 = (1.0 - v1607);
        self.scalar_v1608 = v1608;
        let v1609: f64 = (1.0 / v1608);
        self.scalar_v1609 = v1609;
        let v1610: f64 = (if v1586 { v1609 } else { 0.0 });
        self.scalar_v1610 = v1610;
        let v1611: f64 = p.p80;
        self.scalar_v1611 = v1611;
        let v1612: f64 = (v100 * p.p80);
        self.scalar_v1612 = v1612;
        let v1613: f64 = (if v1586 { v1612 } else { 0.0 });
        self.scalar_v1613 = v1613;
        let v1614: f64 = (v1610 * v1610);
        self.scalar_v1614 = v1614;
        let v1615: f64 = (p.p81 - 1.0);
        self.scalar_v1615 = v1615;
        let v1616: f64 = f64::powf(v100, v1615);
        self.scalar_v1616 = v1616;
        let v1617: f64 = (v1614 * v1616);
        self.scalar_v1617 = v1617;
        let v1618: f64 = (p.p81 * v1617);
        self.scalar_v1618 = v1618;
        let v1619: f64 = (v1618 / p.p80);
        self.scalar_v1619 = v1619;
        let v1620: f64 = (if v1586 { v1619 } else { 0.0 });
        self.scalar_v1620 = v1620;
        let v1634: bool = (!v1586);
        self.scalar_v1634 = v1634;
        let v1661: f64 = p.p38;
        self.scalar_v1661 = v1661;
        let v1662: bool = (1.0 == p.p38);
        self.scalar_v1662 = v1662;
        let v1663: f64 = p.p43;
        self.scalar_v1663 = v1663;
        let v1666: f64 = p.p41;
        self.scalar_v1666 = v1666;
        let v1685: f64 = p.p40;
        self.scalar_v1685 = v1685;
        let v1699: f64 = p.p39;
        self.scalar_v1699 = v1699;
        let v1704: bool = (2.0 == p.p38);
        self.scalar_v1704 = v1704;
        let v1706: bool = (!v1662);
        self.scalar_v1706 = v1706;
        let v1710: f64 = p.p45;
        self.scalar_v1710 = v1710;
        let v1711: f64 = (2.0 * p.p45);
        self.scalar_v1711 = v1711;
        let v1712: f64 = p.p44;
        self.scalar_v1712 = v1712;
        let v1713: f64 = (p.p44 * p.p44);
        self.scalar_v1713 = v1713;
        let v1714: f64 = (v1711 / v1713);
        self.scalar_v1714 = v1714;
        let v1723: f64 = p.p7;
        self.scalar_v1723 = v1723;
        let v1724: bool = (0.0 == p.p7);
        self.scalar_v1724 = v1724;
        let v1727: bool = (!v1724);
        self.scalar_v1727 = v1727;
        let v1750: f64 = p.p46;
        self.scalar_v1750 = v1750;
        let v1751: f64 = (2.0 * p.p46);
        self.scalar_v1751 = v1751;
        let v1757: f64 = (1.0 + p.p46);
        self.scalar_v1757 = v1757;
        let v1758: f64 = (1.0 + v1751);
        self.scalar_v1758 = v1758;
        let v1759: f64 = (v1757 / v1758);
        self.scalar_v1759 = v1759;
        let v1807: bool = (3.0 == p.p38);
        self.scalar_v1807 = v1807;
        let v1808: bool = (!v1704);
        self.scalar_v1808 = v1808;
        let v1813: f64 = p.p47;
        self.scalar_v1813 = v1813;
        let v1817: f64 = p.p48;
        self.scalar_v1817 = v1817;
        let v1824: f64 = p.p51;
        self.scalar_v1824 = v1824;
        let v1829: f64 = p.p50;
        self.scalar_v1829 = v1829;
        let v1849: f64 = p.p49;
        self.scalar_v1849 = v1849;
        let v1869: f64 = p.p52;
        self.scalar_v1869 = v1869;
        let v1870: bool = (1.0 == p.p52);
        self.scalar_v1870 = v1870;
        let v1905: bool = (!v1807);
        self.scalar_v1905 = v1905;
        let v1911: bool = (!v1870);
        self.scalar_v1911 = v1911;
        let v1967: f64 = (1.0 - p.p135);
        self.scalar_v1967 = v1967;
        let v1968: bool = (p.p133 > v28);
        self.scalar_v1968 = v1968;
        let v1969: f64 = p.p132;
        self.scalar_v1969 = v1969;
        let v1970: bool = (0.0 == p.p132);
        self.scalar_v1970 = v1970;
        let v1971: bool = (v1968 && v1970);
        self.scalar_v1971 = v1971;
        let v1975: f64 = ((v1967) as f64).abs();
        self.scalar_v1975 = v1975;
        let v1976: bool = (v1975 < 1e-6);
        self.scalar_v1976 = v1976;
        let v1977: bool = (!v1970);
        self.scalar_v1977 = v1977;
        let v1978: bool = (v1968 && v1977);
        self.scalar_v1978 = v1978;
        let v1979: bool = (v1976 && v1978);
        self.scalar_v1979 = v1979;
        let v1987: bool = (!v1976);
        self.scalar_v1987 = v1987;
        let v1988: bool = (v1978 && v1987);
        self.scalar_v1988 = v1988;
        let v1996: bool = (!v1968);
        self.scalar_v1996 = v1996;
        let v2001: f64 = p.p129;
        self.scalar_v2001 = v2001;
        let v2002: bool = (p.p129 > 0.0);
        self.scalar_v2002 = v2002;
        let v2006: bool = (!v2002);
        self.scalar_v2006 = v2006;
        let v2047: f64 = (if v624 { 0.0 } else { 0.0 });
        self.scalar_v2047 = v2047;
        let v2052: f64 = (if v632 { 0.0 } else { 0.0 });
        self.scalar_v2052 = v2052;
        let v2055: f64 = (if v472 { 0.0 } else { 0.0 });
        self.scalar_v2055 = v2055;
        let v2056: f64 = (if v1240 { 0.0 } else { 0.0 });
        self.scalar_v2056 = v2056;
        let v2057: bool = (v618 && v626);
        self.scalar_v2057 = v2057;
        let v2058: f64 = (if v2057 { 0.0 } else { 0.0 });
        self.scalar_v2058 = v2058;
        let v2059: bool = (v618 && v632);
        self.scalar_v2059 = v2059;
        let v2060: f64 = (if v2059 { 0.0 } else { 0.0 });
        self.scalar_v2060 = v2060;
        let v2061: bool = (v624 && v626);
        self.scalar_v2061 = v2061;
        let v2062: f64 = (if v2061 { 0.0 } else { 0.0 });
        self.scalar_v2062 = v2062;
        let v2063: bool = (v624 && v632);
        self.scalar_v2063 = v2063;
        let v2064: f64 = (if v2063 { 0.0 } else { 0.0 });
        self.scalar_v2064 = v2064;
        let v2534: f64 = (-p.p3);
        self.scalar_v2534 = v2534;
        let v2535: f64 = (p.p3 + v2534);
        self.scalar_v2535 = v2535;
        let v2536: f64 = (v2534 - v2534);
        self.scalar_v2536 = v2536;
        let v2537: f64 = (p.p3 + v2535);
        self.scalar_v2537 = v2537;
        let v3480: f64 = (v1008 - 1.0);
        self.scalar_v3480 = v3480;
        let v3504: f64 = (if v1017 { p.p3 } else { 0.0 });
        self.scalar_v3504 = v3504;
        let v3505: f64 = (if v1017 { v2534 } else { 0.0 });
        self.scalar_v3505 = v3505;
        let v3522: f64 = (v1030 - 1.0);
        self.scalar_v3522 = v3522;
        let v3608: f64 = (p.p75 - 1.0);
        self.scalar_v3608 = v3608;
        let v3627: f64 = (v1053 - 1.0);
        self.scalar_v3627 = v3627;
        let v3970: f64 = (v2534 / 0.0001);
        self.scalar_v3970 = v3970;
        let v3971: f64 = (p.p3 / 0.0001);
        self.scalar_v3971 = v3971;
        let v3980: f64 = (-v3970);
        self.scalar_v3980 = v3980;
        let v3981: f64 = (-v3971);
        self.scalar_v3981 = v3981;
        let v4005: f64 = (v2534 / 0.001);
        self.scalar_v4005 = v4005;
        let v4006: f64 = (p.p3 / 0.001);
        self.scalar_v4006 = v4006;
        let v4017: f64 = (-v4005);
        self.scalar_v4017 = v4017;
        let v4018: f64 = (-v4006);
        self.scalar_v4018 = v4018;
        let v4483: f64 = (v1364 - 1.0);
        self.scalar_v4483 = v4483;
        let v4540: f64 = (v34 * v2534);
        self.scalar_v4540 = v4540;
        let v4541: f64 = (p.p3 * v34);
        self.scalar_v4541 = v4541;
        let v4604: f64 = (0.5 * v2534);
        self.scalar_v4604 = v4604;
        let v4605: f64 = (p.p3 * 0.5);
        self.scalar_v4605 = v4605;
        let v4738: f64 = (v1457 - 1.0);
        self.scalar_v4738 = v4738;
        let v4795: f64 = (p.p3 * v69);
        self.scalar_v4795 = v4795;
        let v4796: f64 = (v69 * v2534);
        self.scalar_v4796 = v4796;
        let v5051: f64 = (if v1546 { v2535 } else { 0.0 });
        self.scalar_v5051 = v5051;
        let v5052: f64 = (if v1546 { v2537 } else { 0.0 });
        self.scalar_v5052 = v5052;
        let v5054: f64 = (if v1546 { v2536 } else { 0.0 });
        self.scalar_v5054 = v5054;
        let v5055: f64 = (if v1546 { v2534 } else { 0.0 });
        self.scalar_v5055 = v5055;
        let v5258: f64 = (if v1586 { p.p3 } else { 0.0 });
        self.scalar_v5258 = v5258;
        let v5259: f64 = (if v1586 { v2535 } else { 0.0 });
        self.scalar_v5259 = v5259;
        let v5260: f64 = (if v1586 { v2534 } else { 0.0 });
        self.scalar_v5260 = v5260;
        let v5261: f64 = (-v5258);
        self.scalar_v5261 = v5261;
        let v5262: f64 = (-v5259);
        self.scalar_v5262 = v5262;
        let v5263: f64 = (-v5260);
        self.scalar_v5263 = v5263;
        let v5690: f64 = (p.p40 - 1.0);
        self.scalar_v5690 = v5690;
        let v6297: f64 = (p.p48 - 1.0);
        self.scalar_v6297 = v6297;
        let v6394: f64 = (p.p49 - 1.0);
        self.scalar_v6394 = v6394;
        let v6707: f64 = (if v472 { p.p3 } else { 0.0 });
        self.scalar_v6707 = v6707;
        let v6708: f64 = (if v472 { v2534 } else { 0.0 });
        self.scalar_v6708 = v6708;
        let v6709: f64 = (if v1240 { p.p3 } else { v6707 });
        self.scalar_v6709 = v6709;
        let v6710: f64 = (if v1240 { 0.0 } else { v6708 });
        self.scalar_v6710 = v6710;
        let v6711: f64 = (if v1240 { v2534 } else { 0.0 });
        self.scalar_v6711 = v6711;
        let v6842: f64 = (0.0 * v2534);
        self.scalar_v6842 = v6842;
        let v6843: f64 = (p.p3 * 0.0);
        self.scalar_v6843 = v6843;
        let v6929: f64 = (0.0 * v2535);
        self.scalar_v6929 = v6929;
        let v6930: f64 = (0.0 * v2536);
        self.scalar_v6930 = v6930;
        let v7004: f64 = (v1967 - 1.0);
        self.scalar_v7004 = v7004;
        let v7010: f64 = (1.0 / v26);
        self.scalar_v7010 = v7010;
        let v7136: f64 = (p.p3 * p.p3);
        self.scalar_v7136 = v7136;
        let v7137: f64 = (p.p3 * v2534);
        self.scalar_v7137 = v7137;
        let v7196: f64 = (p.p3 * v2535);
        self.scalar_v7196 = v7196;
        let v7197: f64 = (p.p3 * v2536);
        self.scalar_v7197 = v7197;
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
        let v606: f64 = (self.scalar_v20 / self.scalar_v17);
        self.scalar_v606 = v606;
        let v608: f64 = f64::powf(self.scalar_v606, self.scalar_v607);
        self.scalar_v608 = v608;
        let v609: f64 = (self.scalar_v605 * self.scalar_v608);
        self.scalar_v609 = v609;
        let v1980: f64 = (self.scalar_v20 / self.scalar_v609);
        self.scalar_v1980 = v1980;
        let v1981: f64 = (self.scalar_v27 * self.scalar_v1980);
        self.scalar_v1981 = v1981;
        let v1989: f64 = (self.scalar_v609 * self.scalar_v1967);
        self.scalar_v1989 = v1989;
        let v1990: f64 = (self.scalar_v20 / self.scalar_v1989);
        self.scalar_v1990 = v1990;
        let v1991: f64 = (self.scalar_v27 * self.scalar_v1990);
        self.scalar_v1991 = v1991;
        let v6997: f64 = (1.0 / self.scalar_v609);
        self.scalar_v6997 = v6997;
        let v6998: f64 = (self.scalar_v27 * self.scalar_v6997);
        self.scalar_v6998 = v6998;
        let v6999: f64 = (if self.scalar_v1971 { self.scalar_v6998 } else { 0.0 });
        self.scalar_v6999 = v6999;
        let v7000: f64 = (1.0 / self.scalar_v20);
        self.scalar_v7000 = v7000;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
