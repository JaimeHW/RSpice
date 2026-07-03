#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub p0: f64, pub p1: f64, pub p2: f64, pub p3: f64, pub p4: f64, pub p5: f64, pub p6: f64, pub p7: f64, 
    pub p8: f64, pub p9: f64, pub p10: f64, pub p11: f64, pub p12: f64, pub p13: f64, pub p14: f64, pub p15: f64, 
    pub p16: f64, pub p17: f64, pub p18: f64, pub p19: f64, pub p20: f64, pub p21: f64, pub p22: f64, pub p23: f64, 
    pub p24: f64, pub p25: f64, pub p26: f64, pub p27: f64, pub p28: f64, pub p29: f64, pub p30: f64, pub p31: f64, 
    pub p32: f64, pub p33: f64, pub p34: f64, pub p35: f64, pub p36: f64, pub p37: f64, pub p38: f64, pub p39: f64, 
    pub p40: f64, pub p41: f64, pub p42: f64, pub p43: f64, pub p44: f64, pub p45: f64, pub p46: f64, pub p47: f64, 
    pub p48: f64, pub p49: f64, pub p50: f64, pub p51: f64, pub p52: f64, pub p53: f64, pub p54: f64, pub p55: f64, 
    pub p56: f64, pub p57: f64, pub p58: f64, pub p59: f64, pub p60: f64, pub p61: f64, pub p62: f64, pub p63: f64, 
    pub p64: f64, pub p65: f64, pub p66: f64, pub p67: f64, pub p68: f64, pub p69: f64, pub p70: f64, pub p71: f64, 
    pub p72: f64, pub p73: f64, pub p74: f64, pub p75: f64, pub p76: f64, pub p77: f64, pub p78: f64, pub p79: f64, 
    pub p80: f64, pub p81: f64, pub p82: f64, pub p83: f64, pub p84: f64, pub p85: f64, pub p86: f64, pub p87: f64, 
    pub p88: f64, pub p89: f64, pub p90: f64, pub p91: f64, pub p92: f64, pub p93: f64, pub p94: f64, pub p95: f64, 
    pub p96: f64, pub p97: f64, pub p98: f64, pub p99: f64, pub p100: f64, pub p101: f64, pub p102: f64, pub p103: f64, 
    pub p104: f64, pub p105: f64, pub p106: f64, pub p107: f64, pub p108: f64, pub p109: f64, pub p110: f64, pub p111: f64, 
    pub p112: f64, pub p113: f64, pub p114: f64, pub p115: f64, pub p116: f64, pub p117: f64, pub p118: f64, pub p119: f64, 
    pub p120: f64, pub p121: f64, pub p122: f64, pub p123: f64, pub p124: f64, pub p125: f64, pub p126: f64, pub p127: f64, 
    pub p128: f64, pub p129: f64, pub p130: f64, pub p131: f64, pub p132: f64, pub p133: f64, pub p134: f64, pub p135: f64, 
    pub p136: f64, pub p137: f64, pub p138: f64, pub p139: f64, pub p140: f64, pub p141: f64, pub p142: f64, pub p143: f64, 
    pub p144: f64, pub p145: f64, pub p146: f64, pub p147: f64, pub p148: f64, pub p149: f64, pub p150: f64, pub p151: f64, 
    pub p152: f64, pub p153: f64, pub p154: f64, pub p155: f64, pub p156: f64, pub p157: f64, pub p158: f64, pub p159: f64, 
    pub p160: f64, pub p161: f64, pub p162: f64, pub p163: f64, pub p164: f64, pub p165: f64, pub p166: f64, pub p167: f64, 
    pub p168: f64, pub p169: f64, pub p170: f64, pub p171: f64, pub p172: f64, pub p173: f64, pub p174: f64, pub p175: f64, 
    pub p176: f64, pub p177: f64, pub p178: f64, pub p179: f64, pub p180: f64, pub p181: f64, pub p182: f64, pub p183: f64, 
    pub p184: f64, pub p185: f64, pub p186: f64, pub p187: f64, pub p188: f64, pub p189: f64, pub p190: f64, pub p191: f64, 
    pub p192: f64, pub p193: f64, pub p194: f64, pub p195: f64, pub p196: f64, pub p197: f64, pub p198: f64, pub p199: f64, 
    pub p200: f64, pub p201: f64, pub p202: f64, pub p203: f64, pub p204: f64, pub p205: f64, pub p206: f64, pub p207: f64, 
    pub p208: f64, pub p209: f64, pub p210: f64, pub p211: f64, pub p212: f64, pub p213: f64, pub p214: f64, pub p215: f64, 
    pub p216: f64, pub p217: f64, pub p218: f64, pub p219: f64, pub p220: f64, pub p221: f64, pub p222: f64, pub p223: f64, 
    pub p224: f64, pub p225: f64, pub p226: f64, pub p227: f64, pub p228: f64, pub p229: f64, pub p230: f64, pub p231: f64, 
    pub p232: f64, pub p233: f64, pub p234: f64, pub p235: f64, pub p236: f64, pub p237: f64, pub p238: f64, pub p239: f64, 
    pub p240: f64, pub p241: f64, pub p242: f64, pub p243: f64, pub p244: f64, pub p245: f64, pub p246: f64, pub p247: f64, 
    pub p248: f64, pub p249: f64, pub p250: f64, pub p251: f64, pub p252: f64, pub p253: f64, pub p254: f64, pub p255: f64, 
    pub p256: f64, pub p257: f64, pub p258: f64, pub p259: f64, pub p260: f64, pub p261: f64, pub p262: f64, pub p263: f64, 
    pub p264: f64, pub p265: f64, pub p266: f64, pub p267: f64, pub p268: f64, pub p269: f64, pub p270: f64, pub p271: f64, 
    pub p272: f64, pub p273: f64, pub p274: f64, pub p275: f64, pub p276: f64, pub p277: f64, pub p278: f64, pub p279: f64, 
    pub p280: f64, pub p281: f64, pub p282: f64, pub p283: f64, pub p284: f64, pub p285: f64, pub p286: f64, pub p287: f64, 
    pub p288: f64, pub p289: f64, pub p290: f64, pub p291: f64, pub p292: f64, pub p293: f64, pub p294: f64, pub p295: f64, 
    pub p296: f64, pub p297: f64, pub p298: f64, pub p299: f64, pub p300: f64, pub p301: f64, pub p302: f64, pub p303: f64, 
    pub p304: f64, pub p305: f64, pub p306: f64, pub p307: f64, pub p308: f64, pub p309: f64, pub p310: f64, pub p311: f64, 
    pub p312: f64, pub p313: f64, pub p314: f64, pub p315: f64, pub p316: f64, pub p317: f64, pub p318: f64, pub p319: f64, 
    pub p320: f64, pub p321: f64, pub p322: f64, pub p323: f64, pub p324: f64, pub p325: f64, pub p326: f64, pub p327: f64, 
    pub p328: f64, pub p329: f64, pub p330: f64, pub p331: f64, 
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 42] = [
                5e-6, 5e-6, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 27.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 7000000.0, 9.025e-5, 1e-7, 1.1785, 0.0, 0.0,
                1e19, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 42);
            {
                let params = &mut *ptr;
                params.p42 = params.p41;
                validate_finite_parameter("XWDC", params.p42).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 71] = [
                1e-6, 1e-6, 0.0, 0.0, 2.0, 0.0, -1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.1, 1e-8, 1e-8, 0.0,
                1e17, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.23,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0,
                0.0, 0.0, 1.0, 300.0, 30.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.3, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 2000000000000000.0, 2.0,
                0.0, 0.0, 1.0, 1.0, 1.5, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(43), 71);
            {
                let params = &mut *ptr;
                params.p114 = if (params.p33 > 0.0) { 2.0 } else { 1.0 };
                validate_parameter("BB", params.p114, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 113] = [
                10.0, 50.0, 0.0, 10.0, 20.0, 0.0025, 1.0, 2e-6,
                0.8, 3e-8, 0.5, 0.0, 1.0, 0.8, 0.0, 1.0,
                0.0, 1.0, -1.0, 0.0, 1.0, 0.002, 1e-8, 1e-20,
                1.5, 0.55, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                5e17, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 3e-8, 0.7, 2.0, 1.0,
                1.0, 0.0, 0.01, 0.1, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 5e18, 0.0, 0.0,
                0.0, 5e-6, 1000000.0, 0.3, 0.0, 0.2, 1e-6, 0.0,
                10000.0, 20000000.0, 0.3, 0.0, 7500.0, 0.25, 1e-6, 1e-15,
                5000000.0, -5000000.0, 5e-16, 1.0, 0.0, 0.01, 0.005, 10000000000.0,
                1e-16, 0.0, 1.0, 27.0, 1e-10, 0.7, 8e-7, 3.5e-9,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(115), 113);
            {
                let params = &mut *ptr;
                params.p228 = params.p226;
                validate_parameter("TFOXGIDL", params.p228, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 46] = [
                1e-8, 1e17, 1e18, 0.0, 1.0, 0.0, 1.0, 100000000000000.0,
                0.1, 1e-7, 0.0, 3.5, 0.0, 1.0, 0.0, 1.0,
                100.0, 0.0, 0.0, 0.0, 25000.0, 0.0, 2e-8, 1e-8,
                0.0, 3.0, 3.5, 1.0, 0.5, 0.0, 0.0, 1.0,
                1.0, 1.0, 1.0, 1e-11, 1.5e-11, 5e-16, 1.0, 0.0,
                1.0, 1.0, 1.0, 1e-11, 1.5e-11, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(229), 46);
            {
                let params = &mut *ptr;
                params.p275 = params.p94;
                validate_finite_parameter("MUEPH0B", params.p275).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p276 = params.p249;
                validate_finite_parameter("MUEPH1B", params.p276).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p277 = params.p95;
                validate_finite_parameter("MUEPHWB", params.p277).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p278 = params.p96;
                validate_finite_parameter("MUEPWPB", params.p278).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p279 = params.p99;
                validate_finite_parameter("MUEPHSB", params.p279).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p280 = params.p100;
                validate_finite_parameter("MUEPSPB", params.p280).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p281 = params.p97;
                validate_finite_parameter("MUEPHLB", params.p281).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p282 = params.p98;
                validate_finite_parameter("MUEPLPB", params.p282).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p283 = params.p106;
                validate_finite_parameter("MUESR0B", params.p283).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p284 = params.p105;
                validate_parameter("MUESR1B", params.p284, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p285 = params.p107;
                validate_finite_parameter("MUESRLB", params.p285).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p286 = params.p110;
                validate_finite_parameter("MUESLPB", params.p286).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p287 = params.p108;
                validate_finite_parameter("MUESRWB", params.p287).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p288 = params.p109;
                validate_finite_parameter("MUESWPB", params.p288).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p289 = params.p86;
                validate_finite_parameter("MUECB0B", params.p289).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p290 = params.p87;
                validate_finite_parameter("MUECB1B", params.p290).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p291 = params.p88;
                validate_finite_parameter("MUECB0LPB", params.p291).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p292 = params.p89;
                validate_finite_parameter("MUECB1LPB", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p293 = params.p90;
                validate_finite_parameter("MUECB0L2B", params.p293).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p294 = params.p91;
                validate_finite_parameter("MUECB0L2PB", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p295 = params.p92;
                validate_finite_parameter("MUECB1L2B", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p296 = params.p93;
                validate_finite_parameter("MUECB1L2PB", params.p296).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(297), 5);
            {
                let params = &mut *ptr;
                params.p302 = params.p299;
                validate_finite_parameter("MUEQBB", params.p302).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1e-6,
                1e-6, 0.0, 0.0, 0.0, 1e19, 1000.0, 1000.0, 30000000.0,
                30000000.0, 0.0, 0.0, 1e-6, 1.0, 1.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(303), 29);
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
}

#[derive(Copy, Clone)]
struct ParameterBound {
    value: f64,
    label: &'static str,
}

const PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
const PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

fn validate_parameter_metadata(index: usize, value: f64) -> Result<(), String> {
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if let Some(min) = PARAMETER_MIN_BOUNDS[index] {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = PARAMETER_MAX_BOUNDS[index] {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in PARAMETER_EXCLUDED_BOUNDS[index] {
        if value == excluded.value {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
        }
    }
    Ok(())
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 335] = [
    ("l", 0), ("w", 1), ("ngcon", 2), ("xgw", 3), ("xgl", 4), ("nf", 5), ("sa", 6), ("sb", 7), ("sd", 8), ("temp", 9), ("dtemp", 10), ("sca", 11), ("scb", 12), ("scc", 13), ("coadov", 14), ("coisub", 15), 
    ("cofbe", 16), ("coiigs", 17), ("cogidl", 18), ("coovlp", 19), ("coign", 20), ("coflick", 21), ("coisti", 22), ("cothrml", 23), ("conqs", 24), ("corg", 25), ("coievb", 26), ("cohist", 27), ("coselfheat", 28), ("covbsbiz", 29), ("coqovsm", 30), ("info", 31), 
    ("conewmub", 32), ("type", 33), ("vmax", 34), ("bgtmp1", 35), ("bgtmp2", 36), ("eg0", 37), ("xld", 38), ("vfbover", 39), ("nover", 40), ("xwd", 41), ("xwdc", 42), ("saref", 43), ("sbref", 44), ("xqy", 45), ("xqy1", 46), ("xqy2", 47), 
    ("rshg", 48), ("vfbc", 49), ("vfbcl1", 50), ("vfbcl1p", 51), ("vfbcl2", 52), ("vfbcl2p", 53), ("vfbhamp", 54), ("vbi", 55), ("parl1", 56), ("parl2", 57), ("lp", 58), ("nsubp", 59), ("nsubp0", 60), ("nsubwp", 61), ("wl1", 62), ("wl1p", 63), 
    ("wl2", 64), ("wl2p", 65), ("scp1", 66), ("scp2", 67), ("scp3", 68), ("sc1", 69), ("sc2", 70), ("sc3", 71), ("scr1", 72), ("scr2", 73), ("scr3", 74), ("pgd1", 75), ("pgd2", 76), ("pgd4", 77), ("ndep", 78), ("ndepl", 79), 
    ("ndeplp", 80), ("ninv", 81), ("ninvl", 82), ("ninvlp", 83), ("ninvd", 84), ("ninvdp", 85), ("muecb0", 86), ("muecb1", 87), ("muecb0lp", 88), ("muecb1lp", 89), ("muecb0l2", 90), ("muecb0l2p", 91), ("muecb1l2", 92), ("muecb1l2p", 93), ("mueph0", 94), ("muephw", 95), 
    ("muepwp", 96), ("muephl", 97), ("mueplp", 98), ("muephs", 99), ("muepsp", 100), ("vtmp", 101), ("vtmpl", 102), ("vtmplp", 103), ("wvth0", 104), ("muesr1", 105), ("muesr0", 106), ("muesrl", 107), ("muesrw", 108), ("mueswp", 109), ("mueslp", 110), ("muetmp", 111), 
    ("muetmpl", 112), ("muetmplp", 113), ("bb", 114), ("ddltmax", 115), ("ddltslp", 116), ("ddltict", 117), ("sub1", 118), ("sub2", 119), ("sub1l", 120), ("sub1lp", 121), ("sub2l", 122), ("svds", 123), ("slg", 124), ("svbs", 125), ("svbsl", 126), ("svbslp", 127), 
    ("svgs", 128), ("svgsl", 129), ("svgslp", 130), ("svgsw", 131), ("svgswp", 132), ("vfbsub", 133), ("vfbsubl", 134), ("vfbsublp", 135), ("subdlt", 136), ("hist1", 137), ("hist2", 138), ("qhe1", 139), ("qhe2", 140), ("evb1", 141), ("evb2", 142), ("evb3", 143), 
    ("fvbs", 144), ("ibpc1", 145), ("ibpc2", 146), ("nsti", 147), ("nstil", 148), ("nstilp", 149), ("nstiw", 150), ("nstiwp", 151), ("wsti", 152), ("ratwsti", 153), ("wstil", 154), ("wstilp", 155), ("wstiw", 156), ("wstiwp", 157), ("scsti1", 158), ("scsti2", 159), 
    ("vthsti", 160), ("vdsti", 161), ("muesti1", 162), ("muesti2", 163), ("muesti3", 164), ("nsubpsti1", 165), ("nsubpsti2", 166), ("nsubpsti3", 167), ("nsubssti1", 168), ("nsubssti2", 169), ("nsubssti3", 170), ("tpoly", 171), ("cgbo", 172), ("cgdo", 173), ("cgso", 174), ("lover", 175), 
    ("clm1", 176), ("clm2", 177), ("clm3", 178), ("clm5", 179), ("clm6", 180), ("vover", 181), ("voverp", 182), ("vovers", 183), ("voversp", 184), ("voverl", 185), ("voverlp", 186), ("voverw", 187), ("voverwp", 188), ("wfc", 189), ("nsubsw", 190), ("nsubcw", 190), 
    ("nsubswp", 191), ("nsubcwp", 191), ("nsubsmax", 192), ("nsubcmax", 192), ("qme1", 193), ("qme2", 194), ("qme3", 195), ("gidl1", 196), ("gidl2", 197), ("gidl3", 198), ("gidl4", 199), ("gidl5", 200), ("gidlbpl1", 201), ("gidlbplt", 202), ("gleak1", 203), ("gleak2", 204), 
    ("gleak3", 205), ("gleak4", 206), ("gleak5", 207), ("gleak6", 208), ("gleak7", 209), ("glksd1", 210), ("glksd2", 211), ("glksd3", 212), ("glkb1", 213), ("glkb2", 214), ("glkb3", 215), ("vzadd0", 216), ("pzadd0", 217), ("nftrp", 218), ("nfalp", 219), ("cit", 220), 
    ("falph", 221), ("tnom", 222), ("dly1", 223), ("dly2", 224), ("dly3", 225), ("tfox", 226), ("tsoi", 227), ("tfoxgidl", 228), ("tbox", 229), ("nsubs", 230), ("nsubb", 231), ("nsubbl", 232), ("nsubblp", 233), ("nsubbw", 234), ("nsubbwp", 235), ("nsubbmin", 236), 
    ("rth0", 237), ("cth0", 238), ("ptl", 239), ("ptp", 240), ("pt2", 241), ("ptlp", 242), ("pt4", 243), ("pt4p", 244), ("ptdlt", 245), ("gdl", 246), ("gdlp", 247), ("gdld", 248), ("mueph1", 249), ("sc5", 250), ("xldl", 251), ("xldlmin", 252), 
    ("muetmp1", 253), ("vbsbnd", 254), ("vbsmax", 255), ("gleak8", 256), ("gleak9", 257), ("gleak10", 258), ("glksd4", 259), ("glksd5", 260), ("glkb4", 261), ("glkb5", 262), ("glkb6", 263), ("glkb7", 264), ("glkb8", 265), ("glkb21", 266), ("glkb22", 267), ("glkb23", 268), 
    ("glkb24", 269), ("glkb25", 270), ("glkb26", 271), ("glkb27", 272), ("glkb28", 273), ("ptmueph", 274), ("mueph0b", 275), ("mueph1b", 276), ("muephwb", 277), ("muepwpb", 278), ("muephsb", 279), ("muepspb", 280), ("muephlb", 281), ("mueplpb", 282), ("muesr0b", 283), ("muesr1b", 284), 
    ("muesrlb", 285), ("mueslpb", 286), ("muesrwb", 287), ("mueswpb", 288), ("muecb0b", 289), ("muecb1b", 290), ("muecb0lpb", 291), ("muecb1lpb", 292), ("muecb0l2b", 293), ("muecb0l2pb", 294), ("muecb1l2b", 295), ("muecb1l2pb", 296), ("pthrou", 297), ("vfbshift", 298), ("mueqb", 299), ("mueqbl", 300), 
    ("mueqblp", 301), ("mueqbb", 302), ("cocinv", 303), ("web", 304), ("wec", 305), ("nsubswpe", 306), ("nsubpwpe", 307), ("nrs", 308), ("nrd", 309), ("ldrift", 310), ("ldrifts", 311), ("cors", 312), ("cord", 313), ("rsh", 314), ("novers", 315), ("rdrmued", 316), 
    ("rdrmues", 317), ("rdrvmaxd", 318), ("rdrvmaxs", 319), ("rdrmuetmp", 320), ("rdrvtmp", 321), ("rdrdjunc", 322), ("rdrbbd", 323), ("rdrbbs", 324), ("rdrbbtmp", 325), ("rdrvmaxw", 326), ("rdrvmaxwp", 327), ("rdrvmaxl", 328), ("rdrvmaxlp", 329), ("rdrmuel", 330), ("rdrmuelp", 331), 
];

const PARAMETER_DISPLAY_NAMES: [&str; 332] = [
    "L", "W", "NGCON", "XGW", "XGL", "NF", "SA", "SB", "SD", "TEMP", "DTEMP", "SCA", "SCB", "SCC", "COADOV", "COISUB", 
    "COFBE", "COIIGS", "COGIDL", "COOVLP", "COIGN", "COFLICK", "COISTI", "COTHRML", "CONQS", "CORG", "COIEVB", "COHIST", "COSELFHEAT", "COVBSBIZ", "COQOVSM", "INFO", 
    "CONEWMUB", "TYPE", "VMAX", "BGTMP1", "BGTMP2", "EG0", "XLD", "VFBOVER", "NOVER", "XWD", "XWDC", "SAREF", "SBREF", "XQY", "XQY1", "XQY2", 
    "RSHG", "VFBC", "VFBCL1", "VFBCL1P", "VFBCL2", "VFBCL2P", "VFBHAMP", "VBI", "PARL1", "PARL2", "LP", "NSUBP", "NSUBP0", "NSUBWP", "WL1", "WL1P", 
    "WL2", "WL2P", "SCP1", "SCP2", "SCP3", "SC1", "SC2", "SC3", "SCR1", "SCR2", "SCR3", "PGD1", "PGD2", "PGD4", "NDEP", "NDEPL", 
    "NDEPLP", "NINV", "NINVL", "NINVLP", "NINVD", "NINVDP", "MUECB0", "MUECB1", "MUECB0LP", "MUECB1LP", "MUECB0L2", "MUECB0L2P", "MUECB1L2", "MUECB1L2P", "MUEPH0", "MUEPHW", 
    "MUEPWP", "MUEPHL", "MUEPLP", "MUEPHS", "MUEPSP", "VTMP", "VTMPL", "VTMPLP", "WVTH0", "MUESR1", "MUESR0", "MUESRL", "MUESRW", "MUESWP", "MUESLP", "MUETMP", 
    "MUETMPL", "MUETMPLP", "BB", "DDLTMAX", "DDLTSLP", "DDLTICT", "SUB1", "SUB2", "SUB1L", "SUB1LP", "SUB2L", "SVDS", "SLG", "SVBS", "SVBSL", "SVBSLP", 
    "SVGS", "SVGSL", "SVGSLP", "SVGSW", "SVGSWP", "VFBSUB", "VFBSUBL", "VFBSUBLP", "SUBDLT", "HIST1", "HIST2", "QHE1", "QHE2", "EVB1", "EVB2", "EVB3", 
    "FVBS", "IBPC1", "IBPC2", "NSTI", "NSTIL", "NSTILP", "NSTIW", "NSTIWP", "WSTI", "RATWSTI", "WSTIL", "WSTILP", "WSTIW", "WSTIWP", "SCSTI1", "SCSTI2", 
    "VTHSTI", "VDSTI", "MUESTI1", "MUESTI2", "MUESTI3", "NSUBPSTI1", "NSUBPSTI2", "NSUBPSTI3", "NSUBSSTI1", "NSUBSSTI2", "NSUBSSTI3", "TPOLY", "CGBO", "CGDO", "CGSO", "LOVER", 
    "CLM1", "CLM2", "CLM3", "CLM5", "CLM6", "VOVER", "VOVERP", "VOVERS", "VOVERSP", "VOVERL", "VOVERLP", "VOVERW", "VOVERWP", "WFC", "NSUBSW", "NSUBSWP", 
    "NSUBSMAX", "QME1", "QME2", "QME3", "GIDL1", "GIDL2", "GIDL3", "GIDL4", "GIDL5", "GIDLBPL1", "GIDLBPLT", "GLEAK1", "GLEAK2", "GLEAK3", "GLEAK4", "GLEAK5", 
    "GLEAK6", "GLEAK7", "GLKSD1", "GLKSD2", "GLKSD3", "GLKB1", "GLKB2", "GLKB3", "VZADD0", "PZADD0", "NFTRP", "NFALP", "CIT", "FALPH", "TNOM", "DLY1", 
    "DLY2", "DLY3", "TFOX", "TSOI", "TFOXGIDL", "TBOX", "NSUBS", "NSUBB", "NSUBBL", "NSUBBLP", "NSUBBW", "NSUBBWP", "NSUBBMIN", "RTH0", "CTH0", "PTL", 
    "PTP", "PT2", "PTLP", "PT4", "PT4P", "PTDLT", "GDL", "GDLP", "GDLD", "MUEPH1", "SC5", "XLDL", "XLDLMIN", "MUETMP1", "VBSBND", "VBSMAX", 
    "GLEAK8", "GLEAK9", "GLEAK10", "GLKSD4", "GLKSD5", "GLKB4", "GLKB5", "GLKB6", "GLKB7", "GLKB8", "GLKB21", "GLKB22", "GLKB23", "GLKB24", "GLKB25", "GLKB26", 
    "GLKB27", "GLKB28", "PTMUEPH", "MUEPH0B", "MUEPH1B", "MUEPHWB", "MUEPWPB", "MUEPHSB", "MUEPSPB", "MUEPHLB", "MUEPLPB", "MUESR0B", "MUESR1B", "MUESRLB", "MUESLPB", "MUESRWB", 
    "MUESWPB", "MUECB0B", "MUECB1B", "MUECB0LPB", "MUECB1LPB", "MUECB0L2B", "MUECB0L2PB", "MUECB1L2B", "MUECB1L2PB", "PTHROU", "VFBSHIFT", "MUEQB", "MUEQBL", "MUEQBLP", "MUEQBB", "COCINV", 
    "WEB", "WEC", "NSUBSWPE", "NSUBPWPE", "NRS", "NRD", "LDRIFT", "LDRIFTS", "CORS", "CORD", "RSH", "NOVERS", "RDRMUED", "RDRMUES", "RDRVMAXD", "RDRVMAXS", 
    "RDRMUETMP", "RDRVTMP", "RDRDJUNC", "RDRBBD", "RDRBBS", "RDRBBTMP", "RDRVMAXW", "RDRVMAXWP", "RDRVMAXL", "RDRVMAXLP", "RDRMUEL", "RDRMUELP", 
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 332] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 22.0, label: "22.0" }), None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, 
    None, None, None, None, 
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 332] = [
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 32.0, label: "32.0" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, 
];

const PARAMETER_RANGE_FLAGS: [u8; 332] = [
    3, 3, 3, 0, 2, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 3, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 2, 3, 0, 2, 3, 0, 2, 3, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 
    0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 0, 3, 3, 3, 3, 
    0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 332] = [
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], 
];

fn parameter_index_for_name(name: &str) -> Option<usize> {
    PARAMETER_NAME_LOOKUP
        .iter()
        .find_map(|(candidate, index)| (*candidate == name).then_some(*index))
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
    pub nodes: [usize; 13],
    pub branches: [usize; 8],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 332]>,
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
    pub(crate) scalar_static_f64: Box<[f64; 947]>,
    pub(crate) scalar_static_bool: Box<[bool; 115]>,
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
            scalar_static_f64: self.scalar_static_f64.clone(),
            scalar_static_bool: self.scalar_static_bool.clone(),
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 9;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["t", "gp", "bp", "n", "nqs_qi", "nqs_qb", "nqs_qhs", "dp", "sp"];

    pub const BRANCH_COUNT: usize = 8;
    pub const PARAMETER_COUNT: usize = 332;
    pub const VARIABLE_COUNT: usize = 1096;
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
            scalar_static_f64: boxed_zero_f64_array::<947>(),
            scalar_static_bool: boxed_zero_bool_array::<115>(),
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
            scalar_static_f64,
            scalar_static_bool,
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
            scalar_static_f64,
            scalar_static_bool,
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
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimsotb_va'", name));
        };
        validate_parameter_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
        Ok(())
    }

    #[inline]
    fn write_parameter_slot(&mut self, index: usize, value: f64) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        // SAFETY: Parameters is repr(C), contains only f64 fields, and index is produced from generated parameter metadata.
        unsafe {
            let ptr = self.params.as_mut() as *mut Parameters as *mut f64;
            *ptr.add(index) = value;
        }
    }

    #[inline]
    fn finish_set_parameter(&mut self, index: usize) {
        self.mark_param_given(index);
        self.recompute_instance_static(); self.invalidate_temperature_static(); 
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
        self.scalar_static_f64[0]=p.p24;
        self.scalar_static_f64[1]=if param_given[172] { 1.0 } else { 0.0 };
        self.scalar_static_f64[2]=if param_given[173] { 1.0 } else { 0.0 };
        self.scalar_static_f64[3]=if param_given[174] { 1.0 } else { 0.0 };
        self.scalar_static_f64[4]=if param_given[9] { 1.0 } else { 0.0 };
        self.scalar_static_f64[5]=p.p239;
        self.scalar_static_bool[0]=(0.0!=self.scalar_static_f64[5]);
        self.scalar_static_f64[6]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[7]=p.p207;
        self.scalar_static_f64[8]=p.p17;
        self.scalar_static_f64[9]=p.p228;
        self.scalar_static_f64[10]=p.p18;
        self.scalar_static_f64[11]=p.p201;
        self.scalar_static_f64[12]=p.p162;
        self.scalar_static_f64[13]=p.p164;
        self.scalar_static_f64[14]=if param_given[177] { 1.0 } else { 0.0 };
        self.scalar_static_f64[15]=p.p177;
        self.scalar_static_f64[16]=p.p227;
        self.scalar_static_f64[17]=p.p230;
        self.scalar_static_f64[18]=(self.scalar_static_f64[16]*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(5000000000.0/self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=(if (self.scalar_static_f64[14]!=0.0){self.scalar_static_f64[15]}else{self.scalar_static_f64[19]});
        self.scalar_static_bool[1]=(self.scalar_static_f64[20]<2.1);
        self.scalar_static_bool[2]=(self.scalar_static_bool[1]&&true);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[22]=(2.1-self.scalar_static_f64[20]);
        self.scalar_static_f64[23]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=(self.scalar_static_f64[23]*self.scalar_static_f64[23]);
        self.scalar_static_f64[25]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[24]}else{0.0});
        self.scalar_static_f64[26]=(if (self.scalar_static_f64[21]!=0.0){0.010000000000000002}else{0.0});
        self.scalar_static_f64[27]=(if (self.scalar_static_f64[21]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[28]=(self.scalar_static_f64[25]*self.scalar_static_f64[27]);
        self.scalar_static_f64[29]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[28]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[30]=(self.scalar_static_f64[26]*self.scalar_static_f64[27]);
        self.scalar_static_f64[31]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[30]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[32]=(self.scalar_static_f64[25]*self.scalar_static_f64[29]);
        self.scalar_static_f64[33]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[32]}else{self.scalar_static_f64[29]});
        self.scalar_static_f64[34]=(self.scalar_static_f64[26]*self.scalar_static_f64[31]);
        self.scalar_static_f64[35]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[34]}else{self.scalar_static_f64[31]});
        self.scalar_static_f64[36]=(self.scalar_static_f64[33]+self.scalar_static_f64[35]);
        self.scalar_static_f64[37]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[36]}else{0.0});
        self.scalar_static_f64[38]=(if (self.scalar_static_f64[21]!=0.0){self.scalar_static_f64[37]}else{0.0});
        self.scalar_static_bool[3]=((self.scalar_static_f64[21]!=0.0)&&false);
        self.scalar_static_f64[39]=(0.1*self.scalar_static_f64[23]);
        self.scalar_static_f64[40]=p.p34;
        self.scalar_static_f64[41]=(self.scalar_static_f64[40]*0.01);
        self.scalar_static_f64[42]=p.p59;
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]/1e-6);
        self.scalar_static_f64[44]=p.p101;
        self.scalar_static_f64[45]=(0.01*self.scalar_static_f64[44]);
        self.scalar_static_f64[46]=p.p192;
        self.scalar_static_f64[47]=(self.scalar_static_f64[46]/1e-6);
        self.scalar_static_f64[48]=p.p219;
        self.scalar_static_f64[49]=(0.01*self.scalar_static_f64[48]);
        self.scalar_static_f64[50]=p.p220;
        self.scalar_static_f64[51]=(self.scalar_static_f64[50]/0.0001);
        self.scalar_static_f64[52]=(self.scalar_static_f64[17]/1e-6);
        self.scalar_static_f64[53]=p.p231;
        self.scalar_static_f64[54]=(self.scalar_static_f64[53]/1e-6);
        self.scalar_static_f64[55]=p.p237;
        self.scalar_static_f64[56]=p.p40;
        self.scalar_static_f64[57]=(self.scalar_static_f64[56]/1e-6);
        self.scalar_static_f64[58]=p.p236;
        self.scalar_static_f64[59]=(self.scalar_static_f64[58]/1e-6);
        self.scalar_static_f64[60]=p.p197;
        self.scalar_static_f64[61]=(self.scalar_static_f64[60]/0.01);
        self.scalar_static_f64[62]=p.p306;
        self.scalar_static_f64[63]=(self.scalar_static_f64[62]/1e-6);
        self.scalar_static_f64[64]=p.p307;
        self.scalar_static_f64[65]=(self.scalar_static_f64[64]/1e-6);
        self.scalar_static_f64[66]=p.p189;
        self.scalar_static_f64[67]=(self.scalar_static_f64[66]*10000.0);
        self.scalar_static_f64[68]=p.p147;
        self.scalar_static_f64[69]=(self.scalar_static_f64[68]/1e-6);
        self.scalar_static_f64[70]=p.p196;
        self.scalar_static_f64[71]=(self.scalar_static_f64[70]/10.0);
        self.scalar_static_f64[72]=p.p222;
        self.scalar_static_f64[73]=(self.scalar_static_f64[72]+273.15);
        self.scalar_static_f64[74]=p.p9;
        self.scalar_static_f64[75]=(273.15+self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=p.p41;
        self.scalar_static_f64[77]=p.p42;
        self.scalar_static_f64[78]=p.p0;
        self.scalar_static_f64[79]=p.p1;
        self.scalar_static_f64[80]=p.p5;
        self.scalar_static_f64[81]=(self.scalar_static_f64[79]/self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=(self.scalar_static_f64[78]*1000000.0);
        self.scalar_static_f64[83]=(self.scalar_static_f64[81]*1000000.0);
        self.scalar_static_f64[84]=(self.scalar_static_f64[82]*self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=p.p62;
        self.scalar_static_f64[86]=p.p63;
        self.scalar_static_f64[87]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[86]);
        self.scalar_static_f64[88]=(self.scalar_static_f64[85]/self.scalar_static_f64[87]);
        self.scalar_static_f64[89]=(self.scalar_static_f64[78]+self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[81]+self.scalar_static_f64[88]);
        self.scalar_static_f64[91]=p.p64;
        self.scalar_static_f64[92]=p.p65;
        self.scalar_static_f64[93]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=(self.scalar_static_f64[91]/self.scalar_static_f64[93]);
        self.scalar_static_f64[95]=p.p148;
        self.scalar_static_f64[96]=(1000000.0*self.scalar_static_f64[89]);
        self.scalar_static_f64[97]=p.p149;
        self.scalar_static_f64[98]=f64::powf(self.scalar_static_f64[96],self.scalar_static_f64[97]);
        self.scalar_static_f64[99]=(self.scalar_static_f64[95]/self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(1.0+self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=p.p150;
        self.scalar_static_f64[102]=(1000000.0*self.scalar_static_f64[90]);
        self.scalar_static_f64[103]=p.p151;
        self.scalar_static_f64[104]=f64::powf(self.scalar_static_f64[102],self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=(self.scalar_static_f64[101]/self.scalar_static_f64[104]);
        self.scalar_static_f64[106]=(1.0+self.scalar_static_f64[105]);
        self.scalar_static_f64[107]=(self.scalar_static_f64[69]*self.scalar_static_f64[100]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[106]*self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=p.p154;
        self.scalar_static_f64[110]=p.p155;
        self.scalar_static_f64[111]=f64::powf(self.scalar_static_f64[96],self.scalar_static_f64[110]);
        self.scalar_static_f64[112]=(self.scalar_static_f64[109]/self.scalar_static_f64[111]);
        self.scalar_static_f64[113]=(1.0+self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=p.p156;
        self.scalar_static_f64[115]=p.p157;
        self.scalar_static_f64[116]=f64::powf(self.scalar_static_f64[102],self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[114]/self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(1.0+self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=p.p152;
        self.scalar_static_f64[120]=(self.scalar_static_f64[113]*self.scalar_static_f64[119]);
        self.scalar_static_f64[121]=(self.scalar_static_f64[118]*self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(2.0*self.scalar_static_f64[121]);
        self.scalar_static_f64[123]=p.p153;
        self.scalar_static_f64[124]=(self.scalar_static_f64[122]*self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(2.0*self.scalar_static_f64[76]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[81]-self.scalar_static_f64[125]);
        self.scalar_static_f64[127]=(self.scalar_static_f64[126]-self.scalar_static_f64[124]);
        self.scalar_static_f64[128]=(2.0*self.scalar_static_f64[77]);
        self.scalar_static_f64[129]=(self.scalar_static_f64[81]-self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[129]-self.scalar_static_f64[124]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[80]*self.scalar_static_f64[127]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[80]*self.scalar_static_f64[130]);
        self.scalar_static_f64[133]=p.p11;
        self.scalar_static_f64[134]=p.p304;
        self.scalar_static_f64[135]=p.p12;
        self.scalar_static_f64[136]=(self.scalar_static_f64[134]*self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[133]+self.scalar_static_f64[136]);
        self.scalar_static_f64[138]=p.p305;
        self.scalar_static_f64[139]=p.p13;
        self.scalar_static_f64[140]=(self.scalar_static_f64[138]*self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=(self.scalar_static_f64[137]+self.scalar_static_f64[140]);
        self.scalar_static_f64[142]=(self.scalar_static_f64[63]*self.scalar_static_f64[141]);
        self.scalar_static_f64[143]=(self.scalar_static_f64[52]+self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[143]-1e21);
        self.scalar_static_f64[145]=(self.scalar_static_f64[144]-10000.0);
        self.scalar_static_f64[146]=(self.scalar_static_f64[145]*self.scalar_static_f64[145]);
        self.scalar_static_f64[147]=(4e25+self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[147]).sqrt();
        self.scalar_static_f64[149]=(self.scalar_static_f64[145]+self.scalar_static_f64[148]);
        self.scalar_static_f64[150]=(0.5*self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=(1e21+self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=(self.scalar_static_f64[65]*self.scalar_static_f64[141]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[43]+self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=(self.scalar_static_f64[153]-1e21);
        self.scalar_static_f64[155]=(self.scalar_static_f64[154]-10000.0);
        self.scalar_static_f64[156]=(self.scalar_static_f64[155]*self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=(4e25+self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=(self.scalar_static_f64[157]).sqrt();
        self.scalar_static_f64[159]=(self.scalar_static_f64[155]+self.scalar_static_f64[158]);
        self.scalar_static_f64[160]=(0.5*self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(1e21+self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=p.p86;
        self.scalar_static_f64[163]=p.p88;
        self.scalar_static_f64[164]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=(self.scalar_static_f64[162]*self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=p.p90;
        self.scalar_static_f64[167]=p.p91;
        self.scalar_static_f64[168]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=(self.scalar_static_f64[166]/self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(1.0+self.scalar_static_f64[169]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[165]*self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=p.p87;
        self.scalar_static_f64[173]=p.p89;
        self.scalar_static_f64[174]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[173]);
        self.scalar_static_f64[175]=(self.scalar_static_f64[172]*self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=p.p92;
        self.scalar_static_f64[177]=p.p93;
        self.scalar_static_f64[178]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[177]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[176]/self.scalar_static_f64[178]);
        self.scalar_static_f64[180]=(1.0+self.scalar_static_f64[179]);
        self.scalar_static_f64[181]=(self.scalar_static_f64[175]*self.scalar_static_f64[180]);
        self.scalar_static_f64[182]=p.p289;
        self.scalar_static_f64[183]=p.p291;
        self.scalar_static_f64[184]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[183]);
        self.scalar_static_f64[185]=(self.scalar_static_f64[182]*self.scalar_static_f64[184]);
        self.scalar_static_f64[186]=p.p293;
        self.scalar_static_f64[187]=p.p294;
        self.scalar_static_f64[188]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[187]);
        self.scalar_static_f64[189]=(self.scalar_static_f64[186]/self.scalar_static_f64[188]);
        self.scalar_static_f64[190]=(1.0+self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(self.scalar_static_f64[185]*self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=p.p290;
        self.scalar_static_f64[193]=p.p292;
        self.scalar_static_f64[194]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[193]);
        self.scalar_static_f64[195]=(self.scalar_static_f64[192]*self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=p.p295;
        self.scalar_static_f64[197]=p.p296;
        self.scalar_static_f64[198]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=(self.scalar_static_f64[196]/self.scalar_static_f64[198]);
        self.scalar_static_f64[200]=(1.0+self.scalar_static_f64[199]);
        self.scalar_static_f64[201]=(self.scalar_static_f64[195]*self.scalar_static_f64[200]);
        self.scalar_static_f64[202]=p.p106;
        self.scalar_static_f64[203]=p.p107;
        self.scalar_static_f64[204]=p.p110;
        self.scalar_static_f64[205]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[203]/self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=(1.0+self.scalar_static_f64[206]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[202]*self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=p.p108;
        self.scalar_static_f64[210]=p.p109;
        self.scalar_static_f64[211]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(self.scalar_static_f64[209]/self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(1.0+self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=(self.scalar_static_f64[208]*self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=p.p283;
        self.scalar_static_f64[216]=p.p285;
        self.scalar_static_f64[217]=p.p286;
        self.scalar_static_f64[218]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[217]);
        self.scalar_static_f64[219]=(self.scalar_static_f64[216]/self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=(1.0+self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=(self.scalar_static_f64[215]*self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=p.p287;
        self.scalar_static_f64[223]=p.p288;
        self.scalar_static_f64[224]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[223]);
        self.scalar_static_f64[225]=(self.scalar_static_f64[222]/self.scalar_static_f64[224]);
        self.scalar_static_f64[226]=(1.0+self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=(self.scalar_static_f64[221]*self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=p.p232;
        self.scalar_static_f64[229]=p.p233;
        self.scalar_static_f64[230]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[229]);
        self.scalar_static_f64[231]=(self.scalar_static_f64[228]/self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(1.0+self.scalar_static_f64[231]);
        self.scalar_static_f64[233]=(self.scalar_static_f64[54]*self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(self.scalar_static_f64[233]-self.scalar_static_f64[59]);
        self.scalar_static_f64[235]=(self.scalar_static_f64[54]*0.001);
        self.scalar_static_f64[236]=(self.scalar_static_f64[234]-self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=(4.0*self.scalar_static_f64[59]);
        self.scalar_static_f64[238]=(self.scalar_static_f64[235]*self.scalar_static_f64[237]);
        self.scalar_static_bool[4]=(self.scalar_static_f64[238]>0.0);
        self.scalar_static_f64[239]=(-self.scalar_static_f64[238]);
        self.scalar_static_f64[240]=(if self.scalar_static_bool[4]{self.scalar_static_f64[238]}else{self.scalar_static_f64[239]});
        self.scalar_static_f64[241]=(self.scalar_static_f64[236]*self.scalar_static_f64[236]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[240]+self.scalar_static_f64[241]);
        self.scalar_static_f64[243]=(self.scalar_static_f64[242]).sqrt();
        self.scalar_static_f64[244]=(self.scalar_static_f64[236]+self.scalar_static_f64[243]);
        self.scalar_static_f64[245]=(0.5*self.scalar_static_f64[244]);
        self.scalar_static_f64[246]=(self.scalar_static_f64[59]+self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=p.p32;
        self.scalar_static_f64[248]=p.p234;
        self.scalar_static_f64[249]=p.p235;
        self.scalar_static_f64[250]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[249]);
        self.scalar_static_f64[251]=(self.scalar_static_f64[248]/self.scalar_static_f64[250]);
        self.scalar_static_f64[252]=(1.0+self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[246]*self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[253]}else{self.scalar_static_f64[233]});
        self.scalar_static_f64[255]=(self.scalar_static_f64[254]-self.scalar_static_f64[59]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[255]-self.scalar_static_f64[235]);
        self.scalar_static_f64[257]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[256]}else{self.scalar_static_f64[236]});
        self.scalar_static_f64[258]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[238]}else{self.scalar_static_f64[243]});
        self.scalar_static_bool[5]=(self.scalar_static_f64[258]>0.0);
        self.scalar_static_f64[259]=(-self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(if self.scalar_static_bool[5]{self.scalar_static_f64[258]}else{self.scalar_static_f64[259]});
        self.scalar_static_f64[261]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[260]}else{self.scalar_static_f64[258]});
        self.scalar_static_f64[262]=(self.scalar_static_f64[257]*self.scalar_static_f64[257]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[261]+self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=(self.scalar_static_f64[263]).sqrt();
        self.scalar_static_f64[265]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[264]}else{self.scalar_static_f64[261]});
        self.scalar_static_f64[266]=(self.scalar_static_f64[257]+self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(0.5*self.scalar_static_f64[266]);
        self.scalar_static_f64[268]=(self.scalar_static_f64[59]+self.scalar_static_f64[267]);
        self.scalar_static_f64[269]=(if (self.scalar_static_f64[247]!=0.0){self.scalar_static_f64[268]}else{self.scalar_static_f64[246]});
        self.scalar_static_f64[270]=p.p60;
        self.scalar_static_f64[271]=p.p61;
        self.scalar_static_f64[272]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[271]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[270]/self.scalar_static_f64[272]);
        self.scalar_static_f64[274]=(1.0+self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[161]*self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=p.p43;
        self.scalar_static_f64[277]=(0.5*self.scalar_static_f64[78]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[276]+self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(1.0/self.scalar_static_f64[278]);
        self.scalar_static_f64[280]=p.p44;
        self.scalar_static_f64[281]=(self.scalar_static_f64[277]+self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(1.0/self.scalar_static_f64[281]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[279]+self.scalar_static_f64[282]);
        self.scalar_static_f64[284]=(2.0/self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=p.p6;
        self.scalar_static_bool[6]=(self.scalar_static_f64[285]>0.0);
        self.scalar_static_f64[286]=p.p7;
        self.scalar_static_bool[7]=(self.scalar_static_f64[286]>0.0);
        self.scalar_static_bool[8]=(self.scalar_static_bool[6]&&self.scalar_static_bool[7]);
        self.scalar_static_bool[9]=(1.0==self.scalar_static_f64[80]);
        self.scalar_static_bool[10]=(self.scalar_static_f64[80]>1.0);
        self.scalar_static_f64[287]=p.p8;
        self.scalar_static_bool[11]=(self.scalar_static_f64[287]>0.0);
        self.scalar_static_bool[12]=(self.scalar_static_bool[10]&&self.scalar_static_bool[11]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[9]||self.scalar_static_bool[12]);
        self.scalar_static_bool[14]=(self.scalar_static_bool[8]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[288]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[289]=(if (self.scalar_static_f64[288]!=0.0){0.0}else{self.scalar_static_f64[283]});
        self.scalar_static_f64[290]=(2.0*self.scalar_static_f64[80]);
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[288]!=0.0));
        self.scalar_static_f64[291]=p.p166;
        self.scalar_static_f64[292]=(1.0+self.scalar_static_f64[291]);
        self.scalar_static_f64[293]=(1.0/self.scalar_static_f64[292]);
        self.scalar_static_f64[294]=p.p169;
        self.scalar_static_f64[295]=(1.0+self.scalar_static_f64[294]);
        self.scalar_static_f64[296]=(1.0/self.scalar_static_f64[295]);
        self.scalar_static_f64[297]=p.p168;
        self.scalar_static_f64[298]=p.p170;
        self.scalar_static_f64[299]=(self.scalar_static_f64[297]/self.scalar_static_f64[284]);
        self.scalar_static_f64[300]=f64::powf(self.scalar_static_f64[299],self.scalar_static_f64[298]);
        self.scalar_static_f64[301]=p.p190;
        self.scalar_static_f64[302]=p.p191;
        self.scalar_static_f64[303]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[301]/self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=(1.0+self.scalar_static_f64[304]);
        self.scalar_static_f64[306]=p.p58;
        self.scalar_static_bool[16]=(self.scalar_static_f64[78]>self.scalar_static_f64[306]);
        self.scalar_static_bool[17]=(self.scalar_static_f64[306]<=0.0);
        self.scalar_static_bool[18]=(self.scalar_static_bool[16]||self.scalar_static_bool[17]);
        self.scalar_static_f64[307]=(if self.scalar_static_bool[18]{1.0}else{0.0});
        self.scalar_static_f64[308]=(self.scalar_static_f64[78]-self.scalar_static_f64[306]);
        self.scalar_static_bool[19]=(!(self.scalar_static_f64[307]!=0.0));
        self.scalar_static_f64[309]=(self.scalar_static_f64[306]-self.scalar_static_f64[78]);
        self.scalar_static_f64[310]=(self.scalar_static_f64[269]*1.6021918e-19);
        self.scalar_static_f64[311]=(1.034943e-10*self.scalar_static_f64[310]);
        self.scalar_static_f64[312]=p.p242;
        self.scalar_static_f64[313]=(-self.scalar_static_f64[312]);
        self.scalar_static_f64[314]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[313]);
        self.scalar_static_f64[315]=(self.scalar_static_f64[5]*self.scalar_static_f64[314]);
        self.scalar_static_f64[316]=p.p243;
        self.scalar_static_f64[317]=p.p244;
        self.scalar_static_f64[318]=(-self.scalar_static_f64[317]);
        self.scalar_static_f64[319]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[318]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[316]*self.scalar_static_f64[319]);
        self.scalar_static_f64[321]=p.p246;
        self.scalar_static_f64[322]=p.p248;
        self.scalar_static_f64[323]=(self.scalar_static_f64[82]+self.scalar_static_f64[322]);
        self.scalar_static_f64[324]=p.p247;
        self.scalar_static_f64[325]=(-self.scalar_static_f64[324]);
        self.scalar_static_f64[326]=f64::powf(self.scalar_static_f64[323],self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=(self.scalar_static_f64[321]*self.scalar_static_f64[326]);
        self.scalar_static_f64[328]=(2.0*self.scalar_static_f64[306]);
        self.scalar_static_bool[20]=(self.scalar_static_f64[78]<=self.scalar_static_f64[328]);
        self.scalar_static_bool[21]=(self.scalar_static_f64[306]>0.0);
        self.scalar_static_bool[22]=(self.scalar_static_bool[20]&&self.scalar_static_bool[21]);
        self.scalar_static_f64[329]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_bool[23]=(!(self.scalar_static_f64[329]!=0.0));
        self.scalar_static_f64[330]=(1.0/self.scalar_static_f64[82]);
        self.scalar_static_f64[331]=(1.0+self.scalar_static_f64[330]);
        self.scalar_static_f64[332]=p.p77;
        self.scalar_static_f64[333]=f64::powf(self.scalar_static_f64[331],self.scalar_static_f64[332]);
        self.scalar_static_f64[334]=p.p75;
        self.scalar_static_f64[335]=(self.scalar_static_f64[333]*self.scalar_static_f64[334]);
        self.scalar_static_f64[336]=p.p116;
        self.scalar_static_f64[337]=(self.scalar_static_f64[82]*self.scalar_static_f64[336]);
        self.scalar_static_f64[338]=p.p115;
        self.scalar_static_f64[339]=(self.scalar_static_f64[337]*self.scalar_static_f64[338]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[337]+self.scalar_static_f64[338]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[339]/self.scalar_static_f64[340]);
        self.scalar_static_f64[342]=p.p117;
        self.scalar_static_f64[343]=(self.scalar_static_f64[341]+self.scalar_static_f64[342]);
        self.scalar_static_f64[344]=(1e-50+self.scalar_static_f64[343]);
        self.scalar_static_f64[345]=p.p179;
        self.scalar_static_f64[346]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=p.p180;
        self.scalar_static_f64[348]=(self.scalar_static_f64[346]*self.scalar_static_f64[347]);
        self.scalar_static_f64[349]=(1.0+self.scalar_static_f64[348]);
        self.scalar_static_f64[350]=p.p25;
        self.scalar_static_bool[24]=(1.0==self.scalar_static_f64[350]);
        self.scalar_static_f64[351]=(if self.scalar_static_bool[24]{1.0}else{0.0});
        self.scalar_static_f64[352]=p.p3;
        self.scalar_static_f64[353]=p.p2;
        self.scalar_static_f64[354]=(3.0*self.scalar_static_f64[353]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[127]/self.scalar_static_f64[354]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[352]+self.scalar_static_f64[355]);
        self.scalar_static_f64[357]=(if (self.scalar_static_f64[351]!=0.0){self.scalar_static_f64[356]}else{self.scalar_static_f64[337]});
        self.scalar_static_f64[358]=p.p48;
        self.scalar_static_f64[359]=(self.scalar_static_f64[357]*self.scalar_static_f64[358]);
        self.scalar_static_f64[360]=p.p4;
        self.scalar_static_f64[361]=(self.scalar_static_f64[78]-self.scalar_static_f64[360]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[353]*self.scalar_static_f64[361]);
        self.scalar_static_f64[363]=(self.scalar_static_f64[80]*self.scalar_static_f64[362]);
        self.scalar_static_f64[364]=(self.scalar_static_f64[359]/self.scalar_static_f64[363]);
        self.scalar_static_f64[365]=(if (self.scalar_static_f64[351]!=0.0){self.scalar_static_f64[364]}else{0.0});
        self.scalar_static_bool[25]=(self.scalar_static_f64[365]>0.001);
        self.scalar_static_f64[366]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_bool[26]=((self.scalar_static_f64[351]!=0.0)&&(self.scalar_static_f64[366]!=0.0));
        self.scalar_static_f64[367]=(1.0/self.scalar_static_f64[365]);
        self.scalar_static_f64[368]=(if self.scalar_static_bool[26]{self.scalar_static_f64[367]}else{self.scalar_static_f64[365]});
        self.scalar_static_bool[27]=(!(self.scalar_static_f64[366]!=0.0));
        self.scalar_static_bool[28]=((self.scalar_static_f64[351]!=0.0)&&self.scalar_static_bool[27]);
        self.scalar_static_f64[369]=(if self.scalar_static_bool[28]{1000.0}else{self.scalar_static_f64[368]});
        self.scalar_static_bool[29]=(!(self.scalar_static_f64[351]!=0.0));
        self.scalar_static_f64[370]=(if self.scalar_static_bool[29]{1000.0}else{self.scalar_static_f64[369]});
        self.scalar_static_f64[371]=p.p131;
        self.scalar_static_f64[372]=p.p132;
        self.scalar_static_f64[373]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[372]);
        self.scalar_static_f64[374]=(self.scalar_static_f64[371]/self.scalar_static_f64[373]);
        self.scalar_static_f64[375]=(1.0+self.scalar_static_f64[374]);
        self.scalar_static_f64[376]=p.p125;
        self.scalar_static_f64[377]=p.p126;
        self.scalar_static_f64[378]=p.p127;
        self.scalar_static_f64[379]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[378]);
        self.scalar_static_f64[380]=(self.scalar_static_f64[377]/self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=(1.0+self.scalar_static_f64[380]);
        self.scalar_static_f64[382]=(self.scalar_static_f64[376]*self.scalar_static_f64[381]);
        self.scalar_static_f64[383]=p.p124;
        self.scalar_static_f64[384]=(self.scalar_static_f64[82]+self.scalar_static_f64[383]);
        self.scalar_static_f64[385]=(self.scalar_static_f64[82]/self.scalar_static_f64[384]);
        self.scalar_static_f64[386]=p.p118;
        self.scalar_static_f64[387]=p.p120;
        self.scalar_static_f64[388]=p.p121;
        self.scalar_static_f64[389]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[388]);
        self.scalar_static_f64[390]=(self.scalar_static_f64[387]/self.scalar_static_f64[389]);
        self.scalar_static_f64[391]=(1.0+self.scalar_static_f64[390]);
        self.scalar_static_f64[392]=(self.scalar_static_f64[386]*self.scalar_static_f64[391]);
        self.scalar_static_f64[393]=p.p119;
        self.scalar_static_f64[394]=p.p122;
        self.scalar_static_f64[395]=(self.scalar_static_f64[394]/self.scalar_static_f64[82]);
        self.scalar_static_f64[396]=(1.0+self.scalar_static_f64[395]);
        self.scalar_static_f64[397]=(self.scalar_static_f64[393]*self.scalar_static_f64[396]);
        self.scalar_static_f64[398]=(10000.0*self.scalar_static_f64[132]);
        self.scalar_static_f64[399]=p.p46;
        self.scalar_static_f64[400]=(self.scalar_static_f64[398]*self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=p.p47;
        self.scalar_static_f64[402]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[401]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[400]/self.scalar_static_f64[402]);
        self.scalar_static_f64[404]=p.p133;
        self.scalar_static_f64[405]=p.p134;
        self.scalar_static_f64[406]=p.p135;
        self.scalar_static_f64[407]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[406]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[405]/self.scalar_static_f64[407]);
        self.scalar_static_f64[409]=(1.0+self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[404]*self.scalar_static_f64[409]);
        self.scalar_static_f64[411]=p.p128;
        self.scalar_static_f64[412]=p.p129;
        self.scalar_static_f64[413]=p.p130;
        self.scalar_static_f64[414]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[412]/self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=(1.0+self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=(self.scalar_static_f64[411]*self.scalar_static_f64[416]);
        self.scalar_static_f64[418]=p.p33;
        self.scalar_static_f64[419]=p.p28;
        self.scalar_static_bool[30]=(self.scalar_static_f64[55]>0.0);
        self.scalar_static_bool[31]=((self.scalar_static_f64[419]!=0.0)&&self.scalar_static_bool[30]);
        self.scalar_static_f64[420]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[420]!=0.0));
        self.scalar_static_bool[33]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[421]=p.p10;
        self.scalar_static_f64[422]=p.p37;
        self.scalar_static_f64[423]=(self.scalar_static_f64[73]*1e-7);
        self.scalar_static_f64[424]=(9.025e-5+self.scalar_static_f64[423]);
        self.scalar_static_f64[425]=(self.scalar_static_f64[73]*self.scalar_static_f64[424]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[422]-self.scalar_static_f64[425]);
        self.scalar_static_f64[427]=(self.scalar_static_f64[73]*self.scalar_static_f64[73]);
        self.scalar_static_f64[428]=p.p35;
        self.scalar_static_f64[429]=p.p36;
        self.scalar_static_f64[430]=(self.scalar_static_f64[73]*1.3806226e-23);
        self.scalar_static_f64[431]=(1.6021918e-19/self.scalar_static_f64[430]);
        self.scalar_static_f64[432]=p.p202;
        self.scalar_static_f64[433]=p.p249;
        self.scalar_static_f64[434]=p.p95;
        self.scalar_static_f64[435]=p.p96;
        self.scalar_static_f64[436]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[435]);
        self.scalar_static_f64[437]=(self.scalar_static_f64[434]/self.scalar_static_f64[436]);
        self.scalar_static_f64[438]=(1.0+self.scalar_static_f64[437]);
        self.scalar_static_f64[439]=(self.scalar_static_f64[433]*self.scalar_static_f64[438]);
        self.scalar_static_f64[440]=p.p97;
        self.scalar_static_f64[441]=p.p98;
        self.scalar_static_f64[442]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[441]);
        self.scalar_static_f64[443]=(self.scalar_static_f64[440]/self.scalar_static_f64[442]);
        self.scalar_static_f64[444]=(1.0+self.scalar_static_f64[443]);
        self.scalar_static_f64[445]=(self.scalar_static_f64[439]*self.scalar_static_f64[444]);
        self.scalar_static_f64[446]=p.p99;
        self.scalar_static_f64[447]=p.p100;
        self.scalar_static_f64[448]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[446]/self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=(1.0+self.scalar_static_f64[449]);
        self.scalar_static_f64[451]=(self.scalar_static_f64[445]*self.scalar_static_f64[450]);
        self.scalar_static_f64[452]=p.p276;
        self.scalar_static_f64[453]=p.p277;
        self.scalar_static_f64[454]=p.p278;
        self.scalar_static_f64[455]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[454]);
        self.scalar_static_f64[456]=(self.scalar_static_f64[453]/self.scalar_static_f64[455]);
        self.scalar_static_f64[457]=(1.0+self.scalar_static_f64[456]);
        self.scalar_static_f64[458]=(self.scalar_static_f64[452]*self.scalar_static_f64[457]);
        self.scalar_static_f64[459]=p.p281;
        self.scalar_static_f64[460]=p.p282;
        self.scalar_static_f64[461]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(self.scalar_static_f64[459]/self.scalar_static_f64[461]);
        self.scalar_static_f64[463]=(1.0+self.scalar_static_f64[462]);
        self.scalar_static_f64[464]=(self.scalar_static_f64[458]*self.scalar_static_f64[463]);
        self.scalar_static_f64[465]=p.p279;
        self.scalar_static_f64[466]=p.p280;
        self.scalar_static_f64[467]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[466]);
        self.scalar_static_f64[468]=(self.scalar_static_f64[465]/self.scalar_static_f64[467]);
        self.scalar_static_f64[469]=(1.0+self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[464]*self.scalar_static_f64[469]);
        self.scalar_static_f64[471]=p.p163;
        self.scalar_static_f64[472]=(1.0+self.scalar_static_f64[471]);
        self.scalar_static_f64[473]=(1.0/self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=(self.scalar_static_f64[12]/self.scalar_static_f64[284]);
        self.scalar_static_f64[475]=f64::powf(self.scalar_static_f64[474],self.scalar_static_f64[13]);
        self.scalar_static_f64[476]=p.p112;
        self.scalar_static_f64[477]=p.p113;
        self.scalar_static_f64[478]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[476]/self.scalar_static_f64[478]);
        self.scalar_static_f64[480]=(1.0+self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=p.p111;
        self.scalar_static_f64[482]=(self.scalar_static_f64[480]*self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=p.p253;
        self.scalar_static_f64[484]=p.p181;
        self.scalar_static_f64[485]=p.p182;
        self.scalar_static_f64[486]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=(self.scalar_static_f64[484]/self.scalar_static_f64[486]);
        self.scalar_static_f64[488]=(1.0+self.scalar_static_f64[487]);
        self.scalar_static_f64[489]=p.p185;
        self.scalar_static_f64[490]=p.p186;
        self.scalar_static_f64[491]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[490]);
        self.scalar_static_f64[492]=(self.scalar_static_f64[489]/self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=(1.0+self.scalar_static_f64[492]);
        self.scalar_static_f64[494]=(self.scalar_static_f64[488]*self.scalar_static_f64[493]);
        self.scalar_static_f64[495]=p.p187;
        self.scalar_static_f64[496]=p.p188;
        self.scalar_static_f64[497]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[496]);
        self.scalar_static_f64[498]=(self.scalar_static_f64[495]/self.scalar_static_f64[497]);
        self.scalar_static_f64[499]=(1.0+self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=(self.scalar_static_f64[494]*self.scalar_static_f64[499]);
        self.scalar_static_f64[501]=p.p183;
        self.scalar_static_f64[502]=p.p184;
        self.scalar_static_f64[503]=f64::powf(self.scalar_static_f64[84],self.scalar_static_f64[502]);
        self.scalar_static_f64[504]=(self.scalar_static_f64[501]/self.scalar_static_f64[503]);
        self.scalar_static_f64[505]=(1.0+self.scalar_static_f64[504]);
        self.scalar_static_f64[506]=(self.scalar_static_f64[500]*self.scalar_static_f64[505]);
        self.scalar_static_f64[507]=(self.scalar_static_f64[506]*self.scalar_static_f64[506]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[507]+4e-6);
        self.scalar_static_f64[509]=(self.scalar_static_f64[508]).sqrt();
        self.scalar_static_f64[510]=(self.scalar_static_f64[506]+self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(0.5*self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[511]+1e-13);
        self.scalar_static_bool[34]=(self.scalar_static_f64[512]<0.0);
        self.scalar_static_f64[513]=(if self.scalar_static_bool[34]{1.0}else{0.0});
        self.scalar_static_f64[514]=(if (self.scalar_static_f64[513]!=0.0){0.0}else{self.scalar_static_f64[512]});
        self.scalar_static_f64[515]=p.p102;
        self.scalar_static_f64[516]=p.p103;
        self.scalar_static_f64[517]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[516]);
        self.scalar_static_f64[518]=(self.scalar_static_f64[515]/self.scalar_static_f64[517]);
        self.scalar_static_f64[519]=(1.0+self.scalar_static_f64[518]);
        self.scalar_static_f64[520]=(self.scalar_static_f64[41]*self.scalar_static_f64[514]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[45]*self.scalar_static_f64[519]);
        self.scalar_static_f64[522]=(self.scalar_static_f64[426]/2.0);
        self.scalar_static_f64[523]=(self.scalar_static_f64[431]*self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(self.scalar_static_f64[108]*3.2043836e-19);
        self.scalar_static_f64[525]=(1.034943e-10*self.scalar_static_f64[524]);
        self.scalar_static_f64[526]=(self.scalar_static_f64[525]).sqrt();
        self.scalar_static_f64[527]=(self.scalar_static_f64[108]*self.scalar_static_f64[108]);
        self.scalar_static_f64[528]=(1.0/self.scalar_static_f64[527]);
        self.scalar_static_f64[529]=p.p38;
        self.scalar_static_f64[530]=p.p251;
        self.scalar_static_f64[531]=p.p252;
        self.scalar_static_f64[532]=(self.scalar_static_f64[530]+self.scalar_static_f64[531]);
        self.scalar_static_f64[533]=(self.scalar_static_f64[529]/self.scalar_static_f64[532]);
        self.scalar_static_f64[534]=(self.scalar_static_f64[78]*self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(0.001*self.scalar_static_f64[529]);
        self.scalar_static_f64[536]=(self.scalar_static_f64[535]+2.2204460492503132e-17);
        self.scalar_static_f64[537]=(self.scalar_static_f64[536]).abs();
        self.scalar_static_bool[35]=(self.scalar_static_f64[529]>0.0);
        self.scalar_static_f64[538]=(if self.scalar_static_bool[35]{1.0}else{0.0});
        self.scalar_static_f64[539]=(self.scalar_static_f64[529]-self.scalar_static_f64[534]);
        self.scalar_static_f64[540]=(self.scalar_static_f64[539]-self.scalar_static_f64[537]);
        self.scalar_static_f64[541]=(4.0*self.scalar_static_f64[529]);
        self.scalar_static_f64[542]=(self.scalar_static_f64[537]*self.scalar_static_f64[541]);
        self.scalar_static_f64[543]=(if (self.scalar_static_f64[538]!=0.0){self.scalar_static_f64[542]}else{self.scalar_static_f64[509]});
        self.scalar_static_bool[36]=(self.scalar_static_f64[543]>0.0);
        self.scalar_static_f64[544]=(-self.scalar_static_f64[543]);
        self.scalar_static_f64[545]=(if self.scalar_static_bool[36]{self.scalar_static_f64[543]}else{self.scalar_static_f64[544]});
        self.scalar_static_f64[546]=(if (self.scalar_static_f64[538]!=0.0){self.scalar_static_f64[545]}else{self.scalar_static_f64[543]});
        self.scalar_static_bool[37]=(!(self.scalar_static_f64[538]!=0.0));
        self.scalar_static_f64[547]=p.p49;
        self.scalar_static_f64[548]=(-self.scalar_static_f64[547]);
        self.scalar_static_f64[549]=p.p50;
        self.scalar_static_f64[550]=p.p51;
        self.scalar_static_f64[551]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[550]);
        self.scalar_static_f64[552]=(self.scalar_static_f64[549]/self.scalar_static_f64[551]);
        self.scalar_static_f64[553]=(1.0+self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=(self.scalar_static_f64[548]*self.scalar_static_f64[553]);
        self.scalar_static_f64[555]=p.p52;
        self.scalar_static_f64[556]=p.p53;
        self.scalar_static_f64[557]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[556]);
        self.scalar_static_f64[558]=(self.scalar_static_f64[555]/self.scalar_static_f64[557]);
        self.scalar_static_f64[559]=(1.0+self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[548]*self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=p.p54;
        self.scalar_static_f64[562]=(self.scalar_static_f64[82]*self.scalar_static_f64[561]);
        self.scalar_static_f64[563]=(self.scalar_static_f64[547]+self.scalar_static_f64[562]);
        self.scalar_static_f64[564]=(-self.scalar_static_f64[563]);
        self.scalar_static_f64[565]=(self.scalar_static_f64[554]-self.scalar_static_f64[560]);
        self.scalar_static_f64[566]=(self.scalar_static_f64[565]-1e-12);
        self.scalar_static_f64[567]=(4.0*self.scalar_static_f64[560]);
        self.scalar_static_f64[568]=(1e-12*self.scalar_static_f64[567]);
        self.scalar_static_bool[38]=(self.scalar_static_f64[568]>0.0);
        self.scalar_static_f64[569]=(-self.scalar_static_f64[568]);
        self.scalar_static_f64[570]=(if self.scalar_static_bool[38]{self.scalar_static_f64[568]}else{self.scalar_static_f64[569]});
        self.scalar_static_f64[571]=(self.scalar_static_f64[566]*self.scalar_static_f64[566]);
        self.scalar_static_f64[572]=(self.scalar_static_f64[570]+self.scalar_static_f64[571]);
        self.scalar_static_f64[573]=(self.scalar_static_f64[572]).sqrt();
        self.scalar_static_f64[574]=(self.scalar_static_f64[566]+self.scalar_static_f64[573]);
        self.scalar_static_f64[575]=(0.5*self.scalar_static_f64[574]);
        self.scalar_static_f64[576]=(self.scalar_static_f64[560]+self.scalar_static_f64[575]);
        self.scalar_static_f64[577]=(self.scalar_static_f64[576]-self.scalar_static_f64[564]);
        self.scalar_static_f64[578]=(self.scalar_static_f64[577]-1e-12);
        self.scalar_static_f64[579]=(4.0*self.scalar_static_f64[564]);
        self.scalar_static_f64[580]=(1e-12*self.scalar_static_f64[579]);
        self.scalar_static_bool[39]=(self.scalar_static_f64[580]>0.0);
        self.scalar_static_f64[581]=(-self.scalar_static_f64[580]);
        self.scalar_static_f64[582]=(if self.scalar_static_bool[39]{self.scalar_static_f64[580]}else{self.scalar_static_f64[581]});
        self.scalar_static_f64[583]=(self.scalar_static_f64[578]*self.scalar_static_f64[578]);
        self.scalar_static_f64[584]=(self.scalar_static_f64[582]+self.scalar_static_f64[583]);
        self.scalar_static_f64[585]=(self.scalar_static_f64[584]).sqrt();
        self.scalar_static_f64[586]=(self.scalar_static_f64[578]+self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=(0.5*self.scalar_static_f64[586]);
        self.scalar_static_f64[588]=(self.scalar_static_f64[564]+self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(-self.scalar_static_f64[588]);
        self.scalar_static_f64[590]=(2.0*self.scalar_static_f64[311]);
        self.scalar_static_f64[591]=p.p226;
        self.scalar_static_f64[592]=(3.453133e-11/self.scalar_static_f64[591]);
        self.scalar_static_f64[593]=(self.scalar_static_f64[591]/3.453133e-11);
        self.scalar_static_f64[594]=p.p229;
        self.scalar_static_f64[595]=(3.453133e-11/self.scalar_static_f64[594]);
        self.scalar_static_f64[596]=(self.scalar_static_f64[594]/3.453133e-11);
        self.scalar_static_f64[597]=(1.034943e-10/self.scalar_static_f64[16]);
        self.scalar_static_f64[598]=(1.0/self.scalar_static_f64[597]);
        self.scalar_static_f64[599]=(self.scalar_static_f64[596]+self.scalar_static_f64[598]);
        self.scalar_static_f64[600]=p.p254;
        self.scalar_static_f64[601]=p.p255;
        self.scalar_static_f64[602]=(0.5*self.scalar_static_f64[601]);
        self.scalar_static_bool[40]=(self.scalar_static_f64[600]>self.scalar_static_f64[602]);
        self.scalar_static_f64[603]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_f64[604]=(if (self.scalar_static_f64[603]!=0.0){self.scalar_static_f64[602]}else{self.scalar_static_f64[600]});
        self.scalar_static_f64[605]=(self.scalar_static_f64[601]-self.scalar_static_f64[604]);
        self.scalar_static_f64[606]=p.p216;
        self.scalar_static_f64[607]=p.p193;
        self.scalar_static_bool[41]=(0.0==self.scalar_static_f64[607]);
        self.scalar_static_f64[608]=p.p195;
        self.scalar_static_bool[42]=(0.0==self.scalar_static_f64[608]);
        self.scalar_static_bool[43]=(self.scalar_static_bool[41]&&self.scalar_static_bool[42]);
        self.scalar_static_f64[609]=p.p194;
        self.scalar_static_bool[44]=(0.0==self.scalar_static_f64[609]);
        self.scalar_static_bool[45]=(self.scalar_static_bool[43]||self.scalar_static_bool[44]);
        self.scalar_static_f64[610]=(if self.scalar_static_bool[45]{1.0}else{0.0});
        self.scalar_static_bool[46]=(!(self.scalar_static_f64[610]!=0.0));
        self.scalar_static_f64[611]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_bool[47]=(0.0==self.scalar_static_f64[611]);
        self.scalar_static_f64[612]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_f64[613]=(if (self.scalar_static_f64[612]!=0.0){self.scalar_static_f64[591]}else{0.0});
        self.scalar_static_f64[614]=(if (self.scalar_static_f64[612]!=0.0){self.scalar_static_f64[592]}else{0.0});
        self.scalar_static_f64[615]=(if (self.scalar_static_f64[612]!=0.0){self.scalar_static_f64[593]}else{0.0});
        self.scalar_static_bool[48]=(!(self.scalar_static_f64[612]!=0.0));
        self.scalar_static_bool[49]=(0.0!=self.scalar_static_f64[306]);
        self.scalar_static_f64[616]=(if self.scalar_static_bool[49]{1.0}else{0.0});
        self.scalar_static_f64[617]=(2.0*self.scalar_static_f64[16]);
        self.scalar_static_f64[618]=(self.scalar_static_f64[306]*self.scalar_static_f64[306]);
        self.scalar_static_f64[619]=(self.scalar_static_f64[617]/self.scalar_static_f64[618]);
        self.scalar_static_f64[620]=p.p55;
        self.scalar_static_f64[621]=p.p66;
        self.scalar_static_f64[622]=p.p68;
        self.scalar_static_f64[623]=(self.scalar_static_f64[622]/self.scalar_static_f64[306]);
        self.scalar_static_f64[624]=p.p67;
        self.scalar_static_bool[50]=(!(self.scalar_static_f64[616]!=0.0));
        self.scalar_static_f64[625]=p.p297;
        self.scalar_static_bool[51]=(0.0!=self.scalar_static_f64[625]);
        self.scalar_static_f64[626]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_f64[627]=p.p57;
        self.scalar_static_f64[628]=(self.scalar_static_f64[78]-self.scalar_static_f64[627]);
        self.scalar_static_f64[629]=(self.scalar_static_f64[628]*self.scalar_static_f64[628]);
        self.scalar_static_f64[630]=p.p69;
        self.scalar_static_f64[631]=p.p71;
        self.scalar_static_f64[632]=(self.scalar_static_f64[631]/self.scalar_static_f64[78]);
        self.scalar_static_f64[633]=p.p70;
        self.scalar_static_f64[634]=p.p250;
        self.scalar_static_f64[635]=p.p72;
        self.scalar_static_bool[52]=(self.scalar_static_f64[635]>0.0);
        self.scalar_static_f64[636]=(if self.scalar_static_bool[52]{1.0}else{0.0});
        self.scalar_static_f64[637]=p.p74;
        self.scalar_static_f64[638]=(2.0*self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=p.p73;
        self.scalar_static_f64[640]=p.p56;
        self.scalar_static_f64[641]=(self.scalar_static_f64[277]+self.scalar_static_f64[640]);
        self.scalar_static_f64[642]=(self.scalar_static_f64[16]*self.scalar_static_f64[635]);
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[636]!=0.0));
        self.scalar_static_f64[643]=(self.scalar_static_f64[67]/self.scalar_static_f64[127]);
        self.scalar_static_f64[644]=p.p104;
        self.scalar_static_f64[645]=(self.scalar_static_f64[644]/self.scalar_static_f64[83]);
        self.scalar_static_bool[54]=(0.0==self.scalar_static_f64[334]);
        self.scalar_static_f64[646]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_f64[647]=(if (self.scalar_static_f64[646]!=0.0){0.0}else{1.0});
        self.scalar_static_bool[55]=(!(self.scalar_static_f64[646]!=0.0));
        self.scalar_static_f64[648]=(if self.scalar_static_bool[55]{1.0}else{self.scalar_static_f64[647]});
        self.scalar_static_bool[56]=(0.0==self.scalar_static_f64[648]);
        self.scalar_static_f64[649]=(if self.scalar_static_bool[56]{1.0}else{0.0});
        self.scalar_static_bool[57]=(!(self.scalar_static_f64[649]!=0.0));
        self.scalar_static_f64[650]=p.p76;
        self.scalar_static_f64[651]=p.p29;
        self.scalar_static_bool[58]=(!(self.scalar_static_f64[651]!=0.0));
        self.scalar_static_f64[652]=(self.scalar_static_f64[16]*0.99);
        self.scalar_static_f64[653]=(1.0/self.scalar_static_f64[595]);
        self.scalar_static_f64[654]=(0.5*self.scalar_static_f64[598]);
        self.scalar_static_f64[655]=p.p298;
        self.scalar_static_f64[656]=(self.scalar_static_f64[16]/1.034943e-10);
        self.scalar_static_f64[657]=(0.5*self.scalar_static_f64[656]);
        self.scalar_static_f64[658]=(self.scalar_static_f64[653]+self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=p.p15;
        self.scalar_static_bool[59]=(1.0==self.scalar_static_f64[659]);
        self.scalar_static_f64[660]=p.p136;
        self.scalar_static_f64[661]=(2.0*self.scalar_static_f64[127]);
        self.scalar_static_f64[662]=(100.0*self.scalar_static_f64[591]);
        self.scalar_static_f64[663]=(self.scalar_static_f64[131]*100.0);
        self.scalar_static_f64[664]=p.p26;
        self.scalar_static_bool[60]=(0.0==self.scalar_static_f64[664]);
        self.scalar_static_f64[665]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=(!(self.scalar_static_f64[665]!=0.0));
        self.scalar_static_bool[62]=(self.scalar_static_f64[392]<=0.0);
        self.scalar_static_f64[666]=p.p123;
        self.scalar_static_f64[667]=(self.scalar_static_f64[375]*self.scalar_static_f64[385]);
        self.scalar_static_f64[668]=(-self.scalar_static_f64[397]);
        self.scalar_static_f64[669]=p.p16;
        self.scalar_static_bool[63]=(1.0==self.scalar_static_f64[669]);
        self.scalar_static_f64[670]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_f64[671]=(self.scalar_static_f64[16]*1.6021918e-19);
        self.scalar_static_f64[672]=(self.scalar_static_f64[131]*self.scalar_static_f64[671]);
        self.scalar_static_f64[673]=p.p140;
        self.scalar_static_f64[674]=p.p139;
        self.scalar_static_f64[675]=(self.scalar_static_f64[344]-1.0);
        self.scalar_static_f64[676]=(1.0/self.scalar_static_f64[344]);
        self.scalar_static_f64[677]=(self.scalar_static_f64[676]-1.0);
        self.scalar_static_f64[678]=p.p178;
        self.scalar_static_bool[64]=(self.scalar_static_f64[678]<2.220446049250313e-15);
        self.scalar_static_f64[679]=p.p176;
        self.scalar_static_f64[680]=(1.0-self.scalar_static_f64[679]);
        self.scalar_static_f64[681]=(-self.scalar_static_f64[132]);
        self.scalar_static_f64[682]=p.p217;
        self.scalar_static_f64[683]=(100.0*self.scalar_static_f64[594]);
        self.scalar_static_f64[684]=p.p81;
        self.scalar_static_f64[685]=p.p82;
        self.scalar_static_f64[686]=p.p83;
        self.scalar_static_f64[687]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[686]);
        self.scalar_static_f64[688]=(self.scalar_static_f64[685]/self.scalar_static_f64[687]);
        self.scalar_static_f64[689]=(1.0+self.scalar_static_f64[688]);
        self.scalar_static_f64[690]=(self.scalar_static_f64[684]*self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[690]/1.034943e-12);
        self.scalar_static_f64[692]=p.p78;
        self.scalar_static_f64[693]=p.p79;
        self.scalar_static_f64[694]=p.p80;
        self.scalar_static_f64[695]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[694]);
        self.scalar_static_f64[696]=(self.scalar_static_f64[693]/self.scalar_static_f64[695]);
        self.scalar_static_f64[697]=(1.0+self.scalar_static_f64[696]);
        self.scalar_static_f64[698]=(self.scalar_static_f64[692]*self.scalar_static_f64[697]);
        self.scalar_static_f64[699]=(self.scalar_static_f64[698]/1.034943e-12);
        self.scalar_static_f64[700]=(self.scalar_static_f64[606]).sqrt();
        self.scalar_static_f64[701]=p.p85;
        self.scalar_static_f64[702]=p.p84;
        self.scalar_static_f64[703]=p.p299;
        self.scalar_static_f64[704]=p.p300;
        self.scalar_static_f64[705]=p.p301;
        self.scalar_static_f64[706]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[705]);
        self.scalar_static_f64[707]=(self.scalar_static_f64[704]/self.scalar_static_f64[706]);
        self.scalar_static_f64[708]=(1.0+self.scalar_static_f64[707]);
        self.scalar_static_f64[709]=(self.scalar_static_f64[703]*self.scalar_static_f64[708]);
        self.scalar_static_f64[710]=(self.scalar_static_f64[683]*11.7);
        self.scalar_static_bool[65]=(!(self.scalar_static_f64[247]!=0.0));
        self.scalar_static_f64[711]=p.p94;
        self.scalar_static_f64[712]=p.p105;
        self.scalar_static_f64[713]=p.p302;
        self.scalar_static_f64[714]=(self.scalar_static_f64[708]*self.scalar_static_f64[713]);
        self.scalar_static_f64[715]=(if self.scalar_static_bool[65]{self.scalar_static_f64[714]}else{0.0});
        self.scalar_static_f64[716]=p.p275;
        self.scalar_static_f64[717]=p.p284;
        self.scalar_static_f64[718]=p.p114;
        self.scalar_static_bool[66]=(0.9999999999999978<=self.scalar_static_f64[718]);
        self.scalar_static_bool[67]=(self.scalar_static_f64[718]<=1.0000000000000022);
        self.scalar_static_bool[68]=(self.scalar_static_bool[66]&&self.scalar_static_bool[67]);
        self.scalar_static_f64[719]=(if self.scalar_static_bool[68]{1.0}else{0.0});
        self.scalar_static_bool[69]=(1.9999999999999978<=self.scalar_static_f64[718]);
        self.scalar_static_bool[70]=(self.scalar_static_f64[718]<=2.000000000000002);
        self.scalar_static_bool[71]=(self.scalar_static_bool[69]&&self.scalar_static_bool[70]);
        self.scalar_static_f64[720]=(if self.scalar_static_bool[71]{1.0}else{0.0});
        self.scalar_static_bool[72]=(!(self.scalar_static_f64[719]!=0.0));
        self.scalar_static_bool[73]=((self.scalar_static_f64[720]!=0.0)&&self.scalar_static_bool[72]);
        self.scalar_static_bool[74]=(!(self.scalar_static_f64[720]!=0.0));
        self.scalar_static_bool[75]=(self.scalar_static_bool[72]&&self.scalar_static_bool[74]);
        self.scalar_static_f64[721]=(self.scalar_static_f64[718]-1.0);
        self.scalar_static_f64[722]=(-1.0/self.scalar_static_f64[718]);
        self.scalar_static_f64[723]=(self.scalar_static_f64[722]-1.0);
        self.scalar_static_f64[724]=p.p240;
        self.scalar_static_f64[725]=p.p241;
        self.scalar_static_bool[76]=(!(self.scalar_static_f64[6]!=0.0));
        self.scalar_static_bool[77]=(0.0!=self.scalar_static_f64[321]);
        self.scalar_static_f64[726]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_bool[78]=(!(self.scalar_static_f64[726]!=0.0));
        self.scalar_static_f64[727]=p.p245;
        self.scalar_static_f64[728]=(-self.scalar_static_f64[727]);
        self.scalar_static_f64[729]=p.p22;
        self.scalar_static_bool[79]=(0.0!=self.scalar_static_f64[729]);
        self.scalar_static_f64[730]=(if self.scalar_static_bool[79]{1.0}else{0.0});
        self.scalar_static_f64[731]=(self.scalar_static_f64[89]-self.scalar_static_f64[627]);
        self.scalar_static_f64[732]=p.p158;
        self.scalar_static_f64[733]=p.p159;
        self.scalar_static_f64[734]=p.p160;
        self.scalar_static_f64[735]=p.p161;
        self.scalar_static_f64[736]=(4.0*self.scalar_static_f64[660]);
        self.scalar_static_bool[80]=((1.0!=0.0)&&(self.scalar_static_f64[730]!=0.0));
        self.scalar_static_bool[81]=(false&&(self.scalar_static_f64[730]!=0.0));
        self.scalar_static_f64[737]=(self.scalar_static_f64[80]*self.scalar_static_f64[122]);
        self.scalar_static_f64[738]=p.p20;
        self.scalar_static_bool[82]=(0.0!=self.scalar_static_f64[738]);
        self.scalar_static_f64[739]=p.p23;
        self.scalar_static_bool[83]=(0.0!=self.scalar_static_f64[739]);
        self.scalar_static_bool[84]=(self.scalar_static_bool[82]&&self.scalar_static_bool[83]);
        self.scalar_static_f64[740]=(if self.scalar_static_bool[84]{1.0}else{0.0});
        self.scalar_static_f64[741]=p.p145;
        self.scalar_static_bool[85]=(0.0!=self.scalar_static_f64[741]);
        self.scalar_static_f64[742]=p.p146;
        self.scalar_static_bool[86]=(0.0==self.scalar_static_f64[8]);
        self.scalar_static_f64[743]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_bool[87]=(!(self.scalar_static_f64[743]!=0.0));
        self.scalar_static_f64[744]=p.p256;
        self.scalar_static_f64[745]=(self.scalar_static_f64[589]*self.scalar_static_f64[744]);
        self.scalar_static_f64[746]=p.p258;
        self.scalar_static_f64[747]=(-self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=p.p206;
        self.scalar_static_f64[749]=p.p205;
        self.scalar_static_f64[750]=p.p209;
        self.scalar_static_f64[751]=p.p208;
        self.scalar_static_f64[752]=p.p204;
        self.scalar_static_f64[753]=(-self.scalar_static_f64[752]);
        self.scalar_static_f64[754]=p.p203;
        self.scalar_static_f64[755]=p.p257;
        self.scalar_static_f64[756]=p.p211;
        self.scalar_static_f64[757]=(-self.scalar_static_f64[756]);
        self.scalar_static_f64[758]=p.p212;
        self.scalar_static_f64[759]=p.p260;
        self.scalar_static_f64[760]=(1.0/self.scalar_static_f64[662]);
        self.scalar_static_f64[761]=(self.scalar_static_f64[760]/self.scalar_static_f64[662]);
        self.scalar_static_f64[762]=p.p210;
        self.scalar_static_f64[763]=(self.scalar_static_f64[762]/1000000.0);
        self.scalar_static_f64[764]=(self.scalar_static_f64[663]*self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=p.p259;
        self.scalar_static_f64[766]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[765]);
        self.scalar_static_f64[767]=(self.scalar_static_f64[764]*self.scalar_static_f64[766]);
        self.scalar_static_f64[768]=p.p261;
        self.scalar_static_f64[769]=p.p215;
        self.scalar_static_f64[770]=p.p214;
        self.scalar_static_f64[771]=(-self.scalar_static_f64[770]);
        self.scalar_static_f64[772]=p.p263;
        self.scalar_static_f64[773]=p.p264;
        self.scalar_static_f64[774]=(self.scalar_static_f64[82]+self.scalar_static_f64[773]);
        self.scalar_static_f64[775]=p.p265;
        self.scalar_static_f64[776]=(4.0*self.scalar_static_f64[775]);
        self.scalar_static_f64[777]=p.p213;
        self.scalar_static_f64[778]=p.p262;
        self.scalar_static_f64[779]=p.p269;
        self.scalar_static_f64[780]=p.p268;
        self.scalar_static_f64[781]=p.p267;
        self.scalar_static_f64[782]=(-self.scalar_static_f64[781]);
        self.scalar_static_f64[783]=p.p271;
        self.scalar_static_f64[784]=p.p272;
        self.scalar_static_f64[785]=(self.scalar_static_f64[82]+self.scalar_static_f64[784]);
        self.scalar_static_f64[786]=p.p273;
        self.scalar_static_f64[787]=(4.0*self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=p.p266;
        self.scalar_static_f64[789]=p.p270;
        self.scalar_static_f64[790]=(if self.scalar_static_bool[87]{0.5}else{0.0});
        self.scalar_static_bool[88]=(0.0==self.scalar_static_f64[10]);
        self.scalar_static_f64[791]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_bool[89]=(!(self.scalar_static_f64[791]!=0.0));
        self.scalar_static_f64[792]=p.p198;
        self.scalar_static_f64[793]=p.p199;
        self.scalar_static_f64[794]=p.p200;
        self.scalar_static_f64[795]=(-self.scalar_static_f64[61]);
        self.scalar_static_f64[796]=p.p45;
        self.scalar_static_bool[90]=(0.0!=self.scalar_static_f64[796]);
        self.scalar_static_f64[797]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_f64[798]=(1.0/self.scalar_static_f64[592]);
        self.scalar_static_f64[799]=p.p19;
        self.scalar_static_bool[91]=(self.scalar_static_f64[799]>=1.0);
        self.scalar_static_f64[800]=p.p175;
        self.scalar_static_bool[92]=(self.scalar_static_f64[800]>0.0);
        self.scalar_static_bool[93]=(self.scalar_static_bool[91]&&self.scalar_static_bool[92]);
        self.scalar_static_bool[94]=(self.scalar_static_f64[57]>0.0);
        self.scalar_static_bool[95]=(self.scalar_static_bool[93]&&self.scalar_static_bool[94]);
        self.scalar_static_f64[801]=(if self.scalar_static_bool[95]{1.0}else{0.0});
        self.scalar_static_f64[802]=(if (self.scalar_static_f64[801]!=0.0){self.scalar_static_f64[800]}else{0.0});
        self.scalar_static_f64[803]=(if (self.scalar_static_f64[801]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[96]=((self.scalar_static_f64[801]!=0.0)&&(self.scalar_static_f64[803]!=0.0));
        self.scalar_static_bool[97]=((0.0!=0.0)&&(self.scalar_static_f64[801]!=0.0));
        self.scalar_static_f64[804]=p.p39;
        self.scalar_static_f64[805]=p.p30;
        self.scalar_static_bool[98]=(self.scalar_static_f64[805]>0.0);
        self.scalar_static_f64[806]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_bool[99]=(1.0==self.scalar_static_f64[805]);
        self.scalar_static_f64[807]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_f64[808]=(self.scalar_static_f64[132]*self.scalar_static_f64[802]);
        self.scalar_static_f64[809]=(if (self.scalar_static_f64[801]!=0.0){0.0}else{self.scalar_static_f64[803]});
        self.scalar_static_bool[100]=((self.scalar_static_f64[801]!=0.0)&&(self.scalar_static_f64[809]!=0.0));
        self.scalar_static_f64[810]=p.p174;
        self.scalar_static_f64[811]=p.p173;
        self.scalar_static_bool[101]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_bool[102]=(!(self.scalar_static_f64[3]!=0.0));
        self.scalar_static_f64[812]=(if self.scalar_static_bool[92]{1.0}else{0.0});
        self.scalar_static_bool[103]=(!(self.scalar_static_f64[801]!=0.0));
        self.scalar_static_f64[813]=(-self.scalar_static_f64[592]);
        self.scalar_static_f64[814]=(self.scalar_static_f64[800]*self.scalar_static_f64[813]);
        self.scalar_static_f64[815]=(self.scalar_static_f64[132]*self.scalar_static_f64[814]);
        self.scalar_static_bool[104]=(!(self.scalar_static_f64[812]!=0.0));
        self.scalar_static_f64[816]=p.p223;
        self.scalar_static_f64[817]=p.p224;
        self.scalar_static_f64[818]=(self.scalar_static_f64[816]*self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=p.p21;
        self.scalar_static_bool[105]=(0.0!=self.scalar_static_f64[819]);
        self.scalar_static_f64[820]=p.p172;
        self.scalar_static_f64[821]=(-self.scalar_static_f64[820]);
        self.scalar_static_f64[822]=(self.scalar_static_f64[78]*self.scalar_static_f64[821]);
        self.scalar_static_f64[823]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[822]}else{0.0});
        self.scalar_static_bool[106]=(!(self.scalar_static_f64[1]!=0.0));
        self.scalar_static_f64[824]=(self.scalar_static_f64[132]*2.1983327444149834e-11);
        self.scalar_static_f64[825]=(0.0*self.scalar_static_f64[824]);
        self.scalar_static_f64[826]=p.p303;
        self.scalar_static_bool[107]=(!(self.scalar_static_f64[826]!=0.0));
        self.scalar_static_bool[108]=(0.0==self.scalar_static_f64[796]);
        self.scalar_static_f64[827]=(if self.scalar_static_bool[108]{1.0}else{0.0});
        self.scalar_static_bool[109]=(!(self.scalar_static_f64[827]!=0.0));
        self.scalar_static_f64[828]=(self.scalar_static_f64[132]*1.034943e-10);
        self.scalar_static_bool[110]=(0.0!=self.scalar_static_f64[399]);
        self.scalar_static_f64[829]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_f64[830]=p.p14;
        self.scalar_static_bool[111]=(1.0==self.scalar_static_f64[830]);
        self.scalar_static_f64[831]=(if self.scalar_static_bool[111]{1.0}else{0.0});
        self.scalar_static_f64[832]=(1.0-self.scalar_static_f64[790]);
        self.scalar_static_f64[833]=p.p312;
        self.scalar_static_bool[112]=(1.0==self.scalar_static_f64[833]);
        self.scalar_static_f64[834]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_f64[835]=p.p315;
        self.scalar_static_f64[836]=(self.scalar_static_f64[835]/1e-6);
        self.scalar_static_f64[837]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[836]}else{0.0});
        self.scalar_static_f64[838]=p.p317;
        self.scalar_static_f64[839]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[838]}else{0.0});
        self.scalar_static_f64[840]=p.p319;
        self.scalar_static_f64[841]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[840]}else{0.0});
        self.scalar_static_f64[842]=p.p324;
        self.scalar_static_f64[843]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[842]}else{0.0});
        self.scalar_static_f64[844]=p.p314;
        self.scalar_static_bool[113]=(self.scalar_static_f64[844]>0.0);
        self.scalar_static_f64[845]=p.p308;
        self.scalar_static_f64[846]=(self.scalar_static_f64[844]*self.scalar_static_f64[845]);
        self.scalar_static_f64[847]=(if self.scalar_static_bool[113]{self.scalar_static_f64[846]}else{0.0});
        self.scalar_static_f64[848]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[847]}else{0.0});
        self.scalar_static_f64[849]=p.p311;
        self.scalar_static_f64[850]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[851]=p.p322;
        self.scalar_static_f64[852]=(self.scalar_static_f64[851]*self.scalar_static_f64[851]);
        self.scalar_static_f64[853]=(self.scalar_static_f64[529]*self.scalar_static_f64[529]);
        self.scalar_static_f64[854]=(self.scalar_static_f64[852]+self.scalar_static_f64[853]);
        self.scalar_static_f64[855]=(self.scalar_static_f64[854]).sqrt();
        self.scalar_static_f64[856]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[855]}else{0.0});
        self.scalar_static_f64[857]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[131]}else{0.0});
        self.scalar_static_f64[858]=(self.scalar_static_f64[839]/10000.0);
        self.scalar_static_f64[859]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[858]}else{self.scalar_static_f64[839]});
        self.scalar_static_f64[860]=(self.scalar_static_f64[841]/100.0);
        self.scalar_static_f64[861]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[860]}else{self.scalar_static_f64[841]});
        self.scalar_static_f64[862]=p.p320;
        self.scalar_static_f64[863]=p.p321;
        self.scalar_static_f64[864]=p.p325;
        self.scalar_static_f64[865]=p.p330;
        self.scalar_static_f64[866]=p.p331;
        self.scalar_static_f64[867]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[866]);
        self.scalar_static_f64[868]=(self.scalar_static_f64[865]/self.scalar_static_f64[867]);
        self.scalar_static_f64[869]=(1.0+self.scalar_static_f64[868]);
        self.scalar_static_f64[870]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[869]}else{0.0});
        self.scalar_static_f64[871]=p.p328;
        self.scalar_static_f64[872]=p.p329;
        self.scalar_static_f64[873]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[872]);
        self.scalar_static_f64[874]=(self.scalar_static_f64[871]/self.scalar_static_f64[873]);
        self.scalar_static_f64[875]=(1.0+self.scalar_static_f64[874]);
        self.scalar_static_f64[876]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[875]}else{0.0});
        self.scalar_static_f64[877]=p.p326;
        self.scalar_static_f64[878]=p.p327;
        self.scalar_static_f64[879]=f64::powf(self.scalar_static_f64[83],self.scalar_static_f64[878]);
        self.scalar_static_f64[880]=(self.scalar_static_f64[877]/self.scalar_static_f64[879]);
        self.scalar_static_f64[881]=(1.0+self.scalar_static_f64[880]);
        self.scalar_static_f64[882]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[881]}else{0.0});
        self.scalar_static_f64[883]=(1.6021918e-19/self.scalar_static_f64[850]);
        self.scalar_static_f64[884]=p.p313;
        self.scalar_static_bool[114]=(1.0==self.scalar_static_f64[884]);
        self.scalar_static_f64[885]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_f64[886]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[57]}else{0.0});
        self.scalar_static_f64[887]=p.p316;
        self.scalar_static_f64[888]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[887]}else{0.0});
        self.scalar_static_f64[889]=p.p318;
        self.scalar_static_f64[890]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[889]}else{0.0});
        self.scalar_static_f64[891]=p.p323;
        self.scalar_static_f64[892]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[891]}else{0.0});
        self.scalar_static_f64[893]=p.p309;
        self.scalar_static_f64[894]=(self.scalar_static_f64[844]*self.scalar_static_f64[893]);
        self.scalar_static_f64[895]=(if self.scalar_static_bool[113]{self.scalar_static_f64[894]}else{0.0});
        self.scalar_static_f64[896]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[895]}else{0.0});
        self.scalar_static_f64[897]=p.p310;
        self.scalar_static_f64[898]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[897]}else{0.0});
        self.scalar_static_f64[899]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[855]}else{0.0});
        self.scalar_static_f64[900]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[131]}else{0.0});
        self.scalar_static_f64[901]=(self.scalar_static_f64[888]/10000.0);
        self.scalar_static_f64[902]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[901]}else{self.scalar_static_f64[888]});
        self.scalar_static_f64[903]=(self.scalar_static_f64[890]/100.0);
        self.scalar_static_f64[904]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[903]}else{self.scalar_static_f64[890]});
        self.scalar_static_f64[905]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[869]}else{0.0});
        self.scalar_static_f64[906]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[875]}else{0.0});
        self.scalar_static_f64[907]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[881]}else{0.0});
        self.scalar_static_f64[908]=(1.6021918e-19/self.scalar_static_f64[898]);
        self.scalar_static_f64[909]=(self.scalar_static_f64[298]-1.0);
        self.scalar_static_f64[910]=(-self.scalar_static_f64[418]);
        self.scalar_static_f64[911]=(self.scalar_static_f64[910]-self.scalar_static_f64[910]);
        self.scalar_static_f64[912]=(self.scalar_static_f64[432]-1.0);
        self.scalar_static_f64[913]=(self.scalar_static_f64[13]-1.0);
        self.scalar_static_f64[914]=(self.scalar_static_f64[675]-1.0);
        self.scalar_static_f64[915]=(self.scalar_static_f64[677]-1.0);
        self.scalar_static_f64[916]=(self.scalar_static_f64[701]-1.0);
        self.scalar_static_f64[917]=(self.scalar_static_f64[711]-1.0);
        self.scalar_static_f64[918]=(self.scalar_static_f64[214]-1.0);
        self.scalar_static_f64[919]=(self.scalar_static_f64[716]-1.0);
        self.scalar_static_f64[920]=(self.scalar_static_f64[227]-1.0);
        self.scalar_static_f64[921]=(self.scalar_static_f64[721]-1.0);
        self.scalar_static_f64[922]=(self.scalar_static_f64[723]-1.0);
        self.scalar_static_f64[923]=(self.scalar_static_f64[724]-1.0);
        self.scalar_static_f64[924]=(self.scalar_static_f64[755]-1.0);
        self.scalar_static_f64[925]=(self.scalar_static_f64[772]-1.0);
        self.scalar_static_f64[926]=(self.scalar_static_f64[778]-1.0);
        self.scalar_static_f64[927]=(self.scalar_static_f64[783]-1.0);
        self.scalar_static_f64[928]=(self.scalar_static_f64[789]-1.0);
        self.scalar_static_f64[929]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[910]}else{0.0});
        self.scalar_static_f64[930]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[418]}else{0.0});
        self.scalar_static_f64[931]=(self.scalar_static_f64[862]-1.0);
        self.scalar_static_f64[932]=(self.scalar_static_f64[929]/self.scalar_static_f64[850]);
        self.scalar_static_f64[933]=(self.scalar_static_f64[930]/self.scalar_static_f64[850]);
        self.scalar_static_f64[934]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[932]}else{0.0});
        self.scalar_static_f64[935]=(if (self.scalar_static_f64[834]!=0.0){self.scalar_static_f64[933]}else{0.0});
        self.scalar_static_f64[936]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[418]}else{0.0});
        self.scalar_static_f64[937]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[910]}else{0.0});
        self.scalar_static_f64[938]=(self.scalar_static_f64[936]/self.scalar_static_f64[898]);
        self.scalar_static_f64[939]=(self.scalar_static_f64[937]/self.scalar_static_f64[898]);
        self.scalar_static_f64[940]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[938]}else{0.0});
        self.scalar_static_f64[941]=(if (self.scalar_static_f64[885]!=0.0){self.scalar_static_f64[939]}else{0.0});
        self.scalar_static_f64[942]=(-self.scalar_static_f64[370]);
        self.scalar_static_f64[943]=(if (self.scalar_static_f64[350]!=0.0){self.scalar_static_f64[370]}else{0.0});
        self.scalar_static_f64[944]=(if (self.scalar_static_f64[350]!=0.0){self.scalar_static_f64[942]}else{0.0});
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
        self.scalar_static_f64[945]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[75]}else{temperature});
        self.scalar_static_f64[946]=(self.scalar_static_f64[945]+self.scalar_static_f64[421]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
